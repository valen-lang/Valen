# Typing Pass Migration — TL Handoff

**Taking this over?** Read this doc first, then `quest.md` (with the overrides flagged below). Everything else is directory-local.

## One-paragraph orientation

We're porting `src/typing/` from Scala to Rust, slab by slab per `quest.md` §12. The typing pass holds ~60 concrete name types + ~70 concrete payload types, plus a god-struct `Compiler<'s, 'ctx, 't>` that replaces Scala's ~20 sub-compilers and 15 macros. Scout-pass data (`FunctionA`, `StructA`, interned strings/names) is arena-retained and referenced directly from typing output via `&'s`; typing-pass arena-allocated types carry `<'s, 't>`. Slabs 0–3 are done; Slab 4 (environments) is the next piece. `cargo check --lib` is clean.

## Where we are

| Slab | Scope | Status | Tag / handoff |
|---|---|---|---|
| 0 | arena substrate | ✅ | (scaffolding; no tag) |
| 1 | leaf types (OwnershipT, primitive KindT payloads, leaf templatas) | ✅ | commit `9fd7641c` |
| 2 | name hierarchy (~60 concrete names, 22 sub-enums, IdT, ValT companions) | ✅ | `slab-2-complete` · `FrontendRust/docs/migration/handoff-slab-2.md` |
| 3 | Kind / Coord / Templata trio | ✅ | `slab-3-complete` · `FrontendRust/docs/migration/handoff-slab-3.md` |
| **4** | **environments** | **⏳ next** | write handoff-slab-4.md; design spec is `quest.md` Part 3 |
| 5 | expression AST | ⏳ | `quest.md` Part 7 |
| 6 | CompilerOutputs | ⏳ | `quest.md` Part 4 |
| 7 | HinputsT + Compiler shell + run_typing_pass | ⏳ | `quest.md` Part 10 |
| 8 | method signatures, clean `cargo build --lib` | ⏳ | |
| 9+ | method bodies, test-driven | ⏳ | |

## Design decisions that *override* `quest.md`

`quest.md` predates two significant refactors we did during Slabs 2–3. If the doc and the code disagree, **the code and the reasoning doc win**. Sections that are out-of-date:

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

### `quest.md` §6.1 & §1.5 — corrected interned/inline family lists

The refactored families per IDEPFL after Slabs 2 and 3:

**Interned (dedup via `TypingInterner`):**
- ~60 concrete name structs
- `IdT<'s, 't>` (monomorphic) / `IdValT<'s, 't, 'tmp>`
- 6 concrete Kind payloads (`StructTT`, `InterfaceTT`, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `KindPlaceholderT`, `OverloadSetT`)
- 6 interned templata payloads (`CoordTemplataT`, `KindTemplataT`, `PlaceholderTemplataT`, `PrototypeTemplataT`, `IsaTemplataT`, `CoordListTemplataT`)
- `PrototypeT<'s, 't>` / `PrototypeValT<'s, 't, 'tmp>`
- `SignatureT<'s, 't>` / `SignatureValT<'s, 't, 'tmp>`

**Inline-owned, NOT interned** (wrapper enums — stack-only rewraps via `From`/`TryFrom`):
- `INameT` + 21 name sub-enums (`IFunctionNameT`, `IStructNameT`, etc.)
- `KindT` + 3 Kind sub-enums (`ICitizenTT`, `ISubKindTT`, `ISuperKindTT`)
- `ITemplataT`

quest.md §1.5 was partially updated through Slab 2; §6.1 still shows ITemplataT as interned. I've patched both in this handoff commit.

## Notable state that isn't in `quest.md`

### TypingInterner is panic-stubs only

`src/typing/typing_interner.rs`: all `intern_*` method bodies are `panic!()`. The Val type signatures are in place (Slab 2 Step 6 + Slab 3), but no real HashMap/bump-alloc implementation yet. Slab 4 touches this — envs need interned names to be constructible — so Slab 4 is likely where the first real interner bodies land. If that gets punted, Slab 7 definitely needs it.

### Slab 3 patched `env/environment.rs` stubs

To let `OverloadSetT` derive `Hash`/`Eq`/`PartialEq`/`Debug`, Slab 3 added those derives to the `IEnvironmentT` / `IInDenizenEnvironmentT` trait/enum stubs. **Slab 4 will replace those stubs with real enums per §3.1** — make sure the new enums derive the same set, or OverloadSetT breaks.

### Heavy-templata custom pointer-identity Eq/Hash

Slab 3 added custom `PartialEq`/`Eq`/`Hash` impls on `FunctionTemplataT`, `StructDefinitionTemplataT`, `InterfaceDefinitionTemplataT`, `ImplDefinitionTemplataT`, `ExternFunctionTemplataT` because their scout refs (`FunctionA` etc., `FunctionHeaderT`) don't derive those traits. The impls delegate to `std::ptr::eq` on the refs — correct because the scout arena canonicalizes `FunctionA` etc. Worth flagging in a future reasoning doc if the design ever changes.

### Pre-commit hook on `/* scala */` blocks

`.claude/hooks/check-scala-comments` does exact-match comparison on every `/* ... */` Scala block. Any edit inside rejects the commit. This is load-bearing for the migration audit trail — the Scala source is embedded next to every Rust definition as the spec. Explain this to anyone new touching the code.

### Known deferred items (pre-Slab-4)

- `HinputsT` is just a `// mig:` marker + Scala comment — no Rust stub. Slab 7 territory.
- Sub-compiler methods have dangling free `fn equals` / `fn hash_code` stubs from the slice pipeline. Wrap or delete in Slab 8.
- `dispatch_function_body_macro` / friends on `Compiler` are not wired yet — add when env lookup works (Slab 4 or later).
- `IInfererDelegate` trait stays vestigial because `compiler_solver.rs` fn sigs still reference `&dyn IInfererDelegate`. Remove during Slab 8 signature rewrites.

## Key files / directories

| Path | Purpose |
|---|---|
| `quest.md` | design spec (with overrides above) |
| `TL-HANDOFF.md` | this doc |
| `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md` | IdT/wrapper-enum design decision, records alternatives for post-migration |
| `FrontendRust/docs/reasoning/` | other design-decision docs (slice interning, arena maps, check-scala-comments hook, output-data-ref-or-copy) |
| `FrontendRust/docs/migration/handoff-slab-2.md` | Slab 2 handoff (~60 concrete names + sub-enums + IdT + From/TryFrom bridges + ValT) |
| `FrontendRust/docs/migration/handoff-slab-3.md` | Slab 3 handoff (KindT/Coord/Templata trio + IDEPFL Vals) |
| `FrontendRust/docs/migration/handoff-god-struct-progress.md` | Phase 2 god-struct refactor progress notes |
| `FrontendRust/src/typing/names/names.rs` | ~2600 lines: all name types, IdT, From/TryFrom bridges, ValT companions |
| `FrontendRust/src/typing/types/types.rs` | KindT + sub-enums + concrete Kind payloads |
| `FrontendRust/src/typing/templata/templata.rs` | ITemplataT + payload structs |
| `FrontendRust/src/typing/ast/ast.rs` | PrototypeT, SignatureT, IdT-holding structs (ImplT, EdgeT, FunctionHeaderT, etc.) |
| `FrontendRust/src/typing/typing_interner.rs` | panic-stub interner; Val type signatures live here, method bodies don't exist yet |
| `FrontendRust/src/typing/compiler.rs` | god struct |
| `.claude/hooks/check-scala-comments` | pre-commit hook guarding `/* scala */` blocks |
| `Luz/shields/` | project-wide shields (RSMSCPX, NCWSRX, ATDCX, IDEPFL, etc.) |

## How to continue

1. **Review this doc + `quest.md`** (with the overrides in mind).
2. **Optional doc cleanup**: fold the reasoning-doc "chosen" decisions back into `quest.md` §§6.3/6.5/6.6 and trim the reasoning doc to just the deferred alternatives. Non-blocking.
3. **Start Slab 4**: write `FrontendRust/docs/migration/handoff-slab-4.md` matching the Slab 2/3 style. Design spec is `quest.md` Part 3. Key subtlety: **§3.1 wants `trait IEnvironmentT` converted into a real enum** (9 variants). Slab 3 patched the stubs with derives; make sure the new enum carries them. Expect a full workday for Slab 4 — environments are structural, not just data definitions.
4. **Each subsequent slab** gets its own handoff doc, committed + tagged on completion. Keep `slab-N-complete` tags consistent.

## Suggested process for the incoming TL

- Spawn a junior for each slab. Write them a detailed handoff doc (Slab 2/3 style — prescriptive, lists Gotchas, references shields + reasoning docs).
- Answer design questions the junior raises in the handoff *before* they code, not during — saves rework. Gotcha 1 in the Slab 3 handoff is an example of this done well (decision captured, not deferred).
- When a design diverges from `quest.md`, record the divergence in `FrontendRust/docs/reasoning/<topic>.md` with the chosen approach + alternatives considered + why-deferred. Mirror the `idt-typed-view-alternatives.md` shape.
- After Slab 8 the build should be clean. Slab 9+ becomes test-driven; the shape of that work is different — more per-method instead of per-type-family.

## Risks / open questions

- **Post-migration design revisits**: the inline-owned-wrapper philosophy makes casts free but gives up compile-time type-safe "this IdT's local_name is a FunctionName" assertions. `idt-typed-view-alternatives.md` lists four ways to re-introduce that post-migration. Worth the eventual discussion; not pressing.
- **LSP / long-running use**: §Part 13 in quest.md flags this — scout arena retention through instantiation is memory-heavy. Batch compilation for now.
- **Interner implementation**: not yet designed beyond the panic-stub API. Someone needs to think about whether to use `bumpalo::Bump` + hashbrown, or something fancier. Worth a reasoning doc when it's tackled.
- **`HinputsT` shape**: deferred to Slab 7; currently a bare marker. If field set grows (e.g. adding an impl-to-edge cache map or similar), revisit before Slab 7.

Questions? `quest.md` + the reasoning docs + the Slab 2/3 handoffs have the context. When in doubt, prefer Scala parity over Rust-idiomatic optimization — that's been the guiding principle through Slabs 2–3.
