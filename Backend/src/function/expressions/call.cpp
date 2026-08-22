#include <iostream>
#include "shared/shared.h"

#include "../../translatetype.h"

#include "../expression.h"


Ref translateCall(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    Call* call) {
  auto argsLE = std::vector<Ref>{};
  argsLE.reserve(call->args.size());
  for (int i = 0; i < call->args.size(); i++) {
    auto argLE = translateExpression(globalState, functionState, blockState, builder, call->args[i]);
    buildFlare(FL(), globalState, functionState, builder);
    globalState->getRegion(peel_all_references(call->callable->params[i]))
        ->checkValidReference(FL(), functionState, builder, false, call->callable->params[i], argLE);
    argsLE.push_back(argLE);
  }

  return buildCallV(globalState, functionState, builder, call->callable, argsLE);
}
