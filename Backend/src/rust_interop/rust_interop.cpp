// Rust->Vale callback wrappers (the reverse interop direction).
//
// The forward direction (Vale calls Rust) declares each Rust leaf as an extern and marshals the call
// at the call site (buildCallOrSideCall). The reverse direction (Rust calls a Vale trait-impl method)
// is the mirror: a Vale struct implements an imported Rust trait, rustc monomorphizes a generic Rust
// caller over that struct, and its `c.method()` dispatches statically to `<Struct as Trait>::method`.
// Vale supplies that method's body, and we must expose it to Rust under rustc's own mangled symbol.
//
// This file is the one AI-editable corner of the otherwise-core Backend, so the inbound-wrapper
// emission lives here. It reuses the same boundary machinery as `exportFunction`
// (buildBoundarySignature / receiveHostObjectIntoVale / buildCallV / sendValeObjectIntoHost); the
// only difference is the wrapper is named with the rustc-mangled symbol rather than a `vale_abi_` one.

#include <iostream>

#include "../function/expressions/shared/shared.h"
#include "../translatetype.h"
#include "../function/function.h"
#include "../function/boundary.h"
#include <region/common/migration.h>

#include "rust_interop.h"

void emitInboundCallbackWrapper(
    GlobalState* globalState,
    Program* program,
    const std::string& symbol,
    const std::string& valeName) {
  // The internal Vale body to forward to was instantiated into the program under its humanized name
  // (valeName). Locate its metal prototype; the driver only hands us callbacks whose body it emitted.
  Prototype* prototypeM = nullptr;
  for (auto& [coord, package] : program->packages) {
    (void)coord;
    auto iter = package->functions.find(valeName);
    if (iter != package->functions.end()) {
      prototypeM = iter->second->prototype;
      break;
    }
  }
  assert(prototypeM != nullptr && "inbound callback body not found in program");

  // The host-facing signature comes from the inbound ABI recorded on the body's package (keyed by
  // valeName) — the same direction-agnostic buildBoundarySignature the extern declarations use.
  auto sig = buildBoundarySignature(globalState, prototypeM);
  bool usingReturnOutParam = sig.usesReturnOutParam;
  LLVMTypeRef wrapperReturnLT = sig.returnLT;
  LLVMTypeRef wrapperFunctionTypeL =
      LLVMFunctionType(sig.returnLT, sig.paramTypesL.data(), sig.paramTypesL.size(), 0);

  // Single-symbol: define the wrapper under rustc's own mangled name, the sole definition at link.
  LLVMValueRef wrapperL = LLVMAddFunction(globalState->mod, symbol.c_str(), wrapperFunctionTypeL);
  LLVMSetLinkage(wrapperL, LLVMExternalLinkage);

  LLVMBasicBlockRef block = LLVMAppendBasicBlockInContext(globalState->context, wrapperL, "entry");
  LLVMBuilderRef builder = LLVMCreateBuilderInContext(globalState->context);
  LLVMPositionBuilderAtEnd(builder, block);
  // Simple wrapper, no separate locals block needed (mirrors exportFunction).
  LLVMBuilderRef localsBuilder = builder;

  FunctionState functionState(symbol, wrapperL, wrapperReturnLT, localsBuilder);
  BlockState initialBlockState(globalState->addressNumberer, nullptr, std::nullopt);

  // Receive each Rust-ABI argument and adapt it to the Vale ref the body expects (the inverse of
  // buildCallOrSideCall's send side). Coercion-driven: most args are one C param each (a pointer for a
  // borrow, an integer for a scalar), but a Pair struct arrives as TWO C params and is reassembled
  // here. An sret return shifts the first C parameter index by one.
  const ExternAbi* abi = lookupExternAbi(globalState, prototypeM);
  assert(abi != nullptr && "inbound callback wrapper needs the callback's extern ABI");
  assert(abi->args.size() == prototypeM->params.size());
  std::vector<Ref> argsToBody;
  unsigned cParamIndex = usingReturnOutParam ? 1u : 0u;
  for (int logicalParamIndex = 0; logicalParamIndex < (int)prototypeM->params.size(); logicalParamIndex++) {
    auto valeParamRefMT = prototypeM->params[logicalParamIndex];
    const Coercion& c = abi->args[logicalParamIndex];
    if (c.kind == CoercionKind::Pair) {
      // Two integer register params -> reassemble the Vale struct. Store the two scalars into an
      // {iN,iM} slot at their offsets, then reinterpret the slot as the Vale struct and load it by
      // value (the inverse of buildCallOrSideCall's Cast+StructKind spill).
      auto structKind = peel_all_references(valeParamRefMT);
      auto valeStructLT = globalState->getRegion(structKind)->translateType(structKind);
      auto lo = LLVMGetParam(wrapperL, cParamIndex);
      auto hi = LLVMGetParam(wrapperL, cParamIndex + 1);
      LLVMTypeRef pairElems[2] = {LLVMTypeOf(lo), LLVMTypeOf(hi)};
      auto pairLT = LLVMStructTypeInContext(globalState->context, pairElems, 2, /*packed=*/0);
      auto slot = makeBackendLocal(&functionState, builder, pairLT, "pairInSlot", LLVMGetUndef(pairLT));
      LLVMBuildStore(builder, lo, LLVMBuildStructGEP2(builder, pairLT, slot, 0, "pf0"));
      LLVMBuildStore(builder, hi, LLVMBuildStructGEP2(builder, pairLT, slot, 1, "pf1"));
      auto valeStructLE = LLVMBuildLoad2(
          builder, valeStructLT,
          LLVMBuildBitCast(builder, slot, LLVMPointerType(valeStructLT, 0), "pairAsStruct"),
          "pairStruct");
      argsToBody.push_back(toRef(globalState->getRegion(structKind), valeParamRefMT, valeStructLE));
      cParamIndex += 2;
    } else {
      auto cArgLE = LLVMGetParam(wrapperL, cParamIndex);
      argsToBody.push_back(
          receiveHostObjectIntoVale(globalState, &functionState, builder, valeParamRefMT, cArgLE));
      cParamIndex += 1;
    }
  }

  auto valeReturnRefOrVoid =
      buildCallV(globalState, &functionState, builder, prototypeM, argsToBody);

  if (prototypeM->returnType == globalState->metalCache->voidType) {
    LLVMBuildRetVoid(builder);
  } else {
    auto hostReturnRefLE =
        sendValeObjectIntoHost(
            globalState, &functionState, builder, prototypeM->returnType, valeReturnRefOrVoid);
    if (usingReturnOutParam) {
      LLVMBuildStore(builder, hostReturnRefLE, LLVMGetParam(wrapperL, 0));
      LLVMBuildRetVoid(builder);
    } else {
      LLVMBuildRet(builder, hostReturnRefLE);
    }
  }

  LLVMDisposeBuilder(builder);
}
