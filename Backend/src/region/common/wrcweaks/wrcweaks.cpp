#include <llvm-c/Types.h>
#include "../../../globalstate.h"
#include "../../../function/function.h"
#include "../../../function/expressions/shared/shared.h"
#include "../controlblock.h"
#include "../../../utils/counters.h"
#include "../../../utils/branch.h"
#include "../common.h"
#include "wrcweaks.h"
#include <region/common/migration.h>

constexpr int WEAK_REF_HEADER_MEMBER_INDEX_FOR_WRCI = 0;

constexpr uint32_t WRC_ALIVE_BIT = 0x80000000;
constexpr uint32_t WRC_INITIAL_VALUE = WRC_ALIVE_BIT;

static LLVMValueRef makeWrciHeader(
    LLVMBuilderRef builder,
    KindStructs* weakRefStructs,
    Kind* kind,
    LLVMValueRef wrciLE) {
  auto headerLE = LLVMGetUndef(weakRefStructs->getWeakRefHeaderStruct());
  return LLVMBuildInsertValue(builder, headerLE, wrciLE, WEAK_REF_HEADER_MEMBER_INDEX_FOR_WRCI, "header");
}

void WrcWeaks::buildCheckWrc(
    LLVMBuilderRef builder,
    LLVMValueRef wrciLE) {
  // fine, proceed
  std::vector<LLVMValueRef> checkWrcsArgs = {
      wrcTablePtrLE,
      wrciLE,
  };
  globalState->checkWrci.call(builder, checkWrcsArgs, "");
}

LLVMValueRef WrcWeaks::getWrciFromWeakRef(
    LLVMBuilderRef builder,
    WeakFatPtrLE weakFatPtrLE) {
//  assert(globalState->opt->regionOverride != RegionOverride::RESILIENT_V1);
  auto headerLE = fatWeaks_.getHeaderFromWeakRef(builder, weakFatPtrLE);
  return LLVMBuildExtractValue(builder, headerLE, WEAK_REF_HEADER_MEMBER_INDEX_FOR_WRCI, "wrci");
}

void WrcWeaks::maybeReleaseWrc(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef wrciLE,
    LLVMValueRef ptrToWrcLE,
    LLVMValueRef wrcLE) {
  auto int32LT = LLVMInt32TypeInContext(globalState->context);
  buildIfV(
      globalState, functionState,
      builder,
      isZeroLE(builder, wrcLE),
      [this, functionState, wrciLE, ptrToWrcLE, int32LT](LLVMBuilderRef thenBuilder) {
        // __wrc_entries[wrcIndex] = __wrc_firstFree;
        LLVMBuildStore(
            thenBuilder,
            LLVMBuildLoad2(
                thenBuilder, int32LT, getWrcFirstFreeWrciPtr(thenBuilder), "firstFreeWrci"),
            ptrToWrcLE);
        // __wrc_firstFree = wrcIndex;
        LLVMBuildStore(thenBuilder, wrciLE, getWrcFirstFreeWrciPtr(thenBuilder));
      });
}

static LLVMValueRef getWrciFromControlBlockPtr(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    Kind* refM,
    ControlBlockPtrLE controlBlockPtr) {
  { assert(false); throw 1337; }
//   auto int32LT = LLVMInt32TypeInContext(globalState->context);
// //  assert(globalState->opt->regionOverride != RegionOverride::RESILIENT_V1);
//
//   if (dynamic_cast<ShareRef*>(refM) != nullptr) {
//     // Shares never have weak refs
//     { assert(false); throw 1337; }
//     return nullptr;
//   } else {
//     auto wrciPtrLE =
//         LLVMBuildStructGEP2(
//             builder,
//             controlBlockPtr.structLT,
//             controlBlockPtr.refLE,
//             structs->getControlBlock(refM->kind)->getMemberIndex(ControlBlockMember::WRCI_32B),
//             "wrciPtr");
//     return LLVMBuildLoad2(builder, int32LT, wrciPtrLE, "wrci");
//   }
}

LLVMValueRef WrcWeaks::getWrcPtr(
    LLVMBuilderRef builder,
    LLVMValueRef wrciLE) {
  auto int32LT = LLVMInt32TypeInContext(globalState->context);
  auto int32PtrLT = LLVMPointerType(int32LT, 0);
  auto wrcEntriesPtrLE =
      LLVMBuildLoad2(builder, int32PtrLT, getWrcEntriesArrayPtr(builder), "wrcEntriesArrayPtr");
  auto ptrToWrcLE =
      LLVMBuildInBoundsGEP2(builder, int32LT, wrcEntriesPtrLE, &wrciLE, 1, "ptrToWrc");
  return ptrToWrcLE;
}

WrcWeaks::WrcWeaks(GlobalState *globalState_, KindStructs* kindStructsSource_, KindStructs* weakRefStructsSource_)
  : globalState(globalState_),
    fatWeaks_(globalState_, weakRefStructsSource_),
    kindStructsSource(kindStructsSource_),
    weakRefStructsSource(weakRefStructsSource_) {
//  auto voidLT = LLVMVoidTypeInContext(globalState->context);
  auto int1LT = LLVMInt1TypeInContext(globalState->context);
  auto int8LT = LLVMInt8TypeInContext(globalState->context);
  auto voidPtrLT = LLVMPointerType(int8LT, 0);
  auto int32LT = LLVMInt32TypeInContext(globalState->context);
  auto int32PtrLT = LLVMPointerType(int32LT, 0);
  auto int64LT = LLVMInt64TypeInContext(globalState->context);
  auto int8PtrLT = LLVMPointerType(int8LT, 0);

  wrcTablePtrLE = LLVMAddGlobal(globalState->mod, globalState->wrcTableStructLT, "__wrc_table");
  LLVMSetLinkage(wrcTablePtrLE, LLVMExternalLinkage);
  std::vector<LLVMValueRef> wrcTableMembers = {
      constI32LE(globalState, 0),
      constI32LE(globalState, 0),
      LLVMConstNull(int32PtrLT)
  };
  LLVMSetInitializer(
      wrcTablePtrLE,
      LLVMConstNamedStruct(
          globalState->wrcTableStructLT, wrcTableMembers.data(), wrcTableMembers.size()));
}

void WrcWeaks::mainSetup(FunctionState* functionState, LLVMBuilderRef builder) {

}

void WrcWeaks::mainCleanup(FunctionState* functionState, LLVMBuilderRef builder) {
  if (globalState->opt->census) {
    std::vector<LLVMValueRef> args = {
        LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 0, false),
        LLVMBuildZExt(
            builder,
            globalState->getNumWrcs.call(
                builder, {wrcTablePtrLE}, "numWrcs"),
            LLVMInt64TypeInContext(globalState->context),
            ""),
        globalState->getOrMakeStringConstant("WRC leaks!"),
    };
    globalState->externs->assertI64Eq.call(builder, args, "");
  }
}

LLVMValueRef WrcWeaks::getWrcCapacityPtr(LLVMBuilderRef builder) {
  return LLVMBuildStructGEP2(builder, globalState->wrcTableStructLT, wrcTablePtrLE, 0, "wrcCapacityPtr");
}
LLVMValueRef WrcWeaks::getWrcFirstFreeWrciPtr(LLVMBuilderRef builder) {
  return LLVMBuildStructGEP2(builder, globalState->wrcTableStructLT, wrcTablePtrLE, 1, "wrcFirstFree");
}
LLVMValueRef WrcWeaks::getWrcEntriesArrayPtr(LLVMBuilderRef builder) {
  return LLVMBuildStructGEP2(builder, globalState->wrcTableStructLT, wrcTablePtrLE, 2, "entries");
}

WeakFatPtrLE WrcWeaks::weakStructPtrToWrciWeakInterfacePtr(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    WeakFatPtrLE sourceWeakStructFatPtrLE,
    StructKind* sourceStructKindM,
    Kind* sourceStructTypeM,
    InterfaceKind* targetInterfaceKindM,
    Kind* targetInterfaceTypeM) {

//  checkValidReference(
//      FL(), globalState, functionState, builder, sourceStructTypeM, sourceRefLE);
  auto controlBlockPtr =
      kindStructsSource->getConcreteControlBlockPtr(
          FL(),
          functionState,
          builder,
          kindStructsSource->makeWrapperPtr(
              FL(), functionState, builder, sourceStructTypeM,
              fatWeaks_.getInnerRefFromWeakRef(
                  functionState, builder, sourceStructTypeM, sourceWeakStructFatPtrLE)));

  auto interfaceRefLT =
      weakRefStructsSource->getInterfaceWeakRefStruct(targetInterfaceKindM);
  auto wrciLE = getWrciFromWeakRef(builder, sourceWeakStructFatPtrLE);
  auto headerLE = makeWrciHeader(builder, weakRefStructsSource, targetInterfaceKindM, wrciLE);

  auto innerRefLE =
      makeInterfaceRefStruct(
          globalState, functionState, builder, kindStructsSource, sourceStructKindM,
          targetInterfaceKindM,
          controlBlockPtr);

  return fatWeaks_.assembleWeakFatPtr(
      functionState, builder, targetInterfaceTypeM, interfaceRefLT, headerLE, innerRefLE);
}

// Makes a non-weak interface ref into a weak interface ref
WeakFatPtrLE WrcWeaks::assembleInterfaceWeakRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceType,
    Kind* targetType,
    InterfaceKind* interfaceKindM,
    InterfaceFatPtrLE sourceInterfaceFatPtrLE) {
//  if (globalState->opt->regionOverride == RegionOverride::RESILIENT_V0) {
//    if (sourceType->ownership == Ownership::BORROW) {
//      { assert(false); throw 1337; } // curiosity, wouldnt we just return sourceRefLE?
//    }
//    assert(sourceType->ownership == Ownership::SHARE || sourceType->ownership == Ownership::OWN);
//  }

  auto controlBlockPtrLE =
      kindStructsSource->getControlBlockPtr(
          FL(), functionState, builder, sourceInterfaceFatPtrLE);
  auto wrciLE = getWrciFromControlBlockPtr(globalState, builder, kindStructsSource, sourceType,
      controlBlockPtrLE);
  auto headerLE = LLVMGetUndef(weakRefStructsSource->getWeakRefHeaderStruct());
  headerLE = LLVMBuildInsertValue(builder, headerLE, wrciLE, WEAK_REF_HEADER_MEMBER_INDEX_FOR_WRCI, "header");

  auto weakRefStructLT =
      weakRefStructsSource->getInterfaceWeakRefStruct(interfaceKindM);

  return fatWeaks_.assembleWeakFatPtr(
      functionState, builder, targetType, weakRefStructLT, headerLE, sourceInterfaceFatPtrLE.refLE);
}

WeakFatPtrLE WrcWeaks::assembleStructWeakRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* structTypeM,
    Kind* targetTypeM,
    StructKind* structKindM,
    WrapperPtrLE objPtrLE) {
  assert(
      isValueType(structTypeM) ||
      dynamic_cast<ShareRef*>(structTypeM) != nullptr ||
      dynamic_cast<BorrowRef*>(structTypeM) != nullptr);

  auto controlBlockPtrLE = kindStructsSource->getConcreteControlBlockPtr(FL(), functionState, builder, objPtrLE);
  auto wrciLE = getWrciFromControlBlockPtr(globalState, builder, kindStructsSource, structTypeM, controlBlockPtrLE);
  auto headerLE = makeWrciHeader(builder, weakRefStructsSource, structKindM, wrciLE);

  auto weakRefStructLT =
      weakRefStructsSource->getStructWeakRefStruct(structKindM);

  return fatWeaks_.assembleWeakFatPtr(
      functionState, builder, targetTypeM, weakRefStructLT, headerLE, objPtrLE.refLE);
}

WeakFatPtrLE WrcWeaks::assembleStaticSizedArrayWeakRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceSSAMT,
    StaticSizedArrayT* staticSizedArrayMT,
    Kind* targetSSAWeakRefMT,
    WrapperPtrLE objPtrLE) {
  auto controlBlockPtrLE = kindStructsSource->getConcreteControlBlockPtr(FL(), functionState, builder, objPtrLE);
  auto wrciLE = getWrciFromControlBlockPtr(globalState, builder, kindStructsSource, sourceSSAMT, controlBlockPtrLE);
  auto headerLE = makeWrciHeader(builder, weakRefStructsSource, staticSizedArrayMT, wrciLE);

  auto weakRefStructLT =
      weakRefStructsSource->getStaticSizedArrayWeakRefStruct(staticSizedArrayMT);

  return fatWeaks_.assembleWeakFatPtr(
      functionState, builder, targetSSAWeakRefMT, weakRefStructLT, headerLE, objPtrLE.refLE);
}

WeakFatPtrLE WrcWeaks::assembleRuntimeSizedArrayWeakRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceType,
    RuntimeSizedArrayT* runtimeSizedArrayMT,
    Kind* targetRSAWeakRefMT,
    WrapperPtrLE sourceRefLE) {
  auto controlBlockPtrLE = kindStructsSource->getConcreteControlBlockPtr(FL(), functionState, builder, sourceRefLE);
  auto wrciLE = getWrciFromControlBlockPtr(globalState, builder, kindStructsSource, sourceType, controlBlockPtrLE);
  auto headerLE = makeWrciHeader(builder, weakRefStructsSource, runtimeSizedArrayMT, wrciLE);

  auto weakRefStructLT =
      weakRefStructsSource->getRuntimeSizedArrayWeakRefStruct(runtimeSizedArrayMT);

  return fatWeaks_.assembleWeakFatPtr(
      functionState, builder, targetRSAWeakRefMT, weakRefStructLT, headerLE, sourceRefLE.refLE);
}

LLVMValueRef WrcWeaks::lockWrciFatPtr(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refM,
    WeakFatPtrLE weakFatPtrLE) {
  auto isAliveLE = getIsAliveFromWeakFatPtr(functionState, builder, weakFatPtrLE);
  buildIfV(
      globalState, functionState, builder, isZeroLE(builder, isAliveLE),
      [this, from, functionState, weakFatPtrLE](LLVMBuilderRef thenBuilder) {
        buildPrintAreaAndFileAndLineToStderr(globalState, thenBuilder, from);
        buildPrintToStderr(globalState, thenBuilder, "Tried dereferencing dangling reference! ");
//        assert(globalState->opt->regionOverride != RegionOverride::RESILIENT_V1);
        auto wrciLE = getWrciFromWeakRef(thenBuilder, weakFatPtrLE);
        buildPrintToStderr(globalState, thenBuilder, "Wrci: ");
        buildPrintToStderr(globalState, thenBuilder, wrciLE);
        buildPrintToStderr(globalState, thenBuilder, " ");
        buildPrintToStderr(globalState, thenBuilder, "Exiting!\n");
        // See MPESC for status codes
        auto exitCodeIntLE = LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 14, false);
        buildCallWith64BitSExt(globalState, thenBuilder, globalState->externs->exit, {exitCodeIntLE});
      });
  return fatWeaks_.getInnerRefFromWeakRef(functionState, builder, refM, weakFatPtrLE);
}

LLVMValueRef WrcWeaks::getNewWrci(
    FunctionState* functionState,
    LLVMBuilderRef builder) {
  auto int32LT = LLVMInt32TypeInContext(globalState->context);

  // uint64_t resultWrci = __wrc_firstFree;
  auto resultWrciLE = LLVMBuildLoad2(builder, int32LT, getWrcFirstFreeWrciPtr(builder), "resultWrci");

  // if (resultWrci == __wrc_capacity) {
  //   __expandWrcTable();
  // }
  auto atCapacityLE =
      LLVMBuildICmp(
          builder,
          LLVMIntEQ,
          resultWrciLE,
          LLVMBuildLoad2(builder, int32LT, getWrcCapacityPtr(builder), "wrcCapacity"),
          "atCapacity");
  buildIfV(
      globalState, functionState,
      builder,
      atCapacityLE,
      [this](LLVMBuilderRef thenBuilder) {
        globalState->expandWrcTable.call(thenBuilder, {wrcTablePtrLE}, "");
      });

  // u64* wrcPtr = &__wrc_entries[resultWrci];
  auto wrcPtrLE = getWrcPtr(builder, resultWrciLE);

  // __wrc_firstFree = *wrcPtr;
  LLVMBuildStore(
      builder,
      // *wrcPtr
      LLVMBuildLoad2(builder, int32LT, wrcPtrLE, ""),
      // __wrc_firstFree
      getWrcFirstFreeWrciPtr(builder));

  // *wrcPtr = WRC_INITIAL_VALUE;
  LLVMBuildStore(
      builder,
      LLVMConstInt(LLVMInt32TypeInContext(globalState->context), WRC_INITIAL_VALUE, false),
      wrcPtrLE);

  return resultWrciLE;
}

void WrcWeaks::innerNoteWeakableDestroyed(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* concreteRefM,
    ControlBlockPtrLE controlBlockPtrLE) {
  auto int32LT = LLVMInt32TypeInContext(globalState->context);
  auto wrciLE =
      getWrciFromControlBlockPtr(
          globalState, builder, kindStructsSource, concreteRefM, controlBlockPtrLE);

  //  unmigratedLLVMBuildCall(builder, globalState->noteWeakableDestroyed, &wrciLE, 1, "");

  auto ptrToWrcLE = getWrcPtr(builder, wrciLE);
  auto prevWrcLE = LLVMBuildLoad2(builder, int32LT, ptrToWrcLE, "wrc");

  auto wrcLE =
      LLVMBuildAnd(
          builder,
          prevWrcLE,
          LLVMConstInt(LLVMInt32TypeInContext(globalState->context), ~WRC_ALIVE_BIT, true),
          "");

  // Equivalent:
  // __wrc_entries[wrcIndex] = __wrc_entries[wrcIndex] & ~WRC_LIVE_BIT;
  // *wrcPtr = *wrcPtr & ~WRC_LIVE_BIT;
  LLVMBuildStore(builder, wrcLE, ptrToWrcLE);

  buildFlare(FL(), globalState, functionState, builder, "maybeReleasing wrci ", wrciLE, " is now ", wrcLE);

  maybeReleaseWrc(functionState, builder, wrciLE, ptrToWrcLE, wrcLE);
}


void WrcWeaks::aliasWeakRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* weakRefMT,
    Ref weakRef) {
  auto weakFatPtrLE =
      weakRefStructsSource->makeWeakFatPtr(
          weakRefMT,
          globalState->getRegion(peel_all_references(weakRefMT))
              ->checkValidReference(FL(), functionState, builder, false, weakRefMT, weakRef));
  auto wrciLE = getWrciFromWeakRef(builder, weakFatPtrLE);
  if (globalState->opt->census) {
    buildCheckWrc(builder, wrciLE);
  }

  auto ptrToWrcLE = getWrcPtr(builder, wrciLE);
  adjustCounterV(globalState, builder, globalState->metalCache->i32Type, ptrToWrcLE, 1, false);
}

void WrcWeaks::discardWeakRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* weakRefMT,
    Ref weakRef) {
  auto weakFatPtrLE =
      weakRefStructsSource->makeWeakFatPtr(
          weakRefMT,
          globalState->getRegion(peel_all_references(weakRefMT))
              ->checkValidReference(FL(), functionState, builder, false, weakRefMT, weakRef));
  auto wrciLE = getWrciFromWeakRef(builder, weakFatPtrLE);
  if (globalState->opt->census) {
    buildCheckWrc(builder, wrciLE);
  }

  auto ptrToWrcLE = getWrcPtr(builder, wrciLE);
  auto wrcLE = adjustCounterV(globalState, builder, globalState->metalCache->i32Type, ptrToWrcLE, -1, false);

  buildFlare(FL(), globalState, functionState, builder, "decrementing ", wrciLE, " to ", wrcLE);

  maybeReleaseWrc(functionState, builder, wrciLE, ptrToWrcLE, wrcLE);
}


LLVMValueRef WrcWeaks::getIsAliveFromWeakFatPtr(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    WeakFatPtrLE weakFatPtrLE) {
  auto int32LT = LLVMInt32TypeInContext(globalState->context);
  auto wrciLE = getWrciFromWeakRef(builder, weakFatPtrLE);
  if (globalState->opt->census) {
    buildCheckWrc(builder, wrciLE);
  }

  auto ptrToWrcLE = getWrcPtr(builder, wrciLE);
  auto wrcLE = LLVMBuildLoad2(builder, int32LT, ptrToWrcLE, "wrc");
  return LLVMBuildICmp(
      builder,
      LLVMIntNE,
      LLVMBuildAnd(
          builder,
          wrcLE,
          LLVMConstInt(LLVMInt32TypeInContext(globalState->context), WRC_ALIVE_BIT, false),
          "wrcLiveBitOrZero"),
      constI32LE(globalState, 0),
      "wrcLive");
}

Ref WrcWeaks::getIsAliveFromWeakRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* weakRefM,
    Ref weakRef) {
  assert(dynamic_cast<WeakRef*>(weakRefM) != nullptr);

  auto weakFatPtrLE =
      weakRefStructsSource->makeWeakFatPtr(
          weakRefM,
          globalState->getRegion(peel_all_references(weakRefM))
              ->checkValidReference(FL(), functionState, builder, false, weakRefM, weakRef));
  auto isAliveLE = getIsAliveFromWeakFatPtr(functionState, builder, weakFatPtrLE);
  return toRef(globalState->getRegion(globalState->metalCache->boolType), globalState->metalCache->boolType, isAliveLE);
}

LLVMValueRef WrcWeaks::fillWeakableControlBlock(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    ValueKind* kindM,
    LLVMValueRef controlBlockLE) {
  auto wrciLE = getNewWrci(functionState, builder);
  return LLVMBuildInsertValue(
      builder,
      controlBlockLE,
      wrciLE,
      structs->getControlBlock(kindM)->getMemberIndex(ControlBlockMember::WRCI_32B),
      "weakableControlBlockWithWrci");
}

WeakFatPtrLE WrcWeaks::weakInterfaceRefToWeakStructRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* weakInterfaceRefMT,
    WeakFatPtrLE weakInterfaceFatPtrLE) {
  { assert(false); throw 1337; }

  // // Disassemble the weak interface ref.
  // auto wrciLE = getWrciFromWeakRef(builder, weakInterfaceFatPtrLE);
  // // The object might not exist, so skip the check.
  // auto interfaceRefLE =
  //     kindStructsSource->makeInterfaceFatPtrWithoutChecking(FL(), functionState, builder,
  //         weakInterfaceRefMT, // It's still conceptually weak even though its not in a weak pointer.
  //         fatWeaks_.getInnerRefFromWeakRef(
  //             functionState,
  //             builder,
  //             weakInterfaceRefMT,
  //             weakInterfaceFatPtrLE));
  // auto controlBlockPtrLE =
  //     kindStructsSource->getControlBlockPtrWithoutChecking(
  //         FL(), functionState, builder, weakInterfaceRefMT->kind, interfaceRefLE);
  //
  // auto headerLE = makeWrciHeader(builder, weakRefStructsSource, weakInterfaceRefMT->kind, wrciLE);
  //
  // // Now, reassemble a weak void* ref to the struct.
  // auto weakVoidStructRefLE =
  //     fatWeaks_.assembleVoidStructWeakRef(
  //         builder,
  //         // We still think of it as an interface pointer, even though its a void*.
  //         // That kind of makes this makes sense.
  //         // We could think of this as making an "Any" pointer perhaps?
  //         weakInterfaceRefMT,
  //         controlBlockPtrLE,
  //         headerLE);
  //
  // return weakVoidStructRefLE;
}

// USE ONLY FOR ASSERTING A REFERENCE IS VALID
std::tuple<Kind*, LLVMValueRef> wrcGetRefInnardsForChecking(Ref ref) {
  Kind* refM = ref.refM;
  LLVMValueRef refLE = ref.refLE;
  return std::make_tuple(refM, refLE);
}

void WrcWeaks::buildCheckWeakRef(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* weakRefM,
    Ref weakRef) {
  Kind* actualRefM = nullptr;
  LLVMValueRef refLE = nullptr;
  std::tie(actualRefM, refLE) = wrcGetRefInnardsForChecking(weakRef);
  auto weakFatPtrLE = weakRefStructsSource->makeWeakFatPtr(weakRefM, refLE);
  auto innerLE =
      fatWeaks_.getInnerRefFromWeakRefWithoutCheck(
          functionState, builder, weakRefM, weakFatPtrLE);

  // WARNING: This check has false positives.
  auto wrciLE = getWrciFromWeakRef(builder, weakFatPtrLE);
  buildCheckWrc(builder, wrciLE);

  // This will also run for objects which have since died, which is fine.
  if (auto interfaceKindM = dynamic_cast<InterfaceKind*>(peel_all_references(weakRefM))) {
    auto interfaceFatPtrLE = kindStructsSource->makeInterfaceFatPtrWithoutChecking(checkerAFL, functionState, builder, weakRefM, innerLE);
    auto itablePtrLE = getTablePtrFromInterfaceRef(builder, interfaceFatPtrLE);
    buildAssertCensusContains(checkerAFL, globalState, functionState, builder, itablePtrLE);
  }
}

Ref WrcWeaks::assembleWeakRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceType,
    Kind* targetType,
    Ref sourceRef) {
  // Now we need to package it up into a weak ref.
  if (auto structKind = dynamic_cast<StructKind*>(peel_all_references(sourceType))) {
    // I *think* we expect it to be live at this point.
    auto sourceRefLE =
        globalState->getRegion(peel_all_references(sourceType))
            ->checkValidReference(FL(), functionState, builder, false, sourceType, sourceRef);
    auto sourceWrapperPtrLE = kindStructsSource->makeWrapperPtr(FL(), functionState, builder, sourceType, sourceRefLE);
    auto resultLE =
        assembleStructWeakRef(
            functionState, builder, sourceType, targetType, structKind, sourceWrapperPtrLE);
    return toRef(globalState->getRegion(peel_all_references(targetType)), targetType, resultLE);
  } else if (auto interfaceKindM = dynamic_cast<InterfaceKind*>(peel_all_references(sourceType))) {
    auto sourceRefLE =
        globalState->getRegion(peel_all_references(sourceType))
            ->checkValidReference(FL(), functionState, builder, false, sourceType, sourceRef);
    auto sourceInterfaceFatPtrLE = kindStructsSource->makeInterfaceFatPtr(FL(), functionState, builder, sourceType, sourceRefLE);
    auto resultLE =
        assembleInterfaceWeakRef(
            functionState, builder, sourceType, targetType, interfaceKindM, sourceInterfaceFatPtrLE);
    return toRef(globalState->getRegion(peel_all_references(targetType)), targetType, resultLE);
  } else if (auto staticSizedArray = dynamic_cast<StaticSizedArrayT*>(peel_all_references(sourceType))) {
    auto sourceRefLE =
        globalState->getRegion(peel_all_references(sourceType))
            ->checkValidReference(FL(), functionState, builder, false, sourceType, sourceRef);
    auto sourceWrapperPtrLE = kindStructsSource->makeWrapperPtr(FL(), functionState, builder, sourceType, sourceRefLE);
    auto resultLE =
        assembleStaticSizedArrayWeakRef(
            functionState, builder, sourceType, staticSizedArray, targetType, sourceWrapperPtrLE);
    return toRef(globalState->getRegion(peel_all_references(targetType)), targetType, resultLE);
  } else if (auto runtimeSizedArray = dynamic_cast<RuntimeSizedArrayT*>(peel_all_references(sourceType))) {
    auto sourceRefLE =
        globalState->getRegion(peel_all_references(sourceType))
            ->checkValidReference(FL(), functionState, builder, false, sourceType, sourceRef);
    auto sourceWrapperPtrLE = kindStructsSource->makeWrapperPtr(FL(), functionState, builder, sourceType, sourceRefLE);
    auto resultLE =
        assembleRuntimeSizedArrayWeakRef(
            functionState, builder, sourceType, runtimeSizedArray, targetType, sourceWrapperPtrLE);
    return toRef(globalState->getRegion(peel_all_references(targetType)), targetType, resultLE);
  } else { assert(false); throw 1337; }
}

LLVMTypeRef WrcWeaks::makeWeakRefHeaderStruct(GlobalState* globalState) {
  auto wrciRefStructL = LLVMStructCreateNamed(globalState->context, "__WrciRef");

  std::vector<LLVMTypeRef> memberTypesL;

  assert(WEAK_REF_HEADER_MEMBER_INDEX_FOR_WRCI == memberTypesL.size());
  memberTypesL.push_back(LLVMInt32TypeInContext(globalState->context));

  LLVMStructSetBody(wrciRefStructL, memberTypesL.data(), memberTypesL.size(), false);

  return wrciRefStructL;
}
