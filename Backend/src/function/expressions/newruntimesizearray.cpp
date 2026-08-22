#include <iostream>
#include "../../region/common/common.h"
#include "../../region/common/controlblock.h"
#include "shared/elements.h"

#include "../../translatetype.h"

#include "shared/members.h"
#include "../expression.h"
#include "shared/shared.h"
#include "../../region/common/heap.h"

Ref translateNewRuntimeSizedArray(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    NewRuntimeSizedArray* constructRuntimeSizedArray) {

  auto sizeExpr = constructRuntimeSizedArray->capacityExpr;

  auto runtimeSizedArrayMT = constructRuntimeSizedArray->arrayType;

  auto capacityRef = translateExpression(globalState, functionState, blockState, builder, sizeExpr);

  // arrayLT is a pointer to our counted struct.
  auto rsaLiveRef =
      globalState->getRegion(peel_all_references(constructRuntimeSizedArray->result))->constructRuntimeSizedArray(
          functionState,
          builder,
          constructRuntimeSizedArray->result,
          runtimeSizedArrayMT,
          capacityRef,
          runtimeSizedArrayMT->name->name);
  auto rsaRef = toRef(globalState, constructRuntimeSizedArray->result, rsaLiveRef);
  buildFlare(FL(), globalState, functionState, builder);
  globalState->getRegion(peel_all_references(constructRuntimeSizedArray->result))
      ->checkValidReference(FL(), functionState, builder, true, constructRuntimeSizedArray->result, rsaRef);

  return rsaRef;
}
