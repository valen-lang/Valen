#include <stdint.h>
#include <assert.h>

#include "vtest/Flamscrankle.h"
#include "vtest/Bogglewoggle.h"
#include "vtest/Spigglewigget.h"
#include "vtest/Flamscrankle_alias.h"
#include "vtest/Flamscrankle_dealias.h"
#include "vtest/Flamscrankle_x.h"
#include "vtest/Flamscrankle_y.h"
#include "vtest/Flamscrankle_b.h"
#include "vtest/Bogglewoggle_alias.h"
#include "vtest/Bogglewoggle_dealias.h"
#include "vtest/Bogglewoggle_x.h"
#include "vtest/Bogglewoggle_s.h"
#include "vtest/Spigglewigget_alias.h"
#include "vtest/Spigglewigget_dealias.h"
#include "vtest/Spigglewigget_x.h"
#include "vtest/Spigglewigget_y.h"
#include "vtest/Spigglewigget_z.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

// Per @FRMACZ: alias each handle at every pass into a getter, and dealias each
// handle we own once we're done.
ValeInt vtest_extFunc(vtest_Flamscrankle flam) {
  int runNumber = incrementIntFile("myfile.bin");

  // Handle-layout probe: a concrete-kind handle is 8 bytes. sizeof doesn't
  // consume anything.
  assert(sizeof(vtest_Flamscrankle) == 8);
  assert(sizeof(vtest_Bogglewoggle) == 8);
  assert(sizeof(vtest_Spigglewigget) == 8);

  assert(vtest_Flamscrankle_x(vtest_Flamscrankle_alias(flam)) == 7);

  vtest_Bogglewoggle b = vtest_Flamscrankle_b(vtest_Flamscrankle_alias(flam));  // b owned 1
  vtest_Spigglewigget s = vtest_Bogglewoggle_s(vtest_Bogglewoggle_alias(b));    // s owned 1

  ValeInt result =
      vtest_Flamscrankle_x(vtest_Flamscrankle_alias(flam)) +
      vtest_Spigglewigget_x(vtest_Spigglewigget_alias(s)) +
      vtest_Spigglewigget_y(vtest_Spigglewigget_alias(s)) +
      vtest_Spigglewigget_z(vtest_Spigglewigget_alias(s)) +
      vtest_Bogglewoggle_x(vtest_Bogglewoggle_alias(b)) +
      vtest_Flamscrankle_y(vtest_Flamscrankle_alias(flam));
  assert(result == 42);

  vtest_Flamscrankle_dealias(flam);
  vtest_Bogglewoggle_dealias(b);
  vtest_Spigglewigget_dealias(s);

  return result * runNumber;
}
