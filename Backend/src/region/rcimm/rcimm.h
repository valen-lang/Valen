#ifndef REGION_COMMON_IMMRC_IMMRC_H_
#define REGION_COMMON_IMMRC_IMMRC_H_

#include <llvm-c/Types.h>
#include "../../globalstate.h"
#include <iostream>
#include "../common/primitives.h"
#include "../../function/expressions/shared/afl.h"
#include "../../function/function.h"
#include "../common/defaultlayout/structs.h"

ControlBlock makeImmControlBlock(GlobalState* globalState);

class RCImm : public IRegion {
public:
  using IRegion::checkValidReference;

  RCImm(GlobalState* globalState_);


  void alias(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* sourceRef,
      Ref ref) override;

  void dealias(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* sourceMT,
      Ref sourceRef) override;

  Ref lockWeak(
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
      std::function<Ref(LLVMBuilderRef)> buildElse) override;


  Ref asSubtype(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* resultOptTypeM,
      Reference* sourceInterfaceRefMT,
      Ref sourceInterfaceRef,
      bool sourceRefKnownLive,
      Kind* targetKind,
      std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
      std::function<Ref(LLVMBuilderRef)> buildElse) override;

  LLVMTypeRef translateType(Reference* referenceM) override;

  LLVMValueRef getCensusObjectId(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* refM,
      Ref ref) override;

  Ref upcastWeak(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      WeakFatPtrLE sourceRefLE,
      StructKind* sourceStructKindM,
      Reference* sourceStructTypeM,
      InterfaceKind* targetInterfaceKindM,
      Reference* targetInterfaceTypeM) override;

  void declareStruct(StructDefinition* structM) override;
  void declareStructExtraFunctions(StructDefinition* structDefM) override;
  void defineStruct(StructDefinition* structM) override;
  void defineStructExtraFunctions(StructDefinition* structDefM) override;

  void declareStaticSizedArray(StaticSizedArrayDefinitionT* staticSizedArrayDefinitionMT) override;
  void declareStaticSizedArrayExtraFunctions(StaticSizedArrayDefinitionT* ssaDef) override;
  void defineStaticSizedArray(StaticSizedArrayDefinitionT* staticSizedArrayDefinitionMT) override;
  void defineStaticSizedArrayExtraFunctions(StaticSizedArrayDefinitionT* ssaDef) override;

  void declareRuntimeSizedArray(RuntimeSizedArrayDefinitionT* runtimeSizedArrayDefinitionMT) override;
  void declareRuntimeSizedArrayExtraFunctions(RuntimeSizedArrayDefinitionT* rsaDefM) override;
  void defineRuntimeSizedArray(RuntimeSizedArrayDefinitionT* runtimeSizedArrayDefinitionMT) override;
  void defineRuntimeSizedArrayExtraFunctions(RuntimeSizedArrayDefinitionT* rsaDefM) override;

  void declareInterface(InterfaceDefinition* interfaceM) override;
  void declareInterfaceExtraFunctions(InterfaceDefinition* structDefM) override;
  void defineInterface(InterfaceDefinition* interfaceM) override;
  void defineInterfaceExtraFunctions(InterfaceDefinition* structDefM) override;

  void declareEdge(Edge* edge) override;
  void defineEdge(Edge* edge) override;

  void declareExtraFunctions() override;
  void defineExtraFunctions() override;

  Ref weakAlias(
      FunctionState* functionState, LLVMBuilderRef builder, Reference* sourceRefMT, Reference* targetRefMT, Ref sourceRef) override;

  void discardOwningRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      BlockState* blockState,
      LLVMBuilderRef builder,
      Reference* sourceMT,
      LiveRef sourceRef) override;


  void noteWeakableDestroyed(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* refM,
      ControlBlockPtrLE controlBlockPtrLE) override;

  Ref loadMember(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* structRefMT,
      LiveRef structRef,
      int memberIndex,
      Reference* expectedMemberType,
      Reference* targetMemberType,
      const std::string& memberName) override;

  void storeMember(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* structRefMT,
      LiveRef structRef,
      int memberIndex,
      const std::string& memberName,
      Reference* newMemberRefMT,
      Ref newMemberRef) override;

  std::tuple<LLVMValueRef, LLVMValueRef> explodeInterfaceRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* virtualParamMT,
      Ref virtualArgRef) override;


  void aliasWeakRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* weakRefMT,
      Ref weakRef) override;

  void discardWeakRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* weakRefMT,
      Ref weakRef) override;

  Ref getIsAliveFromWeakRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* weakRefM,
      Ref weakRef,
      bool knownLive) override;

  LLVMValueRef getStringBytesPtr(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* refMT,
      Ref regionInstanceRef,
      LiveRef ref) override;

  Ref allocate(
      Ref regionInstanceRef,
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* desiredStructMT,
      const std::vector<Ref>& memberRefs) override;

  Ref upcast(
      FunctionState* functionState,
      LLVMBuilderRef builder,

      Reference* sourceStructMT,
      StructKind* sourceStructKindM,
      Ref sourceRefLE,

      Reference* targetInterfaceTypeM,
      InterfaceKind* targetInterfaceKindM) override;

  WrapperPtrLE lockWeakRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* refM,
      Ref weakRefLE,
      bool weakRefKnownLive) override;

  LiveRef checkRefLive(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* refMT,
      Ref ref,
      bool refKnownLive) override;

    LiveRef wrapToLiveRef(
        AreaAndFileAndLine checkerAFL,
        FunctionState* functionState,
        LLVMBuilderRef builder,
        Ref regionInstanceRef,
        Reference* refMT,
        LLVMValueRef ref) override;

  LiveRef preCheckBorrow(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* refMT,
      Ref ref,
      bool refKnownLive) override;

  // Returns a LLVMValueRef for a ref to the string object.
  // The caller should then use getStringBytesPtr to then fill the string's contents.
  LiveRef constructStaticSizedArray(
      Ref regionInstanceRef,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* referenceM,
      StaticSizedArrayT* kindM) override;

  // should expose a dereference thing instead
//  LLVMValueRef getStaticSizedArrayElementsPtr(
//      LLVMBuilderRef builder,
//      LLVMValueRef staticSizedArrayWrapperPtrLE) override;
//  LLVMValueRef getRuntimeSizedArrayElementsPtr(
//      LLVMBuilderRef builder,
//      LLVMValueRef runtimeSizedArrayWrapperPtrLE) override;

  Ref getRuntimeSizedArrayLength(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* rsaRefMT,
      LiveRef arrayRef) override;

  Ref getRuntimeSizedArrayCapacity(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* rsaRefMT,
      LiveRef arrayRef) override;

  LLVMValueRef checkValidReference(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      bool expectLive,
      Reference* refM,
      Ref ref) override;


  // TODO maybe combine with alias/acquireReference?
  // After we load from a local, member, or element, we can feed the result through this
  // function to turn it into a desired ownership.
  // Example:
  // - Can load from an owning ref member to get a constraint ref.
  // - Can load from a constraint ref member to get a weak ref.
  Ref upgradeLoadResultToRefWithTargetOwnership(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* sourceType,
      Reference* targetType,
      LoadResult sourceRef,
      bool resultKnownLive) override;

  void checkInlineStructType(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* refMT,
      Ref ref) override;

  LoadResult loadElementFromSSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* ssaRefMT,
      StaticSizedArrayT* ssaMT,
      LiveRef arrayRef,
      InBoundsLE indexRef) override;
  LoadResult loadElementFromRSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* rsaRefMT,
      RuntimeSizedArrayT* rsaMT,
      LiveRef arrayRef,
      InBoundsLE indexLE) override;


  Ref storeElementInRSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* rsaRefMT,
      RuntimeSizedArrayT* rsaMT,
      LiveRef arrayRef,
      InBoundsLE indexLE,
      Ref elementRef) override;


  void deallocate(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* refMT,
      LiveRef ref) override;


  LiveRef constructRuntimeSizedArray(
      Ref regionInstanceRef,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* rsaMT,
      RuntimeSizedArrayT* runtimeSizedArrayT,
      Ref capacityRef,
      const std::string& typeName) override;

  void pushRuntimeSizedArrayNoBoundsCheck(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* rsaRefMT,
      RuntimeSizedArrayT* rsaMT,
      LiveRef arrayRef,
      InBoundsLE sizeLE,
      Ref elementRef) override;

  Ref popRuntimeSizedArrayNoBoundsCheck(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* rsaRefMT,
      RuntimeSizedArrayT* rsaMT,
      LiveRef arrayRef,
      InBoundsLE indexLE) override;

  void initializeElementInSSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* ssaRefMT,
      StaticSizedArrayT* ssaMT,
      LiveRef arrayRef,
      InBoundsLE indexLE,
      Ref elementRef) override;

  Ref deinitializeElementFromSSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* ssaRefMT,
      StaticSizedArrayT* ssaMT,
      LiveRef arrayRef,
      InBoundsLE indexLE) override;

  Ref mallocStr(
      Ref regionInstanceRef,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      LLVMValueRef lengthLE,
      LLVMValueRef sourceCharsPtrLE) override;

  RegionId* getRegionId() override;

  LLVMValueRef getStringLen(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* refMT,
      Ref regionInstanceRef,
      LiveRef ref) override;


  std::string getExportName(Package* currentPackage, Reference* refMT, bool includeProjectName) override;
  std::string generateStructDefsC(
    Package* currentPackage,
      StructDefinition* refMT) override;
  std::string generateInterfaceDefsC(
    Package* currentPackage,
      InterfaceDefinition* refMT) override;
  std::string generateStaticSizedArrayDefsC(
    Package* currentPackage,
      StaticSizedArrayDefinitionT* ssaDefM) override;
  std::string generateRuntimeSizedArrayDefsC(
    Package* currentPackage,
      RuntimeSizedArrayDefinitionT* rsaDefM) override;


  LLVMTypeRef getExternalType(
      Reference* refMT) override;

  Ref receiveAndDecryptFamiliarReference(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* sourceRefMT,
      LLVMValueRef sourceRefLE) override;

  LLVMValueRef encryptAndSendFamiliarReference(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* sourceRefMT,
      Ref sourceRef) override;

  LLVMTypeRef getInterfaceMethodVirtualParamAnyType(Reference* reference) override;

  void discard(
      AreaAndFileAndLine from,
      GlobalState* globalState,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* sourceMT,
      Ref sourceRef);

  LoadResult loadMember2(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* structRefMT,
      LiveRef structLiveRef,
      int memberIndex,
      Reference* expectedMemberType,
      Reference* targetType,
      const std::string& memberName);

  void checkValidReference(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      KindStructs* kindStructs,
      Reference* refM,
      LLVMValueRef refLE);

  Weakability getKindWeakability(Kind* kind) override;

  ValeFuncPtrLE getInterfaceMethodFunctionPtr(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Reference* virtualParamMT,
      Ref virtualArgRef,
      int indexInEdge) override;

  Prototype* getFreePrototype(Kind* valeKind);
  Prototype* getFreeThunkPrototype(StructKind* structKind, InterfaceKind* interfaceKind);

  LLVMValueRef stackify(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Local* local,
      Ref refToStore,
      bool knownLive) override;

  Ref unstackify(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr) override;

  Ref loadLocal(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr) override;

  Ref localStore(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr, Ref refToStore, bool knownLive) override;

  void mainSetup(FunctionState* functionState, LLVMBuilderRef builder) override {}
  void mainCleanup(FunctionState* functionState, LLVMBuilderRef builder) override {}

  Reference* getRegionRefType() override;

  // This is only temporarily virtual, while we're still creating fake ones on the fly.
  // Soon it'll be non-virtual, and parameters will differ by region.
  Ref createRegionInstanceLocal(FunctionState* functionState, LLVMBuilderRef builder) override;

  Ref mutabilify(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* refMT,
      Ref ref,
      Reference* targetRefMT) override;

  LiveRef immutabilify(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Reference* refMT,
      Ref ref,
      Reference* targetRefMT) override;

private:
  void declareConcreteFreeFunction(Kind* valeKindM);
  void defineConcreteFreeFunction(Kind* valeKindM);
  void declareInterfaceFreeFunction(InterfaceKind* kind);
  void defineEdgeFreeFunction(Edge* edge);

public:
  Prototype* getAliasPrototype(Kind* valeKind);
  Prototype* getDealiasPrototype(Kind* valeKind);
  Prototype* getRefEqPrototype(Kind* valeKind);

  Prototype* getFieldGetterPrototype(StructKind* structKind, int memberIndex);
  void declareConcreteFieldGetter(StructDefinition* structDefM, int memberIndex);
  void defineConcreteFieldGetter(StructDefinition* structDefM, int memberIndex);

  Prototype* getStrLenPrototype();
  Prototype* getStrCharAtPrototype();
  void declareStrPrimitives();
  void defineStrPrimitives();

  Prototype* getTypeTagPrototype(InterfaceKind* interfaceKind);
  Prototype* getAsSubstructPrototype(InterfaceKind* interfaceKind, StructKind* structKind);
  Prototype* getUpcastPrototype(StructKind* structKind, InterfaceKind* interfaceKind);

  Prototype* getStructNewPrototype(StructKind* structKind);
  Prototype* getSsaNewPrototype(StaticSizedArrayT* ssaKind);

  Prototype* getRsaLenPrototype(RuntimeSizedArrayT* rsaKind);
  Prototype* getRsaAtPrototype(RuntimeSizedArrayT* rsaKind);
  Prototype* getSsaLenPrototype(StaticSizedArrayT* ssaKind);
  Prototype* getSsaAtPrototype(StaticSizedArrayT* ssaKind);

  // Read-only view of the edges declared for a given interface, in the order
  // they were seen at declareEdge time. Used by the auto-export loop and by
  // RCImm::generateInterfaceDefsC to emit TAG_* constants.
  const std::vector<Edge*>* getEdgesForInterface(InterfaceKind* interfaceKind);
private:
  void declareConcreteAliasFunction(Kind* valeKind);
  void defineConcreteAliasFunction(Kind* valeKind);
  void declareConcreteDealiasFunction(Kind* valeKind);
  void defineConcreteDealiasFunction(Kind* valeKind);
  void declareConcreteRefEqFunction(Kind* valeKind);
  void defineConcreteRefEqFunction(Kind* valeKind);

  void declareConcreteTypeTagFunction(InterfaceKind* interfaceKind);
  void defineConcreteTypeTagFunction(InterfaceKind* interfaceKind);
  void declareConcreteAsSubstructFunction(Edge* edge);
  void defineConcreteAsSubstructFunction(Edge* edge);
  void declareConcreteUpcastFunction(Edge* edge);
  void defineConcreteUpcastFunction(Edge* edge);

  void declareConcreteStructNewFunction(StructDefinition* structDefM);
  void defineConcreteStructNewFunction(StructDefinition* structDefM);
  void declareConcreteSsaNewFunction(StaticSizedArrayDefinitionT* ssaDef);
  void defineConcreteSsaNewFunction(StaticSizedArrayDefinitionT* ssaDef);

  void declareConcreteRsaLenFunction(RuntimeSizedArrayT* rsaKind);
  void defineConcreteRsaLenFunction(RuntimeSizedArrayDefinitionT* rsaDef);
  void declareConcreteRsaAtFunction(RuntimeSizedArrayT* rsaKind);
  void defineConcreteRsaAtFunction(RuntimeSizedArrayDefinitionT* rsaDef);
  void declareConcreteSsaLenFunction(StaticSizedArrayT* ssaKind);
  void defineConcreteSsaLenFunction(StaticSizedArrayDefinitionT* ssaDef);
  void declareConcreteSsaAtFunction(StaticSizedArrayT* ssaKind);
  void defineConcreteSsaAtFunction(StaticSizedArrayDefinitionT* ssaDef);

  InterfaceMethod* getFreeInterfaceMethod(Kind* valeKind);

  void callFree(
      FunctionState *functionState,
      LLVMBuilderRef builder,
      Ref regionInstanceRef,
      Kind* kind,
      Ref objectRef);

//  // Does the entire serialization process: measuring the length, allocating a buffer, and
//  // serializing into it.
//  Ref topLevelFree(
//      FunctionState* functionState,
//      LLVMBuilderRef builder,
//      Ref regionInstanceRef,
//      Ref sourceRegionInstanceRef,
//      Kind* valeKind,
//      Ref ref);

  Ref makeRegionInstance(LLVMBuilderRef builder);

  GlobalState* globalState = nullptr;

  KindStructs kindStructs;

  DefaultPrimitives primitives;

  std::string namePrefix = "__RCImm";

  StructKind* regionKind = nullptr;
  Reference* regionRefMT = nullptr;

  // Populated during declareEdge; maps interface kind -> ordered list of its
  // edges. Used by the typeTag/asSubstruct emitters and by
  // generateInterfaceDefsC to emit TAG_* constants in the same order the
  // typeTag body returns.
  // The AST provides edges per-struct (StructDefinition.edges) but not the
  // inverse: InterfaceDefinition has no implementing-edge list, so we build the
  // interface->edges index here. See todo/ffi-drop-followups.md for the frontend
  // enhancement that would let this be read straight off the AST.
  std::unordered_map<InterfaceKind*, std::vector<Edge*>, AddressHasher<InterfaceKind*>> edgesByInterface;
};

#endif
