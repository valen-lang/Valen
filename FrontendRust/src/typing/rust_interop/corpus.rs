// The interop corpus: every case as **data**, so both test tiers read one text.
//
// A case is `(Rust fixture, Vale program, expectation)` — arch §26b.1. Tier 1
// (`typing/test/rust_interop/`) checks that the program typechecks, walks the typed AST, and
// asserts the oracle was actually consulted. Tier 2, once the LLVM 16 → ~21 port and the onion
// relink unblock it, runs the identical program and checks **only** what `main` returns. Neither
// tier owns the text, which is what stops the two drifting into two copies of each program.
//
// **Why this lives here rather than in the test tree.** Tier 1 must live under
// `#[cfg(test)] typing::test`, because `NodeRefT` and the `collect_*` macros exist only there.
// Tier 2's likely home is `end_to_end_tests`, which is an ordinary `pub mod` and therefore cannot
// see anything gated on `cfg(test)`. A corpus in the test tree would be invisible to it, and the
// duplication this module exists to prevent would come straight back. So the data sits in the
// interop module proper, where anything in the crate can read it.
//
// **Data only.** No assertions, no AST walking, nothing that would want `collect_*` — those are
// the tier's business, not the case's. A case says what to compile and what must happen; how to
// observe it differs per tier and belongs with the tier.
//
// Adding a case: give it a distinct `Returns` value where that is free. A corpus where every
// program returns the same number gives tier 2 almost no signal, since a case that computed the
// wrong thing could still land on the shared value by accident.

/// What a case's Vale program must do.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Expect {
    /// It typechecks, and `main` returns this. Tier 1 checks that it compiled; tier 2 checks the
    /// value.
    Returns(i64),
    /// It does **not** typecheck, and fails with this `ICompileErrorT` arm. Tier-1-only by
    /// nature: there is no program to run.
    FailsToCompile(&'static str),
    /// rustc rejects the *fixture crate* before Vale's typing pass ever runs. Tier-1-only, and
    /// there is exactly one — it is the regression test for hosting rustc in-process at all.
    RustcFails,
}

/// One corpus case.
pub struct Case {
    /// The fixture crate directory under `typing/rust_interop/`.
    pub fixture: &'static str,
    /// Names this case's private rustc output directory, so concurrent cases do not race on one
    /// rlib path (@TMBFIZ).
    pub name: &'static str,
    /// The Vale program. `main` is the entry point and its return value is the case's observable.
    pub vale: &'static str,
    /// The Rust items declared importable. This is what an `import rust.X.Y` will eventually
    /// populate; supplying it explicitly is the same mechanism with a different source. Scoping is
    /// membership in this list, never a check at the call site.
    pub allowed: &'static [&'static str],
    pub expect: Expect,
}

// ---------------------------------------------------------------------------
// A. Signatures and lowering
// ---------------------------------------------------------------------------

pub const CALLS_A_RUST_FREE_FUNCTION: Case = Case {
    fixture: "fixtures",
    name: "free-function",
    vale: r#"
exported func main() int {
  return add_two_numbers(20, 22);
}
"#,
    allowed: &["add_two_numbers"],
    expect: Expect::Returns(42),
};

/// The negative control for the case above. If the same program compiled with nothing importable,
/// the positive case would prove nothing about where resolution came from.
pub const AN_EMPTY_ALLOWLIST_MAKES_NOTHING_IMPORTABLE: Case = Case {
    fixture: "fixtures",
    name: "empty-allowlist",
    vale: r#"
exported func main() int {
  return add_two_numbers(20, 22);
}
"#,
    allowed: &[],
    expect: Expect::FailsToCompile("CouldntFindFunctionToCallT"),
};

/// `pick<A, B>(a: A, b: B) -> A` called at `<int, bool>`, returning the **first** parameter.
///
/// The mapping is what this pins, not merely that substitution happens: a swapped index yields
/// `bool` where `int` belongs and `main() int` stops typechecking. `id<T>(T) -> T` would pass
/// under either mapping and prove nothing.
pub const READS_A_GENERIC_SIGNATURE_STRUCTURALLY: Case = Case {
    fixture: "fixtures",
    name: "generic-function",
    vale: r#"
exported func main() int {
  return pick<int, bool>(add_two_numbers(10, 5), true);
}
"#,
    allowed: &["add_two_numbers", "pick"],
    expect: Expect::Returns(15),
};

/// An empty parameter list is the degenerate case, not a special one.
pub const CALLS_A_ZERO_ARG_RUST_FUNCTION: Case = Case {
    fixture: "fixtures",
    name: "zero-arg",
    vale: r#"
exported func main() int {
  return seven();
}
"#,
    allowed: &["seven"],
    expect: Expect::Returns(7),
};

/// `()` lowers to `VoidT`, so the call is legal only in statement position.
pub const CALLS_A_RUST_FUNCTION_RETURNING_UNIT: Case = Case {
    fixture: "fixtures",
    name: "returns-unit",
    vale: r#"
exported func main() int {
  do_nothing();
  return 8;
}
"#,
    allowed: &["do_nothing"],
    expect: Expect::Returns(8),
};

/// A bool round-tripping in both directions — out of one Rust signature and into another.
pub const PASSES_AND_RETURNS_A_BOOL: Case = Case {
    fixture: "fixtures",
    name: "bool-round-trip",
    vale: r#"
exported func main() int {
  return to_int(is_positive(5));
}
"#,
    allowed: &["is_positive", "to_int"],
    expect: Expect::Returns(1),
};

/// A Rust citizen in **argument** position of a free function — a different lowering path from
/// return position, and a different discovery path from a method.
pub const TAKES_A_RUST_TYPE_AS_A_PARAMETER: Case = Case {
    fixture: "fixtures",
    name: "rust-type-parameter",
    vale: r#"
exported func main() int {
  return value_of_counter(make_counter());
}
"#,
    allowed: &["make_counter", "value_of_counter", "Counter"],
    expect: Expect::Returns(7),
};

/// The same citizen identity on both sides of one signature. If argument and return position
/// interned differently, this is where it shows.
pub const TAKES_AND_RETURNS_A_RUST_TYPE: Case = Case {
    fixture: "fixtures",
    name: "rust-type-both-sides",
    vale: r#"
exported func main() int {
  return value_of_counter(bump(make_counter()));
}
"#,
    allowed: &["make_counter", "bump", "value_of_counter", "Counter"],
    // `Counter { value: 7 }`, bumped once.
    expect: Expect::Returns(8),
};

/// The mirror canary for the generic index mapping: `pick_second<A, B> -> B` at `<bool, int>`.
///
/// `reads_a_generic_signature_structurally` pins the first parameter; this pins the second. A
/// mapping that is consistently off by one satisfies neither, which is why both exist.
pub const BINDS_THE_SECOND_GENERIC_PARAMETER: Case = Case {
    fixture: "fixtures",
    name: "generic-second-parameter",
    vale: r#"
exported func main() int {
  return pick_second<bool, int>(true, seven());
}
"#,
    allowed: &["pick_second", "seven"],
    expect: Expect::Returns(7),
};

/// `id<T>(T) -> T` — a floor rather than a canary. It passes under any index mapping, so all it
/// says is that substitution happens at all.
pub const INSTANTIATES_A_GENERIC_AT_ONE_PARAMETER: Case = Case {
    fixture: "fixtures",
    name: "generic-one-parameter",
    vale: r#"
exported func main() int {
  return id<int>(9);
}
"#,
    allowed: &["id"],
    expect: Expect::Returns(9),
};

/// A Rust citizen as a **generic argument**, rather than as a parameter or return type.
pub const INSTANTIATES_A_GENERIC_AT_A_RUST_TYPE: Case = Case {
    fixture: "fixtures",
    name: "generic-at-rust-type",
    vale: r#"
exported func main() int {
  return value_of_counter(id<Counter>(make_counter()));
}
"#,
    allowed: &["id", "make_counter", "value_of_counter", "Counter"],
    expect: Expect::Returns(7),
};

/// A signature Vale cannot represent is **declined**, not imported with a hole in it. The rest of
/// the import must survive, which is why the program calls the *other* item.
pub const DECLINES_AN_UNREPRESENTABLE_SIGNATURE: Case = Case {
    fixture: "fixtures",
    name: "declined-signature",
    vale: r#"
exported func main() int {
  return add_two_numbers(1, 4);
}
"#,
    allowed: &["add_two_numbers", "first"],
    expect: Expect::Returns(5),
};

/// The same decline, in **argument** position. A different code path from the return position:
/// parameters are lowered in a loop and one declining drops the whole declaration, whereas the
/// return type is lowered once afterwards.
pub const DECLINES_AN_UNREPRESENTABLE_PARAMETER: Case = Case {
    fixture: "fixtures",
    name: "declined-parameter",
    vale: r#"
exported func main() int {
  return add_two_numbers(2, 4);
}
"#,
    allowed: &["add_two_numbers", "take_first"],
    expect: Expect::Returns(6),
};

/// An unsigned integer declines for the same reason an alias does, and by the same exit.
///
/// This one is about **signedness**, which `IntT` does not carry — so importing it would hand back
/// a plausible `i32` rather than failing, which is the silent-wrong-answer shape this whole arc
/// keeps meeting (§0.2). Until 2026-07-27 it *panicked* instead, which is a different failure with
/// the same cause; the exits are unified now.
pub const DECLINES_AN_UNSIGNED_INTEGER: Case = Case {
    fixture: "fixtures",
    name: "declined-unsigned",
    vale: r#"
exported func main() int {
  return add_two_numbers(3, 4);
}
"#,
    allowed: &["add_two_numbers", "unsigned_count"],
    expect: Expect::Returns(7),
};

/// A float declines because `FloatT` is a unit struct with no width, so `f32` and `f64` would
/// intern identically.
///
/// The fixture takes *and* returns `f32` so the decline is reachable from either the parameter loop
/// or the return lowering — whichever runs first, the declaration is dropped and the case cannot
/// pass for the wrong reason.
pub const DECLINES_A_FLOAT: Case = Case {
    fixture: "fixtures",
    name: "declined-float",
    vale: r#"
exported func main() int {
  return add_two_numbers(4, 4);
}
"#,
    allowed: &["add_two_numbers", "half_of"],
    expect: Expect::Returns(8),
};

/// @RTMEIZ, from the side that is easy to miss: a type reached **only** through another item's
/// signature is not thereby imported.
///
/// `takes_hidden` is allowed and `Hidden` is not, so the parameter lowers to an ADT that is not in
/// the item table. Declining keeps the allowlist meaning *"what Vale may use"* rather than quietly
/// becoming *"what Vale may reach"* — the latter would make the scoping cases (27–30) assert
/// something weaker than they claim.
pub const DECLINES_A_SIGNATURE_NAMING_AN_UNIMPORTED_TYPE: Case = Case {
    fixture: "fixtures",
    name: "declined-unimported-type",
    vale: r#"
exported func main() int {
  return add_two_numbers(4, 5);
}
"#,
    allowed: &["add_two_numbers", "takes_hidden"],
    expect: Expect::Returns(9),
};

/// An item in a **nested module**, named by a dotted path.
///
/// This is the shape `Vec` needs: `std::vec::Vec` lives under `std::vec`, not under the `std` root,
/// so a walk that only reads a crate root's direct children cannot reach it — the item is not
/// merely unimported, it is invisible. Every other case in this corpus sits at a crate root, which
/// is the degenerate path (one segment, nothing to descend), so all of them pass under a root-only
/// walk and none of them would ever catch this.
///
/// The separator is `.`, matching what Vale source will write — `import rust.std.vec.Vec`.
pub const IMPORTS_AN_ITEM_FROM_A_NESTED_MODULE: Case = Case {
    fixture: "fixtures",
    name: "nested-module",
    vale: r#"
exported func main() int {
  return depth_reading();
}
"#,
    allowed: &["instruments.depth_reading"],
    expect: Expect::Returns(31),
};

/// A **type** in a nested module, reached by path, with its method.
///
/// A different `DefKind` from the function case and therefore a different arm — a walk could
/// plausibly descend correctly for one and not the other. The method is expected to come for free:
/// discovery runs off the owner's `inherent_impls`, which knows nothing about how the owner was
/// reached.
pub const IMPORTS_A_TYPE_FROM_A_NESTED_MODULE: Case = Case {
    fixture: "fixtures",
    name: "nested-module-type",
    vale: r#"
exported func main() int {
  return (make_sonar()).depth_of();
}
"#,
    allowed: &["instruments.Sonar", "instruments.make_sonar"],
    // `Sonar { depth: 33 }`.
    expect: Expect::Returns(33),
};

/// An item reached through a **re-exported name**, which is how `std::vec::Vec` actually works.
///
/// `std::vec` is `pub use alloc_crate::vec`, so a user's path and the definition's path differ.
/// `module_children` reports a re-export with its `Res` naming the definition, so a segment walk
/// that takes the `DefId` off the `Res` follows the re-export without knowing it did.
pub const IMPORTS_THROUGH_A_RE_EXPORTED_ITEM: Case = Case {
    fixture: "fixtures",
    name: "re-export-item",
    vale: r#"
exported func main() int {
  return depth_reading();
}
"#,
    allowed: &["readouts.depth_reading"],
    expect: Expect::Returns(31),
};

/// The other re-export shape: descending **through** a re-exported module rather than landing on a
/// re-exported item. A walk could plausibly handle the destination and not the intermediate hop.
pub const IMPORTS_THROUGH_A_RE_EXPORTED_MODULE: Case = Case {
    fixture: "fixtures",
    name: "re-export-module",
    vale: r#"
exported func main() int {
  return (make_sonar()).depth_of();
}
"#,
    allowed: &["gear.instruments.Sonar", "gear.instruments.make_sonar"],
    // `Sonar { depth: 33 }`.
    expect: Expect::Returns(33),
};

/// **Everything at once** — the composition case.
///
/// Every other case is deliberately narrow, so that a failure localizes to one capability. That is
/// the right shape for diagnosis and the wrong shape for answering *"can Vale actually use a Rust
/// crate?"*, because a corpus of narrow cases proves each mechanism in isolation and nothing about
/// them coexisting. Interference between them is a real failure class — a shared name resolving to
/// the wrong item, an import order dependency, a drop that only works when it is the only drop —
/// and none of the others can see it.
///
/// What this composes, in one program and one import list: free functions, a zero-arg function, a
/// Rust type inferred from a signature and bound to a local, scope-end drops on three distinct Rust
/// types, methods, a same-named method on a second type resolving by receiver, a method carrying its
/// own type parameter, an associated function with no receiver, a citizen flowing through two calls,
/// a generic function at concrete types and at a Rust type, a generic Rust type at two different
/// arguments, an item and a type reached through a **nested module path**, one reached through a
/// **re-export**, and three items whose signatures Vale **declines** sitting in the allowlist
/// without disturbing anything.
///
/// Deliberately absent: a second crate (a case names one fixture, and case 24 covers it) and
/// arithmetic (the harness supplies no builtins — a Vale-side harness gap, not an interop one).
pub const A_PROGRAM_USING_EVERYTHING_AT_ONCE: Case = Case {
    fixture: "fixtures",
    name: "everything",
    vale: r#"
exported func main() int {
  held_counter = make_counter();
  held_gauge = make_gauge();
  held_sonar = make_sonar();

  from_zero_arg = seven();
  from_free_fn = add_two_numbers(20, 22);
  from_generic_fn = pick<int, bool>(add_two_numbers(10, 5), true);
  from_generic_at_citizen = id<Counter>(make_counter());

  from_second_type = (make_gauge()).get();
  from_second_method = (make_counter()).doubled();
  from_generic_method = (make_counter()).or_else<int>(19);
  from_chained_calls = value_of_counter(bump(new()));

  from_int_holder = holder_ignore<int>(make_holder());
  from_bool_holder = bool_holder_flag(make_bool_holder());

  from_nested_type = (make_sonar()).depth_of();

  return depth_reading();
}
"#,
    allowed: &[
        // Free functions and a zero-arg one.
        "add_two_numbers",
        "seven",
        // A type, its methods, and free functions over it.
        "Counter",
        "make_counter",
        "value_of_counter",
        "bump",
        // A second type whose `get` collides with the first's by name.
        "Gauge",
        "make_gauge",
        // Generic functions.
        "pick",
        "id",
        // A generic type at two instantiations.
        "Holder",
        "make_holder",
        "make_bool_holder",
        "holder_ignore",
        "bool_holder_flag",
        // Nested module: an item and a type by path, plus one reached through a re-export.
        "instruments.depth_reading",
        "instruments.Sonar",
        "readouts.make_sonar",
        // Three declined signatures, present but unusable. Nothing else here may notice.
        "first",
        "unsigned_count",
        "half_of",
    ],
    // `instruments::depth_reading` returns 31.
    expect: Expect::Returns(31),
};

// ---------------------------------------------------------------------------
// B. Item kinds
// ---------------------------------------------------------------------------

/// A Rust type reaches Vale by inference from a signature — never by name — and its method is an
/// ordinary top-level function whose first parameter is the receiver.
pub const CALLS_A_METHOD_ON_A_RUST_TYPE: Case = Case {
    fixture: "fixtures",
    name: "method",
    vale: r#"
exported func main() int {
  return (make_counter()).get();
}
"#,
    allowed: &["make_counter", "Counter"],
    // `Counter { value: 7 }` in the fixture.
    expect: Expect::Returns(7),
};

/// An associated function with no receiver — the `Vec::new` shape.
///
/// It arrives through the same `associated_items` walk as a method, so under the
/// methods-are-not-special design it is an ordinary declaration that happens to take no
/// parameters. Nothing should need a receiver-shaped path to find it.
pub const CALLS_AN_ASSOCIATED_FUNCTION_WITH_NO_RECEIVER: Case = Case {
    fixture: "fixtures",
    name: "associated-function",
    vale: r#"
exported func main() int {
  return value_of_counter(new());
}
"#,
    // No `make_counter`: `new` is the only way a `Counter` enters this program, so the case
    // cannot pass by accidentally exercising the ordinary constructor path.
    allowed: &["Counter", "value_of_counter"],
    // `Counter::new` builds `Counter { value: 5 }`.
    expect: Expect::Returns(5),
};

/// Method discovery is a **list**, not a lucky single: two methods on one type, both callable.
pub const CALLS_TWO_METHODS_ON_ONE_TYPE: Case = Case {
    fixture: "fixtures",
    name: "two-methods",
    vale: r#"
exported func main() int {
  x = (make_counter()).get();
  return (make_counter()).doubled();
}
"#,
    allowed: &["make_counter", "Counter"],
    // `Counter { value: 7 }`, doubled.
    expect: Expect::Returns(14),
};

/// Two types' methods coexist, and each resolves to its own receiver.
///
/// `Counter::get` and `Gauge::get` share a name on purpose. There is no per-type method table under
/// this design — every method is a top-level function whose first parameter is the receiver — so
/// what could actually break is the importer pairing a method with the wrong type. It would show up
/// here as a resolution failure rather than a wrong answer, since no `Gauge` would satisfy a
/// `Counter` receiver.
pub const CALLS_METHODS_ON_TWO_DIFFERENT_RUST_TYPES: Case = Case {
    fixture: "fixtures",
    name: "two-types-methods",
    vale: r#"
exported func main() int {
  x = (make_counter()).get();
  return (make_gauge()).get();
}
"#,
    allowed: &["make_counter", "Counter", "make_gauge", "Gauge"],
    // `Gauge { reading: 20 }`.
    expect: Expect::Returns(20),
};

/// A value returned and immediately discarded still gets dropped.
///
/// A different path from the bound-local case: there is no local to hang the scope-end drop on, so
/// the drop attaches to a temporary. If only the bound path worked, this leaks silently — the
/// program still compiles and still returns the right number, which is why it needs its own case
/// rather than being assumed to follow from case 20.
pub const A_RUST_VALUE_RETURNED_AND_DISCARDED_GETS_DROPPED: Case = Case {
    fixture: "fixtures",
    name: "discarded-temporary",
    vale: r#"
exported func main() int {
  make_counter();
  return 4;
}
"#,
    allowed: &["make_counter", "Counter"],
    expect: Expect::Returns(4),
};

/// Two Rust types imported in one compilation — the importer is a loop, not a single-item path.
///
/// Deliberately free-function-only, so it does not also depend on method discovery; case 19 covers
/// the method half. One type alone would pass under an importer that handled exactly one.
pub const IMPORTS_TWO_RUST_TYPES_AT_ONCE: Case = Case {
    fixture: "fixtures",
    name: "two-types",
    vale: r#"
exported func main() int {
  x = value_of_counter(make_counter());
  return gauge_reading(make_gauge());
}
"#,
    allowed: &[
        "Counter",
        "make_counter",
        "value_of_counter",
        "Gauge",
        "make_gauge",
        "gauge_reading",
    ],
    // `Gauge { reading: 20 }`, plus 2.
    expect: Expect::Returns(22),
};

/// A Rust citizen produced by one call and consumed by another, with a third in between.
///
/// Citizen identity has to survive crossing a call boundary twice. A lowering that minted a fresh
/// kind per signature would still typecheck each call in isolation and fail only here, where the
/// *same* type has to be recognised as the one that came out of the previous call.
///
/// Shares its return value with other cases, which is unusual for this corpus — the value is
/// whatever `bump` yields from `make_counter`, and the flow is the subject rather than the number.
pub const A_RUST_TYPE_FLOWS_THROUGH_TWO_CALLS: Case = Case {
    fixture: "fixtures",
    name: "flows-through-calls",
    vale: r#"
exported func main() int {
  return value_of_counter(bump(make_counter()));
}
"#,
    allowed: &["Counter", "make_counter", "bump", "value_of_counter"],
    // `Counter { value: 7 }`, bumped once.
    expect: Expect::Returns(8),
};

/// A method carrying its **own** type parameter, on top of the container's.
///
/// The receiver is concrete, so `T` belongs to the method alone — which is the shape where an
/// item's own generic parameters sit above its parent's in rustc's parent-inclusive index.
pub const CALLS_A_GENERIC_METHOD: Case = Case {
    fixture: "fixtures",
    name: "generic-method",
    vale: r#"
exported func main() int {
  return (make_counter()).or_else<int>(19);
}
"#,
    allowed: &["make_counter", "Counter"],
    expect: Expect::Returns(19),
};

/// A Rust value bound to a local and never consumed needs a scope-end drop. `Compiler::drop`'s
/// `KindT::Struct` arm always resolves a destructor call, so the importer synthesizes a `drop` for
/// every imported type — `Counter` has no `Drop` impl, and asking rustc for a method named `drop`
/// would answer `None`.
pub const A_RUST_VALUE_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP: Case = Case {
    fixture: "fixtures",
    name: "scope-end-drop",
    vale: r#"
exported func main() int {
  c = make_counter();
  return 3;
}
"#,
    allowed: &["make_counter", "Counter"],
    expect: Expect::Returns(3),
};

/// A generic function whose parameter is **the generic type applied to its own parameter** —
/// `holder_ignore<T>(Holder<T>)` — called at `<int>`.
///
/// The shape `pick<A, B>` does not reach: its parameters are bare generics, so the declaration
/// references a rune directly with no rule at all. This one needs `LookupSR` + `CallSR` in
/// *parameter* position, and `T` is only knowable by running that call backwards from the
/// argument. It is the same inference a generic type's `drop` needs, isolated from drop.
pub const CALLS_A_GENERIC_FUNCTION_TAKING_A_GENERIC_TYPE: Case = Case {
    fixture: "fixtures",
    name: "generic-fn-generic-param",
    vale: r#"
exported func main() int {
  return holder_ignore<int>(make_holder());
}
"#,
    allowed: &["make_holder", "holder_ignore", "Holder"],
    expect: Expect::Returns(9),
};

// ---------------------------------------------------------------------------
// C. Multiplicity and crates
// ---------------------------------------------------------------------------

/// Two crates, two types with distinct short names, both reaching Vale in one compilation.
///
/// The importer builds one top-level store per package coordinate, so this is what says the
/// coordinate is really derived per item rather than shared: with a single coordinate for
/// everything, both crates' items would land in one store and one package.
pub const IMPORTS_FROM_TWO_CRATES: Case = Case {
    fixture: "fixtures_two_crates",
    name: "two-crates",
    // Both methods are called; only one result is returned, and directly rather than through a
    // local. Two Vale-side gaps shape that, neither of them interop's: `+` resolves no candidate
    // at all in this compilation, and *reading* a local yields `BorrowRef(int)` where `int` is
    // wanted (`NoImplicitCloneDefinedT`) — the same borrow read-out gap that blocks case 39, and
    // Vale2's. A case about multiplicity should not be gated on either.
    vale: r#"
exported func main() int {
  d = (make_doohickey()).doohickey_value();
  return (make_gadget()).gadget_value();
}
"#,
    allowed: &[
        "Gadget",
        "make_gadget",
        "gadget_value",
        "Doohickey",
        "make_doohickey",
        "doohickey_value",
    ],
    // `Gadget { value: 2 }`.
    expect: Expect::Returns(2),
};

/// Two crates each exporting a struct called `Widget` — the @ATAFLBZ identity hazard.
///
/// Rust has no uniqueness rule for short names, and `tcx.crates(())` hands the oracle every loaded
/// crate, so anything deciding by string equality picks whichever it meets first. The two `Widget`s
/// here have different shapes, so conflating them is a real type error rather than a harmless
/// aliasing of identical things.
pub const TWO_CRATES_EXPORTING_THE_SAME_SHORT_NAME_STAY_DISTINCT: Case = Case {
    fixture: "fixtures_two_crates",
    name: "two-crates-same-short-name",
    vale: r#"
exported func main() int {
  a = make_widget();
  b = make_other_widget();
  return 5;
}
"#,
    allowed: &["Widget", "make_widget", "make_other_widget"],
    expect: Expect::Returns(5),
};

// ---------------------------------------------------------------------------
// D. Scoping — the allowlist is load-bearing, and is the only thing that is
// ---------------------------------------------------------------------------

/// The positive control's mirror: the crate exports `seven`, and with it left out of the allowlist
/// Vale still cannot see it. Membership in the allowlist is the whole of scoping.
pub const AN_ITEM_NOT_IN_THE_ALLOWLIST_IS_NOT_IMPORTABLE: Case = Case {
    fixture: "fixtures",
    name: "item-not-allowed",
    vale: r#"
exported func main() int {
  return seven();
}
"#,
    allowed: &["add_two_numbers"],
    expect: Expect::FailsToCompile("CouldntFindFunctionToCallT"),
};

/// A stale allowlist entry naming nothing the crate exports is **inert**, not fatal. An `import`
/// list will outlive the crate versions it was written against, so a name that stops existing must
/// not take the whole compilation down with it.
pub const AN_ALLOWLIST_ENTRY_THE_CRATE_DOES_NOT_EXPORT_IS_IGNORED: Case = Case {
    fixture: "fixtures",
    name: "stale-allowlist-entry",
    vale: r#"
exported func main() int {
  return add_two_numbers(2, 8);
}
"#,
    allowed: &["add_two_numbers", "no_such_item_exists_anywhere"],
    expect: Expect::Returns(10),
};

/// A crate's module children include its own `extern crate std`, so an unfiltered name match would
/// hand back a **module** where a function or type was asked for. The walk filters on `DefKind`
/// for exactly this reason.
pub const A_MODULE_NAMED_IN_THE_ALLOWLIST_IS_FILTERED_BY_DEFKIND: Case = Case {
    fixture: "fixtures",
    name: "module-in-allowlist",
    vale: r#"
exported func main() int {
  return add_two_numbers(4, 8);
}
"#,
    allowed: &["add_two_numbers", "std"],
    expect: Expect::Returns(12),
};

// ---------------------------------------------------------------------------
// E. Failure modes — wrong programs fail, and fail legibly
// ---------------------------------------------------------------------------

/// A Rust callee competes on `params_match` like any other candidate, so wrong argument types are
/// an ordinary resolution failure rather than a special case.
pub const WRONG_ARGUMENT_TYPES_DO_NOT_RESOLVE: Case = Case {
    fixture: "fixtures",
    name: "wrong-argument-types",
    vale: r#"
exported func main() int {
  return add_two_numbers(true, 4);
}
"#,
    allowed: &["add_two_numbers"],
    expect: Expect::FailsToCompile("CouldntFindFunctionToCallT"),
};

/// Arity is checked rather than silently truncated. @ETASTZ records that
/// `build_generic_args_for_item` discards excess type args without complaint, which would turn a
/// user's mistake into a plausible wrong answer.
pub const WRONG_GENERIC_ARITY_DOES_NOT_RESOLVE: Case = Case {
    fixture: "fixtures",
    name: "wrong-generic-arity",
    vale: r#"
exported func main() int {
  return pick<int>(3, true);
}
"#,
    allowed: &["pick"],
    expect: Expect::FailsToCompile("CouldntFindFunctionToCallT"),
};

/// A Vale function and a Rust function sharing a name **do not collide**.
///
/// Candidate collection is `lookup_all_with_imprecise_name` — *plural* — so two same-named
/// functions are two candidates that overload resolution scores, and the outcome is either a clean
/// resolution or a designed `CouldntNarrowDownCandidates`. Never the `panic!("Too many with name")`
/// that a *type*-name collision produces, which is the distinction
/// `two_crates_exporting_the_same_short_name_stay_distinct` sits on the other side of.
pub const A_VALE_FUNCTION_AND_A_RUST_FUNCTION_WITH_THE_SAME_NAME: Case = Case {
    fixture: "fixtures",
    name: "same-named-function",
    vale: r#"
exported func main() int {
  return add_two_numbers(1, 2);
}
func add_two_numbers(a int, b int) int {
  return 99;
}
"#,
    allowed: &["add_two_numbers"],
    // No trailing `T` on this one, unlike most `ICompileErrorT` arms.
    expect: Expect::FailsToCompile("CouldntNarrowDownCandidates"),
};

// ---------------------------------------------------------------------------
// F/G. Provenance, and Vale source naming Rust items
// ---------------------------------------------------------------------------

/// An oracle in scope costs an ordinary Vale program nothing. The interop machinery runs on every
/// compilation in this build mode, so a program that mentions no Rust item at all must be
/// unaffected by its presence.
pub const A_PROGRAM_USING_NO_RUST_ITEMS_COMPILES_WITH_AN_ORACLE_PRESENT: Case = Case {
    fixture: "fixtures",
    name: "no-rust-items",
    vale: r#"
exported func main() int {
  return 17;
}
"#,
    allowed: &["add_two_numbers", "Counter", "make_counter"],
    expect: Expect::Returns(17),
};

/// Hand-written Vale naming a Rust type by bare name, with no import statement.
///
/// The body is deliberately trivial. `return (c).get()` here does *not* compile — reading a
/// parameter yields `BorrowRef(Counter)` where `get(self Counter)` wants it owned, and
/// `is_type_convertible` panics on that borrow read-out. That is one of the two Vale-side
/// onion-arc gaps already with Vale2, not an interop limitation, so this case stays on the naming
/// question. `c` is still unconsumed, so the synthesized `drop` runs on the way out.
pub const VALE_SOURCE_CAN_NAME_A_RUST_TYPE: Case = Case {
    fixture: "fixtures",
    name: "vale-names-a-rust-type",
    vale: r#"
exported func main() int {
  return value_of(make_counter());
}
func value_of(c Counter) int {
  return 11;
}
"#,
    allowed: &["make_counter", "Counter"],
    expect: Expect::Returns(11),
};

/// A generic Rust type imports **with its arguments intact** — `Holder<i32>` and `Holder<bool>`
/// are two distinct Vale kinds.
///
/// This asserted the *defect* until 2026-07-26, when synthesized `StructS` declarations let the
/// ordinary `LookupSR` + `CallSR` path apply arguments to a template. Before that, both interned
/// as a bare argument-less `Holder` — the same answer for different types.
pub const A_GENERIC_RUST_TYPE_CARRIES_ITS_ARGUMENTS: Case = Case {
    fixture: "fixtures",
    name: "generic-type-arguments",
    // Both `Holder`s are **consumed** by a Rust function rather than bound and left to fall out of
    // scope. Scope-end drop on a *generic* type does not resolve: the drop declaration is
    // `drop<T>(Holder<T>)`, a compiler-generated drop call supplies no explicit type argument, and
    // `T` is not in fact inferred from the argument. `holder_ignore<int>(..)` works precisely
    // because the argument is written.
    //
    // This was originally read as arch §1.7 behaving as designed. It is not — that rule was a
    // transcription of Sky's, never ratified for Vale, and has been struck. Inference here is
    // wanted, not forbidden, so this is a defect rather than a constraint. Routed to Vale2; see
    // plan §9 step 2.
    vale: r#"
exported func main() int {
  a = holder_value(make_holder());
  b = bool_holder_flag(make_bool_holder());
  return 13;
}
"#,
    allowed: &[
        "make_holder",
        "make_bool_holder",
        "holder_value",
        "bool_holder_flag",
        "Holder",
    ],
    expect: Expect::Returns(13),
};

/// The surviving hazard of hosting rustc inside `cargo test --lib`, pinned as a regression test.
/// `fixtures_broken_rust/` does not parse, so this drives a rustc **fatal** error through an
/// in-process `run_compiler`. Measured cost: this one case, not the run.
pub const A_FATAL_RUSTC_ERROR_COSTS_ONE_CASE: Case = Case {
    fixture: "fixtures_broken_rust",
    name: "fatal-rustc-error",
    vale: r#"
exported func main() int {
  return add_two_numbers(20, 22);
}
"#,
    allowed: &["add_two_numbers"],
    expect: Expect::RustcFails,
};
