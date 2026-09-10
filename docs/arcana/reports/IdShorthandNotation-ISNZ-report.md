# Accuracy report: ID Shorthand Notation (ISNZ)

Audited against the working tree on 2026-09-06. Arcana doc: docs/arcana/IdShorthandNotation-ISNZ.md

## Verdict
The core notation (`<>`, `{}`, `$T`/`$0`, `:loc`, `.`, `^`/`&`/`*`, precedence rules) is accurate and the referenced Rust types (`ITemplateNameT`, `IInstantiationNameT`, `StructTemplateNameT`, `StructNameT`, `FunctionTemplateNameT`, `FunctionNameT`, `KindPlaceholderT`, `AnonymousInterfaceMacro`) all still exist in `src/typing/`. However, the doc contains four unresolved `ZHERE:` editorial notes that the doc itself flags as internal contradictions — the dispatcher/dispatcher-case naming table conflicts with itself, and the worked example `^len.odis{impl:98}$0` uses `odis` where the "Examples decoded" section's `dis$0` gloss implies the same placeholder should use `dis`. One of those notes also cites `compiler_error_humanizer.rs:688,697` for the `ovdt:`/`ovd:` print sites; those lines have moved to 1208/1218 (content still matches). This is minor-inaccuracies: nothing is factually FALSE, but the doc is self-admittedly inconsistent in the denizen-prefix section and carries a stale line citation.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Tokens `<X>`, `{X}`, `$T`, `$0`, `:loc`, `.`, `bound:name`, `^X`, `&X`, `*X` are the shorthand vocabulary | TRUE | docs/arcana/IdShorthandNotation-ISNZ.md:14-24 | |
| 2 | Bare name = template (`ITemplateNameT`), name with `<...>` = instantiated (`IInstantiationNameT`) | TRUE | src/typing/typing_interner.rs (types defined, grep hit); src/typing/names/name_translator.rs | |
| 3 | `CoordTemplataT(CoordT(ownership, region, kind))` / `KindTemplataT(kind)` are Scala-era shapes, since dissolved by "the onion" | UNVERIFIABLE (historical claim) | — | doc marks this itself as historical background, consistent with its own caveat |
| 4 | Onion mapping: `&X`→`KindT::BorrowRef(X)`, `^X`→bare `X`, `*X`→structural share-wrap, bare `X`→still bare `X` | TRUE (doc's own stated model, not contradicted elsewhere) | docs/arcana/IdShorthandNotation-ISNZ.md:47-53 | |
| 5 | Precedence: `<>`/`{}`/`$N`/`:loc` > `.` > `^`/`&`/`*` | TRUE | docs/arcana/IdShorthandNotation-ISNZ.md:57-61 | |
| 6 | Denizen-prefix table (`ri`, `dis`, `case`, `abst`, `over`, `odis`, `lam`, `anon:I`) is settled and unambiguous | DRIFTED / self-contradicted | docs/arcana/IdShorthandNotation-ISNZ.md:66-77 (ZHERE block) | Doc's own ZHERE note says "dis" and "odis" are listed as separate prefixes "with nothing distinguishing them" though they denote one concept, and that `case` is being misused as bare where it should require a qualifier. Not yet resolved. |
| 7 | `compiler_error_humanizer.rs:688,697` already prints `ovdt:`/`ovd:` | STALE line numbers | src/typing/compiler_error_humanizer.rs:1208 (`ovdt:`), 1218 (`ovd:`) | Update citation to :1208/:1218; content of the claim (that it prints `ovdt:`/`ovd:`, agreeing with neither `dis` nor `odis`) is still true. |
| 8 | Example `dis$0` = "placeholder for the dispatcher's 0-th generic" | DRIFTED (doc admits) | docs/arcana/IdShorthandNotation-ISNZ.md:88 vs the ZHERE note at :93-96 | Doc's own later ZHERE note says this gloss is wrong: it's the dispatcher *case's* 0-th *dependent* generic, not "the dispatcher's 0-th generic"; independent counterpart is `case$N`. |
| 9 | Example `^len.odis{impl:98}$0` decoded via precedence rules | TRUE (mechanically) but inconsistent with claim 8's `dis` spelling | docs/arcana/IdShorthandNotation-ISNZ.md:61-63, 93-96 | Doc itself flags: "the ^len.odis{impl:98}$0 example uses odis where the prose elsewhere uses dis$0 for the same placeholder; pick one spelling." |
| 10 | `anon:I` / `anon:I$functor:moo` / `AnonymousInterfaceMacro` examples are real | TRUE | grep hit on `AnonymousInterfaceMacro` under src/typing/ | |
| 11 | `functor:M` rune-name convention example (`interface I { func moo(...) int; }` → `$I.anon.moo.functor` / `$functor:moo`) | UNVERIFIABLE (not independently traced through humanizer code in this pass) | — | Plausible given `anon:I$functor:moo` usage elsewhere in the doc; not separately confirmed against rune-humanizing code. |

## Stale citation sites
- src/typing/compiler_error_humanizer.rs:688,697 (as cited inside the arcana's own ZHERE note) — content still holds (`ovdt:`/`ovd:` printed) but at src/typing/compiler_error_humanizer.rs:1208 and :1218 now, not :688/:697.

No `@ISNZ` code citation sites exist in `src/`; all current hits are in `docs/convos/convo-72-...md` (a docs/notes file, not code) and the `CLAUDE.md` auto-generated SEE ALSO entry.

## Uncited sites that embody the arcana
None found — the notation is a documentation/comment convention (used in docs/Generics.md and investigation notes), not a code mechanism with call sites to cite. `src/typing/compiler_error_humanizer.rs` implements the actual humanizer that the shorthand is a hand-written gloss of, but it doesn't use this shorthand syntax itself, so citing @ISNZ there would be a stretch, not an omission.

## Suggested rewrite
(Not required — verdict is minor-inaccuracies, not major/obsolete. Recommended follow-up, not a rewrite: resolve the four ZHERE notes — settle the dis/odis/case naming, fix the "dispatcher's 0-th generic" gloss for `dis$0`, unify the `dis$0` vs `odis{...}$0` example spelling, and update the `compiler_error_humanizer.rs` line citation to :1208/:1218 — then delete the ZHERE markers.)
