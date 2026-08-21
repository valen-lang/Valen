#include "../globalstate.h"
#include "expressions/expressions.h"
#include "boundary.h"
#include "../region/iregion.h"

Ref receiveHostObjectIntoVale(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* hostRefMT,
    Kind* valeRefMT,
    LLVMValueRef hostRefLE) {
  // Per @FRMACZ, this conversion does no reference counting — share refs arrive
  // as right-sized handle structs (8B concrete / 16B interface) which we decrypt
  // back into Vale refs, and the ref simply moves in. Primitives pass through
  // unwrapped. (Under the opaque-handle FFI, hostRefMT === valeRefMT.)
  auto kind = hostRefMT->kind;
  bool isPrimitive =
      dynamic_cast<Int*>(kind) || dynamic_cast<Bool*>(kind) ||
      dynamic_cast<Float*>(kind) || dynamic_cast<Void*>(kind);
  if (isPrimitive) {
    if (dynamic_cast<Void*>(kind)) {
      return toRef(globalState->getRegion(valeRefMT), valeRefMT, makeVoid(globalState));
    }
    if (dynamic_cast<Bool*>(kind)) {
      auto asI1LE =
          LLVMBuildTrunc(builder, hostRefLE, LLVMInt1TypeInContext(globalState->context), "boolAsI1");
      return toRef(globalState->getRegion(valeRefMT), valeRefMT, asI1LE);
    }
    return toRef(globalState->getRegion(valeRefMT), valeRefMT, hostRefLE);
  } else {
    // The incoming handle must be exactly the region's external type for this
    // kind (concrete 8B or interface 16B).
    assert(LLVMTypeOf(hostRefLE) == globalState->getRegion(valeRefMT)->getExternalType(hostRefMT));
    return globalState->getRegion(valeRefMT)
        ->receiveAndDecryptFamiliarReference(functionState, builder, hostRefMT, hostRefLE);
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
  // encrypt/send. Primitives are passed through unwrapped — their LE value IS
  // the C-ABI value.
  auto kind = valeRefMT->kind;
  bool isPrimitive =
      dynamic_cast<Int*>(kind) || dynamic_cast<Bool*>(kind) ||
      dynamic_cast<Float*>(kind) || dynamic_cast<Void*>(kind);
  if (isPrimitive) {
    auto valeArgLE =
        globalState->getRegion(valeRefMT)
            ->checkValidReference(FL(), functionState, builder, true, valeRefMT, valeRef);
    if (dynamic_cast<Bool*>(kind)) {
      return LLVMBuildZExt(builder, valeArgLE, LLVMInt8TypeInContext(globalState->context), "boolAsI8");
    }
    return valeArgLE;
  }
  auto encryptedValeRefLE =
      globalState->getRegion(valeRefMT)
          ->encryptAndSendFamiliarReference(functionState, builder, valeRefMT, valeRef);
  assert(LLVMTypeOf(encryptedValeRefLE) == globalState->getRegion(valeRefMT)->getExternalType(valeRefMT));
  return encryptedValeRefLE;
}
