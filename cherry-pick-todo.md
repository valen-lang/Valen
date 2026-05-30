# Cherry-pick Progress: rust-interop-reimpl → master-with-rust-interop-reimpl

Tracking sheet for the per-PR cherry-pick of `rust-interop-reimpl` onto
`master-with-rust-interop-reimpl`. Each PR is reviewed/adjusted/committed
individually so the human can verify each commit before moving on.

**Cycle:** cherry-pick `--no-commit` → run tests → wait for human LGTM → commit.

## Landed

- [x] **PR 1.1** — (landed in earlier session)
- [x] **PR 2.1** — (landed in earlier session)
- [x] **PR 2.2 + 2.3 + 2.3-followup + 4.2 squash** — `OutsideLoadSE.parts: Vector[LoadPartSE]` shape + ExpressionCompiler/OverloadResolver named-arg channel (`containerReceivingRuneToExplicitTemplateArgRune` / `containerRuneInitialKnowns`). Replaces source branch's flat-positional approach.
- [x] **PR 2.5 (postparser + higher-typing)** — Folded into the squash above. Struct internal methods (`StructMethodP → StructS.internalMethods → StructA.internalMethods`).
- [x] **`69997c21`** — PR 2.2-4.2 squash + PR 2.5 postparser/higher-typing commit
- [x] **`7101f29a`** — Fix-up: propagate `receivingRuneToExplicitTemplateArgRune` through `IStructCompilerDelegate`
- [x] **`40c43f00`** — Missing changes follow-up
- [x] **`67442241`** — **PR 2.5 typing-pass**: register struct internal methods in `structOuterEnv`
- [x] **`c894434a`** — **PR 2.6**: `@ICIPCRZ` arcana doc (rewritten to match revised PR 2.7 shape; references `@PRIIROZ` and `containerRuneInitialKnowns`)
- [x] **`a8bf0955`** — **PR 2.7 (revised)**: extern function name `templateArgs` + `GenericParametersInheritance` on `ExternFunctionCallTE`. *Dropped* the original `RustShapeProjector` approach; ValeRuster recovers the own/inherited split at callsites from `ExternFunctionCallTE.genericParameterInheritance` instead.
- [x] **`dd4adbff`** — **PR 3.1**: ValeRuster `rustdoc-types` 0.25 → 0.57.3 upgrade + `--type` CLI flag + `rust-toolchain.toml` (pulled forward from PR 3.5)
- [x] **`8fba6d19`** — Cleanup commit: remove commented-out `OpaqueMember*` scaffolding + `DO NOT SUBMIT` notes that accumulated during PR 3.2 wire-format work
- [x] **`*PR 3.2 commit*`** — **PR 3.2**: wire-format expansion + Backend extern routing. Conflict resolution: dropped `RustShapeProjector.scala` (the revised PR 2.7 obsoletes it); `Hammer.scala` now uses `prototype.id` directly (no projection); `NameHammer.simplifyName` comment updated.
- [x] **`283bdc62`** — **PR 3.6 + named-arg fixes**: cherry-pick of PR 3.6 (gated `AbstractBodyS` routing on `citizenIsInterface`) + two follow-up fixes to PR 2.7's named-arg channel:
  - `OverloadResolver.attemptCandidateBanner` now seeds the rune-type-solver with `callsiteRune → receivingRune's type` for each pair (without this, `MaybeCoercingLookupSR` for callsite runes had no expected type → `SolveIncomplete`)
  - `FunctionCompilerSolvingLayer.evaluateGenericFunctionFromCallForPrototype` now actually appends `containerRuneInitialKnowns` to `initialKnowns` (was being dropped on the floor)

- [x] **`b3ca3c8b`** — **PR 3.7**: don't re-solve generic externs with empty args. Un-ignored "Extern function returning extern struct" (now PR-3.5-blocked on `ExternAttributeP` `vimpl`).
- [x] **`511f26d4`** — **PR 3.8**: fix PR 3.2 wire-format regressions. Adaptations:
  - Hammer: dropped the `mangleFunc(projectedId)` detour (RustShapeProjector already gone); rust externs use `""` directly, non-rust externs use `humanName.str` with `vwat` fallback.
  - Instantiator: refactored to make the non-generic/generic split explicit and disjoint. Non-generic externs flow only via up-front `nonGenericFuncExternsC` (from `HinputsT`); generic externs flow only via `monouts.functionExterns` (added at call sites, now gated on `templateArgs.nonEmpty`). No dedupe needed.

- [x] **`170eab01`** — **PR 3.9**: lex `#DeriveStructConstructor` / `#!DeriveStructConstructor` (single-file lexer addition; auto-merged clean).

- [x] **`9110e649`** — **Option 1**: defaults self-contained (stop hoisting EqualsSR). 7 files, clean auto-merge. Fixed the "with defaulted container generic" infinite loop (6 → 5 failures).
- [x] **`18c76751`** — **postparser fix**: internal-method handling of inherited generic params. Two-part: (a) return-type templex uses `functionEnv` unconditionally (was `interfaceEnv` for `ParentCitizen`, so method's own runes were invisible); (b) seed `runeToExplicitType` with parent generic params' types so HigherTyping rune-solver can solve inherited runes that aren't referenced in the method's signature. Fixed `Namespace method call with both container and method generic args`.
- [x] **`682e16fb`** — **test fix**: borrow T in the "only inherits container generic" minimal repro. The owned-T form failed at the trailing implicit drop because `Builtins`-only test env has no `drop(T)` (same failure for a parallel top-level `func make<T>(t T) bool { true }`). Switched to `&T` so the test exercises only the named-arg path it was designed for.
- [x] **`7d9e6a04`** — **PR 3.5 (subset)**: extern struct typing + Vivem Opaque stubs. Three-line typing-pass plumbing (`ExternAttributeP→ExternS→ExternT`, plus extern-kind dependency allowance), Vivem-side `OpaqueV` value + `newOpaque` helper, three Vivem extern stubs (`newVec`, `newVecWithCapacity`, `vecCapacity`) wired into FunctionVivem. Bonus: `InstantiatedHumanizer` for `ExternFunctionNameI` now emits `humanizeGenericArgs` so generic externs monomorphize to e.g. `Vec.capacity<i32>` (consistent with revised PR 2.7's parity intent). Fixed all 3 PR-3.5-blocked rust-extern tests (5 → 2 failures). Skips PR 3.5's deferred build infra (scripts/, Coordinator wiring).
- [x] **`70957bc0`** — **stdlib + Utils preexisting test breakage**: 1-arg `.expect()` calls in path test got msg args; intermediate locals introduced where chained method calls didn't auto-borrow owned-Path; duplicate `(suite).finish();` removed; `RuntimeSizedArrayEntriesIter` + `entries()` overload added so the top-level stdlib test's `arr.entries()` works on RSAs; `Utils/test.sh` retargeted from the long-deleted `pathadditions/test` to `command2/test`, with the now-auto-bundled `stdlib=` arg removed. Reproduces with the May-5 release-mac valec (pre-cherry-pick) so this isn't a chain regression; just unblocks the test suite running clean against our new artifacts.
- [x] **`ab4f0973`** — **PR 3.3**: Coordinator orchestration + triple-derived clang -arch. Unblocks the end-to-end rust interop script (`tmp/test-rust-interop.sh`) by adding `--vale_ruster_path`, `--divination_path`, `--rust_cargo_toml`, `--rust_output_dir` CLI flags + the post-Backend cbuild static-lib link + clang `-arch` derivation per @CADFRTZ.
- [x] **`5bc1ea7e`** — **PR 3.4**: Frontend PassManager invokes ValeRuster. Pre-scans inputs for `import rust.X.Y.Z`, invokes ValeRuster per distinct import path, chains a rust-package resolver before the generic fallback per @RRPGRZ.
- [x] **`a15df8ea`** — **Route `GenericParametersInheritance` to Hammer for extern wire-format SimpleId reshape**: closes the gap between PR 2.7 revised's typing-pass parity and Backend's pragma emission. Adds the field to `FunctionExternT` (single source of truth, populated by `FunctionCompilerCore.makeExternFunction` so it fires for both top-level and internal-method externs); retires from `ExternFunctionCallTE` (dead carrier); simplifies `GenericParametersInheritance` to just `numInheritedGenericParameters`; adds `numInheritedGenericParameters: Int` to `FunctionExternI` populated via linear scan of `hinputs.functionExternsT` at the generic callsite; Hammer reshape moves trailing inherited args off the leaf step onto the parent citizen step. Also: relaxed `ExternFunctionDependedOnNonExportedKind` to accept `KindPlaceholderT` (placeholders are substitution slots; concrete types matter at instantiation). Pragmas now emit as `Vec<i32>::with_capacity` rather than the broken `Vec::with_capacity<i32>`. **End-to-end Rust interop test PASSES** (binary exits 42). Frontend: 1103/0, +6 net (3 HammerTests + 1 IntegrationTest + 1 new CompilerTest + 1 un-ignored CompilerTest).
- [x] **`08add9a0`** — **PassManager + NameHammer follow-ups for rust interop**: AFTERM markers on `invokeValeRusterIfNeeded` / `resolveRustPackageContents` for eventual removal; `NameHammer.simplifyKind` handles `StrIT` in addition to `IntIT`.

## Pending (in order)

- [ ] **`3c44910a`** — PR 4.1: rust interop docs (was deferred for arcana conflicts with `CADFRTZ.md` / `RRPGRZ.md`; now we *have* both via PRs 3.3+3.4, so it should apply cleanly)
- [ ] **`76ab262c`** — DRSINI doc rewrite
- [ ] **Final sweep**: rebuild release-mac + run Tester (`Backend/test.sh`) + stdlib + Utils + rust-interop e2e against the latest commits to confirm everything's still green.

## Deferred / skipped

- [ ] **PR 3.5 (remainder)** — Build infra + smoke test scaffolding (`scripts/build-rust-interop.sh`, `scripts/README.md`, `scripts/mac/build-compiler.sh` ValeRuster build step). The typing + Vivem subset landed via `7d9e6a04`; the scripts are convenience-only (we already have `tmp/test-rust-interop.sh` working end-to-end), so they remain optional.

## Residual test failures

**None.** Frontend: 1103 succeeded, 0 failed, 27 ignored. Tester (Backend regression): 99/99. stdlib: 7/7 suites, 68 tests. Utils: 1/1. End-to-end rust interop (`tmp/test-rust-interop.sh`): PASS (binary exits 42, pragmas emit in correct shape `Vec<i32>::with_capacity` etc.).

The 4 tests that were originally failing on this branch (pre-PR-3.6 cherry-pick) decomposed as:

- ✅ "Call member function" — fixed by PR 3.6 (`AbstractBodyS` gating) + named-arg fix #2 (containerRuneInitialKnowns wiring)
- ✅ "Namespace method call with multi-param container" — same fixes
- ❌ "Namespace method call with both container and method generic args" — separate user-declared-rune issue (above)
- ❌ "Namespace method call with defaulted container generic" — separate defaults-loop issue (above)

Net: 4 originally-failing → 2 fixed, 2 progressed but still failing for different reasons + 2 PR-3.5-blocked tests we knowingly brought in via PR 3.6, + 1 new minimal repro we added.

## Notes for next session

- The cycle is: cherry-pick `--no-commit`, run `sbt test`, hand back for review, wait for LGTM, commit with original message via `git commit -C <hash>`. For commits with substantive deltas (like PR 3.6 + named-arg fixes), write a fresh commit message.
- Build artifact jars under `Frontend/lib/*.jar` show as modified (LFS pointer corruption) — leave alone.
- Untracked `Frontend/boot/` and `Frontend/java9-rt-ext-adoptopenjdk_11_0_10/` — leave alone.
- Tests run from the `Frontend/` directory: `cd Frontend && sbt test > ./tmp/<session>.txt 2>&1`. Sometimes need to use `bash -c 'cd /Volumes/V/Vale/Frontend && sbt ...'` because sbt has no `--manifest-path` equivalent.
- This file is transient — delete after the cherry-pick chain is fully landed.
