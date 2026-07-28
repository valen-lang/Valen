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
use crate::typing::names::names::{IdT, INameT};
use crate::typing::rust_interop::corpus::*;
use crate::typing::rust_interop::{
    citizen_id, is_rust_backed, peel_refs, Case, OracleQuery, RustItemId, SigPosition,
};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::test::rust_interop::harness::{
    compile_check_fixture, run_case, try_run_case, CaseOutcome,
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
    outcome
        .oracle_log
        .iter()
        .find_map(|c| c.query.offered(name))
        .unwrap_or_else(|| {
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

    let callees = outcome
        .check(&INSTANTIATES_A_GENERIC_AT_A_RUST_TYPE)
        .expect("the case declares it compiles");
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

/// Method discovery is a list, not a lucky single.
#[test]
fn calls_two_methods_on_one_type() {
    assert_rust_callees(&CALLS_TWO_METHODS_ON_ONE_TYPE, &["get", "doubled"]);
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

/// Arity is checked rather than silently truncated (@ETASTZ).
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
    let callees = outcome
        .check(&READS_A_GENERIC_SIGNATURE_STRUCTURALLY)
        .expect("the case declares it compiles");
    assert!(
        callees.iter().any(|c| c.name == "pick" && c.rust_backed && c.ret == "int32"),
        "the generic call did not resolve to a Rust callee returning int: {callees:?}"
    );

    let pick = offered(&outcome, "pick");
    match outcome.find_query(|q| matches!(q, OracleQuery::FnSig { item, .. } if *item == pick)) {
        Some(OracleQuery::FnSig { answer: Some(shape), .. }) => {
            assert_eq!(vec!["A".to_string(), "B".to_string()], shape.generic_params);
            assert_eq!(
                vec![SigPosition::Generic(0), SigPosition::Generic(1)],
                shape.params
            );
            assert_eq!(SigPosition::Generic(0), shape.ret);
        }
        other => panic!(
            "expected a structural signature for `pick`, got {other:?}:\n{}",
            outcome.rendered_log()
        ),
    }
}

/// A Rust type reaches Vale by inference from a signature — never by name — and its method is an
/// ordinary top-level function whose first parameter is the receiver.
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
/// `Counter::get` and `Gauge::get` share a name deliberately. There is no per-type method table to
/// keep apart — the risk is the importer pairing a method with the wrong receiver, which surfaces
/// as a resolution failure rather than a wrong answer.
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

    let callees = outcome
        .check(&A_RUST_TYPE_FLOWS_THROUGH_TWO_CALLS)
        .expect("the case declares it compiles");
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

    let callees = outcome
        .check(&IMPORTS_AN_ITEM_FROM_A_NESTED_MODULE)
        .expect("the case declares it compiles");
    assert!(
        callees.iter().any(|c| c.name == "depth_reading" && c.rust_backed),
        "the nested function did not resolve as a Rust-backed callee: {callees:?}"
    );
}

/// A type in a nested module, plus its method — a different `DefKind` and therefore a different arm.
#[test]
fn imports_a_type_from_a_nested_module() {
    let outcome = run_case(&IMPORTS_A_TYPE_FROM_A_NESTED_MODULE, callees_in_main);

    let callees = outcome
        .check(&IMPORTS_A_TYPE_FROM_A_NESTED_MODULE)
        .expect("the case declares it compiles");
    assert!(
        callees.iter().any(|c| c.name == "depth_of" && c.rust_backed),
        "the nested type's method did not resolve: {callees:?}"
    );
}

/// An item reached through a re-exported name — the shape `std::vec::Vec` actually has.
#[test]
fn imports_through_a_re_exported_item() {
    let outcome = run_case(&IMPORTS_THROUGH_A_RE_EXPORTED_ITEM, callees_in_main);

    let callees = outcome
        .check(&IMPORTS_THROUGH_A_RE_EXPORTED_ITEM)
        .expect("the case declares it compiles");
    assert!(
        callees.iter().any(|c| c.name == "depth_reading" && c.rust_backed),
        "the re-exported function did not resolve: {callees:?}"
    );
}

/// Descending **through** a re-exported module, rather than landing on a re-exported item.
#[test]
fn imports_through_a_re_exported_module() {
    let outcome = run_case(&IMPORTS_THROUGH_A_RE_EXPORTED_MODULE, callees_in_main);

    let callees = outcome
        .check(&IMPORTS_THROUGH_A_RE_EXPORTED_MODULE)
        .expect("the case declares it compiles");
    assert!(
        callees.iter().any(|c| c.name == "depth_of" && c.rust_backed),
        "the method on a type behind a re-exported module did not resolve: {callees:?}"
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

    let callees = outcome
        .check(&A_PROGRAM_USING_EVERYTHING_AT_ONCE)
        .expect("the case declares it compiles");

    // One representative of each mechanism, so a regression names which one broke.
    for expected in [
        "add_two_numbers",   // free function
        "seven",             // zero-arg
        "make_counter",      // type inferred from a signature
        "get",               // method, and the one whose name collides across two types
        "doubled",           // a second method on one type
        "or_else",           // method with its own type parameter
        "new",               // associated function, no receiver
        "bump",              // citizen flowing through two calls
        "pick",              // generic function at concrete types
        "id",                // generic function at a Rust type
        "holder_ignore",     // generic type at one argument
        "bool_holder_flag",  // the same generic type at another
        "depth_reading",     // nested module, by path
        "depth_of",          // method on a nested type
        "make_sonar",        // reached through a re-export
        "drop",              // scope-end drop on the bound Rust values
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
}

/// A signature Vale cannot represent is **declined**, not imported with a hole in it.
///
/// `first<I: Iterator>(i: I) -> I::Item` returns `<I as Iterator>::Item`, and normalizing that
/// requires the `I: Iterator` predicate to find the impl. No predicates are read at all, so this
/// is not merely an unbounded parameter but an un-normalizable alias.
#[test]
fn declines_an_unrepresentable_signature() {
    let outcome = run_case(&DECLINES_AN_UNREPRESENTABLE_SIGNATURE, callees_in_main);

    // Declining one item must not disturb the rest of the import.
    outcome.check(&DECLINES_AN_UNREPRESENTABLE_SIGNATURE);

    let first = offered(&outcome, "first");
    assert!(
        outcome.asked(
            |q| matches!(q, OracleQuery::FnSig { item, answer: None } if *item == first)
        ),
        "`first` was offered but its signature was not declined:\n{}",
        outcome.rendered_log()
    );
}

/// The same decline in **argument** position — a different code path from the return position.
#[test]
fn declines_an_unrepresentable_parameter() {
    let outcome = run_case(&DECLINES_AN_UNREPRESENTABLE_PARAMETER, callees_in_main);

    // Declining one item must not disturb the rest of the import.
    outcome.check(&DECLINES_AN_UNREPRESENTABLE_PARAMETER);

    let take_first = offered(&outcome, "take_first");
    assert!(
        outcome.asked(
            |q| matches!(q, OracleQuery::FnSig { item, answer: None } if *item == take_first)
        ),
        "`take_first` was offered but its signature was not declined:\n{}",
        outcome.rendered_log()
    );
}

/// An unsigned integer declines by the same exit as an alias — it does not panic.
///
/// The gap is **signedness**: `IntT` carries a width and nothing else, so importing `u32` would
/// hand back a plausible `i32`. Until 2026-07-27 this panicked, which made one un-importable item
/// anywhere in a crate's export surface fatal to the whole import rather than to itself.
#[test]
fn declines_an_unsigned_integer() {
    let outcome = run_case(&DECLINES_AN_UNSIGNED_INTEGER, callees_in_main);

    // Declining one item must not disturb the rest of the import.
    outcome.check(&DECLINES_AN_UNSIGNED_INTEGER);

    let unsigned_count = offered(&outcome, "unsigned_count");
    assert!(
        outcome.asked(
            |q| matches!(q, OracleQuery::FnSig { item, answer: None } if *item == unsigned_count)
        ),
        "`unsigned_count` was offered but its signature was not declined:\n{}",
        outcome.rendered_log()
    );
}

/// A float declines because `FloatT` has no width, so `f32` and `f64` would intern identically.
#[test]
fn declines_a_float() {
    let outcome = run_case(&DECLINES_A_FLOAT, callees_in_main);

    // Declining one item must not disturb the rest of the import.
    outcome.check(&DECLINES_A_FLOAT);

    let half_of = offered(&outcome, "half_of");
    assert!(
        outcome.asked(|q| matches!(q, OracleQuery::FnSig { item, answer: None } if *item == half_of)),
        "`half_of` was offered but its signature was not declined:\n{}",
        outcome.rendered_log()
    );
}

/// @RTMEIZ from the side that is easy to miss: reaching a type through another item's signature
/// does not import it.
///
/// `takes_hidden` is allowed, `Hidden` is not. Declining is what keeps the allowlist meaning *"what
/// Vale may use"* rather than quietly becoming *"what Vale may reach"* — under the latter, the
/// scoping cases would be asserting something weaker than they claim.
#[test]
fn declines_a_signature_naming_an_unimported_type() {
    let outcome = run_case(&DECLINES_A_SIGNATURE_NAMING_AN_UNIMPORTED_TYPE, callees_in_main);

    // Declining one item must not disturb the rest of the import.
    outcome.check(&DECLINES_A_SIGNATURE_NAMING_AN_UNIMPORTED_TYPE);

    let takes_hidden = offered(&outcome, "takes_hidden");
    assert!(
        outcome.asked(
            |q| matches!(q, OracleQuery::FnSig { item, answer: None } if *item == takes_hidden)
        ),
        "`takes_hidden` was offered but its signature was not declined:\n{}",
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
    let same_kind = run_case(
        &A_GENERIC_RUST_TYPE_CARRIES_ITS_ARGUMENTS,
        |coutputs| {
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
        },
    );

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
    assert_rust_callees(&AN_ALLOWLIST_ENTRY_THE_CRATE_DOES_NOT_EXPORT_IS_IGNORED, &["add_two_numbers"]);
}

/// A crate's module children include its own `extern crate std`. Without the `DefKind` filter, a
/// name match would hand back a module where a function or type was asked for.
#[test]
fn a_module_named_in_the_allowlist_is_filtered_by_defkind() {
    assert_rust_callees(&A_MODULE_NAMED_IN_THE_ALLOWLIST_IS_FILTERED_BY_DEFKIND, &["add_two_numbers"]);
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
            if entry.extension().is_none_or(|e| e != "rs")
                || entry.to_string_lossy().contains("fixtures")
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
