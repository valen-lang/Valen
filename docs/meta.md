# Documentation Strategy (META)

This document defines how documentation is organized across the Sylvan project. It is the canonical source of truth for documentation structure and conventions.

## Directory Layout

Every major directory (each compiler pass, the project root) can have a `docs/` subdirectory:

```
Sylvan/
  docs/                          # Project-wide docs
    meta.md                      # This file
  FrontendRust/
    docs/                        # Frontend-wide docs
      shields/
      skills/
    src/
      lexing/docs/
      parsing/docs/
      postparsing/docs/
        shields/
        skills/
      higher_typing/docs/
      solver/docs/
```

Docs live next to the code they describe. Project-wide concerns live in `Sylvan/docs/` or `FrontendRust/docs/`. Pass-specific concerns live in that pass's `docs/` directory.

## Document Categories

Every document belongs to exactly one category. The category determines where the doc lives, how it's discovered, and who its audience is.

For any category, the content lives in either a single file `docs/<category>.md` or, if there are multiple documents, in a subdirectory `docs/<category>/<topic>.md`.

### 1. Background

**Audience:** Anyone reading code in this area.

**Purpose:** General knowledge you need to understand what's going on when you encounter this feature in code. "Things you have to know to read code in this part of the codebase."

**Example:** What the two arenas are, what lifetimes mean, what interning is for.

**Discovery:** Guardian auto-imports all background docs from the current directory and all ancestor directories into the pass's `CLAUDE.md`. This means background knowledge is inherited — project-wide background is available everywhere, and pass-specific background is available within that pass.

**Location:** `docs/background.md` or `docs/background/<topic>.md`

### 2. Usage

**Audience:** Anyone writing code that interacts with this feature.

**Purpose:** How to use the feature correctly. Patterns, APIs, do's and don'ts. "Things you have to know to write code that interacts with this feature."

**Example:** How to intern a new rune type, how to allocate into an arena, how to build and freeze a struct.

**Discovery:** Symlinked into `.claude/rules/` so Claude auto-loads them when editing nearby files.

**Location:** `docs/usage.md` or `docs/usage/<topic>.md`

### 3. Arcana

**Audience:** Anyone debugging or writing code who encounters a non-obvious cross-cutting effect.

**Purpose:** Documents a local thing that has surprising, non-obvious effects elsewhere in the codebase. Each arcana has a unique ID (initialism + Z suffix) and `@ID` references at every affected code site.

**Discovery:** `@ID` comments in code point readers to the arcana doc. The doc lives in the `docs/` directory of the feature that *causes* the cross-cutting effect.

**Location:** `docs/arcana/<HammerCaseTitle>-<ID>.md` (e.g., `docs/arcana/PostParserSynthesizesParserASTNodes-PPSPASTNZ.md`)

**ID convention:** Uppercase initialism of title words, Z suffix.

### 4. Shields

**Audience:** AI agents and reviewers enforcing code quality.

**Purpose:** Enforceable rules and constraints. Each shield has a unique ID (initialism + X suffix).

**Discovery:** Guardian discovers shields by scanning for initialisms in parentheses ending in X. Guardian auto-updates the shield list in the containing directory's `CLAUDE.md` as plain markdown links with descriptions from the shield's frontmatter `description:` field.

**Location:** `docs/shields/<HammerCaseTitle>-<ID>.md` (e.g., `docs/shields/NoExpensiveClones-NECX.md`)

**ID convention:** Uppercase initialism of title words, X suffix.

**Placement:** Shields that apply to a specific feature live in that feature's `docs/shields/`. Shields that apply across all projects live in `Luz/shields/`. Only move a shield to `Luz/` if it is genuinely project-agnostic.

### 5. Migration

**Audience:** Anyone working on the Scala-to-Rust migration.

**Purpose:** Living documents tracking migration status, known differences between Scala and Rust, stubbed functions, and temporary workarounds. These are inherently ephemeral and shrink as migration completes.

**Discovery:** Lives in the relevant pass's `docs/` directory. Not auto-loaded (read on demand).

**Location:** `docs/migration.md` or `docs/migration/<topic>.md`

### 6. Architecture

**Audience:** Anyone modifying the feature's own implementation.

**Purpose:** Internal design, data flow, invariants, and implementation details that a maintainer needs to understand before changing the feature. "Things you have to know to modify this feature's internals."

**Example:** How the two-phase build-then-freeze lifecycle works inside arena allocation, how the interning dedup HashMap interacts with the bump allocator.

**Discovery:** Symlinked into `.claude/rules/` for auto-loading when editing the feature's code.

**Location:** `docs/architecture.md` or `docs/architecture/<topic>.md`

### 7. Reasoning (sub-category of Architecture)

**Audience:** Anyone wondering "why is it done this way?"

**Purpose:** Records the alternatives considered and why the current approach was chosen. Lives alongside the architecture it explains.

**Location:** `docs/reasoning.md` or `docs/reasoning/<topic>.md`

### 8. Skills

**Audience:** AI agents executing specific processes.

**Purpose:** Step-by-step methodology for LLM-driven workflows like migration audits, slice pipelines, or batch parity checks.

**Discovery:** Lives in `docs/skills/`. Referenced by skill definitions in `.claude/skills/`.

**Location:** `docs/skills/<skill-name>.md`

### 9. Bugs

**Audience:** Anyone investigating known issues.

**Purpose:** Known bugs and limitations are documented as `#[ignore]`'d tests in the nearest `tests` directory, with explanatory comments describing the bug and expected behavior. Tests *are* the bug tracker.

**Location:** `#[ignore]`'d tests in code, not standalone documents.

### 10. Requirements

**Audience:** Anyone wondering what the system should do.

**Purpose:** Our tests serve as our requirements. They are the source of truth for what the system is expected to do. Other processes will consolidate them for the website; we don't maintain separate requirements documents.

**Location:** Tests in code, not standalone documents.

## Symlink Conventions

Categories #2 (Usage) and #6 (Architecture) are symlinked into `.claude/rules/` so Claude auto-loads them when editing nearby files. The symlink directory structure mirrors the source `docs/` structure:

```
.claude/rules/postparser/usage/interning.mdc  -->  ../../../FrontendRust/src/postparsing/docs/usage/interning.md
.claude/rules/postparser/architecture/arenas.mdc  -->  ../../../FrontendRust/src/postparsing/docs/architecture/arenas.md
```

Shields (#4) are NOT symlinked. They are listed in `CLAUDE.md` as plain markdown links with descriptions pulled from shield frontmatter, so they are visible for reference but not auto-loaded into context.

The source of truth is always the `docs/` file. The `.mdc` symlink exists only for auto-loading.

## CLAUDE.md Auto-Population

Each directory can have a `CLAUDE.md` that Guardian keeps up to date:

- **Background docs (#1):** Auto-imported from current directory and all ancestors. A file in `src/postparsing/` sees project-wide background + postparsing-specific background.
- **Shield lists (#4):** Auto-updated by scanning for X-suffix initialisms in the directory's `docs/shields/`.

## What Does NOT Get a Document

- **Inventories/catalogs** of structs, functions, or types. These are derivable from code and go stale. If needed during migration, they belong in #5.
- **Anything derivable from `git log` or `git blame`.**
- **Debugging solutions or fix recipes.** The fix is in the code; the commit message has the context.
- **Plans and proposals.** These are migration-specific (#5) and get deleted or graduated into architecture (#6) once implemented.
