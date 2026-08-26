#include <utils/branch.h>
#include <utils/call.h>
#include <utils/definefunction.h>
#include <region/common/migration.h>
#include "function/function.h"
#include "function/expressions/expressions.h"
#include "globalstate.h"
#include "translatetype.h"
#include <region/common/migration.h>
#include <utils/counters.h>

Prototype* makeValeMainFunction(
    GlobalState* globalState,
    Prototype* mainSetupFuncProto,
    Prototype* userMainFunctionPrototype,
    Prototype* mainCleanupFunctionPrototype) {
  auto voidLT = LLVMVoidTypeInContext(globalState->context);
  auto int1LT = LLVMInt1TypeInContext(globalState->context);
  auto int8LT = LLVMInt8TypeInContext(globalState->context);
  auto int32LT = LLVMInt32TypeInContext(globalState->context);
  auto int32PtrLT = LLVMPointerType(int32LT, 0);
  auto int64LT = LLVMInt64TypeInContext(globalState->context);
  auto voidPtrLT = LLVMPointerType(int8LT, 0);
  auto int8PtrLT = LLVMPointerType(int8LT, 0);

  auto valeMainName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, "__Vale_Main");
  auto valeMainProto =
      globalState->metalCache->getPrototype(valeMainName, globalState->metalCache->i64Type, {});
  declareAndDefineExtraFunction(
      globalState, valeMainProto, valeMainName->name,
      [globalState, mainSetupFuncProto, int64LT, userMainFunctionPrototype, mainCleanupFunctionPrototype](
          FunctionState *functionState, LLVMBuilderRef entryBuilder) {
        buildFlare(FL(), globalState, functionState, entryBuilder);

        globalState->lookupFunction(mainSetupFuncProto)
            .call(entryBuilder, {}, "");

        buildFlare(FL(), globalState, functionState, entryBuilder);
        if (globalState->opt->census) {
          // Add all the edges to the census, so we can check that fat pointers are right.
          // We remove them again at the end of outer main.
          // We should one day do this for all globals.
          for (auto edgeAndItablePtr : globalState->interfaceTablePtrs) {
            auto itablePtrLE = edgeAndItablePtr.second;
            LLVMValueRef itablePtrAsVoidPtrLE =
                LLVMBuildBitCast(
                    entryBuilder, itablePtrLE, LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0), "");

            //buildFlare(FL(), globalState, functionState, entryBuilder, ptrToIntLE(globalState, entryBuilder, itablePtrAsVoidPtrLE));
            globalState->externs->censusAdd.call(entryBuilder, {itablePtrAsVoidPtrLE}, "");
          }
          buildFlare(FL(), globalState, functionState, entryBuilder);
        }
        buildFlare(FL(), globalState, functionState, entryBuilder);

        auto userMainResultRef = buildCallV(globalState, functionState, entryBuilder, userMainFunctionPrototype, {});
        auto userMainResultLE =
            globalState->getRegion(userMainFunctionPrototype->returnType)
                ->checkValidReference(
                    FL(), functionState, entryBuilder, true, userMainFunctionPrototype->returnType, userMainResultRef);

        buildFlare(FL(), globalState, functionState, entryBuilder);
        buildCallV(globalState, functionState, entryBuilder, mainCleanupFunctionPrototype, {});
        buildFlare(FL(), globalState, functionState, entryBuilder);

        if (globalState->opt->printMemOverhead) {
          buildPrintToStderr(globalState, entryBuilder, "\nRC adjustments: ");
          buildPrintToStderr(
              globalState, entryBuilder,
              LLVMBuildLoad2(entryBuilder, int64LT, globalState->mutRcAdjustCounterLE, "rcadjusts"));

          buildPrintToStderr(globalState, entryBuilder, "\n");
        }

        if (globalState->opt->census) {
          buildFlare(FL(), globalState, functionState, entryBuilder);
          // Remove all the things from the census that we added at the start of the program.
          for (auto edgeAndItablePtr : globalState->interfaceTablePtrs) {
            auto itablePtrLE = edgeAndItablePtr.second;
            LLVMValueRef itablePtrAsVoidPtrLE =
                LLVMBuildBitCast(
                    entryBuilder, itablePtrLE, LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0), "");
            globalState->externs->censusRemove.call(entryBuilder, {itablePtrAsVoidPtrLE}, "");
          }
          buildFlare(FL(), globalState, functionState, entryBuilder);

          std::vector<LLVMValueRef> numLiveObjAssertArgs = {
              LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 0, false),
              LLVMBuildLoad2(entryBuilder, int64LT, globalState->liveHeapObjCounterLE, "numLiveObjs"),
              globalState->getOrMakeStringConstant("Memory leaks!"),
          };
          globalState->externs->assertI64Eq.call(entryBuilder, numLiveObjAssertArgs, "");
        }
        buildFlare(FL(), globalState, functionState, entryBuilder);

        if (userMainFunctionPrototype->returnType == globalState->metalCache->voidType) {
          buildFlare(FL(), globalState, functionState, entryBuilder);
          LLVMBuildRet(entryBuilder, LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 0, true));
        } else if (userMainFunctionPrototype->returnType == globalState->metalCache->i64Type) {
          buildFlare(FL(), globalState, functionState, entryBuilder, userMainResultLE);
          LLVMBuildRet(entryBuilder, userMainResultLE);
        } else if (userMainFunctionPrototype->returnType == globalState->metalCache->i32Type) {
          buildFlare(FL(), globalState, functionState, entryBuilder, userMainResultLE);
          LLVMBuildRet(entryBuilder, LLVMBuildZExt(entryBuilder, userMainResultLE, LLVMInt64TypeInContext(globalState->context), "extended"));
        } else if (userMainFunctionPrototype->returnType == globalState->metalCache->neverType) {
          buildFlare(FL(), globalState, functionState, entryBuilder);
          LLVMBuildRet(entryBuilder, LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 0, true));
        } else {
          { assert(false); throw 1337; }
        }

        return userMainResultLE;
      });

  return valeMainProto;
}

LLVMValueRef makeEntryFunction(
    GlobalState* globalState,
    Prototype* valeMainPrototype,
    const std::string& entryName,
    bool emitLibcShim) {
  auto voidLT = LLVMVoidTypeInContext(globalState->context);
  auto int1LT = LLVMInt1TypeInContext(globalState->context);
  auto int8LT = LLVMInt8TypeInContext(globalState->context);
  auto int32LT = LLVMInt32TypeInContext(globalState->context);
  auto int32PtrLT = LLVMPointerType(int32LT, 0);
  auto int64LT = LLVMInt64TypeInContext(globalState->context);
  auto voidPtrLT = LLVMPointerType(int8LT, 0);
  auto int8PtrLT = LLVMPointerType(int8LT, 0);

  // Standalone/owned mode (emitLibcShim) makes `entryName` the actual libc entry:
  // `int main(int argc, char** argv)`, so wasi-libc's _start shim (which expects exactly
  // that) can find and call it; argc gets sign-extended to i64 and stored into Vale's
  // arg globals, and the Vale main's i64 return is truncated to i32 (POSIX exit codes use
  // the low byte). Borrowed/rustc mode emits a plain `int <entryName>()` (e.g.
  // `__vale_main`) — rustc's own `main` already ran libc startup and owns argc/argv, so
  // no params, no wasi alias, no arg reads.
  LLVMTypeRef functionTypeL;
  if (emitLibcShim) {
    auto entryParamsLT = std::vector<LLVMTypeRef>{ int32LT, LLVMPointerType(LLVMPointerType(int8LT, 0), 0) };
    functionTypeL = LLVMFunctionType(int32LT, entryParamsLT.data(), entryParamsLT.size(), 0);
  } else {
    functionTypeL = LLVMFunctionType(int32LT, nullptr, 0, 0);
  }
  LLVMValueRef entryFunctionL = LLVMAddFunction(globalState->mod, entryName.c_str(), functionTypeL);

  LLVMSetDLLStorageClass(entryFunctionL, LLVMDLLExportStorageClass);
  LLVMSetFunctionCallConv(entryFunctionL, LLVMCCallConv );
  if (emitLibcShim) {
    // wasi-libc's `_start` -> `__main_void` -> `__main_argc_argv` (weak undef). It does
    // NOT call `main` directly. Expose our `main` under both names so the wasi crt
    // resolves to it. On native targets the alias is harmless (the C runtime calls `main`).
    LLVMAddAlias2(
        globalState->mod, functionTypeL, 0, entryFunctionL, "__main_argc_argv");
  }
  LLVMBuilderRef entryBuilder = LLVMCreateBuilderInContext(globalState->context);
  LLVMBasicBlockRef blockL =
      LLVMAppendBasicBlockInContext(globalState->context, entryFunctionL, "thebestblock");
  LLVMPositionBuilderAtEnd(entryBuilder, blockL);


  if (emitLibcShim) {
    auto numMainArgsI32LE = LLVMGetParam(entryFunctionL, 0);
    auto numMainArgsLE = LLVMBuildSExt(entryBuilder, numMainArgsI32LE, int64LT, "argcI64");
    auto mainArgsLE = LLVMGetParam(entryFunctionL, 1);
    LLVMBuildStore(entryBuilder, numMainArgsLE, globalState->numMainArgsLE);
    LLVMBuildStore(entryBuilder, mainArgsLE, globalState->mainArgsLE);
  }

  auto calleeUserFunction = globalState->lookupFunction(valeMainPrototype);
  auto calleeUserFunctionReturnMT = valeMainPrototype->returnType;
  auto calleeUserFunctionReturnLT =
      globalState->getRegion(calleeUserFunctionReturnMT)->translateType(calleeUserFunctionReturnMT);
  auto resultLE =
      buildMaybeNeverCallV(
          globalState, entryBuilder, calleeUserFunction, {});

  // Vale main returns i64 (Vale Int); C main returns i32. Truncate.
  // POSIX/wasi exit codes only use the low byte anyway.
  auto resultI32LE = LLVMBuildTrunc(entryBuilder, resultLE, int32LT, "exitCodeI32");
  LLVMBuildRet(entryBuilder, resultI32LE);
  LLVMDisposeBuilder(entryBuilder);

  return entryFunctionL;
}