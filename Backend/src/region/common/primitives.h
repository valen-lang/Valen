#ifndef REGION_COMMON_PRIMITIVES_H_
#define REGION_COMMON_PRIMITIVES_H_

class DefaultPrimitives {
public:
  // We need to pick an arbitrary type to map "Never" to. It shouldn't matter,
  // because the type system uses Never to signal that the program will literally
  // never get there.
  // We arbitrarily use a zero-len array of i57, because it's zero sized and very
  // unlikely to be used anywhere else.
  // See usages of this int to see where we make those zero-len arrays of these.
  static constexpr int NEVER_INT_BITS = 57;

  // Similar to Never, we pick an arbitrary integer to be our void.
  static constexpr int VOID_INT_BITS = 37;


  bool isPrimitive(Reference* referenceM) {
    return dynamic_cast<Void *>(referenceM->kind) != nullptr ||
        dynamic_cast<Int *>(referenceM->kind) != nullptr ||
        dynamic_cast<Bool *>(referenceM->kind) != nullptr ||
        dynamic_cast<Float *>(referenceM->kind) != nullptr;
  }

  // Phase 1 of Option A2 (vcoord-handoff.md): primitives can flow non-Own (borrow-flavor), so
  // translatePrimitive always returns the scalar type regardless of ownership. Phase 2 (when
  // `*int_ptr = 42` semantics land) will dispatch on ownership — scalar for Own, pointer for Borrow.
  LLVMTypeRef translatePrimitive(GlobalState* globalState, Reference* referenceM) {
    if (auto innt = dynamic_cast<Int*>(referenceM->kind)) {
      return LLVMIntTypeInContext(globalState->context, innt->bits);
    } else if (auto vooid = dynamic_cast<Void*>(referenceM->kind)) {
      return LLVMIntTypeInContext(globalState->context, VOID_INT_BITS);
    } else if (dynamic_cast<Bool*>(referenceM->kind) != nullptr) {
      return LLVMInt1TypeInContext(globalState->context);
    } else if (dynamic_cast<Float*>(referenceM->kind) != nullptr) {
      return LLVMDoubleTypeInContext(globalState->context);
    } else if (dynamic_cast<Never*>(referenceM->kind) != nullptr) {
      return LLVMArrayType(LLVMIntTypeInContext(globalState->context, NEVER_INT_BITS), 0);
    } else {
      { assert(false); throw 1337; }
    }
  }
};

#endif
