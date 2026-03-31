# Using AI in This Project

A guide to all the ways you can interact with Claude and Guardian in Sylvan.

## Slash Commands (Skills)

Type these directly in Claude Code.

### Documentation
| Command | What it does |
|---|---|
| `/document` | Categorize information and write it to the correct `docs/` directories per `docs/meta.md`. Handles arcana (@ID references) and shields. |

### Migration — Driving Work Forward
| Command | What it does |
|---|---|
| `/migration-drive` | Make minimal, iterative parity-only changes. Adds `panic!` placeholders liberally. No novel logic. |
| `/migration-test-fixer` | Run a failing test, diagnose what's missing, migrate Scala code until it passes. Stops after 5 consecutive failures. |
| `/migration-test-fixer-2` | Like above but uses `migrate-director` for smarter orchestration. |
| `/slice-pipeline` | Run the full slice pipeline on a file: start → rustify → placehold → reconcile (mark/copy/delete). |

### Migration — Reviewing & Checking
| Command | What it does |
|---|---|
| `/migration-diff-review` | Review a migration diff for Scala parity, correctness, and principle compliance. Read-only audit. |
| `/migration-check-correct-loop` | Loop: check a definition for Scala parity → fix violations → run tests → repeat until APPROVED. |

### Guardian Integration
| Command | What it does |
|---|---|
| `/guardian-teach` | Process a `// VV:` violation comment. Match it to a Guardian shield or create a new one, then add a test case. |
| `/guardian-post-review` | Process `//f` annotations from a Guardian review. Validates context quality and creates disagreement cases. |
| `/guardian-curate` | Weekly curation of shield disagreements. Triage opus/ cases, refine prompts, promote cases to tests/. |

### Infrastructure
| Command | What it does |
|---|---|
| `/write-pretooluse-hook` | Step-by-step guide to build a Rust binary PreToolUse hook that can block Edit/Write/Bash calls. Covers JSON protocol, exit codes, settings.json config. |

## Agents (Spawned Internally)

These are invoked by skills or by Claude via the Agent tool. You don't call them directly, but it helps to know what they do.

### Migration Pipeline
| Agent | Role |
|---|---|
| `migrate-diagnoser` | Diagnose what missing migration causes a test failure. Writes `migrate-direction.md`. |
| `migrate-director` | Orchestrate diagnosis + scoping. Tells Claude what to implement next. |
| `migrate-scoper` | Generate implementation instructions from diagnoser findings. Internal to migrate-director. |
| `migration-migrate` | Bring over minimum Scala code to make changes compile. Uses `panic!` heavily. |

### Slice Pipeline
| Agent | Role |
|---|---|
| `slice-orchestrator` | Run the full slice pipeline by invoking subagents in sequence. |
| `slice-start` | Insert `// mig:` markers above every Scala definition in commented code. |
| `slice-start-check` | Verify `// mig:` markers were inserted correctly. Read-only. |
| `slice-rustify` | Convert Scala-style `// mig:` comments to Rust-style (def→fn, class→struct). |
| `slice-placehold` | Generate `panic!("Unimplemented: ...")` stubs below `// mig:` comments. |
| `slice-reconcile-mark` | Mark old Rust definitions as `// old, obsolete`. |
| `slice-reconcile-copy` | Copy old Rust code into matching placeholder stubs. |
| `slice-reconcile-delete` | Delete definitions marked `// old, obsolete`. |

### Quality Gates
| Agent | Role |
|---|---|
| `migration-check-specific` | Check a specific definition for Scala parity (32+ criteria). Read-only. Returns APPROVED/NEEDS_WORK. |
| `migration-gate` | Check a git diff for novel logic or structure mismatches. Read-only. Returns APPROVED/NEEDS_WORK. |
| `agent-check-correct-loop` | Loop: check → fix → test → repeat. The engine behind `/migration-check-correct-loop`. |

## Guardian

Guardian is the LLM-powered code validation system. It runs shield checks against code changes.

### As a Real-Time Hook
Guardian runs automatically when Claude edits code (configured in `FrontendRust/guardian.toml`). It checks each changed definition against active shields and blocks violations.

### CLI Commands
```bash
# Review changes against all shields
guardian review --config FrontendRust/guardian.toml --mode review_mode --base HEAD --votes 1

# Optimize a shield prompt for a weaker model using test cases
guardian optimize --shield path/to/shield.md --rounds 5 --config guardian.toml --cache-dir .cache

# Future: documentation support (see Guardian/doc-design-doc.md)
guardian docs rebuild    # Regenerate CLAUDE.md files and symlinks
guardian docs check      # Validate doc structure, arcana references, shield IDs
guardian docs list       # List all docs by category and scope
```

### Bringing In a New Shield
See `Luz/shields/GUIDE-bringing-in-a-shield.md`. Summary:
1. Add shield as only entry in `[review_mode]` in `guardian.toml`
2. Add `model:` frontmatter to shield file
3. Run single-vote review, audit results
4. Add clarifications for false positives
5. Re-run until clean, then deploy to `[guard_mode]`

### Shield and Arcana IDs
- **Shields**: uppercase initialism + **X** suffix (e.g., `NECX`, `AASSNCMCX`)
- **Arcana**: uppercase initialism + **Z** suffix (e.g., `PPSPASTNZ`)

## Migration Workflows

### "I want to migrate a whole file" (slice pipeline)
1. `/slice-pipeline` on the file
2. Review the result
3. `/migration-check-correct-loop` on specific definitions that need fixing

### "I want to get a test passing"
1. `/migration-test-fixer-2` with the test name
2. It will diagnose, migrate, and iterate until the test passes

### "I want to check if my migration is correct"
1. `/migration-diff-review` to audit the current diff
2. Or `/migration-check-correct-loop` on a specific definition for automated fix-and-verify

### "I found a code violation"
1. Add `// VV: <description>` above the definition
2. `/vv` to match it to a shield and create a test case

### "I want to document something"
1. `/document` — it will categorize and place the information per `docs/meta.md`
