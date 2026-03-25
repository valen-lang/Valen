# HigherTypingPass has `scout_arena` field (not in Scala)

Scala's `HigherTypingPass` has no arena because the JVM's GC manages `StructA`/`InterfaceA` lifetimes. In Rust, `HigherTypingPass` holds `scout_arena: &'s Bump` so it can arena-allocate these types, allowing the `Astrouts` cache maps to store `&'s` references that can be both cached and returned without cloning. This also applies to `HigherTypingCompilation`, which stores and forwards the same arena.
