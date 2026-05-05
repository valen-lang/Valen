use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::ast::*;
use crate::typing::env::environment::*;
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::typing_interner::MustIntern;
use crate::typing::hinputs_t::{InstantiationBoundArgumentsT, InstantiationReachableBoundArgumentsT};
use crate::postparsing::names::{IRuneS, IImpreciseNameS};
use crate::postparsing::ast::{GenericParameterS, IRegionMutabilityS, LocationInDenizen};
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::rules::rules::IRulexSR;
use crate::typing::infer_compiler::include_rule_in_call_site_solve;
use crate::postparsing::rune_type_solver::IRuneTypeSolverEnv;
use crate::utils::range::RangeS;
use std::collections::HashMap;

/*
package dev.vale.typing

import dev.vale._
import dev.vale.postparsing.rules.{EqualsSR, IRulexSR, RuneUsage}
import dev.vale.postparsing._
import dev.vale.typing.env._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.highertyping._
import dev.vale.parsing.ast._
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.citizen._
import dev.vale.typing.templata.ITemplataT._
import dev.vale.typing.types._
import dev.vale.typing.templata._

import scala.annotation.tailrec
import scala.collection.immutable.{List, Map, Set}

// See SBITAFD, we need to register bounds for these new instantiations. This instructs us where
// to get those new bounds from.
*/

#[derive(Copy, Clone)]
pub enum IBoundArgumentsSource<'s, 't> {
    InheritBoundsFromTypeItself,
    UseBoundsFromContainer {
        instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
        instantiation_bound_arguments: &'t InstantiationBoundArgumentsT<'s, 't>,
    },
}
/*
sealed trait IBoundArgumentsSource
*/
/*
case object InheritBoundsFromTypeItself extends IBoundArgumentsSource
*/
/*
case class UseBoundsFromContainer(
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
  instantiationBoundArguments: InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]
) extends IBoundArgumentsSource
*/

/*
trait ITemplataCompilerDelegate {
*/
/*
  def isParent(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    subKindTT: ISubKindTT,
    superKindTT: ISuperKindTT):
  IsParentResult
*/
/*
  def resolveStruct(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    structTemplata: StructDefinitionTemplataT,
    uncoercedTemplateArgs: Vector[ITemplataT[ITemplataType]]
  ):
  IResolveOutcome[StructTT]
*/
/*
  def resolveInterface(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    // We take the entire templata (which includes environment and parents) so we can incorporate
    // their rules as needed
    interfaceTemplata: InterfaceDefinitionTemplataT,
    uncoercedTemplateArgs: Vector[ITemplataT[ITemplataType]]
  ):
  IResolveOutcome[InterfaceTT]
}

object TemplataCompiler {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_top_level_denizen_id(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let steps = id.steps();
        let is_instantiation_name = |name: &INameT<'s, 't>| -> bool {
            match name {
                INameT::Function(_) |
                INameT::LambdaCallFunction(_) |
                INameT::ForwarderFunction(_) |
                INameT::Struct(_) |
                INameT::LambdaCitizen(_) |
                INameT::Interface(_) |
                INameT::Impl(_) |
                INameT::Export(_) |
                INameT::ExternFunction(_) |
                INameT::KindPlaceholder(_) |
                INameT::AnonymousSubstructImpl(_) |
                INameT::OverrideDispatcher(_) => true,
                _ => false,
            }
        };
        let index = steps.iter().position(is_instantiation_name);
        let index = index.expect("get_top_level_denizen_id: no IInstantiationNameT found in steps");
        let last_step = steps[index];
        assert!(is_instantiation_name(&last_step), "get_top_level_denizen_id: step at index is not IInstantiationNameT");
        let init_steps_slice = self.typing_interner.alloc_slice_copy(&steps[..index]);
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: init_steps_slice,
            local_name: last_step,
        })
    }
}
/*
  def getTopLevelDenizenId(
    id: IdT[INameT],
  ): IdT[IInstantiationNameT] = {
    // That said, some things are namespaced inside templates. If we have a `struct Marine` then
    // we'll also have a func drop within its namespace; we'll have a free function instance under
    // a Marine struct template. We want to grab the instance.
    val index =
    id.steps.indexWhere({
      case x : IInstantiationNameT => true
      case _ => false
    })
    vassert(index >= 0)
    val initSteps = id.steps.slice(0, index)
    val lastStep =
      id.steps(index) match {
        case x : IInstantiationNameT => x
        case _ => vwat()
      }
    IdT(id.packageCoord, initSteps, lastStep)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_placeholder_templata_id(
        &self,
        impl_placeholder: ITemplataT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getPlaceholderTemplataId(implPlaceholder: ITemplataT[ITemplataType]): IdT[IPlaceholderNameT] = {
    implPlaceholder match {
      case PlaceholderTemplataT(n, _) => n
      case KindTemplataT(KindPlaceholderT(n)) => n
      case CoordTemplataT(CoordT(_, _, KindPlaceholderT(n))) => n
      case other => vwat(other)
    }
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn assemble_predict_rules(
        &self,
        generic_parameters: &'s [&'s GenericParameterS<'s>],
        num_explicit_template_args: i32,
    ) -> Vec<IRulexSR<'s>> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  // See SFWPRL
  def assemblePredictRules(genericParameters: Vector[GenericParameterS], numExplicitTemplateArgs: Int): Vector[IRulexSR] = {
    genericParameters.zipWithIndex.flatMap({ case (genericParam, index) =>
      if (index >= numExplicitTemplateArgs) {
        genericParam.default match {
          case Some(x) => {
            x.rules :+
              EqualsSR(genericParam.range, genericParam.rune, RuneUsage(genericParam.range, x.resultRune))
          }
          case None => Vector()
        }
      } else {
        Vector()
      }
    })
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn assemble_call_site_rules(
        &self,
        rules: &'s [IRulexSR<'s>],
        generic_parameters: &'s [&'s GenericParameterS<'s>],
        num_explicit_template_args: i32,
    ) -> Vec<&'s IRulexSR<'s>> {
        let mut result: Vec<&'s IRulexSR<'s>> =
            rules.iter().filter(|r| include_rule_in_call_site_solve(r)).collect();
        for (index, generic_param) in generic_parameters.iter().enumerate() {
            if index as i32 >= num_explicit_template_args {
                match &generic_param.default {
                    Some(x) => {
                        panic!("implement: assembleCallSiteRules default rules");
                    }
                    None => {}
                }
            }
        }
        result
    }
}
/*
  def assembleCallSiteRules(rules: Vector[IRulexSR], genericParameters: Vector[GenericParameterS], numExplicitTemplateArgs: Int): Vector[IRulexSR] = {
    rules.filter(InferCompiler.includeRuleInCallSiteSolve) ++
      (genericParameters.zipWithIndex.flatMap({ case (genericParam, index) =>
        if (index >= numExplicitTemplateArgs) {
          genericParam.default match {
            case Some(x) => x.rules
            case None => Vector()
          }
        } else {
          Vector()
        }
      }))
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_function_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getFunctionTemplate(id: IdT[IFunctionNameT]): IdT[IFunctionTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_citizen_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let local_name = match id.local_name {
            INameT::Struct(s) => {
                match s.template {
                    IStructTemplateNameT::StructTemplate(tmpl) => INameT::StructTemplate(tmpl),
                    IStructTemplateNameT::LambdaCitizenTemplate(tmpl) => INameT::LambdaCitizenTemplate(tmpl),
                    IStructTemplateNameT::AnonymousSubstructTemplate(tmpl) => INameT::AnonymousSubstructTemplate(tmpl),
                }
            }
            INameT::LambdaCitizen(lc) => INameT::LambdaCitizenTemplate(lc.template),
            INameT::Interface(i) => INameT::InterfaceTemplate(i.template),
            _ => panic!("get_citizen_template called with non-citizen name: {:?}", id.local_name),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name,
        })
    }
}
/*
  def getCitizenTemplate(id: IdT[ICitizenNameT]): IdT[ICitizenTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_name_template(
        &self,
        name: INameT<'s, 't>,
    ) -> INameT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getNameTemplate(name: INameT): INameT = {
    name match {
      case x : IInstantiationNameT => x.template
      case _ => name
    }
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_super_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getSuperTemplate(id: IdT[INameT]): IdT[INameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      getNameTemplate(last))
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_root_super_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  // Removes lambda citizens / lambda calls from the end, so we get the root function.
  def getRootSuperTemplate(interner: Interner, id: IdT[INameT]): IdT[INameT] = {
    @tailrec
    def removeTrailingLambdas(tentativeId: IdT[INameT]): IdT[INameT] = {
      val containsLambda =
        tentativeId.steps.exists {
          case LambdaCitizenTemplateNameT(_) => true
          case LambdaCallFunctionTemplateNameT(_, _) => true
          case OverrideDispatcherCaseNameT(_) => true
          case _ => false
        }
      if (containsLambda) { // We do this logic because lambdas sometimes have FunctionNameT after them.
        // Recurse
        removeTrailingLambdas(tentativeId.initId(interner))
      } else {
        tentativeId
      }
    }
    removeTrailingLambdas(getSuperTemplate(id))
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_template(
        &self,
        id: IdT<'s, 't>,
    ) -> &'t IdT<'s, 't> {
        // val IdT(packageCoord, initSteps, last) = id
        // IdT(packageCoord, initSteps, last.template)
        let template_name = match id.local_name {
            INameT::StaticSizedArray(ssa) => INameT::StaticSizedArrayTemplate(ssa.template),
            INameT::LambdaCitizen(lc) => INameT::LambdaCitizenTemplate(lc.template),
            INameT::Struct(s) => {
                match s.template {
                    IStructTemplateNameT::StructTemplate(tmpl) => INameT::StructTemplate(tmpl),
                    IStructTemplateNameT::LambdaCitizenTemplate(tmpl) => INameT::LambdaCitizenTemplate(tmpl),
                    IStructTemplateNameT::AnonymousSubstructTemplate(tmpl) => INameT::AnonymousSubstructTemplate(tmpl),
                }
            }
            INameT::Interface(i) => INameT::InterfaceTemplate(i.template),
            INameT::Function(f) => INameT::FunctionTemplate(f.template),
            _ => panic!("get_template: not yet implemented for {:?}", id.local_name),
        };
        self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: template_name,
        })
    }
}
/*
  def getTemplate(id: IdT[IInstantiationNameT]): IdT[ITemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_sub_kind_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getSubKindTemplate(id: IdT[ISubKindNameT]): IdT[ISubKindTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_super_kind_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getSuperKindTemplate(id: IdT[ISuperKindNameT]): IdT[ISuperKindTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_struct_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let local_name = match id.local_name {
            INameT::Struct(s) => {
                match s.template {
                    IStructTemplateNameT::StructTemplate(tmpl) => INameT::StructTemplate(tmpl),
                    IStructTemplateNameT::LambdaCitizenTemplate(tmpl) => INameT::LambdaCitizenTemplate(tmpl),
                    IStructTemplateNameT::AnonymousSubstructTemplate(tmpl) => INameT::AnonymousSubstructTemplate(tmpl),
                }
            }
            INameT::LambdaCitizen(lc) => INameT::LambdaCitizenTemplate(lc.template),
            _ => panic!("get_struct_template called with non-struct name: {:?}", id.local_name),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name,
        })
    }
}
/*
Guardian: temp-disable: SPDMX — In Scala, LambdaCitizenNameT extends IStructNameT, so getStructTemplate's `last.template` handles it polymorphically. In Rust, IStructNameT is flattened into INameT enum, so we must explicitly match LambdaCitizen — this is the standard SSTREX pattern, not novel logic. — FrontendRust/guardian-logs/request-1332-1777936399967/hook-1332/get_struct_template--405.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def getStructTemplate(id: IdT[IStructNameT]): IdT[IStructTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_interface_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let local_name = match id.local_name {
            INameT::Interface(i) => INameT::InterfaceTemplate(i.template),
            _ => panic!("get_interface_template called with non-interface name"),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name,
        })
    }
}
/*
  def getInterfaceTemplate(id: IdT[IInterfaceNameT]): IdT[IInterfaceTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_export_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getExportTemplate(id: IdT[ExportNameT]): IdT[ExportTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_extern_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getExternTemplate(id: IdT[ExternNameT]): IdT[ExternTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps, //.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_impl_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getImplTemplate(id: IdT[IImplNameT]): IdT[IImplTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_placeholder_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        // val IdT(packageCoord, initSteps, last) = id
        // IdT(packageCoord, initSteps, last.template)
        let template_name = match id.local_name {
            INameT::KindPlaceholder(kp) => INameT::KindPlaceholderTemplate(kp.template),
            _ => panic!("get_placeholder_template: unexpected local_name"),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: template_name,
        })
    }
}
/*
  def getPlaceholderTemplate(id: IdT[KindPlaceholderNameT]): IdT[KindPlaceholderTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn assemble_rune_to_function_bound(
        &self,
        templatas: &'t TemplatasStoreT<'s, 't>,
    ) -> HashMap<IRuneS<'s>, &'t PrototypeT<'s, 't>> {
        let mut result = HashMap::new();
        for (name, entry) in templatas.name_to_entry.iter() {
            match (name, entry) {
                (INameT::Rune(rune_name), IEnvEntryT::Templata(ITemplataT::Prototype(proto_templata))) => {
                    match &proto_templata.prototype.id.local_name {
                        INameT::FunctionBound(_) => {
                            panic!("implement: assemble_rune_to_function_bound — FunctionBoundNameT match");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        result
    }
}
/*
  def assembleRuneToFunctionBound(templatas: TemplatasStore): Map[IRuneS, PrototypeT[FunctionBoundNameT]] = {
    templatas.entriesByNameT.toIterable.flatMap({
      case (RuneNameT(rune), TemplataEnvEntry(PrototypeTemplataT(PrototypeT(IdT(packageCoord, initSteps, name @ FunctionBoundNameT(_, _, _)), returnType)))) => {
        Some(rune -> PrototypeT(IdT(packageCoord, initSteps, name), returnType))
      }
      case _ => None
    }).toMap
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn assemble_rune_to_impl_bound(
        &self,
        templatas: &'t TemplatasStoreT<'s, 't>,
    ) -> HashMap<IRuneS<'s>, IdT<'s, 't>> {
        let mut result = HashMap::new();
        for (name, entry) in templatas.name_to_entry.iter() {
            match (name, entry) {
                (INameT::Rune(rune_name), IEnvEntryT::Templata(ITemplataT::Isa(isa))) => {
                    match &isa.impl_name.local_name {
                        INameT::ImplBound(_) => {
                            panic!("implement: assemble_rune_to_impl_bound — ImplBoundNameT match");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        result
    }
}
/*
  def assembleRuneToImplBound(templatas: TemplatasStore): Map[IRuneS, IdT[ImplBoundNameT]] = {
    templatas.entriesByNameT.toIterable.flatMap({
      case (RuneNameT(rune), TemplataEnvEntry(IsaTemplataT(_, IdT(packageCoord, initSteps, name @ ImplBoundNameT(_, _)), _, _))) => {
        Some(rune -> IdT(packageCoord, initSteps, name))
      }
      case _ => None
    }).toMap
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_coord(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        coord: CoordT<'s, 't>,
    ) -> CoordT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def substituteTemplatasInCoord(
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],

    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    coord: CoordT):
  CoordT = {
    val CoordT(ownership, originalRegion, kind) = coord
    val resultRegion = vregionmut(originalRegion)
    substituteTemplatasInKind(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, kind) match {
      case KindTemplataT(kind) => CoordT(ownership, resultRegion, kind)
      case CoordTemplataT(CoordT(innerOwnership, innerRegion, kind)) => {
        val resultOwnership =
          (ownership, innerOwnership) match {
            case (ShareT, _) => ShareT
            case (_, ShareT) => ShareT
            case (OwnT, OwnT) => OwnT
            case (OwnT, BorrowT) => BorrowT
            case (BorrowT, OwnT) => BorrowT
            case (BorrowT, BorrowT) => BorrowT
            case _ => vimpl()
          }
        CoordT(resultOwnership, resultRegion, kind)
      }
    }

  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_kind(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        kind: KindT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  // This returns an ITemplata because...
  // Let's say we have a parameter that's a Coord(own, $_0).
  // $_0 is a PlaceholderT(0), which means it's a standing for whatever the first template arg is.
  // Let's say the first template arg is a CoordTemplata containing &Ship.
  // We're in the weird position of turning a PlaceholderT kind into a &Ship coord!
  // That's why we have to return an ITemplata, because it could be a coord or a kind.
  def substituteTemplatasInKind(
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],

    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    kind: KindT):
  ITemplataT[ITemplataType] = {
    kind match {
      case IntT(bits) => KindTemplataT(kind)
      case BoolT() => KindTemplataT(kind)
      case StrT() => KindTemplataT(kind)
      case FloatT() => KindTemplataT(kind)
      case VoidT() => KindTemplataT(kind)
      case NeverT(_) => KindTemplataT(kind)
      case RuntimeSizedArrayTT(IdT(
      packageCoord,
      initSteps,
      RuntimeSizedArrayNameT(template, RawArrayNameT(mutability, elementType, region)))) => {
        KindTemplataT(
          interner.intern(RuntimeSizedArrayTT(
            IdT(
              packageCoord,
              initSteps,
              interner.intern(RuntimeSizedArrayNameT(
                template,
                interner.intern(RawArrayNameT(
                  expectMutability(substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, mutability)),
                  substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, elementType),
                  RegionT()))))))))
      }
      case StaticSizedArrayTT(IdT(packageCoord, initSteps, StaticSizedArrayNameT(template, size, variability, RawArrayNameT(mutability, elementType, region)))) => {
        KindTemplataT(
          interner.intern(StaticSizedArrayTT(
            IdT(
              packageCoord,
              initSteps,
              interner.intern(StaticSizedArrayNameT(
                template,
                expectInteger(substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, size)),
                expectVariability(substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, variability)),
                interner.intern(RawArrayNameT(
                  expectMutability(substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, mutability)),
                  substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, elementType),
                  RegionT()))))))))
      }
      case p @ KindPlaceholderT(id @ IdT(_, _, KindPlaceholderNameT(KindPlaceholderTemplateNameT(index, rune)))) => {
        if (id.initId(interner) == needleTemplateName) {
          newSubstitutingTemplatas(index)
        } else {
          KindTemplataT(kind)
        }
      }
      case s @ StructTT(_) => KindTemplataT(substituteTemplatasInStruct(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, s))
      case s @ InterfaceTT(_) => KindTemplataT(substituteTemplatasInInterface(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, s))
    }
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_struct(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        struct_tt: &'t StructTT<'s, 't>,
    ) -> &'t StructTT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def substituteTemplatasInStruct(
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],
    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    structTT: StructTT):
  StructTT = {
    val StructTT(IdT(packageCoord, initSteps, last)) = structTT
    val newStruct =
      interner.intern(
        StructTT(
          IdT(
            packageCoord,
            initSteps,
            last match {
              case AnonymousSubstructNameT(template, templateArgs) => {
                interner.intern(AnonymousSubstructNameT(
                  template,
                  templateArgs.map((templata: ITemplataT[ITemplataType]) => substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, templata))))
              }
              case StructNameT(template, templateArgs) => {
                interner.intern(StructNameT(
                  template,
                  templateArgs.map((templata: ITemplataT[ITemplataType]) => substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, templata))))
              }
              case LambdaCitizenNameT(template) => {
                interner.intern(LambdaCitizenNameT(template))
              }
            })))
    // See SBITAFD, we need to register bounds for these new instantiations.
    val instantiationBoundArgs =
      vassertSome(coutputs.getInstantiationBounds(structTT.id))
    coutputs.addInstantiationBounds(
      sanityCheck,
      interner,
      originalCallingDenizenId,
      newStruct.id,
      translateInstantiationBounds(
        coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, instantiationBoundArgs))
    newStruct
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_instantiation_bounds(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        instantiation_bound_args: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> InstantiationBoundArgumentsT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  private def translateInstantiationBounds(
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],

    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    instantiationBoundArgs: InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]):
  InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT] = {
    boundArgumentsSource match {
      case InheritBoundsFromTypeItself => {
        val x =
          substituteTemplatasInBounds(
            coutputs,
            sanityCheck,
            interner,
            keywords,
            originalCallingDenizenId,
            needleTemplateName,
            newSubstitutingTemplatas,
            boundArgumentsSource,
            instantiationBoundArgs)
        // If we're inside:
        //   MyList.drop<MyList.drop$0>(MyList<MyList.drop$0>)void
        // we might be reaching into that MyList struct, which contains a:
        //   Opt<MyList<MyList$0>>
        // and we want to turn it into a:
        //   Opt<MyList<MyList.drop$0>>
        // We'll need to create some bound args for that MyList<MyList.drop$0>.
        // First, we take the original bound args for
        //   MyList<MyList.drop$0>
        // which was
        //   _2114 -> MyList.bound:drop<>(^MyList$0)void
        // and we can just substitute it to:
        //   _2114 -> MyList.bound:drop<>(^MyList.bound:drop$0)void
        // This is the bound that MyList.drop will look for.
        x
      }
      case UseBoundsFromContainer(containerInstantiationBoundParams, containerInstantiationBoundArgs) => {
        // Here, we're grabbing something inside a struct, like with the dot operator. We'll want to
        // make some instantiation bound args for this new type that our function knows about.
        // Luckily, we can use some bounds from the containing struct to satisfy its members bounds.

        val containerFuncBoundToBoundArg =
          containerInstantiationBoundArgs.runeToBoundPrototype.map({ case (rune, containerFuncBoundArg) =>
            vassertSome(containerInstantiationBoundParams.runeToBoundPrototype.get(rune)) -> containerFuncBoundArg
          })
        val containerImplBoundToBoundArg =
          containerInstantiationBoundArgs.runeToBoundImpl.map({ case (rune, containerImplBoundArg) =>
            vassertSome(containerInstantiationBoundParams.runeToBoundImpl.get(rune)) -> containerImplBoundArg
          })
        InstantiationBoundArgumentsT.make(
          instantiationBoundArgs.runeToBoundPrototype.mapValues(funcBoundArg => {
            funcBoundArg match {
              case PrototypeT(IdT(packageCoord, initSteps, fbn@FunctionBoundNameT(_, _, _)), returnType) => {
                vassertSome(
                  containerFuncBoundToBoundArg.get(PrototypeT(IdT(packageCoord, initSteps, fbn), returnType)))
              }
              case _ => {
                // Not sure if this call is really necessary...
                substituteTemplatasInPrototype(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, funcBoundArg)
              }
            }
          }),
          instantiationBoundArgs.runeToCitizenRuneToReachablePrototype.map({ case (calleeRune, InstantiationReachableBoundArgumentsT(citizenRuneToReachablePrototype)) =>
            calleeRune ->
                InstantiationReachableBoundArgumentsT(
                  citizenRuneToReachablePrototype.map({ case (citizenRune, reachablePrototype) =>
                    citizenRune ->
                        (reachablePrototype match {
                          case PrototypeT(IdT(packageCoord, initSteps, fbn@FunctionBoundNameT(_, _, _)), returnType) => {
                            vassertSome(containerFuncBoundToBoundArg.get(PrototypeT(IdT(packageCoord, initSteps, fbn), returnType)))
                          }
                          case _ => {
                            // Not sure if this call is really necessary...
                            substituteTemplatasInPrototype(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, reachablePrototype)
                          }
                        })
                  }))
          }),
          instantiationBoundArgs.runeToBoundImpl.mapValues(implBoundArg => {
            implBoundArg match {
              case IdT(packageCoord, initSteps, ibn@ImplBoundNameT(_, _)) => {
                vassertSome(containerImplBoundToBoundArg.get(IdT(packageCoord, initSteps, ibn)))
              }
              case _ => {
                // Not sure if this call is really necessary...
                substituteTemplatasInImplId(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, implBoundArg)
              }
            }
          }))
      }
    }
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_impl_id(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        impl_id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def substituteTemplatasInImplId[T <: IImplNameT](
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],

    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    implId: IdT[T]):
  IdT[T] = {
    val IdT(packageCoord, initSteps, last) = implId
    val substitutedImplId =
      IdT(
        packageCoord,
        initSteps,
        last match {
          case in @ ImplNameT(template, templateArgs, subCitizen) => {
            interner.intern(ImplNameT(
              template,
              templateArgs.map((templata: ITemplataT[ITemplataType]) => substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, templata)),
              expectKindTemplata(substituteTemplatasInKind(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, subCitizen)).kind.expectCitizen()))
          }
          case other => vimpl(other)
        })

    val instantiationBoundArgs = vassertSome(coutputs.getInstantiationBounds(implId))
    // See SBITAFD, we need to register bounds for these new instantiations.
    coutputs.addInstantiationBounds(
      sanityCheck,
      interner,
      originalCallingDenizenId,
      substitutedImplId,
      translateInstantiationBounds(
        coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, instantiationBoundArgs))

    vassert(substitutedImplId.localName.getClass.equals(implId.localName.getClass))
    vassert(substitutedImplId.getClass.equals(implId.getClass))
    val result = substitutedImplId.asInstanceOf[IdT[T]]
    assert(result != null)
    return result
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_bounds(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        bound_args: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> InstantiationBoundArgumentsT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def substituteTemplatasInBounds(
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],

    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    boundArgs: InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]):
  InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT] = {
    val InstantiationBoundArgumentsT(runeToFunctionBoundArg, callerKindRuneToReachableBoundArguments, runeToImplBoundArg) = boundArgs
    InstantiationBoundArgumentsT.make(
      runeToFunctionBoundArg.mapValues({ case funcBoundArg =>
        substituteTemplatasInPrototype(
          coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, funcBoundArg)
      }),
      callerKindRuneToReachableBoundArguments.map({ case (callerRune, InstantiationReachableBoundArgumentsT(citizenRuneToReachablePrototype)) =>
        callerRune ->
            InstantiationReachableBoundArgumentsT(
              citizenRuneToReachablePrototype.map({ case (citizenRune, reachablePrototype) =>
                citizenRune ->
                  substituteTemplatasInPrototype(
                    coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, reachablePrototype)
              }))
      }),
      runeToImplBoundArg.mapValues(implBoundArg => {
        substituteTemplatasInImplId(
          coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, implBoundArg)
      }))
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_interface(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        interface_tt: &'t InterfaceTT<'s, 't>,
    ) -> &'t InterfaceTT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def substituteTemplatasInInterface(
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],

    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    interfaceTT: InterfaceTT):
  InterfaceTT = {
    val InterfaceTT(IdT(packageCoord, initSteps, last)) = interfaceTT
    val newInterface =
      interner.intern(
        InterfaceTT(
          IdT(
            packageCoord,
            initSteps,
            last match {
              case InterfaceNameT(template, templateArgs) => {
                interner.intern(InterfaceNameT(
                  template,
                  templateArgs.map((templata: ITemplataT[ITemplataType]) => substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, templata))))
              }
            })))
    // See SBITAFD, we need to register bounds for these new instantiations.
    val instantiationBoundArgs =
      vassertSome(coutputs.getInstantiationBounds(interfaceTT.id))
    coutputs.addInstantiationBounds(
      sanityCheck,
      interner,
      originalCallingDenizenId,
      newInterface.id,
      translateInstantiationBounds(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, instantiationBoundArgs))
    newInterface
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_templata(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        templata: ITemplataT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def substituteTemplatasInTemplata(
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],

    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    templata: ITemplataT[ITemplataType]):
  ITemplataT[ITemplataType] = {
    templata match {
      case CoordTemplataT(c) => CoordTemplataT(substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, c))
      case KindTemplataT(k) => substituteTemplatasInKind(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, k)
      case PlaceholderTemplataT(id @ IdT(_, _, pn), _) => {
        if (id.initId(interner) == needleTemplateName) {
          newSubstitutingTemplatas(pn.index)
        } else {
          templata
        }
      }
      case MutabilityTemplataT(_) => templata
      case VariabilityTemplataT(_) => templata
      case IntegerTemplataT(_) => templata
      case BooleanTemplataT(_) => templata
      case PrototypeTemplataT(prototype) => {
        PrototypeTemplataT(
          substituteTemplatasInPrototype(
            coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, prototype))
      }
      case other => vimpl(other)
    }
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        original_prototype: &'t PrototypeT<'s, 't>,
    ) -> &'t PrototypeT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def substituteTemplatasInPrototype[T <: IFunctionNameT](
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],
    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    originalPrototype: PrototypeT[T]):
  PrototypeT[T] = {
    val PrototypeT(IdT(packageCoord, initSteps, funcName), returnType) = originalPrototype
    val substitutedTemplateArgs = funcName.templateArgs.map((templata: ITemplataT[ITemplataType]) => substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, templata))
    val substitutedParams = funcName.parameters.map((coord: CoordT) => substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, coord))
    val substitutedReturnType = substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, returnType)
    val substitutedFuncName = funcName.template.makeFunctionName(interner, keywords, substitutedTemplateArgs, substitutedParams)
    val tentativeId = IdT(packageCoord, initSteps, substitutedFuncName)

    val perhapsImportedId =
      tentativeId.localName match {
        case n @ FunctionBoundNameT(_, _, _) => {
          // Always import a seen function bound into our own environment, see MFBFDP.
          val importedId = originalCallingDenizenId.addStep(n)
          // It's a function bound, it has no function bounds of its own.
          coutputs.addInstantiationBounds(
            sanityCheck,
            interner,
            originalCallingDenizenId,
            importedId,
            InstantiationBoundArgumentsT.make[IFunctionNameT, IImplNameT](Map(), Map(), Map()))
          importedId
      }
      case _ => {
        // Not really sure if we're supposed to add bounds or something here.
          vassert(coutputs.getInstantiationBounds(tentativeId).nonEmpty)
          tentativeId
      }
    }
    val prototype = PrototypeT(perhapsImportedId, substitutedReturnType)

    vassert(substitutedFuncName.getClass.equals(funcName.getClass))
    vassert(originalPrototype.getClass.equals(prototype.getClass))
    val result = prototype.asInstanceOf[PrototypeT[T]]
    assert(result != null)
    return result
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn substitute_templatas_in_function_bound_id(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        original: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def substituteTemplatasInFunctionBoundId(
    coutputs: CompilerOutputs,
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],

    needleTemplateName: IdT[ITemplateNameT],
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    boundArgumentsSource: IBoundArgumentsSource,
    original: IdT[FunctionBoundNameT]):
  IdT[FunctionBoundNameT] = {
    val IdT(packageCoord, initSteps, funcName) = original
    val substitutedTemplateArgs =
      funcName.templateArgs.map((templata: ITemplataT[ITemplataType]) => substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, templata))
    val substitutedParams = funcName.parameters.map((coord: CoordT) => substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, coord))
//    val substitutedReturnType = substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, returnType, substitutions)
    val substitutedFuncName = funcName.template.makeFunctionName(interner, keywords, substitutedTemplateArgs, substitutedParams)
    val newId = IdT(packageCoord, initSteps, substitutedFuncName)

    // It's a function bound, it has no function bounds of its own.
    coutputs.addInstantiationBounds(
      sanityCheck,
      interner,
      originalCallingDenizenId,
      newId,
      InstantiationBoundArgumentsT.make(Map(), Map(), Map()))

    newId
  }

  // def substituteTemplatasInFunctionReachableId(
  //     coutputs: CompilerOutputs,
  //     sanityCheck: Boolean, interner: Interner,
  //     keywords: Keywords,
  //     needleTemplateName: IdT[ITemplateNameT],
  //     newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
  //     boundArgumentsSource: IBoundArgumentsSource,
  //     original: IdT[ReachableFunctionNameT]):
  // IdT[ReachableFunctionNameT] = {
  //   val IdT(packageCoord, initSteps, funcName) = original
  //   val substitutedTemplateArgs =
  //     funcName.templateArgs.map((templata: ITemplataT[ITemplataType]) => substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, templata))
  //   val substitutedParams = funcName.parameters.map((coord: CoordT) => substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, coord))
  //   //    val substitutedReturnType = substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, returnType, substitutions)
  //   val substitutedFuncName = funcName.template.makeFunctionName(interner, keywords, substitutedTemplateArgs, substitutedParams)
  //   val newId = IdT(packageCoord, initSteps, substitutedFuncName)
  //
  //   // It's a function bound, it has no function bounds of its own.
  //   coutputs.addInstantiationBounds(
  //     interner,
  //     originalCallingDenizenId,
  //     newId,
  //     InstantiationBoundArgumentsT(Map(), Map(), Map()))
  //
  //   newId
  // }
*/
// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)

// IPlaceholderSubstituter: Scala source is a trait defined inside TemplataCompiler.getPlaceholderSubstituter,
// so it has no separate top-level case-class anchor in TemplataCompiler.scala. Defined here as a struct per
// Slab 14 Gotcha 9 (single-implementor trait → struct with inherent methods). The five fields below mirror
// Scala's anonymous-trait-impl closure captures at TemplataCompiler.scala:808-824 (sanityCheck,
// originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource).
pub struct IPlaceholderSubstituter<'s, 't> {
    pub sanity_check: bool,
    pub original_calling_denizen_id: IdT<'s, 't>,
    pub needle_template_name: IdT<'s, 't>,
    pub new_substituting_templatas: &'t [ITemplataT<'s, 't>],
    pub bound_arguments_source: IBoundArgumentsSource<'s, 't>,
}
impl<'s, 't> IPlaceholderSubstituter<'s, 't> {
    pub fn substitute_for_coord(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        coord_t: CoordT<'s, 't>,
    ) -> CoordT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    pub fn substitute_for_interface(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        interface_tt: InterfaceTT<'s, 't>,
    ) -> InterfaceTT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    pub fn substitute_for_templata(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        templata: ITemplataT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    pub fn substitute_for_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        proto: &'t PrototypeT<'s, 't>,
    ) -> &'t PrototypeT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    pub fn substitute_for_impl_id(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        impl_id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
}
/*
  trait IPlaceholderSubstituter {
*/
// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)
/*
    def substituteForCoord(coutputs: CompilerOutputs, coordT: CoordT): CoordT
*/
/*
    def substituteForInterface(coutputs: CompilerOutputs, interfaceTT: InterfaceTT): InterfaceTT
*/
/*
    def substituteForTemplata(coutputs: CompilerOutputs, coordT: ITemplataT[ITemplataType]): ITemplataT[ITemplataType]
*/
/*
    def substituteForPrototype[T <: IFunctionNameT](coutputs: CompilerOutputs, proto: PrototypeT[T]): PrototypeT[T]
*/
/*
    def substituteForImplId[T <: IImplNameT](coutputs: CompilerOutputs, implId: IdT[T]): IdT[T]
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_placeholder_substituter(
        &self,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        name: IdT<'s, 't>,
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
    ) -> IPlaceholderSubstituter<'s, 't> {
        let top_level_denizen_id = self.get_top_level_denizen_id(name);
        let top_level_local_name: IInstantiationNameT<'s, 't> =
            top_level_denizen_id.local_name.try_into()
                .unwrap_or_else(|_| panic!("get_placeholder_substituter: topLevelDenizenId.localName must be IInstantiationNameT, got {:?}", top_level_denizen_id.local_name));
        let template_args: &[ITemplataT<'s, 't>] = top_level_local_name.template_args();
        let top_level_denizen_template_id = self.get_template(top_level_denizen_id);
        self.get_placeholder_substituter_ext(
            sanity_check,
            original_calling_denizen_id,
            *top_level_denizen_template_id,
            template_args,
            bound_arguments_source,
        )
    }
}
/*
  def getPlaceholderSubstituter(
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],

    // This is the Ship<WarpFuel>.
    name: IdT[IInstantiationNameT],
    boundArgumentsSource: IBoundArgumentsSource):
    // The Engine<T> is given later to the IPlaceholderSubstituter
  IPlaceholderSubstituter = {
    val topLevelDenizenId = getTopLevelDenizenId(name)
    val templateArgs = topLevelDenizenId.localName.templateArgs
    val topLevelDenizenTemplateId = getTemplate(topLevelDenizenId)

    TemplataCompiler.getPlaceholderSubstituter(
      sanityCheck,
      interner,
      keywords,
      originalCallingDenizenId,
      topLevelDenizenTemplateId,
      templateArgs,
      boundArgumentsSource)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_placeholder_substituter_ext(
        &self,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &'t [ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
    ) -> IPlaceholderSubstituter<'s, 't> {
        IPlaceholderSubstituter {
            sanity_check,
            original_calling_denizen_id,
            needle_template_name,
            new_substituting_templatas,
            bound_arguments_source,
        }
    }
}
/*
  // Let's say you have the line:
  //   myShip.engine
  // You need to somehow combine these two bits of knowledge:
  // - You have a Ship<WarpFuel>
  // - Ship<T> contains an Engine<T>.
  // To get back an Engine<WarpFuel>. This is the function that does that.
  def getPlaceholderSubstituter(
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],
    // This is the Ship.
    needleTemplateName: IdT[ITemplateNameT],
    // This is the <WarpFuel>.
    newSubstitutingTemplatas: Vector[ITemplataT[ITemplataType]],
    // The Engine<T> is given later to the IPlaceholderSubstituter
    boundArgumentsSource: IBoundArgumentsSource):
  IPlaceholderSubstituter = {
    new IPlaceholderSubstituter {
      override def substituteForCoord(coutputs: CompilerOutputs, coordT: CoordT): CoordT = {
        TemplataCompiler.substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, coordT)
      }
      override def substituteForInterface(coutputs: CompilerOutputs, interfaceTT: InterfaceTT): InterfaceTT = {
        TemplataCompiler.substituteTemplatasInInterface(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, interfaceTT)
      }
      override def substituteForTemplata(coutputs: CompilerOutputs, templata: ITemplataT[ITemplataType]): ITemplataT[ITemplataType] = {
        TemplataCompiler.substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, templata)
      }
      override def substituteForPrototype[T <: IFunctionNameT](coutputs: CompilerOutputs, proto: PrototypeT[T]): PrototypeT[T] = {
        TemplataCompiler.substituteTemplatasInPrototype(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, proto)
      }
      override def substituteForImplId[T <: IImplNameT](coutputs: CompilerOutputs, implId: IdT[T]): IdT[T] = {
        TemplataCompiler.substituteTemplatasInImplId(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, implId)
      }
    }
  }

//  // If you have a type (citizenTT) and it contains something (like a member) then
//  // you can use this function to figure out what the member looks like to you, the outsider.
//  // It will take out all the internal placeholders internal to the citizen, and replace them
//  // with what was given in citizenTT's template args.
//  def getTemplataTransformer(sanityCheck: Boolean, interner: Interner, coutputs: CompilerOutputs, citizenTT: ICitizenTT):
//  (ITemplata[ITemplataType]) => ITemplata[ITemplataType] = {
//    val citizenTemplateFullName = TemplataCompiler.getCitizenTemplate(citizenTT.fullName)
//    val citizenTemplateDefinition = coutputs.lookupCitizen(citizenTemplateFullName)
//    vassert(
//      citizenTT.fullName.last.templateArgs.size ==
//        citizenTemplateDefinition.placeholderedCitizen.fullName.last.templateArgs.size)
//    val substitutions =
//      citizenTT.fullName.last.templateArgs
//        .zip(citizenTemplateDefinition.placeholderedCitizen.fullName.last.templateArgs)
//        .flatMap({
//          case (arg, p @ PlaceholderTemplata(_, _)) => Some((p, arg))
//          case _ => None
//        }).toVector
//    (templataToTransform: ITemplata[ITemplataType]) => {
//      TemplataCompiler.substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, templataToTransform, substitutions)
//    }
//  }

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_reachable_bounds(
        &self,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        citizen: ICitizenTT<'s, 't>,
    ) -> InstantiationReachableBoundArgumentsT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getReachableBounds(
    sanityCheck: Boolean, interner: Interner,
    keywords: Keywords,
    originalCallingDenizenId: IdT[ITemplateNameT],
    coutputs: CompilerOutputs,
    citizen: ICitizenTT):
  InstantiationReachableBoundArgumentsT[FunctionBoundNameT] = {
    val substituter =
      TemplataCompiler.getPlaceholderSubstituter(
        sanityCheck, interner, keywords,
        originalCallingDenizenId,
        citizen.id,
        // This function is all about gathering bounds from the incoming parameter types.
        InheritBoundsFromTypeItself)
    val citizenTemplateId = TemplataCompiler.getCitizenTemplate(citizen.id)
    val innerEnv = coutputs.getInnerEnvForType(citizenTemplateId)
    InstantiationReachableBoundArgumentsT(
      innerEnv
          .templatas
          .entriesByNameT
          .collect({
            case (RuneNameT(rune), TemplataEnvEntry(PrototypeTemplataT(PrototypeT(IdT(packageCoord, initSteps, FunctionBoundNameT(FunctionBoundTemplateNameT(humanName), templateArgs, params)), returnType)))) => {
              val prototype = PrototypeT(IdT(packageCoord, initSteps, interner.intern(FunctionBoundNameT(interner.intern(FunctionBoundTemplateNameT(humanName)), templateArgs, params))), returnType)
              rune -> substituter.substituteForPrototype(coutputs, prototype)
            }
          })
          .toMap)
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_first_unsolved_identifying_rune(
        &self,
        generic_parameters: &'s [&'s GenericParameterS<'s>],
        is_solved: impl Fn(IRuneS<'s>) -> bool,
    ) -> Option<(&'s GenericParameterS<'s>, i32)> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
}
/*
  def getFirstUnsolvedIdentifyingRune(
    genericParameters: Vector[GenericParameterS],
    isSolved: IRuneS => Boolean):
  Option[(GenericParameterS, Int)] = {
    genericParameters
      .zipWithIndex
      .map({ case (genericParam, index) =>
        (genericParam, index, isSolved(genericParam.rune.rune))
      })
      .filter(!_._3)
      .map({ case (genericParam, index, false) => (genericParam, index) })
      .headOption
  }
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn create_rune_type_solver_env(
        &self,
        parent_env: &'t IInDenizenEnvironmentT<'s, 't>,
    ) -> TemplataCompilerRuneTypeSolverEnv<'_, 's, 't> {
        TemplataCompilerRuneTypeSolverEnv {
            parent_env,
            typing_interner: self.typing_interner,
        }
    }
    /*
Guardian: disable-all
      def createRuneTypeSolverEnv(parentEnv: IInDenizenEnvironmentT): IRuneTypeSolverEnv = {
        new IRuneTypeSolverEnv {
    */
}


// Concrete IRuneTypeSolverEnv produced by `create_rune_type_solver_env` above. The
// Scala anonymous `new IRuneTypeSolverEnv` at TemplataCompiler.scala:1513 closes over
// `parentEnv` and dispatches to either a LambdaStructImpreciseNameS special case or
// `parentEnv.lookupNearestWithImpreciseName`. Same shape pattern as
// `HigherTypingRuneTypeSolverEnv` (higher_typing_pass.rs) and `LetExprRuneTypeSolverEnv`
// (expression_compiler.rs).
// Rust adaptation (SPDMX-B): typing_interner field added for entry_to_templata.
pub struct TemplataCompilerRuneTypeSolverEnv<'a, 's, 't>
where
    's: 't,
{
    parent_env: &'t IInDenizenEnvironmentT<'s, 't>,
    typing_interner: &'a crate::typing::typing_interner::TypingInterner<'s, 't>,
}
/*
Guardian: disable-all
*/

impl<'a, 's, 't> crate::postparsing::rune_type_solver::IRuneTypeSolverEnv<'s>
for TemplataCompilerRuneTypeSolverEnv<'a, 's, 't>
where
    's: 't,
{
    fn lookup(
        &self,
        range: RangeS<'s>,
        name_s: crate::postparsing::names::IImpreciseNameS<'s>,
    ) -> Result<
        crate::postparsing::rune_type_solver::IRuneTypeSolverLookupResult<'s>,
        crate::postparsing::rune_type_solver::IRuneTypingLookupFailedError<'s>,
    > {
        match name_s {
            crate::postparsing::names::IImpreciseNameS::LambdaStructImpreciseName(_) => {
                // Scala: vregionmut() // Take out with regions
                // Lambdas look up their struct as a KindTemplata in their environment, they don't
                // look up the origin template by name. (Scala comment from astronomizeLambda.)
                Ok(crate::postparsing::rune_type_solver::IRuneTypeSolverLookupResult::Templata(
                    crate::postparsing::rune_type_solver::TemplataLookupResult {
                        templata: crate::postparsing::itemplatatype::ITemplataType::KindTemplataType(
                            crate::postparsing::itemplatatype::KindTemplataType {},
                        ),
                    },
                ))
            }
            _ => {
                let mut filter = std::collections::HashSet::new();
                filter.insert(crate::typing::env::environment::ILookupContext::TemplataLookupContext);
                match self.parent_env.lookup_nearest_with_imprecise_name(name_s, filter, self.typing_interner) {
                    Some(crate::typing::templata::templata::ITemplataT::StructDefinition(t)) => {
                        Ok(crate::postparsing::rune_type_solver::IRuneTypeSolverLookupResult::Citizen(
                            crate::postparsing::rune_type_solver::CitizenRuneTypeSolverLookupResult {
                                tyype: crate::postparsing::itemplatatype::ITemplataType::TemplateTemplataType(
                                    t.origin_struct.tyype,
                                ),
                                generic_params: t.origin_struct.generic_parameters,
                            },
                        ))
                    }
                    Some(crate::typing::templata::templata::ITemplataT::InterfaceDefinition(t)) => {
                        Ok(crate::postparsing::rune_type_solver::IRuneTypeSolverLookupResult::Citizen(
                            crate::postparsing::rune_type_solver::CitizenRuneTypeSolverLookupResult {
                                tyype: crate::postparsing::itemplatatype::ITemplataType::TemplateTemplataType(
                                    t.origin_interface.tyype,
                                ),
                                generic_params: t.origin_interface.generic_parameters,
                            },
                        ))
                    }
                    Some(_x) => {
                        // Scala: case Some(x) => Ok(TemplataLookupResult(x.tyype))
                        // Requires `ITemplataT::tyype()` getter — separate scaffolding gap (see TL.md residual items).
                        panic!("TemplataCompilerRuneTypeSolverEnv: ITemplataT::tyype() not yet implemented");
                    }
                    None => Err(
                        crate::postparsing::rune_type_solver::IRuneTypingLookupFailedError::CouldntFindType(
                            crate::postparsing::rune_type_solver::RuneTypingCouldntFindType {
                                range,
                                name: name_s,
                            },
                        ),
                    ),
                }
            }
        }
    }
/*
      override def lookup(
          range: RangeS,
          nameS: IImpreciseNameS
      ): Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
        nameS match {
          case LambdaStructImpreciseNameS(_) => {
            vregionmut() // Take out with regions
            Ok(TemplataLookupResult(KindTemplataType()))
            // Put back in with regions
            // Ok(TemplataLookupResult(TemplateTemplataType(Vector(RegionTemplataType()), KindTemplataType())))
          }
          case _ => {
            parentEnv.lookupNearestWithImpreciseName(nameS, Set(TemplataLookupContext)) match {
              case Some(CitizenDefinitionTemplataT(environment, a)) => {
                Ok(CitizenRuneTypeSolverLookupResult(a.tyype, a.genericParameters))
              }
              case Some(x) => Ok(TemplataLookupResult(x.tyype))
              case None => Err(RuneTypingCouldntFindType(range, nameS))
            }
          }
        }
      }
    }
  }
}
*/
}
/*
class TemplataCompiler(
  interner: Interner,
  opts: TypingPassOptions,

  nameTranslator: NameTranslator,
  delegate: ITemplataCompilerDelegate) {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn is_type_convertible(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: &'t IInDenizenEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        source_pointer_type: CoordT<'s, 't>,
        target_pointer_type: CoordT<'s, 't>,
    ) -> bool {
        let CoordT { ownership: target_ownership, region: target_region, kind: target_type } = target_pointer_type;
        let CoordT { ownership: source_ownership, region: source_region, kind: source_type } = source_pointer_type;

        match (&source_type, &target_type) {
            (KindT::Never(_), _) => return true,
            (a, b) if a == b => {}
            _ => {
                panic!("implement: isTypeConvertible — non-equal kind cases");
            }
        }

        if source_region != target_region {
            return false;
        }

        match (source_ownership, target_ownership) {
            (a, b) if a == b => {}
            _ => {
                panic!("implement: isTypeConvertible — non-equal ownership cases");
            }
        }

        true
    }
/*
  def isTypeConvertible(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    sourcePointerType: CoordT,
    targetPointerType: CoordT):
  Boolean = {

    val CoordT(targetOwnership, targetRegion, targetType) = targetPointerType;
    val CoordT(sourceOwnership, sourceRegion, sourceType) = sourcePointerType;

    // Note the Never case will short-circuit a true, regardless of the other checks (ownership)

    (sourceType, targetType) match {
      case (NeverT(_), _) => return true
      case (a, b) if a == b =>
      case (VoidT() |
            IntT(_) |
            BoolT() |
            StrT() |
            FloatT() |
            contentsRuntimeSizedArrayTT(_, _, _) |
            contentsStaticSizedArrayTT(_, _, _, _, _), _) => {
        return false
      }
      case (_, VoidT() |
               IntT(_) |
               BoolT() |
               StrT() |
               FloatT() |
               contentsRuntimeSizedArrayTT(_, _, _) |
               contentsStaticSizedArrayTT(_, _, _, _, _)) => {
        return false
      }
      case (_, StructTT(_)) => return false
      case (a : ISubKindTT, b : ISuperKindTT) => {
        delegate.isParent(coutputs, callingEnv, parentRanges, callLocation, a, b) match {
          case IsParent(_, _, _) =>
          case IsntParent(_) => return false
        }
      }
      case _ => {
        vfail("Dont know if we can convert from " + sourceType + " to " + targetType)
      }
    }

    if (sourceRegion != targetRegion) {
      return false
    }

    (sourceOwnership, targetOwnership) match {
      case (a, b) if a == b =>
      // At some point maybe we should automatically convert borrow to pointer and vice versa
      // and perhaps automatically promote borrow or pointer to weak?
      case (OwnT, BorrowT) => return false
      case (OwnT, WeakT) => return false
      case (OwnT, ShareT) => return false
      case (BorrowT, OwnT) => return false
      case (BorrowT, WeakT) => return false
      case (BorrowT, ShareT) => return false
      case (WeakT, OwnT) => return false
      case (WeakT, BorrowT) => return false
      case (WeakT, ShareT) => return false
      case (ShareT, BorrowT) => return false
      case (ShareT, WeakT) => return false
      case (ShareT, OwnT) => return false
    }

    true
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn pointify_kind(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        kind: KindT<'s, 't>,
        region: RegionT,
        ownership_if_mutable: OwnershipT,
    ) -> CoordT<'s, 't> {
        let mutability = self.get_mutability(coutputs, kind);
        let ownership =
            match mutability {
                ITemplataT::Placeholder(_) => { panic!("Unimplemented: pointify_kind PlaceholderTemplataT"); }
                ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => ownership_if_mutable,
                ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => OwnershipT::Share,
                _ => { panic!("Unimplemented: pointify_kind unexpected mutability"); }
            };
        match kind {
            KindT::RuntimeSizedArray(_) => { panic!("Unimplemented: pointify_kind RuntimeSizedArray"); }
            KindT::StaticSizedArray(_) => { panic!("Unimplemented: pointify_kind StaticSizedArray"); }
            KindT::Struct(_) => CoordT { ownership, region, kind },
            KindT::Interface(_) => CoordT { ownership, region, kind },
            KindT::Void(_) => CoordT { ownership: OwnershipT::Share, region, kind },
            KindT::Int(_) => CoordT { ownership: OwnershipT::Share, region, kind },
            KindT::Float(_) => CoordT { ownership: OwnershipT::Share, region, kind },
            KindT::Bool(_) => CoordT { ownership: OwnershipT::Share, region, kind },
            KindT::Str(_) => CoordT { ownership: OwnershipT::Share, region, kind },
            _ => { panic!("Unimplemented: pointify_kind other kind"); }
        }
    }
/*
  def pointifyKind(
    coutputs: CompilerOutputs,
    kind: KindT,
    region: RegionT,
    ownershipIfMutable: OwnershipT
  ): CoordT = {
    val mutability = Compiler.getMutability(coutputs, kind)
    val ownership =
      mutability match {
        case PlaceholderTemplataT(idT, tyype) => vimpl()
        case MutabilityTemplataT(MutableT) => ownershipIfMutable
        case MutabilityTemplataT(ImmutableT) => ShareT
      }
    kind match {
      case a@contentsRuntimeSizedArrayTT(_, _, _) => {
        CoordT(ownership, region, a)
      }
      case a@contentsStaticSizedArrayTT(_, _, _, _, _) => {
        CoordT(ownership, region, a)
      }
      case s @ StructTT(_) => {
        CoordT(ownership, region, s)
      }
      case i @ InterfaceTT(_) => {
        CoordT(ownership, region, i)
      }
      case VoidT() => {
        CoordT(ShareT, region, VoidT())
      }
      case i @ IntT(_) => {
        CoordT(ShareT, region, i)
      }
      case FloatT() => {
        CoordT(ShareT, region, FloatT())
      }
      case BoolT() => {
        CoordT(ShareT, region, BoolT())
      }
      case StrT() => {
        CoordT(ShareT, region, StrT())
      }
    }
  }

//  def evaluateStructTemplata(
//    coutputs: CompilerOutputs,
//    callRange: List[RangeS],
//    template: StructTemplata,
//    templateArgs: Vector[ITemplata[ITemplataType]],
//    expectedType: ITemplataType):
//  (ITemplata[ITemplataType]) = {
//    val uncoercedTemplata =
//      delegate.resolveStruct(coutputs, callRange, template, templateArgs)
//    val templata =
//      coerce(coutputs, callRange, KindTemplata(uncoercedTemplata), expectedType)
//    (templata)
//  }

//  def evaluateBuiltinTemplateTemplata(
//    env: IEnvironment,
//    coutputs: CompilerOutputs,
//    range: List[RangeS],
//    template: RuntimeSizedArrayTemplateTemplata,
//    templateArgs: Vector[ITemplata[ITemplataType]],
//    expectedType: ITemplataType):
//  (ITemplata[ITemplataType]) = {
//    val Vector(m, CoordTemplata(elementType)) = templateArgs
//    val mutability = ITemplata.expectMutability(m)
//    val arrayKindTemplata = delegate.getRuntimeSizedArrayKind(env, coutputs, elementType, mutability)
//    val templata =
//      coerce(coutputs, callingEnv, range, KindTemplata(arrayKindTemplata), expectedType)
//    (templata)
//  }

//  def getStaticSizedArrayKind(
//    env: IEnvironment,
//    coutputs: CompilerOutputs,
//    callRange: List[RangeS],
//    mutability: ITemplata[MutabilityTemplataType],
//    variability: ITemplata[VariabilityTemplataType],
//    size: ITemplata[IntegerTemplataType],
//    element: CoordT,
//    expectedType: ITemplataType):
//  (ITemplata[ITemplataType]) = {
//    val uncoercedTemplata =
//      delegate.getStaticSizedArrayKind(env, coutputs, mutability, variability, size, element)
//    val templata =
//      coerce(coutputs, callingEnv, callRange, KindTemplata(uncoercedTemplata), expectedType)
//    (templata)
//  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn lookup_templata_by_name(
        &self,
        env: IEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        name: INameT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def lookupTemplata(
    env: IEnvironmentT,
    coutputs: CompilerOutputs,
    range: List[RangeS],
    name: INameT):
  (ITemplataT[ITemplataType]) = {
    // Changed this from AnythingLookupContext to TemplataLookupContext
    // because this is called from StructCompiler to figure out its members.
    // We could instead pipe a lookup context through, if this proves problematic.
    vassertOne(env.lookupNearestWithName(name, Set(TemplataLookupContext)))
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn lookup_templata_by_rune(
        &self,
        env: IEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        name: IImpreciseNameS<'s>,
    ) -> Option<ITemplataT<'s, 't>> {
        // Changed this from AnythingLookupContext to TemplataLookupContext
        // because this is called from StructCompiler to figure out its members.
        // We could instead pipe a lookup context through, if this proves problematic.
        let mut lookup_filter = std::collections::HashSet::new();
        lookup_filter.insert(ILookupContext::TemplataLookupContext);
        let results = env.lookup_nearest_with_imprecise_name(name, lookup_filter, self.typing_interner);
        if results.iter().count() > 1 {
            panic!("vfail");
        }
        results
    }
/*
  def lookupTemplata(
    env: IEnvironmentT,
    coutputs: CompilerOutputs,
    range: List[RangeS],
    name: IImpreciseNameS):
  Option[ITemplataT[ITemplataType]] = {
    // Changed this from AnythingLookupContext to TemplataLookupContext
    // because this is called from StructCompiler to figure out its members.
    // We could instead pipe a lookup context through, if this proves problematic.
    val results = env.lookupNearestWithImpreciseName(name, Set(TemplataLookupContext))
    if (results.size > 1) {
      vfail()
    }
    results.headOption
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn coerce_kind_to_coord(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        kind: KindT<'s, 't>,
        region: RegionT,
    ) -> CoordT<'s, 't> {
        let mutability = self.get_mutability(coutputs, kind);
        let ownership = match mutability {
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => OwnershipT::Own,
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => OwnershipT::Share,
            ITemplataT::Placeholder(_) => OwnershipT::Own,
            other => unreachable!("Unexpected mutability templata: {:?}", other),
        };
        CoordT { ownership, region, kind }
    }
/*
  def coerceKindToCoord(coutputs: CompilerOutputs, kind: KindT, region: RegionT):
  CoordT = {
    val mutability = Compiler.getMutability(coutputs, kind)
    CoordT(
      mutability match {
        case MutabilityTemplataT(MutableT) => OwnT
        case MutabilityTemplataT(ImmutableT) => ShareT
        case PlaceholderTemplataT(idT, tyype) => OwnT
      },
      region,
      kind)
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn coerce_to_coord(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        range: &[RangeS<'s>],
        templata: ITemplataT<'s, 't>,
        region: RegionT,
    ) -> ITemplataT<'s, 't> {
        match templata {
            ITemplataT::Kind(kind_templata) => {
                ITemplataT::Coord(self.typing_interner.alloc(
                    CoordTemplataT { coord: self.coerce_kind_to_coord(coutputs, kind_templata.kind, region) }
                ))
            }
            ITemplataT::Coord(_) => { panic!("vcurious"); }
            ITemplataT::StructDefinition(_) => { panic!("vcurious"); }
            ITemplataT::InterfaceDefinition(_) => { panic!("vcurious"); }
            _ => { panic!("Unimplemented: coerce_to_coord for {:?}", templata); }
        }
    }
/*
  def coerceToCoord(
    coutputs: CompilerOutputs,
    env: IInDenizenEnvironmentT,
    range: List[RangeS],
    templata: ITemplataT[ITemplataType],
    region: RegionT):
  (ITemplataT[ITemplataType]) = {
    if (templata.tyype == CoordTemplataType()) {
      vcurious()
      templata
    } else {
      templata match {
        case KindTemplataT(kind) => {
          CoordTemplataT(coerceKindToCoord(coutputs, kind, region))
        }
        case st@StructDefinitionTemplataT(declaringEnv, structA) => {
          vcurious()
//          if (structA.isTemplate) {
//            vfail("Can't coerce " + structA.name + " to be a coord, is a template!")
//          }
//          val kind =
//            delegate.resolveStruct(coutputs, env, range, st, Vector.empty).expect().kind
//          val mutability = Compiler.getMutability(coutputs, kind)
//
//          // Default ownership is own for mutables, share for imms
//          val ownership =
//            mutability match {
//              case MutabilityTemplata(MutableT) => OwnT
//              case MutabilityTemplata(ImmutableT) => ShareT
//              case PlaceholderTemplata(fullNameT, MutabilityTemplataType()) => vimpl()
//            }
//          val coerced = CoordTemplata(CoordT(ownership, kind))
//          (coerced)
        }
        case it@InterfaceDefinitionTemplataT(declaringEnv, interfaceA) => {
          vcurious()
//          if (interfaceA.isTemplate) {
//            vfail("Can't coerce " + interfaceA.name + " to be a coord, is a template!")
//          }
//          val kind =
//            delegate.resolveInterface(coutputs, env, range, it, Vector.empty).expect().kind
//          val mutability = Compiler.getMutability(coutputs, kind)
//          val coerced =
//            CoordTemplata(
//              CoordT(
//                mutability match {
//                  case MutabilityTemplata(MutableT) => OwnT
//                  case MutabilityTemplata(ImmutableT) => ShareT
//                  case PlaceholderTemplata(fullNameT, MutabilityTemplataType()) => vimpl()
//                },
//                kind))
//          (coerced)
        }
        case _ => {
          vfail("Can't coerce a " + templata.tyype + " to be a coord!")
        }
      }
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resolve_struct_template(
        &self,
        struct_templata: &'t StructDefinitionTemplataT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def resolveStructTemplate(structTemplata: StructDefinitionTemplataT): IdT[IStructTemplateNameT] = {
    val StructDefinitionTemplataT(declaringEnv, structA) = structTemplata
    declaringEnv.id.addStep(nameTranslator.translateStructName(structA.name))
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resolve_interface_template(
        &self,
        interface_templata: &'t InterfaceDefinitionTemplataT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def resolveInterfaceTemplate(interfaceTemplata: InterfaceDefinitionTemplataT): IdT[IInterfaceTemplateNameT] = {
    val InterfaceDefinitionTemplataT(declaringEnv, interfaceA) = interfaceTemplata
    declaringEnv.id.addStep(nameTranslator.translateInterfaceName(interfaceA.name))
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resolve_citizen_template(
        &self,
        citizen_templata: &'t CitizenDefinitionTemplataT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def resolveCitizenTemplate(citizenTemplata: CitizenDefinitionTemplataT): IdT[ICitizenTemplateNameT] = {
    citizenTemplata match {
      case st @ StructDefinitionTemplataT(_, _) => resolveStructTemplate(st)
      case it @ InterfaceDefinitionTemplataT(_, _) => resolveInterfaceTemplate(it)
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn citizen_is_from_template(
        &self,
        actual_citizen_ref: ICitizenTT<'s, 't>,
        expected_citizen_templata: ITemplataT<'s, 't>,
    ) -> bool {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def citizenIsFromTemplate(actualCitizenRef: ICitizenTT, expectedCitizenTemplata: ITemplataT[ITemplataType]): Boolean = {
    val citizenTemplateId =
      expectedCitizenTemplata match {
        case st @ StructDefinitionTemplataT(_, _) => resolveStructTemplate(st)
        case it @ InterfaceDefinitionTemplataT(_, _) => resolveInterfaceTemplate(it)
        case KindTemplataT(c : ICitizenTT) => TemplataCompiler.getCitizenTemplate(c.id)
        case CoordTemplataT(CoordT(OwnT | ShareT, _, c : ICitizenTT)) => TemplataCompiler.getCitizenTemplate(c.id)
        case _ => return false
      }
    TemplataCompiler.getCitizenTemplate(actualCitizenRef.id) == citizenTemplateId
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn create_placeholder(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        name_prefix: IdT<'s, 't>,
        generic_param: &'s GenericParameterS<'s>,
        index: i32,
        rune_to_type: &HashMap<IRuneS<'s>, ITemplataType<'s>>,
        current_height: Option<i32>,
        register_with_compiler_outputs: bool,
    ) -> ITemplataT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def createPlaceholder(
      coutputs: CompilerOutputs,
      env: IInDenizenEnvironmentT,
      namePrefix: IdT[INameT],
      genericParam: GenericParameterS,
      index: Int,
      runeToType: Map[IRuneS, ITemplataType],
      currentHeight: Option[Int],
      registerWithCompilerOutputs: Boolean
  ):
  ITemplataT[ITemplataType] = {
    val runeType = vassertSome(runeToType.get(genericParam.rune.rune))
    val rune = genericParam.rune.rune

    runeType match {
      case KindTemplataType() => {
        val (kindMutable, regionMutable) =
          genericParam.tyype match {
            case CoordGenericParameterTypeS(coordRegion, kindMutable, regionMutable) => {
              (if (kindMutable) OwnT else ShareT, regionMutable)
            }
            case _ => (OwnT, false)
          }
        createKindPlaceholderInner(
          coutputs, env, namePrefix, index, rune, kindMutable, registerWithCompilerOutputs)
      }
      case CoordTemplataType() => {
        val (kindMutable, regionMutability) =
          genericParam.tyype match {
            case CoordGenericParameterTypeS(coordRegion, kindMutable, regionMutable) => {
              (if (kindMutable) OwnT else ShareT, if (regionMutable) ReadWriteRegionS else ReadOnlyRegionS)
            }
            case _ => (OwnT, ReadOnlyRegionS)
          }
        createCoordPlaceholderInner(
          coutputs,
          env,
          namePrefix,
          index,
          rune,
          currentHeight,
          regionMutability,
          kindMutable,
          registerWithCompilerOutputs)
      }
      case otherType => {
        createNonKindNonRegionPlaceholderInner(namePrefix, index, rune, otherType)
      }
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn create_coord_placeholder_inner(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        name_prefix: IdT<'s, 't>,
        index: i32,
        rune: IRuneS<'s>,
        current_height: Option<i32>,
        region_mutability: IRegionMutabilityS,
        kind_ownership: OwnershipT,
        register_with_compiler_outputs: bool,
    ) -> CoordTemplataT<'s, 't> {
        // val regionPlaceholderTemplata = RegionT()
        let region_placeholder_templata = RegionT;

        // val kindPlaceholderT =
        //   createKindPlaceholderInner(
        //     coutputs, env, namePrefix, index, rune, kindOwnership, registerWithCompilerOutputs)
        let kind_placeholder_t = self.create_kind_placeholder_inner(
            coutputs, env, name_prefix, index, rune, kind_ownership, register_with_compiler_outputs);

        // CoordTemplataT(CoordT(kindOwnership, regionPlaceholderTemplata, kindPlaceholderT.kind))
        CoordTemplataT {
            coord: CoordT {
                ownership: kind_ownership,
                region: region_placeholder_templata,
                kind: kind_placeholder_t.kind,
            }
        }
    }
/*
  def createCoordPlaceholderInner(
      coutputs: CompilerOutputs,
      env: IInDenizenEnvironmentT,
      namePrefix: IdT[INameT],
      index: Int,
      rune: IRuneS,
      currentHeight: Option[Int],
      regionMutability: IRegionMutabilityS,
      kindOwnership: OwnershipT,
      registerWithCompilerOutputs: Boolean
  ): CoordTemplataT = {
    val regionPlaceholderTemplata = RegionT()

    val kindPlaceholderT =
      createKindPlaceholderInner(
        coutputs, env, namePrefix, index, rune, kindOwnership, registerWithCompilerOutputs)

    CoordTemplataT(CoordT(kindOwnership, regionPlaceholderTemplata, kindPlaceholderT.kind))
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn create_kind_placeholder_inner(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        name_prefix: IdT<'s, 't>,
        index: i32,
        rune: IRuneS<'s>,
        kind_ownership: OwnershipT,
        register_with_compiler_outputs: bool,
    ) -> KindTemplataT<'s, 't> {
        // val kindPlaceholderId =
        //   namePrefix.addStep(
        //     interner.intern(KindPlaceholderNameT(
        //       interner.intern(KindPlaceholderTemplateNameT(index, rune)))))
        let template_name = self.typing_interner.intern_kind_placeholder_template_name(
            KindPlaceholderTemplateNameT { index, rune, _phantom: std::marker::PhantomData });
        let placeholder_name = self.typing_interner.intern_kind_placeholder_name(
            KindPlaceholderNameT { template: template_name });
        let kind_placeholder_id = name_prefix.add_step(
            self.typing_interner, INameT::KindPlaceholder(placeholder_name));

        // val kindPlaceholderTemplateId =
        //   TemplataCompiler.getPlaceholderTemplate(kindPlaceholderId)
        let kind_placeholder_template_id_val = self.get_placeholder_template(*kind_placeholder_id);
        let kind_placeholder_template_id = self.typing_interner.intern_id(IdValT {
            package_coord: kind_placeholder_template_id_val.package_coord,
            init_steps: kind_placeholder_template_id_val.init_steps,
            local_name: kind_placeholder_template_id_val.local_name,
        });

        // if (registerWithCompilerOutputs) {
        if register_with_compiler_outputs {
            // coutputs.declareType(kindPlaceholderTemplateId)
            coutputs.declare_type(kind_placeholder_template_id);

            // val mutability = MutabilityTemplataT(kindOwnership match {
            //   case OwnT => MutableT
            //   case ShareT => ImmutableT
            // })
            let mutability = ITemplataT::Mutability(MutabilityTemplataT {
                mutability: match kind_ownership {
                    OwnershipT::Own => MutabilityT::Mutable,
                    OwnershipT::Share => MutabilityT::Immutable,
                    _ => panic!("create_kind_placeholder_inner: unexpected ownership"),
                },
            });
            // coutputs.declareTypeMutability(kindPlaceholderTemplateId, mutability)
            coutputs.declare_type_mutability(kind_placeholder_template_id, mutability);

            // val placeholderEnv = GeneralEnvironmentT.childOf(interner, env, kindPlaceholderTemplateId, kindPlaceholderTemplateId)
            let placeholder_env = child_of(
                self.typing_interner,
                self.scout_arena,
                *env,
                *kind_placeholder_template_id,
                kind_placeholder_template_id,
                vec![],
            );
            let placeholder_env_ref: &'t IInDenizenEnvironmentT<'s, 't> =
                self.typing_interner.alloc(IInDenizenEnvironmentT::General(placeholder_env));
            // coutputs.declareTypeOuterEnv(kindPlaceholderTemplateId, placeholderEnv)
            coutputs.declare_type_outer_env(kind_placeholder_template_id, placeholder_env_ref);
            // coutputs.declareTypeInnerEnv(kindPlaceholderTemplateId, placeholderEnv)
            coutputs.declare_type_inner_env(kind_placeholder_template_id, placeholder_env_ref);
        }

        // KindTemplataT(KindPlaceholderT(kindPlaceholderId))
        let kind_placeholder = self.typing_interner.intern_kind_placeholder(
            KindPlaceholderT { id: *kind_placeholder_id });
        KindTemplataT { kind: KindT::KindPlaceholder(kind_placeholder) }
    }
/*
  def createKindPlaceholderInner(
      coutputs: CompilerOutputs,
      env: IInDenizenEnvironmentT,
      namePrefix: IdT[INameT],
      index: Int,
      rune: IRuneS,
      kindOwnership: OwnershipT,
      registerWithCompilerOutputs: Boolean):
  KindTemplataT = {
    val kindPlaceholderId =
      namePrefix.addStep(
        interner.intern(KindPlaceholderNameT(
          interner.intern(KindPlaceholderTemplateNameT(index, rune)))))
    val kindPlaceholderTemplateId =
      TemplataCompiler.getPlaceholderTemplate(kindPlaceholderId)

    if (registerWithCompilerOutputs) {
      coutputs.declareType(kindPlaceholderTemplateId)

      val mutability =
        MutabilityTemplataT(
          kindOwnership match {
            case OwnT => MutableT
            case ShareT => ImmutableT
          })
      coutputs.declareTypeMutability(kindPlaceholderTemplateId, mutability)

      val placeholderEnv = GeneralEnvironmentT.childOf(interner, env, kindPlaceholderTemplateId, kindPlaceholderTemplateId)
      coutputs.declareTypeOuterEnv(kindPlaceholderTemplateId, placeholderEnv)
      coutputs.declareTypeInnerEnv(kindPlaceholderTemplateId, placeholderEnv)
    }

    KindTemplataT(KindPlaceholderT(kindPlaceholderId))
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn create_non_kind_non_region_placeholder_inner(
        &self,
        name_prefix: IdT<'s, 't>,
        index: i32,
        rune: IRuneS<'s>,
        tyype: ITemplataType<'s>,
    ) -> ITemplataT<'s, 't> {
        // val idT = namePrefix.addStep(interner.intern(NonKindNonRegionPlaceholderNameT(index, rune)))
        let placeholder_name = self.typing_interner.intern_non_kind_non_region_placeholder_name(
            NonKindNonRegionPlaceholderNameT { index, rune, _phantom: std::marker::PhantomData }
        );
        let id_t = name_prefix.add_step(
            self.typing_interner,
            INameT::NonKindNonRegionPlaceholder(placeholder_name),
        );
        // PlaceholderTemplataT(idT, tyype)
        ITemplataT::Placeholder(self.typing_interner.alloc(PlaceholderTemplataT {
            id: *id_t,
            tyype,
        }))
    }
/*
  def createNonKindNonRegionPlaceholderInner[T <: ITemplataType](
      namePrefix: IdT[INameT],
      index: Int,
      rune: IRuneS,
      tyype: T):
  PlaceholderTemplataT[T] = {
    val idT = namePrefix.addStep(interner.intern(NonKindNonRegionPlaceholderNameT(index, rune)))
    PlaceholderTemplataT(idT, tyype)
  }
}
*/
}
