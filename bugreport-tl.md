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
- **Tests assert correct behavior, not symptoms.** Don't pin current panic messages or buggy output.
- **No bug-class commentary in test bodies.** The "what went wrong" lives in the commit message. Tests are self-evident from name + code.
- **Tight assertions only.** `assert_eq!` or `ends_with(<deterministic suffix>)`. Never `contains() && contains()` (NSTDX).
- **Tests live in `<module>/tests/<module>_tests.rs`.** Mirror the `parsing/tests/` convention. No inline `#[cfg(test)] mod` in production source.
- **External-repro tests stay `#[ignore]`'d.** Out-of-tree-path / real-world probes document the bug; only the minimal in-tree repro joins the baseline.
- **Don't trust the bug-report's or JR's framing.** Verify the cited file:line and the call graph; hypotheses get tested, not adopted. Pre-existing precedent isn't license — if both sites are wrong, fix both.
- **Encoding / byte-vs-char is a recurring bug class.** Rust byte positions vs Scala UTF-16 code units; `chars().nth(byte_index)` is the canonical mistranslation.
- **Nondeterminism → stop and report.** Don't push through suspected races or probabilistic failures.
- **Characterize the work before scoping.** Is it Scala parity, or does it need new things? Absolute parity outranks tidiness.
