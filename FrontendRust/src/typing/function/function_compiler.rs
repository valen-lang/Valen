use crate::higher_typing::ast::FunctionA;
use crate::postparsing::ast::{LocationInDenizen, IBodyS};
use crate::postparsing::names::{IFunctionDeclarationNameS, IVarNameS};
use crate::typing::ast::ast::FunctionHeaderT;
use crate::typing::ast::citizens::NormalStructMemberT;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::{IInDenizenEnvironmentT, IEnvironmentT};
use crate::typing::env::function_environment_t::NodeEnvironmentT;
use crate::typing::templata::templata::*;
use crate::typing::types::types::*;
use crate::typing::names::names::*;
use crate::typing::env::environment::ILookupContext;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use std::collections::HashSet;
use crate::utils::range::RangeS;
use crate::typing::ast::citizens::{IMemberTypeT, ReferenceMemberTypeT, AddressMemberTypeT};
use crate::typing::env::function_environment_t::{IVariableT, ReferenceLocalVariableT, AddressibleLocalVariableT, ReferenceClosureVariableT, AddressibleClosureVariableT};
use crate::typing::templata::templata::PrototypeTemplataT;
use crate::postparsing::names::IRuneS;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::infer_compiler::IDefiningError;
use crate::typing::infer_compiler::InitialKnown;
use crate::typing::infer_compiler::IResolvingError;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::overload_resolver::IFindFunctionFailureReason;
use std::collections::HashMap;
use std::marker::PhantomData;

/*
package dev.vale.typing.function

import dev.vale.{Interner, Keywords, Profiler, RangeS, postparsing, vassert, vassertOne, vfail, vimpl, vwat}
import dev.vale.postparsing._
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.highertyping.CouldntSolveRulesA
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.parsing._
import dev.vale.postparsing.RuneTypeSolver
import dev.vale.postparsing.patterns._
import dev.vale.postparsing.rules._
import dev.vale.typing.OverloadResolver.IFindFunctionFailureReason
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.highertyping.FunctionA
import dev.vale.typing.{CompilerOutputs, ConvertHelper, IFunctionGenerator, InferCompiler, InitialKnown, TemplataCompiler, TypingPassOptions}
import dev.vale.typing.ast.{FunctionBannerT, FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT, PrototypeT, ReferenceExpressionTE}
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.env._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.names.LambdaCitizenNameT

import scala.collection.immutable.{List, Set}



*/
// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)
/*
trait IFunctionCompilerDelegate {
  def evaluateBlockStatements(
    coutputs: CompilerOutputs,
    startingNenv: NodeEnvironmentT,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    ranges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    exprs: BlockSE):
  (ReferenceExpressionTE, Set[CoordT])

  def translatePatternList(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    ranges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    patterns1: Vector[AtomSP],
    patternInputExprs2: Vector[ReferenceExpressionTE]):
  ReferenceExpressionTE

//  def evaluateParent(
//    env: IEnvironment, coutputs: CompilerOutputs, callRange: List[RangeS], sparkHeader: FunctionHeaderT):
//  Unit

  def generateFunction(
    functionCompilerCore: FunctionCompilerCore,
    generator: IFunctionGenerator,
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
    // We might be able to move these all into the function environment... maybe....
    originFunction: Option[FunctionA],
    paramCoords: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  FunctionHeaderT
}

*/
pub enum IEvaluateFunctionResult<'s, 't> {
    EvaluateFunctionSuccess(EvaluateFunctionSuccess<'s, 't>),
    EvaluateFunctionFailure(EvaluateFunctionFailure<'s, 't>),
}
/*
trait IEvaluateFunctionResult

*/
pub struct EvaluateFunctionSuccess<'s, 't> {
    pub prototype: &'t PrototypeTemplataT<'s, 't>,
    pub inferences: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
    pub instantiation_bound_args: &'t InstantiationBoundArgumentsT<'s, 't>,
}
/*
case class EvaluateFunctionSuccess(
    prototype: PrototypeTemplataT[IFunctionNameT],
    inferences: Map[IRuneS, ITemplataT[ITemplataType]],
    instantiationBoundArgs: InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]
) extends IEvaluateFunctionResult

*/
pub struct EvaluateFunctionFailure<'s, 't> {
    pub reason: IDefiningError<'s, 't>,
}
/*
case class EvaluateFunctionFailure(
    reason: IDefiningError
) extends IEvaluateFunctionResult

*/
pub enum IDefineFunctionResult<'s, 't> {
    DefineFunctionSuccess(DefineFunctionSuccess<'s, 't>),
    DefineFunctionFailure(DefineFunctionFailure<'s, 't>),
}
/*
trait IDefineFunctionResult

*/
pub struct DefineFunctionSuccess<'s, 't> {
    pub prototype: &'t PrototypeTemplataT<'s, 't>,
    pub inferences: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
    pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
}
/*
case class DefineFunctionSuccess(
    prototype: PrototypeTemplataT[IFunctionNameT],
    inferences: Map[IRuneS, ITemplataT[ITemplataType]],
    instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]
) extends IDefineFunctionResult

*/
pub struct DefineFunctionFailure<'s, 't> {
    pub reason: IDefiningError<'s, 't>,
}
/*
case class DefineFunctionFailure(
    reason: IDefiningError
) extends IDefineFunctionResult


*/
pub enum IResolveFunctionResult<'s, 't> {
    ResolveFunctionSuccess(ResolveFunctionSuccess<'s, 't>),
    ResolveFunctionFailure(ResolveFunctionFailure<'s, 't>),
}
/*
trait IResolveFunctionResult

*/
pub struct ResolveFunctionSuccess<'s, 't> {
    pub prototype: &'t PrototypeTemplataT<'s, 't>,
    pub inferences: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
}
/*
case class ResolveFunctionSuccess(
    prototype: PrototypeTemplataT[IFunctionNameT],
    inferences: Map[IRuneS, ITemplataT[ITemplataType]]
) extends IResolveFunctionResult

*/
pub struct ResolveFunctionFailure<'s, 't> {
    pub reason: IResolvingError<'s, 't>,
}
/*
case class ResolveFunctionFailure(
    reason: IResolvingError
) extends IResolveFunctionResult


*/
pub enum IStampFunctionResult<'s, 't> {
    StampFunctionSuccess(StampFunctionSuccess<'s, 't>),
    StampFunctionFailure(StampFunctionFailure<'s, 't>),
}
/*
trait IStampFunctionResult

*/
pub struct StampFunctionSuccess<'s, 't> {
    pub prototype: &'t PrototypeT<'s, 't>,
    pub inferences: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
}
/*
case class StampFunctionSuccess(
  prototype: PrototypeT[IFunctionNameT],
  inferences: Map[IRuneS, ITemplataT[ITemplataType]]
) extends IStampFunctionResult

*/
pub struct StampFunctionFailure<'s, 't> {
    pub reason: IFindFunctionFailureReason<'s, 't>,
}
/*
case class StampFunctionFailure(
  reason: IFindFunctionFailureReason
) extends IStampFunctionResult


*/
/*
// When typingpassing a function, these things need to happen:
// - Spawn a local environment for the function
// - Add any closure args to the environment
// - Incorporate any template arguments into the environment
// There's a layer to take care of each of these things.
// This file is the outer layer, which spawns a local environment for the function.
class FunctionCompiler(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    nameTranslator: NameTranslator,
    templataCompiler: TemplataCompiler,
    inferCompiler: InferCompiler,
    convertHelper: ConvertHelper,
    structCompiler: StructCompiler,
    delegate: IFunctionCompilerDelegate) {
  val closureOrLightLayer =
    new FunctionCompilerClosureOrLightLayer(
      opts, interner, keywords, nameTranslator, templataCompiler, inferCompiler, convertHelper, structCompiler, delegate)

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_function_from_non_call(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function_templata: FunctionTemplataT<'s, 't>,
    ) -> Result<&'t FunctionHeaderT<'s, 't>, ICompileErrorT<'s, 't>> {
        let env = function_templata.outer_env;
        let function = function_templata.function;
        if function.is_light() {
            let mut new_ranges: Vec<RangeS<'s>> = Vec::with_capacity(1 + parent_ranges.len());
            new_ranges.push(function.range);
            new_ranges.extend_from_slice(parent_ranges);
            self.evaluate_generic_light_function_from_non_call(
                env, coutputs, &new_ranges, call_location, function)
        } else {
            panic!("vfail: I think we need a call to evaluate a lambda?")
        }
    }
/*
  // We would want only the prototype instead of the entire header if, for example,
  // we were calling the function. This is necessary for a recursive function like
  // func main():Int{main()}
  def evaluateGenericFunctionFromNonCall(
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    functionTemplata: FunctionTemplataT):
  (FunctionHeaderT) = {
    Profiler.frame(() => {
      val FunctionTemplataT(env, function) = functionTemplata
      if (function.isLight) {
        closureOrLightLayer.evaluateGenericLightFunctionFromNonCall(
          env, coutputs, function.range :: parentRanges, callLocation, function)
      } else {
        vfail() // I think we need a call to evaluate a lambda?
      }
    })

  }

*/
    pub fn evaluate_templated_light_function_from_call_for_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function_templata: FunctionTemplataT<'s, 't>,
        already_specified_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        arg_types: &[CoordT<'s, 't>],
    ) -> IEvaluateFunctionResult<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
        // Profiler.frame(() => {
        //   val FunctionTemplataT(declaringEnv, function) = functionTemplata
        //   closureOrLightLayer.evaluateTemplatedLightBannerFromCall(
        //     declaringEnv, coutputs, callingEnv, callRange, callLocation, function,
        //     alreadySpecifiedTemplateArgs, contextRegion, argTypes)
        // })
    }
/*
  def evaluateTemplatedLightFunctionFromCallForPrototype(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    functionTemplata: FunctionTemplataT,
    alreadySpecifiedTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    argTypes: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    Profiler.frame(() => {
      val FunctionTemplataT(declaringEnv, function) = functionTemplata
      closureOrLightLayer.evaluateTemplatedLightBannerFromCall(
        declaringEnv,
        coutputs,
        callingEnv, // See CSSNCE
        callRange, callLocation, function, alreadySpecifiedTemplateArgs, contextRegion, argTypes)
    })
  }

*/
    pub fn evaluate_templated_function_from_call_for_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function_templata: FunctionTemplataT<'s, 't>,
        already_specified_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        arg_types: &[CoordT<'s, 't>],
    ) -> Result<IEvaluateFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        let FunctionTemplataT { outer_env: declaring_env, function } = function_templata;
        if function.is_light() {
            self.evaluate_templated_light_banner_from_call_closure_or_light(
                declaring_env, coutputs, calling_env, call_range, call_location,
                function, already_specified_template_args, context_region, arg_types)
        } else {
            let lambda_citizen_name_2 =
                match function.name {
                    IFunctionDeclarationNameS::LambdaDeclarationName(lambda_name) => {
                        INameT::LambdaCitizen(self.typing_interner.alloc(LambdaCitizenNameT {
                            template: self.typing_interner.alloc(LambdaCitizenTemplateNameT {
                                code_location: lambda_name.code_location,
                            }),
                        }))
                    }
                    _ => { panic!("vwat"); }
                };

            let lookup_result = declaring_env.lookup_nearest_with_name(
                lambda_citizen_name_2,
                HashSet::from([ILookupContext::TemplataLookupContext]),
                self.typing_interner);
            let closure_struct_ref: StructTT<'s, 't> = match lookup_result {
                Some(ITemplataT::Kind(KindTemplataT { kind: KindT::Struct(s) })) => **s,
                _ => {
                    panic!("Unimplemented: evaluateTemplatedFunctionFromCallForPrototype lookup failed");
                    // vassertSome(...) — pattern that would vfail on absence
                }
            };

            let banner = self.evaluate_templated_closure_function_from_call_for_banner(
                declaring_env, coutputs, calling_env, call_range, call_location,
                closure_struct_ref, function, already_specified_template_args,
                context_region, arg_types);
            banner
        }
    }
/*
  // Per @LAGTNGZ, the isLight branch routes top-level functions through the generic path and closures through the template path.
  def evaluateTemplatedFunctionFromCallForPrototype(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    functionTemplata: FunctionTemplataT,
    alreadySpecifiedTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    argTypes: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    Profiler.frame(() => {
      val FunctionTemplataT(declaringEnv, function) = functionTemplata
      if (function.isLight()) {
        closureOrLightLayer.evaluateTemplatedLightBannerFromCall(
          declaringEnv,
          coutputs,
          callingEnv, // See CSSNCE
          callRange, callLocation, function, alreadySpecifiedTemplateArgs, contextRegion, argTypes)
      } else {
        val lambdaCitizenName2 =
          functionTemplata.function.name match {
            case LambdaDeclarationNameS(codeLocation) => interner.intern(LambdaCitizenNameT(interner.intern(LambdaCitizenTemplateNameT(nameTranslator.translateCodeLocation(codeLocation)))))
            case _ => vwat()
          }

        val KindTemplataT(closureStructRef@StructTT(_)) =
          vassertOne(
            declaringEnv.lookupNearestWithName(
              lambdaCitizenName2,
              Set(TemplataLookupContext)))
        val banner =
          closureOrLightLayer.evaluateTemplatedClosureFunctionFromCallForBanner(
            declaringEnv, coutputs, callingEnv, callRange, callLocation, closureStructRef, function,
            alreadySpecifiedTemplateArgs, contextRegion, argTypes)
        (banner)
      }
    })

  }

*/
    pub fn evaluate_templated_function_from_call_for_prototype_ext(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        function_templata: FunctionTemplataT<'s, 't>,
        explicit_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        arg_types: &[CoordT<'s, 't>],
    ) -> IEvaluateFunctionResult<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
        // Profiler.frame(() => {
        //   val FunctionTemplataT(env, function) = functionTemplata
        //   if (function.isLight()) {
        //     closureOrLightLayer.evaluateTemplatedLightFunctionFromCallForPrototype2(
        //       env, coutputs, callingEnv, callRange, callLocation, function,
        //       explicitTemplateArgs, contextRegion, argTypes)
        //   } else {
        //     val lambdaCitizenName2 =
        //       function.name match {
        //         case LambdaDeclarationNameS(codeLocation) => interner.intern(LambdaCitizenNameT(interner.intern(LambdaCitizenTemplateNameT(nameTranslator.translateCodeLocation(codeLocation)))))
        //         case _ => vwat()
        //       }
        //     val KindTemplataT(closureStructRef @ StructTT(_)) =
        //       vassertOne(env.lookupNearestWithName(lambdaCitizenName2, Set(TemplataLookupContext)))
        //     closureOrLightLayer.evaluateTemplatedClosureFunctionFromCallForPrototype(
        //       env, coutputs, callingEnv, callRange, callLocation, closureStructRef, function,
        //       explicitTemplateArgs, contextRegion, argTypes)
        //   }
        // })
    }
/*
  def evaluateTemplatedFunctionFromCallForPrototype(
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    functionTemplata: FunctionTemplataT,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    argTypes: Vector[CoordT]):
  IEvaluateFunctionResult = {
    Profiler.frame(() => {
      val FunctionTemplataT(env, function) = functionTemplata
      if (function.isLight()) {
        closureOrLightLayer.evaluateTemplatedLightFunctionFromCallForPrototype2(
          env, coutputs, callingEnv, callRange, callLocation, function, explicitTemplateArgs, contextRegion, argTypes)
      } else {
        val lambdaCitizenName2 =
          function.name match {
            case LambdaDeclarationNameS(codeLocation) => interner.intern(LambdaCitizenNameT(interner.intern(LambdaCitizenTemplateNameT(nameTranslator.translateCodeLocation(codeLocation)))))
            case _ => vwat()
          }
        val KindTemplataT(closureStructRef @ StructTT(_)) =
          vassertOne(
            env.lookupNearestWithName(
              lambdaCitizenName2,
              Set(TemplataLookupContext)))
        closureOrLightLayer.evaluateTemplatedClosureFunctionFromCallForPrototype(
          env, coutputs, callingEnv, callRange, callLocation, closureStructRef, function, explicitTemplateArgs,
          contextRegion, argTypes)
      }
    })

  }

*/
    pub fn evaluate_generic_virtual_dispatcher_function_for_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        function_templata: FunctionTemplataT<'s, 't>,
        args: &[Option<CoordT<'s, 't>>],
    ) -> Result<IDefineFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        let FunctionTemplataT { outer_env, function } = function_templata;
        self.evaluate_generic_virtual_dispatcher_function_for_prototype_closure_or_light(
            outer_env, coutputs, calling_env, call_range, call_location, function, args)
    }
/*
  def evaluateGenericVirtualDispatcherFunctionForPrototype(
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    functionTemplata: FunctionTemplataT,
    args: Vector[Option[CoordT]]):
  IDefineFunctionResult = {
    Profiler.frame(() => {
      val FunctionTemplataT(env, function) = functionTemplata
      closureOrLightLayer.evaluateGenericVirtualDispatcherFunctionForPrototype(
        env, coutputs, callingEnv, callRange, callLocation, function, args)
    })
  }

*/
    pub fn evaluate_generic_light_function_from_call_for_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        function_templata: FunctionTemplataT<'s, 't>,
        explicit_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
        container_rune_initial_knowns: &[InitialKnown<'s, 't>],
    ) -> Result<IResolveFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        let FunctionTemplataT { outer_env: env, function } = function_templata;
        self.evaluate_generic_light_function_from_call_for_prototype2(
            env, coutputs, calling_env, call_range, call_location, function, explicit_template_args,
            context_region, &args.iter().map(|a| Some(*a)).collect::<Vec<_>>(), container_rune_initial_knowns)
    }
/*
  def evaluateGenericLightFunctionFromCallForPrototype(
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    functionTemplata: FunctionTemplataT,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    args: Vector[CoordT],
    containerRuneInitialKnowns: Vector[InitialKnown] = Vector.empty):
  IResolveFunctionResult = {
    Profiler.frame(() => {
      val FunctionTemplataT(env, function) = functionTemplata
      closureOrLightLayer.evaluateGenericLightFunctionFromCallForPrototype2(
        env, coutputs, callingEnv, callRange, callLocation, function, explicitTemplateArgs,
        contextRegion, args.map(Some(_)), containerRuneInitialKnowns)
    })
  }

*/
    pub fn evaluate_closure_struct(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        containing_node_env: &'t NodeEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        name: IFunctionDeclarationNameS<'s>,
        function_a: &'s FunctionA<'s>,
        verify_conclusions: bool,
    ) -> Result<StructTT<'s, 't>, ICompileErrorT<'s, 't>> {
        let code_body = match &function_a.body {
            IBodyS::CodeBody(code_body) => code_body,
            _ => panic!("evaluate_closure_struct: expected CodeBodyS"),
        };
        let closured_names = code_body.body.closured_names;

        // Note, this is where the unordered closuredNames set becomes ordered.
        let closured_var_names_and_types: Vec<&'t NormalStructMemberT<'s, 't>> =
            closured_names.iter().map(|name| {
                self.determine_closure_variable_member(containing_node_env, coutputs, *name)
            }).collect();

        let (struct_tt, _, _function_templata) =
            self.make_closure_understruct(
                containing_node_env, coutputs, call_range, call_location, name, function_a,
                &closured_var_names_and_types)?;

        Ok(struct_tt)
    }
/*
  def evaluateClosureStruct(
    coutputs: CompilerOutputs,
    containingNodeEnv: NodeEnvironmentT,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    name: IFunctionDeclarationNameS,
    functionA: FunctionA,
    verifyConclusions: Boolean):
  (StructTT) = {
    val CodeBodyS(body) = functionA.body
    val closuredNames = body.closuredNames;

    // Note, this is where the unordered closuredNames set becomes ordered.
    val closuredVarNamesAndTypes =
      closuredNames
        .map(name => determineClosureVariableMember(containingNodeEnv, coutputs, name))
        .toVector;

    val (structTT, _, functionTemplata) =
      structCompiler.makeClosureUnderstruct(
        containingNodeEnv, coutputs, callRange, callLocation, name, functionA, closuredVarNamesAndTypes)

    (structTT)
  }

*/
    pub fn determine_closure_variable_member(
        &self,
        env: &'t NodeEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        name: IVarNameS<'s>,
    ) -> &'t NormalStructMemberT<'s, 't> {
        let translated_name = self.translate_var_name_step(name);
        let (variability, tyype) = match env.get_variable(translated_name).unwrap() {
            IVariableT::ReferenceLocal(ReferenceLocalVariableT { variability, coord, .. }) => {
                // See "Captured own is borrow" test for why we do this
                let tyype = match coord.ownership {
                    OwnershipT::Own => IMemberTypeT::Reference(ReferenceMemberTypeT { reference: CoordT { ownership: OwnershipT::Borrow, region: coord.region, kind: coord.kind } }),
                    OwnershipT::Borrow | OwnershipT::Share => IMemberTypeT::Reference(ReferenceMemberTypeT { reference: coord }),
                    OwnershipT::Weak => {
                        unreachable!("Scala determineClosureVariableMember ReferenceLocalVariableT has no Weak arm — only OwnT and BorrowT|ShareT");
                    }
                };
                (variability, tyype)
            }
            IVariableT::AddressibleLocal(AddressibleLocalVariableT { variability, coord: reference, .. }) => {
                (variability, IMemberTypeT::Address(AddressMemberTypeT { reference }))
            }
            IVariableT::ReferenceClosure(ReferenceClosureVariableT { variability, coord, .. }) => {
                // See "Captured own is borrow" test for why we do this
                let tyype = match coord.ownership {
                    OwnershipT::Own => IMemberTypeT::Reference(ReferenceMemberTypeT { reference: CoordT { ownership: OwnershipT::Borrow, region: coord.region, kind: coord.kind } }),
                    OwnershipT::Borrow | OwnershipT::Share => IMemberTypeT::Reference(ReferenceMemberTypeT { reference: coord }),
                    OwnershipT::Weak => {
                        unreachable!("Scala determineClosureVariableMember ReferenceClosureVariableT has no Weak arm — only OwnT and BorrowT|ShareT");
                    }
                };
                (variability, tyype)
            }
            IVariableT::AddressibleClosure(AddressibleClosureVariableT { variability, coord: reference, .. }) => {
                (variability, IMemberTypeT::Address(AddressMemberTypeT { reference }))
            }
        };
        self.typing_interner.alloc(NormalStructMemberT { name: translated_name, variability, tyype })
    }
/*
  private def determineClosureVariableMember(
    env: NodeEnvironmentT,
    coutputs: CompilerOutputs,
    name: IVarNameS) = {
    val (variability2, memberType) =
      env.getVariable(nameTranslator.translateVarNameStep(name)).get match {
        case ReferenceLocalVariableT(_, variability, coord@CoordT(ownership, region, kind)) => {
          // See "Captured own is borrow" test for why we do this
          val tyype =
            ownership match {
              case OwnT => ReferenceMemberTypeT(CoordT(BorrowT, region, kind))
              case BorrowT | ShareT => ReferenceMemberTypeT(coord)
            }
          (variability, tyype)
        }
        case AddressibleLocalVariableT(_, variability, reference) => {
          (variability, AddressMemberTypeT(reference))
        }
        case ReferenceClosureVariableT(_, _, variability, coord@CoordT(ownership, region, kind)) => {
          // See "Captured own is borrow" test for why we do this
          val tyype =
            ownership match {
              case OwnT => ReferenceMemberTypeT(CoordT(BorrowT, region, kind))
              case BorrowT | ShareT => ReferenceMemberTypeT(coord)
            }
          (variability, tyype)
        }
        case AddressibleClosureVariableT(_, _, variability, reference) => {
          (variability, AddressMemberTypeT(reference))
        }
      }
    NormalStructMemberT(nameTranslator.translateVarNameStep(name), variability2, memberType)
  }

}
*/
}
