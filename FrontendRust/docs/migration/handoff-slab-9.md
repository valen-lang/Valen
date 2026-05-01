# Handoff: Typing Pass Slab 9 — `TemplataCompiler` (Scala `object`) Method Signature Rewrite

> **Post-completion note (2026-04-20):** This doc was written before the audit that expanded the signature-rewrite phase. References to "Slab 10 = body migration" and "Slab 11 = compiler.rs residuals" are **stale**. Current numbering per `quest.md` §12.1: Slabs 10-13 are all additional signature-rewrite slabs (~122 sub-compiler method sigs still to fill); body migration is now **Slab 14+**. The `panic!("Unimplemented: Slab 10 — body migration")` messages in the code will be bulk-updated when bodies actually land. See TL-HANDOFF.md for the current slab roadmap.

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0-8 are done:

- **Slabs 0-7** — arena substrate, names, types, templatas, envs, AST, `CompilerOutputs` data shape, `HinputsT`/`Compiler` scaffolding. All tagged `slab-N-complete`.
- **Slab 8** — lifted all 54 `CompilerOutputs` method stubs in `compiler_outputs.rs` into `impl<'s, 't> CompilerOutputs<'s, 't>` blocks with real receivers/parameters/returns. Bodies stay `panic!("Unimplemented: Slab 10 — body migration")`.

Slab 8 established the **signature-rewrite pattern** for this phase of migration. Slab 9 is the second signature-rewrite slab and continues the pattern on a different file.

**Slab 9 scope:** lift the **35 panic-stub free-functions** at the head of `FrontendRust/src/typing/templata_compiler.rs` (lines 88-996) into `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { ... }` blocks with proper receiver (`&self`), lifetimes, arena refs (`&'t`), and `()`-placeholder flips. Bodies stay `panic!()`. Body migration is Slab 10+. This is narrow, mechanical work — budget ~3 hours focused.

**Read these first in this order**, then come back:

1. `FrontendRust/docs/migration/handoff-slab-8.md` — the prior signature-rewrite slab. The translation-rules table, receiver-classification rules, and gotchas from Slab 8 apply directly. Slab 9 is the same pattern on a different file.
2. `TL-HANDOFF.md` at repo root — the file-layout standards section ("one fn per impl block, multi-line body").
3. `FrontendRust/src/typing/templata_compiler.rs` lines 1034-1524 — the **14 already-lifted `impl Compiler` blocks** at the tail of the file. These are your style template. Slab 9 adds 35 more just like them.
4. This doc.

You shouldn't need to read the Scala source externally — every Scala `def` sig is already embedded inline in the `/* def ... */` blocks beneath each Rust stub in `templata_compiler.rs`. Those blocks are authoritative.

---

## The big picture: why Slab 9 exists

`templata_compiler.rs` mirrors two Scala entities:

- **`object TemplataCompiler`** (the Scala companion object) — 35 static utility methods for manipulating `IdT` templates, substituting templatas, and assembling rune-to-bound maps. **These are Slab 9's targets.** Current state: 35 panic-stub free-fns at file scope, lines 88-996.
- **`class TemplataCompiler(...)`** (the Scala instance class) — ~14 methods that used state (delegate, nameTranslator). **Already lifted into `impl Compiler`** at lines 1034-1524 in prior work.

The split was unusual in Scala (object + class with the same name) but the design decision has already been made on the Rust side: **both go onto `Compiler<'s, 'ctx, 't>`**. Why?

1. The god-struct refactor pushed every sub-compiler's methods onto `Compiler` so `Compiler` holds `scout_arena`, `typing_interner`, `keywords`, `opts` and methods access them via `&self.typing_interner` etc. instead of passing them as parameters.
2. The already-lifted 14 impl blocks at the tail of the same file establish the pattern for this file specifically.
3. Several "static" Scala methods (like `getFunctionTemplate(id: IdT[IFunctionNameT])`) *appear* stateless but in Rust require the typing interner to re-canonicalize the rebuilt `IdT`. Passing `&self` gives them access to `self.typing_interner` without adding a parameter.
4. Other "static" methods take `interner: Interner` and `keywords: Keywords` explicitly — on Rust those drop to `&self.typing_interner` / `&self.keywords`, shrinking parameter lists from 9-10 args to 5-6.

So: **every free-fn stub becomes a `&self` method on `Compiler`**. Slab 9 follows the existing pattern.

By the end of Slab 9:

- `cargo check --lib` passes with 0 errors.
- `src/typing/templata_compiler.rs`:
  - All 35 free-fn stubs at lines 88-996 are **deleted**. Their `panic!()` bodies reappear inside one-fn `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn foo(&self, ...) -> ... { panic!(...) } }` blocks, each placed where the stub was (directly above the Scala `/* def ... */` anchor).
  - Each method has a correct receiver (`&self`), parameters/returns translated per the rules below, and `pub` visibility.
  - Panic message bumped to `panic!("Unimplemented: Slab 10 — body migration");`.
  - Scala `/* */` blocks unchanged byte-for-byte.
- No other file touched.

**What Slab 9 is NOT:**

- No body migration. Bodies stay `panic!()`. Slab 10+.
- No `IBoundArgumentsSource` refactor. The current `pub trait IBoundArgumentsSource<'s, 't> {}` marker trait with two empty-impl unit structs is **dysfunctional for the body pattern-match** in Scala (body patterns on `InheritBoundsFromTypeItself` vs `UseBoundsFromContainer(params, args)`) — but Slab 9 is signatures only. Use `&'t dyn IBoundArgumentsSource<'s, 't>` in parameter slots. Slab 10 body migration decides whether to flip to an enum.
- No fill-in of the empty `UseBoundsFromContainer` unit struct. Its Scala variant carries two `InstantiationBoundArgumentsT` fields; Rust side is still empty. Leave alone — Slab 10.
- No touching of the 14 already-lifted `impl Compiler` blocks at lines 1034-1524.
- No `compiler.rs` residual `()`-flip sweep (Slab 11).
- No `local_helper.rs` orphan (Slab 11).

---

## What's already in place (don't duplicate; don't delete)

Open `src/typing/templata_compiler.rs` (~1540 lines). Structure:

- **Lines 1-29**: imports + big Scala `/* */` block header + Scala's `sealed trait IBoundArgumentsSource` block.
- **Line 30**: `pub trait IBoundArgumentsSource<'s, 't> {}` (marker trait).
- **Line 34**: `pub struct InheritBoundsFromTypeItself;` (empty unit struct + Scala anchor).
- **Line 39**: `pub struct UseBoundsFromContainer;` (empty unit struct, **Scala variant has two InstantiationBoundArgumentsT fields — not yet filled on Rust side; leave alone**).
- **Lines 47-85**: deleted delegate-trait comment + Scala `trait ITemplataCompilerDelegate` anchor.
- **Line 86-87**: Scala `object TemplataCompiler {` comment header.
- **Lines 88-996**: **35 free-fn panic stubs** — your targets.
- **Lines 1034-1524**: **14 already-lifted `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>` blocks** — DO NOT MODIFY. These are the Scala `class TemplataCompiler(...)` methods already migrated.

The 35 stubs fall into three rough families:

1. **Small IdT-transform helpers** (pure name/template manipulation): `get_top_level_denizen_id`, `get_placeholder_templata_id`, `get_function_template`, `get_citizen_template`, `get_name_template`, `get_super_template`, `get_root_super_template`, `get_template`, `get_sub_kind_template`, `get_super_kind_template`, `get_struct_template`, `get_interface_template`, `get_export_template`, `get_extern_template`, `get_impl_template`, `get_placeholder_template`. ~16 methods.
2. **Rune-to-bound assemblers**: `assemble_predict_rules`, `assemble_call_site_rules`, `assemble_rune_to_function_bound`, `assemble_rune_to_impl_bound`. 4 methods.
3. **Substitution engine** (big signatures, mutate `coutputs`): `substitute_templatas_in_coord`, `substitute_templatas_in_kind`, `substitute_templatas_in_struct`, `translate_instantiation_bounds`, `substitute_templatas_in_impl_id`, `substitute_templatas_in_bounds`, `substitute_templatas_in_interface`, `substitute_templatas_in_templata`, `substitute_templatas_in_prototype`, `substitute_templatas_in_function_bound_id`, `get_placeholder_substituter`, `get_placeholder_substituter_ext`, `get_reachable_bounds`, `get_first_unsolved_identifying_rune`, `create_rune_type_solver_env`. 15 methods.

Total: 35.

### Things NOT in scope (don't touch)

- Any file outside `src/typing/templata_compiler.rs`. Slabs 2-8 are frozen. Compiler.rs residuals are Slab 11.
- The 14 already-lifted `impl Compiler` blocks at lines 1034-1524. They have their own Slab 10+ body migration.
- The `pub trait IBoundArgumentsSource<'s, 't> {}` — the marker trait is a known design issue. Pass as `&'t dyn IBoundArgumentsSource<'s, 't>` in signatures; body migration fixes the pattern-match story.
- The empty `pub struct UseBoundsFromContainer;` — Scala variant has two fields; Slab 10 or later fills them.
- The `pub struct InheritBoundsFromTypeItself;` — unit struct; leave as-is.
- Scala `/* */` blocks anywhere in the file.
- File-level imports — add to them only if you truly need a new type (most are already in scope since the file already has `use crate::typing::compiler::Compiler;` and the tail impls use the full type-alphabet).

---

## Signature translation rules

Same table as Slab 8, with additions specific to this file. For each stub, read the Scala `/* def ... */` block beneath it, translate each parameter and return type, and classify the receiver.

### The Slab-8 rules (reuse)

| Scala | Rust |
|---|---|
| `SignatureT` | `&'t SignatureT<'s, 't>` (Slab 3 interned) |
| `PrototypeT[IFunctionNameT]` / `PrototypeT[T <: IFunctionNameT]` | `&'t PrototypeT<'s, 't>` (phantom `[T]` erased per Slab 3) |
| `IdT[INameT]` / `IdT[ITemplateNameT]` / `IdT[IFunctionNameT]` / `IdT[T <: ...]` | `IdT<'s, 't>` by value (monomorphic per Slab 2, `Copy` with pointer-identity `Hash`/`Eq`); **all phantom `[T]` parameters erased** |
| `CoordT` | `CoordT<'s, 't>` by value |
| `ITemplataT[X]` / `ITemplataT[ITemplataType]` | `ITemplataT<'s, 't>` by value (Copy; phantom `[X]` erased) |
| `RangeS` | `RangeS<'s>` by value |
| `List[RangeS]` / `Vector[RangeS]` | `&[RangeS<'s>]` |
| `StrI` | `StrI<'s>` by value |
| `PackageCoordinate` | `PackageCoordinate<'s>` by value |
| `Boolean` / `Int` / `Unit` | `bool` / `i32` / `()` or no return |
| `IInDenizenEnvironmentT` | `&'t IInDenizenEnvironmentT<'s, 't>` (**Slab-4 override**, never `&'s IEnvironmentT`) |
| `FunctionEnvironmentT` | `&'t FunctionEnvironmentT<'s, 't>` |
| `StructDefinitionT` / `InterfaceDefinitionT` / `ImplT` / `FunctionDefinitionT` / `CitizenDefinitionT` | `&'t X<'s, 't>` |
| `Interner` (explicit param on Scala methods) | **dropped**; use `self.typing_interner` in the body. See Gotcha 2. |
| `Keywords` (explicit param on Scala methods) | **dropped**; use `self.keywords` in the body. See Gotcha 2. |
| `CompilerOutputs` (param) | `&mut CompilerOutputs<'s, 't>` (most substitute_* methods mutate via `.addInstantiationBounds`; even read-only uses like `.getInstantiationBounds` take `&self` on `CompilerOutputs`, but the param `coutputs` still needs `&mut` if ANY call in the body mutates — conservative choice: `&mut`) |
| `InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]` | `&'t InstantiationBoundArgumentsT<'s, 't>` (phantoms erased; matches Slab 7 flip) |
| `InstantiationReachableBoundArgumentsT[FunctionBoundNameT]` | `&'t InstantiationReachableBoundArgumentsT<'s, 't>` (phantom erased) |
| `InstantiationReachableBoundArgumentsT[BF]` (generic position) | `InstantiationReachableBoundArgumentsT<'s, 't>` (by value — this is a return type in `translateInstantiationBounds`) |

### Slab-9-specific additions

| Scala | Rust |
|---|---|
| `GenericParameterS` | `GenericParameterS<'s>` by value (**scout-side** type, `Copy`? verify; if not Copy, use `&'s GenericParameterS<'s>`) — **default to `&'s GenericParameterS<'s>`** |
| `Vector[GenericParameterS]` | `&'s [GenericParameterS<'s>]` |
| `IRulexSR` | `IRulexSR<'s>` by value (**scout-side** enum, `Copy`; if not, `&'s IRulexSR<'s>`) — **default to `&'s IRulexSR<'s>`** |
| `Vector[IRulexSR]` | `&'s [IRulexSR<'s>]` |
| `IRuneS` | `IRuneS<'s>` by value (scout-side, `Copy`) |
| `TemplatasStore` (the Scala type) | `&'t TemplatasStoreT<'s, 't>` (defined in `src/typing/env/environment.rs:332`, arena-allocated) |
| `INameT` | `INameT<'s, 't>` by value (enum of name variants; most are `Copy`. If not, `&'t INameT<'s, 't>`) — **default to `INameT<'s, 't>` by value** |
| `ICitizenTT` | `ICitizenTT<'s, 't>` by value (Copy enum wrapping `&'t StructTT` / `&'t InterfaceTT`) |
| `IBoundArgumentsSource` (the trait param) | `&'t dyn IBoundArgumentsSource<'s, 't>` — see Gotcha 3 |
| `IPlaceholderSubstituter` (return type, trait) | `&'t dyn IPlaceholderSubstituter<'s, 't>` — see Gotcha 4 (may need to define the trait; check if already exists) |
| `IRuneTypeSolverEnv` (return type, trait) | `&'t dyn IRuneTypeSolverEnv<'s>` (defined in postparsing if it exists) — see Gotcha 5 |
| `Option[(GenericParameterS, Int)]` | `Option<(&'s GenericParameterS<'s>, i32)>` |
| `isSolved: IRuneS => Boolean` (Scala closure param) | `&mut dyn FnMut(IRuneS<'s>) -> bool` — a closure parameter. See Gotcha 6 |
| `Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError]` (return inside closure) | Left alone — it's inside a Scala body that stays panic-stubbed |

### Receiver classification

All 35 methods on `&self`. None need `&mut self` because:
- The "mutation" happens via `coutputs: &mut CompilerOutputs` parameter, not on `Compiler`.
- `Compiler` holds only immutable refs (`scout_arena`, `typing_interner`, `keywords`, `opts`).

### Derive / lifetime bounds

- Every `impl` block gets `<'s, 'ctx, 't>` with `where 's: 't` — matches the bound on `Compiler` and matches the existing 14 impl blocks at lines 1034-1524.
- `pub fn` (not `fn`).
- No new derives.

---

## Shape of a lifted stub

Before (current file, line 186):

```rust
fn get_super_template() { panic!("Unimplemented: get_super_template"); }
/*
  def getSuperTemplate(id: IdT[INameT]): IdT[INameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      getNameTemplate(last))
  }
*/
```

After (Slab 9):

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_super_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getSuperTemplate(id: IdT[INameT]): IdT[INameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      getNameTemplate(last))
  }
*/
```

Bigger example: `substitute_templatas_in_coord`:

Before:

```rust
fn substitute_templatas_in_coord() { panic!("Unimplemented: substitute_templatas_in_coord"); }
/*
  def substituteTemplatasInCoord(
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],
    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    coord: CoordT):
  CoordT = { ... }
*/
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_coord(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: &'t dyn IBoundArgumentsSource<'s, 't>,
        coord: CoordT<'s, 't>,
    ) -> CoordT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/* ... Scala block unchanged ... */
```

Note: `interner` and `keywords` parameters are **dropped** — they come from `self.typing_interner` and `self.keywords` in the body (Slab 10). Parameter count drops from 9 to 7.

---

## Gotchas

### Gotcha 1: envs are `&'t IInDenizenEnvironmentT`, not `&'s IEnvironmentT`

Same as Slab 8 Gotcha 1. The `create_rune_type_solver_env` method takes `parentEnv: IInDenizenEnvironmentT` — translate to `parent_env: &'t IInDenizenEnvironmentT<'s, 't>`. **Never** `&'s IEnvironmentT`.

### Gotcha 2: drop `interner: Interner` and `keywords: Keywords` parameters

Scala methods on the `object TemplataCompiler` take these as explicit parameters because Scala objects don't have a `this` to store infrastructure state. In Rust, `Compiler<'s, 'ctx, 't>` holds `typing_interner: &'ctx TypingInterner<'s, 't>` and `keywords: &'ctx Keywords<'s>` as fields. Methods access via `self.typing_interner` / `self.keywords`.

**Remove both from the Rust parameter list when lifting.** Slab 10 body migration will use `self.typing_interner.intern_...(...)` in the body.

Exception: `getRootSuperTemplate(interner: Interner, id: IdT[INameT])` also takes interner. Drop it; body uses `self.typing_interner`.

### Gotcha 3: `IBoundArgumentsSource` is a marker trait, not a pattern-matchable enum

The current Rust type is `pub trait IBoundArgumentsSource<'s, 't> {}` with two empty-impl unit structs. This is **dysfunctional** for the Scala body's pattern match:

```scala
boundArgumentsSource match {
  case InheritBoundsFromTypeItself => ...
  case UseBoundsFromContainer(params, args) => ...
}
```

The trait has no `.variant()` method and the unit structs have no fields. A `&dyn IBoundArgumentsSource` can't be pattern-matched in Rust.

**Slab 9 treatment:** pass as `bound_arguments_source: &'t dyn IBoundArgumentsSource<'s, 't>` in every signature. This compiles. Slab 10 body migration decides the fix — likely converting to an enum.

Do NOT flip the trait to an enum in Slab 9. That's a breaking structural change that touches multiple files. Defer.

### Gotcha 4: `IPlaceholderSubstituter` — probably doesn't exist yet as a Rust type

Scala has `trait IPlaceholderSubstituter { def substituteForCoord(...): CoordT; ... }` — see the commented block at lines 842-860 showing the Scala trait definition. It's the return type of `getPlaceholderSubstituter` and `getPlaceholderSubstituterExt`.

**Check first**: does `pub trait IPlaceholderSubstituter<'s, 't>` exist anywhere in the project? Grep for it. If yes, return `&'t dyn IPlaceholderSubstituter<'s, 't>`.

If no, **define a minimal empty trait** in templata_compiler.rs just below the `IBoundArgumentsSource` definition:

```rust
pub trait IPlaceholderSubstituter<'s, 't> {}
```

And place the Scala `/* trait IPlaceholderSubstituter { ... } */` block above it (moving the Scala anchor from its current commented-out position at lines 842-860 — actually don't move; leave the Scala blocks in place, just put the empty trait nearby).

Hmm, that's a file-restructure. **Simpler alternative**: return `-> ()` as the Rust signature for `get_placeholder_substituter` and `get_placeholder_substituter_ext` in Slab 9, with a `// TODO: Slab 10 — return &'t dyn IPlaceholderSubstituter<'s, 't> after defining the trait` comment. This matches prior slabs' treatment of missing types (e.g. `ICompileErrorT`'s `_Phantom` placeholder).

**Recommend**: use `-> ()` with a TODO comment. Keeps Slab 9 narrow. Slab 10 defines the trait and re-flips.

### Gotcha 5: `IRuneTypeSolverEnv` — check if it exists

Same question as Gotcha 4. Scala `createRuneTypeSolverEnv` returns `IRuneTypeSolverEnv`. Grep for `IRuneTypeSolverEnv` in the Rust codebase.

If it exists (likely defined in `src/postparsing/` or similar), return `&'t dyn IRuneTypeSolverEnv<'s>` or similar.

If not, return `-> ()` with a TODO comment. Don't define the trait in Slab 9.

### Gotcha 6: `isSolved: IRuneS => Boolean` closure param

`get_first_unsolved_identifying_rune` takes a closure parameter. Translate as:

```rust
pub fn get_first_unsolved_identifying_rune(
    &self,
    generic_parameters: &'s [GenericParameterS<'s>],
    is_solved: impl Fn(IRuneS<'s>) -> bool,
) -> Option<(&'s GenericParameterS<'s>, i32)> {
    panic!("Unimplemented: Slab 10 — body migration");
}
```

`impl Fn(...) -> ...` is the idiomatic Rust form for a non-mutating closure parameter. Don't use `Box<dyn Fn>` or `&dyn Fn` — `impl Fn` is cheaper and matches what a Slab 10 body would want.

### Gotcha 7: methods that call other `TemplataCompiler` methods in their body

Several substitute_* methods recursively call each other (e.g. `substituteTemplatasInCoord` calls `substituteTemplatasInKind`). In the Scala body these are plain static calls; in Rust they'll be `self.substitute_templatas_in_kind(...)`.

**Slab 9 doesn't care** about body translations — body stays `panic!()`. But be aware: the recursion pattern in the Scala body is what justifies `&self` on methods that don't directly use Compiler state. Even a pure-id-transform method like `get_super_template` might call `get_name_template` internally (see its Scala body) — that call becomes `self.get_name_template(...)` in Slab 10.

### Gotcha 8: `coutputs: &mut CompilerOutputs` not `&CompilerOutputs` — conservative choice

Most substitute_* methods call `coutputs.addInstantiationBounds(...)` inside the body, which is a `&mut self` method on CompilerOutputs (Slab 8 made it `&mut self`). So the parameter must be `&mut CompilerOutputs`.

Some methods (like `getReachableBounds`) only *read* from coutputs. **Conservative choice**: still pass `&mut CompilerOutputs<'s, 't>`. Two reasons:

1. Slab 10 body migration may discover the read-only method also needs to mutate transitively (calling a substitute_* that mutates).
2. Rust's borrow checker is happier with consistent mut-ness across a function family — reduces Slab 10 refactoring.

If you're confident a method is strictly read-only, use `&CompilerOutputs<'s, 't>`. Default to `&mut`.

### Gotcha 9: `IRuneS`, `GenericParameterS`, `IRulexSR` are scout-side (`'s`), not typing-side (`'t`)

These types live in `src/postparsing/`. They carry `<'s>` not `<'s, 't>`. Don't accidentally write `GenericParameterS<'s, 't>` — just `GenericParameterS<'s>`.

When passed by reference, they borrow from the scout arena: `&'s GenericParameterS<'s>` (borrow and type both scout-side).

### Gotcha 10: bodies stay `panic!()` — resist the temptation

Same as Slab 8 Gotcha 8. Several methods here have trivial Scala bodies:

- `getNameTemplate(name: INameT): INameT = name match { case x : IInstantiationNameT => x.template; case _ => name }` — 3 lines, tempting.
- `getFunctionTemplate(id: IdT[...])` — 4 lines of destructure+rebuild.

**Don't port them.** Slab 10 migrates all 35 bodies together in a reviewable pass. Consistency wins.

### Gotcha 11: one-fn impl blocks per TL-HANDOFF convention

Same as Slab 8 Gotcha 9. Each method gets its own `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn foo(...) { panic!(...) } }` block, adjacent to its Scala `/* def ... */` anchor.

Do NOT consolidate 35 methods into a single impl block. The existing 14 impl blocks at lines 1034-1524 each wrap one method — follow that pattern.

### Gotcha 12: Scala `/* */` blocks are frozen

Same as every prior slab. Pre-commit hook `.claude/hooks/check-scala-comments` enforces byte-for-byte.

### Gotcha 13: no downstream breakage expected

The 35 free-fn stubs are private (`fn`, not `pub fn`), take no arguments, have panic bodies. Nothing calls them. Lifting doesn't create new call sites; Slab 10 wires callers.

If you see a compile error after a lift, it's a signature mistranslation, not a downstream-call-site issue.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-9.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-9.txt
```

Must print `0`.

### Step 2: Quick traits check

```bash
grep -rn "pub trait IPlaceholderSubstituter" FrontendRust/src
grep -rn "pub trait IRuneTypeSolverEnv" FrontendRust/src
```

If either exists, note the path and use the real trait in signatures. If neither exists, use `-> ()` with `// TODO: Slab 10 — return &'t dyn IFoo<'s, 't>` comments for the affected methods (Gotchas 4-5).

### Step 3: Lift in file order, family-by-family

The 35 stubs fall into 3 families (see "What's already in place" above). Lift top-to-bottom — the natural file order interleaves them slightly but it's fine.

For each stub:

1. Read the Scala `/* def ... */` block beneath.
2. Drop `interner` and `keywords` parameters (Gotcha 2).
3. Translate remaining params per the rules table. `IdT` by value, `&'t` on definition types, `&mut` on `coutputs`, `&'t dyn` on trait params.
4. Translate return type.
5. Replace the single-line stub with:
   ```rust
   impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
   where 's: 't,
   {
       pub fn <name>(
           &self,
           // ... params on their own lines ...
       ) -> <ret> {
           panic!("Unimplemented: Slab 10 — body migration");
       }
   }
   ```
6. Leave the Scala `/* */` block directly below untouched.

### Step 4: Incremental verification

After every ~10 stubs:

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-9.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-9.txt
```

Fix errors before moving on. Common mistakes:
- `Interner` or `Keywords` param left in (Gotcha 2).
- `InterfaceT` without `&'t` when it's a definition-level type.
- `IEnvironmentT` instead of `IInDenizenEnvironmentT`.
- Forgetting `&mut` on `coutputs`.
- Treating `IBoundArgumentsSource` as a concrete enum rather than `&'t dyn IBoundArgumentsSource<'s, 't>`.

### Step 5: Final verification

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-9.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-9.txt        # must be 0
tail -3 /tmp/sylvan-slab-9.txt                  # must show "Finished"

grep -c '^fn ' FrontendRust/src/typing/templata_compiler.rs
# ^ must be 0 (all 35 stubs lifted; only pub fn inside impl blocks remain)

grep -cE '^impl<.s, .ctx, .t> Compiler' FrontendRust/src/typing/templata_compiler.rs
# ^ must be >= 49 (14 existing + 35 lifted)

grep -cE 'Slab 10 — body migration' FrontendRust/src/typing/templata_compiler.rs
# ^ must be 35 (every new panic message; existing 14 impl methods still say "Unimplemented: <method_name>")
```

### Step 6: Diff self-review

```bash
git diff FrontendRust/src/typing/templata_compiler.rs | head -200
git diff --stat
```

Confirm:
- Only stub rewrites and new `impl` wrappers.
- No edits inside Scala `/* */` blocks.
- No other files touched.
- Panic messages on NEW methods bumped; panic messages on EXISTING 14 impl methods unchanged.

### Step 7: Hand off

**Never commit.** Hand back uncommitted; the human tags `slab-9-complete`.

Work-order checkpoints:
- Step 2 — traits verified.
- Step 3 (1/3) — ~12 stubs lifted (family 1 IdT-transforms).
- Step 3 (2/3) — ~16 stubs lifted (add rune-to-bound assemblers).
- Step 3 (3/3) — all 35 stubs lifted (add substitution engine).
- Step 5 — verification greps pass.
- Step 6 — diff self-review.
- Step 7 — hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors.
- `src/typing/templata_compiler.rs` has all 35 "object TemplataCompiler" methods inside one-fn `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn ... (&self, ...) { panic!(...) } }` blocks.
- `interner` and `keywords` parameters dropped across all 35 lifted methods (accessed via `self.typing_interner` / `self.keywords` in bodies; bodies stay panic-stubbed).
- Every env param is `&'t IInDenizenEnvironmentT<'s, 't>`.
- Every definition return/param is `&'t X<'s, 't>`.
- `IdT<'s, 't>` passed by value everywhere.
- `coutputs` params are `&mut CompilerOutputs<'s, 't>` (conservative default).
- `IBoundArgumentsSource` parameters are `&'t dyn IBoundArgumentsSource<'s, 't>`.
- `IPlaceholderSubstituter` / `IRuneTypeSolverEnv` returns are either `&'t dyn X<'s, 't>` (if trait exists) or `-> ()` with TODO comment.
- `isSolved` closure is `impl Fn(IRuneS<'s>) -> bool`.
- Panic messages on lifted methods read `"Unimplemented: Slab 10 — body migration"`.
- The existing 14 `impl Compiler` blocks at lines 1034-1524 **unchanged**.
- Scala `/* */` blocks unchanged byte-for-byte.
- No other file modified.
- Handed back uncommitted.

---

## When you're stuck

- **"cannot find type `IBoundArgumentsSource` in this scope"**: the trait is defined at line 30 of the same file. Use `&'t dyn IBoundArgumentsSource<'s, 't>` — the trait already exists.
- **"the trait `Sized` is not implemented for `dyn IBoundArgumentsSource`"**: you passed `dyn IBoundArgumentsSource` by value. Always `&'t dyn` (borrowed).
- **"`'s` may not live long enough"**: missing `where 's: 't` on the `impl` header.
- **"lifetime mismatch: expected `&'t IInDenizenEnvironmentT<'s, 't>`, found `&'s IInDenizenEnvironmentT<'s, 't>`"**: env borrowed from wrong lifetime. Always `'t`.
- **"cannot find type `IPlaceholderSubstituter`"**: Gotcha 4. If the trait doesn't exist, return `-> ()` with a `// TODO: Slab 10 — return &'t dyn IPlaceholderSubstituter<'s, 't> after defining the trait` comment.
- **"too many arguments to function"**: you left `interner` or `keywords` in the param list. Drop them (Gotcha 2).
- **"I want to port `getSuperTemplate` because it's 5 lines"**: don't. Gotcha 10. Every body stays `panic!("Unimplemented: Slab 10 — body migration")`.
- **"I want to convert `IBoundArgumentsSource` from trait to enum so I can pattern-match"**: don't. Gotcha 3. Slab 10 decides.
- **"I want to fill in `UseBoundsFromContainer`'s fields"**: don't. Slab 10+ work.
- **"the 14 impl blocks at the tail already have panic stubs with different messages — should I update them?"**: no. Those are untouched in Slab 9. Slab 10+ body migration handles them with the rest.
- **"I want to touch compiler.rs / local_helper.rs / anywhere else"**: don't. Slab 9 is one file.

## Where to file questions

- **Design**: `FrontendRust/docs/migration/handoff-slab-8.md` is the spec for the signature-rewrite pattern this slab continues. If there's disagreement between this doc and Slab 8, **Slab 8 wins** on general pattern; **this doc wins** on file-specific details (Compiler-as-host, drop-interner-keywords, trait-param handling).
- **Scala semantics**: each Scala `/* def ... */` block beneath a stub is the spec. The body tells you parameter uses.
- **Hook rejections**: read the hook's diff output. Usually accidental whitespace inside `/* */`.

## Final advice

This is the second signature-rewrite slab in a row. The pattern from Slab 8 carries directly — just apply it to a different file. Key differences from Slab 8:

1. **Host is `Compiler`, not `CompilerOutputs`**. Methods get `&self` (not `&self` on `CompilerOutputs`).
2. **Drop `interner` and `keywords` parameters** (they live on `Compiler`).
3. **Trait parameters** (`IBoundArgumentsSource`) use `&'t dyn` — Slab 10 may convert to enum later.
4. **Closure parameter** on `get_first_unsolved_identifying_rune` — `impl Fn(IRuneS<'s>) -> bool`.

After Slab 9, the typing pass has:
- All `CompilerOutputs` methods with real sigs (Slab 8).
- All `TemplataCompiler::object` methods with real sigs (Slab 9).
- All data-def structs real (Slabs 0-7).
- All existing `impl Compiler` methods in sub-compilers with partial sigs (Slab 11 cleanup).

**Slab 10** = first body-migration pass. Start with the trivial lookups in `CompilerOutputs` to establish the pattern, then graduate to `TemplataCompiler` id-transforms, then the substitution engine.

**Slab 11** = compiler.rs residual `()`-flip sweep + local_helper.rs (2 stubs) + struct_compiler.rs orphan (1 stub).

After Slabs 8-11, the signature-rewrite phase is complete and the typing pass compiles with all methods having real parameter/return types. The remaining work (Slab 12+) is pure body migration, driven by tests.

Good luck.
