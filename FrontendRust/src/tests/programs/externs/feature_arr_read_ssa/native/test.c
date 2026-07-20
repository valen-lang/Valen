#include <stdint.h>

#include "vtest/MyIntArray.h"
#include "vtest/MyIntArray_alias.h"
#include "vtest/MyIntArray_at.h"
#include "vtest/MyIntArray_dealias.h"

// arr is OWN (moved in). SSA size known at compile time via _SIZE. Each at()
// consumes an array count, so wrap it in MyIntArray_alias (returns the same
// handle, +1); a trailing dealias discharges the original owned count.
ValeInt vtest_testArrRead(vtest_MyIntArray arr) {
  ValeInt total = 0;
  for (int i = 0; i < vtest_MyIntArray_SIZE; i++) {
    total += vtest_MyIntArray_at(vtest_MyIntArray_alias(arr), i);
  }
  vtest_MyIntArray_dealias(arr);               // discharge the original owned count
  return total;    // 6 + 14 + 22 == 42
}
