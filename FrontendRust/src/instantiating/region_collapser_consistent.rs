/*
package dev.vale.instantiating

import dev.vale.instantiating.ast._
import dev.vale.{vassertSome, vimpl}

// See ICRHRC for why/when we count the regions.
// This one will use one map for the entire deep collapse, rather than making a new map for everything.
object RegionCollapserConsistent {

*/
// mig: fn collapse_prototype
pub fn collapse_prototype() { panic!("Unimplemented: collapse_prototype"); }
/*
  def collapsePrototype(map: Map[Int, Int], prototype: PrototypeI[sI]): PrototypeI[nI] = {
    val PrototypeI(id, returnType) = prototype
    PrototypeI(
      collapseFunctionId(map, id),
      collapseCoord(map, returnType))
  }

*/
// mig: fn collapse_id
pub fn collapse_id() { panic!("Unimplemented: collapse_id"); }
/*
  def collapseId[T <: INameI[sI], Y <: INameI[nI]](
      map: Map[Int, Int],
      id: IdI[sI, T],
      func: T => Y):
  IdI[nI, Y] = {
    val IdI(packageCoord, initSteps, localName) = id
    IdI(
      packageCoord,
      initSteps.map(x => collapseName(map, x)),
      func(localName))
  }

*/
// mig: fn collapse_function_id
pub fn collapse_function_id() { panic!("Unimplemented: collapse_function_id"); }
/*
  def collapseFunctionId(
      map: Map[Int, Int],
      id: IdI[sI, IFunctionNameI[sI]]):
  IdI[nI, IFunctionNameI[nI]] = {
    collapseId[IFunctionNameI[sI], IFunctionNameI[nI]](
      map,
      id,
      x => collapseFunctionName(map, x))
  }

*/
// mig: fn collapse_function_name
pub fn collapse_function_name() { panic!("Unimplemented: collapse_function_name"); }
/*
  def collapseFunctionName(
      map: Map[Int, Int],
      name: IFunctionNameI[sI]):
  IFunctionNameI[nI] = {
    name match {
      case n @ FunctionNameIX(FunctionTemplateNameI(humanName, codeLocation), templateArgs, parameters) => {
        val templateC = FunctionTemplateNameI[nI](humanName, codeLocation)
        val templateArgsC = templateArgs.map(collapseTemplata(map, _))
        val paramsC =
          parameters.map(param => {
            collapseCoord(map, param)
          })
        FunctionNameIX[nI](templateC, templateArgsC, paramsC)
      }
      case ExternFunctionNameI(humanName, templateArgs, parameters) => {
        val paramsC =
          parameters.map(param => {
            collapseCoord(map, param)
          })
        val templateArgsC = templateArgs.map(collapseTemplata(map, _))
        ExternFunctionNameI[nI](humanName, templateArgsC, paramsC)
      }
      case LambdaCallFunctionNameI(LambdaCallFunctionTemplateNameI(codeLocation, paramsTT), templateArgs, parameters) => {
        val templateC = LambdaCallFunctionTemplateNameI[nI](codeLocation, paramsTT)
        val templateArgsC = templateArgs.map(collapseTemplata(map, _))
        val paramsC =
          parameters.map(param => {
            collapseCoord(map, param)
          })
        LambdaCallFunctionNameI[nI](templateC, templateArgsC, paramsC)
      }
      case AnonymousSubstructConstructorNameI(AnonymousSubstructConstructorTemplateNameI(substruct), templateArgs, parameters) => {
        val templateC = AnonymousSubstructConstructorTemplateNameI[nI](collapseCitizenTemplateName(substruct))
        val templateArgsC = templateArgs.map(collapseTemplata(map, _))
        val paramsC =
          parameters.map(param => {
            collapseCoord(map, param)
          })
        AnonymousSubstructConstructorNameI[nI](templateC, templateArgsC, paramsC)
      }
      // case OverrideDispatcherNameI(OverrideDispatcherTemplateNameI(implId), templateArgs, parameters) => {
      //   val templateC = OverrideDispatcherTemplateNameI[nI](collapseImplTemplateId(map, implId))
      //   val templateArgsC = templateArgs.map(collapseTemplata(map, _))
      //   val paramsC =
      //     parameters.map(param => {
      //       collapseCoord(map, param)
      //     })
      //   OverrideDispatcherNameI[nI](templateC, templateArgsC, paramsC)
      // }
      // case CaseFunctionFromImplNameI(CaseFunctionFromImplTemplateNameI(humanName, runeInImpl, runeInCitizen), templateArgs, parameters) => {
      //   val templateC = CaseFunctionFromImplTemplateNameI[nI](humanName, runeInImpl, runeInCitizen)
      //   val templateArgsC = templateArgs.map(collapseTemplata(map, _))
      //   val paramsC =
      //     parameters.map(param => {
      //       collapseCoord(map, param)
      //     })
      //   CaseFunctionFromImplNameI[nI](templateC, templateArgsC, paramsC)
      // }
      case ForwarderFunctionNameI(ForwarderFunctionTemplateNameI(funcTemplateName, index), funcName) => {
        ForwarderFunctionNameI(
          ForwarderFunctionTemplateNameI(collapseFunctionTemplateName(funcTemplateName), index),
          collapseFunctionName(map, funcName))
      }
    }
  }

*/
// mig: fn collapse_var_name
pub fn collapse_var_name() { panic!("Unimplemented: collapse_var_name"); }
/*
  def collapseVarName(
    name: IVarNameI[sI]):
  IVarNameI[sI] = {
    name match {
      case TypingPassBlockResultVarNameI(life) => TypingPassBlockResultVarNameI(life)
      case CodeVarNameI(name) => CodeVarNameI(name)
      case TypingPassTemporaryVarNameI(life) => TypingPassTemporaryVarNameI(life)
      case TypingPassFunctionResultVarNameI() => TypingPassFunctionResultVarNameI()
    }
  }

*/
// mig: fn collapse_name
pub fn collapse_name() { panic!("Unimplemented: collapse_name"); }
/*
  def collapseName(
      map: Map[Int, Int],
      name: INameI[sI]):
  INameI[nI] = {
    name match {
      case s: IStructTemplateNameI[_] => {
        collapseStructTemplateName(s.asInstanceOf[IStructTemplateNameI[sI]])
      }
      case s: IInterfaceTemplateNameI[_] => {
        collapseInterfaceTemplateName(s.asInstanceOf[IInterfaceTemplateNameI[sI]])
      }
      case s: IFunctionTemplateNameI[_] => {
        collapseFunctionTemplateName(s.asInstanceOf[IFunctionTemplateNameI[sI]])
      }
      case n : IFunctionNameI[_] => {
        collapseFunctionName(map, n.asInstanceOf[IFunctionNameI[sI]])
      }
      case n @ LambdaCallFunctionNameI(_, _, _) => collapseFunctionName(map, n)
      case s @ StructNameI(_, _) => collapseStructName(map, s)
      case other => vimpl(other)
    }
  }

*/
// mig: fn collapse_coord_templata
pub fn collapse_coord_templata() { panic!("Unimplemented: collapse_coord_templata"); }
/*
  def collapseCoordTemplata(
      map: Map[Int, Int],
      templata: CoordTemplataI[sI]):
  CoordTemplataI[nI] = {
    val CoordTemplataI(region, coord) = templata
    CoordTemplataI(collapseRegionTemplata(map, region), collapseCoord(map, coord))
  }

*/
// mig: fn collapse_templata
pub fn collapse_templata() { panic!("Unimplemented: collapse_templata"); }
/*
  def collapseTemplata(
    map: Map[Int, Int],
    templata: ITemplataI[sI]):
  ITemplataI[nI] = {
    templata match {
      case c @ CoordTemplataI(_, _) => collapseCoordTemplata(map, c)
      case KindTemplataI(kind) => KindTemplataI(collapseKind(map, kind))
      case r @ RegionTemplataI(_) => collapseRegionTemplata(map, r)
      case MutabilityTemplataI(mutability) => MutabilityTemplataI(mutability)
      case IntegerTemplataI(x) => IntegerTemplataI(x)
      case VariabilityTemplataI(variability) => VariabilityTemplataI(variability)
      case other => vimpl(other)
    }
  }

*/
// mig: fn collapse_region_templata
pub fn collapse_region_templata() { panic!("Unimplemented: collapse_region_templata"); }
/*
  def collapseRegionTemplata(
    map: Map[Int, Int],
    templata: RegionTemplataI[sI]):
  RegionTemplataI[nI] = {
    val RegionTemplataI(oldPureHeight) = templata
    RegionTemplataI[nI](vassertSome(map.get(oldPureHeight)))
  }

*/
// mig: fn collapse_coord
pub fn collapse_coord() { panic!("Unimplemented: collapse_coord"); }
/*
  def collapseCoord(
      map: Map[Int, Int],
      coord: CoordI[sI]):
  CoordI[nI] = {
    val CoordI(ownership, kind) = coord
    CoordI(ownership, collapseKind(map, kind))
  }

*/
// mig: fn collapse_kind
pub fn collapse_kind() { panic!("Unimplemented: collapse_kind"); }
/*
  def collapseKind(
      map: Map[Int, Int],
      kind: KindIT[sI]):
  KindIT[nI] = {
    kind match {
      case NeverIT(fromBreak) => NeverIT(fromBreak)
      case VoidIT() => VoidIT()
      case IntIT(x) => IntIT(x)
      case BoolIT() => BoolIT()
      case FloatIT() => FloatIT()
      case StrIT() => StrIT()
      case StructIT(id) => StructIT(collapseStructId(map, id))
      case InterfaceIT(id) => InterfaceIT(collapseInterfaceId(map, id))
      case ssa @ StaticSizedArrayIT(_) => collapseStaticSizedArray(map, ssa)
      case rsa @ RuntimeSizedArrayIT(_) => collapseRuntimeSizedArray(map, rsa)
    }
  }

*/
// mig: fn collapse_runtime_sized_array
pub fn collapse_runtime_sized_array() { panic!("Unimplemented: collapse_runtime_sized_array"); }
/*
  def collapseRuntimeSizedArray(
    map: Map[Int, Int],
    rsa: RuntimeSizedArrayIT[sI]):
  RuntimeSizedArrayIT[nI] = {
    val RuntimeSizedArrayIT(ssaId) = rsa
    RuntimeSizedArrayIT(
      collapseId[RuntimeSizedArrayNameI[sI], RuntimeSizedArrayNameI[nI]](
        map,
        ssaId,
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
pub fn collapse_static_sized_array() { panic!("Unimplemented: collapse_static_sized_array"); }
/*
  def collapseStaticSizedArray(
    map: Map[Int, Int],
    ssa: StaticSizedArrayIT[sI]):
  StaticSizedArrayIT[nI] = {
    val StaticSizedArrayIT(ssaId) = ssa
    StaticSizedArrayIT(
      collapseId[StaticSizedArrayNameI[sI], StaticSizedArrayNameI[nI]](
        map,
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
// mig: fn collapse_citizen
pub fn collapse_citizen() { panic!("Unimplemented: collapse_citizen"); }
/*
  def collapseCitizen(
      map: Map[Int, Int],
      citizen: ICitizenIT[sI]):
  ICitizenIT[nI] = {
    citizen match {
      case StructIT(structId) => StructIT(collapseStructId(map, structId))
      case InterfaceIT(structId) => InterfaceIT(collapseInterfaceId(map, structId))
    }
  }

*/
// mig: fn collapse_citizen_id
pub fn collapse_citizen_id() { panic!("Unimplemented: collapse_citizen_id"); }
/*
  def collapseCitizenId(
      map: Map[Int, Int],
      implId: IdI[sI, ICitizenNameI[sI]]):
  IdI[nI, ICitizenNameI[nI]] = {
    collapseId[ICitizenNameI[sI], ICitizenNameI[nI]](
      map,
      implId,
      collapseCitizenName(map, _))
  }

*/
// mig: fn collapse_citizen_name
pub fn collapse_citizen_name() { panic!("Unimplemented: collapse_citizen_name"); }
/*
  def collapseCitizenName(
      map: Map[Int, Int],
      citizenName: ICitizenNameI[sI]):
  ICitizenNameI[nI] = {
    citizenName match {
      case s: IStructNameI[_] => collapseStructName(map, s.asInstanceOf[IStructNameI[sI]])
      case i: IInterfaceNameI[_] => collapseInterfaceName(map, i.asInstanceOf[IInterfaceNameI[sI]])
    }
  }

*/
// mig: fn collapse_struct_name
pub fn collapse_struct_name() { panic!("Unimplemented: collapse_struct_name"); }
/*
  def collapseStructName(
      map: Map[Int, Int],
      structName: IStructNameI[sI]):
  IStructNameI[nI] = {
    structName match {
      case StructNameI(template, templateArgs) => {
        StructNameI(
          collapseStructTemplateName(template),
          templateArgs.map(collapseTemplata(map, _)))
      }
      case LambdaCitizenNameI(LambdaCitizenTemplateNameI(codeLocation)) => {
        LambdaCitizenNameI(LambdaCitizenTemplateNameI(codeLocation))
      }
      case AnonymousSubstructNameI(AnonymousSubstructTemplateNameI(interface), templateArgs) => {
        AnonymousSubstructNameI(
          AnonymousSubstructTemplateNameI(collapseInterfaceTemplateName(interface)),
          templateArgs.map(collapseTemplata(map, _)))
      }
    }
  }

*/
// mig: fn collapse_struct_id
pub fn collapse_struct_id() { panic!("Unimplemented: collapse_struct_id"); }
/*
  def collapseStructId(
    map: Map[Int, Int],
    structId: IdI[sI, IStructNameI[sI]]):
  IdI[nI, IStructNameI[nI]] = {
    collapseId[IStructNameI[sI], IStructNameI[nI]](
      map,
      structId,
      collapseStructName(map, _))
  }

*/
// mig: fn collapse_interface_name
pub fn collapse_interface_name() { panic!("Unimplemented: collapse_interface_name"); }
/*
  def collapseInterfaceName(
      map: Map[Int, Int],
      interfaceName: IInterfaceNameI[sI]):
  IInterfaceNameI[nI] = {
    interfaceName match {
      case InterfaceNameI(template, templateArgs) => {
        InterfaceNameI(
          collapseInterfaceTemplateName(template),
          templateArgs.map(collapseTemplata(map, _)))
      }
    }
  }

*/
// mig: fn collapse_interface_id
pub fn collapse_interface_id() { panic!("Unimplemented: collapse_interface_id"); }
/*
  def collapseInterfaceId(
      map: Map[Int, Int],
      interfaceId: IdI[sI, IInterfaceNameI[sI]]):
  IdI[nI, IInterfaceNameI[nI]] = {
    collapseId[IInterfaceNameI[sI], IInterfaceNameI[nI]](
      map,
      interfaceId,
      collapseInterfaceName(map, _))
  }

*/
// mig: fn collapse_export_id
pub fn collapse_export_id() { panic!("Unimplemented: collapse_export_id"); }
/*
  def collapseExportId(
    map: Map[Int, Int],
    structId: IdI[sI, ExportNameI[sI]]):
  IdI[nI, ExportNameI[nI]] = {
    collapseId[ExportNameI[sI], ExportNameI[nI]](
      map,
      structId,
      { case ExportNameI(ExportTemplateNameI(codeLoc), templateArg) =>
        ExportNameI(
          ExportTemplateNameI(codeLoc),
          collapseRegionTemplata(map, templateArg))
      })
  }

*/
// mig: fn collapse_extern_id
pub fn collapse_extern_id() { panic!("Unimplemented: collapse_extern_id"); }
/*
  def collapseExternId(
    map: Map[Int, Int],
    structId: IdI[sI, ExternNameI[sI]]):
  IdI[nI, ExternNameI[nI]] = {
    collapseId[ExternNameI[sI], ExternNameI[nI]](
      map,
      structId,
      { case ExternNameI(ExternTemplateNameI(codeLoc), templateArg) =>
        ExternNameI(
          ExternTemplateNameI(codeLoc),
          collapseRegionTemplata(map, templateArg))
      })
  }

*/
// mig: fn collapse_citizen_template_name
pub fn collapse_citizen_template_name() { panic!("Unimplemented: collapse_citizen_template_name"); }
/*
  def collapseCitizenTemplateName(
      citizenName: ICitizenTemplateNameI[sI]):
  ICitizenTemplateNameI[nI] = {
    citizenName match {
      case s : IStructTemplateNameI[_] => {
        collapseStructTemplateName(s.asInstanceOf[IStructTemplateNameI[nI]])
      }
      case s: IInterfaceTemplateNameI[_] => {
        collapseInterfaceTemplateName(s.asInstanceOf[IInterfaceTemplateNameI[nI]])
      }
    }
  }

*/
// mig: fn collapse_struct_template_name
pub fn collapse_struct_template_name() { panic!("Unimplemented: collapse_struct_template_name"); }
/*
  def collapseStructTemplateName(
    structName: IStructTemplateNameI[sI]):
  IStructTemplateNameI[nI] = {
    structName match {
      case StructTemplateNameI(humanName) => StructTemplateNameI(humanName)
      case AnonymousSubstructTemplateNameI(interface) => AnonymousSubstructTemplateNameI(collapseInterfaceTemplateName(interface))
      case LambdaCitizenTemplateNameI(codeLocation) => LambdaCitizenTemplateNameI(codeLocation)
    }
  }

*/
// mig: fn collapse_function_template_name
pub fn collapse_function_template_name() { panic!("Unimplemented: collapse_function_template_name"); }
/*
  def collapseFunctionTemplateName(
      structName: IFunctionTemplateNameI[sI]):
  IFunctionTemplateNameI[nI] = {
    structName match {
      case FunctionTemplateNameI(humanName, codeLocation) => FunctionTemplateNameI(humanName, codeLocation)
    }
  }

*/
// mig: fn collapse_interface_template_name
pub fn collapse_interface_template_name() { panic!("Unimplemented: collapse_interface_template_name"); }
/*
  def collapseInterfaceTemplateName(
      structName: IInterfaceTemplateNameI[sI]):
  IInterfaceTemplateNameI[nI] = {
    structName match {
      case InterfaceTemplateNameI(humanName) => InterfaceTemplateNameI(humanName)
    }
  }

*/
// mig: fn collapse_impl_name
pub fn collapse_impl_name() { panic!("Unimplemented: collapse_impl_name"); }
/*
  def collapseImplName(
      map: Map[Int, Int],
      name: IImplNameI[sI]):
  IImplNameI[nI] = {
    name match {
      case ImplNameI(template, templateArgs, subCitizen) => {
        ImplNameI(
          collapseImplTemplateName(map, template),
          templateArgs.map(collapseTemplata(map, _)),
          collapseCitizen(map, subCitizen))
      }
      case AnonymousSubstructImplNameI(AnonymousSubstructImplTemplateNameI(interface), templateArgs, subCitizen) => {
        AnonymousSubstructImplNameI(
          AnonymousSubstructImplTemplateNameI(collapseInterfaceTemplateName(interface)),
          templateArgs.map(collapseTemplata(map, _)),
          collapseCitizen(map, subCitizen))
      }
      case ImplBoundNameI(ImplBoundTemplateNameI(codeLocationS), templateArgs) => {
        ImplBoundNameI(
          ImplBoundTemplateNameI(codeLocationS),
          templateArgs.map(collapseTemplata(map, _)))
      }
    }
  }

*/
// mig: fn collapse_impl_id
pub fn collapse_impl_id() { panic!("Unimplemented: collapse_impl_id"); }
/*
  def collapseImplId(
    map: Map[Int, Int],
    structId: IdI[sI, IImplNameI[sI]]):
  IdI[nI, IImplNameI[nI]] = {
    collapseId[IImplNameI[sI], IImplNameI[nI]](
      map,
      structId,
      collapseImplName(map, _))
  }

*/
// mig: fn collapse_impl_template_id
pub fn collapse_impl_template_id() { panic!("Unimplemented: collapse_impl_template_id"); }
/*
  def collapseImplTemplateId(
      map: Map[Int, Int],
      structId: IdI[sI, IImplTemplateNameI[sI]]):
  IdI[nI, IImplTemplateNameI[nI]] = {
    collapseId[IImplTemplateNameI[sI], IImplTemplateNameI[nI]](
      map,
      structId,
      collapseImplTemplateName(map, _))
  }

*/
// mig: fn collapse_impl_template_name
pub fn collapse_impl_template_name() { panic!("Unimplemented: collapse_impl_template_name"); }
/*
  def collapseImplTemplateName(
    map: Map[Int, Int],
    structName: IImplTemplateNameI[sI]):
  IImplTemplateNameI[nI] = {
    structName match {
      case ImplTemplateNameI(humanName) => ImplTemplateNameI(humanName)
      case AnonymousSubstructImplTemplateNameI(interface) => AnonymousSubstructImplTemplateNameI(collapseInterfaceTemplateName(interface))
    }
  }

}
*/