# Check Scala Comments Hook — Usage

A PreToolUse hook that runs before every Edit/Write, verifying that Scala block comments in Rust files still match their original Scala source files.

## When it fires

Before every `Edit` or `Write` tool call. If the file being edited is in the FILE_MAP (211 Rust↔Scala file pairs), the hook reads the current Rust file, extracts all `/* */` block comments, and compares them against the original Scala source.

## What the error means

If you see a rejection from this hook, it means the Scala block comments in the Rust file have drifted from the original Scala source. The diff shows exactly which lines differ.

Common causes:
- A block comment was accidentally modified during an edit
- A block comment was deleted or split incorrectly
- Content was accidentally placed outside a `/* */` block
- The original Scala file was modified but the Rust file's comments weren't updated

## How to fix

1. Read the diff in the rejection message
2. Fix the block comment in the Rust file to match the Scala source, OR
3. If the Scala source intentionally changed, update both files to match

## Files the hook skips

- Files not in the FILE_MAP (exit 0, no check)
- Files outside `FrontendRust/src/` (exit 0)

## Nested block comments

Nested `/* */` is **not allowed**. If detected, the hook panics immediately. Fix by flattening into separate non-nested block comments.

## Adding a new file pair

Edit the `FILE_MAP` array in `.claude/hooks/check-scala-comments/src/main.rs`, then rebuild:
```bash
cd .claude/hooks/check-scala-comments && cargo build --release
```

## What is filtered before comparison

These migration annotations are stripped from block comments before comparing:
- `Guardian:` lines
- `// MIGALLOW` lines (including multi-line continuations)
- `MIGALLOW:` lines
- `AFTERM:` lines
- Trailing `// MIGALLOW...` suffixes
