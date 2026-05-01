// From Frontend/HigherTypingPass/src/dev/vale/highertyping/
pub mod ast;
pub mod astronomer_error_reporter;
pub mod higher_typing_pass;

#[cfg(test)]
mod tests;

pub use higher_typing_pass::HigherTypingCompilation;
