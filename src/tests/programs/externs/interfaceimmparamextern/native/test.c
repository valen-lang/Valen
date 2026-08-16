#include <stdint.h>
#include <stdlib.h>

#include "vtest/IShip.h"
#include "vtest/Spaceship.h"
#include "vtest/Seaship.h"
#include "vtest/IShip_alias.h"
#include "vtest/IShip_dealias.h"
#include "vtest/IShip_typeTag.h"
#include "vtest/IShip_asSpaceship.h"
#include "vtest/IShip_asSeaship.h"
#include "vtest/Spaceship_alias.h"
#include "vtest/Spaceship_dealias.h"
#include "vtest/Spaceship_fuel.h"
#include "vtest/Seaship_alias.h"
#include "vtest/Seaship_dealias.h"
#include "vtest/Seaship_leftFuel.h"
#include "vtest/Seaship_rightFuel.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

// Per @FRMACZ: alias each handle at every pass into an accessor, and dealias
// each handle we own once we're done.
ValeInt vtest_cGetShipFuel(vtest_IShip s) {
  int runNumber = incrementIntFile("myfile.bin");
  ValeInt result = 0;
  switch (vtest_IShip_typeTag(vtest_IShip_alias(s))) {
    case vtest_IShip_TAG_Seaship: {
      vtest_Seaship ship = vtest_IShip_asSeaship(vtest_IShip_alias(s));
      result = (vtest_Seaship_leftFuel(vtest_Seaship_alias(ship))
                + vtest_Seaship_rightFuel(vtest_Seaship_alias(ship))) * runNumber;
      vtest_Seaship_dealias(ship);
      break;
    }
    case vtest_IShip_TAG_Spaceship: {
      vtest_Spaceship ship = vtest_IShip_asSpaceship(vtest_IShip_alias(s));
      result = vtest_Spaceship_fuel(vtest_Spaceship_alias(ship)) * runNumber;
      vtest_Spaceship_dealias(ship);
      break;
    }
    default:
      exit(1);
  }
  vtest_IShip_dealias(s);
  return result;
}
