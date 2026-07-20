#include <stdint.h>

#include "vtest/Outer.h"
#include "vtest/Inner.h"
#include "vtest/Outer_alias.h"
#include "vtest/Outer_dealias.h"
#include "vtest/Outer_name.h"
#include "vtest/Outer_inner.h"
#include "vtest/Inner_alias.h"
#include "vtest/Inner_dealias.h"
#include "vtest/Inner_x.h"

// Per @FRMACZ: alias each handle at every pass into an accessor, and dealias
// each handle we own once we're done.
ValeInt vtest_testGetters(vtest_Outer o) {
  ValeInt n = vtest_Outer_name(vtest_Outer_alias(o));
  vtest_Inner i = vtest_Outer_inner(vtest_Outer_alias(o));   // i is a NEW owned ref
  vtest_Outer_dealias(o);
  ValeInt x = vtest_Inner_x(vtest_Inner_alias(i));
  vtest_Inner_dealias(i);
  return n + x;                                               // 10 + 32 == 42
}
