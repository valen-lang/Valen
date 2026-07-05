# Postparse Slice Plan — Onion Typing

**Scope.** Bring `postparsing/` back online (uncomment `pub mod postparsing;` in `lib.rs`) at its long-term onion shape. Land all postparse-side type changes in one atomic set of commits. Everything downstream (`typing/`, `instantiating/`, `simplifying/`, `final_ast/`, `backend_ffi/`, `testvm/`, `integration_tests/`) stays unlinked until its own slice. (`higher_typing/` was retired outright in a follow-up slice — no longer part of the pipeline.)

**Precondition.** Parser + lexer are at their long-term shape as of `b5bde70e6`. `parsing::tests` 397/0/1, `lexing` 3/0/0 green.

**Post-condition.** `postparsing::tests` + upstream slices green. `typing/` and later stay unlinked but re-link is the next slice's forcing function.

---

## Type changes

### 1. `ITemplataType` (13 → 9 variants)

**Delete:**
- `CoordTemplataType` — Kind=Coord
- `OwnershipTemplataType` — axis dies
- `LocationTemplataType` — Location axis retired at parser
- `PrototypeTemplataType` — only user was `PrototypeComponentsSR`

**Survive (canonical: `KindTemplataType`):**
- `RegionTemplataType`, `KindTemplataType`, `ImplTemplataType`, `FunctionTemplataType`, `IntegerTemplataType`, `BooleanTemplataType`, `StringTemplataType`, `PackTemplataType { element_type }` (element type shifts Coord→Kind), `TemplateTemplataType`

**Not present, stays absent:** `MutabilityTemplataType`, `MutabilityLiteralSL`. No `where M Mutability = mut` surface syntax.

### 2. `IRulexSR` (26 → 14 variants)

**Delete outright (16):**
```
CoordSend, DefinitionCoordIsa, CallSiteCoordIsa,
CoordComponents, CoerceToCoord, KindComponents, PrototypeComponents,
Augment,
OneOf, IsInterface, IsConcrete, IsStruct,
RefListCompoundMutability,
IndexList,
MaybeCoercingLookup, MaybeCoercingCall
```

**Rename (1):**
- `Pack → KindList` (element type shifts Coord→Kind, otherwise unchanged)

**Add (4):**
```rust
BorrowRefSR  { range, result_rune, inner_rune, region_rune: Option<RuneUsage> }
HeapOwnRefSR { range, result_rune, inner_rune }
ShareRefSR   { range, result_rune, inner_rune }
WeakRefSR    { range, result_rune, inner_rune }
```

Each fires bidirectionally: inner (+ region for BorrowRef) → wrap → result; result matches variant → peel → inner. No `Option<OwnershipP>` conditional field; the SR variant IS the discriminator.

**Survive unchanged in shape (9):**
- `Equals`, `Literal`, `Lookup` (absorbs `MaybeCoercingLookup`'s rune-type-seeding role), `Call` (absorbs `MaybeCoercingCall`), `RuneParentEnvLookup`, `Resolve`, `CallSiteFunc`, `DefinitionFunc`, `KindList` (renamed from `Pack`)

### 3. `ILiteralSL` (5 → 3 variants)

**Delete:** `LocationLiteral`, `OwnershipLiteral`
**Survive:** `IntLiteral`, `StringLiteral`, `BoolLiteral`

### 4. `IRegionMutabilityS` — delete entire enum

Parser retired `ro`/`rw`/`additive` keywords. Only `ReadWriteRegion` is constructible (5 sites in `post_parser.rs` + `function_scout.rs`, all hard-coded).

- Delete `enum IRegionMutabilityS`
- Delete `RegionGenericParameterTypeS.mutability` field
- Every construction site drops the field

### 5. `CoordGenericParameterTypeS` → `KindGenericParameterTypeS`

Fields `coord_region: Option<RuneUsage>`, `kind_mutable: bool`, `region_mutable: bool` are all dead payload (audit-confirmed: all three are constant at every construction site).

- Rename struct
- Rename `IGenericParameterTypeS::CoordGenericParameterType` → `IGenericParameterTypeS::KindGenericParameterType`
- Delete all three fields
- Emits `KindTemplataType`

### 6. Pattern AST — `AtomSP.coord_rune` → `AtomSP.kind_rune`

~193 call sites rename together (Coord→Kind mass rename for postparse). Not deferred to end-of-arc.

### 7. `IRuneS` — rename Kind-flavored variants

**Rename (Coord→Kind):**
- `ImplDropCoordRune` → `ImplDropKindRune`
- `SelfCoordRune` → `SelfKindRune`
- `MacroVoidCoordRune` → `MacroVoidKindRune`
- `MacroSelfCoordRune` → `MacroSelfKindRune`
- `AnonymousSubstructParentInterfaceCoordRune` → `AnonymousSubstructParentInterfaceKindRune`
- `AnonymousSubstructCoordRune` → `AnonymousSubstructKindRune`
- `AnonymousSubstructVoidCoordRune` → `AnonymousSubstructVoidKindRune`
- `AnonymousSubstructMethodSelfBorrowCoordRune` → `AnonymousSubstructMethodSelfBorrowKindRune`

**Rename `ValS` companions in the SAME edit** — interner-key drift landmine.

### 8. `IRuneS` — delete Ownership-flavored variants

- `ImplicitCoercionOwnershipRune` (+ `ValS`)
- `SelfOwnershipRune` (+ `ValS`)
- `AnonymousSubstructMethodSelfOwnCoordRune` (+ `ValS`) — the "Own" here is ownership; collapses into `AnonymousSubstructMethodSelfBorrowKindRune` at the anon-interface-macro rewrite (deferred to typing slice; postparse leaves the ownership variant deleted and the macro's stub arms will fail loud when typing re-links)

### 9. Error variants (`ICompileErrorS` — 14 total)

**Survive unchanged (12).** Every one is orthogonal to Coord/Ownership.

**Keep name for now (2):**
- `CantOwnershipInterfaceInImpl`
- `CantOwnershipStructInImpl`

Semantics still apply under onion (can't `impl &Vehicle for Cat`). Humanize sites are `panic!("implement:")` stubs — nothing user-visible. Deferred rename to something like `CantRefWrapInterfaceInImpl` / merge into `ImplTargetMustBeBareCitizen` — design call at end of arc.

### 10. `IRuneTypeRuleError` (6 variants) — all survive

Payload structs carry `ITemplataType` — content shifts (no `CoordTemplataType`, no `OwnershipTemplataType`), variant shapes unchanged.

**Also delete internal patterns in `rune_type_solver.rs`:**
- Lines 486-489 — `(KindTemplataType, CoordTemplataType) => {}` "Will convert, so is fine" — collapses (Coord=Kind)
- Lines 474, 504 — Kind/Coord unified-branch acceptance in conflict-detection — becomes single-branch
- Lines 558-572, 607-621 — `matches!` branches lose their `CoordTemplataType` sibling arms

### 11. `IIdentifiabilityRuleError` — empty enum, no change

### 12. Postparse expression IR — unchanged shapes

- `OwnershippedSE { inner_expr, target_ownership: LoadAsP }` — survives; `LoadAsP` already 4-armed after parser slice
- `LocalLoadSE { target_ownership: LoadAsP }` — same

`expression_scout` dispatches the 4 new `IExpressionPE` variants (`Move`/`Borrow`/`Weak`/`Share`) into `OwnershippedSE` with the corresponding `LoadAsP`.

### 13. Citizen definitions — unchanged shapes

`StructS.sharedness: SharednessP` and `InterfaceS.sharedness: SharednessP` both survive as-is. Sharedness lookup goes through temputs later per plan — postparse just propagates.

`NormalStructMemberS { range, name, type_rune }` — no variability field (audit-confirmed; variability already fully retired at parser).

---

## Scout stage behavior changes

### `templex_scout.rs`

Dispatch on the 4 new `ITemplexPT` variants:
- `ITemplexPT::BorrowRef { inner, region }` → `IRulexSR::BorrowRef(BorrowRefSR { .., region_rune })`
- `ITemplexPT::HeapOwnRef { inner }` → `IRulexSR::HeapOwnRef(HeapOwnRefSR { .. })`
- `ITemplexPT::ShareRef { inner }` → `IRulexSR::ShareRef(ShareRefSR { .. })`
- `ITemplexPT::WeakRef { inner }` → `IRulexSR::WeakRef(WeakRefSR { .. })`

Result-rune stamping: every `CoordTemplataType` construction (templex_scout.rs:565) flips to `KindTemplataType`.

### `expression_scout.rs`

Dispatch on the 4 new `IExpressionPE` variants:
- `Move` → `OwnershippedSE { target_ownership: LoadAsP::Move }`
- `Borrow` → `LoadAsP::LoadAsBorrow`
- `Weak` → `LoadAsP::LoadAsWeak`
- `Share` → `LoadAsP::LoadAsShare`

Delete the old `Augment` dispatch (already gone at parser AST level).

### `rule_scout.rs`

Retire where-clause builtin translations:
- `any(...)`, `isInterface(...)`, `isConcrete(...)`, `refListCompoundMutability(...)`, `Prot[...]` — all delete
- `refs(...)` — rewires to emit `KindListSR` (renamed from `PackSR`)

Delete Coord-flavored stamps at rule_scout.rs:172, 174, 220, 364, 366; keep only Kind-flavored:
- Line 364: `ITypePR::CoordType => CoordTemplataType` — delete (`CoordType` already retired at parser; this arm is dead)
- Line 368: `ITypePR::KindType => KindTemplataType` — survives

### `pattern_scout.rs`

Line 67: `ITemplataType::CoordTemplataType(...)` → `ITemplataType::KindTemplataType(...)`. Field name also renames (`coord_rune` → `kind_rune`).

### `function_scout.rs`

~6 stamp sites (lines 381, 434, 534, 565, 883, 948) flip `CoordTemplataType` → `KindTemplataType`.

Also drop the dead `coord_region: None, kind_mutable: true, region_mutable: false` payload at lines 472-474 and 713-715 (CoordGenericParameterTypeS restructure).

---

## Solver-side dispatch tables

Every table indexed by `IRulexSR` variant loses 16 arms + gains 4:

### `rune_type_solver.rs`

- 16 delete arms (per retired variant)
- 4 add arms:
  - `BorrowRefSR`: stamp `result_rune = KindTemplataType`, `inner_rune = KindTemplataType`, `region_rune = RegionTemplataType`
  - `HeapOwnRefSR` / `ShareRefSR` / `WeakRefSR`: stamp `result_rune = KindTemplataType`, `inner_rune = KindTemplataType`
- 1 rename arm (`Pack` → `KindList`; element type stays `KindTemplataType`)
- ~17 in-body `CoordTemplataType` flips to `KindTemplataType` (per audit)
- Unify `MaybeCoercingLookup` and `LookupSR` seeding paths (lines 547-582 vs 583+) into a single Lookup arm

### `identifiability_solver.rs`

Same 16 delete + 4 add. Same-shape identifiability propagation on the 4 new `*RefSR` variants (result + inner + region all mark identifiable when their sources are).

### `post_parser_error_humanizer.rs`

- Delete `humanize_ownership` (with `OwnershipP` already gone at parser)
- Delete 16 humanizer arms for retired SR variants (many already `panic!("implement:")` stubs)
- Add 4 humanizer arms:
  - `BorrowRefSR` → `& inner` (with optional `region ' inner`)
  - `HeapOwnRefSR` → `heap inner`
  - `ShareRefSR` → `@ inner`
  - `WeakRefSR` → `weak inner`

### `postparsing/test/traverse.rs`

- 16 delete arms + 4 add arms + 1 rename (`Pack` → `KindList`)

---

## Sequencing

Two atomic sub-commits:

**Sub-commit 1 — Type-shape shift (bulk of the work).**
- All enum variant deletions/additions/renames land together
- All stamp-site flips (`CoordTemplataType` → `KindTemplataType`) land together
- All rune-name renames land together (with `ValS` companions in the same edit — interner-key drift)
- `CoordGenericParameterTypeS` → `KindGenericParameterTypeS` rename lands together
- `postparse` re-linked in `lib.rs`; downstream stays unlinked
- Expected end state: postparse + upstream compile clean; postparse tests green

**Sub-commit 2 — Scout-arena stub cleanup.**
- Un-gate `#[cfg(any())]` on `scout_arena.rs`, `Keywords::new_for_scout`, `RangeS::internal`/`test_zero`, `CodeLocationS::internal`/`test_zero`, `FileCoordinate::test`, `PackageCoordinate::internal`, `code_hierarchy::test`, `FileCoordinateMap::test`
- Verify tests still green

**Non-goals during this slice:**
- Typing pass (`typing/`) — stays unlinked; its slice comes next
- Anon-interface macro (`typing/macros/`) — stays unlinked with panic stubs; typing slice rewrites it
- Instantiator, hammer, backend, testvm, integration tests — all stay unlinked
- Overload resolver, `convert()`, `is_type_convertible` — typing-layer concerns, out of scope
- End-of-arc renames of `CantOwnership*` errors — deferred

---

## Test discipline

Follow RFIGA per subsystem where feasible; some parts (bulk stamp flips) are mechanical enough that TDD doesn't pay for itself. Slice suggestions:

1. **P1 — `ITemplataType` slim + `ILiteralSL` slim + `IRegionMutabilityS` delete.** R: no new tests; existing rune-type-solver + humanizer tests catch regressions. I: delete variants. G: postparse test suite (once re-linked).
2. **P2 — `IRulexSR` delete + add + rename.** R: probably need new tests for the 4 `*RefSR` variants — asserting `BorrowRefSR` emission for `&T` templex, symmetric for the other three. I: land the enum shift + templex_scout dispatch + rune_type_solver dispatch + humanizer arms. G: new tests green + existing tests green.
3. **P3 — Coord→Kind stamp flips + field/name renames.** R: this is mechanical rename; the tests that already exist for pattern-scout and function-scout catch shape errors. I: mass rename + stamp flip. G: existing tests.
4. **P4 — Where-clause builtin retirements.** R: delete tests for `Prot[...]`, `any(...)`, `isInterface`, `isConcrete`, `refListCompoundMutability`. I: delete rule_scout branches. G: remaining tests green.
5. **P5 — Scout-arena un-gate.** R: none. I: un-gate `#[cfg(any())]` sites. G: full postparse + upstream test suite.

---

## Landmines flagged

- **Interner-key drift** at `IRuneS` variant renames — the `ValS` companion MUST rename in the same edit. Missing this = silent lookup misses.
- **`humanize_ownership` deletion** — with `OwnershipP` gone at parser, this function has no callers already. Verify before deleting to avoid a dangling ref.
- **`MaybeCoercingLookup`→`LookupSR` unification** in rune-type-solver — the two paths differ in seeding behavior (:547-582 vs :583+). Must unify carefully; this is the largest concrete piece of work in the merge.
- **Anon-interface macro** references `IRulexSR::Augment`, `IRulexSR::OneOf`, `IRulexSR::IsStruct`, `IRulexSR::IsInterface`, `IRulexSR::CoordComponents`, `IRulexSR::RefListCompoundMutability`, `IRulexSR::PrototypeComponents` — all in `panic!("implement:")` stubs (`typing/macros/anonymous_interface_macro.rs`). Since `typing/` stays unlinked, these are inert until the typing slice; postparse doesn't need to fix them.
- **`AnonymousSubstructMethodSelfOwnCoordRune`** — deleting this variant means the anon-interface macro that used to produce it now can't. But `typing/macros/` is unlinked; the fallout only fires when typing re-links. Design decision (collapse into Borrow variant vs delete) can be made in the typing slice.

---

## Expected suite delta

Pre-slice (`b5bde70e6`):
- `parsing::tests`: 397/0/1
- `lexing`: 3/0/0
- Everything else: unlinked

Post-slice target:
- `parsing::tests`: unchanged
- `lexing`: unchanged
- `postparsing::tests`: green, some ignored for downstream dependencies. (`higher_typing::tests` was retired with the pass and no longer exists.)
- Everything else: still unlinked

The exact test count depends on how many postparse tests referenced retired features (`any(...)`, `Prot[...]`, etc.). Expect a small net loss from deleted feature-specific tests, offset by new `*RefSR` tests.
