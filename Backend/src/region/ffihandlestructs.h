#ifndef REGION_FFIHANDLESTRUCTS_H_
#define REGION_FFIHANDLESTRUCTS_H_

#include <llvm-c/Core.h>

class GlobalState;
class FunctionState;

// The exploded components of an FFI handle. Concrete handles fill only
// objPtrI64LE (typeInfoPtrI64LE stays null); interface handles fill both.
struct FfiHandleExplodedMembers {
  LLVMValueRef objPtrI64LE;
  LLVMValueRef typeInfoPtrI64LE;

  FfiHandleExplodedMembers() = delete;

  FfiHandleExplodedMembers(
      LLVMValueRef objPtrI64LE_,
      LLVMValueRef typeInfoPtrI64LE_) :
      objPtrI64LE(objPtrI64LE_),
      typeInfoPtrI64LE(typeInfoPtrI64LE_) {}
};

// The FFI handle types that cross the C boundary, each sized to exactly what
// its ref layer needs. Per @HTSLVBDTCZ, all concretes share one of these types
// and all interfaces share the other; per-class distinctness lives only in the
// C typedefs.
//   - concrete (struct/str/RSA/SSA): { i64 obj }              — 8 bytes
//   - interface:                     { i64 obj, i64 typeinfo } — 16 bytes
// Fields are plain i64 pointer bits (PtrToInt/IntToPtr), no compression. They
// stay LLVM structs (not bare i64) so per-kind C typedefs keep type
// distinctness and the explicit-pointer-param ABI machinery is untouched.
struct FfiHandleStructs {
  explicit FfiHandleStructs(LLVMContextRef context);

  FfiHandleExplodedMembers explodeForRegularConcrete(
      GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, LLVMValueRef handleLE);

  FfiHandleExplodedMembers explodeForRegularInterface(
      GlobalState* globalState, FunctionState* functionState, LLVMBuilderRef builder, LLVMValueRef handleLE);

  LLVMValueRef implodeForRegularConcrete(
      GlobalState* globalState,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      LLVMValueRef objPtrI64LE);
  LLVMValueRef implodeForRegularInterface(
      GlobalState* globalState,
      FunctionState* functionState,
      LLVMBuilderRef builder,
      LLVMValueRef typeInfoPtrI64LE,
      LLVMValueRef objPtrI64LE);

  [[nodiscard]] LLVMTypeRef getConcreteHandleStructLT() const { return concreteHandleStructLT; }
  [[nodiscard]] LLVMTypeRef getInterfaceHandleStructLT() const { return interfaceHandleStructLT; }

private:
  LLVMTypeRef concreteHandleStructLT;
  LLVMTypeRef interfaceHandleStructLT;
};

#endif
