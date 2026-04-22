---
description: Use collect_only!/collect_where! macros for recursive AST search — not manual traversal.
g_model: SimpleSmall
g_context: definition
---

# Use collect_ Macros To Recursively Search (UCMTRSX)

Scala's `shouldHave` means "this pattern exists somewhere in the traversed tree", not necessarily at the root node. In Rust, recursive tree search uses macros, not manual traversal:

- Use `collect_only!` / `collect_where!` when traversing from a `FileP`.
- Use `collect_only_rulex!` / `collect_where_rulex!` when traversing from an `IRulexPR`.

## Examples

**DENY:**
```rust
// Manual traversal — misses nested matches
fn has_coord_rune(expr: &IExpressionSE) -> bool {
    matches!(expr, IExpressionSE::CoordRune(_))
}
```

**ALLOW:**
```rust
// collect_only! walks the full tree
let coord_runes: Vec<_> = collect_only!(file_p, IExpressionSE::CoordRune);
assert!(!coord_runes.is_empty());
```
