use bumpalo::Bump;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::postparsing::names::{CodeNameS, IImpreciseNameS};
use crate::scout_arena::ScoutArena;
use crate::typing::ast::expressions::RestackifyTE;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::env::function_environment_t::{ILocalVariableT, ReferenceLocalVariableT};
use crate::typing::names::names::IVarNameT;
use crate::typing::overload_resolver::FindFunctionFailure;
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::utils::code_hierarchy::{self, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::names::names::CodeVarNameT;
use crate::typing::typing_interner::TypingInterner;
use crate::builtins::builtins::get_embedded_modulized_code_map;
use crate::collect_only_tnode;
use std::fs::read_to_string;
use std::path::PathBuf;

/*
package dev.vale.typing

import dev.vale._
import OverloadResolver.FindFunctionFailure
import dev.vale.postparsing.CodeNameS
import dev.vale.typing.ast.RestackifyTE
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.names.CodeVarNameT
import dev.vale.vassert
import dev.vale.typing.templata._
import dev.vale.typing.types._
import org.scalatest._

import scala.collection.immutable.List
import scala.io.Source

class CompilerOwnershipTests extends FunSuite with Matchers {
  // TODO: pull all of the typingpass specific stuff out, the unit test-y stuff
*/
// mig: fn read_code_from_resource
fn read_code_from_resource(resource_filename: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/tests")
        .join(resource_filename);
    read_to_string(&path).expect("readCodeFromResource: file not found")
}
/*
  def readCodeFromResource(resourceFilename: String): String = {
    val is = Source.fromInputStream(getClass().getClassLoader().getResourceAsStream(resourceFilename))
    vassert(is != null)
    is.mkString("")
  }

*/
// mig: fn parenthesized_method_syntax_will_move_instead_of_borrow
#[test]
fn parenthesized_method_syntax_will_move_instead_of_borrow() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct Bork { a int; }\nfunc doSomething(bork Bork) int {\n  return bork.a;\n}\nfunc main() int {\n  bork = Bork(42);\n  return (bork).doSomething();\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Parenthesized method syntax will move instead of borrow") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Bork { a int; }
        |func doSomething(bork Bork) int {
        |  return bork.a;
        |}
        |func main() int {
        |  bork = Bork(42);
        |  return (bork).doSomething();
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn calling_a_method_on_a_returned_own_ref_will_supply_owning_arg
#[test]
fn calling_a_method_on_a_returned_own_ref_will_supply_owning_arg() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct Bork { a int; }\nfunc doSomething(bork Bork) int {\n  return bork.a;\n}\nfunc main() int {\n  return Bork(42).doSomething();\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Calling a method on a returned own ref will supply owning arg") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Bork { a int; }
        |func doSomething(bork Bork) int {
        |  return bork.a;
        |}
        |func main() int {
        |  return Bork(42).doSomething();
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn explicit_borrow_method_call
#[test]
fn explicit_borrow_method_call() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct Bork { a int; }\nfunc doSomething(bork &Bork) int {\n  return bork.a;\n}\nfunc main() int {\n  return Bork(42)&.doSomething();\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Explicit borrow method call") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Bork { a int; }
        |func doSomething(bork &Bork) int {
        |  return bork.a;
        |}
        |func main() int {
        |  return Bork(42)&.doSomething();
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn calling_a_method_on_a_local_will_supply_borrow_ref
#[test]
fn calling_a_method_on_a_local_will_supply_borrow_ref() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct Bork { a int; }\nfunc doSomething(bork &Bork) int {\n  return bork.a;\n}\nfunc main() int {\n  bork = Bork(42);\n  return bork.doSomething();\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Calling a method on a local will supply borrow ref") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Bork { a int; }
        |func doSomething(bork &Bork) int {
        |  return bork.a;
        |}
        |func main() int {
        |  bork = Bork(42);
        |  return bork.doSomething();
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn calling_a_method_on_a_member_will_supply_borrow_ref
#[test]
fn calling_a_method_on_a_member_will_supply_borrow_ref() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nstruct Zork { bork Bork; }\nstruct Bork { a int; }\nfunc doSomething(bork &Bork) int {\n  return bork.a;\n}\nfunc main() int {\n  zork = Zork(Bork(42));\n  return zork.bork.doSomething();\n}\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Calling a method on a member will supply borrow ref") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Zork { bork Bork; }
        |struct Bork { a int; }
        |func doSomething(bork &Bork) int {
        |  return bork.a;
        |}
        |func main() int {
        |  zork = Zork(Bork(42));
        |  return zork.bork.doSomething();
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn no_derived_or_custom_drop_gives_error
#[test]
fn no_derived_or_custom_drop_gives_error() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\n\n#!DeriveStructDrop\nstruct Muta { }\n\nexported func main() {\n  Muta();\n}\n";
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
  test("No derived or custom drop gives error") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |
        |#!DeriveStructDrop
        |struct Muta { }
        |
        |exported func main() {
        |  Muta();
        |}
      """.stripMargin)
    compile.getCompilerOutputs().expectErr() match {
      case CouldntFindFunctionToCallT(_, FindFunctionFailure(CodeNameS(StrI("drop")), _, _)) =>
    }
  }
*/
// mig: fn opt_with_undroppable_contents
#[test]
fn opt_with_undroppable_contents() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n#!DeriveInterfaceDrop\nsealed interface Opt<T> where T Ref { }\n\n#!DeriveStructDrop\nstruct Some<T> where T Ref { value T; }\n\nimpl<T> Opt<T> for Some<T>;\n\nabstract func drop<T>(virtual opt Opt<T>)\nwhere func drop(T)void;\n\nfunc drop<T>(opt Some<T>)\nwhere func drop(T)void\n{\n  [x] = opt;\n}\n\nabstract func get<T>(virtual opt Opt<T>) T;\nfunc get<T>(opt Some<T>) T {\n  [value] = opt;\n  return value;\n}\n\n#!DeriveStructDrop\nstruct Spaceship { }\n\nexported func main() {\n  s Opt<Spaceship> = Some<Spaceship>(Spaceship());\n  // Drops the ship manually\n  [ ] = (s).get();\n}\n\n";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Opt with undroppable contents") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |sealed interface Opt<T> where T Ref { }
        |
        |#!DeriveStructDrop
        |struct Some<T> where T Ref { value T; }
        |
        |impl<T> Opt<T> for Some<T>;
        |
        |abstract func drop<T>(virtual opt Opt<T>)
        |where func drop(T)void;
        |
        |func drop<T>(opt Some<T>)
        |where func drop(T)void
        |{
        |  [x] = opt;
        |}
        |
        |abstract func get<T>(virtual opt Opt<T>) T;
        |func get<T>(opt Some<T>) T {
        |  [value] = opt;
        |  return value;
        |}
        |
        |#!DeriveStructDrop
        |struct Spaceship { }
        |
        |exported func main() {
        |  s Opt<Spaceship> = Some<Spaceship>(Spaceship());
        |  // Drops the ship manually
        |  [ ] = (s).get();
        |}
        |
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
*/
// mig: fn opt_with_undroppable_mutable_ref_contents
#[test]
fn opt_with_undroppable_mutable_ref_contents() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\nimport v.builtins.drop.*;\n\n#!DeriveInterfaceDrop\nsealed interface Opt<T Ref> { }\n\n#!DeriveStructDrop\nstruct Some<T Ref> { value T; }\n\nimpl<T> Opt<T> for Some<T>;\n\nabstract func drop<T>(virtual opt Opt<T>)\nwhere func drop(T)void;\n\nfunc drop<T>(opt Some<T>)\nwhere func drop(T)void\n{\n  [x] = opt;\n}\n\n#!DeriveStructDrop\nstruct Spaceship { }\n\nstruct ContainerWithDerivedDrop {\n  maybeThing Opt<&Spaceship>;\n}\n\nexported func main() {\n  ship = Spaceship();\n  c = ContainerWithDerivedDrop(Some<&Spaceship>(&ship));\n  [ ] = ship;\n}\n\n";
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    compile.expect_compiler_outputs();
}
/*
  test("Opt with undroppable mutable ref contents") {
    // This is here because we had a bug where if we had a Opt<&T> and there was no drop(T)
    // it would error. It should be fine dropping a &T because any borrow is droppable.

    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |#!DeriveInterfaceDrop
        |sealed interface Opt<T Ref> { }
        |
        |#!DeriveStructDrop
        |struct Some<T Ref> { value T; }
        |
        |impl<T> Opt<T> for Some<T>;
        |
        |abstract func drop<T>(virtual opt Opt<T>)
        |where func drop(T)void;
        |
        |func drop<T>(opt Some<T>)
        |where func drop(T)void
        |{
        |  [x] = opt;
        |}
        |
        |#!DeriveStructDrop
        |struct Spaceship { }
        |
        |struct ContainerWithDerivedDrop {
        |  maybeThing Opt<&Spaceship>;
        |}
        |
        |exported func main() {
        |  ship = Spaceship();
        |  c = ContainerWithDerivedDrop(Some<&Spaceship>(&ship));
        |  // Drops c automatically here. This should work, because it found a drop(&Spaceship)
        |  // specifically from builtins' drop.vale.
        |
        |  // And we'll manually drop this, though its not really what the test is testing.
        |  [ ] = ship;
        |}
        |
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
*/
// mig: fn restackify
#[test]
fn restackify() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = read_code_from_resource("programs/restackify.vale");
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Restackify(RestackifyTE {
            variable: ILocalVariableT::Reference(ReferenceLocalVariableT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("ship"), .. }),
                ..
            }),
            ..
        }) => Some(())
    );
}
/*
  test("Restackify") {
    // Allow set on variables that have been moved already, which is useful for linear style.
    val compile =
      CompilerTestCompilation.test(readCodeFromResource("programs/restackify.vale"))
    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, {
      case RestackifyTE(ReferenceLocalVariableT(CodeVarNameT(StrI("ship")), _, _), _) =>
    })
  }
*/
// mig: fn loop_restackify
#[test]
fn loop_restackify() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = read_code_from_resource("programs/loop_restackify.vale");
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Restackify(RestackifyTE {
            variable: ILocalVariableT::Reference(ReferenceLocalVariableT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("ship"), .. }),
                ..
            }),
            ..
        }) => Some(())
    );
}
/*
  test("Loop restackify") {
    // Allow set on variables that have been moved already, which is useful for linear style.
    val compile =
      CompilerTestCompilation.test(readCodeFromResource("programs/loop_restackify.vale"))
    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, {
      case RestackifyTE(ReferenceLocalVariableT(CodeVarNameT(StrI("ship")), _, _), _) =>
    })
  }
*/
// mig: fn destructure_restackify
#[test]
fn destructure_restackify() {

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = read_code_from_resource("programs/destructure_restackify.vale");
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code]))
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver);
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Restackify(RestackifyTE {
            variable: ILocalVariableT::Reference(ReferenceLocalVariableT {
                name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("ship"), .. }),
                ..
            }),
            ..
        }) => Some(())
    );
}
/*
  test("Destructure restackify") {
    // Allow set on variables that have been moved already, which is useful for linear style.
    val compile =
      CompilerTestCompilation.test(readCodeFromResource("programs/destructure_restackify.vale"))
    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, {
      case RestackifyTE(ReferenceLocalVariableT(CodeVarNameT(StrI("ship")), _, _), _) =>
    })
  }
}

*/
