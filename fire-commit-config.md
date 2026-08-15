# Fire-Commit Config

**Machine facts live in the `fire-commit-config.toml` files, not here.** The host's is at
`fire-commit-config.toml` and each external carries its own at `<external>/fire-commit-config.toml`;
they declare the branch model, sweeps, test commands, and external list, and `commit-preflight`
reads them. This doc holds only the prose and judgment the `.toml` can't encode.

**Policy:** commit everything dirty in Luz and Guardian, even if it looks unrelated to this session.
This is only safe because anything *not* meant to be committed lives in each repo's gitignored `tmp/`
(host, Luz, and Guardian all ignore `tmp/`) — the wholesale `git -C <repo> add -A` never sees it.
Keep scratch, debug output, and one-off files there; `commit-preflight` lists an external's pending
files so you can spot a stray before it lands and move it to `tmp/`.

**External repos** (declared as `[[external]]` in `fire-commit-config.toml`):
- `Luz/` — before committing, drain any `Luz/shields/*/cases/need-*/` curate queue (`guardian-curate`).
- `Guardian/` — its five submodules (Rabble, ShieldFile, ContextifiedDiff, ContextifiedShield,
  opencode) are `[[external]]` entries in `Guardian/fire-commit-config.toml`, so the tool composes
  them (commits/pushes each, then bumps Guardian's pins). A dirty `bun.lock` in a submodule is just
  build-time regeneration — feel free to discard it.

**Branches** (the `.toml` says `branch_model = "rebase-ff"`; the tool can't name them):
- `main` — the trunk; the `exp-*-wipbx` working branches feed it, ratcheted via `git fetch . <branch>:main`.
- `stable` — a periodic fast-forward snapshot of `main` (`git branch -f stable main` every few
  months); not a direct commit target.

Pick the family matching the working branch; ask if ambiguous. What `temporary` withholds versus a
full commit (the target ratchet + target push) is spelled out in `Luz/skills/fire-commit.md`.

**Test commands** are the `[[repo.tests]]` in each `.toml` (host: `cargo build` + both FrontendRust
nextest backends, native and wasi; Guardian: its workspace `cargo nextest run` with the API key).
No known-environmental failures are whitelisted — treat any failure as real. `commit-preflight`
emits these as plan steps for you to run and judge; a **green suite is required unless the architect
says the literal `fire override green`.**

**CI** (not in the `.toml` — the tool doesn't gate on CI): GitHub Actions workflow `CI`
(`.github/workflows/ci.yml`) on `origin` (`valen-lang/Valen`). Jobs: `build_and_test_ubuntu`,
`build_and_test_mac`, `build_and_test_docker`, `build_and_test_wasi`. Auto-triggers on pull requests
targeting `main`, and on push to `stable`; other branches (e.g. `exp-*-wipbx`) need manual dispatch: `gh workflow run CI
--ref <branch>` then `gh run watch`. Opt-in via `with CI`.

**Sweep judgment.** The sweeps themselves (DO-NOT-SUBMIT, absolute paths, broken symlinks, Guardian
temp-disable, test-delta, new-`#[ignore]`) run from each repo's `[repo.sweeps]` config; what still
needs a human:
- Guardian temp-disable hits need ratifying (architect) or the underlying issue fixed before commit.
- Test-delta: one-sentence why per deleted / renamed / modified test; the architect confirms each.
  When running the detection by hand (rather than via the tool's `test_delta` sweep):
  - Signatures added/removed: `git diff --cached -U0 -- '*.rs' | grep -E '^[+-][[:space:]]*(pub )?(async )?fn [a-zA-Z_0-9]+'`
  - Bodies changed: `git diff --cached -U0 -- '*.rs' | grep -oE '@@ .*fn [a-zA-Z_0-9]+' | sed 's/.*fn //' | sort -u`
- New `#[ignore]`: confirm intended-permanent vs. temporary scaffolding per hit.
