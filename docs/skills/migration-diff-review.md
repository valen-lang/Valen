---
name: migration-diff-review
description: Reviews a Scala-to-Rust migration diff for correctness, Scala parity, and migration checklist compliance. Use when reviewing migration diffs, checking migration quality, auditing Scala-to-Rust port accuracy, or when the user asks to review what someone missed in a migration.
---


Please do a `git diff HEAD` (for the entire codebase, or a specific file if I mentioned one). We'll be looking at either the entire codebase or a specific function if I mentioned one.

Someone just helped this migration from Scala to Rust, but im not sure they did it well. Can you tell me what they missed or got wrong? Don't fix it yet.

- Does it correspond well to the scala code below it?
- Does it conform to all the checks in FrontendRust/docs/migration/migration-policy.md?
- Are there any other differences that we should be worried about? It's okay if there are panic!s for unimplemented parts, but I want to know about any other problems.
- this passes tests, so we don't want to implement any missing logic, but we might want to put in assertions and panics. everything should either be correct or panic!ing.
- In tests, is there something that Scala checks that Rust does not?
- In tests, are there any mismatches between what Rust is checking for and what Scala is checking for?
- Is there anything we can do to make this more closely match the old Scala code? For example:
   - Is the Rust code inlining code from somewhere where Scala instead makes a call to another function?
   - Is there any code that isn't directly above the old Scala code that inspired it? If so, that's likely novel code, which we DON'T WANT.
   - Did we make everything closer to scala behavior, or is there anything that is now further from scala behavior?
   - Do the if-statements/match-statements/loops match up between the Rust version and the Scala version? They should. But feel free to ignore any branches with panic!s in them.
- Are there any Rust functions that are not above their old Scala version?
