# Accuracy report: DefPathStr Is For Diagnostics Only (DPSFDOZ)

Audited against the working tree on 2026-09-06. Arcana section: docs/architecture/vale-rust-interop-architecture.md:3362 ("### 26.3 DPSFDOZ (DefPathStr Is For Diagnostics Only)")

## Verdict
The core rule (avoid `tcx.def_path_str`, use `def_path(...)`/`crate_name` instead) is TRUE and actively followed in src/typing/rust_interop/tyctxt_oracle.rs. But the section's third sentence — `is_from_vale_stubs(tcx, def_id)` uses marker-detection — names a function that does not exist anywhere in the codebase. The real, analogous function is `is_vale_codegen_target` in src/instantiating/rust_interop/mod.rs, it lives in a different pass (instantiating, not typing/rust_interop as the section implies), and it gates on the `#[vale::emit_consumer_body]` attribute rather than on `__VALE_STUBS_MARKER` "marker-detection" — the code's own doc comment explicitly says the marker-crate check was dropped in favor of the per-item attribute. This is a fabricated/stale detail riding along with an otherwise-accurate rule.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `tcx.def_path_str(def_id)` ICEs outside diagnostic contexts | TRUE | rustc-internals fact, corroborated by src/typing/rust_interop/tyctxt_oracle.rs:229-230 doc comment | — |
| 2 | Vale's path-based matching uses `tcx.def_path(def_id).data` walks or `tcx.crate_name(def_id.krate)` checks | TRUE | src/typing/rust_interop/tyctxt_oracle.rs:100 (`.def_path(def_id)`), :107 and :185 (`tcx.crate_name(...)`) | — |
| 3 | `is_from_vale_stubs(tcx, def_id)` uses marker-detection | FALSE | No function named `is_from_vale_stubs` exists anywhere in src/. The analogous function is `is_vale_codegen_target` at src/instantiating/rust_interop/mod.rs:704-711, which checks the `#[vale::emit_consumer_body]` attribute via `tcx.has_attrs_with_path`, not marker detection on `__VALE_STUBS_MARKER` (its own doc comment says the marker-crate check is part of "the full design" but isn't what this function does) | Rename to `is_vale_codegen_target(tcx, def_id)` (src/instantiating/rust_interop/mod.rs) and describe it as an attribute check (`#[vale::emit_consumer_body]`), not marker-detection. |

## Stale citation sites
src/typing/rust_interop/tyctxt_oracle.rs:231 — cites @DPSFDOZ correctly for the def_path_str-avoidance rule; this site itself is accurate and not stale. No stale sites for claims 1-2.

## Uncited sites that embody the arcana
- src/instantiating/rust_interop/mod.rs:704 (`is_vale_codegen_target`) — the actual function the section's claim 3 is trying to describe; uncited and misnamed in the doc.
- src/typing/rust_interop/tyctxt_oracle.rs:100, :107, :185 — additional `def_path`/`crate_name` uses beyond the cited line 231 that could also carry the citation but don't need to (one citation per file is typical).

## Suggested rewrite
`tcx.def_path_str(def_id)` ICEs outside diagnostic contexts. Vale's path-based matching uses `tcx.def_path(def_id).data` walks or `tcx.crate_name(def_id.krate)` checks. `is_vale_codegen_target(tcx, def_id)` (src/instantiating/rust_interop/mod.rs) uses attribute-detection (`#[vale::emit_consumer_body]`), not `def_path_str`.
