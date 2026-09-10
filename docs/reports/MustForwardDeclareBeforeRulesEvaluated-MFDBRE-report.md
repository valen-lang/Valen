# Accuracy report: Must Forward Declare Before Rules Evaluated (MFDBRE)

Audited against the working tree on 2026-09-06. Arcana section: docs/HigherTypingPass.md:76 ("Must Forward Declare Before Rules Evaluated (MFDBRE)")

## Verdict
The core mechanism the section describes — forward-declare a struct/interface's template name before evaluating its rules, so a recursive reference (e.g. `MyList<#T>` referring to `MyList` itself) can look itself up instead of infinitely recursing, and defer properties that depend on rule evaluation (sealedness, mutability) until after the runes are known — is alive and accurately reflected by `src/typing/compiler_outputs.rs`'s `declare_type` / `declare_type_sealed` / `lookup_sealed` sequence, and the cited site (compiler_outputs.rs:499) still panics exactly as the comment implies when sealed-ness hasn't been forward-declared yet. However the section's closing claim — "we'll need the scout to predict" the mutability of an incoming rune — refers to "the scout," an Astronomer/Scala-era component with no current equivalent in `src/`; that specific claim is stale terminology, not a live code path.

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | Recursive struct definitions (e.g. `MyList<Ref#T>` whose body refers to `MyList<#T>`) create a self-lookup problem while evaluating the struct's own rules. | TRUE | src/typing/compiler_outputs.rs:490-500 (`declare_type`/`declare_type_sealed`/`lookup_sealed`) shows the two-phase declare/finalize pattern this problem requires | — |
| 2 | "The classic forward-declaring solution" is used to solve it. | TRUE | src/typing/compiler_outputs.rs:330-341: `declare_type` records the template name into `type_declared_names` before `declare_type_sealed` fills in the sealed flag; `lookup_sealed` (line 497-501) panics ("Still figuring out sealed for struct") if code tries to read the value before it's been forward-declared-then-filled — this is the forward-declare/fill-later pattern the section describes | — |
| 3 | The inner environment can no longer be declared at forward-declare time because it depends on the runes, which aren't known yet. | UNVERIFIABLE (design-history claim about "used to be the case") | — narrative describing a prior design change, not a checkable current-state fact | — |
| 4 | Mutability can't be forward-declared either, because it's "determined by rules." | TRUE (by analogy to sealed) | Same declare/fill pattern exists for sealedness (compiler_outputs.rs:337-341); mutability of a struct is likewise a rule-computed property, consistent with the general claim, though there's no direct "mutability" lookup mirroring `lookup_sealed` in this file | — |
| 5 | To resolve nested cases, "we'll need the scout to predict" the mutability of an incoming rune `#T`. | DRIFTED/stale terminology | `grep -rn "scout"` under src/ matches only `scout_arena.rs`, `keywords.rs`, `pass_manager/*` — none is an Astronomer-era "type predictor" component; `grep -rn "Astronomer"` under src/ returns nothing | The mutability-prediction step this describes now lives in the higher-typing/rune-typing pass itself (this doc's own title), not a separate "scout" — the sentence should say "we'll need the typing pass's rune-type predictor" or similar, dropping "scout." |

## Stale citation sites
None. The single known site, src/typing/compiler_outputs.rs:499, still does exactly what the comment implies: `lookup_sealed` panics with "Still figuring out sealed for struct" when the sealed flag was never forward-declared/filled — a live illustration of the MFDBRE pattern.

## Uncited sites that embody the arcana
- src/typing/compiler_outputs.rs:330-334 (`declare_type`) — the forward-declare step itself.
- src/typing/compiler_outputs.rs:337-341 (`declare_type_sealed`) — the deferred fill-in step, asserting the type was already forward-declared.
- src/typing/compiler_outputs.rs:397-401 (struct definition insertion, asserting no prior entry) — same declare-once-then-fill discipline for struct definitions.

## Suggested rewrite
Not written — verdict is minor-inaccuracies (core idea and its one cited site remain accurate); only the "scout" sentence is stale, and per instructions a rewrite is only required for major-inaccuracies/obsolete verdicts.
