#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use crate::collect_only_tnode;
use crate::instantiating::ast::ast::PrototypeI;
use crate::instantiating::ast::expressions::ExpressionIE;
use crate::instantiating::ast::expressions::FunctionCallIE;
use crate::instantiating::ast::expressions::LetNormalIE;
use crate::instantiating::ast::names::FunctionNameIX;
use crate::instantiating::ast::names::FunctionTemplateNameI;
use crate::instantiating::ast::names::IdI;
use crate::instantiating::ast::names::IInterfaceNameI;
use crate::instantiating::ast::names::INameI;
use crate::instantiating::ast::templata::ITemplataI;
use crate::instantiating::ast::templata::KindTemplataI;
use crate::instantiating::ast::types::BorrowRefIT;
use crate::instantiating::ast::types::InterfaceIT;
use crate::instantiating::ast::types::KindIT;
use crate::instantiating::collector::only_in_function;
use crate::instantiating::collector::NodeRefI;
use crate::integration_tests::tests::run_compilation::test;
use crate::integration_tests::tests::run_compilation::test_no_builtins;
use crate::integration_tests::tests::run_compilation::test_without_borrow_check;
use crate::interner::StrI;
use crate::typing::ast::ast::AbstractT;
use crate::typing::ast::ast::ParameterT;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::names::names::IdT;
use crate::typing::names::names::InterfaceNameT;
use crate::typing::names::names::InterfaceTemplateNameT;
use crate::typing::types::types::InterfaceTT;
use crate::typing::ast::expressions::ExpressionTE;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::ast::expressions::LetNormalTE;
use crate::typing::names::names::FunctionNameT;
use crate::typing::names::names::FunctionTemplateNameT;
use crate::typing::names::names::ICitizenNameT;
use crate::typing::names::names::INameT;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::templata::templata::KindTemplataT;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::types::types::BorrowRefT;
use crate::typing::types::types::IntT;
use crate::typing::types::types::KindT;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::load_expected;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::CodeLocationS;
use crate::testvm::von::IVonData;
use crate::testvm::von::VonInt;
use crate::testvm::von::VonStr;
use std::marker::PhantomData;

pub struct VirtualTests;

// VINTERFACE: parked while interfaces migrate to the enum representation; re-enable after the enum work lands.
#[ignore = "VINTERFACE: re-enable after the enum work"]
#[test]
fn simple_program_containing_a_virtual_function() {
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
interface I  {}
func doThing(virtual i I) int { return 4; }
func main(i I) int {
  return doThing(^i);
}
",
    );
    let coutputs = compile.expect_compiler_outputs();
    assert_eq!(coutputs.get_all_user_functions().len(), 2);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type, KindT::Int(IntT::I32));
    let do_thing = coutputs.lookup_function_by_str("doThing");
    assert_eq!(do_thing.header.return_type, KindT::Int(IntT::I32));
    match do_thing.header.params {
        [ParameterT {
            virtuality: Some(AbstractT),
            tyype: KindT::Interface(InterfaceTT {
                id: IdT { local_name: INameT::Interface(InterfaceNameT {
                    template: InterfaceTemplateNameT { human_namee: StrI("I"), .. }, .. }), .. },
                ..
            }),
            ..
        }] => {}
        other => panic!("expected doThing to take one abstract param of interface I, got {:?}", other),
    }
}

// VINTERFACE: parked while interfaces migrate to the enum representation; re-enable after the enum work lands.
#[ignore = "VINTERFACE: re-enable after the enum work"]
#[test]
fn can_call_virtual_function() {
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
interface I  {}
func doThing(virtual i I) int { return 4; }
func main(i I) int {
  return doThing(^i);
}
",
    );
    let coutputs = compile.expect_compiler_outputs();
    assert_eq!(coutputs.get_all_user_functions().len(), 2);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type, KindT::Int(IntT::I32));
    let do_thing = coutputs.lookup_function_by_str("doThing");
    assert_eq!(do_thing.header.return_type, KindT::Int(IntT::I32));
    match do_thing.header.params {
        [ParameterT {
            virtuality: Some(AbstractT),
            tyype: KindT::Interface(InterfaceTT {
                id: IdT { local_name: INameT::Interface(InterfaceNameT {
                    template: InterfaceTemplateNameT { human_namee: StrI("I"), .. }, .. }), .. },
                ..
            }),
            ..
        }] => {}
        other => panic!("expected doThing to take one abstract param of interface I, got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn owning_interface() {
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
import v.builtins.opt.*;
import v.builtins.box.*;
exported func main() int {
  x Box<dyn OptI<int>> = Box<dyn OptI<int>>(Box<SomeI<int>>(SomeI<int>(7)));
  return 7;
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
        other => panic!("Expected VonInt(7), got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn simple_override_with_param_and_bound() {
    // This is the Serenity case in ROWC.
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
import v.builtins.drop.*;

interface ISpaceship<E, F, G> { }
abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
    where func drop(X)void;

struct Serenity<A, B, C> { }
impl<H, I, J> ISpaceship<H, I, J> for Serenity<H, I, J>;
func launch<M, N, P>(self &Serenity<M, N, P>, bork M)
    where func drop(M)void { }

exported func main() {
  ship ISpaceship<int, bool, str> = Serenity<int, bool, str>();
  ship.launch(7);
}
",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn struct_with_different_ordered_runes() {
    // This is the Firefly case in ROWC.
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
import v.builtins.drop.*;

interface ISpaceship<E, F, G> { }
abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
    where func drop(X)void;

struct Firefly<A, B, C> { }
impl<H, I, J> ISpaceship<H, I, J> for Firefly<J, I, H>;
func launch<M, N, P>(self &Firefly<M, N, P>, bork P)
    where func drop(P)void { }

exported func main() {
  ship ISpaceship<int, bool, str> = Firefly<str, bool, int>();
  ship.launch(7);
}
",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn struct_with_less_generic_params_than_interface() {
    // This is the Raza case in ROWC.
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
import v.builtins.drop.*;

interface ISpaceship<E, F, G> { }
abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
    where func drop(X)void;

struct Raza<B, C> { }
impl<I, J> ISpaceship<int, I, J> for Raza<I, J>;
func launch<N, P>(self &Raza<N, P>, bork int) { }

exported func main() {
  ship ISpaceship<int, bool, str> = Raza<bool, str>();
  ship.launch(7);
}
",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn struct_with_more_generic_params_than_interface() {
    // This is the Milano case in ROWC.
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
import v.builtins.drop.*;

interface ISpaceship<E, F, G> { }
abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
    where func drop(X)void;

struct Milano<A, B, C, D> { }
impl<H, I, J, K> ISpaceship<H, I, J> for Milano<H, I, J, K>;
func launch<H, I, J, K>(self &Milano<H, I, J, K>, bork H) where func drop(H)void { }

exported func main() {
  ship ISpaceship<int, bool, str> = Milano<int, bool, str, float>();
  ship.launch(7);
}
",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn struct_repeating_generic_params_for_interface() {
    // This is the Enterprise case in ROWC.
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
import v.builtins.drop.*;

interface ISpaceship<E, F, G> { }
abstract func launch<X, Y, Z>(virtual self &ISpaceship<X, Y, Z>, bork X)
    where func drop(X)void;

struct Enterprise<A> { }
impl<H> ISpaceship<H, H, H> for Enterprise<H>;
func launch<H>(self &Enterprise<H>, bork H) where func drop(H)void { }

exported func main() {
  ship ISpaceship<int, int, int> = Enterprise<int>();
  ship.launch(7);
}
",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn imm_interface() {
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
    let source = load_expected("programs/virtuals/interfaceimm.vale");
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
}

// VINTERFACE: parked while interfaces migrate to the enum representation; re-enable after the enum work lands.
#[ignore = "VINTERFACE: re-enable after the enum work"]
#[test]
fn can_call_interface_envs_function_from_outside() {
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
interface I {
  func doThing(virtual i I) int;
}
func main(i I) int {
  return doThing(^i);
}
",
    );
    let coutputs = compile.expect_compiler_outputs();
    assert_eq!(coutputs.get_all_user_functions().len(), 1);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type, KindT::Int(IntT::I32));
    let do_thing = coutputs.lookup_function_by_str("doThing");
    assert_eq!(do_thing.header.return_type, KindT::Int(IntT::I32));
    match do_thing.header.params {
        [ParameterT {
            virtuality: Some(AbstractT),
            tyype: KindT::Interface(InterfaceTT {
                id: IdT { local_name: INameT::Interface(InterfaceNameT {
                    template: InterfaceTemplateNameT { human_namee: StrI("I"), .. }, .. }), .. },
                ..
            }),
            ..
        }] => {}
        other => panic!("expected doThing to take one abstract param of interface I, got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn interface_with_method_with_param_of_substruct() {
    unimplemented!(); // ZONION-deferred: needs get_hamuts harness method
    /*
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
struct List<T> { }

interface SectionMember {}
struct Header {}
impl SectionMember for Header;
abstract func collectHeaders2(header &List<&Header>, virtual this &SectionMember);
func collectHeaders2(header &List<&Header>, this &Header) { }
",
    );
    let _coutputs = compile.get_hamuts();
    */
}

#[test]
fn feeding_instantiation_bounds_for_something_created_in_same_function() {
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
#!DeriveStructDrop
struct Spork<T, Y>
where func splork(Y)void {
  lam Y;
}

func bork<T, Y>(
  self &Spork<T, Y> // It had trouble here finding the bound for splork
) { }

func splork(x int) {}

exported func main() int {
  f = Spork<int>(42);
  f.bork(); // We should be feeding in Spork's instantiation bounds here for the params' reachables?
  [z] = ^f;
  return z;
}
",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn generic_interface_forwarder_with_bound() {
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
#!DeriveInterfaceDrop
interface Bork<T>
where func threeify(T)T {
  func bork(virtual self &Bork<T>) int;
}

#!DeriveStructDrop
struct BorkForwarder<T, Lam>
where func drop(Lam)void, func __call(&Lam)T, func threeify(T)T {
  lam Lam;
}

impl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;

func bork<T, Lam>(self &BorkForwarder<T, Lam>) T {
  return (self.lam)().threeify();
}

func threeify(x int) int { 3 }

exported func main() int {
  f = BorkForwarder<int>({ 7 });
  z = f.bork();
  [_] = ^f;
  return z;
}
",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn generic_interface_forwarder_with_drop_bound() {
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
interface Bork<T>
where func threeify(T)T {
  func bork(virtual self &Bork<T>) int;
}

struct BorkForwarder<T, Lam>
where func drop(Lam)void, func __call(&Lam)T, func threeify(T)T {
  lam Lam;
}

impl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;

func bork<T, Lam>(self &BorkForwarder<T, Lam>) T {
  return (self.lam)().threeify();
}

func threeify(x int) int { 3 }

exported func main() int {
  f = BorkForwarder<int>({ 7 });
  return f.bork();
}
",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn open_interface_constructor() {
    unimplemented!(); // ZONION-deferred: needs get_hamuts harness method
    /*
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
interface Bipedal {
  func hop(virtual s &Bipedal) int;
}

func hopscotch(s &Bipedal) int {
  s.hop();
  return s.hop();
}

exported func main() int {
   x = Bipedal({ 3 });
  // x is an unnamed substruct which implements Bipedal.

  return hopscotch(&x);
}
",
    );
    let _coutputs = compile.get_hamuts();
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 3 }) => {}
        other => panic!("Expected VonInt(3), got {:?}", other),
    }
    */
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn open_interface_constructor_multiple_methods() {
    unimplemented!(); // ZONION-deferred: needs get_hamuts harness method
    /*
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
interface Bipedal {
  func hop(virtual s &Bipedal) int;
  func skip(virtual s &Bipedal) int;
}

struct Human {  }
func hop(s &Human) int { return 7; }
func skip(s &Human) int { return 9; }
impl Bipedal for Human;

func hopscotch(s &Bipedal) int {
  s.hop();
  s.skip();
  return s.hop();
}

exported func main() int {
   x = Bipedal({ 3 }, { 5 });
  // x is an unnamed substruct which implements Bipedal.

  return hopscotch(&x);
}
",
    );
    let _coutputs = compile.get_hamuts();
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 3 }) => {}
        other => panic!("Expected VonInt(3), got {:?}", other),
    }
    */
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn successful_pointer_downcast_with_as() {
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
    let source = load_expected("programs/downcast/downcastPointerSuccess.vale");
    let mut compile = test_without_borrow_check(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn failed_pointer_downcast_with_as() {
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
    let source = load_expected("programs/downcast/downcastPointerFailed.vale");
    let mut compile = test_without_borrow_check(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let moo = coutputs.lookup_function_by_str("moo");
        let (dest_var, return_type) = collect_only_tnode!(
            NodeRefT::FunctionDefinition(moo),
            NodeRefT::LetNormal(LetNormalTE {
                variable: dest_var,
                expr: ExpressionTE::FunctionCall(FunctionCallTE {
                    callable: PrototypeT {
                        id: IdT { local_name: INameT::Function(FunctionNameT {
                            template: FunctionTemplateNameT { human_name: StrI("try_as"), .. }, .. }), .. },
                        return_type,
                        ..
                    },
                    ..
                }),
                ..
            }) => Some((*dest_var, *return_type))
        );
        assert_eq!(dest_var.tyype, return_type);
        let result_interface = match return_type {
            KindT::Interface(itt) => itt,
            other => panic!("expected the try_as result to be an (owned) Result interface, got {:?}", other),
        };
        let citizen_name = ICitizenNameT::try_from(result_interface.id.local_name).unwrap();
        let &[success_type, fail_type] = citizen_name.template_args() else {
            panic!("expected 2 interface template args (success, fail)")
        };
        assert!(matches!(success_type, ITemplataT::Kind(KindTemplataT { kind: KindT::BorrowRef(_) })));
        assert!(matches!(fail_type, ITemplataT::Kind(KindTemplataT { kind: KindT::BorrowRef(_) })));
    }
    {
        let monouts = compile.get_monouts();
        let moo = monouts.lookup_function_by_str("moo");
        let (dest_var, return_type) = only_in_function(moo, &|node| match node {
            NodeRefI::LetNormal(LetNormalIE {
                variable: dest_var,
                expr: ExpressionIE::FunctionCall(FunctionCallIE {
                    callable: PrototypeI {
                        id: IdI { local_name: INameI::FunctionNameIX(FunctionNameIX {
                            template: FunctionTemplateNameI { human_name: StrI("try_as"), .. }, .. }), .. },
                        return_type,
                        ..
                    },
                    ..
                }),
                ..
            }) => Some((*dest_var, *return_type)),
            _ => None,
        });
        assert_eq!(dest_var.tyype, return_type);
        let result_interface = match return_type {
            KindIT::InterfaceIT(itt) => itt,
            other => panic!("expected the collapsed try_as result to be a Result interface, got {:?}", other),
        };
        let interface_name = IInterfaceNameI::try_from(result_interface.id.local_name).unwrap();
        let &[success_type, fail_type] = interface_name.template_args() else {
            panic!("expected 2 interface template args (success, fail)")
        };
        assert!(matches!(success_type, ITemplataI::Kind(KindTemplataI { kind: KindIT::BorrowRefIT(_) })));
        assert!(matches!(fail_type, ITemplataI::Kind(KindTemplataI { kind: KindIT::BorrowRefIT(_) })));
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn successful_owning_downcast_with_as() {
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
    let source = load_expected("programs/downcast/downcastOwningSuccessful.vale");
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn failed_owning_downcast_with_as() {
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
    let source = load_expected("programs/downcast/downcastOwningFailed.vale");
    let mut compile = test(
        &compilation_bump,
        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn lambda_is_compatible_anonymous_interface() {
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
        r"
import castutils.*;

interface AFunction2<R, P1, P2> {
  func __call(virtual this &AFunction2<R, P1, P2>, a P1, b P2) R;
}
exported func main() str {
  func = AFunction2<str, int, bool>((i, b) => { str(i) + str(b) });
  return func(42, true);
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Str(VonStr { value }) if value == "42true" => {}
        other => panic!("Expected VonStr(\"42true\"), got {:?}", other),
    }
}

