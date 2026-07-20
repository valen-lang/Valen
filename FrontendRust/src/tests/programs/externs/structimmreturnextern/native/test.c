#include <stdint.h>
#include <stdio.h>

#include "vtest/Flamscrankle.h"
#include "vtest/Flamscrankle_new.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

vtest_Flamscrankle vtest_cMakeStruct() {
  int runNumber = incrementIntFile("myfile.bin");

  printf("run number: %d\n", runNumber);
  vtest_Flamscrankle flam = vtest_Flamscrankle_new(37 * runNumber, 5 * runNumber);
  printf("returning a flam, handle size: %zu\n", sizeof(vtest_Flamscrankle));
  return flam;
}
