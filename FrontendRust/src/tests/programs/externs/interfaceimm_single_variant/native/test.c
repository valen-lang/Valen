#include <stdint.h>
#include <stdlib.h>
#include <assert.h>

#include "vtest/OnlyOne.h"
#include "vtest/TheOne.h"
#include "vtest/OnlyOne_alias.h"
#include "vtest/OnlyOne_dealias.h"
#include "vtest/OnlyOne_typeTag.h"
#include "vtest/OnlyOne_asTheOne.h"
#include "vtest/TheOne_alias.h"
#include "vtest/TheOne_dealias.h"
#include "vtest/TheOne_val.h"

// Per @FRMACZ: alias each handle at every pass into an accessor, and dealias
// each handle we own once we're done.
ValeInt vtest_handleIt(vtest_OnlyOne o) {
  assert(vtest_OnlyOne_typeTag(vtest_OnlyOne_alias(o)) == vtest_OnlyOne_TAG_TheOne);
  ValeInt result = 0;
  switch (vtest_OnlyOne_typeTag(vtest_OnlyOne_alias(o))) {
    case vtest_OnlyOne_TAG_TheOne: {
      vtest_TheOne t = vtest_OnlyOne_asTheOne(vtest_OnlyOne_alias(o));
      result = vtest_TheOne_val(vtest_TheOne_alias(t));
      vtest_TheOne_dealias(t);
      break;
    }
    default:
      exit(1);
  }
  vtest_OnlyOne_dealias(o);
  return result;
}
