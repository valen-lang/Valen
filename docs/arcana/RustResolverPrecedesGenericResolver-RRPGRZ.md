# Rust Resolver Precedes Generic Resolver (RRPGRZ)

When the Frontend's `PassManager.build` chains its package-content resolvers, the Rust-bindings resolver must come **before** the generic `resolvePackageContents`, not after.

## Why

`resolvePackageContents` is the catch-all that walks the user's input set. For a `rust.foo.bar` package coordinate the user never wrote (it's synthesized from the `import rust.foo.bar` regex pre-scan), the input set has no matching files — but the way `resolvePackageContents` is written, it returns `Some(Map.empty)` rather than `None` for any package whose module appears in the input set after we register `rust.*` synthetically. That `Some(Map.empty)` short-circuits the `.or(...)` chain, and the Rust resolver below it never gets called.

The fix is structural rather than tweaking `resolvePackageContents`'s contract: chain the Rust resolver first.

```scala
Builtins.getCodeMap(interner, keywords)
  .or(packageCoord => resolveRustPackageContents(rustBindingsDir, packageCoord))
  .or(packageCoord => resolvePackageContents(interner, allInputs, packageCoord)),
```

The Rust resolver itself returns `None` for non-`rust` modules, so it's a no-op for normal user packages. It returns `None` (rather than `Some(Map.empty)`) for unknown rust packages, so the chain falls through correctly to the generic resolver if ValeRuster didn't produce bindings for that path.

## When this matters

Only when `--vale_ruster_path` etc are set and the program contains `import rust.X.Y.Z`. In all other configurations the Rust resolver is `None` and adding it to the chain costs nothing.
