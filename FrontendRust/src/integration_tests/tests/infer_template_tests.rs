/*
package dev.vale

import dev.vale.typing.ast.{FunctionCallTE, ParameterT, PrototypeT}
import dev.vale.typing.names.{CitizenNameT, CitizenTemplateNameT, CodeVarNameT, IdT, FunctionNameT, FunctionTemplateNameT, StructNameT, StructTemplateNameT}
import dev.vale.typing.templata.CoordTemplataT
import dev.vale.typing.types._
import dev.vale.typing.templata.simpleNameT
import dev.vale.typing.types.StructTT
import dev.vale.von.VonInt
import org.scalatest._
*/
// mig: struct InferTemplateTests
pub struct InferTemplateTests;
/*
class InferTemplateTests extends FunSuite with Matchers {
*/
// mig: fn test_inferring_a_borrowed_argument
#[test]
pub fn test_inferring_a_borrowed_argument() {
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
        "\nstruct Muta { hp int; }\nfunc moo<T>(m &T) &T { return m; }\nexported func main() int {\n  x = Muta(10);\n  return moo(&x).hp;\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let moo = coutputs.lookup_function_by_str("moo");
        match moo.header.params {
            [crate::typing::ast::ast::ParameterT {
                name: crate::typing::names::names::IVarNameT::CodeVar(crate::typing::names::names::CodeVarNameT { name: crate::interner::StrI("m"), .. }),
                tyype: crate::typing::types::types::CoordT { ownership: crate::typing::types::types::OwnershipT::Borrow, .. },
                ..
            }] => {}
            _ => panic!("moo.header.params didn't match expected pattern"),
        }
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE {
                callable: crate::typing::ast::ast::PrototypeT {
                    id: crate::typing::names::names::IdT {
                        local_name: crate::typing::names::names::INameT::Function(crate::typing::names::names::FunctionNameT {
                            template: crate::typing::names::names::FunctionTemplateNameT { human_name: crate::interner::StrI("moo"), .. },
                            template_args: &[crate::typing::templata::templata::ITemplataT::Coord(crate::typing::templata::templata::CoordTemplataT {
                                coord: crate::typing::types::types::CoordT {
                                    ownership: crate::typing::types::types::OwnershipT::Own,
                                    kind: crate::typing::types::types::KindT::Struct(crate::typing::types::types::StructTT {
                                        id: crate::typing::names::names::IdT {
                                            package_coord: x_package_coord,
                                            init_steps: &[],
                                            local_name: crate::typing::names::names::INameT::Struct(crate::typing::names::names::StructNameT {
                                                template: crate::typing::names::names::IStructTemplateNameT::StructTemplate(crate::typing::names::names::StructTemplateNameT { human_name: crate::interner::StrI("Muta"), .. }),
                                                template_args: &[],
                                                ..
                                            }),
                                            ..
                                        },
                                        ..
                                    }),
                                    ..
                                },
                                ..
                            })],
                            ..
                        }),
                        ..
                    },
                    ..
                },
                ..
            }) if x_package_coord.is_test() => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 10 }) => {}
        other => panic!("expected VonInt(10), got {:?}", other),
    }
}
/*
  test("Test inferring a borrowed argument") {
    val compile = RunCompilation.test(
      """
        |struct Muta { hp int; }
        |func moo<T>(m &T) &T { return m; }
        |exported func main() int {
        |  x = Muta(10);
        |  return moo(&x).hp;
        |}
      """.stripMargin)

    val moo = compile.expectCompilerOutputs().lookupFunction("moo")
    moo.header.params match {
      case Vector(ParameterT(CodeVarNameT(StrI("m")), _, _, CoordT(BorrowT,_, _))) =>
    }
    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, {
      case FunctionCallTE(PrototypeT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("moo"), _), templateArgs, _)), _), _, _) => {
        templateArgs match {
          case Vector(CoordTemplataT(CoordT(OwnT, _, StructTT(IdT(x, Vector(), StructNameT(StructTemplateNameT(StrI("Muta")), Vector())))))) => {
            vassert(x.isTest)
          }
        }
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(10) => }
  }
*/
// mig: fn test_inferring_a_borrowed_static_sized_array
#[test]
pub fn test_inferring_a_borrowed_static_sized_array() {
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
        "\nstruct Muta { hp int; }\nfunc moo<N Int>(m &[#N]Muta) int { return m[0].hp; }\nexported func main() int {\n  x = [#](Muta(10));\n  return moo(&x);\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 10 }) => {}
        other => panic!("expected VonInt(10), got {:?}", other),
    }
}
/*
  test("Test inferring a borrowed static sized array") {
    val compile = RunCompilation.test(
      """
        |struct Muta { hp int; }
        |func moo<N Int>(m &[#N]Muta) int { return m[0].hp; }
        |exported func main() int {
        |  x = [#](Muta(10));
        |  return moo(&x);
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(10) => }
  }
*/
// mig: fn test_inferring_an_owning_static_sized_array
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
pub fn test_inferring_an_owning_static_sized_array() {
  panic!("Unmigrated test: test_inferring_an_owning_static_sized_array");
}
/*
  test("Test inferring an owning static sized array") {
    val compile = RunCompilation.test(
      """
        |struct Muta { hp int; }
        |func moo<N Int>(m [#N]Muta) int { return m[0].hp; }
        |exported func main() int {
        |  x = [#](Muta(10));
        |  return moo(x);
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(10) => }
  }
}

*/
