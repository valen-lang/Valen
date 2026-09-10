# Accuracy report: Assemble the Case Environment For Resolving the Override (ACEFRO)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:1318 ("## Step 5: Assemble the Case Environment For Resolving the Override (ACEFRO)")

## Verdict
The section's core mechanism is intact and the citation site (src/typing/edge_compiler.rs:660) is accurate, but claim 1 misattributes which step produces the "dispatcher interface" — the doc says Step 2 gave it, while both the doc's own Step 1 (GTCII, docs/arcana/Generics.md:1026) and the code comments (src/typing/edge_compiler.rs:320 vs 390) show it's Step 1 (GTCII) that produces it; Step 2 (CDFGI) only consumes it. This is a genuine drift not already covered by the ZHERE TODO notes surrounding the section.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Step 2 gave us the dispatcher interface: `ISpaceship<dis$0, dis$1, dis$2>`" | FALSE | src/typing/edge_compiler.rs:320 (Step 1: GTCII, builds `dispatcher_placeholdered_interface`) vs :390 (Step 2: CDFGI, only consumes it); docs/arcana/Generics.md:1026-1046 (Step 1/GTCII text: "We'll refer to this as the 'dispatcher interface.'") | Should read "Step 1 gave us the dispatcher interface". |
| 2 | "Step 4 gave us the case struct, `Milano<dis$0, dis$1, dis$2, case$3>`." | TRUE | src/typing/edge_compiler.rs:648-652 (Step 4: FOSFC, computes `dispatcher_case_placeholdered_sub_citizen`); matches docs/arcana/Generics.md:1231-1235 (Step 4/FOSFC text) | — |
| 3 | "Now, let's put this information... into a new environment. This is the 'case environment'." | TRUE | src/typing/edge_compiler.rs:660-677 (builds `dispatcher_case_env` via `child_of(...)`) | — |
| 4 | "It's particularly necessary because it has any bounds that originally came from the abstract function or the case struct." | DRIFTED | src/typing/edge_compiler.rs:593-610 (`dispatcher_inner_env_with_bounds_for_sub_citizen`, built from `dispatcher_and_case_placeholdered_impl_reachable_prototypes`) is the parent env `dispatcher_case_env` is `child_of`; the doc's own ZHERE note above the section already flags that this is "the *with-independents* one" and should be called out explicitly | Matches the doc's own pending ZHERE note; not a new finding but confirms the note is warranted. |
| 5 | "We'll be using this to resolve an overload later." | TRUE | src/typing/edge_compiler.rs:682 (Step 6: UCEFO, uses `dispatcher_case_env` as `calling_env` to resolve the override) | — |

## Stale citation sites
None — src/typing/edge_compiler.rs:660 correctly reads "// Step 5: Assemble the Case Environment For Resolving the Override, see ACEFRO" and the code that follows matches the doc's description of building the case environment.

## Uncited sites that embody the arcana
None found — this is a single-purpose step inside one function (`edge_compiler.rs`); no other code builds an analogous "case environment".

## Suggested rewrite
Not written — verdict is minor-inaccuracies (one misattributed step number), not major/obsolete. Fix: change "Step 2 gave us the dispatcher interface" to "Step 1 gave us the dispatcher interface" in docs/arcana/Generics.md:1319.
