// Binary entry point for FrontendRust
// Mirrors Frontend.jar entry point which calls PassManager.main()
// From Frontend/PassManager/src/dev/vale/passmanager/PassManager.scala lines 390-481

use std::env;

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();
  frontend_rust::pass_manager::pass_manager::main(args);
}
