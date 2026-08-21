#ifndef BOUNDARY_H_
#define BOUNDARY_H_

#include "../globalstate.h"
#include "boundary.h"

Ref receiveHostObjectIntoVale(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* hostRefMT,
    Kind* valeRefMT,
    LLVMValueRef hostRefLE);

LLVMValueRef sendValeObjectIntoHost(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* valeRefMT,
    Ref valeRef);

#endif
