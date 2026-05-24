/*
package dev.vale.instantiating

import dev.vale.instantiating.ast._
import dev.vale.{vassertSome, vimpl, vwat}

import scala.collection.immutable.Map

// See ICRHRC for why/when we count the regions.
// This one will collapse every node based on only the things it contains.
// It creates a new collapsing map for each one.
object RegionCollapserIndividual {
*/
// mig: fn collapse_prototype
pub fn collapse_prototype() {
    panic!("Unimplemented: collapse_prototype")
}
/*
  def collapsePrototype(prototype: PrototypeI[sI]): PrototypeI[cI] = {
    val PrototypeI(id, returnType) = prototype
    PrototypeI(
      collapseFunctionId(id),
      collapseCoord(returnType))
  }
*/
// mig: fn collapse_id
pub fn collapse_id() {
    panic!("Unimplemented: collapse_id")
}
/*
  def collapseId[T <: INameI[sI], Y <: INameI[cI]](
    id: IdI[sI, T],
    func: T => Y):
  IdI[cI, Y] = {
    val IdI(packageCoord, initSteps, localName) = id
    IdI(
      packageCoord,
      initSteps.map(x => collapseName(x)),
      func(localName))
  }
*/
// mig: fn collapse_function_id
pub fn collapse_function_id() {
    panic!("Unimplemented: collapse_function_id")
}
/*
  def collapseFunctionId(
    id: IdI[sI, IFunctionNameI[sI]]):
  IdI[cI, IFunctionNameI[cI]] = {
    collapseId[IFunctionNameI[sI], IFunctionNameI[cI]](
      id,
      x => collapseFunctionName(x))
  }
*/
// mig: fn collapse_function_name
pub fn collapse_function_name() {
    panic!("Unimplemented: collapse_function_name")
}
/*
  def collapseFunctionName(
    name: IFunctionNameI[sI]):
  IFunctionNameI[cI] = {
    name match {
      case n @ FunctionNameIX(FunctionTemplateNameI(humanName, codeLocation), templateArgs, parameters) => {
        val map = RegionCounter.countFunctionName(n)
        val templateC = FunctionTemplateNameI[cI](humanName, codeLocation)
        val templateArgsC = templateArgs.map(collapseTemplata(map, _))
        val paramsC =
          parameters.map(param => {
            collapseCoord(param)
          })
        FunctionNameIX[cI](templateC, templateArgsC, paramsC)
      }
      case n @ ExternFunctionNameI(humanName, templateArgs, parameters) => {
        val map = RegionCounter.countFunctionName(n)
        val paramsC =
          parameters.map(param => {
            collapseCoord(param)
          })
        val templateArgsC = templateArgs.map(collapseTemplata(map, _))
        ExternFunctionNameI[cI](humanName, templateArgsC, paramsC)
      }
      case n @ LambdaCallFunctionNameI(LambdaCallFunctionTemplateNameI(codeLocation, paramsTT), templateArgs, parameters) => {
        val map = RegionCounter.countFunctionName(n)
        val templateC = LambdaCallFunctionTemplateNameI[cI](codeLocation, paramsTT)
        val templateArgsC = templateArgs.map(collapseTemplata(map, _))
        val paramsC =
          parameters.map(param => {
            collapseCoord(param)
          })
        LambdaCallFunctionNameI[cI](templateC, templateArgsC, paramsC)
      }
      case n @ AnonymousSubstructConstructorNameI(AnonymousSubstructConstructorTemplateNameI(substruct), templateArgs, parameters) => {
        val map = RegionCounter.countFunctionName(n)
        val templateC = AnonymousSubstructConstructorTemplateNameI[cI](collapseCitizenTemplateName(substruct))
        val templateArgsC = templateArgs.map(collapseTemplata(map, _))
        val paramsC =
          parameters.map(param => {
            collapseCoord(param)
          })
        AnonymousSubstructConstructorNameI[cI](templateC, templateArgsC, paramsC)
      }
      // case n@OverrideDispatcherNameI(OverrideDispatcherTemplateNameI(implId), templateArgs, parameters) => {
      //   val map = RegionCounter.countFunctionName(n)
      //   val templateC = OverrideDispatcherTemplateNameI[cI](collapseImplTemplateId(implId))
      //   val templateArgsC = templateArgs.map(collapseTemplata(map, _))
      //   val paramsC =
      //     parameters.map(param => {
      //       collapseCoord(param)
      //     })
      //   OverrideDispatcherNameI[cI](templateC, templateArgsC, paramsC)
      // }
      // case n@CaseFunctionFromImplNameI(CaseFunctionFromImplTemplateNameI(humanName, runeInImpl, runeInCitizen), templateArgs, parameters) => {
      //   val map = RegionCounter.countFunctionName(n)
      //   val templateC = CaseFunctionFromImplTemplateNameI[cI](humanName, runeInImpl, runeInCitizen)
      //   val templateArgsC = templateArgs.map(collapseTemplata(map, _))
      //   val paramsC =
      //     parameters.map(param => {
      //       collapseCoord(param)
      //     })
      //   CaseFunctionFromImplNameI[cI](templateC, templateArgsC, paramsC)
      // }
      case ForwarderFunctionNameI(ForwarderFunctionTemplateNameI(innerTemplate, index), funcName) => {
        ForwarderFunctionNameI(
          ForwarderFunctionTemplateNameI(collapseFunctionTemplateName(innerTemplate), index),
          collapseFunctionName(funcName))
      }
      case other => vimpl(other)
    }
  }
*/
// mig: fn collapse_citizen_template_name
pub fn collapse_citizen_template_name() {
    panic!("Unimplemented: collapse_citizen_template_name")
}
/*
  def collapseCitizenTemplateName(citizen: ICitizenTemplateNameI[sI]): ICitizenTemplateNameI[cI] = {
    citizen match {
      case s : IStructTemplateNameI[_] => {
        collapseStructTemplateName(s.asInstanceOf[IStructTemplateNameI[sI]])
      }
      case i : IInterfaceTemplateNameI[_] => {
        collapseInterfaceTemplateName(i.asInstanceOf[IInterfaceTemplateNameI[sI]])
      }
      case other => vimpl(other)
    }
  }
*/
// mig: fn collapse_var_name
pub fn collapse_var_name() {
    panic!("Unimplemented: collapse_var_name")
}
/*
  def collapseVarName(
    name: IVarNameI[sI]):
  IVarNameI[cI] = {
    name match {
      case TypingPassBlockResultVarNameI(life) => TypingPassBlockResultVarNameI(life)
      case CodeVarNameI(name) => CodeVarNameI(name)
      case TypingPassTemporaryVarNameI(life) => TypingPassTemporaryVarNameI(life)
      case TypingPassFunctionResultVarNameI() => TypingPassFunctionResultVarNameI()
      case ClosureParamNameI(codeLocation) => ClosureParamNameI(codeLocation)
      case MagicParamNameI(codeLocation2) => MagicParamNameI(codeLocation2)
      case IterableNameI(range) => IterableNameI(range)
      case ConstructingMemberNameI(name) => ConstructingMemberNameI(name)
      case IteratorNameI(range) => IteratorNameI(range)
      case IterationOptionNameI(range) => IterationOptionNameI(range)
      case SelfNameI() => SelfNameI()
    }
  }
*/
// mig: fn collapse_function_template_name
pub fn collapse_function_template_name() {
    panic!("Unimplemented: collapse_function_template_name")
}
/*
  def collapseFunctionTemplateName(
      functionName: IFunctionTemplateNameI[sI]):
  IFunctionTemplateNameI[cI] = {
    functionName match {
      case FunctionTemplateNameI(humanName, codeLocation) => FunctionTemplateNameI(humanName, codeLocation)
    }
  }
*/
// mig: fn collapse_name
pub fn collapse_name() {
    panic!("Unimplemented: collapse_name")
}
/*
  def collapseName(
    name: INameI[sI]):
  INameI[cI] = {
    name match {
      case n : IFunctionNameI[_] => collapseFunctionName(n.asInstanceOf[IFunctionNameI[sI]])
      case x : IFunctionTemplateNameI[_] => collapseFunctionTemplateName(x.asInstanceOf[IFunctionTemplateNameI[sI]])
      case StructTemplateNameI(humanName) => StructTemplateNameI(humanName)
      case s @ StructNameI(_,_) => collapseStructName(s)
      case x @ LambdaCitizenNameI(_) => collapseStructName(x)
      case LambdaCitizenTemplateNameI(codeLocation) => LambdaCitizenTemplateNameI(codeLocation)
      case n @ LambdaCallFunctionNameI(LambdaCallFunctionTemplateNameI(codeLocation, paramTypes), templateArgs, parameters) => collapseFunctionName(n)
      case InterfaceTemplateNameI(humanName) => InterfaceTemplateNameI(humanName)
      case AnonymousSubstructTemplateNameI(interface) => AnonymousSubstructTemplateNameI(collapseInterfaceTemplateName(interface))
      case other => vimpl(other)
    }
  }
*/
// mig: fn collapse_coord_templata
pub fn collapse_coord_templata() {
    panic!("Unimplemented: collapse_coord_templata")
}
/*
  def collapseCoordTemplata(
      map: Map[Int, Int],
      templata: CoordTemplataI[sI]):
  CoordTemplataI[cI] = {
    val CoordTemplataI(region, coord) = templata
    CoordTemplataI(collapseRegionTemplata(map, region), collapseCoord(coord))
  }
*/
// mig: fn collapse_templata
pub fn collapse_templata() {
    panic!("Unimplemented: collapse_templata")
}
/*
  def collapseTemplata(
    map: Map[Int, Int],
    templata: ITemplataI[sI]):
  ITemplataI[cI] = {
    templata match {
      case c @ CoordTemplataI(_, _) => collapseCoordTemplata(map, c)
      case KindTemplataI(kind) => KindTemplataI(collapseKind(kind))
      case r @ RegionTemplataI(_) => collapseRegionTemplata(map, r)
      case MutabilityTemplataI(mutability) => MutabilityTemplataI(mutability)
      case IntegerTemplataI(x) => IntegerTemplataI(x)
      case VariabilityTemplataI(variability) => VariabilityTemplataI(variability)
      case other => vimpl(other)
    }
  }
*/
// mig: fn collapse_region_templata
pub fn collapse_region_templata() {
    panic!("Unimplemented: collapse_region_templata")
}
/*
  def collapseRegionTemplata(
    map: Map[Int, Int],
    templata: RegionTemplataI[sI]):
  RegionTemplataI[cI] = {
    val RegionTemplataI(oldPureHeight) = templata
    RegionTemplataI[cI](vassertSome(map.get(oldPureHeight)))
  }
*/
// mig: fn collapse_coord
pub fn collapse_coord() {
    panic!("Unimplemented: collapse_coord")
}
/*
  def collapseCoord(
    coord: CoordI[sI]):
  CoordI[cI] = {
    val CoordI(ownership, kind) = coord
    CoordI(ownership, collapseKind(kind))
  }
*/
// mig: fn collapse_kind
pub fn collapse_kind() {
    panic!("Unimplemented: collapse_kind")
}
/*
  def collapseKind(
    kind: KindIT[sI]):
  KindIT[cI] = {
    kind match {
      case NeverIT(fromBreak) => NeverIT(fromBreak)
      case VoidIT() => VoidIT()
      case IntIT(x) => IntIT(x)
      case BoolIT() => BoolIT()
      case FloatIT() => FloatIT()
      case StrIT() => StrIT()
      case StructIT(id) => StructIT(collapseStructId(id))
      case InterfaceIT(id) => InterfaceIT(collapseInterfaceId(id))
      case ssa @ StaticSizedArrayIT(_) => collapseStaticSizedArray(ssa)
      case rsa @ RuntimeSizedArrayIT(_) => collapseRuntimeSizedArray(rsa)
    }
  }
*/
// mig: fn collapse_runtime_sized_array
pub fn collapse_runtime_sized_array() {
    panic!("Unimplemented: collapse_runtime_sized_array")
}
/*
  def collapseRuntimeSizedArray(
    rsa: RuntimeSizedArrayIT[sI]):
  RuntimeSizedArrayIT[cI] = {
    val RuntimeSizedArrayIT(rsaId) = rsa
    val map = RegionCounter.countRuntimeSizedArray(rsa)
    RuntimeSizedArrayIT(
      collapseId[RuntimeSizedArrayNameI[sI], RuntimeSizedArrayNameI[cI]](
        rsaId,
        { case RuntimeSizedArrayNameI(RuntimeSizedArrayTemplateNameI(), RawArrayNameI(mutability, elementType, selfRegion)) =>
          RuntimeSizedArrayNameI(
            RuntimeSizedArrayTemplateNameI(),
            RawArrayNameI(
              mutability,
              collapseTemplata(map, elementType).expectCoordTemplata(),
              collapseRegionTemplata(map, selfRegion)))
        }))
  }
*/
// mig: fn collapse_static_sized_array
pub fn collapse_static_sized_array() {
    panic!("Unimplemented: collapse_static_sized_array")
}
/*
  def collapseStaticSizedArray(
    ssa: StaticSizedArrayIT[sI]):
  StaticSizedArrayIT[cI] = {
    val StaticSizedArrayIT(ssaId) = ssa
    val map = RegionCounter.countStaticSizedArray(ssa)
    StaticSizedArrayIT(
      collapseId[StaticSizedArrayNameI[sI], StaticSizedArrayNameI[cI]](
        ssaId,
        { case StaticSizedArrayNameI(StaticSizedArrayTemplateNameI(), size, variability, RawArrayNameI(mutability, elementType, selfRegion)) =>
          StaticSizedArrayNameI(
            StaticSizedArrayTemplateNameI(),
            size,
            variability,
            RawArrayNameI(
              mutability,
              collapseTemplata(map, elementType).expectCoordTemplata(),
              collapseRegionTemplata(map, selfRegion)))
        }))
  }
*/
// mig: fn collapse_interface_id
pub fn collapse_interface_id() {
    panic!("Unimplemented: collapse_interface_id")
}
/*
  def collapseInterfaceId(
      interfaceId: IdI[sI, IInterfaceNameI[sI]]):
  IdI[cI, IInterfaceNameI[cI]] = {
    collapseId[IInterfaceNameI[sI], IInterfaceNameI[cI]](
      interfaceId,
      x => collapseInterfaceName(x))
  }
*/
// mig: fn collapse_struct_id
pub fn collapse_struct_id() {
    panic!("Unimplemented: collapse_struct_id")
}
/*
  def collapseStructId(
    structId: IdI[sI, IStructNameI[sI]]):
  IdI[cI, IStructNameI[cI]] = {
    collapseId[IStructNameI[sI], IStructNameI[cI]](
      structId,
      x => collapseStructName(x))
  }
*/
// mig: fn collapse_struct_name
pub fn collapse_struct_name() {
    panic!("Unimplemented: collapse_struct_name")
}
/*
  def collapseStructName(
      structName: IStructNameI[sI]):
  IStructNameI[cI] = {
    structName match {
      case StructNameI(template, templateArgs) => {
        val map = RegionCounter.countCitizenName(structName)
        StructNameI(
          collapseStructTemplateName(template),
          templateArgs.map(collapseTemplata(map, _)))
      }
      case LambdaCitizenNameI(LambdaCitizenTemplateNameI(codeLocation)) => {
        LambdaCitizenNameI(LambdaCitizenTemplateNameI(codeLocation))
      }
      case AnonymousSubstructNameI(AnonymousSubstructTemplateNameI(interface), templateArgs) => {
        val map = RegionCounter.countCitizenName(structName)
        AnonymousSubstructNameI(
          AnonymousSubstructTemplateNameI(collapseInterfaceTemplateName(interface)),
          templateArgs.map(collapseTemplata(map, _)))
      }
    }
  }
*/
// mig: fn collapse_impl_name
pub fn collapse_impl_name() {
    panic!("Unimplemented: collapse_impl_name")
}
/*
  def collapseImplName(
      implName: IImplNameI[sI]):
  IImplNameI[cI] = {
    implName match {
      case ImplNameI(template, templateArgs, subCitizen) => {
        val map = RegionCounter.countImplName(implName)
        ImplNameI[cI](
          collapseImplTemplateName(template),
          templateArgs.map(collapseTemplata(map, _)),
          collapseCitizen(subCitizen))
      }
      case ImplBoundNameI(ImplBoundTemplateNameI(codeLocationS), templateArgs) => {
        val map = RegionCounter.countImplName(implName)
        ImplBoundNameI(
          ImplBoundTemplateNameI(codeLocationS),
          templateArgs.map(collapseTemplata(map, _)))
      }
      case AnonymousSubstructImplNameI(AnonymousSubstructImplTemplateNameI(interface), templateArgs, subCitizen) => {
        val map = RegionCounter.countImplName(implName)
        AnonymousSubstructImplNameI[cI](
          AnonymousSubstructImplTemplateNameI(collapseInterfaceTemplateName(interface)),
          templateArgs.map(collapseTemplata(map, _)),
          collapseCitizen(subCitizen))
      }
    }
  }
*/
// mig: fn collapse_interface_name
pub fn collapse_interface_name() {
    panic!("Unimplemented: collapse_interface_name")
}
/*
  def collapseInterfaceName(
      interfaceName: IInterfaceNameI[sI]):
  IInterfaceNameI[cI] = {
    interfaceName match {
      case InterfaceNameI(InterfaceTemplateNameI(humanNamee), templateArgs) => {
        val map = RegionCounter.countCitizenName(interfaceName)
        InterfaceNameI(
          InterfaceTemplateNameI(humanNamee),
          templateArgs.map(collapseTemplata(map, _)))
      }
      case other => vimpl(other)
    }
  }
*/
// mig: fn collapse_export_id
pub fn collapse_export_id() {
    panic!("Unimplemented: collapse_export_id")
}
/*
  def collapseExportId(
    map: Map[Int, Int],
    structId: IdI[sI, ExportNameI[sI]]):
  IdI[cI, ExportNameI[cI]] = {
    collapseId[ExportNameI[sI], ExportNameI[cI]](
      structId,
      { case ExportNameI(ExportTemplateNameI(codeLoc), templateArg) =>
        ExportNameI(
          ExportTemplateNameI(codeLoc),
          collapseRegionTemplata(map, templateArg))
      })
  }
*/
// mig: fn collapse_extern_id
pub fn collapse_extern_id() {
    panic!("Unimplemented: collapse_extern_id")
}
/*
  def collapseExternId(
    map: Map[Int, Int],
    structId: IdI[sI, ExternNameI[sI]]):
  IdI[cI, ExternNameI[cI]] = {
    collapseId[ExternNameI[sI], ExternNameI[cI]](
      structId,
      { case ExternNameI(ExternTemplateNameI(codeLoc), templateArg) =>
        ExternNameI(
          ExternTemplateNameI(codeLoc),
          collapseRegionTemplata(map, templateArg))
      })
  }
*/
// mig: fn collapse_struct_template_name
pub fn collapse_struct_template_name() {
    panic!("Unimplemented: collapse_struct_template_name")
}
/*
  def collapseStructTemplateName(
    structName: IStructTemplateNameI[sI]):
  IStructTemplateNameI[cI] = {
    structName match {
      case StructTemplateNameI(humanName) => StructTemplateNameI(humanName)
      case AnonymousSubstructTemplateNameI(interface) => AnonymousSubstructTemplateNameI(collapseInterfaceTemplateName(interface))
    }
  }
*/
// mig: fn collapse_interface_template_name
pub fn collapse_interface_template_name() {
    panic!("Unimplemented: collapse_interface_template_name")
}
/*
  def collapseInterfaceTemplateName(
      structName: IInterfaceTemplateNameI[sI]):
  IInterfaceTemplateNameI[cI] = {
    structName match {
      case InterfaceTemplateNameI(humanName) => InterfaceTemplateNameI(humanName)
    }
  }
*/
// mig: fn collapse_impl_id
pub fn collapse_impl_id() {
    panic!("Unimplemented: collapse_impl_id")
}
/*
  def collapseImplId(
      implId: IdI[sI, IImplNameI[sI]]):
  IdI[cI, IImplNameI[cI]] = {
    collapseId[IImplNameI[sI], IImplNameI[cI]](
      implId,
      x => collapseImplName(x))
  }
  //
  // def collapseImplTemplateId(
  //     implId: IdI[sI, IImplTemplateNameI[sI]]):
  // IdI[cI, IImplTemplateNameI[cI]] = {
  //   collapseId[IImplTemplateNameI[sI], IImplTemplateNameI[cI]](
  //     implId,
  //     x => collapseImplTemplateName(x))
  // }
*/
// mig: fn collapse_impl_template_name
pub fn collapse_impl_template_name() {
    panic!("Unimplemented: collapse_impl_template_name")
}
/*
  def collapseImplTemplateName(
    structName: IImplTemplateNameI[sI]):
  IImplTemplateNameI[cI] = {
    structName match {
      case ImplTemplateNameI(humanName) => ImplTemplateNameI(humanName)
      case AnonymousSubstructImplTemplateNameI(interface) => AnonymousSubstructImplTemplateNameI(collapseInterfaceTemplateName(interface))
    }
  }
*/
// mig: fn collapse_citizen
pub fn collapse_citizen() {
    panic!("Unimplemented: collapse_citizen")
}
/*
  def collapseCitizen(
      citizen: ICitizenIT[sI]):
  ICitizenIT[cI] = {
    citizen match {
      case StructIT(structIdT) => StructIT(collapseStructId(structIdT))
      case InterfaceIT(interfaceIdT) => InterfaceIT(collapseInterfaceId(interfaceIdT))
    }
  }

}
*/