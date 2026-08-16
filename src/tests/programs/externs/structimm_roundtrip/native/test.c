#include <stdint.h>

#include "vtest/Widget.h"

// w is OWN (moved in). Return it, moving ownership straight back to Vale — no
// alias or dealias needed at the boundary.
vtest_Widget vtest_roundtrip(vtest_Widget w) {
  return w;
}
