# Migration Policy

This is the **single central migration policy** for the whole frontend. It covers every pass; there is no per-pass policy file. The slice-pipeline is pass-agnostic by default and produces wrong defaults for any pass that has non-trivial arena/lifetime/interning conventions, so `slice-rustify`, `slice-placehold`, and `slice-impl-wrap` read this file before emitting anything and use the values for the target file's pass. The orchestrator skill (`docs/skills/slice-pipeline.md`) points the agents here.

Each schema section below lists its value per pass (e.g. Type-name suffix: typing `T`, postparsing `S`, …). When a pass isn't listed in a section, that section doesn't apply to it yet — add its value here when the pass enters the pipeline. The typing-pass values (which the slabs-15 migration converged on) are the most complete and serve as the canonical example.

---

## Per-pass quick reference

The core values for each pass. The schema sections below expand on these; the deeper per-pass lists (dual-enum pairs, builder/frozen pairs, naming exceptions) are filled in as each pass enters the pipeline.

| Pass | Source dir | Suffix | Lifetimes | Interner / Arena | Collections | String |
|---|---|---|---|---|---|---|
| parsing | `src/parsing/` | `P` | `'p` | `ParseArena<'p>` | `&'p [X]` | `StrI<'p>` |
| postparsing | `src/postparsing/` | `S` | `'s` | `ScoutArena<'s>` | `&'s [X]`, `IndexMap` | `StrI<'s>` |
| higher_typing | `src/higher_typing/` | `A` | `'s` | `ScoutArena<'s>` | `&'s [X]` | `StrI<'s>` |
| typing | `src/typing/` | `T` | `'s, 't` | `&'ctx TypingInterner<'s,'t>` | `&'t [X]`, `ArenaIndexMap<'t,…>` | `StrI<'s>` |
| instantiating | `src/instantiating/` | `I` | `'s, 'i` (+ region-mode generic `R: IRegionsModeI`) | `InstantiatingInterner<'s,'i>` / `InstantiatingArena<'i>` | `&'i [X]`, `ArenaIndexMap` | `StrI<'s>` |
| simplifying + final_ast | `src/simplifying/`, `src/final_ast/` | `H` | `'s, 'h` | `HammerInterner<'s,'h>` / `HammerArena` | `&'h [X]` | `StrI<'s>` |
| testvm | `src/TestVM/` | `V` | `'v` (reads `ProgramH` → borrows `'h`/`'s`) | `VivemArena<'v>` / `VivemInterner<'v>` (refcounted internally) | `&'v [X]`; mutable heap = owned `Vec`/`HashMap` + `Cell` | `StrI<'s>` |
| integration_tests | `src/integration_tests/` | (test classes: none) | n/a — `#[test]` module-scope fns | n/a (assert on frontend output only) | n/a | n/a |

Notes:
- **instantiating** carries an extra region-mode type parameter `R: IRegionsModeI` on most types (`Foo<'s,'i,R>`) — the T-erased region representation (`IdI<'s,'i,R>`). It is a generic *type* param, not a lifetime.
- **simplifying** emits the final AST (the `H`-suffixed types in `src/final_ast/`) and uses the `MustIntern` seal. Per typing-pass precedent the `VonHammer` and `HamutsBox` compiler classes were collapsed onto `Hammer`/`Hamuts` — there is no separate `VonHammer`/`HamutsBox` state.
- **integration_tests** is a test-only module: its `.rs` files are mostly `#[test]` fns that assert on frontend pipeline output (compile-succeeds / expected hamuts), not on executed programs. Test classes (`class FooTests extends FunSuite`) get **no** type suffix and are not impl-wrapped.
- **testvm** is the Vivem reference interpreter (`src/TestVM/`) — unlike every other (immutable-AST) pass it has a **mutable runtime heap**. It uses a dedicated `VivemArena<'v>` and reads the `ProgramH` final AST (so it borrows `'h`/`'s`). Mutation goes through **interior mutability**, not `&mut`: per architect decision `Cell` is used heavily — every refcount lives in a `Cell`, and value fields are usually enums containing a `Cell`. So a Scala `var` field → a Rust `Cell<…>` field, and a `this`-mutating Scala `def` stays a `&self` Rust method (no `&mut self` collapse like the typing pass). A refcount free-list for manual GC is **deliberately deferred** — do **not** build it during slicing or body migration; it lands later as explicit Rust-only arena infra.

---

## Policy schema

A pass policy is a single Markdown file with the following H2 sections. Every section must be present; "(none)" is a valid value where it makes sense.

### Pass name
Short identifier used in agent output, e.g. `typing`, `instantiating`, `simplifying`.

### Source dir
Pass-local source root, e.g. `FrontendRust/src/typing/`.

### Type-name suffix
Single ASCII letter appended to every Scala type name on translation. Typing: `T`. Postparsing: `S`. Parsing: `P`. Higher-typing: `A`. Instantiating: `I`. Simplifying / final AST: `H`. Testvm: `V` (the Vivem runtime-value types — `ReferenceV`, `PrimitiveKindV`, etc. — already carry it). If a Scala class name already carries this suffix, it stays unchanged; otherwise the agent appends it. Applies to `struct`, `enum`, `trait` mig comments. **Test classes** (`class FooTests extends FunSuite`) get no suffix.

### Lifetimes
Ordered list of lifetime parameters every `struct`/`enum`/`trait`/`impl` carries by default, e.g. `'s, 't`. Plus any default `where`-clause (e.g. `where 's: 't`). slice-placehold emits these on every type definition and impl block unless the policy says otherwise for a specific case. Per pass: parsing `'p`; postparsing/higher_typing `'s`; typing `'s, 't`; instantiating `'s, 'i` plus the region-mode type generic `R: IRegionsModeI`; simplifying/final_ast `'s, 'h`; testvm `'v` (plus `'h`/`'s` where it touches the `ProgramH` it reads).

### Interner type
The arena/interner type that body-emitting methods take as a parameter. Typing: `&'ctx TypingInterner<'s, 't>`. Postparsing/higher_typing: `&'ctx ScoutArena<'s>`. Parsing: `&'ctx ParseArena<'p>`. Instantiating: `&'ctx InstantiatingInterner<'s, 'i>` (arena `InstantiatingArena<'i>`). Simplifying/final_ast: `&'ctx HammerInterner<'s, 'h>` (arena `HammerArena`). Testvm: `&'ctx VivemInterner<'v>` (arena `VivemArena<'v>`, refcounted internally). The agent threads this as an extra leading parameter on every method whose Scala body calls `interner.intern(...)` — i.e. methods that *produce* an interned value. (Sanctioned under SPDMX exception B.)

### Default collection types
Scala → Rust mapping for collection literals. Typing pass:
- `Vector[X]` → `&'t [X]` (arena slice). `List[X]` → `&'t [X]`. `Array[X]` → `&'t [X]`.
- `Map[K, V]` → `ArenaIndexMap<'t, K, V>` (insertion-ordered, arena-keyed). `mutable.Map` → builder `IndexMap`.
- `Set[X]` → `ArenaIndexSet<'t, X>`.
- Postparsing differs (uses `&'s [X]` and `IndexMap<...>` on the ScoutArena).
- Testvm: stable/AST-derived data → `&'v [X]`. The **mutable runtime heap** uses owned `Vec`/`HashMap` (Scala `mutable.HashMap`/`mutable.ArrayBuffer`), with mutated cells wrapped in `Cell` (see the testvm Notes bullet).

If the default is wrong for a specific stub, the agent leaves it and a manual fix happens in body migration — but the default needs to be right *most* of the time.

### String type
Scala `String` → … . Typing pass: `StrI<'s>` (interned). Testvm: `StrI<'s>` (inherited from the `ProgramH`/final_ast names it reads). Most occurrences of `String` in a Scala signature are an interned identifier, not a free string.

### Sealed-trait policy
How `sealed trait Foo` translates. Options:
- `enum-with-arena-refs`: `pub enum Foo<'s,'t> { Variant1(&'t Variant1Payload<'s,'t>), … }`. Each variant holds an arena-allocated payload. Used in typing pass.
- `enum-with-box`: `pub enum Foo { Variant1(Box<Variant1Payload>), … }`. For non-arena passes.
- `trait-with-impls`: keep as Rust `trait` with concrete `impl Foo for Variant1` blocks. Rare; only when no closed-set guarantee is needed.

The policy file must also state: *does the sealed trait become Polyvalue?* (i.e. should the enum `#[derive(PartialEq, Eq, Hash, Clone, Copy)]`?) — per @PVECFPZ.

Testvm: `enum-with-arena-refs` on `VivemArena<'v>`. The Vivem value/kind hierarchies (e.g. `KindV`, `ReferenceV`) become enums; because the heap is mutable, variant payload fields that Scala declared `var` are wrapped in `Cell` (see the testvm Notes bullet). Not Polyvalue.

### Abstract-def dispatcher policy
When a `sealed trait` declares abstract `def`s, what to emit. Options:
- `dispatcher-on-enum`: emit `impl Foo { pub fn method(&self, ...) -> ... { match self { Foo::Variant1(p) => p.method(...), … } } }` on the enum, plus per-variant impl blocks. Default for typing pass.
- `dispatcher-with-panic-arms`: emit dispatcher with `panic!("Unimplemented: variant.method")` arms — variants get filled in as test paths hit them. Used when variants are stubbed late.
- `skip`: don't emit dispatcher (caller will pattern-match directly).

Slice-placehold annotates emitted dispatchers with `/* Guardian: disable-all */` because they have no 1:1 Scala counterpart (dispatcher generation is a Rust adaptation of Scala's virtual call).

### `equals` / `hashCode` / `unapply` policy
- `override def equals` → realized via `impl PartialEq for FooT`, not a `pub fn eq`. slice-placehold emits a marker stub:
  ```rust
  // mig: fn eq
  // (Realized by `impl PartialEq for FooT` below.)
  /*
    override def equals(other: Any): Boolean = ...
  */
  ```
- `override def hashCode` → realized via `impl Hash for FooT`. Same marker stub pattern.
- `def unapply` → realized via `TryFrom` / pattern-match. Marker stub:
  ```rust
  // mig: fn unapply
  // (Realized via `impl TryFrom<Wide> for Narrow` or inline match.)
  ```

### Identity equality (`@IEOIBZ`) policy
Which types use `ptr::eq` identity vs structural derive:
- All types carrying `MustIntern` (per @SICZ): `impl PartialEq` via `ptr::eq` on the canonical pointer; `impl Hash` likewise.
- Polyvalue wrapper enums (per @PVECFPZ): `#[derive(PartialEq, Eq, Hash, Clone, Copy)]`.
- Value-type / non-interned: `#[derive(PartialEq, Eq, Hash)]` (no Copy unless explicit).

### `case object` / companion `object` policy
- `case object Foo` (Scala unit variant of a sealed trait) → unit variant on the Rust enum: `Foo(())`. Not a separate struct.
- `object Foo` (companion object with static defs) → associated `fn`s on the corresponding `impl Foo`. Free fns at module scope are wrong.
- `object Foo` (pure namespace, no companion class) → module-level `pub fn`s in a submodule named `foo`. Rare.

### `MustIntern` seal policy
Whether the pass uses the `MustIntern` seal pattern (per @SICZ) for its arena-allocated types:
- Typing pass: yes. Every struct that becomes a payload of an interned enum variant carries `pub _must_intern: MustIntern` as a private-module-only field.
- slice-placehold emits the seal field on every struct that the policy classifies as interned. The placehold step doesn't *decide* — it copies what the policy says, and the policy file enumerates interned types by name regex (e.g. `^I[A-Z].*T$` for typing-pass `IFooT` interfaces, plus the named-type whitelist below).

### Val/Ref dual-enum pairs (IDEPFL)
List of interned-enum families that need a transient `*ValT` companion enum + per-variant `*ValT` payloads. Each pair entry: `(canonical enum name, ValT enum name, interner method)`. slice-placehold emits both enums skeleton-wise; bodies stay panic-stubs.

Typing pass:
- `IdT<'s,'t>` / `IdValT<'s,'t,'tmp>` / `typing_interner.intern_id`
- `INameT<'s,'t>` / `INameValT<'s,'t,'tmp>` / `typing_interner.intern_name`
- `ITemplataT<'s,'t>` / `ITemplataValT<'s,'t,'tmp>` / `typing_interner.intern_templata`
- (plus per-variant sub-pairs — see `docs/architecture/typing-pass-design-v3.md` §6)

### Arena-classification doc-comment
Every type definition gets one of:
- `/// Arena-allocated` — held by `&'t Foo` references; constructed only via interner; identity by `ptr::eq`.
- `/// Temporary state` — held by value or `Box`; constructed freely; identity by structural eq.
- `/// Polyvalue` — closed-set tagged-pointer enum; `#[derive]`s identity.

slice-placehold emits one of these above every struct/enum based on the policy classification.

### Builder/Frozen pair policy
List of Scala types that split into a Builder + Frozen pair in Rust. Each entry: `(Scala name, Builder Rust name, Frozen Rust name)`. slice-placehold emits both with parallel method skeletons; bodies stay panic-stubs and *must be reviewed against the single Scala source side-by-side* (TL.md recurring-bug-class B).

Typing pass:
- `TemplatasStore` → `TemplatasStoreBuilder` + `TemplatasStoreT`
- `NodeEnvironmentBox` → `NodeEnvironmentBuilder` + `NodeEnvironmentBoxT`
- (full list in design-v3 §3.2)

### Default fn skeleton
What slice-placehold emits as the body of a freshly-stubbed `fn`. Options:
- `whole-panic`: `{ panic!("Unimplemented: foo"); }`. Default; safest. Trips SPDMX when refined to skeleton-with-panics later.
- `iteration-skeleton`: mirror Scala's outer iteration shape (`.map(|_| panic!())`, `for x in xs { panic!() }`) — only for fns whose Scala body is a single `.map`/`.foreach`/`groupBy` chain. Per TL.md §"Good Partial Implementing" + the SPDMX rationale boilerplate. **TL/architect-only**: emitting iteration-skeleton requires the policy file to whitelist the fn by name, since SPDMX will fire and need a temp-disable.

Typing pass: default `whole-panic` everywhere; iteration-skeleton whitelist is empty (filled in as TL handles SPDMX escalations).

Testvm: default `whole-panic` everywhere; iteration-skeleton whitelist is empty.

### Naming exceptions (SPDMX exception J)
Pre-approved Scala → Rust renames, for cases where the literal translation collides or reads badly. Each entry: `Scala name → Rust name`. slice-rustify applies these instead of the default snake_case conversion.

Typing pass:
- `lookupFunctionByHumanName` → `lookup_function_by_str`
- (extend as the architect approves more)

---

## Conventions & lessons learned

Durable lessons from the migration so far. These govern how slicing and body migration should be done; see also `TL.md`, `docs/architecture/typing-pass-ai-guide.md`, the `guardian-tl` skill, and the shields (SPDMX, SCPX, DCCR, PSMONMX) for the authoritative rules.

### `vimpl`/`vfail`/`vwat` → `panic!` is faithful, not a gap
Scala's `vimpl()`, `vfail()`, `vwat()`, `vcurious()`, and a failed `vassertSome` translate to Rust `panic!(...)`. These are correct 1:1 translations. A high panic count in a migrated file is mostly faithful translation, **not** incompleteness — distinguish genuine not-yet-migrated stubs (`panic!("Unimplemented: …")` / `"Unmigrated …"`) from faithful `vimpl`-style panics when gauging progress.

### Match Scala structurally, not just behaviorally
SPDMX checks shape, not only behavior. Mirror Scala's control structure: a single `match` with destructured patterns → a single Rust `match` (not nested matches, per PSMONMX); `flatMap`/`flatMap` chains → `filter_map`/`and_then` (not a combined helper); `xs ++ ys` → `iter().chain().collect()` (not mutate-then-extend). Matching behavior alone leaves latent SPDMX hits.

### When canonical throws and Rust silently succeeds, the Rust is wrong
If canonical Scala throws an error on a path where the Rust silently wraps/succeeds (or vice versa), the Rust diverged. Align to Scala. This may break tests that were asserting the wrong-Rust behavior — **update or delete those tests to match canonical's test set** (the test corpus is the spec). This is the DCCR-sanctioned exception: changing a test because it encoded wrong behavior, with canonical as the witness. A test passing on wrong-Rust behavior is worse than one failing on correct-Rust behavior.

### DCCR architect escape-hatch: edit the Scala first, don't annotate a divergence
When the Scala carries dead-weight machinery whose mutation/dispatch surface is unused on the call paths being ported (`FunctionEnvironmentBoxT`'s `setReturnType`/`addEntry` mutators on read-only paths is the canonical case), do **not** write the Rust without it and add a "diverges from Scala" note — those rot, and reviewers can't verify the divergence. Instead: edit the Scala source first to match what the Rust will become, update the Rust audit-trail `/* ... */` blocks to reflect the new Scala, then make the Rust change. Only valid when the wrapper is unused on the ported paths (verify with `grep`), the replacement is design-doc-blessed, and SCPX `--check-all` still passes after. **TL/architect-level only — juniors must escalate.**

### Unmigrated tests are `#[ignore]`'d panic-stubs
A not-yet-migrated test is `#[test] #[ignore = "unmigrated - pending <pass> body migration"] fn x() { panic!("Unmigrated test: x"); }`. An unconditional `panic!` **without** `#[ignore]` fails the suite — always ignore them. The body-migration loop un-ignores one at a time, fills the panicked paths it hits, and repeats.

### Generic-parameter ordering (@PRIIROZ)
`genericParametersS` and equivalents concatenate as: user-specified ++ extra-from-explicit-params ++ extra-from-body ++ extra-from-parent (**parent last**).

### Orphan-file traps
A `.rs` file that is (a) not declared in any `mod.rs`/`lib.rs`, or (b) contains bare Scala not wrapped in `/* */`, is invisible to **cargo** (not compiled), **SCPX** (no audit-trail block → no mismatch reported), and **slice-pipeline** (nothing to anchor on). Before slicing a module: ensure every file is `/* */`-wrapped *and* the module is wired into the crate. (This is exactly how 13 `simplifying/` files and the entire `integration_tests/` module silently slipped past every check.)

### Cross-pass wiring during mid-migration
When an orchestration type (e.g. `pass_manager`/`FullCompilation`) must construct a type from a downstream pass that is still a mid-migration stub (no real constructor, possibly extra lifetimes), **stub the seam**: hold the dependency as `PhantomData` (keeping the lifetimes live), and `panic!` in the constructor and any delegating methods, until the downstream pass's body migration provides a real constructor. Don't force a premature real wiring.

### Combining parallel migration branches
When two branches migrate different passes against forked `Frontend/` Scala: do a **scoped transplant** of the owning branch's pass directories (plus their support files and any new crate-level deps, e.g. `paste`, wiring an orphaned `final_ast` module), **not** a full `git merge` — a blanket merge drags the whole-frontend slice-pipeline stubs and regresses already-completed passes back to stubs. After transplant, drift-reconcile the transplanted passes' audit-trail to whichever `Frontend/` is canonical. Pick **one** canonical `Frontend/` (the one that's ahead) and have both branches' body-migration target it.

## How the agents consume this file

`slice-rustify`, `slice-placehold`, and `slice-impl-wrap` read this central file (`FrontendRust/docs/migration/migration-policy.md`) before emitting anything, and look up the values for the target file's pass (identified by its source dir, e.g. `src/typing/` → typing). If a pass has no values in a section, the agent treats that section as not-applicable for that pass.

The file is read *once per agent invocation* — the agent stuffs the relevant pass's values into its working context and references them in every emit decision. Agents are explicitly told (in their own .md files) not to deviate from the policy without architect approval.

---

## Adding a new pass

1. In each H2 section above, add the new pass's value alongside the existing passes (e.g. add a `Simplifying: H` line to **Type-name suffix**). Cover every section that applies; sections that don't apply to the pass can be omitted for it.
2. Get architect sign-off on the new pass's values *before* running the pipeline. The cost of running the pipeline with wrong values is high (~32 orphan free fns in `src/typing/` is the typing-pass cost).
3. As the migration proceeds and new patterns surface, edit this central file rather than special-casing individual files. This file is canonical.
