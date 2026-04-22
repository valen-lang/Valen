---
name: migration-test-fixer
description: Migrate Scala code to Rust verbatim until a given test passes
---

You were pointed at a Rust test that is currently failing.

Here's what I want you to do:

 1. First, look at these files:
    * ./FrontendRust/zen/migration_principles.md
    * ./FrontendRust/zen/testing.md
    * ./Luz/shields/NoValidSimplifications-NVSEX.md
    * ./Luz/shields/ScalaSealedTraitsToRustEnums-SSTREX.md
    * ./Luz/shields/TodosAndUnimplementedCodeMustPanic-TUCMPX.md
    * ./Luz/shields/MigrateAllCommentsToo-MACTX.md
    * ./Luz/shields/NeverRecoverAlwaysFail-NRAFX.md
    * ./Luz/shields/FailFastFailLoud-FFFLX.md
    * ./Luz/shields/NoChangesWithoutScalaReference-NCWSRX.md
    * ./Luz/shields/ImmediateInterningDiscipline-IIDX.md
    * ./Luz/shields/UseUseForShortNamesNotCrateInBodies-UUSNNCBX.md
    * ./Luz/shields/AvoidIfMatchesInTestsIfPossible-AIMITIPX.md
    * ./Luz/shields/KeepInlineComparisonsInline-KICIX.md
 2. Try to build the project. if it doesn't build, then please make it build.
    * If you run into any easy lifetime fixes, please do them. If you run into any medium or complicated ones, or ones that span multiple definitions, please stop and tell me, because I like solving lifetime challenges.  
 3. Run all tests for the project.
    * `cargo test`. DON'T JUST BUILD; don't just `cargo build` or `cargo check`, those aren't enough. Do `cargo test` with the right flags.
    * If it all passes, good! Stop, you're done.
    * If it fails, proceed to step 3.
 4. Pick a the simplest-looking failing test, say it out loud like "The next simplest failing test is compiler_tests.rs's simple_program_returning_an_int_explicit test", and then run just that specific test.
 5. Run the "migrate-diagnoser" agent and give it the command line you used to run the specific test. It should report a status. Verify it made a migrate-direction.md file, but DON'T look at it. It will report one of:
    * `INCONCLUSIVE`: please stop and tell me what's going on.
    * `QUESTION`: please stop and ask me that question.
    * `FINDINGS` (something that needs to be migrated further): please proceed to step 6.
    * `PANIC` (a `panic!` corresponding to not-yet-migrated Scala code): please proceed to step 6.
    * If it reports anything else, please stop and tell me what's going on. I like to help with that.
 6. Run the "migration-scoper" agent. Don't tell it anything, just run it. It will know what to do. It will either:
    * Say "VERDAGON": please stop and tell me what's going on. That means I need to look at it myself immediately.
    * Give you clear instructions for the next step: please proceed to step 7.
    * Not give you clear instructions: please stop and tell me what's going on. I like to help with that.
 7. If you get here, it's because there's something that needs to be migrated further. Please execute the instructions it gave you. IMPORTANT:
    * DON'T IMPLEMENT ANYTHING ELSE. Just do the one step it gives you.
    * DO NOT ADD ANY novel logic! All the functions you need should already exist as Scala code in a comment. NO adding new functions. You will only be modifying existing functions.
    * Anything you add should be *directly immediately above* the Scala comment. NOT below the comment. NOT in a different file. Feel free to slice scala comments apart so the new rust code can be directly above the corresponding old scala code.
    * Only implement the bare minimum that you need to make it compile. Add panic!/assert! placeholders until it compiles, then implement only the panic!s/assert!s your test runs into.
        * Every new `match` arm or `if` branch you add should simply have `panic!` in it. We'll fill it out later if the tests actually trigger them.
        * Every new loop you add should simply have `panic!`s in the body. We'll fill it out later if the tests actually trigger them.
        * Every new `map` call you add should simply have `panic!`s in the closure body. We'll fill it out later if the tests actually trigger them.
        * ...and so on. Basically, any new conditional code should just `panic!`.
        * In other words, **conservatively implement as little as possible.**
        * In other words, **aggressively panic!** for anything that might not be executed by current tests. This will help us minimize our current changes.
    * If you're unsure about anything, or there's a choice to be made, pause and ask me for help. I like being a part of things, so please don't hesitate.
    * If you run into any lifetime errors, STOP. We'll need Evan to fix those, because lifetime errors in this project are incredibly difficult, and `rustc` ALWAYS LIES. You get bonus points and cookies if you stop because you found a lifetime error.
 8. Run the test again.
    * If it passes, stop here, you're done.
    * If it fails:
       * If this is at least the fifth failure in a row, please pause and ask me for help.
       * If this isn't the fifth failure in a row, go to step 4.
