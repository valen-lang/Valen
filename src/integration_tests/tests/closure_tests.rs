#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use crate::collect_only_tnode;
use crate::integration_tests::tests::run_compilation::test;
use crate::integration_tests::tests::run_compilation::test_without_borrow_check;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::typing::ast::citizens::StructMemberT;
use crate::typing::ast::expressions::LetNormalTE;
use crate::typing::ast::expressions::MemberLookupTE;
use crate::typing::env::function_environment_t::LocalVariable;
use crate::typing::names::names::FunctionNameT;
use crate::typing::names::names::FunctionTemplateNameT;
use crate::typing::names::names::IdT;
use crate::typing::names::names::INameT;
use crate::typing::names::names::IVarNameT;
use crate::typing::names::names::LambdaCitizenNameT;
use crate::typing::names::names::LambdaCitizenTemplateNameT;
use crate::typing::types::types::SharednessT;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::types::types::BorrowRefT;
use crate::typing::types::types::IntT;
use crate::typing::types::types::KindT;
use crate::typing::types::types::StructTT;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::load_expected;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::FileCoordinate;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::CodeLocationS;
use crate::testvm::von::IVonData;
use crate::testvm::von::VonInt;
use std::marker::PhantomData;
pub struct ClosureTests;

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
pub fn addressibility() {
    unimplemented!();
    /*
    let scout_bump = bumpalo::Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let calc = |self_borrowed, self_moved, self_mutated, child_borrowed, child_moved, child_mutated| {
        let local_s = LocalS {
            var_name: IVarDeclarationNameS::CodeVarName(CodeVarNameS {
                name: scout_arena.intern_str("x"),
                lid: LocationInDenizenBuilder::new(vec![]).consume_in_arena(&scout_arena),
            }),
            self_borrowed,
            self_moved,
            self_mutated,
            child_borrowed,
            child_moved,
            child_mutated,
        };
        let local_a: &LocalS = scout_arena.alloc(local_s);
        let addressible_if_mutable = Compiler::determine_if_local_is_addressible(
            SharednessT::Single,
            local_a);
        let addressible_if_immutable = Compiler::determine_if_local_is_addressible(
            SharednessT::Shared,
            local_a);
        (addressible_if_mutable, addressible_if_immutable)
    };
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
    */
}

#[test]
pub fn captured_own_is_borrow() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    // Here, the scout determined that the closure is only ever borrowing
    // it (during the dereference to get its member) so typingpass doesn't put
    // an address into the closure, it instead puts a reference. Specifically,
    // a borrow reference (because why would we want to move this into the
    // closure struct?).
    // This means the closure struct contains a borrow reference. This means
    // the environment in the closure has to match this; the environment has
    // to have a borrow reference instead of an owning reference.
    let mut compile = test_without_borrow_check(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        // TSUGAR: m.hp is &int
        r"
struct Marine {
  hp int;
}
exported func main() int {
  m = Marine(9);
  return { __copy_prim(&m.hp) }();
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}

#[test]
fn test_closure_s_local_variables() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test_without_borrow_check(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        // TSUGAR: "exported func main() int { x = 4; return {x}(); }"
        r#"
exported func main() int {
  x = 4;
  return {__copy_prim(&x)}();
}
"#,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_lambda_in("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::LetNormal(LetNormalTE {
            variable: LocalVariable {
                name: IVarNameT::ClosureParam(_),
                tyype: KindT::BorrowRef(BorrowRefT {
                    inner: KindT::Struct(StructTT {
                        id: IdT {
                            init_steps: &[INameT::Function(FunctionNameT {
                                template: FunctionTemplateNameT { human_name: StrI("main"), .. },
                                template_args: &[],
                                parameters: &[],
                                ..
                            })],
                            local_name: INameT::LambdaCitizen(LambdaCitizenNameT {
                                template: LambdaCitizenTemplateNameT { .. },
                                ..
                            }),
                            ..
                        },
                        ..
                    }),
                }),
            },
            ..
        }) => Some(())
    );
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::LetNormal(LetNormalTE {
            variable: LocalVariable {
                name: IVarNameT::TypingPassBlockResultVar(_),
                tyype: KindT::Int(IntT { bits: 32 }),
            },
            ..
        }) => Some(())
    );
}

// VCOORD: have another one of these tests, but actually returning a reference
#[test]
// ZONION: re-enable for onion
#[ignore = "share-blanket / bound-resolution not yet honest for clone-of-borrow-in-generics; needs `&&T` structural distinctness or primitive-borrow flip"]
fn test_returning_a_nonmutable_closured_variable_from_the_closure() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        // TSUGAR: "exported func main() int { x = 4; return {x}(); }"
        r#"
exported func main() int {
  x = 4;
  return {__copy_prim(&x)}();
}
"#,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let lambda = coutputs.lookup_lambda_in("main");

        // The lambda's first param is the borrowed closure struct; find its definition and confirm
        // it captured exactly one member, by borrow-of-int.
        let closure_struct_tt = match lambda.header.params.first().unwrap().tyype {
            KindT::BorrowRef(BorrowRefT { inner: KindT::Struct(stt) }) => stt,
            other => panic!("expected a borrowed closure-struct param, got {:?}", other),
        };
        let closure_def = coutputs.lookup_struct(*closure_struct_tt.id);
        match closure_def.members {
            [StructMemberT {
                tyype: KindT::BorrowRef(BorrowRefT { inner: KindT::Int(IntT { bits: 32 }) }),
                ..
            }] => {}
            other => panic!("expected one borrow-of-int captured member, got {:?}", other),
        }

        // Reading the captured var lowers to a MemberLookup on the closure struct, named by the
        // struct's member name (IVarNameT::Member), not the capture's original Local name.
        collect_only_tnode!(
            NodeRefT::FunctionDefinition(lambda),
            NodeRefT::MemberLookup(MemberLookupTE { member_name: IVarNameT::Member(_), .. }) => Some(())
        );
    }

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 4 }) => {}
        other => panic!("expected VonInt(4), got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn mutates_from_inside_a_closure() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        // TSUGAR: x is reused after addressible-promotion → wrap with __copy_prim
        r"
exported func main() int {
  x = 4;
  { set x = x + 1; }();
  return __copy_prim(&x);
}
",
    );


    {
        unimplemented!();
        // let interner = compile.interner;
        // let coutputs = compile.expect_compiler_outputs();
        //
        // // The struct should have an int x in it.
        // let closure = coutputs.lookup_lambda_in("main");
        // let closure_struct = closure.header.params.first().unwrap().tyype.kind.expect_struct();
        // let closure_struct_def = coutputs.lookup_struct(closure_struct.id);
        // let expected_members = vec![
        //     IStructMemberT::Normal(NormalStructMemberT {
        //         name: IVarNameT::Member(interner.intern_member_name(MemberNameT { name: scout_arena.intern_str("x")})),
        //         tyype: IMemberTypeT::Address(AddressMemberTypeT {
        //             reference: CoordT::new(
        //                 OwnershipT::Own,
        //                 RegionT { region: IRegionT::Default },
        //                 KindT::Int(IntT { bits: 32 }),
        //             ),
        //         }),
        //     }),
        // ];
        // assert_eq!(closure_struct_def.members, expected_members.as_slice());
        //
        // let lambda = coutputs.lookup_lambda_in("main");
        // collect_only_tnode!(
        //     NodeRefT::FunctionDefinition(lambda),
        //     NodeRefT::Mutate(MutateTE {
        //         destination_expr: AddressExpressionTE::AddressMemberLookup(AddressMemberLookupTE {
        //             member_name: IVarNameT::Member(MemberNameT { name: StrI("x"), .. }),
        //             result_type2: CoordT {
        //                 ownership: OwnershipT::Own,
        //                 kind: KindT::Int(IntT { bits: 32 }),
        //                 ..
        //             },
        //             ..
        //         }),
        //         ..
        //     }) => Some(())
        // );
        //
        // let main = coutputs.lookup_function_by_str("main");
        // collect_only_tnode!(
        //     NodeRefT::FunctionDefinition(main),
        //     NodeRefT::LetNormal(LetNormalTE {
        //         variable: ILocalVariableT::Addressible(AddressibleLocalVariableT {
        //                 ..
        //         }),
        //         ..
        //     }) => Some(())
        // );
    }

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
pub fn mutates_from_inside_a_closure_inside_a_closure() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "exported func main() int { x = 4; { { set x = x + 1; }(); }(); return x; }",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}

#[test]
// ZONION: re-enable for onion
#[ignore = "share-blanket / bound-resolution not yet honest for clone-of-borrow-in-generics; needs `&&T` structural distinctness or primitive-borrow flip"]
fn read_from_inside_a_closure_inside_a_closure() {
    unimplemented!();
    /*
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
        // TSUGAR: x captured by nested closure — Own primitive, copy
        r"
exported func main() int {
  x = 42;
  return { { __copy_prim(&x) }() }();
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
    */
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
pub fn mutable_lambda() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let source = load_expected("programs/lambdas/lambdamut.vale");
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let closure_structs: Vec<_> = coutputs.structs.iter().filter(|s| matches!(
            s.instantiated_citizen.id.local_name,
            INameT::LambdaCitizen(LambdaCitizenNameT {
                template: LambdaCitizenTemplateNameT {
                    code_location: CodeLocationS {
                        file: FileCoordinate {
                            package_coord: PackageCoordinate {
                                module: StrI("test"),
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
        assert!(matches!(closure_struct.sharedness, SharednessT::Single));
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}

#[test]
fn capture() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test_without_borrow_check(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        // TSUGAR: box.i is &int
        r"
func myFunc<T, F>(generator &F) T
where func(&F, int)T, func drop(F)void
{
  return generator(9);
}

struct IntBox {
  i int;
}

exported func main() int {
  box = IntBox(7);
  lam = (col) => { __copy_prim(&box.i) };
  board = myFunc<int>(&lam);
  return board;
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}

