use super::util::{assert_borrow_error_renders, assert_compiles_clean};

// Slice 1: passing one local to two params in *distinct* named groups, one of which is a `mut`
// target, aliases two arguments the callee is allowed to believe are disjoint — a borrow error.
#[test]
fn test_alias_same_local_into_distinct_mut_groups_rejected() {
  assert_borrow_error_renders(
    r#"
struct Entity { hp int; }
func badpair<r', s'>(a &Entity in r, d &Entity in s) mut(r) { }
exported func main() int {
  e = Entity(5);
  badpair(&e, &e);
  return 0;
}
"#,
    r#"At test:0.vale:6:12:
  badpair(&e, &e);
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 2: two distinct groups, but the callee mutates neither — free immutable aliasing, no error.
#[test]
fn test_alias_into_distinct_groups_without_mut_is_clean() {
  assert_compiles_clean(r#"
struct Entity { hp int; }
func purepair<r', s'>(a &Entity in r, d &Entity in s) { }
exported func main() int {
  e = Entity(5);
  purepair(&e, &e);
  return 0;
}
"#);
}

// Slice 3: both params share the mutated group `g`, so passing the same local twice is common-group
// aliasing — safe.
#[test]
fn test_common_group_aliasing_is_clean() {
  assert_compiles_clean(r#"
struct Entity { hp int; }
func heal<g'>(a &Entity in g, d &Entity in g) mut(g) { }
exported func main() int {
  e = Entity(5);
  heal(&e, &e);
  return 0;
}
"#);
}

// Slice 4: distinct locals into distinct mutated groups do not alias.
#[test]
fn test_distinct_locals_into_distinct_mut_groups_clean() {
  assert_compiles_clean(r#"
struct Entity { hp int; }
func badpair<r', s'>(a &Entity in r, d &Entity in s) mut(r) { }
exported func main() int {
  e1 = Entity(5);
  e2 = Entity(6);
  badpair(&e1, &e2);
  return 0;
}
"#);
}

// Slice 5: borrowing the *same field* twice into distinct mutated groups aliases through a member
// path.
#[test]
fn test_same_field_alias_rejected() {
  assert_borrow_error_renders(
    r#"
struct Ship { fuel int; }
struct Fleet { flagship Ship; escort Ship; }
func badships<r', s'>(a &Ship in r, d &Ship in s) mut(r) { }
exported func main() int {
  f = Fleet(Ship(1), Ship(2));
  badships(&f.flagship, &f.flagship);
  return 0;
}
"#,
    r#"At test:0.vale:7:13:
  badships(&f.flagship, &f.flagship);
Arguments 0 and 1 both borrow into f, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 6: distinct fields of one struct are provably disjoint (the sibling-disjointness lemma), so
// borrowing two different fields into two mutated groups is clean even though they share a root.
#[test]
fn test_sibling_fields_are_disjoint_clean() {
  assert_compiles_clean(r#"
struct Ship { fuel int; }
struct Fleet { flagship Ship; escort Ship; }
func badships<r', s'>(a &Ship in r, d &Ship in s) mut(r) { }
exported func main() int {
  f = Fleet(Ship(1), Ship(2));
  badships(&f.flagship, &f.escort);
  return 0;
}
"#);
}

// Slice 7: a whole-struct borrow and a borrow of one of its fields are nested (one path a prefix of
// the other), so into distinct mutated groups they alias.
#[test]
fn test_prefix_path_alias_rejected() {
  assert_borrow_error_renders(
    r#"
struct Ship { fuel int; }
struct Fleet { flagship Ship; escort Ship; }
func badmix<r', s'>(a &Fleet in r, d &Ship in s) mut(r) { }
exported func main() int {
  f = Fleet(Ship(1), Ship(2));
  badmix(&f, &f.flagship);
  return 0;
}
"#,
    r#"At test:0.vale:7:11:
  badmix(&f, &f.flagship);
Arguments 0 and 1 both borrow into f, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 8: aliasing is checked over every unordered argument pair — here arguments 0 and 2 alias.
#[test]
fn test_nonadjacent_arg_pair_alias_rejected() {
  assert_borrow_error_renders(
    r#"
struct Entity { hp int; }
func bad3<r', s', u'>(a &Entity in r, b &Entity in s, c &Entity in u) mut(r) { }
exported func main() int {
  e = Entity(5);
  other = Entity(6);
  bad3(&e, &other, &e);
  return 0;
}
"#,
    r#"At test:0.vale:7:9:
  bad3(&e, &other, &e);
Arguments 0 and 2 both borrow into e, but their parameters are in disjoint mutated groups r and u, which the callee may treat as non-aliasing.
"#,
  );
}

// Slice 9: the mutated group can be *either* of the pair — here it is the second group `s`.
#[test]
fn test_mut_on_second_group_triggers() {
  assert_borrow_error_renders(
    r#"
struct Entity { hp int; }
func badpair_s<r', s'>(a &Entity in r, d &Entity in s) mut(s) { }
exported func main() int {
  e = Entity(5);
  badpair_s(&e, &e);
  return 0;
}
"#,
    r#"At test:0.vale:6:14:
  badpair_s(&e, &e);
Arguments 0 and 1 both borrow into e, but their parameters are in disjoint mutated groups r and s, which the callee may treat as non-aliasing.
"#,
  );
}
