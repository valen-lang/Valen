# Accuracy report: Know If We're the Only Tether (KIWOT)

Audited against the working tree on 2026-09-06. Source: docs/old/HGM V16, V17, V18.md:167-235

## Verdict
This is a speculative brainstorm entry under a "Tools" section explicitly framed as "a lot of techniques found when exploring the conundrum... could be useful to keep around, for the next time we discover a flaw." It proposes tagging borrow refs with a 1-byte "tether" storing the stack height of the referencing local, to cheaply detect uniqueness. The text itself flags its own scheme as broken mid-document ("**this whole == H thing might not work... madness ensues.**") and lists several unresolved alternative encodings (8-bit refcount, compile-time number + stack pointer, thread-local Q-prime scheme) without settling on one. No claim in the section asserts anything about code as it currently exists — it's forward-looking design exploration for the old generational-references memory model, and the current Rust compiler's borrow checker (src/typing/borrow_checker/) uses a wholly different mechanism (group-based aliasing/noalias facts, per the recent commits on this branch), not per-object tether bytes. There are zero citations of KIWOT anywhere in src/, Backend/, or docs/, and no "tether" concept appears in the current codebase except an unrelated test fixture name (Backend/test/tethercrash.vale, which tests something else and does not implement this scheme). Recommendation: leave it in docs/old/ (or delete) — it is not-an-arcana and has nothing to migrate.

## Claims
This section is a design musing / open technique proposal, not a set of assertions about existing code. It contains no checkable claims about the current compiler. Per the audit instructions, no claims table is produced.

## Stale citation sites
None — grep -rn -w "KIWOT" across src/, Backend/, docs/ (excluding the source file itself) returns zero matches.

## Uncited sites that embody the arcana
None found. The current borrow checker (src/typing/borrow_checker/aliasing_info.rs, groupify.rs) solves the "am I the sole reference" problem via static group analysis and noalias-attribute derivation at compile time, not via any runtime tether/stack-height byte scheme, so there is no code this concept would be cited from.

## Suggested text
Not applicable — verdict is not-an-arcana (D3 kind; suggested text only required for major-inaccuracies/obsolete verdicts on load-bearing content, and this section has no real-world code to reconcile).
