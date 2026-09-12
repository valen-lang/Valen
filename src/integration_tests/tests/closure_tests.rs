#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use crate::collect_only_tnode;
use crate::integration_tests::tests::run_compilation::test;
use crate::integration_tests::tests::run_compilation::test_without_borrow_check;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::postparsing::names::CodeNameS;
use crate::typing::ast::citizens::StructMemberT;
use crate::typing::ast::expressions::DerefTE;
use crate::typing::ast::expressions::ExpressionTE;
use crate::typing::ast::expressions::LetNormalTE;
use crate::typing::ast::expressions::MemberLookupTE;
use crate::typing::ast::expressions::MutateTE;
use crate::typing::names::names::MemberNameT;
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
    let mut compile = test_without_borrow_check(
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
        let coutputs = compile.expect_compiler_outputs();
        let lambda = coutputs.lookup_lambda_in("main");

        let closure_struct_tt = match lambda.header.params.first().unwrap().tyype {
            KindT::BorrowRef(BorrowRefT { inner: KindT::Struct(stt) }) => stt,
            other => panic!("expected a borrowed closure-struct param, got {:?}", other),
        };
        let closure_def = coutputs.lookup_struct(*closure_struct_tt.id);
        match closure_def.members {
            [StructMemberT {
                name: MemberNameT { imprecise_name: CodeNameS { name: StrI("x"), .. }, .. },
                tyype: KindT::BorrowRef(BorrowRefT { inner: KindT::Int(IntT { bits: 32 }) }),
            }] => {}
            other => panic!("expected one borrow-of-int captured member `x`, got {:?}", other),
        }

        collect_only_tnode!(
            NodeRefT::FunctionDefinition(lambda),
            NodeRefT::Mutate(MutateTE {
                destination_expr: ExpressionTE::Deref(DerefTE {
                    inner: ExpressionTE::MemberLookup(MemberLookupTE {
                        member_name: IVarNameT::Member(MemberNameT {
                            imprecise_name: CodeNameS { name: StrI("x"), .. }, ..
                        }),
                        ..
                    }),
                    ..
                }),
                ..
            }) => Some(())
        );
    }

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}

#[test]
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
    let mut compile = test_without_borrow_check(
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
fn read_from_inside_a_closure_inside_a_closure() {
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
}

#[test]
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
    let mut compile = test_without_borrow_check(
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

