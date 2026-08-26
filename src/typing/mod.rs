// Core entry point
pub mod compilation;
pub use compilation::{TypingPassCompilation, TypingPassOptions};

// Type system and core data structures (high priority - needed for all others)
pub mod ast;
pub mod names;
pub mod templata;
pub mod types;

// Environments and context
pub mod env;

// Basic helpers and outputs
pub mod compiler_outputs;
pub mod hinputs_t;
pub mod ptr_key;

// Top-level compiler orchestration
pub mod compiler;
pub mod typing_interner;

// Error reporting
pub mod compiler_error_humanizer;
pub mod compiler_error_reporter;

// Specific compilers
pub mod array_compiler;
pub mod convert_helper;
pub mod edge_compiler;
pub mod infer_compiler;
pub mod overload_resolver;
pub mod borrow_checker;
pub mod reachability;
pub mod sequence_compiler;
pub mod templata_compiler;
pub mod type_st_match;

// Sub-compilers grouped by concern
pub mod citizen;
pub mod expression;
pub mod function;
pub mod infer;
pub mod macros;
pub mod rule_runes;
pub mod rune_typing;

// The query services the typing pass can consult. Always compiled: it is what keeps the
// interop `#[cfg]` out of `Compiler` and its constructors.
pub mod oracles;

// Rust interop. Only present in the rustc-linked binary; under the standalone
// binary this module doesn't exist and every seam hook compiles out with it.
#[cfg(feature = "rust_interop")]
pub mod rust_interop;

// Tests
#[cfg(test)]
pub mod test;
