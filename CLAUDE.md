# Vale

**Never commit unless the architect says the literal phrase "fire commit" — no other phrasing ("just commit", "go ahead", "ship it", etc.) authorizes a commit.**

This is the Vale compiler. The `FrontendRust/` tree is a Rust compiler frontend.

> Migration-era content (project overview, key directories, lifetime model, migration philosophy, slice/migration subagents, Scala→Rust notes) has moved to `migrate-tl.md`. Read it when working on anything tied to the Scala→Rust migration tail.

## Build & Test

Always run **`cargo build --lib`** after making changes. The project builds as a library. Use **`cargo check`** for faster iteration.

Eliminate all compiler warnings (unused imports, unused variables, dead code) before saying you're done. Variables prefixed with `_` are intentionally unused and don't count.

## Agent Rules

**Never use spawned agents (the Agent tool) to make code modifications.** All edits must be made directly by the main conversation using Read/Edit/Write tools. Spawned agents may only be used for **read-only tasks**: searching, exploring, analyzing, reading files, running read-only commands. The only exception is agents defined in `.claude/agents/` which are explicitly human-written and approved for modifications.

**When spawning any agent**, the prompt must include clear instructions that the agent **must not modify any files in this project** — only the main conversation and the human are allowed to do that. Agents are free to create and read/write temporary files in `/tmp` for their own use.

## Bulk Edits

See the `scripting` skill (`docs/skills/scripting.md`) for the full bulk-edit / `safe-script-runner` protocol. Quick rule: `sed`/`perl -pi` outlawed, prefer the Edit tool up to ~40 invocations, `safe-script-runner` for Python transforms beyond that, never parallelize bulk edits.

## Build & Run Convention

Always pipe `cargo run`, `cargo test`, `cargo build`, `cargo check`, and all `sbt` output into a fixed file in `./tmp/` (use the same file for the entire session/project, e.g. `./tmp/refactor-project.txt`). Come up with a name instead of refactor-project.txt, and then use the same file for the rest of the session.

**Never chain a heavy command with `| tail`, `| head`, `| grep`.** Run the build/test with `>` as one command (redirecting fully to the file) and the inspection as a separate follow-up command. Chaining defeats the purpose: you lose the ability to re-analyze a different part of the output without re-running the expensive build.

DO have them in separate commands:

```bash
cargo run --bin benchmark -- --model openai/gpt-oss-20b > ./tmp/fixing-bug-1047-quest.txt 2>&1
tail -20 ./tmp/fixing-bug-1047-quest.txt
# Later, to see a different part:
head -40 ./tmp/fixing-bug-1047-quest.txt
grep "error" ./tmp/fixing-bug-1047-quest.txt
```

```bash
sbt 'testOnly dev.vale.AfterRegionsIntegrationTests' > ./tmp/fixing-borrowing-test.txt 2>&1
grep "SUCCESS" ./tmp/fixing-borrowing-test.txt
# Later, to see a different part:
tail -30 ./tmp/fixing-borrowing-test.txt
# Later, do some changes to the code, and then same command into same file:
sbt 'testOnly dev.vale.AfterRegionsIntegrationTests' > ./tmp/fixing-borrowing-test.txt 2>&1
grep "SUCCESS" ./tmp/fixing-borrowing-test.txt
```

DON'T chain them together like this:

```bash
# This is bad:
cargo build --lib > ./tmp/build4.txt && grep -B2 "i_env_entry" ./tmp/build4.txt | grep "src/" | head -20
```

Instead, they must be separate entire commands.

DON'T use a different file for each build like this:

```bash
sbt 'testOnly dev.vale.AfterRegionsIntegrationTests' > ./tmp/borrowing-build1.txt 2>&1
grep "SUCCESS" ./tmp/borrowing-build1.txt
# BAD: Don't use a different file
sbt 'testOnly dev.vale.AfterRegionsIntegrationTests' > ./tmp/borrowing-build2.txt 2>&1
grep "SUCCESS" ./tmp/borrowing-build2.txt
```

Instead, use the same file.

## SEE ALSO (auto)

- **Read when testing, calibrating, or deploying a new Guardian shield end-to-end.** → Luz/arcana/BringingInAShield-BIASZ.md
- **Read when designing test strategy, deciding where to draw the testing boundary, or structuring a new program's entry point.** → Luz/arcana/DarkBoxAPI-DBAPIZ.md
- **Read when creating a reqwest HTTP client, or debugging 'Too many open files' errors in a reqwest-using service.** → Luz/arcana/DontMakeNewReqwestClientPerRequest-DMNRCPRZ.md
- **Read when tempted to use anyhow::Error, stringly-typed errors, or loose types in public APIs.** → Luz/arcana/GoFurtherOnTheStaticTypingSpectrum-GFSTSZ.md
- **Read when coordinating concurrent access to a limited resource across threads (semaphores, mutexes, RAII guards).** → Luz/arcana/RaiiOverSemaphoresForMovedResources-ROSFMRZ.md
- **Read when designing test strategy or deciding whether to expose internals for testing.** → Luz/arcana/TestingArchitecture-TAZ.md
- **Read when writing tests that touch shared state, temp dirs, or global state.** → Luz/arcana/TestsMustBeFullyIsolated-TMBFIZ.md
- **Read when writing tests that use if matches!(...).** → Luz/shields/AvoidIfMatchesInTestsIfPossible-AIMITIPX.md
- **Read when doing file I/O or handling paths.** → Luz/shields/BaseDirPathDiscipline-BDPDX.md
- **Read when composing a bulk-edit apply command (`cp` to `./tmp/backup/` then `mv` from `./tmp/working/`) or designing the surrounding workflow.** → Luz/shields/BulkEditScriptWorkflow-BESWX.md
- **Read when declaring public functions or types.** → Luz/shields/DocumentPublicAPIs-DPAPIX.md
- **Read when a test fails and you're considering loosening assertions or requirements.** → Luz/shields/DontConvenientlyChangeRequirements-DCCRX.md
- **Read when seeing compiler warnings in Rust code.** → Luz/shields/EliminateAllWarnings-EAWX.md
- **Read when defining an enum variant with non-trivial fields.** → Luz/shields/EnumsShouldntContainComplexData-ESCCDX.md
- **Read when defining an error type that propagates out of a component that owns a logger.** → Luz/shields/ErrorsMustCarryTheirLogFilePath-EMCTLFPX.md
- **Read when defining function arguments or considering default values.** → Luz/shields/ExplicitArgumentsNoOptionalOrDefaultedValues-EANODVX.md
- **Read when using a literal number in Rust code.** → Luz/shields/ExtractMagicNumbersIntoNamedConstants-EMNINCX.md
- **Read when designing error handling, propagation, or failure paths.** → Luz/shields/FailFastFailLoud-FFFLX.md
- **Read when creating a value type that gets interned.** → Luz/shields/ImmediateInterningDiscipline-IIDX.md
- **Read when writing integration tests.** → Luz/shields/IntegrationTestsBehaveLikeUsers-ITBLUX.md
- **Read when porting a Scala match expression with inline comparisons.** → Luz/shields/KeepInlineComparisonsInline-KICIX.md
- **Read when porting Scala definitions that had comments.** → Luz/shields/MigrateAllCommentsToo-MACTX.md
- **Read when using trait objects or considering Any/TypeId.** → Luz/shields/NeverDowncastTraits-NEDCX.md
- **Read when writing test code with if-statements.** → Luz/shields/NeverHaveConditionalsInTests-NHCITX.md
- **Read when handling an error — you need to both log it and propagate it, not one or the other.** → Luz/shields/NeverLoseErrorInformation-NLEIX.md
- **Read when handling an unexpected runtime condition.** → Luz/shields/NeverRecoverAlwaysFail-NRAFX.md
- **Read when writing test setup or assertion helpers.** → Luz/shields/NeverRepeatImplementationCodeInTests-NRICITX.md
- **Read when writing code that aggregates, reports, or prints errors to the user.** → Luz/shields/NeverSummarizeAwayErrorContent-NSAECX.md
- **Read when writing Rust code that uses or references 'static lifetime.** → Luz/shields/NeverUseStaticLifetime-NUSLX.md
- **Read when writing comments that might accidentally resemble Guardian directives, or when deleting lines that contain them.** → Luz/shields/NoAddingGuardianDirectives-NAGDX.md
- **Read when adding Rust code during Scala-to-Rust migration.** → Luz/shields/NoChangesWithoutScalaReference-NCWSRX.md
- **Read when writing test code that must branch on match or conditional.** → Luz/shields/NoConditionalsInTestsOneBranchProceedsAllOthersPanic-NCTOBPAOPX.md
- **Read when introducing local bindings or closures.** → Luz/shields/NoDroppedLocalVariablesOrCaptures-NDLVOCX.md
- **Read when adding #[derive(Clone)] or .clone() calls.** → Luz/shields/NoExpensiveClones-NECX.md
- **Read when considering a static, thread_local, or global singleton.** → Luz/shields/NoGlobalStateAnywhere-NGSAX.md
- **Read when editing any file inside a shields/ directory.** → Luz/shields/NoModificationsToShieldFiles-NMSFX.md
- **Read when reorganizing a file during Scala-to-Rust migration.** → Luz/shields/NoMovedDefinitions-NMDX.md
- **Read when adding a new fn, struct, trait, enum, or impl during migration.** → Luz/shields/NoNewDefinitions-NNDX.md
- **Read when porting a Scala definition's name to Rust.** → Luz/shields/NoRenamedDefinitions-NRDX.md
- **Read when tempted to use string matching in tests instead of structured types.** → Luz/shields/NoStringlyTypedData-NSTDX.md
- **Read when using println!, eprintln!, or other output macros.** → Luz/shields/OutputAndLoggingZenDiscipline-OALZDX.md
- **Read when deciding between Result and panic!.** → Luz/shields/PreferResultOverPanicForRecoverableCases-PROPRCX.md
- **Read when writing nested match expressions.** → Luz/shields/PreferSingleMatchOverNestedMatches-PSMONMX.md
- **Read when invoking a Python script that might mutate state.** → Luz/shields/PythonScriptMutation-PSMX.md
- **Read when editing Rust code that carries Scala migration comments.** → Luz/shields/ScalaCommentParity-SCPX.md
- **Read when porting a Scala function to Rust.** → Luz/shields/ScalaParityDuringMigration-SPDMX.md
- **Read when porting Scala sealed traits to Rust.** → Luz/shields/ScalaSealedTraitsToRustEnums-SSTREX.md
- **Read when inserting migration slice markers.** → Luz/shields/SliceInTheRightPlaces-SITRPX.md
- **Read when naming locals that go through multiple transformation stages.** → Luz/shields/SuffixWhenDealingWithMultipleStages-SWDWMSX.md
- **Read when writing tests that use expect() or unwrap().** → Luz/shields/TestsPreferUnwrapToExpectForConciseness-TPUTEFCX.md
- **Read when writing todo!() or unimplemented!() placeholders.** → Luz/shields/TodosAndUnimplementedCodeMustPanic-TUCMPX.md
- **Read when asserting on collection size followed by indexed access.** → Luz/shields/UseExpectFunctionsInsteadOfAssertingSizeThenIndexing-UEFIAIX.md
- **Read when writing Rust code that imports or references paths via crate::, std::, core::, alloc::, or proc_macro::, or considering placing a `use` statement inside a function/impl body.** → Luz/shields/UseUseForShortNamesNotCrateInBodies-UUSNNCBX.md
- **Read when composing a Bash command to understand which shapes are auto-allowed.** → Luz/shields/ValidateReadonlyBash-VRBX.md
- **Read when investigating a compiler bug by tracing execution with debug printouts and narrowing the call graph.** → Luz/skills/CollapsedCallTree.md
- **Read when starting a new feature, to follow the gated discuss/plan/stub/test/implement sequence.** → Luz/skills/feature-development-flow.md
- **Read when the architect says the literal phrase "fire commit" (or you're about to commit + sync as a TL).** → Luz/skills/fire-commit.md
- **Read when the architect says the literal phrase "fire rebase".** → Luz/skills/fire-rebase.md
- **Read when reviewing or critiquing a plan for testing correctness before implementation.** → Luz/skills/good-testing.md
- **Read when writing a plan that includes implementation work — every such plan needs an RFIGA list, defined here.** → Luz/skills/tdd.md
- **Read when planning or making a large change to the typing pass (FrontendRust/src/typing/).** → docs/architecture/typing-pass-ai-guide.md
- **Read when a Guardian shield just fired or failed at hook time and you need to diagnose it.** → docs/skills/guardian-diagnose.md
- **Read when promoting an LLM-mode shield to Rust mode with a deterministic companion program.** → docs/skills/guardian-rustify.md
- **Read when authoring or running any bulk-edit script (`./tmp/scripts/*.py`, shell loops over many files, or any per-file transform across more than a handful of files).** → docs/skills/scripting.md
