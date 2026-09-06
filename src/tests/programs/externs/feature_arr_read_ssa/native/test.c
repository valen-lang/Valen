#include <stdint.h>

#include "vtest/IntTriple.h"
#include "vtest/testArrRead.h"

// The SSA crosses inside the exported struct's layout; read its elements directly.
ValeInt vtest_testArrRead(vtest_IntTriple* triple) {
  ValeInt total = 0;
  for (int i = 0; i < 3; i++) {
    total += triple->values[i];
  }
  return total;   // 6 + 14 + 22 == 42
}
