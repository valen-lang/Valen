# Accuracy report: Solver Must Choose Most Specific Type (SMCMST)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Templar/Infer Templar.md:507-571

## Verdict

This is a Scala-era design musing posing an open problem — "when a generic call site has multiple valid types for its inferred type parameter, how does the solver know to pick the most specific one?" — and sketching two candidate solutions (Approach A: track a possibilities set per rune; Approach B: let the solve halt, then revisit unsolved Sends/Isa rules and pick the most specific sender), settling on Approach B. It has no doc citation sites (`sites: []`), and it was never implemented as described: `complex_solve()` in `src/typing/rune_typing/rune_type_solver.rs:956` unconditionally panics with `"Unimplemented complex_solve"`, and `src/typing/infer/compiler_solver.rs` has no complex-solve stage at all. More importantly, the current design has explicitly rejected the problem framing itself: `docs/handoffs/call-site-dispatch-handoff.md:164-172` states plainly that `launch<T>(a &T, b &T)` called with two different subtypes is now a **type error** — `T` unifies exactly, first argument wins, no most-specific-common-ancestor guessing — and says outright "This retires SMCMST/CSALR." `docs/arcana/Generics.md:1711` also references SMCMST/CSALR as the (unimplemented, dead-end) hail-mary mechanism. The concept isn't wrong as history — it correctly identifies a real ambiguity the old Scala solver never cleanly solved either (per `docs/convos/convo-84-...md:3019`) — but the code and design direction it describes no longer exist and were deliberately abandoned in favor of a stricter, simpler rule (exact unification, explicit upcast required). Recommendation: leave in docs/old (dead history, already correctly superseded and cited as retired by the handoff and Generics.md) — do not migrate into docs/arcana.

## Claims

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A generic call like `launch<T impl IShip, Y impl IWeapon>(ship &T, weapon &Y) &T` called with concrete subtypes has two valid solutions for the return type (concrete type vs. bound) | UNVERIFIABLE (historical, Scala-era) | — | No live generics-inference ambiguity to check; not a claim about current code. |
| 2 | Solver needs a way to "choose the most specific type," and the doc proposes doing so via Approach B: halt the solve, then revisit unsolved Sends/Isa rules and pick the narrowest sender | FALSE (as a description of current code) | src/typing/rune_typing/rune_type_solver.rs:956-957 (`complex_solve()` panics, unimplemented); src/typing/infer/compiler_solver.rs (no complex-solve stage) | Never implemented in the Rust typing pass; superseded by exact-unification rule, see below. |
| 3 (implicit) | This most-specific-ancestor guessing is how generic type-parameter inference currently works | FALSE | docs/handoffs/call-site-dispatch-handoff.md:164-172 | Current design: `T` unifies exactly across call arguments; mismatched types is a type error, not resolved by picking a common ancestor. Explicit upcast required by the caller instead. |

## Stale citation sites

None — `sites: []`, and grep confirms no code cites `SMCMST` (only doc/convo prose references it, all of which already correctly frame it as retired).

## Uncited sites that embody the arcana

None found — the concern it describes (most-specific-common-ancestor inference) is not implemented anywhere in current code; `complex_solve()` is a stub panic, not a working mechanism, so there is nothing live to cite it from.

## Suggested text

N/A (D3 kind, verdict is obsolete but this is not major-inaccuracies from drift — it's a superseded design already correctly documented as dead by `docs/handoffs/call-site-dispatch-handoff.md` and `docs/arcana/Generics.md:1711`; no new arcana doc is warranted).
