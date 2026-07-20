#include <stdint.h>

#include "vtest/cGetA.h"
#include "vtest/cGetB.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

// Vale `int` is 32-bit, so these externs return ValeInt (int32_t). Returning
// int64_t traps under wasm's strict call signatures (native tolerates it).
extern ValeInt vtest_cGetA() {
  int runNumber = incrementIntFile("myfile.bin");
  return 42 * runNumber;
}
extern ValeInt vtest_cGetB() {
  int runNumber = incrementIntFile("myfile.bin");
  return 42 * runNumber;
}
