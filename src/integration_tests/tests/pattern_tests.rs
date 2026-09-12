#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use crate::integration_tests::tests::run_compilation::test;
use crate::integration_tests::tests::run_compilation::test_without_borrow_check;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::instantiating::ast::names::IdI;
use crate::instantiating::ast::names::INameI;
use crate::instantiating::ast::names::IStructTemplateNameI;
use crate::instantiating::ast::names::StructNameI;
use crate::instantiating::ast::names::StructTemplateNameI;
use crate::instantiating::ast::types::BorrowRefIT;
use crate::instantiating::ast::types::IntIT;
use crate::instantiating::ast::types::KindIT;
use crate::instantiating::ast::types::StructIT;
use crate::typing::types::types::IntT;
use crate::typing::types::types::KindT;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::typing_interner::TypingInterner;
use crate::testvm::von::IVonData;
use crate::testvm::von::VonInt;
pub struct PatternTests;

#[test]
fn test_matching_a_multiple_member_seq_of_immutables() {
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
    // Checks that the 5 made it into y, and it was an int
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        // TSUGAR: "exported func main() int { [x, y] = (4, 5); return y; }"
        r#"
exported func main() int {
  [x, y] = (4, 5);
  return __copy_prim(&y);
}
"#,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        assert_eq!(main.header.return_type, KindT::Int(IntT::I32));
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}



#[test]
fn test_matching_a_multiple_member_seq_of_mutables() {
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
    // Checks that the 5 made it into y, and it was an int
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        // TSUGAR: y.hp is &int
        r#"
struct Marine { hp int; }
exported func main() int {
  [x, y] = (Marine(6), Marine(8));
  return __copy_prim(&y.hp);
}
"#,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        assert_eq!(main.header.return_type, KindT::Int(IntT::I32));
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 8 }) => {}
        other => panic!("expected VonInt(8), got {:?}", other),
    }
}



#[test]
fn test_matching_a_multiple_member_pack_of_immutable_and_own() {
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
    // Checks that the 5 made it into y, and it was an int
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        // TSUGAR: y.hp is &int
        r#"
struct Marine { hp int; }
exported func main() int {
  [x, y] = (7, Marine(8));
  return __copy_prim(&y.hp);
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 8 }) => {}
        other => panic!("expected VonInt(8), got {:?}", other),
    }
}



#[test]
fn test_matching_a_multiple_member_pack_of_immutable_and_borrow() {
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
    // Checks that the 5 made it into y, and it was an int
    let mut compile = test_without_borrow_check(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        // TSUGAR: y.hp is &int
        r#"
struct Marine { hp int; }
exported func main() int {
  m = Marine(8);
  [x, y] = (7, &m);
  return __copy_prim(&y.hp);
}
"#,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        assert_eq!(coutputs.lookup_function_by_str("main").header.return_type, KindT::Int(IntT::I32));
    }
    {
        let monouts = compile.get_monouts();
        let tup_def = monouts.lookup_struct_by_name("Tup2");
        let tup_def_member_types: Vec<KindIT<'_, '_>> = tup_def.members.iter().map(|m| m.tyype).collect();
        match tup_def_member_types.as_slice() {
            [
                KindIT::IntIT(IntIT { bits: 32, .. }),
                KindIT::BorrowRefIT(BorrowRefIT {
                    inner: KindIT::StructIT(StructIT {
                        id: IdI {
                            init_steps: &[],
                            local_name: INameI::StructName(StructNameI {
                                template: IStructTemplateNameI::StructTemplate(StructTemplateNameI {
                                    human_name: StrI("Marine"),
                                    ..
                                }),
                                template_args: &[],
                            }),
                            ..
                        },
                    }),
                }),
            ] => {}
            other => panic!("expected Tup2 members [own int, borrow Marine], got {:?}", other),
        }
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 8 }) => {}
        other => panic!("expected VonInt(8), got {:?}", other),
    }
}



#[ignore = "blocked on borrow checker (borrow-group) — owned by another worktree"]
#[test]
fn test_destructuring_a_shared() {
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
        // TSUGAR: i is &int
        r"
import array.iter.*;
exported func main() int {
  sm = [#]([#](42, 73, 73));
  foreach [i, m1] in sm {
    return __copy_prim(&i);
  }
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}




#[test]
fn ignore_destructure() {
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
        r"
struct Marine {
  hp int;
}
exported func main() int {
  m = Marine(4);
  Marine[_] = ^m;
  return 42;
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}


