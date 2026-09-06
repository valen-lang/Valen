# Syntax migration — handoff

The corpus syntax migration to Valen spelling. It splits by whether the change alters the AST, and the
tiers are ordered so each has a verifier — nothing is ever done blind.

**Numbers here are stale unless marked otherwise.** The caret and attribute inventories were counted
once by a method not recorded; **re-measure before sizing any sweep**. Grep the corpus fresh —
`grep` for the caret/attribute spellings across `.vale` and `typing/test/` — rather than trusting a
figure below.

## The tiers

- **TIER 1 — spelling-only, do NOW, parser-verified.** `^`→postfix, `own`→`ownref` **(rename only — do
  not also narrow it; see below)**, `#!DeriveStructDrop`→`#explicitly_destroyed`, optional colons. These
  leave the AST identical (`^x` and `x^` are both `IExpressionPE::Move`), so the **146
  `parse_sample_test!` cases plus postparse's 106 `#[test]` functions are a real verifier** — and they are green
  *independently of typing's state*. Not blind. The plan is `~/.claude/plans/compressed-whistling-sketch.md`,
  approved. Doing it now also stops the corpus accreting more stale syntax.
- **TIER 2 — meaning-changing, do AFTER the feature lands, error-driven.** C1's `&` insertion, `*`
  insertion, the `set` / `set *` split. **Never sweep these.** Implement the rule so the old form becomes
  a *compile error*, then fix what the compiler rejects; a green suite means the migration is complete by
  construction. The hazard that makes a blind sweep unacceptable: a missed borrow becomes a strong ref
  and **still typechecks**.
- **TIER 3 — much later.** Position-dependent bare class, and anything riding on design-2.

**The rule for tier 1: change the `.vale` source text, never the assertion values.** The only deliberate
exceptions are tests asserting the old syntax is rejected.

## Tier-1 caret inventory (`^`→postfix)

*(Counted once by a method not recorded; re-measure before sizing any sweep — the figures below are
current greps but drift fast.)* **~411 carets repo-wide** (408 under `src/`), of which ~111 are inside the
`.vale` corpus under `src/`. The old 271-Vale / 240-non-Vale split is not reproducible and must be
re-derived by classifying each `.rs` caret as embedded-Vale fixture vs humanizer `^->` arrow. The
restructure bucket is **3 live constructs, not 4** — the `tests/list/list.vale` `^set` construct is gone.

- **The dangerous bucket is ~89 humanizer caret-arrow carets**, concentrated in two files:
  `after_regions_error_tests.rs` (**28**, around lines 622-626) and `compiler_solver_tests.rs`
  (**61**, lines 693-703 and 1607-1609). `typing/test/` totals **186** carets. A blind sweep mangles these.
- **THREE sites are constructs, not spellings** — illegal under local-names-only, needing a local bound
  first. Two are in a `remove` function; grep the snippet rather than trusting a line, and **note there
  are two `hashmap.vale`s** — the one you want is `tests/hashmap/`, not `tests/regionhashmap/`, which is
  shorter and has neither site:
  - `tests/hashmap/hashmap.vale`, in `func remove` — **`^innerRemove(`**
  - same function — **`^(^maybeNeighbor).get()`**, an outer `^` on a parenthesized call result
  - `move_call_via_caret`'s `"^Muta()"`
- **A separate bucket of type-position carets, already broken today** — `func moo(m ^Muta)`,
  `func drop(m ^Muta)`, `wand ^Wand;`, `&[#3]^MutableStruct`. Two sit in currently-failing typing tests
  with `BadTypeExpression`. **These want `ownref`, not a postfix `^`.**
- **Two tests to handle deliberately.** `caret_type_is_error` asserts `^T` at templex level is a parse
  error, is **currently passing**, and **must survive** — it's the only `is_err()` assertion in all of
  `parsing/tests`, `postparsing/test`, and `lexing`. And `move_call_via_caret` asserts `"^Muta()"` parses
  to `Move(FunctionCall)`, is **currently green**, and is the exact inverse of the new rule — delete or
  invert it.

`^` is POSTFIX and applies to LOCAL NAMES ONLY — design-1:93, *"`^` (postfix). Move operator on local
names (**not on paths**)"*; restated at 2308. We parse `^` as a prefix operator — `Prefix::Move` in
`expression_parser.rs`, lowering to `IExpressionPE::Move`. Flipping this touches the parser, the
expression tests, and every `^x` in the corpus (~264 sites, overwhelmingly `^<local>`).

## Tier-1 attribute inversion hazard (`#!` is semantic, not cosmetic)

`#DeriveStructDrop` → `CallMacro` (run it); `#!DeriveStructDrop` → `DontCallMacro` (**suppress** it),
dispatched in `determine_macros_to_call` (`typing/compiler.rs`) and `compile_struct_core`. **Every
attribute site in the tree is `#!`** — the lexer has arms for the bangless form but nothing writes it, so
the corpus is uniformly *suppress*. Rewriting `#!DeriveStructDrop` → `#derive(StructDrop)` would therefore
**invert every one of them.** **The target is `#explicitly_destroyed`**: a bare `#name`, admitted upstream
(design-1:2716), also meaning *suppress*, so the rewrite preserves meaning.

Attribute grammar is `#name` or `#name(args)`, never `#[name]` — design-1:2279/2283, with bare `#name`
admitted at design-1:2716.

The three derives map as follows:
- **`#!DeriveStructDrop`** (measure the count fresh) → `#explicitly_destroyed`. **One-for-one**, and safe
  to run: bare `#name` attributes are admitted and both spellings mean *suppress*, so the rewrite
  preserves meaning.
- **`#!DeriveInterfaceDrop`** → **not an attribute at all — it's the KIND choice.** `interface` carries a
  drop, `trait` doesn't. **Not a pure rename**: the two kinds also differ on erasure, so check what our
  interfaces actually *are* first (the interface/open-trait tier split is a separate, unbuilt semantic
  gap, not a spelling change).
- **`#!DeriveAnonymousSubstruct`** → no Valen analogue. Ours to resolve.

**Attribute inventory, measured** *(re-measure before acting)*: `#!DeriveStructDrop` **67**,
`#!DeriveInterfaceDrop` **14**, `#!DeriveAnonymousSubstruct` **2** — 83 total across `.vale` and `.rs`.

~252 mechanical sites ⇒ `safe-script-runner` with a **context-aware** transform, never a global one.

## `own` → `ownref` (tier 1 rename; the narrowing is not)

`own` is the `OwnRef` wrap — parser `OwnRefPT`, postparse `OwnRefSR`, `translate_own_ref_templex`,
humanizer, onion-wrap permitted-list and traverse, all mirroring `WeakRef`.

The rename is nearly free: the `Keywords::own` field and its two `intern_str("own")` initializers, one
consumer (`parse_ref_prefix`'s `OwnRefPT` arm), two humanizer arms
(`IRulexSR::OwnRef` in `post_parser_error_humanizer.rs`, `KindT::OwnRef` in `compiler_error_humanizer.rs`),
and **only two fixture sites**, both passing. **Zero `.vale` files use it** — the keyword exists
but no corpus file caught up. The Rust identifiers (`OwnRefPT`/`OwnRefSR`/`KindT::OwnRef`) already read
as "ownref" and need no rename.

**The rename is tier 1; the *narrowing* is not.** After it, the parser still accepts `ownref Point` on a
movable struct, which design-1:1555 calls *"not terser or safer, it is wrong"* — `ownref` is for
**immovable** types only. Rejecting it needs the movability axis, which we do not have. Ship the rename
knowing the parser stays permissive; narrowing it waits on the immovable-types axis, a later unbuilt
semantic gap.

## The blind-surface answer

`integration_tests/` is **linked into `lib.rs`** (`pub mod integration_tests;` is active; only its
`#[cfg(test)]` gate is commented), so under `cargo test` it compiles its ~321 `#[test]`s across ~30 files
— it IS a verifier, not a blind surface. `parse_sample_test!` covers **146 of 195** `.vale` files, so the
genuinely blind `.vale` surface is small: a few `*restackify.vale` carets and `regionhashmap/hashmap.vale`
attributes, plus **`builtins/resources/*.vale`** (29 carets, 9 active `#!Derive` sites), **not in the
parse-sample corpus at all** — they reach the parser only via typing tests.

## The `*` deref operator (tier 2 for the corpus; buildable now)

Ruled, small, and nothing is blocked on it. `k` = the reference, `*k` = the pointee; `set k = …`
re-points, `set *k = …` writes through; field and method access still auto-deref (`k.field`,
`k.method()`, `set self.hp -= 10`), so no `(*k).field` anywhere. We have **no `*` prefix operator at
all** — parser + postparse node + the two-depth `set` distinction. A lookup yields the address of the
slot and the read-path `DerefTE` peels exactly one storage layer, so `k` is the stored reference and `*k`
is simply *one more peel*. That is a parser addition plus a `DerefTE` at a new site, not a model change.
It is **tier 2** for the corpus (a site wanting the pointee type-errors rather than silently working, so
the migration is compile-error-driven), but the *feature* is buildable now. It flips design-1:171,
design-1:1496, and rubric:161, which all predate it.

## Optional colon, and `&` borrow-only

- **Optional colon in `name: type`.** No colon support for the `name: type` param/pattern form in
  `parsing/` (it does parse an optional `: T` group-element-type annotation on a group parameter,
  `<g': Entity>`). Add the param/pattern form as accepted-but-not-required. This is one of the two intended divergences: Vale2 *allows but does not
  require* it (design-1:2350 permits the colonless form). Coupled to the next item: colonless only parses
  if `&` can't be bitwise-and.
- **`&` is borrow-only, permanently — no bitwise-and.** Bitwise-and gets its own spelling (`bitand` or
  similar). This is the accepted price of a colonless form being parseable. Record before we need bitwise
  operators.

## Lessons learned

*Accumulates wisdom, not events. One or two sentences per entry; prune what nobody can act on.*

- **A meaning-changing rewrite is never swept, always driven by compile errors** (tier 2). A missed
  borrow becomes a strong ref and **still typechecks**, so a blind sweep can silently change semantics —
  make the old form a compile error first, then fix what the compiler rejects.
- **Measure a corpus population with every spelling it has.** A `share` census that missed the retired
  `imm` spelling undercounted by more than half; a migration sized off it is wrong in the same
  proportion.
- **"It compiles" does not prove a syntax was captured.** `&T in g[]` compiled clean while `parse_group`
  silently dropped the `[]`; only a parser/postparse unit test asserting the scouted shape caught it.
  Assert the shape, not just that a fixture compiles.
- **A blind sweep mangles the humanizer caret-arrow carets.** The dangerous bucket is diagnostic-diagram
  carets in the error-test files, not source moves — separate them out before any `safe-script-runner`
  transform.
- **`#!` is suppress, not derive.** The whole corpus is `#!` (suppress); rewriting to `#derive(...)`
  would invert every site. The meaning-preserving target is `#explicitly_destroyed`.
