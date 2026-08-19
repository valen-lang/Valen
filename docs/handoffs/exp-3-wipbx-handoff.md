# exp-3-wipbx — handoff

This branch pushes the onion type model all the way to the backend: the instantiator emits an
**onion-typed `HinputsI`**, then the simplifying (hammer) pass and `ProgramH`/`final_ast` get deleted
and the C++ backend + TestVM are rewired to consume `HinputsI` directly. The full four-step plan is
`docs/plans/complete-backend-plan.md`.

## State (regenerate, don't trust stale)

- Branch tip: `git log --oneline -1`; uncommitted work: `git status --short`.
- **`instantiating` links and compiles now** (`pub mod instantiating` in `src/lib.rs`). Still commented
  out downstream: `backend_ffi`, `final_ast`, `simplifying`, `testvm` (grep `pub mod` in `src/lib.rs`
  for the live set).
- Suites are green: `cargo test --manifest-path ./Cargo.toml --lib typing::test::` and
  `... --lib instantiating::` (read counts from `grep "test result"`).
- **The full `cargo nextest run --manifest-path Cargo.toml` gate is red, on the `valec` bin only** —
  `src/bin/valec/*` imports `frontend_rust::backend_ffi` and `pass_manager::pass_manager`, both still
  commented out of `src/lib.rs`, and the bin has no `required-features` gate (unlike `valec-rs`). This
  predates the onion work; it clears at step 4 or by gating the bin. Do not chase it as a regression.

## Where the plan stands

Steps 1 and 2 are complete. **Step 1** — typing owns deferred temp-drops via a linear
`PendingTempDrops` obligation; `DeferTE` is retired. **Step 2** — the instantiator emits onion
`HinputsI`: `CoordI`/`OwnershipI`/`LocationI` gone, `KindIT` carries the four ref-wrap variants, the
two-sort `ExpressionIE` split collapsed, and interning is dropped entirely (see Lessons). Verified by
matcher-based tests in `instantiated_tests.rs` (`fn test` per program → `get_monouts()` →
`collect_only_inode!` over the onion IR), walking the completed `visit_expression_ie` in
`src/instantiating/collector.rs`.

**Step 3** deletes `src/simplifying/` + `src/final_ast/` and repoints every `ProgramH`/`get_hamuts()`
reference to `HinputsI`/`get_monouts()`. **Step 4** rewires the C++ backend + TestVM and derives
placement (Inline/Yonder) from the onion shape at codegen — the one genuinely new semantic.

## The load-bearing constraint

Blast `Coord`/`Ownership`/`Location` away wherever you touch it — never make new code interoperate with
`CoordH`/`OwnershipH`/`LocationH`/`ProgramH`. Ownership is which onion wrap surrounds the kind (or none,
for an owned bare kind); placement is **derived from the onion shape at codegen** (step 4), not a
carried field.

## Lessons Learned

- The typing pass is human-authored (`src/typing/.claude/CLAUDE.md`): get explicit architect approval
  before editing `src/typing/`. Exceptions AI may edit: `rust_interop`, `borrow_checker`, `macros`.
- **Instantiateds are not interned.** They are write-once/read-once, so the I-types are plain structural
  value-types — no `MustIntern`, no `*ValI` transients, no interner dedup maps; the interner is a bare
  arena wrapper. If a plan or comment mentions interning `HinputsI` types, it is superseded.
- **`'s` stays on the instantiator types.** It is load-bearing via source runes (`IRuneS<'s>` key the
  `rune_to_*_bound` maps) and `StrI<'s>` mangling names; removing it would mean re-keying the bounds
  machinery. Don't chase it.
- Rebasing onto `main` can surface *semantic* conflicts git merges cleanly but the compiler rejects —
  e.g. incoming borrow-checker code listing `ExpressionTE::Defer` after this branch removed it. Always
  re-verify (build + suites) after a rebase, not just on textual conflicts.
- Do not call the `valec`-bin nextest failure a regression; it is the pre-onion break above.
- Verify instantiator IR shape with `collect_only_inode!` matchers over `get_monouts()`, not golden
  text dumps — same as the other passes. The macros and the `NodeRefI` walker live in
  `src/instantiating/collector.rs`.
- The instantiator `fn test` harness omits builtins; array/`v.builtins.*` fixtures need
  `fn test_with_array_builtins` (prepends `builtin_source_for_arrays`) in `instantiated_tests.rs`.
- Instantiator semantics that reading code alone misleads on: a primitive value-read peels its ref
  wrap via `CopyPrim` (not `Deref`); an owned construct is a bare kind with zero wraps; a virtual
  call site is a plain `FunctionCall` to the abstract fn while the `InterfaceFunctionCall` lives once
  in the generated dispatcher.
