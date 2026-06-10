// From Frontend/Utils/src/dev/vale/CodeHierarchy.scala

use std::collections::HashMap;
use crate::interner::{InternedSlice, StrI};
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::Keywords;
use bumpalo::Bump;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct OrResolver<P, F> {
  primary: P,
  fallback: F,
}

impl<'a, T, P, F> IPackageResolver<'a, T> for OrResolver<P, F>
where
  P: IPackageResolver<'a, T>,
  F: IPackageResolver<'a, T>,
{
  fn resolve(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<T> {
    self
      .primary
      .resolve(package_coord)
      .or_else(|| self.fallback.resolve(package_coord))
  }
}
/*
Guardian: disable-all
*/

/// Implement IPackageResolver for function pointers (for lambda-style resolvers)
impl<'a, T, F> IPackageResolver<'a, T> for F
where
  F: Fn(&'a PackageCoordinate<'a>) -> Option<T>,
{
  fn resolve(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<T> {
    self(package_coord)
  }
}
/*
Guardian: disable-all
*/

/*
package dev.vale

import scala.collection.immutable.List
import scala.collection.mutable

*/
// mig: struct FileCoordinate
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileCoordinate<'a> {
  pub package_coord: &'a PackageCoordinate<'a>,
  pub filepath: StrI<'a>,
}
// mig: impl FileCoordinate
// compareTo and compare methods were commented out in Scala (ordering not implemented)
impl<'a> FileCoordinate<'a> {
/*
case class FileCoordinate(packageCoordinate: PackageCoordinate, filepath: String) extends IInterning {
  def isInternal = packageCoordinate.isInternal
  def isTest(): Boolean = packageCoordinate.isTest && filepath == "test.vale"
//  def compareTo(that: FileCoordinate) = FileCoordinate.compare(this, that)
}

object FileCoordinate {// extends Ordering[FileCoordinate] {
*/
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

// mig: fn test
  pub fn test(scout_arena: &ScoutArena<'a>) -> FileCoordinate<'a> {
    let test_module = scout_arena.intern_str(TEST_MODULE);
    let package_coord = scout_arena.intern_package_coordinate(test_module, &[]);
    *scout_arena.intern_file_coordinate(package_coord, "test.vale")
  }
/*
  def test(interner: Interner): FileCoordinate = {
    interner.intern(FileCoordinate(
      interner.intern(PackageCoordinate(
        interner.intern(StrI("test")),
        Vector.empty)),
      "test.vale"))
  }

//  override def compare(a: FileCoordinate, b: FileCoordinate):Int = {
//    val diff = a.packageCoordinate.compareTo(b.packageCoordinate)
//    if (diff != 0) {
//      diff
//    } else {
//      a.filepath.compareTo(b.filepath)
//    }
//  }
}

*/
}
// mig: struct PackageCoordinate
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PackageCoordinate<'a> {
  pub module: StrI<'a>,
  pub packages: InternedSlice<'a, StrI<'a>>,
}
// mig: impl PackageCoordinate
// compareTo and compare methods were commented out in Scala (ordering not implemented)
impl<'a> PackageCoordinate<'a> {
/*
case class PackageCoordinate(module: StrI, packages: Vector[StrI]) extends IInterning {
  def isInternal = module.str == ""
  def isTest = module.str == "test" && packages == Vector()

//  def compareTo(that: PackageCoordinate) = PackageCoordinate.compare(this, that)
*/
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

// mig: fn parent
  pub fn parent(&self, bump: &'a Bump) -> Option<PackageCoordinate<'a>> {
    if self.packages.is_empty() {
      return None;
    }
    let parent_packages = &self.packages.as_slice()[0..self.packages.len() - 1];
    let arena_packages = bump.alloc_slice_copy(parent_packages);
    Some(PackageCoordinate {
      module: self.module,
      packages: InternedSlice::new(arena_packages),
    })
    // V: do we have a coherent story for when something is inline or in the arena?
    // VA: Not yet. This parent() method takes a raw &Bump and calls alloc_slice_copy directly,
    // VA: bypassing interning. Every other PackageCoordinate constructor goes through
    // VA: parse_arena/scout_arena intern_package_coordinate() which deduplicates. If called, this
    // VA: would produce non-interned coordinates that break pointer-identity equality. Rule: semantic
    // VA: types (PackageCoordinate, FileCoordinate) should always go through an arena intern method;
    // VA: raw bump is only for internal data structures (ArenaIndexMap). This method has zero callers.
  }
/*
  def parent(interner: Interner): Option[PackageCoordinate] = {
    if (packages.isEmpty) {
      None
    } else {
      Some(interner.intern(PackageCoordinate(module, packages.init)))
    }
  }
}

object PackageCoordinate {// extends Ordering[PackageCoordinate] {
*/
// mig: fn test_tld
  pub fn test_tld<'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    _keywords: &'ctx Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    parse_arena.intern_package_coordinate(parse_arena.intern_str(TEST_MODULE), &[])
  }
/*
  def TEST_TLD(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(interner.intern(StrI("test")), Vector.empty))
*/
// mig: fn builtin
  pub fn builtin<'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    parse_arena.intern_package_coordinate(keywords.empty_string, &[])
  }
/*
  def BUILTIN(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))
*/
// mig: fn internal
  pub fn internal(
    scout_arena: &ScoutArena<'a>,
    keywords: &Keywords<'a>,
  ) -> PackageCoordinate<'a> {
    *scout_arena.intern_package_coordinate(keywords.empty_string, &[])
  }
/*
  def internal(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))
//
//  override def compare(a: PackageCoordinate, b: PackageCoordinate):Int = {
//    val lenDiff = a.packages.length - b.packages.length
//    if (lenDiff != 0) {
//      return lenDiff
//    }
//    a.packages.zip(b.packages).foreach({ case (stepA, stepB) =>
//      val stepDiff = stepA.uid - stepB.uid
//      if (stepDiff != 0L) {
//        return U.sign(stepDiff)
//      }
//    })
//    return U.sign(a.module.uid - b.module.uid)
//  }
}

object FileCoordinateMap {
*/
}
// Realizes Scala's case-class auto-toString for PackageCoordinate:
//   PackageCoordinate(<module>,Vector(<pkg1>, <pkg2>, ...))
// Per Scala convention: no space between case-class fields, comma+space between Vector elements.
// NOTE: Rust StrI's Display canon (interner.rs:40) prints the bare string, while Scala StrI's
// own case-class toString wraps as `StrI(<value>)`. This divergence is inherited from the
// existing canon and not propagated here.
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
/* Guardian: disable-all */
// mig: const TEST_MODULE
const TEST_MODULE: &str = "test";
/*
  val TEST_MODULE = "test"
*/
// mig: fn simple
pub fn simple<'a, T: Clone>(
    file_coord: &'a FileCoordinate<'a>,
    contents: T,
  ) -> FileCoordinateMap<'a, T> {
    let mut result = FileCoordinateMap::new();
    result.put(file_coord, contents);
    result
  }
/*
  def simple[T](fileCoord: FileCoordinate, contents: T): FileCoordinateMap[T] = {
    val result = new FileCoordinateMap[T]()
    result.put(fileCoord, contents)
    result
  }
*/
// mig: fn test
pub fn test<'a, C: Clone>(
    scout_arena: &ScoutArena<'a>,
    contents: C,
  ) -> FileCoordinateMap<'a, C> {
    const TEST_MODULE: &str = "test";
    let test_module = scout_arena.intern_str(TEST_MODULE);
    let package_coord = scout_arena.intern_package_coordinate(test_module, &[]);
    let file_coord = scout_arena.intern_file_coordinate(package_coord, "test.vale");
    let mut result = FileCoordinateMap::new();
    result.put(file_coord, contents);
    result
  }
/*
  def test[T](interner: Interner, contents: T): FileCoordinateMap[T] = {
    val result = new FileCoordinateMap[T]()
    result.put(
      interner.intern(FileCoordinate(
        interner.intern(PackageCoordinate(
          interner.intern(StrI(TEST_MODULE)), Vector.empty)),
        "test.vale")),
      contents)
    result
  }
*/
// mig: fn test
pub fn test_from_vec<'a, T: Clone>(
    parse_arena: &ParseArena<'a>,
    contents: Vec<T>,
  ) -> FileCoordinateMap<'a, T> {
    let mut map = HashMap::new();
    for (index, code) in contents.into_iter().enumerate() {
      map.insert(format!("{}.vale", index), code);
    }
    test_from_map(parse_arena, map)
  }
/*
  def test[T](interner: Interner, contents: Vector[T]): FileCoordinateMap[T] = {
    test(interner, contents.zipWithIndex.map({ case (code, index) => (index + ".vale", code) }).toMap)
  }
*/
// mig: fn test
pub fn test_from_map<'a, T: Clone>(
    parse_arena: &ParseArena<'a>,
    contents: HashMap<String, T>,
  ) -> FileCoordinateMap<'a, T> {
    let mut result = FileCoordinateMap::new();
    let package_coord = parse_arena.intern_package_coordinate(parse_arena.intern_str(TEST_MODULE), &[]);
    for (filepath, file_contents) in contents {
      let file_coord = parse_arena.intern_file_coordinate(package_coord, &filepath);
      result.put(file_coord, file_contents);
    }
    result
  }
/*
  def test[T](interner: Interner, contents: Map[String, T]): FileCoordinateMap[T] = {
    val result = new FileCoordinateMap[T]()
    contents.foreach({ case (filepath, contents) =>
      result.put(
        interner.intern(FileCoordinate(
          interner.intern(PackageCoordinate(
            interner.intern(StrI(TEST_MODULE)), Vector.empty)),
          filepath)),
        contents)
    })
    result
  }
}

*/
// mig: struct FileCoordinateMap
#[derive(Clone, Debug)]
pub struct FileCoordinateMap<'a, Contents> {
  pub package_coord_to_file_coords: HashMap<&'a PackageCoordinate<'a>, Vec<&'a FileCoordinate<'a>>>,
  pub file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, Contents>,
}
// mig: impl FileCoordinateMap
// mergeNonOverlapping was commented out in Scala (not yet needed)
impl<'a, Contents: Clone> FileCoordinateMap<'a, Contents> {
/*
class FileCoordinateMap[Contents](
  val packageCoordToFileCoords: mutable.Map[PackageCoordinate, Vector[FileCoordinate]] =
    mutable.HashMap[PackageCoordinate, Vector[FileCoordinate]](),
  val fileCoordToContents: mutable.Map[FileCoordinate, Contents] =
    mutable.HashMap[FileCoordinate, Contents]()
) extends IPackageResolver[Map[String, Contents]] {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/
  pub fn new() -> Self {
    FileCoordinateMap {
      package_coord_to_file_coords: HashMap::new(),
      file_coord_to_contents: HashMap::new(),
    }
  }

  /// Companion-object style constructor for tests. Mirrors FileCoordinateMap.test(scout_arena, contents).
  pub fn test(scout_arena: &ScoutArena<'a>, contents: Contents) -> Self {
    super::code_hierarchy::test(scout_arena, contents)
  }

// mig: fn apply
  pub fn apply(&self, coord: &'a FileCoordinate<'a>) -> &Contents {
    self
      .file_coord_to_contents
      .get(coord)
      .expect("FileCoordinateMap::apply - coordinate not found")
  }
/*
  def apply(coord: FileCoordinate): Contents = {
    vassertSome(fileCoordToContents.get(coord))
  }

  // This is different from put in that we can hand in an empty map here.
  // It's the only way to have an empty package in the FileCoordinateMap.
*/
  pub fn get_by_value(&self, coord: &FileCoordinate<'_>) -> Option<&Contents> {
    self.file_coord_to_contents.iter()
      .find(|(k, _)| k.eq_by_value(coord))
      .map(|(_, v)| v)
  }

// mig: fn put_package
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
/*
  def putPackage(
    interner: Interner,
    packageCoord: PackageCoordinate,
    newFileCoordToContents: Map[FileCoordinate, Contents]):
  Unit = {
    packageCoordToFileCoords.put(packageCoord, newFileCoordToContents.keys.toVector)
    newFileCoordToContents.foreach({ case (fileCoord, contents) =>
      fileCoordToContents.put(fileCoord, contents)
    })
  }
*/
// mig: fn put
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
/*
  def put(fileCoord: FileCoordinate, contents: Contents): Unit = {
    vassert(!fileCoordToContents.contains(fileCoord))
    fileCoordToContents.put(fileCoord, contents)
    packageCoordToFileCoords.put(
      fileCoord.packageCoordinate,
      packageCoordToFileCoords.getOrElse(fileCoord.packageCoordinate, Vector()) :+ fileCoord)
  }
*/
// mig: fn map
  pub fn map<T, F>(&self, func: F) -> FileCoordinateMap<'a, T>
  where
    F: Fn(&'a FileCoordinate<'a>, &Contents) -> T,
    T: Clone,
  {
    let mut result_file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, T> = HashMap::new();
    for (file_coord, contents) in &self.file_coord_to_contents {
      result_file_coord_to_contents.insert(file_coord, func(file_coord, contents));
    }
    FileCoordinateMap {
      package_coord_to_file_coords: self.package_coord_to_file_coords.clone(),
      file_coord_to_contents: result_file_coord_to_contents,
    }
  }
/*
  def map[T](func: (FileCoordinate, Contents) => T): FileCoordinateMap[T] = {
    val resultFileCoordToContents = mutable.HashMap[FileCoordinate, T]()
    fileCoordToContents.foreach({ case (fileCoord, contents) =>
      resultFileCoordToContents.put(fileCoord, func(fileCoord, contents))
    })
    new FileCoordinateMap(packageCoordToFileCoords, resultFileCoordToContents)
  }
*/
// mig: fn flat_map
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
/*
  def flatMap[T](func: (FileCoordinate, Contents) => T): Iterable[T] = {
    fileCoordToContents.map({ case (fileCoord, contents) =>
      func(fileCoord, contents)
    })
  }
*/
// mig: fn expect_one
  pub fn expect_one(&self) -> &Contents {
    assert!(
      self.file_coord_to_contents.len() == 1,
      "FileCoordinateMap::expect_one - expected exactly one entry"
    );
    self.file_coord_to_contents.values().next().unwrap()
  }
/*
  def expectOne(): Contents = {
    vassertOne(fileCoordToContents.values)
  }

//  def mergeNonOverlapping(that: FileCoordinateMap[Contents]): FileCoordinateMap[Contents] = {
//    val result =
//      FileCoordinateMap(
//        this.packageCoordToFileCoords ++ that.packageCoordToFileCoords,
//        this.fileCoordToContents ++ that.fileCoordToContents)
//    vassert(
//      result.packageCoordToFileCoords.size ==
//        this.packageCoordToFileCoords.size + that.packageCoordToFileCoords.size)
//    vassert(
//      result.fileCoordToContents.size ==
//        this.fileCoordToContents.size + that.fileCoordToContents.size)
//    result
////      (this.fileCoordToContents.keySet ++ that.fileCoordToContents.keySet).map(fileCoord => {
////        val contents =
////          (this.fileCoordToContents.get(fileCoord).toList ++ that.fileCoordToContents.get(fileCoord).toList) match {
////            case List(_, _) => vfail()
////            case List(only) => only
////            case List() => vwat()
////          }
////        fileCoord -> contents
////      }).toMap)
//  }

*/
// mig: fn resolve
  pub fn resolve(
    &self,
    package_coord: &'a PackageCoordinate<'a>,
  ) -> Option<HashMap<String, Contents>> {
    self
      .package_coord_to_file_coords
      .get(package_coord)
      .map(|file_coords| {
        file_coords
          .iter()
          .map(|file_coord| {
            let contents = self
              .file_coord_to_contents
              .get(file_coord)
              .expect("FileCoordinateMap::resolve - file coord not found in contents");
            (file_coord.filepath.as_str().to_string(), contents.clone())
          })
          .collect()
      })
  }
/*
  def resolve(packageCoord: PackageCoordinate): Option[Map[String, Contents]] = {
    Profiler.frame(() => {
      packageCoordToFileCoords
        .get(packageCoord)
        .map(fileCoords => {
          fileCoords.map(fileCoord => {
            fileCoord.filepath -> vassertSome(fileCoordToContents.get(fileCoord))
          }).toMap
        })
    })
  }
}

object PackageCoordinateMap {
*/
}
// mig: fn compose_resolvers
pub fn compose_resolvers<'a, Contents>(
  resolver_a: impl Fn(&'a PackageCoordinate<'a>) -> Option<HashMap<String, Contents>>,
  resolver_b: impl Fn(&'a PackageCoordinate<'a>) -> HashMap<String, Contents>,
  package_coord: &'a PackageCoordinate<'a>,
) -> HashMap<String, Contents> {
  match resolver_a(package_coord) {
    Some(result) => result,
    None => resolver_b(package_coord),
  }
}
/*
  def composeResolvers[Contents](
    resolverA: PackageCoordinate => Option[Map[String, Contents]],
    resolverB: PackageCoordinate => Map[String, Contents])
    (packageCoord: PackageCoordinate):
  Map[String, Contents] = {
    resolverA(packageCoord) match {
      case Some(result) => result
      case None => resolverB(packageCoord)
    }
  }
*/
// mig: fn compose_map_and_resolver
pub fn compose_map_and_resolver<'a, Contents>(
  files: &FileCoordinateMap<'a, Contents>,
  then_resolver: impl Fn(&'a PackageCoordinate<'a>) -> HashMap<String, Contents>,
  package_coord: &'a PackageCoordinate<'a>,
) -> HashMap<String, Contents>
where
  Contents: Clone,
{
  match files.resolve(package_coord) {
    Some(filename_to_contents) => filename_to_contents,
    None => then_resolver(package_coord),
  }
}
/*
  def composeMapAndResolver[Contents](
    files: FileCoordinateMap[Contents],
    thenResolver: PackageCoordinate => Map[String, Contents])
    (packageCoord: PackageCoordinate):
  Map[String, Contents] = {
    files.resolve(packageCoord) match {
      case Some(filenameToContents) => {
        return filenameToContents
      }
      case None =>
    }
    thenResolver(packageCoord)
  }
}

*/
// mig: trait IPackageResolver
pub trait IPackageResolver<'a, T> {
/*
trait IPackageResolver[T] {
*/
// mig: fn resolve
  fn resolve(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<T>;
/*
  def resolve(packageCoord: PackageCoordinate): Option[T]
*/
// mig: fn or
  fn or<F>(self, fallback: F) -> OrResolver<Self, F>
  where
    Self: Sized,
    F: IPackageResolver<'a, T>,
  {
    OrResolver {
      primary: self,
      fallback,
    }
  }
/*
  def or(fallback: IPackageResolver[T]): IPackageResolver[T] =
    x => innerOr(fallback, x)
*/
// mig: fn inner_or
  fn inner_or(
    &self,
    fallback: &impl IPackageResolver<'a, T>,
    package_coord: &'a PackageCoordinate<'a>,
  ) -> Option<T>
  where
    Self: Sized,
  {
    match self.resolve(package_coord) {
      Some(x) => Some(x),
      None => fallback.resolve(package_coord),
    }
  }
/*
  def innerOr(fallback: IPackageResolver[T], packageCoord: PackageCoordinate): Option[T] = {
    resolve(packageCoord) match {
      case Some(x) => Some(x)
      case None => fallback.resolve(packageCoord)
    }
  }
}

*/
}

impl<'a, Contents: Clone> IPackageResolver<'a, HashMap<String, Contents>>
  for FileCoordinateMap<'a, Contents>
{
  fn resolve(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<HashMap<String, Contents>> {
    FileCoordinateMap::resolve(self, package_coord)
  }
}
// mig: struct PackageCoordinateMap
#[derive(Clone, Debug)]
pub struct PackageCoordinateMap<'a, Contents> {
  pub package_coord_to_contents: HashMap<&'a PackageCoordinate<'a>, Contents>,
}
// mig: impl PackageCoordinateMap
impl<'a, Contents> PackageCoordinateMap<'a, Contents> {
/*
case class PackageCoordinateMap[Contents](
  packageCoordToContents: mutable.HashMap[PackageCoordinate, Contents] =
    mutable.HashMap[PackageCoordinate, Contents]()) {

  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/
  pub fn new() -> Self {
    PackageCoordinateMap {
      package_coord_to_contents: HashMap::new(),
    }
  }

// mig: fn put
  pub fn put(&mut self, package_coord: &'a PackageCoordinate<'a>, contents: Contents) {
    self.package_coord_to_contents.insert(package_coord, contents);
  }
/*
  def put(packageCoord: PackageCoordinate, contents: Contents): Unit = {
    packageCoordToContents.put(packageCoord, contents)
  }
*/
// mig: fn get
  pub fn get(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<&Contents> {
    self.package_coord_to_contents.get(package_coord)
  }
/*
  def get(packageCoord: PackageCoordinate): Option[Contents] = {
    packageCoordToContents.get(packageCoord)
  }
*/
// mig: fn expect_one
  pub fn expect_one(&self) -> &Contents {
    assert!(
      self.package_coord_to_contents.len() == 1,
      "PackageCoordinateMap::expect_one - expected exactly one entry"
    );
    self.package_coord_to_contents.values().next().unwrap()
  }
/*
  def expectOne(): Contents = {
    vassertOne(packageCoordToContents.values)
  }
*/
// mig: fn map
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
/*
  def map[T](func: (PackageCoordinate, Contents) => T): PackageCoordinateMap[T] = {
    val result = new PackageCoordinateMap[T]()
    packageCoordToContents.foreach({ case (packageCoord, contents) =>
      result.put(packageCoord, func(packageCoord, contents))
    })
    result
  }
*/
// mig: fn flat_map
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
/*
  def flatMap[T](func: (PackageCoordinate, Contents) => T): Iterable[T] = {
    packageCoordToContents.map({ case (packageCoord, contents) =>
      func(packageCoord, contents)
    })
  }
}

*/
}
