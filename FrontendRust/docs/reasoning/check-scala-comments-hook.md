# Check Scala Comments Hook — Reasoning

## PreToolUse (not PostToolUse)

Checks file state before each edit. If parity is already broken from a prior edit, blocks further edits until fixed. The rejection message includes a doc link so Claude can self-correct.

## Rust binary (not Python script)

Matches the existing `validate-readonly` hook pattern. Fast startup (<200ms including process spawn). No Python dependency at runtime.

## No nested block comments

Nested `/* */` is rejected outright. This keeps extraction simple (no recursive parsing needed) and the codebase clean. The Python script's regex also can't handle nesting, so this is consistent.

## Single-file check (not full scan)

Only checks the file being edited, not all 211 pairs. The Python script checks everything; this hook checks one file per edit. Keeps latency under 200ms.

## Standalone hook (not Guardian shield)

This check is fully deterministic (text extraction + diff). Guardian shields are LLM-evaluated and appropriate for judgment calls. A deterministic check should be a deterministic program.
