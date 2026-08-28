// valec-rs — the rustc-hosted Vale driver.
//
// rustc cannot be called as a library that hands back a `TyCtxt`: the type exists only inside
// `rustc_driver::run_compiler`'s callback. So the control flow inverts — rustc hosts, and Vale's
// typing pass runs inside `Callbacks::after_expansion`. That machinery now lives in the library
// (`typing::rust_interop::drive`); this binary is the thin CLI over it (@DBAPIZ): it parses argv and
// calls `run_drive`, nothing more.
//
// `#![feature(rustc_private)]` is still required even though this file names no rustc type: it links the
// interop-enabled library, which pulls in rustc's private crates, and a crate linking those must opt in.
//
// The `drive` subcommand is the interim bridge that unblocks NobiliaV early — compile and run a Valen
// program against already-built rlibs (which Pearl produces with `cargo +rustc-fork build`), forwarding
// caller-supplied `--extern`/`-L dependency` flags. The permanent cargo-workspace pipeline (arch §18/§20)
// supersedes the manual front-end; `drive_and_link` and the `vale-stub-gen` seed it calls survive it.

#![feature(rustc_private)]

use std::process::exit;

use clap::{Parser, Subcommand};

use frontend_rust::typing::rust_interop::drive::{run_drive, DriveArgs};

#[derive(Parser)]
#[command(name = "valec-rs", about = "The rustc-hosted Vale driver.", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  /// Compile and run a Valen program against already-built Rust rlibs (the interim bridge).
  Drive(DriveArgs),
}

fn main() {
  let cli = Cli::parse();
  match cli.command {
    Command::Drive(args) => exit(run_drive(&args)),
  }
}
