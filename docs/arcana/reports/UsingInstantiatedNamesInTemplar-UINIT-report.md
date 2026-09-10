# Accuracy report: Using Instantiated Names in Templar (UINIT)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:170 ("# Using Instantiated Names in Templar (UINIT)")

## Verdict
Core idea is intact and still cited from live code (`src/instantiating/ast/ast.rs:228`), but the section's one concrete anchor — the field name "FunctionHeaderT.fullName" — is wrong for the current code: the field is called `id` (of type `IdT`/`IdI`), not `fullName`, in both the typing pass's `FunctionHeaderT` and the instantiator's `FunctionHeaderI`. The rest of the mechanism (ordinary vs. generic functions, placeholder template args, per-call-site lambda instantiation) still holds under Rust names (`IFunctionNameT`, `KindPlaceholderT`/`PlaceholderTemplataT`, `LambdaCallFunctionNameT`).

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | "This one little name field (FunctionHeaderT.fullName)..." | DRIFTED | src/typing/ast/ast.rs:324-329 defines `FunctionHeaderT` with field `id: IdT<'s,'t>`, no field named `fullName` | Field is now `id`, not `fullName` |
| 2 | Ordinary FunctionA → ordinary FunctionT/FunctionHeaderT; IFunctionNameT params as expected, no templateArgs | TRUE | src/typing/names/names.rs:491-539 `IFunctionNameT::Function` variant carries `template_args`, empty for non-generic instantiations in practice | — |
| 3 | Generic FunctionA → FunctionHeaderT whose params/templateArgs are Placeholder(Templata)s | TRUE | src/typing/types/types.rs:402 `KindPlaceholderT`, src/typing/templata/templata.rs:154 `PlaceholderTemplataT` still used for generic substitution | — |
| 4 | Lambdas manifest multiple FunctionTs/FunctionHeaderTs, one per call-site instantiation, each with distinct template arg | TRUE | src/typing/names/names.rs:200 `INameT::LambdaCallFunction`, structural support for per-call lambda naming still present | — |
| 5 | "We also use this same scheme for the CompilerOutputs, to map names to environments." | UNVERIFIABLE | No `CompilerOutputs` type found by name in src/typing; likely renamed but not confirmed in this pass | — |

## Stale citation sites
None — the single code citation (src/instantiating/ast/ast.rs:228, on `FunctionHeaderI.id`) correctly points at the field this arcana describes, it's just that the field is `id` not `fullName` as the prose says, in both the code comment's target and the arcana text itself.

## Uncited sites that embody the arcana
- src/typing/ast/ast.rs:324 — `FunctionHeaderT.id` field, the typing-pass twin of the cited instantiator-pass field, has no UINIT citation.

## Suggested rewrite
Not written — verdict is minor-inaccuracies (a single stale field name), core mechanism and example remain accurate; only claim 1's literal name needs fixing: replace "FunctionHeaderT.fullName" with "FunctionHeaderT.id (and its instantiator-pass twin FunctionHeaderI.id)" wherever it appears in the section.
