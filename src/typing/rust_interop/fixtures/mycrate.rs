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
    /// By-value `self`, on purpose: this is a *consuming* method, exercising the owned-receiver
    /// path (the rvalue-call cases lean on it). Borrow receivers (`&self`) are supported now — see
    /// `peek` below — so this is a deliberate consume, not a workaround.
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

    /// A **borrow-receiver** method: `&self` lowers to `KindT::BorrowRef(Counter)`, unlike the
    /// by-value `self` the other methods use. It is the probe for whether the onion arc's
    /// reference-wrap arms (`substitute_templatas_in_kind`, the borrow arms of
    /// `is_type_convertible`/`convert`) now let a borrow receiver resolve — the shape every real
    /// `&self`/`&mut self` Rust method (`Vec::len`, `Vec::push`) takes. Called on a *local*, so it
    /// also exercises reading a local as a borrow against a borrow receiver.
    pub fn peek(&self) -> i32 {
        self.value
    }
}

/// A nested module, so path resolution is exercised below the crate root.
///
/// Every other item in this fixture sits **at** the root, which is the degenerate path — one
/// segment, zero modules to descend — and a root-only walk passes every one of them. `std::vec::Vec`
/// is the real shape: it lives under `std::vec`, not under the `std` root, so a root-only walk
/// cannot see it at all. Without something nested here, the walk could stay one level deep
/// indefinitely and the whole corpus would still be green.
///
/// The names inside are deliberately absent from the crate root, so a walk that ignored the module
/// segment and matched on the final name alone would find nothing rather than accidentally
/// succeeding.
pub mod instruments {
    pub fn depth_reading() -> i32 {
        31
    }

    /// A nested *type*, so the type path is covered as well as the function path — they are
    /// different `DefKind`s and a walk could plausibly handle one and not the other.
    pub struct Sonar {
        pub depth: i32,
    }

    impl Sonar {
        /// A method on a nested type. Method discovery runs off `inherent_impls` of the owner's
        /// `DefId`, which knows nothing about how the owner was reached — so this should work for
        /// free, and the case exists to confirm that rather than to drive new code.
        pub fn depth_of(self) -> i32 {
            self.depth
        }
    }

    pub fn make_sonar() -> Sonar {
        Sonar { depth: 33 }
    }
}

/// Re-exports, which is how `std::vec::Vec` actually reaches a user.
///
/// `std::vec` is `pub use alloc_crate::vec`, so the path a user writes (`std::vec::Vec`) is **not**
/// the path the definition has (`alloc::vec::Vec`). Whether a segment walk follows that for free
/// turns on what `module_children` reports for a re-export — its `Res` names the *definition*, so
/// it plausibly already works. These exist to decide that by measurement rather than by reading.
///
/// Two shapes, because they are different: re-exporting an **item** into a module, and re-exporting
/// a **module** so the walk has to descend through the alias.
pub mod readouts {
    pub use crate::instruments::depth_reading;
    pub use crate::instruments::make_sonar;
    pub use crate::instruments::Sonar;
}

pub mod gear {
    pub use crate::instruments;
}

/// A second type with methods, so two types' item sets can be exercised at once.
///
/// `get` collides with `Counter::get` **by name, deliberately**. Under methods-are-ordinary-
/// functions there is no per-type method table to keep apart — both land in one store and overload
/// resolution separates them by receiver type. So the failure this shape catches is not "the tables
/// leaked" but the importer attaching a method to the **wrong receiver**: cross `Gauge::get`'s
/// receiver with `Counter` and `(make_gauge()).get()` stops resolving, loudly.
///
/// No `new`, on purpose. `Counter::new` takes no parameters, so a same-named zero-argument
/// associated function on a second type would be genuinely ambiguous — nothing to resolve on — and
/// that is a *different* case about candidate narrowing rather than about two types coexisting.
pub struct Gauge {
    pub reading: i32,
}

impl Gauge {
    pub fn get(self) -> i32 {
        self.reading
    }
}

pub fn make_gauge() -> Gauge {
    Gauge { reading: 20 }
}

/// The same type reached through the free-function path rather than the method path, so a case
/// about *importing two types* need not also depend on method discovery working.
pub fn gauge_reading(g: Gauge) -> i32 {
    g.reading + 2
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

impl<T> Holder<T> {
    /// A method whose signature names the impl's inherited `T` — `Holder<int>.into_value()` returns the
    /// element type. Its parent impl is `impl<T> Holder<T>`, so the parent-inclusive generic list is
    /// `[T]` and the receiver/return resolve against it.
    pub fn into_value(self) -> T {
        self.value
    }
}

pub fn make_holder() -> Holder<i32> {
    Holder { value: 9 }
}

/// The same generic type at a *different* argument. Two instantiations that Vale must be able to
/// tell apart; today it cannot (see `a_generic_rust_type_loses_its_arguments`).
pub fn make_bool_holder() -> Holder<bool> {
    Holder { value: true }
}

/// Consumers for the two `Holder` instantiations, letting a case observe the two distinct *kinds* by
/// consuming each. See `a_generic_rust_type_carries_its_arguments`. (A scope-end drop on a generic
/// local also resolves — see `a_generic_rust_type_gets_a_scope_end_drop` — so consuming is one option,
/// not a workaround.)
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

/// A concrete zero-field type standing in for `std::alloc::Global`: the argument an impl *pins* in the
/// `Vec<T, Global>` shape. Imported as an ordinary opaque type.
pub struct Fixed;

/// A **two-parameter** generic type — the `Vec<T, A>` shape in miniature, with `A` the pinned
/// allocator. Imported opaque, so the field types never reach Vale.
pub struct Boxed<T, A> {
    pub value: Option<T>,
    pub alloc: Option<A>,
}

impl<T> Boxed<T, Fixed> {
    /// An associated function whose impl **fixes** the second parameter to `Fixed` — the `Vec::new`
    /// shape. Its own generics are just `[T]` (`Fixed` is not a parameter here); it returns
    /// `Boxed<T, Fixed>` with `Fixed` concrete, and takes no receiver and no arguments, so `T` is
    /// knowable only from the call-site type application, never inferred from an argument.
    pub fn new() -> Boxed<T, Fixed> {
        Boxed { value: None, alloc: None }
    }
}

/// Consumes a `Boxed` by value — exercises a two-parameter generic in argument position. (A scope-end
/// drop on a generic local also resolves now; see `a_generic_assoc_result_bound_to_a_local_...`.)
pub fn boxed_ignore<T>(_b: Boxed<T, Fixed>) -> i32 {
    7
}

/// Produces a `usize` — imported as the Vale `usize` primitive. The `Vec::len` shape: a Rust function
/// whose return is `usize`, which used to decline (`UnsignedInteger`).
pub fn some_size() -> usize {
    3
}

/// Consumes a `usize`, so a case can pass a produced `usize` somewhere. `usize` is a primitive, so no
/// drop is involved either way — this just exercises `usize` in argument position too.
pub fn consume_usize(_n: usize) -> i32 {
    8
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

/// An unsigned integer — deliberately un-importable.
///
/// Vale's `IntT` carries a width and no signedness, so a `u32` would become an `i32` and every
/// value above `i32::MAX` would read back negative. That is the silent-wrong-answer shape, so the
/// oracle declines rather than guessing.
pub fn unsigned_count() -> u32 {
    7
}

/// A float — deliberately un-importable.
///
/// Vale's `FloatT` is a unit struct with no width field, so `f32` and `f64` would intern
/// identically and a caller could not tell which one it was handed.
pub fn half_of(x: f32) -> f32 {
    x / 2.0
}

/// Reached only by walking `takes_hidden`'s signature, and never itself in an allowlist.
///
/// @RTMEIZ: every Rust type Vale uses is explicitly imported, including one it meets only through
/// another item's signature. Importing `takes_hidden` while `Hidden` stays out must decline the
/// function — conjuring the type instead would import something nobody asked for, and silently
/// widen the allowlist's meaning from "what Vale may use" to "what Vale may reach".
pub struct Hidden {
    pub magnitude: i32,
}

pub fn takes_hidden(h: Hidden) -> i32 {
    h.magnitude
}
