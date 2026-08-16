#include <stdint.h>

#include "vtest/Widget.h"
#include "vtest/Widget_n.h"
#include "vtest/Widget_alias.h"
#include "vtest/Widget_dealias.h"

// w is BORROW (extern arg).
// Call _alias (RC +1) then _dealias (RC -1) — net zero. The Widget survives
// through the getter call because the borrow's +1 is still held by the
// Vale caller.
ValeInt vtest_testAliasDealias(vtest_Widget w) {
  vtest_Widget_alias(w);
  vtest_Widget_dealias(w);
  return vtest_Widget_n(w);
}
