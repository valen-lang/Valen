use bumpalo::Bump;

use crate::utils::arena_index_map::ArenaIndexMap;
use std::hash::Hash;
use std::marker::PhantomData;

/// Arena wrapper for the instantiating pass. Instantiateds are write-once / read-once and are
/// NOT interned, so this only hands out arena allocations — no deduplication, no canonical
/// identity. (It keeps the `InstantiatingInterner` name and `alloc*` surface the pass already
/// threads; the dedup maps and `intern_*` methods are gone.)
/// Temporary state (see @TFITCX)
pub struct InstantiatingInterner<'s, 'i>
where 's: 'i,
{
    bump: &'i Bump,
    _marker: PhantomData<&'s ()>,
}

impl<'s, 'i> InstantiatingInterner<'s, 'i>
where 's: 'i,
{
    pub fn new(bump: &'i Bump) -> Self {
        InstantiatingInterner { bump, _marker: PhantomData }
    }

    // --- Arena access ---
    pub fn bump(&self) -> &'i Bump { self.bump }
    pub fn alloc<T>(&self, val: T) -> &'i mut T { self.bump.alloc(val) }
    pub fn alloc_slice_copy<T: Copy>(&self, src: &[T]) -> &'i [T] {
        self.bump.alloc_slice_copy(src)
    }
    pub fn alloc_slice_from_vec<T>(&self, vec: Vec<T>) -> &'i [T] {
        self.bump.alloc_slice_fill_iter(vec.into_iter())
    }

    pub fn alloc_index_map<K: Hash + Eq + Clone, V>(&self) -> ArenaIndexMap<'i, K, V> {
        ArenaIndexMap::new_in(self.bump)
    }

    pub fn alloc_index_map_from_iter<K, V, I>(&self, iter: I) -> ArenaIndexMap<'i, K, V>
    where K: Hash + Eq + Clone, I: IntoIterator<Item = (K, V)>
    {
        ArenaIndexMap::from_iter_in(iter, self.bump)
    }
}

// V: figure out where these go
#[cfg(all(test, any()))]
mod tests {
    use super::*;

    #[test]
    fn intern_struct_it_si_canonicalizes() {
        let bump = Bump::new();
        let intr = InstantiatingInterner::new(&bump);

        let v1 = StructITValI::<'_, '_, sI> { id: IdI(PhantomData) };
        let v2 = StructITValI::<'_, '_, sI> { id: IdI(PhantomData) };

        let r1 = intr.intern_struct_it_si(v1);
        let r2 = intr.intern_struct_it_si(v2);

        // Pointer equality: two equal Val inputs canonicalize to the same arena ref.
        assert!(eq(r1, r2));
    }

    #[test]
    fn intern_kind_payload_si_dispatches() {
        let bump = Bump::new();
        let intr = InstantiatingInterner::new(&bump);

        let val = InternedKindPayloadValI::<'_, '_, sI>::StructIT(StructITValI { id: IdI(PhantomData) });
        let r1 = intr.intern_kind_payload_si(val);
        let r2 = intr.intern_kind_payload_si(val);

        match (r1, r2) {
            (InternedKindPayloadI::StructIT(a), InternedKindPayloadI::StructIT(b)) => {
                assert!(eq(a, b));
            }
            _ => panic!("expected StructIT variant"),
        }
    }

    // Region-mode separation is enforced at the type level — `StructIT` and
    // `StructIT` are distinct types that can't be confused, so the interner's
    // 3 per-mode HashMaps are statically separate. No runtime test needed (and
    // address-level checks are unreliable while StructIT is still a ZST due to
    // bare-placeholder IdI).

    #[test]
    fn intern_name_si_canonicalizes_via_family() {
        use crate::instantiating::ast::names::{
            INameI, INameValI, PackageTopLevelNameI,
        };
        let bump = Bump::new();
        let intr = InstantiatingInterner::new(&bump);

        let v1 = PackageTopLevelNameI::<'_, '_, sI>(PhantomData);
        let v2 = PackageTopLevelNameI::<'_, '_, sI>(PhantomData);

        let r1 = match intr.intern_name_si(INameValI::PackageTopLevel(v1)) {
            INameI::PackageTopLevel(r) => r,
            _ => unreachable!(),
        };
        let r2 = match intr.intern_name_si(INameValI::PackageTopLevel(v2)) {
            INameI::PackageTopLevel(r) => r,
            _ => unreachable!(),
        };

        // Two equal Val inputs canonicalize to the same arena ref via family dispatch.
        assert!(eq(r1, r2));
    }

    #[test]
    fn intern_name_per_concrete_wrapper_works() {
        let bump = Bump::new();
        let intr = InstantiatingInterner::new(&bump);

        let v = PackageTopLevelNameI::<'_, '_, sI>(PhantomData);
        let r1 = intr.intern_package_top_level_name_si(v);
        let r2 = intr.intern_package_top_level_name_si(v);

        // Per-concrete wrapper goes through the family dispatch and unwraps.
        assert!(eq(r1, r2));
    }
}
