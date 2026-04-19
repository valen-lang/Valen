use std::marker::PhantomData;

use crate::typing::ast::ast::{PrototypeT, PrototypeValT, SignatureT, SignatureValT};
use crate::typing::names::names::{IdT, IdValT};

// TODO: placeholder — replace with real fields (bump, RefCell<Inner>) during Slab 2–3.
pub struct TypingInterner<'t>(pub PhantomData<&'t ()>);

impl<'t> TypingInterner<'t> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn intern_id<'s, 'tmp>(
        &self,
        _val: IdValT<'s, 't, 'tmp>,
    ) -> &'t IdT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_id not yet implemented")
    }

    pub fn intern_prototype<'s, 'tmp>(
        &self, _val: PrototypeValT<'s, 't, 'tmp>,
    ) -> &'t PrototypeT<'s, 't>
    where 's: 't, 't: 'tmp {
        panic!("TypingInterner::intern_prototype not yet implemented")
    }

    pub fn intern_signature<'s, 'tmp>(
        &self, _val: SignatureValT<'s, 't, 'tmp>,
    ) -> &'t SignatureT<'s, 't>
    where 's: 't, 't: 'tmp {
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
