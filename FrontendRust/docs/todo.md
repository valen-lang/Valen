# FrontendRust Engineering TODO

Deferred engineering work that's identified but not currently in flight. New entries go at the bottom; checked items can be removed once committed.

---

## Apply the seal pattern (@SICZ) to postparsing

The typing pass interner now seals every TFITCX-Interned type via the `MustIntern` private-constructor token (see `docs/arcana/SealedInternedConstruction-SICZ.md`). Postparsing already uses the dual-enum pattern (`IDEPFL` — separate `*Val` lookup type alongside the canonical `&'s T`), so the discipline is partially in place — but its **canonical** payload structs (`CodeRuneS`, `ImplicitRuneS`, `LambdaImpreciseNameS`, etc.) are not sealed. Anyone outside `ScoutArena` can construct them directly, bypassing interning.

**What to do:** define a postparsing-side `MustIntern` token in `scout_arena.rs` (its own version, not shared with the typing pass — different arena, different module). Add `pub _must_intern: MustIntern` to every Interned permanent payload struct in `postparsing/names.rs`, `postparsing/rules/`, and the rune/imprecise-name hierarchies. Fill the field at the canonical-construction sites inside `ScoutArena::intern_*` methods.

**Expected scope:** ~75-90 permanent payload structs, all already constructed exclusively inside `scout_arena.rs`. Cargo will surface any external construction sites — those become the bug list to fix.

**Why we deferred:** typing pass is the active migration surface and where the original `signature-id-mismatch` bug lived. Postparsing is more stable; sealing it is hygiene, not a hot-path fix. Doing it as its own focused pass makes the diff cleaner and easier to review.

**Cross-references:**
- `docs/arcana/SealedInternedConstruction-SICZ.md` — pattern explanation
- `.claude/rules/postparser/IDEPFL-postparser-interning.md` — existing dual-enum pattern in postparsing
- `docs/shields/TypesFitIntoTheseCategories-TFITCX.md` — Interned-category requirement
