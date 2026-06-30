# Slice Pipeline Examples

## Example 1: Fresh File with Only Scala Comments

**Input:** `src/postparsing/new_feature.rs` with only commented Scala code

**Steps:**
1. `/slice-pipeline src/postparsing/new_feature.rs`
2. Pipeline runs: start → rustify → placehold
3. Skips reconciliation (no pre-existing Rust code)

**Result:** Rust placeholder stubs ready for implementation

## Example 2: File with Mixed Old and New Code

**Input:** `src/postparsing/names.rs` with:
- Commented Scala code
- Old Rust implementations (written before slicing)

**Steps:**
1. `/slice-pipeline src/postparsing/names.rs`
2. Pipeline runs: start → rustify → placehold → reconcile-mark → reconcile-copy → reconcile-delete
3. Old implementations merged into new stubs
4. Old code removed

**Result:** Consolidated Rust code with all implementations under `// mig:` markers

## Example 3: Incremental Update

**Input:** File already sliced, need to add one new Scala function

**Steps:**
1. Add commented Scala code to file
2. `/slice-start` - adds `// mig:` marker for new function only
3. `/slice-rustify` - converts marker
4. `/slice-placehold` - generates stub
5. Implement manually or use `/migration-migrate`

**Result:** New function integrated into existing sliced structure

## Common Patterns

### Pattern: Function with Complex Lifetimes
```rust



fn postparse_closure<'a, 'p, 's>(
    &mut self,
    closure_p: &'p ClosureP<'a, 'p>,
) -> &'s ClosureSE<'a, 's> {
    panic!("TODO: migrate from Scala")
}
```

### Pattern: Struct Definition
```rust


pub struct CodeBodyS<'a, 's> {
    pub body_location: CodeLocationS<'a>,
    pub body: &'s BlockSE<'a, 's>,
}
```

### Pattern: Enum Variant
```rust


Consecutor(ConsecutorSE<'a, 's>),
```

## Troubleshooting

**Issue:** Reconciliation incorrectly matches functions
- **Fix:** Check function signatures match exactly (name + param count)
- **Workaround:** Temporarily rename old function, run pipeline, merge manually

**Issue:** Placeholder has wrong lifetimes
- **Fix:** Check the `// mig:` comment annotations
- **Prevention:** Use `/slice-rustify` to ensure Scala types → Rust types correctly

**Issue:** Build fails after pipeline
- **Cause:** Stubs use `panic!()` - expected during migration
- **Next step:** Use `/migration-migrate` to implement the panicking functions
