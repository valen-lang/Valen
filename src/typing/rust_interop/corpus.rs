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
  /// The Vale program, including its `import rust.X.Y;` statements. `main` is the entry point and its
  /// return value is the case's observable. Importable Rust items are exactly what the program imports.
  pub vale: &'static str,
  pub expect: Expect,
}

// ---------------------------------------------------------------------------
// A. Signatures and lowering
// ---------------------------------------------------------------------------

pub const CALLS_A_RUST_FREE_FUNCTION: Case = Case {
  fixture: "fixtures",
  name: "free-function",
  vale: r#"
import rust.mycrate.add_two_numbers;
exported func main() int {
  return add_two_numbers(20, 22);
}
"#,
  expect: Expect::Returns(42),
};

/// A Rust **trait** imports as a synthesized interface — the first step toward a Vale struct
/// implementing a Rust trait so Rust can call back in. The import alone must resolve and compile;
/// the trait is unused here, exactly as an imported-but-uncalled function is.
pub const IMPORTS_A_RUST_TRAIT: Case = Case {
  fixture: "fixtures",
  name: "import-trait",
  vale: r#"
import rust.mycrate.Callback;
exported func main() int {
  return 0;
}
"#,
  expect: Expect::Returns(0),
};

/// A Vale struct implements an imported Rust trait, and the trait's method resolves through an
/// interface reference — proof the override matched. `invoke(cb &Callback)` calls `cb.on_call()`,
/// which needs `Callback` to carry an abstract `on_call`; `invoke(&c)` upcasts `MyCb` to `&Callback`,
/// which needs the impl's edge. An empty or unimplemented interface fails both. This is the frontend
/// half of Rust calling a Vale callback.
pub const A_STRUCT_IMPLEMENTS_A_RUST_TRAIT: Case = Case {
  fixture: "fixtures",
  name: "impl-rust-trait",
  vale: r#"
import rust.mycrate.Callback;
struct MyCb { }
impl Callback for MyCb;
func on_call(self &MyCb) int {
  return 7;
}
func invoke(cb &Callback) int {
  return cb.on_call();
}
exported func main() int {
  c = MyCb();
  return invoke(&c);
}
"#,
  expect: Expect::Returns(7),
};

/// An `impl` of a Rust trait that provides no override for the trait's method must be rejected: the
/// trait projects an abstract `on_call` into the interface, so `impl Callback for MyCb` with no
/// `on_call` leaves an abstract method unimplemented and fails to compile. This guards that the
/// projected abstract method is actually *enforced* — without the projection the impl would compile
/// vacuously. (This tests a mismatch Vale reliably catches; a wrong *return type* is a pre-existing,
/// native-wide gap that Vale does not yet catch — see `native_interface_rejects_a_wrong_return_override`.)
pub const A_TRAIT_IMPL_MISSING_ITS_OVERRIDE_IS_REJECTED: Case = Case {
  fixture: "fixtures",
  name: "impl-rust-trait-missing-override",
  vale: r#"
import rust.mycrate.Callback;
struct MyCb { }
impl Callback for MyCb;
func invoke(cb &Callback) int {
  return cb.on_call();
}
exported func main() int {
  c = MyCb();
  return invoke(&c);
}
"#,
  expect: Expect::FailsToCompile("CouldntFindOverrideT"),
};

/// Rust calls back into a Valen callback: a Valen `MyCb` implements the Rust trait `Callback`, and a
/// generic Rust `run_callback<C: Callback>(&C)` is monomorphized with `C = MyCb`, whose body calls
/// `c.on_call()` back into Valen's override (returning 7). Its own fixture (`fixtures_rust_trait`)
/// carries the trait, the generic fn, and a stub projecting `MyCb` + `impl Callback for MyCb`.
pub const RUST_CALLS_A_VALEN_TRAIT_IMPL_CALLBACK: Case = Case {
  fixture: "fixtures_rust_trait",
  name: "rust-trait-callback",
  vale: r#"
import rust.mycrate.Callback;
import rust.mycrate.run_callback;
struct MyCb { }
impl Callback for MyCb;
func on_call(self &MyCb) int {
  return 7;
}
exported func main() int {
  mmlcb = MyCb();
  return run_callback(&mmlcb);
}
"#,
  expect: Expect::Returns(7),
};

/// A Rust->Valen callback that takes a scalar argument. `Adder::add(&self, n: i32) -> i32`, with a
/// Valen `MyAdder` implementing it; Rust's `run_adder::<MyAdder>(&a, 35)` passes `35` inbound and
/// Valen's `add` returns it. Proves an inbound *value* crosses Rust->Valen — the `&self`-only
/// callback above never passed one. `add` returns the argument rather than transforming it because
/// the driven harness compiles no builtins, so Valen operators are unavailable here.
pub const A_VALEN_CALLBACK_TAKES_A_SCALAR_ARG: Case = Case {
  fixture: "fixtures_rust_callback_scalar",
  name: "rust-callback-scalar-arg",
  vale: r#"
import rust.mycrate.Adder;
import rust.mycrate.run_adder;
struct MyAdder { }
impl Adder for MyAdder;
func add(self &MyAdder, n int) int {
  return n;
}
exported func main() int {
  a = MyAdder();
  return run_adder(&a, 35);
}
"#,
  expect: Expect::Returns(35),
};

/// A Rust->Valen callback that receives a Rust **borrow** and calls back out to Rust through it.
/// `Ticker::on_tick(&self, w: &Counter) -> i32`, with a Valen `MyTicker` implementing it; Rust's
/// `run_ticker::<MyTicker>()` makes a `Counter` (value 5) and hands `&Counter` inbound, and Valen's
/// `on_tick` returns `w.peek()` — an outbound Rust call on the received borrow. Proves a Rust borrow
/// crosses inbound and that a callback body can itself call back out to Rust (value 5).
pub const A_VALEN_CALLBACK_RECEIVES_A_RUST_BORROW: Case = Case {
  fixture: "fixtures_rust_callback_borrow",
  name: "rust-callback-borrow-arg",
  vale: r#"
import rust.mycrate.Counter;
import rust.mycrate.Ticker;
import rust.mycrate.run_ticker;
struct MyTicker { }
impl Ticker for MyTicker;
func on_tick(self &MyTicker, w &Counter) int {
  return w.peek();
}
exported func main() int {
  t = MyTicker();
  return run_ticker(&t);
}
"#,
  expect: Expect::Returns(5),
};

/// A Rust->Valen callback that receives a Rust struct **by value**. `Summer::on_sum(&self, s: Small)
/// -> i32`, with a Valen `MySummer` implementing it; Rust's `run_summer::<MySummer>()` makes a
/// `Small { a: 3, b: 6 }` and hands it inbound by value, and Valen's `on_sum` returns `s.sum()` (a
/// by-value method that consumes it back out to Rust). Proves a small aggregate crosses inbound in
/// registers (3 + 6 = 9).
pub const A_VALEN_CALLBACK_RECEIVES_A_RUST_STRUCT_BY_VALUE: Case = Case {
  fixture: "fixtures_rust_callback_byval",
  name: "rust-callback-byval-arg",
  vale: r#"
import rust.mycrate.Small;
import rust.mycrate.Summer;
import rust.mycrate.run_summer;
struct MySummer { }
impl Summer for MySummer;
func on_sum(self &MySummer, s Small) int {
  return s.sum();
}
exported func main() int {
  m = MySummer();
  return run_summer(&m);
}
"#,
  expect: Expect::Returns(9),
};

/// Forward direction, `Pair` **return**: Vale calls a Rust associated fn returning a small `{i32,i32}`
/// struct by value (`Small2.new(3,6)`), binds it, and reads it (`s.sum()`). The struct comes back in
/// two registers and is reassembled Vale-side. Returns 9.
pub const VALE_RECEIVES_A_RUST_PAIR_RETURN: Case = Case {
  fixture: "fixtures_pair_forward",
  name: "pair-return-forward",
  vale: r#"
import rust.mycrate.Small2;
exported func main() int {
  s = Small2.new(3, 6);
  return s.sum();
}
"#,
  expect: Expect::Returns(9),
};

/// Forward direction, `Pair` **argument**: Vale passes a small `{i32,i32}` struct by value into a Rust
/// free function (`add_small(s)`). The struct crosses outbound in two registers. Returns 9.
pub const VALE_PASSES_A_RUST_PAIR_ARG: Case = Case {
  fixture: "fixtures_pair_forward",
  name: "pair-arg-forward",
  vale: r#"
import rust.mycrate.Small2;
import rust.mycrate.add_small;
exported func main() int {
  s = Small2.new(3, 6);
  return add_small(^s);
}
"#,
  expect: Expect::Returns(9),
};

/// A Rust->Valen callback that **returns** a Rust struct by value. `Maker::make(&self) -> Small`, with
/// a Valen `MyMaker` implementing it; Valen's `make` returns `Small.new(3,6)` and Rust's
/// `run_maker::<MyMaker>()` reads it (`c.make().sum()`). The struct crosses Valen -> Rust in two
/// registers (an inbound Pair return). Returns 9.
pub const A_VALEN_CALLBACK_RETURNS_A_RUST_STRUCT_BY_VALUE: Case = Case {
  fixture: "fixtures_rust_callback_retpair",
  name: "rust-callback-retpair",
  vale: r#"
import rust.mycrate.Small;
import rust.mycrate.Maker;
import rust.mycrate.run_maker;
struct MyMaker { }
impl Maker for MyMaker;
func make(self &MyMaker) Small {
  return Small.new(3, 6);
}
exported func main() int {
  m = MyMaker();
  return run_maker(&m);
}
"#,
  expect: Expect::Returns(9),
};

/// The capstone: **Rust owns a loop** that calls the Valen callback once per iteration. `main_loop::
/// <MyCb>(&cb)` loops `i = 0..5`, each iteration calling `c.on_tick(i)` into Valen's override (which
/// returns `i`), and sums the returns (0 + 1 + 2 + 3 + 4 = 10). Proves the callback survives repeated
/// re-entry with a fresh scalar each time — the NobiliaV shape where Rust drives the frame loop and
/// calls Valen's `on_tick` every frame.
pub const RUST_OWNS_A_LOOP_CALLING_THE_CALLBACK: Case = Case {
  fixture: "fixtures_rust_main_loop",
  name: "rust-main-loop",
  vale: r#"
import rust.mycrate.Looper;
import rust.mycrate.main_loop;
struct MyCb { }
impl Looper for MyCb;
func on_tick(self &MyCb, i int) int {
  return i;
}
exported func main() int {
  cb = MyCb();
  return main_loop(&cb);
}
"#,
  expect: Expect::Returns(10),
};

/// Repro (reverse direction): an imported trait whose method takes TWO imported-type borrow params and
/// returns void. Compiling the synthesized `Cb` interface's abstract `go` header panics in
/// `get_inner_env_for_type` for one of the imported param types. Proven single-param callbacks
/// (`&Counter`) work; this pins down the two-imported-param / void shape Pearl's `on_tick` needs.
pub const A_TRAIT_METHOD_WITH_TWO_IMPORTED_PARAMS: Case = Case {
  fixture: "fixtures_two_imported_params",
  name: "two-imported-params",
  vale: r#"
import rust.mycrate.Alpha;
import rust.mycrate.Beta;
import rust.mycrate.Cb;
struct MyCb { }
impl Cb for MyCb;
func go(self &MyCb, x &Alpha, y &Beta) {
  x.touch();
}
exported func main() int {
  a = Alpha.new();
  cb = MyCb();
  return a.run_cb(&cb);
}
"#,
  expect: Expect::Returns(7),
};

/// Laziness, proven positively: three representable free functions are imported and exactly one is
/// called. Only the called function's signature is ever queried. This is the whole payoff — importing
/// a type with a hundred methods must not pay `fn_sig` for the ones a program never calls.
pub const LAZY_SYNTHESIS_ONLY_QUERIES_CALLED_FUNCTIONS: Case = Case {
  fixture: "fixtures",
  name: "lazy-only-called",
  vale: r#"
import rust.mycrate.add_two_numbers;
import rust.mycrate.seven;
import rust.mycrate.is_positive;
exported func main() int {
  return add_two_numbers(3, 4);
}
"#,
  expect: Expect::Returns(7),
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
import rust.mycrate.add_two_numbers;
import rust.mycrate.pick;
exported func main() int {
  return pick<int, bool>(add_two_numbers(10, 5), true);
}
"#,
  expect: Expect::Returns(15),
};

/// An empty parameter list is the degenerate case, not a special one.
pub const CALLS_A_ZERO_ARG_RUST_FUNCTION: Case = Case {
  fixture: "fixtures",
  name: "zero-arg",
  vale: r#"
import rust.mycrate.seven;
exported func main() int {
  return seven();
}
"#,
  expect: Expect::Returns(7),
};

/// `()` lowers to `VoidT`, so the call is legal only in statement position.
pub const CALLS_A_RUST_FUNCTION_RETURNING_UNIT: Case = Case {
  fixture: "fixtures",
  name: "returns-unit",
  vale: r#"
import rust.mycrate.do_nothing;
exported func main() int {
  do_nothing();
  return 8;
}
"#,
  expect: Expect::Returns(8),
};

/// A bool round-tripping in both directions — out of one Rust signature and into another.
pub const PASSES_AND_RETURNS_A_BOOL: Case = Case {
  fixture: "fixtures",
  name: "bool-round-trip",
  vale: r#"
import rust.mycrate.is_positive;
import rust.mycrate.to_int;
exported func main() int {
  return to_int(is_positive(5));
}
"#,
  expect: Expect::Returns(1),
};

/// A Rust citizen in **argument** position of a free function — a different lowering path from
/// return position, and a different discovery path from a method.
pub const TAKES_A_RUST_TYPE_AS_A_PARAMETER: Case = Case {
  fixture: "fixtures",
  name: "rust-type-parameter",
  vale: r#"
import rust.mycrate.make_counter;
import rust.mycrate.value_of_counter;
import rust.mycrate.Counter;
exported func main() int {
  return value_of_counter(make_counter());
}
"#,
  expect: Expect::Returns(7),
};

/// The same citizen identity on both sides of one signature. If argument and return position
/// interned differently, this is where it shows.
pub const TAKES_AND_RETURNS_A_RUST_TYPE: Case = Case {
  fixture: "fixtures",
  name: "rust-type-both-sides",
  vale: r#"
import rust.mycrate.make_counter;
import rust.mycrate.bump;
import rust.mycrate.value_of_counter;
import rust.mycrate.Counter;
exported func main() int {
  return value_of_counter(bump(make_counter()));
}
"#,
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
import rust.mycrate.pick_second;
import rust.mycrate.seven;
exported func main() int {
  return pick_second<bool, int>(true, seven());
}
"#,
  expect: Expect::Returns(7),
};

/// `id<T>(T) -> T` — a floor rather than a canary. It passes under any index mapping, so all it
/// says is that substitution happens at all.
pub const INSTANTIATES_A_GENERIC_AT_ONE_PARAMETER: Case = Case {
  fixture: "fixtures",
  name: "generic-one-parameter",
  vale: r#"
import rust.mycrate.id;
exported func main() int {
  return id<int>(9);
}
"#,
  expect: Expect::Returns(9),
};

/// A Rust citizen as a **generic argument**, rather than as a parameter or return type.
pub const INSTANTIATES_A_GENERIC_AT_A_RUST_TYPE: Case = Case {
  fixture: "fixtures",
  name: "generic-at-rust-type",
  vale: r#"
import rust.mycrate.id;
import rust.mycrate.make_counter;
import rust.mycrate.value_of_counter;
import rust.mycrate.Counter;
exported func main() int {
  return value_of_counter(id<Counter>(make_counter()));
}
"#,
  expect: Expect::Returns(7),
};

/// A signature Vale cannot represent is **declined**, not imported with a hole in it. The rest of
/// the import must survive, which is why the program calls the *other* item.
pub const DECLINES_AN_UNREPRESENTABLE_SIGNATURE: Case = Case {
  fixture: "fixtures",
  name: "declined-signature",
  vale: r#"
import rust.mycrate.add_two_numbers;
import rust.mycrate.first;
exported func main() int {
  return add_two_numbers(1, 4);
}
"#,
  expect: Expect::Returns(5),
};

/// The same decline, in **argument** position. A different code path from the return position:
/// parameters are lowered in a loop and one declining drops the whole declaration, whereas the
/// return type is lowered once afterwards.
pub const DECLINES_AN_UNREPRESENTABLE_PARAMETER: Case = Case {
  fixture: "fixtures",
  name: "declined-parameter",
  vale: r#"
import rust.mycrate.add_two_numbers;
import rust.mycrate.take_first;
exported func main() int {
  return add_two_numbers(2, 4);
}
"#,
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
import rust.mycrate.add_two_numbers;
import rust.mycrate.unsigned_count;
exported func main() int {
  return add_two_numbers(3, 4);
}
"#,
  expect: Expect::Returns(7),
};

/// A declined signature that is actually **called** surfaces as a compile error, not a panic. The
/// decline cases here import an unrepresentable item but never call it, so lazy synthesis never reads
/// its signature. Calling it forces `fn_sig`, which declines (an unsigned-int return), and the compiler
/// must report `CouldNotPostparseFunction` — a real diagnostic naming the item and reason — rather than
/// aborting with a `vfail` panic.
pub const CALLING_A_DECLINED_SIGNATURE_IS_A_COMPILE_ERROR: Case = Case {
  fixture: "fixtures",
  name: "call-declined-unsigned",
  vale: r#"
import rust.mycrate.unsigned_count;
exported func main() int {
  unsigned_count();
  return 0;
}
"#,
  expect: Expect::FailsToCompile("CouldNotPostparseFunction"),
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
import rust.mycrate.add_two_numbers;
import rust.mycrate.half_of;
exported func main() int {
  return add_two_numbers(4, 4);
}
"#,
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
import rust.mycrate.add_two_numbers;
import rust.mycrate.takes_hidden;
exported func main() int {
  return add_two_numbers(4, 5);
}
"#,
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
import rust.mycrate.instruments.depth_reading;
exported func main() int {
  return depth_reading();
}
"#,
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
import rust.mycrate.instruments.Sonar;
import rust.mycrate.instruments.make_sonar;
exported func main() int {
  return (make_sonar()).depth_of();
}
"#,
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
import rust.mycrate.readouts.depth_reading;
exported func main() int {
  return depth_reading();
}
"#,
  expect: Expect::Returns(31),
};

/// The other re-export shape: descending **through** a re-exported module rather than landing on a
/// re-exported item. A walk could plausibly handle the destination and not the intermediate hop.
pub const IMPORTS_THROUGH_A_RE_EXPORTED_MODULE: Case = Case {
  fixture: "fixtures",
  name: "re-export-module",
  vale: r#"
import rust.mycrate.gear.instruments.Sonar;
import rust.mycrate.gear.instruments.make_sonar;
exported func main() int {
  return (make_sonar()).depth_of();
}
"#,
  // `Sonar { depth: 33 }`.
  expect: Expect::Returns(33),
};

/// A re-export whose target lives in **another crate**, reached by a path through the crate doing
/// the re-exporting.
///
/// Cases 46 and 47 are intra-crate. `std::vec::Vec` is not: `std` reaches it by
/// `pub use alloc_crate::vec`, so the crate a user descends through and the crate the definition
/// lives in differ, and `module_children` reports that hop differently.
pub const IMPORTS_THROUGH_A_CROSS_CRATE_RE_EXPORTED_ITEM: Case = Case {
  fixture: "fixtures_two_crates",
  name: "cross-crate-re-export-item",
  vale: r#"
import rust.othercrate.vendored.make_gadget;
import rust.othercrate.vendored.Gadget;
exported func main() int {
  return (make_gadget()).gadget_value();
}
"#,
  // `Gadget { value: 2 }`.
  expect: Expect::Returns(2),
};

/// The other cross-crate shape: descending **through** a re-exported module whose target is in
/// another crate. This is `std::vec`'s exact form, and the one a walk could plausibly get wrong
/// while handling a re-exported item correctly.
pub const IMPORTS_THROUGH_A_CROSS_CRATE_RE_EXPORTED_MODULE: Case = Case {
  fixture: "fixtures_two_crates",
  name: "cross-crate-re-export-module",
  vale: r#"
import rust.othercrate.toolkit.tools.make_spanner;
import rust.othercrate.toolkit.tools.Spanner;
exported func main() int {
  return (make_spanner()).spanner_size();
}
"#,
  // `Spanner { size: 6 }`.
  expect: Expect::Returns(6),
};

/// An item defined in the **compiled crate itself** is not importable.
///
/// The walk resolves allowlist paths against `tcx.crates(())`, which is the loaded *dependency*
/// crates, so the crate being compiled is out of scope by construction. That is the right answer —
/// the stub exists to force dependencies to load, not to export anything of its own — but it is
/// invisible until something asks for it, and a fixture that puts an item in the stub looks
/// identical to a broken walk.
///
/// `stub.rs` re-exports `add_two_numbers` from `mycrate`, so the *name* is present in the compiled
/// crate's own children; only the definition's crate makes it reachable.
pub const AN_ITEM_IN_THE_COMPILED_CRATE_IS_NOT_IMPORTABLE: Case = Case {
  fixture: "fixtures",
  name: "compiled-crate-not-importable",
  vale: r#"
import rust.stub.stub_only;
exported func main() int {
  return stub_only();
}
"#,
  // `stub_only` lives in the crate being compiled (the stub), which is not among the loaded
  // dependency crates, so naming it as `rust.stub.stub_only` resolves to nothing.
  expect: Expect::FailsToCompile("UnresolvableRustImport"),
};

/// A Vale package may not claim the reserved `rust` module.
///
/// Every synthesized declaration names its citizen by a package path rooted at `rust`, and
/// `lookup_nearest_with_path` selects a store by matching that coordinate whole. A Vale package
/// compiled as `rust` is therefore indistinguishable from an imported crate at the moment of
/// selection — and the collision is silent, because selection takes a match rather than reporting
/// two. The case compiles an ordinary program under that coordinate; what it pins is that doing so
/// is refused rather than quietly permitted.
pub const A_VALE_PACKAGE_MAY_NOT_CLAIM_THE_RUST_MODULE: Case = Case {
  fixture: "fixtures",
  name: "reserved-rust-module",
  vale: r#"
import rust.mycrate.add_two_numbers;
exported func main() int {
  return add_two_numbers(20, 22);
}
"#,
  expect: Expect::Returns(42),
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
import rust.mycrate.add_two_numbers;
import rust.mycrate.seven;
import rust.mycrate.Counter;
import rust.mycrate.make_counter;
import rust.mycrate.value_of_counter;
import rust.mycrate.bump;
import rust.mycrate.Gauge;
import rust.mycrate.make_gauge;
import rust.mycrate.pick;
import rust.mycrate.id;
import rust.mycrate.Holder;
import rust.mycrate.make_holder;
import rust.mycrate.make_bool_holder;
import rust.mycrate.holder_ignore;
import rust.mycrate.bool_holder_flag;
import rust.mycrate.instruments.depth_reading;
import rust.mycrate.instruments.Sonar;
import rust.mycrate.readouts.make_sonar;
import rust.mycrate.first;
import rust.mycrate.unsigned_count;
import rust.mycrate.half_of;
exported func main() int {
  held_counter = make_counter();
  held_gauge = make_gauge();
  held_sonar = make_sonar();
  held_holder = make_holder();

  from_zero_arg = seven();
  from_free_fn = add_two_numbers(20, 22);
  from_generic_fn = pick<int, bool>(add_two_numbers(10, 5), true);
  from_generic_at_citizen = id<Counter>(make_counter());

  from_second_type = (make_gauge()).get();
  from_second_method = (make_counter()).doubled();
  from_generic_method = (make_counter()).or_else<int>(19);
  from_chained_calls = value_of_counter(bump(Counter.new()));

  from_int_holder = holder_ignore<int>(make_holder());
  from_bool_holder = bool_holder_flag(make_bool_holder());

  from_nested_type = (make_sonar()).depth_of();
  from_vale_fn = vale_counter_value(make_counter());

  return depth_reading();
}
func vale_counter_value(c Counter) int {
  return (^c).get();
}
"#,
  // `instruments::depth_reading` returns 31.
  expect: Expect::Returns(31),
};

// ---------------------------------------------------------------------------
// B. Item kinds
// ---------------------------------------------------------------------------

/// A Rust type reaches Vale by inference from a signature — never by name — and its method lives in
/// the type's outer environment, a function whose first parameter is the receiver. `v.get()` desugars
/// to `get(v)`, which overload resolution finds via the receiver's outer env.
pub const CALLS_A_METHOD_ON_A_RUST_TYPE: Case = Case {
  fixture: "fixtures",
  name: "method",
  vale: r#"
import rust.mycrate.make_counter;
import rust.mycrate.Counter;
exported func main() int {
  return (make_counter()).get();
}
"#,
  // `Counter { value: 7 }` in the fixture.
  expect: Expect::Returns(7),
};

/// A **borrow-receiver** (`&self`) method called on a **local** — the shape a real `Vec::len`/`push`
/// takes, and the case the fixtures used to have to dodge with by-value `self`. Reading the local `c`
/// yields `BorrowRef(Counter)`, which must match `peek`'s `&self`; `c` survives the borrow and takes a
/// scope-end drop. Whether this resolves is the probe for the onion arc's reference-wrap arms.
pub const CALLS_A_BORROW_SELF_METHOD_ON_A_LOCAL: Case = Case {
  fixture: "fixtures",
  name: "borrow-self-on-local",
  vale: r#"
import rust.mycrate.make_counter;
import rust.mycrate.Counter;
exported func main() int {
  c = make_counter();
  return c.peek();
}
"#,
  // `make_counter()` builds `Counter { value: 7 }`; `peek` reads it back.
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
import rust.mycrate.Counter;
import rust.mycrate.value_of_counter;
exported func main() int {
  return value_of_counter(Counter.new());
}
"#,
  // No `make_counter`: `Counter.new()` is the only way a `Counter` enters this program, so the case
  // cannot pass by accidentally exercising the ordinary constructor path. An associated function (no
  // receiver) is named type-prefixed and resolves through the type's outer environment.
  // `Counter::new` builds `Counter { value: 5 }`.
  expect: Expect::Returns(5),
};

/// An associated function whose impl **fixes** one of the type's parameters — `impl<T> Boxed<T, Fixed>`,
/// the `Vec::new` shape where the allocator is pinned. `new` ranges over one generic (`T`); `Fixed` is
/// concrete in its return. Called with the generic on the **method**: `Boxed.new<int>()`. Because the
/// call names one type argument for `new`'s one generic, the resolver's container-vs-function rune
/// arithmetic does not underflow. The `Boxed` is consumed by `boxed_ignore`, which exercises the
/// two-param generic in argument position; the bind-a-local-and-drop shape is covered separately by
/// `A_GENERIC_ASSOC_RESULT_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP`.
pub const CALLS_AN_ASSOC_FN_FIXED_IMPL_PARAM_METHOD_GENERIC: Case = Case {
  fixture: "fixtures",
  name: "assoc-fixed-param-method-generic",
  vale: r#"
import rust.mycrate.Boxed;
import rust.mycrate.Fixed;
import rust.mycrate.boxed_ignore;
exported func main() int {
  return boxed_ignore<int>(Boxed.new<int>());
}
"#,
  // `boxed_ignore` returns 7.
  expect: Expect::Returns(7),
};

/// The same fixed-impl-param associated function, called with the generic on the **type**:
/// `Boxed<int>.new()` — the form needed for `Vec<int>.with_capacity()`. `Boxed<int>` names one
/// argument for a two-parameter type, routed as a receiving rune onto `new`'s single generic.
pub const CALLS_AN_ASSOC_FN_FIXED_IMPL_PARAM_TYPE_GENERIC: Case = Case {
  fixture: "fixtures",
  name: "assoc-fixed-param-type-generic",
  vale: r#"
import rust.mycrate.Boxed;
import rust.mycrate.Fixed;
import rust.mycrate.boxed_ignore;
exported func main() int {
  return boxed_ignore<int>(Boxed<int>.new());
}
"#,
  // `boxed_ignore` returns 7.
  expect: Expect::Returns(7),
};

/// A **two-parameter generic** value from an associated function, bound to a local and left to fall
/// out of scope — the real `let v = Vec<int, Global>.new();` shape. Its scope-end `drop` is a generated
/// call naming no type argument, so `T` (and the pinned `Fixed`) must come from the value. Probes
/// whether generic scope-end drop resolves for the associated-function/2-param shape.
pub const A_GENERIC_ASSOC_RESULT_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP: Case = Case {
  fixture: "fixtures",
  name: "generic-assoc-local-drop",
  vale: r#"
import rust.mycrate.Boxed;
import rust.mycrate.Fixed;
exported func main() int {
  b = Boxed<int>.new();
  return 5;
}
"#,
  expect: Expect::Returns(5),
};

/// The capstone: the **real** `std::vec::Vec` and `std::alloc::Global`, imported from the actual `alloc`
/// crate, exercising `Vec.new<int>()` bound to a local with a scope-end drop, against live rustc.
/// `Vec::new` lives in `impl<T> Vec<T, Global>` — one own generic (`T`), `Global` pinned — so
/// `Vec.new<int>()` returns `Vec<int, Global>`. Nothing runs; this is a typecheck.
pub const IMPORTS_REAL_VEC_AND_CONSTRUCTS_IT: Case = Case {
  fixture: "fixtures",
  name: "real-vec-new",
  vale: r#"
import rust.alloc.vec.Vec;
import rust.alloc.alloc.Global;
exported func main() int {
  v = Vec.new<int>();
  return 0;
}
"#,
  expect: Expect::Returns(0),
};

/// A Rust **enum** imported as an opaque sealed interface (`KindT::Interface`), with an inherent method
/// called on it — the opaque tier's payoff (`Option::unwrap`'s shape: a method without the variants
/// being represented). `make_shade()` returns the enum; `.level()` is its inherent `self` method.
pub const CALLS_A_METHOD_ON_AN_IMPORTED_ENUM: Case = Case {
  fixture: "fixtures",
  name: "enum-method",
  vale: r#"
import rust.mycrate.Shade;
import rust.mycrate.make_shade;
exported func main() int {
  return (make_shade()).level();
}
"#,
  // `make_shade()` builds `Shade::Bright`; `level` returns 2.
  expect: Expect::Returns(2),
};

/// An imported enum bound to a local and never consumed gets a scope-end drop — an interface's drop,
/// synthesized on demand exactly like a struct's.
pub const AN_IMPORTED_ENUM_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP: Case = Case {
  fixture: "fixtures",
  name: "enum-scope-end-drop",
  vale: r#"
import rust.mycrate.Shade;
import rust.mycrate.make_shade;
exported func main() int {
  s = make_shade();
  return 4;
}
"#,
  expect: Expect::Returns(4),
};

/// A Rust `usize` imported as the Vale `usize` **primitive** (alongside `int`/`bool`/`float`), rather
/// than declining as it used to. `some_size() -> usize` produces one and `consume_usize(usize) -> i32`
/// takes it, so `usize` is exercised in both return and argument position. It is a distinct primitive —
/// never `int` — and needs no drop.
pub const CALLS_A_FUNCTION_RETURNING_USIZE: Case = Case {
  fixture: "fixtures",
  name: "usize-primitive",
  vale: r#"
import rust.mycrate.some_size;
import rust.mycrate.consume_usize;
exported func main() int {
  return consume_usize(some_size());
}
"#,
  // `consume_usize` returns 8.
  expect: Expect::Returns(8),
};

/// Real `Vec` with a **`&mut self` method call**: `v.push(42)`. `push` is `fn push(&mut self, value: T)`
/// — a borrow receiver (slice 1) plus an element of the type's generic `T`. The local `v` is read as a
/// borrow to match `&mut self`, then still dropped at scope end.
pub const CALLS_PUSH_ON_A_REAL_VEC: Case = Case {
  fixture: "fixtures",
  name: "real-vec-push",
  vale: r#"
import rust.alloc.vec.Vec;
import rust.alloc.alloc.Global;
exported func main() int {
  v = Vec.new<int>();
  v.push(42);
  return 0;
}
"#,
  expect: Expect::Returns(0),
};

/// The capstone: real `Vec::pop() -> Option<int>` then `Option::unwrap() -> int`, tying every piece
/// together — a `&mut self` method on a struct returning a real `std` **enum** (`Option`, imported as
/// an opaque interface), whose inherent `unwrap` consumes it and hands back the element. `Option` comes
/// from the real `core` crate. Nothing runs; this is a typecheck against live rustc.
pub const CALLS_POP_THEN_UNWRAP_ON_A_REAL_VEC: Case = Case {
  fixture: "fixtures",
  name: "real-vec-pop-unwrap",
  vale: r#"
import rust.alloc.vec.Vec;
import rust.alloc.alloc.Global;
import rust.core.option.Option;
exported func main() int {
  v = Vec.new<int>();
  return (v.pop()).unwrap();
}
"#,
  expect: Expect::Returns(0),
};

/// Real `Vec` with a **`&self` method returning `usize`**: `v.len()`. Combines the borrow receiver
/// (slice 1) and the `usize` primitive (slice 3); the returned `usize` is passed to `consume_usize`.
pub const CALLS_LEN_ON_A_REAL_VEC: Case = Case {
  fixture: "fixtures",
  name: "real-vec-len",
  vale: r#"
import rust.alloc.vec.Vec;
import rust.alloc.alloc.Global;
import rust.mycrate.consume_usize;
exported func main() int {
  v = Vec.new<int>();
  return consume_usize(v.len());
}
"#,
  // `consume_usize` returns 8.
  expect: Expect::Returns(8),
};

/// A method on a **generic** type, whose signature names the type's own parameter. `Holder<int>`'s
/// `into_value(self) -> T` returns `T`, which is inherited from the impl (`impl<T> Holder<T>`), not
/// declared by the method. This is the shape that used to decline as `InheritedParameter`; the fix is
/// the parent-inclusive generic list.
pub const CALLS_A_METHOD_NAMING_THE_TYPES_GENERIC: Case = Case {
  fixture: "fixtures",
  // Distinct from CALLS_A_GENERIC_METHOD's "generic-method": the name is the per-case build out_dir
  // (`vale-interop-cases/<name>`), so a shared name races two tests on one `libmycrate.rlib`.
  name: "method-naming-types-generic",
  vale: r#"
import rust.mycrate.Holder;
import rust.mycrate.make_holder;
exported func main() int {
  return (make_holder()).into_value();
}
"#,
  // Called on the rvalue directly (not a local), so the owned `self` matches without the borrow-read
  // gap that reading a local variable would trip.
  // `Holder<int> { value: 9 }`, unwrapped.
  expect: Expect::Returns(9),
};

/// Method discovery is a **list**, not a lucky single: two methods on one type, both callable.
pub const CALLS_TWO_METHODS_ON_ONE_TYPE: Case = Case {
  fixture: "fixtures",
  name: "two-methods",
  vale: r#"
import rust.mycrate.make_counter;
import rust.mycrate.Counter;
exported func main() int {
  x = (make_counter()).get();
  return (make_counter()).doubled();
}
"#,
  // `Counter { value: 7 }`, doubled.
  expect: Expect::Returns(14),
};

/// Two types' methods coexist, and each resolves to its own receiver.
///
/// `Counter::get` and `Gauge::get` share a name on purpose. Each lives in its own type's outer
/// environment, so what could actually break is the importer pairing a method with the wrong type. It
/// would show up here as a resolution failure rather than a wrong answer, since no `Gauge` would
/// satisfy a `Counter` receiver.
pub const CALLS_METHODS_ON_TWO_DIFFERENT_RUST_TYPES: Case = Case {
  fixture: "fixtures",
  name: "two-types-methods",
  vale: r#"
import rust.mycrate.make_counter;
import rust.mycrate.Counter;
import rust.mycrate.make_gauge;
import rust.mycrate.Gauge;
exported func main() int {
  x = (make_counter()).get();
  return (make_gauge()).get();
}
"#,
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
import rust.mycrate.make_counter;
import rust.mycrate.Counter;
exported func main() int {
  make_counter();
  return 4;
}
"#,
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
import rust.mycrate.Counter;
import rust.mycrate.make_counter;
import rust.mycrate.value_of_counter;
import rust.mycrate.Gauge;
import rust.mycrate.make_gauge;
import rust.mycrate.gauge_reading;
exported func main() int {
  x = value_of_counter(make_counter());
  return gauge_reading(make_gauge());
}
"#,
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
import rust.mycrate.Counter;
import rust.mycrate.make_counter;
import rust.mycrate.bump;
import rust.mycrate.value_of_counter;
exported func main() int {
  return value_of_counter(bump(make_counter()));
}
"#,
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
import rust.mycrate.make_counter;
import rust.mycrate.Counter;
exported func main() int {
  return (make_counter()).or_else<int>(19);
}
"#,
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
import rust.mycrate.make_counter;
import rust.mycrate.Counter;
exported func main() int {
  c = make_counter();
  return 3;
}
"#,
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
import rust.mycrate.make_holder;
import rust.mycrate.holder_ignore;
import rust.mycrate.Holder;
exported func main() int {
  return holder_ignore<int>(make_holder());
}
"#,
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
import rust.mycrate.Gadget;
import rust.mycrate.make_gadget;
import rust.othercrate.Doohickey;
import rust.othercrate.make_doohickey;
exported func main() int {
  d = (make_doohickey()).doohickey_value();
  return (make_gadget()).gadget_value();
}
"#,
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
import rust.mycrate.Widget;
import rust.othercrate.Widget;
import rust.mycrate.make_widget;
import rust.othercrate.make_other_widget;
exported func main() int {
  a = make_widget();
  b = make_other_widget();
  return 5;
}
"#,
  expect: Expect::Returns(5),
};

/// The distinctness half, and the only shape that can observe it.
///
/// The case above proves both `Widget`s *import*. It cannot prove they stayed **distinct** — if
/// Vale had conflated them into one kind, that program would still typecheck, because every call in
/// it is consistent within its own crate. Distinctness is only visible when the two are *crossed*,
/// and then it shows up as a failure: `widget_value` takes `mycrate`'s `Widget`, and handing it
/// `othercrate`'s must not resolve.
///
/// So this is a negative case by necessity rather than by preference — passing here means the
/// compiler rejected a program, and a regression that merged the two types would make it start
/// compiling.
pub const A_TYPE_FROM_ONE_CRATE_DOES_NOT_SATISFY_THE_OTHERS_PARAMETER: Case = Case {
  fixture: "fixtures_two_crates",
  name: "two-crates-crossed",
  vale: r#"
import rust.mycrate.Widget;
import rust.othercrate.Widget;
import rust.mycrate.make_widget;
import rust.mycrate.widget_value;
import rust.othercrate.make_other_widget;
exported func main() int {
  return widget_value(make_other_widget());
}
"#,
  expect: Expect::FailsToCompile("CouldntFindFunctionToCallT"),
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
import rust.mycrate.add_two_numbers;
exported func main() int {
  return seven();
}
"#,
  expect: Expect::FailsToCompile("CouldntFindFunctionToCallT"),
};

/// A stale allowlist entry naming nothing the crate exports is **inert**, not fatal. An `import`
/// list will outlive the crate versions it was written against, so a name that stops existing must
/// not take the whole compilation down with it.
pub const AN_ALLOWLIST_ENTRY_THE_CRATE_DOES_NOT_EXPORT_IS_IGNORED: Case = Case {
  fixture: "fixtures",
  name: "stale-allowlist-entry",
  vale: r#"
import rust.mycrate.add_two_numbers;
import rust.mycrate.no_such_item_exists_anywhere;
exported func main() int {
  return add_two_numbers(2, 8);
}
"#,
  // The crate exports no such item, so the import resolves to nothing — an error now, rather than a
  // silently-ignored allowlist entry.
  expect: Expect::FailsToCompile("UnresolvableRustImport"),
};

/// A crate's module children include its own `extern crate std`, so an unfiltered name match would
/// hand back a **module** where a function or type was asked for. The walk filters on `DefKind`
/// for exactly this reason.
pub const A_MODULE_NAMED_IN_THE_ALLOWLIST_IS_FILTERED_BY_DEFKIND: Case = Case {
  fixture: "fixtures",
  name: "module-in-allowlist",
  vale: r#"
import rust.mycrate.add_two_numbers;
import rust.mycrate.std;
exported func main() int {
  return add_two_numbers(4, 8);
}
"#,
  // `std` names a module, not a fn/struct, so the import resolves to nothing — an error now, rather
  // than a silently-ignored allowlist entry.
  expect: Expect::FailsToCompile("UnresolvableRustImport"),
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
import rust.mycrate.add_two_numbers;
exported func main() int {
  return add_two_numbers(true, 4);
}
"#,
  expect: Expect::FailsToCompile("CouldntFindFunctionToCallT"),
};

/// **More** type arguments than the item declares does not resolve. `pick<A, B>` has two slots and
/// the call names three, so silently dropping the excess and resolving anyway would turn a user's
/// mistake into a plausible wrong answer.
///
/// **Under-supply is deliberately not what this pins**, and the distinction is the whole point:
/// `pick<int>(3, true)` is legal, because argument types reach the call-site solve and deduce `B`
/// from the argument. A case written against the under-supplied form tests inference's absence
/// rather than arity, and stops meaning anything the moment inference works.
pub const WRONG_GENERIC_ARITY_DOES_NOT_RESOLVE: Case = Case {
  fixture: "fixtures",
  name: "wrong-generic-arity",
  vale: r#"
import rust.mycrate.pick;
exported func main() int {
  return pick<int, bool, int>(3, true);
}
"#,
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
import rust.mycrate.add_two_numbers;
exported func main() int {
  return add_two_numbers(1, 2);
}
func add_two_numbers(a int, b int) int {
  return 99;
}
"#,
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
import rust.mycrate.add_two_numbers;
import rust.mycrate.Counter;
import rust.mycrate.make_counter;
exported func main() int {
  return 17;
}
"#,
  expect: Expect::Returns(17),
};

/// Hand-written Vale naming a Rust type by bare name, with no import statement.
///
/// The body is deliberately trivial, so this case pins naming and nothing else — case 39 covers
/// reading the parameter into a receiver. `c` is never consumed here, so the synthesized `drop`
/// runs on the way out, which is the second thing this case holds.
pub const VALE_SOURCE_CAN_NAME_A_RUST_TYPE: Case = Case {
  fixture: "fixtures",
  name: "vale-names-a-rust-type",
  vale: r#"
import rust.mycrate.make_counter;
import rust.mycrate.Counter;
exported func main() int {
  return value_of(make_counter());
}
func value_of(c Counter) int {
  return 11;
}
"#,
  expect: Expect::Returns(11),
};

/// A **generic** Rust value bound to a local and never consumed needs a scope-end drop.
///
/// Case 20 covers the non-generic shape. This one differs in that the synthesized declaration is
/// `drop<T>(Holder<T>)` and a compiler-generated drop call names no explicit type argument, so `T`
/// has to come from the value being dropped.
pub const A_GENERIC_RUST_TYPE_GETS_A_SCOPE_END_DROP: Case = Case {
  fixture: "fixtures",
  name: "generic-scope-end-drop",
  vale: r#"
import rust.mycrate.make_holder;
import rust.mycrate.Holder;
exported func main() int {
  h = make_holder();
  return 17;
}
"#,
  expect: Expect::Returns(17),
};

/// Hand-written Vale naming a Rust type in a parameter **and calling a method on it**.
///
/// Case 38 covers naming alone, with a body that never touches the parameter. This one reads the
/// parameter into a receiver position, which is the half that was never exercised.
///
/// **The `^` is load-bearing.** A Rust method with a by-value receiver lowers to
/// `get(self Counter)`, which *consumes*, so the caller has to move — the same rule that makes
/// `drop(bare_local)` an error and `drop(^local)` correct. Without it a bare mention reads as a
/// borrow, no candidate takes a borrow, and the call reports `CouldntFindFunctionToCallT`. That
/// error names the callee rather than the mention, so it reads like a missing import; the fix is at
/// the call site.
pub const VALE_SOURCE_CALLS_A_METHOD_ON_A_NAMED_RUST_PARAMETER: Case = Case {
  fixture: "fixtures",
  name: "vale-param-method-call",
  vale: r#"
import rust.mycrate.make_counter;
import rust.mycrate.Counter;
exported func main() int {
  return value_of(make_counter());
}
func value_of(c Counter) int {
  return (^c).get();
}
"#,
  // `Counter { value: 7 }` in the fixture.
  expect: Expect::Returns(7),
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
  // Both `Holder`s are **consumed** by a Rust function so the case observes the two distinct kinds
  // directly. A scope-end drop of a generic local also resolves now — the generated `drop<T>(Holder<T>)`
  // call infers `T` from the value (see `a_generic_rust_type_gets_a_scope_end_drop`) — so consuming
  // here is a choice, not a workaround.
  vale: r#"
import rust.mycrate.make_holder;
import rust.mycrate.make_bool_holder;
import rust.mycrate.holder_value;
import rust.mycrate.bool_holder_flag;
import rust.mycrate.Holder;
exported func main() int {
  a = holder_value(make_holder());
  b = bool_holder_flag(make_bool_holder());
  return 13;
}
"#,
  expect: Expect::Returns(13),
};

/// A Rust struct that **wraps a `HashMap`**, exercised end to end through methods — the "collection
/// held behind an opaque type" shape. `Domino` hides a `HashMap<i32, Glyph>` field that never crosses
/// into Vale, so none of the map's generics or trait bounds reach the importer. The program builds a
/// `Domino`, adds a `Glyph` through a `&mut self` method, reads one back through a `&self` method
/// returning a **borrow** (`&Glyph`) bound to a local, and reads the glyph's field through an accessor.
///
/// The borrow-return bound to a local (`d_ref`) is the new mechanic here: earlier cases proved borrow
/// *receivers*, never a borrow *return* of a citizen held in a local.
pub const A_STRUCT_WRAPPING_A_HASHMAP_IS_USED_THROUGH_METHODS: Case = Case {
  fixture: "fixtures",
  name: "domino-glyphs",
  vale: r#"
import rust.mycrate.Domino;
import rust.mycrate.Glyph;
exported func main() int {
  d = Domino.new();
  d.add_glyph(Glyph.new(7));
  d_ref = d.get_glyph(7);
  return d_ref.location();
}
"#,
  // The glyph stored under key 7 has location 7, so `d_ref.location()` is 7.
  expect: Expect::Returns(7),
};

/// Rust's `&mut` mutation, mirrored into Vale groups. `nudge(a: &mut Counter, b: &Counter)` imports as
/// `func nudge<g0', g1'>(a &Counter in g0, b &Counter in g1) mut(g0)`. Passing the same local for both
/// arguments aliases it into the mutated group `g0` and the disjoint group `g1`, which the callee is
/// entitled to treat as non-aliasing — so the borrow checker rejects it.
pub const A_MUT_BORROW_ALIASING_A_SHARED_BORROW_IS_REJECTED: Case = Case {
  fixture: "fixtures",
  name: "nudge-aliasing-rejected",
  vale: r#"
import rust.mycrate.Counter;
import rust.mycrate.nudge;
exported func main() int {
  s = Counter.new();
  return nudge(&s, &s);
}
"#,
  expect: Expect::FailsToCompile("BorrowCheckError"),
};

/// The disjoint counterpart: `nudge(&s, &t)` sends two *distinct* locals into `nudge`'s mutated and
/// shared groups, so nothing aliases and the program compiles. Guards against the group mirroring
/// over-rejecting every two-borrow call.
pub const A_MUT_BORROW_AND_A_SHARED_BORROW_OF_DISTINCT_LOCALS_IS_CLEAN: Case = Case {
  fixture: "fixtures",
  name: "nudge-distinct-clean",
  vale: r#"
import rust.mycrate.Counter;
import rust.mycrate.nudge;
exported func main() int {
  s = Counter.new();
  t = Counter.new();
  return nudge(&s, &t);
}
"#,
  expect: Expect::Returns(5),
};

/// A Rust function that shares one lifetime across two parameters (`fn tie<'a>(a: &'a mut Counter,
/// b: &'a mut Counter)`) is declined, not imported. Faithfully mirroring it would tie both parameters
/// into one group, which needs lifetime decoding Vale does not do yet — so rather than guess the two
/// are disjoint (what per-parameter groups assume), calling it is a `CouldNotPostparseFunction` error.
pub const CALLING_A_SHARED_PARAMETER_LIFETIME_IMPORT_IS_A_COMPILE_ERROR: Case = Case {
  fixture: "fixtures",
  name: "call-shared-lifetime",
  vale: r#"
import rust.mycrate.Counter;
import rust.mycrate.tie;
exported func main() int {
  s = Counter.new();
  t = Counter.new();
  return tie(&s, &t);
}
"#,
  expect: Expect::FailsToCompile("CouldNotPostparseFunction"),
};

/// A large struct crosses the boundary BY VALUE as an argument: `domino_size(d)` moves a 48-byte
/// `Domino` into a Rust free function. rustc classifies the parameter `PassMode::Indirect`, so it must
/// cross as LLVM `byval` (a pointer to a caller-owned copy, ownership moved to the callee). This is the
/// argument mirror of the sret return; one glyph is inserted before the move, so `domino_size` returns 1.
pub const DOMINO_BY_VALUE_ARG: Case = Case {
  fixture: "fixtures",
  name: "domino-byval-arg",
  vale: r#"
import rust.mycrate.Domino;
import rust.mycrate.Glyph;
import rust.mycrate.domino_size;
exported func main() int {
  d = Domino.new();
  d.add_glyph(Glyph.new(7));
  return domino_size(^d);
}
"#,
  expect: Expect::Returns(1),
};

/// A byval argument BEHIND an sret return: `add_and_return(^d, 7)` moves a 48-byte `Domino` in by value
/// (`Indirect`) and returns a `Domino` by value (`Indirect`/sret). The sret out-pointer occupies the
/// first physical parameter, so the byval `Domino` is the second; a byval attribute placed by logical
/// argument index would land on the sret pointer. The returned domino has a glyph at key 7 (location 7).
pub const DOMINO_BYVAL_ARG_WITH_SRET_RETURN: Case = Case {
  fixture: "fixtures",
  name: "domino-byval-arg-sret-return",
  vale: r#"
import rust.mycrate.Domino;
import rust.mycrate.Glyph;
import rust.mycrate.add_and_return;
exported func main() int {
  d = Domino.new();
  d2 = add_and_return(^d, 7);
  d_ref = d2.get_glyph(7);
  return d_ref.location();
}
"#,
  expect: Expect::Returns(7),
};

/// A Rust function returns an 8-byte struct BY VALUE: rustc `PassMode::Cast` crossing as a single `i64`
/// (count 1). Vale reassembles the `Small8` from the `i64` and reads field `a` (=6) through a borrow.
/// This is `PieceId`'s shape (`unpack_id`).
pub const SMALL8_CAST_RETURN: Case = Case {
  fixture: "fixtures",
  name: "small8-cast-return",
  vale: r#"
import rust.mycrate.Small8;
import rust.mycrate.make_small;
exported func main() int {
  s = make_small(6, 1, 2);
  return s.small_a();
}
"#,
  expect: Expect::Returns(6),
};

/// A Vale program passes an 8-byte struct BY VALUE into a Rust function: rustc `PassMode::Cast` crossing
/// as a single `i64`, alongside a scalar arg. `small_plus(^s, 4)` returns `s.a + 4` = 10. This is the
/// Cast argument direction (`pack_id`'s shape).
pub const SMALL8_CAST_ARG: Case = Case {
  fixture: "fixtures",
  name: "small8-cast-arg",
  vale: r#"
import rust.mycrate.Small8;
import rust.mycrate.make_small;
import rust.mycrate.small_plus;
exported func main() int {
  s = make_small(6, 1, 2);
  return small_plus(^s, 4);
}
"#,
  expect: Expect::Returns(10),
};

/// The surviving hazard of hosting rustc inside `cargo test --lib`, pinned as a regression test.
/// `fixtures_broken_rust/` does not parse, so this drives a rustc **fatal** error through an
/// in-process `run_compiler`. Measured cost: this one case, not the run.
pub const A_FATAL_RUSTC_ERROR_COSTS_ONE_CASE: Case = Case {
  fixture: "fixtures_broken_rust",
  name: "fatal-rustc-error",
  vale: r#"
import rust.mycrate.add_two_numbers;
exported func main() int {
  return add_two_numbers(20, 22);
}
"#,
  expect: Expect::RustcFails,
};

// ---------------------------------------------------------------------------
// H. Borrow checking across the interop boundary (RED — awaiting importer group facts)
// ---------------------------------------------------------------------------
//
// The borrow checker catches use-after-churn natively: a reference into a group's child (an array
// element, a returned `&T in g[]`) is invalid after a call that declares `mut(g)`. The checker reads
// those facts off the callee's scout `FunctionS` — `effects` and the group-annotated param/return
// `ITypeST` — and is entirely source-agnostic, so it would work across the Rust boundary too, if the
// importer attached the same facts to a synthesized Rust declaration.
//
// It does not yet. `declarations.rs` builds each imported function with `&[]` effects ("an extern
// Rust function's body is opaque, so it declares none") and `None` for the return group. So a Rust
// `&mut self` method carries no `mut(g)`, and a Rust `&self` method returning `&T` carries no return
// group — the checker sees no churn and no tracked reference, and the program below compiles.
//
// These cases are the spec for teaching the importer to translate Rust's own borrow facts into Vale
// groups: `&mut self` → `mut(g)` on the receiver's group; a returned reference borrowed from `&self`
// → `&T in g`. They are RED until that lands, and no borrow-checker change is needed — only the
// importer.

/// A `&Glyph` returned by a Rust `&self` method (`get_glyph`) is used after a Rust `&mut self` method
/// (`add_glyph`) churns its owner — a use-after-churn through the interop boundary, the Rust-`Vec`
/// shape (an element reference held across a mutation). RED until the importer emits `mut(g)` for
/// `&mut self` and a return group for a `&self`-borrowed return.
pub const USE_AFTER_CHURN_THROUGH_A_RUST_BORROW_RETURN: Case = Case {
  fixture: "fixtures",
  name: "interop-use-after-churn",
  vale: r#"
import rust.mycrate.Domino;
import rust.mycrate.Glyph;
exported func main() int {
  d = Domino.new();
  d.add_glyph(Glyph.new(7));
  d_ref = d.get_glyph(7);
  d.add_glyph(Glyph.new(8));
  return d_ref.location();
}
"#,
  expect: Expect::FailsToCompile("BorrowCheckError"),
};

/// RED: a `std::vec::Vec` element accessor should be importable, so `v.get(0)` should resolve. It
/// does not today — `get`/`first`/index are slice methods reached through `Deref<Target=[T]>`, and
/// the importer discovers only a type's *inherent* methods (`new`/`push`/`pop`/`len`), never
/// Deref-reached ones — so the call fails with `CouldntFindFunctionToCallT`, upstream of the borrow
/// checker. This is the first blocker to a real-`Vec` element use-after-churn; the R test drives the
/// importer to follow `Deref<Target=[T]>`.
///
/// Two further blockers sit behind it (so the use-after-churn R test itself uses the Domino wrapper,
/// whose *inherent* `get_glyph` returns a bare `&Glyph`): a `Vec` element accessor returns
/// `Option<&T>` — a group nested in a reference-typed field, which the checker does not yet track —
/// and using a `&int` element needs a read-out the pass does not do yet.
pub const REAL_VEC_ELEMENT_ACCESSOR_IS_IMPORTABLE: Case = Case {
  fixture: "fixtures",
  name: "interop-vec-element-accessor-importable",
  vale: r#"
import rust.alloc.vec.Vec;
import rust.alloc.alloc.Global;
import rust.core.option.Option;
exported func main() int {
  v = Vec.new<int>();
  v.push(7);
  e = (v.get(0)).unwrap();
  return 0;
}
"#,
  expect: Expect::Returns(0),
};

/// The clean companion — a negative control: taking the `&Glyph` borrow *after* the last churn is
/// valid, so this program must compile. It shares the fixture and the churn method with the case
/// above; what differs is only the order (`add_glyph` before `get_glyph`), which is what use-after-
/// churn turns on. Once the importer carries the group facts, this must stay `Returns`, guarding the
/// churn rule against over-rejection. GREEN today (nothing is checked), RED never.
pub const RUST_BORROW_RETURN_TAKEN_AFTER_LAST_CHURN_IS_CLEAN: Case = Case {
  fixture: "fixtures",
  name: "interop-borrow-after-last-churn",
  vale: r#"
import rust.mycrate.Domino;
import rust.mycrate.Glyph;
exported func main() int {
  d = Domino.new();
  d.add_glyph(Glyph.new(7));
  d.add_glyph(Glyph.new(8));
  d_ref = d.get_glyph(7);
  return d_ref.location();
}
"#,
  expect: Expect::Returns(7),
};
