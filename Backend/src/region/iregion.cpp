#include "iregion.h"

LLVMValueRef checkValidReference(
    AreaAndFileAndLine checkerAFL,
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool expectLive,
    Kind* refM,
    Ref ref) {
  return globalState->getRegion(peel_all_references(refM))->checkValidReference(
      checkerAFL, functionState, builder, expectLive, refM, ref);
}

LLVMValueRef checkValidReference(
    AreaAndFileAndLine checkerAFL,
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool expectLive,
    Kind* refM,
    LiveRef liveRef) {
  auto ref = toRef(globalState, refM, liveRef);
  return checkValidReference(
      checkerAFL, globalState, functionState, builder, expectLive, refM, ref);
}
