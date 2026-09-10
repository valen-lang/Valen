# Accuracy report: Postparser Interning: Dual-Enum Pattern For Lookups (IDEPFL)

Audited against the working tree on 2026-09-06. Arcana section: src/postparsing/docs/architecture/interning-dual-enum.md:7 ("# Postparser Interning: Dual-Enum Pattern For Lookups (IDEPFL)")

## Verdict

The core dual-enum mechanism (permanent `&'s`-holding enum + transient by-value `Val` enum used as a HashMap lookup key, discarded on hit / arena-promoted on miss) is real and accurately described for `IRuneS`/`IRuneValS`, `IImpreciseNameS`/`IImpreciseNameValS`, and `INameS`/`INameValS`. But the "Five Dual-Enum Pairs" table's last two rows are false: `IFunctionDeclarationNameS` and `IVarDeclarationNameS` are explicitly **not interned** in the current code — `src/postparsing/names.rs` documents both as identity-bearing types that must never be interned (mirroring each other, citing arcana `@WVSBIZ`), and no `IFunctionDeclarationNameValS` or `IVarDeclarationNameValS` type exists anywhere in the codebase. The doc's claim that they participate in the dual-enum pattern "via `INameS`" is fabricated/stale — they're built directly and wrapped in `INameS::FunctionDeclaration`/`INameS::VarName` without any lookup-key Val or arena promotion step. This is a factual error in 2 of the table's 5 rows, so the doc misrepresents a chunk of its own central claim (minor-inaccuracies overall since the mechanism it describes for the other three pairs, and the general pattern explanation, are all still accurate).

## Claims

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Arena-backed interning on `ScoutArena<'s>` uses two parallel enums per type hierarchy: a reference enum ... and a value enum ..." (general mechanism) | TRUE | src/postparsing/names.rs:76 (`INameValS`), :221 (`IImpreciseNameValS`), :1148 (`IRuneValS`); src/scout_arena.rs:191,496,568 (`intern_imprecise_name`, `intern_name`, `intern_rune`) | — |
| 2 | `IRuneS<'s>` / `IRuneValS<'s,'tmp>` pair, interned via `intern_rune()` | TRUE | src/postparsing/names.rs:1148; src/scout_arena.rs:568 | — |
| 3 | `IImpreciseNameS<'s>` / `IImpreciseNameValS<'s>` pair, interned via `intern_imprecise_name()` | TRUE | src/postparsing/names.rs:221; src/scout_arena.rs:191 | — |
| 4 | `INameS<'s>` / `INameValS<'s>` pair, interned via `intern_name()` | TRUE | src/postparsing/names.rs:76; src/scout_arena.rs:496 | — |
| 5 | `IFunctionDeclarationNameS<'s>` / `IFunctionDeclarationNameValS<'s>` pair, interned "via `INameS`" | FALSE | src/postparsing/names.rs:304-307 (doc comment: "Identity-bearing ... so **not interned**, mirroring `IVarDeclarationNameS` (@WVSBIZ). Built directly and wrapped in `INameS::FunctionDeclaration`") | `IFunctionDeclarationNameValS` does not exist in the codebase (grep finds zero hits outside this doc). Row should be removed or rewritten to state this type is explicitly *not* interned. |
| 6 | `IVarDeclarationNameS<'s>` / `IVarDeclarationNameValS<'s>` pair, interned "via `INameS`" | FALSE | src/postparsing/names.rs:248-250 (doc comment: "Identity-bearing ... so never interned per @WVSBIZ; the per-variant disambiguator ... makes structural eq be identity") | `IVarDeclarationNameValS` does not exist anywhere in the codebase. Row should be removed or rewritten. |
| 7 | `ForwarderFunctionDeclarationNameValS` is a shallow Val struct holding `IFunctionDeclarationNameS<'s>` as its child | DRIFTED | src/postparsing/names.rs:318-321 (struct is real, field is `inner: IFunctionDeclarationNameS<'s>` plus an `index: i32`) but this struct is unused — grep finds it only declared and re-exported in src/scout_arena.rs:33, never constructed or passed to any `intern_*` call anywhere in src/postparsing/. | The struct exists but appears dead code, not an active example of the pattern; also note it's the Val for the nested `ForwarderFunctionDeclarationNameS` struct (referenced inside the `IFunctionDeclarationNameS::ForwarderFunctionDeclarationName` variant), not for `IFunctionDeclarationNameS` itself as claim 5's table row implies. |
| 8 | Other shallow Val examples: `ImplicitCoercionOwnershipRuneValS`, `AnonymousSubstructImplDeclarationNameValS`, `ImplImpreciseNameValS` | TRUE (spot-checked 2 of 3) | src/postparsing/names.rs:96 (`AnonymousSubstructImplDeclarationNameValS`), :195 (`ImplImpreciseNameValS`) | — |
| 9 | `'tmp`-deferred example: `ImplicitRuneS`/`ImplicitRuneValS`, `LocationInDenizen` slice-deferred allocation | TRUE | src/postparsing/names.rs:1335 (`ImplicitRuneS`), :1067 (`ImplicitRuneValS`); src/postparsing/ast.rs:718 (`LocationInDenizen`), :726 (`LocationInDenizenVal`) | — |

## Stale citation sites

- src/typing/names/names.rs:3236 — comment cites `.claude/rules/postparser/IDEPFL-postparser-interning.md` as the pattern's home doc; that path no longer exists (the doc now lives at src/postparsing/docs/architecture/interning-dual-enum.md). Minor drift, not incorrect about the pattern itself.
- docs/todo/frontendrust-docs-todo.md:36 and docs/todo/todo-mega.md:196 similarly reference the old `.claude/rules/postparser/IDEPFL-postparser-interning.md` path as a to-do relocation target that has, in fact, already happened (the doc already lives under `src/postparsing/docs/architecture/`), so these todo entries are themselves stale/completed but that's a todo-doc bookkeeping issue, not an IDEPFL content error.
- docs/architecture/typing-pass-design-v3.md:435,438 and docs/architecture/simplifier-design.md:337 apply "IDEPFL" to the *typing pass* interner (`IdT`/`IdValT`), a different, later type hierarchy than the postparser one this arcana doc defines. Cross-checked src/typing/names/names.rs:3529 — the typing-pass code does follow the same transient/permanent split, so these citations are consistent extensions of the pattern, not stale.

## Uncited sites that embody the arcana

- src/postparsing/rules/rule_scout.rs — uses `IRuneValS`/`intern_rune` per earlier grep; embodies the pattern without citing IDEPFL.
- src/postparsing/rules/templex_scout.rs — same.
- src/postparsing/function_scout.rs — uses `IImpreciseNameValS`, `INameValS`.
- src/postparsing/expression_scout.rs — same.
- src/postparsing/patterns/pattern_scout.rs — uses `IVarDeclarationNameS` construction directly (consistent with claim 6's "not interned" finding).

## Suggested rewrite

Replace the "Five Dual-Enum Pairs" section (src/postparsing/docs/architecture/interning-dual-enum.md, the table under "## The Five Dual-Enum Pairs") with a three-pair table, and add a short note explaining the two identity-bearing exceptions:

> ## The Three Dual-Enum Pairs
>
> | Permanent Enum (canonical) | Transient Enum (lookup key) | ScoutArena Method |
> |---|---|---|
> | `IRuneS<'s>` | `IRuneValS<'s, 'tmp>` | `intern_rune()` |
> | `IImpreciseNameS<'s>` | `IImpreciseNameValS<'s>` | `intern_imprecise_name()` |
> | `INameS<'s>` | `INameValS<'s>` | `intern_name()` |
>
> Note: `IFunctionDeclarationNameS<'s>` and `IVarDeclarationNameS<'s>` are *not* part of this pattern despite living in the same `INameS` family. Both are identity-bearing (each instance names one specific declaration site, carrying a `lid`/location), so they are built directly and wrapped in `INameS::FunctionDeclaration` / `INameS::VarName` without ever being interned — see `@WVSBIZ` and the doc comments on `IFunctionDeclarationNameS` and `IVarDeclarationNameS` in `src/postparsing/names.rs`. No `IFunctionDeclarationNameValS` or `IVarDeclarationNameValS` type exists.
