use crate::collect_only_tnode;
use crate::collect_where_tnode;
use crate::integration_tests::tests::run_compilation::test;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::tests::tests::load_expected;
use crate::testvm::vivem::VmRuntimeErrorV;
use crate::typing::ast::expressions::LetNormalTE;
use crate::typing::ast::expressions::SoftLoadTE;
use crate::typing::env::function_environment_t::ILocalVariableT;
use crate::typing::env::function_environment_t::ReferenceLocalVariableT;
use crate::typing::names::names::CodeVarNameT;
use crate::typing::names::names::INameT;
use crate::typing::names::names::IStructTemplateNameT;
use crate::typing::names::names::IVarNameT;
use crate::typing::names::names::IdT;
use crate::typing::names::names::InterfaceNameT;
use crate::typing::names::names::InterfaceTemplateNameT;
use crate::typing::names::names::StructNameT;
use crate::typing::names::names::StructTemplateNameT;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::types::types::CoordT;
use crate::typing::types::types::InterfaceTT;
use crate::typing::types::types::KindT;
use crate::typing::types::types::OwnershipT;
use crate::typing::types::types::StructTT;
use crate::typing::types::types::VariabilityT;
use crate::typing::typing_interner::TypingInterner;
use crate::von::ast::IVonData;
use crate::von::ast::VonInt;
/*
package dev.vale

import dev.vale.typing.ast.{BorrowToWeakTE, LetNormalTE, SoftLoadTE}
import dev.vale.typing.{WeakableImplingMismatch, TookWeakRefOfNonWeakableError}
import dev.vale.typing.env.ReferenceLocalVariableT
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
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/lockWhileLiveStruct.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        collect_only_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::LetNormal(LetNormalTE {
                variable: ILocalVariableT::Reference(ReferenceLocalVariableT {
                    name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("weakMuta"), .. }),
                    variability: VariabilityT::Final,
                    coord: CoordT { ownership: OwnershipT::Weak, .. },
                }),
                expr: ref_expr,
            }) => match ref_expr.result().coord {
                CoordT {
                    ownership: OwnershipT::Weak,
                    kind: KindT::Struct(StructTT {
                        id: IdT {
                            local_name: INameT::Struct(StructNameT {
                                template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Muta"), .. }),
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
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
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
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/dropThenLockStruct.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
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
fn drop_while_locked_with_struct() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/dropWhileLockedStruct.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        Ok(_) => panic!("vfail"),
        Err(VmRuntimeErrorV::ConstraintViolatedException(_)) => {}
        Err(_) => panic!("vfail"),
    }
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
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/weakFromLocalCRefStruct.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches: Vec<()> = collect_where_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::SoftLoad(SoftLoadTE { target_ownership: OwnershipT::Weak, .. }) => Some(())
        );
        assert!(matches.len() >= 1);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
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
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/weakFromCRefStruct.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches: Vec<()> = collect_where_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::SoftLoad(SoftLoadTE { target_ownership: OwnershipT::Weak, .. }) => Some(())
        );
        assert!(matches.len() >= 1);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
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
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nweakable struct Muta { hp int; }\nfunc getHp(weakMuta &&Muta) int { return (lock(weakMuta)).get().hp; }\nexported func main() int { return getHp(&&Muta(7)); }\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        collect_only_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::BorrowToWeak(_) => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
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
fn make_and_lock_weak_ref_then_destroy_own_with_interface() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/lockWhileLiveInterface.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        collect_only_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::LetNormal(LetNormalTE {
                variable: ILocalVariableT::Reference(ReferenceLocalVariableT {
                    name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("weakUnit"), .. }),
                    variability: VariabilityT::Final,
                    coord: CoordT { ownership: OwnershipT::Weak, .. },
                }),
                expr: ref_expr,
            }) => match ref_expr.result().coord {
                CoordT {
                    ownership: OwnershipT::Weak,
                    kind: KindT::Interface(InterfaceTT {
                        id: IdT {
                            local_name: INameT::Interface(InterfaceNameT {
                                template: InterfaceTemplateNameT { human_namee: StrI("IUnit"), .. },
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
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
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
fn destroy_own_then_locking_gives_none_with_interface() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/dropThenLockInterface.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
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
fn drop_while_locked_with_interface() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/dropWhileLockedInterface.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        Ok(_) => panic!("vfail"),
        Err(VmRuntimeErrorV::ConstraintViolatedException(_)) => {}
        Err(other) => panic!("vfail: {:?}", other),
    }
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
fn make_and_lock_weak_ref_from_borrow_local_then_destroy_own_with_interface() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/weakFromLocalCRefInterface.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches: Vec<()> = collect_where_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::SoftLoad(SoftLoadTE { target_ownership: OwnershipT::Weak, .. }) => Some(())
        );
        assert!(matches.len() >= 1);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
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
fn make_and_lock_weak_ref_from_borrow_then_destroy_own_with_interface() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/weakFromCRefInterface.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches: Vec<()> = collect_where_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::SoftLoad(SoftLoadTE { target_ownership: OwnershipT::Weak, .. }) => Some(())
        );
        assert!(matches.len() >= 1);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
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
fn call_weak_self_method_after_drop() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/callWeakSelfMethodAfterDrop.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches: Vec<()> = collect_where_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::SoftLoad(SoftLoadTE { target_ownership: OwnershipT::Weak, .. }) => Some(())
        );
        assert!(matches.len() >= 1);
    }
    let _hamuts = compile.get_hamuts();

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
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
fn call_weak_self_method_while_alive() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/weaks/callWeakSelfMethodWhileLive.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches: Vec<()> = collect_where_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::SoftLoad(SoftLoadTE { target_ownership: OwnershipT::Weak, .. }) => Some(())
        );
        assert!(matches.len() >= 1);
    }
    let _hamuts = compile.get_hamuts();

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
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
fn weak_yonder_member() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nweakable struct Base {\n  hp int;\n}\nstruct Spaceship {\n  origin &&Base;\n}\nexported func main() int {\n  base = Base(73);\n  ship = Spaceship(&&base);\n\n  (base).drop(); // Destroys base.\n\n  maybeOrigin = lock(ship.origin);\n  if (not maybeOrigin.isEmpty()) {\n    o = maybeOrigin.get();\n    return o.hp;\n  } else {\n    return 42;\n  }\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let _main = coutputs.lookup_function_by_str("main");
    }
    let _hamuts = compile.get_hamuts();

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
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
