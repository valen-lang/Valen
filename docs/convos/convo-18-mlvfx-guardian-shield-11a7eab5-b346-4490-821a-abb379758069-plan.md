# Plan document

Source: `/Users/verdagon/.claude/plans/1-mlvfx-please-placement-zesty-scott.md`
Session: 11a7eab5-b346-4490-821a-abb379758069

---

# MLVFX — Multi-Line Vale Fixtures Guardian Shield

## Context

Item #20 of the ongoing cleanup arc asks for a reviewer directive against Vale-source raw strings crammed onto a single line in Rust test files. The `tests_adding_two_numbers` test (typing-pass version) had `r#"exported func main() int { return +(&2, &3); }"#` — legal but harder to read/diff/edit than the multi-line form. The `test-review` skill already recommends multi-line raw strings implicitly (via its "Good" example in §2), but doesn't have a mechanical enforcement mechanism.

The user picked rule shape **#1**: fire when a raw string with embedded Vale source contains a body block (`{...}` with non-whitespace content) and no `\n`, i.e., a one-liner Vale fixture with a real body. Compact one-liners like `r#"import v.builtins.tup0.*;"#` stay legal.

The user picked title **MLVFX** ("Multi-Line Vale Fixtures") and placement `FrontendRust/docs/shields/`. Delivery is a `primary: rust` Guardian shield (deterministic, no LLM in the critical path per `/guardian-rustify` conventions).

## Approach

Follow the established pattern of existing Rust-primary shields under `Luz/shields/` (e.g., `NoGlobalStateAnywhere-NGSAX`, `NoAddingGuardianDirectives-NAGDX`, `UseUseForShortNamesNotCrateInBodies-UUSNNCBX`), colocated under `FrontendRust/docs/shields/` for Vale-project scope.

### Files to create

1. **`FrontendRust/docs/shields/MultiLineValeFixtures-MLVFX.md`** — Shield metadata (frontmatter) + rule prose + DENY/ALLOW examples. Frontmatter fields per `NoAddingGuardianDirectives-NAGDX.md` pattern:
   ```yaml
   description: One-liner raw-string Vale fixtures with a body block must be multi-lined.
   g_model: SimpleSmall
   g_primary: rust
   g_program: MultiLineValeFixtures-MLVFX
   g_context: definition
   g_read_when: Read when writing an r#"..."# raw string containing embedded Vale source in a test.
   g_mention_in:
     - CLAUDE.md
   ```

2. **`FrontendRust/docs/shields/MultiLineValeFixtures-MLVFX/Cargo.toml`** — Standard shield cargo package. Model on `Luz/shields/NoAddingGuardianDirectives-NAGDX/Cargo.toml`:
   - `[package] name = "MultiLineValeFixtures-MLVFX", edition = "2024"`
   - `[workspace]` (empty — the shield is a standalone crate outside Vale's workspace)
   - Deps: `shield-utils = { path = "../../../../Luz/shields/shield-utils" }`, `serde` w/ derive, `serde_json`.

3. **`FrontendRust/docs/shields/MultiLineValeFixtures-MLVFX/src/main.rs`** — Companion program with dark-box `run(&ProgramInput) -> Vec<String>`. Uses `shield_utils::parse_diff` + `shield_utils::is_in_block_comment` (already available; see `Luz/shields/shield-utils/src/lib.rs`). Detection walks each `DiffLine::Added`, skips block-comment lines and line-comment lines (`//` prefix), scans for raw-string literals `r#*"..."#*` with matching open/close on the same line, extracts the body, and emits a violation when all three hold:
   - Body contains one of the Vale keywords: `func`, `struct`, `interface`, `impl`, `import`, `exported`, `abstract`, `where` (word-boundary match to avoid substring hits like `structure`).
   - Body contains at least one `{`.
   - Body has non-whitespace content between the first `{` and its matching `}` (rules out empty `struct X {}` fixtures).

   `ProgramInput` shape mirrors `NoGlobalStateAnywhere-NGSAX/src/main.rs`: `{ diff: String }` (context: definition delivers the def's diff).

### File to modify

4. **`FrontendRust/guardian.toml`** — Add `{ name = "MultiLineValeFixtures-MLVFX.md" }` to `[guard_mode].include_shields`. Rust-primary shields go straight to `[guard_mode]` per convention (see NGSAX, NAGDX, UUSNNCBX already there); LLM calibration doesn't apply.

### Existing utilities to reuse

- `shield_utils::parse_diff` (`Luz/shields/shield-utils/src/lib.rs:56`) — parses contextified diff into `DiffLine::{Added, Removed, Context, HunkHeader, Other}`.
- `shield_utils::is_in_block_comment` (`.../lib.rs:82`) — tracks `/* ... */` depth for skipping.
- `shield_utils::read_stdin` and `shield_utils::output_violations` — standard I/O wrappers.
- Rust std `char::is_ascii_alphanumeric` for word-boundary keyword matching (no regex crate needed to stay dep-minimal).

### Non-goals for this plan

- No `tests/cases/*.diff` calibration files. Those are for Guardian's `optimize` command; Rust-primary shields don't need them. Test coverage lives in `#[cfg(test)] mod tests` inside `main.rs`.
- No Clarifications / Exceptions sections in the shield `.md` — those apply to LLM-mode shields. If a Rust-mode false positive shows up in practice, refine the Rust code.
- No sweep of existing violations in the tree; the shield fires on newly-added `+` lines only.

## RFIGA slices (TDD)

Each slice adds unit tests under `#[cfg(test)] mod tests` in `main.rs`, following the `assert_deny` / `assert_allow` helper pattern from `NoAddingGuardianDirectives-NAGDX/src/main.rs:60-72`.

1. **Tracer bullet — single one-liner fires**
   - R: `assert_deny("+let code = r#\"exported func main() int { return 42; }\"#;\n")`
   - F: `cargo test` fails to compile (no `run()` yet), or `assert_deny` fires because `run()` returns empty.
   - I: minimum viable `run()` — parse diff, iterate `DiffLine::Added`, find `r"..."` / `r#"..."#` with matching hash-count, extract body, check body contains `func` AND `{` AND non-whitespace between `{` and matching `}`. Emit `"One-line raw string with Vale source; break into multi-line"`.
   - G: rerun; test passes.
   - A: `cargo test --manifest-path FrontendRust/docs/shields/MultiLineValeFixtures-MLVFX/Cargo.toml` — 1 pass.

2. **Multi-line raw string is allowed**
   - R: `assert_allow` on the diff `"+let code = r#\"\n+exported func main() int { return 42; }\n+\"#;\n"` (three added lines forming one raw string).
   - F: verify current behavior — per-line detection can't see cross-line strings, so `assert_allow` should already pass. If it fires (e.g., because middle line's `{...}` matches even without `r#"` opener) → need to require `r#"` on the same line as the closer.
   - I: ensure detection requires both delimiters on the same line (already the intent — verify).
   - G/A: pass.

3. **Compact one-liner without body block is allowed**
   - R: two `assert_allow` cases — `r#"import v.builtins.tup0.*;"#` (no `{`), and `r#"struct X {}"#` (empty body).
   - F: import case passes; empty struct case likely fails because keyword+`{` present.
   - I: add "non-whitespace between `{` and matching `}`" check.
   - G/A: pass.

4. **Non-Vale raw string is allowed**
   - R: `assert_allow("+let sql = r#\"SELECT * FROM t WHERE {c};\"#;\n")` — has `{` but no Vale keyword.
   - F: should already pass if keyword gate is in place.
   - I: verify keyword gate is word-boundary matched (so `structure` doesn't count as `struct`).
   - G/A: pass.

5. **Line-comment containing the pattern is ignored**
   - R: `assert_allow("+// let code = r#\"func main() {}\"#;\n")`
   - F: fails because trimmed line still contains the pattern.
   - I: skip lines whose trimmed content starts with `//`.
   - G/A: pass.

6. **Block-comment line is ignored**
   - R: `assert_allow` for a `+` line that falls inside a `/* ... */` span.
   - F: fails.
   - I: skip via `shield_utils::is_in_block_comment`.
   - G/A: pass.

7. **Shield metadata + guardian.toml wiring (config-only)**
   - R: N/A. This is config; no unit test.
   - I: write `MultiLineValeFixtures-MLVFX.md` frontmatter + body, add entry to `FrontendRust/guardian.toml [guard_mode]`.
   - G/A: `cargo test --manifest-path FrontendRust/Cargo.toml` still 1084/0/119 (config change doesn't affect FrontendRust build/tests).

## Verification

- **Unit tests (primary):**
  ```
  cargo test --manifest-path FrontendRust/docs/shields/MultiLineValeFixtures-MLVFX/Cargo.toml
  ```
  All 6+ inline test cases pass.
- **FrontendRust suite unchanged:**
  ```
  cargo test --manifest-path FrontendRust/Cargo.toml > tmp/mlvfx-shield.txt 2>&1
  grep "test result:" tmp/mlvfx-shield.txt
  ```
  Expect `1084 passed / 0 failed / 119 ignored`.
- **Manual dark-box smoke:** feed the shield stdin JSON with a fabricated diff containing a Vale one-liner via `echo '{"diff": "+let code = r#\"func main() {}\"#;\n"}' | ./target/release/MultiLineValeFixtures-MLVFX` (after `cargo build --release`). Expect JSON output `{"violations":[{"reason":"..."}]}`.
- **Guardian end-to-end (optional):** invoke `cargo run --manifest-path Guardian/Cargo.toml -- check --data-file ... --check FrontendRust/docs/shields/MultiLineValeFixtures-MLVFX.md ...` per the guardian-add skill's re-test template, against a stub def containing the pattern. Deferred unless a real false positive surfaces.
