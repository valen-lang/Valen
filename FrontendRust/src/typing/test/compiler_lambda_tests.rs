use bumpalo::Bump;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::ast::ast::ParameterT;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::names::names::IVarNameT;
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::typing::types::types::{CoordT, IntT, IRegionT, KindT, OwnershipT, RegionT};
use crate::utils::code_hierarchy::{self, IPackageResolver, PackageCoordinate};
use crate::utils::fx::HashMap;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::names::names::CodeVarNameT;
use crate::interner::StrI;
use crate::typing::typing_interner::TypingInterner;
use crate::builtins::builtins::get_embedded_modulized_code_map;
use crate::collect_only_tnode;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::names::names::{FunctionNameT, FunctionTemplateNameT, IdT, INameT};
use crate::tests::tests::get_package_to_resource_resolver;

pub struct CompilerLambdaTests;

impl CompilerLambdaTests {}


fn read_code_from_resource(resource_filename: &str) -> String {
    panic!("Unimplemented: read_code_from_resource");
}

#[test]
fn simple_lambda() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nexported func main() int { return { 7 }(); }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let expected = CoordT::new(OwnershipT::Own, RegionT { region: IRegionT::Default }, KindT::Int(IntT { bits: 32 }));
    assert_eq!(coutputs.lookup_lambda_in("main").header.return_type, expected);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type, expected);
}

#[test]
fn lambda_with_one_magic_arg() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nexported func main() int { return {^_}(3); }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambda = coutputs.lookup_lambda_in("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(lambda),
        NodeRefT::Parameter(
            ParameterT {
                virtuality: None,
                tyype: CoordT {
                    ownership: OwnershipT::Own,
                    kind: KindT::Int(IntT { bits: 32 }),
                    ..
                },
                ..
            }
        ) => Some(())
    );
    assert_eq!(
        coutputs.lookup_lambda_in("main").header.return_type,
        CoordT::new(OwnershipT::Own, RegionT { region: IRegionT::Default }, KindT::Int(IntT { bits: 32 })),
    );
}

#[test]
fn lambda_is_reused() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r"
exported func main() {
  lam = x => ^x;
  lam(4);
  lam(7);
}
";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambdas = coutputs.lookup_lambdas_in("main");
    assert_eq!(lambdas.len(), 1);
}

#[test]
fn lambda_called_with_different_types() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r"
exported func main() {
  lam = x => ^x;
  lam(4);
  lam(true);
}
";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambdas = coutputs.lookup_lambdas_in("main");
    assert_eq!(lambdas.len(), 2);
}

#[test]
fn curried_lambda() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r#"
exported func main() {
  lam = x => y => 7;
  lam(true)(4);
  lam(true)("hello");
}
"#;
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambdas = coutputs.lookup_lambdas_in("main");
    assert_eq!(lambdas.len(), 3);
}

#[test]
fn lambda_with_a_type_specified_param() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    // TSUGAR: function-call +(a,a) → binary a + a (auto-borrows operands to match +(&int, &int) post-flip)
    let code = r"
import v.builtins.arith.*;
exported func main() int {
  return (a int) => {a + a}(3);
}
";
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambda = coutputs.lookup_lambda_in("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(lambda),
        NodeRefT::Parameter(
            ParameterT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("a"), .. }),
                virtuality: None,
                tyype: CoordT {
                    ownership: OwnershipT::Own,
                    kind: KindT::Int(IntT { bits: 32 }),
                    ..
                },
                ..
            }
        ) => Some(())
    );
}

#[test]
fn lambda_emits_call_and_drop() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r#"
exported func main() int { return { 7 }(); }
"#;
    // AFTERM: we should move away from the .or stuff for resolving
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    // Exactly one FunctionCall in main is the lambda's __call.
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(FunctionCallTE {
            callable: PrototypeT { id: IdT {
                local_name: INameT::LambdaCallFunction(_), ..
            }, .. },
            ..
        }) => Some(())
    );
    // Exactly one FunctionCall in main is the lambda struct's auto-generated drop.
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(FunctionCallTE {
            callable: PrototypeT { id: IdT {
                local_name: INameT::Function(FunctionNameT {
                    template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                    ..
                }),
                init_steps: [_, INameT::LambdaCitizenTemplate(_)],
                ..
            }, .. },
            ..
        }) => Some(())
    );
}

#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn tests_lambda_and_concept_function() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r#"
import v.builtins.print.*;
import v.builtins.drop.*;
import v.builtins.str.*;

func moo<X, F>(x X, f F)
where func(&F, &X)void, func drop(X)void, func drop(F)void {
  f(&x);
}
exported func main() {
  moo("hello", { print(_); });
}
"#;
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    compile.expect_compiler_outputs();
}

#[test]
fn lambda_inside_different_function_with_same_name() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    // TSUGAR: print(x) where x is captured int → needs copy
    let code = r#"
import printutils.*;

func helperFunc(x int) {
  { print(__copy_prim(&x)); }();
}
func helperFunc(x str) {
  { print(&x); }();
}
exported func main() {
  helperFunc(4);
  helperFunc("bork");
}
"#;
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    compile.expect_compiler_outputs();
}

#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn lambda_inside_template() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    // TSUGAR: where `func print(&T)void` → `func print(T)void`; `print(x)` → `print(__copy_prim(x))` (auto-coerce-reversal of borrow + addressible-primitive capture)
    let code = r#"
import v.builtins.drop.*;
import printutils.*;

func helperFunc<T>(x T)
where func print(T)void, func drop(T)void
{
  { print(__copy_prim(&x)); }();
}
exported func main() {
  helperFunc(4);
  helperFunc("bork");
}
"#;
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    compile.expect_compiler_outputs();
}

#[test]
fn curried_lambda_inside_template() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    // TSUGAR: helper(4) → helper(&4); helper("bork") → helper(&"bork") — helper wants &T, `4`/`"bork"` are Own
    let code = r#"
import v.builtins.drop.*;
func helper<T>(x &T) &T {
  lam = a => b => x;
  return lam(true)(7);
}
exported func main() {
  helper(&4);
  helper(&"bork");
}
"#;
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambdas = coutputs.lookup_lambdas_in("helper");
    assert_eq!(lambdas.len(), 2);
}


// Probe for the primitive-flip experiment: same body as curried_lambda_inside_template
// but with explicit `&` on the literal call-site args. Verifies whether the parser/typer
// already supports `&` on rvalue literals as the auto-borrow workaround.
#[test]
fn curried_lambda_inside_template_explicit_borrow_probe() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "import v.builtins.drop.*;\nfunc helper<T>(x &T) &T {\n  lam = a => b => x;\n  return lam(true)(7);\n}\nexported func main() {\n  helper(&4);\n  helper(&\"bork\");\n}\n";
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambdas = coutputs.lookup_lambdas_in("helper");
    assert_eq!(lambdas.len(), 2);
}

