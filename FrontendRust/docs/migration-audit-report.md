# Migration Audit Report — Full Parity Check

**Date:** 2026-03-11
**Branch:** rustmigrate-z
**Auditor:** migration-check-specific agents (haiku model, read-only)
**Scope:** 30 definitions across 7 files

---

## Summary

| Verdict | Count |
|---------|-------|
| APPROVED | 4 |
| NEEDS_WORK | 26 |
| Total | 30 |

---

## APPROVED Definitions

### 1. `coerce_kind_lookup_to_coord` — higher_typing_pass.rs
**Verdict: APPROVED**

The function exactly mirrors the Scala logic. No novel functions, correct structure and flow, same rules created (Lookup, Call). Variable naming is appropriate with suffixes. Function-local imports are acceptable. The function signature and all call sites are correct. No MACT violations (no comments in the Scala function body to migrate).

### 2. `coerce_kind_template_lookup_to_kind` — higher_typing_pass.rs
**Verdict: APPROVED**

No novel functions — the function exists in Scala. Same structure and flow. Calls the same functions via interning. RSMSCP satisfied — mirrors Scala exactly (except for necessary interning). No TODOs or unimplemented panics. No changed requirements. Scala code preserved as comments. Clear variable suffixes. The `ImplicitCoercionTemplateRuneS` Val struct correctly follows the IDEPFL shallow pattern.

### 3. `test_infer_pack_from_empty_result` — higher_typing_pass_tests.rs
**Verdict: APPROVED**

The test structure mirrors Scala — compile, expect_astrouts, get program, lookup function, assert rune type. Same logical steps with appropriate Rust idioms. Uses `.unwrap()` correctly per TPUTEFC. No conditionals in the test (NHCIT). The `IRuneValS::CodeRune` construction with `interner.intern("P")` correctly matches Scala's `CodeRuneS(compilation.interner.intern(StrI("P")))`. Test expectations identical to Scala. Order of operations matches.

### 4. `FuncPT` translation block — templex_scout.rs (~line 550)
**Verdict: APPROVED**

The code correctly translates `FuncPT` patterns. Parameter iteration, return type translation, and rune creation all match the Scala structure. Uses `lidb.child()` correctly for location tracking. The `translate_templex` calls pass correct argument types. No novel logic added.

---

## NEEDS_WORK Definitions

---

### File: `higher_typing_pass.rs`

---

#### 5. `explicify_lookups` (lines 177-250)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — structural control flow mismatch**

The `MaybeCoercingCallSR` case uses an if-else structure followed by a match statement, but the Scala code uses a single match statement with a guard clause.

**Scala code (lines 270-282):**
```scala
(actualType, expectedType) match {
  case (x, y) if x == y => {
    ruleBuilder += CallSR(range, resultRune, templateRune, args)
  }
  case (KindTemplataType(), CoordTemplataType()) => {
    val kindRune = RuneUsage(range, ImplicitCoercionKindRuneS(range, resultRune.rune))
    runeAToType.put(kindRune.rune, KindTemplataType())
    ruleBuilder += CallSR(range, kindRune, templateRune, args)
    ruleBuilder += CoerceToCoordSR(range, resultRune, kindRune)
  }
  case _ => vimpl()
}
```

**Rust code (lines 183-198):**
```rust
if actual_type == expected_type {
  rule_builder.push(IRulexSR::Call(CallSR { range, result_rune, template_rune, args }));
} else {
  match (&actual_type, &expected_type) {
    (ITemplataType::KindTemplataType(_), ITemplataType::CoordTemplataType(_)) => { ... }
    _ => panic!("vimpl"),
  }
}
```

**Fix:** Replace the if-else + match with a single match using a guard:
```rust
match (&actual_type, &expected_type) {
  (x, y) if x == y => { ... }
  (ITemplataType::KindTemplataType(_), ITemplataType::CoordTemplataType(_)) => { ... }
  _ => panic!("vimpl"),
}
```

---

#### 6. `imprecise_name_matches_absolute_name` (lines ~480-530)
**Verdict: NEEDS_WORK**
**Violations: RSMSCP — split match arm; MACT — missing comments**

**Issue 1 — Split match arm:** The Scala version has a single match arm for `TopLevelCitizenDeclarationNameS(humanNameA, _)` that matches both struct and interface variants via a shared sealed trait pattern. The Rust version splits this into two separate match arms:

```rust
(IImpreciseNameS::CodeName(code_name), INameS::TopLevelStructDeclaration(s)) => {
  s.name == code_name.name
}
(IImpreciseNameS::CodeName(code_name), INameS::TopLevelInterfaceDeclaration(i)) => {
  i.name == code_name.name
}
```

While functionally equivalent, this doesn't mirror the Scala structure. The Rust version should use a single match arm with a combined pattern or a helper that extracts the name from both variants.

**Issue 2 — Missing comments:** The Scala comment `// Returns whether the imprecise name could be referring to the absolute name.` and `// See MINAAN for what we're doing here.` are not migrated to the Rust version.

---

#### 7. `lookup_types` (lines 544-579)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — if-let chains instead of match statements**

The Scala code uses three separate match statements on the input:
1. First match validates input type without binding
2. Second match matches `CodeNameS` and performs primitives lookup with a nested match
3. Third match matches `RuneNameS` and performs `rune_to_type` lookup with a nested match

The Rust code uses one match statement for validation, then two if-let chains:
```rust
if let IImpreciseNameS::CodeName(code_name) = needle_imprecise_name_s {
  if let Some(x) = self.primitives.get(&code_name.name) {
    return vec![...];
  }
}
```

**Fix:** Use three separate match expressions like Scala, not if-let chains.

**Additional issue:** Lines 564 and 568 wrap `s.tyype` (a `TemplateTemplataType`) in `ITemplataType::TemplateTemplataType(...)`. This may indicate a type mismatch in `CitizenRuneTypeSolverLookupResult` (it should accept `TemplateTemplataType`, not `ITemplataType`).

---

#### 8. `lookup_type` (lines ~580-620)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — match on len() instead of slice patterns**

The Scala version uses pattern matching directly on the `.distinct()` result:
```scala
lookupTypes(astrouts, env, name).distinct match {
  case Vector() => Err(CouldntFindTypeA(range, name))
  case Vector(only) => Ok(only)
  case others => Err(TooManyMatchingTypesA(range, name))
}
```

The Rust version decouples deduplication from matching and matches on `distinct.len()`:
```rust
let mut distinct = Vec::new();
for r in results {
  if !distinct.contains(&r) { distinct.push(r); }
}
match distinct.len() {
  0 => Err(...),
  1 => Ok(distinct.into_iter().next().unwrap()),
  _ => Err(...),
}
```

**Fix:** Use Rust's slice pattern matching:
```rust
match distinct.as_slice() {
  [] => Err(CouldntFindTypeA { ... }),
  [only] => Ok(only.clone()),
  _ => Err(TooManyMatchingTypesA { ... }),
}
```

---

#### 9. `translate_struct` (lines 669-797)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — missing `MaybeCoercingCallSR` checks**

After `explicify_lookups` and before constructing `StructA::new`, the Scala code (lines 882-887) checks that no `MaybeCoercingCallSR` rules remain in either `headerRulesExplicitS` or `memberRulesExplicitS`:

```scala
headerRulesExplicitS.collect({ case x @ MaybeCoercingCallSR(_, _, _, _) => vwat() })
memberRulesExplicitS.collect({ case x @ MaybeCoercingCallSR(_, _, _, _) => vwat() })
```

The Rust code (after line 778, before line 780) has no such check.

**Fix:** Add:
```rust
for rule in header_rules_builder.iter() {
  match rule {
    IRulexSR::MaybeCoercingCall(_) => panic!("vwat"),
    _ => {}
  }
}
for rule in member_rules_builder.iter() {
  match rule {
    IRulexSR::MaybeCoercingCall(_) => panic!("vwat"),
    _ => {}
  }
}
```

---

#### 10. `translate_interface` (lines 930-1080)
**Verdict: NEEDS_WORK**
**Violations: 5 issues found**

**Issue 1 — MACT:** Missing comment `"// Weird because this means we already evaluated it, in which case we should have hit the above return"` (Scala line 1046).

**Issue 2 — MACT:** Missing comment block about LookupSR rules being loose (Scala lines 1076-1079) explaining the need for explicit coercion. Should appear before `rule_builder` creation at line 996.

**Issue 3 — RSMSCP (cache hit):** Scala returns the cached value on cache hit (line 1041: `case Some(value) => return value`), but Rust panics (lines 941-943: `panic!("translate_interface: cache hit not yet supported")`). The Rust version needs to actually return the cached interface.

**Issue 4 — DCCR/RSMSCP (missing cache insert):** The computed `interface_a` is never stored back into the cache. Scala does `astrouts.codeLocationToInterface.put(rangeS.begin, interfaceA)` at line 1104 before returning, but Rust never inserts it. This breaks the caching mechanism.

**Issue 5 — RSMSCP (runeTypingEnv shape):** Scala creates an anonymous class with error-transforming lookup logic (lines 1027-1038), while Rust creates a simple struct (lines 989-994) and passes `self.interner` as a separate parameter. The `explicify_lookups` call signature differs: Scala passes `(runeTypingEnv, runeAToType, ruleBuilder, rulesWithImplicitlyCoercingLookupsS)`, but Rust passes `(&rune_typing_env, self.interner, &mut rune_a_to_type, &mut rule_builder, ...)`.

---

#### 11. `translate_impl` (lines ~1100-1200)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — statement order differs**

In Scala (line 504-515), `runeTypingEnv` is created immediately after destructuring, before calling `calculateRuneTypes` (line 517). In Rust, `rune_typing_env` is created much later (line 1149), after the HashMap creation (lines 1142-1143).

This is a systematic pattern across all `translate_*` functions — the Rust version lazily initializes `rune_typing_env` just before use, while Scala initializes it early.

Per RSMSCP and the migration principle "Port the structure exactly as it is in Scala", the statement order should match. Additionally, kind types should be added inline during the `calculate_rune_types` call (Scala line 522) rather than pre-computed separately (Rust lines 1124-1126).

---

#### 12. `translate_function` (lines 1292-1450)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — error handling panics instead of matching specific variants**

The Rust code at lines 1328-1337 does not properly handle errors from `explicify_lookups`. The Scala version (lines 1383-1387) pattern matches on specific error variants:
- `Err(RuneTypingTooManyMatchingTypes(...))` → throws `TooManyMatchingTypesA`
- `Err(RuneTypingCouldntFindType(...))` → throws `CouldntFindTypeA`

The Rust code has a catch-all `Err(_e) => panic!("explicify_lookups failed")` which loses all error information.

**Fix:** Use proper pattern matching:
```rust
Err(e) => match e {
  IRuneTypingLookupFailedError::TooManyMatchingTypes(t) => { /* create TooManyMatchingTypesA */ }
  IRuneTypingLookupFailedError::CouldntFindType(c) => { /* create CouldntFindTypeA */ }
}
```

---

#### 13. `calculate_rune_types` (lines 1404-1443)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — missing `use_optimized_solver` parameter**

The Scala call (line 1472):
```scala
new RuneTypeSolver(interner).solve(
  globalOptions.sanityCheck,
  globalOptions.useOptimizedSolver,  // <-- MISSING FROM RUST
  runeTypingEnv,
  List(rangeS),
  false, rulesS, identifyingRunesS, true, runeSToPreKnownTypeA)
```

The Rust call (lines 1429-1438):
```rust
rune_type_solver.solve_rune_type(
  self.global_options.sanity_check,
  // <-- use_optimized_solver MISSING HERE
  &rune_typing_env,
  vec![range_s.clone()],
  false,
  rules_s,
  &identifying_runes_s,
  true,
  rune_s_to_pre_known_type_a,
)
```

The `use_optimized_solver` parameter determines whether to use `OptimizedSolverState` or `SimpleSolverState` (Scala Solver.scala lines 100-105). This functionality appears to be entirely missing from the Rust `Solver` implementation.

---

#### 14. `translate_program` (lines 1484-1520)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — missing `primitives` parameter**

The Scala signature (line 1523-1526) includes `primitives: Map[StrI, ITemplataType]` as a parameter:
```scala
def translateProgram(
  codeMap: PackageCoordinateMap[ProgramS],
  primitives: Map[StrI, ITemplataType],  // <-- MISSING
  suppliedFunctions: Vector[FunctionA],
  suppliedInterfaces: Vector[InterfaceA]): ProgramA
```

The Rust version completely omits this parameter. While `primitives` is not used in the function body in either version, it must still be present in the signature to match the Scala interface and support callers like `run_pass` which pass this parameter.

---

#### 15. `run_pass` (lines 1556-1638)
**Verdict: NEEDS_WORK**
**Violations: 3 issues**

**Issue 1 — Missing `primitives` in `translate_program` call (line 1589):** The Rust version calls `self.translate_program(merged_program_s, supplied_functions, supplied_interfaces)` but the Scala version (line 1679) calls `translateProgram(mergedProgramS, primitives, suppliedFunctions, suppliedInterfaces)`.

**Issue 2 — Incomplete `translate_program` signature:** As noted above, the `primitives` parameter is missing.

**Issue 3 — Missing error handling:** The Scala version uses try/catch (lines 1674-1710) to wrap potential `CompileErrorExceptionA` exceptions. While the Rust version has a `Result` return type, there's no explicit error handling in the function body matching the Scala structure.

---

#### 16. `get_astrouts` (lines ~1640-1680)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — control flow structure mismatch**

The Scala code uses explicit nested `match` statements:
1. First `match` on `astroutsCache` (Option type) with `Some`/`None` branches
2. Second `match` on the result of `runPass` (Either type with `Left`/`Right` variants)

The Rust code uses:
1. An `if is_some()` early-return pattern
2. Implicit error handling via the `?` operator

**Fix:** Use explicit match statements to preserve the exact control flow structure:
```rust
match self.astrouts_cache {
  Some(ref cached) => cached.clone(),
  None => {
    match self.run_pass() {
      Ok(result) => { ... }
      Err(e) => { ... }
    }
  }
}
```

---

### File: `higher_typing_pass_tests.rs`

---

#### 17-21. All 5 test functions (except `test_infer_pack_from_empty_result`)
**Verdict: NEEDS_WORK (all same issue)**

Affected tests:
- `infer_coord_type_from_parameters` (lines 114-131)
- `template_call_recursively_evaluate` (lines 180-197)
- `infer_generic_type_through_param_type_template_call` (lines 267-284)
- `test_evaluate_pack` (lines 301-320)
- `test_infer_pack_from_result` (lines 336-353)

**Violation: Style guide — excessive `crate::` paths**

All five tests use long fully-qualified paths in their assertion bodies:
- `crate::postparsing::names::IRuneValS::CodeRune(...)`
- `crate::postparsing::names::CodeRuneS { ... }`
- `crate::postparsing::itemplatatype::ITemplataType::CoordTemplataType(...)`
- `crate::postparsing::itemplatatype::ITemplataType::PackTemplataType(...)`
- `crate::postparsing::itemplatatype::CoordTemplataType {}`
- `crate::postparsing::itemplatatype::PackTemplataType { ... }`

Per the style guide: "Avoid `crate::` and long module paths in function bodies, type signatures, and match arms. Add the necessary `use` statements at the top of the file."

**Fix:** Add imports at the top of the file (after line 22):
```rust
use crate::postparsing::names::{IRuneValS, CodeRuneS};
use crate::postparsing::itemplatatype::{ITemplataType, CoordTemplataType, PackTemplataType};
```

Then use short names throughout all tests: `IRuneValS::CodeRune(CodeRuneS { name: keywords.t })` instead of the full path.

Note: The test logic itself is correct in all cases — same structure, same assertions, same expectations as Scala. The only issue is the import style.

---

### File: `rune_type_solver.rs`

---

#### 22. `solve_rune_type` (lines 995-1160)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — `additionalRunes` handling differs structurally**

The Scala code:
1. Computes `allRunesForSolver` **without** `additionalRunes` (line 1302): `(rules.flatMap(getRunes) ++ initiallyKnownRunes.keys).distinct.toVector`
2. Passes that to `Solver` constructor
3. After solving, **recomputes** `allRunes` by calling `solver.getAllRunes().map(solver.getUserRune) ++ additionalRunes` (line 1312)
4. Checks unsolved runes against the recomputed value (line 1313)

The Rust code:
1. Computes `all_runes` **with** `additionalRunes` included (lines 1095-1108)
2. Passes this combined value to `Solver::new` (line 1117)
3. After solving, **reuses** the same `all_runes` for checking unsolved runes (lines 1136-1139) — no recomputation from solver state

**Fix:**
1. Build initial runes set from `rules_s` and `initially_known_runes` only (without `additional_runes`)
2. Pass that to `Solver::new`
3. After solving, call solver methods to get final `all_runes` and add `additional_runes`
4. Use recomputed `all_runes` to determine unsolved runes

---

#### 23. `solve_rule` (lines 576-717)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — match arms in completely wrong order**

**Rust order:** CoordComponents, CoerceToCoord, Call, Literal, Equals, OneOf, IsInterface, Augment, Lookup, MaybeCoercingLookup, MaybeCoercingCall, RuneParentEnvLookup, Pack, CallSiteFunc, DefinitionFunc, Resolve, CoordSend, DefinitionCoordIsa, CallSiteCoordIsa, KindComponents, PrototypeComponents, IsConcrete, IsStruct, RefListCompoundMutability, IndexList

**Scala order:** KindComponents, CoordComponents, PrototypeComponents, MaybeCoercingCall, Resolve, CallSiteFunc, DefinitionFunc, DefinitionCoordIsa, CallSiteCoordIsa, OneOf, Equals, IsConcrete, IsInterface, IsStruct, RefListCompoundMutability, CoerceToCoord, Literal, Lookup, MaybeCoercingLookup, RuneParentEnvLookup, Augment, Pack

The entire match statement needs to be reordered to follow the Scala sequence exactly.

---

#### 24. `lookup_rune_type` (lines 897-940)
**Verdict: NEEDS_WORK**
**Violations: 3 issues — incomplete implementation**

**Issue 1 — Missing `Templata` case:** The Scala version (lines 379-391) has a full implementation with nested pattern matching on `(actualType, expectedType)` with three sub-cases and proper error returns. The Rust version (lines 922-923) simply panics: `panic!("lookup_rune_type Templata not yet migrated")`.

**Issue 2 — Missing error return in `Primitive` case:** The Scala version (line 376) returns `FoundPrimitiveDidntMatchExpectedType` error in the default case. The Rust version (line 919) panics instead.

**Issue 3 — Missing error return in `Citizen` case:** The Scala version (line 405) returns `FoundCitizenDidntMatchExpectedType` error in the default case. The Rust version (line 935) panics instead.

**Fix:** Implement the `Templata` case fully with nested pattern matching. Replace panics in `Primitive` and `Citizen` default branches with proper error returns using the existing error types.

---

#### 25. `get_puzzles_rune_type` (lines 433-486)
**Verdict: NEEDS_WORK**
**Violations: 2 issues**

**Issue 1 — Missing comments (MACT):** The Scala version contains explanatory comments throughout:
- "This Vector() means nothing can solve this puzzle"
- "Vector(Vector()) because we can solve it immediately"
- Comments about `initiallyKnownRunes`
- Comments about "needing to know the type beforehand"
- Comments explaining coercion logic
- "Packs being lists of coords"

The Rust version has stripped out ALL of these comments.

**Issue 2 — Wrong match arm order:** The Rust match arms are in a completely different order than Scala.

**Scala order:** Equals, Lookup, MaybeCoercingLookup, RuneParentEnvLookup, MaybeCoercingCall, Pack, DefinitionCoordIsa, CallSiteCoordIsa, KindComponents, CoordComponents, PrototypeComponents, Resolve, CallSiteFunc, DefinitionFunc, OneOf, IsConcrete, IsInterface, IsStruct, CoerceToCoord, Literal, Augment, RefListCompoundMutability

**Rust order:** Equals, MaybeCoercingLookup, Lookup, RuneParentEnvLookup, MaybeCoercingCall, CoordComponents, OneOf, IsInterface, CoerceToCoord, Call, Literal, Augment, Pack, DefinitionCoordIsa, CallSiteCoordIsa, KindComponents, PrototypeComponents, IsConcrete, IsStruct, RefListCompoundMutability, CoordSend, IndexList

---

#### 26. `solve_stub` (lines 1283-1292)
**Verdict: NEEDS_WORK**
**Violations: 4 issues**

**Issue 1 — Naming inconsistency:** The comment says `// mig: fn solve` but the function is named `solve_stub`.

**Issue 2 — Signature mismatch:** The Scala code shows `solve` should accept `(state: Unit, env: IRuneTypeSolverEnv, solverState: ISolverState[...], ruleIndex: Int, rule: IRulexSR, stepState: IStepState[...])` and call `solveRule(...)`. The Rust signature is `(state: (), env: (), solver_state: (), rule_index: usize, rule: &IRulexSR, step_state: ())` with generic type parameters completely missing. Return type is `Result<(), ()>` instead of proper error types.

**Issue 3 — Incorrect implementation:** Instead of calling the existing `solve_rule` function (which is fully implemented), it just panics with `"Unimplemented solve"`. The Scala code delegates directly to `solveRule`.

**Issue 4 — Unused parameters:** All parameters are prefixed with `_`, indicating the implementation was never attempted.

---

### File: `post_parser.rs`

---

#### 27. `scout_interface` (lines 2280-2484)
**Verdict: NEEDS_WORK**
**Violations: 3 issues**

**Issue 1 — MACT:** Missing comment `"This is an array instead of a map so we can detect conflicts afterward"` (Scala line 2517, should be near Rust line 2386).

**Issue 2 — MACT:** Missing comment `"Put this back in when we have regions"` (Scala line 831, should be near Rust line 2388).

**Issue 3 — RSMSCP (missing helper call):** The Rust version processes attributes directly in a loop (lines 2311-2338) without calling the `translate_citizen_attributes` helper function. The Scala version (line 887) explicitly calls `translateCitizenAttributes` with filtered attributes. The `translate_citizen_attributes` function exists at lines 2117-2143 but is not called from `scout_interface`, even though the Scala version calls it. The Rust `scout_struct` correctly calls this helper (lines 1920-1934), making this an inconsistency.

**Additional:** Line 2283 uses `&crate::parsing::ast::InterfaceP<'a, 'p>` — should import `InterfaceP` at the top per the style guide.

---

### File: `identifiability_solver.rs`

---

#### 28. `get_puzzles` (lines 139-184)
**Verdict: NEEDS_WORK**
**Violation: RSMSCP — match arm order wrong**

**Scala order:** Equals, MaybeCoercingLookup, Lookup, RuneParentEnvLookup, MaybeCoercingCall, Pack, DefinitionCoordIsa, CallSiteCoordIsa, KindComponents, CoordComponents, PrototypeComponents, Resolve, CallSiteFunc, DefinitionFunc, OneOf, IsConcrete, IsInterface, IsStruct, CoerceToCoord, Literal, Augment, RefListCompoundMutability

**Rust order:** Equals, MaybeCoercingLookup, Lookup, RuneParentEnvLookup, MaybeCoercingCall, Augment, OneOf, IsInterface, CoordComponents, CoerceToCoord, Call, Literal, Pack, CallSiteFunc, DefinitionFunc, Resolve, CoordSend, DefinitionCoordIsa, CallSiteCoordIsa, KindComponents, PrototypeComponents, IsConcrete, IsStruct, RefListCompoundMutability, IndexList

The first 5 match. Starting from position 6, the order diverges. Additionally, several cases that should return actual values from Scala logic are panicking instead (DefinitionCoordIsa, CallSiteCoordIsa, KindComponents, IsConcrete, IsStruct, RefListCompoundMutability).

---

#### 29. `solve_rule_impl` (lines 231-411)
**Verdict: NEEDS_WORK**
**Violations: 6 issues**

**Issue 1 — Order is completely wrong:** Scala starts with `KindComponents` (line 422), then `CoordComponents`, then `PrototypeComponents`, etc. Rust starts with `CoordComponents`, `MaybeCoercingCall`, `OneOf`, etc.

**Issue 2 — Missing implementations:** Rust panics on `KindComponents` (line 393), `IsConcrete` (line 406), and `IsStruct` (line 407), but Scala has full implementations for these (lines 422-426, 489-492, 497-500 respectively).

**Issue 3 — Missing function parameters:** The Scala `solveRule` receives `(state: Unit, env: Unit, ruleIndex: Int, callRange: List[RangeS], rule: IRulexSR, stepState: IStepState[...])` but Rust removes `state` and `env`, receiving only `(_rule_index: i32, call_range: &[RangeS<'a>], rule: &IRulexSR<'a>, solver_state: &mut S)`.

**Issue 4 — KICI violation (CallSiteCoordIsa):** Scala's `CallSiteCoordIsaSR` case (lines 471-479) has inline pattern matching on the `resultRune` Option: `resultRune match { case Some(resultRune) => ... case None => }`. Rust (line 392) just panics.

**Issue 5 — Duplicate case in Scala:** Lines 536-539 repeat `MaybeCoercingLookupSR` case (already at lines 519-522). This appears to be a Scala quirk, but Rust should match the structure.

**Issue 6 — Variable naming:** Scala uses `stepState`; Rust uses `solver_state`.

---

### File: `rule_scout.rs`

---

#### 30. `refs` builtin call block (~line 202) + `PrototypeType` components block (~line 311)
**Verdict: NEEDS_WORK**
**Violations: 3 issues**

**Issue 1 — RSMSCP (`refs` block, lines 202-222):** The result is returned differently:
- Scala creates a fresh `RuneUsage` with `evalRange(range)`: `rules.RuneUsage(evalRange(range), resultRune.rune)`
- Rust returns the previously-created `result_rune` directly, which has a different range value

**Issue 2 — RSMSCP (`PrototypeType` block, lines 311-333):** Uses index-based access after collecting instead of Scala's direct destructuring:
- Scala: `val Vector(paramsRuneS, returnRuneS) = translateRulexes(...)`
- Rust: `let component_usages = translate_rulexes(...); let params_rune = component_usages[0].clone();`

**Fix:** Use array/slice pattern matching: `let [params_rune, return_rune] = ...` or similar destructuring.

**Issue 3 — Style:** Full module paths like `crate::postparsing::rules::rules::PackSR` and `crate::postparsing::rules::rules::PrototypeComponentsSR` should be imported.

---

---

## Additional Findings from Opus Single-Prompt Review

The following issues were identified by a separate Opus single-prompt pass that reviewed all changed files at once. They include items not covered by the 30-agent audit above, as well as deeper analysis of items that were covered. Organized by priority.

---

### HIGH PRIORITY (logic differences from Scala)

#### H1. `translate_interface` — missing cache store and return (higher_typing_pass.rs)
Scala inserts into `codeLocationToInterface` and returns the cached value on hit. Rust panics on cache hit and never stores the result. *(Also found by agent #10 above — Issues 3 & 4.)*

#### H2. `translate_program` — not appending supplied items (higher_typing_pass.rs)
**NEW FINDING.** Scala does `suppliedInterfaces ++ interfacesA` and `suppliedFunctions ++ functionsA` when building the final `ProgramA`. Rust ignores the supplied items entirely. Currently harmless (empty vectors passed in) but semantically wrong — the Rust version will break when non-empty supplied items are passed.

#### H3. `explicify_lookups` — `TemplataLookupResult` branch (higher_typing_pass.rs)
**NEW FINDING (deeper than agent #5).** The entire `TemplataLookupResult` branch is a full panic stub. Scala has 4 sub-cases for coercing lookups in this branch. Will crash on rune-name lookups that resolve to templata results.

#### H4. `coerce_kind_template_lookup_to_coord` — full panic stub (higher_typing_pass.rs)
**NEW FINDING — contradicts agent #2 APPROVED verdict.** The agent approved `coerce_kind_template_lookup_to_kind`, but this is a *different* function: `coerce_kind_template_lookup_to_coord`. Scala creates 3 rules (LookupSR + CallSR + CoerceToCoordSR). The Rust version crashes on struct-in-coord-context because this function is a panic stub.

#### H5. `explicify_lookups` — missing error returns (higher_typing_pass.rs)
Panics instead of returning `FoundPrimitiveDidntMatchExpectedType` / `FoundTemplataDidntMatchExpectedTypeA` on type mismatches. *(Related to agent #12's finding about `translate_function` error handling.)*

#### H6. `FunctionA::new` — missing 3 Scala assertions (higher_typing_pass.rs or ast.rs)
**NEW FINDING.** Missing:
- Package coord consistency check
- Rune coverage check across rules
- Param coord rune coverage check

These are constructor-level assertions that Scala's `FunctionA` case class performs.

#### H7. `run_pass` — grouping key difference (higher_typing_pass.rs)
**NEW FINDING.** Rust groups functions/impls by `f.range.begin.file.package_coord`. Scala groups by `_.name.packageCoordinate`. These are different fields and may produce different groupings when range and name disagree on package coordinates.

#### H8. `scout_interface` — `predict_rune_types` arguments mismatch (post_parser.rs)
**NEW FINDING (deeper than agent #27).** Two differences in the arguments passed to `predict_rune_types`:
- **(a)** Rust passes `identifying_runes_s` but Scala passes `userDeclaredRunes` (which includes runes from rules, not just identifying runes).
- **(b)** Rust passes `rune_to_explicit_type.clone()` but Scala passes an empty `ArrayBuffer()`.

Both differences change what the rune type prediction step receives, potentially affecting which rune types are predicted.

#### H9. `solve_rune_type` — `MaybeCoercingLookup` pre-computation, Templata branch (rune_type_solver.rs)
**NEW FINDING (deeper than agent #22).** Rust does not exclude `TemplateTemplataType([], CoordTemplataType)` as Scala does. Scala checks `TemplateTemplataType(Vector(), KindTemplataType() | CoordTemplataType())` but Rust only checks for `KindTemplataType` return type. This means Rust will fail to pre-compute types for template lookups that return `CoordTemplataType`.

#### H10. `lookup_rune_type` — Templata branch (rune_type_solver.rs)
Full panic. Scala has complete match logic including `KindTemplataType -> CoordTemplataType` coercion and `TemplateTemplataType(Vector(), Kind|Coord) -> Coord|Kind` implicit call. *(Also found by agent #24 — Issue 1.)*

#### H11. `lookup_rune_type` — error paths (rune_type_solver.rs)
Primitive and Citizen branches panic instead of returning proper `FoundPrimitiveDidntMatchExpectedType` / `FoundCitizenDidntMatchExpectedType` errors. *(Also found by agent #24 — Issues 2 & 3.)*

---

### MEDIUM PRIORITY (incomplete but consistently panic-stubbed)

#### M12. `solve_rule` — 11 rule variant panics (rune_type_solver.rs)
KindComponents, DefinitionCoordIsa, CallSiteCoordIsa, IsConcrete, IsStruct, RefListCompoundMutability, Lookup, RuneParentEnvLookup, Call, CoordSend, IndexList. Most have trivial Scala implementations. *(Also found by agent #23.)*

#### M13. `get_puzzles` / `solve_rule` — 8 variant panics (identifiability_solver.rs)
KindComponents, IsConcrete, IsStruct, RefListCompoundMutability, DefinitionCoordIsa, CallSiteCoordIsa, CoordSend, IndexList. *(Also found by agents #28 and #29.)*

---

### LOWER PRIORITY (structural/novel but likely correct)

#### L14. `imprecise_name_matches_absolute_name` — split match arms (higher_typing_pass.rs)
Scala matches `TopLevelCitizenDeclarationNameS` (one type). Rust splits into two arms: `INameS::TopLevelStructDeclaration` and `INameS::TopLevelInterfaceDeclaration`. Functionally equivalent but verify no other variants should match. *(Also found by agent #6.)*

#### L15. `lookup_types` — extra panic fallthrough (higher_typing_pass.rs)
Rust adds `_ => panic!(...)` for the `IImpreciseNameS` match. Scala is exhaustive with just `CodeNameS` and `RuneNameS`. The panic is a safety net but represents novel code not in Scala.

#### L16. `EnvironmentA.rune_to_type` key type change (higher_typing_pass.rs)
Changed from `HashMap<&'a IRuneS<'a>, ...>` to `HashMap<IRuneS<'a>, ...>`. This is actually a fix — matches Scala's by-value semantics. Likely correct.

#### L17. `ILookupFailedErrorA` changed from trait to enum (higher_typing_pass.rs)
Verify it still satisfies `ICompileErrorA` where needed (`run_pass` returns `Box<dyn ICompileErrorA>`). The change from trait to enum is a Rust idiom but needs to be checked for compatibility.

#### L18. `HigherTypingPass` struct — added `scout_arena` field (higher_typing_pass.rs)
Rust-specific arena allocation field. Correct but structural difference from Scala. No Scala equivalent.

#### L19. `intern_struct_declaration_name` / `intern_interface_declaration_name` — novel convenience wrappers (interner.rs)
Novel convenience wrappers with no Scala equivalent. They're just sugar over `intern_name` but verify callers pass correct types.

#### L20. `&'a IRuneS<'a>` → `IRuneS<'a>` across ~12 fields (names.rs)
Removes double-indirection. Confirmed correct per IDEPFL document (tagged pointer is already cheap to store by value).

#### L21. `PackageCoordinateMap` — `Clone` bound removed (code_hierarchy.rs)
Actually improves Scala parity. Verify no methods in the impl need `Clone`.

---

### Opus Review Priority Recommendation

> Items H1, H2, H8, and H9 are the ones to prioritize most — they're subtle logic differences rather than obvious panic stubs. Items H3-H5 and H10-H11 are panics that will crash on specific code patterns but are at least obvious when hit.

---

## Common Themes

### 1. Match Arm Ordering (7 functions)
`solve_rule`, `get_puzzles_rune_type`, `get_puzzles`, `solve_rule_impl` — all have match arms in different order than Scala. This is the most pervasive issue.

### 2. Control Flow Structure Mismatches (5 functions)
`explicify_lookups` (if-else vs match+guard), `lookup_types` (if-let vs match), `lookup_type` (len() vs slice patterns), `get_astrouts` (if+? vs match), `rule_scout.rs` (index vs destructuring).

### 3. Missing Error Handling (4 functions)
`translate_function`, `translate_interface`, `lookup_rune_type`, `solve_stub` — all panic where Scala returns proper errors.

### 4. Style: `crate::` Paths (7 locations)
All 5 test functions, `scout_interface`, `rule_scout.rs` — use long `crate::` paths instead of imports.

### 5. Missing Comments — MACT (4 functions)
`imprecise_name_matches_absolute_name`, `translate_interface`, `get_puzzles_rune_type`, `scout_interface`.

### 6. Missing Parameters (3 functions)
`translate_program` (missing `primitives`), `calculate_rune_types` (missing `use_optimized_solver`), `solve_rule_impl` (missing `state`/`env`).

### 7. Missing Cache Logic (1 function)
`translate_interface` — cache hit panics instead of returning; cache insert never happens.

### 8. Statement Order (1 function pattern)
`translate_impl` (and likely other `translate_*` functions) — `rune_typing_env` created lazily instead of matching Scala's early initialization.
