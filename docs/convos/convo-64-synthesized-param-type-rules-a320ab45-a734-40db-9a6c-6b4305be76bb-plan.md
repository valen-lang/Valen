# Plan document

Source: `/Users/verdagon/.claude/plans/lexical-doodling-snowflake.md`
Session: a320ab45-a734-40db-9a6c-6b4305be76bb

---

# Put a synthesized parameter's type rules in the parameter, not the header

## Context

Vale2's ratified candidate filter (plan §10.10, their handoff's *"THE FILTER IS FINAL, AND IT IS
PURELY STATIC"*) decides *"params match the args"* before any solving, from three things read off
each parameter: **arity**, the **wrap chain** in `type_outer_ref_rules`, and the **value-type
template name** taken from `value_type_rules`' outermost `Call`.

Plan §4 claims we satisfy that filter by construction, because `declarations.rs` emits exactly the
`LookupSR` + `CallSR` pair it reads. **We emit the right rules in the wrong place.** Every
synthesized parameter is built with both per-param buckets empty:

```rust
// FrontendRust/src/typing/rust_interop/declarations.rs:147-148
scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),   // type_outer_ref_rules
scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),   // value_type_rules
```

and the parameter's `LookupSR`/`CallSR` go into the function's flat rule list instead. For a
source-written param, `function_scout.rs:418-445` fills two local vectors from
`translate_signature_templex` and hands them to `ParameterS::new`; nothing about a parameter's type
reaches the function's rules. So our declarations are structurally unlike what the postparser
produces for equivalent hand-written Vale — the exact self-check plan §8 (@SMLRZ) states.

**The consequence is not "invisible", it is worse.** Under the filter's spec an empty
`value_type_rules` reads as *"a bare rune, which accepts anything."* When the filter lands, every
imported Rust function becomes a candidate for every call of matching arity, and under
filter-is-final with `>1 → ambiguity`, ordinary Vale calls start colliding with Rust ones.

**Why nothing fails today**, and why no test can catch it: @PFVSZ's fold is
`header_rules ++ params.flat_map(value_type_rules ++ type_outer_ref_rules)`, so `all_rules` is the
same set either way. Both paths a synthesized extern actually travels — `is_light()` treats
`ExternBody` as light, so calls go
`evaluate_generic_light_function_from_call_for_prototype` → `..._from_call_for_prototype` and
definitions go `evaluate_generic_light_function_from_non_call` → `..._from_non_call_solving` — are
folding sites (`function_compiler_solving_layer.rs:406` and `:680`). The non-folding sites
(`:105`, `:216`, `:547`) are the pre-generic templated paths we never reach. And the filter that
would notice does not exist yet.

Intended outcome: a synthesized declaration is filterable by construction rather than by intent,
and the claim §4 makes becomes true.

## Scope

**Ours only — no core change.** Everything below is inside
`FrontendRust/src/typing/rust_interop/` plus the two doc files. The change is behaviour-neutral by
the fold argument above; the suite is the check.

## RFIGA

One slice. Its **R/F is unusual and the reason is worth stating**: there is no red available before
the fix. The property is invisible at every boundary a test can observe — the corpus extractor sees
only `HinputsT`, the fold makes the rule set identical, and the filter that reads the buckets is not
built. Rather than pretend, the slice validates the fix by **making it fail afterwards** (§0.3c:
validate a check by making it fire, and check *which* failure you got).

1. **A synthesized parameter's value-type rules live in its own `value_type_rules`.**
   * **R** — none available; see above. State this in the commit rather than implying coverage.
   * **F** — establish the baseline instead: **628 / 166 / 8** interop, **582 / 166 / 8** default.
     Any movement in either direction is a stop (plan §2).
   * **I** — in `synthesize_extern_function` (`declarations.rs:109-150`): replace the single shared
     `rules` vector with (a) a `header_rules` vector used **only** for the return type, and (b) a
     fresh vector created per parameter inside the loop, passed to `bind_sig_type` and handed
     straight to `ParameterS::new` as `value_type_rules`. `type_outer_ref_rules` stays empty — our
     params are by-value, and `ParameterS::new` asserts `full == value` when it is.
     **`next_synthetic` must stay function-scoped**, not per-parameter: it names synthetic runes, and
     resetting it per param would let two parameters mint the same rune name.
   * **G** — re-run both configs; expect **628 / 166 / 8** and **582 / 166 / 8**, unchanged.
   * **A** — full suite both configs, plus `cargo build --lib` at 7 warnings and the `valec-rs`
     driver at exit 0.

   Then the validation that replaces the missing red, as an explicit step:

   * **Probe** — blank the new `value_type_rules` without restoring the rules anywhere, re-run, and
     confirm the interop suite breaks. That is what proves a parameter's binding now flows through
     the bucket rather than through the header list. Revert the probe and re-measure to the numbers
     above. If it *doesn't* break, the move didn't land and the slice is not done.

## Files

- `FrontendRust/src/typing/rust_interop/declarations.rs` — the only code file. `bind_sig_type`
  itself needs no change: it already takes `rules: &mut Vec<IRulexSR<'s>>` and appends, so it works
  against a per-parameter vector unmodified. The change is entirely in its caller.
- `docs/convos/rust_interop/synthesized-declarations-plan.md` — §4's static-filterability bullet
  currently asserts the false half. Rewrite it forward (per `update-handoff`: no "we thought X"),
  stating that a parameter's value type is described by its own `value_type_rules` and that this is
  what the filter reads. §8's self-check gains the parameter-bucket case as a concrete instance.
- No change to `synthesize_extern_struct`: our extern structs have zero members, so there is no
  per-member rule bucket to misplace.

## Verification

```bash
cargo test --manifest-path ./FrontendRust/Cargo.toml --lib --features rust_interop > ./tmp/param-rules.txt 2>&1
grep "test result" ./tmp/param-rules.txt        # expect 628 passed; 166 failed; 8 ignored
cargo test --manifest-path ./FrontendRust/Cargo.toml --lib > ./tmp/param-rules.txt 2>&1
grep "test result" ./tmp/param-rules.txt        # expect 582 passed; 166 failed; 8 ignored
cargo build --manifest-path ./FrontendRust/Cargo.toml --lib > ./tmp/param-rules.txt 2>&1
grep -c "^warning" ./tmp/param-rules.txt        # expect 8 lines = 7 real warnings + rustc's summary
cargo run --manifest-path ./FrontendRust/Cargo.toml --features rust_interop \
    --bin valec-rs -- <fixture-dir> <out-dir>   # expect exit 0
```

The config's own gate (`cargo build` / `cargo nextest`) still cannot run — 9 errors in
`src/bin/valec/` against the `backend_ffi`/`pass_manager` modules the onion arc commented out of
`lib.rs`, unchanged by this work. `--lib` is the ratified substitute.

## Out of scope, deliberately

- **The `ArgumentRune` + `Equals` shape for a bare generic parameter.** The postparser uses the
  declared rune directly for `func foo<T>(x T)`; we mint an `ArgumentRune` and equate it. That is a
  second structural divergence from hand-written Vale, but it is filter-correct either way — an
  `Equals` is not a `Call`, so the parameter reads as a bare rune, which is the right answer for
  `x T`. Separate slice if we ever want it.
- **Building any part of the filter.** It is Vale2's and unstarted.
- **A test that observes declarations.** Making the buckets assertable needs an accessor on
  `TypingPassCompilation` (`typing/compilation.rs`), which is core. Worth raising separately — it
  would also give us the first test of the @NNGZ zero-arg `CallSR` property, which today has none —
  but it is not this change.

## Worth sending to Vale2

They are editing the @PFVSZ fold right now (uncommitted, `experimental-2`), and their two
`unimplemented!("header_rules alone: fold in the per-param type-binding rules")` landmines sit at
the non-folding sites. Our finding is the same invariant seen from the producing side, it is
refutable (file, line, the two empty slices), and §0.5 says the reporting half is the half that
pays.
