use crate::end_to_end_tests::{assert_replay_test, programs_dir};
use crate::wasi_skip;

fn run(dir_rel: &str, first: i32, repeated: i32) {
    wasi_skip!("record/replay reads files via fopen/fread; wasi-libc returns ENOTTY here, needs separate investigation");
    let dir = programs_dir().join(dir_rel);
    assert_replay_test(&dir, &[], first, repeated);
}

#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn simpleexternreturn_replay()        { run("programs/externs/simpleexternreturn", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn simpleexternparam_replay()         { run("programs/externs/simpleexternparam", 42, 84); }
// imm replay tests — blocked on the OwnInline/OwnHeap split. See vcoord-handoff.md
// "Mission — Replay / FFI design for the own-based world" for the port plan.
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn structimmreturnextern_replay()     { run("programs/externs/structimmreturnextern", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn structimmreturnexport_replay()     { run("programs/externs/structimmreturnexport", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn structimmparamextern_replay()      { run("programs/externs/structimmparamextern", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn structimmparamexport_replay()      { run("programs/externs/structimmparamexport", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn structimmparamdeepextern_replay()  { run("programs/externs/structimmparamdeepextern", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn structimmparamdeepexport_replay()  { run("programs/externs/structimmparamdeepexport", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsaimmreturnextern_replay()        { run("programs/externs/rsaimmreturnextern", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsaimmparamextern_replay()         { run("programs/externs/rsaimmparamextern", 10, 20); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsaimmparamexport_replay()         { run("programs/externs/rsaimmparamexport", 10, 20); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsaimmparamdeepextern_replay()     { run("programs/externs/rsaimmparamdeepextern", 20, 40); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsaimmparamdeepexport_replay()     { run("programs/externs/rsaimmparamdeepexport", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssaimmparamextern_replay()         { run("programs/externs/ssaimmparamextern", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssaimmparamexport_replay()         { run("programs/externs/ssaimmparamexport", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssaimmreturnextern_replay()        { run("programs/externs/ssaimmreturnextern", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssaimmparamdeepextern_replay()     { run("programs/externs/ssaimmparamdeepextern", 42, 84); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssaimmparamdeepexport_replay()     { run("programs/externs/ssaimmparamdeepexport", 42, 84); }
#[test] fn strreturnexport_replay()           { run("programs/externs/strreturnexport", 6, 12); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn strlenextern_replay()              { run("programs/externs/strlenextern", 11, 22); }
