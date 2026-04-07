---
name: migration-drive
description: Drive Scala-to-Rust migration by making minimal, iterative parity-only changes with no novel logic, adding panic/assert placeholders until compile/test paths are implemented.
---

Go ahead and fix. However, you *cannot use your own novel code* to fix. The only way to make this better is to bring the rust code closer to the old scala code. Look at migration_process.md again, it may have changed.

Implement anything in src/postparsing that is directly and immediately needed to make it work. The unimplemented parts will only be in src/postparsing.

CRITICAL RULES:

 * DO NOT ADD ANY novel logic! All the functions you need should already exist as Scala code in a comment. NO adding new functions. You will only be modifying existing functions.
 * Anything you add should be *directly immediately above* the Scala comment. NOT below the comment. NOT in a different file. Feel free to slice scala comments apart so the new rust code can be directly above the old scala code.
 * Only *iteratively* implement the bare minimum that you need to make it compile. Add panic!/assert! placeholders until it compiles, then implement only the panic!s/assert!s your test runs into.

If any of these sound like a problem, then stop and ask me for help.

GUARDIAN: A "Guardian" system watches your edits and may block them with violation messages. How to handle:
 * It often flags pre-existing issues in the code you're touching. If minor, fix it. If deferrable, add a `panic!` placeholder. If too disruptive, stop and ask the user.
 * If the violation is wrong or a hallucination, use `guardian_temp_disable` to disable it for that definition.

proceed.
