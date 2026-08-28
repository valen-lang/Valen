// vale-stub-gen (the seed): emit a Rust stub crate's source from a scouted Vale program.
//
// This is the real, permanent mechanism (arch §6.4, @RTMEIZ §26.9), not a throwaway — the eventual
// per-project cargo-workspace pipeline calls this same generator. Today the only stubs are hand-written
// fixtures (`fixtures/stub.rs`, which calls itself "the stand-in for the eventual vale-stub-gen
// output"); this replaces that hand-writing for the driven path.
//
// It is driven by the program's *scouted* structure (not a text scan, not `HinputsT`): the load-bearing
// stub content for a consumer program — one `pub use` per `import rust.X.Y` (@RTMEIZ), the marker, and a
// `#[vale::emit_consumer_body]` root per exported func — all lives in the parsed AST, so it is derivable
// before rustc. The permanent form extends this to also walk `HinputsT` for *exported* Vale
// types/traits/closures (Vale→Rust decls, §6.4); a consumer driver like NobiliaV's hits none of those.

use bumpalo::Bump;

use crate::code_source::{CodeSource, Source};
use crate::compile_options::GlobalOptions;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::postparsing::ast::{IFunctionAttributeS, ProgramS};
use crate::postparsing::names::IFunctionDeclarationNameS;
use crate::postparsing::ScoutCompilation;
use crate::scout_arena::ScoutArena;
use crate::typing::rust_interop::RUST_MODULE;
use crate::utils::code_hierarchy::FileCoordinateMap;

/// A shape the interim generator cannot yet express (the permanent, `HinputsT`-driven form does).
#[derive(Debug, Clone)]
pub enum StubGenError {
  /// A `rust` import whose path names no crate (`rust.<crate>...` is required). Carries the rendering.
  ImportMissingCrate(String),
  /// An exported item whose declaration name is not a plain function name (a root the seed can't emit).
  UnsupportedExportedName(String),
}

impl std::fmt::Display for StubGenError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StubGenError::ImportMissingCrate(path) => {
        write!(f, "a `rust` import must name a crate (rust.<crate>...): got `{path}`")
      }
      StubGenError::UnsupportedExportedName(name) => {
        write!(f, "vale-stub-gen cannot yet emit a stub root for exported non-function `{name}`")
      }
    }
  }
}

/// Generate the stub crate source for one scouted Vale program.
///
/// Emits, in the order the hand-written `fixtures/stub.rs` uses: the `register_tool(vale)` header, an
/// `extern crate` per distinct imported crate, `use std::process::exit;` (only when a `main` is
/// exported), one `pub use` per `import rust.X.Y` (@RTMEIZ), the `__VALE_STUBS_MARKER`, a
/// `#[vale::emit_consumer_body]` root per exported func, the `fn main` bin shim (when `main` is
/// exported), and the generic `__vale_drop<T>` shim.
pub fn generate_stub_source(program: &ProgramS) -> Result<String, StubGenError> {
  let mut extern_crates: Vec<String> = Vec::new();
  let mut pub_uses: Vec<String> = Vec::new();
  for import in program.imports {
    if import.module_name.0 != RUST_MODULE {
      continue;
    }
    let crate_name = match import.package_names.first() {
      Some(name) => name.0,
      None => return Err(StubGenError::ImportMissingCrate(render_import(import))),
    };
    if !extern_crates.iter().any(|existing| existing == crate_name) {
      extern_crates.push(crate_name.to_string());
    }
    // The Rust path is crate :: (middle module segments) :: item. `package_names` is [crate, ..mods],
    // and `importee_name` is the item; joining them all with `::` yields exactly that.
    let mut segments: Vec<&str> = import.package_names.iter().map(|s| s.0).collect();
    segments.push(import.importee_name.0);
    pub_uses.push(segments.join("::"));
  }

  let mut exported_fn_names: Vec<String> = Vec::new();
  let mut has_main = false;
  for func in program.implemented_functions {
    let is_exported =
      func.attributes.iter().any(|attr| matches!(attr, IFunctionAttributeS::Export(_)));
    if !is_exported {
      continue;
    }
    let name = match &func.name {
      IFunctionDeclarationNameS::FunctionName(n) => n.imprecise_name.name.0,
      other => return Err(StubGenError::UnsupportedExportedName(format!("{other:?}"))),
    };
    if name == "main" {
      has_main = true;
    }
    exported_fn_names.push(name.to_string());
  }

  let mut out = String::new();
  out.push_str("#![feature(register_tool)]\n#![register_tool(vale)]\n\n");
  for crate_name in &extern_crates {
    out.push_str(&format!("extern crate {crate_name};\n"));
  }
  if has_main {
    out.push_str("\nuse std::process::exit;\n");
  }
  out.push('\n');
  for path in &pub_uses {
    out.push_str(&format!("pub use {path};\n"));
  }
  out.push_str("\npub const __VALE_STUBS_MARKER: () = ();\n\n");
  for name in &exported_fn_names {
    out.push_str(&format!(
      "#[vale::emit_consumer_body]\npub fn __vale_{name}() -> i32 {{\n    unreachable!()\n}}\n\n"
    ));
  }
  if has_main {
    out.push_str("fn main() {\n    exit(__vale_main());\n}\n\n");
  }
  out.push_str(
    "#[inline(never)]\npub unsafe fn __vale_drop<T>(x: *mut T) {\n    core::ptr::drop_in_place(x)\n}\n",
  );
  Ok(out)
}

/// Scout `vale_source` and generate its stub crate source. The scout needs no rustc — it runs on the
/// Vale text alone — so the stub exists before the rustc invocation that compiles it. Errors on a scout
/// failure or a shape the seed can't express. Interim: exactly one Vale file.
pub fn generate_stub_source_from_vale(vale_source: &str) -> Result<String, String> {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);

  let package_coord = parse_arena.intern_package_coordinate(parse_arena.intern_str("test"), &[]);
  let mut files = FileCoordinateMap::<String>::new();
  files.put(parse_arena.intern_file_coordinate(package_coord, "0.vale"), vale_source.to_string());
  let code_source = CodeSource::new(vec![Source::from_code_map(&files)]);

  let global_options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: true,
    debug_output: false,
  };

  let mut scout = ScoutCompilation::new(
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    vec![package_coord],
    &code_source,
    global_options,
  );
  let scoutput =
    scout.get_scoutput().map_err(|e| format!("scouting the Vale program failed: {e:?}"))?;
  let programs: Vec<&ProgramS> = scoutput.file_coord_to_contents.values().collect();
  match programs.as_slice() {
    [program] => generate_stub_source(program).map_err(|e| e.to_string()),
    other => Err(format!("expected exactly one Vale file to scout, found {}", other.len())),
  }
}

fn render_import(import: &crate::postparsing::ast::ImportS) -> String {
  let mut segments: Vec<&str> = vec![import.module_name.0];
  segments.extend(import.package_names.iter().map(|s| s.0));
  segments.push(import.importee_name.0);
  segments.join(".")
}
