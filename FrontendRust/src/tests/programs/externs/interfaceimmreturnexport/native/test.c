#include <stdint.h>

#include "vtest/IShip.h"
#include "vtest/valeMakeShip.h"

vtest_IShip vtest_cMakeShip() {
  // valeMakeShip returns NEW; transfer directly to Vale as the extern return.
  return vtest_valeMakeShip();
}
