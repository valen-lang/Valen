# Plan document

Source: `/Users/verdagon/.claude/plans/please-plan-out-denying-vivid-eagle.md`
Session: e1757dd0-48e7-4058-b16b-6571914a06db

---

# Plan: Phase 1 — Error-Reporting Redesign (single-candidate surface-the-cause)

## Context

The architect's active Mission in `vcoord-handoff.md` ("Overload resolution & dispatch model redesign") proposes a 5-phase arc. **Phase 1** is scoped as the small, high-value, standalone quick win and is what this plan covers — Phases 2–5 are out of scope.

The architect's meta-point:
> "if there's one candidate, we dont print the `CouldntFindFunctionToCallT` error, we print the underlying cause why that one function didnt match."

The diagnostic on `compiler_tests::reports_when_rsa_callable_returns_wrong_element_type` showed the compiler **correctly detecting** the bool-vs-i32 mismatch the test was designed to surface, but reporting it as the generic `CouldntFindFunctionToCallT` instead of as the specialized `UnexpectedArrayElementType` the test asserts on. The inner rejection reason is `IFindFunctionFailureReason::InferFailure → FailedSolve → ITypingPassSolverError::ReturnTypeConflict { expected: i32, actual: bool }` — already correct data, just buried under the generic wrapper.

**Phase 1 outcome**: when overload resolution rejects exactly one candidate, the error variant + humanizer surface that single candidate's specific rejection reason directly. The wrapper "Couldn't find a suitable function name(args). Rejected candidates: Candidate 1 (of 1): …" text disappears for singleton rejections. Tests that pattern-match on `CouldntFindFunctionToCallT` for singleton-fail scenarios update to match the new variant. `reports_when_rsa_callable_returns_wrong_element_type` and probably several sister tests in the 87-test `compilation.rs:145` cluster un-`#[ignore]`.

This Phase 1 work does NOT include: auto-borrow coercion (Phase 2), bare-use target-aware materialization (Phase 3), namespace dispatch (Phase 4), typeclass-like reorganization (Phase 5). The interim "explicit `&` at callsites" convention from CHECKPOINT 22 stays in place — Phase 2 retires it later.

## Code landscape (from Phase 1 exploration)

### Where the data already exists (good news)

- `FrontendRust/src/typing/overload_resolver.rs:62-67` — `FindFunctionFailure { name, args, rejected_callee_to_reason: &'t [(ICalleeCandidate, IFindFunctionFailureReason)] }`. The per-candidate rejection list already rides through verbatim — no data is lost at error construction.
- `FrontendRust/src/typing/compiler_error_reporter.rs:55` — `CouldntFindFunctionToCallT { range, fff }`. Embeds the full `FindFunctionFailure`. The wrapping happens only at humanization time.
- `FrontendRust/src/typing/compiler_error_humanizer.rs:327-347` — `humanize_find_function_failure` is the wrapper site (`"Couldn't find a suitable function {}({}). Rejected candidates: …"`).

### Wrap sites (`FindFunctionFailure` → `ICompileErrorT::CouldntFindFunctionToCallT`)

Five sites construct the variant. All preserve `fff` verbatim. Centralizing the singleton check at a helper avoids per-site duplication:

- `FrontendRust/src/typing/overload_resolver.rs:774-778` — `attempt_find_function` wrapper.
- `FrontendRust/src/typing/array_compiler.rs:311-314` — array element resolution (path for the target RSA test).
- `FrontendRust/src/typing/expression/call_compiler.rs:104-108` and `110-113` — normal call resolution.
- `FrontendRust/src/typing/expression/expression_compiler.rs:443-446` — `wrap_in_implicit_clone` (path for `mutable_foreach` and the `implicit_clone(&Opt<_>)` failure cluster).
- `FrontendRust/src/typing/function/destructor_compiler.rs:33-37` — destructor resolution.

### Already-special-cased shape (template for "surface inner")

`FrontendRust/src/typing/expression/call_compiler.rs:65-103` already inspects `rejected_callee_to_reason` for the `as`-cast path — pattern-matches over the rejection list filtering for `InferFailure`/`FindFunctionResolveFailure` whose inner `ITypingPassSolverError::IsaFailed` indicates a downcast failure, then re-routes to `CantDowncastUnrelatedTypes`. **This is the existing precedent for "unwrap one specific reason"; the new singleton helper generalizes the same idea to any reason.**

### Humanizer gaps that must be filled

Currently panicking arms that block the redesign:

- `FrontendRust/src/typing/compiler_error_humanizer.rs:457` — `SpecificParamRegionDoesntMatch` arm.
- `FrontendRust/src/typing/compiler_error_humanizer.rs:461` — `InferFailure` arm. **This is the one the RSA target test needs.**
- `FrontendRust/src/typing/compiler_error_humanizer.rs:462` — `Outscored` arm.
- `FrontendRust/src/typing/compiler_error_humanizer.rs:484` — `ITypingPassSolverError::ReturnTypeConflict` arm (reached via `InferFailure` → `FailedSolve` → `ReturnTypeConflict`).

`InferFailure` humanization delegates to the solver-error humanizer (same shape as the existing `RuleTypeSolveFailure` arm at `:421-442` does for `RuneTypeSolveError`). The `solver_humanize_failed_solve` helper exists; the work is wiring `humanize_rule_error` into it for `ITypingPassSolverError`, then filling the `ReturnTypeConflict` arm in `humanize_rule_error` with `"Returned {actual}, but expected return type of {expected}"`-style output.

### Tests pattern-matching `CouldntFindFunctionToCallT`

7 distinct tests across `FrontendRust/src/typing/test/*.rs` plus 1 humanizer unit test. Most use `..` so they survive a variant rename if `fff` is preserved. The ones requiring real rewrites are the after-regions tests asserting on humanized strings (`after_regions_error_tests.rs:145, 217, 365, 578`), and `compiler_solver_tests.rs:122-125` / `compiler_ownership_tests.rs:189-192` asserting on `IImpreciseNameS::CodeName("drop")` (these would need to pivot through the new variant to find the imprecise name).

## Recommended design

### Step 1: Add `OnlyCandidateRejectedT` variant

In `FrontendRust/src/typing/compiler_error_reporter.rs` (alongside `CouldntFindFunctionToCallT` at `:55`):

```rust
/// Single-candidate overload-rejection: surface the inner rejection reason
/// directly instead of wrapping in CouldntFindFunctionToCallT. Constructed
/// at the FindFunctionFailure → ICompileErrorT boundary when
/// `rejected_callee_to_reason.len() == 1`.
OnlyCandidateRejectedT {
    range: &'t [RangeS<'s>],
    name: IImpreciseNameS<'s>,
    args: &'t [CoordT<'s, 't>],
    candidate: ICalleeCandidate<'s, 't>,
    reason: IFindFunctionFailureReason<'s, 't>,
},
```

Keep `CouldntFindFunctionToCallT` unchanged. The new variant carries everything needed to humanize without the wrapper.

### Step 2: Central wrap helper

In `FrontendRust/src/typing/overload_resolver.rs` (alongside the `FindFunctionFailure` definition at `:62`):

```rust
/// Convert a FindFunctionFailure into the appropriate ICompileErrorT
/// variant. Singleton-rejection failures surface the inner reason via
/// OnlyCandidateRejectedT; zero-rejection and multi-rejection failures
/// stay as CouldntFindFunctionToCallT.
pub fn find_function_failure_into_error<'s, 't>(
    typing_interner: &TypingInterner<'s, 't>,
    range: &'t [RangeS<'s>],
    fff: FindFunctionFailure<'s, 't>,
) -> ICompileErrorT<'s, 't> {
    if fff.rejected_callee_to_reason.len() == 1 {
        let (candidate, reason) = fff.rejected_callee_to_reason[0];
        ICompileErrorT::OnlyCandidateRejectedT {
            range, name: fff.name, args: fff.args, candidate, reason,
        }
    } else {
        ICompileErrorT::CouldntFindFunctionToCallT { range, fff }
    }
}
```

### Step 3: Update the 5 wrap sites

Replace each `ICompileErrorT::CouldntFindFunctionToCallT { range, fff: e }` constructor with a call to `find_function_failure_into_error(typing_interner, range, e)`. Sites:

- `overload_resolver.rs:775`
- `array_compiler.rs:311`
- `call_compiler.rs:104, 110`
- `expression_compiler.rs:443`
- `destructor_compiler.rs:33`

The existing `as`-cast special-case path at `call_compiler.rs:65-103` keeps its own `IsaFailed`-detection logic — that's an even more specific surfacing than the generic singleton path, and runs first.

### Step 4: Humanizer arm for the new variant

In `FrontendRust/src/typing/compiler_error_humanizer.rs` (alongside the existing `CouldntFindFunctionToCallT` arm at `:206-207`):

```rust
ICompileErrorT::OnlyCandidateRejectedT { range, name, args, candidate, reason } => {
    // Surface the inner rejection reason DIRECTLY — no "Couldn't find a
    // suitable function name(args). Rejected candidates: Candidate 1 (of 1)"
    // wrapper. Architect's quick-win.
    let args_str = args.iter().map(|t| humanize_templata(…)).collect::<Vec<_>>().join(", ");
    format!(
        "At {}:\n{}\n{}{}",
        humanize_range(invocation_range, …),
        humanize_candidate(…, candidate),
        humanize_rejection_reason(…, reason),
        "",  // any trailing newline normalization
    )
}
```

Exact wording is tuned to match the target test's expected output. The minimum is "drop the wrapper text" — what specifically replaces it is determined by what the target tests assert on.

### Step 5: Fill panicking rejection-reason arms

In `humanize_rejection_reason` (`compiler_error_humanizer.rs:416-468`):

- **`InferFailure { reason }`** at `:461`: mirror the existing `RuleTypeSolveFailure` arm at `:421-442` — delegate to `solver_humanize_failed_solve` with `humanize_rule_error` injected (for `ITypingPassSolverError`) instead of `humanize_rune_type_error` (for `RuneTypeSolveError`). The signatures align; the implementation is a near-copy.
- **`Outscored`** at `:462`: emit `"Outscored by another candidate"` (matches Scala `OverloadResolver.scala`).
- **`SpecificParamRegionDoesntMatch { rune, supplied_mutability, callee_mutability }`** at `:457`: emit `"Region for rune {rune}: supplied {supplied_mutability}, callee expects {callee_mutability}"` (matches Scala shape).

In `humanize_rule_error` (`compiler_error_humanizer.rs:470+`):

- **`ITypingPassSolverError::ReturnTypeConflict { expected, actual, .. }`** at `:484`: emit the same shape as `IConclusionResolveError::ReturnTypeConflictInConclusionResolve` at `:320-321` — `"Returned {actual}, but expected return type of {expected}"`. **This is the arm that lets the RSA target test produce a meaningful surface.**
- `CantGetComponentsOfPlaceholderPrototype` at `:483` — leave panicking unless its absence blocks a target test; speculatively filling unreached arms is out of scope.

### Step 6: Update tests

Re-run the suite after Steps 1–5 and triage. Three categories:

- **Tests that survive unchanged**: those using `CouldntFindFunctionToCallT { fff, .. }` for multi-candidate or zero-candidate failures. Most of the `..`-using tests fall here.
- **Tests that pivot pattern**: tests that fail with a singleton rejection now match `OnlyCandidateRejectedT { reason, .. }` instead. `compiler_solver_tests.rs:122-125` and `compiler_ownership_tests.rs:189-192` (asserting on `"drop"` name) pivot to `OnlyCandidateRejectedT { name, .. }` — the imprecise name lives in the new variant too.
- **Tests that update expected humanized text**: `after_regions_error_tests.rs:145, 217, 365, 578` re-snapshot their `assert_humanized_eq` strings against the new (shorter) output.

### Step 7: Un-ignore + verify

- Remove `#[ignore = "deferred at experimental-2 squash baseline"]` from `compiler_tests::reports_when_rsa_callable_returns_wrong_element_type` (line 2707 in `FrontendRust/src/typing/test/compiler_tests.rs`).
- Adjust the test's pattern-match if needed: it currently expects `ICompileErrorT::UnexpectedArrayElementType` (per the explore report). The "morally equivalent" mapping from the handoff doc is `OnlyCandidateRejectedT { reason: InferFailure { reason } }` where `reason.failed_solve` carries `ITypingPassSolverError::ReturnTypeConflict { expected: i32, actual: bool }`. Either:
  - (a) Update the test to match the new shape, OR
  - (b) Construct `UnexpectedArrayElementType` from `array_compiler.rs` when the singleton `ReturnTypeConflict` shape is detected at the wrap site (narrower, more specific path; test stays as-is).
  - **Recommend (a)** — generic singleton surfacing is what the architect's meta-point endorses; per-site specialization is the older bucket-6 framing that was rejected.

### Step 8: Sister-test survey

After (1)–(7), probe the 87-test `compilation.rs:145` cluster (currently tagged `#[ignore = "deferred at experimental-2 squash baseline"]`) and identify which now pass under the new error-reporting. Sample at minimum:

- `reports_when_rsa_callable_returns_wrong_element_type` (the canonical target).
- `compiler_lambda_tests::tests_lambda_and_concept_function` (one of the 8 typing failures).
- `compiler_lambda_tests::lambda_inside_template`.
- `compiler_solver_tests::pointer_becomes_share_if_kind_is_immutable`.
- `after_regions_tests::failure_to_resolve_a_prot_rules_function_doesnt_halt`.
- A handful of `integration_tests::tests::array_tests::array_map_*` (5+ tests in the cluster).
- `integration_tests::tests::while_tests::mutable_foreach` (the `implicit_clone(&Opt<_>)` path).

For each that flips green, remove the `#[ignore]`. For each still red, document the underlying reason in `vcoord-handoff.md` (which Phase 2/3/4 dependency it's blocked on). Phase 1 success is measured by the count of `#[ignore]`s removed, not by clearing the whole cluster.

## Critical files to modify

- `FrontendRust/src/typing/compiler_error_reporter.rs` (add `OnlyCandidateRejectedT` variant).
- `FrontendRust/src/typing/overload_resolver.rs` (add `find_function_failure_into_error` helper + update wrap site at `:775`).
- `FrontendRust/src/typing/array_compiler.rs` (update wrap site at `:311`).
- `FrontendRust/src/typing/expression/call_compiler.rs` (update wrap sites at `:104, :110`).
- `FrontendRust/src/typing/expression/expression_compiler.rs` (update wrap site at `:443`).
- `FrontendRust/src/typing/function/destructor_compiler.rs` (update wrap site at `:33`).
- `FrontendRust/src/typing/compiler_error_humanizer.rs` (add `OnlyCandidateRejectedT` arm, fill `InferFailure`/`Outscored`/`SpecificParamRegionDoesntMatch`/`ReturnTypeConflict` arms).
- Test files updated per Step 6 (representative paths): `FrontendRust/src/typing/test/{after_regions_error_tests.rs, compiler_tests.rs, compiler_solver_tests.rs, compiler_ownership_tests.rs, compiler_mutate_tests.rs, after_regions_tests.rs}` + un-ignores per Step 8.

## Verification

1. **Build clean**: `cargo build --manifest-path FrontendRust/Cargo.toml` after each step. Each humanizer fill is independent — partial completion still compiles.
2. **Targeted tests pass**:
   - `cargo nextest run --manifest-path FrontendRust/Cargo.toml -E 'test(reports_when_rsa_callable_returns_wrong_element_type)'` — target test, must pass with `#[ignore]` removed.
   - `cargo nextest run --manifest-path FrontendRust/Cargo.toml -E 'test(reports_when_ssa_callable_returns_wrong_element_type)'` — sister test, must STILL pass (regression guard).
3. **Full suite baseline**: `cargo nextest run --manifest-path FrontendRust/Cargo.toml --no-fail-fast > tmp/phase1-tests.txt 2>&1` — confirm 1090 passed baseline holds, and tally how many of the 112 deferred ignores now flip green.
4. **Sister-test survey** (Step 8): un-ignore each candidate one-by-one, run, document in `vcoord-handoff.md`.
5. **Suite count after Phase 1**: target 1090 + N passes (N = count of un-ignored sister tests), 0 failed, (120 - N) skipped. Phase 1 must NOT introduce new failures.

## Out of scope (explicit)

- Phases 2–5 of the redesign (auto-borrow at call sites, bare-use target-aware materialization, namespace dispatch, typeclass-like reorganization).
- Filling unreached panicking humanizer arms (e.g., `CantGetComponentsOfPlaceholderPrototype` at `:483`, `BadIsaSuperKind` at `:488`) unless they block a target test surface.
- Adding new `#[ignore]`s. Per `vcoord-handoff.md` critical reminder: **no `#[ignore]` additions**. Only removals.
- Changing the multi-candidate (`> 1` rejected) error path. The architect's framing is "ambiguity error" for multiple successes; multiple rejections stay as `CouldntFindFunctionToCallT` (showing all reasons) until a future phase addresses them.
- Scala parity backport. Scala wraps unconditionally too; the Rust-side singleton surfacing is intentional divergence from Scala, sanctioned by the architect's Mission.
