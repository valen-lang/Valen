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
use crate::final_ast::types::{SimpleId, SimpleIdStep};
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::Hammer;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::names::{IdI, INameI};
use crate::instantiating::ast::templata::ITemplataI;
use crate::instantiating::ast::types::{cI, CoordI, KindIT};

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
*/

/*
object NameHammer {
*/

// mig: fn translate_code_location (object NameHammer free function)
pub fn translate_code_location<'p>(location: &CodeLocationS<'p>) -> VonObject {
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

// mig: fn translate_file_coordinate (object NameHammer free function)
pub fn translate_file_coordinate<'p>(coord: &FileCoordinate<'p>) -> VonObject {
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

// mig: fn translate_package_coordinate (object NameHammer free function)
pub fn translate_package_coordinate<'p>(coord: &PackageCoordinate<'p>) -> VonObject {
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
*/

// mig: fn simplify_id (object NameHammer free function)
pub fn simplify_id<'s, 'i, 'h>(id: &IdI<'s, 'i, cI>) -> SimpleId<'s, 'h>
where 's: 'i, 'i: 'h,
{
    panic!("Unimplemented: simplify_id");
}
/*
  def simplifyId(id: IdI[cI, INameI[cI]]): SimpleId = {
    val IdI(packageCoord, initSteps, localName) = id
    val PackageCoordinate(module, packages) = packageCoord
    SimpleId(
      (SimpleIdStep(module.str, Vector()) +:
          packages.map(paackage => SimpleIdStep(paackage.str, Vector()))) ++
          initSteps.map(step => simplifyName(step)) :+
          simplifyName(localName))
  }
*/

// mig: fn simplify_name (object NameHammer free function)
pub fn simplify_name<'s, 'i, 'h>(name: &INameI<'s, 'i, cI>) -> SimpleIdStep<'s, 'h>
where 's: 'i, 'i: 'h,
{
    panic!("Unimplemented: simplify_name");
}
/*
  def simplifyName(name: INameI[cI]): SimpleIdStep = {
    name match {
      case StructNameI(StructTemplateNameI(humanName), templateArgs) =>
        SimpleIdStep(humanName.str, templateArgs.map(simplifyTemplata))
      case StructTemplateNameI(humanName) =>
        SimpleIdStep(humanName.str, Vector())
      case InterfaceNameI(InterfaceTemplateNameI(humanName), templateArgs) =>
        SimpleIdStep(humanName.str, templateArgs.map(simplifyTemplata))
      case InterfaceTemplateNameI(humanName) =>
        SimpleIdStep(humanName.str, Vector())
      case ExternFunctionNameI(humanName, templateArgs, parameters) =>
        SimpleIdStep(humanName.str, templateArgs.map(simplifyTemplata))
      case other => vimpl(other)
    }
  }
*/

// mig: fn simplify_templata (object NameHammer free function)
pub fn simplify_templata<'s, 'i, 'h>(templata: &ITemplataI<'s, 'i, cI>) -> SimpleId<'s, 'h>
where 's: 'i, 'i: 'h,
{
    panic!("Unimplemented: simplify_templata");
}
/*
  def simplifyTemplata(templata: ITemplataI[cI]): SimpleId = {
    templata match {
      case CoordTemplataI(region, coord) => simplifyCoord(coord)
      case other => vimpl(other)
    }
  }
*/

// mig: fn simplify_kind (object NameHammer free function)
pub fn simplify_kind<'s, 'i, 'h>(value: &KindIT<'s, 'i, cI>) -> SimpleId<'s, 'h>
where 's: 'i, 'i: 'h,
{
    panic!("Unimplemented: simplify_kind");
}
/*
  def simplifyKind(value: KindIT[cI]): SimpleId = {
    value match {
      case IntIT(bits) => SimpleId(Vector(SimpleIdStep("i" + bits, Vector())))
      case StrIT() => SimpleId(Vector(SimpleIdStep("str", Vector())))
      case other => vimpl(other)
    }
  }
*/

// mig: fn simplify_coord (object NameHammer free function)
pub fn simplify_coord<'s, 'i, 'h>(value: &CoordI<'s, 'i, cI>) -> SimpleId<'s, 'h>
where 's: 'i, 'i: 'h,
{
    panic!("Unimplemented: simplify_coord");
}
/*
  def simplifyCoord(value: CoordI[cI]): SimpleId = {
    val CoordI(ownership, kind) = value
    val kindId = simplifyKind(kind)
    (ownership match {
      case ImmutableShareI => kindId
      case MutableShareI => kindId
      case OwnI => kindId
      case WeakI => vimpl()
      case ImmutableBorrowI => SimpleId(Vector(SimpleIdStep("&", Vector(kindId))))
      case MutableBorrowI => SimpleId(Vector(SimpleIdStep("&mut", Vector(kindId))))
    })
  }
}
*/
