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
use crate::instantiating::ast::names::{IdI, INameI, INameValI, IFunctionNameI, FunctionNameIX, FunctionTemplateNameI, ExportNameI, ExportTemplateNameI, ExternNameI, ExternTemplateNameI, ExternFunctionNameI, IVarNameI, CodeVarNameI, IStructNameI, IInterfaceNameI};
use crate::instantiating::ast::templata::ITemplataI;
use crate::instantiating::ast::types::{sI, cI};
use crate::instantiating::ast::templata::RegionTemplataI;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::instantiating::region_counter;
use crate::instantiating::ast::ast::{PrototypeI, PrototypeIValI};
use crate::instantiating::ast::types::{CoordI, KindIT, VoidIT, NeverIT, IntIT, BoolIT, FloatIT, StrIT};
use std::collections::HashMap;

// mig: fn collapse_prototype
pub fn collapse_prototype<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, prototype: &PrototypeI<'s, 'i, sI>) -> PrototypeI<'s, 'i, cI>
where 's: 'i {
    let PrototypeI { id, return_type, .. } = *prototype;
    *interner.intern_prototype_ci(PrototypeIValI {
        id: collapse_function_id(interner, &id),
        return_type: collapse_coord(interner, &return_type),
    })
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
pub fn collapse_id<'s, 'i>(
    interner: &InstantiatingInterner<'s, 'i>,
    id_i: &IdI<'s, 'i, sI>,
    func: impl Fn(&INameI<'s, 'i, sI>) -> INameI<'s, 'i, cI>,
) -> IdI<'s, 'i, cI>
where 's: 'i {
    let init_steps_c = id_i.init_steps.iter().map(|x| collapse_name(interner, x)).collect::<Vec<_>>();
    IdI {
        package_coord: id_i.package_coord,
        init_steps: interner.alloc_slice_from_vec(init_steps_c),
        local_name: func(&id_i.local_name),
    }
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
pub fn collapse_function_id<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, id: &IdI<'s, 'i, sI>) -> IdI<'s, 'i, cI>
where 's: 'i {
    collapse_id(interner, id, |x| INameI::from(collapse_function_name(interner, &IFunctionNameI::try_from(*x).unwrap())))
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
pub fn collapse_function_name<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, name: &IFunctionNameI<'s, 'i, sI>) -> IFunctionNameI<'s, 'i, cI>
where 's: 'i {
    match *name {
        IFunctionNameI::Function(n) => {
            let map = region_counter::count_function_name_map(name);
            let FunctionNameIX { template: FunctionTemplateNameI { human_name, code_location, .. }, template_args, parameters, .. } = *n;
            let template_c = FunctionTemplateNameI { _marker: std::marker::PhantomData, human_name, code_location };
            let template_args_c = interner.alloc_slice_from_vec(template_args.iter().map(|template_arg| collapse_templata(interner, &map, template_arg)).collect::<Vec<_>>());
            let params_c = interner.alloc_slice_from_vec(parameters.iter().map(|param| collapse_coord(interner, param)).collect::<Vec<_>>());
            IFunctionNameI::Function(interner.intern_function_name_x_ci(FunctionNameIX { template: template_c, template_args: template_args_c, parameters: params_c }))
        }
        IFunctionNameI::ExternFunction(n) => {
            let map = region_counter::count_function_name_map(name);
            let ExternFunctionNameI { human_name, template_args, parameters } = *n;
            let params_c = interner.alloc_slice_from_vec(parameters.iter().map(|param| collapse_coord(interner, param)).collect::<Vec<_>>());
            let template_args_c = interner.alloc_slice_from_vec(template_args.iter().map(|template_arg| collapse_templata(interner, &map, template_arg)).collect::<Vec<_>>());
            IFunctionNameI::ExternFunction(interner.intern_extern_function_name_ci(ExternFunctionNameI { human_name, template_args: template_args_c, parameters: params_c }))
        }
        IFunctionNameI::LambdaCallFunction(n) => {
            let map = region_counter::count_function_name_map(name);
            let crate::instantiating::ast::names::LambdaCallFunctionNameI { template: crate::instantiating::ast::names::LambdaCallFunctionTemplateNameI { code_location, param_types: params_tt, .. }, template_args, parameters } = *n;
            let params_tt_c = interner.alloc_slice_from_vec(params_tt.iter().map(|p| collapse_coord(interner, p)).collect::<Vec<_>>());
            let template_c = *interner.intern_lambda_call_function_template_name_ci(crate::instantiating::ast::names::LambdaCallFunctionTemplateNameI { _marker: std::marker::PhantomData, code_location, param_types: params_tt_c });
            let template_args_c = interner.alloc_slice_from_vec(template_args.iter().map(|t| collapse_templata(interner, &map, t)).collect::<Vec<_>>());
            let params_c = interner.alloc_slice_from_vec(parameters.iter().map(|p| collapse_coord(interner, p)).collect::<Vec<_>>());
            crate::instantiating::ast::names::IFunctionNameI::LambdaCallFunction(interner.intern_lambda_call_function_name_ci(crate::instantiating::ast::names::LambdaCallFunctionNameI { template: template_c, template_args: template_args_c, parameters: params_c }))
        }
        IFunctionNameI::AnonymousSubstructConstructor(_) => panic!("Unimplemented: collapse_function_name AnonymousSubstructConstructor"),
        IFunctionNameI::ForwarderFunction(_) => panic!("Unimplemented: collapse_function_name ForwarderFunction"),
        _ => panic!("Unimplemented: collapse_function_name other"),
    }
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
        val templateC = LambdaCallFunctionTemplateNameI[cI](codeLocation, paramsTT.map(collapseCoord(_)))
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
pub fn collapse_var_name<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, name: &IVarNameI<'s, 'i, sI>) -> IVarNameI<'s, 'i, cI>
where 's: 'i {
    match name {
        IVarNameI::TypingPassBlockResultVar(crate::instantiating::ast::names::TypingPassBlockResultVarNameI { life, .. }) => {
            IVarNameI::TypingPassBlockResultVar(interner.intern_typing_pass_block_result_var_name_ci(crate::instantiating::ast::names::TypingPassBlockResultVarNameI {
                _marker: std::marker::PhantomData,
                life: *life,
            }))
        }
        IVarNameI::CodeVar(x) => IVarNameI::CodeVar(interner.intern_code_var_name_ci(CodeVarNameI { _marker: std::marker::PhantomData, name: x.name })),
        IVarNameI::TypingPassTemporaryVar(_) => panic!("Unimplemented: collapse_var_name TypingPassTemporaryVar"),
        IVarNameI::TypingPassFunctionResultVar(_) => IVarNameI::TypingPassFunctionResultVar(interner.intern_typing_pass_function_result_var_name_ci(crate::instantiating::ast::names::TypingPassFunctionResultVarNameI(std::marker::PhantomData))),
        IVarNameI::ClosureParam(crate::instantiating::ast::names::ClosureParamNameI { code_location, .. }) => IVarNameI::ClosureParam(interner.intern_closure_param_name_ci(crate::instantiating::ast::names::ClosureParamNameI { _marker: std::marker::PhantomData, code_location: *code_location })),
        IVarNameI::MagicParam(crate::instantiating::ast::names::MagicParamNameI { code_location_2, .. }) => IVarNameI::MagicParam(interner.intern_magic_param_name_ci(crate::instantiating::ast::names::MagicParamNameI { _marker: std::marker::PhantomData, code_location_2: *code_location_2 })),
        IVarNameI::Iterable(_) => panic!("Unimplemented: collapse_var_name Iterable"),
        IVarNameI::ConstructingMember(_) => panic!("Unimplemented: collapse_var_name ConstructingMember"),
        IVarNameI::Iterator(_) => panic!("Unimplemented: collapse_var_name Iterator"),
        IVarNameI::IterationOption(_) => panic!("Unimplemented: collapse_var_name IterationOption"),
        IVarNameI::Self_(_) => panic!("Unimplemented: collapse_var_name SelfName"),
        _ => panic!("Unimplemented: collapse_var_name other"),
    }
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
pub fn collapse_name<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, name: &INameI<'s, 'i, sI>) -> INameI<'s, 'i, cI>
where 's: 'i {
    match name {
        INameI::OverrideDispatcher(_) | INameI::ExternFunction(_) | INameI::FunctionNameIX(_)
        | INameI::ForwarderFunction(_) | INameI::FunctionBound(_) | INameI::LambdaCallFunction(_)
        | INameI::AnonymousSubstructConstructor(_) => {
            let n: crate::instantiating::ast::names::IFunctionNameI<'s, 'i, sI> = (*name).try_into().unwrap();
            collapse_function_name(interner, &n).into()
        }
        INameI::LambdaCitizenTemplate(crate::instantiating::ast::names::LambdaCitizenTemplateNameI { code_location, .. }) => {
            INameI::LambdaCitizenTemplate(interner.intern_lambda_citizen_template_name_ci(crate::instantiating::ast::names::LambdaCitizenTemplateNameI { _marker: std::marker::PhantomData, code_location: *code_location }))
        }
        INameI::StructTemplate(stn) => {
            let crate::instantiating::ast::names::StructTemplateNameI { human_name, .. } = **stn;
            INameI::StructTemplate(interner.intern_struct_template_name_ci(crate::instantiating::ast::names::StructTemplateNameI { _marker: std::marker::PhantomData, human_name }))
        }
        other => panic!("Unimplemented: collapse_name {:?}", std::mem::discriminant(other)),
    }
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
pub fn collapse_coord_templata<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, map: &HashMap<i32, i32>, templata: crate::instantiating::ast::templata::CoordTemplataI<'s, 'i, sI>) -> crate::instantiating::ast::templata::CoordTemplataI<'s, 'i, cI> where 's: 'i {
    let crate::instantiating::ast::templata::CoordTemplataI { region, coord } = templata;
    crate::instantiating::ast::templata::CoordTemplataI {
        region: collapse_region_templata(map, region),
        coord: collapse_coord(interner, &coord),
    }
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
pub fn collapse_templata<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, map: &HashMap<i32, i32>, templata: &ITemplataI<'s, 'i, sI>) -> ITemplataI<'s, 'i, cI>
where 's: 'i {
    use crate::instantiating::ast::templata::{ITemplataI, MutabilityTemplataI, IntegerTemplataI, VariabilityTemplataI};
    match templata {
        ITemplataI::Coord(c) => ITemplataI::Coord(collapse_coord_templata(interner, map, *c)),
        ITemplataI::Kind(k) => ITemplataI::Kind(crate::instantiating::ast::templata::KindTemplataI { kind: collapse_kind(interner, &k.kind) }),
        ITemplataI::Region(r) => ITemplataI::Region(collapse_region_templata(map, *r)),
        ITemplataI::Mutability(m) => ITemplataI::Mutability(MutabilityTemplataI { mutability: m.mutability, _marker: std::marker::PhantomData }),
        ITemplataI::Integer(i) => ITemplataI::Integer(IntegerTemplataI { value: i.value, _marker: std::marker::PhantomData }),
        ITemplataI::Variability(v) => ITemplataI::Variability(VariabilityTemplataI { variability: v.variability, _marker: std::marker::PhantomData }),
        _ => panic!("collapse_templata: unimplemented variant"),
    }
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
pub fn collapse_region_templata<'s, 'i>(map: &HashMap<i32, i32>, templata: RegionTemplataI<'s, 'i, sI>) -> RegionTemplataI<'s, 'i, cI>
where 's: 'i {
    let RegionTemplataI { pure_height: old_pure_height, .. } = templata;
    RegionTemplataI { pure_height: *map.get(&old_pure_height).unwrap(), _marker: std::marker::PhantomData }
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
pub fn collapse_coord<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, coord: &CoordI<'s, 'i, sI>) -> CoordI<'s, 'i, cI>
where 's: 'i {
    let CoordI { ownership, kind } = *coord;
    CoordI { ownership, kind: collapse_kind(interner, &kind) }
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
pub fn collapse_kind<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, kind: &KindIT<'s, 'i, sI>) -> KindIT<'s, 'i, cI>
where 's: 'i {
    match kind {
        KindIT::NeverIT(never) => KindIT::NeverIT(NeverIT { from_break: never.from_break, _marker: std::marker::PhantomData }),
        KindIT::VoidIT(_) => KindIT::VoidIT(VoidIT { _marker: std::marker::PhantomData }),
        KindIT::IntIT(int) => KindIT::IntIT(IntIT { bits: int.bits, _marker: std::marker::PhantomData }),
        KindIT::BoolIT(_) => KindIT::BoolIT(BoolIT { _marker: std::marker::PhantomData }),
        KindIT::FloatIT(_) => KindIT::FloatIT(FloatIT { _marker: std::marker::PhantomData }),
        KindIT::StrIT(_) => KindIT::StrIT(StrIT { _marker: std::marker::PhantomData }),
        KindIT::StructIT(s) => KindIT::StructIT(interner.intern_struct_it_ci(crate::instantiating::ast::types::StructITValI { id: collapse_struct_id(interner, &s.id) })),
        KindIT::InterfaceIT(i) => KindIT::InterfaceIT(interner.intern_interface_it_ci(crate::instantiating::ast::types::InterfaceITValI { id: collapse_interface_id(interner, &i.id) })),
        KindIT::StaticSizedArrayIT(ssa) => KindIT::StaticSizedArrayIT(interner.alloc(collapse_static_sized_array(interner, ssa))),
        KindIT::RuntimeSizedArrayIT(_) => panic!("Unimplemented: collapse_kind RuntimeSizedArray"),
    }
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
pub fn collapse_static_sized_array<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, ssa: &crate::instantiating::ast::types::StaticSizedArrayIT<'s, 'i, sI>) -> crate::instantiating::ast::types::StaticSizedArrayIT<'s, 'i, cI>
where 's: 'i {
    let ssa_id = ssa.name;
    let map = crate::instantiating::region_counter::count_static_sized_array_map(ssa);
    let collapsed_id = collapse_id(interner, &ssa_id, |local_name| {
        match local_name {
            INameI::StaticSizedArray(n) => {
                let crate::instantiating::ast::names::StaticSizedArrayNameI { template: _, size, variability, arr } = **n;
                let crate::instantiating::ast::names::RawArrayNameI { mutability, element_type, self_region } = arr;
                INameI::StaticSizedArray(interner.alloc(crate::instantiating::ast::names::StaticSizedArrayNameI {
                    template: crate::instantiating::ast::names::StaticSizedArrayTemplateNameI(std::marker::PhantomData),
                    size,
                    variability,
                    arr: crate::instantiating::ast::names::RawArrayNameI {
                        mutability,
                        element_type: crate::instantiating::ast::templata::expect_coord_templata(collapse_templata(interner, &map, &ITemplataI::Coord(element_type))),
                        self_region: collapse_region_templata(&map, self_region),
                    },
                }))
            }
            _ => panic!("collapse_static_sized_array: non-StaticSizedArrayName local name"),
        }
    });
    *interner.intern_static_sized_array_it_ci(crate::instantiating::ast::types::StaticSizedArrayITValI { name: collapsed_id })
}
/*
Guardian: temp-disable: SPDMX — Per SPDMX Exception S, _map suffix disambiguates from the unit-returning count_static_sized_array(counter, ssa) at line 487 — same pattern as count_struct_id/count_struct_id_map sibling pair. Both Rust fns mirror the same-named Scala method (Scala has overloading by counter-presence). — FrontendRust/guardian-logs/request-179-1780503381221/hook-179/collapse_static_sized_array--431.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
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
pub fn collapse_interface_id<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, interface_id: &IdI<'s, 'i, sI>) -> IdI<'s, 'i, cI> where 's: 'i {
    collapse_id(interner, interface_id, |x| {
        match x {
            INameI::InterfaceName(i) => INameI::InterfaceName(match collapse_interface_name(interner, &IInterfaceNameI::Interface(i)) {
                IInterfaceNameI::Interface(r) => r,
            }),
            _ => panic!("collapse_interface_id: non-InterfaceName local name"),
        }
    })
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
pub fn collapse_struct_id<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, struct_id: &IdI<'s, 'i, sI>) -> IdI<'s, 'i, cI> where 's: 'i {
    collapse_id(interner, struct_id, |x| {
        let narrowed: IStructNameI<'s, 'i, sI> = (*x).try_into().unwrap();
        collapse_struct_name(interner, &narrowed).into()
    })
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
pub fn collapse_struct_name<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, struct_name: &IStructNameI<'s, 'i, sI>) -> IStructNameI<'s, 'i, cI> where 's: 'i {
    match struct_name {
        IStructNameI::Struct(crate::instantiating::ast::names::StructNameI { template, template_args }) => {
            let map = crate::instantiating::region_counter::count_citizen_name_map(&(*struct_name).into());
            let template_c = collapse_struct_template_name(interner, template);
            let template_args_c: Vec<ITemplataI<'s, 'i, cI>> = template_args.iter().map(|t| collapse_templata(interner, &map, t)).collect();
            IStructNameI::Struct(interner.intern_struct_name_ci(crate::instantiating::ast::names::StructNameI {
                template: template_c,
                template_args: interner.bump().alloc_slice_fill_iter(template_args_c.into_iter()),
            }))
        }
        IStructNameI::LambdaCitizen(crate::instantiating::ast::names::LambdaCitizenNameI { template: crate::instantiating::ast::names::LambdaCitizenTemplateNameI { code_location, _marker: _ } }) => {
            IStructNameI::LambdaCitizen(interner.intern_lambda_citizen_name_ci(crate::instantiating::ast::names::LambdaCitizenNameI {
                template: *interner.intern_lambda_citizen_template_name_ci(crate::instantiating::ast::names::LambdaCitizenTemplateNameI { _marker: std::marker::PhantomData, code_location: *code_location }),
            }))
        }
        IStructNameI::AnonymousSubstruct(_) => panic!("collapse_struct_name: AnonymousSubstruct branch"),
    }
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
pub fn collapse_impl_name<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, name: &crate::instantiating::ast::names::IImplNameI<'s, 'i, sI>) -> crate::instantiating::ast::names::IImplNameI<'s, 'i, cI>
where 's: 'i {
    use crate::instantiating::ast::names::{IImplNameI, ImplNameI};
    match name {
        IImplNameI::Impl(ImplNameI { template, template_args, sub_citizen }) => {
            let map = crate::instantiating::region_counter::count_impl_name_map(name);
            let template_c = collapse_impl_template_name(interner, template);
            let template_args_c: Vec<ITemplataI<'s, 'i, cI>> = template_args.iter().map(|t| collapse_templata(interner, &map, t)).collect();
            let sub_citizen_c = collapse_citizen(interner, sub_citizen);
            IImplNameI::Impl(interner.intern_impl_name_ci(ImplNameI {
                template: template_c,
                template_args: interner.bump().alloc_slice_fill_iter(template_args_c.into_iter()),
                sub_citizen: sub_citizen_c,
            }))
        }
        IImplNameI::AnonymousSubstructImpl(_) => panic!("collapse_impl_name: AnonymousSubstructImpl branch"),
        IImplNameI::ImplBound(_) => panic!("collapse_impl_name: ImplBound branch"),
    }
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
pub fn collapse_interface_name<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, interface_name: &IInterfaceNameI<'s, 'i, sI>) -> IInterfaceNameI<'s, 'i, cI> where 's: 'i {
    use crate::instantiating::ast::names::{InterfaceNameI, IInterfaceTemplateNameI, InterfaceTemplateNameI};
    match interface_name {
        IInterfaceNameI::Interface(InterfaceNameI { template: IInterfaceTemplateNameI::InterfaceTemplate(InterfaceTemplateNameI { human_namee, .. }), template_args }) => {
            let map = crate::instantiating::region_counter::count_citizen_name_map(&(*interface_name).into());
            let template_args_c: Vec<ITemplataI<'s, 'i, cI>> = template_args.iter().map(|t| collapse_templata(interner, &map, t)).collect();
            IInterfaceNameI::Interface(interner.intern_interface_name_ci(InterfaceNameI {
                template: IInterfaceTemplateNameI::InterfaceTemplate(interner.intern_interface_template_name_ci(InterfaceTemplateNameI { _marker: std::marker::PhantomData, human_namee: *human_namee })),
                template_args: interner.bump().alloc_slice_fill_iter(template_args_c.into_iter()),
            }))
        }
    }
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
pub fn collapse_export_id<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, map: HashMap<i32, i32>, export_id_s: &IdI<'s, 'i, sI>) -> IdI<'s, 'i, cI>
where 's: 'i {
    collapse_id(interner, export_id_s, |name| {
        match name {
            INameI::Export(e) => {
                interner.intern_name_ci(INameValI::Export(ExportNameI {
                    template: ExportTemplateNameI { _marker: std::marker::PhantomData, code_loc: e.template.code_loc },
                    region: collapse_region_templata(&map, e.region),
                }))
            }
            _ => panic!("Unimplemented: collapse_export_id closure"),
        }
    })
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
pub fn collapse_extern_id<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, map: HashMap<i32, i32>, extern_id_s: &IdI<'s, 'i, sI>) -> IdI<'s, 'i, cI>
where 's: 'i {
    collapse_id(interner, extern_id_s, |name| {
        match name {
            INameI::Extern(e) => {
                interner.intern_name_ci(INameValI::Extern(ExternNameI {
                    template: ExternTemplateNameI { _marker: std::marker::PhantomData, code_loc: e.template.code_loc },
                    region: collapse_region_templata(&map, e.region),
                }))
            }
            _ => panic!("Unimplemented: collapse_extern_id closure"),
        }
    })
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
pub fn collapse_struct_template_name<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, struct_name: &crate::instantiating::ast::names::IStructTemplateNameI<'s, 'i, sI>) -> crate::instantiating::ast::names::IStructTemplateNameI<'s, 'i, cI> where 's: 'i {
    use crate::instantiating::ast::names::{IStructTemplateNameI, StructTemplateNameI};
    match struct_name {
        IStructTemplateNameI::StructTemplate(StructTemplateNameI { human_name, .. }) => IStructTemplateNameI::StructTemplate(interner.intern_struct_template_name_ci(StructTemplateNameI { _marker: std::marker::PhantomData, human_name: *human_name })),
        IStructTemplateNameI::AnonymousSubstructTemplate(_) => panic!("collapse_struct_template_name: AnonymousSubstructTemplate branch"),
        IStructTemplateNameI::LambdaCitizenTemplate(_) => panic!("collapse_struct_template_name: LambdaCitizenTemplate branch (no Scala counterpart in collapseStructTemplateName)"),
    }
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
pub fn collapse_impl_id<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, impl_id: &IdI<'s, 'i, sI>) -> IdI<'s, 'i, cI>
where 's: 'i {
    collapse_id(interner, impl_id, |x| {
        match x {
            INameI::Impl(i) => INameI::Impl(match collapse_impl_name(interner, &crate::instantiating::ast::names::IImplNameI::Impl(i)) {
                crate::instantiating::ast::names::IImplNameI::Impl(r) => r,
                _ => panic!("collapse_impl_id: collapse_impl_name returned non-Impl"),
            }),
            _ => panic!("collapse_impl_id: non-Impl local name"),
        }
    })
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
pub fn collapse_impl_template_name<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, name: &crate::instantiating::ast::names::IImplTemplateNameI<'s, 'i, sI>) -> crate::instantiating::ast::names::IImplTemplateNameI<'s, 'i, cI>
where 's: 'i {
    use crate::instantiating::ast::names::{IImplTemplateNameI, ImplTemplateNameI};
    match name {
        IImplTemplateNameI::ImplTemplate(ImplTemplateNameI { code_location_s, .. }) => {
            IImplTemplateNameI::ImplTemplate(interner.intern_impl_template_name_ci(ImplTemplateNameI { _marker: std::marker::PhantomData, code_location_s: *code_location_s }))
        }
        _ => panic!("collapse_impl_template_name: other"),
    }
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
pub fn collapse_citizen<'s, 'i>(interner: &InstantiatingInterner<'s, 'i>, citizen: &crate::instantiating::ast::types::ICitizenIT<'s, 'i, sI>) -> crate::instantiating::ast::types::ICitizenIT<'s, 'i, cI>
where 's: 'i {
    use crate::instantiating::ast::types::{ICitizenIT, StructITValI, InterfaceITValI};
    match citizen {
        ICitizenIT::StructIT(s) => ICitizenIT::StructIT(interner.intern_struct_it_ci(StructITValI { id: collapse_struct_id(interner, &s.id) })),
        ICitizenIT::InterfaceIT(i) => ICitizenIT::InterfaceIT(interner.intern_interface_it_ci(InterfaceITValI { id: collapse_interface_id(interner, &i.id) })),
    }
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