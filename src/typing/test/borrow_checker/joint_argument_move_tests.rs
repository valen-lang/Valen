use super::util::{assert_borrow_error_renders, assert_compiles_clean};

// Slice 10: one argument moves a local while a sibling argument borrows into it. The move destroys
// the group the borrow points into, so the borrow would dangle. (Borrow argument first, so it
// survives typing's unstackify check.)
#[test]
fn test_borrow_into_moved_local_rejected() {
  assert_borrow_error_renders(
    concat!(
      "struct Holder { n int; }\n",
      "func consume<g'>(a &Holder in g, b Holder) { }\n",
      "exported func main() int {\n",
      "  h = Holder(1);\n",
      "  consume(&h, ^h);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:3:1:
exported func main() int {
Argument 0 borrows into h, but argument 1 moves it, so the borrow would dangle.
"#,
  );
}

// Slice 11: the borrow reaches into the moved local through a field — `&h.ship` is within the
// territory of the moved `h`, so it dangles just the same.
#[test]
fn test_field_borrow_into_moved_local_rejected() {
  assert_borrow_error_renders(
    concat!(
      "struct Ship { fuel int; }\n",
      "struct Holder { ship Ship; }\n",
      "func consume2<g'>(a &Ship in g, b Holder) { }\n",
      "exported func main() int {\n",
      "  h = Holder(Ship(1));\n",
      "  consume2(&h.ship, ^h);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:4:1:
exported func main() int {
Argument 0 borrows into h, but argument 1 moves it, so the borrow would dangle.
"#,
  );
}

// Slice 12: borrowing a *different* local than the one being moved is disjoint and clean.
#[test]
fn test_borrow_into_other_local_with_move_is_clean() {
  assert_compiles_clean(concat!(
    "struct Holder { n int; }\n",
    "func consume<g'>(a &Holder in g, b Holder) { }\n",
    "exported func main() int {\n",
    "  h = Holder(1);\n",
    "  y = Holder(2);\n",
    "  consume(&y, ^h);\n",
    "  return 0;\n",
    "}\n",
  ));
}
