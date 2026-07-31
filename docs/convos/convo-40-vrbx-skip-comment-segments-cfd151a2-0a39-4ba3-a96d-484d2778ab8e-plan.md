# Plan document

Source: `/Users/verdagon/.claude/plans/plan-out-the-change-peppy-pascal.md`
Session: cfd151a2-0a39-4ba3-a96d-484d2778ab8e

---

# VRBX: skip shell-comment lines in compound segment loop

## Context

Guardian request-797 asked the user to confirm this Bash command instead of auto-approving:

```
# E0422 retired type breakdown
grep "^error\[E0422\]" tmp/vcoord-typing-slice.txt | grep -oE "cannot find struct, variant or union type \`[A-Za-z_]+\`" | sort | uniq -c | sort -rn
```

Every piece of that pipeline (`grep`, `sort`, `uniq`) is read-only and VRBX would auto-allow the command by itself. But a leading `# …` comment line poisons the classification: VRBX's segment splitter treats `\n` as a separator (`Luz/shields/ValidateReadonlyBash-VRBX/src/lib.rs:1821`), so the command splits into two segments — `# E0422 retired type breakdown` and the pipeline — and then the compound loop at `lib.rs:1737-1751` runs each through `is_readonly_simple`. The comment segment's "base command" is the literal token `#`, which isn't in `READONLY_COMMANDS`, so `is_readonly_simple` returns `false` and the whole compound denies.

I verified by piping variants through the built binary: even `# comment\nls` fails to auto-allow. The comment poisons anything.

The shield markdown already has a "Newline as command separator" section (line 250) but nothing about shell comments. This is a Category D (missing auto-allow) — the shield is behaving correctly per its current rules; the rule is what needs to expand.

Intended outcome: a compound with `# …` no-op comment lines auto-allows iff the non-comment segments are independently read-only. Comments alone (with no real command) do NOT auto-allow — mirroring the existing "assignment-only denies" invariant.

## Approach

Treat shell comment segments (trimmed line starts with `#`) as no-ops in the same loop that already recognizes empty segments and assignment-only segments. Comments contribute a "saw_noop" signal but not a "saw_real" signal — so the existing "at least one real command" gate still fires if the compound is nothing but comments.

Notes on scope:

- **Line-level comments only.** `# …\ncmd` and `cmd\n# …` are in scope. Inline trailing comments (`ls  # note`) are word-level, not line-level, and out of scope — they would require touching `is_readonly_simple`'s tokenizer and are rare enough to not be worth the risk. If the user wants them, we can follow up.
- **Existing `#!` shebang handling is not affected** — `#!` only appears at the top of script files, not in Bash tool invocations, and no VRBX test exercises it. The `starts_with('#')` check happily treats a hypothetical `#! …` line as a comment segment, which is the correct behavior anyway.
- **`saw_assignment` is generalized to `saw_noop`.** Comments and assignment-only segments both set the same flag. Terminal check: `if saw_noop && !saw_real { return false }`. This keeps the pattern uniform rather than adding a parallel `saw_comment` flag.

## Files to modify

- **`Luz/shields/ValidateReadonlyBash-VRBX/src/lib.rs`** — extend `is_readonly_compound` (function starts at line 1685; the segment loop is at 1737-1754). Add a `starts_with('#')` skip after the empty-segment skip. Rename the `saw_assignment` local to `saw_noop` (or add a second branch that sets it). New tests go in the existing `#[cfg(test)] mod tests` block (starts at 1857), using the existing `is_approved(cmd)` dark-box helper (defined at 1861).
- **`Luz/shields/ValidateReadonlyBash-VRBX.md`** — extend the "Newline as command separator" section (line 250-252) with a paragraph describing shell comment lines as no-op segments, plus ALLOW/DENY examples. The markdown is both the LLM prompt for crash-fallback and the requirements document; both roles need the new rule.

## Reused existing utilities

- `is_approved(cmd)` and `is_approved_with_cwd(cmd, cwd)` helpers at `lib.rs:1861-1867` — dark-box wrappers around `run(&ProgramInput { … })`. All new tests go through these.
- `split_on_shell_operators` at `lib.rs:1759` already splits on `\n`; no changes there.
- The existing `strip_env_assignments` + `saw_assignment`/`saw_real` bookkeeping pattern at `lib.rs:1732-1754` — the comment case slots into the same loop.

## RFIGA plan

Single vertical slice — one coherent behavior change ("`# …` lines are no-op segments in the compound loop").

1. Treat shell-comment lines as no-op segments in the compound loop.
   * **R**: Add tests to `Luz/shields/ValidateReadonlyBash-VRBX/src/lib.rs` under `mod tests`, using `is_approved(cmd)`:
     * ALLOW: `approve_leading_comment_then_ls` — `"# c\nls"`
     * ALLOW: `approve_leading_comment_then_grep_pipeline` — the actual request-797 command verbatim
     * ALLOW: `approve_trailing_comment_after_ls` — `"ls\n# c"`
     * ALLOW: `approve_interior_comment_between_commands` — `"ls\n# c\ngit status"`
     * ALLOW: `approve_multiple_leading_comments` — `"# c1\n# c2\nls"`
     * ALLOW: `approve_comment_with_leading_whitespace` — `"  # c\nls"` (per-line indented comment)
     * DENY: `deny_comments_only_no_real_command` — `"# c"` alone
     * DENY: `deny_comments_only_multiline` — `"# c1\n# c2"` (still no real command)
     * DENY: `deny_leading_comment_then_rm` — `"# c\nrm /etc/passwd"` (comment must not launder a following destructive segment; regression guard)
   * **F**: `cargo test --manifest-path Luz/shields/ValidateReadonlyBash-VRBX/Cargo.toml > tmp/vrbx-comment-line-support.txt 2>&1`; confirm the ALLOW tests fail (segment `#` denies) and the DENY tests already pass. Report: "Tests are correctly failing, proceeding with implementation."
   * **I**: In `is_readonly_compound` at `lib.rs:1685`, in the segment loop at 1737-1751:
     - Rename `saw_assignment` → `saw_noop` (or keep as `saw_assignment` and add a second branch — implementor's call for readability).
     - After the `trimmed.is_empty()` skip, add:
       ```rust
       if trimmed.starts_with('#') {
           saw_noop = true;
           continue;
       }
       ```
     - Update the terminal check to use `saw_noop` uniformly.
   * **G**: Re-run the same `cargo test` command into the same tmp file; confirm all new ALLOW tests pass and all DENY tests still pass.
   * **A**: Re-run the full VRBX test suite (`cargo test --manifest-path Luz/shields/ValidateReadonlyBash-VRBX/Cargo.toml`) — confirm no regressions in the existing ~300+ tests.

## Documentation update (part of slice 1, applied together)

Extend `Luz/shields/ValidateReadonlyBash-VRBX.md` "Newline as command separator" section (line 250-252) with:

- New paragraph: "A segment whose first non-whitespace character is `#` is a shell comment line — a no-op that runs no command. Comment segments are skipped when validating the compound, subject to the same 'at least one real command' invariant as assignment-only segments: a command that is nothing but comments does NOT auto-allow."
- ALLOW examples: `# note about the next command\ngrep foo bar.txt | sort | uniq`, `ls\n# reminder`, `# step 1\nls\n# step 2\ngit status`
- Not-auto-allowed examples: `# just a note` (comments only, no command), `# note\nrm /etc/passwd` (real command still mutates)

## Verification

End-to-end (post-implementation, not in plan-mode):

1. **Unit tests** — `cargo test --manifest-path Luz/shields/ValidateReadonlyBash-VRBX/Cargo.toml > tmp/vrbx-comment-line-support.txt 2>&1` — new tests green, no regressions in existing suite.
2. **Direct binary check** — re-run the request-797 command through the built binary:
   ```bash
   python3 -c 'import json, subprocess; …' # (same one-shot I used during diagnosis)
   ```
   Expect `{"violations":[],"auto_allow":"Read-only command auto-approved"}`.
3. **Guardian `test-shield`** (optional but recommended, per the `guardian-diagnose` skill Phase 6):
   ```bash
   OPENROUTER_API_KEY=$(cat Guardian/api_key.txt) Guardian/target/debug/guardian test-shield \
     --shield Luz/shields/ValidateReadonlyBash-VRBX.md \
     --config FrontendRust/guardian.toml \
     --cache-dir /tmp/guardian-cache --log-level overview
   ```
   Confirm shield-level tests still pass (this exercises the markdown's LLM crash-fallback path too).

## Non-goals

- Inline trailing comments (`ls  # note`) — word-level, out of scope for this change.
- Any other Bash shape that currently fails to auto-allow — this fix targets exactly the shell-comment-line case.
- Changing the shield's `g_primary: rust` or any pipeline behavior.
