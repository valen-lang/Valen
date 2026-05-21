# Per-Pass Migration Policy

The slice-pipeline is pass-agnostic by default and produces wrong defaults for any pass that has non-trivial arena/lifetime/interning conventions. Each pass that wants slice-pipeline support drops a `migration-policy.md` next to its source root (e.g. `src/instantiating/migration-policy.md`). `slice-rustify` and `slice-placehold` read this file before emitting anything; the orchestrator skill (`docs/skills/slice-pipeline.md`) points the agents at it.

The file below is the **template + canonical example** (filled in with the typing-pass values that the slabs-15 migration converged on). Copy it into a new pass directory, swap the values, and the agents will pick it up.

---

## Policy schema

A pass policy is a single Markdown file with the following H2 sections. Every section must be present; "(none)" is a valid value where it makes sense.

### Pass name
Short identifier used in agent output, e.g. `typing`, `instantiating`, `simplifying`.

### Source dir
Pass-local source root, e.g. `FrontendRust/src/typing/`.

### Type-name suffix
Single ASCII letter appended to every Scala type name on translation. Typing pass: `T`. Postparsing: `S`. Parsing: `P`. Higher-typing: `A`. If a Scala class name already carries this suffix, it stays unchanged; otherwise the agent appends it. Applies to `struct`, `enum`, `trait`, `impl` mig comments.

### Lifetimes
Ordered list of lifetime parameters every `struct`/`enum`/`trait`/`impl` carries by default, e.g. `'s, 't`. Plus any default `where`-clause (e.g. `where 's: 't`). slice-placehold emits these on every type definition and impl block unless the policy file says otherwise for a specific case.

### Interner type
The arena/interner type that body-emitting methods take as a parameter. Typing pass: `&'ctx TypingInterner<'s, 't>`. Postparsing: `&'ctx ScoutArena<'s>`. The agent threads this as an extra leading parameter on every method whose Scala body calls `interner.intern(...)` — i.e. methods that *produce* an interned value. (SPDMX-B adaptation; documented as a `// Rust adaptation (SPDMX-B): ...` comment above the fn.)

### Default collection types
Scala → Rust mapping for collection literals. Typing pass:
- `Vector[X]` → `&'t [X]` (arena slice). `List[X]` → `&'t [X]`. `Array[X]` → `&'t [X]`.
- `Map[K, V]` → `ArenaIndexMap<'t, K, V>` (insertion-ordered, arena-keyed). `mutable.Map` → builder `IndexMap`.
- `Set[X]` → `ArenaIndexSet<'t, X>`.
- Postparsing differs (uses `&'s [X]` and `IndexMap<...>` on the ScoutArena).

If the default is wrong for a specific stub, the agent leaves it and a manual fix happens in body migration — but the default needs to be right *most* of the time.

### String type
Scala `String` → … . Typing pass: `StrI<'s>` (interned). Most occurrences of `String` in a Scala signature are an interned identifier, not a free string.

### Sealed-trait policy
How `sealed trait Foo` translates. Options:
- `enum-with-arena-refs`: `pub enum Foo<'s,'t> { Variant1(&'t Variant1Payload<'s,'t>), … }`. Each variant holds an arena-allocated payload. Used in typing pass.
- `enum-with-box`: `pub enum Foo { Variant1(Box<Variant1Payload>), … }`. For non-arena passes.
- `trait-with-impls`: keep as Rust `trait` with concrete `impl Foo for Variant1` blocks. Rare; only when no closed-set guarantee is needed.

The policy file must also state: *does the sealed trait become Polyvalue?* (i.e. should the enum `#[derive(PartialEq, Eq, Hash, Clone, Copy)]`?) — per @PVECFPZ.

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

### Naming exceptions (SPDMX exception J)
Pre-approved Scala → Rust renames, for cases where the literal translation collides or reads badly. Each entry: `Scala name → Rust name`. slice-rustify applies these instead of the default snake_case conversion.

Typing pass:
- `lookupFunctionByHumanName` → `lookup_function_by_str`
- (extend as the architect approves more)

---

## How the agents consume this file

`slice-rustify` and `slice-placehold` look for a `migration-policy.md` by walking up from the target file's directory until they find one or hit the repo root. If none is found, they fall back to a no-policy mode that prints a warning ("no migration-policy.md found; using generic defaults — output will need manual cleanup"). The orchestrator skill (`docs/skills/slice-pipeline.md`) verifies a policy exists before starting and refuses to run the pipeline without one.

The policy file is read *once per agent invocation* — the agent stuffs the relevant section into its working context and references it in every emit decision. Agents are explicitly told (in their own .md files) not to deviate from the policy without architect approval.

---

## Writing a new policy for a new pass

1. Copy this file to `<new-pass-source-dir>/migration-policy.md`.
2. Fill in every H2 section. Don't leave any blank — "(none)" is the right answer when the section doesn't apply (e.g. a pass with no `case object`s).
3. Get architect sign-off on the policy *before* running the pipeline. The cost of running the pipeline with a wrong policy is high (~32 orphan free fns in `src/typing/` is the typing-pass cost).
4. As the migration proceeds and new patterns surface, edit the policy file rather than special-casing individual files. The policy is canonical.
