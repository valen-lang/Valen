// mig: struct ClosureTests
pub struct ClosureTests;
/*
package dev.vale

import dev.vale.postparsing._
import dev.vale.typing.ast.{AddressMemberLookupTE, ConstructTE, FunctionCallTE, LetNormalTE, LocalLookupTE, MutateTE, ReferenceMemberLookupTE}
import dev.vale.typing.env.{AddressibleLocalVariableT, ReferenceLocalVariableT}
import dev.vale.typing.expression.LocalHelper
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.types._
import org.scalatest._
import dev.vale.typing.templata.MutabilityTemplataT
import dev.vale.von.VonInt

class ClosureTests extends FunSuite with Matchers {
*/
// mig: fn addressibility
#[test]
pub fn addressibility() {
    let scout_bump = bumpalo::Bump::new();
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let calc = |self_borrowed, self_moved, self_mutated, child_borrowed, child_moved, child_mutated| {
        let local_s = crate::postparsing::expressions::LocalS {
            var_name: crate::postparsing::names::IVarNameS::CodeVarName(scout_arena.intern_str("x")),
            self_borrowed,
            self_moved,
            self_mutated,
            child_borrowed,
            child_moved,
            child_mutated,
        };
        let local_a: &crate::postparsing::expressions::LocalS = scout_arena.alloc(local_s);
        let addressible_if_mutable = crate::typing::compiler::Compiler::determine_if_local_is_addressible(
            crate::typing::templata::templata::ITemplataT::Mutability(crate::typing::templata::templata::MutabilityTemplataT { mutability: crate::typing::types::types::MutabilityT::Mutable }),
            local_a);
        let addressible_if_immutable = crate::typing::compiler::Compiler::determine_if_local_is_addressible(
            crate::typing::templata::templata::ITemplataT::Mutability(crate::typing::templata::templata::MutabilityTemplataT { mutability: crate::typing::types::types::MutabilityT::Immutable }),
            local_a);
        (addressible_if_mutable, addressible_if_immutable)
    };
    use crate::postparsing::expressions::IVariableUseCertainty::{NotUsed, Used};
    // If we don't do anything with the variable, it can be just a reference.
    assert_eq!(calc(NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed), (false, false));
    // If we or our children only ever read, it can be just a reference.
    assert_eq!(calc(Used, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed), (false, false));
    assert_eq!(calc(NotUsed, NotUsed, NotUsed, Used, NotUsed, NotUsed), (false, false));
    // If only we mutate it, it can be just a reference.
    assert_eq!(calc(NotUsed, NotUsed, Used, NotUsed, NotUsed, NotUsed), (false, false));
    // Even if we're certain it's moved, it must be addressible.
    // Imagine:
    // exported func main() int {
    //   m = Marine();
    //   if (something) {
    //     something.consume(m);
    //   } else {
    //     otherthing.consume(m);
    //   }
    // }
    // (or, we can change it so we move it into the closure struct, but that
    // seems weird, i like thinking that closures only ever have borrows or
    // addressibles)
    // However, this doesnt apply to immutable, since move = copy.
    assert_eq!(calc(NotUsed, NotUsed, NotUsed, NotUsed, Used, NotUsed), (true, false));
    // If we're certain children mutate it, it also has to be addressible.
    assert_eq!(calc(NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, Used), (true, true));
}
/*
  test("Addressibility") {
    val interner = new Interner()

    def calc(
        selfBorrowed: IVariableUseCertainty,
        selfMoved: IVariableUseCertainty,
        selfMutated: IVariableUseCertainty,
        childBorrowed: IVariableUseCertainty,
        childMoved: IVariableUseCertainty,
        childMutated: IVariableUseCertainty) = {
      val addressibleIfMutable =
        LocalHelper.determineIfLocalIsAddressible(
          MutabilityTemplataT(MutableT),
          LocalS(
            CodeVarNameS(interner.intern(StrI("x"))), selfBorrowed, selfMoved, selfMutated, childBorrowed, childMoved, childMutated))
      val addressibleIfImmutable =
        LocalHelper.determineIfLocalIsAddressible(
          MutabilityTemplataT(ImmutableT),
          LocalS(
            CodeVarNameS(interner.intern(StrI("x"))), selfBorrowed, selfMoved, selfMutated, childBorrowed, childMoved, childMutated))
      (addressibleIfMutable, addressibleIfImmutable)
    }

    // If we don't do anything with the variable, it can be just a reference.
    calc(NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) shouldEqual (false, false)

    // If we or our children only ever read, it can be just a reference.
    calc(Used, NotUsed, NotUsed,      NotUsed, NotUsed, NotUsed) shouldEqual (false, false)
    calc(NotUsed, NotUsed, NotUsed,   Used, NotUsed, NotUsed) shouldEqual (false, false)

    // If only we mutate it, it can be just a reference.
    calc(NotUsed, NotUsed, Used, NotUsed, NotUsed, NotUsed) shouldEqual (false, false)

    // Even if we're certain it's moved, it must be addressible.
    // Imagine:
    // exported func main() int {
    //   m = Marine();
    //   if (something) {
    //     something.consume(m);
    //   } else {
    //     otherthing.consume(m);
    //   }
    // }
    // (or, we can change it so we move it into the closure struct, but that
    // seems weird, i like thinking that closures only ever have borrows or
    // addressibles)
    // However, this doesnt apply to immutable, since move = copy.
    calc(NotUsed, NotUsed, NotUsed, NotUsed, Used, NotUsed) shouldEqual (true, false)

    // If we're certain children mutate it, it also has to be addressible.
    calc(NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, Used) shouldEqual (true, true)
  }
*/
// mig: fn captured_own_is_borrow
#[test]
pub fn captured_own_is_borrow() {
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
    // Here, the scout determined that the closure is only ever borrowing
    // it (during the dereference to get its member) so typingpass doesn't put
    // an address into the closure, it instead puts a reference. Specifically,
    // a borrow reference (because why would we want to move this into the
    // closure struct?).
    // This means the closure struct contains a borrow reference. This means
    // the environment in the closure has to match this; the environment has
    // to have a borrow reference instead of an owning reference.
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nstruct Marine {\n  hp int;\n}\nexported func main() int {\n  m = Marine(9);\n  return { m.hp }();\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}
/*
  test("Captured own is borrow") {
    // Here, the scout determined that the closure is only ever borrowing
    // it (during the dereference to get its member) so typingpass doesn't put
    // an address into the closure, it instead puts a reference. Specifically,
    // a borrow reference (because why would we want to move this into the
    // closure struct?).
    // This means the closure struct contains a borrow reference. This means
    // the environment in the closure has to match this; the environment has
    // to have a borrow reference instead of an owning reference.

    val compile = RunCompilation.test(
      """
        |struct Marine {
        |  hp int;
        |}
        |exported func main() int {
        |  m = Marine(9);
        |  return { m.hp }();
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn test_closure_s_local_variables
#[test]
fn test_closure_s_local_variables() {
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
        "exported func main() int { x = 4; return {x}(); }",
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_lambda_in("main");
    crate::collect_only_tnode!(
        crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
        crate::typing::test::traverse::NodeRefT::LetNormal(crate::typing::ast::expressions::LetNormalTE {
            variable: crate::typing::env::function_environment_t::ILocalVariableT::Reference(crate::typing::env::function_environment_t::ReferenceLocalVariableT {
                name: crate::typing::names::names::IVarNameT::ClosureParam(_),
                variability: crate::typing::types::types::VariabilityT::Final,
                coord: crate::typing::types::types::CoordT {
                    ownership: crate::typing::types::types::OwnershipT::Share,
                    kind: crate::typing::types::types::KindT::Struct(crate::typing::types::types::StructTT {
                        id: crate::typing::names::names::IdT {
                            init_steps: &[crate::typing::names::names::INameT::Function(crate::typing::names::names::FunctionNameT {
                                template: crate::typing::names::names::FunctionTemplateNameT {
                                    human_name: crate::interner::StrI("main"), ..
                                },
                                template_args: &[],
                                parameters: &[],
                                ..
                            })],
                            local_name: crate::typing::names::names::INameT::LambdaCitizen(crate::typing::names::names::LambdaCitizenNameT {
                                template: crate::typing::names::names::LambdaCitizenTemplateNameT { .. },
                            }),
                            ..
                        },
                        ..
                    }),
                    ..
                },
            }),
            ..
        }) => Some(())
    );
    crate::collect_only_tnode!(
        crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
        crate::typing::test::traverse::NodeRefT::LetNormal(crate::typing::ast::expressions::LetNormalTE {
            variable: crate::typing::env::function_environment_t::ILocalVariableT::Reference(crate::typing::env::function_environment_t::ReferenceLocalVariableT {
                name: crate::typing::names::names::IVarNameT::TypingPassBlockResultVar(_),
                variability: crate::typing::types::types::VariabilityT::Final,
                coord: crate::typing::types::types::CoordT {
                    ownership: crate::typing::types::types::OwnershipT::Share,
                    kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT { bits: 32 }),
                    ..
                },
            }),
            ..
        }) => Some(())
    );
}
/*
  test("Test closure's local variables") {
    val compile = RunCompilation.test("exported func main() int { x = 4; return {x}(); }")
    val coutputs = compile.expectCompilerOutputs()

    val main = coutputs.lookupLambdaIn("main")
    Collector.only(main, {
      case LetNormalTE(
        ReferenceLocalVariableT(
          ClosureParamNameT(_),
          FinalT,
          CoordT(ShareT, _,StructTT(IdT(_,Vector(FunctionNameT(FunctionTemplateNameT(StrI("main"),_),Vector(),Vector())),LambdaCitizenNameT(LambdaCitizenTemplateNameT(_)))))),
        _) =>
////
//      case LetNormalTE(
//        ReferenceLocalVariableT(
//          FullNameT(_, Vector(FunctionNameT(FunctionTemplateNameT(StrI("main"), _), _, _), LambdaCitizenNameT(_), FunctionNameT(FunctionTemplateNameT(StrI("__call"), _), _, _)), ClosureParamNameT()),
//          FinalT,
//          CoordT(ShareT, StructTT(FullNameT(_, Vector(FunctionNameT(FunctionTemplateNameT(StrI("main"), _), Vector(), Vector())), LambdaCitizenNameT(_))))),
//        _) =>
    })
    Collector.only(main, {
      case LetNormalTE(
        ReferenceLocalVariableT(
          TypingPassBlockResultVarNameT(_),
          FinalT,
          CoordT(ShareT,_,IntT(32))),
        _) =>
    })
  }
*/
// mig: fn test_returning_a_nonmutable_closured_variable_from_the_closure
#[test]
fn test_returning_a_nonmutable_closured_variable_from_the_closure() {
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
        "exported func main() int { x = 4; return {x}(); }",
    );
    {
        let interner = compile.interner;
        let coutputs = compile.expect_compiler_outputs();

        // The struct should have an int x in it which is a reference type.
        // It's a reference because we know for sure that it's moved from our child,
        // which means we don't need to check afterwards, which means it doesn't need
        // to be boxed/addressible.
        let closured_vars_struct_tt =
            coutputs.lookup_lambda_in("main").header.params.first().unwrap().tyype.kind.expect_struct();
        let closured_vars_struct_def =
            coutputs.structs.iter().find(|struct_def| {
                *crate::typing::compiler::Compiler::get_template(interner, struct_def.instantiated_citizen.id) ==
                    *crate::typing::compiler::Compiler::get_template(interner, closured_vars_struct_tt.id)
            }).expect("closured_vars_struct_def not found");

        let expected_members = vec![
            crate::typing::ast::citizens::IStructMemberT::Normal(crate::typing::ast::citizens::NormalStructMemberT {
                name: crate::typing::names::names::IVarNameT::CodeVar(interner.intern_code_var_name(crate::typing::names::names::CodeVarNameT { name: scout_arena.intern_str("x"), _phantom: std::marker::PhantomData })),
                variability: crate::typing::types::types::VariabilityT::Final,
                tyype: crate::typing::ast::citizens::IMemberTypeT::Reference(crate::typing::ast::citizens::ReferenceMemberTypeT {
                    reference: crate::typing::types::types::CoordT {
                        ownership: crate::typing::types::types::OwnershipT::Share,
                        region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
                        kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT { bits: 32 }),
                    },
                }),
            }),
        ];
        assert_eq!(closured_vars_struct_def.members, expected_members.as_slice());

        let lambda = coutputs.lookup_lambda_in("main");
        // Make sure we're doing a referencememberlookup, since it's a reference member
        // in the closure struct.
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(lambda),
            crate::typing::test::traverse::NodeRefT::ReferenceMemberLookup(crate::typing::ast::expressions::ReferenceMemberLookupTE {
                member_name: crate::typing::names::names::IVarNameT::CodeVar(crate::typing::names::names::CodeVarNameT { name: crate::interner::StrI("x"), .. }),
                ..
            }) => Some(())
        );

        // Make sure there's a function that takes in the closured vars struct, and returns an int
        let function_calls = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(coutputs.lookup_function_by_str("main")),
            crate::typing::test::traverse::NodeRefT::FunctionCall(crate::typing::ast::expressions::FunctionCallTE {
                callable: p @ crate::typing::ast::ast::PrototypeT { id: crate::typing::names::names::IdT { local_name: crate::typing::names::names::INameT::LambdaCallFunction(_), .. }, .. },
                ..
            }) => Some(*p)
        );
        assert_eq!(function_calls.len(), 1);
        let prototype: crate::typing::ast::ast::PrototypeT<'_, '_> = function_calls[0];
        let lambda_call_name: &crate::typing::names::names::LambdaCallFunctionNameT = match prototype.id.local_name {
            crate::typing::names::names::INameT::LambdaCallFunction(n) => n,
            _ => panic!("expected LambdaCallFunction local_name"),
        };
        let params = lambda_call_name.parameters;
        let return_type = prototype.return_type;
        match params.first().unwrap() {
            crate::typing::types::types::CoordT {
                ownership: crate::typing::types::types::OwnershipT::Share,
                kind: crate::typing::types::types::KindT::Struct(crate::typing::types::types::StructTT {
                    id: crate::typing::names::names::IdT {
                        init_steps: &[crate::typing::names::names::INameT::Function(crate::typing::names::names::FunctionNameT {
                            template: crate::typing::names::names::FunctionTemplateNameT { human_name: crate::interner::StrI("main"), .. },
                            template_args: &[],
                            parameters: &[],
                            ..
                        })],
                        local_name: crate::typing::names::names::INameT::LambdaCitizen(_),
                        ..
                    },
                    ..
                }),
                ..
            } => {}
            other => panic!("expected lambda struct param, got {:?}", other),
        }
        assert_eq!(return_type, crate::typing::types::types::CoordT {
            ownership: crate::typing::types::types::OwnershipT::Share,
            region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
            kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT { bits: 32 }),
        });

        // Make sure we make it with a function pointer and a constructed vars struct
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::Construct(crate::typing::ast::expressions::ConstructTE {
                struct_tt: crate::typing::types::types::StructTT {
                    id: crate::typing::names::names::IdT {
                        init_steps: &[crate::typing::names::names::INameT::Function(crate::typing::names::names::FunctionNameT {
                            template: crate::typing::names::names::FunctionTemplateNameT { human_name: crate::interner::StrI("main"), .. },
                            template_args: &[],
                            parameters: &[],
                            ..
                        })],
                        local_name: crate::typing::names::names::INameT::LambdaCitizen(_),
                        ..
                    },
                    ..
                },
                ..
            }) => Some(())
        );

        // Make sure we call the function somewhere
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::FunctionCall(_) => Some(())
        );

        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(lambda),
            crate::typing::test::traverse::NodeRefT::LocalLookup(crate::typing::ast::expressions::LocalLookupTE {
                local_variable: crate::typing::env::function_environment_t::ILocalVariableT::Reference(crate::typing::env::function_environment_t::ReferenceLocalVariableT {
                    name: crate::typing::names::names::IVarNameT::ClosureParam(_),
                    variability: crate::typing::types::types::VariabilityT::Final,
                    ..
                }),
                ..
            }) => Some(())
        );
    }

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 4 }) => {}
        other => panic!("expected VonInt(4), got {:?}", other),
    }
}
/*
  test("Test returning a nonmutable closured variable from the closure") {
    val compile = RunCompilation.test("exported func main() int { x = 4; return {x}(); }")
    val coutputs = compile.expectCompilerOutputs()
    val interner = compile.interner

    // The struct should have an int x in it which is a reference type.
    // It's a reference because we know for sure that it's moved from our child,
    // which means we don't need to check afterwards, which means it doesn't need
    // to be boxed/addressible.
    val closuredVarsStructTT =
      coutputs.lookupLambdaIn("main").header.params.head.tyype.kind.expectStruct()
    val closuredVarsStructDef =
      vassertSome(
        coutputs.structs.find(structDef => {
          TemplataCompiler.getTemplate(structDef.instantiatedCitizen.id) ==
            TemplataCompiler.getTemplate(closuredVarsStructTT.id)
        }))

    val expectedMembers =
      Vector(NormalStructMemberT(interner.intern(CodeVarNameT(interner.intern(StrI("x")))), FinalT, ReferenceMemberTypeT(CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))));
    vassert(closuredVarsStructDef.members == expectedMembers)

    val lambda = coutputs.lookupLambdaIn("main")
    // Make sure we're doing a referencememberlookup, since it's a reference member
    // in the closure struct.
    Collector.only(lambda, {
      case ReferenceMemberLookupTE(_,_, CodeVarNameT(StrI("x")), _, _) =>
    })

    // Make sure there's a function that takes in the closured vars struct, and returns an int
    val PrototypeT(IdT(_, _, LambdaCallFunctionNameT(_, _, params)), returnType) =
      vassertOne(
        Collector.all(
          coutputs.lookupFunction("main"),
          {
            case FunctionCallTE(p @ PrototypeT(IdT(_, _, LambdaCallFunctionNameT(_, _, _)), _), _, _) => p
          }))
    params.head match {
      case CoordT(ShareT, _, StructTT(IdT(_, Vector(FunctionNameT(FunctionTemplateNameT(StrI("main"), _),Vector(),Vector())),LambdaCitizenNameT(_)))) =>
    }
    returnType shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)

    // Make sure we make it with a function pointer and a constructed vars struct
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case ConstructTE(
        StructTT(IdT(_, Vector(FunctionNameT(FunctionTemplateNameT(StrI("main"), _),Vector(),Vector())),LambdaCitizenNameT(_))), _, _) =>
    })

    // Make sure we call the function somewhere
    Collector.onlyOf(main, classOf[FunctionCallTE])

    Collector.only(lambda, {
      case LocalLookupTE(_,ReferenceLocalVariableT(ClosureParamNameT(_),FinalT,_)) =>
    })

    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn mutates_from_inside_a_closure
#[test]
fn mutates_from_inside_a_closure() {
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
        "\nexported func main() int {\n  x = 4;\n  { set x = x + 1; }();\n  return x;\n}\n",
    );
    {
        let interner = compile.interner;
        let coutputs = compile.expect_compiler_outputs();

        // The struct should have an int x in it.
        let closure = coutputs.lookup_lambda_in("main");
        let closure_struct = closure.header.params.first().unwrap().tyype.kind.expect_struct();
        let closure_struct_def = coutputs.lookup_struct(closure_struct.id);
        let expected_members = vec![
            crate::typing::ast::citizens::IStructMemberT::Normal(crate::typing::ast::citizens::NormalStructMemberT {
                name: crate::typing::names::names::IVarNameT::CodeVar(interner.intern_code_var_name(crate::typing::names::names::CodeVarNameT { name: scout_arena.intern_str("x"), _phantom: std::marker::PhantomData })),
                variability: crate::typing::types::types::VariabilityT::Varying,
                tyype: crate::typing::ast::citizens::IMemberTypeT::Address(crate::typing::ast::citizens::AddressMemberTypeT {
                    reference: crate::typing::types::types::CoordT {
                        ownership: crate::typing::types::types::OwnershipT::Share,
                        region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
                        kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT { bits: 32 }),
                    },
                }),
            }),
        ];
        assert_eq!(closure_struct_def.members, expected_members.as_slice());

        let lambda = coutputs.lookup_lambda_in("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(lambda),
            crate::typing::test::traverse::NodeRefT::Mutate(crate::typing::ast::expressions::MutateTE {
                destination_expr: crate::typing::ast::expressions::AddressExpressionTE::AddressMemberLookup(crate::typing::ast::expressions::AddressMemberLookupTE {
                    member_name: crate::typing::names::names::IVarNameT::CodeVar(crate::typing::names::names::CodeVarNameT { name: crate::interner::StrI("x"), .. }),
                    result_type2: crate::typing::types::types::CoordT {
                        ownership: crate::typing::types::types::OwnershipT::Share,
                        kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT { bits: 32 }),
                        ..
                    },
                    ..
                }),
                ..
            }) => Some(())
        );

        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::LetNormal(crate::typing::ast::expressions::LetNormalTE {
                variable: crate::typing::env::function_environment_t::ILocalVariableT::Addressible(crate::typing::env::function_environment_t::AddressibleLocalVariableT {
                    variability: crate::typing::types::types::VariabilityT::Varying,
                    ..
                }),
                ..
            }) => Some(())
        );
    }

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Mutates from inside a closure") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  x = 4;
        |  { set x = x + 1; }();
        |  return x;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val interner = compile.interner

    // The struct should have an int x in it.
    val closure = coutputs.lookupLambdaIn("main")
    val closureStruct = closure.header.params.head.tyype.kind.expectStruct()
    val closureStructDef = coutputs.lookupStruct(closureStruct.id)
    val expectedMembers = Vector(NormalStructMemberT(interner.intern(CodeVarNameT(interner.intern(StrI("x")))), VaryingT, AddressMemberTypeT(CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))));
    closureStructDef.members shouldEqual expectedMembers

    val lambda = coutputs.lookupLambdaIn("main")
    Collector.only(lambda, {
      case MutateTE(
        AddressMemberLookupTE(_,_,CodeVarNameT(StrI("x")),CoordT(ShareT, _, IntT.i32), _),
        _) =>
    })

    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case LetNormalTE(AddressibleLocalVariableT(_, VaryingT, _), _) =>
    })

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn mutates_from_inside_a_closure_inside_a_closure
#[test]
pub fn mutates_from_inside_a_closure_inside_a_closure() {
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
        "exported func main() int { x = 4; { { set x = x + 1; }(); }(); return x; }",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Mutates from inside a closure inside a closure") {
    val compile = RunCompilation.test("exported func main() int { x = 4; { { set x = x + 1; }(); }(); return x; }")

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn read_from_inside_a_closure_inside_a_closure
#[test]
fn read_from_inside_a_closure_inside_a_closure() {
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
        "\nexported func main() int {\n  x = 42;\n  return { { x }() }();\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Read from inside a closure inside a closure") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  x = 42;
        |  return { { x }() }();
        |}
        |""".stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn mutable_lambda
#[test]
pub fn mutable_lambda() {
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
    let source = crate::tests::tests::load_expected("programs/lambdas/lambdamut.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let closure_structs: Vec<_> = coutputs.structs.iter().filter(|s| matches!(
            s.instantiated_citizen.id.local_name,
            crate::typing::names::names::INameT::LambdaCitizen(crate::typing::names::names::LambdaCitizenNameT {
                template: crate::typing::names::names::LambdaCitizenTemplateNameT {
                    code_location: crate::utils::range::CodeLocationS {
                        file: crate::utils::code_hierarchy::FileCoordinate {
                            package_coord: crate::utils::code_hierarchy::PackageCoordinate {
                                module: crate::interner::StrI("test"),
                                packages,
                            },
                            ..
                        },
                        ..
                    },
                    ..
                },
                ..
            }) if packages.is_empty()
        )).collect();
        assert_eq!(closure_structs.len(), 1);
        let closure_struct = closure_structs[0];
        assert!(matches!(closure_struct.mutability, crate::typing::templata::templata::ITemplataT::Mutability(crate::typing::templata::templata::MutabilityTemplataT { mutability: crate::typing::types::types::MutabilityT::Mutable })));
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Mutable lambda") {
    val compile =
      RunCompilation.test(
        Tests.loadExpected("programs/lambdas/lambdamut.vale"))

    val coutputs = compile.expectCompilerOutputs()
    val closureStruct =
      vassertOne(coutputs.structs.filter(struct => {
        struct.instantiatedCitizen.id.localName match {
          case LambdaCitizenNameT(LambdaCitizenTemplateNameT(CodeLocationS(FileCoordinate(PackageCoordinate(StrI("test"),Vector()), _), _))) => true
          case _ => false
        }
      }))
    closureStruct.mutability match {
      case MutabilityTemplataT(MutableT) =>
    }
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
}

*/
