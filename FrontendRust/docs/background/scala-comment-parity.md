# Scala Comment Parity

Every Rust file in `FrontendRust/src/` contains the original Scala code as `/* */` block comments, interleaved with the Rust implementations. This is a migration invariant (see `docs/migration/process.md` rule P2: "Rust Code Should Be Above its Scala Code"). The Scala comments serve as the authoritative reference for what the Rust code should implement.

A PreToolUse hook (`check-scala-comments`) automatically verifies this parity before every AI edit. It compares 211 Rust↔Scala file pairs, checking that the block comment contents match the original Scala source files. See `docs/usage/check-scala-comments-hook.md` for details.