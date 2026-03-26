# LocationInDenizen: Cross-Arena Path Addressing

## What it is

`LocationInDenizen<'x>` is a tree address — a path of child indices identifying a specific location within a denizen (function, struct, interface, etc.). For example, `[2, 1, 3]` means "the 2nd child's 1st child's 3rd child."

It's used to generate unique implicit rune names. When the postparser encounters something that needs a rune (like an inferred type), it uses the `LocationInDenizenBuilder` to get a unique path, then wraps it in `ImplicitRuneS { lid: ... }`.

## The `'x` lifetime

`LocationInDenizen` is parameterized on `'x` rather than a specific arena lifetime because it lives in **different arenas** depending on its owner:

| Owner | Arena | `'x` becomes |
|-------|-------|---------------|
| `ImplicitRuneS`, `LetImplicitRuneS`, `MagicParamRuneS`, and other rune structs | Interner (`'a`) | `'a` |
| `PureSE`, `FunctionCallSE` (expression structs) | Scout (`'s`) | `'s` |

This works because `LocationInDenizen<'x>` just holds `path: &'x [i32]` — a slice reference into whichever arena the owner was allocated in. The `'x` unifies with the owner's arena lifetime at each use site.

## Builder pattern

`LocationInDenizenBuilder` is a mutable builder with `path: Vec<i32>`. It tracks a `next_child` counter and a `consumed` flag.

```rust
let mut lidb = LocationInDenizenBuilder::new();

// Create a child builder (appends next_child index to path)
let mut child_lidb = lidb.child();

// Freeze the path into an arena, consuming the builder
let lid: LocationInDenizen<'a> = child_lidb.consume_in(interner.arena());

// Use the lid in a rune
let rune = ImplicitRuneS { lid };
```

The `consume_in(arena)` method allocates the path as `&'x [i32]` in the given arena. For rune creation, pass `interner.arena()`. For expression locations, pass `self.scout_arena`.

## Why not just Vec?

Before this change, `LocationInDenizen` held `path: Vec<i32>`. This meant every arena-allocated struct containing a `LocationInDenizen` had a heap pointer — defeating the purpose of arena allocation. With `&'x [i32]`, the path data lives in the same arena as its owner.
