# Lazy citizen compilation handoff

## What this is

Moving struct/interface/impl/function compilation in the typing pass off the eager loops in `fn evaluate` (`src/typing/compiler.rs`) onto a lazy, resolve-driven queue: nothing compiles unless resolved from a root, one deterministic queue holds all kinds, and a type's environment is established on demand ("illuminate"). A step toward LSP single-item compilation.

- **Design (source of truth):** `src/typing/docs/architecture/lazy-compilation-design.md` — ratified; Design Proposals empty.
- **Implementation plan (RFIGA, staged decouple → flip → forced-root):** `~/.claude/plans/please-plan-all-these-parsed-wolf.md`.

Both agree; if they diverge, the design doc wins and the plan is stale.

## Status

Design and plan are complete; **no code is written**. Implementation edits `src/typing/` (core), so it cannot start until the architect says the literal phrase **"fire core edits"**. Begin at RFIGA slice 1 (A1). Confirm the baseline is green first: `cargo test --manifest-path ./Cargo.toml --lib`.

## Constraints

- **Determinism is P0.** The queue must be insertion-ordered and seeded/traversed deterministically.
- **`main` is always exported** (convention — write `exported func main`); a non-exported `main` in a test is a bug to fix, not to accommodate.

## Lessons learned

- **Resolve is already compile-independent.** Do not plan against "resolve needs compile" — the coupling that forces a prior compile is method/overload resolution reading a type's outer env (illuminate), not resolve itself.
- **Architect preference:** roots are exported items only; a forced-root flag (for LSP and non-exported items) is the one general seam, not per-case special-casing.
- **Trap:** `fn get_parents` in `impl_compiler.rs` walks only the sub-citizen's file today; the design needs both the sub-citizen's and super-interface's files for impl enqueue.
- **Trap:** the interface sealed flag is read only by `fn evaluate_maybe_virtuality` in `function_compiler_middle_layer.rs` via `fn lookup_sealed`, which panics on a miss and never touches the interface's env — so lazy illuminate must be forced there before the read.
