use bumpalo::Bump;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::code_source::{CodeSource, Source};
use crate::postparsing::names::{CodeNameS, FunctionNameS, IFunctionDeclarationNameS, IImpreciseNameValS};
use crate::scout_arena::ScoutArena;
use crate::typing::ast::expressions::{ExpressionTE, ConstantIntTE, LocalLookupTE, MutateTE, ReferenceMemberLookupTE};
use crate::typing::compiler_error_humanizer::humanize;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::env::function_environment_t::{LocalVariable};
use crate::typing::names::names::{CodeVarNameT, FunctionNameValT, FunctionTemplateNameT, IdT, IdValT, INameT, IStructTemplateNameT, IVarNameT, RawArrayNameT, StaticSizedArrayNameT, StructNameValT, StructTemplateNameT};
use crate::typing::templata::templata::{ITemplataT, KindTemplataT};
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::typing::test::humanize_helper::{humanize_compile_error, assert_humanized_eq};
use crate::typing::types::types::{KindT, IntT, RegionT, SharednessT, ShareRefT, StaticSizedArrayTT, StructTTValT};
use crate::typing::typing_interner::TypingInterner;
use crate::tests::tests::{new_humanizer_test_code_map, new_test_code_map};
use crate::utils::code_hierarchy::{FileCoordinateMap, PackageCoordinate};
use crate::utils::range::{CodeLocationS, RangeS};
use crate::utils::source_code_utils::{humanize_pos_code_map, line_containing, line_range_containing, lines_between};
use crate::utils::fx::HashMap;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::names::names::StructNameT;
use crate::typing::overload_resolver::FindFunctionFailure;
use crate::builtins::builtins::{builtin_source_for_arrays, empty_v_builtins_stub};
use crate::collect_only_tnode;
use std::marker::PhantomData;



pub fn read_code_from_resource(resource_filename: &str) -> String {
  panic!("Unimplemented: read_code_from_resource");
}

#[test]
fn test_mutating_a_local_var() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r"
exported func main() {a = 3; set a = 4; }
";
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, code),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Mutate(MutateTE {
            destination_expr: ExpressionTE::LocalLookup(LocalLookupTE {
                local_variable: LocalVariable {
                    name: IVarNameT::CodeVar(CodeVarNameT { name: StrI("a"), .. }),
                    ..
                },
                ..
            }),
            source_expr: ExpressionTE::ConstantInt(ConstantIntTE {
                value: ITemplataT::Integer(4),
                ..
            }),
            ..
        }) => Some(())
    );

    let lookup: &LocalLookupTE = collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::LocalLookup(l) => Some(l)
    );
    // The lookup is a borrow of the local's storage, so the int is what it points at.
    assert_eq!(lookup.result.inner, KindT::Int(IntT { bits: 32 }));
}

#[test]
fn test_mutable_member_permission() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r"
struct Engine { fuel int; }
struct Spaceship { engine Engine; }
exported func main() {
  ship = Spaceship(Engine(10));
  set ship.engine = Engine(15);
}
";
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, code),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");

    let lookup: &ReferenceMemberLookupTE = collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::ReferenceMemberLookup(l) => Some(l)
    );
    // The lookup is a borrow of the member, so the struct is what it points at.
    match lookup.result.inner {
        KindT::Struct(_) => {}
        x => panic!("{:?}", x),
    }
}

#[test]
fn local_set_upcasts() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r"
import v.builtins.drop.*;

interface IXOption<T Ref> where func drop(T)void { }
struct XSome<T Ref> where func drop(T)void { value T; }
impl<T Ref> IXOption<T> for XSome<T> where func drop(T)void;
struct XNone<T Ref> where func drop(T)void { }
impl<T Ref> IXOption<T> for XNone<T> where func drop(T)void;

exported func main() {
  m IXOption<int> = XNone<int>();
  set m = XSome(6);
}
";
    let code_source = CodeSource::new(vec![
        Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
        new_test_code_map(&parse_arena, code),
        Source::Fn(empty_v_builtins_stub),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Mutate(MutateTE {
            source_expr: ExpressionTE::Upcast(_),
            ..
        }) => Some(())
    );
}

#[test]
fn expr_set_upcasts() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r"
import v.builtins.drop.*;

interface IXOption<T Ref> where func drop(T)void { }
struct XSome<T Ref> where func drop(T)void { value T; }
impl<T Ref> IXOption<T> for XSome<T>;
struct XNone<T Ref> where func drop(T)void { }
impl<T Ref> IXOption<T> for XNone<T>;

struct Marine {
  weapon IXOption<int>;
}
exported func main() {
  m = Marine(XNone<int>());
  set m.weapon = XSome(6);
}
";
    let code_source = CodeSource::new(vec![
        Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
        new_test_code_map(&parse_arena, code),
        Source::Fn(empty_v_builtins_stub),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source,
    );
    let coutputs = compile.expect_compiler_outputs();
    let main = coutputs.lookup_function_by_str("main");
    collect_only_tnode!(
        NodeRefT::FunctionDefinition(main),
        NodeRefT::Mutate(MutateTE {
            source_expr: ExpressionTE::Upcast(_),
            ..
        }) => Some(())
    );
}

#[test]
fn reports_when_we_try_to_mutate_a_local_variable_with_wrong_type() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r#"

exported func main() {
  a = 5;
  set a = "blah";
}
"#;
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, code),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source);
    let err = compile.get_compiler_outputs().err().unwrap();
    match &err {
        ICompileErrorT::CouldntConvertForMutateT { expected_type: KindT::Int(IntT { bits: 32 }), actual_type: KindT::ShareRef(ShareRefT { inner: KindT::Str(_) }), .. } => {}
        _ => panic!("expected CouldntConvertForMutateT"),
    }
    assert_humanized_eq(
        &humanize_compile_error(&mut compile, err),
        r##"At test:0.vale:3:1:
exported func main() {
At test:0.vale:5:7:
  set a = "blah";
Mutate couldn't convert @str to expected destination type i32
"##,
    );
}

#[test]
fn reports_when_we_try_to_override_a_non_interface() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r"
impl int for Bork;
struct Bork { }
exported func main() {
  Bork();
}
";
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, code),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source);
    let err = compile.get_compiler_outputs().err()
        .unwrap_or_else(|| panic!("expected Err(CantImplNonInterface), got Ok"));
    match &err {
        ICompileErrorT::CantImplNonInterface { templata: ITemplataT::Kind(KindTemplataT { kind: KindT::Int(IntT { bits: 32 }) }), .. } => {}
        _ => panic!("expected CantImplNonInterface"),
    }
    assert_humanized_eq(
        &humanize_compile_error(&mut compile, err),
        r#"At test:0.vale:2:1:
impl int for Bork;
Can't extend a non-interface: i32
"#,
    );
}

#[test]
fn can_mutate_an_element_in_a_runtime_sized_array() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = r"
import v.builtins.arrays.*;
import v.builtins.drop.*;

exported func main() int {
  arr = Array<int>(3);
  arr.push(0);
  arr.push(1);
  arr.push(2);
  set arr[1] = 10;
  return 73;
}
";
    let code_source = CodeSource::new(vec![
        builtin_source_for_arrays(&parse_arena, &parser_keywords),
        new_test_code_map(&parse_arena, code),
        Source::Fn(empty_v_builtins_stub),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source);
    compile.expect_compiler_outputs();
}

#[test]
fn can_restackify_in_destructure_pattern() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    // TSUGAR: ship.fuel is &int
    let code = r"
#!DeriveStructDrop
struct Ship { fuel int; }

/// TODO: Bring tuples back
#!DeriveStructDrop
struct GetFuelResult { fuel int; ship Ship; }

func GetFuel(ship Ship) GetFuelResult {
  return GetFuelResult(__copy_prim(&ship.fuel), ^ship);
}

exported func main() int {
  ship = Ship(42);
  [fuel, set ship] = GetFuel(^ship);
  [f] = ^ship;
  return ^fuel;
}
";
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, code),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source);
    compile.expect_compiler_outputs();
}
#[test]
fn if_branches_must_move_same_variables() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    // TSUGAR: s.x is &int
    let code = r"
struct S { x int; }
func consume(s S) int { return __copy_prim(&s.x); }
exported func main() int {
  s = S(7);
  result = 0;
  if true {
    set result = consume(^s);
  } else {
    set result = 5;
  }
  return ^result;
}
";
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, code),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source);
    let err = compile.get_compiler_outputs().err()
        .unwrap_or_else(|| panic!("expected Err(RangedInternalErrorT) for if-branch-move-mismatch, got Ok"));
    match &err {
        crate::typing::compiler_error_reporter::ICompileErrorT::RangedInternalErrorT { .. } => {},
        other => panic!("expected RangedInternalErrorT for if-branch-move-mismatch, got: {:?}", other),
    }
    assert_humanized_eq(
        &humanize_compile_error(&mut compile, err),
        r##"At test:0.vale:4:1:
exported func main() int {
At test:0.vale:7:3:
  if true {
Internal error: Must move same variables from inside branches!
From then branch: {CodeVar(CodeVarNameT { name: "s" })}
From else branch: {}
"##,
    );
}
#[test]
fn if_branches_moving_same_vars_different_order_compiles() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    // TSUGAR: s.x is &int
    let code = r"
struct S { x int; }
func consume(s S) int { return __copy_prim(&s.x); }
exported func main() int {
  a = S(1);
  b = S(2);
  result = 0;
  if true {
    set result = consume(^a);
    set result = consume(^b);
  } else {
    set result = consume(^b);
    set result = consume(^a);
  }
  return ^result;
}
";
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, code),
    ]);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = compiler_test_compilation(&typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source);
    match compile.get_compiler_outputs() {
        Ok(_) => {},
        Err(e) => panic!("expected Ok (same vars moved in both branches, just different order), got Err: {:?}", e),
    }
}

#[test]
fn humanize_errors() {
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let typing_interner = TypingInterner::new(&typing_bump);

    let tz_code_loc = CodeLocationS::test_zero(&scout_arena);
    let tz = RangeS::test_zero(&scout_arena);
    let tz_slice: &[RangeS] = typing_bump.alloc_slice_copy(&[tz]);
    let test_tld = scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]);

    let filenames_and_sources = new_humanizer_test_code_map(&scout_arena, "blah blah blah\nblah blah blah");
    let humanize_pos = |x| humanize_pos_code_map(&filenames_and_sources, &x);
    let lines_between = |x, y| lines_between(&filenames_and_sources, &x, &y);
    let line_range_containing = |x| line_range_containing(&filenames_and_sources, &x);
    let line_containing = |x| line_containing(&filenames_and_sources, &x);

    let firefly_struct_template_name = typing_interner.intern_struct_template_name(
        StructTemplateNameT { human_name: scout_arena.intern_str("Firefly")});
    let firefly_struct_name = typing_interner.intern_struct_name(
        StructNameValT { template: IStructTemplateNameT::StructTemplate(firefly_struct_template_name), template_args: &[] });
    let firefly_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Struct(firefly_struct_name),
    });
    let firefly_tt = typing_interner.intern_struct_tt(StructTTValT { id: *firefly_id });
    let firefly_kind = KindT::Struct(firefly_tt);
    let firefly_coord = firefly_kind;

    let serenity_struct_template_name = typing_interner.intern_struct_template_name(
        StructTemplateNameT { human_name: scout_arena.intern_str("Serenity")});
    let serenity_struct_name = typing_interner.intern_struct_name(
        StructNameValT { template: IStructTemplateNameT::StructTemplate(serenity_struct_template_name), template_args: &[] });
    let serenity_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Struct(serenity_struct_name),
    });
    let serenity_tt = typing_interner.intern_struct_tt(StructTTValT { id: *serenity_id });
    let serenity_kind = KindT::Struct(serenity_tt);
    let serenity_coord = serenity_kind;

    let myfunc_template_name = typing_interner.intern_function_template_name(
        FunctionTemplateNameT { human_name: scout_arena.intern_str("myFunc"), code_location: tz_code_loc});
    let myfunc_func_name = typing_interner.intern_function_name(
        FunctionNameValT { template: myfunc_template_name, template_args: &[], parameters: &[] });
    let myfunc_id = typing_interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Function(myfunc_func_name),
    });

    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindTypeT { range: tz_slice, name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str("Spaceship") })) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindFunctionToCallT { range: tz_slice, fff: FindFunctionFailure {
            name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str("") })),
            args: &[], rejected_callee_to_reason: &[],
        } }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CannotSubscriptT { range: tz_slice, tyype: firefly_kind }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindIdentifierToLoadT { range: tz_slice, name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str("spaceship") })) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntFindMemberT { range: tz_slice, member_name: "hp" }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::BodyResultDoesntMatch {
            range: tz_slice,
            function_name: IFunctionDeclarationNameS::FunctionName(FunctionNameS {
                name: scout_arena.intern_str("myFunc"),
                code_location: tz_code_loc,
            }),
            expected_return_type: firefly_coord,
            result_type: serenity_coord,
        }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntConvertForReturnT { range: tz_slice, expected_type: firefly_coord, actual_type: serenity_coord }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntConvertForMutateT { range: tz_slice, expected_type: firefly_coord, actual_type: serenity_coord }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CouldntConvertForMutateT { range: tz_slice, expected_type: firefly_coord, actual_type: serenity_coord }).is_empty());
    let hp_var_name: &CodeVarNameT = typing_bump.alloc(CodeVarNameT { name: scout_arena.intern_str("hp")});
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantMoveOutOfMemberT { range: tz_slice, name: IVarNameT::CodeVar(hp_var_name) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantReconcileBranchesResults { range: tz_slice, then_result: firefly_coord, else_result: serenity_coord }).is_empty());
    let firefly_var_name: &CodeVarNameT = typing_bump.alloc(CodeVarNameT { name: scout_arena.intern_str("firefly")});
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantUseUnstackifiedLocal { range: tz_slice, local_id: IVarNameT::CodeVar(firefly_var_name) }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::FunctionAlreadyExists { old_function_range: tz, new_function_range: tz, signature: *myfunc_id }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::LambdaReturnDoesntMatchInterfaceConstructor { range: tz_slice }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::IfConditionIsntBoolean { range: tz_slice, actual_type: firefly_coord }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::WhileConditionIsntBoolean { range: tz_slice, actual_type: firefly_coord }).is_empty());
    assert!(!humanize(&scout_arena, &typing_interner, false, &humanize_pos, &lines_between, &line_range_containing, &line_containing,
        ICompileErrorT::CantImplNonInterface { range: tz_slice, templata: ITemplataT::Kind(typing_bump.alloc(KindTemplataT { kind: firefly_kind })) }).is_empty());
}

