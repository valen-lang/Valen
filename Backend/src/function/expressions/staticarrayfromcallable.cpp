#include <iostream>
#include "../../region/common/common.h"
#include "../../region/common/controlblock.h"
#include "shared/elements.h"

#include "../../translatetype.h"

#include "shared/members.h"
#include "../expression.h"
#include "shared/shared.h"
#include "../../region/common/heap.h"

Ref translateStaticArrayFromCallable(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    StaticArrayFromCallable* staticArrayFromCallable) {
  // Not yet migrated to the inline (values-based) constructStaticSizedArray. A static array built
  // from a callable runs the generator per element; constructStaticSizedArray now takes the element
  // values up front, so this path must unroll the generator over the (statically-known) size into an
  // element vector. It's closure-blocked and its tests are deferred, so it's left unimplemented.
  { assert(false); throw 1337; }
}
