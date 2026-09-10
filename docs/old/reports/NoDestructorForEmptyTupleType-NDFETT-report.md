# Accuracy report: No Destructor For Empty Tuple Type (NDFETT)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Templar/Templar.md:524-534

## Verdict
This is a historical design note (old Templar-era musing) about how to handle the destructor of the empty-tuple type `[]`, resolved by deciding to have no destructor for it at all — and the note itself already appends "**Note from later:** This was repealed. We now have a specific VoidT kind." Current code confirms the repeal: `KindT::Void(VoidT)` is a first-class kind (src/typing/types/types.rs:54,136), constructed explicitly in the compiler (src/typing/compiler.rs:829) and in the Rust-interop importer/oracle (src/typing/rust_interop/tyctxt_oracle.rs:512, src/typing/rust_interop/importer.rs:289), so `()`/`[]` gets its own kind rather than being a destructor-less special case. There are no live code citations of NDFETT. Since the note is self-aware of its own obsolescence and the code has moved on entirely, this is not a candidate for migration — recommendation: leave it in docs/old/ as historical record (it correctly documents its own repeal); do not migrate to docs/arcana and do not delete.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Every destructor returns []." (old Templar-era rule) | UNVERIFIABLE (historical, Scala-era Templar) | n/a | Describes the old Scala Templar, no longer applicable; not checkable against current Rust typing pass. |
| 2 | "So what does []'s destructor return? ... we just don't have a destructor for it." (the decision) | OBSOLETE (superseded, and the doc says so itself) | src/typing/types/types.rs:54,136; src/typing/compiler.rs:829 | Empty tuple / `()` now lowers to `VoidT`, a real kind, not an exemption from having a destructor. The doc's own "Note from later" already states this. |

## Stale citation sites
None — no code cites NDFETT (grep across src/, Backend/, docs/ excluding docs/old found zero matches).

## Uncited sites that embody the arcana
None found — the current design (VoidT as a first-class kind) doesn't correspond to "no destructor for X"; it's a different, resolved approach, so there's no code embodying the *old* NDFETT concern to cite.

## Suggested text
Not applicable (D3 kind, verdict is not major-inaccuracies/obsolete-requiring-rewrite — the section is a resolved historical note that already documents its own repeal; no corrected arcana text is warranted).
