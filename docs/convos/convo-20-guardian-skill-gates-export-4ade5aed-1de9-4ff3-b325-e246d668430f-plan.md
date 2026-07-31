# Plan document

Source: `/Users/verdagon/.claude/plans/tender-hopping-moonbeam.md`
Session: 4ade5aed-1de9-4ff3-b325-e246d668430f

---

# Guardian Skill Gates + Automatic Convo Export (stateless design)

## Context

A session bloated `vcoord-handoff.md` by 176 lines because nothing forced it to load the
`update-handoff` skill before editing the file — the skill was only read later, when the architect
asked about it. The file even carries a prose plea (`vcoord-handoff.md:3`: "Use the /update-handoff
skill before editing this file") with no enforcement behind it. Separately, exporting conversations
to `docs/convos/` is a manual close-session step that is skipped whenever a session dies at the
context limit — precisely the sessions whose record matters most.

Two features, both reading Claude Code transcript JSONLs
(`~/.claude/projects/<encoded-cwd>/<session-id>.jsonl`), both **stateless in Guardian** — the
transcript files and the exported artifacts are the only state; Guardian re-derives everything it
needs at decision time (architect's call, and consonant with Guardian's own NGSAX philosophy and
the MWGX/mailbox state-in-files precedent):

1. **Skill gate** — a new Rust-primary shield denies Edits/Writes to a file whose head carries a
   `guardian-require-skill:` marker unless this session's transcript shows the skill was loaded.
   The companion program itself opens the transcript at decision time. The deny message tells the
   agent exactly what to read and to retry — the enforcement *is* the delivery mechanism.
2. **Auto-exporter** — a poller thread in `guardian serve` keeps a per-session conversation
   markdown in `docs/convos/` current throughout the session's life, surviving context-limit
   death, crashes, and abandoned sessions. Each sweep is a pure function of the filesystem.

The transcript-reading mechanics already exist, tested, in **phoenix** (`Luz/phoenix`): offset
tailing with torn-line handling, transcript-line parsing, the `claude-extract` wrapper with
output-file recognition, and projects-dir transcript location. They get factored into a shared
`Luz/transcript` crate consumed by phoenix, the gate companion, and the exporter.

## Decisions already made (architect-confirmed in conversation)

- **No state in Guardian.** The gate check runs inside the deterministic companion, which reads
  the transcript fresh per gated edit; the exporter derives session→file mapping, throttle
  position, and numbering from the filesystem on every sweep. Guardian gains only a poller thread
  (an activity, not state) and one plumbed field. A Guardian restart loses nothing.
- **Guardian hosts this, not phoenix.** Gate decisions must be in-process with the shields, and
  Guardian serve is effectively always-on (edit hooks fail closed without it) while phoenix runs
  only for supervised chains. Phoenix keeps its lifecycle role; all three consumers share the new
  crate.
- **Read-evidence is the accepted bar** for the gate: the transcript proves the skill was loaded
  this session, not that it's still in live context or was adhered to. No recency knob for now.
- **The gate check is structural, never substring**: count only `tool_use` blocks inside
  assistant-type transcript entries (`name: "Skill"` with matching `input.skill`, or `name: "Read"`
  with `input.file_path` equal to the skill doc). A raw grep is self-defeating — the gate's own
  deny message plants the path string in the transcript (as would a user pasting a conversation).
- **Gate cost profile**: the companion checks `original_content` for markers FIRST and only opens
  the transcript when one is present — zero transcript reads for ungated edits (the overwhelming
  majority), one cheap scan for gated ones (substring pre-filter: JSON-parse only lines containing
  `"name":"Skill"` / `"name":"Read"`).
- **LLM fallback is conservative-deny**: the shield markdown (which doubles as the crash-fallback
  prompt) instructs an LLM judge that cannot verify transcript evidence to deny with the
  read-then-retry instruction. Worst case is one redundant read.
- **`claude-extract` is used at export time, not gate time.** The gate companion parses raw JSONL;
  the exporter shells out to `claude-extract` (the architect's editable fork at
  `/Volumes/V/claude-conversation-extractor`) — fail-open, log-only on failure. (`claude-extract`
  has no incremental mode — every export re-renders the whole session — so a cursor never helped
  the heavy step anyway; size-vs-stamp throttling gates it equally well.)
- **Why polling (not hook-traffic-driven) for the exporter**: a context-limit wedge writes its
  final messages to the JSONL but sends no further hook requests; chat-only sessions send none at
  all. Context-limit death IS visible in the file: phoenix's live tests verified overflow is
  appended as `isApiErrorMessage:true` + `"Prompt is too long"`.
- Marker frontmatter in the gated file (travels with the file; any file can opt in); adding and
  removing markers stays a human act. NAGDX today matches only the literal substring `Guardian:`
  (`Luz/shields/NoAddingGuardianDirectives-NAGDX/src/main.rs:12-14`), so it must be extended to
  also match `guardian-require-skill:` — which covers both addition and removal, since NAGDX flags
  any changed (`+` or `-`) line containing a directive.

## Decisions with recommended defaults (overridable at plan approval)

- **Shield name/code**: `RequireSkillBeforeEdit-RSBEX`.
- **Marker syntax**: HTML comment `<!-- guardian-require-skill: update-handoff -->` in the first 10
  lines (works in files without YAML frontmatter blocks; invisible in rendered markdown).
- **Export naming**: `convo-<N>-<slug>.md` following the existing `docs/convos/` convention; `N`
  minted per sweep as max existing + 1, slug from the transcript's latest `ai-title` entry. The
  exporter stamps `<!-- session: <uuid>; exported-bytes: <N>; transcript: <abs path> -->` at the
  top of the file after each extract; a session's existing file is found by scanning
  `docs/convos/` heads for the uuid; the throttle position is the `exported-bytes` value; the
  transcript path is the debugging pointer to the raw JSONL. All bookkeeping lives in the
  artifact.
- **Exports land directly in `docs/convos/`** (no staging dir). The file sits modified until a
  fire commit, same as any other working change.
- **Cadence**: poller sweeps every 30s; a session is (re-)exported when
  `transcript size − exported-bytes ≥ 64KB`, or when any growth exists and the convo file's mtime
  is ≥ 60s old. No memory between sweeps.
- **Raw JSONLs are NOT checked in** (architect reversed the earlier LFS idea — they're for
  debugging only). Instead they stay in `~/.claude/projects/` and retention is extended by config:
  `"cleanupPeriodDays": 60` in `~/.claude/settings.json` (verified currently unset → default
  ~30 days applies). The exported markdown carries the pointer — the head stamp includes the
  transcript's absolute path — so debugging starts from the convo doc. Accepted limitation: the
  pointer is machine-local and goes stale after retention; the markdown (the archival part) is
  already in git by then.
- **close-session.md's export step (step 2) is retired entirely**, not just slimmed: guardian
  serve outlives the session and the transcript persists after SIGTERM, so the next sweep exports
  the final turns posthumously — including the close-session wrap-up itself, which the manual
  step could never capture. A one-line note replaces it (exports are automatic; where they land).
  The plan-document sibling (`claude-plan-…`) is kept when claude-extract produces one, renamed
  alongside the conversation.

## Architecture

```
Luz/transcript (new crate — mechanics extracted from phoenix; no state, pure functions + IO helpers)
  ├─ tailing: consume whole appended lines past a byte offset (from supervisor.rs::scan_appended_lines)
  ├─ parsing: entry iteration, message_text, is_context_overflow, last_user_message (from detect.rs),
  │           NEW: loaded_skills_in(lines) (assistant tool_use only), latest_ai_title(lines)
  ├─ extract: claude-extract invocation + output recognition (from supervisor.rs::extract_dialogue,
  │           handoff.rs::pick_extract_outputs / session_prefix)
  └─ discover: projects-dir transcript location without cwd-encoding reconstruction
              (from chain.rs::find_transcript_file, discover.rs)

Guardian serve (request-driven, plus one new stateless poller thread)
  ├─ plumbing: transcript_path into PreToolUse HookInput and file-context ProgramInput
  └─ poller/exporter thread: each 30s sweep = stat transcripts → read docs/convos head stamps →
       run claude-extract where threshold met → restamp. Pure function of the filesystem; fail-open.

Luz/shields/RequireSkillBeforeEdit-RSBEX (new shield + Rust companion; depends on Luz/transcript)
  └─ run(input): markers in original_content head (else allow, transcript untouched) →
     structural scan of transcript_path for skill-load evidence → allow / deny naming the skill file

Luz/shields/NoAddingGuardianDirectives-NAGDX — matcher extended to guardian-require-skill:

phoenix — re-pointed at Luz/transcript; behavior unchanged
```

## Key existing code (reuse, don't reinvent)

- `Luz/phoenix/src/supervisor.rs:60` `scan_appended_lines` — offset tailing, whole-lines-only.
- `Luz/phoenix/src/detect.rs` — `message_text` (string vs blocks content), `is_context_overflow`,
  `last_user_message` (skips tool_result-as-user and `isMeta` lines).
- `Luz/phoenix/src/supervisor.rs:83` `extract_dialogue`, `Luz/phoenix/src/handoff.rs:51`
  `pick_extract_outputs`, `session_prefix` — claude-extract wrapper + output recognition.
- `Luz/phoenix/src/chain.rs:355` `find_transcript_file` — locate `<uuid>.jsonl` by scanning
  projects-dir subdirs (never reconstruct the cwd encoding); `discover.rs::cwd_from_transcript`
  for filtering a sweep to this checkout's sessions.
- `Guardian/claude_hook/src/lib.rs:5` `HookInput` — PreToolUse input; gains `transcript_path`
  (serde-default, backward compatible; Claude Code already sends it, serde currently drops it).
- `Guardian/src/serve/hook.rs:95` `validate_hook` (Edit/Write path), `:470` `validate_stop_hook`
  (already threads `session_id`/`cwd`/`transcript_path` at `:535` — the threading model).
- `Guardian/ShieldFile/src/program.rs:46` `ProgramInput` — already has `session_id` /
  `transcript_path` fields (stop-context only today); file-context population is the only change.
- `Luz/shields/MailboxWatcherGuard-MWGX/src/main.rs` — companion template: thin `main()` reading
  stdin JSON, dark-box function with all IO/env injected, `#[cfg(test)]` tests calling it directly.
- Test fixture generation: use `claude-extract --list` / `--search` to find real sessions to carve
  JSONL fixtures from (dev-time only).

## Wiring specifics (verified against the working tree)

- **Shield registration**: `FrontendRust/guardian.toml:1` has
  `shields_dirs = ["../Luz/shields", "docs/shields"]` (resolved relative to the config file).
  Only shields listed in a mode's `include_shields` run; RSBEX gets an entry in `[guard_mode]`
  (`guardian.toml:57`, MWGX's is at `:74`). Every `.md` on disk must appear in `include_shields`
  or `exclude_shields` or startup fails (`Guardian/src/config.rs:297` `check_shield_coverage`).
- **RSBEX frontmatter**: `g_context: diff` (file-scope, one run over the whole-file contextified
  diff — same as NAGDX), `g_primary: rust`, `g_program: RequireSkillBeforeEdit-RSBEX`,
  `g_filter_file: "*"`. Context values parse at `ShieldFile/src/lib.rs:759-788`; Edit/Write
  partition at `ContextifiedShield/src/lib.rs:310-321`.
- **Companion conventions**: directory name = package name = binary name (Guardian looks for
  `<dir>/target/debug/<dirname>`, `ShieldFile/src/lib.rs:264-269`); `Cargo.toml` starts with a
  bare `[workspace]` table (mandatory — without it a host repo's root workspace claims the crate
  and Guardian's startup companion build fails); dark box is `run(input) -> violations` per
  `guardian-rustify.md:52`. The companion declares its own minimal `ProgramInput` struct with
  `#[serde(default)]` fields — serde ignores the rest of Guardian's payload — naming only
  `file_path`, `original_content`, `modified_content`, `transcript_path`. Path-dependency on the
  new crate: `transcript = { path = "../../transcript" }` (same pattern as
  `shield-utils = { path = "../shield-utils" }`). 30s timeout (`ShieldFile/src/lib.rs:288`) is
  ample for one filtered transcript scan.
- **Plumbing route for `transcript_path` into file-context shields**: `validate_hook`
  (`Guardian/src/serve/hook.rs:95`, call at `:217`) →
  `ContextifiedShield::run_shields_on_file_change` (`ContextifiedShield/src/lib.rs:294-400`) →
  `validate.rs:58/:66/:74` (`run_shield_file_on_diff` / `..._on_definition`) → `ProgramInput`
  struct literals at `ShieldFile/src/lib.rs:1381-1391` (file-scope) and `:1478-1488` (per-def).
  These currently leave the stop-only fields defaulted; `transcript_path` + `session_id` thread
  through the same signatures. The stop path's existing threading (`hook.rs:529-541` →
  `run_shield_file_on_stop` → `ShieldFile/src/lib.rs:1626-1632`) is the model.
- **NAGDX extension**: `contains_directive` at
  `Luz/shields/NoAddingGuardianDirectives-NAGDX/src/main.rs:12-14` gains the
  `guardian-require-skill:` match; its 12 in-file unit tests + `tests/cases/` corpus get
  companions. (The marker deliberately does NOT match the `Guardian:` directive grammar in
  `ContextifiedShield/src/policy.rs:80`, so `strip_directives` leaves it visible to shields.)
  Known consequence, accepted: NAGDX is deny-only with no allowlist, so after the extension,
  spelling the literal marker string in ordinary prose (docs, comments) is also architect-gated —
  the same discipline that already applies to `Guardian:` in prose. Docs written by sessions
  should say "the require-skill marker" rather than quoting it.
- **New crate conventions**: model is `Luz/mailbox` — `[lib]` (+ `[[bin]]` only if a CLI is
  wanted; not needed), bare `[workspace]` table (Luz commit `f35a2e0` explains why). Prior art to
  fold in eventually: `Luz/mailbox/src/transcript.rs` (`resolve_transcript_path`) — out of scope
  to migrate now, but the new crate should not duplicate its behavior differently.
- **Poller placement**: `guardian serve` has no periodic thread today — everything is
  request-driven (axum router at `Guardian/src/serve/mod.rs:278-288`). The one-shot LSP warm
  thread at `serve/mod.rs:183-200` is the spawn template; the poller is a new long-lived thread
  (or `tokio::spawn` in `async_run` near `:252-276`). It carries only its config (paths,
  cadence, extract-bin) — no shared mutable state with the handlers. Serve runs one instance per
  checkout from the repo root (this repo: port 7880), so `docs/convos/` resolves against the
  serve process's cwd; the sweep filters transcripts to this checkout via `cwd_from_transcript`.
- **NMSFX**: shield files (and their companions) can't be edited by unordained AI sessions. The
  RSBEX shield markdown + companion, the NAGDX matcher change, and the guardian.toml entry are
  prepared as content and either applied by the architect or in an ordained session — per
  `guardian-add` / @BIASZ. Plan execution should surface each such file rather than fight the
  denial.

## Implementation phases + RFIGA list

(Phases in dependency order. Guardian workspace tests run per Guardian/CLAUDE.md with the three env
vars; phoenix suite with `-- --test-threads=4`.)

### Phase A — `Luz/transcript` crate (extraction from phoenix)

1. Crate skeleton + tailing primitive moved.
   * R: port phoenix's tailing tests (torn tail left for next pass; offset advances over whole
     lines) against the new crate's public API.
   * F: run them; expect failure (crate empty).
   * I: create `Luz/transcript` (standalone crate, bare `[workspace]` table); move/generalize
     `scan_appended_lines` into a cursor-tailing API that returns consumed complete lines.
   * G: re-run; pass.
   * A: full phoenix + new-crate suites.
2. Parsing module moved + extended: `message_text`, `is_context_overflow`, `last_user_message`
   move; NEW `loaded_skills_in(lines)` (assistant-entry `tool_use` of `Skill`/`Read` only, with
   the substring pre-filter) and `latest_ai_title(lines)`.
   * R: move phoenix's detect tests; add new tests from real-transcript-shaped fixtures — a Skill
     invocation counts, a Read of the skill doc counts, the path inside a deny-reason tool_result
     does NOT count, a user-pasted mention does NOT count, a Skill/Read of a *different*
     skill/path does NOT count, a malformed JSONL line is skipped not fatal (torn tails are
     routine); `latest_ai_title`: the latest of several wins, absent → None.
   * F/I/G/A as above.
3. Extract + discover modules moved (`extract_dialogue`, `pick_extract_outputs`,
   `session_prefix`, `find_transcript_file`, `cwd_from_transcript`).
   * R: move the phoenix tests for these.
   * F/I/G/A.
4. Phoenix re-pointed at `Luz/transcript`; its local copies deleted.
   * R: (no new tests — this slice is green-preserving refactor)
   * I: swap imports; delete moved code.
   * G/A: full phoenix suite green (`cargo test --manifest-path Luz/phoenix/Cargo.toml --
     --test-threads=4`).

### Phase B — transcript_path plumbing (Guardian)

5. `HookInput` gains `transcript_path`; populate `transcript_path` + `session_id` in file-context
   `ProgramInput` (threading through `run_shields_on_file_change` → `validate.rs` → the struct
   literals at `ShieldFile/src/lib.rs:1381`/`:1478`, modeled on the stop path).
   * R: claude_hook deserialization test (payload with transcript_path); tests through
     `validate_hook` with Rust-primary shields asserting the companion receives
     transcript_path/session_id in BOTH file-scope (`g_context: diff`) and per-definition
     (`g_context: definition`) contexts — they are separate `ProgramInput` construction sites
     (`ShieldFile/src/lib.rs:1381` vs `:1478`) and either could silently stay defaulted.
   * F/I/G/A (Guardian workspace suite).

### Phase C — skill-gate shield RSBEX

6. Companion program: dark-box `run()` — parse markers from `original_content` head (first 10
   lines; new-file Writes fall back to `modified_content`); no marker → allow without touching
   the transcript; marker present → structural scan of `transcript_path` via `Luz/transcript`;
   deny text names `docs/skills/<name>.md` and says retry after reading. Own minimal
   `ProgramInput`, MWGX-shaped thin `main()`.
   * R: unit tests in the companion's `main.rs` against dark-box `run()` (per @DBAPIZ), with
     fixture transcripts in tempdirs: no marker → allow (and provably no transcript read — use a
     nonexistent transcript_path); marker + Skill-invoked → allow; marker + skill doc Read →
     allow; **Read evidence counts under BOTH path spellings** — `docs/skills/<name>.md` and the
     registration symlink `.claude/skills/<name>/SKILL.md` (the Skill tool and a
     deny-message-following Read hit different spellings of the same file); marker + evidence
     only inside a tool_result/user entry → deny; marker + no evidence → deny naming the skill
     file; multiple markers → all must be satisfied; new file carrying a marker is gated via
     `modified_content`; missing/unreadable transcript → deny (conservative); marker on line 11 →
     NOT gated (the first-10-lines boundary is spec, so test it); marker with irregular interior
     whitespace (`<!--  guardian-require-skill:  x  -->`) → recognized (tolerant parse).
   * F/I/G/A (`cargo test` in the companion).
7. NAGDX matcher extension: `contains_directive` also matches `guardian-require-skill:`, making
   marker addition AND removal architect-gated. (NMSFX: prepared content, applied by the
   architect or an ordained session.)
   * R: new unit tests beside the existing 12 in NAGDX's `main.rs` — added marker line flagged,
     removed marker line flagged, unrelated `guardian-…` prose not flagged; plus a
     `tests/cases/` corpus pair.
   * F/I/G/A (NAGDX companion `cargo test` + `guardian test-shield`).
8. Shield markdown + registration (`[guard_mode] include_shields` entry in
   `FrontendRust/guardian.toml`) + end-to-end `test-shield` cases; conservative-deny LLM-fallback
   prose; marker added to `vcoord-handoff.md` (formalizing its line-3 plea). Follow
   `guardian-add`/@BIASZ; NMSFX gating as in slice 7.
   * R: `test-shield` expect-deny case (gated file, skill not loaded) + expect-allow case (skill
     loaded) built from carved fixtures.
   * F/I/G/A (`guardian test-shield` + workspace suite).

### Phase D — auto-exporter (stateless poller)

9. Sweep logic as a pure function: given (transcript metas, docs/convos head stamps, cadence
   config) → list of export actions. Stamp format
   `<!-- session: <uuid>; exported-bytes: <N>; transcript: <abs path> -->`;
   find-existing-by-stamp; mint next `convo-<N>` from existing filenames; slug from
   `latest_ai_title`.
   * R: unit tests on the pure function: first export mints next N with slug; growth below
     threshold → no action; growth ≥ 64KB → re-export same file; stale-mtime + any growth →
     re-export; foreign-checkout transcripts (different `cwd`) ignored; **legacy stampless files**
     (`convo-8-…` etc., which exist today) are counted when minting N but are never selected as an
     overwrite target; two quiet sessions in one sweep mint distinct N and N+1.
   * F/I/G/A.
10. Export execution + poller thread: run `claude-extract` via the crate wrapper into a temp dir,
    move/rename into `docs/convos/`, restamp; failures logged, never propagated. Thread spawned in
    `guardian serve` (template: warm thread at `serve/mod.rs:183-200`), 30s interval.
    * R: dark-box tests with a fake `extract-bin` (phoenix's fake-script test pattern): a grown
      quiet transcript gets exported by a sweep with no hook traffic; extract failure leaves the
      previous export intact and the server serving; second sweep with no growth does nothing; a
      `claude-plan-…` sibling emitted by the extract is renamed alongside the conversation.
    * F/I/G/A.
11. Retention + docs: set `"cleanupPeriodDays": 60` in `~/.claude/settings.json` (config change,
    architect-visible); retire `docs/skills/close-session.md` step 2 (the manual export),
    replacing it with a one-line note that exports are automatic, land in `docs/convos/`, and
    carry the raw-transcript path in their head stamp (mechanism/discipline sections unchanged).
    * R: (config+doc slice — no tests; prose per prose-reviewer; the stamp's transcript-path
      field is already covered by slice 9's unit tests)
    * I/A: edit; full suites still green.

## Out of scope (explicitly)

- Recency/compaction-awareness for the gate (read-evidence bar accepted).
- Phoenix behavior changes beyond the crate re-point (its 200ms overflow tailing is untouched).
- Moving the exporter into phoenix in an all-sessions-supervised future (crate boundary makes that
  cheap later).
- SessionEnd/PreCompact hook registration (polling covers the cases).
- Migrating `Luz/mailbox/src/transcript.rs` onto the new crate.

## Verification (end-to-end, after all phases)

1. Guardian workspace suite + phoenix suite green; `guardian test-shield` green for RSBEX and
   NAGDX.
2. Live smoke: in a scratch session, attempt an Edit to `vcoord-handoff.md` without loading the
   skill → deny names the skill file; Read it → retry succeeds. Restart guardian serve between
   the deny and the retry → retry still succeeds (statelessness).
3. Live smoke: converse without tool calls, wait one poll interval → `docs/convos/convo-<N>-….md`
   appears and refreshes; kill the session hard → file survives, and the next sweep exports the
   final turns posthumously (guardian serve outlives the session).
4. Spot-check: an exported convo doc's head stamp names a transcript path that exists and is the
   session's JSONL; `~/.claude/settings.json` carries `cleanupPeriodDays: 60`.
