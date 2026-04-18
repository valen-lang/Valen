use std::marker::PhantomData;

use crate::typing::ast::ast::{PrototypeT, SignatureT};
use crate::typing::names::names::{IdT, IdValT, INameT, IFunctionNameT};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::KindT;

// TODO: placeholder — replace with real fields (bump, RefCell<Inner>) during Slab 2–3.
pub struct TypingInterner<'t>(pub PhantomData<&'t ()>);

impl<'t> TypingInterner<'t> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn intern_kind<'s>(&self, _val: KindValT<'s, 't>) -> &'t KindT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_kind not yet implemented")
    }

    pub fn intern_id<'s, 'tmp, T: Copy>(
        &self,
        _val: IdValT<'s, 't, 'tmp, T>,
    ) -> &'t IdT<'s, 't, T>
    where 's: 't {
        panic!("TypingInterner::intern_id not yet implemented")
    }

    pub fn intern_templata<'s>(&self, _val: ITemplataValT<'s, 't>) -> &'t ITemplataT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_templata not yet implemented")
    }

    pub fn intern_prototype<'s>(&self, _val: PrototypeValT<'s, 't>) -> &'t PrototypeT<'s, 't, IFunctionNameT<'s, 't>>
    where 's: 't {
        panic!("TypingInterner::intern_prototype not yet implemented")
    }

    pub fn intern_signature<'s>(&self, _val: SignatureValT<'s, 't>) -> &'t SignatureT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_signature not yet implemented")
    }
}

// TODO: these remaining *ValT stubs live here temporarily; move next to their
// ref counterparts (KindValT alongside KindT, etc.) during Slab 3–5 when those
// types' real Val-enum variants are filled in. Concrete name *ValT types and
// IdValT have already been moved to names.rs (Slab 2 Step 6). INameT is no
// longer interned under the inline-owned sub-enum design — no INameValT.
pub struct KindValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct ITemplataValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct PrototypeValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct SignatureValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
