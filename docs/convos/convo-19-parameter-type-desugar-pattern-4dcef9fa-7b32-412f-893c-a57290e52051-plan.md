# Plan document

Source: `/Users/verdagon/.claude/plans/delegated-shimmying-unicorn.md`
Session: 4dcef9fa-7b32-412f-893c-a57290e52051

---

# Plan: Parameters carry a name + type, not a pattern; desugar to a body `let` only for destructures

## Context

**Why.** On `experimental-2` (onion typing), a function parameter's type is stored on `ParameterS` in two shapes at once — the split type fields *and* a full `pattern: AtomSP`. The signature-time `translate_pattern` call walks the whole pattern and emits its type rules into a **throwaway** vec (`discarded_pattern_rules`, `function_scout.rs:417/431`), because the split is then recomputed by `translate_signature_templex`. That discard is a design smell: it means the parameter's type is being computed twice, in two shapes.

**Root cause.** A prior session desugared **every** parameter to a body-head `let` and gave every param a synthetic `DesugaredParamName`, moving the user's binding name to that let. The architect confirms this was a misunderstanding — the intent was a body `let` **only for parameters that actually destructure**. The blanket desugar is what forced `ParameterS` to carry a full pattern and made `translate_pattern` do double duty.

**Outcome.** A parameter is fundamentally a **name + a type**. The signature keeps the type (the split stays — it's needed so a function is findable in each param's value-kind namespace). The **destructure** is the only body concern, so it moves to a synthesized body-head `let`, built once, whose rules land in the real `LetSE.rules` bucket. `translate_pattern` runs at most once per param, only when there's a destructure, and nothing is thrown away.

**Two orthogonal rules** (from `PatternPP`'s three independent optionals — `destination` name, `templex` type, `destructure`):
- Synthetic `DesugaredParamName` is used **iff the param has no user-written name** (anonymous `Pair[a,b]`, or ignored `_`). Named params keep their real name (`p`, `self`, closure name, magic name).
- A body-head `let` is synthesized **iff the param has a destructure.**

| Param form | `ParameterS.name` | Body-head let |
|---|---|---|
| `p Pair` | `CodeVarName(p)` | none |
| `_ Pair` / anon | synthetic `DesugaredParamName` | none |
| `p Pair[a,b]` | `CodeVarName(p)` | `let [a,b] = load(p)` |
| `Pair[a,b]` | synthetic `DesugaredParamName` | `let [a,b] = load(slot)` |
| `&self` / closure / magic | real name (`self` / closure / magic) | none |

## Scope boundary

Baseline is the **green working tree** (501/0/1) with `typing`/`tests`/`solver` gated out in `lib.rs` (`#[cfg(any())]`, uncommitted). **This refactor is postparse-only** and must keep that build green. The ~20 gated `typing/` read-sites of `param.pattern.*` and the old 4-arg `ParameterS::new` callers (`struct_constructor_macro.rs`, `*_drop_macro.rs`, `anonymous_interface_macro.rs`, etc.) are already RED at HEAD and excluded from the working-tree build; they are handled when the typing slice is un-gated, **not here**.

## Changes

### 1. `ParameterS` shape (`src/postparsing/ast.rs:362-421`)
- **Remove** the `pattern: AtomSP` field (`:384`) and the trailing `pattern` arg of `ParameterS::new` (9 → 8 args).
- **Delete** the `assert!(pattern.kind_rune.is_some(), …)` (`:399`) — type is now carried by `full_type_rune`/`value_type_rune`.
- **Relax** `debug_assert!(matches!(name, DesugaredParamName(_)))` (`:401`) to a whitelist of the legal name variants actually produced (`DesugaredParamName | CodeVarName | ConstructingMemberName | ClosureParamName | MagicParamName`), since named params now keep real names.
- **Keep** the two split-bucket asserts (`:403-408`, `:409-412`) unchanged.
- Rewrite the doc comment (`:367-370`) to state the two orthogonal rules.

### 2. Param loop (`src/postparsing/function_scout.rs:361-495`)
Introduce a per-explicit-param carry struct, returned alongside the `ParameterS` and the existing synthesized rune; build the destructure **once** here (single source of truth for both captures and the body-let — do not re-derive at body-synthesis):

```rust
struct ExplicitParamExtras<'s> {
  range: RangeS<'s>,
  abi_name: IVarNameS<'s>,          // == ParameterS.name; slot the body-let loads
  destructure: Option<(AtomSP<'s>, &'s [IRulexSR<'s>])>, // Some iff parser pattern had a destructure
}
```
- `.map` closure now returns `(ParameterS, Option<RuneUsage>, ExplicitParamExtras)`; the split loop at `:496-503` grows a third `Vec<ExplicitParamExtras>` in explicit-param order.
- **`&self` arm (`:371-405`):** keep the synthesized `kind_rune`/split; set `ParameterS.name` to the real `CodeVarName(self_)` (not `DesugaredParamName`); delete the inline `AtomSP`; emit `ExplicitParamExtras { abi_name: CodeVarName(self_), destructure: None }`.
- **User-param arm (`:406-490`):** keep `translate_signature_templex` (`:447-456`) and the untyped-lambda implicit-rune path (`:460-472`) **unchanged** — the split is untouched. **Delete** the throwaway `translate_pattern` block (`:417-443`) and the `pattern_s.kind_rune = …` overwrites (`:457`, `:470`). Derive `ParameterS.name` from `pattern.destination` using the same mapping `translate_pattern` uses (`pattern_scout.rs:93-121`): `LocalNameDeclaration→CodeVarName`, `ConstructingMemberNameDeclaration→ConstructingMemberName`, ignored/`None`→synthetic `DesugaredParamName`. If `pattern.destructure.is_some()`, translate only the **inner** destructure patterns (mirroring `pattern_scout.rs:73-91`) into a fresh rule `Vec`, assemble a top `AtomSP { name: None, kind_rune: None, destructure: <inner atoms> }`, and set `destructure: Some((atom, rules))`; merge new `rune_to_explicit_type` entries via the existing dedup loop (`:436-443`).

### 3. Synthesized params (`create_closure_param :891-980`, `create_magic_parameters :982-1029`)
- Delete the inline `AtomSP`s; change `ParameterS.name` from `DesugaredParamName` to the **real** name already computed (`ClosureParamName` at `:907-914`; `magic_param_name`). No other logic change — they never destructure, carry no `ExplicitParamExtras`, and append straight into `total_params_s`.
- **Magic→GenericParameterS (`:744-763`):** retarget `magic_param.pattern.kind_rune`/`.range` (`:750-757`) to `magic_param.full_type_rune` / `magic_param.range`.

### 4. Capture declarations (`:517-549`)
Replace the `get_parameter_captures(&param.pattern)` loop (`:541-546`) with gathering off `explicit_param_extras`: push `extras.abi_name` as a declared variable **only when it's a real name** (not `DesugaredParamName(_)`), then `get_parameter_captures(atom)` for the destructure's inner names when present. `get_parameter_captures` (`pattern_scout.rs:15-40`) keeps its signature. Closure-local declaration (`:520-539`) unchanged.

### 5. Body-head `let` loop (`:765-802`)
Iterate `explicit_param_extras` (explicit order) and emit a `LetSE` **only** for entries with `destructure: Some((atom, rules))`: `let atom = load(abi_name)` (`LoadAsP::Use`, `rules` → `LetSE.rules`). Wrap in the `ConsecutorSE` **only if** at least one let was produced; otherwise leave `body_s` untouched. Interleaving of closure/magic in `total_params_s` is irrelevant since only explicit params emit lets and each loads its own `abi_name`.

### 6. `translate_pattern` (`pattern_scout.rs:42-129`)
**No change to the function** — only its call site moves (into the destructure branch of the param loop, §2). The body-let's top `AtomSP` is deliberately **typeless** (`name: None, kind_rune: None`): the param's type rides into the let via `load(abi_name)`, so feeding `pattern.templex` would duplicate the type rule at the wrong altitude. Inner annotations (`Pair[a int, b int]`) still resolve via each inner `translate_pattern` call.

### 7. Postparse tests (`src/postparsing/test/`)
These assert the old blanket-desugar behavior and are **deliberately** rewritten to the new behavior (flag for architect awareness — this is a behavior change, not a fix):
- `post_parser_tests.rs:1520-1557` `test_bare_param_desugars_to_let_at_body_head` — a bare `foo(x int)` now yields **no** head-let. Rewrite to assert `params[0].name == CodeVarName("x")` and no leading `LetSE`. Rename off "desugars_to_let".
- `post_parser_tests.rs:1377`, `:1735` — `assert!(matches!(param.name, DesugaredParamName(_)))` invert for named params; assert the real `CodeVarName`.
- `post_parser_tests.rs:409-455` (lambda param matches) — the `ParameterS { pattern: AtomSP {…} }` literals break; reassert on `param.name` + type runes (magic/named params now carry real names, no pattern).
- **Add** a test that a destructure param (`Pair[a,b]` and the nested case) **does** produce a head-let with the inner names, and that `p Pair[a,b]` yields param name `p` **plus** the `[a,b]` let.
- `traverse.rs:411` — drop `visit_pattern(&parameter.pattern)`; optionally add visits of `parameter.full_type_rune`/`value_type_rune` and the two rule slices if the traverse is meant to reach every node. Destructure patterns are still reached via the body `LetSE` (`traverse.rs:487`).

## Open questions (surface, do not block postparse)

1. **`p Pair[a,b]` load semantics.** The head-let is `let [a,b] = load(p)` with `LoadAsP::Use`. Whether `p` stays usable after the destructure (move vs borrow) is a **typing/runtime** decision, gated and unverifiable now. Keep `LoadAsP::Use` (status quo) and confirm intended semantics when typing is un-gated.
2. **`::new` name assert.** Whitelist legal name variants (recommended) vs. drop entirely.

## Verification

- `cargo build --manifest-path ./FrontendRust/Cargo.toml --lib` → clean, zero warnings (typing stays gated).
- `cargo test --manifest-path ./FrontendRust/Cargo.toml --lib` → **501 passed / 0 failed / 1 ignored** (same count; the rewritten desugar tests replace old assertions, and the new destructure-let test adds coverage). Pipe both to a single `./tmp/param-desugar.txt` per repo convention.
- Targeted: the param/pattern postparse tests in `post_parser_tests.rs` (bare param → no let + real name; `Pair[a,b]` → head-let with `a,b`; `p Pair[a,b]` → name `p` + `a,b` let; `&self` → name `self`, no let; lambda magic/named params → real names).
- Sanity: `grep` confirms no remaining `ParameterS.pattern` reads or `discarded_pattern_rules` in postparse, and `ParameterS::new` has 8 args at every live call site.
