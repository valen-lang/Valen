//! Held-register tests. A held register is a reference produced mid-expression and waiting in a
//! register to be passed to a call — like `get(&arr)` in `use2(get(&arr), churn(&arr))`, which is
//! never bound to a named local. If a sibling argument churns the group it points into while it waits,
//! consuming it in the outer call is a use-after-churn. Unlike `use_after_churn_tests`' named-local
//! cases, the reference here is an unnamed temporary — the case the borrowing-design doc's
//! "Held register" section calls out specifically. See `docs/architecture/borrowing-design.md`.

use super::util::{assert_borrow_error_renders_with_arrays, assert_compiles_clean_with_arrays};

// A held register (the unnamed result of `get(&arr)`) is invalidated by a sibling argument that churns
// its group before the outer call consumes it. `get` returns `&int in g[]` (a reference into `arr`'s
// elements); `churn_ret` declares `mut(g)`; evaluating it for the second argument churns `arr` while
// the first argument's reference waits in a register, so `use2` consumes a dangling reference.
#[test]
fn test_held_call_result_invalidated_by_sibling_arg_churn_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func get<g'>(a &[]int in g) &int in g[] { return &a[0]; }\n",
      "func churn_ret<g'>(a &[]int in g) int mut(g) { return 0; }\n",
      "func use2<T>(a &T, b int) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  use2(get(&arr), churn_ret(&arr));\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:8:8:
  use2(get(&arr), churn_ret(&arr));
This reference into an array element is held while a sibling argument churns its group, which may have moved or deleted the element, so it can't be passed here.
"#,
  );
}

// Clean control: the held register points into a group that is never churned. `churn_ret(&other)`
// churns a different array, so the reference waiting in the register stays live and `use2` consumes it
// safely.
#[test]
fn test_held_call_result_into_untouched_group_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func get<g'>(a &[]int in g) &int in g[] { return &a[0]; }\n",
    "func churn_ret<g'>(a &[]int in g) int mut(g) { return 0; }\n",
    "func use2<T>(a &T, b int) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  other = Array<int>(3);\n",
    "  use2(get(&arr), churn_ret(&other));\n",
    "  return 0;\n",
    "}\n",
  ));
}
