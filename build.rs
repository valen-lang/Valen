
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RUST_INTEROP");
  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_NO_BACKEND");
  if env::var_os("CARGO_FEATURE_RUST_INTEROP").is_some() {
    emit_rustc_private_rpath();
  }

  if env::var_os("CARGO_FEATURE_NO_BACKEND").is_some() {
    emit_allow_unresolved_backend_symbols();
    return;
  }

  let backend_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("Backend");

  let llvm_config = locate_llvm_config();
  let llvm_dir = run(&llvm_config, &["--cmakedir"]);

  let rust_interop = env::var_os("CARGO_FEATURE_RUST_INTEROP").is_some();
  let dst = cmake::Config::new(&backend_dir)
    .define("LLVM_DIR", &llvm_dir)
    .define("CMAKE_BUILD_TYPE", "Debug")
    .define("VALE_RUST_INTEROP", if rust_interop { "ON" } else { "OFF" })
    .build_target("backend_lib")
    .build();

  let build_dir = dst.join("build");
  println!("cargo:rustc-link-search=native={}", build_dir.display());
  println!("cargo:rustc-link-lib=static=backend_lib");

  let llvm_libdir = run(&llvm_config, &["--libdir"]);
  println!("cargo:rustc-link-search=native={}", llvm_libdir);
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

  let system_libs = run(&llvm_config, &["--system-libs", "--link-shared"]);
  for lib in system_libs.split_whitespace() {
    if let Some(name) = lib.strip_prefix("-l") {
      println!("cargo:rustc-link-lib=dylib={}", name);
    }
  }

  if cfg!(target_os = "macos") {
    if std::path::Path::new("/opt/homebrew/lib").exists() {
      println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    }
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
  } else if cfg!(target_os = "freebsd") {
    println!("cargo:rustc-link-lib=dylib=c++");
  } else {
    println!("cargo:rustc-link-lib=dylib=stdc++");
  }

  println!("cargo:rerun-if-changed={}", backend_dir.join("CMakeLists.txt").display());
  watch_dir_recursive(&backend_dir.join("src"));
  println!("cargo:rerun-if-env-changed=LLVM_CONFIG");
  println!("cargo:rerun-if-env-changed=LLVM_DIR");
}

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
  let mut candidates: Vec<PathBuf> = Vec::new();
  if let Ok(out) = Command::new("brew").args(["--prefix", "llvm@21"]).output() {
    if out.status.success() {
      let prefix = String::from_utf8_lossy(&out.stdout).trim().to_string();
      candidates.push(PathBuf::from(prefix).join("bin").join("llvm-config"));
    }
  }
  candidates.push(PathBuf::from("/opt/homebrew/opt/llvm@21/bin/llvm-config"));
  candidates.push(PathBuf::from("llvm-config-21"));
  candidates.push(PathBuf::from("llvm-config"));
  for cand in candidates {
    if llvm_config_is_v21(&cand) {
      return cand;
    }
  }
  panic!(
    "no LLVM 21 llvm-config found. Set $LLVM_CONFIG to an LLVM 21 llvm-config, or install one \
     (macOS: `brew install llvm@21`). A stock-nightly dev build needs a standalone LLVM 21; the \
     Vale rustc fork supplies its own only for `--features rust_interop`."
  );
}

fn llvm_config_is_v21(path: &PathBuf) -> bool {
  match Command::new(path).arg("--version").output() {
    Ok(out) if out.status.success() => {
      String::from_utf8_lossy(&out.stdout).trim().split('.').next() == Some("21")
    }
    _ => false,
  }
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
