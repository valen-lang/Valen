# Clang Arch Derived From Rust Triple (CADFRTZ)

When the Coordinator invokes clang to link the final binary, it must pass `-arch arm64` on Darwin so a Rosetta-translated parent process (x86_64 valec on an arm64 Mac) doesn't silently cause clang to default to x86_64 output, which then fails to link against the arm64 `build.o` and arm64 Rust static library.

For Rust interop builds, we additionally need `-I<rust_include_dir>` so clang can find the cbindgen-generated `rust_deps.h`. The Rust target triple (e.g. `aarch64-apple-darwin`) is recoverable from `rust_include_dir` because Divination's `cbuild` writes its output to `<rust_output_dir>/target/<triple>/release/`. So `parent(rust_include_dir).name()` IS the triple — no need to plumb it through as a separate parameter.

For now we hardcode `-arch arm64` on Darwin since that's our only Apple target. If we need x86_64 mac builds again, derive `-arch` from the Rust triple's first segment (`aarch64` → `arm64`, `x86_64` → `x86_64`).

Linux/Windows don't use `-arch` this way, so the gating is `IsDarwin()`-only.
