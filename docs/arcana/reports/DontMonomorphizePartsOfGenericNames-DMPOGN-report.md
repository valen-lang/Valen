# Accuracy report: Don't Monomorphize Parts Of Generic Names (DMPOGN)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:833 ("## Don't Monomorphize Parts Of Generic Names (DMPOGN)")

## Verdict
The core moral — the instantiator must not blindly substitute placeholders that are part of a lambda's "which generic did I come from" identity — still corresponds to a real mechanism in `src/instantiating/instantiator.rs`, but the mechanism has moved: instead of the instantiator skipping substitution for those placeholder slots, `assemble_placeholder_map_inner` (src/instantiating/instantiator.rs:946) now finds those slots *already filled in by the typing phase* with concrete kinds (not placeholders) by the time instantiation runs, and just ignores them rather than mapping them. `translate_kind` for `KindT::KindPlaceholder` (src/instantiating/instantiator.rs:2177-2183) now unconditionally substitutes and panics if a substitution is missing — it does not special-case "generic name" placeholders the way the doc's "leave that alone" prescription implies. The doc's concrete example strings (`mvtest/genFunc<int>.lam:2:6.__call{genFunc$0}<int>`, using a `__call{...}` template-arg syntax) also no longer match the current humanizer output, which renders lambda-call names as `λF:<code_location>` (src/instantiating/instantiated_humanizer.rs:112-117) with no `__call{...}` braces form. No code cites DMPOGN anywhere in src/ or docs/, so there are no stale citation sites, but the section's own worked example and stated fix are drifted relative to current code.

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | Lambda functions need to remember their original generic (because of GLIOGN) | TRUE | src/instantiating/instantiator.rs:2363-2372 (`LambdaCallFunctionTemplateNameT`/`...NameI` keep `code_location` + `param_types` distinct from `template_args`) | — |
| 2 | Instantiated names can contain an un-substituted placeholder like `__call{genFunc$0}` | DRIFTED | src/instantiating/instantiator.rs:2177-2183 shows `translate_kind` on `KindT::KindPlaceholder` always looks up and substitutes, panicking (`"translate_kind: missing placeholder substitution"`) if none is found — it does not leave placeholders in generic-name positions unsubstituted | Current code relies on the typing phase to have already resolved these slots to concrete kinds before instantiation (see claim 4), not on the instantiator skipping substitution |
| 3 | Naming syntax shown, e.g. `mvtest/genFunc<int>.lam:2:6.__call{genFunc$0}<int>` | DRIFTED | src/instantiating/instantiated_humanizer.rs:112,116-117 — lambda-call names humanize as `λF:` + code location, no `__call{...}` bracket form present in current humanizer | Treat the `__call{...}` strings as illustrative/historical Scala-era notation, not current compiler output |
| 4 | Fix: instantiator "shouldn't replace placeholders in generic names... leave that alone" | DRIFTED | src/instantiating/instantiator.rs:955-961 (comment): "placeholderedName *doesn't* contain a placeholder like one might normally expect... instead placeholderedName might already be filled in... because the typing phase already filled it in. ... Just ignore it, we don't need a mapping for it." | The actual current fix is upstream: the typing phase pre-populates the lambda's generic-name kind slots with concrete kinds before the instantiator sees them, so `assemble_placeholder_map_inner` simply skips (ignores) those already-populated slots rather than the instantiator refraining from substituting a placeholder it finds |

## Stale citation sites
None — no code or docs cite DMPOGN (grep -rn -w "DMPOGN" across src/, Backend/, docs/ returns only the arcana file itself).

## Uncited sites that embody the arcana
- src/instantiating/instantiator.rs:946-970 (`assemble_placeholder_map_inner`) — the comment there directly documents the modern version of this exact oddity/fix.
- src/instantiating/instantiator.rs:2363-2372 (`LambdaCallFunctionTemplateNameT`/`NameI` translation) — where a lambda's generic-name param types are threaded through.
- src/instantiating/instantiated_humanizer.rs:112-117 (`LambdaCallFunctionTemplate`/`LambdaCallFunction` humanization) — current name-printing behavior for lambda call functions.

## Suggested rewrite
(Not written — verdict is minor-inaccuracies, not major-inaccuracies/obsolete per instructions.)
