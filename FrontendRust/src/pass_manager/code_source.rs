// Onion arc: `Source::builtins` / `Source::builtin_module` / `Source::Inputs`
// depend on `builtins::` and `pass_manager::pass_manager::` which are unlinked
// during this arc. The `CodeMap` and `Fn` variants are the only ones the
// currently-linked pipeline (lex_and_explore → parse_and_explore → parse →
// postparse) needs, so the others are gated out until higher_typing and its
// dependencies come back.
use crate::utils::code_hierarchy::{FileCoordinateMap, PackageCoordinate};
use crate::utils::fx::HashMap;

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
