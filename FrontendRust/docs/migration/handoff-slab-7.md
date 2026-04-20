# Handoff: Typing Pass Slab 7 — `HinputsT` + `Compiler` shell + `run_typing_pass`

> **Post-completion note (2026-04-20):** Slab 7 shipped as described. The doc's forward references to "Slab 8 = signature rewrite for every panic-stub method" and "Slab 9+ = body migration" are **outdated** — signature rewriting has since split into Slabs 8-13 (Slab 8 = CompilerOutputs, Slab 9 = TemplataCompiler object, Slabs 10-13 = remaining sub-compilers). Body migration is now **Slab 14+**. See `TL-HANDOFF.md` at repo root for the current slab roadmap.

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0–6 are done:

- **Slab 0** (arena substrate, `TypingInterner<'s, 't>` skeleton): merged.
- **Slab 1** (leaf types): merged.
- **Slab 2** (name hierarchy: ~60 concrete names, monomorphic `IdT<'s, 't>`): tagged `slab-2-complete`.
- **Slab 3** (Kind/Coord/Templata trio: monomorphic `PrototypeT`/`SignatureT`): tagged `slab-3-complete`.
- **Slab 4** (envs + real interner bodies): tagged `slab-4-complete`.
- **Slab 5** (expression AST: 53 payload structs, 3 wrapper enums, `#[derive(PartialEq, Debug)]` family-wide because of `f64` in `ConstantFloatTE`): tagged `slab-5-complete`.
- **Slab 6** (`CompilerOutputs<'s, 't>` + `PtrKey<'t, T>` + `DeferredActionT<'s, 't>`): tagged `slab-6-complete`.

You're doing **Slab 7** — the **top-level pass-driver scaffolding**: `pub fn run_typing_pass<'s, 'ctx, 't>(…)` (the entry point), two `Compiler` panic-stub methods (`compile_program`, `drain_all_deferred`) that the entry point references, plus a small `Slab-3 residual cleanup` in `hinputs_t.rs` (two `()` placeholders flip to `&'t PrototypeT<'s, 't>` now that `PrototypeT` is monomorphic). HinputsT and Compiler are largely already in place — you're verifying their shape and adding the entry-point glue, not rebuilding them.

This is the **smallest slab** so far — budget 1–2 hours focused. The bulk of the typing pass's "real work" is Slab 8 (every existing panic-stub method gets a real signature) and Slab 9+ (test-driven body migration). Slab 7 just lays the entry-point landing pad so Slab 8 has somewhere to wire bodies into.

**Read these first in this order**, then come back:

1. `quest.md` — **Part 10** (§§10.1–10.2, "Driving The Pass") is the spec for `run_typing_pass`. Also §§2.1–2.4 (the god struct), 11 (Invariants Summary — short, useful refresher), 12.1 Slab-7 paragraph.
2. `TL-HANDOFF.md` at repo root — the file-layout standards section, the post-Slab-5/6 state. Notably: **`Compiler<'s, 'ctx, 't>` already exists with its 4 fields and a `new()` constructor** — you don't rebuild it, you just verify it compiles after Slab 6 changes and add two new methods to it.
3. `FrontendRust/docs/migration/handoff-slab-6.md` §§"What 'done' looks like" + Gotchas 1/2/4. You'll be calling `CompilerOutputs::new()` and using `DeferredActionT` from your panic-stub method bodies.
4. `Luz/shields/ArenaAllocatedStructsShouldNotContainMallocdCollections-AASSNCMCX.md` — same as Slab 6: `HinputsT` is conceptually `'t`-arena-allocated, but the current Vec/HashMap fields are an acknowledged deviation per the Scala-parity rule. **You don't fix this in Slab 7** — it's body-migration territory.
5. This doc.

You shouldn't need to read the Scala source externally — every Scala definition is already embedded inline in the `/* ... */` blocks in `compiler.rs`, `compilation.rs`, and `hinputs_t.rs`. If a block is ambiguous, ask; don't guess.

---

## The big picture: why Slab 7 exists

The typing pass needs an entry point. In Scala, that's `Compiler.evaluate(codeMap, packageToProgramA): Result[HinputsT, ICompileErrorT]` — a method on `Compiler` that:

1. Builds the `globalEnv` from `packageToProgramA`.
2. Builds a `CompilerOutputs()` accumulator.
3. Walks the program's denizens, calling sub-compiler methods that mutate `coutputs`.
4. Drains the deferred-evaluation queue.
5. Materializes a `HinputsT` from the populated `coutputs`.
6. Returns `Ok(hinputs)`.

Per `quest.md` §10.1, the Rust analog is a **free function** (not a `Compiler` method) called `run_typing_pass`. Why a free function? Because in Rust the `Compiler` struct is built from references (`&'ctx ScoutArena<'s>`, `&'ctx TypingInterner<'s, 't>`, etc.) that the caller already owns. Wrapping the construction-and-drive in a free function makes the lifetime relationships obvious: the caller holds the arenas across the call; `run_typing_pass` builds a `Compiler` and `CompilerOutputs` internally and returns the `HinputsT` (which itself holds `'s` and `'t` refs into the caller's arenas).

The Scala `Compiler.evaluate` itself becomes (in Rust) a `Compiler::compile_program(&self, &mut coutputs, program_a)` method — but that's body work. Slab 7 only stubs it so `run_typing_pass` has something to call. Same for `drain_all_deferred`.

By the end of Slab 7:

- `cargo check --lib` passes with 0 errors.
- `src/typing/compilation.rs` (or a new `src/typing/run_typing_pass.rs`, your call — see Gotcha 5) gets `pub fn run_typing_pass<'s, 'ctx, 't>(…) -> Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>> where 's: 't { panic!(…) }`. Body is a panic-stub; signature matches quest.md §10.1 modulo the `ICompileErrorT` naming (current Rust spells it `ICompileErrorT`, not `CompileErrorT`).
- `src/typing/compiler.rs` gains two new methods on `Compiler`: `pub fn compile_program(&self, coutputs: &mut CompilerOutputs<'s, 't>, program_a: &'s ProgramA<'s>) -> Result<(), ICompileErrorT<'s, 't>>` and `pub fn drain_all_deferred(&self, coutputs: &mut CompilerOutputs<'s, 't>)`. Each in its own one-fn impl block, body `panic!()`. These are landing pads for Slab 8.
- `src/typing/hinputs_t.rs`: the two `()` placeholder slots in `InstantiationReachableBoundArgumentsT.citizen_rune_to_reachable_prototype` and `InstantiationBoundArgumentsT.rune_to_bound_prototype` flip to `&'t PrototypeT<'s, 't>` (the comment block says "broken upstream" but Slab 3 fixed it — `PrototypeT` is monomorphic now). The `make()` helper's matching parameter type also flips.
- `HinputsT<'s, 't>` itself is **already done** (real fields exist with the AASSNCMCX-deviation comment); you confirm the shape and optionally add `#[derive(Debug)]` + a panic-stub `HinputsT::new()` constructor for Slab 8 to wire from `coutputs`.
- `ICompileErrorT<'s, 't>` stays `_Phantom` — known residual; bodies don't construct error variants yet (everything panics), so the placeholder is fine. See Gotcha 6.
- All Scala `/* */` blocks unchanged byte-for-byte.

**What Slab 7 is NOT:**

- No body migration. `run_typing_pass`, `compile_program`, `drain_all_deferred` all `panic!()`.
- No `HinputsT::new(coutputs)` real implementation — Slab 8 wires the materialization from `CompilerOutputs` fields.
- No `ICompileErrorT` variant filling — known residual; Slab 9+ as bodies need them.
- No `InstantiationBoundArgumentsT` HashMap → arena-slice conversion. Vec/HashMap stay.
- No `Compiler.evaluate` method porting — that's Slab 8 (when porting, it folds into `compile_program` per the new Rust naming).
- No `TypingPassCompilation::expect_compiler_outputs` rewiring — the existing panic-stub stays panic-stubbed; Slab 8 wires it to call `run_typing_pass` once `run_typing_pass`'s body is real.
- No HinputsT method bodies (`lookup_struct`, `lookup_interface`, `lookup_edge`, etc. — all stay panic-stubbed).
- No god-struct macro-method migration (`generate_function_body_lock_weak`, `dispatch_function_body_macro`, etc. — all Slab 8).

---

## What's already in place (don't duplicate; don't delete)

### `src/typing/compiler.rs` (1799 lines)

`Compiler<'s, 'ctx, 't>` is real and matches quest.md §2.1:

```rust
pub struct Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub scout_arena: &'ctx ScoutArena<'s>,
    pub typing_interner: &'ctx TypingInterner<'s, 't>,  // Slab-4 two-lifetime flip applied
    pub keywords: &'ctx Keywords<'s>,
    pub opts: &'ctx TypingPassOptions<'s>,
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn new(/* same 4 args */) -> Self { Compiler { … } }
}
```

The rest of the file (~1700 lines) is panic-stubbed free-function methods that Slab 8 will lift into `impl Compiler` blocks. **Don't touch any of those.** Specifically: `Compiler::evaluate(code_map: (), package_to_program_a: ()) -> () { panic!(…) }` at line 800 is the Scala analog of what becomes `compile_program` in Rust naming — leave it as a panic-stubbed free fn. You're adding **new** `compile_program` and `drain_all_deferred` methods inside an `impl Compiler` block, not rewriting `evaluate`.

### `src/typing/hinputs_t.rs` (334 lines)

`HinputsT<'s, 't>` is real with 11 fields matching Scala parity (`structs`, `interfaces`, `functions`, `interface_to_edge_blueprints`, `interface_to_sub_citizen_to_edge`, `instantiation_name_to_instantiation_bounds`, `kind_exports`, `function_exports`, `kind_externs`, `function_externs`, `sub_citizen_to_interface_to_edge`). Fields use `Vec<…>` and `HashMap<…>` per the documented AASSNCMCX deviation comment. **Don't change those fields** — Scala parity wins.

`InstantiationBoundArgumentsT<'s, 't>` is real but has two `()` placeholder slots from when `PrototypeT` was thought to be broken-upstream. Slab 3 fixed `PrototypeT` (it's monomorphic now). Slab 7 flips the placeholders — see "Hinputs cleanup" below.

`InstantiationReachableBoundArgumentsT<'s, 't>` has the same `()` placeholder issue. Same flip.

`pub fn make(…)` helper at line 37 — also has the `()` placeholder. Same flip.

`HinputsT` has ~10 panic-stubbed `lookup_*` methods at the bottom of the file. **Don't touch.** Slab 8.

### `src/typing/compiler_error_reporter.rs`

`ICompileErrorT<'s, 't>` is a `_Phantom`-only enum stub at line 27. ~80 case-class variants in `/* */` blocks. **Don't fill the variants** — Slab 9+ adds them as needed. See Gotcha 6.

### `src/typing/compilation.rs` (181 lines)

`TypingPassOptions<'s>` is real. `TypingPassCompilation<'s, 'ctx, 't, 'p>` is real with several panic-stubbed methods (`get_compiler_outputs`, `expect_compiler_outputs`, etc.). **Don't touch the panic stubs.** Slab 8 wires them to call `run_typing_pass`. Slab 7 just adds `run_typing_pass` itself as a sibling free function in this file.

### Things NOT in Slab 7 scope (don't touch)

- `src/typing/names/*.rs`, `src/typing/types/*.rs`, `src/typing/templata/*.rs`, `src/typing/env/*.rs`, `src/typing/ast/*.rs`, `src/typing/typing_interner.rs`, `src/typing/compiler_outputs.rs`, `src/typing/ptr_key.rs` — Slabs 2–6, frozen.
- Sub-compiler files (`array_compiler.rs`, `edge_compiler.rs`, `expression/*.rs`, `function/*.rs`, etc.) — Slab 8 territory.
- `src/typing/compiler_error_humanizer.rs` — Slab 8/9 territory.
- `src/typing/compiler_error_reporter.rs` — `ICompileErrorT` enum filling is Slab 9+; you only **reference** it from `run_typing_pass`'s return type.
- The ~50 method stubs in `compiler_outputs.rs` (Slab 6's residue) — Slab 8.
- Any sub-compiler method body migration — Slab 8/9.

---

## Rules for each field translation

This is mostly a glue slab; field translation rules barely apply. The two `()` placeholder flips need:

| Scala | Current Rust (stub) | Slab-7 Rust |
|---|---|---|
| `PrototypeT[BF <: IFunctionNameT]` (in `runeToBoundPrototype` Map value) | `()` | `&'t PrototypeT<'s, 't>` |
| `PrototypeT[R <: IFunctionNameT]` (in `citizenRuneToReachablePrototype` Map value) | `()` | `&'t PrototypeT<'s, 't>` |

`PrototypeT` is interned (Slab 3, accessed by `&'t`) and monomorphic (the Scala phantom `[BF <: IFunctionNameT]` was erased in Slab 3 per `quest.md` §6.6). Drop the parametric phantoms; reference by `&'t`. The two stub-comment lines acknowledging the broken-upstream-blocker should be removed since the upstream is now fixed.

For everything else in Slab 7, the rules are the same as prior slabs. Quick reference for the entry-point signature:

| Scala | Rust |
|---|---|
| `(codeMap: FileCoordinateMap[String], packageToProgramA: PackageCoordinateMap[ProgramA])` (Scala `Compiler.evaluate` sig) | `program_a: &'s ProgramA<'s>` (a single ProgramA per package; quest.md §10.1 simplification) |
| `Result[HinputsT, ICompileErrorT]` | `Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>>` |
| `(opts: TypingPassOptions, interner: Interner, keywords: Keywords)` (Scala `Compiler` constructor args) | `(scout_arena: &'ctx ScoutArena<'s>, typing_interner: &'ctx TypingInterner<'s, 't>, keywords: &'ctx Keywords<'s>, opts: &'ctx TypingPassOptions<'s>)` |

### Derives

- `HinputsT<'s, 't>`: optionally add `#[derive(Debug)]`. Currently un-derived. **Verify first** — `Debug` requires every field type to be `Debug`. `InterfaceDefinitionT`, `StructDefinitionT`, `FunctionDefinitionT` derive `Debug` (Slab 5 / 3); `InterfaceEdgeBlueprintT` should — confirm with a test compile. If a field doesn't derive `Debug`, skip the `Debug` derive on `HinputsT` for now and flag as a Slab 8 cleanup. **Don't try to derive `Copy` / `Clone` / `PartialEq` / `Eq` / `Hash`** — Scala overrides `equals`/`hashCode` to `vfail`/`vcurious` on HinputsT (it's too big to compare).
- `InstantiationBoundArgumentsT<'s, 't>` / `InstantiationReachableBoundArgumentsT<'s, 't>`: keep whatever derives (or lack thereof) they currently have — don't add `Debug` here either, since `IRuneS` deriving might break.
- `Compiler<'s, 'ctx, 't>`: no derives — it owns references and method pointers conceptually; comparison/hashing/printing are meaningless.

### `where 's: 't` bound

Already on `Compiler` and `HinputsT`. The new `run_typing_pass` function needs it too:

```rust
pub fn run_typing_pass<'s, 'ctx, 't>(/*…*/) -> Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>>
where 's: 't,
```

The two new method stubs on `Compiler` get `where 's: 't` from the surrounding `impl Compiler<'s, 'ctx, 't> where 's: 't { … }` block — same as `Compiler::new`.

---

## The `run_typing_pass` shape

Per quest.md §10.1, with the current-Rust adjustments (`ICompileErrorT` not `CompileErrorT`, two-lifetime `TypingInterner`):

```rust
pub fn run_typing_pass<'s, 'ctx, 't>(
    scout_arena: &'ctx ScoutArena<'s>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    keywords: &'ctx Keywords<'s>,
    opts: &'ctx TypingPassOptions<'s>,
    program_a: &'s ProgramA<'s>,
) -> Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>>
where 's: 't,
{
    panic!("Unimplemented: run_typing_pass — Slab 8 wires Compiler::compile_program + HinputsT materialization");
}
```

**Why panic instead of the §10.1 fully-fleshed body?** Two reasons:

1. The §10.1 sketch builds `HinputsT { function_definitions: typing_interner.alloc_slice_iter(…), … }` with field names that **don't match** the current Scala-parity-named `HinputsT` (`functions`, `structs`, etc.). Wiring the materialization means resolving field-name mapping AND deciding whether to keep `Vec<…>` (Scala parity, current code) or flip to `&'t [...]` (quest.md §10.1 sketch + AASSNCMCX). That decision is a body-migration concern; Slab 7 punts.
2. `Compiler::compile_program` and `Compiler::drain_all_deferred` are themselves panic-stubs — calling them from `run_typing_pass`'s body wouldn't make `run_typing_pass` actually work, just shift the panic site one level. Cleaner to make the whole entry point panic with one informative message.

The point of writing `run_typing_pass` in Slab 7 is establishing the **signature** as the lifetime-relationships anchor — every type the body would touch (`ScoutArena<'s>`, `TypingInterner<'s, 't>`, `Keywords<'s>`, `TypingPassOptions<'s>`, `ProgramA<'s>`, `HinputsT<'s, 't>`, `ICompileErrorT<'s, 't>`) appears in the signature. If any lifetime bound is wrong, you'll find out in Slab 7 instead of mid-Slab-8.

---

## The `compile_program` / `drain_all_deferred` stubs

Both go in `src/typing/compiler.rs`, each in their own one-fn `impl` block, near the top of the file (after `Compiler::new`). Per the TL-HANDOFF file-layout convention.

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_program(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        program_a: &'s ProgramA<'s>,
    ) -> Result<(), ICompileErrorT<'s, 't>> {
        panic!("Unimplemented: Compiler::compile_program — Slab 8");
    }
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn drain_all_deferred(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
    ) {
        panic!("Unimplemented: Compiler::drain_all_deferred — Slab 8");
    }
}
```

**No Scala `/* */` block companion** — these are Rust-side methods derived from the Scala-side `Compiler.evaluate` body. The original `Compiler.evaluate` Scala block stays at line ~800 with its existing `pub fn evaluate(code_map: (), package_to_program_a: ()) -> () { panic!(…) }` Rust stub. You're adding **new methods next to it**, not replacing it.

`evaluate` will be deleted (or rewritten as a thin wrapper around `compile_program`) in Slab 8 once bodies migrate. Slab 7 leaves it as-is — having two stubs side-by-side is fine; the panic message disambiguates if anyone hits it.

---

## The `hinputs_t.rs` cleanup

Two `()` placeholder slots flip to `&'t PrototypeT<'s, 't>`:

**`InstantiationReachableBoundArgumentsT`** at line 20:
```rust
pub struct InstantiationReachableBoundArgumentsT<'s, 't> {
    pub citizen_rune_to_reachable_prototype: Vec<(
        crate::postparsing::names::IRuneS<'s>,
        &'t crate::typing::ast::ast::PrototypeT<'s, 't>,  // was: ()
    )>,
    // _phantom can be deleted now that &'t PrototypeT anchors 't (and IRuneS<'s> anchors 's).
}
```

Drop the `_phantom: PhantomData<(&'s (), &'t ())>` field — both lifetimes are now anchored by real fields. The TODO comments (lines 16-19) referencing "PrototypeT upstream declares T: IFunctionNameT as a trait bound on an enum (broken)" are obsolete; delete them.

**`InstantiationBoundArgumentsT`** at line 60:
```rust
pub struct InstantiationBoundArgumentsT<'s, 't> {
    pub rune_to_bound_prototype: Vec<(
        crate::postparsing::names::IRuneS<'s>,
        &'t crate::typing::ast::ast::PrototypeT<'s, 't>,  // was: ()
    )>,
    pub rune_to_citizen_rune_to_reachable_prototype: Vec<(
        crate::postparsing::names::IRuneS<'s>,
        InstantiationReachableBoundArgumentsT<'s, 't>,
    )>,
    pub rune_to_bound_impl: Vec<(
        crate::postparsing::names::IRuneS<'s>,
        crate::typing::names::names::IdT<'s, 't>,
    )>,
}
```

Drop the matching TODO comments (lines 57–59). The `rune_to_bound_impl` field stays as `IdT<'s, 't>` by-value (not `&'t IdT`) — `IdT` is monomorphic + has custom Hash/Eq from Slab 2, so by-value is fine here. Don't flip it.

**`pub fn make(…)`** at line 37: update the `_rune_to_bound_prototype` parameter type to match:
```rust
pub fn make<'s, 't>(
    _rune_to_bound_prototype: Vec<(crate::postparsing::names::IRuneS<'s>, &'t crate::typing::ast::ast::PrototypeT<'s, 't>)>,
    _rune_to_citizen_rune_to_reachable_prototype: Vec<(crate::postparsing::names::IRuneS<'s>, InstantiationReachableBoundArgumentsT<'s, 't>)>,
    _rune_to_bound_impl: Vec<(crate::postparsing::names::IRuneS<'s>, crate::typing::names::names::IdT<'s, 't>)>,
) -> InstantiationBoundArgumentsT<'s, 't> {
    panic!("Unimplemented: InstantiationBoundArgumentsT::make");
}
```

Body stays panic-stubbed. Drop the `// TODO: stub` comment immediately above (the upstream blocker is gone).

**Verify downstream.** Both types are referenced from `CompilerOutputs.instantiation_name_to_bounds: HashMap<PtrKey<'t, IdT>, &'t InstantiationBoundArgumentsT<'s, 't>>` (Slab 6) and from `ImplT.instantiation_bound_params: InstantiationBoundArgumentsT<'s, 't>` (Slab 3). Neither call site reaches into the changed fields, so both should still compile. `cargo check --lib` after the flip; if any sub-compiler method tries to construct an `InstantiationBoundArgumentsT { rune_to_bound_prototype: vec![(rune, ())], … }`, patch with `panic!("Unimplemented: Slab 8")` (rare; most sub-compiler bodies already panic and don't construct).

---

## Gotchas

### Gotcha 1: Don't try to fill `run_typing_pass`'s body

`quest.md` §10.1 shows a full-body sketch with `compiler.compile_program(…)?`, `compiler.drain_all_deferred(…)`, and `HinputsT { … }` materialization. **Don't write it.** Two reasons:

- The `HinputsT` field-set in §10.1 doesn't match the current Scala-parity HinputsT field-set (the sketch uses `function_definitions`/`struct_definitions`; the real struct has `functions`/`structs`).
- `compile_program` and `drain_all_deferred` are themselves panic-stubs in Slab 7. Wiring the body would just shift panics, not enable real execution.

Body migration is Slab 8. Slab 7's `run_typing_pass` body is one panic line.

### Gotcha 2: `ICompileErrorT` stays `_Phantom`

`src/typing/compiler_error_reporter.rs` has `ICompileErrorT<'s, 't>` as a `_Phantom`-only enum with ~80 Scala case-class variants in `/* */` blocks. **Don't fill the variants in Slab 7.** Why:

- The variants are wide-ranging (`CouldntNarrowDownCandidates`, `CouldntSolveRuneTypesT`, `ImplSubCitizenNotFound`, `BodyResultDoesntMatch`, etc.) and each has its own field set. Filling all 80 is half a slab on its own.
- Nothing constructs an `ICompileErrorT` variant yet — every sub-compiler body that would `return Err(…)` is panic-stubbed. So the placeholder is fine until bodies need it.
- Slab 9+ tests will drive variant filling on-demand: as bodies migrate and a test exercises a code path that should produce an error, that code path's variants get filled.

`run_typing_pass`'s return type uses `ICompileErrorT<'s, 't>` symbolically — Rust accepts the type even though the enum has only `_Phantom`. The signature compiles; no caller will ever construct the error. Good enough.

### Gotcha 3: `HinputsT` Vec/HashMap fields are intentional Scala-parity deviation from AASSNCMCX

`HinputsT<'s, 't>` holds `Vec<InterfaceDefinitionT>`, `HashMap<IdT, EdgeT>`, etc. AASSNCMCX (no `Vec` / `HashMap` / `String` in arena-allocated types) appears to apply because per quest.md §1.5 `HinputsT` is "'t-arena-allocated". The current code's documenting comment (line 94-97) acknowledges the deviation and defers it:

> // TODO: stub — Vec/HashMap fields mirror the Scala case class. Per quest.md §1.5
> // HinputsT is 't-arena-allocated, which per AASSNCMCX means these should later become
> // arena slices, not std Vec/HashMap. Keeping Vec/HashMap for now to match Scala shape;
> // revisit during body migration.

**Slab 7 leaves this deviation in place.** Don't try to flip `Vec<X>` → `&'t [X]` — doing so requires materialization logic at HinputsT-construction time (which is Slab 8 work). Body migration revisits.

If you want to add `#[derive(Debug)]` to `HinputsT` and find it doesn't derive cleanly because of one of the field types, skip the derive and add a one-line `// TODO: derive Debug — blocked on Slab N: FooT doesn't derive Debug` comment. Don't unblock by adding Debug to a Slab-2/3/4/5 type — those slabs are frozen.

### Gotcha 4: `Compiler.evaluate` panic-stub stays — don't delete it

The existing `pub fn evaluate(&self, code_map: (), package_to_program_a: ()) -> ()` at compiler.rs line 800 is the Scala analog of what'll become `compile_program` in Rust naming. Slab 8 will either rename `evaluate` → `compile_program` (folding the bodies) or delete `evaluate` entirely. **Slab 7 leaves it untouched.** Add `compile_program` and `drain_all_deferred` as **new sibling methods** in their own impl blocks. Two slightly-overlapping panic stubs is fine; the panic message ("Unimplemented: Compiler::compile_program — Slab 8") tells the reader which is which.

### Gotcha 5: File placement for `run_typing_pass` — `compilation.rs` vs new `run_typing_pass.rs`

Two reasonable choices:

**Option A: put `run_typing_pass` in `src/typing/compilation.rs`** (alongside `TypingPassOptions` and `TypingPassCompilation`). Pros: it's the existing pass-coordination module; `TypingPassCompilation::expect_compiler_outputs` will eventually call `run_typing_pass`. Cons: `compilation.rs` is small (181 lines) and currently does the higher-typing wrapping; mixing the typing-pass entry with the compilation orchestration makes the file's role less clean.

**Option B: new `src/typing/run_typing_pass.rs` file.** Pros: clean separation — one file = one concern. Cons: a new ~30-line file just for one panic-stub fn feels like overkill.

**Recommendation: Option A.** Put it in `compilation.rs`, near the top (above `TypingPassCompilation` impl block). Add a `pub use compilation::run_typing_pass;` to `src/typing/mod.rs` for easy import.

Why: a one-fn standalone file is awkward, and `TypingPassCompilation` already lives in `compilation.rs`, so the entry point being its sibling is the natural reading order — caller calls `TypingPassCompilation::expect_compiler_outputs(…)` (today panics), which in Slab 8 will internally call `run_typing_pass(…)`. Putting both in the same file makes that call shape easy to scan.

### Gotcha 6: Don't wire `TypingPassCompilation::expect_compiler_outputs` to call `run_typing_pass` yet

The existing panic-stub at compilation.rs ~line 156 — `pub fn expect_compiler_outputs(&mut self) -> ()` — will eventually call `run_typing_pass`. **Don't wire it in Slab 7.** Why:

- Both methods panic. Wiring just shifts the panic site.
- `TypingPassCompilation::expect_compiler_outputs` needs to provide the `scout_arena` / `typing_interner` / `keywords` / `opts` / `program_a` that `run_typing_pass` takes. The wiring requires real thread-through of those references from `TypingPassCompilation`'s current state, which is Slab 8 plumbing.

Slab 7: leave `expect_compiler_outputs` as `panic!("…")`. Slab 8 wires it.

### Gotcha 7: `_phantom` field on `InstantiationReachableBoundArgumentsT` should be deleted, not preserved

After flipping `()` → `&'t PrototypeT<'s, 't>` in `InstantiationReachableBoundArgumentsT.citizen_rune_to_reachable_prototype`, the `_phantom: PhantomData<(&'s (), &'t ())>` field becomes redundant — both `'s` (via `IRuneS<'s>`) and `'t` (via `&'t PrototypeT<'s, 't>`) are now anchored by real fields. **Delete the `_phantom` field.** Same for the matching TODO comment.

`InstantiationBoundArgumentsT` doesn't have a `_phantom` (its three Vec-fields anchor both lifetimes already), so nothing to delete there.

### Gotcha 8: Pre-commit hook on `/* */` blocks (unchanged)

Same as every prior slab. `.claude/hooks/check-scala-comments` does exact-match comparison on every `/* ... */` block. Rules for Slab 7:

- Don't edit inside any `/* */` block.
- You can delete/edit Rust comments **outside** `/* */` blocks (specifically: the `// TODO: stub — replace Vec…` lines around `InstantiationReachableBoundArgumentsT` and `InstantiationBoundArgumentsT`, since those are stale post-Slab-3 and you're cleaning them up).
- You can add new Rust definitions (the `compile_program` impl block, the `drain_all_deferred` impl block, the `run_typing_pass` fn) — new code outside Scala blocks is unrestricted.

### Gotcha 9: `pub fn make(…)` at hinputs_t.rs line 37 is NOT inside an `impl` block

It's a top-level `pub fn` (per the slice-pipeline output). Scala's `object InstantiationBoundArgumentsT { def make(…) }` is a companion-object factory; the Rust analog would be `impl InstantiationBoundArgumentsT { pub fn make(…) }`. **Don't lift it into an impl block in Slab 7.** Slab 8 does the lift as part of method-signature rewrite. You're only updating the parameter type to flip `()` → `&'t PrototypeT`.

### Gotcha 10: No method body migration on HinputsT

The bottom of `hinputs_t.rs` has ~10 panic-stubbed lookup methods (`lookup_struct`, `lookup_struct_by_template`, `lookup_interface`, `lookup_edge`, etc.) with Scala bodies in `/* */` blocks. **All stay panic-stubbed.** Slab 8.

Same for the `subCitizenToInterfaceToEdge` post-hoc-computed map: Scala builds it in the case-class body from `interfaceToSubCitizenToEdge` via a 5-line nested-loop. The Rust-side equivalent would be a `HinputsT::new(…)` constructor that does the inversion. **Don't write that constructor in Slab 7.** Materialization is Slab 8. The current code keeps `sub_citizen_to_interface_to_edge` as a separate field that callers populate, which works for the data-def shape we need now.

If you do add a `HinputsT::new(…)` constructor in Slab 7, make it panic-stubbed and take only the obvious args (no inversion logic):
```rust
impl<'s, 't> HinputsT<'s, 't> {
    pub fn new(/* … all 11 fields by value … */) -> Self {
        panic!("Unimplemented: HinputsT::new — Slab 8 wires from CompilerOutputs");
    }
}
```

This is optional — Slab 8 can construct HinputsT via a struct literal directly, no factory needed. Skip the `new()` if you'd rather keep Slab 7 minimal.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-7.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-7.txt
```

Must print `0`. If not, stop and ask the senior. Project sets `#![allow(unused_variables, unused_imports)]`, so warnings are noise — only errors matter.

### Step 2: Hinputs `()` → `&'t PrototypeT` flip

In `src/typing/hinputs_t.rs`:

1. **`InstantiationReachableBoundArgumentsT`** (line 20): change the second tuple element from `()` to `&'t crate::typing::ast::ast::PrototypeT<'s, 't>`. Delete the `_phantom: PhantomData<(&'s (), &'t ())>` field. Delete the TODO-stub comment block above (lines 16–19).
2. **`InstantiationBoundArgumentsT`** (line 60): change the second tuple element of `rune_to_bound_prototype` from `()` to `&'t crate::typing::ast::ast::PrototypeT<'s, 't>`. Delete the TODO-stub comment block above (lines 57–59).
3. **`pub fn make(…)`** (line 37): update the first parameter's type accordingly. Delete the TODO comment above.

`cargo check --lib`. Expect 0 errors. Downstream sites that destructure these tuples should still compile because (a) nothing currently destructures them outside panic-stubbed bodies, and (b) the field types changing from `()` to a ref doesn't break by-name access.

If a sub-compiler call site does break (uncommon), patch with `panic!("Unimplemented: Slab 8")`.

### Step 3: Add `compile_program` and `drain_all_deferred` panic-stub methods

In `src/typing/compiler.rs`, add two new one-fn `impl Compiler` blocks **near the top of the file, right after the existing `Compiler::new()` impl block** (around line 138):

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_program(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        program_a: &'s ProgramA<'s>,
    ) -> Result<(), ICompileErrorT<'s, 't>> {
        panic!("Unimplemented: Compiler::compile_program — Slab 8");
    }
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn drain_all_deferred(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
    ) {
        panic!("Unimplemented: Compiler::drain_all_deferred — Slab 8");
    }
}
```

Add `use crate::typing::compiler_outputs::CompilerOutputs;` and `use crate::typing::compiler_error_reporter::ICompileErrorT;` at the top of compiler.rs if not already present. `ProgramA` should already be imported (the existing `evaluate` panic stub doesn't reference it, but any sub-compiler body might).

`cargo check --lib`. Expect 0 errors.

### Step 4: Add `run_typing_pass` in `compilation.rs`

In `src/typing/compilation.rs`, near the top (above `TypingPassCompilation` — say after the `TypingPassOptions` definition + its Scala block):

```rust
pub fn run_typing_pass<'s, 'ctx, 't>(
    scout_arena: &'ctx crate::scout_arena::ScoutArena<'s>,
    typing_interner: &'ctx crate::typing::typing_interner::TypingInterner<'s, 't>,
    keywords: &'ctx crate::keywords::Keywords<'s>,
    opts: &'ctx TypingPassOptions<'s>,
    program_a: &'s crate::higher_typing::ast::ProgramA<'s>,
) -> Result<
    crate::typing::hinputs_t::HinputsT<'s, 't>,
    crate::typing::compiler_error_reporter::ICompileErrorT<'s, 't>,
>
where 's: 't,
{
    panic!("Unimplemented: run_typing_pass — Slab 8");
}
```

Use full paths or add `use` statements at the top — your call. The existing imports in `compilation.rs` already cover most of these (`ScoutArena`, `Keywords`, `PackageCoordinate`); add what's missing.

`cargo check --lib`. Expect 0 errors. If a lifetime bound complains ("`'s` may not live long enough"), confirm `where 's: 't` is on the signature.

Add `pub use compilation::run_typing_pass;` to `src/typing/mod.rs` near the existing `pub use compilation::{TypingPassCompilation, TypingPassOptions};` line.

### Step 5: (Optional) `HinputsT::new(…)` panic-stub constructor

Skip unless you really want it. If you add it, make it panic-stubbed (Gotcha 10 shape).

### Step 6: (Optional) `#[derive(Debug)]` on HinputsT

Try `#[derive(Debug)]` on `HinputsT<'s, 't>`. If it compiles cleanly, keep it. If a field type doesn't derive `Debug`, drop the derive and add a `// TODO: Slab 8 — derive Debug after FooT gets it` line. **Don't add Debug to a Slab-2/3/5 type to make HinputsT derive cleanly** — those slabs are frozen.

### Step 7: Verify and hand off

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-7.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-7.txt   # must be 0
grep -nE 'pub fn run_typing_pass\b' FrontendRust/src/typing/compilation.rs
#   ^ should show 1 hit
grep -nE 'pub fn (compile_program|drain_all_deferred)\b' FrontendRust/src/typing/compiler.rs
#   ^ should show 2 hits (one each)
grep -nE '\(\s*$' FrontendRust/src/typing/hinputs_t.rs | head -5
grep -nE '\),$\|\)\s*$' FrontendRust/src/typing/hinputs_t.rs | grep -B1 '()' | head -10
#   ^ check no '()' tuple-element stubs remain in InstantiationReachableBound* / InstantiationBound*
grep -n 'PrototypeT' FrontendRust/src/typing/hinputs_t.rs
#   ^ should show several hits — the new &'t PrototypeT<'s, 't> field types
```

**Never commit.** Hand back uncommitted; the human reviews and tags `slab-7-complete`.

Work-order checkpoints:
- Step 2 — Hinputs `()` flip + TODO cleanup.
- Step 3 — `compile_program` / `drain_all_deferred` panic stubs.
- Step 4 — `run_typing_pass` entry point.
- Step 5/6 — optional polish.
- Step 7 — verify + hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors.
- `src/typing/compilation.rs` has `pub fn run_typing_pass<'s, 'ctx, 't>(…) -> Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>> where 's: 't { panic!(…) }`. Signature matches quest.md §10.1 modulo `ICompileErrorT` naming and the two-lifetime `TypingInterner`.
- `src/typing/mod.rs` has `pub use compilation::run_typing_pass;`.
- `src/typing/compiler.rs` has two new one-fn `impl Compiler` blocks: `compile_program(&self, coutputs, program_a) -> Result<(), ICompileErrorT>` and `drain_all_deferred(&self, coutputs)`. Both panic-stubbed.
- `src/typing/hinputs_t.rs`: the two `()` tuple-element placeholders flipped to `&'t PrototypeT<'s, 't>`. `_phantom` field deleted from `InstantiationReachableBoundArgumentsT`. Stale TODO comments deleted. `make()` parameter updated.
- All Scala `/* */` blocks unchanged byte-for-byte.
- TL-HANDOFF file-layout conventions preserved.
- Downstream sub-compilers compile (with `panic!("Unimplemented: Slab 8")` patches if any sub-compiler reached into the changed `()` tuple fields — uncommon).
- `ICompileErrorT<'s, 't>` stays `_Phantom` — known residual.
- `Compiler::evaluate` panic-stub stays in compiler.rs unchanged — Slab 8 deletes/folds.
- `TypingPassCompilation::expect_compiler_outputs` panic-stub stays unchanged — Slab 8 wires.
- **Never commit.** Hand back with uncommitted changes; human tags `slab-7-complete`.

---

## When you're stuck

- **"`'s` may not live long enough" on `run_typing_pass`**: confirm `where 's: 't` is on the function signature (not just on `Compiler` and `HinputsT`).
- **"cannot find type `ICompileErrorT` in this scope"** in `run_typing_pass` or `compile_program`: add `use crate::typing::compiler_error_reporter::ICompileErrorT;` at the top of the file.
- **"`PrototypeT` doesn't derive `Debug`"** when adding `#[derive(Debug)]` to `InstantiationBoundArgumentsT` or `HinputsT`: skip the Debug derive on the outer struct. Don't try to add Debug to PrototypeT — Slab 3 is frozen.
- **"`_phantom: PhantomData<(&'s (), &'t ())>` is unused"** warning after deleting the field: that's expected; the field is gone, the warning resolves itself. If a phantom field is still needed (you're in a struct with no `'s`-anchoring real fields after the flip), keep it. Step 2 explicitly says delete only `InstantiationReachableBoundArgumentsT._phantom`; don't preemptively delete others.
- **"sub-compiler file X doesn't compile because it constructs `InstantiationBoundArgumentsT { rune_to_bound_prototype: vec![(rune, ())], … }`"**: patch with `panic!("Unimplemented: Slab 8 — re-construct with &'t PrototypeT")`. Real construction is body work.
- **"`ProgramA` is not in scope"** in `Compiler::compile_program` or `run_typing_pass`: add `use crate::higher_typing::ast::ProgramA;` (or use the full path).
- **"I want to fill `ICompileErrorT` variants because the panic message says it's a `_Phantom` enum"**: don't. Gotcha 2. Slab 9+.
- **"I want to wire `TypingPassCompilation::expect_compiler_outputs` to call `run_typing_pass`"**: don't. Gotcha 6. Slab 8 plumbing.
- **"I want to delete `Compiler::evaluate` since `compile_program` replaces it"**: don't. Gotcha 4. Slab 8 cleanup.
- **"I want to convert HinputsT's `Vec` and `HashMap` fields to arena slices"**: don't. Gotcha 3. Body migration.
- **"I want to write a real `HinputsT::new` that materializes from `CompilerOutputs`"**: don't. Gotcha 10. Slab 8.
- **"I want to touch a Slab 2–6 file"**: don't. The earlier slabs are frozen. The hinputs_t.rs `()` flip is a Slab-3-residual cleanup explicitly carved out for Slab 7; everything else is hands-off.
- **Pre-commit hook rejection**: read the diff. Probably accidental whitespace inside a `/* */` block. Revert and retry.

## Where to file questions

- **Design**: `quest.md` Part 10 is the spec; this doc covers the "current Rust" adjustments (`ICompileErrorT` not `CompileErrorT`, two-lifetime `TypingInterner`, `_Phantom` `ICompileErrorT` deferred, file-placement choice). If they disagree, **this doc wins** because it folds in post-Phase-3 reality.
- **Scala semantics**: the `/* */` block is the spec. `Compiler.evaluate` at compiler.rs ~line 800 and `TypingPassCompilation.expectCompilerOutputs` at compilation.rs ~line 156 are the relevant Scala bodies; you're not porting them, but skim them to understand what Slab 8 will eventually wire.
- **Hook rejections**: read the hook's diff output. Usually accidental whitespace inside `/* */`.

## Final advice

This is a small, mostly-glue slab. The risk is **doing too much** — getting tempted by "while I'm here, let me…" detours that drift into Slab 8 work. Stay tight:

- Two `()` flips in `hinputs_t.rs` (delete two TODO blocks in the process).
- One `_phantom` field deletion.
- Two new panic-stub methods on `Compiler`.
- One new free fn `run_typing_pass`.
- One `pub use` line in `mod.rs`.
- (Optional) `#[derive(Debug)]` on HinputsT if it derives cleanly.

Total diff: probably 30–60 net new lines of Rust + ~10 lines deleted (TODO blocks + `_phantom`). If your diff is significantly bigger than that, you've drifted.

After Slab 7, the data-def slab series is **complete**. Slab 8 is the signature-rewrite pass — every panic-stub method gets its real parameter types and `&'s` / `&'t` lifetimes wired through, with bodies still panicking. Slab 9+ is test-driven body migration. The shape of those slabs is meaningfully different (per-method instead of per-type-family); this is the last of the broad-stroke data-shape slabs.

Good luck.
