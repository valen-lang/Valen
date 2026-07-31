# Plan document

Source: `/Users/verdagon/.claude/plans/sunny-wiggling-pearl.md`
Session: 637b1efc-49f6-4630-b0e9-8d63a662d587

---

# Plan: Remove placeholder sharedness (T is always single-ownership)

## Context

Under the onion type model, **sharedness became structural**: a share (immutable/RC'd) citizen is never held bare — it only ever appears wrapped in `ShareRef`, and a `ShareRef` handle is itself single-owned. Consequently a generic type parameter `T` always binds to a single-ownership thing (a value, a `ShareRef` handle, a borrow, an own-box). Every sharedness-dependent decision (drop, clone, `weak`, bare-legality) is dispatched on the **wrap structure** of the concrete type T binds to, deferred to instantiation for placeholders — never on a sharedness/mutability tag stored on the placeholder.

So a **kind placeholder does not need a stored sharedness/mutability**. The current machinery that declares/looks it up is vestigial: `get_sharedness`'s only live caller (`expression/local_helper.rs:80`) computes `let mutable = ...` and discards it; every other `get_sharedness` call is commented out. On top of that, the mutability is laundered through a pointless `OwnershipT` encoding hop (`kind_mutable: bool` → `OwnershipT::{Own,Share}` at the caller → `SharednessT::{Single,Shared}` in the callee).

**Goal:** remove sharedness/mutability from kind placeholders end-to-end. Keep sharedness on real **citizen definitions** (structs/interfaces) untouched — that's a genuine, load-bearing property. Net result: delete a vestigial cluster and one `OwnershipT` dependency; also clears the `get_sharedness` non-exhaustive-match error (one of the E0004s).

## Principle: REMOVALS ONLY
This plan writes **no new code** — every change is a deletion. If executing a removal turns out to need compensating new code (a replacement arm, a new helper, a value to plug a gap), **do not write it**: drop a `// ZHERE:` marker at that spot describing what's needed, and move on. Additions are deferred, never smuggled into this plan. (Per the earlier analysis, all five removals below are pure deletions with no gap — but this is the guardrail if execution surprises us.)

## Invariant this rests on
Share citizens only ever appear `ShareRef`-wrapped, never bare (the onion validity table). As long as that holds, "T is always single" is sound. (Not changed by this plan — just relied upon.)

## Scope — REMOVE (placeholder sharedness)
1. **`create_kind_placeholder_inner`** (`templata_compiler.rs:1431`): drop the `kind_ownership: OwnershipT` param (line 1438) and the whole `let sharedness = match kind_ownership {…}; coutputs.declare_type_sharedness(kind_placeholder_template_id, sharedness);` block (1470-1475). The `register_with_compiler_outputs` block keeps its env/type declarations; only the sharedness store goes.
2. **Its caller** (`templata_compiler.rs:1404-1411`, in `create_placeholder_inner`): drop the `let (kind_mutable, _region_mutable) = …` computation and the extra arg; call `create_kind_placeholder_inner` without it. (The real `kind_mutable` source is already commented out; it currently just passes `OwnershipT::Own`.)
3. **`create_override_placeholder_mimicking`** (`edge_compiler.rs:201`): drop the `let mutability = coutputs.lookup_mutability(original_placeholder_template_id); coutputs.declare_type_sharedness(placeholder_template_id_ref, mutability);` pair in **both** arms (244-245 and 255-256). (These two arms are the same Coord/Kind collision noted earlier — merging them is a separate task; here we just remove the placeholder-sharedness copy from whichever arm(s) survive.)
4. **`get_sharedness`** (`compiler.rs:1698-1717`): **DELETE the whole function.** Confirmed entirely dead — its only live caller (`local_helper.rs:80`) discards the result, every other call site is commented out, and it's a latent non-exhaustive-match error (12 arms, no wildcard, missing the 4 onion ref-wraps). Deleting it removes that would-be `E0004` for free.
5. **`local_helper.rs:80`**: drop the now-orphaned `let mutable = self.get_sharedness(coutputs, reference_type2);` line — it's a dead binding (the `LocalVariable` it precedes has only `name` + `tyype`).

## KEEP (citizen sharedness — untouched)
- `struct_compiler.rs:104` (struct), `struct_compiler.rs:152` (interface), `struct_compiler_core.rs:453` (closure struct) — `declare_type_sharedness` on real definitions via `evaluate_sharedness(...)`.
- `compiler.rs:1713-1714` — `lookup_mutability` on struct/interface.
- The `declare_type_sharedness` / `lookup_mutability` fn definitions (`compiler_outputs.rs:331/534`).

## Adjacent / future (NOT in this removal)
- **`create_coord_placeholder_inner` has no definition** (grep finds no `fn`) — it's only *called* (`array_compiler.rs:518`, `:639`) with an `OwnershipT::Own` arg, plus a commented block at `templata_compiler.rs:1421-1423`. Reintroducing it is an **addition**, so it's out of this plan entirely — this plan does **not** touch `array_compiler`. Informational only: whoever re-adds it should omit the ownership/mutability param.
- `create_non_kind_non_region_placeholder_inner` (`templata_compiler.rs:1503`) takes **no** ownership param — nothing to do.

## Do NOT confuse / do NOT touch
- **`struct_compiler_get_sharedness`** (`struct_compiler.rs:290`) is a *separate*, LIVE function (reads `lookup_struct(...).sharedness` directly; consumed by `struct_constructor_macro.rs:177` to drive `constructor_return_ownership`). Similar name, unrelated — leave it.
- **Region mutability** (`_region_mutable`, `IRegionMutabilityS`, the commented coord-placeholder arm at `templata_compiler.rs:1413-1424`) is a different axis (regions), not kind sharedness. Out of scope.

## Possible follow-on (verify, don't force)
After the removal, `lookup_mutability` (`compiler_outputs.rs:534`) may have **zero remaining callers** (its only callers were inside the deleted `get_sharedness` and the removed `edge_compiler` copies). Leaving an unused `pub fn` is harmless (no warning). If a build confirms it's caller-less, deleting it is an optional tidy — but it's out of this plan's scope. `declare_type_sharedness` stays (still called by the citizen declares).

## Execution order
1. `create_kind_placeholder_inner` — drop param + sharedness block.
2. Its caller in `create_placeholder_inner` — drop the `kind_mutable` computation + arg.
3. `create_override_placeholder_mimicking` — drop the mutability copy in the surviving arm(s).
4. Delete `get_sharedness`; remove the dead `let mutable = …` at `local_helper.rs:80`.
5. Build; confirm no new errors and the removed sites are clean.

(All four removal spans are V-marker-free — no `NRVMX` shield friction on the deletions.)

## Verification
The crate is intentionally RED mid-arc (~305 errors), so this is a delta check, not a green check. Build to the session file:
```
cargo test --manifest-path FrontendRust/Cargo.toml --lib --no-run > tmp/onion-arc.txt 2>&1
grep -cE '^error(\[|:)' tmp/onion-arc.txt
```
Expect:
- **Total error count drops** (clears the `get_sharedness` `E0004`, the `OwnershipT`/sharedness references at the removed placeholder sites, and the dead `local_helper.rs:80` binding).
- **No NEW errors** introduced — in particular grep that the touched files no longer appear as error locations: `create_kind_placeholder_inner` region, `create_override_placeholder_mimicking`, the old `get_sharedness` span, `local_helper.rs`.
- Zero new warnings (the removals should not leave unused imports; check `OwnershipT`/`SharednessT` imports in `templata_compiler.rs`/`edge_compiler.rs`/`compiler.rs` — drop any that go unused).
