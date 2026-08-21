#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use crate::integration_tests::tests::run_compilation::test_no_builtins;
use crate::interner::StrI;
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

#[test]
#[ignore] // ZONION: re-enable for onion
fn simple_program_containing_a_virtual_function() {
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
sealed interface I  {}
func doThing(virtual i I) int { return 4; }
func main(i I) int {
  return doThing(^i);
}
",
    );
    let interner = compile.interner;
    let coutputs = compile.expect_compiler_outputs();
    let _keywords_ref = &keywords;

    assert_eq!(coutputs.get_all_user_functions().len(), 2);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type,
        CoordT::new(
            OwnershipT::Own,
            RegionT { region: IRegionT::Default },
            KindT::Int(IntT::I32),
        ));

    let test_tld = PackageCoordinate::test_tld(&parse_arena, &parser_keywords);
    let interface_template = interner.intern_interface_template_name(InterfaceTemplateNameT {
        human_namee: scout_arena.intern_str("I"),
    });
    let interface_name = interner.intern_interface_name(InterfaceNameValT {
        template: interface_template,
        template_args: &[],
    });
    let interface_id = interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Interface(interface_name),
    });
    let interface_tt = interner.intern_interface_tt(InterfaceTTValT { id: *interface_id });
    let i_coord = CoordT::new(
        OwnershipT::Own,
        RegionT { region: IRegionT::Default },
        KindT::Interface(interface_tt),
    );
    let do_thing_template = interner.intern_function_template_name(FunctionTemplateNameT {
        human_name: scout_arena.intern_str("doThing"),
        code_location: CodeLocationS {
            file: scout_arena.intern_file_coordinate(
                scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]),
                "0.vale"),
            offset: 24,
        },
    });
    let do_thing_name = interner.intern_function_name(FunctionNameValT {
        template: do_thing_template,
        template_args: &[],
        parameters: &[i_coord],
    });
    let do_thing_id = interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Function(do_thing_name),
    });
    let do_thing = coutputs.lookup_function_by_signature(
        SignatureT { id: *do_thing_id }).expect("vassertSome");
    assert_eq!(do_thing.header.params[0].virtuality, Some(AbstractT));
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn can_call_virtual_function() {
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
sealed interface I  {}
func doThing(virtual i I) int { return 4; }
func main(i I) int {
  return doThing(^i);
}
",
    );
    let interner = compile.interner;
    let coutputs = compile.expect_compiler_outputs();
    let _keywords_ref = &keywords;

    assert_eq!(coutputs.get_all_user_functions().len(), 2);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type,
        CoordT::new(
            OwnershipT::Own,
            RegionT { region: IRegionT::Default },
            KindT::Int(IntT::I32),
        ));

    let test_tld = PackageCoordinate::test_tld(&parse_arena, &parser_keywords);
    let interface_template = interner.intern_interface_template_name(InterfaceTemplateNameT {
        human_namee: scout_arena.intern_str("I"),
    });
    let interface_name = interner.intern_interface_name(InterfaceNameValT {
        template: interface_template,
        template_args: &[],
    });
    let interface_id = interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Interface(interface_name),
    });
    let interface_tt = interner.intern_interface_tt(InterfaceTTValT { id: *interface_id });
    let i_coord = CoordT::new(
        OwnershipT::Own,
        RegionT { region: IRegionT::Default },
        KindT::Interface(interface_tt),
    );
    let do_thing_template = interner.intern_function_template_name(FunctionTemplateNameT {
        human_name: scout_arena.intern_str("doThing"),
        code_location: CodeLocationS {
            file: scout_arena.intern_file_coordinate(
                scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]),
                "0.vale"),
            offset: 24,
        },
    });
    let do_thing_name = interner.intern_function_name(FunctionNameValT {
        template: do_thing_template,
        template_args: &[],
        parameters: &[i_coord],
    });
    let do_thing_id = interner.intern_id(IdValT {
        package_coord: test_tld, init_steps: &[], local_name: INameT::Function(do_thing_name),
    });
    let do_thing = coutputs.lookup_function_by_signature(
        SignatureT { id: *do_thing_id }).expect("vassertSome");
    assert_eq!(do_thing.header.params[0].virtuality, Some(AbstractT));
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn owning_interface() {
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
import v.builtins.opt.*;
exported func main() int {
  x Opt<int> = Some(7);
  return 7;
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 7 }) => {}
        other => panic!("Expected VonInt(7), got {:?}", other),
    }
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn simple_override_with_param_and_bound() {
    unimplemented!();
    /*
    // This is the Serenity case in ROWC.
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
import v.builtins.drop.*;

sealed interface ISpaceship<E, F, G> { }
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
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn struct_with_different_ordered_runes() {
    unimplemented!();
    /*
    // This is the Firefly case in ROWC.
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
import v.builtins.drop.*;

sealed interface ISpaceship<E, F, G> { }
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
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn struct_with_less_generic_params_than_interface() {
    unimplemented!();
    /*
    // This is the Raza case in ROWC.
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
import v.builtins.drop.*;

sealed interface ISpaceship<E, F, G> { }
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
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn struct_with_more_generic_params_than_interface() {
    unimplemented!();
    /*
    // This is the Milano case in ROWC.
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
import v.builtins.drop.*;

sealed interface ISpaceship<E, F, G> { }
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
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn struct_repeating_generic_params_for_interface() {
    unimplemented!();
    /*
    // This is the Enterprise case in ROWC.
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
import v.builtins.drop.*;

sealed interface ISpaceship<E, F, G> { }
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
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn imm_interface() {
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
    let source = load_expected("programs/virtuals/interfaceimm.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn can_call_interface_envs_function_from_outside() {
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
sealed interface I {
  func doThing(virtual i I) int;
}
func main(i I) int {
  return doThing(^i);
}
",
    );
    let _interner = compile.interner;
    let coutputs = compile.expect_compiler_outputs();

    assert_eq!(coutputs.get_all_user_functions().len(), 1);
    assert_eq!(coutputs.lookup_function_by_str("main").header.return_type,
        CoordT::new(
            OwnershipT::Own,
            RegionT { region: IRegionT::Default },
            KindT::Int(IntT::I32),
        ));

    let do_thing = coutputs.lookup_function_by_str("doThing");
    assert_eq!(do_thing.header.params[0].virtuality, Some(AbstractT));
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn interface_with_method_with_param_of_substruct() {
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
        r"
struct List<T> { }

sealed interface SectionMember {}
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
#[ignore] // ZONION: re-enable for onion
fn feeding_instantiation_bounds_for_something_created_in_same_function() {
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
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn generic_interface_forwarder_with_bound() {
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
        r"
#!DeriveInterfaceDrop
sealed interface Bork<T>
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
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn generic_interface_forwarder_with_drop_bound() {
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
        r"
sealed interface Bork<T>
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
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn open_interface_constructor() {
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
    let mut compile = test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
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
#[ignore] // ZONION: re-enable for onion
fn open_interface_constructor_multiple_methods() {
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
#[ignore] // ZONION: re-enable for onion
fn successful_pointer_downcast_with_as() {
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
    let source = load_expected("programs/downcast/downcastPointerSuccess.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn failed_pointer_downcast_with_as() {
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
    let source = load_expected("programs/downcast/downcastPointerFailed.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
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
                expr: ReferenceExpressionTE::FunctionCall(FunctionCallTE {
                    callable: PrototypeT {
                        id: IdT {
                            local_name: INameT::Function(FunctionNameT {
                                template: FunctionTemplateNameT { human_name: StrI("as"), .. },
                                ..
                            }),
                            ..
                        },
                        ..
                    },
                    return_type,
                    ..
                }),
            }) => Some((*dest_var, *return_type))
        );
        assert!(dest_var.coord() == return_type);
        let citizen_name = ICitizenNameT::try_from(return_type.kind.expect_interface().id.local_name).unwrap();
        let &[success_type, fail_type] = citizen_name.template_args() else { panic!("expected 2 template args") };
        assert!(expect_coord_templata_t(success_type).coord.ownership == OwnershipT::Borrow);
        assert!(expect_coord_templata_t(fail_type).coord.ownership == OwnershipT::Borrow);
    }
    {
        let monouts = compile.get_monouts();
        let moo = monouts.lookup_function_by_str("moo");
        let (dest_var, return_type) = only_in_function(moo, &|node| match node {
            NodeRefI::LetNormal(LetNormalIE {
                variable: dest_var,
                expr: ReferenceExpressionIE::FunctionCall(FunctionCallIE {
                    callable: PrototypeI {
                        id: IdI {
                            local_name: INameI::FunctionNameIX(FunctionNameIX {
                                template: FunctionTemplateNameI { human_name: StrI("as"), .. },
                                ..
                            }),
                            ..
                        },
                        return_type,
                        ..
                    },
                    ..
                }),
                ..
            }) => Some((*dest_var, *return_type)),
            _ => None,
        });
        assert!(dest_var.collapsed_coord() == return_type);
        let interface_id_local_name = IInterfaceNameI::try_from(return_type.kind.expect_interface().id.local_name).unwrap();
        let &[success_type, fail_type] = interface_id_local_name.template_args() else { panic!("expected 2 template args") };
        assert!(expect_coord_templata_i(success_type).coord.ownership == OwnershipI::MutableBorrow);
        assert!(expect_coord_templata_i(fail_type).coord.ownership == OwnershipI::MutableBorrow);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn successful_owning_downcast_with_as() {
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
    let source = load_expected("programs/downcast/downcastOwningSuccessful.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn failed_owning_downcast_with_as() {
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
    let source = load_expected("programs/downcast/downcastOwningFailed.vale");
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        source.as_str(),
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
    */
}

#[test]
#[ignore] // ZONION: re-enable for onion
fn lambda_is_compatible_anonymous_interface() {
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
    */
}

