
#include "function/expressions/expressions.h"
#include "externs.h"
#include "globalstate.h"
#include "utils/definefunction.h"

Externs::Externs(LLVMModuleRef mod, LLVMContextRef context, int ptrSizeBits) {
  auto emptyLT = LLVMStructTypeInContext(context, nullptr, 0, false);
  auto emptyPtrLT = LLVMPointerType(emptyLT, 0);
  auto voidLT = LLVMVoidTypeInContext(context);
  auto int1LT = LLVMInt1TypeInContext(context);
  auto int8LT = LLVMInt8TypeInContext(context);
  auto int32LT = LLVMInt32TypeInContext(context);
  auto int32PtrLT = LLVMPointerType(int32LT, 0);
  auto int64LT = LLVMInt64TypeInContext(context);
  auto voidPtrLT = LLVMPointerType(int8LT, 0);
  auto int8PtrLT = LLVMPointerType(int8LT, 0);
  auto int256LT = LLVMIntTypeInContext(context, 256);
  // C `size_t` width — matches pointer width on every target we care
  // about. i64 on x86_64/arm64, i32 on wasm32. Used for libc functions
  // whose signatures take/return size_t (malloc, memcpy, strlen, etc.).
  auto sizeTLT = LLVMIntTypeInContext(context, ptrSizeBits);
  // C `int` width — always i32 on the targets we support. Used for libc
  // functions that take/return `int` (exit, getchar, fclose, strncmp ret).
  auto cIntLT = int32LT;

  censusContains = addExtern(mod, "__vcensusContains", int64LT, {voidPtrLT});
  censusAdd = addExtern(mod, "__vcensusAdd", voidLT, {voidPtrLT});
  censusRemove = addExtern(mod, "__vcensusRemove", voidLT, {voidPtrLT});
  malloc = addExtern(mod, "malloc", int8PtrLT, {sizeTLT});
  free = addExtern(mod, "free", voidLT, {int8PtrLT});
  exit = addExtern(mod, "exit", voidLT, {cIntLT});
  perror = addExtern(mod, "perror", voidLT, {int8PtrLT});
  assert = addExtern(mod, "__vassert", voidLT, {int1LT, int8PtrLT});
  assertI64Eq = addExtern(mod, "__vassertI64Eq", voidLT, {int64LT, int64LT, int8PtrLT});
  printCStr = addExtern(mod, "__vprintCStr", voidLT, {int8PtrLT});
  printCStrToStderr = addExtern(mod, "__vprintCStrToStderr", voidLT, {int8PtrLT});
  getch = addExtern(mod, "getchar", cIntLT, {});
  printInt = addExtern(mod, "__vprintI64", voidLT, {int64LT});
  printIntToStderr = addExtern(mod, "__vprintI64ToStderr", voidLT, {int64LT});
  strlen = addExtern(mod, "strlen", sizeTLT, {int8PtrLT});
  strncpy = addExtern(mod, "strncpy", int8PtrLT, {int8PtrLT, int8PtrLT, sizeTLT});
  strncmp = addExtern(mod, "strncmp", cIntLT, {int8PtrLT, int8PtrLT, sizeTLT});
  memcpy = addExtern(mod, "memcpy", int8PtrLT, {int8PtrLT, int8PtrLT, sizeTLT});
  memset = addExtern(mod, "memset", int8PtrLT, {int8PtrLT, cIntLT, sizeTLT});

  fopen = addExtern(mod, "fopen", int8PtrLT, {int8PtrLT, int8PtrLT});
  fclose = addExtern(mod, "fclose", cIntLT, {int8PtrLT});
  fread = addExtern(mod, "fread", sizeTLT, {int8PtrLT, sizeTLT, sizeTLT, int8PtrLT});
  fwrite = addExtern(mod, "fwrite", sizeTLT, {int8PtrLT, sizeTLT, sizeTLT, int8PtrLT});

  strHasherCallLF = addExtern(mod, "strHasherCall", int64LT, {emptyPtrLT, int8PtrLT});
  strEquatorCallLF = addExtern(mod, "strEquatorCall", int1LT, {emptyPtrLT, int8PtrLT, int8PtrLT});

  int256HasherCallLF =
      addRawFunction(mod, "int256HasherCall", int64LT, {emptyPtrLT, int256LT});
  defineRawFunctionBody(
      context, int256HasherCallLF.ptrLE, int64LT, "int256HasherCall",
      [int64LT, int256LT](FunctionState* functionState, LLVMBuilderRef builder) {
        // Ignore 'this' arg 0
        auto int256LE = LLVMGetParam(functionState->containingFuncL, 1);
        auto maskLE = LLVMConstInt(int256LT, 0xFFFFFFFFFFFFFFFF, false);
        auto firstI256 = LLVMBuildAnd(builder, int256LE, maskLE, "x1as256");
        auto firstI64 = LLVMBuildTrunc(builder, firstI256, int64LT, "x1");

        auto secondShiftLE = LLVMConstInt(int256LT, 64 * 1, false);
        auto secondMaskLE = LLVMBuildShl(builder, maskLE, secondShiftLE, "m2");
        auto unshiftedSecondI64LE = LLVMBuildAnd(builder, int256LE, secondMaskLE, "u2");
        auto secondI256 = LLVMBuildLShr(builder, unshiftedSecondI64LE, secondShiftLE, "x2");
        auto secondI64 = LLVMBuildTrunc(builder, secondI256, int64LT, "x1");

        auto thirdShiftLE = LLVMConstInt(int256LT, 64 * 2, false);
        auto thirdMaskLE = LLVMBuildShl(builder, maskLE, thirdShiftLE, "m3");
        auto unshiftedThirdI64LE = LLVMBuildAnd(builder, int256LE, thirdMaskLE, "u3");
        auto thirdI256 = LLVMBuildLShr(builder, unshiftedThirdI64LE, thirdShiftLE, "x3");
        auto thirdI64 = LLVMBuildTrunc(builder, thirdI256, int64LT, "x1");

        auto fourthShiftLE = LLVMConstInt(int256LT, 64 * 3, false);
        auto fourthMaskLE = LLVMBuildShl(builder, maskLE, fourthShiftLE, "m4");
        auto unshiftedFourthI64LE = LLVMBuildAnd(builder, int256LE, fourthMaskLE, "u4");
        auto fourthI256 = LLVMBuildLShr(builder, unshiftedFourthI64LE, fourthShiftLE, "x4");
        auto fourthI64 = LLVMBuildTrunc(builder, fourthI256, int64LT, "x1");

        auto resultLE = firstI64;
        resultLE = LLVMBuildXor(builder, resultLE, secondI64, "r2");
        resultLE = LLVMBuildXor(builder, resultLE, thirdI64, "r3");
        resultLE = LLVMBuildXor(builder, resultLE, fourthI64, "r4");
        LLVMBuildRet(builder, resultLE);
      });
  int256EquatorCallLF =
      addRawFunction(mod, "int256EquatorCall", int1LT, {emptyPtrLT, int256LT, int256LT});
  defineRawFunctionBody(
      context, int256EquatorCallLF.ptrLE, int1LT, "int256HasherCall",
      [](FunctionState* functionState, LLVMBuilderRef builder) {
        // Ignore 'this' arg 0
        LLVMGetParam(functionState->containingFuncL, 0);
        auto firstInt256LE = LLVMGetParam(functionState->containingFuncL, 1);
        auto secondInt256LE = LLVMGetParam(functionState->containingFuncL, 2);
        auto resultLE = LLVMBuildICmp(builder, LLVMIntEQ, firstInt256LE, secondInt256LE, "equal");
        LLVMBuildRet(builder, resultLE);
      });

//  initTwinPages = addExtern(mod, "__vale_initTwinPages", int8PtrLT, {});
}

bool hasEnding(std::string const &fullString, std::string const &ending) {
  if (fullString.length() >= ending.length()) {
    return (0 == fullString.compare(fullString.length() - ending.length(), ending.length(), ending));
  } else {
    return false;
  }
}

bool includeSizeParam(GlobalState* globalState, Prototype* prototype, int paramIndex) {
  // See SASP for what this is all about.
  if (hasEnding(prototype->name->name, "_vasp")) {
    auto paramMT = prototype->params[paramIndex];
    if (dynamic_cast<StructKind*>(paramMT->kind) ||
        dynamic_cast<InterfaceKind*>(paramMT->kind) ||
        dynamic_cast<StaticSizedArrayT*>(paramMT->kind) ||
        dynamic_cast<RuntimeSizedArrayT*>(paramMT->kind)) {
      return true;
    }
  }
  return false;
}

