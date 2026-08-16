#include <stdint.h>
#include <string.h>
#include <assert.h>

#include "vtest/getAStr.h"
#include "vtest/makeRepeatingHello.h"
#include "vtest/str_alias.h"
#include "vtest/str_len.h"
#include "vtest/str_char_at.h"
#include "vtest/str_dealias.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

// Returns a Vale str. Dynamic content built via Vale-side factory since C→Vale
// str construction from raw bytes isn't in the auto-gen list (see plan
// Known-unresolved regressions).
vtest_str vtest_runExtCommand() {
  int runNumber = incrementIntFile("myfile.bin");

  vtest_str str = vtest_getAStr();   // owned 1

  // Each str accessor consumes a count; wrap every read in str_alias (returns the
  // same handle, +1), then a trailing dealias discharges the original.
  assert(vtest_str_len(vtest_str_alias(str)) == 6);
  // Byte-level equality: copy Vale str chars into a C buffer, strncmp.
  // Preserves OLD `strncmp(str->chars, "hello!", 6)` structure.
  char buf[7];
  for (int i = 0; i < 6; i++) {
    buf[i] = vtest_str_char_at(vtest_str_alias(str), i);
  }
  buf[6] = 0;
  int diff = strncmp(buf, "hello!", 6);
  assert(diff == 0);

  vtest_str_dealias(str);   // discharge the original owned count

  return vtest_makeRepeatingHello(runNumber);
}
