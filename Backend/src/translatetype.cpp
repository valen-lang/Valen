#include <iostream>
#include "region/common/heap.h"

#include "translatetype.h"

std::vector<LLVMTypeRef> translateTypes(
    GlobalState* globalState,
    std::vector<Reference*> referencesM) {
  std::vector<LLVMTypeRef> result;
  for (auto referenceM : referencesM) {
    result.push_back(globalState->getRegion(referenceM)->translateType(referenceM));
  }
  return result;
}

// VCOORD: do we still need this?
Sharedness ownershipToSharedness(Ownership ownership) {
  switch (ownership) {
    case Ownership::MUTABLE_SHARE:
    case Ownership::IMMUTABLE_SHARE:
      return Sharedness::SHARED;
    case Ownership::MUTABLE_BORROW:
    case Ownership::IMMUTABLE_BORROW:
    case Ownership::OWN:
    case Ownership::WEAK:
      return Sharedness::SINGLE;
    default:
      { assert(false); throw 1337; }
  }
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
  paramsLT[method->virtualParamIndex] =
      globalState->getRegion(paramsMT[method->virtualParamIndex])
          ->getInterfaceMethodVirtualParamAnyType(paramsMT[method->virtualParamIndex]);
  return LLVMFunctionType(returnLT, paramsLT.data(), paramsLT.size(), false);
}
