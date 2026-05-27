use std::collections::HashMap;
use crate::typing::compiler::Compiler;
use crate::typing::infer_compiler::*;
use crate::solver::solver::*;
use crate::typing::infer::compiler_solver::ITypingPassSolverError;
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::postparsing::*;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::rules::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::compiler_outputs::*;
use crate::higher_typing::ast::*;
use crate::interner::Interner;
use crate::typing::infer_compiler::include_rule_in_call_site_solve;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::typing::env::environment::TemplatasStoreBuilder;
use crate::typing::env::environment::child_of;
use crate::typing::hinputs_t::{InstantiationBoundArgumentsT, InstantiationReachableBoundArgumentsT};
use crate::utils::arena_index_map::ArenaIndexMap;
use std::marker::PhantomData;
use crate::typing::infer_compiler::CompleteResolveSolve;
use crate::typing::types::types::{KindT, InterfaceTT};
use crate::typing::templata::templata::{ITemplataT, KindTemplataT};
use crate::postparsing::names::{IImpreciseNameValS, ImplSubCitizenImpreciseNameValS};
use crate::typing::env::environment::{get_imprecise_name, ILookupContext};
use crate::typing::templata::templata::{ImplDefinitionTemplataT, IsaTemplataT};
use crate::typing::types::types::ICitizenTT;
use crate::postparsing::names::ImplImpreciseNameValS;
use crate::typing::compiler_error_reporter::ICompileErrorT;

/*
package dev.vale.typing.citizen

import dev.vale.highertyping.ImplA
import dev.vale.postparsing.{IRuneS, ITemplataType, ImplImpreciseNameS, ImplSubCitizenImpreciseNameS, ImplTemplataType, LocationInDenizen}
import dev.vale.postparsing.rules.{Equivalencies, IRulexSR, RuleScout}
import dev.vale.solver._
import dev.vale.typing.OverloadResolver.InferFailure
import dev.vale.typing.env.{ExpressionLookupContext, TemplataLookupContext, TemplatasStore}
import dev.vale.typing._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{Accumulator, Err, Interner, Ok, Profiler, RangeS, Result, U, postparsing, vassert, vassertSome, vcurious, vfail, vimpl, vregionmut, vwat}
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.typing._
import dev.vale.typing.ast.{CitizenDefinitionT, ImplT, InterfaceDefinitionT, StructDefinitionT}
import dev.vale.typing.env._
import dev.vale.typing.function._
import dev.vale.typing.infer.ITypingPassSolverError

import scala.collection.immutable.Set

*/

pub enum IsParentResult<'s, 't> {
    IsParent(IsParent<'s, 't>),
    IsntParent(IsntParent<'s, 't>),
}
/*
sealed trait IsParentResult
*/
pub struct IsParent<'s, 't> {
    pub templata: ITemplataT<'s, 't>,
    pub conclusions: std::collections::HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
    pub impl_id: IdT<'s, 't>,
}
/*
case class IsParent(
  templata: ITemplataT[ImplTemplataType],
  conclusions: Map[IRuneS, ITemplataT[ITemplataType]],
  implId: IdT[IImplNameT]
) extends IsParentResult
*/
#[derive(Debug)]
pub struct IsntParent<'s, 't> {
    pub candidates: Vec<IResolvingError<'s, 't>>,
}
/*
case class IsntParent(
  candidates: Vector[IResolvingError]
) extends IsParentResult

*/
/*
class ImplCompiler(
    opts: TypingPassOptions,
    interner: Interner,
    nameTranslator: NameTranslator,
    structCompiler: StructCompiler,
    templataCompiler: TemplataCompiler,
    inferCompiler: InferCompiler) {

  // We don't have an isAncestor call, see REMUIDDA.
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resolve_impl(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        initial_knowns: &[InitialKnown<'s, 't>],
        impl_templata: ImplDefinitionTemplataT<'s, 't>,
    ) -> Result<CompleteResolveSolve<'s, 't>, IResolvingError<'s, 't>> {

        let parent_env = impl_templata.env;
        let impl_a = impl_templata.impl_;

        let impl_template_name: IImplTemplateNameT<'s, 't> = self.translate_impl_name(impl_a.name);
        let impl_template_name_local: INameT<'s, 't> = match impl_template_name {
            IImplTemplateNameT::ImplTemplate(r) => INameT::ImplTemplate(r),
            IImplTemplateNameT::ImplBoundTemplate(_) => panic!("Unimplemented: ImplBoundTemplate in resolve_impl"),
            IImplTemplateNameT::AnonymousSubstructImplTemplate(r) => INameT::AnonymousSubstructImplTemplate(r),
        };
        let impl_template_id: &'t IdT<'s, 't> = parent_env.id().add_step(self.typing_interner, impl_template_name_local);

        let outer_env_store = {
            let store = TemplatasStoreBuilder::new(impl_template_id);
            store.build_in(self.typing_interner)
        };
        let outer_env: &'t CitizenEnvironmentT<'s, 't> = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env: parent_env.global_env(),
            parent_env: IEnvironmentT::from(parent_env),
            template_id: *impl_template_id,
            id: *impl_template_id,
            templatas: outer_env_store,
        });

        let call_site_rules: Vec<IRulexSR<'s>> =
            impl_a.rules.iter().copied().filter(|r| include_rule_in_call_site_solve(r)).collect();

        let rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
            impl_a.rune_to_type.iter().map(|(k, v)| (*k, *v)).collect();

        let mut all_ranges: Vec<RangeS<'s>> = vec![impl_a.range];
        all_ranges.extend_from_slice(parent_ranges);
        let all_ranges_slice = self.typing_interner.alloc_slice_copy(&all_ranges);

        let original_calling_env = calling_env;
        let envs = InferEnv {
            original_calling_env,
            parent_ranges: all_ranges_slice,
            call_location,
            self_env: IEnvironmentT::from(IInDenizenEnvironmentT::Citizen(outer_env)),
            context_region: RegionT { region: IRegionT::Default },
        };
        let mut solver_state = self.make_solver_state(
            envs, coutputs, &call_site_rules, &rune_to_type, all_ranges_slice, initial_knowns, &[]);
        match self.r#continue(envs, coutputs, &mut solver_state) {
            Ok(()) => {}
            Err(e) => return Err(IResolvingError::ResolvingSolveFailedOrIncomplete(e)),
        }
        self.check_resolving_conclusions_and_resolve(
            envs,
            coutputs,
            all_ranges_slice,
            call_location,
            &rune_to_type,
            &call_site_rules,
            &[impl_a.sub_citizen_rune.rune],
            &mut solver_state,
        ).unwrap_or_else(|_e| panic!("Unimplemented: ICompileErrorT from check_resolving_conclusions_and_resolve in resolve_impl"))
    }
/*
  def resolveImpl(
      coutputs: CompilerOutputs,
      parentRanges: List[RangeS],
      callLocation: LocationInDenizen,
      callingEnv: IInDenizenEnvironmentT,
      initialKnowns: Vector[InitialKnown],
      implTemplata: ImplDefinitionTemplataT
  ):
  Result[CompleteResolveSolve, IResolvingError] = {

    val ImplDefinitionTemplataT(parentEnv, impl) = implTemplata
    val ImplA(
    range,
    name,
    identifyingRunes,
    rules,
    runeToType,
    structKindRune,
    subCitizenImpreciseName,
    interfaceKindRune,
    superInterfaceImpreciseName
    ) = impl

    val implTemplateId =
      parentEnv.id.addStep(nameTranslator.translateImplName(name))

    val outerEnv =
      CitizenEnvironmentT(
        parentEnv.globalEnv,
        parentEnv,
        implTemplateId,
        implTemplateId,
        TemplatasStore(implTemplateId, Map(), Map()))

    // Remember, impls can have rules too, such as:
    //   impl<T> Opt<T> for Some<T> where func drop(T)void;
    // so we do need to filter them out when compiling.
    val callSiteRules = rules.filter(InferCompiler.includeRuleInCallSiteSolve)

    // This is callingEnv because we might be coming from an abstract function that's trying
    // to evaluate an override.
    val originalCallingEnv = callingEnv
    val envs = InferEnv(originalCallingEnv, range :: parentRanges, callLocation, outerEnv, RegionT(DefaultRegionT))
    // Per @ECSIIOSZ, this is a per-call-site solver instantiation for impl resolution;
    // initialKnowns come from the caller via solveImplForCall's preprocessing.
    val solver =
      inferCompiler.makeSolverState(
        envs, coutputs, callSiteRules, runeToType, range :: parentRanges, initialKnowns, Vector())

    inferCompiler.continue(envs, coutputs, solver) match {
      case Ok(()) =>
      case Err(e) => return Err(ResolvingSolveFailedOrIncomplete(e))
    }

    inferCompiler.checkResolvingConclusionsAndResolve(
      envs,
      coutputs,
      range :: parentRanges,
      callLocation,
      runeToType,
      callSiteRules,
      // We include the reachable bounds for the struct rune. Those are bounds that this impl will
      // have to satisfy when it calls the interface.
      Vector(structKindRune.rune),
      solver)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn partial_resolve_impl(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        initial_knowns: &[InitialKnown<'s, 't>],
        impl_templata: ImplDefinitionTemplataT<'s, 't>,
    ) -> Result<HashMap<IRuneS<'s>, ITemplataT<'s, 't>>, FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>> {

        let parent_env = impl_templata.env;
        let impl_a = impl_templata.impl_;

        let impl_template_name: IImplTemplateNameT<'s, 't> = self.translate_impl_name(impl_a.name);
        let impl_template_name_local: INameT<'s, 't> = match impl_template_name {
            IImplTemplateNameT::ImplTemplate(r) => INameT::ImplTemplate(r),
            IImplTemplateNameT::ImplBoundTemplate(_) => panic!("Unimplemented: ImplBoundTemplate in partial_resolve_impl"),
            IImplTemplateNameT::AnonymousSubstructImplTemplate(r) => INameT::AnonymousSubstructImplTemplate(r),
        };
        let impl_template_id: &'t IdT<'s, 't> = parent_env.id().add_step(self.typing_interner, impl_template_name_local);

        let outer_env_store = {
            let store = TemplatasStoreBuilder::new(impl_template_id);
            store.build_in(self.typing_interner)
        };
        let outer_env: &'t CitizenEnvironmentT<'s, 't> = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env: parent_env.global_env(),
            parent_env: IEnvironmentT::from(parent_env),
            template_id: *impl_template_id,
            id: *impl_template_id,
            templatas: outer_env_store,
        });

        let call_site_rules: Vec<IRulexSR<'s>> =
            impl_a.rules.iter().copied().filter(|r| include_rule_in_call_site_solve(r)).collect();

        let rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
            impl_a.rune_to_type.iter().map(|(k, v)| (*k, *v)).collect();

        let mut all_ranges: Vec<RangeS<'s>> = vec![impl_a.range];
        all_ranges.extend_from_slice(parent_ranges);
        let all_ranges_slice = self.typing_interner.alloc_slice_from_vec(all_ranges);

        let original_calling_env = calling_env;
        let envs = InferEnv {
            original_calling_env,
            parent_ranges: all_ranges_slice,
            call_location,
            self_env: IEnvironmentT::from(IInDenizenEnvironmentT::Citizen(outer_env)),
            context_region: RegionT { region: IRegionT::Default },
        };
        let mut solver_state = self.make_solver_state(
            envs, coutputs, &call_site_rules, &rune_to_type, all_ranges_slice, initial_knowns, &[]);
        match self.r#continue(envs, coutputs, &mut solver_state) {
            Ok(()) => {}
            Err(e) => return Err(e),
        }
        Ok(solver_state.userify_conclusions().into_iter().collect())
    }
/*
  // WARNING: Doesn't verify conclusions to make sure that any bounds are satisfied!
  def partialResolveImpl(
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    callingEnv: IInDenizenEnvironmentT,
    initialKnowns: Vector[InitialKnown],
    implTemplata: ImplDefinitionTemplataT):
  Result[Map[IRuneS, ITemplataT[ITemplataType]], FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]] = {

    val ImplDefinitionTemplataT(parentEnv, impl) = implTemplata
    val ImplA(
    range,
    name,
    identifyingRunes,
    rules,
    runeToType,
    structKindRune,
    subCitizenImpreciseName,
    interfaceKindRune,
    superInterfaceImpreciseName
    ) = impl

    val implTemplateId =
      parentEnv.id.addStep(nameTranslator.translateImplName(name))

    val outerEnv =
      CitizenEnvironmentT(
        parentEnv.globalEnv,
        parentEnv,
        implTemplateId,
        implTemplateId,
        TemplatasStore(implTemplateId, Map(), Map()))

    // Remember, impls can have rules too, such as:
    //   impl<T> Opt<T> for Some<T> where func drop(T)void;
    // so we do need to filter them out when compiling.
    val callSiteRules = rules.filter(InferCompiler.includeRuleInCallSiteSolve)

    // This is callingEnv because we might be coming from an abstract function that's trying
    // to evaluate an override.
    val originalCallingEnv = callingEnv
    val envs = InferEnv(originalCallingEnv, range :: parentRanges, callLocation, outerEnv, RegionT(DefaultRegionT))
    val solverState =
      inferCompiler.makeSolverState(
        envs, coutputs, callSiteRules, runeToType, range :: parentRanges, initialKnowns, Vector())
    inferCompiler.continue(envs, coutputs, solverState) match {
      case Ok(()) =>
      case Err(e) => return Err(e)
    }
    Ok(solverState.userifyConclusions().toMap)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_impl(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_location: LocationInDenizen<'s>,
        impl_templata: ImplDefinitionTemplataT<'s, 't>,
    ) -> Result<(), ICompileErrorT<'s, 't>> {

        let parent_env = impl_templata.env;
        let impl_a = impl_templata.impl_;

        let impl_template_name: IImplTemplateNameT<'s, 't> = self.translate_impl_name(impl_a.name);
        let impl_template_name_local: INameT<'s, 't> = match impl_template_name {
            IImplTemplateNameT::ImplTemplate(r) => INameT::ImplTemplate(r),
            IImplTemplateNameT::ImplBoundTemplate(_) => panic!("Unimplemented: ImplBoundTemplate in compile_impl"),
            IImplTemplateNameT::AnonymousSubstructImplTemplate(r) => INameT::AnonymousSubstructImplTemplate(r),
        };
        let impl_template_id: &'t IdT<'s, 't> = parent_env.id().add_step(self.typing_interner, impl_template_name_local);

        let impl_outer_env_store_ref = {
            let store = TemplatasStoreBuilder::new(impl_template_id);
            store.build_in(self.typing_interner)
        };
        let impl_outer_env: &'t CitizenEnvironmentT<'s, 't> = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env: parent_env.global_env(),
            parent_env: IEnvironmentT::from(parent_env),
            template_id: *impl_template_id,
            id: *impl_template_id,
            templatas: impl_outer_env_store_ref,
        });
        let impl_outer_env_iden: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(impl_outer_env);

        let rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
            impl_a.rune_to_type.iter().map(|(k, v)| (*k, *v)).collect();

        let impl_placeholders: Vec<InitialKnown<'s, 't>> =
            impl_a.generic_params.iter().enumerate().map(|(index, generic_param)| {
                let placeholder = self.create_placeholder(
                    coutputs, impl_outer_env_iden, *impl_template_id, generic_param, index as i32, &rune_to_type,
                    None, true);
                InitialKnown { rune: generic_param.rune, templata: placeholder }
            }).collect();

        let definition_rules: Vec<IRulexSR<'s>> =
            impl_a.rules.iter().copied().filter(|r| include_rule_in_definition_solve(r)).collect();

        let envs = InferEnv {
            original_calling_env: impl_outer_env_iden,
            parent_ranges: self.typing_interner.alloc_slice_from_vec(vec![impl_a.range]),
            call_location,
            self_env: IEnvironmentT::from(impl_outer_env_iden),
            context_region: RegionT { region: IRegionT::Default },
        };

        let complete_define_solve = match self.solve_for_defining(
            envs,
            coutputs,
            &definition_rules,
            &rune_to_type,
            &[impl_a.range],
            call_location,
            &impl_placeholders,
            &[],
            &[impl_a.sub_citizen_rune.rune],
        ) {
            Ok(c) => c,
            Err(_e) => {
                panic!("TypingPassDefiningError from compile_impl");
            }
        };

        let inferences = complete_define_solve.conclusions;
        let reachable_bounds_from_sub_citizen = &complete_define_solve.rune_to_bound.rune_to_citizen_rune_to_reachable_prototype;

        let sub_citizen: ICitizenTT<'s, 't> = match inferences.get(&impl_a.sub_citizen_rune.rune) {
            None => panic!("vwat: sub_citizen_rune not in inferences"),
            Some(ITemplataT::Kind(k)) => match k.kind {
                KindT::Struct(s) => ICitizenTT::Struct(s),
                KindT::Interface(i) => ICitizenTT::Interface(i),
                _ => panic!("vwat: sub citizen kind is not a citizen"),
            },
            Some(_) => panic!("vwat: expected KindTemplataT for sub_citizen"),
        };
        let sub_citizen_id = match sub_citizen {
            ICitizenTT::Struct(s) => s.id,
            ICitizenTT::Interface(i) => i.id,
        };
        let sub_citizen_template_id = self.get_citizen_template(sub_citizen_id);

        let super_interface: &'t InterfaceTT<'s, 't> = match inferences.get(&impl_a.interface_kind_rune.rune) {
            None => panic!("vwat: interface_kind_rune not in inferences"),
            Some(ITemplataT::Kind(k)) => match k.kind {
                KindT::Interface(i) => i,
                _ => return Err(ICompileErrorT::CantImplNonInterface {
                    range: self.typing_interner.alloc_slice_copy(&[impl_a.range]),
                    templata: ITemplataT::Kind(*k),
                }),
            },
            Some(other) => return Err(ICompileErrorT::CantImplNonInterface {
                range: self.typing_interner.alloc_slice_copy(&[impl_a.range]),
                templata: *other,
            }),
        };
        let super_interface_template_id = self.get_interface_template(super_interface.id);

        let template_args: Vec<ITemplataT<'s, 't>> =
            impl_a.generic_params.iter().map(|p| *inferences.get(&p.rune.rune).expect("rune in inferences")).collect();
        let instantiated_id: IdT<'s, 't> = self.assemble_impl_name(*impl_template_id, &template_args, sub_citizen);
        let instantiated_id_ref: &'t IdT<'s, 't> = self.typing_interner.alloc(instantiated_id);

        let mut inner_env_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
            reachable_bounds_from_sub_citizen.iter()
                .flat_map(|(_, rb)| rb.citizen_rune_to_reachable_prototype.iter().map(|(_, proto)| proto))
                .enumerate()
                .map(|(index, proto)| -> (INameT<'s, 't>, IEnvEntryT<'s, 't>) {
                    let name = self.typing_interner.intern_reachable_prototype_name(
                        ReachablePrototypeNameT { num: index as i32, _phantom: PhantomData });
                    let entry = IEnvEntryT::Templata(ITemplataT::Prototype(
                        self.typing_interner.alloc(PrototypeTemplataT { prototype: proto })));
                    (INameT::ReachablePrototype(name), entry)
                })
                .collect();
        inner_env_entries.extend(inferences.iter().map(|(name_s, templata)| {
            let rune_name = self.typing_interner.intern_rune_name(
                RuneNameT { rune: *name_s, _phantom: PhantomData });
            (INameT::Rune(rune_name), IEnvEntryT::Templata(*templata))
        }));

        let impl_inner_env: &'t GeneralEnvironmentT<'s, 't> = child_of(
            self.typing_interner,
            self.scout_arena,
            IInDenizenEnvironmentT::Citizen(impl_outer_env),
            *impl_template_id,
            instantiated_id_ref,
            inner_env_entries,
        );
        let impl_inner_env_iden: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::General(impl_inner_env);

        let rune_to_needed_function_bound = self.assemble_rune_to_function_bound(impl_inner_env.templatas);
        let rune_to_needed_impl_bound = self.assemble_rune_to_impl_bound(impl_inner_env.templatas);

        let rune_index_to_independence =
            self.calculate_runes_independence(coutputs, call_location, impl_templata, impl_outer_env_iden, *super_interface);

        let mut rune_to_reachable: ArenaIndexMap<'t, IRuneS<'s>, &'t InstantiationReachableBoundArgumentsT<'s, 't>> =
            self.typing_interner.alloc_index_map();
        for (k, v) in reachable_bounds_from_sub_citizen.iter() {
            rune_to_reachable.insert(*k, *v);
        }

        let instantiation_bound_params = self.typing_interner.alloc(InstantiationBoundArgumentsT {
            rune_to_bound_prototype: self.typing_interner.alloc_index_map_from_iter(
                rune_to_needed_function_bound.into_iter().map(|(k, v)| (k, *v))),
            rune_to_citizen_rune_to_reachable_prototype: rune_to_reachable,
            rune_to_bound_impl: self.typing_interner.alloc_index_map_from_iter(
                rune_to_needed_impl_bound.into_iter()),
        });

        let impl_t = ImplT {
            templata: impl_templata,
            instantiated_id,
            template_id: *impl_template_id,
            sub_citizen_template_id,
            sub_citizen,
            super_interface: *super_interface,
            super_interface_template_id,
            instantiation_bound_params,
            rune_index_to_independence: self.typing_interner.alloc_slice_from_vec(rune_index_to_independence),
        };

        coutputs.declare_type(impl_template_id);
        coutputs.declare_type_outer_env(impl_template_id, impl_outer_env_iden);
        coutputs.declare_type_inner_env(impl_template_id, impl_inner_env_iden);
        coutputs.add_impl(self.typing_interner.alloc(impl_t));
        Ok(())
    }
/*
  // This will just figure out the struct template and interface template,
  // so we can add it to the temputs.
  def compileImpl(coutputs: CompilerOutputs, callLocation: LocationInDenizen, implTemplata: ImplDefinitionTemplataT): Unit = {
    val ImplDefinitionTemplataT(parentEnv, implA) = implTemplata

    val implTemplateId =
      parentEnv.id.addStep(
        nameTranslator.translateImplName(implA.name))

    val implOuterEnv =
      CitizenEnvironmentT(
        parentEnv.globalEnv,
        parentEnv,
        implTemplateId,
        implTemplateId,
        TemplatasStore(implTemplateId, Map(), Map()))

    // We might one day need to incrementally solve and add placeholders here like we do for
    // functions and structs, see IRAGP.
    val implPlaceholders =
      implA.genericParams.zipWithIndex.map({ case (rune, index) =>
        val placeholder =
          templataCompiler.createPlaceholder(
            coutputs, implOuterEnv, implTemplateId, rune, index, implA.runeToType, vregionmut(None), true)
        InitialKnown(rune.rune, placeholder)
      })

    val ImplA(
    range,
    name,
    identifyingRunes,
    rules,
    runeToType,
    structKindRune,
    subCitizenImpreciseName,
    interfaceKindRune,
    superInterfaceImpreciseName
    ) = implA

    val outerEnv =
      CitizenEnvironmentT(
        parentEnv.globalEnv,
        parentEnv,
        implTemplateId,
        implTemplateId,
        TemplatasStore(implTemplateId, Map(), Map()))

    // Remember, impls can have rules too, such as:
    //   impl<T> Opt<T> for Some<T> where func drop(T)void;
    // so we do need to filter them out when compiling.
    val definitionRules = rules.filter(InferCompiler.includeRuleInDefinitionSolve)

    val envs =
      InferEnv(
        implOuterEnv,
        List(range),
        callLocation,
        outerEnv,
        RegionT(DefaultRegionT))
    val CompleteDefineSolve(inferences, InstantiationBoundArgumentsT(_, reachableBoundsFromSubCitizen, _)) =
      inferCompiler.solveForDefining(
        envs,
        coutputs,
        definitionRules,
        runeToType,
        List(range),
        callLocation,
        implPlaceholders,
        Vector(),
        // We include reachable bounds for the struct so we don't have to re-specify all its bounds in the impl.
        Vector(structKindRune.rune)) match {
        case Ok(i) => i
        case Err(e) => throw CompileErrorExceptionT(TypingPassDefiningError(List(implA.range), e))
      }

    val subCitizen =
      inferences.get(implA.subCitizenRune.rune) match {
        case None => vwat()
        case Some(KindTemplataT(s: ICitizenTT)) => s
        case _ => vwat()
      }
    val subCitizenTemplateId =
      TemplataCompiler.getCitizenTemplate(subCitizen.id)

    val superInterface =
      inferences.get(implA.interfaceKindRune.rune) match {
        case None => vwat()
        case Some(KindTemplataT(i@InterfaceTT(_))) => i
        case Some(other) => throw CompileErrorExceptionT(CantImplNonInterface(List(implA.range), other))
      }
    val superInterfaceTemplateId =
      TemplataCompiler.getInterfaceTemplate(superInterface.id)

    val subCitizenWeakable =
      coutputs.lookupCitizen(subCitizen) match {
        case s: StructDefinitionT => s.weakable
        case i: InterfaceDefinitionT => i.weakable
      }
    val superInterfaceWeakable = coutputs.lookupInterface(superInterface).weakable
    if (subCitizenWeakable != superInterfaceWeakable) {
      throw WeakableImplingMismatch(subCitizenWeakable, superInterfaceWeakable)
    }
    val templateArgs = implA.genericParams.map(_.rune.rune).map(inferences)
    val instantiatedId = assembleImplName(implTemplateId, templateArgs, subCitizen)

    val implInnerEnv =
      GeneralEnvironmentT.childOf(
        interner,
        implOuterEnv,
        implTemplateId,
        instantiatedId,
        reachableBoundsFromSubCitizen.values.flatMap(_.citizenRuneToReachablePrototype.values).zipWithIndex.map({ case (templata, index) =>
          interner.intern(ReachablePrototypeNameT(index)) -> TemplataEnvEntry(PrototypeTemplataT(templata))
        }).toVector ++
        inferences.map({ case (nameS, templata) =>
          interner.intern(RuneNameT((nameS))) -> TemplataEnvEntry(templata)
        }).toVector)
    val runeToNeededFunctionBound = TemplataCompiler.assembleRuneToFunctionBound(implInnerEnv.templatas)
    val runeToNeededImplBound = TemplataCompiler.assembleRuneToImplBound(implInnerEnv.templatas)
//    vcurious(runeToFunctionBound1 == runeToNeededFunctionBound) // which do we want?

    val runeIndexToIndependence =
      calculateRunesIndependence(coutputs, callLocation, implTemplata, implOuterEnv, superInterface)

    val implT =
      ImplT(
        implTemplata,
        //implOuterEnv,
        instantiatedId,
        implTemplateId,
        subCitizenTemplateId,
        subCitizen,
        superInterface,
        superInterfaceTemplateId,
        InstantiationBoundArgumentsT.make[FunctionBoundNameT, ImplBoundNameT](
          runeToNeededFunctionBound,
          reachableBoundsFromSubCitizen,
          runeToNeededImplBound),
        runeIndexToIndependence.toVector)
    coutputs.declareType(implTemplateId)
    coutputs.declareTypeOuterEnv(implTemplateId, implOuterEnv)
    coutputs.declareTypeInnerEnv(implTemplateId, implInnerEnv)
    coutputs.addImpl(implT)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn calculate_runes_independence(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_location: LocationInDenizen<'s>,
        impl_templata: ImplDefinitionTemplataT<'s, 't>,
        impl_outer_env: IInDenizenEnvironmentT<'s, 't>,
        interface: InterfaceTT<'s, 't>,
    ) -> Vec<bool> {
        let initial_knowns = vec![InitialKnown {
            rune: impl_templata.impl_.interface_kind_rune,
            templata: ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::Interface(self.typing_interner.alloc(interface)) })),
        }];
        let partial_case_conclusions = match self.partial_resolve_impl(
            coutputs,
            &[impl_templata.impl_.range],
            call_location,
            impl_outer_env,
            &initial_knowns,
            impl_templata,
        ) {
            Ok(c) => c,
            Err(_e) => panic!("CouldntEvaluatImpl from calculate_runes_independence"),
        };
        impl_templata.impl_.generic_params.iter()
            .map(|p| !partial_case_conclusions.contains_key(&p.rune.rune))
            .collect()
    }
/*
  def calculateRunesIndependence(
    coutputs: CompilerOutputs,
    callLocation: LocationInDenizen,
    implTemplata: ImplDefinitionTemplataT,
    implOuterEnv: IInDenizenEnvironmentT,
    interface: InterfaceTT,
  ): Vector[Boolean] = {

    // Now we're going to figure out the <ZZ> for the eg Milano case.

    // Don't verify conclusions, because this will likely be a partial solve, which means we
    // might not even be able to solve the struct, which means we can't pull in any declared
    // function bounds that come from them. We'll check them later.
    val partialCaseConclusionsFromSuperInterface =
      partialResolveImpl(
        coutputs,
        List(implTemplata.impl.range),
        callLocation,
        implOuterEnv,
        Vector(
          InitialKnown(
            implTemplata.impl.interfaceKindRune,
            // We may be feeding in something interesting like IObserver<Opt<T>> here should be fine,
            // the impl will receive it and match it to its own unknown runes appropriately.
            KindTemplataT(interface))),
        implTemplata) match {
        case Ok(c) => c
        case Err(e) => {
          throw CompileErrorExceptionT(CouldntEvaluatImpl(List(implTemplata.impl.range), e))
        }
      }
    // These will be anything that wasn't already determined by the incoming interface.
    // These are the "independent" generic params, like the <ZZ> in Milano.
    // No particular reason they're ordered, it just feels appropriate to keep them in the same
    // order they appeared in the impl.
    val runeToIndependence =
      implTemplata.impl.genericParams.map(_.rune.rune)
        .map(rune => !partialCaseConclusionsFromSuperInterface.contains(rune))

    runeToIndependence
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn assemble_impl_name(
        &self,
        template_name: IdT<'s, 't>,
        template_args: &[ITemplataT<'s, 't>],
        sub_citizen: ICitizenTT<'s, 't>,
    ) -> IdT<'s, 't> {
        let impl_template_name: IImplTemplateNameT<'s, 't> = match template_name.local_name {
            INameT::ImplTemplate(r) => IImplTemplateNameT::ImplTemplate(r),
            INameT::ImplBoundTemplate(r) => IImplTemplateNameT::ImplBoundTemplate(r),
            INameT::AnonymousSubstructImplTemplate(r) => IImplTemplateNameT::AnonymousSubstructImplTemplate(r),
            other => panic!("assemble_impl_name: expected impl template name, got {:?}", other),
        };
        let new_local_name = impl_template_name.make_impl_name(self.typing_interner, template_args, sub_citizen);
        *self.typing_interner.intern_id(IdValT {
            package_coord: template_name.package_coord,
            init_steps: template_name.init_steps,
            local_name: new_local_name,
        })
    }
/*
  def assembleImplName(
    templateName: IdT[IImplTemplateNameT],
    templateArgs: Vector[ITemplataT[ITemplataType]],
    subCitizen: ICitizenTT):
  IdT[IImplNameT] = {
    templateName.copy(
      localName = templateName.localName.makeImplName(interner, templateArgs, subCitizen))
  }

  //    // First, figure out what citizen is implementing.
  //    val subCitizenImpreciseName = RuleScout.getRuneKindTemplate(implA.rules, implA.structKindRune.rune)
  //    val subCitizenTemplata =
  //      implOuterEnv.lookupNearestWithImpreciseName(subCitizenImpreciseName, Set(TemplataLookupContext)) match {
  //        case None => throw CompileErrorExceptionT(ImplSubCitizenNotFound(implA.range, subCitizenImpreciseName))
  //        case Some(it @ CitizenTemplata(_, _)) => it
  //        case Some(other) => throw CompileErrorExceptionT(NonCitizenCantImpl(implA.range, other))
  //      }
  //    val subCitizenTemplateFullName = templataCompiler.resolveCitizenTemplate(subCitizenTemplata)
  //    val subCitizenDefinition = coutputs.lookupCitizen(subCitizenTemplateFullName)
  //    val subCitizenPlaceholders =
  //      subCitizenDefinition.genericParamTypes.zipWithIndex.map({ case (tyype, index) =>
  //        templataCompiler.createPlaceholder(coutputs, implOuterEnv, implTemplateFullName, index, tyype)
  //      })
  //    val placeholderedSubCitizenTT =
  //      structCompiler.resolveCitizen(coutputs, implOuterEnv, implA.range, subCitizenTemplata, subCitizenPlaceholders)
  //
  //
  //    // Now, figure out what interface is being implemented.
  //    val superInterfaceImpreciseName = RuleScout.getRuneKindTemplate(implA.rules, implA.interfaceKindRune.rune)
  //    val superInterfaceTemplata =
  //      implOuterEnv.lookupNearestWithImpreciseName(superInterfaceImpreciseName, Set(TemplataLookupContext)) match {
  //        case None => throw CompileErrorExceptionT(ImplSuperInterfaceNotFound(implA.range, superInterfaceImpreciseName))
  //        case Some(it @ InterfaceTemplata(_, _)) => it
  //        case Some(other) => throw CompileErrorExceptionT(CantImplNonInterface(implA.range, other))
  //      }
  //    val superInterfaceTemplateFullName = templataCompiler.resolveCitizenTemplate(superInterfaceTemplata)
  //    val superInterfaceDefinition = coutputs.lookupCitizen(superInterfaceTemplateFullName)
  //    val superInterfacePlaceholders =
  //      superInterfaceDefinition.genericParamTypes.zipWithIndex.map({ case (tyype, index) =>
  //        val placeholderNameT = implTemplateFullName.addStep(PlaceholderNameT(PlaceholderTemplateNameT(index)))
  //        templataCompiler.createPlaceholder(coutputs, implOuterEnv, implTemplateFullName, index, tyype)
  //      })
  //    val placeholderedSuperInterfaceTT =
  //      structCompiler.resolveInterface(coutputs, implOuterEnv, implA.range, superInterfaceTemplata, superInterfacePlaceholders)
  //
  //    // Now compile it from the sub citizen's perspective.
  //    compileImplGivenSubCitizen(coutputs, placeholderedSubCitizenTT, implTemplata)
  //    // Now compile it from the super interface's perspective.
  //    compileImplGivenSuperInterface(coutputs, placeholderedSuperInterfaceTT, implTemplata)
  //  }
  //
  //  def compileParentImplsForSubCitizen(
  //    coutputs: CompilerOutputs,
  //    subCitizenDefinition: CitizenDefinitionT):
  //  Unit = {
  //    Profiler.frame(() => {
  //      val subCitizenTemplateFullName = subCitizenDefinition.templateName
  //      val subCitizenEnv = coutputs.getEnvForTemplate(subCitizenTemplateFullName)
  //      // See INSHN, the imprecise name for an impl is the wrapped imprecise name of its struct template.
  //      val needleImplTemplateFullName = interner.intern(ImplTemplateSubNameT(subCitizenTemplateFullName))
  //      val implTemplates =
  //        subCitizenEnv.lookupAllWithName(needleImplTemplateFullName, Set(TemplataLookupContext))
  //      implTemplates.foreach({
  //        case it @ ImplTemplata(_, _) => {
  //          compileImplGivenSubCitizen(coutputs, subCitizenDefinition, it)
  //        }
  //        case other => vwat(other)
  //      })
  //    })
  //  }
  //
  //  def compileChildImplsForParentInterface(
  //    coutputs: CompilerOutputs,
  //    parentInterfaceDefinition: InterfaceDefinitionT):
  //  Unit = {
  //    Profiler.frame(() => {
  //      val parentInterfaceTemplateFullName = parentInterfaceDefinition.templateName
  //      val parentInterfaceEnv = coutputs.getEnvForTemplate(parentInterfaceTemplateFullName)
  //      // See INSHN, the imprecise name for an impl is the wrapped imprecise name of its struct template.
  //      val needleImplTemplateFullName = interner.intern(ImplTemplateSuperNameT(parentInterfaceTemplateFullName))
  //      val implTemplates =
  //        parentInterfaceEnv.lookupAllWithName(needleImplTemplateFullName, Set(TemplataLookupContext))
  //      implTemplates.foreach({
  //        case impl @ ImplTemplata(_, _) => {
  //          compileImplGivenSuperInterface(coutputs, parentInterfaceDefinition, impl)
  //        }
  //        case other => vwat(other)
  //      })
  //    })
  //  }

  //  // Doesn't include self
  //  def compileGetAncestorInterfaces(
  //    coutputs: CompilerOutputs,
  //    descendantCitizenRef: ICitizenTT):
  //  (Map[InterfaceTT, ImplTemplateNameT]) = {
  //    Profiler.frame(() => {
  //      val parentInterfacesAndImpls =
  //        compileGetParentInterfaces(coutputs, descendantCitizenRef)
  //
  //      // Make a map that contains all the parent interfaces, with distance 1
  //      val foundSoFar =
  //        parentInterfacesAndImpls.map({ case (interfaceRef, impl) => (interfaceRef, impl) }).toMap
  //
  //      compileGetAncestorInterfacesInner(
  //        coutputs,
  //        foundSoFar,
  //        parentInterfacesAndImpls.toMap)
  //    })
  //  }
  //
  //  private def compileGetAncestorInterfacesInner(
  //    coutputs: CompilerOutputs,
  //    // This is so we can know what we've already searched.
  //    nearestDistanceByInterfaceRef: Map[InterfaceTT, ImplTemplateNameT],
  //    // These are the interfaces that are *exactly* currentDistance away.
  //    // We will do our searching from here.
  //    interfacesAtCurrentDistance: Map[InterfaceTT, ImplTemplateNameT]):
  //  (Map[InterfaceTT, ImplTemplateNameT]) = {
  //    val interfacesAtNextDistance =
  //      interfacesAtCurrentDistance.foldLeft((Map[InterfaceTT, ImplTemplateNameT]()))({
  //        case ((previousAncestorInterfaceRefs), (parentInterfaceRef, parentImpl)) => {
  //          val parentAncestorInterfaceRefs =
  //            compileGetParentInterfaces(coutputs, parentInterfaceRef)
  //          (previousAncestorInterfaceRefs ++ parentAncestorInterfaceRefs)
  //        }
  //      })
  //
  //    // Discard the ones that have already been found; they're actually at
  //    // a closer distance.
  //    val newlyFoundInterfaces =
  //    interfacesAtNextDistance.keySet
  //      .diff(nearestDistanceByInterfaceRef.keySet)
  //      .toVector
  //      .map(key => (key -> interfacesAtNextDistance(key)))
  //      .toMap
  //
  //    if (newlyFoundInterfaces.isEmpty) {
  //      (nearestDistanceByInterfaceRef)
  //    } else {
  //      // Combine the previously found ones with the newly found ones.
  //      val newNearestDistanceByInterfaceRef =
  //        nearestDistanceByInterfaceRef ++ newlyFoundInterfaces.toMap
  //
  //      compileGetAncestorInterfacesInner(
  //        coutputs,
  //        newNearestDistanceByInterfaceRef,
  //        newlyFoundInterfaces)
  //    }
  //  }
  //
  //  def getParents(
  //    coutputs: CompilerOutputs,
  //    subCitizenTT: ICitizenTT):
  //  Vector[InterfaceTT] = {
  //    val subCitizenTemplateFullName = TemplataCompiler.getCitizenTemplate(subCitizenTT.fullName)
  //    coutputs
  //      .getParentImplsForSubCitizenTemplate(subCitizenTemplateFullName)
  //      .map({ case ImplT(_, parentInterfaceFromPlaceholderedSubCitizen, _, _) =>
  //        val substituter =
  //          TemplataCompiler.getPlaceholderSubstituter(interner, subCitizenTT.fullName)
  //        substituter.substituteForInterface(parentInterfaceFromPlaceholderedSubCitizen)
  //      }).toVector
  //  }
  //

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn is_descendant(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        kind: ISubKindTT<'s, 't>,
    ) -> bool {
        self.get_parents(coutputs, parent_ranges, call_location, calling_env, kind).is_empty() == false
    }
/*
  def isDescendant(
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    callingEnv: IInDenizenEnvironmentT,
    kind: ISubKindTT):
  Boolean = {
    getParents(coutputs, parentRanges, callLocation, callingEnv, kind).nonEmpty
  }

  // def getImplDescendantGivenParent(
  //   coutputs: CompilerOutputs,
  //   parentRanges: List[RangeS],
  //     callLocation: LocationInDenizen,
  //   callingEnv: IInDenizenEnvironmentT,
  //   implTemplata: ImplDefinitionTemplataT,
  //   parent: InterfaceTT,
  //   verifyConclusions: Boolean,
  //   declareBounds: Boolean):
  // Result[ICitizenTT, FailedSolve] = {
  //   val initialKnowns =
  //     Vector(
  //       InitialKnown(implTemplata.impl.interfaceKindRune, KindTemplataT(parent)))
  //   val CompleteCompilerSolve(_, conclusions, _, _) =
  //     solveImplForCall(coutputs, parentRanges, callLocation, callingEnv, initialKnowns, implTemplata, declareBounds, true) match {
  //       case ccs @ CompleteCompilerSolve(_, _, _, _) => ccs
  //       case x : FailedSolve => return Err(x)
  //     }
  //   val parentTT = conclusions.get(implTemplata.impl.subCitizenRune.rune)
  //   vassertSome(parentTT) match {
  //     case KindTemplataT(i : ICitizenTT) => Ok(i)
  //     case _ => vwat()
  //   }
  // }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_impl_parent_given_sub_citizen(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        impl_templata: ImplDefinitionTemplataT<'s, 't>,
        child: ICitizenTT<'s, 't>,
    ) -> Result<InterfaceTT<'s, 't>, IResolvingError<'s, 't>> {

        let initial_knowns = vec![
            InitialKnown {
                rune: impl_templata.impl_.sub_citizen_rune,
                templata: ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::from(child) })),
            }
        ];
        let _child_env = coutputs.get_outer_env_for_type(parent_ranges, self.get_citizen_template(child.id()));
        let conclusions = match self.resolve_impl(coutputs, parent_ranges, call_location, calling_env, &initial_knowns, impl_templata) {
            Ok(CompleteResolveSolve { conclusions, .. }) => conclusions,
            Err(x) => return Err(x),
        };
        let parent_tt = conclusions.get(&impl_templata.impl_.interface_kind_rune.rune)
            .unwrap_or_else(|| panic!("vassertSome: interfaceKindRune not in conclusions"));
        match *parent_tt {
            ITemplataT::Kind(kt) => match kt.kind {
                KindT::Interface(i) => Ok(*i),
                _ => panic!("vwat: expected InterfaceTT from interfaceKindRune conclusions"),
            },
            _ => panic!("vwat: expected KindTemplataT from interfaceKindRune conclusions"),
        }
    }
/*
  def getImplParentGivenSubCitizen(
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
      callLocation: LocationInDenizen,
    callingEnv: IInDenizenEnvironmentT,
    implTemplata: ImplDefinitionTemplataT,
    child: ICitizenTT):
  Result[InterfaceTT, IResolvingError] = {
    val initialKnowns =
      Vector(
        InitialKnown(implTemplata.impl.subCitizenRune, KindTemplataT(child)))
    val childEnv =
      coutputs.getOuterEnvForType(
        parentRanges,
        TemplataCompiler.getCitizenTemplate(child.id))
    val CompleteResolveSolve(conclusions, _) =
      resolveImpl(coutputs, parentRanges, callLocation, callingEnv, initialKnowns, implTemplata) match {
        case Ok(ccs) => ccs
        case Err(x) => return Err(x)
      }
    val parentTT = conclusions.get(implTemplata.impl.interfaceKindRune.rune)
    vassertSome(parentTT) match {
      case KindTemplataT(i @ InterfaceTT(_)) => Ok(i)
      case _ => vwat()
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_parents(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        sub_kind: ISubKindTT<'s, 't>,
    ) -> Vec<ISuperKindTT<'s, 't>> {
        let sub_kind_id = sub_kind.id();
        let sub_kind_template_name = self.get_sub_kind_template(sub_kind_id);
        let sub_kind_env = coutputs.get_outer_env_for_type(parent_ranges, sub_kind_template_name);
        let sub_kind_imprecise_name = match get_imprecise_name(self.scout_arena, sub_kind_id.local_name) {
            None => return vec![],
            Some(n) => n,
        };
        let impl_imprecise_name = self.scout_arena.intern_imprecise_name(
            IImpreciseNameValS::ImplSubCitizenImpreciseName(ImplSubCitizenImpreciseNameValS { sub_citizen_imprecise_name: sub_kind_imprecise_name }));
        let lookup_filter = [ILookupContext::TemplataLookupContext].into_iter().collect::<std::collections::HashSet<_>>();
        let mut matching: Vec<ITemplataT<'s, 't>> = Vec::new();
        matching.extend(sub_kind_env.lookup_all_with_imprecise_name(impl_imprecise_name, lookup_filter.clone(), self.typing_interner));
        matching.extend(calling_env.lookup_all_with_imprecise_name(impl_imprecise_name, lookup_filter, self.typing_interner));
        let mut impl_defs_with_duplicates: Vec<ImplDefinitionTemplataT<'s, 't>> = Vec::new();
        let mut impl_templatas_with_duplicates: Vec<IsaTemplataT<'s, 't>> = Vec::new();
        for m in matching {
            match m {
                ITemplataT::ImplDefinition(it) => impl_defs_with_duplicates.push(*it),
                ITemplataT::Isa(it) => impl_templatas_with_duplicates.push(*it),
                _ => panic!("vwat: unexpected templata in getParents matching"),
            }
        }
        let mut seen_ranges: std::collections::HashSet<RangeS<'s>> = std::collections::HashSet::new();
        let impl_defs: Vec<ImplDefinitionTemplataT<'s, 't>> = impl_defs_with_duplicates.into_iter()
            .filter(|d| seen_ranges.insert(d.impl_.range))
            .collect();
        let parents_from_impl_defs: Vec<ISuperKindTT<'s, 't>> = impl_defs.iter().flat_map(|impl_def| {
            match ICitizenTT::try_from(sub_kind) {
                Ok(sub_citizen) => {
                    match self.get_impl_parent_given_sub_citizen(coutputs, parent_ranges, call_location, calling_env, *impl_def, sub_citizen) {
                        Ok(x) => vec![ISuperKindTT::from(&*self.typing_interner.alloc(x))],
                        Err(_) => {
                            // Throwing away error! TODO: Use an index or something instead.
                            vec![]
                        }
                    }
                }
                Err(_) => vec![],
            }
        }).collect();
        let kind_as_kind_t = KindT::from(sub_kind);
        let mut seen_super: std::collections::HashSet<ISuperKindTT<'s, 't>> = std::collections::HashSet::new();
        let parents_from_impl_templatas: Vec<ISuperKindTT<'s, 't>> =
            impl_templatas_with_duplicates.iter()
                .filter(|it| it.sub_kind == kind_as_kind_t)
                .filter_map(|it| ISuperKindTT::try_from(it.super_kind).ok())
                .filter(|sk| seen_super.insert(*sk))
                .collect();
        let mut result = parents_from_impl_defs;
        result.extend(parents_from_impl_templatas);
        result
    }
/*
  def getParents(
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    callingEnv: IInDenizenEnvironmentT,
    subKind: ISubKindTT):
  Vector[ISuperKindTT] = {
    val subKindId = subKind.id
    val subKindTemplateName = TemplataCompiler.getSubKindTemplate(subKindId)
    val subKindEnv = coutputs.getOuterEnvForType(parentRanges, subKindTemplateName)
    val subKindImpreciseName =
      TemplatasStore.getImpreciseName(interner, subKindId.localName) match {
        case None => return Vector()
        case Some(n) => n
      }
    val implImpreciseNameS =
      interner.intern(ImplSubCitizenImpreciseNameS(subKindImpreciseName))

    val matching =
      subKindEnv.lookupAllWithImpreciseName(implImpreciseNameS, Set(TemplataLookupContext)) ++
      callingEnv.lookupAllWithImpreciseName(implImpreciseNameS, Set(TemplataLookupContext))

    val implDefsWithDuplicates = new Accumulator[ImplDefinitionTemplataT]()
    val implTemplatasWithDuplicates = new Accumulator[IsaTemplataT]()

    matching.foreach({
      case it@ImplDefinitionTemplataT(_, _) => implDefsWithDuplicates.add(it)
      case it@IsaTemplataT(_, _, _, _) => implTemplatasWithDuplicates.add(it)
      case _ => vwat()
    })

    val implDefs =
      implDefsWithDuplicates.buildArray().groupBy(_.impl.range).map(_._2.head)
    val parentsFromImplDefs =
      implDefs.flatMap(impl => {
        subKind match {
          case subCitizen : ICitizenTT => {
            getImplParentGivenSubCitizen(coutputs, parentRanges, callLocation, callingEnv, impl, subCitizen) match {
              case Ok(x) => List(x)
              case Err(_) => {
                opts.debugOut("Throwing away error! TODO: Use an index or something instead.")
                List()
              }
            }
          }
          case _ => List()
        }
      }).toVector

    val parentsFromImplTemplatas =
      implTemplatasWithDuplicates
        .buildArray()
        .filter(_.subKind == subKind)
        .map(_.superKind)
        .collect({ case x : ISuperKindTT => x })
        .distinct

    parentsFromImplDefs ++ parentsFromImplTemplatas
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn is_parent(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        sub_kind_tt: ISubKindTT<'s, 't>,
        super_kind_tt: ISuperKindTT<'s, 't>,
    ) -> IsParentResult<'s, 't> {

        let super_kind_imprecise_name = match get_imprecise_name(self.scout_arena, super_kind_tt.id().local_name) {
            None => return IsParentResult::IsntParent(IsntParent { candidates: vec![] }),
            Some(n) => n,
        };
        let sub_kind_imprecise_name = match get_imprecise_name(self.scout_arena, sub_kind_tt.id().local_name) {
            None => return IsParentResult::IsntParent(IsntParent { candidates: vec![] }),
            Some(n) => n,
        };
        let impl_imprecise_name = self.scout_arena.intern_imprecise_name(
            IImpreciseNameValS::ImplImpreciseName(ImplImpreciseNameValS { sub_citizen_imprecise_name: sub_kind_imprecise_name, super_interface_imprecise_name: super_kind_imprecise_name }));

        let sub_kind_env = coutputs.get_outer_env_for_type(parent_ranges, self.get_sub_kind_template(sub_kind_tt.id()));
        let super_kind_env = coutputs.get_outer_env_for_type(parent_ranges, self.get_super_kind_template(super_kind_tt.id()));

        let lookup_filter = [ILookupContext::TemplataLookupContext].into_iter().collect::<std::collections::HashSet<_>>();
        let mut matching: Vec<ITemplataT<'s, 't>> = Vec::new();
        matching.extend(calling_env.lookup_all_with_imprecise_name(impl_imprecise_name, lookup_filter.clone(), self.typing_interner));
        matching.extend(sub_kind_env.lookup_all_with_imprecise_name(impl_imprecise_name, lookup_filter.clone(), self.typing_interner));
        matching.extend(super_kind_env.lookup_all_with_imprecise_name(impl_imprecise_name, lookup_filter, self.typing_interner));

        let mut impl_defs_with_duplicates: Vec<ImplDefinitionTemplataT<'s, 't>> = Vec::new();
        let mut impl_templatas_with_duplicates: Vec<IsaTemplataT<'s, 't>> = Vec::new();
        for m in matching {
            match m {
                ITemplataT::ImplDefinition(it) => impl_defs_with_duplicates.push(*it),
                ITemplataT::Isa(it) => impl_templatas_with_duplicates.push(*it),
                _ => panic!("vwat: unexpected templata in isParent matching"),
            }
        }

        // Check if there's already a compiled IsaTemplataT that directly matches.
        if let Some(impl_isa) = impl_templatas_with_duplicates.iter().find(|i| KindT::from(sub_kind_tt) == i.sub_kind && KindT::from(super_kind_tt) == i.super_kind) {
            coutputs.add_instantiation_bounds(
                self.opts.global_options.sanity_check,
                self.typing_interner,
                calling_env.denizen_template_id(),
                impl_isa.impl_name,
                crate::typing::hinputs_t::make(self.typing_interner, vec![], vec![], vec![]));
            return IsParentResult::IsParent(IsParent {
                templata: ITemplataT::Isa(self.typing_interner.alloc(*impl_isa)),
                conclusions: std::collections::HashMap::new(),
                impl_id: impl_isa.impl_name,
            });
        }

        let mut seen_ranges: std::collections::HashSet<RangeS<'s>> = std::collections::HashSet::new();
        let impl_defs: Vec<ImplDefinitionTemplataT<'s, 't>> = impl_defs_with_duplicates.into_iter()
            .filter(|d| seen_ranges.insert(d.impl_.range))
            .collect();

        let results: Vec<Result<(ImplDefinitionTemplataT<'s, 't>, CompleteResolveSolve<'s, 't>), IResolvingError<'s, 't>>> =
            impl_defs.iter().map(|impl_def| {
                let initial_knowns = vec![
                    InitialKnown { rune: impl_def.impl_.sub_citizen_rune, templata: ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::from(sub_kind_tt) })) },
                    InitialKnown { rune: impl_def.impl_.interface_kind_rune, templata: ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::from(super_kind_tt) })) },
                ];
                self.resolve_impl(coutputs, parent_ranges, call_location, calling_env, &initial_knowns, *impl_def)
                    .map(|ccs| (*impl_def, ccs))
            }).collect();

        let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(|r| r.is_ok());
        assert!(oks.len() <= 1);
        match oks.into_iter().next() {
            Some(Ok((impl_templata, CompleteResolveSolve { conclusions, rune_to_bound }))) => {
                let template_args: Vec<ITemplataT<'s, 't>> =
                    impl_templata.impl_.generic_params.iter().map(|p| *conclusions.get(&p.rune.rune).unwrap()).collect();
                let impl_template_name: INameT<'s, 't> = match self.translate_impl_name(impl_templata.impl_.name) {
                    IImplTemplateNameT::ImplTemplate(r) => INameT::ImplTemplate(r),
                    IImplTemplateNameT::ImplBoundTemplate(_) => panic!("Unimplemented: ImplBoundTemplate in isParent"),
                    IImplTemplateNameT::AnonymousSubstructImplTemplate(r) => INameT::AnonymousSubstructImplTemplate(r),
                };
                let impl_template_id = impl_templata.env.id().add_step(self.typing_interner, impl_template_name);
                let instantiated_id = self.assemble_impl_name(*impl_template_id, &template_args, sub_kind_tt.expect_citizen());
                coutputs.add_instantiation_bounds(
                    self.opts.global_options.sanity_check,
                    self.typing_interner,
                    calling_env.root_compiling_denizen_env().denizen_template_id(),
                    instantiated_id,
                    rune_to_bound);
                IsParentResult::IsParent(IsParent {
                    templata: ITemplataT::ImplDefinition(self.typing_interner.alloc(impl_templata)),
                    conclusions,
                    impl_id: instantiated_id,
                })
            }
            Some(Err(_)) => unreachable!(),
            None => {
                let err_vec: Vec<IResolvingError<'s, 't>> = errs.into_iter().map(|r| match r { Err(e) => e, Ok(_) => unreachable!() }).collect();
                IsParentResult::IsntParent(IsntParent { candidates: err_vec })
            }
        }
    }
/*
  def isParent(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    subKindTT: ISubKindTT,
    superKindTT: ISuperKindTT):
  IsParentResult = {
    val superKindImpreciseName =
      TemplatasStore.getImpreciseName(interner, superKindTT.id.localName) match {
        case None => return IsntParent(Vector())
        case Some(n) => n
      }
    val subKindImpreciseName =
      TemplatasStore.getImpreciseName(interner, subKindTT.id.localName) match {
        case None => return IsntParent(Vector())
        case Some(n) => n
      }
    val implImpreciseNameS =
      interner.intern(ImplImpreciseNameS(subKindImpreciseName, superKindImpreciseName))

    val subKindEnv =
      coutputs.getOuterEnvForType(
        parentRanges, TemplataCompiler.getSubKindTemplate(subKindTT.id))
    val superKindEnv =
      coutputs.getOuterEnvForType(
        parentRanges, TemplataCompiler.getSuperKindTemplate(superKindTT.id))

    val matching =
      callingEnv.lookupAllWithImpreciseName(implImpreciseNameS, Set(TemplataLookupContext)) ++
      subKindEnv.lookupAllWithImpreciseName(implImpreciseNameS, Set(TemplataLookupContext)) ++
      superKindEnv.lookupAllWithImpreciseName(implImpreciseNameS, Set(TemplataLookupContext))

    val implsDefsWithDuplicates = new Accumulator[ImplDefinitionTemplataT]()
    val implTemplatasWithDuplicatesAcc = new Accumulator[IsaTemplataT]()
    matching.foreach({
      case it@ImplDefinitionTemplataT(_, _) => implsDefsWithDuplicates.add(it)
      case it@IsaTemplataT(_, _, _, _) => implTemplatasWithDuplicatesAcc.add(it)
      case _ => vwat()
    })
    val implTemplatasWithDuplicates = implTemplatasWithDuplicatesAcc.buildArray()

    implTemplatasWithDuplicates.find(i => i.subKind == subKindTT && i.superKind == superKindTT) match {
      case Some(impl) => {
        coutputs.addInstantiationBounds(
          opts.globalOptions.sanityCheck,
          interner,
          callingEnv.denizenTemplateId,
          impl.implName, InstantiationBoundArgumentsT.make(Map(), Map(), Map()))
        return IsParent(impl, Map(), impl.implName)
      }
      case None =>
    }

    val impls =
      implsDefsWithDuplicates.buildArray().groupBy(_.impl.range).map(_._2.head)
    val results =
      impls.map(impl => {
        val initialKnowns =
          Vector(
            InitialKnown(impl.impl.subCitizenRune, KindTemplataT(subKindTT)),
            InitialKnown(impl.impl.interfaceKindRune, KindTemplataT(superKindTT)))
        resolveImpl(coutputs, parentRanges, callLocation, callingEnv, initialKnowns, impl) match {
          case Ok(ccs) => Ok((impl, ccs))
          case Err(x) => Err(x)
        }
      })
    val (oks, errs) = Result.split(results)
    vcurious(oks.size <= 1)
    oks.headOption match {
      case Some((implTemplata, CompleteResolveSolve(conclusions, runeToSuppliedFunction))) => {
        val templateArgs =
          implTemplata.impl.genericParams.map(_.rune.rune).map(conclusions)
        val implTemplateId =
          implTemplata.env.id.addStep(
            nameTranslator.translateImplName(implTemplata.impl.name))
        val instantiatedId = assembleImplName(implTemplateId, templateArgs, subKindTT.expectCitizen())
        coutputs.addInstantiationBounds(
          opts.globalOptions.sanityCheck,
          interner, callingEnv.rootCompilingDenizenEnv.denizenTemplateId,
          instantiatedId, runeToSuppliedFunction)
        IsParent(implTemplata, conclusions, instantiatedId)
      }
      case None => IsntParent(errs.toVector)
    }
  }
}
*/
}