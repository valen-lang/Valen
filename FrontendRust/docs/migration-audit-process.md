# Migration Audit Process: Batch Parity Checking

This document describes the process we used to audit 30 Scala-to-Rust migrated definitions for parity, using the `migration-check-specific` subagent in parallel waves.

## Overview

After a large migration diff touching multiple files (higher_typing_pass, rune_type_solver, identifiability_solver, tests, etc.), we needed a systematic way to verify each changed function matched its Scala counterpart. Rather than reviewing everything manually, we used the read-only `migration-check-specific` agent to audit each definition independently and in parallel.

## Step 1: Identify Changed Functions

We started by listing all changed functions across the diff. The initial scan produced ~50 definitions. To focus agent time on where it matters, the user asked to trim the list to ~30 by removing:

- **Obviously correct definitions** — simple delegations, trivial wrappers, or functions that were clearly 1:1 translations with no room for error
- **Functions 3 lines or less** — too small to have meaningful parity issues

After this triage, ~30 definitions remained across 7 files:

- `higher_typing_pass.rs` — 15 functions
- `higher_typing_pass_tests.rs` — 6 tests
- `rune_type_solver.rs` — 5 functions
- `post_parser.rs` — 1 function
- `identifiability_solver.rs` — 2 functions
- `rule_scout.rs` — 2 code blocks
- `templex_scout.rs` — 1 code block

## Step 2: Plan Waves

To avoid overwhelming the system, we grouped agents into 3 waves by file:

- **Wave 1 (15 agents):** All `higher_typing_pass.rs` functions
- **Wave 2 (11 agents):** Tests + `rune_type_solver.rs` functions + a re-check of `solve_stub` (which was in the wrong file in Wave 1)
- **Wave 3 (5 agents):** `post_parser.rs`, `identifiability_solver.rs`, `rule_scout.rs`, `templex_scout.rs`

## Step 3: Launch Agents

Each agent was launched via the Agent tool with `subagent_type: "migration-check-specific"` and `run_in_background: true`. The prompt for each was minimal:

```
File: FrontendRust/src/<path>, Definition: `<name>`
```

The agent's own system prompt (in `.claude/agents/migration-check-specific.md`) handles everything: reading the Rust code, finding the corresponding Scala source, checking against the migration principles (RSMSCP, MACT, TUCMP, DCCR, etc.), and returning APPROVED / NEEDS_WORK / QUESTION.

All agents within a wave were launched in a single message to maximize parallelism.

## Step 4: Collect Results

As each agent completed (via background task notifications), we recorded:
- The verdict (APPROVED / NEEDS_WORK)
- A brief summary of the issues found

One agent (`solve_stub`) couldn't find its definition in the specified file, so we re-launched it with the correct file path in Wave 2.

## Step 5: Assemble Summary

After all 30 agents completed, we compiled a final report organized by:
- **APPROVED** definitions (4 total)
- **NEEDS_WORK** definitions (26 total), grouped by file
- **Common themes** across all findings

## Results Summary

- **4 APPROVED** (13%): These matched Scala closely enough
- **26 NEEDS_WORK** (87%): Various parity violations found

### Most Common Issues Found

1. **Match arm ordering** (5 functions) — Rust match arms in different order than Scala
2. **Control flow structure** (4 functions) — if-let/if-else where Scala uses match statements
3. **Missing error handling** (4 functions) — panic! where Scala returns proper error types
4. **Style: long `crate::` paths** (5 tests) — should use `use` imports instead
5. **Missing comments** (3 functions) — MACT violations
6. **Missing parameters** (3 functions) — parameters like `primitives`, `use_optimized_solver` dropped from signatures

## Timing

- Wave 1 (15 agents): ~3 minutes for all to complete
- Wave 2 (11 agents): ~2 minutes
- Wave 3 (5 agents): ~3 minutes
- Total wall-clock time: ~10 minutes for 30 audits

## Lessons Learned

1. **File accuracy matters.** One agent was sent to the wrong file and returned "not found" — caught and re-launched quickly, but worth double-checking file paths up front.

2. **The agents are thorough but sometimes strict.** Some "NEEDS_WORK" verdicts were for relatively minor style issues (like `crate::` paths in tests). Grouping findings by severity would help prioritize.

3. **Parallelism works well.** Launching 15 agents simultaneously caused no issues. The background notification system made it easy to track completion without polling.

4. **Common themes emerge.** Many issues (match arm ordering, missing comments, control flow shape) appeared across multiple functions, suggesting systematic patterns in how the migration was done rather than one-off mistakes.

## How to Repeat This Process

1. Identify changed definitions from `git diff` or code review
2. Group into waves of 10-15 agents
3. Launch each wave with the Agent tool:
   ```
   subagent_type: "migration-check-specific"
   prompt: "File: FrontendRust/src/<path>, Definition: `<name>`"
   run_in_background: true
   ```
4. Track results as notifications arrive
5. Compile summary report with verdicts and common themes
