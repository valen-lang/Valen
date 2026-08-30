# Building the Compiler

This page describes how to build the Vale compiler.

Note that the below instructions don't build LLVM from source, but we highly recommend it. Building LLVM from source will enable its debug-only checks, which help immensely when modifying the compiler.


## Ubuntu

```sh
sudo apt install -y git
git clone --single-branch --branch master https://github.com/ValeLang/Vale
Vale/scripts/ubuntu/install-compiler-prereqs.sh -l LLVMForVale -b BootstrappingValeCompiler
cd Vale
./scripts/ubuntu/build-compiler.sh "$PWD/../LLVMForVale/clang+llvm-13.0.1-x86_64-linux-gnu-ubuntu-18.04" "$PWD/../BootstrappingValeCompiler" --test=all ./scripts/VERSION

```


## Mac

```sh
git clone --single-branch --branch master https://github.com/ValeLang/Vale
Vale/scripts/mac/install-compiler-prereqs.sh ~/BootstrappingValeCompiler
source ~/.zshrc
cd Vale
./scripts/mac/build-compiler.sh ~/BootstrappingValeCompiler --test=all ./scripts/VERSION
```


## Windows

One *must* build LLVM from source, because [the Windows LLVM release is broken](https://bugs.llvm.org/show_bug.cgi?id=28677).


### Dependencies

 1. Install Visual Studio
 1. Install python 3, remember to check the box to add it to the path
 1. Install git: https://git-scm.com/download/win
 1. Install 7-zip: https://www.7-zip.org/download.html
 1. Install the previous version of the vale compiler: https://vale.dev/download
 1. Build LLVM, see next section.
 1. Build the compiler, which is the section after next.


### Build LLVM

If you want to skip this, you can download and extract [this file](https://github.com/Verdagon/LLVMWinMinimal/releases/download/14.0.6.0/llvm-project-llvmorg-14.0.6.zip) to `C:\llvm`. **Disclaimer**: Download at your own risk; we made this .zip file by building it, stripping it down, and merging the include files to fix the problems with the [regular LLVM windows release](https://bugs.llvm.org/show_bug.cgi?id=28677).

Ensure your machine (or VM) has sufficient resources: 5 cores, 10gb ram, 200gb disk. You'll be building all of LLVM, which is quite resource intensive.

Download [LLVM sources](https://github.com/llvm/llvm-project/releases).

Unzip to e.g. `C:\llvm-project-llvmorg-11.0.1`.

Depending on where visual studio is:

 * If in Program Files (x86), use the program "x86_64 Cross Tools Command Prompt for VS 2019"
 * If in Program Files, use the program "Developer Command Prompt for VS 2019"

`cd C:\llvm-project-llvmorg-13.0.1`

`mkdir build`

`cd build`

For when building a distributable (such as for CI) LLVM release: the built LLVM (in the `build` directory here) **will reference things in the original source directory** (the `C:\llvm-project-llvmorg-13.0.1` here). This is why we make a `build` subdirectory under the source directory; we can then package it all up together and distribute it as one archive.

`cmake "C:\llvm-project-llvmorg-13.0.1\llvm" -D "CMAKE_INSTALL_PREFIX=C:\llvm-project-llvmorg-13.0.1\build" -D CMAKE_BUILD_TYPE=Release -G "Visual Studio 17 2022" -Thost=x64 -A x64`

`cmake --build .`

`cmake --build . --target install`


### Build the Compiler

Once youve done the above steps and installed LLVM, run the below commands:

```sh
git clone https://github.com/ValeLang/Vale --single-branch --branch master
cd Vale
.\scripts\windows\build-compiler.bat C:\llvm\llvm-project-llvmorg-13.0.1 C:\OldValeCompiler --test=all ./scripts/VERSION
```

If you get an error "fatal error LNK1112: module machine type 'x86' conflicts with target machine type 'x64'" and you're running in the shell "x64_x86 Cross Tools Command Prompt for VS 2022", try instead running in the shell "x64 Native Tools Command Prompt for VS 2022".


## For development

If working on the Vale compiler, it's best to:

 * Build LLVM from scratch, in debug mode.
 * Use CLion.
    * [Build with a profile](https://www.jetbrains.com/help/clion/cmake-profile.html#CMakeProfileSwitcher), with an environment variable `LLVM_DIR=(llvm build dir)`. [Verify in the CMake log](https://stackoverflow.com/a/34772936/1424454) looking for "Using LLVMConfig.make in (llvm build dir)".


## Rust Interop

The `rust_interop` cargo feature makes a Vale program typecheck against real Rust items read from a
live rustc `TyCtxt` — `import rust.alloc.vec.Vec;` and the like. It is **off by default**; a normal
build (the sections above) is the standalone compiler and never touches rustc.

A program that imports `Vec` typechecks against live rustc; with the collector-driven backend a Vale
program that calls real Rust functions also links and runs (see `docs/handoffs/rust-interop-handoff.md`).
The C++ backend links the fork's shared libLLVM 21 in **both** the standalone and interop builds — under
interop the backend and rustc resolve to the same one libLLVM, so there is no dual-LLVM duplicate-symbol UB.

### Toolchain: stock nightly for development

**Developing the compiler uses stock Rust nightly** — there is no `rust-toolchain.toml` pin. A default
build compiles as plain Rust (every fork-touching line is `#[cfg(feature = "rust_interop")]`-gated), so it
needs only nightly plus a standalone **LLVM 21** for the C++ backend:

```sh
rustup toolchain install nightly     # if it is not already your rustup default
brew install llvm@21                  # macOS; build.rs finds it automatically
```

`build.rs` locates `llvm-config` in this order: `$LLVM_CONFIG`, then the fork's sibling llvm-config (only
when the active toolchain is the fork), then a standalone LLVM 21 (Homebrew `llvm@21`, or `llvm-config-21`
/ `llvm-config` on `PATH`, version-checked to be 21). So a stock-nightly checkout builds with no fork
present.

### The Vale rustc fork (interop only)

`--features rust_interop` links rustc's internals (`#![feature(rustc_private)]`) and needs the fork's
`per_instance_mir` patch, so **that build alone** uses the Vale rustc fork, selected explicitly with
`+rustc-fork`. Build and link it once:

```sh
git clone https://github.com/Verdagon/rust ~/rust
cd ~/rust && git checkout per-instance-mir && ./x build   # builds stage1 (~hours)
rustup toolchain link rustc-fork ~/rust/build/host/stage1
ln -sf ~/rust/build/aarch64-apple-darwin/stage0/bin/cargo \
       ~/rust/build/host/stage1/bin/cargo                 # give the toolchain its own cargo
```

Link **stage1**, not stage2: a plain `./x build` populates stage1 with the `rustc-dev` component, whereas
`./x build --stage 2` regenerates the stage2 sysroot *without* it. The fork's sysroot ships the
`rustc_private` libraries and `rust-src`, so there is **no** `rustup component add rustc-dev` step. Under
`+rustc-fork`, `build.rs`'s sibling-llvm-config derivation finds the fork's shared libLLVM automatically,
which the interop build must share with rustc's own (two libLLVMs in one process is duplicate-symbol UB).

### Build and test

From the repo root:

```sh
cargo +rustc-fork build --features rust_interop
cargo +rustc-fork test --lib --features rust_interop
```

The interop tests host rustc in-process (`run_compiler`) and assert against the typed AST, so they
live in the lib's own test target — run them with `--lib`, not as integration tests.

### Notes

 * **Backend-driving tests are excluded under the feature.** The suites that drive full owned-mode
   codegen/execution through `pass_manager::build` (`end_to_end_tests`, the `backend_ffi` FFI tests) are
   `#[cfg(all(test, not(feature = "rust_interop")))]` — the interop path never exercises that owned-mode
   path, and it is untested against the interop rustc-private linkage. They run in the normal (non-interop)
   `cargo nextest` gates instead.
 * **`cargo clean` after moving the repo directory.** Fixture paths are baked in at compile time via
   `env!("CARGO_MANIFEST_DIR")`; a stale artifact from a previous location makes every fixture-loading
   test fail with file-not-found. Rebuild clean after any move or rename.
 * Interop runs only where the fork toolchain is built and linked (`+rustc-fork`), so it is not part of
   standard CI — run it by hand.
