# Migration Policy

Single, universal policy file for the slice-pipeline and adjacent migration agents (`slice-rustify`, `slice-placehold`, etc.). One file, one source of truth, applied across **every** pass — `parsing/`, `postparsing/`, `higher_typing/`, `typing/`, `instantiating/`, `simplifying/`. The agent reads this file once per invocation, looks up the values for the pass containing the target file (path-based — `src/typing/...` → typing pass, `src/simplifying/...` → simplifying pass), and emits accordingly.

If a pass isn't listed in the per-pass values table below, the agent stops and reports "no row in migration-policy.md for pass <name>; need an architect-approved row before the pipeline can run."

---

## Per-pass values

| Pass | Path prefix | Suffix | Lifetimes | Where clause | Interner type | Default slice | Default map | String type | Sealed-trait policy | Identity equality | MustIntern seal |
|---|---|---|---|---|---|---|---|---|---|---|---|
| parsing | `src/parsing/` | `P` | `'p` | (none) | `&'ctx ParseArena<'p>` | `&'p [X]` | `IndexMap<K,V>` | `StrI<'p>` | enum-with-arena-refs | `ptr::eq` on interned, derive on Polyvalue | yes |
| postparsing | `src/postparsing/` | `S` | `'s` | (none) | `&'ctx ScoutArena<'s>` | `&'s [X]` | `IndexMap<K,V>` | `StrI<'s>` | enum-with-arena-refs | `ptr::eq` on interned, derive on Polyvalue | yes |
| higher_typing | `src/higher_typing/` | `A` | `'s` | (none) | `&'ctx ScoutArena<'s>` | `&'s [X]` | `IndexMap<K,V>` | `StrI<'s>` | enum-with-arena-refs | `ptr::eq` on interned, derive on Polyvalue | yes |
| typing | `src/typing/` | `T` | `'s, 't` | `where 's: 't` | `&'ctx TypingInterner<'s, 't>` | `&'t [X]` | `ArenaIndexMap<'t, K, V>` | `StrI<'s>` | enum-with-arena-refs | `ptr::eq` on interned, derive on Polyvalue | yes |
| instantiating | `src/instantiating/` | `I` | `'s, 'i` | `where 's: 'i` | `&'ctx InstantiatingInterner<'s, 'i>` | `&'i [X]` | `ArenaIndexMap<'i, K, V>` | `StrI<'s>` | enum-with-arena-refs | `ptr::eq` on interned, derive on Polyvalue | yes |
| simplifying | `src/simplifying/` | `H` | `'s, 'i, 'h` | `where 's: 'i, 'i: 'h` | `&'ctx HammerInterner<'s, 'h>` | `&'h [X]` | `ArenaIndexMap<'h, K, V>` | `StrI<'s>` | enum-with-arena-refs | `ptr::eq` on interned, derive on Polyvalue | yes |

TBD cells are explicit deferrals — the agent must stop and escalate to the architect when it hits a TBD it would need to use. Don't silently substitute typing-pass values.

---

## Schema (what each column means)

### Suffix
Single ASCII letter appended to every Scala type name on translation. If a Scala class name already carries the suffix (e.g. `FunctionT` in the typing pass), it stays unchanged; otherwise the agent appends. Test classes (e.g. `class FooTests extends FunSuite`) do NOT get a suffix.

### Lifetimes + Where clause
Ordered list of lifetime parameters every `struct`/`enum`/`trait`/`impl` carries by default, e.g. `'s, 't`. Plus any default `where`-clause appended to every impl (e.g. `where 's: 't`).

### Interner type
The arena/interner type that body-emitting methods take as a parameter. Threaded as the first parameter on every method whose Scala body intern-allocates (look for `interner.intern(` / `arena.alloc(` / `new FooT(...)` patterns in the body). Annotated with `// Rust adaptation (SPDMX-B): interner threaded explicitly because the Rust pass arena-allocates where Scala used GC.`

### Default slice / map / string types
Scala → Rust mapping for collection literals.
- `Vector[X]` / `List[X]` / `Array[X]` → policy's slice type.
- `Map[K, V]` → policy's map type.
- `mutable.Map[K, V]` → builder map (typically a heap `IndexMap` until `build_in(interner)` finalizes).
- `Set[X]` → analogous set type.
- `String` → policy's string type (almost always an interned `StrI` for the pass).

If the default is wrong for a specific stub, the agent leaves it and a manual fix happens in body migration — but the default needs to be right *most* of the time.

### Sealed-trait policy
How `sealed trait Foo` translates. Options:
- `enum-with-arena-refs` — `pub enum Foo<'s,'t> { Variant1(&'t Variant1Payload<'s,'t>), … }`. Each variant holds an arena-allocated payload. Used in every arena-backed pass (all of them, currently).
- `enum-with-box` — `pub enum Foo { Variant1(Box<Variant1Payload>), … }`. For non-arena passes (not currently used).
- `trait-with-impls` — keep as Rust `trait` with concrete `impl Foo for Variant1` blocks. Rare; only when no closed-set guarantee is needed.

The agent emits two mig markers per sealed trait: `// mig: enum FooX` + `// mig: impl FooX` (where X is the suffix), and a dispatcher fn per abstract `def` on the trait — see [Abstract-def dispatcher](#abstract-def-dispatcher) below.

### Identity equality
Which types use `ptr::eq` identity vs structural derive (per @IEOIBZ + @PVECFPZ):
- All types carrying `MustIntern` (per @SICZ): `impl PartialEq` via `ptr::eq` on the canonical pointer; `impl Hash` likewise.
- Polyvalue wrapper enums (per @PVECFPZ): `#[derive(PartialEq, Eq, Hash, Clone, Copy)]`.
- Value-type / non-interned: `#[derive(PartialEq, Eq, Hash)]` (no Copy unless explicit).

### MustIntern seal
Whether the pass uses the `MustIntern` seal pattern (per @SICZ) for its arena-allocated types. When yes, every struct that becomes a payload of an interned enum variant carries `pub _must_intern: MustIntern` as a private-module-only field. The agent emits the seal on every struct that the **interned-type classifier** flags — see below.

---

## Cross-pass conventions (apply everywhere)

### Abstract-def dispatcher
When a `sealed trait` declares abstract `def`s, the agent emits one dispatcher fn per abstract def on the enum's impl block:

```rust
/* Guardian: disable-all */
pub fn method(&self, /* args */) -> /* return */ {
    match self {
        // Per-variant panic arms; filled in as test paths hit them.
        _ => panic!("Unimplemented: FooX::method dispatch"),
    }
}
```

The `/* Guardian: disable-all */` annotation is mandatory — dispatchers have no 1:1 Scala counterpart (Scala's virtual call is the counterpart, realized differently in Rust). Without it, NNDX fires and TL has to add it manually.

### `equals` / `hashCode` / `unapply` policy
- `override def equals` → realized via `impl PartialEq for FooX`, not a `pub fn eq`. The agent emits a marker stub:
  ```rust
  // mig: fn eq (realized-by-impl PartialEq)
  // (Realized by `impl PartialEq for FooX` below.)
  ```
- `override def hashCode` → realized via `impl Hash for FooX`. Same marker shape.
- `def unapply` → realized via `TryFrom` / pattern-match. Marker: `// mig: fn unapply (realized-by-TryFrom)`.

### `case object` / companion `object`
- `case object Foo` (Scala unit variant of a sealed trait) → unit variant on the Rust enum: `Foo(())`. Not a separate struct.
- `object Foo` (companion object with static defs) → associated `fn`s on the corresponding `impl Foo` (or `impl FooX` after suffix application). Free fns at module scope are wrong.
- `object Foo` (pure namespace, no companion class) → module-level `pub fn`s in a submodule named `foo`. Rare.

### Val/Ref dual-enum pairs (IDEPFL)
Interned-enum families that need a transient `*ValX` companion enum + per-variant `*ValX` payloads. Each pair is: `(canonical enum name, ValX enum name, interner method)`. The agent emits both enums skeleton-wise; bodies stay panic-stubs.

Typing pass examples (architect-blessed):
- `IdT<'s,'t>` / `IdValT<'s,'t,'tmp>` / `typing_interner.intern_id`
- `INameT<'s,'t>` / `INameValT<'s,'t,'tmp>` / `typing_interner.intern_name`
- `ITemplataT<'s,'t>` / `ITemplataValT<'s,'t,'tmp>` / `typing_interner.intern_templata`

For other passes, the equivalent pairs are derived from the Scala `IInterning` hierarchy + the per-pass suffix.

### Arena-classification doc-comment
Every **fully-shaped** type definition gets one of:
- `/// Arena-allocated` — held by `&'t Foo` references; constructed only via interner; identity by `ptr::eq`.
- `/// Temporary state` — held by value or `Box`; constructed freely; identity by structural eq.
- `/// Polyvalue` — closed-set tagged-pointer enum; `#[derive]`s identity.

Bare placeholders (see [Bare-placeholder pattern](#bare-placeholder-pattern) below) deliberately omit the doc-comment — they aren't yet classified.

### Bare-placeholder pattern

Used by **TL/architect only** when scaffolding a type whose upstream module isn't migrated yet (typical: I-side types referenced by H-side simplifying code where `instantiating/ast/<file>` is partially exposed; H-side types referenced by simplifying code where the FinalAST migration hasn't started). Lets a downstream pass make forward progress against named types whose bodies will be populated later.

Shape:
```rust
// mig: case class IdI[+R <: IRegionsModeI, +T <: INameI[R]]
pub struct IdI<'s, 't, R, T>(std::marker::PhantomData<&'s &'t (R, T)>);
// TODO: populate fields when src/instantiating/ast/names.rs is fully migrated.
```

Rules:
- **Canonical Scala name** — no `_full` / `_placeholder` / `_stub` suffix (trips NRDX).
- **Type and lifetime params preserved** from the Scala signature, all absorbed in one `PhantomData` tuple field. SPDMX exception D extended to cover `_marker: PhantomData<…>` fields on otherwise-empty placeholders — these are a documented Rust-mechanism workaround for phantom params, not novel SPDMX data.
- **No arena-classification doc-comment** until fields are populated.
- **No `#[derive]`** until fields are populated.
- **TODO comment** identifying the upstream module that will eventually populate the body.
- **Full Scala shape stays in the adjacent `/* … */` block** per SCPX — reviewers/JR can see the target shape at all times.

Bare placeholders are intentionally an **out-of-band TL move** — the slice-placehold agent emits full-shape structs (with fields guessed from Scala) by default. TL switches to bare-placeholder mode by hand-editing the emitted stub when an upstream dependency forces it.

### NMSFX-bypass workflow exception (SCPX FILE_MAP additions)

Adding entries to `Luz/shields/ScalaCommentParity-SCPX/src/main.rs`'s `FILE_MAP` constant is a documented workflow exception to NMSFX (No Modifications To Shield Files). The FILE_MAP is a registration table, not shield logic — it's how SCPX learns which Rust files have Scala counterparts. Pass-migration work routinely adds entries.

Process:
- JR or TL adds the new `("src/<pass>/<file>.rs", "<Pass>Pass/src/dev/vale/<pass>/<File>.scala"),` line.
- If NMSFX fires (Guardian was running), use `mcp__guardian__guardian_temp_disable` for the specific edit with rationale: "SCPX FILE_MAP registration entry for migrated file; not shield-logic edit."
- No persistent NMSFX exception needed — the temp-disable is the documented path.

### PSMX: no Python scripts that mutate source

Per PSMX (Python Script Mutation), do not use Python (or any inline scripting language) to write/mutate source files. Use the Edit tool. This applies to TL as well as JR — even one-shot bulk-rename or annotation-injection scripts. The temptation grows with the size of the change; resist. If a bulk operation feels needed, decompose it into Edit calls or hand back to the architect for a different approach.

The agent classifies based on whether the Scala type extends `IInterning` (→ Arena-allocated) vs. whether it's a value-typed record (→ Temporary state) vs. a closed-set wrapper enum (→ Polyvalue). When ambiguous: stop and escalate to the architect for that specific type.

### Builder/Frozen pair policy
List of Scala types that split into a Builder + Frozen pair in Rust. The agent emits both with parallel method skeletons; bodies stay panic-stubs and **must be reviewed against the single Scala source side-by-side**.

Typing pass examples:
- `TemplatasStore` → `TemplatasStoreBuilder` + `TemplatasStoreT`
- `NodeEnvironmentBox` → `NodeEnvironmentBuilder` + `NodeEnvironmentBoxT`

For other passes: derive from the Scala `XxxBox` / `XxxBuilder` types where they exist.

### Default fn skeleton
Default emitted body for a freshly-stubbed `fn`:
- `whole-panic` — `{ panic!("Unimplemented: foo"); }`. Default everywhere; safest. Trips SPDMX when refined to skeleton-with-panics later (TL handles the temp-disable per the TL.md "Good Partial Implementing" boilerplate).
- `iteration-skeleton` — only used for fns whose Scala body is a single `.map`/`.foreach`/`groupBy` chain. **TL/architect-only**: requires architect approval per-fn; agent never emits this on its own.

### Naming exceptions (SPDMX exception J)
Pre-approved Scala → Rust renames, for cases where the literal translation collides or reads badly. Architect-blessed only — never invented by the agent.

Current list:
- `lookupFunctionByHumanName` → `lookup_function_by_str`
- (extend as the architect approves more)

---

## How the agents consume this file

`slice-rustify` and `slice-placehold` read `FrontendRust/docs/migration/migration-policy.md` (this file, fixed path) at invocation time. They look up the row for the target file's pass via the **Path prefix** column. They then apply the row's values throughout the emission.

If the agent can't find a row whose path-prefix matches the target file's path, it stops and escalates: "no migration-policy.md row for pass containing <file>; need an architect-approved row before the pipeline can run."

The agent **never** uses values from a different pass's row as a fallback. Typing-pass values are not safe defaults for instantiating or simplifying.

---

## Adding a new pass row

When a new pass starts its first slice-pipeline run, the architect adds a row to the per-pass values table above. The row must specify all columns; `(TBD — defer to architect when first file gets migrated)` is acceptable for any column that doesn't have an obvious value yet, but the agent will halt and re-escalate when it actually needs that column's value.

Don't pre-populate TBD rows with typing-pass values "as a starting point" — that's the bug class this single-file policy is meant to prevent.
