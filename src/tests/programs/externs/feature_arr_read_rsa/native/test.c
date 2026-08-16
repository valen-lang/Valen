#include <stdint.h>

#include "vtest/MyIntArray.h"
#include "vtest/MyIntArray_alias.h"
#include "vtest/MyIntArray_len.h"
#include "vtest/MyIntArray_at.h"
#include "vtest/MyIntArray_dealias.h"

// arr is OWN (moved in). len and at each consume an array count, so wrap each
// read in MyIntArray_alias (returns the same handle, +1); a trailing dealias
// discharges the original owned count.
ValeInt vtest_testArrRead(vtest_MyIntArray arr) {
  ValeInt total = 0;
  int len = vtest_MyIntArray_len(vtest_MyIntArray_alias(arr));
  for (int i = 0; i < len; i++) {
    total += vtest_MyIntArray_at(vtest_MyIntArray_alias(arr), i);
  }
  vtest_MyIntArray_dealias(arr);               // discharge the original owned count
  return total;    // 0+2+4+6+8+10 == 30
}
