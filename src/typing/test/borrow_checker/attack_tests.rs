use super::util::assert_compiles_clean;

// Slice 20: the canonical common-group `attack` — both borrows are in the one mutated group `r`, so
// passing the same entity twice is safe (the callee already treats them as one group).
#[test]
fn test_common_group_attack_aliasing_call_is_safe() {
  assert_compiles_clean(concat!(
    "struct Entity { hp int; }\n",
    "func attack<r'>(a &Entity in r, d &Entity in r) mut(r) { }\n",
    "exported func main() int {\n",
    "  e = Entity(5);\n",
    "  attack(&e, &e);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 21: the disjoint-fields `attack2` mutates two distinct groups `r` and `s`, but the arguments
// are two sibling fields of one fleet, which are provably disjoint — safe.
#[test]
fn test_disjoint_fields_attack_is_safe() {
  assert_compiles_clean(concat!(
    "struct Ship { fuel int; }\n",
    "struct Fleet { flagship Ship; escort Ship; }\n",
    "func attack2<r', s'>(a &Ship in r, d &Ship in s) mut(r, s) { }\n",
    "exported func main() int {\n",
    "  fleet = Fleet(Ship(1), Ship(2));\n",
    "  attack2(&fleet.flagship, &fleet.escort);\n",
    "  return 0;\n",
    "}\n",
  ));
}

// Slice 22 (capstone): `attack`'s own body mutates both borrows' members (no structural op), and
// `main` calls it with both distinct and aliasing arguments. The whole program borrow-checks clean
// end-to-end — member writes are not call violations, and common-group aliasing is safe.
#[test]
fn test_full_attack_program_is_safe() {
  assert_compiles_clean(concat!(
    "struct Entity { hp int; }\n",
    "func attack<r'>(a &Entity in r, d &Entity in r) mut(r) {\n",
    "  set a.hp = 1;\n",
    "  set d.hp = 2;\n",
    "}\n",
    "exported func main() int {\n",
    "  e = Entity(5);\n",
    "  e2 = Entity(6);\n",
    "  attack(&e, &e2);\n",
    "  attack(&e, &e);\n",
    "  return 0;\n",
    "}\n",
  ));
}
