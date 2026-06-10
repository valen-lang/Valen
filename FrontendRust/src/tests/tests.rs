/*
package dev.vale

import scala.io.Source

object Tests {
*/
use std::collections::HashMap;
use std::path::PathBuf;
use crate::utils::code_hierarchy::PackageCoordinate;
use std::fs::File;
use std::io::read_to_string;

// mig: fn load
// Rust adaptation: Scala's `vassert(source != null)` is dropped — `read_to_string`
// returns Result<String>, so `.unwrap()` already enforces non-null by the type system.
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
/*
  def load(resourceFilename: String): Option[String] = {
    val stream = getClass().getClassLoader().getResourceAsStream(resourceFilename)
    if (stream == null)
      return None
    val source = Source.fromInputStream(stream)
    vassert(source != null)
    Some(source.mkString(""))
  }
*/
// mig: fn load_expected
pub fn load_expected(resource_filename: &str) -> String {
  load(resource_filename)
    .unwrap_or_else(|| panic!("Failed to load resource: {}", resource_filename))
}
/*
  def loadExpected(resourceFilename: String): String = {
    load(resourceFilename).get
  }
*/
// mig: fn resolve_package_to_resource
pub fn resolve_package_to_resource(package_coord: &PackageCoordinate) -> Option<HashMap<String, String>> {
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
      let mut m = HashMap::new();
      m.insert(filename, source);
      Some(m)
    }
  }
}
/*
  def resolvePackageToResource(packageCoord: PackageCoordinate): Option[Map[String, String]] = {
    val directory = (Vector(packageCoord.module) ++ packageCoord.packages).map(_.str)
    val filename = directory.last + ".vale"
    val filepath = (directory :+ filename).mkString("/")
    load(filepath) match {
      case None => {
        None
      }
      case Some(source) => Some(Map(filename -> source))
    }
  }
*/
// mig: fn get_package_to_resource_resolver
pub fn get_package_to_resource_resolver() -> fn(&PackageCoordinate) -> Option<HashMap<String, String>> {
  resolve_package_to_resource
}
/*
  def getPackageToResourceResolver: IPackageResolver[Map[String, String]]
    = resolvePackageToResource
}
*/