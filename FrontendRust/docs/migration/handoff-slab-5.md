# Handoff: Typing Pass Slab 5 — Expression AST

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0–4 are done:

- **Slab 0** (arena substrate, `TypingInterner<'s, 't>` skeleton): merged.
- **Slab 1** (leaf types: `OwnershipT`/`MutabilityT`/`VariabilityT`/`LocationT` enums, primitive `KindT` payloads): merged.
- **Slab 2** (name hierarchy: ~60 concrete names, 22 sub-enums, monomorphic `IdT<'s, 't>`): tagged `slab-2-complete`.
- **Slab 3** (Kind/Coord/Templata trio: `KindT`/`ITemplataT` inline wrappers, `PrototypeT`/`SignatureT`): tagged `slab-3-complete`.
- **Slab 4** (environments + real interner bodies, 9 env types + `TemplatasStoreT` + `GlobalEnvironmentT`, ~84 per-concrete intern wrappers, `NodeEnvironmentBox` family deleted in favor of the builder-freeze pattern): tagged `slab-4-complete`.

You're doing **Slab 5** — the **expression AST** in `src/typing/ast/expressions.rs`. This is a ~2100-line file that already has every Scala `case class` / sealed trait embedded as a `/* */` block with a Rust stub struct/enum above it. Your job is to turn those stubs into real, Scala-parity data definitions — nothing more. No method bodies, no interner wiring (expressions are **not** interned), no visitor, no compiler logic.

Compared to Slab 4, this slab is smaller and more mechanical — **no interner work, no builders, no From/TryFrom bridges across sibling enums, no heavy back-references to envs**. The tricky parts are all upfront: (a) the existing stubs are structurally unsound (see "Up-front: the stubs are infinitely-sized"), (b) three enums and ~44 payload structs have to be filled consistently, (c) AASSNCMCX applies — every `Vec<T>` in a stub must become `&'t [T]`, and (d) every sub-expression field holds `&'t ReferenceExpressionTE<'s, 't>` / `&'t AddressExpressionTE<'s, 't>`, not by-value. Budget: plan for 0.5–1 workday if you stay focused.

**Read these first in this order**, then come back:

1. `quest.md` — at least **§§1.5, 6.4, 7 (all of Part 7), 12.1-Slab-5 paragraph**. Part 7 is the design spec; accurate, no known overrides.
2. `TL-HANDOFF.md` at repo root — the current top-level handoff. The "File layout + style standards for typing/" section is load-bearing for this slab; every new `impl` block in `expressions.rs` must follow it.
3. `.claude/rules/postparser/IDEPFL-postparser-interning.md` — the IDEPFL interning pattern. Skim only. **Expression AST is NOT interned** (§7.2), so you won't be adding `*ValT` companions or intern methods, but you should know what IDEPFL is so you understand why it doesn't apply here.
4. `FrontendRust/docs/migration/handoff-slab-4.md` — just §§"What 'done' looks like" and the first four Gotchas. Slab 4 settled the conventions around `#[derive]` on arena-allocated structs (ATDCX), `where 's: 't`, and the "wrapper enum is inline-Copy, payloads are `&'t`-refs" philosophy. Slab 5 reuses them verbatim — you're not re-deciding anything.
5. `FrontendRust/docs/shields/ArenaTypesDontClone-ATDCX.md` — the rule that expression AST payload structs don't derive `Clone`/`Copy`. One-page read.
6. `Luz/shields/ArenaAllocatedStructsShouldNotContainMallocdCollections-AASSNCMCX.md` (shield AASSNCMCX) — the rule that `Vec<T>` / `HashMap` fields don't belong inside arena-allocated structs. The existing stubs all violate this; you'll fix them.
7. This doc.

You shouldn't need to read the Scala source externally — every Scala `case class` / sealed trait is already embedded inline in the `/* ... */` blocks in `expressions.rs`. If a block is ambiguous, ask; don't guess.

---

## The big picture: why Slab 5 exists

After the typing pass resolves names, generic parameters, environments, and kinds, it produces a typed expression tree for each function body. That tree is what Slab 5 defines the shape of. In Scala:

- `ExpressionT` is a `sealed trait` — the root of the tree.
- `ReferenceExpressionTE extends ExpressionT` — expressions whose result is a value/reference. ~38 concrete subclasses (`FunctionCallTE`, `IfTE`, `WhileTE`, `LetNormalTE`, etc.).
- `AddressExpressionTE extends ExpressionT` — expressions whose result is an address (an l-value). 5 concrete subclasses (`LocalLookupTE`, `ReferenceMemberLookupTE`, etc.).
- `IExpressionResultT` is a sealed trait for the *computed type* of an expression — `ReferenceResultT(coord)` or `AddressResultT(coord)`. Returned from `def result: IExpressionResultT` on every node. Not a tree node itself — just a 2-variant discriminated union over `CoordT`.

In Rust, per `quest.md` Part 7:

- **Three enums** (§7.1): `ReferenceExpressionTE<'s, 't>` (~38 variants), `AddressExpressionTE<'s, 't>` (5 variants), and a narrow wrapper `ExpressionTE<'s, 't>` (2 variants: `Reference(&'t ReferenceExpressionTE)` / `Address(&'t AddressExpressionTE)`). The first two are **inline-Copy wrappers** whose variants hold the payload struct inline by value; the third is a 2-variant `&'t`-ref wrapper used only in places where a slot can hold either.
- **~44 payload structs** — one per Scala `case class`. `#[derive(PartialEq, Eq, Hash, Debug)]`, **not** `Copy`/`Clone` (ATDCX — they're arena-allocated conceptually: the whole `ReferenceExpressionTE` enum lives in `'t`, so transitively the payload data lives there too).
- **Arena-allocated, not interned** (§7.2). Expression trees are unique per function body; no dedup.
- **Sub-expressions are `&'t` refs; collections are arena slices** (§7.2). A parent node's single child-expression field is `&'t ReferenceExpressionTE<'s, 't>` (or `&'t AddressExpressionTE<'s, 't>`); a parent's list-of-children field is `&'t [ReferenceExpressionTE<'s, 't>]` — a dense arena slice of inline-enum values, same pattern as `ConsecutorSE.exprs` in postparsing.
- **Can hold scout data** (§7.3). `RangeS<'s>`, `StrI<'s>`, etc. are fine as-is.
- **`IExpressionResultT` / `ReferenceResultT` / `AddressResultT`** — small inline `Copy` enum + two tiny wrapper structs. `ReferenceResultT { coord: CoordT }`, `AddressResultT { coord: CoordT }`, `IExpressionResultT::Reference(ReferenceResultT) | Address(AddressResultT)`. Not arena-allocated — too small (24 bytes), passed by value like `CoordT`.

By the end of Slab 5:

- `cargo check --lib` passes with 0 errors.
- `expressions.rs`: three real enums with all their variants, ~44 real payload structs with Scala-parity fields, `IExpressionResultT`/`ReferenceResultT`/`AddressResultT` are real 2-variant inline enum + 2 structs.
- Every `Vec<T>` field in a payload struct has been flipped to `&'t [T]` per AASSNCMCX.
- Every by-value sub-expression field (`expr: ReferenceExpressionTE<'s, 't>`, `inner_expr: ReferenceExpressionTE<'s, 't>`, etc.) has been flipped to `&'t ReferenceExpressionTE<'s, 't>` / `&'t AddressExpressionTE<'s, 't>` per §7.2. **This is the most important structural change in this slab — see Gotcha 1.**
- All `fn equals` / `fn hash_code` / `fn result` / `fn new` stubs keep `panic!()` bodies. **Do not port the `vassert`/`vcurious`/`vfail` logic inside the Scala `/* */` blocks** — that's Slab 8+ body work.
- Scala `/* */` blocks unchanged byte-for-byte (the pre-commit hook enforces this).
- The file-layout conventions from TL-HANDOFF are preserved: `use` imports at top, class-then-impl ordering, one fn per impl block, multi-line impl bodies, `fn new(...)` panic-stub for any case class whose Scala body has `vassert`.

**What Slab 5 is NOT:**

- No interner wiring. Expression nodes aren't interned.
- No `*ValT` companions. Expression nodes aren't interned.
- No builders. Expression trees are constructed bottom-up by the expression compiler (Slab 8+), not via builder-freeze.
- No `From`/`TryFrom` bridges from payload to enum. Patterns are `ReferenceExpressionTE::FunctionCall(FunctionCallTE { … })` construction and `match` destructuring — no ceremony. (Contrast with Slab 2's names: there we wrote `From<&'t FunctionNameT> for INameT` because `INameT` wrapped `&'t FunctionNameT`. Here the wrapper enum holds the payload **by value**, so there's no indirection to bridge across — variants are constructed and matched directly.)
- No `NodeRefT` visitor, no `visit_*` functions, no `collect_*` macros. Quest.md §7.4 names them as "same as parsing/postparsing" (the existing examples live in `src/parsing/tests/traverse.rs` and `src/postparsing/test/traverse.rs`, ~1000+ lines each), but those were written in support of test suites. **The typing pass has no test suite yet** — Slab 9+ is test-driven and will write the visitor then. Slab 5 skips it entirely. (If you find yourself reaching for the visitor to make something compile, you've gone off-spec; ask the senior.)
- No method body migration. `def result`, `def equals`, `def lastReferenceExpr`, the extractor objects at the bottom of the file (`referenceExprResultStructName`, `referenceExprResultKind`), etc. all stay `panic!()`.

---

## Up-front: the stubs are infinitely-sized

**Read this section twice before writing any code.** It's the single biggest mental adjustment this slab requires.

Open `src/typing/ast/expressions.rs`. You'll see ~44 payload struct stubs that look like this:

```rust
pub struct LetAndLendTE<'s, 't> {
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: ReferenceExpressionTE<'s, 't>,         // ← by value
    pub target_ownership: OwnershipT,
}

pub struct DeferTE<'s, 't> {
    pub inner_expr: ReferenceExpressionTE<'s, 't>,   // ← by value
    pub deferred_expr: ReferenceExpressionTE<'s, 't>, // ← by value
}

pub struct ConsecutorTE<'s, 't> {
    pub exprs: Vec<ReferenceExpressionTE<'s, 't>>,   // ← Vec + by value
}
```

These compile **today** only because the enum itself is `_Phantom`:

```rust
pub enum ReferenceExpressionTE<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
```

The enum has size 0 right now, so an inline `ReferenceExpressionTE` field is zero-cost. **The moment you fill the enum with its ~38 real variants — each carrying a payload struct that transitively may contain `ReferenceExpressionTE` again — the compiler rejects it with "recursive type has infinite size"**. The stubs were authored before the enum was filled, as placeholder shapes; they're wrong, and you're fixing them.

**Two rules for the flip:**

1. **Single sub-expression fields** (`expr`, `inner_expr`, `condition`, `array_expr`, `generator`, `source_expr`, etc.) → flip to `&'t ReferenceExpressionTE<'s, 't>` (or `&'t AddressExpressionTE<'s, 't>` depending on which wrapper Scala uses).
2. **`Vec<ReferenceExpressionTE>` fields** (e.g. `ConsecutorTE.exprs`, `TupleTE.elements`, `FunctionCallTE.args`) → flip to `&'t [ReferenceExpressionTE<'s, 't>]`. This is a dense arena slice of inline-enum values; same pattern as `ConsecutorSE.exprs: &'s [IExpressionSE<'s>]` in postparsing. The elements stay inline (16 bytes each) — you're not also flipping them to `&'t ReferenceExpressionTE`. Rationale: postparsing already settled this; collections of small-inline-wrapper values are kept dense-packed, single-child slots are kept indirect.

Same story for `AddressExpressionTE` sub-expressions (only `MutateTE.destination_expr` has one — it's `AddressExpressionTE`), and same story for the `Vec<ReferenceLocalVariableT>` fields (those become `&'t [ReferenceLocalVariableT<'s, 't>]` too — no `Vec` in arena-pinned structs per AASSNCMCX).

After the flip, `LetAndLendTE` etc. become finite-sized:

```rust
pub struct LetAndLendTE<'s, 't> {
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: &'t ReferenceExpressionTE<'s, 't>,     // 8 bytes
    pub target_ownership: OwnershipT,                // 1 byte
}

pub struct DeferTE<'s, 't> {
    pub inner_expr: &'t ReferenceExpressionTE<'s, 't>,     // 8 bytes
    pub deferred_expr: &'t ReferenceExpressionTE<'s, 't>,  // 8 bytes
}

pub struct ConsecutorTE<'s, 't> {
    pub exprs: &'t [ReferenceExpressionTE<'s, 't>],   // 16 bytes (ptr + len)
}
```

---

## Rules for each field translation

Same rules as Slabs 2/3/4. Quick reference:

| Scala | Rust |
|---|---|
| `StrI` | `StrI<'s>` |
| `RangeS` / `CodeLocationS` | `RangeS<'s>` / `CodeLocationS<'s>` |
| `CoordT` | `CoordT<'s, 't>` (Copy, inline) |
| `KindT` / `ITemplataT` | `KindT<'s, 't>` / `ITemplataT<'s, 't>` (inline Copy wrappers from Slab 3) |
| `StructTT` / `InterfaceTT` / `StaticSizedArrayTT` / `RuntimeSizedArrayTT` | `&'t StructTT<'s, 't>` etc. — interned Slab 3 Kind payloads, accessed by `&'t` ref |
| `ISuperKindTT` | `ISuperKindTT<'s, 't>` — inline Copy wrapper (Slab 3) |
| `IdT[SomeNameT]` | `IdT<'s, 't>` — monomorphic, no generic T. Narrow via pattern-match at use sites. |
| `PrototypeT[IFunctionNameT]` | `PrototypeT<'s, 't>` — monomorphic |
| `IVarNameT` | `IVarNameT<'s, 't>` — inline Slab 2 wrapper |
| `ILocalVariableT` | `ILocalVariableT<'s, 't>` — Slab 4 inline 2-variant enum |
| `ReferenceLocalVariableT` | `ReferenceLocalVariableT<'s, 't>` — Slab 4 inline struct (not `&'t`; already 40 bytes, stays inline) |
| `IExpressionSE` | `&'s IExpressionSE<'s>` — scout-lifetime postparser expression (used by `NodeEnvironmentT`, not by Slab 5) |
| **Sub-expression of wrapper type `ReferenceExpressionTE`** | **`&'t ReferenceExpressionTE<'s, 't>`** (see the up-front section above) |
| **Sub-expression of wrapper type `AddressExpressionTE`** | **`&'t AddressExpressionTE<'s, 't>`** |
| `Vector[ReferenceExpressionTE]` | `&'t [ReferenceExpressionTE<'s, 't>]` — dense arena slice |
| `Vector[ExpressionT]` | `&'t [ExpressionTE<'s, 't>]` — see Gotcha 3 for the rename |
| `Vector[ReferenceLocalVariableT]` | `&'t [ReferenceLocalVariableT<'s, 't>]` — AASSNCMCX, no `Vec` in arena-pinned data |
| `String` (source-literal) | `StrI<'s>` — re-use the scout interner. See Gotcha 5 |
| `Double` | `f64` |
| `Int` | `i32` |
| `Long` | `i64` |
| `Boolean` | `bool` |
| `RegionT` | `RegionT` (Copy, no lifetime) |

### Derives

**Payload structs** (`LetAndLendTE`, `IfTE`, `FunctionCallTE`, etc.) — arena-allocated per §1.5 ("allocated but NOT interned"). Apply ATDCX:

```rust
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct FooTE<'s, 't> where 's: 't { /* … */ }
```

**NOT `Copy`/`Clone`.** These are arena-allocated; callers pass them via `&'t FooTE`, never by value.

**Wrapper enums** — `ReferenceExpressionTE<'s, 't>`, `AddressExpressionTE<'s, 't>`, `ExpressionTE<'s, 't>`, `IExpressionResultT<'s, 't>`. Inline Copy wrappers, same philosophy as `KindT` / `ITemplataT` / `IEnvironmentT`:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ReferenceExpressionTE<'s, 't> { /* 38 variants holding payload structs by value */ }
```

Wait — hold on. Payload structs aren't `Copy`, so `ReferenceExpressionTE::FunctionCall(FunctionCallTE)` with inline payload can't derive `Copy` either. **This is correct and intentional.** See Gotcha 2 for the resolution.

**`ReferenceResultT` / `AddressResultT`** — tiny 24-byte structs, one `CoordT<'s, 't>` field each. `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`. Not arena-allocated; passed by value.

**`IExpressionResultT`** — 2-variant enum over the two tiny result structs, same pattern as above. `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`.

### `where 's: 't` bound

Every payload struct with a `&'t` field should carry `where 's: 't` — same as Slab 4's convention (Gotcha 14 in handoff-slab-4.md). Example:

```rust
pub struct LetAndLendTE<'s, 't>
where 's: 't,
{
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: &'t ReferenceExpressionTE<'s, 't>,
    pub target_ownership: OwnershipT,
}
```

The three wrapper enums already compile without the bound (Rust infers it from the `&'t`-ref variants), but adding `where 's: 't` to them too keeps the file uniform. Your call.

---

## The three enums

### `ReferenceExpressionTE<'s, 't>` — ~38 variants

One variant per Scala `case class … extends ReferenceExpressionTE`. The variant name is the struct name minus the trailing `TE`; the payload is the struct **by value** (not `&'t`). Skeleton:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]  // see Gotcha 2 for Copy resolution
pub enum ReferenceExpressionTE<'s, 't> {
    LetAndLend(LetAndLendTE<'s, 't>),
    LockWeak(LockWeakTE<'s, 't>),
    BorrowToWeak(BorrowToWeakTE<'s, 't>),
    LetNormal(LetNormalTE<'s, 't>),
    Unlet(UnletTE<'s, 't>),
    Discard(DiscardTE<'s, 't>),
    Defer(DeferTE<'s, 't>),
    If(IfTE<'s, 't>),
    While(WhileTE<'s, 't>),
    Mutate(MutateTE<'s, 't>),
    Restackify(RestackifyTE<'s, 't>),
    Transmigrate(TransmigrateTE<'s, 't>),
    Return(ReturnTE<'s, 't>),
    Break(BreakTE<'s, 't>),
    Block(BlockTE<'s, 't>),
    Pure(PureTE<'s, 't>),
    Consecutor(ConsecutorTE<'s, 't>),
    Tuple(TupleTE<'s, 't>),
    StaticArrayFromValues(StaticArrayFromValuesTE<'s, 't>),
    ArraySize(ArraySizeTE<'s, 't>),
    IsSameInstance(IsSameInstanceTE<'s, 't>),
    AsSubtype(AsSubtypeTE<'s, 't>),
    VoidLiteral(VoidLiteralTE<'s, 't>),
    ConstantInt(ConstantIntTE<'s, 't>),
    ConstantBool(ConstantBoolTE<'s, 't>),
    ConstantStr(ConstantStrTE<'s, 't>),
    ConstantFloat(ConstantFloatTE<'s, 't>),
    ArgLookup(ArgLookupTE<'s, 't>),
    ArrayLength(ArrayLengthTE<'s, 't>),
    InterfaceFunctionCall(InterfaceFunctionCallTE<'s, 't>),
    ExternFunctionCall(ExternFunctionCallTE<'s, 't>),
    FunctionCall(FunctionCallTE<'s, 't>),
    Reinterpret(ReinterpretTE<'s, 't>),
    Construct(ConstructTE<'s, 't>),
    NewMutRuntimeSizedArray(NewMutRuntimeSizedArrayTE<'s, 't>),
    StaticArrayFromCallable(StaticArrayFromCallableTE<'s, 't>),
    DestroyStaticSizedArrayIntoFunction(DestroyStaticSizedArrayIntoFunctionTE<'s, 't>),
    DestroyStaticSizedArrayIntoLocals(DestroyStaticSizedArrayIntoLocalsTE<'s, 't>),
    DestroyMutRuntimeSizedArray(DestroyMutRuntimeSizedArrayTE<'s, 't>),
    RuntimeSizedArrayCapacity(RuntimeSizedArrayCapacityTE<'s, 't>),
    PushRuntimeSizedArray(PushRuntimeSizedArrayTE<'s, 't>),
    PopRuntimeSizedArray(PopRuntimeSizedArrayTE<'s, 't>),
    InterfaceToInterfaceUpcast(InterfaceToInterfaceUpcastTE<'s, 't>),
    Upcast(UpcastTE<'s, 't>),
    SoftLoad(SoftLoadTE<'s, 't>),
    Destroy(DestroyTE<'s, 't>),
    DestroyImmRuntimeSizedArray(DestroyImmRuntimeSizedArrayTE<'s, 't>),
    NewImmRuntimeSizedArray(NewImmRuntimeSizedArrayTE<'s, 't>),
}
```

**Verify the variant list against the file.** The authoritative source is every stub in `expressions.rs` whose Scala `/* */` block ends with `… extends ReferenceExpressionTE {`. I counted 47 payload struct stubs in the file (not 38 — `quest.md` §1.5 and §7.1 cite "~38" but were rough estimates; the file has more). Some of those 47 are `AddressExpressionTE`-extending (5 of them — see below), so ~42 stay as `ReferenceExpressionTE` variants. **Trust the file, not the estimate.**

### `AddressExpressionTE<'s, 't>` — 5 variants

One variant per Scala `case class … extends AddressExpressionTE`:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum AddressExpressionTE<'s, 't> {
    LocalLookup(LocalLookupTE<'s, 't>),
    StaticSizedArrayLookup(StaticSizedArrayLookupTE<'s, 't>),
    RuntimeSizedArrayLookup(RuntimeSizedArrayLookupTE<'s, 't>),
    ReferenceMemberLookup(ReferenceMemberLookupTE<'s, 't>),
    AddressMemberLookup(AddressMemberLookupTE<'s, 't>),
}
```

(Verify by grepping for `extends AddressExpressionTE` in the file's Scala blocks.)

### `ExpressionTE<'s, 't>` — 2-variant wrapper

Per §7.1, the wrapper used only where Scala typed a field as the broad `ExpressionT`:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExpressionTE<'s, 't> {
    Reference(&'t ReferenceExpressionTE<'s, 't>),
    Address(&'t AddressExpressionTE<'s, 't>),
}
```

**Narrow-use rule (§7.1).** The only payload struct that uses `ExpressionTE` is `ConstructTE.args` (Scala: `args: Vector[ExpressionT]`). Every other sub-expression slot in every other node uses the narrower `&'t ReferenceExpressionTE<'s, 't>` or `&'t AddressExpressionTE<'s, 't>`. When in doubt, check what the Scala field is typed as — if it's `ReferenceExpressionTE` / `AddressExpressionTE`, use the narrow version; only if it's bare `ExpressionT` do you reach for the wrapper.

### `IExpressionResultT<'s, 't>` — 2-variant, inline Copy, not a tree node

Computed *type* of an expression. Returned from the (panic-stubbed) `fn result(&self) -> …` method on every node. It's a small inline value — not arena-allocated, not a tree node, not part of any of the three enums above.

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IExpressionResultT<'s, 't> {
    Reference(ReferenceResultT<'s, 't>),
    Address(AddressResultT<'s, 't>),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceResultT<'s, 't> {
    pub coord: CoordT<'s, 't>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressResultT<'s, 't> {
    pub coord: CoordT<'s, 't>,
}
```

The existing file has `AddressResultT` and `ReferenceResultT` already typed correctly (`{ pub coord: CoordT<'s, 't> }`). You're just replacing the `IExpressionResultT::_Phantom` stub with the 2-variant enum.

### There's also the (misnamed) `ExpressionT` enum stub

Line 116 of the current file has:

```rust
pub enum ExpressionT<'s, 't> { _Phantom(…) }
/*
trait ExpressionT  {
*/
```

This is the wrapper enum — rename its Rust identifier from `ExpressionT` to `ExpressionTE` per quest.md §7.1, then fill with the 2 variants above. The Scala block stays byte-identical (it says `trait ExpressionT`; that's the Scala name, and the hook freezes the block content). See Gotcha 3.

---

## Per-variant shapes (reading the Scala blocks)

Every Scala `case class` in a `/* */` block is the source of truth for its Rust payload struct. Below is a quick table of the tricky ones; for everything not listed here, the translation is mechanical — read the Scala `case class`, map fields via the table above, flip sub-expr fields to `&'t`, flip `Vector` to `&'t [...]`.

| Stub | Fields (Scala → Rust post-flip) |
|---|---|
| `LetAndLendTE` | `variable: ILocalVariableT<'s,'t>`; `expr: &'t ReferenceExpressionTE<'s,'t>`; `target_ownership: OwnershipT` |
| `LetNormalTE` | `variable: ILocalVariableT<'s,'t>`; `expr: &'t ReferenceExpressionTE<'s,'t>` |
| `UnletTE` | `variable: ILocalVariableT<'s,'t>` |
| `RestackifyTE` | `variable: ILocalVariableT<'s,'t>`; `source_expr: &'t ReferenceExpressionTE<'s,'t>` |
| `LocalLookupTE` | `range: RangeS<'s>`; `local_variable: ILocalVariableT<'s,'t>` |
| `DeferTE` | `inner_expr: &'t ReferenceExpressionTE<'s,'t>`; `deferred_expr: &'t ReferenceExpressionTE<'s,'t>` |
| `IfTE` | three `&'t ReferenceExpressionTE<'s,'t>` sub-exprs |
| `WhileTE` | `block: BlockTE<'s,'t>` — note: inline `BlockTE`, not `&'t BlockTE`. `BlockTE` itself has `inner: &'t ReferenceExpressionTE<'s,'t>`. Verify against the Scala block (`case class WhileTE(block: BlockTE)` vs `case class BlockTE(inner: ReferenceExpressionTE)`). Decision rule: if the Scala field holds a payload struct (not a trait), keep it inline by value; only flip to `&'t` when the Scala type is the broad trait. Applies to a handful of composition points: `WhileTE.block`, `IfTE` conditional sub-trees if they were `BlockTE` (but they're not — Scala uses `ReferenceExpressionTE`, not `BlockTE`), etc. |
| `MutateTE` | `destination_expr: &'t AddressExpressionTE<'s,'t>`; `source_expr: &'t ReferenceExpressionTE<'s,'t>` |
| `ConsecutorTE` | `exprs: &'t [ReferenceExpressionTE<'s,'t>]` |
| `TupleTE` | `elements: &'t [ReferenceExpressionTE<'s,'t>]`; `result_reference: CoordT<'s,'t>` |
| `StaticArrayFromValuesTE` | `elements: &'t [ReferenceExpressionTE<'s,'t>]`; `result_reference: CoordT<'s,'t>`; `array_type: &'t StaticSizedArrayTT<'s,'t>` |
| `FunctionCallTE` | `callable: &'t PrototypeT<'s,'t>`; `args: &'t [ReferenceExpressionTE<'s,'t>]`; `return_type: CoordT<'s,'t>`. **Note**: `PrototypeT` is interned (Slab 3), so it's a `&'t` ref. Check the current stub's type — if it says `callable: PrototypeT<'s,'t>` by value, flip to `&'t`. |
| `InterfaceFunctionCallTE` | `super_function_prototype: &'t PrototypeT<'s,'t>`; `virtual_param_index: i32`; `result_reference: CoordT<'s,'t>`; `args: &'t [ReferenceExpressionTE<'s,'t>]` |
| `ExternFunctionCallTE` | `prototype2: &'t PrototypeT<'s,'t>`; `args: &'t [ReferenceExpressionTE<'s,'t>]` |
| `ConstructTE` | `struct_tt: &'t StructTT<'s,'t>`; `result_reference: CoordT<'s,'t>`; `args: &'t [ExpressionTE<'s,'t>]` — **the only payload using `ExpressionTE`**. See §7.1's narrow-use rule. |
| `DestroyTE` | `expr: &'t ReferenceExpressionTE<'s,'t>`; `struct_tt: &'t StructTT<'s,'t>`; `destination_reference_variables: &'t [ReferenceLocalVariableT<'s,'t>]` |
| `DestroyStaticSizedArrayIntoLocalsTE` | `expr: &'t ReferenceExpressionTE<'s,'t>`; `static_sized_array: &'t StaticSizedArrayTT<'s,'t>`; `destination_reference_variables: &'t [ReferenceLocalVariableT<'s,'t>]` |
| `LockWeakTE` | `inner_expr: &'t ReferenceExpressionTE<'s,'t>`; `result_opt_borrow_type: CoordT<'s,'t>`; `some_constructor: &'t PrototypeT<'s,'t>`; `none_constructor: &'t PrototypeT<'s,'t>`; `some_impl_name: IdT<'s,'t>`; `none_impl_name: IdT<'s,'t>`. Note: `IdT` is interned (Slab 2) but has a custom `Hash`/`Eq` — holding it by value is fine per Slab 2 convention; it's 24 bytes (package_coord + init_steps + local_name). Check the existing stub — if `id` fields say `IdT<'s,'t>` by value, keep by value (doesn't need `&'t`). |
| `UpcastTE` | `inner_expr: &'t ReferenceExpressionTE<'s,'t>`; `target_super_kind: ISuperKindTT<'s,'t>`; `impl_name: IdT<'s,'t>` |
| `AsSubtypeTE` | `source_expr: &'t ReferenceExpressionTE<'s,'t>`; `target_type: CoordT<'s,'t>`; `result_result_type: CoordT<'s,'t>`; `ok_constructor: &'t PrototypeT<'s,'t>`; `err_constructor: &'t PrototypeT<'s,'t>`; `impl_name: IdT<'s,'t>`; `ok_impl_name: IdT<'s,'t>`; `err_impl_name: IdT<'s,'t>` |
| `ConstantIntTE` | `value: ITemplataT<'s,'t>`; `bits: i32`; `region: RegionT`. (Drop the `_phantom` field — `ITemplataT<'s,'t>` already anchors both lifetimes.) See Gotcha 4. |
| `ConstantBoolTE` / `ConstantStrTE` / `ConstantFloatTE` | `value: T`; `region: RegionT`. The existing stubs carry a `_phantom: PhantomData<(&'s (), &'t ())>` because the struct currently has no `'s`-anchoring field. After filling, `ConstantStrTE` will have `value: StrI<'s>` (Gotcha 5), which anchors `'s`. `ConstantBoolTE`/`ConstantFloatTE` still have no `'s`-anchoring field — **keep their `_phantom` field** (they're fieldless-in-scala, and we've chosen to keep the lifetime params on every AST type for uniformity with the enum). |
| `VoidLiteralTE` | `region: RegionT` + `_phantom` for the same reason as above. |
| `BreakTE` | `region: RegionT` + `_phantom` for the same reason. |

**For everything else**, read the Scala block in the file, apply the translation table, and you're done. If in doubt about whether a field should be inline vs `&'t`, default to: **inline** for small Copy types (`CoordT`, `IdT`, `IVarNameT`, `ILocalVariableT`, `ISuperKindTT`, `OwnershipT`, `VariabilityT`, `RegionT`, `ITemplataT`); **`&'t` ref** for interned heavy things (`PrototypeT`, `StructTT`, `InterfaceTT`, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`); **`&'t` ref** for sub-expressions and sub-`AddressExpression`s; **`&'t [_]`** for collections.

---

## Existing stubs you'll hit

The current file has three kinds of content interleaved with the Scala `/* */` blocks. Know what each one is:

### A. Payload struct stubs (what you fill)

```rust
pub struct LetAndLendTE<'s, 't> {
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: ReferenceExpressionTE<'s, 't>,   // ← flip to &'t
    pub target_ownership: OwnershipT,
}
```

You'll edit these. Flip sub-expr fields, flip `Vec<>` to `&'t []`, add `where 's: 't` if not present, replace `#[derive]` stanza with `#[derive(PartialEq, Eq, Hash, Debug)]` (or add it if missing — the current stubs have no `#[derive]` at all).

### B. `impl` blocks with panic-stubbed methods (leave alone)

```rust
impl<'s, 't> LetAndLendTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = { … }
*/
}
```

**Don't touch the `fn … { panic!(…) }` bodies.** Those are Slab 8+ method-migration work. Same for `fn equals`, `fn hash_code`, `fn new`, `fn variability`, `fn last_reference_expr`, etc. Each is wrapped in its own single-fn `impl` block per the TL-HANDOFF file-layout convention — keep that shape.

### C. `fn new(...)` constructor stubs (leave alone, they're on purpose)

```rust
impl<'s, 't> LetAndLendTE<'s, 't> {
    fn new(
        variable: ILocalVariableT<'s, 't>,
        expr: ReferenceExpressionTE<'s, 't>,  // ← existing param type
        target_ownership: OwnershipT,
    ) -> LetAndLendTE<'s, 't> { panic!("Unimplemented: LetAndLendTE::new"); }
/*
  vassert(variable.coord == expr.result.coord)
  …
*/
}
```

These exist because the Scala case-class body has `vassert` preconditions; per TL-HANDOFF "`fn new(...)` scaffolding stubs", each gets a panic-bodied `fn new` to preserve the API shape as a landing spot for Slab 8+ validation migration. **You do need to update the `fn new` parameter types to match the field-type flip** — if you flip `expr: ReferenceExpressionTE` to `expr: &'t ReferenceExpressionTE`, the `fn new` param signature should flip too, for consistency. Body stays `panic!()`. This flip propagates to ~13 `fn new` stubs (`LetAndLendTE`, `DeferTE`, `ConsecutorTE`, `IsSameInstanceTE`, `RuntimeSizedArrayLookupTE`, `FunctionCallTE`, `ReinterpretTE`, `SoftLoadTE`, `DestroyStaticSizedArrayIntoFunctionTE`, `DestroyStaticSizedArrayIntoLocalsTE`, `DestroyImmRuntimeSizedArrayTE`, `FunctionDefinitionT`, `FunctionHeaderT` — the last two are in `ast/ast.rs`; you'll touch them only if their param types name a sub-expr type).

**Don't add new `fn new` stubs unless the Scala case class has a `vassert` block you're newly exposing.** The existing set is complete.

### D. Free-function stubs at file top and bottom (leave alone)

```rust
fn expression_result_expect_reference<'s, 't>() -> ReferenceResultT<'s, 't> { panic!("Unimplemented: expect_reference"); }
/*
  def expectReference(): ReferenceResultT = { … }
*/
```

These are Scala trait-methods-as-free-fns — part of the slice pipeline's output. Slab 8 wires them into their actual impl blocks or deletes them. Don't touch.

---

## Gotchas

### Gotcha 1 (the big one): flip sub-expression fields from by-value to `&'t`

Already covered in the "Up-front" section. Restating here as the single most important gotcha:

- Single sub-expression → `&'t ReferenceExpressionTE<'s, 't>` / `&'t AddressExpressionTE<'s, 't>`.
- Collection of sub-expressions → `&'t [ReferenceExpressionTE<'s, 't>]` / `&'t [ExpressionTE<'s, 't>]`.
- Every `Vec<T>` on a payload struct → `&'t [T]`. Covers both `Vec<ReferenceExpressionTE>` and `Vec<ReferenceLocalVariableT>` (AASSNCMCX — no `Vec` in arena-pinned types).

**Check after flip:** `grep -n "Vec<" src/typing/ast/expressions.rs` — should return zero hits in `pub struct FooTE` field positions. (It's fine if `Vec` shows up in a Scala `/* */` block, which is frozen, or in a `fn new` parameter list for a stub you haven't touched yet.)

### Gotcha 2 (resolved): wrapper enum `Copy` — payloads aren't `Copy`, but the enum is, via `#[derive]` with inline-value variants

`ReferenceExpressionTE::LetAndLend(LetAndLendTE)` holds the payload by value. But `LetAndLendTE` doesn't derive `Copy`/`Clone` (ATDCX). So can `ReferenceExpressionTE` derive `Copy, Clone`?

**Answer: no.** If a variant's payload isn't `Copy`, the enum can't be `Copy` either — Rust requires all variants to be `Copy` for the enum to derive `Copy`. Same for `Clone`.

So:

```rust
#[derive(PartialEq, Eq, Hash, Debug)]   // NOT Copy, NOT Clone
pub enum ReferenceExpressionTE<'s, 't> { /* … */ }

#[derive(PartialEq, Eq, Hash, Debug)]   // NOT Copy, NOT Clone
pub enum AddressExpressionTE<'s, 't> { /* … */ }
```

This is a real departure from Slab 2/3's "wrapper enums are Copy" philosophy — but it's forced by ATDCX applied to the payloads, and it's the right call. The expression tree is arena-allocated; callers hold `&'t ReferenceExpressionTE` (8 bytes) and don't need to Copy the enum value.

**Consequence for `ExpressionTE`:** since it only holds `&'t ReferenceExpressionTE` and `&'t AddressExpressionTE` (8-byte refs), it CAN derive `Copy, Clone`:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]   // Copy OK — only holds refs
pub enum ExpressionTE<'s, 't> {
    Reference(&'t ReferenceExpressionTE<'s, 't>),
    Address(&'t AddressExpressionTE<'s, 't>),
}
```

Same for `IExpressionResultT` / `ReferenceResultT` / `AddressResultT` — those are tiny value types (just a `CoordT`), all `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`.

**If you're ever tempted to add `Copy, Clone` to `ReferenceExpressionTE` to fix a compile error:** don't. The error is almost certainly a downstream sub-compiler call site that was written against the old `_Phantom` enum (which was trivially `Copy`). Patch the call site with `panic!("Unimplemented: Slab 8 — reconstruct reference to ReferenceExpressionTE")` or `todo!()` instead; don't weaken the ATDCX invariant.

### Gotcha 3: rename `ExpressionT` → `ExpressionTE`

The current stub at line 116 is `pub enum ExpressionT<'s, 't>`. Per `quest.md` §7.1, the Rust name is `ExpressionTE`. Rename the Rust identifier; the Scala block (`/* trait ExpressionT … */`) stays frozen verbatim, because it's the Scala-side name.

This rename propagates to exactly one call site in `expressions.rs` itself (`ConstructTE.args: Vec<ExpressionT<'s, 't>>` → `&'t [ExpressionTE<'s, 't>]`) plus the free-fn stubs `fn expression_result` / `fn expression_kind` that reference `ExpressionT` in their panic bodies (flip those too, they're part of the same slice-pipeline artifact).

Downstream: `grep -rn "ExpressionT[^ETa-z]" src/typing/` for the dregs. `ExpressionT` is easy to mis-grep against `ExpressionTE` / `IExpressionResultT` / `ReferenceExpressionTE` — prefer the word-boundary form. If a sub-compiler file uses `ExpressionT` as a function param, flip to `ExpressionTE`. If compilation still breaks there, patch the body with `panic!("Unimplemented: Slab 8")`.

### Gotcha 4: `ConstantIntTE.value: ITemplataT<'s, 't>` — phantom parameter erased per Slab 3

Scala: `value: ITemplataT[IntegerTemplataType]`. The outer `[IntegerTemplataType]` phantom was erased in Slab 3 (§6.6); `ITemplataT<'s, 't>` is monomorphic. Just use `ITemplataT<'s, 't>`.

The existing stub has `value: ITemplataT<'s, 't>` plus a stray `_phantom: PhantomData<(&'s (), &'t ())>`. Drop the `_phantom` — `ITemplataT<'s, 't>` already anchors both lifetimes.

### Gotcha 5: `ConstantStrTE.value: String` — use `StrI<'s>` instead

Scala: `value: String`. The existing stub has `value: String` (owned, heap-allocated).

Rust option landscape:
- **(a) `String`** — heap allocation inside an arena-pinned struct. AASSNCMCX violation. The destructor won't run on arena drop → leak.
- **(b) `StrI<'s>`** — already-interned scout string. Zero allocation at construction, pointer-identity lookup.
- **(c) `&'t str`** — arena-allocated slice, not interned.

**Use (b): `value: StrI<'s>`.** Rationale: `ConstantStrTE` represents a source-literal string from the Vale program (e.g. `let x = "hello"`). Those string literals are tokenized and interned by the parser into the scout arena long before typing runs. The value coming into typing is always a `StrI<'s>` already. Re-interning to the typing arena or allocating owned `String` is wasteful and breaks AASSNCMCX.

If you hit a call site (in a macro or sub-compiler) that constructs a `ConstantStrTE` from a Rust `&str` literal it made up itself, patch it with `panic!("Unimplemented: Slab 8 — intern string literal via scout arena")` until Slab 8 fixes it.

### Gotcha 6: don't port `vassert` / `vcurious` / `vfail` bodies

Every payload struct's Scala `/* */` block contains method bodies:

```
/*
case class LetAndLendTE(...) extends ReferenceExpressionTE {
  vassert(variable.coord == expr.result.coord)

  (expr.result.coord.ownership, targetOwnership) match {
    case (ShareT, ShareT) =>
    case (OwnT | BorrowT | WeakT, BorrowT) =>
  }

  expr match {
    case BreakTE(_) | ReturnTE(_) => vwat() // See BRCOBS
    case _ =>
  }

  override def result: ReferenceResultT = { … }
*/
```

**All of this is Slab 8+ method work.** Slab 5 only fills the struct's fields. Every existing `fn new(...) { panic!(…) }` stub, every `fn result(...) { panic!(…) }` stub, every `fn equals(...) { panic!(…) }` stub stays with a `panic!()` body.

You should be able to complete Slab 5 **without typing a `vassert` translation or a `match` pattern on any expression variant**. If you catch yourself doing either, you've drifted into Slab 8 work — back out.

### Gotcha 7: `where 's: 't` — add to every payload struct, not just the wrapper enums

Same as Slab 4 Gotcha 14. Any struct with `&'t FooBarT<'s, 't>` fields needs `where 's: 't` in the `impl` header and the struct header, otherwise the compiler errors with "`'s` may not live long enough".

```rust
pub struct FunctionCallTE<'s, 't>
where 's: 't,
{
    pub callable: &'t PrototypeT<'s, 't>,
    pub args: &'t [ReferenceExpressionTE<'s, 't>],
    pub return_type: CoordT<'s, 't>,
}

impl<'s, 't> FunctionCallTE<'s, 't>
where 's: 't,
{
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
    /* … */
}
```

The wrapper enums (`ReferenceExpressionTE`, `AddressExpressionTE`, `ExpressionTE`) compile without the bound because Rust infers it from their `&'t`-ref-holding variants (`ExpressionTE`) or from transitively-constrained payload types (the TE enums). Adding the bound uniformly doesn't hurt; your call.

### Gotcha 8: pre-commit hook on `/* */` blocks (unchanged)

`.claude/hooks/check-scala-comments` does exact-match on every Scala block. Rules from Slab 4 Gotcha 7:

- Don't edit inside `/* */` blocks — frozen content.
- You may delete a Rust stub above a `/* */` block (hook only checks block content). You will NOT need to do this in Slab 5 — no type families collapse into enum variants the way `IEnvEntryT` did in Slab 4.
- You may reorder `/* */` blocks within a file if content stays identical. You will NOT need to do this either.

The typical way to get bitten: you editor-auto-indents whitespace inside a `/* */` block and don't notice. The hook prints the diff when it rejects; read it, revert the block's content, re-stage.

### Gotcha 9: file layout conventions from TL-HANDOFF — re-apply if you move things

Summarized here for convenience (full spec in TL-HANDOFF.md §"File layout + style standards for typing/"):

- `use` imports at file top.
- Scala `package …; import …` block sits below the `use` block, not above.
- Scala `case class` comment sits **above** its Rust `pub struct FooTE`.
- Per-method Scala `def foo` comment sits **inside** an `impl` block, **below** the Rust `fn foo`.
- **One fn per `impl` block** — each fn gets its own `impl<'s, 't> FooTE<'s, 't> { fn foo() { panic!(…); } /* scala */ }` wrapper.
- **Multi-line impl bodies** — don't collapse to one line even for trivial bodies.
- **No empty placeholder impls** — `impl<'s, 't> FooTE<'s, 't> {}` with zero methods gets deleted.
- **One `/* */` per Rust definition** — don't pack multiple definitions into one block.
- **`fn new(...)` scaffolding** — panic-bodied constructor stub for any case class with `vassert`. The existing file already has these for all the cases that need them; don't add or remove.

The file already follows these conventions at the start of Slab 5. As you flip field types in struct stubs, the impl-block shape stays unchanged. If you find yourself *moving* an impl block or *changing* its shape, stop — you've drifted. The only edits are: (a) struct field types, (b) enum variant lists (`_Phantom` → real variants), (c) derive stanzas, (d) parameter types on `fn new` stubs to match the field flip.

### Gotcha 10: downstream sub-compiler files will break — use `panic!("Unimplemented: Slab 8")`

32 files under `src/typing/` reference `ReferenceExpressionTE` / `AddressExpressionTE` / `ExpressionT`. Most pass-through take `ReferenceExpressionTE<'s, 't>` by value as a param — fine, the wrapper enum is still small (its size is the max variant payload, ~80-ish bytes with the big payloads like `AsSubtypeTE`, but still passable by value). **Those call sites don't break.**

What does break:
- Any site that constructs a struct like `LetAndLendTE { variable, expr, target_ownership }` using a by-value `ReferenceExpressionTE` for `expr`. After the flip to `&'t ReferenceExpressionTE`, that construction needs `&interner.alloc(expr)` or similar. **Patch the body with `panic!("Unimplemented: Slab 8 — allocate sub-expression into arena")`** — Slab 8 rewrites these when interner/compiler wiring happens.
- Any site that matches `ReferenceExpressionTE::_Phantom(_)` as a catch-all. After the flip, `_Phantom` is gone; the catch-all either needs to go (match is now exhaustive) or stays as `_ => …` — depends on the site.
- Any site that uses `ExpressionT` (the old name) — flip to `ExpressionTE`.

Expected blast radius: ~10-30 sites across sub-compiler files. All should resolve to either trivial rename fixes or `panic!("Unimplemented: Slab 8")` patches. If a single site needs more than 10 lines of real logic to get compiling, you've drifted — back out and ask the senior.

**Hot tip:**

```bash
rg -n "ReferenceExpressionTE\b|AddressExpressionTE\b" src/typing/ --type rust | rg -v "/\*|\* "
```

gives you the active-code hits (filtering out `/* */` and `*` doc comments). Expect 150+ matches; most are type annotations on value params (fine).

### Gotcha 11: `ConstructTE.args` uses the wide `ExpressionTE` wrapper — everything else uses narrow

Per §7.1's narrow-use rule: `ConstructTE` is the **only** payload that uses `&'t [ExpressionTE<'s, 't>]`. Every other collection of expressions (`ConsecutorTE.exprs`, `TupleTE.elements`, `StaticArrayFromValuesTE.elements`, `FunctionCallTE.args`, `InterfaceFunctionCallTE.args`, `ExternFunctionCallTE.args`) uses the narrow `&'t [ReferenceExpressionTE<'s, 't>]`.

Rationale: closure structs can have **addressible members mixed with reference-valued ones**, so `ConstructTE.args` needs the wide wrapper. No other expression mixes reference and address at the list level.

Rule of thumb when translating: what does the Scala field say? `Vector[ExpressionT]` → `&'t [ExpressionTE<'s, 't>]`; `Vector[ReferenceExpressionTE]` → `&'t [ReferenceExpressionTE<'s, 't>]`. Don't substitute one for the other.

### Gotcha 12: the extractor-object free-fns at the bottom of the file stay panic-stubbed

Lines 2068–2090 ish have:

```rust
fn reference_expr_result_struct_name_unapply<'s, 't>(expr: &ReferenceExpressionTE<'s, 't>) -> Option<StrI<'s>> { panic!("Unimplemented: unapply"); }
/*
object referenceExprResultStructName {
  def unapply(expr: ReferenceExpressionTE): Option[StrI] = { … }
}
*/
```

These are Scala `object X { def unapply }` extractors — used in pattern matching like `case referenceExprResultStructName(name) =>`. They're a Slab 8+ concern. Leave as `panic!()`. Flip the param type from `&ReferenceExpressionTE` to `&ReferenceExpressionTE` (no change) — it's already a borrow; no flip needed.

### Gotcha 13: no `From`/`TryFrom` bridges, no `*ValT` companions

A clarification, since Slabs 2–4 all wrote From/TryFrom bridges:

- **No From/TryFrom bridges.** Slab 2's bridges existed because name wrapper enums hold `&'t FooNameT` and constructors wanted an ergonomic `FooNameT → INameT` path. Here the wrapper enums hold payload **by value** (`ReferenceExpressionTE::If(IfTE { … })`), so you construct via plain variant syntax, no bridge needed. Same for destructuring via `match`. If a downstream caller reaches for a `From` impl, it was expecting the name-hierarchy convention — redirect them to the plain variant syntax.
- **No `*ValT` companions.** Expressions aren't interned (§7.2), so there's no IDEPFL hashmap to key on; no Val.

If a previous slab's work has a pattern you half-remember, double-check whether it applies here before copy-pasting. Slab 5 is structurally the simplest of the 5 so far — when in doubt, do less, not more.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-5.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-5.txt
```

Must print `0`. If it doesn't, stop and ask the senior. Project sets `#![allow(unused_variables, unused_imports)]`, so don't rely on warning counts.

### Step 2: Fill `IExpressionResultT` / `ReferenceResultT` / `AddressResultT`

Small, self-contained, downstream-safe. Do this first.

1. Replace `IExpressionResultT::_Phantom(...)` with `Reference(ReferenceResultT<'s, 't>) | Address(AddressResultT<'s, 't>)`.
2. `ReferenceResultT` and `AddressResultT` already have `{ pub coord: CoordT<'s, 't> }` — just add `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` to each if not present.
3. Add `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` to `IExpressionResultT`.
4. `cargo check --lib` → /tmp/sylvan-slab-5.txt, expect 0 errors.

### Step 3: Fill the ~44 payload structs

Go file-top-to-bottom. For each stub:

1. Read the Scala `case class` in the `/* */` block.
2. Translate fields via the table at the top of this doc. Remember:
   - Sub-expression fields → `&'t ReferenceExpressionTE<'s, 't>` or `&'t AddressExpressionTE<'s, 't>`.
   - Collection fields → `&'t [...]`.
   - `StructTT` / `InterfaceTT` / `PrototypeT` / array `TT` types → `&'t` refs (interned).
   - `CoordT` / `IdT` / `IVarNameT` / `ILocalVariableT` / `ISuperKindTT` / `ITemplataT` / `OwnershipT` / `VariabilityT` / `RegionT` → inline by value.
   - `String` → `StrI<'s>` (Gotcha 5).
3. Drop `_phantom` fields that are no longer needed (e.g. `ConstantIntTE` once `value: ITemplataT<'s, 't>` anchors both lifetimes). Keep `_phantom` for truly fieldless structs (`VoidLiteralTE { region: RegionT, _phantom }`, `BreakTE { region, _phantom }`, `ConstantBoolTE { value: bool, region, _phantom }`, `ConstantFloatTE { value: f64, region, _phantom }`).
4. Add `#[derive(PartialEq, Eq, Hash, Debug)]` to the struct — NOT `Copy`/`Clone`.
5. Add `where 's: 't` to struct header if any field is `&'t _`.
6. If the stub has a `fn new(...)` stub with a sub-expr param, flip the param type to match the field flip. Body stays `panic!()`.

After every ~10 structs, `cargo check --lib` to catch cascading errors early. You may see errors in the existing `fn result` / `fn equals` / `fn new` panic stubs complaining about the field-type mismatch — fix only the parameter types (to match the new field types), not the bodies.

### Step 4: Fill the three wrapper enums

1. Replace `ReferenceExpressionTE::_Phantom` with the ~42 real variants (one per Scala subclass of `ReferenceExpressionTE`). Derive `PartialEq, Eq, Hash, Debug` — NOT `Copy`, NOT `Clone` (Gotcha 2).
2. Replace `AddressExpressionTE::_Phantom` with 5 real variants. Same derive.
3. Rename `ExpressionT` → `ExpressionTE` (Gotcha 3) and fill with 2 variants: `Reference(&'t ReferenceExpressionTE<'s, 't>)` / `Address(&'t AddressExpressionTE<'s, 't>)`. Derive `Copy, Clone, PartialEq, Eq, Hash, Debug`.
4. `cargo check --lib` → /tmp/sylvan-slab-5.txt.

You'll hit downstream errors here — mostly in sub-compiler files that used the old `_Phantom`-erased signature or by-value `ReferenceExpressionTE` fields. Fix per Gotcha 10: rename `ExpressionT` → `ExpressionTE` where it appears, patch struct-construction sites with `panic!("Unimplemented: Slab 8 — arena-allocate sub-expression")` if they need a ref-conversion.

### Step 5: Sweep downstream errors

Now the bulk of compile errors. Handle them in this order:

1. `ExpressionT` → `ExpressionTE` renames across sub-compiler files. Mechanical.
2. By-value construction sites (e.g. `LetAndLendTE { expr: some_ref_expr, … }` when `expr` is now `&'t`). Patch body with `panic!("Unimplemented: Slab 8")`.
3. Pattern-match sites that destructured `_Phantom`. Those either get real pattern arms (if the body is obvious and mechanical — rare) or `_ => panic!("Unimplemented: Slab 8 — match over ReferenceExpressionTE variants")`.

Heuristic: **if a body is more than 5 lines of fix, panic it out instead**. Slab 5 is data-definition; bodies are Slab 8+.

### Step 6: Verify and hand off

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-5.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-5.txt   # must be 0
grep -n "_Phantom\b" FrontendRust/src/typing/ast/expressions.rs
#   ^ should show 0 hits
grep -n "^\s*pub .*: ReferenceExpressionTE<" FrontendRust/src/typing/ast/expressions.rs
#   ^ should show 0 hits (every field is &'t ReferenceExpressionTE<… or &'t [ReferenceExpressionTE<…)
grep -n "^\s*pub .*: Vec<" FrontendRust/src/typing/ast/expressions.rs
#   ^ should show 0 hits
grep -n "^\s*pub enum ExpressionT\b" FrontendRust/src/typing/ast/expressions.rs
#   ^ should show 0 hits (renamed to ExpressionTE)
```

**Never commit. The human handles all commits and tags.** When `cargo check --lib` is clean, skim your own diff, then hand back for review with uncommitted changes in the working tree. If you need a local savepoint mid-slab, `git stash` or a WIP branch — don't commit on `rustmigrate-z`.

Work-order checkpoints:
- Step 2 — Result types (tiny, self-contained).
- Step 3 — Payload structs (the bulk; do in file-order).
- Step 4 — Wrapper enums.
- Step 5 — Downstream error sweep.
- Step 6 — Final `cargo check --lib` green, diff self-review, hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors.
- `src/typing/ast/expressions.rs`:
  - `IExpressionResultT<'s, 't>` is a 2-variant `Reference(ReferenceResultT)` / `Address(AddressResultT)` enum. `Copy, Clone, PartialEq, Eq, Hash, Debug`.
  - `ReferenceResultT<'s, 't>` / `AddressResultT<'s, 't>` each have `pub coord: CoordT<'s, 't>`. `Copy, Clone, PartialEq, Eq, Hash, Debug`.
  - `ReferenceExpressionTE<'s, 't>` has ~42 variants, one per Scala `extends ReferenceExpressionTE` case class. `PartialEq, Eq, Hash, Debug` — NOT `Copy`/`Clone`.
  - `AddressExpressionTE<'s, 't>` has 5 variants. Same derives.
  - `ExpressionTE<'s, 't>` (renamed from `ExpressionT`) has 2 variants holding `&'t` refs. `Copy, Clone, PartialEq, Eq, Hash, Debug`.
  - All ~44 payload structs have real Scala-parity fields. Sub-expression fields are `&'t ReferenceExpressionTE<'s, 't>` / `&'t AddressExpressionTE<'s, 't>`. Collection fields are `&'t [...]`. No `Vec<>` fields. `#[derive(PartialEq, Eq, Hash, Debug)]`, `where 's: 't`, no `Copy`/`Clone`.
- All Scala `/* */` blocks unchanged byte-for-byte.
- All existing `fn result` / `fn equals` / `fn hash_code` / `fn new` / `fn variability` etc. stubs keep `panic!()` bodies. Parameter types on `fn new` stubs are updated to match the field flip.
- The extractor-object free-fns (`reference_expr_result_struct_name_unapply`, `reference_expr_result_kind_unapply`) stay panic-stubbed.
- Downstream sub-compiler files compile (possibly with new `panic!("Unimplemented: Slab 8 …")` patches at construction sites).
- File-layout conventions preserved (one fn per impl, multi-line impl bodies, class-then-impl ordering, Scala block below use imports at top).
- **Do NOT** add `NodeRefT` / `visit_*` / `collect_*` — those are Slab 9+.
- **Do NOT** add interner wiring for expressions — they're not interned.
- **Do NOT** commit. Hand back with uncommitted changes; the human tags `slab-5-complete`.

---

## When you're stuck

- **"recursive type `ReferenceExpressionTE` has infinite size"**: you left a by-value sub-expr field somewhere. `grep -n "pub .*: ReferenceExpressionTE<" src/typing/ast/expressions.rs` — every hit outside a Scala `/* */` block needs to become `&'t ReferenceExpressionTE<'s, 't>`. Same drill for `AddressExpressionTE`.
- **"the trait bound `FooTE<'_, '_>: Copy` is not satisfied"**: you tried to derive `Copy` on `ReferenceExpressionTE` or `AddressExpressionTE`. See Gotcha 2. Remove `Copy` and `Clone` from the derive on those two enums. `ExpressionTE` keeps Copy because it only holds refs.
- **"`ReferenceExpressionTE` cannot be constructed outside of the crate"**: you forgot `pub` on a variant or on the enum itself. Slab 5 preserves all `pub` qualifiers from the stubs.
- **"mismatched types: expected `&'t ReferenceExpressionTE`, found `ReferenceExpressionTE`"** at a sub-compiler call site: that's Gotcha 10 — patch the site with `panic!("Unimplemented: Slab 8 — allocate into arena")` or the fixup `interner.alloc(expr)` if the context is trivially arena-aware. If the fix needs more than a couple lines, panic it out and move on.
- **"cannot find type `ExpressionT` in this scope"** downstream: rename to `ExpressionTE`. Gotcha 3.
- **"'s may not live long enough"**: add `where 's: 't` to the struct / impl header. Gotcha 7.
- **"the method `result` is not a member of type `ReferenceExpressionTE`"**: someone called `expr.result()` expecting a method on the enum. That method isn't Slab 5 work — it's Slab 8's `impl ReferenceExpressionTE { fn result(&self) -> ReferenceResultT { match self { … } } }`. Patch the call site with `panic!("Unimplemented: Slab 8 — dispatch .result() across expression variants")`.
- **Pre-commit hook rejection on a `/* */` block**: you accidentally edited whitespace inside a frozen Scala block. Read the hook's diff output and revert the block's content byte-for-byte. The hook only checks block content, not surrounding code.
- **"I want to port a `vassert` body because it would make my code feel complete"**: don't. Slab 8+ work. Scala body stays in `/* */`, Rust `fn new { panic!(…) }`. If you can't resist, put your would-be port in a side note and hand it back with the slab for the senior to accept or reject — don't bake it into the diff.
- **"I want to add a new type or helper not in the stubs"**: don't. If a downstream file is missing something, patch its call site to panic. Adding to `expressions.rs` outside the existing stub set is Slab 8+ work (or Slab 9+ for visitors).
- **"I want to touch `names.rs` / `types.rs` / `templata.rs` / `env/*.rs` / `ast/ast.rs`"**: don't. Slabs 2/3/4 are frozen. If you think one of those files needs a change to support Slab 5, the shape of Slab 5 probably shifted — ask the senior. The one narrow exception is `ast/ast.rs` `fn new` parameter types (`FunctionDefinitionT::new`, `FunctionHeaderT::new`) if they reference the flipped sub-expr types; flip those params to match, keep bodies `panic!()`.

## Where to file questions

- **Design**: `quest.md` Part 7 is the spec; this doc is the detailed how. If they disagree, `quest.md` wins and you should flag the disagreement. If the Scala `/* */` block is ambiguous, the senior has the Scala repo.
- **Field-type disagreements** (inline vs `&'t`): check the rules table at the top, then Gotcha 1. When in doubt between "inline Copy wrapper" and "`&'t` interned ref", the rule is: **interned Slab 3 payloads (StructTT, InterfaceTT, PrototypeT, array TT types) are always `&'t`; everything else is inline**.
- **Scala semantics** (`vassert`, `BRCOBS`, `RMLRMO`, `DDSOT`, etc.): these are acronym-codes for design rationale. They're documented elsewhere in the Scala repo but irrelevant for Slab 5 — you're not porting the assertions. Ignore.
- **Hook rejections**: read the hook's diff output. Usually accidental whitespace inside `/* */`.

## Final advice

Slab 5 is the mechanical payoff slab. Slabs 2–4 each had a design question that took senior input (Slab 2: DAG of sub-enums, Slab 3: KindT inline-vs-interned, Slab 4: envs-in-`'t` + one-map-per-family). Slab 5 has **no outstanding design question** — Part 7 is accurate, the philosophy is settled by Slab 4 (arena-allocated payload structs, `#[derive(PartialEq, Eq, Hash, Debug)]` no Copy/Clone, `&'t`-refs for interned/sub-expr slots, `&'t [...]` for collections).

What's left is: read the Scala, translate the fields, flip the three sub-expr patterns (`expr: ReferenceExpressionTE` → `&'t ReferenceExpressionTE`, `Vec<...>` → `&'t [...]`, `String` → `StrI<'s>`), fill the three wrapper enums, sweep downstream `panic!("Unimplemented: Slab 8")` patches. If you find yourself writing a body with more than a match arm or two in it, you've drifted — back out.

**Ship-readiness check before handing back:** run the four `grep -n` spot-checks in Step 6. If they all show 0 hits and `cargo check --lib` is 0 errors, you're done.

Good luck.
