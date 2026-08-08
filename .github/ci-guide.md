# CI Guide

## What GitHub CI runs

`.github/workflows/ci.yml` runs three jobs on push and PR:

1. **`build_and_test_ubuntu`**: installs LLVM 16 + clang-16, builds `FrontendRust/`, runs `cargo test`, packages a Linux release zip.
2. **`build_and_test_mac`**: same as Ubuntu but on `macos-latest` via Homebrew `llvm@16`.
3. **`build_and_test_docker`**: builds the `scripts/docker/Dockerfile` image, runs `docker run valec --version` as a smoke test.

That's the whole story. **CI does not run the Backend extern suite, does not exercise the VAST/FFI boundary, does not render any `.vmd` page, and does not validate that `valec` actually compiles a non-trivial program.** It's a compile-and-unit-test gate, not an integration gate.

## Reproducing CI locally

```
cargo build --manifest-path FrontendRust/Cargo.toml
cargo test --manifest-path FrontendRust/Cargo.toml
```

These two commands are sufficient to predict CI's pass/fail. If both succeed locally, CI will almost certainly pass (modulo platform-specific LLVM/clang link issues).

For the faster, more informative test run:

```
cargo nextest run --manifest-path FrontendRust/Cargo.toml
```

The nextest output is parallelized and per-test-timed; `cargo test` is serial and noisier. Target: **1402/1402 + 22 skipped** on master as of June 2026.

## Known gotchas

### Release build trips a MIR-cycle ICE

```
cargo build --release --manifest-path FrontendRust/Cargo.toml --bin valec
# error[E0391]: cycle detected when optimizing MIR for ...
# at src/postparsing/expressions.rs:243 PartialEq derive
```

This is a Rust compiler internal error from a `#[derive(PartialEq)]` cycle in `postparsing::expressions`. It's pre-existing and unrelated to any specific change. **Use debug builds for valec** when you need to actually run the compiler locally. CI builds debug, so this doesn't affect CI.

### CI doesn't catch schema drift between Frontend and Backend

If you rename a VAST JSON key on one side, FrontendRust unit tests pass, Backend unit tests pass, CI is green — and every compiled program is broken at runtime. The only check that round-trips a real VAST through both halves is the Backend extern suite (`Backend/test.sh`) or the broader VerdagonSite probe below. **Run one of these before pushing a schema-adjacent change.**

### CI doesn't catch CLI-flag drift

Same shape: if `valec` removes or renames a flag that `vmdsitegen`'s `build.sh` passes (`--region_override resilient-v3`, `--sanity_check false`, `--builtins_dir_override`, etc.), CI doesn't know. The VerdagonSite probe catches this immediately because `vmdsitegen` compilation fails at the first unknown flag.

## The VerdagonSite end-to-end probe

This is the load-bearing local integration check for master. It exercises: FrontendRust → `valec` binary → Vale-language compilation of a non-trivial program (`vmdsitegen`) → Backend code generation → clang link → binary execution → 71 `.vmd` page renders.

### Setup (one-time)

VerdagonSite has hard path dependencies on sibling repos. Verify they're present:

```
ls -d ../VmdSiteGen ../VmdParse ../ParseIter ../Snippet
```

`SylvanHighlighting` is only needed for *building* the `vmd-highlighter` Rust binary. The binary is usually pre-built at `../VmdSiteGen/tools/highlighter/target/release/vmd-highlighter` and can be used directly without `SylvanHighlighting` on disk.

### Run

```bash
# 1. Build valec (debug — release trips the MIR cycle).
cargo build --manifest-path FrontendRust/Cargo.toml --bin valec

# 2. Compile vmdsitegen via valec.
cd ../VmdSiteGen
../Vale4/FrontendRust/target/debug/valec build \
  --sanity_check false \
  --builtins_dir_override ../Vale4/Backend/builtins \
  vmdsitegen=src vmdsitegencmd=cmd \
  vmdparse=../VmdParse/src parseiter=../ParseIter/src \
  stdlib=../Vale4/stdlib/src \
  --output_dir build --region_override resilient-v3 \
  -o vmdsitegen

# 3. Render all 71 VerdagonSite pages.
cd ../VerdagonSite
rm -rf public
mkdir -p public/{components,images,blog,blog/next,grimoire,releases}
bash build.sh build all \
  ../VmdSiteGen/build/vmdsitegen \
  ../Snippet \
  ../VmdSiteGen/tools/highlighter/target/release/vmd-highlighter
```

### Green criteria

- Step 2 exit 0, no errors.
- Step 3 exit 0, **zero hits** for `fail|error|panic` in the build log, and **71 output files** under `VerdagonSite/public/{blog,grimoire,releases,home,...}` (count with `find VerdagonSite/public -maxdepth 3 -type f -not -name '*.html' -not -path '*/images/*' -not -path '*/components/*' | wc -l`).

If any page fails, the build log names the failing `.vmd` source and the valec/backend error.

## When to use which check

| Change shape | Minimum check |
|---|---|
| FrontendRust comment-only or doc-only | `cargo check --lib` |
| FrontendRust source change, no schema/CLI impact | `cargo nextest run` |
| Schema or CLI surface change (VAST keys, valec flags, hammer JSON) | nextest + VerdagonSite probe |
| Cherry-pick from `experimental` (any size) | nextest + VerdagonSite probe |
| Cross-cutting refactor | full `fire-commit` matrix (`Backend/test.sh` + `TesterRust`) |

`fire-commit` defines the full matrix; this guide is the local-validation companion. For substantive changes, run the broader probe even when CI would be green — CI's pass-rate is not the integration-correctness signal.

## See also

- `docs/skills/fire-commit.md` — the commit-and-sync protocol; defines the full test matrix and references this guide for local pre-flight.
- `docs/skills/merging-from-experimental.md` — cherry-pick workflow; calls out VerdagonSite as the load-bearing master e2e probe.
- `VerdagonSite/README.md` — the canonical invocation strings for the page-render step; consult when paths drift.
