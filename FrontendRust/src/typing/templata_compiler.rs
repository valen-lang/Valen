use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::ast::*;
use crate::typing::env::environment::*;
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::typing_interner::{MustIntern, TypingInterner};
use crate::keywords::Keywords;
use crate::typing::hinputs_t::{InstantiationBoundArgumentsT, InstantiationReachableBoundArgumentsT};
use crate::postparsing::names::{IRuneS, IImpreciseNameS};
use crate::postparsing::ast::{GenericParameterS, IRegionMutabilityS, LocationInDenizen};
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::rules::rules::{EqualsSR, IRulexSR, RuneUsage};
use crate::typing::infer_compiler::include_rule_in_call_site_solve;
use crate::postparsing::rune_type_solver::IRuneTypeSolverEnv;
use crate::utils::range::RangeS;
use std::collections::HashMap;
use crate::typing::types::types::{KindPlaceholderT, KindT};
use crate::typing::names::names::IInstantiationNameT;
use crate::typing::names::names::{ISuperKindNameT, ITemplateNameT};
use crate::typing::names::names::{INameValT, IdValT};
use crate::typing::names::names::StructNameValT;
use crate::typing::names::names::INameT;
use crate::typing::types::types::StructTTValT;
use crate::typing::names::names::FunctionBoundNameT;
use crate::typing::names::names::ImplBoundNameT;
use crate::typing::names::names::InterfaceNameValT;
use crate::typing::types::types::InterfaceTTValT;
use crate::typing::names::names::IPlaceholderNameT;
use crate::typing::names::names::IFunctionNameT;
use crate::typing::ast::ast::PrototypeValT;
use crate::typing::citizen::impl_compiler::IsParentResult;
use crate::postparsing::itemplatatype::KindTemplataType;
use crate::postparsing::itemplatatype::CoordTemplataType;
use crate::postparsing::ast::IGenericParameterTypeS;
use crate::postparsing::ast::CoordGenericParameterTypeS;
use crate::scout_arena::ScoutArena;
use crate::postparsing::rune_type_solver::IRuneTypeSolverLookupResult;
use crate::postparsing::rune_type_solver::IRuneTypingLookupFailedError;
use crate::postparsing::rune_type_solver::TemplataLookupResult;
use crate::typing::env::environment::ILookupContext;
use crate::typing::templata::templata::ITemplataT;
use crate::postparsing::rune_type_solver::CitizenRuneTypeSolverLookupResult;
use crate::postparsing::rune_type_solver::RuneTypingCouldntFindType;
use std::collections::HashSet;
use std::iter::empty;
use std::marker::PhantomData;

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
            IInstantiationNameT::try_from(*name).is_ok()
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
    pub fn get_placeholder_templata_id(
        impl_placeholder: ITemplataT<'s, 't>,
    ) -> IdT<'s, 't> {
        match impl_placeholder {
            ITemplataT::Placeholder(pt) => pt.id,
            ITemplataT::Kind(kt) => match kt.kind {
                KindT::KindPlaceholder(kp) => kp.id,
                _ => panic!("vwat: get_placeholder_templata_id unexpected kind: {:?}", kt.kind),
            },
            ITemplataT::Coord(ct) => match ct.coord.kind {
                KindT::KindPlaceholder(kp) => kp.id,
                _ => panic!("vwat: get_placeholder_templata_id unexpected coord kind: {:?}", ct.coord.kind),
            },
            other => panic!("vwat: get_placeholder_templata_id unexpected templata: {:?}", other),
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
    // See SFWPRL. Per @DRSINI, this is the only place that eagerly adds default rules.
    // Safe because prediction has no actual arguments being inferred that could conflict.
    pub fn assemble_predict_rules(
        &self,
        generic_parameters: &'s [&'s GenericParameterS<'s>],
        num_explicit_template_args: i32,
    ) -> Vec<IRulexSR<'s>> {
        let mut result: Vec<IRulexSR<'s>> = Vec::new();
        for (index, generic_param) in generic_parameters.iter().enumerate() {
            if (index as i32) >= num_explicit_template_args {
                match &generic_param.default {
                    Some(x) => {
                        for rule in x.rules.iter() {
                            result.push(**rule);
                        }
                    }
                    None => {}
                }
            }
        }
        result
    }
/*
  // See SFWPRL. Per @DRSINI, this is the only place that eagerly adds default rules.
  // Safe because prediction has no actual arguments being inferred that could conflict.
  def assemblePredictRules(genericParameters: Vector[GenericParameterS], numExplicitTemplateArgs: Int): Vector[IRulexSR] = {
    genericParameters.zipWithIndex.flatMap({ case (genericParam, index) =>
      if (index >= numExplicitTemplateArgs) {
        genericParam.default match {
          case Some(x) => x.rules
          case None => Vector()
        }
      } else {
        Vector()
      }
    })
  }
*/
    // Per @DRSINI, default rules are no longer added eagerly here. They're added
    // incrementally by solveForResolving and evaluateGenericFunctionFromCallForPrototype
    // only for runes that remain unsolved after argument inference.
    pub fn assemble_call_site_rules(
        &self,
        rules: &'s [IRulexSR<'s>],
    ) -> Vec<IRulexSR<'s>> {
        rules.iter().copied().filter(|r| include_rule_in_call_site_solve(r)).collect()
    }
/*
  // Per @DRSINI, default rules are no longer added eagerly here. They're added
  // incrementally by solveForResolving and evaluateGenericFunctionFromCallForPrototype
  // only for runes that remain unsolved after argument inference.
  def assembleCallSiteRules(rules: Vector[IRulexSR]): Vector[IRulexSR] = {
    rules.filter(InferCompiler.includeRuleInCallSiteSolve)
  }
*/
    pub fn get_function_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let func_name = IFunctionNameT::try_from(id.local_name)
            .unwrap_or_else(|_| panic!("get_function_template: not a function name: {:?}", id.local_name));
        let template_local: INameT<'s, 't> = ITemplateNameT::from(func_name.template()).into();
        *interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: template_local,
        })
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
            INameT::AnonymousSubstruct(a) => INameT::AnonymousSubstructTemplate(a.template),
            _ => panic!("get_citizen_template called with non-citizen name: {:?}", id.local_name),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name,
        })
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
    // Rust adaptation: associated fn (no &self) — Scala's TemplataCompiler.getNameTemplate is a companion-object static.
    pub fn get_name_template(
        name: INameT<'s, 't>,
    ) -> INameT<'s, 't> {
        match IInstantiationNameT::try_from(name) {
            Ok(x) => INameT::from(x.template()),
            Err(_) => name,
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
    // Rust adaptation: associated fn (no &self) — Scala's TemplataCompiler.getSuperTemplate is a companion-object static.
    pub fn get_super_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let new_init_steps: Vec<INameT<'s, 't>> =
            id.init_steps.iter().map(|n| Self::get_name_template(*n)).collect();
        let new_local_name = Self::get_name_template(id.local_name);
        *interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: &new_init_steps,
            local_name: new_local_name,
        })
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
    // Rust adaptation: associated fn (no &self) — Scala's TemplataCompiler.getRootSuperTemplate is a companion-object static.
    pub fn get_root_super_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let mut tentative_id = Self::get_super_template(interner, id);
        loop {
            let contains_lambda = tentative_id.init_steps.iter().any(|n| {
                match n {
                    INameT::LambdaCitizenTemplate(_) => true,
                    INameT::LambdaCallFunctionTemplate(_) => true,
                    INameT::OverrideDispatcherCase(_) => true,
                    _ => false,
                }
            }) || match tentative_id.local_name {
                INameT::LambdaCitizenTemplate(_) => true,
                INameT::LambdaCallFunctionTemplate(_) => true,
                INameT::OverrideDispatcherCase(_) => true,
                _ => false,
            };
            if contains_lambda {
                tentative_id = tentative_id.init_id(interner);
            } else {
                return tentative_id;
            }
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
    pub fn get_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> &'t IdT<'s, 't> {
        let last = IInstantiationNameT::try_from(id.local_name).unwrap();
        interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps, //.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
            local_name: INameT::from(last.template()),
        })
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
    pub fn get_sub_kind_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let last = IInstantiationNameT::try_from(id.local_name)
            .unwrap_or_else(|_| panic!("get_sub_kind_template: unexpected local_name {:?}", id.local_name));
        let template_name = INameT::from(last.template());
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: template_name,
        })
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
    pub fn get_super_kind_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let last = ISuperKindNameT::try_from(id.local_name)
            .unwrap_or_else(|_| panic!("get_super_kind_template: unexpected local_name {:?}", id.local_name));
        let template_name = INameT::from(ITemplateNameT::from(last.template()));
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: template_name,
        })
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
            INameT::AnonymousSubstruct(a) => INameT::AnonymousSubstructTemplate(a.template),
            _ => panic!("get_struct_template called with non-struct name: {:?}", id.local_name),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name,
        })
    }
/*
  def getStructTemplate(id: IdT[IStructNameT]): IdT[IStructTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
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
/*
  def getInterfaceTemplate(id: IdT[IInterfaceNameT]): IdT[IInterfaceTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
    pub fn get_export_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
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
    pub fn get_extern_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
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
    pub fn get_impl_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let IdT { package_coord, init_steps, local_name, .. } = id;
        let impl_name = IImplNameT::try_from(local_name).expect("get_impl_template: not an impl name");
        let template = INameT::from(impl_name.template());
        *interner.intern_id(crate::typing::names::names::IdValT { package_coord, init_steps, local_name: template })
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
/*
  def getPlaceholderTemplate(id: IdT[KindPlaceholderNameT]): IdT[KindPlaceholderTemplateNameT] = {
    val IdT(packageCoord, initSteps, last) = id
    IdT(
      packageCoord,
      initSteps,//.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
      last.template)
  }
*/
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
                            result.insert(rune_name.rune, proto_templata.prototype);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        result
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
    pub fn assemble_rune_to_impl_bound(
        &self,
        templatas: &'t TemplatasStoreT<'s, 't>,
    ) -> HashMap<IRuneS<'s>, IdT<'s, 't>> {
        let mut result = HashMap::new();
        for (name, entry) in templatas.name_to_entry.iter() {
            match (name, entry) {
                (INameT::Rune(rune_name), IEnvEntryT::Templata(ITemplataT::Isa(isa))) => {
                    match isa.impl_name.local_name {
                        INameT::ImplBound(_) => {
                            result.insert(rune_name.rune, isa.impl_name);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        result
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
    pub fn substitute_templatas_in_coord(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        coord: CoordT<'s, 't>,
    ) -> CoordT<'s, 't> {
        let CoordT { ownership, region: original_region, kind } = coord;
        let result_region = original_region;
        match Compiler::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, kind) {
            ITemplataT::Kind(k) => CoordT { ownership, region: result_region, kind: k.kind },
            ITemplataT::Coord(c) => {
                let result_ownership = match (ownership, c.coord.ownership) {
                    (OwnershipT::Share, _) => OwnershipT::Share,
                    (_, OwnershipT::Share) => OwnershipT::Share,
                    (OwnershipT::Own, OwnershipT::Own) => OwnershipT::Own,
                    (OwnershipT::Own, OwnershipT::Borrow) => OwnershipT::Borrow,
                    (OwnershipT::Borrow, OwnershipT::Own) => OwnershipT::Borrow,
                    (OwnershipT::Borrow, OwnershipT::Borrow) => OwnershipT::Borrow,
                    _ => panic!("vimpl: unexpected ownership combination in substitute_templatas_in_coord"),
                };
                CoordT { ownership: result_ownership, region: result_region, kind: c.coord.kind }
            }
            _ => panic!("Unimplemented: substitute_templatas_in_coord unexpected templata result"),
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
    pub fn substitute_templatas_in_kind(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        kind: KindT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        match kind {
            KindT::Int(_) => ITemplataT::Kind(interner.alloc(KindTemplataT { kind })),
            KindT::Bool(_) => ITemplataT::Kind(interner.alloc(KindTemplataT { kind })),
            KindT::Str(_) => ITemplataT::Kind(interner.alloc(KindTemplataT { kind })),
            KindT::Float(_) => ITemplataT::Kind(interner.alloc(KindTemplataT { kind })),
            KindT::Void(_) => ITemplataT::Kind(interner.alloc(KindTemplataT { kind })),
            KindT::Never(_) => ITemplataT::Kind(interner.alloc(KindTemplataT { kind })),
            KindT::RuntimeSizedArray(rsa) => {
                let INameT::RuntimeSizedArray(rsa_name) = rsa.name.local_name else { panic!("vwat") };
                let new_arr_name = interner.intern_raw_array_name(RawArrayNameT {
                    mutability: expect_mutability(Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, rsa_name.arr.mutability)),
                    element_type: Self::substitute_templatas_in_coord(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, rsa_name.arr.element_type),
                    self_region: RegionT { region: IRegionT::Default },
                });
                let new_rsa_name = interner.intern_runtime_sized_array_name(RuntimeSizedArrayNameT {
                    template: rsa_name.template,
                    arr: new_arr_name,
                });
                let new_id = *interner.intern_id(IdValT {
                    package_coord: rsa.name.package_coord,
                    init_steps: rsa.name.init_steps,
                    local_name: INameT::RuntimeSizedArray(new_rsa_name),
                });
                let new_rsa = interner.intern_runtime_sized_array_tt(RuntimeSizedArrayTTValT { name: new_id });
                ITemplataT::Kind(interner.alloc(KindTemplataT { kind: KindT::RuntimeSizedArray(new_rsa) }))
            }
            KindT::StaticSizedArray(ssa) => {
                let INameT::StaticSizedArray(ssa_name) = ssa.name.local_name else { panic!("vwat") };
                let new_arr_name = interner.intern_raw_array_name(RawArrayNameT {
                    mutability: expect_mutability(Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, ssa_name.arr.mutability)),
                    element_type: Self::substitute_templatas_in_coord(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, ssa_name.arr.element_type),
                    self_region: RegionT { region: IRegionT::Default },
                });
                let new_ssa_name = interner.intern_static_sized_array_name(StaticSizedArrayNameT {
                    template: ssa_name.template,
                    size: expect_integer(Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, ssa_name.size)),
                    variability: expect_variability(Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, ssa_name.variability)),
                    arr: new_arr_name,
                });
                let new_id = *interner.intern_id(IdValT {
                    package_coord: ssa.name.package_coord,
                    init_steps: ssa.name.init_steps,
                    local_name: INameT::StaticSizedArray(new_ssa_name),
                });
                let new_ssa = interner.intern_static_sized_array_tt(StaticSizedArrayTTValT { name: new_id });
                ITemplataT::Kind(interner.alloc(KindTemplataT { kind: KindT::StaticSizedArray(new_ssa) }))
            }
            KindT::KindPlaceholder(p) => {
                let index = match p.id.local_name {
                    INameT::KindPlaceholder(kp) => kp.template.index,
                    _ => panic!("KindPlaceholderT has non-KindPlaceholder local_name"),
                };
                if p.id.init_id(interner) == needle_template_name {
                    new_substituting_templatas[index as usize]
                } else {
                    ITemplataT::Kind(interner.alloc(KindTemplataT { kind }))
                }
            }
            KindT::Struct(s) => {
                let new_struct = Compiler::substitute_templatas_in_struct(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, s);
                ITemplataT::Kind(interner.alloc(KindTemplataT { kind: KindT::Struct(new_struct) }))
            }
            KindT::Interface(i) => {
                let new_interface = Compiler::substitute_templatas_in_interface(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, i);
                ITemplataT::Kind(interner.alloc(KindTemplataT { kind: KindT::Interface(new_interface) }))
            }
            KindT::OverloadSet(_) => panic!("Unimplemented: substitute_templatas_in_kind OverloadSet"),
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
                  RegionT(DefaultRegionT)))))))))
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
                  RegionT(DefaultRegionT)))))))))
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
    pub fn substitute_templatas_in_struct(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        struct_tt: &'t StructTT<'s, 't>,
    ) -> &'t StructTT<'s, 't> {
        let id = struct_tt.id;
        let new_local_name = match id.local_name {
            INameT::AnonymousSubstruct(asub_name_t) => {
                let new_template_args: Vec<ITemplataT<'s, 't>> = asub_name_t.template_args.iter()
                    .map(|templata| Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *templata))
                    .collect();
                let new_template_args_ref = interner.alloc_slice_from_vec(new_template_args);
                interner.intern_name(INameValT::AnonymousSubstruct(AnonymousSubstructNameValT {
                    template: asub_name_t.template,
                    template_args: new_template_args_ref,
                }))
            }
            INameT::Struct(struct_name_t) => {
                let new_template_args: Vec<ITemplataT<'s, 't>> = struct_name_t.template_args.iter()
                    .map(|templata| Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *templata))
                    .collect();
                let new_template_args_ref = interner.alloc_slice_from_vec(new_template_args);
                interner.intern_name(INameValT::Struct(StructNameValT {
                    template: struct_name_t.template,
                    template_args: new_template_args_ref,
                }))
            }
            INameT::LambdaCitizen(lambda_citizen_name_t) => {
                INameT::LambdaCitizen(lambda_citizen_name_t)
            }
            _ => panic!("implement: substituteTemplatasInStruct — unexpected local_name kind"),
        };
        let new_id = interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: new_local_name,
        });
        let new_struct = interner.intern_struct_tt(StructTTValT { id: *new_id });
        // See SBITAFD, we need to register bounds for these new instantiations.
        let instantiation_bound_args = coutputs.get_instantiation_bounds(interner, struct_tt.id).unwrap();
        let translated_bounds = interner.alloc(Self::translate_instantiation_bounds(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, instantiation_bound_args));
        coutputs.add_instantiation_bounds(
            sanity_check, interner,
            original_calling_denizen_id,
            new_struct.id,
            translated_bounds);
        new_struct
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
    pub fn translate_instantiation_bounds(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        instantiation_bound_args: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> InstantiationBoundArgumentsT<'s, 't> {
        match bound_arguments_source {
            IBoundArgumentsSource::InheritBoundsFromTypeItself => {
                let x = Self::substitute_templatas_in_bounds(
                    coutputs, sanity_check, interner, keywords,
                    original_calling_denizen_id, needle_template_name,
                    new_substituting_templatas, bound_arguments_source,
                    instantiation_bound_args);
                x
            }
            IBoundArgumentsSource::UseBoundsFromContainer { instantiation_bound_params: container_instantiation_bound_params, instantiation_bound_arguments: container_instantiation_bound_args } => {
                let container_func_bound_to_bound_arg: HashMap<PrototypeT<'s, 't>, PrototypeT<'s, 't>> =
                    container_instantiation_bound_args.rune_to_bound_prototype.iter()
                        .map(|(rune, container_func_bound_arg)| {
                            let param_proto = *container_instantiation_bound_params.rune_to_bound_prototype.get(rune).unwrap();
                            (param_proto, *container_func_bound_arg)
                        })
                        .collect();
                let container_impl_bound_to_bound_arg: HashMap<IdT<'s, 't>, IdT<'s, 't>> =
                    container_instantiation_bound_args.rune_to_bound_impl.iter()
                        .map(|(rune, container_impl_bound_arg)| {
                            let param_impl = *container_instantiation_bound_params.rune_to_bound_impl.get(rune).unwrap();
                            (param_impl, *container_impl_bound_arg)
                        })
                        .collect();
                let rune_to_bound_prototype = interner.alloc_index_map_from_iter(
                    instantiation_bound_args.rune_to_bound_prototype.iter().map(|(rune, func_bound_arg)| {
                        let new_val = match func_bound_arg.id.local_name {
                            INameT::FunctionBound(_) => {
                                *container_func_bound_to_bound_arg.get(func_bound_arg).unwrap()
                            }
                            _ => {
                                // Not sure if this call is really necessary...
                                *Self::substitute_templatas_in_prototype(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, func_bound_arg)
                            }
                        };
                        (*rune, new_val)
                    }));
                let rune_to_citizen_rune_to_reachable_prototype = interner.alloc_index_map_from_iter(
                    instantiation_bound_args.rune_to_citizen_rune_to_reachable_prototype.iter().map(|(callee_rune, reachable_bound_args)| {
                        let new_citizen = interner.alloc_index_map_from_iter(
                            reachable_bound_args.citizen_rune_to_reachable_prototype.iter().map(|(citizen_rune, reachable_prototype)| {
                                let new_val = match reachable_prototype.id.local_name {
                                    INameT::FunctionBound(_) => {
                                        *container_func_bound_to_bound_arg.get(reachable_prototype).unwrap()
                                    }
                                    _ => {
                                        // Not sure if this call is really necessary...
                                        *Self::substitute_templatas_in_prototype(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, reachable_prototype)
                                    }
                                };
                                (*citizen_rune, new_val)
                            }));
                        let new_reachable: &'t InstantiationReachableBoundArgumentsT<'s, 't> = interner.alloc(InstantiationReachableBoundArgumentsT { citizen_rune_to_reachable_prototype: new_citizen });
                        (*callee_rune, new_reachable)
                    }));
                let rune_to_bound_impl = interner.alloc_index_map_from_iter(
                    instantiation_bound_args.rune_to_bound_impl.iter().map(|(rune, impl_bound_arg)| {
                        let new_val = match impl_bound_arg.local_name {
                            INameT::ImplBound(_) => {
                                *container_impl_bound_to_bound_arg.get(impl_bound_arg).unwrap()
                            }
                            _ => {
                                // Not sure if this call is really necessary...
                                Self::substitute_templatas_in_impl_id(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *impl_bound_arg)
                            }
                        };
                        (*rune, new_val)
                    }));
                InstantiationBoundArgumentsT {
                    rune_to_bound_prototype,
                    rune_to_citizen_rune_to_reachable_prototype,
                    rune_to_bound_impl,
                }
            }
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
    pub fn substitute_templatas_in_impl_id(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        impl_id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
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
    pub fn substitute_templatas_in_bounds(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        bound_args: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> InstantiationBoundArgumentsT<'s, 't> {
        let rune_to_bound_prototype = interner.alloc_index_map_from_iter(
            bound_args.rune_to_bound_prototype.iter().map(|(rune, func_bound_arg)| {
                (*rune, *Self::substitute_templatas_in_prototype(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, func_bound_arg))
            }));
        let rune_to_citizen_rune_to_reachable_prototype = interner.alloc_index_map_from_iter(
            bound_args.rune_to_citizen_rune_to_reachable_prototype.iter().map(|(caller_rune, reachable_bound_args)| {
                let new_citizen_rune_to_reachable_prototype = interner.alloc_index_map_from_iter(
                    reachable_bound_args.citizen_rune_to_reachable_prototype.iter().map(|(citizen_rune, reachable_prototype)| {
                        (*citizen_rune, *Self::substitute_templatas_in_prototype(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, reachable_prototype))
                    }));
                let new_reachable: &'t InstantiationReachableBoundArgumentsT<'s, 't> = interner.alloc(InstantiationReachableBoundArgumentsT { citizen_rune_to_reachable_prototype: new_citizen_rune_to_reachable_prototype });
                (*caller_rune, new_reachable)
            }));
        let rune_to_bound_impl = interner.alloc_index_map_from_iter(
            bound_args.rune_to_bound_impl.iter().map(|(rune, impl_bound_arg)| {
                (*rune, Self::substitute_templatas_in_impl_id(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *impl_bound_arg))
            }));
        InstantiationBoundArgumentsT {
            rune_to_bound_prototype,
            rune_to_citizen_rune_to_reachable_prototype,
            rune_to_bound_impl,
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
    pub fn substitute_templatas_in_interface(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        interface_tt: &'t InterfaceTT<'s, 't>,
    ) -> &'t InterfaceTT<'s, 't> {
        let id = interface_tt.id;
        let new_local_name = match id.local_name {
            INameT::Interface(interface_name_t) => {
                let new_template_args: Vec<ITemplataT<'s, 't>> = interface_name_t.template_args.iter()
                    .map(|templata| Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *templata))
                    .collect();
                let new_template_args_ref = interner.alloc_slice_from_vec(new_template_args);
                interner.intern_name(INameValT::Interface(InterfaceNameValT {
                    template: interface_name_t.template,
                    template_args: new_template_args_ref,
                }))
            }
            _ => panic!("implement: substituteTemplatasInInterface — unexpected local_name kind"),
        };
        let new_id = interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: new_local_name,
        });
        let new_interface = interner.intern_interface_tt(InterfaceTTValT { id: *new_id });
        // See SBITAFD, we need to register bounds for these new instantiations.
        let instantiation_bound_args = coutputs.get_instantiation_bounds(interner, interface_tt.id).unwrap();
        let translated_bounds = interner.alloc(Self::translate_instantiation_bounds(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, instantiation_bound_args));
        coutputs.add_instantiation_bounds(
            sanity_check, interner,
            original_calling_denizen_id,
            new_interface.id,
            translated_bounds);
        new_interface
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
    pub fn substitute_templatas_in_templata(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        templata: ITemplataT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        match templata {
            ITemplataT::Coord(c) => ITemplataT::Coord(interner.alloc(CoordTemplataT { coord: Compiler::substitute_templatas_in_coord(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, c.coord) })),
            ITemplataT::Kind(k) => Compiler::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, k.kind),
            ITemplataT::Placeholder(p) => {
                let pn = IPlaceholderNameT::try_from(p.id.local_name).unwrap();
                if p.id.init_id(interner) == needle_template_name {
                    new_substituting_templatas[pn.index() as usize]
                } else {
                    templata
                }
            }
            ITemplataT::Mutability(_) => templata,
            ITemplataT::Variability(_) => templata,
            ITemplataT::Integer(_) => templata,
            ITemplataT::Boolean(_) => templata,
            ITemplataT::Prototype(p) => {
                panic!("Unimplemented: substitute_templatas_in_templata Prototype");
            }
            _ => panic!("vimpl: substitute_templatas_in_templata unexpected templata"),
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
    pub fn substitute_templatas_in_prototype(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        original_prototype: &'t PrototypeT<'s, 't>,
    ) -> &'t PrototypeT<'s, 't> {
        let package_coord = original_prototype.id.package_coord;
        let init_steps = original_prototype.id.init_steps;
        let func_name = IFunctionNameT::try_from(original_prototype.id.local_name).unwrap();
        let substituted_template_args_vec: Vec<ITemplataT<'s, 't>> = func_name.template_args().iter().map(|templata| {
            Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *templata)
        }).collect();
        let substituted_template_args = interner.alloc_slice_from_vec(substituted_template_args_vec);
        let substituted_params_vec: Vec<CoordT<'s, 't>> = func_name.parameters().iter().map(|coord| {
            Self::substitute_templatas_in_coord(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *coord)
        }).collect();
        let substituted_params = interner.alloc_slice_from_vec(substituted_params_vec);
        let substituted_return_type = Self::substitute_templatas_in_coord(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, original_prototype.return_type);
        let substituted_func_name = func_name.template().make_function_name(interner, keywords, substituted_template_args, substituted_params);
        let tentative_id = *interner.intern_id(IdValT { package_coord, init_steps, local_name: substituted_func_name });
        let perhaps_imported_id = match tentative_id.local_name {
            INameT::FunctionBound(n) => {
                // Always import a seen function bound into our own environment, see MFBFDP.
                let imported_id = *original_calling_denizen_id.add_step(interner, INameT::FunctionBound(n));
                // It's a function bound, it has no function bounds of its own.
                coutputs.add_instantiation_bounds(
                    sanity_check,
                    interner,
                    original_calling_denizen_id,
                    imported_id,
                    interner.alloc(InstantiationBoundArgumentsT {
                        rune_to_bound_prototype: interner.alloc_index_map_from_iter(empty()),
                        rune_to_citizen_rune_to_reachable_prototype: interner.alloc_index_map_from_iter(empty()),
                        rune_to_bound_impl: interner.alloc_index_map_from_iter(empty()),
                    }),
                );
                imported_id
            }
            _ => {
                // Not really sure if we're supposed to add bounds or something here.
                assert!(coutputs.get_instantiation_bounds(interner, tentative_id).is_some());
                tentative_id
            }
        };
        // Rust adaptation: Scala had vassert(substitutedFuncName.getClass.equals(funcName.getClass))
        // and vassert(originalPrototype.getClass.equals(prototype.getClass)) to guard the cast-back
        // to T. Rust has no generic T to cast back to, so these class-equality asserts are omitted.
        interner.intern_prototype(PrototypeValT {
            id: IdValT { package_coord: perhaps_imported_id.package_coord, init_steps: perhaps_imported_id.init_steps, local_name: perhaps_imported_id.local_name },
            return_type: substituted_return_type,
        })
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
    pub fn substitute_templatas_in_function_bound_id(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
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
// Slab 14 Gotcha 9 (single-implementor trait → struct with inherent methods). The seven fields below mirror
// Scala's anonymous-trait-impl closure captures at TemplataCompiler.scala:808-824 (sanityCheck, interner,
// keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource).
pub struct IPlaceholderSubstituter<'s, 'ctx, 't> {
    pub sanity_check: bool,
    pub interner: &'ctx TypingInterner<'s, 't>,
    pub keywords: &'ctx Keywords<'s>,
    pub original_calling_denizen_id: IdT<'s, 't>,
    pub needle_template_name: IdT<'s, 't>,
    pub new_substituting_templatas: &'t [ITemplataT<'s, 't>],
    pub bound_arguments_source: IBoundArgumentsSource<'s, 't>,
}
// Per TL.md "Guardian Annotations For New Definitions Without Scala Counterparts" and the
// LetExprRuneTypeSolverEnv / OverloadRuneTypeSolverEnv precedent (Slab 15f): the methods below realize
// Scala's anonymous `new IPlaceholderSubstituter { override def ... }` block at TemplataCompiler.scala:808-824.
// The Scala bodies live inside getPlaceholderSubstituter (later in the file) so direct adjacency isn't possible
// here — Guardian shields disabled on the impl methods.
impl<'s, 'ctx, 't> IPlaceholderSubstituter<'s, 'ctx, 't> {
    /* Guardian: disable-all */
    pub fn substitute_for_coord(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        coord_t: CoordT<'s, 't>,
    ) -> CoordT<'s, 't> {
        Compiler::substitute_templatas_in_coord(
            coutputs,
            self.sanity_check,
            self.interner,
            self.keywords,
            self.original_calling_denizen_id,
            self.needle_template_name,
            self.new_substituting_templatas,
            self.bound_arguments_source,
            coord_t,
        )
    }
    /* Guardian: disable-all */
    pub fn substitute_for_interface(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        interface_tt: InterfaceTT<'s, 't>,
    ) -> InterfaceTT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    /* Guardian: disable-all */
    pub fn substitute_for_templata(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        templata: ITemplataT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        Compiler::substitute_templatas_in_templata(
            coutputs,
            self.sanity_check,
            self.interner,
            self.keywords,
            self.original_calling_denizen_id,
            self.needle_template_name,
            self.new_substituting_templatas,
            self.bound_arguments_source,
            templata,
        )
    }
    /* Guardian: disable-all */
    pub fn substitute_for_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        proto: &'t PrototypeT<'s, 't>,
    ) -> &'t PrototypeT<'s, 't> {
        Compiler::substitute_templatas_in_prototype(
            coutputs,
            self.sanity_check,
            self.interner,
            self.keywords,
            self.original_calling_denizen_id,
            self.needle_template_name,
            self.new_substituting_templatas,
            self.bound_arguments_source,
            proto,
        )
    }
    /* Guardian: disable-all */
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
    ) -> IPlaceholderSubstituter<'s, 'ctx, 't> {
        let top_level_denizen_id = self.get_top_level_denizen_id(name);
        let top_level_local_name: IInstantiationNameT<'s, 't> =
            top_level_denizen_id.local_name.try_into()
                .unwrap_or_else(|_| panic!("get_placeholder_substituter: topLevelDenizenId.localName must be IInstantiationNameT, got {:?}", top_level_denizen_id.local_name));
        let template_args: &[ITemplataT<'s, 't>] = top_level_local_name.template_args();
        let top_level_denizen_template_id = Compiler::get_template(self.typing_interner, top_level_denizen_id);
        self.get_placeholder_substituter_ext(
            sanity_check,
            original_calling_denizen_id,
            *top_level_denizen_template_id,
            template_args,
            bound_arguments_source,
        )
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
    pub fn get_placeholder_substituter_ext(
        &self,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &'t [ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
    ) -> IPlaceholderSubstituter<'s, 'ctx, 't> {
        IPlaceholderSubstituter {
            sanity_check,
            interner: self.typing_interner,
            keywords: self.keywords,
            original_calling_denizen_id,
            needle_template_name,
            new_substituting_templatas,
            bound_arguments_source,
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
    pub fn get_reachable_bounds(
        &self,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        citizen: ICitizenTT<'s, 't>,
    ) -> InstantiationReachableBoundArgumentsT<'s, 't> {
        let citizen_id = match citizen {
            ICitizenTT::Struct(s) => s.id,
            ICitizenTT::Interface(i) => i.id,
        };
        let substituter =
            self.get_placeholder_substituter(
                sanity_check,
                original_calling_denizen_id,
                citizen_id,
                IBoundArgumentsSource::InheritBoundsFromTypeItself,
            );
        let citizen_template_id = self.get_citizen_template(citizen_id);
        let inner_env = coutputs.get_inner_env_for_type(citizen_template_id);
        let citizen_rune_to_reachable_prototype: Vec<(IRuneS<'s>, PrototypeT<'s, 't>)> =
            inner_env.templatas().name_to_entry.iter()
                .filter_map(|(name, entry)| {
                    match (name, entry) {
                        (INameT::Rune(rune_name), IEnvEntryT::Templata(ITemplataT::Prototype(proto_tt))) => {
                            match proto_tt.prototype.id.local_name {
                                INameT::FunctionBound(_) => {
                                    let substituted = substituter.substitute_for_prototype(coutputs, proto_tt.prototype);
                                    Some((rune_name.rune, *substituted))
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                })
                .collect();
        InstantiationReachableBoundArgumentsT {
            citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map_from_iter(
                citizen_rune_to_reachable_prototype.into_iter()),
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
    pub fn get_first_unsolved_identifying_rune(
        &self,
        generic_parameters: &'s [&'s GenericParameterS<'s>],
        is_solved: impl Fn(IRuneS<'s>) -> bool,
    ) -> Option<(&'s GenericParameterS<'s>, i32)> {
        generic_parameters.iter().enumerate()
            .map(|(index, generic_param)| (generic_param, index as i32, is_solved(generic_param.rune.rune)))
            .filter(|(_, _, solved)| !solved)
            .map(|(generic_param, index, _)| (*generic_param, index))
            .next()
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
    pub fn create_rune_type_solver_env(
        &self,
        parent_env: IInDenizenEnvironmentT<'s, 't>,
    ) -> TemplataCompilerRuneTypeSolverEnv<'_, 's, 't> {
        TemplataCompilerRuneTypeSolverEnv {
            parent_env,
            typing_interner: self.typing_interner,
            scout_arena: self.scout_arena,
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
pub struct TemplataCompilerRuneTypeSolverEnv<'a, 's, 't>
where
    's: 't,
{
    parent_env: IInDenizenEnvironmentT<'s, 't>,
    typing_interner: &'a TypingInterner<'s, 't>,
    scout_arena: &'a ScoutArena<'s>,
}
/*
Guardian: disable-all
*/

impl<'a, 's, 't> IRuneTypeSolverEnv<'s>
for TemplataCompilerRuneTypeSolverEnv<'a, 's, 't>
where
    's: 't,
{
    fn lookup(
        &self,
        range: RangeS<'s>,
        name_s: IImpreciseNameS<'s>,
    ) -> Result<
        IRuneTypeSolverLookupResult<'s>,
        IRuneTypingLookupFailedError<'s>,
    > {
        match name_s {
            IImpreciseNameS::LambdaStructImpreciseName(_) => {
                // Scala: vregionmut() // Take out with regions
                // Lambdas look up their struct as a KindTemplata in their environment, they don't
                // look up the origin template by name. (Scala comment from astronomizeLambda.)
                Ok(IRuneTypeSolverLookupResult::Templata(
                    TemplataLookupResult {
                        templata: ITemplataType::KindTemplataType(
                            KindTemplataType {},
                        ),
                    },
                ))
            }
            _ => {
                let mut filter = HashSet::new();
                filter.insert(ILookupContext::TemplataLookupContext);
                match self.parent_env.lookup_nearest_with_imprecise_name(name_s, filter, self.typing_interner) {
                    Some(ITemplataT::StructDefinition(t)) => {
                        Ok(IRuneTypeSolverLookupResult::Citizen(
                            CitizenRuneTypeSolverLookupResult {
                                tyype: ITemplataType::TemplateTemplataType(
                                    t.origin_struct.tyype,
                                ),
                                generic_params: t.origin_struct.generic_parameters,
                            },
                        ))
                    }
                    Some(ITemplataT::InterfaceDefinition(t)) => {
                        Ok(IRuneTypeSolverLookupResult::Citizen(
                            CitizenRuneTypeSolverLookupResult {
                                tyype: ITemplataType::TemplateTemplataType(
                                    t.origin_interface.tyype,
                                ),
                                generic_params: t.origin_interface.generic_parameters,
                            },
                        ))
                    }
                    Some(x) => {
                        Ok(IRuneTypeSolverLookupResult::Templata(
                            TemplataLookupResult {
                                templata: x.tyype(self.scout_arena),
                            },
                        ))
                    }
                    None => Err(
                        IRuneTypingLookupFailedError::CouldntFindType(
                            RuneTypingCouldntFindType {
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
        calling_env: IInDenizenEnvironmentT<'s, 't>,
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
            (KindT::Void(_) | KindT::Int(_) | KindT::Bool(_) | KindT::Str(_) | KindT::Float(_)
                | KindT::RuntimeSizedArray(_) | KindT::StaticSizedArray(_), _) => {
                return false;
            }
            (_, KindT::Void(_) | KindT::Int(_) | KindT::Bool(_) | KindT::Str(_) | KindT::Float(_)
                | KindT::RuntimeSizedArray(_) | KindT::StaticSizedArray(_)) => {
                return false;
            }
            (_, KindT::Struct(_)) => return false,
            (a, b) if ISubKindTT::try_from(*a).is_ok() && ISuperKindTT::try_from(*b).is_ok() => {
                let source_sub_kind = ISubKindTT::try_from(source_type).unwrap();
                let target_super_kind = ISuperKindTT::try_from(target_type).unwrap();
                match self.is_parent(coutputs, calling_env, parent_ranges, call_location, source_sub_kind, target_super_kind) {
                    IsParentResult::IsParent(_) => {}
                    IsParentResult::IsntParent(_) => return false,
                }
            }
            _ => {
                panic!("implement: isTypeConvertible — non-equal kind cases: {:?} -> {:?}", source_type, target_type);
            }
        }

        if source_region != target_region {
            return false;
        }

        match (source_ownership, target_ownership) {
            (a, b) if a == b => {}
            // At some point maybe we should automatically convert borrow to pointer and vice versa
            // and perhaps automatically promote borrow or pointer to weak?
            (OwnershipT::Own, OwnershipT::Borrow) => return false,
            (OwnershipT::Own, OwnershipT::Weak) => return false,
            (OwnershipT::Own, OwnershipT::Share) => return false,
            (OwnershipT::Borrow, OwnershipT::Own) => return false,
            (OwnershipT::Borrow, OwnershipT::Weak) => return false,
            (OwnershipT::Borrow, OwnershipT::Share) => return false,
            (OwnershipT::Weak, OwnershipT::Own) => return false,
            (OwnershipT::Weak, OwnershipT::Borrow) => return false,
            (OwnershipT::Weak, OwnershipT::Share) => return false,
            (OwnershipT::Share, OwnershipT::Borrow) => return false,
            (OwnershipT::Share, OwnershipT::Weak) => return false,
            (OwnershipT::Share, OwnershipT::Own) => return false,
            _ => unreachable!(),
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
        let mut lookup_filter = HashSet::new();
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
    pub fn coerce_to_coord(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
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
    pub fn resolve_struct_template(
        &self,
        struct_templata: &'t StructDefinitionTemplataT<'s, 't>,
    ) -> &'t IdT<'s, 't> {
        let declaring_env = struct_templata.declaring_env;
        let struct_a = struct_templata.origin_struct;
        let translated = self.translate_struct_name(struct_a.name);
        let local_name = match translated {
            IStructTemplateNameT::StructTemplate(r) => INameT::StructTemplate(r),
            IStructTemplateNameT::AnonymousSubstructTemplate(r) => INameT::AnonymousSubstructTemplate(r),
            IStructTemplateNameT::LambdaCitizenTemplate(r) => INameT::LambdaCitizenTemplate(r),
        };
        declaring_env.id().add_step(self.typing_interner, local_name)
    }
/*
  def resolveStructTemplate(structTemplata: StructDefinitionTemplataT): IdT[IStructTemplateNameT] = {
    val StructDefinitionTemplataT(declaringEnv, structA) = structTemplata
    declaringEnv.id.addStep(nameTranslator.translateStructName(structA.name))
  }
*/
    pub fn resolve_interface_template(
        &self,
        interface_templata: &'t InterfaceDefinitionTemplataT<'s, 't>,
    ) -> &'t IdT<'s, 't> {
        let declaring_env = interface_templata.declaring_env;
        let interface_a = interface_templata.origin_interface;
        let translated = self.translate_interface_name(*interface_a.name);
        let local_name = match translated {
            IInterfaceTemplateNameT::InterfaceTemplate(r) => INameT::InterfaceTemplate(r),
        };
        declaring_env.id().add_step(self.typing_interner, local_name)
    }
/*
  def resolveInterfaceTemplate(interfaceTemplata: InterfaceDefinitionTemplataT): IdT[IInterfaceTemplateNameT] = {
    val InterfaceDefinitionTemplataT(declaringEnv, interfaceA) = interfaceTemplata
    declaringEnv.id.addStep(nameTranslator.translateInterfaceName(interfaceA.name))
  }
*/
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
    pub fn citizen_is_from_template(
        &self,
        actual_citizen_ref: ICitizenTT<'s, 't>,
        expected_citizen_templata: ITemplataT<'s, 't>,
    ) -> bool {
        let citizen_template_id = match expected_citizen_templata {
            ITemplataT::StructDefinition(st) => *self.resolve_struct_template(st),
            ITemplataT::InterfaceDefinition(it) => *self.resolve_interface_template(it),
            ITemplataT::Kind(kt) => {
                match ISubKindTT::try_from(kt.kind) {
                    Ok(sub) => self.get_citizen_template(sub.id()),
                    Err(_) => return false,
                }
            }
            ITemplataT::Coord(ct) => {
                match (ct.coord.ownership, ISubKindTT::try_from(ct.coord.kind)) {
                    (OwnershipT::Own, Ok(sub)) | (OwnershipT::Share, Ok(sub)) => self.get_citizen_template(sub.id()),
                    _ => return false,
                }
            }
            _ => return false,
        };
        self.get_citizen_template(ISubKindTT::from(actual_citizen_ref).id()) == citizen_template_id
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
    pub fn create_placeholder(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
        name_prefix: IdT<'s, 't>,
        generic_param: &'s GenericParameterS<'s>,
        index: i32,
        rune_to_type: &HashMap<IRuneS<'s>, ITemplataType<'s>>,
        current_height: Option<i32>,
        register_with_compiler_outputs: bool,
    ) -> ITemplataT<'s, 't> {
        let rune_type = *rune_to_type.get(&generic_param.rune.rune).unwrap();
        let rune = generic_param.rune.rune;
        match rune_type {
            ITemplataType::KindTemplataType(_) => {
                let (kind_mutable, _region_mutable) = match &generic_param.tyype {
                    IGenericParameterTypeS::CoordGenericParameterType(CoordGenericParameterTypeS { kind_mutable, region_mutable, .. }) => {
                        (if *kind_mutable { OwnershipT::Own } else { OwnershipT::Share }, *region_mutable)
                    }
                    _ => (OwnershipT::Own, false),
                };
                ITemplataT::Kind(self.typing_interner.alloc(self.create_kind_placeholder_inner(
                    coutputs, env, name_prefix, index, rune, kind_mutable, register_with_compiler_outputs)))
            }
            ITemplataType::CoordTemplataType(_) => {
                let (kind_mutable, region_mutability) = match &generic_param.tyype {
                    IGenericParameterTypeS::CoordGenericParameterType(CoordGenericParameterTypeS { kind_mutable, region_mutable, .. }) => {
                        (if *kind_mutable { OwnershipT::Own } else { OwnershipT::Share },
                         if *region_mutable { IRegionMutabilityS::ReadWriteRegion } else { IRegionMutabilityS::ReadOnlyRegion })
                    }
                    _ => (OwnershipT::Own, IRegionMutabilityS::ReadOnlyRegion),
                };
                ITemplataT::Coord(self.typing_interner.alloc(self.create_coord_placeholder_inner(
                    coutputs, env, name_prefix, index, rune, current_height,
                    region_mutability, kind_mutable, register_with_compiler_outputs)))
            }
            other_type => {
                self.create_non_kind_non_region_placeholder_inner(name_prefix, index, rune, other_type)
            }
        }
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
    pub fn create_coord_placeholder_inner(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
        name_prefix: IdT<'s, 't>,
        index: i32,
        rune: IRuneS<'s>,
        current_height: Option<i32>,
        region_mutability: IRegionMutabilityS,
        kind_ownership: OwnershipT,
        register_with_compiler_outputs: bool,
    ) -> CoordTemplataT<'s, 't> {
        // val regionPlaceholderTemplata = RegionT(DefaultRegionT)
        let region_placeholder_templata = RegionT { region: IRegionT::Default };

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
    val regionPlaceholderTemplata = RegionT(DefaultRegionT)

    val kindPlaceholderT =
      createKindPlaceholderInner(
        coutputs, env, namePrefix, index, rune, kindOwnership, registerWithCompilerOutputs)

    CoordTemplataT(CoordT(kindOwnership, regionPlaceholderTemplata, kindPlaceholderT.kind))
  }
*/
    pub fn create_kind_placeholder_inner(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
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
            KindPlaceholderTemplateNameT { index, rune, _phantom: PhantomData });
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

            // Per @BDPFWDZ: the placeholder env stays empty. Bound declarations
            // (IsaTemplataT, FunctionBoundNameT) live in the introducing function's near-env, not
            // here. Lookups walk from the calling env to find them.
            // val placeholderEnv = GeneralEnvironmentT.childOf(interner, env, kindPlaceholderTemplateId, kindPlaceholderTemplateId)
            let placeholder_env = child_of(
                self.typing_interner,
                self.scout_arena,
                env,
                *kind_placeholder_template_id,
                kind_placeholder_template_id,
                vec![],
            );
            let placeholder_env_ref: IInDenizenEnvironmentT<'s, 't> =
                IInDenizenEnvironmentT::General(placeholder_env);
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

      // Per @BDPFWDZ: the placeholder env stays empty. Bound declarations
      // (IsaTemplataT, FunctionBoundNameT) live in the introducing function's near-env, not
      // here. Lookups walk from the calling env to find them.
      val placeholderEnv = GeneralEnvironmentT.childOf(interner, env, kindPlaceholderTemplateId, kindPlaceholderTemplateId)
      coutputs.declareTypeOuterEnv(kindPlaceholderTemplateId, placeholderEnv)
      coutputs.declareTypeInnerEnv(kindPlaceholderTemplateId, placeholderEnv)
    }

    KindTemplataT(KindPlaceholderT(kindPlaceholderId))
  }
*/
    pub fn create_non_kind_non_region_placeholder_inner(
        &self,
        name_prefix: IdT<'s, 't>,
        index: i32,
        rune: IRuneS<'s>,
        tyype: ITemplataType<'s>,
    ) -> ITemplataT<'s, 't> {
        // val idT = namePrefix.addStep(interner.intern(NonKindNonRegionPlaceholderNameT(index, rune)))
        let placeholder_name = self.typing_interner.intern_non_kind_non_region_placeholder_name(
            NonKindNonRegionPlaceholderNameT { index, rune, _phantom: PhantomData }
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
