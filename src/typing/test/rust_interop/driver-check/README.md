# driver-check — a captured repro for the imported-param interface-header crash

This is a minimal `valen build` project that reproduces a typing-pass crash on the
reverse-direction interop path: a Vale struct implements an imported Rust trait whose
abstract method takes **two imported (Rust) types as borrow params**. It is the shape of
NobiliaV's real frame-loop callback, `MainLoopCallback::on_tick(&self, w &NobiliaWindow,
input &FrameInput)`.

## The crash

```
thread 'rustc' panicked at src/typing/compiler_outputs.rs:639:
called `Option::unwrap()` on a `None` value
```

Compiling the synthesized `MainLoopCallback` interface's abstract `on_tick` header, the
reachable-bounds gather in `check_defining_conclusions_and_resolve`
(`src/typing/infer_compiler.rs:690`) calls `get_inner_env_for_type` for an imported param
type (`NobiliaWindow` / `FrameInput`) whose inner env was never registered
(`declare_type_inner_env` had not run for it at that point), so the lookup unwraps `None`
at `src/typing/compiler_outputs.rs:639`. The call path is
`Compiler::evaluate` → `compile_interface` → `compile_interface_core`
(`src/typing/citizen/struct_compiler_core.rs:267`) →
`evaluate_generic_function_from_non_call_for_header`.

## Why it only reproduces through `valen build`

It surfaces **only with the builtins compiled in** — the `valen build` / `run_driven_rustc`
path. The builtins add the bounds that pull the imported param-type runes into the
reachable-bounds set, so the bare `--lib` harness (`run_case_rustc_driven*` / `drive_rustc`,
which compiles no builtins) does not hit it. This is why the repro is a real `valen build`
project rather than an in-tree harness case; see the "builtins repro" trap in
`docs/handoffs/rust-interop-handoff.md`.

## Reproduce

From the repo root:

```sh
# 1. Build the interop binaries (both need --features rust_interop).
cargo build --manifest-path Cargo.toml --features rust_interop --bin valenc-rs --bin valen

# 2. Build this project. target/debug must be on PATH so `valen` finds `valenc-rs`
#    (cargo invokes it as RUSTC_WORKSPACE_WRAPPER). RUST_BACKTRACE=1 shows the call path.
PATH="$PWD/target/debug:$PATH" RUST_BACKTRACE=1 ./target/debug/valen build \
  --manifest-path src/typing/test/rust_interop/driver-check/Valen.toml
```

→ exit 101, the panic above.

## The vendored `nobiliav` is a stub

`Valen.toml` depends on `nobiliav/` beside it — a **dependency-free stub** of NobiliaV's real
crate. Every body is `unimplemented!()`; it keeps only the public API `driver_check.valen`
imports (`NobiliaWindow` / `FrameInput` / `MainLoopCallback` and the arrow-key functions). So the
whole repro is self-contained in this repo — no external NobiliaV checkout, and none of the real
render / geometry / app / winit / wgpu graph. The crash is in the typing pass, before any of that
implementation would matter, so stubbing the bodies changes nothing about the reproduction.

The dependency path is written `../../nobiliav`, not `nobiliav`, because `valen build` renders it
verbatim into the *generated* `target/valen-build/Cargo.toml` — two levels down — and cargo
resolves it relative to that file.

## Purpose

The seed for the fast in-tree repro (a `drive_tests.rs`-style `run_wrapper` test with
builtins present) and the core typing fix. `on_tick` is void-returning with two
imported-borrow params; a sibling codegen bug (`toRef` assert) waits behind this one when
the same callback is invoked through a generic *method* caller.
