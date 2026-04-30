# FrontendRust Engineering TODO

Deferred engineering work that's identified but not currently in flight. New entries go at the bottom; checked items can be removed once committed.

---

## Extend the @SICZ seal to the ~57 simple Name types in the typing pass

The 21 currently sealed Interned types are `IdT`, the 15 transient (slice-bearing) Name types, and the 5 Scala-`IInterning` kind payloads. The ~57 simple Name types — `PrimitiveNameT`, `PackageTopLevelNameT`, `StructTemplateNameT`, `InterfaceTemplateNameT`, all the `*TemplateNameT` variants, var-name types, etc. — are classified Interned per Scala parity (they all live under `INameT extends IInterning`) but lack the `_must_intern: MustIntern` field.

**What to do:** for each of the ~57 simple name types, introduce a `*NameValT` mirror struct (same fields, no `_must_intern`). Change the `impl_intern_name_wrapper_simple!` macro signature to take Val + canonical separately (same way we did for `impl_intern_kind_wrapper!`). Update each match arm in `alloc_name_canonical` (the simple variants under "// 57 simple variants" in `typing_interner.rs`) from `(V::Foo(p), T::Foo(self.bump.alloc(p)))` to `(V::Foo(v), { let c = FooT { ..v.fields, _must_intern: MustIntern(()) }; T::Foo(self.bump.alloc(c)) })`. Update `INameValT` enum's 57 variants to wrap `*NameValT` instead of canonical. Add `_must_intern` field to each canonical.

**Expected scope:** mechanical. ~57 new ValT struct definitions, 57 enum variant changes, 57 match-arm changes, 57 `_must_intern` additions, 1 macro signature change, plus fixing whatever external `FooNameT { ... }` literal construction sites cargo surfaces.

**Why we deferred:** mid-day fatigue. Scope is well-understood; no design questions remain.

**Cross-references:**
- `docs/arcana/SealedInternedConstruction-SICZ.md`
- `docs/architecture/typing-pass-design-v3.md` §6.1 (notes the gap)
- The same refactor done for the 5 kind-payload types is the model

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
