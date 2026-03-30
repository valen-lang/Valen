# Check Scala Comments Hook — Architecture

Location: `.claude/hooks/check-scala-comments/`

## Input

JSON on stdin (Claude Code PreToolUse format):
```json
{"tool_input": {"file_path": "/absolute/path/to/file.rs"}}
```

## Flow

1. Parse stdin JSON, extract `tool_input.file_path`
2. Resolve project root from `CLAUDE_PROJECT_DIR` env var
3. Strip `FrontendRust/` prefix from file path, look up in `FILE_MAP`
4. If not found: exit 0 (not our concern)
5. Read both Rust and Scala files
6. Extract block comments from Rust file (depth-tracking state machine, panics on nesting)
7. Filter migration annotations (Guardian, MIGALLOW, AFTERM)
8. Normalize both sides (strip leading whitespace, remove blank lines)
9. Compare. If equal: exit 0
10. Generate unified diff via `similar` crate, print to stderr with doc link, exit 2

## FILE_MAP

Static array of 211 `(&str, &str)` tuples embedded in the binary. First element is relative to `FrontendRust/`, second is relative to `Frontend/`.

## Block comment extraction

Uses a char-based state machine (not regex) to correctly handle multi-byte UTF-8 characters. Tracks depth — if depth exceeds 1, panics (nested block comments are banned).

## Exit codes

- **0**: File not in map, or parity check passed
- **2**: Parity check failed (diff printed to stderr)
- **panic**: Nested block comment, missing file, missing env var, parse failure