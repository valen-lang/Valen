# Accuracy report: PostParser Synthesizes Parser AST Nodes (PPSPASTNZ)

Audited against the working tree on 2026-09-06. Arcana doc: src/postparsing/docs/arcana/PostParserSynthesizesParserASTNodes-PPSPASTNZ.md

## Verdict

The core mechanism the doc describes (postparser fabricates `IExpressionPE` nodes — `LookupPE`/`DotPE`/`FunctionCallPE` — allocated in the `'p` arena, then feeds them through `scout_expression`) is real and accurately described for its one worked example (struct constructor synthesis in `expression_scout.rs`). But the doc's "Where" section names only `expression_scout.rs`, while `src/postparsing/loop_post_parser.rs` is actually the heavier user of the same pattern — it synthesizes `LetPE`, `PatternPP`, `ConsecutorPE`, and multiple `LookupPE`/`FunctionCallPE`/`DotPE` nodes across four `@PPSPASTNZ`-tagged sites to desugar `for` loops. The doc undersells the scope of the concern it documents. Also, the "Cross-cutting effect" section describes the two arena fields as `&'s Bump` / `&'p Bump`; the actual fields are typed wrapper structs (`&'ctx ScoutArena<'s>`, `&'ctx ParseArena<'p>`), not raw `Bump` references. These are drift, not contradiction — verdict is minor-inaccuracies.

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Postparser creates synthetic `IExpressionPE<'p>` nodes during expression scouting, not produced by the parser | TRUE | src/postparsing/expression_scout.rs:228-259 | |
| 2 | Fabricated to represent implicit operations like struct constructor calls at end of function bodies | TRUE | src/postparsing/expression_scout.rs:218-259 (constructing_member_names branch) | |
| 3 | "Where": src/postparsing/expression_scout.rs, in block-scouting logic handling constructing members | DRIFTED | src/postparsing/loop_post_parser.rs:50,207,290,331 also synthesize parser AST for loop desugaring, and are cited with the same tag; the doc names only one of the two real sites | Add loop_post_parser.rs's for-loop desugaring as a second "Where" location |
| 4 | Postparser builds `LookupPE`, `DotPE`, and `FunctionCallPE` nodes, then feeds them back through `scout_expression` | TRUE | src/postparsing/expression_scout.rs:229,239,244,254,266-269 | |
| 5 | `PostParser` holds `scout_arena: &'s Bump` (postparser output) and `parse_arena: &'p Bump` (synthetic parser nodes) | DRIFTED | src/postparsing/post_parser.rs:506-511 declares `pub scout_arena: &'ctx ScoutArena<'s>` and `pub parse_arena: &'ctx ParseArena<'p>` — typed wrapper arenas, not raw `Bump`, and both are `&'ctx`-borrowed, not owned by the lifetimes named | Update to `scout_arena: &'ctx ScoutArena<'s>` and `parse_arena: &'ctx ParseArena<'p>` |
| 6 | Postparser synthesizes a constructor call expression from the struct's member names, then scouts it like any other expression | TRUE | src/postparsing/expression_scout.rs:235-269 | |
| 7 | This reuses existing expression-scouting logic rather than duplicating it | TRUE | src/postparsing/expression_scout.rs:266-269 calls `self.scout_expression(...)` on the synthetic node | |
| 8 | Synthetic parser AST nodes are not stored in the final postparsed output; created, scouted (producing `IExpressionSE`), then abandoned | TRUE | src/postparsing/expression_scout.rs:260-269 — only `constructor_result` (an `IExpressionSE`) is threaded onward, the `'p`-typed `constructor_call_p` is not | |
| 9 | The `'p` arena keeps the synthetic nodes alive until it drops, but nothing in the postparsed AST references them | TRUE | Consistent with arena-per-pass design; `IExpressionSE` types in scout_arena.rs hold no `'p`-lifetime fields referencing parser nodes | |

## Stale citation sites

None of the code citation sites are stale — each of the 11 `@PPSPASTNZ` comments in expression_scout.rs, loop_post_parser.rs, and post_parser.rs sits next to code that does what the doc describes (synthesizing parser-typed AST nodes in `parse_arena`). The doc-only mentions in docs/todo/docs-todo.md:14, docs/todo/todo-mega.md:155, and docs/using_ai_guide.md:71 are historical/naming references, not claims about current code, so they are not evaluated as stale.

## Uncited sites that embody the arcana

- src/postparsing/loop_post_parser.rs:59-113 — `let_iterable_expr_p`/`let_iterator_expr_p` synthesis (`pa.alloc(IExpressionPE::Let...)`, `PatternPP`) sits inside the same function as the line-50 `@PPSPASTNZ` comment but the individual allocations 20+ lines downstream aren't separately tagged; acceptable under one-cite-per-block, listed for completeness.
- src/postparsing/loop_post_parser.rs:339-356 — a `LookupPE`/`FunctionCallPE`/`LetPE` synthesis block with no nearby `@PPSPASTNZ` comment (nearest tag is at line 331, ~8 lines above; the block itself runs to line 356 uncommented further).

## Suggested rewrite

Only the "Where" section needs correction (minor-inaccuracies, not a full rewrite):

> ## Where
>
> `src/postparsing/expression_scout.rs` — in the block-scouting logic that handles constructing members, and `src/postparsing/loop_post_parser.rs` — in `for`-loop desugaring. Both build synthetic parser nodes (`LookupPE`, `DotPE`, `FunctionCallPE`, `LetPE`, `PatternPP`, `ConsecutorPE`) and feed them back through `scout_expression`/`new_block` to produce the final `IExpressionSE`/`BlockSE` output.

And in "Cross-cutting effect", replace `&'s Bump` / `&'p Bump` with the actual field types `&'ctx ScoutArena<'s>` / `&'ctx ParseArena<'p>`.
