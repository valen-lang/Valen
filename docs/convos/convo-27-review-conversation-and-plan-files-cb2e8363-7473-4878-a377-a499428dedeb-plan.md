# Plan document

Source: `/Users/verdagon/.claude/plans/glimmering-honking-manatee.md`
Session: cb2e8363-7473-4878-a377-a499428dedeb

---

# Plan: Phase 2 — Unified Bare-Use + Borrow/Share Collapse Fix + Target-Side Coercions

**Scope estimate**: multi-week. ~5-7 days for the typing-pass changes alone, plus stdlib audit and cluster cleanup.

**Active references**: `/Volumes/V/Vale2/vcoord-handoff.md` (decision tables, mission framing). `/Volumes/V/Vale2/tmp/claude-conversation-2026-06-13-e1757dd0.md` (Phase 1 bail context). `/Volumes/V/Vale2/tmp/diagnostic-sample.txt` and `tmp/diagnostic-sample-2.txt` (triage data).

---

## Context

### Why this change exists

The Vale compiler's typing pass has three intertwined problems blocking ~55-60% of the 87 currently-`#[ignore = "deferred at experimental-2 squash baseline"]`-tagged tests:

1. **The "explicit `&` at callsite" interim convention.** Today, calling `print(my_str)` where `my_str` is an Own local and `print` expects `&str` fails — the user has to write `print(&my_str)` explicitly. The CHECKPOINT 22 stdlib sweep papered over this with manual `&` annotations. This is a UX pain point and creates an entire class of test failures.

2. **The `&Share T → Share T` type-system collapse.** When the typing pass tries to produce a `Borrow + share-flavored kind` coord, it silently collapses to `Share T`. This kills the type-system distinction between `drop(&T)` and `drop<T>(T)` for share-flavored types, producing the dominant ambiguity cluster — ~32% of the deferred cluster tests fail with "Multiple candidates for call: drop(X) / drop<X>(X)". The handoff calls this the "bucket-6 collapse."

3. **Silent auto-clone for Own non-primitive locals.** Today, bare-use of an Own struct local goes through `wrap_in_implicit_clone`, which clones the struct silently. This violates the principle of "expensive operations should be explicit" — users have no idea their `foo(my_struct)` is implicitly deep-copying.

The architect's chosen solution unifies these into one architectural change: **bare-use of any local always produces a `Borrow`-flavored coord, regardless of source ownership/kind, and target-side rules handle auto-coercions where they make sense.**

### What success looks like

After Phase 2 lands cleanly:

- Bare-use of an Own struct local resolves to a Borrow-flavored expression. `foo(my_struct)` where `foo: func(&Struct)` works without explicit `&`.
- Bare-use of a Share local resolves to a `Borrow + share-kind` expression. Same `foo` call works for share-flavored structs.
- Bare-use of an Own primitive local resolves to a `Borrow + primitive` expression. At an Own-target context, auto `implicit_clone(&p)` fires for primitives (the only auto-clone remaining).
- Own non-primitive → Own target raises a **compile error** demanding explicit `^s` (move).
- The `drop(&T)` vs `drop<T>(T)` ambiguity dissolves at the type-system level for share-flavored T — `&Share T` is now a distinct flavor.
- Suite delta: from `1091/0/119` baseline → expected `1140-1155 / 0 / 55-70`. Roughly 50-65 cluster tests un-ignore.
- The CHECKPOINT 22 interim "explicit `&` at callsite" stdlib sweep is no longer needed (those manual `&`s can be removed in a follow-up cleanup, not part of Phase 2 proper).

### Reading order before starting

1. `/Volumes/V/Vale2/vcoord-handoff.md` — read the active "Mission — Overload resolution & dispatch model redesign" section, especially the Coercions table and the rewritten Phase 2 in "Practical scope of work."
2. The REACTIVATED bucket-6 section in the same doc — its three-collapse-site analysis is the load-bearing reference for sub-arc (a) below.
3. `tmp/diagnostic-sample.txt` and `tmp/diagnostic-sample-2.txt` — the 22-test triage data showing what failure mechanisms exist in the cluster.

---

## The decision table (locked 2026-06-29)

| Source ownership | Source kind | Target wants | Action |
|---|---|---|---|
| Own | primitive | Own | bare-use → `Borrow + primitive`; auto `implicit_clone(&p)` → fresh Own |
| Own | primitive | Borrow | bare-use → `Borrow + primitive`; pass-through |
| Own | non-primitive (struct/interface/array) | Own | bare-use → `Borrow + kind`; **error** — user must write `^s` (explicit move) |
| Own | non-primitive | Borrow | bare-use → `Borrow + kind`; pass-through |
| Share | * | Own | not possible (sharedness mismatch) |
| Share | * | Share | bare-use → `Borrow + share-kind`; auto-alias to Share at target |
| Share | * | Borrow | bare-use → `Borrow + share-kind`; pass-through |
| Borrow | * | Own | error — `Borrow → Own` never coerces |
| Borrow | * | Borrow | soft-load (today's behavior) |

The full row "bare-use → produces Borrow" applies uniformly. Branching lives entirely on the target side.

---

## Architectural sub-arcs

The change is three architectural sub-arcs that must land together for the suite to be green. They are described below as "what to change." The implementation slices in the next section interleave them — don't try to land all of (a) before starting (b).

### Sub-arc (a) — Stop the `Borrow + share-kind → Share` collapse

Three frontend typing-pass sites + two instantiator sites currently collapse `Borrow + share-flavored kind` to `Share`. All five sites need consistent treatment so the new flavor is preserved end-to-end:

**(a.1) `soft_load` Share-arm `LoadAsP::Use` arm** at `FrontendRust/src/typing/expression/local_helper.rs:115`. Currently:

```rust
OwnershipT::Share => {
    match load_as_p {
        LoadAsP::Use => ReferenceExpressionTE::SoftLoad(
            self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Share })),
        ...
        LoadAsP::LoadAsBorrow => {
            ReferenceExpressionTE::SoftLoad(
                self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Borrow }))
        }
```

The `Use` arm should produce `OwnershipT::Borrow` (matching the LoadAsBorrow arm's behavior, which was the CHECKPOINT 22 partial fix). The post-change code: `target_ownership: OwnershipT::Borrow` for both Use and LoadAsBorrow.

**(a.2) `AugmentSR` Share arms, both directions** at `FrontendRust/src/typing/infer/compiler_solver.rs:1131-1199`. Two collapse points:

- **Outer→Inner direction at line 1149**: `SharednessT::Shared => outer_coord.ownership` returns the outer ownership as-is when the kind is shared. If outer is `Share`, inner becomes `Share` instead of being preserved as `Borrow + share-kind`. Needs to be `OwnershipT::Borrow` (the inner should reflect the augment's `&` semantics, not the outer's Share).
- **Inner→Outer direction at line 1178**: `SharednessT::Shared => evaluate_ownership(augment_ownership)` returns whatever the user-written augment said. For `&T` (Borrow augment), this correctly returns Borrow — but only if `evaluate_ownership` returns Borrow for `&`. Verify behavior and confirm.
- Line 1176 has the comment `// VCOORD: this should go away probably?` — the architect's existing hint that this arm needs revisiting.

**(a.3) `substitute_templatas_in_coord` composition** at `FrontendRust/src/typing/templata_compiler.rs:405-413`:

```rust
let result_ownership = match (ownership, c.coord.ownership) {
    (OwnershipT::Share, _) => OwnershipT::Share,         // <-- collapse #1
    (_, OwnershipT::Share) => OwnershipT::Share,         // <-- collapse #2 (this is the &Share T case)
    (OwnershipT::Own, OwnershipT::Own) => OwnershipT::Own,
    (OwnershipT::Own, OwnershipT::Borrow) => OwnershipT::Borrow,
    (OwnershipT::Borrow, OwnershipT::Own) => OwnershipT::Borrow,
    (OwnershipT::Borrow, OwnershipT::Borrow) => OwnershipT::Borrow,
    _ => unreachable!("remaining Weak-on-substituting-side ownership pairs are degenerate"),
};
```

The line `(_, OwnershipT::Share) => OwnershipT::Share` is the bucket-6 collapse for substitution. Change to a full match that preserves Borrow:
- `(OwnershipT::Borrow, OwnershipT::Share) => OwnershipT::Borrow` — preserves the Borrow flavor (the new distinct shape).
- `(OwnershipT::Share, OwnershipT::Share) => OwnershipT::Share` — keeps Share→Share natural.
- `(OwnershipT::Own, OwnershipT::Share) => OwnershipT::Share` — Own composed with Share = Share (no Borrow wrapper).
- `(OwnershipT::Weak, OwnershipT::Share)` — likely keep as collapse to Share or Weak; verify.

**(a.4) Instantiator `compose_ownerships`** at `FrontendRust/src/instantiating/instantiator.rs:2005-2034`. Lines 2008-2010:

```rust
(OwnershipT::Own, OwnershipI::MutableShare) | (OwnershipT::Own, OwnershipI::ImmutableShare)
| (OwnershipT::Borrow, OwnershipI::MutableShare) | (OwnershipT::Borrow, OwnershipI::ImmutableShare) => {
    OwnershipI::MutableShare
}
```

This collapses `(OwnershipT::Borrow, OwnershipI::*Share)` to `MutableShare`. Split into two:
- `(OwnershipT::Own, OwnershipI::*Share)` stays at `MutableShare` (Own composed with Share = Share).
- `(OwnershipT::Borrow, OwnershipI::*Share)` becomes `OwnershipI::MutableBorrow` (or a new `BorrowOfShare` variant — see open question below).

**(a.5) Instantiator `compose_ownerships_second`** at `FrontendRust/src/instantiating/instantiator.rs:2037-2053`. Line 2044:

```rust
(OwnershipT::Borrow, OwnershipI::MutableShare) => OwnershipT::Share,
```

Same fix direction: change to `OwnershipT::Borrow`.

**Open architectural question for sub-arc (a)** — surface to architect during Slice 1: does the instantiator need a NEW `OwnershipI::BorrowOfShare` variant (mirroring `MutableBorrow` / `MutableShare`), or is reusing `OwnershipI::MutableBorrow` with a share-flavored kind sufficient? The `OwnershipI` enum at `FrontendRust/src/instantiating/...` (find via grep) needs an audit. If `MutableBorrow` already carries a kind and the kind's sharedness can be queried downstream, no new variant. If downstream code branches on `OwnershipI` alone without consulting the kind, a new variant is needed.

### Sub-arc (b) — Unified bare-use

**(b.1) `coerce_to_reference_expression` Own-arm** at `FrontendRust/src/typing/expression/expression_compiler.rs:393-418`. Today:

```rust
ExpressionTE::Address(a) => {
    let range_with_parent: Vec<RangeS<'s>> =
        once(a.range()).chain(parent_ranges.iter().copied()).collect();
    match a.result().coord.ownership {
        OwnershipT::Own => {
            let _ = life;
            // VCOORD: this is likely at the wrong layer
            self.wrap_in_implicit_clone(coutputs, nenv, &range_with_parent, call_location, region, a)
        }
        _ => Ok(self.soft_load(nenv, &range_with_parent, a, LoadAsP::Use, region)),
    }
}
```

Replace with:

```rust
ExpressionTE::Address(a) => {
    let _ = life;  // not needed; kept to match pattern of life-threading in callers
    Ok(self.borrow_soft_load(coutputs, a))
}
```

That's it. The Own arm now routes through `borrow_soft_load` (which picks the right borrow flavor via `get_borrow_ownership`). The Share/Borrow/Weak arms also flow through `borrow_soft_load` once (a.1) is landed (so the soft_load Share-arm Use produces Borrow).

**(b.2) `get_borrow_ownership` for primitives** at `FrontendRust/src/typing/expression/local_helper.rs:212-229`. Currently returns `Share` for `Int`/`Bool`/`Float`/`Str`/`Void`:

```rust
KindT::Int(_) => OwnershipT::Share,   // VCOORD comment says this is a workaround
```

After (a) lands, primitives need to return `Borrow` here so bare-use produces `Borrow + primitive`, not `Share + primitive`. Change all five primitive arms (and confirm `OverloadSet` and aggregate kinds remain unchanged). The VCOORD comment can be removed.

**(b.3) Reference-Own (temporary) case in `coerce_to_reference_expression`** at line 384. Today:

```rust
match expr_2 {
    ExpressionTE::Reference(r) => Ok(r),  // <-- pass-through, no target awareness
    ExpressionTE::Address(a) => { ... }
}
```

For Phase 2's call-site case where a temporary (`str(i)` etc.) is passed to a Borrow-needing target, the temporary needs to be materialized into a hidden local first. The existing helper is `make_temporary_local_defer` at `local_helper.rs:35-56`. It already takes a `ReferenceExpressionTE`, allocates a temporary local via `make_temporary_local`, wraps in `LetAndLend`, registers a defer-drop, and returns a `DeferTE` (which is a `ReferenceExpressionTE::Defer` variant).

The signature asserts `target_ownership == OwnershipT::Borrow`. Today this helper exists but isn't routinely called for arbitrary Own temporaries — it gets used in narrower contexts. For Phase 2, this helper becomes the standard path for Own temporaries flowing into Borrow targets.

The call-site dispatch happens at `convert_helper.rs` (see sub-arc (c)), not at `coerce_to_reference_expression` (which doesn't know the target). So this is wired up via the target-side rules, not at `coerce_to_reference_expression`.

### Sub-arc (c) — Target-side auto-coercions in `convert()`

**(c.1) `convert()` at `FrontendRust/src/typing/convert_helper.rs:48-104`.** Today the ownership match is:

```rust
let converted_expr =
    match (source_ownership, target_ownership) {
        (OwnershipT::Own, OwnershipT::Own) => converted_kind_expr,
        (OwnershipT::Borrow, OwnershipT::Own) => panic!("Supplied a borrow but target wants to own the argument"),
        (OwnershipT::Own, OwnershipT::Borrow) => panic!("Supplied an owning but target wants to only borrow"),
        (OwnershipT::Borrow, OwnershipT::Borrow) => converted_kind_expr,
        (OwnershipT::Share, OwnershipT::Share) => converted_kind_expr,
        (OwnershipT::Weak, OwnershipT::Weak) => converted_kind_expr,
        _ => panic!("Supplied a {:?} but target wants {:?}", source_ownership, target_ownership),
    };
```

After (b) lands, bare-use produces Borrow, so:
- The `(Own, Own)` arm becomes rare — only struct literals or expression results that are genuinely Own (not bare-use).
- The `(Borrow, Borrow)` arm with matching kinds is the new common case.

The rewrite handles target-side auto-coercions. Pseudo-code:

```rust
let converted_expr = match (source_ownership, target_ownership) {
    // Pass-through cases (kinds already verified equal upstream)
    (OwnershipT::Own, OwnershipT::Own) => converted_kind_expr,
    (OwnershipT::Borrow, OwnershipT::Borrow) => converted_kind_expr,
    (OwnershipT::Share, OwnershipT::Share) => converted_kind_expr,
    (OwnershipT::Weak, OwnershipT::Weak) => converted_kind_expr,

    // New: Borrow + share-kind → Share at target = auto-alias.
    // After (a.1)–(a.3), bare-use of a Share local produces Borrow + share-kind.
    // The alias is a SoftLoad with target_ownership = Share.
    // BUT note: convert() takes a ReferenceExpressionTE not an Address, so this
    // is not a soft_load — it's a no-op at the type level if the simplifier
    // peephole-folds it, or it's a deliberate re-flavor expression.
    // Need to verify whether there's an existing "alias" IR node we can use,
    // or whether we synthesize a SoftLoad-equivalent at the Reference layer.
    (OwnershipT::Borrow, OwnershipT::Share) if self.get_sharedness(coutputs, target_pointer_type.kind) == SharednessT::Shared => {
        // Use an existing aliasing IR node, or fabricate one. SURFACE TO ARCHITECT.
        todo!("Phase 2: emit Borrow+share-kind → Share alias IR")
    }

    // New: Borrow + primitive → Own at target = auto implicit_clone(&p).
    // Mirrors today's wrap_in_implicit_clone but called from the target side.
    (OwnershipT::Borrow, OwnershipT::Own) if self.is_primitive(target_pointer_type.kind) => {
        // Resolve implicit_clone(&primitive_kind) and emit the call.
        // Reuse the implicit_clone resolution logic from wrap_in_implicit_clone
        // (extract to a helper if needed).
        self.emit_implicit_clone_call(coutputs, nenv, range, call_location, region, converted_kind_expr)
    }

    // ERROR: Own non-primitive → Own. User must write ^s.
    (OwnershipT::Own, OwnershipT::Own) if !self.is_primitive(...kind) => {
        // Note: this is reached only for non-bare-use Own values, since bare-use
        // produces Borrow. So this would trigger if the user writes e.g.
        // `foo(SomeStruct())` and foo takes Own — the temporary is Own and
        // target is Own. That should be OK (no error).
        // HMMMM — wait, this conflicts with the decision table.
        // Re-examine: bare-use produces Borrow, but constructor calls produce
        // Own temporaries. The decision table's "Own non-primitive → Own = error"
        // is for BARE-USE of a local, not for fresh temporaries.
        // So this match arm is actually fine as pass-through.
        converted_kind_expr
    }

    // Hard error: Borrow → Own with non-primitive kind.
    (OwnershipT::Borrow, OwnershipT::Own) => {
        // emit a CompileError variant: BorrowToOwnNotPermitted or similar
        panic!("Implement: Borrow→Own error CompileError variant")
    }

    // Hard error: Own non-primitive → Borrow (shouldn't happen after Phase 2 — bare-use
    // already produced Borrow; this would only arise for a fresh Own temporary at a
    // Borrow target, which IS legitimate — auto-materialize via make_temporary_local_defer).
    (OwnershipT::Own, OwnershipT::Borrow) => {
        // For non-bare Own values (constructor results, function returns), promote to
        // hidden local via make_temporary_local_defer.
        // SURFACE TO ARCHITECT: confirm convert() is the right layer for this, OR if
        // make_temporary_local_defer should be called by the caller of convert() instead.
        todo!("Phase 2: materialize Own temporary as hidden local at Borrow target")
    }

    _ => panic!("Supplied a {:?} but target wants {:?}", source_ownership, target_ownership),
};
```

Open questions for (c.1) that need architect input during Slice 4:
- Is there an existing IR node for "re-flavor Borrow to Share" (alias) that's separate from `SoftLoad`? Likely yes — look for `Alias`/`Refcount`/similar in `FrontendRust/src/typing/ast/expressions.rs`. If not, may need to synthesize via SoftLoad-at-Reference-layer.
- The `make_temporary_local_defer` call from `convert()` for Own temporaries → Borrow target needs to thread through the `life`/`nenv` machinery. `convert()` already has `nenv`. Confirm `life` can be reasonably derived (it's the LocationInFunctionEnvironmentT — comes from the caller).
- The Own-primitive→Own case today goes through `wrap_in_implicit_clone` in `coerce_to_reference_expression`. After Phase 2, it needs to go through `convert()` since bare-use no longer fires implicit_clone. Extract the implicit_clone resolution logic into a reusable helper.

**(c.2) `is_type_convertible` at `FrontendRust/src/typing/templata_compiler.rs:1179-1194`.** Currently rejects ALL cross-ownership conversions. After Phase 2, the resolver's `params_match` (non-exact mode) needs to accept the new auto-coercion shapes:
- `(Own, Borrow)` with non-primitive kind — TRUE (will auto-borrow at convert time).
- `(Borrow, Share)` with share-flavored kind — TRUE (will auto-alias at convert time).
- `(Borrow, Own)` with primitive kind — TRUE (will auto-implicit_clone at convert time).
- `(Own, Own)` with non-primitive kind — FALSE (user must write ^s).
- `(Borrow, Own)` with non-primitive kind — FALSE (no coercion).
- All other cross-ownership — FALSE.

The new logic should match the decision table cell-for-cell.

---

## Reusable infrastructure already in place

These helpers exist and should be used unchanged or with minimal edits:

| Helper | Path | What it does |
|---|---|---|
| `borrow_soft_load` | `typing/expression/local_helper.rs:207-211` | Wraps `get_borrow_ownership` + `SoftLoadTE`. Reuse in `coerce_to_reference_expression` Own-arm. |
| `get_borrow_ownership` | `typing/expression/local_helper.rs:212-229` | Picks borrow flavor by kind. Needs update for primitives (sub-arc b.2). |
| `make_temporary_local` | `typing/expression/local_helper.rs:27-33` | Allocates an unnamed reference local. |
| `make_temporary_local_defer` | `typing/expression/local_helper.rs:35-56` | Materializes a temporary into a hidden local + LetAndLend + deferred drop. Returns `DeferTE`. |
| `soft_load` | `typing/expression/local_helper.rs:111-205` | Standard soft-load. Used by `borrow_soft_load`. |
| `wrap_in_implicit_clone` | `typing/expression/expression_compiler.rs:424-454` | Today's clone wiring. Phase 2 demotes this — extract its `resolve_function(implicit_clone)` logic into a helper for reuse from `convert()`. |
| `DeferTE` | `typing/ast/expressions.rs:411-440` | The expression type returned by `make_temporary_local_defer`. Already a `ReferenceExpressionTE::Defer` variant. |

---

## Implementation slices (RFIGA)

Each slice is one RFIGA cycle: write the red test(s), run them to confirm they fail for the expected reason ("Tests are correctly failing, proceeding with implementation"), do the minimum implementation, re-run to green, full-suite check.

The order is chosen to minimize regression-valley pain: start with the smallest behavior change and gradually expand.

### Slice 1 — Tracer: Single Own struct bare-use → Borrow at Borrow target

**Goal**: smallest end-to-end change. `func bork(x &SomeStruct) {}; func main() { bork(SomeStruct()); }` should compile. This is the simplest auto-borrow case — Single Own struct, not involving Share kinds, not involving the collapse fix.

**R (Red)**:
- Add a typing-pass test in `FrontendRust/src/typing/test/compiler_tests.rs` (alongside existing test_overloads et al.) that defines a struct, a function taking `&Struct`, and a main that calls with bare struct. Assert the call resolves (no panic).
- Un-ignore `typing::test::compiler_solver_tests::pointer_becomes_share_if_kind_is_immutable` (it has the exact shape).

**F (Fail)**:
- Run both. Today they should fail with "Solver conflict on rune (arg 0): was SomeStruct but now concluding &SomeStruct" (which is the same error from the original diagnostic).
- Report: "Tests are correctly failing, proceeding with implementation."

**I (Implement)**:
- Update `coerce_to_reference_expression` Own arm at `expression_compiler.rs:408-413` to call `borrow_soft_load(coutputs, a)` instead of `wrap_in_implicit_clone(...)`. Don't touch `get_borrow_ownership` yet — Single struct kind already returns Borrow there.
- Update `is_type_convertible` at `templata_compiler.rs:1179-1194` to permit `(Own, Borrow)` when the kind matches AND the kind is non-primitive. Don't add the other rows yet.
- Update `convert()` at `convert_helper.rs:96` to handle `(Own, Borrow)` for non-primitive — but since bare-use now produces Borrow, this case is uncommon. For fresh Own temporaries (constructor calls) to Borrow target, route through `make_temporary_local_defer`. Surface to architect if this is unclear.
- Build clean.

**G (Green)**: both tests pass.

**A (All-tests)**: run the full suite.

```bash
cargo nextest run --manifest-path /Volumes/V/Vale2/FrontendRust/Cargo.toml --no-fail-fast > /Volumes/V/Vale2/tmp/phase2-slice1.txt 2>&1
grep -E "Summary|test result" /Volumes/V/Vale2/tmp/phase2-slice1.txt | tail -5
```

Confirm 1091 baseline holds plus 2 new tests pass. Expected delta: +2 passes. If regressions appear (other tests fail), the most likely cause is: stdlib code that relied on silent `implicit_clone` for Own non-primitives is now bare-use-borrowing instead. Triage those (they're the stdlib `^s` audit candidates for Slice 2).

**Stop and surface to architect if**: more than ~5 regressions appear, or any test fails with a panic that doesn't reference an Own→Borrow shape.

### Slice 2 — Stdlib `^s` audit pre-flight

**Goal**: identify and rewrite stdlib callsites that today rely on silent `implicit_clone` for Own non-primitive → Own target. These will become errors in Slice 3, so they must be cleaned up first.

This slice is NOT a code-implementation slice — it's an audit + targeted edits to stdlib `.vale` files.

**Where to look**:
- Root: `/Volumes/V/Vale2/FrontendRust/src/builtins/resources/`
- Files of interest from the third agent's investigation:
  - `drop.vale` (lines 12-15 — universal owned blanket — see open question below)
  - `Result.vale` (lines 22-26, 40-43 — owned param patterns)
  - `Opt.vale` (lines 19-23, 45-48 — owned param patterns)
  - `arrays.vale` (lines 16-30 — owned generator pattern)
  - Plus anything else returning a fresh Own non-primitive and then passing it to another function expecting Own

**Process**:
1. Run a baseline test pass to confirm everything green: `cargo nextest run ... > tmp/phase2-slice2-baseline.txt 2>&1`.
2. For each stdlib file, grep for patterns matching `func X(param OwnNonPrimitive)` (taking owned non-primitive by value).
3. For each such function, find callsites and check whether they pass `^arg` (already explicit) or bare `arg` (silently relying on auto-clone). The bare ones need updating.
4. Hand-rewrite each bare `arg` to `^arg` at the callsite.
5. Re-run suite. If anything breaks, surface — may indicate a callsite the audit missed or a different mechanism.

**Open question to architect**: does the universal `drop<T>(v void, x T) where func drop(T)void { drop(^x) }` in `drop.vale:12-15` stay, get removed (per the "no universal owned blankets, ever" note in the open items), or get rewritten with explicit `^x`? Surface before touching.

**Done criterion**: stdlib audit complete, all callsites either use `^s` or correctly pass borrows. Suite still at 1091/0/119 baseline (no behavior change yet).

### Slice 3 — Own non-primitive → Own target becomes an error

**Goal**: enforce "user must write `^s`" for the Own non-primitive → Own case. Today this silently auto-clones via `wrap_in_implicit_clone`. After Slice 1, bare-use no longer routes through `wrap_in_implicit_clone` for non-primitives; this slice replaces it with an explicit error.

**R**: add a typing-pass test that defines `struct Foo {}; func consume(f Foo) {}; func main() { x = Foo(); consume(x); }`. Assert it fails with the new `MustExplicitlyMove` error variant (or whatever you name it).

**F**: today it passes (silent auto-clone). After Slice 1 changes, it might already fail with a different error (Borrow vs Own mismatch). The new test asserts a specific error variant.

**I**:
- Add new variant to `ICompileErrorT` in `FrontendRust/src/typing/compiler_error_reporter.rs`: `MustExplicitlyMoveT { range, arg_coord }` or similar.
- Add humanizer arm in `FrontendRust/src/typing/compiler_error_humanizer.rs` producing message like `"Cannot pass owned value as owned argument silently. Write '^x' to explicitly move."`
- In `convert()` at `convert_helper.rs:96` (or wherever the Own non-primitive → Own path falls through), emit the new error variant.
- Build clean.

**G**: the new test asserts the new variant; passes.

**A**: full suite. Expected delta: 0 if Slice 2's audit was complete; otherwise a flurry of regressions pointing to missed audit sites. Iterate Slice 2.

### Slice 4 — Type-system collapse fix (bucket-6 sub-arc (a))

**Goal**: stop the `&Share T → Share T` collapse at all five sites. After this slice, bare-use of a Share local produces `Borrow + share-kind` instead of `Share T`.

This slice is the highest-risk part of Phase 2 — it touches the typing-pass solver, the substitution composition, and the instantiator. All five sites must land together; landing partial subset causes worse regressions than not landing at all (the bucket-6 mission notes explicitly).

**R**: write a typing-pass test that defines a share struct (`struct Lam share {}`), takes its bare-use, and asserts the coord is `Borrow + share-kind` (not `Share`). Inspect the resulting `CoordT`'s ownership field via the test's collect_only_tnode! pattern.

**F**: should fail today (the collapse produces Share at one of the five sites).

**I (multi-step, do all in one commit)**:
- **(a.1)** `local_helper.rs:115`: change `target_ownership: OwnershipT::Share` to `OwnershipT::Borrow` for `LoadAsP::Use` Share-arm.
- **(a.2)** `compiler_solver.rs:1149`: change `SharednessT::Shared => outer_coord.ownership` to `SharednessT::Shared => OwnershipT::Borrow`. Also `compiler_solver.rs:1178`: verify `evaluate_ownership(augment_ownership)` returns Borrow when augment is `&` — should already be correct; if not, force Borrow here too.
- **(a.3)** `templata_compiler.rs:405-413`: replace the Share-dominance arms with a full match preserving Borrow for `(Borrow, Share)`.
- **(a.4)** `instantiator.rs:2008-2010`: split the `(Borrow, *Share)` cases out — they become `OwnershipI::MutableBorrow` (or new variant if architect approves).
- **(a.5)** `instantiator.rs:2044`: change `(OwnershipT::Borrow, OwnershipI::MutableShare) => OwnershipT::Share` to `OwnershipT::Borrow`.

**Surface to architect during this slice**:
- Whether to add `OwnershipI::BorrowOfShare` as a new variant (vs reusing `MutableBorrow`).
- Whether `evaluate_ownership` for `&` returns Borrow uniformly (verify by inspection).
- The fallout. Expect 30-50 tests to break temporarily — those are the bucket-6-blocked ones that should re-pass once Slice 5+6 lands.

**G**: the new typing-pass test passes (coord is `Borrow + share-kind`).

**A**: full suite. Expected delta: -30 to -50 (regression valley). DO NOT FIX REGRESSIONS IN THIS SLICE — they should re-pass after Slices 5+6. Document each unique failure mode so Slice 5+6 can target them.

### Slice 5 — Unified bare-use for Share locals + primitive bare-use

**Goal**: complete sub-arc (b). After Slice 4, the type system distinguishes `Borrow + share-kind`. After Slice 5, bare-use of Share locals and Own primitives produces `Borrow + share-kind` / `Borrow + primitive` consistently.

**R**: extend the typing-pass test from Slice 4 to also test bare-use of an Own primitive and bare-use of a Share local. Assert the resulting coord ownerships.

**F**: Slice 4 alone produces `Borrow + share-kind` for some Share paths but not all — primitives still go through `get_borrow_ownership → Share`.

**I**:
- **(b.2)** `local_helper.rs:212-229` `get_borrow_ownership`: change `KindT::Int(_) | KindT::Bool(_) | KindT::Float(_) | KindT::Str(_) | KindT::Void(_)` to return `OwnershipT::Borrow`. Remove the VCOORD comment.
- Verify `coerce_to_reference_expression`'s Own arm (changed in Slice 1) now routes through `borrow_soft_load` which produces `Borrow + primitive` for primitives (consequence of the get_borrow_ownership change).

**G**: all three coord-shape tests (struct, primitive, share-struct bare-use) pass.

**A**: full suite. Most of the Slice 4 regression valley should start re-passing. Expected delta: +20-30 (recovering some of Slice 4's losses).

### Slice 6 — Target-side auto-coercions (sub-arc (c))

**Goal**: complete sub-arc (c). After this slice, the resolver/`convert()` boundary handles all the auto-coercion shapes from the decision table.

**R**: write tests for each auto-coercion shape:
- Bare Share local → Share target = alias.
- Bare Share local → Borrow target = pass-through (no coercion needed).
- Bare Own primitive → Own target = auto implicit_clone(&p).
- Bare Own primitive → Borrow target = pass-through.

**F**: today (post-Slice-5) these likely fail at `convert()` with the "supplied X but target wants Y" panic.

**I**:
- **(c.1)** Rewrite `convert()` at `convert_helper.rs:48-104` per the pseudo-code in the Architectural sub-arcs section above. Surface architectural questions to architect during the slice:
  - The alias IR node question (does one exist or do we synthesize via SoftLoad?).
  - The Own temporary → Borrow target materialization (via `make_temporary_local_defer`).
- **(c.2)** Update `is_type_convertible` at `templata_compiler.rs:1179-1194` to match the decision table.
- **(c.3)** Extract the `implicit_clone` resolution logic from `wrap_in_implicit_clone` into a reusable helper that `convert()` can call. Don't delete `wrap_in_implicit_clone` yet — it might still be referenced; mark for removal in cleanup slice.

**G**: the new tests pass.

**A**: full suite. Most of Slice 4's remaining regressions should resolve. Expected suite state: 1091 baseline + ~50 cluster wins = ~1140-1155.

### Slice 7 — Sister-test sweep, un-ignore confirmed wins

**Goal**: confirm cluster wins by un-ignoring tests that now pass.

**R**: enumerate the 22 triage-classified Phase-2/Phase-5/test-source tests (see `tmp/diagnostic-sample.txt` and `tmp/diagnostic-sample-2.txt`) plus the auto-borrow-shaped ones from the broader 87-test cluster. Un-ignore them.

**F**: run the suite; expected outcomes:
- Phase-2 tests (5 from sample): green.
- Phase-5 (drop blanket ambiguity) tests: now also green if Slice 4's collapse fix dissolved the auto-generated-drop vs universal-drop-blanket clash. Verify experimentally.
- Q1-hammer tests: still red (unrelated mechanism, out of Phase 2 scope).
- Frontend-macro tests (struct_drop_macro Result propagation): still red.

**I**: NO CODE CHANGE in this slice. For tests that pass, leave un-ignored. For tests that re-fail with a non-Phase-2 mechanism, re-ignore with an updated rationale tag (e.g. `#[ignore = "Q1-hammer pending separate scoping"]`) — per architect approval, per the no-`#[ignore]`-addition rule.

**G**: suite at the post-Slice-6 baseline + N (count of un-ignored tests).

**A**: final full suite. Report final delta. Surface to architect with the post-Phase-2 cluster composition report.

### Optional Slice 8 — Stdlib `&` cleanup (follow-up)

**Goal**: remove the CHECKPOINT 22 interim "explicit `&` at callsites" workaround from stdlib that's now redundant.

This is OUT OF SCOPE for Phase 2 proper (Phase 2 enables it, not requires it). Document for follow-up but don't tackle in this arc.

---

## Verification

After each slice:
- `cargo build --manifest-path /Volumes/V/Vale2/FrontendRust/Cargo.toml --lib` — must be warning-free.
- `cargo nextest run --manifest-path /Volumes/V/Vale2/FrontendRust/Cargo.toml --no-fail-fast > /Volumes/V/Vale2/tmp/phase2-progress.txt 2>&1` (use the same file across slices for easy comparison).
- `grep -E "Summary|test result" /Volumes/V/Vale2/tmp/phase2-progress.txt | tail -5` — quick check.

After Phase 2 lands fully:
- Run the 22-test triage batch from earlier (commands in `tmp/diagnostic-sample-2.txt`) and verify Phase-2-shaped ones now pass.
- Verify a hand-built end-to-end test: write a small Vale program using share structs, Own structs, primitives, bare-use, and explicit `^s`. Compile and run.

---

## Out of scope (explicit boundaries)

- **Phase 3 (namespace dispatch)**: separate, larger arc. Resolves the remaining cluster ambiguities that Phase 2 doesn't touch.
- **Phase 4 (typeclass reorganization)**: downstream of Phase 3.
- **Q1-hammer fallout** (the `expression_hammer.rs:368` OwnH vs MutableBorrowH cluster, ~4/22 of triage sample): separate concern in the simplifier, NOT a Phase 2 target.
- **`struct_drop_macro` Result-propagation gaps** (test_int_generic, typing_pass_on_roguelike): separate frontend-macro work.
- **Backend changes**: the handoff says no backend work until frontend is solid. Phase 2 is frontend-only.
- **Adding new `#[ignore]`s**: per the handoff's "NO `#[ignore]` additions" rule. Slice 7 re-ignores require per-test architect approval.
- **Stdlib `&` workaround cleanup**: Slice 8 is the follow-up, not part of Phase 2 proper.
- **Phase 1 error-reporting redesign**: bailed. Don't revisit unless architect re-opens.

---

## Common pitfalls and risks

1. **Don't try to land sub-arcs (a)+(b)+(c) without Slice 1 first.** The full collapse fix is much riskier than the tracer bullet. Confirm the architectural intervention points work on the simplest case before scaling up.

2. **Slice 4 will look terrible mid-flight.** The full suite will regress by 30-50 tests temporarily. That's expected. Slices 5+6 recover them. If you panic and try to fix the regressions mid-Slice-4, you'll spend hours fighting symptoms.

3. **Watch out for the instantiator's `OwnershipI` enum.** Sub-arc (a.4)+(a.5) may need a new `OwnershipI::BorrowOfShare` variant. Check the enum definition and grep all match-on-OwnershipI sites to confirm coverage. SURFACE EARLY if the enum needs extension.

4. **`make_temporary_local_defer` already asserts `target_ownership == Borrow`.** Don't relax that assertion casually — if you find yourself needing to materialize for a non-Borrow target, you're probably doing the wrong thing. The assertion encodes the "this helper is for Own → Borrow" invariant.

5. **`wrap_in_implicit_clone`'s `resolve_function("implicit_clone")` lookup.** When extracting this for `convert()` to call, preserve the `range`, `call_location`, and `region` threading exactly — the lookup happens in the callsite env, not the resolved-callee env.

6. **`is_type_convertible` is called from non-call-arg contexts too.** Grep all callers before changing semantics. If it's used for "can these types be assigned to each other" in a let-binding context, the auto-coercion changes might over-permit. Default safe: only relax it when called from `params_match` with `exact == false`.

7. **The `evaluate_ownership` function in (a.2).** I haven't read it personally — verify it returns the right thing for the `&` augment ownership. The architect's existing VCOORD comment ("this should go away probably?") at line 1176 suggests this whole arm may be reworkable.

8. **Suite-running discipline.** Always pipe `cargo nextest` to a fixed file in `./tmp/`. Per CLAUDE.md: never chain `cargo nextest ... | tail`. Use separate commands for run and inspection. Same file across slices for diff-ability.

9. **The architect's commit policy.** NEVER commit without the literal phrase "fire commit." Each slice's working tree changes stay uncommitted until architect approval.

10. **The `#[ignore]` rule.** NEVER add `#[ignore]` without explicit architect approval. Slice 7 has the only legitimate re-ignore step (un-ignored-then-re-ignored with new rationale).

---

## When to surface to architect

Surface immediately (don't push through) if:
- Slice 1 produces more than ~5 regressions.
- Slice 4 produces backend-side panics (suggests sub-arc (a) needs CoordH legality work too).
- The `OwnershipI` enum needs a new variant.
- The alias IR node for sub-arc (c.1) doesn't exist (need to design one).
- `evaluate_ownership` behavior is unexpected.
- Stdlib audit reveals callsites that can't be cleanly rewritten with `^s`.
- Any test fails with a panic that doesn't reference an ownership-related shape mismatch.

Don't surface for:
- Expected regression valley in Slice 4 (just document and continue).
- Routine "wrong-error-message" mismatches in tests with `assert_humanized_eq` — those are mechanical re-snapshots.
- New `#[ignore]` rationale text changes (the no-ignore rule is about ADDITIONS, not modifications to existing ignores — but per-test architect approval is still required for Slice 7).

---

## Reference: existing helpers and where to call them from

| Helper | Location | When to call |
|---|---|---|
| `borrow_soft_load(coutputs, addr)` | `local_helper.rs:207` | From `coerce_to_reference_expression` Own arm (Slice 1). |
| `get_borrow_ownership(coutputs, kind)` | `local_helper.rs:212` | Internally by `borrow_soft_load`. Update primitives in Slice 5. |
| `make_temporary_local_defer(coutputs, nenv, range, call_location, life, region, expr, OwnershipT::Borrow)` | `local_helper.rs:35` | From `convert()` Own→Borrow path for fresh Own temporaries (Slice 6). Wrap the result as `ReferenceExpressionTE::Defer(defer)`. |
| `resolve_function(...)` for `implicit_clone` | Used internally by `wrap_in_implicit_clone` at `expression_compiler.rs:432-454` | Extract to reusable helper for `convert()` to call (Slice 6). |
| `is_primitive(kind)` | grep for it in `FrontendRust/src/typing/` | Used in `convert()`'s new arms. |
| `get_sharedness(coutputs, kind)` | grep for it | Used to distinguish share-flavored kinds from Single. |

---

## Final note for the implementor

This is a complex multi-week change. The decision table at the top of this plan is **locked** — refer back to it if you're uncertain about any specific (source, kind, target) combination. The implementation should reproduce the table exactly.

If you find yourself wanting to deviate from the table to make a test pass — STOP and surface. The table is the architect's answer to a multi-hour design conversation. Deviations need explicit re-approval.

When in doubt about implementation details (which IR node to use, which helper to extract, which file to edit), prefer surfacing one extra question to the architect over making the wrong silent call. Phase 1 was bailed because of mid-implementation assumption drift; don't repeat the pattern.

Good luck.
