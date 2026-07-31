<!-- session: 02461b86-ec82-451c-8de3-439f8e8c62e1; exported-bytes: 5731302; transcript: /Users/verdagon/.claude/projects/-Volumes-V-Vale2/02461b86-ec82-451c-8de3-439f8e8c62e1.jsonl -->
# Claude Conversation Log

Session ID: 02461b86-ec82-451c-8de3-439f8e8c62e1
Date: 2026-07-04 20:55:47

---

## 👤 User

please read vcoord-handoff.md in full and then tmp/claude-conversation-2026-07-03-aa25253d.md in full

---

## 🤖 Claude

Continuing to read the conversation log.

---

## 🤖 Claude

Both files read in full. Here's what I've absorbed:

**`vcoord-handoff.md` (524 lines)** — the current handoff at commit `b5bde70e6`. Covers:
- The onion typing mission: `Kind` gains 4 ref variants (`BorrowRef`, `HeapOwnRef`, `ShareRef`, `WeakRef`); `Coord` dissolves; region migrates onto `BorrowRefT` only.
- Six resolved design decisions (Q1–Q6): weak spelling, share intrinsic to citizen, bare-use pass-through for BorrowRef sources, Coord's fate, coercion probe mechanism, all IR stages get the onion.
- Full coercion table (probe rows 1/2/3/a/c/b, structural rows, error cases including `(d)` double-borrow).
- Parser-slice progress at `b5bde70e6`: parser AST at long-term shape; all later passes (`postparsing` → `typing` → `hammer` → `backend` → `testvm` → `integration_tests`) commented out or `#[cfg(any())]`-gated so downstream migration is forced by breakages.
- Next slice: postparsing. Big work items — retire `AugmentSR`, kill the `Coord*SR` family, restructure anon-interface macro, rename `PackSR`→`KindListSR`, etc.

**`tmp/claude-conversation-2026-07-03-aa25253d.md` (6138 lines)** — the session that produced the parser slice + updated handoff:
- Design pass working through Q1–Q6 to lock in the four ref variants, `Coord=Kind`, always-wrap bare-use (later softened to pass-through-for-`BorrowRef` sources).
- 11-agent scouting mission producing `onion-typing-scouting.md` + big-bang `onion-typing-plan.md`.
- Extended solver-simplification discussion: sweep of 14 SR rules — 12 delete outright, 2 merge (`MaybeCoercing*`→plain), 1 rename (`PackSR`→`KindListSR`), 4 new (`BorrowRefSR`/`HeapOwnRefSR`/`ShareRefSR`/`WeakRefSR`); adopting rustc-style strip-solve-check with enumerated coercion sites; killing overload resolution.
- RFIGA-planned parser slice T1–T7 + E1–E5 + C1–C4, executed slice-by-slice.
- Post-execution cleanup: legacy AST fully retired; `LocationP`/`inl`/`location` axis retired; 7 additional keywords (`additive`, `vary`, `fiinal`, `pre`, `ro`, `rw`, `linear`) + their attribute variants deleted; `#[cfg(any())]` gates chosen over deletion for `scout_arena.rs` and scout-facing helpers.
- Committed as `b5bde70e6`; `vcoord-handoff.md` updated to point at the parser slice's landing state and the three companion docs.

Ready for the next slice.

---

## 👤 User

next is updating the postparser. walk me through how all the types are going to change?

---

## 🤖 Claude

Here's the type-shape walk through the postparser, organized by data-structure family. This is what changes at the enum/struct level — logic changes (dispatch tables, scout emission bodies) fall out of these.

## 1. Solver rules (`IRulexSR`) — the biggest surface

**Delete outright (14 variants + their structs):**

```
CoordSendSR                — kill Send rule; unification moves inline post-strip
CallSiteCoordIsaSR         — isa moves to post-solve coercion pass
DefinitionCoordIsaSR       — impl-decl isa becomes direct edge_compiler check
CoordComponentsSR          — anon-interface macro synthesizes directly
CoerceToCoordSR            — no-op under Kind=Coord
KindComponentsSR           — already dead
AugmentSR                  — replaced by 4 new SRs (see below)
OneOfSR                    — retires `any(...)`; nothing real uses it
IsInterfaceSR              — solver body already panics
IsConcreteSR               — zero construction sites
RefListCompoundMutabilitySR — zero construction sites
PrototypeComponentsSR      — duplicate of CallSiteFuncSR's decomp
MaybeCoercingLookupSR      — byte-identical to LookupSR; merge
MaybeCoercingCallSR        — same, merge to CallSR
```

**Rename:**

```
PackSR → KindListSR   — element type shifts Coord→Kind, otherwise unchanged
```

**Add (four new SRs, one per KindT ref variant, structurally symmetric):**

```rust
BorrowRefSR  { range, result_rune, inner_rune, region_rune: Option<RuneUsage> }
HeapOwnRefSR { range, result_rune, inner_rune }
ShareRefSR   { range, result_rune, inner_rune }
WeakRefSR    { range, result_rune, inner_rune }
```

Bidirectional: know inner (+region for BorrowRef) → conclude result; know result and it matches → conclude inner.

**Survive unchanged in shape** (bodies simplify, or absorb dead siblings):

```
LookupSR                — absorbs MaybeCoercingLookup's rune-type-seeding role
CallSR                  — absorbs MaybeCoercingCall
ResolveSR               — bound resolution
RuneParentEnvLookupSR   — MKRFA env-chain closure, orthogonal
DefinitionFuncSR        — function-signature construction
CallSiteFuncSR          — call-site function-signature resolution
LiteralSR family        — minus OwnershipLiteralSL
```

Net: ~20 SR variants → ~12. Solver body plausibly 300-500 LOC.

## 2. Literals (`ILiteralSL`)

**Delete:**
```
OwnershipLiteralSL   — dies with OwnershipP
```

**Survive:**
```
IntLiteralSL, BoolLiteralSL, StringLiteralSL, MutabilityLiteralSL
```
(`LocationLiteralSL` also dies with the Location axis retirement we already did in the parser slice.)

## 3. Rune types (`ITemplataType`)

**Delete:**
```
CoordTemplataType     — Coord=Kind
OwnershipTemplataType — Ownership axis dies
LocationTemplataType  — retired with Location axis
PrototypeTemplataType — if PrototypeComponentsSR-only; verify
```

**Survive:**
```
KindTemplataType       — canonical
MutabilityTemplataType
IntTemplataType, BoolTemplataType, StringTemplataType
RegionTemplataType
TemplateTemplataType
PackTemplataType { element_type }  — element_type shifts Coord→Kind
```

## 4. Pattern AST (`AtomSP` etc.)

**Modify semantically, keep field name:**
```
AtomSP.coord_rune: Option<RuneUsage>
```
Semantically now a Kind rune. Per your earlier decision, don't rename yet — the mass Coord→Ty rename happens at the very end of the arc. Field stays named `coord_rune`, ~193 call sites unchanged.

## 5. Postparse expression IR (`IExpressionSE` and friends)

The scout output IR. Currently has:
- Whatever the postparse LetSE / MutateSE / ReturnSE / etc. carry — none of these carry `CoordT`, they're pre-typing.
- Uses `LoadAsP` (with `LoadAsShare` added during parser slice for the `@x` value form).

Only real change: `LoadAsP` is complete with 4 arms (Move / LoadAsBorrow / LoadAsWeak / LoadAsShare), and `expression_scout` dispatches by `IExpressionPE::{Move, Borrow, Weak, Share}` directly instead of the old `Augment { ownership }` pattern.

## 6. Scout-level names (`postparsing/names.rs`)

**Audit for:** any `INameS` variant carrying ownership tags — drop those fields. Most names are unaffected (they carry template args as `ITemplataT`, which changes independently). Ones to check:

- `RawArrayNameS` — carries mutability, not ownership; probably unaffected
- Function/struct/interface names — carry template-arg lists; the element type of the list shifts Coord→Kind but the variant shape stays

## 7. Error variants (`IPostParserError`)

**Delete errors tied to retired concepts:**
```
Any error naming: OneOfFailed, KindIsNotConcrete, ReceivingDifferentOwnerships,
                  RefListCompoundMutability, IsInterface, IsConcrete,
                  Prot decomposition, any(...) shape errors
Any error mentioning humanize_ownership output
```

Preserve the three-way `implicit_clone` error taxonomy for later reuse — `ImplicitCloneRejectedT` and `NoImplicitCloneDefinedT` are typing-pass errors, not postparse. Postparse loses only its own now-unreachable variants.

## 8. Templata-related types (`ITemplataT`, `CoordTemplataT`, etc.)

Postparsing itself doesn't define `ITemplataT` (that lives in typing/), but it references `ITemplataType` heavily via rune-type dispatch. Key postparse-visible changes cascading in from typing:

- `CoordTemplataT` — deleted (as data). Postparse doesn't construct it, just references the type name in comments/matches.
- `OwnershipTemplataT` — deleted.

## 9. `MutabilityP` at rule literal level

**Survives.** Rule-level `where M Mutability = mut` still works.

## 10. Rune-type-solver internal tables

Not strictly type changes, but the type of every solver-dispatch table shifts:
- `initially_known_runes` seeding table — 14 arms delete + 4 added + MaybeCoercing/Lookup unification
- `get_puzzles` / `solve_rule` dispatch — same shape
- `identifiability_solver` — same

## Rough net delta

| Category | Adds | Deletes | Net |
|---|---:|---:|---:|
| `IRulexSR` variants | +4 | −14 | −10 |
| `ILiteralSL` variants | 0 | −2 | −2 |
| `ITemplataType` variants | 0 | −2 to −4 | −2 to −4 |
| Error variants | 0 | ~−8 to −10 | ~−9 |
| Struct fields | 0 | trivia | 0 |

Total postparse enum-arm count drops by ~20-25. Every remaining variant carries only fields its semantics needs — no more `Option<OwnershipP>` conditional fields, no more MaybeCoercing tag doing coordinated-with-higher-typer signaling.

Want me to go one level deeper on any of these — e.g., sketch the exact field lists on the 4 new SR variants, or trace the pattern-scout data flow that keeps `coord_rune` named that way while semantically shifting?

---

## 👤 User

MutabilityTemplataType should go away, so should everything to do with variability. where M Mutability = mut shouldnt be a thing anymore.

and you can do the audit now please

---

## 🤖 Claude

Let me audit. Starting by mapping the postparsing structure and then honing in on mutability/variability and the ownership-carrying names.

---

## 🤖 Claude

Audit complete. Here's the corrected picture — a couple of things I said earlier turned out different once I looked at the source.

## Confirmed corrections to my earlier walkthrough

**`MutabilityTemplataType`** — **not present** in `ITemplataType`. Only appears in commented-out Scala reference code. Nothing to delete; just confirm it stays absent. Your directive holds by default.

**`MutabilityLiteralSL`** — **not present** in `ILiteralSL`. Grep-clean. Same deal.

**`where M Mutability = mut` surface syntax** — likewise dies structurally, since there's no `MutabilityTemplataType` to type the rune with in the first place. The parser's rune-type dispatch for `Mutability` needs deleting if it exists there (audit-adjacent — parser layer already handled).

**Variability at postparse** — **already zero references** anywhere in `postparsing/` or `parsing/`. Fully retired by the parser slice. `NormalStructMemberS { range, name, type_rune }` — no variability field. Locals (`LocalS`) — no variability field.

## Full audit findings by file

### `postparsing/itemplatatype.rs` — `ITemplataType` (13 variants)

**Delete (4):**
- `CoordTemplataType` — Kind=Coord
- `OwnershipTemplataType` — axis dies
- `LocationTemplataType` — Location axis retired
- `PrototypeTemplataType` — only user was `PrototypeComponentsSR`; verify no stragglers, then delete

**Survive (9):**
- `RegionTemplataType`, `KindTemplataType`, `ImplTemplataType`, `FunctionTemplataType`, `IntegerTemplataType`, `BooleanTemplataType`, `StringTemplataType`, `PackTemplataType { element_type }` (element type shifts semantically Coord→Kind), `TemplateTemplataType`

### `postparsing/rules/rules.rs` — `IRulexSR` (26 variants)

**Delete outright (16):**
```
CoordSend, DefinitionCoordIsa, CallSiteCoordIsa,
CoordComponents, CoerceToCoord, KindComponents, PrototypeComponents,
Augment,
OneOf, IsInterface, IsConcrete, IsStruct,      ← IsStruct is also dead (zero emissions, panic stub in anon-interface macro)
RefListCompoundMutability,
IndexList,                                     ← audit confirmed: zero real emissions
MaybeCoercingLookup, MaybeCoercingCall
```

**Rename (1):**
```
Pack → KindList   (element type Coord→Kind)
```

**Add (4):**
```
BorrowRef  { range, result_rune, inner_rune, region_rune: Option<RuneUsage> }
HeapOwnRef { range, result_rune, inner_rune }
ShareRef   { range, result_rune, inner_rune }
WeakRef    { range, result_rune, inner_rune }
```

**Survive unchanged in shape (9):**
```
Equals, Literal, Lookup, Call, RuneParentEnvLookup,
Resolve, CallSiteFunc, DefinitionFunc,
Pack (renamed)
```

Net: 26 → 14 variants.

### `postparsing/rules/rules.rs` — `ILiteralSL` (5 variants)

**Delete:** `LocationLiteral`, `OwnershipLiteral`
**Survive:** `IntLiteral`, `StringLiteral`, `BoolLiteral`

Net: 5 → 3.

### `postparsing/ast.rs` — `IRegionMutabilityS` (4 variants → 1)

Parser retired `ro`/`rw`/`additive` keywords. Only `ReadWriteRegion` is still constructed (5 sites in `post_parser.rs` + `function_scout.rs`; all hard-coded).

**Collapse:** delete the enum entirely; `RegionGenericParameterTypeS` loses its `mutability: IRegionMutabilityS` field. Every site that was `mutability: IRegionMutabilityS::ReadWriteRegion` becomes trivial.

### `postparsing/ast.rs` — `CoordGenericParameterTypeS`

Currently `{ coord_region: Option<RuneUsage>, kind_mutable: bool, region_mutable: bool }`. Audit shows:
- `coord_region` is always `None` (asserted at line 490)
- `kind_mutable: true`, `region_mutable: false` — always constant at both construction sites

**All three fields dead payload.** Under onion:
- Delete the field-carrying shape entirely
- Rename `CoordGenericParameterTypeS` → `KindGenericParameterTypeS` (semantic rename deferred to end-of-arc mass rename)
- Emits `KindTemplataType` (was `CoordTemplataType`)

Same treatment: `IGenericParameterTypeS::CoordGenericParameterType` variant name stays until end-of-arc rename.

### `postparsing/expressions.rs` — scout expression IR

**No CoordT fields anywhere** at this layer (types haven't been resolved yet — everything's runes and templata-types).

**`OwnershippedSE { inner_expr, target_ownership: LoadAsP }`** — survives. `LoadAsP` was already extended to 4 arms during parser slice (Move / LoadAsBorrow / LoadAsWeak / LoadAsShare). `expression_scout` dispatches the four new `IExpressionPE` variants (`Move`/`Borrow`/`Weak`/`Share`) into `OwnershippedSE` with the corresponding `LoadAsP`.

**`LocalLoadSE { target_ownership: LoadAsP }`** — same shape, survives.

### `postparsing/names.rs` — `IRuneS` (25+ variants)

**Delete (Ownership-flavored, axis dies):**
- `ImplicitCoercionOwnershipRune`
- `SelfOwnershipRune`
- `AnonymousSubstructMethodSelfOwnCoordRune` — the "Own" is ownership; collapses with `AnonymousSubstructMethodSelfBorrowCoordRune`, or one of the two dies (typing-macro rewrite decides)

**Keep with rename deferred** (all `*CoordRune*` names — Kind semantics under onion, field-name rename comes at end-of-arc mass rename per your earlier decision):
- `ImplDropCoordRune`, `SelfCoordRune`, `MacroVoidCoordRune`, `MacroSelfCoordRune`, `AnonymousSubstructParentInterfaceCoordRune`, `AnonymousSubstructCoordRune`, `AnonymousSubstructVoidCoordRune`, `AnonymousSubstructMethodSelfBorrowCoordRune`

**Also delete the `ValS` companions** for the removed variants (interner-key drift landmine — must delete in the same commit).

### `postparsing/patterns/patterns.rs` — `AtomSP`

`AtomSP.coord_rune: Option<RuneUsage>` — field name stays per your earlier decision; semantically now Kind. ~193 call sites unchanged.

### `postparsing/ast.rs` — `NormalStructMemberS` / `VariadicStructMemberS`

Both are `{ range, name, type_rune }` (no variability). Clean.

### `postparsing/ast.rs` — `StructS` / `InterfaceS`

Both carry `sharedness: SharednessP` (imported from parser AST). Per your plan: `SharednessP` at parser stays, `SharednessT` at typing stays as descriptive enum. Postparse propagates it through. No change here.

### Error variants (`IPostParserError` — need to open the file to enumerate)

Delete errors tied to retired concepts. Expect (based on the retired rules + keywords):
- Errors naming `any(...)`, `isInterface(...)`, `isConcrete(...)`, `refListCompoundMutability(...)`, `Prot[...]` sugar
- Errors naming `ownership_literal` in where clauses
- Errors naming `IRegionMutabilityS` variants that are gone

## Rough net enum-arm delta at postparse

| Enum | Before | After | Delta |
|---|---:|---:|---:|
| `IRulexSR` | 26 | 14 | −12 |
| `ITemplataType` | 13 | 9 | −4 |
| `ILiteralSL` | 5 | 3 | −2 |
| `IRegionMutabilityS` | 4 | (deleted) | −4 |
| `IRuneS` / `INameValS` (ownership-flavored) | ~25 | ~22 | −3 |
| `CoordGenericParameterTypeS` fields | 3 | 0 | −3 |

Total postparse enum-variant surface shrinks by ~28 arms, with roughly 4 new arms (the four ref-variant SRs). Every remaining variant carries only fields its semantics needs.

Want me to check `IPostParserError` next, or move to the field-flow question (how `coord_rune` semantically becomes Kind without renaming)?

---

## 👤 User

investigate both please

---

## 🤖 Claude

Now investigating the `coord_rune` flow — how the field goes from Coord-semantic to Kind-semantic without renaming.

---

## 🤖 Claude

Both threads investigated. Here's what I found.

## Thread 1 — `ICompileErrorS` audit (postparse error enum)

14 variants total. Onion impact:

**Survive unchanged (12):**
```
CouldntFindVarToMutateS, CouldntFindRuneS, StatementAfterReturnS,
VariableNameAlreadyExists, InterfaceMethodNeedsSelf, VirtualAndAbstractGoTogether,
RuneExplicitTypeConflictS, InitializingRuntimeSizedArrayRequires..., 
InitializingStaticSizedArrayRequires..., ExternHasBodyS,
IdentifyingRunesIncompleteS, RangedInternalErrorS
```
Every one is orthogonal to Coord/Ownership — variable resolution, control flow, generic-arg validation, etc.

**Rename in spirit, keep name for now (2):**
```
CantOwnershipInterfaceInImpl
CantOwnershipStructInImpl
```
Both are emitted when a user writes `impl &Vehicle for Cat` (an impl with a ref-wrapped kind on either side). Under onion the constraint still holds — you can't `impl` on a `BorrowRef(Interface)` any more than on a `Borrow interface`. The error name is stale but the semantics preserve. Both `humanize_*` sites are `panic!("implement:")` stubs, so nothing user-visible changes.

Suggested rename at end-of-arc mass rename: `CantRefWrapInterfaceInImpl` / `CantRefWrapStructInImpl`. Or (since the check is essentially "impl target must be a bare citizen kind") merge into a single `ImplTargetMustBeBareCitizen`. Design call.

## Thread 1b — `IRuneTypeRuleError` (rune-type solver)

6 variants:
```
FoundCitizenDidntMatchExpectedType     — survives (still a real check)
FoundTemplataDidntMatchExpectedType    — survives
NotEnoughArgumentsForGenericCall       — survives
GenericCallArgTypeMismatch             — survives
TooManyMatchingTypes                   — survives
CouldntFindType                        — survives
```
All 6 survive. Their payload structs carry `expected_type: ITemplataType` / `actual_type: ITemplataType` — those fields' *content* shifts (no more `CoordTemplataType`, no more `OwnershipTemplataType`), but the error variants and their field shapes are unchanged.

**`IIdentifiabilityRuleError`** — empty enum (`{}`). Nothing to touch.

**Rune-type solver internal signature acceptance** — a few sites die:
- `rune_type_solver.rs:474, 486-489, 504` — patterns like `(KindTemplataType, CoordTemplataType) => {} // Will convert, so is fine` collapse to single-branch under `Kind=Coord`. Kind-vs-Coord conflict detection becomes a no-op.
- `rune_type_solver.rs:558-572, 607-621` — `if matches!(t.return_type, ITemplataType::KindTemplataType(_))` branches lose their `CoordTemplataType` sibling arms.

## Thread 2 — the `coord_rune` semantic flip without renaming

The field name stays `coord_rune`. What actually changes is the *type* the rune gets stamped with in `rune_to_explicit_type`. Concrete flow:

**The single "stamp" per rune** happens where a scout stage inserts a `(rune, ITemplataType)` pair into `rune_to_explicit_type`. Every such stamp site that today writes `ITemplataType::CoordTemplataType(...)` flips to `ITemplataType::KindTemplataType(...)`.

**~30 stamp sites in postparse:**

| File | Sites | Context |
|---|---|---|
| `pattern_scout.rs:67` | 1 | `AtomSP.coord_rune` — pattern's type-rune |
| `templex_scout.rs:565` | 1 | Every templex result rune |
| `rule_scout.rs:172, 174, 220, 364, 366` | 5 | Surface `Coord`/`Ref` keyword; `refs(...)` element type |
| `function_scout.rs:381, 434, 534, 565, 883, 948` | 6 | Function param/return type runes |
| `rune_type_solver.rs` various | ~17 | Per-SR-variant conclusions (`CoordSend`, `CallSiteFunc`, `DefinitionFunc`, `Resolve`, `CoordIsa`, `CoordComponents`, `PackSR`, `AugmentSR`) |

The mechanical shift: `ITemplataType::CoordTemplataType(CoordTemplataType {})` → `ITemplataType::KindTemplataType(KindTemplataType {})`. Same at every site.

**What that does semantically.** The rune's `ITemplataType` is what the higher-typer's rune-type solver dispatches on to decide which lookup to use, which literals type-check against it, etc. When the stamp changes from `Coord` to `Kind`:
- Lookups that were "resolve to a Coord-shaped templata" now resolve to "Kind-shaped templata" — same templata under `Kind=Coord`
- The rune-type solver's per-SR-variant conclusions all use `Kind` instead of `Coord`
- Rules that used to co-emit a `CoerceToCoordSR` (from higher_typing's `explicify_lookups`) never emit it — the rule dies, so does the coercion

**Field name `coord_rune` = a mislabel** during the arc. Someone reading `atom.coord_rune` post-onion sees a name that says "coord" and a value that's semantically Kind. The lie is contained — the compiler doesn't care about the string "coord_rune", only about the `ITemplataType` stamp. Per your earlier decision, the mass rename to `type_rune` (or the `Coord→Ty` sweep) happens at the very end of the arc so we don't churn 193 call sites twice.

**Where the mismatch shows up (worth flagging):**

1. **`AtomSP.coord_rune: Option<RuneUsage>`** — field name reads Coord, type is Kind. Comment at declaration or a `// VCOORD: renamed to type_rune at end of arc` marker helps future readers.
2. **`IRuneS::*CoordRune*` variants** (~8 rune-name variants) — same lie. Field names stay, semantics shift.
3. **`translate_maybe_type_into_rune` return value** — the name of the returned `RuneUsage` is passed as `coord_rune.rune`. Same lie.

The plumbing works because Rust's type system doesn't care what the field is called — only that the stamped `ITemplataType` is the right variant. The scout-to-solver contract is "here's a rune, here's its type" — the type is what carries the semantic; the field name is documentation only.

**Cost of the field-name lie:** everyone reading postparse during the arc sees a misleading name. Mitigated by (a) the marker convention and (b) the mass rename at the end.

**Benefit:** ~193 call-site edits happen once, not twice. And every one lands in a single commit where the semantics also flip cleanly, rather than spread across the arc.

Trade-off is worth it as long as we don't forget the end-of-arc rename. Suggest adding it to the vcoord-handoff.md "at end of arc" list explicitly so it doesn't get orphaned.

Want me to sketch the concrete rune-name variant renames next (which `*CoordRune*` variants merge with which under onion, and which `*OwnershipRune*` variants die), or shift back to the plan for the postparse slice execution?

---

## 👤 User

nevermind, we can rename it

---

## 🤖 Claude

Understood. Then the field-name rename lands with the postparse slice, not at end-of-arc:

**Rename:** `AtomSP.coord_rune` → `AtomSP.kind_rune` (~193 call sites).

**Cascade renames in the same commit:**
- `translate_maybe_type_into_rune` — returns a `RuneUsage` that gets assigned to `kind_rune` fields; the function itself probably renames too (`translate_maybe_type_into_kind_rune` or just `translate_maybe_type_into_rune` if the name is generic enough).
- `IRuneS::*CoordRune` variants — the Kind-flavored ones:
  - `ImplDropCoordRune` → `ImplDropKindRune`
  - `SelfCoordRune` → `SelfKindRune`
  - `MacroVoidCoordRune` → `MacroVoidKindRune`
  - `MacroSelfCoordRune` → `MacroSelfKindRune`
  - `AnonymousSubstructParentInterfaceCoordRune` → `AnonymousSubstructParentInterfaceKindRune`
  - `AnonymousSubstructCoordRune` → `AnonymousSubstructKindRune`
  - `AnonymousSubstructVoidCoordRune` → `AnonymousSubstructVoidKindRune`
  - `AnonymousSubstructMethodSelfBorrowCoordRune` → `AnonymousSubstructMethodSelfBorrowKindRune`
- `IRuneS::*OwnershipRune` variants — die outright (Ownership axis gone):
  - `ImplicitCoercionOwnershipRune`
  - `SelfOwnershipRune`
  - `AnonymousSubstructMethodSelfOwnCoordRune` — the "Own" here is ownership; likely collapses into the surviving `SelfBorrowKindRune` (or one of the two dies, depending on how the anon-interface macro rewrites)
- Their `ValS` companions (interner-key drift landmine — rename with the parent in the same commit).

**Other Coord→Kind rename opportunities visible from the audit:**
- `ImplicitCoercionKindRune*` — already Kind-named. Keep as is.
- `CoordGenericParameterTypeS` → `KindGenericParameterTypeS` (with the 3 dead fields dropped).
- `IGenericParameterTypeS::CoordGenericParameterType` → `IGenericParameterTypeS::KindGenericParameterType`.
- `CoordTemplataType` in `ITemplataType` — deleted (per the earlier plan), so no rename needed.

**Not renamed** (per the "don't-do-Ty-yet" position):
- The struct `CoordT` in typing/ is deleted, not renamed.
- The struct `KindT` stays `KindT` — the survivor.
- We're not doing the mass `Coord→Ty` sweep; we're doing `Coord→Kind` locally where the field actually means "kind of a thing."

Want to update the postparse plan to bundle this rename with the type shifts, or hold it as a follow-up sub-commit?

---

## 👤 User

sure update

---

## 🤖 Claude

I'll write a compact postparse-slice plan at the project root, mirroring the pattern of the earlier plans. Captures every design decision from this session.

---

## 🤖 Claude

Written to `/Volumes/V/Vale2/postparse-slice-plan.md` — 200-odd lines. Structure:

1. **Type changes (13 sections)** — every enum/struct in postparse that shifts, with concrete variant lists
2. **Scout stage behavior changes** — templex/expression/rule/pattern/function scout updates
3. **Solver-side dispatch tables** — rune-type-solver, identifiability-solver, humanizer, traverse-test
4. **Sequencing** — 2 atomic sub-commits (type-shape shift, then scout-arena un-gate)
5. **Test discipline** — 5 RFIGA-style sub-slices (P1–P5)
6. **Landmines flagged** — interner-key drift, MaybeCoercing/Lookup unification, anon-interface macro stubs
7. **Expected suite delta**

Key decisions baked in from this session:
- `coord_rune` → `kind_rune` mass rename lands in this slice, not end-of-arc
- All 8 `*CoordRune` variants rename to `*KindRune` (with `ValS` companions in same edit)
- 3 `*OwnershipRune` variants delete outright
- `MutabilityTemplataType` / `MutabilityLiteralSL` confirmed absent — no `where M Mutability = mut` surface
- `CoordGenericParameterTypeS` → `KindGenericParameterTypeS` with the 3 dead fields dropped
- `IRegionMutabilityS` enum deleted (only `ReadWriteRegion` constructible after parser slice)
- `CantOwnership*` errors keep names for now (rename deferred)

Ready to start P1 whenever you say, or want me to walk through anything in the plan first?

---

## 👤 User

tell me more about the rune type solver please

---

## 🤖 Claude

Let me read the file to give you a substantive walkthrough.

---

## 🤖 Claude

The rune-type solver is a two-phase constraint solver that runs **before** the value solver. Its job is to classify every rune by templata type — Kind, Prototype, Region, Integer, etc. — so the value solver can dispatch on the right lookup path.

Here's what it does, how it's wired, and what changes under onion.

## What it computes

Every rule in `IRulexSR` participates in a small type inference. Given rules like:

```
where T = Ship, prot = someFunction, N Int = 5
```

The rune-type solver produces:
```
T   → KindTemplataType
prot → PrototypeTemplataType
N   → IntegerTemplataType
```

That mapping is stored on the citizen/function's `rune_to_explicit_type` and `predicted_rune_to_type` fields, and it's what the *value* solver (`compiler_solver.rs`) reads later to decide "when I look up `T`, do I want a Coord-shaped result or a Kind-shaped one?"

## The two phases

### Phase 1 — Initial known-rune seeding (`solve_rune_type` body, lines 541-638)

Iterate over all rules **once**. For each `LookupSR` or `MaybeCoercingLookupSR`, do the actual env lookup (calling into `env.lookup` — the trait method that dispatches into `templex_scout` / higher-typer name resolution). Depending on what the lookup returns:

- **Primitive** (`Int`, `Bool`, ...) → seed the rune's type as whatever the primitive's `tyype` is.
- **Citizen** (a struct/interface template) → seed the rune's type as the citizen's `tyype`, which is a `TemplateTemplataType`.
- **Templata** (a specific templata instance) → seed as `t.templata`.

**But EXCLUDE certain results from the seed** (this is the subtle bit at lines 555-575):
- If the result is a plain `KindTemplataType`, don't seed. Reason: the rune's type is context-dependent — enclosing rules will pin it.
- If the result is a zero-arg `TemplateTemplataType` returning `KindTemplataType` (i.e. `Ship` = `template<>() -> Kind`), don't seed either — same reason.

This is the mechanism behind `MaybeCoercingLookup`: "I could be a Kind rune or a Coord rune; let the enclosing context decide." Under today's model, the higher typer's `explicify_lookups` picks the winner and inserts `CoerceToCoordSR` if needed.

### Phase 2 — Puzzle-driven fixed-point solve (`get_puzzles_rune_type` + `solve_rule`)

Hand the rules + initial knowns to the generic `SimpleSolverState` framework. It calls two hooks per rule:

**`get_puzzles_rune_type`** (lines 208-260) — "what other runes' conclusions does this rule need before it can fire?" Returns `Vec<Vec<IRuneS>>`:
- `vec![vec![]]` = "no dependencies, fire immediately." Most arms.
- `vec![vec![a.rune], vec![b.rune]]` = "either a OR b conclusion is enough." Used by `EqualsSR`.
- `vec![vec![x.rune]]` = "needs x concluded first."

**`solve_rule`** (lines 264-451) — "produce conclusions for this rule." Each arm stamps a set of `(rune → ITemplataType)` conclusions.

The framework runs a worklist: rules with satisfied puzzles fire, their conclusions unblock more rules, repeat until fixed-point. Result is either a full solve, or a `RuneTypeSolveError` if some runes stay untyped.

## The arm structure today

There are **26 arms** in `solve_rule` (one per `IRulexSR` variant). Grouped by what they do:

**Real logic that survives (renamed or simplified):**
- `Equals` — bidirectional: if left concluded, stamp right; else stamp left from right
- `Lookup` — env-lookup then stamp result's type
- `MaybeCoercingLookup` — same body as `Lookup`, different error path (returns error vs panics today)
- `Literal` — stamp rune with the literal's static type
- `MaybeCoercingCall` — reads template rune's conclusion, stamps args from template's `param_types`
- `RuneParentEnvLookup` — env-chain closure, uses `lookup_rune_type` helper

**Real logic that dies with its rule:**
- `KindComponents` — stamps `kind_rune = Kind` (dead code — no emissions)
- `CoordComponents` — stamps `result_rune=Coord`, `ownership_rune=Ownership`, `kind_rune=Kind`
- `PrototypeComponents` — stamps `result_rune=Prototype`, `params_rune=Pack<Coord>`, `return_rune=Coord`
- `Resolve` — stamps `result_rune=Prototype`, `params_list_rune=Pack<Coord>`, `return_rune=Coord`
- `CallSiteFunc` — same shape as Resolve
- `DefinitionFunc` — same shape as Resolve
- `DefinitionCoordIsa` — stamps `result_rune=Impl`, `sub_rune=Coord`, `super_rune=Coord`
- `CallSiteCoordIsa` — stamps `sub_rune=Coord`, `super_rune=Coord`, optional `result_rune=Impl`
- `OneOf` — asserts all literals share one templata type, stamps rune with that type
- `IsInterface` — stamps rune = `Kind`
- `CoerceToCoord` — stamps `coord_rune=Coord`, `kind_rune=Kind`
- `Augment` — stamps `result_rune=Coord`, `inner_rune=Coord`
- `Pack` — stamps each member = `Coord`, result = `Pack<Coord>`

**Panic stubs (dead — no real emissions):**
- `IsConcrete`, `IsStruct`, `RefListCompoundMutability`, `Call`, `CoordSend`, `IndexList`

## The Kind/Coord acceptance rule (`lookup_rune_type`, lines 455-520)

This helper validates that what `env.lookup` returned matches what the rune was pre-seeded with. Three key acceptance patterns that collapse under onion:

**Line 486:**
```rust
(ITemplataType::KindTemplataType(_), ITemplataType::CoordTemplataType(_)) => {} // Will convert
```
"Actual is Kind, expected is Coord — that's fine, `CoerceToCoord` will handle it." Under Kind=Coord this arm becomes redundant with line 485 (`x == y`), because `Kind` and `Coord` become the same thing. Delete.

**Lines 487-495:**
```rust
(TemplateTemplataType(tt), CoordTemplataType | KindTemplataType)
    if tt.param_types.is_empty() && tt.return_type is Kind|Coord => { implicit call }
```
Zero-arg template returning Kind/Coord flowing into a Kind/Coord rune → auto-invoke. The `Kind|Coord` alternation collapses to just `Kind`. Simplifies to single-branch.

**Line 474, 504:** same story — `Kind|Coord` alternations collapse to just `Kind`.

## What the 4 new `*RefSR` arms look like

Direct successors to `Augment`'s stamp pattern (which today stamps `result_rune=Coord, inner_rune=Coord`):

```rust
IRulexSR::BorrowRef(x) => {
    solver_state.commit_step(false, vec![rule_index], [
        (x.result_rune.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {})),
        (x.inner_rune.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {})),
        // BorrowRef is the only variant that has a region_rune:
        (x.region_rune.rune.clone(), ITemplataType::RegionTemplataType(RegionTemplataType {})),
    ].into_iter().collect(), vec![], IndexSet::default())
}

IRulexSR::HeapOwnRef(x) => {
    solver_state.commit_step(false, vec![rule_index], [
        (x.result_rune.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {})),
        (x.inner_rune.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {})),
    ].into_iter().collect(), vec![], IndexSet::default())
}

// ShareRefSR and WeakRefSR have the same shape as HeapOwnRefSR.
```

Region on `BorrowRefSR` needs a decision: is `region_rune` always `Some`, or can it be `None` (bare `&T` with defaulted region)? If `Option<RuneUsage>`, the arm conditionally stamps.

Puzzle deps for all four: `vec![vec![]]` — fire immediately, same as today's `Augment`.

## The `MaybeCoercing → Lookup` unification (the biggest concrete piece of merge work)

Under the plan, `MaybeCoercingLookupSR` and `LookupSR` become one arm. Concretely:

**In Phase 1 seeding (lines 547-582 for `Lookup`, 583-631 for `MaybeCoercingLookup`):** Bodies are **byte-identical** for the seeding logic. Only difference: `Lookup` panics on `Err(_e)` (line 550), `MaybeCoercing` builds and returns a proper `RuneTypeSolveError` (585-601). Merge = pick the error path (proper error).

**In `get_puzzles_rune_type`:** `Lookup` returns `vec![vec![]]` if predicting or `vec![vec![]]` non-predicting (215-221). `MaybeCoercing` returns `vec![vec![x.rune.rune]]` non-predicting (222-228) — waits for the rune's expected type. **This is a real difference:** `MaybeCoercing` waits until the enclosing context has typed the rune, then validates the lookup against that expectation. `Lookup` fires immediately.

**In `solve_rule`:** `Lookup` (392-407) stamps the rune with the lookup result's type. `MaybeCoercing` (408-420) calls `lookup_rune_type` (which validates but doesn't stamp) and then commits empty conclusions.

**Post-merge model:** one `LookupSR` arm that behaves like today's `MaybeCoercingLookup` in the ambiguous case (rune's type is context-dependent) and today's `Lookup` in the unambiguous case. Simplest form: the puzzle deps become `vec![vec![], vec![x.rune]]` — "fire either when no info is needed, or when the rune's type is known" — but that's a two-puzzle disjunction, which the framework has to handle correctly.

Concrete work items:
1. Pick the seeding path (byte-identical, just wire the error branch).
2. Decide the puzzle-deps behavior — probably keep the "waits until rune is typed" behavior, since it's the more careful one, and audit whether that breaks the `Lookup` current use sites where `predicting=true`.
3. Merge the `solve_rule` arms — combine "stamp result type" (Lookup arm) with "validate against expectation" (MaybeCoercing arm). The stamp fires unconditionally, validation fires when there's an expectation to compare against.

## Total arm accounting

| Kind | Count | Fate |
|---|---:|---|
| Real arms surviving unchanged | 4 | Equals, Literal, RuneParentEnvLookup, LookupSR (post-merge) |
| Real arms surviving with body flip (Coord→Kind stamps) | 3 | Resolve, CallSiteFunc, DefinitionFunc |
| Real arms surviving with rename + body flip | 1 | Pack → KindList (element type Coord→Kind) |
| Real arms deleted with their variants | 13 | KindComponents, CoordComponents, PrototypeComponents, DefinitionCoordIsa, CallSiteCoordIsa, OneOf, IsInterface, CoerceToCoord, Augment, MaybeCoercingCall, MaybeCoercingLookup (merge), + more |
| Panic stubs deleted with their variants | 6 | IsConcrete, IsStruct, RefListCompoundMutability, Call, CoordSend, IndexList |
| **New arms added** | **4** | BorrowRefSR, HeapOwnRefSR, ShareRefSR, WeakRefSR |
| **Net arms after slice** | **~12** | Down from 26 |

## What subtle behavior is preserved

Post-slice, the rune-type solver still does:
1. **Env lookup for Lookup rules** — pre-seed known-rune types.
2. **Puzzle-driven fixed-point** on the surviving rule set.
3. **Kind-context deferral** — some lookups don't pre-seed because their type is context-dependent (via the enclosing `*RefSR` or generic-call context).
4. **Type conflict detection** via `lookup_rune_type` (with the Kind/Coord acceptance rules collapsed to plain `Kind`).

What it stops doing:
1. No more Ownership-axis dispatch. No `OwnershipTemplataType` stamping anywhere.
2. No more Coord/Kind conversion semantics — every rune that was `Coord` is now `Kind`.
3. No more Prototype decomposition (`PrototypeComponentsSR` gone). Prototype construction still works via `Resolve`/`CallSiteFunc`/`DefinitionFunc`.
4. No more `implements`/`isInterface`/`isConcrete` rune shape assertions. Interface/concrete facts come from the value solver's citizen lookup.

Want me to walk through the arm-by-arm rewrite next, or shift back to the identifiability solver (which mirrors this structure but stamps identifiability instead of types)?

---

## 👤 User

why do we run a rune type solver in postparsing? why not just have highertyping do it?

---

## 🤖 Claude

Good question — the answer is nuanced. Postparse isn't actually the sole user of the rune-type solver; it lives in postparse but higher_typing drives it too. There's a division of labor.

## The two-phase model

The solver runs in **two modes**, controlled by the `predicting: bool` param:

**Phase 1 — Predict (postparse-time, `predicting=true`).**
Runs before the full environment is built. Only rules whose puzzles are `vec![]` in predicting mode fire. Env lookups are avoided (`Lookup` and `MaybeCoercingLookup` return no puzzle in predicting mode — see lines 215-220). Output: a **partial** `predicted_rune_to_type` map that captures everything determinable from local rule structure alone — literals, `Augment` results, `Pack` structure, function-signature decompositions, etc.

**Phase 2 — Finalize (higher_typing-time, `predicting=false`).**
Runs after env resolution is available. Now `Lookup`/`MaybeCoercingLookup` fire real env queries. Fills in the runes whose types depend on what a name actually resolves to (`where T = Ship` — need to know Ship is a Kind). Output: the complete `rune_to_explicit_type` map.

The predicted map is stored on the citizen/function AST node during postparse. Higher_typing reads it as a starting point and completes the solve.

## Why the prediction has to happen at postparse

Three things at postparse depend on rune-type info being available before higher_typing runs:

**1. Pattern scout stamps rune types (`pattern_scout.rs:67`).**
Every `AtomSP.coord_rune` gets its type stamped when the pattern is scouted — `CoordTemplataType` today, `KindTemplataType` post-slice. If we deferred this to higher_typing, `AtomSP.coord_rune` would have to become an untyped placeholder that higher_typing fills in.

**2. Rune identifiability check.**
`identifiability_solver.rs` runs at postparse and produces the `IdentifyingRunesIncompleteS` error at postparse time. It depends on knowing rune types to decide whether a rune can be independently identified from context. Moving this to higher_typing means the error surfaces later in the pipeline.

**3. Explicit-type conflict check (`RuneExplicitTypeConflictS`).**
When a user writes `where T Kind, T Coord`, that conflict is detected at postparse via the rune-type solver's conclusion-vs-annotation comparison. Deferrable to higher_typing but same "error surfaces later" cost.

The predicted map on the citizen AST is also used by anonymous-interface / anonymous-substruct macros at typing-time to synthesize derived structs — they need the outer citizen's predicted rune types to correctly propagate them into the derived one.

## Why it lives in postparse (not higher_typing) even though higher_typing drives it

Two reasons:

**Cyclic dependency.** Postparse needs the solver (for prediction). Higher_typing needs the solver (for finalization). The solver code has to live where both can reach it — and postparse is lower in the module tree, so higher_typing depends on postparse (not vice versa). Putting it in postparse is the natural place.

**Coupling to `IRulexSR`.** The solver knows every SR variant. `IRulexSR` lives in `postparsing/rules/rules.rs`. Moving the solver up to higher_typing would either drag `IRulexSR` up with it, or make higher_typing depend on postparse's IR anyway. Same end result, so it stays where it's grounded.

## The architectural question under onion typing

Under onion, higher_typing's raison d'être — `explicify_lookups` deciding when to insert `CoerceToCoordSR` — **dies with Kind=Coord**. `CoerceToCoordSR` becomes identity, so nothing to explicify. The scouting doc estimated the higher-typer main pass shrinks from 1102 → ~500-700 LOC.

What survives in higher_typing after that gutting:
1. **Non-predicting rune-type solve** — the second call to the same solver.
2. **Pattern typing helpers** — `AtomSP.coord_rune → kind_rune` finalization.
3. **Rune identifiability re-check** (or removal — depends).

That's a much thinner pass. Reasonable questions:

- **Could higher_typing die entirely?** If we push its remaining work into postparse (make postparse call the solver in both modes back-to-back once the env is available), then yes. The env-availability question becomes: is the env fully built by end-of-postparse, or does typing bootstrap it?
- **Could we drop the predicting phase entirely?** If every consumer that today depends on the predicted map (pattern scout, identifiability, conflict check) is deferred to typing-time, we skip a whole pass. Cost: errors surface later, `AtomSP.coord_rune` becomes untyped until typing runs.
- **Or split differently?** Keep prediction at postparse but move only the "final env-fed solve" work into postparse's own second pass. Higher_typing dies. This works if postparse can hold the env dependency.

I don't have a strong recommendation because the answer depends on where the env-building actually completes in the pipeline (I'd need to look at the typing-pass entry to know), and on how much the anon-interface macro machinery leans on the predicted map.

**My weak lean:** under onion, the higher-typer becomes a shell around one solver call plus a few pattern-typing helpers. If we're doing the postparse slice anyway, folding higher_typing into postparse-second-pass (or into typing's setup) is a plausible bonus simplification. But it's a design change that goes beyond "make postparse compile under onion" — worth flagging for a separate decision.

Want me to look at what the higher-typer actually does today so we can decide whether to fold it in this slice or keep it as-is, or is that a "next slice" concern?

---

## 👤 User

im not sure your analysis is correct. please dispatch agents to doublecheck everything you just said

---

## 🤖 Claude

Good instinct. Let me dispatch three parallel investigators to fact-check the claims. Each is read-only, reports back with `file:line` evidence.

---

## 🤖 Claude

My analysis was substantially wrong on several key points. The agents came back with hard evidence that inverts my earlier picture. Here's what actually holds up:

## What I got wrong

**REFUTED — "Postparse calls the solver with predicting=true."**
Postparse **never calls `solve_rune_type` at all.** The solver has 7 call sites total:
- `higher_typing/higher_typing_pass.rs:829` — `predicting=false`
- `typing/array_compiler.rs:68, 190, 366` — `predicting=false`
- `typing/overload_resolver.rs:343` — `predicting=false`
- `typing/expression/expression_compiler.rs:623, 2336` — `predicting=false`

**No caller in the crate passes `predicting=true`.** The `predicting: bool` parameter is dead code — probably a Scala-era vestige that didn't finish the port.

**REFUTED — "Two-phase model: predict at postparse, finalize at higher_typing."**
There is no two-phase model. The solver runs once (at higher_typing time), and the "prediction" work is done by a completely different, simpler function.

**PARTIALLY TRUE — "Postparse populates `predicted_rune_to_type`."**
Postparse does populate it, but not via the solver. The mechanism is `PostParser::predict_rune_types` (`post_parser.rs:1409`) — a lightweight group-by-rune duplicate-check that groups explicit type annotations per rune and errors on conflicts. Not a constraint solver.

**REFUTED — "The predicted map is a load-bearing input to higher_typing."**
For structs and interfaces, `higher_typing_pass.rs:383, 386, 530` **explicitly discards** the predicted fields via `_` bindings. Only the function-level `rune_to_predicted_type` is actually read (line 744), and only as the initial-knowns seed for the solver.

**REFUTED — "The anon-interface macro reads `predicted_rune_to_type`."**
Zero matches. The macro reads the higher_typing-resolved `rune_to_type` on `*A` nodes, not the postparse-predicted map.

**PARTIALLY TRUE — "Pattern scout stamps `AtomSP.coord_rune` with a type."**
`AtomSP.coord_rune` is `Option<RuneUsage>` — an untyped rune reference. `pattern_scout.rs:67` stamps `(rune, CoordTemplataType)` into a separate `rune_to_explicit_type: HashMap` (owned by the caller, threaded through), not on `AtomSP`. Downstream (`higher_typing/patterns.rs:15-21`) hardcodes `CoordTemplataType` when reading `pattern.coord_rune` — doesn't consult that map.

**CONFIRMED — "Identifiability runs at postparse."**
`solve_identifiability` (`identifiability_solver.rs:179`) is called only from `PostParser::check_identifiability` (`post_parser.rs:1455`), invoked at `function_scout.rs:794`. Higher_typing never touches it.

**REFUTED (mostly) — "`RuneExplicitTypeConflictS` comes from the rune-type solver."**
The error does fire at postparse time, but from `predict_rune_types` — the lightweight duplicate-check, not the solver. It's group-by-rune conflict detection.

**PARTIALLY TRUE — "Higher_typing's main job is `explicify_lookups`."**
Structurally load-bearing (every `*A::new` constructor asserts no `MaybeCoercing*` survives — the AST literally can't be built without `explicify_lookups` running). But by LOC, only ~13% of the file (~140 lines out of 1102). Bulk is orchestration: 6 `translate_*` fns, package-merge plumbing, caching wrapper.

## Also wrong

- I said higher_typing does pattern typing. **False.** `patterns::get_rune_types_from_pattern` isn't called from `run_pass` at all — it's called from `typing/expression/expression_compiler.rs:618`.
- I said higher_typing does rune identifiability. **False.** Postparse does. Higher_typing passes through already-computed identifying runes.

## The actual model

Cleaner than what I described, and more mundane:

1. **Postparse** has its own lightweight rune-type prediction (`predict_rune_types`) that (a) errors on annotation conflicts and (b) populates `predicted_rune_to_type` fields. For functions, that map is used as a solver seed later. For structs/interfaces, it's essentially unused (higher_typing discards).

2. **Postparse** also runs `solve_identifiability` at function-scout time to check identifying-rune completeness.

3. **The full rune-type solver** (`rune_type_solver.rs`) lives in postparsing/, but nothing in postparse invokes it. It's a shared library. Every real caller is in higher_typing (once) or typing (5 sites). All pass `predicting=false`.

4. **Higher_typing's structural mandate** is `explicify_lookups` — the invariant "no `MaybeCoercing*` survives" is what makes `*A` nodes constructible. Everything else in the file is orchestration.

## What this means for the postparse plan

Two immediate consequences:

**The `MaybeCoercingLookup → LookupSR` merge might not be a rune-type-solver problem at all.** The solver has both arms, but if the arms are only exercised at higher_typing time (via `explicify_lookups`), then the merge work primarily lives in higher_typing, not postparse. Postparse's own predict logic is a separate codepath (`predict_rune_types`) that doesn't touch the MaybeCoercing arms.

**The `predicting` flag is dead code.** No caller passes `true`. When we do the postparse slice, we can delete the `predicting` param entirely (or leave it as `false`-only) — a simplification I didn't flag.

**Higher_typing's role under onion is even smaller than I said.** If `explicify_lookups` dies with `CoerceToCoordSR` (because Kind=Coord makes coercion identity), and `explicify_lookups` is 13% of the file's LOC but is the reason `*A` constructors even work — then the whole `*A` layer's raison d'être evaporates too. Higher_typing may not shrink to 500-700 LOC. It may collapse to a trivial pass whose only job is "call the solver, produce `*A` nodes." Question worth raising: does the whole `*A` intermediate AST even survive, or does typing consume `*S` directly?

Want me to update the postparse-slice-plan.md to correct these errors, or investigate the `*A` collapse question further first?

---

## 👤 User

keep investigating. the reason i bring it up is because i suspect that all solving in postparsing *and* in highertyping is going to be obsolete.

---

## 🤖 Claude

Good hypothesis. Let me investigate whether the current "solving" is actually solving or just walking that could be inlined. Three parallel investigators.

---

## 🤖 Claude

Verdict: your hypothesis is **mostly correct** — but the details matter. Here's the synthesis with hard evidence.

## Rune-type solver (postparse-lives, higher_typing-calls)

**16 of 25 arms are pure syntactic stamping.** No cross-rule dependency, no read of `solver_state.get_conclusion()`, just static-per-variant conclusions. Walk-and-stamp works fine.

**3 arms are non-trivial:**

- **`EqualsSR`** — bidirectional copy. Needs SOME other rule to have typed one side. Requires ordering (topological sort or 2-pass), not fixed-point.
- **`MaybeCoercingCallSR` (merges to `CallSR` under onion)** — genuine data flow: reads `template_rune`'s concluded `TemplateTemplataType{param_types}` and stamps arg runes with param types 1:1. The output types AREN'T statically fixed — they depend on which template symbol resolved. This is the one case that resists a fixed pre-typing walk.
- **`RuneParentEnvLookupSR`** — depends on another rule's output for its own type (not precomputed like `LookupSR`).

**Framework overkill.** Uses puzzle scheduling but never does classic unification / fixed-point iteration / conflict backtracking. The framework does "topological-ish sequencing." A ~50 LOC 2-pass walk (pass 1: stamp fixed-contract rules, pass 2: propagate `Equals` + resolve `CallSR` template-derived arg types) replaces the ~810 LOC solver.

**Precompute pass at lines 541-638** does one-shot env lookups for `LookupSR`/`MaybeCoercingLookupSR` before the framework runs. That's already "just a walk" — trivially preserved.

## Identifiability solver (postparse)

**Framework wildly overkill.** Never triggers conflict detection (conclusions are always literal `true`, no way to conflict). Never reaches Stage 2 complex solve. Small hyperedge structure (a few rules AND multiple runes).

**Best fact:** the returned `HashMap<IRuneS, bool>` is **discarded** at `post_parser.rs:1469` (`Ok(_) => Ok(())`). Only success/failure matters. So identifiability is really "did every rune get marked known before we ran out of firable rules?" — a monotone reachability question over a small hypergraph.

**Replacement:** ~30 LOC monotone worklist. Or arguably deletable entirely if you're willing to move the check to typing-time (where you'd catch it via a different mechanism).

## Value solver (`compiler_solver.rs`)

**Genuinely needs to survive, but much smaller.**

**Dead outright under onion:**
- Stage 2 `complex_solve` (~190 LOC): operates exclusively on `CoordSend`/`CallSiteCoordIsa`. Both retiring. Delete.
- `solve_receives`/`narrow` helpers (~80 LOC): same fate.
- 16 SR arm handlers.

**One-directional survivors (inline-able at instantiation sites):**
- `DefinitionFuncSR`, `CallSiteFuncSR` — pure `prototype ↔ (params, return)` decomp/assembly. Trivial helpers.
- `LookupSR`, `LiteralSR` — one-directional lookup/stamp.
- `ResolveSR` — already effectively inline (`resolve_function` at post-solve time, `predict_function` at solve time based on availability).

**What resists inlining (~150-200 LOC of surviving solver work):**
- **`EqualsSR`** — order-flexible unification.
- **`CallSR`** — bidirectional: (template + args) → kind, OR kind → (template + args). Decomposition direction genuinely used.
- **`PackSR`/`KindListSR`** — bidirectional assembly/disassembly. Disassembly is exercised by concept-function-bound plumbing (`anonymous_interface_macro.rs:463-476, 594-610`): `CallSiteFuncSR` populates `params_list_rune` from a solved prototype, then `PackSR` distributes coords back to per-param runes. Rare but real.

**Verdict on residual value solver:** ~150-200 LOC. Not 1556. But not zero either — the concept-function-bound / bidirectional `Call`/`Pack` pattern is real work that isn't clean to inline at specific AST sites.

## What this means for the plan

**Your instinct holds for postparse + higher_typing solving.**
- `predict_rune_types` (postparse): not the solver, already a walk. Keeps its shape.
- `identifiability_solver`: 270 LOC → 30 LOC walker, or delete if we're OK moving the check.
- `rune_type_solver`: 810 LOC → ~50-80 LOC walker (fixed-contract stamp pass + Equals/CallSR resolution pass). Delete the `SimpleSolverState` framework usage entirely.

**Higher_typing's collapse becomes bigger than I said.** Under onion:
- `explicify_lookups` dies with `CoerceToCoordSR` (~140 LOC).
- Rune-type solver call at :829 → replaced by walker (~50 LOC, potentially moved inline or into postparse).
- Package-merge plumbing (~230 LOC) doesn't obviously need to be its own pass.
- Caching wrapper (~95 LOC).
- 6 `translate_*` orchestrators (~430 LOC) that build `*A` nodes — but if the whole pass collapses, maybe `*A` nodes aren't needed at all and typing consumes `*S` directly.

**The `*A` intermediate AST is the interesting question.** Every `*A::new` constructor asserts "no `MaybeCoercing*` survives." That invariant is the reason `*A` exists as a stage separate from `*S`. If `MaybeCoercing*` merges into plain `Lookup`/`Call` (and there's no coercion decision to bake in), then `*A` and `*S` become structurally identical. `*A` collapses into `*S`.

**Higher_typing collapses to zero** if that's true. Not "500-700 LOC after cleanup." Delete the whole pass; typing reads `*S` directly.

**Value solver survives at 150-200 LOC.** Real work. The `EqualsSR`/bidirectional `CallSR`/concept-function-bound-`PackSR`-disassembly triangle can't cleanly go inline. But 90% shrink.

## Follow-up questions worth investigating before landing this design

1. **Does typing depend on `*A`-specific fields that don't exist on `*S`?** If yes, some of higher_typing's translate_* work has to happen somewhere (in postparse, or inline in typing). If no, `*A` disappears cleanly.
2. **Are the anon-interface-macro-driven concept-function bounds the ONLY use of `PackSR` disassembly?** If yes, we can inline that specific chain and drop `PackSR` bidirectionality — collapsing more solver.
3. **What does typing's use of `solve_rune_type` (5 sites in array_compiler/overload_resolver/expression_compiler) look like?** Are those calls also just "walk-shape," or do they exercise `Equals`/`CallSR` bidirectionality that would force the walker to stay solver-shaped?

Want me to fire another round to investigate those three?

---

## 👤 User

so wait, why is this not all in typing? whats the benefit of having all this before typing?

---

## 🤖 Claude

Fair question — let me steelman the current architecture, then explain why it doesn't hold up under onion.

## The three traditional reasons for pre-typing passes

**1. "Names before types."** Classical compiler layering: resolve identifiers to canonical names before assigning types. Vale postparse does real work here that isn't type-shaped:

- **Naming.** `INameS` canonical names, `IImpreciseNameS` for lookup, name resolution across nested scopes (`IEnvironmentS`, `StackFrame`, lambda captures).
- **Expression lowering.** Surface `IExpressionPE` → scout `IExpressionSE`. Loop lowering (`loop_post_parser.rs`, 528 LOC). Variable-use tracking (`variable_uses.rs`).
- **`LoadAsP` synthesis.** Turning `Move`/`Borrow`/`Weak`/`Share` prefixes into `OwnershippedSE` nodes with target-ownership tags.
- **Scope machinery.** Which rune is declared where, which lambda captures which local.

None of this is typing. All of it needs to happen before typing runs — you can't type an expression whose identifiers haven't been resolved to scoped canonical names.

**2. "Rune-typing before value-typing."** Higher_typing's classical reason to exist: figure out `rune → ITemplataType` before the value solver runs, because the value solver needs to know "when I look up `T`, do I want a Coord result or a Kind result?" — different lookup paths.

**Under onion this evaporates.** Kind=Coord means there's no Kind-vs-Coord decision to make. The only rune types left (Kind, Region, Prototype, Impl, Int, Bool, String, Pack, Template, Function) either come from explicit annotation, literal, or a fixed SR contract. Rune "typing" collapses to a walk.

**3. "Cache the intermediate."** Traditional argument for stage boundaries: a per-file postparse output is cacheable; typing is program-wide and isn't. Splitting them lets incremental compilation invalidate at the right granularity.

**Vale doesn't do incremental compilation.** Not today, not on any current roadmap. So this benefit is theoretical.

## What actually justifies postparse (survives onion)

Real work postparse does that has to happen before typing:

- Canonical name resolution
- Scope tables & lambda capture
- Expression lowering (LoadAsP, loops, etc.)
- Some early errors that don't need types (identifiability, variable-use, statement-after-return)

**None of this is solving.** The parts of postparse that ARE solving (rune-type prediction for functions, identifiability, explicit-type conflict) are ~15% of the work. Everything else is real non-typing lowering.

**Recommendation for postparse:** keep the pass, delete `identifiability_solver.rs`, delete the solver-shaped scaffolding from `predict_rune_types`, keep the naming/scoping/lowering work.

## What justifies higher_typing (doesn't survive onion)

Higher_typing exists because pre-onion:
- Kind vs Coord was a real distinction, and `MaybeCoercing*` rules needed to know which one before typing.
- `explicify_lookups` inserted `CoerceToCoordSR` to bridge the ambiguity.
- The `*A` AST was the "post-explicify" invariant witness.

**Under onion, every one of these dies:**
- No Kind-vs-Coord to distinguish.
- No `CoerceToCoordSR` to insert.
- `MaybeCoercing*` collapses to plain `Lookup`/`Call`.
- The `*A` invariant becomes trivially satisfied by `*S` (nothing to explicify).

What's left in higher_typing's 1102 LOC:
- ~140 LOC of `explicify_lookups` — dies.
- ~50 LOC of `solve_rune_type` invocation — moves to typing.
- ~230 LOC of package merging — has to happen somewhere; moves to typing entry.
- ~430 LOC of `translate_*` orchestrators building `*A` nodes — dies with `*A`.
- ~95 LOC of caching wrapper — dies.

**Recommendation for higher_typing:** delete the pass. Move package merging to typing entry. Move the rune-type walk (successor to `solve_rune_type`) to wherever typing needs it — probably typing entry, run once per denizen before value inference.

## What actually gets simpler

Under this architecture:

**Postparse** (kept): Names, scopes, expression lowering. No solving. ~15-20% smaller after shedding solver machinery.

**Higher_typing** (deleted): Nothing left after `explicify_lookups` dies. Package merging + rune-type walk absorb into typing entry.

**Typing** (still exists): Value solving (~200 LOC residual solver for `Equals`/`Call`/`Pack` bidirectionality + concept-function bounds), inline unification at call sites (strip-solve-check), coercion at the 7 enumerated AST sites (call args, return, let, mutate, struct fields, array elements, if/match branches via LCA).

## The one non-onion argument for keeping stages separated

**Error surface.** Errors detected at earlier passes fire before you attempt later work. If typing has a bug, you still get sensible naming/scoping errors from postparse. Delete higher_typing → some errors that today fire at higher_typing (`CouldntFindType`, `TooManyMatchingTypes`) fire at typing instead. Not obviously worse.

**Verdict.** The pre-typing separation IS defensible for non-typing work (naming, scoping, expression lowering). Postparse keeps that. But the *typing* work that lives at postparse and higher_typing is defensible only under pre-onion assumptions. Onion collapses higher_typing entirely and sheds the solver-shaped parts of postparse.

Want me to sketch what "typing entry that consumes `*S` directly" would look like, or investigate whether typing has hidden dependencies on `*A`-specific structure before we commit to the collapse?

---

## 👤 User

so wait, why is this not all in typing? whats the benefit of having all this solving before typing?

---

## 🤖 Claude

Direct answer: **there's no real benefit under onion typing.** It's historical.

## The reasons someone might have had, and why they don't hold up

**1. "Rune-typing is independent of value-typing."** In principle: you can figure out "T is a Kind" without knowing "T = Ship." So do it early, feed clean rune-types into the value solver. **In practice:** the value solver still has to do bound resolution, template instantiation, and generic-param inference — all of which reference rune-types anyway. Nothing about doing rune-typing early makes the value solver simpler; it just gives value-solver a `HashMap<IRuneS, ITemplataType>` as input instead of computing the same map inline at entry.

**2. "`*A` is a stable IR."** Higher_typing produces `*A` nodes whose constructors assert "no `MaybeCoercing*` survives." Typing consumes `*A`. This was a witness that pre-typing coercion decisions were made. **Under onion:** `MaybeCoercing*` merges into plain `Lookup`/`Call`, no coercion decision to bake in, `*A` becomes structurally identical to `*S`. The witness is trivially true. `*A` is dead weight.

**3. "Separate error classes."** Rune-typing errors ("this rune was declared Kind but appears as Coord") are conceptually distinct from value-typing errors ("wrong args for template"). Separate stages = cleaner error surfaces. **Under onion:** most of these errors go away. Kind=Coord makes Kind-vs-Coord conflicts vanish. The remaining rune-type errors (`RuneExplicitTypeConflictS`, `FoundCitizenDidntMatchExpectedType`) can fire at typing time without loss.

**4. "Caching / incremental compilation."** Traditional argument for stable pre-typing IRs. **Vale doesn't do this.**

**5. "Package merging happens at higher_typing."** Higher_typing merges per-file `ProgramS` into per-package. That's real cross-file work. **But it doesn't need its own pass.** Move to typing entry, run once.

**6. "The Scala port did it this way."** The honest answer. The Rust port faithfully mirrors the Scala structure. Scala's rune-typing lived in a separate pass; Rust's copies that.

## What actually justifies pre-typing SOLVING under onion?

Nothing. The onion typing decisions (Kind=Coord, no overload resolution, coercion at enumerated AST sites, strip-solve-check) make every pre-typing rune-solve either trivial or redundant.

The rune-type solver's non-trivial arms (`EqualsSR`, `CallSR` bidirectionality, `PackSR` disassembly) work on the **same solver framework** the value solver uses. There's no benefit to invoking that framework twice — once at higher_typing on rune-types, once at typing on values. Do it once, at typing, on both together.

## Recommendation

Delete higher_typing entirely. Move package merging to typing entry. Delete `identifiability_solver.rs` (or shrink to a ~30-LOC walker). Delete `rune_type_solver.rs` (or fold into a small helper at typing entry). Postparse keeps its non-solving work (names, scopes, expression lowering); everything solver-shaped moves to typing.

**Payoff:** one solver (in typing) instead of three. ~2500 LOC deleted across `rune_type_solver.rs` + `identifiability_solver.rs` + `higher_typing_pass.rs` + `higher_typing/ast.rs` + associated humanizers/traversers.

**Cost:** some errors that today fire at postparse/higher_typing fire at typing instead. Not obviously worse. The `*A` layer's failure-loud "no MaybeCoercing survives" invariant goes away, but that invariant is trivially satisfied by `*S` under onion anyway.

Want me to trace what the typing entry point currently expects (`*A` fields it reads) so we can scope the "typing consumes `*S` directly" work?

---

## 👤 User

so we can get rid of higher_typing, what about the solving in postparsing?

---

## 🤖 Claude

Yes — all three can move. Postparse ends up with zero solving.

## What "solving in postparsing" actually is today

Three pieces:

**1. `predict_rune_types` in `post_parser.rs:1409`.** Not really a solver — a group-by-rune duplicate-check. Groups every explicit rune-type annotation per rune and fires `RuneExplicitTypeConflictS` on conflict.

**2. `solve_identifiability` in `identifiability_solver.rs`.** Called only from `function_scout.rs:794` via `check_identifiability`. Fires `IdentifyingRunesIncompleteS`. Result is discarded (only success/failure matters).

**3. `rune_type_solver.rs` (810 LOC).** Lives in postparse but is never invoked from postparse. Only higher_typing (which we're deleting) and typing call it.

## Moving each to typing

**`predict_rune_types`** — dies almost entirely.
- Its output for structs/interfaces is already discarded by higher_typing (bound to `_`).
- Its output for functions (`rune_to_predicted_type`) feeds higher_typing's solver as initial-knowns. When higher_typing dies, so does the seed.
- The `RuneExplicitTypeConflictS` detection is the only real work. Cheap to preserve at typing entry: when typing builds the rune-type map, catch the same annotation conflict.

Delete cost: ~50-80 LOC across `predict_rune_types` + its callers.

**`solve_identifiability`** — moves as a ~30-LOC helper called at typing entry.
- Same monotone hypergraph reachability (per Agent 2's finding).
- Same trigger point (once per function or denizen at typing entry).
- Same error (fires from typing instead of postparse).

Delete cost: `identifiability_solver.rs` file (~270 LOC) shrinks to a small helper (~30 LOC).

**`rune_type_solver.rs`** — moves out of `postparsing/` into `typing/` (or wherever the residual solver lives).
- Postparse doesn't call it anyway. Just moving the source file.
- Under onion, the file itself shrinks dramatically (walker replaces framework, per Agent 1's finding).

Move cost: file relocation + import updates.

## What's left of postparse after solving leaves

Non-solving work only:
- Names & canonical name construction (`INameS`, `IImpreciseNameS`, `IRuneS`)
- Scope tables (`IEnvironmentS`, `StackFrame`, variable-use tracking)
- Expression lowering (`IExpressionPE` → `IExpressionSE`, LoadAsP synthesis)
- Loop lowering, closure capture resolution
- The rule scout & templex scout (translating parse-AST templex/rules to scout-AST rules — pure structural translation, no solving)
- Errors that don't need types: `CouldntFindVarToMutateS`, `CouldntFindRuneS`, `StatementAfterReturnS`, `VariableNameAlreadyExists`, `InterfaceMethodNeedsSelf`, `ExternHasBodyS`, `RangedInternalErrorS`, `CantOwnership*InImpl`, `VirtualAndAbstractGoTogether`.

**Postparse becomes purely "surface syntax → normalized scout AST + scope tables."** No typing decisions at all.

## The one honest cost

**Errors fire later.** Today:
- `RuneExplicitTypeConflictS` fires at postparse (immediately after parsing).
- `IdentifyingRunesIncompleteS` fires at postparse (right after function scouting).

After the move:
- Both fire at typing entry.

Not obviously worse — typing entry runs anyway per denizen, and both checks are cheap enough not to affect perf. But if you care about "wrong-annotations error before any type work happens," you lose that.

## Recommendation

Yes, move all three. Postparse becomes purely non-typing work. The complete stack under onion:

```
parsing/lexing        →  bytes → parse tree
postparsing           →  names, scopes, expression lowering (no solving)
[higher_typing        →  deleted entirely]
typing                →  rune-type walk + value solving + coercion
                         (with residual ~150-200 LOC solver for Equals/Call/Pack)
instantiating/hammer  →  as today
```

Three passes doing solving today → one pass doing solving after the refactor. Postparse output is a clean, typing-agnostic normalized AST.

Want me to update the postparse-slice-plan.md to reflect this? The slice's scope shifts — postparse's own solver code deletes rather than getting rewritten, and the plan becomes "postparse comes back online with its solving-related types deleted; typing eventually absorbs the moved-out work in a subsequent slice."

---

## 👤 User

are you sure that typing pass doesnt invoke any rune type solving?

---

## 🤖 Claude

Good catch. Let me check the 5 typing-side call sites of `solve_rune_type` that Agent 1 flagged.

---

## 🤖 Claude

Big correction to what I said. **Typing invokes the full rune-type solver pipeline — including `explicify_lookups` — at multiple contextual sites during typing.** This isn't a one-time entry-point call.

Concrete evidence from the three sites:

**Site 1: `array_compiler.rs:60-100+` (`resolve_static_array_from_callable` and siblings).**
- Takes `rules_with_implicitly_coercing_lookups_s` as input — rules that still contain `MaybeCoercing*` variants at this point.
- Calls `solve_rune_type` (line 68) to type-check those rules.
- Then calls `explicify_lookups` (line 86) — meaning the coercion-decision pass runs here, at typing time, on typing-time-supplied rules.
- Purpose: infer element type and size type when constructing a runtime/static array where the callable's signature has template-arg-derived rune shape.

**Site 2: `overload_resolver.rs:343`.**
- Called per-candidate during overload resolution.
- Takes `explicit_template_arg_rules_with_connections` — rules generated at the call site from user-supplied explicit template args like `foo<Ship, Int>()`.
- Solves to figure out which rune each explicit template-arg position binds.

**Site 3: `expression_compiler.rs:623` (`IExpressionSE::Let`).**
- Called per `let` statement.
- Seeds with `get_rune_types_from_pattern(&let_se.pattern)` — the pattern's already-known rune types.
- Passes `let_se.rules` — rules attached to the let pattern.
- Solves to type-check the pattern's runes given the source expression's inferred type.

## What this means for the "delete higher_typing" recommendation

**Still true:** higher_typing's per-denizen pre-typing invocation of `solve_rune_type` + `explicify_lookups` can be absorbed into typing entry (one call per denizen). The higher_typing PASS can die.

**But false:** "typing does the solving once at entry and then walks." **Typing invokes the rune-type solver + `explicify_lookups` many times per denizen:**
- Once at denizen entry (replacing higher_typing's per-denizen call).
- Once per array/vec construction site.
- Once per overload candidate (times number of candidates).
- Once per `let` binding.
- Probably more sites I haven't traced.

The solver has to survive as a library that typing calls on demand.

**Under onion:**
- The 810-LOC framework-based solver in `rune_type_solver.rs` still shrinks to a ~50-80 LOC walker (per Agent 1: 16 arms are fixed-contract stamps, only Equals/CallSR/RuneParentEnvLookup need real ordering).
- `explicify_lookups` dies at every site — array_compiler line 86 stops existing, overload_resolver equivalent stops, let-expression equivalent stops.
- But `solve_rune_type` (in walker form) is still called from the same sites for the same reasons.

## Revised model

Pre-onion, today:
- **postparse:** predict + identifiability (dies)
- **higher_typing:** per-denizen solve + explicify (pass dies, work moves to typing entry)
- **typing:** solve + explicify at 5+ contextual sites (survives; explicify dies with CoerceToCoordSR)

Post-onion:
- **postparse:** naming, scopes, expression lowering. No solving.
- **higher_typing:** deleted.
- **typing:**
  - At entry: rune-type walk (per denizen).
  - At array/overload/let sites: same walk on contextual rules.
  - No explicify anywhere.
  - Value solver: ~150-200 LOC residual for Equals/Call/Pack bidirectionality.

So my "one pass doing solving" phrasing was misleading. Better phrasing: **one module owns the solver code, and it's typing.** But typing calls it many times, not once.

## What's actually at risk here

The subtle claim I made — "postparse solving dies, typing entry absorbs everything" — is right for the postparse/higher_typing SIDE, but I understated typing's dependency on the solver. Under onion the solver's SHAPE shrinks (walker replaces framework), but its INVOCATION PATTERN in typing doesn't change: still multi-site, still on demand.

Concrete correction to the plan:
- Move `rune_type_solver.rs` from `postparsing/` to `typing/` (owner is typing, not postparse — the location is wrong today).
- Shrink it to a walker.
- Postparse deletes `predict_rune_types` + `identifiability_solver.rs` + its solver-shaped scaffolding.
- Higher_typing deletes entirely.
- **Do not** promise "typing does the solve once at entry" — that's not the model.

Want me to trace the other sites (I stopped at 3 — there are 5 in typing plus maybe more I haven't found) so we have a full inventory of where typing needs the walker?

---

## 👤 User

sounds reasonable. the higher typing pass dies, but higher typing solving stays as a library, but moved into a subfolder in typing preferably. postparsing doesnt do any solving. is that all right?

also, IIRC the higher typing has its own pass outputs, which it consults recursively. i assume that will become some sort of maps in the coutputs?

---

## 🤖 Claude

Yes to #1 — that's the plan. Let me verify #2 by looking at what `coutputs` already holds and what higher_typing outputs contain.

---

## 🤖 Claude

Yes, that maps naturally to coutputs. Concrete answer:

## What higher_typing outputs today

Each `*A` node (`ProgramA`, `StructA`, `InterfaceA`, `ImplA`, `FunctionA`, `ExportAsA`) carries the same fields as its `*S` counterpart PLUS two things:

- **`rune_to_type: ArenaIndexMap<IRuneS, ITemplataType>`** — the solved rune-type map (or `header_rune_to_type` + `members_rune_to_type` split for structs).
- **`rules: &[IRulexSR]`** — post-`explicify_lookups` rules, with the asserted invariant "no `MaybeCoercing*` survives" (enforced by `*A::new` constructors at `ast.rs:110-123, 169-175, 235-241, 311-318`).

Everything else on `*A` is verbatim from `*S`.

## How typing consumes them recursively

From `compiler.rs:540-615`: typing iterates `program_a.structs / interfaces / impls / functions`, wraps each denizen as an `IEnvEntryT::Struct(struct_a)` etc., inserts into environment entries, then creates templatas that hold `origin_struct: struct_a` etc. (compiler.rs:738-744).

When typing processes struct A and A's rules reference struct B, it does `env.lookup(B_name)` → finds `IEnvEntryT::Struct(struct_b_a)` → uses `struct_b_a`'s `header_rune_to_type` + `header_rules`. Recursion via env lookup into other denizens' cached `*A` data.

## What replaces `*A` under onion

Under onion:
- **`*A` invariant becomes vacuous.** `MaybeCoercing*` merges into plain `Lookup`/`Call` at the postparse slice, so `*S`'s rules already satisfy the "no `MaybeCoercing` survives" constraint. `*A::new`'s panic-asserts become tautologies.
- **`rules` field is identical to `*S`.** `explicify_lookups` dies with `CoerceToCoordSR`. The post-explicify rule list = the pre-explicify rule list.
- **Only the `rune_to_type` map is real new data.** Everything else on `*A` is 1:1 with `*S`.

So `*A` collapses. Typing consumes `*S` directly. What's left to hold: the per-denizen `rune_to_type` map.

## Where the map lives — coutputs, per your guess

`CompilerOutputs` (compiler_outputs.rs:52) already has this shape for other per-denizen info:
- `type_name_to_outer_env: HashMap<IdT, IInDenizenEnvironmentT>`
- `type_name_to_inner_env: HashMap<IdT, IInDenizenEnvironmentT>`
- `type_name_to_mutability: HashMap<IdT, SharednessT>`
- `struct_template_name_to_definition: IndexMap<IdT, &StructDefinitionT>`
- `instantiation_name_to_bounds: HashMap<IdT, &InstantiationBoundArgumentsT>`

Natural additions:
```rust
pub type_name_to_rune_types: HashMap<IdT<'s, 't>, ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>>,
pub function_name_to_rune_types: HashMap<IdT<'s, 't>, ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>>,
```

Or a single unified `denizen_name_to_rune_types` keyed by IdT — struct-header vs struct-members can be a wrapper enum since only structs need the split.

Populated when: typing first encounters a denizen (matching the existing `type_name_to_outer_env` population pattern). The lookup happens via the same env-chain mechanism typing already uses — env lookup returns `IEnvEntryT::Struct(struct_s)`, and any code that today reads `struct_a.header_rune_to_type` reads `coutputs.type_name_to_rune_types[struct_name]` instead.

## The recursion works the same

Today's recursion works because typing has already-processed `*A` entries in the env by the time later denizens look them up. Higher_typing produces the entire `ProgramA` before typing starts.

Under the collapse:
- Typing entry runs the rune-type walk per denizen (topologically or on-demand).
- Result lands in `coutputs.type_name_to_rune_types`.
- Later denizens' env lookups consult `coutputs` — same pattern as consulting `type_name_to_outer_env` today.

**Ordering.** Higher_typing today processes in the order given by `translate_program`. Typing already has its own denizen-processing order (structures → interfaces → impls → functions, deferred queues for out-of-order needs). The rune-type walk piggybacks: run it as a first step of each denizen's typing processing, before value work.

Cyclic references between denizens work the same as today — the env has entries as soon as they're declared (name → outer env), value/rune-type work fills in later.

## Small subtlety on the 5+ contextual sites in typing

The typing-time calls at `array_compiler.rs:68`, `overload_resolver.rs:343`, `expression_compiler.rs:623`, etc. solve rules that aren't per-denizen — they're per-call-site (arrays, overload candidates, lets). Those still call the walker on demand with locally-supplied rules. The map they build doesn't need coutputs storage — it's a transient result used to resolve one site's inference.

So coutputs holds per-denizen rune-type maps; the transient per-site solves stay stack-local.

## Confirming your plan

- **Higher_typing pass dies:** ✓
- **Rune-type solving stays as a library, in typing subfolder** (probably `typing/rune_typing/` or `typing/infer_rune_types/`): ✓
- **Postparse does no solving:** ✓
- **Higher_typing's per-denizen outputs → maps in coutputs:** ✓ — matches the existing `type_name_to_*` pattern.

Ready to update the plan doc when you say. Anything else you want me to verify first?

---

## 👤 User

sounds good. please update vcoord-handoff.md with this direction

---

## 🤖 Claude

Let me open the handoff to find the right anchor for the update.

---

## 🤖 Claude

Now let me update the "Where to find more context" section to reference `postparse-slice-plan.md`.

---

## 🤖 Claude

Also updating the top read-order to include the postparse plan.

---

## 🤖 Claude

Done. Updates applied to `vcoord-handoff.md`:

1. **Read-order intro** (lines 5-11) — added `postparse-slice-plan.md` as #2 in the sequence; noted `onion-typing-plan.md` predates recent refinements.

2. **"Immediate next step" section** (was ~10 lines of stale postparse work items, now ~60 lines) — captures the architectural direction:
   - Higher_typing pass dies entirely (with the LOC breakdown)
   - Rune-type solver moves to `typing/` subfolder as a library (multi-site typing caller, not one-time)
   - Postparse does zero solving (with what dies, what survives)
   - Higher_typing per-denizen outputs → coutputs maps (with concrete field shapes)
   - Value solver shrinks to ~150-200 LOC residual under onion
   - 4-slice ordering: postparse → higher_typing deletion → typing → instantiator/hammer/backend
   - Anon-interface macro note (reads new coutputs maps instead of `*A` fields)

3. **"Where to find more context"** — added `postparse-slice-plan.md` entry, noted `onion-typing-plan.md` is pre-refinement reference.

The old postparse-slice work-items list (that talked about `AugmentSR needs restructuring`, etc.) is replaced — the postparse-slice-plan.md now owns that detail, and the handoff sends readers there instead of duplicating.

Nothing else changed. Missions, coercion table, state-at-handoff block, history sections, blocks/preserve, critical reminders — all intact.

---

## 👤 User

this session, lets just do the postparsing changes. please make a plan for it.

---

## 🤖 Claude

Following plan mode workflow. Phase 1 — one Explore agent to verify postparse test coverage and confirm no surprise callers of the solvers.

---

## 🤖 Claude

I have enough context. The Explore agent confirmed the test surface. Skipping Phase 2 Plan agents — the design is fully specified from this session's earlier work (the postparse-slice-plan.md I wrote, the recent vcoord-handoff.md update, and the design-verification agents).

One scope question to lock before finalizing.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

and feel free to unlink highertyping and all passes after it. i dont want to preserve anything that we dont want in the final postparser. nuke everything that we dont want long term.

---

## 🤖 Claude

Understood. Scope is now: postparse at long-term shape, no half-measures, nuke everything else.

Let me write the plan.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

proceed, ty

---

## 🤖 Claude

Setting up task tracking, then starting P1.

---

## 🤖 Claude

Full rewrite of rules.rs at long-term shape.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

<command-message>guardian-ordain</command-message>
<command-name>/guardian-ordain</command-name>
<command-args>"thunder road" 7880</command-args>

---

## 🤖 Claude

Ordained. Retrying the rules.rs rewrite.

---

## 🤖 Claude

Now itemplatatype.rs — delete `CoordTemplataType`, `OwnershipTemplataType`, `LocationTemplataType`, `PrototypeTemplataType`.

---

## 🤖 Claude

P1 done. Moving to P2.

---

## 🤖 Claude

Let me see the `INameValS` companions and check for existing twins.

---

## 🤖 Claude

The picture: several `*CoordRune` variants have `*KindRune` twins (collapse), two don't (rename), three ownership variants die outright. Renames first via `replace_all`.

---

## 🤖 Claude

Now deleting the 9 dying variants. Grep first to find all sites.

---

## 🤖 Claude

Deleting variants from the IRuneS enum first.

---

## 🤖 Claude

Now the `canonical_ptr()` match arms.

---

## 🤖 Claude

Now the `IRuneValS` enum + Equivalent impl arms.

---

## 🤖 Claude

Now the Equivalent impl match arms.

---

## 🤖 Claude

Now `AtomSP.coord_rune → kind_rune`.

---

## 🤖 Claude

Bulk `coord_rune` → `kind_rune` rename across postparse.

---

## 🤖 Claude

Now `IRegionMutabilityS` deletion and `CoordGenericParameterTypeS` restructure in ast.rs.

---

## 🤖 Claude

Now the `FunctionS` `rune_to_predicted_type` removal.

---

## 🤖 Claude

Let me finish the P2 mutability/imports cleanups then flow into P3.

---

## 🤖 Claude

Now the interface predict_rune_types caller.

---

## 🤖 Claude

Now function_scout.rs's predict_rune_types caller + check_identifiability caller.

---

## 🤖 Claude

The `create_closure_param` fn needs a rewrite for onion — it uses several dead SR variants. Let me look at the full function and its caller.

---

## 🤖 Claude

Now delete the `predict_rune_types` + `check_identifiability` call from function_scout.rs.

---

## 🤖 Claude

Now delete the two solver files and update mod.rs.

---

## 🤖 Claude

P3 done. Marking complete and moving to P4.

---

## 🤖 Claude

Now the templex_scout main dispatch changes. Add BorrowRef/HeapOwnRef/ShareRef/WeakRef, delete Inline, delete stub comment, flip MaybeCoercingCall→Call, Pack→KindList.

---

## 🤖 Claude

Now rule_scout.rs.

---

## 🤖 Claude

Now `expression_scout.rs`. Let me see the key sites.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

"ITypePR::OwnershipType => panic!("POSTPARSER_TRANSLATE_TYPE_OWNERSHIP_RETIRED"),"
whats this about?

---

## 🤖 Claude

Fair question. Those panics are placeholders — the parser (`templex_parser.rs:744, 756, 764`) still produces those variants from surface syntax like `T Coord`, `T Ownership`, `T Prot`. Under onion those surface forms should be dead.

Two ways to handle this cleanly under the "nuke everything not wanted long term" directive:

1. **Also retire at the parser** — delete `ITypePR::CoordType`, `ITypePR::OwnershipType`, `ITypePR::PrototypeType` variants + the parser code that produces them + tests that assert them. `T Coord` becomes a parse error. Consistent with the parser-at-long-term-shape principle.

2. **Keep the panics** — parser still produces, postparse rejects at scout time. Ugly but keeps parser untouched.

The plan says postparse only, but this is a shape-level inconsistency that goes against the "nuke" directive. I'd lean option 1 — it removes the ITypePR asymmetry and finishes what the parser slice started.

Should I go option 1 (small parser cleanup + tests) or leave the panics? Or delete rulescout's translate_type panic arms entirely and let Rust's non-exhaustive match error force the parser cleanup?

---

## 👤 User

option 1, nuke, ty

---

## 🤖 Claude

Now find remaining references to the retired variants across parser + tests + keywords.

---

## 🤖 Claude

Now the `ref_`, `prot`, `ownership` keywords in Keywords struct.

---

## 🤖 Claude

Parser cleanup lands cleanly. Let me verify parser tests then try linking postparsing.

---

## 🤖 Claude

Now bulk rename `Ref` → `Kind` in the other 12 files (only `T Ref`-style usage).

---

## 🤖 Claude

Parser tests green (385/0/1). Now linking postparse to see remaining P4 breakage.

---

## 🤖 Claude

Now bulk fix imports/references across postparse files.

---

## 🤖 Claude

Now fix function_scout and post_parser remaining issues.

---

## 🤖 Claude

Build clean! Now testing.

---

## 🤖 Claude

Now post_parser_tests.rs.

---

## 🤖 Claude

Postparse tests: **67/0/0 green**. Now full suite verification.

---

## 🤖 Claude

All six sub-slices done and clean. Final state:

**Build:** `cargo build --lib` clean, zero warnings.

**Test suite: 489 passed / 0 failed / 1 ignored**
- `parsing::tests`: green (dropped a few tests using retired `Ref`/`Prot`/`Ownership` syntax)
- `lexing`: 3/0/0
- `postparsing::tests`: 67/0/0 (new baseline; deleted 4 test files that tested retired features)

**What landed under the "nuke everything not wanted long term" directive:**

- `IRulexSR`: 26 → 13 variants. Deleted `AugmentSR`, all `Coord*SR`, `CoerceToCoordSR`, `KindComponentsSR`, `PrototypeComponentsSR`, `MaybeCoercing*`, `OneOfSR`, `IsInterfaceSR`, `IsConcreteSR`, `IsStructSR`, `RefListCompoundMutabilitySR`, `IndexListSR`. Added `BorrowRefSR`, `HeapOwnRefSR`, `ShareRefSR`, `WeakRefSR`. Renamed `PackSR → KindListSR`.
- `ITemplataType`: 13 → 9. Deleted `CoordTemplataType`, `OwnershipTemplataType`, `LocationTemplataType`, `PrototypeTemplataType`.
- `ILiteralSL`: 5 → 3. Deleted `LocationLiteral`, `OwnershipLiteral`.
- `ITypePR` (parser): deleted `OwnershipType`, `CoordType`, `PrototypeType` — parser now rejects `T Ownership` / `T Ref` / `T Prot` as parse errors.
- 8 `*CoordRune` variants renamed/collapsed into `*KindRune` equivalents; 3 `*OwnershipRune` variants deleted; `AtomSP.coord_rune → kind_rune` (~200 sites); `CoordGenericParameterTypeS → KindGenericParameterTypeS` (dead fields dropped); `IRegionMutabilityS` enum + field deleted; `predicted_rune_to_type` fields on `StructS`/`InterfaceS`/`FunctionS` deleted.
- Deleted files: `postparsing/rune_type_solver.rs`, `postparsing/identifiability_solver.rs`, `postparsing/test/post_parsing_rule_tests.rs`, `postparsing/test/post_parsing_parameters_tests.rs`, `postparsing/test/after_regions_error_tests.rs`, `postparsing/test/post_parser_error_humanizer_tests.rs`, `parsing/tests/rules/coord_rule_tests.rs`. Deleted fns `predict_rune_types`, `check_identifiability` from `post_parser.rs`. Deleted error variants `IdentifyingRunesIncompleteS`, `RuneExplicitTypeConflictS`.
- `postparsing`, `scout_arena`, and scout-facing helpers re-linked; `#[cfg(any())]` gates removed. Higher_typing + typing + all downstream still unlinked (unchanged).
- `.vale` fixture files renamed `T Ref → T Kind` across builtins and test programs; `as.vale` + `functor1.vale` rewritten to drop retired constructs.

Everything downstream still unlinked and ready for the next slice. Handoff-worthy state.

---

## 👤 User

anything else we should clean out of postparsing or parser?

---

## 🤖 Claude

Let me audit for lingering dead code.

---

## 🤖 Claude

Now clean up remaining Inline/SharednessPT references in mod.rs, rules.rs, and traverse.rs.

---

## 🤖 Claude

Now clean up stale `STUB: onion typing` comments and stale `TEMP` markers in `lib.rs`, `function_scout.rs`, `post_parser.rs`.

---

## 🤖 Claude

Cleanup pass complete. Summary of what got nuked beyond the main slice:

**Parser AST**
- Deleted `ITemplexPT::Inline` + `InlinePT` struct (retired at parser slice, zero remaining producers)
- Deleted `ITemplexPT::Point` + `PointPT` struct (zero construction sites in the entire crate)
- Deleted `SharednessPT` struct (zero construction sites)

**Postparse AST**
- Deleted `SimpleParameterS` struct (zero construction sites)
- Deleted `IFunctionAttributeS::Additive` variant + `AdditiveS` struct (retired at parser slice — `IAttributeP::AdditiveAttribute` doesn't exist)

**Documentation/comments**
- Cleaned stale `// STUB: onion typing —` comments in `lib.rs`, `function_scout.rs`, `post_parser.rs` (the retirements are done)
- Deleted `// TEMP:` block comments about scout_arena gating (gates are gone)
- Deleted stale references to `parsing::ast::OwnershipP`
- Deleted the `CantOwnership*` guard-restoration TODO

**Test files touched**: `parsing/tests/traverse.rs`, `postparsing/test/traverse.rs` (removed dead arms + node variants for the deleted types).

**Test suite: still 489 / 0 / 1**. Nothing silently dropped.

## What I deliberately left standing

- **`MacroCallP`** — still active for `#DeriveStructDrop`-style attribute syntax on citizens.
- **`TypedRunePT`** — still constructed by parser (`T Kind` style typed runes).
- **`AnonymousRunePT`** — still constructed for `_` runes.
- **`CopyPrimSE`** — still emitted by `expression_scout` for the current primitive-copy path (comment there flags it for eventual removal once typing auto-inserts, but that's a typing-side concern).
- **`SharednessP::Single` / `Shared`** — both live at parser (surface `share` keyword on citizen decl).
- **`PureS`** — live (parser has `PureAttribute`).
- **`MIGALLOW` / `SPORK` comments** — Scala-migration bookkeeping markers, informational only; not blocking anything. Could sweep in a broader "kill Scala breadcrumbs" pass but not onion-scoped.

Working tree ready for `fire commit` whenever you want the state saved.

---

## 👤 User

kill pure please

---

## 🤖 Claude

Now delete parser AST attribute + lexer variant + keywords.

---

## 🤖 Claude

You've hit your session limit · resets 9:20pm (America/New_York)

---

