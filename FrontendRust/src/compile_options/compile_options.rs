// mig: struct GlobalOptions
#[derive(Clone)]
pub struct GlobalOptions {
  pub sanity_check: bool,
  pub use_overload_index: bool,
  pub use_optimized_solver: bool,
  pub verbose_errors: bool,
  pub debug_output: bool,
}

// mig: impl GlobalOptions
impl GlobalOptions {
  
  // mig: fn apply
  pub fn apply() -> GlobalOptions {
    GlobalOptions {
      sanity_check: false,
      use_overload_index: false,
      use_optimized_solver: true,
      verbose_errors: false,
      debug_output: false,
    }
  }
  
  // mig: fn test
  pub fn test() -> GlobalOptions {
    GlobalOptions {
      sanity_check: true,
      use_overload_index: false,
      use_optimized_solver: true,
      verbose_errors: true,
      debug_output: true,
    }
  }
  
}


