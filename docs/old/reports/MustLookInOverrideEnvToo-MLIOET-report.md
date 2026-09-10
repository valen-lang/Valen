# Accuracy report: Must Look In Override Env Too (MLIOET)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Virtuals.md:728-757

## Verdict
This is a Scala-era Templar design note about a specific bug in override resolution: when compiling an override function like `go` (declared with old syntax `this &MyFunc impl MyIFunction1<Int, Int>`), the compiler needs to look inside the `impl` clause's environment to find the parent abstract function `go` on `MyIFunction1`, rather than just the struct's own environment. The concern (env lookup for override resolution) is conceptually still real, but the mechanism it describes — walking an environment for a per-parameter `impl` annotation — no longer exists. Override resolution today runs through `look_for_override` in src/typing/edge_compiler.rs, which generates named "override dispatcher" cases and matches by interned name/param-type, not by env lookup through an `impl` parameter modifier. The example syntax (`impl` as a parameter-position keyword) is also gone from the language; today `impl X for Y` is a standalone top-level declaration (confirmed live and unchanged in src/typing/test/compiler_virtual_tests.rs and elsewhere), and override functions are matched to abstract functions structurally, not by syntactic env nesting. No code cites MLIOET (sites: none), so there's nothing to leave hooked up. Recommendation: delete (it's fully superseded — the specific bug it targets doesn't map onto the current edge_compiler.rs design, and there are no citing comments to clean up).

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | "When we call a method like fly(spaceship, 4, 5), we look in spaceship's env." | UNVERIFIABLE (describes old Scala Templar call-dispatch behavior; not something checkable in current Rust overload_resolver, which resolves by name/type signature matching, not per-receiver "env lookup") | src/typing/overload_resolver.rs | — |
| 2 | "We need to do that when we're looking for an override function's parent abstract function too" — otherwise resolution fails because the abstract fn lives inside the `impl` target's env | FALSE for current code | src/typing/edge_compiler.rs:275-360 (`look_for_override`) resolves overrides via interned dispatcher names and explicit parameter-type substitution, not by searching an "impl env" | Current mechanism: `look_for_override` builds an `override_imprecise_name`/`OverrideDispatcherCaseNameValT` and looks up the abstract function by structural type match, independent of any per-parameter env nesting the old note assumes |
| 3 | Example syntax: `fn go(this &MyFunc impl MyIFunction1<Int, Int>, param Int) Int` — `impl` used as a parameter-position modifier naming the implemented interface | FALSE (obsolete syntax) | src/typing/test/compiler_virtual_tests.rs:80,155,158,192,223 and after_regions_tests.rs:64,257,343 all use standalone `impl IShip for Raza;` declarations, never `impl` inside a parameter list | Overrides are declared as ordinary functions taking `virtual`/interface-typed params, matched to the interface via a separate top-level `impl X for Y;` statement, resolved later by edge_compiler.rs — not by a parameter-level `impl` annotation |
| 4 | Implicit claim (from the preceding paragraph, part of the same conceptual unit) — "we evaluated all the impls first... if a struct is templated we won't know its hierarchy until it's stamped" | UNVERIFIABLE / plausible design musing, not a checkable code claim as phrased | — | Not scored against current code; it's flagged in-doc as a to-be-resolved note ("lets mention this somewhere"), not an assertion about current behavior |

## Stale citation sites
None — sites is empty; no code or docs cite MLIOET.

## Uncited sites that embody the arcana
None found. The concern this note is chasing (resolve an override's parent abstract function correctly across environments/instantiations) is real in spirit, but its current embodiment in src/typing/edge_compiler.rs (`look_for_override`, `create_override_placeholder_mimicking`, override dispatcher case naming) uses a structurally different algorithm (interned dispatcher names + type substitution) that doesn't correspond to "look in the impl env" in any citable way — there's no single line that would honestly carry this old note's framing.

## Suggested text
Not applicable — kind D3, verdict obsolete, not major-inaccuracies (the note is simply superseded, not actively misleading anyone since nothing cites it). No suggested replacement text needed.
