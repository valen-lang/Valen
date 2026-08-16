#include <stdint.h>

#include "vtest/cGetInt.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

// Vale `int` is 32-bit, so the extern returns ValeInt (int32_t). Returning
// int64_t traps under wasm's strict call signatures (native tolerates it).
extern ValeInt vtest_cGetInt() {
  int runNumber = incrementIntFile("myfile.bin");
  return runNumber * 10;
}
