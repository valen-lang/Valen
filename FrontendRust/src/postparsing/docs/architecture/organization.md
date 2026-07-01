---
g_read_when: "Read when navigating PostParser methods across files."
g_auto_load_when_editing:
  - FrontendRust/src/postparsing/*.rs
---

# Postparsing Organization Map

- Postparsing responsibilities are split across files as `impl PostParser` methods.
- `expression_scout.rs` contains expression-scouting methods on `PostParser`.
- `function_scout.rs` contains function-scouting methods on `PostParser`.
- `post_parser.rs` is the top-level orchestrator calling those methods via `self`.
- Methods take `&self` (`PostParser`) and can directly access all PostParser sub-parts.
