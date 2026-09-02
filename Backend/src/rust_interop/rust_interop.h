#ifndef VALE_RUST_INTEROP_H_
#define VALE_RUST_INTEROP_H_

#include <string>

#include "../globalstate.h"
#include "../metal/ast.h"

// Emit a Rust->Vale callback wrapper: an LLVM function defined under `symbol` (the rustc-mangled name
// Rust's monomorphized call site targets) that receives the Rust-ABI arguments, forwards them to the
// internal Vale body named `valeName`, and marshals the return back. Single-symbol (arch §5.2): this
// wrapper is the sole definition of `symbol` (rustc stripped its own unreachable!() stub for the
// method via the partition filter), so Rust's static call resolves straight to Vale's body.
void emitInboundCallbackWrapper(
    GlobalState* globalState,
    Program* program,
    const std::string& symbol,
    const std::string& valeName);

#endif
