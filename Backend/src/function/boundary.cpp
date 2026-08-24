#include <vector>

#include "../globalstate.h"
#include "expressions/expressions.h"
#include "boundary.h"
#include "../region/iregion.h"

bool translatesToCVoid(GlobalState* globalState, ValueKind* returnMT) {
  return returnMT == globalState->metalCache->neverType
      || returnMT == globalState->metalCache->voidType;
}

// The single source of truth for what LLVM type a full param/return type crosses
// the C-ABI boundary as (S8). One branch per Kind, no fallthrough: a reference
// wrap crosses as a pointer to its value, void/never as void, and every other
// value kind as its own C-ABI representation (getExternalType). Both the param and
// return paths derive from this.
static LLVMTypeRef hostBoundaryType(GlobalState* globalState, Kind* valeRefMT) {
  auto valeKind = peel_all_references(valeRefMT);
  auto valueLT = globalState->getRegion(valeKind)->getExternalType(valeKind);
  auto voidLT = LLVMVoidTypeInContext(globalState->context);
  if (dynamic_cast<BorrowRef*>(valeRefMT)) {
    return LLVMPointerType(valueLT, 0);
  } else if (dynamic_cast<OwnRef*>(valeRefMT)) {
    return LLVMPointerType(valueLT, 0);
  } else if (dynamic_cast<ShareRef*>(valeRefMT)) {
    return LLVMPointerType(valueLT, 0);
  } else if (dynamic_cast<WeakRef*>(valeRefMT)) {
    return LLVMPointerType(valueLT, 0);
  } else if (dynamic_cast<Void*>(valeRefMT)) {
    return voidLT;
  } else if (dynamic_cast<Never*>(valeRefMT)) {
    return voidLT;
  } else if (dynamic_cast<Bool*>(valeRefMT)) {
    return valueLT;
  } else if (dynamic_cast<Int*>(valeRefMT)) {
    return valueLT;
  } else if (dynamic_cast<Float*>(valeRefMT)) {
    return valueLT;
  } else if (dynamic_cast<USize*>(valeRefMT)) {
    return valueLT;
  } else if (dynamic_cast<Str*>(valeRefMT)) {
    return valueLT;
  } else if (dynamic_cast<StructKind*>(valeRefMT)) {
    return valueLT;
  } else if (dynamic_cast<InterfaceKind*>(valeRefMT)) {
    return valueLT;
  } else if (dynamic_cast<StaticSizedArrayT*>(valeRefMT)) {
    return valueLT;
  } else if (dynamic_cast<RuntimeSizedArrayT*>(valeRefMT)) {
    return valueLT;
  } else {
    { assert(false); throw 1337; }
  }
}

// A by-value aggregate return crosses through a hidden sret out-parameter rather
// than a real return value — true exactly when its boundary type is a by-value
// struct (a pointer or scalar return needs none).
bool returnNeedsOutParam(GlobalState* globalState, Kind* returnRefMT) {
  return LLVMGetTypeKind(hostBoundaryType(globalState, returnRefMT)) == LLVMStructTypeKind;
}

LLVMTypeRef translateExternReturnType(GlobalState* globalState, Kind* returnRefMT) {
  if (returnNeedsOutParam(globalState, returnRefMT)) {
    // Returned through the out-parameter instead.
    return LLVMVoidTypeInContext(globalState->context);
  }
  return hostBoundaryType(globalState, returnRefMT);
}

BoundarySignature buildBoundarySignature(GlobalState* globalState, Prototype* prototypeM) {
  bool usesReturnOutParam = returnNeedsOutParam(globalState, prototypeM->returnType);
  std::vector<LLVMTypeRef> paramTypesL;
  if (usesReturnOutParam) {
    paramTypesL.push_back(
        LLVMPointerType(hostBoundaryType(globalState, prototypeM->returnType), 0));
  }
  for (auto valeParamRefMT : prototypeM->params) {
    paramTypesL.push_back(hostBoundaryType(globalState, valeParamRefMT));
  }
  return BoundarySignature{
      translateExternReturnType(globalState, prototypeM->returnType),
      std::move(paramTypesL),
      usesReturnOutParam};
}

Ref receiveHostObjectIntoVale(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* valeRefMT,
    LLVMValueRef hostRefLE) {
  auto valeRefValueType = peel_all_references(valeRefMT);
  if (dynamic_cast<Void*>(valeRefMT)) {
    return toRef(globalState->getRegion(valeRefValueType), valeRefMT, makeVoid(globalState));
  }
  if (dynamic_cast<Bool*>(valeRefMT)) {
    auto asI1LE =
        LLVMBuildTrunc(builder, hostRefLE, LLVMInt1TypeInContext(globalState->context), "boolAsI1");
    return toRef(globalState->getRegion(valeRefValueType), valeRefMT, asI1LE);
  }
  return toRef(globalState->getRegion(valeRefValueType), valeRefMT, hostRefLE);
}

LLVMValueRef sendValeObjectIntoHost(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* valeRefMT,
    Ref valeRef) {
  auto valeRefValueType = peel_all_references(valeRefMT);
  auto valeArgLE =
      globalState->getRegion(valeRefValueType)
          ->checkValidReference(FL(), functionState, builder, true, valeRefMT, valeRef);
  if (dynamic_cast<Bool*>(valeRefMT)) {
    return LLVMBuildZExt(builder, valeArgLE, LLVMInt8TypeInContext(globalState->context), "boolAsI8");
  }
  return valeArgLE;
}
