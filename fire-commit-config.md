# Fire-Commit Config — Vale1

**External repos:**
- `Luz/` (gitignored) — own remote (`Verdagon/Luz`), always lands on `main`. Drain any `Luz/shields/*/cases/need-*/` curate queue (`guardian-curate`) before committing.
- `Guardian/` (gitignored) — own remote (`Verdagon/Guardian`), always lands on `main`. Has its own submodules (Rabble, ShieldFile, ContextifiedDiff, ContextifiedShield, opencode) — check `git -C Guardian submodule status` too.

**Test command:**
- `cargo build --manifest-path FrontendRust/Cargo.toml`
- `cargo nextest run --manifest-path FrontendRust/Cargo.toml` (full unfiltered suite, native backend)
- `VALE_TEST_BACKEND=wasi cargo nextest run --manifest-path FrontendRust/Cargo.toml` (full suite, wasm32-wasi backend) — both backends are the gate.
- When committing Guardian: `cargo nextest run --manifest-path Guardian/Cargo.toml` + each Guardian submodule's own tests.

**Branch model:** rebase-and-fast-forward, two families:
- `experimental` family — side-branches `experimental-1`, `experimental-2`, … feed local tip `experimental`, ratcheted via `git fetch . <branch>:experimental`. That local ratchet **is** the sync step; `fire commit` stops there and pushes nothing. Note the family *is* mirrored on origin (`origin/experimental`, `origin/experimental-1`, …) and those mirrors run stale as a result — pushing any of them is a separate, explicitly-requested step, and a side-branch that has been rebased will need a force-push.
- `master` family — side-branches (e.g. `repair-vale`) feed tip `master`, mirrored to `origin/master`.

Pick the family matching the working branch; ask if ambiguous.

**CI:** GitHub Actions workflow `CI` (`.github/workflows/ci.yml`) on `origin` (`Verdagon/Vale`). Jobs: `build_and_test_ubuntu`, `build_and_test_mac`, `build_and_test_docker`, `build_and_test_wasi`. Auto-triggers on push/PR to `master`/`stable`/`repair-vale`; `experimental-*` branches need manual dispatch: `gh workflow run CI --ref <branch>` then `gh run watch`. Opt-in via `with CI`.

**Repo-specific sweeps:**
- Guardian temp-disable sweep: `git grep -n "Guardian: temp-disable:"` — every hit needs ratifying (architect) or the underlying issue fixed before commit.
- Test-delta report: diff `#[test]` additions/deletions/modifications in the staged Rust diff (`git diff --cached -U0 -- '*.rs' | grep -B0 -A1 '^[+-].*#\[test\]' | grep -E '^[+-].*fn [a-zA-Z_]'`), one-sentence why per deleted/modified test, architect confirms.
- New `#[ignore]` scan in the staged diff — confirm intended-permanent vs. temporary scaffolding per hit.

<!-- NOTE: Test command uses cargo-nextest (native + wasm32-wasi). Vale2's config still lists `cargo test`; it should move to nextest too. -->
