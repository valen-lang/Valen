use bumpalo::Bump;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::names::names::{FunctionNameT, FunctionTemplateNameT, IdT, INameT};
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::utils::code_hierarchy::{self, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use crate::typing::typing_interner::TypingInterner;

/*
package dev.vale.typing

import dev.vale.typing.ast.{AsSubtypeTE, FunctionHeaderT, PrototypeT, SignatureT}
import dev.vale.typing.names._
import dev.vale.typing.templata.CoordTemplataT
import dev.vale.typing.types._
import dev.vale.{Collector, StrI, Tests, vassert}
import dev.vale.typing.types.InterfaceTT
import org.scalatest._

import scala.collection.immutable.Set

class CompilerVirtualTests extends FunSuite with Matchers {
*/
// mig: fn regular_interface_and_struct
#[test]
fn regular_interface_and_struct() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nsealed interface Opt { }\n\nstruct Some { x int; }\nimpl Opt for Some;\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();

    let drop_func_names: Vec<_> = coutputs.functions.iter().map(|f| f.header.id).filter_map(|f| {
        match f {
            id @ IdT { local_name: INameT::Function(FunctionNameT { template: FunctionTemplateNameT { human_name: StrI("drop"), .. }, .. }), .. } => Some(id),
            _ => None,
        }
    }).collect();
    assert_eq!(drop_func_names.len(), 2);

    let interface = coutputs.lookup_interface_by_human_name("Opt");
    let _ = interface.internal_methods;
}
/*
  test("Regular interface and struct") {
    val compile = CompilerTestCompilation.test(
      """
        |sealed interface Opt { }
        |
        |struct Some { x int; }
        |impl Opt for Some;
      """.stripMargin)
    val interner = compile.interner
    val coutputs = compile.expectCompilerOutputs()

    // Make sure there's two drop functions
    val dropFuncNames =
      coutputs.functions.map(_.header.id).collect({
        case f @ IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, _)) => f
      })
    vassert(dropFuncNames.size == 2)

    val interface = coutputs.lookupInterface("Opt")
    interface.internalMethods
  }
*/
// mig: fn regular_open_interface_and_struct_no_anonymous_interface
#[test]
fn regular_open_interface_and_struct_no_anonymous_interface() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveAnonymousSubstruct\ninterface Opt { }\n\nstruct Some { x int; }\nimpl Opt for Some;\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let drop_func_names: Vec<_> = coutputs.functions.iter().map(|f| f.header.id).filter_map(|f| {
        match f {
            id @ IdT { local_name: INameT::Function(FunctionNameT { template: FunctionTemplateNameT { human_name: StrI("drop"), .. }, .. }), .. } => Some(id),
            _ => None,
        }
    }).collect();
    assert_eq!(drop_func_names.len(), 2);
}
/*
  test("Regular open interface and struct, no anonymous interface") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveAnonymousSubstruct
        |interface Opt { }
        |
        |struct Some { x int; }
        |impl Opt for Some;
      """.stripMargin)
    val interner = compile.interner
    val coutputs = compile.expectCompilerOutputs()

    // Make sure there's two drop functions
    val dropFuncNames =
      coutputs.functions.map(_.header.id).collect({
        case f @ IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, _)) => f
      })
    dropFuncNames.size shouldEqual 2

//    val interface = coutputs.lookupInterface("Opt")
//    interface.internalMethods.collect({
//      case (PrototypeT(FullNameT(_, _, FreeNameT(FreeTemplateNameT(_), _, coord)), _), _) => {
//        vassert(coord.kind == interface.ref)
//      }
//    }).size shouldEqual 1
  }
*/
// mig: fn implementing_two_interfaces_causes_no_vdrop_conflict
#[test]
fn implementing_two_interfaces_causes_no_vdrop_conflict() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nstruct MyStruct {}\n\ninterface IA {}\nimpl IA for MyStruct;\n\ninterface IB {}\nimpl IB for MyStruct;\n\nfunc bork(a IA) {}\nfunc zork(b IB) {}\nexported func main() {\n  bork(MyStruct());\n  zork(MyStruct());\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Implementing two interfaces causes no vdrop conflict") {
    // See NIIRII
    val compile = CompilerTestCompilation.test(
      """
        |struct MyStruct {}
        |
        |interface IA {}
        |impl IA for MyStruct;
        |
        |interface IB {}
        |impl IB for MyStruct;
        |
        |func bork(a IA) {}
        |func zork(b IB) {}
        |exported func main() {
        |  bork(MyStruct());
        |  zork(MyStruct());
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn upcast
#[test]
fn upcast() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\ninterface IShip {}\nstruct Raza { fuel int; }\nimpl IShip for Raza;\n\nexported func main() {\n  ship IShip = Raza(42);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Upcast") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |interface IShip {}
        |struct Raza { fuel int; }
        |impl IShip for Raza;
        |
        |exported func main() {
        |  ship IShip = Raza(42);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn virtual_with_body
#[test]
fn virtual_with_body() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\ninterface IBork { }\nstruct Bork { }\nimpl IBork for Bork;\n\nfunc rebork(virtual result *IBork) bool { true }\nexported func main() {\n  rebork(&Bork());\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let _compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
}
/*
  test("Virtual with body") {
    CompilerTestCompilation.test(
      """
        |interface IBork { }
        |struct Bork { }
        |impl IBork for Bork;
        |
        |func rebork(virtual result *IBork) bool { true }
        |exported func main() {
        |  rebork(&Bork());
        |}
        |""".stripMargin)
  }
*/
// mig: fn templated_interface_and_struct
#[test]
fn templated_interface_and_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nsealed interface Opt<T Ref>\nwhere func drop(T)void\n{ }\n\nstruct Some<T>\nwhere func drop(T)void\n{ x T; }\n\nimpl<T> Opt<T> for Some<T>\nwhere func drop(T)void;\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let drop_func_names: Vec<_> = coutputs.functions.iter().map(|f| f.header.id).filter_map(|f| {
        match f {
            id @ IdT { local_name: INameT::Function(FunctionNameT { template: FunctionTemplateNameT { human_name: StrI("drop"), .. }, .. }), .. } => Some(id),
            _ => None,
        }
    }).collect();
    assert_eq!(drop_func_names.len(), 2);
}
/*
  test("Templated interface and struct") {
    val compile = CompilerTestCompilation.test(
      """
        |sealed interface Opt<T Ref>
        |where func drop(T)void
        |{ }
        |
        |struct Some<T>
        |where func drop(T)void
        |{ x T; }
        |
        |impl<T> Opt<T> for Some<T>
        |where func drop(T)void;
        |""".stripMargin)
    val interner = compile.interner
    val coutputs = compile.expectCompilerOutputs()
    val dropFuncNames =
      coutputs.functions.map(_.header.id).collect({
        case f @ IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, _)) => f
      })
    dropFuncNames.size shouldEqual 2
  }
*/
// mig: fn custom_drop_with_concept_function
#[test]
fn custom_drop_with_concept_function() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveInterfaceDrop\nsealed interface Opt<T Ref> { }\n\nabstract func drop<T>(virtual opt Opt<T>)\nwhere func drop(T)void;\n\n#!DeriveStructDrop\nstruct Some<T> { x T; }\nimpl<T> Opt<T> for Some<T>;\n\nfunc drop<T>(opt Some<T>)\nwhere func drop(T)void\n{\n  [x] = opt;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Custom drop with concept function") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |sealed interface Opt<T Ref> { }
        |
        |abstract func drop<T>(virtual opt Opt<T>)
        |where func drop(T)void;
        |
        |#!DeriveStructDrop
        |struct Some<T> { x T; }
        |impl<T> Opt<T> for Some<T>;
        |
        |func drop<T>(opt Some<T>)
        |where func drop(T)void
        |{
        |  [x] = opt;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn test_complex_interface
#[test]
fn test_complex_interface() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = crate::tests::tests::load_expected("programs/genericvirtuals/templatedinterface.vale");
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Test complex interface") {
    val compile = CompilerTestCompilation.test(
      Tests.loadExpected("programs/genericvirtuals/templatedinterface.vale"))
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn test_specializing_interface
#[test]
fn test_specializing_interface() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = crate::tests::tests::load_expected("programs/genericvirtuals/specializeinterface.vale");
    let resolver = crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Test specializing interface") {
    val compile = CompilerTestCompilation.test(
      Tests.loadExpected("programs/genericvirtuals/specializeinterface.vale"))
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn use_bound_from_struct
#[test]
fn use_bound_from_struct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveStructDrop\nstruct BorkForwarder<Lam>\nwhere func __call(&Lam)int // 3\n{\n  lam Lam;\n}\n\n\nfunc bork<Lam>( // 1\n  self &BorkForwarder<Lam> // 2\n) int {\n  return (self.lam)();\n}\n\nexported func main() {\n  b = BorkForwarder({ 7 });\n  b.bork();\n  [_] = b;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Use bound from struct") {
    // See NBIFP.
    // Without it, when it tries to compile (1), at (2) it tries to resolve BorkForwarder
    // and fails bound (3) because (1) has no such bound.
    // NBIFP says we should first get that knowledge from (2).
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveStructDrop
        |struct BorkForwarder<Lam>
        |where func __call(&Lam)int // 3
        |{
        |  lam Lam;
        |}
        |
        |
        |func bork<Lam>( // 1
        |  self &BorkForwarder<Lam> // 2
        |) int {
        |  return (self.lam)();
        |}
        |
        |exported func main() {
        |  b = BorkForwarder({ 7 });
        |  b.bork();
        |  [_] = b;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn basic_interface_forwarder
#[test]
fn basic_interface_forwarder() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveInterfaceDrop\nsealed interface Bork {\n  func bork(virtual self &Bork) int;\n}\n\n#!DeriveStructDrop\nstruct BorkForwarder<Lam>\nwhere func drop(Lam)void, func __call(&Lam)int {\n  lam Lam;\n}\n\nimpl<Lam> Bork for BorkForwarder<Lam>;\n\nfunc bork<Lam>(self &BorkForwarder<Lam>) int {\n  return (self.lam)();\n}\n\nexported func main() int {\n  f = BorkForwarder({ 7 });\n  z = f.bork();\n  [_] = f;\n  return z;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Basic interface forwarder") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |sealed interface Bork {
        |  func bork(virtual self &Bork) int;
        |}
        |
        |#!DeriveStructDrop
        |struct BorkForwarder<Lam>
        |where func drop(Lam)void, func __call(&Lam)int {
        |  lam Lam;
        |}
        |
        |impl<Lam> Bork for BorkForwarder<Lam>;
        |
        |func bork<Lam>(self &BorkForwarder<Lam>) int {
        |  return (self.lam)();
        |}
        |
        |exported func main() int {
        |  f = BorkForwarder({ 7 });
        |  z = f.bork();
        |  [_] = f;
        |  return z;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn generic_interface_forwarder
#[test]
fn generic_interface_forwarder() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveInterfaceDrop\nsealed interface Bork<T Ref> {\n  func bork(virtual self &Bork<T>) int;\n}\n\n#!DeriveStructDrop\nstruct BorkForwarder<T Ref, Lam>\nwhere func drop(Lam)void, func __call(&Lam)T {\n  lam Lam;\n}\n\nimpl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;\n\nfunc bork<T, Lam>(self &BorkForwarder<T, Lam>) T {\n  return (self.lam)();\n}\n\nexported func main() int {\n  f = BorkForwarder<int>({ 7 });\n  z = f.bork();\n  [_] = f;\n  return z;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Generic interface forwarder") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |sealed interface Bork<T Ref> {
        |  func bork(virtual self &Bork<T>) int;
        |}
        |
        |#!DeriveStructDrop
        |struct BorkForwarder<T Ref, Lam>
        |where func drop(Lam)void, func __call(&Lam)T {
        |  lam Lam;
        |}
        |
        |impl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;
        |
        |func bork<T, Lam>(self &BorkForwarder<T, Lam>) T {
        |  return (self.lam)();
        |}
        |
        |exported func main() int {
        |  f = BorkForwarder<int>({ 7 });
        |  z = f.bork();
        |  [_] = f;
        |  return z;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn generic_interface_forwarder_with_bound
#[test]
fn generic_interface_forwarder_with_bound() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveInterfaceDrop\nsealed interface Bork<T Ref>\nwhere func threeify(T)T {\n  func bork(virtual self &Bork<T>) int;\n}\n\n#!DeriveStructDrop\nstruct BorkForwarder<T Ref, Lam>\nwhere func drop(Lam)void, func __call(&Lam)T, func threeify(T)T {\n  lam Lam;\n}\n\nimpl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;\n\nfunc bork<T, Lam>(self &BorkForwarder<T, Lam>) T {\n  return (self.lam)().threeify();\n}\n\nfunc threeify(x int) int { 3 }\n\nexported func main() int {\n  f = BorkForwarder<int>({ 7 });\n  z = f.bork();\n  [_] = f;\n  return z;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Generic interface forwarder with bound") {
    val compile = CompilerTestCompilation.test(
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
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn basic_interface_anonymous_subclass
#[test]
fn basic_interface_anonymous_subclass() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\ninterface Bork {\n  func bork(virtual self &Bork) int;\n}\n\nexported func main() int {\n  f = Bork({ 7 });\n  return f.bork();\n}\n\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Basic interface anonymous subclass") {
    val compile = CompilerTestCompilation.test(
      """
        |interface Bork {
        |  func bork(virtual self &Bork) int;
        |}
        |
        |exported func main() int {
        |  f = Bork({ 7 });
        |  return f.bork();
        |}
        |
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn integer_is_compatible_with_interface_anonymous_substruct
#[test]
fn integer_is_compatible_with_interface_anonymous_substruct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.drop.*;\ninterface AFunction2<R Ref, P1 Ref> {\n  func doCall(virtual this &AFunction2<R, P1>, a P1) R;\n}\nfunc __call(x6 int, x42 int)str { \"hi\" }\nexported func main() str {\n  func = AFunction2<str, int>(6);\n  return func.doCall(42);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Integer is compatible with interface anonymous substruct") {
    // We had a bug where the forwarder function was trying to solve the interface rules.
    // But the forwarder function is just:
    //   struct Forwarder<R, P1, Lam>
    //   where func __call(Lam, P1)R
    //   { }
    //   func forwarder:__call<R, P1, Lam>(&Forwarder<R, P1, Lam>, P1)R { }
    // and doesn't ever mention the interface.
    // We would just take out any mention of the interface, but it's hard to inherit everything but the interface.
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |interface AFunction2<R Ref, P1 Ref> {
        |  func doCall(virtual this &AFunction2<R, P1>, a P1) R;
        |}
        |func __call(x6 int, x42 int)str { "hi" }
        |exported func main() str {
        |  func = AFunction2<str, int>(6);
        |  return func.doCall(42);
        |}
    """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn lambda_is_compatible_with_interface_anonymous_substruct
#[test]
fn lambda_is_compatible_with_interface_anonymous_substruct() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.str.*;\n\ninterface AFunction2<R Ref, P1 Ref> {\n  func __call(virtual this &AFunction2<R, P1>, a P1) R;\n}\nexported func main() str {\n  func = AFunction2<str, int>((i) => { str(i) });\n  return func(42);\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Lambda is compatible with interface anonymous substruct") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.str.*;
        |
        |interface AFunction2<R Ref, P1 Ref> {
        |  func __call(virtual this &AFunction2<R, P1>, a P1) R;
        |}
        |exported func main() str {
        |  func = AFunction2<str, int>((i) => { str(i) });
        |  return func(42);
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn implementing_a_non_generic_interface_call
#[test]
fn implementing_a_non_generic_interface_call() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveInterfaceDrop\ninterface IObserver<T Ref> { }\n\n#!DeriveStructDrop\nstruct MyThing { }\n\nimpl<T> IObserver<T> for MyThing;\n\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Implementing a non-generic interface call") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |interface IObserver<T Ref> { }
        |
        |#!DeriveStructDrop
        |struct MyThing { }
        |
        |impl<T> IObserver<T> for MyThing;
        |
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn anonymous_substruct_8
#[test]
fn anonymous_substruct_8() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.arrays.*;\n//import array.make.*;\n\ninterface IThing {\n  func __call(virtual self &IThing, i int) int;\n}\n\nstruct MyThing { }\nfunc __call(self &MyThing, i int) int { i }\n\nimpl IThing for MyThing;\n\nexported func main() int {\n  i IThing = MyThing();\n  a = Array<imm, int>(10, &i);\n  return a.3;\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(crate::builtins::builtins::get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Anonymous substruct 8") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arrays.*;
        |//import array.make.*;
        |
        |interface IThing {
        |  func __call(virtual self &IThing, i int) int;
        |}
        |
        |struct MyThing { }
        |func __call(self &MyThing, i int) int { i }
        |
        |impl IThing for MyThing;
        |
        |exported func main() int {
        |  i IThing = MyThing();
        |  a = Array<imm, int>(10, &i);
        |  return a.3;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

}

*/
