// From Frontend/TypingPass/src/dev/vale/typing/

// Core entry point
pub mod compilation;
pub use compilation::{TypingPassCompilation, TypingPassOptions};

// Type system and core data structures (high priority - needed for all others)
pub mod types;
pub mod names;
pub mod ast;
pub mod templata;

// Environments and context
pub mod env;

// Basic helpers and outputs
pub mod compiler_outputs;
pub mod hinputs_t;

// TODO: Files with nested block comment issues that need fixing:
// - citizen/ (struct_compiler.rs has nested comments)
// - compiler.rs
// - compiler_error_humanizer.rs
// - compiler_error_reporter.rs
// - templata_compiler.rs
// - infer_compiler.rs
// - overload_resolver.rs
// - array_compiler.rs
// - sequence_compiler.rs
// - edge_compiler.rs
// - reachability.rs
// - convert_helper.rs
// - expression/
// - function/
// - infer/
// - macros/

// Tests
#[cfg(test)]
pub mod tests;

