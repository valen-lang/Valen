/*
package dev.vale

import dev.vale.typing.ast.{BorrowToWeakTE, LetNormalTE, SoftLoadTE}
import dev.vale.typing.citizen.WeakableImplingMismatch
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.expression.TookWeakRefOfNonWeakableError
import dev.vale.typing.names.{CodeVarNameT, IdT}
import dev.vale.typing.templata.simpleNameT
import dev.vale.typing.types._
import dev.vale.testvm.ConstraintViolatedException
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.names.CodeVarNameT
import dev.vale.typing.types._
import dev.vale.von.VonInt
import org.scalatest._
*/
// mig: struct WeakTests
pub struct WeakTests;
/*
class WeakTests extends FunSuite with Matchers {
*/
// mig: fn make_and_lock_weak_ref_then_destroy_own_with_struct
#[test]
fn make_and_lock_weak_ref_then_destroy_own_with_struct() {
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
    let source = crate::tests::tests::load_expected("programs/weaks/lockWhileLiveStruct.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::LetNormal(crate::typing::ast::expressions::LetNormalTE {
                variable: crate::typing::env::function_environment_t::ILocalVariableT::Reference(crate::typing::env::function_environment_t::ReferenceLocalVariableT {
                    name: crate::typing::names::names::IVarNameT::CodeVar(crate::typing::names::names::CodeVarNameT { name: crate::interner::StrI("weakMuta"), .. }),
                    variability: crate::typing::types::types::VariabilityT::Final,
                    coord: crate::typing::types::types::CoordT { ownership: crate::typing::types::types::OwnershipT::Weak, .. },
                }),
                expr: ref_expr,
            }) => match ref_expr.result().coord {
                crate::typing::types::types::CoordT {
                    ownership: crate::typing::types::types::OwnershipT::Weak,
                    kind: crate::typing::types::types::KindT::Struct(crate::typing::types::types::StructTT {
                        id: crate::typing::names::names::IdT {
                            local_name: crate::typing::names::names::INameT::Struct(crate::typing::names::names::StructNameT {
                                template: crate::typing::names::names::IStructTemplateNameT::StructTemplate(crate::typing::names::names::StructTemplateNameT { human_name: crate::interner::StrI("Muta"), .. }),
                                ..
                            }),
                            ..
                        },
                        ..
                    }),
                    ..
                } => Some(()),
                _ => None,
            }
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}
/*
  test("Make and lock weak ref then destroy own, with struct") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/lockWhileLiveStruct.vale"))

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, {
      case LetNormalTE(ReferenceLocalVariableT(CodeVarNameT(StrI("weakMuta")),FinalT,CoordT(WeakT, _, _)),refExpr) => {
        refExpr.result.coord match {
          case CoordT(WeakT, _, StructTT(simpleNameT("Muta"))) =>
        }
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn destroy_own_then_locking_gives_none_with_struct
#[test]
fn destroy_own_then_locking_gives_none_with_struct() {
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
    let source = crate::tests::tests::load_expected("programs/weaks/dropThenLockStruct.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Destroy own then locking gives none, with struct") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/dropThenLockStruct.vale"))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn drop_while_locked_with_struct
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn drop_while_locked_with_struct() {
    panic!("Unmigrated test: drop_while_locked_with_struct");
}
/*
  test("Drop while locked, with struct") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/dropWhileLockedStruct.vale"))

    try {
      compile.evalForKind(Vector()) match { case VonInt(42) => }
      vfail()
    } catch {
      case ConstraintViolatedException(_) =>
      case _ => vfail()
    }
  }
*/
// mig: fn make_and_lock_weak_ref_from_borrow_local_then_destroy_own_with_struct
#[test]
fn make_and_lock_weak_ref_from_borrow_local_then_destroy_own_with_struct() {
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
    let source = crate::tests::tests::load_expected("programs/weaks/weakFromLocalCRefStruct.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches: Vec<()> = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::SoftLoad(crate::typing::ast::expressions::SoftLoadTE { target_ownership: crate::typing::types::types::OwnershipT::Weak, .. }) => Some(())
        );
        assert!(matches.len() >= 1);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}
/*
  test("Make and lock weak ref from borrow local then destroy own, with struct") {
    val compile = RunCompilation.test(
      Tests.loadExpected("programs/weaks/weakFromLocalCRefStruct.vale"))

    val main = compile.expectCompilerOutputs().lookupFunction("main")

    vassert(Collector.all(main.body, { case SoftLoadTE(_, WeakT) => true }).size >= 1)

    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn make_and_lock_weak_ref_from_borrow_then_destroy_own_with_struct
#[test]
fn make_and_lock_weak_ref_from_borrow_then_destroy_own_with_struct() {
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
    let source = crate::tests::tests::load_expected("programs/weaks/weakFromCRefStruct.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches: Vec<()> = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::SoftLoad(crate::typing::ast::expressions::SoftLoadTE { target_ownership: crate::typing::types::types::OwnershipT::Weak, .. }) => Some(())
        );
        assert!(matches.len() >= 1);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}
/*
  test("Make and lock weak ref from borrow then destroy own, with struct") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/weakFromCRefStruct.vale"))

    val main = compile.expectCompilerOutputs().lookupFunction("main")

    vassert(Collector.all(main.body, { case SoftLoadTE(_, WeakT) => true }).size >= 1)

    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn make_weak_ref_from_temporary
#[test]
fn make_weak_ref_from_temporary() {
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
        "\nweakable struct Muta { hp int; }\nfunc getHp(weakMuta &&Muta) int { return (lock(weakMuta)).get().hp; }\nexported func main() int { return getHp(&&Muta(7)); }\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::BorrowToWeak(_) => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}
/*
  test("Make weak ref from temporary") {
    val compile = RunCompilation.test(
        """
          |weakable struct Muta { hp int; }
          |func getHp(weakMuta &&Muta) int { return (lock(weakMuta)).get().hp; }
          |exported func main() int { return getHp(&&Muta(7)); }
          |""".stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main.body, { case BorrowToWeakTE(_) => })
    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn make_and_lock_weak_ref_then_destroy_own_with_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn make_and_lock_weak_ref_then_destroy_own_with_interface() {
    panic!("Unmigrated test: make_and_lock_weak_ref_then_destroy_own_with_interface");
}
/*
  test("Make and lock weak ref then destroy own, with interface") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/lockWhileLiveInterface.vale"))

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, {
      case LetNormalTE(ReferenceLocalVariableT(CodeVarNameT(StrI("weakUnit")),FinalT,CoordT(WeakT, _, _)),refExpr) => {
        refExpr.result.coord match {
          case CoordT(WeakT, _, InterfaceTT(simpleNameT("IUnit"))) =>
        }
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn destroy_own_then_locking_gives_none_with_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn destroy_own_then_locking_gives_none_with_interface() {
    panic!("Unmigrated test: destroy_own_then_locking_gives_none_with_interface");
}
/*
  test("Destroy own then locking gives none, with interface") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/dropThenLockInterface.vale"))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn drop_while_locked_with_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn drop_while_locked_with_interface() {
    panic!("Unmigrated test: drop_while_locked_with_interface");
}
/*
  test("Drop while locked, with interface") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/dropWhileLockedInterface.vale"))

    try {
      compile.evalForKind(Vector()) match { case VonInt(42) => }
      vfail()
    } catch {
      case ConstraintViolatedException(_) =>
      case other => vfail(other)
    }
  }
*/
// mig: fn make_and_lock_weak_ref_from_borrow_local_then_destroy_own_with_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn make_and_lock_weak_ref_from_borrow_local_then_destroy_own_with_interface() {
    panic!("Unmigrated test: make_and_lock_weak_ref_from_borrow_local_then_destroy_own_with_interface");
}
/*
  test("Make and lock weak ref from borrow local then destroy own, with interface") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/weakFromLocalCRefInterface.vale"))

    val main = compile.expectCompilerOutputs().lookupFunction("main")

    vassert(Collector.all(main.body, { case SoftLoadTE(_, WeakT) => true }).size >= 1)

    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn make_and_lock_weak_ref_from_borrow_then_destroy_own_with_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn make_and_lock_weak_ref_from_borrow_then_destroy_own_with_interface() {
    panic!("Unmigrated test: make_and_lock_weak_ref_from_borrow_then_destroy_own_with_interface");
}
/*
  test("Make and lock weak ref from borrow then destroy own, with interface") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/weakFromCRefInterface.vale"))

    val main = compile.expectCompilerOutputs().lookupFunction("main")

    vassert(Collector.all(main.body, { case SoftLoadTE(_, WeakT) => true }).size >= 1)

    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn call_weak_self_method_after_drop
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn call_weak_self_method_after_drop() {
    panic!("Unmigrated test: call_weak_self_method_after_drop");
}
/*
  test("Call weak-self method, after drop") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/callWeakSelfMethodAfterDrop.vale"))

    val main = compile.expectCompilerOutputs().lookupFunction("main")

    vassert(Collector.all(main.body, { case SoftLoadTE(_, WeakT) => true }).size >= 1)

    val hamuts = compile.getHamuts()

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn call_weak_self_method_while_alive
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn call_weak_self_method_while_alive() {
    panic!("Unmigrated test: call_weak_self_method_while_alive");
}
/*
  test("Call weak-self method, while alive") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/weaks/callWeakSelfMethodWhileLive.vale"))

    val main = compile.expectCompilerOutputs().lookupFunction("main")

    vassert(Collector.all(main.body, { case SoftLoadTE(_, WeakT) => true }).size >= 1)

    val hamuts = compile.getHamuts()

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn weak_yonder_member
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn weak_yonder_member() {
    panic!("Unmigrated test: weak_yonder_member");
}
/*
  test("Weak yonder member") {
    val compile = RunCompilation.test(
        """
          |weakable struct Base {
          |  hp int;
          |}
          |struct Spaceship {
          |  origin &&Base;
          |}
          |exported func main() int {
          |  base = Base(73);
          |  ship = Spaceship(&&base);
          |
          |  (base).drop(); // Destroys base.
          |
          |  maybeOrigin = lock(ship.origin); «14»«15»
          |  if (not maybeOrigin.isEmpty()) { «16»
          |    o = maybeOrigin.get();
          |    return o.hp;
          |  } else {
          |    return 42;
          |  }
          |}
          |""".stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")

    val hamuts = compile.getHamuts()

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }


}

*/
