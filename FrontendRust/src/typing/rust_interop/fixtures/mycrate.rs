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

    /// A second method, so method discovery is exercised as a *list* rather than as a lucky
    /// single. One method passing says nothing about whether the walk collects them all.
    pub fn doubled(self) -> i32 {
        self.value * 2
    }

    /// A method carrying **its own** type parameter, on top of whatever the container declares.
    ///
    /// The receiver is concrete here, so `T` is the method's alone. That matters for the index
    /// mapping: a `ty::Param` indexes the *parent-inclusive* generic list, and this is the shape
    /// where an item's own parameters sit above its parent's.
    pub fn or_else<T>(self, fallback: T) -> T {
        fallback
    }

    /// An associated function with **no receiver** — the `Vec::new` shape.
    ///
    /// It arrives through the same `associated_items` walk as `get`, so under the
    /// methods-are-not-special design it becomes an ordinary top-level declaration that simply
    /// happens to take no parameters. If associated functions needed their own path, this is
    /// where that would show up.
    pub fn new() -> Counter {
        Counter { value: 5 }
    }
}

/// A **free function** taking a Rust type as a parameter.
///
/// Distinct from a method: `get` reaches Vale through `associated_items`, this through
/// `importable_functions`. Both must produce the same kind of declaration, and a Rust type in
/// argument position is a different lowering path from one in return position.
pub fn value_of_counter(c: Counter) -> i32 {
    c.value
}

/// The same Rust citizen identity on **both sides** of one signature.
pub fn bump(c: Counter) -> Counter {
    Counter { value: c.value + 1 }
}

/// Takes nothing. The degenerate parameter list, which must go through the ordinary path rather
/// than a special case (@NNGZ in miniature).
pub fn seven() -> i32 {
    7
}

/// Returns `()`, which lowers to `VoidT` — and is therefore callable only in statement position.
pub fn do_nothing() {}

/// A bool round-tripping in both directions, so a non-integer primitive is covered.
pub fn is_positive(x: i32) -> bool {
    x > 0
}

pub fn to_int(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}

/// The mirror of `pick`: two parameters, returning the **second**.
///
/// `pick` alone cannot catch an index mapping that is consistently off — both canaries together
/// can, because no single wrong mapping satisfies both.
pub fn pick_second<A, B>(_a: A, b: B) -> B {
    b
}

/// One type parameter, in and out. A *floor* rather than a canary: it passes under any mapping,
/// so it proves substitution happens at all and nothing more.
pub fn id<T>(x: T) -> T {
    x
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

/// Consumers for the two `Holder` instantiations, so a case can observe the two *kinds* without
/// also needing a scope-end drop on a generic type — which does not resolve yet. See
/// `a_generic_rust_type_carries_its_arguments`.
pub fn holder_value(h: Holder<i32>) -> i32 {
    h.value
}

/// A **generic function whose parameter is the generic type applied to its own parameter** —
/// `Holder<T>` rather than a bare `T`.
///
/// This is the shape `pick<A, B>` does not cover: its parameters are bare generics, so the
/// declaration references a rune directly with no rule. Here the parameter needs `LookupSR` +
/// `CallSR`, and `T` is only knowable by running that call backwards from the argument — which is
/// exactly what a generic type's `drop` needs too. Whether this resolves says whether the drop gap
/// is drop-specific or general.
pub fn holder_ignore<T>(_h: Holder<T>) -> i32 {
    9
}

pub fn bool_holder_flag(h: Holder<bool>) -> i32 {
    if h.value {
        1
    } else {
        0
    }
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

/// The same projection in **argument** position, which is a different code path from `first`'s
/// return position — parameters are lowered in a loop and the whole declaration is dropped if any
/// one of them declines, whereas the return type is lowered once afterwards.
///
/// `I` appears as an ordinary parameter too, so the signature is well-formed Rust; without it `I`
/// would be unconstrained and rustc would reject the fixture rather than Vale declining it.
pub fn take_first<I: Iterator>(_i: I, _x: I::Item) {}
