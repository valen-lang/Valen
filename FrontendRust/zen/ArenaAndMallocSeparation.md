# Arena-Allocated Structs Should Not Contain Malloc'd Collections (AASSNCMC)

Arena-allocated structs (those stored as `&'s T` via `bumpalo::Bump::alloc`) should prefer
arena slices (`&'s [T]`) over `Vec<T>` for their collection fields.

This is for speed and consistency, but also because bumpalo does not run destructors.
A `Vec<T>` inside an arena-allocated struct will leak its
heap allocation when the arena is dropped, because `Vec::drop` never runs.

Use `alloc_slice_from_vec()` from `utils/arena_utils.rs` to convert a `Vec<T>` into a
`&'s [T]` before storing it in an arena-allocated struct.
