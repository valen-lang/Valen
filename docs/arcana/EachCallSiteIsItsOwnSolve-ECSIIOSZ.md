# Each Call-Site Is Its Own Solve (ECSIIOSZ)

Every call-site in source code — every `Some<T>(x)`, every `[]&E(size, callable)`, every `add(list, 7)` — is lowered by the postparser into its own self-contained vector of solver rules, and the typing pass spins up a fresh `InferCompiler` solver instance per call-site to resolve them.

Call-site solves don't share state. Each gets its own rule vector, its own rune-to-type map, its own initial-knowns, and its own conclusion map.

For example, `Some<T>(x)` inside `moo<T>`'s body is scouted into roughly four rules: a lookup for `Some`, a `RuneParentEnvLookupSR` for `T` (per MKRFA, since `T` comes from moo's enclosing template), a `CallSR` applying `Some` to `T`, and a `ResolveSR` finding a prototype matching the argument type.

When compiling moo's body, the typing pass hits this call-site, builds a fresh solver over these four rules, seeds `T` as an initial known from moo's env, solves — producing `Some<moo$T>` — and discards the solver.

**Why per call-site:** call-sites have contextual knowledge that definitions don't — argument types, explicit template args, the surrounding environment — and each call-site's knowledge is different.

A per-call-site solver lets that knowledge be consumed as initial-knowns without polluting a shared definition solve or requiring the solver to partition its conclusion map by call-site.

It also maps cleanly to Vale's two solve modes, `solveForDefining` vs `solveForResolving` (see DBDAR), each of which runs over an appropriately-filtered rule vector per SROACSD.

**What this demands of call-site setup code:** every site that instantiates a solver (`ArrayCompiler`, `OverloadResolver`, `ImplCompiler`, `StructCompilerGenericArgsLayer`, `FunctionCompilerSolvingLayer`) is individually responsible for the full setup contract.

The rule vector is self-contained *except* for "portal" rules that only make sense relative to the caller. `RuneParentEnvLookupSR` must be preprocessed out into initial-knowns (MKRFA). Default generic-param rules must be added incrementally rather than up front (DRSINI). Site-specific rules (`ResolveSR`, `CallSiteFuncSR`, `DefinitionFuncSR`) must be filtered for the right solve mode (SROACSD). The calling function's env must be threaded through as `InferEnv.callingEnv` (CSSNCE).
