#include "types.h"
#include "ast.h"

bool isValueType(Kind* kind) {
  return dynamic_cast<BorrowRef*>(kind) == nullptr &&
      dynamic_cast<OwnRef*>(kind) == nullptr &&
      dynamic_cast<ShareRef*>(kind) == nullptr &&
      dynamic_cast<WeakRef*>(kind) == nullptr;
}
