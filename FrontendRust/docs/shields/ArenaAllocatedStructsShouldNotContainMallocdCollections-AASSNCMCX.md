---
description: Arena-allocated structs must use arena slices and ArenaIndexMap, not Vec, HashMap, or String.
model: AgenticSmall
defs: struct
---

# Arena-Allocated Structs Should Not Contain Malloc'd Collections (AASSNCMCX)

Arena-allocated structs (those stored as `&'s T` via `bumpalo::Bump::alloc`) must not contain `Vec`, `HashMap`, or `String` fields.

Bumpalo does not run destructors. A `Vec<T>` inside an arena-allocated struct leaks its heap allocation when the arena drops.

Use `&'s [T]` (via `alloc_slice_from_vec()`) and `ArenaIndexMap<'s, K, V>` (via `ArenaIndexMap::from_iter_in()`) instead.
