#include <stdint.h>
#include <assert.h>

#include "vtest/getVal.h"
#include "vtest/str_alias.h"
#include "vtest/str_dealias.h"
#include "vtest/str_len.h"
#include "vtest/str_char_at.h"

// C reads a Vale-produced str char-by-char. Per @FRMACZ: alias at every pass,
// dealias the handle we own once we're done.
ValeInt vtest_testStrRead() {
  vtest_str s = vtest_getVal();       // own 1

  assert(vtest_str_len(vtest_str_alias(s)) == 5);
  assert(vtest_str_char_at(vtest_str_alias(s), 0) == 'h');
  assert(vtest_str_char_at(vtest_str_alias(s), 1) == 'e');
  assert(vtest_str_char_at(vtest_str_alias(s), 2) == 'l');
  assert(vtest_str_char_at(vtest_str_alias(s), 3) == 'l');
  assert(vtest_str_char_at(vtest_str_alias(s), 4) == 'o');
  vtest_str_dealias(s);

  return 42;
}
