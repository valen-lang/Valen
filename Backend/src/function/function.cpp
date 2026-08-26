#include <iostream>
#include <utils/definefunction.h>
#include "expressions/shared/shared.h"

#include "../translatetype.h"

#include "function.h"
#include "expression.h"
#include "boundary.h"
#include <region/common/migration.h>
#include <utils/counters.h>

ValeFuncPtrLE declareFunction(
    GlobalState* globalState,
    Function* functionM) {

  auto valeParamTypesL = translateTypes(globalState, functionM->prototype->params);
  auto valeReturnTypeL =
      globalState->getRegion(functionM->prototype->returnType)
          ->translateType(functionM->prototype->returnType);

  auto valeFunctionNameL = functionM->prototype->name->name;
  if (valeFunctionNameL == "main") {
    // Otherwise we conflict with the main that we create for entry setup.
    valeFunctionNameL = ":main";
  }
  auto valeFunctionL =
      addValeFunction(globalState, valeFunctionNameL.c_str(), valeReturnTypeL, valeParamTypesL);

  assert(globalState->functions.count(functionM->prototype->name->name) == 0);
  globalState->functions.emplace(functionM->prototype->name->name, valeFunctionL);

  return valeFunctionL;
}

void exportFunction(GlobalState* globalState, Package* package, const std::string& exportName, Prototype* prototypeM) {
  auto sig = buildBoundarySignature(globalState, prototypeM);
  bool usingReturnOutParam = sig.usesReturnOutParam;
  LLVMTypeRef exportReturnLT = sig.returnLT;
  LLVMTypeRef exportFunctionTypeL =
      LLVMFunctionType(sig.returnLT, sig.paramTypesL.data(), sig.paramTypesL.size(), 0);

  auto fullExportName = package->packageCoordinate->projectName + "_" + exportName;
  auto abiExportName = std::string("vale_abi_") + fullExportName;

  // The full name should end in _0, _1, etc. The exported name shouldnt.
  assert(abiExportName != prototypeM->name->name);
  // Per @FRMACZ, this export thunk is a silent shim: it receives the host args
  // without touching their RC and forwards them to the real Vale function, which
  // consumes them as a normal callee. The return is likewise handed to C without
  // any RC adjustment, and C owns it.
  LLVMValueRef exportFunctionL = LLVMAddFunction(globalState->mod, abiExportName.c_str(), exportFunctionTypeL);
  LLVMSetLinkage(exportFunctionL, LLVMExternalLinkage);

  LLVMBasicBlockRef block = LLVMAppendBasicBlockInContext(globalState->context, exportFunctionL, "entry");
  LLVMBuilderRef builder = LLVMCreateBuilderInContext(globalState->context);
  LLVMPositionBuilderAtEnd(builder, block);
  // This is unusual because normally we have a separate localsBuilder which points to a separate
  // block at the beginning. This is a simple function which should require no locals, so this
  // should be fine.
  LLVMBuilderRef localsBuilder = builder;

  FunctionState functionState(abiExportName, exportFunctionL, exportReturnLT, localsBuilder);
  BlockState initialBlockState(globalState->addressNumberer, nullptr, std::nullopt);
  buildFlare(FL(), globalState, &functionState, builder, "Calling export function ", functionState.containingFuncName, " from native");

  std::vector<Ref> argsToActualFunction;

  for (int logicalParamIndex = 0; logicalParamIndex < prototypeM->params.size(); logicalParamIndex++) {
    auto cParamIndex = logicalParamIndex + (usingReturnOutParam ? 1 : 0);

    auto valeParamRefMT = prototypeM->params[logicalParamIndex];

    // TODO: find a way to not rely on LLVMGetParam directly
    auto cArgLE = LLVMGetParam(exportFunctionL, cParamIndex);

    auto valeRef =
        receiveHostObjectIntoVale(
            globalState, &functionState, builder, valeParamRefMT, cArgLE);

    argsToActualFunction.push_back(valeRef);

    // No free here: per @FRMACZ the arg moves into the real Vale function called
    // below, which consumes it like any callee.
  }

  buildFlare(FL(), globalState, &functionState, builder, "Suspending export function ", functionState.containingFuncName);
  buildFlare(FL(), globalState, &functionState, builder, "Calling vale function ", prototypeM->name->name);
  auto valeReturnRefOrVoid =
      buildCallV(globalState, &functionState, builder, prototypeM, argsToActualFunction);
  buildFlare(FL(), globalState, &functionState, builder, "Done calling vale function ", prototypeM->name->name);
  buildFlare(FL(), globalState, &functionState, builder, "Resuming export function ", functionState.containingFuncName);

  if (prototypeM->returnType == globalState->metalCache->voidType) {
    LLVMBuildRetVoid(builder);
  } else {
    auto valeReturnRef = valeReturnRefOrVoid;

    auto valeReturnMT = prototypeM->returnType;

    auto hostReturnRefLE =
        sendValeObjectIntoHost(
            globalState, &functionState, builder, valeReturnMT, valeReturnRef);

    buildFlare(FL(), globalState, &functionState, builder, "Done calling export function ", functionState.containingFuncName, " from native");

    if (usingReturnOutParam) {
      LLVMBuildStore(builder, hostReturnRefLE, LLVMGetParam(exportFunctionL, 0));
      LLVMBuildRetVoid(builder);
    } else {
      LLVMBuildRet(builder, hostReturnRefLE);
    }
  }

  LLVMDisposeBuilder(builder);
}

RawFuncPtrLE declareExternFunction(
    GlobalState* globalState,
    Package* package,
    Prototype* prototypeM) {
  auto sig = buildBoundarySignature(globalState, prototypeM);

  std::string abiFuncNameL;
  if (package->packageCoordinate->projectName == "rust") {
    abiFuncNameL = package->getFunctionExternName(prototypeM);
  } else {
    // This is the name of the Valen-generated shim that calls the actual extern C function.
    // We should remove this one day (see @BDCABIBZ)
    abiFuncNameL = std::string("vale_abi_") + package->packageCoordinate->projectName + "_" + package->getFunctionExternName(prototypeM);
  }

  RawFuncPtrLE functionL =
      addRawFunction(globalState->mod, abiFuncNameL.c_str(), sig.returnLT, sig.paramTypesL);

  assert(globalState->externFunctions.count(prototypeM->name->name) == 0);
  globalState->externFunctions.emplace(prototypeM->name->name, functionL);

  return functionL;
}

void translateFunction(
    GlobalState* globalState,
    Function* functionM) {

  auto functionL = globalState->getFunction(functionM->prototype);
  auto returnTypeL =
      globalState->getRegion(functionM->prototype->returnType)->translateType(functionM->prototype->returnType);

  defineValeFunctionBody(
      globalState->context,
      functionL,
      returnTypeL,
      functionM->prototype->name->name,
      [globalState, functionM](FunctionState* functionState, LLVMBuilderRef builder) {
        BlockState initialBlockState(globalState->addressNumberer, nullptr, std::nullopt);

        // Translate the body of the function. Can ignore the result because it's a
        // Never, because Valestrom guarantees we end function bodies in a ret.
        auto resultLE =
            translateExpression(
                globalState, functionState, &initialBlockState, builder, functionM->block);

        initialBlockState.checkAllIntroducedLocalsWereUnstackified();
      });
}

void declareExtraFunction(
    GlobalState* globalState,
    Prototype* prototype,
    std::string llvmName) {
  auto returnTypeLT = globalState->translateType(prototype->returnType);

  std::vector<LLVMTypeRef> paramsLT;
  for (int i = 0; i < prototype->params.size(); i++) {
    auto paramMT = prototype->params[i];
    paramsLT.push_back(globalState->translateType(paramMT));
  }

  auto functionL = addValeFunction(globalState, llvmName.c_str(), returnTypeLT, paramsLT);
  // Don't define it yet, we're just declaring them right now.
  globalState->extraFunctions.emplace(std::make_pair(prototype, functionL));
}

void defineFunctionBodyV(
    GlobalState* globalState,
    Prototype* prototype,
    std::function<void(FunctionState*, LLVMBuilderRef)> definer) {
  auto functionL = globalState->lookupFunction(prototype);
  auto retTypeLT = globalState->translateType(prototype->returnType);
  defineValeFunctionBody(
      globalState->context,
      functionL,
      retTypeLT,
      prototype->name->name,
      definer);
}

void declareAndDefineExtraFunction(
    GlobalState* globalState,
    Prototype* prototype,
    std::string llvmName,
    std::function<void(FunctionState*, LLVMBuilderRef)> definer) {
  declareExtraFunction(globalState, prototype, llvmName);
  defineFunctionBodyV(globalState, prototype, definer);
}


LLVMValueRef FunctionState::getParam(UserArgIndex userArgIndex) {
  return LLVMGetParam(containingFuncL, userArgIndex.userArgIndex);
}