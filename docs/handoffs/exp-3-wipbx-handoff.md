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
allocate/members, extern/export roundtrips, string read/len, lambdas, upcast — plus the `Mutate`
expression handler (the `set x = …` case, unimplemented in `translateExpressionInner`,
`Backend/src/function/expression.cpp`). A residual placement/ownership cluster is shrinking but not gone
(`grep -rc '\->location\|\->ownership' Backend/src`; the `VCOORD` sites in `Backend/src`); resolve each
onto `Sharedness` (see constraint). See `docs/plans/complete-backend-plan.md` Step 4.

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
- **Placement is `Sharedness`, and inline aggregates are real.** `SINGLE` structs/arrays are inline/by-value
  (member access via `extractvalue`, no control block); inline structs/SSAs are wanted, so never delete a
  `location == INLINE` branch as "dead code." For placement sites, ask what to *remove* — most `location`
  branches are dead/redundant/defensive and collapse — but keep the genuine `Sharedness` branch.
- `rcimm.cpp` is the shared/heap-and-`Str`-only region; every value type (primitives, single-owner
  aggregates) lives in the `mut` region (`unsafe.cpp`). Primitives route there via their `mutRegionId`
  singletons (`Backend/src/metal/metalcache.h`) and `getRegion(Kind*)` peels onion wraps to the inner
  kind's region. Don't add primitive/single handling to `rcimm.cpp`.
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
