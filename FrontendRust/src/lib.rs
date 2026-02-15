#![feature(box_patterns)]
#![feature(deref_patterns)]
#![allow(incomplete_features)]

pub mod builtins;
pub mod compile_options;
pub mod file_coordinate_map;
pub mod higher_typing;
pub mod instantiating;
pub mod interner;
pub mod keywords;
pub mod lexing;
pub mod parsing;
pub mod pass_manager;
pub mod postparsing;
pub mod simplifying;
pub mod typing;
pub mod utils;
pub mod von;

pub use interner::{Interner, StrI};
pub use keywords::Keywords;
