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
3. Run the non-ignored tests: `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml > ./tmp/slab15-tests.txt 2>&1`. Most tests have `#[ignore]` — only the currently-active test(s) will run. Do NOT use `-E` to filter to a specific test — run all non-ignored tests so you catch regressions in previously-passing tests. If the active test panics with "implement", proceed to step 4. If it passes, pick the next simplest-looking ignored test, un-ignore it, write its Rust test body (using the Scala comment as a guide), and start driving it through the same loop.
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
    * If it passes, pick the next simplest-looking ignored test, un-ignore it, write its Rust test body (using the Scala comment as a guide), and start driving it through the same loop.


Notes:

* **temp-disable:** Guardian will be running and watching any commands and edits. Pay attention to what it says, it's trying to keep things going in a good direction. However, if it's objectively wrong about something, or you feel an exception is extremely justified, then feel free to use the `guardian_temp_disable` command to temporarily turn off Guardian for a given definition.
* **Scaffolding gap escalation:** If you need to call a method that doesn't exist yet on a Rust enum, but the Scala trait *does* have the corresponding `def` (e.g. `parentEnv.globalEnv` where `def globalEnv` exists on the Scala trait but no `fn global_env()` exists on the Rust enum) — this is a scaffolding gap from the slice pipeline. **Do NOT add the method yourself** (Guardian NNDX will rightly block you) and **do NOT temp-disable NNDX**. Instead, STOP and escalate: report what method is missing, which Scala trait defines it, and which Rust enum needs it. The TL will add it.

* **Include enough context in every TL escalation that the TL can find what you're looking at without re-deriving your investigation.** The TL doesn't see your conversation — your escalation message is all they have. At minimum: the Rust file path and line number, the Scala counterpart's file/line, the exact error message if any, and the relevant TFITCX classifications. Quote, don't paraphrase. If you considered multiple options, list them with the trade-offs you saw.

* **Before escalating "X doesn't exist," grep for it.** Rust names often diverge from Scala (operators like `def +` become `fn add`, etc.). Scan the type's `impl` blocks and the audit-trail `/* ... */` for a renamed counterpart before declaring something missing.

* **Cite Scala paths, not Rust audit-trail lines.** When pointing the TL at a Scala counterpart, cite the actual `Frontend/.../Foo.scala:line`, not the Rust file's quoted comment. Saves the TL a hop.

* **SPDMX vs skeleton-with-panics: escalate, don't temp-disable.** When you're porting a Scala function whose body is a `.map.groupBy.mapValues` / `.foreach` chain and you write a Scala-shaped iteration skeleton with `panic!` in the closure bodies, SPDMX may flag it as "novel scaffolding" or recommend `panic!()` for the whole function. **Both are wrong** — the skeleton IS the Scala parity (per TL.md "Good Partial Implementing"), and whole-function `panic!` breaks the test. The resolution is a TL temp-disable with a standard rationale, but the temp-disable is **TL/architect-level only**. Don't temp-disable SPDMX yourself; STOP and escalate, naming the function you're porting and quoting the Scala body (`.map.groupBy.mapValues` / `.foreach` chain). The TL will issue the temp-disable.

* **Guardian flags a pre-existing parity issue: fix it if easy, pause if not.** When Guardian rejects an edit because of a parity violation that predates your change (e.g. you're threading a parameter through a function and SPDMX flags a match-arm collapse that was already there), don't reflexively reach for a temp-disable. First ask: "is the underlying parity gap easy to close right here?" If it's a small adjustment (split one match arm into two with a `panic!` in the new arm, restore a `_ => {}` to the Scala-listed variant, add a missing `assert!` line, etc.), just fix it as part of your edit — the temp-disable is a worse outcome than a 5-line parity fix. If closing the gap would need deeper surgery (call-graph changes, new helpers, restructuring beyond the local function, or anything that needs a judgment call about how Scala's logic should land in Rust), **pause and ask the user**. Don't issue a temp-disable as the default escape hatch when the underlying complaint is legitimate.

* **Adding an `interner` parameter is always a fine Rust adaptation.** When Scala parity wants something that needs the typing interner (e.g. materializing a `&'t T` snapshot, allocating a slice, re-interning a name) and Scala didn't take an interner because it used GC + mutable references, just thread `interner: &TypingInterner<'s, 't>` through the Rust signature. Don't escalate for this shape — it's JR-level work, doesn't trip Guardian (signature shape change on an existing definition is fine), and is the documented Rust adaptation pattern. Add a comment above the fn explaining why:
    ```rust
    // Rust adaptation (SPDMX-B): interner threaded because <reason — typically:
    // arena-allocation of a result that Scala mutated in place, or re-allocation
    // of a slice that Scala grew via GC>.
    ```
    Examples already in the codebase: `CompilerOutputs::add_function(signature, ...)`, `NodeEnvironmentBox::nearest_block_env(interner)`, `NodeEnvironmentBox::add_entry(interner, ...)`. If you're unsure whether the interner-add is the right shape (vs some other adaptation), escalate — but the default answer is yes.

* **Interner macro wrappers return struct references, not enum variants.** Methods like `intern_typing_pass_block_result_var_name` return `&'t StructType`, not `IVarNameT` — use `.into()` or the From impl to convert to the final enum variant needed.

* **Understand the full type wrapper hierarchy before implementing.** When building values like `ReferenceLocalVariableT`, trace the full path to the final enum type (e.g., `ReferenceLocalVariableT` → `ILocalVariableT::Reference` → `IVariableT::ReferenceLocal`) and build from innermost out to avoid multiple rounds of type errors.

* **You do as many changes as possible. The TL only does Guardian-blocked changes.** If Guardian doesn't fire, you don't need to escalate. Threading a new parameter through call sites, renaming a local to match Scala, fixing an obvious lifetime annotation, adding a `&'t self` receiver where the body needs it — all yours. Escalate only when Guardian fires on something legitimate (NNDX on a missing definition, SPDMX on a Scala-shaped skeleton, etc.). This means more responsibility on you, but faster iteration overall.

* **Check the `///` TFITCX classification before adding `Clone`/`Copy` or other derives to existing types.** When you hit an ownership/borrow error and the obvious fix looks like "add `Clone`" (or `Copy`, or `PartialEq`, or `Hash`), pause and look at the `///` doc comment on the type:
    * `/// Arena-allocated (see @TFITCX)` — Clone/Copy explicitly forbidden by the rule ("immutable after construction, no Clone"). The intended access pattern is `&'t T` everywhere; if you need the value in two places, restructure to pass references, not to clone. Common shape: build locally, arena-allocate into the parent struct, return `&parent.field` to get `&'t T`. Adding Clone also breaks @IEOIBZ identity-equality for the type — two arena allocations are supposed to be distinct things.
    * `/// Value-type (see @TFITCX)` — Copy/Clone are appropriate and usually already derived. If they're not, check whether the type's fields are all Copy; if yes, fine to add. If no, the type might be misclassified.
    * `/// Interned (see @TFITCX)` — Copy is correct (Interned types are tagged-pointer-sized, e.g. `IdT`, name enums); the canonical refs are already Copy. Don't impl Clone manually — derive only.
    * `/// Temporary state (see @TFITCX)` — depends. Builders/boxes usually aren't cloned; they're either consumed (`build_in`) or snapshot via `&self`. Ask the TL.
    * `/// Miscellaneous` — case by case, ask.

  Same rule applies to `PartialEq`/`Hash` derives: check the classification + the Scala counterpart. If Scala has `vcurious()` equals overrides on the type, mirror with no impl in Rust (compile error > runtime panic). If Scala has structural-by-default and Rust is Value-type, derive. If Rust is Arena-allocated with identity, manual `std::ptr::eq` per @IEOIBZ. **The classification is the spec — don't paper over compile errors by adding derives that contradict it.** When in doubt, escalate.
