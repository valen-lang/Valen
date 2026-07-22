
use crate::parse_arena::ParseArena;
use crate::pass_manager::Source;
use crate::scout_arena::ScoutArena;
use crate::utils::code_hierarchy::{FileCoordinateMap, PackageCoordinate};
use crate::utils::fx::HashMap;
use std::fs::File;
use std::io::read_to_string;
use std::path::PathBuf;

const TEST_MODULE: &str = "test";

/// Build a test code map from a single code string. The file is named
/// `0.vale`. Accepts `&str` or `String`.
/// A "test code map" is just a code map whose package is hardcoded to
/// `("test", [])`.
pub fn new_test_code_map<'a, 'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    code: impl Into<String>,
  ) -> Source<'a>
  where 'a: 'ctx,
  {
    let mut map = HashMap::default();
    map.insert("0.vale".to_string(), code.into());
    new_test_code_map_from_files(parse_arena, map)
  }

/// Build a test code map from a filename→contents map. Caller controls the
/// filenames (e.g. `"test.vale"`).
/// A "test code map" is just a code map whose package is hardcoded to
/// `("test", [])`.
pub fn new_test_code_map_from_files<'a, 'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    contents: HashMap<String, String>,
  ) -> Source<'a>
  where 'a: 'ctx,
  {
    let mut result = FileCoordinateMap::<'a, String>::new();
    let package_coord = parse_arena.intern_package_coordinate(parse_arena.intern_str(TEST_MODULE), &[]);
    for (filepath, file_contents) in contents {
      let file_coord = parse_arena.intern_file_coordinate(package_coord, &filepath);
      result.put(file_coord, file_contents);
    }
    Source::from_code_map(&result)
  }

/// Build a `FileCoordinateMap<String>` for the "test" package holding a single
/// file `"test.vale"` with `contents`. Used by humanizer-shaped tests to feed
/// `humanize_pos_code_map`, `lines_between`, etc. — they need a code map to
/// render source snippets, but the specific contents don't matter for what
/// they're checking.
pub fn new_humanizer_test_code_map<'a>(
    scout_arena: &ScoutArena<'a>,
    contents: impl Into<String>,
  ) -> FileCoordinateMap<'a, String> {
    let test_module = scout_arena.intern_str(TEST_MODULE);
    let package_coord = scout_arena.intern_package_coordinate(test_module, &[]);
    let file_coord = scout_arena.intern_file_coordinate(package_coord, "test.vale");
    let mut result = FileCoordinateMap::new();
    result.put(file_coord, contents.into());
    result
  }

pub fn load(resource_filename: &str) -> Option<String> {
  let full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("src/tests")
    .join(resource_filename);
  let stream = File::open(&full_path);
  if stream.is_err() {
    return None;
  }
  let stream = stream.unwrap();
  let source = read_to_string(stream).unwrap();
  Some(source)
}

pub fn load_expected(resource_filename: &str) -> String {
  load(resource_filename)
    .unwrap_or_else(|| panic!("Failed to load resource: {}", resource_filename))
}

/// Build a `Source::CodeMap` for the named test package by reading
/// `FrontendRust/src/tests/<module>/<packages...>/<last>.vale` off disk.
/// `package_path` uses dotted notation: `"panicutils"` for a top-level
/// package, `"array.make"` for a subpackage.
pub fn new_test_package_source<'a>(
    parse_arena: &'a ParseArena<'a>,
    package_path: &str,
) -> Source<'a> {
    let parts: Vec<&str> = package_path.split('.').collect();
    let last = parts.last().expect("new_test_package_source - empty package_path");
    let filename = format!("{}.vale", last);
    let filepath = format!("{}/{}", parts.join("/"), filename);
    let contents = load_expected(&filepath);
    let module_stri = parse_arena.intern_str(parts[0]);
    let sub_packages: Vec<_> = parts[1..].iter().map(|s| parse_arena.intern_str(s)).collect();
    let package_coord = parse_arena.intern_package_coordinate(module_stri, &sub_packages);
    let file_coord = parse_arena.intern_file_coordinate(package_coord, &filename);
    let mut map = FileCoordinateMap::<'a, String>::new();
    map.put(file_coord, contents);
    Source::from_code_map(&map)
}

/// Resolve a `PackageCoordinate` by reading `FrontendRust/src/tests/<module>/<packages>/<last>.vale`
/// off disk. Called lazily per-coord by `Source::Fn` at resolve time —
/// only the specific packages a test's imports actually reach get read.
pub fn test_source_from_dir(package_coord: &PackageCoordinate) -> Option<HashMap<String, String>> {
  let directory: Vec<&str> = {
    let mut v = vec![package_coord.module.as_str()];
    v.extend(package_coord.packages.iter().map(|s| s.as_str()));
    v
  };
  let filename = format!("{}.vale", directory.last().unwrap());
  let filepath = {
    let mut v = directory.clone();
    v.push(&filename);
    v.join("/")
  };
  match load(&filepath) {
    None => None,
    Some(source) => {
      let mut m = HashMap::default();
      m.insert(filename, source);
      Some(m)
    }
  }
}
