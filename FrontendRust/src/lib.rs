#![feature(box_patterns)]
#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]

// VCOORD: Onion typing arc: parser + postparsing linked; typing and downstream
// stay unlinked pending their own slices. higher_typing was retired outright.
// pub mod backend_ffi;
pub mod builtins;
// pub mod clang;
pub mod code_source;
pub mod compile_options;
// pub mod file_coordinate_map;
// pub mod final_ast;
// #[cfg(test)]
// pub mod end_to_end_tests;
// #[cfg(test)]
// pub mod integration_tests;
// pub mod instantiating;
pub mod interner;
pub mod keywords;
pub mod parse_arena;
pub mod scout_arena;
pub mod lexing;
pub mod parsing;
pub mod pass_manager;
pub mod postparsing;
// pub mod simplifying;
// VCOORD: typing gated so parsing/postparse compile alone. Un-gate when the typing
// cascade compiles again.
#[cfg(any())]
pub mod typing;
#[cfg(any())]
pub mod tests;
// #[cfg(test)]
// pub mod testvm;
pub mod utils;
// pub mod von;
#[cfg(any())]
#[path = "solver/lib.rs"]
pub mod solver;

pub use interner::StrI;
pub use keywords::Keywords;
