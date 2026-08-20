
#ifndef VALE_TYPES_H_
#define VALE_TYPES_H_

#include <string>
#include <vector>
#include <cassert>

#include "name.h"

// Defined elsewhere
class Name;
class PackageCoordinate;
class CodeLocation;

// Defined in this file
class Reference;
class Kind;
class Int;
class Bool;
class Str;
class Void;
class Float;
class Never;
class InterfaceKind;
class StructKind;
class RawArrayT;
class StaticSizedArrayT;
class RuntimeSizedArrayT;
class USize;
class BorrowRef;
class OwnRef;
class ShareRef;
class WeakRef;

enum class Ownership {
  OWN,
  MUTABLE_BORROW,
  IMMUTABLE_BORROW,
  WEAK,
  MUTABLE_SHARE,
  IMMUTABLE_SHARE
};

enum class Weakability {
  WEAKABLE,
  NON_WEAKABLE,
};

enum class Permission {
    READONLY,
    READWRITE,
    EXCLUSIVE_READWRITE
};

enum class Location {
    INLINE,
    YONDER
};

enum class Sharedness {
    SHARED,
    SINGLE
};

enum class Virtuality {
  NORMAL,
  ABSTRACT
};

struct RegionId {
  PackageCoordinate* packageCoord;
  std::string id;

  RegionId(PackageCoordinate* packageCoord_, std::string id_) :
      packageCoord(packageCoord_), id(id_) {}
};

// Interned
class Reference {
public:
  Ownership ownership;
  Location location;
  Kind* kind;
//  std::string debugStr;

  Reference(
      Ownership ownership_,
      Location location_,
      Kind* kind_
//      , const std::string& debugStr_
  ) :
      ownership(ownership_),
      location(location_),
      kind(kind_)
//    , debugStr(debugStr_)
  {

    if (location == Location::INLINE) {
      assert(ownership == Ownership::OWN || ownership == Ownership::MUTABLE_SHARE);
    }
    if (ownership == Ownership::MUTABLE_BORROW || ownership == Ownership::IMMUTABLE_BORROW || ownership == Ownership::WEAK) {
      assert(location == Location::YONDER);
    }
  }

  // Someday, have a nice way to print out this Reference...
  std::string str() { return ""; }
};

class Kind {
public:
    virtual ~Kind() {}
    virtual PackageCoordinate* getPackageCoordinate() const = 0;
};

class Int : public Kind {
public:
  RegionId* regionId;
  int bits;

  Int(RegionId* regionId_, int bits_) :
      regionId(regionId_),
      bits(bits_) {}

  PackageCoordinate* getPackageCoordinate() const override { return regionId->packageCoord; }
};

class Bool : public Kind {
public:
  RegionId* regionId;

  Bool(RegionId* regionId_) :
      regionId(regionId_) {}

  PackageCoordinate* getPackageCoordinate() const override { return regionId->packageCoord; }
};

class Str : public Kind {
public:
  RegionId* regionId;

  Str(RegionId* regionId_) :
      regionId(regionId_) {}

  PackageCoordinate* getPackageCoordinate() const override { return regionId->packageCoord; }
};

class Float : public Kind {
public:
  RegionId* regionId;

  Float(RegionId* regionId_) :
      regionId(regionId_) {}

  PackageCoordinate* getPackageCoordinate() const override { return regionId->packageCoord; }
};

class Never : public Kind {
public:
  RegionId* regionId;

  Never(RegionId* regionId_) :
      regionId(regionId_) {}

  PackageCoordinate* getPackageCoordinate() const override { return regionId->packageCoord; }
};

class Void : public Kind {
public:
  RegionId* regionId;

  Void(RegionId* regionId_) :
      regionId(regionId_) {}

  PackageCoordinate* getPackageCoordinate() const override { return regionId->packageCoord; }
};

class InterfaceKind : public Kind {
public:
    Name* fullName;

  InterfaceKind(Name* fullName_) :
      fullName(fullName_) {}

  PackageCoordinate* getPackageCoordinate() const override { return fullName->packageCoord; }

};

// Interned
class StructKind : public Kind {
public:
    Name* fullName;

    StructKind(Name* fullName_) :
        fullName(fullName_) {}

  PackageCoordinate* getPackageCoordinate() const override { return fullName->packageCoord; }
};


// Interned
class StaticSizedArrayT : public Kind {
public:
  Name* name;

  StaticSizedArrayT(
      Name* name_) :
      name(name_) {}

  PackageCoordinate* getPackageCoordinate() const override { return name->packageCoord; }
};

class StaticSizedArrayDefinitionT {
public:
  Name* name;
  StaticSizedArrayT* kind;
  int size;
  RegionId* regionId;
  Kind *elementType;

  StaticSizedArrayDefinitionT(
      Name* name_,
      StaticSizedArrayT* kind_,
      int size_,
      RegionId* regionId_,
      Kind* elementType_) :
      name(name_),
      kind(kind_),
      size(size_),
      regionId(regionId_),
      elementType(elementType_) {}

};



// Interned
class RuntimeSizedArrayT : public Kind {
public:
  Name* name;

  RuntimeSizedArrayT(
      Name* name_) :
      name(name_) {}

  PackageCoordinate* getPackageCoordinate() const override { return name->packageCoord; }
};

class RuntimeSizedArrayDefinitionT {
public:
  Name* name;
  RuntimeSizedArrayT* kind;
  RegionId* regionId;
  Kind *elementType;

  RuntimeSizedArrayDefinitionT(
      Name* name_,
      RuntimeSizedArrayT* kind_,
      RegionId* regionId_,
      Kind* elementType_) :
      name(name_),
      kind(kind_),
      regionId(regionId_),
      elementType(elementType_) {}
};


class USize : public Kind {
public:
  RegionId* regionId;

  USize(RegionId* regionId_) :
      regionId(regionId_) {}

  PackageCoordinate* getPackageCoordinate() const override { return regionId->packageCoord; }
};


// The onion "wrap" layers, mirroring the instantiated IR's KindIT
// (BorrowRefIT / OwnRefIT / ShareRefIT / WeakRefIT). Ownership is which wrap surrounds the
// base kind, or none: an owned value is a bare kind with zero wraps (an owned Ship is a
// StructKind directly). Placement (inline vs yonder) is derived from this shape at codegen,
// not stored, so there is no ownership/location field here. There is no region/group on a wrap.
class BorrowRef : public Kind {
public:
  Kind* inner;

  BorrowRef(Kind* inner_) :
      inner(inner_) {}

  PackageCoordinate* getPackageCoordinate() const override { return inner->getPackageCoordinate(); }
};

class OwnRef : public Kind {
public:
  Kind* inner;

  OwnRef(Kind* inner_) :
      inner(inner_) {}

  PackageCoordinate* getPackageCoordinate() const override { return inner->getPackageCoordinate(); }
};

class ShareRef : public Kind {
public:
  Kind* inner;

  ShareRef(Kind* inner_) :
      inner(inner_) {}

  PackageCoordinate* getPackageCoordinate() const override { return inner->getPackageCoordinate(); }
};

class WeakRef : public Kind {
public:
  Kind* inner;

  WeakRef(Kind* inner_) :
      inner(inner_) {}

  PackageCoordinate* getPackageCoordinate() const override { return inner->getPackageCoordinate(); }
};


class IContainer {
public:
    std::string humanName;
    CodeLocation* location;
};

#endif