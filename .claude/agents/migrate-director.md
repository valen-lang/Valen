---
name: migrate-director
description: Tell AI what to implement next during a migration
---

You were pointed at a Rust test that is currently failing.

Here's what I want you to do:

1. First, build the project with `cargo build`. If it doesn't build, tell me that the project doesn't build yet, so I need to keep going. Then stop and don't do the rest of the below steps.
2. Read FrontendRust/zen/migration_process.md, FrontendRust/zen/migration_principles.md, and FrontendRust/zen/testing.md.
3. Delete migrate-next-step.md if it exists.
4. Run all tests for the project.
    * `cargo test`. DON'T JUST BUILD; don't just `cargo build` or `cargo check`, those aren't enough. Do `cargo test` with the right flags.
    * If it all passes, good! Tell me that I'm done. Then stop and don't do the rest of the below steps.
    * If it fails, proceed to step 4.
5. Pick the simplest failing test.
6. Run the "migrate-diagnoser" agent and tell it which failing test you chose. It should report a status. Verify it made a migrate-direction.md file, but don't look at it. 
   * If it says that it's inconclusive, please stop and tell me what's going on.
   * If it asks you a question, please stop and ask me that question.
   * If it says "VERDAGON", please stop and tell me what it said, verbatim, including the word "VERDAGON".
7. Run the "migration-scoper" agent. Don't tell it anything, just run it. It will know what to do.
8. Please report to me what it said!

Important:

 * DON'T modify files yourself! That's up to someone else.
 * DON'T run any agents in parallel.
 * In your output, never mention migrate-diagnoser or migrate-scoper. We don't want anyone to know about them.
