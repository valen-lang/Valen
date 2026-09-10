# Accuracy report: Rust Resolver Precedes Generic Resolver (RRPGRZ)

Audited against the working tree on 2026-09-06. Source: git history, `82f9ec4b^:docs/arcana/RustResolverPrecedesGenericResolver-RRPGRZ.md` (deleted by commit 82f9ec4b)

## Verdict
The recovered text describes a Scala-era ordering bug in `Frontend/PassManager/src/dev/vale/passmanager/PassManager.scala`'s `.or(...)` resolver chain, specific to the abandoned rust-interop prototype (ValeRuster, `resolveRustPackageContents`, `import rust.X.Y.Z` synthesis). Commit 82f9ec4b deleted this exact arcana file in the same commit that removed `PassManager.scala`'s rust hooks, `resolveRustPackageContents`, `resolvePackageContents`'s rust-specific chain wiring, and the whole `ValeRuster/` crate — confirmed directly in that commit's message and stat. None of `PassManager`, `resolvePackageContents`, `resolveRustPackageContents`, or `ValeRuster` exist anywhere in the current tree. The unrelated `src/typing/rust_interop/` module present today is a different, later rust-interop reimplementation with no package-coordinate resolver chain of this shape. This is obsolete: the mechanism, the file it lived in, and the surrounding subsystem are all gone. Recommendation: delete (no citing comments exist to clean up — `sites` is empty).

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `PassManager.build` chains package-content resolvers via `.or(...)`, including a Rust-bindings resolver and `resolvePackageContents` | FALSE (obsolete) | commit 82f9ec4b message: "Frontend/PassManager/src/dev/vale/passmanager/PassManager.scala — removed ... resolveRustPackageContents fn, call site, and resolver-chain .or wiring" | File and mechanism deleted entirely; no `PassManager.scala` or equivalent Rust chain exists in the current tree. |
| 2 | `resolvePackageContents` returns `Some(Map.empty)` instead of `None` for synthesized `rust.*` packages, short-circuiting the `.or` chain | UNVERIFIABLE / OBSOLETE | grep found no `resolvePackageContents` anywhere in the tree | Function no longer exists; behavior can't be checked against current code. |
| 3 | The fix is to chain the Rust resolver before the generic one | OBSOLETE | same commit removed all three: the Rust resolver, the generic resolver's rust-specific short-circuit, and the ordering itself | No longer applicable — there is no resolver chain of this kind. |

## Stale citation sites
None — `sites` is empty; no code or docs currently cite RRPGRZ (confirmed via `grep -rlw "RRPGRZ" src/ Backend/ docs/`, which found nothing but this session's own convo log).

## Uncited sites that embody the arcana
None found. The concern (package-coordinate resolver ordering for synthesized rust bindings) has no analog in the current architecture — there is no `PassManager`, no rust-bindings package-content resolver, and no ValeRuster-style synthesis of `rust.*` coordinates in `src/typing/rust_interop/` (that module does direct rustdoc/crate introspection, not a resolver-chain pattern).

## Suggested text
Not applicable in the modern style — this documented a one-off ordering bugfix in a Scala resolver chain that was deleted wholesale along with the subsystem it patched. There is no durable, still-relevant concern here to carry forward into docs/arcana/; recommend deletion of the historical record rather than migration.
