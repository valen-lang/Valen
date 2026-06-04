/*
package dev.vale

import dev.vale.instantiating.ast._
import dev.vale.typing.{ast, types}
import dev.vale.typing.ast.{AbstractT, SignatureT}
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale.testvm.IntV
import dev.vale.typing.ast._
import dev.vale.typing.templata.ITemplataT.{expectCoord, expectCoordTemplata}
import dev.vale.typing.types._
import dev.vale.von.{VonInt, VonStr}
import org.scalatest._
*/
// mig: struct VirtualTests
pub struct VirtualTests;
/*
class VirtualTests extends FunSuite with Matchers {
*/
// mig: fn simple_program_containing_a_virtual_function
#[test]
fn simple_program_containing_a_virtual_function() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nsealed interface I  {}\nfunc doThing(virtual i I) int { return 4; }\nfunc main(i I) int {\n  return doThing(i);\n}\n",
    );
    let interner = compile.interner;
    let coutputs = compile.expect_compiler_outputs();
    let _keywords_ref = &keywords;

    assert_eq!(coutputs.get_all_user_functions().len(), 2);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type,
        crate::typing::types::types::CoordT {
            ownership: crate::typing::types::types::OwnershipT::Share,
            region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
            kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT::I32),
        });

    let test_tld = crate::utils::code_hierarchy::PackageCoordinate::test_tld(&parse_arena, &parser_keywords);
    let interface_template = interner.intern_interface_template_name(crate::typing::names::names::InterfaceTemplateNameT {
        human_namee: scout_arena.intern_str("I"),
        _phantom: std::marker::PhantomData,
    });
    let interface_name = interner.intern_interface_name(crate::typing::names::names::InterfaceNameValT {
        template: interface_template,
        template_args: &[],
    });
    let interface_id = interner.intern_id(crate::typing::names::names::IdValT {
        package_coord: test_tld, init_steps: &[], local_name: crate::typing::names::names::INameT::Interface(interface_name),
    });
    let interface_tt = interner.intern_interface_tt(crate::typing::types::types::InterfaceTTValT { id: *interface_id });
    let i_coord = crate::typing::types::types::CoordT {
        ownership: crate::typing::types::types::OwnershipT::Own,
        region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
        kind: crate::typing::types::types::KindT::Interface(interface_tt),
    };
    let do_thing_template = interner.intern_function_template_name(crate::typing::names::names::FunctionTemplateNameT {
        human_name: scout_arena.intern_str("doThing"),
        code_location: crate::utils::range::CodeLocationS {
            file: scout_arena.intern_file_coordinate(
                scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]),
                "0.vale"),
            offset: 24,
        },
        _phantom: std::marker::PhantomData,
    });
    let do_thing_name = interner.intern_function_name(crate::typing::names::names::FunctionNameValT {
        template: do_thing_template,
        template_args: &[],
        parameters: &[i_coord],
    });
    let do_thing_id = interner.intern_id(crate::typing::names::names::IdValT {
        package_coord: test_tld, init_steps: &[], local_name: crate::typing::names::names::INameT::Function(do_thing_name),
    });
    let do_thing = coutputs.lookup_function_by_signature(
        crate::typing::ast::ast::SignatureT { id: *do_thing_id }).expect("vassertSome");
    assert_eq!(do_thing.header.params[0].virtuality, Some(crate::typing::ast::ast::AbstractT));
}
/*
    test("Simple program containing a virtual function") {
      val compile = RunCompilation.test(
        """
          |sealed interface I  {}
          |func doThing(virtual i I) int { return 4; }
          |func main(i I) int {
          |  return doThing(i);
          |}
        """.stripMargin, false)
      val coutputs = compile.expectCompilerOutputs()
      val interner = compile.interner
      val keywords = compile.keywords

      vassert(coutputs.getAllUserFunctions.size == 2)
      vassert(coutputs.lookupFunction("main").header.returnType == CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))

      val doThing =
        vassertSome(
          coutputs.lookupFunction(
            SignatureT(
              IdT(
                PackageCoordinate.TEST_TLD(interner, keywords),
                Vector.empty,
                interner.intern(
                  FunctionNameT(
                    interner.intern(FunctionTemplateNameT(
                      interner.intern(StrI("doThing")),
                      CodeLocationS(
                        interner.intern(FileCoordinate(
                          interner.intern(PackageCoordinate(interner.intern(StrI("test")),Vector())),"0.vale")), 24))),
                    Vector.empty,
                    Vector(
                      CoordT(
                        OwnT,
                        RegionT(DefaultRegionT),
                        interner.intern(
                          InterfaceTT(
                            IdT(PackageCoordinate.TEST_TLD(interner, keywords), Vector.empty, interner.intern(InterfaceNameT(interner.intern(InterfaceTemplateNameT(interner.intern(StrI("I")))), Vector.empty)))))))))))))
      vassert(doThing.header.params(0).virtuality.get == AbstractT())
    }
*/
// mig: fn can_call_virtual_function
#[test]
fn can_call_virtual_function() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nsealed interface I  {}\nfunc doThing(virtual i I) int { return 4; }\nfunc main(i I) int {\n  return doThing(i);\n}\n",
    );
    let interner = compile.interner;
    let coutputs = compile.expect_compiler_outputs();
    let _keywords_ref = &keywords;

    assert_eq!(coutputs.get_all_user_functions().len(), 2);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type,
        crate::typing::types::types::CoordT {
            ownership: crate::typing::types::types::OwnershipT::Share,
            region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
            kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT::I32),
        });

    let test_tld = crate::utils::code_hierarchy::PackageCoordinate::test_tld(&parse_arena, &parser_keywords);
    let interface_template = interner.intern_interface_template_name(crate::typing::names::names::InterfaceTemplateNameT {
        human_namee: scout_arena.intern_str("I"),
        _phantom: std::marker::PhantomData,
    });
    let interface_name = interner.intern_interface_name(crate::typing::names::names::InterfaceNameValT {
        template: interface_template,
        template_args: &[],
    });
    let interface_id = interner.intern_id(crate::typing::names::names::IdValT {
        package_coord: test_tld, init_steps: &[], local_name: crate::typing::names::names::INameT::Interface(interface_name),
    });
    let interface_tt = interner.intern_interface_tt(crate::typing::types::types::InterfaceTTValT { id: *interface_id });
    let i_coord = crate::typing::types::types::CoordT {
        ownership: crate::typing::types::types::OwnershipT::Own,
        region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
        kind: crate::typing::types::types::KindT::Interface(interface_tt),
    };
    let do_thing_template = interner.intern_function_template_name(crate::typing::names::names::FunctionTemplateNameT {
        human_name: scout_arena.intern_str("doThing"),
        code_location: crate::utils::range::CodeLocationS {
            file: scout_arena.intern_file_coordinate(
                scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]),
                "0.vale"),
            offset: 24,
        },
        _phantom: std::marker::PhantomData,
    });
    let do_thing_name = interner.intern_function_name(crate::typing::names::names::FunctionNameValT {
        template: do_thing_template,
        template_args: &[],
        parameters: &[i_coord],
    });
    let do_thing_id = interner.intern_id(crate::typing::names::names::IdValT {
        package_coord: test_tld, init_steps: &[], local_name: crate::typing::names::names::INameT::Function(do_thing_name),
    });
    let do_thing = coutputs.lookup_function_by_signature(
        crate::typing::ast::ast::SignatureT { id: *do_thing_id }).expect("vassertSome");
    assert_eq!(do_thing.header.params[0].virtuality, Some(crate::typing::ast::ast::AbstractT));
}
/*
  test("Can call virtual function") {
    val compile = RunCompilation.test(
      """
        |sealed interface I  {}
        |func doThing(virtual i I) int { return 4; }
        |func main(i I) int {
        |  return doThing(i);
        |}
      """.stripMargin, false)
    val coutputs = compile.expectCompilerOutputs()
    val interner = compile.interner
    val keywords = compile.keywords

    vassert(coutputs.getAllUserFunctions.size == 2)
    vassert(coutputs.lookupFunction("main").header.returnType == CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))


    val doThing =
      vassertSome(
        coutputs.lookupFunction(
          ast.SignatureT(
            IdT(
              PackageCoordinate.TEST_TLD(interner, keywords),
              Vector.empty,
              interner.intern(
                FunctionNameT(
                  interner.intern(
                    FunctionTemplateNameT(
                      interner.intern(StrI("doThing")),
                      CodeLocationS(
                        interner.intern(FileCoordinate(
                          interner.intern(PackageCoordinate(interner.intern(StrI("test")),Vector())),"0.vale")), 24))),
                  Vector.empty,
                  Vector(
                    CoordT(
                      OwnT,
                      RegionT(DefaultRegionT),
                      interner.intern(
                        InterfaceTT(
                          IdT(PackageCoordinate.TEST_TLD(interner, keywords), Vector.empty, interner.intern(InterfaceNameT(interner.intern(InterfaceTemplateNameT(interner.intern(StrI("I")))), Vector.empty)))))))))))))
    vassert(doThing.header.params(0).virtuality.get == AbstractT())
  }
*/
// mig: fn owning_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn owning_interface() {
    panic!("Unmigrated test: owning_interface");
}
/*
  test("Owning interface") {
    val compile = RunCompilation.test(
      """
        |import v.builtins.opt.*;
        |exported func main() int {
        |  x Opt<int> = Some(7);
        |  return 7;
        |}
        |""".stripMargin, false)
    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn simple_override_with_param_and_bound
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_override_with_param_and_bound() {
    panic!("Unmigrated test: simple_override_with_param_and_bound");
}
/*
  test("Simple override with param and bound") {
    // This is the Serenity case in ROWC.
    val compile = RunCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |sealed interface ISpaceship<E Ref, F Ref, G Ref> { }
        |abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
        |    where func drop(X)void;
        |
        |struct Serenity<A Ref, B Ref, C Ref> { }
        |impl<H, I, J> ISpaceship<H, I, J> for Serenity<H, I, J>;
        |func launch<M, N, P>(self &Serenity<M, N, P>, bork M)
        |    where func drop(M)void { }
        |
        |exported func main() {
        |  ship ISpaceship<int, bool, str> = Serenity<int, bool, str>();
        |  ship.launch(7);
        |}
        |""".stripMargin, false)
    compile.evalForKind(Vector())
  }
*/
// mig: fn struct_with_different_ordered_runes
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn struct_with_different_ordered_runes() {
    panic!("Unmigrated test: struct_with_different_ordered_runes");
}
/*
  test("Struct with different ordered runes") {
    // This is the Firefly case in ROWC.
    val compile = RunCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |sealed interface ISpaceship<E Ref, F Ref, G Ref> { }
        |abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
        |    where func drop(X)void;
        |
        |struct Firefly<A Ref, B Ref, C Ref> { }
        |impl<H, I, J> ISpaceship<H, I, J> for Firefly<J, I, H>;
        |func launch<M, N, P>(self &Firefly<M, N, P>, bork P)
        |    where func drop(P)void { }
        |
        |exported func main() {
        |  ship ISpaceship<int, bool, str> = Firefly<str, bool, int>();
        |  ship.launch(7);
        |}
        |""".stripMargin, false)
    compile.evalForKind(Vector())
  }
*/
// mig: fn struct_with_less_generic_params_than_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn struct_with_less_generic_params_than_interface() {
    panic!("Unmigrated test: struct_with_less_generic_params_than_interface");
}
/*
  test("Struct with less generic params than interface") {
    // This is the Raza case in ROWC.
    val compile = RunCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |sealed interface ISpaceship<E Ref, F Ref, G Ref> { }
        |abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
        |    where func drop(X)void;
        |
        |struct Raza<B Ref, C Ref> { }
        |impl<I, J> ISpaceship<int, I, J> for Raza<I, J>;
        |func launch<N, P>(self &Raza<N, P>, bork int) { }
        |
        |exported func main() {
        |  ship ISpaceship<int, bool, str> = Raza<bool, str>();
        |  ship.launch(7);
        |}
        |""".stripMargin, false)
    compile.evalForKind(Vector())
  }
*/
// mig: fn struct_with_more_generic_params_than_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn struct_with_more_generic_params_than_interface() {
    panic!("Unmigrated test: struct_with_more_generic_params_than_interface");
}
/*
  test("Struct with more generic params than interface") {
    // This is the Milano case in ROWC.
    val compile = RunCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |sealed interface ISpaceship<E Ref, F Ref, G Ref> { }
        |abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
        |    where func drop(X)void;
        |
        |struct Milano<A Ref, B Ref, C Ref, D Ref> { }
        |impl<H, I, J, K> ISpaceship<H, I, J> for Milano<H, I, J, K>;
        |func launch<H, I, J, K>(self &Milano<H, I, J, K>, bork H) where func drop(H)void { }
        |
        |exported func main() {
        |  ship ISpaceship<int, bool, str> = Milano<int, bool, str, float>();
        |  ship.launch(7);
        |}
        |""".stripMargin, false)
    compile.evalForKind(Vector())
  }
*/
// mig: fn struct_repeating_generic_params_for_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn struct_repeating_generic_params_for_interface() {
    panic!("Unmigrated test: struct_repeating_generic_params_for_interface");
}
/*
  test("Struct repeating generic params for interface") {
    // This is the Enterprise case in ROWC.
    val compile = RunCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |sealed interface ISpaceship<E Ref, F Ref, G Ref> { }
        |abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
        |    where func drop(X)void;
        |
        |struct Enterprise<A Ref> { }
        |impl<H> ISpaceship<H, H, H> for Enterprise<H>;
        |func launch<H>(self &Enterprise<H>, bork H) where func drop(H)void { }
        |
        |exported func main() {
        |  ship ISpaceship<int, int, int> = Enterprise<int>();
        |  ship.launch(7);
        |}
        |""".stripMargin, false)
    compile.evalForKind(Vector())
  }
*/
// mig: fn imm_interface
#[test]
fn imm_interface() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let source = crate::tests::tests::load_expected("programs/virtuals/interfaceimm.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Imm interface") {
    val compile = RunCompilation.test(
      Tests.loadExpected("programs/virtuals/interfaceimm.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn can_call_interface_envs_function_from_outside
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn can_call_interface_envs_function_from_outside() {
    panic!("Unmigrated test: can_call_interface_envs_function_from_outside");
}
/*
  test("Can call interface env's function from outside") {
    val compile = RunCompilation.test(
      """
        |sealed interface I {
        |  func doThing(virtual i I) int;
        |}
        |func main(i I) int {
        |  return doThing(i);
        |}
      """.stripMargin, false)
    val coutputs = compile.expectCompilerOutputs()
    val interner = compile.interner

    vassert(coutputs.getAllUserFunctions.size == 1)
    vassert(coutputs.lookupFunction("main").header.returnType == CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))


    val doThing = coutputs.lookupFunction("doThing")
    vassert(doThing.header.params(0).virtuality.get == AbstractT())
  }

*/
// mig: fn interface_with_method_with_param_of_substruct
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn interface_with_method_with_param_of_substruct() {
    panic!("Unmigrated test: interface_with_method_with_param_of_substruct");
}
/*
  test("Interface with method with param of substruct") {
    val compile = RunCompilation.test(
        """
          |struct List<T Ref> { }
          |
          |sealed interface SectionMember {}
          |struct Header {}
          |impl SectionMember for Header;
          |abstract func collectHeaders2(header &List<&Header>, virtual this &SectionMember);
          |func collectHeaders2(header &List<&Header>, this &Header) { }
        """.stripMargin)
    val coutputs = compile.getHamuts()
  }
*/
// mig: fn feeding_instantiation_bounds_for_something_created_in_same_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn feeding_instantiation_bounds_for_something_created_in_same_function() {
    panic!("Unmigrated test: feeding_instantiation_bounds_for_something_created_in_same_function");
}
/*
  test("Feeding instantiation bounds for something created in same function") {
    val compile = RunCompilation.test(
      """
        |#!DeriveStructDrop
        |struct Spork<T Ref, Y>
        |where func splork(Y)void {
        |  lam Y;
        |}
        |
        |func bork<T, Y>(
        |  self &Spork<T, Y> // It had trouble here finding the bound for splork
        |) { }
        |
        |func splork(x int) {}
        |
        |exported func main() int {
        |  f = Spork<int>(42);
        |  f.bork(); // We should be feeding in Spork's instantiation bounds here for the params' reachables?
        |  [z] = f;
        |  return z;
        |}
  """.stripMargin)
    compile.evalForKind(Vector())
  }
*/
// mig: fn generic_interface_forwarder_with_bound
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn generic_interface_forwarder_with_bound() {
    panic!("Unmigrated test: generic_interface_forwarder_with_bound");
}
/*
  test("Generic interface forwarder with bound") {
    val compile = RunCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |sealed interface Bork<T Ref>
        |where func threeify(T)T {
        |  func bork(virtual self &Bork<T>) int;
        |}
        |
        |#!DeriveStructDrop
        |struct BorkForwarder<T Ref, Lam>
        |where func drop(Lam)void, func __call(&Lam)T, func threeify(T)T {
        |  lam Lam;
        |}
        |
        |impl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;
        |
        |func bork<T, Lam>(self &BorkForwarder<T, Lam>) T {
        |  return (self.lam)().threeify();
        |}
        |
        |func threeify(x int) int { 3 }
        |
        |exported func main() int {
        |  f = BorkForwarder<int>({ 7 });
        |  z = f.bork();
        |  [_] = f;
        |  return z;
        |}
    """.stripMargin)
    compile.evalForKind(Vector())
  }
*/
// mig: fn generic_interface_forwarder_with_drop_bound
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn generic_interface_forwarder_with_drop_bound() {
    panic!("Unmigrated test: generic_interface_forwarder_with_drop_bound");
}
/*
  test("Generic interface forwarder with drop bound") {
    val compile = RunCompilation.test(
      """
        |sealed interface Bork<T Ref>
        |where func threeify(T)T {
        |  func bork(virtual self &Bork<T>) int;
        |}
        |
        |struct BorkForwarder<T Ref, Lam>
        |where func drop(Lam)void, func __call(&Lam)T, func threeify(T)T {
        |  lam Lam;
        |}
        |
        |impl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;
        |
        |func bork<T, Lam>(self &BorkForwarder<T, Lam>) T {
        |  return (self.lam)().threeify();
        |}
        |
        |func threeify(x int) int { 3 }
        |
        |exported func main() int {
        |  f = BorkForwarder<int>({ 7 });
        |  return f.bork();
        |}
  """.stripMargin)
    compile.evalForKind(Vector())
  }
*/
// mig: fn open_interface_constructor
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn open_interface_constructor() {
    panic!("Unmigrated test: open_interface_constructor");
}
/*
  test("Open interface constructor") {
    val compile = RunCompilation.test(
      """
        |interface Bipedal {
        |  func hop(virtual s &Bipedal) int;
        |}
        |
        |func hopscotch(s &Bipedal) int {
        |  s.hop();
        |  return s.hop();
        |}
        |
        |exported func main() int {
        |   x = Bipedal({ 3 });
        |  // x is an unnamed substruct which implements Bipedal.
        |
        |  return hopscotch(&x);
        |}
        """.stripMargin, false)
    val coutputs = compile.getHamuts()
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn open_interface_constructor_multiple_methods
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn open_interface_constructor_multiple_methods() {
    panic!("Unmigrated test: open_interface_constructor_multiple_methods");
}
/*
  test("Open interface constructor, multiple methods") {
    val compile = RunCompilation.test(
        """
          |interface Bipedal {
          |  func hop(virtual s &Bipedal) int;
          |  func skip(virtual s &Bipedal) int;
          |}
          |
          |struct Human {  }
          |func hop(s &Human) int { return 7; }
          |func skip(s &Human) int { return 9; }
          |impl Bipedal for Human;
          |
          |func hopscotch(s &Bipedal) int {
          |  s.hop();
          |  s.skip();
          |  return s.hop();
          |}
          |
          |exported func main() int {
          |   x = Bipedal({ 3 }, { 5 });
          |  // x is an unnamed substruct which implements Bipedal.
          |
          |  return hopscotch(&x);
          |}
        """.stripMargin)
    val coutputs = compile.getHamuts()
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
/*
//  test("Successful borrow downcast with as") {
//    val compile = RunCompilation.test(
//      Tests.loadExpected("programs/downcast/downcastBorrowSuccessful.vale"))
//    compile.evalForKind(Vector()) match { case VonInt(42) => }
//  }
//
//  test("Failed borrow downcast with as") {
//    val compile = RunCompilation.test(
//      Tests.loadExpected("programs/downcast/downcastBorrowFailed.vale"))
//    compile.evalForKind(Vector()) match { case VonInt(42) => }
//  }
*/
// mig: fn successful_pointer_downcast_with_as
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn successful_pointer_downcast_with_as() {
    panic!("Unmigrated test: successful_pointer_downcast_with_as");
}
/*
  test("Successful pointer downcast with as") {
    val compile = RunCompilation.test(
      Tests.loadExpected("programs/downcast/downcastPointerSuccess.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn failed_pointer_downcast_with_as
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn failed_pointer_downcast_with_as() {
    panic!("Unmigrated test: failed_pointer_downcast_with_as");
}
/*
  test("Failed pointer downcast with as") {
    val compile = RunCompilation.test(
      Tests.loadExpected("programs/downcast/downcastPointerFailed.vale"))

    {
      val moo = compile.expectCompilerOutputs().lookupFunction("moo")
      val (destVar, returnType) =
        Collector.only(moo, {
          case LetNormalTE(destVar, FunctionCallTE(PrototypeT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("as"), _), _, _)), returnType), _, _)) => {
            (destVar, returnType)
          }
        })
      vassert(destVar.coord == returnType)
      val Vector(successType, failType) = returnType.kind.expectInterface().id.localName.templateArgs
      vassert(expectCoordTemplata(successType).coord.ownership == BorrowT)
      vassert(expectCoordTemplata(failType).coord.ownership == BorrowT)
    }

    {
      val moo = compile.getMonouts().lookupFunction("moo")
      val (destVar, returnType) =
        Collector.only(moo, {
          case LetNormalIE(destVar, FunctionCallIE(PrototypeI(IdI(_, _, FunctionNameIX(FunctionTemplateNameI(StrI("as"), _), _, _)), returnType), _, _), _) => {
            (destVar, returnType)
          }
        })
      vassert(destVar.collapsedCoord == returnType)
      val Vector(successType, failType) = returnType.kind.expectInterface().id.localName.templateArgs
      vassert(successType.expectCoordTemplata().coord.ownership == MutableBorrowI)
      vassert(failType.expectCoordTemplata().coord.ownership == MutableBorrowI)
    }

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn successful_owning_downcast_with_as
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn successful_owning_downcast_with_as() {
    panic!("Unmigrated test: successful_owning_downcast_with_as");
}
/*
  test("Successful owning downcast with as") {
    val compile = RunCompilation.test(
      Tests.loadExpected("programs/downcast/downcastOwningSuccessful.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn failed_owning_downcast_with_as
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn failed_owning_downcast_with_as() {
    panic!("Unmigrated test: failed_owning_downcast_with_as");
}
/*
  test("Failed owning downcast with as") {
    val compile = RunCompilation.test(
      Tests.loadExpected("programs/downcast/downcastOwningFailed.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn lambda_is_compatible_anonymous_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn lambda_is_compatible_anonymous_interface() {
    panic!("Unmigrated test: lambda_is_compatible_anonymous_interface");
}
/*
  test("Lambda is compatible anonymous interface") {
    val compile = RunCompilation.test(
      """
        |import castutils.*;
        |
        |interface AFunction2<R Ref, P1 Ref, P2 Ref> {
        |  func __call(virtual this &AFunction2<R, P1, P2>, a P1, b P2) R;
        |}
        |exported func main() str {
        |  func = AFunction2<str, int, bool>((i, b) => { str(i) + str(b) });
        |  return func(42, true);
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonStr("42true") => }
  }
}

*/
