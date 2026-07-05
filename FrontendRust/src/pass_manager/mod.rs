pub mod code_source;
// pub mod full_compilation;   // Depends on higher_typing pipeline — unlinked during onion arc.
// pub mod pass_manager;       // Depends on higher_typing_error_humanizer — unlinked during onion arc.

pub use code_source::{CodeSource, Source};
// pub use full_compilation::{FullCompilation, FullCompilationOptions};
