#include <globalstate.h>
#include <function/function.h>
#include <function/expressions/expressions.h>
#include <region/common/common.h>
#include "urefstructlt.h"

UniversalRefStructLT::UniversalRefStructLT(LLVMContextRef context, LLVMTargetDataRef dataLayout) {
  structLT =
      std::make_unique<StructLT<UniversalRefStructNumMembers, UniversalRefStructMember>>(
          context,
          "__UniversalRef",
          // See URSL for why things are laid out like this.
          std::array<LLVMTypeRef, UniversalRefStructNumMembers>{
              LLVMInt32TypeInContext(context), // object generation
              LLVMInt32TypeInContext(context), // region generation
              LLVMIntTypeInContext(context, 56), // object pointer
              LLVMIntTypeInContext(context, 52), // type info pointer
              LLVMIntTypeInContext(context, 52), // region pointer
              LLVMInt16TypeInContext(context), // offset to generation
              LLVMInt16TypeInContext(context), // scope tether bits mask
          });
  // The size of the above struct isn't necessarily 32 bytes, it's not compressed by LLVM.
  // Later, we use buildCompressStruct for that.
}

UniversalRefStructExplodedMembersLT UniversalRefStructLT::explodeForRegularConcrete(GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, LLVMValueRef urefLE) {
  UniversalRefStructExplodedMembersLT result = explodeInner(globalState, functionState, builder, urefLE);
  result.typeInfoPtrI64LE = nullptr;
  return result;
}

UniversalRefStructExplodedMembersLT UniversalRefStructLT::explodeForRegularInterface(GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, LLVMValueRef urefLE) {
  return explodeInner(globalState, functionState, builder, urefLE);
}

UniversalRefStructExplodedMembersLT UniversalRefStructLT::explodeInner(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef urefStructLE) {
  assert(LLVMTypeOf(urefStructLE) == globalState->universalRefCompressedStructLT);
  auto urefI256LE = LLVMBuildExtractValue(builder, urefStructLE, 0, "");
  assert(LLVMTypeOf(urefI256LE) == LLVMIntTypeInContext(globalState->context, 256));
  auto urefLE = buildDecompressStruct(globalState, *structLT, builder, urefI256LE);
  auto typeInfoPtrI52LE = structLT->extractMember(builder, urefLE, UniversalRefStructMember::TYPE_INFO_PTR);
  auto typeInfoPtrI64LE = decompressI52PtrToI64(globalState, functionState, builder, typeInfoPtrI52LE);
  auto objectPtrI56LE = structLT->extractMember(builder, urefLE, UniversalRefStructMember::OBJECT_PTR);
  auto objectPtrI64LE = decompressI56PtrToI64(globalState, functionState, builder, objectPtrI56LE);
  return UniversalRefStructExplodedMembersLT{objectPtrI64LE, typeInfoPtrI64LE};
}

LLVMValueRef UniversalRefStructLT::implodeForRegularConcrete(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef objPtrI64LE) {
  auto int64LT = LLVMInt64TypeInContext(globalState->context);
  StructBuilderLT<UniversalRefStructNumMembers, UniversalRefStructMember> urefBuilder(structLT.get());
  auto objectGenLE = constI32LE(globalState, 0);
  urefBuilder.insertMember(builder, UniversalRefStructMember::OBJECT_GEN, objectGenLE);
  auto objPtrI56LE = compressI64PtrToI56(globalState, functionState, builder, objPtrI64LE);
  urefBuilder.insertMember(builder, UniversalRefStructMember::OBJECT_PTR, objPtrI56LE);
  auto typeInfoPtrI64LE = LLVMConstInt(int64LT, 0, false);
  auto typeInfoPtrI52LE = compressI64PtrToI52(globalState, functionState, builder, typeInfoPtrI64LE);
  urefBuilder.insertMember(builder, UniversalRefStructMember::TYPE_INFO_PTR, typeInfoPtrI52LE);
  fillUnusedFields(globalState, functionState, builder, &urefBuilder);
  auto structLE = urefBuilder.build();
  auto resultI256LE = buildCompressStruct(globalState, *structLT, builder, structLE);
  assert(LLVMSizeOfTypeInBits(globalState->dataLayout, LLVMTypeOf(resultI256LE)) == 256);
  auto resultStructLE =
      LLVMBuildInsertValue(builder, LLVMGetUndef(globalState->universalRefCompressedStructLT), resultI256LE, 0, "");
  return resultStructLE;
}

LLVMValueRef UniversalRefStructLT::implodeForRegularInterface(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef typeInfoPtrI64LE,
    LLVMValueRef objPtrI64LE) {
  StructBuilderLT<UniversalRefStructNumMembers, UniversalRefStructMember> urefBuilder(structLT.get());
  auto objectGenLE = constI32LE(globalState, 0);
  urefBuilder.insertMember(builder, UniversalRefStructMember::OBJECT_GEN, objectGenLE);
  auto objPtrI56LE = compressI64PtrToI56(globalState, functionState, builder, objPtrI64LE);
  urefBuilder.insertMember(builder, UniversalRefStructMember::OBJECT_PTR, objPtrI56LE);
  auto typeInfoPtrI52LE = compressI64PtrToI52(globalState, functionState, builder, typeInfoPtrI64LE);
  urefBuilder.insertMember(builder, UniversalRefStructMember::TYPE_INFO_PTR, typeInfoPtrI52LE);
  fillUnusedFields(globalState, functionState, builder, &urefBuilder);
  auto structLE = urefBuilder.build();
  auto resultI256LE = buildCompressStruct(globalState, *structLT, builder, structLE);
  assert(LLVMSizeOfTypeInBits(globalState->dataLayout, LLVMTypeOf(resultI256LE)) == 256);
  auto resultStructLE =
      LLVMBuildInsertValue(builder, LLVMGetUndef(globalState->universalRefCompressedStructLT), resultI256LE, 0, "");
  return resultStructLE;
}

void UniversalRefStructLT::fillUnusedFields(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    StructBuilderLT<UniversalRefStructNumMembers, UniversalRefStructMember>* urefBuilder) {
  auto int64LT = LLVMInt64TypeInContext(globalState->context);
  auto regionPtrI64LE = LLVMConstInt(int64LT, 0, false);
  auto regionPtrI52LE = compressI64PtrToI52(globalState, functionState, builder, regionPtrI64LE);
  urefBuilder->insertMember(builder, UniversalRefStructMember::REGION_PTR, regionPtrI52LE);
  urefBuilder->insertMember(builder, UniversalRefStructMember::REGION_GEN, constI32LE(globalState, 0));
  urefBuilder->insertMember(builder, UniversalRefStructMember::OBJECT_PTR_OFFSET_TO_GEN, constI16LE(globalState, 0));
  urefBuilder->insertMember(builder, UniversalRefStructMember::SCOPE_TETHER_BITS_MASK, constI16LE(globalState, 0));
}
