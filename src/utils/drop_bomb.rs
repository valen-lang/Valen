use std::thread;

/// A guard that panics if it is dropped while **armed** (unless a panic is already unwinding).
/// Extracts the "you forgot to discharge this obligation" enforcement so an owning type can embed one
/// instead of hand-rolling a `Drop`. Debug-only (`debug_assert!`), so release builds pay nothing.
///
/// Arm it while an obligation is outstanding; `defuse` it the moment the obligation is discharged (or
/// deliberately declined). A defused bomb drops silently.
pub struct DropBomb {
  armed: bool,
  message: &'static str,
}

impl DropBomb {
  /// The only constructor — a bomb is always born armed. There is deliberately no defused
  /// constructor: it starts armed and stays that way until someone `defuse`s it, so forgetting to
  /// handle the obligation it guards trips the bomb.
  pub fn armed(message: &'static str) -> DropBomb {
    DropBomb { armed: true, message }
  }

  pub fn arm(&mut self) {
    self.armed = true;
  }

  pub fn defuse(&mut self) {
    self.armed = false;
  }
}

impl Drop for DropBomb {
  fn drop(&mut self) {
    debug_assert!(!self.armed || thread::panicking(), "{}", self.message);
  }
}
