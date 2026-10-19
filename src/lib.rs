#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
#![cfg_attr(feature = "rust_interop", feature(rustc_private))]

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
#[cfg(feature = "rust_interop")]
extern crate rustc_codegen_ssa;
#[cfg(feature = "rust_interop")]
extern crate rustc_codegen_llvm;
#[cfg(feature = "rust_interop")]
extern crate rustc_monomorphize;
#[cfg(feature = "rust_interop")]
extern crate rustc_index;
#[cfg(feature = "rust_interop")]
extern crate rustc_abi;
#[cfg(feature = "rust_interop")]
extern crate rustc_target;
extern crate core;
#[cfg(feature = "rust_interop")]
extern crate rustc_hashes;

pub mod backend_ffi;
pub mod builtins;
pub mod clang;
pub mod code_source;
pub mod compile_options;
#[cfg(all(test, not(feature = "rust_interop")))]
pub mod end_to_end_tests;
pub mod integration_tests;
pub mod instantiating;
pub mod interner;
pub mod keywords;
pub mod lexing;
pub mod parse_arena;
pub mod parsing;
pub mod pass_manager;
pub mod postparsing;
pub mod scout_arena;
pub mod tests;
pub mod typing;
#[cfg(test)]
pub mod testvm;
pub mod utils;
#[path = "solver/lib.rs"]
pub mod solver;

pub use interner::StrI;
pub use keywords::Keywords;