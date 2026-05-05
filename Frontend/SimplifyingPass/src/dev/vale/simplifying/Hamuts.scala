package dev.vale.simplifying

import dev.vale.{PackageCoordinate, StrI, vassert, vcurious, vfail, vimpl}
import dev.vale.finalast._
import dev.vale.instantiating.ast._
import dev.vale.von.IVonData


case class HamutsBox(var inner: Hamuts) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vfail() // Shouldnt hash, is mutable

  def packageCoordToExportNameToFunction: Map[PackageCoordinate, Map[StrI, PrototypeH]] = inner.packageCoordToExportNameToFunction
  def packageCoordToExportNameToKind: Map[PackageCoordinate, Map[StrI, KindHT]] = inner.packageCoordToExportNameToKind
  def packageCoordToPrototypeToExtern: Map[PackageCoordinate, Map[PrototypeH, HamutsFunctionExtern]] = inner.packageCoordToPrototypeToExtern
  def packageCoordToKindToExtern: Map[PackageCoordinate, Map[OpaqueHT, HamutsKindExtern]] = inner.packageCoordToKindToExtern
  def structTToOpaqueH: Map[StructIT[cI], OpaqueHT] = inner.structTToOpaqueH
  def structTToStructH: Map[StructIT[cI], StructHT] = inner.structTToStructH
  def structTToStructDefH: Map[StructIT[cI], StructDefinitionH] = inner.structTToStructDefH
  def structDefs: Vector[StructDefinitionH] = inner.structDefs
  def interfaceTToInterfaceH: Map[InterfaceIT[cI], InterfaceHT] = inner.interfaceTToInterfaceH
  def interfaceTToInterfaceDefH: Map[InterfaceIT[cI], InterfaceDefinitionH] = inner.interfaceTToInterfaceDefH
  def functionRefs: Map[PrototypeI[cI], FunctionRefH] = inner.functionRefs
  def functionDefs: Map[PrototypeI[cI], FunctionH] = inner.functionDefs
  def staticSizedArrays: Map[StaticSizedArrayIT[cI], StaticSizedArrayDefinitionHT] = inner.staticSizedArrays
  def runtimeSizedArrays: Map[RuntimeSizedArrayIT[cI], RuntimeSizedArrayDefinitionHT] = inner.runtimeSizedArrays

  def forwardDeclareStruct(structIT: StructIT[cI], structRefH: StructHT): Unit = {
    inner = inner.forwardDeclareStruct(structIT, structRefH)
  }

  def addStructOriginatingFromTypingPass(structIT: StructIT[cI], structDefH: StructDefinitionH): Unit = {
    inner = inner.addStructOriginatingFromTypingPass(structIT, structDefH)
  }

  def addOpaque(structIT: StructIT[cI], opaqueH: OpaqueHT): Unit = {
    inner = inner.addOpaque(structIT, opaqueH)
  }

  def addStructOriginatingFromHammer(structDefH: StructDefinitionH): Unit = {
    inner = inner.addStructOriginatingFromHammer(structDefH)
  }

  def forwardDeclareInterface(interfaceIT: InterfaceIT[cI], interfaceRefH: InterfaceHT): Unit = {
    inner = inner.forwardDeclareInterface(interfaceIT, interfaceRefH)
  }

  def addInterface(interfaceIT: InterfaceIT[cI], interfaceDefH: InterfaceDefinitionH): Unit = {
    inner = inner.addInterface(interfaceIT, interfaceDefH)
  }

  def addStaticSizedArray(ssaIT: StaticSizedArrayIT[cI], staticSizedArrayDefinitionTH: StaticSizedArrayDefinitionHT): Unit = {
    inner = inner.addStaticSizedArray(ssaIT, staticSizedArrayDefinitionTH)
  }

  def addRuntimeSizedArray(rsaIT: RuntimeSizedArrayIT[cI], runtimeSizedArrayDefinitionTH: RuntimeSizedArrayDefinitionHT): Unit = {
    inner = inner.addRuntimeSizedArray(rsaIT, runtimeSizedArrayDefinitionTH)
  }

  def forwardDeclareFunction(functionRef2: PrototypeI[cI], functionRefH: FunctionRefH): Unit = {
    inner = inner.forwardDeclareFunction(functionRef2, functionRefH)
  }

  def addFunction(functionRef2: PrototypeI[cI], functionDefH: FunctionH): Unit = {
    inner = inner.addFunction(functionRef2, functionDefH)
  }

  def addKindExport(kind: KindHT, packageCoordinate: PackageCoordinate, exportedName: StrI): Unit = {
    inner = inner.addKindExport(kind, packageCoordinate, exportedName)
  }

//  def addKindExtern(kind: KindHT, packageCoordinate: PackageCoordinate, exportedName: StrI): Unit = {
//    inner = inner.addKindExtern(kind, packageCoordinate, exportedName)
//  }

  def addFunctionExport(prototype: PrototypeH, packageCoordinate: PackageCoordinate, exportedName: StrI): Unit = {
    inner = inner.addFunctionExport(prototype, packageCoordinate, exportedName)
  }

  def addKindExtern(opaqueH: OpaqueHT, simpleId: SimpleId, exportedName: String): Unit = {
    inner = inner.addKindExtern(opaqueH, simpleId, exportedName)
  }

  def addFunctionExtern(prototype: PrototypeH, simpleId: SimpleId, exportedName: String): Unit = {
    inner = inner.addFunctionExtern(prototype, simpleId, exportedName)
  }

//  def getNameId(readableName: String, packageCoordinate: PackageCoordinate, parts: Vector[IVonData]): Int = {
//    val (newInner, id) = inner.getNameId(readableName, packageCoordinate, parts)
//    inner = newInner
//    id
//  }

  def getStaticSizedArray(staticSizedArrayTH: StaticSizedArrayHT): StaticSizedArrayDefinitionHT = {
    inner.getStaticSizedArray(staticSizedArrayTH)
  }
  def getRuntimeSizedArray(runtimeSizedArrayTH: RuntimeSizedArrayHT): RuntimeSizedArrayDefinitionHT = {
    inner.getRuntimeSizedArray(runtimeSizedArrayTH)
  }
}

case class Hamuts(
    humanNameToFullNameToId: Map[String, Map[String, Int]],
    structTToOpaqueH: Map[StructIT[cI], OpaqueHT],
    structTToStructH: Map[StructIT[cI], StructHT],
    structTToStructDefH: Map[StructIT[cI], StructDefinitionH],
    structDefs: Vector[StructDefinitionH],
    staticSizedArrays: Map[StaticSizedArrayIT[cI], StaticSizedArrayDefinitionHT],
    runtimeSizedArrays: Map[RuntimeSizedArrayIT[cI], RuntimeSizedArrayDefinitionHT],
    interfaceTToInterfaceH: Map[InterfaceIT[cI], InterfaceHT],
    interfaceTToInterfaceDefH: Map[InterfaceIT[cI], InterfaceDefinitionH],
    functionRefs: Map[PrototypeI[cI], FunctionRefH],
    functionDefs: Map[PrototypeI[cI], FunctionH],
    packageCoordToExportNameToFunction: Map[PackageCoordinate, Map[StrI, PrototypeH]],
    packageCoordToExportNameToKind: Map[PackageCoordinate, Map[StrI, KindHT]],
    packageCoordToPrototypeToExtern: Map[PackageCoordinate, Map[PrototypeH, HamutsFunctionExtern]],
    packageCoordToKindToExtern: Map[PackageCoordinate, Map[OpaqueHT, HamutsKindExtern]]) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vfail() // Would need a really good reason to hash something this big

  vassert(functionDefs.values.map(_.id).toVector.distinct.size == functionDefs.values.size)
  vassert(structDefs.map(_.id).distinct.size == structDefs.size)
  vassert(runtimeSizedArrays.values.map(_.name).toVector.distinct.size == runtimeSizedArrays.size)

  def forwardDeclareStruct(structIT: StructIT[cI], structRefH: StructHT): Hamuts = {
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH + (structIT -> structRefH),
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def addStructOriginatingFromTypingPass(structTT: StructIT[cI], structDefH: StructDefinitionH): Hamuts = {
    vassert(structTToStructH.contains(structTT))
    // structTToStructDefH.get(structTT) match {
    //   case Some(existingDef) => {
    //     // Added all this to help VmdSiteGen. Apparently it calls this method twice with the same structs sometimes?
    //     vassert(existingDef.id == structDefH.id)
    //     vassert(existingDef.members.map(_.name) == structDefH.members.map(_.name))
    //     vassert(existingDef.members.map(_.tyype) == structDefH.members.map(_.tyype))
    //     vassert(structDefs.exists(_.id == structDefH.id))
    //     this
    //   }
    //   case None => {
        Hamuts(
          humanNameToFullNameToId,
          structTToOpaqueH,
          structTToStructH,
          structTToStructDefH + (structTT -> structDefH),
          structDefs :+ structDefH,
          staticSizedArrays,
          runtimeSizedArrays,
          interfaceTToInterfaceH,
          interfaceTToInterfaceDefH,
          functionRefs,
          functionDefs,
          packageCoordToExportNameToFunction,
          packageCoordToExportNameToKind,
          packageCoordToPrototypeToExtern,
          packageCoordToKindToExtern)
      // }
    // }
  }

  def addOpaque(structIT: StructIT[cI], opaqueH: OpaqueHT): Hamuts = {
    vassert(!structTToOpaqueH.contains(structIT)) // I think we only do this function once and use the cached thing
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH + (structIT -> opaqueH),
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def addStructOriginatingFromHammer(structDefH: StructDefinitionH): Hamuts = {
    vassert(!structDefs.exists(_.id == structDefH.id))

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs :+ structDefH,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def forwardDeclareInterface(interfaceIT: InterfaceIT[cI], interfaceRefH: InterfaceHT): Hamuts = {
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH + (interfaceIT -> interfaceRefH),
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def addInterface(interfaceIT: InterfaceIT[cI], interfaceDefH: InterfaceDefinitionH): Hamuts = {
    vassert(interfaceTToInterfaceH.contains(interfaceIT))
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH + (interfaceIT -> interfaceDefH),
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def forwardDeclareFunction(functionRef2: PrototypeI[cI], functionRefH: FunctionRefH): Hamuts = {
    vassert(!functionRefs.contains(functionRef2))

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs + (functionRef2 -> functionRefH),
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def addFunction(functionRef2: PrototypeI[cI], functionDefH: FunctionH): Hamuts = {
    vassert(functionRefs.contains(functionRef2))
    functionDefs.find(_._2.id == functionDefH.id) match {
      case None =>
      case Some(existing) => {
        vfail("Internal error: Can't add function:\n" + functionRef2 + "\nbecause there's already a function with same hammer name:\b" + existing._1 + "\nHammer name:\n" + functionDefH.id)
      }
    }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs + (functionRef2 -> functionDefH),
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def addKindExport(kind: KindHT, packageCoordinate: PackageCoordinate, exportedName: StrI): Hamuts = {
    val newPackageCoordToExportNameToKind =
      packageCoordToExportNameToKind.get(packageCoordinate) match {
        case None => {
          packageCoordToExportNameToKind + (packageCoordinate -> Map(exportedName -> kind))
        }
        case Some(exportNameToFullName) => {
          exportNameToFullName.get(exportedName) match {
            case None => {
              packageCoordToExportNameToKind + (packageCoordinate -> (exportNameToFullName + (exportedName -> kind)))
            }
            case Some(existingFullName) => {
              vfail("Already exported a `" + exportedName + "` from package `" + packageCoordinate + " : " + existingFullName)
            }
          }
        }
      }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      newPackageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def addFunctionExport(function: PrototypeH, packageCoordinate: PackageCoordinate, exportedName: StrI): Hamuts = {
    val newPackageCoordToExportNameToFunction =
      packageCoordToExportNameToFunction.get(packageCoordinate) match {
        case None => {
          packageCoordToExportNameToFunction + (packageCoordinate -> Map(exportedName -> function))
        }
        case Some(exportNameToFullName) => {
          exportNameToFullName.get(exportedName) match {
            case None => {
              packageCoordToExportNameToFunction + (packageCoordinate -> (exportNameToFullName + (exportedName -> function)))
            }
            case Some(existingFullName) => {
              vfail("Already exported a `" + exportedName + "` from package `" + packageCoordinate + " : " + existingFullName)
            }
          }
        }
      }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      newPackageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def addKindExtern(opaqueH: OpaqueHT, simpleId: SimpleId, exportedName: String): Hamuts = {
    val packageCoordinate = opaqueH.packageCoord
    val newPackageCoordToKindToExtern: Map[PackageCoordinate, Map[OpaqueHT, HamutsKindExtern]] =
      packageCoordToKindToExtern.get(packageCoordinate) match {
        case None => {
          packageCoordToKindToExtern + (packageCoordinate -> Map(opaqueH -> HamutsKindExtern(exportedName, opaqueH, simpleId)))
        }
        case Some(exportNameToFullName) => {
          exportNameToFullName.get(opaqueH) match {
            case None => {
              packageCoordToKindToExtern + (packageCoordinate -> (exportNameToFullName + (opaqueH -> HamutsKindExtern(exportedName, opaqueH, simpleId))))
            }
            case Some(existingFullName) => {
              vfail("Already exported a `" + exportedName + "` from package `" + packageCoordinate + " : " + existingFullName)
            }
          }
        }
      }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      newPackageCoordToKindToExtern)
  }

  def addFunctionExtern(function: PrototypeH, simpleId: SimpleId, exportedName: String): Hamuts = {
    val packageCoordinate = function.id.packageCoordinate
    val newPackageCoordToPrototypeToExtern =
      packageCoordToPrototypeToExtern.get(packageCoordinate) match {
        case None => {
          packageCoordToPrototypeToExtern + (packageCoordinate -> Map(function -> HamutsFunctionExtern(exportedName, function, simpleId)))
        }
        case Some(prototypeToExtern) => {
          prototypeToExtern.get(function) match {
            case None => {
              packageCoordToPrototypeToExtern + (packageCoordinate -> (prototypeToExtern + (function -> HamutsFunctionExtern(exportedName, function, simpleId))))
            }
            case Some(existingFullName) => {
              vfail("Already exported a `" + exportedName + "` from package `" + packageCoordinate + " : " + existingFullName)
            }
          }
        }
      }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      newPackageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def addStaticSizedArray(
    ssaIT: StaticSizedArrayIT[cI],
    staticSizedArrayDefinitionHT: StaticSizedArrayDefinitionHT
  ): Hamuts = {
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays + (ssaIT -> staticSizedArrayDefinitionHT),
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

  def addRuntimeSizedArray(
    rsaIT: RuntimeSizedArrayIT[cI],
    runtimeSizedArrayDefinitionHT: RuntimeSizedArrayDefinitionHT
  ): Hamuts = {
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays + (rsaIT -> runtimeSizedArrayDefinitionHT),
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

//  // This returns a unique ID for that specific human name.
//  // Two things with two different human names could result in the same ID here.
//  // This ID is meant to be concatenated onto the human name.
//  def getNameId(readableName: String, packageCoordinate: PackageCoordinate, parts: Vector[IVonData]): (Hamuts, Int) = {
//    val namePartsString = IdH.namePartsToString(packageCoordinate, parts)
//    val idByFullNameForHumanName =
//      humanNameToFullNameToId.get(readableName) match {
//        case None => Map[String, Int]()
//        case Some(x) => x
//      }
//    val id =
//      idByFullNameForHumanName.get(namePartsString) match {
//        case None => idByFullNameForHumanName.size
//        case Some(i) => i
//      }
//    val idByFullNameForHumanNameNew = idByFullNameForHumanName + (namePartsString -> id)
//    val idByFullNameByHumanNameNew = humanNameToFullNameToId + (readableName -> idByFullNameForHumanNameNew)
//    val newHamuts =
//      Hamuts(
//        idByFullNameByHumanNameNew,
//        structTToStructH,
//        structTToStructDefH,
//        structDefs,
//        staticSizedArrays,
//        runtimeSizedArrays,
//        interfaceTToInterfaceH,
//        interfaceTToInterfaceDefH,
//        functionRefs,
//        functionDefs,
//        packageCoordToExportNameToFunction,
//        packageCoordToExportNameToKind,
//        packageCoordToPrototypeToExtern,
//        packageCoordToKindToExtern)
//    (newHamuts, id)
//  }

  def getStaticSizedArray(staticSizedArrayHT: StaticSizedArrayHT): StaticSizedArrayDefinitionHT = {
    staticSizedArrays.values.find(_.kind == staticSizedArrayHT).get
  }
  def getRuntimeSizedArray(runtimeSizedArrayTH: RuntimeSizedArrayHT): RuntimeSizedArrayDefinitionHT = {
    runtimeSizedArrays.values.find(_.kind == runtimeSizedArrayTH).get
  }
}
