# Documentation Reorganization TODO

We are migrating all documentation to the structure defined in `docs/meta.md`. To process a file, use the `good-doc` skill: read it, categorize its content (background / usage / arcana / shields / migration / architecture / reasoning / skills), write each part to the correct `docs/` directory, add `@ID` references at code sites for any arcana, then delete the original.

## Current state (2026-05)

The **infrastructure is built and ahead of this checklist**:

- **`Luz/manifest-sync/`** — Rust tool that parses `g_`-prefixed doc frontmatter and regenerates `CLAUDE.md` `## SEE ALSO` soft-mention blocks, `.claude/rules/` auto-load symlinks, and `.claude/skills/` symlinks. This is the realized form of what was once planned as "Guardian docs rebuild/check/list" (the old `Guardian/doc-design-doc.md` is retired).
- **`docs/meta.md`** — matured into the full spec: context-cost principles, the `g_` frontmatter table, `Luz/arcana/` placement, cross-reference chains.
- **Skills (#8)** — source of truth in `docs/skills/` (19 docs), surfaced via `.claude/skills/<name>/SKILL.md` symlinks. The `/document` skill became `good-doc`.
- **`Luz/`** — reorganized: `Luz/zen/ → Luz/arcana/`; 47 shields + 7 arcana + skills + manifest-sync + hooks.
- **`FrontendRust/docs/`** — populated: 6 arcana, architecture/background/reasoning docs, 4 Sylvan-specific shields.
- **`FrontendRust/zen/`** — **dissolved** (this pass): shield dupes deleted, `PPSPASTNZ` → `src/postparsing/docs/arcana/`, migration docs relocated/retired, dead `NOT WORKING` python parity scripts + 77MB stale `logs/` removed.

Remaining work is the **cleanup tail** below.

---

## Remaining work

### `.claude/rules/` — convert to docs + auto-load symlinks (not started)

These 12 are still hand-written real files, not docs-with-symlinks. Move each into the right pass `docs/` (usage or architecture), give it `g_auto_load_when_editing` frontmatter, and let `manifest-sync` generate the `.claude/rules/` symlink. Watch for overlap with newer docs (esp. interning).

- [ ] `general/frontendrust-build-test.mdc` → `FrontendRust/docs/usage/build-test.md`
- [ ] `general/interning-patterns.mdc` → `FrontendRust/docs/usage/` (check overlap with arena/interning arcana)
- [ ] `general/no-unsolicited-restructuring.mdc` → shield, or keep Claude-only
- [ ] `general/style-guide.mdc` → `FrontendRust/docs/usage/style.md`
- [ ] `migration/frontendrust-migration-context.mdc` → `FrontendRust/docs/migration/`
- [ ] `migration/solver-migration.mdc` → `src/solver/docs/migration/`
- [ ] `parser/parser_impl_scala_rust_mapping.mdc` → `src/parsing/docs/migration/`
- [ ] `postparser/postparser-migration-guidelines.mdc` → `src/postparsing/docs/migration/`
- [ ] `postparser/postparser-organization-map.mdc` → `src/postparsing/docs/architecture/`
- [ ] `postparser/postparser_impl_scala_rust_mapping.mdc` → `src/postparsing/docs/migration/`
- [ ] `postparser/IDEPFL-postparser-interning.md` → `src/postparsing/docs/usage/` or `architecture/`
- [ ] `postparser/postparser-migration.md` → `src/postparsing/docs/migration/differences.md`

### Migration doc pruning

- [ ] `FrontendRust/docs/migration/` has **20 `handoff-slab-*` / handoff docs** — ephemeral migration handoffs that should be pruned as their work lands (meta.md: ephemeral plans shouldn't accumulate).
- [ ] `FrontendRust/docs/reasoning/migration-strategies.md` (relocated this pass) needs curation: Part 2 describes the stale 4-arena model; Parts 5–8 + appendix duplicate Luz shields. Trim to the unique Part 1/3 narrative.

### FrontendRust loose files

- [ ] `FrontendRust/todo.md`
- [ ] `FrontendRust/cursor_setup.md`
- [ ] `FrontendRust/thoughts.md`
- [ ] `FrontendRust/manual/building.md` → `FrontendRust/docs/usage/building.md`
- [ ] `FrontendRust/src/gripes.md`

### Skills

- [ ] 1 of 19 `.claude/skills/*/SKILL.md` is still a real file (not a `docs/skills/` symlink) — migrate it for consistency.

### CLAUDE.md content audit (Phase 9)

- [ ] Audit every `CLAUDE.md` for inlined content that belongs in a categorized doc. Root `./CLAUDE.md` still inlines operational guidance (Bulk Sed Protocol, Build & Run Convention, lifetime model) — decide keep vs. move-to-doc + `@`-reference.

### Guardian config decision

- [ ] `guardian.toml` still uses `include_shields` (3 mode sections) + coverage check. The earlier plan called for removing it in favor of auto-discovery from `shields_dirs`. Confirm whether to keep explicit lists (current) or move to auto-discovery, and record the decision.

### Sylvan project-level docs (lower priority — Vale language docs, not migration)

- [ ] Triage the loose Vale design docs in `docs/` (`Architecture.md`, `Environments.md`, `Generics.md`, `Catalyst.md`, `Allocators.md`, etc.) and the `docs/old/`, `docs/notes/`, `docs/refactor-thoughts/`, `docs/historical/` directories.
- [ ] Top-level: `README.md` (stays), `architectural-direction.md`, `build-compiler.md`, `compiler-overview.md`.

### Luz shields triage (mostly settled)

- [ ] Spot-check that remaining `Luz/shields/` are genuinely project-agnostic; the Sylvan/arena-specific ones already moved to `FrontendRust/docs/shields/` (AASSNCMCX, ATDCX, MIMBEX, TFITCX).
