use crate::end_to_end_tests::{assert_compile_and_run_with_c, compile_program, programs_dir};

fn run(dir_rel: &str, expected: i32) {
    let dir = programs_dir().join(dir_rel);
    // `native/test.c` is auto-discovered by the Frontend-driven walker in
    // pass_manager::build; no need to pass it via extra_c.
    assert_compile_and_run_with_c(&dir, &[], expected);
}

// --- Non-shared FFI ---

#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfacemutreturnexport() { run("programs/externs/interfacemutreturnexport", 42); }
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfacemutparamexport()  { run("programs/externs/interfacemutparamexport", 42); }
#[test]
#[ignore = "deferred: share (str field — borrowed str reaches the share region's translateType, which asserts ShareRef)"]
fn structmutreturnexport()    { run("programs/externs/structmutreturnexport", 42); }
#[test]
fn structmutparamexport()     { run("programs/externs/structmutparamexport", 42); }
// structmutparamdeepexport moved to a typing-pass test
// (typing::test::compiler_tests::reports_when_exported_struct_depends_on_non_exported_member):
// it asserts the export-transitivity error, which belongs at the typing level, not e2e.
#[test]
#[ignore = "deferred: runtime-sized array (RSA) — standing order to defer RSA"]
fn rsamutparamexport()        { run("programs/externs/rsamutparamexport", 10); }
#[test]
#[ignore = "deferred: runtime-sized array (RSA) — standing order to defer RSA"]
fn rsamutreturnexport()       { run("programs/externs/rsamutreturnexport", 42); }
#[test]
fn ssamutparamexport()        { run("programs/externs/ssamutparamexport", 10); }
#[test]
#[ignore = "deferred: SSA across the C extern boundary by value — needs by-value-SSA compile error + struct-field-SSA export"]
fn ssamutreturnexport()       { run("programs/externs/ssamutreturnexport", 42); }

// --- Shared FFI ---

// Extern/export roundtrips, by kind.
#[test]
fn simpleexternreturn()        { run("programs/externs/simpleexternreturn", 42); }
#[test]
fn simpleexternparam()         { run("programs/externs/simpleexternparam", 42); }
#[test]
#[ignore = "deferred: share"]
fn structimmreturnextern()     { run("programs/externs/structimmreturnextern", 42); }
#[test]
#[ignore = "deferred: share"]
fn structimmreturnexport()     { run("programs/externs/structimmreturnexport", 42); }
#[test]
#[ignore = "deferred: share"]
fn structimmparamextern()      { run("programs/externs/structimmparamextern", 42); }
#[test]
#[ignore = "deferred: share"]
fn structimmparamexport()      { run("programs/externs/structimmparamexport", 42); }
#[test]
#[ignore = "deferred: share"]
fn structimmparamdeepextern()  { run("programs/externs/structimmparamdeepextern", 42); }
#[test]
#[ignore = "deferred: share"]
fn structimmparamdeepexport()  { run("programs/externs/structimmparamdeepexport", 42); }
#[test]
#[ignore = "deferred: share"]
fn strreturnexport()           { run("programs/externs/strreturnexport", 6); }
#[test]
#[ignore = "deferred: share"]
fn strlenextern()              { run("programs/externs/strlenextern", 11); }

// Interfaces (incl. Vale-side dispatch variants).
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfaceimmparamextern_vale_dispatch()     { run("programs/externs/interfaceimmparamextern_vale_dispatch", 42); }
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfaceimmparamextern()                   { run("programs/externs/interfaceimmparamextern", 42); }
// The `_owned` variant: C discharges the moved-in arg with an explicit
// `_dealias` (per @FRMACZ) rather than passing it onward.
#[test]
#[ignore = "deferred: immutable-interface override dispatch (share) — &Firefly matches neither &IShip nor Firefly"]
fn interfaceimmparamextern_owned()             { run("programs/externs/interfaceimmparamextern_owned", 42); }
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfaceimmparamdeepextern_vale_dispatch() { run("programs/externs/interfaceimmparamdeepextern_vale_dispatch", 42); }
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfaceimmparamdeepextern()               { run("programs/externs/interfaceimmparamdeepextern", 42); }
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfaceimmparamexport()                   { run("programs/externs/interfaceimmparamexport", 42); }
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfaceimmparamdeepexport()               { run("programs/externs/interfaceimmparamdeepexport", 42); }
#[test]
#[ignore = "deferred: immutable-interface override dispatch (share) — &Firefly matches neither &IShip nor Firefly"]
fn interfaceimmreturnextern()                  { run("programs/externs/interfaceimmreturnextern", 42); }
#[test]
#[ignore = "deferred: immutable-interface override dispatch (share) — &Firefly matches neither &IShip nor Firefly"]
fn interfaceimmreturnexport()                  { run("programs/externs/interfaceimmreturnexport", 42); }

// Feature-targeted fixtures (each isolates one auto-gen emitter family).
#[test]
#[ignore = "deferred: share"]
fn feature_alias_dealias()      { run("programs/externs/feature_alias_dealias", 42); }
#[test]
#[ignore = "deferred: share"]
fn feature_ref_eq()             { run("programs/externs/feature_ref_eq", 42); }
#[test]
#[ignore = "deferred: share"]
fn feature_field_getters()      { run("programs/externs/feature_field_getters", 42); }
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn feature_interface_dispatch() { run("programs/externs/feature_interface_dispatch", 42); }
#[test]
#[ignore = "deferred: share"]
fn feature_str_read()           { run("programs/externs/feature_str_read", 42); }
#[test]
#[ignore = "deferred: runtime-sized array (RSA) — standing order to defer RSA"]
fn feature_arr_read_rsa()       { run("programs/externs/feature_arr_read_rsa", 42); }
#[test]
#[ignore = "deferred: SSA across the C extern boundary by value — needs by-value-SSA compile error + struct-field-SSA export"]
fn feature_arr_read_ssa()       { run("programs/externs/feature_arr_read_ssa", 42); }

// RC-correctness fixtures.
#[test]
#[ignore = "deferred: share"]
fn structimm_roundtrip()          { run("programs/externs/structimm_roundtrip", 42); }
#[test]
#[ignore = "deferred: share"]
fn structimm_alias()              { run("programs/externs/structimm_alias", 42); }
#[test]
#[ignore = "deferred: share"]
fn str_empty()                    { run("programs/externs/str_empty", 42); }
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfaceimm_single_variant()  { run("programs/externs/interfaceimm_single_variant", 42); }

// __vbi_ string intrinsics fed from extern-returned primitives.
#[test]
#[ignore = "deferred: share"]
fn stradd_fromextern()      { run("programs/externs/stradd_fromextern", 4); }
#[test]
#[ignore = "deferred: share"]
fn substring_fromextern()   { run("programs/externs/substring_fromextern", 1); }
#[test]
#[ignore = "deferred: share"]
fn casti32str_fromextern()  { run("programs/externs/casti32str_fromextern", 12); }

// Nested share-ref (a str inside a struct) crossing the boundary.
// runNumber 1: id=42, len("hello")=5 -> 47.
#[test]
#[ignore = "deferred: share"]
fn structimm_with_str_return()  { run("programs/externs/structimm_with_str_return", 47); }
// Two such calls in one run: (1+5)+(2+5)=13.
#[test]
#[ignore = "deferred: share"]
fn structimm_with_str_return_twice()  { run("programs/externs/structimm_with_str_return_twice", 13); }

// --- Misc ---

// getMainArg: drives argv directly (not an FFI roundtrip). The harness passes
// "hello" as argv[1], so the program returns len("hello") = 5.
#[test]
#[ignore = "deferred: share"]
fn getmainarg_basic() {
    let dir = programs_dir().join("programs/externs/getmainarg_basic");
    let cp = compile_program(&dir, &[], |_| {});
    let r = cp.run(&["hello"]);
    assert_eq!(r.exit_code, 5, "stdout={:?} stderr={:?}", r.stdout, r.stderr);
}
