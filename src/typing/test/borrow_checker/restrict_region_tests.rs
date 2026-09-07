//! Restrict regions — the block-scoped half of the aliasing analysis. Where one reference is the sole
//! live reference into its group across a span, that span is a "restrict region"; the backend emits
//! `!alias.scope`/`!noalias` metadata on the accesses inside it (Phase B). A region is pinned in a test
//! by a bare integer landmark (`103;`) placed inside it, so the assertion survives node renumbering.

use super::util::assert_restrict_region;

// `a` and `b` share group `g`, so neither is whole-function `noalias`. But in the trailing span — marked
// by the bare `103` — only `a` reaches into `g`, and `nothing()` doesn't touch `g`, so `a` is restrict
// there: two stores through `a` across one opaque call. `g` is the scope; nothing else is live, so the
// disjoint set is empty.
#[test]
fn shared_group_param_is_restrict_in_trailing_region() {
  assert_restrict_region(
    r#"
struct Ship { fuel int; }
func nothing() { }
func do_things<g'>(a &Ship in g, b &Ship in g) mut(g) {
  set a.fuel = 1;
  set b.fuel = 2;
  set a.fuel = 3;
  103;
  nothing();
  set a.fuel = 4;
}
exported func main() int {
  s1 = Ship(1);
  s2 = Ship(2);
  do_things(&s1, &s2);
  return 0;
}
"#,
    "do_things",
    103,
    "g",
    &[],
    2,
    1,
  );
}
