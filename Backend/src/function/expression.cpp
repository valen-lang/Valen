#include <iostream>
#include "../region/common/common.h"
#include "../utils/branch.h"
#include "../utils/call.h"
#include "expressions/shared/elements.h"
#include "../region/common/controlblock.h"
#include "expressions/shared/members.h"
#include "../region/common/heap.h"

#include "../translatetype.h"

#include "expressions/expressions.h"
#include "expressions/shared/shared.h"
#include "expressions/shared/members.h"
#include "expression.h"

Ref translateExpressionInner(
    GlobalState* globalState,
    FunctionState* constraintRef,
    BlockState* blockState,
    LLVMBuilderRef builder,
    Expression* expr);

std::vector<Ref> translateExpressions(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    std::vector<Expression*> exprs) {
  auto result = std::vector<Ref>{};
  result.reserve(exprs.size());
  for (auto expr : exprs) {
    result.push_back(
        translateExpression(globalState, functionState, blockState, builder, expr));
  }
  return result;
}

Ref translateExpression(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    Expression* expr) {
  functionState->instructionDepthInAst++;
  auto resultLE = translateExpressionInner(globalState, functionState, blockState, builder, expr);
  functionState->instructionDepthInAst--;
  return resultLE;
}

Ref translateExpressionInner(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    Expression* expr) {
  if (auto constantInt = dynamic_cast<ConstantInt*>(expr)) {
    // See ULTMCIE for why we load and store here.
    auto resultLE = makeConstIntExpr(functionState, builder, LLVMIntTypeInContext(globalState->context, constantInt->bits), constantInt->value);
    auto intType =
        globalState->metalCache->getInt(globalState->metalCache->mutRegionId, constantInt->bits);
    return toRef(globalState->getRegion(intType), intType, resultLE);
  } else if (auto constantVoid = dynamic_cast<ConstantVoid*>(expr)) {
    // See ULTMCIE for why we load and store here.
    auto resultRef = makeVoidRef(globalState);
    auto resultLE =
        globalState->getRegion(globalState->metalCache->voidType)
            ->checkValidReference(FL(), functionState, builder, true, globalState->metalCache->voidType, resultRef);
    auto resultLT =
        globalState->getRegion(globalState->metalCache->voidType)
            ->translateType(globalState->metalCache->voidType);
    auto loadedLE = makeConstExpr(functionState, builder, resultLT, resultLE);
    return toRef(globalState->getRegion(globalState->metalCache->voidType), globalState->metalCache->voidType, loadedLE);
  } else if (auto constantFloat = dynamic_cast<ConstantF64*>(expr)) {
    // See ULTMCIE for why we load and store here.
    auto resultLT =
        globalState->getRegion(globalState->metalCache->floatType)
            ->translateType(globalState->metalCache->floatType);
    auto resultLE =
            makeConstExpr(
                functionState,
                builder,
                resultLT,
                LLVMConstReal(resultLT, constantFloat->value));
    return toRef(globalState->getRegion(globalState->metalCache->floatType), globalState->metalCache->floatType, resultLE);
  } else if (auto constantBool = dynamic_cast<ConstantBool*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    // See ULTMCIE for why this is an add.
    auto resultLE = makeConstIntExpr(functionState, builder, LLVMInt1TypeInContext(globalState->context), constantBool->value);
    return toRef(globalState->getRegion(globalState->metalCache->boolType), globalState->metalCache->boolType, resultLE);
  } else if (auto discardM = dynamic_cast<Discard*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    Ref result = translateDiscard(globalState, functionState, blockState, builder, discardM);
    return result;
  } else if (auto copyPrimM = dynamic_cast<CopyPrim*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    // We basically just use the incoming value as is, to make LLVM copy it.
    return translateExpression(globalState, functionState, blockState, builder, copyPrimM->inner);
  } else if (auto ret = dynamic_cast<Return*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto sourceRef = translateExpression(globalState, functionState, blockState, builder, ret->sourceExpr);
    if (ret->sourceType == globalState->metalCache->neverType) {
      return sourceRef;
    } else {
      auto toReturnLE =
          globalState->getRegion(ret->sourceType)
              ->checkValidReference(FL(), functionState, builder, false, ret->sourceType, sourceRef);
      LLVMBuildRet(builder, toReturnLE);
      return toRef(globalState->getRegion(globalState->metalCache->neverType), globalState->metalCache->neverType, globalState->neverLE);
    }
  } else if (auto breeak = dynamic_cast<Break*>(expr)) {
    if (auto nearestLoopBlockStateAndEnd = blockState->getNearestLoopEnd()) {
      auto [nearestLoopBlockState, nearestLoopEnd] = *nearestLoopBlockStateAndEnd;

      LLVMBuildBr(builder, nearestLoopEnd);

      return toRef(
        globalState->getRegion(globalState->metalCache->neverType), globalState->metalCache->neverType,
        globalState->neverLE);

//      buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
//      auto sourceRef = translateExpression(globalState, functionState, blockState, builder, ret->sourceExpr);
//      if (ret->sourceType->kind == globalState->metalCache->never) {
//        return sourceRef;
//      } else {
//        auto toReturnLE =
//            globalState->getRegion(ret->sourceType)
//                ->checkValidReference(FL(), functionState, builder, ret->sourceType, sourceRef);
//        LLVMBuildRet(builder, toReturnLE);
//        return toRef(
//            globalState->getRegion(globalState->metalCache->neverType), globalState->metalCache->neverType,
//            globalState->neverPtrLE);
//      }
    } else {
      std::cerr << "Error: found a break not inside a loop!" << std::endl;
      exit(1);
    }
  } else if (auto stackify = dynamic_cast<Stackify*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto refToStore =
        translateExpression(
            globalState, functionState, blockState, builder, stackify->expr);
    globalState->getRegion(stackify->variable->type)
        ->checkValidReference(FL(), functionState, builder, false, stackify->variable->type, refToStore);
    makeHammerLocal(
        globalState, functionState, blockState, builder, stackify->variable, refToStore);
    return makeVoidRef(globalState);
  } else if (auto restackify = dynamic_cast<Restackify*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    // The purpose of LocalStore is to put a swap value into a local, and give
    // what was in it.
    auto localAddr = blockState->getLocalAddr(restackify->variable->id, false);

    auto refToStore =
        translateExpression(
            globalState, functionState, blockState, builder, restackify->sourceExpr);

    // This needs to be after translating sourceExpr because it might be unstackified then, and then
    // we immediately restackify it after.
    blockState->restackify(restackify->variable->id);

    // We need to load the old ref *after* we evaluate the source expression,
    // Because of expressions like: Ship() = (mut b = (mut a = (mut b = Ship())));
    // See mutswaplocals.vale for test case.
    auto restackifyVariableValueType = peel_all_references(restackify->variable->type);
    auto oldRef =
        globalState->getRegion(restackifyVariableValueType)
            ->localStore(functionState, builder, restackify->variable, localAddr, refToStore);

    auto toStoreLE =
        globalState->getRegion(restackifyVariableValueType)->checkValidReference(FL(),
            functionState, builder, false, restackify->variable->type, refToStore);
    LLVMBuildStore(builder, toStoreLE, localAddr);
    return makeVoidRef(globalState);
  // } else if (auto localStore = dynamic_cast<LocalStore*>(expr)) {
  //   buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
  //   // The purpose of LocalStore is to put a swap value into a local, and give
  //   // what was in it.
  //   auto localAddr = blockState->getLocalAddr(localStore->local, true);
  //
  //   auto refToStore =
  //       translateExpression(
  //           globalState, functionState, blockState, builder, localStore->sourceExpr);
  //
  //   // We need to load the old ref *after* we evaluate the source expression,
  //   // Because of expressions like: Ship() = (mut b = (mut a = (mut b = Ship())));
  //   // See mutswaplocals.vale for test case.
  //   auto oldRef =
  //       globalState->getRegion(localStore->local->type)
  //           ->localStore(functionState, builder, localStore->local, localAddr, refToStore);
  //
  //   auto toStoreLE =
  //       globalState->getRegion(localStore->local->type)->checkValidReference(FL(),
  //           functionState, builder, false, localStore->local->type, refToStore);
  //   LLVMBuildStore(builder, toStoreLE, localAddr);
  //   return oldRef;
  } else if (auto weakAlias = dynamic_cast<WeakAlias*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());

    auto sourceRef =
        translateExpression(
            globalState, functionState, blockState, builder, weakAlias->innerExpr);

    auto weakAliasSourceValueType = peel_all_references(weakAlias->sourceType);
    globalState
        ->getRegion(weakAliasSourceValueType)
            ->checkValidReference(FL(), functionState, builder, false, weakAlias->sourceType, sourceRef);

    auto resultRef = globalState->getRegion(weakAliasSourceValueType)->weakAlias(functionState, builder, weakAlias->sourceType, weakAlias->result, sourceRef);
    globalState->getRegion(weakAlias->result)->aliasWeakRef(FL(), functionState, builder, weakAlias->result, resultRef);
    globalState->getRegion(weakAliasSourceValueType)->dealias(
        AFL("WeakAlias drop constraintref"),
        functionState, builder, weakAlias->sourceType, sourceRef);
    return resultRef;
  } else if (auto localLoad = dynamic_cast<LocalLoad*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name(), " ", localLoad->localName);

    return translateLocalLoad(globalState, functionState, blockState, builder, localLoad);
  } else if (auto unstackify = dynamic_cast<Unstackify*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    // The purpose of Unstackify is to destroy the local and give what was in
    // it, but in LLVM there's no instruction (or need) for destroying a local.
    // So, we just give what was in it. It's ironically identical to LocalLoad.
    auto localAddr = blockState->getLocalAddr(unstackify->variable->id, true);
    blockState->markLocalUnstackified(unstackify->variable->id);
    return globalState->getRegion(unstackify->variable->type)->unstackify(functionState, builder, unstackify->variable, localAddr);
  } else if (auto argument = dynamic_cast<Argument*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name(), " arg ", argument->paramIndex);
    // This +1 is because the 0th argument is always the next gen ptr, see RPPFNG.
    auto resultLE = functionState->getParam(UserArgIndex{argument->paramIndex});
    auto argumentValueType = peel_all_references(argument->tyype);
    auto resultRef = toRef(globalState->getRegion(argumentValueType), argument->tyype, resultLE);
    auto resultLT = globalState->getRegion(argumentValueType)->translateType(argument->tyype);
    globalState->getRegion(argumentValueType)
        ->checkValidReference(FL(), functionState, builder, false, argument->tyype, resultRef);
//    buildFlare(FL(), globalState, functionState, builder, "/", typeid(*expr).name());
    return resultRef;
  } else if (auto constantStr = dynamic_cast<ConstantStr*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto resultLE = translateConstantStr(FL(), globalState, functionState, builder, constantStr);
    return resultLE;
  } else if (auto newStruct = dynamic_cast<NewStruct*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto memberExprs =
        translateExpressions(
            globalState, functionState, blockState, builder, newStruct->args);
    auto resultLE =
        translateConstruct(
            AFL("NewStruct"), globalState, functionState, builder, newStruct->result, memberExprs);
    return resultLE;
  } else if (auto consecutor = dynamic_cast<Consecutor*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto exprs =
        translateExpressions(globalState, functionState, blockState, builder, consecutor->exprs);
    assert(!exprs.empty());
    return exprs.back();
  } else if (auto block = dynamic_cast<Block*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    return translateBlock(globalState, functionState, blockState, builder, block);
  } else if (auto iff = dynamic_cast<If*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    return translateIf(globalState, functionState, blockState, builder, iff);
  } else if (auto whiile = dynamic_cast<While*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    return translateWhile(globalState, functionState, blockState, builder, whiile);
  } else if (auto destructureM = dynamic_cast<Destroy*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    return translateDestructure(globalState, functionState, blockState, builder, destructureM);
  } else if (auto destroySSAIntoLocalsM = dynamic_cast<DestroyStaticSizedArrayIntoLocals*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    return translateDestroySSAIntoLocals(globalState, functionState, blockState, builder, destroySSAIntoLocalsM);
  } else if (auto memberLoad = dynamic_cast<MemberLoad*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name(), " ", memberLoad->memberName);
    auto structType = memberLoad->structType;
    auto structValueType = peel_all_references(structType);
    auto expectedResultValueType = peel_all_references(memberLoad->expectedResultType);

    auto structRef =
        translateExpression(
            globalState, functionState, blockState, builder, memberLoad->structExpr);
    auto memberIndex = memberLoad->memberIndex;
    auto memberName = memberLoad->memberName;

    auto structLiveRef =
        globalState->getRegion(structValueType)
            ->checkRefLive(FL(), functionState, builder, structType, structRef);

    auto resultRef =
        loadMember(
            AFL("MemberLoad"),
            globalState,
            functionState,
            builder,
            memberLoad->structType,
            structLiveRef,
            memberLoad->expectedMemberType,
            memberIndex,
            memberLoad->expectedResultType,
            memberName);
    globalState->getRegion(expectedResultValueType)
        ->checkValidReference(FL(), functionState, builder, false, memberLoad->expectedResultType, resultRef);
    if (memberLoad->expectedMemberType == globalState->metalCache->i32Type) {
      auto valueForPrintingLE =
          globalState->getRegion(expectedResultValueType)
              ->checkValidReference(FL(), functionState, builder, true, memberLoad->expectedResultType, resultRef);
      buildFlare(FL(), globalState, functionState, builder, "Loaded value: ", valueForPrintingLE);
    }

    globalState->getRegion(structValueType)->dealias(
        AFL("MemberLoad drop struct"),
        functionState, builder, memberLoad->structType, structRef);
    return resultRef;
  } else if (auto destroyStaticSizedArrayIntoFunction = dynamic_cast<DestroyStaticSizedArrayIntoFunction*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto arrayExpr = destroyStaticSizedArrayIntoFunction->arrayExpr;
    auto consumerExpr = destroyStaticSizedArrayIntoFunction->consumer;
    auto consumerMethod = destroyStaticSizedArrayIntoFunction->consumerMethod;
    auto arrayKind = destroyStaticSizedArrayIntoFunction->arrayType;
    auto arrayType = destroyStaticSizedArrayIntoFunction->arrayType;
    // The consume method is called as (consumer, element), so its params are those types.
    // VCOORD: thats icky, go back to having their types in DestroyStaticSizedArrayIntoFunction, if this survives.
    auto consumerType = consumerMethod->params[0];
    auto elementType = consumerMethod->params[1];
    int arraySize = globalState->program->getStaticSizedArray(arrayKind)->size;

    auto sizeLE = LLVMConstInt(LLVMInt32TypeInContext(globalState->context), arraySize, false);
    auto sizeRef =
        toRef(
            globalState->getRegion(globalState->metalCache->i32Type),
            globalState->metalCache->i32Type,
            sizeLE);

    auto arrayRef = translateExpression(globalState, functionState, blockState, builder, arrayExpr);

    auto consumerRef = translateExpression(globalState, functionState, blockState, builder, consumerExpr);
    auto consumerValueType = peel_all_references(consumerType);
    globalState->getRegion(consumerValueType)
        ->checkValidReference(FL(), functionState, builder, true, consumerType, consumerRef);

    auto arrayLiveRef =
        globalState->getRegion(arrayType)
            ->checkRefLive(FL(), functionState, builder, arrayType, arrayRef);

    intRangeLoopReverse(
        globalState, functionState, builder, globalState->metalCache->i32Type, sizeLE,
        [globalState, functionState, elementType, consumerType, consumerValueType, consumerMethod, arrayType, arrayKind, consumerRef, arrayLiveRef](
            LLVMValueRef indexLE, LLVMBuilderRef bodyBuilder) {
          // We know it's in bounds because we used size as a bound for the loop.
          auto inBoundsIndexLE = InBoundsLE{indexLE};

          globalState->getRegion(consumerValueType)->alias(
              AFL("DestroySSAIntoF consume iteration"),
              functionState, bodyBuilder, consumerType, consumerRef);

          auto elementLoadResult =
              globalState->getRegion(arrayType)->loadElementFromSSA(
                  functionState, bodyBuilder, arrayType, arrayKind,
                  arrayLiveRef,
                  inBoundsIndexLE);
          auto elementRef = elementLoadResult.move();

          globalState->getRegion(elementType)
              ->checkValidReference(
                  FL(), functionState, bodyBuilder, false, elementType, elementRef);
          std::vector<Ref> argExprRefs = {consumerRef, elementRef};

          buildCallV(globalState, functionState, bodyBuilder, consumerMethod, argExprRefs);
        });

    if (isValueType(arrayType)) {
      globalState->getRegion(arrayType)
          ->discardOwningRef(FL(), functionState, blockState, builder, arrayType, arrayLiveRef);
    } else if (dynamic_cast<ShareRef*>(arrayType) != nullptr) {
      // We dont decrement anything here, we're only here because we already hit zero.

      globalState->getRegion(arrayType)
          ->deallocate(
              AFL("DestroySSAIntoF"), functionState, builder, arrayType, arrayLiveRef);
    } else {
      { assert(false); throw 1337; }
    }

    globalState->getRegion(consumerValueType)
        ->dealias(
            AFL("DestroySSAIntoF"), functionState, builder, consumerType, consumerRef);

    return makeVoidRef(globalState);
  } else if (auto pushRuntimeSizedArray = dynamic_cast<PushRuntimeSizedArray*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto arrayExpr = pushRuntimeSizedArray->arrayExpr;
    auto arrayType = pushRuntimeSizedArray->arrayType;
    auto arrayValueType = peel_all_references(arrayType);
    auto arrayMT = dynamic_cast<RuntimeSizedArrayT*>(arrayValueType);
    assert(arrayMT);
    // This is true because this instruction only appears in the push func which takes in a pre&.
    auto newcomerExpr = pushRuntimeSizedArray->newElementExpr;
    auto newcomerType = pushRuntimeSizedArray->elementType;

    auto arrayRef = translateExpression(globalState, functionState, blockState, builder, arrayExpr);
    globalState->getRegion(arrayValueType)
        ->checkValidReference(FL(), functionState, builder, true, arrayType, arrayRef);

    auto arrayLiveRef =
        globalState->getRegion(arrayValueType)
            ->checkRefLive(FL(), functionState, builder, arrayType, arrayRef);

    auto arrayLenRef =
        globalState->getRegion(arrayValueType)
            ->getRuntimeSizedArrayLength(
                functionState, builder, arrayType, arrayLiveRef);
    auto arrayLenLE =
        globalState->getRegion(globalState->metalCache->i32Type)
            ->checkValidReference(FL(),
                functionState, builder, true, globalState->metalCache->i32Type, arrayLenRef);

    auto arrayCapacityRef =
        globalState->getRegion(arrayValueType)
            ->getRuntimeSizedArrayCapacity(
                functionState, builder, arrayType, arrayLiveRef);

    auto sizeInBoundsLE = checkIndexInBounds(globalState, functionState, builder, globalState->metalCache->i32Type, arrayCapacityRef, arrayLenLE, "Error: Array has no room for new element!");

    auto newcomerRef = translateExpression(globalState, functionState, blockState, builder, newcomerExpr);
    globalState->getRegion(newcomerType)
        ->checkValidReference(FL(), functionState, builder, true, newcomerType, newcomerRef);

    globalState->getRegion(arrayValueType)
        ->pushRuntimeSizedArrayNoBoundsCheck(
            functionState, builder, arrayType, arrayMT, arrayLiveRef, sizeInBoundsLE, newcomerRef);

    globalState->getRegion(arrayValueType)
        ->dealias(
            AFL("pushRuntimeSizedArrayNoBoundsCheck"), functionState, builder, arrayType, arrayRef);

    return makeVoidRef(globalState);
  } else if (auto popRuntimeSizedArray = dynamic_cast<PopRuntimeSizedArray*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto rsaME = popRuntimeSizedArray->arrayExpr;
    auto rsaRefMT = popRuntimeSizedArray->arrayType;
    auto rsaRefValueType = peel_all_references(rsaRefMT);
    auto rsaMT = dynamic_cast<RuntimeSizedArrayT*>(rsaRefValueType);
    assert(rsaMT);

    auto arrayRef = translateExpression(globalState, functionState, blockState, builder, rsaME);
    globalState->getRegion(rsaRefValueType)
        ->checkValidReference(FL(), functionState, builder, true, rsaRefMT, arrayRef);
    auto rsaLT = globalState->getRegion(rsaRefValueType)->translateType(rsaRefMT);

    auto arrayLiveRef =
        globalState->getRegion(rsaRefValueType)
            ->checkRefLive(FL(), functionState, builder, rsaRefMT, arrayRef);

    auto arrayLenRef =
        globalState->getRegion(rsaRefValueType)
            ->getRuntimeSizedArrayLength(
                functionState, builder, rsaRefMT, arrayLiveRef);
    auto arrayLenLE =
        globalState->getRegion(globalState->metalCache->i32Type)
            ->checkValidReference(FL(),
                functionState, builder, true, globalState->metalCache->i32Type, arrayLenRef);

    auto indexLE = LLVMBuildSub(builder, arrayLenLE, constI32LE(globalState, 1), "index");
    auto indexRef =
        toRef(globalState->getRegion(globalState->metalCache->i32Type), globalState->metalCache->i32Type, indexLE);

    auto indexInBoundsLE =
        checkLastElementExists(
            globalState, functionState, builder, arrayLenRef,
            "Error: Cannot pop element from empty array!");

    auto resultRef =
        globalState->getRegion(rsaRefValueType)
            ->popRuntimeSizedArrayNoBoundsCheck(
                functionState, builder, rsaRefMT, rsaMT, arrayLiveRef, indexInBoundsLE);

    globalState->getRegion(rsaRefValueType)
        ->dealias(
            AFL("popRuntimeSizedArrayNoBoundsCheck"), functionState, builder, rsaRefMT, arrayRef);

    return resultRef;
  } else if (auto dmrsa = dynamic_cast<DestroyRuntimeSizedArray*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto arrayExpr = dmrsa->arrayExpr;
    auto arrayType = dmrsa->arrayType;
    auto arrayValueType = peel_all_references(arrayType);

    auto arrayRef = translateExpression(globalState, functionState, blockState, builder, arrayExpr);

    auto arrayLiveRef =
        globalState->getRegion(arrayValueType)
            ->checkRefLive(FL(), functionState, builder, arrayType, arrayRef);

    globalState->getRegion(arrayValueType)
        ->checkValidReference(FL(), functionState, builder, true, arrayType, arrayRef);
    auto arrayLenRef =
        globalState->getRegion(arrayValueType)
            ->getRuntimeSizedArrayLength(
                functionState, builder, arrayType, arrayLiveRef);

    checkArrayEmpty(globalState, functionState, builder, arrayLenRef, "Error: Destroying non-empty array!");

    if (isValueType(arrayType)) {
      globalState->getRegion(arrayValueType)
          ->discardOwningRef(FL(), functionState, blockState, builder, arrayType, arrayLiveRef);
    } else if (dynamic_cast<ShareRef*>(arrayType) != nullptr) {
      // We dont decrement anything here, we're only here because we already hit zero.

      // Free it!
      globalState->getRegion(arrayValueType)
          ->deallocate(
              AFL("DestroyRSAIntoF"), functionState, builder, arrayType, arrayLiveRef);
    } else {
      { assert(false); throw 1337; }
    }

    return makeVoidRef(globalState);
  } else if (auto staticSizedArrayLoad = dynamic_cast<StaticSizedArrayLookup*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto arrayType = staticSizedArrayLoad->arrayType;
    auto arrayValueType = peel_all_references(arrayType);
    auto arrayExpr = staticSizedArrayLoad->arrayExpr;
    auto indexExpr = staticSizedArrayLoad->indexExpr;
    auto arrayKind = dynamic_cast<StaticSizedArrayT*>(staticSizedArrayLoad->arrayType->inner);
    auto ssaDef = globalState->program->getStaticSizedArray(arrayKind);
    auto elementType = ssaDef->elementType;
    auto elementValueType = peel_all_references(elementType);
    auto resultType = staticSizedArrayLoad->result;
    auto resultValueType = peel_all_references(resultType);
    int arraySize = ssaDef->size;

    auto arrayRef = translateExpression(globalState, functionState, blockState, builder, arrayExpr);



    globalState->getRegion(arrayValueType)
        ->checkValidReference(FL(), functionState, builder, true, arrayType, arrayRef);
    auto sizeLE =
        toRef(
            globalState->getRegion(globalState->metalCache->i32Type),
            globalState->metalCache->i32Type,
            constI32LE(globalState, arraySize));
    auto indexRef = translateExpression(globalState, functionState, blockState, builder, indexExpr);
    globalState->getRegion(arrayValueType)
        ->dealias(AFL("SSALoad"), functionState, builder, arrayType, arrayRef);

    auto arrayLiveRef =
        globalState->getRegion(arrayValueType)
            ->checkRefLive(FL(), functionState, builder, arrayType, arrayRef);

    auto intMT = globalState->metalCache->i32Type;
    auto indexLE =
        globalState->getRegion(intMT)
            ->checkValidReference(
                FL(), functionState, builder, false, intMT, indexRef);

    auto indexInBoundsLE =
        checkIndexInBounds(
            globalState, functionState, builder, intMT, sizeLE, indexLE,
            "Error: Array index out of bounds!");

    auto loadResult =
        globalState->getRegion(arrayValueType)
            ->loadElementFromSSA(
                functionState, builder, arrayType, arrayKind, arrayLiveRef,
                indexInBoundsLE);
    auto resultRef =
        globalState->getRegion(resultValueType)
            ->upgradeLoadResultToRefWithTargetOwnership(
                functionState, builder, elementType, resultType, loadResult);
    globalState->getRegion(resultValueType)
        ->checkValidReference(FL(), functionState, builder, false, resultType, resultRef);
    globalState->getRegion(elementValueType)
        ->alias(FL(), functionState, builder, resultType, resultRef);
    globalState->getRegion(elementValueType)
        ->checkValidReference(FL(), functionState, builder, false, resultType, resultRef);
    return resultRef;
  } else if (auto runtimeSizedArrayLoad = dynamic_cast<RuntimeSizedArrayLookup*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto arrayType = runtimeSizedArrayLoad->arrayType;
    auto arrayValueType = peel_all_references(arrayType);
    auto arrayExpr = runtimeSizedArrayLoad->arrayExpr;
    auto indexExpr = runtimeSizedArrayLoad->indexExpr;
    auto arrayKind = dynamic_cast<RuntimeSizedArrayT*>(runtimeSizedArrayLoad->arrayType->inner);
    auto elementType = globalState->program->getRuntimeSizedArray(arrayKind)->elementType;
    auto resultType = runtimeSizedArrayLoad->result;
    auto resultValueType = peel_all_references(resultType);

    auto arrayRef = translateExpression(globalState, functionState, blockState, builder, arrayExpr);

    auto arrayLiveRef =
        globalState->getRegion(arrayValueType)
            ->checkRefLive(FL(), functionState, builder, arrayType, arrayRef);

    globalState->getRegion(arrayValueType)
        ->checkValidReference(FL(), functionState, builder, true, arrayType, arrayRef);

    auto sizeRef =
        globalState->getRegion(arrayValueType)->getRuntimeSizedArrayLength(
            functionState, builder, arrayType, arrayLiveRef);
    auto indexRef = translateExpression(globalState, functionState, blockState, builder, indexExpr);
    auto indexLE =
        globalState->getRegion(globalState->metalCache->i32Type)
            ->checkValidReference(FL(), functionState, builder, false, globalState->metalCache->i32Type, indexRef);
    auto indexInBoundsLE =
        checkIndexInBounds(
            globalState, functionState, builder, globalState->metalCache->i32Type, sizeRef, indexLE,
            "Error: Array index out of bounds!");

    auto loadResult =
        globalState->getRegion(arrayValueType)->loadElementFromRSA(
            functionState, builder, arrayType, arrayKind, arrayLiveRef, indexInBoundsLE);
    auto resultRef =
        globalState->getRegion(elementType)
            ->upgradeLoadResultToRefWithTargetOwnership(
                functionState, builder, elementType, resultType, loadResult);

    globalState->getRegion(resultValueType)
        ->alias(FL(), functionState, builder, resultType, resultRef);

    globalState->getRegion(resultValueType)
        ->checkValidReference(FL(), functionState, builder, false, resultType, resultRef);

    globalState->getRegion(arrayValueType)
        ->dealias(AFL("RSALoad"), functionState, builder, arrayType, arrayRef);

    return resultRef;
  // } else if (auto runtimeSizedArrayStore = dynamic_cast<RuntimeSizedArrayStore*>(expr)) {
  //   buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
  //   auto arrayType = runtimeSizedArrayStore->arrayType;
  //   auto arrayExpr = runtimeSizedArrayStore->arrayExpr;
  //   auto indexExpr = runtimeSizedArrayStore->indexExpr;
  //   auto arrayKind = runtimeSizedArrayStore->arrayKind;
  //
  //   auto elementType = globalState->program->getRuntimeSizedArray(arrayKind)->elementType;
  //
  //   auto arrayRef = translateExpression(globalState, functionState, blockState, builder, arrayExpr);
  //   globalState->getRegion(arrayType)
  //       ->checkValidReference(FL(), functionState, builder, true, arrayType, arrayRef);
  //
  //   auto arrayLiveRef =
  //       globalState->getRegion(arrayType)
  //           ->checkRefLive(FL(), functionState, builder, arrayType, arrayRef);
  //
  //   auto sizeRef =
  //       globalState->getRegion(arrayType)
  //           ->getRuntimeSizedArrayLength(
  //               functionState, builder, arrayType, arrayLiveRef);
  //   auto sizeLE =
  //       globalState->getRegion(globalState->metalCache->i32Type)
  //           ->checkValidReference(FL(), functionState, builder, true, globalState->metalCache->i32Type, sizeRef);
  //
  //
  //   auto indexRef =
  //       translateExpression(globalState, functionState, blockState, builder, indexExpr);
  //   auto indexLE =
  //       globalState->getRegion(globalState->metalCache->i32Type)
  //           ->checkValidReference(FL(), functionState, builder, true, globalState->metalCache->i32Type, indexRef);
  //
  //   auto sharedness = ownershipToSharedness(arrayType->ownership);
  //
  //   auto indexInBoundsLE =
  //       checkIndexInBounds(
  //           globalState, functionState, builder, globalState->metalCache->i32Type, sizeRef, indexLE,
  //           "Error: Array index out of bounds!");
  //
  //   // The purpose of RuntimeSizedArrayStore is to put a swap value into a spot, and give
  //   // what was in it.
  //
  //   auto valueToStoreLE =
  //       translateExpression(
  //           globalState, functionState, blockState, builder, runtimeSizedArrayStore->sourceExpr);
  //
  //   globalState->getRegion(elementType)
  //       ->checkValidReference(FL(), functionState, builder, false, elementType, valueToStoreLE);
  //
  //   auto loadResult =
  //       globalState->getRegion(arrayType)->
  //           loadElementFromRSA(
  //               functionState, builder, arrayType, arrayKind, arrayLiveRef, indexInBoundsLE);
  //   auto oldValueLE = loadResult.move();
  //   globalState->getRegion(elementType)
  //       ->checkValidReference(FL(), functionState, builder, false, elementType, oldValueLE);
  //   // We dont acquireReference here because we aren't aliasing the reference, we're moving it out.
  //
  //   globalState->getRegion(arrayType)
  //       ->storeElementInRSA(
  //           functionState, builder,
  //           arrayType, arrayKind, arrayLiveRef, indexInBoundsLE, valueToStoreLE);
  //
  //   globalState->getRegion(arrayType)
  //       ->dealias(AFL("RSAStore"), functionState, builder, arrayType, arrayRef);
  //
  //   return oldValueLE;
  } else if (auto arrayLength = dynamic_cast<ArrayLength*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto arrayType = arrayLength->arrayType;
    auto arrayValueType = peel_all_references(arrayType);
    auto arrayExpr = arrayLength->arrayExpr;

    auto arrayRef =
        translateExpression(globalState, functionState, blockState, builder, arrayExpr);
    globalState->getRegion(arrayValueType)
        ->checkValidReference(FL(), functionState, builder, true, arrayType, arrayRef);

    auto arrayLiveRef =
        globalState->getRegion(arrayValueType)
            ->checkRefLive(FL(), functionState, builder, arrayType, arrayRef);

    auto sizeLE =
        globalState->getRegion(arrayValueType)
            ->getRuntimeSizedArrayLength(
                functionState, builder, arrayType, arrayLiveRef);
    globalState->getRegion(arrayValueType)
        ->dealias(AFL("RSALen"), functionState, builder, arrayType, arrayRef);

    return sizeLE;
  } else if (auto arrayCapacity = dynamic_cast<ArrayCapacity*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto arrayType = arrayCapacity->arrayType;
    auto arrayValueType = peel_all_references(arrayType);
    auto arrayExpr = arrayCapacity->arrayExpr;
//    auto indexExpr = arrayLength->indexExpr;

    auto arrayRef = translateExpression(globalState, functionState, blockState, builder, arrayExpr);
    globalState->getRegion(arrayValueType)
        ->checkValidReference(FL(), functionState, builder, true, arrayType, arrayRef);

    auto arrayLiveRef =
        globalState->getRegion(arrayValueType)
            ->checkRefLive(FL(), functionState, builder, arrayType, arrayRef);

    auto sizeLE =
        globalState->getRegion(arrayValueType)
            ->getRuntimeSizedArrayCapacity(
                functionState, builder, arrayType, arrayLiveRef);
    globalState->getRegion(arrayValueType)
        ->dealias(AFL("RSACapacity"), functionState, builder, arrayType, arrayRef);

    return sizeLE;
  // } else if (auto narrowPermission = dynamic_cast<NarrowPermission*>(expr)) {
  //   buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
  //   auto sourceExpr = narrowPermission->sourceExpr;
  //   return translateExpression(globalState, functionState, blockState, builder, sourceExpr);
  } else if (auto newArrayFromValues = dynamic_cast<NewArrayFromValues*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    return translateNewArrayFromValues(globalState, functionState, blockState, builder, newArrayFromValues);
  } else if (auto nmrsa = dynamic_cast<NewRuntimeSizedArray*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    return translateNewRuntimeSizedArray(globalState, functionState, blockState, builder, nmrsa);
  } else if (auto staticArrayFromCallable = dynamic_cast<StaticArrayFromCallable*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    return translateStaticArrayFromCallable(globalState, functionState, blockState, builder, staticArrayFromCallable);
  } else if (auto call = dynamic_cast<Call*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name(), " ", call->callable->name->name);
    auto resultLE = translateCall(globalState, functionState, blockState, builder, call);
//    buildFlare(FL(), globalState, functionState, builder, "/", typeid(*expr).name(), " ", call->function->name->name);
    return resultLE;
  } else if (auto externCall = dynamic_cast<ExternCall*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    auto resultLE = translateExternCall(globalState, functionState, blockState, builder, externCall);
    return resultLE;
  } else if (auto interfaceCall = dynamic_cast<InterfaceCall*>(expr)) {
    buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name(), " ", interfaceCall->superFunctionPrototype->name->name);
    auto resultLE = translateInterfaceCall(globalState, functionState, blockState, builder, interfaceCall);
//    if (interfaceCall->functionType->returnType->kind != globalState->metalCache->never) {
//      buildFlare(FL(), globalState, functionState, builder, "/", typeid(*expr).name());
//    }
    return resultLE;
  // } else if (auto memberStore = dynamic_cast<MemberStore*>(expr)) {
  //   buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
  //   auto structKind =
  //       dynamic_cast<StructKind*>(memberStore->structType->kind);
  //   auto structDefM = globalState->program->getStruct(structKind);
  //   auto memberIndex = memberStore->memberIndex;
  //   auto memberName = memberStore->memberName;
  //   auto structType = memberStore->structType;
  //   auto memberType = structDefM->members[memberIndex]->type;
  //
  //   auto sourceExpr =
  //       translateExpression(
  //           globalState, functionState, blockState, builder, memberStore->sourceExpr);
  //   globalState->getRegion(memberType)
  //       ->checkValidReference(FL(), functionState, builder, false, memberType, sourceExpr);
  //
  //   auto structRef =
  //       translateExpression(
  //           globalState, functionState, blockState, builder, memberStore->structExpr);
  //   globalState->getRegion(memberStore->structType)
  //       ->checkValidReference(FL(), functionState, builder, true, memberStore->structType, structRef);
  //
  //   auto structLiveRef =
  //       globalState->getRegion(structType)
  //           ->checkRefLive(FL(), functionState, builder, structRegionInstanceRef, structType, structRef);
  //
  //   auto oldMemberLE =
  //       swapMember(
  //           globalState, functionState, builder, structRegionInstanceRef, structDefM, structType, structLiveRef, memberIndex, memberName, sourceExpr);
  //   globalState->getRegion(memberType)
  //       ->checkValidReference(FL(), functionState, builder, false, memberType, oldMemberLE);
  //   globalState->getRegion(structType)
  //       ->dealias(
  //           AFL("MemberStore discard struct"),
  //           functionState, builder, structType, structRef);
  //   return oldMemberLE;
  } else if (auto structToInterfaceUpcast = dynamic_cast<StructToInterfaceUpcast*>(expr)) {
      std::cout << "StructToInterfaceUpcast unimplemented" << std::endl;
      { assert(false); throw 1337; }

    // buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    // auto sourceLE =
    //     translateExpression(
    //         globalState, functionState, blockState, builder, structToInterfaceUpcast->sourceExpr);
    // globalState->getRegion(structToInterfaceUpcast->sourceStructType)
    //     ->checkValidReference(
    //         FL(), functionState, builder, false, structToInterfaceUpcast->sourceStructType, sourceLE);
    //
    // // If it was inline before, upgrade it to a yonder struct.
    // // This however also means that small imm virtual params must be pointers,
    // // and value-ify themselves immediately inside their bodies.
    // // If the receiver expects a yonder, then they'll assume its on the heap.
    // // But if receiver expects an inl, its in a register.
    // // But we can only interfacecall with a yonder.
    // // So we need a thunk to receive that yonder, copy it, fire it into the
    // // real function.
    // // fuck... thunks. didnt want to do that.
    //
    // // alternative:
    // // what if we made it so someone receiving an override of an imm inl interface
    // // just takes in that much memory? it really just means a bit of wasted stack
    // // space, but it means we wouldnt need any thunking.
    // // It also means we wouldnt need any heap allocating.
    // // So, the override function will receive the entire interface, and just
    // // assume that the right thing is in there.
    // // Any callers will also have to wrap in an interface. but theyre copying
    // // anyway so should be fine.
    //
    // // alternative:
    // // only inline primitives. Which cant have interfaces anyway.
    // // maybe the best solution for now?
    //
    // // maybe function params that are inl can take a pointer, and they can
    // // just copy it immediately?
    //
    // return globalState->getRegion(structToInterfaceUpcast->sourceStructType)
    //     ->upcast(
    //         functionState,
    //         builder,
    //         structToInterfaceUpcast->sourceStructType,
    //         structToInterfaceUpcast->sourceStructKind,
    //         sourceLE,
    //         structToInterfaceUpcast->targetInterfaceType,
    //         structToInterfaceUpcast->targetInterfaceKind);
  } else if (auto lockWeak = dynamic_cast<LockWeak*>(expr)) {
      std::cout << "LockWeak unimplemented" << std::endl;
      { assert(false); throw 1337; }

    // buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
    //
    // auto sourceType = lockWeak->sourceType;
    // auto sourceLE =
    //     translateExpression(
    //         globalState, functionState, blockState, builder, lockWeak->sourceExpr);
    // globalState->getRegion(sourceType)
    //     ->checkValidReference(FL(), functionState, builder, false, sourceType, sourceLE);
    //
    // auto sourceTypeAsConstraintRefM =
    //     globalState->metalCache->getReference(
    //         Ownership::MUTABLE_BORROW,
    //         sourceType->location,
    //         sourceType->kind);
    //
    // auto resultOptTypeLE =
    //     globalState->getRegion(lockWeak->resultOptType)
    //         ->translateType(lockWeak->resultOptType);
    //
    // auto resultOptLE =
    //     globalState->getRegion(sourceType)->lockWeak(
    //         functionState, builder,
    //         false, false, lockWeak->resultOptType,
    //         sourceTypeAsConstraintRefM,
    //         sourceType,
    //         sourceLE,
    //         [globalState, functionState, lockWeak, sourceLE](LLVMBuilderRef thenBuilder, Ref constraintRef) -> Ref {
    //           globalState->getRegion(lockWeak->someConstructor->params[0])
    //               ->checkValidReference(
    //                   FL(), functionState, thenBuilder, false,
    //                   lockWeak->someConstructor->params[0],
    //                   constraintRef);
    //           globalState->getRegion(lockWeak->someConstructor->params[0])
    //               ->alias(
    //                   FL(), functionState, thenBuilder,
    //                   lockWeak->someConstructor->params[0],
    //                   constraintRef);
    //           // If we get here, object is alive, return a Some.
    //           auto someRef =
    //               buildCallV(globalState, functionState, thenBuilder, lockWeak->someConstructor, {constraintRef});
    //           globalState->getRegion(lockWeak->someType)
    //               ->checkValidReference(
    //                   FL(), functionState, thenBuilder, true, lockWeak->someType, someRef);
    //           return globalState->getRegion(lockWeak->someType)
    //               ->upcast(
    //                   functionState,
    //                   thenBuilder,
    //                   lockWeak->someType,
    //                   lockWeak->someKind,
    //                   someRef,
    //                   lockWeak->resultOptType,
    //                   lockWeak->resultOptKind);
    //         },
    //         [globalState, functionState, lockWeak](LLVMBuilderRef elseBuilder) {
    //           auto noneConstructor = lockWeak->noneConstructor;
    //           // If we get here, object is dead, return a None.
    //           auto noneRef = buildCallV(globalState, functionState, elseBuilder, noneConstructor, {});
    //           globalState->getRegion(lockWeak->noneType)
    //               ->checkValidReference(
    //                   FL(), functionState, elseBuilder, true, lockWeak->noneType, noneRef);
    //           return globalState->getRegion(lockWeak->noneType)
    //               ->upcast(
    //                   functionState,
    //                   elseBuilder,
    //                   lockWeak->noneType,
    //                   lockWeak->noneKind,
    //                   noneRef,
    //                   lockWeak->resultOptType,
    //                   lockWeak->resultOptKind);
    //         });
    //
    // globalState->getRegion(sourceType)->dealias(
    //     AFL("LockWeak drop weak ref"),
    //     functionState, builder, sourceType, sourceLE);
    //
    // return resultOptLE;
  } else if (auto asSubtype = dynamic_cast<AsSubtype*>(expr)) {
      std::cout << "AsSubtype unimplemented" << std::endl;
      { assert(false); throw 1337; }
//     buildFlare(FL(), globalState, functionState, builder, typeid(*expr).name());
//
//     auto sourceType = asSubtype->sourceType;
//     auto sourceLE =
//         translateExpression(
//             globalState, functionState, blockState, builder, asSubtype->sourceExpr);
//     globalState->getRegion(sourceType)
//         ->checkValidReference(FL(), functionState, builder, false, sourceType, sourceLE);
//
// //    auto sourceTypeAsConstraintRefM =
// //        globalState->metalCache->getReference(
// //            Ownership::BORROW,
// //            sourceType->location,
// //            sourceType->kind);
//
//     auto resultResultTypeLE =
//         globalState->getRegion(asSubtype->resultResultType)
//             ->translateType(asSubtype->resultResultType);
//
//     auto resultOptLE =
//         globalState->getRegion(sourceType)->asSubtype(
//             functionState, builder,
//             asSubtype->resultResultType,
//             sourceType,
//             sourceLE,
//             asSubtype->targetKind,
//             [globalState, functionState, asSubtype](LLVMBuilderRef thenBuilder, Ref refAsSubtype) -> Ref {
//               globalState->getRegion(asSubtype->okConstructor->params[0])
//                   ->checkValidReference(
//                       FL(), functionState, thenBuilder, false,
//                       asSubtype->okConstructor->params[0],
//                       refAsSubtype);
//
//               // If we get here, object is of the desired targetType, return a Ok containing it.
//               auto okRef = buildCallV(globalState, functionState, thenBuilder, asSubtype->okConstructor, {refAsSubtype});
//               globalState->getRegion(asSubtype->okType)
//                   ->checkValidReference(
//                       FL(), functionState, thenBuilder, true, asSubtype->okType, okRef);
//               return globalState->getRegion(asSubtype->okType)
//                   ->upcast(
//                       functionState,
//                       thenBuilder,
//                       asSubtype->okType,
//                       asSubtype->okKind,
//                       okRef,
//                       asSubtype->resultResultType,
//                       asSubtype->resultResultKind);
//             },
//             [globalState, functionState, asSubtype, sourceLE](LLVMBuilderRef thenBuilder) -> Ref {
//               // If we get here, object is not of the desired targetType, return a Err containing the original ref.
//               auto errRef = buildCallV(globalState, functionState, thenBuilder, asSubtype->errConstructor, {sourceLE});
//               globalState->getRegion(asSubtype->errType)
//                   ->checkValidReference(
//                       FL(), functionState, thenBuilder, true, asSubtype->errType, errRef);
//               return globalState->getRegion(asSubtype->errType)
//                   ->upcast(
//                       functionState,
//                       thenBuilder,
//                       asSubtype->errType,
//                       asSubtype->errKind,
//                       errRef,
//                       asSubtype->resultResultType,
//                       asSubtype->resultResultKind);
//             });
//
//     return resultOptLE;
  } else {
    std::string name = typeid(*expr).name();
    std::cout << name << std::endl;
    { assert(false); throw 1337; }
  }
  { assert(false); throw 1337; }
}
