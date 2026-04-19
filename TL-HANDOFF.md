# Typing Pass Migration — TL Handoff

**Taking this over?** Read this doc first, then `quest.md` (with the overrides flagged below). Everything else is directory-local.

## One-paragraph orientation

We're porting `src/typing/` from Scala to Rust, slab by slab per `quest.md` §12. The typing pass holds ~60 concrete name types + ~70 concrete payload types + 9 env types, plus a god-struct `Compiler<'s, 'ctx, 't>` that replaces Scala's ~20 sub-compilers and 15 macros. Scout-pass data (`FunctionA`, `StructA`, interned strings/names) is arena-retained and referenced directly from typing output via `&'s`; typing-pass arena-allocated types carry `<'s, 't>`. **Slabs 0–4 are done** — every typing-pass *data-definition* family now has real fields, the `TypingInterner<'s, 't>` has real bodies, and envs are real. Slab 5 (expression AST) is next. `cargo check --lib` is clean (0 errors, 0 warnings).

## Where we are

| Slab | Scope | Status | Tag / handoff |
|---|---|---|---|
| 0 | arena substrate | ✅ | (scaffolding; no tag) |
| 1 | leaf types (OwnershipT, primitive KindT payloads, leaf templatas) | ✅ | commit `9fd7641c` |
| 2 | name hierarchy (~60 concrete names, 22 sub-enums, IdT, ValT companions) | ✅ | `slab-2-complete` · `FrontendRust/docs/migration/handoff-slab-2.md` |
| 3 | Kind / Coord / Templata trio | ✅ | `slab-3-complete` · `FrontendRust/docs/migration/handoff-slab-3.md` |
| 4 | environments + real interner bodies | ✅ | `slab-4-complete` · `FrontendRust/docs/migration/handoff-slab-4.md` |
| **5** | **expression AST** (`ast/expressions.rs`) | **⏳ next** | handoff not yet drafted; design spec `quest.md` Part 7 |
| 6 | CompilerOutputs | ⏳ | `quest.md` Part 4 |
| 7 | HinputsT + Compiler shell + run_typing_pass | ⏳ | `quest.md` Part 10 |
| 8 | method signatures, clean `cargo build --lib` | ⏳ | |
| 9+ | method bodies, test-driven | ⏳ | |

## Design decisions that *override* `quest.md`

`quest.md` predates several design refactors we did during Slabs 2–4. If the doc and the code/reasoning-doc disagree, **the code and the reasoning doc win**. Sections that are out-of-date:

### `quest.md` §6.3 — `IdT` is monomorphic, not generic

Original: `IdT<'s, 't, T: Copy>` generic in the leaf-name type, with `widen` / `widen_to` / `try_narrow` conversion methods and widest-form-keyed interning.

Current: `IdT<'s, 't>` monomorphic — `local_name: INameT<'s, 't>` always at the widest form. Callers pattern-match to narrow, like Scala does at runtime (Scala's `+T <: INameT` is a JVM-runtime-erased phantom anyway — we matched that shape in Rust). `PrototypeT<'s, 't>` and `SignatureT<'s, 't>` are also monomorphic.

Rationale + alternatives (unsafe-transmute typed views, RawIdT+TypedIdT wrapper, etc.) are recorded in `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md`. All four alternatives are available post-migration; none fit the Scala-parity goal during migration.

### `quest.md` §6.5 — `KindT` is an inline wrapper, not interned

Original: `KindT` interned with `Struct(StructTT<'s, 't>)` (payloads inline).

Current: `KindT` is an inline-owned 16-byte Copy enum. Non-primitive variants hold `&'t StructTT` (payload arena-interned). Primitive variants (`Never`, `Void`, `Int`, etc.) stay inline — too small to warrant arena allocation. Same philosophy for the three Kind sub-enums (`ICitizenTT`, `ISubKindTT`, `ISuperKindTT`).

### `quest.md` §6.6 — `ITemplataT` is an inline wrapper too; `PrototypeTemplataT`'s inner T was also erased

Original: `ITemplataT` interned; `PrototypeTemplataT[T <: IFunctionNameT]` inner T "keep it, threaded through to `IdT<'s, 't, T>`".

Current: `ITemplataT` is inline-owned, not interned. Six of its variants hold `&'t` refs to interned leaf payloads (Coord/Kind/Placeholder/Prototype/Isa/CoordList); five hold `&'t` refs to heavy allocated-but-not-interned payloads (Function/StructDefinition/InterfaceDefinition/ImplDefinition/ExternFunction); the rest are inline Copy values (Integer, Boolean, Mutability, etc.). Heavy-templata `PartialEq`/`Eq`/`Hash` use `std::ptr::eq` on the scout-lifetime refs (scout canonicalizes those).

`PrototypeTemplataT` has no inner T — since `PrototypeT<'s, 't>` is monomorphic, `PrototypeTemplataT<'s, 't>` just holds `&'t PrototypeT<'s, 't>`.

### `quest.md` §3.1 — envs live in `'t` arena, not `'s`; sibling `IInDenizenEnvironmentT` enum

Original: §3.1 has `IEnvironmentT<'s, 't>` variants owning payloads by value and allocated via `scout_arena.alloc(...)` into the scout arena.

Current: envs live in the typing arena `'t` (a `'s`-allocated struct can't hold the `&'t` refs that `TemplatasStoreT` transitively requires, since `'s: 't` and `'t: 's` is false). `IEnvironmentT<'s, 't>` is a 9-variant inline wrapper whose variants hold `&'t FooEnvironmentT<'s, 't>`. There's also a sibling `IInDenizenEnvironmentT<'s, 't>` with the 6-variant in-denizen subset. `global_env` back-refs are `&'t GlobalEnvironmentT<'s, 't>`. See `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` for the rationale and the long-term per-denizen two-tier target; `FrontendRust/docs/architecture/typing-pass-arenas.md` is the current architecture reference; `FrontendRust/docs/migration/handoff-slab-4.md` is the executed Slab 4 spec.

### `quest.md` §6.1 & §1.5 — corrected interned/inline family lists

The refactored families per IDEPFL:

**Interned (dedup via `TypingInterner<'s, 't>`, 6 family-level HashMaps):**
- `INameT` family: ~60 concrete name structs, one shared `name_val_to_ref` map keyed on the tagged-union `INameValT<'s, 't, 'tmp>` enum (72 variants).
- `IdT<'s, 't>` / `IdValT<'s, 't, 'tmp>` — monomorphic.
- Interned Kind payloads family: `StructTT`, `InterfaceTT`, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `KindPlaceholderT`, `OverloadSetT` — one shared `kind_payload_val_to_ref` map keyed on `InternedKindPayloadValT<'s, 't>` (6 variants).
- Interned templata payloads family: `CoordTemplataT`, `KindTemplataT`, `PlaceholderTemplataT`, `PrototypeTemplataT`, `IsaTemplataT`, `CoordListTemplataT` — one shared `templata_payload_val_to_ref` map keyed on `InternedTemplataPayloadValT<'s, 't, 'tmp>` (6 variants).
- `PrototypeT<'s, 't>` / `PrototypeValT<'s, 't, 'tmp>` — singleton map.
- `SignatureT<'s, 't>` / `SignatureValT<'s, 't, 'tmp>` — singleton map.

~84 per-concrete intern methods (caller-facing API) dispatch through those 6 family-level `intern_<family>` methods via four `impl_intern_*_wrapper_*` macros. Pattern mirrors `scout_arena.rs`. One hand-written wrapper (`intern_coord_list_templata`) for the single `'tmp`+slice case the macro can't express.

**Inline-owned, NOT interned** (wrapper enums — stack-only rewraps via `From`/`TryFrom`):
- `INameT` + 21 name sub-enums (`IFunctionNameT`, `IStructNameT`, etc.)
- `KindT` + 3 Kind sub-enums (`ICitizenTT`, `ISubKindTT`, `ISuperKindTT`)
- `ITemplataT`
- `IEnvironmentT` + `IInDenizenEnvironmentT` (Slab 4)
- `IEnvEntryT` (Slab 4 — 5 variants)
- `IVariableT` + `ILocalVariableT` (Slab 4)

## Notable post-Slab-4 state

### `TypingInterner<'s, 't>` is fully implemented

No more `panic!()` stubs. `src/typing/typing_interner.rs` (560 lines) has the 6 family-level HashMaps in `Inner<'s, 't>`, real `intern_<family>` bodies mirroring `scout_arena.rs::intern_rune` / `intern_name`, and ~84 per-concrete wrapper methods via macros. The struct takes `&'t Bump` and exposes `alloc` / `alloc_slice_copy` / `alloc_slice_from_vec` wrappers so builders can arena-alloc non-interned data without reaching through a separate `Bump` handle. **The type now carries two lifetime params `<'s, 't>`, not `<'t>`** — sub-compiler call sites all flipped to `&TypingInterner<'s, 't>` in Slab 4 Step 2.

### Env types are real and shipping

9 concrete env structs (`PackageEnvironmentT`, `CitizenEnvironmentT`, `FunctionEnvironmentT`, `NodeEnvironmentT`, `BuildingFunctionEnvironmentWithClosuredsT`, `BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT`, `GeneralEnvironmentT`, `ExportEnvironmentT`, `ExternEnvironmentT`), 2 wrapper enums (`IEnvironmentT`/`IInDenizenEnvironmentT`), `GlobalEnvironmentT`, `TemplatasStoreT` + `TemplatasStoreBuilder`, `IEnvEntryT` 5-variant enum, `IVariableT`/`ILocalVariableT` + 4 concrete variable structs, and 9 env-specific builders (`PackageEnvironmentBuilder` etc.). 17 `From`/`TryFrom` bridges between wrappers and concretes. `TemplatasStoreT` uses slice-of-pairs + nested-slice layout per AASSNCMCX (no `HashMap`-in-arena); linear-scan lookup.

Env `Hash`/`PartialEq`/`Eq` are **manual impls**, not derived — they match Scala's `id`-based identity (or `(id, life)` for `NodeEnvironmentT`, `panic!("vcurious")` for `GeneralEnvironmentT`). Deriving would walk into `TemplatasStoreT`'s slices and diverge from Scala.

### Heavy-templata env refs flipped to `&'t`

Slab 3 originally wrote `FunctionTemplataT.outer_env`, `Struct/InterfaceDefinitionTemplataT.declaring_env`, `ImplDefinitionTemplataT.env`, and `OverloadSetT.env` as `&'s`, matching the old §3.1 spec that placed envs in the scout arena. Slab 4 flipped all 5 to `&'t`. The Slab-3 custom `ptr::eq`-based `PartialEq`/`Eq`/`Hash` on heavy templatas stayed green through the flip (ptr-eq is arena-agnostic).

### Box stubs deleted

`NodeEnvironmentBox`, `FunctionEnvironmentBoxT`, and the `IDenizenEnvironmentBoxT` trait are gone. The builder-freeze pattern subsumes Scala's mutable-box role: mutation happens in a stack-local `*Builder`, `build_in(interner)` freezes into `'t` as an immutable `&'t`. Scala `/* */` blocks for the deleted stubs stay as audit trail (the pre-commit hook only checks block content).

### Val types use content-based Hash, not ptr-based

Slab 4 caught a subtle bug: `*ValT` types (the interner lookup keys) using `ptr::eq`-based `Hash` would have broken heterogeneous lookup. Two stack-local Vals with identical content would hash to different addresses and miss the HashMap. Switched to derived `Hash`/`PartialEq`/`Eq` (content-based) so query-Val (`'tmp`) and stored-Val (`'t`) hash consistently. The `ptr::eq` treatment stays for the *canonical `&'t` refs*, which is where pointer identity is genuinely the intent.

### Pre-commit hook on `/* scala */` blocks (unchanged)

`.claude/hooks/check-scala-comments` does exact-match comparison on every `/* ... */` Scala block. Any edit inside rejects the commit. Load-bearing for the migration audit trail — Scala source is embedded next to every Rust definition as the spec. Explain this to anyone new touching the code.

### Known residual items (post-Slab-4, pre-Slab-5)

- **`HinputsT`** is still a `// mig:` marker + Scala comment — no Rust stub. Slab 7.
- **Sub-compiler free-fn stubs** (`fn entry_matches_filter`, `fn entry_to_templata`, `fn lookup_with_name_inner`, etc.) in `env/environment.rs` and `env/function_environment_t.rs` — slice-pipeline artifacts. Slab 8 wires them into `Compiler` methods or deletes.
- **`dispatch_function_body_macro`** and friends on `Compiler` aren't wired yet — they need env-based macro resolution, which is Slab 8+ work.
- **`IInfererDelegate` trait** stays vestigial because `compiler_solver.rs` fn sigs still reference `&dyn IInfererDelegate`. Slab 8 signature rewrites remove it.
- **`LocationInFunctionEnvironmentT.path: Vec<i32>`** in `ast/ast.rs` violates AASSNCMCX (heap `Vec` inside a `'t`-arena-allocated conceptual type) and would leak if we ever run arena drop without a destructor pass. Not blocking current work; a future cleanup turns the `Vec` into `&'t [i32]`.
- **`NodeEnvironmentT`-downstream call sites** in some sub-compiler files (`local_helper.rs`, `array_compiler.rs`) got `panic!("Unimplemented: Slab 8")` patches in Slab 4 where the type of a `&NodeEnvironmentBox` param couldn't be meaningfully translated without body migration. Slab 8 resolves.
- **Typing storage → two-tier per-denizen arenas** is the scheduled post-Slab-8 redesign. Migration-phase uses a single `'t` interner; the reasoning doc describes the two-tier model (`'out` outputs arena + per-top-level-denizen `'scratch` scratchpad arenas, with `EnvIdx` handoff), the cross-denizen edge audit justifying the split, and an LSP trajectory built on top. See `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md`. Out of scope until the build is clean end-to-end.

## Key files / directories

| Path | Purpose |
|---|---|
| `quest.md` | design spec (with overrides above) |
| `TL-HANDOFF.md` | this doc |
| `FrontendRust/docs/architecture/typing-pass-arenas.md` | typing-pass arena architecture (current state + pointer to long-term reasoning) |
| `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` | two-tier per-denizen target + cross-denizen edge audit + LSP direction |
| `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md` | `IdT` monomorphic / typed-view decision |
| `FrontendRust/docs/reasoning/` | other design-decision docs (slice interning, arena maps, hook, output-data-ref-or-copy) |
| `FrontendRust/docs/migration/handoff-slab-2.md` | Slab 2 handoff (names) |
| `FrontendRust/docs/migration/handoff-slab-3.md` | Slab 3 handoff (Kind/Coord/Templata) |
| `FrontendRust/docs/migration/handoff-slab-4.md` | Slab 4 handoff (envs + interner bodies) — historical but the 16 Gotchas are reusable patterns |
| `FrontendRust/docs/migration/handoff-god-struct-progress.md` | Phase 2 god-struct refactor progress notes |
| `FrontendRust/src/typing/names/names.rs` | ~3100 lines: all name types, IdT, From/TryFrom bridges, ValT companions + INameValT union |
| `FrontendRust/src/typing/types/types.rs` | KindT + sub-enums + concrete Kind payloads + InternedKindPayloadValT/T unions |
| `FrontendRust/src/typing/templata/templata.rs` | ITemplataT + payload structs + InternedTemplataPayloadValT/T unions |
| `FrontendRust/src/typing/ast/ast.rs` | PrototypeT, SignatureT, IdT-holding structs + IDEPFL Query wrappers |
| `FrontendRust/src/typing/ast/expressions.rs` | Slab 5 target — 3 enums + ~44 payload structs still at `_Phantom` |
| `FrontendRust/src/typing/env/environment.rs` | 5 of 9 env types + wrapper enums + GlobalEnvironmentT + TemplatasStoreT + 6 builders |
| `FrontendRust/src/typing/env/function_environment_t.rs` | 4 of 9 env types + 4 builders + IVariableT/ILocalVariableT + 4 concrete variables |
| `FrontendRust/src/typing/env/i_env_entry.rs` | IEnvEntryT 5-variant enum |
| `FrontendRust/src/typing/typing_interner.rs` | 560 lines: 6-family HashMap design, ~84 per-concrete wrappers via macros |
| `FrontendRust/src/typing/compiler.rs` | god struct (`Compiler<'s, 'ctx, 't>`, 4 fields) |
| `.claude/hooks/check-scala-comments` | pre-commit hook guarding `/* scala */` blocks |
| `Luz/shields/` | project-wide shields (RSMSCPX, NCWSRX, ATDCX, IDEPFL, etc.) |

## How to continue

1. **Read this doc + `quest.md`** (with the overrides in mind). Also read `docs/architecture/typing-pass-arenas.md` for the current arena shape and `docs/reasoning/environments-per-denizen-long-term.md` for the long-term target — the latter records the cross-denizen audit that validates the target and is worth understanding before any future storage-layer work.
2. **Start Slab 5 (expression AST)**. Design spec is `quest.md` Part 7 — accurate, no overrides known. First task is to draft `FrontendRust/docs/migration/handoff-slab-5.md` in the Slab 2/3/4 style before handing off to a junior:
   - 3 enums: `ReferenceExpressionTE` (~38 variants), `AddressExpressionTE` (~6 variants), `ExpressionTE` wrapper.
   - Per-variant payload structs (~44 total).
   - Arena-allocated in `'t` (not interned — see §7.2).
   - `NodeRefT` visitor pattern + `visit_*` / `collect_*` macros per §7.
   - Narrow-use rule: default to `&'t ReferenceExpressionTE`/`&'t AddressExpressionTE`; only `ConstructTE` uses the broad `ExpressionTE` wrapper.
   - `ILocalVariableT<'s, 't>` is already real (Slab 4), so `LetNormalTE`/`UnletTE`/`LetAndLendTE`/`LocalLookupTE`/`RestackifyTE` unblock on fill-in.
3. **Don't commit.** The human handles all commits and tags. Hand back uncommitted working trees at slab boundaries; the human reviews and commits.
4. **Each subsequent slab** gets its own handoff doc. Tag naming for the human: `slab-N-complete`.

## Suggested process for the incoming TL

- Spawn a junior for each slab. Write them a detailed handoff doc (Slab 2/3/4 style — prescriptive, Gotchas, references to shields + reasoning docs). Slab 4's 16-gotcha format turned out to be a good template — pre-answering design questions before the junior codes saves rework.
- Answer design questions the junior raises in the handoff *before* they code. Slab 3 Gotcha 1 (the KindT inline-vs-interned question) and Slab 4 Gotcha 1 (`'t` vs `'s` for envs) are examples of this done well — the answer was baked into the doc, not deferred to review.
- When a design diverges from `quest.md`, record the divergence in `FrontendRust/docs/reasoning/<topic>.md` with the chosen approach + alternatives considered + why-deferred. Mirror `idt-typed-view-alternatives.md` or `environments-per-denizen-long-term.md` shapes.
- **Never commit.** Juniors and TLs finish a slab, run `cargo check --lib` clean, self-review their diff, then hand back to the human for review with uncommitted changes in the working tree. The human reviews, directs any changes, and is the one who runs `git commit` / `git tag`. If a junior needs a local savepoint mid-slab, use `git stash` or a WIP branch; don't commit on `rustmigrate-z`. Handoff docs may describe work-organization "steps" or "checkpoints" — those are self-review savepoints, not commit points.
- **Expect and invite push-back.** Slab 4 had two rounds of significant TL corrections — one on interner design (per-concrete maps → 6 family-level maps), one on a family of box stubs the handoff missed. Both came from the junior noticing something was off. Reward the instinct; handoffs are proposals, not spec.
- After Slab 8 the build should be clean with `panic!()` bodies. Slab 9+ becomes test-driven; the shape of that work is different — per-method instead of per-type-family.

## Risks / open questions

- **LSP / long-running use**: scout arena retention through instantiation is memory-heavy; single-arena typing makes batch-only the safe mode. The reasoning doc's "Further future direction: LSP support" section sketches per-denizen arenas + DefId cross-denizen refs + local/global split interner + single-writer cleanup promotion as the evolution path. Concrete design deferred past the two-tier per-denizen refactor.
- **Post-migration design revisits**: the inline-owned-wrapper philosophy (Slab 2/3) makes casts free but gives up compile-time "this IdT's local_name is a FunctionName" assertions. `idt-typed-view-alternatives.md` lists four post-migration ways to re-introduce that. Not pressing.
- **`HinputsT` shape**: deferred to Slab 7; currently a bare marker. If field set grows during Slab 6 (CompilerOutputs) — e.g. adding an impl-to-edge cache map — revisit before Slab 7 kicks off.
- **`TypingInterner` perf**: six `hashbrown::HashMap` maps with heterogeneous `'tmp`→`'t` lookup via `Equivalent`. Works today but hasn't been measured. If typing becomes the bottleneck at scale, profile before optimizing; the two-tier per-denizen redesign might be the right place to revisit.

Questions? `quest.md` + the reasoning docs + the Slab 2/3/4 handoffs have the context. When in doubt, prefer Scala parity over Rust-idiomatic optimization — that's been the guiding principle through Slabs 2–4.
