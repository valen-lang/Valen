// Arena-backed slice allocation helpers for AST construction.
//
// INVARIANTS (see plan: Arena-Backed AST Slices):
// - 'a: interner arena only (canonical/interned data)
// - 'p: parsed AST arena (FileP graph)
// - 's: postparsed AST arena (ProgramS graph)
//
// These helpers allocate in a Bump; do NOT route parsed/postparsed AST
// collections through Interner or the interner arena.

use bumpalo::Bump;

/// Allocate a copy of a slice in the arena.
/// For `Copy` payloads (e.g. RangeL, NameP).
#[inline(always)]
pub fn alloc_slice_copy<'p, T: Copy>(arena: &'p Bump, slice: &[T]) -> &'p [T] {
  arena.alloc_slice_copy(slice)
}

/// Allocate a slice from an iterator into the arena.
/// For non-Copy node vectors (e.g. Vec<IExpressionPE<'a>>).
/// Iterator must be ExactSizeIterator (e.g. Vec::into_iter()).
#[inline(always)]
pub fn alloc_slice_fill_iter<'p, T, I>(arena: &'p Bump, iter: I) -> &'p [T]
where
  I: IntoIterator<Item = T>,
  I::IntoIter: ExactSizeIterator,
{
  arena.alloc_slice_fill_iter(iter)
}

/// Convenience: allocate a slice from a Vec (moves elements).
#[inline(always)]
pub fn alloc_slice_from_vec<'p, T>(arena: &'p Bump, vec: Vec<T>) -> &'p [T] {
  arena.alloc_slice_fill_iter(vec.into_iter())
}
