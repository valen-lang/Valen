# Per-Pass Arenas Migration

## Status: Complete

The long-lived `'a` interner arena has been eliminated. Each pass now owns its own arena with its own interning maps. Data is re-interned (copied) at pass boundaries.

- `Interner<'a>` fully deleted
- Replaced by `ParseArena<'p>` and `ScoutArena<'s>`
- All parser types collapsed from `<'a, 'p>` to `<'p>`
- All postparser/higher typing types collapsed from `<'a, 's>` to `<'s>`
- `CodeLocationS`/`RangeS` now fully `Copy` (Arc eliminated)
- Keywords created fresh per pass
- All tests pass

## Remaining Future Work

- **Typing pass arena (`TypingArena<'t>`)**: Not yet implemented. The typing pass still uses the scout arena. Design notes in `src/higher_typing/docs/architecture/typing-pass.md` (when created).
- **Arena-deterministic maps**: HashMap fields in arena structs still use heap allocation. See `docs/reasoning/arena-deterministic-maps.md`.
