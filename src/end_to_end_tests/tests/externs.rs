use crate::end_to_end_tests::{assert_compile_and_run_with_c, compile_program, programs_dir};

fn run(dir_rel: &str, expected: i32) {
    let dir = programs_dir().join(dir_rel);
    // `native/test.c` is auto-discovered by the Frontend-driven walker in
    // pass_manager::build; no need to pass it via extra_c.
    assert_compile_and_run_with_c(&dir, &[], expected);
}

// --- Non-shared FFI ---

#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfacemutreturnexport() { run("programs/externs/interfacemutreturnexport", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfacemutparamexport()  { run("programs/externs/interfacemutparamexport", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn structmutreturnexport()    { run("programs/externs/structmutreturnexport", 42); }
#[test] #[ignore] // VCOORD: re enable w borrowing
fn structmutparamexport()     { run("programs/externs/structmutparamexport", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn structmutparamdeepexport() { run("programs/externs/structmutparamdeepexport", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn rsamutparamexport()        { run("programs/externs/rsamutparamexport", 10); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn rsamutreturnexport()       { run("programs/externs/rsamutreturnexport", 42); }
#[test] #[ignore] // VCOORD: re enable w borrowing
fn ssamutparamexport()        { run("programs/externs/ssamutparamexport", 10); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn ssamutreturnexport()       { run("programs/externs/ssamutreturnexport", 42); }

// --- Shared FFI ---

// Extern/export roundtrips, by kind.
#[test] #[ignore] // VCOORD: re enable w borrowing
fn simpleexternreturn()        { run("programs/externs/simpleexternreturn", 42); }
#[test] #[ignore] // VCOORD: re enable w borrowing
fn simpleexternparam()         { run("programs/externs/simpleexternparam", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn structimmreturnextern()     { run("programs/externs/structimmreturnextern", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn structimmreturnexport()     { run("programs/externs/structimmreturnexport", 42); }
#[test]
#[ignore] // ZCOORD: re-enable with onion
fn structimmparamextern()      { run("programs/externs/structimmparamextern", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn structimmparamexport()      { run("programs/externs/structimmparamexport", 42); }
#[test]
#[ignore] // ZCOORD: re-enable with onion
fn structimmparamdeepextern()  { run("programs/externs/structimmparamdeepextern", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn structimmparamdeepexport()  { run("programs/externs/structimmparamdeepexport", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn strreturnexport()           { run("programs/externs/strreturnexport", 6); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn strlenextern()              { run("programs/externs/strlenextern", 11); }

// Interfaces (incl. Vale-side dispatch variants).
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimmparamextern_vale_dispatch()     { run("programs/externs/interfaceimmparamextern_vale_dispatch", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimmparamextern()                   { run("programs/externs/interfaceimmparamextern", 42); }
// The `_owned` variant: C discharges the moved-in arg with an explicit
// `_dealias` (per @FRMACZ) rather than passing it onward.
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimmparamextern_owned()             { run("programs/externs/interfaceimmparamextern_owned", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimmparamdeepextern_vale_dispatch() { run("programs/externs/interfaceimmparamdeepextern_vale_dispatch", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimmparamdeepextern()               { run("programs/externs/interfaceimmparamdeepextern", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimmparamexport()                   { run("programs/externs/interfaceimmparamexport", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimmparamdeepexport()               { run("programs/externs/interfaceimmparamdeepexport", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimmreturnextern()                  { run("programs/externs/interfaceimmreturnextern", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimmreturnexport()                  { run("programs/externs/interfaceimmreturnexport", 42); }

// Feature-targeted fixtures (each isolates one auto-gen emitter family).
#[test]
#[ignore] // ZCOORD: re-enable with onion
fn feature_alias_dealias()      { run("programs/externs/feature_alias_dealias", 42); }
#[test]
#[ignore] // ZCOORD: re-enable with onion
fn feature_ref_eq()             { run("programs/externs/feature_ref_eq", 42); }
#[test]
#[ignore] // ZCOORD: re-enable with onion
fn feature_field_getters()      { run("programs/externs/feature_field_getters", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn feature_interface_dispatch() { run("programs/externs/feature_interface_dispatch", 42); }
#[test]
#[ignore] // ZCOORD: re-enable with onion
fn feature_str_read()           { run("programs/externs/feature_str_read", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn feature_arr_read_rsa()       { run("programs/externs/feature_arr_read_rsa", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn feature_arr_read_ssa()       { run("programs/externs/feature_arr_read_ssa", 42); }

// RC-correctness fixtures.
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn structimm_roundtrip()          { run("programs/externs/structimm_roundtrip", 42); }
#[test]
#[ignore] // ZCOORD: re-enable with onion
fn structimm_alias()              { run("programs/externs/structimm_alias", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn str_empty()                    { run("programs/externs/str_empty", 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn interfaceimm_single_variant()  { run("programs/externs/interfaceimm_single_variant", 42); }

// __vbi_ string intrinsics fed from extern-returned primitives.
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn stradd_fromextern()      { run("programs/externs/stradd_fromextern", 4); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn substring_fromextern()   { run("programs/externs/substring_fromextern", 1); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn casti32str_fromextern()  { run("programs/externs/casti32str_fromextern", 12); }

// Nested share-ref (a str inside a struct) crossing the boundary.
// runNumber 1: id=42, len("hello")=5 -> 47.
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn structimm_with_str_return()  { run("programs/externs/structimm_with_str_return", 47); }
// Two such calls in one run: (1+5)+(2+5)=13.
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn structimm_with_str_return_twice()  { run("programs/externs/structimm_with_str_return_twice", 13); }

// --- Misc ---

// getMainArg: drives argv directly (not an FFI roundtrip). The harness passes
// "hello" as argv[1], so the program returns len("hello") = 5.
#[test]
#[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"]
fn getmainarg_basic() {
    let dir = programs_dir().join("programs/externs/getmainarg_basic");
    let cp = compile_program(&dir, &[], |_| {});
    let r = cp.run(&["hello"]);
    assert_eq!(r.exit_code, 5, "stdout={:?} stderr={:?}", r.stdout, r.stderr);
}
