# Frontend Rust - Scala to Rust Migration Project

This is a Rust compiler frontend being migrated from Scala. The project is **mid-migration** with extensive commented Scala code alongside working Rust implementations.

## Project Overview

The codebase implements a compiler frontend with parsing, post-parsing validation/transformation, and type solving. The original Scala implementation used garbage collection; the Rust version uses arena allocation with explicit lifetime management.

## Key Directories

- **`src/postparsing/`** - Post-parsing pass: validates and transforms parsed AST (actively migrating)
- **`src/solver/`** - Type solver/inference engine (actively migrating)
- **`src/interner.rs`** - String and type interning with arena-backed allocation
- **`src/postparsing/names.rs`** - Name resolution and scope management
- **`src/postparsing/function_scout.rs`** - Function signature extraction and validation
- **`src/postparsing/post_parser.rs`** - Main post-parser orchestration

## Migration Philosophy

We're doing **incremental, safe migration**. Many functions have commented-out Scala code above working (or placeholder) Rust implementations. The migration process uses systematic "slicing" to isolate and translate individual definitions.

## Lifetime Model

The Rust codebase uses **three arena lifetimes** (see `docs/background/arenas.md` for full details):

- **`'p`** - Parser arena (via `ParseArena<'p>`): interned strings, coordinates, parser AST nodes
- **`'s`** - Scout (postparser + higher_typing) arena (via `ScoutArena<'s>`): interned names, runes, imprecise names, postparser/higher-typing output nodes
- **`'ctx`** - Context/infrastructure borrows: `&'ctx ParseArena<'p>`, `&'ctx ScoutArena<'s>`, `&'ctx Keywords<'p>`

Each arena is self-contained with its own interning maps. Cross-pass data is re-interned at pass boundaries (e.g. `StrI<'p>` → `StrI<'s>` via `scout_arena.intern_str()`).

## Conventions

All rules in `.claude/rules/` are **path-targeted** and auto-load when editing relevant files. They contain:

- Scala→Rust type mappings
- Allowable differences between implementations
- Architecture and organization maps
- Style guidelines

## Migration Subagents

The codebase includes specialized subagents for systematic migration. These are autonomous agents that can be invoked using the Task tool.

### Slice Pipeline (Full Migration)
- **`slice-orchestrator`** - Orchestrates the full migration pipeline on a Rust file
- **`slice-start`** - Add `// mig:` marker comments above Scala definitions
- **`slice-rustify`** - Convert Scala-style markers to Rust-style
- **`slice-placehold`** - Generate Rust placeholder stubs
- **`slice-reconcile-mark`** - Mark old definitions as obsolete
- **`slice-reconcile-copy`** - Copy old code into stubs
- **`slice-reconcile-delete`** - Remove obsolete definitions

### Incremental Migration
- **`migration-migrate`** - Partially migrate specific Scala code sections
- **`migration-diagnoser`** - Diagnose migration issues and differences
- **`migration-check-specific`** - Check specific definitions for correctness
- **`migration-gate`** - Validate migration readiness before proceeding

### Verification
- **`agent-check-correct-loop`** - Loop-based correctness verification

All subagents are defined in `.claude/agents/` and can be invoked using the Task tool.

## Build & Test

Always run **`cargo build --lib`** after making changes. The project builds as a library.

Use **`cargo check`** for faster iteration during development.

The build may have warnings during migration - that's expected. Focus on getting it to compile first.

Eliminate all compiler warnings (unused imports, unused variables, dead code) before saying you're done. Variables prefixed with `_` are intentionally unused and don't count.

## Working with This Project

1. When editing postparser files, relevant lifetime and migration rules auto-load
2. Use the slice subagents for systematic translation of commented Scala code
3. Use migration subagents for incremental fixes and verification
4. Always verify builds succeed after changes
5. Respect the lifetime invariants - see the rules for guidance when rustc complains

## Agent Rules

**Never use spawned agents (the Agent tool) to make code modifications.** All edits must be made directly by the main conversation using Read/Edit/Write tools. Spawned agents may only be used for **read-only tasks**: searching, exploring, analyzing, reading files, running read-only commands. The only exception is agents defined in `.claude/agents/` which are explicitly human-written and approved for modifications.

**When spawning any agent**, the prompt must include clear instructions that the agent **must not modify any files in this project** — only the main conversation and the human are allowed to do that. Agents are free to create and read/write temporary files in `/tmp` for their own use.

## Notes

- **Interning:** Rust interns more aggressively than Scala. This is intentional and allowed.
- **Panics:** `panic!()` placeholders are acceptable during mid-migration. Scala's `vimpl` maps to Rust `panic!`.
- **Naming:** Rust uses `snake_case` (e.g., `self_uses`) vs Scala's `camelCase` (e.g., `selfUses`).
- **Profiling:** Rust doesn't need Scala's `Profiler.frame(() => { ... })` wrappers.


## Bulk Edits

`sed` and `perl -pi` are outlawed. For bulk transforms, **prefer the Edit tool** — ~40 distinct invocations is the threshold before Python becomes worth it.

### Bulk-edit workflow: `safe-script-runner`

The canonical path for any `./tmp/scripts/<NAME>.py` transform is the `safe-script-runner` CLI (`Luz/safe-script-runner/`). It splits the flow into three subcommands and enforces strict iteration via a single-marker invariant on disk — you cannot batch reviews, and you cannot apply cold without a prior review of the exact same content. Built binary: `Luz/safe-script-runner/target/release/safe-script-runner`.

One file at a time:

1. Write the script to `./tmp/scripts/<name>.py` (pure stdin → stdout transform; no file I/O, no subprocess, no `exec`/`eval`).
2. **Review:** `safe-script-runner review ./tmp/scripts/<name>.py <SRC>`. The tool runs the script, prints the full `=== STDERR (<SRC>) ===` and `=== DIFF (<SRC>) ===` to stdout, and writes the marker `./tmp/working/.current-review` (script + src + SHA-256 of working/stderr).
3. Iterate: edit the script and re-run step 2 on the SAME `<SRC>` — the marker refreshes in place. Switching to a different `<SRC>` while the marker is unapplied is **refused** (`ReviewPending`).
4. **Apply:** `safe-script-runner apply ./tmp/scripts/<name>.py <SRC>`. The tool re-runs the script, verifies the marker exists and its `(script, src)` and hashes match the new run, then backs up `<SRC>` to `./tmp/backup/<ts>/<src-with-dirs>` and `mv`'s working over src. Marker is deleted. If anything drifted between review and apply (src or script was edited), apply refuses with a hash-mismatch error and the JR must re-review.
5. Next file: back to step 2.

To discard a pending review without applying: `safe-script-runner abandon` (no args, no-op if no marker).

### Single-marker invariant (enforced)

Only ONE review may be pending at a time — by design. Trying to `review` a second `<SRC>` while another is unapplied is refused. This makes batch-review-then-batch-apply impossible: the only physically possible sequence is `review A → apply A → review B → apply B → …`. Combined with the marker's SHA-256 binding of review to apply, the architect's "go" is always against the diff the apply will actually land.

### Reviewing diffs

`safe-script-runner` mechanically enforces full-diff review: `review` always emits the entire diff and stderr, BESWX denies any pipe/filter/redirect/chain on the command, and the marker hash binds review to apply (drift refuses with a re-review prompt). The architect should still actually look at the diff — the tool guarantees evidence exists in the transcript, not that the human reads it.

### Legacy raw-`python3` form (discouraged)

Bare `python3 ./tmp/scripts/<NAME>.py < <SRC> > ./tmp/working/<BN> 2>./tmp/working/<BN>.stderr` followed (in any of `\n`, `;`, `&&`) by `cat ./tmp/working/<BN>.stderr` then `diff -u <SRC> ./tmp/working/<BN>` is still allowed via BESWX's strict-chain backstop. The order is fixed (cat, then diff), and the diff segment must be unfiltered (no `| head`, `| tail`, `| wc`, `| grep`, no redirect). Prefer `safe-script-runner` — only it gets marker-bound iteration enforcement; the raw form has no protection against batched reviews or basename collisions across colliding sibling files.

A failed apply via either form is recoverable from the backup at `./tmp/backup/<ts>/...`.

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
- **Read when reviewing or critiquing a plan for testing correctness before implementation.** → Luz/skills/good-testing.md
- **Read when writing a plan that includes implementation work — every such plan needs an RFIGA list, defined here.** → Luz/skills/tdd.md
- **Read when planning or making a large change to the typing pass (FrontendRust/src/typing/).** → docs/architecture/typing-pass-ai-guide.md
- **Read when a Guardian shield just fired or failed at hook time and you need to diagnose it.** → docs/skills/guardian-diagnose.md
- **Read when promoting an LLM-mode shield to Rust mode with a deterministic companion program.** → docs/skills/guardian-rustify.md
