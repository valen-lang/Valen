#include <iostream>
#include "../../region/common/controlblock.h"
#include "shared/members.h"
#include "../../region/common/heap.h"

#include "../../translatetype.h"

#include "../expression.h"
#include "shared/shared.h"

Ref translateDestructure(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    Destroy* destructureM) {
  buildFlare(FL(), globalState, functionState, builder);

  auto structRef =
      translateExpression(
          globalState, functionState, blockState, builder, destructureM->structExpr);
  auto structLiveRef =
      globalState->getRegion(destructureM->structType)->checkRefLive(FL(),
          functionState, builder, destructureM->structType, structRef);
  // The struct arrives as its whole value in a register (a struct is its inner value now, not a
  // pointer), so copy each field straight out with extractvalue into the destination local.
  auto structRefLE =
      globalState->getRegion(destructureM->structType)->checkValidReference(FL(),
          functionState, builder, true, destructureM->structType, structRef);

  buildFlare(FL(), globalState, functionState, builder);

  auto structKind =
      dynamic_cast<StructKind *>(destructureM->structType);
  assert(structKind);

  auto structM = globalState->program->getStruct(structKind);

  for (int i = 0; i < structM->members.size(); i++) {
    buildFlare(FL(), globalState, functionState, builder);
    auto memberName = structM->members[i]->name;
    auto memberType = structM->members[i]->type;
    auto memberLE =
        toRef(globalState->getRegion(memberType), memberType,
            LLVMBuildExtractValue(builder, structRefLE, i, memberName.c_str()));
    makeHammerLocal(
        globalState, functionState, blockState, builder, destructureM->destinationLocals[i], memberLE);
    buildFlare(FL(), globalState, functionState, builder);
  }
  buildFlare(FL(), globalState, functionState, builder);

  if (isValueType(destructureM->structType)) {
    buildFlare(FL(), globalState, functionState, builder);
    globalState->getRegion(destructureM->structType)
        ->discardOwningRef(FL(), functionState, blockState, builder, destructureM->structType, structLiveRef);
  } else if (dynamic_cast<ShareRef*>(destructureM->structType) != nullptr) {
    buildFlare(FL(), globalState, functionState, builder);
    // We dont decrement anything here, we're only here because we already hit zero.

    globalState->getRegion(destructureM->structType)->deallocate(
        AFL("Destroy freeing"), functionState, builder,
        destructureM->structType, structLiveRef);
  } else {
    { assert(false); throw 1337; }
  }

  buildFlare(FL(), globalState, functionState, builder);

  return makeVoidRef(globalState);
}

Ref translateDestroySSAIntoLocals(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    DestroyStaticSizedArrayIntoLocals* destroySSAIntoLocalsM) {
  buildFlare(FL(), globalState, functionState, builder);

  auto structRef =
      translateExpression(
          globalState, functionState, blockState, builder, destroySSAIntoLocalsM->expr);
  auto structLiveRef =
      globalState->getRegion(destroySSAIntoLocalsM->staticSizedArray)->checkRefLive(FL(),
                                                                     functionState, builder, destroySSAIntoLocalsM->staticSizedArray, structRef);
  // The array arrives as its whole value in a register (a SINGLE array is its inner value now, not a
  // pointer), so move each element straight out with extractvalue into the destination local.
  auto arrayRefLE =
      globalState->getRegion(destroySSAIntoLocalsM->staticSizedArray)->checkValidReference(FL(),
          functionState, builder, true, destroySSAIntoLocalsM->staticSizedArray, structRef);

  auto ssaKind = destroySSAIntoLocalsM->staticSizedArray;
  assert(ssaKind);

  auto ssaM = globalState->program->getStaticSizedArray(ssaKind);

  for (int i = 0; i < ssaM->size; i++) {
    auto elementType = ssaM->elementType;
    auto memberLE =
        toRef(globalState->getRegion(elementType), elementType,
            LLVMBuildExtractValue(builder, arrayRefLE, i, ""));
    makeHammerLocal(
        globalState, functionState, blockState, builder, destroySSAIntoLocalsM->destinationLocals[i], memberLE);
  }
  buildFlare(FL(), globalState, functionState, builder);

  if (isValueType(destroySSAIntoLocalsM->staticSizedArray)) {
    buildFlare(FL(), globalState, functionState, builder);
    globalState->getRegion(destroySSAIntoLocalsM->staticSizedArray)
        ->discardOwningRef(FL(), functionState, blockState, builder, destroySSAIntoLocalsM->staticSizedArray, structLiveRef);
  } else if (dynamic_cast<ShareRef*>(destroySSAIntoLocalsM->staticSizedArray) != nullptr) {
    buildFlare(FL(), globalState, functionState, builder);
    // We dont decrement anything here, we're only here because we already hit zero.

    globalState->getRegion(destroySSAIntoLocalsM->staticSizedArray)->deallocate(
        AFL("Destroy freeing"), functionState, builder,
        destroySSAIntoLocalsM->staticSizedArray, structLiveRef);
  } else {
    { assert(false); throw 1337; }
  }

  buildFlare(FL(), globalState, functionState, builder);

  return makeVoidRef(globalState);
}
