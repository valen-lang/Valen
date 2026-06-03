use bumpalo::Bump;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::utils::code_hierarchy::{self, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use crate::interner::StrI;
use crate::postparsing::names::{IImpreciseNameS, CodeNameS};
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::overload_resolver::FindFunctionFailure;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::names::names::{IFunctionNameT, INameT};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::{CoordT, KindT, OwnershipT};
use crate::typing::ast::expressions::ReferenceExpressionTE;
use crate::typing::ast::expressions::ConstantIntTE;
use crate::typing::types::types::IntT;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::typing::ast::expressions::BlockTE;
use crate::typing::ast::expressions::ConsecutorTE;
use crate::typing::ast::expressions::ReturnTE;
use crate::typing::ast::expressions::VoidLiteralTE;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::source_code_utils::{humanize_pos_code_map, line_containing, line_range_containing, lines_between};
use crate::postparsing::names::{CodeRuneS, IRuneS, IRuneValS};
use crate::postparsing::ast::LocationInDenizenBuilder;
use crate::postparsing::rules::rules::{CoordComponentsSR, IRulexSR, KindComponentsSR, RuneUsage};
use crate::solver::solver::{FailedSolve, ISolverError, RuleError, SolveIncomplete, Step};
use crate::typing::ast::ast::SignatureT;
use crate::typing::compiler_error_humanizer::humanize;
use crate::typing::infer::compiler_solver::ITypingPassSolverError;
use crate::typing::names::names::FunctionNameValT;
use crate::typing::names::names::FunctionTemplateNameT;
use crate::typing::names::names::IdValT;
use crate::typing::names::names::InterfaceNameValT;
use crate::typing::names::names::InterfaceTemplateNameT;
use crate::typing::names::names::IStructTemplateNameT;
use crate::typing::names::names::KindPlaceholderNameT;
use crate::typing::names::names::KindPlaceholderTemplateNameT;
use crate::typing::names::names::StructNameValT;
use crate::typing::names::names::StructTemplateNameT;
use crate::typing::templata::templata::OwnershipTemplataT;
use crate::typing::types::types::InterfaceTTValT;
use crate::typing::types::types::{IRegionT, RegionT};
use crate::typing::types::types::StructTTValT;
use crate::typing::ast::expressions::UpcastTE;
use crate::postparsing::names::{IStructDeclarationNameS, TopLevelStructDeclarationNameS};
use crate::higher_typing::ast::StructA;
use crate::solver::solver::SolverConflict;
use crate::typing::names::names::IdT;
use crate::typing::names::names::StructNameT;
use crate::typing::templata::templata::StructDefinitionTemplataT;
use crate::typing::templata::templata::KindTemplataT;
use crate::typing::types::types::StructTT;
use crate::typing::templata::templata::CoordTemplataT;
use crate::typing::test::traverse::NodeRefT;
use crate::postparsing::names::INameValS;
use crate::postparsing::names::IFunctionDeclarationNameValS;
use crate::postparsing::names::FunctionNameS;
use crate::postparsing::names::DenizenDefaultRegionRuneS;
use crate::typing::names::names::ExportTemplateNameT;
use crate::typing::names::names::ExportNameT;
use crate::typing::ast::ast::KindExportT;
use crate::utils::code_hierarchy::FileCoordinate;
use crate::postparsing::names::ImplicitRuneValS;
use crate::typing::types::types::ISuperKindTT;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::names::names::FunctionNameT;
use crate::typing::names::names::FunctionBoundNameT;
use crate::typing::names::names::FunctionBoundTemplateNameT;
use crate::typing::types::types::KindPlaceholderT;
/*
package dev.vale.typing

import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.expression.CallCompiler
import dev.vale._
import dev.vale.highertyping._
import dev.vale.typing.types._
import dev.vale._
import dev.vale.postparsing._
import dev.vale.postparsing.rules._
import dev.vale.solver.{FailedSolve, RuleError, SolveIncomplete, SolverConflict, Step}
import OverloadResolver.{FindFunctionFailure, InferFailure, SpecificParamDoesntSend, WrongNumberOfArguments}
import dev.vale.Collector.ProgramWithExpect
import dev.vale.postparsing._
import dev.vale.typing.ast._
import dev.vale.typing.infer._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.ast._
import dev.vale.typing.templata._
import dev.vale.typing.types._
//import dev.vale.typingpass.infer.NotEnoughToSolveError
import org.scalatest._

import scala.io.Source

class CompilerSolverTests extends FunSuite with Matchers {
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
// mig: fn test_simple_generic_function
#[test]
fn test_simple_generic_function() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc bork<T>(a T) T { return a; }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();

    assert_eq!(coutputs.get_all_user_functions().len(), 1);
}
/*
  test("Test simple generic function") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<T>(a T) T { return a; }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    vassert(coutputs.getAllUserFunctions.size == 1)
  }
*/
// mig: fn test_lacking_drop_function
#[test]
fn test_lacking_drop_function() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc bork<T>(a T) { }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CouldntFindFunctionToCallT { fff: FindFunctionFailure { name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("drop") }), .. }, .. } => {}
        _ => panic!("expected CouldntFindFunctionToCallT with FindFunctionFailure(CodeNameS(\"drop\"))"),
    }
}
/*
  test("Test lacking drop function") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<T>(a T) { }
      """.stripMargin)
    compile.getCompilerOutputs().expectErr() match {
      case CouldntFindFunctionToCallT(_, FindFunctionFailure(CodeNameS(StrI("drop")), _, _)) =>
    }
  }
*/
// mig: fn test_having_drop_function_concept_function
#[test]
fn test_having_drop_function_concept_function() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc bork<T>(a T) where func drop(T)void { }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let bork = coutputs.lookup_function_by_str("bork");

    // Only identifying template arg coord should be of PlaceholderT(0)
    let template_args = IFunctionNameT::try_from(bork.header.id.local_name).unwrap().template_args();
    assert_eq!(template_args.len(), 1);
    match template_args[0] {
        ITemplataT::Coord(ct) => match ct.coord {
            CoordT { ownership: OwnershipT::Own, kind: KindT::KindPlaceholder(kp), .. } => match kp.id.local_name {
                INameT::KindPlaceholder(kpn) => assert_eq!(kpn.template.index, 0),
                _ => panic!("expected KindPlaceholder local_name"),
            },
            _ => panic!("expected CoordT with KindPlaceholder"),
        },
        _ => panic!("expected Coord template arg"),
    }

    // Make sure it calls drop, and that it has the right placeholders
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(bork),
        NodeRefT::FunctionCall(FunctionCallTE {
            callable: PrototypeT {
                id: IdT {
                    local_name: INameT::FunctionBound(FunctionBoundNameT {
                        template: FunctionBoundTemplateNameT { human_name: StrI("drop"), .. },
                        template_args: &[],
                        parameters: &[CoordT {
                            ownership: OwnershipT::Own,
                            kind: KindT::KindPlaceholder(KindPlaceholderT {
                                id: IdT {
                                    init_steps: &[INameT::FunctionTemplate(FunctionTemplateNameT { human_name: StrI("bork"), .. })],
                                    local_name: INameT::KindPlaceholder(KindPlaceholderNameT {
                                        template: KindPlaceholderTemplateNameT { index: 0, .. },
                                    }),
                                    ..
                                },
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
        }) => Some(())
    );
}
/*
Guardian: temp-disable: SPDMX — In Scala, `header.id` is statically `IdT[IFunctionNameT]` so `templateArgs` is defined on `IFunctionNameT`, not on `INameT`. Rust uses the +T erasure pattern (design v3 §6.0) where `IdT.local_name: INameT` is widened and use sites narrow via `TryFrom<INameT>` — precedents at infer_compiler.rs:618, templata_compiler.rs:277/1564, overload_resolver.rs:716, ast.rs:1027. — /Volumes/V/Sylvan/FrontendRust/guardian-logs/request-1232-1779031866703/hook-1232/test_having_drop_function_concept_function--123.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  test("Test having drop function concept function") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<T>(a T) where func drop(T)void { }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val bork = coutputs.lookupFunction("bork")

    // Only identifying template arg coord should be of PlaceholderT(0)
    bork.header.id.localName.templateArgs match {
      case Vector(CoordTemplataT(CoordT(OwnT,_, KindPlaceholderT(IdT(_, _, KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))))))
      =>
    }

    // Make sure it calls drop, and that it has the right placeholders
    bork.body shouldHave {
      case FunctionCallTE(
        PrototypeT(
          IdT(
            _,
            _,
            FunctionBoundNameT(
              FunctionBoundTemplateNameT(StrI("drop")),
              Vector(),
              Vector(
                CoordT(
                  OwnT,
                  _,
                  KindPlaceholderT(
                    IdT(
                      _,
                      Vector(FunctionTemplateNameT(StrI("bork"),_)),
                      KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))))))),
          CoordT(ShareT,_, VoidT())),
        _,
        _) =>
    }
  }
*/
// mig: fn test_calling_a_generic_function_with_a_concept_function
#[test]
fn test_calling_a_generic_function_with_a_concept_function() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc moo(x int) { }\n\nfunc bork<T>(a T) T where func moo(T)void { a }\n\nexported func main() {\n  bork(3);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");

    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(FunctionCallTE {
            callable: PrototypeT {
                id: IdT {
                    local_name: INameT::Function(FunctionNameT {
                        template: FunctionTemplateNameT { human_name: StrI("bork"), .. },
                        template_args: &[ITemplataT::Coord(CoordTemplataT {
                            coord: CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT::I32), .. },
                        })],
                        parameters: &[CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT::I32), .. }],
                        ..
                    }),
                    ..
                },
                return_type: CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT::I32), .. },
            },
            args: &[ReferenceExpressionTE::ConstantInt(ConstantIntTE { value: ITemplataT::Integer(3), bits: 32, .. })],
            ..
        }) => Some(())
    );
}
/*
  test("Test calling a generic function with a concept function") {
    val compile = CompilerTestCompilation.test(
      """
        |func moo(x int) { }
        |
        |func bork<T>(a T) T where func moo(T)void { a }
        |
        |exported func main() {
        |  bork(3);
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    main shouldHave {
      case FunctionCallTE(
        PrototypeT(
          IdT(_,
            _,
            FunctionNameT(
              FunctionTemplateNameT(StrI("bork"), _),
              Vector(CoordTemplataT(CoordT(ShareT,RegionT(DefaultRegionT), IntT(32)))),
              Vector(CoordT(ShareT,RegionT(DefaultRegionT), IntT(32))))),
          CoordT(ShareT,RegionT(DefaultRegionT), IntT(32))),
        Vector(ConstantIntTE(IntegerTemplataT(3),32, _)),
        _) =>
    }
  }
*/
// mig: fn test_rune_type_in_generic_param
#[test]
fn test_rune_type_in_generic_param() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc bork<I Int>() int { I }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("bork");
    let template_args = IFunctionNameT::try_from(main.header.id.local_name).unwrap().template_args();
    match template_args {
        [ITemplataT::Placeholder(p)] => match p.tyype {
            ITemplataType::IntegerTemplataType(_) => {}
            _ => panic!("expected IntegerTemplataType"),
        },
        _ => panic!("expected Vector(PlaceholderTemplataT(_, IntegerTemplataType()))"),
    }
}
/*
Guardian: temp-disable: SPDMX — In Scala, `header.id` is statically `IdT[IFunctionNameT]` so `templateArgs` is defined on `IFunctionNameT`, not on `INameT`. Rust uses the +T erasure pattern (design v3 §6.0) where `IdT.local_name: INameT` is widened and use sites narrow via `TryFrom<INameT>` — precedents at infer_compiler.rs:618, templata_compiler.rs:277/1564, overload_resolver.rs:716, ast.rs:1027. — /Volumes/V/Sylvan/FrontendRust/guardian-logs/request-1250-1779032498314/hook-1250/test_rune_type_in_generic_param--332.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  test("Test rune type in generic param") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<I Int>() int { I }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("bork")
    main.header.id.localName.templateArgs match {
      case Vector(PlaceholderTemplataT(_, IntegerTemplataType())) =>
    }
  }
*/
// mig: fn test_single_parameter_function
#[test]
fn test_single_parameter_function() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nstruct Functor1<F Prot = func(P1)R> imm\nwhere P1 Ref, R Ref { }\n\nfunc __call<F Prot = func(P1)R>(self &Functor1<F>, param1 P1) R\nwhere P1 Ref, R Ref {\n  F(param1)\n}\n\nexported func main() int {\n  Functor1({_})(4)\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let _compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
}
/*
  test("Test single parameter function") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Functor1<F Prot = func(P1)R> imm
        |where P1 Ref, R Ref { }
        |
        |func __call<F Prot = func(P1)R>(self &Functor1<F>, param1 P1) R
        |where P1 Ref, R Ref {
        |  F(param1)
        |}
        |
        |exported func main() int {
        |  Functor1({_})(4)
        |}
      """.stripMargin)

  }
*/
// mig: fn test_calling_a_generic_function_with_a_drop_concept_function
#[test]
fn test_calling_a_generic_function_with_a_drop_concept_function() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc bork<T>(a T) where func drop(T)void {\n}\n\nstruct Mork {}\n\nexported func main() {\n  bork(Mork());\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let bork = coutputs.lookup_function_by_str("main");
    let prototype = match bork.body {
        ReferenceExpressionTE::Block(BlockTE { inner }) => match inner {
            ReferenceExpressionTE::Return(ReturnTE { source_expr }) => match source_expr {
                ReferenceExpressionTE::Consecutor(ConsecutorTE { exprs }) => match exprs {
                    [ReferenceExpressionTE::FunctionCall(call), ReferenceExpressionTE::VoidLiteral(VoidLiteralTE { .. })] => call.callable,
                    _ => panic!("expected Vector(FunctionCallTE, VoidLiteralTE)"),
                },
                _ => panic!("expected ConsecutorTE"),
            },
            _ => panic!("expected ReturnTE"),
        },
        _ => panic!("expected BlockTE"),
    };
    match prototype.id.local_name {
        INameT::Function(fn_name) => {
            assert_eq!(fn_name.template.human_name.0, "bork");
            let template_arg_coord = match fn_name.template_args {
                [ITemplataT::Coord(ct)] => ct.coord,
                _ => panic!("expected Vector(CoordTemplataT(templateArgCoord))"),
            };
            let arg = match fn_name.parameters {
                [a] => *a,
                _ => panic!("expected Vector(arg)"),
            };
            match prototype.return_type {
                CoordT { ownership: OwnershipT::Share, kind: KindT::Void(_), .. } => {}
                _ => panic!("expected Share Void return_type"),
            }
            match template_arg_coord {
                CoordT { ownership: OwnershipT::Own, kind: KindT::Struct(stt), .. } => match stt.id.local_name {
                    INameT::Struct(sn) => match sn.template {
                        IStructTemplateNameT::StructTemplate(st) => assert_eq!(st.human_name.0, "Mork"),
                        _ => panic!("expected StructTemplate"),
                    },
                    _ => panic!("expected Struct local_name"),
                },
                _ => panic!("expected Own Struct(Mork)"),
            }
            assert_eq!(arg, template_arg_coord);
        }
        _ => panic!("expected Function local_name"),
    }
}
/*
  test("Test calling a generic function with a drop concept function") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<T>(a T) where func drop(T)void {
        |}
        |
        |struct Mork {}
        |
        |exported func main() {
        |  bork(Mork());
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val bork = coutputs.lookupFunction("main")
    val prototype =
      bork.body match {
        case BlockTE(
            ReturnTE(
              ConsecutorTE(
                Vector(FunctionCallTE(prototype, _, _),
                VoidLiteralTE(_))))) => prototype
      }
    prototype match {
      case PrototypeT(
          IdT(
            _,_,
            FunctionNameT(
              FunctionTemplateNameT(StrI("bork"), _),
              Vector(CoordTemplataT(templateArgCoord)),
              Vector(arg))),
          CoordT(ShareT,RegionT(DefaultRegionT), VoidT())) => {

        templateArgCoord match {
          case CoordT(
              OwnT,
          _,
          StructTT(
                IdT(_,_,
                  StructNameT(StructTemplateNameT(StrI("Mork")),Vector())))) =>
        }

        vassert(arg == templateArgCoord)
      }
    }
  }
*/
// mig: fn humanize_errors
#[test]
fn humanize_errors() {

    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let _keywords = Keywords::new_for_scout(&scout_arena);
    let typing_interner = TypingInterner::new(&typing_bump);

    let tz = vec![RangeS::test_zero(&scout_arena)];
    let test_package_coord = scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]);
    let tz_code_loc = CodeLocationS::test_zero(&scout_arena);
    let func_template_name = typing_interner.intern_function_template_name(FunctionTemplateNameT { human_name: scout_arena.intern_str("main"), code_location: tz_code_loc, _phantom: std::marker::PhantomData });
    let func_template_id = typing_interner.intern_id(IdValT { package_coord: test_package_coord, init_steps: &[], local_name: INameT::FunctionTemplate(func_template_name) });
    let main_func_template = typing_interner.intern_function_template_name(FunctionTemplateNameT { human_name: scout_arena.intern_str("main"), code_location: tz_code_loc, _phantom: std::marker::PhantomData });
    let main_func_name = typing_interner.intern_function_name(FunctionNameValT { template: main_func_template, template_args: &[], parameters: &[] });
    let _func_name = typing_interner.intern_id(IdValT { package_coord: test_package_coord, init_steps: &[], local_name: INameT::Function(main_func_name) });
    let denizen_name_s = scout_arena.intern_name(INameValS::FunctionDeclaration(IFunctionDeclarationNameValS::FunctionName(FunctionNameS { name: scout_arena.intern_str("main"), code_location: tz_code_loc })));
    let denizen_default_region_rune_s = DenizenDefaultRegionRuneS { denizen_name: denizen_name_s };
    let kpt_name = typing_interner.intern_kind_placeholder_template_name(KindPlaceholderTemplateNameT { index: 0, rune: IRuneS::DenizenDefaultRegionRune(scout_arena.alloc(denizen_default_region_rune_s)), _phantom: std::marker::PhantomData });
    let kp_name = typing_interner.intern_kind_placeholder_name(KindPlaceholderNameT { template: kpt_name });
    let mut region_init_steps: Vec<INameT> = func_template_id.init_steps.to_vec();
    region_init_steps.push(func_template_id.local_name);
    let region_init_steps_slice: &[INameT] = typing_bump.alloc_slice_copy(&region_init_steps);
    let _region_name = typing_interner.intern_id(IdValT { package_coord: func_template_id.package_coord, init_steps: region_init_steps_slice, local_name: INameT::KindPlaceholder(kp_name) });
    let region = RegionT { region: IRegionT::Default };

    let firefly_struct_template_name = typing_interner.intern_struct_template_name(
        StructTemplateNameT { human_name: scout_arena.intern_str("Firefly"), _phantom: std::marker::PhantomData });
    let firefly_struct_name = typing_interner.intern_struct_name(
        StructNameValT { template: IStructTemplateNameT::StructTemplate(firefly_struct_template_name), template_args: &[] });
    let firefly_id = typing_interner.intern_id(IdValT { package_coord: test_package_coord, init_steps: &[], local_name: INameT::Struct(firefly_struct_name) });
    let firefly_tt = typing_interner.intern_struct_tt(StructTTValT { id: *firefly_id });
    let firefly_kind = KindT::Struct(firefly_tt);
    let _firefly_coord = CoordT { ownership: OwnershipT::Own, region, kind: firefly_kind };

    let serenity_struct_template_name = typing_interner.intern_struct_template_name(
        StructTemplateNameT { human_name: scout_arena.intern_str("Serenity"), _phantom: std::marker::PhantomData });
    let serenity_struct_name = typing_interner.intern_struct_name(
        StructNameValT { template: IStructTemplateNameT::StructTemplate(serenity_struct_template_name), template_args: &[] });
    let serenity_id = typing_interner.intern_id(IdValT { package_coord: test_package_coord, init_steps: &[], local_name: INameT::Struct(serenity_struct_name) });
    let serenity_tt = typing_interner.intern_struct_tt(StructTTValT { id: *serenity_id });
    let serenity_kind = KindT::Struct(serenity_tt);
    let _serenity_coord = CoordT { ownership: OwnershipT::Own, region, kind: serenity_kind };

    let ispaceship_interface_template_name = typing_interner.intern_interface_template_name(
        InterfaceTemplateNameT { human_namee: scout_arena.intern_str("ISpaceship"), _phantom: std::marker::PhantomData });
    let ispaceship_interface_name = typing_interner.intern_interface_name(
        InterfaceNameValT { template: ispaceship_interface_template_name, template_args: &[] });
    let ispaceship_id = typing_interner.intern_id(IdValT { package_coord: test_package_coord, init_steps: &[], local_name: INameT::Interface(ispaceship_interface_name) });
    let ispaceship_tt = typing_interner.intern_interface_tt(InterfaceTTValT { id: *ispaceship_id });
    let ispaceship_kind = KindT::Interface(ispaceship_tt);
    let _ispaceship_coord = CoordT { ownership: OwnershipT::Own, region, kind: ispaceship_kind };

    let unrelated_struct_template_name = typing_interner.intern_struct_template_name(
        StructTemplateNameT { human_name: scout_arena.intern_str("Spoon"), _phantom: std::marker::PhantomData });
    let unrelated_struct_name = typing_interner.intern_struct_name(
        StructNameValT { template: IStructTemplateNameT::StructTemplate(unrelated_struct_template_name), template_args: &[] });
    let unrelated_id = typing_interner.intern_id(IdValT { package_coord: test_package_coord, init_steps: &[], local_name: INameT::Struct(unrelated_struct_name) });
    let unrelated_tt = typing_interner.intern_struct_tt(StructTTValT { id: *unrelated_id });
    let unrelated_kind = KindT::Struct(unrelated_tt);
    let _unrelated_coord = CoordT { ownership: OwnershipT::Own, region, kind: unrelated_kind };

    let myfunc_template = typing_interner.intern_function_template_name(FunctionTemplateNameT { human_name: scout_arena.intern_str("myFunc"), code_location: tz[0].begin, _phantom: std::marker::PhantomData });
    let myfunc_params: &[CoordT] = typing_bump.alloc_slice_copy(&[CoordT { ownership: OwnershipT::Own, region, kind: firefly_kind }]);
    let myfunc_name = typing_interner.intern_function_name(FunctionNameValT { template: myfunc_template, template_args: &[], parameters: myfunc_params });
    let myfunc_id = typing_interner.intern_id(IdValT { package_coord: test_package_coord, init_steps: &[], local_name: INameT::Function(myfunc_name) });
    let _firefly_signature = SignatureT { id: *myfunc_id };

    let export_template_name = typing_interner.intern_export_template_name(ExportTemplateNameT { code_loc: tz[0].begin, _phantom: std::marker::PhantomData });
    let firefly_export_name = typing_interner.intern_export_name(ExportNameT { template: export_template_name, region: RegionT { region: IRegionT::Default } });
    let firefly_export_id = typing_interner.intern_id(IdValT { package_coord: test_package_coord, init_steps: &[], local_name: INameT::Export(firefly_export_name) });
    let _firefly_export = KindExportT { range: tz[0], tyype: firefly_kind, id: *firefly_export_id, exported_name: scout_arena.intern_str("Firefly") };
    let serenity_export_name = typing_interner.intern_export_name(ExportNameT { template: export_template_name, region: RegionT { region: IRegionT::Default } });
    let serenity_export_id = typing_interner.intern_id(IdValT { package_coord: test_package_coord, init_steps: &[], local_name: INameT::Export(serenity_export_name) });
    let _serenity_export = KindExportT { range: tz[0], tyype: firefly_kind, id: *serenity_export_id, exported_name: scout_arena.intern_str("Serenity") };

    let code_str = "Hello I am A large piece Of code [that has An error]";
    let filenames_and_sources = FileCoordinateMap::test(&scout_arena, code_str.to_string());

    let test_file = FileCoordinate::test(&scout_arena);
    let make_loc = |pos: i32| CodeLocationS { file: scout_arena.alloc(test_file), offset: pos };
    let make_range = |begin: i32, end: i32| RangeS { begin: make_loc(begin), end: make_loc(end) };

    let humanize_pos = |x| humanize_pos_code_map(&filenames_and_sources, &x);
    let lines_between_f = |x, y| lines_between(&filenames_and_sources, &x, &y);
    let line_range_containing_f = |x| line_range_containing(&filenames_and_sources, &x);
    let line_containing_f = |x| line_containing(&filenames_and_sources, &x);

    let rune_i = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: scout_arena.intern_str("I") }));
    let rune_a = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: scout_arena.intern_str("A") }));
    let rune_an = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: scout_arena.intern_str("An") }));
    let rune_of = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: scout_arena.intern_str("Of") }));
    let mut lid_builder = LocationInDenizenBuilder::new(vec![7]);
    let implicit_rune = scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lid_builder.borrow_val())));

    let unsolved_rules: Vec<IRulexSR> = vec![
        IRulexSR::CoordComponents(CoordComponentsSR {
            range: make_range(0, code_str.len() as i32),
            result_rune: RuneUsage { range: make_range(6, 7), rune: rune_i },
            ownership_rune: RuneUsage { range: make_range(11, 12), rune: rune_a },
            kind_rune: RuneUsage { range: make_range(33, 52), rune: implicit_rune },
        }),
        IRulexSR::KindComponents(KindComponentsSR {
            range: make_range(33, 52),
            kind_rune: RuneUsage { range: make_range(33, 52), rune: implicit_rune },
            mutability_rune: RuneUsage { range: make_range(43, 45), rune: rune_an },
        }),
    ];

    let step1 = {
        let mut conclusions = std::collections::HashMap::new();
        conclusions.insert(rune_a, ITemplataT::Ownership(OwnershipTemplataT { ownership: OwnershipT::Own }));
        Step { complex: false, solved_rules: vec![], added_rules: vec![], conclusions }
    };

    let failed_solve_1 = FailedSolve {
        steps: vec![step1.clone()],
        conclusions: std::collections::HashMap::new(),
        unsolved_rules: unsolved_rules.clone(),
        unsolved_runes: vec![],
        error: ISolverError::RuleError(RuleError {
            err: ITypingPassSolverError::KindIsNotConcrete { kind: ispaceship_kind },
            _phantom: std::marker::PhantomData,
        }),
    };
    let text1 = humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between_f, &line_range_containing_f, &line_containing_f,
        ICompileErrorT::TypingPassSolverError { range: typing_bump.alloc_slice_copy(&tz), failed_solve: failed_solve_1 });
    assert!(!text1.is_empty());

    let mut conclusions2 = std::collections::HashMap::new();
    conclusions2.insert(rune_a, ITemplataT::Ownership(OwnershipTemplataT { ownership: OwnershipT::Own }));
    let failed_solve_2 = FailedSolve {
        steps: vec![step1],
        conclusions: conclusions2,
        unsolved_rules,
        unsolved_runes: vec![rune_i, rune_of, rune_an, implicit_rune],
        error: ISolverError::SolveIncomplete(SolveIncomplete { _phantom: std::marker::PhantomData }),
    };
    let error_text = humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between_f, &line_range_containing_f, &line_containing_f,
        ICompileErrorT::TypingPassSolverError { range: typing_bump.alloc_slice_copy(&tz), failed_solve: failed_solve_2 });
    println!("{}", error_text);
    assert!(!error_text.is_empty());
    assert!(error_text.contains("\n           ^ A: own"), "missing A:own caret line, got: {}", error_text);
    assert!(error_text.contains("\n      ^ I: (unknown)"), "missing I:(unknown) caret line, got: {}", error_text);
    assert!(error_text.contains("\n                                 ^^^^^^^^^^^^^^^^^^^ _7: (unknown)"), "missing _7:(unknown) caret line, got: {}", error_text);
}
/*
  test("Humanize errors") {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val tz = List(RangeS.testZero(interner))
    val testPackageCoord = PackageCoordinate.TEST_TLD(interner, keywords)
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
    val fireflySignature = SignatureT(IdT(testPackageCoord, Vector(), interner.intern(FunctionNameT(interner.intern(FunctionTemplateNameT(interner.intern(StrI("myFunc")), tz.head.begin)), Vector(), Vector(fireflyCoord)))))
    val fireflyExportId = IdT(testPackageCoord, Vector(), interner.intern(ExportNameT(interner.intern(ExportTemplateNameT(tz.head.begin)), RegionT(DefaultRegionT))))
    val fireflyExport = KindExportT(tz.head, fireflyKind, fireflyExportId, interner.intern(StrI("Firefly")));
    val serenityExportId = IdT(testPackageCoord, Vector(), interner.intern(ExportNameT(interner.intern(ExportTemplateNameT(tz.head.begin)), RegionT(DefaultRegionT))))
    val serenityExport = KindExportT(tz.head, fireflyKind, serenityExportId, interner.intern(StrI("Serenity")));

    val codeStr = "Hello I am A large piece Of code [that has An error]"
    val filenamesAndSources = FileCoordinateMap.test(interner, codeStr)
*/
// mig: fn make_loc
fn make_loc(pos: i32) {
    panic!("Unimplemented: make_loc");
}
/*
    def makeLoc(pos: Int) = CodeLocationS(FileCoordinate.test(interner), pos)
*/
// mig: fn make_range
fn make_range(begin: i32, end: i32) {
    panic!("Unimplemented: make_range");
}
/*
    def makeRange(begin: Int, end: Int) = RangeS(makeLoc(begin), makeLoc(end))

    val humanizePos = (x: CodeLocationS) => SourceCodeUtils.humanizePos(filenamesAndSources, x)
    val linesBetween = (x: CodeLocationS, y: CodeLocationS) => SourceCodeUtils.linesBetween(filenamesAndSources, x, y)
    val lineRangeContaining = (x: CodeLocationS) => SourceCodeUtils.lineRangeContaining(filenamesAndSources, x)
    val lineContaining = (x: CodeLocationS) => SourceCodeUtils.lineContaining(filenamesAndSources, x)

    val unsolvedRules =
      Vector(
        CoordComponentsSR(
          makeRange(0, codeStr.length),
          RuneUsage(makeRange(6, 7), CodeRuneS(interner.intern(StrI("I")))),
          RuneUsage(makeRange(11, 12), CodeRuneS(interner.intern(StrI("A")))),
          RuneUsage(makeRange(33, 52), ImplicitRuneS(LocationInDenizen(Vector(7))))),
        KindComponentsSR(
          makeRange(33, 52),
          RuneUsage(makeRange(33, 52), ImplicitRuneS(LocationInDenizen(Vector(7)))),
          RuneUsage(makeRange(43, 45), CodeRuneS(interner.intern(StrI("An"))))))

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
                CodeRuneS(interner.intern(StrI("A"))) -> OwnershipTemplataT(OwnT)))).toStream,
          Map(),
          unsolvedRules,
          Vector(),
          RuleError(KindIsNotConcrete(ispaceshipKind)))))
      .nonEmpty)

    val errorText =
      CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
        TypingPassSolverError(
          tz,
          FailedSolve(
            Vector(
              Step[IRulexSR, IRuneS, ITemplataT[ITemplataType]](
                false,
                Vector(),
                Vector(),
                Map(
                  CodeRuneS(interner.intern(StrI("A"))) -> OwnershipTemplataT(OwnT)))).toStream,
            Map(
              CodeRuneS(interner.intern(StrI("A"))) -> OwnershipTemplataT(OwnT)),
            unsolvedRules,
            Vector(
              CodeRuneS(interner.intern(StrI("I"))),
              CodeRuneS(interner.intern(StrI("Of"))),
              CodeRuneS(interner.intern(StrI("An"))),
              ImplicitRuneS(LocationInDenizen(Vector(7)))),
            SolveIncomplete())))
    println(errorText)
    vassert(errorText.nonEmpty)
    vassert(errorText.contains("\n           ^ A: own"))
    vassert(errorText.contains("\n      ^ I: (unknown)"))
    vassert(errorText.contains("\n                                 ^^^^^^^^^^^^^^^^^^^ _7: (unknown)"))
  }
*/
// mig: fn simple_int_rule
#[test]
fn simple_int_rule() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nexported func main() int where N Int = 3 {\n  return N;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let _ci: &ConstantIntTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ConstantInt(ci @ ConstantIntTE { value: ITemplataT::Integer(3), bits: 32, .. }) => Some(ci)
    );
}
/*
  test("Simple int rule") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() int where N Int = 3 {
        |  return N;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case ConstantIntTE(IntegerTemplataT(3), 32, _) => })
  }
*/
// mig: fn equals_transitive
#[test]
fn equals_transitive() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nexported func main() int where N Int = 3, M Int = N {\n  return M;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let _ci: &ConstantIntTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ConstantInt(ci @ ConstantIntTE { value: ITemplataT::Integer(3), bits: 32, .. }) => Some(ci)
    );
}
/*
  test("Equals transitive") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() int where N Int = 3, M Int = N {
        |  return M;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case ConstantIntTE(IntegerTemplataT(3), 32, _) => })
  }
*/
// mig: fn one_of
#[test]
fn one_of() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nexported func main() int where N Int = any(2, 3, 4), N = 3 {\n  return N;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let _ci: &ConstantIntTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ConstantInt(ci @ ConstantIntTE { value: ITemplataT::Integer(3), bits: 32, .. }) => Some(ci)
    );
}
/*
  test("OneOf") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() int where N Int = any(2, 3, 4), N = 3 {
        |  return N;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case ConstantIntTE(IntegerTemplataT(3), 32, _) => })
  }
*/
// mig: fn components
#[test]
fn components() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nexported struct MyStruct { }\nexported func main() X\nwhere\n  MyStruct = Ref[O Ownership, K Kind],\n  X Ref = Ref[borrow, K]\n{\n  return &MyStruct();\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    match coutputs.lookup_function_by_str("main").header.return_type {
        CoordT { ownership: OwnershipT::Borrow, kind: KindT::Struct(_), .. } => {}
        _ => panic!("expected Borrow Struct return_type"),
    }
}
/*
  test("Default generic param should not conflict with arg inference") {
    // H has a default of 5, but calling moo(MyStruct<10>()) should infer H=10 from
    // the argument type. The default should act as a fallback, not an eager constraint
    // that conflicts with argument inference.
    val compile = CompilerTestCompilation.test(
      """
        |struct MyStruct<H Int = 5> { }
        |func moo<H Int = 5>(s MyStruct<H>) int { return H; }
        |exported func main() int {
        |  return moo(MyStruct<10>());
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

  test("DRSINI interface default generic arg in struct member") {
    // MyInterface<bool> must resolve H=5 from default during resolveInterface.
    // Before fix: the interface's abstract drop function conflicts (H=5 vs H=placeholder).
    val compile = CompilerTestCompilation.test(
      """
        |sealed interface MyInterface<K Ref, H Int = 5> { }
        |struct MyStruct {
        |  x MyInterface<bool>;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

  test("DRSINI multiple defaults with partial override") {
    // A has a default of 10, B has a default of 20.
    // Calling with MyStruct<7>() overrides A=7 but B should still default to 20.
    // Returning A verifies the override; compiling at all verifies B's default works.
    val compile = CompilerTestCompilation.test(
      """
        |struct MyStruct<A Int = 10, B Int = 20> { }
        |func moo<A Int = 10, B Int = 20>(s MyStruct<A, B>) int { return A; }
        |exported func main() int {
        |  return moo(MyStruct<7>());
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

  test("Components") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct MyStruct { }
        |exported func main() X
        |where
        |  MyStruct = Ref[O Ownership, K Kind],
        |  X Ref = Ref[borrow, K]
        |{
        |  return &MyStruct();
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    coutputs.lookupFunction("main").header.returnType match {
      case CoordT(BorrowT, _, StructTT(_)) =>
    }
  }
*/
// mig: fn prototype_rule_call_via_rune
#[test]
fn prototype_rule_call_via_rune() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nfunc moo(i int, b bool) str { return \"hello\"; }\nexported func main() str\nwhere mooFunc Prot = func moo(int, bool)str\n{\n  return (mooFunc)(5, true);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let call: &FunctionCallTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(c) => Some(c)
    );
    assert_eq!(crate::typing::templata::templata_utils::unapply_simple_name(&call.callable.id), Some("moo".to_string()));
}
/*
  test("Prototype rule, call via rune") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |func moo(i int, b bool) str { return "hello"; }
        |exported func main() str
        |where mooFunc Prot = func moo(int, bool)str
        |{
        |  return (mooFunc)(5, true);
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case FunctionCallTE(PrototypeT(simpleNameT("moo"), _), _, _) =>
    })
  }
*/
// mig: fn prototype_rule_call_directly
#[test]
fn prototype_rule_call_directly() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nfunc moo(i int, b bool) str { return \"hello\"; }\nexported func main() str\nwhere func moo(int, bool)str\n{\n  return moo(5, true);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let call: &FunctionCallTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(c) => Some(c)
    );
    assert_eq!(crate::typing::templata::templata_utils::unapply_simple_name(&call.callable.id), Some("moo".to_string()));
}
/*
  test("Prototype rule, call directly") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |func moo(i int, b bool) str { return "hello"; }
        |exported func main() str
        |where func moo(int, bool)str
        |{
        |  return moo(5, true);
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case FunctionCallTE(PrototypeT(simpleNameT("moo"), _), _, _) =>
    })
  }
*/
// mig: fn send_struct_to_struct
#[test]
fn send_struct_to_struct() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct MyStruct {}\nfunc moo(m MyStruct) { }\nexported func main() {\n  moo(MyStruct())\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Send struct to struct") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct MyStruct {}
        |func moo(m MyStruct) { }
        |exported func main() {
        |  moo(MyStruct())
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn send_struct_to_interface
#[test]
fn send_struct_to_interface() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct MyStruct {}\ninterface MyInterface {}\nimpl MyInterface for MyStruct;\nfunc moo(m MyInterface) { }\nexported func main() {\n  moo(MyStruct())\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Send struct to interface") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct MyStruct {}
        |interface MyInterface {}
        |impl MyInterface for MyStruct;
        |func moo(m MyInterface) { }
        |exported func main() {
        |  moo(MyStruct())
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn assume_most_specific_generic_param
#[test]
fn assume_most_specific_generic_param() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct MyStruct {}\ninterface MyInterface {}\nimpl MyInterface for MyStruct;\nfunc moo<T>(m T) where func drop(T)void { }\nexported func main() {\n  moo(MyStruct())\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let call: &FunctionCallTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(c @ FunctionCallTE { args: [_], .. }) => Some(c)
    );
    let arg = call.args[0];
    match arg.result().coord {
        CoordT { kind: KindT::Struct(_), .. } => {}
        _ => panic!("expected Struct arg"),
    }
}
/*
  test("Assume most specific generic param") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct MyStruct {}
        |interface MyInterface {}
        |impl MyInterface for MyStruct;
        |func moo<T>(m T) where func drop(T)void { }
        |exported func main() {
        |  moo(MyStruct())
        |}
        |""".stripMargin
    )

    val coutputs = compile.expectCompilerOutputs()
    val arg =
      coutputs.lookupFunction("main").body shouldHave {
        case FunctionCallTE(_, Vector(arg), _) => arg
      }
    arg.result.coord match {
      case CoordT(_, _, StructTT(_)) =>
    }
  }
*/
// mig: fn assume_most_specific_common_ancestor
#[test]
fn assume_most_specific_common_ancestor() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\ninterface IShip {}\nstruct Firefly {}\nimpl IShip for Firefly;\nstruct Serenity {}\nimpl IShip for Serenity;\nfunc moo<T>(a T, b T) where func drop(T)void { }\nexported func main() {\n  moo(Firefly(), Serenity())\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let _moo = coutputs.lookup_function_by_str("moo");
    let main = coutputs.lookup_function_by_str("main");
    let _call: &FunctionCallTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(c @ FunctionCallTE { args: [_, _], .. }) => Some({
            let fn_name = match c.callable.id.local_name {
                INameT::Function(fn_name) => fn_name,
                _ => panic!("expected Function local_name"),
            };
            match fn_name.template_args[0] {
                ITemplataT::Coord(ct) => match ct.coord {
                    CoordT { kind: KindT::Interface(_), .. } => {}
                    _ => panic!("expected Interface template arg"),
                },
                _ => panic!("expected Coord template arg"),
            }
            c
        })
    );
    let upcasts: Vec<&UpcastTE> = crate::collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Upcast(u) => Some(u)
    );
    assert_eq!(upcasts.len(), 2);
}
/*
Guardian: temp-disable: SPDMX — In Scala, `prototype.id` is statically `IdT[IFunctionNameT]` so `templateArgs` is defined on `IFunctionNameT`, not on `INameT`. Rust uses +T erasure (design v3 §6.0): `IdT.local_name: INameT` is widened, narrowed at use sites via match/TryFrom — precedent at compiler_tests.rs:1776 and earlier in this file (test_having_drop_function_concept_function:151). — /Volumes/V/Sylvan/FrontendRust/guardian-logs/request-1346-1779035154606/hook-1346/assume_most_specific_common_ancestor--1086.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  test("Assume most specific common ancestor") {
    val compile = CompilerTestCompilation.test(
      """
        |interface IShip {}
        |struct Firefly {}
        |impl IShip for Firefly;
        |struct Serenity {}
        |impl IShip for Serenity;
        |func moo<T>(a T, b T) where func drop(T)void { }
        |exported func main() {
        |  moo(Firefly(), Serenity())
        |}
        |""".stripMargin
    )

    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupFunction("moo")
    val main = coutputs.lookupFunction("main")
    main.body shouldHave {
      case FunctionCallTE(prototype, Vector(_, _), _) => {
        prototype.id.localName.templateArgs.head match {
          case CoordTemplataT(CoordT(_, _, InterfaceTT(_))) =>
        }
      }
    }
    Collector.all(main, {
      case UpcastTE(_, _, _) =>
    }).size shouldEqual 2
  }
*/
// mig: fn descendant_satisfying_call
#[test]
fn descendant_satisfying_call() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\ninterface IShip<T> where T Ref {}\nstruct Firefly<T> where T Ref {}\nimpl<T> IShip<T> for Firefly<T>;\nfunc moo<T>(a IShip<T>) { }\nexported func main() {\n  moo(Firefly<int>())\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let moo = coutputs.lookup_function_by_str("moo");
    match moo.header.params[0].tyype {
        CoordT { kind: KindT::Interface(itt), .. } => match itt.id.local_name {
            INameT::Interface(in_) => match in_.template_args {
                [ITemplataT::Coord(ct)] => match ct.coord {
                    CoordT { kind: KindT::KindPlaceholder(_), .. } => {}
                    _ => panic!("expected KindPlaceholder template arg"),
                },
                _ => panic!("expected single Coord template arg"),
            },
            _ => panic!("expected Interface local_name"),
        },
        _ => panic!("expected Interface param coord"),
    }
    let main = coutputs.lookup_function_by_str("main");
    let calls: Vec<&FunctionCallTE> = crate::collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(c) => Some(c)
    );
    let _moo_call = calls.iter().find(|c| {
        let is_moo = match c.callable.id.local_name {
            INameT::Function(fn_name) => fn_name.template.human_name.0 == "moo",
            _ => false,
        };
        if !is_moo { return false; }
        if c.args.len() != 1 { return false; }
        let upcast = match c.args[0] {
            ReferenceExpressionTE::Upcast(u) => u,
            _ => return false,
        };
        match upcast.target_super_kind {
            ISuperKindTT::Interface(itt) => match itt.id.local_name {
                INameT::Interface(in_) => {
                    in_.template.human_namee.0 == "IShip" && match in_.template_args {
                        [ITemplataT::Coord(ct)] => matches!(ct.coord, CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT::I32), .. }),
                        _ => false,
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }).expect("expected FunctionCallTE moo(UpcastTE(_, IShip<int>, _))");
}
/*
Guardian: temp-disable: SPDMX — Scala field IS `humanNamee` (double 'e') on `InterfaceTemplateNameT` (see Frontend/.../names.scala — Rust port at names.rs:2217). Guardian mistakenly assumed single 'e'; the change is correct parity. — /Volumes/V/Sylvan/FrontendRust/guardian-logs/request-1368-1779035889070/hook-1368/descendant_satisfying_call--1166.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  test("Descendant satisfying call") {
    val compile = CompilerTestCompilation.test(
      """
        |interface IShip<T> where T Ref {}
        |struct Firefly<T> where T Ref {}
        |impl<T> IShip<T> for Firefly<T>;
        |func moo<T>(a IShip<T>) { }
        |exported func main() {
        |  moo(Firefly<int>())
        |}
        |""".stripMargin
    )

    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupFunction("moo")
    moo.header.params.head.tyype match {
      case CoordT(_, _, InterfaceTT(IdT(_, _, CitizenNameT(_, Vector(CoordTemplataT(CoordT(_, _, KindPlaceholderT(IdT(_,_,KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _))))))))))) =>
    }
    val main = coutputs.lookupFunction("main")
    main.body shouldHave {
      case FunctionCallTE(
        PrototypeT(IdT(_,_, FunctionNameT(FunctionTemplateNameT(StrI("moo"), _), _, _)), _),
        Vector(
          UpcastTE(
            _,
            InterfaceTT(IdT(_,_,InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")),Vector(CoordTemplataT(CoordT(ShareT,_,IntT(32))))))),
            _)),
        _) =>
    }
  }
*/
// mig: fn reports_incomplete_solve
#[test]
fn reports_incomplete_solve() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nexported func main() int where N Int {\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::TypingPassSolverError { failed_solve, .. } => {
            assert!(failed_solve.unsolved_rules.is_empty(), "expected empty unsolved_rules");
            assert!(matches!(failed_solve.error, ISolverError::SolveIncomplete(_)));
            let expected_n_rune = IRuneS::CodeRune(scout_arena.alloc(CodeRuneS {
                name: scout_arena.intern_str("N"),
            }));
            let unsolved_set: std::collections::HashSet<_> = failed_solve.unsolved_runes.iter().copied().collect();
            let mut expected: std::collections::HashSet<IRuneS> = std::collections::HashSet::new();
            expected.insert(expected_n_rune);
            assert_eq!(unsolved_set, expected);
        }
        _ => panic!("expected TypingPassSolverError"),
    }
}
/*
  test("Reports incomplete solve") {
    val interner = new Interner()
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() int where N Int {
        |}
        |""".stripMargin,
      interner)
    compile.getCompilerOutputs() match {
      case Err(TypingPassSolverError(_,FailedSolve(_,_,Vector(),unsolved, SolveIncomplete()))) => {
        unsolved.toSet shouldEqual Set(CodeRuneS(interner.intern(interner.intern(StrI("N")))))
      }
    }
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
    let code = "\nimport v.builtins.drop.*;\n\ninterface MyInterface<X Ref> { }\n\nstruct SomeStruct<X Ref> where func drop(X)void { x X; }\nimpl<X> MyInterface<X> for SomeStruct<X> where func drop(X)void;\n\nfunc doAThing<T>(t T) SomeStruct<T>\nwhere func drop(T)void {\n  return SomeStruct<T>(t);\n}\n\nexported func main() {\n  doAThing(4);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Stamps an interface template via a function return") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |interface MyInterface<X Ref> { }
        |
        |struct SomeStruct<X Ref> where func drop(X)void { x X; }
        |impl<X> MyInterface<X> for SomeStruct<X> where func drop(X)void;
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
*/
// mig: fn pointer_becomes_share_if_kind_is_immutable
#[test]
fn pointer_becomes_share_if_kind_is_immutable() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\n\nstruct SomeStruct imm { i int; }\n\nfunc bork(x &SomeStruct) int {\n  return x.i;\n}\n\nexported func main() int {\n  return bork(SomeStruct(7));\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    assert_eq!(coutputs.lookup_function_by_str("bork").header.params[0].tyype.ownership, OwnershipT::Share);
}
/*
  test("Pointer becomes share if kind is immutable") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |
        |struct SomeStruct imm { i int; }
        |
        |func bork(x &SomeStruct) int {
        |  return x.i;
        |}
        |
        |exported func main() int {
        |  return bork(SomeStruct(7));
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    coutputs.lookupFunction("bork").header.params.head.tyype.ownership shouldEqual ShareT
  }
*/
// mig: fn detects_conflict_between_types
#[test]
fn detects_conflict_between_types() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct ShipA {}\nstruct ShipB {}\nexported func main<N Kind>() where N Kind = ShipA, N Kind = ShipB {\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::TypingPassSolverError { failed_solve: FailedSolve { error: ISolverError::SolverConflict(SolverConflict { previous_conclusion: ITemplataT::StructDefinition(StructDefinitionTemplataT { origin_struct: &StructA { name: IStructDeclarationNameS::TopLevelStructDeclarationName(TopLevelStructDeclarationNameS { name: StrI("ShipA"), .. }), .. }, .. }), new_conclusion: ITemplataT::StructDefinition(StructDefinitionTemplataT { origin_struct: &StructA { name: IStructDeclarationNameS::TopLevelStructDeclarationName(TopLevelStructDeclarationNameS { name: StrI("ShipB"), .. }), .. }, .. }), .. }), .. }, .. } => {}
        ICompileErrorT::TypingPassSolverError { failed_solve: FailedSolve { error: ISolverError::SolverConflict(SolverConflict { previous_conclusion: ITemplataT::StructDefinition(StructDefinitionTemplataT { origin_struct: &StructA { name: IStructDeclarationNameS::TopLevelStructDeclarationName(TopLevelStructDeclarationNameS { name: StrI("ShipB"), .. }), .. }, .. }), new_conclusion: ITemplataT::StructDefinition(StructDefinitionTemplataT { origin_struct: &StructA { name: IStructDeclarationNameS::TopLevelStructDeclarationName(TopLevelStructDeclarationNameS { name: StrI("ShipA"), .. }), .. }, .. }), .. }), .. }, .. } => {}
        ICompileErrorT::TypingPassSolverError { failed_solve: FailedSolve { error: ISolverError::SolverConflict(SolverConflict { previous_conclusion: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipA"), .. }), .. }), .. }, .. }) }), new_conclusion: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipB"), .. }), .. }), .. }, .. }) }), .. }), .. }, .. } => {}
        ICompileErrorT::TypingPassSolverError { failed_solve: FailedSolve { error: ISolverError::SolverConflict(SolverConflict { previous_conclusion: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipB"), .. }), .. }), .. }, .. }) }), new_conclusion: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipA"), .. }), .. }), .. }, .. }) }), .. }), .. }, .. } => {}
        ICompileErrorT::TypingPassSolverError { failed_solve: FailedSolve { error: ISolverError::RuleError(RuleError { err: ITypingPassSolverError::CallResultWasntExpectedType { actual: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipB"), .. }), .. }), .. }, .. }) }), .. }, .. }), .. }, .. } => {}
        ICompileErrorT::TypingPassSolverError { failed_solve: FailedSolve { error: ISolverError::RuleError(RuleError { err: ITypingPassSolverError::CallResultWasntExpectedType { actual: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipA"), .. }), .. }), .. }, .. }) }), .. }, .. }), .. }, .. } => {}
        // Rust-only extra arms: Rust's `.min()`-based solver firing order produces a
        // RuleError(InternalSolverError(SolverConflict(...))) shape that Scala's
        // HashMap-iteration-order solver doesn't hit for this test. See
        // docs/historical/nondeterministic-solver.md for the full investigation.
        ICompileErrorT::TypingPassSolverError { failed_solve: FailedSolve { error: ISolverError::RuleError(RuleError { err: ITypingPassSolverError::InternalSolverError { err: ISolverError::SolverConflict(SolverConflict { previous_conclusion: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipA"), .. }), .. }), .. }, .. }) }), new_conclusion: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipB"), .. }), .. }), .. }, .. }) }), .. }), .. }, .. }), .. }, .. } => {}
        ICompileErrorT::TypingPassSolverError { failed_solve: FailedSolve { error: ISolverError::RuleError(RuleError { err: ITypingPassSolverError::InternalSolverError { err: ISolverError::SolverConflict(SolverConflict { previous_conclusion: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipB"), .. }), .. }), .. }, .. }) }), new_conclusion: ITemplataT::Kind(&KindTemplataT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("ShipA"), .. }), .. }), .. }, .. }) }), .. }), .. }, .. }), .. }, .. } => {}
        other => panic!("vfail: {:#?}", other),
    }
}
/*
  test("Detects conflict between types") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct ShipA {}
        |struct ShipB {}
        |exported func main<N Kind>() where N Kind = ShipA, N Kind = ShipB {
        |}
        |""".stripMargin
    )
    compile.getCompilerOutputs() match {
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, SolverConflict(_, StructDefinitionTemplataT(_, StructA(_, TopLevelStructDeclarationNameS(StrI("ShipA"), _), _, _, _, _, _, _, _, _, _, _, _, _)), StructDefinitionTemplataT(_, StructA(_, TopLevelStructDeclarationNameS(StrI("ShipB"), _), _, _, _, _, _, _, _, _, _, _, _, _)))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, SolverConflict(_, StructDefinitionTemplataT(_, StructA(_, TopLevelStructDeclarationNameS(StrI("ShipB"), _), _, _, _, _, _, _, _, _, _, _, _, _)), StructDefinitionTemplataT(_, StructA(_, TopLevelStructDeclarationNameS(StrI("ShipA"), _), _, _, _, _, _, _, _, _, _, _, _, _)))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, SolverConflict(_, KindTemplataT(StructTT(IdT(_,_,StructNameT(StructTemplateNameT(StrI("ShipA")),_)))), KindTemplataT(StructTT(IdT(_,_,StructNameT(StructTemplateNameT(StrI("ShipB")),_)))))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, SolverConflict(_, KindTemplataT(StructTT(IdT(_,_,StructNameT(StructTemplateNameT(StrI("ShipB")),_)))), KindTemplataT(StructTT(IdT(_,_,StructNameT(StructTemplateNameT(StrI("ShipA")),_)))))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, RuleError(CallResultWasntExpectedType(_,KindTemplataT(StructTT(IdT(_,Vector(),StructNameT(StructTemplateNameT(StrI("ShipB")),Vector()))))))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, RuleError(CallResultWasntExpectedType(_,KindTemplataT(StructTT(IdT(_,Vector(),StructNameT(StructTemplateNameT(StrI("ShipA")),Vector()))))))))) =>
      case other => vfail(other)
    }
  }
*/
// mig: fn can_match_kind_templata_type_against_struct_env_entry_struct_templata
#[test]
fn can_match_kind_templata_type_against_struct_env_entry_struct_templata() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\n#!DeriveStructDrop\nstruct SomeStruct<T>\n{ x T; }\n\nfunc bork<X Kind, Z>() Z\nwhere X Kind = SomeStruct<int>, X = SomeStruct<Z> {\n  return 9;\n}\n\nexported func main() int {\n  return bork();\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let bork = coutputs.lookup_function_by_str("bork");
    let template_args = match bork.header.id.local_name {
        INameT::Function(fn_name) => fn_name.template_args,
        _ => panic!("expected Function local_name"),
    };
    let last = *template_args.last().unwrap();
    assert_eq!(last, ITemplataT::Coord(typing_bump.alloc(CoordTemplataT { coord: CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: KindT::Int(IntT::I32) } })));
}
/*
Guardian: temp-disable: SPDMX — In Scala, `header.id` is statically `IdT[IFunctionNameT]` so `templateArgs` is defined on `IFunctionNameT`, not on `INameT`. Rust uses +T erasure (design v3 §6.0): `IdT.local_name: INameT` is widened, narrowed at use sites via match — precedents at compiler_tests.rs:1776 and earlier in this file (test_having_drop_function_concept_function:151). — /Volumes/V/Sylvan/FrontendRust/guardian-logs/request-056-1779049047476/hook-056/can_match_kind_templata_type_against_struct_env_entry_struct_templata--1452.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  test("Can match KindTemplataType() against StructEnvEntry / StructTemplata") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |#!DeriveStructDrop
        |struct SomeStruct<T>
        |{ x T; }
        |
        |func bork<X Kind, Z>() Z
        |where X Kind = SomeStruct<int>, X = SomeStruct<Z> {
        |  return 9;
        |}
        |
        |exported func main() int {
        |  return bork();
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    coutputs.lookupFunction("bork").header.id.localName.templateArgs.last shouldEqual CoordTemplataT(CoordT(ShareT, RegionT(DefaultRegionT), IntT(32)))
  }
*/
// mig: fn can_destructure_and_assemble_static_sized_array
#[test]
fn can_destructure_and_assemble_static_sized_array() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nimport v.builtins.arrays.*;\nimport v.builtins.drop.*;\n\nfunc swap<T>(x [#2]T) [#2]T {\n  [a, b] = x;\n  return [#](b, a);\n}\n\nexported func main() int {\n  return swap([#](5, 7)).0;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();

    let swap = coutputs.lookup_function_by_str("swap");
    let swap_template_args = match swap.header.id.local_name {
        INameT::Function(fn_name) => fn_name.template_args,
        _ => panic!("expected Function local_name"),
    };
    match swap_template_args.last().unwrap() {
        ITemplataT::Coord(ct) => match ct.coord {
            CoordT { ownership: OwnershipT::Own, kind: KindT::KindPlaceholder(kp), .. } => match kp.id {
                IdT {
                    init_steps: [INameT::FunctionTemplate(FunctionTemplateNameT { human_name: StrI("swap"), .. })],
                    local_name: INameT::KindPlaceholder(KindPlaceholderNameT { template: KindPlaceholderTemplateNameT { index: 0, .. } }),
                    ..
                } => {}
                _ => panic!("expected KindPlaceholder local_name inside swap init_step"),
            },
            _ => panic!("expected Own KindPlaceholder coord"),
        },
        _ => panic!("expected Coord template arg"),
    }

    let main = coutputs.lookup_function_by_str("main");
    let call: &FunctionCallTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(c @ FunctionCallTE {
            callable: PrototypeT {
                id: IdT {
                    local_name: INameT::Function(FunctionNameT {
                        template: FunctionTemplateNameT { human_name: StrI("swap"), .. },
                        ..
                    }),
                    ..
                },
                ..
            },
            ..
        }) => Some(c)
    );
    let call_template_args = match call.callable.id.local_name {
        INameT::Function(fn_name) => fn_name.template_args,
        _ => panic!("expected Function local_name"),
    };
    match call_template_args.last().unwrap() {
        ITemplataT::Coord(ct) => match ct.coord {
            CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT::I32), .. } => {}
            _ => panic!("expected Share Int32 template arg"),
        },
        _ => panic!("expected Coord template arg"),
    }
}
/*
Guardian: temp-disable: SPDMX — In Scala, `header.id` and `call.callable.id` are statically `IdT[IFunctionNameT]` so `templateArgs` is on `IFunctionNameT`, not `INameT`. Rust +T erasure (design v3 §6.0) widens to `INameT` and narrows at use sites via match — precedents at compiler_tests.rs:1776 and elsewhere in this file. — /Volumes/V/Sylvan/FrontendRust/guardian-logs/request-063-1779049245500/hook-063/can_destructure_and_assemble_static_sized_array--1510.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  test("Can destructure and assemble static sized array") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |import v.builtins.arrays.*;
        |import v.builtins.drop.*;
        |
        |func swap<T>(x [#2]T) [#2]T {
        |  [a, b] = x;
        |  return [#](b, a);
        |}
        |
        |exported func main() int {
        |  return swap([#](5, 7)).0;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()

    val swap = coutputs.lookupFunction("swap")
    swap.header.id.localName.templateArgs.last match {
      case CoordTemplataT(CoordT(OwnT,_, KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("swap"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))))) =>
    }

    val main = coutputs.lookupFunction("main")
    val call =
      Collector.only(main, {
        case call @ FunctionCallTE(PrototypeT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("swap"), _), _, _)), _), _, _) => call
      })
    call.callable.id.localName.templateArgs.last match {
      case CoordTemplataT(CoordT(ShareT, _, IntT(32))) =>
    }
  }
*/
// mig: fn test_equivalent_identifying_runes_in_functions
#[test]
fn test_equivalent_identifying_runes_in_functions() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc bork<T, Y>(a T) Y where T = Y { return a; }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Test equivalent identifying runes in functions") {
    // Previously, the compiler would populate placeholders for all identifying runes at once.
    // This meant that it added a placeholder $T and a placeholder $Y at the same time.
    // Of course, that led to a conflict, because $T != $Y.
    // Now, we populate the placeholders one at a time. Both T and Y should be $T now.
    // This should also help when we switch to regions, where we want to say that two generic coords
    // share the same region.
    // See IRAGP.

    val compile = CompilerTestCompilation.test(
      """
        |func bork<T, Y>(a T) Y where T = Y { return a; }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn iragp_test_equivalent_identifying_runes_in_struct
#[test]
fn iragp_test_equivalent_identifying_runes_in_struct() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveStructDrop\nstruct Bork<T, Y> where T = Y { t T; y Y; }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("IRAGP: Test equivalent identifying runes in struct") {
    // See IRAGP, the original problem was for functions but we use the same solution for structs.
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveStructDrop
        |struct Bork<T, Y> where T = Y { t T; y Y; }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
}
*/
