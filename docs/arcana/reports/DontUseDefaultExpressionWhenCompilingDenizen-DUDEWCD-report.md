# Accuracy report: Don't Use Default Expression When Compiling Denizen (DUDEWCD)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:153 ("# Don't Use Default Expression When Compiling Denizen (DUDEWCD)")

## Verdict
The core idea is still true and still implemented, but the mechanism described ("we need to *not* execute that LiteralSR(N, 5) rule") is stale Scala-era phrasing. In current Rust code, a generic parameter's default is not a rule mixed into the definition's rule list that gets conditionally skipped — it's stored separately on the generic-parameter struct (`gp.default`) and simply never appears among the rules run during definition compilation (`struct_a.header_rules` / `member_rules`). Any generic param left unsolved after running those rules gets a fresh placeholder via `create_placeholder`, not the default value. So the behavior matches, but "not execute LiteralSR(N, 5)" no longer describes the actual code path — there's no rule-skipping step for defaults; they're architecturally excluded rather than filtered out. minor-inaccuracies.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | When compiling definitions, we always populate placeholders for every unresolved argument, never using default expressions | TRUE | src/typing/citizen/struct_compiler_generic_args_layer.rs:511-535 (`create_placeholder` called for first unsolved generic param in `compile_struct_layer`'s solve loop) | — |
| 2 | Example `struct Thing<N Int = 5> { vec Vec<N, Float>; }` is valid current syntax | TRUE | src/typing/test/compiler_tests.rs:5343 (`struct MyHashSet<K, H Int = 5> { }`) | — |
| 3 | "we need to *not* execute that LiteralSR(N, 5) rule" — implies a rule named LiteralSR is present in the definition's rule set and is skipped | DRIFTED | src/typing/citizen/struct_compiler_generic_args_layer.rs:184,294 (`gp.default` is a separate field, not part of `header_rules`/`member_rules`); src/typing/infer_compiler.rs:1249-1258 (`include_rule_in_definition_solve` filters `CallSiteFunc`/`Resolve`, not any literal/default rule) | Default value is stored on the generic-parameter declaration (`gp.default: Option<...>`) and is simply not part of the rule list solved when compiling the denizen's own definition (`all_rules_s = header_rules + member_rules`); there is no "LiteralSR" rule being filtered out — the exclusion is structural, not a rule-kind filter. |

## Stale citation sites
None — the only in-repo citation (docs/architecture/instantiator-design.md:663) is a bare list of IDs with no specific claim to check.

## Uncited sites that embody the arcana
- src/typing/citizen/struct_compiler_generic_args_layer.rs:519-535 — `create_placeholder` fills in unsolved generic params instead of using `gp.default`, for structs.
- src/typing/citizen/struct_compiler_generic_args_layer.rs:665-681 — same pattern for interfaces.
- src/typing/citizen/struct_compiler_generic_args_layer.rs:184, 294 — `defaults_rune_to_type` built from `gp.default`, kept separate from the rules solved during definition compile.

## Suggested rewrite
When compiling definitions, we need to always populate placeholders for every argument, and never use default expressions.

Let's say we have:

```
struct Thing<N Int = 5> {
  vec Vec<N, Float>;
}
```

When we compile the innards of that, we don't want to assume that N is 5, because it could be anything.

So, when compiling the struct's own definition, we solve only its `header_rules`/`member_rules` (see `include_rule_in_definition_solve` in src/typing/infer_compiler.rs) — the generic parameter's default expression (`gp.default`) is kept separate and is never run here. Any generic parameter left unsolved after that gets a fresh placeholder (`create_placeholder`) instead of its default value.
