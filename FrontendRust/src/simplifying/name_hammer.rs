// From Frontend/SimplifyingPass/src/dev/vale/simplifying/NameHammer.scala
use crate::interner::StrI;
use crate::utils::range::CodeLocationS;
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::von::ast::VonObject;
use crate::simplifying::ast::{IdH, HamutsBox};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::names::IdI;

/*
package dev.vale.simplifying

import dev.vale._
import dev.vale.finalast.IdH
import dev.vale.finalast._
import dev.vale.postparsing.AnonymousSubstructParentInterfaceTemplateRuneS
import dev.vale.instantiating._
import dev.vale.instantiating.ast._
import dev.vale.von.{IVonData, VonArray, VonInt, VonMember, VonObject, VonStr}

import scala.collection.immutable.List

*/
// mig: struct NameHammerH
/// Temporary state
pub struct NameHammerH {
}

// mig: impl NameHammerH
/*
class NameHammer() {
*/
// mig: fn translate_full_name
impl<'h> NameHammerH {
    pub fn translate_full_name(&self, hinputs: &HinputsI, hamuts: &HamutsBox, full_name2: &IdI) -> IdH {
        panic!("Unimplemented: translate_full_name");
    }
}
/*
  def translateFullName(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    fullName2: IdI[cI, INameI[cI]]
  ): IdH = {
    val IdI(packageCoord, _, localNameT) = fullName2
    val longName = InstantiatedHumanizer.humanizeId(_.toString, fullName2)
    val localName = InstantiatedHumanizer.humanizeName(_.toString, localNameT)
    finalast.IdH(localName, packageCoord, longName, longName)
  }

*/
// mig: fn add_step
impl<'h> NameHammerH {
    pub fn add_step(&self, hamuts: &HamutsBox, full_name: &IdH, s: StrI<'h>) -> IdH {
        panic!("Unimplemented: add_step");
    }
}
/*
  // Adds a step to the name.
  def addStep(
    hamuts: HamutsBox,
    fullName: IdH,
    s: String):
  IdH = {
    val IdH(_, packageCoordinate, shortenedName, fullyQualifiedName) = fullName
    IdH(s, packageCoordinate, shortenedName + "." + s, fullyQualifiedName + "." + s)
  }
}

object NameHammer {
*/
// mig: fn translate_code_location
pub fn translate_code_location(location: &CodeLocationS) -> VonObject {
    panic!("Unimplemented: translate_code_location");
}
/*
  def translateCodeLocation(location: CodeLocationS): VonObject = {
    val CodeLocationS(fileCoord, offset) = location
    VonObject(
      "CodeLocation",
      None,
      Vector(
        VonMember("file", translateFileCoordinate(fileCoord)),
        VonMember("offset", VonInt(offset))))
  }

*/
// mig: fn translate_file_coordinate
pub fn translate_file_coordinate(coord: &FileCoordinate) -> VonObject {
    panic!("Unimplemented: translate_file_coordinate");
}
/*
  def translateFileCoordinate(coord: FileCoordinate): VonObject = {
    val FileCoordinate(PackageCoordinate(module, paackage), filename) = coord
    VonObject(
      "FileCoordinate",
      None,
      Vector(
        VonMember("module", VonStr(module.str)),
        VonMember("paackage", VonArray(None, paackage.map(_.str).map(VonStr).toVector)),
        VonMember("filename", VonStr(filename))))
  }

*/
// mig: fn translate_package_coordinate
pub fn translate_package_coordinate(coord: &PackageCoordinate) -> VonObject {
    panic!("Unimplemented: translate_package_coordinate");
}
/*
  def translatePackageCoordinate(coord: PackageCoordinate): VonObject = {
    val PackageCoordinate(module, paackage) = coord
    val nonEmptyModuleName = if (module.str == "") "__vale" else module.str;
    VonObject(
      "PackageCoordinate",
      None,
      Vector(
        VonMember("project", VonStr(nonEmptyModuleName)),
        VonMember("packageSteps", VonArray(None, paackage.map(_.str).map(VonStr).toVector))))
  }
}
*/
