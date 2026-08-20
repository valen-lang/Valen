#ifndef METAL_CACHE_H_
#define METAL_CACHE_H_

#include <unordered_map>

#include "types.h"
#include "ast.h"
#include "instructions.h"

namespace std {
    template<>
    struct hash<Location> {
        inline size_t operator()(Location location) const {
            return (size_t)location;
        }
    };
    template<>
    struct hash<Ownership> {
        inline size_t operator()(Ownership ownership) const {
            return (size_t)ownership;
        }
    };
    template<>
    struct hash<Sharedness> {
        inline size_t operator()(Sharedness sharedness) const {
            return (size_t)sharedness;
        }
    };
}

template<typename K, typename V, typename H, typename E, typename F>
V& makeIfNotPresent(std::unordered_map<K, V, H, E>* map, const K& key, F&& makeElement) {
  auto iter = map->find(key);
  if (iter == map->end()) {
    auto p = map->emplace(key, makeElement());
    iter = p.first;
  }
  return iter->second;
}

// Prototype interning keys on its param list of onion Kind*.
struct HashKindVec {
  AddressHasher<Kind*> hasher;
  HashKindVec(AddressHasher<Kind*> hasher_) : hasher(hasher_) {}

  size_t operator()(const std::vector<Kind *> &kinds) const {
    size_t result = 1337;
    for (auto el : kinds) {
      result += (size_t) el;
    }
    return result;
  }
};
struct KindVecEquals {
  bool operator()(
      const std::vector<Kind *> &a,
      const std::vector<Kind *> &b) const {
    if (a.size() != b.size())
      return false;
    for (size_t i = 0; i < a.size(); i++) {
      if (a[i] != b[i])
        return false;
    }
    return true;
  }
};

class MetalCache {
public:
  explicit MetalCache(AddressNumberer* addressNumberer_) :
      addressNumberer(addressNumberer_),
      structKinds(0, addressNumberer->makeHasher<Name*>()),
      interfaceKinds(0, addressNumberer->makeHasher<Name*>()),
      names(0, addressNumberer->makeHasher<PackageCoordinate*>()),
      ints(0, addressNumberer->makeHasher<RegionId*>()),
      bools(0, addressNumberer->makeHasher<RegionId*>()),
      strs(0, addressNumberer->makeHasher<RegionId*>()),
      floats(0, addressNumberer->makeHasher<RegionId*>()),
      nevers(0, addressNumberer->makeHasher<RegionId*>()),
      voids(0, addressNumberer->makeHasher<RegionId*>()),
      usizes(0, addressNumberer->makeHasher<RegionId*>()),
      borrowRefs(0, addressNumberer->makeHasher<Kind*>()),
      ownRefs(0, addressNumberer->makeHasher<Kind*>()),
      shareRefs(0, addressNumberer->makeHasher<Kind*>()),
      weakRefs(0, addressNumberer->makeHasher<Kind*>()),
      runtimeSizedArrays(0, addressNumberer->makeHasher<Name*>()),
      staticSizedArrays(0, addressNumberer->makeHasher<Name*>()),
      unconvertedReferences(0, addressNumberer->makeHasher<Kind*>()),
      prototypes(0, addressNumberer->makeHasher<Name*>()),
      interfaceMethods(0, addressNumberer->makeHasher<Prototype*>()) {

    builtinPackageCoord = getPackageCoordinate(BUILTIN_PROJECT_NAME, {});
    rcImmRegionId = getRegionId(builtinPackageCoord, "rcimm");
    mutRegionId = getRegionId(builtinPackageCoord, "mut");

    // Base primitive kind singletons. Ownership and placement are not stored: a bare kind is
    // owned; the wrap layers (getBorrowRef/getShareRef/…) express references, and placement is
    // derived from the onion shape at codegen.
    // VCOORD: Str should be different soon
    i32 = getInt(rcImmRegionId, 32);
    i64 = getInt(rcImmRegionId, 64);
    boool = getBool(rcImmRegionId);
    flooat = getFloat(rcImmRegionId);
    str = getStr(rcImmRegionId);
    never = getNever(rcImmRegionId);
    vooid = getVoid(rcImmRegionId);

    // Codegen-internal Reference singletons for the primitives. The region layer still runs on
    // Reference{ownership, location, kind}; these are the canonical references codegen builds from
    // the bare kinds (a primitive is OWN and INLINE; a string is a shared yonder value).
    i32Ref = getReference(Ownership::OWN, Location::INLINE, i32);
    i64Ref = getReference(Ownership::OWN, Location::INLINE, i64);
    boolRef = getReference(Ownership::OWN, Location::INLINE, boool);
    floatRef = getReference(Ownership::OWN, Location::INLINE, flooat);
    mutStrRef = getReference(Ownership::MUTABLE_SHARE, Location::YONDER, str);
    immStrRef = getReference(Ownership::IMMUTABLE_SHARE, Location::YONDER, str);
    neverRef = getReference(Ownership::OWN, Location::INLINE, never);
    voidRef = getReference(Ownership::OWN, Location::INLINE, vooid);
  }

  PackageCoordinate* getPackageCoordinate(const std::string& projectName, const std::vector<std::string>& packageSteps) {
    return makeIfNotPresent(
        &packageCoords[projectName],
        packageSteps,
        [&](){ return new PackageCoordinate{projectName, packageSteps}; });
  }

  Int* getInt(RegionId* regionId, int bits) {
    return makeIfNotPresent(
        &ints[regionId],
        bits,
        [&](){ return new Int(regionId, bits); });
  }

  Bool* getBool(RegionId* regionId) {
    return makeIfNotPresent(
        &bools,
        regionId,
        [&](){ return new Bool(regionId); });
  }

  Str* getStr(RegionId* regionId) {
    return makeIfNotPresent(
        &strs,
        regionId,
        [&](){ return new Str(regionId); });
  }

  Float* getFloat(RegionId* regionId) {
    return makeIfNotPresent(
        &floats,
        regionId,
        [&](){ return new Float(regionId); });
  }

  Void* getVoid(RegionId* regionId) {
    return makeIfNotPresent(
        &voids,
        regionId,
        [&](){ return new Void(regionId); });
  }

  Never* getNever(RegionId* regionId) {
    return makeIfNotPresent(
        &nevers,
        regionId,
        [&](){ return new Never(regionId); });
  }

  USize* getUSize(RegionId* regionId) {
    return makeIfNotPresent(
        &usizes,
        regionId,
        [&](){ return new USize(regionId); });
  }

  StructKind* getStructKind(Name* structName) {
    return makeIfNotPresent(
        &structKinds,
        structName,
        [&]() { return new StructKind(structName); });
  }

  InterfaceKind* getInterfaceKind(Name* structName) {
    return makeIfNotPresent(
        &interfaceKinds,
        structName,
        [&]() { return new InterfaceKind(structName); });
  }

  RuntimeSizedArrayT* getRuntimeSizedArray(Name* name) {
    return makeIfNotPresent(
        &runtimeSizedArrays,
        name,
        [&](){ return new RuntimeSizedArrayT(name); });
  }

  StaticSizedArrayT* getStaticSizedArray(Name* name) {
    return makeIfNotPresent(
        &staticSizedArrays,
        name,
        [&](){ return new StaticSizedArrayT(name); });
  }

  Name* getName(PackageCoordinate* packageCoordinate, std::string nameStr) {
    return makeIfNotPresent(
        &names[packageCoordinate],
        nameStr,
        [&](){ return new Name(packageCoordinate, nameStr); });
  }

  RegionId* getRegionId(PackageCoordinate* packageCoordinate, std::string nameStr) {
    return makeIfNotPresent(
        &regionIds,
        nameStr,
        [&](){ return new RegionId(packageCoordinate, nameStr); });
  }

  // Onion wrap kinds: ownership as a layer around the base kind, interned by the inner kind.
  BorrowRef* getBorrowRef(Kind* inner) {
    return makeIfNotPresent(&borrowRefs, inner, [&](){ return new BorrowRef(inner); });
  }
  OwnRef* getOwnRef(Kind* inner) {
    return makeIfNotPresent(&ownRefs, inner, [&](){ return new OwnRef(inner); });
  }
  ShareRef* getShareRef(Kind* inner) {
    return makeIfNotPresent(&shareRefs, inner, [&](){ return new ShareRef(inner); });
  }
  WeakRef* getWeakRef(Kind* inner) {
    return makeIfNotPresent(&weakRefs, inner, [&](){ return new WeakRef(inner); });
  }

  // Codegen-internal: intern a Reference{ownership, location, kind}. The onion IR no longer hands
  // codegen a Reference (it hands an onion Kind*); codegen derives one via refFromKind and interns
  // it here so the region layer, which still runs on Reference*, keeps working.
  Reference* getReference(Ownership ownership, Location location, Kind* kind) {
    return makeIfNotPresent<Location, Reference*>(
        &unconvertedReferences[kind][ownership],
        location,
        [&](){ return new Reference(ownership, location, kind); });
  }

  Prototype* getPrototype(Name* name, Kind* returnType, std::vector<Kind*> paramTypes) {
    return makeIfNotPresent(
        &makeIfNotPresent(
            &makeIfNotPresent(
                &prototypes,
                name,
                [&](){ return PrototypeByParamListByReturnTypeMap(0, AddressHasher<Kind*>(addressNumberer)); }),
            returnType,
            [&](){ return PrototypeByParamListMap(0, HashKindVec(addressNumberer)); }),
        paramTypes,
        [&](){ return new Prototype(name, paramTypes, returnType); });
  }

  InterfaceMethod* getInterfaceMethod(Prototype* prototype, int virtualParamIndex) {
    return makeIfNotPresent(
        &interfaceMethods[prototype],
        virtualParamIndex,
        [&](){ return new InterfaceMethod(prototype, virtualParamIndex); });
  }

  AddressNumberer* addressNumberer;

  std::unordered_map<std::string, RegionId*> regionIds;
  std::unordered_map<Name*, StructKind*, AddressHasher<Name*>> structKinds;
  std::unordered_map<Name*, InterfaceKind*, AddressHasher<Name*>> interfaceKinds;
  std::unordered_map<PackageCoordinate*, std::unordered_map<std::string, Name*>, AddressHasher<PackageCoordinate*>> names;

  std::unordered_map<std::string, std::unordered_map<std::vector<std::string>, PackageCoordinate*, PackageCoordinate::StringVectorHasher, PackageCoordinate::StringVectorEquator>> packageCoords;
  std::unordered_map<RegionId*, std::unordered_map<int, Int*>, AddressHasher<RegionId*>> ints;
  std::unordered_map<RegionId*, Bool*, AddressHasher<RegionId*>> bools;
  std::unordered_map<RegionId*, Str*, AddressHasher<RegionId*>> strs;
  std::unordered_map<RegionId*, Float*, AddressHasher<RegionId*>> floats;
  std::unordered_map<RegionId*, Void*, AddressHasher<RegionId*>> voids;
  std::unordered_map<RegionId*, Never*, AddressHasher<RegionId*>> nevers;
  std::unordered_map<RegionId*, USize*, AddressHasher<RegionId*>> usizes;

  // Onion wrap-kind interning, keyed by the inner kind.
  std::unordered_map<Kind*, BorrowRef*, AddressHasher<Kind*>> borrowRefs;
  std::unordered_map<Kind*, OwnRef*, AddressHasher<Kind*>> ownRefs;
  std::unordered_map<Kind*, ShareRef*, AddressHasher<Kind*>> shareRefs;
  std::unordered_map<Kind*, WeakRef*, AddressHasher<Kind*>> weakRefs;

  std::unordered_map<Name*, RuntimeSizedArrayT*, AddressHasher<Name*>> runtimeSizedArrays;
  std::unordered_map<Name*, StaticSizedArrayT*, AddressHasher<Name*>> staticSizedArrays;

  // Codegen-internal Reference interning, keyed kind -> ownership -> location.
  std::unordered_map<
      Kind*,
      std::unordered_map<
          Ownership,
          std::unordered_map<
              Location,
              Reference*>>,
      AddressHasher<Kind*>> unconvertedReferences;

  using PrototypeByParamListMap =
      std::unordered_map<std::vector<Kind*>, Prototype*, HashKindVec, KindVecEquals>;
  using PrototypeByParamListByReturnTypeMap =
      std::unordered_map<Kind*, PrototypeByParamListMap, AddressHasher<Kind*>>;
  using PrototypeByParamListByReturnTypeByNameMap =
      std::unordered_map<Name*, PrototypeByParamListByReturnTypeMap, AddressHasher<Name*>>;
  PrototypeByParamListByReturnTypeByNameMap prototypes;

  std::unordered_map<Prototype*, std::unordered_map<int, InterfaceMethod*>, AddressHasher<Prototype*>> interfaceMethods;

  RegionId* rcImmRegionId = nullptr;
  RegionId* mutRegionId = nullptr;

  PackageCoordinate* builtinPackageCoord = nullptr;
  Int* i32 = nullptr;
  Reference* i32Ref = nullptr;
  Int* i64 = nullptr;
  Reference* i64Ref = nullptr;
  Bool* boool = nullptr;
  Reference* boolRef = nullptr;
  Float* flooat = nullptr;
  Reference* floatRef = nullptr;
  Str* str = nullptr;
  Reference* immStrRef = nullptr;
  Reference* mutStrRef = nullptr;
  Never* never = nullptr;
  Reference* neverRef = nullptr;
  Void* vooid = nullptr;
  Reference* voidRef = nullptr;
};


#endif
