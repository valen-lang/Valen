pub mod full_compilation;
pub mod pass_manager;

#[cfg(test)]
mod end_to_end_test;

pub use full_compilation::{FullCompilation, FullCompilationOptions};
