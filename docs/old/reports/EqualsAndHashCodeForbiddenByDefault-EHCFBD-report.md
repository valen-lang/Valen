# Accuracy report: Equals and Hash Code Forbidden By Default (EHCFBD)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Optimization.md:1 ("# Equals and Hash Code Forbidden By Default (EHCFBD)")

## Verdict
The section is a two-sentence Scala-era coding caution ("We often accidentally use them, and their default implementations are *very* expensive") about relying on Scala's auto-generated `equals`/`hashCode` (e.g. on case classes), which silently do deep structural comparison/hashing and are easy to invoke by accident (`==`, putting objects in a `HashSet`/`HashMap`, etc.). It makes no checkable claim about the current Rust code: Rust has no implicit default `Eq`/`Hash` impl that gets "accidentally used" — `PartialEq`/`Eq`/`Hash` must be explicitly derived or implemented, so the entire premise (accidental use of expensive default equality/hashing) does not carry over to the Rust codebase. The only citation site (docs/architecture/instantiator-design.md:663) is a bare glossary-style ID list with no discussion of equality/hashing, not a functional citation. This is a Scala-specific implementation caution with nothing left to migrate.

Recommendation: delete (or leave in docs/old with no live citation — but do not migrate to docs/arcana).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "We often accidentally use them [equals/hashCode]" | UNVERIFIABLE / not-an-arcana | docs/old/Compiler/Optimization.md:1-4 | This describes a hazard specific to Scala's implicit `equals`/`hashCode`; Rust requires explicit `#[derive(PartialEq, Eq, Hash)]` or manual impls, so there is no "accidental" default use to guard against in the current (Rust) codebase. |
| 2 | "their default implementations are *very* expensive" | UNVERIFIABLE / not-an-arcana | docs/old/Compiler/Optimization.md:3-4 | Same — refers to Scala case-class default deep equals/hashCode cost, not a property of any current Rust type. |

## Stale citation sites
docs/architecture/instantiator-design.md:663 — the line is a bare list of ~20 IDs ("...NNSPAFOC, EHCFBD, IMRFDI, UINIT..." — "various spot citations. Most defined in docs/; see grep.") with no surrounding text about equals/hashCode, performance, or comparison semantics. It doesn't corroborate or contradict the section's content; it's a glossary pointer, not a functional citation (same pattern independently noted in docs/arcana/reports/InterfacesMustRememberFunctionsDeclaredInside-IMRFDI-report.md:18 for the same list).

## Uncited sites that embody the arcana
None found. A search for `derive(...Hash` / `derive(...PartialEq` / `impl Hash for` / `impl PartialEq for` in src/ and Backend/ turns up ordinary, deliberate derives (structural equality on small typing-pass value types, interned ID types, etc.) — none of it reflects "forbid equals/hashCode by default" as a design rule; Rust's opt-in trait model already enforces the intent this note was guarding against in Scala, so there's no Rust-side rule or convention to point at.

## Suggested rewrite
N/A — no rewrite; the concern doesn't apply to the Rust codebase. Recommend deletion from docs/old rather than any doc making a claim about current code.
