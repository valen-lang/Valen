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

/// Two generic parameters, returning the *first*, so a call at two different types can tell
/// which parameter Vale bound where.
///
/// This is the canary for the index mapping, not merely for substitution. A `ty::Param` carries
/// an index into its item's *parent-inclusive* generic list, so a lowering that mishandles the
/// offset produces a well-formed reference to the wrong slot — invisible at a glance. Called at
/// `<int, bool>` and returning `A`, a swap yields `bool` where `int` belongs, which fails to
/// typecheck loudly. `id<T>(x: T) -> T` would pass under either mapping and prove nothing.
pub fn pick<A, B>(a: A, _b: B) -> A {
    a
}

/// A **generic** Rust type — the `Vec<T>` shape in miniature.
///
/// Distinct from `pick<A, B>`, which is a generic *function*: its parameters live on the signature
/// and Vale's solver substitutes them. A generic *type* needs the citizen itself to carry template
/// args, which is a different mechanism entirely.
pub struct Holder<T> {
    pub value: T,
}

pub fn make_holder() -> Holder<i32> {
    Holder { value: 9 }
}

/// The same generic type at a *different* argument. Two instantiations that Vale must be able to
/// tell apart; today it cannot (see `a_generic_rust_type_loses_its_arguments`).
pub fn make_bool_holder() -> Holder<bool> {
    Holder { value: true }
}

/// An associated-type projection in return position — deliberately un-importable.
///
/// `I::Item` is `<I as Iterator>::Item`, and normalizing it *requires* the `I: Iterator`
/// predicate to find the impl. Since no predicates are read at all, this is not merely an
/// unbounded parameter but an un-normalizable alias: importing it would put something in the
/// declaration that nothing can ever resolve. The oracle must decline it rather than produce a
/// declaration with a hole.
/// The projection is in *bare* return position on purpose. Wrapping it — `Option<I::Item>` —
/// would make the return type an ordinary ADT and the un-imported-`Option` check would fire
/// first, so the test would pass for the wrong reason and prove nothing about projections.
pub fn first<I: Iterator>(mut i: I) -> I::Item {
    i.next().unwrap()
}
