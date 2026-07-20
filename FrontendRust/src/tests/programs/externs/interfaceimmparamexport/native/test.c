#include <stdint.h>

#include "vtest/IShip.h"
#include "vtest/valeGetShipFuel.h"

// s is BORROW (extern arg) — do not dealias.
ValeInt vtest_cGetShipFuel(vtest_IShip s) {
  return vtest_valeGetShipFuel(s);
}
