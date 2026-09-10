# Accuracy report: Clang Arch Derived From Rust Triple (CADFRTZ)

Audited against the working tree on 2026-09-06. Source: git:82f9ec4b^:docs/arcana/ClangArchDerivedFromRustTriple-CADFRTZ.md (deleted in commit 82f9ec4b)

## Verdict
The definition describes the experimental rust-interop prototype's Coordinator: its clang-invocation code forcing `-arch arm64` on Darwin (to defend against a Rosetta-translated x86_64 valec process), and deriving the Rust target triple from `parent(rust_include_dir).name()` instead of threading it as a separate parameter. That prototype was deliberately deleted in 82f9ec4b along with this doc. `CoordinatorRust/` at the repo root now contains only a `target/` build-artifact directory — no source files, no Coordinator, no ValeRuster, no cbindgen/rust_deps.h pipeline exist anywhere in the tree. No file in src/, Backend/, or docs/ cites CADFRTZ, and none embody the concern (there is no clang-invocation code in the current codebase at all — codegen goes through Backend/src, not a separate clang-invoking Coordinator). This matches the prior census agent's finding (docs/convos/convo-162-*.md:292: "deleted deliberately with the rust-interop prototype (82f9ec4b) — no, code gone"). Recommendation: delete — the concern and its code are both gone, no citing comments remain to clean up, and there is nothing to migrate.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | The Coordinator invokes clang to link the final binary and must pass `-arch arm64` on Darwin to avoid Rosetta-triggered x86_64 output | OBSOLETE (code deleted) | No `Coordinator` source exists; `CoordinatorRust/` contains only `target/` (build cache) | Coordinator and its clang invocation were removed with the rust-interop prototype in 82f9ec4b |
| 2 | For Rust interop builds, `-I<rust_include_dir>` is needed so clang finds cbindgen-generated `rust_deps.h` | OBSOLETE (code deleted) | `grep -rn cbindgen src Backend docs` finds nothing but convo logs | Rust-interop/cbindgen pipeline no longer exists in this repo |
| 3 | The Rust target triple is recoverable as `parent(rust_include_dir).name()` because Divination's cbuild writes to `<rust_output_dir>/target/<triple>/release/` | OBSOLETE (code deleted) | No `Divination`, `cbuild`, or triple-derivation code found in src/Backend | Same — deleted wholesale with the prototype |
| 4 | `-arch arm64` is hardcoded for now; if x86_64 Mac builds return, derive `-arch` from the triple's first segment | N/A — design musing about future extensibility, code gone | — | — |
| 5 | Linux/Windows don't use `-arch` this way; gating is `IsDarwin()`-only | OBSOLETE (code deleted) | No `IsDarwin` symbol found in src/Backend | — |

## Stale citation sites
None — `grep -rn -w "CADFRTZ"` across src/, Backend/, docs/ (excluding convo logs) returns zero hits.

## Uncited sites that embody the arcana
None found — no clang-invocation, Coordinator, or Rust-interop code exists in the current tree to cite it from.

## Suggested text
Not applicable — verdict is delete, not migrate. No corrected arcana paragraph is warranted since the underlying mechanism (a Vale-invoked Coordinator linking against a separately-built Rust static library) no longer exists in this codebase.
