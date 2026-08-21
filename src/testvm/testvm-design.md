# TestVM onion migration — design

Rewire `src/testvm/` (the in-process Vale interpreter) to consume the onion-typed `HinputsI`
instead of the deleted `final_ast`/`ProgramH` types, and decide how the VM models inline
(by-value) types now that onion typing introduces them.

## Design (human-only)

### Handling Inline Structs

For now, every struct in TestVM will be a separate allocation, even inline structs.

That means this:
```
struct Ship { engine Engine; }
struct Engine { fuel int; }
```
will actually be *two* allocations in TestVM, like this C++:
```c++
struct Ship { Engine* engine; };
struct Engine { int fuel; };
```
We'll have this rule: We can **never change the pointer pointing to an "inline" struct.**.
In other words, TestVM can never change Ship to point to a _different_ engine.
When the user says `set my_ship.engine = Engine()` that should do an overwrite of the existing engine.

Notes:
 * We won't do this for primitives yet. That means the testvm won't be able to handle `*my_arg_int = 42;` quite yet. That'll be a followup.
 * When a reference registers itself with a pointee, it should strip off the references.

Soon, when we have group borrowing do invalidation, we'll want to switch our borrow references to work more like weak references.

(In the distant future, we'll have them inline, probably using random generational references.)

We might need to enable placement-destroy and placement-new to really make this work well.

### Double References

TestVM treats double refs (`&&`) as single refs (`&`).
IOW, TestVM treats a `&&Ship` like a `&Ship`. 

We can sometimes produce a `&&Ship`, such as if we have a `my_ref: &Ship = ...; print(my_ref);`, that load of `my_ref` will actually be a `&&Ship` until it's coerced down to a `&Ship`. In this case, TestVM's LocalLoad should produce a `ReferenceV` that thinks it's a `&&Ship`, but it really is just pointing at the `Ship` directly.

Specifically, it removes the outer wrap. So a `& weak Ship` would become a `weak Ship`.

## Design Proposals

<!-- Claude adds concise proposals here. The human ratifies by moving them up into the section above. -->

**S1.** Rewire `src/testvm/` from the deleted `final_ast`/`ProgramH` vocabulary to the onion
`HinputsI` vocabulary. The VM is one of two consumers of `HinputsI` (the other is `backend_ffi`),
driven by a thin driver; the instantiator stays a pure producer that returns `HinputsI` as data.

**S4.** Remove `ownership` and `location` as stored fields on `ReferenceV`. Read the
borrow/weak/owned view from the onion wrap of the reference's kind (`BorrowRefIT`/`WeakRefIT`/
`ShareRefIT`, or a bare kind for owned). Where the VM needs sharedness (`Single`/`Shared`), look it
up from the citizen definition (`StructDefinitionI.sharedness` in
`src/instantiating/ast/citizens.rs`); do not store it on `ReferenceV`.

**S7.** Only a bare-kind (owned) member gets the never-repoint plus overwrite-in-place rule from the
ratified "Handling Inline Structs" direction. A wrapped member (borrow/weak) is a pointer field that
repoints on `set`. The IR gives no help: a member set is one `Mutate` node with no inline-vs-pointer
distinction, so the VM picks the path from the destination member's onion type and asserts the branch
it took, to catch a wrong choice loudly.

**S6.** Rebuild the two vivem tests to run real `.vale` programs through the instantiator (via
`test_source_from_dir`) rather than hand-building IR fixtures, asserting on the computed VON return
value.

## Details

### Current state

The VM compiles and is linked test-only (`#[cfg(test)] pub mod testvm` in `lib.rs`) on the onion IR:
entry points in `vivem.rs` return `IVonData` (via `Heap::to_von`) from a `HinputsI`. Two vivem tests
(`return_7`, `adding`) and one integration test (`simple_program_returning_an_int` in
`integration_tests/tests/smoke_tests.rs`) run real `.vale` programs through the instantiator (S6);
the integration harness `run_compilation.rs` is on the onion path (`InstantiatedCompilation`), the
rest of `integration_tests/tests/mod.rs` commented out pending revival. Only trivial programs run —
most expression arms are still `panic!("unimplemented")` (structs, arrays, if/while, interfaces,
weak, the S7 inline-struct mutate path); `grep -rn 'unimplemented\|vimpl' src/testvm/` lists them.

Run every suite under `--features no_backend` (plain `cargo test`/`build` invokes the intentionally-red
C++ backend via `build.rs`); this is branch-wide, and why the standard fire-commit test gate needs
`fire override green` here.

<!-- Derived from the Design. Each item names its S-number. Empty while we design. -->

## Discussed examples and test cases

### `set container.field = NewStruct()` on an inline struct field (Handling Inline Structs, S7)

For an owned (bare-kind) member, `set` does not swing the parent's member slot to a fresh
allocation. It drops the old contents of the field's allocation and copies NewStruct's members into
that same allocation, keeping the field's `AllocationIdV`.

A bonus falls out: a borrow `&container.field` taken before the `set` sees the new value afterward,
which matches real inline memory. A repoint model would leave that borrow looking at the stale old
object.

The parent construction still allocates the field as its own `AllocationV` (Handling Inline
Structs); only the identity
policy (never repoint) and the mutation policy (overwrite in place) change relative to a pointer
field.

## Background and Current State

### The VM represents every value as a heap allocation

A runtime value is always a `ReferenceV` (`struct ReferenceV` in `src/testvm/values.rs`). It carries
`actual_kind`/`seen_as_kind` (`RRKindV`, stored stripped of wraps), a wrap-derived `ownership:
OwnershipV`, and `num` (the allocation number); `location` is gone and `ownership` is no longer stored
as a coord field (S4). The payload data lives in `KindV` (Void/Int/Bool/Float/Str/Opaque/
StructInstance/ArrayInstance) inside an `AllocationV` held in the heap's `objects_by_id` map
(`struct AllocationV`, `struct HeapV` in `src/testvm/heap.rs`). Even an `Int` is a `KindV::Int`
inside its own `AllocationV`.

### Refcounting and leak-checking are pervasive

`AllocationV` holds `strong_referrers` and `weak_referrers` maps keyed by `IObjectReferrerV`
(`src/testvm/values.rs`). The heap increments and decrements these on local add/remove, member and
array mutate, argument take, and struct construction (`fn increment_reference_ref_count`,
`fn decrement_reference_ref_count`, `fn new_struct` in `src/testvm/heap.rs`). `fn check_for_leaks`
panics if any non-void allocation survives to program end. This is Vale's constraint-reference
checking, expressed as referrer bookkeeping.

### Members and elements are themselves references

`StructInstanceV.members` is `&[ReferenceV]` and `ArrayInstanceV.elements` is `&[ReferenceV]`
(`src/testvm/values.rs`). A struct does not embed its members; it refers to them. Today
`fn mutate_struct` (`src/testvm/heap.rs`) always repoints via `fn set_reference_member`
(`src/testvm/values.rs`).

### The onion IR the VM must consume

`KindIT` (`src/instantiating/ast/types.rs`) is the onion kind with four wrap variants
`BorrowRefIT`/`OwnRefIT`/`ShareRefIT`/`WeakRefIT` around a bare kind. An owned value is a bare kind
with zero wraps; there is no `CoordI`/`OwnershipI`/`LocationI`. `ExpressionIE`
(`src/instantiating/ast/expressions.rs`) is one flat enum with `MutateIE`, `DerefIE`, and named
member lookups (`MemberLookupIE`). The instantiator exposes it via `get_monouts()`
returning `HinputsI` (`src/instantiating/ast/hinputs.rs`).

### A member set is one `Mutate` node

`set container.field = x` lowers to a single `Mutate` (`struct MutateTE` in
`src/typing/ast/expressions.rs`, translated to `MutateIE` at the `ExpressionTE::Mutate` arm of
`src/instantiating/instantiator.rs`). Its `destination_expr` is any expression whose result is a
`BorrowRef` of the storage (asserted in `fn MutateTE::new`); `source_expr` is the new value. There is
no separate inline-member-store versus pointer-member-store node, so the VM decides overwrite-in-place
versus repoint from the destination member's onion type. `Mutate` also yields the old value (its
`result` is the replaced value's type), so the overwrite path must extract the old contents as the
result before writing the new ones.

### What is deleted and unlinked

`src/final_ast/` and `src/simplifying/` are deleted, so `ProgramH`, `CoordH`, `KindHT`,
`OwnershipH`, `LocationH`, `PrototypeH`, `StructDefinitionH`, and `HammerInterner` no longer exist.

### Entry points and execution shape

`fn execute_with_primitive_args` and `fn execute_with_heap` (`src/testvm/vivem.rs`) set up a heap and
call `fn inner_execute`, which finds `main` through the program's export map, runs it, reads the
return value back out with `fn to_von` (`src/testvm/heap.rs`), drops it, and runs the leak check.
`fn execute_node_inner` (`src/testvm/expression_vivem.rs`) is the main expression walk over
`ExpressionIE` arms (most still stubbed — see Details > Current state).

### Externs and tests

`src/testvm/vivem_externs.rs` holds ~40 extern implementations (arithmetic, casts, string ops, array
builtins). The vivem and integration tests are described under Details > Current state.

### The design already ruled on inline in the VM

`docs/architecture/bare-clone-borrow-move-design.md` § "vivem caveat" (as of 2026-08-15) states the
VM keeps treating every value as a separately-allocated heap object and treats a primitive borrow as
"just another referrer to the same allocation." It marks write-through-borrow on primitives as out
of scope, with a note that supporting it would need a primitive-storage rework.

## Open Questions

<!-- None open. Answered questions move to Background (facts) or Design Proposals (decisions). -->
