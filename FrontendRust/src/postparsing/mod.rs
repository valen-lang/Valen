// From Frontend/PostParsingPass/src/dev/vale/postparsing/
pub mod ast;
pub mod expressions;
pub mod itemplatatype;
pub mod names;
pub mod post_parser;
pub mod patterns;
pub mod rules;

pub use post_parser::ScoutCompilation;

#[cfg(test)]
pub mod test;
