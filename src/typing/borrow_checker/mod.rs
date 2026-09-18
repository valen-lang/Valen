// Two mutually-exclusive borrow-checker impls, selected by the `borrow_checker_experimental` feature.
// Default (feature off) compiles the new `luminance` checker; enabling the feature compiles the old
// `experimental` one instead. Both define `Compiler::check_function`, so only one is ever compiled.
#[cfg(not(feature = "borrow_checker_experimental"))]
pub mod sorcerous;
#[cfg(not(feature = "borrow_checker_experimental"))]
pub use sorcerous::errors::humanize_borrow_error;

#[cfg(feature = "borrow_checker_experimental")]
pub mod experimental;
#[cfg(feature = "borrow_checker_experimental")]
pub use experimental::errors::humanize_borrow_error;

pub mod borrow_error;
pub mod templata_g;
pub mod kind_g;
pub mod group_expr;
pub mod check_usages_types;
pub mod ast_g;
pub mod access_event;
