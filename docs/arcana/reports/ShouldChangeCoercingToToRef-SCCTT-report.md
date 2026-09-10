# Accuracy report: Should Change Coercing to toRef (SCCTT)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Impls.md:185-192 ("# Should Change Coercing to toRef")

## Verdict

This is a Scala-era design proposal ("we should just have astronomer insert toRef calls...") that references a pass, "Astronomer", which no longer exists in the Rust codebase — its responsibilities were absorbed elsewhere (scout/typing) and there is no `astronomer` module or "insert toRef" mechanism to point to. Worse, the section still frames this as a proposed future change ("Right now we're doing coercing... Instead, we should..."), but the only citation site in the code (src/integration_tests/tests/integration_tests_c.rs:449) talks about "the SCCTT fix" in the past tense, as something already done — and that test is entirely `#[ignore]`d with a commented-out body and `unimplemented!()`, so it proves nothing about current behavior either way. The doc is stale on both ends: the proposed mechanism's namesake pass is gone, and the one place that references SCCTT treats the proposal as resolved while the section itself still describes it as open. There's no live code today that can be checked against "astronomer inserting toRef calls," so this reads as obsolete rather than an accurate still-open TODO.

## Claims

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Right now we're doing coercing to make kinds into coords." | DRIFTED | src/typing/expression/expression_compiler.rs:64 (`evaluate_and_coerce_to_reference_expressions`) | Coercion from kind-like values to references still exists but under `coerce_to_reference_expression`/`evaluate_and_coerce_to_reference_expressions`, not under any name tied to "astronomer" |
| 2 | "Instead, we should just have astronomer insert toRef calls where it sees us trying to access a kind as a coord." | FALSE / OBSOLETE | no `Astronomer` symbol anywhere in src/ (`grep -rln Astronomer src` returns nothing) | The "Astronomer" pass this proposal is built on doesn't exist in the current (post-Scala) compiler; the proposal can't be evaluated against current architecture as written |
| 3 | "This will resolve a couple hacks, search for SCCTT." | FALSE (as a live instruction) | src/integration_tests/tests/integration_tests_c.rs:449, which is inside `#[ignore] fn test_narrowing_between_borrow_and_owning_overloads()` with `unimplemented!()` and a fully commented-out body | Searching for SCCTT today finds a single dead, ignored test, not live hacks pending this fix. Its comment ("Before the SCCTT fix, it couldn't resolve...") implies the fix already happened, contradicting the section's present-tense "Right now we're doing coercing" framing |

## Stale citation sites

src/integration_tests/tests/integration_tests_c.rs:449 — the section presents SCCTT as an unimplemented proposal to search hacks for; the actual site is a disabled, unimplemented test whose comment describes SCCTT as an already-applied fix in the past. Neither framing can currently be confirmed against live code since the test body is entirely commented out and the test itself never runs.

## Uncited sites that embody the arcana

None found — there is no current mechanism (astronomer-inserted `toRef`, or any equivalent kind→coord coercion pass) that plausibly implements this specific proposal to check for missing citations.

## Suggested rewrite

Since the underlying "Astronomer" pass is gone and the sole citation is dead/ignored code that treats the issue as already resolved, this section should either be deleted or replaced with a short historical note, e.g.:

> # Should Change Coercing to toRef (historical)
>
> (SCCTT)
>
> This was a Scala-era proposal to have the (now-removed) Astronomer pass insert explicit `toRef` calls instead of implicitly coercing kinds into coords. The Astronomer pass no longer exists post-Rust-port; kind→coord coercion today lives in `coerce_to_reference_expression` / `evaluate_and_coerce_to_reference_expressions` (src/typing/expression/expression_compiler.rs). The one test that referenced this id (`test_narrowing_between_borrow_and_owning_overloads` in src/integration_tests/tests/integration_tests_c.rs) is `#[ignore]`d and unimplemented, so whether the original overload-resolution bug is actually fixed is unverified — if this narrowing behavior still matters, it needs a real, un-ignored test rather than this arcana entry.
