# Debugging Branch

The `debugging` branch parks the 7-commit debugger arc (`f9b17956b` Backend DWARF emission + Frontend source-location plumbing → `6e7c3c06f` arc retraction), rebased onto the experimental-2 squash baseline (`71e91d6a2`) plus a parking commit (`34611c611`) that carries the post-rebase semantic-merge fixes and a `handoff.md` writeup.

## What it's waiting on

The squash baseline introduced an `assert(referenceM->ownership == Ownership::OWN)` in `Backend/src/region/common/primitives.h:27,33` (called from `RCImm::defineStruct` for every primitive struct member). This is part of the **sharedness arc** — the arc's intent is that primitives (Int / Bool / Float / Void) are always Own at the Backend boundary, but many Frontend lowering paths still produce them with Share/Borrow ownership. The squash baseline shipped with 120 tests tagged `#[ignore = "deferred at experimental-2 squash baseline"]` for this exact reason.

The debugger arc added 21 new end-to-end tests (`FrontendRust/src/end_to_end_tests/tests/debugger::*`) that exercise the same broken paths — bool/float locals, structs with primitive members, RSA/SSA locals. All 21 fail with the same `translatePrimitive` assertion. Per architect, they are **left un-ignored** rather than tagged with the squash deferral.

Otherwise the suite is green: 1096/1117 pass, 121 skipped, no regressions outside the documented 21.

## What needs to happen before this lands on `experimental`

The sharedness arc needs to close so primitives reach the Backend as Own. When it does:

1. The 21 debugger tests should pass without further changes — they don't depend on Share-ownership being supported, just on the Frontend producing valid Own-typed primitives.
2. The 120 squash-deferred `#[ignore]` tags can be lifted in a single sweep across `FrontendRust/src/end_to_end_tests/tests/*.rs`.
3. The `translatePrimitive` assertion stays — it's the correct invariant.

No work on the `debugging` branch itself is needed in the meantime. This is a **wait for sharedness**, not a TODO.

## Parked stash

The `debugging` checkout also carries one `git stash` entry (`fire-rebase WIP`) from the rebase session that holds:

- **Gate 1** at `FrontendRust/src/backend_ffi/metal_lowerer.rs:178` — panics if `RangeS::internal` (synthetic range) reaches the FFI; replaces a silent `None` fallback.
- **Gate 2** at `Backend/src/function/expression.cpp` — `assert(expr->sourceLocation != nullptr)` at the `translateExpressionInner` debug-loc setter and the Stackify M3 path.
- ~20 minting-site fixes across typing-pass + instantiator + macros that eliminate `RangeS::internal` so the gates stay green.
- The `unlet_local_without_dropping` signature refactor (range threaded through 9 call sites).
- A `ScopedDebugLoc` RAII guard wrapping `LLVMSetCurrentDebugLocation2` in `expression.cpp`.

The stash couldn't auto-pop at the end of the rebase (overlap with post-rebase semantic-merge fixes). The `native_walker` piece was hand-applied; the rest is parked. See `handoff.md` on the `debugging` branch for the recovery sequence.

## Workflow

When the sharedness arc closes on `experimental`:

1. From the `debugging` branch, run `fire rebase` to pull in the closure.
2. Run the test matrix — the 21 debugger tests should now go green without further work.
3. Recover the stash entry (Gates + minting fixes) and re-verify.
4. Land via `fire commit` to integrate into `experimental`.
5. Delete the `debugging` branch and this todo file.
