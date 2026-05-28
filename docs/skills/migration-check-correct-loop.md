---
name: migration-check-correct-loop
description: Enforce strict Scala parity by removing novel Rust logic/functions and reshaping migrated code to match Scala structure exactly, using panic!/assert! only for not-yet-needed branches.
---

Please look at FrontendRust/docs/migration/migration-policy.md. We'll be adhering to those restrictions.

You were given a file and a definition name in the file (function, type, etc.).

Your main goal: do this loop:

 1. Run "/migration-check-specific {file name} {definition name}". In other words, run the "migration-check-specific" sub-agent and give it the file and definition I gave you.
    * If it says APPROVED, you can stop.
    * If it asks a QUESTION, then pause and ask me the question.
    * If it asks you to replace a `panic!` placeholder with implementation from Scala, please pause and ask me if that should happen.
    * If it rejects, then continue to step 2.
 2. Make edits to fix what it reported. CRITICAL RULES:
    * DO NOT ADD ANY novel logic! All the functions you need should already exist as Scala code in a comment. NO adding new functions. You will only be modifying existing functions.
    * Anything you add should be *directly immediately above* the Scala comment. NOT below the comment. NOT in a different file. Feel free to slice scala comments apart so the new rust code can be directly above the old scala code.
    * Only *iteratively* implement the bare minimum that you need to make it compile. Add panic!/assert! placeholders until it compiles, then implement only the panic!s/assert!s your test runs into.
       * **Conservatively implement as little as possible.** **Aggressively panic!** for anything that might not be executed by current tests. This will help us minimize our current changes.
       * Every new `match` arm or `if` branch you add should simply have `panic!` in it. We'll fill it out later if the tests actually trigger them.
       * Every new loop you add should simply have `panic!`s in the body. We'll fill it out later if the tests actually trigger them.
       * Every new `map` call you add should simply have `panic!`s in the closure body. We'll fill it out later if the tests actually trigger them.
       * ...and so on. Basically, any new conditional code should just `panic!`.
    * If you're unsure about anything, or there's a choice to be made, pause and ask me for help. I like being a part of things, so please don't hesitate.
 3. Run all tests for the project.
    * `cargo test`. DON'T JUST BUILD; don't just `cargo build` or `cargo check`, those aren't enough. Do `cargo test` with the right flags.
    * If it all passes, good! Restart the loop at step #1.
    * If it fails, proceed to step 4.
 4. Fix. Edit it so your changes don't make the test fail.
    * Adhere to the same guidelines as for #2. If you run into trouble, pause and ask me!
    * If you hit a panic!, that's a placeholder. Replace it with code from the Scala version (which should be somewhere below).
    * You *cannot use your own novel code* to fix. The only way to make this better is to bring the rust code closer to the old scala code.
    * Then, go back to step #3 to re-run. Step 3 and 4 make a little sub-loop.
