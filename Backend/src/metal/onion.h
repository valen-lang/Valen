#ifndef METAL_ONION_H_
#define METAL_ONION_H_

#include "types.h"
#include "metalcache.h"

// Bridges the onion IR (bare Kind*) to the codegen's Reference{ownership, location, kind}.
//
// The instruction nodes now carry an onion Kind* whose wrap encodes ownership (a bare kind is
// owned; BorrowRef/ShareRef/WeakRef are references). The region layer still runs on Reference*,
// so codegen derives one here.

// Milestone placement rule: the only inline values are the scalar primitives. Every citizen
// (struct, interface, array) and the heap string are yonder, i.e. a pointer to their own
// allocation. Real inline-vs-yonder placement later replaces only this predicate.
inline bool isInlineKind(Kind* k) {
  return dynamic_cast<Int*>(k) != nullptr ||
      dynamic_cast<Bool*>(k) != nullptr ||
      dynamic_cast<Float*>(k) != nullptr ||
      dynamic_cast<Void*>(k) != nullptr ||
      dynamic_cast<Never*>(k) != nullptr ||
      dynamic_cast<USize*>(k) != nullptr;
}

// Derive an interned codegen Reference from an onion Kind*. Ownership is which wrap surrounds the
// base kind (mirrors the deleted lower_ownership: bare -> OWN, BorrowRef -> MUTABLE_BORROW,
// ShareRef -> MUTABLE_SHARE, WeakRef -> WEAK). Only owned and shared values can be inline (and only
// when primitive); borrow and weak are always yonder pointers, which also keeps the Reference ctor
// invariant (BORROW/WEAK => YONDER) satisfied.
inline Reference* refFromKind(MetalCache* cache, Kind* k) {
  Ownership ownership;
  Kind* base;
  if (auto w = dynamic_cast<OwnRef*>(k)) {
    ownership = Ownership::OWN;
    base = w->inner;
  } else if (auto w = dynamic_cast<BorrowRef*>(k)) {
    ownership = Ownership::MUTABLE_BORROW;
    base = w->inner;
  } else if (auto w = dynamic_cast<ShareRef*>(k)) {
    ownership = Ownership::MUTABLE_SHARE;
    base = w->inner;
  } else if (auto w = dynamic_cast<WeakRef*>(k)) {
    ownership = Ownership::WEAK;
    base = w->inner;
  } else {
    ownership = Ownership::OWN;
    base = k;
  }
  bool ownedOrShared =
      ownership == Ownership::OWN ||
      ownership == Ownership::MUTABLE_SHARE ||
      ownership == Ownership::IMMUTABLE_SHARE;
  auto location =
      (ownedOrShared && isInlineKind(base)) ? Location::INLINE : Location::YONDER;
  return cache->getReference(ownership, location, base);
}

#endif
