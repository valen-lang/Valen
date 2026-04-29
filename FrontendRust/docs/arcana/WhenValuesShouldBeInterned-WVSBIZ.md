# When Values Should Be Interned (WVSBIZ)

A value (something without identity) defaults to inline Copy. Three conditions promote it to interned:

1. **Subcollections.** If the value contains a variable-length collection (like a list of template args or parameter types), inlining it would require a heap-allocated `Vec`. Interning lets the collection live as an arena slice (`&'t [T]`) inside the interned payload, avoiding heap allocation entirely.

2. **Size.** If the value is large enough that copying it repeatedly is expensive, interning stores it once in the arena and hands out a thin `&'t` pointer. The pointer is Copy; the large payload is shared.

3. **Enum budget.** If the value is stored as a variant in a Copy enum (like `KindT` or `ITemplataT`), the enum's size is its largest variant. Interning the payload behind `&'t` keeps every variant pointer-sized, keeping the enum small and Copy. This is why `StructTT` and `CoordTemplataT` are interned even though they're individually small — they live inside `KindT` and `ITemplataT` respectively.

If none of these apply, the value stays inline as a Copy value-type. `CoordT`, `OwnershipT`, `RegionT` are examples — small, no subcollections, not stored behind `&'t` in a heterogeneous enum.

**Scala parity overrides the heuristics.** Rust's Interned set is anchored to Scala's `IInterning` trait — types that extend `IInterning` in Scala (sealed traits `INameT` and `ICitizenTT`, plus `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `OverloadSetT`) are Interned in Rust. Types that don't (the Templata variants, `SignatureT`, `PrototypeT`, `KindPlaceholderT`) stay as Value-types per @TFITCX even when they superficially fit one of the heuristics above. The one deliberate divergence is `IdT` — Scala leaves it as a plain case class and gets correct equality from `Vector`'s structural compare; Rust interns it specifically for `&'t [INameT]` slice-pointer-equality performance, which Scala didn't need.

Once a type is classified as Interned, construction must go through the interner — see @SICZ for the privacy enforcement.

**Interaction with identity-bearing types:** Types with identity (definitions, environments, expression nodes) are arena-allocated, not interned, especially if they're outputs of the pass. Interning is specifically for values — structural data where two instances with the same fields are considered equal. The distinction: arena-allocation preserves identity (each `alloc()` produces a unique pointer), while interning erases it (structurally equal values share a pointer).

**Definitional components are arena-allocated, not values.** Types like `ParameterT` and `NormalStructMemberT` don't have independent identity, but they *define* part of an identity-bearing output (`FunctionHeaderT`, `StructDefinitionT`). They are the parameter, they are the member — not a reference to one. They're arena-allocated along with their parent. The test: if something *is* part of a definition, it's arena-allocated; if it *refers to* a definition, it's a value (and may be interned per the rules above).
