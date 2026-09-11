#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use crate::integration_tests::tests::run_compilation::test;
use crate::integration_tests::tests::run_compilation::test_without_borrow_check;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::typing_interner::TypingInterner;
use crate::testvm::von::IVonData;
use crate::testvm::von::VonInt;
pub struct OptTests;

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn test_empty_and_get_for_some() {
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
        // TSUGAR: opt.get() returns &int
        r"
import v.builtins.opt.*;
import v.builtins.box.*;

exported func main() int {
  opt Box<dyn OptI<int>> = Box<dyn OptI<int>>(Box<SomeI<int>>(SomeI<int>(9)));
  return if (opt.isEmpty()) { 0 }
    else { __copy_prim(&opt.get()) };
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn test_empty_and_get_for_none() {
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
        // TSUGAR: opt.get() returns &int
        r"
import v.builtins.opt.*;
import v.builtins.box.*;

exported func main() int {
  opt Box<dyn OptI<int>> = Box<dyn OptI<int>>(Box<NoneI<int>>(NoneI<int>()));
  return if (opt.isEmpty()) { 0 }
    else { __copy_prim(&opt.get()) };
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 0 }) => {}
        other => panic!("expected VonInt(0), got {:?}", other),
    }
}

// VINTERFACE: parked while interfaces migrate to the enum representation; re-enable after the enum work lands.
#[ignore = "VINTERFACE: re-enable after the enum work"]
#[test]
fn test_empty_and_get_for_borrow() {
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
        // TSUGAR: bork.borrowGet().fuel is &int
        r"
// This is the same as the one in optutils.vale, just named differently,
// so its easier to debug.
func borrowGet<T>(opt &SomeI<T>) &T { &opt.value }

struct Spaceship { fuel int; }
exported func main() int {
  s = Spaceship(42);
  bork = SomeI<&Spaceship>(&s);
  return __copy_prim(&bork.borrowGet().fuel);
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}


