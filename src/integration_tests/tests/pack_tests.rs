#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use crate::collect_where_tnode;
use crate::integration_tests::tests::run_compilation::test;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::typing::ast::expressions::ConstructTE;
use crate::typing::ast::expressions::ExpressionTE;
use crate::typing::names::names::IdT;
use crate::typing::names::names::INameT;
use crate::typing::names::names::IStructTemplateNameT;
use crate::typing::names::names::StructNameT;
use crate::typing::names::names::StructTemplateNameT;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::types::types::StructTT;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::typing_interner::TypingInterner;
use crate::testvm::von::IVonData;
use crate::testvm::von::VonInt;

pub struct PackTests;

#[test]
fn extract_seq() {
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
exported func main() int {
  [x, y] = (5, 6);
  return x;
}
",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches = collect_where_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::Construct(ConstructTE {
                struct_tt: StructTT {
                    id: IdT { local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Tup2"), .. }),
                        .. }), .. },
                    ..
                },
                args: &[_, _],
                ..
            }) => Some(())
        );
        assert_eq!(matches.len(), 1);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 5 }) => {}
        other => panic!("Expected VonInt(5), got {:?}", other),
    }
}

#[test]
fn nested_seqs() {
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
exported func main() int {
  [x, [y, z]] = ((4, 5), (6, 7));
  return y;
}
",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches = collect_where_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::Construct(ConstructTE {
                struct_tt: StructTT {
                    id: IdT { local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Tup2"), .. }),
                        .. }), .. },
                    ..
                },
                args: &[
                    ExpressionTE::Construct(ConstructTE { args: &[_, _], .. }),
                    ExpressionTE::Construct(ConstructTE { args: &[_, _], .. }),
                ],
                ..
            }) => Some(())
        );
        assert_eq!(matches.len(), 1);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 6 }) => {}
        other => panic!("Expected VonInt(6), got {:?}", other),
    }
}

#[test]
fn nested_tuples() {
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
exported func main() int {
  [x, [y, z]] = (5, (6, false));
  return x;
}
",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let matches = collect_where_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::Construct(ConstructTE {
                struct_tt: StructTT {
                    id: IdT { local_name: INameT::Struct(StructNameT {
                        template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Tup2"), .. }),
                        .. }), .. },
                    ..
                },
                args: &[
                    _,
                    ExpressionTE::Construct(ConstructTE { args: &[_, _], .. }),
                ],
                ..
            }) => Some(())
        );
        assert_eq!(matches.len(), 1);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 5 }) => {}
        other => panic!("Expected VonInt(5), got {:?}", other),
    }
}

