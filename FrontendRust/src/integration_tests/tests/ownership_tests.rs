// mig: struct OwnershipTests
pub struct OwnershipTests;

/*
package dev.vale

import dev.vale.postparsing.patterns.AtomSP
import dev.vale.typing.ast.{FunctionCallTE, LetAndLendTE, LetNormalTE, UnletTE}
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.names.{IdT, FunctionNameT, FunctionTemplateNameT, StructNameT, StructTemplateNameT, TypingPassTemporaryVarNameT}
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.templata.functionNameT
import dev.vale.typing.types._
import org.scalatest._
import dev.vale.von.VonInt

class OwnershipTests extends FunSuite with Matchers {
*/
// mig: fn borrowing_a_temporary_mutable_makes_a_local_var
#[test]
fn borrowing_a_temporary_mutable_makes_a_local_var() {
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
        "\nstruct Muta { hp int; }\nexported func main() int {\n  return (&Muta(9)).hp;\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::LetAndLend(let_te) if matches!(
                let_te.variable,
                crate::typing::env::function_environment_t::ILocalVariableT::Reference(crate::typing::env::function_environment_t::ReferenceLocalVariableT {
                    name: crate::typing::names::names::IVarNameT::TypingPassTemporaryVar(_),
                    variability: crate::typing::types::types::VariabilityT::Final,
                    ..
                })
            ) => {
                match let_te.expr.result().coord {
                    crate::typing::types::types::CoordT {
                        ownership: crate::typing::types::types::OwnershipT::Own,
                        kind: crate::typing::types::types::KindT::Struct(crate::typing::types::types::StructTT { id, .. }),
                        ..
                    } if crate::typing::templata::templata_utils::unapply_simple_name(&id) == Some("Muta".to_string()) => {}
                    other => panic!("unexpected coord: {:?}", other),
                }
                assert_eq!(let_te.target_ownership, crate::typing::types::types::OwnershipT::Borrow);
                assert_eq!(let_te.result().coord.ownership, crate::typing::types::types::OwnershipT::Borrow);
                Some(())
            }
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("Expected VonInt(9), got {:?}", other),
    }
}

/*
  test("Borrowing a temporary mutable makes a local var") {
    val compile = RunCompilation.test(
      """
        |struct Muta { hp int; }
        |exported func main() int {
        |  return (&Muta(9)).hp;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, {
      case letTE @ LetAndLendTE(ReferenceLocalVariableT(TypingPassTemporaryVarNameT(_),FinalT,_),refExpr,targetOwnership) => {
        refExpr.result.coord match {
          case CoordT(OwnT, _, StructTT(simpleNameT("Muta"))) =>
        }
        targetOwnership shouldEqual BorrowT
        letTE.result.coord.ownership shouldEqual BorrowT
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn owning_ref_method_call
#[test]
fn owning_ref_method_call() {
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
        "\nstruct Muta { hp int; }\nfunc take(m Muta) int {\n  return m.hp;\n}\nexported func main() int {\n  m = Muta(9);\n  return (m).hp;\n}\n",
    );
    {
        let _main = compile.expect_compiler_outputs().lookup_function_by_str("main");
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("Expected VonInt(9), got {:?}", other),
    }
}

/*
  test("Owning ref method call") {
    val compile = RunCompilation.test(
      """
        |struct Muta { hp int; }
        |func take(m Muta) int {
        |  return m.hp;
        |}
        |exported func main() int {
        |  m = Muta(9);
        |  return (m).hp;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn derive_drop
#[test]
fn derive_drop() {
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
        "\nimport printutils.*;\n\nstruct Muta { }\n\nexported func main() {\n  Muta();\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE { callable, .. })
                if crate::typing::templata::templata_utils::unapply_function_name_prototype(callable) == Some("drop".to_string())
                => Some(())
        );
        let matches = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(_) => Some(())
        );
        assert_eq!(matches.len(), 2);
    }
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}

/*
  test("Derive drop") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |struct Muta { }
        |
        |exported func main() {
        |  Muta();
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.all(main, { case FunctionCallTE(_, _, _) => }).size shouldEqual 2

    compile.evalForKind(Vector())
  }
*/
// mig: fn custom_drop_result_is_an_owning_ref_calls_destructor
#[test]
fn custom_drop_result_is_an_owning_ref_calls_destructor() {
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
        "\nimport printutils.*;\n\n#!DeriveStructDrop\nstruct Muta { }\n\nfunc drop(m ^Muta) void {\n  println(\"Destroying!\");\n  Muta[ ] = m;\n}\n\nexported func main() {\n  Muta();\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE { callable, .. })
                if crate::typing::templata::templata_utils::unapply_function_name_prototype(callable) == Some("drop".to_string())
                => Some(())
        );
        let matches = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(_) => Some(())
        );
        assert_eq!(matches.len(), 2);
    }
    assert_eq!(compile.eval_for_stdout(Vec::new()).unwrap(), "Destroying!\n");
}

/*
  test("Custom drop result is an owning ref, calls destructor") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { }
        |
        |func drop(m ^Muta) void {
        |  println("Destroying!");
        |  Muta[ ] = m;
        |}
        |
        |exported func main() {
        |  Muta();
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.all(main, { case FunctionCallTE(_, _, _) => }).size shouldEqual 2

    compile.evalForStdout(Vector()) shouldEqual "Destroying!\n"
  }
*/
// mig: fn saves_return_value_then_destroys_temporary
#[test]
fn saves_return_value_then_destroys_temporary() {
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
        "\nimport printutils.*;\n\n#!DeriveStructDrop\nstruct Muta { hp int; }\n\nfunc drop(m ^Muta) {\n  println(\"Destroying!\");\n  Muta[hp] = m;\n}\n\nexported func main() int {\n  return (Muta(10)).hp;\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE { callable, .. })
                if crate::typing::templata::templata_utils::unapply_function_name_prototype(callable) == Some("drop".to_string())
                => Some(())
        );
    }
    match compile.eval_for_kind_and_stdout(Vec::new()).unwrap() {
        (crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 10 }), ref s) if s == "Destroying!\n" => {}
        other => panic!("expected (VonInt(10), \"Destroying!\\n\"), got {:?}", other),
    }
}

/*
  test("Saves return value then destroys temporary") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { hp int; }
        |
        |func drop(m ^Muta) {
        |  println("Destroying!");
        |  Muta[hp] = m;
        |}
        |
        |exported func main() int {
        |  return (Muta(10)).hp;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })

    compile.evalForKindAndStdout(Vector()) match { case (VonInt(10), "Destroying!\n") => }
  }
*/
// mig: fn calls_destructor_on_local_var
#[test]
fn calls_destructor_on_local_var() {
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
        "\nimport printutils.*;\n\n#!DeriveStructDrop\nstruct Muta { }\n\nfunc drop(m ^Muta) {\n  println(\"Destroying!\");\n  Muta[ ] = m;\n}\n\nexported func main() {\n  a = Muta();\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE { callable, .. })
                if crate::typing::templata::templata_utils::unapply_function_name_prototype(callable) == Some("drop".to_string())
                => Some(())
        );
        let matches = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(_) => Some(())
        );
        assert_eq!(matches.len(), 2);
    }
    assert_eq!(compile.eval_for_stdout(Vec::new()).unwrap(), "Destroying!\n");
}

/*
  test("Calls destructor on local var") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { }
        |
        |func drop(m ^Muta) {
        |  println("Destroying!");
        |  Muta[ ] = m;
        |}
        |
        |exported func main() {
        |  a = Muta();
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.all(main, { case FunctionCallTE(_, _, _) => }).size shouldEqual 2

    compile.evalForStdout(Vector()) shouldEqual "Destroying!\n"
  }
*/
// mig: fn calls_destructor_on_local_var_unless_moved
#[test]
fn calls_destructor_on_local_var_unless_moved() {
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
    // Should call the destructor in moo, but not in main
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nimport printutils.*;\n\n#!DeriveStructDrop\nstruct Muta { }\n\nfunc drop(m ^Muta) {\n  println(\"Destroying!\");\n  Muta[ ] = m;\n}\n\nfunc moo(m ^Muta) {\n}\n\nexported func main() {\n  a = Muta();\n  moo(a);\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();

        // Destructor should only be calling println, NOT the destructor (itself)
        let destructor = coutputs.functions.iter().find(|func| {
            matches!(func.header.id.local_name,
                crate::typing::names::names::INameT::Function(crate::typing::names::names::FunctionNameT {
                    template: crate::typing::names::names::FunctionTemplateNameT { human_name: crate::interner::StrI("drop"), .. },
                    parameters: [crate::typing::types::types::CoordT {
                        ownership: crate::typing::types::types::OwnershipT::Own,
                        kind: crate::typing::types::types::KindT::Struct(crate::typing::types::types::StructTT {
                            id: crate::typing::names::names::IdT {
                                local_name: crate::typing::names::names::INameT::Struct(crate::typing::names::names::StructNameT {
                                    template: crate::typing::names::names::IStructTemplateNameT::StructTemplate(crate::typing::names::names::StructTemplateNameT {
                                        human_name: crate::interner::StrI("Muta"), ..
                                    }),
                                    ..
                                }),
                                ..
                            },
                            ..
                        }),
                        ..
                    }],
                    ..
                })
            )
        }).unwrap();
        // The only function lookup should be println
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(destructor),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE { callable, .. })
                if crate::typing::templata::templata_utils::unapply_function_name_prototype(callable) == Some("println".to_string())
                => Some(())
        );
        // Only one call (the above println)
        let destructor_calls = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(destructor),
            crate::typing::test::traverse::NodeRefT::FunctionCall(_) => Some(())
        );
        assert_eq!(destructor_calls.len(), 1);

        // moo should be calling the destructor
        let moo = coutputs.lookup_function_by_str("moo");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(moo),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE { callable, .. })
                if crate::typing::templata::templata_utils::unapply_function_name_prototype(callable) == Some("drop".to_string())
                => Some(())
        );
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(moo),
            crate::typing::test::traverse::NodeRefT::FunctionCall(_) => Some(())
        );

        // main should not be calling the destructor
        let main = coutputs.lookup_function_by_str("main");
        let main_drops = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE { callable, .. })
                if crate::typing::templata::templata_utils::unapply_function_name_prototype(callable) == Some("drop".to_string())
                => Some(true)
        );
        assert_eq!(main_drops.len(), 0);
    }
    assert_eq!(compile.eval_for_stdout(Vec::new()).unwrap(), "Destroying!\n");
}

/*
  test("Calls destructor on local var unless moved") {
    // Should call the destructor in moo, but not in main
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { }
        |
        |func drop(m ^Muta) {
        |  println("Destroying!");
        |  Muta[ ] = m;
        |}
        |
        |func moo(m ^Muta) {
        |}
        |
        |exported func main() {
        |  a = Muta();
        |  moo(a);
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()

    // Destructor should only be calling println, NOT the destructor (itself)
    val destructor =
      vassertOne(
        coutputs.functions.find(func => {
          func.header.id.localName match {
            case FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, Vector(CoordT(OwnT, _, StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("Muta")), _)))))) => true
            case _ => false
          }
        }))
    // The only function lookup should be println
    Collector.only(destructor, { case FunctionCallTE(functionNameT("println"), _, _) => })
    // Only one call (the above println)
    Collector.all(destructor, { case FunctionCallTE(_, _, _) => }).size shouldEqual 1

    // moo should be calling the destructor
    val moo = coutputs.lookupFunction("moo")
    Collector.only(moo, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.only(moo, { case FunctionCallTE(_, _, _) => })

    // main should not be calling the destructor
    val main = coutputs.lookupFunction("main")
    Collector.all(main, { case FunctionCallTE(functionNameT("drop"), _, _) => true }).size shouldEqual 0

    compile.evalForStdout(Vector()) shouldEqual "Destroying!\n"
  }
*/
// mig: fn saves_return_value_then_destroys_local_var
#[test]
fn saves_return_value_then_destroys_local_var() {
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
        "\nimport printutils.*;\n\n#!DeriveStructDrop\nstruct Muta { hp int; }\n\nfunc drop(m ^Muta) {\n  println(\"Destroying!\");\n  Muta[hp] = m;\n}\n\nexported func main() int {\n  a = Muta(10);\n  return a.hp;\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE { callable, .. })
                if crate::typing::templata::templata_utils::unapply_function_name_prototype(callable) == Some("drop".to_string())
                => Some(())
        );
        let matches = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(_) => Some(())
        );
        assert_eq!(matches.len(), 2);
    }
    match compile.eval_for_kind_and_stdout(Vec::new()).unwrap() {
        (crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 10 }), ref s) if s == "Destroying!\n" => {}
        other => panic!("expected (VonInt(10), \"Destroying!\\n\"), got {:?}", other),
    }
}

/*
  test("Saves return value then destroys local var") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { hp int; }
        |
        |func drop(m ^Muta) {
        |  println("Destroying!");
        |  Muta[hp] = m;
        |}
        |
        |exported func main() int {
        |  a = Muta(10);
        |  return a.hp;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.all(main, { case FunctionCallTE(_, _, _) => }).size shouldEqual 2

    compile.evalForKindAndStdout(Vector()) match { case (VonInt(10), "Destroying!\n") => }
  }
*/
// mig: fn gets_from_temporary_struct_a_members_member
#[test]
fn gets_from_temporary_struct_a_members_member() {
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
        "\nstruct Wand {\n  charges int;\n}\nstruct Wizard {\n  wand ^Wand;\n}\nexported func main() int {\n  return Wizard(Wand(10)).wand.charges;\n}\n      ",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 10 }) => {}
        other => panic!("Expected VonInt(10), got {:?}", other),
    }
}

/*
  test("Gets from temporary struct a member's member") {
    val compile = RunCompilation.test(
      """
        |struct Wand {
        |  charges int;
        |}
        |struct Wizard {
        |  wand ^Wand;
        |}
        |exported func main() int {
        |  return Wizard(Wand(10)).wand.charges;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(10) => }
  }

  // test that when we create a block closure, we hoist to the beginning its constructor,
  // and hoist to the end its destructor.

  // test that when we borrow an owning, we hoist its destructor to the end.
*/
// mig: fn unstackifies_local_vars
#[test]
fn unstackifies_local_vars() {
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
        "\nexported func main() int {\n  i = 0;\n  return i;\n}\n",
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    let num_variables: Vec<()> = crate::collect_where_tnode!(
        crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
        crate::typing::test::traverse::NodeRefT::LetAndLend(_) | crate::typing::test::traverse::NodeRefT::LetNormal(_) => Some(())
    );
    let unlets: Vec<()> = crate::collect_where_tnode!(
        crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
        crate::typing::test::traverse::NodeRefT::Unlet(_) => Some(())
    );
    assert_eq!(unlets.len(), num_variables.len());
}

/*
  test("Unstackifies local vars") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  i = 0;
        |  return i;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")

    val numVariables =
      Collector.all(main, {
        case LetAndLendTE(_, _, _) =>
        case LetNormalTE(_, _) =>
      }).size
    Collector.all(main, { case UnletTE(_) => }).size shouldEqual numVariables
  }
*/
// mig: fn basic_builder_pattern
#[test]
fn basic_builder_pattern() {
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
        "\nstruct Ship { hp! int; fuel! int; }\nfunc setHp(ship Ship, hp int) Ship {\n  set ship.hp = hp;\n  return ship;\n}\nfunc setFuel(ship Ship, fuel int) Ship {\n  set ship.fuel = fuel;\n  return ship;\n}\nexported func main() int {\n  ship = Ship(0, 0).setHp(42).setFuel(43);\n  return ship.hp;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
}

/*
  test("Basic builder pattern") {
    val compile = RunCompilation.test(
      """
        |struct Ship { hp! int; fuel! int; }
        |func setHp(ship Ship, hp int) Ship {
        |  set ship.hp = hp;
        |  return ship;
        |}
        |func setFuel(ship Ship, fuel int) Ship {
        |  set ship.fuel = fuel;
        |  return ship;
        |}
        |exported func main() int {
        |  ship = Ship(0, 0).setHp(42).setFuel(43);
        |  return ship.hp;
        |}
        |""".stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn member_access_on_returned_owning_ref
#[test]
fn member_access_on_returned_owning_ref() {
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
        "\nstruct Ship { hp int; }\nexported func main() int {\n  return Ship(42).hp;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
}

/*
  test("Member access on returned owning ref") {
    val compile = RunCompilation.test(
      """
        |struct Ship { hp int; }
        |exported func main() int {
        |  return Ship(42).hp;
        |}
        |""".stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }

}

*/
