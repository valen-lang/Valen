# Accuracy report: Need Optional Template Args in Name (NOTAN)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Namespaces.md:59-71

## Verdict
This is a Scala-era design note about the Templar's `FullName`/`NamePart` scheme, where `NamePart.templateArgs: Option[List[ITemplata]]` needed to be optional (rather than always-present) so that `Seq` (a template, `TemplateTemplataType`) could be distinguished by name from `Seq<>` (an applied kind, `KindTemplataType`). No such `FullName`/`NamePart`/`templateArgs: Option[...]` structure exists anywhere in the current Rust typing pass or Backend — the Scala class hierarchy this note describes was not ported, and the disambiguation problem it discusses (an empty-vs-absent template-arg list) has no live analogue found in the current name/mangling code. It has zero citations (confirmed empty `sites`) and zero uncited code that embodies it. Recommendation: delete — the described Scala mechanism no longer exists in the tree, so there is nothing here to migrate.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "A NameStep's templateArgs is an Option[List[ITemplata]]" — the name-part structure has an optional template-args field | OBSOLETE | no `FullName`, `NamePart`, or `NameStep` type exists in src/typing (grep returns nothing) | The Scala `NameStep`/`NamePart` types were not ported to the Rust typing pass; whatever name representation exists today (interned templata/name types in src/typing/typing_interner.rs) uses a different scheme entirely. |
| 2 | The optionality exists "because we want to disambiguate" `Seq` (a `TemplateTemplataType`) from `Seq<>` (a `KindTemplataType`) | UNVERIFIABLE / OBSOLETE | grep for `TemplateTemplataType`/`KindTemplataType` in src/typing found no matches | These Scala type names don't appear in the Rust code; cannot verify whether/how the modern typing pass disambiguates bare templates from empty-arg-list applications, or whether it still needs an Option-vs-empty-list distinction at all. |

## Stale citation sites
None — `sites` is empty; grep -rn -w "NOTAN" across src/, Backend/, docs/ (excluding docs/old) returns nothing.

## Uncited sites that embody the arcana
None found — no current code implements a `NamePart`/`FullName`-style optional-template-args-per-step naming scheme.

## Suggested text
N/A (D3 kind, verdict is obsolete, not major-inaccuracies — the section isn't "wrong," it just describes a data structure that has no descendant in the current codebase).
