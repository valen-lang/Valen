# Accuracy report: Groups are declaration-side (GroupS), not templata-side (GROUPS)

Audited against the working tree on 2026-09-06. Source: reconstructed from citing code — src/typing/templata/templata.rs:83 (`See @GROUPS-are-declaration-side`), whose comment block (src/typing/templata/templata.rs:80-84) plus `/// VGB: arcana for this` is the only trace of the concept.

## Verdict
The core claim is accurate: the borrow checker derives a callee's/param's group from the declaration-side `ParameterS.tyype` / `FunctionS.maybe_return_type` (`ITypeST`, group-annotated via `GroupS`), and `ITemplataT::Group(GroupTemplataT)` carries no group data itself (`GroupTemplataT` is an empty struct). But the reconstructed summary overstates it: `ITemplataT::Group` **is** matched (as a bare variant tag, not for its payload) in `src/typing/borrow_checker/borrow_types.rs:294` and `:371`, so "the borrow checker ... never reads it" is false as stated — it reads the *variant*, just not the *payload*, and then falls through to the declaration-side written type to get the real group. The concept summary also cites a file, `call_check.rs`, that does not exist anywhere in the tree — the real logic lives in `src/typing/borrow_checker/borrow_types.rs` and `aliasing_info.rs`. Recommendation: migrate into docs/arcana with corrected wording (fix the "never reads it" overclaim and the wrong file name).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A generic function's "groups" are declared via GroupP/GroupS/GroupB syntax on a function signature | TRUE | src/parsing/ast/templex.rs:122-139 (GroupP), src/postparsing/rules/templex_scout.rs:817-855 (GroupP→GroupS lowering), src/postparsing/rules/types.rs:150 (GroupB mentioned as the borrow-checker-side resolution) | — |
| 2 | Groups are represented/read from the declaration side (`ParameterS.tyype`/`GroupS`), not from the instantiated `ITemplataT::Group(GroupTemplataT)` | TRUE | src/postparsing/ast.rs:388-390 (doc comment: "borrow checker reads a param's `in g` group off this"); src/borrow_checker/aliasing_info.rs:35-41 reads `ps.tyype` per param; src/postparsing/ast.rs:566-567 same for `maybe_return_type` | — |
| 3 | `ITemplataT::Group` is "a ceremonial placeholder added only so generic-param arity/index bookkeeping stays uniform across type/int/group params" | TRUE | src/typing/templata/templata.rs:170 `pub struct GroupTemplataT {}` (zero-field); comment at templata.rs:167 says "this is only the uniform param's value" | — |
| 4 | It "never flows into a `KindT`" | TRUE | grep of `KindT` variants shows no `Group` arm; borrow_types.rs derives kinds/groups from `written: ITypeST`, never from an `ITemplataT::Group` payload | — |
| 5 | "the borrow checker (`call_check.rs`) never reads it" | FALSE | `call_check.rs` does not exist in the repo (confirmed via `find`); the borrow checker in fact **matches** `ITemplataT::Group(_)` at src/typing/borrow_checker/borrow_types.rs:294 and :371 (as a variant tag, to switch derivation strategy) — it reads the variant but not its (empty) payload | Should say: "the borrow checker (`src/typing/borrow_checker/borrow_types.rs`) matches the `Group` variant only to redirect to the declaration-side written type — it never reads the templata's own (empty) payload." |
| 6 | The `/// VGB: arcana for this` marker means the arcana doc was never subsequently created | TRUE | `grep -rn -w "GROUPS"` across src/, Backend/, docs/ finds only this one site; no docs/arcana/*GROUPS* file exists | — |

## Stale citation sites
None — the single citation (src/typing/templata/templata.rs:83) still matches the surrounding code (the `Group(GroupTemplataT)` variant, its zero-field struct at templata.rs:170, and the `tyype()` match arm at templata.rs:108).

## Uncited sites that embody the arcana
- src/typing/borrow_checker/borrow_types.rs:288-303 — `make_templata_g`'s `ITemplataT::Group(_) => match written { ... }` arm, the actual code that discards the templata's own payload and reads the written/declaration-side type instead. This is the best citation target (currently uncited).
- src/typing/borrow_checker/borrow_types.rs:365-372 — the groupless-path sibling (`templata_groupless`), same pattern.
- src/postparsing/ast.rs:388-390 — `ParameterS.tyype` doc comment already describes the mechanism in different words; could carry the `@GROUPS...` tag too.
- src/postparsing/ast.rs:566-567 — `FunctionS.maybe_return_type` doc comment, same.
- src/typing/borrow_checker/aliasing_info.rs:35-41 — where `ps.tyype` (declaration-side) is actually read per parameter to build the group graph.

## Suggested text
**Groups live on the declaration, not the templata.** A generic function's borrow-checker "groups" (`in g` annotations from `GroupP`/`GroupS`/`GroupB`) are read off the declaration-side AST — `ParameterS.tyype` and `FunctionS.maybe_return_type`, both `ITypeST` trees — never off the instantiated `ITemplataT::Group(GroupTemplataT)` generic-argument value. `GroupTemplataT` is a zero-field struct: it exists only so a group generic parameter has *some* templata value, keeping arity/index bookkeeping uniform with type and integer params across the templata machinery. The borrow checker's `make_templata_g`/`templata_groupless` do pattern-match on the `Group` variant, but only as a tag telling them to switch to reading the parameter's written declaration-side type — they never read anything out of the variant's own payload, because there's nothing there to read. Keeping the real group data on the declaration side (rather than threading it through instantiation as templata) avoids having to keep a second, redundant group representation in sync with the one the parser and scout already built.
