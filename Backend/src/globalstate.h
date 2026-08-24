#ifndef GLOBALSTATE_H_
#define GLOBALSTATE_H_

#include <llvm-c/Core.h>

#include <unordered_map>
#include "metal/metalcache.h"
#include "region/common/defaultlayout/structs.h"
#include "region/ffihandlestructs.h"

#include "metal/ast.h"
#include "metal/instructions.h"
#include "valeopts.h"
#include "addresshasher.h"
#include "externs.h"
#include "utils/structlt.h"

class IRegion;
class KindStructs;
class KindStructs;
class ControlBlock;
class RCImm;

constexpr int LGT_ENTRY_MEMBER_INDEX_FOR_GEN = 0;
constexpr int LGT_ENTRY_MEMBER_INDEX_FOR_NEXT_FREE = 1;

class GlobalState {
public:
  GlobalState(AddressNumberer* addressNumberer);

  AddressNumberer* addressNumberer;

  LLVMTargetMachineRef machine = nullptr;
  LLVMContextRef context = nullptr;
  LLVMDIBuilderRef dibuilder = nullptr;
  LLVMMetadataRef compileUnit = nullptr;
  LLVMMetadataRef difile = nullptr;

  ValeOptions *opt = nullptr;

  Externs* externs = nullptr;

  LLVMTargetDataRef dataLayout = nullptr;
  LLVMModuleRef mod = nullptr;
  int ptrSize = 0;

  MetalCache* metalCache = nullptr;

  Program* program = nullptr;

  LLVMValueRef numMainArgsLE = nullptr;
  LLVMValueRef mainArgsLE = nullptr;

  LLVMValueRef objIdCounterLE = nullptr;
  LLVMValueRef liveHeapObjCounterLE = nullptr;
  LLVMValueRef derefCounterLE = nullptr;
  LLVMValueRef mutRcAdjustCounterLE = nullptr;
  LLVMValueRef writeOnlyGlobalLE = nullptr;
  LLVMValueRef crashGlobalLE = nullptr;
//  LLVMValueRef nullLE = nullptr;

  LLVMTypeRef wrcTableStructLT = nullptr;
  RawFuncPtrLE expandWrcTable, checkWrci, getNumWrcs;

  LLVMTypeRef lgtTableStructLT = nullptr, lgtEntryStructLT = nullptr; // contains generation and next free
  RawFuncPtrLE expandLgt, checkLgti, getNumLiveLgtEntries;

//  LLVMValueRef genMalloc = nullptr, genFree = nullptr;

  // The FFI handle types the outside world uses to refer to our objects, each
  // sized to exactly what its ref layer needs. See ffihandlestructs.h.
  // Per @HTSLVBDTCZ, all concrete kinds share one LLVM handle type and all
  // interfaces share one; per-class distinctness lives only in the C typedefs.
  std::unique_ptr<FfiHandleStructs> ffiHandleStructs;

  // This is a global, we can return this when we want to return never. It should never actually be
  // used as an input to any expression in any function though.
  LLVMValueRef neverLE = nullptr;

//  LLVMValueRef coroutineEntryFunc = nullptr;

  LLVMBuilderRef stringConstantBuilder = nullptr;
  std::unordered_map<std::string, LLVMValueRef> stringConstants;

  std::unordered_map<Edge*, LLVMValueRef, AddressHasher<Edge*>> interfaceTablePtrs;

  std::unordered_map<std::string, ValeFuncPtrLE> functions;
  std::unordered_map<std::string, RawFuncPtrLE> externFunctions;

  // This is temporary, Valestrom should soon embed mutability and region into the kind for us
  // so we won't have to do this.
  std::unordered_map<Kind*, RegionId*, AddressHasher<Kind*>> regionIdByKind;

  // These contain the extra methods that Backend adds to particular interfaces.
  std::unordered_map<Prototype*, ValeFuncPtrLE, AddressHasher<Prototype*>> extraFunctions;

  // Backend-owned registry of the auto-generated FFI accessor exports
  // (alias/dealias/ref_eq/getters/upcast/asSubstruct/typeTag/_new/_len/_at/str
  // primitives), grouped per package. Mirrors Package::exportNameToFunction but
  // lives here so codegen doesn't mutate the input AST. The emitters read this
  // alongside each package's frontend-provided exports.
  std::unordered_map<
      PackageCoordinate*,
      std::unordered_map<std::string, Prototype*>,
      AddressHasher<PackageCoordinate*>> autoExportsByPackage;
  std::unordered_map<
      InterfaceKind*,
      std::vector<InterfaceMethod*>,
          AddressHasher<InterfaceKind*>> interfaceExtraMethods;

  using OverridesBySubstructMap =
      std::unordered_map<
          StructKind*,
          std::vector<std::pair<InterfaceMethod*, Prototype*>>,
          AddressHasher<StructKind*>>;
  using OverridesBySubstructByInterfaceMap =
      std::unordered_map<
          InterfaceKind*,
          OverridesBySubstructMap,
          AddressHasher<InterfaceKind*>>;
  OverridesBySubstructByInterfaceMap overridesBySubstructByInterface;
  // This keeps us from adding more edges or interfaces after we've already started compiling them.
  bool interfacesOpen = true;

  FfiHandleStructs* getFfiHandleStructs() { return ffiHandleStructs.get(); }

  void addInterfaceExtraMethod(InterfaceKind* interfaceKind, InterfaceMethod* method) {
    assert(interfacesOpen);
    interfaceExtraMethods[interfaceKind].push_back(method);
  }
  void addEdgeExtraMethod(InterfaceKind* interfaceMT, StructKind* structMT, InterfaceMethod* interfaceMethod, Prototype* function) {
    auto interfaceExtraMethodsI = interfaceExtraMethods.find(interfaceMT);
    assert(interfaceExtraMethodsI != interfaceExtraMethods.end());

    assert(interfacesOpen);
    auto iter = overridesBySubstructByInterface.find(interfaceMT);
    if (iter == overridesBySubstructByInterface.end()) {
      iter =
          overridesBySubstructByInterface.emplace(
              interfaceMT,
              OverridesBySubstructMap{0, addressNumberer->makeHasher<StructKind*>()})
              .first;
    }
    int index = iter->second[structMT].size();

    assert(interfaceExtraMethodsI->second[index] == interfaceMethod);
    iter->second[structMT].push_back(std::make_pair(interfaceMethod, function));
  }

  StructDefinition* lookupStruct(StructKind* structMT) {
//    auto structI = extraStructs.find(name);
//    if (structI != extraStructs.end()) {
//      return structI->second;
//    }
    return program->getStruct(structMT);
  }

  InterfaceDefinition* lookupInterface(InterfaceKind* interfaceMT) {
//    auto interfaceI = extraInterfaces.find(name);
//    if (interfaceI != extraInterfaces.end()) {
//      return interfaceI->second;
//    }
    return program->getInterface(interfaceMT);
  }

  ValeFuncPtrLE lookupFunction(Prototype* prototype) {
    auto iter = extraFunctions.find(prototype);
    if (iter != extraFunctions.end()) {
      return iter->second;
    }
    auto funcIter = functions.find(prototype->name->name);
    assert(funcIter != functions.end());
    return funcIter->second;
  }

  int getInterfaceMethodIndex(InterfaceKind* interfaceKindM, Prototype* prototype) {
    int numMetalMethods = 0;
    if (auto maybeInterfaceDefM = program->getMaybeInterface(interfaceKindM)) {
      auto interfaceDefM = *maybeInterfaceDefM;
      for (int i = 0; i < interfaceDefM->methods.size(); i++) {
        if (interfaceDefM->methods[i]->prototype == prototype) {
          return i;
        }
      }
      numMetalMethods = interfaceDefM->methods.size();
    }
    auto iter = interfaceExtraMethods.find(interfaceKindM);
    assert(iter != interfaceExtraMethods.end());
    auto extraMethods = iter->second;
    for (int i = 0; i < extraMethods.size(); i++) {
      if (extraMethods[i]->prototype == prototype) {
        return numMetalMethods + i;
      }
    }
    std::cerr << "Couldn't find method " << prototype->name->name << " in interface " << interfaceKindM->fullName->name << std::endl;
    { assert(false); throw 1337; }
  }

  Weakability getKindWeakability(ValueKind* kind);

  Ref constI64(int64_t x);
  Ref constI32(int32_t x);
  Ref constI1(bool b);
  Ref buildAdd(FunctionState* functionState, LLVMBuilderRef builder, Ref a, Ref b);
  Ref buildMod(FunctionState* functionState, LLVMBuilderRef builder, Ref a, Ref b);
  Ref buildMultiply(FunctionState* functionState, LLVMBuilderRef builder, Ref a, Ref b);
  Ref buildDivide(FunctionState* functionState, LLVMBuilderRef builder, Ref a, Ref b);

  Name* getKindName(ValueKind* kind);

  template<typename T>
  AddressHasher<T> makeAddressHasher() {
    return addressNumberer->makeHasher<T>();
  }

  Name* freeName = nullptr;
  Name* freeThunkName = nullptr;
  Name* aliasName = nullptr;
  Name* dealiasName = nullptr;
  Name* refEqName = nullptr;
  Name* strLenName = nullptr;
  Name* strCharAtName = nullptr;
  Name* typeTagName = nullptr;
  Name* asSubstructName = nullptr;
  Name* upcastName = nullptr;
  Name* arrLenName = nullptr;
  Name* arrAtName = nullptr;
  Name* structNewName = nullptr;
  Name* ssaNewName = nullptr;

  RCImm* rcImm = nullptr;
  IRegion* mutRegion = nullptr;
  std::unordered_map<RegionId*, IRegion*, AddressHasher<RegionId*>> regions;


  std::vector<ValeFuncPtrLE> getEdgeFunctions(Edge* edge);

  // Note that this returns a vector of function types, not pointers.
  // The caller may have to make them pointers.
  std::vector<LLVMTypeRef> getInterfaceFunctionTypesNonPointer(InterfaceKind* kind);
  std::vector<LLVMTypeRef> getInterfaceFunctionPointerTypes(InterfaceKind* kind);


  IRegion* getRegion(Kind* kindM);
  IRegion* getRegion(RegionId* regionId);

  // Convenience wrapper around getRegion(refM)->translateType(refM); the
  // most common cross-region dispatch, spelled shorter.
  LLVMTypeRef translateType(Kind* refM);

  ValeFuncPtrLE getFunction(Prototype* proto);
  LLVMValueRef getInterfaceTablePtr(Edge* edge);
  LLVMValueRef getOrMakeStringConstant(const std::string& str);
};

#endif
