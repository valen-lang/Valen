#ifndef REGION_COMMON_WRCWEAKS_WRCWEAKS_H_
#define REGION_COMMON_WRCWEAKS_WRCWEAKS_H_

#include <llvm-c/Types.h>
#include "../../../globalstate.h"
#include "../../../function/function.h"
#include "../fatweaks/fatweaks.h"

class WrcWeaks {
public:
  WrcWeaks(GlobalState* globalState, KindStructs* kindStructsSource, KindStructs* weakRefStructsSource);

  WeakFatPtrLE weakStructPtrToWrciWeakInterfacePtr(
      GlobalState *globalState,
      FunctionState *functionState,
      LLVMBuilderRef builder,
      WeakFatPtrLE sourceRefLE,
      StructKind *sourceStructKindM,
      Kind *sourceStructTypeM,
      InterfaceKind *targetInterfaceKindM,
      Kind *targetInterfaceTypeM);

  Ref assembleWeakRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* sourceType,
      Kind* targetType,
      Ref sourceRef);

  // Makes a non-weak interface ref into a weak interface ref
  WeakFatPtrLE assembleInterfaceWeakRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* sourceType,
      Kind* targetType,
      InterfaceKind* interfaceKindM,
      InterfaceFatPtrLE sourceInterfaceFatPtrLE);

  WeakFatPtrLE assembleStructWeakRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* structTypeM,
      Kind* targetTypeM,
      StructKind* structKindM,
      WrapperPtrLE objPtrLE);

  WeakFatPtrLE assembleStaticSizedArrayWeakRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* sourceSSAMT,
      StaticSizedArrayT* staticSizedArrayMT,
      Kind* targetSSAWeakRefMT,
      WrapperPtrLE objPtrLE);

  WeakFatPtrLE assembleRuntimeSizedArrayWeakRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* sourceType,
      RuntimeSizedArrayT* runtimeSizedArrayMT,
      Kind* targetRSAWeakRefMT,
      WrapperPtrLE sourceRefLE);

  LLVMValueRef lockWrciFatPtr(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* refM,
      WeakFatPtrLE weakFatPtrLE);


  void innerNoteWeakableDestroyed(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* concreteRefM,
      ControlBlockPtrLE controlBlockPtrLE);

  LLVMValueRef getIsAliveFromWeakFatPtr(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      WeakFatPtrLE weakFatPtrLE);

  LLVMValueRef fillWeakableControlBlock(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      KindStructs* structs,
      ValueKind* kindM,
      LLVMValueRef controlBlockLE);

  WeakFatPtrLE weakInterfaceRefToWeakStructRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* weakInterfaceRefMT,
      WeakFatPtrLE weakInterfaceFatPtrLE);

  void aliasWeakRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* weakRefMT,
      Ref weakRef);

  void discardWeakRef(
      AreaAndFileAndLine from,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* weakRefMT,
      Ref weakRef);

  Ref getIsAliveFromWeakRef(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* weakRefM,
      Ref weakRef);

  void buildCheckWeakRef(
      AreaAndFileAndLine checkerAFL,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      Kind* weakRefM,
      Ref weakRef);

  LLVMValueRef getWrciFromWeakRef(
      LLVMBuilderRef builder,
      WeakFatPtrLE weakFatPtrLE);


  static LLVMTypeRef makeWeakRefHeaderStruct(GlobalState* globalState);


  void mainSetup(FunctionState* functionState, LLVMBuilderRef builder);
  void mainCleanup(FunctionState* functionState, LLVMBuilderRef builder);

private:
  void buildCheckWrc(
      LLVMBuilderRef builder,
      LLVMValueRef wrciLE);

  void maybeReleaseWrc(
      FunctionState* functionState,
      LLVMBuilderRef builder,
      LLVMValueRef wrciLE,
      LLVMValueRef ptrToWrcLE,
      LLVMValueRef wrcLE);

  LLVMValueRef getNewWrci(
      FunctionState* functionState,
      LLVMBuilderRef builder);

  LLVMValueRef getWrcPtr(
      LLVMBuilderRef builder,
      LLVMValueRef wrciLE);

  LLVMValueRef getWrcCapacityPtr(LLVMBuilderRef builder);
  LLVMValueRef getWrcFirstFreeWrciPtr(LLVMBuilderRef builder);
  LLVMValueRef getWrcEntriesArrayPtr(LLVMBuilderRef builder);


  GlobalState* globalState = nullptr;
  FatWeaks fatWeaks_;
  KindStructs* kindStructsSource;
  KindStructs* weakRefStructsSource;

  LLVMValueRef wrcTablePtrLE = nullptr;
};

#endif