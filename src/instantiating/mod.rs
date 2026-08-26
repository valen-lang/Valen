pub mod ast;
pub mod collector;
pub mod instantiated_compilation;
pub mod instantiating_arena;
pub mod instantiating_interner;
pub mod instantiated_humanizer;
#[cfg(test)]
pub mod tests;
pub mod instantiator;
// The rustc-collector-driven instantiation path: the per_instance_mir provider and its state. Only
// under rust_interop, where the crate root links the rustc internals it names (TyCtxt/Instance/Body).
#[cfg(feature = "rust_interop")]
pub mod rust_interop;

pub use instantiated_compilation::{InstantiatedCompilation, InstantiatorCompilationOptions};
