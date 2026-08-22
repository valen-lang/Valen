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
  auto sizeRef = globalState->constI32(ssaDefMT->size);

  auto generatorRef = translateExpression(globalState, functionState, blockState, builder, generatorExpr);
  globalState->getRegion(peel_all_references(generatorType))
      ->checkValidReference(FL(), functionState, builder, true, generatorType, generatorRef);

  // arrayLT is a pointer to our counted struct.
  auto ssaLiveRef =
      globalState->getRegion(peel_all_references(staticArrayFromCallable->result))->constructStaticSizedArray(
          functionState,
          builder,
          staticArrayFromCallable->result,
          staticSizedArrayMT);
  auto ssaRef = toRef(globalState, staticArrayFromCallable->result, ssaLiveRef);

  buildFlare(FL(), globalState, functionState, builder);
  fillStaticSizedArrayFromCallable(
      globalState,
      functionState,
      builder,
      staticArrayFromCallable->result,
      staticSizedArrayMT,
      elementType,
      generatorType,
      staticArrayFromCallable->generatorMethod,
      generatorRef,
      sizeRef,
      ssaLiveRef);
  buildFlare(FL(), globalState, functionState, builder);

  globalState->getRegion(peel_all_references(staticArrayFromCallable->result))
      ->checkValidReference(FL(), functionState, builder, true, staticArrayFromCallable->result, ssaRef);

  globalState->getRegion(peel_all_references(generatorType))->dealias(AFL("ConstructRSA"), functionState, builder, generatorType, generatorRef);

  return ssaRef;
}
