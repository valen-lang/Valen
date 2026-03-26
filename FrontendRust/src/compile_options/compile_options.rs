#[derive(Clone)]
pub struct GlobalOptions {
  pub sanity_check: bool,
  pub use_overload_index: bool,
  pub use_optimized_solver: bool,
  pub verbose_errors: bool,
  pub debug_output: bool,
}
/*
Guardian: disable: NECX

package dev.vale.options

object GlobalOptions {
  def apply(): GlobalOptions = {
    GlobalOptions(
      sanityCheck = false,
      useOverloadIndex = false,
      useOptimizedSolver = true,
      verboseErrors = false,
      debugOutput = false)
  }

  def test(): GlobalOptions = {
    GlobalOptions(true, false, true, true, true)
  }
}

case class GlobalOptions(
  sanityCheck: Boolean,
  useOverloadIndex: Boolean,
  useOptimizedSolver: Boolean,
  verboseErrors: Boolean,
  debugOutput: Boolean)
*/
