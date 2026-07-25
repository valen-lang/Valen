// VCOORD:
// Onion arc: `Source::Inputs` depends on `pass_manager::pass_manager::` which
// stays gated during this arc, so its variant + `resolve` arm are still out.
// `Source::builtins` / `Source::builtin_module` came back once `builtins::`
// re-linked.
use crate::builtins::builtins::{builtin_module_code_map, get_code_map as get_builtins_code_map};
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::utils::code_hierarchy::{FileCoordinateMap, PackageCoordinate};
use crate::utils::fx::HashMap;
#[cfg(feature = "rust_interop")]
use crate::typing::rust_interop::RUST_MODULE;

pub type SourceFn = for<'r, 's> fn(&'r PackageCoordinate<'s>) -> Option<HashMap<String, String>>;

/// One layer of a `CodeSource`. Each variant knows how to answer resolution
/// requests for its own slice of package-coord space; layers are expected to
/// be disjoint, and `CodeSource::resolve` returns the first hit.
pub enum Source<'a> {
  /// A fixed code map: package → filename → contents. Cheapest lookup.
  CodeMap(HashMap<&'a PackageCoordinate<'a>, HashMap<String, String>>),
  /// An escape hatch for anything computed at resolve-time (test resource
  /// loaders, targeted stubs, etc.).
  Fn(SourceFn),
}

impl<'a> Source<'a> {
  /// Build a `CodeMap` source from a `FileCoordinateMap<String>`.
  pub fn from_code_map(map: &FileCoordinateMap<'a, String>) -> Self {
    Source::CodeMap(flatten_code_map(map))
  }

  /// Build a `CodeMap` source holding the compiler's built-in vale sources.
  pub fn builtins<'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
  ) -> Self
  where
    'a: 'ctx,
  {
    let map = get_builtins_code_map(parse_arena, keywords);
    Source::CodeMap(flatten_code_map(&map))
  }

  /// Build a `CodeMap` source holding one specific builtin module (keyed at
  /// `("v", ["builtins", name])`). Tests use this to declare exactly which
  /// builtin content their code actually reaches, paired with
  /// `empty_v_builtins_stub` as a fallback for anything transitively walked.
  pub fn builtin_module<'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
    name: &str,
  ) -> Self
  where
    'a: 'ctx,
  {
    let map = builtin_module_code_map(parse_arena, keywords, name);
    Source::CodeMap(flatten_code_map(&map))
  }

  /// The slice of package-coord space owned by the reserved `rust` package.
  ///
  /// Add this layer to a `CodeSource` when the program being compiled contains
  /// `import rust.X.Y`; leave it out otherwise. Like every other layer it is the
  /// caller's to declare, a project with no Rust dependencies should not carry it.
  #[cfg(feature = "rust_interop")]
  pub fn rust() -> Self {
    Source::Fn(resolve_rust_package)
  }
}

pub struct CodeSource<'a> {
  sources: Vec<Source<'a>>,
}

impl<'a> CodeSource<'a> {
  pub fn new(sources: Vec<Source<'a>>) -> Self {
    CodeSource { sources }
  }

  pub fn resolve(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<HashMap<String, String>> {
    for source in &self.sources {
      let hit = match source {
        Source::CodeMap(m) => m.get(package_coord).cloned(),
        Source::Fn(f) => f(package_coord),
      };
      if hit.is_some() {
        return hit;
      }
    }
    None
  }
}

/// Resolves any package in the reserved `rust` module to an empty file set.
///
/// A Rust package contributes no `.vale` files, so there is nothing to lex, but
/// resolution still has to succeed, or `lex_and_explore` panics on the import long
/// before typing runs. Which *items* live in the package is a separate question,
/// answered later and lazily by the oracle rather than by reading source here.
///
/// An empty file set is an already-exercised shape: `flatten_code_map` produces one
/// for any package with no files, and `integration_tests/tests/import_tests.rs`
/// covers importing a package that has none.
#[cfg(feature = "rust_interop")]
fn resolve_rust_package<'r, 's>(
  package_coord: &'r PackageCoordinate<'s>,
) -> Option<HashMap<String, String>> {
  if package_coord.module.0 == RUST_MODULE {
    Some(HashMap::default())
  } else {
    None
  }
}

fn flatten_code_map<'a>(
  map: &FileCoordinateMap<'a, String>,
) -> HashMap<&'a PackageCoordinate<'a>, HashMap<String, String>> {
  let mut result: HashMap<&'a PackageCoordinate<'a>, HashMap<String, String>> =
    HashMap::default();
  for (package_coord, file_coords) in &map.package_coord_to_file_coords {
    let mut file_map = HashMap::default();
    for fc in file_coords {
      let contents = map
        .file_coord_to_contents
        .get(fc)
        .expect("flatten_code_map - file coord missing");
      file_map.insert(fc.filepath.as_str().to_string(), contents.clone());
    }
    result.insert(*package_coord, file_map);
  }
  result
}
