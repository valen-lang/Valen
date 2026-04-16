use std::marker::PhantomData;

use crate::typing::ast::ast::{PrototypeT, SignatureT};
use crate::typing::names::names::{IdT, INameT};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::KindT;

// TODO: placeholder — replace with real fields (bump, RefCell<Inner>) during Slab 2–3.
pub struct TypingInterner<'t>(pub PhantomData<&'t ()>);

impl<'t> TypingInterner<'t> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn intern_name<'s>(&self, _val: INameValT<'s, 't>) -> &'t INameT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_name not yet implemented")
    }

    pub fn intern_kind<'s>(&self, _val: KindValT<'s, 't>) -> &'t KindT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_kind not yet implemented")
    }

    pub fn intern_id<'s>(&self, _val: IdValT<'s, 't>) -> &'t IdT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_id not yet implemented")
    }

    pub fn intern_templata<'s>(&self, _val: ITemplataValT<'s, 't>) -> &'t ITemplataT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_templata not yet implemented")
    }

    pub fn intern_prototype<'s>(&self, _val: PrototypeValT<'s, 't>) -> &'t PrototypeT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_prototype not yet implemented")
    }

    pub fn intern_signature<'s>(&self, _val: SignatureValT<'s, 't>) -> &'t SignatureT<'s, 't>
    where 's: 't {
        panic!("TypingInterner::intern_signature not yet implemented")
    }
}

// TODO: these *ValT stubs live here temporarily; move next to their ref counterparts
// (KindValT alongside KindT, etc.) during Slab 2–3 when real variants are filled in.
pub struct INameValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct KindValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct IdValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct ITemplataValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct PrototypeValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct SignatureValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
