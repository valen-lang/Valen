#include <stdint.h>

#include "vtest/Flamscrankle.h"
#include "vtest/Flamscrankle_alias.h"
#include "vtest/Flamscrankle_dealias.h"
#include "vtest/Flamscrankle_a.h"
#include "vtest/Flamscrankle_c.h"
#include "vtest/Flamscrankle_new.h"
#include "vtest/valeMakeStruct.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

// Original C mutated the returned handle in-place. Under opaque handles that
// capability is gone (deferred to the share-as-mutable arc + setters); the
// structural equivalent is read -> reconstruct via `_new`. Per @FRMACZ: alias x
// at each getter pass, dealias it, and return the fresh `_new` handle (moves out).
vtest_Flamscrankle vtest_cMakeStruct() {
  int runNumber = incrementIntFile("myfile.bin");

  vtest_Flamscrankle x = vtest_valeMakeStruct();                                // owned 1
  ValeInt newA = vtest_Flamscrankle_a(vtest_Flamscrankle_alias(x)) * runNumber;
  ValeInt newC = vtest_Flamscrankle_c(vtest_Flamscrankle_alias(x)) * runNumber;
  vtest_Flamscrankle_dealias(x);
  return vtest_Flamscrankle_new(newA, newC);                                    // moves out
}
