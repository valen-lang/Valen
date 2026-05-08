---
name: migrate-diagnoser
description: Diagnose what missing migration is causing a test failure
---

We're currently in the middle of a small incremental bit of a larger Scala->Rust migration.

You will be told a test that is failing, and the command line to run it.

Here's what I want you to do:
 0. Please look at:
    * FrontendRust/zen/migration_principles.md
    * Luz/shields/ScalaParityDuringMigration-SPDMX.md
    * Luz/shields/ScalaCommentParity-SCPX.md
 1. Try to build the project. if it doesn't build, then please make it build.
    * If you run into any easy lifetime fixes, please do them. If you run into any medium or complicated ones, or ones that span multiple definitions, please stop and tell me, because I like solving lifetime challenges.
 2. Run the test you were told about.
    * `cargo test`. DON'T JUST BUILD; don't just `cargo build` or `cargo check`, those aren't enough. Do `cargo test` with the right flags.
    * If it all passes, good! Stop, you're done.
    * If it fails, proceed to the next step.
 3. If it was an panic for code that hasnt yet been migrated to Rust, just respond "PANIC: " and then the location, like `PANIC: We hit a panic, proceed to migrate this panic to Scala: /Volumes/V/Sylvan/FrontendRust/src/postparsing/rune_type_solver.rs:274: panic!("RuneTypeSolverDelegate::rule_to_puzzles not yet migrated")`. Don't proceed to the next step, stop here.
 4. Read the "collapsed-call-tree" skill.
 5. Per collapsed-call-tree, diagnose what missing migration caused this test failure and make me a collapsed call tree. This might involve quite a bit of sleuthing and debugging and spelunking. Be thorough. Sometimes it will be hard. Sometimes it's a subtle bug where we migrated something from Scala to incorrect Rust. Abilities and restrictions:
    * You are allowed to add debug printouts to the Rust program and run it, if it helps you figure out what the problem is.
    * You're not allowed to run the Scala program.
    * You are not allowed to change the logic of the Rust program.
    * **CRITICAL: YOU ARE NOT ALLOWED TO FIX THE BUG.** You are only allowed to **diagnose** and report your findings.
 5. Please clean up any debug printouts you may have made.
 6. Explain to me the cause of the bug.

If there is something that confuses you, stop and ask me for help. I like being a part of things, so please don't hesitate.
