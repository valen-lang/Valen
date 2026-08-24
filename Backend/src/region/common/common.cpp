#include <llvm-c/Types.h>
#include "../../globalstate.h"
#include "../../function/function.h"
#include "../../function/expressions/shared/shared.h"
#include "controlblock.h"
#include "../../function/expressions/shared/members.h"
#include "../../utils/counters.h"
#include "../../function/expressions/shared/elements.h"
#include "../../utils/branch.h"
#include "../../function/expressions/shared/string.h"
#include "common.h"
#include "primitives.h"
#include <region/common/migration.h>

constexpr int INTERFACE_REF_MEMBER_INDEX_FOR_OBJ_PTR = 0;
constexpr int INTERFACE_REF_MEMBER_INDEX_FOR_ITABLE_PTR = 1;

LLVMValueRef upcastThinPtr(
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* kindStructsSource,
    LLVMBuilderRef builder,

    Kind* sourceStructTypeM,
    StructKind* sourceStructKindM,
    WrapperPtrLE sourceRefLE,

    Kind* targetInterfaceTypeM,
    InterfaceKind* targetInterfaceKindM) {
  assert(
      dynamic_cast<ShareRef*>(sourceStructTypeM) != nullptr &&
      isValueType(sourceStructTypeM) &&
      dynamic_cast<BorrowRef*>(sourceStructTypeM) != nullptr);
  ControlBlockPtrLE controlBlockPtrLE =
      kindStructsSource->getConcreteControlBlockPtr(
          FL(), functionState, builder, sourceRefLE);
  auto interfaceRefLE =
      makeInterfaceRefStruct(
          globalState, functionState, builder, kindStructsSource, sourceStructKindM, targetInterfaceKindM,
          controlBlockPtrLE);
  return interfaceRefLE;
}

LLVMTypeRef translateWeakReference(GlobalState* globalState, KindStructs* weakRefStructs, ValueKind* kind) {
  if (auto ssaMT = dynamic_cast<StaticSizedArrayT *>(kind)) {
    return weakRefStructs->getStaticSizedArrayWeakRefStruct(ssaMT);
  } else if (auto rsaMT = dynamic_cast<RuntimeSizedArrayT *>(kind)) {
    return weakRefStructs->getRuntimeSizedArrayWeakRefStruct(rsaMT);
  } else if (auto structKind = dynamic_cast<StructKind *>(kind)) {
    return weakRefStructs->getStructWeakRefStruct(structKind);
  } else if (auto interfaceKind = dynamic_cast<InterfaceKind *>(kind)) {
    return weakRefStructs->getInterfaceWeakRefStruct(interfaceKind);
  } else {
    { assert(false); throw 1337; }
  }
}

LoadResult loadInnerInnerStructMember(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMTypeRef innerStructLT,
    LLVMValueRef innerStructPtrLE,
    int memberIndex,
    Kind* expectedType,
    std::string memberName) {
  assert(LLVMGetTypeKind(LLVMTypeOf(innerStructPtrLE)) == LLVMPointerTypeKind);

  auto ptrToMemberLE =
      LLVMBuildStructGEP2(builder, innerStructLT, innerStructPtrLE, memberIndex, memberName.c_str());

  auto memberRegion = globalState->getRegion(expectedType);
  auto memberLT = memberRegion->translateType(expectedType);
  auto resultLE = LLVMBuildLoad2(builder, memberLT, ptrToMemberLE, memberName.c_str());
  return LoadResult{toRef(memberRegion, expectedType, resultLE)};
}

void storeInnerInnerStructMember(
    LLVMBuilderRef builder,
    LLVMTypeRef innerStructLT,
    LLVMValueRef innerStructPtrLE,
    int memberIndex,
    std::string memberName,
    LLVMValueRef newValueLE) {
  assert(LLVMGetTypeKind(LLVMTypeOf(innerStructPtrLE)) == LLVMPointerTypeKind);
  LLVMBuildStore(
      builder,
      newValueLE,
      LLVMBuildStructGEP2(
          builder, innerStructLT, innerStructPtrLE, memberIndex, memberName.c_str()));
}

LLVMValueRef getItablePtrFromInterfacePtr(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* virtualParamMT,
    InterfaceFatPtrLE virtualArgLE) {
  buildFlare(FL(), globalState, functionState, builder);
  assert(LLVMTypeOf(virtualArgLE.refLE) == globalState->getRegion(virtualParamMT)->translateType(virtualParamMT));
  return getTablePtrFromInterfaceRef(builder, virtualArgLE);
}


LLVMValueRef fillControlBlockCensusFields(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* structs,
    LLVMBuilderRef builder,
    ValueKind* kindM,
    LLVMValueRef newControlBlockLE,
    const std::string& typeName) {
  if (globalState->opt->census) {
    auto objIdLE = adjustCounterV(
        globalState, builder, globalState->metalCache->i64Type, globalState->objIdCounterLE, 1, false);
    newControlBlockLE =
        LLVMBuildInsertValue(
            builder,
            newControlBlockLE,
            objIdLE,
            structs->getControlBlock(kindM)->getMemberIndex(ControlBlockMember::CENSUS_OBJ_ID),
            "strControlBlockWithObjId");
    newControlBlockLE =
        LLVMBuildInsertValue(
            builder,
            newControlBlockLE,
            globalState->getOrMakeStringConstant(typeName),
            structs->getControlBlock(kindM)->getMemberIndex(ControlBlockMember::CENSUS_TYPE_STR),
            "strControlBlockWithTypeStr");
    buildFlare(from, globalState, functionState, builder, "Allocating ", typeName, " ", objIdLE);
  }
  return newControlBlockLE;
}

LLVMValueRef insertStrongRc(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    ValueKind* kindM,
    LLVMValueRef newControlBlockLE) {
  return LLVMBuildInsertValue(
      builder,
      newControlBlockLE,
      // Start RC at 1, see SRCAZ.
      LLVMConstInt(LLVMInt32TypeInContext(globalState->context), 1, false),
      structs->getControlBlock(kindM)->getMemberIndex(ControlBlockMember::STRONG_RC_32B),
      "controlBlockWithRc");
}

// Not returning Ref because we might need to wrap it in something else like a weak fat ptr
LLVMValueRef makeInterfaceRefStruct(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    StructKind* sourceStructKindM,
    InterfaceKind* targetInterfaceKindM,
    ControlBlockPtrLE controlBlockPtrLE) {
  auto itablePtrLE =
      globalState->getInterfaceTablePtr(
          globalState->program->getStruct(sourceStructKindM)
              ->getEdgeForInterface(targetInterfaceKindM));
  return makeInterfaceRefStruct(
      globalState, functionState, builder, structs, targetInterfaceKindM, controlBlockPtrLE.refLE, itablePtrLE);
}

LLVMValueRef makeInterfaceRefStruct(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    InterfaceKind* targetInterfaceKindM,
    LLVMValueRef objControlBlockPtrLE,
    LLVMValueRef itablePtrLE) {

  auto interfaceRefLT = structs->getInterfaceRefStruct(targetInterfaceKindM);

  auto interfaceRefLE = LLVMGetUndef(interfaceRefLT);
  interfaceRefLE =
      LLVMBuildInsertValue(
          builder,
          interfaceRefLE,
          objControlBlockPtrLE,
          INTERFACE_REF_MEMBER_INDEX_FOR_OBJ_PTR,
          "interfaceRefWithOnlyObj");
  interfaceRefLE =
      LLVMBuildInsertValue(
          builder,
          interfaceRefLE,
          itablePtrLE,
          INTERFACE_REF_MEMBER_INDEX_FOR_ITABLE_PTR,
          "interfaceRef");

//  buildFlare(FL(), globalState, functionState, builder, "Imploding, objPtrLE: ", ptrToIntLE(globalState, builder, objControlBlockPtrLE), " itablePtrLE ", ptrToIntLE(globalState, builder, itablePtrLE));

  return interfaceRefLE;
}


LLVMValueRef getObjPtrFromInterfaceRef(
    LLVMBuilderRef builder,
    InterfaceFatPtrLE interfaceRefLE) {
  return LLVMBuildExtractValue(builder, interfaceRefLE.refLE, INTERFACE_REF_MEMBER_INDEX_FOR_OBJ_PTR, "objPtr");
}

LLVMValueRef getTablePtrFromInterfaceRef(
    LLVMBuilderRef builder,
    InterfaceFatPtrLE interfaceRefLE) {
  return LLVMBuildExtractValue(builder, interfaceRefLE.refLE, INTERFACE_REF_MEMBER_INDEX_FOR_ITABLE_PTR, "itablePtr");
}

void callFree(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef ptrLE) {
  auto concreteAsCharPtrLE =
      LLVMBuildBitCast(
          builder,
          ptrLE,
          LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0),
          "concreteCharPtrForFree");
  buildFlare(FL(), globalState, functionState, builder, "Freeing ", ptrToIntLE(globalState, builder, concreteAsCharPtrLE));
  buildCallWith64BitSExt(globalState, builder, globalState->externs->free, {concreteAsCharPtrLE});
}

void innerDeallocateYonder(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* kindStructsSource,
    LLVMBuilderRef builder,
    Kind* refMT,
    LiveRef liveRef) {
  buildFlare(FL(), globalState, functionState, builder);

  auto ref = toRef(globalState, refMT, liveRef);
  auto refValueType = peel_all_references(refMT);
  if (globalState->opt->census) {
    auto ptrLE =
        globalState->getRegion(refValueType)
            ->checkValidReference(FL(), functionState, builder, true, refMT, ref);
    auto objIdLE =
        globalState->getRegion(refValueType)
            ->getCensusObjectId(FL(), functionState, builder, refMT, ref);
    if (dynamic_cast<InterfaceKind*>(refValueType) == nullptr) {
      buildFlare(FL(), globalState, functionState, builder,
          "Deallocating object &", ptrToIntLE(globalState, builder, ptrLE), " obj id ", objIdLE, "\n");
    }
  }

  auto controlBlockPtrLE =
      kindStructsSource->getControlBlockPtr(from, functionState, builder, ref, refMT);

//  globalState->getRegion(refMT)
//      ->noteWeakableDestroyed(functionState, builder, refMT, controlBlockPtrLE);

  if (globalState->opt->census) {
    LLVMValueRef resultAsVoidPtrLE =
        LLVMBuildBitCast(
            builder, controlBlockPtrLE.refLE, LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0), "");
    globalState->externs->censusRemove.call(builder, {resultAsVoidPtrLE}, "");
  }

  callFree(globalState, functionState, builder, controlBlockPtrLE.refLE);

  if (globalState->opt->census) {
    adjustCounterV(
        globalState, builder, globalState->metalCache->i64Type, globalState->liveHeapObjCounterLE, -1, false);
  }
}

void innerDeallocate(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* kindStrutsSource,
    LLVMBuilderRef builder,
    Kind* refMT,
    LiveRef ref) {
  buildFlare(FL(), globalState, functionState, builder);
  // VCOORD: clean this
  // Inline (SINGLE) placement only applies to single-owner structs; everything else the free
  // path reaches here — shared structs, strings, arrays — is a heap object, deallocated yonder.
  if (auto structKindM = dynamic_cast<StructKind *>(peel_all_references(refMT))) {
    if (globalState->program->getStruct(structKindM)->sharedness == Sharedness::SINGLE) {
      // Do nothing, it's inline!
      return;
    }
  }
  innerDeallocateYonder(from, globalState, functionState, kindStrutsSource, builder, refMT, ref);
}

void fillStaticSizedArray(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* ssaRefMT,
    StaticSizedArrayT* ssaMT,
    LiveRef ssaRef,
    const std::vector<Ref>& elementRefs) {

  for (int i = 0; i < elementRefs.size(); i++) {
    // Making an InBoundsLE because the bound of the containing loop is the size of the array.
    auto indexInBoundsLE = InBoundsLE{constI64LE(globalState, i)};
    globalState->getRegion(ssaRefMT)->initializeElementInSSA(
        functionState, builder, ssaRefMT, ssaMT, ssaRef, indexInBoundsLE, elementRefs[i]);
  }
}

void fillRuntimeSizedArray(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* rsaRefMT,
    RuntimeSizedArrayT* rsaMT,
    Kind* elementType,
    Kind* generatorType,
    Prototype* generatorMethod,
    Ref generatorLE,
    Ref sizeLE,
    LiveRef rsaRef) {
  intRangeLoopV(
      globalState, functionState, builder, sizeLE,
      [globalState, functionState, rsaRefMT, rsaMT, generatorMethod, generatorType, rsaRef, generatorLE](
          Ref indexRef, LLVMBuilderRef bodyBuilder) {
        globalState->getRegion(generatorType)->alias(
            AFL("ConstructRSA generate iteration"),
            functionState, bodyBuilder, generatorType, generatorLE);
        std::vector<Ref> argExprsLE = {generatorLE, indexRef};

        auto indexLE =
            globalState->getRegion(globalState->metalCache->i32Type)
                ->checkValidReference(FL(), functionState, bodyBuilder, false, globalState->metalCache->i32Type, indexRef);
        // Manually making InBoundsLE because the array's size is the bound of the containing loop.
        auto indexInBoundsLE = InBoundsLE{indexLE};

        auto elementRef =
            buildCallV(
                globalState, functionState, bodyBuilder, generatorMethod, argExprsLE);
        globalState->getRegion(rsaMT)->pushRuntimeSizedArrayNoBoundsCheck(
            functionState, bodyBuilder, rsaRefMT, rsaMT, rsaRef, indexInBoundsLE, elementRef);
      });
}

void fillStaticSizedArrayFromCallable(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* ssaRefMT,
    StaticSizedArrayT* ssaMT,
    Kind* elementType,
    Kind* generatorType,
    Prototype* generatorMethod,
    Ref generatorLE,
    Ref sizeLE,
    LiveRef ssaRef) {

  intRangeLoopV(
      globalState, functionState, builder, sizeLE,
      [globalState, functionState, ssaRefMT, ssaMT, generatorMethod, generatorType, ssaRef, generatorLE](
          Ref indexRef, LLVMBuilderRef bodyBuilder) {
        globalState->getRegion(generatorType)->alias(
            AFL("ConstructSSA generate iteration"),
            functionState, bodyBuilder, generatorType, generatorLE);
        std::vector<Ref> argExprsLE = {generatorLE, indexRef};

        auto indexLE =
            globalState->getRegion(globalState->metalCache->i32Type)
                ->checkValidReference(FL(), functionState, bodyBuilder, false, globalState->metalCache->i32Type, indexRef);
        // Manually making InBoundsLE because the array's size is the bound of the containing loop.
        auto indexInBoundsLE = InBoundsLE{indexLE};

        auto elementRef =
            buildCallV(
                globalState, functionState, bodyBuilder, generatorMethod, argExprsLE);
        globalState->getRegion(ssaMT)->initializeElementInSSA(
            functionState, bodyBuilder, ssaRefMT, ssaMT, ssaRef, indexInBoundsLE, elementRef);
      });
}

std::tuple<Kind*, LLVMValueRef> megaGetRefInnardsForChecking(Ref ref) {
  Kind* refM = ref.refM;
  LLVMValueRef refLE = ref.refLE;
  return std::make_tuple(refM, refLE);
}

LLVMValueRef callMalloc(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    LLVMValueRef sizeLE) {
  assert(LLVMTypeOf(sizeLE) == LLVMInt64TypeInContext(globalState->context));
  return buildCallWith64BitSExt(globalState, builder, globalState->externs->malloc, {sizeLE});
}

WrapperPtrLE mallocStr(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef lenI32LE,
    LLVMValueRef sourceCharsPtrLE,
    KindStructs* kindStructs,
    std::function<void(LLVMBuilderRef builder, ControlBlockPtrLE controlBlockPtrLE)> fillControlBlock) {
  auto int8LT = LLVMInt8TypeInContext(globalState->context);
  auto int8PtrLT = LLVMPointerType(int8LT, 0);

  auto lenI64LE = LLVMBuildZExt(builder, lenI32LE, LLVMInt64TypeInContext(globalState->context), "lenAsI64");
  // The +1 is for the null terminator at the end, for C compatibility.
  auto sizeBytesLE =
      LLVMBuildAdd(
          builder,
          lenI64LE,
          LLVMBuildAdd(
              builder,
              constI64LE(globalState, 1),
              constI64LE(globalState, LLVMABISizeOfType(globalState->dataLayout, kindStructs->getStringWrapperStruct())),
              "lenPlus1"),
          "strMallocSizeBytes");

  auto destCharPtrLE =callMalloc(globalState, builder, sizeBytesLE);

  if (globalState->opt->census) {
    adjustCounterV(
        globalState, builder, globalState->metalCache->i64Type, globalState->liveHeapObjCounterLE, 1, false);

    LLVMValueRef resultAsVoidPtrLE =
        LLVMBuildBitCast(
            builder, destCharPtrLE, LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0), "");
    globalState->externs->censusAdd.call(builder, {resultAsVoidPtrLE}, "");
  }

  auto newStrWrapperPtrLE =
      kindStructs->makeWrapperPtr(
          FL(), functionState, builder, globalState->metalCache->str,
          LLVMBuildBitCast(
              builder,
              destCharPtrLE,
              LLVMPointerType(kindStructs->getStringWrapperStruct(), 0),
              "newStrWrapperPtr"));
  assert(LLVMTypeOf(newStrWrapperPtrLE.refLE) == LLVMPointerType(kindStructs->getStringWrapperStruct(), 0));

  fillControlBlock(
      builder,
      kindStructs->getConcreteControlBlockPtr(
          FL(), functionState, builder, newStrWrapperPtrLE));
  assert(LLVMTypeOf(lenI32LE) == LLVMInt32TypeInContext(globalState->context));
  LLVMBuildStore(
      builder,
      lenI32LE,
      kindStructs->getStringLenPtr(functionState, builder, newStrWrapperPtrLE));

  // Set the null terminating character to the 0th spot and the end spot, just to guard against bugs
  auto charsBeginPtr =
      kindStructs->getStringBytesPtr(functionState, builder, newStrWrapperPtrLE);


  std::vector<LLVMValueRef> strncpyArgsLE = { charsBeginPtr, sourceCharsPtrLE, lenI64LE };
  buildCallWith64BitSExt(globalState, builder, globalState->externs->strncpy, strncpyArgsLE);

  auto charsEndPtr = LLVMBuildInBoundsGEP2(builder, int8LT, charsBeginPtr, &lenI32LE, 1, "charsEndPtrZ");
  LLVMBuildStore(builder, constI8LE(globalState, 0), charsEndPtr);

  // The caller still needs to initialize the actual chars inside!

  return newStrWrapperPtrLE;
}
// VCOORD: rename or remove this function
LLVMValueRef mallocKnownSize(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMTypeRef kindLT) {
  if (globalState->opt->census) {
    adjustCounterV(
        globalState, builder, globalState->metalCache->i64Type, globalState->liveHeapObjCounterLE, 1, false);
  }

  LLVMValueRef resultPtrLE = makeBackendLocal(functionState, builder, kindLT, "newstruct", LLVMGetUndef(kindLT));

  if (globalState->opt->census) {
    LLVMValueRef resultAsVoidPtrLE =
        LLVMBuildBitCast(
            builder, resultPtrLE, LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0), "");
    globalState->externs->censusAdd.call(builder, {resultAsVoidPtrLE}, "");
  }
  return resultPtrLE;
}

void fillInnerStruct(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    StructDefinition* structM,
    std::vector<Ref> membersLE,
    LLVMTypeRef innerStructLT,
    LLVMValueRef innerStructPtrLE) {
  for (int i = 0; i < membersLE.size(); i++) {
    auto memberRef = membersLE[i];
    auto memberType = structM->members[i]->type;
    auto memberName = structM->members[i]->name;
    auto ptrLE =
        LLVMBuildStructGEP2(builder, innerStructLT, innerStructPtrLE, i, memberName.c_str());
    auto memberLE =
        globalState->getRegion(memberType)
            ->checkValidReference(FL(), functionState, builder, false, structM->members[i]->type, memberRef);
    LLVMBuildStore(builder, memberLE, ptrLE);
  }
}

LLVMValueRef constructInnerStruct(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    StructDefinition* structM,
    LLVMTypeRef valStructL,
    const std::vector<Ref>& memberRefs) {

  // We always start with an undef, and then fill in its fields one at a
  // time.
  LLVMValueRef structValueBeingInitialized = LLVMGetUndef(valStructL);
  for (int i = 0; i < memberRefs.size(); i++) {
    auto memberLE =
        globalState->getRegion(structM->members[i]->type)
            ->checkValidReference(FL(), functionState, builder, false, structM->members[i]->type, memberRefs[i]);
    auto memberName = structM->members[i]->name;
    // Every time we fill in a field, it actually makes a new entire
    // struct value, and gives us a LLVMValueRef for the new value.
    // So, `structValueBeingInitialized` contains the latest one.
    structValueBeingInitialized =
        LLVMBuildInsertValue(
            builder,
            structValueBeingInitialized,
            memberLE,
            i,
            memberName.c_str());
  }
  return structValueBeingInitialized;
}

// Transmutes a weak ref of one ownership (such as borrow) to another ownership (such as weak).
Ref transmuteWeakRef(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceWeakRefMT,
    Kind* targetWeakRefMT,
    KindStructs* weakRefStructs,
    Ref sourceWeakRef) {
  // The WeakFatPtrLE constructors here will make sure that its a safe and valid transmutation.
  auto sourceWeakFatPtrLE =
      weakRefStructs->makeWeakFatPtr(
          sourceWeakRefMT,
          globalState->getRegion(sourceWeakRefMT)->checkValidReference(
              FL(), functionState, builder, false, sourceWeakRefMT, sourceWeakRef));
  auto sourceWeakFatPtrRawLE = sourceWeakFatPtrLE.refLE;
  auto targetWeakFatPtrLE = weakRefStructs->makeWeakFatPtr(targetWeakRefMT, sourceWeakFatPtrRawLE);
  auto targetWeakRef = toRef(globalState->getRegion(targetWeakRefMT), targetWeakRefMT, targetWeakFatPtrLE);
  return targetWeakRef;
}

LLVMValueRef mallocRuntimeSizedArray(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    LLVMTypeRef rsaWrapperLT,
    LLVMTypeRef rsaElementLT,
    LLVMValueRef lenI32LE) {
  auto lenI64LE = LLVMBuildZExt(builder, lenI32LE, LLVMInt64TypeInContext(globalState->context), "lenI16");
  auto sizeBytesLE =
      LLVMBuildAdd(
          builder,
          constI64LE(globalState, LLVMABISizeOfType(globalState->dataLayout, rsaWrapperLT)),
          LLVMBuildMul(
              builder,
              constI64LE(globalState, LLVMABISizeOfType(globalState->dataLayout, LLVMArrayType(rsaElementLT, 1))),
              lenI64LE,
              ""),
          "rsaMallocSizeBytes");

  auto newWrapperPtrLE = callMalloc(globalState, builder, sizeBytesLE);

  if (globalState->opt->census) {
    adjustCounterV(
        globalState, builder, globalState->metalCache->i64Type, globalState->liveHeapObjCounterLE, 1, false);
  }

  if (globalState->opt->census) {
    LLVMValueRef resultAsVoidPtrLE =
        LLVMBuildBitCast(
            builder, newWrapperPtrLE, LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0), "");
    globalState->externs->censusAdd.call(builder, {resultAsVoidPtrLE}, "");
  }

  return LLVMBuildBitCast(
      builder,
      newWrapperPtrLE,
      LLVMPointerType(rsaWrapperLT, 0),
      "newstruct");
}

// Transmutes a ptr of one ownership (such as own) to another ownership (such as borrow).
Ref transmutePtr(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool expectLive,
    Kind* sourceRefMT,
    Kind* targetRefMT,
    Ref sourceRef) {
  // The WrapperPtrLE constructors here will make sure that its a safe and valid transmutation.
  auto sourcePtrRawLE =
      globalState->getRegion(sourceRefMT)
          ->checkValidReference(FL(), functionState, builder, expectLive, sourceRefMT, sourceRef);
  auto targetWeakRef = toRef(globalState->getRegion(targetRefMT), targetRefMT, sourcePtrRawLE);
  return targetWeakRef;
}

//// Transmutes a ptr of one ownership (such as own) to another ownership (such as borrow).
//LiveRef transmuteLiveRef(
//    GlobalState* globalState,
//    FunctionState* functionState,
//    LLVMBuilderRef builder,
//    Kind* sourceRefMT,
//    Kind* targetRefMT,
//    LiveRef sourceRef) {
//  auto sourcePtrRawLE =
//      globalState->getRegion(sourceRefMT)
//          ->checkValidReference(FL(), functionState, builder, sourceRefMT, sourceRef);
//  auto targetWeakRef = toLiveRef(FL(), globalState, functionState, builder, targetRefMT, sourcePtrRawLE);
//  return targetWeakRef;
//}

Ref getRuntimeSizedArrayCapacity(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    WrapperPtrLE arrayRefLE) {
  auto int32LT = LLVMInt32TypeInContext(globalState->context);
  auto capacityPtrLE = getRuntimeSizedArrayCapacityPtr(globalState, builder, arrayRefLE);
  auto intLE = LLVMBuildLoad2(builder, int32LT, capacityPtrLE, "rsaCapacity");
  return toRef(globalState->getRegion(globalState->metalCache->i32Type), globalState->metalCache->i32Type, intLE);
}

ControlBlock makeFastWeakableControlBlock(GlobalState* globalState) {
  ControlBlock controlBlock(globalState, LLVMStructCreateNamed(globalState->context, "mutWeakableControlBlock"));
  // Fast mode mutables have no strong RC
  controlBlock.addMember(ControlBlockMember::UNUSED_32B);
  // This is where we put the size in the current generational heap, we can use it for something
  // else until we get rid of that.
  controlBlock.addMember(ControlBlockMember::UNUSED_32B);
  if (globalState->opt->census) {
    controlBlock.addMember(ControlBlockMember::CENSUS_TYPE_STR);
    controlBlock.addMember(ControlBlockMember::CENSUS_OBJ_ID);
  }
  controlBlock.addMember(ControlBlockMember::WRCI_32B);
  controlBlock.build();
  return controlBlock;
}

ControlBlock makeFastNonWeakableControlBlock(GlobalState* globalState) {
  ControlBlock controlBlock(globalState, LLVMStructCreateNamed(globalState->context, "mutNonWeakableControlBlock"));
  // Fast mode mutables have no strong RC
  controlBlock.addMember(ControlBlockMember::UNUSED_32B);
  // This is where we put the size in the current generational heap, we can use it for something
  // else until we get rid of that.
  controlBlock.addMember(ControlBlockMember::UNUSED_32B);
  if (globalState->opt->census) {
    controlBlock.addMember(ControlBlockMember::CENSUS_TYPE_STR);
    controlBlock.addMember(ControlBlockMember::CENSUS_OBJ_ID);
  }
  controlBlock.build();
  return controlBlock;
}


Ref resilientLockWeak(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool thenResultIsNever,
    bool elseResultIsNever,
    Kind* resultOptTypeM,
    Kind* constraintRefM,
    Kind* sourceWeakRefMT,
    Ref sourceWeakRefLE,
    std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
    std::function<Ref(LLVMBuilderRef)> buildElse,
    Ref isAliveLE,
    LLVMTypeRef resultOptTypeL,
    KindStructs* weakRefStructs) {
  return buildIfElseV(
      globalState, functionState, builder, isAliveLE,
//      resultOptTypeL,
      resultOptTypeM,
      resultOptTypeM,
      [globalState, functionState, constraintRefM, weakRefStructs, sourceWeakRefLE, sourceWeakRefMT, buildThen](
          LLVMBuilderRef thenBuilder) {
        // TODO extract more of this common code out?
        // The incoming "constraint" ref is actually already a weak ref, so just return it
        // (after wrapping it in a different Ref that actually thinks/knows it's a weak
        // reference).
        auto constraintRef =
            transmuteWeakRef(
                globalState, functionState, thenBuilder, sourceWeakRefMT, constraintRefM,
                weakRefStructs, sourceWeakRefLE);
        return buildThen(thenBuilder, constraintRef);
      },
      buildElse);
}


Ref interfaceRefIsForEdge(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    Kind* sourceInterfaceRefMT,
    Ref sourceInterfaceRef,
    StructKind *targetStructKind,
    InterfaceKind *sourceInterfaceKind) {

  LLVMValueRef itablePtrLE = nullptr;
  LLVMValueRef possibilityPtrLE = nullptr;
  std::tie(itablePtrLE, possibilityPtrLE) =
      globalState->getRegion(sourceInterfaceRefMT)
          ->explodeInterfaceRef(
              functionState, builder, sourceInterfaceRefMT, sourceInterfaceRef);

  auto targetStructDefM = globalState->program->getStruct(targetStructKind);
  auto targetEdgeM = targetStructDefM->getEdgeForInterface(sourceInterfaceKind);

  auto edgePtrLE = globalState->getInterfaceTablePtr(targetEdgeM);
  auto itableLT = structs->getInterfaceTableStruct(sourceInterfaceKind);

  auto itablePtrDiffLE = LLVMBuildPtrDiff2(builder, itableLT, itablePtrLE, edgePtrLE, "ptrDiff");
  auto itablePtrsMatchLE = LLVMBuildICmp(builder, LLVMIntEQ, itablePtrDiffLE, constI64LE(globalState, 0), "ptrsMatch");
  auto itablePtrsMatchRef =
      toRef(globalState->getRegion(globalState->metalCache->boolType),
          globalState->metalCache->boolType,
          itablePtrsMatchLE);
  return itablePtrsMatchRef;
}

Ref regularDowncast(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    Kind* resultOptTypeM,
    Kind* sourceInterfaceRefMT,
    Ref sourceInterfaceRef,
    Kind* targetKind,
    std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
    std::function<Ref(LLVMBuilderRef)> buildElse) {
  { assert(false); throw 1337; }
//   LLVMValueRef itablePtrLE = nullptr;
//   LLVMValueRef newVirtualArgLE = nullptr;
//   std::tie(itablePtrLE, newVirtualArgLE) =
//       globalState->getRegion(sourceInterfaceRefMT)
//           ->explodeInterfaceRef(
//               functionState, builder, sourceInterfaceRefMT, sourceInterfaceRef);
//   buildFlare(FL(), globalState, functionState, builder);
//
//   auto targetStructKind = dynamic_cast<StructKind*>(targetKind);
//   assert(targetStructKind);
//
//   auto sourceInterfaceKind = dynamic_cast<InterfaceKind*>(peel_all_references(sourceInterfaceRefMT));
//   assert(sourceInterfaceKind);
//
//   auto targetStructDefM = globalState->program->getStruct(targetStructKind);
//   auto targetEdgeM = targetStructDefM->getEdgeForInterface(sourceInterfaceKind);
//
//   auto edgePtrLE = globalState->getInterfaceTablePtr(targetEdgeM);
//   auto itableLT = structs->getInterfaceTableStruct(sourceInterfaceKind);
//
//   auto itablePtrDiffLE = LLVMBuildPtrDiff2(builder, itableLT, itablePtrLE, edgePtrLE, "ptrDiff");
//   auto itablePtrsMatchLE = LLVMBuildICmp(builder, LLVMIntEQ, itablePtrDiffLE, constI64LE(globalState, 0), "ptrsMatch");
//   auto itablePtrsMatchRef =
//       toRef(globalState->getRegion(globalState->metalCache->boolType), globalState->metalCache->boolType, itablePtrsMatchLE);
//
//   auto resultOptTypeLE = globalState->getRegion(resultOptTypeM)->translateType(resultOptTypeM);
//
//   return buildIfElseV(
//       globalState, functionState, builder, itablePtrsMatchRef,
// //      resultOptTypeLE,
//       resultOptTypeM,
//       resultOptTypeM,
//       [globalState, sourceInterfaceRefMT, structs, targetKind, newVirtualArgLE, buildThen](
//           LLVMBuilderRef thenBuilder) {
//         auto resultStructRefMT =
//             globalState->metalCache->getReference(
//                 sourceInterfaceRefMT->ownership, sourceInterfaceRefMT->location, targetKind);
//         auto resultStructRefLE =
//             structs->downcastPtr(thenBuilder, resultStructRefMT, newVirtualArgLE);
//         auto resultStructRef = toRef(globalState->getRegion(resultStructRefMT), resultStructRefMT, resultStructRefLE);
//         return buildThen(thenBuilder, resultStructRef);
//       },
//       buildElse);
}

Ref resilientDowncast(
    GlobalState* globalState,
    FunctionState *functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    KindStructs* weakRefStructs,
    Kind *resultOptTypeM,
    Kind *sourceInterfaceRefMT,
    Ref &sourceInterfaceRef,
    Kind *targetKind,
    const std::function<Ref(LLVMBuilderRef, Ref)> &buildThen,
    std::function<Ref(LLVMBuilderRef)> &buildElse,
    StructKind *targetStructKind,
    InterfaceKind *sourceInterfaceKind) {
  { assert(false); throw 1337; }
//   auto itablePtrsMatchRef =
//       interfaceRefIsForEdge(
//           globalState,
//           functionState,
//           builder,
//           structs,
//           sourceInterfaceRefMT,
//           sourceInterfaceRef,
//           targetStructKind,
//           sourceInterfaceKind);
//
//   auto resultOptTypeLE = globalState->getRegion(resultOptTypeM)->translateType(resultOptTypeM);
//
//   return buildIfElseV(
//       globalState, functionState, builder, itablePtrsMatchRef,
// //      resultOptTypeLE,
//       resultOptTypeM,
//       resultOptTypeM,
//       [globalState, weakRefStructs, structs, functionState, sourceInterfaceRef, sourceInterfaceRefMT, targetKind, targetStructKind, buildThen](
//           LLVMBuilderRef thenBuilder) {
//         auto possibilityPtrLE =
//             std::get<1>(
//                 globalState->getRegion(sourceInterfaceRefMT)
//                     ->explodeInterfaceRef(functionState, thenBuilder, sourceInterfaceRefMT, sourceInterfaceRef));
//         buildFlare(FL(), globalState, functionState, thenBuilder);
//
//         auto resultStructRefMT =
//             globalState->metalCache->getReference(
//                 sourceInterfaceRefMT->ownership, sourceInterfaceRefMT->location, targetKind);
//         switch (sourceInterfaceRefMT->ownership) {
//           case Ownership::OWN: {
//             auto resultStructRefLE = structs->downcastPtr(thenBuilder, resultStructRefMT, possibilityPtrLE);
//             auto
//                 resultStructRef = toRef(globalState->getRegion(resultStructRefMT), resultStructRefMT, resultStructRefLE);
//             return buildThen(thenBuilder, resultStructRef);
//           }
//           case Ownership::MUTABLE_BORROW:
//           case Ownership::IMMUTABLE_BORROW:
//           case Ownership::WEAK: {
//             auto resultStructRefLE =
//                 weakRefStructs->downcastWeakFatPtr(
//                     thenBuilder, targetStructKind, resultStructRefMT, possibilityPtrLE);
//             auto targetWeakRef = toRef(globalState->getRegion(resultStructRefMT), resultStructRefMT, resultStructRefLE);
//             return buildThen(thenBuilder, targetWeakRef);
//           }
//           default:
//             { assert(false); throw 1337; }
//         }
//       },
//       buildElse);
}

Ref normalLocalStore(GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr, Ref refToStore) {
  auto region = globalState->getRegion(local->type);
  auto localLT = region->translateType(local->type);
  // We need to load the old ref *after* we evaluate the source expression,
  // Because of expressions like: Ship() = (mut b = (mut a = (mut b = Ship())));
  // See mutswaplocals.vale for test case.
  auto oldRefLE = LLVMBuildLoad2(builder, localLT, localAddr, local->name.c_str());
  auto oldRef = toRef(region, local->type, oldRefLE);
  region->checkValidReference(FL(), functionState, builder, false, local->type, oldRef);
  auto toStoreLE = region->checkValidReference(FL(), functionState, builder, false, local->type, refToStore);
  LLVMBuildStore(builder, toStoreLE, localAddr);
  return oldRef;
}


void regularCheckValidReference(
    AreaAndFileAndLine checkerAFL,
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* refM,
    LLVMValueRef refLE) {

  if (auto interfaceKindM = dynamic_cast<InterfaceKind *>(peel_all_references(refM))) {
    auto interfaceFatPtrLE = kindStructs->makeInterfaceFatPtr(checkerAFL, functionState, builder,
        refM, refLE);
    auto itablePtrLE = getTablePtrFromInterfaceRef(builder, interfaceFatPtrLE);
    buildAssertCensusContains(checkerAFL, globalState, functionState, builder, itablePtrLE);
  }
  if (dynamic_cast<ShareRef*>(refM)) {
    auto controlBlockPtrLE =
        kindStructs->getControlBlockPtr(checkerAFL, functionState, builder, refLE, refM);

    // We dont check ref count >0 because imm destructors receive with rc=0.
    //      auto rcLE = getRcFromControlBlockPtr(globalState, builder, controlBlockPtrLE);
    //      auto rcPositiveLE = LLVMBuildICmp(builder, LLVMIntSGT, rcLE, constI64LE(globalState, 0), "");
    //      buildAssertV(checkerAFL, globalState, functionState, blockState, builder, rcPositiveLE, "Invalid RC!");

    buildAssertCensusContains(checkerAFL, globalState, functionState, builder,
        controlBlockPtrLE.refLE);
  } else {
    // Is inline, nothing to do
  }
}

//LoadResult resilientLoadElementFromRSAWithoutUpgrade(
//    GlobalState* globalState,
//    FunctionState* functionState,
//    LLVMBuilderRef builder,
//    KindStructs* kindStructs,
//    bool capacityExists,
//    Kind* rsaRefMT,
//    Mutability mutability,
//    Kind* elementType,
//    RuntimeSizedArrayT* rsaMT,
//    LiveRef arrayRef,
//    Ref indexRef) {
//  switch (rsaRefMT->ownership) {
//    case Ownership::MUTABLE_SHARE:
//    case Ownership::IMMUTABLE_SHARE:
//    case Ownership::OWN: {
//      auto rsaRefLE =
//          globalState->getRegion(rsaRefMT)
//              ->checkValidReference(FL(), functionState, builder, true, rsaRefMT, arrayRef.inner);
//      auto wrapperPtrLE =
//          kindStructs->makeWrapperPtr(FL(), functionState, builder, rsaRefMT, rsaRefLE);
//      auto sizeRef = ::getRuntimeSizedArrayLength(globalState, functionState, builder, wrapperPtrLE);
//      auto arrayElementsPtrLE = getRuntimeSizedArrayContentsPtr(builder, capacityExists, wrapperPtrLE);
//      buildFlare(FL(), globalState, functionState, builder);
//      return loadElement(
//          globalState, functionState, builder, arrayElementsPtrLE, elementType, sizeRef, indexRef);
//    }
//    case Ownership::MUTABLE_BORROW:
//    case Ownership::IMMUTABLE_BORROW: {
//      auto wrapperPtrLE =
//          kindStructs.makeWrapperPtr(
//              FL(), functionState, builder, rsaRefMT,
//              hgmWeaks.checkGenFatPtr(
//                  FL(), functionState, builder, rsaRefMT, arrayRef.inner, true));
//      return ::getRuntimeSizedArrayLength(globalState, functionState, builder, wrapperPtrLE);
//    }
////    case Ownership::IMMUTABLE_BORROW: {
////      auto rsaWrapperPtrLE =
////          kindStructs->makeWrapperPtr(
////              FL(), functionState, builder, rsaRefMT,
////              globalState->getRegion(rsaRefMT)
////                  ->checkValidReference(FL(), functionState, builder, true, rsaRefMT, arrayRef.inner));
////      auto sizeRef = ::getRuntimeSizedArrayLength(globalState, functionState, builder, rsaWrapperPtrLE);
////      auto arrayElementsPtrLE =
////          getRuntimeSizedArrayContentsPtr(
////              builder, capacityExists, rsaWrapperPtrLE);
////      buildFlare(FL(), globalState, functionState, builder);
////      return loadElement(
////          globalState, functionState, builder, arrayElementsPtrLE, elementType,
////          sizeRef, indexRef);
////    }
//    case Ownership::WEAK:
//      { assert(false); throw 1337; } // VIR never loads from a weak ref
//    default:
//      { assert(false); throw 1337; }
//  }
//}

LiveRef constructRuntimeSizedArray(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* rsaMT,
    Kind* elementType,
    RuntimeSizedArrayT* runtimeSizedArrayT,
    LLVMTypeRef rsaWrapperPtrLT,
    LLVMTypeRef rsaElementLT,
    Ref initialSizeRef,
    Ref capacityRef,
    bool capacityExists,
    const std::string& typeName,
    std::function<void(LLVMBuilderRef builder, ControlBlockPtrLE controlBlockPtrLE)> fillControlBlock) {
  buildFlare(FL(), globalState, functionState, builder, "Constructing RSA!");

  auto capacityLE =
      globalState->getRegion(globalState->metalCache->i32Type)->checkValidReference(FL(),
          functionState, builder, true, globalState->metalCache->i32Type, capacityRef);
  buildFlare(FL(), globalState, functionState, builder, "RSA capacity: ", capacityLE);

  auto ptrLE = mallocRuntimeSizedArray(globalState, builder, rsaWrapperPtrLT, rsaElementLT, capacityLE);
  auto rsaWrapperPtrLE =
      kindStructs->makeWrapperPtr(FL(), functionState, builder, rsaMT, ptrLE);
  fillControlBlock(
      builder,
      kindStructs->getConcreteControlBlockPtr(FL(), functionState, builder, rsaWrapperPtrLE));
  auto sizeLE =
      globalState->getRegion(globalState->metalCache->i32Type)->checkValidReference(FL(),
          functionState, builder, true, globalState->metalCache->i32Type, initialSizeRef);
  LLVMBuildStore(builder, sizeLE, getRuntimeSizedArrayLengthPtr(globalState, builder, rsaWrapperPtrLE));
  if (capacityExists) {
    LLVMBuildStore(builder, capacityLE, getRuntimeSizedArrayCapacityPtr(globalState, builder, rsaWrapperPtrLE));
  }
  auto rsaLiveRef = toLiveRef(rsaWrapperPtrLE);
  auto rsaRef = toRef(globalState, rsaMT, rsaLiveRef);

  if (globalState->opt->census) {
    auto objIdLE =
        globalState->getRegion(rsaMT)
            ->getCensusObjectId(FL(), functionState, builder, rsaMT, rsaRef);
    auto addrIntLE = ptrToIntLE(globalState, builder, ptrLE);
    buildFlare(
        FL(), globalState, functionState, builder,
        "Allocated object ", typeName, " &", addrIntLE, " obj id ", objIdLE, "\n");
  }

  return rsaLiveRef;
}

Ref upcastStrong(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* sourceStructMT,
    StructKind* sourceStructKindM,
    Ref sourceRefLE,
    Kind* targetInterfaceTypeM,
    InterfaceKind* targetInterfaceKindM) {
  auto sourceStructWrapperPtrLE =
      kindStructs->makeWrapperPtr(
          FL(), functionState, builder, sourceStructMT,
          globalState->getRegion(sourceStructMT)
              ->checkValidReference(FL(), functionState, builder, false, sourceStructMT, sourceRefLE));
  auto resultInterfaceFatPtrLE =
      upcastThinPtr(
          globalState, functionState, kindStructs, builder, sourceStructMT,
          sourceStructKindM,
          sourceStructWrapperPtrLE, targetInterfaceTypeM, targetInterfaceKindM);
  return toRef(globalState->getRegion(targetInterfaceTypeM), targetInterfaceTypeM, resultInterfaceFatPtrLE);
}

Ref upcastWeak(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* weakRefStructs,
    Kind* sourceStructMT,
    StructKind* sourceStructKindM,
    Ref sourceRefLE,
    Kind* targetInterfaceTypeM,
    InterfaceKind* targetInterfaceKindM) {
  auto sourceStructValueType = peel_all_references(sourceStructMT);
  auto sourceWeakStructFatPtrLE =
      weakRefStructs->makeWeakFatPtr(
          sourceStructMT,
          globalState->getRegion(sourceStructValueType)->checkValidReference(FL(),
              functionState, builder, false, sourceStructMT, sourceRefLE));
  return globalState->getRegion(sourceStructValueType)->upcastWeak(
      functionState,
      builder,
      sourceWeakStructFatPtrLE,
      sourceStructKindM,
      sourceStructMT,
      targetInterfaceKindM,
      targetInterfaceTypeM);
}

void regularFillControlBlock(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* structs,
    LLVMBuilderRef builder,
    ValueKind* kindM,
    ControlBlockPtrLE controlBlockPtrLE,
    const std::string& typeName,
    WrcWeaks* wrcWeaks) {
  LLVMValueRef newControlBlockLE = LLVMGetUndef(structs->getControlBlock(kindM)->getStruct());

  newControlBlockLE =
      fillControlBlockCensusFields(
          from, globalState, functionState, structs, builder, kindM, newControlBlockLE, typeName);

  newControlBlockLE =
      insertStrongRc(globalState, builder, structs, kindM, newControlBlockLE);
  if (globalState->getKindWeakability(kindM) == Weakability::WEAKABLE) {
    newControlBlockLE =
        wrcWeaks->fillWeakableControlBlock(functionState, builder, structs, kindM, newControlBlockLE);
  }

  LLVMBuildStore(
      builder,
      newControlBlockLE,
      controlBlockPtrLE.refLE);
}

Ref getRuntimeSizedArrayLengthStrong(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* rsaRefMT,
    LiveRef arrayRef) {
  auto wrapperPtrLE = toWrapperPtr(functionState, builder, kindStructs, rsaRefMT, arrayRef);
  return ::getRuntimeSizedArrayLength(globalState, functionState, builder, wrapperPtrLE);
}

Ref getRuntimeSizedArrayCapacityStrong(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* rsaRefMT,
    LiveRef arrayRef) {
  auto wrapperPtrLE = toWrapperPtr(functionState, builder, kindStructs, rsaRefMT, arrayRef);
  return ::getRuntimeSizedArrayCapacity(globalState, functionState, builder, wrapperPtrLE);
}

LoadResult regularLoadStrongMember(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* structRefMT,
    LiveRef structRef,
    int memberIndex,
    Kind* expectedMemberType,
    Kind* targetType,
    const std::string& memberName) {

  auto wrapperPtrLE = toWrapperPtr(functionState, builder, kindStructs, structRefMT, structRef);

  auto innerStructPtrLE =
      kindStructs->getStructContentsPtr(builder, wrapperPtrLE);

  auto structMT = dynamic_cast<StructKind*>(peel_all_references(structRefMT));
  assert(structMT);
  auto innerStructLT = kindStructs->getStructInnerStruct(structMT);

  auto memberLE =
      loadInnerInnerStructMember(
          globalState,
          functionState,
          builder,
          innerStructLT,
          innerStructPtrLE,
          memberIndex,
          expectedMemberType,
          memberName);
  return memberLE;
}

std::tuple<LLVMValueRef, LLVMValueRef> explodeStrongInterfaceRef(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* virtualParamMT,
    Ref virtualArgRef) {
  auto virtualArgLE =
      globalState->getRegion(virtualParamMT)->checkValidReference(
          FL(), functionState, builder, false, virtualParamMT, virtualArgRef);
  LLVMValueRef itablePtrLE = nullptr;
  LLVMValueRef newVirtualArgLE = nullptr;
  auto virtualArgInterfaceFatPtrLE =
      kindStructs->makeInterfaceFatPtr(
          FL(), functionState, builder, virtualParamMT, virtualArgLE);
  itablePtrLE = getItablePtrFromInterfacePtr(globalState, functionState, builder,
      virtualParamMT, virtualArgInterfaceFatPtrLE);
  buildFlare(FL(), globalState, functionState, builder);
  auto objVoidPtrLE =
      kindStructs->getVoidPtrFromInterfacePtr(
          functionState, builder, virtualParamMT, virtualArgInterfaceFatPtrLE);
  newVirtualArgLE = objVoidPtrLE;

//  buildFlare(FL(), globalState, functionState, builder, "Exploding, objPtrLE: ", ptrToIntLE(globalState, builder, objVoidPtrLE), " itablePtrLE ", ptrToIntLE(globalState, builder, itablePtrLE));

  return std::make_tuple(itablePtrLE, newVirtualArgLE);
}

std::tuple<LLVMValueRef, LLVMValueRef> explodeWeakInterfaceRef(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    FatWeaks* fatWeaks,
    KindStructs* weakRefStructs,
    Kind* virtualParamMT,
    Ref virtualArgRef,
    std::function<WeakFatPtrLE(WeakFatPtrLE weakInterfaceFatPtrLE)> weakInterfaceRefToWeakStructRef) {
  auto virtualArgLE =
      globalState->getRegion(virtualParamMT)
          ->checkValidReference(FL(), functionState, builder, false, virtualParamMT, virtualArgRef);
  auto weakFatPtrLE = weakRefStructs->makeWeakFatPtr(virtualParamMT, virtualArgLE);
  // Disassemble the weak interface ref.
  LLVMValueRef itablePtrLE = nullptr;
  LLVMValueRef objPtrLE = nullptr;
  auto interfaceRefLE =
      kindStructs->makeInterfaceFatPtrWithoutChecking(
          FL(), functionState, builder, virtualParamMT,
          fatWeaks->getInnerRefFromWeakRef(
              functionState, builder, virtualParamMT, weakFatPtrLE));
  itablePtrLE = getTablePtrFromInterfaceRef(builder, interfaceRefLE);
  // Now, reassemble a weak void* ref to the struct.
  auto weakVoidStructRefLE = weakInterfaceRefToWeakStructRef(weakFatPtrLE);
  objPtrLE = weakVoidStructRefLE.refLE;
  return std::make_tuple(itablePtrLE, objPtrLE);
}

Ref regularWeakAlias(
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* kindStructs,
    WrcWeaks* wrcWeaks,
    LLVMBuilderRef builder,
    Kind* sourceRefMT,
    Kind* targetRefMT,
    Ref sourceRef) {
  auto sourceValueType = peel_all_references(sourceRefMT);
  auto targetValueType = peel_all_references(targetRefMT);
  if (auto structKindM = dynamic_cast<StructKind*>(sourceValueType)) {
    auto objPtrLE =
        kindStructs->makeWrapperPtr(
            FL(), functionState, builder, sourceRefMT,
            globalState->getRegion(sourceValueType)
                ->checkValidReference(FL(), functionState, builder, false, sourceRefMT, sourceRef));
    return toRef(
        globalState->getRegion(targetValueType),
        targetRefMT,
        wrcWeaks->assembleStructWeakRef(
            functionState, builder,
            sourceRefMT, targetRefMT, structKindM, objPtrLE));
  } else if (auto interfaceKind = dynamic_cast<InterfaceKind*>(sourceValueType)) {
    auto objPtrLE =
        kindStructs->makeInterfaceFatPtr(
            FL(), functionState, builder, sourceRefMT,
            globalState->getRegion(sourceValueType)
                ->checkValidReference(FL(), functionState, builder, false, sourceRefMT, sourceRef));
    return toRef(
        globalState->getRegion(targetValueType),
        targetRefMT,
        wrcWeaks->assembleInterfaceWeakRef(
            functionState, builder,
            sourceRefMT, targetRefMT, interfaceKind, objPtrLE));
  } else { assert(false); throw 1337; }
}

Ref regularInnerLockWeak(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool thenResultIsNever,
    bool elseResultIsNever,
    Kind* resultOptTypeM,
    Kind* constraintRefM,
    Kind* sourceWeakRefMT,
    Ref sourceWeakRefLE,
    std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
    std::function<Ref(LLVMBuilderRef)> buildElse,
    Ref isAliveLE,
    LLVMTypeRef resultOptTypeL,
    KindStructs* weakRefStructsSource,
    FatWeaks* fatWeaks) {
  return buildIfElseV(
      globalState, functionState, builder, isAliveLE,
//      resultOptTypeL,
      resultOptTypeM,
      resultOptTypeM,
      [globalState, functionState, fatWeaks, weakRefStructsSource, constraintRefM, sourceWeakRefLE, sourceWeakRefMT, buildThen](
          LLVMBuilderRef thenBuilder) {
        auto weakFatPtrLE =
            weakRefStructsSource->makeWeakFatPtr(
                sourceWeakRefMT,
                globalState->getRegion(sourceWeakRefMT)
                    ->checkValidReference(FL(), functionState, thenBuilder, false, sourceWeakRefMT, sourceWeakRefLE));
        auto constraintRefLE =
            fatWeaks->getInnerRefFromWeakRef(
                functionState,
                thenBuilder,
                sourceWeakRefMT,
                weakFatPtrLE);
        auto constraintRef =
            toRef(globalState->getRegion(constraintRefM), constraintRefM, constraintRefLE);
        return buildThen(thenBuilder, constraintRef);
      },
      buildElse);
}

void storeMemberStrong(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* structRefMT,
    LiveRef structRef,
    int memberIndex,
    const std::string& memberName,
    LLVMValueRef newValueLE) {
  auto structMT = dynamic_cast<StructKind*>(peel_all_references(structRefMT));
  assert(structMT);
  LLVMValueRef innerStructPtrLE = nullptr;
  auto wrapperPtrLE = toWrapperPtr(functionState, builder, kindStructs, structRefMT, structRef);
  innerStructPtrLE = kindStructs->getStructContentsPtr(builder, wrapperPtrLE);
  auto innerStructLT = kindStructs->getStructInnerStruct(structMT);
  storeInnerInnerStructMember(
      builder, innerStructLT, innerStructPtrLE, memberIndex, memberName, newValueLE);
}

void storeMemberWeak(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* structRefMT,
    LiveRef structRef,
    int memberIndex,
    const std::string& memberName,
    LLVMValueRef newValueLE) {
  { assert(false); throw 1337; } // we dont really do weak anymore
//  LLVMValueRef innerStructPtrLE = nullptr;
//  auto wrapperPtrLE =
//      globalState->getRegion(structRefMT)->lockWeakRef(
//          FL(), functionState, builder, structRefMT, structRef);
//  innerStructPtrLE = kindStructs->getStructContentsPtr(builder, structRefMT->kind, wrapperPtrLE);
//  storeInnerInnerStructMember(builder, innerStructPtrLE, memberIndex, memberName, newValueLE);
}

ValeFuncPtrLE getInterfaceMethodFunctionPtrFromItable(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    Kind* virtualParamMT,
    Ref virtualArgRef,
    int indexInEdge) {
  auto virtualParamValueType = peel_all_references(virtualParamMT);
  LLVMValueRef itablePtrLE = nullptr;
  LLVMValueRef newVirtualArgLE = nullptr;
  std::tie(itablePtrLE, newVirtualArgLE) =
      globalState->getRegion(virtualParamValueType)
          ->explodeInterfaceRef(
              functionState, builder, virtualParamMT, virtualArgRef);
  buildFlare(FL(), globalState, functionState, builder);

  auto interfaceMT = dynamic_cast<InterfaceKind*>(virtualParamValueType);
  assert(interfaceMT);
//  int indexInEdge = 0;
//  InterfaceMethod* method = nullptr;
//  std::tie(indexInEdge, method) = globalState->getInterfaceMethod(interfaceMT, prototype);

  assert(LLVMGetTypeKind(LLVMTypeOf(itablePtrLE)) == LLVMPointerTypeKind);
  //buildFlare(FL(), globalState, functionState, builder, "index in edge: ", indexInEdge);
  auto itableStructLT = structs->getInterfaceTableStruct(interfaceMT);
  auto funcPtrPtrLE =
      LLVMBuildStructGEP2(builder, itableStructLT, itablePtrLE, indexInEdge, "methodPtrPtr");

  auto funcLT =
      globalState->getInterfaceFunctionTypesNonPointer(interfaceMT)[indexInEdge];

  auto resultLE = LLVMBuildLoad2(builder, LLVMPointerType(funcLT, 0), funcPtrPtrLE, "methodPtr");
  //buildFlare(FL(), globalState, functionState, builder, "method ptr: ", ptrToIntLE(globalState, builder, resultLE));
  return ValeFuncPtrLE(RawFuncPtrLE(funcLT, resultLE));
}


Ref normalLocalLoad(GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr) {
  auto region = globalState->getRegion(local->type);
  auto localLT = region->translateType(local->type);
  auto sourceLE = LLVMBuildLoad2(builder, localLT, localAddr, local->name.c_str());
  auto sourceRef = toRef(region, local->type, sourceLE);
  region->checkValidReference(FL(), functionState, builder, false, local->type, sourceRef);
  return sourceRef;
}

Ref regularRefFromHostHandle(
    GlobalState* globalState,
    FunctionState *functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind *sourceRefMT,
    LLVMValueRef sourceRefLE) {
  // Per @FRMACZ, the boundary does no reference counting: unpack the handle's
  // packed pointer without touching the RC.
  auto sourceValueType = peel_all_references(sourceRefMT);

  if (dynamic_cast<StructKind*>(sourceValueType) ||
      dynamic_cast<StaticSizedArrayT*>(sourceValueType) ||
      dynamic_cast<RuntimeSizedArrayT*>(sourceValueType) ||
      dynamic_cast<Str*>(sourceValueType)) {
    assert(LLVMTypeOf(sourceRefLE) == globalState->getFfiHandleStructs()->getConcreteHandleStructLT());

    auto ffiHandleStructs = globalState->getFfiHandleStructs();

    auto refLT = globalState->getRegion(sourceValueType)->translateType(sourceRefMT);

    auto membersLE = ffiHandleStructs->explodeForRegularConcrete(globalState, functionState, builder, sourceRefLE);
    auto objPtrLE = LLVMBuildIntToPtr(builder, membersLE.objPtrI64LE, refLT, "refA");

    auto ref = toRef(globalState->getRegion(sourceValueType), sourceRefMT, objPtrLE);
    globalState->getRegion(sourceValueType)
        ->checkValidReference(FL(), functionState, builder, true, sourceRefMT, ref);
    return ref;
  } else if (auto interfaceMT = dynamic_cast<InterfaceKind*>(sourceValueType)) {
    assert(LLVMTypeOf(sourceRefLE) == globalState->getFfiHandleStructs()->getInterfaceHandleStructLT());

    auto ffiHandleStructs = globalState->getFfiHandleStructs();

    auto itablePtrLT = LLVMPointerType(kindStructs->getInterfaceTableStruct(interfaceMT), 0);
    auto objPtrLT = LLVMPointerType(kindStructs->getControlBlock(interfaceMT)->getStruct(), 0);
    auto refLT = globalState->getRegion(sourceValueType)->translateType(sourceRefMT);

    auto membersLE = ffiHandleStructs->explodeForRegularInterface(globalState, functionState, builder, sourceRefLE);
    auto itablePtrLE = LLVMBuildIntToPtr(builder, membersLE.typeInfoPtrI64LE, itablePtrLT, "refC");
    auto objPtrLE = LLVMBuildIntToPtr(builder, membersLE.objPtrI64LE, objPtrLT, "refB");

    auto interfaceFatPtrRawLE = makeInterfaceRefStruct(globalState, functionState, builder, kindStructs, interfaceMT, objPtrLE, itablePtrLE);

    auto interfaceFatPtrLE = kindStructs->makeInterfaceFatPtr(FL(), functionState, builder, sourceRefMT, interfaceFatPtrRawLE);

    auto ref = toRef(globalState->getRegion(sourceValueType), sourceRefMT, interfaceFatPtrLE);
    globalState->getRegion(sourceValueType)
        ->checkValidReference(FL(), functionState, builder, true, sourceRefMT, ref);
    return ref;
  } else {
    { assert(false); throw 1337; }
  }
  { assert(false); throw 1337; }
}

LLVMValueRef regularRefToHostHandle(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceRefMT,
    Ref sourceRef) {
  // Per @FRMACZ, the boundary does no reference counting: pack the pointer
  // without touching the RC.
  auto sourceValueType = peel_all_references(sourceRefMT);

  if (dynamic_cast<StructKind*>(sourceValueType) ||
      dynamic_cast<StaticSizedArrayT*>(sourceValueType) ||
      dynamic_cast<RuntimeSizedArrayT*>(sourceValueType) ||
      dynamic_cast<Str*>(sourceValueType)) {
    auto sourceRefLE =
        globalState->getRegion(sourceValueType)
            ->checkValidReference(FL(), functionState, builder, false, sourceRefMT, sourceRef);
    auto objPtrI64LE = LLVMBuildPtrToInt(builder, sourceRefLE, LLVMInt64TypeInContext(globalState->context), "objPtrInt");

    auto handleLE =
        globalState->getFfiHandleStructs()->implodeForRegularConcrete(
            globalState, functionState, builder, objPtrI64LE);
    return handleLE;
  } else if (dynamic_cast<InterfaceKind*>(sourceValueType)) {
    globalState->getRegion(sourceValueType)
        ->checkValidReference(FL(), functionState, builder, false, sourceRefMT, sourceRef);
    LLVMValueRef itablePtrLE = nullptr, objPtrLE = nullptr;
    std::tie(itablePtrLE, objPtrLE) = globalState->getRegion(sourceValueType)->explodeInterfaceRef(functionState, builder, sourceRefMT, sourceRef);
    auto objPtrI64LE = LLVMBuildPtrToInt(builder, objPtrLE, LLVMInt64TypeInContext(globalState->context), "objPtrInt");
    auto itablePtrI64LE = LLVMBuildPtrToInt(builder, itablePtrLE, LLVMInt64TypeInContext(globalState->context), "itablePtrInt");

    auto handleLE =
        globalState->getFfiHandleStructs()->implodeForRegularInterface(
            globalState, functionState, builder,
            itablePtrI64LE, objPtrI64LE);
    return handleLE;
  } else {
    { assert(false); throw 1337; }
  }
  { assert(false); throw 1337; }
}


// Per @HTSLVBDTCZ, these emit the per-kind C typedefs that give each class its
// own distinct C type, even though all concretes (and all interfaces) share one
// LLVM handle type internally. The typedef name is the kind's export name
// verbatim. Both regions (RCImm and Unsafe) emit through these.
std::string generateConcreteHandleStructDefC(Package* currentPackage, const std::string& name) {
  // Concrete handle: 8-byte { i64 obj }. See ffihandlestructs.h.
  return std::string() + "typedef struct " + name + " { uint64_t _reserved; } " + name + ";\n";
}

std::string generateInterfaceHandleStructDefC(Package* currentPackage, const std::string& name) {
  // Interface handle: 16-byte { i64 obj, i64 typeinfo }. See ffihandlestructs.h.
  return std::string() + "typedef struct " + name + " { uint64_t _reserved0; uint64_t _reserved1; } " + name + ";\n";
}


void fastPanic(GlobalState* globalState, AreaAndFileAndLine from, LLVMBuilderRef builder) {
  buildPrintAreaAndFileAndLineToStderr(globalState, builder, from);
  buildPrintToStderr(globalState, builder, "Tried dereferencing dangling reference! ");
  buildPrintToStderr(globalState, builder, "Exiting!\n");
  // See MPESC for status codes
  auto exitCodeIntLE = LLVMConstInt(LLVMInt64TypeInContext(globalState->context), 14, false);
  buildCallWith64BitSExt(globalState, builder, globalState->externs->exit, {exitCodeIntLE});
}


