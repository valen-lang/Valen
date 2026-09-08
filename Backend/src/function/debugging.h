#ifndef FUNCTION_DEBUGGING_H
#define FUNCTION_DEBUGGING_H

#include <string>
#include <llvm-c/Core.h>
#include <llvm-c/DebugInfo.h>

class GlobalState;
class Kind;
class Function;
class Local;
class FunctionState;

// All the backend's DWARF debug-info emission lives in debugging.cpp. Everything
// here is a no-op / opaque fallback unless the module was compiled with --debug.

// Set up / tear down the module's DIBuilder. init installs the builder and the
// Dwarf-Version / Debug-Info-Version module flags (without which LLVM emits no
// DWARF even with full DI metadata); finalize resolves temporary DI nodes before
// the module is verified or handed to codegen. Both no-op when debug is off.
void initDebugInfo(GlobalState* globalState);
void finalizeDebugInfo(GlobalState* globalState);

// Attach a DISubprogram to a freshly-declared Vale function so lldb can resolve
// frames to (file, line). No-op without a real source location.
void attachDISubprogram(
    GlobalState* globalState,
    LLVMValueRef functionLF,
    const std::string& linkageName,
    Function* functionM);

// Get-or-create a DIType for a Vale Kind (cached by Kind*). Primitives
// (Int/Bool/Float) map to precise DIBasicTypes; a struct to a flattened
// DICompositeType; everything else to a pointer-sized opaque "ref".
LLVMMetadataRef getOrCreateDIType(GlobalState* globalState, Kind* kind);

// A pointer-sized opaque "ref" DIType — for a pointer whose pointee we don't
// describe as a value.
LLVMMetadataRef makeOpaqueRefDIType(GlobalState* globalState);

// A DIType for a pointer-typed local. A borrow ref to a struct becomes a
// DW_TAG_pointer_type to that struct's composite (so `frame variable -P 1 x`
// walks the fields); anything else falls back to the opaque "ref".
LLVMMetadataRef getOrCreateDIPointerType(GlobalState* globalState, Kind* localType);

// Emit a DILocalVariable + llvm.dbg.declare against a local's alloca so
// `frame variable <name>` shows it in lldb. No-op for temporaries (no name),
// no source location, or when no subprogram is attached.
void emitLocalVariableDebugInfo(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Local* local,
    LLVMValueRef localAddr);

// Save/restore guard around the LLVM builder's "current debug location". Each
// translateExpression may set the builder's loc from its own source location,
// and the LAST set wins for every subsequent LLVMBuildXxx; this RAII restores
// the enclosing expression's loc on scope exit so a child's loc doesn't leak to
// its siblings.
class ScopedDebugLoc {
  LLVMBuilderRef builder;
  LLVMMetadataRef saved;
 public:
  explicit ScopedDebugLoc(LLVMBuilderRef b)
      : builder(b), saved(LLVMGetCurrentDebugLocation2(b)) {}
  ~ScopedDebugLoc() {
    if (saved) {
      LLVMSetCurrentDebugLocation2(builder, saved);
    }
  }
  ScopedDebugLoc(const ScopedDebugLoc&) = delete;
  ScopedDebugLoc& operator=(const ScopedDebugLoc&) = delete;
};

#endif
