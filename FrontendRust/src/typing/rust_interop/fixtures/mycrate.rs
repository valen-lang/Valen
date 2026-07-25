// A Rust dependency crate for the Rust-interop milestone.
//
// Compiled to an rlib and handed to the driver host with `--extern mycrate=<rlib>`, so the
// items Vale imports live in a *dependency* rather than in the crate under compilation.
// That distinction is load-bearing: rustc resolves items in an upstream crate through
// different queries than local ones (`module_children` vs `module_children_local`), and the
// dependency form is the only one that occurs in real use. The toylang/Sky prototype started
// with local definitions and had to migrate, hitting three separate blockers that were all
// invisible while everything was local.
//
// `i32` because Vale's `int` is 32-bit (`KindT::Int(IntT { bits: 32 })`).

pub fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// A Rust type for Vale to hold and call methods on.
///
/// No `Drop` impl on purpose: the importer must synthesize a `drop` for every imported type
/// regardless, because `Compiler::drop`'s `KindT::Struct` arm always resolves a destructor
/// call. Asking rustc for a method named `drop` would answer `None` here.
pub struct Counter {
    pub value: i32,
}

impl Counter {
    /// By-value `self` rather than `&self`, deliberately and temporarily.
    ///
    /// A `&self` receiver lowers to `KindT::BorrowRef`, and
    /// `substitute_templatas_in_kind` currently has `unimplemented!()` for all four
    /// reference-wrap arms (`templata_compiler.rs:522-525`) — a gap in the onion arc, not
    /// something interop introduced. Substitution runs even for non-generic callees, so a
    /// borrow receiver hits it immediately. Switch this back to `&self` once those arms land.
    pub fn get(self) -> i32 {
        self.value
    }
}

/// Lets a `Counter` reach Vale by *inference from a signature* rather than by name, so the
/// milestone needs no import-visibility work and no name-collision precedence rule.
pub fn make_counter() -> Counter {
    Counter { value: 7 }
}
