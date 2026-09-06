//! Ellipsis (`...`) use-after-churn tests. A reference `&T in g...` points *somewhere* inside g's
//! territory; a churn that touches that territory invalidates it. A churn of an unrelated group does
//! not.

use super::util::{assert_borrow_error_renders_with_arrays, assert_compiles_clean_with_arrays};

// A returned `&int in r...` reference (somewhere inside r) is invalidated by a churn of r.
#[test]
fn test_use_ellipsis_return_after_churn_rejected() {
  assert_borrow_error_renders_with_arrays(
    r#"
import v.builtins.arrays.*;
import v.builtins.drop.*;
func churn<r'>(a &[]int in r) mut(r) { }
func peek<r'>(a &[]int in r) &int in r... { return &a[0]; }
func observe<T>(x &T) { }
exported func main() int {
  arr = Array<int>(3);
  ref = peek(&arr);
  churn(&arr);
  observe(ref);
  return 0;
}
"#,
    r#"At test:0.vale:11:11:
  observe(ref);
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// A `&int in r...` reference survives a churn of a *different* group — the churn never touched r.
#[test]
fn test_ellipsis_ref_into_untouched_group_is_clean() {
  assert_compiles_clean_with_arrays(r#"
import v.builtins.arrays.*;
import v.builtins.drop.*;
func churn<r'>(a &[]int in r) mut(r) { }
func peek<r'>(a &[]int in r) &int in r... { return &a[0]; }
func observe<T>(x &T) { }
exported func main() int {
  arr = Array<int>(3);
  other = Array<int>(3);
  ref = peek(&arr);
  churn(&other);
  observe(ref);
  return 0;
}
"#);
}

// A `&int in r...` reference is invalidated by a churn *below* its base — `mut(r[])` churns r's
// elements, which r's territory contains.
#[test]
fn test_ellipsis_ref_invalidated_by_element_churn() {
  assert_borrow_error_renders_with_arrays(
    r#"
import v.builtins.arrays.*;
import v.builtins.drop.*;
func churn_elems<r'>(a &[]int in r) mut(r[]) { }
func peek<r'>(a &[]int in r) &int in r... { return &a[0]; }
func observe<T>(x &T) { }
exported func main() int {
  arr = Array<int>(3);
  ref = peek(&arr);
  churn_elems(&arr);
  observe(ref);
  return 0;
}
"#,
    r#"At test:0.vale:11:11:
  observe(ref);
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// `mut(r...)` churns exactly `mut(r)`: an element reference into r (a child group) is invalidated.
#[test]
fn test_ellipsis_effect_invalidates_child_element() {
  assert_borrow_error_renders_with_arrays(
    r#"
import v.builtins.arrays.*;
import v.builtins.drop.*;
func churn_ellipsis<r'>(a &[]int in r) mut(r...) { }
func observe<T>(x &T) { }
exported func main() int {
  arr = Array<int>(3);
  ref = &arr[0];
  churn_ellipsis(&arr);
  observe(ref);
  return 0;
}
"#,
    r#"At test:0.vale:10:11:
  observe(ref);
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// `mut(r...)` churns exactly `mut(r)`: a reference to the whole array (group r itself) survives.
#[test]
fn test_ellipsis_effect_spares_whole_array() {
  assert_compiles_clean_with_arrays(r#"
import v.builtins.arrays.*;
import v.builtins.drop.*;
func churn_ellipsis<r'>(a &[]int in r) mut(r...) { }
func observe<T>(x &T) { }
exported func main() int {
  arr = Array<int>(3);
  whole = &arr;
  churn_ellipsis(&arr);
  observe(whole);
  return 0;
}
"#);
}

// S1: a churn of an *ancestor* group invalidates a deeper ellipsis reference. `&int in r[]...` points
// somewhere within an element of r; `mut(r)` churns r (above `r[]`), touching that territory.
#[test]
fn test_ancestor_churn_invalidates_nested_ellipsis() {
  assert_borrow_error_renders_with_arrays(
    r#"
import v.builtins.arrays.*;
import v.builtins.drop.*;
func churn<r'>(a &[]int in r) mut(r) { }
func peek_deep<r'>(a &[]int in r) &int in r[]... { return &a[0]; }
func observe<T>(x &T) { }
exported func main() int {
  arr = Array<int>(3);
  ref = peek_deep(&arr);
  churn(&arr);
  observe(ref);
  return 0;
}
"#,
    r#"At test:0.vale:11:11:
  observe(ref);
ref references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}
