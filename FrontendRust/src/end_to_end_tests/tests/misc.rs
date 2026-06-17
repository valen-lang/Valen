#![allow(non_snake_case)]

use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn mutswaplocals()          { assert_compile_and_run(&p("programs/mutswaplocals.vale"), 42); }
#[test] fn restackify()             { assert_compile_and_run(&p("programs/restackify.vale"), 42); }
#[test] fn destructure_restackify() { assert_compile_and_run(&p("programs/destructure_restackify.vale"), 42); }
#[test] fn loop_restackify()        { assert_compile_and_run(&p("programs/loop_restackify.vale"), 42); }
#[test] fn mutlocal()               { assert_compile_and_run(&p("programs/mutlocal.vale"), 42); }
#[test] fn constraintRef()          { assert_compile_and_run(&p("programs/constraintRef.vale"), 8); }
#[test] fn unstackifyret()          { assert_compile_and_run(&p("programs/unstackifyret.vale"), 42); }
#[test] fn unreachablemoot()        { assert_compile_and_run(&p("programs/unreachablemoot.vale"), 42); }
#[test] fn panic()                  { assert_compile_and_run(&p("programs/panic.vale"), 1); }
#[test] fn panicnot()               { assert_compile_and_run(&p("programs/panicnot.vale"), 42); }
#[test] fn nestedblocks()           { assert_compile_and_run(&p("programs/nestedblocks.vale"), 42); }
