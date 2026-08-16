#include <stdint.h>

#include "vtest/IShip.h"
#include "vtest/Firefly.h"
#include "vtest/IShip_alias.h"
#include "vtest/IShip_dealias.h"
#include "vtest/IShip_asFirefly.h"
#include "vtest/Firefly_alias.h"
#include "vtest/Firefly_dealias.h"
#include "vtest/Firefly_fuel.h"

// Per @FRMACZ: alias each handle at every pass into an accessor, and dealias
// each handle we own once we're done.
ValeInt vtest_cGetShipFuel(vtest_IShip s) {
  vtest_Firefly f = vtest_IShip_asFirefly(vtest_IShip_alias(s));
  vtest_IShip_dealias(s);
  ValeInt result = vtest_Firefly_fuel(vtest_Firefly_alias(f));
  vtest_Firefly_dealias(f);
  return result;
}
