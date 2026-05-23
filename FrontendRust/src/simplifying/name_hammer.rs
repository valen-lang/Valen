// From Frontend/SimplifyingPass/src/dev/vale/simplifying/NameHammer.scala
//
// Per typing-pass `Compiler` precedent, `NameHammer` is not a Rust struct.
// Its methods live as `impl Hammer { ... }` blocks colocated here; its free
// functions (in Scala's `object NameHammer`) become module-level Rust fns.

use crate::interner::StrI;
use crate::utils::range::CodeLocationS;
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::von::ast::VonObject;
use crate::final_ast::ast::IdH;
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::Hammer;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::names::IdI;
use crate::instantiating::ast::types::cI;

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

class NameHammer() {
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
  def translateCodeLocation(location: CodeLocationS): VonObject = {
    val CodeLocationS(fileCoord, offset) = location
    VonObject(
      "CodeLocation",
      None,
      Vector(
        VonMember("file", translateFileCoordinate(fileCoord)),
        VonMember("offset", VonInt(offset))))
  }

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

// mig: fn translate_full_name
impl<'s, 'h, 'ctx> Hammer<'s, 'h, 'ctx>
where 's: 'h,
{
    pub fn translate_full_name<'i>(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &Hamuts<'s, 'i, 'h>,
        full_name2: &IdI<'s, 'i, cI>,
    ) -> &'h IdH<'s, 'h>
    where 's: 'i, 'i: 'h,
    {
        panic!("Unimplemented: translate_full_name");
    }
}

// mig: fn add_step
impl<'s, 'h, 'ctx> Hammer<'s, 'h, 'ctx>
where 's: 'h,
{
    pub fn add_step<'i>(
        &self,
        hamuts: &Hamuts<'s, 'i, 'h>,
        full_name: &IdH<'s, 'h>,
        s: StrI<'s>,
    ) -> &'h IdH<'s, 'h>
    where 's: 'i, 'i: 'h,
    {
        panic!("Unimplemented: add_step");
    }
}

// mig: fn translate_code_location (object NameHammer free function)
pub fn translate_code_location<'p>(location: &CodeLocationS<'p>) -> VonObject {
    panic!("Unimplemented: translate_code_location");
}

// mig: fn translate_file_coordinate (object NameHammer free function)
pub fn translate_file_coordinate<'p>(coord: &FileCoordinate<'p>) -> VonObject {
    panic!("Unimplemented: translate_file_coordinate");
}

// mig: fn translate_package_coordinate (object NameHammer free function)
pub fn translate_package_coordinate<'p>(coord: &PackageCoordinate<'p>) -> VonObject {
    panic!("Unimplemented: translate_package_coordinate");
}
