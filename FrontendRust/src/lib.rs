#![feature(box_patterns)]

pub mod interner;
pub mod keywords;
pub mod utils;
pub mod lexing;
pub mod parsing;
pub mod von;
pub mod builtins;
pub mod file_coordinate_map;
pub mod pass_manager;
pub mod passmanager;
pub mod simplifying;
pub mod instantiating;
pub mod typing;
pub mod highertyping;
pub mod postparsing;

#[cfg(test)]
mod tests;

pub use interner::{Interner, StrI};
pub use keywords::Keywords;

