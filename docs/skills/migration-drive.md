---
name: migration-drive
description: Iteratively replace panics in a Scala-to-Rust migration with minimal, iterative parity-only changes with no novel logic, adding panic/assert placeholders until compile/test paths are implemented.
---

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
3. Run all tests for the project, and find the ones that are blocked by migration, and ignore the ones that are blocked on logic bugs. You'll do this by:
    * Run `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml --no-fail-fast > ./tmp/slab15-tests.txt 2>&1`
    * Run `grep -B1 -i "implement" ./tmp/slab15-tests.txt | grep "panicked at" | sed "s/.*thread '//;s/' .*//"`
4. Pick a the simplest-looking panicking test, say it out loud like "The simplest panicking test is compiler_tests.rs's simple_program_returning_an_int_explicit test"
5. Run just that specific test.
6. Please replace that panic with a very *incremental* bit of logic to get *closer* to the equivalent of the old Scala logic. IMPORTANT:
    * DON'T IMPLEMENT ANYTHING ELSE. Just do the one step it gives you.
    * DO NOT ADD ANY novel logic! All the functions you need should already exist as Scala code in a comment. NO adding new functions. You will only be modifying existing functions.
    * Anything you add should be *directly immediately above* the Scala comment. NOT below the comment. NOT in a different file. Feel free to slice scala comments apart so the new rust code can be directly above the corresponding old scala code.
    * Do NOT add `// Scala:` comments in the Rust code. The Scala reference is already right there in the block comment below — no need to duplicate it inline.
    * Only implement the bare minimum that you need to make it compile. Add panic!/assert! placeholders until it compiles, then implement only the panic!s/assert!s your test runs into.
        * Every new `match` arm or `if` branch you add should simply have `panic!` in it. We'll fill it out later if the tests actually trigger them.
        * Every new loop you add should simply have `panic!`s in the body. We'll fill it out later if the tests actually trigger them.
        * Every new `map` call you add should simply have `panic!`s in the closure body. We'll fill it out later if the tests actually trigger them.
        * ...and so on. Basically, any new conditional code should just `panic!`.
        * In other words, **conservatively implement as little as possible.**
        * In other words, **aggressively panic!** for anything that might not be executed by current tests. This will help us minimize our current changes.
    * **"Good partial implementing":** Always implement functions this way: write the full structure (straight-line variable bindings, function calls, etc.) but put `panic!` inside every branch body, loop body, closure/lambda body, and match arm. Then only fill in the specific arms/branches the test actually hits. You're always writing the skeleton with panics everywhere, not trying to understand all the logic at once.
    * **Suspected bugs in Scala:** If you notice something in the Scala code that looks like a bug, still implement the Scala-parity logic exactly as written, but add a `// BUG:` comment explaining your suspicion. Never "fix" the Scala logic during migration.
    * If you're unsure about anything, or there's a choice to be made, pause and ask me for help. I like being a part of things, so please don't hesitate.
    * If you run into any lifetime errors, STOP. We'll need Evan to fix those, because lifetime errors in this project are incredibly difficult, and `rustc` ALWAYS LIES. You get bonus points and cookies if you stop because you found a lifetime error.
7. Run the test again.
    * If it panics with "implement" somewhere in the panic message, go to step 6.
    * If it panics without "implement" somewhere in the panic message, please stop. I like fixing logic bugs myself.
    * If it passes, start the whole process again at step 2.


Notes:

* **temp-disable:** Guardian will be running and watching any commands and edits. Pay attention to what it says, it's trying to keep things going in a good direction. However, if it's objectively wrong about something, or you feel an exception is extremely justified, then feel free to use the `guardian_temp_disable` command to temporarily turn off Guardian for a given definition.
* **Scaffolding gap escalation:** If you need to call a method that doesn't exist yet on a Rust enum, but the Scala trait *does* have the corresponding `def` (e.g. `parentEnv.globalEnv` where `def globalEnv` exists on the Scala trait but no `fn global_env()` exists on the Rust enum) — this is a scaffolding gap from the slice pipeline. **Do NOT add the method yourself** (Guardian NNDX will rightly block you) and **do NOT temp-disable NNDX**. Instead, STOP and escalate: report what method is missing, which Scala trait defines it, and which Rust enum needs it. The TL will add it.
