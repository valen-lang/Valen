#include "types.h"
#include "ast.h"

bool isValueType(Kind* kind) {
  // A value kind is exactly one that is not a reference wrap — i.e. a ValueKind.
  return dynamic_cast<ValueKind*>(kind) != nullptr;
}

ValueKind* peel_all_references(Kind* kind) {
  for (;;) {
    if (auto borrowRef = dynamic_cast<BorrowRef*>(kind)) {
      kind = borrowRef->inner;
    } else if (auto ownRef = dynamic_cast<OwnRef*>(kind)) {
      kind = ownRef->inner;
    } else if (auto shareRef = dynamic_cast<ShareRef*>(kind)) {
      kind = shareRef->inner;
    } else if (auto weakRef = dynamic_cast<WeakRef*>(kind)) {
      kind = weakRef->inner;
    } else {
      // Not a wrap ⇒ a value kind. Every non-wrap Kind subclass derives from ValueKind, so this
      // dynamic_cast always succeeds; it's how we hand back the witness type.
      auto result = dynamic_cast<ValueKind*>(kind);
      assert(result);
      return result;
    }
  }
}
