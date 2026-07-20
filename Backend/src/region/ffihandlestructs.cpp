#include <array>
#include <globalstate.h>
#include <function/function.h>
#include <function/expressions/expressions.h>
#include <region/common/common.h>
#include "ffihandlestructs.h"

FfiHandleStructs::FfiHandleStructs(LLVMContextRef context) {
  // Per @HTSLVBDTCZ, these two named structs are the only handle types in the
  // whole program: every concrete kind shares __ConcreteHandle and every
  // interface shares __InterfaceHandle. Per-class distinctness is added later,
  // in the C typedef emitters.
  auto int64LT = LLVMInt64TypeInContext(context);
  // Concrete handle: { i64 obj } — 8 bytes.
  concreteHandleStructLT = LLVMStructCreateNamed(context, "__ConcreteHandle");
  std::array<LLVMTypeRef, 1> concreteMembersLT{int64LT};
  LLVMStructSetBody(concreteHandleStructLT, concreteMembersLT.data(), concreteMembersLT.size(), false);
  // Interface handle: { i64 obj, i64 typeinfo } — 16 bytes. Field order is
  // obj=0, typeinfo=1.
  interfaceHandleStructLT = LLVMStructCreateNamed(context, "__InterfaceHandle");
  std::array<LLVMTypeRef, 2> interfaceMembersLT{int64LT, int64LT};
  LLVMStructSetBody(interfaceHandleStructLT, interfaceMembersLT.data(), interfaceMembersLT.size(), false);
}

FfiHandleExplodedMembers FfiHandleStructs::explodeForRegularConcrete(
    GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, LLVMValueRef handleLE) {
  assert(LLVMTypeOf(handleLE) == concreteHandleStructLT);
  auto objPtrI64LE = LLVMBuildExtractValue(builder, handleLE, 0, "objPtrI64");
  // Concretes carry no type info; the pointer alone identifies the object.
  return FfiHandleExplodedMembers{objPtrI64LE, nullptr};
}

FfiHandleExplodedMembers FfiHandleStructs::explodeForRegularInterface(
    GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, LLVMValueRef handleLE) {
  assert(LLVMTypeOf(handleLE) == interfaceHandleStructLT);
  auto objPtrI64LE = LLVMBuildExtractValue(builder, handleLE, 0, "objPtrI64");
  auto typeInfoPtrI64LE = LLVMBuildExtractValue(builder, handleLE, 1, "typeInfoPtrI64");
  return FfiHandleExplodedMembers{objPtrI64LE, typeInfoPtrI64LE};
}

LLVMValueRef FfiHandleStructs::implodeForRegularConcrete(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef objPtrI64LE) {
  auto int64LT = LLVMInt64TypeInContext(globalState->context);
  assert(LLVMTypeOf(objPtrI64LE) == int64LT);
  auto handleLE = LLVMGetUndef(concreteHandleStructLT);
  handleLE = LLVMBuildInsertValue(builder, handleLE, objPtrI64LE, 0, "handle");
  return handleLE;
}

LLVMValueRef FfiHandleStructs::implodeForRegularInterface(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    LLVMValueRef typeInfoPtrI64LE,
    LLVMValueRef objPtrI64LE) {
  auto int64LT = LLVMInt64TypeInContext(globalState->context);
  assert(LLVMTypeOf(objPtrI64LE) == int64LT);
  assert(LLVMTypeOf(typeInfoPtrI64LE) == int64LT);
  auto handleLE = LLVMGetUndef(interfaceHandleStructLT);
  handleLE = LLVMBuildInsertValue(builder, handleLE, objPtrI64LE, 0, "handle"); // field 0 = obj
  handleLE = LLVMBuildInsertValue(builder, handleLE, typeInfoPtrI64LE, 1, "handle"); // field 1 = typeinfo
  return handleLE;
}
