use crate::end_to_end_tests::{assert_replay_test, programs_dir};
use crate::wasi_skip;

fn run(dir_rel: &str, first: i32, repeated: i32) {
    wasi_skip!("record/replay reads files via fopen/fread; wasi-libc returns ENOTTY here, needs separate investigation");
    let dir = programs_dir().join(dir_rel);
    assert_replay_test(&dir, &[], first, repeated);
}

#[test] fn simpleexternreturn_replay()        { run("programs/externs/simpleexternreturn", 42, 84); }
#[test] fn simpleexternparam_replay()         { run("programs/externs/simpleexternparam", 42, 84); }
#[test] fn structimmreturnextern_replay()     { run("programs/externs/structimmreturnextern", 42, 84); }
#[test] fn structimmreturnexport_replay()     { run("programs/externs/structimmreturnexport", 42, 84); }
#[test] fn structimmparamextern_replay()      { run("programs/externs/structimmparamextern", 42, 84); }
#[test] fn structimmparamexport_replay()      { run("programs/externs/structimmparamexport", 42, 84); }
#[test] fn structimmparamdeepextern_replay()  { run("programs/externs/structimmparamdeepextern", 42, 84); }
#[test] fn structimmparamdeepexport_replay()  { run("programs/externs/structimmparamdeepexport", 42, 84); }
#[test] fn rsaimmreturnextern_replay()        { run("programs/externs/rsaimmreturnextern", 42, 84); }
#[test] fn rsaimmparamextern_replay()         { run("programs/externs/rsaimmparamextern", 10, 20); }
#[test] fn rsaimmparamexport_replay()         { run("programs/externs/rsaimmparamexport", 10, 20); }
#[test] fn rsaimmparamdeepextern_replay()     { run("programs/externs/rsaimmparamdeepextern", 20, 40); }
#[test] fn rsaimmparamdeepexport_replay()     { run("programs/externs/rsaimmparamdeepexport", 42, 84); }
#[test] fn ssaimmparamextern_replay()         { run("programs/externs/ssaimmparamextern", 42, 84); }
#[test] fn ssaimmparamexport_replay()         { run("programs/externs/ssaimmparamexport", 42, 84); }
#[test] fn ssaimmreturnextern_replay()        { run("programs/externs/ssaimmreturnextern", 42, 84); }
#[test] fn ssaimmparamdeepextern_replay()     { run("programs/externs/ssaimmparamdeepextern", 42, 84); }
#[test] fn ssaimmparamdeepexport_replay()     { run("programs/externs/ssaimmparamdeepexport", 42, 84); }
#[test] fn strreturnexport_replay()           { run("programs/externs/strreturnexport", 6, 12); }
#[test] fn strlenextern_replay()              { run("programs/externs/strlenextern", 11, 22); }
