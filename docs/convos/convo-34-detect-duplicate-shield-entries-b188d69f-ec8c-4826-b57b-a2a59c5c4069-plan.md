# Plan document

Source: `/Users/verdagon/.claude/plans/graceful-puzzling-karp.md`
Session: b188d69f-ec8c-4826-b57b-a2a59c5c4069

---

# Detect duplicate shield entries at Guardian startup

## Context

Guardian hook request 042 died with a panic instead of validating an edit:

```
panicked at Rabble/src/steppy_logger.rs:220:9:
Log file already exists: .../hook-042/log.file-scope.AllowedFileExtensionsOnly-AFEOX.log
```

which propagated as `JoinError::Panic` through `.unwrap()` at `Guardian/src/serve/mod.rs:407`.

**Root cause:** `FrontendRust/guardian.toml` lists `AllowedFileExtensionsOnly-AFEOX.md` **twice** in
`[guard_mode].include_shields` (once at the end of the first group, again at the start of the next).
`resolve_config_filtered` (`Guardian/src/config.rs:167-181`) pushes one `check_files` entry per
`include_shields` entry with no duplicate detection, so AFEOX was loaded twice. AFEOX is
`g_context: diff`, so both copies landed in `diff_shields` (`ContextifiedShield/src/lib.rs:317`), and
`run_wave_executor` (`ContextifiedShield/src/validate.rs:198-204`) called
`logger.child("AllowedFileExtensionsOnly-AFEOX")` twice. The second call hit
`SteppyLogger::create_file`'s `assert!(!path.exists())`.

The log directory confirms it: `log.file-scope.AllowedFileExtensionsOnly-AFEOX.log` exists at 0 bytes
with no verdict artifacts — it blew up creating the logger, before any shield ran. Requests 040/041
were Bash commands (command-context path), which never reach the diff-scope executor, which is why
only file edits were affected.

**Intended outcome:** a duplicated shield entry becomes a clear startup failure naming the mode and
the shield, instead of a mid-hook panic pointing at a log file path. A second, narrower guard inside
`run_wave_executor` catches the same condition for any non-toml caller.

Verified: `Guardian/guardian.toml` has no per-mode duplicates, so the new check will not block
Guardian's own startup or test suite. `FrontendRust/guardian.toml` has exactly one duplicate.

## Design decisions (confirmed with the architect)

- **Scope: all modes in the file.** `check_no_duplicate_shields` scans every `[*_mode]` section
  regardless of which mode is being started, mirroring the existing `check_shield_coverage`
  (`Guardian/src/config.rs:260`) which already iterates `config.modes.values()`. The same shield
  appearing in *different* modes stays legal — `ValidateReadonlyBash-VRBX.md` is legitimately in both
  `guard_mode` and `review_mode` today. Only repetition *within one mode* is an error.
- **Defense in depth.** Both the config check and a guard in `run_wave_executor`.
- **The executor guard returns `Err(ValidationError)`, not a panic.** `run_wave_executor` already
  returns `Result<Vec<ShieldRun>, ValidationError>`, and the hook path already renders
  `ValidationError` to the user — so an `Err` naming the duplicated shield reaches the human as a
  readable message, and it's testable without `#[should_panic]`. Flagging this because the option
  preview said "panic"; say the word and I'll make it a `panic!` instead.
- `SteppyLogger`'s `assert!` stays untouched. It did its job here — it just fired late.

## Prerequisite (not an RFIGA slice — config data, no test)

Remove the duplicate line from `FrontendRust/guardian.toml` `[guard_mode].include_shields`:

```toml
    { name = "AllowedFileExtensionsOnly-AFEOX.md" },

    { name = "AllowedFileExtensionsOnly-AFEOX.md" },   # <- delete this one
```

This must land **first**. Once slice 1 is in, a rebuilt guardian binary refuses to start against the
current `FrontendRust/guardian.toml`.

## RFIGA list

1. **`resolve_config_filtered` rejects a shield listed twice within one mode, in any mode of the file.**
   * **R:** add three tests to `Guardian/tests/config_tests.rs`, following the existing tempdir +
     `parse_and_resolve` style of `test_parse_and_resolve_end_to_end` (line 6):
     - `test_duplicate_shield_within_mode_is_rejected` — `[guard_mode]` lists `ShieldA-XSA.md`
       twice; assert `parse_and_resolve(&toml_path, "guard_mode")` is `Err` and the message names
       both `guard_mode` and `ShieldA-XSA.md`.
     - `test_same_shield_in_two_modes_is_allowed` — `ShieldA-XSA.md` in both `[normal_mode]` and
       `[guard_mode]`; assert `Ok`.
     - `test_duplicate_in_other_mode_still_rejected` — duplicate lives in `[review_mode]` while
       resolving `normal_mode`; assert `Err`. This is the "all modes" decision, pinned.
   * **F:** run the suite; confirm all three fail (first two compile-fail on the missing function if
     the test calls it directly — call only through `parse_and_resolve` so they fail on behavior).
     Report "Tests are correctly failing, proceeding with implementation."
   * **I:** add `pub fn check_no_duplicate_shields(config: &GuardianConfig) -> Result<(), Vec<String>>`
     to `Guardian/src/config.rs`, placed beside `check_shield_coverage`. Per mode, count
     `entry.name` occurrences; collect `"[mode] name (xN)"` for any count > 1; sort for determinism.
     Call it in `resolve_config_filtered` immediately after the `check_shield_coverage` call
     (`config.rs:151-156`), with a parallel error wrapper:
     `"Duplicate shield error: these shields are listed more than once within a single mode:\n  {}"`.
   * **G:** re-run the three tests; confirm they pass.
   * **A:** run the full workspace suite.

2. **`run_wave_executor` refuses a shields slice containing a repeated shield basename.**
   * **R:** add `Guardian/ContextifiedShield/tests/duplicate_shield_tests.rs` (new file, styled on
     `tests/assumes_tests.rs`): write one shield file to a tempdir, compile it with
     `Shield::get_and_compile_with_temp_logger`, put the **same** compiled `Shield` in the slice
     twice, and call the public entry point `run_shields_on_file_change` with a diff-scope shield.
     Assert `Err` whose `message` names the duplicated shield. Testing through
     `run_shields_on_file_change` rather than `run_wave_executor` keeps the test on the dark-box
     boundary (@DBAPIZ) and reproduces the exact hook path that panicked.
   * **F:** run it; confirm it fails — and specifically that it fails today with the
     `Log file already exists` panic, i.e. it reproduces the original bug.
     Report "Tests are correctly failing, proceeding with implementation."
   * **I:** in `ContextifiedShield/src/validate.rs`, before the child-logger loop at line 198, walk
     `shields` collecting file stems into a `HashSet`; on the first repeat return
     `Err(ValidationError { message: format!("Duplicate shield in executor input: {}", basename), log_file: None })`.
     The basename computation already exists inline at lines 199-202 and again at 208-211 — extract
     it to a small local helper and use it in all three places rather than writing it a fourth time.
   * **G:** re-run the new test; confirm it passes with the named error instead of the panic.
   * **A:** run the full workspace suite.

## Files touched

- `FrontendRust/guardian.toml` — delete the duplicate `AFEOX` entry (prerequisite).
- `Guardian/src/config.rs` — new `check_no_duplicate_shields`, wired into `resolve_config_filtered`.
- `Guardian/tests/config_tests.rs` — three new tests.
- `Guardian/ContextifiedShield/src/validate.rs` — executor guard + basename helper extraction.
- `Guardian/ContextifiedShield/tests/duplicate_shield_tests.rs` — new test file.

`ContextifiedShield` is a submodule; per `Guardian/CLAUDE.md` it needs `git checkout main` before
editing, and the `[patch]` in `Guardian/Cargo.toml` means edits compile immediately with no pin bump.

## Verification

Test command (all three env vars required, per `Guardian/CLAUDE.md`), output to one fixed file for
the whole session:

```bash
cd /Volumes/V/Vale2/Guardian
OPENROUTER_API_KEY=$(cat api_key.txt) \
GUARDIAN_OPENCODE_ROOT=$(pwd)/opencode \
GUARDIAN_BUN_PATH=/Users/verdagon/.bun/bin/bun \
  cargo nextest run --workspace > ./tmp/dup-shield-detect.txt 2>&1
```

then inspect `./tmp/dup-shield-detect.txt` as a separate command.

End-to-end, after both slices land:

1. `cargo build --workspace` in `Guardian/`.
2. Restart the Guardian server on port 7880 and confirm it starts cleanly against
   `FrontendRust/guardian.toml` (no "Duplicate shield error" — the prerequisite fix removed it).
3. Negative check: temporarily re-add the duplicate line, start `guardian serve`, confirm it exits
   non-zero printing `Duplicate shield error: ... [guard_mode] AllowedFileExtensionsOnly-AFEOX.md (x2)`,
   then remove it again.
4. Positive check: edit a `.md` file so the diff-scope wave runs, and confirm the hook returns a
   normal verdict — the case that produced request 042's panic.
