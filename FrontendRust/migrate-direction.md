# Migration Direction: type_simple_main_function

## Failing Test
`higher_typing::tests::higher_typing_pass_tests::type_simple_main_function`

Panics with: `"Unimplemented: translate_function"` at `src/higher_typing/higher_typing_pass.rs:822`

## Diagnosis

The test calls into `translate_program` (already migrated at line 916), which iterates over all functions and calls `translate_function` (line 821). This function is a `panic!` stub.

### Chain of stubs that need migration (in call order):

1. **`translate_function`** (line 821-823) — stub, needs full migration. The Scala code (lines 825-869) shows it should:
   - Destructure the `FunctionS` into its parts
   - Create a `runeTypingEnv` that delegates lookups to `lookupType`
   - Call `calculateRuneTypes` to get rune-to-type mapping
   - Call `explicifyLookups` to make implicit coercions explicit
   - Return a `FunctionA` with all the data plus `UserFunctionS` attribute appended

2. **`calculate_rune_types`** (line 873-875) — stub, needs migration. The Scala code (lines 877-912) shows it should:
   - Create a `runeTypingEnv` similar to translate_function
   - Compute `runeSToPreKnownTypeA` from explicit types + param coord rune types
   - Call `RuneTypeSolver.solve` to infer all rune types
   - Return the solved rune-to-type map

3. **`explicify_lookups`** (line 143-145) — stub, needs migration. The Scala code (lines 147+) shows it should:
   - Iterate over rules, replacing implicit coercing lookups with explicit coercion rules
   - Handle `MaybeCoercingCallSR` and `MaybeCoercingLookupSR` cases

### What needs to happen for the test to pass:
1. Implement `translate_function` (first panic hit)
2. Implement `calculate_rune_types` (called by translate_function)
3. Implement `explicify_lookups` (called by translate_function)
