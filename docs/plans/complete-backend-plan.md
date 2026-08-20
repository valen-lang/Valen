# Plan: Delete the Simplifying (Hammer) Pass — Instantiator Emits Onion IR Straight to the Backend

> **Audience:** a fresh tree with **no access to the conversation that produced this plan.** It is
> deliberately heavy on context. Read "Background" fully before touching code. All paths are relative
> to the repo root. Line numbers are snapshots — grep the named symbol; don't trust the number.

---

## Context — why this change

The Vale frontend pipeline is:

```
parsing → postparsing → higher_typing → typing        (T-IR, "HinputsT")
        → instantiating                                 (I-IR, "HinputsI" / "monouts")
        → simplifying  (a.k.a. the HAMMER; H-IR "ProgramH" / "hamuts", in src/final_ast/)
        → backend (C++/LLVM, in Backend/)   and   TestVM (src/testvm/, interprets ProgramH)
```

The **simplifying pass** (`src/simplifying/`, every file `*_hammer.rs`) lowers the instantiator's IR
into `final_ast`'s `ProgramH`, which the C++ backend consumes over FFI and TestVM interprets.

Two things changed the calculus:

1. **The "onion typing" refactor** rebuilt the *typing* pass. It deleted the old
   `CoordT`/`OwnershipT`/`LocationT` value model and replaced it with **`KindT` carrying four
   reference "wrap" layers** — `BorrowRef` / `OwnRef` / `ShareRef` / `WeakRef` — an "onion" of ref
   layers around a base kind. The expression hierarchy flattened to a **single `ExpressionTE`** (the
   old `ReferenceExpressionTE` / `AddressExpressionTE` two-sort split is gone), `SoftLoadTE` dissolved
   into a read-path `DerefTE`, and **addressibility was retired** (an LLVM-style storage model: every
   local is storage; a lookup yields a borrow of that storage).

2. That directly eliminates the hammer's two largest jobs — l-value/r-value flattening + `SoftLoad`
   fusion, and box-wrapping of addressible (mutably-captured) variables. What the hammer did there,
   the onion now does *in typing*. But the onion currently lives **only in typing**; the instantiator,
   simplifying, and final_ast are frozen at the pre-onion checkpoint and commented out of `src/lib.rs`.

**The decision:** rather than revive the pre-onion instantiator/hammer and keep the `Coord`-shaped
`ProgramH`, push the onion all the way to the metal. The instantiator is rewritten to emit an
**onion-typed `HinputsI`**; the simplifying pass and `ProgramH` are **deleted**; the backend + TestVM
are rewired to consume `HinputsI` directly. `HinputsI` becomes the sole IR from instantiation onward.

### The load-bearing principle (do not violate)

**Blast `Coord` away wherever you touch it. Never make new code "work with" the existing
`Coord`/`Ownership`/`Location`/`ProgramH` shapes.** If a task tempts you to adapt to `CoordI`,
`CoordH`, `OwnershipH`, `OwnershipI`, `LocationH`, `LocationI`, or `ProgramH`, stop — replace that
shape with the onion, don't interoperate with it. Memory placement (Inline vs Yonder) is **not** a
carried field in the new world; it is **derived from the onion shape at codegen** (bare kind → value;
ref-wrap → pointer).

### Locked decisions (agreed with the architect; do not re-open)

1. **`HinputsI` is the backend contract.** After this work there is no `ProgramH` / H-IR. The
   instantiator's onion-typed `HinputsI` is what the C++ backend FFI and TestVM consume.
2. **`ProgramH` / `src/final_ast/` / the whole `src/simplifying/` pass are deleted**, not revived or
   re-onioned.
3. **Inline/Yonder (`LocationH`/`LocationI`) goes away as a field/enum.** Placement is derived from the
   onion shape at codegen.
4. **The instantiator output IR is named + onion + monomorphized** — the same vocabulary as typing's
   `ExpressionTE` / `KindT`, minus generics. It is *not* a new `Coord`-flavored dialect. Locals and
   struct members stay **named**; any numbering / member-index resolution (if still needed at all) is a
   backend/codegen concern.
5. **DeferTE moves into the typing pass** (step 1) — drop *timing* becomes explicit in the typed tree,
   which also serves the future borrow checker.
6. **Instantiateds are NOT interned.** They are write-once/read-once, so the I-types are plain
   structural value-types — `MustIntern`, every `*ValI` transient, and the interner's dedup maps
   are deleted; `instantiating_interner.rs` is a bare arena wrapper. (This supersedes the step-2 text
   below about keeping the interner.)
7. **`'s` stays on the instantiator types.** It is load-bearing via source runes (`IRuneS<'s>` key
   the `rune_to_*_bound` maps) and `StrI<'s>` mangling names; removing it would mean re-keying the
   bounds machinery for no real gain.

---

## Background the fresh tree needs

**Current build state (`src/lib.rs`).** The downstream arc is commented out: `backend_ffi`,
`final_ast`, `simplifying`, `instantiating`, `testvm`, `von`, `clang`, `end_to_end_tests`,
`integration_tests`, `file_coordinate_map` are all `// pub mod …`. Only `parsing`, `postparsing`,
`typing`, `pass_manager`, `solver` (etc.) link. So today only the **typing** suite builds/runs.

**Why the downstream passes are stale, not just gated.** The instantiator matches on typing enum
variants that no longer exist (`ReferenceExpressionTE::While/Return/Break` at
`src/instantiating/instantiator.rs:1498,1544,1552`) and imports deleted typing types (`SoftLoadTE` at
`:141`, `AddressibleLocalVariableT` at `:26`). It also *emits* the pre-onion model (`CoordI`,
two-sort `ExpressionIE`, `SoftLoadIE`, addressibility). So this is a **rewrite of the instantiator's
read and write sides**, not a re-link.

**The onion target vocabulary** (what the instantiator's output mirrors) lives in typing:
- **Kinds** — `KindT` (`src/typing/types/types.rs:52-70`, 17 variants). The four wrap structs:
  `BorrowRefT { inner: KindT, region: RegionT }` (`:24`), `OwnRefT { inner }` (`:31`),
  `ShareRefT { inner }` (`:37`), `WeakRefT { inner }` (`:43`) — all stored behind `&'t`. **An owned
  value is a bare kind** (zero wraps), e.g. an owned `Ship` is `KindT::Struct(&StructTT)` directly.
  `RegionT { Iso, Default }` (`:16`) lives only on `BorrowRefT`. **No location/ownership field exists
  anywhere** — ownership is which wrap (or none) surrounds the kind.
- **Expressions** — one flat `ExpressionTE` (`src/typing/ast/expressions.rs:33-84`, 50 variants, no
  Reference/Address split, no `SoftLoad`). `DerefTE` (`:932`) peels exactly one wrap via
  `peel_one_reference`. The five lookup nodes (`LocalLookupTE` `:752`, `ReferenceMemberLookupTE`
  `:874`, `AddressMemberLookupTE` `:903`, the two array lookups `:793`/`:824`) each return
  `KindT::BorrowRef(...)` of their target. `result()` dispatcher at `:90-143`.
- **Locals & members** — a local is `LocalVariable { name: IVarNameT, tyype: KindT }`
  (`src/typing/env/function_environment_t.rs:998`). Members are keyed by interned **name**
  (`IVarNameT`), never index (member lookups' `member_name: IVarNameT`).
- **Key helpers to mirror** — `peel_one_reference` / `is_ref` / `replace_value_type_in_ref`
  (`src/typing/templata_compiler.rs:65-142`), and `substitute_templatas_in_kind` (recurses through all
  four wraps — the precedent for the instantiator's placeholder substitution).

**Caveat: the typing pass is human-edited.** `src/typing/.claude/CLAUDE.md` marks typing as
human-authored — get explicit architect approval before modifying `src/typing/` files (this bears on
step 1). Some typing functions are still stubs (`evaluate_block` panics "Slab 15";
`InterfaceToInterfaceUpcastTE::new` is `unimplemented!`) — don't treat those as authoritative shapes.

---

## Step 1 — Typing handles Defer (delete `DeferTE`) — **DONE (landed on `main`)**

Typing owns deferred temp-drops via a linear `PendingTempDrops` obligation (a `DropBomb`-backed
move-only token); `DeferTE` is retired. The rest of this section is the original design context.


**Goal:** make deferred-drop *timing* explicit in the typed `ExpressionTE` tree, so the downstream IRs
never need a `Defer` node or a defer-timing algorithm.

**Current reality.** Drop *insertion* is already fully in typing. Only defer *timing* lives downstream
(the hammer's "bubble-up-and-flush" accumulator). The pieces:
- `make_temporary_local_defer` (`src/typing/expression/local_helper.rs:41-65`) builds
  `DeferTE { inner_expr = LetAndLend(temp, value), deferred_expr = drop(unlet temp) }`. The deferred
  drop is already a concrete `Unlet`+destructor expression.
- `DeferTE` node: `src/typing/ast/expressions.rs:300-324` (asserts `deferred_expr` is `Void`; result =
  `inner_expr.result()`).
- `drop_since` (`src/typing/expression/expression_compiler.rs:2586-2659`) already runs the isomorphic
  **scope-end** flush: collect live-vars-introduced-since-block-start
  (`get_live_variables_introduced_since`), **reverse for LIFO**, `unlet_and_drop_all` → splice into a
  `Consecutor` (via `consecutive`, `src/typing/compiler.rs:1994`). Its `Never` arm unlets **without**
  dropping (`:2624`). Block ends funnel through it (`src/typing/expression/block_compiler.rs:41-81`);
  `break` funnels through it against the loop env (`expression_compiler.rs:1320-1347`).
- `drop()` (`src/typing/function/destructor_compiler.rs:78-138`) decides `Discard` (primitives, refs,
  str) vs `FunctionCall(destructor)` (struct/interface/array/placeholder) by kind.

**What to build.** Thread a pending-deferred accumulator through typing's expression evaluation
(`evaluate_expression`, `src/typing/expression/expression_compiler.rs:354-363`, returns
`(ExpressionTE, HashSet<KindT>)`), and splice each deferred into a `Consecutor` at the nearest
consuming op / end-of-statement / before `Return`, discarding pending deferreds past a `Never` — reusing
`drop_since`'s LIFO machinery and `consecutive`. Recall `ConsecutorTE::new`
(`src/typing/ast/expressions.rs:507-533`) makes the whole sequence `Never` if any element is `Never`,
which gives the "discard past Never" behavior structurally.

**Then delete `DeferTE`.** Convert the four live `make_temporary_local_defer` callers to emit the
deferred at the flush point directly instead of wrapping in `DeferTE`:
- `src/typing/expression/call_compiler.rs:224-233` (owned callable in `__call`),
- `src/typing/expression/expression_compiler.rs:871-880` (`LoadAsBorrow` of a share rvalue),
- `src/typing/expression/expression_compiler.rs:907-916` (`LoadAsBorrow` of an owning rvalue — the
  `&2` case),
- `src/typing/expression/expression_compiler.rs:922-932` (`LoadAsWeak` of an owning rvalue; note the
  next step is `panic!("unimplemented")` at `:932` — weak-alias not built, so this site is partially
  dead and can be handled minimally).

Remove the `DeferTE` variant and struct once the callers no longer produce it.

**Why first / standalone.** Typing is the only linked suite, so it's fully testable in isolation; it
removes one node kind and the `Vec<deferred>` threading from the step-2 rewrite; and the borrow checker
wants explicit drop timing in the tree anyway.

**Gate:** the typing `--lib` suite stays green. **This step touches `src/typing/` — get architect
approval per the human-edited-pass rule.**

---

## Step 2 — Onion-ify `HinputsI` (blast `Coord`) — **DONE (landed on `main`)**

The instantiator emits onion `HinputsI` and the whole module compiles. Verified by matcher-based
tests in `instantiated_tests.rs`: `fn test` compiles a program, `get_monouts()` yields the onion IR,
and `collect_only_inode!` (over the completed `visit_expression_ie` walker in
`src/instantiating/collector.rs`) pins the onion shape — ownership as ref-wrap layers, `SoftLoad` as
`Deref`, lookups yielding `BorrowRefIT`, monomorphized prototypes. The rest of this section is the
original design context.


**Goal:** rewrite `src/instantiating/` to (a) *consume* the current flat onion `ExpressionTE`, and
(b) *emit* an onion-typed `HinputsI` (no `CoordI`/`OwnershipI`/`LocationI`, no two-sort split, no
`SoftLoadIE`, no addressibility). This is the bulk of the work.

**What stays unchanged (onion-neutral — do NOT rewrite):** the monomorphization spine —
`translate` (`src/instantiating/instantiator.rs:270`) → `translate_method` (`:290`) → the worklist
drain loop (`:433-463`); `assemble_placeholder_map` (`:851`/`:873`); bound discharge in
`translate_prototype` (`:943`, pushes onto `monouts.new_functions` at `:1018`); the callsite-discovery
functions (`translate_function_callsite` `:794`, `translate_impl_callsite` `:782`,
`translate_abstract_func` `:822`, `translate_override` `:685`); all name translation
(`translate_*_name`, `translate_id`); `get_monouts`
(`src/instantiating/instantiated_compilation.rs:148`). (The interner is NOT kept — per locked
decision 6 it collapsed to an arena wrapper; every `intern_*` call became `bump.alloc`.)

**Read side (re-source from onion typing):** rewrite the expression/type translators that match
deleted typing variants —
- Merge `translate_expr` (`:1365`), `translate_ref_expr` (`:1379`, the big match `:1381-1981`), and
  `translate_addr_expr` (`:1295`) into a single translator over the flat `ExpressionTE`. There is no
  Reference/Address split to dispatch anymore; add a `Deref` arm; delete the `SoftLoad` handling.
- `translate_coord` (`:2170`) and `translate_kind` (`:2353`): translate the onion kind, substituting
  `KindPlaceholder`s. The placeholder-substitution + `compose_ownerships` logic (`:2011`/`:2020`/`:2058`)
  is **replaced by wrap-splicing** — splice the concrete onion kind into the wrap structure, mirroring
  typing's `substitute_templatas_in_kind` / `replace_value_type_in_ref`. Ownership is no longer a scalar
  to compose; it's the wrap that surrounds the substituted inner.
- `translate_local_variable` (`:1253`): keep named locals; **delete**
  `translate_addressible_local_variable` (`:1283`) and the addressibility path.
- `translate_struct_member` (`:899`): already name-keyed; carry an onion member type.

**Output IR (onion-ify `src/instantiating/ast/`):**
- `types.rs`: **delete `OwnershipI` (`:12`), `LocationI` (`:43`), `CoordI` (`:65`).** Replace `KindIT`
  (`:90`, 10 variants) with an onion kind mirroring typing's `KindT` — add wrap variants
  `BorrowRefIT { inner, region }`, `OwnRefIT { inner }`, `ShareRefIT { inner }`, `WeakRefIT { inner }`.
  A coord becomes just an onion kind. (Note `CoordI::new` `:78` already bakes one onion invariant —
  primitives are `Own` — which now becomes "primitives are bare kinds.")
- `expressions.rs`: collapse `ExpressionIE` (`:22`, 2-sort) + `ReferenceExpressionIE` (`:94`, 49
  variants) + `AddressExpressionIE` (`:148`, 5 variants) into **one flat `ExpressionIE`** mirroring
  `ExpressionTE`. **Delete `SoftLoadIE` (`:812`), `AddressExpressionIE`, `AddressMemberLookupIE`; add a
  `DerefIE`.** Move the lookup nodes into the single enum (returning `BorrowRef` of their target, like
  typing).
- `ast.rs`: **delete the addressible local/closure variants** in `IVariableI` (`:333`) /
  `ILocalVariableI` (`:356`) — keep only the reference (named) ones. `ParameterI` (`:148`) /
  `FunctionHeaderI` (`:226`) / `PrototypeI` (`:302`) carry onion coords instead of `CoordI`.
- `citizens.rs`: `StructMemberI` (`:41`) stays name-keyed; its `IMemberTypeI` carries an onion coord;
  drop the `AddressMemberTypeI` distinction if it only existed for addressibility.
- `hinputs.rs`: `HinputsI` (`:29`) top-level shape is fine; its contained defs now hold onion types.

**Verification (done):** matcher-based tests in `instantiated_tests.rs` — the `test` source →
`get_monouts()` harness, plus `collect_only_inode!` over the onion IR via the completed
`visit_expression_ie` walker and `NodeRefI` in `src/instantiating/collector.rs`. Array/`v.builtins.*`
fixtures use the `test_with_array_builtins` harness (the default `test` omits builtins). No golden
text dumps — pin shapes with matchers like the other passes. **Do not** run simplifying or downstream
tests.

---

## Step 3 — Delete simplifying and `ProgramH` — **DONE (landed on `main`)**

`src/simplifying/` and `src/final_ast/` are deleted; `HinputsI`/`get_monouts()` is the backend's input.
`pass_manager`/`full_compilation` are rewired off `ProgramH` (the `'h` hammer lifetime is gone), and
`get_astrouts` is dropped — its only provider was the deleted `HammerCompilation`, and the astronomer
step now folds into `TypingPassCompilation::get_compiler_outputs`. The hammer's still-needed mechanical
jobs (placement, member-indexing, name mangling, vtable slots) are re-homed to the backend/codegen
(step 4).

---

## Step 4 — Backend + TestVM think onion

**Status.** The onion metal IR, the FFI builder layer, the Rust bridge, and the driver are reshaped and
compile — validate the Rust side with `CARGO_FEATURE_RUST_INTEROP=1 cargo check --lib`, which skips the
red C++ build. **Done:** `Backend/src/metal/{types.h,ast.h,metalcache.h,instructions.h}` are onion-shaped
(`Reference`/`Ownership`/`Location` retired; kinds carry the four wrap layers + `USize`; `instructions.h`
mirrors `ExpressionIE` 1:1, dormant pre-onion nodes kept aside); `metal_cache_ffi.{h,cpp}` +
`src/backend_ffi/metal_cache.rs` are dumb 1:1 onion builders; `src/backend_ffi/metal_lowerer.rs` walks
`HinputsI` → those builders (name mangling via `instantiated_humanizer`); `pass_manager`/`clang` are
re-linked and rewired to `get_monouts()`. **Remaining:** the C++ codegen readers (below), the deferred
lowerer bits, and re-linking/running the suites.

**The single new rule — implement in C++ codegen only:** placement is a function of the onion shape —
bare primitive/value → Inline; ref-wrap (borrow/weak) → pointer/Yonder. The FFI, `metal_cache.rs`, and
`metal_lowerer.rs` do **no** lowering: they pass the onion `Kind*` straight through. Placement,
member-name→index, deref/load fusion, and vtable-slot resolution all happen at C++ codegen.

**Deferred in `metal_lowerer.rs` (stubbed with TODOs):** edge/vtable reconstruction from
`HinputsI.interface_to_sub_citizen_to_edge`, interface super-lists, and static/runtime array
*definitions* (there is no array-def list on `HinputsI` — collect from the array kinds used).

**C++ codegen (`Backend/`) — the remaining big pass.** `instructions.h`/`types.h` are already onion; now
reshape the *readers*: per-instruction codegen (`Backend/src/function/expressions/*.cpp`, dispatcher
`Backend/src/function/expression.cpp`), the regions (`region/`), and `vale.cpp`. Derive placement from the
onion wrap; resolve member name→index from struct layout and the vtable index-in-edge at emit; implement
`Deref`/`*Lookup`/unified `Mutate` in codegen (the hammer's old lowering); retire the dormant
`instructions.h` nodes. Scope (raw counts under `Backend/src/`): `->location` ×41, `->ownership` ×86.
Resolve the **13 `// VCOORD` sites** flagged as backwards under the new model (full list: `translatetype.cpp:17`;
`vale.cpp:353,928,1043`; `metal/ast.h:189`; `metal/metalcache.h:92`; `function/expressions/localload.cpp:22`;
`function/expressions/externs.cpp:298`; `function/expression.cpp:604,678`; `region/common/common.cpp:309`;
`region/rcimm/rcimm.cpp:1068,1101`).

**TestVM (`src/testvm/`) — ported in a separate session (exp-1), not this branch.** Repoint the entry points from `ProgramH` to `HinputsI` — `vivem.rs`
`execute_with_primitive_args` (`:56`), `execute_with_heap` (`:67`), `inner_execute` (`:112`, finds
`main` via the program's export map). The placement-derivation target: `heap.rs` `add(interner,
ownership, location, kind)` (`:160`) and `add_allocation_for_return(ownership, location, kind)` (`:66`)
take `location` as a parameter today — change them to **compute** Inline/Yonder from the onion
`(ownership, kind)` when constructing the `ReferenceV`, instead of receiving `LocationH`. The VM is
threaded with final_ast types across `expression_vivem.rs` (~91 refs), `vivem_externs.rs` (~41),
`values.rs` (~31), `heap.rs`, `function_vivem.rs`, `call.rs` — all move to onion `HinputsI` types.

**Owned-bare-struct placement (the one deferred decision).** Because an owned value is now a bare kind
(zero wraps), the backend must decide how a bare owned *struct* is placed (by-value vs. heap) from the
kind alone rather than from a stored `Yonder`. Pin this rule when you reach step 4 — it's the point
where "no Location field" turns into a concrete codegen choice (historically owned citizens were
`Yonder`; primitives `Inline`).

**Gate:** now run the **full** suite — `src/integration_tests/`, `src/end_to_end_tests/`, and TestVM
(`src/testvm/test/vivem_tests.rs`). Re-link the downstream modules in `src/lib.rs`.

---

## Cross-cutting notes

- **The one genuinely new semantic thing** across the whole plan is the **onion→placement derivation**
  in step 4. Everything else is either deletion (defer timing, box lowering, the two-sort split,
  Coord/Ownership/Location fields) or mechanical renaming (kind/expr node translation, name mangling).
- **Keep locals and members named end-to-end.** No numbering, no member indices in the frontend. If
  codegen needs indices, resolve them in the backend from struct layout (step 4).

## Verification strategy

- **Step 1:** `cargo test --manifest-path Cargo.toml --lib --no-fail-fast` (typing suite) green;
  census panic sites per the handoff PICK-UP command. Behavior parity: deferred drops must fire at the
  same points (statement end / before `Return`, LIFO, discarded past `Never`) — compare against
  `drop_since`'s existing rules.
- **Step 2:** instantiator matcher tests (`collect_only_inode!` over `get_monouts()`) pin the onion IR
  shape on representative programs. Downstream held.
- **Step 3:** frontend compiles; `simplifying/` + `final_ast/` removed; no dangling `ProgramH` refs.
- **Step 4:** full suite green — TestVM interpretation and (where wired) the C++ backend — with
  placement derived from the onion.

## Guardrails

- **Never commit** without the architect's literal "fire commit" / "fire commit temporary". Steps 1–3
  land as `TEMP CHECKPOINT` commits; the green-at-commit invariant is suspended until step 4.
- **`src/typing/` is human-edited** — get architect approval before step 1's typing edits.
- No `#[ignore]` additions without approval. Surface before reverting landed work.
- Pipe all cargo output to a single fixed `./tmp/` file per session; never chain heavy commands with
  `| tail`/`| grep`/`| head`. Build via `--manifest-path Cargo.toml`, never `cd … && cargo`.

## Ordering & dependency summary

```
Step 1 (typing: Defer→tree, delete DeferTE)   — runs/tests in isolation; only linked suite
   ↓ (removes a node kind + Vec<deferred> threading from step 2)
Step 2 (instantiator: onion-ify HinputsI)      — instantiator tests only; downstream held
   ↓ (HinputsI is now backend-ready onion IR)
Step 3 (delete simplifying/ + final_ast/)      — frontend compiles; downstream held
   ↓ (nothing produces ProgramH anymore)
Step 4 (backend C++ + FFI bridge + TestVM)     — full suite runs; placement derived from onion
```
