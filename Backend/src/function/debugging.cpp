#include "debugging.h"

#include <cstring>
#include <string>
#include <vector>

#include <utils/definefunction.h>
#include "expressions/shared/shared.h"
#include "../translatetype.h"
#include "function.h"
#include "expression.h"
#include "boundary.h"
#include <region/common/migration.h>
#include <utils/counters.h>
#include <llvm-c/DebugInfo.h>
#include <llvm-c/Target.h>
#include "metal/instructions.h"
#include "metal/types.h"
#include "metal/ast.h"

// Get-or-create a DIFile for a source path, caching on GlobalState so every
// function from the same file shares one DIFile. Splits into (dir, basename)
// at the last '/'; a path with no slash gets "." as its directory.
static LLVMMetadataRef getOrCreateDIFile(GlobalState* globalState, const std::string& path) {
  auto it = globalState->diFileCache.find(path);
  if (it != globalState->diFileCache.end()) {
    return it->second;
  }
  // `path` is the basename on the SourceLocation. Resolve it to its absolute on-disk path
  // (conveyed from the frontend) so the DIFile's directory is real and DW_AT_comp_dir resolves;
  // fall back to the basename (dir ".") when the frontend supplied no path for it.
  auto sp = globalState->program->sourcePaths.find(path);
  const std::string& resolved = (sp != globalState->program->sourcePaths.end()) ? sp->second : path;
  auto slash = resolved.find_last_of('/');
  std::string dir = (slash == std::string::npos) ? "." : resolved.substr(0, slash);
  std::string base = (slash == std::string::npos) ? resolved : resolved.substr(slash + 1);
  auto file = LLVMDIBuilderCreateFile(
      globalState->dibuilder, base.c_str(), base.size(), dir.c_str(), dir.size());
  globalState->diFileCache.emplace(path, file);
  return file;
}

// Get-or-create the module's single DICompileUnit (one CU per module is an LLVM-DI constraint).
// The CU's file supplies DW_AT_comp_dir, so anchor it to a *user* source file — the lexically-first
// basename in the conveyed source map — not whatever function compiled first, which could be a
// builtin (e.g. arith.vale) that isn't in the map and would leave comp_dir "." (breaking lldb's
// source resolution, and varying with compile order). Falls back to the caller's file only when no
// source paths were conveyed (interop, or inputs with no on-disk file). Later functions in other files still
// get their own DIFile but share this CU.
static LLVMMetadataRef getOrCreateCompileUnit(
    GlobalState* globalState, LLVMMetadataRef anchorFile) {
  if (globalState->compileUnit) {
    return globalState->compileUnit;
  }
  LLVMMetadataRef cuFile = anchorFile;
  // This is suspicious, source paths shouldnt be empty.
  if (!globalState->program->sourcePaths.empty()) {
    const std::string* chosen = nullptr;
    for (auto& entry : globalState->program->sourcePaths) {
      if (chosen == nullptr || entry.first < *chosen) {
        chosen = &entry.first;
      }
    }
    cuFile = getOrCreateDIFile(globalState, *chosen);
  }
  globalState->compileUnit = LLVMDIBuilderCreateCompileUnit(
      globalState->dibuilder, LLVMDWARFSourceLanguageC, cuFile, "Vale compiler",
      13, 0, "", 0, 0, "", 0, LLVMDWARFEmissionFull, 0, 0, 0, "", 0, "", 0);
  return globalState->compileUnit;
}

// Attach a DISubprogram to a freshly-declared Vale function so LLDB can resolve
// frames to (file, line). Skipped when the frontend supplied no source location
// (synthetic / extern / non-debug build). Milestone 1: minimal subroutine type
// with no parameter/return DI — sufficient for function-level breakpoints and
// backtraces; locals come with a later milestone.
void attachDISubprogram(
    GlobalState* globalState,
    LLVMValueRef functionLF,
    const std::string& linkageName,
    Function* functionM) {
  if (!globalState->opt->debug) {
    return;
  }
  if (functionM->sourceLocation == nullptr ||
      functionM->sourceLocation->filePath.empty()) {
    return;
  }

  auto file = getOrCreateDIFile(globalState, functionM->sourceLocation->filePath);
  getOrCreateCompileUnit(globalState, file);
  int32_t line = functionM->sourceLocation->line;
  auto subroutineType = LLVMDIBuilderCreateSubroutineType(
      globalState->dibuilder, file, nullptr, 0, LLVMDIFlagZero);
  auto subprogram = LLVMDIBuilderCreateFunction(
      globalState->dibuilder,
      /*scope*/ file,
      linkageName.c_str(), linkageName.size(),
      linkageName.c_str(), linkageName.size(),
      file, line, subroutineType,
      /*is_local_to_unit*/ true,
      /*is_definition*/ true,
      /*scope_line*/ line,
      LLVMDIFlagZero,
      /*is_optimized*/ false);
  LLVMSetSubprogram(functionLF, subprogram);
}

// DWARF type-encoding / tag constants; the LLVM C API takes them as bare
// unsigneds. Only the few we model.
static constexpr unsigned DW_ATE_ADDRESS = 0x01;
static constexpr unsigned DW_ATE_BOOLEAN = 0x02;
static constexpr unsigned DW_ATE_FLOAT = 0x04;
static constexpr unsigned DW_ATE_SIGNED = 0x05;
static constexpr unsigned DW_TAG_STRUCTURE_TYPE = 0x13;

// A pointer-sized opaque "ref" DIType, for kinds we don't model as a value yet
// (heap refs, arrays, strings, interfaces, void, never). ptrSize is in bits.
LLVMMetadataRef makeOpaqueRefDIType(GlobalState* globalState) {
  return LLVMDIBuilderCreateBasicType(
      globalState->dibuilder, "ref", 3, globalState->ptrSize, DW_ATE_ADDRESS, LLVMDIFlagZero);
}

// Layer-1: a flattened DICompositeType for a Vale struct's INNER LLVM struct
// (`%<name>`, no control block — that's what an inline struct local's alloca
// holds). User members at their offsets. A forward decl goes in the cache before
// recursing into member DITypes so a self-referential struct terminates via RAUW.
static LLVMMetadataRef getOrCreateDIStructType(GlobalState* globalState, StructKind* kind) {
  auto structM = globalState->lookupStruct(kind);
  const std::string& fullName = kind->fullName->name;
  auto innerLT = LLVMGetTypeByName2(globalState->context, fullName.c_str());
  // No inner struct only for synthetic / never-defined kinds; fall back to opaque.
  if (innerLT == nullptr) {
    auto opaque = makeOpaqueRefDIType(globalState);
    globalState->diTypeCache.emplace(kind, opaque);
    return opaque;
  }

  uint64_t sizeBits = LLVMSizeOfTypeInBits(globalState->dataLayout, innerLT);
  auto fwd = LLVMDIBuilderCreateReplaceableCompositeType(
      globalState->dibuilder, DW_TAG_STRUCTURE_TYPE,
      fullName.c_str(), fullName.size(),
      /*Scope*/ globalState->compileUnit ? globalState->compileUnit : nullptr,
      /*File*/ nullptr, /*Line*/ 0, /*RuntimeLang*/ 0,
      sizeBits, /*AlignInBits*/ 0, LLVMDIFlagZero,
      fullName.c_str(), fullName.size());
  globalState->diTypeCache[kind] = fwd;

  std::vector<LLVMMetadataRef> diMembers;
  for (size_t i = 0; i < structM->members.size(); i++) {
    auto sm = structM->members[i];
    auto memberDIType = getOrCreateDIType(globalState, sm->type);
    auto memberLT = LLVMStructGetTypeAtIndex(innerLT, i);
    // ABI storage size, not abstract type size (i1 abstractly is 1 bit; lldb
    // wants the slot's byte size or it silently drops the sub-byte field).
    uint64_t memberSizeBits = 8 * LLVMABISizeOfType(globalState->dataLayout, memberLT);
    uint64_t memberOffsetBits = 8 * LLVMOffsetOfElement(globalState->dataLayout, innerLT, i);
    diMembers.push_back(LLVMDIBuilderCreateMemberType(
        globalState->dibuilder, /*Scope*/ fwd, sm->name.c_str(), sm->name.size(),
        /*File*/ nullptr, /*LineNo*/ 0,
        memberSizeBits, /*AlignInBits*/ 0, memberOffsetBits,
        LLVMDIFlagZero, memberDIType));
  }

  auto real = LLVMDIBuilderCreateStructType(
      globalState->dibuilder,
      /*Scope*/ globalState->compileUnit ? globalState->compileUnit : nullptr,
      fullName.c_str(), fullName.size(),
      /*File*/ nullptr, /*Line*/ 0,
      sizeBits, /*AlignInBits*/ 0, LLVMDIFlagZero,
      /*DerivedFrom*/ nullptr,
      diMembers.data(), diMembers.size(),
      /*RuntimeLang*/ 0, /*VTableHolder*/ nullptr,
      fullName.c_str(), fullName.size());
  LLVMMetadataReplaceAllUsesWith(fwd, real);
  globalState->diTypeCache[kind] = real;
  return real;
}

// Layer 1.5: a DW_TAG_array_type for a static-sized array's INNER LLVM type
// (`[N x elem]`, no control block — what an SSA local's alloca holds). The
// element DIType recurses through getOrCreateDIType and the count N rides a
// subrange, so `frame variable -P 1 a` walks the elements. (Runtime-sized arrays
// are deferred; they'd be a pointer-to-wrapper with a runtime count.)
static LLVMMetadataRef getOrCreateDIArrayType(GlobalState* globalState, StaticSizedArrayT* kind) {
  auto def = globalState->program->getStaticSizedArray(kind);
  auto innerLT = globalState->getRegion(kind)->translateType(kind);
  uint64_t sizeBits = LLVMSizeOfTypeInBits(globalState->dataLayout, innerLT);
  auto elemDI = getOrCreateDIType(globalState, def->elementType);
  LLVMMetadataRef subrange =
      LLVMDIBuilderGetOrCreateSubrange(globalState->dibuilder, /*LowerBound*/ 0, def->size);
  auto arrayDI = LLVMDIBuilderCreateArrayType(
      globalState->dibuilder, sizeBits, /*AlignInBits*/ 0, elemDI, &subrange, 1);
  globalState->diTypeCache.emplace(kind, arrayDI);
  return arrayDI;
}

LLVMMetadataRef getOrCreateDIType(GlobalState* globalState, Kind* kind) {
  auto it = globalState->diTypeCache.find(kind);
  if (it != globalState->diTypeCache.end()) {
    return it->second;
  }
  if (auto structKind = dynamic_cast<StructKind*>(kind)) {
    return getOrCreateDIStructType(globalState, structKind);
  }
  if (auto ssaMT = dynamic_cast<StaticSizedArrayT*>(kind)) {
    return getOrCreateDIArrayType(globalState, ssaMT);
  }
  LLVMMetadataRef di = nullptr;
  if (auto intK = dynamic_cast<Int*>(kind)) {
    auto name = std::string("i") + std::to_string(intK->bits);
    di = LLVMDIBuilderCreateBasicType(
        globalState->dibuilder, name.c_str(), name.size(),
        intK->bits, DW_ATE_SIGNED, LLVMDIFlagZero);
  } else if (dynamic_cast<Bool*>(kind)) {
    // Bool's LLVM storage is i1 but it occupies a 1-byte ABI slot; report 8 bits
    // so lldb doesn't drop it as a sub-byte member. Matches Rust/Swift bool DI.
    di = LLVMDIBuilderCreateBasicType(
        globalState->dibuilder, "bool", 4, 8, DW_ATE_BOOLEAN, LLVMDIFlagZero);
  } else if (dynamic_cast<Float*>(kind)) {
    di = LLVMDIBuilderCreateBasicType(
        globalState->dibuilder, "f64", 3, 64, DW_ATE_FLOAT, LLVMDIFlagZero);
  } else {
    // Arrays, strings, interfaces, void, never: a pointer-sized opaque "ref" so
    // the local still lists in lldb. Walking their fields is the deferred Layers.
    di = makeOpaqueRefDIType(globalState);
  }
  globalState->diTypeCache.emplace(kind, di);
  return di;
}

// A DIType for a pointer-typed local. If it points at a struct (a borrow ref to
// a StructKind), returns a DW_TAG_pointer_type to that struct's inner composite,
// so `frame variable -P 1 x` walks the fields. Otherwise the opaque "ref". The
// debugger models the user-visible value and ignores control blocks: in this
// region a borrow lowers to a pointer straight to the inner struct, no wrapper.
LLVMMetadataRef getOrCreateDIPointerType(GlobalState* globalState, Kind* localType) {
  if (auto borrowRef = dynamic_cast<BorrowRef*>(localType)) {
    if (auto structKind = dynamic_cast<StructKind*>(borrowRef->inner)) {
      auto pointee = getOrCreateDIStructType(globalState, structKind);
      return LLVMDIBuilderCreatePointerType(
          globalState->dibuilder, pointee, globalState->ptrSize,
          /*AlignInBits*/ 0, /*AddressSpace*/ 0, "", 0);
    }
  }
  return makeOpaqueRefDIType(globalState);
}

// Emit a DILocalVariable + llvm.dbg.declare against a local's alloca so `frame
// variable <name>` shows it in lldb. Skipped without debug info, a declaration
// location, an attached subprogram, or a user-facing name (compiler-generated
// temporaries have none). Called from makeHammerLocal — the single point where
// every local's alloca is created — so no local can slip through undeclared (a
// per-call-site emission would, and did, miss the destructure paths).
void emitLocalVariableDebugInfo(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Local* local,
    LLVMValueRef localAddr) {
  if (!globalState->opt->debug || local->sourceLocation == nullptr || local->name.empty()) {
    return;
  }
  auto subprogram = LLVMGetSubprogram(functionState->containingFuncL);
  if (!subprogram) {
    return;
  }
  int32_t line = local->sourceLocation->line;
  auto scopeFile = LLVMDIScopeGetFile(subprogram);
  // The DIType must match what the alloca holds. An inline value (primitive or
  // inline struct) gets its value-level DIType; a pointer local (a borrow ref)
  // gets a pointer DIType — to the pointee's composite when it's a struct (so
  // `frame variable -P 1` walks its fields), else an opaque address.
  auto storageLT = globalState->getRegion(local->type)->translateType(local->type);
  LLVMMetadataRef diType =
      (LLVMGetTypeKind(storageLT) == LLVMPointerTypeKind)
          ? getOrCreateDIPointerType(globalState, local->type)
          : getOrCreateDIType(globalState, local->type);
  const std::string& name = local->name;
  auto diVar = LLVMDIBuilderCreateAutoVariable(
      globalState->dibuilder, subprogram, name.c_str(), name.size(),
      scopeFile, line, diType,
      /*AlwaysPreserve*/ true, LLVMDIFlagZero, /*AlignInBits*/ 0);
  auto diExpr = LLVMDIBuilderCreateExpression(globalState->dibuilder, nullptr, 0);
  auto diLoc = LLVMDIBuilderCreateDebugLocation(
      globalState->context, line, 1, subprogram, /*inlinedAt*/ nullptr);
  LLVMDIBuilderInsertDeclareRecordAtEnd(
      globalState->dibuilder, localAddr, diVar, diExpr, diLoc,
      LLVMGetInsertBlock(builder));
}

// Create the module's DIBuilder and install the module flags that make the LLVM
// DwarfDebug pass actually emit DWARF. No-op unless --debug.
void initDebugInfo(GlobalState* globalState) {
  if (!globalState->opt->debug) {
    return;
  }
  globalState->dibuilder = LLVMCreateDIBuilder(globalState->mod);
  // The DICompileUnit is created lazily on the first user function with a source
  // path (see attachDISubprogram), so the CU anchors to a real Vale source file
  // — that's what makes lldb's `b set -f <file> -l <line>` resolve. Without these
  // two module-flag entries the LLVM DwarfDebug pass emits no DWARF even with
  // fully-populated DI metadata. Dwarf 4 is what clang ships by default on this
  // toolchain; DI metadata version 3 has been stable for years.
  auto i32Ty = LLVMInt32TypeInContext(globalState->context);
  LLVMAddModuleFlag(globalState->mod, LLVMModuleFlagBehaviorWarning,
      "Dwarf Version", strlen("Dwarf Version"),
      LLVMValueAsMetadata(LLVMConstInt(i32Ty, 4, 0)));
  LLVMAddModuleFlag(globalState->mod, LLVMModuleFlagBehaviorWarning,
      "Debug Info Version", strlen("Debug Info Version"),
      LLVMValueAsMetadata(LLVMConstInt(i32Ty, 3 /* LLVM DEBUG_METADATA_VERSION */, 0)));
}

// Resolve temporary DI nodes. Must run after all DISubprograms/DILocations are
// emitted and before the module is verified or handed to codegen — otherwise
// LLVM sees unresolved debug metadata. No-op when debug info was never enabled.
void finalizeDebugInfo(GlobalState* globalState) {
  if (globalState->dibuilder) {
    LLVMDIBuilderFinalize(globalState->dibuilder);
  }
}
