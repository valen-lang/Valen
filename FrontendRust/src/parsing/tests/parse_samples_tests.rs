/// Integration tests that parse real Vale sample files
/// Mirrors tests from Frontend/ParsingPass/test/dev/vale/parsing/ParseSamplesTests.scala

use crate::parsing::tests::test_parse_utils::*;
use crate::parsing::parse_error_humanizer::ParseErrorHumanizer;
use std::fs;
use std::path::Path;

/// Helper function to parse a Vale file from the samples directory
/// Mirrors parse() in ParseSamplesTests.scala lines 10-19
fn parse_sample_file(relative_path: &str) {
    let base_path = "/Volumes/V/Sylvan/Frontend/Tests/test/main/resources";
    let full_path = Path::new(base_path).join(relative_path);
    
    // Mirrors Tests.loadExpected(path) in Scala line 13
    let code = fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", full_path.display(), e));
    
    // Mirrors ParserTestCompilation.test(interner, keywords, code) in Scala line 14
    // and compilation.getParseds() in Scala lines 15-18
    let result = compile_file(&code);
    
    // Mirrors the match in Scala lines 15-18
    match result {
        Ok(_file) => {
            // Success - test passes
        }
        Err(e) => {
            // Mirrors vfail(ParseErrorHumanizer.humanize(path, code, e.error)) in Scala line 17
            let humanized = ParseErrorHumanizer::humanize(&full_path, &code, &e);
            panic!("Failed to parse {}:\n{}", relative_path, humanized);
        }
    }
}

// Mirrors ParseSamplesTests.scala line 21
#[test]
fn test_optutils_optutils() {
    parse_sample_file("optutils/optutils.vale");
}

// Mirrors ParseSamplesTests.scala line 22
#[test]
fn test_printutils_printutils() {
    parse_sample_file("printutils/printutils.vale");
}

// Mirrors ParseSamplesTests.scala line 23
#[test]
fn test_ioutils_ioutils() {
    parse_sample_file("ioutils/ioutils.vale");
}

// Mirrors ParseSamplesTests.scala line 24
#[test]
fn test_array_indices() {
    parse_sample_file("array/indices/indices.vale");
}

// Mirrors ParseSamplesTests.scala line 25
#[test]
fn test_array_iter() {
    parse_sample_file("array/iter/iter.vale");
}

// Mirrors ParseSamplesTests.scala line 26
#[test]
fn test_array_each() {
    parse_sample_file("array/each/each.vale");
}

// Mirrors ParseSamplesTests.scala line 27
#[test]
fn test_array_drop_into() {
    parse_sample_file("array/drop_into/drop_into.vale");
}

// Mirrors ParseSamplesTests.scala line 28
#[test]
fn test_array_make() {
    parse_sample_file("array/make/make.vale");
}

// Mirrors ParseSamplesTests.scala line 29
#[test]
fn test_array_has() {
    parse_sample_file("array/has/has.vale");
}

// Mirrors ParseSamplesTests.scala line 30
#[test]
fn test_listprintutils() {
    parse_sample_file("listprintutils/listprintutils.vale");
}

// Mirrors ParseSamplesTests.scala line 31
#[test]
fn test_logic() {
    parse_sample_file("logic/logic.vale");
}

// Mirrors ParseSamplesTests.scala line 32
#[test]
fn test_math() {
    parse_sample_file("math/math.vale");
}

// Mirrors ParseSamplesTests.scala line 33
#[test]
fn test_hashmap() {
    parse_sample_file("hashmap/hashmap.vale");
}

// Mirrors ParseSamplesTests.scala line 34
#[test]
fn test_intrange() {
    parse_sample_file("intrange/intrange.vale");
}

// Mirrors ParseSamplesTests.scala line 35
#[test]
fn test_programs_lambdas_doubleclosure() {
    parse_sample_file("programs/lambdas/doubleclosure.vale");
}

// Mirrors ParseSamplesTests.scala line 36
#[test]
fn test_programs_lambdas_mutate() {
    parse_sample_file("programs/lambdas/mutate.vale");
}

// Mirrors ParseSamplesTests.scala line 37
#[test]
fn test_programs_lambdas_lambda() {
    parse_sample_file("programs/lambdas/lambda.vale");
}

// Mirrors ParseSamplesTests.scala line 38
#[test]
fn test_programs_lambdas_lambdamut() {
    parse_sample_file("programs/lambdas/lambdamut.vale");
}

// Mirrors ParseSamplesTests.scala line 39
#[test]
fn test_programs_comparei64() {
    parse_sample_file("programs/comparei64.vale");
}

// Mirrors ParseSamplesTests.scala line 40
#[test]
fn test_programs_unreachablemoot() {
    parse_sample_file("programs/unreachablemoot.vale");
}

// Mirrors ParseSamplesTests.scala line 41
#[test]
fn test_programs_constraint_ref() {
    parse_sample_file("programs/constraintRef.vale");
}

// Mirrors ParseSamplesTests.scala line 42
#[test]
fn test_programs_add64ret() {
    parse_sample_file("programs/add64ret.vale");
}

// Mirrors ParseSamplesTests.scala line 43
#[test]
fn test_programs_concatstrfloat() {
    parse_sample_file("programs/concatstrfloat.vale");
}

// Mirrors ParseSamplesTests.scala line 44
#[test]
fn test_programs_strings_smallstr() {
    parse_sample_file("programs/strings/smallstr.vale");
}

// Mirrors ParseSamplesTests.scala line 45
#[test]
fn test_programs_strings_complex_main() {
    parse_sample_file("programs/strings/complex/main.vale");
}

// Mirrors ParseSamplesTests.scala line 46
#[test]
fn test_programs_strings_stradd() {
    parse_sample_file("programs/strings/stradd.vale");
}

// Mirrors ParseSamplesTests.scala line 47
#[test]
fn test_programs_strings_strprint() {
    parse_sample_file("programs/strings/strprint.vale");
}

// Mirrors ParseSamplesTests.scala line 48
#[test]
fn test_programs_strings_strneq() {
    parse_sample_file("programs/strings/strneq.vale");
}

// Mirrors ParseSamplesTests.scala line 49
#[test]
fn test_programs_strings_i64tostr() {
    parse_sample_file("programs/strings/i64tostr.vale");
}

// Mirrors ParseSamplesTests.scala line 50
#[test]
fn test_programs_strings_inttostr() {
    parse_sample_file("programs/strings/inttostr.vale");
}

// Mirrors ParseSamplesTests.scala line 51
#[test]
fn test_programs_strings_strlen() {
    parse_sample_file("programs/strings/strlen.vale");
}

// Mirrors ParseSamplesTests.scala line 52
#[test]
fn test_programs_externs_structimmparamextern() {
    parse_sample_file("programs/externs/structimmparamextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 53
#[test]
fn test_programs_externs_structimmparamexport() {
    parse_sample_file("programs/externs/structimmparamexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 54
#[test]
fn test_programs_externs_strreturnexport() {
    parse_sample_file("programs/externs/strreturnexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 55
#[test]
fn test_programs_externs_voidreturnextern() {
    parse_sample_file("programs/externs/voidreturnextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 56
#[test]
fn test_programs_externs_voidreturnexport() {
    parse_sample_file("programs/externs/voidreturnexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 57
#[test]
fn test_programs_externs_structmutreturnexport() {
    parse_sample_file("programs/externs/structmutreturnexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 58
#[test]
fn test_programs_externs_structmutparamexport() {
    parse_sample_file("programs/externs/structmutparamexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 59
#[test]
fn test_programs_externs_structimmparamdeepextern() {
    parse_sample_file("programs/externs/structimmparamdeepextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 60
#[test]
fn test_programs_externs_ssaimmparamdeepextern() {
    parse_sample_file("programs/externs/ssaimmparamdeepextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 61
#[test]
fn test_programs_externs_rsaimmparamdeepexport() {
    parse_sample_file("programs/externs/rsaimmparamdeepexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 62
#[test]
fn test_programs_externs_tupleparamextern() {
    parse_sample_file("programs/externs/tupleparamextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 63
#[test]
fn test_programs_externs_ssamutreturnexport() {
    parse_sample_file("programs/externs/ssamutreturnexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 64
#[test]
fn test_programs_externs_structimmparamdeepexport() {
    parse_sample_file("programs/externs/structimmparamdeepexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 65
#[test]
fn test_programs_externs_ssaimmparamdeepexport() {
    parse_sample_file("programs/externs/ssaimmparamdeepexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 66
#[test]
fn test_programs_externs_rsaimmparamdeepextern() {
    parse_sample_file("programs/externs/rsaimmparamdeepextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 67
#[test]
fn test_programs_externs_rsaimmparamextern() {
    parse_sample_file("programs/externs/rsaimmparamextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 68
#[test]
fn test_programs_externs_rsaimmreturnexport() {
    parse_sample_file("programs/externs/rsaimmreturnexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 69
#[test]
fn test_programs_externs_export() {
    parse_sample_file("programs/externs/export.vale");
}

// Mirrors ParseSamplesTests.scala line 70
#[test]
fn test_programs_externs_rsaimmparamexport() {
    parse_sample_file("programs/externs/rsaimmparamexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 71
#[test]
fn test_programs_externs_rsaimmreturnextern() {
    parse_sample_file("programs/externs/rsaimmreturnextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 72
#[test]
fn test_programs_externs_interfaceimmreturnexport() {
    parse_sample_file("programs/externs/interfaceimmreturnexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 73
#[test]
fn test_programs_externs_interfacemutparamexport() {
    parse_sample_file("programs/externs/interfacemutparamexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 74
#[test]
fn test_programs_externs_interfaceimmreturnextern() {
    parse_sample_file("programs/externs/interfaceimmreturnextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 75
#[test]
fn test_programs_externs_strlenextern() {
    parse_sample_file("programs/externs/strlenextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 76
#[test]
fn test_programs_externs_ssamutparamexport() {
    parse_sample_file("programs/externs/ssamutparamexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 77
#[test]
fn test_programs_externs_rsamutreturnexport() {
    parse_sample_file("programs/externs/rsamutreturnexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 78
#[test]
fn test_programs_externs_ssaimmreturnextern() {
    parse_sample_file("programs/externs/ssaimmreturnextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 79
#[test]
fn test_programs_externs_interfaceimmparamdeepextern() {
    parse_sample_file("programs/externs/interfaceimmparamdeepextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 80
#[test]
fn test_programs_externs_ssaimmreturnexport() {
    parse_sample_file("programs/externs/ssaimmreturnexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 81
#[test]
fn test_programs_externs_rsamutparamexport() {
    parse_sample_file("programs/externs/rsamutparamexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 82
#[test]
fn test_programs_externs_interfaceimmparamdeepexport() {
    parse_sample_file("programs/externs/interfaceimmparamdeepexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 83
#[test]
fn test_programs_externs_extern() {
    parse_sample_file("programs/externs/extern.vale");
}

// Mirrors ParseSamplesTests.scala line 84
#[test]
fn test_programs_externs_interfaceimmparamextern() {
    parse_sample_file("programs/externs/interfaceimmparamextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 85
#[test]
fn test_programs_externs_tupleretextern() {
    parse_sample_file("programs/externs/tupleretextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 86
#[test]
fn test_programs_externs_interfaceimmparamexport() {
    parse_sample_file("programs/externs/interfaceimmparamexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 87
#[test]
fn test_programs_externs_structmutparamdeepexport() {
    parse_sample_file("programs/externs/structmutparamdeepexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 88
#[test]
fn test_programs_externs_ssaimmparamextern() {
    parse_sample_file("programs/externs/ssaimmparamextern/test.vale");
}

// Mirrors ParseSamplesTests.scala line 89
#[test]
fn test_programs_externs_interfacemutreturnexport() {
    parse_sample_file("programs/externs/interfacemutreturnexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 90
#[test]
fn test_programs_externs_ssaimmparamexport() {
    parse_sample_file("programs/externs/ssaimmparamexport/test.vale");
}

// Mirrors ParseSamplesTests.scala line 91
#[test]
fn test_programs_weaks_call_weak_self_method_after_drop() {
    parse_sample_file("programs/weaks/callWeakSelfMethodAfterDrop.vale");
}

// Mirrors ParseSamplesTests.scala line 92
#[test]
fn test_programs_weaks_call_weak_self_method_while_live() {
    parse_sample_file("programs/weaks/callWeakSelfMethodWhileLive.vale");
}

// Mirrors ParseSamplesTests.scala line 93
#[test]
fn test_programs_weaks_drop_then_lock_interface() {
    parse_sample_file("programs/weaks/dropThenLockInterface.vale");
}

// Mirrors ParseSamplesTests.scala line 94
#[test]
fn test_programs_weaks_lock_while_live_interface() {
    parse_sample_file("programs/weaks/lockWhileLiveInterface.vale");
}

// Mirrors ParseSamplesTests.scala line 95
#[test]
fn test_programs_weaks_drop_while_locked_struct() {
    parse_sample_file("programs/weaks/dropWhileLockedStruct.vale");
}

// Mirrors ParseSamplesTests.scala line 96
#[test]
fn test_programs_weaks_weak_from_cref_interface() {
    parse_sample_file("programs/weaks/weakFromCRefInterface.vale");
}

// Mirrors ParseSamplesTests.scala line 97
#[test]
fn test_programs_weaks_weak_from_cref_struct() {
    parse_sample_file("programs/weaks/weakFromCRefStruct.vale");
}

// Mirrors ParseSamplesTests.scala line 98
#[test]
fn test_programs_weaks_lock_while_live_struct() {
    parse_sample_file("programs/weaks/lockWhileLiveStruct.vale");
}

// Mirrors ParseSamplesTests.scala line 99
#[test]
fn test_programs_weaks_load_from_weakable() {
    parse_sample_file("programs/weaks/loadFromWeakable.vale");
}

// Mirrors ParseSamplesTests.scala line 100
#[test]
fn test_programs_weaks_drop_while_locked_interface() {
    parse_sample_file("programs/weaks/dropWhileLockedInterface.vale");
}

// Mirrors ParseSamplesTests.scala line 101
#[test]
fn test_programs_weaks_weak_from_local_cref_struct() {
    parse_sample_file("programs/weaks/weakFromLocalCRefStruct.vale");
}

// Mirrors ParseSamplesTests.scala line 102
#[test]
fn test_programs_weaks_weak_from_local_cref_interface() {
    parse_sample_file("programs/weaks/weakFromLocalCRefInterface.vale");
}

// Mirrors ParseSamplesTests.scala line 103
#[test]
fn test_programs_weaks_drop_then_lock_struct() {
    parse_sample_file("programs/weaks/dropThenLockStruct.vale");
}

// Mirrors ParseSamplesTests.scala line 104
#[test]
fn test_programs_readwriteufcs() {
    parse_sample_file("programs/readwriteufcs.vale");
}

// Mirrors ParseSamplesTests.scala line 105
#[test]
fn test_programs_tuples_immtupleaccess() {
    parse_sample_file("programs/tuples/immtupleaccess.vale");
}

// Mirrors ParseSamplesTests.scala line 106
#[test]
fn test_programs_downcast_downcast_pointer_failed() {
    parse_sample_file("programs/downcast/downcastPointerFailed.vale");
}

// Mirrors ParseSamplesTests.scala line 107
#[test]
fn test_programs_downcast_downcast_pointer_success() {
    parse_sample_file("programs/downcast/downcastPointerSuccess.vale");
}

// Mirrors ParseSamplesTests.scala line 108
#[test]
fn test_programs_downcast_downcast_borrow_failed() {
    parse_sample_file("programs/downcast/downcastBorrowFailed.vale");
}

// Mirrors ParseSamplesTests.scala line 109
#[test]
fn test_programs_downcast_downcast_owning_successful() {
    parse_sample_file("programs/downcast/downcastOwningSuccessful.vale");
}

// Mirrors ParseSamplesTests.scala line 110
#[test]
fn test_programs_downcast_downcast_borrow_successful() {
    parse_sample_file("programs/downcast/downcastBorrowSuccessful.vale");
}

// Mirrors ParseSamplesTests.scala line 111
#[test]
fn test_programs_downcast_downcast_owning_failed() {
    parse_sample_file("programs/downcast/downcastOwningFailed.vale");
}

// Mirrors ParseSamplesTests.scala line 112
#[test]
fn test_programs_if_ifnevers() {
    parse_sample_file("programs/if/ifnevers.vale");
}

// Mirrors ParseSamplesTests.scala line 113
#[test]
fn test_programs_if_nestedif() {
    parse_sample_file("programs/if/nestedif.vale");
}

// Mirrors ParseSamplesTests.scala line 114
#[test]
fn test_programs_if_if() {
    parse_sample_file("programs/if/if.vale");
}

// Mirrors ParseSamplesTests.scala line 115
#[test]
fn test_programs_if_upcastif() {
    parse_sample_file("programs/if/upcastif.vale");
}

// Mirrors ParseSamplesTests.scala line 116
#[test]
fn test_programs_if_neverif() {
    parse_sample_file("programs/if/neverif.vale");
}

// Mirrors ParseSamplesTests.scala line 117
#[test]
fn test_programs_virtuals_calling_through_borrow() {
    parse_sample_file("programs/virtuals/callingThroughBorrow.vale");
}

// Mirrors ParseSamplesTests.scala line 118
#[test]
fn test_programs_virtuals_calling() {
    parse_sample_file("programs/virtuals/calling.vale");
}

// Mirrors ParseSamplesTests.scala line 119
#[test]
fn test_programs_virtuals_round() {
    parse_sample_file("programs/virtuals/round.vale");
}

// Mirrors ParseSamplesTests.scala line 120
#[test]
fn test_programs_virtuals_upcasting() {
    parse_sample_file("programs/virtuals/upcasting.vale");
}

// Mirrors ParseSamplesTests.scala line 121
#[test]
fn test_programs_virtuals_interfaceimm() {
    parse_sample_file("programs/virtuals/interfaceimm.vale");
}

// Mirrors ParseSamplesTests.scala line 122
#[test]
fn test_programs_virtuals_ret_upcast() {
    parse_sample_file("programs/virtuals/retUpcast.vale");
}

// Mirrors ParseSamplesTests.scala line 123
#[test]
fn test_programs_virtuals_ordinarylinkedlist() {
    parse_sample_file("programs/virtuals/ordinarylinkedlist.vale");
}

// Mirrors ParseSamplesTests.scala line 124
#[test]
fn test_programs_virtuals_interfacemut() {
    parse_sample_file("programs/virtuals/interfacemut.vale");
}

// Mirrors ParseSamplesTests.scala line 125
#[test]
fn test_programs_genericvirtuals_get_or() {
    parse_sample_file("programs/genericvirtuals/getOr.vale");
}

// Mirrors ParseSamplesTests.scala line 126
#[test]
fn test_programs_genericvirtuals_templatedinterface() {
    parse_sample_file("programs/genericvirtuals/templatedinterface.vale");
}

// Mirrors ParseSamplesTests.scala line 127
#[test]
fn test_programs_genericvirtuals_templatedlinkedlist() {
    parse_sample_file("programs/genericvirtuals/templatedlinkedlist.vale");
}

// Mirrors ParseSamplesTests.scala line 128
#[test]
fn test_programs_genericvirtuals_stamp_multiple_ancestors() {
    parse_sample_file("programs/genericvirtuals/stampMultipleAncestors.vale");
}

// Mirrors ParseSamplesTests.scala line 129
#[test]
fn test_programs_genericvirtuals_foreachlinkedlist() {
    parse_sample_file("programs/genericvirtuals/foreachlinkedlist.vale");
}

// Mirrors ParseSamplesTests.scala line 130
#[test]
fn test_programs_genericvirtuals_map_func() {
    parse_sample_file("programs/genericvirtuals/mapFunc.vale");
}

// Mirrors ParseSamplesTests.scala line 131
#[test]
fn test_programs_genericvirtuals_calling_abstract() {
    parse_sample_file("programs/genericvirtuals/callingAbstract.vale");
}

// Mirrors ParseSamplesTests.scala line 132
#[test]
fn test_programs_genericvirtuals_templatedoption() {
    parse_sample_file("programs/genericvirtuals/templatedoption.vale");
}

// Mirrors ParseSamplesTests.scala line 133
#[test]
fn test_programs_addret() {
    parse_sample_file("programs/addret.vale");
}

// Mirrors ParseSamplesTests.scala line 134
#[test]
fn test_programs_nestedblocks() {
    parse_sample_file("programs/nestedblocks.vale");
}

// Mirrors ParseSamplesTests.scala line 135
#[test]
fn test_programs_panicnot() {
    parse_sample_file("programs/panicnot.vale");
}

// Mirrors ParseSamplesTests.scala line 136
#[test]
fn test_programs_floateq() {
    parse_sample_file("programs/floateq.vale");
}

// Mirrors ParseSamplesTests.scala line 137
#[test]
fn test_programs_roguelike() {
    parse_sample_file("programs/roguelike.vale");
}

// Mirrors ParseSamplesTests.scala line 138
#[test]
fn test_programs_truncate() {
    parse_sample_file("programs/truncate.vale");
}

// Mirrors ParseSamplesTests.scala line 139
#[test]
fn test_programs_multi_unstackify() {
    parse_sample_file("programs/multiUnstackify.vale");
}

// Mirrors ParseSamplesTests.scala line 140
#[test]
fn test_programs_borrow_ref() {
    parse_sample_file("programs/borrowRef.vale");
}

// Mirrors ParseSamplesTests.scala line 141
#[test]
fn test_programs_structs_constructor() {
    parse_sample_file("programs/structs/constructor.vale");
}

// Mirrors ParseSamplesTests.scala line 142
#[test]
fn test_programs_structs_structmutstore() {
    parse_sample_file("programs/structs/structmutstore.vale");
}

// Mirrors ParseSamplesTests.scala line 143
#[test]
fn test_programs_structs_get_member() {
    parse_sample_file("programs/structs/getMember.vale");
}

// Mirrors ParseSamplesTests.scala line 144
#[test]
fn test_programs_structs_structs() {
    parse_sample_file("programs/structs/structs.vale");
}

// Mirrors ParseSamplesTests.scala line 145
#[test]
fn test_programs_structs_mutate() {
    parse_sample_file("programs/structs/mutate.vale");
}

// Mirrors ParseSamplesTests.scala line 146
#[test]
fn test_programs_structs_deadmutstruct() {
    parse_sample_file("programs/structs/deadmutstruct.vale");
}

// Mirrors ParseSamplesTests.scala line 147
#[test]
fn test_programs_structs_structmutstoreinner() {
    parse_sample_file("programs/structs/structmutstoreinner.vale");
}

// Mirrors ParseSamplesTests.scala line 148
#[test]
fn test_programs_structs_bigstructimm() {
    parse_sample_file("programs/structs/bigstructimm.vale");
}

// Mirrors ParseSamplesTests.scala line 149
#[test]
fn test_programs_structs_structmut() {
    parse_sample_file("programs/structs/structmut.vale");
}

// Mirrors ParseSamplesTests.scala line 150
#[test]
fn test_programs_structs_structimm() {
    parse_sample_file("programs/structs/structimm.vale");
}

// Mirrors ParseSamplesTests.scala line 151
#[test]
fn test_programs_structs_memberrefcount() {
    parse_sample_file("programs/structs/memberrefcount.vale");
}

// Mirrors ParseSamplesTests.scala line 152
#[test]
fn test_programs_arrays_ssaimmfromvalues() {
    parse_sample_file("programs/arrays/ssaimmfromvalues.vale");
}

// Mirrors ParseSamplesTests.scala line 153
#[test]
fn test_programs_arrays_rsamutdestroyintocallable() {
    parse_sample_file("programs/arrays/rsamutdestroyintocallable.vale");
}

// Mirrors ParseSamplesTests.scala line 154
#[test]
fn test_programs_arrays_ssamutfromcallable() {
    parse_sample_file("programs/arrays/ssamutfromcallable.vale");
}

// Mirrors ParseSamplesTests.scala line 155
#[test]
fn test_programs_arrays_ssaimmfromcallable() {
    parse_sample_file("programs/arrays/ssaimmfromcallable.vale");
}

// Mirrors ParseSamplesTests.scala line 156
#[test]
fn test_programs_arrays_rsamutlen() {
    parse_sample_file("programs/arrays/rsamutlen.vale");
}

// Mirrors ParseSamplesTests.scala line 157
#[test]
fn test_programs_arrays_ssamutfromvalues() {
    parse_sample_file("programs/arrays/ssamutfromvalues.vale");
}

// Mirrors ParseSamplesTests.scala line 158
#[test]
fn test_programs_arrays_rsaimmlen() {
    parse_sample_file("programs/arrays/rsaimmlen.vale");
}

// Mirrors ParseSamplesTests.scala line 159
#[test]
fn test_programs_arrays_rsamut() {
    parse_sample_file("programs/arrays/rsamut.vale");
}

// Mirrors ParseSamplesTests.scala line 160
#[test]
fn test_programs_arrays_swaprsamutdestroy() {
    parse_sample_file("programs/arrays/swaprsamutdestroy.vale");
}

// Mirrors ParseSamplesTests.scala line 161
#[test]
fn test_programs_arrays_rsamutfromcallable() {
    parse_sample_file("programs/arrays/rsamutfromcallable.vale");
}

// Mirrors ParseSamplesTests.scala line 162
#[test]
fn test_programs_arrays_rsaimm() {
    parse_sample_file("programs/arrays/rsaimm.vale");
}

// Mirrors ParseSamplesTests.scala line 163
#[test]
fn test_programs_arrays_rsaimmfromcallable() {
    parse_sample_file("programs/arrays/rsaimmfromcallable.vale");
}

// Mirrors ParseSamplesTests.scala line 164
#[test]
fn test_programs_arrays_rsamutcapacity() {
    parse_sample_file("programs/arrays/rsamutcapacity.vale");
}

// Mirrors ParseSamplesTests.scala line 165
#[test]
fn test_programs_arrays_ssamutdestroyintocallable() {
    parse_sample_file("programs/arrays/ssamutdestroyintocallable.vale");
}

// Mirrors ParseSamplesTests.scala line 166
#[test]
fn test_programs_arrays_inlssaimm() {
    parse_sample_file("programs/arrays/inlssaimm.vale");
}

// Mirrors ParseSamplesTests.scala line 167
#[test]
fn test_programs_ufcs() {
    parse_sample_file("programs/ufcs.vale");
}

// Mirrors ParseSamplesTests.scala line 168
#[test]
fn test_programs_functions_overloads() {
    parse_sample_file("programs/functions/overloads.vale");
}

// Mirrors ParseSamplesTests.scala line 169
#[test]
fn test_programs_functions_recursion() {
    parse_sample_file("programs/functions/recursion.vale");
}

// Mirrors ParseSamplesTests.scala line 170
#[test]
fn test_programs_printfloat() {
    parse_sample_file("programs/printfloat.vale");
}

// Mirrors ParseSamplesTests.scala line 171
#[test]
fn test_programs_mutswaplocals() {
    parse_sample_file("programs/mutswaplocals.vale");
}

// Mirrors ParseSamplesTests.scala line 172
#[test]
fn test_programs_mutlocal() {
    parse_sample_file("programs/mutlocal.vale");
}

// Mirrors ParseSamplesTests.scala line 173
#[test]
fn test_programs_unstackifyret() {
    parse_sample_file("programs/unstackifyret.vale");
}

// Mirrors ParseSamplesTests.scala line 174
#[test]
fn test_programs_while_while() {
    parse_sample_file("programs/while/while.vale");
}

// Mirrors ParseSamplesTests.scala line 175
#[test]
fn test_programs_while_foreach() {
    parse_sample_file("programs/while/foreach.vale");
}

// Mirrors ParseSamplesTests.scala line 176
#[test]
fn test_programs_invalidaccess() {
    parse_sample_file("programs/invalidaccess.vale");
}

// Mirrors ParseSamplesTests.scala line 177
#[test]
fn test_programs_floatarithmetic() {
    parse_sample_file("programs/floatarithmetic.vale");
}

// Mirrors ParseSamplesTests.scala line 178
#[test]
fn test_programs_panic() {
    parse_sample_file("programs/panic.vale");
}

// Mirrors ParseSamplesTests.scala line 179
#[test]
fn test_list() {
    parse_sample_file("list/list.vale");
}

// Mirrors ParseSamplesTests.scala line 180
#[test]
fn test_ifunction1() {
    parse_sample_file("ifunction/ifunction1/ifunction1.vale");
}

// Mirrors ParseSamplesTests.scala line 181
#[test]
fn test_string() {
    parse_sample_file("string/string.vale");
}

// Mirrors ParseSamplesTests.scala line 182
#[test]
fn test_panicutils() {
    parse_sample_file("panicutils/panicutils.vale");
}

// Mirrors ParseSamplesTests.scala line 183
#[test]
fn test_castutils() {
    parse_sample_file("castutils/castutils.vale");
}

