// The tier-1 interop corpus: one behaviour per case, each against a real `TyCtxt`.
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

use crate::collect_where_tnode;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::names::names::INameT;
use crate::typing::rust_interop::{
    citizen_id, is_rust_backed, peel_refs, OracleQuery, RustItemId, SigPosition,
};
use crate::typing::test::rust_interop::harness::{run_case, try_run_case, CaseOutcome};
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
            Some(id) if is_rust_backed(id) => "rust-citizen".to_string(),
            Some(_) => "vale-citizen".to_string(),
            None => "non-citizen".to_string(),
        },
    }
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
    let outcome = run_case(
        "fixtures",
        "free-function",
        "\nexported func main() int {\n  return add_two_numbers(3, 4);\n}",
        &["add_two_numbers"],
        callees_in_main,
    );

    assert_eq!(
        &vec![Callee {
            name: "add_two_numbers".to_string(),
            rust_backed: true,
            params: vec!["int32".to_string(), "int32".to_string()],
            ret: "int32".to_string(),
        }],
        outcome.expect_compiled()
    );

    // Vacuity: the program above would compile just as happily if a Vale `add_two_numbers` were
    // in scope. This is what says the name came from Rust.
    assert!(
        outcome.asked(|q| q.offered("add_two_numbers").is_some()),
        "the oracle was never asked for the function this program calls:\n{}",
        outcome.rendered_log()
    );
}

/// The negative control for the case above. If the same program compiled with nothing importable,
/// the positive case would prove nothing about where resolution came from.
#[test]
fn an_empty_allowlist_makes_nothing_importable() {
    let outcome = run_case(
        "fixtures",
        "empty-allowlist",
        "\nexported func main() int {\n  return add_two_numbers(3, 4);\n}",
        &[],
        callees_in_main,
    );

    let failure = outcome.expect_failure();
    assert!(
        failure.is("CouldntFindFunctionToCallT"),
        "expected the call to go unresolved, but it failed with {}:\n{}",
        failure.variant,
        failure.detail
    );
}

/// A generic Rust function is read **structurally** — parameters intact, not collapsed to one
/// instantiation. This is the thing the previous design could not express at all, and the reason
/// the arc pivoted.
#[test]
fn reads_a_generic_signature_structurally() {
    let outcome = run_case(
        "fixtures",
        "generic-function",
        "\nexported func main() int {\n  return pick<int, bool>(add_two_numbers(3, 4), true);\n}",
        &["add_two_numbers", "pick"],
        callees_in_main,
    );

    // The strong half of this assertion is that the program compiled at all: it calls
    // `pick<int, bool>` and returns the result from `main() int`, so binding `A` to the wrong slot
    // yields `bool` where `int` belongs and fails to resolve. `id<T>(x: T) -> T` would pass under
    // either mapping and prove nothing.
    let callees = outcome.expect_compiled();
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
    let outcome = run_case(
        "fixtures",
        "method",
        "\nexported func main() int {\n  return (make_counter()).get();\n}",
        &["make_counter", "Counter"],
        callees_in_main,
    );

    let callees = outcome.expect_compiled();
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
    let outcome = run_case(
        "fixtures",
        "scope-end-drop",
        "\nexported func main() int {\n  c = make_counter();\n  return 7;\n}",
        &["make_counter", "Counter"],
        callees_in_main,
    );

    let callees = outcome.expect_compiled();
    assert!(
        callees.iter().any(|c| c.name == "drop" && c.rust_backed),
        "the bound Rust value got no scope-end drop: {callees:?}"
    );
}

/// A signature Vale cannot represent is **declined**, not imported with a hole in it.
///
/// `first<I: Iterator>(i: I) -> I::Item` returns `<I as Iterator>::Item`, and normalizing that
/// requires the `I: Iterator` predicate to find the impl. No predicates are read at all, so this
/// is not merely an unbounded parameter but an un-normalizable alias.
#[test]
fn declines_an_unrepresentable_signature() {
    let outcome = run_case(
        "fixtures",
        "declined-signature",
        "\nexported func main() int {\n  return add_two_numbers(3, 4);\n}",
        &["add_two_numbers", "first"],
        callees_in_main,
    );

    // Declining one item must not disturb the rest of the import.
    outcome.expect_compiled();

    let first = offered(&outcome, "first");
    assert!(
        outcome.asked(
            |q| matches!(q, OracleQuery::FnSig { item, answer: None } if *item == first)
        ),
        "`first` was offered but its signature was not declined:\n{}",
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
    let outcome = run_case(
        "fixtures",
        "vale-names-a-rust-type",
        "\nexported func main() int {\n  return value_of(make_counter());\n}\n\
         func value_of(c Counter) int {\n  return 7;\n}",
        &["make_counter", "Counter"],
        callees_in_main,
    );

    outcome.expect_compiled();
}

/// A generic Rust type imports **with its arguments silently dropped**.
///
/// `Holder<i32>` and `Holder<bool>` both intern as a bare `Holder` with `template_args: []`, so
/// Vale cannot tell two instantiations apart. It does not fail — it compiles, and gives the same
/// answer for different types, which is the worst of the three possible behaviours.
///
/// The cause is one line: `TyCtxtOracle::type_kind` builds its `StructNameValT` with
/// `template_args: &[]` and never reads the ADT's `GenericArgsRef`. Note the shape of the gap —
/// generic *functions* work (their parameters live on the signature and Vale's solver substitutes
/// them), while a generic *type* needs the citizen itself to carry args, which is a different
/// mechanism nothing has built yet.
///
/// **This case asserts the defect**, deliberately, so that it is pinned rather than merely known.
/// When generic types land, invert the assertion — it becomes the regression test for the fix.
#[test]
fn a_generic_rust_type_loses_its_arguments() {
    let same_kind = run_case(
        "fixtures",
        "generic-type-arguments",
        "\nexported func main() int {\n  a = make_holder();\n  b = make_bool_holder();\n  return 7;\n}",
        &["make_holder", "make_bool_holder", "Holder"],
        |coutputs| {
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
            ret_of("make_holder") == ret_of("make_bool_holder")
        },
    );

    assert!(
        *same_kind.expect_compiled(),
        "generic Rust types now carry their arguments — invert this assertion, it is the \
         regression test for that fix"
    );
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
    let outcome = try_run_case(
        "fixtures_broken_rust",
        "fatal-rustc-error",
        "\nexported func main() int {\n  return add_two_numbers(3, 4);\n}",
        &["add_two_numbers"],
        callees_in_main,
    );

    assert!(
        outcome.is_none(),
        "rustc was expected to fail before after_expansion, but the callback ran"
    );
}
