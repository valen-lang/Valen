# Accuracy report: Coercing Calls And Lookups (CCAL)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:1696 ("# Coercing Calls And Lookups (CCAL)")

## Verdict
The section describes a problem the compiler "struggles" with, framed around two rule kinds — `MaybeCoercingLookup` and `MaybeCoercingCall` — that would have to disambiguate whether a name like `Moo` resolves as a `kind<coord>` or a `coord<coord>`. Those two rule variants no longer exist anywhere in the current `IRulexSR` enum (src/postparsing/rules/rules.rs:33-46); the codebase now has plain `Lookup`/`Call` variants instead, and every reference to `MaybeCoercingLookup`/`MaybeCoercingCall` in the Rust tree is dead, commented-out code left over from the port. The typing-pass solver explicitly asserts these are "desugared before reaching the typing-pass solver" (src/typing/infer/compiler_solver.rs:1365), i.e. the ambiguity described is no longer handled the way the section says. The section is describing a since-removed mechanism, not current machinery — major-inaccuracies bordering on obsolete.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Given `struct Moo<T> { bork T; } struct Bork<T> { x Moo<T>; }`, the compiler struggles on the `x Moo<T>` part | UNVERIFIABLE | no matching test found | No test in src/typing/test reproduces this exact example; can't confirm current behavior either way. |
| 2 | There are two rules generated: `$1 = "Moo"` as a `MaybeCoercingLookup`, and `$2 = $1<T>` as a `MaybeCoercingCall` | FALSE | src/postparsing/rules/rules.rs:33-46 (enum has no such variants); src/typing/rune_typing/rune_type_solver.rs:243-253,323-329 (all `MaybeCoercingLookup`/`MaybeCoercingCall` handling is commented out) | Current rule kinds for this pattern are plain `Lookup` and `Call` (rules.rs:36-37). |
| 3 | It can't figure out `$1` because it doesn't know whether to load "Moo" as `kind<coord>` or `coord<coord>` | UNVERIFIABLE (design musing about disambiguation) | — | Plausible in spirit but no longer tied to a `MaybeCoercingLookup` code path since that path is gone. |
| 4 | It can't figure out the second rule because it doesn't know `$1`, and can't conclude it just from `$2` and args `T` | UNVERIFIABLE | — | Same caveat as above; the described solver mechanics (MaybeCoercingCall) don't exist in current code. |

## Stale citation sites
None found via `grep -w CCAL` — the section has zero code citations anywhere in src/, Backend/, or docs/, so nothing points back at it as stale. But note: src/typing/infer/compiler_solver.rs:1365 and src/typing/rune_typing/rune_type_solver.rs (throughout) reference the exact `MaybeCoercingLookup`/`MaybeCoercingCall` names this section is about, uncited, and show those variants are now dead/commented-out — the strongest signal the section is describing removed machinery.

## Uncited sites that embody the arcana
- src/postparsing/rules/rules.rs:33-46 — current `IRulexSR` enum; no `MaybeCoercing*` variants, just `Lookup`/`Call`.
- src/typing/infer/compiler_solver.rs:1365 — explicit comment that `MaybeCoercingLookup`/`MaybeCoercingCall`/`IndexList` are desugared before the typing-pass solver even sees them (this is the closest thing to documenting where/how the ambiguity in the section is actually resolved today, and it isn't via those two rule kinds).
- src/typing/rune_typing/rune_type_solver.rs:243,253,323-329,495,499,814 — all-commented-out `MaybeCoercingLookup`/`MaybeCoercingCall` handling, evidence of the Scala-era mechanism being retired.
- src/typing/overload_resolver.rs:374 — a comment mentioning `MaybeCoercingLookupSR` as still influencing rune-type seeding for explicit template args, the one live (non-dead-code) reference to the old name, itself now inaccurate since the type no longer exists.

## Suggested rewrite
Since the exact mechanism (two dedicated `MaybeCoercingLookup`/`MaybeCoercingCall` rule kinds) has been removed and replaced by plain `Lookup`/`Call` rules that are desugared before the typing-pass solver, the section needs a rewrite rather than a touch-up. A minimal correction, keeping the doc's voice:

> We have a case:
>
> ```
> struct Moo<T> { bork T; }
> struct Bork<T> { x Moo<T>; }
> ```
>
> and the compiler used to struggle on the `x Moo<T>` part.
>
> There used to be two dedicated rule kinds for this: `MaybeCoercingLookup` (for `$1 = "Moo"`) and `MaybeCoercingCall` (for `$2 = $1<T>`), meant to disambiguate whether `Moo` should load as a `kind<coord>` or a `coord<coord>` before the call's args were known. Those rule kinds have since been removed; the postparsing rule set now only has plain `Lookup` and `Call` (src/postparsing/rules/rules.rs), and any coercion this pattern needs is desugared before the rules reach the typing-pass solver (see compiler_solver.rs's `solve_rule`). This section should be re-investigated to describe how (or whether) the original ambiguity is still resolved under the new rule set, or retired if it no longer applies.
