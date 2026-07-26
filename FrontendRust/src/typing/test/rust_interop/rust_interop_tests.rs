// Typing-pass tests for the Rust-interop seam, driven by a fixture oracle.
//
// No rustc, and no `import rust.X` line: `resolve_function` is name-keyed, and which
// names are in scope is the oracle's to decide. That keeps these tests to the typing
// pass alone.

use bumpalo::Bump;
use crate::collect_only_tnode;
use crate::code_source::CodeSource;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::new_test_code_map;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::names::names::INameT;
use crate::typing::rust_interop::{is_rust_backed, FixtureFunction, FixtureOracle};
use crate::typing::test::compiler_test_compilation::compiler_test_compilation_with_rust_oracle;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::types::types::*;
use crate::typing::typing_interner::TypingInterner;

#[test]
fn calls_a_rust_free_function() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r"
exported func main() int {
  return add_two_numbers(3, 4);
}";
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, code),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);

    let int = KindT::Int(IntT { bits: 32 });
    let rust_module = scout_arena.intern_str("rust");
    let mycrate = scout_arena.intern_str("mycrate");
    let oracle = FixtureOracle::new(
        &scout_arena,
        rust_module,
        &[mycrate],
        vec![FixtureFunction { name: "add_two_numbers", params: vec![int, int], ret: int }],
    );

    let mut compile = compiler_test_compilation_with_rust_oracle(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &code_source, &oracle,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");

    // The call resolves to an ordinary Vale function — the extern *wrapper* that
    // `make_extern_function` builds — living in the reserved `rust` package. That it is an
    // ordinary `Function` rather than a bare `ExternFunction` prototype is the point of the
    // synthesized-declaration design: the oracle produced a declaration, and the ordinary
    // machinery compiled it. The `ExternFunctionNameT` prototype still exists, one level down,
    // as the target of the `ExternFunctionCallTE` in this wrapper's body.
    let callee: &PrototypeT = collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(call) => Some(call.callable)
    );
    assert!(is_rust_backed(&callee.id));
    assert_eq!(callee.return_type, int);
    match callee.id.local_name {
        INameT::Function(name) => {
            assert_eq!(name.template.human_name.0, "add_two_numbers");
            // Params ride the name, because PrototypeT::param_types is name-derived — a name
            // that disagreed with the signature would report wrong types at every call site.
            assert_eq!(name.parameters, &[int, int]);
        }
        other => panic!("expected a Function name, got {:?}", other),
    }
}
