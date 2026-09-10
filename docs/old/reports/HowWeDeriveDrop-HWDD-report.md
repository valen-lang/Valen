# Accuracy report: How We Derive Drop (HWDD)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templar/Templar.md:345 ("## How We Derive Drop (HWDD)")

## Verdict
The core mechanism (auto-derived `drop` for structs/interfaces, suppressible with `#!DeriveXDrop`, replaceable with a hand-written `drop` function) is still exactly how Vale works today, and the `#!DeriveStructDrop` example is essentially byte-for-byte alive in the current corpus (only `fn`→`func` has changed). But the doc also claims a third macro, `#DeriveImplDrop`, "added to every impl" — that macro does not exist anywhere in the codebase (no keyword, no lexer support, no usage); only `DeriveStructDrop` and `DeriveInterfaceDrop` are real. That's a factual claim about the code that is false, so the section can't be called accurate as written even though most of it holds up well. Recommendation: migrate to docs/arcana/ after dropping the `#DeriveImplDrop` claim and updating `fn`→`func`.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `#DeriveStructDrop` is added to every struct automatically | TRUE | src/lexing/lexer.rs:49,61; src/keywords.rs:243,404 | — |
| 2 | `#DeriveInterfaceDrop` is added to every interface automatically | TRUE | src/lexing/lexer.rs:97,109; src/keywords.rs:245,406 | — |
| 3 | `#DeriveImplDrop` is added to every impl | FALSE | no hits for `DeriveImplDrop` anywhere in src/ or docs/ (excl. docs/old) | Only two derive-drop macros exist today: `DeriveStructDrop` and `DeriveInterfaceDrop`. There is no per-impl drop macro. |
| 4 | Opt out via `#!DeriveStructDrop`, e.g. `struct Muta #!DeriveStructDrop { }` | TRUE (syntax current) | src/integration_tests/tests/ownership_tests.rs:196-199; src/lexing/lexer.rs:61 | — |
| 5 | Opting out lets you define a custom `drop` function, e.g. `fn drop(m ^Muta) void { println("Destroying!"); Muta() = m; }` | DRIFTED (keyword rename only) | src/integration_tests/tests/ownership_tests.rs:199 (`func drop(m ^Muta) void {`) | `fn` has been renamed to `func`; everything else (`^Muta` param, deconstruction-assignment body) is unchanged. |
| 6 | Future explicit derive syntax `#[derive(Drop)]` / `#DeriveDrop(MyStruct)` is speculative ("we'll have something like it") | UNVERIFIABLE / not-a-claim | — | This is an open design musing, not a claim about current code; still true that no such explicit-target derive syntax exists today (only the bare struct/interface macros). |
| 7 | Illustrative signature `fn drop(this MyStruct impl IWhatev impl IOther) extern("dropGenerator")` | UNVERIFIABLE / not-a-claim | — | Speculative sketch, not a real current API; multiple-`impl` clause and `extern("dropGenerator")` have no current equivalent found. |

## Stale citation sites
None — `grep -rn -w "HWDD"` outside docs/old finds no citations at all.

## Uncited sites that embody the arcana
- src/lexing/lexer.rs:49-109 — lexes `#DeriveStructDrop`/`#!DeriveStructDrop`/`#DeriveInterfaceDrop`/`#!DeriveInterfaceDrop`.
- src/keywords.rs:243,245,404,406 — interns the derive-drop keyword strings.
- src/typing/function/destructor_compiler.rs:79 — `pub fn drop(...)`, the typing-pass entry point that derives/compiles struct/interface drop functions.
- src/typing/rust_interop/declarations.rs:477 — comment explaining why `DeriveStructConstructor`/`DeriveStructDrop` are turned off for certain interop structs, i.e. documents the opt-out mechanism in practice.
- src/builtins/resources/opt.vale:5-11, result.vale:5-13, tup1.vale:7, tup2.vale:7, tupN.vale:7 — builtin structs using `#!DeriveStructDrop`/`#!DeriveInterfaceDrop` and hand-written `drop` functions, i.e. the exact pattern the doc describes.
- docs/handoffs/syntax-migration-handoff.md:63-84 — active handoff measuring/renaming `#!DeriveStructDrop`/`#!DeriveInterfaceDrop` usage counts, confirming these are the only two live derive-drop attributes (no impl variant).

## Suggested rewrite
Not written — verdict is minor-inaccuracies, not major/obsolete, per the report rules (rewrite only required for major-inaccuracies/obsolete). The single needed fix on migration is: drop the `#DeriveImplDrop` bullet and change `fn drop` to `func drop` in the example.
