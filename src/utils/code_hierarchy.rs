
use crate::utils::fx::IndexMap;
use crate::utils::fx::HashMap;
use crate::interner::{InternedSlice, StrI};
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::Keywords;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileCoordinate<'a> {
  pub package_coord: &'a PackageCoordinate<'a>,
  pub filepath: StrI<'a>,
}
impl<'a> FileCoordinate<'a> {

  pub fn is_internal(&self) -> bool {
    self.package_coord.is_internal()
  }

  pub fn is_test(&self) -> bool {
    self.package_coord.is_test() && self.filepath == "test.vale"
  }

  pub fn eq_by_value<'b>(&self, other: &FileCoordinate<'b>) -> bool {
    self.filepath.as_str() == other.filepath.as_str()
      && self.package_coord.eq_by_value(other.package_coord)
  }


  pub fn test(scout_arena: &ScoutArena<'a>) -> FileCoordinate<'a> {
    let test_module = scout_arena.intern_str(TEST_MODULE);
    let package_coord = scout_arena.intern_package_coordinate(test_module, &[]);
    *scout_arena.intern_file_coordinate(package_coord, "test.vale")
  }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PackageCoordinate<'a> {
  pub module: StrI<'a>,
  pub packages: InternedSlice<'a, StrI<'a>>,
}
impl<'a> PackageCoordinate<'a> {

  pub fn is_internal(&self) -> bool {
    self.module == ""
  }

  pub fn is_test(&self) -> bool {
    self.module == TEST_MODULE && self.packages.is_empty()
  }

  pub fn eq_by_value<'b>(&self, other: &PackageCoordinate<'b>) -> bool {
    self.module.as_str() == other.module.as_str()
      && self.packages.as_slice().len() == other.packages.as_slice().len()
      && self.packages.as_slice().iter().zip(other.packages.as_slice().iter())
          .all(|(a, b)| a.as_str() == b.as_str())
  }


  pub fn test_tld<'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    _keywords: &'ctx Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    parse_arena.intern_package_coordinate(parse_arena.intern_str(TEST_MODULE), &[])
  }


  pub fn builtin<'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    parse_arena.intern_package_coordinate(keywords.empty_string, &[])
  }


  pub fn internal(
    scout_arena: &ScoutArena<'a>,
    keywords: &Keywords<'a>,
  ) -> PackageCoordinate<'a> {
    *scout_arena.intern_package_coordinate(keywords.empty_string, &[])
  }

}
impl<'a> Display for PackageCoordinate<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "PackageCoordinate({},Vector(", self.module)?;
    let mut first = true;
    for pkg in self.packages.as_slice() {
      if !first { write!(f, ", ")?; }
      write!(f, "{}", pkg)?;
      first = false;
    }
    write!(f, "))")
  }
}

const TEST_MODULE: &str = "test";

#[derive(Clone, Debug)]
pub struct FileCoordinateMap<'a, Contents> {
  pub package_coord_to_file_coords: HashMap<&'a PackageCoordinate<'a>, Vec<&'a FileCoordinate<'a>>>,
  // Per @IIIOZ, the typing pass iterates this to seed its environment, so it is an IndexMap:
  // insertion-ordered iteration keeps the resulting denizen order stable across runs.
  pub file_coord_to_contents: IndexMap<&'a FileCoordinate<'a>, Contents>,
}
impl<'a, Contents: Clone> FileCoordinateMap<'a, Contents> {

  pub fn new() -> Self {
    FileCoordinateMap {
      package_coord_to_file_coords: HashMap::default(),
      file_coord_to_contents: IndexMap::default(),
    }
  }

  pub fn apply(&self, coord: &'a FileCoordinate<'a>) -> &Contents {
    self
      .file_coord_to_contents
      .get(coord)
      .expect("FileCoordinateMap::apply - coordinate not found")
  }

  pub fn get_by_value(&self, coord: &FileCoordinate<'_>) -> Option<&Contents> {
    self.file_coord_to_contents.iter()
      .find(|(k, _)| k.eq_by_value(coord))
      .map(|(_, v)| v)
  }


  // This is different from put in that we can hand in an empty map here.
  // It's the only way to have an empty package in the FileCoordinateMap.
  pub fn put_package(
    &mut self,
    package_coord: &'a PackageCoordinate<'a>,
    new_file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, Contents>,
  ) {
    let file_coords: Vec<&'a FileCoordinate<'a>> =
      new_file_coord_to_contents.keys().cloned().collect();
    self
      .package_coord_to_file_coords
      .insert(package_coord, file_coords);

    for (file_coord, contents) in new_file_coord_to_contents {
      self.file_coord_to_contents.insert(file_coord, contents);
    }
  }


  pub fn put(&mut self, file_coord: &'a FileCoordinate<'a>, contents: Contents) {
    assert!(
      !self.file_coord_to_contents.contains_key(&file_coord),
      "FileCoordinateMap::put - file coordinate already exists"
    );

    self
      .file_coord_to_contents
      .insert(file_coord, contents.clone());

    let package_coord = file_coord.package_coord;
    let file_coords = self
      .package_coord_to_file_coords
      .entry(package_coord)
      .or_insert_with(Vec::new);
    file_coords.push(file_coord);
  }


  pub fn map<T, F>(&self, func: F) -> FileCoordinateMap<'a, T>
  where
    F: Fn(&'a FileCoordinate<'a>, &Contents) -> T,
    T: Clone,
  {
    let mut result_file_coord_to_contents: IndexMap<&'a FileCoordinate<'a>, T> = IndexMap::default();
    for (file_coord, contents) in &self.file_coord_to_contents {
      result_file_coord_to_contents.insert(file_coord, func(file_coord, contents));
    }
    FileCoordinateMap {
      package_coord_to_file_coords: self.package_coord_to_file_coords.clone(),
      file_coord_to_contents: result_file_coord_to_contents,
    }
  }


  pub fn flat_map<T, F>(&self, func: F) -> Vec<T>
  where
    F: Fn(&'a FileCoordinate<'a>, &Contents) -> T,
  {
    self
      .file_coord_to_contents
      .iter()
      .map(|(file_coord, contents)| func(file_coord, contents))
      .collect()
  }


  pub fn expect_one(&self) -> &Contents {
    assert!(
      self.file_coord_to_contents.len() == 1,
      "FileCoordinateMap::expect_one - expected exactly one entry"
    );
    self.file_coord_to_contents.values().next().unwrap()
  }


}

#[derive(Clone, Debug)]
pub struct PackageCoordinateMap<'a, Contents> {
  pub package_coord_to_contents: IndexMap<&'a PackageCoordinate<'a>, Contents>,
}

impl<'a, Contents> PackageCoordinateMap<'a, Contents> {

  pub fn new() -> Self {
    PackageCoordinateMap {
      package_coord_to_contents: IndexMap::default(),
    }
  }


  pub fn put(&mut self, package_coord: &'a PackageCoordinate<'a>, contents: Contents) {
    self.package_coord_to_contents.insert(package_coord, contents);
  }


  pub fn get(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<&Contents> {
    self.package_coord_to_contents.get(package_coord)
  }


  pub fn expect_one(&self) -> &Contents {
    assert!(
      self.package_coord_to_contents.len() == 1,
      "PackageCoordinateMap::expect_one - expected exactly one entry"
    );
    self.package_coord_to_contents.values().next().unwrap()
  }


  pub fn map<T, F>(&self, func: F) -> PackageCoordinateMap<'a, T>
  where
    F: Fn(&'a PackageCoordinate<'a>, &Contents) -> T,
    T: Clone,
  {
    let mut result = PackageCoordinateMap::new();
    for (package_coord, contents) in &self.package_coord_to_contents {
      result.put(package_coord, func(package_coord, contents));
    }
    result
  }


  pub fn flat_map<T, F>(&self, func: F) -> Vec<T>
  where
    F: Fn(&'a PackageCoordinate<'a>, &Contents) -> T,
  {
    self
      .package_coord_to_contents
      .iter()
      .map(|(package_coord, contents)| func(package_coord, contents))
      .collect()
  }

}
