# exp-3-wipbx — handoff

This branch pushes the onion type model all the way to the backend: the instantiator emits an
onion-typed `HinputsI`, the simplifying (hammer) pass and `ProgramH`/`final_ast` are deleted, and the
C++ backend is rewired to consume `HinputsI` directly. Full plan: `docs/plans/complete-backend-plan.md`.

## State (regenerate, don't trust stale)

- Branch tip: `git log --oneline -1`; uncommitted work: `git status --short`. `main` and the
  `exp-*-wipbx` working branch ratchet together via `git fetch . <branch>:main`.
- Linked modules: grep `pub mod` in `src/lib.rs`. `final_ast`/`simplifying` are **deleted** (not
  commented); `backend_ffi`, `pass_manager`, `clang`, `testvm`, `end_to_end_tests`, and
  `integration_tests` are all linked.
- **The gate is `cargo nextest run --manifest-path Cargo.toml`, plus the same with
  `VALE_TEST_BACKEND=wasi`** — both green. It **compiles, links, and runs the C++ backend**: the linked
  e2e/integration suites drive it via `pass_manager::build` → clang, so a green gate means the backend
  builds and its live programs run end-to-end.
- Backend features not yet migrated to the onion IR keep their e2e/integration tests `#[ignore]`d with
  `// ZCOORD: re-enable with onion`. **That marker is the live map of remaining backend work** — `grep
  -rn 'ZCOORD' src/` lists every parked test; the un-ignored e2e tests (e.g. `misc::unstackifyret`) pass.

## Where the plan stands

Steps 1–3 are landed; Step 4 (backend thinks onion) builds and runs. The C++ backend compiles, links,
and runs simple programs end-to-end. Remaining is the feature set behind the `// ZCOORD` tests — struct
allocate/members, extern/export roundtrips, string read/len, lambdas, upcast. A residual
placement/ownership cluster is shrinking but not gone (`grep -rc '\->location\|\->ownership' Backend/src`;
the `VCOORD` sites in `Backend/src`); resolve each onto `Sharedness` (see constraint). See
`docs/plans/complete-backend-plan.md` Step 4.

**The mut/single struct path compiles through codegen.** Lookups yield a borrow *of storage* — a pointer
(the local's alloca, a member GEP); `Deref`/`CopyPrim` read through it via `IRegion::load`, `Mutate` stores
through it. A `StructKind`/SSA translates to its inner value, a reference wrap to a pointer
(`Unsafe::translateType`); construction assembles the value with `insertvalue` (no wrapper), destructure
loads the whole struct and `extractvalue`s each field, and `&x` is `LetAndLend` (store into a local, lend
its pointer). See `Backend/backend-design.md` S5–S7 for the design (wrapper structs are RAM-only).
`mutswaplocals` and the mut/single struct path (`structmutparamexport`) compile end-to-end.

**Next: the generated C-ABI headers for structs.** `structmutparamexport` now fails at clang, not codegen —
the export/extern C headers name the struct two ways (`vtest_Spaceship` in the signature vs
`vtest_SpaceshipRef` typedef) and emit it as an opaque `{ uint64_t }` instead of its real fields (the
`getExportName`/C-header path, `Backend/src/vale.cpp`).

## ValueKind — a type-level "wrap-free kind" (orthogonal to the onion migration)

`ValueKind : Kind` (`Backend/src/metal/types.h`) is a compile-time witness that a kind carries no onion
ref wrap: the 11 value kinds derive from it, the 4 wraps derive from `Kind` directly, `peel_all_references`
returns `ValueKind*`, and a `= delete`d `peel_all_references(ValueKind*)` overload makes re-peeling a
compile error. Every audited value-kind inspector now demands the witness: the control-block cluster,
the naming/export/weakability group (`GlobalState::getKindName`/`getKindWeakability`, region
`getExportName`/`getExternalType`/`getKindWeakability`, Package `getKindExportName`/`getKindHumanName`/
`getKindExternName`), the `function.cpp` C-ABI trio (`translatesToCVoid`/`typeNeedsPointerParameter`/
`translateExternReturnType`), `translateWeakReference`, `fillWeakableControlBlock`, `intRangeLoopReverse`, and
the rcimm `valeKind` prototype cluster.
The dead never-read `Kind*` params the audit found are deleted (`getInterfaceMethodVirtualParamAnyType`,
`getWeakRefHeaderStruct`/`getWeakVoidRefStruct`, `getIsAliveFromWeakFatPtr`, the `reference`/`kindM` on the
`getConcreteControlBlockPtr`/interface-`getControlBlockPtr` variants, and the RSA-size helpers' `rsaRefMT`).
When a function peels the same reference more than once, hoist it into a single `<name>ValueType` local and
reuse it (the backend follows this; a repeat that reaches into a lambda is added to the lambda's capture).

**Signature rule:** a function that only inspects the concrete kind (peels immediately, or dispatches on
value subtypes with `assert(false)` on wraps) takes `ValueKind*`; one that dynamic_casts the param to a
wrap, asserts a wrap, or stores/forwards the raw ref (into a `Ref`/fat-ptr/`ControlBlockPtrLE`) keeps
`Kind*`. A wrap's `inner` stays `Kind*` — wraps nest (`BorrowRef<BorrowRef<…>>` is real). `getRegion(Kind*)`
is the deliberate exception: it peels internally rather than demanding `ValueKind*` (a flip to `ValueKind*`
was tried and reverted — it only pushed a redundant peel onto its ~400 callers for no real safety gain at
the region-dispatch boundary).

**Still deferred:** `asSubtype`/`regularDowncast`/`resilientDowncast` `targetKind` — live bodies are
`assert(false)` stubs; flip to `ValueKind*` when implemented.

## The load-bearing constraint

Blast `Coord`/`Ownership`/`Location` away wherever you touch it. Ownership is which onion wrap surrounds
the kind (bare / `BorrowRef` / `OwnRef` / `ShareRef` / `WeakRef`); **placement is `Sharedness` on the
citizen definition** (`Sharedness sharedness` on `StructDefinition`/`InterfaceDefinition`): `SINGLE` →
inline/by-value, `SHARED` → yonder/heap, looked up at codegen. Neither is a carried field on `Reference`.
The FFI, `metal_cache.rs`, and `metal_lowerer.rs` are dumb 1:1 plumbing: no lowering logic lives there —
except the vtable slot (`index_in_edge` on `InterfaceFunctionCallIE`), which the instantiator supplies by
reading typing's edge blueprint (`super_family_root_headers`), not a C++-codegen resolution.

## Lessons Learned

- The typing pass is human-authored (`src/typing/.claude/CLAUDE.md`): get explicit architect approval —
  the literal "fire core edits" — before editing `src/typing/`. AI may edit freely: `rust_interop`,
  `borrow_checker`, `macros`.
- The metal IR / FFI / `metal_cache.rs` / `metal_lowerer.rs` are faithful 1:1 with the instantiated IR
  and do **no** lowering. Placement (from `Sharedness`) and member-name→index are resolved downstream in
  C++ codegen; the vtable slot is the exception — supplied by the instantiator from typing's blueprint.
  (Architect preference: don't force lowering into the plumbing.)
- **Placement is `Sharedness`, and inline aggregates are real.** A `SINGLE` struct/array translates to its
  inner value (no wrapper/control block); a reference to it is a pointer. Never delete a `location == INLINE`
  branch as "dead code." For placement sites, ask what to *remove* — most `location` branches are
  dead/redundant/defensive and collapse — but keep the genuine `Sharedness` branch.
- `rcimm.cpp` is the shared/heap-and-`Str`-only region; every value type (primitives, single-owner
  aggregates) lives in the `mut` region (`unsafe.cpp`). Primitives route there via their `mutRegionId`
  singletons (`Backend/src/metal/metalcache.h`); `getRegion` resolves any kind to its region by its inner
  (peeled) kind. Don't add primitive/single handling to `rcimm.cpp`.
- **Backend local identity is the variable name, not the node pointer.** Metal `Local` carries a
  `VarNameM id` (the frontend declaration name, per-function-unique via its LID); `BlockState`
  (`Backend/src/function/function.h`) keys on it. The instantiator reallocates a fresh `LocalVariableI`
  per mention, so the node pointer is NOT stable — this restores the pre-onion `VariableId` contract.
  `VarNameM` is the metal lowering target of `IVarNameI` (a string wrapper for now).
- Eliminate a vestigial IR node rather than carry it: a tuple lowers to `Construct` in typing; a
  `Reinterpret` is asserted and dropped by the instantiator; member lookup is one `MemberLookup`. Check
  whether the info already exists upstream before adding a node.
- Instantiateds are not interned — write-once/read-once value-types; the interner is a bare arena wrapper.
- `'s` stays on the instantiator types — load-bearing via source runes keying the bounds maps and
  `StrI<'s>` mangling; don't chase removing it.
- Rebasing onto `main` can surface *semantic* conflicts git merges cleanly but the compiler rejects.
  Always re-verify (the nextest gate) after a rebase, not just on textual conflicts.
- Verify instantiator IR shape with `collect_only_inode!` matchers over `get_monouts()`, not golden
  dumps (`src/instantiating/collector.rs`). The `fn test` harness omits builtins; array/`v.builtins.*`
  fixtures need `fn test_with_array_builtins` in `instantiated_tests.rs`.
- Instantiator semantics that reading code alone misleads on: a primitive value-read peels its ref wrap
  via `CopyPrim` (not `Deref`); an owned construct is a bare kind with zero wraps; a virtual call site is
  a plain `FunctionCall` to the abstract fn while the `InterfaceFunctionCall` lives once in the generated
  dispatcher.
