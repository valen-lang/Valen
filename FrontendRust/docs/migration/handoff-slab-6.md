# Handoff: Typing Pass Slab 6 — `CompilerOutputs`

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0–5 are done:

- **Slab 0** (arena substrate, `TypingInterner<'s, 't>` skeleton): merged.
- **Slab 1** (leaf types: `OwnershipT`/`MutabilityT`/`VariabilityT`/`LocationT` enums, primitive `KindT` payloads): merged.
- **Slab 2** (name hierarchy: ~60 concrete names, 22 sub-enums, monomorphic `IdT<'s, 't>`): tagged `slab-2-complete`.
- **Slab 3** (Kind/Coord/Templata trio: `KindT`/`ITemplataT` inline wrappers, `PrototypeT`/`SignatureT`): tagged `slab-3-complete`.
- **Slab 4** (environments + real interner bodies: 9 env types, `TemplatasStoreT`, `GlobalEnvironmentT`, ~84 per-concrete intern wrappers, `NodeEnvironmentBox` family deleted): tagged `slab-4-complete`.
- **Slab 5** (expression AST: `ReferenceExpressionTE` 48 variants, `AddressExpressionTE` 5 variants, `ExpressionTE` 2-variant wrapper, 53 payload structs; `IExpressionResultT` + `ReferenceResultT`/`AddressResultT`; `#[derive(PartialEq, Debug)]` family-wide because of `f64` in `ConstantFloatTE`): tagged `slab-5-complete`.

You're doing **Slab 6** — `CompilerOutputs<'s, 't>` in `src/typing/compiler_outputs.rs`. This is the **mutable accumulator** that the typing pass threads through every sub-compiler as `&mut coutputs`. It carries declaration registries, definition tables, env back-refs, deferred-evaluation queues, and a handful of vecs/sets. Scope is smaller than Slab 4 and closer to Slab 3 / Slab 5 — a few hundred lines of real work; expect ~3–5 hours focused.

**Read these first in this order**, then come back:

1. `quest.md` — **all of Part 4** (§§4.1–4.5). That's the design spec for this slab. Also §§1.5 ("neither arena" bullet for `CompilerOutputs`), 12.1-Slab-6 paragraph.
2. `TL-HANDOFF.md` at repo root — two pieces matter here: (a) the "`IEnvironmentT`/`IInDenizenEnvironmentT` live in `'t`, not `'s`" override (originally a Slab-4 override; it propagates to this slab — see Gotcha 1); (b) the "File layout + style standards for typing/" section that every new `pub struct` / `impl` block must follow.
3. `FrontendRust/docs/migration/handoff-slab-4.md` §§"What 'done' looks like" + Gotchas 1/10/14. Slab 4 fixed the conventions this slab inherits (`where 's: 't` on every `'t`-holding struct, wrapper enums vs concrete derives, the `'t`-vs-`'s` env lifetime, idiomatic `Copy`-out-then-mutate).
4. `Luz/shields/ArenaAllocatedStructsShouldNotContainMallocdCollections-AASSNCMCX.md` — the shield. **`CompilerOutputs` is NOT arena-allocated** (stack-owned accumulator), so the shield doesn't apply to it; `HashMap`/`HashSet`/`Vec`/`VecDeque` are fine here. This is a genuine exception to the shield, and it's worth reading the shield first so you understand *why* the exception holds (the stack-owned accumulator's drop runs destructors; the arena's doesn't).
5. This doc.

You shouldn't need to read the Scala source externally — every Scala `case class` / sealed trait / `val`-field is already embedded inline in the `/* ... */` blocks in `compiler_outputs.rs`. If a block is ambiguous, ask; don't guess.

---

## The big picture: why Slab 6 exists

Every typing-pass sub-compiler (function compiler, struct compiler, overload resolver, ...) needs to write into a shared accumulator: "I've declared a type called `MyStruct`", "here's the signature of a function I just resolved", "this impl connects this struct to that interface", "defer evaluating this body until later". Scala does this with a mutable case class (`CompilerOutputs()`) full of `mutable.HashMap` / `mutable.ArrayBuffer` / `mutable.LinkedHashMap` fields. Rust does the same shape with `std::collections::HashMap` + `Vec` + `VecDeque`, keyed via a pointer-identity newtype.

The key engineering concern: **how do you hash a `&'t InternedT` for HashMap lookup?** Options:

- Derive `Hash` on the inner type → walks every field; slow and diverges from Scala's reference-equality.
- `std::ptr::hash(&self.0, state)` → fast, pointer-identity, matches Scala's `eq` after interning.

`quest.md` §4.2 picks option 2 via a `PtrKey<'t, T>(&'t T)` newtype. The wrapper's `Hash`/`Eq` use pointer identity; callers wrap an interned `&'t SomethingT` in `PtrKey(ref)` when inserting/looking up. This gives O(1) hash and `ptr::eq` lookup — exactly the Scala parity you'd expect from a post-interning HashMap.

The second concern: **deferred actions**. Scala has `mutable.LinkedHashMap[Key, DeferredEvaluatingFoo]` where `DeferredEvaluatingFoo.call: (CompilerOutputs) => Unit` — a closure that captures `coutputs`. In Rust, `Box<dyn FnOnce(&mut CompilerOutputs)>` would work but hides captures and costs a heap alloc per deferral. §4.4 replaces it with a tagged enum carrying the action parameters; the drain loop matches on the variant and calls the corresponding `Compiler::do_X` method with `&mut coutputs`. No `Box`, no `dyn`, no hidden captures, no self-borrow.

By the end of Slab 6:

- `cargo check --lib` passes with 0 errors.
- `src/typing/compiler_outputs.rs`:
  - `PtrKey<'t, T>` newtype with pointer-identity `Hash`/`PartialEq`/`Eq` + `Copy`/`Clone`.
  - `CompilerOutputs<'s, 't>` struct with ~20 real fields per §4.1 (with the `'t`-env override from Slab 4 applied).
  - `DeferredActionT<'s, 't>` enum with 2 variants (per §4.4); the existing `DeferredEvaluatingFunctionBody` and `DeferredEvaluatingFunction` struct stubs are deleted (they roll into enum variants; Scala `/* */` blocks stay as audit trail, no Rust sibling).
  - `CompilerOutputs::new()` constructor that initializes every field to empty.
- ~50 existing free-fn method stubs on `CompilerOutputs` stay `panic!()`. Don't port bodies.
- Scala `/* */` blocks unchanged byte-for-byte.
- File layout per TL-HANDOFF conventions preserved.

**What Slab 6 is NOT:**

- No method body migration (`declare_function_return_type`, `add_function`, `get_instantiation_bounds`, etc. stay panic-stubbed).
- No `Compiler::new` / `run_typing_pass` wiring — that's Slab 7.
- No `HinputsT` changes — Slab 7.
- No sub-compiler file fixes — Slab 8. If a sub-compiler call site breaks after this slab, patch with `panic!("Unimplemented: Slab 8")`, don't fix properly.
- No `InstantiationBoundArgumentsT` completion — it's a known Slab-7/8 residual (its value types still have `()` placeholders). Use it as-is.
- No visitor/collector pattern, no `NodeRefT`, no `impl_hash` macros.

---

## What's already in place (don't duplicate; don't delete)

Open `src/typing/compiler_outputs.rs` (756 lines). You'll see:

- A `/* package dev.vale.typing … */` imports block at top.
- **Two stub structs that must be deleted** (Gotcha 5):
  - `DeferredEvaluatingFunctionBody<'s, 't> { prototype_t: PrototypeT, call: fn() }` — line 37.
  - `DeferredEvaluatingFunction<'s, 't> { name: IdT, call: fn() }` — line 46.
- **`pub struct CompilerOutputs<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);`** at line 56 — the stub you'll fill.
- ~50 free-fn method stubs with bodies that `panic!("Unimplemented: …")`, each followed by its Scala `def …` block. These look like:
  ```rust
  fn declare_function_return_type(signature: SignatureT, return_type_2: CoordT) { panic!("Unimplemented: declare_function_return_type"); }
  /*
    def declareFunctionReturnType(signature: SignatureT, returnType2: CoordT): Unit = { … }
  */
  ```
  **Leave every one of these alone.** They're slice-pipeline artifacts. Slab 8 is where method signatures get lifted into `impl CompilerOutputs` blocks (or onto `Compiler`, depending on the scala-side caller) and bodies get migrated. Slab 6 only touches data shape.

### Things NOT in Slab 6 scope (don't touch)

- `src/typing/names/*.rs` (Slab 2 frozen).
- `src/typing/types/*.rs` (Slab 3 frozen).
- `src/typing/templata/*.rs` (Slab 3 frozen).
- `src/typing/env/*.rs` (Slab 4 frozen).
- `src/typing/ast/ast.rs`, `ast/citizens.rs`, `ast/expressions.rs` (Slabs 3 and 5 frozen — `StructDefinitionT`, `InterfaceDefinitionT`, `ImplT`, `FunctionDefinitionT`, `FunctionHeaderT`, `KindExportT`, `FunctionExportT`, `KindExternT`, `FunctionExternT`, `EdgeT`, `OverrideT` are all already real structs with Scala-parity fields; you just reference them by `&'t` here).
- `src/typing/hinputs_t.rs` — specifically, the `InstantiationBoundArgumentsT<'s, 't>` there is a known-incomplete stub (its HashMap value types still have `()` placeholders per the comment). That's Slab 7/8 cleanup, not yours. Reference it as `InstantiationBoundArgumentsT<'s, 't>` and move on.
- The `TypingInterner` (Slab 4 frozen).
- `src/typing/typing_interner.rs` (Slab 4 frozen).
- Any sub-compiler file (`array_compiler.rs`, `edge_compiler.rs`, etc.). If a reference there breaks, `panic!("Unimplemented: Slab 8 — rewire CompilerOutputs call")` and move on.

---

## Rules for each field translation

Same rules as Slabs 2–5. Quick reference:

| Scala | Rust |
|---|---|
| `mutable.HashMap[IdT[INameT], X]` | `HashMap<PtrKey<'t, IdT<'s, 't>>, X>` |
| `mutable.HashMap[SignatureT, X]` | `HashMap<PtrKey<'t, SignatureT<'s, 't>>, X>` |
| `mutable.HashMap[PrototypeT[IFunctionNameT], X]` | `HashMap<PtrKey<'t, PrototypeT<'s, 't>>, X>` |
| `mutable.HashSet[IdT[...]]` | `HashSet<PtrKey<'t, IdT<'s, 't>>>` |
| `mutable.ArrayBuffer[KindExportT]` | `Vec<&'t KindExportT<'s, 't>>` — arena-allocated exports, by ref |
| `mutable.LinkedHashMap[Key, DeferredEvaluatingFoo]` | `VecDeque<DeferredActionT<'s, 't>>` — FIFO queue; see Gotcha 4 for the rename |
| `mutable.LinkedHashSet[PrototypeT / IdT]` | `HashSet<PtrKey<'t, T>>` — ordering isn't preserved, but the set's purpose is membership, not order. See Gotcha 6 |
| Map value `FunctionDefinitionT` / `StructDefinitionT` / `InterfaceDefinitionT` / `ImplT` | `&'t FunctionDefinitionT<'s, 't>` etc. — definitions are arena-allocated (Slab 3/5), referenced by `&'t` |
| Map value `CoordT` / `ITemplataT` / `Boolean` | value by itself — inline Copy. `CoordT<'s, 't>`, `ITemplataT<'s, 't>`, `bool` |
| Map value `IInDenizenEnvironmentT` | `&'t IInDenizenEnvironmentT<'s, 't>` — see Gotcha 1 |
| Map value `Vector[ImplT]` | `Vec<&'t ImplT<'s, 't>>` — NOT `&'t [...]` (this is stack-owned, mutation required); see Gotcha 3 |
| Map value `RangeS` | `RangeS<'s>` — scout-lifetime, Copy |
| Map value `InstantiationBoundArgumentsT[...]` | `&'t InstantiationBoundArgumentsT<'s, 't>` — arena-allocated, referenced by `&'t` (even though the type itself is still stub-ish) |

### Derives

- `PtrKey<'t, T: ?Sized>` — `Copy, Clone, PartialEq, Eq, Hash, Debug`. All impls per quest.md §4.2 (manual `Hash`/`PartialEq`/`Eq` using `std::ptr::eq` and a pointer-hash, `Copy`/`Clone` manual so they work without `T: Copy`).
- `CompilerOutputs<'s, 't>` — **no derives**. It owns mutable state; `Copy`/`Clone`/`Hash`/`Eq` are meaningless. `Debug` is optional — add if you can derive it cleanly (you can't because `DeferredActionT` holds `&'s FunctionA` which doesn't impl `Debug`; skip `Debug` on the outer struct too). Effectively: no `#[derive]` at all.
- `DeferredActionT<'s, 't>` — **no derives.** Same reason as `CompilerOutputs`: variants hold `&'s FunctionA<'s>` and other non-`Debug`-able refs. It's an enum whose variants get popped off a queue and pattern-matched; no comparison, no hashing, no printing.

### `where 's: 't` bound

Every struct with both `'s` and `'t` params and `&'t` fields holding `<'s, 't>` types needs `where 's: 't`. Applies to:
- `CompilerOutputs<'s, 't>` (has `&'t FunctionDefinitionT<'s, 't>` etc.)
- `DeferredActionT<'s, 't>` (has `&'t PrototypeT<'s, 't>` etc.)

`PtrKey<'t, T>` has only `'t`, so no bound needed.

---

## The `PtrKey<'t, T>` shape

Per quest.md §4.2 verbatim:

```rust
use std::hash::{Hash, Hasher};

pub struct PtrKey<'t, T: ?Sized>(pub &'t T);

impl<'t, T: ?Sized> PartialEq for PtrKey<'t, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl<'t, T: ?Sized> Eq for PtrKey<'t, T> {}

impl<'t, T: ?Sized> Hash for PtrKey<'t, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0 as *const T as *const ()).hash(state)
    }
}

impl<'t, T: ?Sized> Copy for PtrKey<'t, T> {}

impl<'t, T: ?Sized> Clone for PtrKey<'t, T> {
    fn clone(&self) -> Self { *self }
}

impl<'t, T: ?Sized + std::fmt::Debug> std::fmt::Debug for PtrKey<'t, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PtrKey({:p})", self.0 as *const T)
    }
}
```

**File placement.** Put `PtrKey` in its own module at `src/typing/ptr_key.rs`, then `pub use crate::typing::ptr_key::PtrKey;` from `src/typing/mod.rs` if not already exposed. Rationale: (a) keeps `compiler_outputs.rs` focused on the accumulator; (b) Slab 7/8 consumers (Compiler methods, HinputsT construction) will need `PtrKey` too, and a separate module is cleaner than a deep import path; (c) mirrors how `TypingInterner` lives in its own file.

If `mod.rs` doesn't currently declare submodules in a pub way, just add `mod ptr_key;` and use the full path `crate::typing::ptr_key::PtrKey`. Don't rearrange the rest of `mod.rs`.

**Why `T: ?Sized`?** Forward-compat — allows `PtrKey<'t, dyn SomeTrait>` or slices in the future. Nothing in Slab 6 uses the `?Sized` form; every concrete key is a `Sized` type (`IdT`, `SignatureT`, `PrototypeT`). Keep the bound as written — it's harmless and matches the spec.

**Why manual `Copy`/`Clone` instead of `#[derive]`?** `#[derive(Copy, Clone)]` would require `T: Copy + Clone`. The wrapped reference `&'t T` is `Copy` regardless of `T`, but the derive macro doesn't know that. Writing the impls by hand lets `PtrKey<'t, T>` be `Copy` for any `T: ?Sized` — including non-`Copy` types like `FunctionDefinitionT` (which can't be `Copy` per ATDCX, but their `&'t` refs are).

---

## The `CompilerOutputs<'s, 't>` structure

Per quest.md §4.1 with the Slab-4 env-lifetime override applied (Gotcha 1). Read each Scala block in `compiler_outputs.rs` (lines 57–136) to confirm field names and map-key semantics — the live spec tops out at this table:

```rust
use std::collections::{HashMap, HashSet, VecDeque};

pub struct CompilerOutputs<'s, 't>
where 's: 't,
{
    // --- Function registries ---
    pub return_types_by_signature:
        HashMap<PtrKey<'t, SignatureT<'s, 't>>, CoordT<'s, 't>>,
    pub signature_to_function:
        HashMap<PtrKey<'t, SignatureT<'s, 't>>, &'t FunctionDefinitionT<'s, 't>>,

    // --- Declaration tracking ---
    pub function_declared_names:
        HashMap<PtrKey<'t, IdT<'s, 't>>, RangeS<'s>>,
    pub type_declared_names:
        HashSet<PtrKey<'t, IdT<'s, 't>>>,

    // --- Env back-refs (see Gotcha 1 for '&'t IInDenizenEnvironmentT' choice) ---
    pub function_name_to_outer_env:
        HashMap<PtrKey<'t, IdT<'s, 't>>, &'t IInDenizenEnvironmentT<'s, 't>>,
    pub function_name_to_inner_env:
        HashMap<PtrKey<'t, IdT<'s, 't>>, &'t IInDenizenEnvironmentT<'s, 't>>,
    pub type_name_to_outer_env:
        HashMap<PtrKey<'t, IdT<'s, 't>>, &'t IInDenizenEnvironmentT<'s, 't>>,
    pub type_name_to_inner_env:
        HashMap<PtrKey<'t, IdT<'s, 't>>, &'t IInDenizenEnvironmentT<'s, 't>>,

    // --- Type metadata ---
    pub type_name_to_mutability:
        HashMap<PtrKey<'t, IdT<'s, 't>>, ITemplataT<'s, 't>>,
    pub interface_name_to_sealed:
        HashMap<PtrKey<'t, IdT<'s, 't>>, bool>,

    // --- Definitions ---
    pub struct_template_name_to_definition:
        HashMap<PtrKey<'t, IdT<'s, 't>>, &'t StructDefinitionT<'s, 't>>,
    pub interface_template_name_to_definition:
        HashMap<PtrKey<'t, IdT<'s, 't>>, &'t InterfaceDefinitionT<'s, 't>>,

    // --- Impls + reverse indexes (see Gotcha 3 for the Vec-in-value exceptions) ---
    pub all_impls:
        HashMap<PtrKey<'t, IdT<'s, 't>>, &'t ImplT<'s, 't>>,
    pub sub_citizen_template_to_impls:
        HashMap<PtrKey<'t, IdT<'s, 't>>, Vec<&'t ImplT<'s, 't>>>,
    pub super_interface_template_to_impls:
        HashMap<PtrKey<'t, IdT<'s, 't>>, Vec<&'t ImplT<'s, 't>>>,

    // --- Exports / externs ---
    pub kind_exports: Vec<&'t KindExportT<'s, 't>>,
    pub function_exports: Vec<&'t FunctionExportT<'s, 't>>,
    pub kind_externs: Vec<&'t KindExternT<'s, 't>>,
    pub function_externs: Vec<&'t FunctionExternT<'s, 't>>,

    // --- Instantiation bounds ---
    pub instantiation_name_to_bounds:
        HashMap<PtrKey<'t, IdT<'s, 't>>, &'t InstantiationBoundArgumentsT<'s, 't>>,

    // --- Deferred evaluation queues ---
    pub deferred_actions: VecDeque<DeferredActionT<'s, 't>>,
    pub finished_deferred_function_body_compiles:
        HashSet<PtrKey<'t, PrototypeT<'s, 't>>>,
    pub finished_deferred_function_compiles:
        HashSet<PtrKey<'t, IdT<'s, 't>>>,
}
```

**Constructor.** Add a `CompilerOutputs::new()` that initializes every field to empty. This is the only `impl` block you add in Slab 6 (the ~50 existing method stubs stay in their single-fn panic-impl form). Shape:

```rust
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn new() -> Self {
        Self {
            return_types_by_signature: HashMap::new(),
            signature_to_function: HashMap::new(),
            function_declared_names: HashMap::new(),
            type_declared_names: HashSet::new(),
            function_name_to_outer_env: HashMap::new(),
            function_name_to_inner_env: HashMap::new(),
            type_name_to_outer_env: HashMap::new(),
            type_name_to_inner_env: HashMap::new(),
            type_name_to_mutability: HashMap::new(),
            interface_name_to_sealed: HashMap::new(),
            struct_template_name_to_definition: HashMap::new(),
            interface_template_name_to_definition: HashMap::new(),
            all_impls: HashMap::new(),
            sub_citizen_template_to_impls: HashMap::new(),
            super_interface_template_to_impls: HashMap::new(),
            kind_exports: Vec::new(),
            function_exports: Vec::new(),
            kind_externs: Vec::new(),
            function_externs: Vec::new(),
            instantiation_name_to_bounds: HashMap::new(),
            deferred_actions: VecDeque::new(),
            finished_deferred_function_body_compiles: HashSet::new(),
            finished_deferred_function_compiles: HashSet::new(),
        }
    }
}
```

This is a one-fn `impl` block — follow the TL-HANDOFF file-layout convention (own `impl` block, fn on its own lines, multi-line body). The Scala side has this as `case class CompilerOutputs()` with mutable collections initialized inline; your `new()` is the Rust equivalent. No Scala `/* */` equivalent needed for `new()` — Scala's default constructor isn't a separate definition to anchor. Just the `impl` block with the fn inside.

---

## The `DeferredActionT<'s, 't>` enum

Per quest.md §4.4, with the Slab-4 `'t`-env override applied:

```rust
pub enum DeferredActionT<'s, 't>
where 's: 't,
{
    EvaluateFunctionBody {
        prototype: &'t PrototypeT<'s, 't>,
        function_env: &'t FunctionEnvironmentT<'s, 't>,   // '&t, not '&s — see Gotcha 1
        origin: &'s FunctionA<'s>,
    },
    EvaluateFunction {
        name: &'t IdT<'s, 't>,
        calling_env: &'t IInDenizenEnvironmentT<'s, 't>,  // '&t; IInDenizenEnvironmentT for env scope
        origin: &'s FunctionA<'s>,
        template_args: &'t [ITemplataT<'s, 't>],
    },
}
```

**Why two variants and not more?** The Scala side has two existing `case class`es — `DeferredEvaluatingFunctionBody` and `DeferredEvaluatingFunction` — that already cover the cases. If body-migration later discovers a third kind of deferred action, quest.md §4.4's "Add variants as needed" clause applies. For Slab 6, match the Scala source: two variants.

**Why no `Box<dyn FnOnce>`?** Scala's `call: (CompilerOutputs) => Unit` closure captures whatever context it needs. Rust closures capture environment too, but (a) they force a heap alloc per deferral, (b) they hide the capture list from audit review, and (c) `Box<dyn FnOnce(&mut CompilerOutputs)>` has lifetime trouble with `&mut self` in the drain loop. The structured-enum approach makes captures explicit fields, costs nothing to pop (`VecDeque::pop_front` moves the value out), and the drain loop just matches and calls the right `Compiler` method. See quest.md §4.4's invariant: "`DeferredActionT` variants hold only owned or `Copy` context; they never reference `CompilerOutputs` itself."

**Derive.** No `#[derive]` — see "Derives" above. Manually implementing `Debug` would require all variant payloads to be `Debug`; `FunctionA` isn't, so skip it. If any downstream caller wants to debug-print a `DeferredActionT`, they can match on the variant and print fields individually.

**Naming.** Scala's `DeferredEvaluatingFunctionBody` / `DeferredEvaluatingFunction` become `DeferredActionT::EvaluateFunctionBody` / `DeferredActionT::EvaluateFunction` — quest.md §4.4's naming. The Rust side is more "action" than "evaluating", but the spec says `DeferredActionT`, so use it. The Scala `/* */` blocks for both deleted structs stay in the file (pre-commit hook requires it); just place them next to their corresponding enum variants as audit-trail anchors (see Gotcha 5 for block placement after stub deletion).

---

## Gotchas

### Gotcha 1 (critical): envs are `&'t IInDenizenEnvironmentT`, not `&'s IEnvironmentT`

`quest.md` §4.1 shows env map values as `&'s IEnvironmentT<'s, 't>`. Both parts of that are wrong for the current codebase:

- **`&'s` → `&'t`.** Slab 4 moved envs into the typing arena (`'t`), not the scout arena (`'s`). A `'s`-allocated env can't hold the `&'t` refs that `TemplatasStoreT` transitively requires. This is documented in the TL-HANDOFF "overrides" section and in `docs/reasoning/environments-per-denizen-long-term.md`. Apply uniformly: every env-map value in `CompilerOutputs` becomes `&'t`, not `&'s`.
- **`IEnvironmentT` → `IInDenizenEnvironmentT`.** Check the Scala blocks in `compiler_outputs.rs` (lines 75, 78, 86, 93): they say `mutable.HashMap[IdT[IFunctionTemplateNameT], IInDenizenEnvironmentT]` — specifically the narrower 6-variant in-denizen enum (Slab 4), not the wider 9-variant `IEnvironmentT`. §4.1 is off by that narrowing. Use `IInDenizenEnvironmentT` for all four env maps (`function_name_to_outer_env`, `function_name_to_inner_env`, `type_name_to_outer_env`, `type_name_to_inner_env`) and for the `calling_env` field on `DeferredActionT::EvaluateFunction`.

**`FunctionEnvironmentT` in `DeferredActionT::EvaluateFunctionBody`** is different — it's specifically a `FunctionEnvironmentT`, one of the concrete env types (not the wrapper enum). Use `&'t FunctionEnvironmentT<'s, 't>`. That matches Scala exactly.

### Gotcha 2: `CompilerOutputs` is stack-owned, so `HashMap`/`Vec`/`VecDeque` are fine (AASSNCMCX does NOT apply)

AASSNCMCX (`ArenaAllocatedStructsShouldNotContainMallocdCollections`) bans `HashMap`/`Vec` inside arena-allocated structs because the arena's drop doesn't run destructors → leak. `CompilerOutputs` is NOT arena-allocated — it's stack-owned, constructed in `run_typing_pass` via `let mut coutputs = CompilerOutputs::new()`, dropped at pass end by normal Rust RAII. The HashMaps, HashSets, Vec, and VecDeque inside it all get their destructors run. No leak.

So: freely use `std::collections::HashMap`, `std::collections::HashSet`, `std::collections::VecDeque`, and `Vec`. Not `hashbrown::HashMap` — the interner uses hashbrown for heterogeneous `'tmp`→`'t` lookup, but `CompilerOutputs` keys are homogeneous `PtrKey<'t, T>` on both insert and lookup, so `std::HashMap` is fine.

### Gotcha 3: `Vec<&'t ImplT>` in HashMap values is an AASSNCMCX non-issue too — same rationale as Gotcha 2

Two fields need `HashMap<PtrKey, Vec<&'t ImplT>>`:
- `sub_citizen_template_to_impls` — reverse index: "given this citizen template, list all impls that target it as sub-citizen"
- `super_interface_template_to_impls` — reverse index: "given this interface template, list all impls that target it as super-interface"

The `Vec` is mutable (new impls get pushed as they're discovered), so `&'t [...]` is wrong — you can't push to an arena slice. Keep `Vec<&'t ImplT<'s, 't>>` for both. Same rationale as Gotcha 2: `CompilerOutputs` is stack-owned, so its internal `Vec`s get dropped properly.

Quest.md §4.3 calls out that these two fields are **exceptions to the copy-out invariant** (most HashMap values are `Copy` and can be copied-out-before-mutate; these can't because `Vec` isn't `Copy`). Access them via `mem::take(&mut map.entry(key).or_default())` / `drain()` / reinsert patterns when Slab 8 migrates bodies. Nothing for you to do at data-definition time — just make sure the field type is `Vec<&'t ImplT<'s, 't>>`, not `&'t [ImplT]` or similar.

### Gotcha 4: Deferred queues — `VecDeque<DeferredActionT>`, not `LinkedHashMap`

Scala uses `mutable.LinkedHashMap[PrototypeT, DeferredEvaluatingFunctionBody]` — insertion-ordered map keyed on the prototype. The typical access pattern is `headOption` (FIFO peek) + removal by key. Rust's `VecDeque` gives FIFO-peek via `front()` and FIFO-pop via `pop_front()`; it's the natural replacement.

**Key difference:** Scala's `LinkedHashMap` supports keyed removal (`-= prototype`). `VecDeque` doesn't — removal is FIFO-only via `pop_front`. That matches the Scala drain pattern (`peekNext…Compile` + `markDeferredFunction…Compiled` always operate on the head), so it's fine. If Slab 7/8 discovers a case where mid-queue removal is required, switch to `indexmap::IndexMap` — but `VecDeque` is what §4.4 prescribes. Don't pre-optimize.

Two companion sets track what's been completed — `finished_deferred_function_body_compiles: HashSet<PtrKey<'t, PrototypeT>>` and `finished_deferred_function_compiles: HashSet<PtrKey<'t, IdT>>`. Scala uses `LinkedHashSet` for these (insertion-ordered); Rust's `HashSet` drops ordering but keeps O(1) membership, which is what `vassert(finishedSet contains prototype)` assertions actually need. Ordering loss is fine.

### Gotcha 5: Delete the `DeferredEvaluatingFunctionBody` / `DeferredEvaluatingFunction` struct stubs; their Scala blocks stay

The existing stubs at lines 37 and 46 are:
```rust
pub struct DeferredEvaluatingFunctionBody<'s, 't> {
    prototype_t: PrototypeT<'s, 't>,
    call: fn(),
}
/*
case class DeferredEvaluatingFunctionBody(...)
*/
```

The `call: fn()` field is a Rust-side placeholder that doesn't match Scala's `call: (CompilerOutputs) => Unit`; it's a mid-slice-pipeline artifact. Both structs get rolled into `DeferredActionT` variants, so the Rust `pub struct …` definitions are **deleted**. The Scala `/* */` blocks stay in the file byte-for-byte (pre-commit hook rule). Place the two enum variants (`EvaluateFunctionBody`, `EvaluateFunction`) so they're anchored near their Scala blocks — variant-by-variant like Slab 4's `IEnvEntryT`.

Suggested placement:
1. Write `pub enum DeferredActionT<'s, 't> { … two variants … }` above the `/* case class DeferredEvaluatingFunctionBody */` block, so the enum declaration comes first and both Scala blocks sit below as audit trail.
2. Delete the `pub struct DeferredEvaluatingFunctionBody` and `pub struct DeferredEvaluatingFunction` stub lines. Leave the `/* */` blocks in place, unchanged.

Alternative: split the enum into two `impl` placeholders with per-variant anchoring (overkill — it's two variants, one enum is cleaner).

### Gotcha 6: `HashSet<PtrKey<'t, T>>` wraps the key, not the inner ref

When inserting a name into `type_declared_names: HashSet<PtrKey<'t, IdT<'s, 't>>>`, the caller does `set.insert(PtrKey(id_ref))`, not `set.insert(id_ref)`. Same for lookup: `set.contains(&PtrKey(id_ref))`. Slab 8+ bodies will do this; you just need to get the type right.

`PtrKey` is `Copy`, so passing it around is cheap. Don't take `&PtrKey` where `PtrKey` by value works — `Copy` makes it free.

### Gotcha 7: `std::HashMap` default hasher is fine; don't bring in `hashbrown` or a custom `BuildHasher`

The `TypingInterner` uses `hashbrown::HashMap` because it needs heterogeneous lookup via `hashbrown::Equivalent` (to match a `'tmp`-borrowed query Val against a `'t`-owned stored Val). `CompilerOutputs` has no heterogeneous-lookup concern — both the insert and the lookup use `PtrKey<'t, T>` with the same `'t`. `std::collections::HashMap` with the default `RandomState` hasher is correct and cheap enough (pointer hashes are basically free).

If a Slab 7/8 downstream profiles the compiler and finds hashing is a bottleneck, consider `ahash` or `fxhash` as a drop-in `BuildHasher`. Not a Slab 6 concern.

### Gotcha 8: `CompilerOutputs` is NOT `Default` / `Clone` / `Copy`

Don't derive any traits on `CompilerOutputs` itself. It's a mutable accumulator with nontrivial constructor semantics. The sub-compilers take `&mut CompilerOutputs<'s, 't>` and mutate it directly. Cloning would duplicate all the HashMap state — wrong semantically.

If someone later wants `CompilerOutputs::default()`, they can write `fn default() -> Self { Self::new() }` as a `Default` impl — but don't add it speculatively. Slab 7's top-level wiring will use `CompilerOutputs::new()` explicitly.

### Gotcha 9: Pre-commit hook on `/* */` blocks (unchanged)

Same rule as every prior slab. `.claude/hooks/check-scala-comments` does exact-match comparison on every `/* ... */` Scala block. Rules for this slab:

- Don't edit inside `/* */` blocks — frozen content.
- You can delete the two `pub struct DeferredEvaluating*` Rust stubs above their Scala blocks as long as the `/* */` content stays byte-for-byte (see Gotcha 5).
- You can replace the `pub struct CompilerOutputs(...)` PhantomData stub with a real struct, as long as the `/* case class CompilerOutputs() { … }` block that follows stays identical.
- You cannot reformat the whitespace inside any Scala block, even by accident. Editor auto-indenting inside a frozen block will bounce at commit time. If the hook rejects your commit, read its diff — it'll show exactly which block diverged.

### Gotcha 10: No method body migration (read this before you're tempted)

The file has ~50 free-fn method stubs like `fn declare_function_return_type(...)`, `fn add_function(...)`, `fn get_instantiation_bounds(...)`, `fn mark_deferred_function_body_compiled(...)`. Every one of them has a `panic!("Unimplemented: …")` body and a `/* def foo … */` Scala block below it, often containing complex logic (`vassert` chains, pattern matches on deeply nested `IdT` structures, etc.).

**Leave every one of these alone.** They are Slab 8 work.

A few of them will look tempting because the body is trivial (`peekNextDeferredFunctionBodyCompile` is just `deferredFunctionBodyCompiles.headOption.map(_._2)`). **Still don't port.** Consistency wins — if you port one, the next slab-6 reader might assume they all stay as-is and you've just introduced a trap. If a stub's panic body keeps downstream from compiling, patch the downstream call site with `panic!("Unimplemented: Slab 8 — CompilerOutputs::foo")` instead. Method bodies are Slab 8.

### Gotcha 11: Idempotent writes (§4.5) aren't your concern here

Quest.md §4.5 mentions that during overload resolution, `attempt_candidate_banner` writes to `coutputs` even for candidates that may be rejected, and safety rests on the writes being idempotent. That's a *runtime* invariant on how sub-compilers use `CompilerOutputs` — not a field-shape or derive concern. Slab 6 doesn't enforce it at the type level. Skip.

### Gotcha 12: `InstantiationBoundArgumentsT` is incomplete — reference it but don't fix it

Look at `src/typing/hinputs_t.rs:60`. `InstantiationBoundArgumentsT<'s, 't>` exists but has `Vec<(IRuneS<'s>, ())>` placeholders where Scala has `HashMap[IRuneS, PrototypeT[...]]`. The `()` is a stand-in for a type that depends on Slab 7/8 plumbing not yet done.

For Slab 6's purposes, use `&'t InstantiationBoundArgumentsT<'s, 't>` as the value type in `instantiation_name_to_bounds`. You're just storing refs to the (partially-stubbed) type; you're not reaching into its fields. If you're ever tempted to fix the stub, don't — it's Slab 7/8.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-6.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-6.txt
```

Must print `0`. If not, stop and ask the senior. Project sets `#![allow(unused_variables, unused_imports)]`, so warnings are expected noise — only errors matter.

### Step 2: Add `PtrKey<'t, T>` in its own module

1. Create `src/typing/ptr_key.rs` with the `PtrKey` definition + impls from "The `PtrKey<'t, T>` shape" section above. ~35 lines.
2. Register in `src/typing/mod.rs`: add `pub mod ptr_key;` at the appropriate place (alphabetical with other `pub mod` entries; check the existing layout).
3. `cargo check --lib` → /tmp/sylvan-slab-6.txt. Expect 0 errors.

### Step 3: Define `DeferredActionT<'s, 't>` and delete the two existing stub structs

1. In `src/typing/compiler_outputs.rs`, replace the `pub struct DeferredEvaluatingFunctionBody … / pub struct DeferredEvaluatingFunction …` stubs with a single `pub enum DeferredActionT<'s, 't> where 's: 't { EvaluateFunctionBody { … }, EvaluateFunction { … } }`. Place the enum above the first deleted stub's Scala block.
2. Keep both `/* case class … */` blocks in place, byte-for-byte.
3. `cargo check --lib`. Any sub-compiler file that references `DeferredEvaluatingFunctionBody` / `DeferredEvaluatingFunction` by name will break; patch those references with `panic!("Unimplemented: Slab 8 — DeferredActionT")` or similar. (Expected blast radius: 0-3 sites.)

### Step 4: Replace `CompilerOutputs` stub with the real struct

1. Replace `pub struct CompilerOutputs<'s, 't>(pub std::marker::PhantomData<…>);` with the full field list from "The `CompilerOutputs<'s, 't>` structure" section above. ~23 fields.
2. Add `use` imports at the top for `std::collections::{HashMap, HashSet, VecDeque}` if not already present, and `use crate::typing::ptr_key::PtrKey;` (or the appropriate path).
3. Respect the `where 's: 't` bound on the struct header.
4. Don't add any `#[derive]` — `CompilerOutputs` derives nothing.
5. `cargo check --lib`. Downstream breakage is expected here; most fixes are either `panic!("Unimplemented: Slab 8")` patches or mechanical type-param fixes (e.g. a sub-compiler method sig that says `coutputs: &mut CompilerOutputs<'_, '_>` may need `<'s, 't>` elision fixed).

### Step 5: Add `CompilerOutputs::new()` constructor

1. Add a single-fn `impl<'s, 't> CompilerOutputs<'s, 't> where 's: 't { fn new() -> Self { … } }` block initializing every field to empty (see "Constructor" subsection). Follow TL-HANDOFF file-layout conventions (one fn per impl, multi-line body).
2. `cargo check --lib`. Clean.

### Step 6: Sweep downstream errors

Expected errors here:
- Sub-compiler files that construct `CompilerOutputs` directly or call methods that now have new field-type requirements. Patch with `panic!("Unimplemented: Slab 8")` where needed.
- Places that used the old `PhantomData`-stub `CompilerOutputs(PhantomData)` constructor (probably 0 — it's a stub that doesn't get called).
- Sub-compilers that referenced `DeferredEvaluatingFunctionBody` / `DeferredEvaluatingFunction` by name — patch.

Heuristic: if a fix needs more than 5 lines of real logic, panic it out. Bodies are Slab 8+.

### Step 7: Verify and hand off

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-6.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-6.txt   # must be 0
grep -n "PhantomData<(&'s (), &'t ())>" FrontendRust/src/typing/compiler_outputs.rs
#   ^ should show 0 hits — CompilerOutputs is real now
grep -n "pub struct DeferredEvaluatingFunction" FrontendRust/src/typing/compiler_outputs.rs
#   ^ should show 0 hits — both structs deleted
grep -n "^pub enum DeferredActionT\b" FrontendRust/src/typing/compiler_outputs.rs
#   ^ should show 1 hit
grep -n "PtrKey\b" FrontendRust/src/typing/compiler_outputs.rs | head -5
#   ^ should show several hits inside CompilerOutputs field types
grep -n "^pub struct PtrKey\b" FrontendRust/src/typing/ptr_key.rs
#   ^ should show 1 hit
```

**Never commit. The human handles all commits and tags.** When `cargo check --lib` is clean, skim your own diff, then hand back to the human for review with uncommitted changes in the working tree. If you need a local savepoint mid-slab, `git stash` or a WIP branch — don't commit on `rustmigrate-z`.

Work-order checkpoints:
- Step 2 — `PtrKey` shipped, its own module.
- Step 3 — `DeferredActionT` enum lands, old stubs deleted.
- Step 4 — `CompilerOutputs` real fields.
- Step 5 — `new()` constructor.
- Step 6 — downstream panic-patches sweep.
- Step 7 — `cargo check --lib` green, diff self-review, hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors.
- `src/typing/ptr_key.rs` exists with the full `PtrKey<'t, T: ?Sized>` newtype + manual `Copy/Clone/PartialEq/Eq/Hash/Debug` impls per quest.md §4.2.
- `src/typing/mod.rs` has `pub mod ptr_key;` (or `mod ptr_key;` with appropriate `pub use`).
- `src/typing/compiler_outputs.rs`:
  - `pub struct CompilerOutputs<'s, 't> where 's: 't { … 23 fields … }` — all real, no `PhantomData`. Fields use the types from "The `CompilerOutputs<'s, 't>` structure" section. Env map values are `&'t IInDenizenEnvironmentT<'s, 't>` (Gotcha 1). `DeferredActionT` queue is a `VecDeque`. Reverse-index impls are `Vec<&'t ImplT>` (Gotcha 3). No `#[derive]`.
  - `pub enum DeferredActionT<'s, 't> where 's: 't { EvaluateFunctionBody { … }, EvaluateFunction { … } }` — 2 variants per quest.md §4.4 with the `'t`-env override applied. No `#[derive]`.
  - `pub struct DeferredEvaluatingFunctionBody` and `pub struct DeferredEvaluatingFunction` Rust stubs are deleted; their Scala `/* */` blocks stay byte-for-byte as audit trail.
  - `impl<'s, 't> CompilerOutputs<'s, 't> where 's: 't { pub fn new() -> Self { … } }` as a single-fn impl block.
  - All ~50 existing method stubs keep `panic!()` bodies.
- Scala `/* */` blocks unchanged byte-for-byte in `compiler_outputs.rs`.
- TL-HANDOFF file-layout conventions preserved.
- Downstream sub-compilers compile (with `panic!("Unimplemented: Slab 8")` patches where needed).
- **Never commit.** Hand back with uncommitted changes; the human tags `slab-6-complete`.

---

## When you're stuck

- **"the trait `Hash` is not implemented for `&'t FunctionDefinitionT`"**: you used `&'t FunctionDefinitionT` as a HashMap key directly instead of wrapping it in `PtrKey<'t, FunctionDefinitionT>`. All HashMap *keys* in `CompilerOutputs` are `PtrKey<'t, T>`; values can be `&'t T` without wrapping.
- **"`'s` may not live long enough"**: add `where 's: 't` to the `CompilerOutputs` / `DeferredActionT` struct/enum header and their `impl` blocks. Gotcha applies to every struct/enum holding `&'t`+`'s`-bearing refs.
- **"cannot find type `PtrKey`"**: add `use crate::typing::ptr_key::PtrKey;` to `compiler_outputs.rs` (and anywhere else that needs it).
- **"`DeferredActionT` cannot derive `Debug`"**: you added `#[derive(Debug)]`. Don't. It doesn't derive cleanly because `FunctionA` doesn't impl `Debug`. No derives on `DeferredActionT` or `CompilerOutputs`.
- **"mismatched types: expected `&'t IInDenizenEnvironmentT`, found `&'s IEnvironmentT`"** at a sub-compiler call site: the Slab-4 `'t`-env override means the env ref you're passing is the wrong lifetime or the wrong wrapper. Patch the call site with `panic!("Unimplemented: Slab 8")` unless it's a trivial rename. Don't try to thread the lifetime fix through a sub-compiler body — that's Slab 8.
- **"`PtrKey` requires `T: Sized`"**: you added `T: Sized` somewhere. Remove it. `PtrKey<'t, T: ?Sized>` is the spec.
- **Pre-commit hook rejection on a `/* */` block**: you accidentally edited whitespace inside a frozen Scala block. Read the hook's diff output and revert the block's content byte-for-byte.
- **"I want to port `declare_function_return_type` because the body is 3 lines"**: don't. Gotcha 10. Every method body stays `panic!()` until Slab 8. If you port one, port none.
- **"I want to switch from `std::HashMap` to `hashbrown::HashMap` for speed"**: don't. Gotcha 7. `std::HashMap` is correct and sufficient for Slab 6. Perf optimization is Slab 9+.
- **"I want to fix the `InstantiationBoundArgumentsT` stub in `hinputs_t.rs`"**: don't. Gotcha 12. That's Slab 7/8.
- **"I want to touch `names.rs` / `types.rs` / `templata.rs` / `env/*.rs` / `ast/*.rs`"**: don't. Earlier slabs are frozen. If you think one of those files needs a change for Slab 6 to work, the shape of Slab 6 probably shifted — ask the senior first.
- **"A sub-compiler file still uses `&mut CompilerOutputs<'_, '_>` and I want to flip it to `<'s, 't>`"**: leave it. The elided-lifetime form is fine at function signatures — Rust infers `<'s, 't>` from the caller. Only flip if the compiler complains, and even then prefer `panic!("Unimplemented: Slab 8")` on the body over real signature rework.

## Where to file questions

- **Design (field layout, enum shape, arena lifetime)**: `quest.md` Part 4 is the spec; this doc covers the detailed how. If they disagree, **this doc wins** (because it folds in Slab 4's env override that quest.md §4.1 doesn't reflect). If both agree on a point and you still disagree, ask the senior.
- **Scala semantics**: the `/* */` block is the spec. Cross-reference with the other Scala blocks at lines 57–136 for the full mutable-collection shape.
- **PtrKey subtleties** (Hash impl, `?Sized`, manual Copy): quest.md §4.2 is verbatim. Cross-reference with `src/typing/typing_interner.rs` for the analogous pointer-identity pattern used in the interner (different use case, similar philosophy).
- **Hook rejections**: read the hook's diff output. Usually accidental whitespace inside `/* */`.

## Final advice

This is a narrow slab. `PtrKey` is ~35 lines of module code; `DeferredActionT` is a 2-variant enum; `CompilerOutputs` is a 23-field struct + an empty-constructor. The hardest part is **not porting method bodies** — the temptation is real because several of the ~50 existing method stubs have trivial Scala implementations sitting right there in the `/* */` block. Resist. Method bodies are Slab 8.

Two carve-outs to remember:
1. **Env overrides from Slab 4.** Quest.md §4.1's `&'s IEnvironmentT` becomes `&'t IInDenizenEnvironmentT` in every env-map value. Gotcha 1 is critical.
2. **AASSNCMCX doesn't apply.** `CompilerOutputs` is stack-owned; `HashMap` / `Vec` / `VecDeque` / `HashSet` are all fine. Gotcha 2.

After Slab 6, the only remaining data-definition slab is Slab 7 (HinputsT + Compiler shell + `run_typing_pass`). Slab 8 is the signature-rewrite pass. Slab 9+ is test-driven body migration.

Good luck.
