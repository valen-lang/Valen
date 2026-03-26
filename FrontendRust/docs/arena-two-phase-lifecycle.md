# Arena Two-Phase Lifecycle: Build Mutable, Freeze Immutable

## The Pattern

Every arena-allocated struct follows a two-phase lifecycle:

**Phase 1 — Build (mutable, heap).** Code constructs data using normal Rust collections (`Vec`, `HashMap`, `IndexMap`) on the heap. It pushes, inserts, filters, and transforms freely. This is the "working" phase — the data is still being computed.

**Phase 2 — Freeze (immutable, arena).** Once the data is complete, it's moved into the arena and never mutated again. `Vec<T>` becomes `&'s [T]` via `alloc_slice_from_vec(arena, vec)`. `HashMap<K, V>` becomes `ArenaIndexMap<'s, K, V>` via `ArenaIndexMap::from_iter_in(iter, arena)`. The struct is allocated with `arena.alloc(MyStruct { ... })`, returning a `&'s MyStruct`.

## Example: StructA construction in higher_typing_pass.rs

```
// Phase 1: Build mutable collections
let mut header_rules_builder: Vec<IRulexSR> = Vec::new();
let mut rune_a_to_type: HashMap<IRuneS, ITemplataType> = HashMap::new();

// ... populate them through computation ...
header_rules_builder.push(some_rule);
rune_a_to_type.insert(rune, some_type);

// Phase 2: Freeze into arena
let struct_a = arena.alloc(StructA::new(
    // Vec -> arena slice
    alloc_slice_from_vec(arena, attributes),
    // HashMap -> ArenaIndexMap
    ArenaIndexMap::from_iter_in(rune_a_to_type.into_iter(), arena),
    // Vec -> arena slice
    alloc_slice_from_vec(arena, header_rules_builder),
    ...
));
// struct_a is &'s StructA — immutable from here on
```

## Why immutability matters

Arena-allocated data is **never mutated after construction**. This is enforced by convention, not the type system (fields are `pub`). The guarantees this provides:

- **No resizing.** `ArenaIndexMap`'s internal hash table and entry vector are allocated once at the right size. No rehashing, no dead space from growth.
- **No dangling pointers.** Nothing in the arena points to data that might move. Slices point into the same arena (or the longer-lived `'a` arena).
- **Bulk deallocation.** When the arena drops, everything in it is freed at once. No individual destructors, no use-after-free.

## The exception: working accumulators

Some structs hold `HashMap`/`Vec` fields but are **not** arena-allocated. These are mutable accumulators that live on the stack or heap during computation:

- `Astrouts` — accumulates translation results during the higher typing pass. Stack-allocated, mutated via `&mut`.
- `EnvironmentA` — context struct with `rune_to_type: HashMap`. Created functionally (new instance per scope), never arena-allocated.
- `EnvironmentS` / `FunctionEnvironmentS` — postparser environments. Cloned and boxed, not arena-stored.

These are Phase 1 infrastructure — they *build* the data that eventually gets frozen into arenas.

## Converting between phases

| Phase 1 (mutable) | Phase 2 (frozen in arena) | Conversion |
|---|---|---|
| `Vec<T>` | `&'s [T]` | `alloc_slice_from_vec(arena, vec)` |
| `Vec<&'s T>` | `&'s [&'s T]` | `alloc_slice_from_vec_of_refs(arena, vec)` |
| `HashMap<K, V>` | `ArenaIndexMap<'s, K, V>` | `ArenaIndexMap::from_iter_in(map.into_iter(), arena)` |
| `String` | `StrI<'a>` | `interner.intern(string)` |
| `LocationInDenizenBuilder` | `LocationInDenizen<'x>` | `builder.consume_in(arena)` |
| `T` (single value) | `&'s T` | `arena.alloc(value)` |
