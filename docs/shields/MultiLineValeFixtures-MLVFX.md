---
description: Embedded Vale fixtures must be multi-line raw strings — not one-line raw strings with a body block, and not concat! string-fragment builders.
g_model: SimpleSmall
g_primary: rust
g_program: MultiLineValeFixtures-MLVFX
g_context: definition
g_filter_file: "*"
g_read_when: Read when writing an `r#"..."#` raw string containing embedded Vale source in a Rust test.
g_mention_in:
  - CLAUDE.md
---

# Multi-Line Vale Fixtures (MLVFX)

Embedded Vale source in a Rust test fixture goes in a **multi-line** raw string when the fixture has a body block (`{...}` with content) or more than one statement/definition. Compact one-liners like a bare import or an empty struct declaration stay legal.

Multi-line fixtures diff cleanly, read like real Vale, and let you edit line-by-line. Cramming a function body onto a single line makes future edits noisy — the whole line changes for a one-token tweak.

The same applies to fixtures built with `concat!("...\n", ...)`: the per-line quoting and `\n` escapes are noise, and the fixture no longer reads like Vale. Use a multi-line raw string instead. (A `concat!` that is not building Vale source — a path, a message — is fine.)

## Examples

**DENY:**
```rust
let code = r#"exported func main() int { return +(&2, &3); }"#;
```

**DENY:**
```rust
let code = r#"struct Foo {} func bar() { return 1; }"#;
```

**DENY:**
```rust
let code = concat!(
  "import v.builtins.tup0.*;\n",
  "func main() { }\n",
);
```

**ALLOW:**
```rust
let path = concat!(env!("OUT_DIR"), "/generated.rs");
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
