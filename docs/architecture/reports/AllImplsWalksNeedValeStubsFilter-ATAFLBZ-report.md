# Accuracy report: All-impls Walks Need Vale-Stubs Filter (ATAFLBZ)

Audited against the working tree on 2026-09-06. Arcana section: docs/architecture/vale-rust-interop-architecture.md:3406 ("### 26.13.5 ATAFLBZ (All-impls Walks Need Vale-Stubs Filter)")

## Verdict

Major inaccuracy: the section prescribes a fix (`is_from_vale_stubs` filter on `tcx.all_impls(trait_def_id)` walks) that does not exist anywhere in the current codebase, and per the same document's own later section (§26b.6, lines 3626-3651) was never the real fix — the actual resolution was deleting the two offending name-matching functions (`resolve_method`, `resolve_function`) and deriving identity from `tcx.def_path` for the one that remained (`package_coord_for`), backed by a grep-based lint (`no_rust_item_identity_comes_from_a_human_name` in src/typing/test/rust_interop/cases.rs:1847). The summary table entry at line 3344 repeats the same stale claim. All five cited code sites do exist and correctly reference the *general hazard* (identity from a human name), but none of them implement or mention `tcx.all_impls`/`is_from_vale_stubs` — that specific mechanism appears to be a superseded draft of the fix that was never updated in §26.13.5 itself.

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `tcx.all_impls(trait_def_id)` walks return impls from every crate including std | FALSE (no longer applicable) | No `all_impls` call exists in src/typing/rust_interop/*.rs (grep returns nothing) | No such walk exists in the current design; identity hazard instead arose from `resolve_method`/`resolve_function` string-matching and `tcx.crates(())` iteration (src/typing/rust_interop/oracle.rs:104-110, corpus.rs:1300-1304) |
| 2 | Self-type-name check is ambiguous since std and Vale could both define e.g. `Box` | DRIFTED | src/typing/rust_interop/corpus.rs:1300-1304, tyctxt_oracle.rs:82 | The general hazard (name collisions across crates) is real and still documented, but not tied to an "all_impls walk" or self-type check specifically |
| 3 | Fix: add `is_from_vale_stubs(tcx, adt_def.did())` filter inside impl walks | FALSE | grep for `is_from_vale_stubs` across src/ returns nothing; docs/architecture/vale-rust-interop-architecture.md:3649 says "its real fix was deleting the two functions that turned a name string into identity, not the lint added afterwards" | Replace with: delete name-matching functions; derive identity via `tcx.def_path` (tyctxt_oracle.rs:78-99 `package_coord_for`); backstop with grep lint (test/rust_interop/cases.rs:1847-1904) |
| 4 | Under single-symbol architecture, wrong DefId → wrong rustc-mangled name when Vale's bitcode emits a body | DRIFTED | src/typing/rust_interop/oracle.rs:104-110 ("a wrong `DefId` eventually drives a wrong mangled symbol") | Core consequence (wrong DefId → wrong mangled symbol) still holds and is stated in current code comments, just not framed around the (nonexistent) all_impls filter |

## Stale citation sites

None of the 5 cited sites are individually wrong — they all correctly reference @ATAFLBZ as the general "identity from human name" hazard. The staleness is in the arcana section's own text (3406-3409) and the table row (3344), which describe a specific mechanism (`all_impls` + `is_from_vale_stubs`) that doesn't match what any cited site actually shows.

## Uncited sites that embody the arcana

None found beyond the 5 already cited — `package_coord_for` (tyctxt_oracle.rs:78) and the lint test (cases.rs:1847) already carry @ATAFLBZ references.

## Suggested rewrite

> ### 26.13.5 ATAFLBZ (Rust Item Identity Must Not Come From a Human Name)
>
> Rust has no uniqueness rule for short names (`new`, `len`, `Box`, `Widget`, ...), and rustc APIs like `tcx.crates(())` hand back every loaded crate, so matching a Rust item by name string picks whichever crate's item is encountered first. A `DefId` chosen this way eventually drives a wrong rustc-mangled symbol — the failure surfaces as a link error against a plausible-looking name, far from the mistake.
>
> Two oracle methods (`resolve_method`, `resolve_function`) did this and were deleted outright once their last callers (the per-call-site query seam) were retired. The one surviving name-derived path, `package_coord_for`, now keys on `tcx.def_path(def_id)` instead — unique by construction, so two crates each exporting `Widget` land in different packages. A grep-based lint (`no_rust_item_identity_comes_from_a_human_name`, src/typing/test/rust_interop/cases.rs:1847) guards against a name-comparison creeping back into `rust_interop/`; deleting the unrepresentable API was the real fix, the lint is only the backstop for the next occurrence.

Also update the summary table row at docs/architecture/vale-rust-interop-architecture.md:3344 from `ATAFLBZ | Walks of tcx.all_impls(...) filter by is_from_vale_stubs(self_type_did).` to something like `ATAFLBZ | Rust item identity must come from DefId/def_path, never a human name string.`
