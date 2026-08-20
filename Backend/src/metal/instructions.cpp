#include "instructions.h"
#include "metalcache.h"

// resultKind for each instruction: the onion kind it evaluates to. Mirrors the instantiated IR's
// ExpressionIE::result. Nodes that carry a `result` field just return it; the rest name their
// singleton kind (a constant's primitive, Void, Never, Int for array sizes) from the cache.

Kind* ConstantVoid::resultKind(MetalCache* cache) const { return cache->vooid; }
Kind* ConstantInt::resultKind(MetalCache* cache) const { return cache->getInt(cache->rcImmRegionId, bits); }
Kind* ConstantBool::resultKind(MetalCache* cache) const { return cache->boool; }
Kind* ConstantStr::resultKind(MetalCache* cache) const { return result; }
Kind* ConstantF64::resultKind(MetalCache* cache) const { return cache->flooat; }

Kind* Argument::resultKind(MetalCache* cache) const { return tyype; }
Kind* Stackify::resultKind(MetalCache* cache) const { return result; }
Kind* Restackify::resultKind(MetalCache* cache) const { return result; }
Kind* Unstackify::resultKind(MetalCache* cache) const { return result; }
Kind* Destroy::resultKind(MetalCache* cache) const { return cache->vooid; }

Kind* StructToInterfaceUpcast::resultKind(MetalCache* cache) const { return result; }
Kind* InterfaceToInterfaceUpcast::resultKind(MetalCache* cache) const { return result; }
Kind* IsSameInstance::resultKind(MetalCache* cache) const { return cache->boool; }
Kind* WeakAlias::resultKind(MetalCache* cache) const { return result; }
Kind* NewArrayFromValues::resultKind(MetalCache* cache) const { return result; }

Kind* Call::resultKind(MetalCache* cache) const { return result; }
Kind* ExternCall::resultKind(MetalCache* cache) const { return result; }
Kind* InterfaceCall::resultKind(MetalCache* cache) const { return result; }
Kind* If::resultKind(MetalCache* cache) const { return result; }
Kind* While::resultKind(MetalCache* cache) const { return result; }
Kind* Consecutor::resultKind(MetalCache* cache) const { return result; }
Kind* Block::resultKind(MetalCache* cache) const { return result; }
Kind* Break::resultKind(MetalCache* cache) const { return cache->never; }
Kind* Return::resultKind(MetalCache* cache) const { return cache->never; }

Kind* NewRuntimeSizedArray::resultKind(MetalCache* cache) const { return result; }
Kind* StaticArrayFromCallable::resultKind(MetalCache* cache) const { return result; }
Kind* DestroyStaticSizedArrayIntoFunction::resultKind(MetalCache* cache) const { return cache->vooid; }
Kind* DestroyStaticSizedArrayIntoLocals::resultKind(MetalCache* cache) const { return cache->vooid; }
Kind* DestroyRuntimeSizedArray::resultKind(MetalCache* cache) const { return cache->vooid; }
Kind* NewStruct::resultKind(MetalCache* cache) const { return result; }

Kind* ArrayLength::resultKind(MetalCache* cache) const { return cache->i32; }
Kind* ArrayCapacity::resultKind(MetalCache* cache) const { return cache->i32; }
Kind* PushRuntimeSizedArray::resultKind(MetalCache* cache) const { return cache->vooid; }
Kind* PopRuntimeSizedArray::resultKind(MetalCache* cache) const { return result; }
Kind* Discard::resultKind(MetalCache* cache) const { return cache->vooid; }

Kind* LockWeak::resultKind(MetalCache* cache) const { return result; }
Kind* AsSubtype::resultKind(MetalCache* cache) const { return result; }
Kind* CopyPrim::resultKind(MetalCache* cache) const { return result; }

Kind* LetAndLend::resultKind(MetalCache* cache) const { return result; }
Kind* LocalLookup::resultKind(MetalCache* cache) const { return result; }
Kind* Deref::resultKind(MetalCache* cache) const { return result; }
Kind* MemberLookup::resultKind(MetalCache* cache) const { return result; }
Kind* StaticSizedArrayLookup::resultKind(MetalCache* cache) const { return result; }
Kind* RuntimeSizedArrayLookup::resultKind(MetalCache* cache) const { return result; }
Kind* Mutate::resultKind(MetalCache* cache) const { return result; }
Kind* ArraySize::resultKind(MetalCache* cache) const { return result; }
