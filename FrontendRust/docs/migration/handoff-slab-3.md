# Handoff: Typing Pass Slab 3 — Kind / Coord / Templata Trio

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0–2 are done:

- **Slab 0** (arena substrate, TypingInterner stub): merged.
- **Slab 1** (leaf types: `OwnershipT`/`MutabilityT`/`VariabilityT`/`LocationT` enums, primitive `KindT` payloads `NeverT`/`VoidT`/`IntT`/`BoolT`/`StrT`/`FloatT`): merged.
- **Slab 2** (name hierarchy: ~60 concrete names, 21 sub-enums, generic `IdT<'s, 't, T: Copy>`, `From`/`TryFrom` bridges, IDEPFL `*ValT` companions): merged. Tagged `slab-2-complete`.

You're doing Slab 3 — fleshing out the remaining types in `src/typing/types/types.rs` and `src/typing/templata/templata.rs`. Compared to Slab 2 this is a smaller slab (~1000 lines of scope vs ~2600), but it leans heavily on Slab 2's name types and touches the IDEPFL interning machinery. Budget: plan for half a workday if you stay focused.

**Read these first in this order**, then come back:

1. `quest.md` — at least §§1.2, 1.4, 1.5, 6.4, 6.5, 6.6, 6.7, 12.1 Slab 3 paragraph. §6.4–6.6 is the design spec for the types you're building; it is the source of truth.
2. `.claude/rules/postparser/IDEPFL-postparser-interning.md` — the dual-enum "value enum for lookup / reference enum for storage" pattern. You'll add `*ValT` companions per IDEPFL, mirroring Slab 2's pattern.
3. `FrontendRust/docs/migration/handoff-slab-2.md` — not strictly required, but it sets up the conventions you'll follow (Scala `/* */` block rules, `#[derive]` rules, `where 's: 't` bounds). Most of the "Rules for each field translation" table there applies verbatim to Slab 3.
4. This doc.

**Important design note up front:** Slab 2 made a significant design refactor that `quest.md` §6.5 doesn't fully reflect yet — name sub-enum families (`IFunctionNameT`, `IStructNameT`, `INameT`, etc.) were moved from arena-interned wrappers to inline-owned 16-byte `Copy` values. When you hit the analogous question for `KindT`'s ICitizenTT/ISubKindTT/ISuperKindTT sub-enums, stop and ask the senior whether to apply the same pattern. See "Gotcha 1" below.

You shouldn't need to read the Scala source externally — every Scala `case class` / sealed trait is already embedded inline in the `/* ... */` blocks in both files. If you can't figure out what a Scala type does from its block, ask; don't guess.

## The big picture: why Slab 3 exists

Three orthogonal type hierarchies live together in this slab:

1. **Kind** (`KindT<'s, 't>`) — Scala's `KindT` sealed trait. This is "what kind of thing is this type?" — a struct, an interface, an int, a placeholder, etc. Already has its ~6 primitive variants (Slab 1 did Never/Void/Int/Bool/Str/Float). Slab 3 adds the ~6 non-primitive variants (Struct/Interface/StaticSizedArray/RuntimeSizedArray/KindPlaceholder/OverloadSet).

2. **Coord** (`CoordT<'s, 't>`) — ownership + region + kind. The type of *a reference to* a thing. `share int`, `own MyStruct`, `borrow &MyClass`. Small Copy struct; Slab 1 already got its three fields. In Slab 3 you mostly leave `CoordT` alone but verify the derives match the new pattern.

3. **Templata** (`ITemplataT<'s, 't>`) — generic-parameter arguments. When you call `foo<int, MyStruct>`, the typing pass resolves `int` and `MyStruct` into `ITemplataT::Kind(...)` and/or `ITemplataT::Coord(...)` values that get stuffed into an `IdT`'s `template_args`. Scala's `ITemplataT[+T <: ITemplataType]` has a phantom outer-type parameter that's **erased** in the Rust port (per §6.6) — the Rust enum is plain `ITemplataT<'s, 't>` with no outer parameter. The enum has 19 variants from small value-kinds (integer, boolean) to heavy ones holding `&'s FunctionA` references.

By the end of Slab 3:

- `cargo check --lib` passes with 0 errors.
- `types/types.rs` has `KindT` with all variants filled, plus `ICitizenTT`/`ISubKindTT`/`ISuperKindTT` sub-enums with real variants (DAG rule applies, but it's a tiny DAG).
- `templata/templata.rs` has `ITemplataT` with all 19 variants and real field-bearing payload structs for each.
- `PrototypeT<'s, 't, T: Copy = ()>` and `SignatureT<'s, 't>` already have fields from Slab 2; verify and leave alone.
- IDEPFL `*ValT` companion types for the newly-interned families: `KindValT`, `ITemplataValT`, `PrototypeValT`, `SignatureValT`. Pattern already in use for names — you're porting the same shape to these families.

As with Slab 2: **body migration of data definitions only** — no method bodies, no solver logic, no compiler logic. Stamp out case classes and enums.

## What's already in place (don't duplicate; don't delete)

Open `src/typing/types/types.rs` (405 lines) and `src/typing/templata/templata.rs` (608 lines). You'll see a mix of fleshed-out definitions (from Slab 1 and from Slab 2's prep work) and `_Phantom` stubs still waiting.

### In `types/types.rs`

**Already done (Slab 1 + Slab 2 prep):**
- `OwnershipT`, `MutabilityT`, `VariabilityT`, `LocationT` enums — real variants.
- `RegionT` — unit struct with derives.
- `CoordT<'s, 't>` — three fields, derives.
- Primitive `KindT` payloads: `NeverT { from_break: bool }`, `VoidT`, `IntT { bits: i32 }`, `BoolT`, `StrT`, `FloatT`.
- `StructTT { id: IdT<'s, 't, IStructNameT<'s, 't>> }`.
- `InterfaceTT { id: IdT<'s, 't, IInterfaceNameT<'s, 't>> }`.
- `StaticSizedArrayTT { name: IdT<'s, 't, StaticSizedArrayNameT<'s, 't>> }`.
- `RuntimeSizedArrayTT { name: IdT<'s, 't, RuntimeSizedArrayNameT<'s, 't>> }`.
- `KindPlaceholderT { id: IdT<'s, 't, KindPlaceholderNameT<'s, 't>> }`.
- `OverloadSetT { env: &'s IInDenizenEnvironmentT<'s, 't>, name: &'s IImpreciseNameS<'s> }`.

**Stubs that need filling:**
- `KindT<'s, 't>` — currently has primitive variants + `_Phantom`. You'll add 6 non-primitive variants and remove `_Phantom`.
- `ICitizenTT<'s, 't>`, `ISubKindTT<'s, 't>`, `ISuperKindTT<'s, 't>` — `_Phantom` enums, need real variants.

### In `templata/templata.rs`

**Already done (Slab 1):**
- `OwnershipTemplataT { ownership: OwnershipT }`.
- `VariabilityTemplataT { variability: VariabilityT }`.
- `MutabilityTemplataT { mutability: MutabilityT }`.
- `LocationTemplataT { location: LocationT }`.
- `BooleanTemplataT { value: bool }`.
- `IntegerTemplataT { value: i64 }`.
- `StringTemplataT<'s> { value: StrI<'s> }`.

**Stubs that need filling** (all `PhantomData`-tuple-struct placeholders):
- `ITemplataT<'s, 't>` enum — needs all 19 variants (the union).
- `CoordTemplataT`, `KindTemplataT`, `PlaceholderTemplataT`, `RuntimeSizedArrayTemplateTemplataT`, `StaticSizedArrayTemplateTemplataT`, `PrototypeTemplataT`, `IsaTemplataT`, `CoordListTemplataT`, `ExternFunctionTemplataT` — mid-weight payload structs.
- `FunctionTemplataT`, `StructDefinitionTemplataT`, `InterfaceDefinitionTemplataT`, `ImplDefinitionTemplataT` — the "heavy" templatas; each holds one or two `&'s` refs into scout-pass output (e.g. `&'s FunctionA<'s>`, `&'s IEnvironmentT<'s, 't>`).
- `IContainer<'s, 't>` enum + `ContainerInterface`/`ContainerStruct`/`ContainerFunction`/`ContainerImpl` — scout-context wrappers. Look at their Scala blocks.
- `CitizenDefinitionTemplataT<'s, 't>` — another narrow enum wrapping the two citizen-definition templatas.

### In `typing_interner.rs`

Current state (after Slab 2 Step 6): four `*ValT` stub structs remain waiting for Slab 3:

```rust
pub struct KindValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct ITemplataValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct PrototypeValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
pub struct SignatureValT<'s, 't>(pub PhantomData<(&'s (), &'t ())>);
```

And four `intern_*` stub methods that take these Vals and return `&'t` refs. You'll leave the method bodies as `panic!()` stubs (Slab 3 only wires up the types), but you'll:
- Replace the four `*ValT` stub definitions with real Val enums/structs (move them into their respective files — `KindValT` alongside `KindT` in `types.rs`, `ITemplataValT` alongside `ITemplataT` in `templata.rs`, and `PrototypeValT`/`SignatureValT` alongside `PrototypeT`/`SignatureT` in `ast/ast.rs`).
- Update `typing_interner.rs` imports to pull the new locations.

This is exactly the same move Slab 2 Step 6 did for `IdValT` / concrete-name Val structs.

## Rules for each field translation

Same rules as Slab 2. Quick reference:

| Scala | Rust |
|---|---|
| `StrI` | `StrI<'s>` |
| `PackageCoordinate` | `&'s PackageCoordinate<'s>` |
| `IImpreciseNameS` | `&'s IImpreciseNameS<'s>` |
| `IRuneS` | `IRuneS<'s>` |
| `RangeS` / `CodeLocationS` | `RangeS<'s>` / `CodeLocationS<'s>` |
| `CoordT` | `CoordT<'s, 't>` (Copy, inline per §6.4) |
| `KindT` | `&'t KindT<'s, 't>` (interned, per §6.5) |
| `ITemplataT[...]` | `ITemplataT<'s, 't>` (inline enum, passed by value; see §6.6 — **erase** the outer `[T]` parameter) |
| `IdT[SomeNameT]` | `IdT<'s, 't, SomeNameT<'s, 't>>` (inline owned sub-enum, from Slab 2) |
| Concrete `StructTT` / `InterfaceTT` / `KindT` variants | inline by value on the variant payload (e.g. `Struct(StructTT<'s, 't>)`) unless quest.md says otherwise |
| `FunctionA` / `StructA` / `InterfaceA` / `ImplA` | `&'s FunctionA<'s>` etc. — scout-lifetime refs into higher_typing output |
| `IEnvironmentT` | `&'s IEnvironmentT<'s, 't>` (scout-allocated from Slab 4 — for now just use the existing trait/enum stub as a type) |
| `Vector[T]` of Copy T | `&'t [T]` (arena slice) |
| `Int` | `i32` |
| `Long` | `i64` |
| `Boolean` | `bool` |

Uniform derive on every concrete payload struct: `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`. Sub-enums in this slab (`KindT`, `ICitizenTT`, `ISubKindTT`, `ISuperKindTT`, `ITemplataT`, `IContainer`, `CitizenDefinitionTemplataT`) get the same derive set.

For fieldless structs (if any), use a named `_phantom` field instead of a tuple struct:
```rust
pub struct EmptyT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
```

## The DAG rule — tiny DAG for KindT sub-enums

`KindT` has three sub-traits analogous to the name hierarchy: `ICitizenTT`, `ISubKindTT`, `ISuperKindTT`. The DAG is much smaller than the name DAG:

```scala
sealed trait ISubKindTT extends KindT       // structs, interfaces, placeholders, arrays
sealed trait ISuperKindTT extends KindT     // interfaces, placeholders (things you can cast *up* to)
sealed trait ICitizenTT extends ISubKindTT with IInterning  // structs and interfaces
```

Concrete Kind variants extend these:
- `StructTT extends ICitizenTT` (so it's also `ISubKindTT`).
- `InterfaceTT extends ICitizenTT with ISuperKindTT`.
- `StaticSizedArrayTT extends ISubKindTT`.
- `RuntimeSizedArrayTT extends ISubKindTT`.
- `KindPlaceholderT extends ISubKindTT with ISuperKindTT`.
- `NeverT`, `VoidT`, `IntT`, `BoolT`, `StrT`, `FloatT`, `OverloadSetT` extend **only** `KindT` (not the sub-traits).

Apply the Slab 2 DAG rule: each concrete type gets a variant in each sub-enum it transitively belongs to. Expect:

```rust
pub enum ICitizenTT<'s, 't> {
    Struct(StructTT<'s, 't>),
    Interface(InterfaceTT<'s, 't>),
}

pub enum ISubKindTT<'s, 't> {
    Struct(StructTT<'s, 't>),
    Interface(InterfaceTT<'s, 't>),
    StaticSizedArray(StaticSizedArrayTT<'s, 't>),
    RuntimeSizedArray(RuntimeSizedArrayTT<'s, 't>),
    KindPlaceholder(KindPlaceholderT<'s, 't>),
}

pub enum ISuperKindTT<'s, 't> {
    Interface(InterfaceTT<'s, 't>),
    KindPlaceholder(KindPlaceholderT<'s, 't>),
}
```

**But see Gotcha 1 below** — the inline-vs-interned question for these sub-enums needs to be settled with the senior before you commit.

## The quest.md §6.5 reference `KindT` shape

Per §6.5:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum KindT<'s, 't> {
    Never(NeverT),
    Void(VoidT),
    Int(IntT),
    Bool(BoolT),
    Str(StrT),
    Float(FloatT),
    Struct(StructTT<'s, 't>),
    Interface(InterfaceTT<'s, 't>),
    StaticSizedArray(StaticSizedArrayTT<'s, 't>),
    RuntimeSizedArray(RuntimeSizedArrayTT<'s, 't>),
    KindPlaceholder(KindPlaceholderT<'s, 't>),
    OverloadSet(&'t OverloadSetT<'s, 't>),
}
```

Note the mix: primitive and struct-like variants are **inline by value**; `OverloadSet` is a `&'t` ref (because `OverloadSetT` is heavier — it holds a whole `&'s IEnvironmentT`). That's the quest.md plan; follow it literally unless Gotcha 1 shifts things.

## The quest.md §6.6 reference `ITemplataT` shape

Per §6.6, `ITemplataT` has 19 variants. The Scala `ITemplataT[+T <: ITemplataType]` outer phantom parameter is **erased** in Rust:

```rust
pub enum ITemplataT<'s, 't> {
    // Leaf-like (all Copy-by-value)
    Coord(&'t CoordTemplataT<'s, 't>),
    Kind(&'t KindTemplataT<'s, 't>),
    Placeholder(&'t PlaceholderTemplataT<'s, 't>),
    Mutability(MutabilityTemplataT),
    Variability(VariabilityTemplataT),
    Ownership(OwnershipTemplataT),
    Integer(i64),
    Boolean(bool),
    String(&'s StrI<'s>),  // scout-lifetimed StrI is fine
    Prototype(&'t PrototypeTemplataT<'s, 't>),
    Isa(&'t IsaTemplataT<'s, 't>),
    CoordList(&'t CoordListTemplataT<'s, 't>),
    RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataT),
    StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataT),

    // Heavy — carry direct &'s refs, like Scala (not interned themselves)
    Function(&'t FunctionTemplataT<'s, 't>),
    StructDefinition(&'t StructDefinitionTemplataT<'s, 't>),
    InterfaceDefinition(&'t InterfaceDefinitionTemplataT<'s, 't>),
    ImplDefinition(&'t ImplDefinitionTemplataT<'s, 't>),
    ExternFunction(&'t ExternFunctionTemplataT<'s, 't>),
}
```

Note the **mixed-mode equality** per §6.6: Copy-value variants compare by value; `&'t`-ref variants compare by pointer identity (relying on interner uniqueness). `ITemplataValT` mirrors the split. This matches the pattern you see in the names work (inline values vs interned refs), so it should feel familiar.

Heavy templata payloads (`FunctionTemplataT` etc.) hold scout-lifetime refs directly. These are **not** interned (they're `allocated but not interned` per §1.5). For example:

```rust
pub struct FunctionTemplataT<'s, 't> {
    pub outer_env: &'s IEnvironmentT<'s, 't>,
    pub function: &'s FunctionA<'s>,
}
```

`IEnvironmentT` is a Slab 4 type — there's a placeholder enum/trait in `env/environment.rs`. Use it as-is; don't redefine.

## IDEPFL `*ValT` companions

Same pattern as Slab 2 Step 6. Four families get Val companions this slab:

1. **`KindValT<'s, 't>`** — moves out of `typing_interner.rs` into `types/types.rs`. Mirrors `KindT`: one variant per Kind case. Most variants just wrap the same payload by value (since `StructTT`, `InterfaceTT`, etc. are Copy). The `OverloadSet` variant is interesting — since `KindT::OverloadSet(&'t OverloadSetT)` uses a ref, `KindValT::OverloadSet` can either take the Val struct by value or take a ref-to-just-interned `&'t OverloadSetT`. Use `&'t OverloadSetT` for consistency with the canonical ref form.

2. **`ITemplataValT<'s, 't>`** — moves into `templata/templata.rs`. Per §6.6's mixed-mode note, Val variants for the Copy-value templatas hold them by value; Val variants for the `&'t`-ref templatas hold `&'t` too.

3. **`PrototypeValT<'s, 't, T: Copy>`** — moves into `ast/ast.rs` next to `PrototypeT`. Since `PrototypeT<'s, 't, T: Copy>` has `id: IdT<'s, 't, T>` and `return_type: CoordT<'s, 't>`, the Val has the same shape — but note `IdT<'s, 't, T>` contains an arena slice (`init_steps`), so `PrototypeValT` needs a `'tmp` lifetime and an `IdValT<'s, 't, 'tmp, T>` for its id field. Mirrors Slab 2's `IdValT` work.

4. **`SignatureValT<'s, 't>`** — moves into `ast/ast.rs` next to `SignatureT`. Since `SignatureT { id: IdT<'s, 't, &'t IFunctionNameT<'s, 't>> }` — wait, check: after the Slab 2 refactor this is actually `IdT<'s, 't, IFunctionNameT<'s, 't>>` (inline sub-enum). So `SignatureValT<'s, 't, 'tmp>` holds `id: IdValT<'s, 't, 'tmp, IFunctionNameT<'s, 't>>`. Same pattern.

For each `*ValT`, add `#[derive(Copy, Clone, Debug)]` plus custom `PartialEq`/`Eq`/`Hash` *if* the struct has slices or fields where structural hashing isn't enough. If the Val is pure Copy + structural-eq'able (no slices, no `&'s` refs needing pointer-eq), just derive all six traits.

Finally, update `typing_interner.rs` to import the relocated Vals and leave the four `intern_*` method bodies as `panic!()` stubs. No body implementation this slab.

## Step-by-step plan

### Step 1: Confirm your starting point

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-3.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-3.txt
```

Must print `0`. If it doesn't, stop and ask the senior. The project sets `#![allow(unused_variables, unused_imports)]`, so don't rely on warning counts.

### Step 2 — Design-question pause

Before writing any code: **read Gotcha 1 below and get an answer from the senior.** The question is whether `ICitizenTT`/`ISubKindTT`/`ISuperKindTT` should follow the Slab 2 inline-sub-enum pattern (not interned, 16-40 bytes inline) or the quest.md §6.5 pattern (interned). Your implementation changes depending on the answer. Don't guess.

### Step 3 — Fill `KindT` non-primitive variants + sub-enums (types/types.rs)

1. Delete the `_Phantom` variant on `KindT` and add the 6 non-primitive variants per quest.md §6.5.
2. Delete the `_Phantom` variants on `ICitizenTT`/`ISubKindTT`/`ISuperKindTT` and add the DAG-per-§6.2 variants (see "The DAG rule" section above).
3. Add `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` to all four enums if not already present.
4. Leave `/* scala */` blocks untouched.
5. Remove any `// TODO: placeholder PhantomData` comments immediately above each filled definition.
6. Run `cargo check --lib` per the session-file convention.

Expected fallout: dozens of downstream uses of `KindT<'s, 't>` pattern-match on variants; they may now have new variants unhandled. Since you're only defining types, match arms in callers are still `panic!()`-stubbed — nothing structural breaks.

### Step 4 — Fill `ITemplataT` enum + payload structs (templata/templata.rs)

1. Fill each of the ~16 `PhantomData`-tuple-struct payloads (`CoordTemplataT`, `KindTemplataT`, `PlaceholderTemplataT`, `FunctionTemplataT`, `StructDefinitionTemplataT`, `InterfaceDefinitionTemplataT`, `ImplDefinitionTemplataT`, etc.) with their real Scala-parity fields per the `/* */` block.
2. Replace `ITemplataT`'s `_Phantom` with the 19 variants per quest.md §6.6 / the shape above.
3. Fill `IContainer<'s, 't>` + its 4 `Container*` payload stubs with their Scala-parity fields.
4. Fill `CitizenDefinitionTemplataT<'s, 't>` enum (narrow union of struct-def and interface-def templatas).
5. Derive `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` on everything.
6. Cargo check, verify 0 errors.

### Step 5 — Update `PrototypeT` / `SignatureT` derives + verify

Currently `PrototypeT<'s, 't, T: Copy = ()>` and `SignatureT<'s, 't>` compile fine with their fields from Slab 2. Verify:
- `PrototypeT` has custom Hash/Eq if needed (it should work with derive since `id: IdT<'s, 't, T>` already has custom Hash/Eq from Slab 2, and `return_type: CoordT<'s, 't>` is plain Copy + derive).

Actually — check whether `#[derive(Hash)]` on `PrototypeT` works now that `IdT`'s Hash is custom. Rust's `derive` delegates to the field's `Hash` impl, so this should Just Work. If it doesn't, add custom Hash/Eq mirroring the IdT pattern.

### Step 6 — IDEPFL `*ValT` companions

Per "IDEPFL `*ValT` companions" section above. Move each `*ValT` stub out of `typing_interner.rs` into its home file, expand it to a real Val type with real variants / fields:

1. **`KindValT`** — into `types/types.rs` right after `KindT`.
2. **`ITemplataValT`** — into `templata/templata.rs` right after `ITemplataT`.
3. **`PrototypeValT`** — into `ast/ast.rs` right after `PrototypeT`. Needs `'tmp` lifetime and generic `T: Copy`.
4. **`SignatureValT`** — into `ast/ast.rs` right after `SignatureT`. Needs `'tmp`.

Update `typing_interner.rs` imports to pull from the new locations. Leave the 4 `intern_*` method bodies as `panic!()` stubs with matching signatures.

Remove the stub `*ValT` struct definitions in `typing_interner.rs` (they'll only exist in their new homes).

Run `cargo check --lib`.

### Step 7 — Verify and commit

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-3.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-3.txt   # must be 0
grep -rn "_Phantom" FrontendRust/src/typing/types/types.rs FrontendRust/src/typing/templata/templata.rs
#   ^ should show few or zero hits — only the handful of genuinely
#   lifetime-anchoring PhantomData fields, not placeholder stubs.
```

Recommended commit cadence:
- Prep derive additions (if any) as one commit.
- `KindT` + sub-enums as one commit.
- `ITemplataT` + payloads as one commit.
- `*ValT` companion migration as one commit.

Or collapse into a single "Slab 3" commit if the diff stays readable (~500–800 lines).

Finally, tag the result:

```bash
git tag -a slab-3-complete -m "Typing Slab 3 complete: KindT + ITemplataT + ValT companions."
```

## Gotchas

### Gotcha 1 (READ BEFORE CODING): Inline vs interned for KindT sub-enums

Slab 2 refactored name sub-enum families (`IFunctionNameT`, `IStructNameT`, `INameT`, etc.) from arena-interned wrappers to inline-owned 16-byte `Copy` values. quest.md §§1.5/6.1/6.2/6.3 were updated. But **§6.5 KindT was not updated** because it says "All variants interned" and treats `KindT`-family types analogously to the *old* names design.

You have two reasonable paths and I want you to ask the senior before committing to either:

**Path A (literal §6.5):** `KindT` itself stays interned (`&'t KindT` everywhere it's used). `ICitizenTT`/`ISubKindTT`/`ISuperKindTT` also stay interned (arena-allocated wrappers, pointer-eq identity). This matches §6.5 verbatim. `KindValT` has variants per KindT variant, mirroring what Slab 2 did for name concretes.

**Path B (apply Slab 2 refactor to kind sub-enums):** `ICitizenTT`/`ISubKindTT`/`ISuperKindTT` become inline-owned 16-40 byte Copy values. `KindT` might stay interned because it's 40+ bytes and the existing `CoordT.kind: &'t KindT<'s, 't>` field is pervasive (`&'t` avoids growing Coord). This matches the names-refactor philosophy: cast-between-family-enums is a stack rewrap.

The trade-off is exactly like Slab 2's:
- Path A: pointer-eq identity on `&'t ICitizenTT` for free; but every cast (concrete → ICitizenTT → ISubKindTT → KindT) costs an interner allocation.
- Path B: casts are stack-only; but structural compare on 40-byte enum values.

Given the sizes involved here (ICitizenTT variants hold inline StructTT/InterfaceTT which are 40 bytes each), inline-owned is **bigger** (~48 bytes) than for names (16 bytes). That changes the calculus.

**Ask the senior.** They'll have context on whether the names-refactor philosophy should be uniformly applied. Once you have the answer, write the corresponding Val types to match.

### Gotcha 2: `PrototypeT` derive(Hash) after Slab 2

Slab 2 made `IdT<'s, 't, T: Copy>` use custom `Hash`/`PartialEq`/`Eq` instead of derive. `PrototypeT<'s, 't, T: Copy>` has `id: IdT<'s, 't, T>` and `return_type: CoordT<'s, 't>`. Its `#[derive(Hash, PartialEq, Eq)]` should still work because Rust's `derive` generates an impl that calls the field's `Hash` — which is the custom `IdT` impl. But if it somehow fails to compile, add a manual Hash/Eq to `PrototypeT` that delegates to `self.id.hash(state); self.return_type.hash(state);`.

### Gotcha 3: `IContainer` / `CitizenDefinitionTemplataT` — narrow enums, small DAGs

These two enums are mini-unions that aren't used everywhere — they appear in specific spots like `CitizenDefinitionTemplataT` wraps `StructDefinitionTemplataT` or `InterfaceDefinitionTemplataT`. Read the Scala blocks carefully; the variant set is small (2-4 each). These are also inline-owned (not interned).

### Gotcha 4: Heavy templatas hold `&'s FunctionA` / `&'s StructA` / etc.

These reference higher-typing (scout-pass) output directly. `FunctionA`, `StructA`, `InterfaceA`, `ImplA` live in `src/higher_typing/ast.rs`. Use `&'s` — the same scout-arena lifetime that interned strings and runes use. Don't re-intern.

Also, heavy templatas themselves are **allocated but not interned** per §1.5 — they live in the typing arena but the interner doesn't dedup them. In practice, the `ITemplataT` enum holds `&'t FunctionTemplataT` (arena ref), but that arena ref came from a simple `arena.alloc(FunctionTemplataT { ... })` call, not from an `intern()` call.

You don't need to do anything special about this during Slab 3 — just define the types correctly and trust the interner to be a no-op for these families. The distinction matters in Slab 3+ when someone actually writes the arena-alloc calls.

### Gotcha 5: Scala `ITemplataT[+T <: ITemplataType]` outer parameter is erased

Scala's `ITemplataT` has an outer phantom type parameter tracking what kind of templata it is (`CoordTemplataType`, `KindTemplataType`, `MutabilityTemplataType`, etc.). In Rust this is **erased** — the variant tag + the `tyype` field on `PlaceholderTemplataT` carry the same information at runtime. Don't try to preserve the phantom parameter; the Rust enum is just `ITemplataT<'s, 't>`.

A **separate** inner parameter — `PrototypeTemplataT[T <: IFunctionNameT]` — is NOT erased (see §6.6). That's the `T` in `PrototypeT<'s, 't, T: Copy>`, threaded through to the `id: IdT<'s, 't, T>` field. Keep that T.

### Gotcha 6: Don't write `impl XxxT { fn ... }` method bodies

Like Slab 2, this is data-definition work only. Scala `case class` bodies with `vpass()`, `override def equals`, `def toString`, etc. are Slab 8+ work. If a Scala block has methods, leave them in the `/* */` and don't port them in Slab 3. The stubs that already exist (`impl<'s, 't> KindT<'s, 't> { fn expect_struct(&self) -> StructTT<'s, 't> { panic!(...) } }`) stay `panic!()` until later slabs.

### Gotcha 7: Pre-commit hook on `/* */` blocks

`.claude/hooks/check-scala-comments` does exact-match comparison between every `/* */` block and the Scala source. If you reformat or edit text inside a `/* */`, the commit is rejected. Same rule as Slab 2.

## What "done" looks like

- `cargo check --lib` passes with 0 errors.
- `KindT<'s, 't>` has 12 variants (6 primitives from Slab 1 + 6 non-primitives from Slab 3). No `_Phantom`.
- `ICitizenTT`, `ISubKindTT`, `ISuperKindTT` have real variants per the DAG and Gotcha 1 resolution.
- `ITemplataT<'s, 't>` has 19 variants. All payload structs (`CoordTemplataT`, `FunctionTemplataT`, etc. — ~16 of them) have Scala-parity fields.
- `IContainer<'s, 't>` and `CitizenDefinitionTemplataT<'s, 't>` have real variants.
- `KindValT`, `ITemplataValT`, `PrototypeValT`, `SignatureValT` exist in their home files with real shapes, not `PhantomData` stubs. `typing_interner.rs` imports them from their new locations.
- The four `intern_*` method bodies in `typing_interner.rs` stay `panic!()` — signatures updated to use the relocated Val types.
- Scala `/* */` blocks unchanged.
- Tag `slab-3-complete`.

## When you're stuck

- **"Lifetime error on `'t` unused"**: some stub structs need `PhantomData<&'t ()>` to anchor the lifetime. See Slab 2 Tier A/B for examples.
- **"cannot derive Hash because field X doesn't implement Hash"**: the field type's Hash derive is missing. Check the type; if it's a name or kind you own, add the derive; if it's something like `&'t FunctionA<'s>` where `FunctionA` doesn't have Hash, custom-impl Hash by doing pointer-hash (`std::ptr::hash(ptr, state)`) since `FunctionA` refs are scout-arena-interned and thus have pointer identity.
- **"Scala extends `with IInterning` or `with Equatable` or `with IInstantiationBoundEquivalent`"**: these are marker traits in Scala, not real sub-traits. They don't map to any Rust enum variant. Ignore them.
- **"I don't know whether `KindValT::Struct` should hold `StructTT` by value or `&'t StructTT`"**: quest.md §6.5's Rust block answers this — the canonical enum holds it by value; the Val mirrors. But see Gotcha 1 first.
- **"Something in env/*.rs is missing and I need it"**: Slab 4 hasn't happened yet. Use whatever existing placeholder (`IEnvironmentT` stub, `IInDenizenEnvironmentT` stub) is already there. Don't try to build Slab 4 types in Slab 3.
- **Anywhere you're tempted to modify `names/names.rs`**: don't. Slab 2 is frozen. If you think a name type needs changing, the design shifted and you should ask the senior.

## Where to file questions

- **Design** (Gotcha 1 especially): ask the senior first. The inline-vs-interned question for KindT sub-enums has real downstream implications.
- **Scala semantics**: the `/* */` block is the spec. If that's ambiguous, the senior has access to the external Scala repo.
- **Hook rejections**: read the diff the hook prints. Usually it's you accidentally editing inside a `/* */` block.

## Final advice

Like Slab 2, this is data-definition grunt work. It's smaller (~1000 lines scope vs 2600) and simpler (tiny DAG for kind sub-enums), but the IDEPFL Val work is fiddly — especially `PrototypeValT` and `SignatureValT` which need `'tmp` lifetimes because they transitively contain IdT's init_steps slice. Follow the Slab 2 Step 6 template: `IdValT<'s, 't, 'tmp, T: Copy>` is your reference.

Get the Gotcha 1 answer before writing code. Everything else is mechanical.

Good luck.
