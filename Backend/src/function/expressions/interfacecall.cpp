#include <iostream>
#include "shared/shared.h"
#include "../../region/common/controlblock.h"

#include "../../translatetype.h"

#include "../expression.h"

Ref translateInterfaceCall(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    InterfaceCall* call) {

  auto argExprs = call->args;
  auto virtualParamIndex = call->virtualParamIndex;
  auto indexInEdge = call->indexInEdge;
  auto functionType = call->superFunctionPrototype;
  auto returnValueType = peel_all_references(call->superFunctionPrototype->returnType);

  auto argExprsLE =
      translateExpressions(globalState, functionState, blockState, builder, call->args);

  buildFlare(FL(), globalState, functionState, builder);

  auto argsLE = std::vector<Ref>{};
  argsLE.reserve(call->args.size());
  for (int i = 0; i < call->args.size(); i++) {
    buildFlare(FL(), globalState, functionState, builder);

    auto argLE = translateExpression(globalState, functionState, blockState, builder, call->args[i]);
    globalState->getRegion(call->superFunctionPrototype->params[i])
        ->checkValidReference(FL(), functionState, builder, false, call->superFunctionPrototype->params[i], argLE);
    argsLE.push_back(argLE);

    buildFlare(FL(), globalState, functionState, builder);
  }


  buildFlare(FL(), globalState, functionState, builder);

  auto virtualArgRefMT = functionType->params[virtualParamIndex];
  auto virtualArgRef = argsLE[virtualParamIndex];
  auto methodFunctionPtrLE =
      globalState->getRegion(virtualArgRefMT)
          ->getInterfaceMethodFunctionPtr(functionState, builder, virtualArgRefMT, virtualArgRef, indexInEdge);

  buildFlare(FL(), globalState, functionState, builder);

  auto resultLE =
      buildInterfaceCall(
          globalState,
          functionState,
          builder,
          call->superFunctionPrototype,
          methodFunctionPtrLE,
          argExprsLE,
          call->virtualParamIndex);

  buildFlare(FL(), globalState, functionState, builder);

  globalState->getRegion(returnValueType)
      ->checkValidReference(FL(), functionState, builder, false, call->superFunctionPrototype->returnType, resultLE);

  if (returnValueType == globalState->metalCache->neverType) {
    return toRef(
        globalState->getRegion(globalState->metalCache->neverType),
        globalState->metalCache->neverType,
        LLVMBuildRet(builder, LLVMGetUndef(functionState->returnTypeL)));
  } else {
    return resultLE;
  }
}
