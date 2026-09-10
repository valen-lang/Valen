# Accuracy report: WTF Is Going On With Impls (WIGOWI)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:973 ("# WTF Is Going On With Impls (WIGOWI)")

## Verdict

The core mechanism described (per-impl override resolution, minting one dispatcher per (abstract function, impl)) is still accurate in `src/typing/edge_compiler.rs::look_for_override`. However the section is explicitly marked by the author as needing revision: it contains five unresolved "ZHERE:" TODO notes stating that the terminology is wrong ("dispatcher" should be split into "dispatcher" = the abstract function vs. "dispatcher case" = the per-impl compiled function) and that the illustrative code snippet is "misleading, not just misnamed." Current code (`OverrideDispatcherTemplate`, `OverrideDispatcherCase` in `src/typing/compiler.rs:758-759`, `src/typing/templata_compiler.rs:268/273`, `src/typing/edge_compiler.rs:306-316`) confirms the ZHERE corrections are right and the prose above/below them (using bare "dispatcher" for the per-impl case, and the multi-arm `self match` snippet) is stale relative to the code. Since the doc itself flags this as a known, unfixed inaccuracy, this is minor-inaccuracies rather than accurate.

## Claims

| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | "For each impl, the typing phase will identify all the sub citizen's overrides for the interface." | TRUE | `src/typing/edge_compiler.rs:275` `look_for_override`, called from `src/typing/edge_compiler.rs:91` | — |
| 2 | The illustrative multi-arm `self match { case raza ... }` is how overrides are conceptually organized (one dispatcher function per abstract function, with per-impl arms) | DRIFTED | `src/typing/edge_compiler.rs:284-318`: `look_for_override` mints a distinct `OverrideDispatcherTemplate`/env keyed by `impl_t.template_id` per call, i.e. one per (abstract function, impl) pair — not one function with sibling match-arms per impl | Doc's own ZHERE note (Generics.md:988-996) already says this; snippet should be redrawn as a single arm compiled once per impl |
| 3 | Terminology: the per-impl compiled function is called "the dispatcher" | DRIFTED | Code distinguishes `OverrideDispatcherTemplate`/`INameT::OverrideDispatcherTemplate` (compiler.rs:758-759) from `INameT::OverrideDispatcherCase` (templata_compiler.rs:268,273) — the per-impl entity is the "dispatcher case", not "the dispatcher" | Per ZHERE note: reserve "dispatcher" for the abstract function, use "dispatcher case" for the per-impl compiled function |
| 4 | "the code mints one per (abstract function, impl) at edge_compiler.rs:284-287" (ZHERE note's own citation) | TRUE (approx) | `look_for_override` signature starts at `src/typing/edge_compiler.rs:275`, and the per-impl `dispatcher_template_name`/env construction is at `src/typing/edge_compiler.rs:306-318` | Line numbers in the note are a few lines off from current file but the claim holds |

No code anywhere cites `WIGOWI` (see below), so there are no external stale citation sites — the staleness is entirely internal (the doc's own unresolved ZHERE TODOs).

## Stale citation sites

None — `grep -rn -w "WIGOWI"` across src/, Backend/, docs/ (excluding target/, tmp/, guardian-logs/, Guardian/, Luz/, docs/convos/) finds only the header line itself (docs/arcana/Generics.md:973). No code site cites WIGOWI at all.

## Uncited sites that embody the arcana

- src/typing/edge_compiler.rs:275 — `look_for_override`, the function that does the per-impl override lookup described by this section.
- src/typing/edge_compiler.rs:306-318 — construction of the per-impl `OverrideDispatcherTemplate` env, the "dispatcher"/"dispatcher case" mechanism.
- src/typing/compiler.rs:758-759 — `INameT::OverrideDispatcherTemplate` naming.
- src/typing/templata_compiler.rs:268,273 — `INameT::OverrideDispatcherCase` naming.

## Suggested rewrite

Not written — the doc's own ZHERE notes already specify the exact fix (terminology split "dispatcher" vs "dispatcher case", redraw the snippet as a single arm compiled once per impl, heading rename to "In Terms of Dispatcher Case"). Applying those author-authored corrections resolves this report; no independent rewrite is needed.
