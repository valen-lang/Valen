---
name: guardian-curate
description: Review shield disagreements, refine shield prompts, and promote cases to the curated tests/ corpus. Invoke periodically (typically weekly) to improve shield accuracy.
argument-hint: [optional: shield name or path to focus on, defaults to all shields]
---

# Curate Shields

Review disagreement cases, refine shield prompts, fix Rust companion programs, and promote curated cases to `tests/`. This is the human-initiated feedback loop described in `Guardian/docs/shield-feedback-loop-spec.md`.

## Step 1: Triage Opus Disagreements

For each shield with cases in `disagreements/opus/`:

1. Read each case's `case-N-input.txt` (the contextified diff Claude saw) and `case-N-context.json` (metadata including temp_disable_reason).
2. Present **one case at a time** to the human with context: what the shield decided, why Claude disagreed. Include your assessment and **present labeled options** for the human to choose from. Always include at least these options:
   - **(A) Opus was right** (shield was wrong) → move the case to `disagreements/human/` for shield refinement
   - **(B) Delete** — uninteresting case, not worth keeping
   - **(C) Opus was wrong** (shield was right) → move the case to `tests/` as a confirmed-correct example
   - **(D) Permanently disable** — add a `Guardian: disable: <SHIELD_CODE>` directive inside the function's post-block-comment (`/* ... */`), preferably above the Scala source within that comment, then delete the case
3. **Wait for the human's feedback before presenting the next case.** Do not batch multiple cases together.

## Step 2: Refine Shield Prompt

Review `disagreements/human/` cases (including any just moved from Step 1).

**Required reading:** Before this step, read `Guardian/README.md` (especially the `## Exceptions` section) to understand the available mechanisms.

1. Read each case and the current shield prompt.
2. Present your recommended fix and **labeled options** for how to address it:
   - **(A) Add a clarification** — add text to the shield's `# Clarifications` section to help the LLM avoid this false positive class
   - **(B) Add an exception** — add a lettered entry to the shield's `# Exceptions` section (see `Guardian/README.md`). Exceptions run as a second LLM pass that auto-dismisses matching violations. Better for broad categories of false positives.
   - **(C) Both** — add a clarification and an exception
   - **(D) Skip** — case doesn't warrant a shield change right now
3. Wait for the human's feedback before proceeding.
4. Goal: clarify the shield instructions to eliminate the class of error each case represents.

**Important:** Only humans edit shield files. Present proposed changes and let the human approve.

## Step 3: Validate Prompt Changes

Run the updated shield prompt against the `disagreements/human/` cases using `check-direct`:

```bash
cd /Volumes/V/Sylvan && \
Guardian/target/debug/guardian check-direct \
  --input <case-N-input.txt> \
  --referenced-defs <case-N-referenced_defs.txt> \
  --file-path <file_path from case-N-context.json> \
  --check <shield_path> \
  --cache-dir /tmp/guardian-cache \
  --backend claude \
  --log-dir /tmp/guardian-logs \
  --format human \
  --log-level overview
```

If `case-N-referenced_defs.txt` doesn't exist (older cases), create an empty file and use that.

Report results to the human. Iterate on the prompt until satisfied.

## Step 4: Promote to Tests

Opus and human decide which `disagreements/human/` cases should move to `tests/`.

- **Cluster by code pattern before promoting.** Two cases are "the same pattern" if they trigger the shield for the same underlying reason on structurally similar code. For example, three cases where the shield false-positives on `assert!` because it thinks `assert!` is stripped in release builds are all the same pattern — keep 1-2, not all three. But a case where the shield false-positives on `assert!` vs one where it false-positives on `.unwrap()` are different patterns, even though both involve fail-fast.
- **Cap at ~6 examples per pattern.** If `tests/` already has 4 cases of the same pattern and you're promoting 3 more, pick the 2 most distinct and drop the rest. The goal is enough examples to anchor the optimizer without redundancy.
- **Distribute across odd and even case numbers** — odd cases are training data for the optimizer, even cases are held-out evaluation. Aim for roughly equal distribution so the optimizer has both training examples and unseen test examples for each pattern.
- Move selected cases to `tests/`, delete the rest from `disagreements/human/`.

## Step 5: Validate Rust Program Against Tests

If the shield has a companion Rust program, run it through all `tests/` cases:

```bash
for f in <shield_dir>/tests/case-*-input.txt; do
  echo "=== $f ==="
  cat "$f" | <program_binary>
done
```

Compare outputs against `case-N-expected.json`. Any failures → add to `disagreements/rust/`.

## Step 6: Fix Rust Program

If `disagreements/rust/` has cases, process them one by one:

1. **Re-run through shield LLM.** If the LLM now agrees with Rust (prompt was updated in step 2), delete the case.
2. **Re-run through Rust program.** If Rust now passes (program was already updated), delete the case.
3. **If Rust still fails and disagrees with the LLM:** Propose a fix to the Rust program. Ask the human to approve. Implement the fix. Run the fixed version through all `tests/` cases to catch regressions.
4. **Ask the human** whether this case should be moved to `tests/`.

## Notes

- Shield files can be anywhere — check `guardian.toml` for configured shield paths. The companion directory is always next to the shield file (strip `.md`, that's the companion dir).
- The `tests/` directory uses odd/even train/test split for the optimizer.
- This skill should be run from the Sylvan repo root.
- Only the human edits shield files and approves Rust program changes.
- When moving cases between directories, preserve the `case-N-input.txt` + `case-N-expected.json` (or `case-N-context.json`) pairs. Renumber if needed.
- **Ignore shields under `tests/` directories** (e.g. `Guardian/tests/sandbox-final/`). Only process shields from the active project shield paths configured in `guardian.toml`.
