use bumpalo::Bump;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::names::names::{FunctionNameT, FunctionTemplateNameT, IdT, INameT, IStructTemplateNameT, StructNameT, StructTemplateNameT};
use crate::typing::templata::templata::{CoordTemplataT, ITemplataT};
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::types::types::{CoordT, KindT, StructTT};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::{self, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use crate::builtins::builtins::get_embedded_modulized_code_map;
use crate::collect_only_tnode;
use crate::collect_where_tnode;
use crate::postparsing::names::CodeRuneS;
use crate::postparsing::names::IRuneS;
use crate::tests::tests::get_package_to_resource_resolver;
use crate::typing::ast::citizens::ReferenceMemberTypeT;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::names::names::IFunctionNameT;
use crate::typing::names::names::InterfaceNameT;
use crate::typing::names::names::InterfaceTemplateNameT;
use crate::typing::names::names::KindPlaceholderNameT;
use crate::typing::names::names::KindPlaceholderTemplateNameT;
use crate::typing::names::names::LambdaCallFunctionNameT;
use crate::typing::names::names::LambdaCallFunctionTemplateNameT;
use crate::typing::overload_resolver::IFindFunctionFailureReason;
use crate::typing::types::types::InterfaceTT;
use crate::typing::types::types::KindPlaceholderT;
use crate::typing::types::types::OwnershipT;
use std::collections::HashSet;

// mig: struct AfterRegionsTests
pub struct AfterRegionsTests {}
/*
package dev.vale.typing

import dev.vale.typing.infer._
import dev.vale.solver.{FailedSolve, RuleError}
import dev.vale.typing.OverloadResolver.InferFailure
import dev.vale.typing.ResolvingSolveFailedOrIncomplete
import dev.vale.typing.ast.{SignatureT, _}
import dev.vale.typing.infer.SendingNonCitizen
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.postparsing._
import dev.vale.typing.types._
import dev.vale.{Collector, Err, Ok, vassert, vwat, _}
//import dev.vale.typingpass.infer.NotEnoughToSolveError
import org.scalatest._

import scala.io.Source
import OverloadResolver._

class AfterRegionsTests extends FunSuite with Matchers {
*/
// mig: fn method_call_on_generic_data
#[test]
fn method_call_on_generic_data() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.drop.*;\n\nsealed interface IShip {\n  func launch(virtual self &IShip);\n}\n\nstruct Raza { fuel int; }\n\nimpl IShip for Raza;\nfunc launch(self &Raza) { }\n\nfunc launchGeneric<T>(x &T)\nwhere implements(T, IShip) {\n  x.launch();\n}\n\nexported func main() {\n  launchGeneric(&Raza(42));\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();

    let launch_generic = coutputs.lookup_function_by_str("launchGeneric");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(launch_generic),
        NodeRefT::FunctionCall(FunctionCallTE {
            callable: PrototypeT {
                id: IdT { local_name: INameT::Function(FunctionNameT { template: FunctionTemplateNameT { human_name: StrI("launch"), .. }, .. }), .. },
                ..
            },
            ..
        }) => Some(())
    );

    let main = coutputs.lookup_function_by_str("main");
    let upcasts: Vec<_> = collect_where_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Upcast(u) => Some(u)
    );
    assert_eq!(upcasts.len(), 0);
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(FunctionCallTE {
            callable: PrototypeT {
                id: IdT {
                    local_name: INameT::Function(FunctionNameT {
                        template: FunctionTemplateNameT { human_name: StrI("launchGeneric"), .. },
                        template_args: &[ITemplataT::Coord(CoordTemplataT { coord: CoordT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Raza"), .. }), .. }), .. }, .. }), .. }, .. })],
                        ..
                    }),
                    ..
                },
                ..
            },
            ..
        }) => Some(())
    );
}
/*
  test("Method call on generic data") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |sealed interface IShip {
        |  func launch(virtual self &IShip);
        |}
        |
        |struct Raza { fuel int; }
        |
        |impl IShip for Raza;
        |func launch(self &Raza) { }
        |
        |func launchGeneric<T>(x &T)
        |where implements(T, IShip) {
        |  x.launch();
        |}
        |
        |exported func main() {
        |  launchGeneric(&Raza(42));
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val launchGeneric = coutputs.lookupFunction("launchGeneric")
    // launchGeneric's body resolves x.launch() to a virtual call on IShip (dispatched via the impl
    // bound). The placeholder T survives as the arg type.
    Collector.only(launchGeneric, {
      case FunctionCallTE(
        PrototypeT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("launch"), _), _, _)), _),
        _, _) =>
    })

    val main = coutputs.lookupFunction("main")
    // No upcast — main passes &Raza directly without coercing to &IShip.
    Collector.all(main, { case UpcastTE(_, _, _) => }).size shouldEqual 0
    // The call site in main resolves to launchGeneric<Raza>.
    Collector.only(main, {
      case FunctionCallTE(
        PrototypeT(
          IdT(_, _, FunctionNameT(
            FunctionTemplateNameT(StrI("launchGeneric"), _),
            Vector(CoordTemplataT(CoordT(_, _, StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("Raza")), _)))))),
            _)),
          _),
        _, _) =>
    })
  }
*/
// mig: fn tests_overload_set_and_concept_function
#[test]
#[ignore = "ignored upstream in Scala"]
fn tests_overload_set_and_concept_function() { panic!("Unmigrated test: tests_overload_set_and_concept_function"); }
/*
  ignore("Tests overload set and concept function") {
    // Search @POSIPP for why this doesn't work.
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
        |  moo("hello", print);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn generic_interface_anonymous_subclass
#[test]
#[ignore = "ignored upstream in Scala"]
fn generic_interface_anonymous_subclass() { panic!("Unmigrated test: generic_interface_anonymous_subclass"); }
/*
  ignore("Generic interface anonymous subclass") {
    val compile = CompilerTestCompilation.test(
      """
        |interface Bork<T Ref> {
        |  func bork(virtual self &Bork<T>, x T) int;
        |}
        |
        |exported func main() int {
        |  f = Bork((x) => { 7 });
        |  return f.bork();
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn lambda_body_type_matches_anonymous_interface_return_type
#[test]
fn lambda_body_type_matches_anonymous_interface_return_type() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\ninterface AFunction1<P Ref> {\n  func __call(virtual this &AFunction1<P>, a P) int;\n}\nexported func main() {\n  arr = AFunction1<int>((_) => { 4 });\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Lambda body type matches anonymous interface return type") {
    val compile = CompilerTestCompilation.test(
      """
        |interface AFunction1<P Ref> {
        |  func __call(virtual this &AFunction1<P>, a P) int;
        |}
        |exported func main() {
        |  arr = AFunction1<int>((_) => { 4 });
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

  // Prot[name, params, return] decomposition is dead syntax — no .vale code uses it,
  // and the func syntax (CallSiteFuncSR/ResolveSR) can't discover an unknown return type
  // from just name+params. The solver requires either the return type or the full prototype
  // to already be known. This test needs the old Prot decomposition feature which was
  // never updated from 2-component to 3-component form in RuleScout.
//  test("Prototype rule to get return type") {
//    val compile = CompilerTestCompilation.test(
//      """
//        |
//        |import v.builtins.panic.*;
//        |
//        |func moo(i int, b bool) str { return "hello"; }
//        |
//        |exported func main() R
//        |where mooFunc Prot = Prot["moo", Refs(int, bool), R Ref] {
//        |  __vbi_panic();
//        |}
//        |
//        |""".stripMargin
//    )
//    val coutputs = compile.expectCompilerOutputs()
//    coutputs.lookupFunction("main").header.returnType match {
//      case CoordT(_,_, StrT()) =>
//    }
//  }
*/
// mig: fn tuple_with_all_imm_fields_is_imm
#[test]
#[ignore = "ignored upstream in Scala"]
fn tuple_with_all_imm_fields_is_imm() { panic!("Unmigrated test: tuple_with_all_imm_fields_is_imm"); }
/*
  ignore("Tuple with all imm fields is imm") {
    // Aspirational. The Builtins ship Tup2<T0, T1> as unconditionally `mut` (see
    // Builtins/src/dev/vale/resources/tup2.vale). A tuple constructed from imm-able
    // primitives like `(true, 42)` therefore produces a `mut` Tup2<bool, int>. To
    // make this test pass, one of:
    //   (a) Tup2 (and Tup3, Tup4...) gain conditional-imm declarations — `imm if all
    //       T0..Tn are imm`, paralleling how Tup0 is unconditionally `imm`.
    //   (b) The compiler auto-promotes a mut tuple struct to imm at instantiation time
    //       when every field-type templata is imm-able.
    //
    // History: variadic `Tup<T RefList>` (pre-Sept-2022) may have had this property;
    // commit c1f24496 ("Found the Milano case, attempting to fix") replaced it with
    // hand-written mut Tup2/Tup3, dropping any imm inference. See Family 4.1 in
    // docs/historical/after-regions-test-fixing-quest.md.
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.tup2.*;
        |import v.builtins.drop.*;
        |
        |exported func main() int {
        |  t = (true, 42);
        |  return t.1;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()

    vassertOne(coutputs.structs.collectFirst({
      case sd @ StructDefinitionT(simpleNameT("Tup2"), _, _, _, MutabilityTemplataT(ImmutableT), _, _, _) => sd
    }))
  }
*/
// mig: fn can_destructure_and_assemble_tuple
#[test]
fn can_destructure_and_assemble_tuple() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.tup2.*;\nimport v.builtins.drop.*;\n\nfunc swap<T, Y>(x (T, Y)) (Y, T) {\n  [a, b] = x;\n  return (b, a);\n}\n\nexported func main() bool {\n  return swap((5, true)).0;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();

    match coutputs.lookup_function_by_str("main").header.return_type {
        CoordT { ownership: OwnershipT::Share, kind: KindT::Bool(_), .. } => {}
        other => panic!("expected CoordT(ShareT, _, BoolT()), got {:?}", other),
    }
}
/*
  test("Can destructure and assemble tuple") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.tup2.*;
        |import v.builtins.drop.*;
        |
        |func swap<T, Y>(x (T, Y)) (Y, T) {
        |  [a, b] = x;
        |  return (b, a);
        |}
        |
        |exported func main() bool {
        |  return swap((5, true)).0;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()

    coutputs.lookupFunction("main").header.returnType match {
      case CoordT(ShareT, _, BoolT()) =>
    }

//    coutputs.lookupFunction("swap").header.fullName.last.templateArgs.last match {
//      case CoordTemplata(CoordT(ShareT, BoolT())) =>
//    }
  }
*/
// mig: fn can_turn_a_borrow_coord_into_an_owning_coord
#[test]
fn can_turn_a_borrow_coord_into_an_owning_coord() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.panicutils.*;\n\nstruct SomeStruct { }\n\nfunc inner<T>() T {\n  panic(\"never\");\n}\n\nfunc bork() ^&SomeStruct {\n  return inner<^&SomeStruct>();\n}\n\nexported func main() {\n  bork();\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    match coutputs.lookup_function_by_str("bork").header.return_type {
        CoordT { ownership: OwnershipT::Own, kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("SomeStruct"), .. }), .. }), .. }, .. }), .. } => {}
        other => panic!("expected CoordT(OwnT, _, StructTT(SomeStruct)), got {:?}", other),
    }
}
/*
  test("Can turn a borrow coord into an owning coord") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.panicutils.*;
        |
        |struct SomeStruct { }
        |
        |func inner<T>() T {
        |  panic("never");
        |}
        |
        |func bork() ^&SomeStruct {
        |  return inner<^&SomeStruct>();
        |}
        |
        |exported func main() {
        |  bork();
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    coutputs.lookupFunction("bork").header.returnType match {
      case CoordT(OwnT, _, StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("SomeStruct")), _)))) =>
    }
  }
*/
// mig: fn impl_rule
#[test]
fn impl_rule() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\n\ninterface IShip {\n  func getFuel(virtual self &IShip) int;\n}\nstruct Firefly {}\nfunc getFuel(self &Firefly) int { return 7; }\nimpl IShip for Firefly;\n\nfunc genericGetFuel<T>(x &T) int\nwhere implements(T, IShip) {\n  return x.getFuel();\n}\n\nexported func main() int {\n  return genericGetFuel(&Firefly());\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let generic_get_fuel = coutputs.lookup_function_by_str("genericGetFuel");
    let template_args = IFunctionNameT::try_from(generic_get_fuel.header.id.local_name).unwrap().template_args();
    match template_args.last().unwrap() {
        ITemplataT::Coord(CoordTemplataT { coord: CoordT { kind: KindT::KindPlaceholder(KindPlaceholderT { id: IdT { local_name: INameT::KindPlaceholder(KindPlaceholderNameT { template: KindPlaceholderTemplateNameT { index: 0, rune: IRuneS::CodeRune(CodeRuneS { name: StrI("T"), .. }), .. } }), .. }, .. }), .. } }) => {}
        other => panic!("expected CoordTemplataT(KindPlaceholderT(T)), got {:?}", other),
    }
    let main = coutputs.lookup_function_by_str("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::FunctionCall(FunctionCallTE {
            callable: PrototypeT {
                id: IdT {
                    local_name: INameT::Function(FunctionNameT {
                        template: FunctionTemplateNameT { human_name: StrI("genericGetFuel"), .. },
                        template_args: &[ITemplataT::Coord(CoordTemplataT { coord: CoordT { kind: KindT::Struct(StructTT { id: IdT { local_name: INameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Firefly"), .. }), .. }), .. }, .. }), .. }, .. })],
                        ..
                    }),
                    ..
                },
                ..
            },
            ..
        }) => Some(())
    );
}
/*
  // Depends on Method call on generic data
  test("Impl rule") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |
        |interface IShip {
        |  func getFuel(virtual self &IShip) int;
        |}
        |struct Firefly {}
        |func getFuel(self &Firefly) int { return 7; }
        |impl IShip for Firefly;
        |
        |func genericGetFuel<T>(x &T) int
        |where implements(T, IShip) {
        |  return x.getFuel();
        |}
        |
        |exported func main() int {
        |  return genericGetFuel(&Firefly());
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    // The typing pass compiles genericGetFuel as a template with placeholder T; per-call-site
    // monomorphization to Firefly happens in the instantiator pass, not here.
    coutputs.lookupFunction("genericGetFuel").header.id.localName.templateArgs.last match {
      case CoordTemplataT(CoordT(_,_, KindPlaceholderT(IdT(_,_,KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, CodeRuneS(StrI("T")))))))) =>
    }
    // The call site in main resolves to a prototype whose callable id contains Firefly.
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case FunctionCallTE(
        PrototypeT(
          IdT(_, _, FunctionNameT(
            FunctionTemplateNameT(StrI("genericGetFuel"), _),
            Vector(CoordTemplataT(CoordT(_, _, StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("Firefly")), _)))))),
            _)),
          _),
        _, _) =>
    })
  }
*/
// mig: fn can_downcast_interface_to_interface_through_registered_impl
#[test]
fn can_downcast_interface_to_interface_through_registered_impl() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.as.*;\nimport v.builtins.result.*;\nimport v.builtins.logic.*;\nimport v.builtins.drop.*;\nimport panicutils.*;\n\nsealed interface ISuper { }\nsealed interface ISub { }\nimpl ISuper for ISub;\n\nfunc tryDowncast(ship ISuper) bool {\n  result Result<&ISub, &ISuper> = (&ship).as<ISub>();\n  return result.is_ok();\n}\n\nexported func main() bool {\n  return tryDowncast(__pretend<ISuper>());\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();

    match coutputs.lookup_function_by_str("tryDowncast").header.return_type {
        CoordT { ownership: OwnershipT::Share, kind: KindT::Bool(_), .. } => {}
        other => panic!("expected CoordT(ShareT, _, BoolT()), got {:?}", other),
    }
}
/*
  test("Can downcast interface to interface through registered impl") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.as.*;
        |import v.builtins.result.*;
        |import v.builtins.logic.*;
        |import v.builtins.drop.*;
        |import panicutils.*;
        |
        |sealed interface ISuper { }
        |sealed interface ISub { }
        |impl ISuper for ISub;
        |
        |func tryDowncast(ship ISuper) bool {
        |  result Result<&ISub, &ISuper> = (&ship).as<ISub>();
        |  return result.is_ok();
        |}
        |
        |exported func main() bool {
        |  return tryDowncast(__pretend<ISuper>());
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    coutputs.lookupFunction("tryDowncast").header.returnType match {
      case CoordT(ShareT, _, BoolT()) =>
    }
  }
*/
// mig: fn test_two_instantiations_of_anonymous_param_lambda
#[test]
fn test_two_instantiations_of_anonymous_param_lambda() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.arith.*;\nimport v.builtins.logic.*;\n\nfunc doThing<T, F>(func F, a T, b T) bool\nwhere func __call(&F, T, T)bool, func drop(F)void {\n  func(a, b)\n}\n\nexported func main() {\n  lam = (a, b) => { a == b };\n  doThing(lam, 7, 8);\n  doThing(lam, true, false);\n}\n\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();

    let lambda_funcs = coutputs.lookup_lambdas_in("main");
    assert_eq!(lambda_funcs.len(), 2);

    let param_type_tuples: HashSet<&[CoordT]> = lambda_funcs.iter().map(|f| match f.header.id.local_name {
        INameT::LambdaCallFunction(LambdaCallFunctionNameT { template: LambdaCallFunctionTemplateNameT { param_types, .. }, .. }) => *param_types,
        _ => panic!("expected LambdaCallFunctionNameT"),
    }).collect();
    assert_eq!(param_type_tuples.len(), 2);
}
/*
  test("Test two instantiations of anonymous-param lambda") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arith.*;
        |import v.builtins.logic.*;
        |
        |func doThing<T, F>(func F, a T, b T) bool
        |where func __call(&F, T, T)bool, func drop(F)void {
        |  func(a, b)
        |}
        |
        |exported func main() {
        |  lam = (a, b) => { a == b };
        |  doThing(lam, 7, 8);
        |  doThing(lam, true, false);
        |}
        |
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    // Per @LAGTNGZ, two call-sites with different arg types produce two distinct
    // LambdaCallFunctionNameT entries in coutputs.functions, sharing one closure struct.
    val lambdaFuncs = coutputs.lookupLambdasIn("main")
    lambdaFuncs.size shouldEqual 2

    val paramTypeTuples =
      lambdaFuncs.map(f => f.header.id.localName match {
        case LambdaCallFunctionNameT(LambdaCallFunctionTemplateNameT(_, paramTypes), _, _) => paramTypes
      }).toSet
    paramTypeTuples.size shouldEqual 2 // distinct per-call-site instantiations
  }
*/
// mig: fn test_interface_default_generic_argument_in_type
#[test]
fn test_interface_default_generic_argument_in_type() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nsealed interface MyInterface<K Ref, H Int = 5> { }\nstruct MyStruct {\n  x MyInterface<bool>;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let moo = coutputs.lookup_struct_by_str("MyStruct");
    let tyype: CoordT = collect_only_tnode!(
        NodeRefT::StructDefinition(moo),
        NodeRefT::ReferenceMemberType(ReferenceMemberTypeT { reference }) => Some(*reference)
    );
    match tyype {
        CoordT {
            ownership: OwnershipT::Own,
            kind: KindT::Interface(InterfaceTT {
                id: IdT {
                    local_name: INameT::Interface(InterfaceNameT {
                        template: InterfaceTemplateNameT { human_namee: StrI("MyInterface"), .. },
                        template_args: &[
                            ITemplataT::Coord(CoordTemplataT { coord: CoordT { ownership: OwnershipT::Share, kind: KindT::Bool(_), .. } }),
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
        other => panic!("expected CoordT(OwnT, _, InterfaceTT(MyInterface<bool,5>)), got {:?}", other),
    }
}
/*
Guardian: temp-disable: SPDMX — Rust enum decision: ITemplataT::Integer holds i64 directly (templata.rs:212), not a wrapping IntegerTemplataT struct. Strong in-file precedent: compiler_solver_tests.rs:320, compiler_mutate_tests.rs:92, compiler_mutate_tests.rs:426 all use ITemplataT::Integer(N) literal form. The struct-wrapped form never compiled. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1165-1781021715924/hook-1165/test_interface_default_generic_argument_in_type--586.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  test("Test interface default generic argument in type") {
    val compile = CompilerTestCompilation.test(
      """
        |sealed interface MyInterface<K Ref, H Int = 5> { }
        |struct MyStruct {
        |  x MyInterface<bool>;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupStruct("MyStruct")
    val tyype = Collector.only(moo, { case ReferenceMemberTypeT(c) => c })
    tyype match {
      case CoordT(
      OwnT,
      _,
      InterfaceTT(
      IdT(_,_,
      InterfaceNameT(
      InterfaceTemplateNameT(StrI("MyInterface")),
      Vector(
      CoordTemplataT(CoordT(ShareT,_, BoolT())),
      IntegerTemplataT(5)))))) =>
    }
  }
*/
// mig: fn reports_when_we_give_too_many_args
#[test]
fn reports_when_we_give_too_many_args() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc moo(a int, b bool, s str) int { a }\nexported func main() int {\n  moo(42, true, \"hello\", false)\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    match compile.get_compiler_outputs().err().unwrap() {
        ICompileErrorT::CouldntFindFunctionToCallT { fff, .. } => {
            assert!(fff.rejected_callee_to_reason.len() == 1);
            match fff.rejected_callee_to_reason[0].1 {
                IFindFunctionFailureReason::WrongNumberOfArguments { supplied: 4, expected: 3 } => {}
                _ => panic!("expected WrongNumberOfArguments(4, 3)"),
            }
        }
        _ => panic!("expected CouldntFindFunctionToCallT"),
    }
}
/*
  test("Reports when we give too many args") {
    val compile = CompilerTestCompilation.test(
      """
        |func moo(a int, b bool, s str) int { a }
        |exported func main() int {
        |  moo(42, true, "hello", false)
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      // Err(     case WrongNumberOfArguments(_, _)) =>
      case Err(CouldntFindFunctionToCallT(_, fff)) => {
        vassert(fff.rejectedCalleeToReason.size == 1)
        fff.rejectedCalleeToReason.head._2 match {
          case WrongNumberOfArguments(4, 3) =>
        }
      }
    }
  }
*/
// mig: fn reports_when_ownership_doesnt_match
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn reports_when_ownership_doesnt_match() { panic!("Unmigrated test: reports_when_ownership_doesnt_match"); }
/*
  test("Reports when ownership doesnt match") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Firefly {}
        |func getFuel(self &Firefly) int { return 7; }
        |
        |exported func main() int {
        |  f = Firefly();
        |  return (f).getFuel();
        |}
        |""".stripMargin
    )
    compile.getCompilerOutputs() match {
      case Err(CouldntFindFunctionToCallT(range, fff)) => {
        fff.name match {
          case CodeNameS(StrI("getFuel")) =>
        }
        fff.rejectedCalleeToReason.size shouldEqual 1
        val reason = fff.rejectedCalleeToReason.head._2
        reason match {
          case FindFunctionResolveFailure(ResolvingSolveFailedOrIncomplete(FailedSolve(_, _, _, _, RuleError(OwnershipDidntMatch(CoordT(OwnT, _, _), BorrowT))))) =>
          case InferFailure(FailedSolve(_, _, _, _, RuleError(OwnershipDidntMatch(CoordT(OwnT, _, _), BorrowT)))) =>
          //          case SpecificParamDoesntSend(0, _, _) =>
          case other => vfail(other)
        }
      }
    }
  }
*/
// mig: fn failure_to_resolve_a_prot_rules_function_doesnt_halt
#[test]
fn failure_to_resolve_a_prot_rules_function_doesnt_halt() {
    // In the below example, it should disqualify the first foo() because T = bool
    // and there exists no moo(bool). Instead, we saw the Prot rule throw and halt
    // compilation.

    // Instead, we need to bubble up that failure to find the right function, so
    // it disqualifies the candidate and goes with the other one.

    // Note from later: It seems this isn't detected by the typing phase anymore.
    // When we try to resolve a func moo(str)void, we actually find one in the overload index,
    // specifically foo.bound:moo(str).
    // Obviously we shouldnt be considering that.
    // Normally, bounds have some sort of placeholder type that acts as a filter so people don't
    // see them unless they have that placeholder type. Here, not so much.

    // We can solve this in two ways:
    // - Making a visibility mask for various overloads in the overload set. This one is only visible from foo,
    //   so when we try to resolve it from main it wont be found.
    // - Require all bounds have a placeholder type in them. Seems reasonable tbh.

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.drop.*;\n\nfunc moo(a str) { }\nfunc foo<T>(f T) void where func drop(T)void, func moo(str)void { }\nfunc foo<T>(f T) void where func drop(T)void, func moo(bool)void { }\nfunc main() { foo(\"hello\"); }\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Failure to resolve a Prot rule's function doesnt halt") {
    // In the below example, it should disqualify the first foo() because T = bool
    // and there exists no moo(bool). Instead, we saw the Prot rule throw and halt
    // compilation.

    // Instead, we need to bubble up that failure to find the right function, so
    // it disqualifies the candidate and goes with the other one.

    // Note from later: It seems this isn't detected by the typing phase anymore.
    // When we try to resolve a func moo(str)void, we actually find one in the overload index,
    // specifically foo.bound:moo(str).
    // Obviously we shouldnt be considering that.
    // Normally, bounds have some sort of placeholder type that acts as a filter so people don't
    // see them unless they have that placeholder type. Here, not so much.

    // We can solve this in two ways:
    // - Making a visibility mask for various overloads in the overload set. This one is only visible from foo,
    //   so when we try to resolve it from main it wont be found.
    // - Require all bounds have a placeholder type in them. Seems reasonable tbh.

    CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |func moo(a str) { }
        |func foo<T>(f T) void where func drop(T)void, func moo(str)void { }
        |func foo<T>(f T) void where func drop(T)void, func moo(bool)void { }
        |func main() { foo("hello"); }
        |""".stripMargin).expectCompilerOutputs()
  }
*/
// mig: fn bound_driven_return_rune_cannot_be_inferred_from_lambda_msae_general
// Canonical minimal repro for @BRRZ. The generic function `callAndReturn` has a
// bound `func(&G)E` where E is an identifying generic rune appearing only in the
// bound's return position. The caller supplies a lambda for G but does not (and
// syntactically cannot) write E. The relaxed ResolveSR (CompilerSolver.scala:636)
// resolves `__call(&closure)` and takes its return type as E.
#[test]
fn bound_driven_return_rune_cannot_be_inferred_from_lambda_msae_general() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc callAndReturn<E, G>(g &G) E\nwhere func(&G)E {\n  return g();\n}\n\nexported func main() int {\n  f = { 7 };\n  return callAndReturn(&f);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  // Canonical minimal repro for @BRRZ. The generic function `callAndReturn` has a
  // bound `func(&G)E` where E is an identifying generic rune appearing only in the
  // bound's return position. The caller supplies a lambda for G but does not (and
  // syntactically cannot) write E. The relaxed ResolveSR (CompilerSolver.scala:636)
  // resolves `__call(&closure)` and takes its return type as E.
  test("Bound-driven return rune cannot be inferred from lambda (MSAE general)") {
    val compile = CompilerTestCompilation.test(
      """
        |func callAndReturn<E, G>(g &G) E
        |where func(&G)E {
        |  return g();
        |}
        |
        |exported func main() int {
        |  f = { 7 };
        |  return callAndReturn(&f);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn brrz_nested_bound_return_inference_through_a_lambda_body
// Edge case for @BRRZ: the lambda body itself invokes another generic function
// with its own bound. Exercises stamping-during-solve recursing into a nested
// generic. The CompilerOutputs.signatureToFunction cache terminates recursion.
#[test]
fn brrz_nested_bound_return_inference_through_a_lambda_body() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc callAndReturn<E, G>(g &G) E\nwhere func(&G)E {\n  return g();\n}\n\nexported func main() int {\n  f = { 7 };\n  g = { callAndReturn(&f) };\n  return callAndReturn(&g);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  // Edge case for @BRRZ: the lambda body itself invokes another generic function
  // with its own bound. Exercises stamping-during-solve recursing into a nested
  // generic. The CompilerOutputs.signatureToFunction cache terminates recursion.
  test("BRRZ: nested bound-return inference through a lambda body") {
    val compile = CompilerTestCompilation.test(
      """
        |func callAndReturn<E, G>(g &G) E
        |where func(&G)E {
        |  return g();
        |}
        |
        |exported func main() int {
        |  f = { 7 };
        |  g = { callAndReturn(&f) };
        |  return callAndReturn(&g);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn brrz_two_bound_return_inferences_in_the_same_call
// Edge case for @BRRZ: two bounds on the same function, each resolving to a
// different lambda. Exercises multiple ResolveSR rules firing in the same solve
// under the relaxed puzzle.
#[test]
fn brrz_two_bound_return_inferences_in_the_same_call() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nfunc applyTwo<E, F, G, H>(g &G, h &H) E\nwhere func(&G)E, func(&H)F {\n  return g();\n}\n\nexported func main() int {\n  a = { 7 };\n  b = { true };\n  return applyTwo(&a, &b);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  // Edge case for @BRRZ: two bounds on the same function, each resolving to a
  // different lambda. Exercises multiple ResolveSR rules firing in the same solve
  // under the relaxed puzzle.
  test("BRRZ: two bound-return inferences in the same call") {
    val compile = CompilerTestCompilation.test(
      """
        |func applyTwo<E, F, G, H>(g &G, h &H) E
        |where func(&G)E, func(&H)F {
        |  return g();
        |}
        |
        |exported func main() int {
        |  a = { 7 };
        |  b = { true };
        |  return applyTwo(&a, &b);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn basic_ifunction1_anonymous_subclass
// Depends on IFunction1, and maybe Generic interface anonymous subclass
#[test]
fn basic_ifunction1_anonymous_subclass() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport ifunction.ifunction1.*;\n\nexported func main() int {\n  f = IFunction1<mut, int, int>({_});\n  return (f)(7);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  // Depends on IFunction1, and maybe Generic interface anonymous subclass
  test("Basic IFunction1 anonymous subclass") {
    val compile = CompilerTestCompilation.test(
      """
        |import ifunction.ifunction1.*;
        |
        |exported func main() int {
        |  f = IFunction1<mut, int, int>({_});
        |  return (f)(7);
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
}
*/
