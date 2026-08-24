#include "../globalstate.h"
#include "expressions/expressions.h"
#include "boundary.h"
#include "../region/iregion.h"

Ref receiveHostObjectIntoVale(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* valeRefMT,
    LLVMValueRef hostRefLE) {
  // Per @FRMACZ, this conversion does no reference counting: share refs arrive
  // as right-sized handle structs (8B concrete / 16B interface) which we convert
  // back into Vale refs, and the ref simply moves in. Primitives pass through
  // unwrapped.
  bool isPrimitive =
      dynamic_cast<Int*>(valeRefMT) || dynamic_cast<Bool*>(valeRefMT) ||
      dynamic_cast<Float*>(valeRefMT) || dynamic_cast<Void*>(valeRefMT);
  auto valeRefValueType = peel_all_references(valeRefMT);
  if (isPrimitive) {
    if (dynamic_cast<Void*>(valeRefMT)) {
      return toRef(globalState->getRegion(valeRefValueType), valeRefMT, makeVoid(globalState));
    }
    if (dynamic_cast<Bool*>(valeRefMT)) {
      auto asI1LE =
          LLVMBuildTrunc(builder, hostRefLE, LLVMInt1TypeInContext(globalState->context), "boolAsI1");
      return toRef(globalState->getRegion(valeRefValueType), valeRefMT, asI1LE);
    }
    return toRef(globalState->getRegion(valeRefValueType), valeRefMT, hostRefLE);
  } else {
    // The incoming handle must be exactly the region's external type for this
    // kind (concrete 8B or interface 16B).
    assert(LLVMTypeOf(hostRefLE) == globalState->getRegion(valeRefValueType)->getExternalType(valeRefValueType));
    return globalState->getRegion(valeRefValueType)
        ->refFromHostHandle(functionState, builder, valeRefMT, hostRefLE);
  }
}

LLVMValueRef sendValeObjectIntoHost(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* valeRefMT,
    Ref valeRef) {
  // Under the opaque-handle FFI, share refs (struct/interface/RSA/SSA/Str)
  // cross as right-sized handle structs (8B concrete / 16B interface) via
  // refToHostHandle. Primitives are passed through unwrapped: their LE value is
  // the C-ABI value.
  bool isPrimitive =
      dynamic_cast<Int*>(valeRefMT) || dynamic_cast<Bool*>(valeRefMT) ||
      dynamic_cast<Float*>(valeRefMT) || dynamic_cast<Void*>(valeRefMT);
  auto valeRefValueType = peel_all_references(valeRefMT);
  if (isPrimitive) {
    auto valeArgLE =
        globalState->getRegion(valeRefValueType)
            ->checkValidReference(FL(), functionState, builder, true, valeRefMT, valeRef);
    if (dynamic_cast<Bool*>(valeRefMT)) {
      return LLVMBuildZExt(builder, valeArgLE, LLVMInt8TypeInContext(globalState->context), "boolAsI8");
    }
    return valeArgLE;
  }
  auto hostHandleLE =
      globalState->getRegion(valeRefValueType)
          ->refToHostHandle(functionState, builder, valeRefMT, valeRef);
  assert(LLVMTypeOf(hostHandleLE) == globalState->getRegion(valeRefValueType)->getExternalType(valeRefValueType));
  return hostHandleLE;
}
