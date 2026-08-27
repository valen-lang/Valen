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

  auto generatorExpr = staticArrayFromCallable->generator;
  auto staticSizedArrayMT = staticArrayFromCallable->arrayType;

  auto ssaDefMT = globalState->program->getStaticSizedArray(staticSizedArrayMT);
  auto elementType = ssaDefMT->elementType;
  // The generator is the receiver (param 0) of the per-element generator method.
  auto generatorType = staticArrayFromCallable->generatorMethod->params[0];
  auto generatorValueType = peel_all_references(generatorType);
  auto resultValueType = peel_all_references(staticArrayFromCallable->result);

  auto generatorRef = translateExpression(globalState, functionState, blockState, builder, generatorExpr);
  globalState->getRegion(generatorValueType)
      ->checkValidReference(FL(), functionState, builder, true, generatorType, generatorRef);

  std::vector<Ref> elementRefs;
  for (int i = 0; i < ssaDefMT->size; i++) {
    auto indexRef = globalState->constI32(i);
    auto elementRef =
        buildCallV(
            globalState, functionState, builder, staticArrayFromCallable->generatorMethod,
            {generatorRef, indexRef});
    globalState->getRegion(elementType)
        ->checkValidReference(FL(), functionState, builder, false, elementType, elementRef);
    elementRefs.push_back(elementRef);
  }

  auto ssaRef =
      globalState->getRegion(staticArrayFromCallable->result)->constructStaticSizedArray(
          functionState, builder, staticArrayFromCallable->result, staticSizedArrayMT, elementRefs);
  globalState->getRegion(resultValueType)
      ->checkValidReference(FL(), functionState, builder, true, staticArrayFromCallable->result, ssaRef);

  return ssaRef;
}
