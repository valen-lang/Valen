# Identifiability Checks Include Parent Citizen Runes (ICIPCRZ)

When a function is an internal method of a citizen (struct or interface), the
identifiability check at the postparser sees the parent citizen's generic
parameters as already-identifying runes — even though the user didn't write
them on the function header.

The justification: parent runes are *always* supplied by the callsite via the
qualified container syntax. `Vec<int>.with_capacity(42i64)` writes `T = int`
on the prefix, not on the method. The identifiability solver has no concept of
"parent environment," only identifying runes — so parent runes have to be
explicitly added to the identifying set to model that input avenue.

## Where it lives

- `PostParsingPass/.../FunctionScout.scala` — when scouting a method whose
  parent is a citizen (`ParentInterface`), `extraGenericParamsFromParentS` are
  appended to the function's own generic parameters before
  `checkIdentifiability` runs (parent runes go at the end per @PRIIROZ). The
  check sees a unified list and treats parent runes as legitimate inputs.

- `PostParsingPass/.../FunctionScout.scala` — `RuneParentEnvLookupSR` rules
  are filtered out of the rule array before identifiability runs. The
  `IdentifiabilitySolver` has no environment to look anything up from — its
  `solveRule` for that rule is `vimpl()` — so the rule must be removed at the
  postparser. The runtime side instead delivers parent runes through the
  explicit-callsite channel (see below); no env-lookup rule is needed.

- `TypingPass/.../OverloadResolver.scala` — call sites that *don't* go through
  the explicit-callsite channel still preprocess any surviving
  `RuneParentEnvLookupSR` rules into `InitialKnown`s by looking up the rune in
  the calling env. This handles e.g. function bounds and other paths that
  weren't reshaped by the rust-interop work.

## The runtime delivery: explicit callsite container template args

See @PRIIROZ for the callsite shape and rationale. On top of that, the
typing pass turns each container rune binding into an `EqualsSR` for the
explicit-args pre-solve, then (once resolved) re-emits them as
`InitialKnown`s that get appended to the function solve's known templatas
— see `OverloadResolver.attemptCandidateBanner` and follow
`containerRuneInitialKnowns` down to `FunctionCompilerSolvingLayer`.

This is the channel that *justifies* including parent runes in the
identifiability check: the callsite syntactically supplied them, and the
typing pass wires them into the solver as InitialKnowns.

## See also

- @PRIIROZ (Parent Runes Inherited In Reverse Order) — comment block in
  `ExpressionCompiler.scala` explaining why parent runes go at the end of
  the method's identifying-rune list, and why the named-arg channel is used
  instead of positional flatten.
- `PostParsingPass/.../expressions.scala` — `OutsideLoadSE` and `LoadPartSE`
  definitions.
- `FunctionCompilerSolvingLayer.evaluateGenericFunctionFromCallForPrototype`
  — the solve site where `containerRuneInitialKnowns` is appended to
  `initialKnowns`.
