# Slab 15e Handoff — `simple_program_returning_an_int_explicit` is green; next driving test is `hardcoding_negative_numbers`

## Welcome

Start here:

1. Read `TL.md` (top to bottom — the required-reading list at the top is real, not boilerplate).
2. Read `docs/skills/migration-drive.md` — your operating instructions. **Re-read it after every compaction**, since the TL adds notes there as gotchas surface.
3. Drive on: `compiler_tests.rs::hardcoding_negative_numbers`. It's already `#[ignore]`-removed and is the only un-ignored test besides `simple_program_returning_an_int_explicit` (which stays un-ignored as a regression guard).

## Where you're picking up

Slab 15e was the first slab to land an end-to-end-passing test. `simple_program_returning_an_int_explicit` (`func main() int { return 3; }`) now goes through the full pipeline: parse → postparse → higher-typing → typing → `evaluate` → `HinputsT` construction → `lookup_function_by_human_name("main")` → return-type assertion. All eight `Slab 10` panic stubs in `compiler_outputs.rs` got filled in, plus the post-deferred phase of `Compiler::evaluate`, plus skeletons for `compile_i_tables` / `make_interface_edge_blueprints` / `ensure_deep_exports`.

The next test is one notch more complex: `func main() int { return -3; }` plus a tree-walker assertion.

## What `hardcoding_negative_numbers` does

`compiler_tests.rs:96-110`:

```rust
let compile = ...test("exported func main() int { return -3; }");
let main = compile.expect_compiler_outputs().lookup_function_by_human_name("main");
Collector.only(main, { case ConstantIntTE(IntegerTemplataT(-3), _, _) => true })
```

In Scala, `Collector.only(node, partialFn)` walks the entire expression tree of `node`, asserts that **exactly one** node matches `partialFn`, and (optionally) returns the matched node. It's used in **a lot** of tests across the typing pass — `compiler_mutate_tests.rs`, `compiler_ownership_tests.rs`, `compiler_solver_tests.rs` all reference it. It is **not yet migrated** to Rust; you'll be the one to bring it over.

## Likely path through the test

1. **Run the test first.** `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml > ./tmp/<session>.txt 2>&1`. Don't filter with `-E`; let all non-ignored tests run so regressions show. The test will panic somewhere — start with the panic site.

2. **Diagnose what's new vs `simple_program_returning_an_int_explicit`.** The new test program adds:
    - `exported` keyword on `main` — produces a non-empty `function_exports` collection in `coutputs`. That hits the `panic!` inside `ensure_deep_exports`'s `.for_each` over `function_exports`. Migrate the closure body next.
    - `-3` (negative literal) — hits the unary-minus path in `evaluate_expression`. Probably already handled or close to it (the `-3` parses as a unary `Negate` over `ConstantInt(3)`); track down what panics.
    - The `Collector.only` assertion at the end — this is post-compile, doesn't affect the typing pass itself, only the test harness.

3. **Migrate inside the panicking closure bodies.** Each panic in `ensure_deep_exports` / `compile_i_tables` / `make_interface_edge_blueprints` names the Scala line it's a stub for. Quote the Scala verbatim (per TL.md "Don't Simplify Scala On The Way Over") and translate line-for-line. Re-skeleton if the body has its own iteration; fill only the arms the test exercises.

4. **`Collector.only` migration is its own escalation.** Don't write a Rust analogue from scratch — the API design is a TL/architect call. When you get to the assertion line, escalate with: the Scala source location of `object Collector` (grep `Frontend/Tests` or similar), the call sites you've identified, and your read on whether it should be a free helper, a method on `FunctionDefinitionT`, a macro, or something else. The TL will design; you port.

5. **The pattern-match in the assertion** (`case ConstantIntTE(IntegerTemplataT(-3), _, _) => true`) becomes a Rust `if let` or `matches!` arm on something like:

    ```rust
    matches!(
        node,
        ReferenceExpressionTE::ConstantInt(ConstantIntTE {
            value: ITemplataT::Integer(IntegerTemplataT { value: -3, .. }),
            ..
        })
    )
    ```

    Verify the field names — `ConstantIntTE` and `IntegerTemplataT` are migrated (`expressions.rs:1373` and `templata.rs`), but the exact field shape may have drifted from Scala. Walk the Rust types before writing the pattern.

## State you should know about

- **`HinputsT` fields are `Vec<&'t T>`**, not owned `T`. Don't try to mutate or clone the `T`s. Per TL.md §"TemplatasStoreT Is `&'t` In All Env Structs" — same precedent.
- **Three HinputsT fields are populated via skeleton-with-panics** in their loop bodies (`interface_to_edge_blueprints`, `instantiation_name_to_instantiation_bounds`, plus `compile_i_tables`'s itables HashMap). For an `exported func main() int { return -3 }` program (no impls, no instantiation bounds expected), those panics shouldn't fire. If they DO, that's signal that this program triggers a path the previous test didn't — diagnose what produces the non-empty input and migrate the closure body.
- **`ensure_deep_exports` has a Scala-shaped skeleton** with panics in the closure / match-arm bodies. The `exported` keyword in this test program produces non-empty `function_exports`, so you'll hit a panic inside `function_exports.iter().for_each(...)`. That's the next thing to migrate.
- **SPDMX may complain about the iteration skeletons** in `ensure_deep_exports` and friends. The TL has approved temp-disables on those functions (see TL.md §"Skeleton-With-Panics vs SPDMX" for the standard rationale). If new temp-disable scopes are needed, **escalate** — don't temp-disable yourself.
- **`add_function` takes an extra `signature: &'t SignatureT<'s, 't>` parameter** that Scala doesn't have. Documented Rust-side adaptation to interning (CompilerOutputs doesn't hold the typing_interner). Don't refactor it away.
- **`lookup_function_by_human_name` should be `lookup_function_by_str`** per SPDMX exception J's table — there's a standing tidy item in TL.md's Known Residual Items. Don't worry about it for now; the function works under either name.

## The non-negotiables

- **Don't commit.** The architect handles all commits.
- **Don't Scala-edit.** Architect-level only (TL.md §"Editing Scala To Match A Rust Simplification").
- **Don't add novel logic.** TL.md "Guiding Principle" and SPDMX. Verbatim translation; if Rust can't compile a verbatim port, escalate.
- **Don't temp-disable Guardian shields.** Always escalate; the TL/architect issues the disable.
- **Keep `simple_program_returning_an_int_explicit` un-ignored** as a regression guard. If it starts failing, that's a real regression you need to fix before continuing on `hardcoding_negative_numbers`.
- **Escalate with context.** TL.md and migration-drive.md both list what to include — file/line on both Rust and Scala sides, exact error message, TFITCX classifications, options-considered with trade-offs. Quote, don't paraphrase.

Good luck. Ping the TL when you hit `Collector.only` — that one's mine to design.
