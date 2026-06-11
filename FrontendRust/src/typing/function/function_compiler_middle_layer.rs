use std::collections::HashSet;
use crate::utils::range::RangeS;

use crate::postparsing::names::*;
use crate::higher_typing::ast::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::ast::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::postparsing::ast::{LocationInDenizen, ParameterS};
use crate::postparsing::ast::AbstractSP;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::compiler::Compiler;
use crate::typing::typing_interner::MustIntern;
use crate::typing::types::types::KindT;
use crate::typing::ast::ast::AbstractT;
use crate::typing::names::names::TypingIgnoredParamNameT;
use std::iter::once;
use std::marker::PhantomData;

/*
package dev.vale.typing.function

import dev.vale.{Interner, Keywords, Profiler, RangeS, vassert, vassertSome, vcurious, vfail, vimpl, vwat}
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing._
import dev.vale.postparsing.patterns._
import dev.vale.typing.{AbstractMethodOutsideOpenInterface, CompileErrorExceptionT, CompilerOutputs, ConvertHelper, FunctionAlreadyExists, RangedInternalErrorT, TemplataCompiler, TypingPassOptions, ast, env}
import dev.vale.typing.ast._
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.env.{BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT, ExpressionLookupContext, FunctionEnvironmentT, IInDenizenEnvironmentT, TemplataLookupContext}
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.names._
import dev.vale.typing.templata.CoordTemplataT
import dev.vale.typing.types._
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.postparsing.patterns._
import dev.vale.typing.{ast, names, _}
import dev.vale.typing.ast._
import dev.vale.typing.env._

import scala.collection.immutable.{List, Set}

*/
/*
class FunctionCompilerMiddleLayer(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    nameTranslator: NameTranslator,
    templataCompiler: TemplataCompiler,
    convertHelper: ConvertHelper,
    structCompiler: StructCompiler,
    delegate: IFunctionCompilerDelegate) {
  val core = new FunctionCompilerCore(opts, interner, keywords, nameTranslator, templataCompiler, convertHelper, delegate)

//  // This is for the early stages of Compiler when it's scanning banners to put in
//  // its env. We just want its banner, we don't want to evaluate it.
//  // Preconditions:
//  // - already spawned local env
//  // - either no template args, or they were already added to the env.
//  // - either no closured vars, or they were already added to the env.
//  def predictOrdinaryFunctionBanner(
//    runedEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgs,
//    coutputs: CompilerOutputs,
//    parentRanges: List[RangeS],
//    function1: FunctionA):
//  (FunctionBannerT) = {
//
//    // Check preconditions
//    function1.runeToType.keySet.foreach(templateParam => {
//      vassert(runedEnv.lookupNearestWithImpreciseName(vimpl(templateParam.toString), Set(TemplataLookupContext, ExpressionLookupContext)).nonEmpty)
//    })
//    function1.body match {
//      case CodeBodyS(body1) => vassert(body1.closuredNames.isEmpty)
//      case _ =>
//    }
//
//    val params2 = assembleFunctionParams(runedEnv, coutputs, parentRanges, function1.params)
//    val maybeReturnType = getMaybeReturnType(runedEnv, function1.maybeRetCoordRune.map(_.rune))
//    val namedEnv = makeNamedEnv(runedEnv, params2.map(_.tyype), maybeReturnType)
//    val banner = FunctionBannerT(Some(function1), namedEnv.fullName)
//    banner
//  }

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_maybe_virtuality(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        param_kind: &KindT<'s, 't>,
        maybe_virtuality: Option<&AbstractSP<'s>>,
    ) -> Result<Option<AbstractT>, ICompileErrorT<'s, 't>> {
        match maybe_virtuality {
            None => Ok(None),
            Some(abstract_sp) => {
                let interface_tt = match param_kind {
                    KindT::Interface(i) => i,
                    _ => panic!("RangedInternalErrorT: Can only have virtual parameters for interfaces"),
                };
                // Open (non-sealed) interfaces can't have abstract methods defined outside the interface.
                // See https://github.com/ValeLang/Vale/issues/374
                if !abstract_sp.is_internal_method {
                    let interface_template = self.get_interface_template(interface_tt.id);
                    if !coutputs.lookup_sealed(interface_template) {
                        if env.id().init_steps != &interface_template.steps()[..] {
                            let ranges: Vec<RangeS<'s>> =
                                once(abstract_sp.range).chain(parent_ranges.iter().copied()).collect();
                            let ranges_t = self.typing_interner.alloc_slice_copy(&ranges);
                            return Err(ICompileErrorT::AbstractMethodOutsideOpenInterface { range: ranges_t });
                        }
                    }
                }
                Ok(Some(AbstractT))
            }
        }
    }

/*
  private def evaluateMaybeVirtuality(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    paramKind: KindT,
    maybeVirtuality1: Option[AbstractSP]):
  (Option[AbstractT]) = {
    maybeVirtuality1 match {
      case None => (None)
      case Some(AbstractSP(rangeS, isInternalMethod)) => {
        val interfaceTT =
          paramKind match {
            case i @ InterfaceTT(_) => i
            case _ => throw CompileErrorExceptionT(RangedInternalErrorT(rangeS :: parentRanges, "Can only have virtual parameters for interfaces"))
          }
        // Open (non-sealed) interfaces can't have abstract methods defined outside the interface.
        // See https://github.com/ValeLang/Vale/issues/374
        if (!isInternalMethod) {
          if (!coutputs.lookupSealed(TemplataCompiler.getInterfaceTemplate(interfaceTT.id))) {
            // Macros can put e.g. functions inside an interface by prefixing the function name
            // with the interface.
            // For example, InterfaceFreeMacro will look at mymod.MyInterface and conjure a
            // mymod.MyInterface.free function.
            if (env.id.steps.init != TemplataCompiler.getInterfaceTemplate(interfaceTT.id).steps) {
              throw CompileErrorExceptionT(AbstractMethodOutsideOpenInterface(rangeS :: parentRanges))
            }
          }
        }
        (Some(AbstractT()))
      }
//      case Some(OverrideSP(range, interfaceRuneA)) => {
//        val interface =
//          env.lookupNearestWithImpreciseName(interner.intern(RuneNameS(interfaceRuneA.rune)), Set(TemplataLookupContext)) match {
//            case None => vcurious()
//            case Some(KindTemplata(ir @ InterfaceTT(_))) => ir
//            case Some(it @ InterfaceTemplata(_, _)) => structCompiler.getInterfaceRef(coutputs, range, it, Vector.empty)
//            case Some(KindTemplata(kind)) => {
//              throw CompileErrorExceptionT(CantImplNonInterface(range, kind))
//            }
//            case _ => vwat()
//          }
//        Some(OverrideT(interface))
//      }
    }
  }

*/
    pub fn get_or_evaluate_templated_function_for_banner(
        &self,
        outer_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        rued_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function1: &FunctionA<'s>,
        instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> Result<PrototypeTemplataT<'s, 't>, ICompileErrorT<'s, 't>> {
        // Check preconditions
        let rued_env_as_i = IInDenizenEnvironmentT::BuildingWithClosuredsAndTemplateArgs(rued_env);
        for template_param in function1.rune_to_type.keys() {
            let imprecise_name = self.scout_arena.intern_imprecise_name(
                IImpreciseNameValS::RuneName(RuneNameValS { rune: *template_param }));
            let mut lookup_filter = HashSet::new();
            lookup_filter.insert(ILookupContext::TemplataLookupContext);
            lookup_filter.insert(ILookupContext::ExpressionLookupContext);
            assert!(
                rued_env_as_i.lookup_nearest_with_imprecise_name(imprecise_name, lookup_filter, self.typing_interner).is_some());
        }
        let params2 = self.assemble_function_params(rued_env_as_i, coutputs, call_range, &function1.params)?;

        let maybe_return_type = self.get_maybe_return_type(rued_env, function1.maybe_ret_coord_rune.as_ref().map(|r| &r.rune));
        let param_types: Vec<CoordT<'s, 't>> = params2.iter().map(|p| p.tyype).collect();
        let named_env: &'t FunctionEnvironmentT<'s, 't> =
            self.typing_interner.alloc(self.make_named_env(rued_env, &param_types, maybe_return_type));
        let banner = FunctionBannerT {
            origin_function_templata: Some(named_env.templata()),
            name: named_env.id,
        };

        let signature = self.typing_interner.alloc(SignatureT { id: banner.name });
        match coutputs.lookup_function(signature) {
            Some(function_def) => {
                Ok(PrototypeTemplataT { prototype: self.typing_interner.alloc(function_def.header.to_prototype()) })
            }
            None => {
                coutputs.declare_function(call_range, &named_env.id);
                let outer_env_as_i: IInDenizenEnvironmentT<'s, 't> =
                    IInDenizenEnvironmentT::BuildingWithClosureds(outer_env);
                coutputs.declare_function_outer_env(&outer_env.id, outer_env_as_i);
                let named_env_as_i: IInDenizenEnvironmentT<'s, 't> =
                    IInDenizenEnvironmentT::Function(named_env);
                coutputs.declare_function_inner_env(&named_env.id, named_env_as_i);

                let header =
                    self.evaluate_function_for_header_core(named_env, coutputs, call_range, call_location, &params2, instantiation_bound_params)?;
                if !header.to_banner().same(&banner) {
                    panic!("wut: banner mismatch in get_or_evaluate_templated_function_for_banner");
                }

                Ok(PrototypeTemplataT { prototype: self.typing_interner.alloc(header.to_prototype()) })
            }
        }
    }

/*
  // Preconditions:
  // - already spawned local env
  // - either no template args, or they were already added to the env.
  // - either no closured vars, or they were already added to the env.
  def getOrEvaluateTemplatedFunctionForBanner(
    outerEnv: BuildingFunctionEnvironmentWithClosuredsT,
    runedEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    function1: FunctionA,
    instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]):
  (PrototypeTemplataT[IFunctionNameT]) = {
    // Check preconditions
    function1.runeToType.keySet.foreach(templateParam => {
      vassert(runedEnv.lookupNearestWithImpreciseName(interner.intern(RuneNameS(templateParam)), Set(TemplataLookupContext, ExpressionLookupContext)).nonEmpty);
    })

    val params2 = assembleFunctionParams(runedEnv, coutputs, callRange, function1.params)

    val maybeReturnType = getMaybeReturnType(runedEnv, function1.maybeRetCoordRune.map(_.rune))
    val namedEnv = makeNamedEnv(runedEnv, params2.map(_.tyype), maybeReturnType)
    val banner = ast.FunctionBannerT(Some(namedEnv.templata), namedEnv.id)//, params2)

    coutputs.lookupFunction(SignatureT(banner.name)) match {
      case Some(FunctionDefinitionT(header, _, _)) => {
        PrototypeTemplataT(header.toPrototype)
      }
      case None => {
        coutputs.declareFunction(callRange, namedEnv.id)
        coutputs.declareFunctionOuterEnv(outerEnv.id, outerEnv)
        coutputs.declareFunctionInnerEnv(namedEnv.id, namedEnv)

        val header =
          core.evaluateFunctionForHeader(namedEnv, coutputs, callRange, callLocation, params2, instantiationBoundParams)
        if (!header.toBanner.same(banner)) {
          val bannerFromHeader = header.toBanner
          vfail("wut\n" + bannerFromHeader + "\n" + banner)
        }

        //        delegate.evaluateParent(namedEnv, coutputs, callRange, header)

        PrototypeTemplataT(header.toPrototype)
      }
    }
  }

*/
    pub fn get_or_evaluate_function_for_header(
        &self,
        outer_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        rued_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function1: &FunctionA<'s>,
        instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> Result<&'t FunctionHeaderT<'s, 't>, ICompileErrorT<'s, 't>> {
        // Check preconditions
        // function1.runeToType.keySet.foreach(rune => {
        //   vassert(
        //     runedEnv.lookupNearestWithImpreciseName(
        //       interner.intern(RuneNameS(rune)),
        //       Set(TemplataLookupContext, ExpressionLookupContext)).nonEmpty)
        // })
        let rued_env_as_i: IInDenizenEnvironmentT<'s, 't> = IInDenizenEnvironmentT::BuildingWithClosuredsAndTemplateArgs(rued_env);
        for rune in function1.rune_to_type.keys() {
            // vassert(runedEnv.lookupNearestWithImpreciseName(
            //   interner.intern(RuneNameS(rune)), Set(TemplataLookupContext, ExpressionLookupContext)).nonEmpty)
            let imprecise_name = self.scout_arena.intern_imprecise_name(
                IImpreciseNameValS::RuneName(RuneNameValS { rune: *rune }));
            let mut lookup_filter = HashSet::new();
            lookup_filter.insert(ILookupContext::TemplataLookupContext);
            lookup_filter.insert(ILookupContext::ExpressionLookupContext);
            assert!(
                rued_env_as_i.lookup_nearest_with_imprecise_name(imprecise_name, lookup_filter, self.typing_interner).is_some());
        }

        // val paramTypes2 = evaluateFunctionParamTypes(runedEnv, function1.params);
        let param_types2 = self.evaluate_function_param_types(rued_env_as_i, &function1.params);

        // val functionId = assembleName(runedEnv.id, runedEnv.templateArgs, paramTypes2)
        let function_id = self.assemble_name(&rued_env.id, rued_env.template_args, &param_types2);

        // val needleSignature = SignatureT(functionId)
        let needle_signature = self.typing_interner.intern_signature(SignatureValT {
            id: IdValT {
                package_coord: function_id.package_coord,
                init_steps: function_id.init_steps,
                local_name: function_id.local_name,
            },
        });

        // coutputs.lookupFunction(needleSignature) match {
        match coutputs.lookup_function(needle_signature) {
            //   case Some(FunctionDefinitionT(header, _, _)) => { (header) }
            Some(func_def) => {
                Ok(&func_def.header)
            }
            //   case None => {
            None => {
                // coutputs.declareFunction(callRange, functionId)
                let function_id_ref = self.typing_interner.intern_id(IdValT {
                    package_coord: function_id.package_coord,
                    init_steps: function_id.init_steps,
                    local_name: function_id.local_name,
                });
                coutputs.declare_function(call_range, function_id_ref);

                // coutputs.declareFunctionOuterEnv(outerEnv.id, outerEnv)
                let outer_env_id_ref = self.typing_interner.intern_id(IdValT {
                    package_coord: outer_env.id.package_coord,
                    init_steps: outer_env.id.init_steps,
                    local_name: outer_env.id.local_name,
                });
                let outer_env_as_i: IInDenizenEnvironmentT<'s, 't> =
                    IInDenizenEnvironmentT::BuildingWithClosureds(outer_env);
                coutputs.declare_function_outer_env(outer_env_id_ref, outer_env_as_i);

                // val params2 = assembleFunctionParams(runedEnv, coutputs, callRange, function1.params)
                let params2 = self.assemble_function_params(rued_env_as_i, coutputs, call_range, &function1.params)?;

                // val maybeReturnType = getMaybeReturnType(runedEnv, function1.maybeRetCoordRune.map(_.rune))
                let maybe_return_type = self.get_maybe_return_type(rued_env, function1.maybe_ret_coord_rune.as_ref().map(|r| &r.rune));

                // val namedEnv = makeNamedEnv(runedEnv, params2.map(_.tyype), maybeReturnType)
                let param_types_for_env: Vec<CoordT<'s, 't>> = params2.iter().map(|p| p.tyype).collect();
                let named_env = self.make_named_env(rued_env, &param_types_for_env, maybe_return_type);

                // coutputs.declareFunctionInnerEnv(functionId, namedEnv)
                let named_env_ref: &'t FunctionEnvironmentT<'s, 't> = self.typing_interner.alloc(named_env);
                let named_env_as_i: IInDenizenEnvironmentT<'s, 't> =
                    IInDenizenEnvironmentT::Function(named_env_ref);
                coutputs.declare_function_inner_env(function_id_ref, named_env_as_i);

                // val header = core.evaluateFunctionForHeader(namedEnv, coutputs, callRange, callLocation, params2, instantiationBoundParams)
                let header = self.evaluate_function_for_header_core(
                    named_env_ref, coutputs, call_range, call_location, &params2, instantiation_bound_params)?;

                // vassert(header.toSignature == needleSignature)
                let header_sig = header.to_signature();
                assert!(header_sig.id == needle_signature.id);

                // (header)
                Ok(self.typing_interner.alloc(header))
            }
        }
    }

/*
  // Preconditions:
  // - already spawned local env
  // - either no template args, or they were already added to the env.
  // - either no closured vars, or they were already added to the env.
  def getOrEvaluateFunctionForHeader(
    outerEnv: BuildingFunctionEnvironmentWithClosuredsT,
    runedEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    function1: FunctionA,
    instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]):
  (FunctionHeaderT) = {

    // Check preconditions
    function1.runeToType.keySet.foreach(rune => {
      vassert(
        runedEnv
          .lookupNearestWithImpreciseName(
            interner.intern(RuneNameS(rune)),
            Set(TemplataLookupContext, ExpressionLookupContext))
          .nonEmpty)
    })

    val paramTypes2 = evaluateFunctionParamTypes(runedEnv, function1.params);

    val functionId = assembleName(runedEnv.id, runedEnv.templateArgs, paramTypes2)
    val needleSignature = SignatureT(functionId)
    coutputs.lookupFunction(needleSignature) match {
      case Some(FunctionDefinitionT(header, _, _)) => {
        (header)
      }
      case None => {
        coutputs.declareFunction(callRange, functionId)
        coutputs.declareFunctionOuterEnv(outerEnv.id, outerEnv)

        val params2 = assembleFunctionParams(runedEnv, coutputs, callRange, function1.params)

        val maybeReturnType = getMaybeReturnType(runedEnv, function1.maybeRetCoordRune.map(_.rune))
        val namedEnv = makeNamedEnv(runedEnv, params2.map(_.tyype), maybeReturnType)

        coutputs.declareFunctionInnerEnv(functionId, namedEnv)

        //        coutputs.declareFunctionSignature(function1.range, needleSignature, Some(namedEnv))

        val header =
          core.evaluateFunctionForHeader(
            namedEnv, coutputs, callRange, callLocation, params2, instantiationBoundParams)
        vassert(header.toSignature == needleSignature)
        (header)
      }
    }
  }

//  // Preconditions:
//  // - already spawned local env
//  // - either no template args, or they were already added to the env.
//  // - either no closured vars, or they were already added to the env.
//  def getOrEvaluateOrdinaryFunctionForPrototype(
//    runedEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgs,
//    coutputs: CompilerOutputs,
//    callRange: List[RangeS],
//    function1: FunctionA):
//  (PrototypeTemplata) = {
//    // Check preconditions
//    function1.runeToType.keySet.foreach(templateParam => {
//      vassert(
//        runedEnv
//          .lookupNearestWithImpreciseName(
//            interner.intern(RuneNameS(templateParam)),
//            Set(TemplataLookupContext, ExpressionLookupContext))
//          .nonEmpty);
//    })
//
//    val paramTypes2 = evaluateFunctionParamTypes(runedEnv, function1.params);
//    val functionId = assembleName(runedEnv.fullName, runedEnv.templateArgs, paramTypes2)
//    val needleSignature = SignatureT(functionId)
//    val p = vassertSome(coutputs.lookupFunction(needleSignature)).header.toPrototype
//    PrototypeTemplata(function1.range, p)
//  }

//  // We would want only the prototype instead of the entire header if, for example,
//  // we were calling the function. This is necessary for a recursive function like
//  // func main():Int{main()}
//  // Preconditions:
//  // - already spawned local env
//  // - either no template args, or they were already added to the env.
//  // - either no closured vars, or they were already added to the env.
//  def getFunctionPrototype(
//    runedEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgs,
//    coutputs: CompilerOutputs,
//    callRange: List[RangeS],
//    function1: FunctionA):
//  (PrototypeT) = {
//
//    // Check preconditions
//    function1.runeToType.keySet.foreach(templateParam => {
//      vassert(
//        runedEnv.lookupNearestWithImpreciseName(
//          interner.intern(RuneNameS(templateParam)),
//          Set(TemplataLookupContext, ExpressionLookupContext)).nonEmpty);
//    })
//
//    val paramTypes2 = evaluateFunctionParamTypes(runedEnv, function1.params)
//    val maybeReturnType = getMaybeReturnType(runedEnv, function1.maybeRetCoordRune.map(_.rune))
//    val namedEnv = makeNamedEnv(runedEnv, paramTypes2, maybeReturnType)
//    val needleSignature = SignatureT(namedEnv.fullName)
//
//    val params2 = assembleFunctionParams(namedEnv, coutputs, function1.params)
//    val prototype =
//      core.getFunctionPrototypeForCall(
//        namedEnv, coutputs, callRange, params2)
//
//    vassert(prototype.toSignature == needleSignature)
//    prototype
//  }



*/
    pub fn evaluate_function_param_types(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        params1: &[ParameterS<'s>],
    ) -> Vec<CoordT<'s, 't>> {
        // params1.map(param1 => {
        //   val CoordTemplataT(coord) =
        //     env.lookupNearestWithImpreciseName(
        //       interner.intern(RuneNameS(param1.pattern.coordRune.get.rune)),
        //       Set(TemplataLookupContext)).get
        //   coord
        // })
        params1.iter().map(|param1| {
            let rune = param1.pattern.coord_rune.as_ref().unwrap().rune;
            let imprecise_name = self.scout_arena.intern_imprecise_name(
                IImpreciseNameValS::RuneName(RuneNameValS { rune }));
            let mut lookup_filter = HashSet::new();
            lookup_filter.insert(ILookupContext::TemplataLookupContext);
            match env.lookup_nearest_with_imprecise_name(imprecise_name, lookup_filter, self.typing_interner).unwrap() {
                ITemplataT::Coord(coord_templata) => coord_templata.coord,
                other => panic!("implement unexpected templata in evaluateFunctionParamTypes: {:?}", other),
            }
        }).collect()
    }

/*
  private def evaluateFunctionParamTypes(
    env: IInDenizenEnvironmentT,
    params1: Vector[ParameterS]):
  Vector[CoordT] = {
    params1.map(param1 => {
      val CoordTemplataT(coord) =
        env
          .lookupNearestWithImpreciseName(
            interner.intern(RuneNameS(param1.pattern.coordRune.get.rune)),
            Set(TemplataLookupContext))
          .get
      coord
    })
  }

*/
    pub fn assemble_function_params(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        params1: &[ParameterS<'s>],
    ) -> Result<Vec<ParameterT<'s, 't>>, ICompileErrorT<'s, 't>> {
        // params1.zipWithIndex.map({ case (param1, index) =>
        params1.iter().enumerate().map(|(index, param1)| {
            //   val CoordTemplataT(coord) = vassertSome(
            //     env.lookupNearestWithImpreciseName(
            //       interner.intern(RuneNameS(param1.pattern.coordRune.get.rune)),
            //       Set(TemplataLookupContext)))
            let rune = param1.pattern.coord_rune.as_ref().unwrap().rune;
            let imprecise_name = self.scout_arena.intern_imprecise_name(
                IImpreciseNameValS::RuneName(RuneNameValS { rune }));
            let mut lookup_filter = HashSet::new();
            lookup_filter.insert(ILookupContext::TemplataLookupContext);
            let coord = match env.lookup_nearest_with_imprecise_name(imprecise_name, lookup_filter, self.typing_interner).unwrap() {
                ITemplataT::Coord(coord_templata) => coord_templata.coord,
                other => panic!("implement unexpected templata in assembleFunctionParams: {:?}", other),
            };

            //   val maybeVirtuality = evaluateMaybeVirtuality(env, coutputs, parentRanges, coord.kind, param1.virtuality)
            let maybe_virtuality = self.evaluate_maybe_virtuality(
                env, coutputs, parent_ranges, &coord.kind, param1.virtuality.as_ref())?;

            //   val nameT = param1.pattern.name match {
            //     case None => interner.intern(TypingIgnoredParamNameT(index))
            //     case Some(x) => nameTranslator.translateVarNameStep(x.name)
            //   }
            let name_t: IVarNameT<'s, 't> = match &param1.pattern.name {
                None => {
                    IVarNameT::TypingIgnoredParam(self.typing_interner.intern_typing_ignored_param_name(TypingIgnoredParamNameT { num: index as i32}))
                }
                Some(x) => {
                    self.translate_var_name_step(x.name)
                }
            };

            //   ParameterT(nameT, maybeVirtuality, param1.preChecked, coord)
            Ok(ParameterT {
                name: name_t,
                virtuality: maybe_virtuality,
                pre_checked: param1.pre_checked,
                tyype: coord,
            })
        }).collect()
    }

/*
  def assembleFunctionParams(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    params1: Vector[ParameterS]):
  (Vector[ParameterT]) = {
    params1.zipWithIndex.map({ case (param1, index) =>
        val CoordTemplataT(coord) =
          vassertSome(
            env
              .lookupNearestWithImpreciseName(

                interner.intern(RuneNameS(param1.pattern.coordRune.get.rune)),
                Set(TemplataLookupContext)))
        val maybeVirtuality =
          evaluateMaybeVirtuality(
            env, coutputs, parentRanges, coord.kind, param1.virtuality)
        val nameT =
          param1.pattern.name match {
            case None => interner.intern(TypingIgnoredParamNameT(index))
            case Some(x) => nameTranslator.translateVarNameStep(x.name)
          }
        ParameterT(nameT, maybeVirtuality, param1.preChecked, coord)
      })
  }

*/
    pub fn get_maybe_return_type(
        &self,
        near_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>,
        maybe_ret_coord_rune: Option<&IRuneS<'s>>,
    ) -> Option<CoordT<'s, 't>> {
        // maybeRetCoordRune.map(retCoordRuneA => {
        //   val retCoordRune = (retCoordRuneA)
        //   nearEnv.lookupNearestWithImpreciseName(interner.intern(RuneNameS(retCoordRune)), Set(TemplataLookupContext)) match {
        //     case Some(CoordTemplataT(coord)) => coord
        //     case other => vwat(retCoordRune, other)
        //   }
        // })
        let near_env_as_i: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::BuildingWithClosuredsAndTemplateArgs(near_env);
        maybe_ret_coord_rune.map(|ret_coord_rune| {
            let imprecise_name = self.scout_arena.intern_imprecise_name(
                IImpreciseNameValS::RuneName(RuneNameValS { rune: *ret_coord_rune }));
            let mut lookup_filter = HashSet::new();
            lookup_filter.insert(ILookupContext::TemplataLookupContext);
            match near_env_as_i.lookup_nearest_with_imprecise_name(imprecise_name, lookup_filter, self.typing_interner) {
                Some(ITemplataT::Coord(coord_templata)) => coord_templata.coord,
                other => panic!("implement vwat in getMaybeReturnType: {:?}", other),
            }
        })
    }

/*
  private def getMaybeReturnType(
    nearEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT,
    maybeRetCoordRune: Option[IRuneS]
  ): Option[CoordT] = {
    maybeRetCoordRune.map(retCoordRuneA => {
      val retCoordRune = (retCoordRuneA)
      nearEnv.lookupNearestWithImpreciseName(interner.intern(RuneNameS(retCoordRune)), Set(TemplataLookupContext)) match {
        case Some(CoordTemplataT(coord)) => coord
        case other => vwat(retCoordRune, other)
      }
    })
  }

*/
    pub fn get_generic_function_banner_from_call(
        &self,
        rued_env: &BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>,
        coutputs: &CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        function_templata: &FunctionTemplataT<'s, 't>,
    ) -> FunctionBannerT<'s, 't> {
        panic!("Unimplemented: get_generic_function_banner_from_call");
    }

/*
  // Preconditions:
  // - already spawned local env
  // - either no template args, or they were already added to the env.
  // - either no closured vars, or they were already added to the env.
  def getGenericFunctionBannerFromCall(
    runedEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    functionTemplata: FunctionTemplataT):
  (FunctionBannerT) = {
    val function1 = functionTemplata.function

    // Check preconditions
    function1.runeToType.keySet.foreach(templateParam => {
      vassert(runedEnv.lookupNearestWithImpreciseName(interner.intern(RuneNameS(templateParam)), Set(TemplataLookupContext, ExpressionLookupContext)).nonEmpty);
    })

    val params2 = assembleFunctionParams(runedEnv, coutputs, callRange, function1.params)

    val maybeReturnType = getMaybeReturnType(runedEnv, function1.maybeRetCoordRune.map(_.rune))
    val namedEnv = makeNamedEnv(runedEnv, params2.map(_.tyype), maybeReturnType)
    val banner = ast.FunctionBannerT(Some(functionTemplata), namedEnv.id)//, params2)
    banner
  }

*/
    pub fn get_generic_function_prototype_from_call(
        &self,
        rued_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>,
        coutputs: &CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        function1: &FunctionA<'s>,
    ) -> Result<PrototypeT<'s, 't>, ICompileErrorT<'s, 't>> {
        // Check preconditions
        for (template_param, _) in function1.rune_to_type.iter() {
            let imprecise_name = self.scout_arena.intern_imprecise_name(
                IImpreciseNameValS::RuneName(RuneNameValS { rune: *template_param }));
            let mut lookup_filter = HashSet::new();
            lookup_filter.insert(ILookupContext::TemplataLookupContext);
            lookup_filter.insert(ILookupContext::ExpressionLookupContext);
            let rued_env_as_i = IInDenizenEnvironmentT::BuildingWithClosuredsAndTemplateArgs(rued_env);
            assert!(rued_env_as_i.lookup_nearest_with_imprecise_name(imprecise_name, lookup_filter, self.typing_interner).is_some());
        }

        let rued_env_as_i = IInDenizenEnvironmentT::BuildingWithClosuredsAndTemplateArgs(rued_env);
        let param_types = self.evaluate_function_param_types(rued_env_as_i, function1.params);
        let maybe_return_type = self.get_maybe_return_type(rued_env, function1.maybe_ret_coord_rune.as_ref().map(|ru| &ru.rune));
        let named_env = self.typing_interner.alloc(self.make_named_env(rued_env, &param_types, maybe_return_type));
        let needle_signature = SignatureT { id: named_env.id };

        let named_env_as_i = IInDenizenEnvironmentT::Function(named_env);
        let params2 = self.assemble_function_params(named_env_as_i, coutputs, call_range, function1.params)?;

        let prototype = self.get_function_prototype_for_call(
            named_env, coutputs, call_range, &params2);

        assert!(prototype.to_signature() == needle_signature);
        Ok(prototype)
    }

/*
  def getGenericFunctionPrototypeFromCall(
    runedEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    function1: FunctionA):
  (PrototypeT[IFunctionNameT]) = {

    // Check preconditions
    function1.runeToType.keySet.foreach(templateParam => {
      vassert(
        runedEnv.lookupNearestWithImpreciseName(
          interner.intern(RuneNameS(templateParam)),
          Set(TemplataLookupContext, ExpressionLookupContext)).nonEmpty);
    })

    val paramTypes2 = evaluateFunctionParamTypes(runedEnv, function1.params)
    val maybeReturnType = getMaybeReturnType(runedEnv, function1.maybeRetCoordRune.map(_.rune))
    val namedEnv = makeNamedEnv(runedEnv, paramTypes2, maybeReturnType)
    val needleSignature = SignatureT(namedEnv.id)

    //    coutputs.getDeclaredSignatureOrigin(needleSignature) match {
    //      case None => {
    //        coutputs.declareFunctionSignature(function1.range, needleSignature, Some(namedEnv))
    val params2 = assembleFunctionParams(namedEnv, coutputs, callRange, function1.params)

    val prototype =
      core.getFunctionPrototypeForCall(
        namedEnv, coutputs, callRange, params2)

    vassert(prototype.toSignature == needleSignature)
    (prototype)
    //      }
    //      case Some(existingOriginS) => {
    //        if (existingOriginS != function1.range) {
    //          throw CompileErrorExceptionT(FunctionAlreadyExists(existingOriginS, function1.range, needleSignature))
    //        }
    //        coutputs.getReturnTypeForSignature(needleSignature) match {
    //          case Some(returnType2) => {
    //            (PrototypeT(namedEnv.fullName, returnType2))
    //          }
    //          case None => {
    //            throw CompileErrorExceptionT(RangedInternalErrorT(runedEnv.function.range, "Need return type for " + needleSignature + ", cycle found"))
    //          }
    //        }
    //      }
    //    }
  }

//  // Preconditions:
//  // - already spawned local env
//  // - either no template args, or they were already added to the env.
//  // - either no closured vars, or they were already added to the env.
//  def getOrEvaluateGenericFunction(
//    runedEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgs,
//    coutputs: CompilerOutputs,
//    callRange: List[RangeS],
//    function1: FunctionA):
//  (FunctionHeaderT) = {
//
//    // Check preconditions
//    function1.runeToType.keySet.foreach(templateParam => {
//      vassert(
//        runedEnv
//          .lookupNearestWithImpreciseName(
//            interner.intern(RuneNameS(templateParam)),
//            Set(TemplataLookupContext, ExpressionLookupContext))
//          .nonEmpty);
//    })
//
//    val paramTypes2 = evaluateFunctionParamTypes(runedEnv, function1.params);
//    val functionId = assembleName(runedEnv.fullName, runedEnv.templateArgs, paramTypes2)
//    val needleSignature = SignatureT(functionId)
//    coutputs.lookupFunction(needleSignature) match {
//      case Some(FunctionT(header, _)) => {
//        (header)
//      }
//      case None => {
//        val params2 = assembleFunctionParams(runedEnv, coutputs, function1.params)
//
//        val maybeReturnType = getMaybeReturnType(runedEnv, function1.maybeRetCoordRune.map(_.rune))
////        val namedEnv = makeNamedEnv(runedEnv, params2.map(_.tyype), maybeReturnType)
//
//        //        coutputs.declareFunctionSignature(function1.range, needleSignature, Some(namedEnv))
//
//        val header =
//          core.evaluateFunctionForHeader(
//            runedEnv, coutputs, callRange, params2)
////        vassert(header.toSignature == needleSignature)
//        (header)
//      }
//    }
//    vimpl()
//  }

*/
    pub fn assemble_name(
        &self,
        template_name: &IdT<'s, 't>,
        template_args: &[ITemplataT<'s, 't>],
        param_types: &[CoordT<'s, 't>],
    ) -> IdT<'s, 't> {
        // templateName.copy(localName = templateName.localName.makeFunctionName(interner, keywords, templateArgs, paramTypes))
        let function_template_name: IFunctionTemplateNameT<'s, 't> =
            template_name.local_name.try_into().unwrap();
        let local_name = function_template_name.make_function_name(
            self.typing_interner, self.keywords, template_args, param_types);
        *self.typing_interner.intern_id(IdValT {
            package_coord: template_name.package_coord,
            init_steps: template_name.init_steps,
            local_name,
        })
    }

/*
  def assembleName(
      templateName: IdT[IFunctionTemplateNameT],
      templateArgs: Vector[ITemplataT[ITemplataType]],
      paramTypes: Vector[CoordT]):
  IdT[IFunctionNameT] = {
    templateName.copy(
      localName = templateName.localName.makeFunctionName(interner, keywords, templateArgs, paramTypes))
  }

*/
    pub fn make_named_env(
        &self,
        rued_env: &BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>,
        param_types: &[CoordT<'s, 't>],
        maybe_return_type: Option<CoordT<'s, 't>>,
    ) -> FunctionEnvironmentT<'s, 't> {
        // val BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT(
        //   globalEnv, parentEnv, templateId, templateArgs, templatas,
        //   function, variables, isRootCompilingDenizen, defaultRegion) = runedEnv
        // val id = assembleName(templateId, templateArgs, paramTypes)
        let id = self.assemble_name(&rued_env.id, rued_env.template_args, param_types);
        // FunctionEnvironmentT(globalEnv, parentEnv, templateId, id, templatas, function, maybeReturnType, variables, isRootCompilingDenizen, defaultRegion)
        FunctionEnvironmentT {
            global_env: rued_env.global_env,
            parent_env: rued_env.parent_env,
            template_id: rued_env.id,
            id,
            templatas: rued_env.templatas,
            function: rued_env.function,
            maybe_return_type,
            closured_locals: rued_env.variables,
            is_root_compiling_denizen: rued_env.is_root_compiling_denizen,
            default_region: rued_env.default_region,
        }
    }

/*
  def makeNamedEnv(
    runedEnv: BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT,
    paramTypes: Vector[CoordT],
    maybeReturnType: Option[CoordT]):
  FunctionEnvironmentT = {
    val BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT(
      globalEnv,
      parentEnv,
      templateId,
      templateArgs,
      templatas,
      function,
      variables,
      isRootCompilingDenizen,
      defaultRegion) = runedEnv
    val id = assembleName(templateId, templateArgs, paramTypes)
    FunctionEnvironmentT(
      globalEnv,
      parentEnv,
      templateId,
      id,
      templatas,
      function,
      maybeReturnType,
      variables,
      isRootCompilingDenizen,
      defaultRegion)
  }
}
*/
}
