# Postparsing Migration: Scala vs Rust Differences

This document catalogs known differences between the Scala postparsing pass (`Frontend/PostParsingPass/src/dev/vale/postparsing/`) and the Rust port (`FrontendRust/src/postparsing/`).

---

## Logic Differences

### function_scout.rs

5. **Void-stripping in `scout_body`** — Kept as-is. Strips trailing `Void` from `Consecutor`; Scala doesn't have this, but removing it breaks tests because the Rust expression generator produces trailing Voids that Scala doesn't. Root cause is elsewhere.

### post_parser.rs

1. **`predict_rune_types`** — Only deduplicates explicit types, never calls the rune type solver. Returns only explicit types, not inferred.
2. **`scout_interface`** — Simplified: `assert!` blocks template rules, mutability, default region rune, generic params. Predicted types always empty. Mutability hardcoded to `Mutable`.
3. **`scout_generic_parameter`** — Two `NOT_YET_IMPLEMENTED` panics: coord region handling and default value handling.
4. **`get_scoutput`** — Caches `()` instead of `ProgramS`; doesn't actually scout.
5. **`expect_scoutput`** — No error humanization.
6. **`EnvironmentS.user_declared_runes`** — `Vec<IRuneS>` (ordered, allows duplicates) vs Scala's `Set[IRuneS]`.
7. **4 stubbed functions** — `determine_denizen_type`, `get_human_name`, `scout_export_as`, `scout_import`.
8. **Extra assertion in `scout_program`** — Checks `closured_names.is_empty()` on top-level function bodies; Scala only checks `variableUses.uses.isEmpty`.

### expression_scout.rs

14 expression types have no Rust implementation (commented-out Scala, fall to catch-all `panic!`):
`StrInterpolatePE`, `BreakPE`, `NotPE`, `RangePE`, `ConstantFloatPE`, `DestructPE`, `UnletPE`, `PackPE`, `BraceCallPE`, `TuplePE`, `AndPE`, `OrPE`, `IndexPE`, `ShortcallPE`.

2 fully stubbed helpers: `ends_with_return`, `flatten_expressions`. `ConstructArray` panics after validation (both runtime and static branches).

### templex_scout.rs

**Novel logic in `translate_maybe_type_into_maybe_rune`** — Inserts `CoordTemplataType` into `rune_to_explicit_type`; Scala's version accepts the same parameter but never uses it. 5 panic stubs: `RegionRune(None)`, `Function`, `Func`, `Pack`, catch-all.

### loop_post_parser.rs

`scout_loop` is a `panic!` stub. `scout_each`, `scout_while`, and body helpers are fully migrated and match Scala.

### identifiability_solver.rs

Migrated for 11 `IRulexSR` variants. Missing match arms for 11+ variants not yet in the Rust enum (`PackSR`, `DefinitionCoordIsaSR`, `CallSiteCoordIsaSR`, `KindComponentsSR`, `PrototypeComponentsSR`, `ResolveSR`, `CallSiteFuncSR`, `DefinitionFuncSR`, `IsConcreteSR`, `IsStructSR`, `RefListCompoundMutabilitySR`). Missing sanity-check assertion in `get_runes`.

### pattern_scout.rs

`rune_to_explicit_type` uses `HashMap` (last-write-wins) vs Scala's `ArrayBuffer` (keeps all entries). No interning of variable names.

---

## Missing Type Variants

### rules.rs — 14 missing `IRulexSR` variants
`PackSR`, `DefinitionCoordIsaSR`, `CallSiteCoordIsaSR`, `KindComponentsSR`, `PrototypeComponentsSR`, `ResolveSR`, `CallSiteFuncSR`, `DefinitionFuncSR`, `IsConcreteSR`, `IsStructSR`, `RefListCompoundMutabilitySR`, `CallSR`, `IndexListSR`, `CoordSendSR`.

### post_parser.rs — 11 missing `ICompileErrorS` variants
`UnknownRuleFunctionS`, `BadRuneAttributeErrorS`, `CantHaveMultipleMutabilitiesS`, `UnimplementedExpression`, `ForgotSetKeywordError`, `UnknownRegionError`, `CantOwnershipInterfaceInImpl`, `CantOwnershipStructInImpl`, `CantOverrideOwnershipped`, `VirtualAndAbstractGoTogether`, `CouldntSolveRulesS`.

### rule_scout.rs — `Equivalencies` class fully stubbed
All 5 methods (`mark_kind_equivalent`, `find_transitively_equivalent_into`, `get_kind_equivalent_runes`, `get_kind_equivalent_runes_iter`, `get_rune_kind_template`) panic. `OrPR` case missing from `translate_rulex`.

---

## Type Narrowing in names.rs

4 fields use `TopLevelCitizenDeclarationNameS` where Scala uses the broader `ICitizenDeclarationNameS`:
- `StructNameRuneS.struct_name`
- `InterfaceNameRuneS.interface_name`
- `ConstructorNameS.tlcd`

1 field goes wider: `LambdaStructImpreciseNameS.lambda_name` uses `IImpreciseNameS` where Scala uses `LambdaImpreciseNameS`.

---

## Missing Constructor Assertions in ast.rs

- **StructS** — Missing `DenizenDefaultRegionRuneS` checks on `genericParams` and all four rune-to-type maps.
- **InterfaceS** — Missing `DenizenDefaultRegionRuneS` checks and `internalMethod.genericParams == genericParams` validation.
- **FunctionS** — Missing `DenizenDefaultRegionRuneS` checks, `runeToPredictedType` check, and body/name consistency validation (extern/abstract/generated must not be lambda; closured code body must be lambda).
- **ParameterS** — Missing `vassert(pattern.coordRune.nonEmpty)`.
- **OtherGenericParameterTypeS** — Missing validation that `tyype` is not `RegionTemplataType` or `CoordTemplataType`.
