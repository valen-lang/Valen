/*
package dev.vale.typing

import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.infer.{KindIsNotConcrete, OwnershipDidntMatch}
import dev.vale._
import dev.vale.parsing.ParseErrorHumanizer
import dev.vale.postparsing.PostParser
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.highertyping.{FunctionA, HigherTypingCompilation}
import dev.vale.solver.RuleError
import OverloadResolver._
import dev.vale.Collector.ProgramWithExpect
import dev.vale.parsing.ast.DontCallMacroP
import dev.vale.postparsing._
import dev.vale.postparsing.rules.IRulexSR
import dev.vale.solver.{FailedSolve, RuleError, Step}
import dev.vale.typing.ast._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.ast._
//import dev.vale.typingpass.infer.NotEnoughToSolveError
import org.scalatest._

import scala.collection.immutable.List
import scala.io.Source
*/
use super::compiler_test_compilation::compiler_test_compilation;
use bumpalo::Bump;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::utils::code_hierarchy::{self, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use crate::typing::types::types::{CoordT, IntT, IRegionT, KindT, OwnershipT, RegionT};
use crate::typing::ast::ast::ParameterT;
use crate::typing::ast::expressions::{LetNormalTE, LocalLookupTE};
use crate::typing::env::function_environment_t::{ILocalVariableT, ReferenceLocalVariableT};
use crate::typing::names::names::{INameT, IVarNameT};
use crate::typing::types::types::{MutabilityT, NeverT};
use crate::typing::templata::templata::{ITemplataT, KindTemplataT, MutabilityTemplataT};
use crate::interner::StrI;
use crate::parsing::tests::utils::expect_1;
use crate::postparsing::names::{CodeNameS, CodeRuneS, FunctionNameS, IFunctionDeclarationNameS, IImpreciseNameS, IImpreciseNameValS, INameS, IRuneValS, TopLevelStructDeclarationNameS};
use crate::solver::solver::{FailedSolve, ISolverError, RuleError, Step};
use crate::typing::ast::ast::{KindExportT, SignatureValT};
use crate::typing::compiler_error_humanizer::humanize;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::infer::compiler_solver::ITypingPassSolverError;
use crate::typing::names::names::{CodeVarNameT, ExportNameT, ExportTemplateNameT, FunctionNameValT, FunctionTemplateNameT, IdT, IdValT, IStructTemplateNameT, InterfaceNameValT, InterfaceTemplateNameT, StructNameT, StructNameValT, StructTemplateNameT};
use crate::typing::overload_resolver::FindFunctionFailure;
use crate::typing::templata::templata_utils::unapply_simple_name;
use crate::typing::types::types::{BoolT, InterfaceTTValT, StructTT, StructTTValT};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::utils::source_code_utils::{humanize_pos_code_map, line_containing, line_range_containing, lines_between};
use std::collections::HashSet;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::ast::expressions::ConstantIntTE;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::ast::expressions::ReferenceExpressionTE;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::names::names::FunctionNameT;
use crate::typing::ast::citizens::IStructMemberT;
use crate::typing::ast::citizens::NormalStructMemberT;
use crate::typing::types::types::VariabilityT;
use crate::typing::ast::citizens::IMemberTypeT;
use crate::typing::ast::citizens::ReferenceMemberTypeT;
use crate::typing::ast::expressions::ReferenceMemberLookupTE;
use crate::typing::templata::templata::CoordTemplataT;
use crate::typing::types::types::KindPlaceholderT;
use crate::typing::names::names::KindPlaceholderNameT;
use crate::typing::names::names::KindPlaceholderTemplateNameT;
use crate::postparsing::names::IRuneS;
use crate::typing::ast::expressions::UpcastTE;
use crate::typing::names::names::InterfaceNameT;
use crate::typing::types::types::ISuperKindTT;
use crate::typing::types::types::InterfaceTT;
use crate::typing::ast::expressions::SoftLoadTE;
use crate::typing::ast::expressions::AddressExpressionTE;
use crate::typing::ast::expressions::LetAndLendTE;
use crate::typing::ast::citizens::StructDefinitionT;
use crate::typing::ast::ast::FunctionHeaderT;
// mig: struct CompilerTests
pub struct CompilerTests {}
// mig: impl CompilerTests
impl CompilerTests {}
/*
class CompilerTests extends FunSuite with Matchers {
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
// mig: fn simple_program_returning_an_int_explicit
#[test]
fn simple_program_returning_an_int_explicit() {
    // We had a bug once looking up "int" in the environment, hence this test.
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "func main() int { return 3; }";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    assert!(main.header.return_type.kind == KindT::Int(IntT { bits: 32 }));
}
/*
  test("Simple program returning an int, explicit") {
    // We had a bug once looking up "int" in the environment, hence this test.

    val compile = CompilerTestCompilation.test(
      """
        |func main() int { return 3; }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupFunction("main")
    main.header.returnType.kind shouldEqual IntT(32)
  }

*/
// mig: fn hardcoding_negative_numbers
#[test]
fn hardcoding_negative_numbers() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main() int { return -3; }";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ConstantInt(
            ConstantIntTE {
                value: ITemplataT::Integer(-3),
                ..
            }
        ) => Some(())
    );
}
/*
  test("Hardcoding negative numbers") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func main() int { return -3; }
        |""".stripMargin)
    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case ConstantIntTE(IntegerTemplataT(-3), _, _) => true })
  }

*/
// mig: fn simple_local
#[test]
fn simple_local() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main() int {\n  a = 42;\n  return a;\n}";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    assert!(main.header.return_type.kind == KindT::Int(IntT { bits: 32 }));
}
/*
  test("Simple local") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func main() int {
        |  a = 42;
        |  return a;
        |}
    """.stripMargin)
    val main = compile.expectCompilerOutputs().lookupFunction("main")
    vassert(main.header.returnType.kind == IntT(32))
  }

*/
// mig: fn tests_panic_return_type
#[test]
fn tests_panic_return_type() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "import v.builtins.panic.*;\nexported func main() int {\n  x = { __vbi_panic() }();\n}";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::LetNormal(LetNormalTE {
            variable: ILocalVariableT::Reference(ReferenceLocalVariableT {
                coord: CoordT { ownership: OwnershipT::Share, kind: KindT::Never(NeverT { from_break: false }), .. },
                ..
            }),
            ..
        }) => Some(())
    );
}
/*
  test("Tests panic return type") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.panic.*;
        |exported func main() int {
        |  x = { __vbi_panic() }();
        |}
        """.stripMargin)
    val main = compile.expectCompilerOutputs().lookupFunction("main")
    main shouldHave {
      case LetNormalTE(
        ReferenceLocalVariableT(_,_,CoordT(ShareT,_,NeverT(false))),
        _) =>
    }
  }

*/
// mig: fn taking_an_argument_and_returning_it
#[test]
fn taking_an_argument_and_returning_it() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "func main(a int) int { return a; }";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");

    let param: &ParameterT = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Parameter(p) => Some(p)
    );
    assert!(param.tyype == CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: KindT::Int(IntT { bits: 32 }) });

    let lookup: &LocalLookupTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::LocalLookup(l) => Some(l)
    );
    match lookup.local_variable.name() {
        IVarNameT::CodeVar(c) => assert!(c.name.as_str() == "a"),
        _ => panic!("Expected CodeVarNameT"),
    }
    match lookup.local_variable.coord() {
        CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT { bits: 32 }), .. } => {}
        other => panic!("Expected CoordT(Share, _, Int(32)), got {:?}", other),
    }
}
/*
  test("Taking an argument and returning it") {
    val compile = CompilerTestCompilation.test(
      """
        |func main(a int) int { return a; }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    Collector.onlyOf(coutputs.lookupFunction("main"), classOf[ParameterT]).tyype == CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
    val lookup = Collector.onlyOf(coutputs.lookupFunction("main"), classOf[LocalLookupTE]);
    lookup.localVariable.name match { case CodeVarNameT(StrI("a")) => }
    lookup.localVariable.coord match { case CoordT(ShareT, _, IntT.i32) => }
  }

*/
// mig: fn tests_adding_two_numbers
#[test]
fn tests_adding_two_numbers() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "import v.builtins.arith.*;\nexported func main() int { return +(2, 3); }";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ConstantInt(
            ConstantIntTE {
                value: ITemplataT::Integer(2),
                ..
            }
        ) => Some(())
    );

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ConstantInt(
            ConstantIntTE {
                value: ITemplataT::Integer(3),
                ..
            }
        ) => Some(())
    );

    let func_call: &FunctionCallTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(call) => Some(call)
    );

    match func_call.callable.id.local_name {
        INameT::Function(fname) => {
            assert!(fname.template.human_name.as_str() == "+");
        }
        _ => panic!("Expected function name for + operator"),
    }

    assert_eq!(func_call.args.len(), 2);
    match (&func_call.args[0], &func_call.args[1]) {
        (
            ReferenceExpressionTE::ConstantInt(c1),
            ReferenceExpressionTE::ConstantInt(c2)
        ) => {
            match (&c1.value, &c2.value) {
                (
                    ITemplataT::Integer(2),
                    ITemplataT::Integer(3)
                ) => {}
                _ => panic!("Expected ConstantInt(2) and ConstantInt(3)"),
            }
        }
        _ => panic!("Expected function call with ConstantInt arguments"),
    }
}
/*
  test("Tests adding two numbers") {
    val compile =
      CompilerTestCompilation.test(
        """
          |import v.builtins.arith.*;
          |exported func main() int { return +(2, 3); }
          |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, { case ConstantIntTE(IntegerTemplataT(2), _, _) => true })
    Collector.only(main, { case ConstantIntTE(IntegerTemplataT(3), _, _) => true })
    Collector.only(main, {
      case FunctionCallTE(
        functionNameT("+"),
        Vector(
          ConstantIntTE(IntegerTemplataT(2), _, _),
          ConstantIntTE(IntegerTemplataT(3), _, _)),
        _) =>
    })
  }

*/
// mig: fn simple_struct_read
#[test]
fn simple_struct_read() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported struct Moo { hp int; }\nexported func main(moo &Moo) int {\n  return moo.hp;\n}";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
}
/*
  test("Simple struct read") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct Moo { hp int; }
        |exported func main(moo &Moo) int {
        |  return moo.hp;
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
  }

*/
// mig: fn make_array_and_dot_it
#[test]
fn make_array_and_dot_it() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r#"
exported func main() int {
  arr = [#]int(6, 60, 103);
  x = arr.2;
  [_, _, _] = arr;
  return x;
}
"#;
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Make array and dot it") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func main() int {
        |  arr = [#]int(6, 60, 103);
        |  x = arr.2;
        |  [_, _, _] = arr;
        |  return x;
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }

*/
// mig: fn simple_struct_instantiate
#[test]
fn simple_struct_instantiate() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r#"
exported struct Moo { hp int; }
exported func main() Moo {
  return Moo(42);
}
"#;
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let _main = coutputs.lookup_function_by_str("main");
}
/*
  test("Simple struct instantiate") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct Moo { hp int; }
        |exported func main() Moo {
        |  return Moo(42);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
  }

*/
// mig: fn call_destructor
#[test]
fn call_destructor() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r#"
exported struct Moo { hp int; }
exported func main() int {
  return Moo(42).hp;
}
"#;
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let _drop_call: &FunctionCallTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(call @ FunctionCallTE {
            callable: PrototypeT {
                id: IdT {
                    local_name: INameT::Function(FunctionNameT {
                        template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                        ..
                    }),
                    ..
                },
                ..
            },
            ..
        }) => Some(call)
    );
}
/*
  test("Call destructor") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct Moo { hp int; }
        |exported func main() int {
        |  return Moo(42).hp;
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case FunctionCallTE(PrototypeT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, _)), _), _, _) =>
    })
  }

*/
// mig: fn custom_destructor
#[test]
fn custom_destructor() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "#!DeriveStructDrop\n",
        "exported struct Moo { hp int; }\n",
        "func drop(self ^Moo) {\n",
        "  [_] = self;\n",
        "}\n",
        "exported func main() int {\n",
        "  return Moo(42).hp;\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(
            FunctionCallTE {
                callable: PrototypeT {
                    id: IdT {
                        local_name: INameT::Function(FunctionNameT {
                            template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                            ..
                        }),
                        ..
                    },
                    ..
                },
                ..
            }
        ) => Some(())
    );
}
/*
  test("Custom destructor") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveStructDrop
        |exported struct Moo { hp int; }
        |func drop(self ^Moo) {
        |  [_] = self;
        |}
        |exported func main() int {
        |  return Moo(42).hp;
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case FunctionCallTE(PrototypeT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, _)), _), _, _) =>
    })
  }

*/
// mig: fn make_constraint_reference
#[test]
fn make_constraint_reference() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r#"
struct Moo {}
exported func main() void {
  m = Moo();
  b = &m;
}
"#;
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let let_normal: &LetNormalTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::LetNormal(ln @ LetNormalTE {
            variable: ILocalVariableT::Reference(ReferenceLocalVariableT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("b"), .. }),
                ..
            }),
            ..
        }) => Some(ln)
    );
    assert_eq!(let_normal.variable.coord().ownership, OwnershipT::Borrow);
}
/*
  test("Make constraint reference") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Moo {}
        |exported func main() void {
        |  m = Moo();
        |  b = &m;
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    val tyype =
      Collector.only(main.body, {
        case LetNormalTE(ReferenceLocalVariableT(CodeVarNameT(StrI("b")), _, tyype), _) => tyype
      })
    tyype.ownership shouldEqual BorrowT
  }



*/
// mig: fn recursion
#[test]
fn recursion() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main() int { return main(); }";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    // Make sure it inferred the param type and return type correctly
    assert!(coutputs.lookup_function_by_str("main").header.return_type == CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: KindT::Int(IntT { bits: 32 }) });
}
/*
  test("Recursion") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func main() int { return main(); }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // Make sure it inferred the param type and return type correctly
    coutputs.lookupFunction("main").header.returnType shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
  }

*/
// mig: fn test_overloads
#[test]
fn test_overloads() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = crate::tests::tests::load_expected("programs/functions/overloads.vale");
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    assert!(matches!(coutputs.lookup_function_by_str("main").header.return_type,
        CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT { bits: 32 }), .. }
    ));
}
/*
  test("Test overloads") {
    val compile = CompilerTestCompilation.test(Tests.loadExpected("programs/functions/overloads.vale"))
    val coutputs = compile.expectCompilerOutputs()

    coutputs.lookupFunction("main").header.returnType shouldEqual
      CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
  }

*/
// mig: fn test_readonly_ufcs
#[test]
fn test_readonly_ufcs() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = crate::tests::tests::load_expected("programs/ufcs.vale");
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    compile.expect_compiler_outputs();
}
/*
  test("Test readonly UFCS") {
    val compile = CompilerTestCompilation.test(Tests.loadExpected("programs/ufcs.vale"))
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn test_readwrite_ufcs
#[test]
fn test_readwrite_ufcs() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = crate::tests::tests::load_expected("programs/readwriteufcs.vale");
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    compile.expect_compiler_outputs();
}
/*
  test("Test readwrite UFCS") {
    val compile = CompilerTestCompilation.test(Tests.loadExpected("programs/readwriteufcs.vale"))
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn test_templates
#[test]
fn test_templates() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "func bork<T>(a T) T { return a; }\n",
        "exported func main() int { bork(true); bork(2); bork(3) }\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    // Tests that there's only two functions, because we have generics not templates
    assert!(coutputs.get_all_user_functions().len() == 2);
}
/*
  test("Test templates") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<T>(a T) T { return a; }
        |exported func main() int { bork(true); bork(2); bork(3) }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // Tests that there's only two functions, because we have generics not templates
    vassert(coutputs.getAllUserFunctions.size == 2)
  }

*/
// mig: fn test_taking_a_callable_param
#[test]
fn test_taking_a_callable_param() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "func do<F>(callable F) int\n",
        "where func(&F)int, func drop(F)void\n",
        "{\n",
        "  return callable();\n",
        "}\n",
        "exported func main() int { return do({ return 3; }); }\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let do_fn = coutputs.lookup_function_by_str("do");
    assert!(matches!(do_fn.header.return_type,
        CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT { bits: 32 }), .. }
    ));
}
/*
  test("Test taking a callable param") {
    val compile = CompilerTestCompilation.test(
      """
        |func do<F>(callable F) int
        |where func(&F)int, func drop(F)void
        |{
        |  return callable();
        |}
        |exported func main() int { return do({ return 3; }); }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    coutputs.functions.collect({ case x @ functionNameT("do") => x }).head.header.returnType shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
  }

*/
// mig: fn simple_struct
#[test]
fn simple_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "#!DeriveStructDrop\n",
        "struct MyStruct { a int; }\n",
        "exported func main() {\n",
        "  ms = MyStruct(7);\n",
        "  [_] = ms;\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    // Check the struct was made
    coutputs.structs.iter().find(|def| matches!(def,
        StructDefinitionT {
            template_name: IdT {
                local_name: INameT::StructTemplate(StructTemplateNameT { human_name: StrI("MyStruct"), .. }),
                ..
            },
            instantiated_citizen: StructTT {
                id: IdT {
                    local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(
                            StructTemplateNameT { human_name: StrI("MyStruct"), .. }
                        ),
                        ..
                    }),
                    ..
                },
                ..
            },
            weakable: false,
            mutability: ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }),
            members: [IStructMemberT::Normal(NormalStructMemberT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("a"), .. }),
                variability: VariabilityT::Final,
                tyype: IMemberTypeT::Reference(ReferenceMemberTypeT {
                    reference: CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT { bits: 32 }), .. },
                }),
            })],
            is_closure: false,
            ..
        }
    )).unwrap();
    // Check there's a constructor
    let _ = crate::collect_where_tnode!(
        NodeRefT::FunctionDefinition(coutputs.lookup_function_by_str("MyStruct")),
        NodeRefT::FunctionHeader(h @ FunctionHeaderT {
            id: IdT {
                local_name: INameT::Function(FunctionNameT {
                    template: FunctionTemplateNameT { human_name: StrI("MyStruct"), .. },
                    ..
                }),
                ..
            },
            params: [ParameterT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("a"), .. }),
                virtuality: None,
                tyype: CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT { bits: 32 }), .. },
                ..
            }],
            return_type: CoordT {
                ownership: OwnershipT::Own,
                kind: KindT::Struct(StructTT {
                    id: IdT {
                        local_name: INameT::Struct(StructNameT {
                            template: IStructTemplateNameT::StructTemplate(
                                StructTemplateNameT { human_name: StrI("MyStruct"), .. }
                            ),
                            ..
                        }),
                        ..
                    },
                    ..
                }),
                ..
            },
            ..
        }) => Some(h)
    );
    let main = coutputs.lookup_function_by_str("main");
    // Check that we call the constructor
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(
            FunctionCallTE {
                callable: PrototypeT {
                    id: IdT {
                        local_name: INameT::Function(FunctionNameT {
                            template: FunctionTemplateNameT { human_name: StrI("MyStruct"), .. },
                            ..
                        }),
                        ..
                    },
                    ..
                },
                args: [ReferenceExpressionTE::ConstantInt(
                    ConstantIntTE {
                        value: ITemplataT::Integer(7),
                        ..
                    }
                )],
                ..
            }
        ) => Some(())
    );
}
/*
  test("Simple struct") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveStructDrop
        |struct MyStruct { a int; }
        |exported func main() {
        |  ms = MyStruct(7);
        |  [_] = ms;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // Check the struct was made
    coutputs.structs.collectFirst({
      case StructDefinitionT(
        simpleNameT("MyStruct"),
        StructTT(simpleNameT("MyStruct")),
        _,
        false,
        MutabilityTemplataT(MutableT),
        Vector(NormalStructMemberT(CodeVarNameT(StrI("a")), FinalT, ReferenceMemberTypeT((CoordT(ShareT, _,IntT.i32))))),
        false,
        _) =>
    }).get
    // Check there's a constructor
    Collector.all(coutputs.lookupFunction("MyStruct"), {
      case FunctionHeaderT(
        simpleNameT("MyStruct"),
        _,
        Vector(ParameterT(CodeVarNameT(StrI("a")), None, _, CoordT(ShareT, _,IntT.i32))),
        CoordT(OwnT, _,StructTT(simpleNameT("MyStruct"))),
        _) =>
    })
    val main = coutputs.lookupFunction("main")
    // Check that we call the constructor
    Collector.only(main, {
      case FunctionCallTE(
        PrototypeT(simpleNameT("MyStruct"), _),
        Vector(ConstantIntTE(IntegerTemplataT(7), _, _)),
        _) =>
    })
  }

*/
// mig: fn calls_destructor_on_local_var
#[test]
fn calls_destructor_on_local_var() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct Muta { }\n",
        "func destructor(m ^Muta) {\n",
        "  Muta[ ] = m;\n",
        "}\n",
        "exported func main() {\n",
        "  a = Muta();\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(
            FunctionCallTE {
                callable: PrototypeT {
                    id: IdT {
                        local_name: INameT::Function(FunctionNameT {
                            template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                            ..
                        }),
                        ..
                    },
                    ..
                },
                ..
            }
        ) => Some(())
    );
    let all_calls = crate::collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(_fpc) => Some(())
    );
    assert_eq!(all_calls.len(), 2);
}
/*
  test("Calls destructor on local var") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Muta { }
        |
        |func destructor(m ^Muta) {
        |  Muta[ ] = m;
        |}
        |
        |exported func main() {
        |  a = Muta();
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(PrototypeT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, _)), _), _, _) => })
    Collector.all(main, { case FunctionCallTE(_, _, _) => }).size shouldEqual 2
  }

*/
// mig: fn tests_defining_an_empty_interface_and_an_implementing_struct
#[test]
fn tests_defining_an_empty_interface_and_an_implementing_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "sealed interface MyInterface { }\n",
        "struct MyStruct { }\n",
        "impl MyInterface for MyStruct;\n",
        "func main(a MyStruct) {}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();


    let interfaces_matching: Vec<_> = coutputs.interfaces.iter()
        .filter(|d| unapply_simple_name(&d.template_name).as_deref() == Some("MyInterface")
            && !d.weakable
            && matches!(d.mutability, ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }))
            && d.internal_methods.is_empty())
        .collect();
    let interface_def = expect_1(&interfaces_matching);

    let structs_matching: Vec<_> = coutputs.structs.iter()
        .filter(|d| unapply_simple_name(&d.template_name).as_deref() == Some("MyStruct")
            && !d.weakable
            && matches!(d.mutability, ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }))
            && !d.is_closure)
        .collect();
    let struct_def = expect_1(&structs_matching);

    assert!(coutputs.interface_to_sub_citizen_to_edge.iter()
        .flat_map(|(_, sub_map)| sub_map.values())
        .any(|edge| {
            edge.sub_citizen.id() == struct_def.instantiated_citizen.id &&
            edge.super_interface == interface_def.instantiated_interface.id
        }));
}
/*
  test("Tests defining an empty interface and an implementing struct") {
    val compile = CompilerTestCompilation.test(
      """
        |sealed interface MyInterface { }
        |struct MyStruct { }
        |impl MyInterface for MyStruct;
        |func main(a MyStruct) {}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val interfaceDef =
      vassertOne(coutputs.interfaces.collectFirst({
        case id @ InterfaceDefinitionT(simpleNameT("MyInterface"), _, _, _, false, MutabilityTemplataT(MutableT), _, Vector()) => id
      }))

    val structDef =
      vassertOne(coutputs.structs.collectFirst({
        case sd @ StructDefinitionT(simpleNameT("MyStruct"), _, _, false, MutabilityTemplataT(MutableT), _, false, _) => sd
      }))

    vassert(coutputs.interfaceToSubCitizenToEdge.flatMap(_._2.values).exists(impl => {
      impl.subCitizen.id == structDef.instantiatedCitizen.id &&
        impl.superInterface == interfaceDef.instantiatedCitizen.id
    }))
  }

*/
// mig: fn tests_defining_a_non_empty_interface_and_an_implementing_struct
#[test]
fn tests_defining_a_non_empty_interface_and_an_implementing_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "exported sealed interface MyInterface {\n",
        "  func bork(virtual self &MyInterface);\n",
        "}\n",
        "exported struct MyStruct { }\n",
        "impl MyInterface for MyStruct;\n",
        "func bork(self &MyStruct) {}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();


    let interfaces_matching: Vec<_> = coutputs.interfaces.iter()
        .filter(|d| unapply_simple_name(&d.template_name).as_deref() == Some("MyInterface")
            && !d.weakable
            && matches!(d.mutability, ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable })))
        .collect();
    let interface_def = expect_1(&interfaces_matching);

    let bork_method = interface_def.internal_methods.iter()
        .find(|(proto, _)| unapply_simple_name(&proto.id).as_deref() == Some("bork"))
        .unwrap();
    let _ = bork_method;

    let structs_matching: Vec<_> = coutputs.structs.iter()
        .filter(|d| unapply_simple_name(&d.template_name).as_deref() == Some("MyStruct")
            && !d.weakable
            && matches!(d.mutability, ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }))
            && !d.is_closure)
        .collect();
    let struct_def = expect_1(&structs_matching);

    assert!(coutputs.interface_to_sub_citizen_to_edge.iter()
        .flat_map(|(_, sub_map)| sub_map.values())
        .any(|edge| {
            edge.sub_citizen.id() == struct_def.instantiated_citizen.id &&
            edge.super_interface == interface_def.instantiated_interface.id
        }));
}
/*
  test("Tests defining a non-empty interface and an implementing struct") {
    val compile = CompilerTestCompilation.test(
      """
        |exported sealed interface MyInterface {
        |  func bork(virtual self &MyInterface);
        |}
        |exported struct MyStruct { }
        |impl MyInterface for MyStruct;
        |func bork(self &MyStruct) {}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val (interfaceDef, methods) =
      vassertOne(coutputs.interfaces.collectFirst({
        case id @ InterfaceDefinitionT(simpleNameT("MyInterface"), _, _, _, false, MutabilityTemplataT(MutableT), _, methods) => (id, methods)
      }))
    vassertSome(methods.collectFirst({
      case (f @ PrototypeT(simpleNameT("bork"), _), _) => f
    }))

    val structDef =
      vassertOne(coutputs.structs.collectFirst({
        case sd @ StructDefinitionT(simpleNameT("MyStruct"), _, _, false, MutabilityTemplataT(MutableT), _, false, _) => sd
      }))

    vassert(coutputs.interfaceToSubCitizenToEdge.values.flatMap(_.values).exists(impl => {
      impl.subCitizen.id == structDef.instantiatedCitizen.id &&
        impl.superInterface == interfaceDef.instantiatedCitizen.id
    }))
  }

*/
// mig: fn stamps_an_interface_template_via_a_function_return
#[test]
fn stamps_an_interface_template_via_a_function_return() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.drop.*;\n",
        "\n",
        "sealed interface MyInterface<X Ref> where func drop(X)void { }\n",
        "\n",
        "struct SomeStruct<X Ref> where func drop(X)void { x X; }\n",
        "impl<X> MyInterface<X> for SomeStruct<X>;\n",
        "\n",
        "func doAThing<T>(t T) SomeStruct<T>\n",
        "where func drop(T)void {\n",
        "  return SomeStruct<T>(t);\n",
        "}\n",
        "\n",
        "exported func main() {\n",
        "  doAThing(4);\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Stamps an interface template via a function return") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |sealed interface MyInterface<X Ref> where func drop(X)void { }
        |
        |struct SomeStruct<X Ref> where func drop(X)void { x X; }
        |impl<X> MyInterface<X> for SomeStruct<X>;
        |
        |func doAThing<T>(t T) SomeStruct<T>
        |where func drop(T)void {
        |  return SomeStruct<T>(t);
        |}
        |
        |exported func main() {
        |  doAThing(4);
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
  }

//  test("Constructor is stamped even without calling") {
//    val compile = RunCompilation.test(
//      """
//        |struct MyStruct imm {}
//        |func wot(b: *MyStruct) int { return 9; }
//      """.stripMargin)
//    val coutputs = compile.expectCompilerOutputs()
//
//    coutputs.lookupFunction("MyStruct")
//  }

*/
// mig: fn reads_a_struct_member
#[test]
fn reads_a_struct_member() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "#!DeriveStructDrop\n",
        "struct MyStruct { a int; }\n",
        "exported func main() int {\n",
        "  ms = MyStruct(7);\n",
        "  x = ms.a;\n",
        "  [_] = ms;\n",
        "  return x;\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let main = coutputs.lookup_function_by_str("main");
    // check for the member access
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ReferenceMemberLookup(
            ReferenceMemberLookupTE {
                struct_expr: ReferenceExpressionTE::SoftLoad(SoftLoadTE { target_ownership: OwnershipT::Borrow, .. }),
                member_name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("a"), .. }),
                member_reference: CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT { bits: 32 }), .. },
                variability: VariabilityT::Final,
                ..
            }
        ) => Some(())
    );
}
/*
  test("Reads a struct member") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveStructDrop
        |struct MyStruct { a int; }
        |exported func main() int {
        |  ms = MyStruct(7);
        |  x = ms.a;
        |  [_] = ms;
        |  return x;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupFunction("main")
    // check for the member access
    main shouldHave {
      case ReferenceMemberLookupTE(_,
        SoftLoadTE(_,BorrowT),
        CodeVarNameT(StrI("a")),
        CoordT(ShareT,_,IntT(32)),
        FinalT) =>
    }
  }


*/
// mig: fn automatically_drops_struct
#[test]
fn automatically_drops_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct MyStruct { a int; }\n",
        "exported func main() int {\n",
        "  ms = MyStruct(7);\n",
        "  return ms.a;\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let main = coutputs.lookup_function_by_str("main");
    // check for the call to drop
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(
            FunctionCallTE {
                callable: PrototypeT {
                    id: IdT {
                        init_steps: [INameT::StructTemplate(StructTemplateNameT { human_name: StrI("MyStruct"), .. })],
                        local_name: INameT::Function(FunctionNameT {
                            template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                            template_args: &[],
                            parameters: [CoordT {
                                ownership: OwnershipT::Own,
                                kind: KindT::Struct(StructTT {
                                    id: IdT {
                                        local_name: INameT::Struct(StructNameT {
                                            template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("MyStruct"), .. }),
                                            template_args: &[],
                                            ..
                                        }),
                                        ..
                                    },
                                    ..
                                }),
                                ..
                            }],
                            ..
                        }),
                        ..
                    },
                    return_type: CoordT { ownership: OwnershipT::Share, kind: KindT::Void(_), .. },
                },
                ..
            }
        ) => Some(())
    );
}
/*
  test("Automatically drops struct") {
    val compile = CompilerTestCompilation.test(
      """
        |struct MyStruct { a int; }
        |exported func main() int {
        |  ms = MyStruct(7);
        |  return ms.a;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupFunction("main")
    // check for the call to drop
    main shouldHave {
      case FunctionCallTE(
        PrototypeT(
          IdT(_,
            Vector(StructTemplateNameT(StrI("MyStruct"))),
            FunctionNameT(
              FunctionTemplateNameT(StrI("drop"),_),
              Vector(),
              Vector(CoordT(OwnT,_,StructTT(IdT(_,_,StructNameT(StructTemplateNameT(StrI("MyStruct")),Vector()))))))),
          CoordT(ShareT,_,VoidT())), _, _) =>
    }
  }

*/
// mig: fn tests_stamping_an_interface_template_from_a_function_param
#[test]
fn tests_stamping_an_interface_template_from_a_function_param() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "interface MyOption<T Ref> { }\n",
        "func main(a &MyOption<int>) { }\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let interface_template_name = compile.typing_interner.intern_interface_template_name(
        InterfaceTemplateNameT {
            human_namee: scout_arena.intern_str("MyOption"),
            _phantom: std::marker::PhantomData,
        });
    let template_args_vec = vec![
        ITemplataT::Coord(
            compile.typing_interner.alloc(CoordTemplataT {
                coord: CoordT {
                    ownership: OwnershipT::Share,
                    region: RegionT { region: IRegionT::Default },
                    kind: KindT::Int(IntT { bits: 32 }),
                },
            })
        ),
    ];
    let interface_name = compile.typing_interner.intern_interface_name(
        InterfaceNameValT {
            template: interface_template_name,
            template_args: &template_args_vec,
        });
    let test_tld = scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]);
    let interface_id = compile.typing_interner.intern_id(
        IdValT {
            package_coord: test_tld,
            init_steps: &[],
            local_name: INameT::Interface(interface_name),
        });
    let interface_tt = compile.typing_interner.intern_interface_tt(
        InterfaceTTValT { id: *interface_id });
    let expected_coord = CoordT {
        ownership: OwnershipT::Borrow,
        region: RegionT { region: IRegionT::Default },
        kind: KindT::Interface(interface_tt),
    };

    let coutputs = compile.expect_compiler_outputs();
    coutputs.lookup_interface_by_template_name(interface_template_name);
    let main = coutputs.lookup_function_by_str("main");
    assert_eq!(main.header.params[0].tyype, expected_coord);
}
/*
  test("Tests stamping an interface template from a function param") {
    val compile = CompilerTestCompilation.test(
      """
        |interface MyOption<T Ref> { }
        |func main(a &MyOption<int>) { }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val interner = compile.interner
    val keywords = compile.keywords

    coutputs.lookupInterfaceByTemplateName(
      interner.intern(
        InterfaceTemplateNameT(interner.intern(StrI("MyOption")))))
    coutputs.lookupFunction("main").header.params.head.tyype shouldEqual
        CoordT(
          BorrowT,
          RegionT(DefaultRegionT),
          interner.intern(
            InterfaceTT(IdT(PackageCoordinate.TEST_TLD(interner, keywords), Vector(), interner.intern(InterfaceNameT(interner.intern(InterfaceTemplateNameT(interner.intern(StrI("MyOption")))), Vector(CoordTemplataT(CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)))))))))

    // Can't run it because there's nothing implementing that interface >_>
  }

*/
// mig: fn reports_mismatched_return_type_when_expecting_void
#[test]
fn reports_mismatched_return_type_when_expecting_void() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main() { 73 }\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::BodyResultDoesntMatch { function_name, expected_return_type, result_type, .. } => {
            match function_name {
                IFunctionDeclarationNameS::FunctionName(fn_name) => assert_eq!(fn_name.name.as_str(), "main"),
                other => panic!("expected FunctionName: {:?}", other),
            }
            assert_eq!(expected_return_type.ownership, OwnershipT::Share);
            match expected_return_type.kind {
                KindT::Void(_) => {}
                other => panic!("expected VoidT: {:?}", other),
            }
            assert_eq!(result_type.ownership, OwnershipT::Share);
            match result_type.kind {
                KindT::Int(_) => {}
                other => panic!("expected IntT: {:?}", other),
            }
        }
        _other => panic!("expected BodyResultDoesntMatch"),
    }
}
/*
  test("Reports mismatched return type when expecting void") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func main() { 73 }
        |""".stripMargin)
    compile.getCompilerOutputs().expectErr() match {
      case BodyResultDoesntMatch(_,
        FunctionNameS(StrI("main"),_),
        CoordT(ShareT,_,VoidT()),
        CoordT(ShareT,_,IntT(_))) =>
    }
  }

*/
// mig: fn tests_exporting_function
#[test]
fn tests_exporting_function() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func moo() { }\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let moo = coutputs.lookup_function_by_str("moo");
    let export = expect_1(&coutputs.function_exports);
    assert_eq!(export.prototype, moo.header.to_prototype());
}
/*
  test("Tests exporting function") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func moo() { }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupFunction("moo")
    val export = vassertOne(coutputs.functionExports)
    `export`.prototype shouldEqual moo.header.toPrototype
  }

*/
// mig: fn tests_exporting_struct
#[test]
fn tests_exporting_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported struct Moo { a int; }\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let moo = coutputs.lookup_struct_by_str("Moo");
    let export = expect_1(&coutputs.kind_exports);
    assert_eq!(export.tyype, KindT::from(&moo.instantiated_citizen));
}
/*
  test("Tests exporting struct") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct Moo { a int; }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupStruct("Moo")
    val export = vassertOne(coutputs.kindExports)
    `export`.tyype shouldEqual moo.instantiatedCitizen
  }

*/
// mig: fn tests_exporting_interface
#[test]
fn tests_exporting_interface() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported sealed interface IMoo { func hi(virtual this &IMoo) void; }\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let moo = coutputs.lookup_interface_by_human_name("IMoo");
    let export = expect_1(&coutputs.kind_exports);
    assert_eq!(export.tyype, KindT::from(&moo.instantiated_interface));
}
/*
  test("Tests exporting interface") {
    val compile = CompilerTestCompilation.test(
      """
        |exported sealed interface IMoo { func hi(virtual this &IMoo) void; }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupInterface("IMoo")
    val export = vassertOne(coutputs.kindExports)
    `export`.tyype shouldEqual moo.instantiatedInterface
  }

*/
// mig: fn tests_single_expression_and_single_statement_functions_returns
#[test]
fn tests_single_expression_and_single_statement_functions_returns() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct MyThing { value int; }\n",
        "func moo() MyThing { return MyThing(4); }\n",
        "exported func main() { moo(); }\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let moo = coutputs.lookup_function_by_str("moo");
    match moo.header.return_type {
        CoordT {
            ownership: OwnershipT::Own,
            kind: KindT::Struct(StructTT {
                id: IdT {
                    init_steps: &[],
                    local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("MyThing"), .. }),
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        } => {}
        other => panic!("moo.header.returnType: {:?}", other),
    }
    let main = coutputs.lookup_function_by_str("main");
    match main.header.return_type {
        CoordT { ownership: OwnershipT::Share, kind: KindT::Void(_), .. } => {}
        other => panic!("main.header.returnType: {:?}", other),
    }
}
/*
  test("Tests single expression and single statement functions' returns") {
    val compile = CompilerTestCompilation.test(
      """
        |struct MyThing { value int; }
        |func moo() MyThing { return MyThing(4); }
        |exported func main() { moo(); }
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupFunction("moo")
    moo.header.returnType match {
      case CoordT(OwnT,_, StructTT(simpleNameT("MyThing"))) =>
    }
    val main = coutputs.lookupFunction("main")
    main.header.returnType match {
      case CoordT(ShareT, _, VoidT()) =>
    }
  }

*/
// mig: fn tests_calling_a_templated_struct_s_constructor
#[test]
fn tests_calling_a_templated_struct_s_constructor() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.drop.*;\n",
        "struct MySome<T Ref> where func drop(T)void { value T; }\n",
        "exported func main() int {\n",
        "  return MySome<int>(4).value;\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    coutputs.lookup_struct_by_template_name(
        StructTemplateNameT {
            human_name: scout_arena.intern_str("MySome"),
            _phantom: std::marker::PhantomData,
        });

    let constructor = coutputs.lookup_function_by_str("MySome");
    match constructor.header {
        FunctionHeaderT {
            id: IdT {
                local_name: INameT::Function(FunctionNameT {
                    template: FunctionTemplateNameT { human_name: StrI("MySome"), .. },
                    template_args: [ITemplataT::Coord(CoordTemplataT { coord: CoordT {
                        ownership: OwnershipT::Own,
                        kind: KindT::KindPlaceholder(KindPlaceholderT {
                            id: IdT {
                                local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                                    template: KindPlaceholderTemplateNameT {
                                        index: 0,
                                        rune: IRuneS::CodeRune(CodeRuneS { name: StrI("T") }),
                                        ..
                                    },
                                }),
                                ..
                            },
                            ..
                        }),
                        ..
                    } })],
                    parameters: [CoordT {
                        ownership: OwnershipT::Own,
                        kind: KindT::KindPlaceholder(KindPlaceholderT {
                            id: IdT {
                                local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                                    template: KindPlaceholderTemplateNameT { index: 0, .. },
                                }),
                                ..
                            },
                            ..
                        }),
                        ..
                    }],
                    ..
                }),
                ..
            },
            attributes: &[],
            params: [ParameterT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("value"), .. }),
                virtuality: None,
                tyype: CoordT {
                    ownership: OwnershipT::Own,
                    kind: KindT::KindPlaceholder(KindPlaceholderT {
                        id: IdT {
                            local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                                template: KindPlaceholderTemplateNameT { index: 0, .. },
                            }),
                            ..
                        },
                        ..
                    }),
                    ..
                },
                ..
            }],
            return_type: CoordT {
                ownership: OwnershipT::Own,
                kind: KindT::Struct(StructTT {
                    id: IdT {
                        local_name: INameT::Struct(StructNameT {
                            template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("MySome"), .. }),
                            template_args: [ITemplataT::Coord(CoordTemplataT { coord: CoordT {
                                ownership: OwnershipT::Own,
                                kind: KindT::KindPlaceholder(KindPlaceholderT {
                                    id: IdT {
                                        local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                                            template: KindPlaceholderTemplateNameT { index: 0, .. },
                                        }),
                                        ..
                                    },
                                    ..
                                }),
                                ..
                            } })],
                            ..
                        }),
                        ..
                    },
                    ..
                }),
                ..
            },
            maybe_origin_function_templata: Some(_),
            ..
        } => {}
        other => panic!("constructor.header: {:?}", other),
    }

    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(
            FunctionCallTE {
                callable: PrototypeT {
                    id: IdT {
                        local_name: INameT::Function(FunctionNameT {
                            template: FunctionTemplateNameT { human_name: StrI("MySome"), .. },
                            ..
                        }),
                        ..
                    },
                    ..
                },
                ..
            }
        ) => Some(())
    );
}
/*
  test("Tests calling a templated struct's constructor") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |struct MySome<T Ref> where func drop(T)void { value T; }
        |exported func main() int {
        |  return MySome<int>(4).value;
        |}
        |""".stripMargin
    )

    val coutputs = compile.expectCompilerOutputs()
    val interner = compile.interner
    val keywords = compile.keywords

    coutputs.lookupStructByTemplateName(
      interner.intern(StructTemplateNameT(interner.intern(StrI("MySome")))))

    val constructor = coutputs.lookupFunction("MySome")
    constructor.header match {
      case FunctionHeaderT(
        IdT(_,
          _,
          FunctionNameT(
            FunctionTemplateNameT(StrI("MySome"), _),
            Vector(CoordTemplataT(CoordT(OwnT, _,KindPlaceholderT(IdT(_,_,KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, CodeRuneS(StrI("T"))))))))),
            Vector(CoordT(OwnT,_,KindPlaceholderT(IdT(_,_,KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))))))),
        Vector(),
        Vector(
          ParameterT(
            CodeVarNameT(StrI("value")),
            None,
            _,
            CoordT(OwnT,_,KindPlaceholderT(IdT(_,_,KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _))))))),
        CoordT(
          OwnT,
          _,
          StructTT(
            IdT(_,
              _,
              StructNameT(
                StructTemplateNameT(StrI("MySome")),
                Vector(
                  CoordTemplataT(CoordT(OwnT, _,KindPlaceholderT(IdT(_,_,KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _))))))))))),
        Some(_)) =>
    }

    Collector.all(coutputs.lookupFunction("main"), {
      case FunctionCallTE(functionNameT("MySome"), _, _) =>
    })
  }

*/
// mig: fn tests_upcasting_from_a_struct_to_an_interface
#[test]
fn tests_upcasting_from_a_struct_to_an_interface() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = include_str!("../../tests/programs/virtuals/upcasting.vale");
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let main = coutputs.lookup_function_by_str("main");

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::LetNormal(LetNormalTE {
            variable: ILocalVariableT::Reference(ReferenceLocalVariableT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("x"), .. }),
                variability: VariabilityT::Final,
                coord: CoordT {
                    ownership: OwnershipT::Own,
                    kind: KindT::Interface(InterfaceTT {
                        id: IdT {
                            local_name: INameT::Interface(InterfaceNameT {
                                template: InterfaceTemplateNameT { human_namee: StrI("MyInterface"), .. },
                                ..
                            }),
                            ..
                        },
                        ..
                    }),
                    ..
                },
            }),
            ..
        }) => Some(())
    );

    let upcast: &UpcastTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Upcast(u) => Some(u)
    );

    match upcast.result().coord {
        CoordT {
            ownership: OwnershipT::Own,
            kind: KindT::Interface(InterfaceTT {
                id: IdT {
                    package_coord: x,
                    init_steps: &[],
                    local_name: INameT::Interface(InterfaceNameT {
                        template: InterfaceTemplateNameT { human_namee: StrI("MyInterface"), .. },
                        template_args: &[],
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        } => assert!(x.is_test()),
        other => panic!("upcast result coord: {:?}", other),
    }
    match upcast.inner_expr.result().coord {
        CoordT {
            ownership: OwnershipT::Own,
            kind: KindT::Struct(StructTT {
                id: IdT {
                    package_coord: x,
                    init_steps: &[],
                    local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("MyStruct"), .. }),
                        template_args: &[],
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        } => assert!(x.is_test()),
        other => panic!("inner expr coord: {:?}", other),
    }
}
/*
  test("Tests upcasting from a struct to an interface") {
    val compile = CompilerTestCompilation.test(readCodeFromResource("programs/virtuals/upcasting.vale"))
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupFunction("main")

    Collector.only(main, { case LetNormalTE(ReferenceLocalVariableT(CodeVarNameT(StrI("x")),FinalT,CoordT(OwnT,_, InterfaceTT(simpleNameT("MyInterface")))), _) => })

    val upcast = Collector.onlyOf(main, classOf[UpcastTE])
    upcast.result.coord match { case CoordT(OwnT,_, InterfaceTT(IdT(x, Vector(), InterfaceNameT(InterfaceTemplateNameT(StrI("MyInterface")), Vector())))) => vassert(x.isTest) }
    upcast.innerExpr.result.coord match { case CoordT(OwnT,_, StructTT(IdT(x, Vector(), StructNameT(StructTemplateNameT(StrI("MyStruct")), Vector())))) => vassert(x.isTest) }
  }

*/
// mig: fn tests_calling_a_virtual_function
#[test]
fn tests_calling_a_virtual_function() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = include_str!("../../tests/programs/virtuals/calling.vale");
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let main = coutputs.lookup_function_by_str("main");

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Upcast(u @ UpcastTE {
            target_super_kind: ISuperKindTT::Interface(InterfaceTT {
                id: IdT {
                    local_name: INameT::Interface(InterfaceNameT {
                        template: InterfaceTemplateNameT { human_namee: StrI("Car"), .. },
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        }) => {
            match u.inner_expr.result().coord.kind {
                KindT::Struct(StructTT {
                    id: IdT {
                        local_name: INameT::Struct(StructNameT {
                            template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Toyota"), .. }),
                            ..
                        }),
                        ..
                    },
                    ..
                }) => {}
                other => panic!("inner expr kind: {:?}", other),
            }
            match u.result().coord.kind {
                KindT::Interface(InterfaceTT {
                    id: IdT {
                        package_coord: pc,
                        init_steps: &[],
                        local_name: INameT::Interface(InterfaceNameT {
                            template: InterfaceTemplateNameT { human_namee: StrI("Car"), .. },
                            template_args: &[],
                            ..
                        }),
                        ..
                    },
                    ..
                }) => {
                    assert!(pc.is_test());
                }
                other => panic!("upcast result kind: {:?}", other),
            }
            Some(())
        }
    );
}
/*
  test("Tests calling a virtual function") {
    val compile = CompilerTestCompilation.test(readCodeFromResource("programs/virtuals/calling.vale"))
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case up @ UpcastTE(innerExpr, InterfaceTT(simpleNameT("Car")), _) => {
        Collector.only(innerExpr.result, {
          case StructTT(simpleNameT("Toyota")) =>
        })
        up.result.coord.kind match { case InterfaceTT(IdT(x, Vector(), InterfaceNameT(InterfaceTemplateNameT(StrI("Car")), Vector()))) => vassert(x.isTest) }
      }
    })
  }

*/
// mig: fn tests_upcasting_has_the_right_stuff
#[test]
fn tests_upcasting_has_the_right_stuff() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = include_str!("../../tests/programs/virtuals/calling.vale");
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let main = coutputs.lookup_function_by_str("main");

    let upcast: &UpcastTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Upcast(u @ UpcastTE {
            target_super_kind: ISuperKindTT::Interface(InterfaceTT {
                id: IdT {
                    local_name: INameT::Interface(InterfaceNameT {
                        template: InterfaceTemplateNameT { human_namee: StrI("Car"), .. },
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        }) => Some(u)
    );

    match upcast.inner_expr.result().coord.kind {
        KindT::Struct(StructTT {
            id: IdT {
                local_name: INameT::Struct(StructNameT {
                    template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Toyota"), .. }),
                    ..
                }),
                ..
            },
            ..
        }) => {}
        other => panic!("inner expr kind: {:?}", other),
    }
    match upcast.result().coord.kind {
        KindT::Interface(InterfaceTT {
            id: IdT {
                package_coord: x,
                init_steps: &[],
                local_name: INameT::Interface(InterfaceNameT {
                    template: InterfaceTemplateNameT { human_namee: StrI("Car"), .. },
                    template_args: &[],
                    ..
                }),
                ..
            },
            ..
        }) => assert!(x.is_test()),
        other => panic!("upcast result kind: {:?}", other),
    }

    let impl_edge = coutputs.lookup_edge(upcast.impl_name);
    assert!(impl_edge.sub_citizen.id() == upcast.inner_expr.result().coord.kind.expect_citizen().id());
    assert!(impl_edge.super_interface == upcast.result().coord.kind.expect_citizen().id());

//    freePrototype.fullName.last.parameters.head shouldEqual up.result.reference
}
/*
  test("Tests upcasting has the right stuff") {
    val compile = CompilerTestCompilation.test(readCodeFromResource("programs/virtuals/calling.vale"))
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupFunction("main")
    val up @ UpcastTE(innerExpr, _, implName) =
      Collector.only(main, { case up @ UpcastTE(_, InterfaceTT(simpleNameT("Car")), _) => up})

    Collector.only(innerExpr.result, {
      case StructTT(simpleNameT("Toyota")) =>
    })
    up.result.coord.kind match { case InterfaceTT(IdT(x, Vector(), InterfaceNameT(InterfaceTemplateNameT(StrI("Car")), Vector()))) => vassert(x.isTest) }

    val impl = coutputs.lookupEdge(implName)
    vassert(impl.subCitizen.id == up.innerExpr.result.coord.kind.expectCitizen().id)
    vassert(impl.superInterface == up.result.coord.kind.expectCitizen().id)

//    freePrototype.fullName.last.parameters.head shouldEqual up.result.reference
  }

*/
// mig: fn tests_calling_a_virtual_function_through_a_borrow_ref
#[test]
fn tests_calling_a_virtual_function_through_a_borrow_ref() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = include_str!("../../tests/programs/virtuals/callingThroughBorrow.vale");
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let main = coutputs.lookup_function_by_str("main");

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(FunctionCallTE {
            callable: PrototypeT {
                id: IdT {
                    local_name: INameT::Function(
                        FunctionNameT {
                            template: FunctionTemplateNameT { human_name: StrI("doCivicDance"), .. },
                            ..
                        }
                    ),
                    ..
                },
                return_type: CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT::I32), .. },
                ..
            },
            ..
        }) => {
//        vassert(f.callable.paramTypes == Vector(Coord(Borrow,InterfaceRef2(simpleName("Car")))))
            Some(())
        }
    );
}
/*
  test("Tests calling a virtual function through a borrow ref") {
    val compile = CompilerTestCompilation.test(readCodeFromResource("programs/virtuals/callingThroughBorrow.vale"))
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case f @ FunctionCallTE(PrototypeT(simpleNameT("doCivicDance"),CoordT(ShareT,_, IntT.i32)), _, _) => {
//        vassert(f.callable.paramTypes == Vector(Coord(Borrow,InterfaceRef2(simpleName("Car")))))
      }
    })
  }

*/
// mig: fn tests_calling_a_templated_function_with_explicit_template_args
#[test]
fn tests_calling_a_templated_function_with_explicit_template_args() {
    // Tests putting MyOption<int> as the type of x.
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "func moo<T> () where T Ref { }\n",
        "exported func main() {\n",
        "  moo<int>();\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Tests calling a templated function with explicit template args") {
    // Tests putting MyOption<int> as the type of x.
    val compile = CompilerTestCompilation.test(
      """
        |
        |func moo<T> () where T Ref { }
        |
        |exported func main() {
        |	moo<int>();
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

  // See DSDCTD
*/
// mig: fn tests_destructuring_borrow_doesnt_compile_to_destroy
#[test]
fn tests_destructuring_borrow_doesnt_compile_to_destroy() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "\n",
        "struct Vec3i {\n",
        "  x int;\n",
        "  y int;\n",
        "  z int;\n",
        "}\n",
        "\n",
        "exported func main() int {\n",
        "  v = Vec3i(3, 4, 5);\n",
        "\t [x, y, z] = &v;\n",
        "  return y;\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let destroys = crate::collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Destroy(_) => Some(())
    );
    assert_eq!(destroys.len(), 0);
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ReferenceMemberLookup(
            ReferenceMemberLookupTE {
                struct_expr: ReferenceExpressionTE::SoftLoad(
                    SoftLoadTE {
                        expr: AddressExpressionTE::LocalLookup(
                            LocalLookupTE {
                                local_variable: ILocalVariableT::Reference(
                                    ReferenceLocalVariableT {
                                        variability: VariabilityT::Final,
                                        coord: CoordT { kind: KindT::Struct(_), .. },
                                        ..
                                    }
                                ),
                                ..
                            }
                        ),
                        target_ownership: OwnershipT::Borrow,
                    }
                ),
                member_name: IVarNameT::CodeVar(
                    CodeVarNameT { name: StrI("x"), .. }
                ),
                member_reference: CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT { bits: 32 }), .. },
                variability: VariabilityT::Final,
                ..
            }
        ) => Some(())
    );
}
/*
  test("Tests destructuring borrow doesnt compile to destroy") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Vec3i {
        |  x int;
        |  y int;
        |  z int;
        |}
        |
        |exported func main() int {
        |  v = Vec3i(3, 4, 5);
        |	 [x, y, z] = &v;
        |  return y;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupFunction("main")

    Collector.all(main, {
      case DestroyTE(_, _, _) =>
    }).size shouldEqual 0

    Collector.only(main, {
      case ReferenceMemberLookupTE(_,
        SoftLoadTE(LocalLookupTE(_,ReferenceLocalVariableT(_,FinalT,CoordT(_,_,StructTT(_)))),BorrowT),
        CodeVarNameT(StrI("x")),CoordT(ShareT,_,IntT.i32),FinalT) =>
    })
  }

*/
// mig: fn tests_making_a_variable_with_a_pattern
#[test]
fn tests_making_a_variable_with_a_pattern() {
    // Tests putting MyOption<int> as the type of x.
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "\n",
        "sealed interface MyOption<T> where T Ref { }\n",
        "\n",
        "struct MySome<T> where T Ref {}\n",
        "impl<T> MyOption<T> for MySome<T>;\n",
        "\n",
        "func doSomething(opt MyOption<int>) int {\n",
        "  return 9;\n",
        "}\n",
        "\n",
        "exported func main() int {\n",
        "\tx MyOption<int> = MySome<int>();\n",
        "\treturn doSomething(x);\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Tests making a variable with a pattern") {
    // Tests putting MyOption<int> as the type of x.
    val compile = CompilerTestCompilation.test(
      """
        |
        |sealed interface MyOption<T> where T Ref { }
        |
        |struct MySome<T> where T Ref {}
        |impl<T> MyOption<T> for MySome<T>;
        |
        |func doSomething(opt MyOption<int>) int {
        |  return 9;
        |}
        |
        |exported func main() int {
        |	x MyOption<int> = MySome<int>();
        |	return doSomething(x);
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn tests_a_linked_list
#[test]
fn tests_a_linked_list() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = crate::tests::tests::load_expected("programs/virtuals/ordinarylinkedlist.vale");
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Tests a linked list") {
    val compile = CompilerTestCompilation.test(
      Tests.loadExpected("programs/virtuals/ordinarylinkedlist.vale"))
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn test_borrow_ref
#[test]
fn test_borrow_ref() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = crate::tests::tests::load_expected("programs/borrowRef.vale");
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Test borrow ref") {
    val compile = CompilerTestCompilation.test(Tests.loadExpected("programs/borrowRef.vale"))
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn tests_calling_a_function_with_an_upcast
#[test]
fn tests_calling_a_function_with_an_upcast() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "interface ISpaceship {}\n",
        "struct Firefly {}\n",
        "impl ISpaceship for Firefly;\n",
        "func launch(ship &ISpaceship) { }\n",
        "func main() {\n",
        "  launch(&Firefly());\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let main = coutputs.lookup_function_by_str("main");

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Upcast(UpcastTE {
            target_super_kind: ISuperKindTT::Interface(InterfaceTT {
                id: IdT {
                    local_name: INameT::Interface(InterfaceNameT {
                        template: InterfaceTemplateNameT { human_namee: StrI("ISpaceship"), .. },
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        }) => Some(())
    );
}
/*
  test("Tests calling a function with an upcast") {
    val compile = CompilerTestCompilation.test(
        """
          |interface ISpaceship {}
          |struct Firefly {}
          |impl ISpaceship for Firefly;
          |func launch(ship &ISpaceship) { }
          |func main() {
          |  launch(&Firefly());
          |}
          |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case UpcastTE(
        _,
        InterfaceTT(IdT(_, _, InterfaceNameT(InterfaceTemplateNameT(StrI("ISpaceship")), _))),
        _) =>
    })
  }

*/
// mig: fn tests_calling_a_templated_function_with_an_upcast
#[test]
fn tests_calling_a_templated_function_with_an_upcast() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "interface ISpaceship<T> where T Ref {}\n",
        "struct Firefly<T> where T Ref {}\n",
        "impl<T> ISpaceship<T> for Firefly<T>;\n",
        "func launch<T>(ship &ISpaceship<T>) { }\n",
        "func main() {\n",
        "  launch(&Firefly<int>());\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let main = coutputs.lookup_function_by_str("main");

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Upcast(UpcastTE {
            target_super_kind: ISuperKindTT::Interface(InterfaceTT {
                id: IdT {
                    local_name: INameT::Interface(InterfaceNameT {
                        template: InterfaceTemplateNameT { human_namee: StrI("ISpaceship"), .. },
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        }) => Some(())
    );
}
/*
  test("Tests calling a templated function with an upcast") {
    val compile = CompilerTestCompilation.test(
      """
        |interface ISpaceship<T> where T Ref {}
        |struct Firefly<T> where T Ref {}
        |impl<T> ISpaceship<T> for Firefly<T>;
        |func launch<T>(ship &ISpaceship<T>) { }
        |func main() {
        |  launch(&Firefly<int>());
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case UpcastTE(
        _,
        InterfaceTT(IdT(_, _, InterfaceNameT(InterfaceTemplateNameT(StrI("ISpaceship")), _))),
        _) =>
    })
  }


*/
// mig: fn tests_upcast_with_generics_has_the_right_stuff
#[test]
fn tests_upcast_with_generics_has_the_right_stuff() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "interface ISpaceship<T> where T Ref {}\n",
        "struct Firefly<T> where T Ref {}\n",
        "impl<T> ISpaceship<T> for Firefly<T>;\n",
        "func launch<T>(ship &ISpaceship<T>) { }\n",
        "func main() {\n",
        "  launch(&Firefly<int>());\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let main = coutputs.lookup_function_by_str("main");

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Upcast(UpcastTE {
            target_super_kind: ISuperKindTT::Interface(InterfaceTT {
                id: IdT {
                    local_name: INameT::Interface(InterfaceNameT {
                        template: InterfaceTemplateNameT { human_namee: StrI("ISpaceship"), .. },
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        }) => Some(())
    );
}
/*
  test("Tests upcast with generics has the right stuff") {
    val compile = CompilerTestCompilation.test(
      """
        |interface ISpaceship<T> where T Ref {}
        |struct Firefly<T> where T Ref {}
        |impl<T> ISpaceship<T> for Firefly<T>;
        |func launch<T>(ship &ISpaceship<T>) { }
        |func main() {
        |  launch(&Firefly<int>());
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case UpcastTE(
      _,
      InterfaceTT(IdT(_, _, InterfaceNameT(InterfaceTemplateNameT(StrI("ISpaceship")), _))),
      _) =>
    })
  }

*/
// mig: fn tests_a_templated_linked_list
#[test]
fn tests_a_templated_linked_list() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = crate::tests::tests::load_expected("programs/genericvirtuals/templatedlinkedlist.vale");
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Tests a templated linked list") {
    val compile = CompilerTestCompilation.test(
      Tests.loadExpected("programs/genericvirtuals/templatedlinkedlist.vale"))
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn tests_a_foreach_for_a_linked_list
#[test]
fn tests_a_foreach_for_a_linked_list() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = crate::tests::tests::load_expected("programs/genericvirtuals/foreachlinkedlist.vale");
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Tests a foreach for a linked list") {
    val compile = CompilerTestCompilation.test(
        Tests.loadExpected("programs/genericvirtuals/foreachlinkedlist.vale"))
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case f @ FunctionCallTE(functionNameT("forEach"), _, _) => f
    })
  }

*/
// mig: fn test_return_from_inside_if_destroys_locals
#[test]
fn test_return_from_inside_if_destroys_locals() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct Marine { hp int; }\n",
        "exported func main() int {\n",
        "  m = Marine(5);\n",
        "  x =\n",
        "    if (true) {\n",
        "      return 7;\n",
        "    } else {\n",
        "      m.hp\n",
        "    };\n",
        "  return x;\n",
        "}",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let destructor_calls = crate::collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(fpc @ FunctionCallTE {
            callable: PrototypeT {
                id: IdT {
                    local_name: INameT::Function(FunctionNameT {
                        template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                        parameters: [CoordT {
                            ownership: OwnershipT::Own,
                            kind: KindT::Struct(StructTT {
                                id: IdT {
                                    local_name: INameT::Struct(StructNameT {
                                        template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Marine"), .. }),
                                        ..
                                    }),
                                    ..
                                },
                                ..
                            }),
                            ..
                        }],
                        ..
                    }),
                    init_steps: [INameT::StructTemplate(StructTemplateNameT { human_name: StrI("Marine"), .. })],
                    ..
                },
                ..
            },
            ..
        }) => Some(fpc)
    );
    assert_eq!(destructor_calls.len(), 2);
}
/*
  test("Test return from inside if destroys locals") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Marine { hp int; }
        |exported func main() int {
        |  m = Marine(5);
        |  x =
        |    if (true) {
        |      return 7;
        |    } else {
        |      m.hp
        |    };
        |  return x;
        |}
        |""".stripMargin)// +
    //        Tests.loadExpected("castutils/castutils.vale") +
    //        Tests.loadExpected("printutils/printutils.vale"))
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    val destructorCalls =
      Collector.all(main, {
        case fpc @ FunctionCallTE(
          PrototypeT(IdT(_,Vector(StructTemplateNameT(StrI("Marine"))),FunctionNameT(FunctionTemplateNameT(StrI("drop"),_),Vector(),Vector(CoordT(OwnT,_, StructTT(IdT(_,Vector(),StructNameT(StructTemplateNameT(StrI("Marine")),Vector()))))))),_),_,_) => fpc
      })
    destructorCalls.size shouldEqual 2
  }

*/
// mig: fn recursive_struct
#[test]
fn recursive_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct ListNode imm {\n",
        "  tail ListNode;\n",
        "}\n",
        "func main(a ListNode) {}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Recursive struct") {
    val compile = CompilerTestCompilation.test(
      """
        |struct ListNode imm {
        |  tail ListNode;
        |}
        |func main(a ListNode) {}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn recursive_struct_with_opt
#[test]
fn recursive_struct_with_opt() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.opt.*;\n",
        "struct ListNode {\n",
        "  tail Opt<ListNode>;\n",
        "}\n",
        "func main(a ListNode) {}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Recursive struct with Opt") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.opt.*;
        |struct ListNode {
        |  tail Opt<ListNode>;
        |}
        |func main(a ListNode) {}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

  // Make sure a ListNode struct made it out
*/
// mig: fn templated_imm_struct
#[test]
fn templated_imm_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct ListNode<T Ref> imm {\n",
        "  tail ListNode<T>;\n",
        "}\n",
        "func main(a ListNode<int>) {}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Templated imm struct") {
    val compile = CompilerTestCompilation.test(
      """
        |struct ListNode<T Ref> imm {
        |  tail ListNode<T>;
        |}
        |func main(a ListNode<int>) {}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn borrow_load_member
#[test]
fn borrow_load_member() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct Bork {\n",
        "  x int;\n",
        "}\n",
        "func getX(bork &Bork) int { return bork.x; }\n",
        "struct List {\n",
        "  array! Bork;\n",
        "}\n",
        "exported func main() int {\n",
        "  l = List(Bork(0));\n",
        "  return getX(&l.array);\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    compile.expect_compiler_outputs();
}
/*
  test("Borrow-load member") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Bork {
        |  x int;
        |}
        |func getX(bork &Bork) int { return bork.x; }
        |struct List {
        |  array! Bork;
        |}
        |exported func main() int {
        |  l = List(Bork(0));
        |  return getX(&l.array);
        |}
        """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    vpass()
  }

*/
// mig: fn test_vector_of_struct_templata
#[test]
fn test_vector_of_struct_templata() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.arrays.*;\n",
        "import v.builtins.drop.*;\n",
        "\n",
        "struct Vec2 imm {\n",
        "  x float;\n",
        "  y float;\n",
        "}\n",
        "struct Pattern imm {\n",
        "  patternTiles []<imm>Vec2;\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Test Vector of StructTemplata") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arrays.*;
        |import v.builtins.drop.*;
        |
        |struct Vec2 imm {
        |  x float;
        |  y float;
        |}
        |struct Pattern imm {
        |  patternTiles []<imm>Vec2;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }


*/
// mig: fn if_branches_returns_never_and_struct
#[test]
fn if_branches_returns_never_and_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.panicutils.*;\n",
        "exported struct Moo {}\n",
        "exported func main() Moo {\n",
        "  if true {\n",
        "    Moo()\n",
        "  } else {\n",
        "    panic(\"Error in CreateDir\");\n",
        "  }\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("If branches returns never and struct") {
    // We had a bug where it couldn't reconcile never and struct.

    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.panicutils.*;
        |
        |exported struct Moo {}
        |exported func main() Moo {
        |  if true {
        |    Moo()
        |  } else {
        |    panic("Error in CreateDir");
        |  }
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn test_return
#[test]
fn test_return() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main() int {\n  return 7;\n}";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Return(_) => Some(())
    );
}
/*
  test("Test return") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func main() int {
        |  return 7;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, { case ReturnTE(_) => })
  }

*/
// mig: fn test_return_from_inside_if
#[test]
fn test_return_from_inside_if() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "import v.builtins.panic.*;\nexported func main() int {\n  if (true) {\n    return 7;\n  } else {\n    return 9;\n  }\n  __vbi_panic();\n}";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let returns = crate::collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Return(_) => Some(())
    );
    assert_eq!(returns.len(), 2);
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ConstantInt(
            ConstantIntTE {
                value: ITemplataT::Integer(7),
                ..
            }
        ) => Some(())
    );
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ConstantInt(
            ConstantIntTE {
                value: ITemplataT::Integer(9),
                ..
            }
        ) => Some(())
    );
}
/*
  test("Test return from inside if") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.panic.*;
        |exported func main() int {
        |  if (true) {
        |    return 7;
        |  } else {
        |    return 9;
        |  }
        |  __vbi_panic();
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.all(main, { case ReturnTE(_) => }).size shouldEqual 2
    Collector.only(main, { case ConstantIntTE(IntegerTemplataT(7), _, _) => })
    Collector.only(main, { case ConstantIntTE(IntegerTemplataT(9), _, _) => })
  }

*/
// mig: fn zero_method_anonymous_interface
#[test]
fn zero_method_anonymous_interface() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "interface MyInterface {}\n",
        "exported func main() {\n",
        "  x = MyInterface();\n",
        "}\n",
    );
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    compile.expect_compiler_outputs();
}
/*
  test("Zero method anonymous interface") {
    val compile = CompilerTestCompilation.test(
      """
        |interface MyInterface {}
        |exported func main() {
        |  x = MyInterface();
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }

*/
// mig: fn reports_when_exported_function_depends_on_non_exported_param
#[test]
fn reports_when_exported_function_depends_on_non_exported_param() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "struct Firefly { }\nexported func moo(firefly &Firefly) { }";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ExportedFunctionDependedOnNonExportedKind { .. } => {}
        _other => panic!("expected ExportedFunctionDependedOnNonExportedKind"),
    }
}
/*
  test("Reports when exported function depends on non-exported param") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Firefly { }
        |exported func moo(firefly &Firefly) { }
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ExportedFunctionDependedOnNonExportedKind(_, _, _, _)) =>
    }
  }

*/
// mig: fn reports_when_exported_function_depends_on_non_exported_return
#[test]
fn reports_when_exported_function_depends_on_non_exported_return() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "import panicutils.*;\nstruct Firefly { }\nexported func moo() &Firefly { __pretend<&Firefly>() }";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ExportedFunctionDependedOnNonExportedKind { .. } => {}
        _other => panic!("expected ExportedFunctionDependedOnNonExportedKind"),
    }
}
/*
  test("Reports when exported function depends on non-exported return") {
    val compile = CompilerTestCompilation.test(
      """
        |import panicutils.*;
        |struct Firefly { }
        |exported func moo() &Firefly { __pretend<&Firefly>() }
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ExportedFunctionDependedOnNonExportedKind(_, _, _, _)) =>
      case _ => compile.expectCompilerOutputs(); vfail()
    }
  }

*/
// mig: fn reports_when_extern_function_depends_on_non_exported_param
#[test]
fn reports_when_extern_function_depends_on_non_exported_param() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "struct Firefly { }\nextern func moo(firefly &Firefly);";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ExternFunctionDependedOnNonExportedKind { .. } => {}
        _other => panic!("expected ExternFunctionDependedOnNonExportedKind"),
    }
}
/*
  test("Reports when extern function depends on non-exported param") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Firefly { }
        |extern func moo(firefly &Firefly);
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ExternFunctionDependedOnNonExportedKind(_, _, _, _)) =>
    }
  }

*/
// mig: fn reports_when_extern_function_depends_on_non_exported_return
#[test]
fn reports_when_extern_function_depends_on_non_exported_return() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "struct Firefly imm { }\nextern func moo() &Firefly;";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ExternFunctionDependedOnNonExportedKind { .. } => {}
        _other => panic!("expected ExternFunctionDependedOnNonExportedKind"),
    }
}
/*
  test("Reports when extern function depends on non-exported return") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Firefly imm { }
        |extern func moo() &Firefly;
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ExternFunctionDependedOnNonExportedKind(_, _, _, _)) =>
    }
  }

*/
// mig: fn reports_when_exported_struct_depends_on_non_exported_member
#[test]
fn reports_when_exported_struct_depends_on_non_exported_member() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported struct Firefly imm {\n  raza Raza;\n}\nstruct Raza imm { }";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ExportedImmutableKindDependedOnNonExportedKind { .. } => {}
        _other => panic!("expected ExportedImmutableKindDependedOnNonExportedKind"),
    }
}
/*
  test("Extern function can depend on exported kind") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct Firefly imm { }
        |extern func moo() &Firefly;
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }

  test("Top-level extern function's FunctionExternT has None genericParameterInheritance") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct Firefly imm { }
        |extern func moo() &Firefly;
        |""".stripMargin)
    val interner = compile.interner
    val coutputs = compile.expectCompilerOutputs()
    val externs = coutputs.functionExterns
    val moo = externs.find(_.externName == interner.intern(StrI("moo"))).get
    vassert(moo.genericParameterInheritance.isEmpty)
  }

  test("Extern func inside extern struct's FunctionExternT has Some genericParameterInheritance pointing to container") {
    val compile = CompilerTestCompilation.test(
      """
        |extern struct Vec<T> imm {
        |  extern func with_capacity(c i64) Vec<T>;
        |}
        |""".stripMargin)
    val interner = compile.interner
    val coutputs = compile.expectCompilerOutputs()
    val externs = coutputs.functionExterns
    val withCapacity = externs.find(_.externName == interner.intern(StrI("with_capacity"))).get
    withCapacity.genericParameterInheritance match {
      case Some(GenericParametersInheritance(num)) => vassert(num == 1)
    }
  }

  test("ExternFunctionCallTE no longer carries genericParameterInheritance") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct Firefly imm { }
        |extern func moo() &Firefly;
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupFunction("moo")
    moo.body match {
      case ReturnTE(ExternFunctionCallTE(_, _)) =>
    }
  }

  test("Reports when exported struct depends on non-exported member") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct Firefly imm {
        |  raza Raza;
        |}
        |struct Raza imm { }
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ExportedImmutableKindDependedOnNonExportedKind(_, _, _, _)) =>
    }
  }



*/
// mig: fn checks_that_we_stored_a_borrowed_temporary_in_a_local
#[test]
fn checks_that_we_stored_a_borrowed_temporary_in_a_local() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct Muta { }\n",
        "func doSomething(m &Muta, i int) {}\n",
        "exported func main() {\n",
        "  doSomething(&Muta(), 1)\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::LetAndLend(
            LetAndLendTE {
                target_ownership: OwnershipT::Borrow,
                ..
            }
        ) => Some(())
    );
}
/*
  test("Checks that we stored a borrowed temporary in a local") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Muta { }
        |func doSomething(m &Muta, i int) {}
        |exported func main() {
        |  doSomething(&Muta(), 1)
        |}
      """.stripMargin)

    // Should be a temporary for this object
    Collector.onlyOf(
      compile.expectCompilerOutputs().lookupFunction("main"),
      classOf[LetAndLendTE]) match {
        case LetAndLendTE(_, _, BorrowT) =>
      }
  }

*/
// mig: fn reports_when_reading_nonexistant_local
// Removed this test because this is now caught in the postparser (the scout pass now throws
// CouldntFindVarToMutateS on an unrecognized name, before the typing pass runs). Per canonical
// Scala which removed it for the same reason.
/*
  // Removed this test because this is now caught in the postparser
  //test("Reports when reading nonexistant local") {
  //  val compile = CompilerTestCompilation.test(
  //    """
  //      |exported func main() int {
  //      |  moo
  //      |}
  //      |""".stripMargin)
  //  compile.getCompilerOutputs() match {
  //    case Err(CouldntFindIdentifierToLoadT(_, CodeNameS(StrI("moo")))) =>
  //  }
  //}

*/
// mig: fn reports_when_mutating_after_moving
#[test]
fn reports_when_mutating_after_moving() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct Weapon { ammo! int; }\n",
        "struct Marine { weapon! Weapon; }\n",
        "exported func main() int {\n",
        "  m = Marine(Weapon(7));\n",
        "  newWeapon = Weapon(10);\n",
        "  set m.weapon = newWeapon;\n",
        "  set newWeapon.ammo = 11;\n",
        "  return 42;\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CantUseUnstackifiedLocal { local_id: IVarNameT::CodeVar(CodeVarNameT { name: StrI("newWeapon"), .. }), .. } => {}
        _other => panic!("expected CantUseUnstackifiedLocal"),
    }
}
/*
  test("Reports when mutating after moving") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Weapon {
        |  ammo! int;
        |}
        |struct Marine {
        |  weapon! Weapon;
        |}
        |
        |exported func main() int {
        |  m = Marine(Weapon(7));
        |  newWeapon = Weapon(10);
        |  set m.weapon = newWeapon;
        |  set newWeapon.ammo = 11;
        |  return 42;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantUseUnstackifiedLocal(_, CodeVarNameT(StrI("newWeapon")))) =>
    }
  }

*/
// mig: fn tests_export_struct_twice
#[test]
fn tests_export_struct_twice() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "exported struct Moo { }\n",
        "export Moo as Bork;\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::TypeExportedMultipleTimes { exports, .. } => {
            assert_eq!(exports.len(), 2);
        }
        _ => panic!("Expected TypeExportedMultipleTimes"),
    }
}
/*
  test("Tests export struct twice") {
    // See MMEDT why this is an error
    val compile = CompilerTestCompilation.test(
      """
        |exported struct Moo { }
        |export Moo as Bork;
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(TypeExportedMultipleTimes(_, _, Vector(_, _))) =>
    }
  }

*/
// mig: fn reports_when_reading_after_moving
#[test]
fn reports_when_reading_after_moving() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct Weapon { ammo! int; }\n",
        "struct Marine { weapon! Weapon; }\n",
        "exported func main() int {\n",
        "  m = Marine(Weapon(7));\n",
        "  newWeapon = Weapon(10);\n",
        "  set m.weapon = newWeapon;\n",
        "  println(newWeapon.ammo);\n",
        "  return 42;\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CantUseUnstackifiedLocal { local_id: IVarNameT::CodeVar(CodeVarNameT { name: StrI("newWeapon"), .. }), .. } => {}
        _other => panic!("expected CantUseUnstackifiedLocal"),
    }
}
/*
  test("Reports when reading after moving") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Weapon {
        |  ammo! int;
        |}
        |struct Marine {
        |  weapon! Weapon;
        |}
        |
        |exported func main() int {
        |  m = Marine(Weapon(7));
        |  newWeapon = Weapon(10);
        |  set m.weapon = newWeapon;
        |  println(newWeapon.ammo);
        |  return 42;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantUseUnstackifiedLocal(_, CodeVarNameT(StrI("newWeapon")))) =>
    }
  }

*/
// mig: fn reports_when_moving_from_inside_a_while
#[test]
fn reports_when_moving_from_inside_a_while() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct Marine { ammo int; }\n",
        "exported func main() int {\n",
        "  m = Marine(7);\n",
        "  while (false) {\n",
        "    drop(m);\n",
        "  }\n",
        "  return 42;\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CantUnstackifyOutsideLocalFromInsideWhile { local_id: IVarNameT::CodeVar(CodeVarNameT { name: StrI("m"), .. }), .. } => {}
        _other => panic!("expected CantUnstackifyOutsideLocalFromInsideWhile"),
    }
}
/*
  test("Reports when moving from inside a while") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Marine {
        |  ammo int;
        |}
        |
        |exported func main() int {
        |  m = Marine(7);
        |  while (false) {
        |    drop(m);
        |  }
        |  return 42;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantUnstackifyOutsideLocalFromInsideWhile(_, CodeVarNameT(StrI("m")))) =>
    }
  }

*/
// mig: fn cant_subscript_non_subscriptable_type
#[test]
fn cant_subscript_non_subscriptable_type() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct Weapon { ammo! int; }\n",
        "exported func main() int {\n",
        "  weapon = Weapon(10);\n",
        "  return weapon[42];\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CannotSubscriptT {
            tyype: KindT::Struct(StructTT {
                id: IdT {
                    local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                            human_name: StrI("Weapon"), ..
                        }),
                        template_args: &[],
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        } => {}
        _other => panic!("expected CannotSubscriptT for Weapon struct"),
    }
}
/*
  test("Cant subscript non-subscriptable type") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Weapon {
        |  ammo! int;
        |}
        |
        |exported func main() int {
        |  weapon = Weapon(10);
        |  return weapon[42];
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CannotSubscriptT(_, StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("Weapon")), Vector()))))) =>
    }
  }

*/
// mig: fn humanize_errors
#[test]
fn humanize_errors() {

    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let typing_interner = TypingInterner::new(&typing_bump);

    let tz_code_loc = CodeLocationS::test_zero(&scout_arena);
    let tz = RangeS::test_zero(&scout_arena);
    let tz_slice: &[RangeS] = typing_bump.alloc_slice_copy(&[tz]);
    let test_tld = scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]);

    let filenames_and_sources = FileCoordinateMap::test(&scout_arena, "blah blah blah\nblah blah blah".to_string());
    let humanize_pos = |x| humanize_pos_code_map(&filenames_and_sources, &x);
    let lines_between = |x, y| lines_between(&filenames_and_sources, &x, &y);
    let line_range_containing = |x| line_range_containing(&filenames_and_sources, &x);
    let line_containing = |x| line_containing(&filenames_and_sources, &x);

    let firefly_struct_template_name = typing_interner.intern_struct_template_name(
        StructTemplateNameT { human_name: scout_arena.intern_str("Firefly"), _phantom: std::marker::PhantomData });
    let firefly_struct_name = typing_interner.intern_struct_name(
        StructNameValT { template: IStructTemplateNameT::StructTemplate(firefly_struct_template_name), template_args: &[] });
    let firefly_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Struct(firefly_struct_name),
    });
    let firefly_tt = typing_interner.intern_struct_tt(StructTTValT { id: *firefly_id });
    let firefly_kind = KindT::Struct(firefly_tt);
    let firefly_coord = CoordT { ownership: OwnershipT::Own, region: RegionT { region: IRegionT::Default }, kind: firefly_kind };

    let serenity_struct_template_name = typing_interner.intern_struct_template_name(
        StructTemplateNameT { human_name: scout_arena.intern_str("Serenity"), _phantom: std::marker::PhantomData });
    let serenity_struct_name = typing_interner.intern_struct_name(
        StructNameValT { template: IStructTemplateNameT::StructTemplate(serenity_struct_template_name), template_args: &[] });
    let serenity_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Struct(serenity_struct_name),
    });
    let serenity_tt = typing_interner.intern_struct_tt(StructTTValT { id: *serenity_id });
    let serenity_kind = KindT::Struct(serenity_tt);
    let serenity_coord = CoordT { ownership: OwnershipT::Own, region: RegionT { region: IRegionT::Default }, kind: serenity_kind };

    let ispaceship_interface_template_name = typing_interner.intern_interface_template_name(
        InterfaceTemplateNameT { human_namee: scout_arena.intern_str("ISpaceship"), _phantom: std::marker::PhantomData });
    let ispaceship_interface_name = typing_interner.intern_interface_name(
        InterfaceNameValT { template: ispaceship_interface_template_name, template_args: &[] });
    let ispaceship_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Interface(ispaceship_interface_name),
    });
    let ispaceship_tt = typing_interner.intern_interface_tt(InterfaceTTValT { id: *ispaceship_id });
    let ispaceship_kind = KindT::Interface(ispaceship_tt);

    let unrelated_struct_template_name = typing_interner.intern_struct_template_name(
        StructTemplateNameT { human_name: scout_arena.intern_str("Spoon"), _phantom: std::marker::PhantomData });
    let unrelated_struct_name = typing_interner.intern_struct_name(
        StructNameValT { template: IStructTemplateNameT::StructTemplate(unrelated_struct_template_name), template_args: &[] });
    let unrelated_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Struct(unrelated_struct_name),
    });
    let unrelated_tt = typing_interner.intern_struct_tt(StructTTValT { id: *unrelated_id });
    let unrelated_kind = KindT::Struct(unrelated_tt);

    let myfunc_template_name = typing_interner.intern_function_template_name(
        FunctionTemplateNameT { human_name: scout_arena.intern_str("myFunc"), code_location: tz_code_loc, _phantom: std::marker::PhantomData });
    let firefly_func_name = typing_interner.intern_function_name(
        FunctionNameValT { template: myfunc_template_name, template_args: &[], parameters: &[firefly_coord] });
    let firefly_signature_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Function(firefly_func_name),
    });
    let firefly_signature = typing_interner.intern_signature(
        SignatureValT { id: IdValT { package_coord: test_tld, init_steps: &[], local_name: INameT::Function(firefly_func_name) } });

    let export_template_name = typing_interner.intern_export_template_name(
        ExportTemplateNameT { code_loc: tz_code_loc, _phantom: std::marker::PhantomData });
    let export_name = typing_interner.intern_export_name(
        ExportNameT { template: export_template_name, region: RegionT { region: IRegionT::Default } });
    let firefly_export_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Export(export_name),
    });
    let firefly_export = KindExportT { range: tz, tyype: firefly_kind, id: *firefly_export_id, exported_name: scout_arena.intern_str("Firefly") };
    let serenity_export_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Export(export_name),
    });
    let serenity_export = KindExportT { range: tz, tyype: firefly_kind, id: *serenity_export_id, exported_name: scout_arena.intern_str("Serenity") };
    let exports_slice: &[KindExportT] = typing_bump.alloc_slice_fill_iter([firefly_export, serenity_export].into_iter());

    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindTypeT { range: tz_slice, name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str("Spaceship") })) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindFunctionToCallT { range: tz_slice, fff: FindFunctionFailure {
            name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str("someFunc") })),
            args: &[], rejected_callee_to_reason: &[],
        } }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindFunctionToCallT { range: tz_slice, fff: FindFunctionFailure {
            name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str("") })),
            args: &[], rejected_callee_to_reason: &[],
        } }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CannotSubscriptT { range: tz_slice, tyype: firefly_kind }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindIdentifierToLoadT { range: tz_slice, name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str("spaceship") })) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindMemberT { range: tz_slice, member_name: "hp" }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::BodyResultDoesntMatch {
            range: tz_slice,
            function_name: IFunctionDeclarationNameS::FunctionName(FunctionNameS {
                name: scout_arena.intern_str("myFunc"),
                code_location: tz_code_loc,
            }),
            expected_return_type: firefly_coord,
            result_type: serenity_coord,
        }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntConvertForReturnT { range: tz_slice, expected_type: firefly_coord, actual_type: serenity_coord }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntConvertForMutateT { range: tz_slice, expected_type: firefly_coord, actual_type: serenity_coord }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntConvertForMutateT { range: tz_slice, expected_type: firefly_coord, actual_type: serenity_coord }).is_empty());
    let hp_var_name: &CodeVarNameT = typing_bump.alloc(CodeVarNameT { name: scout_arena.intern_str("hp"), _phantom: std::marker::PhantomData });
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantMoveOutOfMemberT { range: tz_slice, name: IVarNameT::CodeVar(hp_var_name) }).is_empty());
    let firefly_var_name: &CodeVarNameT = typing_bump.alloc(CodeVarNameT { name: scout_arena.intern_str("firefly"), _phantom: std::marker::PhantomData });
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantUseUnstackifiedLocal { range: tz_slice, local_id: IVarNameT::CodeVar(firefly_var_name) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantUnstackifyOutsideLocalFromInsideWhile { range: tz_slice, local_id: IVarNameT::CodeVar(firefly_var_name) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::FunctionAlreadyExists { old_function_range: tz, new_function_range: tz, signature: *firefly_signature_id }).is_empty());
    let bork_var_name: &CodeVarNameT = typing_bump.alloc(CodeVarNameT { name: scout_arena.intern_str("bork"), _phantom: std::marker::PhantomData });
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantMutateFinalMember { range: tz_slice, struct_: *serenity_tt, member_name: IVarNameT::CodeVar(bork_var_name) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::LambdaReturnDoesntMatchInterfaceConstructor { range: tz_slice }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::IfConditionIsntBoolean { range: tz_slice, actual_type: firefly_coord }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::WhileConditionIsntBoolean { range: tz_slice, actual_type: firefly_coord }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantImplNonInterface { range: tz_slice, templata: ITemplataT::Kind(typing_bump.alloc(KindTemplataT { kind: firefly_kind })) }).is_empty());
    let spaceship_snapshot_name_s = scout_arena.intern_struct_declaration_name(
        TopLevelStructDeclarationNameS { name: scout_arena.intern_str("SpaceshipSnapshot"), range: tz });
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::ImmStructCantHaveVaryingMember { range: tz_slice, struct_name: INameS::TopLevelStructDeclaration(spaceship_snapshot_name_s), member_name: "fuel" }).is_empty());
    let candidates_slice: &[FailedSolve<_, _, _, _>] = typing_bump.alloc_slice_fill_iter(std::iter::empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantDowncastUnrelatedTypes { range: tz_slice, source_kind: ispaceship_kind, target_kind: unrelated_kind, candidates: candidates_slice }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantDowncastToInterface { range: tz_slice, target_kind: *ispaceship_tt }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::ExportedFunctionDependedOnNonExportedKind { range: tz_slice, paackage: *test_tld, signature: firefly_signature, non_exported_kind: firefly_kind }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::ExportedImmutableKindDependedOnNonExportedKind { range: tz_slice, paackage: *test_tld, exported_kind: serenity_kind, non_exported_kind: firefly_kind }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::ExternFunctionDependedOnNonExportedKind { range: tz_slice, paackage: *test_tld, signature: firefly_signature, non_exported_kind: firefly_kind }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::TypeExportedMultipleTimes { range: tz_slice, paackage: *test_tld, exports: exports_slice }).is_empty());
    let x_rune = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: scout_arena.intern_str("X") }));
    let mut step_conclusions = std::collections::HashMap::new();
    step_conclusions.insert(x_rune, ITemplataT::Kind(typing_bump.alloc(KindTemplataT { kind: firefly_kind })));
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::TypingPassSolverError { range: tz_slice, failed_solve: FailedSolve {
            steps: vec![Step { complex: false, solved_rules: vec![], added_rules: vec![], conclusions: step_conclusions }],
            conclusions: std::collections::HashMap::new(),
            unsolved_rules: vec![],
            unsolved_runes: vec![],
            error: ISolverError::RuleError(RuleError { err: ITypingPassSolverError::KindIsNotConcrete { kind: ispaceship_kind }, _phantom: std::marker::PhantomData }),
        } }).is_empty());
}
/*
  test("Humanize errors") {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val testPackageCoord = PackageCoordinate.TEST_TLD(interner, keywords)
    val tz = List(RangeS.testZero(interner))
    val tzCodeLoc = CodeLocationS.testZero(interner)
    val funcTemplateName = FunctionTemplateNameT(interner.intern(StrI("main")), tzCodeLoc)
    val funcTemplateId = IdT(testPackageCoord, Vector(), funcTemplateName)
    val funcName = IdT(testPackageCoord, Vector(), FunctionNameT(FunctionTemplateNameT(interner.intern(StrI("main")), tzCodeLoc), Vector(), Vector()))
    val regionName = funcTemplateId.addStep(interner.intern(KindPlaceholderNameT(interner.intern(KindPlaceholderTemplateNameT(0, DenizenDefaultRegionRuneS(FunctionNameS(funcTemplateName.humanName, funcTemplateName.codeLocation)))))))
    val region = RegionT(DefaultRegionT)

    val fireflyKind = StructTT(IdT(testPackageCoord, Vector(), StructNameT(StructTemplateNameT(StrI("Firefly")), Vector())))
    val fireflyCoord = CoordT(OwnT,region,fireflyKind)
    val serenityKind = StructTT(IdT(testPackageCoord, Vector(), StructNameT(StructTemplateNameT(StrI("Serenity")), Vector())))
    val serenityCoord = CoordT(OwnT,region,serenityKind)
    val ispaceshipKind = InterfaceTT(IdT(testPackageCoord, Vector(), InterfaceNameT(InterfaceTemplateNameT(StrI("ISpaceship")), Vector())))
    val ispaceshipCoord = CoordT(OwnT,region,ispaceshipKind)
    val unrelatedKind = StructTT(IdT(testPackageCoord, Vector(), StructNameT(StructTemplateNameT(StrI("Spoon")), Vector())))
    val unrelatedCoord = CoordT(OwnT,region,unrelatedKind)
    val fireflyTemplateName = IdT(testPackageCoord, Vector(), interner.intern(FunctionTemplateNameT(interner.intern(StrI("myFunc")), tz.head.begin)))
    val fireflySignature = ast.SignatureT(IdT(testPackageCoord, Vector(), interner.intern(FunctionNameT(interner.intern(FunctionTemplateNameT(interner.intern(StrI("myFunc")), tz.head.begin)), Vector(), Vector(fireflyCoord)))))
    val fireflyExportId = IdT(testPackageCoord, Vector(), interner.intern(ExportNameT(interner.intern(ExportTemplateNameT(tz.head.begin)), RegionT(DefaultRegionT))))
    val fireflyExport = KindExportT(tz.head, fireflyKind, fireflyExportId, interner.intern(StrI("Firefly")));
    val serenityExportId = IdT(testPackageCoord, Vector(), interner.intern(ExportNameT(interner.intern(ExportTemplateNameT(tz.head.begin)), RegionT(DefaultRegionT))))
    val serenityExport = KindExportT(tz.head, fireflyKind, serenityExportId, interner.intern(StrI("Serenity")));

    val filenamesAndSources = FileCoordinateMap.test(interner, "blah blah blah\nblah blah blah")

    val humanizePos = (x: CodeLocationS) => SourceCodeUtils.humanizePos(filenamesAndSources, x)
    val linesBetween = (x: CodeLocationS, y: CodeLocationS) => SourceCodeUtils.linesBetween(filenamesAndSources, x, y)
    val lineRangeContaining = (x: CodeLocationS) => SourceCodeUtils.lineRangeContaining(filenamesAndSources, x)
    val lineContaining = (x: CodeLocationS) => SourceCodeUtils.lineContaining(filenamesAndSources, x)

    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindTypeT(tz, CodeNameS(interner.intern(StrI("Spaceship"))))).nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindFunctionToCallT(
        tz,
        FindFunctionFailure(
          CodeNameS(StrI("someFunc")),
          Vector(),
          Map()))).nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindFunctionToCallT(
        tz,
        FindFunctionFailure(CodeNameS(interner.intern(StrI(""))), Vector(), Map())))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CannotSubscriptT(
        tz,
        fireflyKind))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindIdentifierToLoadT(
        tz,
        CodeNameS(StrI("spaceship"))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindMemberT(
        tz,
        "hp"))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      BodyResultDoesntMatch(
        tz,
        FunctionNameS(StrI("myFunc"), CodeLocationS.testZero(interner)), fireflyCoord, serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntConvertForReturnT(
        tz,
        fireflyCoord, serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntConvertForMutateT(
        tz,
        fireflyCoord, serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntConvertForMutateT(
        tz,
        fireflyCoord, serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantMoveOutOfMemberT(
        tz,
        CodeVarNameT(StrI("hp"))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantUseUnstackifiedLocal(
        tz,
        CodeVarNameT(StrI("firefly"))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantUnstackifyOutsideLocalFromInsideWhile(
        tz,
        CodeVarNameT(StrI("firefly"))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      FunctionAlreadyExists(tz.head, tz.head, fireflySignature.id))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantMutateFinalMember(
        tz,
        serenityKind,
        CodeVarNameT(StrI("bork"))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      LambdaReturnDoesntMatchInterfaceConstructor(
        tz))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      IfConditionIsntBoolean(
        tz, fireflyCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      WhileConditionIsntBoolean(
        tz, fireflyCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantImplNonInterface(
        tz, KindTemplataT(fireflyKind)))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      ImmStructCantHaveVaryingMember(
        tz, TopLevelStructDeclarationNameS(interner.intern(StrI("SpaceshipSnapshot")), tz.head), "fuel"))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantDowncastUnrelatedTypes(
        tz, ispaceshipKind, unrelatedKind, Vector()))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantDowncastToInterface(
        tz, ispaceshipKind))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      ExportedFunctionDependedOnNonExportedKind(
        tz, PackageCoordinate.TEST_TLD(interner, keywords), fireflySignature, fireflyKind))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      ExportedImmutableKindDependedOnNonExportedKind(
        tz, PackageCoordinate.TEST_TLD(interner, keywords), serenityKind, fireflyKind))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      ExternFunctionDependedOnNonExportedKind(
        tz, PackageCoordinate.TEST_TLD(interner, keywords), fireflySignature, fireflyKind))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      TypeExportedMultipleTimes(
        tz, PackageCoordinate.TEST_TLD(interner, keywords), Vector(fireflyExport, serenityExport)))
      .nonEmpty)
//    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
//      NotEnoughToSolveError(
//        tz,
//        Map(
//          CodeRuneS(StrI("X")) -> KindTemplata(fireflyKind)),
//        Vector(CodeRuneS(StrI("Y")))))
//      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      TypingPassSolverError(
        tz,
        FailedSolve(
          Vector(
            Step[IRulexSR, IRuneS, ITemplataT[ITemplataType]](
              false,
              Vector(),
              Vector(),
              Map(
                CodeRuneS(StrI("X")) -> KindTemplataT(fireflyKind)))).toStream,
          Map(),
          Vector(),
          Vector(),
          RuleError(KindIsNotConcrete(ispaceshipKind)))))
      .nonEmpty)
  }

*/
// mig: fn report_when_multiple_types_in_array
#[test]
fn report_when_multiple_types_in_array() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main() int {\n  arr = [#](true, 42);\n  return arr.1;\n}";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ArrayElementsHaveDifferentTypes { types, .. } => {
            let types_set: HashSet<CoordT> = types.iter().copied().collect();
            assert_eq!(types_set, HashSet::from([
                CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: KindT::Int(IntT::I32) },
                CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: KindT::Bool(BoolT) },
            ]));
        }
        _other => panic!("expected ArrayElementsHaveDifferentTypes"),
    }
}
/*
  test("Report when multiple types in array") {
    val compile = CompilerTestCompilation.test(
      """
        |exported func main() int {
        |  arr = [#](true, 42);
        |  return arr.1;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ArrayElementsHaveDifferentTypes(_, types)) => {
        types shouldEqual Set(CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32), CoordT(ShareT, RegionT(DefaultRegionT), BoolT()))
      }
    }
  }

*/
// mig: fn report_when_abstract_method_defined_outside_open_interface
#[test]
fn report_when_abstract_method_defined_outside_open_interface() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "import v.builtins.panic.*;\ninterface IBlah { }\nabstract func bork(virtual moo &IBlah);\nexported func main() {\n  bork(__vbi_panic());\n}";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::AbstractMethodOutsideOpenInterface { .. } => {}
        _other => panic!("expected AbstractMethodOutsideOpenInterface"),
    }
}
/*
  test("Report when abstract method defined outside open interface") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.panic.*;
        |interface IBlah { }
        |abstract func bork(virtual moo &IBlah);
        |exported func main() {
        |  bork(__vbi_panic());
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(AbstractMethodOutsideOpenInterface(_)) =>
    }
  }

*/
// mig: fn report_when_imm_struct_has_varying_member
#[test]
fn report_when_imm_struct_has_varying_member() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "struct Spaceship imm {\n  name! str;\n  numWings int;\n}\nexported func main() {\n  ship = Spaceship(\"Serenity\", 2);\n  println(ship.name);\n}";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ImmStructCantHaveVaryingMember { .. } => {}
        _other => panic!("expected ImmStructCantHaveVaryingMember"),
    }
}
/*
  test("Report when imm struct has varying member") {
    // https://github.com/ValeLang/Vale/issues/131
    val compile = CompilerTestCompilation.test(
      """
        |struct Spaceship imm {
        |  name! str;
        |  numWings int;
        |}
        |exported func main() {
        |  ship = Spaceship("Serenity", 2);
        |  println(ship.name);
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ImmStructCantHaveVaryingMember(_, _, _)) =>
    }
  }

*/
// mig: fn report_imm_mut_mismatch_for_generic_type
#[test]
fn report_imm_mut_mismatch_for_generic_type() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "struct MyImmContainer<T Ref> imm\nwhere func drop(T)void { value T; }\nstruct MyMutStruct { }\nexported func main() { x = MyImmContainer<MyMutStruct>(MyMutStruct()); }";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ImmStructCantHaveMutableMember { .. } => {}
        _other => panic!("expected ImmStructCantHaveMutableMember"),
    }
}
/*
  test("Report imm mut mismatch for generic type") {
    val compile = CompilerTestCompilation.test(
      """
        |struct MyImmContainer<T Ref> imm
        |where func drop(T)void { value T; }
        |struct MyMutStruct { }
        |exported func main() { x = MyImmContainer<MyMutStruct>(MyMutStruct()); }
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ImmStructCantHaveMutableMember(_, _, _)) =>
    }
  }

*/
// mig: fn tests_stamping_a_struct_and_its_implemented_interface_from_a_function_param
#[test]
fn tests_stamping_a_struct_and_its_implemented_interface_from_a_function_param() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.panicutils.*;\n",
        "import v.builtins.drop.*;\n",
        "import panicutils.*;\n",
        "sealed interface MyOption<T Ref> where func drop(T)void { }\n",
        "struct MySome<T Ref> where func drop(T)void { value T; }\n",
        "impl<T> MyOption<T> for MySome<T> where func drop(T)void;\n",
        "func moo(a MySome<int>) { }\n",
        "exported func main() { moo(__pretend<MySome<int>>()); }\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let interface_template_name = compile.typing_interner.intern_interface_template_name(
        InterfaceTemplateNameT {
            human_namee: scout_arena.intern_str("MyOption"),
            _phantom: std::marker::PhantomData,
        });
    let struct_template_name = StructTemplateNameT {
        human_name: scout_arena.intern_str("MySome"),
        _phantom: std::marker::PhantomData,
    };

    let coutputs = compile.expect_compiler_outputs();

    let interface = coutputs.lookup_interface_by_template_name(interface_template_name);
    let my_struct = coutputs.lookup_struct_by_template_name(struct_template_name);

    coutputs.lookup_impl(my_struct.instantiated_citizen.id, interface.instantiated_interface.id);
}
/*
  test("Tests stamping a struct and its implemented interface from a function param") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.panicutils.*;
        |import v.builtins.drop.*;
        |import panicutils.*;
        |sealed interface MyOption<T Ref> where func drop(T)void { }
        |struct MySome<T Ref> where func drop(T)void { value T; }
        |impl<T> MyOption<T> for MySome<T> where func drop(T)void;
        |func moo(a MySome<int>) { }
        |exported func main() { moo(__pretend<MySome<int>>()); }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val interner = compile.interner
    val keywords = compile.keywords

    val interface =
      coutputs.lookupInterfaceByTemplateName(
        interner.intern(InterfaceTemplateNameT(interner.intern(StrI("MyOption")))))

    val struct =
      coutputs.lookupStructByTemplateName(
        interner.intern(StructTemplateNameT(interner.intern(StrI("MySome")))))

    coutputs.lookupImpl(struct.instantiatedCitizen.id, interface.instantiatedInterface.id)
  }

*/
// mig: fn report_when_imm_contains_varying_member
#[test]
fn report_when_imm_contains_varying_member() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "struct Spaceship imm {\n  name! str;\n  numWings int;\n}";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ImmStructCantHaveVaryingMember { struct_name: INameS::TopLevelStructDeclaration(TopLevelStructDeclarationNameS { name: StrI("Spaceship"), .. }), member_name: "name", .. } => {}
        _other => panic!("expected ImmStructCantHaveVaryingMember for Spaceship.name"),
    }
}
/*
  test("Report when imm contains varying member") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Spaceship imm {
        |  name! str;
        |  numWings int;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ImmStructCantHaveVaryingMember(_,TopLevelStructDeclarationNameS(StrI("Spaceship"),_),"name")) =>
    }
  }

*/
// mig: fn test_imm_array
#[test]
fn test_imm_array() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.panic.*;\n",
        "import v.builtins.drop.*;\n",
        "export #[]int as ImmArrInt;\n",
        "exported func main(arr #[]int) {\n",
        "  __vbi_panic();\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    match main.header.params[0].tyype.kind {
        KindT::RuntimeSizedArray(rsa) => {
            match rsa.name.local_name {
                INameT::RuntimeSizedArray(rsan) => {
                    assert_eq!(rsan.arr.mutability, ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }));
                }
                _ => panic!("Expected RuntimeSizedArray local_name"),
            }
        }
        _ => panic!("Expected RuntimeSizedArray kind"),
    }
}
/*
  test("Test imm array") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.panic.*;
        |import v.builtins.drop.*;
        |export #[]int as ImmArrInt;
        |exported func main(arr #[]int) {
        |  __vbi_panic();
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    main.header.params.head.tyype.kind match { case contentsRuntimeSizedArrayTT(MutabilityTemplataT(ImmutableT), _, _) => }
  }


*/
// mig: fn tests_calling_an_abstract_function
#[test]
fn tests_calling_an_abstract_function() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = include_str!("../../tests/programs/genericvirtuals/callingAbstract.vale");
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    coutputs.functions.iter().find(|f| {
        matches!(f.header.id.local_name,
            INameT::Function(
                FunctionNameT {
                    template: FunctionTemplateNameT { human_name, .. },
                    ..
                }
            )
            if human_name == "doThing"
        ) && f.header.get_abstract_interface().is_some()
    }).unwrap();
}
/*
  test("Tests calling an abstract function") {
    val compile = CompilerTestCompilation.test(
      Tests.loadExpected("programs/genericvirtuals/callingAbstract.vale"))
    val coutputs = compile.expectCompilerOutputs()

    coutputs.functions.collectFirst({
      case FunctionDefinitionT(header @ functionNameT("doThing"), _, _) if header.getAbstractInterface != None => true
    }).get
  }

*/
// mig: fn test_struct_default_generic_argument_in_type
#[test]
fn test_struct_default_generic_argument_in_type() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct MyHashSet<K Ref, H Int = 5> { }\n",
        "struct MyStruct {\n",
        "  x MyHashSet<bool>();\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let moo = coutputs.lookup_struct_by_str("MyStruct");
    let tyype = crate::collect_only_tnode!(
        NodeRefT::StructDefinition(moo),
        NodeRefT::ReferenceMemberType(rmt) => Some(rmt.reference)
    );
    match tyype {
        CoordT {
            ownership: OwnershipT::Own,
            kind: KindT::Struct(StructTT {
                id: IdT {
                    local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(
                            StructTemplateNameT {
                                human_name: StrI("MyHashSet"),
                                ..
                            }
                        ),
                        template_args: [
                            ITemplataT::Coord(
                                CoordTemplataT {
                                    coord: CoordT { ownership: OwnershipT::Share, kind: KindT::Bool(_), .. }
                                }
                            ),
                            ITemplataT::Integer(5),
                        ],
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        } => {}
        _ => panic!("unexpected tyype"),
    }
}
/*
Guardian: temp-disable: IIDX — StrI("MyHashSet") appears in a match pattern arm (destructuring), not as a value construction call. IIDX's DENY example is about constructing StrI values outside the interner; pattern matching is not construction. The TL explicitly approved this inline literal pattern approach. — FrontendRust/guardian-logs/request-1715-1778687371724/hook-1715/test_struct_default_generic_argument_in_type--3598.0.ImmediateInterningDiscipline-IIDX.ImmediateInterningDiscipline-IIDX.verdict.md
  test("Test struct default generic argument in type") {
    val compile = CompilerTestCompilation.test(
      """
        |struct MyHashSet<K Ref, H Int = 5> { }
        |struct MyStruct {
        |  x MyHashSet<bool>();
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupStruct("MyStruct")
    val tyype = Collector.only(moo, { case ReferenceMemberTypeT(c) => c })
    tyype match {
      case CoordT(
      OwnT,
      _,
      StructTT(
      IdT(_,_,
      StructNameT(
      StructTemplateNameT(StrI("MyHashSet")),
      Vector(
      CoordTemplataT(CoordT(ShareT,_,BoolT())),
      IntegerTemplataT(5)))))) =>
    }
  }

*/
// mig: fn lock_weak_member
#[test]
fn lock_weak_member() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.opt.*;\n",
        "import v.builtins.weak.*;\n",
        "import v.builtins.logic.*;\n",
        "import v.builtins.drop.*;\n",
        "import panicutils.*;\n",
        "import printutils.*;\n",
        "\n",
        "struct Base {\n",
        "  name str;\n",
        "}\n",
        "struct Spaceship {\n",
        "  name str;\n",
        "  origin &&Base;\n",
        "}\n",
        "func printShipBase(ship &Spaceship) {\n",
        "  maybeOrigin = lock(ship.origin);\n",
        "  if (not maybeOrigin.isEmpty()) {\n",
        "    o = maybeOrigin.get();\n",
        "    println(\"Ship base: \" + o.name);\n",
        "  } else {\n",
        "    println(\"Ship base unknown!\");\n",
        "  }\n",
        "}\n",
        "exported func main() {\n",
        "  base = Base(\"Zion\");\n",
        "  ship = Spaceship(\"Neb\", &&base);\n",
        "  printShipBase(&ship);\n",
        "  (base).drop();\n",
        "  printShipBase(&ship);\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Lock weak member") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.opt.*;
        |import v.builtins.weak.*;
        |import v.builtins.logic.*;
        |import v.builtins.drop.*;
        |import panicutils.*;
        |import printutils.*;
        |
        |weakable struct Base {
        |  name str;
        |}
        |struct Spaceship {
        |  name str;
        |  origin &&Base;
        |}
        |func printShipBase(ship &Spaceship) {
        |  maybeOrigin = lock(ship.origin); «14»«15»
        |  if (not maybeOrigin.isEmpty()) { «16»
        |    o = maybeOrigin.get();
        |    println("Ship base: " + o.name);
        |  } else {
        |    println("Ship base unknown!");
        |  }
        |}
        |exported func main() {
        |  base = Base("Zion");
        |  ship = Spaceship("Neb", &&base);
        |  printShipBase(&ship);
        |  (base).drop(); // Destroys base.
        |  printShipBase(&ship);
        |}
        |""".stripMargin)

    compile.expectCompilerOutputs()
  }

  // See DSDCTD
*/
// mig: fn tests_destructuring_shared_doesnt_compile_to_destroy
#[test]
fn tests_destructuring_shared_doesnt_compile_to_destroy() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "\n",
        "struct Vec3i imm {\n",
        "  x int;\n",
        "  y int;\n",
        "  z int;\n",
        "}\n",
        "\n",
        "exported func main() int {\n",
        "\t Vec3i[x, y, z] = Vec3i(3, 4, 5);\n",
        "  return y;\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let destroys = crate::collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Destroy(_) => Some(())
    );
    assert_eq!(destroys.len(), 0);
}
/*
  test("Tests destructuring shared doesnt compile to destroy") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Vec3i imm {
        |  x int;
        |  y int;
        |  z int;
        |}
        |
        |exported func main() int {
        |	 Vec3i[x, y, z] = Vec3i(3, 4, 5);
        |  return y;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    Collector.all(coutputs.lookupFunction("main"), {
      case DestroyTE(_, _, _) =>
    }).size shouldEqual 0

//    // Make sure there's a destroy in its destructor though.
//    val destructor =
//      vassertOne(
//        coutputs.functions.collect({
//          case f if (f.header.fullName.last match { case FreeNameT(_, _, _) => true case _ => false }) => f
//        }))
//
//    Collector.only(destructor, { case DestroyTE(referenceExprResultStructName(StrI("Vec3i")), _, _) => })
//    Collector.all(destructor, { case DiscardTE(referenceExprResultKind(IntT(_))) => }).size shouldEqual 3
  }


*/
// mig: fn generates_free_function_for_imm_struct
#[test]
fn generates_free_function_for_imm_struct() {
    let code = r#"
        struct Vec3i imm {
          x int;
          y int;
          z int;
        }
      "#;
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Generates free function for imm struct") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Vec3i imm {
        |  x int;
        |  y int;
        |  z int;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

//    // Make sure there's a destroy in its destructor though.
//    val freeFunc =
//      vassertOne(
//        coutputs.functions.collect({
//          case f if (f.header.fullName.last match { case FreeNameT(_, _, _) => true case _ => false }) => f
//        }))
//
//    Collector.only(freeFunc, { case DestroyTE(referenceExprResultStructName(StrI("Vec3i")), _, _) => })
//    Collector.all(freeFunc, { case DiscardTE(referenceExprResultKind(IntT(_))) => }).size shouldEqual 3
  }

*/
// mig: fn reports_when_exported_ssa_depends_on_non_exported_element
#[test]
fn reports_when_exported_ssa_depends_on_non_exported_element() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "export [#5]<imm>Raza as RazaArray;\nstruct Raza imm { }";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ExportedImmutableKindDependedOnNonExportedKind { .. } => {}
        _other => panic!("expected ExportedImmutableKindDependedOnNonExportedKind"),
    }
}
/*
  test("Reports when exported SSA depends on non-exported element") {
    val compile = CompilerTestCompilation.test(
      """
        |export [#5]<imm>Raza as RazaArray;
        |struct Raza imm { }
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ExportedImmutableKindDependedOnNonExportedKind(_, _, _, _)) =>
    }
  }

*/
// mig: fn reports_when_exported_rsa_depends_on_non_exported_element
#[test]
fn reports_when_exported_rsa_depends_on_non_exported_element() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "export []<imm>Raza as RazaArray;\nstruct Raza imm { }";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::ExportedImmutableKindDependedOnNonExportedKind { .. } => {}
        _other => panic!("expected ExportedImmutableKindDependedOnNonExportedKind"),
    }
}
/*
  test("Reports when exported RSA depends on non-exported element") {
    val compile = CompilerTestCompilation.test(
      """
        |export []<imm>Raza as RazaArray;
        |struct Raza imm { }
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(ExportedImmutableKindDependedOnNonExportedKind(_, _, _, _)) =>
    }
  }

*/
// mig: fn imm_generic_can_contain_imm_thing
/*
  test("Imm generic can contain imm thing") {
    val compile = CompilerTestCompilation.test(
      """
        |struct MyImmContainer<T Ref imm> imm
        |where func drop(T)void { value T; }
        |struct MyMutStruct { }
        |exported func main() { x = MyImmContainer<MyMutStruct>(MyMutStruct()); }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn test_make_array
#[test]
fn test_make_array() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r#"
import v.builtins.arith.*;
import array.make.*;
import v.builtins.arrays.*;
import v.builtins.drop.*;

exported func main() int {
  a = MakeArray<int>(11, {_});
  return len(&a);
}
"#;
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Test MakeArray") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arith.*;
        |import array.make.*;
        |import v.builtins.arrays.*;
        |import v.builtins.drop.*;
        |
        |exported func main() int {
        |  a = MakeArray<int>(11, {_});
        |  return len(&a);
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn test_array_push_pop_len_capacity_drop
#[test]
fn test_array_push_pop_len_capacity_drop() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.arrays.*;\n",
        "import v.builtins.drop.*;\n",
        "\n",
        "exported func main() void {\n",
        "  arr = Array<mut, int>(9);\n",
        "  arr.push(420);\n",
        "  arr.push(421);\n",
        "  arr.push(422);\n",
        "  arr.len();\n",
        "  arr.capacity();\n",
        "  // implicit drop with pops\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Test array push, pop, len, capacity, drop") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arrays.*;
        |import v.builtins.drop.*;
        |
        |exported func main() void {
        |  arr = Array<mut, int>(9);
        |  arr.push(420);
        |  arr.push(421);
        |  arr.push(422);
        |  arr.len();
        |  arr.capacity();
        |  // implicit drop with pops
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn upcast_generic
#[test]
fn upcast_generic() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.drop.*;\n",
        "\n",
        "interface IShip {}\n",
        "\n",
        "struct Raza { fuel int; }\n",
        "impl IShip for Raza;\n",
        "\n",
        "func doUpcast<T>(x T) IShip\n",
        "where implements(T, IShip) {\n",
        "  i IShip = x;\n",
        "  return i;\n",
        "}\n",
        "\n",
        "exported func main() {\n",
        "  doUpcast(Raza(42));\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();

    let do_upcast = coutputs.lookup_function_by_str("doUpcast");

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(do_upcast),
        NodeRefT::Upcast(u) => {
            match u.inner_expr.result().coord.kind {
                KindT::KindPlaceholder(_) => {}
                other => panic!("sourceExpr.result.coord.kind: {:?}", other),
            }
            match u.target_super_kind {
                ISuperKindTT::Interface(InterfaceTT {
                    id: IdT {
                        init_steps: &[],
                        local_name: INameT::Interface(InterfaceNameT {
                            template: InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                            template_args: &[],
                            ..
                        }),
                        ..
                    },
                    ..
                }) => {}
                other => panic!("targetSuperKind: {:?}", other),
            }
            Some(())
        }
    );
}
/*
  test("Upcast generic") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |interface IShip {}
        |
        |struct Raza { fuel int; }
        |impl IShip for Raza;
        |
        |func doUpcast<T>(x T) IShip
        |where implements(T, IShip) {
        |  i IShip = x;
        |  return i;
        |}
        |
        |exported func main() {
        |  doUpcast(Raza(42));
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val doUpcast = coutputs.lookupFunction("doUpcast")
    Collector.only(doUpcast, {
      case UpcastTE(sourceExpr, targetSuperKind, _) => {
        sourceExpr.result.coord.kind match {
          case KindPlaceholderT(_) =>
        }
        targetSuperKind match {
          case InterfaceTT(IdT(_, Vector(),InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")),Vector()))) =>
        }
      }
    })
  }

*/
// mig: fn downcast_function_rrbfs
#[test]
fn downcast_function_rrbfs() {
    // Here we had something interesting happen: the complex solve had a race with the thing that
    // populates identifying runes.
    // Populating identifying runes only happens after the solver has done as much as it possibly
    // can... but the solver sometimes takes a leap (as part of CSALR, SMCMST) to figure out the best type
    // to meet some requirements.
    // The solution was to make it only do that leap when solving call sites.
    // See RRBFS.
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "\n",
        "#!DeriveInterfaceDrop\n",
        "sealed interface Result<OkType Ref, ErrType Ref> { }\n",
        "\n",
        "#!DeriveStructDrop\n",
        "struct Ok<OkType Ref, ErrType Ref> { value OkType; }\n",
        "\n",
        "impl<OkType, ErrType> Result<OkType, ErrType> for Ok<OkType, ErrType>;\n",
        "\n",
        "#!DeriveStructDrop\n",
        "struct Err<OkType Ref, ErrType Ref> { value ErrType; }\n",
        "\n",
        "impl<OkType, ErrType> Result<OkType, ErrType> for Err<OkType, ErrType>;\n",
        "\n",
        "\n",
        "extern(\"vale_as_subtype\")\n",
        "func as<SubType Ref, SuperType Ref>(left &SuperType) Result<&SubType, &SuperType>\n",
        "where implements(SubType, SuperType);\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();

    {

        let as_funcs: Vec<_> = coutputs.functions.iter().filter(|f| {
            matches!(f.header.id.local_name, INameT::Function(FunctionNameT {
                template: FunctionTemplateNameT { human_name: StrI("as"), .. },
                parameters: [CoordT { ownership: OwnershipT::Borrow, .. }],
                ..
            }))
        }).copied().collect();
        let as_func = expect_1(&as_funcs);
        let as_ = crate::collect_only_tnode!(
            NodeRefT::FunctionDefinition(as_func),
            NodeRefT::AsSubtype(as_) => Some(as_)
        );
        let source_expr = as_.source_expr;
        let target_subtype = as_.target_type;
        let result_opt_type = as_.result_result_type;
        let ok_constructor = as_.ok_constructor;
        let err_constructor = as_.err_constructor;

        match source_expr.result().coord {
            CoordT {
                ownership: OwnershipT::Borrow,
                kind: KindT::KindPlaceholder(KindPlaceholderT {
                    id: IdT {
                        init_steps: [INameT::FunctionTemplate(FunctionTemplateNameT { human_name: StrI("as"), .. })],
                        local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                            template: KindPlaceholderTemplateNameT { index: 1, .. },
                        }),
                        ..
                    },
                    ..
                }),
                ..
            } => {}
            //case CoordT(BorrowT, InterfaceTT(FullNameT(_, Vector(), InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")), Vector())))) =>
            other => panic!("sourceExpr.result.coord: {:?}", other),
        }
        match target_subtype.kind {
            KindT::KindPlaceholder(KindPlaceholderT {
                id: IdT {
                    init_steps: [INameT::FunctionTemplate(FunctionTemplateNameT { human_name: StrI("as"), .. })],
                    local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                        template: KindPlaceholderTemplateNameT { index: 0, .. },
                    }),
                    ..
                },
                ..
            }) => {}
            KindT::Struct(StructTT {
                id: IdT {
                    init_steps: &[],
                    local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Raza"), .. }),
                        template_args: &[],
                        ..
                    }),
                    ..
                },
                ..
            }) => {}
            other => panic!("targetSubtype.kind: {:?}", other),
        }
        let (first_generic_arg, second_generic_arg) = match result_opt_type {
            CoordT {
                ownership: OwnershipT::Own,
                kind: KindT::Interface(InterfaceTT {
                    id: IdT {
                        init_steps: &[],
                        local_name: INameT::Interface(InterfaceNameT {
                            template: InterfaceTemplateNameT { human_namee: StrI("Result"), .. },
                            template_args: [first, second],
                            ..
                        }),
                        ..
                    },
                    ..
                }),
                ..
            } => (first, second),
            other => panic!("resultOptType: {:?}", other),
        };
        // They should both be pointers, since we dont really do borrows in structs yet
        match first_generic_arg {
            ITemplataT::Coord(CoordTemplataT {
                coord: CoordT {
                    ownership: OwnershipT::Borrow,
                    kind: KindT::KindPlaceholder(KindPlaceholderT {
                        id: IdT {
                            init_steps: [INameT::FunctionTemplate(FunctionTemplateNameT { human_name: StrI("as"), .. })],
                            local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                                template: KindPlaceholderTemplateNameT { index: 0, .. },
                            }),
                            ..
                        },
                        ..
                    }),
                    ..
                }
            }) => {}
            other => panic!("firstGenericArg: {:?}", other),
        }
        match second_generic_arg {
            ITemplataT::Coord(CoordTemplataT {
                coord: CoordT {
                    ownership: OwnershipT::Borrow,
                    kind: KindT::KindPlaceholder(KindPlaceholderT {
                        id: IdT {
                            init_steps: [INameT::FunctionTemplate(FunctionTemplateNameT { human_name: StrI("as"), .. })],
                            local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                                template: KindPlaceholderTemplateNameT { index: 1, .. },
                            }),
                            ..
                        },
                        ..
                    }),
                    ..
                }
            }) => {}
            other => panic!("secondGenericArg: {:?}", other),
        }
        assert_eq!(ok_constructor.id.local_name.parameters()[0], target_subtype);
        assert_eq!(err_constructor.id.local_name.parameters()[0], source_expr.result().coord);
    }
}
/*
  test("Downcast function, RRBFS") {
    // Here we had something interesting happen: the complex solve had a race with the thing that
    // populates identifying runes.
    // Populating identifying runes only happens after the solver has done as much as it possibly
    // can... but the solver sometimes takes a leap (as part of CSALR, SMCMST) to figure out the best type
    // to meet some requirements.
    // The solution was to make it only do that leap when solving call sites.
    // See RRBFS.
    val compile = CompilerTestCompilation.test(
      """
        |
        |#!DeriveInterfaceDrop
        |sealed interface Result<OkType Ref, ErrType Ref> { }
        |
        |#!DeriveStructDrop
        |struct Ok<OkType Ref, ErrType Ref> { value OkType; }
        |
        |impl<OkType, ErrType> Result<OkType, ErrType> for Ok<OkType, ErrType>;
        |
        |#!DeriveStructDrop
        |struct Err<OkType Ref, ErrType Ref> { value ErrType; }
        |
        |impl<OkType, ErrType> Result<OkType, ErrType> for Err<OkType, ErrType>;
        |
        |
        |extern("vale_as_subtype")
        |func as<SubType Ref, SuperType Ref>(left &SuperType) Result<&SubType, &SuperType>
        |where implements(SubType, SuperType);
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val asFunc =
      vassertOne(
        coutputs.functions.filter({
          case FunctionDefinitionT(FunctionHeaderT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("as"), _), _, Vector(CoordT(BorrowT, _, _)))), _, _, _, _), _, _) => true
          case _ => false
        }))
    val as = Collector.only(asFunc, { case as@AsSubtypeTE(_, _, _, _, _, _, _, _) => as })
    val AsSubtypeTE(sourceExpr, targetSubtype, resultOptType, okConstructor, errConstructor, _, _, _) = as
    sourceExpr.result.coord match {
      case CoordT(BorrowT,_, KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("as"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(1, _))))) =>
      //case CoordT(BorrowT, InterfaceTT(FullNameT(_, Vector(), InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")), Vector())))) =>
    }
    targetSubtype.kind match {
      case KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("as"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))) =>
      case StructTT(IdT(_, Vector(), StructNameT(StructTemplateNameT(StrI("Raza")), Vector()))) =>
    }
    val (firstGenericArg, secondGenericArg) =
      resultOptType match {
        case CoordT(
        OwnT,
        _,
        InterfaceTT(
        IdT(
        _, Vector(),
        InterfaceNameT(
        InterfaceTemplateNameT(StrI("Result")),
        Vector(firstGenericArg, secondGenericArg))))) => (firstGenericArg, secondGenericArg)
      }
    // They should both be pointers, since we dont really do borrows in structs yet
    firstGenericArg match {
      case CoordTemplataT(
      CoordT(
      BorrowT,
      _,
      KindPlaceholderT(
      IdT(_,Vector(FunctionTemplateNameT(StrI("as"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))))) =>
    }
    secondGenericArg match {
      case CoordTemplataT(
      CoordT(
      BorrowT,
      _,
      KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("as"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(1, _)))))) =>
    }
    vassert(okConstructor.paramTypes.head == targetSubtype)
    vassert(errConstructor.paramTypes.head == sourceExpr.result.coord)
  }

*/
// AFTERM: doublecheck this
// mig: fn downcast_with_as
#[test]
fn downcast_with_as() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.as.*;\n",
        "import v.builtins.logic.*;\n",
        "import v.builtins.drop.*;\n",
        "\n",
        "interface IShip {}\n",
        "\n",
        "struct Raza { fuel int; }\n",
        "impl IShip for Raza;\n",
        "\n",
        "exported func main() {\n",
        "  ship IShip = Raza(42);\n",
        "  ship.as<Raza>();\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();

    {

        let main_func = coutputs.lookup_function_by_str("main");
        let (as_prototype, as_arg) = crate::collect_only_tnode!(
            NodeRefT::FunctionDefinition(main_func),
            NodeRefT::FunctionCall(c @ FunctionCallTE {
                callable: PrototypeT {
                    id: IdT {
                        local_name: INameT::Function(FunctionNameT {
                            template: FunctionTemplateNameT { human_name: StrI("as"), .. },
                            ..
                        }),
                        init_steps: &[],
                        ..
                    },
                    ..
                },
                args: [_],
                ..
            }) => Some((c.callable, c.args[0]))
        );

        let (as_prototype_template_args, as_prototype_params, as_prototype_return) =
            match as_prototype.id.local_name {
                INameT::Function(fn_name) => (fn_name.template_args, fn_name.parameters, as_prototype.return_type),
                other => panic!("expected Function name: {:?}", other),
            };

        match as_prototype_template_args {
            [
                ITemplataT::Coord(CoordTemplataT { coord: CoordT {
                    ownership: OwnershipT::Own,
                    kind: KindT::Struct(StructTT {
                        id: IdT {
                            init_steps: &[],
                            local_name: INameT::Struct(StructNameT {
                                template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Raza"), .. }),
                                template_args: &[],
                                ..
                            }),
                            ..
                        },
                        ..
                    }),
                    ..
                }}),
                ITemplataT::Coord(CoordTemplataT { coord: CoordT {
                    ownership: OwnershipT::Own,
                    kind: KindT::Interface(InterfaceTT {
                        id: IdT {
                            init_steps: &[],
                            local_name: INameT::Interface(InterfaceNameT {
                                template: InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                                template_args: &[],
                                ..
                            }),
                            ..
                        },
                        ..
                    }),
                    ..
                }}),
            ] => {}
            other => panic!("asPrototypeTemplateArgs: {:?}", other),
        }
        match as_prototype_params {
            [CoordT {
                ownership: OwnershipT::Borrow,
                kind: KindT::Interface(InterfaceTT {
                    id: IdT {
                        init_steps: &[],
                        local_name: INameT::Interface(InterfaceNameT {
                            template: InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                            template_args: &[],
                            ..
                        }),
                        ..
                    },
                    ..
                }),
                ..
            }] => {}
            other => panic!("asPrototypeParams: {:?}", other),
        }
        match as_prototype_return {
            CoordT {
                ownership: OwnershipT::Own,
                kind: KindT::Interface(InterfaceTT {
                    id: IdT {
                        init_steps: &[],
                        local_name: INameT::Interface(InterfaceNameT {
                            template: InterfaceTemplateNameT { human_namee: StrI("Result"), .. },
                            template_args: [
                                ITemplataT::Coord(CoordTemplataT { coord: CoordT {
                                    ownership: OwnershipT::Borrow,
                                    kind: KindT::Struct(StructTT {
                                        id: IdT {
                                            init_steps: &[],
                                            local_name: INameT::Struct(StructNameT {
                                                template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Raza"), .. }),
                                                template_args: &[],
                                                ..
                                            }),
                                            ..
                                        },
                                        ..
                                    }),
                                    ..
                                }}),
                                ITemplataT::Coord(CoordTemplataT { coord: CoordT {
                                    ownership: OwnershipT::Borrow,
                                    kind: KindT::Interface(InterfaceTT {
                                        id: IdT {
                                            init_steps: &[],
                                            local_name: INameT::Interface(InterfaceNameT {
                                                template: InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                                                template_args: &[],
                                                ..
                                            }),
                                            ..
                                        },
                                        ..
                                    }),
                                    ..
                                }}),
                            ],
                            ..
                        }),
                        ..
                    },
                    ..
                }),
                ..
            } => {}
            other => panic!("asPrototypeReturn: {:?}", other),
        }
        match as_arg.result().coord {
            CoordT {
                ownership: OwnershipT::Borrow,
                kind: KindT::Interface(InterfaceTT {
                    id: IdT {
                        init_steps: &[],
                        local_name: INameT::Interface(InterfaceNameT {
                            template: InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                            template_args: &[],
                            ..
                        }),
                        ..
                    },
                    ..
                }),
                ..
            } => {}
            other => panic!("asArg.result.coord: {:?}", other),
        }
    }

    {

        let as_funcs: Vec<_> = coutputs.functions.iter().filter(|f| {
            matches!(f.header.id.local_name, INameT::Function(FunctionNameT {
                template: FunctionTemplateNameT { human_name: StrI("as"), .. },
                parameters: [CoordT { ownership: OwnershipT::Borrow, .. }],
                ..
            }))
        }).copied().collect();
        let as_func = expect_1(&as_funcs);
        let as_ = crate::collect_only_tnode!(
            NodeRefT::FunctionDefinition(as_func),
            NodeRefT::AsSubtype(as_) => Some(as_)
        );
        let source_expr = as_.source_expr;
        let target_subtype = as_.target_type;
        let result_opt_type = as_.result_result_type;
        let ok_constructor = as_.ok_constructor;
        let err_constructor = as_.err_constructor;

        match source_expr.result().coord {
            CoordT {
                ownership: OwnershipT::Borrow,
                kind: KindT::KindPlaceholder(KindPlaceholderT {
                    id: IdT {
                        init_steps: [INameT::FunctionTemplate(FunctionTemplateNameT { human_name: StrI("as"), .. })],
                        local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                            template: KindPlaceholderTemplateNameT { index: 1, .. },
                        }),
                        ..
                    },
                    ..
                }),
                ..
            } => {}
            //case CoordT(BorrowT, InterfaceTT(FullNameT(_, Vector(), InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")), Vector())))) =>
            other => panic!("sourceExpr.result.coord: {:?}", other),
        }
        match target_subtype.kind {
            KindT::KindPlaceholder(KindPlaceholderT {
                id: IdT {
                    init_steps: [INameT::FunctionTemplate(FunctionTemplateNameT { human_name: StrI("as"), .. })],
                    local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                        template: KindPlaceholderTemplateNameT { index: 0, .. },
                    }),
                    ..
                },
                ..
            }) => {}
            KindT::Struct(StructTT {
                id: IdT {
                    init_steps: &[],
                    local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Raza"), .. }),
                        template_args: &[],
                        ..
                    }),
                    ..
                },
                ..
            }) => {}
            other => panic!("targetSubtype.kind: {:?}", other),
        }
        match result_opt_type {
            CoordT {
                ownership: OwnershipT::Own,
                kind: KindT::Interface(InterfaceTT {
                    id: IdT {
                        init_steps: &[],
                        local_name: INameT::Interface(InterfaceNameT {
                            template: InterfaceTemplateNameT { human_namee: StrI("Result"), .. },
                            template_args: [
                                ITemplataT::Coord(CoordTemplataT { coord: CoordT {
                                    ownership: OwnershipT::Borrow,
                                    kind: KindT::KindPlaceholder(KindPlaceholderT {
                                        id: IdT {
                                            local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                                                template: KindPlaceholderTemplateNameT { index: 0, .. },
                                            }),
                                            ..
                                        },
                                        ..
                                    }),
                                    ..
                                }}),
                                ITemplataT::Coord(CoordTemplataT { coord: CoordT {
                                    ownership: OwnershipT::Borrow,
                                    kind: KindT::KindPlaceholder(KindPlaceholderT {
                                        id: IdT {
                                            local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                                                template: KindPlaceholderTemplateNameT { index: 1, .. },
                                            }),
                                            ..
                                        },
                                        ..
                                    }),
                                    ..
                                }}),
                            ],
                            ..
                        }),
                        ..
                    },
                    ..
                }),
                ..
            } => {}
            other => panic!("resultOptType: {:?}", other),
        }
        assert_eq!(ok_constructor.id.local_name.parameters()[0], target_subtype);
        assert_eq!(err_constructor.id.local_name.parameters()[0], source_expr.result().coord);
    }
}
/*
  test("Downcast with as") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.as.*;
        |import v.builtins.logic.*;
        |import v.builtins.drop.*;
        |
        |interface IShip {}
        |
        |struct Raza { fuel int; }
        |impl IShip for Raza;
        |
        |exported func main() {
        |  ship IShip = Raza(42);
        |  ship.as<Raza>();
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    {
      val mainFunc = coutputs.lookupFunction("main")
      val (asPrototype, asArg) =
        Collector.only(mainFunc, {
          case FunctionCallTE(
          prototype @ PrototypeT(IdT(_,Vector(),FunctionNameT(FunctionTemplateNameT(StrI("as"),_),_,_)), _),
          Vector(arg),
          _) => {
            (prototype, arg)
          }
        })
      val (asPrototypeTemplateArgs, asPrototypeParams, asPrototypeReturn) =
        asPrototype match {
          case PrototypeT(IdT(_,Vector(),FunctionNameT(_, templateArgs, params)), retuurn) => {
            (templateArgs, params, retuurn)
          }
        }

      asPrototypeTemplateArgs match {
        case Vector(CoordTemplataT(CoordT(OwnT, _, StructTT(IdT(_,Vector(),StructNameT(StructTemplateNameT(StrI("Raza")),Vector()))))), CoordTemplataT(CoordT(OwnT, _, InterfaceTT(IdT(_,Vector(),InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")),Vector())))))) =>
      }

      asPrototypeParams match {
        case Vector(CoordT(BorrowT,_, InterfaceTT(IdT(_,Vector(),InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")),Vector()))))) =>
      }

      asPrototypeReturn match {
        case CoordT(
        OwnT,
        _,
        InterfaceTT(
        IdT(
        _,
        Vector(),
        InterfaceNameT(
        InterfaceTemplateNameT(StrI("Result")),
        Vector(
        CoordTemplataT(CoordT(BorrowT,_,StructTT(IdT(_,Vector(),StructNameT(StructTemplateNameT(StrI("Raza")),Vector()))))),
        CoordTemplataT(CoordT(BorrowT,_,InterfaceTT(IdT(_,Vector(),InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")),Vector())))))))))) =>
      }

      asArg.result.coord match {
        case CoordT(BorrowT,_, InterfaceTT(IdT(_,Vector(),InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")),Vector())))) =>
      }
    }

    {
      val asFunc =
        vassertOne(
          coutputs.functions.filter({
            case FunctionDefinitionT(FunctionHeaderT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("as"), _), _, Vector(CoordT(BorrowT, _,_)))), _, _, _, _), _, _) => true
            case _ => false
          }))
      val as = Collector.only(asFunc, { case as@AsSubtypeTE(_, _, _, _, _, _, _, _) => as })
      val AsSubtypeTE(sourceExpr, targetSubtype, resultOptType, okConstructor, errConstructor, _, _, _) = as
      sourceExpr.result.coord match {
        case CoordT(BorrowT,_, KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("as"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(1, _))))) =>
        //case CoordT(BorrowT, InterfaceTT(FullNameT(_, Vector(), InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")), Vector())))) =>
      }
      targetSubtype.kind match {
        case KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("as"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))) =>
        case StructTT(IdT(_, Vector(), StructNameT(StructTemplateNameT(StrI("Raza")), Vector()))) =>
      }
      val (firstGenericArg, secondGenericArg) =
        resultOptType match {
          case CoordT(
          OwnT,
          _,
          InterfaceTT(
          IdT(
          _, Vector(),
          InterfaceNameT(
          InterfaceTemplateNameT(StrI("Result")),
          Vector(firstGenericArg, secondGenericArg))))) => (firstGenericArg, secondGenericArg)
        }
      // They should both be pointers, since we dont really do borrows in structs yet
      firstGenericArg match {
        case CoordTemplataT(
        CoordT(
        BorrowT,
        _,
        KindPlaceholderT(
        IdT(_,Vector(FunctionTemplateNameT(StrI("as"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))))) =>
      }
      secondGenericArg match {
        case CoordTemplataT(
        CoordT(
        BorrowT,
        _,
        KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("as"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(1, _)))))) =>
      }
      vassert(okConstructor.paramTypes.head == targetSubtype)
      vassert(errConstructor.paramTypes.head == sourceExpr.result.coord)
    }
  }

*/
// mig: fn closure_using_parent_function_s_bound
#[test]
fn closure_using_parent_function_s_bound() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.arith.*;\n",
        "\n",
        "func genFunc<T>(a &T) T\n",
        "where func +(&T, &T)T {\n",
        "  { a + a }()\n",
        "}\n",
        "exported func main() int {\n",
        "  genFunc(7)\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    compile.expect_compiler_outputs();
}
/*
  test("Closure using parent function's bound") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arith.*;
        |
        |func genFunc<T>(a &T) T
        |where func +(&T, &T)T {
        |  { a + a }()
        |}
        |exported func main() int {
        |  genFunc(7)
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

*/
// mig: fn test_struct_default_generic_argument_in_call
#[test]
fn test_struct_default_generic_argument_in_call() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "struct MyHashSet<K Ref, H Int = 5> { }\n",
        "func moo() {\n",
        "  x = MyHashSet<bool>();\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let coutputs = compile.expect_compiler_outputs();
    let moo = coutputs.lookup_function_by_str("moo");
    let variable = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(moo),
        NodeRefT::LetNormal(let_normal) => Some(let_normal.variable)
    );
    match variable.coord() {
        CoordT {
            ownership: OwnershipT::Own,
            kind: KindT::Struct(StructTT {
                id: IdT {
                    local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(
                            StructTemplateNameT {
                                human_name: StrI("MyHashSet"),
                                ..
                            }
                        ),
                        template_args: [
                            ITemplataT::Coord(
                                CoordTemplataT {
                                    coord: CoordT { ownership: OwnershipT::Share, kind: KindT::Bool(_), .. }
                                }
                            ),
                            ITemplataT::Integer(5),
                        ],
                        ..
                    }),
                    ..
                },
                ..
            }),
            ..
        } => {}
        _ => panic!("unexpected coord"),
    }
}
/*
  test("Test struct default generic argument in call") {
    val compile = CompilerTestCompilation.test(
      """
        |struct MyHashSet<K Ref, H Int = 5> { }
        |func moo() {
        |  x = MyHashSet<bool>();
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupFunction("moo")
    val variable = Collector.only(moo, { case LetNormalTE(v, _) => v })
    variable.coord match {
      case CoordT(
      OwnT,
      _,
      StructTT(
      IdT(_,_,
      StructNameT(
      StructTemplateNameT(StrI("MyHashSet")),
      Vector(
      CoordTemplataT(CoordT(ShareT,_,BoolT())),
      IntegerTemplataT(5)))))) =>
    }
  }

*/
// mig: fn structs_can_resolve_other_structs_instantiation_bound_arguments
#[test]
fn structs_can_resolve_other_structs_instantiation_bound_arguments() {
    // The definition of Marine<T> was trying to resolve the existence of func drop(int)void.
    // Unfortunately, we don't have an overload index at the time of struct definitions yet, that comes later when
    // we define the functions.
    // Normally this wouldnt be a problem as we can usually use things before we compile them, we just use the templata
    // and solve the whole thing on our own, don't even need to know if it's been compiled yet.
    // However, now that we want to rely on the overload index, and the overload index doesn't exist until we compile
    // the functions, we rely on things being compiled before we use them, hence this problem.
    // The solution is to delay resolving function bounds until functions are compiled, see MCFBRBF.
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "import v.builtins.drop.*;\n",
        "\n",
        "struct XNone<T> where func drop(T)void { }\n",
        "\n",
        "// This function will try to do a resolve for func drop(int)void.\n",
        "struct Marine { weapon XNone<int>; }\n",
        "\n",
        "exported func main() {\n",
        "  m = Marine(XNone<int>());\n",
        "}\n",
    );
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Structs can resolve other structs' instantiation bound arguments") {
    // The definition of Marine<T> was trying to resolve the existence of func drop(int)void.
    // Unfortunately, we don't have an overload index at the time of struct definitions yet, that comes later when
    // we define the functions.
    // Normally this wouldnt be a problem as we can usually use things before we compile them, we just use the templata
    // and solve the whole thing on our own, don't even need to know if it's been compiled yet.
    // However, now that we want to rely on the overload index, and the overload index doesn't exist until we compile
    // the functions, we rely on things being compiled before we use them, hence this problem.
    // The solution is to delay resolving function bounds until functions are compiled, see MCFBRBF.

    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |struct XNone<T> where func drop(T)void { }
        |
        |// This function will try to do a resolve for func drop(int)void.
        |struct Marine { weapon XNone<int>; }
        |
        |exported func main() {
        |  m = Marine(XNone<int>());
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
  }
  ignore("Call rust builtin") { // Rust import pipeline not yet implemented
    val compile = CompilerTestCompilation.test(
      """
        |import rust.rstr;
        |
        |exported func main() {
        |  rstr("hello");
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
  ignore("Call rust free function") { // Rust import pipeline not yet implemented
    val compile = CompilerTestCompilation.test(
      """
        |import frust.std.fs.create_dir;
        |
        |exported func main() {
        |  create_dir();
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
  ignore("Import rust object") { // Rust import pipeline not yet implemented
    val compile = CompilerTestCompilation.test(
      """
        |import frust.std.vec.Vec;
        |
        |exported func main() {
        |  v = Vec<int>.new();
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
  test("Call member function") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Vec<T> {
        |  hp int;
        |  func new() Vec<T> { Vec<T>(42) }
        |}
        |exported func main() int {
        |  v = Vec<int>.new();
        |  return v.hp;
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
  // Minimal repro of the callsite rune-type-solver bug exposed by the named-arg channel.
  // The method references its container's T in one param (so higher-typing can solve T)
  // but has a trivial bool return and bool body, keeping the function's own solve simple.
  // Isolates the failure to the CALLSITE rune-type-solver step, which gets
  // MaybeCoercingLookupSR(int_rune, "int") and EqualsSR(T, int_rune) but no expected-type
  // seed for int_rune (because container args flow through the named-arg channel, not positional).
  test("Namespace method call only inherits container generic (minimal callsite rune-type repro)") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Vec<T> {
        |  func make(t &T) bool { true }
        |}
        |exported func main() bool {
        |  x = 42;
        |  return Vec<int>.make(&x);
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
  test("Namespace method call with both container and method generic args") {
    val compile = CompilerTestCompilation.test(
      """
        |struct S<T> {
        |  func foo<U>(x U) U { x }
        |}
        |exported func main() bool {
        |  return S<int>.foo<bool>(true);
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
  test("Namespace method call with multi-param container") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Pair<A, B> {
        |  func make() Pair<A, B> { Pair<A, B>() }
        |}
        |exported func main() {
        |  p = Pair<int, bool>.make();
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
  test("Namespace method call with defaulted container generic") {
    val compile = CompilerTestCompilation.test(
      """
        |struct MyVec<T Ref, N Int = 5> {
        |  func make() MyVec<T, N> { MyVec<T, N>() }
        |}
        |exported func main() {
        |  s = MyVec<bool>.make();
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
  ignore("Namespace method call with nested container chain") {
    // Parser does not yet support nested chains like `Outer<X>.Inner<Y>.foo()`.
    // When it does, this test exercises the multi-part walk in
    // ExpressionCompiler's FunctionCallSE(OutsideLoadSE(...)) arm.
    val compile = CompilerTestCompilation.test(
      """
        |struct Outer<X> {
        |  struct Inner<Y> {
        |    func make() Inner<Y> { Inner<Y>() }
        |  }
        |}
        |exported func main() {
        |  i = Outer<int>.Inner<bool>.make();
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
  ignore("Namespace method call with complex defaulted container generic") {
    // Like "Namespace method call with defaulted container generic" but the defaulted
    // generic's default is a COMPLEX templex (`int` translates into a
    // MaybeCoercingLookupSR + CoerceToCoordSR chain, introducing extra default-only
    // runes like coerceKindRune). Two latent gaps that combine to break this case:
    //   (1) `default.runeToType` only types `resultRune`; coerceKindRune is missing,
    //       so `commitStep`'s newRunes registration won't include it.
    //   (2) `MaybeCoercingLookupSR` reaches the solver instead of being converted
    //       to `LookupSR` via `explicifyLookups` first — the solver throws
    //       `MatchError` in `getPuzzles` (CompilerSolver.scala:238) because it
    //       has no case for it.
    // The simple-defaulted variant works because `LiteralSR(rune, IntLiteralSL(5))`
    // is fully explicit — no MaybeCoercing variant, no unregistered runes.
    val compile = CompilerTestCompilation.test(
      """
        |struct MyBox<U Ref, T Ref = int> {
        |  func make() MyBox<U, T> { MyBox<U, T>() }
        |}
        |exported func main() {
        |  m = MyBox<bool>.make();
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
  ignore("Namespace method call with defaulted container generic AND method generic args") {
    // Latent positional-misalignment gap in the simplified flatten approach.
    //
    // ExpressionCompiler builds explicitTemplateArgRunesS via
    //   parts.flatMap(_.explicitArgs).map(_.rune)
    // which positionally zips against the function's identifying-rune-types in
    // attemptCandidateBanner. Container-method calls have a syntactic split: the
    // container's args sit in parts[0], the method's own args in parts.last. When
    // the container has a TRAILING defaulted param AND the method has its own
    // template args, the user-supplied args are non-contiguous over the function's
    // identifying-rune sequence — and the positional zip can't represent
    // "skip middle (defaulted)".
    //
    // Concrete example: struct S<T, M Mutability = mut> { func foo<F>(...) ... }
    // call S<int>.foo<bool>(...).
    //   parts[0].explicitArgs = [int_rune] (T supplied; M defaulted)
    //   parts.last.explicitArgs = [bool_rune] (F supplied)
    //   flat: [int_rune, bool_rune]
    //   foo's identifying runes: [T, M, F]
    //   zip: [(int, T-type), (bool, M-type)] — bool lands on M instead of F.
    //
    // The current flatten approach works for:
    //   - All-supplied:   S<int, mut>.foo<F>()   ✓
    //   - Trailing-default-only: MyVec<bool>.make() (no method <args>)   ✓
    //   - Method-only:    Pair<int, bool>.make()   ✓
    // It breaks for the case below because the defaulted middle slot can't be
    // skipped positionally.
    //
    // Fix would require either a map-keyed channel for explicit template args,
    // or padding the container's args to its full identifying-rune length using
    // skip-sentinel runes that the explicit-template-args-solve leaves unsolved
    // so the call-solve's default-apply loop can fire.
    val compile = CompilerTestCompilation.test(
      """
        |struct S<T Ref, M Mutability = mut> {
        |  func foo<F Ref>(self &S<T>, f F) F { f }
        |}
        |exported func main() bool {
        |  s = S<int>();
        |  return s.foo<bool>(true);
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
  ignore("Struct internal method inheriting function-typed default") {
    // Exercises function-typed bound default inheritance. Functor1's F generic
    // param has a function-typed default `func(P1)R` whose translated rules
    // include bound-contract rules (PackSR / CallSiteFuncSR / DefinitionFuncSR).
    // Per the @SRHODP audit those rules stay hoisted into Functor1's main rules
    // (always needed for the parent's own solve, default-firing or not).
    //
    // The existing "Test single parameter function" test (CompilerSolverTests.scala:139)
    // works because Functor1's body is empty and `__call` is a free function that
    // re-declares F with its own default, getting the bound-contract rules locally.
    //
    // This test adds an INTERNAL method `use` to Functor1's body that uses F.
    // Currently fails — the precise failure mode depends on how
    // FunctionScout/StructCompiler propagate generic params and bound-contract
    // rules to internal methods (see commit 25a0202f's discussion of "function-typed
    // bound default inheritance" follow-up). Will be unblocked when the
    // bound-contract rules are made available to inheriting methods, e.g. via
    // extending Option 1's pattern to PackSR/CallSiteFuncSR/DefinitionFuncSR.
    val compile = CompilerTestCompilation.test(
      """
        |struct Functor1<F Prot = func(P1)R> imm
        |where P1 Ref, R Ref {
        |  func use(self &Functor1<F>, x P1) R { F(x) }
        |}
        |
        |func __call<F Prot = func(P1)R>(self &Functor1<F>, param1 P1) R
        |where P1 Ref, R Ref {
        |  F(param1)
        |}
        |
        |exported func main() int {
        |  Functor1({_})(4)
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
  ignore("Reports WrongNumberOfTemplateArguments when namespace method call has too many positional args for method's own runes") {
    // Logic gap in OverloadResolver.attemptCandidateBanner:
    // Currently blocked by FunctionScout.scala:114 (`vassert(userDeclaredRunes.isEmpty)`)
    // which rejects internal methods that have their own runes (like `<N>` here).
    // Once that assertion is lifted (same blocker as "Namespace method call with both
    // container and method generic args"), this test should then fail with a match error
    // because the current check uses identifyingRuneTemplataTypes.size (2) instead of
    // the correct own-rune count (1). Fix: subtract receivingRuneToExplicitTemplateArgRune.size.
    //
    //   if (positionalExplicitTemplateArgRunesS.size > identifyingRuneTemplataTypes.size)
    // After @PRIIROZ, a method `func zork<N>` inside `struct S<K>` has identifying runes
    // [N, K] (size 2). The check allows up to 2 positional args, but only 1 (N) is the
    // method's own — K is supplied via the container prefix as an EqualsSR rule.
    // Calling S<int>.zork<float, bool>() supplies 2 positional args when only 1 is valid.
    // The correct check is:
    //   positionalExplicitTemplateArgRunesS.size > identifyingRuneTemplataTypes.size - receivingRuneToExplicitTemplateArgRune.size
    val compile = CompilerTestCompilation.test(
      """
        |struct S<K> {
        |  func zork<N>(x N) N { x }
        |}
        |exported func main() int {
        |  // Two positional args, but zork only has 1 own rune (N); K comes from the container prefix.
        |  S<int>.zork<int, bool>(42)
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CouldntFindFunctionToCallT(_, fff)) => {
        vassert(fff.rejectedCalleeToReason.size == 1)
        fff.rejectedCalleeToReason.head._2 match {
          case WrongNumberOfTemplateArguments(2, 1) =>
        }
      }
    }
  }
}
*/
