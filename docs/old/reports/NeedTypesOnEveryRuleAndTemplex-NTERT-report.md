# Accuracy report: Need Types on Every Rule and Templex (NTERT)

Audited against the working tree on 2026-09-06. Source: docs/old/Coercing Templatas, and Overload Sets.md:88-101

## Verdict
This section is a design musing from the old Scala-era planning notes, asking how the rule inferer (`InferEvaluator`) should statically know whether a bare identifier like `Int` in a rule resolves to a kind or a coord, and concluding this requires deferring resolution until "templar" (now the Rust typing pass) has visibility into the rest of the world (e.g. to look up `MyInterface`'s parameter types). It makes no assertion about a currently-existing mechanism, names no verifiable current API, and has zero citations anywhere in src/, Backend/, or docs/ — it's an open question being reasoned through, not a documented fact about the codebase. Recommendation: leave it in docs/old/ (delete only if the maintainer is pruning historical design notes wholesale); no migration to docs/arcana/ is warranted since there is no active concern to encode.

## Claims
Not applicable — the section poses a design question and reasons toward an implementation strategy for the old Scala compiler; it contains no checkable claims about the current Rust codebase.

## Stale citation sites
None — grep -rn -w "NTERT" across src/, Backend/, docs/ (excluding target/, tmp/, guardian-logs/, Guardian/, Luz/, docs/convos) found only the definition itself.

## Uncited sites that embody the arcana
None found. The general idea (rules need type context resolved with knowledge of the referenced struct/interface's parameter types, so this must happen after earlier passes have full program visibility) is presumably realized somewhere in src/typing/, but nothing cites NTERT, and the concept as worded (Kind vs Coord ambiguity for a bare identifier like `Int` in a rule) is too vague/historical to point to a specific current site without speculative reverse-engineering.

## Suggested text
Not applicable (kind D3, verdict not-an-arcana).
