# Accuracy report: Need Match-Only Rule For Impls (NMORFI)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Impls.md:1-115 ("Need Match-Only Rule For Impls (NMORFI)")

## Verdict
The section is a self-contained historical design musing (Scala-era) reasoning through four candidate solutions (A-D) to a rule-solver stack-overflow problem when resolving impls, and settling on Solution D ("Ordering, Depending on Direction") as the near-term fix with a long-term intent to move to Solution C. It makes no concrete claims about specific current function/type names, so most of its content is UNVERIFIABLE-by-design rather than false. The underlying mechanism it describes — impls resolved by matching a struct-side and interface-side rune against many impls, needing direction-dependent handling — is still present in the Rust port (src/typing/citizen/impl_compiler.rs, resolve_impl/partial_resolve_impl using struct_kind_rune/interface_kind_rune), so the core idea is intact. However, the doc's only code citation (src/integration_tests/tests/integration_tests_c.rs:449) points into a test that is `#[ignore]`d with a `unimplemented!()` body and its real content entirely commented out — it no longer demonstrates or exercises anything about NMORFI. That's a minor staleness, not a contradiction of the arcana's core claims.

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | "We kept hitting stack overflows when we had a lot of impls" because evaluating a struct evaluates all its parent interfaces, which evaluates all impls, which evaluates all impl rules, which evaluates the struct part of each impl | UNVERIFIABLE | historical claim about the old Scala solver; no current equivalent recursion path found to check directly | none — treat as historical context |
| 2 | We need impl matching to not fully evaluate the struct/interface (MDESOI) | TRUE (as a still-relevant design goal) | src/typing/citizen/impl_compiler.rs:66-165 (`resolve_impl`) builds a separate rune-based solve per impl rather than eagerly evaluating the citizen | — |
| 3 | Solution D ("ordering depending on direction") was chosen as the near-term fix | UNVERIFIABLE / DRIFTED | Current code (impl_compiler.rs) filters rules via `include_rule_in_call_site_solve` / `include_rule_in_definition_solve` (src/typing/infer_compiler.rs:1238-1256), a different, rule-exclusion-based mechanism, not an ordering-of-two-rules-per-direction scheme as literally described | The current implementation's exact strategy differs in shape from "put struct rule first / put interface rule first"; doc should note the mechanism evolved, not just was implemented as literally written |
| 4 | Cited test site demonstrates/exercises the NMORFI bug fix | FALSE / STALE | src/integration_tests/tests/integration_tests_c.rs:441-454 — `#[ignore] fn test_narrowing_between_borrow_and_owning_overloads` body is `unimplemented!()` with the real test commented out | Site is dead; it documents intent in a comment only, doesn't verify anything currently |

## Stale citation sites
src/integration_tests/tests/integration_tests_c.rs:449 — comment says "See NMORFI for why this test is here," but the test is `#[ignore]`d and its body is `unimplemented!();` with the actual assertions inside a `/* ... */` block, so it currently proves nothing about NMORFI/SCCTT.

## Uncited sites that embody the arcana
- src/typing/citizen/impl_compiler.rs:66 (`resolve_impl`) — per-impl rune-based struct/interface matching, the direct descendant of the mechanism NMORFI discusses.
- src/typing/citizen/impl_compiler.rs:169 (`partial_resolve_impl`) — same, for partial resolution.
- src/typing/infer_compiler.rs:1238-1256 (`include_rule_in_call_site_solve`, `include_rule_in_definition_solve`) — rule-filtering mechanism that plays a role analogous to Solution D's "which rule to trust first" concern, cited instead to @SROACSD, not NMORFI.

## Suggested rewrite
(omitted — verdict is minor-inaccuracies, not major/obsolete)
