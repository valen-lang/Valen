# Frontend Rust - Scala to Rust Migration Project

This is a Rust compiler frontend being migrated from Scala. The project is **mid-migration** with extensive commented Scala code alongside working Rust implementations.

## Project Overview

The codebase implements a compiler frontend with parsing, post-parsing validation/transformation, and type solving. The original Scala implementation used garbage collection; the Rust version uses arena allocation with explicit lifetime management.

## Key Directories

- **`src/postparsing/`** - Post-parsing pass: validates and transforms parsed AST (actively migrating)
- **`src/solver/`** - Type solver/inference engine (actively migrating)
- **`src/interner.rs`** - String and type interning with arena-backed allocation
- **`src/postparsing/names.rs`** - Name resolution and scope management
- **`src/postparsing/function_scout.rs`** - Function signature extraction and validation
- **`src/postparsing/post_parser.rs`** - Main post-parser orchestration

## Migration Philosophy

We're doing **incremental, safe migration**. Many functions have commented-out Scala code above working (or placeholder) Rust implementations. The migration process uses systematic "slicing" to isolate and translate individual definitions.

## Lifetime Model

The Rust codebase uses **three arena lifetimes** (see `docs/background/arenas.md` for full details):

- **`'p`** - Parser arena (via `ParseArena<'p>`): interned strings, coordinates, parser AST nodes
- **`'s`** - Scout (postparser + higher_typing) arena (via `ScoutArena<'s>`): interned names, runes, imprecise names, postparser/higher-typing output nodes
- **`'ctx`** - Context/infrastructure borrows: `&'ctx ParseArena<'p>`, `&'ctx ScoutArena<'s>`, `&'ctx Keywords<'p>`

Each arena is self-contained with its own interning maps. Cross-pass data is re-interned at pass boundaries (e.g. `StrI<'p>` → `StrI<'s>` via `scout_arena.intern_str()`).

## Conventions

All rules in `.claude/rules/` are **path-targeted** and auto-load when editing relevant files. They contain:

- Scala→Rust type mappings
- Allowable differences between implementations
- Architecture and organization maps
- Style guidelines

## Migration Subagents

The codebase includes specialized subagents for systematic migration. These are autonomous agents that can be invoked using the Task tool.

### Slice Pipeline (Full Migration)
- **`slice-orchestrator`** - Orchestrates the full migration pipeline on a Rust file
- **`slice-start`** - Add `// mig:` marker comments above Scala definitions
- **`slice-rustify`** - Convert Scala-style markers to Rust-style
- **`slice-placehold`** - Generate Rust placeholder stubs
- **`slice-reconcile-mark`** - Mark old definitions as obsolete
- **`slice-reconcile-copy`** - Copy old code into stubs
- **`slice-reconcile-delete`** - Remove obsolete definitions

### Incremental Migration
- **`migration-migrate`** - Partially migrate specific Scala code sections
- **`migration-diagnoser`** - Diagnose migration issues and differences
- **`migration-check-specific`** - Check specific definitions for correctness
- **`migration-gate`** - Validate migration readiness before proceeding

### Verification
- **`agent-check-correct-loop`** - Loop-based correctness verification

All subagents are defined in `.claude/agents/` and can be invoked using the Task tool.

## Build & Test

Always run **`cargo build --lib`** after making changes. The project builds as a library.

Use **`cargo check`** for faster iteration during development.

The build may have warnings during migration - that's expected. Focus on getting it to compile first.

## Working with This Project

1. When editing postparser files, relevant lifetime and migration rules auto-load
2. Use the slice subagents for systematic translation of commented Scala code
3. Use migration subagents for incremental fixes and verification
4. Always verify builds succeed after changes
5. Respect the lifetime invariants - see the rules for guidance when rustc complains

## Agent Rules

**Never use spawned agents (the Agent tool) to make code modifications.** All edits must be made directly by the main conversation using Read/Edit/Write tools. Spawned agents may only be used for **read-only tasks**: searching, exploring, analyzing, reading files, running read-only commands. The only exception is agents defined in `.claude/agents/` which are explicitly human-written and approved for modifications.

**When spawning any agent**, the prompt must include clear instructions that the agent **must not modify any files in this project** — only the main conversation and the human are allowed to do that. Agents are free to create and read/write temporary files in `/tmp` for their own use.

## Migration Shields

These shields define the rules enforced during migration:

@../../Luz/shields/NoValidSimplifications-NVSEX.md
@../../Luz/shields/RustShouldMirrorScalaAsCloseAsPossible-RSMSCPX.md
@../../Luz/shields/ArenaAllocatedStructsShouldNotContainMallocdCollections-AASSNCMCX.md
@../../Luz/shields/ScalaSealedTraitsToRustEnums-SSTREX.md
@../../Luz/shields/NoExpensiveClones-NECX.md
@../../Luz/shields/SuffixWhenDealingWithMultipleStages-SWDWMSX.md
@../../Luz/shields/EnumsShouldntContainComplexData-ESCCDX.md
@../../Luz/shields/TodosAndUnimplementedCodeMustPanic-TUCMPX.md
@../../Luz/shields/MigrateAllCommentsToo-MACTX.md
@../../Luz/shields/NeverRecoverAlwaysFail-NRAFX.md
@../../Luz/shields/NoGlobalStateAnywhere-NGSAX.md
@../../Luz/shields/FailFastFailLoud-FFFLX.md
@../../Luz/shields/SameHelperCallsNoExceptions-SHCNEX.md
@../../Luz/shields/NoChangesWithoutScalaReference-NCWSRX.md
@../../Luz/shields/ImmediateInterningDiscipline-IIDX.md
@../../Luz/shields/CloserToScalaNotFurther-CSTNFX.md
@../../Luz/shields/UseUseForShortNamesNotCrateInBodies-UUSNNCBX.md

## Bulk Sed Safety Protocol

Before running any `sed` command that modifies files in bulk, **always sanity-check first**:

1. **Identify false positives in the target.** The pattern you're replacing may appear in contexts you don't intend to change:
   - **Char literals**: `'a'` looks like lifetime `'a` followed by `'`. Use `s/'a\([^']\)/'p\1/g` to skip char literals.
   - **Scala block comments**: This codebase has extensive `/* ... */` Scala code. Search inside block comments for your pattern: `python3 -c "import re; ..."` to extract comment blocks and grep within them.
   - **String literals**: Your pattern might appear inside `"..."` strings.
   - **Different semantic contexts**: e.g., `'a` in the solver directory is a local callback lifetime, NOT the interner — don't rename it.

2. **Dry-run on representative files.** Pipe through sed without `-i` and diff or grep the output:
   ```bash
   sed "s/pattern/replace/g" file.rs | grep "unexpected_thing"
   ```

3. **Check for collateral damage after the run.** For lifetime renames like `'a` → `'p`:
   - Look for duplicated params: `grep -rn "'p, 'p"` (from collapsing `'a, 'p`)
   - Look for corrupted char literals: `grep -rn "'p'"` where the original had `'a'`
   - Look for changes inside block comments that shouldn't have been touched

4. **Scope your sed precisely.** Run per-directory or per-file, not blanket across the whole repo. Different directories may need different replacements (e.g., `'a` → `'p` in parsing vs `'a` → `'s` in postparsing).

## Notes

- **Interning:** Rust interns more aggressively than Scala. This is intentional and allowed.
- **Panics:** `panic!()` placeholders are acceptable during mid-migration. Scala's `vimpl` maps to Rust `panic!`.
- **Naming:** Rust uses `snake_case` (e.g., `self_uses`) vs Scala's `camelCase` (e.g., `selfUses`).
- **Profiling:** Rust doesn't need Scala's `Profiler.frame(() => { ... })` wrappers.
