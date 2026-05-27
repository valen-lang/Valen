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
use std::collections::HashMap;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::names::names::CodeVarNameT;
use crate::interner::StrI;

// mig: struct CompilerLambdaTests
pub struct CompilerLambdaTests;

// mig: impl CompilerLambdaTests
impl CompilerLambdaTests {}

/*
package dev.vale.typing

import dev.vale.Collector.ProgramWithExpect
import dev.vale.postparsing._
import dev.vale.postparsing.rules.IRulexSR
import dev.vale.solver.{RuleError, Step}
import dev.vale.typing.OverloadResolver.{FindFunctionFailure, InferFailure, WrongNumberOfArguments}
import dev.vale.typing.ast.{ConstantIntTE, DestroyTE, DiscardTE, FunctionCallTE, FunctionHeaderT, FunctionDefinitionT, KindExportT, LetAndLendTE, LetNormalTE, LocalLookupTE, ParameterT, PrototypeT, ReferenceMemberLookupTE, ReturnTE, SoftLoadTE, referenceExprResultKind, referenceExprResultStructName, _}
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.infer.{KindIsNotConcrete, OwnershipDidntMatch}
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{CodeLocationS, Collector, Err, FileCoordinateMap, PackageCoordinate, RangeS, Tests, vassert, vassertOne, vpass, _}
//import dev.vale.typingpass.infer.NotEnoughToSolveError
import org.scalatest._

import scala.collection.immutable.List
import scala.io.Source

class CompilerLambdaTests extends FunSuite with Matchers {
  // TODO: pull all of the typingpass specific stuff out, the unit test-y stuff
*/
// mig: fn read_code_from_resource
fn read_code_from_resource(resource_filename: &str) -> String {
    panic!("Unimplemented: read_code_from_resource");
}
/*
  def readCodeFromResource(resourceFilename: String): String = {
    val is = Source.fromInputStream(getClass().getClassLoader().getResourceAsStream(resourceFilename))
    vassert(is != null)
    is.mkString("")
  }
*/
// mig: fn simple_lambda
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
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let expected = CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: KindT::Int(IntT { bits: 32 }) };
    assert_eq!(coutputs.lookup_lambda_in("main").header.return_type, expected);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type, expected);
}
/*
  test("Simple lambda") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func main() int { return { 7 }(); }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // Make sure it inferred the param type and return type correctly
    coutputs.lookupLambdaIn("main").header.returnType shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
    coutputs.lookupFunction("main").header.returnType shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
  }
*/
// mig: fn lambda_with_one_magic_arg
#[test]
fn lambda_with_one_magic_arg() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nexported func main() int { return {_}(3); }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambda = coutputs.lookup_lambda_in("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(lambda),
        NodeRefT::Parameter(
            ParameterT {
                virtuality: None,
                tyype: CoordT {
                    ownership: OwnershipT::Share,
                    kind: KindT::Int(IntT { bits: 32 }),
                    ..
                },
                ..
            }
        ) => Some(())
    );
    assert_eq!(
        coutputs.lookup_lambda_in("main").header.return_type,
        CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: KindT::Int(IntT { bits: 32 }) },
    );
}
/*
  test("Lambda with one magic arg") {
    val compile =
      CompilerTestCompilation.test(
        """
          |exported func main() int { return {_}(3); }
          |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // Make sure it inferred the param type and return type correctly
    Collector.only(coutputs.lookupLambdaIn("main"),
      { case ParameterT(_, None, _, CoordT(ShareT, _, IntT.i32)) => })

    coutputs.lookupLambdaIn("main").header.returnType shouldEqual
      CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
  }
*/
// mig: fn lambda_is_reused
#[test]
fn lambda_is_reused() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nexported func main() {\n  lam = x => x;\n  lam(4);\n  lam(7);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambdas = coutputs.lookup_lambdas_in("main");
    assert_eq!(lambdas.len(), 1);
}
/*
  test("Lambda is reused") {
    // Since we call it with an int both times, the template generic should only generate one generic.

    val compile = CompilerTestCompilation.test(
      """
        |exported func main() {
        |  lam = x => x;
        |  lam(4);
        |  lam(7);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // There should be three lambdas generated
    // See GLIOGN
    val lambdas = coutputs.lookupLambdasIn("main")
    vassert(lambdas.size == 1)
  }
*/
// mig: fn lambda_called_with_different_types
#[test]
fn lambda_called_with_different_types() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nexported func main() {\n  lam = x => x;\n  lam(4);\n  lam(true);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambdas = coutputs.lookup_lambdas_in("main");
    assert_eq!(lambdas.len(), 2);
}
/*
  test("Lambda called with different types") {
    // Since we call it with an int both times, the template generic should only generate one generic.

    val compile = CompilerTestCompilation.test(
      """
        |exported func main() {
        |  lam = x => x;
        |  lam(4);
        |  lam(true);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // There should be three lambdas generated
    // See GLIOGN
    val lambdas = coutputs.lookupLambdasIn("main")
    vassert(lambdas.size == 2)
  }
*/
// mig: fn curried_lambda
#[test]
fn curried_lambda() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nexported func main() {\n  lam = x => y => 7;\n  lam(true)(4);\n  lam(true)(\"hello\");\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambdas = coutputs.lookup_lambdas_in("main");
    assert_eq!(lambdas.len(), 3);
}
/*
  test("Curried lambda") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func main() {
        |  lam = x => y => 7;
        |  lam(true)(4);
        |  lam(true)("hello");
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // There should be three lambdas generated:
    //  * lam(true)
    //  * lam(true)(4)
    //  * lam(true)("hello")
    // See GLIOGN
    val lambdas = coutputs.lookupLambdasIn("main")
    vassert(lambdas.size == 3)
  }


  // Test that the lambda's arg is the right type, and the name is right
*/
// mig: fn lambda_with_a_type_specified_param
#[test]
fn lambda_with_a_type_specified_param() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.arith.*;\nexported func main() int {\n  return (a int) => {+(a,a)}(3);\n}\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambda = coutputs.lookup_lambda_in("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(lambda),
        NodeRefT::Parameter(
            ParameterT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("a"), .. }),
                virtuality: None,
                tyype: CoordT {
                    ownership: OwnershipT::Share,
                    kind: KindT::Int(IntT { bits: 32 }),
                    ..
                },
                ..
            }
        ) => Some(())
    );
    assert!(coutputs.name_is_lambda_in(lambda.header.id, "main"));
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(FunctionCallTE { callable, .. }) => {
            assert!(coutputs.name_is_lambda_in(callable.id, "main"));
            Some(())
        }
    );
}
/*
  test("Lambda with a type specified param") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arith.*;
        |exported func main() int {
        |  return (a int) => {+(a,a)}(3);
        |}
        |""".stripMargin);
    val coutputs = compile.expectCompilerOutputs()

    val lambda = coutputs.lookupLambdaIn("main");

    // Check that the param type is right
    Collector.only(lambda, { case ParameterT(CodeVarNameT(StrI("a")), None, _, CoordT(ShareT, _, IntT.i32)) => {} })
    // Check the name is right
    vassert(coutputs.nameIsLambdaIn(lambda.header.id, "main"))

    val main = coutputs.lookupFunction("main");
    Collector.only(main, { case FunctionCallTE(callee, _, _) if coutputs.nameIsLambdaIn(callee.id, "main") => })
  }
*/
// mig: fn tests_lambda_and_concept_function
#[test]
fn tests_lambda_and_concept_function() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.print.*;\nimport v.builtins.drop.*;\nimport v.builtins.str.*;\n\nfunc moo<X, F>(x X, f F)\nwhere func(&F, &X)void, func drop(X)void, func drop(F)void {\n  f(&x);\n}\nexported func main() {\n  moo(\"hello\", { print(_); });\n}\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    compile.expect_compiler_outputs();
}
/*
  test("Tests lambda and concept function") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.print.*;
        |import v.builtins.drop.*;
        |import v.builtins.str.*;
        |
        |func moo<X, F>(x X, f F)
        |where func(&F, &X)void, func drop(X)void, func drop(F)void {
        |  f(&x);
        |}
        |exported func main() {
        |  moo("hello", { print(_); });
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn lambda_inside_different_function_with_same_name
#[test]
fn lambda_inside_different_function_with_same_name() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport printutils.*;\n\nfunc helperFunc(x int) {\n  { print(x); }();\n}\nfunc helperFunc(x str) {\n  { print(x); }();\n}\nexported func main() {\n  helperFunc(4);\n  helperFunc(\"bork\");\n}\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    compile.expect_compiler_outputs();
}
/*
  test("Lambda inside different function with same name") {
    // This originally didn't work because both helperFunc(:Int) and helperFunc(:Str)
    // made a closure struct called helperFunc:lam1, which collided.
    // We need to disambiguate by parameters, not just template args.

    val compile = CompilerTestCompilation.test(
      """
        |import printutils.*;
        |
        |func helperFunc(x int) {
        |  { print(x); }();
        |}
        |func helperFunc(x str) {
        |  { print(x); }();
        |}
        |exported func main() {
        |  helperFunc(4);
        |  helperFunc("bork");
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn lambda_inside_template
#[test]
fn lambda_inside_template() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.drop.*;\nimport printutils.*;\n\nfunc helperFunc<T>(x T)\nwhere func print(&T)void, func drop(T)void\n{\n  { print(x); }();\n}\nexported func main() {\n  helperFunc(4);\n  helperFunc(\"bork\");\n}\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    compile.expect_compiler_outputs();
}
/*
  test("Lambda inside template") {
    // This originally didn't work because both helperFunc<int> and helperFunc<Str>
    // made a closure struct called helperFunc:lam1, which collided.
    // This is what spurred paackage support.

    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |import printutils.*;
        |
        |func helperFunc<T>(x T)
        |where func print(&T)void, func drop(T)void
        |{
        |  { print(x); }();
        |}
        |exported func main() {
        |  helperFunc(4);
        |  helperFunc("bork");
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn curried_lambda_inside_template
#[test]
fn curried_lambda_inside_template() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "import v.builtins.drop.*;\nfunc helper<T>(x &T) &T {\n  lam = a => b => x;\n  return lam(true)(7);\n}\nexported func main() {\n  helper(4);\n  helper(\"bork\");\n}\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let lambdas = coutputs.lookup_lambdas_in("helper");
    assert_eq!(lambdas.len(), 2);
}
/*
  test("Curried lambda inside template") {
    val compile = CompilerTestCompilation.test(
      """import v.builtins.drop.*;
        |func helper<T>(x &T) &T {
        |  lam = a => b => x;
        |  return lam(true)(7);
        |}
        |exported func main() {
        |  helper(4);
        |  helper("bork");
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // There should be two lambdas generated:
    //  * helper<helper$0>.lam:3:9.__call{true}
    //  * helper<helper$0>.lam:3:9.__call{true}.lam:3:14.__call{int}
    // See GLIOGN
    val lambdas = coutputs.lookupLambdasIn("helper")
    vassert(lambdas.size == 2)
  }

}
*/
