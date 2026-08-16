#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "vtest/str_alias.h"
#include "vtest/str_dealias.h"
#include "vtest/str_len.h"
#include "vtest/str_char_at.h"

// We use incrementIntFile to get some side effects to test replayability, see AASETR.
int64_t incrementIntFile(const char* filename);

// Per @FRMACZ: alias at every pass, dealias the handle we own once we're done.
// OLD C did `strlen(str->chars)` which implicitly tested NUL termination of
// Vale strings. Under opaque handles we probe NUL termination explicitly.
ValeInt vtest_extStrLen(vtest_str haystackContainerStr) {
  int runNumber = incrementIntFile("myfile.bin");

  printf("extstrlen run number %d\n", runNumber);

  ValeInt result = vtest_str_len(vtest_str_alias(haystackContainerStr));

  // Preserve OLD strlen implicit NUL-termination check: byte at index len
  // must be 0.
  assert(vtest_str_char_at(vtest_str_alias(haystackContainerStr), result) == 0);

  vtest_str_dealias(haystackContainerStr);
  return result * runNumber;
}
