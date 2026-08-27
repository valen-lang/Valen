#ifndef BOUNDARY_H_
#define BOUNDARY_H_

#include <vector>

#include "../globalstate.h"
#include "boundary.h"

// The C-ABI boundary signature for a prototype (see boundary.cpp).
struct BoundarySignature {
  LLVMTypeRef returnLT;
  std::vector<LLVMTypeRef> paramTypesL;
  bool usesReturnOutParam;
};

bool translatesToCVoid(GlobalState* globalState, ValueKind* returnMT);
bool returnNeedsOutParam(GlobalState* globalState, Kind* returnRefMT);
LLVMTypeRef translateExternReturnType(GlobalState* globalState, Kind* returnRefMT);
BoundarySignature buildBoundarySignature(GlobalState* globalState, Prototype* prototypeM);

// The ABI descriptor a producer attached to this extern (on its metal Package), or nullptr for a
// descriptor-less C extern. Keyed by the prototype name, like GlobalState.externFunctions.
const ExternAbi* lookupExternAbi(GlobalState* globalState, Prototype* prototypeM);

Ref receiveHostObjectIntoVale(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* valeRefMT,
    LLVMValueRef hostRefLE);

LLVMValueRef sendValeObjectIntoHost(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* valeRefMT,
    Ref valeRef);

#endif
