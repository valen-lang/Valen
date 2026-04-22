---
description: Every struct and enum must have a doc comment categorizing it as arena-allocated, value-type, interned, temporary state, or miscellaneous.
g_model: SimpleSmall
g_context: definition
primary: rust
program: TypesFitIntoTheseCategories-TFITCX
defs: struct, enum
---

# Types Fit Into These Categories (TFITCX)

Every struct and enum definition must have a `///` doc comment above it (before any `#[derive(...)]` or other attributes) categorizing it into exactly one of these five categories:

- `/// Arena-allocated (see @TFITCX)` — stored as `&'s T` or `&'p T` via `arena.alloc()`, immutable after construction, no Clone
- `/// Value-type (see @TFITCX)` — derives Copy, stored inline in slices or struct fields
- `/// Interned (see @TFITCX)` — a canonical deduplicated handle returned by an intern method
- `/// Temporary state (see @TFITCX)` — mutable working data during a pass (environments, builders, accumulators), may Clone
- `/// Miscellaneous type (see @TFITCX)` — doesn't fit the above (errors, solver state, test infrastructure)

## Examples

**DENY:**
```rust
pub struct StructS<'s> {
    pub name: INameS<'s>,
}
```

**DENY:**
```rust
// Wrong category prefix
pub struct StructS<'s> {
    /// This is a postparser AST node
    pub name: INameS<'s>,
}
```

**ALLOW:**
```rust
/// Arena-allocated (see @TFITCX)
pub struct StructS<'s> {
    pub name: INameS<'s>,
}
```

**ALLOW:**
```rust
/// Value-type (see @TFITCX)
pub struct RangeS<'s> {
    pub begin: CodeLocationS<'s>,
    pub end: CodeLocationS<'s>,
}
```

**ALLOW:**
```rust
/// Temporary state (see @TFITCX)
pub struct StackFrame<'s> {
    pub locals: VariableDeclarations<'s>,
}
```

## Exceptions

A. Structs inside `/* ... */` Scala block comments (commented-out Scala code, not active Rust).

B. Test-only structs (inside `#[cfg(test)]` modules).
