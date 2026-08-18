//! Rung-2 use-after-churn tests. A reference to a runtime-sized array *element* points into a child
//! group; a call that declares `mut(r)` over the group `r` the array is bound to invalidates every
//! live element reference into `r`'s child groups, so using such a reference afterward is an error.
//! A reference to the whole array, or to an inline field, is in the parent group and survives.
//!
//! Fixtures build a monomorphic RSA local, bind a *borrow* to an element (never read the value out),
//! and never push/pop — see the plan `please-plan-out-rung-quiet-kazoo.md`.

use super::util::{assert_borrow_error_renders_with_arrays, assert_compiles_clean_with_arrays};

// Slice 1 (Phase 0): the pipeline reaches the (no-op) checker for an RSA-element fixture — build an
// array, borrow an element, call a `mut(r)` function, and never use the element afterward. Clean.
#[test]
fn test_rsa_element_borrow_no_use_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(arr &[]int in r) mut(r) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  ref = &arr[0];\n",
    "  churn(&arr);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 2 (Phase A): using an element reference after a `mut(r)` churn call is rejected.
#[test]
fn test_use_element_after_churn_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(arr &[]int in r) mut(r) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = &arr[0];\n",
      "  churn(&arr);\n",
      "  observe(ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 3 (Phase A): a call that borrows the array but does not declare `mut` does not churn, so an
// element reference stays live across it.
#[test]
fn test_use_element_after_readonly_call_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func touch<r'>(arr &[]int in r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  ref = &arr[0];\n",
    "  touch(&arr);\n",
    "  observe(ref);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 4 (Phase A): churning a *different* array's group does not invalidate an element reference
// into this array — the callee only churns the group it was handed.
#[test]
fn test_churn_other_group_leaves_element_live() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(a &[]int in r) mut(r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  other = Array<int>(3);\n",
    "  ref = &arr[0];\n",
    "  churn(&other);\n",
    "  observe(ref);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 5 (Phase A): a reference to the whole array is in the parent group, not a child group, so a
// churn does not invalidate it.
#[test]
fn test_whole_array_ref_survives_churn() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(a &[]int in r) mut(r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  whole = &arr;\n",
    "  churn(&arr);\n",
    "  observe(whole);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 5 (Phase A): with both a whole-array reference and an element reference live across one
// churn, only the element reference (child group) is invalidated.
#[test]
fn test_element_ref_dies_but_sibling_whole_array_ref_lives() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  whole = &arr;\n",
      "  ref = &arr[0];\n",
      "  churn(&arr);\n",
      "  observe(whole);\n",
      "  observe(ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 6 (Phase A): using an element reference *before* the churn is clean — a churn only affects
// references live across it.
#[test]
fn test_use_element_before_churn_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(a &[]int in r) mut(r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  ref = &arr[0];\n",
    "  observe(ref);\n",
    "  churn(&arr);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 7 (Phase A): a fresh element reference taken *after* the churn is live — invalidation marks
// the reference that existed across the churn, not the array.
#[test]
fn test_reborrow_after_churn_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(a &[]int in r) mut(r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  ref = &arr[0];\n",
    "  churn(&arr);\n",
    "  ref2 = &arr[0];\n",
    "  observe(ref2);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 8 (Phase B): a churn inside one `if` arm invalidates an element reference used after the
// `if` — the may-invalidation flows to the join.
#[test]
fn test_churn_in_one_arm_use_after_if_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = &arr[0];\n",
      "  if (true) {\n",
      "    churn(&arr);\n",
      "  }\n",
      "  observe(ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 9 (Phase B): a churn in both arms invalidates after the `if`.
#[test]
fn test_churn_in_both_arms_use_after_if_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = &arr[0];\n",
      "  if (true) {\n",
      "    churn(&arr);\n",
      "  } else {\n",
      "    churn(&arr);\n",
      "  }\n",
      "  observe(ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 10 (Phase B): a churn then a use *within* one arm is straight-line inside that arm.
#[test]
fn test_churn_then_use_within_arm_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = &arr[0];\n",
      "  if (true) {\n",
      "    churn(&arr);\n",
      "    observe(ref);\n",
      "  }\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 11 (Phase B): a churn in an arm that *diverges* (returns) never reaches the code after the
// `if`, so an element reference is still live there.
#[test]
fn test_churn_in_returning_arm_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(a &[]int in r) mut(r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  ref = &arr[0];\n",
    "  if (true) {\n",
    "    churn(&arr);\n",
    "    return 0;\n",
    "  }\n",
    "  observe(ref);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 12 (Phase B): using an element reference inside an arm, with the churn only in a later
// statement after the `if`, is clean — the use precedes the churn on every path.
#[test]
fn test_use_in_arm_then_later_churn_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(a &[]int in r) mut(r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  ref = &arr[0];\n",
    "  if (true) {\n",
    "    observe(ref);\n",
    "  }\n",
    "  churn(&arr);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 13 (Phase C): a reference created before a loop, churned inside the body, and used at the
// top of the body is invalidated on the second iteration — the back-edge carries the churn.
#[test]
fn test_use_at_loop_top_after_body_churn_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = &arr[0];\n",
      "  while (false) {\n",
      "    observe(ref);\n",
      "    churn(&arr);\n",
      "  }\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 14 (Phase C): a churn inside a loop body invalidates a reference used after the loop.
#[test]
fn test_use_after_loop_with_body_churn_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = &arr[0];\n",
      "  while (false) {\n",
      "    churn(&arr);\n",
      "  }\n",
      "  observe(ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 15 (Phase C): a reference created fresh each iteration and used before that iteration's
// churn is live — the back-edge does not carry it, because the binding is re-taken.
#[test]
fn test_fresh_element_each_iteration_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(a &[]int in r) mut(r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  while (false) {\n",
    "    ref = &arr[0];\n",
    "    observe(ref);\n",
    "    churn(&arr);\n",
    "  }\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 16 (Phase C): a loop with no churn leaves an element reference live.
#[test]
fn test_loop_without_churn_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  ref = &arr[0];\n",
    "  while (false) {\n",
    "    observe(ref);\n",
    "  }\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 17 (Phase D): an element reference with no churn anywhere is freely usable — no false
// positive.
#[test]
fn test_element_used_without_any_churn_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  ref = &arr[0];\n",
    "  observe(ref);\n",
    "  observe(ref);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 18 (Phase D): passing an invalidated element reference as a (non-first) argument is a use.
#[test]
fn test_pass_invalidated_element_ref_as_arg_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func pair<T>(a int, b &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = &arr[0];\n",
      "  churn(&arr);\n",
      "  pair(7, ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 19 (Phase D): churning one array leaves an element reference into a *different* array live.
#[test]
fn test_two_groups_churn_one_use_other_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(a &[]int in r) mut(r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  other = Array<int>(3);\n",
    "  kept = &other[0];\n",
    "  churn(&arr);\n",
    "  observe(kept);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 20 (Phase D): one churn invalidates every element reference into the churned array; the
// first subsequent use is reported.
#[test]
fn test_multiple_element_refs_all_invalidated_by_one_churn() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  first = &arr[0];\n",
      "  second = &arr[1];\n",
      "  churn(&arr);\n",
      "  observe(first);\n",
      "  observe(second);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
first references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 21 (Phase E): the grimoire's `ring_ref` scenario — an element reference used after a
// `damage` call that churns its group is rejected.
#[test]
fn test_ring_ref_used_after_damage_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func damage<r'>(a &[]int in r) mut(r) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ring = &arr[0];\n",
      "  damage(&arr);\n",
      "  observe(ring);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:1:
exported func main() int {
ring references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// Slice 22 (Phase E): the safe companion — a whole-array reference used after the same `damage`
// call is live, and a fuller clean program compiles.
#[test]
fn test_whole_array_ref_after_damage_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func damage<r'>(a &[]int in r) mut(r) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  whole = &arr;\n",
    "  before = &arr[0];\n",
    "  observe(before);\n",
    "  damage(&arr);\n",
    "  observe(whole);\n",
    "  after = &arr[0];\n",
    "  observe(after);\n",
    "  return 0;\n",
    "}\n",
  ));
}
