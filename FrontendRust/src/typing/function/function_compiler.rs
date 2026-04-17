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
import dev.vale.typing.{CompilerOutputs, ConvertHelper, IFunctionGenerator, InferCompiler, TemplataCompiler, TypingPassOptions}
import dev.vale.typing.ast.{FunctionBannerT, FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT, PrototypeT, ReferenceExpressionTE}
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.env._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.names.LambdaCitizenNameT

import scala.collection.immutable.{List, Set}



*/
use crate::typing::compiler::Compiler;

// mig: trait IFunctionCompilerDelegate
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
// mig: trait IEvaluateFunctionResult
pub enum IEvaluateFunctionResult<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
trait IEvaluateFunctionResult

*/
// mig: struct EvaluateFunctionSuccess
pub struct EvaluateFunctionSuccess<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class EvaluateFunctionSuccess(
    prototype: PrototypeTemplataT[IFunctionNameT],
    inferences: Map[IRuneS, ITemplataT[ITemplataType]],
    instantiationBoundArgs: InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]
) extends IEvaluateFunctionResult

*/
// mig: struct EvaluateFunctionFailure
pub struct EvaluateFunctionFailure<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class EvaluateFunctionFailure(
    reason: IDefiningError
) extends IEvaluateFunctionResult

*/
// mig: trait IDefineFunctionResult
pub enum IDefineFunctionResult<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
trait IDefineFunctionResult

*/
// mig: struct DefineFunctionSuccess
pub struct DefineFunctionSuccess<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class DefineFunctionSuccess(
    prototype: PrototypeTemplataT[IFunctionNameT],
    inferences: Map[IRuneS, ITemplataT[ITemplataType]],
    instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]
) extends IDefineFunctionResult

*/
// mig: struct DefineFunctionFailure
pub struct DefineFunctionFailure<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class DefineFunctionFailure(
    reason: IDefiningError
) extends IDefineFunctionResult


*/
// mig: trait IResolveFunctionResult
pub enum IResolveFunctionResult<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
trait IResolveFunctionResult

*/
// mig: struct ResolveFunctionSuccess
pub struct ResolveFunctionSuccess<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ResolveFunctionSuccess(
    prototype: PrototypeTemplataT[IFunctionNameT],
    inferences: Map[IRuneS, ITemplataT[ITemplataType]]
) extends IResolveFunctionResult

*/
// mig: struct ResolveFunctionFailure
pub struct ResolveFunctionFailure<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ResolveFunctionFailure(
    reason: IResolvingError
) extends IResolveFunctionResult


*/
// mig: trait IStampFunctionResult
pub enum IStampFunctionResult<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
trait IStampFunctionResult

*/
// mig: struct StampFunctionSuccess
pub struct StampFunctionSuccess<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class StampFunctionSuccess(
  prototype: PrototypeT[IFunctionNameT],
  inferences: Map[IRuneS, ITemplataT[ITemplataType]]
) extends IStampFunctionResult

*/
// mig: struct StampFunctionFailure
pub struct StampFunctionFailure<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class StampFunctionFailure(
  reason: IFindFunctionFailureReason
) extends IStampFunctionResult


*/
// mig: struct FunctionCompiler
// mig: impl FunctionCompiler
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
// mig: fn evaluate_generic_function_from_non_call
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_function_from_non_call(&self) { panic!("Unimplemented: evaluate_generic_function_from_non_call"); }
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
}

// mig: fn evaluate_templated_light_function_from_call_for_prototype
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_light_function_from_call_for_prototype(&self) { panic!("Unimplemented: evaluate_templated_light_function_from_call_for_prototype"); }
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
}

// mig: fn evaluate_templated_function_from_call_for_prototype
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_function_from_call_for_prototype(&self) { panic!("Unimplemented: evaluate_templated_function_from_call_for_prototype"); }
/*
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
}

// mig: fn evaluate_templated_function_from_call_for_prototype
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_function_from_call_for_prototype_ext(&self) { panic!("Unimplemented: evaluate_templated_function_from_call_for_prototype"); }
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
}

// mig: fn evaluate_generic_virtual_dispatcher_function_for_prototype
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_virtual_dispatcher_function_for_prototype(&self) { panic!("Unimplemented: evaluate_generic_virtual_dispatcher_function_for_prototype"); }
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
}

// mig: fn evaluate_generic_light_function_from_call_for_prototype
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_light_function_from_call_for_prototype(&self) { panic!("Unimplemented: evaluate_generic_light_function_from_call_for_prototype"); }
/*
  def evaluateGenericLightFunctionFromCallForPrototype(
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    functionTemplata: FunctionTemplataT,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    args: Vector[CoordT]):
  IResolveFunctionResult = {
    Profiler.frame(() => {
      val FunctionTemplataT(env, function) = functionTemplata
      closureOrLightLayer.evaluateGenericLightFunctionFromCallForPrototype2(
        env, coutputs, callingEnv, callRange, callLocation, function, explicitTemplateArgs,
        contextRegion, args.map(Some(_)))
    })
  }

*/
}

// mig: fn evaluate_closure_struct
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_closure_struct(&self) { panic!("Unimplemented: evaluate_closure_struct"); }
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
}

// mig: fn determine_closure_variable_member
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn determine_closure_variable_member(&self) { panic!("Unimplemented: determine_closure_variable_member"); }
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
