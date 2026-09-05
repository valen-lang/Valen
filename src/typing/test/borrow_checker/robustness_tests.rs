use super::util::{assert_borrow_error_renders, assert_compiles_clean};

// Slice 17: among several sibling calls only one is unsafe; the walk checks every call, so the unsafe
// one is caught and the innocuous ones do not interfere.
#[test]
fn test_only_the_unsafe_call_among_many_is_flagged() {
  assert_borrow_error_renders(
    concat!(
      "struct Entity { hp int; }\n",
      "func badpair<r', s'>(a &Entity in r, d &Entity in s) mut(r) { }\n",
      "func safe(x int) int { return x; }\n",
      "exported func main() int {\n",
      "  e = Entity(5);\n",
      "  safe(1);\n",
      "  badpair(&e, &e);\n",
      "  safe(2);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:7:12:
  badpair(&e, &e);
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 18: a call mixing a `&… in g` group parameter with a plain by-value parameter is not a false
// positive — a non-group parameter forms no group pair.
#[test]
fn test_mixed_group_and_plain_params_no_false_positive() {
  assert_compiles_clean(concat!(
    "struct Entity { hp int; }\n",
    "func mixed<r'>(a &Entity in r, b int) mut(r) { }\n",
    "exported func main() int {\n",
    "  e = Entity(5);\n",
    "  mixed(&e, 7);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 19: the same generic callee is safe at one call site and unsafe at another; the verdict is
// per call site, so only the unsafe site is flagged.
#[test]
fn test_same_callee_safe_and_unsafe_sites() {
  assert_borrow_error_renders(
    concat!(
      "struct Entity { hp int; }\n",
      "func badpair<r', s'>(a &Entity in r, d &Entity in s) mut(r) { }\n",
      "exported func main() int {\n",
      "  e = Entity(5);\n",
      "  e2 = Entity(6);\n",
      "  badpair(&e, &e2);\n",
      "  badpair(&e, &e);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:7:12:
  badpair(&e, &e);
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}
