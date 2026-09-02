# `valen build` has no way to produce an optimized build (no release / no profile passthrough)

**Component:** `valen build` (generated-workspace pipeline)
**Severity:** medium — makes any graphics/compute-heavy Valen program unusably slow, and misdiagnoses as "Valen codegen is slow."
**Found:** 2026-08-31, driving the NobiliaV `nobiliav` renderer from a Valen program
(`/Volumes/V/NobiliaV/gamedev-wip/crates/nobiliav/valen/`).

## Symptom

The Valen `driver` (windowed pentagon renderer) felt badly sluggish next to its Rust twin —
a visible delay between an arrow-key press and the camera moving. Same `nobiliav` library, same
per-frame work; only the build pipeline differs (rustc `cargo run` vs `valen build`).

The instinct is to blame Valen codegen or the reverse-interop callback. It is neither: the
per-frame Valen (`on_tick`) is trivial, and all the cost is the shared **Rust** render path
(`render_textures` runs every frame under `ControlFlow::Poll` — glam math + wgpu draw-list build +
M4 water refraction), which `valen build` compiles **unoptimized**.

## Root cause

`valen build` emits a standalone generated workspace whose `Cargo.toml` (a) declares an empty
`[workspace]`, making it its own workspace root, and (b) carries **no `[profile.*]` section**.
So the whole dependency graph (winit/wgpu/glam/render/…) builds at the default `opt-level = 0`,
and there is no flag to change that.

The host project, by contrast, optimizes its dev builds — NobiliaV's workspace root has:

```toml
[profile.dev]
opt-level = 2
```

Because the generated workspace is its own root, it cannot inherit that override, and `valen build`
neither copies it nor offers `--release`. Net: the Rust driver's "debug" build is `-O2`, the Valen
driver's is `-O0`.

## Evidence

`valen build` output — note the profile line:

```
Finished `dev` profile [unoptimized + debuginfo] target(s)
```

After hand-adding `[profile.dev] opt-level = 2` to the generated
`.../valen-build/Cargo.toml` and rebuilding with the same toolchain/wrapper `valen build` uses
(`RUSTUP_TOOLCHAIN=rustc-fork RUSTC_WORKSPACE_WRAPPER=.../valenc-rs cargo build`):

```
Finished `dev` profile [optimized + debuginfo] target(s) in 32.52s
```

The rebuilt binary is snappy — latency gone, framerate up — confirming the cause is opt-level, not
Valen.

## Repro

1. A Valen program whose `[rust-dependencies]` pull a heavy Rust graph (here: `nobiliav → wgpu/glam`).
2. `valen build --manifest-path <Valen.toml>` → generated `Cargo.toml` has no `[profile.dev]`.
3. Run the emitted bin: dominated by unoptimized dep code.
4. There is no `valen build --release` and no `[profile]` passthrough to fix it.

## Suggested fixes (rough preference order)

1. **`valen build --release`** (and/or a `[profile.*]` passthrough from `Valen.toml` into the
   generated `Cargo.toml`) — explicit, matches cargo's mental model.
2. **Emit `[profile.dev] opt-level = 2` by default** in the generated `Cargo.toml` — the graphics
   graph is the common case and `-O0` is unusable for it.
3. **Drop the empty `[workspace]`** so the generated package is absorbed by the host workspace and
   inherits its profiles. (Least preferred — couples the generated package to sitting inside a
   workspace, and silently changes behavior with the host's layout.)

## Workaround (temporary)

Hand-edit `<project>/target/valen-build/Cargo.toml` to add `[profile.dev] opt-level = 2`, then
rebuild via cargo directly (not `valen build`, which regenerates and wipes the edit):

```
env RUSTUP_TOOLCHAIN=rustc-fork RUSTC_WORKSPACE_WRAPPER=<...>/valenc-rs \
  cargo build --manifest-path <project>/target/valen-build/Cargo.toml
```
