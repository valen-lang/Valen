# Metaprogrammed Record/Replay (design sketch)

**Status:** forward-looking design, written 2026-07 on the `pre-squash-ffi-drop` arc.
Waits on: the onion-typing frontend arc, the Backend onion arc, and Mojo-style
comptime metaprogramming (typed comptime generics, `comptime match` on types,
comptime type reflection).

**Update (2026-07): the retirement happened.** The Linear region,
determinism.cpp, ICodec, RCImm's unserialize emitters, and the replay flags
and test harness were deleted from the backend (~5k lines); Vale has no
record/replay until this design lands. The 45 replay tests were converted to
plain single-run imm-FFI tests in `end_to_end_tests/tests/externs.rs` (they
were the only coverage of the opaque-handle FFI surface). File references to
linear.cpp/determinism.cpp below describe the pre-retirement code, recoverable
at the "adjuster simplification" TEMP CHECKPOINT. The prior offset-pointer
scheme this replaces (PSBCBO/PRCBO) is described in the appendix below.
Companion reading: `/Volumes/V/Vale2/vcoord-handoff.md` §"Replay / FFI design for
the own-based world" (the FFI model this builds on), and
verdagon.dev "The Impossible Optimization" (the metaprogramming technique).

The starting sketch (architect's):

```
func serialize<T: AnyType>(file: char*, ptrMapping: HashMap<void*, int>, obj: &T) {
  comptime match &obj {
    case number: &Int => writeInt(file, number)
    case struct_obj: &_ : AnyStructType =>
      comptime for field in tuple_of_references_to_fields(&struct_obj) {
        serialize(file, ptrMapping, &field);
      }
    case dyn_obj: &_ : AnyDynType => // some sort of dynamic dispatch here?
    ...
  }
}
```

This document works that sketch out to its end state. The `ptrMapping`
parameter turns out to be the load-bearing idea; the dyn-dispatch case turns
out to be unnecessary (see §1).


## 0. The punchline

Under the onion-typing FFI model (opaque handles + reentrant export recording),
the successor to the entire Linear region is three small pieces:

| Piece | What it is | Where it lives |
|---|---|---|
| **A. Driver** | per-extern/export wrapper: mode dispatch, call framing, direction asymmetry, debug scramble | generated (metaprogrammed), thin backend hooks |
| **B. Codec** | `serialize<T>` / `unserialize<T>` — bytes for value layers, ids for ref layers | generic Vale, comptime-specialized per type |
| **C. Identity map** | session-lifetime correspondence between record-run refs and replay-run refs | small runtime library |

And the following have **no successor at all** — they dissolve rather than
migrate:

- the PSBCBO address adjuster and stored-form pointers (refs become ids, not
  buffer-internal pointers)
- `predictShallowSize` / dry-run sizing / the bump allocator (the codec streams;
  nothing is pre-sized)
- edge serialize thunks and every dyn-dispatch serialize path (bodies are never
  serialized, so there is nothing to dispatch on)
- cycle/aliasing handling in the codec (the object graph is reproduced by
  deterministic re-execution, not by serialization)
- `mallocStr`-into-buffer, str body serialization entirely

The 2,000-line hand-rolled partial evaluator that is today's `linear.cpp`
becomes roughly a page of generic Vale plus a page of runtime library.


## 1. The load-bearing insight: pointee bodies never serialize

Three facts, all already true or already decided:

1. **Opaque-handle FFI** (this arc): every non-primitive crosses the boundary
   as an opaque handle. C cannot fabricate, dereference, or forge a Vale ref.
   C can only obtain one from (a) Vale passing it out, or (b) C calling an
   exported Vale function.
2. **Reentrant export recording** (exists today: `replayExportCalls`,
   externs.cpp:35; the AASETR side-effect fixtures): when C calls back into an
   exported Vale function during an extern call, that reentrant call is
   recorded, and replay re-executes it. So any object C obtains via route (b)
   is *recreated by real Vale execution* on replay — same order, same values,
   same identity structure.
3. **Determinism of the Vale side**: everything between boundary crossings is
   deterministic, so replay reproduces every Vale-side allocation and RC
   operation without help. (RC parity holds because C-side `Foo_alias` /
   `Foo_dealias` calls are themselves exported calls, recorded and replayed —
   the `feature_alias_dealias` fixture exercises exactly this.)

Consequence: **every ref that ever crosses into Vale refers to an identity the
replay run already has**. There is never a need to serialize an object's body
so that replay can "reconstruct" it — replay already built the real object by
re-executing the code that created it. A ref on the wire is just a name for an
identity both runs know. An id.

What still needs bytes on the wire: **incoming by-value data** — primitives
and OwnInline+exported structs that C computed and returned by value. Their
*fields* recurse structurally... and bottom out at primitives, inline
composites, and refs-as-ids.

Two hard problems from the naive design evaporate here:

- **Dyn dispatch in serialize**: an interface ref is a ref; it becomes an id.
  No tag, no per-impl serialize thunk, no vtable entry. The `case dyn_obj`
  branch in the starting sketch is simply the `AnyRef` branch.
- **Cycles and aliasing**: never traversed. Two refs to one object are two
  occurrences of one id. A cyclic graph is reproduced by re-execution, not
  walked. The codec's recursion runs only over *inline value structure*, which
  is non-recursive **by construction** — an inline recursive struct would have
  infinite size, so the type system already forbids the only thing that could
  make comptime unrolling diverge. `serialize<T>` can be fully
  `@always_inline`d, Futamura-style, with zero termination risk.


## 2. The trait hierarchy (types of types)

Comptime generic parameters are typed; the types mirror the onion `Kind` tree.
The reflection API is the compiler exposing its own Kind structure to
userspace metaprogramming:

```
AnyType
├─ AnyValue                        // "bare" value layers — bytes on the wire
│   ├─ AnyPrimitive                //   Int, Bool, Float, Void
│   ├─ AnyInlineStruct             //   OwnInline composite (incl. share_flavored inline?  no: share is ref'd)
│   └─ AnyInlineArray
│       ├─ AnyStaticArray          //   comptime-known length
│       └─ AnyRuntimeArray         //   runtime length (if inline RSAs exist)
└─ AnyRef                          // onion ref layers — ids on the wire
    ├─ ShareRef<Inner>
    ├─ HeapOwnRef<Inner>
    ├─ BorrowRef<Inner, Region>
    └─ WeakRef<Inner>
```

(`AnyDyn` — "Inner is a sealed interface" — still exists as a trait, but the
codec never needs it; see §1. It remains useful for the general-purpose
serializer of §9.)

## 3. The reflection / intrinsic surface

Small, and notably *safer* than the IR-primitive set an in-backend generator
would need:

| Primitive | Kind | Purpose |
|---|---|---|
| `fields(&obj)` | comptime reflection | tuple of typed references to an inline struct's fields, declaration order |
| `inner<R: AnyRef>` | comptime reflection | peel one ref layer, yielding the inner type |
| `sizeof<T>()`, `alignof<T>()` | comptime, target-aware | unserialize-side allocation of OwnInline temporaries; requires the comptime evaluator to know the target ABI |
| `raw_addr(ref) -> void*` | runtime, read-only | identity-map key |
| `write_bytes` / `read_bytes`, `write_i32` / `read_i32` | runtime I/O | the stream |

That's the whole list. No bump allocator, no pointer-relocation intrinsic, no
reinterpret-a-buffer-slot — the streaming id model needs none of them.


## 4. Layer B: the codec

Contexts (the architect's `file` + `ptrMapping`, bundled):

```
struct Ser {
  file: File;
  out_map: &IdentityMap;      // §5 — shared with the driver, session-lifetime
}
struct Unser {
  file: File;
  in_map: &IdentityMap;
}
```

Serialize — runs on the **record** run, on incoming (C→Vale) by-value data and
on the incoming positions of each recorded call:

```
func serialize<T: AnyType>(ctx: &Ser, obj: &T) {
  comptime match T {
    is Void => {}
    is AnyPrimitive =>
      write_bytes(ctx.file, obj, comptime sizeof<T>());

    is AnyInlineStruct =>
      comptime for field in fields(obj) {      // field: &FieldType — typed!
        serialize(ctx, field);                 // note: &field, not &obj
      }

    is AnyStaticArray =>
      comptime for i in 0 .. comptime len<T>() {
        serialize(ctx, &obj[i]);
      }

    is AnyRuntimeArray => {
      write_i32(ctx.file, obj.len);
      foreach elem in obj { serialize(ctx, &elem); }   // runtime loop, comptime-specialized body
    }

    is AnyRef =>                                // Share | HeapOwn | Borrow | Weak — ALL of them
      write_id(ctx.file, ctx.out_map.id_of(raw_addr(obj)));
      // No body. No first-time special case. No dyn dispatch. See §1.
  }
}
```

Unserialize — runs on the **replay** run, mirroring:

```
func unserialize<T: AnyType>(ctx: &Unser) T {
  comptime match T {
    is Void => void
    is AnyPrimitive => read_bytes<T>(ctx.file)

    is AnyInlineStruct =>
      T(comptime for field_type in field_types<T>() {
        unserialize<field_type>(ctx)           // construct fields in decl order
      })

    is AnyStaticArray => [comptime for ...]     // symmetric
    is AnyRuntimeArray => { n = read_i32(ctx.file); ... }

    is AnyRef =>
      ctx.in_map.ref_for<T>(read_id(ctx.file))
      // Yields the LIVE replay-side ref for that identity — a real object that
      // real re-executed Vale code created. Ownership/RC semantics ride on T:
      // materializing a HeapOwnRef is an ownership transfer (valid because the
      // record run proved C really held it and gave it back); a ShareRef
      // aliases per the recorded _alias/_dealias export-call stream.
  }
}
```

Notes:

- The recursion is entirely over inline structure ⇒ comptime-bounded ⇒ the
  whole thing folds to straight-line loads/stores per type, exactly like the
  blog post's regex. (Not because serialize is hot — it isn't — but the
  elegance is free.)
- There is deliberately no `case AnyDyn`: an interface ref hits `is AnyRef`.
- `unserialize` for inline structs needs "construct from fields in order" —
  either a comptime-generated constructor call or field-wise in-place init;
  surface syntax TBD with the metaprogramming design.


## 5. Layer C: the identity map

One conceptual map per recording session, present in both runs:

- **Record run** (in memory, nothing on disk): at every **outgoing** crossing
  (Vale→C: extern args, export returns), assign the next sequential id to
  `raw_addr(ref)` if unseen. At every **incoming** crossing, look the address
  up and write the id to the recording.
- **Replay run** (in memory): at every outgoing crossing — which occurs at
  the *same point in the deterministic execution* — assign the same next id to
  *its* live ref. At every incoming crossing, read the id and hand back its
  live ref.

Because both runs perform outgoing crossings in identical order, sequential
ids correlate the two address spaces **with nothing written to disk for
outgoing refs at all** — matching the handoff's "Outgoing: nothing recorded;
for pointers, just apply the scramble."

An incoming id with no map entry = C produced a ref Vale never gave it =
undefined behavior made loud: replay aborts with a diagnostic. (A class of
C-side memory bug that today's linear model silently *can't even represent* —
here it's detected.)

**Weak refs**: also just ids; a dangling weak is Vale-side state that replay
reproduces by re-execution. A reserved id 0 covers null/none if the ABI has
one.

**Alternative wire form**: instead of sequential ids, record the outgoing
handle's raw bytes and key incoming lookups on those (the handoff's
"int256 → recordedRefToReplayedRefMap", sized per the handle ABI decision —
8B concrete / 16B interface, see the redo guide). Costs one handle per
outgoing crossing; buys greppable recordings and robustness to crossing-order
drift. Either fits; sequential ids are the minimal design.


## 6. Layer A: the driver

The successor of `replayReturnOrCallAndOrRecord` (externs.cpp:168) and
`buildWriteValueToFile`/`buildReadValueFromFile` (determinism.cpp). Written
once as a stdlib generic whose comptime parameter is the raw extern function
itself — everything else (param types, return type, framing id) derives from
it by reflection:

```
func wrap_extern<comptime raw: F, comptime F: AnyRawExternFunc>
    (args: ...params_of<F>) returns_of<F> {

  comptime if !replay_enabled() {
    return raw(..args);                      // normal build: wrapper folds to the raw call
  }

  match runtime_mode() {
    Normal => raw(..args),

    Recording => {
      write_call_begin(comptime call_id_of<raw>());   // call framing (exists today)
      comptime for arg in args {
        note_outgoing(ctx, arg);             // refs: assign ids in-memory; values: nothing
      }
      r = raw(..scramble_each(args));        // reentrant exports record themselves
      record_incoming<returns_of<F>>(ctx, &r);        // serialize<> per §4
      r
    }

    Replaying => {
      expect_call_begin(comptime call_id_of<raw>());
      comptime for arg in args {
        note_outgoing(ctx, arg);             // C never runs, but outgoing ids MUST still
      }                                      // be assigned to keep both runs' id
                                             // sequences in lockstep (§5)
      replay_export_calls();                 // re-execute C's recorded callbacks
      unserialize<returns_of<F>>(ctx)        // never calls C
    }
  }
}
```

`make_extern_function`'s generated body is then just
`func f(a A, b B) R { return wrap_extern<f__raw>(a, b); }` with `F` inferred.
`call_id_of<raw>` must be stable across the record and replay binaries but
should change when the signature does — see the versioning open question.

Direction asymmetry (from the handoff, now with a mechanism for each cell):

| | outgoing (Vale→C) | incoming (C→Vale) |
|---|---|---|
| **by value** (primitives, OwnInline+exported) | nothing recorded — deterministic | `serialize<T>` bytes |
| **by pointer** (Share, OwnHeap, Borrow, Weak, opaque externs) | id assigned in-memory; debug-mode scramble; nothing recorded | id written; replay maps to live ref |

The **debug scramble** (XOR with per-call key / poison) is the enforcement arm
of the "C never dereferences Vale pointers" invariant that makes §1 sound. It
belongs to this layer and is orthogonal to the codec.

Backend's remaining role here: the actual extern call emission, the runtime
mode flag, file externs — and evaluating the comptime machinery. The mode
dispatch, framing, and per-type codec bodies all stop being hand-emitted
LLVM IR.

### How the wrapper gets called on every FFI crossing

Not by rewriting call sites — by **definition substitution at the extern
declaration itself**, which is the one funnel every crossing already flows
through. And crucially, **this substitution already exists**:
`make_extern_function` (FrontendRust
`typing/function/function_compiler_core.rs:317`) already elaborates every
`extern func f(...)` into (a) an ordinary user-facing Vale function `f`
registered via `add_function` like any other, (b) a separate hidden raw
extern prototype (`ExternFunctionNameValT`), and (c) a generated body for
the wrapper: `Return(ExternFunctionCall(raw_proto, [ArgLookup(0), ...]))`.
Call sites, overload resolution, and stdlib/intrinsic externs all already
bind to the wrapper.

```
extern func f(a A, b B) R;            // what the user (or stdlib) writes

// ...already elaborates (today!) to:
extern prototype f__raw(a A, b B) R;                        // hidden raw
func f(a A, b B) R { return f__raw(a, b); }                 // generated body

// ...and the future change is ONLY the generated body:
func f(a A, b B) R { return wrap_extern<f_proto>(a, b); }
```

So the interception wiring is not future work at all — the one edit is what
body `make_extern_function` generates. Today's record/replay logic hangs off
the *lowering of the raw call node* instead (the backend's
`replayReturnOrCallAndOrRecord` fires when emitting `ExternFunctionCall`);
the migration lifts that content up into the generated body, where it is
visible, typed, and metaprogrammable. Exports get the symmetric treatment:
their C-facing entry wrapper (today hand-emitted in backend `function.cpp`)
becomes a generated framing wrapper calling the real Vale function.

Three properties fall out:

1. **Totality by construction.** There is no way to declare an extern that
   dodges the wrapper, because `extern func` *means* wrapper-plus-hidden-raw.
   Stdlib and compiler-synthesized externs are covered identically — the
   "even the ones the user didn't write" requirement is free.
2. **The instantiator is the specializer.** The wrapper injection happens at
   the *generic* level (pre-instantiator), so the existing monomorphizer
   specializes `wrap_extern<f_proto>` per extern — the compiler's own
   elaborator is the Futamura engine, exactly as in the blog post. No new
   expansion machinery in a later pass.
3. **Zero cost in normal builds.** `wrap_extern` opens with
   `comptime if !replay_enabled { return f__raw(args...) }` — the same
   compile-time gate as today's `enableReplaying` check in
   `replayReturnOrCallAndOrRecord` (externs.cpp:177), so non-replay builds
   fold the wrapper to the raw call.

**The stratification rule (and the recursion hazard).** The replay runtime's
own externs — recording-file I/O, the mode-flag read, the scramble helper —
must be `#[raw]` (tier 0), or the recorder would record its own writes,
recursively. Tier 0 carries a proof obligation: **deterministic or invisible**.
Anything that reads the outside world (argv, files, clocks, env) must be
tier 1 (wrapped). Today's code implicitly has this split (determinism.cpp
calls fopen/fwrite via `globalState->externs` directly) but doesn't enforce
it — and it already leaks: `__vbi_getMainArg` (externs.cpp:724) calls its
`__vale_rt_` helpers directly, bypassing `replayReturnOrCallAndOrRecord`, while
argv *differs* between record and replay runs (`--vale_replay recording.bin`).
A replayed argv-reading program silently diverges today; no test catches it
because `getmainarg_basic` has no replay variant. Under the elaboration rule
this class of bug is unrepresentable: an unwrapped nondeterministic extern
can't be declared by accident, only by writing `#[raw]` and owning the proof.

Requires from the metaprogramming feature set: functions as comptime
parameter values, reflection on function types (`params_of<F>`,
`returns_of<F>`, `call_id_of<raw>`), heterogeneous variadic parameters with
comptime iteration, and argument splat — all within the Mojo-style envelope
this design already assumes (function parameters, `VariadicPack`,
`@parameter for`).

### A vs B: who iterates the arguments

The codec (`serialize<T>` / `unserialize<T>`) is the recursive metaprogrammed
generic in *both* designs. The A/B split is only about the thin driver layer —
who iterates the arguments and threads the framing calls:

- **A — the typing pass iterates.** `make_extern_function` emits, per extern,
  the mode dispatch + framing + one `serialize<T_i>(ctx, arg_i)` call per
  param, each a plain call to the metaprogrammed codec. Needs nothing beyond
  what the codec itself demands. Precedent is strong and local: the
  `GeneratedBody` body-macro registry (function_compiler_core.rs:163-180)
  already generates function bodies this way, and the passthrough body with
  per-arg `ArgLookup`s already exists. Diagnostics are first-class ("extern f:
  param 3 of type X cannot cross because...").
- **B — a variadic stdlib generic iterates.** The typing pass emits only
  `wrap_extern<f__raw>(args...)`; the arg loop lives in comptime Vale.

B is gated on exactly two features beyond the codec's needs:

1. **Prototype/function as an explicit generic argument** — half-built:
   `PrototypeTemplata` and bound-prototype calling
   (`rune_to_bound_prototype`) mean the instantiator already specializes
   generics over prototypes and generic bodies already call handed-in
   prototypes. Since `wrap_extern`'s call site is compiler-generated,
   `make_extern_function` can pass the templata explicitly with **no surface
   syntax at all** — this half is plumbing, not design.
2. **Heterogeneous variadic parameters with comptime iteration**
   (`args: ...params_of<F>`) — genuinely new type-system surface with no
   existing analog. Likely the harder feature and the real gate.

Because both produce the same wrapper body, downstream (hammer, backend, wire
format, tests) is agnostic, and A→B migration is mechanical — **provided A's
generated bodies stay expressible in surface Vale in principle** (no
macro-only superpowers; in particular, how the recording ctx is reached must
be something a stdlib function could also do).

**Migration sequencing** (each step independently shippable):
1. Today — the wrapper exists (`make_extern_function`) but its body is a bare
   passthrough; record/replay content is hand-emitted backend C++ at the
   raw-call lowering.
2. If needed before comptime lands: per-type codec bodies generated via the
   body-macro registry (like drop functions), driver generated by the typing
   pass. Ships after the onion arc alone, zero new language features.
3. **A** — codec becomes the real metaprogrammed `serialize<T>`; typing pass
   still iterates the args and emits the driver. Backend's extern handling is
   already down to raw-call emission.
4. **B** — once prototype-templata passing + heterogeneous variadics land,
   the driver moves into `wrap_extern` and the compiler's role shrinks to
   emitting `wrap_extern<f__raw>(args...)`.


## 7. Worked example: `interfaceimmreturnextern`

Today's hardest replay fixture (C constructs a Firefly via exported
constructor, returns it as IShip), traced through the new model:

**Record run**
1. Vale calls `extern cMakeShip()` → wrapper writes call-begin, calls C.
2. C calls `Firefly_new(42)` — an exported Vale function → reentrant export
   recording notes "export Firefly_new(42) called"; the returned handle gets
   outgoing id, say #7 (in-memory only).
3. C returns that handle as the extern's return value → incoming ref →
   recording gets: `[call cMakeShip] [export Firefly_new args:42] [ret ref id=7]`.

Note what is *absent*: no 16-byte fat struct, no serialized fuel field, no
edge thunk, no adjuster. The recording holds ~three small records.

**Replay run**
1. Reaches the same extern → reads call-begin, does **not** call C.
2. Replays the recorded export call: *real* Vale code runs `Firefly_new(42)`,
   allocating a real Firefly with fuel=42; its handle is assigned outgoing
   id #7 in the replay-side map (same sequence position ⇒ same id).
3. Reads `ret ref id=7` → map yields the live Firefly-as-IShip ref.

The SIGSEGV class this arc spent days on (PSBCBO restoration, §1a of the
followups doc) is not "fixed better" here — the code it lived in has no
successor.

`structimm_with_str_return` (Named{id, label}): under onion, `Named` is
share-flavored ⇒ crosses as a ref ⇒ the whole value is one id; the
`makeNamed`/`fixedLabel` factory calls are recorded exports. Only if it were
ported to OwnInline+exported would `serialize<Named>` bytes appear — and its
`label: str` field is a ref field ⇒ id inside bytes. Str bodies never hit the
wire in either case.


## 8. What dissolves — old machinery → new home

| Today (linear.cpp / determinism.cpp) | New model |
|---|---|
| `topLevelSerialize` dry-run + malloc + real-run | streaming `serialize<T>` — gone as a pattern |
| `predictShallowSize` (LLVMABISizeOfType walk) | comptime `sizeof<T>()` where needed; mostly unneeded |
| address adjuster / PSBCBO stored-form pointers | ids — the concept has no successor |
| `translateBetweenBufferAddressAndPointer` | — |
| `defineEdgeSerializeFunction` (interface thunks) | — (refs are ids; no body ⇒ no dispatch) |
| `defineConcreteSerializeFunction` per-kind emitters | `serialize<T>` comptime specialization |
| RCImm unserialize thunks + `getUnserializePrototype` | `unserialize<T>` |
| `mallocStr` into buffer, str body writes | — (strs are refs ⇒ ids) |
| `receiveUnencryptedAlienReference` | `record_incoming<T>` (generated driver) |
| `Linear` region instance (3 fields, post-cleanup) | `Ser`/`Unser` contexts |
| `regionIdByKind` linear entries, `ICodec` dispatch for linear kinds | — (no linear kinds exist) |
| hand-emitted mode dispatch in `replayReturnOrCallAndOrRecord` | generated `wrap_extern<proto>` |

The `ICodec` extraction from this arc remains the right *current* structure —
it isolated exactly the surface that this design deletes the Linear
implementation of.

### Why the region-shaped problems don't recur

The Linear/IRegion friction (22 ownership stubs, `isLinearKind` leaking into
generic helpers, mixed-universe serialize prototypes) all traced to one
modeling choice: the serialized form was reified as **first-class kinds** —
a shadow type universe (`linearizeReference`, host kinds) whose values flowed
through generic code-gen and therefore needed region-style representation
machinery. That choice was *forced* by the pre-slice-5 FFI, where C compiled
against the linearized layout as a real typed ABI: when the encoded form is
something a C compiler consumes as types, it must be types. The category
error, once the FFI stopped requiring it: modeling a **process** (encoding)
as a **place** (region).

This design returns serialization to a process — generic functions over type
structure. No wire kinds exist, so nothing joins a kind registry, implements
a residency interface, or flows through shared code-gen. What honestly
remains of the old friction: (a) the expression problem transposed — adding
an onion layer touches every codec `comptime match`, but that's
compile-time-checked, single-function-scale, and the layer set is closed by
language design; (b) target-layout knowledge at the leaves (comptime
`sizeof`), the shrunken descendant of `predictShallowSize`; (c) ctx-threading
discipline in the codec, contained because wire values never enter generic
code paths.


## 9. Relationship to a general-purpose serializer

A snapshotting / network / JSON-style serializer — one that must write object
*bodies* for a foreign consumer — is a different feature that shares the same
reflection surface. It is the design where the previously-sketched machinery
genuinely applies: pointer map **with first-sight bodies**, cycle handling via
id-already-assigned back-edges, `AnyDyn` handled by generated per-impl
serialize entries in the itable (tag-before-dispatch on the way back in), and
"register before recurse" ordering in unserialize so cycles resolve. None of
that is record/replay's problem, because record/replay has re-execution and a
live peer; a general serializer has neither. Keeping the two designs distinct
prevents the harder one's complexity from leaking into the simpler one.


## 10. Open questions

1. **Address reuse in the identity map.** Record-run map is keyed by address;
   if an object dies and malloc reuses its address for a *new*
   boundary-crossing object, the map must not conflate them. Mitigations:
   remove entries on dealloc (hook the RC-zero path of crossed objects), key
   by (addr, allocation-generation), or the raw-handle wire form (§5
   alternative) if handles embed generation. Needs a real decision. Upside of
   the dealloc-hook option: C returning a stale handle (a C-side
   use-after-free) becomes a *detected error at record time* (unknown id →
   abort) instead of a silent conflation.
2. **Map entry lifetime / growth.** Entries for refs C holds indefinitely must
   live indefinitely; a long-running recording accumulates. Related to (1) —
   dealloc hooks solve both.
3. **Comptime `sizeof` needs target ABI in the comptime evaluator.** The blog's
   "feed the compiler target details" point, now load-bearing. For
   cross-compilation the comptime evaluator must answer for the *target*.
4. **Recording versioning.** Sequential ids + call framing are stable only for
   an identical binary. A recording should carry a build hash and refuse
   mismatched replays (cheap, decisive).
5. **Threads.** The whole model (and today's) assumes single-threaded
   determinism. Multi-thread record/replay is a research-grade extension —
   out of scope, but the wire format shouldn't paint it out (e.g., leave room
   for a thread-id in call framing).
6. **Inline RSAs** — does the onion world even have inline runtime-sized
   values? If not, `AnyRuntimeArray` drops out of `AnyValue` and every array
   crossing is a ref (id).
7. **Surface syntax** for comptime construct-from-fields in `unserialize`
   (§4 note) — depends on the metaprogramming feature design.
8. **BorrowRef crossing lifetimes** — a borrow inside an incoming value
   references a region; soundness is the typing pass's job (the FFI shape
   table already admits Borrow by-pointer); record/replay just ids it. Verify
   nothing more is needed once borrow-region rules for externs land.
9. **Scramble key: RESOLVED — compile-time constant.** The handoff's "per-call
   key" breaks C legitimately storing handles across calls (a handle scrambled
   under call #1's key returning during call #50 unscrambles into garbage).
   Decision: the key is a **build-time constant** (a `#define`-style input
   handed to the compiler), not even a per-session runtime value. Two reasons
   this is strictly better than a runtime session key: (a) a runtime-generated
   key would itself be a nondeterminism source — C that (illegitimately but
   really) observes handle bits would behave differently run to run, making
   the record run non-representative; a constant key keeps every run of the
   same binary bit-identical. (b) Nothing needs key distribution or setup —
   the generated wrappers embed the constant. Note also: the **replay run
   never unscrambles at all** — C never executes, and incoming refs arrive
   from the recording as ids, not as scrambled handles. Scrambling is a
   record/normal-run concern only. Per-call poisoning survives only as an
   opt-in strict mode for externs declared not to retain handles.
10. **Determinism erosion — OPEN PROBLEM, load-bearing.** This design's
   foundation is "the Vale side is deterministic between crossings." Vale
   historically guaranteed this, but the language is heading in a
   less-deterministic direction, so the property must become *checked or
   enforced* rather than assumed. Address-derived observables are the
   sharpest instance (identity hashing / ptr-to-int / allocation-order
   iteration would diverge record vs replay with no FFI involvement;
   `ref_eq` is safe because the identity correspondence is bijective, an
   identity *hash* is not) — but the general problem is any nondeterministic
   operation. Three enforcement strategies, not mutually exclusive:
   - **Compile-time bar**: an effect-like classification; functions
     transitively touching nondeterministic ops can't compile under
     `--enable_replaying` unless virtualized.
   - **Runtime detection**: divergence checksums — record a rolling digest at
     each crossing (call-site id + arg digests ride the existing framing);
     replay compares and aborts at the *first* mismatch with a location,
     converting silent divergence into loud early failure.
   - **Wrap the source**: generalize tier-1 — every nondeterministic
     primitive becomes a recordable event (record its outcome, replay it),
     exactly as externs are handled; this is how rr treats rdtsc. The tier-0
     rule ("deterministic or invisible") already implies this
     classification; the open work is enumerating the sources and picking
     the mechanism per source.
11. **Vale-as-library: RESOLVED — out of scope.** Record/replay is supported
   only when Vale owns `main`. Embedding Vale in a C application that
   initiates calls is explicitly unsupported for record/replay; no wire-format
   accommodation is made.
12. **Function values crossing: RESOLVED via lowering.** Callbacks lower to a
   closure struct and/or a function pointer, which decomposes the problem:
   the **struct half is just a ref** — an id in the identity map, already
   handled. The **code half**, when a C API demands a bare function pointer
   (qsort-style), must point at the **generated export wrapper**, never the
   raw Vale function — then C invoking it lands in the wrapper and records as
   an ordinary reentrant export call (the framing point is preserved, and the
   recording names the export by call_id, so nothing new is needed on the
   wire). Interface-typed callbacks (the idiomatic Vale shape) avoid code
   pointers entirely: C calls a named generated `invoke_*` export on the
   closure handle. Residual caveat: code pointers can't be scrambled (C
   legitimately calls them) and their bit-values vary under ASLR, so the
   debug tripwire doesn't cover the code half — acceptable, since the invoke
   itself is recorded via the wrapper, and C comparing code-pointer values is
   already in "observing handle bits" territory (see #9, #10).


## 11. Test-port map

Per the handoff, the 16 `*imm*` replay tests were exercising the
bytes-linearization path and port to OwnInline+exported once the Own split
lands. In this design's terms:

- **Codec-bytes tests** (port of the 16): OwnInline structs of primitives,
  nested inline, inline arrays — exercise `serialize`/`unserialize` byte
  paths. The two-value sequencing test (`structimm_with_str_return_twice`
  pattern) ports to "two OwnInline values recorded in sequence."
- **Identity-map tests** (net-new coverage the old model couldn't express):
  ref roundtrip (Vale→C→Vale same id ⇒ `ref_eq` on replay), aliasing (same
  ref crossing twice ⇒ one id), reentrant-creation (the §7 trace), unknown-id
  abort (hostile/broken C), weak-ref crossing, RC parity via recorded
  alias/dealias streams (port of `feature_alias_dealias`).
- **Driver tests**: mode dispatch, framing mismatch abort, scramble
  enforcement (debug-mode deref of a scrambled handle must visibly explode).


## Appendix: the prior scheme this replaces (PSBCBO / PRCBO)

This is the offset-pointer marshaling the retired Linear region used to
serialize imm values — for C consumption and for recording files. It is
preserved here as the design this metaprogrammed scheme replaces; the
implementing code (linear.cpp and its adjuster machinery) is deleted, and the
last working commit is the "adjuster simplification" TEMP CHECKPOINT.

**Pointers in Serialize Buffers Can Be Offsets (PSBCBO).** When Linear writes
into a buffer destined for C, that buffer holds structs whose pointers point to
other places in the *same* buffer, so C can read it directly. But when
serializing for a *recording file* instead of for C, those addresses must be
relative to the file begin (equivalently, the buffer begin) — even though a
file-destined value is still first written into a temporary buffer that's later
`fwrite`n.

So instead of writing a raw pointer into the buffer, Linear subtracts a
**Serialized Address Adjuster** from it. That value is *not* necessarily the
temporary buffer's start address: the recording file may already hold a hundred
prior calls, each with its own temporary buffer, so we might be 10k into the
file now. In that case the adjuster is `(temporary buffer begin addr) - 10k`, and
subtracting it writes the correct file-relative integer. When sending into C
instead, the adjuster is `0`. The adjuster lives on the region object
(`getSerializedAddressAdjuster` reads it).

The Linear region object is created with an **Address Mode** boolean: mode `0`
uses regular pointers; mode `1` uses offsets, and additionally specifies where
in the containing file the buffer begins (e.g. begin `500` means offset `500`
points at the buffer's start).

**Pointers in Registers Can Be Offsets (PRCBO).** One might translate the
offset back to a real pointer when reading it out of the serialized buffer, so a
register always holds a regular pointer. That's *not* how it works: a value in a
register can itself be an offset. The translation happens later, at dereference
time (a load/store through a struct or array field). See PRCBOR
(`docs/notes/LinearRegionNotes.md`) for why.
