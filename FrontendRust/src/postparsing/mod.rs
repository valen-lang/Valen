// From Frontend/PostParsingPass/src/dev/vale/postparsing/
pub mod ast;
pub mod expression_scout;
pub mod expressions;
pub mod function_scout;
pub mod itemplatatype;
pub mod names;
pub mod post_parser;
pub mod patterns;
pub mod rules;
pub mod variable_uses;

pub use post_parser::ScoutCompilation;

#[cfg(test)]
pub mod test;
