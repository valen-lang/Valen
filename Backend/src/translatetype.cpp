#include <iostream>
#include "region/common/heap.h"

#include "translatetype.h"

std::vector<LLVMTypeRef> translateTypes(
    GlobalState* globalState,
    std::vector<Kind*> referencesM) {
  std::vector<LLVMTypeRef> result;
  for (auto referenceM : referencesM) {
    result.push_back(
        globalState->getRegion(referenceM)->translateType(referenceM));
  }
  return result;
}

LLVMTypeRef translatePrototypeToFunctionType(
    GlobalState* globalState,
    Prototype* prototype) {
  auto paramsLT = translateTypes(globalState, prototype->params);
  auto returnLT = globalState->getRegion(prototype->returnType)->translateType(prototype->returnType);
  return LLVMFunctionType(returnLT, paramsLT.data(), paramsLT.size(), false);
}

LLVMTypeRef translateInterfaceMethodToFunctionType(
    GlobalState* globalState,
    InterfaceMethod* method) {
  auto returnMT = method->prototype->returnType;
  auto paramsMT = method->prototype->params;
  auto returnLT = globalState->getRegion(returnMT)->translateType(returnMT);
  auto paramsLT = translateTypes(globalState, paramsMT);
  auto virtualParamMT = paramsMT[method->virtualParamIndex];
  paramsLT[method->virtualParamIndex] =
      globalState->getRegion(virtualParamMT)
          ->getInterfaceMethodVirtualParamAnyType();
  return LLVMFunctionType(returnLT, paramsLT.data(), paramsLT.size(), false);
}
