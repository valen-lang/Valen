# Accuracy report: Some Rules are Hoisted Out of Default Param (SRHODP)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:1504 ("## Some Rules are Hoisted Out of Default Param (SRHODP)")

## Verdict
Minor inaccuracies. The section lives under the doc's "# Not sure if these are true" heading and describes the same rule kinds still present in the Rust code today (`CallSiteFuncSR`, `DefinitionFuncSR`, `ResolveSR` in src/postparsing/rules/rules.rs, generated together in src/postparsing/rules/templex_scout.rs, and consumed in src/typing/infer/compiler_solver.rs). The core "Resolve is the only optional/default one" idea is confirmed by a code comment ("The function (or struct) can either supply a default resolve rule ... or let the caller pass it in", src/typing/infer/compiler_solver.rs:547-549). But the section's framing that CallSiteFunc and DefinitionFunc are "always there" (i.e. not filtered/defaulted like Resolve) is imprecise: the actual code comments show CallSiteFunc is filtered out exactly like Resolve when solving the definition, and DefinitionFunc is filtered out when solving the call site — it's a two-way split (definition-side rules vs. call-site-side rules), not a three-way split where two rules are unconditional and one is optional. No citation sites exist (grep for SRHODP found none), so nothing is stale, but the doc's own description of the hoisting mechanism doesn't match the filtering logic actually implemented.

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | A bound function param like `func foo(int)void` generates three rules: DefinitionFunc, Resolve, CallSiteFunc | TRUE | src/postparsing/rules/templex_scout.rs:1073-1094 pushes exactly `CallSiteFunc`, `DefinitionFunc`, `Resolve` for each bound func param | — |
| 2 | DefinitionFunc creates a prototype used when compiling the definition | TRUE (drifted terminology) | src/typing/infer/compiler_solver.rs:680-687 solves DefinitionFunc from params_list_rune/return_rune conclusions | matches, though "creates a prototype" is closer to how DefinitionFunc's conclusion is used downstream |
| 3 | Resolve looks in the current environment for a matching function, and is a default only run when the caller doesn't specify their own prototype | TRUE | src/typing/infer/compiler_solver.rs:547-549: "The function (or struct) can either supply a default resolve rule ... or let the caller pass it in" | — |
| 4 | CallSiteFunc is used when compiling the call site to check the given prototype has the right params/return | TRUE | src/typing/infer/compiler_solver.rs:651-... solves CallSiteFunc from prototype_rune | — |
| 5 | DefinitionFunc and CallSiteFunc are NOT defaults — they "always be there", hoisted out of the generic param's default rules into the function's main rules, unlike Resolve which is skippable | DRIFTED | src/postparsing/rules/templex_scout.rs:1073-1074 comment: "Only appears in call site; filtered out when solving definition" (CallSiteFunc) — the exact same filtering description as given for Resolve (line ~1091: "Only appears in call site; filtered out when solving definition"). src/typing/infer/compiler_solver.rs:258-259 filters CallSiteFunc-containing rule sets and DefinitionFunc-containing rule sets symmetrically depending on which side is being solved. | The real split is definition-side (DefinitionFunc) vs. call-site-side (CallSiteFunc + Resolve) rules, gated by which pass is running — not "two always-present rules plus one optional default". Resolve is additionally special in that even at the call site it can itself be skipped if the caller supplies their own prototype, but CallSiteFunc is present at the call site exactly like Resolve is, not unconditionally like the doc implies. |

## Stale citation sites
None — grep -rn -w "SRHODP" across src/, Backend/, docs/ found zero citing sites.

## Uncited sites that embody the arcana
- src/postparsing/rules/templex_scout.rs:1073-1094 — generates the CallSiteFunc/DefinitionFunc/Resolve rule triple for a bound function param.
- src/postparsing/rules/rules.rs:40-42,75-101 — defines `CallSiteFuncSR`, `DefinitionFuncSR`, `ResolveSR`.
- src/typing/infer/compiler_solver.rs:181-200,546-580,651-690 — solves each of the three rule kinds and documents the default/filtering behavior.
- src/typing/infer/compiler_solver.rs:255-260 — filters rule sets by CallSiteFunc/DefinitionFunc presence depending on which side (definition vs. call site) is being solved.

## Suggested rewrite
Not written — verdict is minor-inaccuracies, not major/obsolete, per instructions.
