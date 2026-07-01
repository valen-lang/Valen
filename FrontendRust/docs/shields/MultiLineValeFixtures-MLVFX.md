---
description: One-liner raw-string Vale fixtures with a body block must be multi-lined.
g_model: SimpleSmall
g_primary: rust
g_program: MultiLineValeFixtures-MLVFX
g_context: definition
g_read_when: Read when writing an `r#"..."#` raw string containing embedded Vale source in a Rust test.
g_mention_in:
  - CLAUDE.md
---

# Multi-Line Vale Fixtures (MLVFX)

Embedded Vale source in a Rust test fixture goes in a **multi-line** raw string when the fixture has a body block (`{...}` with content) or more than one statement/definition. Compact one-liners like a bare import or an empty struct declaration stay legal.

Multi-line fixtures diff cleanly, read like real Vale, and let you edit line-by-line. Cramming a function body onto a single line makes future edits noisy — the whole line changes for a one-token tweak.

## Examples

**DENY:**
```rust
let code = r#"exported func main() int { return +(&2, &3); }"#;
```

**DENY:**
```rust
let code = r#"struct Foo {} func bar() { return 1; }"#;
```

**ALLOW:**
```rust
let code = r#"
exported func main() int { return +(&2, &3); }
"#;
```

**ALLOW:**
```rust
let code = r#"import v.builtins.tup0.*;"#;
```

**ALLOW:**
```rust
let code = r#"struct X {}"#;
```

**ALLOW:**
```rust
let sql = r#"SELECT * FROM t WHERE {c};"#;
```
