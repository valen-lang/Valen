#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::typing_interner::TypingInterner;
use crate::testvm::von::IVonData;
use crate::testvm::von::VonInt;

pub struct InferTemplateTests;

#[test]
#[ignore] // ZONION: re-enable for onion
pub fn test_inferring_a_borrowed_argument() {
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
        // TSUGAR: moo(&x).hp is &int
        r"
struct Muta { hp int; }
func moo<T>(m &T) &T { return m; }
exported func main() int {
  x = Muta(10);
  return __copy_prim(moo(&x).hp);
}
",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let moo = coutputs.lookup_function_by_str("moo");
        match moo.header.params {
            [ParameterT {
                name: IVarNameT::Member(MemberNameT { name: StrI("m"), .. }),
                tyype: CoordT { ownership: OwnershipT::Borrow, .. },
                ..
            }] => {}
            _ => panic!("moo.header.params didn't match expected pattern"),
        }
        let main = coutputs.lookup_function_by_str("main");
        collect_only_tnode!(
            NodeRefT::FunctionDefinition(main),
            NodeRefT::FunctionCall(FunctionCallTE {
                callable: PrototypeT {
                    id: IdT {
                        local_name: INameT::Function(FunctionNameT {
                            template: FunctionTemplateNameT { human_name: StrI("moo"), .. },
                            template_args: &[ITemplataT::Kind(KindTemplataT {
                                coord: CoordT {
                                    ownership: OwnershipT::Own,
                                    kind: KindT::Struct(StructTT {
                                        id: IdT {
                                            package_coord: x_package_coord,
                                            init_steps: &[],
                                            local_name: INameT::Struct(StructNameT {
                                                template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Muta"), .. }),
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
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 10 }) => {}
        other => panic!("expected VonInt(10), got {:?}", other),
    }
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
pub fn test_inferring_a_borrowed_static_sized_array() {
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
        // TSUGAR: m[0].hp is &int
        r"
struct Muta { hp int; }
func moo<N Int>(m &[#N]Muta) int { return __copy_prim(m[0].hp); }
exported func main() int {
  x = [#](Muta(10));
  return moo(&x);
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 10 }) => {}
        other => panic!("expected VonInt(10), got {:?}", other),
    }
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
pub fn test_inferring_an_owning_static_sized_array() {
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
        // TSUGAR: m[0].hp is &int
        r"
struct Muta { hp int; }
func moo<N Int>(m [#N]Muta) int { return __copy_prim(m[0].hp); }
exported func main() int {
  x = [#](Muta(10));
  return moo(^x);
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 10 }) => {}
        other => panic!("expected VonInt(10), got {:?}", other),
    }
    */
}

