#include <iostream>
#include "../../region/common/common.h"
#include "../../region/common/controlblock.h"
#include "shared/elements.h"

#include "../../translatetype.h"

#include "shared/members.h"
#include "../expression.h"
#include "shared/shared.h"
#include "../../region/common/heap.h"

Ref translateNewArrayFromValues(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    NewArrayFromValues* newArrayFromValues) {
  auto elementsLE =
      translateExpressions(
          globalState, functionState, blockState, builder, newArrayFromValues->elements);
  auto ssaDefM = globalState->program->getStaticSizedArray(newArrayFromValues->arrayType);
  for (auto elementLE : elementsLE) {
    globalState->getRegion(ssaDefM->elementType)
        ->checkValidReference(
            FL(), functionState, builder, false, ssaDefM->elementType, elementLE);
  }

  return globalState->getRegion(newArrayFromValues->result)->constructStaticSizedArray(
      functionState,
      builder,
      newArrayFromValues->result,
      newArrayFromValues->arrayType,
      elementsLE);
}
