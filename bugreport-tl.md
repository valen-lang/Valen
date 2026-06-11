# Bug-Report TL Process

External real-world report → in-tree regression test asserting *correct* output → fix.

## Loop

1. **Probe.** Run the target real-world program through the binary; observe failure.
2. **Diagnose.** Trace to root cause. Test hypotheses against the actual call graph — don't fix from the surface symptom.
3. **Reduce to minimal in-tree repro.** Bisect / synthesize the smallest self-contained input that reproduces. Embed as a test.
4. **Write the test asserting *correct* output** (Scala-faithful). It fails now because of the bug; passes after the fix.
5. **Fix.** Apply the root-cause correction.
6. **Verify.** Regression test passes + full suite + SCPX.

## Rules

- **No fix without an in-tree minimal repro first.** External probes (out-of-tree files, real-world programs, file-path-dependent tests) prove the bug exists; they never license a fix on their own.
- **Print the test for explicit architect approval before the fix lands.** The test motivates the fix and pins the contract — get it ratified before embarking on the implementation.
- **Tests assert correct behavior, not symptoms.** Don't pin current panic messages or buggy output.
- **No bug-class commentary in test bodies.** The "what went wrong" lives in the commit message. Tests are self-evident from name + code.
- **Tight assertions only.** `assert_eq!` or `ends_with(<deterministic suffix>)`. Never `contains() && contains()` (NSTDX).
- **Tests live in `<module>/tests/<module>_tests.rs`.** Mirror the `parsing/tests/` convention. No inline `#[cfg(test)] mod` in production source.
- **Never modify the user's source.** Probes / hypothesis tests on real-world programs (VmdSiteGen etc.) run against `cp -R` copies in `/tmp/`, never in-place edits — and never `sed -i` anywhere.
- **False reduction.** A bisect step that changes the failure mode (new panic site or different error) has strayed onto a different bug. Back up; try a different turn. The new symptom is information, not progress.
- **Verify the in-tree repro fires at the same panic site, not just *any* error.** Run the test and confirm the panic file:line matches the probe before dispatching JR. A test that reaches a different failure silently widens scope and dispatches against the wrong arm.
- **Place the minimal repro at the earliest pass that triggers it.** If the panic fires in higher-typing, the test lives in `higher_typing/tests/`, not `pass_manager/end_to_end_test.rs`. End-to-end is the surface that surfaces the gap; the regression test belongs at the lowest layer that still reproduces it — narrower call graph, faster iteration, fewer downstream changes that could mask it.
- **External-repro tests stay `#[ignore]`'d.** Out-of-tree-path / real-world probes document the bug; only the minimal in-tree repro joins the baseline.
- **Don't trust the bug-report's or JR's framing.** Verify the cited file:line and the call graph; hypotheses get tested, not adopted. Pre-existing precedent isn't license — if both sites are wrong, fix both.
- **Encoding / byte-vs-char is a recurring bug class.** Rust byte positions vs Scala UTF-16 code units; `chars().nth(byte_index)` is the canonical mistranslation.
- **Nondeterminism → stop and report.** Don't push through suspected races or probabilistic failures.
- **Characterize the work before scoping.** Is it Scala parity, or does it need new things? Absolute parity outranks tidiness.
- **Hash-check the copy against canonical at session start.** `diff -rq <canonical> <our-copy>` every sibling tree before probing. A drifted copy surfaces phantom bugs.
- **`git status` + `git log --oneline -20` before debugging.** Glance for stubs, WIP, suspicious recent commits on every tree involved. Catches both committed and uncommitted drift.

## Probe → repro → fix loop (proven this session)

Run this loop when a real-world program (VmdSiteGen, VerdagonSite, anything binary-driven) surfaces a gap. Each iteration closes exactly one gap; the loop ends when the probe succeeds.

1. **Run the probe through the binary.** Capture exit code, stderr, the exact panic site (file:line:col).
2. **Classify** (per `Luz/skills/guardian-tl.md`):
   - **Class A:** unmigrated `panic!("implement: …")` placeholder. JR fills the body 1:1 from Scala.
   - **Class B:** a real Rust-side mis-migration. TL diagnoses; may dispatch JR for the actual fix.
   - **Class C:** the binary exits cleanly with a humanized error. **Compare against canonical Scala** (run the same input through `/Volumes/V/Vale/release-mac/valec`) — if Scala also rejects, it's a real program error and you're done; if Scala accepts, the divergence is yours to find.
   - **Class D:** a different bug.
3. **Build a minimal in-tree repro.** Synthetic first; bisect a real file only when synthesis fails after a real effort.
4. **Place it at the earliest pass that triggers it** (per the existing rule). The "earliest" can be obscured by upstream Err-on-stub failures in the test harness — see "Don't trust test-infra panics that look upstream" below.
5. **Print the test for architect approval, then dispatch JR.** Don't skip the print step even when the fix is "obviously" mechanical.
6. **Parity-review the JR diff** end-to-end before surfacing for `fire commit`. Watch especially for trivial-one-liner Scala arms JR filled-beyond-scope to keep SPDMX happy (that's correct; rule already documented).
7. **`fire commit`** on architect's literal phrase, then **rebuild + re-probe** to enter the next iteration.

## Bisecting cross-module triggers

Some panics only fire when multiple Vale packages are loaded together — never on any single package alone. Standard approach when the obvious single-file repro doesn't trigger:

1. Start with the full set that triggers (e.g. real `stdlib` + `vmdparse` + `parseiter` + `vmdsitegencmd`).
2. **Leave-one-out** each top-level package; the ones whose removal makes the panic *disappear* are required.
3. Within each required package, **leave-one-out each `import`** in the main file. Same logic.
4. Within each required imported file, **leave-one-out each function/struct/impl**. Each step preserves the same failure mode; if the failure mode changes, you've strayed (false reduction — back up).
5. Once you have the minimum set, **find the actual triggering function/site** by adding a one-liner `eprintln!()` inside the panic arm in the Rust source, capturing the struct id / name / range, then mapping that back to the source line that produced it. Revert the `eprintln!()` before committing.

This is how we minimized the `AnonymousSubstruct` repro from 1300 lines of real Vale to 7 lines of synthetic.

## Don't trust test-infra panics that look upstream

Typing-pass + instantiating-pass test infras both have an unmigrated `CompilerErrorHumanizer.humanize` stub at `typing/compilation.rs:213`. When the typing pass returns `Err`, the test harness short-circuits to that stub-panic, regardless of what the underlying Err is. If you see `panicked at typing/compilation.rs:213: Not yet implemented: CompilerErrorHumanizer.humanize`, that's **almost never** the real bug — the real bug is in the Err value above it.

Workaround: bypass `expect_compiler_outputs()` and call `get_compiler_outputs()` directly. Pattern-match on the returned `Err(ICompileErrorT::<variant>)` for the test assertion. We did this for `if_branches_must_move_same_variables` and `if_branches_moving_same_vars_different_order_compiles`.

Long-term fix: port `CompilerErrorHumanizer.humanize` into the test infra (the version we wired into `pass_manager::build` is reusable).

## Scala test conventions to mirror

Several Scala test conventions exist that aren't documented in our codebase but matter:

- **Don't use `>`, `<`, `>=`, `<=` in test Vale source.** `BUILTIN` package is not in `packages_to_build` in either Scala's or Rust's test infra (only `TEST_TLD`). Arithmetic operator lookup fails. Existing Scala instantiating tests (see `Frontend/InstantiatingPass/test/dev/vale/instantiating/InstantiatedTests.scala`) carefully avoid these. Use literal `true`/`false`/`0`/`1` for boolean/numeric bodies in tests.
- **Hand-defined interfaces over imported stdlib types** when synthesizing test programs. Inline `interface IF<R Ref, P Ref> { … }` beats relying on `stdlib.ifunction.IFunction1` — fewer dependencies and faster runs.
- **For Ok-expecting tests, use `compile.expect_compiler_outputs()`.** For Err-expecting tests, bypass it and use `compile.get_compiler_outputs()` per the previous section.
