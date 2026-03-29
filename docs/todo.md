# Documentation Reorganization TODO

We are migrating all documentation to the structure defined in `docs/meta.md`. For each file below, use the `/document` skill to:

1. Read the file and understand what information/wisdom it contains
2. Split/categorize its content into the appropriate categories (background, usage, arcana, shields, architecture, reasoning, migration, skills)
3. Write the content to the correct `docs/` directory for the relevant feature
4. Add `@ID` references at code sites for any arcana
5. Delete or archive the original file once its content has been relocated

Files already in their correct location per `docs/meta.md` can be checked off without moving.

---

## docs/meta.md and docs/skills/

- [ ] `docs/meta.md` — Already in place (this is the strategy doc)
- [ ] `docs/skills/document.md` — Already in place

## Sylvan top-level

- [ ] `README.md`
- [ ] `architectural-direction.md`
- [ ] `build-compiler.md`
- [ ] `compiler-overview.md`

## Sylvan/docs/ (existing, pre-reorganization)

- [ ] `docs/Architecture.md`
- [ ] `docs/HigherTypingPass.md`
- [ ] `docs/Environments.md`
- [ ] `docs/ContextWord.md`
- [ ] `docs/PerfectReplayability.md`
- [ ] `docs/ObjectMetadata.md`
- [ ] `docs/Generics.md`
- [ ] `docs/RegionBookkeeping.md`
- [ ] `docs/virtuals/Impls.md`
- [ ] `docs/regions/RegionsLayout.md`
- [ ] `docs/regions/Transmigration.md`
- [ ] `docs/regions/Regions.md`
- [ ] `docs/old/` — Entire directory. Triage: archive, delete, or extract still-relevant content.

## .claude/CLAUDE.md

- [ ] `.claude/CLAUDE.md` — Strip inlined content that now lives in proper docs; update references.

## .claude/rules/ (.mdc files)

- [ ] `.claude/rules/early-lifetimes.mdc`
- [ ] `.claude/rules/general/frontendrust-build-test.mdc`
- [ ] `.claude/rules/general/interning-patterns.mdc`
- [ ] `.claude/rules/general/no-unsolicited-restructuring.mdc`
- [ ] `.claude/rules/general/style-guide.mdc`
- [ ] `.claude/rules/migration/frontendrust-migration-context.mdc`
- [ ] `.claude/rules/migration/solver-migration.mdc`
- [ ] `.claude/rules/parser/parser_impl_scala_rust_mapping.mdc`
- [ ] `.claude/rules/postparser/postparser-migration-guidelines.mdc`
- [ ] `.claude/rules/postparser/postparser-organization-map.mdc`
- [ ] `.claude/rules/postparser/postparser_impl_scala_rust_mapping.mdc`
- [ ] `.claude/rules/postparser/IDEPFL-postparser-interning.md`
- [ ] `.claude/rules/postparser/postparser-migration.md`

## .claude/skills/

- [ ] `.claude/skills/arcana/SKILL.md` — Fold into `docs/skills/document.md` (already done, verify and delete)
- [ ] `.claude/skills/migration-check-correct-loop/SKILL.md`
- [ ] `.claude/skills/migration-diff-review/SKILL.md`
- [ ] `.claude/skills/migration-drive/SKILL.md`
- [ ] `.claude/skills/migration-test-fixer/SKILL.md`
- [ ] `.claude/skills/migration-test-fixer-2/SKILL.md`
- [ ] `.claude/skills/process-feedback/SKILL.md`
- [ ] `.claude/skills/slice-pipeline/SKILL.md`
- [ ] `.claude/skills/slice-pipeline/EXAMPLES.md`
- [ ] `.claude/skills/slice-pipeline/TROUBLESHOOTING.md`
- [ ] `.claude/skills/vv/SKILL.md`

## .claude/agents/

- [ ] `.claude/agents/agent-check-correct-loop.md`
- [ ] `.claude/agents/migrate-diagnoser.md`
- [ ] `.claude/agents/migrate-director.md`
- [ ] `.claude/agents/migrate-scoper.md`
- [ ] `.claude/agents/migration-check-specific.md`
- [ ] `.claude/agents/migration-gate.md`
- [ ] `.claude/agents/migration-migrate.md`
- [ ] `.claude/agents/slice-orchestrator.md`
- [ ] `.claude/agents/slice-placehold.md`
- [ ] `.claude/agents/slice-reconcile-copy.md`
- [ ] `.claude/agents/slice-reconcile-delete.md`
- [ ] `.claude/agents/slice-reconcile-mark.md`
- [ ] `.claude/agents/slice-rustify.md`
- [ ] `.claude/agents/slice-start-check-supervised.md`
- [ ] `.claude/agents/slice-start-check.md`
- [ ] `.claude/agents/slice-start.md`

## FrontendRust top-level

- [ ] `FrontendRust/todo.md`
- [ ] `FrontendRust/cursor_setup.md`
- [ ] `FrontendRust/thoughts.md`
- [x] `FrontendRust/migrate-direction.md` — Deleted (stale, panic is self-tracking)
- [x] `FrontendRust/check-template.txt` — Synced with Guardian's current version
- [ ] `FrontendRust/manual/building.md`
- [ ] `FrontendRust/src/gripes.md`

## FrontendRust/docs/ (arena cluster — needs dedup)

These 5 docs heavily overlap and should be consolidated during the move:

- [x] `FrontendRust/docs/arena-lifetimes.md` — consolidated into `docs/background/arenas.md`
- [x] `FrontendRust/docs/arena-two-phase-lifecycle.md` — consolidated into `docs/usage/arenas.md` + `docs/architecture/arenas.md`
- [x] `FrontendRust/docs/arena-allocated-structs.md` — consolidated into `docs/architecture/arenas.md`
- [x] `FrontendRust/docs/per-pass-arenas-migration.md` — trimmed to `docs/migration/per-pass-arenas.md`
- [x] `FrontendRust/docs/location-in-denizen.md` — consolidated into `docs/usage/arenas.md` + `docs/architecture/arenas.md`

## FrontendRust/docs/ (other)

- [x] `FrontendRust/docs/migration-audit-process.md` — Deleted (Guardian covers batch orchestration now)
- [x] `FrontendRust/docs/migration-audit-report.md` — Deleted (violations extracted as `// Violation:` comments in code)

## FrontendRust/zen/ (becomes FrontendRust/docs/)

### Shields (have duplicates in Luz/shields/)

- [ ] `FrontendRust/zen/CloserToScalaNotFurther.md`
- [ ] `FrontendRust/zen/NoAddingScalaComments.md`
- [ ] `FrontendRust/zen/NoChangesWithoutScalaReference.md`
- [ ] `FrontendRust/zen/NoMovedDefinitions.md`
- [ ] `FrontendRust/zen/NoNewDefinitions.md`
- [ ] `FrontendRust/zen/NoNovelCodeDuringMigrations.md`
- [ ] `FrontendRust/zen/NoRenamedDefinitions.md`

### Arcana

- [ ] `FrontendRust/zen/PostParserSynthesizesParserASTNodes-PPSPASTNZ.md`
- [x] `FrontendRust/zen/ArenaAndMallocSeparation.md` — moved to `docs/shields/AASSNCMCX.md` (Sylvan-specific shield)
- [x] `FrontendRust/zen/zen.md` — Deleted (EAW and IID already in Luz/shields as EAWX and IIDX)

### Migration process/principles

- [x] `FrontendRust/zen/migration_process.md` — Moved to `docs/migration/process.md`, updated 9 agent/skill references
- [ ] `FrontendRust/zen/migration_principles.md`
- [ ] `FrontendRust/zen/migration_differences.md`
- [x] `FrontendRust/zen/migration_prompt.md` — Deleted obsolete prompt templates, kept AI lessons notes
- [ ] `FrontendRust/zen/migration_report_prompt.md`
- [ ] `FrontendRust/zen/migration_report_instructions.md`
- [ ] `FrontendRust/zen/migration-strategies.md`
- [ ] `FrontendRust/zen/testing.md`

### Architecture

- [ ] `FrontendRust/zen/typing-pass-design.md`

## FrontendRust/todo/

- [x] `FrontendRust/todo/arena-deterministic-map-problem.md` — moved to `docs/reasoning/arena-deterministic-maps.md`
- [x] `FrontendRust/todo/necx-clone-analysis.md` — Deleted (clones fixed, NECX shield covers principle)

## FrontendRust/src/ in-tree docs

- [x] `FrontendRust/src/postparsing/docs/rc-environments-plan.md` — Moved to `src/postparsing/docs/migration/rc-environments.md`, updated lifetimes to `'s`
- [x] `FrontendRust/src/higher_typing/docs/exceptions/HigherTypingPass.md` — Moved to `src/higher_typing/docs/migration/differences.md`

## Luz/shields/

These are cross-project shields and should stay in `Luz/shields/` per `docs/meta.md`. Verify each is genuinely project-agnostic; move Sylvan-specific ones to the relevant feature's `docs/shields/`.

- [ ] `Luz/shields/AllTestsAreExtremelyImportantAndShouldPass-ATEISPX.md`
- [ ] `Luz/shields/ArenaAllocatedStructsShouldNotContainMallocdCollections-AASSNCMCX.md`
- [ ] `Luz/shields/AvoidIfMatchesInTestsIfPossible-AIMITIPX.md`
- [ ] `Luz/shields/BaseDirPathDiscipline-BDPDX.md`
- [ ] `Luz/shields/CloserToScalaNotFurther-CSTNFX.md`
- [ ] `Luz/shields/DocumentPublicAPIs-DPAPIX.md`
- [ ] `Luz/shields/DontConvenientlyChangeRequirements-DCCRX.md`
- [ ] `Luz/shields/EliminateAllWarnings-EAWX.md`
- [ ] `Luz/shields/EnumsShouldntContainComplexData-ESCCDX.md`
- [ ] `Luz/shields/ExplicitArgumentsNoOptionalOrDefaultedValues-EANODVX.md`
- [ ] `Luz/shields/ExtractMagicNumbersIntoNamedConstants-EMNINCX.md`
- [ ] `Luz/shields/ExtractPromptsIntoFunctions-EPIFX.md`
- [ ] `Luz/shields/FailFastFailLoud-FFFLX.md`
- [ ] `Luz/shields/ForkInsideJoin-FIJX.md`
- [ ] `Luz/shields/GUIDE-bringing-in-a-shield.md`
- [ ] `Luz/shields/ImmediateInterningDiscipline-IIDX.md`
- [ ] `Luz/shields/IntegrationTestsBehaveLikeUsers-ITBLUX.md`
- [ ] `Luz/shields/KeepInlineComparisonsInline-KICIX.md`
- [ ] `Luz/shields/MigrateAllCommentsToo-MACTX.md`
- [ ] `Luz/shields/NeverDowncastTraits-NEDCX.md`
- [ ] `Luz/shields/NeverHaveConditionalsInTests-NHCITX.md`
- [ ] `Luz/shields/NeverRecoverAlwaysFail-NRAFX.md`
- [ ] `Luz/shields/NeverRepeatImplementationCodeInTests-NRICITX.md`
- [ ] `Luz/shields/NoAddingOrChangingScalaComments-NAOCSCX.md`
- [ ] `Luz/shields/NoChangesWithoutScalaReference-NCWSRX.md`
- [ ] `Luz/shields/NoConditionalsInTestsOneBranchProceedsAllOthersPanic-NCTOBPAOPX.md`
- [ ] `Luz/shields/NoExpensiveClones-NECX.md`
- [ ] `Luz/shields/NoGlobalStateAnywhere-NGSAX.md`
- [ ] `Luz/shields/NoMovedDefinitions-NMDX.md`
- [ ] `Luz/shields/NoNewDefinitions-NNDX.md`
- [ ] `Luz/shields/NoNovelCodeDuringMigrations-NNCDMX.md`
- [ ] `Luz/shields/NoRenamedDefinitions-NRDX.md`
- [ ] `Luz/shields/NoValidSimplifications-NVSEX.md`
- [ ] `Luz/shields/OutputAndLoggingZenDiscipline-OALZDX.md`
- [ ] `Luz/shields/PortStructureExactly-PSEX.md`
- [ ] `Luz/shields/PreferResultOverPanicForRecoverableCases-PROPRCX.md`
- [ ] `Luz/shields/PreferSingleMatchOverNestedMatches-PSMONMX.md`
- [ ] `Luz/shields/ReturningResultIsFine-RRIFX.md`
- [ ] `Luz/shields/RustShouldMirrorScalaAsCloseAsPossible-RSMSCPX.md`
- [ ] `Luz/shields/SameHelperCallsNoExceptions-SHCNEX.md`
- [ ] `Luz/shields/ScalaSealedTraitsToRustEnums-SSTREX.md`
- [ ] `Luz/shields/SliceInTheRightPlaces-SITRPX.md`
- [ ] `Luz/shields/SuffixWhenDealingWithMultipleStages-SWDWMSX.md`
- [ ] `Luz/shields/TestsPreferUnwrapToExpectForConciseness-TPUTEFCX.md`
- [ ] `Luz/shields/TodosAndUnimplementedCodeMustPanic-TUCMPX.md`
- [ ] `Luz/shields/UseCollectMacrosToRecursivelySearch-UCMTRSX.md`
- [ ] `Luz/shields/UseEnumsForFixedSetsNotStrings-UEFSNSX.md`
- [ ] `Luz/shields/UseExpectFunctionsInsteadOfAssertingSizeThenIndexing-UEFIAIX.md`
- [ ] `Luz/shields/UseUseForShortNamesNotCrateInBodies-UUSNNCBX.md`

## Luz/zen/ and Luz/skills/

- [ ] `Luz/zen/TestingArchitecture-TAZ.md`
- [ ] `Luz/zen/TestsMustBeFullyIsolated-TMBFIZ.md`
- [ ] `Luz/zen/GoFurtherOnTheStaticTypingSpectrum-GFSTSZ.md`
- [ ] `Luz/zen/RaiiOverSemaphoresForMovedResources-ROSFMRZ.md`
- [ ] `Luz/skills/FeatureDevelopmentFlow-FDFZ.md`
