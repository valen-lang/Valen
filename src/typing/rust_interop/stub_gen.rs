// vale-stub-gen (the seed): emit a Rust stub crate's source from a parsed Vale program.
//
// This is the real, permanent mechanism (arch §6.4, @RTMEIZ §26.9), not a throwaway — the eventual
// per-project cargo-workspace pipeline calls this same generator. Today the only stubs are hand-written
// fixtures (`fixtures/stub.rs`, which calls itself "the stand-in for the eventual vale-stub-gen
// output"); this replaces that hand-writing for the driven path.
//
// It is driven by the program's *parsed* structure (not a text scan, not `HinputsT`): the load-bearing
// stub content for a consumer program — one `pub use` per `import rust.X.Y` (@RTMEIZ), the marker, and a
// `#[vale::emit_consumer_body]` root per exported func — all lives in the parsed AST, so it is derivable
// before rustc. The permanent form extends this to also walk `HinputsT` for *exported* Vale
// types/traits/closures (Vale→Rust decls, §6.4); a consumer driver like NobiliaV's hits none of those.

use bumpalo::Bump;

use crate::code_source::{CodeSource, Source};
use crate::compile_options::GlobalOptions;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::parsing::ast::ast::{FileP, FunctionP, IAttributeP, IDenizenP, ImplP, ImportP, StructP};
use crate::parsing::ast::pattern::ParameterP;
use crate::parsing::ast::templex::{BorrowRefPT, EffectP, GroupP, ITemplexPT, RegionP};
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
  /// A Vale struct implementing a Rust trait carries data members; the interim projection emits a ZST,
  /// so a data-carrying callback struct (which would change the crossing ABI) is not expressible yet.
  DataCarryingCallbackStruct(String),
  /// A callback override's parameter or return type is a shape the interim renderer can't lower to Rust.
  UnsupportedCallbackType(String),
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
      StubGenError::DataCarryingCallbackStruct(name) => {
        write!(f, "vale-stub-gen cannot yet project a data-carrying callback struct `{name}` (only ZSTs)")
      }
      StubGenError::UnsupportedCallbackType(rendering) => {
        write!(f, "vale-stub-gen cannot yet lower this callback type to Rust: `{rendering}`")
      }
    }
  }
}

/// Generate the stub crate source for one parsed Vale program (`FileP`).
///
/// Reads the load-bearing shape straight off the parse tree — the `rust.X.Y` imports, the exported
/// func names, and each Vale struct that implements an imported Rust trait — since none needs name
/// resolution or typing (the permanent form adds the `HinputsT`-driven exported *declarations* on top).
/// Emits, in the order the hand-written `fixtures/stub.rs` uses: the `register_tool(vale)` header, an
/// `extern crate` per distinct imported crate, `use std::process::exit;` (only when a `main` is
/// exported), one `pub use` per `import rust.X.Y` (@RTMEIZ), the `__VALE_STUBS_MARKER`, the
/// reverse-direction projection (a `pub struct` + `impl <ImportedTrait> for <Struct>` with a
/// `#[vale::emit_consumer_body]` body per override, so rustc can monomorphize the generic caller over
/// the Vale struct and reach the override Valen's backend fills), a `#[vale::emit_consumer_body]` root
/// per exported func, the `fn main` bin shim (when `main` is exported), and the `__vale_drop<T>` shim.
pub fn generate_stub_source(file: &FileP) -> Result<String, StubGenError> {
  let mut extern_crates: Vec<String> = Vec::new();
  let mut pub_uses: Vec<String> = Vec::new();
  let mut exported_fn_names: Vec<String> = Vec::new();
  let mut has_main = false;
  // The reverse-direction projection. A Vale struct that `impl`s an imported Rust trait must appear in
  // the stub as a real Rust type + trait impl so rustc can monomorphize the generic caller over it and
  // walk to the override body (which the Valen backend fills). Collected here, rendered after the marker.
  let mut imported_item_names: Vec<&str> = Vec::new();
  let mut structs_with_data: Vec<&str> = Vec::new();
  let mut trait_impls: Vec<(&str, &str)> = Vec::new(); // (trait, struct), file order
  let mut override_methods: Vec<(&str, String)> = Vec::new(); // (struct, rendered method), file order
  for denizen in file.denizens {
    match denizen {
      IDenizenP::TopLevelImport(import) => {
        if import.module_name.as_str() != RUST_MODULE {
          continue;
        }
        let crate_name = match import.package_steps.first() {
          Some(name) => name.as_str(),
          None => return Err(StubGenError::ImportMissingCrate(render_import(import))),
        };
        if !extern_crates.iter().any(|existing| existing == crate_name) {
          extern_crates.push(crate_name.to_string());
        }
        // The Rust path is crate :: (middle module segments) :: item. `package_steps` is [crate, ..mods],
        // and `importee_name` is the item; joining them all with `::` yields exactly that.
        let mut segments: Vec<&str> = import.package_steps.iter().map(|s| s.as_str()).collect();
        segments.push(import.importee_name.as_str());
        pub_uses.push(segments.join("::"));
        imported_item_names.push(import.importee_name.as_str());
      }
      IDenizenP::TopLevelFunction(func) => {
        let is_exported = func
          .header
          .attributes
          .iter()
          .any(|attr| matches!(attr, IAttributeP::ExportAttribute(_)));
        if is_exported {
          // A top-level function always parses with a name; an anonymous one (a lambda) is never a
          // top-level denizen, so `None` here is the shape the seed cannot emit a root for.
          let name = match &func.header.name {
            Some(name) => name.as_str(),
            None => return Err(StubGenError::UnsupportedExportedName("<anonymous>".to_string())),
          };
          if name == "main" {
            has_main = true;
          }
          exported_fn_names.push(name.to_string());
        }
        // A top-level function whose first parameter is a `self &Struct` receiver is a method/override
        // of that struct; project it into the struct's trait impl (rendered only if that struct impls
        // an imported trait). Methods and exported roots are disjoint here — a trait override is not
        // exported, and `main` has no receiver.
        if let Some(struct_name) = self_receiver_struct(func) {
          override_methods.push((struct_name, render_impl_method(func)?));
        }
      }
      IDenizenP::TopLevelStruct(s) => {
        if struct_has_data_members(s) {
          structs_with_data.push(s.name.as_str());
        }
      }
      IDenizenP::TopLevelImpl(imp) => {
        // `impl <Interface> for <Struct>;` — the interface is required, the struct optional (a struct-body
        // `impl X;` omits it). Only a top-level impl naming both plain-name types projects a callback.
        if let (Some(trait_name), Some(struct_name)) =
          (templex_name(&imp.interface), imp.struct_.as_ref().and_then(templex_name))
        {
          trait_impls.push((trait_name, struct_name));
        }
      }
      _ => {}
    }
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
  // Reverse-direction projection: one `pub struct` + `impl <ImportedTrait> for <Struct>` per Vale struct
  // that implements an imported trait, its override bodies deferred to Valen (single-symbol, arch §5.2).
  for (trait_name, struct_name) in trait_impls.iter().copied() {
    // Only imported traits are projected; a Vale-native interface impl is codegenned by Valen itself.
    if !imported_item_names.contains(&trait_name) {
      continue;
    }
    if structs_with_data.contains(&struct_name) {
      return Err(StubGenError::DataCarryingCallbackStruct(struct_name.to_string()));
    }
    out.push_str(&format!("pub struct {struct_name} {{}}\n\n"));
    out.push_str(&format!("impl {trait_name} for {struct_name} {{\n"));
    for (owner, method) in &override_methods {
      if *owner == struct_name {
        out.push_str(method);
      }
    }
    out.push_str("}\n\n");
  }
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

/// The bare type name of a templex if it is a plain `NameOrRune` (`MyCb`, `NobiliaWindow`); `None` for
/// any compound shape (a call, a ref, a tuple). Used to read the struct/interface named in an `impl`.
fn templex_name<'p>(t: &ITemplexPT<'p>) -> Option<&'p str> {
  match t {
    ITemplexPT::NameOrRune(n) => Some(n.name.as_str()),
    _ => None,
  }
}

/// Peel borrow/own/weak wrappers off a templex and return the underlying plain type name, if any. A
/// top-level method writes its receiver as `self &Struct` / `self Struct`, so the struct it belongs to
/// is the peeled name of the first parameter's type.
fn peel_ref_to_name<'p>(t: &ITemplexPT<'p>) -> Option<&'p str> {
  match t {
    ITemplexPT::NameOrRune(n) => Some(n.name.as_str()),
    ITemplexPT::BorrowRef(b) => peel_ref_to_name(b.inner),
    ITemplexPT::OwnRef(r) => peel_ref_to_name(r.inner),
    ITemplexPT::WeakRef(r) => peel_ref_to_name(r.inner),
    _ => None,
  }
}

/// The local name a parameter binds, if it binds a plain local (`self`, `w`, …); `None` for an ignored
/// or destructuring pattern.
fn param_local_name<'p>(param: &ParameterP<'p>) -> Option<&'p str> {
  use crate::parsing::ast::pattern::INameDeclarationP;
  let dest = param.pattern.as_ref()?.destination?;
  match dest.decl {
    INameDeclarationP::LocalNameDeclaration(n) => Some(n.as_str()),
    _ => None,
  }
}

/// Whether a parameter is the `self` receiver: either the bare `&self` form or a parameter named `self`.
fn is_self_param(param: &ParameterP) -> bool {
  param.self_borrow.is_some() || param_local_name(param) == Some("self")
}

/// If `func`'s first parameter is a `self` receiver naming a struct (`self &Struct` / `self Struct`),
/// the struct this top-level function is a method/override of.
fn self_receiver_struct<'p>(func: &FunctionP<'p>) -> Option<&'p str> {
  let params = func.header.params.as_ref()?.params;
  let first = params.first()?;
  if !is_self_param(first) {
    return None;
  }
  peel_ref_to_name(first.pattern.as_ref()?.templex.as_ref()?)
}

/// Whether a struct carries data members (as opposed to only methods). The interim projection emits a
/// ZST, so a data-carrying callback struct is rejected rather than silently sized wrong.
fn struct_has_data_members(s: &StructP) -> bool {
  use crate::parsing::ast::ast::IStructContent;
  s.members.contents.iter().any(|c| {
    matches!(c, IStructContent::NormalStructMember(_) | IStructContent::VariadicStructMember(_))
  })
}

/// Whether a borrow's region is declared mutable — i.e. its `in g` names a region in the override's
/// `mut(g)` set. Only a top-level `in g` region is consulted; a nested borrow carries no such annotation.
fn borrow_region_is_mut(b: &BorrowRefPT, mut_regions: &[&str]) -> bool {
  match b.region {
    RegionP::Group(GroupP::Name(name)) => mut_regions.contains(&name.as_str()),
    _ => false,
  }
}

/// A parameter's Rust type rendering. A top-level borrow whose region is `mut` renders `&mut T`,
/// matching the Rust trait's `&mut`; every other shape defers to the shared `render_rust_type`. The
/// mut-ness is a property of *this* borrow's region only — nested borrows are rendered shared, as Rust
/// elision gives us no inner-mutability signal here.
fn render_param_type(t: &ITemplexPT, mut_regions: &[&str]) -> Result<String, StubGenError> {
  if let ITemplexPT::BorrowRef(b) = t {
    if borrow_region_is_mut(b, mut_regions) {
      return Ok(format!("&mut {}", render_rust_type(b.inner)?));
    }
  }
  render_rust_type(t)
}

/// Lower a Vale type templex to its Rust rendering for a projected override signature. Handles the
/// shapes a callback boundary uses today — a plain scalar/imported-type name and a shared borrow of one
/// — and errors on anything else (the interim renderer, not the permanent `HinputsT`-driven form). The
/// `&mut` decision lives in `render_param_type`/`render_receiver`, which read the override's `mut(g)`
/// set; this renders the shared `&T` and the inner types.
fn render_rust_type(t: &ITemplexPT) -> Result<String, StubGenError> {
  match t {
    ITemplexPT::NameOrRune(n) => Ok(map_scalar_name(n.name.as_str())),
    ITemplexPT::BorrowRef(b) => Ok(format!("&{}", render_rust_type(b.inner)?)),
    other => Err(StubGenError::UnsupportedCallbackType(format!("{other:?}"))),
  }
}

/// Map a Vale type name to its Rust counterpart: the scalars a boundary crosses become their rustc
/// primitives, and an imported type (`NobiliaWindow`, `FrameInput`) keeps its name (the stub `pub use`s
/// or projects it). Vale `int` is the 32-bit `i32` these boundaries use (matching the hand-written stubs).
fn map_scalar_name(name: &str) -> String {
  match name {
    "int" => "i32".to_string(),
    "bool" => "bool".to_string(),
    other => other.to_string(),
  }
}

/// Render one Vale override function as a Rust trait-impl method whose body Valen's backend fills:
/// `#[vale::emit_consumer_body] fn <name>(<receiver>, <params>) <-> ret> { unreachable!() }`. The `self`
/// parameter becomes the receiver; every other parameter renders as `_<name>: <rust type>` (underscored
/// since the `unreachable!()` body uses none). Errors on a shape the interim renderer can't express.
fn render_impl_method(func: &FunctionP) -> Result<String, StubGenError> {
  let name = func
    .header
    .name
    .map(|n| n.as_str())
    .ok_or_else(|| StubGenError::UnsupportedCallbackType("anonymous override".to_string()))?;
  let params = func.header.params.as_ref().map(|p| p.params).unwrap_or(&[]);
  // The regions this override declares mutable, read from its `mut(g)` effect clauses. A borrow (a
  // parameter's or the receiver's) whose `in g` region is in this set renders `&mut` — the only place
  // Vale records mutability, since it lives on the effect clause and the region, never on the `&T` type.
  // A `Vec` (not a set) is enough — a handful of regions — and stays deterministic.
  let mut_regions: Vec<&str> = func
    .header
    .effects
    .iter()
    .filter_map(|effect| match effect {
      EffectP::Mut(GroupP::Name(name)) => Some(name.as_str()),
      _ => None,
    })
    .collect();
  let mut rendered: Vec<String> = Vec::new();
  for (i, param) in params.iter().enumerate() {
    if i == 0 && is_self_param(param) {
      rendered.push(render_receiver(param, &mut_regions)?);
    } else {
      let pname = param_local_name(param).unwrap_or("arg");
      let templex = param.pattern.as_ref().and_then(|p| p.templex.as_ref()).ok_or_else(|| {
        StubGenError::UnsupportedCallbackType(format!("parameter `{pname}` has no type"))
      })?;
      rendered.push(format!("_{pname}: {}", render_param_type(templex, &mut_regions)?));
    }
  }
  let ret = match &func.header.ret.ret_type {
    Some(t) => format!(" -> {}", render_rust_type(t)?),
    None => String::new(),
  };
  Ok(format!(
    "    #[vale::emit_consumer_body]\n    fn {name}({}){ret} {{\n        unreachable!()\n    }}\n",
    rendered.join(", ")
  ))
}

/// Render a `self` receiver as its Rust form. A borrow receiver whose region is `mut` (`self &Struct in
/// s` with `mut(s)`) is `&mut self`, matching a `&mut self` Rust trait method; an unmutated borrow
/// receiver (`self &Struct` or the bare `&self`) is `&self`; a by-value one (`self Struct`) is `self`. A
/// weak/other shape is not expressible yet. The bare `&self` form carries no region, so it is shared.
fn render_receiver(param: &ParameterP, mut_regions: &[&str]) -> Result<String, StubGenError> {
  if param.self_borrow.is_some() {
    return Ok("&self".to_string());
  }
  match param.pattern.as_ref().and_then(|p| p.templex.as_ref()) {
    Some(ITemplexPT::BorrowRef(b)) if borrow_region_is_mut(b, mut_regions) => {
      Ok("&mut self".to_string())
    }
    Some(ITemplexPT::BorrowRef(_)) => Ok("&self".to_string()),
    Some(ITemplexPT::NameOrRune(_)) | Some(ITemplexPT::OwnRef(_)) => Ok("self".to_string()),
    other => {
      Err(StubGenError::UnsupportedCallbackType(format!("self receiver shape {other:?}")))
    }
  }
}

/// Parse `vale_source` and generate its stub crate source. The parser needs no rustc — it runs on the
/// Vale text alone — so the stub exists before the rustc invocation that compiles it. Errors on a parse
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
  let parseds =
    scout.get_parseds().map_err(|e| format!("parsing the Vale program failed: {e:?}"))?;
  let files: Vec<&FileP> = parseds.file_coord_to_contents.values().map(|(file, _)| file).collect();
  match files.as_slice() {
    [file] => generate_stub_source(file).map_err(|e| e.to_string()),
    other => Err(format!("expected exactly one Vale file to parse, found {}", other.len())),
  }
}

fn render_import(import: &ImportP) -> String {
  let mut segments: Vec<&str> = vec![import.module_name.as_str()];
  segments.extend(import.package_steps.iter().map(|s| s.as_str()));
  segments.push(import.importee_name.as_str());
  segments.join(".")
}
