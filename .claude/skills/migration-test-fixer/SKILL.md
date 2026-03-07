---
name: migration-text-fixer
description: Migrate Scala code to Rust verbatim until a given test passes
---

You were pointed at a Rust test that is currently failing.

Here's what I want you to do:

 1. First, look at FrontendRust/zen/migration_process.md, FrontendRust/zen/migration_principles.md, and FrontendRust/zen/testing.md.
 2. Run all tests for the project.
    * `cargo test`. DON'T JUST BUILD; don't just `cargo build` or `cargo check`, those aren't enough. Do `cargo test` with the right flags.
    * If it all passes, good! Stop, you're done.
    * If it fails, proceed to step 3.
 3. Pick a failing test.
 4. Run "/migration-diagnoser {which test}". In other words, run the "migration-diagnoser" sub-agent and give it the test that fails.
    * If it says that it's inconclusive, please stop and tell me what's going on.
    * If it asks you a question, please stop and ask me that question.
    * If it identifies something that needs to be migrated further, please proceed to stop 3.
    * If it blames a `panic!` that corresponds to not-yet-migrated Scala code, please proceed to stop 3.
    * If it identifies some other problem (not just a simple bit of further needed migration), please stop and tell me what's going on.
 5. If you get here, it's because there's something that needs to be migrated further. Run "/migration-migrate {file name} {definition name}". In other words, run the "migration-migrate" sub-agent and give it the file and definition I gave you. It will bring over a little bit more Scala.
 6. Run the test again.
    * If it passes, go to step 2.
    * If it fails:
       * If this is at least the fifth failure in a row, please pause and ask me for help.
       * Otherwise, go to step 4.

Important:

 * DON'T modify files yourself! That's up to /migration-migrate. Tell /migration-migrate the things that /migration-diagnoser said. /migration-migrate will do the fixes, not you.
 * DON'T run any sub-agents in parallel.