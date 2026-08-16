#include <stdint.h>

#include "vtest/Bogglewoggle.h"
#include "vtest/Flamscrankle.h"
#include "vtest/expFunc.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

// flam is BORROW (extern arg) — pass-through to another export.
ValeInt vtest_extFunc(vtest_Flamscrankle flam) {
  int runNumber = incrementIntFile("myfile.bin");
  return vtest_expFunc(flam) * runNumber;
}
