#ifndef REGION_IREGION_H_
#define REGION_IREGION_H_

#include <llvm-c/Core.h>
#include "../function/expressions/shared/afl.h"
#include "../function/expressions/shared/ref.h"
#include "../metal/types.h"
#include "../metal/ast.h"
#include "../function/expressions/shared/elements.h"

class FunctionState;
class BlockState;

class IRegion {
public:
  virtual ~IRegion() = default;

  virtual Ref allocate(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* desiredStructMT,
      const std::vector<Ref>& memberRefs) = 0;

  virtual WrapperPtrLE lockWeakRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refM,
      Ref weakRefLE) = 0;

  virtual void alias(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* sourceRef,
      Ref expr) = 0;

  virtual void dealias(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* sourceMT,
      Ref sourceRef) = 0;

  virtual void storeMember(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* structRefMT,
      LiveRef structRef,
      int memberIndex,
      const std::string& memberName,
      Kind* newMemberRefMT,
      Ref newMemberRef) = 0;

  virtual Ref loadMember(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* structRefMT,
      LiveRef structRef,
      int memberIndex,
      Kind* expectedMemberType,
      Kind* targetMemberType,
      const std::string& memberName) = 0;

  virtual Ref upcastWeak(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      WeakFatPtrLE sourceRefLE,
      StructKind* sourceStructKindM,
      Kind* sourceStructTypeM,
      InterfaceKind* targetInterfaceKindM,
      Kind* targetInterfaceTypeM) = 0;

  virtual Ref upcast(
      FunctionState* functionState,
      LLVMBuilderRef builder,

      Kind* sourceStructMT,
      StructKind* sourceStructKindM,
      Ref sourceRefLE,

      Kind* targetInterfaceTypeM,
      InterfaceKind* targetInterfaceKindM) = 0;

  virtual Ref lockWeak(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      bool thenResultIsNever,
      bool elseResultIsNever,
      Kind* resultOptTypeM,
      Kind* constraintRefM,
      Kind* sourceWeakRefMT,
      Ref sourceWeakRefLE,
      std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
      std::function<Ref(LLVMBuilderRef)> buildElse) = 0;

  virtual Ref asSubtype(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* resultOptTypeM,
      Kind* sourceInterfaceRefMT,
      Ref sourceInterfaceRefLE,
      Kind* targetKind,
      std::function<Ref(LLVMBuilderRef, Ref)> buildThen,
      std::function<Ref(LLVMBuilderRef)> buildElse) = 0;

  virtual LiveRef constructStaticSizedArray(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* referenceM,
      StaticSizedArrayT* kindM) = 0;

  virtual LiveRef checkRefLive(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refMT,
      Ref ref) = 0;

    virtual LiveRef wrapToLiveRef(
        AreaAndFileAndLine checkerAFL,
        FunctionState* functionState,
        LLVMBuilderRef builder,
        Kind* refMT,
        LLVMValueRef ref) = 0;

  virtual LiveRef preCheckBorrow(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refMT,
      Ref ref) = 0;

  virtual Ref mutabilify(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refMT,
      Ref ref,
      Kind* targetRefMT) = 0;

  virtual LiveRef immutabilify(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refMT,
      Ref ref,
      Kind* targetRefMT) = 0;

  virtual Ref getRuntimeSizedArrayLength(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* rsaRefMT,
      LiveRef arrayRef) = 0;

  virtual Ref getRuntimeSizedArrayCapacity(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* rsaRefMT,
      LiveRef arrayRef) = 0;

  virtual LLVMValueRef checkValidReference(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      bool expectLive,
      Kind* refM,
      Ref ref) = 0;

  LLVMValueRef checkValidReference(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refM,
      LiveRef liveRef) {
    auto ref = toRef(this, refM, liveRef.refLE);
    return checkValidReference(checkerAFL, functionState, builder, true, refM, ref);
  }

  virtual LLVMValueRef getCensusObjectId(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refM,
      Ref ref) = 0;

  virtual LLVMTypeRef translateType(Kind* referenceM) = 0;


  virtual std::string getExportName(
      Package* package,
      Kind* reference,
      bool includeProjectName) = 0;

//  virtual std::string getMemberArbitraryRefNameCSeeMMEDT(
//      Kind* refMT) = 0;
  virtual std::string generateStructDefsC(
    Package* currentPackage,
      StructDefinition* refMT) = 0;
  virtual std::string generateInterfaceDefsC(
    Package* currentPackage,
      InterfaceDefinition* refMT) = 0;
  virtual std::string generateStaticSizedArrayDefsC(
    Package* currentPackage,
      StaticSizedArrayDefinitionT* ssaDefM) = 0;
  virtual std::string generateRuntimeSizedArrayDefsC(
    Package* currentPackage,
      RuntimeSizedArrayDefinitionT* rsaDefM) = 0;

  virtual void declareStruct(StructDefinition* structM) = 0;
  virtual void declareStructExtraFunctions(StructDefinition* structM) = 0;
  virtual void defineStruct(StructDefinition* structM) = 0;
  virtual void defineStructExtraFunctions(StructDefinition* structM) = 0;

  virtual void declareInterface(InterfaceDefinition* interfaceM) = 0;
  virtual void declareInterfaceExtraFunctions(InterfaceDefinition* structM) = 0;
  virtual void defineInterface(InterfaceDefinition* interfaceM) = 0;
  virtual void defineInterfaceExtraFunctions(InterfaceDefinition* structM) = 0;

  virtual void declareStaticSizedArray(StaticSizedArrayDefinitionT* staticSizedArrayDefinitionMT) = 0;
  virtual void declareStaticSizedArrayExtraFunctions(StaticSizedArrayDefinitionT* structM) = 0;
  virtual void defineStaticSizedArray(StaticSizedArrayDefinitionT* staticSizedArrayDefinitionMT) = 0;
  virtual void defineStaticSizedArrayExtraFunctions(StaticSizedArrayDefinitionT* structM) = 0;

  virtual void declareRuntimeSizedArray(RuntimeSizedArrayDefinitionT* runtimeSizedArrayDefinitionMT) = 0;
  virtual void declareRuntimeSizedArrayExtraFunctions(RuntimeSizedArrayDefinitionT* structM) = 0;
  virtual void defineRuntimeSizedArray(RuntimeSizedArrayDefinitionT* rsaDefM) = 0;
  virtual void defineRuntimeSizedArrayExtraFunctions(RuntimeSizedArrayDefinitionT* structM) = 0;

  virtual void declareEdge(Edge* edge) = 0;
  virtual void defineEdge(Edge* edge) = 0;

  virtual void declareExtraFunctions() = 0;
  virtual void defineExtraFunctions() = 0;


  virtual Ref weakAlias(FunctionState* functionState, LLVMBuilderRef builder, Kind* sourceRefMT, Kind* targetRefMT, Ref sourceRef) = 0;

  virtual void discardOwningRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      BlockState* blockState,
      LLVMBuilderRef builder,
      Kind* sourceMT,
      LiveRef sourceRef) = 0;

  virtual void noteWeakableDestroyed(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refM,
      ControlBlockPtrLE controlBlockPtrLE) = 0;

  // Gets the itable PTR and the new value that we should put into the virtual param's slot
  // (such as a void* or a weak void ref)
  virtual std::tuple<LLVMValueRef, LLVMValueRef> explodeInterfaceRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* virtualParamMT,
      Ref virtualArgRef) = 0;

  virtual ValeFuncPtrLE getInterfaceMethodFunctionPtr(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* virtualParamMT,
      Ref virtualArgRef,
      int indexInEdge) = 0;

  virtual void aliasWeakRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* weakRefMT,
      Ref weakRef) = 0;

  virtual void discardWeakRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* weakRefMT,
      Ref weakRef) = 0;

  virtual Ref getIsAliveFromWeakRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* weakRefM,
      Ref weakRef) = 0;

  virtual LoadResult loadElementFromRSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* rsaRefMT,
      RuntimeSizedArrayT* rsaMT,
      LiveRef structRef,
      InBoundsLE indexLE) = 0;

  virtual void deallocate(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refMT,
      LiveRef ref) = 0;


  virtual LiveRef constructRuntimeSizedArray(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* rsaMT,
      RuntimeSizedArrayT* runtimeSizedArrayT,
      Ref capacityRef,
      const std::string& typeName) = 0;

  virtual void pushRuntimeSizedArrayNoBoundsCheck(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* rsaRefMT,
      RuntimeSizedArrayT* rsaMT,
      LiveRef arrayRef,
      InBoundsLE sizeLE,
      Ref elementRef) = 0;

  virtual Ref popRuntimeSizedArrayNoBoundsCheck(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* rsaRefMT,
      RuntimeSizedArrayT* rsaMT,
      LiveRef arrayRef,
      InBoundsLE indexLE) = 0;

  virtual void initializeElementInSSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* ssaRefMT,
      StaticSizedArrayT* ssaMT,
      LiveRef structRef,
      InBoundsLE indexLE,
      Ref elementRef) = 0;

  virtual Ref deinitializeElementFromSSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* ssaRefMT,
      StaticSizedArrayT* ssaMT,
      LiveRef structRef,
      InBoundsLE indexLE) = 0;

  virtual Ref storeElementInRSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* rsaRefMT,
      RuntimeSizedArrayT* rsaMT,
      LiveRef structRef,
      InBoundsLE indexLE,
      Ref elementRef) = 0;

  virtual void checkInlineStructType(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refMT,
      Ref ref) = 0;

  virtual Ref upgradeLoadResultToRefWithTargetOwnership(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* sourceType,
      Kind* targetType,
      LoadResult sourceRef) = 0;

  // For instance regions, this will return the handle's type.
  // The C-ABI type a ref of this region crosses the FFI boundary as.
  virtual LLVMTypeRef getExternalType(Kind* refMT) = 0;

  virtual LoadResult loadElementFromSSA(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* ssaRefMT,
      StaticSizedArrayT* ssaMT,
      LiveRef structRef,
      InBoundsLE indexRef) = 0;

  // Receives and decrypts a reference to an object in this region.
  virtual Ref receiveAndDecryptFamiliarReference(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* sourceRefMT,
      LLVMValueRef sourceRefLE) = 0;

  // Encrypts and sends a reference to an object in this region.
  virtual LLVMValueRef encryptAndSendFamiliarReference(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* sourceRefMT,
      Ref sourceRef) = 0;

  virtual LLVMValueRef getStringBytesPtr(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refMT,
      LiveRef ref) = 0;
  virtual LLVMValueRef getStringLen(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refMT,
      LiveRef ref) = 0;
  // TODO:
  // One use is for makeNewStrFunc, make that private to the unsafe region.
  // Change this to also take in the bytes pointer.
  virtual Ref mallocStr(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      LLVMValueRef lengthLE,
      LLVMValueRef sourceCharsPtrLE) = 0;

  virtual LLVMTypeRef getInterfaceMethodVirtualParamAnyType(
      Kind* reference) = 0;

  virtual RegionId* getRegionId() = 0;

  virtual Weakability getKindWeakability(Kind* kind) = 0;

  virtual LLVMValueRef stackify(
      FunctionState* functionState, LLVMBuilderRef builder, Local* local, Ref refToStore) = 0;

  virtual Ref unstackify(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr) = 0;

  virtual Ref loadLocal(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr) = 0;

  virtual Ref localStore(FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr, Ref refToStore) = 0;

  virtual void mainSetup(FunctionState* functionState, LLVMBuilderRef builder) = 0;
  virtual void mainCleanup(FunctionState* functionState, LLVMBuilderRef builder) = 0;
};

LLVMValueRef checkValidReference(
    AreaAndFileAndLine checkerAFL,
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool expectLive,
    Kind* refM,
    Ref ref);

LLVMValueRef checkValidReference(
    AreaAndFileAndLine checkerAFL,
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool expectLive,
    Kind* refM,
    LiveRef liveRef);


#endif
