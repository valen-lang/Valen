#include <stdint.h>

#include "vtest/Thing.h"
#include "vtest/makeHelloThing.h"
#include "vtest/runExtCommand.h"

// C used to build the Vale string via ValeStrFrom, but that exposed the
// linear-region layout to C. Under the opaque-handle FFI the string comes
// from a Vale-side factory instead — "C creates a Vale str from raw bytes"
// is a documented capability gap on this arc.
vtest_ThingRef vtest_runExtCommand() {
  return vtest_makeHelloThing(37);
}
