#include <stdint.h>

#include "vtest/Flamscrankle.h"
#include "vtest/Flamscrankle_alias.h"
#include "vtest/Flamscrankle_dealias.h"
#include "vtest/Flamscrankle_a.h"
#include "vtest/Flamscrankle_c.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

// flam is OWN (moved in). Per @FRMACZ: alias each handle at every pass, and
// dealias it once we're done.
ValeInt vtest_extFunc(vtest_Flamscrankle flam) {
  int runNumber = incrementIntFile("myfile.bin");
  ValeInt result = (vtest_Flamscrankle_a(vtest_Flamscrankle_alias(flam))
                    + vtest_Flamscrankle_c(vtest_Flamscrankle_alias(flam))) * runNumber;
  vtest_Flamscrankle_dealias(flam);
  return result;
}
