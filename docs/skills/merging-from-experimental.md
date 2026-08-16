---
name: merging-from-experimental
description: 'How to selectively bring commits from `experimental` into `master` — cherry-pick the wanted ones, skip the rest, resolve conflicts against master''s diverged decisions.'
g_read_when: Read when the architect asks what `experimental` has that `master` might want, or asks to pull specific commits from `experimental`.
g_mention_in:
  - CLAUDE.md
---

# Merging From Experimental

> **⚠️ OUT OF DATE — do not trust the branch names below.** This guide is pending two naming
> decisions still TBD: what the AI-driven branch is named (currently `experimental`) and what the
> main branch is named (currently `master`). Until those are settled, treat every `experimental` /
> `master` reference here as a placeholder.

`experimental` and `master` diverged philosophically. Master revived 5-region backends, Fearless FFI, HGM/generations, and the `pure_height` collapser in commit `29ca93394`. Experimental deleted that apparatus. **Most experimental commits past the divergence are not safe for master.** Triage carefully; do not bulk-cherry-pick.

This is the mirror of `merging-from-master.md` (read it for the experimental-bound flow). Both directions share the same divergence — viewed from opposite sides.

## The flow

### 1. Survey from the integration branches, not the working branches

```
git log --oneline master..experimental   # what experimental has, master doesn't
git log --oneline experimental..master   # what master has, experimental doesn't
```

**Use `experimental`, never `experimental-1`/`-2`/`-N`.** Those are TL working branches with TEMP CHECKPOINTs, debugger arcs, and in-flight work that distorts the diff.

Present the experimental-side list to the architect with a one-line take on each commit. Do not pick anything until told — "anything we want from experimental?" is a survey question, not authorization.

### 2. Categorize each candidate

| Category | Disposition |
|---|---|
| **Language semantics** (changes what Vale programs mean, what parses, what types are legal) | **Skip.** Master's region/FFI revival makes most of these incompatible. |
| **Region/FFI removal** (retires naive-rc, removes `--region_override`, deletes Fearless FFI, etc.) | **Skip.** Master needs the apparatus experimental deleted. |
| **Already on master under a different SHA** | Skip. Check by commit title — porting was often done piecemeal (e.g. `52e6bfaaf` on experimental ≡ `809870010` on master). |
| **Infrastructure** (test harness, build refactor, cross-cutting bug fix, docs/comment scrubs) | Candidate. Cherry-pick, expect conflicts, verify. |

Comment-scrub commits (`// mig:` strip, `/* scala */` strip, etc.) are usually better re-run on master via the original script than cherry-picked — the diffs conflict file-by-file because master has touched those files independently.

### 3. Cherry-pick in experimental's commit order

```
git cherry-pick <oldest> <next> <newest>
```

Single multi-arg invocation, not several separate ones — keeps the conflict-resolution flow linear.

**Drop unrelated churn the commit happens to carry** — CLAUDE.md auto-SEE-ALSO additions, docs/skills/* edits unrelated to the commit's purpose. Unstage with `git restore --staged --worktree <path>` before `--continue`.

### 4. Resolve conflicts against master's intent

The conflicts you hit are almost always because **master kept structure that experimental removed.** Examples from past passes:

- Experimental cut `IRuneS::PureBlockRegionRune` (region apparatus removed). Master kept the arm. **Resolution:** take experimental's substance (real humanizer bodies replacing `panic!("implement: ...")` arms, brace-style with commented-out implementations) while preserving master's `PureBlockRegionRune` arm at the end of the match.
- Experimental switched a private helper chain from `HashSet`/`HashMap` to `IndexSet`/`IndexMap` to fix solver nondeterminism. Master had already done part of the chain. **Resolution:** take experimental's signatures wholesale — finishes the chain master started.

The pattern: when the conflict spans a wholesale rewrite, take experimental's version then graft master-only variants back in. When it's line-by-line in stable code, resolve in-place.

`git checkout --theirs/--ours` during a merge **is not "reverting"** — it's the standard conflict-resolution path. The CLAUDE.md "never use git checkout to revert" rule does not apply here.

### 5. After each resolution: fast checks

```
cargo check --manifest-path ./Cargo.toml --lib --bins
cargo check --manifest-path ./Cargo.toml --tests
```

Conflict resolution often leaves callsites that pass too many or too few args (because experimental trimmed a function signature that master's callsite still uses). `cargo check` catches those immediately. Fix, `git add -u`, `git cherry-pick --continue`.

`cargo check` is the *during-resolution* gate, not the *pre-sync* gate.

### 6. Before sync: the full fire-commit test matrix

`cargo check` is not sufficient. Before reporting done or asking for `fire commit`, run the full matrix from `docs/skills/fire-commit.md` — at minimum `cargo nextest run --manifest-path ./Cargo.toml`, plus any subtree the cherry-picked commits could have rippled into.

**The load-bearing end-to-end probe for master is VerdagonSite.** Build valec (debug — release trips a pre-existing MIR-cycle ICE), use it to rebuild vmdsitegen with `--region_override resilient-v3`, then render all 71 pages via `bash build.sh build all ...`. See `VerdagonSite/README.md` for exact invocation. Zero `fail|error|panic` in the build log + 71 output pages = green.

### 7. Don't sync until `fire commit`

Cherry-pick creates commits automatically — the commits exist locally on your working branch. But do **not** fast-forward `master` or push to `origin master` until the architect says `fire commit`. The local commits are recoverable; a premature push to `master` is not.

When ready, the landing wrinkle: master is not on the `experimental → experimental-N` integration flow. Commit on a feature branch off `origin/master`, FF `master` locally, push `origin master`. Watch for non-FF rejection — local `master` may already be ahead of `origin/master` from a prior session; rebase your branch onto local `master` first (and re-verify after).

## How to think about it

- **Experimental is a sibling, not authoritative.** Cherry-picking from it is a curated import, not a catch-up.
- **The default question is "what is master keeping, intentionally?"** before "what is experimental ahead of master on?" If master kept something on purpose (the region apparatus, the FFI surface), experimental's commit touching the same area will conflict — and the resolution preserves master's structure, not experimental's removal.
- **Substantive commits get explicit scrutiny; cleanup/CI commits usually flow.** A "retire region X" commit needs a hard no. A "humanizer arm fill" commit is usually fine but still gets architect sign-off.

## Common conflict-marker pitfall

`<<<<<<<` / `>>>>>>>` markers in a Rust file briefly create text that confuses tooling — most notably the SCPX shield's block-comment scanner, which sees `/*` inside backticked identifiers on adjacent comment lines and panics with "Nested block comment". The shield is wrong here, but it'll block your Edit calls until the markers are gone.

Workaround: ordain the session (`/guardian-ordain`) before the cherry-pick, or resolve the conflict via Bash/heredoc rather than the Edit tool. After resolution the false-positive goes away on its own; discard any resulting SCPX cases in `Luz/shields/ScalaCommentParity-SCPX/cases/` — they're transient noise, not real triage items.

## See also

- `docs/skills/merging-from-master.md` — the opposite direction (master → experimental). Same divergence, mirrored disposition table.
- `docs/skills/fire-commit.md` — the sync-and-push protocol that runs after cherry-picking is done; defines the full test matrix.
- `docs/skills/fire-rebase.md` — pulling experimental into a working branch (different operation; not cross-branch import).
