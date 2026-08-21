#include <iostream>
#include "../../region/common/controlblock.h"

#include "../../translatetype.h"

#include "../expression.h"
#include "shared/shared.h"

Ref translateDiscard(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    Discard* discardM) {
  auto sourceExpr = discardM->expr;
  auto sourceResultType = discardM->sourceType;

  auto sourceRef =
      translateExpression(
          globalState, functionState, blockState, builder, sourceExpr);

  if (sourceResultType == globalState->metalCache->voidType) {
    return sourceRef;
  }

  globalState->getRegion(sourceResultType)
      ->checkValidReference(FL(), functionState, builder, false, sourceResultType, sourceRef);
  buildFlare(FL(), globalState, functionState, builder, "discarding!");
  globalState->getRegion(sourceResultType)
      ->dealias(
          AFL(std::string("Discard ") + typeid(*discardM->sourceType).name() + " from " + typeid(*sourceExpr).name()),
          functionState,
          builder,
          sourceResultType,
          sourceRef);
  buildFlare(FL(), globalState, functionState, builder, "discarded!");
  return makeVoidRef(globalState);
}
