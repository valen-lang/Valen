use bumpalo::Bump;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::postparsing::names::{CodeNameS, FunctionNameS, IFunctionDeclarationNameS, IImpreciseNameValS};
use crate::scout_arena::ScoutArena;
use crate::typing::ast::expressions::{AddressExpressionTE, ConstantIntTE, LocalLookupTE, MutateTE, ReferenceExpressionTE, ReferenceMemberLookupTE};
use crate::typing::compiler_error_humanizer::humanize;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::env::function_environment_t::{ILocalVariableT, ReferenceLocalVariableT};
use crate::typing::names::names::{CodeVarNameT, FunctionNameValT, FunctionTemplateNameT, IdT, IdValT, INameT, IStructTemplateNameT, IVarNameT, RawArrayNameT, StaticSizedArrayNameT, StructNameValT, StructTemplateNameT};
use crate::typing::templata::templata::{ITemplataT, KindTemplataT, MutabilityTemplataT, VariabilityTemplataT};
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::typing::types::types::{CoordT, IntT, IRegionT, KindT, MutabilityT, OwnershipT, RegionT, StaticSizedArrayTT, StructTTValT, VariabilityT};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::{self, FileCoordinateMap, IPackageResolver, PackageCoordinate};
use crate::utils::range::{CodeLocationS, RangeS};
use crate::utils::source_code_utils::{humanize_pos_code_map, line_containing, line_range_containing, lines_between};
use std::collections::HashMap;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::names::names::StructNameT;
use crate::typing::overload_resolver::FindFunctionFailure;

/*
package dev.vale.typing

import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.{CodeLocationS, Collector, Err, FileCoordinateMap, Interner, PackageCoordinate, RangeS, vassert, vfail}
import dev.vale._
import dev.vale.highertyping.ProgramA
import dev.vale.parsing._
import dev.vale.postparsing.PostParser
import OverloadResolver.{FindFunctionFailure, WrongNumberOfArguments}
import dev.vale.postparsing._
import dev.vale.typing.ast.{ConstantIntTE, LocalLookupTE, MutateTE, ReferenceMemberLookupTE, SignatureT}
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import org.scalatest._

import scala.collection.immutable.List
import scala.io.Source

class CompilerMutateTests extends FunSuite with Matchers {
  // TODO: pull all of the typingpass specific stuff out, the unit test-y stuff
*/
// mig: fn read_code_from_resource
pub fn read_code_from_resource(resource_filename: &str) -> String {
  panic!("Unimplemented: read_code_from_resource");
}
/*
  def readCodeFromResource(resourceFilename: String): String = {
    val is = Source.fromInputStream(getClass().getClassLoader().getResourceAsStream(resourceFilename))
    vassert(is != null)
    is.mkString("")
  }
*/
// mig: fn test_mutating_a_local_var
#[test]
fn test_mutating_a_local_var() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nexported func main() {a = 3; set a = 4; }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Mutate(MutateTE {
            destination_expr: AddressExpressionTE::LocalLookup(LocalLookupTE {
                local_variable: ILocalVariableT::Reference(ReferenceLocalVariableT {
                    name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("a"), .. }),
                    variability: VariabilityT::Varying,
                    ..
                }),
                ..
            }),
            source_expr: ReferenceExpressionTE::ConstantInt(ConstantIntTE {
                value: ITemplataT::Integer(4),
                ..
            }),
        }) => Some(())
    );

    let lookup: &LocalLookupTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::LocalLookup(l) => Some(l)
    );
    let result_coord = lookup.result().coord;
    assert_eq!(result_coord, CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: KindT::Int(IntT { bits: 32 }) });
}
/*
  test("Test mutating a local var") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() {a = 3; set a = 4; }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs();
    val main = coutputs.lookupFunction("main")
    Collector.only(main, { case MutateTE(LocalLookupTE(_,ReferenceLocalVariableT(CodeVarNameT(StrI("a")), VaryingT, _)), ConstantIntTE(IntegerTemplataT(4), _, _)) => })

    val lookup = Collector.only(main, { case l @ LocalLookupTE(range, localVariable) => l })
    val resultCoord = lookup.result.coord
    resultCoord shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
  }
*/
// mig: fn test_mutable_member_permission
#[test]
fn test_mutable_member_permission() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct Engine { fuel int; }\nstruct Spaceship { engine! Engine; }\nexported func main() {\n  ship = Spaceship(Engine(10));\n  set ship.engine = Engine(15);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");

    let lookup: &ReferenceMemberLookupTE = crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ReferenceMemberLookup(l) => Some(l)
    );
    let result_coord = lookup.result().coord;
    // See RMLRMO, it should result in the same type as the member.
    match result_coord {
        CoordT { ownership: OwnershipT::Own, kind: KindT::Struct(_), .. } => {}
        x => panic!("{:?}", x),
    }
}
/*
  test("Test mutable member permission") {
    val compile =
      CompilerTestCompilation.test(
        """
          |
          |struct Engine { fuel int; }
          |struct Spaceship { engine! Engine; }
          |exported func main() {
          |  ship = Spaceship(Engine(10));
          |  set ship.engine = Engine(15);
          |}
          |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs();
    val main = coutputs.lookupFunction("main")

    val lookup = Collector.only(main, { case l @ ReferenceMemberLookupTE(_, _, _, _, _) => l })
    val resultCoord = lookup.result.coord
    // See RMLRMO, it should result in the same type as the member.
    resultCoord match {
      case CoordT(OwnT, _, StructTT(_)) =>
      case x => vfail(x.toString)
    }
  }
*/
// mig: fn local_set_upcasts
#[test]
fn local_set_upcasts() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.drop.*;\n\ninterface IXOption<T Ref> where func drop(T)void { }\nstruct XSome<T Ref> where func drop(T)void { value T; }\nimpl<T Ref> IXOption<T> for XSome<T> where func drop(T)void;\nstruct XNone<T Ref> where func drop(T)void { }\nimpl<T Ref> IXOption<T> for XNone<T> where func drop(T)void;\n\nexported func main() {\n  m IXOption<int> = XNone<int>();\n  set m = XSome(6);\n}\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Mutate(MutateTE {
            source_expr: ReferenceExpressionTE::Upcast(_),
            ..
        }) => Some(())
    );
}
/*
  test("Local-set upcasts") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |interface IXOption<T Ref> where func drop(T)void { }
        |struct XSome<T Ref> where func drop(T)void { value T; }
        |impl<T Ref> IXOption<T> for XSome<T> where func drop(T)void;
        |struct XNone<T Ref> where func drop(T)void { }
        |impl<T Ref> IXOption<T> for XNone<T> where func drop(T)void;
        |
        |exported func main() {
        |  m IXOption<int> = XNone<int>();
        |  set m = XSome(6);
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case MutateTE(_, UpcastTE(_, _, _)) =>
    })
  }
*/
// mig: fn expr_set_upcasts
#[test]
fn expr_set_upcasts() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.drop.*;\n\ninterface IXOption<T Ref> where func drop(T)void { }\nstruct XSome<T Ref> where func drop(T)void { value T; }\nimpl<T Ref> IXOption<T> for XSome<T>;\nstruct XNone<T Ref> where func drop(T)void { }\nimpl<T Ref> IXOption<T> for XNone<T>;\n\nstruct Marine {\n  weapon! IXOption<int>;\n}\nexported func main() {\n  m = Marine(XNone<int>());\n  set m.weapon = XSome(6);\n}\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    crate::collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Mutate(MutateTE {
            source_expr: ReferenceExpressionTE::Upcast(_),
            ..
        }) => Some(())
    );
}
/*
  test("Expr-set upcasts") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |interface IXOption<T Ref> where func drop(T)void { }
        |struct XSome<T Ref> where func drop(T)void { value T; }
        |impl<T Ref> IXOption<T> for XSome<T>;
        |struct XNone<T Ref> where func drop(T)void { }
        |impl<T Ref> IXOption<T> for XNone<T>;
        |
        |struct Marine {
        |  weapon! IXOption<int>;
        |}
        |exported func main() {
        |  m = Marine(XNone<int>());
        |  set m.weapon = XSome(6);
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case MutateTE(_, UpcastTE(_, _, _)) =>
    })
  }
*/
// mig: fn reports_when_we_try_to_mutate_an_imm_struct
#[test]
fn reports_when_we_try_to_mutate_an_imm_struct() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct Vec3 imm { x float; y float; z float; }\nexported func main() int {\n  v = Vec3(3.0, 4.0, 5.0);\n  set v.x = 10.0;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(&scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CantMutateFinalMember { struct_, member_name, .. } => {
            match struct_.id.local_name {
                INameT::Struct(StructNameT {
                    template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Vec3"), .. }),
                    template_args: &[],
                    ..
                }) => {}
                _ => panic!("expected Struct(StructTemplateNameT(\"Vec3\"))"),
            }
            match member_name {
                IVarNameT::CodeVar(CodeVarNameT { name: StrI("x"), .. }) => {}
                _ => panic!("expected CodeVarNameT(\"x\")"),
            }
        }
        _ => panic!("expected CantMutateFinalMember"),
    }
}
/*
  test("Reports when we try to mutate an imm struct") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Vec3 imm { x float; y float; z float; }
        |exported func main() int {
        |  v = Vec3(3.0, 4.0, 5.0);
        |  set v.x = 10.0;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantMutateFinalMember(_, structTT, memberName)) => {
        structTT match {
          case StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("Vec3")), Vector()))) =>
        }
        memberName match {
          case CodeVarNameT(StrI("x")) =>
        }
      }
    }
  }
*/
// mig: fn reports_when_we_try_to_mutate_a_final_member_in_a_struct
#[test]
fn reports_when_we_try_to_mutate_a_final_member_in_a_struct() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct Vec3 { x float; y float; z float; }\nexported func main() int {\n  v = Vec3(3.0, 4.0, 5.0);\n  set v.x = 10.0;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(&scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CantMutateFinalMember { struct_, member_name, .. } => {
            match struct_.id.local_name {
                INameT::Struct(StructNameT {
                    template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Vec3"), .. }),
                    template_args: &[],
                    ..
                }) => {}
                _ => panic!("expected Struct(StructTemplateNameT(\"Vec3\"))"),
            }
            match member_name {
                IVarNameT::CodeVar(CodeVarNameT { name: StrI("x"), .. }) => {}
                _ => panic!("expected CodeVarNameT(\"x\")"),
            }
        }
        _ => panic!("expected CantMutateFinalMember"),
    }
}
/*
  test("Reports when we try to mutate a final member in a struct") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Vec3 { x float; y float; z float; }
        |exported func main() int {
        |  v = Vec3(3.0, 4.0, 5.0);
        |  set v.x = 10.0;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantMutateFinalMember(_, structTT, memberName)) => {
        structTT match {
          case StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("Vec3")), Vector()))) =>
        }
        memberName match {
          case CodeVarNameT(StrI("x")) =>
        }
      }
    }
  }
*/
// mig: fn reports_when_we_try_to_mutate_an_element_in_an_imm_static_sized_array
#[test]
fn reports_when_we_try_to_mutate_an_element_in_an_imm_static_sized_array() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.arrays.*;\nimport v.builtins.drop.*;\n\nexported func main() int {\n  arr = #[#10]({_});\n  set arr[4] = 10;\n  return 73;\n}\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(&scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CantMutateFinalElement {
            coord: CoordT {
                kind: KindT::StaticSizedArray(StaticSizedArrayTT {
                    name: IdT {
                        local_name: INameT::StaticSizedArray(StaticSizedArrayNameT {
                            size: ITemplataT::Integer(10),
                            variability: ITemplataT::Variability(VariabilityTemplataT { variability: VariabilityT::Final }),
                            arr: RawArrayNameT {
                                mutability: ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }),
                                element_type: CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT { .. }), .. },
                                ..
                            },
                            ..
                        }),
                        ..
                    },
                    ..
                }),
                ..
            },
            ..
        } => {}
        _ => panic!("expected CantMutateFinalElement"),
    }
}
/*
Guardian: temp-disable: SPDMX — Scala case class field is `coord: CoordT` (see compiler_error_reporter.rs:423 / `CantMutateFinalElement(range, coord)`). Guardian confused the test's positional destructure binding `case Err(CantMutateFinalElement(_, arrRef2))` (local var name) with the field name. The Rust field `coord` is the correct Scala-parity name; no rename happened. — /Volumes/V/Sylvan/FrontendRust/guardian-logs/request-601-1778979444137/hook-601/reports_when_we_try_to_mutate_an_element_in_an_imm_static_sized_array--433.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  test("Reports when we try to mutate an element in an imm static-sized array") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arrays.*;
        |import v.builtins.drop.*;
        |
        |exported func main() int {
        |  arr = #[#10]({_});
        |  set arr[4] = 10;
        |  return 73;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantMutateFinalElement(_, arrRef2)) => {
        arrRef2.kind match {
          case contentsStaticSizedArrayTT(IntegerTemplataT(10),MutabilityTemplataT(ImmutableT),VariabilityTemplataT(FinalT),CoordT(ShareT,_, IntT(_)), _) =>
        }
      }
    }
  }
*/
// mig: fn reports_when_we_try_to_mutate_a_local_variable_with_wrong_type
#[test]
fn reports_when_we_try_to_mutate_a_local_variable_with_wrong_type() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nexported func main() {\n  a = 5;\n  set a = \"blah\";\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(&scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CouldntConvertForMutateT { expected_type: CoordT { ownership: OwnershipT::Share, kind: KindT::Int(IntT { bits: 32 }), .. }, actual_type: CoordT { ownership: OwnershipT::Share, kind: KindT::Str(_), .. }, .. } => {}
        _ => panic!("expected CouldntConvertForMutateT"),
    }
}
/*
  test("Reports when we try to mutate a local variable with wrong type") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() {
        |  a = 5;
        |  set a = "blah";
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CouldntConvertForMutateT(_, CoordT(ShareT, _, IntT.i32), CoordT(ShareT, RegionT(DefaultRegionT), StrT()))) =>
      case _ => vfail()
    }
  }
*/
// mig: fn reports_when_we_try_to_override_a_non_interface
#[test]
fn reports_when_we_try_to_override_a_non_interface() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nimpl int for Bork;\nstruct Bork { }\nexported func main() {\n  Bork();\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(&scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CantImplNonInterface { templata: ITemplataT::Kind(KindTemplataT { kind: KindT::Int(IntT { bits: 32 }) }), .. } => {}
        _ => panic!("expected CantImplNonInterface"),
    }
}
/*
  test("Reports when we try to override a non-interface") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |impl int for Bork;
        |struct Bork { }
        |exported func main() {
        |  Bork();
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantImplNonInterface(_, KindTemplataT(IntT(32)))) =>
      case _ => vfail()
    }
  }
*/
// mig: fn can_mutate_an_element_in_a_runtime_sized_array
#[test]
fn can_mutate_an_element_in_a_runtime_sized_array() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.arrays.*;\nimport v.builtins.drop.*;\n\nexported func main() int {\n  arr = Array<mut, int>(3);\n  arr.push(0);\n  arr.push(1);\n  arr.push(2);\n  set arr[1] = 10;\n  return 73;\n}\n";
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(&scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump);
    compile.expect_compiler_outputs();
}
/*
  test("Can mutate an element in a runtime-sized array") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arrays.*;
        |import v.builtins.drop.*;
        |
        |exported func main() int {
        |  arr = Array<mut, int>(3);
        |  arr.push(0);
        |  arr.push(1);
        |  arr.push(2);
        |  set arr[1] = 10;
        |  return 73;
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
*/
// mig: fn can_restackify_in_destructure_pattern
#[test]
fn can_restackify_in_destructure_pattern() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveStructDrop\nstruct Ship { fuel int; }\n\n/// TODO: Bring tuples back\n#!DeriveStructDrop\nstruct GetFuelResult { fuel int; ship Ship; }\n\nfunc GetFuel(ship Ship) GetFuelResult {\n  return GetFuelResult(ship.fuel, ship);\n}\n\nexported func main() int {\n  ship = Ship(42);\n  [fuel, set ship] = GetFuel(ship);\n  [f] = ship;\n  return fuel;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(&scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump);
    compile.expect_compiler_outputs();
}
/*
  test("Can restackify in destructure pattern") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveStructDrop
        |struct Ship { fuel int; }
        |
        |/// TODO: Bring tuples back
        |#!DeriveStructDrop
        |struct GetFuelResult { fuel int; ship Ship; }
        |
        |func GetFuel(ship Ship) GetFuelResult {
        |  return GetFuelResult(ship.fuel, ship);
        |}
        |
        |exported func main() int {
        |  ship = Ship(42);
        |  [fuel, set ship] = GetFuel(ship);
        |  [f] = ship;
        |  return fuel;
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
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

    let myfunc_template_name = typing_interner.intern_function_template_name(
        FunctionTemplateNameT { human_name: scout_arena.intern_str("myFunc"), code_location: tz_code_loc, _phantom: std::marker::PhantomData });
    let myfunc_func_name = typing_interner.intern_function_name(
        FunctionNameValT { template: myfunc_template_name, template_args: &[], parameters: &[] });
    let myfunc_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Function(myfunc_func_name),
    });

    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindTypeT { range: tz_slice, name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str("Spaceship") })) }).is_empty());
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
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantReconcileBranchesResults { range: tz_slice, then_result: firefly_coord, else_result: serenity_coord }).is_empty());
    let firefly_var_name: &CodeVarNameT = typing_bump.alloc(CodeVarNameT { name: scout_arena.intern_str("firefly"), _phantom: std::marker::PhantomData });
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantUseUnstackifiedLocal { range: tz_slice, local_id: IVarNameT::CodeVar(firefly_var_name) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::FunctionAlreadyExists { old_function_range: tz, new_function_range: tz, signature: *myfunc_id }).is_empty());
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
}
/*
  test("Humanize errors") {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val nameStr = interner.intern(StrI("main"))
    val testPackageCoord = PackageCoordinate.TEST_TLD(interner, keywords)
    val tzCodeLoc = CodeLocationS.testZero(interner)
    val fireflyKind = StructTT(IdT(PackageCoordinate.TEST_TLD(interner, keywords), Vector.empty, interner.intern(StructNameT(StructTemplateNameT(StrI("Firefly")), Vector.empty))))
    val fireflyCoord = CoordT(OwnT,RegionT(DefaultRegionT), fireflyKind)
    val serenityKind = StructTT(IdT(PackageCoordinate.TEST_TLD(interner, keywords), Vector.empty, interner.intern(StructNameT(StructTemplateNameT(StrI("Serenity")), Vector.empty))))
    val serenityCoord = CoordT(OwnT,RegionT(DefaultRegionT), serenityKind)

    val filenamesAndSources = FileCoordinateMap.test(interner, "blah blah blah\nblah blah blah")

    val humanizePos = (x: CodeLocationS) => SourceCodeUtils.humanizePos(filenamesAndSources, x)
    val linesBetween = (x: CodeLocationS, y: CodeLocationS) => SourceCodeUtils.linesBetween(filenamesAndSources, x, y)
    val lineRangeContaining = (x: CodeLocationS) => SourceCodeUtils.lineRangeContaining(filenamesAndSources, x)
    val lineContaining = (x: CodeLocationS) => SourceCodeUtils.lineContaining(filenamesAndSources, x)

    val tz = List(RangeS.testZero(interner))
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindTypeT(tz, CodeNameS(interner.intern(StrI("Spaceship"))))).nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindFunctionToCallT(
        tz,
        FindFunctionFailure(interner.intern(CodeNameS(interner.intern(StrI("")))), Vector.empty, Map())))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CannotSubscriptT(
        tz,
        fireflyKind))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindIdentifierToLoadT(
        tz,
        interner.intern(CodeNameS(StrI("spaceship")))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindMemberT(
        tz,
        "hp"))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      BodyResultDoesntMatch(
        tz,
        FunctionNameS(interner.intern(StrI("myFunc")), tzCodeLoc), fireflyCoord, serenityCoord))
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
        interner.intern(CodeVarNameT(StrI("hp")))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantReconcileBranchesResults(
        tz,
        fireflyCoord,
        serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantUseUnstackifiedLocal(
        tz,
        interner.intern(CodeVarNameT(StrI("firefly")))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      FunctionAlreadyExists(
        tz.head,
        tz.head,
        IdT(testPackageCoord, Vector.empty, interner.intern(FunctionNameT(interner.intern(FunctionTemplateNameT(interner.intern(StrI("myFunc")), tz.head.begin)), Vector(), Vector())))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantMutateFinalMember(
        tz,
        serenityKind,
        interner.intern(CodeVarNameT(StrI("bork")))))
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
  }
}
*/
