#include "../../function/expressions/shared/shared.h"
#include "../../utils/counters.h"
#include "../../utils/branch.h"
#include "../common/controlblock.h"
#include "../common/heap.h"
#include "../../function/expressions/shared/string.h"
#include "../common/common.h"
#include <sstream>
#include "../../function/expressions/shared/elements.h"
#include "rcimm.h"
#include "../../translatetype.h"

enum FreeFunctionParameter {
  FREE_PARAM_REGION_INSTANCE_REF = 0,
  FREE_PARAM_OBJECT_REF = 1,
};

void fillControlBlock(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* structs,
    LLVMBuilderRef builder,
    Kind* kindM,
    ControlBlockPtrLE controlBlockPtrLE,
    const std::string& typeName) {
  LLVMValueRef newControlBlockLE = LLVMGetUndef(structs->getControlBlock(kindM)->getStruct());
  newControlBlockLE =
      fillControlBlockCensusFields(
          from, globalState, functionState, structs, builder, kindM, newControlBlockLE, typeName);
  newControlBlockLE = insertStrongRc(globalState, builder, structs, kindM, newControlBlockLE);
  LLVMBuildStore(builder, newControlBlockLE, controlBlockPtrLE.refLE);
}

ControlBlock makeImmControlBlock(GlobalState* globalState) {
  ControlBlock controlBlock(globalState, LLVMStructCreateNamed(globalState->context, "immControlBlock"));
  controlBlock.addMember(ControlBlockMember::STRONG_RC_32B);
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

RCImm::RCImm(GlobalState* globalState_)
  : globalState(globalState_),
    kindStructs(globalState, makeImmControlBlock(globalState), makeImmControlBlock(globalState), LLVMStructCreateNamed(globalState->context, "immUnused")),
    edgesByInterface(0, globalState_->addressNumberer->makeHasher<InterfaceKind*>()) {

  regionKind =
      globalState->metalCache->getStructKind(
          globalState->metalCache->getName(
              globalState->metalCache->builtinPackageCoord, namePrefix + "_Region"));
  regionRefMT =
      globalState->metalCache->getReference(
          Ownership::MUTABLE_BORROW, Location::YONDER, regionKind);
  globalState->regionIdByKind.emplace(regionKind, globalState->metalCache->rcImmRegionId);
  kindStructs.declareStruct(regionKind, Weakability::NON_WEAKABLE);
  kindStructs.defineStruct(regionKind, {
      // This region doesnt need anything
  });
}

Reference* RCImm::getRegionRefType() {
  return regionRefMT;
}

RegionId* RCImm::getRegionId() {
  return globalState->metalCache->rcImmRegionId;
}

Ref RCImm::makeRegionInstance(LLVMBuilderRef builder) {
  return toRef(this, regionRefMT, LLVMConstNull(translateType(regionRefMT)));
}

void RCImm::alias(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* sourceRef,
    Ref ref) {
  auto sourceRnd = sourceRef->kind;

  if (dynamic_cast<Int *>(sourceRnd) ||
      dynamic_cast<Bool *>(sourceRnd) ||
      dynamic_cast<Float *>(sourceRnd) ||
      dynamic_cast<Void *>(sourceRnd)) {
    // Do nothing for these, they're always inlined and copied.
  } else if (dynamic_cast<InterfaceKind *>(sourceRnd) ||
             dynamic_cast<StructKind *>(sourceRnd) ||
             dynamic_cast<StaticSizedArrayT *>(sourceRnd) ||
             dynamic_cast<RuntimeSizedArrayT *>(sourceRnd) ||
             dynamic_cast<Str *>(sourceRnd)) {
    if (sourceRef->location == Location::INLINE) {
      // Do nothing, we can just let inline structs disappear
    } else {
      if (sourceRef->ownership == Ownership::IMMUTABLE_SHARE) {
        // Do nothing, immutable yonders need no RC adjustments.
      } else if (sourceRef->ownership == Ownership::MUTABLE_SHARE) {
        adjustStrongRc(from, globalState, functionState, &kindStructs, builder, ref, sourceRef, 1);
      } else {
        { assert(false); throw 1337; }
      }
    }
  } else {
    std::cerr << "Unimplemented type in acquireReference: "
              << typeid(*sourceRef->kind).name() << std::endl;
    { assert(false); throw 1337; }
  }
}

void RCImm::dealias(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* sourceMT,
    Ref sourceRef) {
  buildFlare(FL(), globalState, functionState, builder);
  discard(from, globalState, functionState, builder, sourceMT, sourceRef);
}

Ref RCImm::lockWeak(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool thenResultIsNever,
    bool elseResultIsNever,
    Reference* resultOptTypeM,
//      LLVMTypeRef resultOptTypeL,
    Reference* constraintRefM,
    Reference* sourceWeakRefMT,
    Ref sourceWeakRefLE,
    bool weakRefKnownLive,
    std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
    std::function<Ref(LLVMBuilderRef)> buildElse) {
  { assert(false); throw 1337; }
  exit(1);
}


Ref RCImm::asSubtype(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* resultOptTypeM,
    Reference* sourceInterfaceRefMT,
    Ref sourceInterfaceRef,
    bool sourceRefKnownLive,
    Kind* targetKind,
    std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
    std::function<Ref(LLVMBuilderRef)> buildElse) {
  return regularDowncast(
      globalState, functionState, builder, &kindStructs, resultOptTypeM,
      sourceInterfaceRefMT, sourceInterfaceRef, sourceRefKnownLive, targetKind, buildThen, buildElse);
}

LLVMValueRef RCImm::getCensusObjectId(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* refM,
    Ref ref) {
  if (refM->location == Location::INLINE) {
    return constI64LE(globalState, -1);
  } else if (refM == globalState->metalCache->i32Ref) {
    return constI64LE(globalState, -2);
  } else if (refM == globalState->metalCache->i64Ref) {
    return constI64LE(globalState, -3);
  } else if (refM == globalState->metalCache->boolRef) {
    return constI64LE(globalState, -4);
  } else if (refM == globalState->metalCache->neverRef) {
    return constI64LE(globalState, -5);
  } else if (refM == globalState->metalCache->floatRef) {
    return constI64LE(globalState, -6);
  } else {
    auto controlBlockPtrLE =
        kindStructs.getControlBlockPtr(checkerAFL, functionState, builder, ref, refM);
    auto exprLE =
        kindStructs.getObjIdFromControlBlockPtr(builder, refM->kind, controlBlockPtrLE);
    return exprLE;
  }
}

Ref RCImm::upcastWeak(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    WeakFatPtrLE sourceRefLE,
    StructKind* sourceStructKindM,
    Reference* sourceStructTypeM,
    InterfaceKind* targetInterfaceKindM,
    Reference* targetInterfaceTypeM) {
  { assert(false); throw 1337; }
  exit(1);
}

void RCImm::declareStruct(
    StructDefinition* structM) {
  globalState->regionIdByKind.emplace(structM->kind, getRegionId());

  kindStructs.declareStruct(structM->kind, structM->weakability);
}

void RCImm::declareStructExtraFunctions(StructDefinition* structDefM) {
  declareConcreteFreeFunction(structDefM->kind);
  declareConcreteAliasFunction(structDefM->kind);
  declareConcreteDealiasFunction(structDefM->kind);
  declareConcreteRefEqFunction(structDefM->kind);
  for (int i = 0; i < (int)structDefM->members.size(); i++) {
    declareConcreteFieldGetter(structDefM, i);
  }
  declareConcreteStructNewFunction(structDefM);
}

void RCImm::defineStruct(
    StructDefinition* structM) {
  std::vector<LLVMTypeRef> innerStructMemberTypesL;
  for (int i = 0; i < structM->members.size(); i++) {
    innerStructMemberTypesL.push_back(
        globalState->getRegion(structM->members[i]->type)
            ->translateType(structM->members[i]->type));
  }
  kindStructs.defineStruct(structM->kind, innerStructMemberTypesL);
}

void RCImm::defineStructExtraFunctions(StructDefinition* structDefM) {
  defineConcreteFreeFunction(structDefM->kind);
  defineConcreteAliasFunction(structDefM->kind);
  defineConcreteDealiasFunction(structDefM->kind);
  defineConcreteRefEqFunction(structDefM->kind);
  for (int i = 0; i < (int)structDefM->members.size(); i++) {
    defineConcreteFieldGetter(structDefM, i);
  }
  defineConcreteStructNewFunction(structDefM);
}

void RCImm::declareStaticSizedArray(
    StaticSizedArrayDefinitionT* ssaDefM) {
  globalState->regionIdByKind.emplace(ssaDefM->kind, getRegionId());

  kindStructs.declareStaticSizedArray(ssaDefM->kind, Weakability::NON_WEAKABLE);
}

void RCImm::declareStaticSizedArrayExtraFunctions(StaticSizedArrayDefinitionT* ssaDef) {
  declareConcreteFreeFunction(ssaDef->kind);
  declareConcreteAliasFunction(ssaDef->kind);
  declareConcreteDealiasFunction(ssaDef->kind);
  declareConcreteRefEqFunction(ssaDef->kind);
  declareConcreteSsaLenFunction(ssaDef->kind);
  declareConcreteSsaAtFunction(ssaDef->kind);
  declareConcreteSsaNewFunction(ssaDef);
}

void RCImm::defineStaticSizedArray(
    StaticSizedArrayDefinitionT* staticSizedArrayMT) {
  auto elementLT =
      translateType(
          staticSizedArrayMT->elementType);
  kindStructs.defineStaticSizedArray(staticSizedArrayMT, elementLT);
}

void RCImm::defineStaticSizedArrayExtraFunctions(StaticSizedArrayDefinitionT* ssaDef) {
  defineConcreteFreeFunction(ssaDef->kind);
  defineConcreteAliasFunction(ssaDef->kind);
  defineConcreteDealiasFunction(ssaDef->kind);
  defineConcreteRefEqFunction(ssaDef->kind);
  defineConcreteSsaLenFunction(ssaDef);
  defineConcreteSsaAtFunction(ssaDef);
  defineConcreteSsaNewFunction(ssaDef);
}

void RCImm::declareRuntimeSizedArray(
    RuntimeSizedArrayDefinitionT* rsaDefM) {
  globalState->regionIdByKind.emplace(rsaDefM->kind, getRegionId());

  kindStructs.declareRuntimeSizedArray(rsaDefM->kind, Weakability::NON_WEAKABLE);
}

void RCImm::declareRuntimeSizedArrayExtraFunctions(RuntimeSizedArrayDefinitionT* rsaDefM) {
  declareConcreteFreeFunction(rsaDefM->kind);
  declareConcreteAliasFunction(rsaDefM->kind);
  declareConcreteDealiasFunction(rsaDefM->kind);
  declareConcreteRefEqFunction(rsaDefM->kind);
  declareConcreteRsaLenFunction(rsaDefM->kind);
  declareConcreteRsaAtFunction(rsaDefM->kind);
}

void RCImm::defineRuntimeSizedArray(
    RuntimeSizedArrayDefinitionT* runtimeSizedArrayMT) {
  auto elementLT =
      translateType(
          runtimeSizedArrayMT->elementType);
  kindStructs.defineRuntimeSizedArray(runtimeSizedArrayMT, elementLT, false);
}

void RCImm::defineRuntimeSizedArrayExtraFunctions(RuntimeSizedArrayDefinitionT* rsaDefM) {
  defineConcreteFreeFunction(rsaDefM->kind);
  defineConcreteAliasFunction(rsaDefM->kind);
  defineConcreteDealiasFunction(rsaDefM->kind);
  defineConcreteRefEqFunction(rsaDefM->kind);
  defineConcreteRsaLenFunction(rsaDefM);
  defineConcreteRsaAtFunction(rsaDefM);
}

void RCImm::declareInterface(
    InterfaceDefinition* interfaceM) {
  globalState->regionIdByKind.emplace(interfaceM->kind, getRegionId());

  kindStructs.declareInterface(interfaceM->kind, interfaceM->weakability);
}

void RCImm::declareInterfaceExtraFunctions(InterfaceDefinition* interfaceDefM) {
  declareInterfaceFreeFunction(interfaceDefM->kind);
  declareConcreteAliasFunction(interfaceDefM->kind);
  declareConcreteDealiasFunction(interfaceDefM->kind);
  declareConcreteRefEqFunction(interfaceDefM->kind);
  declareConcreteTypeTagFunction(interfaceDefM->kind);
}

void RCImm::defineInterface(
    InterfaceDefinition* interfaceM) {
  auto interfaceMethodTypesL = globalState->getInterfaceFunctionPointerTypes(interfaceM->kind);
  kindStructs.defineInterface(interfaceM, interfaceMethodTypesL);
}

void RCImm::defineInterfaceExtraFunctions(InterfaceDefinition* interfaceDefM) {
  defineConcreteAliasFunction(interfaceDefM->kind);
  defineConcreteDealiasFunction(interfaceDefM->kind);
  defineConcreteRefEqFunction(interfaceDefM->kind);
  defineConcreteTypeTagFunction(interfaceDefM->kind);
}

void RCImm::declareEdge(Edge* edge) {
  kindStructs.declareEdge(edge);

  auto interfaceFreeMethod = getFreeInterfaceMethod(edge->interfaceName);
  auto freeThunkPrototype = getFreeThunkPrototype(edge->structName, edge->interfaceName);
  globalState->addEdgeExtraMethod(edge->interfaceName, edge->structName, interfaceFreeMethod, freeThunkPrototype);
  auto freeNameL = globalState->freeName->name + "__" + edge->interfaceName->fullName->name + "__" + edge->structName->fullName->name;
  declareExtraFunction(globalState, freeThunkPrototype, freeNameL);

  // Track edge order so the typeTag body and generateInterfaceDefsC agree on
  // the tag value assigned to each substruct.
  edgesByInterface[edge->interfaceName].push_back(edge);

  declareConcreteAsSubstructFunction(edge);
  declareConcreteUpcastFunction(edge);
}

void RCImm::defineEdge(Edge* edge) {
  auto interfaceM = globalState->program->getInterface(edge->interfaceName);

  auto interfaceFunctionsLT = globalState->getInterfaceFunctionPointerTypes(edge->interfaceName);
  auto edgeFunctionsL = globalState->getEdgeFunctions(edge);
  kindStructs.defineEdge(edge, interfaceFunctionsLT, edgeFunctionsL);

  defineEdgeFreeFunction(edge);
  defineConcreteAsSubstructFunction(edge);
  defineConcreteUpcastFunction(edge);
}

const std::vector<Edge*>* RCImm::getEdgesForInterface(InterfaceKind* interfaceKind) {
  auto iter = edgesByInterface.find(interfaceKind);
  if (iter == edgesByInterface.end()) return nullptr;
  return &iter->second;
}

Ref RCImm::weakAlias(
    FunctionState* functionState, LLVMBuilderRef builder, Reference* sourceRefMT, Reference* targetRefMT, Ref sourceRef) {
  { assert(false); throw 1337; }
  exit(1);
}

void RCImm::discardOwningRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    Reference* sourceMT,
    LiveRef sourceRef) {
  { assert(false); throw 1337; }
}


void RCImm::noteWeakableDestroyed(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* refM,
    ControlBlockPtrLE controlBlockPtrLE) {
  // Do nothing
}

Ref RCImm::loadMember(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* structRefMT,
    LiveRef structRef,
    int memberIndex,
    Reference* expectedMemberType,
    Reference* targetMemberType,
    const std::string& memberName) {
  auto memberLE =
      loadMember2(
          functionState, builder, regionInstanceRef, structRefMT, structRef, memberIndex, expectedMemberType,
          targetMemberType, memberName);
  auto resultRef =
      upgradeLoadResultToRefWithTargetOwnership(
          functionState, builder, regionInstanceRef, expectedMemberType, targetMemberType, memberLE, false);
  return resultRef;
}

void RCImm::storeMember(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* structRefMT,
    LiveRef structRef,
    int memberIndex,
    const std::string& memberName,
    Reference* newMemberRefMT,
    Ref newMemberRef) {
  { assert(false); throw 1337; }
}

std::tuple<LLVMValueRef, LLVMValueRef> RCImm::explodeInterfaceRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* virtualParamMT,
    Ref virtualArgRef) {
  return explodeStrongInterfaceRef(
      globalState, functionState, builder, &kindStructs, virtualParamMT, virtualArgRef);
}


void RCImm::aliasWeakRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* weakRefMT,
    Ref weakRef) {
  { assert(false); throw 1337; }
}

void RCImm::discardWeakRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* weakRefMT,
    Ref weakRef) {
  { assert(false); throw 1337; }
}

Ref RCImm::getIsAliveFromWeakRef(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* weakRefM,
    Ref weakRef,
    bool knownLive) {
  { assert(false); throw 1337; }
  exit(1);
}

LLVMValueRef RCImm::getStringBytesPtr(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* refMT,
    Ref regionInstanceRef,
    LiveRef ref) {
  assert(refMT->kind == globalState->metalCache->str);
  auto strWrapperPtrLE =
      toWrapperPtr(functionState, builder, &kindStructs, refMT, ref);
  return kindStructs.getStringBytesPtr(functionState, builder, strWrapperPtrLE);
}

Ref RCImm::allocate(
    Ref regionInstanceRef,
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* desiredReference,
    const std::vector<Ref>& memberRefs) {
  auto structKind = dynamic_cast<StructKind*>(desiredReference->kind);
  auto structM = globalState->program->getStruct(structKind);
  auto resultRef =
      innerAllocate(
          FL(), globalState, functionState, builder, desiredReference, &kindStructs, memberRefs, Weakability::WEAKABLE,
          [this, functionState, desiredReference, structM](LLVMBuilderRef innerBuilder, ControlBlockPtrLE controlBlockPtrLE) {
            fillControlBlock(
                FL(), globalState, functionState, &kindStructs, innerBuilder, desiredReference->kind,
                controlBlockPtrLE, structM->name->name);
          });
  // Dont need to alias here because the RC starts at 1, see SRCAO
  return resultRef;
}

Ref RCImm::upcast(
    FunctionState* functionState,
    LLVMBuilderRef builder,

    Reference* sourceStructMT,
    StructKind* sourceStructKindM,
    Ref sourceRefLE,

    Reference* targetInterfaceTypeM,
    InterfaceKind* targetInterfaceKindM) {
  return upcastStrong(globalState, functionState, builder, &kindStructs, sourceStructMT, sourceStructKindM, sourceRefLE, targetInterfaceTypeM, targetInterfaceKindM);
}

WrapperPtrLE RCImm::lockWeakRef(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* refM,
    Ref weakRefLE,
    bool weakRefKnownLive) {
  { assert(false); throw 1337; }
  exit(1);
}

LiveRef RCImm::constructStaticSizedArray(
    Ref regionInstanceRef,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* referenceM,
    StaticSizedArrayT* kindM) {
  auto resultRef =
      ::constructStaticSizedArray(
          globalState, functionState, builder, referenceM, kindM, &kindStructs,
          [this, functionState, referenceM, kindM](LLVMBuilderRef innerBuilder, ControlBlockPtrLE controlBlockPtrLE) {
//            fillControlBlock(
//                FL(),
//                functionState,
//                innerBuilder,
//                referenceM->kind,
//                kindM->mutability,
//                controlBlockPtrLE,
//                kindM->name->name);
            fillControlBlock(
                FL(), globalState, functionState, &kindStructs, innerBuilder, kindM, controlBlockPtrLE,
                kindM->name->name);
          });
  // Dont need to alias here because the RC starts at 1, see SRCAO
  return resultRef;
}

Ref RCImm::getRuntimeSizedArrayLength(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* rsaRefMT,
    LiveRef arrayRef) {
  return getRuntimeSizedArrayLengthStrong(globalState, functionState, builder, &kindStructs, rsaRefMT, arrayRef);
}

Ref RCImm::getRuntimeSizedArrayCapacity(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* rsaRefMT,
    LiveRef arrayRef) {
  return getRuntimeSizedArrayCapacityStrong(globalState, functionState, builder, &kindStructs, rsaRefMT, arrayRef);
}

LLVMValueRef RCImm::checkValidReference(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool expectLive,
    Reference* refM,
    Ref ref) {
    //buildFlare(FL(), globalState, functionState, builder);
  Reference *actualRefM = nullptr;
  LLVMValueRef refLE = nullptr;
  //buildFlare(FL(), globalState, functionState, builder);
  std::tie(actualRefM, refLE) = megaGetRefInnardsForChecking(ref);
  assert(actualRefM == refM);
  assert(refLE != nullptr);
  //buildFlare(FL(), globalState, functionState, builder);
  assert(LLVMTypeOf(refLE) == globalState->getRegion(refM)->translateType(refM));

  if (globalState->opt->census) {
    checkValidReference(checkerAFL, functionState, builder, &kindStructs, refM, refLE);
  }
  return refLE;
}

Ref RCImm::upgradeLoadResultToRefWithTargetOwnership(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* sourceType,
    Reference* targetType,
    LoadResult sourceLoad,
    bool resultKnownLive) {
  auto sourceRef = sourceLoad.extractForAliasingInternals();
  auto sourceOwnership = sourceType->ownership;
  auto sourceLocation = sourceType->location;
  auto targetOwnership = targetType->ownership;
  auto targetLocation = targetType->location;
//  assert(sourceLocation == targetLocation); // unimplemented

  if (sourceLocation == Location::INLINE) {
    return sourceRef;
  } else {
    return transmutePtr(globalState, functionState, builder, true, sourceType, targetType, sourceRef);
  }
}

void RCImm::checkInlineStructType(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* refMT,
    Ref ref) {
  auto argLE = checkValidReference(FL(), functionState, builder, false, refMT, ref);
  auto structKind = dynamic_cast<StructKind*>(refMT->kind);
  assert(structKind);
  assert(LLVMTypeOf(argLE) == kindStructs.getStructInnerStruct(structKind));
}

LoadResult RCImm::loadElementFromSSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* ssaRefMT,
    StaticSizedArrayT* ssaMT,
    LiveRef arrayRef,
    InBoundsLE indexLE) {
  auto ssaDef = globalState->program->getStaticSizedArray(ssaMT);
  return regularloadElementFromSSA(
      globalState, functionState, builder, ssaRefMT, ssaDef->elementType, arrayRef, indexLE, &kindStructs);
}

LoadResult RCImm::loadElementFromRSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* rsaRefMT,
    RuntimeSizedArrayT* rsaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE) {
  auto rsaDef = globalState->program->getRuntimeSizedArray(rsaMT);
  return regularLoadElementFromRSAWithoutUpgrade(
      globalState,
      functionState,
      builder,
      &kindStructs,
      false,
      rsaRefMT,
      rsaDef->elementType,
      arrayRef,
      indexInBoundsLE);
}

Ref RCImm::popRuntimeSizedArrayNoBoundsCheck(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref arrayRegionInstanceRef,
    Reference* rsaRefMT,
    RuntimeSizedArrayT* rsaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE) {
  auto rsaDef = globalState->program->getRuntimeSizedArray(rsaMT);
  return regularLoadElementFromRSAWithoutUpgrade(
      globalState, functionState, builder, &kindStructs, false, rsaRefMT, rsaDef->elementType, arrayRef,
      indexInBoundsLE).move();
}


Ref RCImm::storeElementInRSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* rsaRefMT,
    RuntimeSizedArrayT* rsaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE,
    Ref elementRef) {
  { assert(false); throw 1337; }
  exit(1);
}

void RCImm::pushRuntimeSizedArrayNoBoundsCheck(
    FunctionState *functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference *rsaRefMT,
    RuntimeSizedArrayT *rsaMT,
    LiveRef rsaRef,
    InBoundsLE indexInBoundsLE,
    Ref elementRef) {
  auto elementType = globalState->program->getRuntimeSizedArray(rsaMT)->elementType;
  buildFlare(FL(), globalState, functionState, builder);

  auto arrayWrapperPtrLE = toWrapperPtr(functionState, builder, &kindStructs, rsaRefMT, rsaRef);
  auto arrayElementsPtrLE = getRuntimeSizedArrayContentsPtr(builder, false, arrayWrapperPtrLE);

  // We don't increment the size because it's populated when we first create the array.
//  auto incrementedSize =
//      incrementRSASize(globalState, functionState, builder, rsaRefMT, arrayWrapperPtrLE);

  ::initializeElementWithoutIncrementSize(
      globalState, functionState, builder, rsaRefMT->location,
      elementType, arrayElementsPtrLE, indexInBoundsLE, elementRef,
      // We dont need to increment the size, so manually create this reminder object
      IncrementedSize{});
}

void RCImm::deallocate(
    AreaAndFileAndLine from,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* refMT,
    LiveRef ref) {
  buildFlare(FL(), globalState, functionState, builder);
  { assert(false); throw 1337; } // Outside shouldnt be able to deallocate anything of ours.
  // We deallocate things ourselves when we discard references, via discard.
  // We call innerDeallocate directly.
}


LiveRef RCImm::constructRuntimeSizedArray(
    Ref regionInstanceRef,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* rsaMT,
    RuntimeSizedArrayT* runtimeSizedArrayT,
    Ref capacityRef,
    const std::string& typeName) {
  auto rsaWrapperPtrLT =
      kindStructs.getRuntimeSizedArrayWrapperStruct(runtimeSizedArrayT);
  auto rsaDef = globalState->program->getRuntimeSizedArray(runtimeSizedArrayT);
  auto elementType = globalState->program->getRuntimeSizedArray(runtimeSizedArrayT)->elementType;
  auto rsaElementLT = globalState->getRegion(elementType)->translateType(elementType);
  buildFlare(FL(), globalState, functionState, builder);
  auto resultRef =
      ::constructRuntimeSizedArray(
          globalState, functionState, builder, &kindStructs, rsaMT, rsaDef->elementType, runtimeSizedArrayT,
          rsaWrapperPtrLT, rsaElementLT,
          // Note we're handing in capacity for the size ref. Because of this, we dont later increment the size
          // when we push elements.
          capacityRef, capacityRef,
          false, typeName,
          [this, functionState, runtimeSizedArrayT, rsaMT, typeName](
              LLVMBuilderRef innerBuilder, ControlBlockPtrLE controlBlockPtrLE) {
            fillControlBlock(
                FL(), globalState, functionState, &kindStructs, innerBuilder, runtimeSizedArrayT, controlBlockPtrLE,
                typeName);
          });
  buildFlare(FL(), globalState, functionState, builder);
  // Dont need to alias here because the RC starts at 1, see SRCAO
  return resultRef;
}


Ref RCImm::mallocStr(
    Ref regionInstanceRef,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef lengthLE,
    LLVMValueRef sourceCharsPtrLE) {
  auto resultRef =
      toRef(this, globalState->metalCache->mutStrRef, ::mallocStr(
          globalState, functionState, builder, lengthLE, sourceCharsPtrLE, &kindStructs,
          [this, functionState](LLVMBuilderRef innerBuilder, ControlBlockPtrLE controlBlockPtrLE) {
//            fillControlBlock(
//                FL(), functionState, innerBuilder, globalState->metalCache->str,
//                Mutability::IMMUTABLE, controlBlockPtrLE, "Str");
            fillControlBlock(
                FL(), globalState, functionState, &kindStructs, innerBuilder, globalState->metalCache->str, controlBlockPtrLE,
                "str");
          }));
  // Dont need to alias here because the RC starts at 1, see SRCAO
  return resultRef;
}

LLVMValueRef RCImm::getStringLen(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* refMT,
    Ref regionInstanceRef,
    LiveRef ref) {
  assert(refMT->kind == globalState->metalCache->str);
  auto strWrapperPtrLE = toWrapperPtr(functionState, builder, &kindStructs, refMT, ref);
  return kindStructs.getStringLen(functionState, builder, strWrapperPtrLE);
}

void RCImm::discard(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* sourceMT,
    Ref sourceRef) {
  buildFlare(FL(), globalState, functionState, builder);
  auto sourceRnd = sourceMT->kind;

  buildFlare(FL(), globalState, functionState, builder, typeid(*sourceRnd).name());

  if (dynamic_cast<Int *>(sourceRnd) ||
      dynamic_cast<Bool *>(sourceRnd) ||
      dynamic_cast<Float *>(sourceRnd)) {
    buildFlare(FL(), globalState, functionState, builder);
    // Do nothing for these, they're always inlined and copied.
  } else if (
      dynamic_cast<Str *>(sourceRnd) ||
      dynamic_cast<InterfaceKind *>(sourceRnd) ||
      dynamic_cast<StructKind *>(sourceRnd) ||
      dynamic_cast<StaticSizedArrayT *>(sourceRnd) ||
      dynamic_cast<RuntimeSizedArrayT *>(sourceRnd)) {
    buildFlare(FL(), globalState, functionState, builder);
    if (auto sr = dynamic_cast<StructKind *>(sourceRnd)) {
      buildFlare(FL(), globalState, functionState, builder, sr->fullName->name);
    }
    assert(sourceMT->ownership == Ownership::MUTABLE_SHARE || sourceMT->ownership == Ownership::IMMUTABLE_SHARE);
    if (sourceMT->location == Location::INLINE) {
      buildFlare(FL(), globalState, functionState, builder);
      // Do nothing, we can just let inline structs disappear
    } else {
      if (sourceMT->ownership == Ownership::IMMUTABLE_SHARE) {
        // Do nothing, immutable yonders need no RC adjustments.
      } else if (sourceMT->ownership == Ownership::MUTABLE_SHARE) {
        buildFlare(FL(), globalState, functionState, builder);
        auto rcLE =
            adjustStrongRc(
                from, globalState, functionState, &kindStructs, builder, sourceRef, sourceMT, -1);
        buildFlare(FL(), globalState, functionState, builder, rcLE);
        buildIfV(
            globalState, functionState,
            builder,
            isZeroLE(builder, rcLE),
            [this, from, globalState, functionState, sourceRef, sourceMT](
                LLVMBuilderRef thenBuilder) {
              buildFlare(FL(), globalState, functionState, thenBuilder);
              auto regionInstanceRef = makeRegionInstance(thenBuilder);
              callFree(functionState, thenBuilder, regionInstanceRef, sourceMT->kind, sourceRef);
              //  auto immDestructor = getFreePrototype(sourceMT->kind);
              ////      globalState->program->getImmDestructor(sourceMT->kind);
              //  auto funcL = globalState->getFunction(immDestructor);
              //
              //  auto sourceLE =
              //      globalState->getRegion(sourceMT)->checkValidReference(FL(),
              //          functionState, thenBuilder, true, sourceMT, sourceRef);
              //  std::vector<LLVMValueRef> argExprsL = {sourceLE};
              //  return unmigratedLLVMBuildCall(thenBuilder, funcL, argExprsL.data(), argExprsL.size(), "");
            });
      } else {
        { assert(false); throw 1337; }
      }
    }
  } else {
    std::cerr << "Unimplemented type in discard: "
        << typeid(*sourceMT->kind).name() << std::endl;
    { assert(false); throw 1337; }
  }
  buildFlare(FL(), globalState, functionState, builder);
}


LLVMTypeRef RCImm::translateType(Reference* referenceM) {
  if (primitives.isPrimitive(referenceM)) {
    return primitives.translatePrimitive(globalState, referenceM);
  } else if (referenceM == regionRefMT) {
    // We just have a raw pointer to region structs
    return LLVMPointerType(kindStructs.getStructInnerStruct(regionKind), 0);
  } else {
    if (dynamic_cast<Str *>(referenceM->kind) != nullptr) {
      assert(referenceM->location != Location::INLINE);
      assert(referenceM->ownership == Ownership::MUTABLE_SHARE || referenceM->ownership == Ownership::IMMUTABLE_SHARE);
      return LLVMPointerType(kindStructs.getStringWrapperStruct(), 0);
    } else if (auto staticSizedArrayMT = dynamic_cast<StaticSizedArrayT *>(referenceM->kind)) {
      assert(referenceM->location != Location::INLINE);
      auto staticSizedArrayCountedStructLT = kindStructs.getStaticSizedArrayWrapperStruct(staticSizedArrayMT);
      return LLVMPointerType(staticSizedArrayCountedStructLT, 0);
    } else if (auto runtimeSizedArrayMT =
        dynamic_cast<RuntimeSizedArrayT *>(referenceM->kind)) {
      assert(referenceM->location != Location::INLINE);
      auto runtimeSizedArrayCountedStructLT =
          kindStructs.getRuntimeSizedArrayWrapperStruct(runtimeSizedArrayMT);
      return LLVMPointerType(runtimeSizedArrayCountedStructLT, 0);
    } else if (auto structKind =
        dynamic_cast<StructKind *>(referenceM->kind)) {
      if (referenceM->location == Location::INLINE) {
        auto innerStructL = kindStructs.getStructInnerStruct(structKind);
        return innerStructL;
      } else {
        auto countedStructL = kindStructs.getStructWrapperStruct(structKind);
        return LLVMPointerType(countedStructL, 0);
      }
    } else if (auto interfaceKind =
        dynamic_cast<InterfaceKind *>(referenceM->kind)) {
      assert(referenceM->location != Location::INLINE);
      auto interfaceRefStructL =
          kindStructs.getInterfaceRefStruct(interfaceKind);
      return interfaceRefStructL;
    } else if (dynamic_cast<Never*>(referenceM->kind)) {
      auto result = LLVMPointerType(makeNeverType(globalState), 0);
      assert(LLVMTypeOf(globalState->neverPtrLE) == result);
      return result;
    } else {
      std::cerr << "Unimplemented type: " << typeid(*referenceM->kind).name() << std::endl;
      { assert(false); throw 1337; }
      return nullptr;
    }
  }
}


//LLVMTypeRef RCImm::getControlBlockStruct(Kind* kind) {
//  if (auto structKind = dynamic_cast<StructKind*>(kind)) {
//    auto structM = globalState->program->getStruct(structKind);
//    assert(structM->sharedness == Sharedness::SHARED);
//  } else if (auto interfaceKind = dynamic_cast<InterfaceKind*>(kind)) {
//    auto interfaceM = globalState->program->getInterface(interfaceKind);
//    assert(interfaceM->sharedness == Sharedness::SHARED);
//  } else if (auto ssaMT = dynamic_cast<StaticSizedArrayT*>(kind)) {
//    auto ssaDef = globalState->program->getStaticSizedArray(ssaMT);
//    assert(ssaDef->sharedness == Sharedness::SHARED);
//  } else if (auto rsaMT = dynamic_cast<RuntimeSizedArrayT*>(kind)) {
//    auto rsaDef = globalState->program->getRuntimeSizedArray(rsaMT);
//    assert(rsaDef->sharedness == Sharedness::SHARED);
//  } else if (auto strMT = dynamic_cast<Str*>(kind)) {
//  } else {
//    { assert(false); throw 1337; }
//  }
//  return kindStructs.getControlBlockStruct();
//}


LoadResult RCImm::loadMember2(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* structRefMT,
    LiveRef structLiveRef,
    int memberIndex,
    Reference* expectedMemberType,
    Reference* targetType,
    const std::string& memberName) {
  if (structRefMT->location == Location::INLINE) {
    auto structRef = toRef(globalState, structRefMT, structLiveRef);
    auto innerStructLE =
        globalState->getRegion(structRefMT)->checkValidReference(
            FL(), functionState, builder, false, structRefMT, structRef);
    auto memberLE =
        LLVMBuildExtractValue(builder, innerStructLE, memberIndex, memberName.c_str());
    return LoadResult{toRef(globalState->getRegion(expectedMemberType), expectedMemberType, memberLE)};
  } else {
    return regularLoadStrongMember(globalState, functionState, builder, &kindStructs, structRefMT, structLiveRef, memberIndex, expectedMemberType, targetType, memberName);
  }
}

void RCImm::checkValidReference(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Reference* refM,
    LLVMValueRef refLE) {
  if (refM == regionRefMT) {
    // Region ref has no control block.
  } else {
    regularCheckValidReference(checkerAFL, globalState, functionState, builder, kindStructs, refM, refLE);
  }
}

//std::string RCImm::getMemberArbitraryRefNameCSeeMMEDT(Reference* sourceMT) {
//  { assert(false); throw 1337; }
//  exit(1);
//}

// The opaque handle typedefs emitted for share-typed exports, each sized to
// exactly what its ref layer needs and matching the LLVM handle structs that
// cross the ABI boundary (see ffihandlestructs.h): concrete kinds get 8 bytes,
// interface kinds get 16.
// Per @HTSLVBDTCZ, this is where per-kind C distinctness comes from: each kind's
// export name becomes its own typedef, even though all concretes (and all
// interfaces) share one LLVM type internally.
static std::string emitConcreteHandleTypedefC(const std::string& name) {
  return
      std::string() + "typedef struct " + name + " { uint64_t _reserved; } " + name + ";\n";
}
static std::string emitInterfaceHandleTypedefC(const std::string& name) {
  return
      std::string() + "typedef struct " + name + " { uint64_t _reserved0; uint64_t _reserved1; } " + name + ";\n";
}

std::string RCImm::generateStructDefsC(
    Package* currentPackage,

    StructDefinition* structDefM) {
  auto name = currentPackage->getKindExportName(structDefM->kind, true);
  return emitConcreteHandleTypedefC(name);
}

std::string RCImm::generateInterfaceDefsC(
    Package* currentPackage, InterfaceDefinition* interfaceDefM) {
  auto name = currentPackage->getKindExportName(interfaceDefM->kind, true);
  std::stringstream s;
  // Emit the TAG_* constants first so users can `switch (typeTag(...))` over
  // them by name. Order matches the tag values returned by the typeTag body,
  // both of which follow the order in which edges were declared (SITTX).
  auto edges = getEdgesForInterface(interfaceDefM->kind);
  if (edges != nullptr) {
    for (int i = 0; i < (int)edges->size(); i++) {
      auto edge = (*edges)[i];
      s << "#define " << name << "_TAG_"
        << currentPackage->getKindExportName(edge->structName, false) << " " << i << "\n";
    }
  }
  s << emitInterfaceHandleTypedefC(name);
  return s.str();
}

std::string RCImm::generateRuntimeSizedArrayDefsC(
    Package* currentPackage,
    RuntimeSizedArrayDefinitionT* rsaDefM) {
  auto name = currentPackage->getKindExportName(rsaDefM->kind, true);
  return std::string() + "typedef struct " + name + " { void* unused; } " + name + ";\n";
}

std::string RCImm::generateStaticSizedArrayDefsC(
    Package* currentPackage,
    StaticSizedArrayDefinitionT* ssaDefM) {
  auto name = currentPackage->getKindExportName(ssaDefM->kind, true);
  return std::string() + "typedef struct " + name + " { void* unused; } " + name + ";\n";
}

LLVMTypeRef RCImm::getExternalType(Reference* refMT) {
  // Per @HTSLVBDTCZ, all concretes share one handle type and all interfaces
  // share one; kind distinctness lives in the C typedefs, not this type.
  // Under the opaque-handle FFI model, share refs cross the boundary as a
  // right-sized handle struct: concretes (struct/str/RSA/SSA) as 8 bytes,
  // interfaces as 16. The RC stays live during the extern call; C sees an
  // opaque handle.
  // The emitted handle typedefs (and the whole per-package C header ABI) are
  // pinned by the *_export_headers_golden tests in
  // FrontendRust/src/end_to_end_tests/tests/externs.rs.
  if (dynamic_cast<InterfaceKind*>(refMT->kind)) {
    return globalState->getFfiHandleStructs()->getInterfaceHandleStructLT();
  }
  if (dynamic_cast<StructKind*>(refMT->kind) ||
      dynamic_cast<StaticSizedArrayT*>(refMT->kind) ||
      dynamic_cast<RuntimeSizedArrayT*>(refMT->kind) ||
      dynamic_cast<Str*>(refMT->kind)) {
    return globalState->getFfiHandleStructs()->getConcreteHandleStructLT();
  }
  // Primitives (Int/Bool/Float/Void) and Never cross as their C-ABI types.
  if (auto innt = dynamic_cast<Int*>(refMT->kind)) {
    return LLVMIntTypeInContext(globalState->context, innt->bits);
  } else if (dynamic_cast<Bool*>(refMT->kind)) {
    return LLVMInt8TypeInContext(globalState->context);
  } else if (dynamic_cast<Float*>(refMT->kind)) {
    return LLVMDoubleTypeInContext(globalState->context);
  } else if (dynamic_cast<Never*>(refMT->kind)) {
    return LLVMArrayType(LLVMIntTypeInContext(globalState->context, NEVER_INT_BITS), 0);
  } else if (dynamic_cast<Void*>(refMT->kind)) {
    return LLVMVoidTypeInContext(globalState->context);
  }
  { assert(false); throw 1337; }
}


LLVMTypeRef RCImm::getInterfaceMethodVirtualParamAnyType(Reference* reference) {
  return LLVMPointerType(LLVMInt8TypeInContext(globalState->context), 0);
}

// VCOORD: do we still encrypt/decrypt?
Ref RCImm::receiveAndDecryptFamiliarReference(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* sourceRefMT,
    LLVMValueRef sourceRefLE) {
  // Per @FRMACZ, the boundary does no reference counting: the handle is a packed
  // pointer, unpack it without touching the RC. C-side alias/dealias is explicit
  // via the auto-gen'd helpers. Mirrors regularReceive minus the trailing alias.
  auto ffiHandleStructs = globalState->getFfiHandleStructs();
  if (dynamic_cast<StructKind*>(sourceRefMT->kind) ||
      dynamic_cast<StaticSizedArrayT*>(sourceRefMT->kind) ||
      dynamic_cast<RuntimeSizedArrayT*>(sourceRefMT->kind) ||
      dynamic_cast<Str*>(sourceRefMT->kind)) {
    auto refLT = translateType(sourceRefMT);
    auto membersLE = ffiHandleStructs->explodeForRegularConcrete(globalState, functionState, builder, sourceRefLE);
    auto objPtrLE = LLVMBuildIntToPtr(builder, membersLE.objPtrI64LE, refLT, "refA");
    return toRef(this, sourceRefMT, objPtrLE);
  } else if (auto interfaceMT = dynamic_cast<InterfaceKind*>(sourceRefMT->kind)) {
    auto itablePtrLT = LLVMPointerType(kindStructs.getInterfaceTableStruct(interfaceMT), 0);
    auto objPtrLT = LLVMPointerType(kindStructs.getControlBlock(interfaceMT)->getStruct(), 0);
    auto membersLE = ffiHandleStructs->explodeForRegularInterface(globalState, functionState, builder, sourceRefLE);
    auto itablePtrLE = LLVMBuildIntToPtr(builder, membersLE.typeInfoPtrI64LE, itablePtrLT, "refC");
    auto objPtrLE = LLVMBuildIntToPtr(builder, membersLE.objPtrI64LE, objPtrLT, "refB");
    auto interfaceFatPtrRawLE =
        makeInterfaceRefStruct(globalState, functionState, builder, &kindStructs, interfaceMT, objPtrLE, itablePtrLE);
    auto interfaceFatPtrLE =
        kindStructs.makeInterfaceFatPtr(FL(), functionState, builder, sourceRefMT, interfaceFatPtrRawLE);
    return toRef(this, sourceRefMT, interfaceFatPtrLE);
  }
  { assert(false); throw 1337; }
}

// VCOORD: do we still encrypt?
LLVMValueRef RCImm::encryptAndSendFamiliarReference(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* sourceRefMT,
    Ref sourceRef) {
  // Per @FRMACZ, the boundary does no reference counting: pack the pointer
  // without touching the RC. Mirrors regularEncrypt minus the leading dealias.
  if (dynamic_cast<StructKind*>(sourceRefMT->kind) ||
      dynamic_cast<StaticSizedArrayT*>(sourceRefMT->kind) ||
      dynamic_cast<RuntimeSizedArrayT*>(sourceRefMT->kind) ||
      dynamic_cast<Str*>(sourceRefMT->kind)) {
    auto sourceRefLE =
        checkValidReference(FL(), functionState, builder, false, sourceRefMT, sourceRef);
    auto objPtrI64LE = LLVMBuildPtrToInt(builder, sourceRefLE, LLVMInt64TypeInContext(globalState->context), "objPtrInt");
    return globalState->getFfiHandleStructs()->implodeForRegularConcrete(
        globalState, functionState, builder, objPtrI64LE);
  } else if (dynamic_cast<InterfaceKind*>(sourceRefMT->kind)) {
    checkValidReference(FL(), functionState, builder, false, sourceRefMT, sourceRef);
    LLVMValueRef itablePtrLE = nullptr, objPtrLE = nullptr;
    std::tie(itablePtrLE, objPtrLE) = explodeInterfaceRef(functionState, builder, sourceRefMT, sourceRef);
    auto objPtrI64LE = LLVMBuildPtrToInt(builder, objPtrLE, LLVMInt64TypeInContext(globalState->context), "objPtrInt");
    auto itablePtrI64LE = LLVMBuildPtrToInt(builder, itablePtrLE, LLVMInt64TypeInContext(globalState->context), "itablePtrInt");
    return globalState->getFfiHandleStructs()->implodeForRegularInterface(
        globalState, functionState, builder, itablePtrI64LE, objPtrI64LE);
  }
  { assert(false); throw 1337; }
}

void RCImm::initializeElementInSSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* ssaRefMT,
    StaticSizedArrayT* ssaMT,
    LiveRef ssaRef,
    InBoundsLE indexInBoundsLE,
    Ref elementRef) {
  auto ssaDefM = globalState->program->getStaticSizedArray(ssaMT);
  auto elementType = ssaDefM->elementType;
  buildFlare(FL(), globalState, functionState, builder);
  regularInitializeElementInSSA(
      globalState, functionState, builder, &kindStructs, ssaRefMT,
      elementType, ssaRef, indexInBoundsLE, elementRef);
}

Ref RCImm::deinitializeElementFromSSA(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* ssaRefMT,
    StaticSizedArrayT* ssaMT,
    LiveRef arrayRef,
    InBoundsLE indexInBoundsLE) {
  { assert(false); throw 1337; }
  exit(1);
}

Weakability RCImm::getKindWeakability(Kind* kind) {
  return Weakability::NON_WEAKABLE;
}

void RCImm::declareExtraFunctions() {
  auto valeStrMT = globalState->metalCache->mutStrRef;

  auto freePrototype =
      globalState->metalCache->getPrototype(
          globalState->freeName, globalState->metalCache->voidRef,
          {getRegionRefType(), valeStrMT});
  auto freeNameL = globalState->freeName->name + "__str";
  declareExtraFunction(globalState, freePrototype, freeNameL);

  declareConcreteAliasFunction(globalState->metalCache->str);
  declareConcreteDealiasFunction(globalState->metalCache->str);
  declareConcreteRefEqFunction(globalState->metalCache->str);
  declareStrPrimitives();
}

void RCImm::defineExtraFunctions() {
  defineConcreteFreeFunction(globalState->metalCache->str);
  defineConcreteAliasFunction(globalState->metalCache->str);
  defineConcreteDealiasFunction(globalState->metalCache->str);
  defineConcreteRefEqFunction(globalState->metalCache->str);
  defineStrPrimitives();
}

ValeFuncPtrLE RCImm::getInterfaceMethodFunctionPtr(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Reference* virtualParamMT,
    Ref virtualArgRef,
    int indexInEdge) {
  return getInterfaceMethodFunctionPtrFromItable(
      globalState, functionState, builder, &kindStructs, virtualParamMT, virtualArgRef, indexInEdge);
}

LLVMValueRef RCImm::stackify(
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Local* local,
    Ref refToStore,
    bool knownLive) {
  auto toStoreLE = checkValidReference(FL(), functionState, builder, false, local->type, refToStore);
  auto typeLT = translateType(local->type);
  return makeBackendLocal(functionState, builder, typeLT, local->id->maybeName.c_str(), toStoreLE);
}

Ref RCImm::unstackify(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr) {
  return loadLocal(functionState, builder, local, localAddr);
}

Ref RCImm::loadLocal(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr) {
  return normalLocalLoad(globalState, functionState, builder, local, localAddr);
}

Ref RCImm::localStore(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr, Ref refToStore, bool knownLive) {
  return normalLocalStore(globalState, functionState, builder, local, localAddr, refToStore);
}

std::string RCImm::getExportName(
    Package* package,
    Reference* reference,
    bool includeProjectName) {
  // Under the opaque-handle FFI, shared kinds cross as right-sized handle
  // value-type typedefs (8B concrete / 16B interface, no `*` suffix).
  // Primitives get their raw C type names.
  auto kind = reference->kind;
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

Ref RCImm::createRegionInstanceLocal(FunctionState* functionState, LLVMBuilderRef builder) {
  auto regionLT = kindStructs.getStructInnerStruct(regionKind);
  auto regionInstancePtrLE =
      makeBackendLocal(functionState, builder, regionLT, "region", LLVMGetUndef(regionLT));
  auto regionInstanceRef = toRef(this, regionRefMT, regionInstancePtrLE);

  return regionInstanceRef;
}

void RCImm::declareConcreteFreeFunction(Kind* valeKind) {
  auto prototype = getFreePrototype(valeKind);
  auto nameL = globalState->freeName->name + "__" + globalState->getKindName(valeKind)->name;
  declareExtraFunction(globalState, prototype, nameL);
}

Prototype* RCImm::getAliasPrototype(Kind* valeKind) {
  auto refMT =
      globalState->metalCache->getReference(
          Ownership::MUTABLE_SHARE, Location::YONDER, valeKind);
  auto nameL = globalState->aliasName->name + "__" + globalState->getKindName(valeKind)->name;
  auto perKindName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  // Per @FRMACZ, `_alias` is C's explicit +1 primitive and returns the same
  // handle it took, so C can alias inline at a use site (e.g.
  // `Foo_name(Foo_alias(o))`) instead of a separate statement.
  return globalState->metalCache->getPrototype(
      perKindName, refMT, {refMT});
}

Prototype* RCImm::getDealiasPrototype(Kind* valeKind) {
  auto refMT =
      globalState->metalCache->getReference(
          Ownership::MUTABLE_SHARE, Location::YONDER, valeKind);
  auto nameL = globalState->dealiasName->name + "__" + globalState->getKindName(valeKind)->name;
  auto perKindName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(
      perKindName, globalState->metalCache->voidRef, {refMT});
}

void RCImm::declareConcreteAliasFunction(Kind* valeKind) {
  auto prototype = getAliasPrototype(valeKind);
  auto nameL = globalState->aliasName->name + "__" + globalState->getKindName(valeKind)->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteAliasFunction(Kind* valeKind) {
  auto prototype = getAliasPrototype(valeKind);
  defineFunctionBodyV(
      globalState, prototype,
      [&](FunctionState* functionState, LLVMBuilderRef builder) -> void {
        // Non-consuming +1 primitive: bump the RC and hand the same handle back
        // so the caller keeps its original AND gets an independent owned handle.
        auto objectRefMT = prototype->params[0];
        auto objectRef =
            toRef(globalState->getRegion(objectRefMT), objectRefMT,
                  functionState->getParam(UserArgIndex{0}));
        alias(FL(), functionState, builder, objectRefMT, objectRef);
        auto resultLE =
            checkValidReference(FL(), functionState, builder, false, objectRefMT, objectRef);
        LLVMBuildRet(builder, resultLE);
      });
}

void RCImm::declareConcreteDealiasFunction(Kind* valeKind) {
  auto prototype = getDealiasPrototype(valeKind);
  auto nameL = globalState->dealiasName->name + "__" + globalState->getKindName(valeKind)->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteDealiasFunction(Kind* valeKind) {
  auto prototype = getDealiasPrototype(valeKind);
  defineFunctionBodyV(
      globalState, prototype,
      [&](FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto objectRefMT = prototype->params[0];
        auto objectRef =
            toRef(globalState->getRegion(objectRefMT), objectRefMT,
                  functionState->getParam(UserArgIndex{0}));
        // Per @FRMACZ, `_dealias` is C's explicit -1 primitive: the boundary
        // receive doesn't touch RC, and this body drops the object's RC by one.
        dealias(FL(), functionState, builder, objectRefMT, objectRef);
        LLVMBuildRet(builder, makeVoid(globalState));
      });
}

Prototype* RCImm::getRefEqPrototype(Kind* valeKind) {
  auto refMT =
      globalState->metalCache->getReference(
          Ownership::MUTABLE_SHARE, Location::YONDER, valeKind);
  auto nameL = globalState->refEqName->name + "__" + globalState->getKindName(valeKind)->name;
  auto perKindName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(
      perKindName, globalState->metalCache->boolRef,
      {refMT, refMT});
}

Prototype* RCImm::getFieldGetterPrototype(StructKind* structKind, int memberIndex) {
  auto structDefM = globalState->program->getStruct(structKind);
  auto member = structDefM->members[memberIndex];
  auto structRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, structKind);
  auto nameL = globalState->getKindName(structKind)->name + "_" + member->name;
  auto perKindName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(
      perKindName, member->type, {structRefMT});
}

void RCImm::declareConcreteFieldGetter(StructDefinition* structDefM, int memberIndex) {
  auto prototype = getFieldGetterPrototype(structDefM->kind, memberIndex);
  auto member = structDefM->members[memberIndex];
  auto nameL = globalState->getKindName(structDefM->kind)->name + "_" + member->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteFieldGetter(StructDefinition* structDefM, int memberIndex) {
  auto prototype = getFieldGetterPrototype(structDefM->kind, memberIndex);
  auto member = structDefM->members[memberIndex];
  auto memberIndexCopy = memberIndex;
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, structDefM, member, memberIndexCopy]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto structRefMT = prototype->params[0];
        auto structRef =
            toRef(globalState->getRegion(structRefMT), structRefMT,
                  functionState->getParam(UserArgIndex{0}));
        auto regionInstanceRef = makeRegionInstance(builder);
        auto structLiveRef =
            checkRefLive(FL(), functionState, builder, regionInstanceRef, structRefMT, structRef, false);

        auto memberRefMT = member->type;
        auto memberRef =
            loadMember(functionState, builder, regionInstanceRef, structRefMT, structLiveRef,
                       memberIndexCopy, memberRefMT, memberRefMT, member->name);

        // Per @FRMACZ, an accessor is a normal Vale function: it consumes its
        // receiver (the struct arg, moved in by C) and returns an owned member.
        // Alias the member (+1) before dealiasing the struct, so if dropping the
        // struct's last ref would cascade a member dealias, the member survives.
        alias(FL(), functionState, builder, memberRefMT, memberRef);
        dealias(FL(), functionState, builder, structRefMT, structRef);

        auto memberLE =
            checkValidReference(FL(), functionState, builder, false, memberRefMT, memberRef);
        LLVMBuildRet(builder, memberLE);
      });
}

void RCImm::declareConcreteRefEqFunction(Kind* valeKind) {
  auto prototype = getRefEqPrototype(valeKind);
  auto nameL = globalState->refEqName->name + "__" + globalState->getKindName(valeKind)->name;
  declareExtraFunction(globalState, prototype, nameL);
}

Prototype* RCImm::getTypeTagPrototype(InterfaceKind* interfaceKind) {
  auto refMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, interfaceKind);
  auto nameL = globalState->typeTagName->name + "__" + interfaceKind->fullName->name;
  auto perKindName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(
      perKindName, globalState->metalCache->i32Ref, {refMT});
}

Prototype* RCImm::getAsSubstructPrototype(InterfaceKind* interfaceKind, StructKind* structKind) {
  auto interfaceRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, interfaceKind);
  auto structRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, structKind);
  auto nameL =
      globalState->asSubstructName->name +
      "__" + interfaceKind->fullName->name +
      "__" + structKind->fullName->name;
  auto perName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(perName, structRefMT, {interfaceRefMT});
}

Prototype* RCImm::getUpcastPrototype(StructKind* structKind, InterfaceKind* interfaceKind) {
  auto interfaceRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, interfaceKind);
  auto structRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, structKind);
  auto nameL =
      globalState->upcastName->name +
      "__" + structKind->fullName->name +
      "__" + interfaceKind->fullName->name;
  auto perName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(perName, interfaceRefMT, {structRefMT});
}

void RCImm::declareConcreteTypeTagFunction(InterfaceKind* interfaceKind) {
  auto prototype = getTypeTagPrototype(interfaceKind);
  auto nameL = globalState->typeTagName->name + "__" + interfaceKind->fullName->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteTypeTagFunction(InterfaceKind* interfaceKind) {
  auto prototype = getTypeTagPrototype(interfaceKind);
  auto edges = getEdgesForInterface(interfaceKind);
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, interfaceKind, edges]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto interfaceRefMT = prototype->params[0];
        auto interfaceRef =
            toRef(globalState->getRegion(interfaceRefMT), interfaceRefMT,
                  functionState->getParam(UserArgIndex{0}));

        LLVMValueRef itablePtrLE = nullptr;
        LLVMValueRef _objPtrLE = nullptr;
        std::tie(itablePtrLE, _objPtrLE) =
            explodeInterfaceRef(functionState, builder, interfaceRefMT, interfaceRef);

        auto int32LT = LLVMInt32TypeInContext(globalState->context);
        auto itableLT = kindStructs.getInterfaceTableStruct(interfaceKind);

        // Chain of selects: for each edge, if itablePtrLE matches the edge's
        // known itable pointer, return that edge's index; otherwise fall
        // through. Terminates in a runtime-unreachable -1 that buildAssertV
        // catches — the last matched select value ends up chosen.
        auto resultLE = LLVMConstInt(int32LT, (uint32_t)-1, true);
        if (edges != nullptr) {
          for (int i = (int)edges->size() - 1; i >= 0; i--) {
            auto edge = (*edges)[i];
            auto edgePtrLE = globalState->getInterfaceTablePtr(edge);
            auto diffLE = LLVMBuildPtrDiff2(builder, itableLT, itablePtrLE, edgePtrLE, "ptrDiff");
            auto matchLE = LLVMBuildICmp(builder, LLVMIntEQ, diffLE, constI64LE(globalState, 0), "ptrsMatch");
            auto tagLE = LLVMConstInt(int32LT, (uint64_t)i, false);
            resultLE = LLVMBuildSelect(builder, matchLE, tagLE, resultLE, "typeTag");
          }
        }

        auto validLE =
            LLVMBuildICmp(
                builder, LLVMIntSGE, resultLE, LLVMConstInt(int32LT, 0, false), "typeTagValid");
        buildAssertV(globalState, functionState, builder, validLE,
            "Interface ref did not match any known substruct edge in typeTag");

        // Normal Vale move semantics: consume the interface arg (moved in by C).
        // The result is a plain int, so nothing to alias.
        dealias(FL(), functionState, builder, interfaceRefMT, interfaceRef);

        LLVMBuildRet(builder, resultLE);
      });
}

void RCImm::declareConcreteAsSubstructFunction(Edge* edge) {
  auto prototype = getAsSubstructPrototype(edge->interfaceName, edge->structName);
  auto nameL =
      globalState->asSubstructName->name +
      "__" + edge->interfaceName->fullName->name +
      "__" + edge->structName->fullName->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteAsSubstructFunction(Edge* edge) {
  auto prototype = getAsSubstructPrototype(edge->interfaceName, edge->structName);
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, edge]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto interfaceRefMT = prototype->params[0];
        auto structRefMT = prototype->returnType;
        auto interfaceRef =
            toRef(globalState->getRegion(interfaceRefMT), interfaceRefMT,
                  functionState->getParam(UserArgIndex{0}));

        LLVMValueRef itablePtrLE = nullptr;
        LLVMValueRef objPtrLE = nullptr;
        std::tie(itablePtrLE, objPtrLE) =
            explodeInterfaceRef(functionState, builder, interfaceRefMT, interfaceRef);

        auto expectedItablePtrLE = globalState->getInterfaceTablePtr(edge);
        auto itableLT = kindStructs.getInterfaceTableStruct(edge->interfaceName);
        auto diffLE = LLVMBuildPtrDiff2(builder, itableLT, itablePtrLE, expectedItablePtrLE, "ptrDiff");
        auto matchLE = LLVMBuildICmp(builder, LLVMIntEQ, diffLE, constI64LE(globalState, 0), "ptrsMatch");
        buildAssertV(globalState, functionState, builder, matchLE,
            "Interface ref's substruct did not match expected downcast target");

        auto resultStructRefLE = kindStructs.downcastPtr(builder, structRefMT, objPtrLE);
        auto resultStructRef =
            toRef(globalState->getRegion(structRefMT), structRefMT, resultStructRefLE);

        // Normal Vale move semantics: the downcast result and the interface
        // arg are the same object. Alias the result (+1) before dealiasing the
        // interface receiver (-1) so the object survives — net zero, i.e. the
        // incoming owned ref is moved out as the struct ref.
        alias(FL(), functionState, builder, structRefMT, resultStructRef);
        dealias(FL(), functionState, builder, interfaceRefMT, interfaceRef);

        auto resultLE = checkValidReference(FL(), functionState, builder, false, structRefMT, resultStructRef);
        LLVMBuildRet(builder, resultLE);
      });
}

void RCImm::declareConcreteUpcastFunction(Edge* edge) {
  auto prototype = getUpcastPrototype(edge->structName, edge->interfaceName);
  auto nameL =
      globalState->upcastName->name +
      "__" + edge->structName->fullName->name +
      "__" + edge->interfaceName->fullName->name;
  declareExtraFunction(globalState, prototype, nameL);
}

Prototype* RCImm::getRsaLenPrototype(RuntimeSizedArrayT* rsaKind) {
  auto arrRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, rsaKind);
  auto nameL = globalState->arrLenName->name + "__" + rsaKind->name->name;
  auto perName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(perName, globalState->metalCache->i32Ref, {arrRefMT});
}

Prototype* RCImm::getRsaAtPrototype(RuntimeSizedArrayT* rsaKind) {
  auto arrRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, rsaKind);
  auto elementType = globalState->program->getRuntimeSizedArray(rsaKind)->elementType;
  auto nameL = globalState->arrAtName->name + "__" + rsaKind->name->name;
  auto perName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(
      perName, elementType, {arrRefMT, globalState->metalCache->i32Ref});
}

Prototype* RCImm::getSsaLenPrototype(StaticSizedArrayT* ssaKind) {
  auto arrRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, ssaKind);
  auto nameL = globalState->arrLenName->name + "__" + ssaKind->name->name;
  auto perName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(perName, globalState->metalCache->i32Ref, {arrRefMT});
}

Prototype* RCImm::getSsaAtPrototype(StaticSizedArrayT* ssaKind) {
  auto arrRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, ssaKind);
  auto elementType = globalState->program->getStaticSizedArray(ssaKind)->elementType;
  auto nameL = globalState->arrAtName->name + "__" + ssaKind->name->name;
  auto perName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(
      perName, elementType, {arrRefMT, globalState->metalCache->i32Ref});
}

void RCImm::declareConcreteRsaLenFunction(RuntimeSizedArrayT* rsaKind) {
  auto prototype = getRsaLenPrototype(rsaKind);
  auto nameL = globalState->arrLenName->name + "__" + rsaKind->name->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteRsaLenFunction(RuntimeSizedArrayDefinitionT* rsaDef) {
  auto prototype = getRsaLenPrototype(rsaDef->kind);
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, rsaDef]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto arrRefMT = prototype->params[0];
        auto arrRef =
            toRef(globalState->getRegion(arrRefMT), arrRefMT,
                  functionState->getParam(UserArgIndex{0}));
        auto regionInstanceRef = makeRegionInstance(builder);
        auto arrLiveRef =
            checkRefLive(FL(), functionState, builder, regionInstanceRef, arrRefMT, arrRef, false);

        auto lenRef =
            getRuntimeSizedArrayLength(
                functionState, builder, regionInstanceRef, arrRefMT, arrLiveRef);

        // Normal Vale move semantics: consume the array arg (moved in by C).
        dealias(FL(), functionState, builder, arrRefMT, arrRef);

        auto lenLE =
            checkValidReference(FL(), functionState, builder, false, globalState->metalCache->i32Ref, lenRef);
        LLVMBuildRet(builder, lenLE);
      });
}

void RCImm::declareConcreteRsaAtFunction(RuntimeSizedArrayT* rsaKind) {
  auto prototype = getRsaAtPrototype(rsaKind);
  auto nameL = globalState->arrAtName->name + "__" + rsaKind->name->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteRsaAtFunction(RuntimeSizedArrayDefinitionT* rsaDef) {
  auto prototype = getRsaAtPrototype(rsaDef->kind);
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, rsaDef]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto arrRefMT = prototype->params[0];
        auto arrRef =
            toRef(globalState->getRegion(arrRefMT), arrRefMT,
                  functionState->getParam(UserArgIndex{0}));
        auto indexRef =
            toRef(globalState->getRegion(globalState->metalCache->i32Ref),
                  globalState->metalCache->i32Ref,
                  functionState->getParam(UserArgIndex{1}));
        auto regionInstanceRef = makeRegionInstance(builder);
        auto arrLiveRef =
            checkRefLive(FL(), functionState, builder, regionInstanceRef, arrRefMT, arrRef, false);

        // Bounds check: assert 0 <= index < length.
        auto lenRef =
            getRuntimeSizedArrayLength(
                functionState, builder, regionInstanceRef, arrRefMT, arrLiveRef);
        auto lenLE =
            checkValidReference(FL(), functionState, builder, false, globalState->metalCache->i32Ref, lenRef);
        auto indexLE =
            checkValidReference(FL(), functionState, builder, false, globalState->metalCache->i32Ref, indexRef);
        auto int32LT = LLVMInt32TypeInContext(globalState->context);
        auto geZeroLE = LLVMBuildICmp(builder, LLVMIntSGE, indexLE, LLVMConstInt(int32LT, 0, false), "geZero");
        auto ltLenLE = LLVMBuildICmp(builder, LLVMIntSLT, indexLE, lenLE, "ltLen");
        auto inBoundsLE = LLVMBuildAnd(builder, geZeroLE, ltLenLE, "inBounds");
        buildAssertV(globalState, functionState, builder, inBoundsLE, "RSA at() index out of bounds");
        auto indexInBoundsLE = InBoundsLE{indexLE};

        auto elementRef =
            loadElementFromRSA(
                functionState, builder, regionInstanceRef, arrRefMT, rsaDef->kind,
                arrLiveRef, indexInBoundsLE).move();

        auto elementRefMT = rsaDef->elementType;
        // Normal Vale move semantics: return the element as an owned ref (+1)
        // and consume the array arg. Alias the element before dealiasing the
        // array so it survives the array's drop.
        alias(FL(), functionState, builder, elementRefMT, elementRef);
        dealias(FL(), functionState, builder, arrRefMT, arrRef);

        auto elementLE =
            checkValidReference(FL(), functionState, builder, false, elementRefMT, elementRef);
        LLVMBuildRet(builder, elementLE);
      });
}

void RCImm::declareConcreteSsaLenFunction(StaticSizedArrayT* ssaKind) {
  auto prototype = getSsaLenPrototype(ssaKind);
  auto nameL = globalState->arrLenName->name + "__" + ssaKind->name->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteSsaLenFunction(StaticSizedArrayDefinitionT* ssaDef) {
  auto prototype = getSsaLenPrototype(ssaDef->kind);
  auto size = ssaDef->size;
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, size]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        // SSA length is a compile-time constant, but the array arg is still
        // moved in by C, so normal Vale move semantics consume it.
        auto arrRefMT = prototype->params[0];
        auto arrRef =
            toRef(globalState->getRegion(arrRefMT), arrRefMT,
                  functionState->getParam(UserArgIndex{0}));
        dealias(FL(), functionState, builder, arrRefMT, arrRef);
        LLVMBuildRet(builder, constI32LE(globalState, size));
      });
}

void RCImm::declareConcreteSsaAtFunction(StaticSizedArrayT* ssaKind) {
  auto prototype = getSsaAtPrototype(ssaKind);
  auto nameL = globalState->arrAtName->name + "__" + ssaKind->name->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteSsaAtFunction(StaticSizedArrayDefinitionT* ssaDef) {
  auto prototype = getSsaAtPrototype(ssaDef->kind);
  auto size = ssaDef->size;
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, ssaDef, size]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto arrRefMT = prototype->params[0];
        auto arrRef =
            toRef(globalState->getRegion(arrRefMT), arrRefMT,
                  functionState->getParam(UserArgIndex{0}));
        auto indexRef =
            toRef(globalState->getRegion(globalState->metalCache->i32Ref),
                  globalState->metalCache->i32Ref,
                  functionState->getParam(UserArgIndex{1}));
        auto regionInstanceRef = makeRegionInstance(builder);
        auto arrLiveRef =
            checkRefLive(FL(), functionState, builder, regionInstanceRef, arrRefMT, arrRef, false);

        auto indexLE =
            checkValidReference(FL(), functionState, builder, false, globalState->metalCache->i32Ref, indexRef);
        auto int32LT = LLVMInt32TypeInContext(globalState->context);
        auto geZeroLE = LLVMBuildICmp(builder, LLVMIntSGE, indexLE, LLVMConstInt(int32LT, 0, false), "geZero");
        auto ltSizeLE = LLVMBuildICmp(builder, LLVMIntSLT, indexLE, constI32LE(globalState, size), "ltSize");
        auto inBoundsLE = LLVMBuildAnd(builder, geZeroLE, ltSizeLE, "inBounds");
        buildAssertV(globalState, functionState, builder, inBoundsLE, "SSA at() index out of bounds");
        auto indexInBoundsLE = InBoundsLE{indexLE};

        auto elementRef =
            loadElementFromSSA(
                functionState, builder, regionInstanceRef, arrRefMT, ssaDef->kind,
                arrLiveRef, indexInBoundsLE).move();

        auto elementRefMT = ssaDef->elementType;
        // Normal Vale move semantics: return the element owned (+1), consume
        // the array. Alias the element before dealiasing the array.
        alias(FL(), functionState, builder, elementRefMT, elementRef);
        dealias(FL(), functionState, builder, arrRefMT, arrRef);

        auto elementLE =
            checkValidReference(FL(), functionState, builder, false, elementRefMT, elementRef);
        LLVMBuildRet(builder, elementLE);
      });
}

Prototype* RCImm::getStructNewPrototype(StructKind* structKind) {
  auto structDefM = globalState->program->getStruct(structKind);
  auto structRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, structKind);
  std::vector<Reference*> params;
  for (auto member : structDefM->members) params.push_back(member->type);
  auto nameL = globalState->structNewName->name + "__" + structKind->fullName->name;
  auto perName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(perName, structRefMT, params);
}

Prototype* RCImm::getSsaNewPrototype(StaticSizedArrayT* ssaKind) {
  auto ssaDef = globalState->program->getStaticSizedArray(ssaKind);
  auto arrRefMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, ssaKind);
  std::vector<Reference*> params(ssaDef->size, ssaDef->elementType);
  auto nameL = globalState->ssaNewName->name + "__" + ssaKind->name->name;
  auto perName = globalState->metalCache->getName(globalState->metalCache->builtinPackageCoord, nameL);
  return globalState->metalCache->getPrototype(perName, arrRefMT, params);
}

void RCImm::declareConcreteStructNewFunction(StructDefinition* structDefM) {
  auto prototype = getStructNewPrototype(structDefM->kind);
  auto nameL = globalState->structNewName->name + "__" + structDefM->kind->fullName->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteStructNewFunction(StructDefinition* structDefM) {
  auto prototype = getStructNewPrototype(structDefM->kind);
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, structDefM]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto structRefMT = prototype->returnType;
        auto regionInstanceRef = makeRegionInstance(builder);

        std::vector<Ref> memberRefs;
        for (int i = 0; i < (int)structDefM->members.size(); i++) {
          auto memberRefMT = structDefM->members[i]->type;
          auto memberRef =
              toRef(globalState->getRegion(memberRefMT), memberRefMT,
                    functionState->getParam(UserArgIndex{i}));
          memberRefs.push_back(memberRef);
        }

        // allocate() starts the new struct at RC=1 (see SRCAO in allocate()).
        // Each share-typed field param arrives already aliased (+1) by the
        // wrapper's regularReceive; the allocation stores those refs into the
        // struct without re-aliasing, so no per-field alias/dealias is needed
        // here — the +1 travels straight from arg into the new struct.
        auto structRef =
            allocate(regionInstanceRef, FL(), functionState, builder, structRefMT, memberRefs);

        auto structLE =
            checkValidReference(FL(), functionState, builder, false, structRefMT, structRef);
        LLVMBuildRet(builder, structLE);
      });
}

void RCImm::declareConcreteSsaNewFunction(StaticSizedArrayDefinitionT* ssaDef) {
  auto prototype = getSsaNewPrototype(ssaDef->kind);
  auto nameL = globalState->ssaNewName->name + "__" + ssaDef->kind->name->name;
  declareExtraFunction(globalState, prototype, nameL);
}

void RCImm::defineConcreteSsaNewFunction(StaticSizedArrayDefinitionT* ssaDef) {
  auto prototype = getSsaNewPrototype(ssaDef->kind);
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, ssaDef]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto arrRefMT = prototype->returnType;
        auto regionInstanceRef = makeRegionInstance(builder);

        auto arrLiveRef =
            constructStaticSizedArray(regionInstanceRef, functionState, builder, arrRefMT, ssaDef->kind);

        auto elementRefMT = ssaDef->elementType;
        for (int i = 0; i < ssaDef->size; i++) {
          auto elementRef =
              toRef(globalState->getRegion(elementRefMT), elementRefMT,
                    functionState->getParam(UserArgIndex{i}));
          auto indexInBoundsLE = InBoundsLE{constI32LE(globalState, i)};
          initializeElementInSSA(
              functionState, builder, regionInstanceRef, arrRefMT, ssaDef->kind,
              arrLiveRef, indexInBoundsLE, elementRef);
        }

        // constructStaticSizedArray starts the array at RC=1 (see SRCAO). The
        // element params flow directly into the array via initializeElement
        // without re-aliasing, mirroring the struct case.
        auto arrRef = toRef(globalState, arrRefMT, arrLiveRef);
        auto arrLE = checkValidReference(FL(), functionState, builder, false, arrRefMT, arrRef);
        LLVMBuildRet(builder, arrLE);
      });
}

void RCImm::defineConcreteUpcastFunction(Edge* edge) {
  auto prototype = getUpcastPrototype(edge->structName, edge->interfaceName);
  defineFunctionBodyV(
      globalState, prototype,
      [this, prototype, edge]
      (FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto structRefMT = prototype->params[0];
        auto interfaceRefMT = prototype->returnType;
        auto structRef =
            toRef(globalState->getRegion(structRefMT), structRefMT,
                  functionState->getParam(UserArgIndex{0}));

        auto interfaceRef =
            upcastStrong(
                globalState, functionState, builder, &kindStructs,
                structRefMT, edge->structName, structRef,
                interfaceRefMT, edge->interfaceName);

        // Normal Vale move semantics: result and struct arg are the same
        // object. Alias the result (+1) before dealiasing the struct receiver
        // (-1) — net zero, the incoming owned ref moves out as the interface.
        alias(FL(), functionState, builder, interfaceRefMT, interfaceRef);
        dealias(FL(), functionState, builder, structRefMT, structRef);

        auto resultLE = checkValidReference(FL(), functionState, builder, false, interfaceRefMT, interfaceRef);
        LLVMBuildRet(builder, resultLE);
      });
}

Prototype* RCImm::getStrLenPrototype() {
  auto strKind = globalState->metalCache->str;
  auto refMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, strKind);
  return globalState->metalCache->getPrototype(
      globalState->strLenName, globalState->metalCache->i32Ref, {refMT});
}

Prototype* RCImm::getStrCharAtPrototype() {
  auto strKind = globalState->metalCache->str;
  auto refMT = globalState->metalCache->getReference(
      Ownership::MUTABLE_SHARE, Location::YONDER, strKind);
  return globalState->metalCache->getPrototype(
      globalState->strCharAtName, globalState->metalCache->i32Ref,
      {refMT, globalState->metalCache->i32Ref});
}

void RCImm::declareStrPrimitives() {
  declareExtraFunction(globalState, getStrLenPrototype(), globalState->strLenName->name);
  declareExtraFunction(globalState, getStrCharAtPrototype(), globalState->strCharAtName->name);
}

void RCImm::defineStrPrimitives() {
  auto strPrototypeL = getStrLenPrototype();
  defineFunctionBodyV(
      globalState, strPrototypeL,
      [this, strPrototypeL](FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto strRefMT = strPrototypeL->params[0];
        auto strRef = toRef(globalState->getRegion(strRefMT), strRefMT,
                            functionState->getParam(UserArgIndex{0}));
        auto regionInstanceRef = makeRegionInstance(builder);
        auto strLiveRef = checkRefLive(FL(), functionState, builder, regionInstanceRef, strRefMT, strRef, false);
        auto lenLE = getStringLen(functionState, builder, strRefMT, regionInstanceRef, strLiveRef);
        // Normal Vale move semantics: consume the str arg (moved in by C).
        dealias(FL(), functionState, builder, strRefMT, strRef);
        LLVMBuildRet(builder, lenLE);
      });

  auto strCharAtPrototypeL = getStrCharAtPrototype();
  defineFunctionBodyV(
      globalState, strCharAtPrototypeL,
      [this, strCharAtPrototypeL](FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto strRefMT = strCharAtPrototypeL->params[0];
        auto strRef = toRef(globalState->getRegion(strRefMT), strRefMT,
                            functionState->getParam(UserArgIndex{0}));
        auto indexLE = functionState->getParam(UserArgIndex{1});
        auto regionInstanceRef = makeRegionInstance(builder);
        auto strLiveRef = checkRefLive(FL(), functionState, builder, regionInstanceRef, strRefMT, strRef, false);
        auto bytesPtrLE = getStringBytesPtr(functionState, builder, strRefMT, regionInstanceRef, strLiveRef);

        // Load byte at index and zero-extend to i32.
        auto int8LT = LLVMInt8TypeInContext(globalState->context);
        auto int32LT = LLVMInt32TypeInContext(globalState->context);
        std::vector<LLVMValueRef> indices = {indexLE};
        auto charPtrLE = LLVMBuildGEP2(builder, int8LT, bytesPtrLE, indices.data(), (unsigned)indices.size(), "charPtr");
        auto byteLE = LLVMBuildLoad2(builder, int8LT, charPtrLE, "byteVal");
        auto byteAsI32LE = LLVMBuildZExt(builder, byteLE, int32LT, "byteI32");

        // Normal Vale move semantics: consume the str arg. The byte is already
        // loaded into a register above, so dealiasing the str now is safe even
        // if it drops the last ref.
        dealias(FL(), functionState, builder, strRefMT, strRef);
        LLVMBuildRet(builder, byteAsI32LE);
      });
}

void RCImm::defineConcreteRefEqFunction(Kind* valeKind) {
  auto prototype = getRefEqPrototype(valeKind);
  defineFunctionBodyV(
      globalState, prototype,
      [&](FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto refMT = prototype->params[0];
        auto aRef = toRef(globalState->getRegion(refMT), refMT,
                          functionState->getParam(UserArgIndex{0}));
        auto bRef = toRef(globalState->getRegion(refMT), refMT,
                          functionState->getParam(UserArgIndex{1}));
        auto aLE = checkValidReference(FL(), functionState, builder, true, refMT, aRef);
        auto bLE = checkValidReference(FL(), functionState, builder, true, refMT, bRef);

        // Compare object pointers. For interface refs (fat ptr), extract the
        // object pointer first.
        LLVMValueRef aPtrLE = aLE;
        LLVMValueRef bPtrLE = bLE;
        if (dynamic_cast<InterfaceKind*>(valeKind)) {
          LLVMValueRef _itableALE = nullptr, _itableBLE = nullptr;
          std::tie(_itableALE, aPtrLE) = explodeInterfaceRef(functionState, builder, refMT, aRef);
          std::tie(_itableBLE, bPtrLE) = explodeInterfaceRef(functionState, builder, refMT, bRef);
        }
        auto int64LT = LLVMInt64TypeInContext(globalState->context);
        auto aI64LE = LLVMBuildPtrToInt(builder, aPtrLE, int64LT, "aI64");
        auto bI64LE = LLVMBuildPtrToInt(builder, bPtrLE, int64LT, "bI64");
        auto eqLE = LLVMBuildICmp(builder, LLVMIntEQ, aI64LE, bI64LE, "refEq");

        // Normal Vale move semantics: ref_eq consumes both operands (moved in
        // by C). The pointers are already compared into registers above, so
        // dealiasing now is safe. Bool result has no RC.
        dealias(FL(), functionState, builder, refMT, aRef);
        dealias(FL(), functionState, builder, refMT, bRef);

        LLVMBuildRet(builder, eqLE);
      });
}

void RCImm::defineConcreteFreeFunction(Kind* valeKind) {
  auto i32MT = globalState->metalCache->i32Ref;
  auto boolMT = globalState->metalCache->boolRef;

  auto prototype = getFreePrototype(valeKind);

  defineFunctionBodyV(
      globalState, prototype,
      [&](FunctionState* functionState, LLVMBuilderRef builder) -> void {
        auto objectRefMT = prototype->params[1];

        auto regionInstanceRef = makeRegionInstance(builder);

        auto objectRef =
            checkRefLive(
                FL(), functionState, builder, regionInstanceRef, objectRefMT,
                toRef(
                    globalState->getRegion(objectRefMT),
                    objectRefMT,
                    functionState->getParam(UserArgIndex{FREE_PARAM_OBJECT_REF})),
                    false);

        if (auto structKind = dynamic_cast<StructKind *>(objectRefMT->kind)) {
          auto structDefM = globalState->program->getStruct(structKind);

          for (int i = 0; i < structDefM->members.size(); i++) {
            auto memberM = structDefM->members[i];
            auto memberRefMT = memberM->type;
            auto memberRef =
                globalState->getRegion(objectRefMT)->loadMember(
                    functionState, builder, regionInstanceRef, objectRefMT, objectRef,
                    i, memberRefMT, memberRefMT, memberM->name);
            discard(FL(), globalState, functionState, builder, memberRefMT, memberRef);
          }

          innerDeallocate(FL(), globalState, functionState, &kindStructs, builder, objectRefMT, objectRef);
          LLVMBuildRet(builder, makeVoid(globalState));
        } else if (dynamic_cast<Str*>(objectRefMT->kind)) {
          buildFlare(FL(), globalState, functionState, builder, "done storing");

          innerDeallocate(FL(), globalState, functionState, &kindStructs, builder, objectRefMT, objectRef);
          LLVMBuildRet(builder, makeVoid(globalState));
        } else if (auto rsaMT = dynamic_cast<RuntimeSizedArrayT *>(objectRefMT->kind)) { // XEGDWR combine with below case
          auto rsaRefMT = objectRefMT;

          auto lengthRef =
              getRuntimeSizedArrayLength(
                  functionState, builder, regionInstanceRef, objectRefMT, objectRef);

          auto memberRefMT = globalState->program->getRuntimeSizedArray(rsaMT)->elementType;

          intRangeLoopReverseV(
              globalState, functionState, builder, globalState->metalCache->i32Ref, lengthRef,

              [this, functionState, regionInstanceRef, objectRefMT, rsaMT, objectRef, memberRefMT](
                  Ref indexRef, LLVMBuilderRef bodyBuilder) {
                auto indexLE =
                    globalState->getRegion(globalState->metalCache->i32Ref)
                        ->checkValidReference(FL(), functionState, bodyBuilder, false, globalState->metalCache->i32Ref, indexRef);
                // Manually making InBoundsLE because the array's size is the bound of the containing loop.
                auto indexInBoundsLE = InBoundsLE{indexLE};

                auto memberRef =
                    globalState->getRegion(objectRefMT)
                        ->loadElementFromRSA(
                            functionState, bodyBuilder, regionInstanceRef, objectRefMT, rsaMT,
                            objectRef, indexInBoundsLE)
                        .move();
                discard(FL(), globalState, functionState, bodyBuilder, memberRefMT, memberRef);
              });

          innerDeallocate(FL(), globalState, functionState, &kindStructs, builder, objectRefMT, objectRef);
          LLVMBuildRet(builder, makeVoid(globalState));
        } else if (auto valeSsaMT = dynamic_cast<StaticSizedArrayT *>(objectRefMT->kind)) { // XEGDWR combine with above case
          auto hostSsaMT = dynamic_cast<StaticSizedArrayT *>(objectRefMT->kind);
          assert(hostSsaMT);
          auto ssaRefMT = objectRefMT;

          auto ssaDefM = globalState->program->getStaticSizedArray(valeSsaMT);
          int length = ssaDefM->size;
          auto memberRefMT = ssaDefM->elementType;

          intRangeLoopReverseV(
              globalState, functionState, builder, globalState->metalCache->i32Ref, globalState->constI32(length),
              [this, functionState, regionInstanceRef, objectRefMT, hostSsaMT, objectRef, memberRefMT](
                  Ref indexRef, LLVMBuilderRef bodyBuilder) {

                auto indexLE =
                    globalState->getRegion(globalState->metalCache->i32Ref)
                        ->checkValidReference(FL(), functionState, bodyBuilder, false, globalState->metalCache->i32Ref, indexRef);
                // Manually making InBoundsLE because the array's size is the bound of the containing loop.
                auto indexInBoundsLE = InBoundsLE{indexLE};

                auto memberRef =
                    globalState->getRegion(objectRefMT)
                        ->loadElementFromSSA(
                            functionState, bodyBuilder, regionInstanceRef, objectRefMT, hostSsaMT,
                            objectRef, indexInBoundsLE)
                        .move();
                discard(FL(), globalState, functionState, bodyBuilder, memberRefMT, memberRef);
              });

          innerDeallocate(FL(), globalState, functionState, &kindStructs, builder, objectRefMT, objectRef);
          LLVMBuildRet(builder, makeVoid(globalState));
        } else
          { assert(false); throw 1337; }
      });
}

void RCImm::callFree(
    FunctionState *functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Kind* kind,
    Ref objectRef) {
  auto prototype = getFreePrototype(kind);
  if (auto interfaceMT = dynamic_cast<InterfaceKind*>(kind)) {
    buildFlare(FL(), globalState, functionState, builder);
    auto virtualArgRefMT = prototype->params[1];
    int indexInEdge = globalState->getInterfaceMethodIndex(interfaceMT, prototype);
    buildFlare(FL(), globalState, functionState, builder);
    auto methodFunctionPtrLE =
        globalState->getRegion(virtualArgRefMT)
            ->getInterfaceMethodFunctionPtr(functionState, builder, virtualArgRefMT, objectRef, indexInEdge);
    buildFlare(FL(), globalState, functionState, builder);
    buildInterfaceCall(globalState, functionState, builder, prototype, methodFunctionPtrLE, {regionInstanceRef, objectRef}, 1);
  } else {
    buildCallV(globalState, functionState, builder, prototype, {regionInstanceRef, objectRef});
  }
}

Prototype* RCImm::getFreePrototype(Kind* valeKind) {
  auto boolMT = globalState->metalCache->boolRef;
  auto refMT =
      globalState->metalCache->getReference(
          Ownership::MUTABLE_SHARE, Location::YONDER, valeKind);
  return globalState->metalCache->getPrototype(
      globalState->freeName, globalState->metalCache->voidRef, {regionRefMT, refMT});
}

Prototype* RCImm::getFreeThunkPrototype(StructKind* valeStructKind, InterfaceKind* valeInterfaceKind) {
  auto boolMT = globalState->metalCache->boolRef;
  auto structRefMT =
      globalState->metalCache->getReference(
          Ownership::MUTABLE_SHARE, Location::YONDER, valeStructKind);
  auto interfaceRefMT =
      globalState->metalCache->getReference(
          Ownership::MUTABLE_SHARE, Location::YONDER, valeInterfaceKind);
  return globalState->metalCache->getPrototype(
      globalState->freeThunkName, globalState->metalCache->voidRef,
      {getRegionRefType(), structRefMT});
}

void RCImm::defineEdgeFreeFunction(Edge* edge) {
  auto boolMT = globalState->metalCache->boolRef;

  auto thunkPrototype = getFreeThunkPrototype(edge->structName, edge->interfaceName);
  defineFunctionBodyV(
      globalState, thunkPrototype,
      [&](FunctionState *functionState, LLVMBuilderRef builder) {
        auto structPrototype = getFreePrototype(edge->structName);

        auto objectRefMT = structPrototype->params[1];

        auto regionInstanceRef =
            toRef(this, regionRefMT, functionState->getParam(UserArgIndex{FREE_PARAM_REGION_INSTANCE_REF}));
        auto objectRef =
            toRef(globalState->getRegion(objectRefMT), objectRefMT, functionState->getParam(UserArgIndex{FREE_PARAM_OBJECT_REF}));

        buildCallV(
            globalState, functionState, builder, structPrototype,
            {regionInstanceRef, objectRef});

//        auto interfaceKind = dynamic_cast<InterfaceKind *>(thunkPrototype->returnType->kind);
//        assert(interfaceKind);
//        auto structKind = dynamic_cast<StructKind *>(structPrototype->returnType->kind);
//        assert(structKind);

//        auto interfaceRef =
//            upcast(
//                functionState, builder, structPrototype->returnType, structKind,
//                objectRef, thunkPrototype->returnType, interfaceKind);
//
//        checkValidReference(FL(), functionState, builder, true, thunkPrototype->returnType, interfaceRef);
        LLVMBuildRet(builder, makeVoid(globalState));
      });
}

void RCImm::declareInterfaceFreeFunction(InterfaceKind* kind) {
  auto interface = dynamic_cast<InterfaceKind*>(kind);
  auto interfaceMethod = getFreeInterfaceMethod(kind);
  globalState->addInterfaceExtraMethod(interface, interfaceMethod);
}

InterfaceMethod* RCImm::getFreeInterfaceMethod(Kind* valeKind) {
  return globalState->metalCache->getInterfaceMethod(
      getFreePrototype(valeKind), 1);
}

LiveRef RCImm::checkRefLive(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* refMT,
    Ref ref,
    bool refKnownLive) {
  // Everything is always known live in an RC world.
  auto refLE = checkValidReference(FL(), functionState, builder, true, refMT, ref);
  return wrapToLiveRef(FL(), functionState, builder, regionInstanceRef, refMT, refLE);
}

LiveRef RCImm::wrapToLiveRef(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* refMT,
    LLVMValueRef ref) {
  assert(translateType(refMT) == LLVMTypeOf(ref));
  return LiveRef(refMT, ref);
}

LiveRef RCImm::preCheckBorrow(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* refMT,
    Ref ref,
    bool refKnownLive) {
  // Everything is always known live in an RC world.
  return checkRefLive(FL(), functionState, builder, regionInstanceRef, refMT, ref, true);
}

Ref RCImm::mutabilify(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* refMT,
    Ref ref,
    Reference* targetRefMT) {
  { assert(false); throw 1337; } // impl
}

LiveRef RCImm::immutabilify(
    AreaAndFileAndLine checkerAFL,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Ref regionInstanceRef,
    Reference* refMT,
    Ref ref,
    Reference* targetRefMT) {
  // Imm and mut refs in RC are the same, so we can just do a transmute.
  auto transmutedRef =
      transmutePtr(globalState, functionState, builder, true, refMT, targetRefMT, ref);
  auto transmutedRefLE =
      checkValidReference(FL(), functionState, builder, true, targetRefMT, transmutedRef);
  return wrapToLiveRef(FL(), functionState, builder, regionInstanceRef, targetRefMT, transmutedRefLE);
}
