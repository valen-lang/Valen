# Typing Pass Migration — TL Handoff

## Required Reading

Read these before doing anything else, in this order:

1. **This file** — read top to bottom before starting work
2. **`docs/architecture/typing-pass-design-v3.md`** — architecture and design decisions for the typing pass migration
3. **`FrontendRust/docs/architecture/typing-pass-arenas.md`** — current arena shape and lifetime model
4. **`FrontendRust/zen/migration_principles.md`** — migration rules (DCCR, RCSBASC, etc.)
5. **`FrontendRust/zen/testing.md`** — test conventions (`find_func_named`, `expect_1`/`expect_2`, etc.)
6. **`docs/skills/migration-drive.md`** — the junior's instructions (know what they're following)

For historical slab-by-slab progress (Slabs 0–14b), see `docs/historical/slab-chronicle.md`. Per-slab handoff docs with translation tables and gotchas are in `FrontendRust/docs/migration/handoff-slab-*.md`. The historical design docs (`docs/historical/typing-pass-design-v1.md`, `docs/historical/typing-pass-design-v2.md`, `docs/historical/typing-pass-migration-setup.md`) are obsolete — they each carry "DO NOT FOLLOW" banners.

---

## Guiding Principle

**1:1 Scala parity is the highest priority.** Every design decision defers to matching Scala's structure, naming, and behavior. No novel logic, no reorganization, no Rust-idiomatic "improvements" beyond what Rust strictly requires to compile. Where Rust can't directly mirror Scala, prefer `panic!`/`assert!` placeholders over inventing alternatives. When in doubt, port Scala verbatim.

**This applies to bodies as much as to types.** When porting a Scala function body, translate every line literally — including assertions, size checks, intermediate bindings, redundant-looking branches, and code paths that appear dead. Do **not** simplify, collapse, flatten, inline, or "optimize away" anything on the way over, even if you can prove the simplification is semantically equivalent. The Scala source is the spec; your job is transcription, not refactoring. If the Scala writes `if (results.size > 1) vfail()` over a value whose size can never exceed 1, the Rust writes the same check. If the Scala binds an intermediate variable that's used once, the Rust binds the same intermediate. If the Scala has a `match` arm that pattern-matches a case the caller "couldn't reach," the Rust has the same arm. Parity-preserving translation is a narrow target — keep the diff against Scala visually obvious so reviewers can verify line-for-line.

**TLs and reviewers: enforce this on hand-offs.** When suggesting a body translation to a junior, never propose a simplification of the Scala — quote the Scala verbatim and tell them to mirror it. When reviewing a junior's diff, flag any place where the Rust shape diverges from the Scala shape, even when the divergence "obviously works." The migration's value comes from the audit trail; a clever translation that nobody can verify against the Scala is worse than a verbose one that anyone can.

---

## Where We Are

The scaffolding phase is complete. Slabs 0–14b built out every type definition, all ~210 method signatures with proper lifetimes, and all placeholder types (only `IRegionNameT` retains `_Phantom` — 0 Scala implementors found). `cargo check --lib` is clean (0 errors, 22 warnings — all minor lifetime elision).

**Current work: body migration (Slab 15+).** 141 panic-stubbed method bodies across 17 files, plus 88 stale stubs from earlier slabs. All are functionally equivalent — they need real Scala-parity implementations. Work is test-driven: pick a test, run it, implement the body it hits, repeat.

Test infrastructure: 14 test files in `src/typing/test/` with 173 test bodies (compiler_tests 91, compiler_solver_tests 27, compiler_virtual_tests 18, compiler_mutate_tests 12, compiler_lambda_tests 10, compiler_ownership_tests 11, compiler_project_tests 3, compiler_generics_tests 1). All currently panic at the first method body they exercise.

---

## Known Residual Items

- **141 panic-stubbed method bodies** across 17 files (top: expression_compiler.rs 21, templata_compiler.rs 20, infer_compiler.rs 15). Plus 88 stale stubs labeled "Slab 10" in compiler_outputs.rs (52) and templata_compiler.rs (36).
- **dispatch_function_body_macro** and friends not wired.
- **LocationInFunctionEnvironmentT.path: Vec<i32>** in `ast/ast.rs` violates AASSNCMCX. Future cleanup turns into `&'t [i32]`.
- **IRegionNameT** retains `_Phantom` — 0 Scala implementors found, deferred.
- **22 cargo warnings** — all minor lifetime elision suggestions.

---

## Good Partial Implementing

When replacing a `panic!` stub with real logic, write just the shallow structure of that scope — straight-line variable bindings, function calls, match expressions with all arms — but put `panic!` inside every new branch body, loop body, closure/lambda body, and match arm. Then only fill in the specific arms/branches the driving test actually hits. This applies recursively: when a test hits one of those inner panics, replace *that* panic with its own skeleton-with-panics, fill in only what the test needs, and so on. Each iteration expands one panic into a new layer of structure. **Aggressively panic for anything that might not be executed by current tests.** This minimizes each batch's diff and ensures untested paths crash loudly rather than silently returning wrong results.

---

## Run Solutions By The Architect First

**Before implementing any structural fix or design change, propose the solution to the architect and wait for approval.** This includes lifetime-parameter additions, signature changes that propagate across many files, new abstractions, and any choice between alternatives. Don't start editing — even for fixes that look mechanical — until the architect has signed off on the approach. The cost of a quick check is small; the cost of unwinding a wrong-direction change across ~20 call sites is large.

---

## Don't Simplify Scala On The Way Over

Restating the guiding principle as an operating rule because TLs slip on it: **when handing a body off to a junior, quote the Scala verbatim and instruct them to translate every line.** Do not flatten redundant checks, do not collapse impossible branches, do not inline single-use bindings, do not reason "well in Rust we can just…". If you find yourself writing "the Rust method already returns `Option`, so it's a direct return" or "we can skip this size check because it can't happen" in a hand-off, stop — that's a parity violation in the making. The whole migration's auditability rests on the diff being a literal line-for-line port. A junior who follows a verbatim Scala translation produces a reviewable patch; a junior who follows a TL's "smarter" translation produces a patch nobody can compare against the source.

---

## Cleaning Up After The Slice Pipeline

The slice pipeline (`slice-start` → `slice-rustify` → `slice-placehold` → reconcile) is the right tool for bringing a file with raw `/* scala */` comments up to SCPX parity. It's also incomplete in known ways. After running it on a non-trivial typing-pass file, plan on a manual cleanup pass before `cargo check` will be clean. These are the lessons from running it on `env/environment.rs`.

### `slice-placehold` doesn't infer struct context

For each `// mig: fn foo` it emits `pub fn foo<'s, 't>(&self, …) { panic!() }` at **module scope**. It does not look at what Scala class the `def` is inside, and it does not wrap the stub in an `impl SomeT<'s, 't>` block.

Two failure modes follow:

1. **Cross-variant name collisions.** A Scala trait method overridden by N case classes (e.g. `lookupWithNameInner` overridden by `PackageEnvironmentT`, `CitizenEnvironmentT`, `ExportEnvironmentT`, `ExternEnvironmentT`, `GeneralEnvironmentT`) becomes N module-level `pub fn lookup_with_name_inner` stubs that all collide (`E0428`).
2. **Invalid `&self`.** `&self` outside an `impl`/`trait` is not valid Rust (`E0061`-style) and the per-fn `<'s, 't>` generics on a free fn don't have anywhere to come from in a real method dispatch.

**Cleanup**: wrap each stub in the right `impl<'s, 't> SomeT<'s, 't> where 's: 't { … }` block, **drop the per-fn `<'s, 't>` generics** (the impl provides them), and indent the existing Scala `/* … */` to live inside the impl alongside the Rust fn (per the SCPX adjacency rule, §"Preserve The `/* scala */` Audit Trail" in the design doc). One impl block per stub matches the rest of the file's pattern; consolidating multiple methods into one impl is allowed but not required.

### `slice-placehold` emits bogus `eq`/`hash_code` stubs

Scala's `override def equals/hashCode` is realized in Rust by `impl PartialEq`/`impl Hash`, not by methods named `eq`/`hash_code`. The placehold agent will sometimes emit `pub fn hash_code(&self) -> i32 { panic!() }` stubs that don't correspond to any real Rust dispatch.

**Cleanup**: replace the bogus `pub fn` body with a one-line marker comment:
```rust
// mig: fn hash_code
// (Realized by `impl Hash for FooT` below.)
/*
  override def hashCode(): Int = …
*/
```
Keep the `// mig:` marker (preserves the audit trail) and the Scala `/* … */` block (SCPX). Just don't pretend there's a Rust method.

### NRDX blocks multi-fn diffs — go one fn at a time

The `NoRenamedDefinitions-NRDX` shield's heuristic flags consecutive context-swaps as renames. An Edit that wraps **two adjacent** `// mig: fn foo` and `// mig: fn bar` stubs in their impl blocks in one shot will trip the shield with "fn foo renamed to bar" — even though no rename is happening.

**Cleanup**: do one stub per Edit. Slow but predictable.

### Verify with cargo + SCPX after

After cleanup, two checks:

1. `cargo check --manifest-path FrontendRust/Cargo.toml --lib > tmp/<session>.txt 2>&1` — must be 0 errors. Pre-existing warnings are fine; new ones aren't.
2. `cargo run --manifest-path Luz/shields/ScalaCommentParity-SCPX/Cargo.toml --release -- --check-all` — must report `All 230 files OK`. SCPX is the canary that the audit trail is intact through the wrap.

### Don't dispatch the orchestrator on a hand-edited file

The slice-orchestrator runs all six steps. If the file already has hand-written Rust impls (like `env/environment.rs` did before this session), reconcile-mark only catches the matching-name old definitions and leaves the rest in place. The colliding fresh placehold stubs then need the manual `impl`-wrap cleanup above. Plan for it; don't expect the orchestrator alone to leave a compile-clean file when the input was mid-state.

---

## TemplatasStoreT Is `&'t` In All Env Structs

As of Slab 15b, all environment structs (`CitizenEnvironmentT`, `FunctionEnvironmentT`, `BuildingFunctionEnvironmentWithClosuredsT`, `BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT`, `NodeEnvironmentT`, `NodeEnvironmentBoxT`, `GeneralEnvironmentT`, `PackageEnvironmentT`) hold `&'t TemplatasStoreT<'s, 't>` instead of owned `TemplatasStoreT<'s, 't>`. This matches Scala's GC reference semantics — Scala environments just hold a reference to the `TemplatasStore`, they don't own it.

**When building a new env struct**, arena-allocate the store first:
```rust
let store = self.typing_interner.alloc(new_templatas_store);  // &'t TemplatasStoreT
```

**When copying an env's templatas into a child env**, just copy the `&'t` reference — no clone needed:
```rust
templatas: parent_env.templatas,  // copies the &'t pointer
```

`TemplatasStoreT::add_entries(&self, ...)` returns a new owned `TemplatasStoreT`. Arena-allocate the result before storing:
```rust
let new_store = self.typing_interner.alloc(
    near_env.templatas.add_entries(self.typing_interner, self.scout_arena, entries)
);
```

`TemplatasStoreBuilder::build_in` already returns `&'t TemplatasStoreT` (it arena-allocates internally).

---

## Proactively Add Inherited Dispatch Methods

The slice pipeline generates stubs only for methods defined **directly on** a Scala trait's body. When a child trait extends a parent (e.g. `IInDenizenEnvironmentT extends IEnvironmentT`), the child enum needs its own delegation methods that widen `self` to the parent enum and call through. The pipeline doesn't generate these — they're invisible inheritance in Scala but explicit wiring in Rust.

**Don't wait for serial JR escalations.** When you see a Scala child trait extending a parent, proactively add all inherited methods on the child enum. Each follows the same pattern:

```rust
impl<'s, 't> IInDenizenEnvironmentT<'s, 't> where 's: 't {
  pub fn method_name(&self, ...) -> ReturnType {
    let as_env: IEnvironmentT<'s, 't> = (*self).into();
    as_env.method_name(...)
  }
  /* Guardian: disable-all */
}
```

Similarly, factory methods on name-template traits (`make_function_name`, `make_struct_name`, `make_interface_name`, `make_impl_name`, `make_citizen_name`) are abstract in Scala and need per-variant match dispatch on the Rust enum. These follow the interning pattern:

```rust
IFunctionTemplateNameT::FunctionTemplate(tmpl) => {
  interner.intern_name(INameValT::Function(FunctionNameT {
    template: tmpl,
    template_args,
    parameters,
  }))
}
```

All of these were swept in Slab 15b. If new trait hierarchies are scaffolded, do the same sweep before handing off body migration to a junior.

---

## NNDX Escalation Pattern

**When a junior is blocked by NNDX on a legitimate Scala counterpart**, the issue is incomplete scaffolding, not a bad shield. The TL adds the missing definition directly — don't temp-disable NNDX. NNDX exists to route definition-creation to the right authority level; the junior escalating is the system working correctly.

Example: Scala's `def globalEnv` on `IEnvironmentT` trait (line 60) becomes a `fn global_env()` match-dispatch method on the Rust enum (per SSTREX). If the slice pipeline didn't generate it, the junior hits NNDX when they try to add it. Correct response: junior stops and escalates; TL adds the accessor.

**How to slice in a missing Rust definition.** The Scala comment blocks are the audit trail — every Rust definition must sit directly above its Scala counterpart. When adding a missing definition:

1. Find the Scala `def` inside its `/* ... */` comment block.
2. Split the comment block: close `*/` just before the target `def`, insert the Rust `impl` block, then reopen `/*` to resume the remaining Scala.
3. The Scala `def` line must end up **inside** the Rust `impl` block, as an inline `/* ... */` comment immediately after the Rust `fn` body — not after the `}` that closes the `impl`. This keeps the Scala counterpart visually adjacent to its Rust translation.

```rust
impl<'s, 't> IEnvironmentT<'s, 't> where 's: 't {
  pub fn global_env(&self) -> &'t GlobalEnvironmentT<'s, 't> {
    match self {
      IEnvironmentT::Package(e) => e.global_env,
      // ...
    }
  }
  /*
    def globalEnv: GlobalEnvironment
  */
}
```

Not like this (Scala comment stranded outside the impl):
```rust
  // WRONG — comment is after the impl's closing brace
}
/*
  def globalEnv: GlobalEnvironment
*/
```

---

## Guardian Annotations For New Definitions Without Scala Counterparts

When adding a Rust function that has no direct Scala counterpart (e.g. delegation wiring for Scala trait inheritance, `From`/`TryFrom` impls, interning Val/Query structs), Guardian's NNDX shield will fire. The TL is ordained and can push through, but must annotate the new code so Guardian doesn't fire on future edits either:

- **Pure wiring (no logic)** — delegation methods, `From`/`TryFrom` match-dispatches, trivial accessors that just forward to another method. Add `/* Guardian: disable-all */` after the function/impl block. These are mechanical and don't need shield scrutiny.
- **Contains logic** — anything with conditionals, assertions, non-trivial transformations. Add an empty `/* */` after the function body (satisfies SCPX's requirement for a Scala comment block, signaling "this definition was reviewed and has no Scala counterpart").

This is TL/architect-level knowledge. The junior (migration-drive.md) should never add these annotations — they escalate to the TL instead, which is the system working correctly.

---

## Preserve The `/* scala */` Audit Trail

The typing/ skeleton has a `/* ... */` block with the Scala source directly below every Rust definition. The `.claude/hooks/check-scala-comments` pre-commit hook does exact-match comparison and rejects any edit inside those blocks. Rules:

- Replace the empty Rust stub in-place with the real definition. Keep the Scala `/* ... */` block below unchanged.
- Never move a Rust definition away from its Scala block.
- A Rust definition grown to need helper structs (e.g. an IdValT companion for an IdT) gets its companion block **adjacent to** the main definition, with a `// (no scala counterpart — …)` note.
- **After any change that touches Scala comment blocks** (splitting, moving, adding new impl blocks between them), run the SCPX shield to verify structural integrity:
  ```
  cargo run --manifest-path Luz/shields/ScalaCommentParity-SCPX/Cargo.toml --release -- --check-all
  ```

---

## Test Promotion Workflow

All 173 tests in `src/typing/test/` have `#[ignore]` except the currently-active test. This means `cargo nextest run` only runs the active test(s), so JR sees regressions immediately — a failure means something they changed broke, not a pre-existing panic.

**When JR reports a test passing:**
1. Verify the pass yourself: `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml > ./tmp/slab15-tests.txt 2>&1`
2. Remove `#[ignore]` from the next simplest test in `compiler_tests.rs`
3. Hand off to JR again

**Choosing the next test:** Start with `compiler_tests.rs` (91 tests, simplest programs). Within that file, go roughly top-to-bottom — the tests are ordered from simple to complex. Once `compiler_tests.rs` is done, move to `compiler_solver_tests.rs`, then `compiler_virtual_tests.rs`, etc.

**Keep passed tests un-ignored.** They serve as regression guards. If a passed test starts failing, that's a real regression JR needs to fix before continuing.

---

## How To Continue

1. **Read the design doc** (`docs/architecture/typing-pass-design-v3.md`) top to bottom. Also read `FrontendRust/docs/architecture/typing-pass-arenas.md` for the current arena shape.
2. **Body migration is test-driven.** The active test (currently `simple_program_returning_an_int_explicit`) drives which panic stubs get implemented. See "Test Promotion Workflow" above.
3. **Don't commit.** The human handles all commits and tags. Hand back uncommitted working trees at batch boundaries.
4. **Group related bodies** into coherent batches for review. Each batch gets a handoff doc listing target methods, driving test(s), and Scala translation gotchas.

---

## Suggested Process For The Incoming TL

- Spawn a junior for each batch. Write a handoff listing the target methods, the test(s) that exercise them, and any Scala translation gotchas for those specific bodies.
- When a design diverges from the design doc, record the divergence in `FrontendRust/docs/reasoning/<topic>.md`. Then update the design doc.
- **Never commit.** Juniors and TLs finish a batch, run `cargo check --lib` clean, self-review their diff, then hand back to the human with uncommitted changes.
- **Expect and invite push-back.** Handoffs are proposals, not spec.
- **Scope discipline.** If edits land in `tl-handoff.md`, the design doc, or reasoning docs, announce in the hand-back summary, not folded silently into the diff. TLs revert off-scope edits before review.

---

Questions? The reasoning docs + the per-slab handoffs have additional context. When in doubt, prefer Scala parity over Rust-idiomatic optimization — that's the guiding principle.
