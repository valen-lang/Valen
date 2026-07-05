// The `rune_type_solver.rs` file inside this directory is preserved verbatim
// from `postparsing/rune_type_solver.rs` at commit `b5bde70e6` (its last
// living state before the postparse slice deleted it). Under onion typing
// the ~810 LOC `SimpleSolverState`-driven framework is scheduled to be
// replaced by a ~50-80 LOC walker; see the typing-slice section in
// `vcoord-handoff.md` for the rewrite plan.
//
// The file is kept here as reference material for that rewrite — real
// callers live in `typing/{array_compiler,overload_resolver}.rs` and
// `typing/expression/expression_compiler.rs`, all of which are gated with
// their host module today. The `#[cfg(any())]` gate below keeps this file
// inert even after `typing/` re-links, so the rewrite happens deliberately
// rather than by accident.
#[cfg(any())]
pub mod rune_type_solver;
