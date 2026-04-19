use std::cell::RefCell;
use std::marker::PhantomData;

use bumpalo::Bump;

use crate::typing::ast::ast::{PrototypeT, PrototypeValT, SignatureT, SignatureValT};
use crate::typing::names::names::{IdT, IdValT};

// Substrate for the typing-arena interner. Mirrors ScoutArena's shape:
// `bump` is a borrowed reference to a caller-owned Bump, and `inner` holds
// the per-family HashMaps behind a RefCell.
//
// Slab 4 sets up the substrate here (empty `Inner`) and flips the lifetime
// parameters from `<'t>` to `<'s, 't>`. The per-family HashMap fields and
// real intern bodies land in Step 6.
pub struct TypingInterner<'s, 't>
where 's: 't,
{
    bump: &'t Bump,
    inner: RefCell<Inner<'s, 't>>,
}

struct Inner<'s, 't>
where 's: 't,
{
    // Per-family HashMaps land in Step 6 (one per interned family: ids, prototypes,
    // signatures, ~60 concrete names, 6 kind payloads, 6 templata payloads).
    _phantom: PhantomData<(&'s (), &'t ())>,
}

impl<'s, 't> TypingInterner<'s, 't>
where 's: 't,
{
    pub fn new(bump: &'t Bump) -> Self {
        TypingInterner {
            bump,
            inner: RefCell::new(Inner {
                _phantom: PhantomData,
            }),
        }
    }

    // Arena access — mirrors ScoutArena::alloc / alloc_slice_copy / alloc_slice_from_vec.
    pub fn bump(&self) -> &'t Bump {
        self.bump
    }

    pub fn alloc<T>(&self, val: T) -> &'t mut T {
        self.bump.alloc(val)
    }

    pub fn alloc_slice_copy<T: Copy>(&self, src: &[T]) -> &'t [T] {
        self.bump.alloc_slice_copy(src)
    }

    pub fn alloc_slice_from_vec<T>(&self, vec: Vec<T>) -> &'t [T] {
        self.bump.alloc_slice_fill_iter(vec.into_iter())
    }

    pub fn intern_id<'tmp>(
        &self,
        _val: IdValT<'s, 't, 'tmp>,
    ) -> &'t IdT<'s, 't> {
        panic!("TypingInterner::intern_id not yet implemented")
    }

    pub fn intern_prototype<'tmp>(
        &self, _val: PrototypeValT<'s, 't, 'tmp>,
    ) -> &'t PrototypeT<'s, 't>
    where 't: 'tmp {
        panic!("TypingInterner::intern_prototype not yet implemented")
    }

    pub fn intern_signature<'tmp>(
        &self, _val: SignatureValT<'s, 't, 'tmp>,
    ) -> &'t SignatureT<'s, 't>
    where 't: 'tmp {
        panic!("TypingInterner::intern_signature not yet implemented")
    }
}

// KindT and ITemplataT are inline-owned (not arena-interned), so they have
// no Val companions and no intern methods. See the "reuse struct as Val"
// comment in types/types.rs for the per-concrete-payload story.
//
// Per-concrete intern methods for StructTT/InterfaceTT/etc. (and for the
// interned templata payloads CoordTemplataT/KindTemplataT/etc.) are
// deferred to Slab 4+ when the interner body is implemented.
