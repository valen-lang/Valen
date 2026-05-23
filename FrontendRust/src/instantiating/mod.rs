// From Frontend/InstantiatingPass/src/dev/vale/instantiating/
pub mod ast;
pub mod instantiated_compilation;
pub mod instantiating_arena;
pub mod instantiating_interner;
pub mod reintern;
pub mod instantiated_humanizer;
pub mod region_collapser_consistent;
pub mod region_collapser_individual;
pub mod region_counter;
#[cfg(test)]
pub mod tests;
// instantiator.rs (4416-line translation engine, 70 methods) is declared but
// cfg-gated out: its method signatures reference many un-migrated types via
// typed params. The declaration makes the module file part of the crate
// structurally (SCPX FILE_MAP can route to it for parity-checking) without
// requiring its contents to compile. Body-migration phase ungates this.
#[cfg(any())]
pub mod instantiator;

pub use instantiated_compilation::{InstantiatedCompilation, InstantiatorCompilationOptions};
