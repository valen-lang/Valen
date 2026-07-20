#include <stdint.h>
#include <assert.h>

#include "vtest/makeEmpty.h"
#include "vtest/str_alias.h"
#include "vtest/str_dealias.h"
#include "vtest/str_len.h"
#include "vtest/str_char_at.h"

// Per @FRMACZ: alias at every pass, dealias the handle we own once we're done.
ValeInt vtest_getLen() {
  vtest_str s = vtest_makeEmpty();       // NEW empty str, owned 1
  assert(vtest_str_len(vtest_str_alias(s)) == 0);
  assert(vtest_str_char_at(vtest_str_alias(s), 0) == 0);  // NUL terminator immediately
  vtest_str_dealias(s);
  return 0;
}
