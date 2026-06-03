/*
package dev.vale

import dev.vale.simplifying.VonHammer
import dev.vale.finalast.YonderH
import dev.vale.passmanager.FullCompilation
import dev.vale.typing._
import dev.vale.typing.types.StrT
import dev.vale.testvm.StructInstanceV
import dev.vale.von.VonInt
import dev.vale.{finalast => m}
import org.scalatest.{FunSuite, Matchers}

*/
// mig: struct ArithmeticTestsA
pub struct ArithmeticTestsA;
/*
class ArithmeticTestsA extends FunSuite with Matchers {
*/
// mig: fn dividing
#[test]
fn dividing() {
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
        "exported func main() int { return 5 / 2; }",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 2 }) => {}
        other => panic!("expected VonInt(2), got {:?}", other),
    }
}
/*
  test("Dividing") {
    val compile = RunCompilation.test("exported func main() int { return 5 / 2; }")
    compile.evalForKind(Vector()) match { case VonInt(2) => }
  }
}
*/
