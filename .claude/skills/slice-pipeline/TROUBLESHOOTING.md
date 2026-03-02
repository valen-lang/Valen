# Slice Pipeline Troubleshooting

## Pipeline Failures

### Step 1 (slice-start) Fails

**Symptom:** Can't identify Scala definitions

**Causes:**
- Scala code formatting is non-standard
- Missing comment markers (`//` at line start)
- Nested class/object definitions confuse parser

**Fix:**
- Manually add `// mig: def functionName` markers
- Ensure Scala code has standard indentation
- Skip slice-start and write markers by hand

### Step 2 (slice-rustify) Produces Wrong Types

**Symptom:** Generated Rust types don't match actual types needed

**Causes:**
- Scala→Rust type mapping incomplete
- Generic parameters lost
- Lifetime annotations missing

**Fix:**
- Check `.claude/rules/postparser_impl_scala_rust_mapping.mdc`
- Manually correct `// mig:` annotations
- Add lifetime bounds: `'a`, `'p`, `'s` per the lifetime rules

### Step 3 (slice-placehold) Creates Duplicates

**Symptom:** Two stubs for the same function

**Causes:**
- Multiple `// mig:` markers for same function
- Old stub wasn't deleted before re-running

**Fix:**
- Remove duplicate `// mig:` markers
- Delete old stubs manually
- Re-run from clean state

### Reconciliation Steps (4-6) Fail

**Symptom:** Old code not matched/copied correctly

**Causes:**
- Function signature mismatch (name or param count differs)
- Old implementation uses different lifetimes
- Impl block vs free function mismatch

**Fix:**
- Verify old and new signatures match exactly
- Update old function to match new signature
- Use manual copy if automatic reconciliation fails

## Build Failures After Pipeline

### Lifetime Errors

**Symptom:** rustc complains about lifetime bounds

**Fix:**
- Read `.claude/rules/postparser/postparser-lifetimes.mdc`
- Don't accept rustc suggestions blindly
- Ensure `'a: 'p`, `'a: 's`, `'a: 'ctx` where needed

### Missing Imports

**Symptom:** Type not found errors

**Fix:**
- Add `use` statements at top of file
- Check what the Scala code imported
- Look at other postparser files for patterns

### Borrow Checker Errors

**Symptom:** "cannot borrow as mutable while borrowed"

**Common Cause:** Mixing `&self` and `&mut self` incorrectly

**Fix:**
- PostParser methods typically use `&mut self`
- Don't hold references across `self.scout_arena.alloc()` calls
- Use `&*self.scout_arena.alloc(...)` pattern for immediate re-borrow

## Migration Workflow Issues

### When to Use slice-pipeline vs migration-migrate

**Use slice-pipeline when:**
- Starting fresh migration of a file
- File has many functions to migrate at once
- Want systematic, automated translation

**Use migration-migrate when:**
- Fixing one specific function
- Debugging build errors
- Incremental refinement after pipeline

### Pipeline Leaves File in Bad State

**Recovery:**
1. `git diff src/postparsing/problematic.rs` - see what changed
2. `git restore src/postparsing/problematic.rs` - revert
3. Identify which step failed
4. Run steps manually one at a time
5. Inspect output after each step

### Markers Out of Sync with Code

**Symptom:** `// mig:` comment says one thing, stub does another

**Cause:** Manual edits broke synchronization

**Fix:**
- Delete the stub
- Keep the `// mig:` marker
- Re-run `/slice-placehold` to regenerate stub
- Or update `// mig:` marker to match current stub

## Best Practices

1. **Always commit before running pipeline** - makes rollback easy
2. **Run one file at a time** - easier to debug issues
3. **Verify each step** - don't blindly run full pipeline if unsure
4. **Check git diff** - review changes before committing
5. **Run `cargo check`** - catch errors early before full build
