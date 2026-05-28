---
name: migrate-scoper
description: Tell AI what to implement next during a migration
---

You should see a "tmp/migrate-direction.md" file in the project directory.
It contains some migration instructions from a rather inexperienced project manager.
If you don't see a "tmp/migrate-direction.md" file, please stop here and say "VERDAGON" to bring my attention to it.

Here's what I want you to do:

First, please look at FrontendRust/docs/migration/migration-policy.md. The information will be necessary for this.

Then, please answer these questions:

1. Do the instructions say that we're already done? If so, please say "VERDAGON" to bring my attention to it. 
2. Do the instructions seem confused? If so, please say "VERDAGON" to bring my attention to it.
3. If it identifies some other problem (not just a simple bit of further needed migration), please say "VERDAGON" to bring my attention to it.
4. Do the instructions have multiple steps? If so, please pick the first one. We want the next step to bring over *the minimum* amount of Scala code that gets us *closer* to addressing what was going wrong.
   * If you see multiple panics causing teh current problem, only recommend fixing the one panic we actually hit (as reported by tmp/migrate-direction.md) or the one we would hit first. Do NOT recommend fixing multiple panics at once.
   * Make sure the instructions mention that we don't need it to work end-to-end yet, we just need it to get a tiny bit closer.

After you answer those questions, please tell me some updated instructions for the next step to implement.

If there is something that confuses you, stop and ask me for help by saying my name "VERDAGON". I like being a part of things, so please don't hesitate.

Important: DON'T modify any files!
