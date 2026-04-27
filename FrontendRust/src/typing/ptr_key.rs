// Guardian: disable-all
use std::hash::{Hash, Hasher};

/// Value-type (see @TFITCX)
pub struct PtrKey<'t, T: ?Sized>(pub &'t T);

impl<'t, T: ?Sized> PartialEq for PtrKey<'t, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl<'t, T: ?Sized> Eq for PtrKey<'t, T> {}

impl<'t, T: ?Sized> Hash for PtrKey<'t, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0 as *const T as *const ()).hash(state)
    }
}

impl<'t, T: ?Sized> Copy for PtrKey<'t, T> {}

impl<'t, T: ?Sized> Clone for PtrKey<'t, T> {
    fn clone(&self) -> Self { *self }
}

impl<'t, T: ?Sized + std::fmt::Debug> std::fmt::Debug for PtrKey<'t, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PtrKey({:p})", self.0 as *const T)
    }
}
