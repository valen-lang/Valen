use super::util::assert_compiles_clean;

// Slice 20: the canonical common-group `attack` — both borrows are in the one mutated group `r`, so
// passing the same entity twice is safe (the callee already treats them as one group).
#[test]
fn test_common_group_attack_aliasing_call_is_safe() {
  assert_compiles_clean(r#"
struct Entity { hp int; }
func attack<r'>(a &Entity in r, d &Entity in r) mut(r) { }
exported func main() int {
  e = Entity(5);
  attack(&e, &e);
  return 0;
}
"#);
}

// Slice 21: the disjoint-fields `attack2` mutates two distinct groups `r` and `s`, but the arguments
// are two sibling fields of one fleet, which are provably disjoint — safe.
#[test]
fn test_disjoint_fields_attack_is_safe() {
  assert_compiles_clean(r#"
struct Ship { fuel int; }
struct Fleet { flagship Ship; escort Ship; }
func attack2<r', s'>(a &Ship in r, d &Ship in s) mut(r) mut(s) { }
exported func main() int {
  fleet = Fleet(Ship(1), Ship(2));
  attack2(&fleet.flagship, &fleet.escort);
  return 0;
}
"#);
}

// Slice 22 (capstone): `attack`'s own body mutates both borrows' members (no structural op), and
// `main` calls it with both distinct and aliasing arguments. The whole program borrow-checks clean
// end-to-end — member writes are not call violations, and common-group aliasing is safe.
#[test]
fn test_full_attack_program_is_safe() {
  assert_compiles_clean(r#"
struct Entity { hp int; }
func attack<r'>(a &Entity in r, d &Entity in r) mut(r) {
  set a.hp = 1;
  set d.hp = 2;
}
exported func main() int {
  e = Entity(5);
  e2 = Entity(6);
  attack(&e, &e2);
  attack(&e, &e);
  return 0;
}
"#);
}
