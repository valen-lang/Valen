#include <stdint.h>
#include <assert.h>

#include "vtest/Widget.h"
#include "vtest/Widget_alias.h"
#include "vtest/Widget_dealias.h"
#include "vtest/Widget_n.h"
#include "vtest/Widget_ref_eq.h"

// a and b are OWN (moved in), both referencing the same underlying Widget.
// Per @FRMACZ: alias each handle at every pass, and dealias each once we're done.
ValeInt vtest_testAlias(vtest_Widget a, vtest_Widget b) {
  assert(vtest_Widget_ref_eq(vtest_Widget_alias(a), vtest_Widget_alias(b)));
  ValeInt result = vtest_Widget_n(vtest_Widget_alias(a)) + vtest_Widget_n(vtest_Widget_alias(b));
  vtest_Widget_dealias(a);
  vtest_Widget_dealias(b);
  return result / 2;    // both are 42, average == 42
}
