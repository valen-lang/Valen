# CoordinatorRust

Rust implementation of the Vale compiler coordinator.

## Overview

This is a 1:1 translation of the Vale `Coordinator` program (written in Vale) to Rust.

The coordinator orchestrates the Vale compilation pipeline by:
1. Parsing command-line arguments and flags
2. Invoking the Frontend (Valestrom - Java/Scala) to produce VAST files
3. Invoking the Backend (Midas - C++) to compile VAST to object files  
4. Invoking clang to link everything into an executable

## Structure

The code mirrors the Vale source structure exactly:

- `main.rs` - Entry point, mirrors `Coordinator/src/main.vale`
- `build.rs` - Build orchestration, mirrors `Coordinator/src/build.vale`
- `valestrom.rs` - Frontend invocation, mirrors `Coordinator/src/valestrom.vale`
- `midas.rs` - Backend invocation, mirrors `Coordinator/src/midas.vale`
- `clang.rs` - Clang linking, mirrors `Coordinator/src/clang.vale`

## Building

```bash
cargo build --release
```

The binary will be created at `target/release/valec`.

## Usage

Same as the Vale version:

```bash
# Show version
./valec version

# Show help
./valec help

# Build a Vale project
./valec build myproject=path/to/source -o myprogram
```

## Implementation Notes

- Function names, logic, and variable names mirror the Vale source as closely as possible
- Comments indicate corresponding line numbers in the original Vale source
- All Vale `List<T>` types are translated to Rust `Vec<T>`
- All Vale `Opt<T>` types are translated to Rust `Option<T>`
- Process spawning uses Rust's `std::process::Command` instead of Vale's `Subprocess`
- Flag parsing is done manually instead of using Vale's `flagger` library

