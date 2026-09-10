# Accuracy report: Possible Values Shouldnt Be Used For Inference (PVSBUFI)

Audited against the working tree on 2026-09-06. Source: docs/old/Parser_Scout.md:518-557

## Verdict
This is not an arcana: it's a Scala-era design Q&A the author poses and answers to himself ("So the question is, can those Es inside the `|` capture? Answer: NO, dear god no...") about whether rune capturing should be allowed inside a `|` (OneOf/possible-values) rule. It makes no assertion about present code behavior — it's a decision record for a rule the parser/solver would enforce. Checking it against the current Rust port: the `OneOf` rule construct it describes does still exist as a variant (`TestRule::OneOf` / `IRulexSR::OneOf`) but only survives in the solver test harness (src/solver/test/test_rules.rs) — every real-pass usage in src/typing/infer/compiler_solver.rs and src/typing/rune_typing/rune_type_solver.rs is commented out, so the `|`-possible-values syntax and its capture restriction are not live in the typing pass today. It has zero citations (`sites: []`) confirming nothing in the current tree relies on it as a defined term. Recommendation: leave it where it is (docs/old/); it's a stale design note for dead/inert syntax, not something to migrate into docs/arcana, and not worth deleting proactively since it costs nothing sitting in docs/old/.

## Claims
This section is a design question with a self-given answer, not a set of factual claims about code. The closest things to checkable claims:

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|
| 1 | Rules can specify "possible values" via `\|`, e.g. `#O = own \| borrow` | DRIFTED | src/solver/test/test_rules.rs:112 (`OneOf`); src/typing/infer/compiler_solver.rs:953-968, src/typing/rune_typing/rune_type_solver.rs:412-415 (all commented out) | The `OneOf`/`\|` rule shape still exists as a type in the solver test harness, but the real typing-pass solver code that would handle it is entirely commented out — the feature is not implemented/live in the current compiler. |
| 2 | "no capturing can go on inside an \|" (the resolved policy) | UNVERIFIABLE | n/a | No live code path evaluates this rule kind, so there's nothing to check the policy against. Not a claim about current behavior, just a note-to-self decision from the Scala era. |

## Stale citation sites
None — `sites: []`, no code cites PVSBUFI at all.

## Uncited sites that embody the arcana
None found. The only surviving trace of the `OneOf`/possible-values rule is in the solver test scaffolding (src/solver/test/test_rules.rs:112, src/solver/test/test_rule_solver.rs:142-145), which is a test-only rule vocabulary unrelated to the actual parser grammar or rune-typing pass; the real solver code that once handled `OneOf` (src/typing/infer/compiler_solver.rs, src/typing/rune_typing/rune_type_solver.rs) has it fully commented out, so there's no live site to cite this note from.

## Suggested text
N/A (D3 kind, not major-inaccuracies/obsolete in the sense of describing removed working code — it describes an inert/never-fully-implemented syntax decision, so no migration text is warranted).
