//! The producer-side churn gate: a function may churn a *parameter's* group only where its own
//! signature declares a `mut(...)` effect covering it. A churn of a fresh *local*'s group needs no
//! declaration — the function owns that group.
//!
//! `grow<r'>(vec &Vec<Entity> in r) mut(r)` is the churn-producer. A function that passes one of its
//! own `&Vec in g` parameters to `grow` churns `g`, so it must declare `mut(g)`; passing a fresh
//! local instead does not.

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
);

fn program(body: &str) -> String {
  format!("{}{}", PREAMBLE, body)
}

// A function that churns one of its parameters' groups (by passing the parameter to a `mut`-declaring
// callee) but does not declare `mut` for that group is rejected at the churning call.
#[test]
fn test_undeclared_param_churn_rejected() {
  assert_borrow_error_renders_with_arrays(
    &program(concat!(
      "func churner<g'>(v &Vec<Entity> in g) {\n",
      "  grow(v);\n",
      "}\n",
      "exported func main() int {\n",
      "  vec = Vec<Entity>(Array<Entity>(3));\n",
      "  churner(&vec);\n",
      "  return 0;\n",
      "}\n",
    )),
    r#"At test:0.vale:12:3:
  grow(v);
this call churns a group reached through a parameter, but the enclosing function does not declare a mut effect for it.
"#,
  );
}

// Declaring `mut(g)` for the churned parameter group makes the same body legal.
#[test]
fn test_declared_param_churn_is_accepted() {
  assert_compiles_clean_with_arrays(&program(concat!(
    "func churner<g'>(v &Vec<Entity> in g) mut(g) {\n",
    "  grow(v);\n",
    "}\n",
    "exported func main() int {\n",
    "  vec = Vec<Entity>(Array<Entity>(3));\n",
    "  churner(&vec);\n",
    "  return 0;\n",
    "}\n",
  )));
}

// Churning a fresh *local*'s group (not a parameter's) needs no declaration.
#[test]
fn test_local_churn_needs_no_declaration() {
  assert_compiles_clean_with_arrays(&program(concat!(
    "func churner<g'>(v &Vec<Entity> in g) {\n",
    "  nv = Vec<Entity>(Array<Entity>(0));\n",
    "  grow(&nv);\n",
    "}\n",
    "exported func main() int {\n",
    "  vec = Vec<Entity>(Array<Entity>(3));\n",
    "  churner(&vec);\n",
    "  return 0;\n",
    "}\n",
  )));
}
