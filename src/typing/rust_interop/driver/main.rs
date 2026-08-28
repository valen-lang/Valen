// valenc-rs — the rustc-hosted Valen wrapper.
//
// cargo invokes it as `RUSTC_WORKSPACE_WRAPPER`, i.e. `valenc-rs <rustc-path> <rustc-args…>`, once per
// crate. rustc cannot be called as a library that hands back a `TyCtxt` — that type exists only inside
// `run_compiler`'s callback — so control inverts: rustc hosts, and Valen's passes run inside the
// callbacks. All that machinery lives in the library (`typing::rust_interop::drive`); this binary is the
// thin shell over its dark box (@DBAPIZ): it strips the rustc path cargo passes, ensures a `--sysroot`,
// and calls `run_wrapper`, which dispatches on the crate-root extension (`.valen` drives Valen, anything
// else passes through to plain rustc).
//
// `#![feature(rustc_private)]` is required even though this file names no rustc type: it links the
// interop-enabled library, which pulls in rustc's private crates, and a crate linking those must opt in.

#![feature(rustc_private)]

use std::process::exit;

use frontend_rust::typing::rust_interop::drive::{default_sysroot, run_wrapper, WrapperInputs};

fn main() {
  let argv: Vec<String> = std::env::args().collect();
  // cargo invokes `valenc-rs <rustc> <args…>`; drop argv[1] (the rustc path — we use our own linked-in
  // rustc) so what remains is the argv `run_compiler` expects: argv[0] (program name) + the crate flags.
  let mut rustc_args: Vec<String> = Vec::with_capacity(argv.len().saturating_sub(1));
  rustc_args.push(argv.first().cloned().unwrap_or_else(|| "valenc-rs".to_string()));
  rustc_args.extend(argv.into_iter().skip(2));
  // Our linked-in rustc needs the fork sysroot; cargo does not always pass one, so inject it if absent.
  // This env read lives above the dark box (@DBAPIZ), never inside `run_wrapper`.
  if !rustc_args.iter().any(|a| a == "--sysroot" || a.starts_with("--sysroot=")) {
    rustc_args.push(format!("--sysroot={}", default_sysroot()));
  }
  match run_wrapper(&WrapperInputs { rustc_args }) {
    Ok(result) => exit(result.rustc_exit),
    Err(e) => {
      eprintln!("valenc-rs: {e}");
      exit(1);
    }
  }
}
