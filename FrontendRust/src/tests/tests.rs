/*
package dev.vale

import scala.io.Source

object Tests {
*/
// mig: fn load
pub fn load(resource_filename: &str) -> Option<String> {
  panic!("Unimplemented: load");
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
  panic!("Unimplemented: load_expected");
}
/*
  def loadExpected(resourceFilename: String): String = {
    load(resourceFilename).get
  }
*/
// mig: fn resolve_package_to_resource
pub fn resolve_package_to_resource(package_coord: &PackageCoordinate) -> Option<HashMap<String, String>> {
  let directory = {
    let mut v = vec![&package_coord.module];
    v.extend(&package_coord.packages);
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