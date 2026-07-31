# Plan document

Source: `/Users/verdagon/.claude/plans/please-plan-out-the-stateless-horizon.md`
Session: d220ff6a-c0d7-4077-810d-afa51c111962

---

# Plan: Extend manifest-sync to register per-pass `docs/skills/` files

## Context

`Luz/manifest-sync/` is a Rust tool that walks the Vale repo and generates `CLAUDE.md ## SEE ALSO` blocks, `.claude/rules/*.mdc` files, and `.claude/skills/<name>/SKILL.md` symlinks from Markdown doc frontmatter.

Its **skills-registration logic** (`src/skills.rs`) is inconsistent with everything else:

- **`walker::walk` + `see_also` + `rules`** already scan the whole tree (respecting `.gitignore` and skipping nested `.git` repos + `.claude/`). Per-pass docs like `FrontendRust/src/typing/docs/architecture/*.md` correctly contribute to their nearest `CLAUDE.md`'s SEE ALSO.
- **`skills::repair`** hardcodes `root.join("docs").join("skills")` as the single source directory. Per-pass files like `FrontendRust/src/typing/docs/skills/typing-reviewer.md`, `FrontendRust/docs/skills/valec-reviewer.md`, and any future ones are silently ignored — they exist on disk but never get a `.claude/skills/<name>/SKILL.md` symlink, so `/typing-reviewer` is not invocable as a slash-command.

The user hit this while pointing at `typing-reviewer.md` and asking "shouldn't manifest-sync just work?" The answer is that skills is the odd one out; the fix is to make it consistent with walker/see_also/rules.

**Intended outcome:** any `.md` file under any `<subpath>/docs/skills/` directory becomes an auto-registered slash-command skill, with the correct relative symlink target computed from its actual path. Root-level `docs/skills/` continues to work unchanged.

## Approach

**One deep change**, isolated to `skills.rs` (with one new helper in `walker.rs` and one new error variant in `lib.rs`).

### 1. Add `walker::find_skill_files`

Add a sibling function to `walker::walk` in `Luz/manifest-sync/src/walker.rs`:

```rust
/// Walk the tree under `root` and return absolute paths of every `.md`
/// file whose parent directory chain ends in `.../docs/skills/`. Unlike
/// `walk`, this returns paths regardless of frontmatter — mirroring the
/// current semantic that any file in `docs/skills/` is a skill.
///
/// Same exclusion rules as `walk`: respects `.gitignore`, skips nested
/// git repos, skips `.claude/`.
pub fn find_skill_files(root: &Path) -> Result<Vec<PathBuf>, SyncError>
```

Implementation reuses the same `WalkBuilder` + `filter_entry` shape from `walker::walk`. The final filter checks that the file's *parent* is named `skills` and *grandparent* is named `docs` (i.e., the path ends in `.../docs/skills/name.md`).

**Why not extend `walk` itself?** `walk` filters to frontmatter-bearing files (used by see_also + rules). Skills-registration deliberately doesn't require frontmatter (see `typing-reviewer.md`, which has none). Keeping them separate matches the existing semantic split.

### 2. Refactor `skills::repair`

Currently: iterate `root/docs/skills/*.md`, then iterate `.claude/skills/*/`.

Replace step 1 with:

```rust
let skill_files = walker::find_skill_files(root)?;
let mut by_name: HashMap<String, PathBuf> = HashMap::new();
for path in skill_files {
    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    if let Some(existing) = by_name.get(&name) {
        return Err(SyncError::SkillNameCollision {
            name,
            first: existing.clone(),
            second: path,
        });
    }
    by_name.insert(name, path);
}
```

Then for each `(name, source_path)`:
- Ensure `.claude/skills/<name>/` exists (same as today).
- Compute symlink target as `PathBuf::from("../../..").join(source_path.strip_prefix(root))`. This yields:
  - `../../../docs/skills/foo.md` for root-level (identical to today's hardcoded string).
  - `../../../FrontendRust/src/typing/docs/skills/typing-reviewer.md` for per-pass.

Step 2 (iterate `.claude/skills/*/` and check each symlink) also needs the update: the "missing doc" check must look up `name` in the `by_name` map rather than checking `docs/skills/<name>.md` directly.

### 3. Add `SyncError::SkillNameCollision`

In `Luz/manifest-sync/src/lib.rs`:

```rust
#[error("skill name {name:?} appears in two source docs: {first:?} and {second:?}")]
SkillNameCollision {
    name: String,
    first: PathBuf,
    second: PathBuf,
},
```

## Files to modify

- `Luz/manifest-sync/src/walker.rs` — add `find_skill_files` helper (~40 lines).
- `Luz/manifest-sync/src/skills.rs` — refactor `repair` to use it (~30 lines net delta after replacing the current dir-scan).
- `Luz/manifest-sync/src/lib.rs` — add `SkillNameCollision` error variant (~7 lines).
- `Luz/manifest-sync/tests/sync_tests.rs` — add 4 new tests (fixture pattern is well-established).

## Reused existing utilities

- `walker::walk`'s `WalkBuilder + filter_entry` shape (mirror it in `find_skill_files`).
- `skills::repair`'s existing "check symlink, repair if wrong" logic (unchanged).
- `Fixture::{write, read, options}` helpers in `sync_tests.rs`.
- `sync_tests.rs::skill_missing_symlink_is_created` (line 217) — template for the happy-path test.
- `sync_tests.rs::skill_with_correct_symlink_is_noop` (line 253) — template for the idempotency test.

## RFIGA (single slice)

Adding per-pass skill discovery is one coherent behavior change (a broader source-set for the same registration action). All four tests belong to it.

1. **Per-pass `docs/skills/` files register as `.claude/skills/<name>/SKILL.md` symlinks with correct relative targets, without regressing root-level behavior, and with collisions reported loudly.**
   - **R** — add four tests to `Luz/manifest-sync/tests/sync_tests.rs` in the "Skill symlinks" section:
     - `per_pass_skill_registered` — writes `FrontendRust/src/typing/docs/skills/typing-reviewer.md` (no frontmatter needed), runs sync, asserts `.claude/skills/typing-reviewer/SKILL.md` is a symlink whose `read_link()` equals `../../../FrontendRust/src/typing/docs/skills/typing-reviewer.md`.
     - `per_pass_skill_idempotent` — same setup, but pre-creates the correct symlink; asserts `report.symlinks_updated.is_empty()`.
     - `nested_git_repo_skills_ignored` — writes `Sub/.git` marker file + `Sub/docs/skills/foo.md`, runs sync, asserts `.claude/skills/foo/` was NOT created (mirrors walker's nested-repo skip).
     - `skill_name_collision_is_error` — writes `docs/skills/dupe.md` AND `FrontendRust/docs/skills/dupe.md`, expects `SyncError::SkillNameCollision { name: "dupe", .. }`.
   - **F** — `cargo test --manifest-path /Volumes/V/Vale2/Luz/manifest-sync/Cargo.toml > /Volumes/V/Vale2/tmp/manifest-sync-fire.txt 2>&1`. Report to user: "Tests are correctly failing, proceeding with implementation." Expected failure modes: (a) `per_pass_skill_registered` fails because `.claude/skills/typing-reviewer/` never gets created (current logic only scans root `docs/skills/`); (b) collision test fails because current logic doesn't detect duplicates; (c) other two are follow-on assertions on that same missing code path.
   - **I** — three edits: (i) new `find_skill_files` in `walker.rs`, (ii) refactored `repair` in `skills.rs` using the helper and a HashMap-based collision check, (iii) new `SkillNameCollision` variant in `lib.rs`. Existing tests (`skill_missing_symlink_is_created` et al.) must continue to pass — they exercise the root-level path, which the new formula still handles.
   - **G** — re-run the same four tests. All pass.
   - **A** — full manifest-sync test suite: `cargo test --manifest-path /Volumes/V/Vale2/Luz/manifest-sync/Cargo.toml > /Volumes/V/Vale2/tmp/manifest-sync-fire.txt 2>&1`. Then a real-world check: run manifest-sync against the Vale repo root and verify that `/Volumes/V/Vale2/.claude/skills/typing-reviewer/SKILL.md` and `/Volumes/V/Vale2/.claude/skills/valec-reviewer/SKILL.md` now exist as symlinks pointing at their per-pass source files.

## Verification (end-to-end)

After the slice lands:

1. **Unit level:** `cargo test --manifest-path Luz/manifest-sync/Cargo.toml` — all existing tests + the 4 new ones pass.
2. **Integration level:** run manifest-sync against the real Vale repo:
   ```bash
   cargo run --manifest-path Luz/manifest-sync/Cargo.toml -- --root /Volumes/V/Vale2 > /Volumes/V/Vale2/tmp/manifest-sync-fire.txt 2>&1
   ```
   (check the crate's `main.rs` for the exact CLI shape — pass `--root` or invoke however it's normally called by the hooks). Then verify:
   - `ls -la /Volumes/V/Vale2/.claude/skills/typing-reviewer/SKILL.md` exists and is a symlink.
   - `readlink /Volumes/V/Vale2/.claude/skills/typing-reviewer/SKILL.md` prints `../../../FrontendRust/src/typing/docs/skills/typing-reviewer.md`.
   - Same for `valec-reviewer` and any `Guardian/docs/skills/*` files (Guardian is a submodule, so those SHOULD be excluded by the nested-git-repo rule — confirm they are NOT registered).
   - Existing root-level skills (e.g., `code-review`, `tdd`, `test-review`) still have correct symlinks.
3. **Functional level:** in a fresh Claude Code session, verify that `/typing-reviewer` is offered as a slash-command (the harness discovers `.claude/skills/*/SKILL.md`).

## Notes / non-scope

- **No frontmatter required on skill docs.** Preserves the current semantic that any `.md` in `docs/skills/` is a skill. `typing-reviewer.md` has no frontmatter today and this plan keeps it working as-is.
- **Frontmatter still governs SEE ALSO placement.** If the user wants `typing-reviewer.md` to appear in `FrontendRust/src/typing/CLAUDE.md`'s SEE ALSO section, that's a separate action (adding `g_read_when` + `g_mention_in` to the file) — orthogonal to this change.
- **Guardian submodule skills stay excluded.** The nested-`.git` skip logic already handles this; the new test explicitly locks it in.
- **No update to Luz's own docs.** The convention "root docs/skills only" is not documented in a load-bearing way that this change contradicts; if anything, this brings skills into line with the already-documented walker behavior.
