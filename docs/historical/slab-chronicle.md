# Typing Pass Migration — Slab Chronicle (Historical)

**This is a historical record.** The slab-based scaffolding phase (Slabs 0–14b) is complete. All type definitions, method signatures, and placeholder types are done. The active work is now test-driven body migration (Slab 15+). See `TL-HANDOFF.md` for the current design spec and operating instructions.

---

## Slab Summary Table

| Slab | Scope | Status | Tag / handoff |
|---|---|---|---|
| 0 | arena substrate | done | (scaffolding; no tag) |
| 1 | leaf types (OwnershipT, primitive KindT payloads, leaf templatas) | done | commit `9fd7641c` |
| 2 | name hierarchy (~60 concrete names, 22 sub-enums, IdT, ValT companions) | done | `slab-2-complete` · `FrontendRust/docs/migration/handoff-slab-2.md` |
| 3 | Kind / Coord / Templata trio | done | `slab-3-complete` · `FrontendRust/docs/migration/handoff-slab-3.md` |
| 4 | environments + real interner bodies | done | `slab-4-complete` · `FrontendRust/docs/migration/handoff-slab-4.md` |
| 5 | expression AST (3 enums + 53 payload structs, `ast/expressions.rs`) | done | `slab-5-complete` · `FrontendRust/docs/migration/handoff-slab-5.md` |
| 6 | `CompilerOutputs` data + `PtrKey` + `DeferredActionT` | done | `slab-6-complete` · `FrontendRust/docs/migration/handoff-slab-6.md` |
| 7 | `HinputsT` residual cleanup + `Compiler` shell + `run_typing_pass` | done | `slab-7-complete` · `FrontendRust/docs/migration/handoff-slab-7.md` |
| 8 | `CompilerOutputs` method signatures (54 sigs) | done | `slab-8-complete` · `FrontendRust/docs/migration/handoff-slab-8.md` |
| 9 | `object TemplataCompiler` method signatures (35 sigs) | done | `slab-9-complete` · `FrontendRust/docs/migration/handoff-slab-9.md` |
| 10 | compiler.rs residuals + orphans + templata_compiler.rs 14 tail methods (~30 sigs) | done | `slab-10-complete` · `FrontendRust/docs/migration/handoff-slab-10.md` |
| 11 | expression-layer sigs: expression/pattern/block/call_compiler (~39 sigs) | done | `slab-11-complete` · `FrontendRust/docs/migration/handoff-slab-11.md` |
| 12 | solver + resolver sigs: infer/overload/impl/reachability + IInfererDelegate deletion (~35 sigs) | done | `slab-13-complete` · `FrontendRust/docs/migration/handoff-slab-12.md` |
| 13 | function/citizen/macros sigs: function_compiler/function_body/destructor/struct_compiler_core/anonymous_interface_macro (~23 sigs) | done | `slab-13-complete` · `FrontendRust/docs/migration/handoff-slab-13.md` |
| 14 | placeholder types flesh-out: ICompileErrorT (55 variants), IBoundArgumentsSource (trait→enum), IFunctionGenerator (trait→enum), IPlaceholderSubstituter (trait def), InferEnv, solver structs, error/result enums, HinputsT::new() | done | `FrontendRust/docs/migration/handoff-slab-14.md` |
| 14b | second-wave placeholder flesh-out: CitizenDefinitionT, IStructMemberT, IMemberTypeT, IFunctionAttributeT, ICitizenAttributeT, ICalleeCandidate, function result enums, ITypingPassSolverError (28 variants), misc demotions | done | `slab-14b-complete` · `FrontendRust/docs/migration/handoff-slab-14b.md` |

---

## Slab Descriptions

### Slab 0 — Arena substrate scaffolding.

### Slab 1 — Leaf types
Real Copy enums for OwnershipT / MutabilityT / VariabilityT / LocationT; primitive KindT payloads; leaf-value templatas.

### Slab 2 — Name hierarchy
Monomorphic IdT; ~60 concrete name structs + 22 inline-owned sub-enums per the DAG rule; From/TryFrom bridges; IDEPFL `*ValT` companions.

### Slab 3 — Kind/Coord/Templata trio
KindT inline wrapper + interned concrete payloads; ITemplataT inline wrapper; CoordListTemplataValT for the one slice-bearing templata payload; PrototypeValT/SignatureValT with `'tmp`.

### Slab 4 — Envs + real interner bodies
9 env structs + 2 wrapper enums + GlobalEnvironmentT + TemplatasStoreT + IEnvEntryT + IVariableT/ILocalVariableT + 4 concrete variables, all Scala-parity. 9 env-specific builders + TemplatasStoreBuilder with `build_in(interner) -> &'t FooEnvironmentT`. TypingInterner got real bodies (560 lines) with 6 family-level hashbrown HashMaps keyed on tagged-union Val enums (INameValT 72 variants / InternedKindPayloadValT 6 / InternedTemplataPayloadValT 6), plus ~84 per-concrete thin-wrapper methods via four `impl_intern_*_wrapper_*` macros. Five heavy-templata env refs flipped `&'s` → `&'t`. Box stubs deleted.

### Slab 5 — Expression AST
3 enums (48 + 5 + 2 variants) + 53 payload structs + result types. `#[derive(PartialEq, Debug)]` family-wide.

### Slab 6 — CompilerOutputs data shape
23-field struct with PtrKey-keyed HashMaps, DeferredActionT 2-variant enum, `::new()` constructor. PtrKey newtype in its own module `ptr_key.rs`. Method signatures deferred to Slab 8.

### Slab 7 — HinputsT + Compiler shell + run_typing_pass
Two `()` → `&'t PrototypeT` flips, `_phantom` deletion, `Compiler::compile_program` and `Compiler::drain_all_deferred` panic-stub methods, `pub fn run_typing_pass(...)` free fn.

### Slab 8 — CompilerOutputs method signatures (54 sigs)
All 54 free-fn stubs lifted into one-fn `impl<'s, 't> CompilerOutputs<'s, 't> where 's: 't { pub fn ... }` blocks. `&self`/`&mut self` per mutation, `&'t` returns, IdT by value, env params `&'t IInDenizenEnvironmentT<'s, 't>`. Bodies panic-stubbed.

### Slab 9 — TemplataCompiler object methods (35 sigs)
35 free-fn stubs lifted onto `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't`. `interner` and `keywords` parameters dropped (use `self.typing_interner` / `self.keywords`). `bound_arguments_source: &'t dyn IBoundArgumentsSource<'s, 't>` on the 11 substitute_* methods.

### Slab 10 — Compiler.rs residuals + orphans + TemplataCompiler tail (~30 sigs)
Flipped `()` placeholders in compiler.rs methods (`preprocess_struct`, `preprocess_interface`, `determine_macros_to_call`, `ensure_deep_exports`, `is_root_function`, `is_root_struct`, `is_root_interface`, `evaluate`) and orphan static fns; filled `local_helper.rs` 2 orphans + `struct_compiler.rs` 1 orphan; filled 14 bare-`(&self)` tail impls in `templata_compiler.rs`. Split giant impl blocks.

### Slab 11 — Expression-layer signatures (~39 sigs)
`expression_compiler.rs` 21 + `pattern_compiler.rs` 12 + `block_compiler.rs` 2 + `call_compiler.rs` 4. Novel patterns: `NodeEnvironmentBox` → `&mut NodeEnvironmentBuilder<'s, 't>`, `FunctionEnvironmentBoxT` → `&mut FunctionEnvironmentBuilder<'s, 't>`, closure params use `impl FnOnce(...)`.

### Slab 12 — Solver + resolver signatures (~35 sigs)
`infer_compiler.rs` 15 + `overload_resolver.rs` 11 + `impl_compiler.rs` 9 + `reachability.rs` 8 lifts. Deleted `IInfererDelegate` vestigial trait + dropped `&dyn IInfererDelegate` params from all signatures. `InferEnv<'s>` is a PhantomData placeholder (pass by value as Copy). `r#continue` raw identifier preserved.

### Slab 13 — Function/citizen/macros signatures (~23 sigs, final signature-rewrite slab)
`function_compiler.rs` 8 + `function_body_compiler.rs` 3 + `destructor_compiler.rs` 2 + `struct_compiler_core.rs` 6 + `anonymous_interface_macro.rs` 4. Preserved `_core`, `_ext`, `_anonymous_interface` suffix disambiguators.

### Slab 14 — Placeholder types flesh-out
Per-type, not per-file. `ICompileErrorT` (55 variants), `IDefiningError` (2), `IResolvingError` (2), `IFindFunctionFailureReason` (11), `IResolveOutcome` (2). `IBoundArgumentsSource` flipped from trait to 2-variant enum. `IFunctionGenerator` flipped from trait to dispatch-tag enum. `IPlaceholderSubstituter` struct definition + 3 panic-stub methods. Solver structs: `InferEnv<'s, 't>`, `InitialKnown`, `InitialSend`, `CompleteDefineSolve`, `CompleteResolveSolve`. `HinputsT::new()` constructor added. Bulk-updated 136 panic messages from `Slab 14` → `Slab 15`.

### Slab 14b — Second-wave placeholder flesh-out
`CitizenDefinitionT` (fold 2 structs), `IStructMemberT` (2 variants), `IMemberTypeT` (2 variants), `IFunctionAttributeT` (4 variants with `ExternT`), `ICitizenAttributeT` (2 variants), `ICalleeCandidate` (3 variants), `AbstractT` (unit struct). Function-result enum triples: `IDefineFunctionResult`, `IResolveFunctionResult`, `IStampFunctionResult`. `ITypingPassSolverError` (28 variants). Reduced `_Phantom` count from 12 → 1 (only `IRegionNameT` remains, deferred — 0 Scala implementors found).

---

## Ground Rule: Preserve The `/* scala */` Audit Trail

The typing/ skeleton has a `/* ... */` block with the Scala source directly below every Rust definition. The `.claude/hooks/check-scala-comments` pre-commit hook does exact-match comparison and rejects any edit inside those blocks. Rules:

- Replace the empty Rust stub in-place with the real definition. Keep the Scala `/* ... */` block below unchanged.
- Never move a Rust definition away from its Scala block.
- A Rust definition grown to need helper structs (e.g. an IdValT companion for an IdT) gets its companion block **adjacent to** the main definition, with a `// (no scala counterpart — …)` note.

---

## Per-slab handoff docs

All per-slab handoff docs live in `FrontendRust/docs/migration/handoff-slab-*.md`. These contain translation tables, step-by-step plans, gotchas, and examples specific to each slab. They remain useful as reference for understanding *why* a particular design choice was made during that slab.
