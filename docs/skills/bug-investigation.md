---
name: bug-investigation
description: External real-world report → minimal in-tree regression test asserting correct output → fix. Use when a real-world program surfaces a compiler bug.
g_read_when: Read when an external real-world program surfaces a compiler bug and you need to reduce it to a minimal in-tree repro before fixing.
g_mention_in:
  - CLAUDE.md
---

# Bug Investigation

This is for going from a real-world report or failure to an in-tree regression test that has a *failing* assertion that asserts correct output.

The deliverable is a *failing* test. We want a failing minimal test that illustrates the problem.

## Loop

1. **Probe.** Run the target real-world program through the binary; observe failure.
2. **Diagnose.** Trace to root cause. Test hypotheses against the actual call graph — don't fix from the surface symptom.
3. **Reduce to minimal in-tree repro.** Bisect / synthesize the smallest self-contained input that reproduces. Embed as a test.
4. **Write the test asserting *correct* output.** It fails now because of the bug; passes after the fix.

## Rules

- **No fixing.** This bug-investigation skill is about investigating, not fixing.
- **Tests assert correct behavior, not symptoms.** Don't pin current panic messages or buggy output.
- **No bug-class commentary in test bodies.** The "what went wrong" lives in the commit message. Tests are self-evident from name + code.
- **Tight assertions only.** `assert_eq!` or `ends_with(<deterministic suffix>)`. Never `contains() && contains()`.
- **Tests live in `<module>/tests/<module>_tests.rs`.** Mirror the `parsing/tests/` convention. No inline `#[cfg(test)] mod` in production source.
- **Never modify the implementation logic or existing tests.** You're allowed to add printouts and add new investigatory tests, but don't modify existing logic or existing tests.
- A "false reduction" is a bisect step that accidentally changes the failure mode (new panic site or different error), in other words it has strayed onto a different bug. Back up; try a different turn. The new symptom is information but not progress.
- **Verify the repro fires at the same panic site, not just *any* error.** Run the test and confirm the panic file:line matches the probe. A test that reaches a different failure means that's on the wrong track.
- **Place the minimal repro at the earliest pass that triggers it.** If the panic fires in higher-typing, the test lives in `higher_typing/tests/`, not `pass_manager/end_to_end_test.rs`. End-to-end is the surface that surfaces the gap; the regression test belongs at the lowest layer that still reproduces it — narrower call graph, faster iteration, fewer downstream changes that could mask it.
- **Don't `#[ignore]`** anything. The user will tell you explicitly if they want something #[ignore]d.
- **Don't trust the bug-report's framing.** Verify the cited file:line and the call graph; hypotheses get tested, not adopted. Pre-existing precedent isn't license — if both sites are wrong, fix both.
- **Nondeterminism → immediately stop and report.** Don't push through suspected races or probabilistic failures. Nondeterminism bugs are P0 and require pausing.

## Wind-down

After a successful investigation, please give some concise feedback about:
 * What could have been better.
 * What logging/error/output information would have helped us identify the cause faster.
 * What processes from other teams/projects might have aided us here or prevented the original problem.
 * What type system guarantees might have prevented this problem.
