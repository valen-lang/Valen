//! Debugger end-to-end tests (Milestone 1 + 2): compile a Vale program with
//! DWARF debug info, then assert on real debugger behavior — `lldb` backtraces
//! and stepping, plus raw DWARF DIE shape via `llvm-dwarfdump`.
//!
//! macOS/Native only — lldb, dsymutil, and llvm-dwarfdump aren't part of the
//! wasi toolchain. Under `VALE_TEST_BACKEND=wasi` each test skips (loudly, so
//! the pass isn't mistaken for real coverage).

use crate::end_to_end_tests::{compile_inline_debug, target_backend, Backend};

/// Returns true (and logs) when this run can't validate a debugger gate — i.e.
/// it isn't the Native backend, so there's no lldb/dSYM. Callers `return` on
/// true. The eprintln keeps the skip visible in the transcript rather than
/// letting a wasi run's early-return masquerade as real coverage.
fn skip_non_native() -> bool {
    if matches!(target_backend(), Backend::Native) {
        return false;
    }
    eprintln!(
        "SKIP: debugger gate requires the Native backend (lldb/dsymutil/dwarfdump); \
         not validated under wasi."
    );
    true
}

// ---------------------------------------------------------------------------
// M1 — function-level DWARF (backtraces resolve file:line)
// ---------------------------------------------------------------------------

/// M1: a function-level DISubprogram lets lldb resolve `b :main` + a backtrace
/// to the function's source line.
#[test]
fn breakpoint_resolves_to_main_source_line() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug("exported func main() int { return 42; }");
    cp.lldb_check(
        &["b :main", "run", "bt"],
        &["test.vale:1", "frame #0", ":main"],
    );
}

/// M1: `b <file>:<line>` (the IDE/user breakpoint shape) resolves too, which
/// requires the DICompileUnit be anchored to the real source file.
#[test]
fn breakpoint_resolves_by_file_and_line() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug("exported func main() int { return 42; }");
    cp.lldb_check(
        &["b test.vale:1", "run", "bt"],
        &["test.vale:1", "frame #0", ":main"],
    );
}

/// M1: the breakpoint tracks the function's declaration line — declaring `main`
/// on line 4 resolves there, not always line 1.
#[test]
fn breakpoint_line_tracks_function_declaration() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "\n\
         \n\
         \n\
         exported func main() int { return 42; }\n",
    );
    cp.lldb_check(&["b :main", "run", "bt"], &["test.vale:4", ":main"]);
}

/// M1: every user function gets its own DISubprogram with the right line, and
/// the per-source-path DIFile cache doesn't smear lines across functions.
#[test]
fn distinct_functions_resolve_to_their_own_lines() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int { return helper(); }\n\
         \n\
         \n\
         func helper() int { return 7; }\n",
    );
    cp.lldb_check(
        &["image lookup -F :main", "image lookup -F helper"],
        &["test.vale:1", "test.vale:4"],
    );
}

// ---------------------------------------------------------------------------
// Structural DWARF (assert raw DIEs, decoupled from lldb formatting drift)
// ---------------------------------------------------------------------------

/// The emitted DWARF has the shape we intend, independent of how lldb renders
/// it: a compile unit tagged as ours, and a `:main` subprogram with a decl
/// line. Guards against lldb someday passing our line-based gates "for the
/// wrong reason" while the underlying DIEs regressed.
#[test]
fn dwarf_has_compile_unit_and_named_subprogram() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug("exported func main() int { return 42; }");
    let out = cp.dwarfdump_capture(&["--debug-info"]);
    assert!(out.contains("DW_TAG_compile_unit"), "no compile unit:\n{out}");
    assert!(
        out.contains("DW_AT_producer") && out.contains("(\"Vale compiler\")"),
        "compile unit not tagged as the Vale compiler:\n{out}"
    );
    assert!(out.contains("DW_TAG_subprogram"), "no subprogram DIE:\n{out}");
    assert!(out.contains("(\":main\")"), "no :main subprogram name:\n{out}");
    assert!(out.contains("DW_AT_decl_line"), "subprogram has no decl line:\n{out}");
    assert!(
        out.contains("DW_AT_decl_file") && out.contains("test.vale"),
        "decl file isn't test.vale:\n{out}"
    );
}

/// Boundary gate for the body-range anchor: with the `{` on a different line
/// than the `func` keyword, the DISubprogram's `DW_AT_decl_line` still resolves
/// to the declaration line (1), because the body IE's range starts at the
/// function declaration, not the brace. Distinguishes "correct decl line" from
/// a "body's first statement" bug — which single-line-body fixtures can't.
#[test]
fn function_decl_line_is_declaration_not_body_statement() {
    if skip_non_native() {
        return;
    }
    // func keyword line 1; `{` line 2; `return 42;` line 3.
    let (cp, _) = compile_inline_debug(
        "exported func main() int\n\
         {\n\
         return 42;\n\
         }\n",
    );
    let out = cp.dwarfdump_capture(&["--debug-info"]);
    assert!(out.contains("(\":main\")"), "no :main subprogram:\n{out}");
    assert!(
        out.contains("DW_AT_decl_line\t(1)"),
        "expected decl_line 1 (the declaration line), not the body statement:\n{out}"
    );
}

// ---------------------------------------------------------------------------
// M2 — per-statement stepping (ordered: the sequence of stops is asserted)
// ---------------------------------------------------------------------------

/// M2: three sequential statements on consecutive lines. Breakpoint at the
/// first, then step twice; each stop lands on the NEXT line, in order.
#[test]
fn three_statements_step_to_distinct_lines() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         x = 7;\n\
         y = 11;\n\
         return x + y;\n\
         }\n",
    );
    cp.lldb_check_ordered(
        &[
            "b test.vale:2",
            "run",
            "thread step-over",
            "frame info",
            "thread step-over",
            "frame info",
        ],
        &["test.vale:2", "test.vale:3", "test.vale:4"],
    );
}

/// M2: stepping through a mutation (`set x = ...`) lands on the assignment line,
/// not the function entry.
#[test]
fn mutate_steps_to_assignment_line() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         x = 7;\n\
         set x = 11;\n\
         return x;\n\
         }\n",
    );
    cp.lldb_check_ordered(
        &[
            "b test.vale:2",
            "run",
            "thread step-over",
            "frame info",
            "thread step-over",
            "frame info",
        ],
        &["test.vale:2", "test.vale:3", "test.vale:4"],
    );
}

/// M2: stepping a `while` loop — step from the loop header into the body.
/// Loop machinery (break-target setup, branch back) doesn't collapse stepping
/// onto the function entry.
#[test]
fn while_loop_steps_through_body() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         x = 0;\n\
         while x < 1 { set x = x + 1; }\n\
         return x;\n\
         }\n",
    );
    cp.lldb_check_ordered(
        &[
            "b test.vale:2",
            "run",
            "thread step-over",
            "frame info",
            "thread step-over",
            "frame info",
        ],
        &["test.vale:2", "test.vale:3"],
    );
}

/// M2: `thread step-over` past a function call advances to the next user
/// statement, not into the callee or back to line 1.
#[test]
fn step_over_function_call() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         x = helper();\n\
         return x;\n\
         }\n\
         func helper() int { return 42; }\n",
    );
    cp.lldb_check_ordered(
        &["b test.vale:2", "run", "thread step-over", "frame info"],
        &["test.vale:2", "test.vale:3"],
    );
}

/// M2: stepping into an `if` branch — the branch body carries its own range,
/// not the function-entry fallback.
#[test]
fn if_branch_steps_to_branch_body() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         x = 7;\n\
         if true { return x; }\n\
         return 0;\n\
         }\n",
    );
    cp.lldb_check_ordered(
        &[
            "b test.vale:2",
            "run",
            "thread step-over",
            "frame info",
            "thread step-over",
            "frame info",
        ],
        &["test.vale:2", "test.vale:3"],
    );
}

/// M2: stepping past a member access (`s.x`). Exercises the member-lookup path;
/// without its own range it would inherit the function-entry line.
#[test]
fn member_access_steps_to_distinct_lines() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "struct S { x int; }\n\
         exported func main() int {\n\
         s = S(7);\n\
         return s.x;\n\
         }\n",
    );
    cp.lldb_check_ordered(
        &["b test.vale:3", "run", "thread step-over", "frame info"],
        &["test.vale:3", "test.vale:4"],
    );
}

// ---------------------------------------------------------------------------
// Backtrace depth — caller + callee frames both resolve to their lines
// ---------------------------------------------------------------------------

/// Break inside a callee and assert `bt` resolves BOTH frames to their source
/// lines: the callee at its statement, and the caller at the call site. Guards
/// multi-frame line resolution, which the step-over gates don't reach.
#[test]
fn backtrace_resolves_caller_and_callee_frames() {
    if skip_non_native() {
        return;
    }
    // main line 1, `return helper()` line 2; helper line 4, `return 42` line 5.
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         return helper();\n\
         }\n\
         func helper() int {\n\
         return 42;\n\
         }\n",
    );
    let out = cp.lldb_capture(&["b test.vale:5", "run", "bt"]);
    // Callee frame (frame #0) at its line, then caller frame at the call site.
    assert!(out.contains("helper"), "no helper frame in bt:\n{out}");
    assert!(out.contains("test.vale:5"), "callee frame not at line 5:\n{out}");
    assert!(out.contains(":main"), "no :main frame in bt:\n{out}");
    assert!(out.contains("test.vale:2"), "caller frame not at call site line 2:\n{out}");
}

// ---------------------------------------------------------------------------
// Exotic kinds — the un-ignored loc plumbing holds across more builders
// ---------------------------------------------------------------------------

// NOTE: an interface upcast + virtual-dispatch gate would exercise the
// interface-call / upcast builders' loc stamping, but interface compilation is
// deferred on this branch (the `virtuals` e2e tests are `#[ignore]`d "deferred
// at experimental-2 squash baseline"; the humanizer panics on interface
// programs). Add that gate when interface compilation is un-deferred.

/// A static-sized-array program compiles with debug info and steps by line —
/// exercises the array-new / array-lookup builders' loc stamping.
#[test]
fn array_program_steps_by_line() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         a = [#](23, 31, 42);\n\
         return __copy_prim(a.2);\n\
         }\n",
    );
    cp.lldb_check_ordered(
        &["b test.vale:2", "run", "thread step-over", "frame info"],
        &["test.vale:2", "test.vale:3"],
    );
}

// ---------------------------------------------------------------------------
// M3 — local variable inspection (frame variable shows values)
// ---------------------------------------------------------------------------

/// M3: `frame variable x` shows an int local's value. Requires the Stackify
/// handler to emit a DILocalVariable + llvm.dbg.declare against the local's
/// alloca, with a DIType matching the local's Vale type.
#[test]
fn local_variable_visible_in_lldb() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         x = 7;\n\
         return x;\n\
         }\n",
    );
    // `frame variable` formats as `(<type>) <name> = <value>`; assert name+value.
    cp.lldb_check(&["b test.vale:3", "run", "frame variable x"], &["x = 7"]);
}

/// M3: function parameters reach lldb as locals too — Vale lowers `func foo(a
/// int)` to a Stackify of the argument into a named local, so the Stackify-side
/// dbg.declare picks them up with no separate formal-parameter handling.
#[test]
fn function_argument_visible_in_lldb() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int { return helper(7); }\n\
         func helper(a int) int {\n\
         return a;\n\
         }\n",
    );
    cp.lldb_check(&["b test.vale:3", "run", "frame variable a"], &["a = 7"]);
}

/// M3: bool and float locals are inspectable too — exercises getOrCreateDIType's
/// non-Int primitive branches (DW_ATE_boolean at 8-bit, DW_ATE_float at 64-bit).
#[test]
fn bool_and_float_locals_visible_in_lldb() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         b = true;\n\
         f = 1.5;\n\
         if b { return 1; }\n\
         return 0;\n\
         }\n",
    );
    let out = cp.lldb_capture(&[
        "b test.vale:4",
        "run",
        "frame variable b",
        "frame variable f",
    ]);
    assert!(out.contains("b = true"), "missing `b = true`:\n{out}");
    assert!(out.contains("f = 1.5"), "missing `f = 1.5`:\n{out}");
}

// ---------------------------------------------------------------------------
// Layer 1 — struct composite types (frame variable walks user fields)
// ---------------------------------------------------------------------------

/// Layer 1: `frame variable -P 1 s` walks an inline struct local's user field
/// and prints its value. Requires getOrCreateDIType to build a flattened
/// DICompositeType for the struct's inner LLVM layout instead of the opaque-ref
/// fallback.
#[test]
fn struct_local_field_visible_in_lldb() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "struct S { x int; }\n\
         exported func main() int {\n\
         s = S(7);\n\
         return s.x;\n\
         }\n",
    );
    cp.lldb_check(
        &["b test.vale:4", "run", "frame variable -P 1 s"],
        &["x = 7"],
    );
}

/// Layer 1: multiple fields of mixed primitive types show up, confirming the
/// per-field DIType dispatch and offset math cover Int + Bool together.
#[test]
fn struct_local_multifield_visible_in_lldb() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "struct S { a int; b bool; }\n\
         exported func main() int {\n\
         s = S(7, true);\n\
         return s.a;\n\
         }\n",
    );
    let out = cp.lldb_capture(&["b test.vale:4", "run", "frame variable -P 1 s"]);
    assert!(out.contains("a = 7"), "missing `a = 7`:\n{out}");
    assert!(out.contains("b = true"), "missing `b = true`:\n{out}");
}

/// Layer 1, DWARF-shaped: assert the raw DIEs for `S` rather than lldb's render
/// — a `DW_TAG_structure_type` named `S` with `DW_TAG_member` children `a`/`b`.
/// Decouples the structural regression check from lldb formatting drift.
#[test]
fn dwarf_dies_for_struct_have_user_members() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "struct S { a int; b bool; }\n\
         exported func main() int {\n\
         s = S(7, true);\n\
         return s.a;\n\
         }\n",
    );
    let out = cp.dwarfdump_capture(&["--debug-info", "--name=S", "--show-children"]);
    assert!(
        out.contains("DW_TAG_structure_type"),
        "no DW_TAG_structure_type for S:\n{out}"
    );
    assert!(out.contains("(\"a\")"), "missing member `a`:\n{out}");
    assert!(out.contains("(\"b\")"), "missing member `b`:\n{out}");
}

/// Locals created by a struct **destructure** (`[x, y] = ...`) are inspectable
/// too. This is the gap that motivated moving the dbg.declare into
/// `makeHammerLocal`: the destructure path calls `makeHammerLocal` without any
/// per-call-site debug emission, so before the move these locals were invisible.
#[test]
fn destructured_locals_visible_in_lldb() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "struct Pair { a int; b int; }\n\
         exported func main() int {\n\
         [x, y] = Pair(7, 11);\n\
         return x + y;\n\
         }\n",
    );
    let out = cp.lldb_capture(&[
        "b test.vale:4",
        "run",
        "frame variable x",
        "frame variable y",
    ]);
    assert!(out.contains("x = 7"), "missing `x = 7`:\n{out}");
    assert!(out.contains("y = 11"), "missing `y = 11`:\n{out}");
}

// ---------------------------------------------------------------------------
// Pointer-to-struct locals — a borrow ref walks its pointee's fields
// ---------------------------------------------------------------------------

/// A borrow-ref local (`ref = &carrier`) lowers to a pointer straight to the
/// inner struct (no control block between the pointer and the fields in this
/// region). `frame variable -P 1 ref` should walk the pointer and show the
/// struct's fields, rather than the opaque address the ref used to render as.
#[test]
fn borrow_ref_struct_fields_visible_in_lldb() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "struct Carrier { hp int; interceptors int; }\n\
         exported func main() int {\n\
         carrier = Carrier(400, 8);\n\
         ref = &carrier;\n\
         return __copy_prim(ref.interceptors);\n\
         }\n",
    );
    let out = cp.lldb_capture(&["b test.vale:5", "run", "frame variable -P 1 ref"]);
    assert!(out.contains("hp = 400"), "missing `hp = 400`:\n{out}");
    assert!(
        out.contains("interceptors = 8"),
        "missing `interceptors = 8`:\n{out}"
    );
}

/// DWARF-shaped: the borrow local's variable DIE is a `DW_TAG_pointer_type`
/// whose pointee is the `DW_TAG_structure_type` for `Carrier` — decoupled from
/// lldb's rendering. Confirms we emit pointer-to-composite, not the opaque ref.
#[test]
fn dwarf_dies_for_borrow_ref_are_pointer_to_struct() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "struct Carrier { hp int; interceptors int; }\n\
         exported func main() int {\n\
         carrier = Carrier(400, 8);\n\
         ref = &carrier;\n\
         return __copy_prim(ref.interceptors);\n\
         }\n",
    );
    let out = cp.dwarfdump_capture(&["--debug-info"]);
    assert!(
        out.contains("DW_TAG_pointer_type"),
        "no DW_TAG_pointer_type for the borrow ref:\n{out}"
    );
    assert!(
        out.contains("DW_TAG_structure_type"),
        "no DW_TAG_structure_type for Carrier:\n{out}"
    );
    assert!(
        out.contains("(\"interceptors\")"),
        "missing member `interceptors`:\n{out}"
    );
}

// ---------------------------------------------------------------------------
// Layer 1.5 — static-sized array locals walk their elements
// ---------------------------------------------------------------------------

/// A static-sized-array local (`a = [#](23, 31, 42)`) is an inline `[N x elem]`
/// value; `frame variable -P 1 a` should walk its elements instead of rendering
/// the opaque address the array used to show as. Runtime-sized arrays (RSA) are
/// deferred on this branch, so this covers only SSA.
#[test]
fn array_local_elements_visible_in_lldb() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         a = [#](23, 31, 42);\n\
         return __copy_prim(a.2);\n\
         }\n",
    );
    let out = cp.lldb_capture(&["b test.vale:3", "run", "frame variable -P 1 a"]);
    assert!(out.contains("23"), "missing element `23`:\n{out}");
    assert!(out.contains("42"), "missing element `42`:\n{out}");
}

/// DWARF-shaped: the array local's DIE is a `DW_TAG_array_type` with a
/// `DW_TAG_subrange_type` — decoupled from lldb's rendering. Confirms we emit an
/// array type, not the opaque ref.
#[test]
fn dwarf_dies_for_array_are_array_type() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         a = [#](23, 31, 42);\n\
         return __copy_prim(a.2);\n\
         }\n",
    );
    let out = cp.dwarfdump_capture(&["--debug-info"]);
    assert!(
        out.contains("DW_TAG_array_type"),
        "no DW_TAG_array_type for the array local:\n{out}"
    );
    assert!(
        out.contains("DW_TAG_subrange_type"),
        "no DW_TAG_subrange_type for the array local:\n{out}"
    );
}

// ---------------------------------------------------------------------------
// Sentinel breakpoints — the breadth gates anchor stop-points on labeled
// `0; // lldb breakpoint: <label>` no-op lines (via `br s -p`) instead of line
// numbers, so they don't rot when a fixture is edited.
// ---------------------------------------------------------------------------

/// A bare `0; // lldb breakpoint: <label>` statement compiles (the discarded int
/// is auto-dropped) and emits a breakpointable line, so an lldb source-pattern
/// breakpoint (`br s -p '<label>'`) binds to it. Critically, it binds *at* the
/// sentinel with prior state visible — a break on a sentinel placed after
/// `x = 73` but before `set x = 42` sees x = 73, not 42 (it doesn't slide past
/// the next statement). This is the mechanism the breadth gates rely on.
#[test]
fn sentinel_breakpoint_binds() {
    if skip_non_native() {
        return;
    }
    let (cp, _) = compile_inline_debug(
        "exported func main() int {\n\
         x int = 73;\n\
         0; // lldb breakpoint: probe-before\n\
         set x = 42;\n\
         0; // lldb breakpoint: probe-after\n\
         return x;\n\
         }\n",
    );
    let out = cp.lldb_capture(&[
        "br s -p 'lldb breakpoint: probe-before' -f test.vale",
        "br s -p 'lldb breakpoint: probe-after' -f test.vale",
        "run",
        "frame variable x",
        "continue",
        "frame variable x",
    ]);
    assert!(
        !out.contains("Unable to resolve breakpoint"),
        "a sentinel breakpoint didn't bind to any location:\n{out}"
    );
    // `(int) x = ` is the `frame variable` output; the bare `x = 42` also appears
    // in lldb's source-context display of `set x = 42;`, so match the typed form.
    assert!(out.contains("(int) x = 73"), "no `x = 73` at probe-before:\n{out}");
    assert!(out.contains("(int) x = 42"), "no `x = 42` at probe-after:\n{out}");
    let before = out.find("(int) x = 73").unwrap();
    let after = out.find("(int) x = 42").unwrap();
    assert!(
        before < after,
        "expected x = 73 (probe-before) before x = 42 (probe-after) — did the \
         breakpoint slide past the `set`?:\n{out}"
    );
}
