#include <stdint.h>

#include "vtest/Named.h"
#include "vtest/makeNamed.h"
#include "vtest/fixedLabel.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

vtest_Named vtest_cMakeNamed() {
  int64_t runNumber = incrementIntFile("myfile.bin");
  // C can't allocate a Vale str directly under the opaque-handle ABI; get one
  // from an exported Vale factory instead.
  vtest_str label = vtest_fixedLabel();
  return vtest_makeNamed(42 * runNumber, label);
}
