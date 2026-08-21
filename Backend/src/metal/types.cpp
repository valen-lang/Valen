#include "types.h"
#include "ast.h"

bool isValueType(Kind* kind) {
  return dynamic_cast<BorrowRef*>(kind) == nullptr &&
      dynamic_cast<OwnRef*>(kind) == nullptr &&
      dynamic_cast<ShareRef*>(kind) == nullptr &&
      dynamic_cast<WeakRef*>(kind) == nullptr;
}

Kind* peel_all_references(Kind* kind) {
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
      return kind;
    }
  }
}
