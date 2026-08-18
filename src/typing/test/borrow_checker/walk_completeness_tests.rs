use super::util::assert_borrow_error_renders;

// The violation is the same aliasing `badpair(&e, &e)`; these slices vary only *where* it is nested,
// exercising the walk. The rendered diagnostic is identical (it points at the caller `main`), which
// is exactly what confirms the walk found the same violation in each position.
const ALIASING_DIAGNOSTIC: &str = r#"At test:0.vale:3:1:
exported func main() int {
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#;

fn prelude_program(statement: &str) -> String {
  format!(
    "struct Entity {{ hp int; }}\n\
     func badpair<r', s'>(a &Entity in r, d &Entity in s) mut(r) {{ }}\n\
     exported func main() int {{\n  e = Entity(5);\n{statement}\n  return 0;\n}}\n"
  )
}

// Slice 13: the violating call is nested in an inner block.
#[test]
fn test_violation_in_nested_block_caught() {
  assert_borrow_error_renders(
    &prelude_program("  block {\n    badpair(&e, &e);\n  }"),
    ALIASING_DIAGNOSTIC,
  );
}

// Slice 14: the violating call is inside an `if` arm.
#[test]
fn test_violation_in_if_arm_caught() {
  assert_borrow_error_renders(
    &prelude_program("  if (true) {\n    badpair(&e, &e);\n  }"),
    ALIASING_DIAGNOSTIC,
  );
}

// Slice 15: the violating call is inside a `while` body.
#[test]
fn test_violation_in_while_body_caught() {
  assert_borrow_error_renders(
    &prelude_program("  while (false) {\n    badpair(&e, &e);\n  }"),
    ALIASING_DIAGNOSTIC,
  );
}

// Slice 16: the violating call is itself an argument of an outer call, so the walk must descend into
// call arguments. (Distinct fixture — the callee returns a value usable as an argument.)
#[test]
fn test_violation_in_nested_arg_call_caught() {
  assert_borrow_error_renders(
    concat!(
      "struct Entity { hp int; }\n",
      "func badpairi<r', s'>(a &Entity in r, d &Entity in s) int mut(r) { return 0; }\n",
      "func outer(x int) int { return x; }\n",
      "exported func main() int {\n",
      "  e = Entity(5);\n",
      "  outer(badpairi(&e, &e));\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:4:1:
exported func main() int {
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}

// A value-returning violating call, laid out so `main` is on line 3, so these slices share
// ALIASING_DIAGNOSTIC. Each nests the call in a different statement position the walk must descend.
fn value_call_program(statement: &str) -> String {
  format!(
    "struct Entity {{ hp int; }}\n\
     func badpairi<r', s'>(a &Entity in r, d &Entity in s) int mut(r) {{ return 0; }}\n\
     exported func main() int {{\n  e = Entity(5);\n{statement}\n}}\n"
  )
}

// Slice 17: the violating call is the initializer of a `let`.
#[test]
fn test_violation_in_let_initializer_caught() {
  assert_borrow_error_renders(
    &value_call_program("  y = badpairi(&e, &e);\n  return y;"),
    ALIASING_DIAGNOSTIC,
  );
}

// Slice 18: the violating call is the operand of a `return`.
#[test]
fn test_violation_in_return_caught() {
  assert_borrow_error_renders(
    &value_call_program("  return badpairi(&e, &e);"),
    ALIASING_DIAGNOSTIC,
  );
}

// Slice 19: the violating call is the source of a `set`.
#[test]
fn test_violation_in_set_source_caught() {
  assert_borrow_error_renders(
    &value_call_program("  y = 0;\n  set y = badpairi(&e, &e);\n  return y;"),
    ALIASING_DIAGNOSTIC,
  );
}

