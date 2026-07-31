# Plan document

Source: `/Users/verdagon/.claude/plans/compressed-whistling-sketch.md`
Session: d71593d2-e995-4cc8-98d6-0a97e3eea32e

---

# Valen syntax alignment — parser/postparse slice (`^` postfix, optional colon, `ownref`)

> ## ⚠️ PARTIALLY STALE as of 2026-07-25 — read `Vale2/vcoord-handoff.md` first
>
> **This plan's numbers were estimates; they have since been measured, and one of its
> assumptions is dangerous.** The handoff's "In flight — the syntax migration" block
> supersedes this document wherever they disagree.
>
> - **`#!DeriveX` → `#derive(X)` IS NOT A SPELLING CHANGE. Do not run it.** The `#!` bang is
>   **semantic**: `#DeriveStructDrop` = *call the macro*, `#!DeriveStructDrop` = **suppress** it,
>   dispatched at `typing/compiler.rs:1395`/`:1403`. All **78** sites in the tree are `#!`, so a
>   naive rewrite **inverts every one of them.** The real target is `#explicitly_destroyed`
>   (ruled upstream 2026-07-25) — but that spelling is itself provisional, so **this item is
>   PARKED.** Run `^` and `own` only.
> - **Measured caret counts** (this plan's are estimates): 511 repo-wide, **271 Vale source**,
>   split **252 mechanical + 4 restructures + 12 type-position repairs + 3 dead-legacy**. The
>   dangerous bucket is **218 humanizer caret-arrows**, 213 of them in just two files
>   (`after_regions_error_tests.rs`, `compiler_solver_tests.rs`).
> - **FOUR restructure sites, not two.** The two new ones: `hashmap.vale:251`
>   (`^(^maybeNeighbor).get()`) and `list.vale:64` (`^` applied to a `set` expression).
> - **A bucket this plan doesn't have:** 12 **type-position** carets (`func drop(self ^Moo)`,
>   `wand ^Wand;`) that already fail to parse today. They want `ownref`, not a postfix `^`.
> - **Blind-surface answer:** `integration_tests/` is commented out of `lib.rs:16-17` and
>   compiles **zero** tests, so its 68 carets are unverified. `parse_sample_test!` covers 146 of
>   195 `.vale` files; the blind `.vale` surface is only 12 carets + 2 attributes.
>   **`builtins/resources/*.vale` is in no parse corpus at all** (30 carets, 10 attributes).
> - **`own` is nearly free** — one consumer, two fixture sites, **zero `.vale` usage**.
> - **The `&`-audit is answered:** exactly three override-shape mismatches exist, all one
>   pattern, and four sibling files show it's drift. See the handoff.

## Context

A peer session finalized Valen's surface syntax on 2026-07-23 and ratified it by rewriting
their design docs (`/Volumes/V/LangNotesValen/Valen/valen-approach-convo-30-finalize-syntax.md`;
the rulebook is §3 of `…-convo-30-plan.md`). Vale2 is out of line on several points. This
slice brings over the three that are cheap and settled, and defers the rest.

**In scope:** `^` moves from prefix to postfix; `name: type` gains an optional colon;
`own` renames to `ownref`. Plus the fixture sweeps those force, and an audit of whether
borrow params carry `&`.

**Explicitly out of scope** (recorded in `vcoord-handoff.md` under "Valen alignment — LATER
TODOs"): the `ownref` *narrowing* to immovable types, the movability axis itself,
`interface`/`open trait`, and the position-dependent bare-class-param rule.

**No typing-pass implementation changes.** The architect does those manually. This slice
touches `typing/` only to fix *test fixtures*, and even there one item is flagged for the
architect rather than changed.

## Decisions already made

- **Colon goes in all three `name type` positions**, including generic params (`<T: Kind>`).
- **The two stale `^T`-in-type-position fixtures become bare** — `func drop(self Moo)`.
- **The `&` audit leaves every `extern`/`exported` signature alone** (see below).

## Confirmed against `valen-design-1.md` (full read, 2026-07-24)

Three of this plan's assumptions were checked against the authority and hold:

- **The colon deviation is exactly Valen's grammar.** design-1:2350 — *"the grammar also admits the colonless `name type` form for experimentation, which documented Valen never uses."* Same permissive grammar; they simply never write it. We're the ones who will.
- **The call-site `name: value` colon is mandatory** (2263) — it's the positional-vs-named disambiguator, a *different* colon from the declaration one. My flagged check is confirmed: don't let the type-colon work collide with it.
- **`ownref` matches what slice 4 assumes** (90, 1180-1197, 2329): immovable-only, parameter position, `Box<ownref T>` ill-formed like `Box<&T>`, and a reified parameter slot (`Fn3<ownref Ship, i64, bool>`) is the one legal nesting.

### ⚠️ One assumption FAILED — `^` is local-names-only

design-1:93 — ***"`^` (postfix). Move operator on local names (not on paths)."*** Restated at 2308: *"`x^` — postfix, local names only."*

This plan (and my earlier framing of the postfix flip as a free expressiveness win) assumed `^` becomes a general suffix operator, so that `a^.b` and `a.b^` both become expressible. **Under Valen's rule neither is** — `a.b^` is a path, and `^` on a call result isn't a local name either.

That collides with the two sites this plan calls "tricky":

- `parsing/tests/expression_tests.rs:265` — `"^Muta()"`, a move of a **call result**
- `src/tests/hashmap/hashmap.vale:242` — `^innerRemove(...)`, likewise

**RESOLVED 2026-07-24: adopt the restriction.** The architect clarified that `valen-design-1.md`
**is** Vale2's language design, with the colon as the *only* intended divergence — so this is not
a fork. `^` parses only as a suffix on a bare local name, and the two sites above are illegal
constructs to be rewritten (bind a local, then move it).

Good news for the implementation: **this makes the parser simpler**, not harder. No spree step is
needed, and no adjacency guard against a hypothetical binary `^` — `^` can never follow an
arbitrary expression, so the ambiguity that motivated the guard cannot arise. Note it also makes
`(^base).drop()` → `base^.drop()` illegal; that site needs a bound local too.

The fixture-sweep count (~264 sites) is essentially unaffected — the overwhelming majority are
`^<local>`, which the restricted form accepts unchanged.

## The `&` audit — result

The suspicious bare params clustered under `src/tests/programs/externs/`, and inspection
says **don't touch them**:

- The types are `share` (`exported struct Firefly share`, `sealed exported interface IShip share`).
  Per our validity table a share citizen is never bare — it's already `ShareRef(...)`, an Rc
  handle. Adding `&` would turn it into `BorrowRef(Struct)`, a real semantic change.
- The always-OWN extern ABI is deliberate and documented
  (`Backend/docs/arcana/FFIRefsMoveAccessorsConsume-FRMACZ.md`): bare at a boundary signature
  means a transferred strong ref.

The only genuine finding is an **override-shape mismatch**, handled in slice 6.

## Files touched

All paths relative to `FrontendRust/src/`.

### Implementation — parser (5 files)

| File | Slice | Change |
|---|---|---|
| `parsing/pattern_parser.rs` | 1 | optional `:` in `parse_pattern`, between the `in` stop (~:230) and `next_is_type` (~:233) |
| `parsing/parser.rs` | 2, 3 | optional `:` in `parse_struct_member` (~:207) and `parse_generic_parameter` (~:57) |
| `parsing/expression_parser.rs` | 5 | drop `Move` from the `Prefix` enum (~:1907-1941); add an adjacency-guarded `^` arm to `parse_spree_step` (~:1366) |
| `keywords.rs` | 4 | `"own"` → `"ownref"` at **both** interning sites — `:160` (parse arena) and `:317` (scout arena) — plus the field name |
| `parsing/templex_parser.rs` | 4 | renamed keyword field at `:270` |

### Implementation — postparse (1 file)

| File | Slice | Change |
|---|---|---|
| `postparsing/post_parser_error_humanizer.rs` | 4 | `"own "` → `"ownref "` (`:235`) |

**That one line is the entire postparse footprint.** The colon needs no postparse change (it's
pure syntax — the AST already carries name and templex as separate fields), and the `^` flip
needs none either (`postparsing/expression_scout.rs:396` consumes `IExpressionPE::Move`
structurally and doesn't care where the sigil sat). The traverse arms
(`parsing/tests/traverse.rs:634`, `postparsing/test/traverse.rs:770`) match on the node names
`OwnRefPT` / `OwnRefSR`, which are **not** renamed — no change.

### Tests — new or edited (7 files)

| File | Slice | Change |
|---|---|---|
| `parsing/tests/patterns/capture_and_type_tests.rs` | 1 | new colon twins of `capture_with_type`, `no_capture_with_type`, `capture_with_borrow_tame` |
| `parsing/tests/patterns/destructure_parser_tests.rs` | 1 | one destructure colon case |
| `parsing/tests/struct_tests.rs` | 2 | `struct Moo { a: int; }` beside the colonless form |
| `parsing/tests/rules/rule_tests.rs` | 3 | `<T: Kind>`, `<T Kind>`, `<T>` |
| `parsing/tests/patterns/type_tests.rs` | 4 | `own_prefix_type` (`:108-116`) → `"_ ownref T"` |
| `postparsing/test/post_parser_tests.rs` | 4 | `x own int` → `x ownref int` (`:1449`, `:1461`, `:1477`) |
| `parsing/tests/expression_tests.rs` | 5 | `move_call_via_caret` → `"Muta()^"`; new spree (`a^.b`) and spaced-caret negative tests |

### Fixture sweep — slice 5 (~58 files, ~264 caret sites)

| Area | Files | Carets | Notes |
|---|---|---|---|
| `tests/**/*.vale` | 24 | 83 | includes the tricky `tests/hashmap/hashmap.vale:242` |
| `builtins/resources/*.vale` | 9 | 30 | |
| `typing/test/*.rs` | 8 | ~73 | **see hazard below** |
| `integration_tests/**/*.rs` | 15 | 73 | gated out of `lib.rs`; sweep anyway for consistency |
| `parsing/tests/*.rs` | 2 | 5 | includes the tricky `expression_tests.rs:265` |

**Sweep hazard — `typing/test/` holds 303 `^` characters but only ~73 are Vale source.** The
remainder are humanizer error-arrow assertions that a blind pass would corrupt:
`compiler_solver_tests.rs:512-514` (`"\n   ^ A: own"`, `"^^^^^^^^^^^^^^^^^^^ _7:"`) and
`compiler_tests.rs:2023`. Scope the sweep to Vale-source contexts (`= ^`, `(^`, `return ^`,
`, ^`, `[^`) and leave assertion strings alone.

### Fixture fixes — slice 6 (2 files)

| File | Change |
|---|---|
| `typing/test/compiler_tests.rs` | `:450` `func drop(self ^Moo)` → `self Moo`; `:793` `func destructor(m ^Muta)` → `m Muta` |
| `tests/programs/externs/interfaceimmparamextern_owned/test.vale` | `:8` `virtual ship &IShip` → `virtual ship IShip`, matching the override and the `_vale_dispatch` sibling |

### Deliberately NOT touched

- `typing/compiler_error_humanizer.rs` (`:146`, `:153`) — the `` `^local` `` message text. In
  `typing/`, so it's the architect's.
- `lexing/lexer.rs` (`:1031-1041`) — `lex_impl_ownership_prefix` still lexing `^`. Flagged, not changed.
- `parsing/tests/patterns/type_tests.rs` (`:83-89`) — the test asserting `^T` is a parse error
  at templex level. Still true; **keep it**.

## RFIGA

Baseline first: parser and postparse suites must be green before slice 1. See Verification.

1. **Optional colon in patterns** (params, lets, destructure elements).
   * R: in `parsing/tests/patterns/capture_and_type_tests.rs`, add colon twins of the
     existing colonless tests — `"a: int"`, `"_: int"`, `"arr: &R"` — asserting identical
     structure to `capture_with_type` / `no_capture_with_type` / `capture_with_borrow_tame`.
     Add one destructure case (`"[a: int, b: bool]"`) in `destructure_parser_tests.rs`.
   * F: run them; expect failure (the `:` reaches `parse_templex` and errors).
   * I: in `pattern_parser.rs::parse_pattern`, between the `in`-keyword stop (~:230) and the
     `next_is_type` heuristic (~:233), consume an optional `:` — **only when a destination
     local was parsed** — and when present force `next_is_type = true`. Leave the heuristic
     untouched otherwise.
   * G: re-run; expect pass, and the colonless tests still pass.
   * A: full suite.

2. **Optional colon in struct members.**
   * R: in `parsing/tests/struct_tests.rs`, add `struct Moo { a: int; }` alongside the
     existing colonless form.
   * F: run; expect failure.
   * I: in `parser.rs::parse_struct_member` (~:207), consume an optional `:` after the name.
     Confirm placement relative to the variadic `..` check at implementation time.
   * G / A: as above.

3. **Optional colon in generic params.**
   * R: in `parsing/tests/rules/rule_tests.rs`, add `<T: Kind>` beside `<T Kind>`, plus a
     `<T>` case to prove the no-type form is unaffected.
   * F: run; expect failure.
   * I: in `parser.rs::parse_generic_parameter` (~:57), consume an optional `:` before
     `parse_rune_type`. Rune branch only — the region branch synthesizes `ITypePR::RegionType`
     and parses no type.
   * G / A: as above.

4. **`own` → `ownref`.**
   * R: change `parsing/tests/patterns/type_tests.rs::own_prefix_type` to `"_ ownref T"`, and
     `postparsing/test/post_parser_tests.rs` (~:1449/1461/1477) to `x ownref int`.
   * F: run; expect failure (`ownref` isn't a keyword yet).
   * I: `keywords.rs:160` and `:317` (interned twice — parse arena and scout arena) →
     `"ownref"`; `templex_parser.rs:270` uses the renamed field;
     `postparsing/post_parser_error_humanizer.rs:235` emits `"ownref "`.
     `OwnRefPT`/`OwnRefSR` are already named correctly.
   * G / A: as above.

5. **`^` becomes postfix, on local names only** (per design-1:93; see above).
   * R: `"a^"` parses as `Move(local a)`. `"a.b^"` (path) and `"Muta()^"` (call result) are
     parse **errors**. A spaced `a ^ b` is not a postfix move. Rewrite
     `move_call_via_caret` accordingly — most naturally as a *negative* test now.
   * F: run; expect failure.
   * I: drop `Move` from the `Prefix` enum (`expression_parser.rs:1907-1941`) and recognize a
     trailing `^` only where the parsed atom is a bare local lookup. No spree-step arm, and no
     adjacency guard — `^` can never follow an arbitrary expression, so the spaced-binary
     ambiguity cannot arise.
   * G: re-run.
   * A: full suite — this is where the fixture sweep lands (below).

6. **Stale caret-in-type-position fixtures + the override mismatch.**
   * R: none to write — `typing/test/compiler_tests.rs:450` (`func drop(self ^Moo)`) and
     `:793` (`func destructor(m ^Muta)`) already fail, at *parse* time. `^T` as a templex is
     not a thing (there's an explicit test asserting it errors at
     `parsing/tests/patterns/type_tests.rs:88`, which **stays**).
   * F: confirm the current failure is the parse error, not something downstream.
   * I: rewrite both to bare — `func drop(self Moo)`, `func destructor(m Muta)`. Separately,
     `tests/programs/externs/interfaceimmparamextern_owned/test.vale:8` declares
     `abstract func getFuel(virtual ship &IShip)` but its override is `func getFuel(ship Firefly)`
     — a borrow vs. a strong ref. The sibling `interfaceimmparamextern_vale_dispatch/test.vale`
     has both bare; make this one match.
   * G: **these will not necessarily go green.** Expect them to advance past the parse error
     to a typing front line, which the architect owns. Report where they land.
   * A: full suite; confirm the typing pass/fail count moves only by these.

## The fixture sweep (part of slice 5)

~264 caret sites, by area: `src/tests/` 83 · `builtins/` 30 · `typing/test/` ~73 ·
`integration_tests/` 73 (gated) · `parsing/tests/` 5.

**Prefix `^` scoped over the whole tight-suffix chain**, so a naive token swap is wrong wherever
a chain follows. Under the local-names-only rule those sites aren't "move the caret" — they're
illegal constructs. Hand-edit them first:

| Site | Fix |
|---|---|
| `src/tests/hashmap/hashmap.vale:242` — `^innerRemove(...)` | bind a local, then `local^` |
| `parsing/tests/expression_tests.rs:265` — `"^Muta()"` | rewrite as a **negative** test (call results can't be moved) |
| any `(^base).drop()` | bind a local first |

Everything else is `^<identifier>` → `<identifier>^` — the overwhelming majority of the ~264
sites, and mechanical.

Per `docs/skills/scripting.md`, >40 edits means `safe-script-runner` with per-file review, not
`sed`/`perl -pi`, and no parallelization.

## Flagged for the architect — not changed here

- **`typing/compiler_error_humanizer.rs:146` and `:153`** suggest `` `^local` `` in two error
  messages; under postfix they should read `` `local^` ``. In `typing/`, so it's yours. The
  assertion at `compiler_tests.rs:2023` only checks `contains("^")`, so it passes either way.
- **`lexing/lexer.rs:1031-1041`** — `lex_impl_ownership_prefix` lexes `&`, `&&`, **and `^`**
  as type-position ownership prefixes for `impl` heads. `^` in type position isn't in the
  model. Likely stale; needs a decision, not necessarily a change.
- **The existing trailing-`&` spree step** (`expression_parser.rs:1366`) has no adjacency
  guard, so a spaced `a & b` may already mis-parse. Pre-existing; slice 5 adds the guard for
  `^` only.

## Verification

Per `docs/skills/tdd.md` §0 the baseline must be green — and per CLAUDE.md, a failure is never
to be waved off as pre-existing. The honest baseline here: **parser and postparse are green;
typing is red mid-arc at 573/175/8**, which is the arc's known state. This slice must leave
the typing count unchanged except for the slice-6 fixtures.

```bash
# baseline + per-slice, into one fixed session file
cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing:: > ./tmp/valen-syntax-slice.txt 2>&1
grep "test result" ./tmp/valen-syntax-slice.txt

cargo test --manifest-path FrontendRust/Cargo.toml --lib postparsing:: > ./tmp/valen-syntax-slice.txt 2>&1
grep "test result" ./tmp/valen-syntax-slice.txt

# the A substep of every slice
cargo test --manifest-path FrontendRust/Cargo.toml --lib --no-fail-fast > ./tmp/valen-syntax-slice.txt 2>&1
grep "test result" ./tmp/valen-syntax-slice.txt
```

The 146 `parse_sample_test!` cases over `src/tests/**/*.vale` are the real backstop for the
caret sweep — they parse the whole corpus and are currently green, so any mis-edited `^` shows
up there immediately.

Eliminate all new warnings before calling it done (CLAUDE.md).
