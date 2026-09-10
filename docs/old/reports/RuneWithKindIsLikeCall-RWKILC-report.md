# Accuracy report: Rune With Kind Is Like Call (RWKILC)

Audited against the working tree on 2026-09-06. Source: docs/old/Parser_Scout.md:704-718

## Verdict
This is a self-resolved historical note: the entire body describing the K:Kind ambiguity is struck through (`~~...~~`), and the section ends with the author's own resolution — "We switched to using < and > for template calls, so there's no more ambiguity here." Current code confirms template calls do use `<...>` syntax (e.g. src/parsing/templex_parser.rs:415 "Parse template call arguments <...>"), so the note's own conclusion still holds, but the note documents a design ambiguity from the old `K:Kind`-colon-call syntax that no longer exists at all — there is no live concept for this ID to migrate onto. No code or doc cites RWKILC anywhere in src/, Backend/, or docs/. Verdict: obsolete. Recommendation: leave it in docs/old/Parser_Scout.md as historical record (it's a dead design note, already self-marked resolved) — do not migrate to docs/arcana/, since there is no ongoing cross-cutting concern here, only a settled historical footnote.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "K:Kind might be two things: a kind rune, or a template call of template K with arg Kind" (struck through) | OBSOLETE | src/parsing/templex_parser.rs:415 | This described the old `K:Kind` colon-call syntax, which no longer exists; superseded per claim 3 below. |
| 2 | "So when there's a rune, we can't say K:Marine; must say K:(Marine)" (struck through) | OBSOLETE | n/a — colon-call syntax gone | Not applicable to current parser, which uses `<...>` for template calls. |
| 3 | "We switched to using < and > for template calls, so there's no more ambiguity here." | TRUE | src/parsing/templex_parser.rs:415, src/parsing/parse_error_humanizer.rs:27,115,118 | Confirmed current — template call syntax is angle-bracket based. |

## Stale citation sites
None — no code or doc cites RWKILC.

## Uncited sites that embody the arcana
None found — the ambiguity described is fully retired; there is no current mechanism to attribute a citation to.

## Suggested text
Not applicable (D3 kind, verdict obsolete, but this is a self-resolved historical note rather than a major inaccuracy — no corrected arcana text is warranted since the concern no longer exists in the current design).
