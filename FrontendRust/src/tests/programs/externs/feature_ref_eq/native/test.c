#include <stdint.h>
#include <assert.h>

#include "vtest/Widget.h"
#include "vtest/Widget_alias.h"
#include "vtest/Widget_dealias.h"
#include "vtest/Widget_ref_eq.h"

// Per @FRMACZ: alias each handle at every pass (ref_eq consumes both operands),
// and dealias each handle we own once we're done.
ValeInt vtest_testRefEq(vtest_Widget a, vtest_Widget b, vtest_Widget c) {
  assert(vtest_Widget_ref_eq(vtest_Widget_alias(a), vtest_Widget_alias(a)));   // handle to itself
  assert(vtest_Widget_ref_eq(vtest_Widget_alias(a), vtest_Widget_alias(b)));   // same object
  assert(!vtest_Widget_ref_eq(vtest_Widget_alias(a), vtest_Widget_alias(c)));  // different objects
  vtest_Widget_dealias(a);
  vtest_Widget_dealias(b);
  vtest_Widget_dealias(c);
  return 42;
}
