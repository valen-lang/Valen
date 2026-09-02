// Tier 1 of the interop corpus: each case's Vale program typechecked against a real `TyCtxt`, then
// observed through the typed AST and the oracle log.
//
// **The programs are not here.** A case is data — fixture, program, allowlist, expectation — and
// lives in `rust_interop/corpus.rs` so that tier 2 can run the identical program and check what it
// returns, without either tier owning the text. What lives here is the *observation*: how to look
// at a typed AST, which is tier-1-specific and needs `collect_*`, which exists only under
// `cfg(test)`.
//
// Two rules shape what these assert on, both learned the hard way (plan §5):
//
//   - **Prefer the Vale program to carry the assertion.** `pick<int, bool>` returning `A` means a
//     swapped generic index yields `bool` where `int` belongs, and `main() int` stops typechecking.
//     That survives any refactor of how the compiler renders anything. Substring assertions against
//     `Debug` output broke twice in one day and neither break was a behaviour change.
//   - **The oracle log's one remaining job is vacuity** — proving the oracle was consulted at all,
//     which no source program can express, and which caught a compilation that silently built
//     nothing on its first run.
//
// Where a case does look at the typed AST, it goes through `describe_kind` and `Callee` below
// rather than at `Debug` renderings, so an assertion names a type the way source does.

use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use crate::collect_where_tnode;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::names::names::{INameT, IdT};
use crate::typing::rust_interop::corpus::*;
use crate::typing::rust_interop::{
  citizen_id, is_rust_backed, peel_refs, Case, OracleQuery, RustItemId, SigPosition,
};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::test::rust_interop::harness::{
  compile_check_fixture, run_case, run_case_in_package, run_case_instantiated,
  run_case_rustc_driven, run_case_rustc_driven_and_run, run_case_rustc_driven_emitting,
  run_case_rustc_driven_full, try_run_case,
  CaseOutcome,
};
use crate::typing::test::traverse::NodeRefT;
use crate::typing::types::types::*;

/// A test-owned vocabulary for naming a kind, so assertions describe a type the way source does
/// instead of keying on `Debug` shape. Reference wraps are peeled: no case here is about
/// borrow-vs-own, and the fixture is by-value throughout (see `mycrate.rs`).
fn describe_kind(kind: KindT) -> String {
  match peel_refs(kind) {
    KindT::Int(i) => format!("int{}", i.bits),
    KindT::Bool(_) => "bool".to_string(),
    KindT::Void(_) => "void".to_string(),
    KindT::Str(_) => "str".to_string(),
    KindT::Float(_) => "float".to_string(),
    other => match citizen_id(other) {
      Some(id) if is_rust_backed(id) => format!("rust-citizen{}", describe_args(id)),
      Some(id) => format!("vale-citizen{}", describe_args(id)),
      None => "non-citizen".to_string(),
    },
  }
}

/// A citizen's template arguments, rendered the way source writes them, or empty for none.
///
/// Worth having rather than collapsing every citizen to one word: without it, a case about generic
/// arguments can only assert "these two differ", which says nothing about *how* — and the whole
/// defect this pins was two instantiations rendering identically.
fn describe_args(id: &IdT) -> String {
  let args = match id.local_name {
    INameT::Struct(name) => name.template_args,
    _ => &[],
  };
  if args.is_empty() {
    return String::new();
  }
  let rendered: Vec<String> = args
    .iter()
    .map(|arg| match arg {
      ITemplataT::Kind(k) => describe_kind(k.kind),
      _ => "non-kind".to_string(),
    })
    .collect();
  format!("<{}>", rendered.join(", "))
}

/// The facts about one resolved callee that survive the callback — owned, so they can escape.
#[derive(Debug, PartialEq, Eq)]
struct Callee {
  name: String,
  /// Whether the callee itself lives in the reserved `rust` package. Only the interop seam
  /// mints ids there, so this *is* the proof that resolution came from Rust rather than from
  /// some Vale-side coincidence.
  rust_backed: bool,
  params: Vec<String>,
  ret: String,
}

fn describe_callee(p: &PrototypeT) -> Callee {
  // A Rust callee resolves to an ordinary Vale `Function` — the extern *wrapper* that
  // `make_extern_function` builds. The `ExternFunctionNameT` prototype still exists, one level
  // down, as the target of the `ExternFunctionCallTE` in that wrapper's body. Seeing an
  // `ExternFunction` here would mean a prototype leaked into a call site, which is the shape
  // the synthesized-declaration design exists to prevent — so it is worth failing loudly on.
  match p.id.local_name {
    INameT::Function(f) => Callee {
      name: f.template.human_name.0.to_string(),
      rust_backed: is_rust_backed(&p.id),
      // Params ride the name because `PrototypeT::param_types` is name-derived; a name that
      // disagreed with the signature would report wrong types at every call site.
      params: f.parameters.iter().map(|k| describe_kind(*k)).collect(),
      ret: describe_kind(p.return_type),
    },
    other => panic!("expected the callee to be an ordinary Vale function, got {other:?}"),
  }
}

/// Every call `main`'s body makes, in traversal order.
fn callees_in_main(coutputs: &HinputsT) -> Vec<Callee> {
  let main = coutputs.lookup_function_by_str("main");
  let callees: Vec<&PrototypeT> = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(call) => Some(call.callable)
  );
  callees.iter().map(|p| describe_callee(p)).collect()
}

/// The handle the oracle offered under `name`, from whichever enumerating query offered it.
fn offered<R>(outcome: &CaseOutcome<R>, name: &str) -> RustItemId {
  outcome.oracle_log.iter().find_map(|c| c.query.offered(name)).unwrap_or_else(|| {
    panic!(
      "the oracle never offered an item named {name:?}, so anything this case \
                 concludes about it is vacuous:\n--- oracle log ---\n{}",
      outcome.rendered_log()
    )
  })
}

#[test]
fn calls_a_rust_free_function() {
  let outcome = run_case(&CALLS_A_RUST_FREE_FUNCTION, callees_in_main);

  assert_eq!(
    &vec![Callee {
      name: "add_two_numbers".to_string(),
      rust_backed: true,
      params: vec!["int32".to_string(), "int32".to_string()],
      ret: "int32".to_string(),
    }],
    outcome.check(&CALLS_A_RUST_FREE_FUNCTION).expect("the case declares it compiles")
  );

  // Vacuity: the program above would compile just as happily if a Vale `add_two_numbers` were
  // in scope. This is what says the name came from Rust.
  assert!(
    outcome.asked(|q| q.offered("add_two_numbers").is_some()),
    "the oracle was never asked for the function this program calls:\n{}",
    outcome.rendered_log()
  );
}

/// Ensures a program can import a Rust trait: the oracle resolves and offers it, and the import
/// compiles even when unused. This is the first step toward a Vale struct implementing a Rust trait.
#[test]
fn imports_a_rust_trait() {
  let outcome = run_case(&IMPORTS_A_RUST_TRAIT, callees_in_main);

  outcome.check(&IMPORTS_A_RUST_TRAIT).expect("the case declares it compiles");

  // Vacuity: the program would compile just as happily if `Callback` were never resolved. This is
  // what says the trait actually reached Vale through the oracle.
  assert!(
    outcome.asked(|q| q.offered("Callback").is_some()),
    "the oracle never offered the imported trait:\n{}",
    outcome.rendered_log()
  );
}

/// Ensures a Vale struct can implement an imported Rust trait: the impl resolves `on_call` against
/// the trait's projected abstract method, and the method is callable through a `&Callback` reference.
/// The program compiles only if the interface carries `on_call` and the impl's edge exists.
#[test]
fn a_struct_implements_a_rust_trait() {
  let outcome = run_case(&A_STRUCT_IMPLEMENTS_A_RUST_TRAIT, callees_in_main);

  outcome.check(&A_STRUCT_IMPLEMENTS_A_RUST_TRAIT).expect("the case declares it compiles");

  // Vacuity: the trait must have reached Vale through the oracle for the impl to mean anything.
  assert!(
    outcome.asked(|q| q.offered("Callback").is_some()),
    "the oracle never offered the implemented trait:\n{}",
    outcome.rendered_log()
  );
}

/// Ensures an `impl` of a Rust trait that provides no override for the trait's method is rejected —
/// the projected abstract `on_call` is enforced, so an impl missing it fails to compile. This guards
/// that the trait's method projection is real: without it, `impl Callback for MyCb` would compile
/// vacuously.
#[test]
fn a_trait_impl_missing_its_override_is_rejected() {
  let outcome = run_case(&A_TRAIT_IMPL_MISSING_ITS_OVERRIDE_IS_REJECTED, callees_in_main);

  assert!(
    outcome.check(&A_TRAIT_IMPL_MISSING_ITS_OVERRIDE_IS_REJECTED).is_none(),
    "an impl missing its trait-method override was accepted:\n{}",
    outcome.rendered_log()
  );
}

/// Milestone (reverse direction): rustc's collector monomorphizes a generic Rust fn with a Valen
/// struct as its type argument (`run_callback::<MyCb>`) and, walking its body, discovers the Valen
/// trait-impl callback `<MyCb as Callback>::on_call`. Collector-driven (no run): asserts rustc drove
/// `__vale_main`, requested `run_callback`, discovered `on_call`, resolved everything, and codegen'd.
#[test]
fn rustc_discovers_a_valen_trait_impl_callback() {
  let run = run_case_rustc_driven_full(&RUST_CALLS_A_VALEN_TRAIT_IMPL_CALLBACK);
  let firings = &run.firings;
  assert!(
    firings.iter().any(|f| f.contains("__vale_main")),
    "per_instance_mir never fired on __vale_main; firings: {firings:?}"
  );
  // `run_callback::<MyCb>` must actually resolve — MyCb, a local Valen struct, converted to a rustc
  // type argument and the generic monomorphization reified — not decline as unconvertible.
  assert!(
    !firings.iter().any(|f| f.contains("ARGS-UNCONVERTIBLE") || f.contains("UNRESOLVED")),
    "run_callback::<MyCb> did not resolve (unconvertible/unresolved); firings: {firings:?}"
  );
  assert!(
    firings.iter().any(|f| f.contains("run_callback") && f.contains("=>") && !f.contains("UNCONVERTIBLE")),
    "run_callback::<MyCb> was not reified for rustc; firings: {firings:?}"
  );
  assert_eq!(
    run.rustc_exit, 0,
    "rustc did not complete codegen (exit {}); firings: {firings:?}",
    run.rustc_exit
  );
}

/// Milestone (reverse direction, tier 2): **Rust owns the call**, and a Valen method runs because Rust
/// called it. `run_callback::<MyCb>(&mmlcb)` is rustc's own generic fn; its `c.on_call()` dispatches
/// statically to `<MyCb as Callback>::on_call`, whose body Valen emits under rustc's mangled symbol
/// (single-symbol). The linked bin runs and returns 7 — `rustc_discovers_...` above only proved rustc
/// *reached* `on_call`; this proves the Valen body actually runs when Rust invokes it.
#[test]
fn rust_calls_back_a_valen_callback_returns_seven() {
  let run = run_case_rustc_driven_and_run(&RUST_CALLS_A_VALEN_TRAIT_IMPL_CALLBACK);
  assert_eq!(
    run.process_exit,
    Some(7),
    "the driven callback bin did not exit 7 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Slice 6 (reverse direction): a scalar argument crosses Rust->Valen. `run_adder::<MyAdder>(&a, 35)`
/// hands the i32 `35` inbound to Valen's `add`, which returns it; the linked bin exits 35. The
/// `&self`-only callback above proved static dispatch; this proves an inbound *value* arrives intact.
#[test]
fn a_valen_callback_takes_a_scalar_arg() {
  let run = run_case_rustc_driven_and_run(&A_VALEN_CALLBACK_TAKES_A_SCALAR_ARG);
  assert_eq!(
    run.process_exit,
    Some(35),
    "the driven scalar-arg callback bin did not exit 35 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Slice 7 (reverse direction): a Rust borrow crosses inbound and the callback calls back out to Rust.
/// `run_ticker::<MyTicker>()` makes a `Counter` and hands `&Counter` to Valen's `on_tick`, which
/// returns `w.peek()` — an outbound Rust call on the received borrow. The linked bin exits 5.
#[test]
fn a_valen_callback_receives_a_rust_borrow() {
  let run = run_case_rustc_driven_and_run(&A_VALEN_CALLBACK_RECEIVES_A_RUST_BORROW);
  assert_eq!(
    run.process_exit,
    Some(5),
    "the driven borrow-arg callback bin did not exit 5 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Slice 8 (reverse direction): a Rust struct crosses inbound **by value**. `run_summer::<MySummer>()`
/// makes a `Small { a: 3, b: 6 }` and hands it inbound by value to Valen's `on_sum`, which returns
/// `s.sum()` (3 + 6). The linked bin exits 9 — a small aggregate reassembled from its two registers.
#[test]
fn a_valen_callback_receives_a_rust_struct_by_value() {
  let run = run_case_rustc_driven_and_run(&A_VALEN_CALLBACK_RECEIVES_A_RUST_STRUCT_BY_VALUE);
  assert_eq!(
    run.process_exit,
    Some(9),
    "the driven byval-struct callback bin did not exit 9 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Slice 8c (forward direction): a `Pair` **return** — Vale calls `Small2.new(3,6)` (a Rust assoc fn
/// returning `{i32,i32}` by value), binds it, and reads `s.sum()`. The struct returns in two registers
/// and is reassembled Vale-side. Exits 9.
#[test]
fn vale_receives_a_rust_pair_return() {
  let run = run_case_rustc_driven_and_run(&VALE_RECEIVES_A_RUST_PAIR_RETURN);
  assert_eq!(
    run.process_exit,
    Some(9),
    "the driven pair-return bin did not exit 9 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Slice 8b (forward direction): a `Pair` **argument** — Vale passes a small `{i32,i32}` struct by
/// value into a Rust free function (`add_small(s)`). The struct crosses outbound in two registers.
/// Exits 9.
#[test]
fn vale_passes_a_rust_pair_arg() {
  let run = run_case_rustc_driven_and_run(&VALE_PASSES_A_RUST_PAIR_ARG);
  assert_eq!(
    run.process_exit,
    Some(9),
    "the driven pair-arg bin did not exit 9 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Slice 8d (reverse direction): a callback **returns** a Rust struct by value. Valen's `make` returns
/// `Small.new(3,6)` and Rust's `run_maker::<MyMaker>()` reads `c.make().sum()`. The struct crosses
/// Valen -> Rust in two registers (an inbound Pair return). Exits 9.
#[test]
fn a_valen_callback_returns_a_rust_struct_by_value() {
  let run = run_case_rustc_driven_and_run(&A_VALEN_CALLBACK_RETURNS_A_RUST_STRUCT_BY_VALUE);
  assert_eq!(
    run.process_exit,
    Some(9),
    "the driven retpair callback bin did not exit 9 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Slice 9 (the capstone): **Rust owns a loop** calling the Valen callback N times. `main_loop::<MyCb>`
/// loops `i = 0..5` calling `c.on_tick(i)` (which returns `i`) and sums the returns; the linked bin
/// exits 10 (0 + 1 + 2 + 3 + 4). Proves the callback survives repeated re-entry with a fresh scalar
/// each iteration — the NobiliaV `main_loop` shape where Rust drives the loop and calls Valen per frame.
#[test]
fn rust_owns_a_loop_calling_the_callback() {
  let run = run_case_rustc_driven_and_run(&RUST_OWNS_A_LOOP_CALLING_THE_CALLBACK);
  assert_eq!(
    run.process_exit,
    Some(10),
    "the driven main-loop bin did not exit 10 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Laziness, proven positively: importing three representable free functions and calling one queries
/// `fn_sig` for the called function and for neither uncalled one. This is the whole point of the slice
/// — importing a type with a hundred methods must not pay `fn_sig` for the ones never called.
#[test]
fn lazy_synthesis_only_queries_called_functions() {
  let outcome = run_case(&LAZY_SYNTHESIS_ONLY_QUERIES_CALLED_FUNCTIONS, callees_in_main);

  outcome.check(&LAZY_SYNTHESIS_ONLY_QUERIES_CALLED_FUNCTIONS);

  let add_two_numbers = offered(&outcome, "add_two_numbers");
  let seven = offered(&outcome, "seven");
  let is_positive = offered(&outcome, "is_positive");

  // The called function's signature is queried — it must be, to compile the call.
  assert!(
    outcome.asked(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == add_two_numbers)),
    "the called function's signature was never queried:\n{}",
    outcome.rendered_log()
  );
  // The two imported-but-uncalled functions are never queried — the laziness guarantee.
  assert!(
    !outcome.asked(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == seven)),
    "`seven` was imported but never called, yet its signature was queried:\n{}",
    outcome.rendered_log()
  );
  assert!(
    !outcome.asked(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == is_positive)),
    "`is_positive` was imported but never called, yet its signature was queried:\n{}",
    outcome.rendered_log()
  );
}

/// The common shape: the case compiles, and each named callee resolved to a **Rust-backed**
/// function. Rust-backedness is the load-bearing half — a Vale function of the same name would
/// satisfy "it compiled" just as well, and only the reserved `rust` package coordinate says where
/// resolution actually came from.
fn assert_rust_callees(case: &Case, expected: &[&str]) {
  let outcome = run_case(case, callees_in_main);
  let callees = outcome.check(case).expect("the case declares it compiles");
  for name in expected {
    assert!(
      callees.iter().any(|c| c.name == *name && c.rust_backed),
      "`{name}` did not resolve to a Rust callee: {callees:?}"
    );
  }
}

#[test]
fn calls_a_zero_arg_rust_function() {
  assert_rust_callees(&CALLS_A_ZERO_ARG_RUST_FUNCTION, &["seven"]);
}

#[test]
fn calls_a_rust_function_returning_unit() {
  assert_rust_callees(&CALLS_A_RUST_FUNCTION_RETURNING_UNIT, &["do_nothing"]);
}

#[test]
fn passes_and_returns_a_bool() {
  assert_rust_callees(&PASSES_AND_RETURNS_A_BOOL, &["is_positive", "to_int"]);
}

/// A Rust citizen in argument position of a free function — a different lowering path from return
/// position, and a different discovery path from a method.
#[test]
fn takes_a_rust_type_as_a_parameter() {
  let outcome = run_case(&TAKES_A_RUST_TYPE_AS_A_PARAMETER, callees_in_main);

  let callees =
    outcome.check(&TAKES_A_RUST_TYPE_AS_A_PARAMETER).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| {
      c.name == "value_of_counter" && c.rust_backed && c.params == vec!["rust-citizen".to_string()]
    }),
    "the free function did not take the Rust citizen as its parameter: {callees:?}"
  );
}

/// The same citizen identity on both sides of one signature.
#[test]
fn takes_and_returns_a_rust_type() {
  let outcome = run_case(&TAKES_AND_RETURNS_A_RUST_TYPE, callees_in_main);

  let callees =
    outcome.check(&TAKES_AND_RETURNS_A_RUST_TYPE).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| {
      c.name == "bump"
        && c.rust_backed
        && c.params == vec!["rust-citizen".to_string()]
        && c.ret == "rust-citizen"
    }),
    "`bump` did not both take and return the Rust citizen: {callees:?}"
  );
}

/// The mirror canary for the generic index mapping. Together with
/// `reads_a_generic_signature_structurally`, no single wrong mapping satisfies both.
#[test]
fn binds_the_second_generic_parameter() {
  let outcome = run_case(&BINDS_THE_SECOND_GENERIC_PARAMETER, callees_in_main);

  let callees =
    outcome.check(&BINDS_THE_SECOND_GENERIC_PARAMETER).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "pick_second" && c.rust_backed && c.ret == "int32"),
    "`pick_second<bool, int>` did not return int, so `B` bound to the wrong slot: {callees:?}"
  );
}

/// A floor, not a canary: `id<T>` passes under any index mapping, so it says only that
/// substitution happens at all.
#[test]
fn instantiates_a_generic_at_one_parameter() {
  assert_rust_callees(&INSTANTIATES_A_GENERIC_AT_ONE_PARAMETER, &["id"]);
}

/// A generic function whose parameter is the generic type applied to its own parameter.
///
/// Isolates the backward inference that a generic type's `drop` also needs, away from drop.
#[test]
fn calls_a_generic_function_taking_a_generic_type() {
  let outcome = run_case(&CALLS_A_GENERIC_FUNCTION_TAKING_A_GENERIC_TYPE, callees_in_main);

  let callees = outcome
    .check(&CALLS_A_GENERIC_FUNCTION_TAKING_A_GENERIC_TYPE)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| {
      c.name == "holder_ignore"
        && c.rust_backed
        && c.params == vec!["rust-citizen<int32>".to_string()]
    }),
    "the parameter did not resolve to `Holder<int>`: {callees:?}"
  );
}

/// A Rust citizen as a **generic argument**, rather than as a parameter or return type.
#[test]
fn instantiates_a_generic_at_a_rust_type() {
  let outcome = run_case(&INSTANTIATES_A_GENERIC_AT_A_RUST_TYPE, callees_in_main);

  let callees =
    outcome.check(&INSTANTIATES_A_GENERIC_AT_A_RUST_TYPE).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "id" && c.rust_backed && c.ret == "rust-citizen"),
    "`id<Counter>` did not return the Rust citizen: {callees:?}"
  );
}

/// An associated function with no receiver — the `Vec::new` shape — is an ordinary declaration
/// that happens to take no parameters.
#[test]
fn calls_an_associated_function_with_no_receiver() {
  let outcome = run_case(&CALLS_AN_ASSOCIATED_FUNCTION_WITH_NO_RECEIVER, callees_in_main);

  let callees = outcome
    .check(&CALLS_AN_ASSOCIATED_FUNCTION_WITH_NO_RECEIVER)
    .expect("the case declares it compiles");
  assert!(
    callees
      .iter()
      .any(|c| c.name == "new" && c.rust_backed && c.params.is_empty() && c.ret == "rust-citizen"),
    "the associated function did not resolve as a no-parameter function returning the \
         citizen: {callees:?}"
  );
}

/// An associated function whose impl **fixes** one of the type's parameters (`impl<T> Boxed<T, Fixed>`,
/// the `Vec::new` shape), called with the generic on the method: `Boxed.new<int>()`. `new` ranges over
/// one generic, so naming one type argument does not trip the resolver's container-vs-function rune
/// subtraction (the `1 - 2` underflow the over-specified `Boxed<int, Fixed>.new()` form would hit).
#[test]
fn calls_an_assoc_fn_with_a_fixed_impl_param_method_generic() {
  assert_rust_callees(&CALLS_AN_ASSOC_FN_FIXED_IMPL_PARAM_METHOD_GENERIC, &["new"]);
}

/// The same fixed-impl-param associated function, called with the generic on the type:
/// `Boxed<int>.new()` — the `Vec<int>.with_capacity()` form.
#[test]
fn calls_an_assoc_fn_with_a_fixed_impl_param_type_generic() {
  assert_rust_callees(&CALLS_AN_ASSOC_FN_FIXED_IMPL_PARAM_TYPE_GENERIC, &["new"]);
}

/// A Rust `usize` imported as the Vale `usize` primitive (`Vec::len`'s shape): `some_size() -> usize`
/// produces one, `consume_usize(usize) -> i32` takes it. `usize` used to decline as `UnsignedInteger`.
#[test]
fn imports_usize_as_a_primitive() {
  assert_rust_callees(&CALLS_A_FUNCTION_RETURNING_USIZE, &["some_size", "consume_usize"]);
}

/// A Rust enum imports as an opaque sealed interface, and its inherent method resolves — the opaque
/// tier's payoff (a method without variants, the `Option::unwrap` shape).
#[test]
fn calls_a_method_on_an_imported_enum() {
  assert_rust_callees(&CALLS_A_METHOD_ON_AN_IMPORTED_ENUM, &["level"]);
}

/// An imported enum bound to a local and never consumed gets a scope-end drop — an interface's drop,
/// synthesized like a struct's.
#[test]
fn an_imported_enum_bound_to_a_local_gets_a_scope_end_drop() {
  let outcome = run_case(&AN_IMPORTED_ENUM_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP, callees_in_main);
  let callees = outcome
    .check(&AN_IMPORTED_ENUM_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "drop" && c.rust_backed),
    "the bound enum got no scope-end drop: {callees:?}"
  );
}

/// The capstone: real `std::vec::Vec` + `std::alloc::Global` from the actual `alloc` crate,
/// `Vec.new<int>()` bound to a local with a scope-end drop, typechecked against live rustc.
#[test]
fn imports_real_vec_and_constructs_it() {
  assert_rust_callees(&IMPORTS_REAL_VEC_AND_CONSTRUCTS_IT, &["new"]);
}

/// Real `Vec` `&mut self` method: `v.push(42)`.
#[test]
fn calls_push_on_a_real_vec() {
  assert_rust_callees(&CALLS_PUSH_ON_A_REAL_VEC, &["push"]);
}

/// Real `Vec` `&self` method returning `usize`: `v.len()`.
#[test]
fn calls_len_on_a_real_vec() {
  assert_rust_callees(&CALLS_LEN_ON_A_REAL_VEC, &["len"]);
}

/// The capstone: real `Vec::pop() -> Option<int>` then `Option::unwrap() -> int` — a struct method
/// returning a real `std` enum, whose inherent method hands back the element.
#[test]
fn calls_pop_then_unwrap_on_a_real_vec() {
  assert_rust_callees(&CALLS_POP_THEN_UNWRAP_ON_A_REAL_VEC, &["pop", "unwrap"]);
}

/// A struct wrapping a `HashMap`, used through methods: build a `Domino`, add a `Glyph` via a `&mut self`
/// method, read one back via a `&self` method returning a **borrow** (`&Glyph`) bound to a local, then
/// read the glyph's field through an accessor. The borrow-return bound to a local (`d_ref`) is the new
/// mechanic — earlier cases proved borrow *receivers*, never a borrow *return* of a citizen held in a
/// local. `location` returning `int32` is main's observable (the stored glyph's location, 7).
#[test]
fn a_struct_wrapping_a_hashmap_is_used_through_methods() {
  let outcome = run_case(&A_STRUCT_WRAPPING_A_HASHMAP_IS_USED_THROUGH_METHODS, callees_in_main);
  let callees = outcome
    .check(&A_STRUCT_WRAPPING_A_HASHMAP_IS_USED_THROUGH_METHODS)
    .expect("the case declares it compiles");
  for name in ["add_glyph", "get_glyph", "location"] {
    assert!(
      callees.iter().any(|c| c.name == name && c.rust_backed),
      "`{name}` did not resolve to a Rust callee: {callees:?}"
    );
  }
  assert!(
    callees.iter().any(|c| c.name == "location" && c.rust_backed && c.ret == "int32"),
    "`location` did not return int, so main's observable is wrong: {callees:?}"
  );
}

/// An imported `fn nudge(a: &mut Counter, b: &Counter)` becomes `func nudge<g0', g1'>(a &Counter in g0,
/// b &Counter in g1) mut(g0)`; calling `nudge(&s, &s)` aliases `s` into the mutated group `g0` and the
/// disjoint group `g1`, so the borrow checker must reject it.
#[test]
fn a_mut_borrow_aliasing_a_shared_borrow_of_one_local_is_rejected() {
  // Mirroring Rust `&mut` into a `mut(g)` group makes a callee's disjoint-group assumption checkable:
  // the same local into a mutated group and a distinct group is an aliasing violation.
  run_case(&A_MUT_BORROW_ALIASING_A_SHARED_BORROW_IS_REJECTED, callees_in_main)
    .check(&A_MUT_BORROW_ALIASING_A_SHARED_BORROW_IS_REJECTED);
}

/// The disjoint counterpart compiles: distinct locals into `nudge`'s mutated and shared groups do not
/// alias, so the group mirroring must not reject them.
#[test]
fn a_mut_borrow_and_a_shared_borrow_of_distinct_locals_compiles() {
  // Guards against over-rejection: emitting groups must flag only aliasing calls, not every two-borrow one.
  run_case(&A_MUT_BORROW_AND_A_SHARED_BORROW_OF_DISTINCT_LOCALS_IS_CLEAN, callees_in_main)
    .check(&A_MUT_BORROW_AND_A_SHARED_BORROW_OF_DISTINCT_LOCALS_IS_CLEAN);
}

/// A Rust signature sharing one lifetime across two parameters is declined, not imported with a guess.
/// Faithfully mirroring it needs lifetime decoding Vale doesn't do yet, so calling it is a compile error.
#[test]
fn calling_a_shared_parameter_lifetime_import_is_a_compile_error() {
  // Shared-across-parameters lifetimes are rejected rather than assumed disjoint (what per-parameter
  // groups would assume) — the one case where we refuse to guess until real decoding lands.
  run_case(&CALLING_A_SHARED_PARAMETER_LIFETIME_IMPORT_IS_A_COMPILE_ERROR, callees_in_main)
    .check(&CALLING_A_SHARED_PARAMETER_LIFETIME_IMPORT_IS_A_COMPILE_ERROR);
}

/// The pass **past typing**: run the instantiator (monomorphizer) on an interop program, no backend.
/// The simplest shape first — a call to a Rust free function — so a failure here isolates "the
/// instantiator cannot handle a synthesized extern at all" from anything the domino case adds.
/// `translate` panics if the typechecked program cannot be monomorphized, so reaching the assert means
/// it survived. The queue filter (`translate_prototype`, `is_rust_backed`) diverts the Rust callee out
/// of the body-translation path and records it as an instantiation request, so the drain never asks
/// `translate_function_callsite` for a body the extern doesn't have.
#[test]
fn a_rust_free_function_call_reaches_the_instantiator() {
  let outcome = run_case_instantiated(&CALLS_A_RUST_FREE_FUNCTION, callees_in_main);
  outcome.check(&CALLS_A_RUST_FREE_FUNCTION);
  let summary = outcome.expect_instantiated();
  assert!(summary.functions > 0, "nothing monomorphized: {summary:?}");
}

/// The domino case pushed past typing into the instantiator — opaque struct wrapping a `HashMap`,
/// `&mut self`/`&self` methods, and a borrow return (`&Glyph`) bound to a local. Proves those
/// synthesized denizens monomorphize, not merely typecheck.
#[test]
fn the_hashmap_wrapping_struct_reaches_the_instantiator() {
  let outcome =
    run_case_instantiated(&A_STRUCT_WRAPPING_A_HASHMAP_IS_USED_THROUGH_METHODS, callees_in_main);
  outcome.check(&A_STRUCT_WRAPPING_A_HASHMAP_IS_USED_THROUGH_METHODS);
  let summary = outcome.expect_instantiated();
  assert!(summary.functions > 0, "nothing monomorphized: {summary:?}");
}

/// Milestone M (free-function case): rustc's mono collector drives *our monomorphizer* end to end.
/// Compiling the stub to completion with the `per_instance_mir` override fires our provider on the
/// `__vale_main` root; the provider seeds that export, drains the instantiator, collects the Rust
/// functions `main` transitively calls, resolves each to a rustc `DefId`, and hands rustc a
/// `ReifyFnPointer` body naming them (which is what queues them for codegen). Reaching here proves
/// the whole loop: rustc drives us, the drive finds the Rust leaf `add_two_numbers`, and it resolves
/// back to a real rustc item (the firing records `<path> => <resolved def path>`).
#[test]
fn rustc_collector_drives_our_monomorphizer() {
  let firings = run_case_rustc_driven(&CALLS_A_RUST_FREE_FUNCTION);
  assert!(
    firings.iter().any(|f| f.contains("__vale_main")),
    "per_instance_mir never fired on __vale_main; firings: {firings:?}"
  );
  assert!(
    firings.iter().any(|f| f.contains("add_two_numbers[] =>")),
    "the drive did not collect + resolve a Rust request for add_two_numbers; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "a Rust request failed to resolve to a DefId; firings: {firings:?}"
  );
}

/// Milestone M, generic callee: the same driven loop, but `main` calls a *generic* Rust function
/// `id<int>(9)`. The request now carries a type argument, so the provider must convert the Vale
/// templata `int` to the rustc `Ty` `i32` and build the callee's `GenericArgs` before reifying
/// `id::<i32>`. Proves the templata → rustc-`Ty` bridge for a primitive type argument.
#[test]
fn rustc_collector_drives_a_generic_rust_callee() {
  let firings = run_case_rustc_driven(&INSTANTIATES_A_GENERIC_AT_ONE_PARAMETER);
  assert!(
    firings.iter().any(|f| f.contains("mycrate.id[i32] =>")),
    "id<int> did not convert its type arg to i32 and resolve; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "the generic id<int> request failed to resolve; firings: {firings:?}"
  );
}

/// Milestone M, generic callee at a Rust type: `main` calls `id<Counter>(make_counter())`, so the
/// generic function's type argument is itself an imported Rust type rather than a primitive. The
/// provider must lower the Vale `Counter` kind to the rustc `Adt` type and build `id::<Counter>`'s
/// args. Proves the templata → rustc-`Ty` bridge for a Rust-backed (non-generic) type argument.
#[test]
fn rustc_collector_drives_a_generic_at_a_rust_type() {
  let firings = run_case_rustc_driven(&INSTANTIATES_A_GENERIC_AT_A_RUST_TYPE);
  assert!(
    firings.iter().any(|f| f.contains("mycrate.id[") && f.contains("Counter")),
    "id<Counter> did not convert its Rust-type arg and resolve; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "a Rust request failed to resolve; firings: {firings:?}"
  );
}

/// Milestone M, scope-end drop: `s = make_shade(); return 4;` binds an imported enum to a local and
/// never consumes it, so it takes a synthesized scope-end drop. An imported type has no Rust `Drop`
/// to resolve to, so the provider maps the drop to a generic `__vale_drop<T>` shim (arch §15.7).
#[test]
fn rustc_collector_drives_a_scope_end_drop() {
  let firings = run_case_rustc_driven(&AN_IMPORTED_ENUM_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP);
  assert!(
    firings.iter().any(|f| f.contains("drop => __vale_drop") && f.contains("(drop shim)")),
    "the scope-end drop did not map to the __vale_drop shim; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "a Rust request failed to resolve; firings: {firings:?}"
  );
}

/// Milestone M, generic associated function: `b = Boxed<int>.new()`. `Boxed<T, Fixed>::new` is a
/// generic assoc fn whose impl pins the second type param to `Fixed`, so the callee's args must be
/// reconstructed from the owner's type args plus the impl-pinned param. The bound-and-dropped local
/// also drops the generic `Boxed<int, Fixed>`.
#[test]
fn rustc_collector_drives_a_generic_assoc_function() {
  let firings = run_case_rustc_driven(&A_GENERIC_ASSOC_RESULT_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP);
  assert!(
    firings.iter().any(|f| f.contains("new =>") && f.contains("(assoc)")),
    "Boxed<int>::new did not resolve as a generic assoc fn; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "a generic assoc fn request failed to resolve; firings: {firings:?}"
  );
}

/// Milestone M, real `std` `Vec`: `v = Vec.new<int>()`. `Vec::new` is in `impl<T> Vec<T, Global>`,
/// and `Global` is a *default* type param, so the dropped `Vec<int, Global>` carries an arg Vale never
/// names — the provider must fill the defaulted allocator param when it cannot from the Vale args.
#[test]
fn rustc_collector_drives_real_vec_new() {
  let firings = run_case_rustc_driven(&IMPORTS_REAL_VEC_AND_CONSTRUCTS_IT);
  assert!(
    firings.iter().any(|f| f.contains("vec.new =>") && f.contains("(assoc)")),
    "Vec::new did not resolve as a generic assoc fn; firings: {firings:?}"
  );
  assert!(
    firings.iter().any(|f| f.contains("__vale_drop")),
    "the Vec<int, Global> drop did not map to the shim; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "a real-Vec request failed to resolve; firings: {firings:?}"
  );
}

/// Milestone M, the composed domino case: opaque struct wrapping a `HashMap`, driven through
/// `Domino.new()` / `Glyph.new()` (associated functions), `&mut self` add, `&self` borrow-return get,
/// a field accessor, and scope-end drops. The end-to-end target of the driven path.
#[test]
fn rustc_collector_drives_the_domino_case() {
  let run = run_case_rustc_driven_full(&A_STRUCT_WRAPPING_A_HASHMAP_IS_USED_THROUGH_METHODS);
  let firings = &run.firings;
  // Every callee shape composed in one program resolves: associated functions, methods (incl.
  // `&mut self`/`&self`), a field accessor, and the scope-end drop shim.
  assert!(
    firings.iter().any(|f| f.contains("(assoc)"))
      && firings.iter().any(|f| f.contains("(method)"))
      && firings.iter().any(|f| f.contains("(drop shim)")),
    "the domino case did not exercise assoc/method/drop resolution; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "a Rust request in the domino case failed to resolve; firings: {firings:?}"
  );
  // rustc drove all the way through codegen without erroring on any reified leaf. This is the
  // stronger "the frontend half is real" signal: every `(DefId, GenericArgs)` we handed back was
  // valid enough for rustc to monomorphize and codegen. (It does not run the program — the real
  // Vale bodies await the backend's `fill_extra_modules`.)
  assert_eq!(
    run.rustc_exit, 0,
    "rustc did not complete codegen on the domino case (exit {}); firings: {firings:?}",
    run.rustc_exit
  );
}

/// Stage 1 (`#2a`) of the run-a-program path: the `fill_extra_modules` codegen hook fires. Installing
/// `set_fill_extra_modules_hook(consumer_fill_modules)` in the driven `config()` means that once rustc
/// reaches codegen (`Compilation::Continue`), it calls our handler on the same armed `DriverState`.
/// This proves the pipe from rustc's codegen into Vale — the seam the real emission (Stage 2) rides.
#[test]
fn rustc_codegen_fires_our_fill_extra_modules_hook() {
  let firings = run_case_rustc_driven(&CALLS_A_RUST_FREE_FUNCTION);
  assert!(
    firings.iter().any(|f| f.contains("consumer_fill_modules fired")),
    "the fill_extra_modules hook never fired at codegen time; firings: {firings:?}"
  );
}

/// Stage 2 (`#2b`) of the run-a-program path: the hook actually lowers the Vale program and emits its
/// bodies into rustc's borrowed module. The handler builds a finalized `HinputsI` via the ordinary
/// `translate_program`, lowers it through the existing `populate_metal_cache`, takes rustc's lent
/// `(context, module)` from the `ExtraModuleAllocator`, and calls `backend_compile_program_into` — the
/// first real exercise of the borrowed C++ backend path (its `LLVMVerifyModule` runs on the Vale IR in
/// rustc's module). rc 0 = emitted and verified. Still a lib crate — not linked or run (Stage 3).
#[test]
fn rustc_codegen_emits_vale_bodies_into_borrowed_module() {
  let run = run_case_rustc_driven_emitting(&CALLS_A_RUST_FREE_FUNCTION);
  assert!(
    run.firings.iter().any(|f| f.contains("consumer_fill_modules emitted rc=0")),
    "the backend did not emit + verify Vale IR into rustc's borrowed module; firings: {:?}",
    run.firings
  );
  assert_eq!(
    run.rustc_exit, 0,
    "rustc did not complete codegen after the borrowed emit (exit {}); firings: {:?}",
    run.rustc_exit, run.firings
  );
}

/// Stage 3 (tier 2): the whole round trip. Drive rustc to a linked bin, emit the Vale bodies into it,
/// run the executable, and assert it exits with what `main` returns. `seven()` (`return seven();`) is
/// the simplest real Rust call — zero args, scalar `i32`, so Rust ABI == C ABI. This is the first case
/// that observes a *value*, not just that emission verified.
#[test]
fn rustc_driven_bin_links_and_returns_seven() {
  let run = run_case_rustc_driven_and_run(&CALLS_A_ZERO_ARG_RUST_FUNCTION);
  assert_eq!(
    run.process_exit,
    Some(7),
    "the driven bin did not exit 7 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Stage 3, the final goal: a real **two-argument** Rust free function. `add_two_numbers(20, 22)`
/// passes two scalar `i32`s across the boundary to rustc's own `add_two_numbers`, linked and run,
/// asserting the process exits 42. This is the canonical driven case (the Stage-1/2 tests emit it),
/// now taken all the way to a running binary — a Vale program calling real Rust, end to end.
#[test]
fn rustc_driven_bin_links_and_returns_from_add_two_numbers() {
  let run = run_case_rustc_driven_and_run(&CALLS_A_RUST_FREE_FUNCTION);
  assert_eq!(
    run.process_exit,
    Some(42),
    "the driven bin did not exit 42 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// The goal (tier 2): the full domino case, `d = Domino.new(); d.add_glyph(Glyph.new(7)); d_ref =
/// d.get_glyph(7); return d_ref.location();`, linked and run, returns 7. It protects every ABI mode at
/// once, each sourced from rustc (`tcx.layout_of` for sizes, `tcx.fn_abi_of_instance` for conventions):
/// - `Indirect`: the 48-byte `Domino` returned via `sret`, with the aarch64 `sret` attribute.
/// - `DirectPtr`: the `&mut self`/`&self` receivers and the `&Glyph` return, as pointers.
/// - `DirectInt`: `Glyph` and `i32` in registers.
/// - `Ignore`: the scope-end `drop_in_place` of `d`, its owned value spilled to a pointer.
#[test]
fn rustc_driven_bin_domino_returns_seven() {
  let run = run_case_rustc_driven_and_run(&A_STRUCT_WRAPPING_A_HASHMAP_IS_USED_THROUGH_METHODS);
  assert_eq!(
    run.process_exit,
    Some(7),
    "the driven domino bin did not exit 7 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Passes a large struct BY VALUE into a Rust free function: `domino_size(d)` moves a 48-byte `Domino`,
/// which rustc classifies `PassMode::Indirect`, so it must cross as LLVM `byval` (a pointer to a
/// caller-owned copy, ownership moved to the callee). One glyph is inserted before the move, so it
/// returns 1. This is the argument mirror of the sret return the domino case already exercises.
#[test]
fn rustc_driven_bin_domino_byval_arg_returns_one() {
  let run = run_case_rustc_driven_and_run(&DOMINO_BY_VALUE_ARG);
  assert_eq!(
    run.process_exit,
    Some(1),
    "the driven byval-arg bin did not exit 1 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Pins the byval attribute index when the byval argument sits BEHIND an sret return.
/// `add_and_return(^d, 7)` moves a `Domino` in by value and returns a `Domino` by value (sret), so the
/// byval argument is physical parameter 1 (the sret out-pointer is 0); a byval attribute placed by
/// logical argument index would land on the sret pointer. Returns 7.
#[test]
fn rustc_driven_bin_domino_byval_arg_with_sret_returns_seven() {
  let run = run_case_rustc_driven_and_run(&DOMINO_BYVAL_ARG_WITH_SRET_RETURN);
  assert_eq!(
    run.process_exit,
    Some(7),
    "the driven byval-arg-with-sret bin did not exit 7 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// A Rust function returns an 8-byte struct by value — rustc `PassMode::Cast` crossing as a single
/// `i64` (count 1). Vale reassembles the `Small8` from the `i64` and reads field `a` (=6). PieceId's
/// return shape.
#[test]
fn rustc_driven_bin_small8_cast_return_returns_six() {
  let run = run_case_rustc_driven_and_run(&SMALL8_CAST_RETURN);
  assert_eq!(
    run.process_exit,
    Some(6),
    "the driven small8 cast-return bin did not exit 6 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// A Vale program passes an 8-byte struct by value into a Rust function — rustc `PassMode::Cast` as a
/// single `i64`, alongside a scalar arg. `small_plus(^s, 4)` returns `s.a + 4` = 10. The Cast argument
/// direction (`pack_id`'s shape).
#[test]
fn rustc_driven_bin_small8_cast_arg_returns_ten() {
  let run = run_case_rustc_driven_and_run(&SMALL8_CAST_ARG);
  assert_eq!(
    run.process_exit,
    Some(10),
    "the driven small8 cast-arg bin did not exit 10 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Regression guard for the driven harness's diagnostics, not an interop feature. When a driven Vale
/// program fails to typecheck, the harness must surface that. Before it did, a failed typecheck left
/// `hinputs` None, which read downstream as an empty `__vale_main -> []` firing log plus an undefined
/// `__vale_main` at link, so a broken program looked like a mysterious ABI or instantiation failure.
/// The harness now panics with the typing diagnostic instead. `expect` is inert on the driven path,
/// which asserts through this panic.
#[test]
#[should_panic(expected = "failed to typecheck")]
fn driven_harness_surfaces_a_typing_failure() {
  let broken = Case {
    fixture: "fixtures",
    name: "driven-typecheck-failure",
    vale: "exported func main() int { return true; }",
    expect: Expect::FailsToCompile("main returns a bool where an int is required"),
  };
  run_case_rustc_driven_and_run(&broken);
}

/// Ladder rung 1 (tier 2): the first case that runs a non-scalar aggregate across the boundary,
/// `(make_counter()).get()`, which returns and consumes `Counter{i32}` by value, returns 7. Two things
/// must hold: the struct-layout map sizes `translateType(Counter)` to a real `[1 x i32]`, and the
/// extern-abi map crosses `Counter` as `DirectInt(32)`, reinterpreting the value and its register integer.
#[test]
fn rustc_driven_bin_method_returns_seven() {
  let run = run_case_rustc_driven_and_run(&CALLS_A_METHOD_ON_A_RUST_TYPE);
  assert_eq!(
    run.process_exit,
    Some(7),
    "the driven bin did not exit 7 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Ladder rung 2 (tier 2): `c = make_counter(); return c.peek();` → 7, linked and run. Adds two ABI
/// modes over rung 1: the `&self` borrow receiver crosses as a real pointer (`DirectPtr`, a pointer-
/// scalar layout, not reinterpreted as an integer), and the scope-end drop of `c` has a unit return
/// (`Ignore`). Sizing of `Counter` is shared with rung 1.
#[test]
fn rustc_driven_bin_borrow_self_method_returns_seven() {
  let run = run_case_rustc_driven_and_run(&CALLS_A_BORROW_SELF_METHOD_ON_A_LOCAL);
  assert_eq!(
    run.process_exit,
    Some(7),
    "the driven bin did not exit 7 (rustc_exit={}, process_exit={:?}); firings: {:?}",
    run.rustc_exit, run.process_exit, run.firings
  );
}

/// Milestone M, borrow-receiver method: `c = make_counter(); return c.peek();`. `peek(&self)` takes a
/// borrow receiver, so the request's first parameter is a borrow-wrapped `Counter` rather than a bare
/// one; the provider must peel the reference to find the owning type. `c` also takes a scope-end drop.
#[test]
fn rustc_collector_drives_a_borrow_receiver_method() {
  let firings = run_case_rustc_driven(&CALLS_A_BORROW_SELF_METHOD_ON_A_LOCAL);
  assert!(
    firings.iter().any(|f| f.contains("peek =>") && f.contains("(method)")),
    "peek(&self) did not resolve through its borrow receiver; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "a Rust request failed to resolve; firings: {firings:?}"
  );
}

/// Milestone M, method callee: `main` calls a Rust method `(make_counter()).get()`. A method is not
/// a crate-qualified free function — `get` lives in `Counter`'s inherent impl — so the provider must
/// resolve it through the receiver type rather than a module path. By-value `get` consumes its
/// receiver, so there is no scope-end drop; this isolates method resolution from drop synthesis.
#[test]
fn rustc_collector_drives_a_method_callee() {
  let firings = run_case_rustc_driven(&CALLS_A_METHOD_ON_A_RUST_TYPE);
  assert!(
    firings.iter().any(|f| f.contains("get =>") && f.contains("(method)")),
    "the method call get() did not resolve through the receiver type; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "a Rust request failed to resolve; firings: {firings:?}"
  );
}

/// Milestone M, multi-parameter generic: `main` calls `pick<int, bool>(...)`, so the callee has two
/// type args and the provider must fill them in declaration order (`A = i32`, `B = bool`). `pick` is
/// the ordering canary — a swap would produce `[bool, i32]` and fail here.
#[test]
fn rustc_collector_drives_a_multi_param_generic() {
  let firings = run_case_rustc_driven(&READS_A_GENERIC_SIGNATURE_STRUCTURALLY);
  assert!(
    firings.iter().any(|f| f.contains("mycrate.pick[i32, bool] =>")),
    "pick<int, bool> did not convert both type args in order; firings: {firings:?}"
  );
  assert!(
    !firings.iter().any(|f| f.contains("UNRESOLVED")),
    "a Rust request failed to resolve; firings: {firings:?}"
  );
}

/// A two-parameter generic value from an associated function, bound to a local and dropped at scope
/// end — the real `let v = Vec<int, Global>.new();` shape. The generated drop names no type argument, so
/// `T` must come from the value.
#[test]
fn a_generic_assoc_result_bound_to_a_local_gets_a_scope_end_drop() {
  let outcome =
    run_case(&A_GENERIC_ASSOC_RESULT_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP, callees_in_main);
  let callees = outcome
    .check(&A_GENERIC_ASSOC_RESULT_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "drop" && c.rust_backed),
    "the bound generic Rust value got no scope-end drop: {callees:?}"
  );
}

/// A method on a generic type whose signature names the type's own parameter (`into_value(self) -> T`).
/// `T` is inherited from `impl<T> Holder<T>`, not declared by the method — the case that used to decline
/// as `InheritedParameter` before the oracle reported parent-inclusive generic params for methods.
#[test]
fn calls_a_method_naming_the_types_generic() {
  let outcome = run_case(&CALLS_A_METHOD_NAMING_THE_TYPES_GENERIC, callees_in_main);
  outcome.check(&CALLS_A_METHOD_NAMING_THE_TYPES_GENERIC);
}

/// Method discovery is a list, not a lucky single — and it is lazy per method.
#[test]
fn calls_two_methods_on_one_type() {
  assert_rust_callees(&CALLS_TWO_METHODS_ON_ONE_TYPE, &["get", "doubled"]);

  // Method laziness: Counter has four methods (get, doubled, or_else, new); this program calls only
  // `get` and `doubled`. Those two are queried; `or_else` and `new` are imported but uncalled, so their
  // signatures are never asked for. This is the payoff for methods — a hundred-method type costs
  // `fn_sig` only for the methods you actually call.
  let outcome = run_case(&CALLS_TWO_METHODS_ON_ONE_TYPE, callees_in_main);
  for called in ["get", "doubled"] {
    let item = offered(&outcome, called);
    assert!(
      outcome.asked(|q| matches!(q, OracleQuery::FnSig { item: i, .. } if *i == item)),
      "called method `{called}` was never queried:\n{}",
      outcome.rendered_log()
    );
  }
  for uncalled in ["or_else", "new"] {
    let item = offered(&outcome, uncalled);
    assert!(
      !outcome.asked(|q| matches!(q, OracleQuery::FnSig { item: i, .. } if *i == item)),
      "uncalled method `{uncalled}` was queried, so per-method laziness is broken:\n{}",
      outcome.rendered_log()
    );
  }
}

/// A method carrying its own type parameter, on top of the container's.
#[test]
fn calls_a_generic_method() {
  let outcome = run_case(&CALLS_A_GENERIC_METHOD, callees_in_main);

  let callees = outcome.check(&CALLS_A_GENERIC_METHOD).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "or_else" && c.rust_backed && c.ret == "int32"),
    "the generic method did not resolve at its own type argument: {callees:?}"
  );
}

/// Excess type arguments do not resolve — three named against `pick<A, B>`'s two slots.
#[test]
fn wrong_generic_arity_does_not_resolve() {
  let outcome = run_case(&WRONG_GENERIC_ARITY_DOES_NOT_RESOLVE, callees_in_main);

  assert!(outcome.check(&WRONG_GENERIC_ARITY_DOES_NOT_RESOLVE).is_none());
}

/// A Vale function and a Rust function sharing a name do **not** collide — candidate collection is
/// plural, so the outcome is a designed error rather than the panic a *type*-name collision gives.
#[test]
fn a_vale_function_and_a_rust_function_with_the_same_name() {
  let outcome = run_case(&A_VALE_FUNCTION_AND_A_RUST_FUNCTION_WITH_THE_SAME_NAME, callees_in_main);

  assert!(outcome.check(&A_VALE_FUNCTION_AND_A_RUST_FUNCTION_WITH_THE_SAME_NAME).is_none());
}

/// The negative control for the case above. If the same program compiled with nothing importable,
/// the positive case would prove nothing about where resolution came from.
///
/// The case declares which `ICompileErrorT` arm it must fail with, so `check` is the whole test.
#[test]
fn an_empty_allowlist_makes_nothing_importable() {
  let outcome = run_case(&AN_EMPTY_ALLOWLIST_MAKES_NOTHING_IMPORTABLE, callees_in_main);

  assert!(outcome.check(&AN_EMPTY_ALLOWLIST_MAKES_NOTHING_IMPORTABLE).is_none());
}

/// A generic Rust function is read **structurally** — parameters intact, not collapsed to one
/// instantiation. This is the thing the previous design could not express at all, and the reason
/// the arc pivoted.
#[test]
fn reads_a_generic_signature_structurally() {
  let outcome = run_case(&READS_A_GENERIC_SIGNATURE_STRUCTURALLY, callees_in_main);

  // The strong half of this assertion is that the program compiled at all: it calls
  // `pick<int, bool>` and returns the result from `main() int`, so binding `A` to the wrong slot
  // yields `bool` where `int` belongs and fails to resolve. `id<T>(x: T) -> T` would pass under
  // either mapping and prove nothing.
  let callees =
    outcome.check(&READS_A_GENERIC_SIGNATURE_STRUCTURALLY).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "pick" && c.rust_backed && c.ret == "int32"),
    "the generic call did not resolve to a Rust callee returning int: {callees:?}"
  );

  let pick = offered(&outcome, "pick");
  match outcome.find_query(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == pick)) {
    Some(OracleQuery::FnSig { answer: Some(shape), .. }) => {
      assert_eq!(vec!["A".to_string(), "B".to_string()], shape.generic_params);
      assert_eq!(vec![SigPosition::Generic(0), SigPosition::Generic(1)], shape.params);
      assert_eq!(SigPosition::Generic(0), shape.ret);
    }
    other => panic!(
      "expected a structural signature for `pick`, got {other:?}:\n{}",
      outcome.rendered_log()
    ),
  }
}

/// A Rust type reaches Vale by inference from a signature — never by name — and its method lives in
/// the type's outer environment, resolved via the receiver when `v.get()` desugars to `get(v)`.
#[test]
fn calls_a_method_on_a_rust_type() {
  let outcome = run_case(&CALLS_A_METHOD_ON_A_RUST_TYPE, callees_in_main);

  let callees =
    outcome.check(&CALLS_A_METHOD_ON_A_RUST_TYPE).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "make_counter" && c.rust_backed && c.ret == "rust-citizen"),
    "no call returned a Rust citizen, so the type never reached Vale: {callees:?}"
  );
  assert!(
    callees.iter().any(|c| {
      c.name == "get" && c.rust_backed && c.params == vec!["rust-citizen".to_string()]
    }),
    "the method did not resolve as a function taking the receiver as parameter zero: {callees:?}"
  );

  assert!(
    outcome.asked(|q| matches!(q, OracleQuery::Methods { .. }) && q.offered("get").is_some()),
    "the method was never discovered from the Rust side:\n{}",
    outcome.rendered_log()
  );
}

/// A `&self` (borrow-receiver) method called on a local. A local read is a `BorrowRef`, so this only
/// resolves if a borrow receiver matches `&self` — the shape every real `Vec::len`/`push` takes.
#[test]
fn calls_a_borrow_self_method_on_a_local() {
  assert_rust_callees(&CALLS_A_BORROW_SELF_METHOD_ON_A_LOCAL, &["peek"]);
}

/// A Rust value bound to a local and never consumed needs a scope-end drop. `Compiler::drop`'s
/// `KindT::Struct` arm always resolves a destructor call, so the importer has to synthesize a
/// `drop` for every imported type — `Counter` has no `Drop` impl, and asking rustc for a method
/// named `drop` would answer `None`.
#[test]
fn a_rust_value_bound_to_a_local_gets_a_scope_end_drop() {
  let outcome = run_case(&A_RUST_VALUE_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP, callees_in_main);

  let callees = outcome
    .check(&A_RUST_VALUE_BOUND_TO_A_LOCAL_GETS_A_SCOPE_END_DROP)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "drop" && c.rust_backed),
    "the bound Rust value got no scope-end drop: {callees:?}"
  );
}

/// A generic Rust value bound to a local and never consumed needs a scope-end drop.
#[test]
fn a_generic_rust_type_gets_a_scope_end_drop() {
  let outcome = run_case(&A_GENERIC_RUST_TYPE_GETS_A_SCOPE_END_DROP, callees_in_main);

  let callees = outcome
    .check(&A_GENERIC_RUST_TYPE_GETS_A_SCOPE_END_DROP)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "drop" && c.rust_backed),
    "the bound generic Rust value got no scope-end drop: {callees:?}"
  );
}

/// Hand-written Vale naming a Rust type in a parameter and calling a method on it.
#[test]
fn vale_source_calls_a_method_on_a_named_rust_parameter() {
  let outcome = run_case(&VALE_SOURCE_CALLS_A_METHOD_ON_A_NAMED_RUST_PARAMETER, callees_in_main);

  let callees = outcome
    .check(&VALE_SOURCE_CALLS_A_METHOD_ON_A_NAMED_RUST_PARAMETER)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "value_of" && !c.rust_backed),
    "the Vale function taking a Rust-typed parameter did not resolve: {callees:?}"
  );
}

/// A value returned and immediately discarded still gets dropped — the temporary path.
///
/// Distinct from the case above: with no local to hang the drop on, the drop attaches to a
/// temporary. The failure is silent — the program compiles and returns the right number either way
/// — so the assertion has to be on the callee list rather than on the outcome.
#[test]
fn a_rust_value_returned_and_discarded_gets_dropped() {
  let outcome = run_case(&A_RUST_VALUE_RETURNED_AND_DISCARDED_GETS_DROPPED, callees_in_main);

  let callees = outcome
    .check(&A_RUST_VALUE_RETURNED_AND_DISCARDED_GETS_DROPPED)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "drop" && c.rust_backed),
    "the discarded Rust temporary got no drop: {callees:?}"
  );
}

/// Two types' methods coexist, each resolving to its own receiver.
///
/// `Counter::get` and `Gauge::get` share a name deliberately. Each lives in its own type's outer env,
/// so the risk is the importer pairing a method with the wrong receiver, which surfaces as a
/// resolution failure rather than a wrong answer.
#[test]
fn calls_methods_on_two_different_rust_types() {
  let outcome = run_case(&CALLS_METHODS_ON_TWO_DIFFERENT_RUST_TYPES, callees_in_main);

  let callees = outcome
    .check(&CALLS_METHODS_ON_TWO_DIFFERENT_RUST_TYPES)
    .expect("the case declares it compiles");

  // Both `get`s resolved, and both are Rust-backed — one call each, so a single shared
  // declaration serving both receivers would show up as one callee rather than two.
  let gets: Vec<_> = callees.iter().filter(|c| c.name == "get" && c.rust_backed).collect();
  assert_eq!(2, gets.len(), "expected one `get` per receiver type, got: {callees:?}");
}

/// Two Rust types imported in one compilation — the importer is a loop, not a single-item path.
#[test]
fn imports_two_rust_types_at_once() {
  let outcome = run_case(&IMPORTS_TWO_RUST_TYPES_AT_ONCE, callees_in_main);

  outcome.check(&IMPORTS_TWO_RUST_TYPES_AT_ONCE);

  // Vacuity: both types must have been walked, not just the one whose value is returned.
  assert!(
    outcome.asked(|q| q.offered("value_of_counter").is_some()),
    "`Counter`'s items were never offered:\n{}",
    outcome.rendered_log()
  );
  assert!(
    outcome.asked(|q| q.offered("gauge_reading").is_some()),
    "`Gauge`'s items were never offered:\n{}",
    outcome.rendered_log()
  );
}

/// A Rust citizen produced by one call and consumed by another, with a third in between.
///
/// A lowering that minted a fresh kind per signature would typecheck each call in isolation and
/// fail only here, where the same type has to be recognised across a call boundary twice.
#[test]
fn a_rust_type_flows_through_two_calls() {
  let outcome = run_case(&A_RUST_TYPE_FLOWS_THROUGH_TWO_CALLS, callees_in_main);

  let callees =
    outcome.check(&A_RUST_TYPE_FLOWS_THROUGH_TWO_CALLS).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "bump" && c.rust_backed),
    "the intermediate call is not in the chain: {callees:?}"
  );
}

/// An item in a nested module, named by a dotted path — the shape `Vec` needs.
///
/// A crate-root-only walk cannot see it at all, so this is the first case in the corpus that a
/// one-level walk fails. Every other case sits at a root, which is the degenerate path.
#[test]
fn imports_an_item_from_a_nested_module() {
  let outcome = run_case(&IMPORTS_AN_ITEM_FROM_A_NESTED_MODULE, callees_in_main);

  let callees =
    outcome.check(&IMPORTS_AN_ITEM_FROM_A_NESTED_MODULE).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "depth_reading" && c.rust_backed),
    "the nested function did not resolve as a Rust-backed callee: {callees:?}"
  );
}

/// A type in a nested module, plus its method — a different `DefKind` and therefore a different arm.
#[test]
fn imports_a_type_from_a_nested_module() {
  let outcome = run_case(&IMPORTS_A_TYPE_FROM_A_NESTED_MODULE, callees_in_main);

  let callees =
    outcome.check(&IMPORTS_A_TYPE_FROM_A_NESTED_MODULE).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "depth_of" && c.rust_backed),
    "the nested type's method did not resolve: {callees:?}"
  );
}

/// An item reached through a re-exported name — the shape `std::vec::Vec` actually has.
#[test]
fn imports_through_a_re_exported_item() {
  let outcome = run_case(&IMPORTS_THROUGH_A_RE_EXPORTED_ITEM, callees_in_main);

  let callees =
    outcome.check(&IMPORTS_THROUGH_A_RE_EXPORTED_ITEM).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "depth_reading" && c.rust_backed),
    "the re-exported function did not resolve: {callees:?}"
  );
}

/// Descending **through** a re-exported module, rather than landing on a re-exported item.
#[test]
fn imports_through_a_re_exported_module() {
  let outcome = run_case(&IMPORTS_THROUGH_A_RE_EXPORTED_MODULE, callees_in_main);

  let callees =
    outcome.check(&IMPORTS_THROUGH_A_RE_EXPORTED_MODULE).expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "depth_of" && c.rust_backed),
    "the method on a type behind a re-exported module did not resolve: {callees:?}"
  );
}

/// A re-export whose target lives in another crate, reached by a path through the re-exporting one.
#[test]
fn imports_through_a_cross_crate_re_exported_item() {
  let outcome = run_case(&IMPORTS_THROUGH_A_CROSS_CRATE_RE_EXPORTED_ITEM, callees_in_main);

  let callees = outcome
    .check(&IMPORTS_THROUGH_A_CROSS_CRATE_RE_EXPORTED_ITEM)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "gadget_value" && c.rust_backed),
    "the method on a cross-crate re-exported type did not resolve: {callees:?}"
  );
}

/// Descending through a re-exported **module** whose target is in another crate — `std::vec`'s form.
#[test]
fn imports_through_a_cross_crate_re_exported_module() {
  let outcome = run_case(&IMPORTS_THROUGH_A_CROSS_CRATE_RE_EXPORTED_MODULE, callees_in_main);

  let callees = outcome
    .check(&IMPORTS_THROUGH_A_CROSS_CRATE_RE_EXPORTED_MODULE)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "spanner_size" && c.rust_backed),
    "the method behind a cross-crate re-exported module did not resolve: {callees:?}"
  );
}

/// An item defined in the compiled crate itself is not importable — the walk sees dependencies.
#[test]
fn an_item_in_the_compiled_crate_is_not_importable() {
  let outcome = run_case(&AN_ITEM_IN_THE_COMPILED_CRATE_IS_NOT_IMPORTABLE, callees_in_main);

  assert!(outcome.check(&AN_ITEM_IN_THE_COMPILED_CRATE_IS_NOT_IMPORTABLE).is_none());
  assert!(
    !outcome.asked(|q| q.offered("stub_only").is_some()),
    "the compiled crate's own item was offered for import: {}",
    outcome.rendered_log()
  );
}

/// A Vale package compiled as the reserved `rust` module is refused.
///
/// The refusal is a panic rather than a compile error: the reason has to travel as a diagnostic
/// before it can be one, and the reservation is worth enforcing before that lands. Asserting the
/// message rather than the mechanism is what lets this survive the upgrade. The expected substring
/// names only the reservation, not the verb around it — the wording has already been revised once,
/// and a test that pins the whole sentence fails on rephrasing rather than on behaviour.
#[test]
#[should_panic(expected = "reserved `rust` module")]
fn a_vale_package_may_not_claim_the_rust_module() {
  run_case_in_package(&A_VALE_PACKAGE_MAY_NOT_CLAIM_THE_RUST_MODULE, "rust", callees_in_main);
}

/// The control for the case above: the identical program under an ordinary package name compiles.
///
/// Without this, the `should_panic` passes for any reason a compilation might blow up, and would
/// keep passing if the program itself went bad.
#[test]
fn the_reserved_module_case_compiles_under_an_ordinary_package() {
  let outcome = run_case(&A_VALE_PACKAGE_MAY_NOT_CLAIM_THE_RUST_MODULE, callees_in_main);

  let callees = outcome
    .check(&A_VALE_PACKAGE_MAY_NOT_CLAIM_THE_RUST_MODULE)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().any(|c| c.name == "add_two_numbers" && c.rust_backed),
    "the control program did not resolve its Rust call: {callees:?}"
  );
}

/// **Everything at once** — the composition case.
///
/// Every other case is narrow so failures localize. This one exists for the question narrowness
/// cannot answer: whether the mechanisms coexist. Interference is its own failure class — a shared
/// name resolving to the wrong item, an import-order dependency, a drop that only works when it is
/// the only drop — and no narrow case can see it.
///
/// The assertions are on the **callee list**, not on the return value alone: a program this size
/// could return 31 while silently having resolved half its calls to the wrong thing.
#[test]
fn a_program_using_everything_at_once() {
  let outcome = run_case(&A_PROGRAM_USING_EVERYTHING_AT_ONCE, callees_in_main);

  let callees =
    outcome.check(&A_PROGRAM_USING_EVERYTHING_AT_ONCE).expect("the case declares it compiles");

  // One representative of each mechanism, so a regression names which one broke.
  for expected in [
    "add_two_numbers",  // free function
    "seven",            // zero-arg
    "make_counter",     // type inferred from a signature
    "get",              // method, and the one whose name collides across two types
    "doubled",          // a second method on one type
    "or_else",          // method with its own type parameter
    "new",              // associated function, no receiver
    "bump",             // citizen flowing through two calls
    "pick",             // generic function at concrete types
    "id",               // generic function at a Rust type
    "holder_ignore",    // generic type at one argument
    "bool_holder_flag", // the same generic type at another
    "depth_reading",    // nested module, by path
    "depth_of",         // method on a nested type
    "make_sonar",       // reached through a re-export
    "drop",             // scope-end drop on the bound Rust values
  ] {
    assert!(
      callees.iter().any(|c| c.name == expected && c.rust_backed),
      "`{expected}` did not resolve as a Rust-backed callee in the composite program: \
             {callees:?}"
    );
  }

  // The three declined items sit in the allowlist and must not have been imported.
  for declined in ["first", "unsigned_count", "half_of"] {
    assert!(
      !callees.iter().any(|c| c.name == declined),
      "`{declined}` should have been declined, but reached the callee list: {callees:?}"
    );
  }

  // A Vale function taking a Rust-typed parameter, resolving alongside everything else. Not
  // Rust-backed: it is the caller, and the point is that it coexists with the callees.
  assert!(
    callees.iter().any(|c| c.name == "vale_counter_value" && !c.rust_backed),
    "the Vale function over a Rust type did not resolve in the composite program: {callees:?}"
  );

  // Drop is the mechanism most likely to break only in company, so it is asserted by shape
  // rather than by presence: four non-generic citizens and one generic one fall out of scope
  // here, and the generic drop is the one that needs `T` deduced from the value.
  let drops: Vec<&Vec<String>> =
    callees.iter().filter(|c| c.name == "drop" && c.rust_backed).map(|c| &c.params).collect();
  assert!(
    drops.iter().any(|params| params.iter().any(|p| p.contains('<'))),
    "no generic citizen was dropped in the composite program, so the generic drop never \
         composed with the others: {drops:?}"
  );
  assert!(
    drops.iter().any(|params| params.iter().all(|p| !p.contains('<'))),
    "no non-generic citizen was dropped, so the two drop shapes were not exercised \
         together: {drops:?}"
  );
}

/// A signature Vale cannot represent costs nothing when it is imported but never called: with lazy
/// synthesis its signature is never even queried.
///
/// `first<I: Iterator>(i: I) -> I::Item` returns `<I as Iterator>::Item`, and normalizing that
/// requires the `I: Iterator` predicate to find the impl. No predicates are read at all, so this is
/// not merely an unbounded parameter but an un-normalizable alias — it would decline if forced. Here
/// it is offered but uncalled, so it is never forced.
#[test]
fn declines_an_unrepresentable_signature() {
  let outcome = run_case(&DECLINES_AN_UNREPRESENTABLE_SIGNATURE, callees_in_main);

  // An uncalled unrepresentable import must not disturb the rest of the import.
  outcome.check(&DECLINES_AN_UNREPRESENTABLE_SIGNATURE);

  let first = offered(&outcome, "first");
  assert!(
    !outcome.asked(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == first)),
    "`first` is imported but never called, so its signature must never be queried:\n{}",
    outcome.rendered_log()
  );
}

/// The same unrepresentable type in **argument** position, offered but uncalled: still never queried.
#[test]
fn declines_an_unrepresentable_parameter() {
  let outcome = run_case(&DECLINES_AN_UNREPRESENTABLE_PARAMETER, callees_in_main);

  // An uncalled unrepresentable import must not disturb the rest of the import.
  outcome.check(&DECLINES_AN_UNREPRESENTABLE_PARAMETER);

  let take_first = offered(&outcome, "take_first");
  assert!(
    !outcome.asked(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == take_first)),
    "`take_first` is imported but never called, so its signature must never be queried:\n{}",
    outcome.rendered_log()
  );
}

/// An unsigned integer would decline if forced — its signature is `u32`-shaped, and `IntT` carries a
/// width but no signedness, so importing it would hand back a plausible `i32`. Offered but uncalled,
/// it is never forced, so it is never queried.
#[test]
fn declines_an_unsigned_integer() {
  let outcome = run_case(&DECLINES_AN_UNSIGNED_INTEGER, callees_in_main);

  // An uncalled unrepresentable import must not disturb the rest of the import.
  outcome.check(&DECLINES_AN_UNSIGNED_INTEGER);

  let unsigned_count = offered(&outcome, "unsigned_count");
  assert!(
    !outcome.asked(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == unsigned_count)),
    "`unsigned_count` is imported but never called, so its signature must never be queried:\n{}",
    outcome.rendered_log()
  );
}

/// The decline path, actually forced: a called Rust function whose signature Vale cannot represent
/// (an unsigned-int return) surfaces as a `CouldNotPostparseFunction` compile error, not a panic.
#[test]
fn calling_a_declined_signature_is_a_compile_error() {
  // A forced decline must be a clean diagnostic naming the item, not a `vfail` panic mid-resolution.
  run_case(&CALLING_A_DECLINED_SIGNATURE_IS_A_COMPILE_ERROR, callees_in_main)
    .check(&CALLING_A_DECLINED_SIGNATURE_IS_A_COMPILE_ERROR);
}

/// A float would decline if forced — `FloatT` has no width, so `f32` and `f64` would intern
/// identically. Offered but uncalled, it is never forced.
#[test]
fn declines_a_float() {
  let outcome = run_case(&DECLINES_A_FLOAT, callees_in_main);

  // An uncalled unrepresentable import must not disturb the rest of the import.
  outcome.check(&DECLINES_A_FLOAT);

  let half_of = offered(&outcome, "half_of");
  assert!(
    !outcome.asked(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == half_of)),
    "`half_of` is imported but never called, so its signature must never be queried:\n{}",
    outcome.rendered_log()
  );
}

/// @RTMEIZ from the side that is easy to miss: reaching a type through another item's signature does
/// not import it. `takes_hidden` is allowed, `Hidden` is not, so `takes_hidden` would decline if
/// forced — but offered and uncalled, it is never forced, so it is never queried.
#[test]
fn declines_a_signature_naming_an_unimported_type() {
  let outcome = run_case(&DECLINES_A_SIGNATURE_NAMING_AN_UNIMPORTED_TYPE, callees_in_main);

  // An uncalled unrepresentable import must not disturb the rest of the import.
  outcome.check(&DECLINES_A_SIGNATURE_NAMING_AN_UNIMPORTED_TYPE);

  let takes_hidden = offered(&outcome, "takes_hidden");
  assert!(
    !outcome.asked(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == takes_hidden)),
    "`takes_hidden` is imported but never called, so its signature must never be queried:\n{}",
    outcome.rendered_log()
  );
}

/// **Vale source can name a Rust type** — hand-written, by bare name, with no import statement.
///
/// Worth pinning because it is easy to assume otherwise. A synthesized declaration names `Counter`
/// through an ordinary `LookupSR`, which is how `make_counter`'s return position resolves; whether
/// *hand-written* source reaches the same entry is a separate question, since it arrives through
/// the postparser rather than through a rule we built. It does: the importer registers the citizen
/// as a `Kind` entry in the reserved `rust` package's top-level store, and `PackageEnvironmentT`
/// unions every top-level store, so the name is ambient.
///
/// The body is deliberately trivial. `return (c).get()` here does *not* compile — reading a
/// parameter yields `BorrowRef(Counter)` where `get(self Counter)` wants it owned, and
/// `is_type_convertible` panics on that borrow read-out (`templata_compiler.rs:1209`). That is one
/// of the two Vale-side onion-arc gaps already with Vale2, not an interop limitation, so this case
/// stays on the naming question and leaves the borrow question to them. `c` is still unconsumed,
/// so the synthesized `drop` is exercised on the way out.
///
/// What this does **not** give us is `x = Vec<int>()`: that needs a constructor (a Rust-backed type
/// gets none, deliberately — `get_struct_sibling_entries` runs only over parsed `StructS` denizens)
/// and generic Rust *types* (we have generic functions only).
#[test]
fn vale_source_can_name_a_rust_type() {
  let outcome = run_case(&VALE_SOURCE_CAN_NAME_A_RUST_TYPE, callees_in_main);

  outcome.check(&VALE_SOURCE_CAN_NAME_A_RUST_TYPE);
}

/// A generic Rust type imports **with its arguments intact**.
///
/// `Holder<i32>` and `Holder<bool>` must be two distinct Vale kinds. Until 2026-07-26 they were
/// not: both interned as a bare `Holder` with `template_args: []`, so Vale gave the same answer
/// for different types — the worst of the three possible behaviours, which is why this case
/// existed asserting the defect before it asserted the fix.
///
/// Two things had to change together. `TyCtxtOracle::type_kind` now reads the ADT's
/// `GenericArgsRef` instead of dropping it — but that alone changes nothing, because a synthesized
/// declaration does not carry the lowered kind. It names the type through rules, so the
/// declaration also had to stop emitting a bare `LookupSR` and start emitting `LookupSR` (bind the
/// template) + `CallSR` (apply the arguments), which needs the type registered as a real
/// `IEnvEntryT::Struct` rather than a finished `ITemplataT::Kind`.
#[test]
fn a_generic_rust_type_carries_its_arguments() {
  let same_kind = run_case(&A_GENERIC_RUST_TYPE_CARRIES_ITS_ARGUMENTS, |coutputs| {
    let main = coutputs.lookup_function_by_str("main");
    let callees: Vec<&PrototypeT> = collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(call) => Some(call.callable)
    );
    let ret_of = |name: &str| {
      describe_kind(
        callees
          .iter()
          .find(|p| describe_callee(p).name == name)
          .unwrap_or_else(|| panic!("no call to {name} resolved"))
          .return_type,
      )
    };
    (ret_of("make_holder"), ret_of("make_bool_holder"))
  });

  // Asserting the arguments rather than merely that the two differ: "they differ" would also be
  // satisfied by two wrong-but-distinct answers, and the defect this replaced was precisely two
  // instantiations rendering the same.
  assert_eq!(
    &("rust-citizen<int32>".to_string(), "rust-citizen<bool>".to_string()),
    same_kind
      .check(&A_GENERIC_RUST_TYPE_CARRIES_ITS_ARGUMENTS)
      .expect("the case declares it compiles")
  );
}

/// The mirror of the empty-allowlist control: the crate exports `seven`, and leaving it out of the
/// allowlist is enough to make it unreachable.
#[test]
fn an_item_not_in_the_allowlist_is_not_importable() {
  let outcome = run_case(&AN_ITEM_NOT_IN_THE_ALLOWLIST_IS_NOT_IMPORTABLE, callees_in_main);

  assert!(outcome.check(&AN_ITEM_NOT_IN_THE_ALLOWLIST_IS_NOT_IMPORTABLE).is_none());
}

/// A stale allowlist entry is inert. An `import` list outlives the crate versions it was written
/// against, so a name that stops existing must not take the compilation down.
#[test]
fn an_allowlist_entry_the_crate_does_not_export_is_ignored() {
  // Importing an item the crate does not export now fails the compile (`UnresolvableRustImport`)
  // rather than being silently ignored.
  let outcome = run_case(&AN_ALLOWLIST_ENTRY_THE_CRATE_DOES_NOT_EXPORT_IS_IGNORED, callees_in_main);
  assert!(outcome.check(&AN_ALLOWLIST_ENTRY_THE_CRATE_DOES_NOT_EXPORT_IS_IGNORED).is_none());
}

/// A crate's module children include its own `extern crate std`. Without the `DefKind` filter, a
/// name match would hand back a module where a function or type was asked for.
#[test]
fn a_module_named_in_the_allowlist_is_filtered_by_defkind() {
  // A module (not a fn/struct) fails the `DefKind` filter, so the import resolves to nothing and the
  // compile fails (`UnresolvableRustImport`) rather than the entry being silently ignored.
  let outcome = run_case(&A_MODULE_NAMED_IN_THE_ALLOWLIST_IS_FILTERED_BY_DEFKIND, callees_in_main);
  assert!(outcome.check(&A_MODULE_NAMED_IN_THE_ALLOWLIST_IS_FILTERED_BY_DEFKIND).is_none());
}

/// A Rust callee competes on `params_match` like any other candidate.
#[test]
fn wrong_argument_types_do_not_resolve() {
  let outcome = run_case(&WRONG_ARGUMENT_TYPES_DO_NOT_RESOLVE, callees_in_main);

  assert!(outcome.check(&WRONG_ARGUMENT_TYPES_DO_NOT_RESOLVE).is_none());
}

/// An oracle in scope costs an ordinary Vale program nothing.
///
/// The vacuity assertion runs the other way round here: the oracle *was* consulted (the allowlist
/// is non-empty, so items were enumerated and declared), and the program still compiled without
/// referring to any of them.
#[test]
fn a_program_using_no_rust_items_compiles_with_an_oracle_present() {
  let outcome =
    run_case(&A_PROGRAM_USING_NO_RUST_ITEMS_COMPILES_WITH_AN_ORACLE_PRESENT, callees_in_main);

  let callees = outcome
    .check(&A_PROGRAM_USING_NO_RUST_ITEMS_COMPILES_WITH_AN_ORACLE_PRESENT)
    .expect("the case declares it compiles");
  assert!(
    callees.iter().all(|c| !c.rust_backed),
    "a program mentioning no Rust item resolved a Rust callee anyway: {callees:?}"
  );
  assert!(
    outcome.asked(|q| q.offered("add_two_numbers").is_some()),
    "the oracle was never consulted, so this says nothing about its presence being free:\n{}",
    outcome.rendered_log()
  );
}

/// Two crates' items reach Vale in one compilation, and stay two types.
///
/// The distinct-short-name half of the two-crate fixture, so this exercises multiplicity without
/// also posing the collision below. Each item's `package_coord` comes from its own `tcx.def_path`,
/// so the two land in different packages and therefore in different top-level stores.
#[test]
fn imports_from_two_crates() {
  let outcome = run_case(&IMPORTS_FROM_TWO_CRATES, callees_in_main);

  let callees = outcome.check(&IMPORTS_FROM_TWO_CRATES).expect("the case declares it compiles");
  for name in ["make_gadget", "make_doohickey", "gadget_value", "doohickey_value"] {
    assert!(
      callees.iter().any(|c| c.name == name && c.rust_backed),
      "`{name}` did not resolve to a Rust callee: {callees:?}"
    );
  }
}

/// Two crates each exporting a `Widget`: **the name-collision trigger, firing.**
///
/// The corpus declares where this case must land — it compiles and returns 5, because the two
/// `Widget`s are unrelated types that merely share a short name. Today it panics instead, and this
/// test pins that, because a deferral whose trigger nobody can observe is indistinguishable from a
/// deferral that never fires (plan §0.4).
///
/// **Half of the defect is already fixed.** Each item's `package_coord` now comes from its own
/// `tcx.def_path`, so `mycrate::Widget` and `othercrate::Widget` are genuinely two Vale types in
/// two packages — before that they interned to one id and the second `declare_type` tripped its
/// own assertion. `imports_from_two_crates` is the green half of that fix.
///
/// **What remains is naming, and it is a core change.** A synthesized declaration refers to a type
/// through `LookupSR` with a bare `CodeNameS`, and `PackageEnvironmentT` unions every top-level
/// store — so looking up `Widget` finds both and `lookup_nearest_with_imprecise_name` panics
/// rather than resolving. Closing it is Problem A of the naming design: add
/// `IImpreciseNameValS::QualifiedCodeName`, have `declarations.rs` emit it, and have
/// `get_imprecise_name` derive the same key for a registered Rust citizen. Two of those three are
/// in core name types, so they are the architect's rather than this arc's.
///
/// **The `should_panic` is gone as of the package-path change** — a synthesized declaration now
/// names a citizen by its package coordinate, so the two `Widget`s are reached by different paths
/// and the ambiguity never forms. What the case asserts is what the corpus always declared: both
/// import, and their constructors return **different kinds**.
///
/// Distinctness has a second half this case structurally cannot see — a conflated pair would still
/// satisfy every call in this program, since each is consistent within its own crate. That is
/// `a_type_from_one_crate_does_not_satisfy_the_others_parameter`.
#[test]
fn two_crates_exporting_the_same_short_name_stay_distinct() {
  run_case(&TWO_CRATES_EXPORTING_THE_SAME_SHORT_NAME_STAY_DISTINCT, |coutputs| {
    let main = coutputs.lookup_function_by_str("main");
    let callees: Vec<&PrototypeT> = collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(call) => Some(call.callable)
    );
    let ret_of = |name: &str| {
      callees
        .iter()
        .find(|p| describe_callee(p).name == name)
        .unwrap_or_else(|| panic!("no call to {name} resolved"))
        .return_type
    };
    ret_of("make_widget") != ret_of("make_other_widget")
  });
}

/// The distinctness half — and the only shape that can observe it.
///
/// The case above proves both `Widget`s import. It cannot prove they stayed *distinct*: a
/// conflated pair satisfies every call in that program, because each call is consistent within its
/// own crate. Crossing them is what tells them apart, and it does so by **failing** — `widget_value`
/// takes `mycrate`'s `Widget` and is handed `othercrate`'s.
///
/// So a regression that merged the two types makes this case start *compiling*. That is an unusual
/// direction for a corpus case and worth stating plainly, since "it passes" here means "the
/// compiler rejected the program."
#[test]
fn a_type_from_one_crate_does_not_satisfy_the_others_parameter() {
  let outcome =
    run_case(&A_TYPE_FROM_ONE_CRATE_DOES_NOT_SATISFY_THE_OTHERS_PARAMETER, callees_in_main);

  outcome.check(&A_TYPE_FROM_ONE_CRATE_DOES_NOT_SATISFY_THE_OTHERS_PARAMETER);

  // Vacuity: the callee must have been *offered*, or the case would pass for the boring reason
  // that nothing named `widget_value` was importable at all.
  assert!(
    outcome.asked(|q| q.offered("widget_value").is_some()),
    "`widget_value` was never offered, so the rejection proves nothing:\n{}",
    outcome.rendered_log()
  );
}

/// **@ATAFLBZ fence: nothing in `rust_interop/` may take a Rust item's identity from its human
/// name.**
///
/// The hazard is that Rust has no uniqueness rule for short names — `new`, `len`, `Error`, `Box`
/// recur across crates — and `tcx.crates(())` hands us every loaded crate. A `DefId` chosen by
/// string match eventually drives a mangled symbol, so the failure surfaces as a link error against
/// a plausible-looking name, far from the mistake.
///
/// Three sites once decided this way; two were deleted with the per-call-site oracle and the third
/// now derives each item's `package_coord` from `tcx.def_path`. **The fence is not for those three
/// — it is for the next one**, which is why Harmonious pushed for it after their own version of
/// this bug: *"the value is not the site that was fixed, it is the next one."*
///
/// A grep rather than an AST walk, deliberately: the pattern is a *comparison against a name
/// field*, which is one line and reads the same in any shape. Add an allow-marker comment on the
/// line if a match is genuinely about **selection** (which items an allowlist admits) rather than
/// **identity** — the allowlist is name-shaped by its own semantics, and that is fine.
#[test]
fn no_rust_item_identity_comes_from_a_human_name() {
  const ALLOW: &str = "ataflbz-allow";
  let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/typing/rust_interop");

  let mut offenders: Vec<String> = Vec::new();
  let mut walk = vec![dir];
  while let Some(path) = walk.pop() {
    for entry in read_dir(&path).expect("could not read rust_interop dir") {
      let entry = entry.expect("could not read dir entry").path();
      if entry.is_dir() {
        walk.push(entry);
        continue;
      }
      // Fixture crates are Rust *input*, not compiler source.
      if entry.extension().is_none_or(|e| e != "rs") || entry.to_string_lossy().contains("fixtures")
      {
        continue;
      }
      let source = read_to_string(&entry).expect("could not read source");
      for (number, line) in source.lines().enumerate() {
        if line.contains(ALLOW) {
          continue;
        }
        let compares_a_name = (line.contains("human_name") || line.contains(".ident"))
          && (line.contains("==") || line.contains("!=") || line.contains(".contains("));
        if compares_a_name {
          offenders.push(format!(
            "{}:{}: {}",
            entry.file_name().expect("a file has a name").to_string_lossy(),
            number + 1,
            line.trim()
          ));
        }
      }
    }
  }

  assert!(
    offenders.is_empty(),
    "these lines take a Rust item's identity from a human name (@ATAFLBZ). Key on `DefId` or \
         on the `tcx.def_path`-derived package coordinate instead; if the comparison is about \
         which items the allowlist *admits* rather than which item something *is*, add a \
         `{ALLOW}` comment on the line:\n  {}",
    offenders.join("\n  ")
  );
}

/// Every `Case`'s `name` must be unique.
///
/// The name is the case's build directory (`vale-interop-cases/<name>`), where `build_dep_rlib`
/// writes `lib<crate>.rlib`. Two cases sharing a name share that directory, so their two test threads
/// race on the one rlib — one build clobbers or half-writes the other's, and whichever loses fails
/// nondeterministically with "building the dependency rlib failed" while both pass in isolation. This
/// asserts uniqueness so a future collision fails loudly and locally instead of flaking the suite.
#[test]
fn every_case_name_is_unique() {
  let corpus = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/typing/rust_interop/corpus.rs");
  let source = read_to_string(&corpus).expect("could not read corpus.rs");

  // Match the `Case` struct field `name: "…",` exactly, so prose and Vale source are not counted.
  let mut names: Vec<String> = Vec::new();
  for line in source.lines() {
    let trimmed = line.trim();
    let Some(rest) = trimmed.strip_prefix("name: \"") else { continue };
    let Some(name) = rest.strip_suffix("\",") else { continue };
    names.push(name.to_string());
  }
  // A floor guards against the extraction silently matching nothing and passing vacuously.
  assert!(
    names.len() > 40,
    "found only {} case names — the `name:` extraction likely broke",
    names.len()
  );

  let mut sorted = names.clone();
  sorted.sort();
  let mut dups: Vec<String> =
    sorted.windows(2).filter(|w| w[0] == w[1]).map(|w| w[0].clone()).collect();
  dups.dedup();
  assert!(
    dups.is_empty(),
    "these Case names are used more than once. The name is the per-case build out_dir \
         (vale-interop-cases/<name>), so a duplicate races two test threads on one libmycrate.rlib \
         and flakes the suite. Give each Case a unique name:\n  {}",
    dups.join("\n  ")
  );
}

/// Every fixture's `stub.rs` must be **valid Rust**, not merely parseable.
///
/// Nothing else checks this. `after_expansion` runs before type checking and the harness returns
/// `Compilation::Stop`, so a stub that type-errors is invisible to every case — they would all keep
/// passing while the fixture rotted. Dependency crates are already covered, because `build_dep_rlib`
/// compiles them in full and asserts success; the stub was the gap.
///
/// **`fixtures_broken_rust` is excluded on purpose** — it does not parse, by design, and is the
/// input to `a_fatal_rustc_error_costs_one_case`. Anyone extending this check must keep skipping
/// it, or they break the case that proves a broken fixture costs one test rather than the run.
#[test]
fn every_fixture_stub_is_valid_rust() {
  for fixture in ["fixtures", "fixtures_two_crates"] {
    if let Err(stderr) = compile_check_fixture(fixture) {
      panic!("fixture `{fixture}`'s stub.rs does not compile:\n{stderr}");
    }
  }
}

/// The surviving hazard of hosting rustc in `cargo test --lib`, pinned as a regression test.
///
/// `fixtures_broken_rust/stub.rs` does not parse, so this drives a rustc **fatal** error through
/// an in-process `run_compiler`. rustc's fatal path exits rather than returning, which would take
/// the whole test binary down — every other test with it — instead of failing here. Measured
/// behaviour is that it costs exactly this case: `run_compiler` returns, `after_expansion` never
/// runs, and there is no outcome.
///
/// A **parse** error specifically. `after_expansion` runs before type checking and the callback
/// returns `Compilation::Stop`, so a type error would never be reached.
#[test]
fn a_fatal_rustc_error_costs_one_case() {
  let outcome = try_run_case(&A_FATAL_RUSTC_ERROR_COSTS_ONE_CASE, callees_in_main);

  assert!(
    outcome.is_none(),
    "rustc was expected to fail before after_expansion, but the callback ran"
  );
}
