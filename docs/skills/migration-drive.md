---
name: migration-drive
description: Iteratively replace panics in a Scala-to-Rust migration with minimal, iterative parity-only changes with no novel logic, adding panic/assert placeholders until compile/test paths are implemented.
---

> **Every time you compact, re-read this file** (`docs/skills/migration-drive.md`). It changes often as the TL adds notes about new gotchas, escalation patterns, and migration rules learned during the session. Compaction drops the prior conversation but not the file — if you re-read it, you pick up everything the previous instance learned. If you don't, you'll repeat mistakes that have already been documented.

**This skill is pass-agnostic.** It drives the panic-replacement migration loop for *any* pass (typing, instantiating, simplifying, …). Many examples in the Notes below are typing-pass-flavored — treat them as illustrations of pass-agnostic principles, not typing-only rules.

**Required: a `migration-drive-todo.md` at the repo root.** It lists the targets to drive (tests or definitions) as a checklist — `- [ ]` not-yet-done, `- [x]` done. The loop below marks items in it and picks the next `- [ ]`. If `migration-drive-todo.md` doesn't exist, stop and ask the architect to create it (or to name the targets to drive) before starting.

Here's what I want you to do:

1. First, look at these files in full. Do not skip any. Read each one in full. You will need to adhere to all of these.
    * FrontendRust/docs/migration/migration-policy.md
    * FrontendRust/docs/usage/test-helpers.md
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
    * **One-shot rule on lifetime fixes:** you get one attempt. If your first fix doesn't compile cleanly, stop and escalate immediately — don't iterate, don't try a second variant, even if you're confident you're close. Lifetime puzzles in this codebase fool rustc and they fool you; a "looks right" second fix usually compounds the original problem rather than solving it.
3. Run the non-ignored tests: `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml > ./tmp/migration-drive-tests.txt 2>&1`. Most tests have `#[ignore]` — only the currently-active test(s) will run. Do NOT use `-E` to filter to a specific test — run all non-ignored tests so you catch regressions in previously-passing tests. If the active test panics with "unimplemented"/"implement"/"not yet migrated", proceed to step 4. If it passes, mark the test done in `migration-drive-todo.md` (change its `- [ ]` to `- [x]`), then pick the next `- [ ]` test in that file, un-ignore it in the test file, write its Rust test body (using the Scala comment as a guide), and start driving it through the same loop.
4. Please replace that panic with a very *incremental* bit of logic to get *closer* to the equivalent of the old Scala logic. IMPORTANT:
    * DON'T IMPLEMENT ANYTHING ELSE. Just do the one step it gives you.
    * DO NOT ADD ANY novel logic! All the Rust functions you need should already exist somewhere. NO adding new functions. You will only be modifying existing functions.
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
    * **DO** feel free to call unimplemented functions. Feel free to call functions that currently have panics. That is often the correct call. If we hit that panic, we'll just migrate that next.
    * **Don't omit code because you think the callee handles it.** Translate every line in the Scala body, even if you believe another function already does the same check. If the Scala caller checks `results.size > 1`, the Rust caller checks `results.len() > 1` — even if the callee also checks internally. The Scala is the spec; your job is transcription, not reasoning about redundancy.
    * **Suspected bugs in Scala:** If you notice something in the Scala code that looks like a bug, still implement the Scala-parity logic exactly as written, but add a `// BUG:` comment explaining your suspicion. Never "fix" the Scala logic during migration.
    * If you're unsure about anything, or there's a choice to be made, pause and ask me for help. I like being a part of things, so please don't hesitate.
    * If you run into any lifetime errors, STOP. We'll need the TL to fix those, because lifetime errors in this project are incredibly difficult, and `rustc` ALWAYS LIES. You get bonus points and cookies if you stop because you found a lifetime error.
5. Run the test again.
    * If it panics with "unimplemented"/"implement"/"not yet migrated" somewhere in the panic message, go to step 4.
    * If it panics without "unimplemented"/"implement"/"not yet migrated" somewhere in the panic message, please stop, because that's a logic bug, and that's outside your task. Someone else is here to fix logic bugs.
    * If it passes, mark the test done in `migration-drive-todo.md` (change its `- [ ]` to `- [x]`), then pick the next `- [ ]` test in that file, un-ignore it in the test file, write its Rust test body (using the Scala comment as a guide), and start driving it through the same loop. **Do not start on a new test until all currently-active tests are passing** — run the full suite first and confirm no regressions before un-ignoring anything new.


Notes:

* **Write escalations to `for-tl.md`.** Whenever you escalate to the TL (scaffolding gap, SPDMX skeleton, lifetime error, choice between alternatives, anything in the bullets below that says "stop and escalate"), write the escalation into `for-tl.md` at the repo root rather than only saying it in chat. Append a new section with a timestamp/heading, then include all the context the TL needs (file paths, line numbers, Scala counterpart, error message, options you considered). The TL reads `for-tl.md` to pick up escalations between turns. You can still mention in chat that you escalated, but `for-tl.md` is the source of truth — chat history is ephemeral, the file is not. **When you escalate, stop — don't keep driving the test in parallel; wait for the TL response in `for-jr.md`. Never defer or skip a test to move on to another one; if you can't solve it, escalate and wait.**

  **After writing `for-tl.md`, kick off a background watcher so you're notified the moment the TL responds, then yield the turn:**
  ```bash
  until [ -f for-jr.md ]; do sleep 10; done; echo "for-jr.md appeared at $(date)"
  ```
  Run that with `run_in_background: true` — completion notifies you, so you don't poll or sit blocked. When it fires, **consume `for-jr.md` in one shot via the exact command `cat for-jr.md && rm for-jr.md && echo done`** — Guardian has that literal command on the readonly-bash allowlist so it runs without a permission prompt; any variant (separating `cat` and `rm`, piping through `head`, dropping `echo done`) drops out of the allowlist and forces a prompt. `cat` lands the TL's instructions in your context; `rm` consumes the file the same turn so the next TL response lands in a clean file. Then apply the instructions and continue driving.

* **When the architect says just "z," check for `for-jr.md` at the repo root; if it exists, consume it via `cat for-jr.md && rm for-jr.md && echo done` (the Guardian-allowlisted shape — see above), apply the TL's instructions, and continue driving.**

* **Don't debug bugs yourself — escalate to TL; Guardian blocks novel debug statements, temp-disable isn't a workaround, and we can't remove temp-disables.**

* **temp-disable:** Guardian will be running and watching any commands and edits. Pay attention to what it says, it's trying to keep things going in a good direction. However, if it's objectively wrong about something, or you feel an exception is extremely justified, then feel free to use the `guardian_temp_disable` command to temporarily turn off Guardian for a given definition.

* **Scaffolding gap escalation:** If you need to call a method that doesn't exist yet on a Rust enum, but the Scala trait *does* have the corresponding `def` (e.g. `parentEnv.globalEnv` where `def globalEnv` exists on the Scala trait but no `fn global_env()` exists on the Rust enum) — this is a scaffolding gap from the slice pipeline. **Do NOT add the method yourself** (Guardian NNDX will rightly block you) and **do NOT temp-disable NNDX**. Instead, STOP and escalate: report what method is missing, which Scala trait defines it, and which Rust enum needs it. The TL will add it.

* **No inlining:** Guardian will be running and watching any commands and edits, and make sure that your Rust code has 1:1 parity with the old Scala code. When Guardian rejects your code, **DO NOT INLINE** methods as a workaround! Either do what Guardian says, or temp-disable it if it's wrong, or escalate to the TL.

* **No simplifications.** 1:1 Scala parity outranks tidiness and blast radius. Never omit dead bindings, inline helpers, skip checks, or substitute idioms without the architect's explicit go-ahead.

* **Include enough context in every TL escalation that the TL can find what you're looking at without re-deriving your investigation.** The TL doesn't see your conversation — your escalation message is all they have. At minimum: the Rust file path and line number, the Scala counterpart's file/line, the exact error message if any, and the relevant TFITCX classifications. Quote, don't paraphrase. If you considered multiple options, list them with the trade-offs you saw.

* **Before escalating "X doesn't exist," grep for it.** Rust names often diverge from Scala (operators like `def +` become `fn add`, `object simpleNameT.unapply` becomes `fn unapply_simple_name`, etc.). Scan the type's `impl` blocks, sibling modules (e.g. `templata_utils.rs` for `TemplataUtils.scala`), and the audit-trail `/* ... */` for a renamed counterpart before declaring something missing.

* Before escalating an `as_foo_name()` helper, grep for `TryFrom<INameT> for IFooT` — the documented `+T` erasure pattern uses `.try_into().unwrap()`.

* **Don't escalate to add a polymorphic method on `INameT`.** When porting `id.localName.fooBar` where Scala's `id` is statically `IdT[ISubTraitNameT]`, the Rust port is `ISubTraitNameT::try_from(id.local_name).unwrap().foo_bar()` — narrow at the use site (§6.0 +T erasure), then call the per-variant method that already exists on the sub-enum. The wide `INameT` doesn't get the method; check the Scala static type of `id` to find the right sub-trait.

* **When citing a Scala method on a sealed trait, check parent traits too.** `sealed trait ISubKindTT extends KindT` inherits every `def` on `KindT` — the Scala source for an "ISubKindTT method" may actually live on `KindT`. Cite the parent in the escalation so the TL can find it.

* **Cite Scala paths, not Rust audit-trail lines.** When pointing the TL at a Scala counterpart, cite the actual `Frontend/.../Foo.scala:line`, not the Rust file's quoted comment. Saves the TL a hop.

* **Missing `import foo.*` in tests: chain the existing resolvers, don't invent helpers.** Three-link chain: `crate::builtins::builtins::get_embedded_modulized_code_map(...)` serves `v.builtins.*` (opt, weak, logic, drop, arith, …); `code_hierarchy::test_from_vec(...)` serves the inline test source; `crate::tests::tests::get_package_to_resource_resolver()` serves on-disk test packages under `FrontendRust/src/tests/` (`printutils`, `panicutils`, `castutils`, `listprintutils`, `genericvirtuals/*`, etc.). `.or(...)` them in that order. If a test errors with `Couldn't find: PackageCoordinate { module: "<name>" }`, the answer is almost always "you didn't include the third resolver" — it is *not* an infrastructure gap, do not escalate to add `<name>` to builtins. Mirror the resolver chain in `tests_destructuring_shared_doesnt_compile_to_destroy` (`compiler_tests.rs:~4152`).

* **Test patterns: no `if`-guards inside `matches!`. Mirror Scala's two styles directly.** Scala `expr shouldEqual fullyConstructed` ports to `assert_eq!(expr, fully_constructed)` — never `matches!`. Scala `expr match { case Pat(x) => vassert(x.foo) }` ports to a real `match` block with `assert!`s inside the arm — never `matches!(... if x.foo())`. The `if`-guard form has no Scala counterpart and is always a sign the destructuring was shallowed out or that a `shouldEqual` was mis-ported to a `matches!`. If a check needs a method call (like `is_test()`), put it as a bare `assert!` inside the matching arm, exactly like Scala's `vassert` after the `case`. If a literal can be destructured (e.g. `StrI("MyInterface")` matches `pub struct StrI<'s>(pub &'s str)`), inline it in the pattern — don't pull it out into an `if x.field == "..."` guard. Same rule for SPDMX false-positives: don't temp-disable; deepen the destructure or switch to `assert_eq!` with a fully-interned expected value.

* **Scaffold typo `&'t [TE<'s,'t>]` → `&'t [&'t TE<'s,'t>]` on AST nodes is JR-fixable per SPDMX exception B.**

* **SPDMX on an in-file precedent pattern: temp-disable yourself.** When SPDMX flags a Rust adaptation pattern that already has a precedent temp-disable in the same file (grep nearby `/*` blocks for `Guardian: temp-disable: SPDMX`), issue the temp-disable via `mcp__guardian__guardian_temp_disable` with a rationale that cites the in-file precedent (e.g. "this `intern_package_coordinate(empty_string, &[])` pattern is the Rust equivalent of `PackageCoordinate.BUILTIN(interner, keywords)`, established in `compile_runtime_sized_array` / `resolve_runtime_sized_array` in this same file"). No escalation needed.

* **SPDMX vs skeleton-with-panics: escalate, don't temp-disable.** When you're porting a Scala function whose body is a `.map.groupBy.mapValues` / `.foreach` chain and you write a Scala-shaped iteration skeleton with `panic!` in the closure bodies, SPDMX may flag it as "novel scaffolding" or recommend `panic!()` for the whole function. **Both are wrong** — the skeleton IS the Scala parity (per TL.md "Good Partial Implementing"), and whole-function `panic!` breaks the test. The resolution is a TL temp-disable with a standard rationale, but the temp-disable is **TL/architect-level only**. Don't temp-disable SPDMX yourself; STOP and escalate, naming the function you're porting and quoting the Scala body (`.map.groupBy.mapValues` / `.foreach` chain). The TL will issue the temp-disable.

* **Fix pre-existing 1:1 parity issues you hit; escalate only egregious ones.** When Guardian flags (or you notice) a parity violation that predates your change, the default is to fix it to match Scala 1:1 — even if it's not your code, even a few lines beyond your edit. Mirror an in-file/sibling precedent of the correct form. Only escalate (write `for-tl.md`, stop) when the fix is too egregious to do safely: deep call-graph surgery, unverifiable behavior change, or spanning many definitions. Temp-disable is not the escape hatch — it's only for objectively-wrong verdicts on already-faithful code.

* **Struct/enum field edits are JR territory.** Changing a field's type, adding a `&` to make it a ref, fixing a scaffold-typo `&'i [T]` → `&'i [&'i T]` (Scala `Vector[T]` is implicitly by-ref; `&'i [T]` of a non-Copy arena def is a typo when the source map already holds `&'i T`), or adding a field the Scala class has but the slice missed — all keep the definition's identity and are yours to land. Guardian shouldn't fire; if NCWSRX does, it's almost always the slicing (Scala audit block sits outside the diff window — relocate it under the def, don't temp-disable). **Caution:** when relocating a `/* scala */` audit block to sit under a def, never include a sibling Rust def in your `old_string` — Guardian only sees the diff, so a multi-def `old_string` whose `new_string` drops one def reads as a clean comment-relocation and silently deletes it. Move only the comment block, or do it as two Edits.

* **1:1 parity outranks blast radius — do the retrofit, don't simplify.** When parity needs an upstream change (retrofit a `&self` method to an associated fn, widen a field type, add a missing field, thread an interner), do the retrofit and update call sites — that's the cost of parity, not a reason to omit. Never omit a live Scala `val` whose only consumer is commented out; keep it `_`-prefixed and per-site-temp-disable TUCMPX. Never substitute an idiom or skip a check to dodge a wider edit. The only thing that justifies stopping is an *egregious* retrofit (deep call-graph surgery, unverifiable behavior change, many definitions) — then escalate.

* **JR may thread `Result<_, ICompileErrorT>` through any chain of panic-stubbed fns themselves under SPDMX Exception I — no escalation needed.**

* **When a Scala fn both throws `CompileErrorExceptionT` and returns `Result[_, SomeLocalError]`, mirror it in Rust as nested `Result<Result<_, SomeLocalError>, ICompileErrorT>` — outer is the always-propagate exception channel, inner is the caller-decides business channel; never merge them.**

* **Adding an `interner` parameter is always a fine Rust adaptation.** When Scala parity wants something that needs the pass's interner (e.g. materializing a `&'t T` snapshot, allocating a slice, re-interning a name) and Scala didn't take an interner because it used GC + mutable references, just thread the pass's interner (e.g. `&TypingInterner<'s, 't>`, `&InstantiatingInterner<'s, 'i>`) through the Rust signature. Don't escalate for this, it won't trip Guardian.

* **Interner macro wrappers return struct references, not enum variants.** Methods like `intern_typing_pass_block_result_var_name` return `&'t StructType`, not `IVarNameT` — use `.into()` or the From impl to convert to the final enum variant needed.

* When building values like `ReferenceLocalVariableT` that implement traits in Scala, those will be enums in Rust. Trace the full path to the final enum type (e.g., `ReferenceLocalVariableT` → `ILocalVariableT::Reference` → `IVariableT::ReferenceLocal`) and build from innermost out to avoid multiple rounds of type errors.

* **The TL only does Guardian-blocked changes.** If Guardian doesn't fire, you don't need to escalate. Threading a new interner/keywords/etc. parameter through call sites, renaming a local to match Scala, fixing an obvious lifetime annotation, adding a `&'t self` receiver where the body needs it — all yours. Escalate only when Guardian prevents you from getting closer to Scala parity.

* **Check the `///` TFITCX classification before adding `Clone`/`Copy` or other derives to existing types.** When you hit an ownership/borrow error and the obvious fix looks like "add `Clone`" (or `Copy`, or `PartialEq`, or `Hash`), pause and look at the `///` doc comment on the type:
    * `/// Arena-allocated (see @TFITCX)` — Clone/Copy explicitly forbidden by the rule ("immutable after construction, no Clone"). The intended access pattern is `&'t T` everywhere; if you need the value in two places, restructure to pass references, not to clone. Common shape: build locally, arena-allocate into the parent struct, return `&parent.field` to get `&'t T`. Adding Clone also breaks @IEOIBZ identity-equality for the type — two arena allocations are supposed to be distinct things.
    * `/// Value-type (see @TFITCX)` — Copy/Clone are appropriate and usually already derived. If they're not, check whether the type's fields are all Copy; if yes, fine to add. If no, the type might be misclassified.
    * `/// Interned (see @TFITCX)` — Copy is correct (Interned types are tagged-pointer-sized, e.g. `IdT`, name enums); the canonical refs are already Copy. Don't impl Clone manually — derive only.
    * `/// Temporary state (see @TFITCX)` — depends. Builders/boxes usually aren't cloned; they're either consumed (`build_in`) or snapshot via `&self`. Ask the TL.
    * `/// Miscellaneous` — case by case, ask.

  Same rule applies to `PartialEq`/`Hash` derives: check the classification + the Scala counterpart. If Scala has `vcurious()` equals overrides on the type, mirror with no impl in Rust (compile error > runtime panic). If Scala has structural-by-default and Rust is Value-type, derive. If Rust is Arena-allocated with identity, manual `std::ptr::eq` per @IEOIBZ. **The classification is the spec — don't paper over compile errors by adding derives that contradict it.** When in doubt, escalate.

* When you need to implement a function, but it will depend on multiple other panicking functions, that's fine. Implement the function. Then run the test, and implement whatever panic you hit next.

* When replacing a panic, just write the code with your best guess of what the Scala-parity Rust code should be, inspired by the corresponding Scala. Scala-parity higher priority than correctness. Then use feedback from the compiler and Guardian to know what you need to look for to make a more informed second iteration. Your first try should be immediate, then do informed iteration. Do not extensively research before your first attempt.
