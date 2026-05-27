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

  def simplifyId(id: IdI[cI, INameI[cI]]): SimpleId = {
    val IdI(packageCoord, initSteps, localName) = id
    val PackageCoordinate(module, packages) = packageCoord
    SimpleId(
      (SimpleIdStep(module.str, Vector()) +:
          packages.map(paackage => SimpleIdStep(paackage.str, Vector()))) ++
          initSteps.map(step => simplifyName(step)) :+
          simplifyName(localName))
  }

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

  def simplifyTemplata(templata: ITemplataI[cI]): SimpleId = {
    templata match {
      case CoordTemplataI(region, coord) => simplifyCoord(coord)
      case other => vimpl(other)
    }
  }

  def simplifyKind(value: KindIT[cI]): SimpleId = {
    value match {
      case IntIT(bits) => SimpleId(Vector(SimpleIdStep("i" + bits, Vector())))
      case StrIT() => SimpleId(Vector(SimpleIdStep("str", Vector())))
      case other => vimpl(other)
    }
  }

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
