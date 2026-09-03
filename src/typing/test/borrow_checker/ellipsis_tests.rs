//! Ellipsis (`...`) use-after-churn tests. A reference `&T in g...` points *somewhere* inside g's
//! territory; a churn that touches that territory invalidates it. A churn of an unrelated group does
//! not.

use super::util::{assert_borrow_error_renders_with_arrays, assert_compiles_clean_with_arrays};

// A returned `&int in r...` reference (somewhere inside r) is invalidated by a churn of r.
#[test]
fn test_use_ellipsis_return_after_churn_rejected() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func peek<r'>(a &[]int in r) &int in r... { return &a[0]; }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = peek(&arr);\n",
      "  churn(&arr);\n",
      "  observe(ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:10:11:
  observe(ref);
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// A `&int in r...` reference survives a churn of a *different* group — the churn never touched r.
#[test]
fn test_ellipsis_ref_into_untouched_group_is_clean() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn<r'>(a &[]int in r) mut(r) { }\n",
    "func peek<r'>(a &[]int in r) &int in r... { return &a[0]; }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  other = Array<int>(3);\n",
    "  ref = peek(&arr);\n",
    "  churn(&other);\n",
    "  observe(ref);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// A `&int in r...` reference is invalidated by a churn *below* its base — `mut(r[])` churns r's
// elements, which r's territory contains.
#[test]
fn test_ellipsis_ref_invalidated_by_element_churn() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn_elems<r'>(a &[]int in r) mut(r[]) { }\n",
      "func peek<r'>(a &[]int in r) &int in r... { return &a[0]; }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = peek(&arr);\n",
      "  churn_elems(&arr);\n",
      "  observe(ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:10:11:
  observe(ref);
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// `mut(r...)` churns exactly `mut(r)`: an element reference into r (a child group) is invalidated.
#[test]
fn test_ellipsis_effect_invalidates_child_element() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn_ellipsis<r'>(a &[]int in r) mut(r...) { }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = &arr[0];\n",
      "  churn_ellipsis(&arr);\n",
      "  observe(ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:9:11:
  observe(ref);
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// `mut(r...)` churns exactly `mut(r)`: a reference to the whole array (group r itself) survives.
#[test]
fn test_ellipsis_effect_spares_whole_array() {
  assert_compiles_clean_with_arrays(concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "func churn_ellipsis<r'>(a &[]int in r) mut(r...) { }\n",
    "func observe<T>(x &T) { }\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  whole = &arr;\n",
    "  churn_ellipsis(&arr);\n",
    "  observe(whole);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// S1: a churn of an *ancestor* group invalidates a deeper ellipsis reference. `&int in r[]...` points
// somewhere within an element of r; `mut(r)` churns r (above `r[]`), touching that territory.
#[test]
fn test_ancestor_churn_invalidates_nested_ellipsis() {
  assert_borrow_error_renders_with_arrays(
    concat!(
      "import v.builtins.arrays.*;\n",
      "import v.builtins.drop.*;\n",
      "func churn<r'>(a &[]int in r) mut(r) { }\n",
      "func peek_deep<r'>(a &[]int in r) &int in r[]... { return &a[0]; }\n",
      "func observe<T>(x &T) { }\n",
      "exported func main() int {\n",
      "  arr = Array<int>(3);\n",
      "  ref = peek_deep(&arr);\n",
      "  churn(&arr);\n",
      "  observe(ref);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:10:11:
  observe(ref);
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}
