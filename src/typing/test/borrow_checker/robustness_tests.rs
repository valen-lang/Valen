use super::util::{assert_borrow_error_renders, assert_compiles_clean};

// Slice 17: among several sibling calls only one is unsafe; the walk checks every call, so the unsafe
// one is caught and the innocuous ones do not interfere.
#[test]
fn test_only_the_unsafe_call_among_many_is_flagged() {
  assert_borrow_error_renders(
    r#"
struct Entity { hp int; }
func badpair<r', s'>(a &Entity in r, d &Entity in s) mut(r) { }
func safe(x int) int { return x; }
exported func main() int {
  e = Entity(5);
  safe(1);
  badpair(&e, &e);
  safe(2);
  return 0;
}
"#,
    r#"At test:0.vale:8:12:
  badpair(&e, &e);
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 18: a call mixing a `&… in g` group parameter with a plain by-value parameter is not a false
// positive — a non-group parameter forms no group pair.
#[test]
fn test_mixed_group_and_plain_params_no_false_positive() {
  assert_compiles_clean(r#"
struct Entity { hp int; }
func mixed<r'>(a &Entity in r, b int) mut(r) { }
exported func main() int {
  e = Entity(5);
  mixed(&e, 7);
  return 0;
}
"#);
}

// Slice 19: the same generic callee is safe at one call site and unsafe at another; the verdict is
// per call site, so only the unsafe site is flagged.
#[test]
fn test_same_callee_safe_and_unsafe_sites() {
  assert_borrow_error_renders(
    r#"
struct Entity { hp int; }
func badpair<r', s'>(a &Entity in r, d &Entity in s) mut(r) { }
exported func main() int {
  e = Entity(5);
  e2 = Entity(6);
  badpair(&e, &e2);
  badpair(&e, &e);
  return 0;
}
"#,
    r#"At test:0.vale:8:12:
  badpair(&e, &e);
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}
