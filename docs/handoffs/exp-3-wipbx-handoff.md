# exp-3-wipbx — handoff

This branch pushes the onion type model all the way to the backend: the instantiator emits an
onion-typed `HinputsI`, the simplifying (hammer) pass and `ProgramH`/`final_ast` are deleted, and the
C++ backend is rewired to consume `HinputsI` directly. Full plan: `docs/plans/complete-backend-plan.md`.

## State (regenerate, don't trust stale)

- Branch tip: `git log --oneline -1`; uncommitted work: `git status --short`. The backend reshape is
  landed on `main` — the `exp-*-wipbx` branches ratchet `main` via `git fetch . <branch>:main`.
- Linked modules: grep `pub mod` in `src/lib.rs`. `final_ast`/`simplifying` are **deleted** (not
  commented); `backend_ffi`, `pass_manager`, `clang`, and `testvm` are all linked.
- **The gate is `cargo nextest run --manifest-path Cargo.toml`, plus the same with
  `VALE_TEST_BACKEND=wasi`** — both green, and neither compiles the C++ backend, so no module-commenting
  is needed. `CARGO_FEATURE_RUST_INTEROP=1 cargo check --lib` checks only the non-test lib and misses
  `cfg(test)` + `testvm` code; it is not the gate.
- The **C++ backend build is separately red** on the placement/`location` cluster (below). Build it
  directly with `make` in the cmake-rs build dir cargo configured
  (`target/debug/build/frontend_rust-*/out/build`); `cargo nextest` does not.

## Where the plan stands

Steps 1–3 are landed; Step 4 (backend thinks onion) is nearly done. The Rust + C++ headers/FFI are
onion-shaped, and the C++ codegen readers are mostly reshaped (onion-wrap ownership, `peel_all_references`
+ `isValueType` in `metal/types.{h,cpp}`, typing-owned vtable slots, array/If/interface-call handlers).
What remains: the **placement/`location` cluster** — `Location` is deleted, but `mallocKnownSize` and a
few sites still branch on the removed field; resolve each onto `Sharedness` (see constraint). Then re-link
and run the downstream (integration/end_to_end/testvm) suites. See `docs/plans/complete-backend-plan.md` Step 4.

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
- `rcimm.cpp` only ever sees shared/heap things: it asserts `ShareRef` and throws on primitives. Don't add
  single/owned or primitive handling there — that belongs in the single region (`unsafe.cpp`).
- **The gate does not compile the C++ backend.** `cargo nextest run` (native + wasi) is green while the
  C++ build is red; and `CARGO_FEATURE_RUST_INTEROP=1 cargo check --lib` misses `cfg(test)` + `testvm`
  code (it green-lit stale `expr_return` / array-lookup test errors the nextest gate caught). Run the real
  gate, not the narrow check.
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
