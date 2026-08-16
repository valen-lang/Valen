#include <stdint.h>

#include "vtest/IShip.h"
#include "vtest/Firefly.h"
#include "vtest/Firefly_new.h"
#include "vtest/Firefly_alias.h"
#include "vtest/Firefly_dealias.h"
#include "vtest/Firefly_asIShip.h"

// Per @FRMACZ: alias firefly for the asIShip pass, dealias the handle we own,
// and return shipRef (which moves out).
vtest_IShip vtest_cMakeShip() {
  vtest_Firefly firefly = vtest_Firefly_new(42);                     // owned 1
  vtest_IShip shipRef = vtest_Firefly_asIShip(vtest_Firefly_alias(firefly));
  vtest_Firefly_dealias(firefly);
  return shipRef;                                                    // moves out
}
