# Typing Pass Migration — TL Status & Residuals

**JR does not have access to this file.** When citing it to JR, paraphrase the rule inline rather than referencing it by name.

**Re-read this file every time you compact** — the prior conversation drops on compaction but this file doesn't.

**TL: read the `guardian-tl` skill and everything it references before starting work** — it covers roles, escalation, the verb protocol, shield mechanics, and ordination.

**Brevity rule:** any addition to this file must be one sentence, max 25 words, unless the architect explicitly asks for more.

**No simplifications.** 1:1 Scala parity outranks tidiness and blast radius; never omit dead bindings, inline helpers, skip checks, or substitute idioms without the architect's explicit go-ahead.

**Edit-surface aversion is not a reason to simplify.** When the lower-ripple option is less Scala-faithful, surface the tradeoff to the architect — don't pick quietly.

**Scope-bias trap: "lots of body fills" is still JR work.** A long mechanical port (e.g. an entire vonify_* family) is JR territory, not a TL one-shot. Push back when JR frames mechanical bulk as TL-level.

**Interner-threading channel:** parameters or god-structs. Runtime-state structs (`HeapV`, etc.) aren't god-structs just because they're widely-passed; don't bolt the interner onto them — thread it as a parameter.

**Per-test Scala-arg discipline:** check the Scala for each test before defaulting to in-tree precedent — Scala explicit args, default args, and "previous Rust test did X" can disagree, and Scala parity wins.

**`@TFITCX` classification is not the spec; Scala equality is.** If a `/// Arena-allocated`-tagged type is compared with case-class `==` in Scala, reclass to `/// Value-type` and derive — don't fight the convention with workarounds.

**Shield exceptions vs temp-disables.** Shield exceptions encode expected, mechanical divergences that recur uniformly; temp-disables flag surprising ones that warrant architect review per site.

**NCWSRX false-positives** often mean the Scala audit block sits outside the diff window; fix the slicing (move it adjacent to its Rust def) rather than temp-disable.

**Guardian directives must sit inside the def's contextified diff window** (between `fn name() {` and `}`, or at the impl-block head for methods) — placement above `#[test]`/`#[attr]` or far from the change is invisible to the shield.

**Debugging is TL's job — never delegate it to JR.** Guardian (SPDMX/NCWSRX/OALZDX) blocks JR from adding eprintlns or diagnostic scaffolding, and a temp-disable is not a workaround because temp-disables persist — we have no mechanism to remove them. TL slices the diagnostic, runs the test, hands JR the printed values, and reverts the diagnostic before JR is unblocked. Don't write "add an eprintln and re-run" into `for-jr.md`. Tools and techniques in **`docs/skills/migrate-diagnoser.md`**.

**Logic bugs are deferred, not blocking.** When JR's active test panics with something *other* than "unimplemented"/"implement"/"not yet migrated" (i.e. a real logic bug, not a still-stubbed body), they mark it `- [~]` in their worktree's `migration-drive-todo.md` with a one-line note (Rust file:line and the panic message), append the same to `for-tl.md` so TL knows, then pick the next `- [ ]` test and continue driving. TL queues these `[~]`s for later — don't try to fix them inline during the parallel sprint. This overrides the older "never defer a test" rule **for logic bugs only** — JR still doesn't defer for missing scaffolding or other Guardian-blocks, which still escalate-and-wait.

**Run SCPX `--check-all`** after every slice-in or audit-block edit; duplicating Scala text already in a top-of-file blob trips it.

**Resolve rebase conflicts in-place — never `git checkout --ours/--theirs` on a whole file.** Three-way merge already applied every non-conflicting hunk; `--ours`/`--theirs` throws those away. Edit only the `<<<<<<< ======= >>>>>>>` blocks.

**Pre-commit submodule-pin sanity check.** Each Vale-N worktree has its own `Guardian/` and `Luz/` sub-clone; another TL's pin advance is invisible until you fetch. Before any commit that touches submodule pins (defensively, before every commit): `git -C Luz fetch --all && git -C Guardian fetch --all`, then `git diff --submodule HEAD` — if it shows a backward diff or `(commits not present)`, your pin is stale and the commit would regress experimental. Observed live: Vale2 `0984c155a` regressed Luz 2 commits; Vale3 `ee57fc12d` regressed Guardian 2 commits (self-corrected in `8de534e50`).

**Include submodule pin bumps in commits — don't exclude them.** Especially Guardian's nested submodules (ContextifiedDiff, Rabble, etc.) and their inner Luz pins; commit from leaves up so each level's index is consistent.

**Push submodule advances to origin immediately after committing.** After any commit that advances a Luz or Guardian pin, the committing TL immediately pushes that submodule (and any nested submodules) to origin. Pins are not "shared" until pushed — otherwise other worktrees hit `(commits not present)` on their next rebase.

**During every sync, check if `migrate-tl.md` changed** (`git diff experimental..experimental-N -- migrate-tl.md` and `git diff HEAD -- migrate-tl.md`). The architect adds rules here mid-session; auto-merge may pull them in silently. Read every change — they're load-bearing for TL behavior and don't surface elsewhere.

**TestVM convention:** every testvm struct/enum/fn carries `<'v, 'h, 's>` with `where 's: 'h, 'h: 'v`; PhantomData for unused params; V-suffix names (`HeapV`, `CallIdV`, etc.).

**TestVM value-type embed-by-value:** small Copy-able value-types in testvm (`VoidV`, `IntV`, `BoolV`, `FloatV`, `StrV`, `OpaqueV`, `NodeContinueV`, `NodeReturnV`, `NodeBreakV`) embed by value in containing enums; only genuinely allocated payloads (`StructInstanceV`, `ArrayInstanceV`) stay `&'v`. Value-types that flow into HashMap keys (referrers, address-types) also derive `Hash, Eq, PartialEq` — apply the cascade through Variable/Member/Element/Argument/ExpressionId address structs as needed.

**TestVM mutation:** plain `HashMap`/`Vec` + `&mut self` for mutators; no `Cell`/`RefCell` interior mutability. Compile-time borrow checking only.

**TestVM writer threading:** `vivem_dout: &'v mut dyn std::io::Write` on `HeapV` (single owner); pass via `&mut heap.vivem_dout` to helpers. No `Box<dyn Write>` wrapper unless caller needs to own.

**Scala mutable-int pattern:** when Rust uses `&mut i32` to mutate via reference, Scala mirrors with `IntCounter` wrapper class (`Frontend/Utils/Utils.scala`). Same idiom, both languages.

**OALZDX (output discipline) is globally disabled** in `guardian.toml` for this migration. Don't write OALZDX rationales into new code; future migrations may re-enable it.

**This file is now thin.** The durable guidance that used to live here has moved into two docs; this file keeps only **current status** and **items not covered there**:

- **Typing-pass change guidance** (the Scala-parity principle, arena/lifetime model, recurring traps, easy-to-get-wrong design decisions, Good Partial Implementing, audit-trail discipline, slicing-in new definitions, slice-pipeline cleanup, Scala-parity tests) → **`docs/architecture/typing-pass-ai-guide.md`**.
- **Roles & coordination** (the TL/JR/architect loop, the division-of-labor litmus test, `for-tl.md`/`for-jr.md`, the "z" protocol, Guardian temp-disables, architect approval / the no-`TODO`/`AFTERM` rule, commit format) → the **`guardian-tl`** skill (`Luz/skills/guardian-tl.md`).

If a code comment or skill cites a former TL.md section by name (e.g. "Good Partial Implementing", "Cleaning Up After The Slice Pipeline", "Guardian Annotations…", the no-`TODO`/`AFTERM` rule), that content now lives in one of those two docs — search there.

---

## Where We Are

Scaffolding (Slabs 0–14b) is complete — every type/signature is built (`IRegionNameT` is the lone remaining `_Phantom`). Build is green (`cargo check --lib`; 2 pre-existing `expression_compiler.rs` warnings), SCPX 0.

**Typing pass — core test suite migrated.** Every test tracked in the now-retired `docs/historical/typing-test-todo.md` passes. The only remaining typing-test tail is the 40 `after_regions_*` tests (~14 are deliberate Scala-side deferrals) — see the residuals doc.

**Instantiating + simplifying — capstone landed.** `local_ids_unique` green; 6 hammer integration tests landed (`simple_main` → `tests_stripping_things_after_panic` + `two_templated_structs_make_it_into_hamuts`); X-bucket `top_level_extern_functions_wire_format_simple_id_has_flat_shape` green; peak 769/769.

**Pilot landed: `simple_program_returning_an_int` green; 770/770 tests passing.** Active frontier: the four-way parallel migration (SI/CL/GE/MI buckets, see `master-todo.md`).

**Parallel sprint structure:** `Vale` → `experimental-1` (SI), `Vale2` → `experimental-2` (CL), `Vale3` → `experimental-3` (GE), `Vale4` → `experimental-4` (MI). Each worktree has its own JR. `experimental` is the shared integration ref, **checked out in no worktree** — never `git checkout experimental` from any worktree.

**Per-TL CI loop (after every green test, from your own worktree):** JR posts sync-ready to `for-tl.md` (test name + `git diff --name-only` + "ready to sync") → **TL parity-reviews every JR change before surfacing to architect**: read `git diff` end-to-end, and for each Rust edit, also read the adjacent `/* scala */` audit block — confirm 1:1 parity, no smuggled simplifications, no inlined helpers, no novel logic. Special attention to **any new `Guardian: temp-disable: …` directives** and `/* Guardian: disable-all */` markers (verify each rationale is principled and matches in-tree precedent — temp-disables are easy to miss in a long diff and the architect can't see them otherwise). Then **pause and wait for the architect's explicit go-ahead before any state-mutating step (commit, rebase, or sync).** On go-ahead: `git commit` on `experimental-N` → `git rebase experimental` → re-verify green → `git fetch . experimental-N:experimental` to fast-forward `experimental` to your tip. Check the exit code; non-ff rejection means another TL beat you — re-rebase and retry. The architect gate is non-negotiable: never commit, rebase, or sync to `experimental` unilaterally. Concurrency-safe via git's ref locking. No remote pushes during the sprint; all worktrees share `/Volumes/V/Vale/.git` so branches are visible by name from anywhere.

**Pause for explicit architect go-ahead before syncing to `experimental`** — the `git fetch . experimental-N:experimental` step never runs unprompted.

**Test-only JR batches (parity-clean, no implementation changes): rebase + release JR uncommitted to accumulate; only surface to architect when JR's batch includes implementation changes.**

**Known deferred fix:** `CoordSendSR` Some-branch — designed, Scala-verified 1104/1104, reverted pending coordinated Scala+Rust landing. Write-up at `investigations/coord_send_some_branch_fix.md`. Blocks `panic_in_expr` and any test whose typing-pass overload resolution hits Never-sender + bound-receiver.

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

---

## Architect Philosophy (TL's read)

These are the principles I've inferred from working with the architect across many sessions. They're descriptive, not prescriptive — the architect's actual call always overrides; this section captures the patterns so a TL can predict the answer or recognize when they're about to violate one.

- **1:1 Scala parity is the absolute top priority, full stop.** Whenever there's a tension between "Scala-faithful" and "smaller diff" / "cleaner Rust" / "less ripple" / "easier ergonomics" — Scala parity wins by default. The architect will redirect when an exception is genuinely warranted; until then, assume parity beats every other consideration.

- **Watch for ripple-aversion bias.** The instinct to minimize edit surface ("don't add `Box<dyn Write>` ripple", "don't cascade `&mut self`", "don't refactor 9 callers for a sig change") is a real bias TLs are prone to, and it pulls against Scala parity. The architect notices when TLs pick the lower-ripple option without surfacing the tradeoff. When parity and ripple disagree, **surface the choice to the architect**; don't pick quietly.

- **Scala-absent Rust constructs are a code smell.** `impl Default`, custom traits, helper free fns, `Cell`/`RefCell`, `Box<dyn Trait>`, `'static` lifetimes — every one of these should justify its existence against "what does Scala do?" If Scala uses references (`val` holding a ref), Rust borrows (`&'v T`). If Scala has `object X { def apply() }`, Rust has `impl X { fn new() }` — not `impl Default`. Interior mutability is suspect; prefer `&mut self`.

- **Idiomatic Rust > ceremonial Rust.** Plain `HashMap` + `&mut self` is preferred over `Cell<HashMap>` + take/set ceremony. Compile-time borrow checking is preferred over runtime checking. The architect has consistently called out interior mutability as a slicer bias toward "preserve `&self`" that doesn't match how good Rust gets written.

- **Embed-by-value for small Copy types.** Value types should embed by value in containing enums, not be wrapped in `&'v`. The "minimize edit surface" temptation to leave some variants `&'v` because the active test doesn't exercise them is a real anti-pattern. Only genuinely heap-allocated payloads stay `&'v`; everything Copy-able embeds.

- **Match Scala's polymorphism semantics.** If Scala uses an abstract class (`PrintStream`, `IPackageResolver`), the Rust port should preserve polymorphism (`dyn Write`, `dyn IPackageResolver`). Pinning to a concrete implementation (`std::io::Stdout`) is a simplification that loses Scala's actual semantic.

- **Look at the canonical Scala directly.** Don't infer from existing audit-trail comments, prior session decisions, or "what JR said the Scala does." Read the actual `Frontend/.../Foo.scala` — the architect has caught me multiple times making decisions from outdated framing or paraphrases.

- **SCPX position-correctness drives Rust layout.** Rust definitions follow the canonical Scala block position, not the other way around. When a slicer mis-positioned a def, the fix is to move the Rust to where the Scala block lives, not move the Scala block to where the Rust ended up.

- **Push back when something looks off.** Questions like "why is there a Cell here?" or "does Scala have a `default()`?" or "why are we disabling all of Guardian on this?" are the architect's way of surfacing design issues the TL has rationalized over. When asked, give the honest answer — including "I picked the smaller diff to minimize ripple" if that's the truth. Then accept the redirect.

- **Codify recurring patterns into shield exceptions, not per-site temp-disables.** "Things should be SPDMX exceptions when they are expected. Things should be temp-disables when they are surprising and warrant my review." If a temp-disable rationale starts citing other in-tree precedents as justification, that's the signal to convert to an exception — or surface for architect review.

- **Never modify a shield without explicit architect approval.** Even when a temp-disable → exception conversion is clearly warranted, the shield edit itself needs the architect's go-ahead per request. Surface the proposed exception text and wait for the green light; don't pre-emptively edit `Luz/shields/*.md`.

- **Don't avoid hard work for the migration's sake.** When given a choice between G1 (easy, partial fix) and G2 (harder, more Scala-correct), the architect consistently picks G2. The migration is a finite effort; cutting corners during it leaves debt that's harder to clean up later than doing it right the first time.

- **Catch tendencies and codify them.** When the architect surfaces a TL bias (ripple-aversion, no-simplifications, position-correctness, etc.), they often want a new ≤25-word rule added to this file so future TLs (or future sessions of the same TL) don't repeat it. Watch for moments where the conversation surfaces a principle worth recording.

- **Brevity in rule additions, longer is fine when asked.** Default to ≤25-word rules; ask before adding anything longer. The exception is sections like this one (Background, Roadmap, Where We Are) that the architect explicitly approves as longer.
