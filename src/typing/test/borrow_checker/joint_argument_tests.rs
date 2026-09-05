use super::util::{assert_borrow_error_renders, assert_compiles_clean};

// Slice 1: passing one local to two params in *distinct* named groups, one of which is a `mut`
// target, aliases two arguments the callee is allowed to believe are disjoint — a borrow error.
#[test]
fn test_alias_same_local_into_distinct_mut_groups_rejected() {
  assert_borrow_error_renders(
    concat!(
      "struct Entity { hp int; }\n",
      "func badpair<r', s'>(a &Entity in r, d &Entity in s) mut(r) { }\n",
      "exported func main() int {\n",
      "  e = Entity(5);\n",
      "  badpair(&e, &e);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:12:
  badpair(&e, &e);
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 2: two distinct groups, but the callee mutates neither — free immutable aliasing, no error.
#[test]
fn test_alias_into_distinct_groups_without_mut_is_clean() {
  assert_compiles_clean(concat!(
    "struct Entity { hp int; }\n",
    "func purepair<r', s'>(a &Entity in r, d &Entity in s) { }\n",
    "exported func main() int {\n",
    "  e = Entity(5);\n",
    "  purepair(&e, &e);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 3: both params share the mutated group `g`, so passing the same local twice is common-group
// aliasing — safe.
#[test]
fn test_common_group_aliasing_is_clean() {
  assert_compiles_clean(concat!(
    "struct Entity { hp int; }\n",
    "func heal<g'>(a &Entity in g, d &Entity in g) mut(g) { }\n",
    "exported func main() int {\n",
    "  e = Entity(5);\n",
    "  heal(&e, &e);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 4: distinct locals into distinct mutated groups do not alias.
#[test]
fn test_distinct_locals_into_distinct_mut_groups_clean() {
  assert_compiles_clean(concat!(
    "struct Entity { hp int; }\n",
    "func badpair<r', s'>(a &Entity in r, d &Entity in s) mut(r) { }\n",
    "exported func main() int {\n",
    "  e1 = Entity(5);\n",
    "  e2 = Entity(6);\n",
    "  badpair(&e1, &e2);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 5: borrowing the *same field* twice into distinct mutated groups aliases through a member
// path.
#[test]
fn test_same_field_alias_rejected() {
  assert_borrow_error_renders(
    concat!(
      "struct Ship { fuel int; }\n",
      "struct Fleet { flagship Ship; escort Ship; }\n",
      "func badships<r', s'>(a &Ship in r, d &Ship in s) mut(r) { }\n",
      "exported func main() int {\n",
      "  f = Fleet(Ship(1), Ship(2));\n",
      "  badships(&f.flagship, &f.flagship);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:6:13:
  badships(&f.flagship, &f.flagship);
Arguments 0 and 1 both borrow into f, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 6: distinct fields of one struct are provably disjoint (the sibling-disjointness lemma), so
// borrowing two different fields into two mutated groups is clean even though they share a root.
#[test]
fn test_sibling_fields_are_disjoint_clean() {
  assert_compiles_clean(concat!(
    "struct Ship { fuel int; }\n",
    "struct Fleet { flagship Ship; escort Ship; }\n",
    "func badships<r', s'>(a &Ship in r, d &Ship in s) mut(r) { }\n",
    "exported func main() int {\n",
    "  f = Fleet(Ship(1), Ship(2));\n",
    "  badships(&f.flagship, &f.escort);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 7: a whole-struct borrow and a borrow of one of its fields are nested (one path a prefix of
// the other), so into distinct mutated groups they alias.
#[test]
fn test_prefix_path_alias_rejected() {
  assert_borrow_error_renders(
    concat!(
      "struct Ship { fuel int; }\n",
      "struct Fleet { flagship Ship; escort Ship; }\n",
      "func badmix<r', s'>(a &Fleet in r, d &Ship in s) mut(r) { }\n",
      "exported func main() int {\n",
      "  f = Fleet(Ship(1), Ship(2));\n",
      "  badmix(&f, &f.flagship);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:6:11:
  badmix(&f, &f.flagship);
Arguments 0 and 1 both borrow into f, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 8: aliasing is checked over every unordered argument pair — here arguments 0 and 2 alias.
#[test]
fn test_nonadjacent_arg_pair_alias_rejected() {
  assert_borrow_error_renders(
    concat!(
      "struct Entity { hp int; }\n",
      "func bad3<r', s', u'>(a &Entity in r, b &Entity in s, c &Entity in u) mut(r) { }\n",
      "exported func main() int {\n",
      "  e = Entity(5);\n",
      "  other = Entity(6);\n",
      "  bad3(&e, &other, &e);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:6:9:
  bad3(&e, &other, &e);
Arguments 0 and 2 both borrow into e, but their parameters are in disjoint mutated groups r and u, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 9: the mutated group can be *either* of the pair — here it is the second group `s`.
#[test]
fn test_mut_on_second_group_triggers() {
  assert_borrow_error_renders(
    concat!(
      "struct Entity { hp int; }\n",
      "func badpair_s<r', s'>(a &Entity in r, d &Entity in s) mut(s) { }\n",
      "exported func main() int {\n",
      "  e = Entity(5);\n",
      "  badpair_s(&e, &e);\n",
      "  return 0;\n",
      "}\n",
    ),
    r#"At test:0.vale:5:14:
  badpair_s(&e, &e);
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}
