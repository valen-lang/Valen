# Typing Pass Migration — TL Handoff

**Taking this over?** Read this doc first, then `quest.md` (with the overrides flagged below). Everything else is directory-local.

## One-paragraph orientation

We're porting `src/typing/` from Scala to Rust, slab by slab per `quest.md` §12. The typing pass holds ~60 concrete name types + ~70 concrete payload types + 9 env types + 53 expression-AST payload types + `CompilerOutputs<'s, 't>` accumulator with `PtrKey<'t, T>` newtype + `HinputsT<'s, 't>` output type + `run_typing_pass` entry-point glue, plus a god-struct `Compiler<'s, 'ctx, 't>` that replaces Scala's ~20 sub-compilers and 15 macros. Scout-pass data (`FunctionA`, `StructA`, interned strings/names) is arena-retained and referenced directly from typing output via `&'s`; typing-pass arena-allocated types carry `<'s, 't>`. **Slabs 0–9 are done** — every typing-pass data-definition family has real fields, the `TypingInterner<'s, 't>` has real bodies, envs are real, the expression AST is real, `CompilerOutputs` has real fields + real method signatures on all 54 methods, and `TemplataCompiler`'s 35 `object`-level static methods have been lifted into `impl Compiler` with real parameters/returns. Remaining work is **Slabs 10–13 (signature completion across ~122 sub-compiler methods) → Slab 14+ (body migration driven by failing tests)**. `cargo check --lib` is clean (0 errors, 0 warnings).

## Where we are

| Slab | Scope | Status | Tag / handoff |
|---|---|---|---|
| 0 | arena substrate | ✅ | (scaffolding; no tag) |
| 1 | leaf types (OwnershipT, primitive KindT payloads, leaf templatas) | ✅ | commit `9fd7641c` |
| 2 | name hierarchy (~60 concrete names, 22 sub-enums, IdT, ValT companions) | ✅ | `slab-2-complete` · `FrontendRust/docs/migration/handoff-slab-2.md` |
| 3 | Kind / Coord / Templata trio | ✅ | `slab-3-complete` · `FrontendRust/docs/migration/handoff-slab-3.md` |
| 4 | environments + real interner bodies | ✅ | `slab-4-complete` · `FrontendRust/docs/migration/handoff-slab-4.md` |
| 5 | expression AST (3 enums + 53 payload structs, `ast/expressions.rs`) | ✅ | `slab-5-complete` · `FrontendRust/docs/migration/handoff-slab-5.md` |
| 6 | `CompilerOutputs<'s, 't>` + `PtrKey<'t, T>` + `DeferredActionT` | ✅ | `slab-6-complete` · `FrontendRust/docs/migration/handoff-slab-6.md` |
| 7 | `HinputsT` residual cleanup + `Compiler` shell + `run_typing_pass` | ✅ | `slab-7-complete` · `FrontendRust/docs/migration/handoff-slab-7.md` |
| 8 | `CompilerOutputs` method signatures (54 sigs) | ✅ | `slab-8-complete` · `FrontendRust/docs/migration/handoff-slab-8.md` |
| 9 | `object TemplataCompiler` method signatures (35 sigs) | ✅ | `slab-9-complete` · `FrontendRust/docs/migration/handoff-slab-9.md` |
| **10** | **compiler.rs residuals + orphans + templata_compiler.rs 14 tail methods (~27 sigs)** | **⏳ next** | handoff doc **not yet drafted**; incoming TL writes it first |
| 11 | expression-layer sigs: expression/pattern/block/call_compiler (~39 sigs) | ⏳ | |
| 12 | solver + resolver sigs: infer/overload/impl/reachability (~35 sigs) | ⏳ | |
| 13 | function/citizen/macros sigs: function_compiler/function_body/destructor/struct_compiler_core/anonymous_interface_macro (~23 sigs) | ⏳ | |
| 14+ | method bodies, test-driven | ⏳ | |

## Design decisions that *override* `quest.md`

`quest.md` predates several design refactors we did during Slabs 2–4. If the doc and the code/reasoning-doc disagree, **the code and the reasoning doc win**. Sections that are out-of-date:

### `quest.md` §6.3 — `IdT` is monomorphic, not generic

Original: `IdT<'s, 't, T: Copy>` generic in the leaf-name type, with `widen` / `widen_to` / `try_narrow` conversion methods and widest-form-keyed interning.

Current: `IdT<'s, 't>` monomorphic — `local_name: INameT<'s, 't>` always at the widest form. Callers pattern-match to narrow, like Scala does at runtime (Scala's `+T <: INameT` is a JVM-runtime-erased phantom anyway — we matched that shape in Rust). `PrototypeT<'s, 't>` and `SignatureT<'s, 't>` are also monomorphic.

Rationale + alternatives (unsafe-transmute typed views, RawIdT+TypedIdT wrapper, etc.) are recorded in `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md`. All four alternatives are available post-migration; none fit the Scala-parity goal during migration.

### `quest.md` §6.5 — `KindT` is an inline wrapper, not interned

Original: `KindT` interned with `Struct(StructTT<'s, 't>)` (payloads inline).

Current: `KindT` is an inline-owned 16-byte Copy enum. Non-primitive variants hold `&'t StructTT` (payload arena-interned). Primitive variants (`Never`, `Void`, `Int`, etc.) stay inline — too small to warrant arena allocation. Same philosophy for the three Kind sub-enums (`ICitizenTT`, `ISubKindTT`, `ISuperKindTT`).

### `quest.md` §6.6 — `ITemplataT` is an inline wrapper too; `PrototypeTemplataT`'s inner T was also erased

Original: `ITemplataT` interned; `PrototypeTemplataT[T <: IFunctionNameT]` inner T "keep it, threaded through to `IdT<'s, 't, T>`".

Current: `ITemplataT` is inline-owned, not interned. Six of its variants hold `&'t` refs to interned leaf payloads (Coord/Kind/Placeholder/Prototype/Isa/CoordList); five hold `&'t` refs to heavy allocated-but-not-interned payloads (Function/StructDefinition/InterfaceDefinition/ImplDefinition/ExternFunction); the rest are inline Copy values (Integer, Boolean, Mutability, etc.). Heavy-templata `PartialEq`/`Eq`/`Hash` use `std::ptr::eq` on the scout-lifetime refs (scout canonicalizes those).

`PrototypeTemplataT` has no inner T — since `PrototypeT<'s, 't>` is monomorphic, `PrototypeTemplataT<'s, 't>` just holds `&'t PrototypeT<'s, 't>`.

### `quest.md` §3.1 — envs live in `'t` arena, not `'s`; sibling `IInDenizenEnvironmentT` enum

Original: §3.1 has `IEnvironmentT<'s, 't>` variants owning payloads by value and allocated via `scout_arena.alloc(...)` into the scout arena.

Current: envs live in the typing arena `'t` (a `'s`-allocated struct can't hold the `&'t` refs that `TemplatasStoreT` transitively requires, since `'s: 't` and `'t: 's` is false). `IEnvironmentT<'s, 't>` is a 9-variant inline wrapper whose variants hold `&'t FooEnvironmentT<'s, 't>`. There's also a sibling `IInDenizenEnvironmentT<'s, 't>` with the 6-variant in-denizen subset. `global_env` back-refs are `&'t GlobalEnvironmentT<'s, 't>`. See `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` for the rationale and the long-term per-denizen two-tier target; `FrontendRust/docs/architecture/typing-pass-arenas.md` is the current architecture reference; `FrontendRust/docs/migration/handoff-slab-4.md` is the executed Slab 4 spec.

### `quest.md` §6.1 & §1.5 — corrected interned/inline family lists

The refactored families per IDEPFL:

**Interned (dedup via `TypingInterner<'s, 't>`, 6 family-level HashMaps):**
- `INameT` family: ~60 concrete name structs, one shared `name_val_to_ref` map keyed on the tagged-union `INameValT<'s, 't, 'tmp>` enum (72 variants).
- `IdT<'s, 't>` / `IdValT<'s, 't, 'tmp>` — monomorphic.
- Interned Kind payloads family: `StructTT`, `InterfaceTT`, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `KindPlaceholderT`, `OverloadSetT` — one shared `kind_payload_val_to_ref` map keyed on `InternedKindPayloadValT<'s, 't>` (6 variants).
- Interned templata payloads family: `CoordTemplataT`, `KindTemplataT`, `PlaceholderTemplataT`, `PrototypeTemplataT`, `IsaTemplataT`, `CoordListTemplataT` — one shared `templata_payload_val_to_ref` map keyed on `InternedTemplataPayloadValT<'s, 't, 'tmp>` (6 variants).
- `PrototypeT<'s, 't>` / `PrototypeValT<'s, 't, 'tmp>` — singleton map.
- `SignatureT<'s, 't>` / `SignatureValT<'s, 't, 'tmp>` — singleton map.

~84 per-concrete intern methods (caller-facing API) dispatch through those 6 family-level `intern_<family>` methods via four `impl_intern_*_wrapper_*` macros. Pattern mirrors `scout_arena.rs`. One hand-written wrapper (`intern_coord_list_templata`) for the single `'tmp`+slice case the macro can't express.

**Inline-owned, NOT interned** (wrapper enums — stack-only rewraps via `From`/`TryFrom`):
- `INameT` + 21 name sub-enums (`IFunctionNameT`, `IStructNameT`, etc.)
- `KindT` + 3 Kind sub-enums (`ICitizenTT`, `ISubKindTT`, `ISuperKindTT`)
- `ITemplataT`
- `IEnvironmentT` + `IInDenizenEnvironmentT` (Slab 4)
- `IEnvEntryT` (Slab 4 — 5 variants)
- `IVariableT` + `ILocalVariableT` (Slab 4)

## Notable post-Slab-4 state

### `TypingInterner<'s, 't>` is fully implemented

No more `panic!()` stubs. `src/typing/typing_interner.rs` (560 lines) has the 6 family-level HashMaps in `Inner<'s, 't>`, real `intern_<family>` bodies mirroring `scout_arena.rs::intern_rune` / `intern_name`, and ~84 per-concrete wrapper methods via macros. The struct takes `&'t Bump` and exposes `alloc` / `alloc_slice_copy` / `alloc_slice_from_vec` wrappers so builders can arena-alloc non-interned data without reaching through a separate `Bump` handle. **The type now carries two lifetime params `<'s, 't>`, not `<'t>`** — sub-compiler call sites all flipped to `&TypingInterner<'s, 't>` in Slab 4 Step 2.

### Env types are real and shipping

9 concrete env structs (`PackageEnvironmentT`, `CitizenEnvironmentT`, `FunctionEnvironmentT`, `NodeEnvironmentT`, `BuildingFunctionEnvironmentWithClosuredsT`, `BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT`, `GeneralEnvironmentT`, `ExportEnvironmentT`, `ExternEnvironmentT`), 2 wrapper enums (`IEnvironmentT`/`IInDenizenEnvironmentT`), `GlobalEnvironmentT`, `TemplatasStoreT` + `TemplatasStoreBuilder`, `IEnvEntryT` 5-variant enum, `IVariableT`/`ILocalVariableT` + 4 concrete variable structs, and 9 env-specific builders (`PackageEnvironmentBuilder` etc.). 17 `From`/`TryFrom` bridges between wrappers and concretes. `TemplatasStoreT` uses slice-of-pairs + nested-slice layout per AASSNCMCX (no `HashMap`-in-arena); linear-scan lookup.

Env `Hash`/`PartialEq`/`Eq` are **manual impls**, not derived — they match Scala's `id`-based identity (or `(id, life)` for `NodeEnvironmentT`, `panic!("vcurious")` for `GeneralEnvironmentT`). Deriving would walk into `TemplatasStoreT`'s slices and diverge from Scala.

### Heavy-templata env refs flipped to `&'t`

Slab 3 originally wrote `FunctionTemplataT.outer_env`, `Struct/InterfaceDefinitionTemplataT.declaring_env`, `ImplDefinitionTemplataT.env`, and `OverloadSetT.env` as `&'s`, matching the old §3.1 spec that placed envs in the scout arena. Slab 4 flipped all 5 to `&'t`. The Slab-3 custom `ptr::eq`-based `PartialEq`/`Eq`/`Hash` on heavy templatas stayed green through the flip (ptr-eq is arena-agnostic).

### Box stubs deleted

`NodeEnvironmentBox`, `FunctionEnvironmentBoxT`, and the `IDenizenEnvironmentBoxT` trait are gone. The builder-freeze pattern subsumes Scala's mutable-box role: mutation happens in a stack-local `*Builder`, `build_in(interner)` freezes into `'t` as an immutable `&'t`. Scala `/* */` blocks for the deleted stubs stay as audit trail (the pre-commit hook only checks block content).

### Val types use content-based Hash, not ptr-based

Slab 4 caught a subtle bug: `*ValT` types (the interner lookup keys) using `ptr::eq`-based `Hash` would have broken heterogeneous lookup. Two stack-local Vals with identical content would hash to different addresses and miss the HashMap. Switched to derived `Hash`/`PartialEq`/`Eq` (content-based) so query-Val (`'tmp`) and stored-Val (`'t`) hash consistently. The `ptr::eq` treatment stays for the *canonical `&'t` refs*, which is where pointer identity is genuinely the intent.

### Pre-commit hook on `/* scala */` blocks (unchanged)

`.claude/hooks/check-scala-comments` does exact-match comparison on every `/* ... */` Scala block. Any edit inside rejects the commit. Load-bearing for the migration audit trail — Scala source is embedded next to every Rust definition as the spec. Explain this to anyone new touching the code.

### Notable post-Slab-9 state

### `CompilerOutputs` is shipping with real method signatures

`src/typing/compiler_outputs.rs` (~1300 lines): real 23-field struct (`return_types_by_signature`, `signature_to_function`, declaration trackers, env back-refs, type metadata, definitions, impls + reverse indexes, exports/externs, instantiation bounds, deferred queues). All HashMap keys go through `PtrKey<'t, T>` for pointer-identity hashing per quest.md §4.2. Env back-refs use `&'t IInDenizenEnvironmentT<'s, 't>` (not `&'s IEnvironmentT` as quest.md §4.1 originally specified — Slab 4 envs-in-`'t` override applies here too, plus Scala uses the narrower in-denizen wrapper). Reverse-index impls are `Vec<&'t ImplT<'s, 't>>` (not `&'t [...]`) because they're mutated as new impls are discovered; this is fine because `CompilerOutputs` is stack-owned, not arena-allocated, so AASSNCMCX doesn't apply. `CompilerOutputs::new()` initializes every field empty.

**All 54 method signatures are real as of Slab 8** (previously private free-fn stubs at file scope). Lifted into one-fn `impl<'s, 't> CompilerOutputs<'s, 't> where 's: 't { pub fn ... }` blocks. Mutating methods (`add_*`, `declare_*`, `defer_*`, `mark_*`) take `&mut self`; read-only methods (`lookup_*`, `get_*`, `peek_*`) take `&self`. Definitions returned by `&'t`, `IdT` passed by value, env params flipped to `&'t IInDenizenEnvironmentT<'s, 't>`. `get_instantiation_name_to_function_bound_to_rune` return preserves the `PtrKey<'t, IdT>` key wrapping. `add_instantiation_bounds` takes `&'t TypingInterner<'s, 't>`. Bodies panic-stubbed with `"Unimplemented: Slab 10 — body migration"` — the message's slab number is stale post-audit (body migration is now Slab 14+), but re-aligning 54+35 panic messages is deferred to when bodies land. See `handoff-slab-8.md` for the full signature translation table.

### `TemplataCompiler` object methods are lifted onto `Compiler` with real signatures

`src/typing/templata_compiler.rs` (~2000 lines): Scala has a split `object TemplataCompiler { ... }` (35 static utility methods) + `class TemplataCompiler(...) { ... }` (14 instance methods). The god-struct refactor unified both onto `Compiler<'s, 'ctx, 't>` — the 14 class-methods were lifted in the original Phase 2 god-struct pass and remain as bare `(&self)` stubs in impl blocks at lines 1034-1524 (signatures to be filled in Slab 10). The 35 object-methods were lifted in Slab 9 with real signatures, interleaved with the file head stubs they replaced, now in one-fn `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn ... (&self, ...) }` blocks. `interner` and `keywords` parameters were dropped across every signature (accessed via `self.typing_interner` / `self.keywords`). `bound_arguments_source: &'t dyn IBoundArgumentsSource<'s, 't>` on all 11 substitute_* methods. `get_placeholder_substituter` / `get_placeholder_substituter_ext` / `create_rune_type_solver_env` return `()` with TODO comments because their trait return types don't exist yet. See `handoff-slab-9.md` for the full Slab 9 translation details.

### `PtrKey<'t, T: ?Sized>` lives in its own module

`src/typing/ptr_key.rs` (~35 lines): newtype wrapper with manual `Copy`/`Clone`/`PartialEq`/`Eq`/`Hash`/`Debug` impls. `PartialEq` uses `std::ptr::eq`; `Hash` casts the inner `&'t T` to `*const ()` and hashes that. Manual `Copy`/`Clone` (not `#[derive]`) so they work for any `T: ?Sized` without requiring `T: Copy`. The `?Sized` bound is forward-compat for `dyn` / slice keys; nothing uses it yet but it's harmless. Registered as `pub mod ptr_key;` in `src/typing/mod.rs`.

### `DeferredActionT<'s, 't>` replaces the two `DeferredEvaluating*` stubs

Slab 6 deleted `DeferredEvaluatingFunctionBody` and `DeferredEvaluatingFunction` Rust struct stubs (their Scala `/* */` blocks stay byte-for-byte) and replaced them with a single 2-variant `DeferredActionT<'s, 't> { EvaluateFunctionBody { … }, EvaluateFunction { … } }` enum per quest.md §4.4. No `Box<dyn FnOnce>`, no captures — variant payloads are explicit `&'s` / `&'t` refs and small Copy values. The drain pattern is `VecDeque::pop_front` + match + call `Compiler` method with `&mut coutputs`; no self-borrow because once the action is popped, it doesn't alias anything inside `coutputs`.

`function_env` on `EvaluateFunctionBody` is `&'t FunctionEnvironmentT` (the concrete env type, matches Scala). `calling_env` on `EvaluateFunction` is `&'t IInDenizenEnvironmentT` (the narrow wrapper, also matches Scala).

### `HinputsT` has real Scala-parity fields; `run_typing_pass` entry point is wired

Slab 7 finished `hinputs_t.rs` data shape: 11 real fields (`structs`, `interfaces`, `functions`, `interface_to_edge_blueprints`, `interface_to_sub_citizen_to_edge`, `instantiation_name_to_instantiation_bounds`, `kind_exports`, `function_exports`, `kind_externs`, `function_externs`, `sub_citizen_to_interface_to_edge`), all Vec/HashMap-backed per the documented AASSNCMCX deviation (revisit during body migration if scale demands arena slices). The two `InstantiationReachableBoundArgumentsT` / `InstantiationBoundArgumentsT` placeholders flipped from `()` → `&'t PrototypeT<'s, 't>` now that Slab 3 made `PrototypeT` monomorphic. `HinputsT::new()` constructor not added — Slab 14+ will build materialization from `CompilerOutputs` fields directly.

`src/typing/compilation.rs` gained `pub fn run_typing_pass<'s, 'ctx, 't>(scout_arena, typing_interner, keywords, opts, program_a) -> Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>> where 's: 't` as the pass entry point (panic-stub body). `src/typing/compiler.rs` gained two panic-stub methods: `Compiler::compile_program(&self, &mut coutputs, &'s ProgramA) -> Result<(), ICompileErrorT>` and `Compiler::drain_all_deferred(&self, &mut coutputs)`. `Compiler::evaluate` panic-stub left untouched (Slab 14+ either folds into `compile_program` or deletes).

### Slab-8/9 panic-message slab-number drift

Slabs 8 and 9 emit `panic!("Unimplemented: Slab 10 — body migration")` across 89 lifted methods. Post-audit (Slab 9 end), body migration is now **Slab 14+**, not Slab 10 — the intervening Slabs 10-13 are signature rewrites. The panic messages are informationally stale but harmless: they run at runtime (never hit during static compilation or signature slabs), and quest.md §12.1 item 9 flags the drift explicitly. Slab 14+ will bulk-update the messages when bodies land.

### Notable post-Slab-5 state

### Expression AST is shipping

`src/typing/ast/expressions.rs` (~2100 lines): `ReferenceExpressionTE<'s, 't>` (48 variants), `AddressExpressionTE<'s, 't>` (5 variants), and the narrow wrapper `ExpressionTE<'s, 't>` (2-variant, `Reference(&'t _)` / `Address(&'t _)`, per §7.1 narrow-use rule — `ConstructTE.args` is the only payload that uses it). 53 payload structs have real Scala-parity fields with ATDCX derives (`#[derive(PartialEq, Debug)]` — no Copy/Clone; see f64 deviation below). Every sub-expression field is `&'t ReferenceExpressionTE` / `&'t AddressExpressionTE`; every collection is `&'t [...]` per AASSNCMCX. `IExpressionResultT<'s, 't>` is a 2-variant inline Copy enum (`Reference(ReferenceResultT) | Address(AddressResultT)`), both result structs hold a single `CoordT`.

Naming carve-out: the Scala trait `ExpressionT` became Rust `ExpressionTE` (per quest.md §7.1), matching the `TE` suffix convention for expression-related types. The Scala block still says `trait ExpressionT`; that's the frozen Scala source and the hook enforces it.

The expression enums are NOT interned (§7.2 — allocated in `'t`, deduplication unnecessary). No `*ValT` companions, no intern methods, no From/TryFrom bridges across the wrapper enums (construction and destructuring use plain variant syntax because payloads live inline inside variants, unlike names/kinds which wrapper-enum-hold `&'t` refs).

### Slab 5 derive deviation: `#[derive(PartialEq, Debug)]` on expression payloads, not the standard `(PartialEq, Eq, Hash, Debug)`

`ConstantFloatTE` holds a `value: f64`, which can't derive `Eq`/`Hash` (Rust's `f64` isn't `Eq`). The junior followed postparsing's precedent (`src/postparsing/expressions.rs` — `ConstantFloatSE` and `IExpressionSE` both `#[derive(Debug, PartialEq)]` only). The whole expression-AST family drops `Eq`/`Hash` uniformly for consistency: every payload struct in `expressions.rs` uses `#[derive(PartialEq, Debug)]`, and the three wrapper enums (`ReferenceExpressionTE`, `AddressExpressionTE`, `ExpressionTE`) follow suit. `IExpressionResultT` / `ReferenceResultT` / `AddressResultT` keep the full `Copy, Clone, PartialEq, Eq, Hash, Debug` — they only hold `CoordT`, no `f64` transitively.

Consequence: you cannot use an expression node as a HashMap key. That's fine — expressions aren't interned, and the typing pass doesn't hash-key on them. If Slab 6+ ever wants to key a map on an expression ref, use `std::ptr::eq` on the `&'t` ref directly (same pattern as heavy templatas), or promote the map to key on a stable id like `PrototypeT`.

This deviation supersedes the handoff-slab-5.md "Derives" rule ("`#[derive(PartialEq, Eq, Hash, Debug)]`"). If a future slab needs to re-revisit, the quickest fix would be wrapping `ConstantFloatTE.value` in a newtype that delegates Eq/Hash via `f64::to_bits()` — but don't do that without senior approval (it would diverge from postparsing's shape).

### Known residual items (post-Slab-9, pre-Slab-10)

- **~122 sub-compiler method signatures still incomplete** — methods already inside `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>` blocks but with bare `(&self)` or `()` placeholders where their Scala `/* def ... */` anchors show real params. Audit results drove the Slabs 10-13 split in `quest.md` §12.1. High-concentration files: `expression_compiler.rs` (21), `infer_compiler.rs` (14), `templata_compiler.rs` tail (14), `pattern_compiler.rs` (12), `overload_resolver.rs` (11). Clean files (for reference): `compiler_error_humanizer.rs`, `array_compiler.rs`, `edge_compiler.rs`, `convert_helper.rs`, `sequence_compiler.rs`, `struct_compiler.rs`, `struct_compiler_generic_args_layer.rs`, `local_helper.rs` (impl methods; 2 orphans remain), `infer/compiler_solver.rs`, plus 17 macro files and 4 function/ files.
- **`ICompileErrorT<'s, 't>`** is still a `_Phantom`-only enum with ~80 case-class variants in `/* */` blocks. Slab 14+ fills variants on-demand as bodies need them. `run_typing_pass`'s return type uses it symbolically — fine because nothing constructs error variants yet.
- **`IBoundArgumentsSource<'s, 't>`** is a marker trait (empty body + two empty unit structs) that Scala pattern-matches against. Slab 9 passes it as `&'t dyn IBoundArgumentsSource<'s, 't>` in signatures; Slab 14+ body migration flips it to a pattern-matchable enum when the first body demands it.
- **`IPlaceholderSubstituter`** trait doesn't exist on the Rust side yet. Referenced by `get_placeholder_substituter` / `get_placeholder_substituter_ext` return types — Slab 9 returns `()` with TODO comments; Slab 14+ defines the trait (or picks `Box<dyn>` / `impl Trait` return, TBD) and fills returns. `IRuneTypeSolverEnv<'s>` does exist (in `src/postparsing/rune_type_solver.rs`); `create_rune_type_solver_env` still returns `()` pending a return-type decision.
- **`Compiler::evaluate`** panic-stubbed Scala-named method in `compiler.rs` will fold into `compile_program` or be deleted in Slab 14+. Slabs 7-9 left it untouched.
- **Sub-compiler free-fn stubs** (`fn entry_matches_filter`, `fn entry_to_templata`, `fn lookup_with_name_inner`, etc.) in `env/environment.rs` and `env/function_environment_t.rs` — slice-pipeline artifacts. Slabs 10-13 per-file sweeps wire them into `Compiler` methods or delete.
- **`dispatch_function_body_macro`** and friends on `Compiler` aren't wired — they need env-based macro resolution, which is Slab 14+ work.
- **`IInfererDelegate` trait** stays vestigial because `compiler_solver.rs` fn sigs still reference `&dyn IInfererDelegate`. Slab 12 signature rewrites remove it.
- **`HinputsT`** has real Scala-parity fields, but all use `Vec`/`HashMap` (not arena slices) per a documented AASSNCMCX deviation. Body-migration territory. `HinputsT::new()` constructor not added either; Slab 14+ builds materialization logic from `CompilerOutputs` fields directly.
- **`HinputsT` lookup methods** (`lookup_struct`, `lookup_interface`, `lookup_edge`, etc., ~10 methods) stay as panic stubs in `hinputs_t.rs`. Slab 14+ body migration.
- **`LocationInFunctionEnvironmentT.path: Vec<i32>`** in `ast/ast.rs` violates AASSNCMCX (heap `Vec` inside a `'t`-arena-allocated conceptual type) and would leak if we ever run arena drop without a destructor pass. Not blocking; a future cleanup turns the `Vec` into `&'t [i32]`.
- **Expression-construction call sites downstream** (the sub-compiler files that construct `LetAndLendTE`, `IfTE`, `FunctionCallTE`, etc.) still use by-value `ReferenceExpressionTE` where the field type is `&'t`. Any such site that actually gets exercised currently panics with `panic!("Unimplemented: Slab N — allocate into arena")` or similar. Slab 11 (expression-layer sigs) resolves the signature half; Slab 14+ resolves the construction side.
- **`NodeEnvironmentT`-downstream call sites** in some sub-compiler files (`local_helper.rs`, `array_compiler.rs`) got `panic!("Unimplemented: Slab 8")` patches in Slab 4 where the type of a `&NodeEnvironmentBox` param couldn't be meaningfully translated without body migration. The `Slab 8` in those panic messages is stale — actual resolution is in Slabs 11-13 depending on file.
- **Typing storage → two-tier per-denizen arenas** is the scheduled post-signature-rewrite redesign. Migration-phase uses a single `'t` interner; the reasoning doc describes the two-tier model (`'out` outputs arena + per-top-level-denizen `'scratch` scratchpad arenas, with `EnvIdx` handoff), the cross-denizen edge audit justifying the split, and an LSP trajectory built on top. See `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md`. Out of scope until the build is clean end-to-end (after Slab 13).

## Key files / directories

| Path | Purpose |
|---|---|
| `quest.md` | design spec (with overrides above) |
| `TL-HANDOFF.md` | this doc |
| `FrontendRust/docs/architecture/typing-pass-arenas.md` | typing-pass arena architecture (current state + pointer to long-term reasoning) |
| `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` | two-tier per-denizen target + cross-denizen edge audit + LSP direction |
| `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md` | `IdT` monomorphic / typed-view decision |
| `FrontendRust/docs/reasoning/` | other design-decision docs (slice interning, arena maps, hook, output-data-ref-or-copy) |
| `FrontendRust/docs/migration/handoff-slab-2.md` | Slab 2 handoff (names) |
| `FrontendRust/docs/migration/handoff-slab-3.md` | Slab 3 handoff (Kind/Coord/Templata) |
| `FrontendRust/docs/migration/handoff-slab-4.md` | Slab 4 handoff (envs + interner bodies) — historical but the 16 Gotchas are reusable patterns |
| `FrontendRust/docs/migration/handoff-slab-5.md` | Slab 5 handoff (expression AST) — historical, with the "infinitely-sized stubs" flip noted upfront |
| `FrontendRust/docs/migration/handoff-slab-6.md` | Slab 6 handoff (`CompilerOutputs` + `PtrKey` + `DeferredActionT`) — historical, settled the 6-Gotcha env-override pattern |
| `FrontendRust/docs/migration/handoff-slab-7.md` | Slab 7 handoff (HinputsT residual + Compiler shell + `run_typing_pass`) — historical |
| `FrontendRust/docs/migration/handoff-slab-8.md` | Slab 8 handoff (CompilerOutputs method sigs, 54 lifts) — historical. The translation-rules table is the canonical reference for Slabs 10-13 — every signature-rewrite slab reuses it verbatim |
| `FrontendRust/docs/migration/handoff-slab-9.md` | Slab 9 handoff (TemplataCompiler object method sigs, 35 lifts) — historical. Extends the Slab 8 translation table with the `interner`/`keywords` drop, closure params (`impl Fn`), and `&'t dyn IBoundArgumentsSource` trait-param handling |
| `FrontendRust/docs/migration/handoff-god-struct-progress.md` | Phase 2 god-struct refactor progress notes |
| `FrontendRust/src/typing/names/names.rs` | ~3100 lines: all name types, IdT, From/TryFrom bridges, ValT companions + INameValT union |
| `FrontendRust/src/typing/types/types.rs` | KindT + sub-enums + concrete Kind payloads + InternedKindPayloadValT/T unions |
| `FrontendRust/src/typing/templata/templata.rs` | ITemplataT + payload structs + InternedTemplataPayloadValT/T unions |
| `FrontendRust/src/typing/ast/ast.rs` | PrototypeT, SignatureT, IdT-holding structs + IDEPFL Query wrappers |
| `FrontendRust/src/typing/ast/expressions.rs` | ~2100 lines: 3 enums (`ReferenceExpressionTE` 48 variants, `AddressExpressionTE` 5, `ExpressionTE` wrapper) + 53 payload structs + result types |
| `FrontendRust/src/typing/compiler_outputs.rs` | ~1300 lines: real `CompilerOutputs<'s, 't>` 23-field struct + `DeferredActionT` 2-variant enum + `CompilerOutputs::new()` + **all 54 methods in one-fn `impl` blocks with real sigs (Slab 8)** |
| `FrontendRust/src/typing/templata_compiler.rs` | ~2000 lines: **35 object-method sigs lifted onto Compiler in Slab 9** at file head; 14 class-method sigs still bare `(&self)` at lines 1034-1524 (Slab 10 target) |
| `FrontendRust/src/typing/ptr_key.rs` | `PtrKey<'t, T: ?Sized>` newtype with manual ptr-identity Copy/Clone/Hash/Eq |
| `FrontendRust/src/typing/hinputs_t.rs` | `HinputsT<'s, 't>` real-fielded (Vec/HashMap deviation noted); `InstantiationBoundArgumentsT` / `InstantiationReachableBoundArgumentsT` `()` → `&'t PrototypeT` flip done in Slab 7; `HinputsT::new()` + lookup method bodies deferred to Slab 14+ |
| `FrontendRust/src/typing/compilation.rs` | `TypingPassOptions<'s>` real, `TypingPassCompilation<'s,'ctx,'t,'p>` mostly panic-stubs, Slab 7 added `pub fn run_typing_pass(...)` entry point here (panic body) |
| `FrontendRust/src/typing/env/environment.rs` | 5 of 9 env types + wrapper enums + GlobalEnvironmentT + TemplatasStoreT + 6 builders |
| `FrontendRust/src/typing/env/function_environment_t.rs` | 4 of 9 env types + 4 builders + IVariableT/ILocalVariableT + 4 concrete variables |
| `FrontendRust/src/typing/env/i_env_entry.rs` | IEnvEntryT 5-variant enum |
| `FrontendRust/src/typing/typing_interner.rs` | 560 lines: 6-family HashMap design, ~84 per-concrete wrappers via macros |
| `FrontendRust/src/typing/compiler.rs` | god struct (`Compiler<'s, 'ctx, 't>`, 4 fields) |
| `.claude/hooks/check-scala-comments` | pre-commit hook guarding `/* scala */` blocks |
| `Luz/shields/` | project-wide shields (RSMSCPX, NCWSRX, ATDCX, IDEPFL, etc.) |

## How to continue

1. **Read this doc + `quest.md`** (with the overrides in mind). Also read `docs/architecture/typing-pass-arenas.md` for the current arena shape and `docs/reasoning/environments-per-denizen-long-term.md` for the long-term target — the latter records the cross-denizen audit that validates the target and is worth understanding before any future storage-layer work.
2. **Start Slab 10 (compiler.rs residuals + orphans + templata_compiler.rs tail, ~27 sigs)**. Handoff doc is **not yet drafted** — incoming TL's first task is to write `FrontendRust/docs/migration/handoff-slab-10.md` in the Slab 8/9 style (translation table, receiver classification, Gotchas). Design spec is `quest.md` Part 2 (god struct) + Slab 8/9 handoffs for the lift-to-Compiler pattern + the signature translation rules table. Scope: flip `()` placeholders in ~8 `compiler.rs` methods (`preprocess_struct`, `preprocess_interface`, `determine_macros_to_call`, `ensure_deep_exports`, `is_root_function`, `is_root_struct`, `is_root_interface`, `evaluate`) + 4-5 orphan static fns (`print`, `consecutive`, `is_primitive`, `get_mutabilities`, `get_mutability`); fill sigs for `local_helper.rs` 2 orphans + `struct_compiler.rs` 1 orphan; fill 14 bare-`(&self)` tail impls in `templata_compiler.rs` (the `class TemplataCompiler` instance methods). Budget ~3 hours. After Slab 10, Slabs 11-13 each target one sub-compiler layer (expression, solver, function/macros) on the same pattern.
3. **Don't commit.** The human handles all commits and tags. Hand back uncommitted working trees at slab boundaries; the human reviews and commits.
4. **Each subsequent slab** gets its own handoff doc. Tag naming for the human: `slab-N-complete`.
5. **The signature-rewrite phase ends at Slab 13.** After that the build is clean with all signatures real and all bodies panic-stubbed. Slab 14+ is body migration driven by failing tests — different shape of work (per-method instead of per-file-family).

## Suggested process for the incoming TL

- Spawn a junior for each slab. Write them a detailed handoff doc (Slab 2/3/4 style — prescriptive, Gotchas, references to shields + reasoning docs). Slab 4's 16-gotcha format turned out to be a good template — pre-answering design questions before the junior codes saves rework.
- Answer design questions the junior raises in the handoff *before* they code. Slab 3 Gotcha 1 (the KindT inline-vs-interned question) and Slab 4 Gotcha 1 (`'t` vs `'s` for envs) are examples of this done well — the answer was baked into the doc, not deferred to review.
- When a design diverges from `quest.md`, record the divergence in `FrontendRust/docs/reasoning/<topic>.md` with the chosen approach + alternatives considered + why-deferred. Mirror `idt-typed-view-alternatives.md` or `environments-per-denizen-long-term.md` shapes.
- **Never commit.** Juniors and TLs finish a slab, run `cargo check --lib` clean, self-review their diff, then hand back to the human for review with uncommitted changes in the working tree. The human reviews, directs any changes, and is the one who runs `git commit` / `git tag`. If a junior needs a local savepoint mid-slab, use `git stash` or a WIP branch; don't commit on `rustmigrate-z`. Handoff docs may describe work-organization "steps" or "checkpoints" — those are self-review savepoints, not commit points.
- **Expect and invite push-back.** Slab 4 had two rounds of significant TL corrections — one on interner design (per-concrete maps → 6 family-level maps), one on a family of box stubs the handoff missed. Slab 5 had a smaller one: the handoff mandated `#[derive(PartialEq, Eq, Hash, Debug)]` on expression payloads, but `ConstantFloatTE.value: f64` blocked that and the junior followed postparsing's `(PartialEq, Debug)` precedent instead — correct call, documented in the "Slab 5 derive deviation" subsection above. All three corrections came from the junior noticing something was off. Reward the instinct; handoffs are proposals, not spec.
- **Scope discipline.** Slab work is narrow by policy — a junior completing Slab N should touch only the files that slab owns. If edits land in `TL-HANDOFF.md`, `quest.md`, or reasoning docs, those should be announced in the hand-back summary, not folded silently into the slab diff. TLs should revert off-scope edits before review and invite the junior to propose them as a separate pass; otherwise audit-trail clarity suffers and each slab boundary stops being a clean review gate.
- After Slab 13 the build is clean with `panic!()` bodies in every method. Slab 14+ becomes test-driven; the shape of that work is different — per-method instead of per-file-family.

## Risks / open questions

- **LSP / long-running use**: scout arena retention through instantiation is memory-heavy; single-arena typing makes batch-only the safe mode. The reasoning doc's "Further future direction: LSP support" section sketches per-denizen arenas + DefId cross-denizen refs + local/global split interner + single-writer cleanup promotion as the evolution path. Concrete design deferred past the two-tier per-denizen refactor.
- **Post-migration design revisits**: the inline-owned-wrapper philosophy (Slab 2/3) makes casts free but gives up compile-time "this IdT's local_name is a FunctionName" assertions. `idt-typed-view-alternatives.md` lists four post-migration ways to re-introduce that. Not pressing.
- **`HinputsT` materialization shape**: deferred to Slab 14+ body migration. `HinputsT::new()` doesn't exist yet; Slab 14+ builds it from `CompilerOutputs` fields directly (no factory detour).
- **`TypingInterner` perf**: six `hashbrown::HashMap` maps with heterogeneous `'tmp`→`'t` lookup via `Equivalent`. Works today but hasn't been measured. If typing becomes the bottleneck at scale, profile before optimizing; the two-tier per-denizen redesign might be the right place to revisit.
- **Signature-rewrite fatigue**: Slabs 10-13 are mechanical but bulk work — ~122 method signatures. Each slab has a well-defined scope and the Slab 8/9 translation rules apply directly. If a slab exceeds ~4 hours, split it in the next handoff rather than pushing through.

Questions? `quest.md` + the reasoning docs + the Slab 2–9 handoffs have the context. When in doubt, prefer Scala parity over Rust-idiomatic optimization — that's been the guiding principle through Slabs 2–9.
