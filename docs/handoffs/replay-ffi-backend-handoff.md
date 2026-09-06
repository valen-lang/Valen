# Replay / FFI / backend arc — handoff

**Dormant.** The pipeline is wired end-to-end — the C++ backend already walks the onion — so there is no
backend *stage* to stand up. What's deferred is the **borrow-shape FFI semantics** on top of it: the
target FFI design for the own-based world. Partly landed, partly deferred — name the layer (design vs
code) before acting, because several mechanisms this describes have already been deleted.

**The FFI boundary is no-refcount** (code: landed): refs move/consume across it, no refcount
bookkeeping, and auto-generated accessors consume their receiver. The **Linear region + determinism**
machinery is deleted — no Linear serialization path remains — and so is **Fearless FFI** (the
side-calling stack swap, universal-ref sentinel encryption, and orphan generational handles are gone;
**do not resurrect them**). The old `imm`-extern `replay::*` end-to-end suite is retired in favor of an
externs/goldens harness, and 50 FFI tests are deferred on the borrow-shape backend arc (measure the
deferred set against the ignored-test count before quoting it). The design direction (scramble+map,
move/consume) is still the target, but **several mechanisms named below are already deleted — check the
tree before planning against them.**

Related in-flight todos: `docs/todo/metaprogrammed-record-replay.md`,
`docs/todo/remove-record-replay-on-experimental.md`, `docs/todo/ffi-drop-followups.md`,
`docs/todo/opaque-extern-drop.md`.

## Compile-time-determined FFI representation

C-side dynamic behavior ("does C hold the ref across calls?", "does C mutate?") is NOT discoverable
from the Vale side. We instead determine the FFI shape statically from `(exported?, shareability,
ownership)`:

| FFI shape | Examples | Replay mechanism |
|---|---|---|
| **By value (bytes)** | primitives, OwnInline (exported value layout) | bytes serialization |
| **By pointer (scrambled + mapped)** | Share, OwnHeap, Borrow, Weak, opaque externs | scramble on Vale→C; int256 → `recordedRefToReplayedRefMap` on C→Vale |

Two FFI mechanisms, mapped from the static type. The typing/lowering pass decides which one applies;
the backend follows.

**Ownership at the boundary is move/consume, not refcounted** (code: landed). Representation is
unchanged — by-pointer values stay opaque handles (scramble+map) — but crossing the boundary
*transfers* the reference rather than adjusting a refcount: Vale→C gives the reference up, C→Vale
receives it as owned. There is no bump-now/dec-later bookkeeping at the boundary anymore.

> **SUPERSEDING DIRECTION (agreed, NOT implemented):** params come in **pre-+1'd by the caller,
> externs included** — C receives a *borrow*, never `_dealias`es it, and calls `_alias` only to retain
> it past the call. `FRMACZ` argues for always-OWN on the grounds of *uniformity with ordinary Vale
> calls*; under Valen's anchored-borrow ruling a bare class param is no longer a move, so always-OWN now
> breaks that uniformity rather than preserving it. Returns still transfer. This removes the boundary's
> most error-prone obligation (a forgotten `_dealias` leaks; under the new rule doing nothing is
> correct) and needs no new Vale syntax for the "C keeps it" case. **`FRMACZ` and the extern test corpus
> still describe the old ABI until this lands.**

## Language-level invariant: C can't modify Vale data through pointers

The pointer path stays simple IF C never mutates the bytes behind a Vale pointer. We enforce this as a
language rule. In debug mode, scramble (XOR with a per-call key, or replace with poison bytes) at
Vale→C; unscramble at C→Vale. Accidental C-side dereference produces garbage; well-behaved C that just
stores the pointer is unaffected. This eliminates any need for snapshot-on-pointer-crossing.

## Recording asymmetry

This is design intent to be **rebuilt**, not current code: the recording machinery it names
(`mapRefFromRecordingFile`, `determinism.cpp`, the recording files) was deleted with the
Linear/determinism removal (`6978d3639`), and the record/replay suite was retired. Treat the shapes
below as the target for the eventual replay rebuild, not as live paths.

- **Outgoing (Vale → C)**: nothing recorded for inline values (Vale-side execution is deterministic;
  replay reproduces the outgoing bytes). For pointers, just apply the scramble.
- **Incoming (C → Vale)**: bytes recorded for inline values (C may have legitimately mutated them via
  the value param/return path). For pointers, write int256 and map the recorded ref to the live
  Vale-side ref on replay.

## No out-pointers

C-side mutation of inline values happens **only via by-value params + returns** (or via embedding
inside other by-value params/returns). No `Foo*` out-pointers. This keeps the design uniform — "by
pointer" always means "opaque handle that C holds but doesn't dereference."

## Implications for the Own split

When Own splits into OwnInline + OwnHeap:
- **OwnInline + exported** → by-value path (bytes)
- **OwnInline + not exported** → structurally impossible at FFI (C can't accept inline bytes without
  layout)
- **OwnHeap** → by-pointer path (scrambled + mapped) regardless of export-ness
- **Share** → by-pointer path (scramble+map), identity-bearing. Crossing the boundary **moves/consumes**
  the reference (Vale→C transfers it, C→Vale receives it as owned), not refcount bookkeeping — the
  no-refcount boundary is landed. Share still does NOT linearize to bytes; it stays a
  pointer/handle.

## Replay-test port plan

The old record/replay suite (the 16 `*imm*` + 4 non-imm `replay::*` tests) was **retired** in the
rebase (`18c0e6450`), replaced by an externs/goldens harness, and **50 FFI tests were deferred** on the
borrow-shape backend arc (`1ef78718f`). So this is no longer a "port the `*imm*` tests" task — those
tests are gone. The coverage intent still holds, re-authored against the new harness:

- **Bytes path** (value-data crossing FFI as bytes) — exercised by **OwnInline + exported**, once the
  OwnInline split lands.
- **Pointer + map path** — exercised by `share` and (eventually) OwnHeap; net-new coverage, since the
  retired `imm` path linearized rather than mapping.

Both wait on the OwnInline split and the borrow-shape backend arc; the 50 deferred FFI tests are the
concrete backlog.

## Backend FFI-semantics items (partly landed)

The backend is already wired; these are the FFI-semantics items on top of it. Several were resolved or
mooted when the interop work landed; below is the current state.

1. **`Backend/src/region/common/primitives.h` Own assertions** — `translatePrimitive` asserted
   `referenceM->ownership == Ownership::OWN` for Int/Bool/Float/Void. Our Q1 borrow-shape work made it
   legitimate for primitives to flow non-Own. **Resolution direction (agreed): Option A2** — backend
   eventually accepts non-Own primitive references with the foundation that primitive borrows will
   become real LLVM pointers when `*int_ptr = 42`-style semantics land.
   - *Phase 1* (**landed** — the "primitives Phase-1" work): drop the asserts, always return scalar. The
     `translateType` half in `linear.cpp` is **moot** — `linear.cpp` was deleted with the Linear region.
     `primitives.h` still exists; verify its current assert state before treating Phase 1 as fully done.
   - *Phase 2 (when `*int_ptr` lands)*: `translatePrimitive` dispatches on ownership — scalar for Own,
     pointer for Borrow.
   - **Rejected**: Option B (lower primitives to Own at Rust→C++ FFI in
     `metal_lowerer::lower_coord_to_reference`) — would actively destroy the borrow-flavor info exactly
     when the type system needs it. Do not implement.
2. **Linear-region audit for Own input** — **MOOT.** `determinism.cpp` and the entire Linear region +
   determinism machinery were deleted in `6978d3639`. There is no Linear serialization path left to
   audit.
3. **Scramble/unscramble helper** (debug-mode) — still pending; scramble+map remains the eventual
   direction. Not blocking, but enforces the load-bearing "C-can't-mutate-Vale-data" rule.
4. **13 new backend `// VCOORD:` sites** the rebase brought in (from `ebd6f5bec`) flag code that's
   backwards under the new FFI model — notably `vale.cpp:353` ("every `sharedness == SHARED` gate in
   this exported-header block is backwards") and `rcimm.cpp` (a VCOORD about naming an "unknown" LLVM load value). These are
   the concrete backend cleanup list for when the Backend arc starts; `git grep '// VCOORD:' -- 'Backend/*'`.

Under onion typing, the borrow-of-share dispatch (sub-slice-4b) that would have been needed to complete
the coherent-collapse route becomes a natural consequence of the onion structure — worth revisiting when
the Backend arc starts.

## Lessons learned

*Accumulates wisdom, not events. One or two sentences per entry; prune what nobody can act on.*

- **Name the layer: design vs code.** This mission's shapes are largely *target design*; the machinery
  they name (`determinism.cpp`, the recording files, the `*imm*` replay suite) is already **deleted**.
  A shape described here is a rebuild target, not a live path — check the tree before planning against it.
- **The interop lane is gated on `rust_interop/**` changes**, so a commit that doesn't touch that path
  never runs it even when it breaks it. A change altering what flows *through* the pass (group params,
  new templata shapes) can break the instantiator/interop without touching `rust_interop/**`; run the
  interop lane by hand for such changes.
- **`FRMACZ` describes the superseded always-OWN ABI.** The agreed direction is pre-+1'd params
  (C receives a borrow); the extern test corpus still describes the old ABI until that lands.
