// Builds the C++ backend as a static library and emits link directives so the
// FrontendRust crate can call into it via FFI.
//
// Requires LLVM 21, dynamically linked against the Vale rustc fork's shared
// libLLVM (so valec and valec-rs share one libLLVM; two static libLLVMs in one
// process is duplicate-symbol UB — arch §3.6/§5.7). The build prefers
// `$LLVM_CONFIG`; otherwise it derives the fork's llvm-config from the active
// toolchain's sysroot (`<sysroot>/../llvm/bin/llvm-config`).

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
  // The C++ backend links against the Vale rustc fork's shared libLLVM 21. An interop
  // build ALSO loads rustc's own libLLVM through rustc_driver's dylibs — but both now
  // resolve to the *same* shared libLLVM, so there is no dual-LLVM duplicate-symbol UB
  // (that was the LLVM-16-static hazard this gate used to guard; arch §3.6/§5.7). So
  // the backend is built and linked in interop builds too; interop additionally needs
  // the rustc-private rpath.
  //
  // Feature flags are read from env (`CARGO_FEATURE_*`) rather than cfg because a build
  // script cannot see RUSTFLAGS.
  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RUST_INTEROP");
  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_NO_BACKEND");
  if env::var_os("CARGO_FEATURE_RUST_INTEROP").is_some() {
    emit_rustc_private_rpath();
  }

  // The `no_backend` feature (Frontend/VM test builds) still skips the C++ backend, so its
  // `backend_ffi` C symbols are left unresolved — tell the linker to tolerate that.
  if env::var_os("CARGO_FEATURE_NO_BACKEND").is_some() {
    emit_allow_unresolved_backend_symbols();
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

  // LLVM shared lib. The fork builds one libLLVM dylib (LLVM_LINK_LLVM_DYLIB=ON),
  // so `--link-shared` collapses the whole component list to a single
  // `-lLLVM-<ver>`. Dynamic linking is mandatory: it lets valec and valec-rs share
  // one libLLVM (arch §3.6/§5.7).
  let llvm_libdir = run(&llvm_config, &["--libdir"]);
  println!("cargo:rustc-link-search=native={}", llvm_libdir);
  // rpath so the shared libLLVM resolves at runtime without DYLD_* being set.
  println!("cargo:rustc-link-arg=-Wl,-rpath,{}", llvm_libdir);

  let llvm_libs = run(
    &llvm_config,
    &[
      "--libs",
      "--link-shared",
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
      println!("cargo:rustc-link-lib=dylib={}", name);
    }
  }

  // System libs LLVM itself needs (empty for a self-contained shared libLLVM, but
  // keep the query for portability across LLVM build configurations).
  let system_libs = run(&llvm_config, &["--system-libs", "--link-shared"]);
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

  // The rustc_private dylibs (librustc_driver-<hash>.dylib et al.) live in the *target* libdir, not
  // <sysroot>/lib (which carries a different-hash copy of librustc_driver), so bake that too or dyld
  // cannot resolve `@rpath/librustc_driver-<hash>.dylib` at startup — the very DYLD_LIBRARY_PATH this
  // rpath is meant to eliminate. `target-libdir` is already the lib dir, so it needs no `/lib` suffix.
  match Command::new(&rustc).args(["--print", "target-libdir"]).output() {
    Ok(o) if o.status.success() => {
      let target_libdir = String::from_utf8_lossy(&o.stdout).trim().to_string();
      println!("cargo:rustc-link-arg=-Wl,-rpath,{}", target_libdir);
    }
    _ => println!(
      "cargo:warning=could not determine rustc target-libdir; rustc_private artifacts may need \
                    DYLD_LIBRARY_PATH set to <sysroot>/lib/rustlib/<target>/lib"
    ),
  }
  println!("cargo:rerun-if-env-changed=RUSTC");
}

/// Tell the linker to tolerate the unresolved `backend_ffi` C symbols left behind when the C++
/// backend is not built. Both the `rust_interop` and `no_backend` feature builds skip the backend,
/// so its symbols have no definition. They are referenced from `pass_manager::build`'s metal lowerer
/// but never *called* on a typecheck-only path, so leaving them unresolved is safe — a stray call
/// faults at runtime, the honest signal that the backend is absent. Without this, `-dead_strip` drops
/// them only while nothing references them; once something does (as the metal lowerer now does), the
/// link fails instead. Temporary, until the backend is reshaped for the onion metal IR.
fn emit_allow_unresolved_backend_symbols() {
  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
  if target_os == "macos" {
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup");
  } else {
    println!("cargo:rustc-link-arg=-Wl,--unresolved-symbols=ignore-all");
  }
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
  // The Vale rustc fork builds its shared libLLVM as a sibling of the stage2
  // sysroot: sysroot is `.../<target>/stage2`, and llvm-config lives at
  // `.../<target>/llvm/bin/llvm-config`. Derive it from the active toolchain so
  // no machine-specific path is baked in and it tracks whatever fork is pinned.
  let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
  if let Ok(out) = Command::new(&rustc).args(["--print", "sysroot"]).output() {
    if out.status.success() {
      let sysroot = String::from_utf8_lossy(&out.stdout).trim().to_string();
      let cand = PathBuf::from(&sysroot).join("..").join("llvm").join("bin").join("llvm-config");
      if cand.exists() {
        return cand;
      }
    }
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
