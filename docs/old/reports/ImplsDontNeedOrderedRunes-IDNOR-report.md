# Accuracy report: Impls Dont Need Ordered Runes (IDNOR)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Parser_Scout.md:1151 ("# Uncategorized Notes")

## Verdict
This is a syntax-design musing from the old colon-based generics notation (`impl:T MyStruct:T for MyInterface:T`), proposing that impls don't need "identifying" ordered runes the way functions/structs do, and suggesting `impl MyStruct:#T for MyInterface:#T` instead. The entire premise — colon-delimited generic parameter lists on `impl`, structs, and functions — has been superseded by Rust-style angle-bracket generics (`impl<T> XOpt<T> for XNone<T>`, confirmed live in src/typing/test/compiler_generics_tests.rs:40 and elsewhere). The `#T` sigil syntax does not exist anywhere in the current parser. The section's core claim (impls have no "identifying name" so their rune list needn't be ordered) is not expressible in or falsifiable against current code since the syntax it's arguing about no longer exists. Recommendation: leave in docs/old (obsolete, not a migration candidate).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Impls are written as `impl:T MyStruct:T for MyInterface:T` with colon-delimited rune lists | OBSOLETE | src/typing/test/compiler_generics_tests.rs:40 shows `impl<T> XOpt<T> for XNone<T>` | Syntax uses Rust-style `<...>` angle brackets, not `:T` |
| 2 | Functions and structs have "identifying names" that require ordered runes (e.g. `moo:Int`), but impls don't since they can't be called explicitly | UNVERIFIABLE (premise obsolete) | same as above — no `:T`-style identifying-name syntax found anywhere in src/parsing | Whole framing is moot; current generic syntax is positional `<T>` for functions, structs, and impls alike |
| 3 | Proposed alternative syntax `impl MyStruct:#T for MyInterface:#T` (the `#` sigil marking an unordered/anonymous rune) | OBSOLETE | grep for `#T` / `:#` sigil syntax in src/parsing returns nothing | Never adopted; current impls just use ordinary `<T>` generic params like everything else |

## Stale citation sites
docs/old/Parser_Scout.md:994 and :1114 — both are internal "see IDNOR" cross-references within the same obsolete doc, not live-code citations.

## Uncited sites that embody the arcana
None found — the `<T>` generic-impl syntax in src/typing/test/compiler_generics_tests.rs:40 and src/typing/test/compiler_mutate_tests.rs:152-203 supersedes rather than embodies this note's proposal.
