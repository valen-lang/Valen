# Onion typing — scouting findings

Read `vcoord-handoff.md` first for the design; this doc synthesizes what a 10-investigator + completeness-critic scouting mission (2026-07-02, workflow `wf_1ee008c0-e40`) found about **what the codebase actually needs to change** to land it. Findings organized by subsystem, followed by cross-cutting landmines, open design questions, and gaps the critic flagged.

**Blast radius (rough numbers surfaced across investigations):**

- ~40 files with pattern-match sites on `ITemplataT::Kind` / `CoordT` fields — highest densities in `compiler_solver.rs` (63 refs), `compiler_error_humanizer.rs` (49), `templata_compiler.rs` (39), `rune_type_solver.rs` (39). (Formerly also `higher_typing_pass.rs` (29 refs) — since retired.)
- ~200 pattern-match sites on the `(ownership, region, kind)` triple.
- ~400 region-threading argument sites disappear from typing (functions taking `context_region: RegionT`).
- ~120 `RegionT { region: IRegionT::Default }` literals evaporate.
- ~150 `& overload` functions in `arith.vale` + `logic.vale` may retire.
- ~13 weak-test fixture .vale files use `&&Muta` (surface flip landmine).
- ~120 testvm sites carry `OwnershipH`/`LocationH` through.
- AliasTE/AliasIE/AliasH cascade footprint: ~104 net LOC added in the recent commit, ~8 downstream stops to retire together.

---

## 1. Type system core (`KindT`, `CoordT`, `ITemplataT`, interner)

### 1.1 `KindT` gains four ref variants — interner plumbing is sequence-critical

Under onion typing (per handoff §What(1)), `KindT` gains `BorrowRef` / `HeapOwnRef` / `ShareRef` / `WeakRef`. Each is a new interned payload type. The interner discipline (see @TFITCX and @WVSBIZ arcana) requires, per variant:

- Payload struct with `_must_intern: MustIntern` witness.
- `ValT` companion (or reuse-self per `types.rs:418-424` pattern).
- `InternedKindPayloadValT` / `InternedKindPayloadT` variant.
- Intern method in `typing_interner.rs` following the wrapper macro conventions at lines 55-96.
- Intern lookup arm in `intern_kind_payload` at line 425+.

**`BorrowRefT` is the tricky one** — its Val key includes `inner: KindT` (itself a Copy enum of interner refs) AND `region: RegionT`. Intern uniqueness depends on inner's pointer identity + region enum tag. `HeapOwnRefT` / `ShareRefT` / `WeakRefT` are simpler (single `inner: KindT` field).

The critic explicitly flagged this as a Slice-A companion: 4 payload structs + entries in the payload-dispatch enums + intern methods (~200 lines added to `typing_interner.rs`) must land alongside or slightly before the Kind variants themselves. Sequencing this wrong means silent intern-cache misses and re-interning of duplicates.

### 1.2 `share_flavored: bool` on `StructTT` / `InterfaceTT` — synchronized change

Per handoff §Q2, share-ness is intrinsic to the citizen. Today it lives externally in `coutputs.type_name_to_mutability` and `StructDefinitionT.sharedness`; it's looked up via `Compiler::get_sharedness`. Under onion it moves onto `StructTT` (`types.rs:370-395`) and `InterfaceTT`.

**Interner-key drift landmine (critic):** `StructTTValT` (`types.rs:379-381`) is `{ id: IdT }` today with derived `Hash` + `Eq`. The Val must exactly match the interned struct's key fields for lookup to succeed. Adding `share_flavored` to `StructTT` without adding it to `StructTTValT` causes lookup misses. Must be a synchronized change across both types + the interner. Same for `InterfaceTT`.

Two systems retire in tandem:

- Three side maps in the instantiator (`struct_to_sharedness`, `interface_to_sharedness`, `impl_to_sharedness` at `instantiator.rs:215-245`) — populated in `translate_struct_definition` :1112-1115, read by `get_sharedness` at :2232. All go away.
- `compiler.rs:1697-1716 get_sharedness` — the citizen arm becomes a struct-field read. **Open question (critic):** `StrT` is a share-flavored leaf primitive, not a citizen — where does its `share_flavored=true` bit live? Enum arm hardcode, or a helper?

### 1.3 `CoordT::new` validity checks migrate to interner-side per-variant validity

`CoordT::new` (`types/types.rs:70-89`) today panics on (Share, primitive) and (Share, OverloadSet) — a construction-time check tied to the flat ownership tag. Under onion, the validity table (handoff §What(3)) replaces this atomically at the interner-side, per new variant:

| Layer / bare | non-share citizen | share citizen |
|---|---|---|
| bare (value) | ✓ | ✗ |
| `HeapOwnRef` | ✓ | ✓ |
| `ShareRef` | ✗ | ✓ |
| `BorrowRef` | ✓ | ✓ |
| `WeakRef` | ✗ | ✓ |

Every intermediate state that still constructs `CoordT` trips today's checks. Either use the migration alias `type Coord = Kind` (handoff §Q4 tactical) or delete `CoordT` construction in the same slice that adds `BorrowRef`.

### 1.4 `ITemplataT::Kind` collapses into `ITemplataT::Kind`

Under onion, the `Coord` / `Kind` variant split in `ITemplataT`, `ITemplataI`, and `ITemplataType` collapses to a single `Kind` variant. Every match arm that pattern-matches `ITemplataT::Kind(ct) => ct.coord` / `ITemplataT::Kind(kt) => kt.kind` in the resolver collapses to a single arm — no more ownership+region unpacking.

- `CoerceToCoordSR` + `coerce_kind_lookup_to_coord` + `coerce_kind_template_lookup_to_coord` + the Kind→Coord "will convert, so is fine" auto-legal in `rune_type_solver.rs:486` — all become no-ops and delete.
- Surface-syntax `Ref` (→`ITypePR::CoordType`) and `Kind` (→`ITypePR::KindType`) merge (architect's call which keyword survives).
- `expect_coord_templata` (`templata.rs:45`) survives semantically as a smart-view (or is renamed `expect_kind_templata`).

**Landmine (critic):** If migration alias is used, `HashMap<CoordT, X>` becomes `HashMap<KindT, X>` transparently. But if `ITemplataT::Kind(x) == ITemplataT::Kind(y)` merges its equality semantics, callers comparing `ITemplataT` values in maps or in the identifiability solver may silently change behavior. Sequencing matters.

### 1.5 `ISubKindTT` / `ISuperKindTT` / `ICitizenTT` — a design decision blocks isa work

The three wrapper enums at `types.rs:262-368` are hard-coded to `Struct`/`Interface`/`KindPlaceholder`. Every isa check flows through `ISubKindTT::try_from(kind)` / `ISuperKindTT::try_from(kind)`:

- `compiler_solver.rs:820-826` (CallSiteCoordIsa)
- `compiler_solver.rs:864-871` (DefinitionCoordIsa)
- `infer_compiler.rs:799-811` (resolve_impl_conclusion)
- `convert_helper.rs:84` (convert() upcast)

**Open design question (from investigators + critic):** does `BorrowRef(Struct(Ship))` isa `BorrowRef(Interface(Vehicle))` when `Ship` isa `Vehicle`? Handoff doesn't say. If yes, `ISubKindTT` / `ISuperKindTT` gain `BorrowRef` arms with recursive variance logic. If no, all bare-use of a subclass through a `&SuperInterface` param fails typechecking. This decision blocks isa-related rewrites.

---

## 2. Solver

The solver is the highest-density Coord-touching area (63 refs in `compiler_solver.rs` alone). Findings split six ways.

### 2.1 Coord-specific rules — three fates

**(1) The isa/send family (`CoordSendSR` + `CallSiteCoordIsaSR` + `DefinitionCoordIsaSR`) survives structurally; bodies collapse.**

Today each rule extracts `coord.kind` and passes it to `is_parent` / `ISubKindTT` / `ISuperKindTT` — a "peel the ownership layer, look at the citizen" op. Under onion that becomes "peel outer `BorrowRef` / `HeapOwnRef` / `ShareRef` / `WeakRef` layer(s) to find the citizen Kind, then run the same `is_parent` check." Rename to `KindSend` / `KindIsa`; the ancestor-lookup mechanism (@SAIRFU) stays.

- `CoordSendSR` handler: `compiler_solver.rs:920-1036`
- `DefinitionCoordIsaSR`: `compiler_solver.rs:849-885`
- `CallSiteCoordIsaSR`: `compiler_solver.rs:788-847`

The **two coercion-accept else-branches from Phase-2-partial are the first code deleted** (per handoff §What(4)):
- `CoordSendSR:978-1029` — the `sender_already`/`should_conclude` gate matching `(Borrow,Share)|(Own,Borrow)|(Borrow,Own)`.
- `CallSiteCoordIsa:806-818` — the `sub_coord.kind == super_coord.kind && matches!(ownership, ...)` arm.

**Load-bearing insight:** `CoordSendSR` is emitted DYNAMICALLY at solve time from `InitialSend` records (`infer_compiler.rs:226` via `function_compiler_solving_layer.rs:770 assemble_initial_sends_from_args`) — one per call-site arg. There is no static grep of "places that emit CoordSend"; the answer is literally every function call. `CoordSendSR` **self-replaces** into `CallSiteCoordIsaSR` at `compiler_solver.rs:930/962` via `commit_step`'s new-rules slot when the sender-or-receiver kind participates in an inheritance hierarchy (the @SAIRFU "Send rule short-circuits or replaces itself with Impl" pattern). This dynamic rewrite must be preserved verbatim under onion. `SROACSD` asserts (`compiler_solver.rs:254`) that `CallSiteCoordIsaSR` and `DefinitionCoordIsaSR` never coexist statically.

**(2) `CoordComponentsSR` is a real problem.**

Two directions: `(Coord → Ownership + Kind)` at line 649, and `(Ownership + Kind → Coord)` at line 620 with a share-flavored override at line 632 (`"VCOORD: this should go away"`). Emitted by `rule_scout.rs:296` for surface `Components` syntax and — critically — by `anonymous_interface_macro.rs:786+813` to derive-a-struct-coord-from-an-interface-coord-preserving-ownership. Under onion, "preserving ownership" means "preserving the ref-layer onion" — the rule needs to be **replaced, not renamed**. Two candidate directions:

- `OwnershipTemplataType` becomes a "ref-shape" templata (an enum-like tag with regions for BorrowRef).
- The anon-interface macro switches to pattern-based reconstruction that peels/rewraps the same ref variant.

Load-bearing: `anonymous_interface_macro.rs:786`/`:813` share `self_ownership_rune` (line 816) with the destructuring interface rule (line 789), making the sameness structural. Any replacement must preserve this structural sameness.

**(3) `KindComponentsSR` is dead code.**

Both emit site (`rule_scout.rs:308`) and handler (`compiler_solver.rs:604`) carry `"VCOORD: retire this"` comments. Zero components, no work, just asserts the rune is a Kind. Delete outright.

### 2.2 `AugmentSR` — templex "ownership prefix" rule

`AugmentSR` binds "outer coord = augment inner coord with this `OwnershipP`" — the type-side spelling of `&T`, `^T`, `weak T`, `share T` sugar.

- **5 emission sites** — 2 postparser (`templex_scout.rs:285`), 3 anonymous-interface macro.
- **3 solve-time arms** — DIR1 outer→inner (`compiler_solver.rs:1181`), DIR2 inner→outer (`:1231`), DIR3 the complex_solve receiver-inference contribution (`:449`).
- **Recently-landed check** — Shared-arm reject-on-contradiction (DIR1 Shared sub-arm at `:1210`), mirroring the Single sub-arm.

**Under onion, "augment" is a Kind-wrapping transform, not a Coord transform:**

- DIR2 → pure structural wrap indexed by `OwnershipP`. `get_sharedness` branch and `CantShareMutable` disappear (interner enforces validity).
- DIR1 → structural peel-and-check: `augment_ownership=Borrow` must see outer as `BorrowRef(_,_)`; else structural mismatch. Shared/Single disambiguation dies with `OwnershipT`.
- DIR3 → "wrap `receiver_instantiation_kind` in the target ref variant"; `ReceivingDifferentOwnerships` (line 469) goes away.

**Landmines:**

- `AugmentSR.ownership: Option<OwnershipP>` — `None` (pass-through region-only) is emitted only by `templex_scout` for Interpreted forms without an ownership prefix. DIR3 blindly `.expect()`s Some (`compiler_solver.rs:450`), so any None reaching complex_solve panics. Under onion, "pass-through" and "ref-wrap" become distinct concerns.
- DIR2 silently sets `new_region = IRegionT::Default` (`:1237`), dropping inner's region. Under onion, `BorrowRef` requires a real region; the source must provide it. **Where does the region come from?** Options: (a) add `region_rune` to `AugmentSR`; (b) look up `context_region` from an ambient env at solve time; (c) restrict `BorrowRef`-emitting Augments to explicit-region syntactic forms only.
- `AugmentSR` DIR1 Shared arm was **just added as the Slice-A backstop**. Handoff §preserves the resolver structural-consistency principle even as the specific check migrates. Do NOT delete before the interner-side onion-validity check lands.
- `anonymous_interface_macro.rs:588` drop-bound emits `Augment` with `ownership=Some(Own)`. Under onion, `Own` has two structural spellings: bare Kind (non-share) vs `HeapOwnRef(Kind)` (share). Macro must know share_flavored at emit time or delegate to solver.
- Adjacent parser-level `IExpressionPE::Augment` (value-side `&x`/`^x`/`weak x` in `parser/ast/expressions.rs:411`, lowered by `expression_scout.rs:404` to `LoadAsP::{LoadAsBorrow|Move|LoadAsWeak}`) is a distinct codepath — panics on `OwnershipP::Share` / `OwnershipP::Live` today, needs real lowering under onion (Share becomes ShareRef wrap / RC bump).

### 2.3 `is_type_convertible` + coercion-accept patches

`is_type_convertible` (`templata_compiler.rs:1143`) is the coercion-legality gatekeeper for `params_match` and return/mutate coerce-legality. The Phase-2-partial work broadened it for (Own,Borrow), (Borrow,Own), (Borrow,Share) uniformly at `:1184-1213`.

Three parallel coercion-accept patches exist purely to keep the resolver from concluding sender=receiver in those legal-mismatch cases:

- CoordSendSR else-branch at `compiler_solver.rs:978-1005`
- CallSiteCoordIsaSR at `:806-819`
- AugmentSR DIR1 Shared-arm at `:1197-1214`

**Under onion these retire together.** A pair like (Borrow,Share) becomes structurally `(BorrowRef(K), ShareRef(K))` — different Kinds, never spuriously unify. `is_type_convertible` degenerates to a **pure kind-shape / isa check** (Never / equality / isa-ancestor); no ownership pairs to permit.

**Landmines:**

- `is_type_convertible`'s region check at `:1180-1182` rejects any region mismatch outright — **less permissive than row 4 of the coercion table**, which permits BorrowRef region unification (r vs r'). Onion port must relax the region check to "compatible regions on the outermost BorrowRef layer" rather than blanket reject.
- The (Borrow, Weak) row is a return-false today (`:1200`), marked unreachable. Under onion, `BorrowRef(WeakRef(SC), r) → WeakRef(SC)` is coercion table row (a) — implemented via the weak blanket `implicit_clone(&weak T) weak T`. Confirm the arm's onion fate.
- `resolve_function`'s outer-Err at `convert_helper.rs:121` is documented `.expect("...unreachable from Vale source")` — checkpoint learning #5. If onion introduces new failure modes (heap-autoderef descoped → error), outer Err becomes reachable in ways the `.expect` masks.

### 2.4 `complex_solve` receiver consensus

`compiler_solver.rs:377-487` — `complex_solve` walks CoordSend/CallSiteCoordIsa sender→receiver edges to compute common-ancestor citizens for interface-typed receivers, and merges senders' ownership into a single receiver ownership.

- Line 456: `CoordT::new(coord.ownership, RegionT { region: IRegionT::Default }, receiver_instantiation_kind)`.
- Line 463-471: `HashSet<OwnershipT>` merge, error variant `ReceivingDifferentOwnerships`.

Under onion this becomes "single ref-layer variant across senders" — the same pattern, different check. `ReceivingDifferentOwnerships` renames to `ReceivingDifferentRefShapes`. The `Augment` override at 449-452 uses `evaluate_ownership` on `augment.ownership.expect` — this couples Augment's `OwnershipP` surface to the propagated inner Kind, needs restructuring for the four ref variants.

**Open question:** under onion, does DIR3 (complex_solve receiver-inference) still need to run? With no ownership axis and no `ReceivingDifferentOwnerships` deduplication, the receiver is either exact-match resolved or errors — the complex-solve orchestration may simplify to "pick the receiver instantiation kind and wrap once" without the ownership set-cardinality dance.

**`get_kind_equivalent_runes` graph** (`rule_scout.rs:397-424`) marks `CoordComponents`' result_rune ↔ kind_rune and `Augment`'s result_rune ↔ inner_rune as kind-equivalent for common-ancestor purposes. Under onion, "same kind" becomes ambiguous: does it mean "same base citizen after peeling ALL ref layers" (deep peel) or "same immediate wrapped kind" (shallow peel one)? Semantically load-bearing.

### 2.5 Runtime templata & rune-type solver

The full `ITemplataT::Ownership` variant + `OwnershipTemplataType` + `OwnershipLiteralSL` + `evaluate_ownership` + `humanize_ownership` form **one closed axis to rename/retype together** — piecemeal migration leaves `CoordComponents` solving `ownership_rune` with an untyped conclusion.

- `OwnershipTemplataT` used at solver:1547 becomes dead.
- `OwnershipLiteralSL` (`rules.rs:339`) becomes dead.
- `evaluate_ownership` (`conversions.rs:16-29`) dies; `OwnershipP::Live` panic at `:23` needs a dead-path confirmation before deletion.
- `humanize_ownership` (`post_parser_error_humanizer.rs:377`) has `OwnershipP::Live => panic!(...)` — Live is a phantom variant, audit before deleting.

`CoerceToCoordSR` (`rules.rs`, solver `:1106`) becomes identity as `Coord = Kind`. Its handler at `compiler_solver.rs:1111-1141` forces `OwnershipT::Own | OwnershipT::Share` only — retires with `CoordT`. Kind→Coord promotion goes away.

`rune_type_solver.rs:486` treats `Kind→Coord` as freely convertible — a fossil pre-onion coherent-collapse hint. Its arm's disappearance is a symptom, not a cause; deleting it triggers `panic!("lookup_rune_type Templata FoundTemplataDidntMatchExpectedType not yet implemented")` at line 497. (Update: the `higher_typing_pass` was retired outright, so the "simplify in the same commit" concern is moot — the whole pass and its `explicify_lookups` disappeared.)

### 2.6 Overload resolver — dispatch redesign

Today's model is **not** namespace-based (contra handoff dispatch mission). `get_candidate_banners` (`overload_resolver.rs:155-182`) collects from:

- (a) calling env
- (b) EVERY param's struct/interface/placeholder outer env via `get_param_environments`
- (c) placeholder-impl super-interface envs (@BDPFWDZ)
- (d) `extra_envs_to_look_in` (documented empirically-dead)

This already resembles "every arg's namespace" — but the per-file/parameter-mentions-T namespace-membership rule doesn't exist; today's env chain returns anything defined in an env chain reachable from the type's outer env, plus everything visible in the calling function's parent chain.

**No Self-firstness** in collection (correct). But there IS specificity/tiebreaking in `narrow_down_callable_overloads` (`overload_resolver.rs:662`) — a two-stage filter that prefers exact-match over conversion-match per-param, then splits normal-vs-FunctionBound candidates and prefers the shortest-id-steps bound, panicking on non-clear-winners at `:743`. All three layers violate handoff's "no specificity, no phases, no tiebreakers, single-candidate wins or ambiguity errors" rule.

**Open clarification (handoff-flagged):** does "mentions T in a parameter" count `&Ship` as mentioning Ship? Today's `get_param_environments` treats `Coord { kind: Struct(Ship), ownership: Borrow }` as visiting Ship's env — so today's behavior is "yes." Confirm design intent matches before ripping.

**Landmine (critic):** `AttemptedCandidate` carries a `prototype: &'t PrototypeT` whose params were `CoordT`. Under onion these become ref-onion Kinds. `AttemptedCandidate` `PartialEq`/`Hash` depends on prototype identity (ptr-eq) — refactoring mid-arc invalidates the interner-keyed dedup at `overload_resolver.rs:671-679`.

---

## 3. Templata / rune types

Kind and Coord collapse into a single templata at all three IR layers (T / I / H). Concretely:

- `ITemplataT::Kind` / `ITemplataT::Kind` variants collapse to single `Kind` variant. Same for `ITemplataI` and `ITemplataType`.
- Every match arm pattern-matching on the two variants collapses.
- `CoerceToCoordSR` + `coerce_kind_lookup_to_coord` + `coerce_kind_template_lookup_to_coord` retire.
- Surface `Ref` / `Kind` keywords merge (architect's call which survives).
- Generic bounds like `where clone(&T) T` change signature — today `CoordSendSR` and `ResolveSR` conclude `ITemplataT::Kind(...)`; under onion they conclude `ITemplataT::Kind(...)` where the Kind may include a `BorrowRef` layer.
- **AtomSP `coord_rune: Option<RuneUsage>`** field is threaded through the entire scout→postparse→typing pipeline. Under onion this becomes `kind_rune` semantically. Rename vs semantic-shift-without-rename is a decision (50+ sites; rename is bigger diff, more grep-able for the future reader).

---

## 4. Instantiator (T→I)

Two layers of walkers, both cascade:

### 4.1 Layer A — T-side substitution (in `typing/templata_compiler.rs`, not the instantiator)

**Attribution matters:** `substitute_templatas_in_coord` (`:389`) + `substitute_templatas_in_kind` (`:426`) + `substitute_templatas_in_struct` (`:504`) + `substitute_templatas_in_interface`. These execute the "Borrow+share-kind preserved distinct" composition (`:400-423`) — the just-landed Phase-2 coherent-collapse fix.

Under onion, the composition table dissolves entirely. Substitution walks `KindT` and returns `KindT`; substituting `T` with `Struct(Ship)` inside `BorrowRef(T)` yields `BorrowRef(Struct(Ship))`; substituting `T` with `BorrowRef(Struct(Ship), r_inner)` inside `BorrowRef(T)` yields `BorrowRef(BorrowRef(Struct(Ship), r_inner), r_outer)` — the double-borrow the trip's blanket needs, produced structurally with no table.

The `unreachable!` arm at `:418` catches "Weak-on-substituting-side" as degenerate — under onion these become valid nested `WeakRef(...)` compositions. Review, not just retag.

### 4.2 Layer B — T→I boundary translation (`instantiating/instantiator.rs`)

`translate_coord` (`:2170`) + `translate_kind` (`:2353`) + `translate_struct` / `translate_interface` / `translate_citizen` / `translate_super_kind` / `translate_static_sized_array` / `translate_runtime_sized_array` / `translate_prototype` / `translate_struct_member` / `translate_local_variable` / `translate_parameter` / `translate_reference_local_variable` / `translate_addressible_local_variable`.

**This is where the handoff's "sites 4-5" live:**
- **Site 4** = `compose_ownerships` (`:2020`, called only from `translate_coord`'s KindPlaceholder branch `:2183`).
- **Site 5** = `compose_ownerships_second` (`:2058`, called only from `translate_ref_expr::LetAndLend` at `:1390`).

Both hard-code `(Borrow, MutableShare) → MutableShare` and `(Own, MutableShare) → MutableShare` — the exact "Borrow-of-share collapses to MutableShare at T→I boundary" behavior the commit message names.

**Sites 6-7 (deliberately unlanded per commit):**
- `translate_ref_expr::SoftLoad` target-ownership match at `:1930-1956` (the two `// VCOORD: papering over` arms — `(OwnershipT::Share, OwnershipI::Own) => Own` at `:1935` and `(OwnershipT::Share, OwnershipI::MutableBorrow) => MutableBorrow` at `:1940`).
- `translate_ref_expr::Alias` reflavor at `:1910-1929` (the entire `AliasIE` emitter — only one in the tree).

Together these are the "Backend borrow-of-share dispatch" deferred to sub-slice-4b. **Under onion, all four sites become obsolete together** because the composition/collapse table degenerates: `BorrowRef(ShareRef(SC))` is a legitimate Kind, not a thing to collapse.

`compose_ownerships` (13 arms) + `compose_ownerships_second` (13 arms) — pure OwnershipT×OwnershipI table — bulk cascade delete.

### 4.3 Landmines and gotchas in the instantiator

- **`assemble_placeholder_map_inner`** (`instantiator.rs:873-896`) panics `unimplemented arm` at `:892` if generic bound params gain ref-wrapped placeholders (e.g. `&T`). Slice A's `KindT::BorrowRef` insertion must not touch bound-param sites, OR this extractor must learn to peel through ref layers to reach the placeholder. Pick one before the tracer starts constructing double-borrows.
- **`translate_coord`'s KindPlaceholder branch** (`:2178-2189`) currently panics `Unimplemented: translate_coord KindPlaceholder->Kind` at `:2186`. Under onion this branch becomes the ONLY branch — every substitution produces a Kind. The panic path is where onion typing plants its first stake at the I-IR boundary.
- **`translate_ownership`** (`:2011-2018`) already panics on Weak (`translate_ownership: WeakT vimpl`). Under onion, Weak becomes a `WeakRef` wrap at construction, not an ownership tag translation. This function retires.
- **`CoordI::void()`** (`types.rs:56-59`) is a stub (`panic!`). Under onion it becomes `KindIT::VoidIT(VoidIT{})` directly at every call site. Search for `CoordI::void` usages to enumerate cascade points.
- **Directory name landmine:** the instantiator lives in `FrontendRust/src/instantiating/` (present participle), not `instantiator/`. Grepping for the wrong path returns empty.

### 4.4 Every I-IR `CoordI`-carrying field that dissolves

- `ast/types.rs:65-86` — the `CoordI` struct itself + interner guard against `Share+primitive` at `:78-83`.
- `ast/citizens.rs:81, 89` — `ReferenceMemberTypeI.reference`, `AddressMemberTypeI.reference`.
- `ast/ast.rs:152, 232, 304, 312, 386, 396, 407, 417` — `ParameterI.tyype`, `FunctionHeaderI.return_type`, Prototype return/params, four `collapsed_coord` fields on local-variable structs, `ILocalVariableI::collapsed_coord` accessor at `:372`.
- `ast/expressions.rs` — 30+ `result: CoordI` / `element_type: CoordI` / `target_type: CoordI` / `result_result_type: CoordI` / `coord: CoordI` / `result_reference: CoordI` / `result_opt_borrow_type: CoordI` fields on IE nodes.
- **`OwnershipI`-typed fields:** `AliasIE.target_ownership`, `SoftLoadIE.target_ownership`, `LetAndLendIE.target_ownership` — all three dissolve entirely once wrapping is expressed structurally.

**Collector and humanizer:** `collector.rs:52-59` (`all_in_coord` / `all_in_kind`) + `:142-155` (`visit_coord`, `visit_kind`) — two-function walker; under onion the Coord walker retires and `visit_kind` grows four recursive arms. `instantiated_humanizer.rs:39-66` — `humanize_coord` renders `OwnershipI::Own` as `""`, `MutableBorrow` as `"&"`, `Weak` as `"weak&"`; under onion `humanize_coord` dissolves and `humanize_kind` grows recursive arms.

---

## 5. Hammer + H-IR + Backend FFI

Today the H-IR mirrors the flat-CoordT/Ownership model:

```rust
CoordH { ownership: OwnershipH, location: LocationH, kind: KindHT, _sealed }
```

- `OwnershipH` is post-cut 4-way: `OwnH` / `MutableBorrowH` / `MutableShareH` / `WeakH` (final_ast/types.rs:20-51).
- `LocationH::{InlineH, YonderH}` is a pre-Q1 concept, computed in `translate_coord` from (ownership, kind) via a hard-coded table (`type_hammer.rs:74-82`) and re-decided in every LocalLoadH-style loader (`instructions.rs:100-107`, `load_hammer.rs:92/133/189/229/270`).
- Backend FFI `Ownership` (`metal_cache.rs:78-85`) stays 6-way (adds `ImmutableBorrow`/`ImmutableShare` — never emitted from Rust; test-only site at `metal_cache.rs:1448`).
- `RegionH` at `ast.rs:27` is a **vestigial unit struct** imported only by `type_hammer.rs`. H-IR is region-less structurally. Only `CoordTemplataI.region` (`ast/templata.rs:93`) carries a vestigial `RegionT` at the instantiator-templata level.

### 5.1 Under onion, `CoordH` dissolves; `KindHT` gains the four ref variants

Concrete change points:

- (a) `CoordH::new`'s validity table (`types.rs:32-51`) becomes a Kind-layer-shape check on the hammer_interner.
- (b) `ExpressionH::result_type` — ~50 arms each fabricating `CoordH::new(OwnershipH::_, LocationH::_, KindHT::_)` — every one collapses to a single `KindHT`.
- (c) 4 IR nodes carry `target_ownership: OwnershipH` (LocalLoadH:417, RuntimeSizedArrayLoadH:495, StaticSizedArrayLoadH:510, plus MemberLoadH's result_type) — replaced by the load's onion-shaped result kind.
- (d) `MutabilifyH::result_type` (`instructions.rs:130-145`) pattern-matches on `OwnershipH` and panics on `WeakH`. Backend still consumes it via `cache.expr_mutabilify` at `metal_lowerer.rs:458-463`. **Open question:** does MutabilifyH survive under onion, or was it always a workaround?
- (e) `translate_kind` (`type_hammer.rs:34-59`) gets 4 new arms for `BorrowRef`/`HeapOwnRef`/`ShareRef`/`WeakRef` mirroring T-IR.
- (f) `translate_coord` (`:65-86`) reduces to `translate_kind`.
- (g) `evaluate_ownership` / `evaluate_location` (`conversions.rs:16-29`) die.
- (h) `load_hammer.rs`'s six loaders each independently recompute `LocationH` from (target_ownership, expected.location) — no shared helper. Under onion the recomputation dies but the six sites still need independent auditing. `get_borrowed_location` at `:373` is unimplemented + unreferenced — safe to delete.
- (i) `hammer_tests.rs:94` asserts exact `CoordH` equality on struct member types — 5 pattern-match sites; mechanical rewrite.

### 5.2 Backend FFI boundary

`metal_lowerer::lower_coord_to_reference` (`:282-295`) and `lower_ownership` (`:266-273`) become no-ops once C++ accepts a Kind-only view — this is the **deferred Backend arc**.

**Backend pre-flight blockers** (all VCOORD-marked, pointing at handoff):
- `Backend/src/region/common/primitives.h:27-45` (4 Own asserts on Int/Void/Bool/Float).
- `Backend/src/region/linear/linear.cpp:170-179` (3 Own asserts on Int/Bool/Float).
- `Backend/src/determinism/determinism.cpp:832` — accepts OWN but downstream codegen is Share-designed.

**Rejected option:** handoff explicitly rejects lowering borrow primitives to Own at the metal_lowerer FFI. Do NOT shim borrow-of-primitive → Own at `metal_lowerer.rs:266-295` during the frontend arc — it destroys the flavor info the type system just started tracking.

**Critic-flagged under-coverage:** `Backend/src/metal/types.h` has `Reference` invariants (INLINE ⇒ OWN|MUTABLE_SHARE, etc.) that will need audit; Backend has region translation (`region/common`, `region/linear`, `region/rcimm`, `region/resilient*`) — each region may have its own Ownership assertions. When `primitives.h` Own-asserts drop, the 2nd/3rd cascades likely hit `region/linear` + `region/rcimm` + `region/resilient*` asserts.

### 5.3 testvm + AliasH

- **testvm** has ~120 sites carrying `OwnershipH` / `LocationH` through — biggest lift outside `instructions.rs`.
- **testvm/values.rs:420-434** `ReferenceV::new` calls `CoordH::new` twice as a validity assertion. When `CoordH` goes away the assertion moves onto the `KindHT` construction path (interner-time onion-layer validity).
- **Critic-flagged under-coverage:** testvm heap accounting for share-flavored citizens (`heap.rs:560-579` transmute keyed on `OwnershipH == MutableShareH`) needs explicit onion semantics. Under `BorrowRef(ShareRef(SC))`, does the heap accounting increment RC once or twice? testvm is the ground-truth semantic reference — no investigator enumerated its semantic invariants.
- **`AliasH`** is emitted only from `AliasIE` (which is only from `AliasTE`). Both `metal_lowerer` and testvm silently delegate to the inner today (the delegate-to-inner is a documented deferred loose end). When AliasH deletes, the RC-bump obligation moves to the `implicit_clone(&@T) @T` share-blanket call site.

### 5.4 `OpaqueHT` — kept as landmine

`OpaqueHT` is a `KindHT` variant but `lower_kind` at `metal_lowerer.rs:262` explicitly panics on it (kind_to_extern uses `info.kind` instead). **Open question:** is there a legitimate `BorrowRef(OpaqueHT(_))` (borrow of an extern kind)? If yes, this panic becomes a real path.

---

## 6. Expression compiler / `convert()`

The coercion pipeline is small in surface (one `convert()`, one `convert_exprs()`, 7 threaded callers, one `wrap_in_implicit_clone()`, `soft_load`/`borrow_soft_load`/`make_temporary_local_defer` as materialize helpers) but the **decisions** are spread across three axes today:

1. **Tag-shape checks in `convert()`** switching on `(source_ownership, target_ownership)` pairs (8-arm match at `convert_helper.rs:106-185`).
2. **`is_primitive` gate on bare-use** in `evaluate_lookup_for_load` / `coerce_to_reference_expression` — redirects Own-non-primitive through `borrow_soft_load` and Own-primitive through `wrap_in_implicit_clone`.
3. **`is_type_convertible` gate** (called at 5 sites: Return / Mutate / LocalMutate / function body / overload_resolver).

Under onion, all three collapse into a single mechanism: **peel/wrap BorrowRef layers structurally, then probe implicit_clone at the target site (rows 1/2/3/a/b/c).**

### 6.1 Concrete replacements

- `convert()` `(Borrow, Own)` arm → probe rows 1+2 (`implicit_clone(&P) P` for primitive; `implicit_clone(&NC) NC` for user-defined).
- `convert()` `(Borrow, Share)` `AliasTE` arm → probe row 3 (share blanket `implicit_clone<T>(&@T) @T`).
- `convert()` `(Own, Borrow)` `make_temporary_local_defer` arm → structural wrap `BorrowRef` (still needs `LetAndLend` + deferred-drop plumbing).
- `convert()` `(Own,Own)` & `(Borrow,Borrow)` & `(Share,Share)` & `(Weak,Weak)` identity arms → structural row 4/5/6 pass-through with region unification for row 4.
- **`wrap_in_implicit_clone` retires entirely.** Its callsite in `evaluate_lookup_for_load` / `coerce_to_reference_expression` becomes "wrap in `BorrowRef` via bare-use, then let `convert()` probe at target."
- **`borrow_soft_load` + `get_borrow_ownership`** (`local_helper.rs:211-233`) together encode "which `OwnershipT` tag to stamp on a SoftLoad" — under onion that's uniformly "wrap in `BorrowRef(inner)`" and `get_borrow_ownership` disappears. The `KindT::Int/Bool/Float/Str/Void → Share` arms are the **Slice-5 landmine** the checkpoint explicitly named — cascaded 66 regressions when tried alone. This is the shape of the cascade the arc will hit.
- **`soft_load`'s four-way `OwnershipT × LoadAsP` match** dispatches to `SoftLoadTE` with `target_ownership: OwnershipT::{Borrow,Weak}`. Under onion the `target_ownership` field on `SoftLoadTE` gets replaced by "which ref layer to wrap the inner kind in" (`BorrowRef` vs `WeakRef`).
- **`LoadAsP`** itself (parse-side enum `{Move, LoadAsBorrow, LoadAsWeak, Use}`) **stays** — it encodes source-syntax user intent (`^x`, `&x`, `&&x` weak-form, bare `x`) and is orthogonal to how the type system represents pointers. Only its Weak and Borrow arms route to `WeakRef`/`BorrowRef` wrapping instead of `OwnershipT::Weak`/`Borrow` stamping.

### 6.2 Landmines

- **`wrap_in_implicit_clone` has the same `Err`-payload-discarding landmine** as pre-Phase-2 `convert()` (checkpoint note 6): `.map_err(|fff| CouldntFindFunctionToCallT { fff })` at `expression_compiler.rs:465-468`. Cleanest: retire it in one commit alongside converting the two call sites to `borrow_soft_load`.
- **`.expect("convert() in pattern position returned NoImplicitCloneDefinedT")` at `pattern_compiler.rs:241-242`** is a KNOWN unhandled path — the ignored `user_defined_implicit_clone_allows_bare_use_of_struct` test at `compiler_tests.rs:4888` documents `let s2 = s;` blocked pending this thread-through. Any onion slice touching let-binding needs to fix this expect at the same time (make `infer_and_translate_pattern` return `Result`).
- **`SoftLoadTE.target_ownership: OwnershipT`** (`ast/expressions.rs:1327-1345`) is READ downstream by instantiator + hammer + backend + testvm. Changing its shape is a whole-IR-stack ripple.
- **IfTE common-ancestor result-coord construction** (`expression_compiler.rs:1163`) hard-codes `ownership` from the else branch. Under onion, both branches must produce the same onion shape.
- **`borrow_soft_load` usage** at `pattern_compiler.rs:333, 565` and `expression_compiler.rs:266, 294, 350` (closure struct construction), `423` (coerce Address branch) — all tag `OwnershipT::Borrow` via `get_borrow_ownership`. Migrating means every site's returned Coord shape changes from `Borrow + K` to `BorrowRef(K)` — downstream consumers (assert `coord.kind` matches at `:345, 355, 353`) need updating to peel the layer.

### 6.3 Open questions

- Under onion, does bare-use of an already-BorrowRef local pass through as-is per Q3, or does it soft-load through some new node? Today `soft_load(Own+K, LoadAsP::Use)` unstackifies + emits `Unlet` — if bare-use of `BorrowRef(K)` is pass-through, does it still `mark_unstackified`? (`local_helper.rs:195` Use arm on Borrow source is a plain SoftLoad no-op with target=source.ownership, no unstackify.)
- Handoff §Coercion §row-c share-autoderef `implicit_clone(&@T) &T` peels a `ShareRef` layer — but eligibility rule §Q5 says "no blankets that peel a BorrowRef layer." A peel of `ShareRef` via row (c) IS allowed. Is that asymmetry intentional? (Verify at implementation time.)
- Does `struct_drop_macro` still exist as a synthesized function under onion, or does the drop-move-only rule + intrinsic `KindT::HeapOwnRef` structure make it unnecessary?
- `weak_alias` (`expression_compiler.rs:2152-2178`) today fires when kind is Struct/Interface with `weakable=true` — those can be non-share. **Handoff validity table says WeakRef on non-share is ✗.** Weakable-struct-non-share tests exist today. Reconcile.

---

## 7. Region subsystem

Region today is a **token, not a system.** `RegionT { region: IRegionT }` (`types/types.rs:49-59`) has exactly two variants (`Default`, `Iso`); **only `Default` is ever constructed in the compiler proper** (120 `RegionT { region: IRegionT::Default }` literals). `Iso` appears only in the instantiated_humanizer stringifier.

Every Coord, every `context_region`, every `default_region`, every `RawArrayNameT::self_region`, every rune-based region flow is threaded through the pipeline but structurally observable only in two places:

1. `is_type_convertible` (`templata_compiler.rs:1180`) rejects mismatch.
2. `substitute_templatas_in_coord` (`templata_compiler.rs:400-420`) propagates outer region.

Both compare `IRegionT::Default == IRegionT::Default` → always true.

### 7.1 Under onion — regions ride only on `BorrowRefT`

- Delete `RegionT`/`IRegionT` from every non-Borrow layer.
- Every `default_region()` call site, `context_region` parameter, `nenv.default_region()` threading, `RawArrayNameT.self_region`, `ExportNameT.region`, function-env `default_region`, closure-env `default_region` become unreachable except at `BorrowRef` construction.
- `is_type_convertible`'s `if source_region != target_region { return false }` becomes structural (regions unify at each `BorrowRef` layer).

**Estimated delete/simplify:** ~400+ region-threading argument sites disappear from typing (functions taking `context_region: RegionT` become argless), ~120 `RegionT { region: IRegionT::Default }` literals evaporate.

### 7.2 Instantiator + H-IR

- The instantiator T→I translation **already collapses regions to `IRegionT::Default` unconditionally** (`instantiator.rs:2172-2184, 2227`) — the region survives in `CoordTemplataI` (`ast/templata.rs:93`) but the underlying `CoordI` has no region field. Only the templata wrapper carries the vestigial `RegionT`. Tiny cleanup.
- **`RegionH` is a placeholder unit struct** (`final_ast/ast.rs:27`) — never used anywhere. H-IR already has NO region flow. The frontend cascade doesn't need a hammer-level region change.
- `struct_hammer.rs:249` constructs `CoordTemplataI { region: RegionT { region: IRegionT::Default }, coord }` for the Box template — sole live region use at the instantiator→hammer boundary. Retires with `CoordTemplataI.region`.

### 7.3 Landmines and open questions

- **Critic-flagged:** the field `context_region: RegionT` on 50+ signatures may need to stay even as ~120 literals evaporate — if ambient `context_region` is dropped at intermediate levels because "the current coord isn't a BorrowRef," a later coercion that wraps in BorrowRef has no region to use.
- `expression_compiler.rs:1260` `assert!(region == nenv.default_region())` — a vcurious sanity check unreachable today. Delete with `default_region`.
- 8 pub-region fields on `BreakTE`/`RangeTE`/`BorrowTE`/etc. (typing/ast/expressions.rs lines 599, 758, 780, 795, 810, 825, 1103, 1127). Under onion these become region-carriers only where they produce BorrowRef output. Most disappear.
- **Open:** under onion, does `context_region` survive at all as a threaded parameter, or does BorrowRef materialization always happen at a call-site that has direct access to `nenv.default_region()`?
- **Open:** the receiver-instantiation kind flow at `compiler_solver.rs:451-456, 473, 633-634` (5 sites) constructs `CoordT` with `RegionT { region: IRegionT::Default }` when reasoning about parameter matching — all need onion-aware analogs.

---

## 8. `AliasTE`/`AliasIE`/`AliasH` cascade

Minimal-surface, load-bearing at every stage.

- **Emitted exactly once**: `convert_helper.rs:179` for `(Borrow, Share)`.
- **Consumed**:
  - `typing/test/traverse.rs:597`
  - `instantiator.rs:1910` (translated to AliasIE)
  - `simplifying/expression_hammer.rs:451` (translated to AliasH)
  - `final_ast/test/traverse.rs:462`
  - `testvm/expression_vivem.rs:200, 1096` (name + execution, delegates to inner)
  - `backend_ffi/metal_lowerer.rs:663` (delegates to inner)

**Full cascade footprint** per `git show 5a5aa93ed --numstat`: ~104 net lines added purely for the Alias node itself (typing enum variant + struct + Result method + instantiator arm + hammer arm + testvm arms + backend arm + traverser arms + humanizer coverage-by-omission).

Under onion, all three delete; the `(Borrow, Share)` coercion becomes a probe-based structural rewrite in the new `convert()`. **`AliasIE`'s `target_ownership: OwnershipI` field** (`expressions.rs:823`) survives even after the emitter is unreachable — deletion must be end-to-end: enum variant (`:143`), struct (`:821`), emitter (`instantiator.rs:1910-1929`), hammer (`expression_hammer.rs:451-461`), AliasH (`final_ast/instructions.rs`), FFI (`metal_lowerer.rs`). Missing any leaves dead code the reviewer flags.

**Sequencing invariant (handoff-preserved):** "Do not delete `AliasTE` / coercion-accept patches / coherent-collapse arms until onion typing is landed end-to-end at T-IR + I-IR + H-IR." So the cascade stays through the entire arc and dies at the very end (post-cascade), not up front.

---

## 9. Docs, arcana, humanizer, fixtures

### 9.1 Architectural docs

Five architecture docs describe `Coord` as `{ ownership, region, kind }` and `OwnershipT` as a flat tag — all need rewrite:

- `typing-pass-design-v3.md`
- `instantiator-design.md` and `instantiator_design_2.md` (**already drift on whether `CoordI` has a region field**; reconcile at the same time as the onion rewrite)
- `simplifier-design.md`

Plus reconciliation with `docs/architecture/bare-clone-borrow-move-design.md` — **conflicts with onion typing** on two points:
- Phase I says "`@T` retires" — but handoff's share blanket uses `func clone<T>(x &@T) @T`.
- Phase H says `share → class` keyword rename — handoff §3 says `share_flavored: bool` intrinsic, no rename mandate. If rename dropped, ~42 fixture .vale files + ~15 inline test fixtures are spared bulk-editing.

**Add an "obsoleted by onion typing" banner** to `bare-clone-borrow-move-design.md` (line 3 says "Status: design (partial implementation in progress)" — outdated).

**Reviewer skills** — 3 embed pre-onion patterns in BEFORE/AFTER examples: `valec-reviewer.md`, `prose-reviewer.md`, `typing-reviewer.md`.

**Historical (keep as-is):** Scala-era `docs/HigherTypingPass.md` and `docs/Generics.md`.

### 9.2 Arcana tags

**Only `VCOORD` exists** (112 hits, ~150 more counting builtin-op comments). No `VOWN`/`VBORR`/`VSHARE`/`VWEAK`/`VOWNSMT` tags. A new shield covering "reference layers are structural, not tag-based" is a candidate but optional.

Interacting arcana that need doc-side rewrite when dispatch redesign lands: `@BRRZ` (BoundReturnResolution) and `@BDPFWDZ` (ByDefaultPullFromWhereDeclared).

### 9.3 Humanizer surface

**Three separate `humanize_ownership` sites** match on the flat ownership tag:
- `typing/compiler_error_humanizer.rs:621-631`
- `instantiating/instantiated_humanizer.rs:37-49`
- `postparsing/post_parser_error_humanizer.rs:369-379`

All three convert to per-ref-layer recursion with the `weak Spaceship` keyword freeing `&&` for double-borrow.

**Humanizer inconsistency today:** I-IR humanizer renders `MutableShare` as `""` (empty string); T-IR humanizer renders `Share` as `"@"`. Onion typing forces resolving this — decide before rewrite whether golden strings should be updated to which way.

**Open:** which surface syntax renders each ref layer? Handoff §Coercion table uses `&T`, `heap T`, `@T`, `weak T` in prose — is that end-state or working shorthand?

### 9.4 Golden strings — surprisingly few

- Only 3-4 golden strings mention `@Kind` prefixes (`compiler_mutate_tests.rs:233`, `after_regions_tests.rs:467`, `compiler_tests.rs:4081`).
- The sole `contains("^")` assertion (`compiler_tests.rs:2043`) survives.

### 9.5 Fixture `.vale` programs

- ~13 weak-tests use `&&Muta` for the weak sigil — need `weak`-keyword syntax updates. **The parser flip must land with all 13 fixture updates + humanizer swap + `weak` keyword introduction in one atomic commit.** No incremental slice on this axis.
- ~42 files use `share` in citizen declarations — survives if declaration syntax stays.
- ~150 `& overload` functions in `arith.vale` + `logic.vale` may retire.

### 9.6 testvm surface

Branches on `OwnershipH` but does NOT render to strings that tests assert on — no golden-string surface, though runtime semantics need refactoring separately (see §5.3).

---

## 10. Cross-cutting landmines (from the completeness critic)

1. **Interner intern-key drift** when adding `share_flavored: bool` to `StructTT`/`InterfaceTT` — must synchronize `StructTTValT` change.
2. **Slice-A sequencing landmine — panicking `translate_placeholder` path** — `assemble_placeholder_map_inner` (`instantiator.rs:892`) panics `unimplemented arm` on any KindT variant it doesn't recognize. Slice A CANNOT be T-IR-only unless T-side code carefully avoids constructing BorrowRef around placeholders — which defeats the tracer's purpose. Explicit sequencing: Slice A adds `KindT::BorrowRef` + `KindIT::BorrowRefIT` + placeholder-peel logic in one commit.
3. **Cross-IR shape divergence during transition** — under handoff Q6 all three IR stages get the onion, but slices likely land T-IR, then I-IR, then H-IR. During interim states, `translate_kind` in instantiator/hammer must handle onion KindT/KindIT variants its target IR doesn't yet support.
4. **`HashSet<CoordT>` and `HashSet<OwnershipT>` collections** across the compiler (block_compiler.rs:32,47; expression_compiler.rs:71,388,487,512,665,2272; function_body_compiler.rs:151) dedupe returns for common-ancestor computation. Under onion, `HashSet<KindT>` — but dedup semantics change: `BorrowRef(K, r1)` and `BorrowRef(K, r2)` hash and compare differently. Regional unification at the collection point needs an explicit strategy.
5. **`get_placeholders_in_kind` walker** (`compiler.rs:174`) — only handles Struct/Interface/RSA/SSA/OverloadSet. Adding BorrowRef/HeapOwnRef/ShareRef/WeakRef needs recursion into inner. If missed, `sanity_check_conclusion` silently misses placeholders nested under ref variants, letting under-substituted templatas through — opaque instantiation errors far downstream.
6. **Reachability walker (`typing/reachability.rs`) is still unimplemented (Slab 15).** Every method panics `Unimplemented`. When Slab 15 lands, it'll be onion-aware from the start — coordinate with the Slab 15 implementer.
7. **`@TFITCX` / `@WVSBIZ` interner discipline** for new payload types — `BorrowRefT` / `HeapOwnRefT` / `ShareRefT` / `WeakRefT` need entries in `InternedKindPayloadValT`/`InternedKindPayloadT`, intern methods in `typing_interner.rs`, `MustIntern` witnesses. Docs `docs/reasoning/environments-per-denizen-long-term.md` and `docs/arcana/WhenValuesShouldBeInterned-WVSBIZ.md` list `CoordT`/`OwnershipT` as inline-Copy value-types; both need updating.
8. **Manual PartialEq/Hash on `IEnvEntryT` dedupe assumption** (`env/i_env_entry.rs:26-55`) — `ITemplataT` variant uses derived Hash. Under onion the `Coord` variant collapses into `Kind`; verify `ITemplataT` still hashes cleanly through the migration alias transition.
9. **`RegionT` vanishing from non-Borrow layers breaks region-propagation code path counts** — ~120 `RegionT { region: IRegionT::Default }` literals thread a region through EVERY call site regardless of whether that call site's Coord will end up as a BorrowRef. If ambient `context_region` is dropped at intermediate levels, a later coercion that wraps in BorrowRef has no region to use.
10. **Parser surface `&&` collision requires atomic flip** — templex parser (`templex_parser.rs:271`) and expression parser (`expression_parser.rs:1937`) both parse `&&` as `OwnershipP::Weak`. Under onion `&&` becomes double-borrow. Must land parser flip + all 13 weak fixtures + humanizer swap + `weak` keyword in **one atomic commit** — no incremental slice possible.
11. **Migration alias `type Coord = Kind` interacts with `expect_coord_templata`** (`templata.rs:45`) — with the alias, `Coord = Kind` at the type level but `ITemplataT::Kind` and `ITemplataT::Kind` remain distinct enum variants (or merge). Sequence carefully around `ITemplataT`.
12. **`KindPlaceholderT` (`types.rs:413`) has NO ref-layer arm and no ownership axis.** Under onion, does a placeholder itself have an implicit shape? Does `func foo<T>(x T)` mean T ranges over any Kind including BorrowRef, or bare non-ref Kinds only? Handoff doesn't say. Blocks the trip's blanket — implies T ranges over all shapes, but then `func drop<T>(self T)` may resolve for T=BorrowRef which is nonsense.

---

## 11. Open design questions (surfaced by investigators)

The scouting also surfaced substantive design questions the handoff doesn't yet answer:

1. **Variance under borrow:** does `BorrowRef(Sub, r)` isa `BorrowRef(Super, r')` when `Sub` isa `Super`? Blocks all is-parent/is-descendant refactoring. (§1.5)
2. **`AugmentSR` shape:** single rule with ref-variant discriminator, or fracture into `BorrowAugmentSR` / `HeapOwnAugmentSR` / `ShareAugmentSR` / `WeakAugmentSR`? (§2.2)
3. **`AugmentSR` region source:** add `region_rune` to `AugmentSR`, or look up `context_region` from ambient env, or restrict `BorrowRef`-emitting Augments to explicit-region syntactic forms? (§2.2)
4. **`CoordComponentsSR` fate:** retire entirely, or restructure into `BorrowRefComponentsSR` (result_rune, region_rune, inner_rune) decomposing only outermost layer? (§2.1)
5. **`IExpressionPE::Augment` value-side:** needed at all under onion given bare-use produces BorrowRef? Or delete for Borrow, keep only for `^x` (Move) and `weak x`? (§2.2)
6. **`get_kind_equivalent_runes` peel depth:** "same kind" = "same base citizen after peeling ALL ref layers" (deep peel) or "same immediate wrapped kind" (shallow peel one)? (§2.4)
7. **`MutabilifyH` fate:** survives under onion, or a workaround for a pre-Q1 mutability tag that onion dissolves? (§5.1)
8. **Interim FFI shape:** during frontend arc, does `metal_lowerer` derive `(OwnershipH, LocationH)` from the peeled onion (FFI stays 6-way), or does the FFI shift to Kind-only first with a Rust-side adapter, forcing Backend arc to open earlier? (§5.2)
9. **`OpaqueHT` under onion:** legitimate `BorrowRef(OpaqueHT(_))` at FFI? (§5.4)
10. **Weak on non-share validity:** handoff table says WeakRef on non-share is ✗, but weakable-struct-non-share tests exist. Reconcile. (§6.3)
11. **Placeholder shape range:** does `T` in `func foo<T>(x T)` range over all Kinds (including BorrowRef) or bare non-ref only? (cross-cutting)
12. **Namespace-membership design:** does "mentions T in a parameter" count `&Ship`? (§2.6)
13. **Surface syntax choices:** which of `Ref` / `Kind` keyword survives? Does `share` keyword survive on citizen declarations (or is the `share → class` rename kept)? Does `weak` as identifier collide (needs lexer audit)?
14. **`ShareRef` autoderef via row (c):** peels `ShareRef` — the eligibility rule Q5 excludes blankets that peel `BorrowRef`, but peeling `ShareRef` is allowed. Confirm the asymmetry is intentional. (§6.3)

---

## 12. Critic-flagged gaps (still to investigate)

These weren't deeply covered by the 10 investigators. Address before or during implementation:

1. **Typing interner + Kind payload family plumbing.** No investigator did a dedicated deep dive on `typing_interner.rs` (498 lines) or the `InternedKindPayloadValT`/`InternedKindPayloadT` dispatch enums. Sequence-critical for Slice A. (§1.1)
2. **`ISubKindTT`/`ISuperKindTT`/`ICitizenTT` dispatch enums under onion** — design decision blocking isa work. (§1.5)
3. **Abstract-dispatch machinery (`edge_compiler.rs`) — heavy CoordT construction** at `:262-681` (dispatcher param types, virtual dispatch evaluation) not enumerated. ~15-20 CoordT construction sites; cascade with `anonymous_interface_macro.rs`.
4. **Postparse patterns and Atom `coord_rune` throughout scout stages.** 50+ construction sites; rename decision changes diff size. (§3)
5. **Sanity checker + reachability + reachable-bounds harvest under nested refs** — small code footprint, high semantic impact. A single missing arm breaks trait-bound resolution silently. (§10.5)
6. **`IExpressionSE::Ownershipped` semantics under `LoadAsP × onion source shape`** — under onion, source-syntax `weak x` becomes a valid new form. Parser AST + lexer surface needs a plan. Check tests/fixtures for `weak` as identifier.
7. **Manual PartialEq/Eq/Hash impls when `CoordT` dissolves via alias** — sequencing matters.
8. **Backend FFI C++ side beyond `primitives.h`/`linear.cpp`/`determinism.cpp`** — `Backend/src/metal/types.h` Reference invariants; region translation across `region/rcimm`, `region/resilient*`, `region/common`. Pre-flight audit needed.
9. **testvm heap/values runtime semantics under new Kind ref variants** — RC accounting under `BorrowRef(ShareRef(SC))`, weak-lock semantics, transmute pathway. testvm is the ground-truth semantic reference.
10. **Determinism / replay serialization of scrambled-int256 ref maps** — which ref variant maps to which FFI representation not spec'd anywhere. Backend arc but forms a hard cross-cut with Rust FFI shape decisions. Stability invariant.
11. **Builtin functions & primitive typeclass instances** — where do `implicit_clone` blanket definitions live? Compiler-synthesized or `.vale`-defined? Vale doesn't have generic bound predicates today. **Blocks Coercion Table row 1.**
12. **`@BDPFWDZ` arcana + bound-based visibility semantics under namespace-dispatch redesign.** Doc rewrite + sequencing decision.
13. **`get_sharedness` special-cases for `Str` + arrays** under onion — where does `StrT`'s `share_flavored=true` live? Are `SSA(BorrowRef(K, r), N)` (reference arrays) legal onions? (§1.2)

---

## Where to go next

Sections 2 and 3 (solver + templata) are the highest-density surfaces and the most likely to teach the arc how to sequence itself. §10-12 name the sequencing decisions the plan needs to answer:

- Interner-side plumbing (§1.1, critic gap 1) must land before or with the Kind variants.
- The atomic parser-flip commit (§10.10) is unmoveable — plan around it.
- `AliasTE` cascade delete happens at the very end (§8, handoff preserve).
- Variance under borrow (§11.1) and placeholder shape range (§11.11) are design gates that block downstream work.
- Backend arc is deferred; frontend arc's interim FFI shape (§11.8) determines whether that stays clean.

The plan doc pairs with this findings doc and answers the sequencing questions concretely.
