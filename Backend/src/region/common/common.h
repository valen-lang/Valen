#ifndef REGION_COMMON_COMMON_H_
#define REGION_COMMON_COMMON_H_

#include "../../globalstate.h"
#include "../../function/function.h"
#include <llvm-c/Types.h>
#include "wrcweaks/wrcweaks.h"

LLVMValueRef weakStructPtrToGenWeakInterfacePtr(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef sourceRefLE,
    StructKind* sourceStructKindM,
    Kind* sourceStructTypeM,
    InterfaceKind* targetInterfaceKindM,
    Kind* targetInterfaceTypeM);

LLVMValueRef upcastThinPtr(
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* kindStructsSource,
    LLVMBuilderRef builder,

    Kind* sourceStructTypeM,
    StructKind* sourceStructKindM,
    WrapperPtrLE sourceRefLE,

    Kind* targetInterfaceTypeM,
    InterfaceKind* targetInterfaceKindM);

LLVMTypeRef translateReferenceSimple(GlobalState* globalState, KindStructs* structs, ValueKind* kind);

LLVMTypeRef translateWeakReference(GlobalState* globalState, KindStructs* weakRefStructs, ValueKind* kind);



LoadResult loadInnerInnerStructMember(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMTypeRef innerStructLT,
    LLVMValueRef innerStructPtrLE,
    int memberIndex,
    Kind* expectedType,
    std::string memberName);

void storeInnerInnerStructMember(
    LLVMBuilderRef builder,
    LLVMTypeRef innerStructLT,
    LLVMValueRef innerStructPtrLE,
    int memberIndex,
    std::string memberName,
    LLVMValueRef newValueLE);

LLVMValueRef getItablePtrFromInterfacePtr(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* virtualParamMT,
    InterfaceFatPtrLE virtualArgLE);


LLVMValueRef fillControlBlockCensusFields(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* structs,
    LLVMBuilderRef builder,
    ValueKind* kindM,
    LLVMValueRef newControlBlockLE,
    const std::string& typeName);

LLVMValueRef insertStrongRc(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    ValueKind* kindM,
    LLVMValueRef newControlBlockLE);

LLVMValueRef makeInterfaceRefStruct(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    StructKind* sourceStructKindM,
    InterfaceKind* targetInterfaceKindM,
    ControlBlockPtrLE controlBlockPtrLE);

LLVMValueRef makeInterfaceRefStruct(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    InterfaceKind* targetInterfaceKindM,
    LLVMValueRef objControlBlockPtrLE,
    LLVMValueRef itablePtrLE);

LLVMValueRef getTablePtrFromInterfaceRef(
    LLVMBuilderRef builder,
    InterfaceFatPtrLE interfaceFatPtrLE);

LLVMValueRef getObjPtrFromInterfaceRef(
    LLVMBuilderRef builder,
    InterfaceFatPtrLE interfaceRefLE);

void innerDeallocate(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* kindStrutsSource,
    LLVMBuilderRef builder,
    Kind* refMT,
    LiveRef ref);

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
    LiveRef rsaRef);

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
    LiveRef ssaRef);

std::tuple<Kind*, LLVMValueRef> megaGetRefInnardsForChecking(Ref ref);

LLVMValueRef callMalloc(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    LLVMValueRef sizeLE);

WrapperPtrLE mallocStr(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef lengthLE,
    LLVMValueRef sourceCharsPtrLE,
    KindStructs* kindStructs,
    std::function<void(LLVMBuilderRef builder, ControlBlockPtrLE controlBlockPtrLE)> fillControlBlock);
LLVMValueRef mallocKnownSize(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMTypeRef kindLT);
void fillInnerStruct(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    StructDefinition* structM,
    std::vector<Ref> membersLE,
    LLVMTypeRef innerStructLT,
    LLVMValueRef innerStructPtrLE);
LLVMValueRef constructInnerStruct(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    StructDefinition* structM,
    LLVMTypeRef valStructL,
    const std::vector<Ref>& memberRefs);
// Transmutes a weak ref of one ownership (such as borrow) to another ownership (such as weak).
Ref transmuteWeakRef(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* sourceWeakRefMT,
    Kind* targetWeakRefMT,
    KindStructs* weakRefStructs,
    Ref sourceWeakRef);

LLVMValueRef mallocRuntimeSizedArray(
    GlobalState* globalState,
    LLVMBuilderRef builder,
    LLVMTypeRef rsaWrapperLT,
    LLVMTypeRef rsaElementLT,
    LLVMValueRef lengthLE);

// Transmutes a ptr of one ownership (such as own) to another ownership (such as borrow).
Ref transmutePtr(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    bool expectLive,
    Kind* sourceRefMT,
    Kind* targetRefMT,
    Ref sourceRef);

//// Transmutes a ptr of one ownership (such as own) to another ownership (such as borrow).
//LiveRef transmuteLiveRef(
//    GlobalState* globalState,
//    FunctionState* functionState,
//    LLVMBuilderRef builder,
//    Kind* sourceRefMT,
//    Kind* targetRefMT,
//    LiveRef sourceRef);

Ref getRuntimeSizedArrayCapacity(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    WrapperPtrLE arrayRefLE);

ControlBlock makeFastWeakableControlBlock(GlobalState* globalState);
ControlBlock makeFastNonWeakableControlBlock(GlobalState* globalState);
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
    KindStructs* weakRefStructs);

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
    InterfaceKind *sourceInterfaceKind);
ControlBlock makeMutNonWeakableControlBlock(GlobalState* globalState, RegionId* regionId);
ControlBlock makeMutWeakableControlBlock(GlobalState* globalState, RegionId* regionId);
void fillStaticSizedArray(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Kind* ssaRefMT,
    StaticSizedArrayT* ssaMT,
    LiveRef ssaRef,
    const std::vector<Ref>& elementRefs);


void regularCheckValidReference(
    AreaAndFileAndLine checkerAFL,
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* refM,
    LLVMValueRef refLE);

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
//    Ref indexRef);


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
    std::function<void(LLVMBuilderRef builder, ControlBlockPtrLE controlBlockPtrLE)> fillControlBlock);

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
    const std::string& memberName);

LoadResult regularLoadMember(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* structRefMT,
    LiveRef structLiveRef,
    int memberIndex,
    Kind* expectedMemberType,
    Kind* targetType,
    const std::string& memberName);


Ref upcastStrong(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* sourceStructMT,
    StructKind* sourceStructKindM,
    Ref sourceRefLE,
    Kind* targetInterfaceTypeM,
    InterfaceKind* targetInterfaceKindM);

Ref upcastWeak(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* weakRefStructs,
    Kind* sourceStructMT,
    StructKind* sourceStructKindM,
    Ref sourceRefLE,
    Kind* targetInterfaceTypeM,
    InterfaceKind* targetInterfaceKindM);

void regularFillControlBlock(
    AreaAndFileAndLine from,
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* structs,
    LLVMBuilderRef builder,
    ValueKind* kindM,
    ControlBlockPtrLE controlBlockPtrLE,
    const std::string& typeName,
    WrcWeaks* wrcWeaks);

Ref getRuntimeSizedArrayLengthStrong(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* rsaRefMT,
    LiveRef arrayRef);

Ref getRuntimeSizedArrayCapacityStrong(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* rsaRefMT,
    LiveRef arrayRef);

std::tuple<LLVMValueRef, LLVMValueRef> explodeStrongInterfaceRef(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* virtualParamMT,
    Ref virtualArgRef);

std::tuple<LLVMValueRef, LLVMValueRef> explodeWeakInterfaceRef(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    FatWeaks* fatWeaks,
    KindStructs* weakRefStructs,
    Kind* virtualParamMT,
    Ref virtualArgRef,
    std::function<WeakFatPtrLE(WeakFatPtrLE weakInterfaceFatPtrLE)> weakInterfaceRefToWeakStructRef);

void storeMemberStrong(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* structRefMT,
    LiveRef structRef,
    int memberIndex,
    const std::string& memberName,
    LLVMValueRef newValueLE);

void storeMemberWeak(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* structRefMT,
    LiveRef structRef,
    int memberIndex,
    const std::string& memberName,
    LLVMValueRef newValueLE);


Ref regularWeakAlias(
    GlobalState* globalState,
    FunctionState* functionState,
    KindStructs* kindStructs,
    WrcWeaks* wrcWeaks,
    LLVMBuilderRef builder,
    Kind* sourceRefMT,
    Kind* targetRefMT,
    Ref sourceRef);

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
    FatWeaks* fatWeaks);

void callFree(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef ptrLE);


ValeFuncPtrLE getInterfaceMethodFunctionPtrFromItable(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* structs,
    Kind* virtualParamMT,
    Ref virtualArgRef,
    int indexInEdge);

Ref normalLocalLoad(
    GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr);

Ref normalLocalStore(GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, Local* local, LLVMValueRef localAddr, Ref refToStore);


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
    std::function<Ref(LLVMBuilderRef)> buildElse);

Ref regularReceiveAndDecryptFamiliarReference(
    GlobalState* globalState,
    FunctionState *functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind *sourceRefMT,
    LLVMValueRef sourceRefLE);

LLVMValueRef regularEncryptAndSendFamiliarReference(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    KindStructs* kindStructs,
    Kind* sourceRefMT,
    Ref sourceRef);

std::string generateConcreteHandleStructDefC(Package* currentPackage, const std::string& name);
std::string generateInterfaceHandleStructDefC(Package* currentPackage, const std::string& name);


void fastPanic(GlobalState* globalState, AreaAndFileAndLine from, LLVMBuilderRef builder);

#endif
