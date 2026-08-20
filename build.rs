// Builds the C++ backend as a static library and emits link directives so the
// FrontendRust crate can call into it via FFI.
//
// Requires LLVM 16. The build prefers `$LLVM_DIR` / `$LLVM_CONFIG`; falls back
// to the Homebrew arm64 prefix on macOS.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
  // Interop builds skip the C++ backend entirely.
  //
  // This script statically links ~20 LLVM 16 component libraries into every artifact.
  // An interop build also loads rustc's own libLLVM (~21) through rustc_driver's dylibs,
  // and two LLVMs in one process is duplicate-symbol UB — LLVM keeps process-global
  // state (pass registries, command-line option registration). See
  // docs/convos/rust_interop/vale-rust-interop-architecture.md §3.6 / §5.7.
  //
  // TEMPORARY, and specifically NOT the backend becoming optional: Vale's C++ Backend
  // owns every byte of Vale-emitted LLVM IR (arch §1.7) and is required in both
  // binaries. This gate expires when the backend is ported from LLVM 16 to rustc's
  // pinned LLVM (~21) and switched to dynamic linking, which §3.6 mandates and which is
  // what makes one shared libLLVM possible. Until then an interop build can typecheck
  // but cannot reach codegen.
  //
  // Read from the cargo feature rather than a cfg because a build script cannot see
  // RUSTFLAGS. Absent the feature this is unset and the backend builds exactly as before.
  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RUST_INTEROP");
  if env::var_os("CARGO_FEATURE_RUST_INTEROP").is_some() {
    emit_rustc_private_rpath();
    return;
  }

  // Frontend/VM test builds (the `no_backend` feature) skip the C++ backend entirely. The backend's
  // C symbols (backend_ffi) are then unresolved; tell the linker to allow that rather than build the
  // red C++ backend. Nothing in the frontend or TestVM tests calls them, so they never bind; a stray
  // call would fault at runtime, which is the honest signal that the backend is absent. Temporary,
  // until the backend is reshaped for the onion metal IR.
  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_NO_BACKEND");
  if env::var_os("CARGO_FEATURE_NO_BACKEND").is_some() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "macos" {
      println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup");
    } else {
      println!("cargo:rustc-link-arg=-Wl,--unresolved-symbols=ignore-all");
    }
    return;
  }

  let backend_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("Backend");

  let llvm_config = locate_llvm_config();
  let llvm_dir = run(&llvm_config, &["--cmakedir"]);

  // Build only the backend_lib static library target. Skips the legacy
  // `backend` executable (which keeps building via plain cmake for now).
  let dst = cmake::Config::new(&backend_dir)
    .define("LLVM_DIR", &llvm_dir)
    .define("CMAKE_BUILD_TYPE", "Debug")
    .build_target("backend_lib")
    .build();

  // cmake-rs places build artifacts under `<dst>/build/`.
  let build_dir = dst.join("build");
  println!("cargo:rustc-link-search=native={}", build_dir.display());
  println!("cargo:rustc-link-lib=static=backend_lib");

  // LLVM static libs.
  let llvm_libdir = run(&llvm_config, &["--libdir"]);
  println!("cargo:rustc-link-search=native={}", llvm_libdir);

  let llvm_libs = run(
    &llvm_config,
    &[
      "--libs",
      "--link-static",
      "core",
      "support",
      "irreader",
      "passes",
      "aarch64asmparser",
      "aarch64codegen",
      "aarch64desc",
      "aarch64disassembler",
      "aarch64info",
      "x86asmparser",
      "x86codegen",
      "x86desc",
      "x86disassembler",
      "x86info",
      "webassemblyasmparser",
      "webassemblycodegen",
      "webassemblydesc",
      "webassemblydisassembler",
      "webassemblyinfo",
    ],
  );
  for lib in llvm_libs.split_whitespace() {
    if let Some(name) = lib.strip_prefix("-l") {
      println!("cargo:rustc-link-lib=static={}", name);
    }
  }

  // System libs LLVM itself needs.
  let system_libs = run(&llvm_config, &["--system-libs", "--link-static"]);
  for lib in system_libs.split_whitespace() {
    if let Some(name) = lib.strip_prefix("-l") {
      println!("cargo:rustc-link-lib=dylib={}", name);
    }
  }

  // C++ stdlib. Rust links libc++ on macOS by default.
  if cfg!(target_os = "macos") {
    // Homebrew installs libs like zstd outside the default search path.
    if std::path::Path::new("/opt/homebrew/lib").exists() {
      println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    }
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
  } else {
    println!("cargo:rustc-link-lib=dylib=stdc++");
  }

  println!("cargo:rerun-if-changed={}", backend_dir.join("CMakeLists.txt").display());
  // Watch the entire Backend/src tree so any C++ edit triggers a rebuild.
  // cmake-rs picks up the actual source changes and rebuilds only what's
  // stale; cargo just needs to know *something* under Backend changed.
  watch_dir_recursive(&backend_dir.join("src"));
  println!("cargo:rerun-if-env-changed=LLVM_CONFIG");
  println!("cargo:rerun-if-env-changed=LLVM_DIR");
}

/// Bakes the rustc sysroot's lib dir into the artifact as an `LC_RPATH` / `-rpath`.
///
/// With `extern crate rustc_driver`, rustc emits a reference to
/// `@rpath/librustc_driver-<hash>.dylib` but does **not** emit a matching rpath load
/// command, so the artifact cannot find the dylib on its own and dies in dyld before
/// `main`. The usual workaround is `DYLD_LIBRARY_PATH` (or `DYLD_FALLBACK_LIBRARY_PATH` —
/// they are interchangeable here, because `@rpath` resolution always fails and so the
/// fallback is always reached). Baking the rpath in is better: the artifact runs standalone,
/// tests need no environment, and it survives code signing, which strips `DYLD_*`.
///
/// Measured and reported by the toylang/Sky prototype on this machine.
fn emit_rustc_private_rpath() {
  let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
  let out = Command::new(&rustc).args(["--print", "sysroot"]).output();
  let sysroot = match out {
    Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
    _ => {
      println!(
        "cargo:warning=could not determine rustc sysroot; rustc_private artifacts \
                      will need DYLD_LIBRARY_PATH set to <sysroot>/lib"
      );
      return;
    }
  };
  let lib_dir = PathBuf::from(&sysroot).join("lib");
  println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
  println!("cargo:rerun-if-env-changed=RUSTC");
}

fn watch_dir_recursive(dir: &std::path::Path) {
  if let Ok(entries) = std::fs::read_dir(dir) {
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        watch_dir_recursive(&path);
      } else {
        println!("cargo:rerun-if-changed={}", path.display());
      }
    }
  }
}

fn locate_llvm_config() -> PathBuf {
  if let Ok(path) = env::var("LLVM_CONFIG") {
    return PathBuf::from(path);
  }
  let homebrew = PathBuf::from("/opt/homebrew/opt/llvm@16/bin/llvm-config");
  if homebrew.exists() {
    return homebrew;
  }
  PathBuf::from("llvm-config")
}

fn run(prog: &PathBuf, args: &[&str]) -> String {
  let out = Command::new(prog)
    .args(args)
    .output()
    .unwrap_or_else(|e| panic!("failed to exec {} {:?}: {}", prog.display(), args, e));
  if !out.status.success() {
    panic!("{} {:?} failed: {}", prog.display(), args, String::from_utf8_lossy(&out.stderr));
  }
  String::from_utf8(out.stdout).unwrap().trim().to_string()
}
