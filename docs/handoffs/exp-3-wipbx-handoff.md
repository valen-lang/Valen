# exp-3-wipbx — handoff

This branch pushes the onion type model all the way to the backend: the instantiator emits an
onion-typed `HinputsI`, the simplifying (hammer) pass and `ProgramH`/`final_ast` are deleted, and the
C++ backend is rewired to consume `HinputsI` directly. Full plan: `docs/plans/complete-backend-plan.md`.

## State (regenerate, don't trust stale)

- Branch tip: `git log --oneline -1`. The whole backend reshape below is **uncommitted** —
  `git status --short`; needs a literal "fire commit".
- Linked modules: grep `pub mod` in `src/lib.rs`. `final_ast`/`simplifying` are **deleted** (not
  commented); `backend_ffi`, `pass_manager`, `clang` are linked; `testvm` is commented (it's ported in
  a separate session, exp-1, not this branch).
- The whole Rust crate compiles with the C++ build skipped:
  `CARGO_FEATURE_RUST_INTEROP=1 cargo check --manifest-path Cargo.toml --lib`. Plain `cargo check` is
  red at `build.rs` because the C++ **codegen** (`Backend/src/function/`, `region/`, `vale.cpp`) isn't
  onion-reshaped yet — the metal IR headers and FFI are (see plan Step 4).
- Frontend suites green: `cargo test --manifest-path Cargo.toml --lib typing::` and `... --lib
  instantiating::` (counts from `grep "test result"`). To run these, add `CARGO_FEATURE_RUST_INTEROP=1`
  and temporarily comment `backend_ffi` in `src/lib.rs` + the two submodules in `src/pass_manager/mod.rs`
  (a full `cargo test` links the C++), then restore.
- Validate a reshaped C++ header in isolation: `clang++ -std=c++17 -fsyntax-only -I Backend/src -I
  Backend/src/metal -I "$(/opt/homebrew/opt/llvm@16/bin/llvm-config --includedir)" <stub.cpp>` where the
  stub `#include`s the header.

## Where the plan stands

Steps 1–3 are landed. Step 4 (backend thinks onion) is done and compiling on the Rust + C++-header side;
what remains is the C++ codegen reshape, a few deferred `metal_lowerer.rs` bits (edges/vtables,
interface super-lists, array *definitions*), and re-linking/running the downstream suites. See
`docs/plans/complete-backend-plan.md` Step 4.

## The load-bearing constraint

Blast `Coord`/`Ownership`/`Location` away wherever you touch it — ownership is which onion wrap surrounds
the kind, placement is **derived from the onion shape at codegen** (C++), never a carried field. The FFI,
`metal_cache.rs`, and `metal_lowerer.rs` are dumb 1:1 plumbing: no lowering logic lives there.

## Lessons Learned

- The typing pass is human-authored (`src/typing/.claude/CLAUDE.md`): get explicit architect approval —
  the literal "fire core edits" — before editing `src/typing/`. Exceptions AI may edit freely:
  `rust_interop`, `borrow_checker`, `macros`.
- The metal IR / FFI / `metal_cache.rs` / `metal_lowerer.rs` are faithful 1:1 with the instantiated IR
  and do **no** lowering. Placement, member-name→index, deref/load fusion, and vtable slots all belong
  downstream in C++ codegen. (Architect preference: don't force lowering into the plumbing.)
- Eliminate a vestigial IR node rather than carry it: a tuple is a struct so it lowers to `Construct` in
  typing; a `Reinterpret` is a post-substitution type-identity so the instantiator asserts and drops it;
  member lookup is one `MemberLookup` (the read/write `AddressMemberLookup` split was pre-onion). Check
  whether the info already exists upstream before adding a node.
- Instantiateds are not interned — write-once/read-once value-types; the interner is a bare arena
  wrapper. Any note about interning `HinputsI` types is superseded.
- `'s` stays on the instantiator types — load-bearing via source runes keying the bounds maps and
  `StrI<'s>` mangling; don't chase removing it.
- Rebasing onto `main` can surface *semantic* conflicts git merges cleanly but the compiler rejects.
  Always re-verify (build + suites) after a rebase, not just on textual conflicts.
- Verify instantiator IR shape with `collect_only_inode!` matchers over `get_monouts()`, not golden
  dumps (`src/instantiating/collector.rs`). The `fn test` harness omits builtins; array/`v.builtins.*`
  fixtures need `fn test_with_array_builtins` in `instantiated_tests.rs`.
- Instantiator semantics that reading code alone misleads on: a primitive value-read peels its ref wrap
  via `CopyPrim` (not `Deref`); an owned construct is a bare kind with zero wraps; a virtual call site is
  a plain `FunctionCall` to the abstract fn while the `InterfaceFunctionCall` lives once in the generated
  dispatcher.
