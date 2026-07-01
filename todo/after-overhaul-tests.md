# After-Overhaul Tests

The `after-overhaul-tests` branch parks the 22 `#[ignore]`d FrontendRust tests that were removed from `experimental-1` in `3cef0eca4`. It is `experimental-1` plus a single inverse "restore" commit (`4ae31fb0d`), so whenever it rebases from any branch in the `experimental{,1,2,3,4}` family the upstream delete replays first (lines already gone in the parent) and the restore commit re-adds them on top. The corpus survives arbitrary advances of the experimental family.

Buckets parked there:

- **3 bare `#[ignore]` with no documented reason** in `FrontendRust/src/lexing/tests/lexing_iterator_tests.rs`: `peek_n_one_at_multibyte_boundary`, `chevron_comment_consumption_leaves_position_at_byte_index`, `testvale_full_file_parses`. Either annotate with a real justification or investigate whether they're still relevant.

- **13 inherited "ignored upstream in Scala" parity stubs** (one-line `panic!("Unmigrated test: name")` bodies). Spread across `after_regions_integration_tests.rs`, `after_regions_tests.rs`, and `after_regions_error_tests.rs`. Whether to migrate or drop depends on whether the upstream Scala suite eventually unignores them.

- **4 tracked Rust-side blockers** with real bodies that are wired up to specific compiler bugs:
  - `panic_in_expr` — CoordSendSR Some-receiver solver-conflict (see `investigations/coord_send_some_branch_fix.md`).
  - `upcasting_in_a_generic_function` — pending CoordT redesign (place a placeholder in CoordT and move Ownership to a generic param so the return type's ownership is calculated from the parameter).
  - `cant_make_weak_ref_to_non_weakable` — Rust typing pass returns `Ok` where Scala throws `TookWeakRefOfNonWeakableError` for `&&m` on a non-weakable struct.
  - `reports_when_ownership_doesnt_match` — unmigrated; typing-pass body still needs porting.

- **2 Phase 4e parser bugs:**
  - `func_with_func_bound_with_missing_where` — Rust parser produces `TopLevelFunction` instead of `ParseError::FuncBoundWithoutWhere` for `func sum<T>() func moo(&T)void {3}`.
  - `report_leaving_out_semicolon_or_ending_body_after_expression_for_paren` — Rust parser produces `Consecutor(...)` instead of `ParseError::BadStartOfStatementError` for `set x = 7 )`.

`FrontendRust/src/parsing/tests/functions/after_regions_function_tests.rs` is emptied on `experimental-1` because the lone parser-bug test was its only resident; the restore commit on `after-overhaul-tests` re-adds the full file.

Workflow: when you fix one of the underlying issues, cherry-pick the corresponding test back onto `experimental-1` (or its current side-branch) and drop the entry here.
