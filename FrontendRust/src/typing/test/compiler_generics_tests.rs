use bumpalo::Bump;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::utils::code_hierarchy::{self, IPackageResolver};
use super::compiler_test_compilation::compiler_test_compilation;
use crate::typing::typing_interner::TypingInterner;
use crate::builtins::builtins::get_embedded_modulized_code_map;
use crate::tests::tests::get_package_to_resource_resolver;
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
*/
// mig: struct CompilerGenericsTests
pub struct CompilerGenericsTests;
// mig: impl CompilerGenericsTests
impl CompilerGenericsTests {}
/*
class CompilerGenericsTests extends FunSuite with Matchers {
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
// mig: fn upcasting_with_generic_bounds
#[test]
fn upcasting_with_generic_bounds() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = concat!(
        "\n",
        "import v.builtins.panic.*;\n",
        "import v.builtins.drop.*;\n",
        "\n",
        "#!DeriveInterfaceDrop\n",
        "sealed interface XOpt<T Ref> where func drop(T)void {\n",
        "  func harvest(virtual opt XOpt<T>) &T;\n",
        "}\n",
        "\n",
        "#!DeriveStructDrop\n",
        "struct XNone<T Ref> where func drop(T)void  { }\n",
        "\n",
        "impl<T> XOpt<T> for XNone<T>;\n",
        "\n",
        "func harvest<T>(opt XNone<T>) &T {\n",
        "  __vbi_panic();\n",
        "}\n",
        "\n",
        "exported func main() int {\n",
        "  m XOpt<int> = XNone<int>();\n",
        "  return (m).harvest();\n",
        "}\n",
        "\n",
    );
    let resolver = get_embedded_modulized_code_map(&parse_arena, &parser_keywords)
        .or(code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()]))
        .or(get_package_to_resource_resolver());
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
    );
    let _coutputs = compile.expect_compiler_outputs();
}
/*
  test("Upcasting with generic bounds") {

    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.panic.*;
        |import v.builtins.drop.*;
        |
        |#!DeriveInterfaceDrop
        |sealed interface XOpt<T Ref> where func drop(T)void {
        |  func harvest(virtual opt XOpt<T>) &T;
        |}
        |
        |#!DeriveStructDrop
        |struct XNone<T Ref> where func drop(T)void  { }
        |
        |impl<T> XOpt<T> for XNone<T>;
        |
        |func harvest<T>(opt XNone<T>) &T {
        |  __vbi_panic();
        |}
        |
        |exported func main() int {
        |  m XOpt<int> = XNone<int>();
        |  return (m).harvest();
        |}
        |
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
}
*/
