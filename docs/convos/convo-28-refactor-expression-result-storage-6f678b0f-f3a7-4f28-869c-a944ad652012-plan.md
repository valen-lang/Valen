# Plan document

Source: `/Users/verdagon/.claude/plans/glimmering-finding-shannon.md`
Session: 6f678b0f-f3a7-4f28-869c-a944ad652012

---

# Plan: Store-the-result refactor for typing `ExpressionTE` nodes (onion)

## Context

`FrontendRust/src/typing/ast/expressions.rs` holds the typing-pass IR expression
hierarchy (`enum ExpressionTE` + ~50 `*TE` payload structs). It is mid-onion-migration
and RED: every per-struct `result()` method still computes a value from the dead
`CoordT` shape (`KindT { coord: .. }`, `KindT::new(ownership, region, kind)`,
`.result().coord.region`), and the five lvalue-lookup nodes still return an
`AddressResultT` that no longer exists.

The architect wants a uniform structural cleanup, seeded by the already-done
`LetAndLendTE`/`BorrowToWeakTE`:
1. **Delete each struct's `result()` method** (keep only the `ExpressionTE::result()`
   enum dispatcher) and **store the result as a field** instead — getters must not
   allocate.
2. **Add a private `_sealed: ()` field** to each struct so it can only be built via its
   `new()` constructor.
3. **Each `new()` computes the `result` field**, allocating any wrap payload
   (`BorrowRefT`/`WeakRefT`/`ShareRefT`) into the arena — replacing the assert-based
   validation `LetAndLendTE` currently does.

This both cleans up the getters *and* is the vehicle for landing each node's onion
result computation.

## Locked decisions (from this session)

- **Flatten the hierarchy.** No `AddressResultT` / `AddressExpressionTE` / `IExpressionResultT`.
  One `ExpressionTE`; `ExpressionTE::result() -> KindT`. A lookup is "a reference
  expression plus a reference": lvalue lookups return **`KindT::BorrowRef` of the
  pointed-at kind** (e.g. borrow-ref of an int element, borrow-ref of a shared-ref class).
- **Per-node natural field type.** Wrap-producing nodes store the specific payload ref
  (`result: &'t BorrowRefT` / `&'t WeakRefT` / `&'t ShareRefT`, as in `LetAndLendTE`);
  everything else stores `result: KindT`. The dispatcher re-wraps per variant
  (`KindT::BorrowRef(e.result)` vs `e.result`) — a heterogeneous match, accepted.
- **Owned = bare kind.** `HeapOwnRef` only appears via an explicit `heap` source; nodes
  that compute an owned value (`NewRuntimeSizedArray`, `StaticArrayFromCallable`, owned
  `Construct`) produce **bare kinds**.
- **`SoftLoadTE` dissolves.** Remove the struct + variant; call sites collapse to the
  lookup's borrow-ref (semantic follow-on, see Phase 4).
- **Region = `RegionT { region: IRegionT::Default }`** for every BorrowRef result, for now.
- **Upcast result = `unimplemented!()`.** `UpcastTE` / `InterfaceToInterfaceUpcastTE`
  constructors leave the result formation as `unimplemented!()`.
- **Str is share-flavored** → `ConstantStrTE` result = `ShareRef(Str)` (a share citizen
  can't be held bare).

## The pattern (applied to every surviving node)

```rust
pub struct FooTE<'s, 't> {
    pub /* children/data fields */,
    pub result: KindT<'s, 't>,   // or &'t BorrowRefT / &'t WeakRefT / &'t ShareRefT
    _sealed: (),                 // private → forces new()
}
impl<'s, 't> FooTE<'s, 't> where 's: 't {
    pub fn new(interner: &'ctx TypingInterner<'s,'t>, /* inputs */) -> FooTE<'s, 't> {
        let result = /* compute; alloc wrap via interner.alloc(BorrowRefT { inner, region }) */;
        FooTE { /* fields */, result, _sealed: () }
    }
    // NO result() method
}
```

- The `interner` param is `self.typing_interner` at every call site
  (`&'ctx TypingInterner<'s,'t>`, `.alloc(v) -> &'t mut T`). Pass it **only** to
  constructors that allocate a wrap payload; bare-kind / passthrough / explicit-result
  constructors don't need it.
- **Dispatcher** (`ExpressionTE::result()`, keep): inline each removed getter —
  wrap nodes → `ExpressionTE::Foo(e) => KindT::BorrowRef(e.result)`; all others →
  `=> e.result`. Keep `ExpressionTE::kind()` delegating to it.
- Standardize the stored field name to `result`; rename existing single result fields
  (`result_reference`, `return_type`, `result_coord`, `common_supertype`,
  `result_result_type`, `result_opt_borrow_type`) to `result`. For wrap nodes whose old
  "pointed-at kind" field (`member_reference`, `element_type`, `result_type2`) equals
  `result.inner`, drop it and read `result.inner`.

## Per-node result table (answers "which expressions change" — all of them)

**Remove (2):** `SoftLoad` (dissolves), `Alias` (already obsolete — referenced at
`convert_helper.rs:179` / `instantiator.rs:1911` but has no struct/variant; keep it gone).

**Wrap-producing → `&'t <WrapT>`, `new()` allocs via interner, `RegionT::Default`:**
| Node | result |
|---|---|
| LetAndLendTE *(done — add `_sealed`, make `new()` alloc instead of assert)* | `BorrowRef(expr.result)` |
| BorrowToWeakTE *(done — add `_sealed`)* | `WeakRef(inner)` |
| LocalLookupTE | `BorrowRef(variable kind)` |
| StaticSizedArrayLookupTE | `BorrowRef(element_type)` |
| RuntimeSizedArrayLookupTE | `BorrowRef(array_type.element_type())` |
| ReferenceMemberLookupTE | `BorrowRef(member kind)` |
| AddressMemberLookupTE | `BorrowRef(member kind)` |
| ConstantStrTE | `ShareRef(Str)` |

**Bare-kind computed → `result: KindT`, no interner:** ConstantIntTE (`Int(bits)`),
ConstantBoolTE (`Bool`), ConstantFloatTE (`Float`), VoidLiteralTE / LetNormalTE /
DiscardTE / RestackifyTE / DestroyTE / DestroyRuntimeSizedArrayTE /
DestroyStaticSizedArrayIntoFunctionTE / DestroyStaticSizedArrayIntoLocalsTE /
PushRuntimeSizedArrayTE (`Void`), ReturnTE (`Never{from_break:false}`), BreakTE
(`Never{from_break:true}`), ArrayLengthTE / RuntimeSizedArrayCapacityTE / ArraySizeTE
(`Int`), IsSameInstanceTE (`Bool`), NewRuntimeSizedArrayTE (bare `RuntimeSizedArray`),
StaticArrayFromCallableTE (bare `StaticSizedArray`).

**Passthrough / already-computed → `result: KindT`:** BlockTE, DeferTE *(has `_sealed`)*,
ConsecutorTE (never-scan/last), MutateTE (destination kind — *flag: old-value semantics
to confirm later*), UnletTE (variable kind), IfTE *(has `_sealed`; keep supertype calc)*,
WhileTE *(has `_sealed`)*.

**Explicit result carried by caller → `result: KindT` (rename existing field):** LockWeakTE,
TupleTE, StaticArrayFromValuesTE, AsSubtypeTE, ArgLookupTE, InterfaceFunctionCallTE,
ExternFunctionCallTE (`prototype2.return_type`), FunctionCallTE, ReinterpretTE, ConstructTE,
PopRuntimeSizedArrayTE, CopyPrimTE.

**Unimplemented → `new()` body is `unimplemented!()`:** UpcastTE, InterfaceToInterfaceUpcastTE.

## Phases

**Phase 1 — `expressions.rs` node sweep.** For each surviving node: add `result` field
(typed per table) + `_sealed`, write/replace `new()` to compute+store `result`, delete the
per-struct `result()`. Fold the `LetAndLendTE`/`BorrowToWeakTE` `new()`s over to
interner-allocating form. Collapse `ExpressionTE::result()` to field reads. Delete
`SoftLoadTE`.

**Phase 2 — construction-site migration.** Route every struct-literal construction through
`new(...)` (they lose `result`/`_sealed`). Blast radius (from exploration):
- `src/typing/expression/expression_compiler.rs` (~49), `local_helper.rs` (~21, mostly the
  now-removed SoftLoad — see Phase 4), `pattern_compiler.rs` (11), `call_compiler.rs` (2)
- `src/typing/macros/**` (~60), `src/typing/function/**` (~14)
- `array_compiler.rs` (5), `convert_helper.rs` (3), `sequence_compiler.rs` (1), `compiler.rs` (1)
- **`src/instantiating/instantiator.rs` — 30 exhaustive field-destructures** (no `..`), which
  read fields and so also need the new field/name updates. (Instantiating is downstream/gated;
  can lag, but will not compile until updated.)
- **Tests do NOT break** — all `*TE { .. }` patterns in `typing/test/**` and
  `integration_tests/**` use `..`; `traverse.rs` never touches fields.

**Phase 3 — dispatcher + `kind()` verification.** Confirm `ExpressionTE::result()` covers all
surviving variants with the wrap/no-wrap split; `kind()` still delegates.

**Phase 4 — SoftLoad call-site collapse (semantic; architect-driven).** Removing `SoftLoadTE`
strands its ~20 producers (`local_helper.rs` bare-use routes, `expression_compiler.rs`,
`pattern_compiler.rs`). Each `SoftLoad(lookup, mode)` collapses to the lookup's `BorrowRef`
result (Use), with Move/other modes handled by whatever replaces the load-mode axis. Recommend
doing this as a focused follow-on rather than mixing into the mechanical sweep.

## Out of scope / follow-ons (flag, don't do here)

- `AddressExpressionIE` on the instantiator side (`instantiating/ast/expressions.rs:148`) and
  `closure_tests.rs:431` still assume the split — reconcile when instantiating relinks.
- `MutateTE` old-value result semantics; potential `AddressMemberLookup`/`ReferenceMemberLookup`
  collapse (kept separate for now).
- The rest of the onion value-model dissolution (`CoordT`/`OwnershipT`/`LocationT`) — this
  refactor consumes the new `KindT` wraps but is not the dissolution itself.

## Verification

Tree is RED (mid-dissolution), so no full build/test yet. After Phase 1–3:
- `cargo check --manifest-path FrontendRust/Cargo.toml --lib > tmp/onion-arc.txt 2>&1`
  and confirm `expressions.rs` itself is clean (no `result()`-related, no `AddressResultT`,
  no `SoftLoad` errors), and that error count moves down / doesn't add new *kinds* in the
  files touched.
- Grep-assert: zero `fn result(` remain in `expressions.rs` except the enum dispatcher;
  zero `KindT { coord:` / `KindT::new(` / `.coord` residue in the rewritten bodies.
- Full `cargo test --lib` only once the broader typing dissolution reaches green (the
  three-phase call-checking plan `spicy-waddling-quasar.md` depends on the same milestone).
