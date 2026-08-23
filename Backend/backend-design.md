# Improved Onion-Style Backend Design

Living source of truth for reshaping the C++ backend to consume the onion IR.
Plan context: `docs/plans/complete-backend-plan.md` (Step 4).

## Design (human-only)

 * For now, rcimm.cpp should only be called for strings (and eventually, classes instead of strings).
 * Never is a zero-sized value type `[0 x i57]`. If we ever execute an instruction involving one of these, something bad has happened.

### Incoming AST (Metal) and Interning

(We should rename it from Metal, that name is taken by Apple's Metal)

Backend interns:

 * Types
 * Prototype
 * InterfaceMethod (prototype + virtual param index)
 * Name
 * PackageCoordinate
 * RegionId (might go away soon)

Not interned:

 * Expressions
 * `Local`
 * Definitions (StructDefinition, InterfaceDefinition, array definitions, Function)
 * StructMember, Edge

### Ref

Ref is a wrapper struct around an LLVMValueRef and an "actual type". It represents the result of a previous instruction, and our knowledge of its actual type from back then.

 * As many places as possible in the backend think in terms of Ref as possible, instead of raw LLVMValueRefs.
 * LLVM needs LLVMValueRefs for its instructions, but to get one out of a Ref, we must call checkValidReference.
    * checkValidReference requires an expected type `Kind* refM` argument (which generally only comes from pre-backend stages).
 * To make a Ref, we need to call toRef. (Note, this isn't enforced, but should be. Ref's constructor should probably be private...)
    * toRef takes an "actual type" Kind* (which generally only comes from pre-backend stages) and a LLVMValueRef (or a LLVMValueRef wrapper).

The above three measures combine to mean that these two are always equal:
 * The type that came out of the compiler's last instruction (Ref's "actual type").
 * The type that we expect now for the current instruction (argument to checkValidReference).

Ref is high-level, and always mirrors the user's intent for a reference. It's meant to think about things in Valen terms.

Note: Ref is currently mis-named now that we have onion typing. Ref doesn't mean "reference" anymore. It can mean just a plain int/bool/whatever too. Long term we should rename it to KindM or TypeM or something.

### PtrLE structs (WrapperPtrLE/InterfaceFatPtrLE/WeakFatPtrLE/ControlBlockPtrLE/etc)

These are a bit lower-level than Ref, and have a somewhat different purpose. LLVM in version 16 removed the types from pointers, as if every pointer is actually a `void*`. Since there's no way to get a LLVM pointer's current type, we made our own structs that act like the old LLVM pointers.

However, these aren't always just pointers. WrapperPtrLE/ControlBlockPtrLE are, but InterfaceFatPtrLE/WeakFatPtrLE are actually tuples of a pointer and other things.

Ever since removing regions, it's a bit unclear whether we really need both Ref and the PtrLE structs. Until we remove them, just regard them as high-level (Ref) and low-level (PtrLE) but mostly filling the same purpose.

### Instructions Specify Involved Types

Each instruction comes with the types it expects to work with:
 * Its relevant input types, supplied by the instantiator. These should equal the result types of the source expressions.
    * These should never be calculated, because then our comparisons would be tautologies.
 * Its result type it expects to produce, also supplied by instantiator.
    * This also isn't calculated, because that would need an interner argument, which is mildly annoying.
When a type is guaranteed to be a certain shape (e.g. `ArrayLookup` always takes a borrow ref to an array), then it should be that specific KindI variant (`BorrowRefI(...)`), not a general `KindI`.

Examples:
 * `Deref(my_ref_ref, &&Ship)` has a `source_type` of `&&Ship`, and `resultKind()` returns `&Ship`.
 * `ArrayLookup(my_arr, &[3]bool, my_index, int)` has a `array_type` of `&[3]bool` and `index_type` of `int`, and `resultKind()` returns `&bool`.

Respectively:
 * Source type will satisfy the Ref checkValidReference calls, to make sure that the previous instructions produced the right type.
 * Result type will satisfy the toRef argument, to make sure that our instruction produces the right kind of LLVMValueRef.

This catches certain mistakes, like the backend mixing up two subexpressions. For an example `ArrayLookup(my_arr, &[3]bool, my_index, int)`, the backend's ArrayLookup lowerer could accidentally do this problematic sequence:
 * Evaluate `my_arr`, produce a `Ref(&[3]bool, LLVMValueRef)`, assign it into local `indexExpr` (BUG!).
 * Evaluate `my_index`, produce a `Ref(int, LLVMValueRef)`, assign it into local `arrayExpr` (BUG!).
 * `checkValidReference(lookup.array_type, arrayExpr)` checks if `lookup.array_type` (which is `&[3]bool`) matches `arrayExpr.type` (which is `int`), which it doesn't so it PANICS (which is good).
So, this helps us keep various expressions straight.

## Design Proposals

*Desired end-state only, at a single consistent point in time. No how-to (that goes in Details).*

- **S7.** A *wrapper struct* (a header wrapping the inner value) exists only for heap/RAM-resident objects, never for inline register/stack values, which use the inner struct directly. The wrapper's reason to exist is to later carry a probabilistic generational reference in the generated unsafe-region code — a runtime sanity check, a last line of defense that catches borrow-checker mistakes.
- **S6.** `translateType` of a value kind returns the value's *own* LLVM representation, not a pointer to it: a struct → its inner struct value, a static-sized array → its inner array value, an interface → its ref struct. Any pointer comes from a reference wrap (borrow/own), never baked into the value kind. (Runtime-sized arrays stay by-pointer — they're heap.)
- **S5.** Destructuring a struct loads the whole struct value into a register once (a single load of its inner struct), then copies each field out with an `extractvalue` into the corresponding destination local — not a per-field pointer load.
- **S4.** Reach full onion directly, with no heap-only intermediate milestone (the direction testvm took).
- **S3.** A `Kind` carries no `ownership` or `location` field. Ownership is which onion wrap surrounds the kind (or none, for an owned bare kind); placement (inline vs yonder) is derived from the onion shape at codegen.
- **S1.** The C++ backend consumes the onion `HinputsI` directly — full onion, with no `Coord`/`Ownership`/`Location`.

## Details

*Each item names the Design item(s) it derives from.*

- Remove `Kind.location`/`Kind.ownership` and the coupling asserts (`Kind` in `Backend/src/metal/types.h`), let the C++ build break, and derive placement from the onion wrap at each break. [S1, S3, S4]
- Delete the dead pre-onion fused nodes and their handlers — `LocalLoad`, `MemberLoad`, `LocalStore`, and the array load/store forms — since the Rust bridge emits only the onion nodes. Implement codegen for the onion nodes the bridge does emit: `Deref`, `LocalLookup`, `MemberLookup`, `Mutate`, the array lookups, `LetAndLend`, `ArraySize`. A local store is `Mutate(destination = LocalLookup(local), source)`, whose `result` is the swapped-out old value. [S1]
- Move the fields the pre-onion nodes carried into codegen-derived computations: `targetOwnership` from the result wrap; member index from the struct layout; placement from the onion wrap; the vtable index-in-edge at the interface call. [S3]

## Discussed Examples and Test Cases

- The onion split the old fused `LocalLoad`/`MemberLoad` into a lookup (an lvalue: a borrow of the storage) plus a `Deref` (the read). A local read is `Deref(LocalLookup(local))`; a member read is `Deref(MemberLookup(struct, name))`.
- A local store is `Mutate(destination = LocalLookup(local), source = newValue)`. The destination is a lookup (a borrow of the storage), never a `Deref`. `Mutate`'s `result` is the old value, matching the swap the old `LocalStore` did.

## Background

### Self-evident from the code

- `mallocKnownSize` (`Backend/src/region/common/common.cpp:469`) does **not** malloc — it `makeBackendLocal`s a stack `alloca` (of `LLVMGetUndef(kindLT)`) and returns a pointer to that stack slot (plus census bookkeeping). So `Unsafe::allocate` currently stack-allocs a wrapper struct and returns a pointer to it.
- `Kind` (`Backend/src/metal/types.h:80`) still carries `Ownership ownership`, `Location location`, `Kind* kind`, and its constructor asserts the coupling (`INLINE ⇒ OWN|MUTABLE_SHARE`; borrow/weak ⇒ `YONDER`) at `:99-104`.
- The onion wrap `Kind` subclasses `BorrowRef`/`OwnRef`/`ShareRef`/`WeakRef` (each `{ Kind* inner; }`) and `USize` exist alongside the still-present `Ownership`/`Location` enums (all in `Backend/src/metal/types.h`).
- Coupling to remove under `Backend/src/`: `->location` ×41, `->ownership` ×84, and ~30 `getReference(Ownership, Location, Kind*)` call sites.
- The Rust bridge emits only the onion nodes — `expr_local_lookup`/`expr_deref`/`expr_member_lookup`/`expr_mutate` and the rest (the `translate_expression` walk in `src/backend_ffi/metal_lowerer.rs:311-339`). It never emits `LocalLoad`/`MemberLoad`/`LocalStore`. `instructions.h` still defines those dead pre-onion nodes, with the previous author's `// TODO: replace ... with this` notes above `LocalLookup`/`MemberLookup`.
- The check-reference enforcement lives in `Ref` (`Backend/src/function/expressions/shared/ref.h:132-153`), not the instruction nodes: `Ref`'s `refM` (type) and `refLE` (value) are private, so the only route to a usable `LLVMValueRef` is `checkValidReference` (`IRegion::checkValidReference` in `Backend/src/region/iregion.h:179`), which forces an independently-supplied expected type (`Kind* refM`). ~170 `checkValidReference` + ~77 `checkValidInternalReference` call sites.
- `WrapperPtrLE`/`InterfaceFatPtrLE`/`WeakFatPtrLE`/`ControlBlockPtrLE`/`LiveRef` (all in `ref.h`) bundle an `LLVMValueRef` with its Vale type (`refM`/`kindM`); the wrapper and control-block ones also carry the LLVM pointee type (`wrapperStructLT`/`structLT`) that opaque pointers no longer carry. They are region-internal representation views; `Ref` is the opaque, cross-instruction handle that hides its representation.

### Documented

- The backend consumes the onion IR; placement (Inline vs Yonder) is derived from the onion shape at codegen, never a carried field. (`docs/plans/complete-backend-plan.md`, Step 4, updated 2026-08-19.)
- The metal-IR headers, the FFI builder layer, the Rust bridge, and the driver were reshaped to onion and compile on the Rust side. (`docs/plans/complete-backend-plan.md`, Step 4 Status, updated 2026-08-19.)
- The FFI, `metal_cache.rs`, and `metal_lowerer.rs` are dumb 1:1 plumbing with no lowering logic. (`docs/handoffs/exp-3-wipbx-handoff.md`, updated 2026-08-19.)

### Undocumented

- HEAD is `2aef9941` ("Backend onion codegen, step 1"); the working tree has uncommitted changes undoing a heap-only backend attempt that went badly.

## Open Questions

- Node naming: keep the onion names (`LocalLookup`/`MemberLookup`/`Deref`/`Mutate`) and delete the backend's dead `LocalLoad`/`MemberLoad`/`LocalStore`, or rename the Rust nodes `LocalLookup`→`LocalLoad` and `MemberLookup`→`MemberLoad`? Leaning: keep the onion names — a lookup yields a borrow of storage (an lvalue), not a read, so "Load" misleads, and keeping the names needs zero Rust-side change.
- Check-reference expected-type source: derive each check's expected type from the callee prototype or the struct/array layout (no redundant node fields), retyping `Ref`/`LiveRef`/`checkValidReference` from `Kind*` to onion `Kind*` — or re-add expected-type fields to the nodes as the pre-onion IR did? Leaning: derive, keeping `Ref`'s private-field discipline as the enforcement. A stronger variant: have the member/element check take the layout source (`structKind, memberName`) and compute the expected type internally, so a call site cannot pass a convenient-but-wrong type.
- Owned-bare-struct placement: an owned value is now a bare kind (zero wraps), so codegen must decide how a bare owned struct is placed (by-value vs. heap) from the kind alone. (Named in `docs/plans/complete-backend-plan.md` Step 4 as the one deferred decision.)
