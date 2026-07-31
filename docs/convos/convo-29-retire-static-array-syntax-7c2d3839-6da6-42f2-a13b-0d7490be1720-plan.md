# Plan document

Source: `/Users/verdagon/.claude/plans/1-yes-2-do-dreamy-lecun.md`
Session: 7c2d3839-6da6-42f2-a13b-0d7490be1720

---

# Retire `[#N]T` static-array TYPE syntax → `StaticArray<N, T>`

## Context

Vale spells three things with `[...]` brackets: destructure patterns (`[a, b]`), static-sized-array types (`[#N]T`), and runtime-sized-array types (`[]T`). In parameter/pattern position the parser can't tell a destructure from a bracket array-type, so the recently-added `destructure_first` path in `pattern_parser.rs` guesses (any bracket with no destination = destructure), and its justifying comment even misnames the array syntax as `[T, N]`.

The fix: move the **known-size array type** out of brackets entirely. `StaticArray<N, T>` is an ordinary generic — angle brackets, a size arg and an element arg — so it can never collide with a destructure. This shrinks the parser (delete a bracket path) rather than adding disambiguation logic. Outcome: `[#N]T` no longer parses; the known-size array type is `StaticArray<N, T>` (size first, element second).

## Locked decisions

- **Name: keep `StaticArray<N, T>`** (existing intrinsic name, capitalized). `StaticArray<N, T>` already parses today as a generic `Call` and already lowers to the *identical* postparse output that `[#N]T` produces (`Lookup(CodeName "StaticArray") + Call([size, element])`, size-first — see `templex_scout.rs:404-450`). So **no gated-typing edits**, and name resolution is preserved end-to-end.
- **Type only.** Retire only the `[#N]T` *type* syntax. Leave `[#N](...)` *construction* (expression position, in `expression_parser.rs` — a separate parser, does not cause the destructure ambiguity) untouched.
- **RSA deferred.** `[]T` runtime-sized-array type stays (architect is retiring RSA separately/soon). Consequence: the `[]` vs empty-destructure ambiguity is not fully gone until RSA also leaves brackets — noted, not solved here.

## Scope / ground truth

Every reference to the parser array variants is in **LIVE** code (per `lib.rs`: `parsing`/`postparsing` live; `typing`/`tests`/`solver`/`instantiating`/`simplifying` gated). Zero gated code references `ITemplexPT::StaticSizedArray`. So the whole change is testable in the live parse + postparse subtrees.

**In scope (delete `StaticSizedArray`, keep `RuntimeSizedArray`):**
- `src/parsing/templex_parser.rs` — `parse_array` (lines 30-80): retire the `#`-size branch (lines 54-63, 72-76); when brackets contain a leading `#`, return a new migration `ParseError`. Keep the no-`#` RSA path.
- `src/parsing/ast/templex.rs` — delete `ITemplexPT::StaticSizedArray` variant (line 23), its `range()` arm (line 45), and `StaticSizedArrayPT` struct (lines 166-170). Keep `RuntimeSizedArray`.
- `src/parsing/ast/mod.rs:45` — drop `StaticSizedArrayPT` from the re-export.
- `src/parsing/ast/rules.rs:195-201` — delete the `StaticSizedArray` rune-collection arm. Keep `RuntimeSizedArray` (202-207).
- `src/postparsing/rules/templex_scout.rs:404-450` — delete the SSA lowering arm. `StaticArray<N,T>` now flows through the existing generic `Call` arm (identical output). Keep the RSA arm (452+).
- `src/lexing/errors.rs` — add a `ParseError` variant (e.g. `RetiredStaticArrayTypeSyntax(i32)`) + its humanizer message ("`[#N]T` static-array type syntax is retired; use `StaticArray<N, T>`"). Wire the humanizer arm wherever ParseError messages are produced.

**Live tests to update (they `cast!`/match `ITemplexPT::StaticSizedArray`, so they won't compile after the variant is deleted — must change in the same slice):**
- `src/parsing/tests/traverse.rs:658` · `src/parsing/tests/struct_tests.rs:329,386` · `src/parsing/tests/patterns/type_tests.rs:54,160,182,201` · `src/parsing/tests/rules/kind_rule_tests.rs:220,227,234,241,248`
- Each: replace the `[#N]T` source + `StaticSizedArray` assertion with `StaticArray<N, T>` source asserting an ordinary `ITemplexPT::Call { template: NameOrRune("StaticArray"), args: [Int(N), <elem>] }`. Leave RSA assertions (`traverse.rs:666`, `struct_tests.rs:109`, `type_tests.rs:75`, `expression_tests.rs:1281`) alone.

**`.vale` fixtures — migrate the ~22 TYPE occurrences of `[#N]T` → `StaticArray<N, T>`** (leave `[#N](...)` construction and `[]T` RSA):
- Builtins: `src/builtins/resources/arrays.vale:7,38,41`, `src/builtins/resources/migrate.vale:10`.
- Tests: `src/tests/array/{each,has,iter}/*.vale`, `src/tests/programs/externs/ssamut{param,return}export/test.vale`, `src/tests/programs/pure/pure_func_{return,take}_ssa.vale` (nested → `StaticArray<2, StaticArray<2, int>>`), `src/tests/programs/roguelike.vale:31`.
- Pattern: `[#S]E` → `StaticArray<S, E>`; `&[#N]T` → `&StaticArray<N, T>`; nest inside-out.

**Out of scope (do not touch):** `[#N](...)` construction (`expression_parser.rs:2185,2202`, `ast/expressions.rs:321,328`, `expression_tests.rs:1153,1183,1226`); all RSA (`RuntimeSizedArray`, `[]T`, `func Array<E>`); the `Initializing*SizedArrayRequiresSizeAndCallable` errors and `*SizedArrayDeclarationName` names; keyword `keywords.static_array` (kept — gated typing still reads it for name recognition).

## Cleanup folded in

- `src/parsing/pattern_parser.rs:236` — fix the `destructure_first` comment: it misnames the array syntax as `[T, N]`. After this change the only bracket array-type left is RSA `[]T`; note the residual `[]`/empty-destructure ambiguity resolves when RSA retires.

## RFIGA slices

**Slice 1 — Retire `[#N]T`; `StaticArray<N,T>` is the known-size array type (atomic: parser + AST + scout + live tests).**
- R: add parser tests — `StaticArray<2, int>` parses as `Call{NameOrRune("StaticArray"), [Int(2), NameOrRune("int")]}`; nested `StaticArray<2, StaticArray<2, int>>`; borrowed `&StaticArray<5, int>`; and `[#2]int` at type position now yields the migration `ParseError`.
- F: the `[#2]int`-errors test fails (still parses as `StaticSizedArray`); the `StaticArray<...>` tests likely pass immediately (they already parse as generics) — that's fine, they lock the contract.
- I: delete the `StaticSizedArray` variant/struct/arm/re-export/rune-arm/scout-arm (list above); add the `ParseError` variant + `#`-branch guard in `parse_array`; update the live SSA-asserting tests.
- G: parse + postparse subtrees green.
- A: `cargo build --manifest-path FrontendRust/Cargo.toml --lib` + `cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing:: postparsing::`.

**Slice 2 — Migrate `.vale` fixtures/builtins `[#N]T` → `StaticArray<N, T>` (type occurrences only).**
- I: rewrite the ~22 type lines. Leave `[#N](...)` construction and `[]T`.
- Verify: `grep -rn '\[#' src/**/*.vale` shows only construction `[#](`/`[#N](` forms remain (no `[#...]` immediately followed by a type). If any live postparse test loads `builtins/resources/arrays.vale`, it stays green (check during impl).

**Slice 3 — Fix the stale `destructure_first` comment** in `pattern_parser.rs:236`.

## Verification

- `cargo build --manifest-path FrontendRust/Cargo.toml --lib` → clean (no warnings).
- `cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing:: postparsing:: --no-fail-fast` → green; new `StaticArray<N,T>` parse tests pass, `[#2]int`-errors test passes.
- `grep -rn '\[#[^]]*\][A-Za-z_&]' src` over `.vale` files → no static-array *type* occurrences remain (construction `[#](` allowed).
- Pipe all cargo output to `./tmp/vcoord-static-array.txt` (one fixed session file), inspect separately.

**Limits:** `typing`/`tests` are gated, so end-to-end array *typing/codegen* isn't runnable now. Because we keep the name `StaticArray`, postparse output is byte-identical to the old bracket lowering, so no typing change is needed and resolution is preserved when typing un-gates. Fixture migration under `tests/programs/` is correctness-only (those run only once typing/tests un-gate); it's verified structurally (they parse as the shapes covered by Slice 1 tests) rather than executed.

## Reusable facts

- `StaticArray<N,T>` needs no keyword to parse — it's a raw-identifier `NameOrRune` + angle-bracket `Call` (`templex_parser.rs:405-408,473-493`); int args are `ITemplexPT::Int(IntPT{value})` (`ast/templex.rs:84-88`).
- Postparse generic-`Call` lowering already exists in `translate_templex`; the deleted SSA arm was redundant with it modulo the (now-user-written) name.
