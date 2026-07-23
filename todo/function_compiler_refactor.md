# Obligation tokens for function compilation

## Context

`src/typing/function/` is organized as a five-layer stack (`function_compiler` → `..._closure_or_light_layer` → `..._solving_layer` → `..._middle_layer` → `..._core` → `function_body_compiler`). The layering exists to answer one question: *for a given function, what is it supposed to think about?* You learn a method's concern by reading its neighbors.

That worked in Scala, where each layer was a class with a constructor-injected delegate — a middle-layer method physically could not call the closure layer. The god-struct refactor deleted that (two tombstones remain: `// deleted: delegate trait removed per god-struct refactor`). Every file is now `impl Compiler`, only 3 of ~50 functions are private, and `self.` reaches everything. The layering is a seating chart, not a boundary.

Three consequences, all measured in the current tree:

1. **The concerns are tracked by hand, at runtime.** `check_not_closure` is called at the top of 3 methods and `check_closure_concerns_handled` at the top of 4 more — seven runtime assertions that are exactly "did you discharge this concern, or explicitly decline it." `middle_layer` additionally opens two different methods with a near-verbatim duplicated loop asserting every generic param's rune resolves.

2. **Clients re-derive the recipe and get it wrong.** There are **17 hand-rolled `FunctionHeaderT { … }` literal constructions**. Only 2 are in `function_compiler_core.rs`; **13 are in body macros** (`struct_constructor`, `struct_drop`, `abstract_body`, `same_instance`, `as_subtype`, `lock_weak`, all seven `rsa_*`/`ssa_*`), and 2 are in tests. Twelve of the thirteen macro copies are byte-identical modulo one equivalent spelling of `maybe_origin_function_templata`. Worse: **only 4 of those 13 macros call `declare_function_return_type`; the other 9 rely on `core.rs` doing it.** Nothing distinguishes correct from lucky — `declare_function_return_type` merely tolerates a matching duplicate.

3. **Closures are accommodated, not desugared.** Closure handling is smeared across 6 files in 4 directories — `function_compiler.rs`, `closure_or_light_layer.rs`, `struct_compiler{,_core,_generic_args_layer}.rs`, `expression_compiler.rs`, `function_environment_t.rs` — because nothing lets downstream code *demand* that closuredness is already resolved.

**Intended outcome:** replace the layering's enforcement job with obligation tokens — small types that prove a concern was either calculated or explicitly declined — and make them unavoidable by sealing the outputs that every path must produce. Ordering constraints move onto token constructors. The layering's narrative job moves to the (already-existing) doc. The files then reorganize onto the subject axis, with closures as the first extraction.

**Prerequisite (hard gate):** typing must be compiling and the suite mostly green. As of writing it is RED at 176 errors, and the macros are not yet onion-clean (`lock_weak_macro.rs` still calls `KindT::new(OwnershipT::Borrow, …)`). Do not start before then. This lands as one atomic change — sealing the type breaks all 17 sites at once.

## Existing pieces to reuse

- **`MustIntern`** (`typing/typing_interner.rs:15`) — `pub struct MustIntern(());`, private field, module-private constructor. This *is* the token pattern, already load-bearing for pointer-identity equality. Its doc comment is the spec to copy. Note it derives `Copy` — fine here (arena types must; `alloc<T>` itself has no `Copy` bound, only `alloc_slice_copy`).
- **`_sealed: ()`** — the anonymous variant, on `RangeS` (`utils/range.rs:45`), `CoordH` (`final_ast/types.rs:25`), and the `*TE` nodes (`typing/ast/expressions.rs`). Same mechanism, unnamed.
- **`FunctionHeaderT::new`** (`typing/ast/ast.rs:339`) — already a private, `panic!("Unimplemented")` stub. The seam exists.
- **`check_not_closure` / `check_closure_concerns_handled`** (`closure_or_light_layer.rs:256`, `solving_layer.rs:311`) — become the two constructors of `ClosureDisposition` verbatim.
- **`DeferredActionT::EvaluateFunctionBody`** (`typing/compiler_outputs.rs`, drained at `typing/compiler.rs:1248`) — already a hand-rolled partial token bundle. It becomes the typed one.
- **`docs/old/Compiler/Templar/FunctionTemplar.md`** — the arcana the code points at via *"See FunctionCompiler doc for what outer/runes/inner envs are"*. Defines near/runed/named env. Relocate and de-Scala it; do not rewrite from scratch.

## Design

### The tokens (`typing/function/obligations.rs`, new)

Each is a struct with private fields in this module, constructed only by a named `pub fn` that performs the calculation or the decline-check. Ordering constraints live on the constructors, taken **by reference**; only `seal_*` takes tokens by value.

| Token | Discharged by | Declined by | Replaces |
|---|---|---|---|
| `ClosureDisposition` | `closured(vars, entries)` ← `make_closure_variables_and_entries` | `light(function)` ← `check_not_closure`'s assertion | 7 runtime assertions |
| `RunesSolved` (carries conclusions) | `new(&ClosureDisposition, conclusions)` | — | 2 duplicated assertion loops |
| `ParamsAssembled` (carries `&'t [ParameterT]`) | `new(&RunesSolved, params)` ← `assemble_function_params` | — | subsumes the per-param `evaluate_maybe_virtuality` |
| `ReturnTypeDisposition` | `declared(&RunesSolved, KindT)` | `inferred_from_body()` | gives `Option<KindT>`'s `None` a meaning |
| `BoundsRecorded` (carries `&'t InstantiationBoundArgumentsT`) | `new(&RunesSolved, bounds)` | — | ~5 scattered `add_instantiation_bounds` sites |
| `BodyDisposition` | `evaluated(expr)` / `generated(expr)` / `extern_()` | `deferred(action)` | untyped defer/resume |
| `DestructorObligation` | `destructee_moved()` ← the unstackified check in `evaluate_function_body` | `not_a_destructor()` | `is_destructor: bool` threaded 5 layers |

Seven tokens. Virtuality is deliberately *not* separate — `assemble_function_params` already calls `evaluate_maybe_virtuality` per param, so `ParamsAssembled` carries it.

### The seals

Two sealed outputs, because the deferred path legitimately produces a header before a body exists:

```rust
// typing/function/header.rs
pub fn seal_header(
    &self, coutputs: &mut CompilerOutputs<'s,'t>,
    id, attributes,
    closures: ClosureDisposition<'s,'t>,
    runes:    RunesSolved<'s,'t>,
    params:   ParamsAssembled<'s,'t>,
    ret:      ReturnTypeDisposition<'s,'t>,
) -> &'t FunctionHeaderT<'s,'t>;   // + declare_function_return_type, exactly once

pub fn seal_definition(
    &self, coutputs: &mut CompilerOutputs<'s,'t>,
    header: &'t FunctionHeaderT<'s,'t>,
    bounds: BoundsRecorded<'s,'t>,
    body:   BodyDisposition<'s,'t>,
    destructor: DestructorObligation,
) -> &'t FunctionDefinitionT<'s,'t>;  // + add_function + add_instantiation_bounds
```

`FunctionHeaderT` and `FunctionDefinitionT` each gain a private `_sealed: ()`, so these are the only constructors. Nothing can *return* from function compilation without supplying every token — the obligation is enforced by the return type, not by a comment.

`seal_header` runs twice on the deferred path (provisional, then final); `declare_function_return_type` already tolerates a matching duplicate. `seal_definition` runs exactly once. `FunctionDefinitionT` is cheap to seal — only 3 construction sites, all already in `core.rs`.

### Macros call `finish` themselves

Each of the 13 macros calls `seal_header` with its own tokens rather than having `core.rs` build one header centrally. The upstream tokens are threaded into `FunctionBodyMacro::generate_function_body` (`typing/macros/macros.rs:39` — a closed enum dispatch, 14 arms, no external implementors, so the signature change is bounded). Each macro author gets the checklist at their own call site, which is the point: the recipe should be visible at each client, not hidden in core.

`struct_constructor_macro` is the one genuine outlier — it builds a header for a *different* id/params/return than its env — and that stays legal, it just supplies its own ingredients.

### File reorganization (subject axis)

The stage axis and the subject axis mostly already coincide. The one concern smeared across every stage is closures — which is why it's the extraction.

| Current | Target |
|---|---|
| — | `obligations.rs` (new, the tokens) |
| — | `closures.rs` (new) ← `evaluate_closure_struct` + `determine_closure_variable_member` from `function_compiler.rs`, `make_closure_variables_and_entries` from the closure layer, the closure arms at `expression_compiler.rs:122` (read) and `:206` (mutate) |
| `function_compiler.rs` | `results.rs` (the 12 result structs/enums); forwarders folded into their single targets; file deleted |
| `function_compiler_closure_or_light_layer.rs` | closure half → `closures.rs`; env-building half → `env_building.rs` |
| `function_compiler_solving_layer.rs` | `solving.rs` |
| `function_compiler_middle_layer.rs` | `signature.rs` |
| `function_compiler_core.rs` | `header.rs` (+ the two `seal_*`) |
| `function_body_compiler.rs` | `body.rs` |
| `destructor_compiler.rs` | unchanged (a service, not a stage) |
| `virtual_compiler.rs` | **deleted** — `pub struct VirtualCompiler {}`, empty, zero references |

Also: dedupe `ResultTypeMismatchError`, defined identically twice (`function_body_compiler.rs:128`, `function_compiler_core.rs:19`). Drop the `_closure_or_light` / `_solving` / `_core` method suffixes, which exist only because the god-struct collapse merged the receivers (`_2` suffixes are inherited from Scala and can go too). Settle `near_env` vs `outer_env` — same type, two names, four sites — on the doc's term (`near env`).

## RFIGA

Most of this is type-level work, so for several slices **the compiler is the test**: sealing a type makes exactly the offending sites fail to build, which is a genuine red with a known expected reason. Where behavior actually changes (registration unification, closures), there are real runtime tests. Per `docs/skills/tdd.md`, F and G are explicit stops — do not collapse them.

1. **Tokens exist and thread through the main (non-macro) path.**
   * R: add `obligations.rs` with all 7 tokens; convert `check_not_closure` and `check_closure_concerns_handled` into `ClosureDisposition::{light, closured}`; make the 7 call sites take the token.
   * F: `cargo build --manifest-path FrontendRust/Cargo.toml --lib` → expect failures at exactly those 7 sites plus their callers. Confirm the list matches before proceeding.
   * I: thread `ClosureDisposition` down `env_building` → `solving` → `signature` → `header`; add the remaining 6 tokens with their by-reference ordering constraints.
   * G: build clean, 0 warnings.
   * A: `cargo nextest run --manifest-path FrontendRust/Cargo.toml`, then the same with `VALE_TEST_BACKEND=wasi`.

2. **Seal `FunctionHeaderT`; convert all 17 construction sites.**
   * R: add `_sealed: ()` to `FunctionHeaderT`; add `seal_header` taking the token bundle.
   * F: build → expect **exactly 17** private-field errors: 2 in `function_compiler_core.rs`, 13 in `typing/macros/**`, 2 in `typing/test/compiler_tests.rs`. Confirm the count and locations; a different count means the survey is stale — stop and re-survey.
   * I: convert each site to `seal_header`, threading tokens into `FunctionBodyMacro::generate_function_body`.
   * G: build clean.
   * A: both backends.

3. **Seal `FunctionDefinitionT`; unify registration.**
   * R: add a test asserting that after compiling a program exercising several macro-generated functions (an array program: `rsa_new`/`rsa_len`/`rsa_push` plus a struct with a drop), **every** function registered in `CompilerOutputs` has both its return type declared and its instantiation bounds recorded.
   * F: run it. Expect failure if any of the 9 macros that skip `declare_function_return_type`, or any that skip `add_instantiation_bounds`, leaves a gap. If it passes as-is, say so plainly — that means the invariant already holds by luck, and the slice's value is pinning it.
   * I: add `_sealed` to `FunctionDefinitionT`; move `add_function` + `add_instantiation_bounds` into `seal_definition`; convert the 3 sites; delete the now-redundant per-macro `declare_function_return_type` calls in `struct_drop`, `rsa_new`, `ssa_drop_into`, `ssa_len`.
   * G: re-run; expect pass.
   * A: both backends.

4. **Extract closures behind `ClosureDisposition`.**
   * R: `typing/test/compiler_lambda_tests.rs` is the behavioral guard. Add one test asserting a captured variable's member type is a borrow of the captured local (the "Captured own is borrow" rule that `determine_closure_variable_member` implements) — currently unpinned by any test.
   * F: run it; confirm it passes *before* the move (this one is a characterization test protecting a move, so a green start is expected and correct — record the baseline).
   * I: create `closures.rs`; move the closure code listed above; wire the two `expression_compiler` arms.
   * G: re-run the lambda tests + the new one; expect unchanged results.
   * A: both backends.

5. **De-layer: rename files, drop suffixes, delete dead weight.**
   * R: pure mechanical; the build is the test.
   * F: perform the renames/moves; build → expect only unresolved-path errors, no semantic errors. Any semantic error means a move was wrong — stop.
   * I: fix the paths; delete `virtual_compiler.rs`; dedupe `ResultTypeMismatchError`; drop the `_closure_or_light`/`_solving`/`_core`/`_2` suffixes; unify `near_env`/`outer_env`.
   * G: build clean, 0 warnings.
   * A: both backends.

6. **Relocate the arcana doc.**
   * R/F/I/G: no code. Move `docs/old/Compiler/Templar/FunctionTemplar.md` to `docs/architecture/`, retitle off "Templar", update the near/runed/named env prose to the current type names, and fix the two stale `// See FunctionCompiler doc` pointers in `solving.rs` to name the real path.
   * A: full suite once, to confirm nothing was touched accidentally.

## Verification

- Per-slice: `cargo build --manifest-path FrontendRust/Cargo.toml --lib` (0 errors, **0 warnings** — the repo standard), then `cargo nextest run --manifest-path FrontendRust/Cargo.toml`, then `VALE_TEST_BACKEND=wasi cargo nextest run --manifest-path FrontendRust/Cargo.toml`. Both backends are the gate per `fire-commit-config.md`.
- Pipe every run to one fixed file in `./tmp/` for the session; never chain a build with `| tail`/`| grep`.
- **The load-bearing end-state check:** grep for `FunctionHeaderT {` and `FunctionDefinitionT {` — both must return **zero** hits outside `typing/ast/ast.rs` and `typing/function/header.rs`. If a literal construction survives anywhere, the seal is not actually closed and the guarantee is fiction.
- Confirm `git grep -n "check_not_closure\|check_closure_concerns_handled"` returns only the two token constructors in `obligations.rs`.
- Confirm `git grep -n "closured\|closure_struct\|CapturedVariableT"` under `typing/function/` hits only `closures.rs` (name types in `names/names.rs` and env shape in `function_environment_t.rs` legitimately remain outside).

## Risks

- **Atomic by nature.** Slice 2 breaks 17 sites simultaneously; there is no partial landing. Budget for it as one sitting.
- **Do not derive `Clone` on the tokens.** `Copy` is unavoidable for arena types and is acceptable (it costs *exactly-once*, not *did-you-do-it*, and double-registration is already caught by the existing `assert!(coutputs.lookup_function(header_sig).is_none())`). But a reflexive `Clone` derive plus token laundering across denizens would silently gut the scheme. The real defense is that tokens carry their evidence and `seal_*` actually uses it — a laundered token produces a wrong answer loudly rather than passing a vacuous check.
- **The declined-constructor must stay as hard to obtain as the discharged one.** `ClosureDisposition::light()` must keep performing `check_not_closure`'s assertion. A token you can conjure is worse than no token, because it looks like a guarantee.
- **Ordering becomes opt-in.** The layering gave sequence for free (unenforced). After this, any ordering not encoded on a token constructor is genuinely unordered. Be deliberate about which orderings are real rather than assuming the old file order was meaningful.
- **A full token set proves the calculations ran, not that they were correct.** Same guarantee as any type. Worth stating so no one later mistakes it for a correctness proof.
