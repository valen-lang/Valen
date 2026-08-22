#include <iostream>
#include "../../region/common/controlblock.h"

#include "../../translatetype.h"

#include "shared/members.h"
#include "../expression.h"
#include "shared/shared.h"
#include "../../region/common/heap.h"

Ref translateLocalLoad(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    LocalLoad* localLoad) {
  auto local = localLoad->local;
  auto localId = local->id;
  auto localName = localLoad->localName;
  auto localType = local->type;
  auto localValueType = peel_all_references(localType);
  auto resultType = globalState->metalCache->getBorrowRef(local->type);

  buildFlare(FL(), globalState, functionState, builder);

  auto localAddr = blockState->getLocalAddr(localId, true);

  auto sourceRef = globalState->getRegion(localValueType)->loadLocal(functionState, builder, local, localAddr);

  auto resultRef =
      globalState->getRegion(localValueType)->upgradeLoadResultToRefWithTargetOwnership(
          functionState, builder, localType, resultType, LoadResult{sourceRef});
  globalState->getRegion(resultType)->alias(FL(), functionState, builder, resultType, resultRef);

  return resultRef;
}
