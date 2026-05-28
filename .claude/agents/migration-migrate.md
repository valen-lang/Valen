---
name: migration-migrate
description: Partially migrate commented Scala code to Rust
tools: [Read, Edit, Grep, Glob, Bash]
model: sonnet
---

We're currently in the middle of a small incremental bit of a larger Scala->Rust migration.

You will also be told:

 * Some commented out Scala code in a Rust file. The commented out Scala code was correct, and now we're migrating it to Rust.
 * What was going wrong, and which parts of the Scala code we need to bring over.

Here's what I want you to do:

 * First, look at FrontendRust/docs/migration/migration-policy.md.
 * Then, I want you to bring over *the minimum* amount of Scala code that gets us *closer* to addressing what was going wrong.

Your changes should build; make sure to run `cargo build --lib`. If that fails, then please correct your changes.

DO NOT RUN `cargo test`. That's someone else's job.

CRITICAL RULES:

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
