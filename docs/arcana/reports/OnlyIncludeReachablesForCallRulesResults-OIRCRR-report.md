# Accuracy report: Only Include Reachables for Call Rules' Results (OIRCRR)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:1726 ("# Only Include Reachables for Call Rules' Results (OIRCRR)")

## Verdict

The section describes a bug (call site over-supplying reachable-function instantiation bounds for a `Spork<Bork<int>>`-style example) and a proposed fix: only include reachable bounds for runes the *definition* expects, discoverable by looking at the `CallSR` rules of the callee. The fix appears to already be implemented — `function_compiler_solving_layer.rs` computes `include_reachable_bounds_for_runes` directly from the callee's own parameter/return-type runes (`function.params.iter().map(|p| p.value_type_rune.rune)` plus `maybe_ret_kind_rune`), which is the same idea (definition-driven, not caller-argument-driven). But the mechanism differs from what the doc describes: the code doesn't inspect `CallSR` rules of the function to derive this set, it just reads the params/return rune list off the function declaration directly. The doc is a historical bug-and-fix-plan note whose plan was carried out in spirit but not verbatim, and it carries zero code citations anywhere in the tree.

## Claims

| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | There's a bug where a call site (e.g. `Spork<Bork<int>>(...)`) over-supplies reachable-function bounds because it sees the argument is a citizen with bounds | UNVERIFIABLE (historical) | — | Can't confirm a bug report from a doc note; no test/repro cited |
| 2 | The fix: only include reachables for runes that the *definition* expects (params the definition calls out as citizens) | TRUE (mechanism) / DRIFTED (method) | src/typing/function/function_compiler_solving_layer.rs:585-589 computes `include_reachable_bounds_for_runes` from `function.params`/`maybe_ret_kind_rune`, i.e. definition-driven | Matches the doc's intent |
| 3 | "We can know that by looking at the CallSR rules of the function we're calling, and only include reachables for what comes out of those" | DRIFTED | Same site — the code does not inspect `CallSR` rules to derive the set; it reads `p.value_type_rune.rune` directly off `function.params` | Correct to: the set is just the function's declared parameter/return-type runes, no CallSR-rule inspection needed |

## Stale citation sites

None — the ID has zero citations anywhere in src/ or docs/.

## Uncited sites that embody the arcana

- src/typing/function/function_compiler_solving_layer.rs:585-589 — computes `include_reachable_bounds_for_runes` from the callee's own param/return runes, the fix this section describes.
- src/typing/infer_compiler.rs:338-476 — consumes `include_reachable_bounds_for_runes` to build `InstantiationReachableBoundArgumentsT`, the reachable-bounds machinery this section is about.

## Suggested rewrite

(minor-inaccuracies — rewrite offered for the drifted mechanism claim only)

Replace the final paragraph ("We can know that by looking at the CallSR rules of the function we're calling, and only include reachables for what comes out of those.") with:

> We can know that by looking at the callee's own declared parameter and return-type runes (not the caller's argument runes), and only including reachable bounds for those. This is what `include_reachable_bounds_for_runes` in `function_compiler_solving_layer.rs` does today: it's built straight from `function.params`' `value_type_rune`s and `maybe_ret_kind_rune`, not from re-walking `CallSR` rules.
