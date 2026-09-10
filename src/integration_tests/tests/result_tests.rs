#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use crate::integration_tests::tests::run_compilation::test;
use crate::integration_tests::tests::run_compilation::test_without_borrow_check;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::testvm::vivem::VmRuntimeErrorV;
use crate::typing::typing_interner::TypingInterner;
use crate::testvm::von::IVonData;
use crate::testvm::von::VonInt;
use crate::testvm::von::VonStr;
pub struct ResultTests;

#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn test_borrow_is_ok_and_expect_for_ok() {
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
        // TSUGAR: result.expect("eh") returns &int; wrap with __copy_prim
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;
import v.builtins.box.*;

exported func main() int {
  result Box<dyn ResultI<int, str>> = Box<dyn ResultI<int, str>>(Box<OkI<int, str>>(OkI<int, str>(42)));
  return if (result.is_ok()) { __copy_prim(&result.expect("eh")) }
    else { panic("wat") };
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}



#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn test_is_err_and_borrow_expect_err_for_err() {
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
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;
import v.builtins.box.*;

exported func main() str {
  result Box<dyn ResultI<int, str>> = Box<dyn ResultI<int, str>>(Box<ErrI<int, str>>(ErrI<int, str>("file not found!")));
  return if (result.is_err()) { result.expect_err("eh") }
    else { panic("fail!") };
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Str(VonStr { value }) if value == "file not found!" => {}
        other => panic!("expected VonStr(\"file not found!\"), got {:?}", other),
    }
}



#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn test_owning_expect() {
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
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;
import v.builtins.box.*;

exported func main() int {
  result Box<dyn ResultI<int, str>> = Box<dyn ResultI<int, str>>(Box<OkI<int, str>>(OkI<int, str>(42)));
  return (^result).expect("eh");
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}



#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn test_owning_expect_err() {
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
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;
import v.builtins.box.*;

exported func main() str {
  result Box<dyn ResultI<int, str>> = Box<dyn ResultI<int, str>>(Box<ErrI<int, str>>(ErrI<int, str>("file not found!")));
  return (^result).expect_err("eh");
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Str(VonStr { value }) if value == "file not found!" => {}
        other => panic!("expected VonStr(\"file not found!\"), got {:?}", other),
    }
}



#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn test_expect_panics_for_err() {
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
        // TSUGAR: result.expect("eh") returns &int; wrap with __copy_prim
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;
import v.builtins.box.*;

exported func main() int {
  result Box<dyn ResultI<int, str>> = Box<dyn ResultI<int, str>>(Box<ErrI<int, str>>(ErrI<int, str>("file not found!")));
  return __copy_prim(&result.expect("eh"));
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        Err(VmRuntimeErrorV::PanicException(_)) => {}
        other => panic!("Expected PanicException, got {:?}", other),
    }
}



#[test]
#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]
fn test_expect_err_panics_for_ok() {
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
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;
import v.builtins.box.*;

exported func main() str {
  result Box<dyn ResultI<int, str>> = Box<dyn ResultI<int, str>>(Box<OkI<int, str>>(OkI<int, str>(73)));
  return result.expect_err("eh");
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        Err(VmRuntimeErrorV::PanicException(_)) => {}
        other => panic!("Expected PanicException, got {:?}", other),
    }
}


