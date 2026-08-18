#![feature(box_patterns)]
#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
// Rust interop links rustc's internals. `rustc_private` is the entire feature list this
// needs — no other `#![feature]` and no extra `#![allow]`.
#![cfg_attr(feature = "rust_interop", feature(rustc_private))]

// The rustc crates the interop read path uses. `rustc_driver` must be named even where its
// API is barely called: declaring it is what pulls in the dylib the other internals live in.
// Deliberately minimal — add on demand rather than up front; the long lists belong to
// codegen work, not to a read-only typing pass.
#[cfg(feature = "rust_interop")]
extern crate rustc_driver;
#[cfg(feature = "rust_interop")]
extern crate rustc_hir;
#[cfg(feature = "rust_interop")]
extern crate rustc_interface;
#[cfg(feature = "rust_interop")]
extern crate rustc_middle;
#[cfg(feature = "rust_interop")]
extern crate rustc_session;
#[cfg(feature = "rust_interop")]
extern crate rustc_span;

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
pub mod instantiating;
pub mod interner;
pub mod keywords;
pub mod lexing;
pub mod parse_arena;
pub mod parsing;
pub mod pass_manager;
pub mod postparsing;
pub mod scout_arena;
// pub mod simplifying;
pub mod tests;
pub mod typing;
// #[cfg(test)]
// pub mod testvm;
pub mod utils;
// pub mod von;
#[path = "solver/lib.rs"]
pub mod solver;

pub use interner::StrI;
pub use keywords::Keywords;
