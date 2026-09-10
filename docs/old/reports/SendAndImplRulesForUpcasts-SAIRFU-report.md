# Accuracy report: Send and Impl Rules For Upcasts (SAIRFU)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templar/Infer Templar.md:356 ("# Send and Impl Rules For Upcasts (SAIRFU)")

## Verdict

Obsolete. The section describes a solver-rule mechanism — a "Send" rule (`CoordSendSR`) that short-circuits into an equality when sender/receiver kind is knowable up front, or replaces itself with an "Impl" rule (`DefinitionCoordIsaSR`/`CallSiteCoordIsaSR`) that checks subtyping once both sides are known — as part of the constraint solver. None of these rule variants exist in the current Rust typing pass. `IRulexSR` (postparsing/rules/rules.rs:33-45) has only `Equals`, `Literal`, `Lookup`, `Call`, `RuneParentEnvLookup`, `KindList`, `CallSiteFunc`, `DefinitionFunc`, `Resolve`, `BorrowRef`, `WeakRef`, `OwnRef` — no Send/Impl/Isa rule of any kind. The single citation site is not live code: it sits inside a block of Scala reference comments (`compiler_solver.rs:203-220`) that is entirely commented out and lists the old Scala `get_puzzles` match arms for rules that were never ported, including `CoordSendSR`. Upcast/subtype checking in the current compiler happens elsewhere as ordinary conversion logic (e.g. convert_helper.rs), not as a rule the solver replaces mid-flight. There is a live TODO (src/typing/typing-pass-todo.md:19) that even names `CoordSendSR` as a feature that was designed, reverted, and never landed — corroborating that this mechanism was deliberately not carried over to Rust, at least not yet.

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | There is a solver rule type that lets sender/receiver upcast constraints be represented without an illegal `X = MyInterface` / `X = MyStruct` conflict, called a "Send" rule (`CoordSendSR`). | FALSE / OBSOLETE | src/postparsing/rules/rules.rs:33-45 (no such variant); src/typing/infer/compiler_solver.rs:213 (only appears inside a commented-out Scala block) | No `CoordSendSR`, `DefinitionCoordIsaSR`, or `CallSiteCoordIsaSR` rule exists in the Rust `IRulexSR` enum. |
| 2 | The Send rule short-circuits to an equality when sending from a non-descendant or into a non-ancestor, otherwise replaces itself with an Impl rule via rule-replacing (see SRCAMP). | UNVERIFIABLE (mechanism doesn't exist) | same as above | Cannot be checked against current code; the replace-self machinery this depends on is gone with the rule. |
| 3 | Receiving into a generic requires the call site to guess the most specific type (see SMCMST). | UNVERIFIABLE | not investigated further given #1/#2 are moot | out of scope once the underlying rule mechanism is confirmed gone |

## Stale citation sites

src/typing/infer/compiler_solver.rs:213 — the comment `// See SAIRFU, this will replace itself with other rules.` sits inside a fully commented-out block transcribing the old Scala `get_puzzles` match (lines ~197-224), documenting rule kinds (`CoordSendSR`, `DefinitionCoordIsaSR`, `CallSiteCoordIsaSR`) that have no Rust counterpart and whose corresponding `IRulexSR::CoordSend(...)` arm is itself commented out just below it. This is dead reference material kept during the Scala→Rust port, not an active code site — the citation is not load-bearing for any compiling code, only for a reader trying to understand the historical Scala arm.

## Uncited sites that embody the arcana

None found. Upcast/subtype conversion logic exists (e.g. src/typing/convert_helper.rs, src/typing/templata_compiler.rs use `is_subtype`/`upcast`-style helpers) but it is implemented as direct conversion-compiler logic, not as a solver rule with short-circuit/replace semantics, so it does not embody this specific arcana's mechanism — it solves the same problem a different way.

## Suggested rewrite

This section documents a Scala-era solver mechanism (`CoordSendSR` short-circuiting into `DefinitionCoordIsaSR`/`CallSiteCoordIsaSR`) that was not carried into the Rust typing pass. The current `IRulexSR` enum (src/postparsing/rules/rules.rs) has no Send or Impl/Isa rule variant; upcast legality is instead checked as ordinary conversion logic outside the constraint solver (see src/typing/convert_helper.rs). A live TODO (src/typing/typing-pass-todo.md:19) records that a `CoordSendSR`-based fix was designed and reverted, so the rule-based approach this section describes may be revisited, but as written today it describes machinery that doesn't exist in the compiler. Treat this section as historical background on the Scala solver's design, not as documentation of current behavior — the sole remaining citation (src/typing/infer/compiler_solver.rs:213) is itself inside dead, commented-out reference code and should be removed or repointed if/when `CoordSendSR` actually lands.
