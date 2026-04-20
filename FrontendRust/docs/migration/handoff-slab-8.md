# Handoff: Typing Pass Slab 8 — `CompilerOutputs` Method Signature Rewrite

> **Post-completion note (2026-04-20):** This doc was written before the Slab 9 audit that expanded the signature-rewrite phase. It references "Slab 10 = body migration" and "Slab 11 = compiler.rs residuals" — both **stale**. Current numbering per `quest.md` §12.1: Slabs 10-13 are additional signature-rewrite slabs covering ~122 sub-compiler methods still with bare `(&self)` or `()` placeholders; body migration is now **Slab 14+**. The `panic!("Unimplemented: Slab 10 — body migration")` messages in the code are informationally stale too — they'll be bulk-updated when bodies land. See TL-HANDOFF.md at repo root for the current slab roadmap.

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0-7 are done:

- **Slab 0** (arena substrate): merged.
- **Slab 1** (leaf types): merged.
- **Slab 2** (name hierarchy): tagged `slab-2-complete`.
- **Slab 3** (Kind/Coord/Templata trio, monomorphic `PrototypeT`/`SignatureT`): tagged `slab-3-complete`.
- **Slab 4** (envs + real interner bodies): tagged `slab-4-complete`.
- **Slab 5** (expression AST): tagged `slab-5-complete`.
- **Slab 6** (`CompilerOutputs<'s, 't>` data struct + `PtrKey<'t, T>` + `DeferredActionT<'s, 't>`): tagged `slab-6-complete`.
- **Slab 7** (`HinputsT` residual cleanup + `Compiler::compile_program`/`drain_all_deferred` panic stubs + `run_typing_pass` entry point): tagged `slab-7-complete`.

**The data-definition slab series is complete.** You're doing the first **signature-rewrite slab**: taking the **54 panic-stub free-functions** in `src/typing/compiler_outputs.rs` that Slab 6 left behind and lifting them into `impl<'s, 't> CompilerOutputs<'s, 't>` blocks with proper receivers, lifetimes, arena refs, and `()`-placeholder flips. Bodies stay `panic!()`. This is narrow, mechanical work — budget ~3 hours focused.

**Read these first in this order**, then come back:

1. `FrontendRust/docs/migration/handoff-slab-6.md` — especially "The `CompilerOutputs<'s, 't>` structure" section (the field types you'll be referencing from method signatures) and Gotchas 1 (env `'t`-lifetime override), 3 (`Vec<&'t ImplT>` in reverse-index fields), 4 (`VecDeque<DeferredActionT>`), and 6 (`PtrKey<'t, T>` wraps the key). Gotcha 10 ("no method body migration") is the **central rule for this slab** — it's being deferred from Slab 8 to Slab 10.
2. `FrontendRust/docs/migration/handoff-slab-7.md` — for TL-HANDOFF file-layout conventions and the `&'t PrototypeT<'s, 't>` flip precedent in `hinputs_t.rs`. You'll repeat the same pattern on CompilerOutputs methods.
3. `TL-HANDOFF.md` at repo root — the file-layout standards section ("one fn per impl block, multi-line body").
4. This doc.

You shouldn't need to read the Scala source externally — every Scala `def` sig is already embedded inline in the `/* def ... */` blocks beneath each Rust stub in `compiler_outputs.rs`. Those blocks are the authoritative signature spec.

---

## The big picture: why Slab 8 exists

Slab 6 landed the `CompilerOutputs<'s, 't>` data struct and its `::new()` constructor. But it deliberately left the ~54 method stubs as **private free-fns** at file scope with panic bodies and partial signatures. Example from the current file state:

```rust
fn lookup_function<'s, 't>(signature: SignatureT<'s, 't>) -> Option<FunctionDefinitionT<'s, 't>> {
    panic!("Unimplemented: lookup_function");
}
/*
  def lookupFunction(signature: SignatureT): Option[FunctionDefinitionT] = {
    signatureToFunction.get(signature)
  }
*/
```

Several things are wrong with that stub for real use:
- It's a free fn, not a method on `CompilerOutputs`. Callers can't reach it via `coutputs.lookup_function(...)`.
- `SignatureT<'s, 't>` is passed by value instead of `&'t SignatureT<'s, 't>` (Slab 3 interned).
- Return type is `Option<FunctionDefinitionT<'s, 't>>` (owned) instead of `Option<&'t FunctionDefinitionT<'s, 't>>` (arena ref).
- No `&self` receiver.

Slab 8 fixes all three across every method. Each stub becomes:

```rust
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_function(
        &self,
        signature: &'t SignatureT<'s, 't>,
    ) -> Option<&'t FunctionDefinitionT<'s, 't>> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def lookupFunction(signature: SignatureT): Option[FunctionDefinitionT] = {
    signatureToFunction.get(signature)
  }
*/
```

Same panic, but a real method with real parameter and return types. Slab 10+ can now migrate bodies (`self.signature_to_function.get(&PtrKey(signature)).copied()`) one at a time without ever having to re-shape the method.

**Why not migrate bodies while you're there?** Because consistency wins. Two rules-of-thumb in this project:
- If one method gets bodies and the next doesn't, the next reader assumes they all do (or none do) and gets confused. Slab 8 gets *zero*; Slab 10 gets *all*.
- Signature-rewrite is mechanical (follow the translation table). Body migration needs judgment (which field is this accessing? does the body still make sense post-`PtrKey`? etc.). Mixing the two blows up the review burden.

See Slab 6 Gotcha 10 for the full argument. Resist the temptation on obvious one-liners.

**By the end of Slab 8:**

- `cargo check --lib` passes with 0 errors.
- `src/typing/compiler_outputs.rs`:
  - All 54 free-fn stubs at file scope are **deleted**. Their `panic!()` bodies reappear inside one-fn `impl<'s, 't> CompilerOutputs<'s, 't> where 's: 't { pub fn foo(...) -> ... { panic!(...) } }` blocks placed exactly where the stub was (directly above the stub's Scala `/* def ... */` anchor).
  - Each method has a correct receiver (`&self` or `&mut self`), parameters/returns translated per the rules below, and an `pub` visibility.
  - Panic message bumped: `panic!("Unimplemented: Slab 10 — body migration");`.
  - Scala `/* */` blocks unchanged byte-for-byte.
- No other file touched.

**What Slab 8 is NOT:**

- No method body migration. `signatureToFunction.get(signature)` in Scala stays `panic!()` in Rust. Slab 10.
- No `templata_compiler.rs` work. Its 35 stubs are Slab 9.
- No `compiler.rs` `()`-placeholder sweep. Slab 11.
- No `local_helper.rs` / `struct_compiler.rs` orphan hosting. Slab 11.
- No `HinputsT` method migration. That's Slab 9 or 10.
- No `ICompileErrorT` variant filling. Slab 12+ when bodies need errors.
- No wiring of `TypingPassCompilation::expect_compiler_outputs`. Slab 10+ plumbing.

---

## What's already in place (don't duplicate; don't delete)

Open `src/typing/compiler_outputs.rs` (~848 lines). You'll see:

- Top imports (Slab 6 added `FunctionA`, `HashSet`, `VecDeque`, `PtrKey`).
- A `pub enum DeferredActionT<'s, 't> where 's: 't { EvaluateFunctionBody { ... }, EvaluateFunction { ... } }` (Slab 6).
- A `pub struct CompilerOutputs<'s, 't> where 's: 't { ... 23 fields ... }` (Slab 6).
- A `impl<'s, 't> CompilerOutputs<'s, 't> where 's: 't { pub fn new() -> Self { ... } }` (Slab 6) — **don't modify this impl block or its contents**; it's done.
- **54 free-fn stubs** starting around line 229 with `count_denizens` and ending around line 842 with `get_function_externs`. These are your targets.
- Each stub has a `/* def ... */` Scala block directly below it. Those blocks are frozen.

### Things NOT in scope (don't touch)

- Any file outside `src/typing/compiler_outputs.rs` — Slabs 2-7 are frozen, sub-compilers are Slab 9-11 territory.
- The `pub struct CompilerOutputs`, its 23 fields, or `CompilerOutputs::new()`. Slab 6 did these.
- `pub enum DeferredActionT` and its Scala anchor blocks. Slab 6 did these.
- The top-of-file imports — add to them only if a specific signature needs a type that isn't already in scope (unlikely; Slab 6 brought most of them in).
- Scala `/* */` blocks anywhere in the file. Frozen.

---

## Signature translation rules

Apply these uniformly. For each stub, read the Scala `/* def ... */` block beneath it, translate each parameter and return type per this table, and classify the receiver per the next section.

| Scala | Rust |
|---|---|
| `SignatureT` (param or return) | `&'t SignatureT<'s, 't>` (Slab 3 interned) |
| `PrototypeT[IFunctionNameT]` | `&'t PrototypeT<'s, 't>` (Slab 3 interned; phantom `[IFunctionNameT]` erased) |
| `IdT[INameT]` / `IdT[ITemplateNameT]` / `IdT[IFunctionNameT]` / `IdT[IInstantiationNameT]` etc. | `IdT<'s, 't>` by value (monomorphic, Slab 2, `Copy` with custom pointer-identity `Hash`/`Eq`). The phantom `[X]` is erased. |
| `CoordT` | `CoordT<'s, 't>` (by value, `Copy`) |
| `ITemplataT[X]` | `ITemplataT<'s, 't>` by value (`Copy`; phantom `[X]` erased per Slab 3) |
| `RangeS` | `RangeS<'s>` (by value, `Copy`) |
| `List[RangeS]` / `Vector[RangeS]` | `&[RangeS<'s>]` (borrowed slice; callers pass `&vec[..]` or `&[r1, r2]`) |
| `StrI` | `StrI<'s>` (by value, `Copy`) |
| `PackageCoordinate` | `PackageCoordinate<'s>` (by value, `Copy`) |
| `Boolean` / `Int` / `Unit` | `bool` / `i32` / `()` (or no return type at all) |
| `IInDenizenEnvironmentT` (param or return) | `&'t IInDenizenEnvironmentT<'s, 't>` — **Slab-4 override, never `&'s IEnvironmentT`** |
| `FunctionEnvironmentT` | `&'t FunctionEnvironmentT<'s, 't>` |
| `StructDefinitionT` / `InterfaceDefinitionT` / `ImplT` / `FunctionDefinitionT` / `CitizenDefinitionT` | `&'t X<'s, 't>` (arena-allocated per Slab 3/5; referenced, not owned) |
| `KindT` / `ICitizenTT` / `InterfaceTT` / `StructTT` | Check Slab 3: if the type is `Copy`, pass by value; if it holds interned children via `&'t`, keep by value (the tagged pointer is cheap). Default: by value. If it fails to compile, fall back to `&'t X<'s, 't>`. |
| `Interner` (the Scala global interner; appears in `addInstantiationBounds`'s param list) | `&'t TypingInterner<'s, 't>` |
| `InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]` (param or return) | `&'t InstantiationBoundArgumentsT<'s, 't>` (matches Slab 7 `hinputs_t.rs` flip precedent; phantoms erased) |
| `DeferredEvaluatingFunctionBody` / `DeferredEvaluatingFunction` (param — push into queue) | `DeferredActionT<'s, 't>` by value |
| `Option[DeferredEvaluatingFunctionBody]` (return — peek head) | `Option<&DeferredActionT<'s, 't>>` (ref into the queue; borrow from `&self`) |
| `Iterable[X]` return (e.g. `getAllStructs: Iterable[StructDefinitionT] = structTemplateNameToDefinition.values`) | `Vec<&'t X<'s, 't>>` (eager collection of arena refs; Scala's `.values` is lazy but the Rust field holds `&'t X`, so we hand callers a `Vec<&'t X>`) |
| `Map[IdT, InstantiationBoundArgumentsT]` return (only `getInstantiationNameToFunctionBoundToRune`) | `HashMap<PtrKey<'t, IdT<'s, 't>>, &'t InstantiationBoundArgumentsT<'s, 't>>` (matches the internal field type from Slab 6; return is a clone of the internal map with its value types) |

### Receiver classification

Rule: methods that mutate a field → `&mut self`. Methods that only read → `&self`. Verify against the Scala body inside the `/* */` block — look for `+=`, `-=`, `.put(...)`, `.remove(...)` on field names → mutation.

**Default classifications (verify each against the Scala body):**

- **`&mut self`**: `mark_deferred_function_body_compiled`, `mark_deferred_function_compiled`, `add_instantiation_bounds`, `declare_function_return_type`, `add_function`, `declare_function`, `declare_type`, `declare_type_mutability`, `declare_type_sealed`, `declare_function_inner_env`, `declare_function_outer_env`, `declare_type_outer_env`, `declare_type_inner_env`, `add_struct`, `add_interface`, `add_impl`, `add_kind_export`, `add_function_export`, `add_kind_extern`, `add_function_extern`, `defer_evaluating_function_body`, `defer_evaluating_function`.
- **`&self`**: `count_denizens`, `peek_next_deferred_function_body_compile`, `peek_next_deferred_function_compile`, `get_instantiation_name_to_function_bound_to_rune`, `lookup_function`, `get_instantiation_bounds`, `get_parent_impls_for_sub_citizen_template`, `get_child_impls_for_super_interface_template`, `struct_declared`, `lookup_mutability`, `lookup_sealed`, `interface_declared`, `lookup_struct`, `lookup_struct_template`, `lookup_interface`, `lookup_interface_by_template_name`, `lookup_citizen_by_template_name`, `lookup_citizen_by_tt`, `get_all_structs`, `get_all_interfaces`, `get_all_functions`, `get_all_impls`, `get_env_for_function_signature`, `get_outer_env_for_type`, `get_inner_env_for_type`, `get_inner_env_for_function`, `get_outer_env_for_function`, `get_return_type_for_signature`, `get_kind_exports`, `get_function_exports`, `get_kind_externs`, `get_function_externs`.

If a method appears in the `&mut self` list but its Scala body reveals it's actually read-only (or vice-versa), trust the Scala body. These lists are defaults, not laws.

### Derive / lifetime bounds

- Every `impl` block gets `<'s, 't>` with `where 's: 't` — matches the bound on the struct itself.
- No new derives (you're not defining new types; just adding methods).
- `pub fn` (not `fn`) — Scala `def` is public by default; Rust method visibility defaults to private, so make them `pub` explicitly.

---

## Shape of a lifted stub

Before (current file):

```rust
fn declare_function_outer_env<'s, 't>(name_t: IdT<'s, 't>, env: IInDenizenEnvironmentT<'s, 't>) { panic!("Unimplemented: declare_function_outer_env"); }
/*
  def declareFunctionOuterEnv(
    nameT: IdT[IFunctionTemplateNameT],
    env: IInDenizenEnvironmentT,
  ): Unit = {
    vassert(!functionNameToOuterEnv.contains(nameT))
    //    vassert(nameT == env.fullName)
    functionNameToOuterEnv += (nameT -> env)
  }
*/
```

After (Slab 8):

```rust
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_function_outer_env(
        &mut self,
        name_t: IdT<'s, 't>,
        env: &'t IInDenizenEnvironmentT<'s, 't>,
    ) {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def declareFunctionOuterEnv(
    nameT: IdT[IFunctionTemplateNameT],
    env: IInDenizenEnvironmentT,
  ): Unit = {
    vassert(!functionNameToOuterEnv.contains(nameT))
    //    vassert(nameT == env.fullName)
    functionNameToOuterEnv += (nameT -> env)
  }
*/
```

Changes applied: free-fn → one-fn `impl` block; `&mut self` added (Scala `+=`); `env` params flipped to `&'t IInDenizenEnvironmentT`; panic message bumped to Slab 10; `<'s, 't>` moved from the `fn` to the surrounding `impl` header; `pub` added. Scala `/* */` block unchanged.

---

## Gotchas

### Gotcha 1 (critical): envs are `&'t IInDenizenEnvironmentT`, not `&'s IEnvironmentT`

Same rule as Slab 6 Gotcha 1. Every env parameter in every method (there are ~10 of them — `declare_function_inner_env`, `declare_function_outer_env`, `declare_type_outer_env`, `declare_type_inner_env`, `get_outer_env_for_type`, `get_inner_env_for_type`, `get_inner_env_for_function`, `get_outer_env_for_function`) becomes `&'t IInDenizenEnvironmentT<'s, 't>`. Never `&'s IEnvironmentT`. **Not even once.** If quest.md §§ on envs show `&'s IEnvironmentT`, quest.md is out-of-date post-Slab-4.

The one exception is `get_env_for_function_signature`, which returns the specific `FunctionEnvironmentT` (a concrete env type, not the wrapper enum). That stays `&'t FunctionEnvironmentT<'s, 't>`.

### Gotcha 2: `IdT<'s, 't>` is by value, not `&'t IdT`

Slab 2 made `IdT` monomorphic and `Copy` with pointer-identity `Hash`/`Eq`. Pass it by value everywhere. This differs from `PrototypeT` / `SignatureT` / arena-allocated structs which are passed by `&'t`.

Easy memory trick: the `PtrKey<'t, IdT<'s, 't>>` fields in `CompilerOutputs` wrap `IdT` by value (via `&'t IdT` inside the `PtrKey`, but the public boundary is by value). When you're building method signatures, just take `IdT` by value.

### Gotcha 3: arena refs (`&'t`) on definition types

Scala returns `StructDefinitionT` or `FunctionDefinitionT` from lookups; that's the JVM hiding the fact that these live on the heap and are passed by reference. Rust is explicit — definition types are arena-allocated (Slab 3/5) and passed by `&'t X<'s, 't>`. Every lookup returns `Option<&'t X<'s, 't>>` or `&'t X<'s, 't>` (for `vassertSome`-style lookups). Every "add" takes `&'t X<'s, 't>` as the parameter.

The internal HashMap fields in `CompilerOutputs` already store `&'t X<'s, 't>` values (Slab 6). Method signatures match.

### Gotcha 4: `DeferredActionT` peek returns a ref

`peek_next_deferred_function_body_compile(&self) -> Option<&DeferredActionT<'s, 't>>`. Not `Option<DeferredActionT<'s, 't>>`. The action stays in the `VecDeque`; peek borrows. `pop_front()` would move-out, but that's the "mark compiled" method's job (Slab 10 body migration).

### Gotcha 5: `add_instantiation_bounds` needs `TypingInterner`, not `Interner`

The Scala `addInstantiationBounds(interner: Interner, ...)` method's `Interner` is the typing-side canonicalizer in the Rust split. Use `&'t TypingInterner<'s, 't>` as the parameter type.

Why not `&'ctx Interner<'s>` (scout)? Because the body reaches into `TemplataCompiler.getRootSuperTemplate(interner, callingTemplateId)` where `callingTemplateId` is typing-side (`IdT[ITemplateNameT]`). The interner used for typing-side canonicalization is `TypingInterner`.

If Slab 10 body migration discovers it actually needs the scout interner, patch then. For now, `&'t TypingInterner<'s, 't>` compiles and matches the slab-4/6 convention.

### Gotcha 6: `HashMap` return in `get_instantiation_name_to_function_bound_to_rune`

Scala `getInstantiationNameToFunctionBoundToRune(): Map[IdT[IInstantiationNameT], InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]] = instantiationNameToInstantiationBounds.toMap` returns an immutable copy of the internal map.

Rust return type: `HashMap<PtrKey<'t, IdT<'s, 't>>, &'t InstantiationBoundArgumentsT<'s, 't>>` — **matches the internal field type from Slab 6**, not Scala's `Map[IdT, InstantiationBoundArgumentsT]` directly.

Why: the internal map is `HashMap<PtrKey<'t, IdT>, &'t InstantiationBoundArgumentsT>`. Slab 10 body migration will do `self.instantiation_name_to_bounds.clone()` — cloning the `HashMap` is cheap (`PtrKey` is `Copy`, `&'t _` is `Copy`, so `HashMap<Copy, Copy>` cloning is O(n) memcpy of the bucket array with no per-entry clones). The alternative — stripping `PtrKey` and returning `HashMap<IdT<'s, 't>, &'t InstantiationBoundArgumentsT<'s, 't>>` — would force a re-key-build on every call, and we'd lose the pointer-identity semantics that `PtrKey` provides.

Keep `PtrKey` in the return. Slab 10 will decide whether callers actually need it or whether an iterator is better — don't pre-optimize.

### Gotcha 7: `pub fn`, not `fn`

All Slab 6 free-fn stubs are private (`fn`). Scala `def` is public by default. Slab 8's lifted methods get `pub fn`. Don't leave them private — callers in other files need to reach them once Slab 10 bodies migrate.

### Gotcha 8: bodies stay `panic!()` — no exceptions

Gotcha 10 from Slab 6, but with the slab number bumped. You will be tempted. Several of the 54 methods have Scala bodies that are one-liners (`signatureToFunction.get(signature)`, `returnTypesBySignature += (sig -> ret)`, `kindExports += KindExportT(...)`). **Don't port them.** Reasons:

- Consistency: if you port three trivial ones, the reviewer has to trust that the other 51 are also correctly left alone. If you port zero, the reviewer's job is O(1).
- Semantics have subtle `PtrKey`-wrapping decisions (see `lookup_function`: is it `.get(&PtrKey(signature))` or `.get(signature)` or `.values().find(...)`?). Getting those wrong means Slab 10 body migration has to audit your work alongside its own.
- Slab 10 is the body-migration slab; it'll handle all 54 together in one reviewable pass.

Bump the panic message from `"Unimplemented: <method_name>"` to `"Unimplemented: Slab 10 — body migration"` so the next reader knows exactly what's deferring.

### Gotcha 9: one-fn impl blocks per TL-HANDOFF convention

Each lifted method gets its own `impl<'s, 't> CompilerOutputs<'s, 't> where 's: 't { pub fn foo(...) -> ... { ... } }` block. Do NOT consolidate all 54 methods into a single giant impl block. Why: each method has an adjacent Scala `/* def ... */` anchor block below it; one-fn-per-impl preserves the method-to-anchor adjacency.

This matches the style Slab 7 established for `compile_program` and `drain_all_deferred` on `Compiler`, and Slab 6's `CompilerOutputs::new`.

### Gotcha 10: Scala `/* */` blocks are frozen — pre-commit hook enforces

Same rule as every prior slab. `.claude/hooks/check-scala-comments` does exact-match comparison on every `/* ... */` block in the file. If your editor auto-indents whitespace inside a Scala block while you're editing the Rust stub directly above it, the pre-commit hook will bounce. Revert the whitespace and retry.

You're deleting Rust stubs and writing new impl blocks above Scala anchors. The Scala content should never change.

### Gotcha 11: no downstream breakage expected

The Slab 6 free-fn stubs are private (`fn`, not `pub fn`), take value-typed params, and have panic bodies. Nothing in the codebase currently calls them — they aren't reachable symbols. Lifting them into `impl CompilerOutputs` doesn't break any existing call sites (there are none).

This means **every compile error you see after a lift is a signature mistranslation**, not a downstream-call-site issue. If `cargo check --lib` flags an error, re-read the Scala block and fix the translation. Don't add `panic!("Unimplemented: Slab 10")` patches in other files — nothing should need them.

### Gotcha 12: `#[allow(unused_variables, unused_imports)]` is project-level

The crate sets `#![allow(unused_variables, unused_imports)]`, so you'll get no warnings for params that aren't read in the panic body. This is intentional during mid-migration. Don't prefix unused params with `_` — Slab 10 will need them by name.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-8.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-8.txt
```

Must print `0`. If not, stop and ask the senior.

### Step 2: Lift in file order

Walk the 54 stubs top-to-bottom in `src/typing/compiler_outputs.rs`. The first stub (`count_denizens`) is around line 229; the last (`get_function_externs`) is around line 842. For each stub:

1. Read the Scala `/* def ... */` block directly beneath it.
2. Classify the receiver (`&self` or `&mut self`) using the defaults list above. Verify against the Scala body — look for `+=`, `-=`, `.put(...)`, `.remove(...)` on field names.
3. Translate each parameter and return type using the translation-rules table. Pay particular attention to env params (Gotcha 1), arena-ref returns (Gotcha 3), and `IdT` by-value (Gotcha 2).
4. Replace the single-line free-fn stub with a one-fn `impl` block:
   ```rust
   impl<'s, 't> CompilerOutputs<'s, 't>
   where 's: 't,
   {
       pub fn <method_name>(
           &self-or-&mut-self,
           // ... params on their own lines ...
       ) -> <return_type> {
           panic!("Unimplemented: Slab 10 — body migration");
       }
   }
   ```
5. Leave the Scala `/* */` block directly below untouched.

### Step 3: Incremental verification

After every ~10 stubs, run:

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-8.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-8.txt
```

Fix any errors before moving on. Common mistakes:

- Missing `where 's: 't` on the `impl` header.
- `IdT<'s, 't>` accidentally wrapped in `&'t` — it's by value.
- `IEnvironmentT` instead of `IInDenizenEnvironmentT` — Slab-4 override.
- `FunctionDefinitionT` without `&'t` — definitions are arena-allocated.
- `<'s, 't>` left on the `fn` instead of hoisted to the `impl` header.

### Step 4: Final verification

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-8.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-8.txt        # must be 0
tail -3 /tmp/sylvan-slab-8.txt                  # must show "Finished"

grep -c '^fn ' FrontendRust/src/typing/compiler_outputs.rs
# ^ must be 0 (all stubs lifted; only pub fn inside impl blocks remain)

grep -cE '^impl<.s, .t> CompilerOutputs' FrontendRust/src/typing/compiler_outputs.rs
# ^ must be >= 55 (54 lifted + 1 Slab-6 new())

grep -cE 'Slab 10 — body migration' FrontendRust/src/typing/compiler_outputs.rs
# ^ must be 54 (every panic message bumped)

grep -c '^    pub fn ' FrontendRust/src/typing/compiler_outputs.rs
# ^ must be >= 55
```

### Step 5: Diff self-review

```bash
git diff FrontendRust/src/typing/compiler_outputs.rs
```

Confirm:

- Only stub rewrites and new `impl` wrappers.
- No edits inside Scala `/* */` blocks.
- No other files touched.
- Panic messages all bumped.
- No stray `#[derive(Debug)]` or other new annotations.

### Step 6: Hand off

**Never commit. The human handles all commits and tags.** When `cargo check --lib` is clean, hand back uncommitted. The human tags `slab-8-complete`.

Work-order checkpoints:

- Step 2 — 10 stubs lifted, incremental check clean.
- Step 2 (continued) — 30 stubs lifted.
- Step 2 (final) — all 54 stubs lifted.
- Step 4 — verification greps pass.
- Step 5 — diff self-review.
- Step 6 — hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors.
- `src/typing/compiler_outputs.rs` has all 54 methods inside one-fn `impl<'s, 't> CompilerOutputs<'s, 't> where 's: 't { pub fn ... { panic!(...) } }` blocks.
- Every env param is `&'t IInDenizenEnvironmentT<'s, 't>` (with `FunctionEnvironmentT` a concrete exception).
- Every definition return is `&'t X<'s, 't>` or `Option<&'t X<'s, 't>>`.
- `IdT<'s, 't>` passed by value everywhere.
- Panic messages all read `"Unimplemented: Slab 10 — body migration"`.
- Scala `/* */` blocks unchanged byte-for-byte.
- No other file modified.
- Handed back uncommitted; human tags `slab-8-complete`.

---

## When you're stuck

- **"the trait `Hash` is not implemented for `&'t FunctionDefinitionT`"**: you used `&'t FunctionDefinitionT` as a HashMap key. Inside signatures, definition types appear as HashMap *values*, not keys. Keys are `PtrKey<'t, IdT>` / `PtrKey<'t, SignatureT>` / etc. Re-check whether the error is complaining about a parameter or a return type; keys in returns should be wrapped.
- **"`'s` may not live long enough"**: missing `where 's: 't` on the `impl` header. Every `impl` block over `CompilerOutputs` needs it.
- **"cannot find type `PtrKey`"**: Slab 6 already added `use crate::typing::ptr_key::PtrKey;` at the top of the file — verify it's still there. If missing, add it.
- **"mismatched types: expected `&'t IInDenizenEnvironmentT`, found `IInDenizenEnvironmentT`"**: you translated an env param as a value type. Always `&'t`.
- **"I want to port `lookup_function` because the body is one line"**: don't. Gotcha 8. Every body stays `panic!("Unimplemented: Slab 10 — body migration")`.
- **"I want to consolidate the 54 impls into one `impl CompilerOutputs { ... }` block"**: don't. Gotcha 9. One fn per impl block, adjacency to Scala anchor preserved.
- **"`KindT` / `InterfaceTT` / `StructTT` — value or `&'t`?"**: check Slab 3 — most are `Copy`-friendly tagged pointers. Default by value. If it doesn't compile, fall back to `&'t X<'s, 't>`.
- **"`add_instantiation_bounds` has `interner: Interner<'s>` in the stub — should I change it?"**: yes. Flip to `&'t TypingInterner<'s, 't>`. See Gotcha 5.
- **"Scala body has a `vassert` I'm not sure how to classify — is it mutation?"**: `vassert` is just an assertion; doesn't mutate. Look past it to the rest of the body. If the body otherwise has no `+=`/`.put(...)`, it's `&self`.
- **"pre-commit hook rejection on a `/* */` block"**: you accidentally edited whitespace inside a Scala block. Revert to byte-for-byte original.
- **"I want to touch `templata_compiler.rs`"**: don't. Slab 9.
- **"I want to fill the `compiler.rs` residual `()` placeholders"**: don't. Slab 11.
- **"a Slab 2–7 file looks broken in some way"**: it isn't. Ask the senior if you disagree.

## Where to file questions

- **Design**: `FrontendRust/docs/migration/handoff-slab-6.md` is the spec for the struct shape this slab lifts methods onto. If there's disagreement between this doc and Slab 6, **Slab 6 wins** on struct shape; **this doc wins** on method signatures (because Slab 6 explicitly deferred method signatures to Slab 8).
- **Scala semantics**: each Scala `/* def ... */` block beneath a stub is the spec. The body tells you receiver kind (`+=` → `&mut self`). The signature tells you parameter and return types.
- **Hook rejections**: read the hook's diff output. Usually accidental whitespace inside `/* */`.

## Final advice

This is narrow, mechanical work. The hardest part is resisting body migration on obvious one-liners. Resist. Slab 10 owns bodies.

Two facts to anchor you:
1. **Every method body in this slab is `panic!("Unimplemented: Slab 10 — body migration");`**. No exceptions.
2. **Every env param is `&'t IInDenizenEnvironmentT<'s, 't>`**. Slab-4 override, non-negotiable.

If you get those two right and follow the translation table, the 54 stubs become 54 one-fn impl blocks in a few hours. If you find yourself writing non-panic code in a method body, stop and re-read Gotcha 8.

After Slab 8, the next data-def-adjacent slab is Slab 9 (`templata_compiler.rs`, 35 stubs, with the open question of whether to host as unit struct or lift into `impl Compiler`). Slab 10 is the first body-migration slab. Slab 11 is the `compiler.rs` residual `()`-flip sweep.

Good luck.
