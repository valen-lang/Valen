#include <iostream>

#include "../../utils/branch.h"

#include "../../translatetype.h"

#include "../expression.h"
#include "expressions.h"

Ref translateBlock(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* parentBlockState,
    LLVMBuilderRef builder,
    Block* block) {

  BlockState childBlockState(globalState->addressNumberer, parentBlockState, std::nullopt);

  auto resultLE =
      translateExpression(globalState, functionState, &childBlockState, builder, block->inner);

  if (block->innerType->kind != globalState->metalCache->neverType) {
    childBlockState.checkAllIntroducedLocalsWereUnstackified();

    auto childUnstackifiedParentLocals =
        childBlockState.getParentLocalsThatSelfUnstackified();
    for (auto childUnstackifiedParentLocal : childUnstackifiedParentLocals) {
      parentBlockState->markLocalUnstackified(childUnstackifiedParentLocal);
    }
  }

  return resultLE;
}
