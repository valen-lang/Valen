#include <utils/branch.h>
#include <utils/call.h>
#include <region/common/migration.h>
#include "function/function.h"
#include "function/expressions/expressions.h"
#include "determinism/determinism.h"
#include "globalstate.h"
#include "translatetype.h"
#include <region/common/migration.h>
#include <utils/counters.h>

std::tuple<RawFuncPtrLE, LLVMBuilderRef> makeStringSetupFunction(GlobalState* globalState) {
  auto voidLT = LLVMVoidTypeInContext(globalState->context);

  auto functionL = addRawFunction(globalState->mod, "__Vale_SetupStrings", voidLT, {});

  auto stringsBuilder = LLVMCreateBuilderInContext(globalState->context);
  LLVMBasicBlockRef blockL = LLVMAppendBasicBlockInContext(globalState->context, functionL.ptrLE, "stringsBlock");
  LLVMPositionBuilderAtEnd(stringsBuilder, blockL);
  auto ret = LLVMBuildRetVoid(stringsBuilder);
  LLVMPositionBuilderBefore(stringsBuilder, ret);

  return {functionL, stringsBuilder};
}


Prototype* makeValeMainFunction(
    GlobalState* globalState,
    RawFuncPtrLE stringSetupFunctionL,
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
      globalState->metalCache->getPrototype(valeMainName, globalState->metalCache->i64Ref, {});
  declareAndDefineExtraFunction(
      globalState, valeMainProto, valeMainName->name,
      [globalState, stringSetupFunctionL, mainSetupFuncProto, int64LT, userMainFunctionPrototype, mainCleanupFunctionPrototype](
          FunctionState *functionState, LLVMBuilderRef entryBuilder) {
        buildFlare(FL(), globalState, functionState, entryBuilder);

        stringSetupFunctionL.call(entryBuilder, {}, "");
        globalState->lookupFunction(mainSetupFuncProto)
            .call(entryBuilder, {}, "");

//        LLVMBuildStore(
//            entryBuilder,
//            LLVMBuildUDiv(
//                entryBuilder,
//                LLVMBuildPointerCast(
//                    entryBuilder,
//                    globalState->writeOnlyGlobalLE,
//                    LLVMInt64TypeInContext(globalState->context),
//                    "ptrAsIntToWriteOnlyGlobal"),
//                constI64LE(globalState, 8),
//                "ram64IndexToWriteOnlyGlobal"),
//            globalState->ram64IndexToWriteOnlyGlobal);

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

        if (userMainFunctionPrototype->returnType->kind == globalState->metalCache->vooid) {
          buildFlare(FL(), globalState, functionState, entryBuilder);
          LLVMBuildRet(entryBuilder, LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 0, true));
        } else if (userMainFunctionPrototype->returnType->kind == globalState->metalCache->i64) {
          buildFlare(FL(), globalState, functionState, entryBuilder, userMainResultLE);
          LLVMBuildRet(entryBuilder, userMainResultLE);
        } else if (userMainFunctionPrototype->returnType->kind == globalState->metalCache->i32) {
          buildFlare(FL(), globalState, functionState, entryBuilder, userMainResultLE);
          LLVMBuildRet(entryBuilder, LLVMBuildZExt(entryBuilder, userMainResultLE, LLVMInt64TypeInContext(globalState->context), "extended"));
        } else if (userMainFunctionPrototype->returnType->kind == globalState->metalCache->never) {
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
    Prototype* valeMainPrototype) {
  auto voidLT = LLVMVoidTypeInContext(globalState->context);
  auto int1LT = LLVMInt1TypeInContext(globalState->context);
  auto int8LT = LLVMInt8TypeInContext(globalState->context);
  auto int32LT = LLVMInt32TypeInContext(globalState->context);
  auto int32PtrLT = LLVMPointerType(int32LT, 0);
  auto int64LT = LLVMInt64TypeInContext(globalState->context);
  auto voidPtrLT = LLVMPointerType(int8LT, 0);
  auto int8PtrLT = LLVMPointerType(int8LT, 0);
  auto int8PtrPtrLT = LLVMPointerType(int8PtrLT, 0);

  // This is the actual entry point for the binary. Uses the standard C
  // signature `int main(int argc, char** argv)` so wasi-libc's _start
  // shim (which expects exactly that) can find and call it. argc gets
  // sign-extended to i64 before being stored into Vale's i64-typed
  // numMainArgsLE global, and the Vale main's i64 return is truncated
  // to i32 on the way out (POSIX exit codes only use the low byte).
  auto entryParamsLT = std::vector<LLVMTypeRef>{ int32LT, LLVMPointerType(LLVMPointerType(int8LT, 0), 0) };
  LLVMTypeRef functionTypeL = LLVMFunctionType(int32LT, entryParamsLT.data(), entryParamsLT.size(), 0);
  LLVMValueRef entryFunctionL = LLVMAddFunction(globalState->mod, "main", functionTypeL);

  LLVMSetDLLStorageClass(entryFunctionL, LLVMDLLExportStorageClass);
  LLVMSetFunctionCallConv(entryFunctionL, LLVMCCallConv );
  // wasi-libc's `_start` -> `__main_void` -> `__main_argc_argv` (weak
  // undef). It does NOT call `main` directly. Expose our `main` under
  // both names so the wasi crt resolves to it. On native targets the
  // alias is harmless (the C runtime calls `main`).
  LLVMAddAlias2(
      globalState->mod, functionTypeL, 0, entryFunctionL, "__main_argc_argv");
  LLVMBuilderRef entryBuilder = LLVMCreateBuilderInContext(globalState->context);
  LLVMBasicBlockRef blockL =
      LLVMAppendBasicBlockInContext(globalState->context, entryFunctionL, "thebestblock");
  LLVMPositionBuilderAtEnd(entryBuilder, blockL);


  auto numMainArgsI32LE = LLVMGetParam(entryFunctionL, 0);
  auto numMainArgsLE = LLVMBuildSExt(entryBuilder, numMainArgsI32LE, int64LT, "argcI64");
  auto mainArgsLE = LLVMGetParam(entryFunctionL, 1);
  LLVMBuildStore(entryBuilder, numMainArgsLE, globalState->numMainArgsLE);
  LLVMBuildStore(entryBuilder, mainArgsLE, globalState->mainArgsLE);

  if (globalState->opt->enableReplaying) {
    auto numConsumedArgsLE =
        globalState->determinism->buildMaybeStartDeterministicMode(
            entryBuilder, numMainArgsLE, mainArgsLE);

    // argv[numConsumed] = argv[0], to move the zeroth arg up.
    LLVMBuildStore(
        entryBuilder,
        LLVMBuildLoad2(entryBuilder, int8PtrPtrLT, mainArgsLE, "zerothArg"),
        LLVMBuildInBoundsGEP2(entryBuilder, int8PtrPtrLT, mainArgsLE, &numConsumedArgsLE, 1, "argv+numConsumed"));
    // argv += numConsumed
    mainArgsLE = LLVMBuildInBoundsGEP2(entryBuilder, int8PtrPtrLT, mainArgsLE, &numConsumedArgsLE, 1, "newMainArgs");
    // argc -= numConsumed
    numMainArgsLE = LLVMBuildSub(entryBuilder, numMainArgsLE, numConsumedArgsLE, "newMainArgsCount");
  }

  auto calleeUserFunction = globalState->lookupFunction(valeMainPrototype);
  auto calleeUserFunctionReturnMT = valeMainPrototype->returnType;
  auto calleeUserFunctionReturnLT =
      globalState->getRegion(calleeUserFunctionReturnMT)->translateType(calleeUserFunctionReturnMT);
  auto resultLE =
      buildMaybeNeverCallV(
          globalState, entryBuilder, calleeUserFunction, {});

  if (globalState->opt->enableReplaying) {
    globalState->determinism->buildMaybeStopDeterministicMode(
        entryFunctionL, entryBuilder);
  }

  // Vale main returns i64 (Vale Int); C main returns i32. Truncate.
  // POSIX/wasi exit codes only use the low byte anyway.
  auto resultI32LE = LLVMBuildTrunc(entryBuilder, resultLE, int32LT, "exitCodeI32");
  LLVMBuildRet(entryBuilder, resultI32LE);
  LLVMDisposeBuilder(entryBuilder);

  return entryFunctionL;
}