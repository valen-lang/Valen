---
name: migration-drive
description: Iteratively replace panics in a Scala-to-Rust migration with minimal, iterative parity-only changes with no novel logic, adding panic/assert placeholders until compile/test paths are implemented.
---

> **Every time you compact, re-read this file** (`docs/skills/migration-drive.md`). It changes often as the TL adds notes about new gotchas, escalation patterns, and migration rules learned during the session. Compaction drops the prior conversation but not the file — if you re-read it, you pick up everything the previous instance learned. If you don't, you'll repeat mistakes that have already been documented.

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
    * ./Luz/shields/ScalaParityDuringMigration-SPDMX.md
2. Try to build the project. if it doesn't build, then please make it build.
    * If you run into any easy lifetime fixes, please do them. If you run into any medium or complicated ones, or ones that span multiple definitions, please stop and tell me, because I like solving lifetime challenges.
3. Run the non-ignored tests: `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml > ./tmp/slab15-tests.txt 2>&1`. Most tests have `#[ignore]` — only the currently-active test(s) will run. Do NOT use `-E` to filter to a specific test — run all non-ignored tests so you catch regressions in previously-passing tests. If the active test panics with "implement", proceed to step 4. If it passes, STOP and report success — the TL will un-ignore the next test.
4. Please replace that panic with a very *incremental* bit of logic to get *closer* to the equivalent of the old Scala logic. IMPORTANT:
    * DON'T IMPLEMENT ANYTHING ELSE. Just do the one step it gives you.
    * DO NOT ADD ANY novel logic! All the functions you need should already exist as Scala code in a comment. NO adding new functions. You will only be modifying existing functions.
    * Anything you add should be *directly immediately above* the Scala comment. NOT below the comment. NOT in a different file. Feel free to slice scala comments apart so the new rust code can be directly above the corresponding old scala code.
    * Do NOT add `// Scala:` comments in the Rust code. The Scala reference is already right there in the block comment below — no need to duplicate it inline.
    * Do NOT copy old Scala code into the Rust code as `//` comments above the new Rust code (e.g. `//   val results = env.lookupFoo(...)`). The Scala is already preserved in the `/* ... */` block below so you don't need to copy those into the new Rust. However, DO bring over any *explanatory* comments from the Scala (e.g. `// Changed this from AnythingLookupContext to TemplataLookupContext because...`) — those are real comments that belong in the Rust code too.
    * Only implement the bare minimum that you need to make it compile. Add panic!/assert! placeholders until it compiles, then implement only the panic!s/assert!s your test runs into.
        * Every new `match` arm or `if` branch you add should simply have `panic!` in it. We'll fill it out later if the tests actually trigger them.
        * Every new loop you add should simply have `panic!`s in the body. We'll fill it out later if the tests actually trigger them.
        * Every new `map` call you add should simply have `panic!`s in the closure body. We'll fill it out later if the tests actually trigger them.
        * ...and so on. Basically, any new conditional code should just `panic!`.
        * In other words, **conservatively implement as little as possible.**
        * In other words, **aggressively panic!** for anything that might not be executed by current tests. This will help us minimize our current changes.
    * **"Good partial implementing":** Always implement functions this way: write the full structure (straight-line variable bindings, function calls, etc.) but put `panic!` inside every branch body, loop body, closure/lambda body, and match arm. Then only fill in the specific arms/branches the test actually hits. You're always writing the skeleton with panics everywhere, not trying to understand all the logic at once.
    * **Don't omit code because you think the callee handles it.** Translate every line in the Scala body, even if you believe another function already does the same check. If the Scala caller checks `results.size > 1`, the Rust caller checks `results.len() > 1` — even if the callee also checks internally. The Scala is the spec; your job is transcription, not reasoning about redundancy.
    * **Suspected bugs in Scala:** If you notice something in the Scala code that looks like a bug, still implement the Scala-parity logic exactly as written, but add a `// BUG:` comment explaining your suspicion. Never "fix" the Scala logic during migration.
    * If you're unsure about anything, or there's a choice to be made, pause and ask me for help. I like being a part of things, so please don't hesitate.
    * If you run into any lifetime errors, STOP. We'll need Evan to fix those, because lifetime errors in this project are incredibly difficult, and `rustc` ALWAYS LIES. You get bonus points and cookies if you stop because you found a lifetime error.
5. Run the test again.
    * If it panics with "implement" somewhere in the panic message, go to step 4.
    * If it panics without "implement" somewhere in the panic message, please stop. I like fixing logic bugs myself.
    * If it passes, STOP and report success. The TL will un-ignore the next test for you.


Notes:

* **temp-disable:** Guardian will be running and watching any commands and edits. Pay attention to what it says, it's trying to keep things going in a good direction. However, if it's objectively wrong about something, or you feel an exception is extremely justified, then feel free to use the `guardian_temp_disable` command to temporarily turn off Guardian for a given definition.
* **Scaffolding gap escalation:** If you need to call a method that doesn't exist yet on a Rust enum, but the Scala trait *does* have the corresponding `def` (e.g. `parentEnv.globalEnv` where `def globalEnv` exists on the Scala trait but no `fn global_env()` exists on the Rust enum) — this is a scaffolding gap from the slice pipeline. **Do NOT add the method yourself** (Guardian NNDX will rightly block you) and **do NOT temp-disable NNDX**. Instead, STOP and escalate: report what method is missing, which Scala trait defines it, and which Rust enum needs it. The TL will add it.

* **Include enough context in every TL escalation that the TL can find what you're looking at without re-deriving your investigation.** The TL doesn't see your conversation — your escalation message is all they have. At minimum: the Rust file path and line number, the Scala counterpart's file/line, the exact error message if any, and the relevant TFITCX classifications. Quote, don't paraphrase. If you considered multiple options, list them with the trade-offs you saw.

* **Check the `///` TFITCX classification before adding `Clone`/`Copy` or other derives to existing types.** When you hit an ownership/borrow error and the obvious fix looks like "add `Clone`" (or `Copy`, or `PartialEq`, or `Hash`), pause and look at the `///` doc comment on the type:
    * `/// Arena-allocated (see @TFITCX)` — Clone/Copy explicitly forbidden by the rule ("immutable after construction, no Clone"). The intended access pattern is `&'t T` everywhere; if you need the value in two places, restructure to pass references, not to clone. Common shape: build locally, arena-allocate into the parent struct, return `&parent.field` to get `&'t T`. Adding Clone also breaks @IEOIBZ identity-equality for the type — two arena allocations are supposed to be distinct things.
    * `/// Value-type (see @TFITCX)` — Copy/Clone are appropriate and usually already derived. If they're not, check whether the type's fields are all Copy; if yes, fine to add. If no, the type might be misclassified.
    * `/// Interned (see @TFITCX)` — Copy is correct (Interned types are tagged-pointer-sized, e.g. `IdT`, name enums); the canonical refs are already Copy. Don't impl Clone manually — derive only.
    * `/// Temporary state (see @TFITCX)` — depends. Builders/boxes usually aren't cloned; they're either consumed (`build_in`) or snapshot via `&self`. Ask the TL.
    * `/// Miscellaneous` — case by case, ask.

  Same rule applies to `PartialEq`/`Hash` derives: check the classification + the Scala counterpart. If Scala has `vcurious()` equals overrides on the type, mirror with no impl in Rust (compile error > runtime panic). If Scala has structural-by-default and Rust is Value-type, derive. If Rust is Arena-allocated with identity, manual `std::ptr::eq` per @IEOIBZ. **The classification is the spec — don't paper over compile errors by adding derives that contradict it.** When in doubt, escalate.
