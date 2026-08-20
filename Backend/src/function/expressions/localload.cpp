#include <iostream>
#include "../../region/common/controlblock.h"

#include "../../translatetype.h"

#include "shared/members.h"
#include "../expression.h"
#include "shared/shared.h"
#include "../../region/common/heap.h"
#include "../../metal/onion.h"

// Reads a local through `Deref(LocalLookup)`: load the local, then upgrade it to the deref's target
// ownership. targetKind is the Deref node's result kind; the local's own type comes from the Local.
Ref translateDerefLocalLookup(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    LocalLookup* lookup,
    Kind* targetKind) {
  auto cache = globalState->metalCache;
  auto local = lookup->localVariable;
  auto localType = refFromKind(cache, local->type);
  auto resultType = refFromKind(cache, targetKind);

  auto regionInstanceRef =
      // At some point, look up the actual region instance, perhaps from the FunctionState?
      globalState->getRegion(localType)->createRegionInstanceLocal(functionState, builder);

  buildFlare(FL(), globalState, functionState, builder);

  auto localAddr = blockState->getLocalAddr(local, true);

  auto sourceRef = globalState->getRegion(localType)->loadLocal(functionState, builder, local, localAddr);

  auto resultRef =
      globalState->getRegion(localType)->upgradeLoadResultToRefWithTargetOwnership(
          functionState, builder, regionInstanceRef, localType, resultType, LoadResult{sourceRef}, false);
  globalState->getRegion(resultType)->alias(FL(), functionState, builder, resultType, resultRef);

  return resultRef;
}
