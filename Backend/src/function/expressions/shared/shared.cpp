#include "../../../region/common/common.h"
#include "../../../region/common/fatweaks/fatweaks.h"
#include "../../../utils/counters.h"
#include "shared.h"

#include "../../../translatetype.h"
#include "../../../region/common/controlblock.h"
#include "../../../region/linear/linear.h"
#include "../../../region/rcimm/rcimm.h"
#include "../../../utils/branch.h"
#include <region/common/migration.h>

// A "Never" is something that should never be read.
// This is useful in a lot of situations, for example:
// - The return type of Panic()
// - The result of the Discard node
LLVMTypeRef makeNeverType(GlobalState* globalState) {
  // We arbitrarily use a zero-len array of i57 here because it's zero sized and
  // very unlikely to be used anywhere else.
  // We could use an empty struct instead, but this'll do.
  return LLVMArrayType(LLVMIntTypeInContext(globalState->context, NEVER_INT_BITS), 0);
}

LLVMValueRef makeVoid(GlobalState* globalState) {
  return LLVMGetUndef(globalState->rcImm->translateType(globalState->metalCache->voidRef));
}

LLVMTypeRef makeEmptyStructType(GlobalState* globalState) {
  return LLVMStructTypeInContext(globalState->context, nullptr, 0, false);
}
LLVMValueRef makeEmptyStruct(GlobalState* globalState) {
  return LLVMConstNamedStruct(makeEmptyStructType(globalState), nullptr, 0);
}

Ref makeVoidRef(GlobalState* globalState) {
  auto voidLE = makeVoid(globalState);
  auto refMT = globalState->metalCache->voidRef;
  return toRef(globalState->rcImm, refMT, voidLE);
}

LLVMValueRef makeBackendLocal(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMTypeRef typeL,
    const std::string& name,
    LLVMValueRef valueToStore) {
  auto localAddr = LLVMBuildAlloca(functionState->localsBuilder, typeL, name.c_str());
  LLVMBuildStore(builder, valueToStore, localAddr);
  return localAddr;
}

void makeHammerLocal(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    Local* local,
    Ref refToStore,
    bool knownLive) {
  auto localAddr = globalState->getRegion(local->type)->stackify(functionState, builder, local, refToStore, knownLive);
  blockState->addLocal(local->id, localAddr);
}

// Returns the new RC
LLVMValueRef adjustStrongRc(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* kindStructsSource,
    LLVMBuilderRef builder,
    Ref exprRef,
    Reference* refM,
    int amount) {
  assert(refM->ownership == Ownership::MUTABLE_SHARE);
  // Shouldnt increment IMMUTABLE_SHARE's RC

  auto controlBlockPtrLE =
      kindStructsSource->getControlBlockPtr(from, functionState, builder, exprRef, refM);
  auto rcPtrLE = kindStructsSource->getStrongRcPtrFromControlBlockPtr(builder, refM, controlBlockPtrLE);
//  auto oldRc = unmigratedLLVMBuildLoad(builder, rcPtrLE, "oldRc");
  auto newRc =
      adjustCounterV(
          globalState, builder, globalState->metalCache->i32, rcPtrLE, amount, globalState->opt->useAtomicRc);

  if (globalState->opt->printMemOverhead) {
    adjustCounterV(
        globalState, builder, globalState->metalCache->i64, globalState->mutRcAdjustCounterLE, 1, false);
  }

//  flareAdjustStrongRc(from, globalState, functionState, builder, refM, controlBlockPtrLE, oldRc, newRc);
  return newRc;
}

void buildPrint(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    const std::string& first) {
  auto voidLT = LLVMVoidTypeInContext(globalState->context);
  auto int8LT = LLVMInt8TypeInContext(globalState->context);
  std::vector<LLVMValueRef> indices = { constI64LE(globalState, 0) };
  auto s = LLVMBuildInBoundsGEP2(builder, int8LT, globalState->getOrMakeStringConstant(first), indices.data(), indices.size(), "stringptr");
  assert(LLVMTypeOf(s) == LLVMPointerType(int8LT, 0));
  globalState->externs->printCStr.call(builder, {s}, "");
}

void buildPrint(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    LLVMValueRef exprLE) {
  auto voidLT = LLVMVoidTypeInContext(globalState->context);

  if (LLVMTypeOf(exprLE) == LLVMInt64TypeInContext(globalState->context)) {
    globalState->externs->printInt.call(builder, {exprLE}, "");
  } else if (LLVMGetTypeKind(LLVMTypeOf(exprLE)) == LLVMIntegerTypeKind) {
    assert(LLVMSizeOfTypeInBits(globalState->dataLayout, LLVMTypeOf(exprLE)) <= 64);
    auto int64LE = LLVMBuildZExt(builder, exprLE, LLVMInt64TypeInContext(globalState->context), "");
    globalState->externs->printInt.call(builder, {int64LE}, "");
  } else if (LLVMTypeOf(exprLE) == LLVMInt32TypeInContext(globalState->context)) {
    auto i64LE = LLVMBuildZExt(builder, exprLE, LLVMInt64TypeInContext(globalState->context), "asI64");
    globalState->externs->printInt.call(builder, {i64LE}, "");
  } else if (LLVMGetTypeKind(LLVMTypeOf(exprLE)) == LLVMPointerTypeKind) {
    // It's a pointer, so interpret it as a char* and print it as a string.
    globalState->externs->printCStr.call(builder, {exprLE}, "");
  } else {
    { assert(false); throw 1337; }
  }
}

void buildPrint(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    Ref ref) {
  buildPrint(globalState, builder, ref.refLE);
}

void buildPrint(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    int num) {
  buildPrint(globalState, builder, LLVMConstInt(LLVMInt64TypeInContext(globalState->context), num, false));
}

void buildPrintToStderr(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    const std::string& first) {
  auto voidLT = LLVMVoidTypeInContext(globalState->context);
  auto int8LT = LLVMInt8TypeInContext(globalState->context);
  auto int8PtrLT = LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0);
  std::vector<LLVMValueRef> indices = { constI64LE(globalState, 0) };
  auto s = LLVMBuildInBoundsGEP2(builder, int8LT, globalState->getOrMakeStringConstant(first), indices.data(), indices.size(), "stringptr");
//  auto s = LLVMBuildLoad2(builder, int8PtrLT, globalState->getOrMakeStringConstant(first), "");
  assert(LLVMTypeOf(s) == LLVMPointerType(int8LT, 0));
  globalState->externs->printCStrToStderr.call(builder, {s}, "");
}

void buildPrintToStderr(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    LLVMValueRef exprLE) {
  auto voidLT = LLVMVoidTypeInContext(globalState->context);

  if (LLVMTypeOf(exprLE) == LLVMInt64TypeInContext(globalState->context)) {
    globalState->externs->printIntToStderr.call(builder, {exprLE}, "");
  } else if (LLVMGetTypeKind(LLVMTypeOf(exprLE)) == LLVMIntegerTypeKind) {
    assert(LLVMSizeOfTypeInBits(globalState->dataLayout, LLVMTypeOf(exprLE)) <= 64);
    auto int64LE = LLVMBuildZExt(builder, exprLE, LLVMInt64TypeInContext(globalState->context), "");
    globalState->externs->printIntToStderr.call(builder, {int64LE}, "");
  } else if (LLVMTypeOf(exprLE) == LLVMInt32TypeInContext(globalState->context)) {
    auto i64LE = LLVMBuildZExt(builder, exprLE, LLVMInt64TypeInContext(globalState->context), "asI64");
    globalState->externs->printIntToStderr.call(builder, {i64LE}, "");
  } else if (LLVMGetTypeKind(LLVMTypeOf(exprLE)) == LLVMPointerTypeKind) {
    // It's a pointer, so interpret it as a char* and print it as a string.
    globalState->externs->printCStrToStderr.call(builder, {exprLE}, "");
  } else {
    { assert(false); throw 1337; }
  }
}

void buildPrintToStderr(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    Ref ref) {
  buildPrintToStderr(globalState, builder, ref.refLE);
}

void buildPrintToStderr(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    int num) {
  buildPrintToStderr(globalState, builder, LLVMConstInt(LLVMInt64TypeInContext(globalState->context), num, false));
}

void buildAssertWithExitCode(
    GlobalState* globalState,
    LLVMValueRef function,
    LLVMBuilderRef builder,
    LLVMValueRef conditionLE,
    int exitCode,
    const std::string& failMessage) {
  buildIf(
      globalState, function, builder, isZeroLE(builder, conditionLE),
      [globalState, exitCode, failMessage](LLVMBuilderRef thenBuilder) {
        buildPrintToStderr(globalState, thenBuilder, failMessage + " Exiting!\n");
        auto exitCodeIntLE = LLVMConstInt(LLVMInt64TypeInContext(globalState->context), exitCode, false);
        globalState->externs->exit.call(thenBuilder, {exitCodeIntLE}, "");
      });
}

// We'll assert if conditionLE is false.
void buildAssert(
    GlobalState* globalState,
    LLVMValueRef function,
    LLVMBuilderRef builder,
    LLVMValueRef conditionLE,
    const std::string& failMessage) {
  buildAssertWithExitCode(globalState, function, builder, conditionLE, 1, failMessage);
}

// We'll assert if conditionLE is false.
void buildAssertV(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef conditionLE,
    const std::string& failMessage) {
  buildAssertWithExitCodeV(globalState, functionState, builder, conditionLE, 1, failMessage);
}


void buildAssertWithExitCodeV(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef conditionLE,
    int exitCode,
    const std::string& failMessage) {
  buildIfV(
      globalState, functionState, builder, isZeroLE(builder, conditionLE),
      [globalState, exitCode, failMessage](LLVMBuilderRef thenBuilder) {
        buildPrintToStderr(globalState, thenBuilder, failMessage + " Exiting!\n");
        auto exitCodeIntLE = LLVMConstInt(LLVMInt64TypeInContext(globalState->context), exitCode, false);
        globalState->externs->exit.call(thenBuilder, {exitCodeIntLE}, "");
      });
}

// We'll assert if conditionLE is false.
void buildAssertIntEq(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef aLE,
    LLVMValueRef bLE,
    const std::string& failMessage) {
  assert(LLVMTypeOf(aLE) == LLVMTypeOf(bLE));
  auto conditionLE = LLVMBuildICmp(builder, LLVMIntEQ, aLE, bLE, "assertCondition");
  buildIfV(
      globalState, functionState, builder, isZeroLE(builder, conditionLE),
      [globalState, functionState, failMessage, aLE, bLE](LLVMBuilderRef thenBuilder) {
        buildPrintToStderr(globalState, thenBuilder, "Assertion failed! Expected ");
        buildPrintToStderr(globalState, thenBuilder, aLE);
        buildPrintToStderr(globalState, thenBuilder, " to equal ");
        buildPrintToStderr(globalState, thenBuilder, bLE);
        buildPrintToStderr(globalState, thenBuilder, ".\n");
        buildPrintToStderr(globalState, thenBuilder, failMessage + " Exiting!\n");
        // See MPESC for status codes
        auto exitCodeIntLE = LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 1, false);
        globalState->externs->exit.call(thenBuilder, {exitCodeIntLE}, "");
      });
}

Ref buildInterfaceCall(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Prototype* prototype,
    ValeFuncPtrLE methodFunctionPtrLE,
    std::vector<Ref> argRefs,
    int virtualParamIndex) {
  auto virtualParamMT = prototype->params[virtualParamIndex];

  auto interfaceKindM = dynamic_cast<InterfaceKind*>(virtualParamMT->kind);
  assert(interfaceKindM);
//  int indexInEdge = globalState->getInterfaceMethod(interfaceKindM, prototype);

  auto virtualArgRef = argRefs[virtualParamIndex];

  LLVMValueRef itablePtrLE = nullptr;
  LLVMValueRef newVirtualArgLE = nullptr;
  std::tie(itablePtrLE, newVirtualArgLE) =
      globalState->getRegion(virtualParamMT)
          ->explodeInterfaceRef(
              functionState, builder, virtualParamMT, virtualArgRef);

  //buildFlare(FL(), globalState, functionState, builder, "Doing an interface call, objPtrLE: ", ptrToIntLE(globalState, builder, newVirtualArgLE), " itablePtrLE ", ptrToIntLE(globalState, builder, itablePtrLE));

  // We can't represent these arguments as refs, because this new virtual arg is a void*, and we
  // can't represent that as a ref.
  std::vector<LLVMValueRef> argsLE;
  for (int i = 0; i < argRefs.size(); i++) {
    argsLE.push_back(
        globalState->getRegion(prototype->params[i])
            ->checkValidReference(FL(),
                functionState, builder, false, prototype->params[i], argRefs[i]));
  }
  argsLE[virtualParamIndex] = newVirtualArgLE;

  buildFlare(FL(), globalState, functionState, builder);
  //buildFlare(FL(), globalState, functionState, builder, interfaceKindM->fullName->name, " ", ptrToIntLE(globalState, builder, methodFunctionPtrLE));

//  assert(LLVMGetTypeKind(LLVMTypeOf(itablePtrLE)) == LLVMPointerTypeKind);
//  auto funcPtrPtrLE =
//      unmigratedLLVMBuildStructGEP(
//          builder, itablePtrLE, indexInEdge, "methodPtrPtr");

//  auto funcPtrLE = unmigratedLLVMBuildLoad(builder, funcPtrPtrLE, "methodPtr");



  assert(
      LLVMTypeOf(newVirtualArgLE) ==
      globalState->getRegion(virtualParamMT)
          ->getInterfaceMethodVirtualParamAnyType(virtualParamMT));
  auto resultLE = methodFunctionPtrLE.call(builder, argsLE, "");
  assert(LLVMTypeOf(resultLE) == LLVMGetReturnType(methodFunctionPtrLE.inner.funcLT));
  buildFlare(FL(), globalState, functionState, builder);
  return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, resultLE);
}

LLVMValueRef makeConstExpr(FunctionState* functionState, LLVMBuilderRef builder, LLVMTypeRef type, LLVMValueRef constExpr) {
  auto localAddr = makeBackendLocal(functionState, builder, type, "", constExpr);
  return LLVMBuildLoad2(builder, type, localAddr, "");
}

LLVMValueRef makeConstIntExpr(FunctionState* functionState, LLVMBuilderRef builder, LLVMTypeRef type, int64_t value) {
  return makeConstExpr(functionState, builder, type, LLVMConstInt(type, value, false));
}

void buildAssertCensusContains(
    AreaAndFileAndLine checkerAFL,
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef ptrLE) {
  if (globalState->opt->census) {
    LLVMValueRef resultAsVoidPtrLE =
        LLVMBuildPointerCast(
            builder, ptrLE, LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0), "");

    auto isNullLE = LLVMBuildIsNull(builder, resultAsVoidPtrLE, "isNull");
    buildIfV(
        globalState, functionState, builder, isNullLE,
        [globalState, checkerAFL, ptrLE](LLVMBuilderRef thenBuilder) {
          buildPrintAreaAndFileAndLineToStderr(globalState, thenBuilder, checkerAFL);
          buildPrintToStderr(globalState, thenBuilder, "Object null, so not in census, exiting!\n");
          // See MPESC for status codes
          auto exitCodeIntLE = LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 14, false);
          globalState->externs->exit.call(thenBuilder, {exitCodeIntLE}, "");
        });

    auto isRegisteredIntLE =
        globalState->externs->censusContains.call(builder, {resultAsVoidPtrLE}, "");
    auto isRegisteredBoolLE =
        LLVMBuildTruncOrBitCast(
            builder, isRegisteredIntLE, LLVMInt1TypeInContext(globalState->context), "");
    buildIfV(
        globalState, functionState, builder, isZeroLE(builder, isRegisteredBoolLE),
        [globalState, checkerAFL, ptrLE](LLVMBuilderRef thenBuilder) {
          buildPrintAreaAndFileAndLineToStderr(globalState, thenBuilder, checkerAFL);
          buildPrintToStderr(globalState, thenBuilder, "Object &");
          buildPrintToStderr(globalState, thenBuilder, ptrToIntLE(globalState, thenBuilder, ptrLE));
          buildPrintToStderr(globalState, thenBuilder, " not registered with census, exiting!\n");
          // See MPESC for status codes
          auto exitCodeIntLE = LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 14, false);
          globalState->externs->exit.call(thenBuilder, {exitCodeIntLE}, "");
        });
  }
}

Ref buildCallV(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Prototype* prototype,
    std::vector<Ref> argRefs) {
  auto funcL = globalState->lookupFunction(prototype);

  buildFlare(FL(), globalState, functionState, builder, "Suspending function ", functionState->containingFuncName);
  buildFlare(FL(), globalState, functionState, builder, "Calling function ", prototype->name->name);

  std::vector<LLVMValueRef> argsLE;
  for (int i = 0; i < argRefs.size(); i++) {
    argsLE.push_back(
        globalState->getRegion(prototype->params[i])
            ->checkValidReference(FL(),
                functionState, builder, false, prototype->params[i], argRefs[i]));
  }

  buildFlare(FL(), globalState, functionState, builder, "Doing call");

  auto resultLE = funcL.call(builder, argsLE, "");

  buildFlare(FL(), globalState, functionState, builder, "Done with call");

  auto resultRef = toRef(globalState->getRegion(prototype->returnType), prototype->returnType, resultLE);
  globalState->getRegion(prototype->returnType)
      ->checkValidReference(FL(), functionState, builder, false, prototype->returnType, resultRef);

  if (prototype->returnType->kind == globalState->metalCache->never) {
    buildFlare(FL(), globalState, functionState, builder, "Done calling function ", prototype->name->name);
    buildFlare(FL(), globalState, functionState, builder, "Resuming function ", functionState->containingFuncName);
    LLVMBuildRet(builder, LLVMGetUndef(functionState->returnTypeL));
    return toRef(globalState->getRegion(globalState->metalCache->neverRef), globalState->metalCache->neverRef, globalState->neverPtrLE);
  } else {
    buildFlare(FL(), globalState, functionState, builder, "Done calling function ", prototype->name->name);
    buildFlare(FL(), globalState, functionState, builder, "Resuming function ", functionState->containingFuncName);
    return resultRef;
  }
}


LLVMValueRef buildMaybeNeverCall(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    RawFuncPtrLE funcL,
    std::vector<LLVMValueRef> argsLE) {
  auto resultLE = funcL.call(builder, argsLE, "");

  auto returnLT = LLVMGetReturnType(funcL.funcLT);
  if (returnLT == makeNeverType(globalState)) {
    LLVMBuildRet(builder, LLVMGetUndef(returnLT));
    return globalState->neverPtrLE;
  } else {
    return resultLE;
  }
}

LLVMValueRef buildMaybeNeverCallV(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    ValeFuncPtrLE functionLE,
    std::vector<LLVMValueRef> argsLE) {
  return buildMaybeNeverCall(globalState, builder, functionLE.inner, argsLE);
}

RawFuncPtrLE addExtern(LLVMModuleRef mod, const std::string& name, LLVMTypeRef retType, std::vector<LLVMTypeRef> paramTypes) {
  auto funcLT = LLVMFunctionType(retType, paramTypes.data(), paramTypes.size(), 0);
  auto funcLE = LLVMAddFunction(mod, name.c_str(), funcLT);
  return RawFuncPtrLE(funcLT, funcLE);
}
