#include <stdint.h>

#include "vtest/Named.h"
#include "vtest/makeNamed.h"
#include "vtest/fixedLabel.h"

// incrementIntFile persists a counter to `filename`, returning the pre-increment
// value +1 on each call, so successive record runs observe distinct values (see
// AASETR). Defined in Backend/test_builtins/testbuiltins.c, auto-linked.
int64_t incrementIntFile(const char* filename);

vtest_Named vtest_cMakeNamed() {
  int64_t runNumber = incrementIntFile("myfile.bin");
  // C can't allocate a Vale str directly under the opaque-handle ABI; get one
  // from an exported Vale factory instead.
  vtest_str label = vtest_fixedLabel();
  return vtest_makeNamed(runNumber, label);
}
