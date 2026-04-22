---
name: guardian-rustify
description: Convert an LLM-based Guardian shield into a Rust-mode shield with a deterministic companion program.
read-when: Read when promoting an LLM-mode shield to Rust mode with a deterministic companion program.
mention-in:
  - CLAUDE.md
---

# Rustifying a Shield

Convert an existing LLM-based shield into a Rust-mode shield where a deterministic Rust program is authoritative and the LLM only runs on appeal or crash fallback.

## When to rustify

A shield is a good candidate for rustification when:
- The check is pattern-based (grep-like) rather than semantic
- The LLM has persistent false positives/negatives that clarifications can't fix
- You want deterministic, fast, zero-cost checks

## Steps

### 1. Create the companion program directory

Create a Rust crate alongside the shield file:

```
shields/
  MyShield-MSX.md
  MyShield-MSX/
    Cargo.toml
    src/
      main.rs
```

`Cargo.toml`:
```toml
[package]
name = "MyShield-MSX"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1"
```

### 2. Write main.rs

The program receives a JSON object on stdin with two fields:
- `file_path` — the absolute path of the file being edited
- `diff` — the contextified diff (same text the LLM would see)

It must print JSON to stdout. Separate the checking logic into a `check` function so unit tests can call it directly without spawning a subprocess.

```rust
fn check(diff: &str) -> Vec<String> {
    let mut violations = Vec::new();

    for line in diff.lines() {
        if !line.starts_with('+') || line.starts_with("+++") {
            continue;
        }
        let content = &line[1..];
        // ... check content for violations ...
        // if bad: violations.push("description of violation".to_string());
    }

    violations
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).expect("failed to read stdin");
    let parsed: serde_json::Value = serde_json::from_str(&input).expect("invalid JSON input from Guardian");
    let diff = parsed["diff"].as_str().expect("missing 'diff' field in Guardian input");

    let violations = check(diff);

    if violations.is_empty() {
        println!("{{\"violations\":[]}}");
    } else {
        let result = serde_json::json!({
            "violations": violations.iter()
                .map(|r: &String| serde_json::json!({"reason": r}))
                .collect::<Vec<_>>()
        });
        println!("{}", result);
    }
}
```

If your check is path-based rather than diff-based, use `parsed["file_path"]` instead of `parsed["diff"]`.

Output format: `{"violations": []}` for pass, `{"violations": [{"reason": "..."}]}` for deny.

### 3. Update the shield frontmatter

Add `primary: rust` and a `program:` field. **Keep `model:`** — it is required even in Rust mode, used for crash fallback and appeal LLM calls:

```markdown
---
description: One-line summary.
g_model: SimpleSmall
g_primary: rust
g_program: MyShield-MSX
---
```

`primary: rust` makes the program authoritative — the LLM does not run during normal operation. If the program crashes, the LLM runs as a fallback using the shield's `model:` tier.

### 4. Write unit tests

Add `#[cfg(test)]` unit tests directly in `main.rs`. These are far more valuable than manual testing — they run instantly, cover edge cases, and document expected behavior. Call `check()` directly with diff strings (or file paths) — no subprocess needed:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allow_with_category() {
        let diff = "+/// Arena-allocated (see @TFITCX)\n+pub struct Foo {}";
        assert!(check(diff).is_empty());
    }

    #[test]
    fn deny_without_category() {
        let diff = "+pub struct Foo {}";
        assert!(!check(diff).is_empty());
    }

    #[test]
    fn allow_with_attributes_between() {
        let diff = "+/// Value-type (see @TFITCX)\n+#[derive(Copy, Clone)]\n+pub struct Bar {}";
        assert!(check(diff).is_empty());
    }

    #[test]
    fn skip_block_comments() {
        let diff = "+/*\n+pub struct ScalaCode {}\n+*/";
        assert!(check(diff).is_empty());
    }
}
```

Aim for at least 5-10 tests covering: basic ALLOW, basic DENY, attributes between category and definition, block comment skipping, and any edge cases specific to your shield.

### 5. Build and run tests

```bash
cd shields/MyShield-MSX && cargo test && cargo build
```

### 6. Verify against existing test cases

If the shield has test cases in `tests/`, run `check-direct` against each to verify the Rust program produces the same results:

```bash
cargo run --manifest-path Guardian/Cargo.toml -- check-direct \
  --input shields/MyShield-MSX/tests/001.diff \
  --referenced-defs shields/MyShield-MSX/tests/001.referenced_defs.txt \
  --file-path src/myfile.rs \
  --check shields/MyShield-MSX.md \
  --cache-dir /tmp/cache \
  --backend opencode \
  --log-dir /tmp/logs \
  --format human \
  --log-level overview
```

### 7. Keep shield markdown in sync

The shield `.md` file is the **requirements document** for the companion program. When the program's behavior changes later (new patterns, expanded rules), the markdown must be updated to match. A program that silently enforces rules not described in the markdown is a drift bug.

The markdown serves dual duty: it is both the specification for the Rust program AND the actual LLM prompt used for crash fallback and doublecheck appeals. So it must remain phrased as shield instructions to an LLM judge — not as developer documentation for the Rust code. Write new rules the way you'd write any shield rule: describe what to ALLOW and DENY, give examples, explain the reasoning. An LLM reading only the markdown should reach the same conclusions as the Rust program.

This applies to all future edits, not just initial rustification. Every PR that changes a companion program's logic should also update the shield markdown.

## How it works at runtime

- **Program runs** (authoritative) — LLM does not run
- **If program crashes** — LLM runs as fallback, case written to `cases/need-trainee-training/`
- **If implementor appeals** (via `guardian_temp_disable`) — case written to `cases/need-doublecheck-override/`; appeal-LLM doublechecks during next review pass (see `docs/architecture/governance.md` section 3.3)

## Working example

See `Luz/shields/UseUseForShortNamesNotCrateInBodies-UUSNNCBX/` for a working companion program.
