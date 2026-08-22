#include "../common/fatweaks/fatweaks.h"
//#include "../common/wrcweaks/wrcweaks.h"
#include "../../translatetype.h"
#include "../common/common.h"
#include "../../utils/counters.h"
#include "../common/controlblock.h"
#include "../../utils/branch.h"
#include "../common/heap.h"
#include "../../function/expressions/shared/members.h"
#include "../../function/expressions/shared/elements.h"
#include "../../function/expressions/shared/string.h"
#include "unsafe.h"
#include <sstream>

LLVMTypeRef makeUnsafeWeakRefHeaderStruct(GlobalState* globalState) {
  auto wrciRefStructL = LLVMStructCreateNamed(globalState->context, "__UnsafeWweakRef");

  std::vector<LLVMTypeRef> memberTypesL;

  // impl weaks
//  assert(WEAK_REF_HEADER_MEMBER_INDEX_FOR_WRCI == memberTypesL.size());
//  memberTypesL.push_back(LLVMInt32TypeInContext(globalState->context));

  LLVMStructSetBody(wrciRefStructL, memberTypesL.data(), memberTypesL.size(), false);

  return wrciRefStructL;
}

Unsafe::Unsafe(GlobalState* globalState_) :
    globalState(globalState_),
    kindStructs(
        globalState,
        makeFastNonWeakableControlBlock(globalState),
        makeFastWeakableControlBlock(globalState),
        WrcWeaks::makeWeakRefHeaderStruct(globalState)),
    fatWeaks(globalState_, &kindStructs) {
}

void Unsafe::mainSetup(FunctionState* functionState, LLVMBuilderRef builder) {
//  wrcWeaks.mainSetup(functionState, builder);
}

void Unsafe::mainCleanup(FunctionState* functionState, LLVMBuilderRef builder) {
//  wrcWeaks.mainCleanup(functionState, builder);
}

RegionId* Unsafe::getRegionId() {
  return globalState->metalCache->mutRegionId;
}

LiveRef Unsafe::constructStaticSizedArray(
    FunctionState *functionState,
    LLVMBuilderRef builder,
    Kind *referenceM,
    StaticSizedArrayT *kindM) {
  auto ssaDef = globalState->program->getStaticSizedArray(kindM);
  auto structLT =
      kindStructs.getStaticSizedArrayWrapperStruct(ssaDef->kind);
  auto newStructLE =
      kindStructs.makeWrapperPtr(
          FL(), functionState, builder, referenceM,
          mallocKnownSize(globalState, functionState, builder, structLT));
  return toLiveRef(newStructLE);
}

Ref Unsafe::mallocStr(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef lengthLE,
    LLVMValueRef sourceCharsPtrLE) {
  { assert(false); throw 1337; }
  exit(1);
}

Ref Unsafe::allocate(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* desiredReference,
    const std::vector<Ref>& memberRefs) {
  auto desiredValueType = peel_all_references(desiredReference);
  auto structKind = dynamic_cast<StructKind*>(desiredValueType);
  auto structM = globalState->program->getStruct(structKind);
  auto countedStructL = kindStructs.getStructWrapperStruct(structKind);

  auto ptrLE = mallocKnownSize(globalState, functionState, builder, countedStructL);

  WrapperPtrLE newStructWrapperPtrLE =
      kindStructs.makeWrapperPtr(
          FL(), functionState, builder, desiredReference,
          ptrLE);
  fillControlBlock(
      FL(), functionState, builder, desiredValueType,
      kindStructs.getConcreteControlBlockPtr(
          FL(), functionState, builder, newStructWrapperPtrLE),
      structM->name->name);
  auto structContentsPtrLT = kindStructs.getStructInnerStruct(structM->kind);
  auto structContentsPtrLE =
      kindStructs.getStructContentsPtr(builder, newStructWrapperPtrLE);
  fillInnerStruct(
      globalState,
      functionState,
      builder,
      structM,
      memberRefs,
      structContentsPtrLT,
      structContentsPtrLE);

  auto resultRef = toRef(globalState->getRegion(desiredValueType), desiredReference, newStructWrapperPtrLE.refLE);

  if (globalState->opt->census) {
    auto objIdLE =
        globalState->getRegion(desiredValueType)
            ->getCensusObjectId(FL(), functionState, builder, desiredReference, resultRef);
    buildFlare(
        FL(), globalState, functionState, builder,
        "Allocated object ", structM->name->name, " &", ptrToIntLE(globalState, builder, ptrLE),
        " obj id ", objIdLE, "\n");
  }

  return resultRef;
}

void Unsafe::alias(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceRef,
    Ref expr) {
  { assert(false); throw 1337; }
  // auto sourceRnd = sourceRef->kind;
  //
  // if (dynamic_cast<Int *>(sourceRnd) ||
  //     dynamic_cast<Bool *>(sourceRnd) ||
  //     dynamic_cast<Float *>(sourceRnd) ||
  //     dynamic_cast<Void *>(sourceRnd)) {
  //   // Do nothing for these, they're always inlined and copied.
  // } else if (dynamic_cast<InterfaceKind *>(sourceRnd) ||
  //     dynamic_cast<StructKind *>(sourceRnd) ||
  //     dynamic_cast<StaticSizedArrayT *>(sourceRnd) ||
  //     dynamic_cast<RuntimeSizedArrayT *>(sourceRnd) ||
  //     dynamic_cast<Str *>(sourceRnd)) {
  //   if (isValueType(sourceRef)) {
  //     // We might be loading a member as an own if we're destructuring.
  //     // Don't adjust the RC, since we're only moving it.
  //   } else if (dynamic_cast<BorrowRef*>(sourceRef) != nullptr) {
  //     // Do nothing, fast mode doesn't do stuff for borrow refs.
  //   } else if (dynamic_cast<WeakRef*>(sourceRef) != nullptr) {
  //     aliasWeakRef(from, functionState, builder, sourceRef, expr);
  //   } else if (dynamic_cast<ShareRef*>(sourceRef) != nullptr) {
  //     if (sourceRef->location == Location::INLINE) {
  //       // Do nothing, we can just let inline structs disappear
  //     } else {
  //       adjustStrongRc(from, globalState, functionState, &kindStructs, builder, expr, sourceRef, 1);
  //     }
  //   } else
  //     { assert(false); throw 1337; }
  // } else {
  //   std::cerr << "Unimplemented type in acquireReference: "
  //       << typeid(*sourceRef->kind).name() << std::endl;
  //   { assert(false); throw 1337; }
  // }
}

void Unsafe::dealias(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceMT,
    Ref sourceRef) {
  if (dynamic_cast<ShareRef*>(sourceMT) != nullptr) {
    { assert(false); throw 1337; }
  } else {
    if (isValueType(sourceMT)) {
      // This can happen if we're sending an owning reference to the outside world, see DEPAR.
    } else if (dynamic_cast<BorrowRef*>(sourceMT) != nullptr) {
      // Do nothing!
    } else if (dynamic_cast<WeakRef*>(sourceMT) != nullptr) {
      discardWeakRef(from, functionState, builder, sourceMT, sourceRef);
    } else { assert(false); throw 1337; }
  }
}

Ref Unsafe::weakAlias(FunctionState* functionState, LLVMBuilderRef builder, Kind* sourceRefMT, Kind* targetRefMT, Ref sourceRef) {
  { assert(false); throw 1337; }
//  return regularWeakAlias(globalState, functionState, &kindStructs, &wrcWeaks, builder, sourceRefMT, targetRefMT, sourceRef);
}

// Doesn't return a constraint ref, returns a raw ref to the wrapper struct.
WrapperPtrLE Unsafe::lockWeakRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refM,
    Ref weakRefLE) {
  { assert(false); throw 1337; }
//  switch (refM->ownership) {
//    case Ownership::OWN:
//    case Ownership::MUTABLE_SHARE:
//    case Ownership::IMMUTABLE_SHARE:
//    case Ownership::MUTABLE_BORROW:
//    case Ownership::IMMUTABLE_BORROW:
//      { assert(false); throw 1337; }
//      break;
//    case Ownership::WEAK: {
//      auto weakFatPtrLE =
//          kindStructs.makeWeakFatPtr(
//              refM,
//              checkValidReference(FL(), functionState, builder, false, refM, weakRefLE));
//      return kindStructs.makeWrapperPtr(
//          FL(), functionState, builder, refM,
//          wrcWeaks.lockWrciFatPtr(from, functionState, builder, refM, weakFatPtrLE));
//    }
//    default:
//      { assert(false); throw 1337; }
//      break;
//  }
}

Ref Unsafe::lockWeak(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool thenResultIsNever,
    bool elseResultIsNever,
    Kind* resultOptTypeM,
    Kind* constraintRefM,
    Kind* sourceWeakRefMT,
    Ref sourceWeakRefLE,
    std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
    std::function<Ref(LLVMBuilderRef)> buildElse) {

  assert(dynamic_cast<WeakRef*>(sourceWeakRefMT) != nullptr);
  auto isAliveLE =
      getIsAliveFromWeakRef(
          functionState, builder, sourceWeakRefMT, sourceWeakRefLE);
  auto resultOptTypeLE = globalState->getRegion(resultOptTypeM)->translateType(resultOptTypeM);
  return regularInnerLockWeak(
      globalState, functionState, builder, thenResultIsNever, elseResultIsNever, resultOptTypeM,
      constraintRefM, sourceWeakRefMT, sourceWeakRefLE, buildThen, buildElse,
      isAliveLE, resultOptTypeLE, &kindStructs, &fatWeaks);
}


Ref Unsafe::asSubtype(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* resultOptTypeM,
    Kind* sourceInterfaceRefMT,
    Ref sourceInterfaceRef,
    Kind* targetKind,
    std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
    std::function<Ref(LLVMBuilderRef)> buildElse) {

  return regularDowncast(
      globalState, functionState, builder, &kindStructs, resultOptTypeM,
      sourceInterfaceRefMT, sourceInterfaceRef, targetKind, buildThen, buildElse);
}

LLVMTypeRef Unsafe::translateType(Kind* referenceM) {
  return translateReferenceSimple(globalState, &kindStructs, peel_all_references(referenceM));
}

Ref Unsafe::upcastWeak(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    WeakFatPtrLE sourceRefLE,
    StructKind* sourceStructKindM,
    Kind* sourceStructTypeM,
    InterfaceKind* targetInterfaceKindM,
    Kind* targetInterfaceTypeM) {
  { assert(false); throw 1337; }
//  auto resultWeakInterfaceFatPtr =
//      wrcWeaks.weakStructPtrToWrciWeakInterfacePtr(
//          globalState, functionState, builder, sourceRefLE, sourceStructKindM,
//          sourceStructTypeM, targetInterfaceKindM, targetInterfaceTypeM);
//  return toRef(this, targetInterfaceTypeM, resultWeakInterfaceFatPtr);
}

void Unsafe::declareStaticSizedArray(
    StaticSizedArrayDefinitionT* staticSizedArrayMT) {
  globalState->regionIdByKind.emplace(staticSizedArrayMT->kind, getRegionId());

  kindStructs.declareStaticSizedArray(staticSizedArrayMT->kind, Weakability::NON_WEAKABLE);
}

void Unsafe::declareRuntimeSizedArray(
    RuntimeSizedArrayDefinitionT* runtimeSizedArrayMT) {
  globalState->regionIdByKind.emplace(runtimeSizedArrayMT->kind, getRegionId());

  kindStructs.declareRuntimeSizedArray(runtimeSizedArrayMT->kind, Weakability::NON_WEAKABLE);
}

void Unsafe::defineRuntimeSizedArray(
    RuntimeSizedArrayDefinitionT* runtimeSizedArrayMT) {
  auto elementLT =
      globalState->getRegion(runtimeSizedArrayMT->elementType)
          ->translateType(runtimeSizedArrayMT->elementType);
  kindStructs.defineRuntimeSizedArray(runtimeSizedArrayMT, elementLT, true);
}

void Unsafe::defineStaticSizedArray(
    StaticSizedArrayDefinitionT* staticSizedArrayMT) {
  auto elementLT =
      globalState->getRegion(staticSizedArrayMT->elementType)
          ->translateType(staticSizedArrayMT->elementType);
  kindStructs.defineStaticSizedArray(staticSizedArrayMT, elementLT);
}

void Unsafe::declareStruct(
    StructDefinition* structM) {
  globalState->regionIdByKind.emplace(structM->kind, getRegionId());

  kindStructs.declareStruct(structM->kind, structM->weakability);
}

void Unsafe::defineStruct(
    StructDefinition* structM) {
  std::vector<LLVMTypeRef> innerStructMemberTypesL;
  for (int i = 0; i < structM->members.size(); i++) {
    innerStructMemberTypesL.push_back(
        globalState->getRegion(structM->members[i]->type)
            ->translateType(structM->members[i]->type));
  }
  kindStructs.defineStruct(structM->kind, innerStructMemberTypesL);
}

void Unsafe::declareEdge(
    Edge* edge) {
  kindStructs.declareEdge(edge);
}

void Unsafe::defineEdge(
    Edge* edge) {
  auto interfaceFunctionsLT = globalState->getInterfaceFunctionPointerTypes(edge->interfaceName);
  auto edgeFunctionsL = globalState->getEdgeFunctions(edge);
  kindStructs.defineEdge(edge, interfaceFunctionsLT, edgeFunctionsL);
}

void Unsafe::declareInterface(
    InterfaceDefinition* interfaceM) {
  globalState->regionIdByKind.emplace(interfaceM->kind, getRegionId());

  kindStructs.declareInterface(interfaceM->kind, interfaceM->weakability);
}

void Unsafe::defineInterface(
    InterfaceDefinition* interfaceM) {
  auto interfaceMethodTypesL = globalState->getInterfaceFunctionPointerTypes(interfaceM->kind);
  kindStructs.defineInterface(interfaceM, interfaceMethodTypesL);
}

void Unsafe::discardOwningRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    Kind* sourceMT,
    LiveRef sourceRef) {
  // Free it!
  deallocate(AFL("discardOwningRef"), functionState, builder, sourceMT, sourceRef);
}

void Unsafe::noteWeakableDestroyed(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refM,
    ControlBlockPtrLE controlBlockPtrLE) {
  { assert(false); throw 1337; }
//  // In fast mode, only shared things are strong RC'd
//  if (refM->ownership == Ownership::MUTABLE_SHARE || refM->ownership == Ownership::IMMUTABLE_SHARE) {
//    { assert(false); throw 1337; }
////    // Only shared stuff is RC'd in fast mode
////    auto rcIsZeroLE = strongRcIsZero(globalState, &kindStructs, builder, refM, controlBlockPtrLE);
////    buildAssertV(globalState, functionState, builder, rcIsZeroLE,
////        "Tried to free concrete that had nonzero RC!");
//  } else {
//    // It's a mutable, so mark WRCs dead
//
//    if (auto structKindM = dynamic_cast<StructKind *>(peel_all_references(refM))) {
//      auto structM = globalState->program->getStruct(structKindM);
//      if (structM->weakability == Weakability::WEAKABLE) {
//        wrcWeaks.innerNoteWeakableDestroyed(functionState, builder, refM, controlBlockPtrLE);
//      }
//    } else if (auto interfaceKindM = dynamic_cast<InterfaceKind *>(peel_all_references(refM))) {
//      auto interfaceM = globalState->program->getInterface(interfaceKindM);
//      if (interfaceM->weakability == Weakability::WEAKABLE) {
//        wrcWeaks.innerNoteWeakableDestroyed(functionState, builder, refM, controlBlockPtrLE);
//      }
//    } else {
//      // Do nothing, only structs and interfaces are weakable in assist mode.
//    }
//  }
}

void Unsafe::storeMember(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* structRefMT,
    LiveRef structRef,
    int memberIndex,
    const std::string& memberName,
    Kind* newMemberRefMT,
    Ref newMemberRef) {
  auto newMemberLE =
      globalState->getRegion(newMemberRefMT)->checkValidReference(
          FL(), functionState, builder, false, newMemberRefMT, newMemberRef);
  storeMemberStrong(
      globalState, functionState, builder, &kindStructs, structRefMT, structRef,
      memberIndex, memberName, newMemberLE);
}

// Gets the itable PTR and the new value that we should put into the virtual param's slot
// (such as a void* or a weak void ref)
std::tuple<LLVMValueRef, LLVMValueRef> Unsafe::explodeInterfaceRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* virtualParamMT,
    Ref virtualArgRef) {
      return explodeStrongInterfaceRef(
          globalState, functionState, builder, &kindStructs, virtualParamMT, virtualArgRef);
}

Ref Unsafe::getRuntimeSizedArrayLength(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* rsaRefMT,
    LiveRef arrayRef) {
  return getRuntimeSizedArrayLengthStrong(globalState, functionState, builder, &kindStructs, rsaRefMT, arrayRef);
}

Ref Unsafe::getRuntimeSizedArrayCapacity(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* rsaRefMT,
    LiveRef arrayRef) {
  return getRuntimeSizedArrayCapacityStrong(globalState, functionState, builder, &kindStructs, rsaRefMT, arrayRef);
}

LLVMValueRef Unsafe::checkValidReference(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool expectLive,
    Kind* refM,
    Ref ref) {
  Kind *actualRefM = nullptr;
  LLVMValueRef refLE = nullptr;
  std::tie(actualRefM, refLE) = megaGetRefInnardsForChecking(ref);
  assert(actualRefM == refM);
  assert(refLE != nullptr);
  assert(LLVMTypeOf(refLE) == translateType(refM));

  if (isValueType(refM)) {
    regularCheckValidReference(checkerAFL, globalState, functionState, builder, &kindStructs, refM, refLE);
  } else if (dynamic_cast<ShareRef*>(refM) != nullptr) {
    { assert(false); throw 1337; }
  } else {
    if (dynamic_cast<BorrowRef*>(refM) != nullptr) {
      regularCheckValidReference(checkerAFL, globalState, functionState, builder,
          &kindStructs, refM, refLE);
    } else if (dynamic_cast<WeakRef*>(refM) != nullptr) {
      { assert(false); throw 1337; }
//      wrcWeaks.buildCheckWeakRef(checkerAFL, functionState, builder, refM, ref);
    } else
      { assert(false); throw 1337; }
  }
  return refLE;
}

// TODO maybe combine with alias/acquireReference?
// After we load from a local, member, or element, we can feed the result through this
// function to turn it into a desired ownership.
// Example:
// - Can load from an owning ref member to get a constraint ref.
// - Can load from a constraint ref member to get a weak ref.
Ref Unsafe::upgradeLoadResultToRefWithTargetOwnership(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceType,
    Kind* targetType,
    LoadResult sourceLoadResult) {
  { assert(false); throw 1337; }
//   auto sourceRef = sourceLoadResult.extractForAliasingInternals();
//   auto sourceOwnership = sourceType->ownership;
//   auto sourceLocation = sourceType->location;
//   auto targetOwnership = targetType->ownership;
//   auto targetLocation = targetType->location;
// //  assert(sourceLocation == targetLocation); // unimplemented
//
//   if (sourceOwnership == Ownership::MUTABLE_SHARE || sourceOwnership == Ownership::IMMUTABLE_SHARE) {
//     if (sourceLocation == Location::INLINE) {
//       return sourceRef;
//     } else {
//       return sourceRef;
//     }
//   } else if (sourceOwnership == Ownership::OWN) {
//     if (targetOwnership == Ownership::OWN) {
//       // We can never "load" an owning ref from any of these:
//       // - We can only get owning refs from locals by unstackifying
//       // - We can only get owning refs from structs by destroying
//       // - We can only get owning refs from elements by destroying
//       // However, we CAN load owning refs by:
//       // - Swapping from a local
//       // - Swapping from an element
//       // - Swapping from a member
//       return sourceRef;
//     } else if (targetOwnership == Ownership::MUTABLE_BORROW || targetOwnership == Ownership::IMMUTABLE_BORROW) {
//       auto resultRef = transmutePtr(globalState, functionState, builder, false, sourceType, targetType, sourceRef);
//       checkValidReference(FL(), functionState, builder, false, targetType, resultRef);
//       return resultRef;
//     } else if (targetOwnership == Ownership::WEAK) {
//       { assert(false); throw 1337; }
// //      return wrcWeaks.assembleWeakRef(functionState, builder, sourceType, targetType, sourceRef);
//     } else {
//       { assert(false); throw 1337; }
//     }
//   } else if (sourceOwnership == Ownership::MUTABLE_BORROW || sourceOwnership == Ownership::IMMUTABLE_BORROW) {
//     buildFlare(FL(), globalState, functionState, builder);
//
//     if (targetOwnership == Ownership::OWN) {
//       { assert(false); throw 1337; } // Cant load an owning reference from a constraint ref local.
//     } else if (targetOwnership == Ownership::MUTABLE_BORROW || targetOwnership == Ownership::IMMUTABLE_BORROW) {
//       return sourceRef;
//     } else if (targetOwnership == Ownership::WEAK) {
//       // Making a weak ref from a constraint ref local.
//       assert(dynamic_cast<StructKind*>(peel_all_references(sourceType)) || dynamic_cast<InterfaceKind*>(peel_all_references(sourceType)));
//       { assert(false); throw 1337; }
// //      return wrcWeaks.assembleWeakRef(functionState, builder, sourceType, targetType, sourceRef);
//     } else {
//       { assert(false); throw 1337; }
//     }
//   } else if (sourceOwnership == Ownership::WEAK) {
//     assert(targetOwnership == Ownership::WEAK);
//     return sourceRef;
//   } else {
//     { assert(false); throw 1337; }
//   }
//   { assert(false); throw 1337; }
}

void Unsafe::aliasWeakRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* weakRefMT,
    Ref weakRef) {
  { assert(false); throw 1337; }
//  return wrcWeaks.aliasWeakRef(from, functionState, builder, weakRefMT, weakRef);
}

void Unsafe::discardWeakRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* weakRefMT,
    Ref weakRef) {
  { assert(false); throw 1337; }
//  return wrcWeaks.discardWeakRef(from, functionState, builder, weakRefMT, weakRef);
}

LLVMValueRef Unsafe::getCensusObjectId(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refM,
    Ref ref) {
  auto controlBlockPtrLE =
      kindStructs.getControlBlockPtr(checkerAFL, functionState, builder, ref, refM);
  return kindStructs.getObjIdFromControlBlockPtr(builder, peel_all_references(refM), controlBlockPtrLE);
}

Ref Unsafe::getIsAliveFromWeakRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* weakRefM,
    Ref weakRef) {
  { assert(false); throw 1337; }
//  return wrcWeaks.getIsAliveFromWeakRef(functionState, builder, weakRefM, weakRef);
}

// Returns object ID
void Unsafe::fillControlBlock(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    ValueKind* kindM,
    ControlBlockPtrLE controlBlockPtrLE,
    const std::string& typeName) {

  LLVMValueRef newControlBlockLE = LLVMGetUndef(kindStructs.getControlBlock(kindM)->getStruct());

  newControlBlockLE =
      fillControlBlockCensusFields(
          from, globalState, functionState, &kindStructs, builder, kindM, newControlBlockLE, typeName);

  if (globalState->getKindWeakability(kindM) == Weakability::WEAKABLE) {
    { assert(false); throw 1337; }
//    newControlBlockLE = wrcWeaks.fillWeakableControlBlock(functionState, builder, &kindStructs, kindM,
//        newControlBlockLE);
  }

  LLVMBuildStore(
      builder,
      newControlBlockLE,
      controlBlockPtrLE.refLE);
}

LoadResult Unsafe::loadElementFromSSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* ssaRefMT,
    StaticSizedArrayT* ssaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE) {
  auto ssaDef = globalState->program->getStaticSizedArray(ssaMT);
  return regularloadElementFromSSA(
      globalState, functionState, builder, ssaRefMT, ssaDef->elementType, arrayRef, indexInBoundsLE, &kindStructs);
}

LoadResult Unsafe::loadElementFromRSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* rsaRefMT,
    RuntimeSizedArrayT* rsaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE) {
  auto rsaDef = globalState->program->getRuntimeSizedArray(rsaMT);
  return regularLoadElementFromRSAWithoutUpgrade(
      globalState, functionState, builder, &kindStructs, true, rsaRefMT, rsaDef->elementType, arrayRef, indexInBoundsLE);
}

Ref Unsafe::storeElementInRSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* rsaRefMT,
    RuntimeSizedArrayT* rsaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE,
    Ref elementRef) {
  auto rsaDef = globalState->program->getRuntimeSizedArray(rsaMT);
  auto arrayWrapperPtrLE = toWrapperPtr(functionState, builder, &kindStructs, rsaRefMT, arrayRef);
  auto arrayElementsPtrLE = getRuntimeSizedArrayContentsPtr(builder, true, arrayWrapperPtrLE);
  buildFlare(FL(), globalState, functionState, builder);
  return ::swapElement(
      globalState, functionState, builder, rsaDef->elementType, arrayElementsPtrLE, indexInBoundsLE, elementRef);
}

Ref Unsafe::upcast(
    FunctionState* functionState,
    LLVMBuilderRef builder,

    Kind* sourceStructMT,
    StructKind* sourceStructKindM,
    Ref sourceRefLE,

    Kind* targetInterfaceTypeM,
    InterfaceKind* targetInterfaceKindM) {
  return upcastStrong(globalState, functionState, builder, &kindStructs, sourceStructMT, sourceStructKindM, sourceRefLE, targetInterfaceTypeM, targetInterfaceKindM);
}


void Unsafe::deallocate(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refMT,
    LiveRef ref) {
  innerDeallocate(from, globalState, functionState, &kindStructs, builder, refMT, ref);
}

LiveRef Unsafe::constructRuntimeSizedArray(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* rsaMT,
    RuntimeSizedArrayT* runtimeSizedArrayT,
    Ref capacityRef,
    const std::string& typeName) {
  auto rsaWrapperPtrLT =
      kindStructs.getRuntimeSizedArrayWrapperStruct(runtimeSizedArrayT);
  auto rsaDef = globalState->program->getRuntimeSizedArray(runtimeSizedArrayT);
  auto elementType = globalState->program->getRuntimeSizedArray(runtimeSizedArrayT)->elementType;
  auto rsaElementLT = globalState->getRegion(elementType)->translateType(elementType);
  auto resultRef =
      ::constructRuntimeSizedArray(
           globalState, functionState, builder, &kindStructs, rsaMT, rsaDef->elementType, runtimeSizedArrayT,
           rsaWrapperPtrLT, rsaElementLT, globalState->constI32(0), capacityRef, true, typeName,
          [this, functionState, runtimeSizedArrayT, rsaMT, typeName](
              LLVMBuilderRef innerBuilder, ControlBlockPtrLE controlBlockPtrLE) {
            fillControlBlock(
                FL(),
                functionState,
                innerBuilder,
                runtimeSizedArrayT,
                controlBlockPtrLE,
                typeName);
          });
  return resultRef;
}

Ref Unsafe::loadMember(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* structRefMT,
    LiveRef structRef,
    int memberIndex,
    Kind* expectedMemberType,
    Kind* targetType,
    const std::string& memberName) {

  if (dynamic_cast<ShareRef*>(structRefMT) != nullptr) {
    { assert(false); throw 1337; }
  } else {
    auto unupgradedMemberLE =
        regularLoadMember(
            globalState, functionState, builder, &kindStructs, structRefMT, structRef,
            memberIndex, expectedMemberType, targetType, memberName);
    return upgradeLoadResultToRefWithTargetOwnership(
        functionState, builder, expectedMemberType, targetType, unupgradedMemberLE);
  }
}

void Unsafe::checkInlineStructType(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refMT,
    Ref ref) {
  auto argLE = checkValidReference(FL(), functionState, builder, false, refMT, ref);
  auto structKind = dynamic_cast<StructKind*>(peel_all_references(refMT));
  assert(structKind);
  assert(LLVMTypeOf(argLE) == kindStructs.getStructInnerStruct(structKind));
}


std::string Unsafe::generateRuntimeSizedArrayDefsC(
    Package* currentPackage,
    RuntimeSizedArrayDefinitionT* rsaDefM) {
  return generateConcreteHandleStructDefC(currentPackage, currentPackage->getKindExportName(rsaDefM->kind, true));
}

std::string Unsafe::generateStaticSizedArrayDefsC(
    Package* currentPackage,
    StaticSizedArrayDefinitionT* ssaDefM) {
  return generateConcreteHandleStructDefC(currentPackage, currentPackage->getKindExportName(ssaDefM->kind, true));
}

std::string Unsafe::generateStructDefsC(
    Package* currentPackage, StructDefinition* structDefM) {
  assert(structDefM->sharedness == Sharedness::SINGLE);
  return generateConcreteHandleStructDefC(currentPackage, currentPackage->getKindExportName(structDefM->kind, true));
}

std::string Unsafe::generateInterfaceDefsC(
    Package* currentPackage, InterfaceDefinition* interfaceDefM) {
  assert(interfaceDefM->sharedness == Sharedness::SINGLE);
  return generateInterfaceHandleStructDefC(currentPackage, currentPackage->getKindExportName(interfaceDefM->kind, true));
}


LLVMTypeRef Unsafe::getExternalType(ValueKind* kind) {
  // Per @HTSLVBDTCZ, all concretes share one handle type and all interfaces
  // share one; kind distinctness lives in the C typedefs, not this type.
  // Same right-sized handle structs the share region uses: mut concretes cross
  // as 8-byte { i64 obj }, mut interfaces as 16-byte { i64 obj, i64 typeinfo }.
  if (dynamic_cast<StructKind*>(kind) ||
      dynamic_cast<StaticSizedArrayT*>(kind) ||
      dynamic_cast<RuntimeSizedArrayT*>(kind)) {
    return globalState->getFfiHandleStructs()->getConcreteHandleStructLT();
  } else if (dynamic_cast<InterfaceKind*>(kind)) {
    return globalState->getFfiHandleStructs()->getInterfaceHandleStructLT();
  } else if (dynamic_cast<Bool*>(kind)) {
    // Bool crosses the C boundary as i8 (see sendValeObjectIntoHost / receiveHostObjectIntoVale).
    return LLVMInt8TypeInContext(globalState->context);
  } else {
    // Other primitives (Int/Float/Void/Never) cross as their scalar C-ABI type.
    DefaultPrimitives primitives;
    return primitives.translatePrimitive(globalState, kind);
  }
}

Ref Unsafe::receiveAndDecryptFamiliarReference(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceRefMT,
    LLVMValueRef sourceRefLE) {
  assert(dynamic_cast<ShareRef*>(sourceRefMT) == nullptr);
  return regularReceiveAndDecryptFamiliarReference(
      globalState, functionState, builder, &kindStructs, sourceRefMT, sourceRefLE);
}

LLVMTypeRef Unsafe::getInterfaceMethodVirtualParamAnyType() {
  return LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0);
}

LLVMValueRef Unsafe::encryptAndSendFamiliarReference(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceRefMT,
    Ref sourceRef) {
  assert(dynamic_cast<ShareRef*>(sourceRefMT) == nullptr);
  assert(dynamic_cast<ShareRef*>(sourceRefMT) == nullptr);
  return regularEncryptAndSendFamiliarReference(
      globalState, functionState, builder, &kindStructs, sourceRefMT, sourceRef);
}

void Unsafe::pushRuntimeSizedArrayNoBoundsCheck(
    FunctionState *functionState,
    LLVMBuilderRef builder,
    Kind *rsaRefMT,
    RuntimeSizedArrayT *rsaMT,
    LiveRef rsaRef,
    InBoundsLE indexInBoundsLE,
    Ref elementRef) {
  auto arrayWrapperPtrLE =
      toWrapperPtr(functionState, builder, &kindStructs, rsaRefMT, rsaRef);
  auto incrementedSize =
      incrementRSASize(
          globalState, functionState, builder, arrayWrapperPtrLE);
  ::initializeElementInRSAWithoutIncrementSize(
      globalState, functionState, builder, true, rsaMT, arrayWrapperPtrLE, indexInBoundsLE,
      elementRef, incrementedSize);
}

Ref Unsafe::popRuntimeSizedArrayNoBoundsCheck(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* rsaRefMT,
    RuntimeSizedArrayT* rsaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE) {
  auto rsaDef = globalState->program->getRuntimeSizedArray(rsaMT);
  auto elementLE =
      regularLoadElementFromRSAWithoutUpgrade(
          globalState,
          functionState,
          builder,
          &kindStructs,
          true,
          rsaRefMT,
          rsaDef->elementType,
          arrayRef,
          indexInBoundsLE)
          .move();
  auto rsaWrapperPtrLE = toWrapperPtr(functionState, builder, &kindStructs, rsaRefMT, arrayRef);
  decrementRSASize(globalState, functionState, &kindStructs, builder, rsaWrapperPtrLE);
  return elementLE;
}

void Unsafe::initializeElementInSSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* ssaRefMT,
    StaticSizedArrayT* ssaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE,
    Ref elementRef) {
  auto ssaDef = globalState->program->getStaticSizedArray(ssaMT);
  auto arrayWrapperPtrLE = toWrapperPtr(functionState, builder, &kindStructs, ssaRefMT, arrayRef);
  auto sizeRef = globalState->constI32(ssaDef->size);
  auto arrayElementsPtrLE = getStaticSizedArrayContentsPtr(builder, arrayWrapperPtrLE);
  ::initializeElementWithoutIncrementSize(
      globalState, functionState, builder, ssaDef->elementType, arrayElementsPtrLE,
      indexInBoundsLE, elementRef,
      // Manually making an IncrementedSize because it's an SSA.
      IncrementedSize{});
}

Ref Unsafe::deinitializeElementFromSSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* ssaRefMT,
    StaticSizedArrayT* ssaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE) {
  { assert(false); throw 1337; }
  exit(1);
}

Weakability Unsafe::getKindWeakability(ValueKind* kind) {
  if (auto structKind = dynamic_cast<StructKind*>(kind)) {
    return globalState->lookupStruct(structKind)->weakability;
  } else if (auto interfaceKind = dynamic_cast<InterfaceKind*>(kind)) {
    return globalState->lookupInterface(interfaceKind)->weakability;
  } else {
    return Weakability::NON_WEAKABLE;
  }
}

ValeFuncPtrLE Unsafe::getInterfaceMethodFunctionPtr(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* virtualParamMT,
    Ref virtualArgRef,
    int indexInEdge) {
  return getInterfaceMethodFunctionPtrFromItable(
      globalState, functionState, builder, &kindStructs, virtualParamMT, virtualArgRef, indexInEdge);
}

LLVMValueRef Unsafe::stackify(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Local* local,
    Ref refToStore) {
  auto toStoreLE = checkValidReference(FL(), functionState, builder, false, local->type, refToStore);
  auto typeLT = translateType(local->type);
  return makeBackendLocal(functionState, builder, typeLT, local->name.c_str(), toStoreLE);
}

Ref Unsafe::unstackify(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr) {
  return loadLocal(functionState, builder, local, localAddr);
}

Ref Unsafe::loadLocal(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr) {
  return normalLocalLoad(globalState, functionState, builder, local, localAddr);
}

Ref Unsafe::localStore(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr, Ref refToStore) {
  return normalLocalStore(globalState, functionState, builder, local, localAddr, refToStore);
}

std::string Unsafe::getExportName(
    Package* package,
    ValueKind* kind,
    bool includeProjectName) {
  // Mirrors RCImm::getExportName: primitives get their raw C type names; concretes cross as
  // right-sized handle value-type typedefs (no `*` suffix). Placement is not part of the ABI name.
  // VCOORD: make sure this is in the right place and isnt duplicated
  if (auto innt = dynamic_cast<Int*>(kind)) {
    return std::string() + "int" + std::to_string(innt->bits) + "_t";
  } else if (dynamic_cast<Bool*>(kind)) {
    return "int8_t";
  } else if (dynamic_cast<Float*>(kind)) {
    return "double";
  } else if (dynamic_cast<Void*>(kind) || dynamic_cast<Never*>(kind)) {
    return "void";
  }
  return package->getKindExportName(kind, includeProjectName);
}

LiveRef Unsafe::checkRefLive(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refMT,
    Ref ref) {
  // The whole point of unsafe is to get around such notions of liveness, so just return a LiveRef.
  auto refLE = checkValidReference(FL(), functionState, builder, true, refMT, ref);
  return wrapToLiveRef(FL(), functionState, builder, refMT, refLE);
}

LiveRef Unsafe::wrapToLiveRef(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refMT,
    LLVMValueRef ref) {
  assert(translateType(refMT) == LLVMTypeOf(ref));
  return LiveRef(refMT, ref);
}

LiveRef Unsafe::preCheckBorrow(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refMT,
    Ref ref) {
  // The whole point of unsafe is to get around such notions of liveness, so just return a LiveRef.
  auto refLE = checkValidReference(FL(), functionState, builder, true, refMT, ref);
  return wrapToLiveRef(FL(), functionState, builder, refMT, refLE);
}
Ref Unsafe::mutabilify(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refMT,
    Ref ref,
    Kind* targetRefMT) {
  assert(dynamic_cast<BorrowRef*>(refMT) != nullptr);
  { assert(false); throw 1337; } // impl
}

LiveRef Unsafe::immutabilify(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* refMT,
    Ref ref,
    Kind* targetRefMT) {
  assert(dynamic_cast<BorrowRef*>(refMT) != nullptr);
  { assert(false); throw 1337; }
}
