#include "structlt.h"
#include "globalstate.h"
#include "function/expressions/shared/shared.h"

// StructLT / StructBuilderLT are header-only templates. This translation unit
// previously held buildCompressStructInner / buildDecompressStructInner, the
// bit-packing that squeezed the 32-byte universal ref into an i256; those were
// retired when FFI handles became right-sized structs (see ffihandlestructs.h).
