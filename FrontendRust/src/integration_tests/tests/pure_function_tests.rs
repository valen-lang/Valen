/*
package dev.vale

import dev.vale.simplifying.VonHammer
import dev.vale.finalast.YonderH
import dev.vale.typing._
import dev.vale.typing.types.StrT
import dev.vale.testvm.StructInstanceV
import dev.vale.von.VonInt
import dev.vale.{finalast => m}
import org.scalatest._
*/
// mig: struct PureFunctionTests
pub struct PureFunctionTests;
/*
class PureFunctionTests extends FunSuite with Matchers {
*/
// mig: fn simple_pure_function
#[test]
fn simple_pure_function() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nstruct Engine {\n  fuel int;\n}\nstruct Spaceship {\n  engine Engine;\n}\npure func pfunc(s &Spaceship) int {\n  return s.engine.fuel;\n}\nexported func main() int {\n  s = Spaceship(Engine(10));\n  return pfunc(&s);\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 10 }) => {}
        other => panic!("expected VonInt(10), got {:?}", other),
    }
}
/*
  test("Simple pure function") {
    val compile =
      RunCompilation.test(
        """
          |struct Engine {
          |  fuel int;
          |}
          |struct Spaceship {
          |  engine Engine;
          |}
          |pure func pfunc(s &Spaceship) int {
          |  return s.engine.fuel;
          |}
          |exported func main() int {
          |  s = Spaceship(Engine(10));
          |  return pfunc(&s);
          |}
          |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(10) => }
  }
*/

/*
}

*/
