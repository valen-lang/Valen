//! Two references in the same declared group may alias, so a churn through one must invalidate a
//! child-group borrow reached through the other. `attack(a &Vec<Entity> in r, t &Vec<Entity> in r)`
//! takes both parameters in group `r`, so `a` and `t` may be the same vector: churning `t` while a
//! borrow into `a`'s elements is live must be rejected. Churning a *fresh local* vector (its own
//! group, not `r`) instead is safe.
//!
//! The element borrow `e = &a.data[0]` stands in for a `for component in a` iteration, which holds a
//! child-group borrow across the loop body — `for`-over-a-collection is not built yet.
//!
//! `test_churn_sibling_param_in_same_group_rejected` is the rung-3 target and is RED: the checker
//! currently roots churn-invalidation by local, so a churn of `t` does not match a borrow rooted in
//! `a` even though both are in group `r`. It goes green once invalidation is matched by group.

use super::util::{assert_borrow_error_renders_with_arrays, assert_compiles_clean_with_arrays};

const PREAMBLE: &str = concat!(
  "import v.builtins.arrays.*;\n",
  "import v.builtins.drop.*;\n",
  "struct Entity { id int; hp int; }\n",
  "#!DeriveStructDrop\n",
  "struct Vec<T> { data []T; }\n",
  "func drop<T>(v Vec<T>) where func drop(T)void {\n",
  "  [data] = ^v;\n",
  "  drop(^data);\n",
  "}\n",
  "func grow<r'>(vec &Vec<Entity> in r) mut(r) { }\n",
  "func observe<T>(x &T) { }\n",
);

fn program(body: &str) -> String {
  format!("{}{}", PREAMBLE, body)
}

// The bad case: `a` and `t` share group `r` (may alias), so churning `t` while a borrow into `a`'s
// elements is live must be rejected. RED until invalidation is matched by group rather than local.
#[test]
fn test_churn_sibling_param_in_same_group_rejected() {
  assert_borrow_error_renders_with_arrays(
    &program(concat!(
      "func attack<r'>(a &Vec<Entity> in r, t &Vec<Entity> in r) mut(r) {\n",
      "  e = &a.data[0];\n",
      "  grow(t);\n",
      "  observe(e);\n",
      "}\n",
      "exported func main() int {\n",
      "  v = Vec<Entity>(Array<Entity>(3));\n",
      "  attack(&v, &v);\n",
      "  return 0;\n",
      "}\n",
    )),
    r#"At test:0.vale:15:11:
  observe(e);
e references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}

// The good case: churn a FRESH local vector (its own group, distinct from `r`) while the borrow into
// `a` is live. `a`'s element borrow cannot alias the local, so it stays live — accepted.
#[test]
fn test_churn_fresh_local_group_is_accepted() {
  assert_compiles_clean_with_arrays(&program(concat!(
    "func attack<r'>(a &Vec<Entity> in r, t &Vec<Entity> in r) {\n",
    "  nv = Vec<Entity>(Array<Entity>(0));\n",
    "  e = &a.data[0];\n",
    "  grow(&nv);\n",
    "  observe(e);\n",
    "}\n",
    "exported func main() int {\n",
    "  v = Vec<Entity>(Array<Entity>(3));\n",
    "  attack(&v, &v);\n",
    "  return 0;\n",
    "}\n",
  )));
}

// Control: churning `a` itself (the borrow's own root) is already rejected, which shows the checker
// runs on a group-parameterized function's body — so the red test above is a genuine group-aliasing
// gap, not the checker failing to run.
#[test]
fn test_churn_same_param_rejected() {
  assert_borrow_error_renders_with_arrays(
    &program(concat!(
      "func attack<r'>(a &Vec<Entity> in r, t &Vec<Entity> in r) mut(r) {\n",
      "  e = &a.data[0];\n",
      "  grow(a);\n",
      "  observe(e);\n",
      "}\n",
      "exported func main() int {\n",
      "  v = Vec<Entity>(Array<Entity>(3));\n",
      "  attack(&v, &v);\n",
      "  return 0;\n",
      "}\n",
    )),
    r#"At test:0.vale:15:11:
  observe(e);
e references an array element, which a preceding churn of its group may have moved or deleted, so it can't be used here.
"#,
  );
}
