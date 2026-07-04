# Onion typing — big-bang plan

Pairs with `vcoord-handoff.md` (design) and `onion-typing-scouting.md` (findings). This is the plan for landing the onion typing refactor as **one coherent change**, not a sequence of phases. There are internal sequencing invariants (some sub-groups must land atomically together), but there is no "Phase 1 lands, then Phase 2." Either the whole frontend arc lands green or none of it lands.

The Backend arc (C++/Metal onion mirror) stays deferred per handoff §Q6 — the frontend arc terminates at the Rust→C++ FFI boundary with an adapter that peels the frontend onion back down to today's flat `Ownership`/`Location` for backend consumption. That adapter dies during the Backend arc.

---

## A. Design gates — answer before writing code

These are questions surfaced by scouting that block downstream work. Each needs an architect answer; none can be inferred from vcoord-handoff.md alone. I've marked my provisional lean; final decisions before starting.

| # | Question | Provisional lean | Blocks |
|---|---|---|---|
| A1 | **Variance under borrow.** Does `BorrowRef(Sub, r)` isa `BorrowRef(Super, r')` when `Sub` isa `Super`? | Yes — recursive covariance. Symmetric to Rust's `&Sub: &Super` upcast. | All isa work in the solver; `ISubKindTT`/`ISuperKindTT` arm design. |
| A2 | **Placeholder shape range.** Does `T` in `func foo<T>(x T)` range over any Kind (including `BorrowRef`, `ShareRef`, etc.) or bare-citizen only? | Any Kind. Required for the trip's borrow-blanket `func clone<T>(x &&T) &T`. Bounds narrow it. | `assemble_placeholder_map_inner`; substitution walkers; every generic-param typing path. |
| A3 | **`AugmentSR` shape post-onion.** Single rule with `ref_variant: RefVariantP` discriminator, or fracture into `BorrowAugmentSR` / `HeapOwnAugmentSR` / `ShareAugmentSR` / `WeakAugmentSR`? | Single rule with discriminator. Smaller diff at emission sites; solver pattern-match cost is negligible. | 5 emission sites; `AugmentSR` solver handler; parser lowering. |
| A4 | **`AugmentSR` region source.** Where does the `BorrowRef` layer's region come from at solve time? | Add `region_rune: RuneUsage` to `AugmentSR` for the BorrowRef case; None for the other three variants. `templex_scout` populates from surrounding context; other 4 emission sites (anon-interface macro) provide explicit rune. | AugmentSR schema (touches rule_scout.rs, get_runes, get_puzzles, all 5 emission sites, rune_type_solver, identifiability_solver, test traversers). |
| A5 | **`CoordComponentsSR` fate.** Retire entirely, or restructure into `BorrowRefComponentsSR(result_rune, region_rune, inner_rune)` decomposing only outermost layer? | Restructure into `BorrowRefComponentsSR` only. `anonymous_interface_macro`'s "peel outer, replace citizen, rewrap" pattern maps to this cleanly. The other three ref variants get analogous rules if needed by macros — but on-demand, not upfront. | `anonymous_interface_macro.rs:786+813` rewrite. |
| A6 | **`IExpressionPE::Augment` value-side survival.** Under bare-use-produces-BorrowRef, does surface `&x` survive? | Kept for `^x` (Move) and `weak x`. Idempotent for BorrowRef sources per handoff Q3 (may not exist long-term). Deleting entirely is a follow-up when the ergonomics settle. | expression_scout.rs:404 lowering; parser `&&` collision. |
| A7 | **`get_kind_equivalent_runes` peel depth.** For common-ancestor computation, is "same kind" deep-peel or shallow-peel? | Deep peel (peel all ref layers to base citizen). Matches semantics of "is this the same underlying type." | `rule_scout.rs:397-424` graph derivation; complex_solve. |
| A8 | **`MutabilifyH` fate.** Survives under onion, or dies? | Investigate first — the LLVM emission path (`metal_lowerer.rs:458-463`) determines this. If purely a mutability-flip vestige, delete; if real semantics, keep with a `WeakRef` arm. | H-IR node inventory. |
| A9 | **Interim FFI shape.** Does `metal_lowerer` derive `(OwnershipH, LocationH)` from the peeled onion (FFI stays 6-way), or does the FFI shift to Kind-only first? | FFI stays 6-way. `lower_coord_to_reference` becomes "peel the onion to compute today's (OwnershipH, LocationH, KindHT) triple, pass to C++." The peel logic dies during Backend arc. | metal_lowerer.rs adapter shape. |
| A10 | **`OpaqueHT` under onion.** Legitimate `BorrowRef(OpaqueHT(_))` at FFI? | Not legal at FFI (extern kinds are opaque handles, not values you borrow). Panic path at `metal_lowerer.rs:262` stays. | FFI panic coverage. |
| A11 | **Weak on non-share.** Handoff table says `WeakRef` on non-share is ✗, but weakable-struct-non-share tests exist. Reconcile. | Handoff table stands — `WeakRef` requires share-flavored citizen. Existing weakable-struct-non-share tests get their non-share struct converted to share, or the tests get retired. Audit before commit. | Interner validity; ~3-5 fixture updates. |
| A12 | **Namespace-membership `&Ship` counts as mentioning `Ship`?** | Yes. Preserves today's `get_param_environments` behavior; matches user intuition. | Dispatch redesign (out of scope for this arc — see §Out of scope). |
| A13 | **Surface keyword choice.** `Ref` vs `Kind` as the rune-type keyword survives? | `Kind`. Matches the merged internal type name. Requires fixture updates (~small; the two keywords are used inconsistently already). | Parser rune-type dispatch; fixture rune-type annotations. |
| A14 | **`weak` as identifier collision.** Does introducing `weak` as a keyword break any existing `.vale` code using `weak` as an identifier? | Audit needed. If any test uses `weak` as a local/param name, either rename the local or scope the keyword to `weak T` positional contexts. | Lexer rules; possibly ~a few fixture renames. |
| A15 | **`share` keyword survival on citizen declarations.** Does the earlier `share → class` rename (from `bare-clone-borrow-move-design.md` Phase H) apply, or is that dropped under onion? | Dropped. `share_flavored: bool` is intrinsic; the `share` keyword survives at the declaration site (`struct Spaceship share`) with no `class` rename. Saves ~42 fixture files + ~15 inline test fixtures. | `bare-clone-borrow-move-design.md` reconciliation; fixture bulk-editing scope. |
| A16 | **Migration alias `type Coord = Kind` usage.** Use during the arc, or hard-delete `CoordT` in one go? | Use as an interim shim — reduces mechanical rename churn during the arc while the semantic collapse proceeds. Retire the alias at the very end of the arc, in the same commit that deletes the AliasTE cascade. Add a `// TEMP:` marker at the alias site. | Rename discipline throughout. |
| A17 | **Builtin `implicit_clone` blanket location.** Compiler-synthesized, or defined in a builtin `.vale` file with a new "is-primitive" or "is-share" bound predicate? | Compiler-synthesized for primitives (they can't be expressed in .vale — Vale lacks bound predicates). User-defined per non-share citizen in .vale (as today). Share + weak + share-autoderef blankets are compiler-synthesized wrappers around RC-bump / weak-bump / peel ops. HeapOwn-autoderef blanket is descoped initially per handoff. | Coercion Table row 1 & row 3 & row (a) implementation. |

**Sequencing gate:** A1, A2, A3, A4, A5, A11, A13, A14, A17 must be answered before writing code. A6, A7, A8, A9, A10, A12, A15, A16 can be answered as the arc progresses but decisions are needed within the arc.

---

## B. Prep work — no code changes, just alignment

Do these first because they're cheap and unblock the code work:

1. **Reconcile `bare-clone-borrow-move-design.md`** — add "obsoleted by onion typing" banner at top. Resolve the `@T retires` (Phase I) and `share → class` (Phase H) conflicts per A15.
2. **Add "under active onion-typing rewrite" banner** to `typing-pass-design-v3.md`, `instantiator-design.md`, `instantiator_design_2.md`, `simplifier-design.md` — flagging pre-onion patterns for readers.
3. **Reviewer skills sync** — mark BEFORE examples in `valec-reviewer.md` / `prose-reviewer.md` / `typing-reviewer.md` with `// pre-onion` where relevant. Don't rewrite yet; the arc will update them.
4. **Interner discipline docs (@TFITCX, @WVSBIZ)** — add the four new ref-variant payload structs to the payload-family inventory. Small doc-only change; sets up the interner cascade.
5. **Weak-fixture audit** — enumerate the 13 `.vale` files using `&&Muta`. Include in the atomic parser-flip subcommit (§C.2).
6. **`weak` identifier audit** — grep `.vale` fixtures for `weak` as a local/param name. Rename any collisions in the atomic parser-flip subcommit.
7. **`share` keyword audit** — confirm A15 by grepping how many fixtures use `share` today; verify keeping the keyword saves the bulk-edit.
8. **Backend Rust→C++ FFI adapter design** — sketch how `metal_lowerer::lower_coord_to_reference` peels the onion back down to `(OwnershipH, LocationH, KindHT)` per A9. Not code, just a design note.

None of B touches Rust source; all can happen concurrently.

---

## C. The change — coherent set of edits landing together

The following ten areas of change land as one coherent refactor. The Rust source doesn't compile until they're all done; the test suite doesn't turn green until they're all done. Internal sequencing invariants (atomic sub-commits within the change) are noted per area. There's no "Phase 1 vs Phase 2" — this is all one landing.

### C.1 Type system core — `KindT` gains four ref variants, `CoordT` dissolves

**Files:** `FrontendRust/src/typing/types/types.rs`, `FrontendRust/src/typing/typing_interner.rs`, `FrontendRust/src/typing/templata/templata.rs`.

- Add `KindT::BorrowRef(&BorrowRefT)`, `KindT::HeapOwnRef(&HeapOwnRefT)`, `KindT::ShareRef(&ShareRefT)`, `KindT::WeakRef(&WeakRefT)`.
- Add payload structs: `BorrowRefT { inner: KindT, region: RegionT }`, `HeapOwnRefT { inner: KindT }`, `ShareRefT { inner: KindT }`, `WeakRefT { inner: KindT }`.
- Add corresponding `ValT` companions per the interner pattern (types.rs:418-424).
- Add `InternedKindPayloadValT` and `InternedKindPayloadT` variants for all four.
- Add intern methods in `typing_interner.rs` per macro conventions (lines 55-96).
- Add intern lookup arms in `intern_kind_payload` (line 425+).
- Add `share_flavored: bool` to `StructTT`, `InterfaceTT`, and their `ValT` companions (synchronized change per landmine).
- **Interner validity table** enforced at intern-time (rejects invalid onions per handoff §What(3)):
  - Bare share citizen → reject.
  - `ShareRef` around non-share citizen → reject.
  - `WeakRef` around non-share citizen → reject.
- `CoordT::new` validity checks (Share+primitive, Share+OverloadSet) delete; replaced by the interner-side validity table.
- `type Coord = Kind` migration alias added (per A16); marked `// TEMP: retire at end of arc alongside AliasTE deletion`.
- `ITemplataT::Coord`, `ITemplataI::Coord`, `ITemplataType::Coord` collapse into `Kind` variant (or the `Coord` variant becomes a transitional alias per A16).
- `expect_coord_templata` (`templata.rs:45`) survives as a smart-view during the alias transition; retired at end of arc.

**Atomic sub-commit invariant:** the interner payload family + `StructTT.share_flavored` + `ValT` companion updates must land together to avoid intern-key drift.

### C.2 Parser + surface syntax + fixtures — the atomic flip

**Files:** `FrontendRust/src/lexing/*`, `FrontendRust/src/parsing/templex_parser.rs`, `FrontendRust/src/parsing/expression_parser.rs`, `FrontendRust/src/postparsing/*`, `FrontendRust/src/keywords.rs`, plus ~13 weak fixtures + any `weak`-as-identifier fixtures + any `Ref`-as-rune-type fixtures.

- Add `weak` keyword to the lexer.
- Parser: `&&T` becomes double-borrow (was `WeakP(T)`); `weak T` becomes the new weak spelling.
- `OwnershipP` enum: `Weak` variant kept (used internally by the parser AST) but its parse-site spelling flips.
- Rune-type keyword: `Ref` → `Kind` per A13 (fixture updates).
- 13 weak `.vale` fixtures updated to `weak Muta` syntax.
- Any fixture using `weak` as an identifier renamed per A14 audit.
- Any fixture using `Ref` as rune-type keyword updated to `Kind`.
- `IExpressionPE::Augment` handling for `weak x` at value-level: routes to `LoadAsP::LoadAsWeak` (unchanged semantics; syntax change only at parse layer).
- `IExpressionPE::Augment` for `&&x` at value-level: parses as `Augment(Augment(x, Borrow), Borrow)` = double-borrow (was Weak).

**Atomic sub-commit invariant:** parser flip + all fixture updates + humanizer swap (C.10) + `weak` keyword lex support **must all land in one commit**. No intermediate state where fixtures parse differently than tests expect.

**Non-goal:** we do NOT change `.vale` fixtures using `share Spaceship` at the declaration site (A15).

### C.3 Solver — rule rename, restructure, and coercion-accept deletion

**Files:** `FrontendRust/src/postparsing/rules/rules.rs`, `FrontendRust/src/postparsing/rules/rule_scout.rs`, `FrontendRust/src/typing/infer/compiler_solver.rs`, `FrontendRust/src/typing/macros/anonymous_interface_macro.rs`, `FrontendRust/src/typing/infer_compiler.rs`, `FrontendRust/src/typing/edge_compiler.rs`.

**Renames (structural survival):**
- `CoordSendSR` → `KindSendSR`. Handler body peels outer ref layers to base citizen, then runs `is_parent` on the base. Dynamic self-replacement into `KindIsaSR` at solve time preserved verbatim (per handoff §What preserves).
- `CallSiteCoordIsaSR` → `CallSiteKindIsaSR`. Same peel-and-check treatment.
- `DefinitionCoordIsaSR` → `DefinitionKindIsaSR`. Same.

**Restructures:**
- `AugmentSR` restructured per A3 (single rule with `RefVariantP` discriminator) + A4 (add `region_rune: Option<RuneUsage>`). Emission sites updated in `templex_scout.rs:285` (2 sites) and `anonymous_interface_macro.rs:588` (3 sites). DIR1/DIR2/DIR3 handlers rewritten to peel/wrap ref layers instead of augmenting ownership tags.
- `CoordComponentsSR` restructured to `BorrowRefComponentsSR(result_rune, region_rune, inner_rune)` per A5. Only decomposes `BorrowRef` layer. `anonymous_interface_macro.rs:786+813` (2 sites) rewritten to use it for interface→struct citizen swap while preserving the outer BorrowRef.
- `OwnershipTemplataT`, `OwnershipLiteralSL`, `OwnershipTemplataType`, `evaluate_ownership`, `humanize_ownership` — the whole Ownership templata axis deletes together.

**Deletions:**
- **CoordSendSR else-branch coercion-accept patch** (`:978-1029`) — deleted (was the whole raison d'être per handoff §What(4)).
- **CallSiteCoordIsaSR ancestor-branch coercion-accept patch** (`:806-818`) — deleted.
- **AugmentSR DIR1 Shared-arm consistency check** (`:1197-1214`) — deleted after the interner-side validity check (C.1) lands.
- **KindComponentsSR** — dead code, deleted outright.
- **CoerceToCoordSR** and its handler (`compiler_solver.rs:1111-1141`) — deleted with `CoordT`.
- **`is_type_convertible`** (`templata_compiler.rs:1143-1216`) — the ownership-pair tail (`:1184-1213`) deletes; the region check (`:1180-1182`) relaxes to "compatible regions on the outermost BorrowRef layer." Function collapses to a pure kind-shape / isa check.

**`complex_solve` receiver consensus** (`:377-487`):
- `HashSet<OwnershipT>` merge (`:463-471`) becomes `HashSet<RefVariantP>` merge over the outer ref-layer variant.
- `ReceivingDifferentOwnerships` renames to `ReceivingDifferentRefShapes`.
- The `Augment` override at `:449-452` restructures to wrap `receiver_instantiation_kind` in the target ref variant.

**Overload resolver** (`overload_resolver.rs`):
- `narrow_down_callable_overloads` per-param exact-preference + normal-vs-bound split + bound-length ordering — DELETE. Ambiguity is an error, not a tiebreak. `panic!("No candidate is a clear winner!")` at `:743` becomes an ambiguity error surface.
- `params_match` exact=true mode survives (bound resolution is exact-match by design).
- **Dispatch redesign per handoff mission is OUT OF SCOPE** for this arc (see §D). `get_candidate_banners` behavior stays as-is; we only remove the tiebreakers.

### C.4 Templata + rune types — Kind/Coord merger

**Files:** `FrontendRust/src/typing/templata/templata.rs`, `FrontendRust/src/typing/rune_type_solver.rs`, `FrontendRust/src/higher_typing/*`, `FrontendRust/src/postparsing/ast.rs`.

- `ITemplataT::Coord` variant collapses into `Kind` (or aliased via A16 shim). Every match arm on `Coord(ct) => ct.coord` becomes a single Kind arm.
- Same for `ITemplataI::Coord` and `ITemplataType::Coord`.
- `CoerceToCoordSR` handler deleted (C.3).
- `coerce_kind_lookup_to_coord`, `coerce_kind_template_lookup_to_coord` deleted.
- `rune_type_solver.rs:486` Kind→Coord auto-convert arm deleted. The `panic!("lookup_rune_type Templata FoundTemplataDidntMatchExpectedType not yet implemented")` at line 497 either becomes reachable (write a real handler) or the `higher_typing_pass` `explicify_lookups` simplification lands in this commit.
- **`AtomSP.coord_rune`** field renamed to `AtomSP.kind_rune` semantically (per findings §3). ~50 construction sites across `postparsing/`, `higher_typing/`, `typing/` — mechanical rename.
- `type_name_to_mutability` map in `CompilerOutputs` — retire.

### C.5 Instantiator (T→I) — walker rewrites and side-map deletion

**Files:** `FrontendRust/src/typing/templata_compiler.rs`, `FrontendRust/src/instantiating/instantiator.rs`, `FrontendRust/src/instantiating/ast/*`.

**Layer A (typing-pass substitution):**
- `substitute_templatas_in_coord` (`:389`) deleted. `substitute_templatas_in_kind` (`:426`) becomes the sole substitution walker, returns `KindT` directly.
- The 7-arm ownership-composition table (`:400-423`) deleted, including the `unreachable!` at `:418` (which becomes reachable under onion — nested WeakRef compositions are valid).

**Layer B (T→I boundary translation):**
- `translate_coord` (`:2170`) collapses into `translate_kind` (`:2353`).
- `translate_kind` gains four arms for `BorrowRef`/`HeapOwnRef`/`ShareRef`/`WeakRef`.
- **`compose_ownerships`** (`:2020`, 13 arms) and **`compose_ownerships_second`** (`:2058`, 13 arms) deleted (sites 4-5).
- **`translate_ref_expr::SoftLoad`** target-ownership match at `:1930-1956` — the two `// VCOORD: papering over` arms deleted (sites 6-7). Result kind computed from the source's onion + `LoadAsP` directly.
- **`translate_ref_expr::Alias`** reflavor at `:1910-1929` deleted (AliasIE emitter, retires with the cascade).
- `translate_ownership` (`:2011-2018`) deleted (was already panicking on Weak).
- `CoordI::void()` (`types.rs:56-59`) stub deleted; call sites use `KindIT::VoidIT(VoidIT{})` directly.
- `translate_coord`'s KindPlaceholder branch (`:2178-2189`) becomes the ONLY branch; the `Unimplemented: translate_coord KindPlaceholder->Kind` panic path becomes reachable (write the real handler).

**Side-map deletion:**
- `struct_to_sharedness` (`:215`), `interface_to_sharedness` (`:217`), `impl_to_sharedness` (`:219`) deleted.
- `translate_mutability` (`:935`), `get_sharedness` (`:2232`) deleted.
- Population point at `:1112-1115` in `translate_struct_definition` / `translate_interface_definition` deleted.

**`assemble_placeholder_map_inner`** (`:873-896`):
- Peel logic added to reach `KindPlaceholder` under ref variants. The `panic!("unimplemented arm")` at `:892` gets real arms for the four ref variants (peel and recurse into inner).
- `(ITemplataT::Coord, ITemplataI::Coord)` arm collapses into single Kind→Kind arm.

**I-IR field cascade** (all these `CoordI` fields become `KindT`/`KindIT`):
- `ast/types.rs:65-86` — `CoordI` struct deleted.
- `ast/citizens.rs:81, 89` — `ReferenceMemberTypeI.reference`, `AddressMemberTypeI.reference`.
- `ast/ast.rs:152, 232, 304, 312, 386, 396, 407, 417` — `ParameterI.tyype`, `FunctionHeaderI.return_type`, Prototype return/params, four `collapsed_coord` fields.
- `ast/expressions.rs` — 30+ `CoordI`-typed fields on IE nodes. `AliasIE.target_ownership`, `SoftLoadIE.target_ownership`, `LetAndLendIE.target_ownership` deleted.
- `ast/templata.rs:93` — `CoordTemplataI.region` deleted.

**Collector + humanizer:**
- `collector.rs:52-59, :142-155` — `all_in_coord` / `visit_coord` walker retires; `visit_kind` grows four recursive arms.
- `instantiated_humanizer.rs:39-66` — `humanize_coord` dissolves; `humanize_kind` grows recursive arms.

### C.6 Hammer + H-IR + Backend FFI adapter

**Files:** `FrontendRust/src/hammer/type_hammer.rs`, `FrontendRust/src/hammer/load_hammer.rs`, `FrontendRust/src/hammer/struct_hammer.rs`, `FrontendRust/src/simplifying/expression_hammer.rs`, `FrontendRust/src/final_ast/types.rs`, `FrontendRust/src/final_ast/ast.rs`, `FrontendRust/src/final_ast/instructions.rs`, `FrontendRust/src/testvm/*`, `FrontendRust/src/backend_ffi/metal_lowerer.rs`.

- `CoordH` (`types.rs:20-51`) deleted. `KindHT` gains `BorrowRefH`, `HeapOwnRefH`, `ShareRefH`, `WeakRefH` variants (regions only on `BorrowRefH`).
- `OwnershipH` deleted.
- `LocationH` **also deleted** (folded into the ref variants — `BorrowRefH`/`HeapOwnRefH`/`ShareRefH`/`WeakRefH` each carry their own inline-vs-yonder implication).
- `RegionH` unit-struct (`ast.rs:27`) deleted.
- `ExpressionH::result_type` — ~50 arms fabricating `CoordH::new(...)` collapse to `KindHT`.
- **4 IR nodes** carrying `target_ownership: OwnershipH` (`LocalLoadH:417`, `RuntimeSizedArrayLoadH:495`, `StaticSizedArrayLoadH:510`, `MemberLoadH`) — field deleted; result_type onion carries the info.
- **`MutabilifyH`** per A8 decision.
- `translate_kind` (`type_hammer.rs:34-59`) gains 4 new arms.
- `translate_coord` (`:65-86`) reduces to `translate_kind`.
- `evaluate_ownership` / `evaluate_location` (`conversions.rs:16-29`) deleted.
- Six `load_hammer.rs` loaders' independent `LocationH` recomputation deleted; each loader's result_type computed from the onion.
- `get_borrowed_location` (`load_hammer.rs:373`) deleted (unimpl + unreferenced).
- `hammer_tests.rs:94` — 5 pattern-match sites rewritten to match onion shape.

**testvm:**
- `ReferenceV::new` (`values.rs:420-434`) validity assertion moves onto interner-time onion validity.
- `heap.rs` heap accounting semantics under `BorrowRef(ShareRef(SC))` explicitly specified (RC bumps at ShareRef materialization, not at BorrowRef).
- `expression_vivem.rs` — 4 arms updated for the four ref-variant SoftLoad results.
- `heap.rs:560-579` transmute pathway keyed on outer ref-variant instead of `OwnershipH == MutableShareH`.
- ~120 sites carrying `OwnershipH`/`LocationH` mechanically updated to the onion form (bulk edit territory — use `safe-script-runner` per `docs/skills/scripting.md`).

**Backend FFI adapter** (per A9):
- `metal_lowerer::lower_coord_to_reference` (`:282-295`) restructured to accept a `KindHT` and peel the onion to compute `(OwnershipH_backend, LocationH_backend, KindHT_stripped)` for the C++ side.
- `lower_ownership` (`:266-273`) — either kept as the peel helper or inlined into `lower_coord_to_reference`.
- Marked `// TEMP: FFI adapter dies during Backend arc`.

**Backend pre-flight (frontend arc doesn't touch, but flags):**
- `Backend/src/region/common/primitives.h:27-45` — 4 Own asserts stay for now; frontend arc's FFI adapter emits Own for primitives so these don't fire.
- Same for `Backend/src/region/linear/linear.cpp:170-179` and `Backend/src/determinism/determinism.cpp:832`.
- Backend arc handles.

### C.7 convert() + expression compiler — probe-based coercion

**Files:** `FrontendRust/src/typing/convert_helper.rs`, `FrontendRust/src/typing/expression/expression_compiler.rs`, `FrontendRust/src/typing/expression/local_helper.rs`, `FrontendRust/src/typing/pattern_compiler.rs`.

- **`convert()`** (`convert_helper.rs:50-190`) rewritten around the probe mechanism:
  - Row 1 (`BorrowRef(P, r) → bare P`) — probe compiler-synthesized `implicit_clone(&P) P` blanket.
  - Row 2 (`BorrowRef(NC, r) → bare NC`) — probe user's `implicit_clone(&NC) NC`; three-way error split preserved (`NoImplicitCloneDefinedT` / `ImplicitCloneRejectedT` / outer-Err `.expect` unreachable-via-Vale-source).
  - Row 3 (`BorrowRef(ShareRef(SC), r) → ShareRef(SC)`) — probe compiler-synthesized share blanket `implicit_clone<T>(&@T) @T`.
  - Row (a) (`BorrowRef(WeakRef(SC), r) → WeakRef(SC)`) — probe compiler-synthesized weak blanket.
  - Row (c) (`BorrowRef(ShareRef(SC), r) → BorrowRef(SC, r')`) — probe compiler-synthesized share-autoderef blanket.
  - Row (b) heap-autoderef — descoped per handoff; error until landed. Explicit `NoHeapOwnAutoderefT` error variant.
  - Row 4 (`BorrowRef(K, r) → BorrowRef(K, r')`) — structural pass-through with region unification.
  - Rows 5-6 — structural pass-through.
  - (d) nested borrows → error.
  - `AliasTE` construction (`:179`) deleted (retires with cascade at end of arc — see §C.9).
  - Probe eligibility rule: **no blankets that peel a `BorrowRef` layer** (excludes borrow blanket `func clone<T>(x &&T) &T` from auto-coercion but keeps it for bound resolution).
- **`wrap_in_implicit_clone`** (`expression_compiler.rs:446-476`) deleted. Its call sites in `evaluate_lookup_for_load` / `coerce_to_reference_expression` become "bare-use wraps in `BorrowRef`; `convert()` probes at target."
- **`borrow_soft_load`** (`local_helper.rs`) restructured — the `get_borrow_ownership` call disappears; the result kind is uniformly `BorrowRef(source_kind, r)`.
- **`get_borrow_ownership`** (`local_helper.rs:216-233`) deleted. The `Int/Bool/Float/Str/Void → Share` arms (the Slice-5 landmine) go with it.
- **`soft_load`** (`local_helper.rs`) restructured — `OwnershipT × LoadAsP` matrix becomes `source_kind × LoadAsP`; result_kind is the appropriate ref-wrap or pass-through.
- **`SoftLoadTE.target_ownership`** (`ast/expressions.rs:1327-1345`) field deleted; result_kind carries the info.
- **`LoadAsP`** enum stays (parse-side surface).
- **`pattern_compiler.rs:241-242`** `.expect` deleted — `infer_and_translate_pattern` returns `Result` so the three-way error split propagates through let-bindings. Un-ignores `user_defined_implicit_clone_allows_bare_use_of_struct` test at `compiler_tests.rs:4888`.
- **`is_primitive` gate** in `evaluate_lookup_for_load` / `coerce_to_reference_expression` deleted. Bare-use is uniform.
- **`weak_alias`** (`expression_compiler.rs:2152-2178`) — under A11, only fires on share-flavored citizens; non-share weakable tests either updated or retired.
- **IfTE common-ancestor result-coord** (`expression_compiler.rs:1163`) — both branches must produce structurally-identical onion kinds; assertion becomes an equality check on the onion.
- **`struct_drop_macro`** — decision per §11 open question. Provisional: kept as synthesized function but its logic updates to peel `HeapOwnRef` layer at the entry point.

### C.8 Region simplification

**Files:** everywhere.

- `RegionT` field deleted from `CoordT` (which is deleted anyway) and from every non-Borrow struct.
- `RegionT` survives only on `BorrowRefT { inner: KindT, region: RegionT }`.
- `IRegionT` enum survives (still 2 variants Default/Iso) but only referenced from BorrowRef sites.
- **~120 `RegionT { region: IRegionT::Default }` literals** deleted — most were coord-construction call sites that no longer need a region argument.
- **~400 region-threading arg sites** — functions taking `context_region: RegionT` become argless where they no longer construct BorrowRef; functions that DO construct BorrowRef (bare-use, `&x`, etc.) keep the arg.
- `nenv.default_region()`, `RawArrayNameT.self_region`, `ExportNameT.region`, function-env `default_region`, closure-env `default_region` — kept ONLY at BorrowRef materialization points.
- `is_type_convertible` region check relaxes to "compatible regions on the outermost BorrowRef layer."
- **8 pub-region fields on expression AST nodes** (`typing/ast/expressions.rs:599, 758, 780, 795, 810, 825, 1103, 1127`) — kept only on expressions that produce BorrowRef output.
- `expression_compiler.rs:1260` `assert!(region == nenv.default_region())` sanity check deleted.
- `struct_hammer.rs:249` sole `IRegionT::Default` construction at instantiator→hammer boundary deleted with `CoordTemplataI.region`.

**Sequencing note:** the region simplification is entangled with C.5 (instantiator) and C.7 (convert). Do them together in the same coherent set of edits.

### C.9 `AliasTE`/`AliasIE`/`AliasH` cascade retirement — the last thing to land

Per handoff §What blocks: "Do not delete until onion typing is landed end-to-end at T-IR + I-IR + H-IR."

**Files (8 sites):**
- `typing/ast/expressions.rs` — `AliasTE` struct + enum variant.
- `convert_helper.rs:179` — sole emitter.
- `typing/test/traverse.rs:597` — consumer.
- `instantiating/ast/expressions.rs:143, :821-825` — `AliasIE` variant + struct.
- `instantiator.rs:1910-1929` — T→I translator arm.
- `simplifying/expression_hammer.rs:451-461` — I→H translator arm.
- `final_ast/instructions.rs` — `AliasH` struct + enum variant.
- `final_ast/test/traverse.rs:462` — consumer.
- `testvm/expression_vivem.rs:200, 1096` — name + execution.
- `backend_ffi/metal_lowerer.rs:663` — FFI delegate.

**End-of-arc atomic sub-commit:** all 8 sites + migration alias `type Coord = Kind` + `expect_coord_templata` retire together. This is the "arc lands green" moment.

### C.10 Docs, arcana, humanizers, fixtures

**Humanizer rewrites (must land with C.2 parser flip):**
- `typing/compiler_error_humanizer.rs:621-631` — per-ref-layer recursion.
- `instantiating/instantiated_humanizer.rs:37-49` — per-ref-layer recursion.
- `postparsing/post_parser_error_humanizer.rs:369-379` — per-ref-layer recursion.

Rendering:
- `BorrowRef(inner, r)` → `&inner` (or `&r'inner` if region non-default).
- `HeapOwnRef(inner)` → `heap inner`.
- `ShareRef(inner)` → `@inner`.
- `WeakRef(inner)` → `weak inner`.
- Reconcile T-IR (`@`) vs I-IR (`""`) share rendering — pick `@`.

**Docs — rewrites (concurrent with the arc, land in the same big-bang commit set):**
- `typing-pass-design-v3.md` — rewrite Coord/Ownership sections to onion.
- `instantiator-design.md` / `instantiator_design_2.md` — reconcile the region-on-CoordI drift, rewrite for onion.
- `simplifier-design.md` — rewrite CoordH/OwnershipH sections.
- `bare-clone-borrow-move-design.md` — apply reconciliation from B.1 (`@T retires` conflict, `share → class` conflict).

**Arcana updates:**
- `docs/arcana/WhenValuesShouldBeInterned-WVSBIZ.md` — add four new ref-variant payload types.
- `FrontendRust/docs/shields/TypesFitIntoTheseCategories-TFITCX.md` — add four new ref-variant categories.
- `docs/reasoning/environments-per-denizen-long-term.md` — remove CoordT/OwnershipT from inline-Copy value-type list.
- Consider adding a new arcana shield covering "reference layers are structural, not tag-based." (Optional; grep-able discoverability.)

**Reviewer skills — update BEFORE examples:**
- `docs/skills/valec-reviewer.md`.
- `docs/skills/prose-reviewer.md`.
- `FrontendRust/src/typing/docs/skills/typing-reviewer.md`.

**Golden strings + fixtures:**
- 3-4 golden strings mentioning `@Kind` prefixes updated (`compiler_mutate_tests.rs:233`, `after_regions_tests.rs:467`, `compiler_tests.rs:4081`).
- `contains("^")` assertion at `compiler_tests.rs:2043` survives.
- 13 weak fixtures updated (already in C.2 atomic sub-commit).
- ~150 `& overload` functions in `arith.vale` + `logic.vale` — provisionally kept during the arc, retired as a follow-up.

---

## D. Out of scope for this arc

Explicitly deferred, per handoff and scouting:

1. **Backend arc** — C++/Metal onion mirror. Per handoff §Q6. Frontend arc terminates at the FFI adapter (§C.6); Backend arc dies that adapter.
2. **Dispatch model redesign** — namespace-based lookup, no Self-specialness, `foo(ship, rocket)` looks in both namespaces. Per handoff §Overload resolution mission — this arc only removes today's tiebreakers (§C.3); the full namespace redesign is a separate arc.
3. **Replay / FFI design** — the scrambled-int256 map + by-value-vs-by-pointer split. Per handoff §Mission (deferred until Backend arc starts).
4. **HeapOwn autoderef row (b)** — descoped initially per handoff. Errors until Backend arc lands or a follow-up implements it.
5. **`arith.vale` + `logic.vale` `& overload` retirement** — ~150 functions. Provisionally kept; separate follow-up.
6. **Reachability walker (Slab 15)** — still unimplemented. Coordinate with Slab 15 implementer to build it onion-aware from the start.
7. **The `share` keyword survival vs `class` rename** — resolved as A15 (keep `share`); the rename is not part of this arc.

---

## E. Sequencing invariants (what must land together)

Within the big-bang, three atomic sub-commit groups have hard invariants:

**Atomic sub-commit 1 — interner core.** C.1 (KindT variants + interner payload family + `share_flavored` on citizens + `ValT` companions + validity table + migration alias) lands as one unit. No intermediate state where `StructTT` has `share_flavored` but `StructTTValT` doesn't (intern-key drift).

**Atomic sub-commit 2 — parser flip.** C.2 (parser `&&` → double-borrow + `weak` keyword + 13 weak fixture updates + `weak`-identifier fixture renames + `Ref` → `Kind` rune-keyword updates) + C.10 humanizers land as one unit. No intermediate state where fixtures parse against a mismatched humanizer.

**Atomic sub-commit 3 — AliasTE cascade retirement.** C.9 (all 8 Alias sites + migration alias `type Coord = Kind` + `expect_coord_templata`) lands as the arc-closing commit. This is the "everything green" moment.

Between these atomic sub-commits, C.3-C.8 are individual work streams that can be authored in any order; the arc is only complete when they've all landed and the test suite passes.

**The overall invariant:** the working tree does not compile between the start of the arc and the end. There is no intermediate "half-onion" state that ships. If the arc has to be paused, roll back to `f47279978` (the current checkpoint).

---

## F. What to build first (once "ok start implementing" is said)

The atomic sub-commit invariants dictate the natural authoring order:

1. **Design gates (§A)** answered — required before any code.
2. **Prep (§B)** — no code, all doc/audit; can land as small commits during A.
3. **C.1 (interner core)** — atomic. Prerequisite for all downstream work.
4. **C.3 (solver) + C.4 (templata) + C.5 (instantiator) + C.7 (convert)** in parallel — the four biggest surfaces. Each depends on C.1 but is otherwise independent.
5. **C.6 (hammer + FFI adapter)** — depends on C.5 producing onion-shaped I-IR.
6. **C.8 (region simplification)** — entangled with C.5 and C.7; author concurrently.
7. **C.2 (parser flip) + C.10 (humanizers)** — atomic, can land at any point during the arc so long as it lands together.
8. **C.9 (AliasTE cascade)** — the arc-closing commit. Lands last.

Between C.1 and C.9, `cargo build --lib` is expected to fail; the aim is that after C.9, `cargo test --lib --no-fail-fast` returns 1111+ / 0 / 95 or fewer (matching the checkpoint baseline or better).

**Fallback:** if the arc grows unmanageable mid-flight, roll back to `f47279978` — never ship a half-onion.

---

## G. What "success" looks like

- `cargo build --lib` clean at end of arc.
- `cargo test --lib --no-fail-fast` returns ≥ 1111 pass / 0 fail / ≤ 95 ignored.
- No new `#[ignore]` additions without architect approval.
- `AliasTE`/`AliasIE`/`AliasH` fully deleted; `Coord*SR` rules renamed to `Kind*SR` or restructured; `OwnershipT`/`OwnershipI`/`OwnershipH` deleted.
- Migration alias `type Coord = Kind` deleted.
- Humanizer + 3 architecture docs + 13 weak fixtures updated.
- Backend FFI adapter (`metal_lowerer::lower_coord_to_reference`) marked `// TEMP: FFI adapter dies during Backend arc`.
- Test suite reflects the new coercion table — `passing_bare_local_to_borrow_param_does_not_need_ampersand`, `error_when_no_implicit_clone_for_borrow_to_own_conversion`, `error_when_implicit_clone_is_defined_but_rejected` still pass (Slice 1-3 tracers preserved). Ideally 16 additional Slice-7 previously-ignored tests un-ignore under structural distinctness.
- ~65 SIGABRT backend tests still ignored (Backend arc territory).
- 20 `replay::*` tests still ignored (Replay arc territory).

---

## H. Explicit non-actions

- Do NOT start writing implementation code until the architect says the literal phrase "ok start implementing" per handoff and CLAUDE.md.
- Do NOT begin C.9 (AliasTE cascade deletion) mid-arc — it's the last thing to land.
- Do NOT lower borrow-of-primitive to Own at metal_lowerer during the arc (handoff explicitly rejects this).
- Do NOT ignore any test to make the suite green mid-arc.
- Do NOT delete AugmentSR DIR1 Shared arm before the interner-side validity check lands.
- Do NOT delete the `.expect("resolve_function outer Err unreachable from Vale source")` invariants without re-verifying they still hold.
