//! End-to-end: the backend emits LLVM `noalias` on a borrow parameter the borrow checker proved is
//! the sole reference into its group, and omits it where two parameters share a group. Asserts against
//! the pre-optimization dump (`build.ll`) so a redundant-attribute pass can't hide the result.

use crate::end_to_end_tests::compile_inline;
use std::fs;

/// The `define` line for the function whose mangled LLVM name contains `needle`.
fn define_line<'a>(ll: &'a str, needle: &str) -> &'a str {
  ll.lines()
    .find(|l| l.contains("define") && l.contains(needle))
    .unwrap_or_else(|| panic!("no `define` line for `{needle}` in:\n{ll}"))
}

#[test]
fn sole_borrow_param_gets_noalias_same_group_does_not() {
  let cp = compile_inline(
    r#"
struct Spaceship { fuel int; }
func getFuel(a &Spaceship) int {
  return __copy_prim(a.fuel);
}
func pairSum<g'>(a &Spaceship in g, b &Spaceship in g) int {
  return __copy_prim(a.fuel);
}
exported func main() int {
  ship = Spaceship(42);
  other = Spaceship(7);
  return (&ship).getFuel() + pairSum(&ship, &other);
}
"#,
    |opts| opts.print_llvmir = true,
  );
  let ll = fs::read_to_string(cp.cwd.join("build.ll")).expect("read build.ll");

  // The sole-borrow parameter is the only way to reach its group, so it is restrict.
  assert!(
    define_line(&ll, "getFuel").contains("noalias"),
    "getFuel's borrow param should be noalias:\n{}",
    define_line(&ll, "getFuel"),
  );
  // Two parameters in the same group may alias each other, so neither is noalias.
  assert!(
    !define_line(&ll, "pairSum").contains("noalias"),
    "pairSum's same-group params must not be noalias:\n{}",
    define_line(&ll, "pairSum"),
  );
}
