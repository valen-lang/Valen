# Which Structs Are Arena-Allocated?

A struct is "arena-allocated" if it's created via `arena.alloc(MyStruct { ... })` and stored as `&'s MyStruct` or `&'a MyStruct`. These structs must not contain heap-allocating fields (`Vec`, `HashMap`, `String`).

## Arena-allocated (scout arena, `'s`)

**Postparser AST** (`src/postparsing/ast.rs`): `StructS`, `InterfaceS`, `ImplS`, `FunctionS`, `GenericParameterS`, `GenericParameterDefaultS`, `ParameterS`, `ExportAsS`, `ImportS`

**Postparser expressions** (`src/postparsing/expressions.rs`): `LetSE`, `IfSE`, `BlockSE`, `BodySE`, `PureSE`, `ConsecutorSE`, `FunctionCallSE`, `OutsideLoadSE`, `ConstantStrSE`, `ConstantIntSE`, `ConstantBoolSE`, `ReturnSE`, `FunctionSE`, `DotSE`, `OwnershippedSE`, `LocalLoadSE`, `RuneLookupSE`, `StaticArrayFromValuesSE`, `StaticArrayFromCallableSE`, `NewRuntimeSizedArraySE`, and all other `IExpressionSE` variants.

**Postparser rules** (`src/postparsing/rules/rules.rs`): All `IRulexSR` variant structs — `EqualsSR`, `LiteralSR`, `MaybeCoercingCallSR`, `CallSR`, `PackSR`, `OneOfSR`, `AugmentSR`, etc.

**Higher typing AST** (`src/higher_typing/ast.rs`): `StructA`, `InterfaceA`, `ImplA`, `FunctionA`, `ExportAsA`, `ProgramA`

## Arena-allocated (interner arena, `'a`)

**Names** (`src/postparsing/names.rs`): All `IRuneS` variant payloads (e.g. `ImplicitRuneS`, `CodeRuneS`), all `INameS` variant payloads, all `IImpreciseNameS` variant payloads, `PackageCoordinate`, `FileCoordinate`.

## NOT arena-allocated (heap/stack)

These are mutable working data or context — they use `Clone`, `Box`, `HashMap`, `IndexSet` freely:

- `EnvironmentS`, `FunctionEnvironmentS` — postparser scope contexts, cloned and boxed
- `StackFrame` — expression scouting context
- `Astrouts` — higher typing pass accumulator (stack-local, `&mut`)
- `EnvironmentA` — higher typing scope context
- `HigherTypingPass`, `PostParser` — pass infrastructure (holds arena references)
- `LocationInDenizenBuilder` — mutable builder for `LocationInDenizen`
- `VariableDeclarations`, `VariableUses` — transient accumulators
- Error types (`ICompileErrorS`, `ICompileErrorA`, `RuneTypeSolveError`, etc.) — returned via `Result`
- Solver state (`SimpleSolverState`, `OptimizedSolverState`) — mutable during solving
