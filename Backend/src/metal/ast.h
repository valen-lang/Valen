#ifndef VALE_AST_H_
#define VALE_AST_H_

#include "name.h"
#include "types.h"
#include "../addresshasher.h"

#include <llvm-c/Core.h>
#include <llvm-c/DebugInfo.h>
#include <llvm-c/ExecutionEngine.h>
#include <llvm-c/Analysis.h>

#include <stdio.h>
#include <assert.h>
#include <cstdint>
#include <string>
#include <vector>
#include <algorithm>
#include <memory>
#include <unordered_map>
#include <iostream>
#include <optional>

using std::move;

extern const std::string BUILTIN_PROJECT_NAME;

// Defined elsewhere
class Block;
class Expression;

// Defined in this file
class Program;
class StructDefinition;
class StructMember;
class InterfaceDefinition;
class Edge;
class Function;
class Prototype;
class Name;

// An imported extern struct's layout (e.g. from rustc's tcx.layout_of), carried on the metal
// Package so Unsafe::defineStruct can size the opaque struct as [size/align x i{align*8}].
// General source-agnostic metadata; the interop provider fills it today.
struct OpaqueStructLayout {
  uint64_t sizeBytes;
  uint64_t alignBytes;
};
// How one argument or return value crosses an extern boundary. Source-agnostic: filled from
// rustc's FnAbi for interop, or a standalone C-ABI classifier later. buildCallOrSideCall marshals
// per this.
enum class CoercionKind {
  Ignore,     // not passed at all: a unit `()` return, like the drop shim's
  DirectInt,  // in a register as an integer of `directIntBits` bits: a small struct, e.g. Counter -> i32
  DirectPtr,  // as a pointer: a borrow (&self, &mut self) or a *mut T
  Indirect,   // through a hidden sret out-parameter: a large struct return, e.g. the 48-byte Domino
};
struct Coercion {
  CoercionKind kind;
  uint32_t directIntBits;  // width when kind == DirectInt; ignored otherwise
};
// One extern function's ABI: how its return and each argument cross.
struct ExternAbi {
  Coercion ret;
  std::vector<Coercion> args;
};

class Package {
public:
  PackageCoordinate* packageCoordinate;
  std::unordered_map<std::string, InterfaceDefinition*> interfaces;
  std::unordered_map<std::string, StructDefinition*> structs;
  std::unordered_map<std::string, StaticSizedArrayDefinitionT*> staticSizedArrays;
  std::unordered_map<std::string, RuntimeSizedArrayDefinitionT*> runtimeSizedArrays;
  std::unordered_map<std::string, Function*> functions;
//  std::unordered_map<Kind*, Prototype*, AddressHasher<Kind*>> immDestructorsByKind;

  // This only contains exports defined in this package. Though, the things they're exporting can
  // be defined somewhere else.
  std::unordered_map<std::string, Prototype*> exportNameToFunction;
  std::unordered_map<std::string, Kind*> exportNameToKind;
  std::unordered_map<std::string, Prototype*> externNameToFunction;
  std::unordered_map<std::string, Kind*> externNameToKind;
  // VCOORD: see if we can key not by name
  // Imported extern-struct layouts, keyed by the struct's fullName->name (like structs / externNameToKind).
  // General source-agnostic metadata, empty unless a producer (the interop provider) filled it;
  // Unsafe::defineStruct reads it to size an opaque struct instead of building from members.
  std::unordered_map<std::string, OpaqueStructLayout> structLayouts;
  // VCOORD: see if we can key not by name
  // Per-extern ABI descriptors, keyed by the extern symbol (like externNameToFunction). Empty for
  // descriptor-less C externs; buildCallOrSideCall reads it to coerce each crossing.
  std::unordered_map<std::string, ExternAbi> externAbis;
  // These are inverses of the above maps
  std::unordered_map<Prototype*, std::string, AddressHasher<Prototype*>> functionToExportName;
  std::unordered_map<Kind*, std::string, AddressHasher<Kind*>> kindToExportName;
  std::unordered_map<Prototype*, std::string, AddressHasher<Prototype*>> functionToExternName;
  std::unordered_map<Kind*, std::string, AddressHasher<Kind*>> kindToExternName;

  Package(
    AddressNumberer* addressNumberer,
    PackageCoordinate* packageCoordinate_,
    std::unordered_map<std::string, InterfaceDefinition*> interfaces_,
    std::unordered_map<std::string, StructDefinition*> structs_,
    std::unordered_map<std::string, StaticSizedArrayDefinitionT*> staticSizedArrays_,
    std::unordered_map<std::string, RuntimeSizedArrayDefinitionT*> runtimeSizedArrays_,
    std::unordered_map<std::string, Function*> functions_,
//    std::unordered_map<Kind*, Prototype*, AddressHasher<Kind*>> immDestructorsByKind_,
    std::unordered_map<std::string, Prototype*> exportNameToFunction_,
    std::unordered_map<std::string, Kind*> exportNameToKind_,
    std::unordered_map<std::string, Prototype*> externNameToFunction_,
    std::unordered_map<std::string, Kind*> externNameToKind_,
    std::unordered_map<std::string, OpaqueStructLayout> structLayouts_,
    std::unordered_map<std::string, ExternAbi> externAbis_) :
      packageCoordinate(packageCoordinate_),
      interfaces(std::move(interfaces_)),
      structs(std::move(structs_)),
      staticSizedArrays(std::move(staticSizedArrays_)),
      runtimeSizedArrays(std::move(runtimeSizedArrays_)),
      functions(std::move(functions_)),
//      immDestructorsByKind(std::move(immDestructorsByKind_)),
      exportNameToFunction(std::move(exportNameToFunction_)),
      exportNameToKind(std::move(exportNameToKind_)),
      externNameToFunction(std::move(externNameToFunction_)),
      externNameToKind(std::move(externNameToKind_)),
      structLayouts(std::move(structLayouts_)),
      externAbis(std::move(externAbis_)),
      functionToExportName(0, addressNumberer->makeHasher<Prototype*>()),
      kindToExportName(0, addressNumberer->makeHasher<Kind*>()),
      functionToExternName(0, addressNumberer->makeHasher<Prototype*>()),
      kindToExternName(0, addressNumberer->makeHasher<Kind*>()) {
    for (auto [exportName, prototype] : exportNameToFunction) {
      assert(functionToExportName.count(prototype) == 0);
      functionToExportName[prototype] = exportName;
    }
    for (auto [exportName, kind] : exportNameToKind) {
      assert(kindToExportName.count(kind) == 0);
      kindToExportName[kind] = exportName;
    }
    for (auto [externName, prototype] : externNameToFunction) {
      assert(functionToExternName.count(prototype) == 0);
      functionToExternName[prototype] = externName;
    }
    for (auto [externName, kind] : externNameToKind) {
      assert(kindToExternName.count(kind) == 0);
      kindToExternName[kind] = externName;
    }
  }

  Function* getFunction(Name* name) {
    auto iter = functions.find(name->name);
    if (iter == functions.end()) {
      std::cerr << "Couldn't find function: " << name->name << std::endl;
      exit(1);
    }
    return iter->second;
  }
  std::optional<Function*> getMaybeFunction(Name* name) {
    auto iter = functions.find(name->name);
    if (iter == functions.end()) {
      return std::nullopt;
    }
    return std::optional(iter->second);
  }
  StructDefinition* getStruct(StructKind* structMT) {
    auto iter = structs.find(structMT->fullName->name);
    if (iter == structs.end()) {
      std::cerr << "Couldn't find struct: " << structMT->fullName->name << std::endl;
      exit(1);
    }
    return iter->second;
  }
  std::optional<StructDefinition*> getMaybeStruct(Name* name) {
    auto iter = structs.find(name->name);
    if (iter == structs.end()) {
      return std::nullopt;
    }
    return std::optional(iter->second);
  }
  InterfaceDefinition* getInterface(InterfaceKind* interfaceMT) {
    auto iter = interfaces.find(interfaceMT->fullName->name);
    assert(iter != interfaces.end());
    return iter->second;
  }
  std::optional<InterfaceDefinition*> getMaybeInterface(InterfaceKind* interfaceMT) {
    auto iter = interfaces.find(interfaceMT->fullName->name);
    if (iter == interfaces.end()) {
      return std::nullopt;
    }
    return std::optional(iter->second);
  }
  StaticSizedArrayDefinitionT* getStaticSizedArray(StaticSizedArrayT* ssaMT) {
    auto iter = staticSizedArrays.find(ssaMT->name->name);
    assert(iter != staticSizedArrays.end());
    return iter->second;
  }
  std::optional<StaticSizedArrayDefinitionT*> getMaybeStaticSizedArray(Name* name) {
    auto iter = staticSizedArrays.find(name->name);
    if (iter == staticSizedArrays.end()) {
      return std::nullopt;
    }
    return std::optional(iter->second);
  }
  RuntimeSizedArrayDefinitionT* getRuntimeSizedArray(RuntimeSizedArrayT* rsaMT) {
    auto iter = runtimeSizedArrays.find(rsaMT->name->name);
    assert(iter != runtimeSizedArrays.end());
    return iter->second;
  }
  std::optional<RuntimeSizedArrayDefinitionT*> getMaybeRuntimeSizedArray(Name* name) {
    auto iter = runtimeSizedArrays.find(name->name);
    if (iter == runtimeSizedArrays.end()) {
      return std::nullopt;
    }
    return std::optional(iter->second);
  }
//  Prototype* getImmDestructor(Kind* kind) {
//    auto iter = immDestructorsByKind.find(kind);
//    assert(iter != immDestructorsByKind.end());
//    return iter->second;
//  }

  std::string getKindExportName(ValueKind* kind, bool includeProjectName) const {
    // The export name of a kind is its own C ABI type name; the onion wrap a reference adds is not
    // part of it, so callers pass the peeled ValueKind.
    if (auto innt = dynamic_cast<Int *>(kind)) {
      return std::string() + "int" + std::to_string(innt->bits) + "_t";
    } else if (dynamic_cast<Bool *>(kind)) {
      return "int8_t";
    } else if (dynamic_cast<Float *>(kind)) {
      return "double";
    } else if (dynamic_cast<Str *>(kind)) {
      // VCOORD: revisit this
      // Under the opaque-handle FFI, str crosses as a 32-byte handle typedef'd
      // per package as `<projName>_str`. See generateStrHeaderPerPackage.
      return (includeProjectName ? packageCoordinate->projectName + "_" : "") + "str";
    } else {
      auto iter = kindToExportName.find(kind);
      if (iter == kindToExportName.end()) {
        std::cerr << "Couldn't find export name for: " << getKindHumanName(kind) << std::endl;
        exit(1);
      }
      return (includeProjectName ? packageCoordinate->projectName + "_" : "") + iter->second;
    }
  }
  std::string getKindHumanName(ValueKind* kind) const {
    if (auto innt = dynamic_cast<Int *>(kind)) {
      return std::string() + "i" + std::to_string(innt->bits);
    } else if (dynamic_cast<Bool *>(kind)) {
      return "bool";
    } else if (dynamic_cast<Float *>(kind)) {
      return "double";
    } else if (dynamic_cast<Str *>(kind)) {
      return "str";
    } else if (auto struuct = dynamic_cast<StructKind *>(kind)) {
      return struuct->fullName->name;
    } else if (auto interface = dynamic_cast<InterfaceKind *>(kind)) {
      return interface->fullName->name;
    } else if (auto ssaMT = dynamic_cast<StaticSizedArrayT *>(kind)) {
      return ssaMT->name->name;
    } else if (auto rsaMT = dynamic_cast<RuntimeSizedArrayT *>(kind)) {
      return rsaMT->name->name;
    } else {
      { assert(false); throw 1337; }
    }
  }
  std::string getFunctionExportName(Prototype* kind) const {
    auto iter = functionToExportName.find(kind);
    assert(iter != functionToExportName.end());
    return packageCoordinate->projectName + "_" + iter->second;
  }
  std::string getKindExternName(ValueKind* kind) const {
    auto iter = kindToExternName.find(kind);
    assert(iter != kindToExternName.end());
    return packageCoordinate->projectName + "_" + iter->second;
  }
  // The extern's real callee symbol, verbatim: the human C name for a C extern, or rustc's mangled
  // name for a Rust-interop leaf. This is not the the Valen-generated shim that calls out to C.
  // See @BDCABIBZ.
  std::string getFunctionExternName(Prototype* kind) const {
    auto iter = functionToExternName.find(kind);
    assert(iter != functionToExternName.end());
    return iter->second;
  }
//  bool isExported(Name* name) {
//    auto exportedNameI = fullNameToExportName.find(name);
//    return exportedNameI != fullNameToExportName.end();
//  }
//  std::string getExportedName(Name* name) {
//    auto exportedNameI = fullNameToExportName.find(name);
//    if (exportedNameI == fullNameToExportName.end()) {
//      std::cerr << "No exported name for " << name->name << std::endl;
//      { assert(false); throw 1337; }
//    }
//    return exportedNameI->second;
//  }
//  bool isExportedAs(Name* name, const std::string& exportName) {
//    auto exportedNamesI = fullNameToExportName.find(name);
//    if (exportedNamesI == fullNameToExportName.end()) {
//      return false;
//    }
//    return exportedNamesI->second == exportName;
//  }
};

class Program {
public:
  std::unordered_map<PackageCoordinate*, Package*, AddressHasher<PackageCoordinate*>, std::equal_to<PackageCoordinate*>> packages;

  Program(
      std::unordered_map<PackageCoordinate*, Package*, AddressHasher<PackageCoordinate*>, std::equal_to<PackageCoordinate*>> packages_) :
      packages(std::move(packages_)) {}

  Package* getPackage(PackageCoordinate* packageCoord) {
    auto iter = packages.find(packageCoord);
    if (iter == packages.end()) {
      std::cerr << "Couldn't find package: " << packageCoord->projectName;
      for (auto i : packageCoord->packageSteps) {
        std::cerr << "." << i;
      }
      std::cerr << ", aborting." << std::endl;
      exit(1);
    }
    return iter->second;
  }

  Function* getFunction(Name* name) {
    return getPackage(name->packageCoord)->getFunction(name);
  }
  std::optional<Function*> getMaybeFunction(Name* name) {
    return getPackage(name->packageCoord)->getMaybeFunction(name);
  }
  StructDefinition* getStruct(StructKind* structMT) {
    return getPackage(structMT->fullName->packageCoord)->getStruct(structMT);
  }
  std::optional<StructDefinition*> getMaybeStruct(StructKind* structMT) {
    return getPackage(structMT->fullName->packageCoord)->getStruct(structMT);
  }
  InterfaceDefinition* getInterface(InterfaceKind* interfaceMT) {
    return getPackage(interfaceMT->fullName->packageCoord)->getInterface(interfaceMT);
  }
  std::optional<InterfaceDefinition*> getMaybeInterface(InterfaceKind* interfaceMT) {
    return getPackage(interfaceMT->fullName->packageCoord)->getMaybeInterface(interfaceMT);
  }
  StaticSizedArrayDefinitionT* getStaticSizedArray(StaticSizedArrayT* ssaMT) {
    return getPackage(ssaMT->name->packageCoord)->getStaticSizedArray(ssaMT);
  }
  RuntimeSizedArrayDefinitionT* getRuntimeSizedArray(RuntimeSizedArrayT* rsaMT) {
    return getPackage(rsaMT->name->packageCoord)->getRuntimeSizedArray(rsaMT);
  }
//  Prototype* getImmDestructor(Kind* kind) {
//    return getPackage(kind->getPackageCoordinate())->getImmDestructor(kind);
//  }
//  bool isExported(Name* name) {
//    return getPackage(name->packageCoord)->isExported(name);
//  }
//  std::vector<std::string> getExportedNames(Name* name) {
//    auto exportedNameI = fullNameToExportedNames.find(name);
//    if (exportedNameI == fullNameToExportedNames.end()) {
//      std::cerr << "No exported name for " << name->name << std::endl;
//      { assert(false); throw 1337; }
//    }
//    return exportedNameI->second;
//  }
//  bool isExportedAs(Name* name, const std::string& exportName) {
//    return getPackage(name->packageCoord)->isExportedAs(name, exportName);
//  }
};

class InterfaceMethod {
public:
  Prototype* prototype;
  int virtualParamIndex;

  InterfaceMethod(
      Prototype* prototype_,
      int virtualParamIndex_) :
      prototype(prototype_),
      virtualParamIndex(virtualParamIndex_) {}
};

// Represents how a struct implements an interface.
// Each edge has a vtable.
class Edge {
public:
  StructKind* structName;
  InterfaceKind* interfaceName;
  std::vector<std::pair<InterfaceMethod*, Prototype*>> structPrototypesByInterfaceMethod;

  Edge(
      StructKind* structName_,
      InterfaceKind* interfaceName_,
      std::vector<std::pair<InterfaceMethod*, Prototype*>> structPrototypesByInterfaceMethod_) :
      structName(structName_),
      interfaceName(interfaceName_),
      structPrototypesByInterfaceMethod(structPrototypesByInterfaceMethod_) {}
};

class StructDefinition {
public:
    Name* name;
    StructKind* kind;
    RegionId* regionId;
    Sharedness sharedness;
    std::vector<Edge*> edges;
    std::vector<StructMember*> members;
    Weakability weakability;

    StructDefinition(
        Name* name_,
        StructKind* kind_,
        RegionId* regionId_,
        Sharedness sharedness_,
        std::vector<Edge*> edges_,
        std::vector<StructMember*> members_,
        Weakability weakable_) :
        name(name_),
        kind(kind_),
        regionId(regionId_),
        sharedness(sharedness_),
        edges(edges_),
        members(members_),
        weakability(weakable_) {}

    Edge* getEdgeForInterface(InterfaceKind* interfaceMT) {
      for (auto e : edges) {
        if (e->interfaceName == interfaceMT)
          return e;
      }
      { assert(false); throw 1337; }
      return nullptr;
    }
};

class StructMember {
public:
    std::string fullName;
    std::string name;
    Kind* type;

    StructMember(
        std::string fullName_,
        std::string name_,
        Kind* type_) :
        fullName(fullName_),
        name(name_),
        type(type_) {}
};


class InterfaceDefinition {
public:
    Name* name;
    InterfaceKind* kind;
    RegionId* regionId;
    Sharedness sharedness;
    std::vector<Name*> superInterfaces;
    std::vector<InterfaceMethod*> methods;
    Weakability weakability;

    InterfaceDefinition(
        Name* name_,
        InterfaceKind* kind_,
        RegionId* regionId_,
        Sharedness sharedness_,
        const std::vector<Name*>& superInterfaces_,
        const std::vector<InterfaceMethod*>& methods_,
        Weakability weakable_) :
      name(name_),
      kind(kind_),
      regionId(regionId_),
      sharedness(sharedness_),
      superInterfaces(superInterfaces_),
      methods(methods_),
      weakability(weakable_) {
      assert((uint64_t)name > 0x10000);
      assert((uint64_t)kind > 0x10000);
    }
};

class Function {
public:
    Prototype* prototype;
    Expression* block;

    Function(

        Prototype* prototype_,
    Expression* block_
        ) :
        prototype(prototype_),
        block(block_) {}
};

// Interned
// Onion: types are onion Kind* (ownership is the wrap; placement is derived at codegen).
class Prototype {
public:
    Name* name;
    std::vector<Kind*> params;
    Kind* returnType;

    Prototype(
        Name* name_,
        std::vector<Kind*> params_,
        Kind* returnType_) :
      name(name_),
      params(std::move(params_)),
      returnType(returnType_) {}
};

// A variable's identity. Unique only within the containing function.
struct VarNameM {
  std::string name;

  bool operator==(const VarNameM& other) const { return name == other.name; }
};

class Local {
public:
  VarNameM id;
  std::string name;
  Kind* type;

  Local(
      VarNameM id_,
      std::string name_,
      Kind* type_) :
      id(std::move(id_)),
      name(std::move(name_)),
      type(type_) {}
};

namespace std {
  template<> struct hash<VarNameM> {
    size_t operator()(const VarNameM& v) const { return std::hash<std::string>()(v.name); }
  };
}

#endif
