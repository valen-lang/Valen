---
name: merging-from-master
description: 'How to selectively bring commits from `master` into `experimental` — cherry-pick the wanted ones, skip the rest, resolve conflicts against experimental''s diverged decisions.'
g_read_when: Read when the architect asks what `master` has that `experimental` might want, or asks to pull specific commits from `master`.
g_mention_in:
  - CLAUDE.md
---

# Merging From Master

`master` and `experimental` diverged philosophically. Master revived 5-region backends, Fearless FFI, HGM/generations, and the `pure_height` collapser in commit `29ca93394`. Experimental deleted that apparatus. **Most master commits past the divergence are not safe for experimental.** Triage carefully; do not bulk-cherry-pick.

This is the mirror of `merging-from-experimental.md` (read it for the master-bound flow). Both directions share the same divergence — viewed from opposite sides.

## The flow

### 1. Survey from the integration branches, not the working branches

```
git log --oneline experimental..master   # what master has, experimental doesn't
git log --oneline master..experimental   # what experimental has, master doesn't
```

**Use `experimental`, never `experimental-1`/`-2`/`-N`.** Those are TL working branches with TEMP CHECKPOINTs, debugger arcs, and in-flight work that distorts the diff.

Present the master-side list to the architect with a one-line take on each commit. Do not pick anything until told — "anything we want from master?" is a survey question, not authorization.

### 2. Categorize each candidate

| Category | Disposition |
|---|---|
| **Region/FFI revival** (touches naive-rc, `--region_override`, region collapser, Fearless FFI, HGM) | **Skip.** Experimental deleted this apparatus on purpose. `29ca93394` and anything that builds on it. |
| **Language semantics** (changes what Vale programs mean, what parses, what types are legal) | **Skip by default.** The region/FFI split makes most of these incompatible with experimental. |
| **Already on experimental under a different SHA** | Skip. Check by commit title — master sometimes cherry-picks from experimental ("Cherry-pick from experimental@<sha>: …"). Comment-scrub sweeps on both sides usually mean the smaller one is subsumed. |
| **Infrastructure** (test harness, build refactor, CLI rewrites, cross-cutting bug fix, docs/comment scrubs) | Candidate. Cherry-pick, expect conflicts, verify. |

Comment-scrub commits (e.g. another `// mig:` strip pass) are usually better re-run on experimental via the original script than cherry-picked — the diffs conflict file-by-file because experimental has touched those files independently.

### 3. Cherry-pick in master's commit order

```
git cherry-pick <oldest> <next> <newest>
```

Single multi-arg invocation, not three separate ones — keeps the conflict-resolution flow linear.

**Drop unrelated churn the commit happens to carry** — CLAUDE.md auto-SEE-ALSO additions, docs/skills/* edits unrelated to the commit's purpose. Unstage with `git restore --staged --worktree <path>` before `--continue`.

### 4. Resolve conflicts against experimental's intent

The conflicts you hit are almost always because **experimental made a deliberate removal that master didn't make.** Examples from past passes:

- Experimental retired naive-rc and removed `--region_override`. Master's clap rewrite still had a `region_override` field. **Resolution:** take master's clap version wholesale (`git checkout --theirs <file>`), then strip `region_override` to match experimental's earlier cleanup.
- Experimental dropped per-test `_unsafe_fast` suffixes and removed a region-string arg from test helpers. Master added a new C-file walker that changes the helper's `extra_c` arg. **Resolution:** keep experimental's naming + helper signature, adopt master's behavior change (`&[]` instead of `&[&test_c]`).

The pattern: when the conflict spans a wholesale rewrite (e.g. clap CLI), `git checkout --theirs <file>` to take master's version, then re-apply experimental's localized cleanups by hand. When the conflict is line-by-line in stable code, resolve in-place.

`git checkout --theirs/--ours` during a merge **is not "reverting"** — it's the standard conflict-resolution path. The CLAUDE.md "never use git checkout to revert" rule does not apply here.

### 5. After each resolution: fast checks

```
cargo check --manifest-path ./FrontendRust/Cargo.toml --lib --bins
cargo check --manifest-path ./FrontendRust/Cargo.toml --tests
```

Conflict resolution often leaves callsites that pass too many or too few args (because experimental trimmed a function signature that master's callsite still uses). `cargo check` catches those immediately. Fix, `git add -u`, `git cherry-pick --continue`.

`cargo check` is the *during-resolution* gate, not the *pre-sync* gate.

### 6. Before sync: the full fire-commit test matrix

`cargo check` is not sufficient. Before reporting done or asking for `fire commit`, run the full matrix from `docs/skills/fire-commit.md` — at minimum `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml`, plus any subtree the cherry-picked commits could have rippled into (Backend extern suite for VAST/CLI-flag changes, etc.). Skipping the matrix is the failure mode that ships broken tips to `experimental`.

### 7. Don't sync until `fire commit`

Cherry-pick creates commits automatically — that's fine, the commits exist locally on your working branch. But do **not** rebase + fast-forward to `experimental` until the architect says `fire commit`. The local commits are recoverable; a premature sync is not.

## How to think about it

- **Master is a sibling, not authoritative.** Cherry-picking from it is a curated import, not a catch-up.
- **The default question is "what is experimental ahead of master on, intentionally?"** before "what is master ahead of experimental on?" If experimental took something out on purpose, master's commit touching the same area will conflict — and the resolution preserves experimental's removal, not master's re-addition.
- **Substantive commits get explicit scrutiny; cleanup/CI commits usually flow.** A "revive 5-region backends" commit needs a hard no. A "docs/CI cleanup" commit is usually fine but still gets architect sign-off.

## Common conflict-marker pitfall

`<<<<<<<` / `>>>>>>>` markers in a Rust file briefly create text that confuses tooling — most notably the SCPX shield's block-comment scanner, which sees `/*` inside backticked identifiers (e.g. `` `native/*.c` ``) on two adjacent comment lines and panics with "Nested block comment". The shield is wrong here, but it'll block your Edit calls until the markers are gone.

Workaround: ordain the session (`/guardian-ordain`) before the cherry-pick, or resolve the conflict via Bash/heredoc rather than the Edit tool. After resolution the false-positive goes away on its own; discard any resulting SCPX cases in `Luz/shields/ScalaCommentParity-SCPX/cases/` — they're transient noise, not real triage items.

## See also

- `docs/skills/merging-from-experimental.md` — the opposite direction (experimental → master). Same divergence, mirrored disposition table.
- `docs/skills/fire-commit.md` — the sync-and-push protocol that runs after cherry-picking is done; defines the full test matrix.
- `docs/skills/fire-rebase.md` — pulling experimental into a working branch (different operation; not cross-branch import).
