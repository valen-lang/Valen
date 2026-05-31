# Typing Pass Migration — TL Status & Residuals

**JR does not have access to this file.** When citing it to JR, paraphrase the rule inline rather than referencing it by name.

**Re-read this file every time you compact** — the prior conversation drops on compaction but this file doesn't.

**Brevity rule:** any addition to this file must be one sentence, max 25 words, unless the architect explicitly asks for more.

**No simplifications.** 1:1 Scala parity outranks tidiness and blast radius; never omit dead bindings, inline helpers, skip checks, or substitute idioms without the architect's explicit go-ahead.

**Shield exceptions vs temp-disables.** Shield exceptions encode expected, mechanical divergences that recur uniformly; temp-disables flag surprising ones that warrant architect review per site.

**NCWSRX false-positives** often mean the Scala audit block sits outside the diff window; fix the slicing (move it adjacent to its Rust def) rather than temp-disable.

**Debugging (eprintln/diagnostic edits) is TL's job** — Guardian (SPDMX/NCWSRX) blocks JR from adding printouts or diagnostic scaffolding to existing bodies, so when JR's cascade hits an unexplained runtime panic, TL slices the diagnostic, re-runs, and hands JR the printed values. Don't tell JR to "add an eprintln and re-run." Tools and techniques for this live in **`docs/skills/migrate-diagnoser.md`**.

**Run SCPX `--check-all`** after every slice-in or audit-block edit; duplicating Scala text already in a top-of-file blob trips it.

**This file is now thin.** The durable guidance that used to live here has moved into two docs; this file keeps only **current status** and **items not covered there**:

- **Typing-pass change guidance** (the Scala-parity principle, arena/lifetime model, recurring traps, easy-to-get-wrong design decisions, Good Partial Implementing, audit-trail discipline, slicing-in new definitions, slice-pipeline cleanup, Scala-parity tests) → **`docs/architecture/typing-pass-ai-guide.md`**.
- **Roles & coordination** (the TL/JR/architect loop, the division-of-labor litmus test, `for-tl.md`/`for-jr.md`, the "z" protocol, Guardian temp-disables, architect approval / the no-`TODO`/`AFTERM` rule, commit format) → the **`guardian-tl`** skill (`Luz/skills/guardian-tl.md`).

If a code comment or skill cites a former TL.md section by name (e.g. "Good Partial Implementing", "Cleaning Up After The Slice Pipeline", "Guardian Annotations…", the no-`TODO`/`AFTERM` rule), that content now lives in one of those two docs — search there.

---

## Where We Are

Scaffolding (Slabs 0–14b) is complete — every type/signature is built (`IRegionNameT` is the lone remaining `_Phantom`). Build is green (`cargo check --lib`; 2 pre-existing `expression_compiler.rs` warnings), SCPX 0.

**Typing pass — core test suite migrated.** Every test tracked in the now-retired `docs/historical/typing-test-todo.md` passes (compiler_tests / solver / virtual / mutate / etc.). The only remaining typing-test tail is the 40 `after_regions_*` tests (~14 are deliberate Scala-side deferrals) — see the residuals doc.

**Active frontier: the instantiating pass.** Driven test-first through the capstone Hammer test `simplifying::test::hammer_test::local_ids_unique`, which exercises the instantiator end-to-end. Currently working through the **region collapser/counter subsystem** (`region_collapser_individual` / `region_collapser_consistent` / `region_counter` — ~90 sliced panic-stubs); JR drives it end-to-end per the standing ruling (escalate only a specific fn whose lifetimes fail the one-shot). The closure/return conventions for the `collapse_*` family were settled in rounds 16–17.

Body migration stays test-driven: the active test drives which panic stubs get implemented (see "How To Continue").

---

## Known Residual Items

Moved to **`FrontendRust/src/typing/typing-pass-todo.md`** (open deferrals, cleanups, and known gaps in the typing-pass migration — lives in the folder it's about).

---

## How To Continue

1. **Body migration is test-driven** — the active test drives which panic stubs get implemented. The promotion mechanism (un-ignoring the next test, the `migration-drive-todo.md` tracker, regression-guard discipline) is JR's, in `docs/skills/migration-drive.md`.
2. **Group related bodies into coherent review batches** — each batch gets a handoff doc listing target methods, driving test(s), and Scala translation gotchas; one junior per batch. Handoffs are proposals, not spec — expect and invite push-back.
3. **Commit only when the architect says "fire commit"** (format + protocol in the `guardian-tl` skill). Hand back uncommitted working trees at batch boundaries by default.
4. For *how* to make a change safely (parity, arena model, traps, slicing, tests) read **`docs/architecture/typing-pass-ai-guide.md`**; for *who does what and how escalations flow* read the **`guardian-tl`** skill.

---

## Roadmap To Completion (beyond migration-drive)

"Done" = FrontendRust replaces Scala `Frontend/`, emits matching VON, all tests green, then `Frontend/` + the `/* scala */` audit trail are retired. Everything *besides* the migration-drive grind:

- **Slicing** — complete (all passes, incl. TestVM).
- **Test harness** (`run_compilation.rs` + per-pass `*test_compilation.rs`) — migrated; the capstone test runs through it.
- **Pipeline / seam wiring** — finish `pass_manager::main`'s stubbed phases (Scout, `.vpst` loading, highlight mode), and replace the `full_compilation.rs` `HammerCompilation` `PhantomData` seam with the real thing.
- **`final_ast` data shapes** — `ast.rs`/`instructions.rs` payloads still on `PhantomData` need real field types.
- **VON wire-format parity** — `von_hammer` output must match what the backend reads (the real acceptance criterion); the ignored tests must assert the *same* behavior as Scala, not merely compile.
- **Cut over the build** — make the `frontend_rust` bin the actual frontend, replacing the Scala invocation.
- **Cleanup** — delete `higher_typing/textifier.rs` ("VISTODO: delete"); decide migrate-vs-delete on `solver/{i_solver_state,optimized_solver_state}.rs`; clear the 2 `expression_compiler.rs` warnings; revert the `RUST_MIN_STACK` workaround (see the todo doc).
- **Canonical management** — decide whether to freeze `Frontend/` or keep drift-reconciling; validate the one architect-approved canonical edit (`InstantiatingPass/.../templata.scala`'s `IsaTemplataI.subKind: KindIT[R]`) still `sbt`-builds.
- **Finish line** — once VON parity is proven: strip/archive the `/* scala */` audit trail, retire SCPX + the slice tooling, delete `Frontend/`.

Rough order: harness + wiring unblock → migration-drive fills bodies → VON parity proves it → cut over → cleanup → retire Scala.

---

## Background: The Sylvan Transplant

`instantiating/`, `simplifying/`, and `final_ast/` were **transplanted from a sibling repo, Sylvan** (`/Volumes/V/Sylvan`), which had migrated them against an *older* `Frontend/` than Vale's. Only those dirs were scope-transplanted (+ the `paste` crate + the `pass_manager`↔`HammerCompilation` seam stubbed with `PhantomData`) — **not** a full git merge (Sylvan slice-piped the whole frontend; a merge would have regressed Vale's hand-migrated typing/postparsing back to stubs). The transplant was then drift-reconciled up to **Vale's `Frontend/`, which is canonical and ahead**. Reconcile toward Vale's `Frontend/`, never toward Sylvan. (This is also why some stale artifacts carry Sylvan-isms — e.g. the old `RUST_MIN_STACK` Sylvan path.)
