use crate::end_to_end_tests::{compile_program, programs_dir};
use std::fs;

// --- Generated C-header ABI goldens (@HTSLVBDTCZ) ---
//
// Each test compiles a fixture and asserts *every* header the backend emits
// for that fixture's package (`include/vtest/*.h`) against an inline golden,
// one header per assertion. The filename list is asserted first as a
// completeness guard: a new or removed header fails there, so nothing crosses
// the FFI boundary un-pinned. Together they pin the whole generated C ABI: the
// handle typedefs (concrete kinds as an 8-byte `{ uint64_t _reserved }`,
// interfaces as a 16-byte `{ _reserved0, _reserved1 }` plus their `_TAG_`
// constants) and every auto-generated accessor's signature (alias/dealias/
// ref_eq/field getters/new/upcast/downcast/typeTag, each with its sret
// `vale_abi_*` form and its by-value form).
//
// The goldens are inline and hand-maintained — there is deliberately no bless
// mechanism. An intentional ABI change must show up as a visible diff here,
// edited by hand.

// VCOORD: we should probably have non-imm versions of the golden tests.

// Concrete share kind: 8-byte handle typedef + the full struct C API.
#[test]
#[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"]
fn structimm_export_headers_golden() {
    let dir = programs_dir().join("programs/externs/structimmreturnexport");
    let cp = compile_program(&dir, &[], |_| {});
    let vtest = cp.cwd.join("include/vtest");
    let read = |name: &str| {
        fs::read_to_string(vtest.join(name))
            .unwrap_or_else(|e| panic!("reading {}: {}", name, e))
    };

    let mut names: Vec<String> = fs::read_dir(&vtest)
        .unwrap_or_else(|e| panic!("reading dir {}: {}", vtest.display(), e))
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "Flamscrankle.h",
            "Flamscrankle_a.h",
            "Flamscrankle_alias.h",
            "Flamscrankle_c.h",
            "Flamscrankle_dealias.h",
            "Flamscrankle_new.h",
            "Flamscrankle_ref_eq.h",
            "cMakeStruct.h",
            "str.h",
            "str_alias.h",
            "str_char_at.h",
            "str_dealias.h",
            "str_len.h",
            "str_ref_eq.h",
            "valeMakeStruct.h",
        ]
    );

    assert_eq!(read("Flamscrankle.h"), r#"#ifndef VALE_EXPORTS_Flamscrankle_H_
#define VALE_EXPORTS_Flamscrankle_H_
#include "ValeBuiltins.h"
typedef struct vtest_Flamscrankle { uint64_t _reserved; } vtest_Flamscrankle;
#endif
"#);

    assert_eq!(read("Flamscrankle_a.h"), r#"#ifndef VALE_EXPORTS_Flamscrankle_a_H_
#define VALE_EXPORTS_Flamscrankle_a_H_
#include "ValeBuiltins.h"
#include "vtest/Flamscrankle.h"
extern int32_t vale_abi_vtest_Flamscrankle_a(vtest_Flamscrankle* param0);
extern int32_t vtest_Flamscrankle_a(vtest_Flamscrankle param0);
#endif
"#);

    assert_eq!(read("Flamscrankle_alias.h"), r#"#ifndef VALE_EXPORTS_Flamscrankle_alias_H_
#define VALE_EXPORTS_Flamscrankle_alias_H_
#include "ValeBuiltins.h"
#include "vtest/Flamscrankle.h"
#include "vtest/Flamscrankle.h"
extern void  vale_abi_vtest_Flamscrankle_alias(vtest_Flamscrankle* __ret, vtest_Flamscrankle* param0);
extern vtest_Flamscrankle vtest_Flamscrankle_alias(vtest_Flamscrankle param0);
#endif
"#);

    assert_eq!(read("Flamscrankle_c.h"), r#"#ifndef VALE_EXPORTS_Flamscrankle_c_H_
#define VALE_EXPORTS_Flamscrankle_c_H_
#include "ValeBuiltins.h"
#include "vtest/Flamscrankle.h"
extern int32_t vale_abi_vtest_Flamscrankle_c(vtest_Flamscrankle* param0);
extern int32_t vtest_Flamscrankle_c(vtest_Flamscrankle param0);
#endif
"#);

    assert_eq!(read("Flamscrankle_dealias.h"), r#"#ifndef VALE_EXPORTS_Flamscrankle_dealias_H_
#define VALE_EXPORTS_Flamscrankle_dealias_H_
#include "ValeBuiltins.h"
#include "vtest/Flamscrankle.h"
extern void vale_abi_vtest_Flamscrankle_dealias(vtest_Flamscrankle* param0);
extern void vtest_Flamscrankle_dealias(vtest_Flamscrankle param0);
#endif
"#);

    assert_eq!(read("Flamscrankle_new.h"), r#"#ifndef VALE_EXPORTS_Flamscrankle_new_H_
#define VALE_EXPORTS_Flamscrankle_new_H_
#include "ValeBuiltins.h"
#include "vtest/Flamscrankle.h"
extern void  vale_abi_vtest_Flamscrankle_new(vtest_Flamscrankle* __ret, int32_t param0, int32_t param1);
extern vtest_Flamscrankle vtest_Flamscrankle_new(int32_t param0, int32_t param1);
#endif
"#);

    assert_eq!(read("Flamscrankle_ref_eq.h"), r#"#ifndef VALE_EXPORTS_Flamscrankle_ref_eq_H_
#define VALE_EXPORTS_Flamscrankle_ref_eq_H_
#include "ValeBuiltins.h"
#include "vtest/Flamscrankle.h"
#include "vtest/Flamscrankle.h"
extern int8_t vale_abi_vtest_Flamscrankle_ref_eq(vtest_Flamscrankle* param0, vtest_Flamscrankle* param1);
extern int8_t vtest_Flamscrankle_ref_eq(vtest_Flamscrankle param0, vtest_Flamscrankle param1);
#endif
"#);

    assert_eq!(read("cMakeStruct.h"), r#"#ifndef VALE_EXPORTS_cMakeStruct_H_
#define VALE_EXPORTS_cMakeStruct_H_
#include "ValeBuiltins.h"
#include "vtest/Flamscrankle.h"
extern vtest_Flamscrankle vtest_cMakeStruct();
extern void  vale_abi_vtest_cMakeStruct(vtest_Flamscrankle* __ret);
#endif
"#);

    assert_eq!(read("str.h"), r#"#ifndef VALE_EXPORTS_str_H_
#define VALE_EXPORTS_str_H_
#include "ValeBuiltins.h"
typedef struct vtest_str { uint64_t _reserved; } vtest_str;
#endif
"#);

    assert_eq!(read("str_alias.h"), r#"#ifndef VALE_EXPORTS_str_alias_H_
#define VALE_EXPORTS_str_alias_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
#include "vtest/str.h"
extern void  vale_abi_vtest_str_alias(vtest_str* __ret, vtest_str* param0);
extern vtest_str vtest_str_alias(vtest_str param0);
#endif
"#);

    assert_eq!(read("str_char_at.h"), r#"#ifndef VALE_EXPORTS_str_char_at_H_
#define VALE_EXPORTS_str_char_at_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
extern int32_t vale_abi_vtest_str_char_at(vtest_str* param0, int32_t param1);
extern int32_t vtest_str_char_at(vtest_str param0, int32_t param1);
#endif
"#);

    assert_eq!(read("str_dealias.h"), r#"#ifndef VALE_EXPORTS_str_dealias_H_
#define VALE_EXPORTS_str_dealias_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
extern void vale_abi_vtest_str_dealias(vtest_str* param0);
extern void vtest_str_dealias(vtest_str param0);
#endif
"#);

    assert_eq!(read("str_len.h"), r#"#ifndef VALE_EXPORTS_str_len_H_
#define VALE_EXPORTS_str_len_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
extern int32_t vale_abi_vtest_str_len(vtest_str* param0);
extern int32_t vtest_str_len(vtest_str param0);
#endif
"#);

    assert_eq!(read("str_ref_eq.h"), r#"#ifndef VALE_EXPORTS_str_ref_eq_H_
#define VALE_EXPORTS_str_ref_eq_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
#include "vtest/str.h"
extern int8_t vale_abi_vtest_str_ref_eq(vtest_str* param0, vtest_str* param1);
extern int8_t vtest_str_ref_eq(vtest_str param0, vtest_str param1);
#endif
"#);

    assert_eq!(read("valeMakeStruct.h"), r#"#ifndef VALE_EXPORTS_valeMakeStruct_H_
#define VALE_EXPORTS_valeMakeStruct_H_
#include "ValeBuiltins.h"
#include "vtest/Flamscrankle.h"
extern void  vale_abi_vtest_valeMakeStruct(vtest_Flamscrankle* __ret);
extern vtest_Flamscrankle vtest_valeMakeStruct();
#endif
"#);
}

// Interface: 16-byte handle typedef + `_TAG_` constants + downcast/upcast/typeTag.
#[test]
#[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"]
fn interfaceimm_export_headers_golden() {
    let dir = programs_dir().join("programs/externs/interfaceimmreturnexport");
    let cp = compile_program(&dir, &[], |_| {});
    let vtest = cp.cwd.join("include/vtest");
    let read = |name: &str| {
        fs::read_to_string(vtest.join(name))
            .unwrap_or_else(|e| panic!("reading {}: {}", name, e))
    };

    let mut names: Vec<String> = fs::read_dir(&vtest)
        .unwrap_or_else(|e| panic!("reading dir {}: {}", vtest.display(), e))
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "Firefly.h",
            "Firefly_alias.h",
            "Firefly_asIShip.h",
            "Firefly_dealias.h",
            "Firefly_fuel.h",
            "Firefly_new.h",
            "Firefly_ref_eq.h",
            "IShip.h",
            "IShip_alias.h",
            "IShip_asFirefly.h",
            "IShip_dealias.h",
            "IShip_ref_eq.h",
            "IShip_typeTag.h",
            "cMakeShip.h",
            "str.h",
            "str_alias.h",
            "str_char_at.h",
            "str_dealias.h",
            "str_len.h",
            "str_ref_eq.h",
            "valeMakeShip.h",
        ]
    );

    assert_eq!(read("Firefly.h"), r#"#ifndef VALE_EXPORTS_Firefly_H_
#define VALE_EXPORTS_Firefly_H_
#include "ValeBuiltins.h"
typedef struct vtest_Firefly { uint64_t _reserved; } vtest_Firefly;
#endif
"#);

    assert_eq!(read("Firefly_alias.h"), r#"#ifndef VALE_EXPORTS_Firefly_alias_H_
#define VALE_EXPORTS_Firefly_alias_H_
#include "ValeBuiltins.h"
#include "vtest/Firefly.h"
#include "vtest/Firefly.h"
extern void  vale_abi_vtest_Firefly_alias(vtest_Firefly* __ret, vtest_Firefly* param0);
extern vtest_Firefly vtest_Firefly_alias(vtest_Firefly param0);
#endif
"#);

    assert_eq!(read("Firefly_asIShip.h"), r#"#ifndef VALE_EXPORTS_Firefly_asIShip_H_
#define VALE_EXPORTS_Firefly_asIShip_H_
#include "ValeBuiltins.h"
#include "vtest/Firefly.h"
#include "vtest/IShip.h"
extern void  vale_abi_vtest_Firefly_asIShip(vtest_IShip* __ret, vtest_Firefly* param0);
extern vtest_IShip vtest_Firefly_asIShip(vtest_Firefly param0);
#endif
"#);

    assert_eq!(read("Firefly_dealias.h"), r#"#ifndef VALE_EXPORTS_Firefly_dealias_H_
#define VALE_EXPORTS_Firefly_dealias_H_
#include "ValeBuiltins.h"
#include "vtest/Firefly.h"
extern void vale_abi_vtest_Firefly_dealias(vtest_Firefly* param0);
extern void vtest_Firefly_dealias(vtest_Firefly param0);
#endif
"#);

    assert_eq!(read("Firefly_fuel.h"), r#"#ifndef VALE_EXPORTS_Firefly_fuel_H_
#define VALE_EXPORTS_Firefly_fuel_H_
#include "ValeBuiltins.h"
#include "vtest/Firefly.h"
extern int32_t vale_abi_vtest_Firefly_fuel(vtest_Firefly* param0);
extern int32_t vtest_Firefly_fuel(vtest_Firefly param0);
#endif
"#);

    assert_eq!(read("Firefly_new.h"), r#"#ifndef VALE_EXPORTS_Firefly_new_H_
#define VALE_EXPORTS_Firefly_new_H_
#include "ValeBuiltins.h"
#include "vtest/Firefly.h"
extern void  vale_abi_vtest_Firefly_new(vtest_Firefly* __ret, int32_t param0);
extern vtest_Firefly vtest_Firefly_new(int32_t param0);
#endif
"#);

    assert_eq!(read("Firefly_ref_eq.h"), r#"#ifndef VALE_EXPORTS_Firefly_ref_eq_H_
#define VALE_EXPORTS_Firefly_ref_eq_H_
#include "ValeBuiltins.h"
#include "vtest/Firefly.h"
#include "vtest/Firefly.h"
extern int8_t vale_abi_vtest_Firefly_ref_eq(vtest_Firefly* param0, vtest_Firefly* param1);
extern int8_t vtest_Firefly_ref_eq(vtest_Firefly param0, vtest_Firefly param1);
#endif
"#);

    assert_eq!(read("IShip.h"), r#"#ifndef VALE_EXPORTS_IShip_H_
#define VALE_EXPORTS_IShip_H_
#include "ValeBuiltins.h"
#define vtest_IShip_TAG_Firefly 0
typedef struct vtest_IShip { uint64_t _reserved0; uint64_t _reserved1; } vtest_IShip;
#endif
"#);

    assert_eq!(read("IShip_alias.h"), r#"#ifndef VALE_EXPORTS_IShip_alias_H_
#define VALE_EXPORTS_IShip_alias_H_
#include "ValeBuiltins.h"
#include "vtest/IShip.h"
#include "vtest/IShip.h"
extern void  vale_abi_vtest_IShip_alias(vtest_IShip* __ret, vtest_IShip* param0);
extern vtest_IShip vtest_IShip_alias(vtest_IShip param0);
#endif
"#);

    assert_eq!(read("IShip_asFirefly.h"), r#"#ifndef VALE_EXPORTS_IShip_asFirefly_H_
#define VALE_EXPORTS_IShip_asFirefly_H_
#include "ValeBuiltins.h"
#include "vtest/IShip.h"
#include "vtest/Firefly.h"
extern void  vale_abi_vtest_IShip_asFirefly(vtest_Firefly* __ret, vtest_IShip* param0);
extern vtest_Firefly vtest_IShip_asFirefly(vtest_IShip param0);
#endif
"#);

    assert_eq!(read("IShip_dealias.h"), r#"#ifndef VALE_EXPORTS_IShip_dealias_H_
#define VALE_EXPORTS_IShip_dealias_H_
#include "ValeBuiltins.h"
#include "vtest/IShip.h"
extern void vale_abi_vtest_IShip_dealias(vtest_IShip* param0);
extern void vtest_IShip_dealias(vtest_IShip param0);
#endif
"#);

    assert_eq!(read("IShip_ref_eq.h"), r#"#ifndef VALE_EXPORTS_IShip_ref_eq_H_
#define VALE_EXPORTS_IShip_ref_eq_H_
#include "ValeBuiltins.h"
#include "vtest/IShip.h"
#include "vtest/IShip.h"
extern int8_t vale_abi_vtest_IShip_ref_eq(vtest_IShip* param0, vtest_IShip* param1);
extern int8_t vtest_IShip_ref_eq(vtest_IShip param0, vtest_IShip param1);
#endif
"#);

    assert_eq!(read("IShip_typeTag.h"), r#"#ifndef VALE_EXPORTS_IShip_typeTag_H_
#define VALE_EXPORTS_IShip_typeTag_H_
#include "ValeBuiltins.h"
#include "vtest/IShip.h"
extern int32_t vale_abi_vtest_IShip_typeTag(vtest_IShip* param0);
extern int32_t vtest_IShip_typeTag(vtest_IShip param0);
#endif
"#);

    assert_eq!(read("cMakeShip.h"), r#"#ifndef VALE_EXPORTS_cMakeShip_H_
#define VALE_EXPORTS_cMakeShip_H_
#include "ValeBuiltins.h"
#include "vtest/IShip.h"
extern vtest_IShip vtest_cMakeShip();
extern void  vale_abi_vtest_cMakeShip(vtest_IShip* __ret);
#endif
"#);

    assert_eq!(read("str.h"), r#"#ifndef VALE_EXPORTS_str_H_
#define VALE_EXPORTS_str_H_
#include "ValeBuiltins.h"
typedef struct vtest_str { uint64_t _reserved; } vtest_str;
#endif
"#);

    assert_eq!(read("str_alias.h"), r#"#ifndef VALE_EXPORTS_str_alias_H_
#define VALE_EXPORTS_str_alias_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
#include "vtest/str.h"
extern void  vale_abi_vtest_str_alias(vtest_str* __ret, vtest_str* param0);
extern vtest_str vtest_str_alias(vtest_str param0);
#endif
"#);

    assert_eq!(read("str_char_at.h"), r#"#ifndef VALE_EXPORTS_str_char_at_H_
#define VALE_EXPORTS_str_char_at_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
extern int32_t vale_abi_vtest_str_char_at(vtest_str* param0, int32_t param1);
extern int32_t vtest_str_char_at(vtest_str param0, int32_t param1);
#endif
"#);

    assert_eq!(read("str_dealias.h"), r#"#ifndef VALE_EXPORTS_str_dealias_H_
#define VALE_EXPORTS_str_dealias_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
extern void vale_abi_vtest_str_dealias(vtest_str* param0);
extern void vtest_str_dealias(vtest_str param0);
#endif
"#);

    assert_eq!(read("str_len.h"), r#"#ifndef VALE_EXPORTS_str_len_H_
#define VALE_EXPORTS_str_len_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
extern int32_t vale_abi_vtest_str_len(vtest_str* param0);
extern int32_t vtest_str_len(vtest_str param0);
#endif
"#);

    assert_eq!(read("str_ref_eq.h"), r#"#ifndef VALE_EXPORTS_str_ref_eq_H_
#define VALE_EXPORTS_str_ref_eq_H_
#include "ValeBuiltins.h"
#include "vtest/str.h"
#include "vtest/str.h"
extern int8_t vale_abi_vtest_str_ref_eq(vtest_str* param0, vtest_str* param1);
extern int8_t vtest_str_ref_eq(vtest_str param0, vtest_str param1);
#endif
"#);

    assert_eq!(read("valeMakeShip.h"), r#"#ifndef VALE_EXPORTS_valeMakeShip_H_
#define VALE_EXPORTS_valeMakeShip_H_
#include "ValeBuiltins.h"
#include "vtest/IShip.h"
extern void  vale_abi_vtest_valeMakeShip(vtest_IShip* __ret);
extern vtest_IShip vtest_valeMakeShip();
#endif
"#);
}
