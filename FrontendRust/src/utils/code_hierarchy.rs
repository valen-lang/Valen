// From Frontend/Utils/src/dev/vale/CodeHierarchy.scala

use std::collections::HashMap;
use crate::interner::{InternedSlice, StrI};

// From CodeHierarchy.scala lines 104-189
#[derive(Clone)]
pub struct FileCoordinateMap<'a, Contents> {
  pub package_coord_to_file_coords: HashMap<&'a PackageCoordinate<'a>, Vec<&'a FileCoordinate<'a>>>,
  pub file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, Contents>,
}

impl<'a, Contents: Clone> FileCoordinateMap<'a, Contents> {
  pub fn new() -> Self {
    FileCoordinateMap {
      package_coord_to_file_coords: HashMap::new(),
      file_coord_to_contents: HashMap::new(),
    }
  }

  // From CodeHierarchy.scala lines 112-114
  pub fn apply(&self, coord: &'a FileCoordinate<'a>) -> &Contents {
    self
      .file_coord_to_contents
      .get(coord)
      .expect("FileCoordinateMap::apply - coordinate not found")
  }

  // From CodeHierarchy.scala lines 118-127: putPackage
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

  // From CodeHierarchy.scala lines 129-135: put
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

  // From CodeHierarchy.scala lines 137-143: map
  pub fn map<T, F>(&self, func: F) -> FileCoordinateMap<'a, T>
  where
    F: Fn(&'a FileCoordinate<'a>, &Contents) -> T,
    T: Clone,
  {
    let mut result_file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, T> = HashMap::new();
    for (file_coord, contents) in &self.file_coord_to_contents {
      result_file_coord_to_contents.insert(file_coord, func(file_coord, contents));
    }

    let mut result: FileCoordinateMap<'a, T> = FileCoordinateMap::new();
    result.put_package(
      &self.package_coord_to_file_coords.keys().next().unwrap(),
      result_file_coord_to_contents,
    );
    result
  }

  // From CodeHierarchy.scala lines 145-149: flatMap
  pub fn flat_map<T, F>(&self, func: F) -> Vec<T>
  where
    F: Fn(&'a FileCoordinate<'a>, &Contents) -> T,
  {
    self
      .file_coord_to_contents
      .iter()
      .map(|(file_coord, contents)| func(&file_coord, contents))
      .collect()
  }

  // From CodeHierarchy.scala lines 151-153: expectOne
  pub fn expect_one(&self) -> &Contents {
    assert!(
      self.file_coord_to_contents.len() == 1,
      "FileCoordinateMap::expect_one - expected exactly one entry"
    );
    self.file_coord_to_contents.values().next().unwrap()
  }
}

// From CodeHierarchy.scala lines 109, 178-188: IPackageResolver implementation
impl<'a, Contents: Clone> IPackageResolver<'a, HashMap<String, Contents>> for FileCoordinateMap<'a, Contents> {
  fn resolve(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<HashMap<String, Contents>> {
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
            (file_coord.filepath.to_string(), contents.clone())
          })
          .collect()
      })
  }
}

// TODO: move to utils/code_hierarchy.rs
/// File coordinate matching Scala's FileCoordinate
/// Interned.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileCoordinate<'a> {
  pub package_coord: &'a PackageCoordinate<'a>,
  pub filepath: StrI<'a>,
}

// TODO: move to utils/code_hierarchy.rs
/// Package coordinate matching Scala's PackageCoordinate
/// Interned.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackageCoordinate<'a> {
  pub module: StrI<'a>,
  pub packages: InternedSlice<StrI<'a>>,
}

impl<'a> PackageCoordinate<'a> {
  // From CodeHierarchy.scala line 50: BUILTIN
  pub fn builtin<'ctx>(
    interner: &'ctx crate::Interner<'a>,
    keywords: &'ctx crate::Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    interner.intern_package_coordinate(keywords.empty_string, &[])
  }
}

// TODO: move to utils/code_hierarchy.rs
/// From CodeHierarchy.scala lines 218-230: IPackageResolver trait
/// Note: Uses parsing::ast::PackageCoordinate (the one used by the parser)
pub trait IPackageResolver<'a, T> {
  fn resolve(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<T>;

  // From CodeHierarchy.scala lines 221-229: or() method for chaining resolvers
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
}

// TODO: move to utils/code_hierarchy.rs
/// From CodeHierarchy.scala lines 221-229: Chained resolver implementation
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

// TODO: move to utils/code_hierarchy.rs
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
package dev.vale

import scala.collection.immutable.List
import scala.collection.mutable

case class FileCoordinate(packageCoordinate: PackageCoordinate, filepath: String) extends IInterning {
  def isInternal = packageCoordinate.isInternal
  def isTest(): Boolean = packageCoordinate.isTest && filepath == "test.vale"
//  def compareTo(that: FileCoordinate) = FileCoordinate.compare(this, that)
}

object FileCoordinate {// extends Ordering[FileCoordinate] {

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

case class PackageCoordinate(module: StrI, packages: Vector[StrI]) extends IInterning {
  def isInternal = module.str == ""
  def isTest = module.str == "test" && packages == Vector()

//  def compareTo(that: PackageCoordinate) = PackageCoordinate.compare(this, that)

  def parent(interner: Interner): Option[PackageCoordinate] = {
    if (packages.isEmpty) {
      None
    } else {
      Some(interner.intern(PackageCoordinate(module, packages.init)))
    }
  }
}

object PackageCoordinate {// extends Ordering[PackageCoordinate] {
  def TEST_TLD(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(interner.intern(StrI("test")), Vector.empty))

  def BUILTIN(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))

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
  val TEST_MODULE = "test"

  def simple[T](fileCoord: FileCoordinate, contents: T): FileCoordinateMap[T] = {
    val result = new FileCoordinateMap[T]()
    result.put(fileCoord, contents)
    result
  }
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
  def test[T](interner: Interner, contents: Vector[T]): FileCoordinateMap[T] = {
    test(interner, contents.zipWithIndex.map({ case (code, index) => (index + ".vale", code) }).toMap)
  }
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

class FileCoordinateMap[Contents](
  val packageCoordToFileCoords: mutable.Map[PackageCoordinate, Vector[FileCoordinate]] =
    mutable.HashMap[PackageCoordinate, Vector[FileCoordinate]](),
  val fileCoordToContents: mutable.Map[FileCoordinate, Contents] =
    mutable.HashMap[FileCoordinate, Contents]()
) extends IPackageResolver[Map[String, Contents]] {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()

  def apply(coord: FileCoordinate): Contents = {
    vassertSome(fileCoordToContents.get(coord))
  }

  // This is different from put in that we can hand in an empty map here.
  // It's the only way to have an empty package in the FileCoordinateMap.
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

  def put(fileCoord: FileCoordinate, contents: Contents): Unit = {
    vassert(!fileCoordToContents.contains(fileCoord))
    fileCoordToContents.put(fileCoord, contents)
    packageCoordToFileCoords.put(
      fileCoord.packageCoordinate,
      packageCoordToFileCoords.getOrElse(fileCoord.packageCoordinate, Vector()) :+ fileCoord)
  }

  def map[T](func: (FileCoordinate, Contents) => T): FileCoordinateMap[T] = {
    val resultFileCoordToContents = mutable.HashMap[FileCoordinate, T]()
    fileCoordToContents.foreach({ case (fileCoord, contents) =>
      resultFileCoordToContents.put(fileCoord, func(fileCoord, contents))
    })
    new FileCoordinateMap(packageCoordToFileCoords, resultFileCoordToContents)
  }

  def flatMap[T](func: (FileCoordinate, Contents) => T): Iterable[T] = {
    fileCoordToContents.map({ case (fileCoord, contents) =>
      func(fileCoord, contents)
    })
  }

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

trait IPackageResolver[T] {
  def resolve(packageCoord: PackageCoordinate): Option[T]

  def or(fallback: IPackageResolver[T]): IPackageResolver[T] =
    x => innerOr(fallback, x)

  def innerOr(fallback: IPackageResolver[T], packageCoord: PackageCoordinate): Option[T] = {
    resolve(packageCoord) match {
      case Some(x) => Some(x)
      case None => fallback.resolve(packageCoord)
    }
  }
}

case class PackageCoordinateMap[Contents](
  packageCoordToContents: mutable.HashMap[PackageCoordinate, Contents] =
    mutable.HashMap[PackageCoordinate, Contents]()) {

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()


  def put(packageCoord: PackageCoordinate, contents: Contents): Unit = {
    packageCoordToContents.put(packageCoord, contents)
  }

  def get(packageCoord: PackageCoordinate): Option[Contents] = {
    packageCoordToContents.get(packageCoord)
  }

  def expectOne(): Contents = {
    vassertOne(packageCoordToContents.values)
  }

  def map[T](func: (PackageCoordinate, Contents) => T): PackageCoordinateMap[T] = {
    val result = new PackageCoordinateMap[T]()
    packageCoordToContents.foreach({ case (packageCoord, contents) =>
      result.put(packageCoord, func(packageCoord, contents))
    })
    result
  }

  def flatMap[T](func: (PackageCoordinate, Contents) => T): Iterable[T] = {
    packageCoordToContents.map({ case (packageCoord, contents) =>
      func(packageCoord, contents)
    })
  }
}

*/
