use crate::compile_options::GlobalOptions;
use crate::instantiating::instantiated_compilation::InstantiatedCompilation;
use crate::instantiating::instantiated_compilation::InstantiatorCompilationOptions;
use crate::code_source::{CodeSource, Source};
use crate::tests::tests::test_source_from_dir;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::tests::tests::new_test_code_map;
use std::marker::PhantomData;
use std::sync::Arc;
use bumpalo::Bump;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::typing_interner::TypingInterner;
use crate::collect_only_inode;
use crate::collect_where_inode;
use crate::instantiating::collector::NodeRefI;
use crate::instantiating::ast::expressions::{ExpressionIE, LocalLookupIE, ConstructIE, ConstantStrIE, ReferenceMemberLookupIE, DerefIE, ConstantIntIE, FunctionCallIE, InterfaceFunctionCallIE, UpcastIE, RuntimeSizedArrayLookupIE};
use crate::builtins::builtins::{builtin_source_for_arrays, empty_v_builtins_stub};
use crate::instantiating::ast::types::{KindIT, IntIT, BorrowRefIT, ShareRefIT};
use crate::instantiating::ast::ast::PrototypeI;
use crate::instantiating::ast::names::{IdI, INameI, FunctionNameIX, FunctionTemplateNameI};
use crate::interner::StrI;

pub fn test<'s, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx bumpalo::Bump,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i bumpalo::Bump,
    code: &str,
) -> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i, 'p: 'ctx,
{
    let packages_to_build: Vec<&'p PackageCoordinate<'p>> =
        vec![PackageCoordinate::test_tld(parse_arena, parser_keywords)];
    let code_source: &'ctx CodeSource<'p> = compilation_bump.alloc(CodeSource::new(vec![
        new_test_code_map(parse_arena, code),
        Source::Fn(test_source_from_dir),
    ]));
    let global_options = GlobalOptions {
        sanity_check: true,
        use_overload_index: true,
        use_optimized_solver: true,
        verbose_errors: true,
        debug_output: true,
    };
    let instantiator_options = InstantiatorCompilationOptions {
        debug_out: Arc::new(|x: &str| println!("{}", x)),
    };
    InstantiatedCompilation::new(
        typing_interner,
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        packages_to_build,
        code_source,
        global_options,
        instantiator_options,
        instantiating_bump,
    )
}

/// Like `test`, but prepends the `v.builtins.arrays` bundle (arrays + arith + drop + implicit_clone)
/// so array-constructing fixtures resolve `Array`/`[]T`. Mirrors the typing pass's array-test setup
/// (`builtin_source_for_arrays` + `empty_v_builtins_stub`). The default `test` harness omits builtins.
pub fn test_with_array_builtins<'s, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx bumpalo::Bump,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i bumpalo::Bump,
    code: &str,
) -> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i, 'p: 'ctx,
{
    let packages_to_build: Vec<&'p PackageCoordinate<'p>> =
        vec![PackageCoordinate::test_tld(parse_arena, parser_keywords)];
    let code_source: &'ctx CodeSource<'p> = compilation_bump.alloc(CodeSource::new(vec![
        builtin_source_for_arrays(parse_arena, parser_keywords),
        new_test_code_map(parse_arena, code),
        Source::Fn(empty_v_builtins_stub),
    ]));
    let global_options = GlobalOptions {
        sanity_check: true,
        use_overload_index: true,
        use_optimized_solver: true,
        verbose_errors: true,
        debug_output: true,
    };
    let instantiator_options = InstantiatorCompilationOptions {
        debug_out: Arc::new(|x: &str| println!("{}", x)),
    };
    InstantiatedCompilation::new(
        typing_interner,
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        packages_to_build,
        code_source,
        global_options,
        instantiator_options,
        instantiating_bump,
    )
}

/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct InstantiatedTests<'s, 't> {
  pub _marker: PhantomData<(&'s (), &'t ())>,
}


#[test]
fn test_templates() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r"
func drop(x int) { }
func bork<T>(a T) void where func drop(T)void {
  // implicitly calls drop
}
exported func main() {
  bork(3);
}
";
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    compile.get_monouts();
}

/// A local lookup yields a borrow of the local's storage: reading `&a` (an int local) instantiates
/// to a LocalLookupIE whose result is `BorrowRefIT<int>` — proving the onion storage-read invariant
/// and int as a bare primitive.
#[test]
fn local_lookup_yields_borrow_of_storage() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
func take(i &int) int { 7 }
exported func main() int {
  a = 2;
  return take(&a);
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let main = monouts.lookup_function_by_str("main");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::LocalLookup(LocalLookupIE {
            result: &BorrowRefIT { inner: KindIT::IntIT(IntIT { bits: 32 }) },
            ..
        })) => Some(())
    );
}

/// An owned value carries zero wraps: the generated `Ship` constructor's ConstructIE result is a
/// bare `StructIT` (not `OwnRefIT`/`BorrowRefIT` of a struct) — proving ownership maps to no wrap.
#[test]
fn owned_construct_is_bare_kind() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
#!DeriveStructDrop
struct Ship { fuel int; }
exported func main() {
  s = Ship(10);
  [_] = ^s;
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let ship_ctor = monouts.lookup_function_by_str("Ship");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(ship_ctor),
        NodeRefI::Expression(ExpressionIE::Construct(ConstructIE {
            result: KindIT::StructIT(_),
            ..
        })) => Some(())
    );
}

/// A borrow-reference parameter's coord is `BorrowRefIT<StructIT>`: `&Ship` instantiates to a
/// borrow wrap around the citizen kind.
#[test]
fn borrow_param_is_borrow_wrapped() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
#!DeriveStructDrop
struct Ship { fuel int; }
func take(s &Ship) int { 7 }
exported func main() {
  s = Ship(10);
  take(&s);
  [_] = ^s;
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let take = monouts.lookup_function_by_str("take");
    let [param] = take.header.params else {
        panic!("expected exactly one param, got {:?}", take.header.params)
    };
    match param.tyype {
        KindIT::BorrowRefIT(&BorrowRefIT { inner: KindIT::StructIT(_) }) => {}
        other => panic!("expected &Ship param as BorrowRefIT<StructIT>, got {:?}", other),
    }
}

/// A string constant is share-wrapped: `"hello"` instantiates to a ConstantStrIE whose result is
/// `ShareRefIT<StrIT>` — proving the immutable/shared wrap.
#[test]
fn string_constant_is_share_wrapped() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
exported func main() str {
  return "hello";
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let main = monouts.lookup_function_by_str("main");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::ConstantStr(ConstantStrIE {
            result: KindIT::ShareRefIT(&ShareRefIT { inner: KindIT::StrIT(_) }),
            ..
        })) => Some(())
    );
}

/// A struct member read yields a borrow of the member's storage: `s.fuel` (s a `&Ship`)
/// instantiates to a ReferenceMemberLookupIE whose result is `BorrowRefIT<int>`.
#[test]
fn struct_member_read_yields_borrow_of_member() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
#!DeriveStructDrop
struct Ship { fuel int; }
func get_fuel(s &Ship) int { s.fuel }
exported func main() {
  s = Ship(10);
  get_fuel(&s);
  [_] = ^s;
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let get_fuel = monouts.lookup_function_by_str("get_fuel");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(get_fuel),
        NodeRefI::Expression(ExpressionIE::ReferenceMemberLookup(ReferenceMemberLookupIE {
            result: &BorrowRefIT { inner: KindIT::IntIT(IntIT { bits: 32 }) },
            ..
        })) => Some(())
    );
}

/// A DerefIE peels exactly one reference wrap. Looking up the `&Ship` local `s` yields a borrow of
/// its storage — `BorrowRefIT<BorrowRefIT<Ship>>` — and the Deref peels the outer borrow to expose
/// the stored `BorrowRefIT<Ship>`. (A primitive member value like `s.fuel` peels via CopyPrim, not
/// Deref — a separate node.)
#[test]
fn deref_peels_one_wrap() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
#!DeriveStructDrop
struct Ship { fuel int; }
func get_fuel(s &Ship) int { s.fuel }
exported func main() {
  s = Ship(10);
  get_fuel(&s);
  [_] = ^s;
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let get_fuel = monouts.lookup_function_by_str("get_fuel");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(get_fuel),
        NodeRefI::Expression(ExpressionIE::Deref(DerefIE {
            inner: ExpressionIE::LocalLookup(LocalLookupIE {
                result: &BorrowRefIT { inner: KindIT::BorrowRefIT(&BorrowRefIT { inner: KindIT::StructIT(_) }) },
                ..
            }),
            result: KindIT::BorrowRefIT(&BorrowRefIT { inner: KindIT::StructIT(_) }),
            ..
        })) => Some(())
    );
}

/// Monomorphization substitutes the concrete kind for the type parameter: `bork<T>` called with
/// `int` instantiates to a `bork` whose param is a bare `IntIT` — no placeholder survives.
#[test]
fn generic_function_monomorphizes_type_param() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
func drop(x int) { }
func bork<T>(a T) void where func drop(T)void {
}
exported func main() {
  bork(3);
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let bork = monouts.lookup_function_by_str("bork");
    let [param] = bork.header.params else {
        panic!("expected exactly one param, got {:?}", bork.header.params)
    };
    match param.tyype {
        KindIT::IntIT(IntIT { bits: 32 }) => {}
        other => panic!("expected monomorphized bare int param, got {:?}", other),
    }
}

/// An `if` instantiates to an IfIE and the walker reaches all three arms: the ConstantBool
/// condition, the ConstantInt in the then-branch, and the ConstantInt in the else-branch.
#[test]
fn if_reaches_all_three_branches() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
exported func main() int {
  return if (true) { 42 } else { 73 };
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let main = monouts.lookup_function_by_str("main");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::If(_)) => Some(())
    );
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::ConstantBool(_)) => Some(())
    );
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::ConstantInt(ConstantIntIE { value: 42, .. })) => Some(())
    );
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::ConstantInt(ConstantIntIE { value: 73, .. })) => Some(())
    );
}

/// A `while` instantiates to a WhileIE and the walker descends into its body — a distinctive
/// constant declared inside the loop body is reachable. (The loop lowering emits more than one
/// `break`, so a break count would not pin body-descent; a unique in-body constant does.)
#[test]
fn while_body_reachable() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
exported func main() {
  while (true) {
    x = 7;
    break;
  }
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let main = monouts.lookup_function_by_str("main");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::While(_)) => Some(())
    );
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::ConstantInt(ConstantIntIE { value: 7, .. })) => Some(())
    );
}

/// A direct call carries a monomorphized prototype: calling generic `count<T>` with `int`
/// instantiates to a FunctionCallIE whose callable is `count` with its `T` parameter substituted to
/// a concrete `int` — proving the callee prototype is monomorphized, not a placeholder.
#[test]
fn function_call_carries_monomorphized_prototype() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
func drop(x int) { }
func count<T>(a T) int where func drop(T)void { 7 }
exported func main() int {
  return count(3);
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let main = monouts.lookup_function_by_str("main");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::FunctionCall(FunctionCallIE {
            callable: PrototypeI {
                id: IdI {
                    local_name: INameI::FunctionNameIX(FunctionNameIX {
                        template: FunctionTemplateNameI { human_name: StrI("count"), .. },
                        parameters: &[KindIT::IntIT(IntIT { bits: 32 })],
                        ..
                    }),
                    ..
                },
                return_type: KindIT::IntIT(IntIT { bits: 32 }),
            },
            ..
        })) => Some(())
    );
}

/// Virtual dispatch is generated: the instantiator emits exactly one InterfaceFunctionCallIE
/// (in the abstract `doCivicDance(Car)` dispatcher), carrying the virtual parameter's index. The
/// call site itself lowers to a plain FunctionCall to that abstract function — the dispatch node
/// lives in the dispatcher, not at the call site.
#[test]
fn interface_call_is_virtual_dispatch() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
sealed interface Car {
  func doCivicDance(virtual this Car) int;
}
struct Civic {}
impl Car for Civic;
func doCivicDance(civic Civic) int {
  return 4;
}
struct Toyota {}
impl Car for Toyota;
func doCivicDance(toyota Toyota) int {
  return 7;
}
exported func main() int {
  x Car = Toyota();
  return doCivicDance(^x);
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let mut virtual_dispatches = Vec::new();
    for f in monouts.functions {
        let cs = collect_where_inode!(
            NodeRefI::FunctionDefinition(f),
            NodeRefI::Expression(ExpressionIE::InterfaceFunctionCall(InterfaceFunctionCallIE {
                virtual_param_index: 0,
                ..
            })) => Some(())
        );
        virtual_dispatches.extend(cs);
    }
    assert_eq!(virtual_dispatches.len(), 1, "expected exactly one virtual dispatch (the abstract doCivicDance dispatcher)");
}

/// Assigning a concrete struct to an interface-typed local instantiates to an UpcastIE: the
/// concrete `Toyota` construction is upcast to an interface result.
#[test]
fn upcast_to_interface() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
sealed interface Car {
  func doCivicDance(virtual this Car) int;
}
struct Civic {}
impl Car for Civic;
func doCivicDance(civic Civic) int {
  return 4;
}
struct Toyota {}
impl Car for Toyota;
func doCivicDance(toyota Toyota) int {
  return 7;
}
exported func main() int {
  x Car = Toyota();
  return doCivicDance(^x);
}
"#;
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let main = monouts.lookup_function_by_str("main");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::Upcast(UpcastIE {
            inner_expr: ExpressionIE::FunctionCall(FunctionCallIE {
                callable: PrototypeI {
                    id: IdI {
                        local_name: INameI::FunctionNameIX(FunctionNameIX {
                            template: FunctionTemplateNameI { human_name: StrI("Toyota"), .. },
                            ..
                        }),
                        ..
                    },
                    ..
                },
                ..
            }),
            result: KindIT::InterfaceIT(_),
            ..
        })) => Some(())
    );
}

/// An array element read yields a borrow of the element's storage: `a.3` on a `[]int`
/// instantiates to a RuntimeSizedArrayLookupIE whose result is `BorrowRefIT<int>` — the same
/// lookup-yields-borrow invariant as locals and members, on arrays.
#[test]
fn array_element_read_yields_borrow() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r#"
import v.builtins.arrays.*;
import v.builtins.arith.*;
import v.builtins.drop.*;

struct Lam {}
func __call(lam &Lam, i int) int { return __copy_prim(&i); }

exported func main() int {
  a = []int(10, Lam());
  return __copy_prim(&a.3);
}
"#;
    let mut compile = test_with_array_builtins(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    let monouts = compile.get_monouts();
    let main = monouts.lookup_function_by_str("main");
    collect_only_inode!(
        NodeRefI::FunctionDefinition(main),
        NodeRefI::Expression(ExpressionIE::RuntimeSizedArrayLookup(RuntimeSizedArrayLookupIE {
            result: &BorrowRefIT { inner: KindIT::IntIT(IntIT { bits: 32 }) },
            ..
        })) => Some(())
    );
}

#[test]
#[ignore = "share-blanket / bound-resolution not yet honest for clone-of-borrow-in-generics; needs `&&T` structural distinctness or primitive-borrow flip"]
fn nested_anonymous_substruct_captures_outer() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r"
interface IF<R, P> {
  func __call(virtual this &IF<R, P>, p P) R;
}
exported func main() int {
  inner = IF<bool, int>((it) => { true });
  outer = IF<bool, int>((it) => { inner(it) });
  return 0;
}
";
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    compile.get_monouts();
}
