use std::collections::{HashMap, HashSet};

use crate::utils::range::RangeS;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::rules::rules::*;
use crate::postparsing::names::*;
use crate::postparsing::*;
use crate::solver::solver::*;
use crate::solver::simple_solver_state::*;
use crate::typing::compiler::Compiler;
use crate::typing::citizen::impl_compiler::IsParentResult;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler_outputs::*;
use crate::typing::templata::templata::*;
use crate::typing::types::types::*;
use crate::typing::names::names::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::higher_typing::ast::*;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::typing::infer_compiler::InferEnv;
use crate::typing::overload_resolver::FindFunctionFailure;
use crate::typing::citizen::impl_compiler::IsntParent;
use crate::typing::citizen::struct_compiler::ResolveFailure;
use crate::typing::templata::conversions::evaluate_ownership;
use crate::parsing::ast::ast::OwnershipP;
use crate::typing::templata::templata::KindTemplataT;
use crate::typing::types::types::OwnershipT;
use crate::typing::templata::templata::CoordTemplataT;
use crate::typing::templata::conversions::evaluate_mutability;
use crate::typing::templata::conversions::evaluate_variability;
use crate::typing::typing_interner::TypingInterner;
use crate::typing::templata::templata::MutabilityTemplataT;
use crate::typing::templata::templata::OwnershipTemplataT;
use crate::typing::templata::templata::VariabilityTemplataT;

/*
package dev.vale.typing.infer

import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.ShareP
import dev.vale.postparsing.rules._
import dev.vale.{Err, Ok, RangeS, Result, vassert, vassertSome, vimpl, vwat}
import dev.vale.postparsing._
import dev.vale.solver.{FailedSolve, ISolverError, RuleError, SimpleSolverState, Solver, SolverConflict}
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.ast.PrototypeT
import dev.vale.typing.function.StampFunctionSuccess
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale._
import dev.vale.postparsing.ArgumentRuneS
import dev.vale.postparsing.rules._
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.citizen.{IsntParent, ResolveFailure}
import dev.vale.typing.env.TemplataLookupContext
import dev.vale.typing._
import dev.vale.typing.templata.ITemplataT.{expectCoordTemplata, expectKindTemplata}
import dev.vale.typing.types._

import scala.collection.immutable.{HashSet, Map}
import scala.collection.mutable

*/
#[derive(Copy, Clone, Debug)]
pub enum ITypingPassSolverError<'s, 't> {
    KindIsNotConcrete { kind: KindT<'s, 't> },
    KindIsNotInterface { kind: KindT<'s, 't> },
    KindIsNotStruct { kind: KindT<'s, 't> },
    CouldntFindFunction { range: &'t [RangeS<'s>], fff: FindFunctionFailure<'s, 't> },
    CouldntFindImpl { range: &'t [RangeS<'s>], fail: &'t IsntParent<'s, 't> },
    CouldntResolveKind { rf: &'t ResolveFailure<'s, 't, KindT<'s, 't>> },
    CantShareMutable { kind: KindT<'s, 't> },
    CantSharePlaceholder { kind: KindT<'s, 't> },
    BadIsaSubKind { kind: KindT<'s, 't> },
    BadIsaSuperKind { kind: KindT<'s, 't> },
    SendingNonCitizen { kind: KindT<'s, 't> },
    CantCheckPlaceholder { range: &'t [RangeS<'s>] },
    ReceivingDifferentOwnerships { params: &'t [(IRuneS<'s>, CoordT<'s, 't>)] },
    SendingNonIdenticalKinds { send_coord: CoordT<'s, 't>, receive_coord: CoordT<'s, 't> },
    NoCommonAncestors { params: &'t [(IRuneS<'s>, CoordT<'s, 't>)] },
    LookupFailed { name: IImpreciseNameS<'s> },
    NoAncestorsSatisfyCall { params: &'t [(IRuneS<'s>, CoordT<'s, 't>)] },
    CantDetermineNarrowestKind { kinds: &'t [KindT<'s, 't>] },
    OwnershipDidntMatch { coord: CoordT<'s, 't>, expected_ownership: OwnershipT },
    CallResultWasntExpectedType { expected: ITemplataT<'s, 't>, actual: ITemplataT<'s, 't> },
    CallResultIsntCallable { result: ITemplataT<'s, 't> },
    OneOfFailed { rule: OneOfSR<'s> },
    IsaFailed { sub: KindT<'s, 't>, suuper: KindT<'s, 't> },
    WrongNumberOfTemplateArgs { expected_min_num_args: i32, expected_max_num_args: i32 },
    FunctionDoesntHaveName { range: &'t [RangeS<'s>], name: IFunctionNameT<'s, 't> },
    CantGetComponentsOfPlaceholderPrototype { range: &'t [RangeS<'s>] },
    ReturnTypeConflict { range: &'t [RangeS<'s>], expected_return_type: CoordT<'s, 't>, actual: PrototypeT<'s, 't> },
    InternalSolverError { range: &'t [RangeS<'s>], err: &'t ISolverError<IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>> },
}
/*
sealed trait ITypingPassSolverError
*/
/*
case class KindIsNotConcrete(kind: KindT) extends ITypingPassSolverError
*/
/*
case class KindIsNotInterface(kind: KindT) extends ITypingPassSolverError
*/
/*
case class KindIsNotStruct(kind: KindT) extends ITypingPassSolverError
*/
/*
case class CouldntFindFunction(range: List[RangeS], fff: FindFunctionFailure) extends ITypingPassSolverError {
  vpass()
}
*/
/*
case class CouldntFindImpl(range: List[RangeS], fail: IsntParent) extends ITypingPassSolverError
*/
/*
case class CouldntResolveKind(
  rf: ResolveFailure[KindT]
) extends ITypingPassSolverError {
  vpass()
}
*/
/*
case class CantShareMutable(kind: KindT) extends ITypingPassSolverError
*/
/*
case class CantSharePlaceholder(kind: KindT) extends ITypingPassSolverError
*/
/*
case class BadIsaSubKind(kind: KindT) extends ITypingPassSolverError {
  vpass()
}
*/
/*
case class BadIsaSuperKind(kind: KindT) extends ITypingPassSolverError {
  vpass()
}
*/
/*
case class SendingNonCitizen(kind: KindT) extends ITypingPassSolverError {
  vpass()
}
*/
/*
case class CantCheckPlaceholder(range: List[RangeS]) extends ITypingPassSolverError
*/
/*
case class ReceivingDifferentOwnerships(params: Vector[(IRuneS, CoordT)]) extends ITypingPassSolverError
*/
/*
case class SendingNonIdenticalKinds(sendCoord: CoordT, receiveCoord: CoordT) extends ITypingPassSolverError
*/
/*
case class NoCommonAncestors(params: Vector[(IRuneS, CoordT)]) extends ITypingPassSolverError
*/
/*
case class LookupFailed(name: IImpreciseNameS) extends ITypingPassSolverError
*/
/*
case class NoAncestorsSatisfyCall(params: Vector[(IRuneS, CoordT)]) extends ITypingPassSolverError
*/
/*
case class CantDetermineNarrowestKind(kinds: Set[KindT]) extends ITypingPassSolverError
*/
/*
case class OwnershipDidntMatch(coord: CoordT, expectedOwnership: OwnershipT) extends ITypingPassSolverError
*/
/*
case class CallResultWasntExpectedType(expected: ITemplataT[ITemplataType], actual: ITemplataT[ITemplataType]) extends ITypingPassSolverError
*/
/*
case class CallResultIsntCallable(result: ITemplataT[ITemplataType]) extends ITypingPassSolverError
*/
/*
case class OneOfFailed(rule: OneOfSR) extends ITypingPassSolverError
*/
/*
case class IsaFailed(sub: KindT, suuper: KindT) extends ITypingPassSolverError
*/
/*
case class WrongNumberOfTemplateArgs(expectedMinNumArgs: Int, expectedMaxNumArgs: Int) extends ITypingPassSolverError
*/
/*
case class FunctionDoesntHaveName(range: List[RangeS], name: IFunctionNameT) extends ITypingPassSolverError
*/
/*
case class CantGetComponentsOfPlaceholderPrototype(range: List[RangeS]) extends ITypingPassSolverError
*/
/*
case class ReturnTypeConflict(range: List[RangeS], expectedReturnType: CoordT, actual: PrototypeT[IFunctionNameT]) extends ITypingPassSolverError
*/
/*
case class InternalSolverError(range: List[RangeS], err: ISolverError[IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends ITypingPassSolverError

*/
/*
trait IInfererDelegate {
//  def lookupMemberTypes(
//    state: CompilerOutputs,
//    kind: KindT,
//    // This is here so that the predictor can just give us however many things
//    // we expect.
//    expectedNumMembers: Int
//  ): Option[Vector[CoordT]]

  def getMutability(state: CompilerOutputs, kind: KindT): ITemplataT[MutabilityTemplataType]

  def lookupTemplata(env: InferEnv, state: CompilerOutputs, range: List[RangeS], name: INameT): ITemplataT[ITemplataType]

  def lookupTemplataImprecise(env: InferEnv, state: CompilerOutputs, range: List[RangeS], name: IImpreciseNameS): Option[ITemplataT[ITemplataType]]

  def coerceToCoord(
    env: InferEnv,
    state: CompilerOutputs,
    range: List[RangeS],
    templata: ITemplataT[ITemplataType],
    region: RegionT):
  ITemplataT[ITemplataType]

  def isDescendant(env: InferEnv, state: CompilerOutputs, kind: KindT): Boolean
  def isAncestor(env: InferEnv, state: CompilerOutputs, kind: KindT): Boolean

  def sanityCheckConclusion(env: InferEnv, state: CompilerOutputs, rune: IRuneS, templata: ITemplataT[ITemplataType]): Unit

  // See SFWPRL for how this is different from resolveStruct.
  def predictStruct(
    env: InferEnv,
    state: CompilerOutputs,
    templata: StructDefinitionTemplataT,
    templateArgs: Vector[ITemplataT[ITemplataType]]):
  (KindT)

  // See SFWPRL for how this is different from resolveInterface.
  def predictInterface(
    env: InferEnv,
    state: CompilerOutputs,
    templata: InterfaceDefinitionTemplataT,
    templateArgs: Vector[ITemplataT[ITemplataType]]):
  (KindT)

  def predictStaticSizedArrayKind(
    env: InferEnv,
    state: CompilerOutputs,
    mutability: ITemplataT[MutabilityTemplataType],
    variability: ITemplataT[VariabilityTemplataType],
    size: ITemplataT[IntegerTemplataType],
    element: CoordT,
    region: RegionT):
  StaticSizedArrayTT

  def predictRuntimeSizedArrayKind(
    env: InferEnv,
    state: CompilerOutputs,
    type2: CoordT,
    arrayMutability: ITemplataT[MutabilityTemplataType],
    region: RegionT):
  RuntimeSizedArrayTT

  def getAncestors(env: InferEnv, coutputs: CompilerOutputs, descendant: KindT, includeSelf: Boolean):
  (Set[KindT])

  def isParent(
    env: InferEnv,
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    subKindTT: ISubKindTT,
    superKindTT: ISuperKindTT,
    includeSelf: Boolean):
  Option[ITemplataT[ImplTemplataType]]

  def structIsClosure(state: CompilerOutputs, structTT: StructTT): Boolean

  def kindIsFromTemplate(
    state: CompilerOutputs,
    actualCitizenRef: KindT,
    expectedCitizenTemplata: ITemplataT[ITemplataType]):
  Boolean

  def predictFunction(
    env: InferEnv,
    state: CompilerOutputs,
    functionRange: RangeS,
    name: StrI,
    paramCoords: Vector[CoordT],
    returnCoord: CoordT):
  PrototypeTemplataT[IFunctionNameT]

  // Per @BRRZ, used by the relaxed ResolveSR handler when the return rune isn't known.
  // Performs a real overload lookup against the caller's (snapshotted) env and returns
  // the stamped function's prototype, whose returnType unblocks the solver. Mirrors
  // the outer InferCompiler delegate's resolveFunction — this delegate exists only
  // inside the solver's own handler dispatch, so we need it declared here too.
  def resolveFunction(
    env: InferEnv,
    state: CompilerOutputs,
    range: List[RangeS],
    name: StrI,
    paramCoords: Vector[CoordT]):
  Result[StampFunctionSuccess, FindFunctionFailure]

  def assemblePrototype(
    env: InferEnv,
    state: CompilerOutputs,
    range: RangeS,
    name: StrI,
    coords: Vector[CoordT],
    returnType: CoordT):
  PrototypeT[IFunctionNameT]

  def assembleImpl(
    env: InferEnv,
    range: RangeS,
    subKind: KindT,
    superKind: KindT):
  IsaTemplataT
}

*/
/*
class CompilerSolver(
  globalOptions: GlobalOptions,
  interner: Interner,
  delegate: IInfererDelegate
) {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_runes(&self, rule: IRulexSR<'s>) -> Vec<IRuneS<'s>> {
        let result: Vec<IRuneS<'s>> = rule.rune_usages().iter().map(|ru| ru.rune).collect();
        if self.opts.global_options.sanity_check {
            // val sanityChecked: Vector[RuneUsage] =
            //   rule match {
            let sanity_checked: Vec<RuneUsage<'s>> =
                match rule {
                    //     case LookupSR(range, rune, literal) => Vector(rune)
                    IRulexSR::Lookup(r) => vec![r.rune],
                    //     case RuneParentEnvLookupSR(range, rune) => Vector(rune)
                    IRulexSR::RuneParentEnvLookup(r) => vec![r.rune],
                    //     case EqualsSR(range, left, right) => Vector(left, right)
                    IRulexSR::Equals(r) => vec![r.left, r.right],
                    //     case DefinitionCoordIsaSR(range, result, sub, suuper) => Vector(result, sub, suuper)
                    IRulexSR::DefinitionCoordIsa(r) => vec![r.result_rune, r.sub_rune, r.super_rune],
                    //     case CallSiteCoordIsaSR(range, result, sub, suuper) => result.toVector ++ Vector(sub, suuper)
                    IRulexSR::CallSiteCoordIsa(r) => {
                        let mut v: Vec<RuneUsage<'s>> = r.result_rune.into_iter().collect();
                        v.push(r.sub_rune);
                        v.push(r.super_rune);
                        v
                    }
                    //     case KindComponentsSR(range, resultRune, mutabilityRune) => Vector(resultRune, mutabilityRune)
                    IRulexSR::KindComponents(r) => vec![r.kind_rune, r.mutability_rune],
                    //     case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(resultRune, ownershipRune, kindRune)
                    IRulexSR::CoordComponents(r) => vec![r.result_rune, r.ownership_rune, r.kind_rune],
                    //     case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => Vector(resultRune, paramsRune, returnRune)
                    IRulexSR::PrototypeComponents(r) => vec![r.result_rune, r.params_rune, r.return_rune],
                    //     case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
                    IRulexSR::DefinitionFunc(r) => vec![r.result_rune, r.params_list_rune, r.return_rune],
                    //     case CallSiteFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
                    IRulexSR::CallSiteFunc(r) => vec![r.prototype_rune, r.params_list_rune, r.return_rune],
                    //     case ResolveSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
                    IRulexSR::Resolve(r) => vec![r.result_rune, r.params_list_rune, r.return_rune],
                    //     case OneOfSR(range, rune, literals) => Vector(rune)
                    IRulexSR::OneOf(r) => vec![r.rune],
                    //     case IsConcreteSR(range, rune) => Vector(rune)
                    IRulexSR::IsConcrete(r) => vec![r.rune],
                    //     case IsInterfaceSR(range, rune) => Vector(rune)
                    IRulexSR::IsInterface(r) => vec![r.rune],
                    //     case IsStructSR(range, rune) => Vector(rune)
                    IRulexSR::IsStruct(r) => vec![r.rune],
                    //     case CoerceToCoordSR(range, coordRune, kindRune) => Vector(coordRune, kindRune)
                    IRulexSR::CoerceToCoord(r) => vec![r.coord_rune, r.kind_rune],
                    //     case LiteralSR(range, rune, literal) => Vector(rune)
                    IRulexSR::Literal(r) => vec![r.rune],
                    //     case AugmentSR(range, resultRune, ownership, innerRune) => Vector(resultRune, innerRune)
                    IRulexSR::Augment(r) => vec![r.result_rune, r.inner_rune],
                    //     case CallSR(range, resultRune, templateRune, args) => Vector(resultRune, templateRune) ++ args
                    IRulexSR::Call(r) => {
                        let mut v = vec![r.result_rune, r.template_rune];
                        v.extend_from_slice(r.args);
                        v
                    }
                    //     case PackSR(range, resultRune, members) => Vector(resultRune) ++ members
                    IRulexSR::Pack(r) => {
                        let mut v = vec![r.result_rune];
                        v.extend_from_slice(r.members);
                        v
                    }
                    //     case CoordSendSR(range, senderRune, receiverRune) => Vector(senderRune, receiverRune)
                    IRulexSR::CoordSend(r) => vec![r.sender_rune, r.receiver_rune],
                    //     case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => Vector(resultRune, coordListRune)
                    IRulexSR::RefListCompoundMutability(r) => vec![r.result_rune, r.coord_list_rune],
                    //     case other => vimpl(other)
                    other => panic!("get_runes sanity check: unhandled rule {:?}", other),
                };
            //   vassert(result sameElements sanityChecked.map(_.rune))
            let sanity_runes: Vec<IRuneS<'s>> = sanity_checked.iter().map(|ru| ru.rune).collect();
            assert!(result.iter().zip(sanity_runes.iter()).all(|(a, b)| a == b) && result.len() == sanity_runes.len());
        }
        result
    }
/*
  def getRunes(rule: IRulexSR): Vector[IRuneS] = {
    val result = rule.runeUsages.map(_.rune)

    if (globalOptions.sanityCheck) {
      val sanityChecked: Vector[RuneUsage] =
        rule match {
          case LookupSR(range, rune, literal) => Vector(rune)
          case RuneParentEnvLookupSR(range, rune) => Vector(rune)
          case EqualsSR(range, left, right) => Vector(left, right)
          case DefinitionCoordIsaSR(range, result, sub, suuper) => Vector(result, sub, suuper)
          case CallSiteCoordIsaSR(range, result, sub, suuper) => result.toVector ++ Vector(sub, suuper)
          case KindComponentsSR(range, resultRune, mutabilityRune) => Vector(resultRune, mutabilityRune)
          case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(resultRune, ownershipRune, kindRune)
          case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => Vector(resultRune, paramsRune, returnRune)
          case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
          case CallSiteFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
          case ResolveSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
          case OneOfSR(range, rune, literals) => Vector(rune)
          case IsConcreteSR(range, rune) => Vector(rune)
          case IsInterfaceSR(range, rune) => Vector(rune)
          case IsStructSR(range, rune) => Vector(rune)
          case CoerceToCoordSR(range, coordRune, kindRune) => Vector(coordRune, kindRune)
          case LiteralSR(range, rune, literal) => Vector(rune)
          case AugmentSR(range, resultRune, ownership, innerRune) => Vector(resultRune, innerRune)
          case CallSR(range, resultRune, templateRune, args) => Vector(resultRune, templateRune) ++ args
//          case PrototypeSR(range, resultRune, name, parameters, returnTypeRune) => Vector(resultRune) ++ parameters ++ Vector(returnTypeRune)
          case PackSR(range, resultRune, members) => Vector(resultRune) ++ members
//          case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => Vector(resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune)
//          case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => Vector(resultRune, mutabilityRune, elementRune)
          //        case ManualSequenceSR(range, resultRune, elements) => Vector(resultRune) ++ elements
          //        case CoordListSR(range, resultRune, elements) => Vector(resultRune) ++ elements
          case CoordSendSR(range, senderRune, receiverRune) => Vector(senderRune, receiverRune)
          case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => Vector(resultRune, coordListRune)
          case other => vimpl(other)
        }
      vassert(result sameElements sanityChecked.map(_.rune))
    }
    result
  }

*/
}

pub fn get_puzzles<'s>(rule: IRulexSR<'s>) -> Vec<Vec<IRuneS<'s>>> {
    //   rule match {
    match rule {
            //     // This means we can solve this puzzle and dont need anything to do it.
            //     case LookupSR(range, _, _) => Vector(Vector())
            IRulexSR::Lookup(_) => vec![vec![]],
            //     case RuneParentEnvLookupSR(range, rune) => Vector(Vector())
            IRulexSR::RuneParentEnvLookup(_) => vec![vec![]],
            //     case CallSR(range, resultRune, templateRune, args) => {
            //       Vector(
            //         Vector(templateRune.rune) ++ args.map(_.rune),
            //         Vector(resultRune.rune, templateRune.rune))
            //     }
            IRulexSR::Call(r) => {
                let mut first = vec![r.template_rune.rune];
                first.extend(r.args.iter().map(|a| a.rune));
                vec![first, vec![r.result_rune.rune, r.template_rune.rune]]
            }
            //     case PackSR(range, resultRune, members) => Vector(Vector(resultRune.rune), members.map(_.rune))
            IRulexSR::Pack(r) => {
                vec![vec![r.result_rune.rune], r.members.iter().map(|m| m.rune).collect()]
            }
            //     case KindComponentsSR(range, kindRune, mutabilityRune) => Vector(Vector(kindRune.rune))
            IRulexSR::KindComponents(r) => vec![vec![r.kind_rune.rune]],
            //     case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(Vector(resultRune.rune), Vector(ownershipRune.rune, kindRune.rune))
            IRulexSR::CoordComponents(r) => vec![vec![r.result_rune.rune], vec![r.ownership_rune.rune, r.kind_rune.rune]],
            //     case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => Vector(Vector(resultRune.rune))
            IRulexSR::PrototypeComponents(r) => vec![vec![r.result_rune.rune]],
            //     case CallSiteFuncSR(range, resultRune, name, paramListRune, returnRune) => Vector(Vector(resultRune.rune))
            IRulexSR::CallSiteFunc(r) => vec![vec![r.prototype_rune.rune]],
            //     // Definition doesn't need the placeholder to be present, it's what populates the placeholder.
            //     case DefinitionFuncSR(range, placeholderRune, name, paramListRune, returnRune) => Vector(Vector(paramListRune.rune, returnRune.rune))
            IRulexSR::DefinitionFunc(r) => vec![vec![r.params_list_rune.rune, r.return_rune.rune]],
            //     // Per @BRRZ, ResolveSR fires in one of two modes: when both params and return
            //     // are known (existing predict path, postponing real resolution per SFWPRL), or
            //     // when only params are known (real overload lookup to discover the return).
            //     // Handler below branches on which condition triggered.
            //     case ResolveSR(range, resultRune, name, paramsListRune, returnRune) =>
            //       Vector(
            //         Vector(paramsListRune.rune, returnRune.rune),
            //         Vector(paramsListRune.rune))
            IRulexSR::Resolve(r) => vec![
                vec![r.params_list_rune.rune, r.return_rune.rune],
                vec![r.params_list_rune.rune],
            ],
            //     case OneOfSR(range, rune, literals) => Vector(Vector(rune.rune))
            IRulexSR::OneOf(r) => vec![vec![r.rune.rune]],
            //     case EqualsSR(range, leftRune, rightRune) => Vector(Vector(leftRune.rune), Vector(rightRune.rune))
            IRulexSR::Equals(r) => vec![vec![r.left.rune], vec![r.right.rune]],
            //     case IsConcreteSR(range, rune) => Vector(Vector(rune.rune))
            IRulexSR::IsConcrete(r) => vec![vec![r.rune.rune]],
            //     case IsInterfaceSR(range, rune) => Vector(Vector(rune.rune))
            IRulexSR::IsInterface(r) => vec![vec![r.rune.rune]],
            //     case IsStructSR(range, rune) => Vector(Vector(rune.rune))
            IRulexSR::IsStruct(r) => vec![vec![r.rune.rune]],
            //     case CoerceToCoordSR(range, coordRune, kindRune) => Vector(Vector(coordRune.rune), Vector(kindRune.rune))
            IRulexSR::CoerceToCoord(r) => vec![vec![r.coord_rune.rune], vec![r.kind_rune.rune]],
            //     case LiteralSR(range, rune, literal) => Vector(Vector())
            IRulexSR::Literal(_) => vec![vec![]],
            //     case AugmentSR(range, resultRune, ownership, innerRune) => Vector(Vector(innerRune.rune), Vector(resultRune.rune))
            IRulexSR::Augment(r) => vec![vec![r.inner_rune.rune], vec![r.result_rune.rune]],
            //     // See SAIRFU, this will replace itself with other rules.
            //     case CoordSendSR(range, senderRune, receiverRune) => Vector(Vector(senderRune.rune), Vector(receiverRune.rune))
            IRulexSR::CoordSend(r) => vec![vec![r.sender_rune.rune], vec![r.receiver_rune.rune]],
            //     case DefinitionCoordIsaSR(range, resultRune, senderRune, receiverRune) => Vector(Vector(senderRune.rune, receiverRune.rune))
            IRulexSR::DefinitionCoordIsa(r) => vec![vec![r.sub_rune.rune, r.super_rune.rune]],
            //     case CallSiteCoordIsaSR(range, resultRune, senderRune, receiverRune) => Vector(Vector(senderRune.rune, receiverRune.rune))
            IRulexSR::CallSiteCoordIsa(r) => vec![vec![r.sub_rune.rune, r.super_rune.rune]],
            //     case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => Vector(Vector(coordListRune.rune))
            IRulexSR::RefListCompoundMutability(r) => vec![vec![r.coord_list_rune.rune]],
            other => panic!("get_puzzles: unhandled rule {:?}", other),
        }
    }
/*
  def getPuzzles(rule: IRulexSR): Vector[Vector[IRuneS]] = {
    rule match {
      // This means we can solve this puzzle and dont need anything to do it.
      case LookupSR(range, _, _) => Vector(Vector())
      case RuneParentEnvLookupSR(range, rune) => Vector(Vector())
      case CallSR(range, resultRune, templateRune, args) => {
        Vector(
          Vector(templateRune.rune) ++ args.map(_.rune),
          // Do we really need to do
          //   Vector(resultRune.rune, templateRune.rune),
          // Because if we have X = T<A> and we know that X is a Moo<int>
          // then we can know T = Moo and A = int.
          // So maybe one day we can not require templateRune here.
          Vector(resultRune.rune, templateRune.rune))
      }
      case PackSR(range, resultRune, members) => Vector(Vector(resultRune.rune), members.map(_.rune))
      case KindComponentsSR(range, kindRune, mutabilityRune) => Vector(Vector(kindRune.rune))
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(Vector(resultRune.rune), Vector(ownershipRune.rune, kindRune.rune))
      case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => Vector(Vector(resultRune.rune))
      case CallSiteFuncSR(range, resultRune, name, paramListRune, returnRune) => Vector(Vector(resultRune.rune))
      // Definition doesn't need the placeholder to be present, it's what populates the placeholder.
      case DefinitionFuncSR(range, placeholderRune, name, paramListRune, returnRune) => Vector(Vector(paramListRune.rune, returnRune.rune))
      // Per @BRRZ, ResolveSR fires in one of two modes: when both params and return
      // are known (existing predict path, postponing real resolution per SFWPRL), or
      // when only params are known (real overload lookup to discover the return).
      // Handler below branches on which condition triggered.
      case ResolveSR(range, resultRune, name, paramsListRune, returnRune) =>
        Vector(
          Vector(paramsListRune.rune, returnRune.rune),
          Vector(paramsListRune.rune))
      case OneOfSR(range, rune, literals) => Vector(Vector(rune.rune))
      case EqualsSR(range, leftRune, rightRune) => Vector(Vector(leftRune.rune), Vector(rightRune.rune))
      case IsConcreteSR(range, rune) => Vector(Vector(rune.rune))
      case IsInterfaceSR(range, rune) => Vector(Vector(rune.rune))
      case IsStructSR(range, rune) => Vector(Vector(rune.rune))
      case CoerceToCoordSR(range, coordRune, kindRune) => Vector(Vector(coordRune.rune), Vector(kindRune.rune))
      case LiteralSR(range, rune, literal) => Vector(Vector())
      case AugmentSR(range, resultRune, ownership, innerRune) => Vector(Vector(innerRune.rune), Vector(resultRune.rune))
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => Vector(Vector(resultRune.rune), Vector(mutabilityRune.rune, variabilityRune.rune, sizeRune.rune, elementRune.rune))
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => Vector(Vector(resultRune.rune), Vector(mutabilityRune.rune, elementRune.rune))
      // See SAIRFU, this will replace itself with other rules.
      case CoordSendSR(range, senderRune, receiverRune) => Vector(Vector(senderRune.rune), Vector(receiverRune.rune))
      case DefinitionCoordIsaSR(range, resultRune, senderRune, receiverRune) => Vector(Vector(senderRune.rune, receiverRune.rune))
      case CallSiteCoordIsaSR(range, resultRune, senderRune, receiverRune) => Vector(Vector(senderRune.rune, receiverRune.rune))
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => Vector(Vector(coordListRune.rune))
    }
  }

*/

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_solver_state_solver(
        &self,
        _range: Vec<RangeS<'s>>,
        env: InferEnv<'s, 't>,
        state: &mut CompilerOutputs<'s, 't>,
        rules: Vec<IRulexSR<'s>>,
        initial_rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>>,
        initially_known_rune_to_templata: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
    ) -> SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>> {
        for rule in &rules {
            for rune_usage in rule.rune_usages() {
                assert!(initial_rune_to_type.contains_key(&rune_usage.rune));
            }
        }

        // These two shouldn't both be in the rules, see SROACSD.
        assert!(
            rules.iter().all(|r| !matches!(r, IRulexSR::CallSiteFunc(_))) ||
            rules.iter().all(|r| !matches!(r, IRulexSR::DefinitionFunc(_))));
        // These two shouldn't both be in the rules, see SROACSD.
        assert!(
            rules.iter().all(|r| !matches!(r, IRulexSR::CallSiteCoordIsa(_))) ||
            rules.iter().all(|r| !matches!(r, IRulexSR::DefinitionCoordIsa(_))));

        for (rune, templata) in &initially_known_rune_to_templata {
            if self.opts.global_options.sanity_check {
                self.sanity_check_conclusion(&env, state, *rune, *templata);
            }
            assert_eq!(templata.tyype(self.scout_arena), *initial_rune_to_type.get(rune).unwrap());
        }

        let all_runes: Vec<IRuneS<'s>> = initial_rune_to_type.keys().copied().collect();

        let rule_to_puzzles: Box<dyn Fn(&IRulexSR<'s>) -> Vec<Vec<IRuneS<'s>>>> =
            Box::new(|rule| get_puzzles(*rule));
        let rule_to_runes: &dyn Fn(&IRulexSR<'s>) -> Vec<IRuneS<'s>> =
            &|rule| self.get_runes(*rule);

        crate::solver::solver::make_solver_state(
            self.opts.global_options.sanity_check,
            self.opts.global_options.use_optimized_solver,
            rule_to_puzzles,
            rule_to_runes,
            rules,
            initially_known_rune_to_templata,
            all_runes,
        )
    }
/*
  def makeSolverState(
    range: List[RangeS],
    env: InferEnv,
    state: CompilerOutputs,
    rules: IndexedSeq[IRulexSR],
    initialRuneToType: Map[IRuneS, ITemplataType],
    initiallyKnownRuneToTemplata: Map[IRuneS, ITemplataT[ITemplataType]]):
  SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]] = {

    rules.foreach(rule => rule.runeUsages.foreach(rune => vassert(initialRuneToType.contains(rune.rune))))

    // These two shouldn't both be in the rules, see SROACSD.
    vassert(
      rules.collect({ case CallSiteFuncSR(range, _, _, _, _) => }).isEmpty ||
        rules.collect({ case DefinitionFuncSR(range, _, _, _, _) => }).isEmpty)
    // These two shouldn't both be in the rules, see SROACSD.
    vassert(
      rules.collect({ case CallSiteCoordIsaSR(range, _, _, _) => }).isEmpty ||
        rules.collect({ case DefinitionCoordIsaSR(range, _, _, _) => }).isEmpty)

    initiallyKnownRuneToTemplata.foreach({ case (rune, templata) =>
      if (globalOptions.sanityCheck) {
        delegate.sanityCheckConclusion(env, state, rune, templata)
      }
      vassert(templata.tyype == vassertSome(initialRuneToType.get(rune)))
    })

    val solver =
      Solver.makeSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]](
        globalOptions.sanityCheck,
        globalOptions.useOptimizedSolver,
//        interner,
        (rule: IRulexSR) => getPuzzles(rule),
        getRunes,
//        new CompilerRuleSolver(globalOptions.sanityCheck, interner, delegate, initialRuneToType),
//        range,
        rules,
        initiallyKnownRuneToTemplata,
        initialRuneToType.keys.toVector.distinct)

    solver
  }


*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn advance_infer(
        &self,
        env: InferEnv<'s, 't>,
        state: &mut CompilerOutputs<'s, 't>,
        solver_state: &mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>,
    ) -> Result<bool, FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>> {
        //   solverState.sanityCheck()
        solver_state.sanity_check();
        for (_rune, _conclusion) in solver_state.userify_conclusions() {
            // Scala calls sanityCheckConclusion here; skipped for now
        }
        // Stage 1: Do simple solves
        match solver_state.get_next_solvable() {
            None => {}
            Some(solving_rule_index) => {
                let rule = *solver_state.get_rule(solving_rule_index);
                let steps_before = solver_state.get_steps().len();
                match self.solve(state, env, solver_state, solving_rule_index, rule) {
                    Ok(()) => {}
                    Err(e) => return Err(FailedSolve {
                        steps: solver_state.get_steps(),
                        conclusions: solver_state.get_conclusions().into_iter().collect(),
                        unsolved_rules: solver_state.get_unsolved_rules(),
                        unsolved_runes: solver_state.get_unsolved_runes(),
                        error: e,
                    }),
                }
                let steps_after = solver_state.get_steps().len();
                assert!(steps_after == steps_before + 1);
                // Per @CSCDSRZ, only true after simple solve.
                assert!(solver_state.rule_is_solved(solving_rule_index));
                solver_state.sanity_check();
                return Ok(true);
            }
        }
        // Stage 2: Do a complex solve if available.
        if !solver_state.get_unsolved_rules().is_empty() {
            let conclusions_before = solver_state.get_conclusions().len();
            match complex_solve(self, self.typing_interner, state, env, solver_state) {
                Ok(()) => {}
                Err(e) => return Err(FailedSolve {
                    steps: solver_state.get_steps(),
                    conclusions: solver_state.get_conclusions().into_iter().collect(),
                    unsolved_rules: solver_state.get_unsolved_rules(),
                    unsolved_runes: solver_state.get_unsolved_runes(),
                    error: e,
                }),
            }
            solver_state.sanity_check();
            let conclusions_after = solver_state.get_conclusions().len();
            // Per @CSCDSRZ, check conclusion count (not rules solved) for progress.
            if conclusions_after == conclusions_before {
                // There's nothing more to be done. Let's continue on to stage 3.
            } else {
                return Ok(true);
            }
        } else {
            // No more rules to solve, so continue to the wrapping up stages of the solve.
        }
        // Stage 3: We're done!
        Ok(false)
    }
/*
  // Returns true if there's more to be done, false if we've gotten as far as we can.
  def advanceInfer(
      env: InferEnv,
      state: CompilerOutputs,
      solverState: SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]],
      delegate: IInfererDelegate):
  Result[Boolean, FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]] = {
    solverState.sanityCheck()
    solverState.userifyConclusions().foreach({ case (rune, conclusion) =>
      CompilerRuleSolver.sanityCheckConclusion(delegate, env, state, rune, conclusion)
    })
    // Stage 1: Do simple solves
    solverState.getNextSolvable() match {
      case None => // continue onto the next stage
      case Some(solvingRuleIndex) => {
        val rule = solverState.getRule(solvingRuleIndex)
        val stepsBefore = solverState.getSteps().size
        CompilerRuleSolver.solve(delegate, state, env, solverState, solvingRuleIndex, rule) match {
          case Ok(()) => {}
          case Err(e) => return Err(FailedSolve(solverState.getSteps(), solverState.getConclusions().toMap, solverState.getUnsolvedRules(), solverState.getUnsolvedRunes(), e))
        }
        val stepsAfter = solverState.getSteps().size
        vassert(stepsAfter == stepsBefore + 1)
        vassert(solverState.ruleIsSolved(solvingRuleIndex)) // Per @CSCDSRZ, only true after simple solve.
        solverState.sanityCheck()
        // Go back to the beginning. Next step, if there's no simple rule ready to solve, then
        // it'll start doing a complex solve if available, or just finish.
        return Ok(true)
      }
    }
    // Stage 2: Do a complex solve if available.
    // Per @CSCDSRZ, complex solve only adds conclusions — we check conclusion count for progress.
    if (solverState.getUnsolvedRules().nonEmpty) {
      val conclusionsBefore = solverState.getConclusions().toMap.size
      CompilerRuleSolver.complexSolve(delegate, state, env, solverState) match {
        case Ok(()) =>
        case Err(e) => return Err(FailedSolve(solverState.getSteps(), solverState.getConclusions().toMap, solverState.getUnsolvedRules(), solverState.getUnsolvedRunes(), e))
      }
      solverState.sanityCheck()
      val conclusionsAfter = solverState.getConclusions().toMap.size
      // Per @CSCDSRZ, check conclusion count (not rules solved) for progress.
      if (conclusionsAfter == conclusionsBefore) {
        // There's nothing more to be done. Let's continue on to stage 3.
      } else {
        return Ok(true) // Go back to stage 1 where the new conclusions may unblock simple solves.
      }
    } else {
      // No more rules to solve, so continue to the wrapping up stages of the solve.
    }
    // Stage 3: We're done! The user should look at the conclusions to see if they're all solved,
    // and they can even add more rules if they want.
    Ok(false)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn continue_solver(
        &self,
        env: InferEnv<'s, 't>,
        state: &mut CompilerOutputs<'s, 't>,
        solver_state: &mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>,
    ) -> Result<(), FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>> {
        //   while ( {
        while {
            //     advanceInfer(
            //       env, state, solverState, delegate
            //     ) match {
            //       case Ok(continue) => continue
            //       case Err(f@FailedSolve(_, _, _, _, _)) => return Err(f)
            //     }
            self.advance_infer(env, state, solver_state)?
        } {}
        //   // If we get here, then there's nothing more the solver can do.
        //   Ok(Unit)
        Ok(())
    }
/*
  // During the solve, we postponed resolving structs and interfaces, see SFWPRL.
  // Caller should remember to do that!
  def continue(
    env: InferEnv,
    state: CompilerOutputs,
    solverState: SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]]):
  Result[Unit, FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]] = {
    while ( {
      advanceInfer(
        env, state, solverState, delegate
      ) match {
        case Ok(continue) => continue
        case Err(f@FailedSolve(_, _, _, _, _)) => return Err(f)
      }
    }) {}
    // If we get here, then there's nothing more the solver can do.
    Ok(Unit)
  }
}

object CompilerRuleSolver {
*/
}

pub fn sanity_check_conclusion<'s, 't>(
    env: InferEnv<'s, 't>,
    state: CompilerOutputs<'s, 't>,
    rune: IRuneS<'s>,
    conclusion: ITemplataT<'s, 't>,
) {
    panic!("Unimplemented: sanity_check_conclusion");
}
/*
  def sanityCheckConclusion(delegate: IInfererDelegate, env: InferEnv, state: CompilerOutputs, rune: IRuneS, conclusion: ITemplataT[ITemplataType]): Unit = {
    delegate.sanityCheckConclusion(env, state, rune, conclusion)
  }

*/
fn complex_solve<'s, 'ctx, 't, 'a>(
    compiler: &'a Compiler<'s, 'ctx, 't>,
    typing_interner: &'a TypingInterner<'s, 't>,
    state: &mut CompilerOutputs<'s, 't>,
    env: InferEnv<'s, 't>,
    solver_state: &mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>,
) -> Result<(), ISolverError<IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>>
where 's: 't,
{
    complex_solve_inner(compiler, typing_interner, state, env, solver_state)
}
/*
  // Per @CSCDSRZ, complex solve infers conclusions from unsolved rules but doesn't solve them.
  def complexSolve(
      delegate: IInfererDelegate,
      state: CompilerOutputs,
      env: InferEnv,
      solverState: SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]]):
  Result[Unit, ISolverError[IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]] = {
    complexSolveInner(delegate, state, env, solverState)
  }

*/
fn complex_solve_inner<'s, 'ctx, 't, 'a>(
    compiler: &'a Compiler<'s, 'ctx, 't>,
    typing_interner: &'a TypingInterner<'s, 't>,
    state: &mut CompilerOutputs<'s, 't>,
    env: InferEnv<'s, 't>,
    solver_state: &mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>,
) -> Result<(), ISolverError<IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>>
where 's: 't,
{
    let _env = env;
    let unsolved_rules = solver_state.get_unsolved_rules();

    let unsolved_receiver_runes: Vec<IRuneS<'s>> = unsolved_rules.iter().filter_map(|rule| {
        match rule {
            IRulexSR::CoordSend(r) => Some(r.receiver_rune.rune),
            IRulexSR::CallSiteCoordIsa(r) => Some(r.super_rune.rune),
            _ => None,
        }
    }).collect();

    let receiver_runes = crate::postparsing::rules::rule_scout::get_kind_equivalent_runes_iter(
        &unsolved_rules,
        unsolved_receiver_runes.into_iter(),
    );

    let new_conclusions: HashMap<IRuneS<'s>, ITemplataT<'s, 't>> = receiver_runes.iter().filter_map(|receiver| {
        let runes_sending_to_this_receiver = crate::postparsing::rules::rule_scout::get_kind_equivalent_runes_iter(
            &unsolved_rules,
            unsolved_rules.iter().filter_map(|rule| match rule {
                IRulexSR::CoordSend(r) if r.receiver_rune.rune == *receiver => Some(r.sender_rune.rune),
                IRulexSR::CallSiteCoordIsa(r) if r.super_rune.rune == *receiver => Some(r.sub_rune.rune),
                _ => None,
            }),
        );
        let call_rules_template_runes: Vec<IRuneS<'s>> =
            unsolved_rules.iter().filter_map(|rule| match rule {
                IRulexSR::Call(r) if receiver_runes.contains(&r.result_rune.rune) => Some(r.template_rune.rune),
                _ => None,
            }).collect();
        let sender_conclusions: Vec<(IRuneS<'s>, CoordT<'s, 't>)> =
            runes_sending_to_this_receiver.iter().filter_map(|sender_rune| {
                solver_state.get_conclusion(sender_rune).and_then(|templata| match templata {
                    ITemplataT::Coord(ct) => Some((*sender_rune, ct.coord)),
                    _ => panic!("vwat: sender conclusion not a coord: {:?}", templata),
                })
            }).collect();
        let call_templates: Vec<ITemplataT<'s, 't>> =
            crate::postparsing::rules::rule_scout::get_kind_equivalent_runes_iter(
                &unsolved_rules,
                call_rules_template_runes.iter().copied(),
            ).iter().filter_map(|rune| solver_state.get_conclusion(rune)).collect();
        assert!(call_templates.iter().map(|t| *t).collect::<std::collections::HashSet<_>>().len() <= 1);
        let all_senders_known = sender_conclusions.len() == runes_sending_to_this_receiver.len();
        let all_calls_known = call_rules_template_runes.len() == call_templates.len();
        match solve_receives(compiler, typing_interner, state, _env, sender_conclusions.clone(), call_templates, all_senders_known, all_calls_known) {
            Err(e) => return Some(Err(ISolverError::RuleError(RuleError { err: e, _phantom: std::marker::PhantomData }))),
            Ok(None) => return None,
            Ok(Some(receiver_instantiation_kind)) => {
                let possible_coords: Vec<CoordT<'s, 't>> = {
                    let mut v: Vec<CoordT<'s, 't>> = unsolved_rules.iter().filter_map(|rule| match rule {
                        IRulexSR::Augment(r) if r.result_rune.rune == *receiver => {
                            let ownership = evaluate_ownership(r.ownership.expect("vassertSome: augment ownership"));
                            Some(CoordT { ownership, region: RegionT { region: IRegionT::Default }, kind: receiver_instantiation_kind })
                        }
                        _ => None,
                    }).collect();
                    for (_, coord) in sender_conclusions.iter() {
                        v.push(CoordT { ownership: coord.ownership, region: RegionT { region: IRegionT::Default }, kind: receiver_instantiation_kind });
                    }
                    v
                };
                if possible_coords.is_empty() {
                    Some(Ok((*receiver, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: receiver_instantiation_kind })))))
                } else {
                    let ownerships: std::collections::HashSet<OwnershipT> = possible_coords.iter().map(|c| c.ownership).collect();
                    let ownership = match ownerships.len() {
                        0 => panic!("vwat: no ownerships in possible_coords"),
                        1 => *ownerships.iter().next().unwrap(),
                        _ => {
                            let params = typing_interner.alloc_slice_from_vec(sender_conclusions);
                            return Some(Err(ISolverError::RuleError(RuleError { err: ITypingPassSolverError::ReceivingDifferentOwnerships { params }, _phantom: std::marker::PhantomData })));
                        }
                    };
                    Some(Ok((*receiver, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT {
                        coord: CoordT { ownership, region: RegionT { region: IRegionT::Default }, kind: receiver_instantiation_kind },
                    })))))
                }
            }
        }
    }).collect::<Result<HashMap<_, _>, _>>().map_err(|e| e)?;

    // Per @CSCDSRZ, complex solve only produces conclusions — empty solvedRules and newRules is correct.
    match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(true, vec![], new_conclusions, vec![], std::collections::HashSet::new()) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    Ok(())
}
/*
  private def complexSolveInner(delegate: IInfererDelegate, state: CompilerOutputs, env: InferEnv, solverState: SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]]): Result[Unit, ISolverError[IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]] = {
    val equivalencies = new Equivalencies(solverState.getUnsolvedRules())

    val unsolvedRules = solverState.getUnsolvedRules()
    val (unsolvedReceiverRunes, ranges) =
      unsolvedRules.collect({
        case CoordSendSR(range, _, receiverRune) => (receiverRune.rune, range)
        // We don't do this for DefinitionCoordIsaSR, see RRBFS.
        // case DefinitionCoordIsaSR(range, _, _, receiverRune) => (receiverRune.rune, range)
        case CallSiteCoordIsaSR(range, _, _, receiverRune) => (receiverRune.rune, range)
      }).unzip
    val receiverRunes =
      equivalencies.getKindEquivalentRunes(unsolvedReceiverRunes)

    val newConclusions =
      receiverRunes.flatMap(receiver => {
        val runesSendingToThisReceiver =
          equivalencies.getKindEquivalentRunes(
            unsolvedRules.collect({
              case CoordSendSR(range, s, r) if r.rune == receiver => s.rune
              // We don't do this for DefinitionCoordIsaSR, see RRBFS.
              // case DefinitionCoordIsaSR(range, _, s, r) if r.rune == receiver => s.rune
              case CallSiteCoordIsaSR(range, _, s, r) if r.rune == receiver => s.rune
            }))
        val callRulesTemplateRunes =
          unsolvedRules
              .collect({
                case z@CallSR(range, r, templateRune, _) if equivalencies.getKindEquivalentRunes(r.rune).contains(receiver) => templateRune
              })
        val senderConclusions =
          runesSendingToThisReceiver
              .flatMap(senderRune => solverState.getConclusion(senderRune).map(senderRune -> _))
              .map({
                case (senderRune, CoordTemplataT(coord)) => (senderRune -> coord)
                case other => vwat(other)
              })
              .toVector
        val callTemplates =
          equivalencies.getKindEquivalentRunes(
                callRulesTemplateRunes.map(_.rune))
              .flatMap(solverState.getConclusion)
              .toVector
        vassert(callTemplates.distinct.size <= 1)
        // If true, there are some senders/constraints we don't know yet, so lets be
        // careful to not assume between any possibilities below.
        val allSendersKnown = senderConclusions.size == runesSendingToThisReceiver.size
        val allCallsKnown = callRulesTemplateRunes.size == callTemplates.size
        solveReceives(delegate, env, state, senderConclusions, callTemplates, allSendersKnown, allCallsKnown) match {
          case Err(e) => return Err(RuleError(e))
          case Ok(None) => None
          case Ok(Some(receiverInstantiationKind)) => {
            // We know the kind, but to really know the coord we have to look at all the rules that
            // factored into it, and may even have to default to something else.

            val possibleCoords =
              unsolvedRules.collect({
                case AugmentSR(range, resultRune, ownership, innerRune)
                  if resultRune.rune == receiver => {
                  CoordT(
                    Conversions.evaluateOwnership(vassertSome(ownership)),
                    RegionT(DefaultRegionT),
                    receiverInstantiationKind)
                }
              }) ++
                  senderConclusions.map(_._2).map({ case CoordT(ownership, _, _) =>
                    CoordT(ownership, RegionT(DefaultRegionT), receiverInstantiationKind)
                  })
            if (possibleCoords.nonEmpty) {
              val ownership =
                possibleCoords.map(_.ownership).distinct match {
                  case Vector() => vwat()
                  case Vector(ownership) => ownership
                  case _ => return Err(RuleError(ReceivingDifferentOwnerships(senderConclusions)))
                }
              val region = RegionT(DefaultRegionT)
              Some(receiver -> CoordTemplataT(CoordT(ownership, region, receiverInstantiationKind)))
            } else {
              // Just conclude a kind, which will coerce to an owning coord, and hope it's right.
              Some(receiver -> templata.KindTemplataT(receiverInstantiationKind))
            }
          }
        }
      }).toMap

    // Per @CSCDSRZ, complex solve only produces conclusions — empty solvedRules and newRules is correct.
    solverState.commitStep[ITypingPassSolverError](true, Vector(), newConclusions, Vector()) match {
      case Ok(_) =>
      case Err(e) => return Err(e)
    }

    //
    //    newConclusions.foreach({ case (rune, conclusion) =>
    //      solverState.concludeRune[ITypingPassSolverError](rune, conclusion) match { case Ok(_) => case Err(e) => return Err(e) }
    //    })

    Ok(())
  }

*/
fn solve_receives<'s, 'ctx, 't>(
    compiler: &Compiler<'s, 'ctx, 't>,
    typing_interner: &TypingInterner<'s, 't>,
    state: &mut CompilerOutputs<'s, 't>,
    env: InferEnv<'s, 't>,
    senders: Vec<(IRuneS<'s>, CoordT<'s, 't>)>,
    call_templates: Vec<ITemplataT<'s, 't>>,
    all_senders_known: bool,
    all_calls_known: bool,
) -> Result<Option<KindT<'s, 't>>, ITypingPassSolverError<'s, 't>>
where 's: 't,
{
    let sender_kinds: Vec<KindT<'s, 't>> = senders.iter().map(|(_, coord)| coord.kind).collect();
    if sender_kinds.is_empty() {
        return Ok(None);
    }
    let sender_ancestor_lists: Vec<std::collections::HashSet<KindT<'s, 't>>> =
        sender_kinds.iter().map(|kind| compiler.get_ancestors(env, state, *kind, true)).collect();
    let common_ancestors: std::collections::HashSet<KindT<'s, 't>> =
        sender_ancestor_lists.into_iter().reduce(|a, b| a.intersection(&b).copied().collect())
            .unwrap_or_default();
    if common_ancestors.is_empty() {
        let params = typing_interner.alloc_slice_from_vec(senders);
        return Err(ITypingPassSolverError::NoCommonAncestors { params });
    }
    let common_ancestors_call_constrained: std::collections::HashSet<KindT<'s, 't>> =
        if call_templates.is_empty() {
            common_ancestors
        } else {
            common_ancestors.into_iter().filter(|ancestor| {
                call_templates.iter().any(|template| compiler.kind_is_from_template(state, *ancestor, *template))
            }).collect()
        };
    let narrowed_common_ancestor =
        if common_ancestors_call_constrained.is_empty() {
            let params = typing_interner.alloc_slice_from_vec(senders);
            return Err(ITypingPassSolverError::NoAncestorsSatisfyCall { params });
        } else if common_ancestors_call_constrained.len() == 1 {
            *common_ancestors_call_constrained.iter().next().unwrap()
        } else {
            if !all_senders_known {
                return Ok(None);
            }
            if !all_calls_known {
                return Ok(None);
            }
            match narrow(compiler, typing_interner, env, state, common_ancestors_call_constrained) {
                Ok(x) => x,
                Err(e) => return Err(e),
            }
        };
    Ok(Some(narrowed_common_ancestor))
}
/*
  private def solveReceives(
    delegate: IInfererDelegate,
    env: InferEnv,
    state: CompilerOutputs,
    senders: Vector[(IRuneS, CoordT)],
    callTemplates: Vector[ITemplataT[ITemplataType]],
    allSendersKnown: Boolean,
    allCallsKnown: Boolean):
  Result[Option[KindT], ITypingPassSolverError] = {
    val senderKinds = senders.map(_._2.kind)
    if (senderKinds.isEmpty) {
      return Ok(None)
    }

    // For example [Flamethrower, Rockets] becomes [[Flamethrower, IWeapon, ISystem], [Rockets, IWeapon, ISystem]]
    val senderAncestorLists = senderKinds.map(delegate.getAncestors(env, state, _, true))
    // Calculates the intersection of them all, eg [IWeapon, ISystem]
    val commonAncestors = senderAncestorLists.reduce(_.intersect(_))
    if (commonAncestors.size == 0) {
      return Err(NoCommonAncestors(senders))
    }
    // Filter by any call templates. eg if there's a X = ISystem:Y call, then we're now [ISystem]
    val commonAncestorsCallConstrained =
      if (callTemplates.isEmpty) {
        commonAncestors
      } else {
        commonAncestors.filter(ancestor => callTemplates.exists(template => delegate.kindIsFromTemplate(state,ancestor, template)))
      }

    val narrowedCommonAncestor =
      if (commonAncestorsCallConstrained.size == 0) {
        // If we get here, it means we passed in a bunch of nonsense that doesn't match our Call rules.
        // For example, passing in a Some<T> when a List<T> is expected.
        return Err(NoAncestorsSatisfyCall(senders))
      } else if (commonAncestorsCallConstrained.size == 1) {
        // If we get here, it doesn't matter if there are any other senders or calls, we know
        // it has to be this.
        // If we're wrong, it will be doublechecked by the solver anyway.
        commonAncestorsCallConstrained.head
      } else {
        if (!allSendersKnown) {
          // There are some senders out there, which might force us to choose one of the ancestors.
          // We don't know them yet, so we can't conclude anything.
          return Ok(None)
        }
        if (!allCallsKnown) {
          // There are some calls out there, which will determine which one of the possibilities it is.
          // We don't know them yet, so we can't conclude anything.
          return Ok(None)
        }
        // If there are multiple, like [IWeapon, ISystem], get rid of any that are parents of others, now [IWeapon].
        narrow(delegate, env, state, commonAncestorsCallConstrained) match {
          case Ok(x) => x
          case Err(e) => return Err(e)
        }
      }
    Ok(Some(narrowedCommonAncestor))
  }

*/
fn narrow<'s, 'ctx, 't, 'a>(
    compiler: &'a Compiler<'s, 'ctx, 't>,
    typing_interner: &'a TypingInterner<'s, 't>,
    env: InferEnv<'s, 't>,
    state: &mut CompilerOutputs<'s, 't>,
    kinds: HashSet<KindT<'s, 't>>,
) -> Result<KindT<'s, 't>, ITypingPassSolverError<'s, 't>>
where 's: 't,
{
    assert!(kinds.len() > 1);
    let mut narrowed_ancestors: HashSet<KindT<'s, 't>> = kinds.iter().copied().collect();
    for kind in kinds.iter() {
        let ancestors = compiler.get_ancestors(env, state, *kind, false);
        for ancestor in ancestors {
            narrowed_ancestors.remove(&ancestor);
        }
    }
    if narrowed_ancestors.is_empty() {
        panic!("vwat: narrowed_ancestors empty in narrow");
    } else if narrowed_ancestors.len() == 1 {
        Ok(*narrowed_ancestors.iter().next().unwrap())
    } else {
        let kinds_slice = typing_interner.alloc_slice_from_vec(narrowed_ancestors.into_iter().collect());
        Err(ITypingPassSolverError::CantDetermineNarrowestKind { kinds: kinds_slice })
    }
}
/*
  def narrow(
    delegate: IInfererDelegate,
    env: InferEnv,
    state: CompilerOutputs,
    kinds: Set[KindT]):
  Result[KindT, ITypingPassSolverError] = {
    vassert(kinds.size > 1)
    val narrowedAncestors = mutable.HashSet[KindT]()
    narrowedAncestors ++= kinds
    // Remove anything that's an ancestor of something else in the set
    kinds.foreach(kind => {
      narrowedAncestors --= delegate.getAncestors(env, state, kind, false)
    })
    if (narrowedAncestors.size == 0) {
      vwat() // Shouldnt happen
    } else if (narrowedAncestors.size == 1) {
      Ok(narrowedAncestors.head)
    } else {
      Err(CantDetermineNarrowestKind(narrowedAncestors.toSet))
    }
  }

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    fn solve(
        &self,
        state: &mut CompilerOutputs<'s, 't>,
        env: InferEnv<'s, 't>,
        solver_state: &mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>,
        rule_index: i32,
        rule: IRulexSR<'s>,
    ) -> Result<(), ISolverError<IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>> {
        //   solveRule(delegate, state, env, ruleIndex, rule, solverState) match {
        //     case Ok(x) => Ok(x)
        //     case Err(e) => Err(RuleError(e))
        //   }
        match self.solve_rule(state, env, rule_index, rule, solver_state) {
            Ok(x) => Ok(x),
            Err(e) => Err(ISolverError::RuleError(RuleError { err: e, _phantom: std::marker::PhantomData })),
        }
    }
}
/*
  def solve(
    delegate: IInfererDelegate,
    state: CompilerOutputs,
    env: InferEnv,
    solverState: SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]],
    ruleIndex: Int,
    rule: IRulexSR):
  Result[Unit, ISolverError[IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]] = {
    solveRule(delegate, state, env, ruleIndex, rule, solverState) match {
      case Ok(x) => Ok(x)
      case Err(e) => Err(RuleError(e))
    }
  }

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    fn solve_rule(
        &self,
        state: &mut CompilerOutputs<'s, 't>,
        env: InferEnv<'s, 't>,
        rule_index: i32,
        rule: IRulexSR<'s>,
        solver_state: &mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>,
    ) -> Result<(), ITypingPassSolverError<'s, 't>> {
        //   rule match {
        match rule {
            //     case KindComponentsSR(...) =>
            //     case KindComponentsSR(range, kindRune, mutabilityRune) => {
            IRulexSR::KindComponents(kc) => {
                let kind = match solver_state.get_conclusion(&kc.kind_rune.rune).expect("kind rune not solved in KindComponentsSR") {
                    ITemplataT::Kind(kt) => kt.kind,
                    _ => panic!("Expected KindTemplataT in KindComponentsSR"),
                };
                let mutability = self.get_mutability(state, kind);
                let mut conclusions = HashMap::new();
                conclusions.insert(kc.mutability_rune.rune, mutability);
                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let ranges = std::iter::once(kc.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                        let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                        let error = self.typing_interner.alloc(e);
                        Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                    }
                }
            }
            //     case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => {
            IRulexSR::CoordComponents(cc) => {
                match solver_state.get_conclusion(&cc.result_rune.rune) {
                    None => {
                        let ownership = match solver_state.get_conclusion(&cc.ownership_rune.rune).expect("ownership rune not solved in CoordComponentsSR") {
                            ITemplataT::Ownership(ot) => ot.ownership,
                            _ => panic!("Expected OwnershipTemplataT in CoordComponentsSR"),
                        };
                        let kind = match solver_state.get_conclusion(&cc.kind_rune.rune).expect("kind rune not solved in CoordComponentsSR") {
                            ITemplataT::Kind(kt) => kt.kind,
                            _ => panic!("Expected KindTemplataT in CoordComponentsSR"),
                        };
                        let new_coord = match self.get_mutability(state, kind) {
                            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => {
                                CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind }
                            }
                            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) | ITemplataT::Placeholder(PlaceholderTemplataT { .. }) => {
                                CoordT { ownership, region: RegionT { region: IRegionT::Default }, kind }
                            }
                            other => panic!("implement: CoordComponents unexpected mutability {:?}", other),
                        };
                        let new_templata = ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord: new_coord }));
                        let mut conclusions = HashMap::new();
                        conclusions.insert(cc.result_rune.rune, new_templata);
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let ranges = std::iter::once(cc.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                    Some(coord_templata) => {
                        let coord = match coord_templata {
                            ITemplataT::Coord(ct) => ct.coord,
                            _ => panic!("Expected CoordTemplataT in CoordComponentsSR result"),
                        };
                        let mut conclusions = HashMap::new();
                        conclusions.insert(cc.ownership_rune.rune, ITemplataT::Ownership(OwnershipTemplataT { ownership: coord.ownership }));
                        conclusions.insert(cc.kind_rune.rune, ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: coord.kind })));
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let ranges = std::iter::once(cc.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                }
            }
            //     case PrototypeComponentsSR(...) =>
            IRulexSR::PrototypeComponents(_) => { panic!("Unimplemented: solve_rule PrototypeComponents"); }
            //     case ResolveSR(range, resultRune, name, paramListRune, returnRune) => {
            IRulexSR::Resolve(resolve) => {
                // If we're here, then we're resolving a prototype.
                // This happens at the call-site.
                // The function (or struct) can either supply a default resolve rule (usually
                // via the `func moo(int)void` syntax) or let the caller pass it in.
                let param_coords = match solver_state.get_conclusion(&resolve.params_list_rune.rune).expect("paramListRune not solved in ResolveSR") {
                    ITemplataT::CoordList(cl) => cl.coords,
                    _ => panic!("Expected CoordListTemplataT in ResolveSR paramListRune"),
                };
                //       solverState.getConclusion(returnRune.rune) match {
                //         case Some(CoordTemplataT(returnCoord)) => {
                match solver_state.get_conclusion(&resolve.return_rune.rune) {
                    Some(ITemplataT::Coord(ct)) => {
                        // Existing predict path: both params and return are known. We only pretend
                        // the function exists for now; actual resolution is postponed to after the
                        // solve completes. See SFWPRL in docs/Generics.md:353.
                        let return_coord = ct.coord;
                        let prototype_templata = self.predict_function(env, state, resolve.range, resolve.name, param_coords, return_coord);
                        let new_templata = ITemplataT::Prototype(self.typing_interner.alloc(prototype_templata));
                        let mut conclusions = HashMap::new();
                        conclusions.insert(resolve.result_rune.rune, new_templata);
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let ranges = std::iter::once(resolve.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                    Some(_) => panic!("Expected CoordTemplataT in ResolveSR returnRune"),
                    //         case None => {
                    None => {
                        // Per @BRRZ, params are known but return isn't. Scala does a real overload
                        // lookup via delegate.resolveFunction (the same delegate method the
                        // post-solve phase uses at InferCompiler.scala:350) to discover the return
                        // type and unblock the solver. Safety of this mid-solve lookup is documented
                        // in CompilerSolver.scala:668-680. Not yet migrated — the second puzzle path
                        // (params-only solved) doesn't yet have a Rust test exercising it.
                        panic!("Unimplemented: ResolveSR @BRRZ None branch — real overload lookup via resolve_function (see Frontend/TypingPass/.../CompilerSolver.scala:668-691)");
                    }
                }
            }
            //     case CallSiteFuncSR(range, prototypeRune, name, paramListRune, returnRune) => {
            IRulexSR::CallSiteFunc(csf) => {
                // If we're here, then we're solving in the callsite, not the definition.
                // This should look up a function with that name and param list, and make sure
                // its return matches.
                match solver_state.get_conclusion(&csf.prototype_rune.rune).expect("prototypeRune not solved in CallSiteFuncSR") {
                    ITemplataT::Prototype(proto_templata) => {
                        let prototype = proto_templata.prototype;
                        let mut conclusions = HashMap::new();
                        conclusions.insert(csf.params_list_rune.rune, ITemplataT::CoordList(self.typing_interner.alloc(CoordListTemplataT { coords: prototype.param_types() })));
                        conclusions.insert(csf.return_rune.rune, ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord: prototype.return_type })));
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let ranges = std::iter::once(csf.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                    _ => {
                        let ranges = std::iter::once(csf.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                        let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                        Err(ITypingPassSolverError::CantCheckPlaceholder { range: ranges_slice })
                    }
                }
            }
            //     case DefinitionFuncSR(range, resultRune, name, paramListRune, returnRune) => {
            IRulexSR::DefinitionFunc(def_func) => {
                let param_coords = match solver_state.get_conclusion(&def_func.params_list_rune.rune).expect("DefinitionFunc paramListRune has no conclusion") {
                    ITemplataT::CoordList(cl) => cl.coords,
                    _ => panic!("implement: solve_rule DefinitionFunc non-CoordList paramList"),
                };
                let return_type = match solver_state.get_conclusion(&def_func.return_rune.rune).expect("DefinitionFunc returnRune has no conclusion") {
                    ITemplataT::Coord(ct) => ct.coord,
                    _ => panic!("implement: solve_rule DefinitionFunc non-Coord return"),
                };
                let new_prototype = self.assemble_prototype(env, state, def_func.range, def_func.name, param_coords, return_type);
                let new_templata = ITemplataT::Prototype(self.typing_interner.alloc(PrototypeTemplataT { prototype: new_prototype }));
                let mut conclusions = HashMap::new();
                conclusions.insert(def_func.result_rune.rune, new_templata);
                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                    Ok(_) => Ok(()),
                    Err(_e) => { panic!("implement: solve_rule DefinitionFunc InternalSolverError wrapping"); }
                }
            }
            //     case CallSiteCoordIsaSR(...) =>
            IRulexSR::CallSiteCoordIsa(csia) => {
                let sub_templata = solver_state.get_conclusion(&csia.sub_rune.rune)
                    .expect("vassertSome: subRune not solved in CallSiteCoordIsaSR");
                let sub_coord = match sub_templata {
                    ITemplataT::Coord(ct) => ct.coord,
                    _ => panic!("Expected CoordTemplataT for subRune in CallSiteCoordIsaSR"),
                };
                let super_templata = solver_state.get_conclusion(&csia.super_rune.rune)
                    .expect("vassertSome: superRune not solved in CallSiteCoordIsaSR");
                let super_coord = match super_templata {
                    ITemplataT::Coord(ct) => ct.coord,
                    _ => panic!("Expected CoordTemplataT for superRune in CallSiteCoordIsaSR"),
                };

                let resulting_isa_templata: ITemplataT<'s, 't> = if sub_coord == super_coord {
                    ITemplataT::Isa(self.typing_interner.alloc(self.assemble_impl(env, csia.range, sub_coord.kind, super_coord.kind)))
                } else if matches!(sub_coord.kind, KindT::Never(_)) {
                    ITemplataT::Isa(self.typing_interner.alloc(self.assemble_impl(env, csia.range, sub_coord.kind, super_coord.kind)))
                } else {
                    let sub_kind = match ISubKindTT::try_from(sub_coord.kind) {
                        Ok(k) => k,
                        Err(_) => return Err(ITypingPassSolverError::BadIsaSubKind { kind: sub_coord.kind }),
                    };
                    let super_kind = match ISuperKindTT::try_from(super_coord.kind) {
                        Ok(k) => k,
                        Err(_) => return Err(ITypingPassSolverError::BadIsaSuperKind { kind: super_coord.kind }),
                    };
                    match self.is_parent(state, env.original_calling_env, env.parent_ranges, env.call_location, sub_kind, super_kind) {
                        IsParentResult::IsntParent(_) => return Err(ITypingPassSolverError::IsaFailed { sub: sub_coord.kind, suuper: super_coord.kind }),
                        IsParentResult::IsParent(is_parent) => is_parent.templata,
                    }
                };

                let mut conclusions = HashMap::new();
                if let Some(result_rune) = csia.result_rune {
                    conclusions.insert(result_rune.rune, resulting_isa_templata);
                }
                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let ranges = std::iter::once(csia.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                        let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                        let error = self.typing_interner.alloc(e);
                        Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                    }
                }
            }
            //     case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => {
            IRulexSR::DefinitionCoordIsa(dcia) => {
                // If we're here, then we're solving in the definition, not the callsite.
                // Skip checking that they match, just assume they do.
                let sub_templata = solver_state.get_conclusion(&dcia.sub_rune.rune)
                    .expect("vassertSome: subRune not solved in DefinitionCoordIsaSR");
                let sub_kind_unchecked = match sub_templata {
                    ITemplataT::Coord(ct) => ct.coord.kind,
                    _ => panic!("Expected CoordTemplataT for subRune in DefinitionCoordIsaSR"),
                };
                let super_templata = solver_state.get_conclusion(&dcia.super_rune.rune)
                    .expect("vassertSome: superRune not solved in DefinitionCoordIsaSR");
                let super_kind_unchecked = match super_templata {
                    ITemplataT::Coord(ct) => ct.coord.kind,
                    _ => panic!("Expected CoordTemplataT for superRune in DefinitionCoordIsaSR"),
                };
                let sub_kind = match ISubKindTT::try_from(sub_kind_unchecked) {
                    Ok(k) => k,
                    Err(_) => return Err(ITypingPassSolverError::BadIsaSubKind { kind: sub_kind_unchecked }),
                };
                let super_kind = match ISuperKindTT::try_from(super_kind_unchecked) {
                    Ok(k) => k,
                    Err(_) => return Err(ITypingPassSolverError::BadIsaSuperKind { kind: super_kind_unchecked }),
                };
                // Now introduce an impl so that we can later know sub implements super.
                let new_impl = self.assemble_impl(env, dcia.range, sub_kind.into(), super_kind.into());
                let mut conclusions = HashMap::new();
                conclusions.insert(dcia.result_rune.rune, ITemplataT::Isa(self.typing_interner.alloc(new_impl)));
                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let ranges = std::iter::once(dcia.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                        let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                        let error = self.typing_interner.alloc(e);
                        Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                    }
                }
            }
            //     case EqualsSR(range, leftRune, rightRune) => {
            IRulexSR::Equals(equals) => {
                match solver_state.get_conclusion(&equals.left.rune) {
                    None => {
                        let right = solver_state.get_conclusion(&equals.right.rune).expect("Neither left nor right rune solved in EqualsSR");
                        let mut conclusions = HashMap::new();
                        conclusions.insert(equals.left.rune, right.clone());
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let ranges = std::iter::once(equals.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                    Some(left) => {
                        let left = left.clone();
                        let mut conclusions = HashMap::new();
                        conclusions.insert(equals.right.rune, left);
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let ranges = std::iter::once(equals.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                }
            }
            //     case CoordSendSR(...) =>
            IRulexSR::CoordSend(coord_send) => {
                // See IRFU and SRCAMP for what's going on here.
                match solver_state.get_conclusion(&coord_send.receiver_rune.rune) {
                    None => {
                        let sender_templata = solver_state.get_conclusion(&coord_send.sender_rune.rune).expect("Neither receiverRune nor senderRune solved in CoordSendSR");
                        let coord = match sender_templata {
                            ITemplataT::Coord(ct) => ct.coord,
                            _ => panic!("Expected CoordTemplataT in CoordSendSR sender"),
                        };
                        if self.is_descendant_kind(&env, state, coord.kind) {
                            let new_rule = IRulexSR::CallSiteCoordIsa(CallSiteCoordIsaSR {
                                range: coord_send.range,
                                result_rune: None,
                                sub_rune: coord_send.sender_rune,
                                super_rune: coord_send.receiver_rune,
                            });
                            match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], HashMap::new(), vec![new_rule], std::collections::HashSet::new()) {
                                Ok(_) => Ok(()),
                                Err(e) => {
                                    let ranges = std::iter::once(coord_send.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                    let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                    let error = self.typing_interner.alloc(e);
                                    Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                                }
                            }
                        } else {
                            let mut conclusions = HashMap::new();
                            conclusions.insert(coord_send.receiver_rune.rune, ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord })));
                            match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                                Ok(_) => Ok(()),
                                Err(e) => {
                                    let ranges = std::iter::once(coord_send.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                    let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                    let error = self.typing_interner.alloc(e);
                                    Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                                }
                            }
                        }
                    }
                    Some(ITemplataT::Coord(receiver_coord_templata)) => {
                        let coord = receiver_coord_templata.coord;
                        if self.is_ancestor_kind(&env, state, coord.kind) {
                            let new_rule = IRulexSR::CallSiteCoordIsa(CallSiteCoordIsaSR {
                                range: coord_send.range,
                                result_rune: None,
                                sub_rune: coord_send.sender_rune,
                                super_rune: coord_send.receiver_rune,
                            });
                            match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], HashMap::new(), vec![new_rule], std::collections::HashSet::new()) {
                                Ok(_) => Ok(()),
                                Err(e) => {
                                    let ranges = std::iter::once(coord_send.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                    let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                    let error = self.typing_interner.alloc(e);
                                    Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                                }
                            }
                        } else {
                            let mut conclusions = HashMap::new();
                            conclusions.insert(coord_send.sender_rune.rune, ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord })));
                            match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                                Ok(_) => Ok(()),
                                Err(e) => {
                                    let ranges = std::iter::once(coord_send.range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                    let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                    let error = self.typing_interner.alloc(e);
                                    Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                                }
                            }
                        }
                    }
                    Some(_other) => { panic!("implement: solve_rule CoordSend unexpected receiver conclusion"); }
                }
            }
            //     case OneOfSR(...) =>
            IRulexSR::OneOf(r) => {
                let result = solver_state.get_conclusion(&r.rune.rune).unwrap();
                let templatas: Vec<ITemplataT<'s, 't>> = r.literals.iter().map(|l| literal_to_templata(*l)).collect();
                if templatas.contains(&result) {
                    let ranges: Vec<RangeS<'s>> = std::iter::once(r.range).chain(env.parent_ranges.iter().copied()).collect();
                    let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                    match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], std::collections::HashMap::new(), vec![], std::collections::HashSet::new()) {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            let error = self.typing_interner.alloc(e);
                            Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                        }
                    }
                } else {
                    Err(ITypingPassSolverError::OneOfFailed { rule: r })
                }
            }
            //     case IsConcreteSR(...) =>
            IRulexSR::IsConcrete(_) => { panic!("Unimplemented: solve_rule IsConcrete"); }
            //     case IsInterfaceSR(...) =>
            IRulexSR::IsInterface(_) => { panic!("Unimplemented: solve_rule IsInterface"); }
            //     case IsStructSR(...) =>
            IRulexSR::IsStruct(_) => { panic!("Unimplemented: solve_rule IsStruct"); }
            //     case CoerceToCoordSR(...) =>
            IRulexSR::CoerceToCoord(r) => {
                match solver_state.get_conclusion(&r.kind_rune.rune) {
                    None => {
                        let coord_templata = solver_state.get_conclusion(&r.coord_rune.rune).unwrap_or_else(|| panic!("implement: solve_rule CoerceToCoord no coord conclusion either"));
                        let coord = match coord_templata { ITemplataT::Coord(ct) => ct.coord, _ => panic!("implement: solve_rule CoerceToCoord coord conclusion not CoordTemplataT") };
                        match coord.ownership {
                            OwnershipT::Own | OwnershipT::Share => {
                                let mut conclusions = std::collections::HashMap::new();
                                conclusions.insert(r.kind_rune.rune, ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: coord.kind })));
                                let ranges: Vec<RangeS<'s>> = std::iter::once(r.range).chain(env.parent_ranges.iter().copied()).collect();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                                    Ok(_) => Ok(()),
                                    Err(e) => {
                                        let error = self.typing_interner.alloc(e);
                                        Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                                    }
                                }
                            }
                            _ => Err(ITypingPassSolverError::OwnershipDidntMatch { coord, expected_ownership: OwnershipT::Own }),
                        }
                    }
                    Some(kind) => {
                        let ranges: Vec<RangeS<'s>> = std::iter::once(r.range).chain(env.parent_ranges.iter().copied()).collect();
                        let coerced = self.coerce_to_coord(state, env.original_calling_env, &ranges, kind, RegionT { region: IRegionT::Default });
                        let mut conclusions = std::collections::HashMap::new();
                        conclusions.insert(r.coord_rune.rune, coerced);
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(_e) => { panic!("Unimplemented: solve_rule CoerceToCoord InternalSolverError wrapping"); }
                        }
                    }
                }
            }
            //     case LiteralSR(range, rune, literal) =>
            IRulexSR::Literal(r) => {
                let templata = literal_to_templata(r.literal);
                let mut conclusions = std::collections::HashMap::new();
                conclusions.insert(r.rune.rune, templata);
                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                    Ok(_) => Ok(()),
                    Err(_e) => { panic!("Unimplemented: solve_rule Literal InternalSolverError wrapping"); }
                }
            }
            //     case LookupSR(...) =>
            IRulexSR::Lookup(r) => {
                let ranges: Vec<RangeS<'s>> = std::iter::once(r.range).chain(env.parent_ranges.iter().copied()).collect();
                let result = match self.lookup_templata_imprecise(env, state, &ranges, r.name) {
                    None => return Err(ITypingPassSolverError::LookupFailed { name: r.name }),
                    Some(x) => x,
                };
                let mut conclusions = std::collections::HashMap::new();
                conclusions.insert(r.rune.rune, result);
                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                    Ok(_) => Ok(()),
                    Err(_e) => { panic!("Unimplemented: solve_rule Lookup InternalSolverError wrapping"); }
                }
            }
            //     case RuneParentEnvLookupSR(...) =>
            IRulexSR::RuneParentEnvLookup(r) => {
                // Canonical Scala vwats here per @MKRFA — the rule should never reach the solver
                // because callers preprocess it out via the OverloadResolver.scala:311-325 fold.
                // The Rust pipeline doesn't yet wire MKRFA preprocessing into its expression-level
                // solver call sites (array_compiler.rs, pattern_compiler.rs), so the rule still
                // reaches the solver and we no-op the step instead of panicking. Tracked alongside
                // the audit-trail vwat: real Rust parity requires landing MKRFA preprocessing first.
                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], std::collections::HashMap::new(), vec![], std::collections::HashSet::new()) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let ranges: Vec<RangeS<'s>> = std::iter::once(r.range).chain(env.parent_ranges.iter().copied()).collect();
                        let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                        let error = self.typing_interner.alloc(e);
                        Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                    }
                }
            }
            //     case AugmentSR(...) =>
            IRulexSR::Augment(augment) => {
                match solver_state.get_conclusion(&augment.result_rune.rune) {
                    Some(outer_coord_templata) => {
                        let outer_coord = match outer_coord_templata { ITemplataT::Coord(ct) => ct.coord, _ => panic!("implement: solve_rule Augment outerCoordRune not CoordTemplataT") };
                        let inner_ownership = match augment.ownership {
                            None => outer_coord.ownership,
                            Some(augment_ownership) => {
                                match self.get_mutability(state, outer_coord.kind) {
                                    ITemplataT::Placeholder(_) | ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => {
                                        if augment_ownership == OwnershipP::Share {
                                            return Err(ITypingPassSolverError::CantShareMutable { kind: outer_coord.kind });
                                        }
                                        if outer_coord.ownership != evaluate_ownership(augment_ownership) {
                                            return Err(ITypingPassSolverError::OwnershipDidntMatch { coord: outer_coord, expected_ownership: evaluate_ownership(augment_ownership) });
                                        }
                                        OwnershipT::Own
                                    }
                                    ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => outer_coord.ownership,
                                    _ => panic!("implement: solve_rule Augment Some unexpected mutability"),
                                }
                            }
                        };
                        let inner_coord = CoordT { ownership: inner_ownership, region: outer_coord.region, kind: outer_coord.kind };
                        let ranges: Vec<RangeS<'s>> = std::iter::once(augment.range).chain(env.parent_ranges.iter().copied()).collect();
                        let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                        let mut conclusions = HashMap::new();
                        conclusions.insert(augment.inner_rune.rune, ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord: inner_coord })));
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                    None => {
                        let inner_templata = solver_state.get_conclusion(&augment.inner_rune.rune).expect("Neither outerCoordRune nor innerRune solved in AugmentSR");
                        let inner_coord = match inner_templata {
                            ITemplataT::Coord(ct) => ct.coord,
                            _ => panic!("Expected CoordTemplataT in AugmentSR inner"),
                        };
                        let new_region = RegionT { region: IRegionT::Default };
                        let new_ownership = match augment.ownership {
                            None => inner_coord.ownership,
                            Some(augment_ownership) => {
                                let mutability = self.get_mutability(state, inner_coord.kind);
                                match mutability {
                                    ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => {
                                        inner_coord.ownership
                                    }
                                    ITemplataT::Placeholder(PlaceholderTemplataT { .. }) => {
                                        if augment_ownership == OwnershipP::Share {
                                            return Err(ITypingPassSolverError::CantSharePlaceholder { kind: inner_coord.kind });
                                        }
                                        evaluate_ownership(augment_ownership)
                                    }
                                    ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => {
                                        if augment_ownership == OwnershipP::Share {
                                            return Err(ITypingPassSolverError::CantShareMutable { kind: inner_coord.kind });
                                        }
                                        evaluate_ownership(augment_ownership)
                                    }
                                    _ => { panic!("implement: solve_rule Augment unexpected mutability"); }
                                }
                            }
                        };
                        let new_coord = CoordT { ownership: new_ownership, region: new_region, kind: inner_coord.kind };
                        let new_templata = ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord: new_coord }));
                        let mut conclusions = HashMap::new();
                        conclusions.insert(augment.result_rune.rune, new_templata);
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                panic!("implement: solve_rule Augment InternalSolverError wrapping");
                            }
                        }
                    }
                }
            }
            //     case PackSR(range, resultRune, memberRunes) => {
            IRulexSR::Pack(pack) => {
                match solver_state.get_conclusion(&pack.result_rune.rune) {
                    None => {
                        let members: Vec<CoordT<'s, 't>> = pack.members.iter().map(|member_rune| {
                            match solver_state.get_conclusion(&member_rune.rune).expect("Pack member rune has no conclusion") {
                                ITemplataT::Coord(ct) => ct.coord,
                                _ => panic!("implement: solve_rule Pack member non-Coord templata"),
                            }
                        }).collect();
                        let members_slice = self.typing_interner.alloc_slice_from_vec(members);
                        let coord_list = self.typing_interner.alloc(CoordListTemplataT { coords: members_slice });
                        let mut conclusions = HashMap::new();
                        conclusions.insert(pack.result_rune.rune, ITemplataT::CoordList(coord_list));
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(_e) => { panic!("implement: solve_rule Pack None InternalSolverError wrapping"); }
                        }
                    }
                    Some(ITemplataT::CoordList(coord_list_templata)) => {
                        let members = coord_list_templata.coords;
                        assert_eq!(members.len(), pack.members.len());
                        let conclusions: HashMap<IRuneS<'s>, ITemplataT<'s, 't>> = pack.members.iter().zip(members.iter()).map(|(rune, coord)| {
                            (rune.rune, ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord: *coord })))
                        }).collect();
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(_e) => { panic!("implement: solve_rule Pack Some InternalSolverError wrapping"); }
                        }
                    }
                    Some(_other) => { panic!("implement: solve_rule Pack unexpected result conclusion type"); }
                }
            }
            //     case CallSR(range, resultRune, templateRune, argRunes) => {
            //       solveCallRule(delegate, state, env, solverState, ruleIndex, range, resultRune, templateRune, argRunes)
            //     }
            IRulexSR::Call(r) => {
                self.solve_call_rule(state, &env, solver_state, rule_index, r.range, r.result_rune, r.template_rune, r.args)
            }
            //     case RefListCompoundMutabilitySR(...) =>
            IRulexSR::RefListCompoundMutability(_) => { panic!("Unimplemented: solve_rule RefListCompoundMutability"); }
            other => panic!("Unimplemented: solve_rule {:?}", other),
        }
    }
}
/*
Guardian: temp-disable: SPDMX — Canonical Scala vwats here because MKRFA preprocessing strips RuneParentEnvLookupSR before reaching the solver. Rust pipeline doesn't yet wire that preprocessing; vwat would fire on tests. Keeping the commit_step noop temporarily — chicken-egg with the MKRFA preprocessing impl backlog (array_compiler.rs / pattern_compiler.rs). — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1397-1779477403034/hook-1397/solve_rule--1285.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  private def solveRule(
    delegate: IInfererDelegate,
    state: CompilerOutputs,
    env: InferEnv,
    ruleIndex: Int,
    rule: IRulexSR,
    solverState: SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]]):
  // One might expect us to return the conclusions in this Result. Instead we take in a
  // lambda to avoid intermediate allocations, for speed.
  Result[Unit, ITypingPassSolverError] = {
    rule match {
      case KindComponentsSR(range, kindRune, mutabilityRune) => {
        val KindTemplataT(kind) = vassertSome(solverState.getConclusion(kindRune.rune))
        val mutability = delegate.getMutability(state, kind)
        solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(mutabilityRune.rune -> mutability), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
      }
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => {
        solverState.getConclusion(resultRune.rune) match {
          case None => {
            val OwnershipTemplataT(ownership) = vassertSome(solverState.getConclusion(ownershipRune.rune))
            val KindTemplataT(kind) = vassertSome(solverState.getConclusion(kindRune.rune))
            val region = RegionT(DefaultRegionT)
            val newCoord =
              delegate.getMutability(state, kind) match {
                case MutabilityTemplataT(ImmutableT) => CoordT(ShareT, region, kind)
                case MutabilityTemplataT(MutableT) | PlaceholderTemplataT(_, MutabilityTemplataType()) => {
                  CoordT(ownership, RegionT(DefaultRegionT), kind)
                }
              }
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> CoordTemplataT(newCoord)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case Some(coord) => {
            val CoordTemplataT(CoordT(ownership, region, kind)) = coord
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(ownershipRune.rune -> OwnershipTemplataT(ownership), kindRune.rune -> KindTemplataT(kind)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
        }
      }
      case PrototypeComponentsSR(range, resultRune, ownershipRune, kindRune) => {
        val PrototypeTemplataT(prototype) = vassertSome(solverState.getConclusion(resultRune.rune))
        solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(ownershipRune.rune -> CoordListTemplataT(prototype.paramTypes), kindRune.rune -> CoordTemplataT(prototype.returnType)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
      }
      case ResolveSR(range, resultRune, name, paramListRune, returnRune) => {
        // If we're here, then we're resolving a prototype.
        // This happens at the call-site.
        // The function (or struct) can either supply a default resolve rule (usually
        // via the `func moo(int)void` syntax) or let the caller pass it in.

        val CoordListTemplataT(paramCoords) = vassertSome(solverState.getConclusion(paramListRune.rune))
        solverState.getConclusion(returnRune.rune) match {
          case Some(CoordTemplataT(returnCoord)) => {
            // Existing predict path: both params and return are known. We only pretend
            // the function exists for now; actual resolution is postponed to after the
            // solve completes. See SFWPRL in docs/Generics.md:353.
            val prototypeTemplata = delegate.predictFunction(env, state, range, name, paramCoords, returnCoord)
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> prototypeTemplata), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case None => {
            // Per @BRRZ, params are known but return isn't. Do a real overload lookup
            // (the same delegate.resolveFunction the post-solve phase uses at
            // InferCompiler.scala:350) so we can discover the return type and unblock
            // the solver. Safety of this mid-solve lookup:
            //   - CompilerOutputs.lookupFunction's signatureToFunction cache is the
            //     recursion terminator for nested bound resolution.
            //   - RuneTypeSolver.scala:210 already types returnRune as CoordTemplataType,
            //     so the commitStep below is guaranteed well-typed.
            //   - Per @SROACSD, no solver call site coexists DefinitionFuncSR with
            //     ResolveSR, so there is no rule-ordering hazard.
            //   - All state read by the call chain is frozen env + settled
            //     CompilerOutputs; the outer solver's in-flight state is never consulted.
            delegate.resolveFunction(env, state, range :: env.parentRanges, name, paramCoords) match {
              case Ok(stampResult) => {
                solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex),
                  Map(
                    resultRune.rune -> PrototypeTemplataT(stampResult.prototype),
                    returnRune.rune -> CoordTemplataT(stampResult.prototype.returnType)),
                  Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
              }
              case Err(fff) => Err(CouldntFindFunction(range :: env.parentRanges, fff))
            }
          }
        }
      }
      case CallSiteFuncSR(range, prototypeRune, name, paramListRune, returnRune) => {
        // If we're here, then we're solving in the callsite, not the definition.
        // This should look up a function with that name and param list, and make sure
        // its return matches.

        vassertSome(solverState.getConclusion(prototypeRune.rune)) match {
          case PrototypeTemplataT(prototype) => {
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(paramListRune.rune -> CoordListTemplataT(prototype.paramTypes), returnRune.rune -> CoordTemplataT(prototype.returnType)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case _ => {
            Err(CantCheckPlaceholder(range :: env.parentRanges))
          }
        }
      }
      case DefinitionFuncSR(range, resultRune, name, paramListRune, returnRune) => {
        // If we're here, then we're solving in the definition, not the callsite.
        // Skip checking that they match, just assume they do.

        val CoordListTemplataT(paramCoords) = vassertSome(solverState.getConclusion(paramListRune.rune))
        val CoordTemplataT(returnType) = vassertSome(solverState.getConclusion(returnRune.rune))

        // Now introduce a prototype that lets us call it with this new name, that we
        // can call it by.
        val newPrototype =
          delegate.assemblePrototype(env, state, range, name, paramCoords, returnType)

        solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> PrototypeTemplataT(newPrototype)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
      }
      case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) => {
        val CoordTemplataT(subCoord) =
          vassertSome(solverState.getConclusion(subRune.rune))
        val CoordTemplataT(superCoord) =
          vassertSome(solverState.getConclusion(superRune.rune))

        val resultingIsaTemplata =
          if (subCoord == superCoord) {
            delegate.assembleImpl(env, range, subCoord.kind, superCoord.kind)
          } else if (subCoord.kind match { case NeverT(_) => true case _ => false }) {
            delegate.assembleImpl(env, range, subCoord.kind, superCoord.kind)
          } else {
            val subKind =
              subCoord.kind match {
                case x : ISubKindTT => x
                case other => return Err(BadIsaSubKind(other))
              }
            val superKind =
              superCoord.kind match {
                case x : ISuperKindTT => x
                case other => return Err(BadIsaSuperKind(other))
              }
            delegate.isParent(env, state, env.parentRanges, subKind, superKind, true) match {
              case None => return Err(IsaFailed(subKind, superKind))
              case Some(implTemplata) => implTemplata
            }
          }

        val conclusions = resultRune match {
          case Some(resultRune) => Map(resultRune.rune -> resultingIsaTemplata)
          case None => Map[IRuneS, ITemplataT[ITemplataType]]()
        }
        solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), conclusions, Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
      }
      case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => {
        // If we're here, then we're solving in the definition, not the callsite.
        // Skip checking that they match, just assume they do.

        val CoordTemplataT(CoordT(_, _, subKindUnchecked)) = vassertSome(solverState.getConclusion(subRune.rune))
        val CoordTemplataT(CoordT(_, _, superKindUnchecked)) = vassertSome(solverState.getConclusion(superRune.rune))

        val subKind =
          subKindUnchecked match {
            case z : ISubKindTT => z
            case _ => return Err(BadIsaSubKind(subKindUnchecked))
          }
        val superKind =
          superKindUnchecked match {
            case z : ISuperKindTT => z
            case _ => return Err(BadIsaSuperKind(superKindUnchecked))
          }

        // Now introduce an impl so that we can later know sub implements super.
        val newImpl = delegate.assembleImpl(env, range, subKind, superKind)

        solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> newImpl), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
      }
      case EqualsSR(range, leftRune, rightRune) => {
        solverState.getConclusion(leftRune.rune) match {
          case None => {
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(leftRune.rune -> vassertSome(solverState.getConclusion(rightRune.rune))), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case Some(left) => {
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(rightRune.rune -> left), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
        }
      }
      case CoordSendSR(range, senderRune, receiverRune) => {
        // See IRFU and SRCAMP for what's going on here.
        solverState.getConclusion(receiverRune.rune) match {
          case None => {
            val CoordTemplataT(coord) = vassertSome(solverState.getConclusion(senderRune.rune))
            if (delegate.isDescendant(env, state, coord.kind)) {
              // We know that the sender can be upcast, so we can't shortcut.
              // We need to wait for the receiver rune to know what to do.
              val newRule = CallSiteCoordIsaSR(range, None, senderRune, receiverRune)
              solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(), Vector(newRule)) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
            } else {
              // We're sending something that can't be upcast, so both sides are definitely the same type.
              // We can shortcut things here, even knowing only the sender's type.
              solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(receiverRune.rune -> CoordTemplataT(coord)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
            }
          }
          case Some(CoordTemplataT(coord)) => {
            if (delegate.isAncestor(env, state, coord.kind)) {
              // We know that the receiver is an interface, so we can't shortcut.
              // We need to wait for the sender rune to be able to confirm the sender
              // implements the receiver.
              val newRule = CallSiteCoordIsaSR(range, None, senderRune, receiverRune)
              solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(), Vector(newRule)) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
            } else {
              // We're receiving a concrete type, so both sides are definitely the same type.
              // We can shortcut things here, even knowing only the receiver's type.
              solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(senderRune.rune -> CoordTemplataT(coord)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
            }
          }
          case other => vwat(other)
        }
      }
      case rule @ OneOfSR(range, resultRune, literals) => {
        val result = vassertSome(solverState.getConclusion(resultRune.rune))
        val templatas = literals.map(literalToTemplata)
        if (templatas.contains(result)) {
          solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
        } else {
          Err(OneOfFailed(rule))
        }
      }
      case rule @ IsConcreteSR(range, rune) => {
        val templata = vassertSome(solverState.getConclusion(rune.rune))
        templata match {
          case KindTemplataT(kind) => {
            kind match {
              case InterfaceTT(_) => {
                Err(KindIsNotConcrete(kind))
              }
              case _ => {
                solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
              }
            }
          }
          case _ => vwat() // Should be impossible, all template rules are type checked
        }
      }
      case rule @ IsInterfaceSR(range, rune) => {
        val templata = vassertSome(solverState.getConclusion(rune.rune))
        templata match {
          case KindTemplataT(kind) => {
            kind match {
              case InterfaceTT(_) => {
                solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
              }
              case _ => Err(KindIsNotInterface(kind))
            }
          }
          case _ => vwat() // Should be impossible, all template rules are type checked
        }
      }
      case IsStructSR(range, rune) => {
        val templata = vassertSome(solverState.getConclusion(rune.rune))
        templata match {
          case KindTemplataT(kind) => {
            kind match {
              case StructTT(_) => {
                solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
              }
              case _ => Err(KindIsNotStruct(kind))
            }
          }
          case _ => vwat() // Should be impossible, all template rules are type checked
        }
      }
      case CoerceToCoordSR(range, coordRune, kindRune) => {
        solverState.getConclusion(kindRune.rune) match {
          case None => {
            val CoordTemplataT(coord) = vassertSome(solverState.getConclusion(coordRune.rune))
            coord.ownership match {
              case OwnT | ShareT => {
                solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(kindRune.rune -> KindTemplataT(coord.kind)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
              }
              case _ => {
                Err(OwnershipDidntMatch(coord, OwnT))
              }
            }
          }
          case Some(kind) => {
            val coerced = delegate.coerceToCoord(env, state, range :: env.parentRanges, kind, RegionT(DefaultRegionT))
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(coordRune.rune -> coerced), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
        }
      }
      case LiteralSR(range, rune, literal) => {
        val templata = literalToTemplata(literal)
        solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(rune.rune -> templata), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
      }
      case LookupSR(range, rune, name) => {
        val result =
          delegate.lookupTemplataImprecise(env, state, range :: env.parentRanges, name) match {
            case None => return Err(LookupFailed(name))
            case Some(x) => x
          }
        solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(rune.rune -> result), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
      }
      case RuneParentEnvLookupSR(range, rune) => {
        // This rule should never reach the solver — callers are required to preprocess
        // it out (look up the rune in callingEnv, emit an InitialKnown, strip the rule).
        // Canonical preprocessing fold: OverloadResolver.scala:311-325. See MKRFA /
        // docs/refactor-thoughts/mkrfa-protocol-leak.md for the full contract.
        vwat(rune)
      }
      case AugmentSR(range, outerCoordRune, maybeAugmentOwnership, innerRune) => {
        solverState.getConclusion(outerCoordRune.rune) match {
          case Some(CoordTemplataT(outerCoord)) => {
            val CoordT(outerOwnership, outerRegion, outerKind) = outerCoord

            val innerOwnership =
              maybeAugmentOwnership match {
                case None => outerOwnership
                case Some(augmentOwnership) => {
                  delegate.getMutability(state, outerKind) match {
                    case PlaceholderTemplataT(_, _) | MutabilityTemplataT(MutableT) => {
                      if (augmentOwnership == ShareP) {
                        return Err(CantShareMutable(outerKind))
                      }
                      if (outerOwnership != Conversions.evaluateOwnership(augmentOwnership)) {
                        return Err(OwnershipDidntMatch(
                          outerCoord,
                          Conversions.evaluateOwnership(augmentOwnership)))
                      }
                      OwnT
                    }
                    case MutabilityTemplataT(ImmutableT) => outerOwnership
                  }
                }
              }

            val innerKind = outerKind

            val innerCoord = CoordT(innerOwnership, outerRegion, innerKind)

            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(innerRune.rune -> CoordTemplataT(innerCoord)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case None => {
            val CoordTemplataT(innerCoord) =
              expectCoordTemplata(vassertSome(solverState.getConclusion(innerRune.rune)))
            val newRegion = RegionT(DefaultRegionT)
            val newOwnership =
              maybeAugmentOwnership match {
                case None => innerCoord.ownership
                case Some(augmentOwnership) => {
                  delegate.getMutability(state, innerCoord.kind) match {
                    case MutabilityTemplataT(ImmutableT) => innerCoord.ownership
                    case PlaceholderTemplataT(_, MutabilityTemplataType()) => {
                      if (augmentOwnership == ShareP) {
                        return Err(CantSharePlaceholder(innerCoord.kind))
                      }
                      Conversions.evaluateOwnership(augmentOwnership)
                    }
                    case MutabilityTemplataT(MutableT) => {
                      if (augmentOwnership == ShareP) {
                        return Err(CantShareMutable(innerCoord.kind))
                      }
                      Conversions.evaluateOwnership(augmentOwnership)
                    }
                  }
                }
              }
            val newCoord = CoordT(newOwnership, newRegion, innerCoord.kind)
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(outerCoordRune.rune -> CoordTemplataT(newCoord)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
        }
      }
      case PackSR(range, resultRune, memberRunes) => {
        solverState.getConclusion(resultRune.rune) match {
          case None => {
            val members =
              memberRunes.map(memberRune => {
                val CoordTemplataT(coord) = vassertSome(solverState.getConclusion(memberRune.rune))
                coord
              })
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> CoordListTemplataT(members.toVector)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case Some(CoordListTemplataT(members)) => {
            vassert(members.size == memberRunes.size)
            val conclusions = memberRunes.zip(members).map({ case (rune, coord) => (rune.rune -> templata.CoordTemplataT(coord)) }).toMap
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), conclusions, Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
        }
      }
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => {
//        solverState.getConclusion(resultRune.rune) match {
//          case None => {
//            val mutability = ITemplata.expectMutability(vassertSome(solverState.getConclusion(mutabilityRune.rune)))
//            val variability = ITemplata.expectVariability(vassertSome(solverState.getConclusion(variabilityRune.rune)))
//            val size = ITemplata.expectInteger(vassertSome(solverState.getConclusion(sizeRune.rune)))
//            val CoordTemplata(element) = vassertSome(solverState.getConclusion(elementRune.rune))
//            val arrKind =
//              delegate.predictStaticSizedArrayKind(env, state, mutability, variability, size, element)
//            solverState.concludeRune[ITypingPassSolverError](resultRune.rune, KindTemplata(arrKind)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//            Ok(())
//          }
//          case Some(result) => {
//            result match {
//              case KindTemplata(contentsStaticSizedArrayTT(size, mutability, variability, elementType)) => {
//                solverState.concludeRune[ITypingPassSolverError](elementRune.rune, CoordTemplata(elementType)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                solverState.concludeRune[ITypingPassSolverError](sizeRune.rune, size) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                solverState.concludeRune[ITypingPassSolverError](mutabilityRune.rune, mutability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                solverState.concludeRune[ITypingPassSolverError](variabilityRune.rune, variability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                Ok(())
//              }
//              case CoordTemplata(CoordT(OwnT | ShareT, contentsStaticSizedArrayTT(size, mutability, variability, elementType))) => {
//                solverState.concludeRune[ITypingPassSolverError](elementRune.rune, CoordTemplata(elementType)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                solverState.concludeRune[ITypingPassSolverError](sizeRune.rune, size) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                solverState.concludeRune[ITypingPassSolverError](mutabilityRune.rune, mutability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                solverState.concludeRune[ITypingPassSolverError](variabilityRune.rune, variability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                Ok(())
//              }
//              case _ => return Err(CallResultWasntExpectedType(StaticSizedArrayTemplateTemplata(), result))
//            }
//          }
//        }
//      }
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => {
//        solverState.getConclusion(resultRune.rune) match {
//          case None => {
//            val mutability = ITemplata.expectMutability(vassertSome(solverState.getConclusion(mutabilityRune.rune)))
//            val CoordTemplata(element) = vassertSome(solverState.getConclusion(elementRune.rune))
//            val arrKind =
//              delegate.predictRuntimeSizedArrayKind(env, state, element, mutability)
//            solverState.concludeRune[ITypingPassSolverError](resultRune.rune, KindTemplata(arrKind)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//            Ok(())
//          }
//          case Some(result) => {
//            result match {
//              case KindTemplata(contentsRuntimeSizedArrayTT(mutability, elementType)) => {
//                solverState.concludeRune[ITypingPassSolverError](elementRune.rune, CoordTemplata(elementType)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                solverState.concludeRune[ITypingPassSolverError](mutabilityRune.rune, mutability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                Ok(())
//              }
//              case CoordTemplata(CoordT(OwnT | ShareT, contentsRuntimeSizedArrayTT(mutability, elementType))) => {
//                solverState.concludeRune[ITypingPassSolverError](elementRune.rune, CoordTemplata(elementType)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                solverState.concludeRune[ITypingPassSolverError](mutabilityRune.rune, mutability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
//                Ok(())
//              }
//              case _ => return Err(CallResultWasntExpectedType(RuntimeSizedArrayTemplateTemplata(), result))
//            }
//          }
//        }
//      }
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => {
        val CoordListTemplataT(coords) = vassertSome(solverState.getConclusion(coordListRune.rune))
        val mutability = if (coords.forall(_.ownership == ShareT)) MutabilityTemplataT(ImmutableT) else MutabilityTemplataT(MutableT)
        solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> mutability), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
      }
      case CallSR(range, resultRune, templateRune, argRunes) => {
        solveCallRule(delegate, state, env, solverState, ruleIndex, range, resultRune, templateRune, argRunes)
      }
    }
  }

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    fn solve_call_rule(
        &self,
        state: &mut CompilerOutputs<'s, 't>,
        env: &InferEnv<'s, 't>,
        solver_state: &mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>,
        rule_index: i32,
        range: RangeS<'s>,
        result_rune: RuneUsage<'s>,
        template_rune: RuneUsage<'s>,
        arg_runes: &[RuneUsage<'s>],
    ) -> Result<(), ITypingPassSolverError<'s, 't>> {
        match solver_state.get_conclusion(&result_rune.rune) {
            Some(result) => {
                let ranges: Vec<RangeS<'s>> = std::iter::once(range).chain(env.parent_ranges.iter().copied()).collect();
                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                match result {
                    ITemplataT::Kind(kt) => {
                        match kt.kind {
                            KindT::Struct(struct_tt) => {
                                let struct_name = IStructNameT::try_from(struct_tt.id.local_name).unwrap_or_else(|_| panic!("solve_call_rule Some StructTT: local_name is not IStructNameT"));
                                let template_def = solver_state.get_conclusion(&template_rune.rune).unwrap_or_else(|| panic!("solve_call_rule Some StructTT: template_rune not solved"));
                                match template_def {
                                    ITemplataT::StructDefinition(it) => {
                                        if !self.citizen_is_from_template(ICitizenTT::Struct(struct_tt), template_def) {
                                            return Err(ITypingPassSolverError::CallResultWasntExpectedType { expected: template_def, actual: result });
                                        }
                                        let conclusions: HashMap<IRuneS<'s>, ITemplataT<'s, 't>> =
                                            struct_name.template_args().iter().zip(arg_runes.iter())
                                                .map(|(template_arg, arg_rune)| (arg_rune.rune, *template_arg))
                                                .collect();
                                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                                            Ok(_) => return Ok(()),
                                            Err(e) => {
                                                let error = self.typing_interner.alloc(e);
                                                return Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error });
                                            }
                                        }
                                    }
                                    other => return Err(ITypingPassSolverError::CallResultWasntExpectedType { expected: other, actual: result }),
                                }
                            }
                            KindT::KindPlaceholder(_) => return Err(ITypingPassSolverError::CallResultIsntCallable { result }),
                            KindT::Str(_) | KindT::Int(_) | KindT::Bool(_) | KindT::Float(_) | KindT::Void(_) => {
                                return Err(ITypingPassSolverError::CallResultIsntCallable { result });
                            }
                            KindT::Interface(interface_tt) => {
                                let interface_inner_name = match interface_tt.id.local_name {
                                    INameT::Interface(r) => r,
                                    other => panic!("solve_call_rule Some InterfaceTT: local_name is not IInterfaceNameT: {:?}", other),
                                };
                                let template_def = solver_state.get_conclusion(&template_rune.rune).unwrap_or_else(|| panic!("solve_call_rule Some InterfaceTT: template_rune not solved"));
                                match template_def {
                                    ITemplataT::InterfaceDefinition(_it) => {
                                        if !self.citizen_is_from_template(ICitizenTT::Interface(interface_tt), template_def) {
                                            return Err(ITypingPassSolverError::CallResultWasntExpectedType { expected: template_def, actual: result });
                                        }
                                    }
                                    other => return Err(ITypingPassSolverError::CallResultWasntExpectedType { expected: other, actual: result }),
                                }
                                let conclusions: HashMap<IRuneS<'s>, ITemplataT<'s, 't>> =
                                    arg_runes.iter().zip(interface_inner_name.template_args.iter())
                                        .map(|(arg_rune, template_arg)| (arg_rune.rune, *template_arg))
                                        .collect();
                                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                                    Ok(_) => return Ok(()),
                                    Err(e) => {
                                        let error = self.typing_interner.alloc(e);
                                        return Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error });
                                    }
                                }
                            }
                            KindT::RuntimeSizedArray(rsa_tt) => {
                                if arg_runes.len() != 2 {
                                    return Err(ITypingPassSolverError::WrongNumberOfTemplateArgs { expected_min_num_args: 2, expected_max_num_args: 2 });
                                }
                                let template_def = solver_state.get_conclusion(&template_rune.rune).expect("vassertSome: template_rune not solved in RuntimeSizedArray arm");
                                match template_def {
                                    ITemplataT::RuntimeSizedArrayTemplate(_) => {
                                        if !self.kind_is_from_template(state, KindT::RuntimeSizedArray(rsa_tt), template_def) {
                                            return Err(ITypingPassSolverError::CallResultWasntExpectedType { expected: template_def, actual: result });
                                        }
                                    }
                                    other => return Err(ITypingPassSolverError::CallResultWasntExpectedType { expected: other, actual: result }),
                                }
                                let mutability_rune = arg_runes[0];
                                let element_rune = arg_runes[1];
                                let mut conclusions = HashMap::new();
                                conclusions.insert(mutability_rune.rune, rsa_tt.mutability());
                                conclusions.insert(element_rune.rune, ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord: rsa_tt.element_type() })));
                                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                                    Ok(_) => return Ok(()),
                                    Err(e) => {
                                        let error = self.typing_interner.alloc(e);
                                        return Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error });
                                    }
                                }
                            }
                            KindT::StaticSizedArray(ssa_tt) => {
                                if arg_runes.len() != 4 {
                                    return Err(ITypingPassSolverError::WrongNumberOfTemplateArgs { expected_min_num_args: 4, expected_max_num_args: 4 });
                                }
                                let template_def = solver_state.get_conclusion(&template_rune.rune).expect("vassertSome: template_rune not solved in StaticSizedArray arm");
                                match template_def {
                                    ITemplataT::StaticSizedArrayTemplate(_) => {
                                        if !self.kind_is_from_template(state, KindT::StaticSizedArray(ssa_tt), template_def) {
                                            return Err(ITypingPassSolverError::CallResultWasntExpectedType { expected: template_def, actual: result });
                                        }
                                    }
                                    other => return Err(ITypingPassSolverError::CallResultWasntExpectedType { expected: other, actual: result }),
                                }
                                // We don't take in the region rune here because there's no syntactical way to specify it.
                                let size_rune = arg_runes[0];
                                let mutability_rune = arg_runes[1];
                                let variability_rune = arg_runes[2];
                                let element_rune = arg_runes[3];
                                let mut conclusions = HashMap::new();
                                conclusions.insert(size_rune.rune, ssa_tt.size());
                                conclusions.insert(mutability_rune.rune, ssa_tt.mutability());
                                conclusions.insert(variability_rune.rune, ssa_tt.variability());
                                conclusions.insert(element_rune.rune, ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord: ssa_tt.element_type() })));
                                match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                                    Ok(_) => return Ok(()),
                                    Err(e) => {
                                        let error = self.typing_interner.alloc(e);
                                        return Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error });
                                    }
                                }
                            }
                            _ => panic!("Unimplemented: solve_call_rule Some Kind {:?}", kt.kind),
                        }
                    }
                    _ => panic!("Unimplemented: solve_call_rule Some non-Kind {:?}", result),
                }
            }
            None => {
                let template = solver_state.get_conclusion(&template_rune.rune).expect("vassertSome: template_rune not solved in solve_call_rule None branch");
                match template {
                    ITemplataT::RuntimeSizedArrayTemplate(_) => {
                        let args: Vec<ITemplataT<'s, 't>> = arg_runes.iter().map(|arg_rune| {
                            solver_state.get_conclusion(&arg_rune.rune).expect("vassertSome: arg_rune not solved in solve_call_rule RuntimeSizedArrayTemplate")
                        }).collect();
                        let m = args[0];
                        let coord = match args[1] {
                            ITemplataT::Coord(ct) => ct.coord,
                            _ => panic!("Expected CoordTemplataT as second arg in solve_call_rule RuntimeSizedArrayTemplate"),
                        };
                        let context_region = RegionT { region: IRegionT::Default };
                        let mutability = crate::typing::templata::templata::expect_mutability(m);
                        let rsa_kind = self.predict_runtime_sized_array_kind(*env, state, coord, mutability, context_region);
                        let mut conclusions = HashMap::new();
                        conclusions.insert(result_rune.rune, ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::RuntimeSizedArray(self.typing_interner.intern_runtime_sized_array_tt(RuntimeSizedArrayTTValT { name: rsa_kind.name })) })));
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let ranges = std::iter::once(range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                    ITemplataT::StaticSizedArrayTemplate(_) => {
                        let args: Vec<ITemplataT<'s, 't>> = arg_runes.iter().map(|arg_rune| {
                            solver_state.get_conclusion(&arg_rune.rune).expect("vassertSome: arg_rune not solved in solve_call_rule StaticSizedArrayTemplate")
                        }).collect();
                        let s = args[0];
                        let m = args[1];
                        let v = args[2];
                        let coord = match args[3] {
                            ITemplataT::Coord(ct) => ct.coord,
                            _ => panic!("Expected CoordTemplataT as fourth arg in solve_call_rule StaticSizedArrayTemplate"),
                        };
                        let context_region = RegionT { region: IRegionT::Default };
                        let size = crate::typing::templata::templata::expect_integer(s);
                        let mutability = crate::typing::templata::templata::expect_mutability(m);
                        let variability = crate::typing::templata::templata::expect_variability(v);
                        let ssa_kind = self.predict_static_sized_array_kind(*env, state, mutability, variability, size, coord, context_region);
                        let mut conclusions = HashMap::new();
                        conclusions.insert(result_rune.rune, ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::StaticSizedArray(self.typing_interner.intern_static_sized_array_tt(StaticSizedArrayTTValT { name: ssa_kind.name })) })));
                        let ranges: Vec<RangeS<'s>> = std::iter::once(range).chain(env.parent_ranges.iter().copied()).collect();
                        let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                    ITemplataT::StructDefinition(it) => {
                        let args: Vec<ITemplataT<'s, 't>> = arg_runes.iter().map(|arg_rune| {
                            solver_state.get_conclusion(&arg_rune.rune).expect("vassertSome: arg_rune not solved in solve_call_rule")
                        }).collect();
                        let kind = self.predict_struct(state, env.original_calling_env, env.parent_ranges, env.call_location, *it, &args);
                        let mut conclusions = HashMap::new();
                        conclusions.insert(result_rune.rune, ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::Struct(self.typing_interner.intern_struct_tt(StructTTValT { id: kind.id })) })));
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let ranges = std::iter::once(range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                    ITemplataT::InterfaceDefinition(it) => {
                        let args: Vec<ITemplataT<'s, 't>> = arg_runes.iter().map(|arg_rune| {
                            solver_state.get_conclusion(&arg_rune.rune).expect("vassertSome: arg_rune not solved in solve_call_rule")
                        }).collect();
                        // See SFWPRL for why we're calling predict_interface instead of resolve_interface
                        let kind = self.predict_interface(state, env.original_calling_env, env.parent_ranges, env.call_location, *it, &args);
                        let mut conclusions = HashMap::new();
                        conclusions.insert(result_rune.rune, ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::Interface(self.typing_interner.intern_interface_tt(InterfaceTTValT { id: kind.id })) })));
                        match solver_state.commit_step::<ITypingPassSolverError<'s, 't>>(false, vec![rule_index], conclusions, vec![], std::collections::HashSet::new()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                let ranges = std::iter::once(range).chain(env.parent_ranges.iter().copied()).collect::<Vec<_>>();
                                let ranges_slice = self.typing_interner.alloc_slice_from_vec(ranges);
                                let error = self.typing_interner.alloc(e);
                                Err(ITypingPassSolverError::InternalSolverError { range: ranges_slice, err: error })
                            }
                        }
                    }
                    ITemplataT::Kind(_kt) => { panic!("Unimplemented: solve_call_rule None Kind"); }
                    other => panic!("vimpl: solve_call_rule None {:?}", other),
                }
            }
        }
    }
}
/*
  private def solveCallRule(
      delegate: IInfererDelegate,
      state: CompilerOutputs,
      env: InferEnv,
      solverState: SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]],
      ruleIndex: Int,
      range: RangeS,
      resultRune: RuneUsage,
      templateRune: RuneUsage,
      argRunes: Vector[RuneUsage]):
  Result[Unit, ITypingPassSolverError] = {
    solverState.getConclusion(resultRune.rune) match {
      case Some(result) => {
        result match {
          case KindTemplataT(rsaTT @ contentsRuntimeSizedArrayTT(mutability, memberType, region)) => {
            if (argRunes.size != 2) {
              return Err(WrongNumberOfTemplateArgs(2, 2))
            }

            vassertSome(solverState.getConclusion(templateRune.rune)) match {
              case it@RuntimeSizedArrayTemplateTemplataT() => {
                if (!delegate.kindIsFromTemplate(state, rsaTT, it)) {
                  return Err(CallResultWasntExpectedType(it, result))
                }
              }
              case other => return Err(CallResultWasntExpectedType(other, result))
            }

            val mutabilityRune = argRunes(0)
            val elementRune = argRunes(1)
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(mutabilityRune.rune -> mutability, elementRune.rune -> CoordTemplataT(memberType)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case KindTemplataT(ssaTT @ contentsStaticSizedArrayTT(size, mutability, variability, memberType, region)) => {
            if (argRunes.size != 4) {
              return Err(WrongNumberOfTemplateArgs(4, 4))
            }

            vassertSome(solverState.getConclusion(templateRune.rune)) match {
              case it@StaticSizedArrayTemplateTemplataT() => {
                if (!delegate.kindIsFromTemplate(state, ssaTT, it)) {
                  return Err(CallResultWasntExpectedType(it, result))
                }
              }
              case other => return Err(CallResultWasntExpectedType(other, result))
            }

            // We don't take in the region rune here because there's no syntactical way to specify it.
            val Vector(sizeRune, mutabilityRune, variabilityRune, elementRune) = argRunes
            // // We still have the region rune though, the rule still gives it to us.
            // solverState.concludeRune[ITypingPassSolverError](contextRegionRune.rune, region.region) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(sizeRune.rune -> size, mutabilityRune.rune -> mutability, variabilityRune.rune -> variability, elementRune.rune -> CoordTemplataT(memberType)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case KindTemplataT(interface@InterfaceTT(_)) => {
            // With all this, it seems like we could solve this rule even
            // without knowing the template. Like we could pull the template
            // from the result. Alas, we couldnt make a template templata
            // for a given lambda. If we can figure that out, this could work.
            // val templateTemplata =
            //   env.selfEnv.lookupNearestWithName(interface.id.localName.template, Set(TemplataLookupContext)) match {
            //     case Some(t@InterfaceDefinitionTemplataT(_, _)) => t
            //     case Some(_) => vwat()
            //     case None => {
            //       // This might happen if there's a lambda?
            //       return Err(CallResultIsntCallable(result))
            //     }
            //   }
            // if (templateTemplata.tyype.paramTypes != interface.id.localName.templateArgs.map(_.tyype)) {
            //   vimpl()//return Err(WrongNumberOfTemplateArgs(argRunes.length, argRunes.length)) need better error
            // }
            // solverState.concludeRune[ITypingPassSolverError]( match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
            //   range :: env.parentRanges, templateRune.rune, templateTemplata)

            vassertSome(solverState.getConclusion(templateRune.rune)) match {
              case it @ InterfaceDefinitionTemplataT(_, _) => {
                if (!delegate.kindIsFromTemplate(state, interface, it)) {
                  return Err(CallResultWasntExpectedType(it, result))
                }
              }
              case other => return Err(CallResultWasntExpectedType(other, result))
            }

            val conclusions = argRunes.zip(interface.id.localName.templateArgs).map({ case (rune, templateArg) => (rune.rune -> templateArg) }).toMap
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), conclusions, Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case KindTemplataT(struct@StructTT(_)) => {
            // With all this, it seems like we could solve this rule even
            // without knowing the template. Like we could pull the template
            // from the result. Alas, we couldnt make a template templata
            // for a given lambda. If we can figure that out, this could work.
            // val templateTemplata =
            //   env.selfEnv.lookupNearestWithName(struct.id.localName.template, Set(TemplataLookupContext)) match {
            //     case Some(t@StructDefinitionTemplataT(_, _)) => t
            //     case Some(_) => vwat()
            //     case None => {
            //       // This might happen if there's a lambda?
            //       return Err(CallResultIsntCallable(result))
            //     }
            //   }
            // if (templateTemplata.tyype.paramTypes != struct.id.localName.templateArgs.map(_.tyype)) {
            //   vimpl() // return Err(WrongNumberOfTemplateArgs(argRunes.length, argRunes.length)) need better error
            // }
            // solverState.concludeRune[ITypingPassSolverError]( match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
            //   range :: env.parentRanges, templateRune.rune, templateTemplata)

            vassertSome(solverState.getConclusion(templateRune.rune)) match {
              case it@StructDefinitionTemplataT(_, _) => {
                if (!delegate.kindIsFromTemplate(state, struct, it)) {
                  return Err(CallResultWasntExpectedType(it, result))
                }
              }
              case other => return Err(CallResultWasntExpectedType(other, result))
            }

            // The user specified this argument, so let's match the result's
            // corresponding generic arg with the user specified argument here.
            val conclusions = struct.id.localName.templateArgs.zip(argRunes).map({ case (templateArg, argRune) => (argRune.rune -> templateArg) }).toMap
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), conclusions, Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case KindTemplataT(StrT() | IntT(_) | BoolT() | FloatT() | VoidT()) => {
            return Err(CallResultIsntCallable(result))
          }
          case KindTemplataT(KindPlaceholderT(_)) => {
            // I can't imagine a placeholder that is a callable template...
            return Err(CallResultIsntCallable(result))
          }
          case other => vwat(other)
        }

        // val template = vassertSome(solverState.getConclusion(templateRune.rune))
        // template match {
        //   case RuntimeSizedArrayTemplateTemplataT() => {
        //     result match {
        //       case CoordTemplataT(CoordT(ShareT | OwnT, _, contentsRuntimeSizedArrayTT(mutability, memberType, region))) => {
        //         vimpl() // doublecheck this case
        //         if (argRunes.size < 2 || argRunes.size > 3) {
        //           return Err(WrongNumberOfTemplateArgs(2, 3))
        //         }
        //         val mutabilityRune = argRunes(0)
        //         val elementRune = argRunes(1)
        //         solverState.concludeRune[ITypingPassSolverError](mutabilityRune.rune, mutability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         solverState.concludeRune[ITypingPassSolverError](elementRune.rune, CoordTemplataT(memberType)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         if (argRunes.size >= 3) {
        //           val regionRune = argRunes(2)
        //           solverState.concludeRune[ITypingPassSolverError](regionRune.rune, region.region) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         }
        //         Ok(())
        //       }
        //       case KindTemplataT(contentsRuntimeSizedArrayTT(mutability, memberType, region)) => {
        //         vimpl() // doublecheck this case
        //         if (argRunes.size < 2 || argRunes.size > 3) {
        //           return Err(WrongNumberOfTemplateArgs(2, 3))
        //         }
        //         val mutabilityRune = argRunes(0)
        //         val elementRune = argRunes(1)
        //         solverState.concludeRune[ITypingPassSolverError](mutabilityRune.rune, mutability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         solverState.concludeRune[ITypingPassSolverError](elementRune.rune, CoordTemplataT(memberType)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         if (argRunes.size >= 3) {
        //           val regionRune = argRunes(2)
        //           solverState.concludeRune[ITypingPassSolverError](regionRune.rune, region.region) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         }
        //         Ok(())
        //       }
        //       case _ => return Err(CallResultWasntExpectedType(template, result))
        //     }
        //   }
        //   case StaticSizedArrayTemplateTemplataT() => {
        //     result match {
        //       case CoordTemplataT(CoordT(ShareT | OwnT, regionFromCoord, contentsStaticSizedArrayTT(size, mutability, variability, memberType, regionFromKind))) => {
        //         vimpl() // doublecheck this case
        //         vassert(regionFromCoord == regionFromKind)
        //         val region = regionFromKind
        //         if (argRunes.size != 4) {
        //           return Err(WrongNumberOfTemplateArgs(4, 4))
        //         }
        //         val Vector(sizeRune, mutabilityRune, variabilityRune, elementRune, regionRune) = argRunes
        //         solverState.concludeRune[ITypingPassSolverError](sizeRune.rune, size) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         solverState.concludeRune[ITypingPassSolverError](mutabilityRune.rune, mutability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         solverState.concludeRune[ITypingPassSolverError](variabilityRune.rune, variability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         solverState.concludeRune[ITypingPassSolverError](elementRune.rune, CoordTemplataT(memberType)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         solverState.concludeRune[ITypingPassSolverError](regionRune.rune, region.region) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         Ok(())
        //       }
        //       case KindTemplataT(contentsStaticSizedArrayTT(size, mutability, variability, memberType, region)) => {
        //         vimpl() // doublecheck this case
        //         if (argRunes.size != 4) {
        //           return Err(WrongNumberOfTemplateArgs(4, 4))
        //         }
        //         // We don't take in the region rune here because there's no syntactical way to specify it.
        //         val Vector(sizeRune, mutabilityRune, variabilityRune, elementRune) = argRunes
        //         solverState.concludeRune[ITypingPassSolverError](sizeRune.rune, size) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         solverState.concludeRune[ITypingPassSolverError](mutabilityRune.rune, mutability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         solverState.concludeRune[ITypingPassSolverError](variabilityRune.rune, variability) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         solverState.concludeRune[ITypingPassSolverError](elementRune.rune, CoordTemplataT(memberType)) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         // // We still have the region rune though, the rule still gives it to us.
        //         // solverState.concludeRune[ITypingPassSolverError](contextRegionRune.rune, region.region) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         Ok(())
        //       }
        //       case _ => return Err(CallResultWasntExpectedType(template, result))
        //     }
        //   }
        //   case it@InterfaceDefinitionTemplataT(_, _) => {
        //     result match {
        //       case KindTemplataT(interface@InterfaceTT(_)) => {
        //         vimpl() // doublecheck this case
        //         if (!delegate.kindIsFromTemplate(state, interface, it)) {
        //           return Err(CallResultWasntExpectedType(it, result))
        //         }
        //         vassert(argRunes.size == interface.id.localName.templateArgs.size)
        //         argRunes.zip(interface.id.localName.templateArgs).foreach({ case (rune, templateArg) =>
        //           solverState.concludeRune[ITypingPassSolverError](rune.rune, templateArg) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         })
        //         Ok(())
        //       }
        //       case CoordTemplataT(CoordT(OwnT | ShareT, _, interface@InterfaceTT(_))) => {
        //         vimpl() // doublecheck this case
        //         if (!delegate.kindIsFromTemplate(state, interface, it)) {
        //           return Err(CallResultWasntExpectedType(it, result))
        //         }
        //         vassert(argRunes.size == interface.id.localName.templateArgs.size)
        //         argRunes.zip(interface.id.localName.templateArgs).foreach({ case (rune, templateArg) =>
        //           solverState.concludeRune[ITypingPassSolverError](rune.rune, templateArg) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         })
        //         Ok(())
        //       }
        //       case _ => return Err(CallResultWasntExpectedType(template, result))
        //     }
        //   }
        //   case it@KindTemplataT(templateInterface@InterfaceTT(_)) => {
        //     result match {
        //       case KindTemplataT(instantiationInterface@InterfaceTT(_)) => {
        //         vimpl() // doublecheck this case
        //         if (templateInterface != instantiationInterface) {
        //           return Err(CallResultWasntExpectedType(it, result))
        //         }
        //         argRunes.zip(instantiationInterface.id.localName.templateArgs).foreach({ case (rune, templateArg) =>
        //           solverState.concludeRune[ITypingPassSolverError](rune.rune, templateArg) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         })
        //         Ok(())
        //       }
        //       case CoordTemplataT(CoordT(OwnT | ShareT, _, instantiationInterface@InterfaceTT(_))) => {
        //         vimpl() // doublecheck this case
        //         if (templateInterface != instantiationInterface) {
        //           return Err(CallResultWasntExpectedType(it, result))
        //         }
        //         argRunes.zip(instantiationInterface.id.localName.templateArgs).foreach({ case (rune, templateArg) =>
        //           solverState.concludeRune[ITypingPassSolverError](rune.rune, templateArg) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         })
        //         Ok(())
        //       }
        //       case _ => return Err(CallResultWasntExpectedType(template, result))
        //     }
        //   }
        //   case st@StructDefinitionTemplataT(_, _) => {
        //     result match {
        //       case KindTemplataT(struct@StructTT(_)) => {
        //         vimpl() // doublecheck this case
        //         if (!delegate.kindIsFromTemplate(state, struct, st)) {
        //           return Err(CallResultWasntExpectedType(st, result))
        //         }
        //         // If we get here, we have the resulting struct, and we're trying to feed it
        //         // backwards through a call to get the arguments it was invoked with.
        //
        //         // Let's say the user has a parameter `x Moo`. However, Moo has a default
        //         // generic arg like Moo<moo'>. The user didn't specify it.
        //         // So there will be 1 less argRunes here than the actual template args of the
        //         // incoming result. See DROIGP for more.
        //
        //         // So as we're running the result through the call backwards, we're not going to
        //         // receive that last generic arg into anything. We're instead going to make
        //         // sure it matches the default value for that template. In this case, the
        //         // default value is the context region.
        //
        //         struct.id.localName.templateArgs.zipWithIndex.foreach({ case (templateArg, index) =>
        //           if (index < argRunes.size) {
        //             // The user specified this argument, so let's match the result's
        //             // corresponding generic arg with the user specified argument here.
        //             val rune = argRunes(index)
        //             solverState.concludeRune[ITypingPassSolverError]( match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //               range :: env.parentRanges, rune.rune, templateArg)
        //           } else {
        //             vcurious() // shouldnt the highertyper prevent this
        //             // // The user didn't specify this argument, so let's match the result's
        //             // // corresponding generic arg with the default here.
        //             // val genericParam = st.originStruct.genericParameters(index)
        //             // if (genericParam.rune.rune == st.originStruct.regionRune) {
        //             //   // The default value for the default region is the context region
        //             //   // so match it against that.
        //             //   val contextRegion = expectRegion(vassertSome(solverState.getConclusion(contextRegionRune.rune)))
        //             //   if (templateArg != contextRegion) {
        //             //     return Err(CallResultWasntExpectedType(st, result))
        //             //   }
        //             // } else {
        //             //   // If we get here, there's an actual default value for it.
        //             //   vimpl()
        //             // }
        //           }
        //         })
        //
        //         Ok(())
        //       }
        //       case CoordTemplataT(CoordT(OwnT | ShareT, _, struct@StructTT(_))) => {
        //         vimpl() // doublecheck this case
        //         if (!delegate.kindIsFromTemplate(state, struct, st)) {
        //           return Err(CallResultWasntExpectedType(st, result))
        //         }
        //         vassert(argRunes.size == struct.id.localName.templateArgs.size)
        //         argRunes.zip(struct.id.localName.templateArgs).foreach({ case (rune, templateArg) =>
        //           solverState.concludeRune[ITypingPassSolverError]( match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //             range :: env.parentRanges, rune.rune, templateArg)
        //         })
        //         Ok(())
        //       }
        //       case _ => return Err(CallResultWasntExpectedType(template, result))
        //     }
        //   }
        //   case it@KindTemplataT(structTT@StructTT(_)) => {
        //     result match {
        //       case KindTemplataT(instantiationStruct@StructTT(_)) => {
        //         vimpl() // doublecheck this case
        //         if (structTT != instantiationStruct) {
        //           return Err(CallResultWasntExpectedType(it, result))
        //         }
        //         argRunes.zip(instantiationStruct.id.localName.templateArgs).foreach({ case (rune, templateArg) =>
        //           solverState.concludeRune[ITypingPassSolverError](rune.rune, templateArg) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         })
        //         Ok(())
        //       }
        //       case CoordTemplataT(CoordT(OwnT | ShareT, _, instantiationStruct@StructTT(_))) => {
        //         vimpl() // doublecheck this case
        //         if (structTT != instantiationStruct) {
        //           return Err(CallResultWasntExpectedType(it, result))
        //         }
        //         argRunes.zip(instantiationStruct.id.localName.templateArgs).foreach({ case (rune, templateArg) =>
        //           solverState.concludeRune[ITypingPassSolverError](rune.rune, templateArg) match { case Ok(_) => case Err(e) => return Err(InternalSolverError(range :: env.parentRanges, e)) }
        //         })
        //         Ok(())
        //       }
        //       case _ => return Err(CallResultWasntExpectedType(template, result))
        //     }
        //   }
        // }
      }
      case None => {
        val template = vassertSome(solverState.getConclusion(templateRune.rune))
        template match {
          case RuntimeSizedArrayTemplateTemplataT() => {
            val args = argRunes.map(argRune => vassertSome(solverState.getConclusion(argRune.rune)))
            val Vector(m, CoordTemplataT(coord)) = args
            val contextRegion = RegionT(DefaultRegionT)
            val mutability = ITemplataT.expectMutability(m)
            val rsaKind = delegate.predictRuntimeSizedArrayKind(env, state, coord, mutability, contextRegion)
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> KindTemplataT(rsaKind)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case StaticSizedArrayTemplateTemplataT() => {
            val args = argRunes.map(argRune => vassertSome(solverState.getConclusion(argRune.rune)))
            val Vector(s, m, v, CoordTemplataT(coord)) = args
            val contextRegion = RegionT(DefaultRegionT)
            val size = ITemplataT.expectInteger(s)
            val mutability = ITemplataT.expectMutability(m)
            val variability = ITemplataT.expectVariability(v)
            val rsaKind = delegate.predictStaticSizedArrayKind(env, state, mutability, variability, size, coord, contextRegion)
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> KindTemplataT(rsaKind)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case it@StructDefinitionTemplataT(_, _) => {
            val args = argRunes.map(argRune => vassertSome(solverState.getConclusion(argRune.rune)))
            // See SFWPRL for why we're calling predictStruct instead of resolveStruct
            val kind = delegate.predictStruct(env, state, it, args.toVector)
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> KindTemplataT(kind)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case it@InterfaceDefinitionTemplataT(_, _) => {
            val args = argRunes.map(argRune => vassertSome(solverState.getConclusion(argRune.rune)))
            // See SFWPRL for why we're calling predictInterface instead of resolveInterface
            val kind = delegate.predictInterface(env, state, it, args.toVector)
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> KindTemplataT(kind)), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case kt@KindTemplataT(_) => {
            solverState.commitStep[ITypingPassSolverError](false, Vector(ruleIndex), Map(resultRune.rune -> kt), Vector()) match { case Ok(_) => Ok(()) case Err(e) => Err(InternalSolverError(range :: env.parentRanges, e)) }
          }
          case other => vimpl(other)
        }
      }
    }
  }

*/
fn literal_to_templata<'s, 't>(literal: ILiteralSL<'s>) -> ITemplataT<'s, 't> {
    match literal {
        ILiteralSL::MutabilityLiteral(m) => ITemplataT::Mutability(MutabilityTemplataT { mutability: evaluate_mutability(m.mutability) }),
        ILiteralSL::OwnershipLiteral(o) => ITemplataT::Ownership(OwnershipTemplataT { ownership: evaluate_ownership(o.ownership) }),
        ILiteralSL::VariabilityLiteral(v) => ITemplataT::Variability(VariabilityTemplataT { variability: evaluate_variability(v.variability) }),
        ILiteralSL::StringLiteral(s) => ITemplataT::String(s.value),
        ILiteralSL::IntLiteral(i) => ITemplataT::Integer(i.value),
        ILiteralSL::BoolLiteral(_) => panic!("Unimplemented: literal_to_templata BoolLiteral"),
        ILiteralSL::LocationLiteral(_) => panic!("Unimplemented: literal_to_templata LocationLiteral"),
    }
}
/*
  private def literalToTemplata(literal: ILiteralSL) = {
    literal match {
      case MutabilityLiteralSL(mutability) => MutabilityTemplataT(Conversions.evaluateMutability(mutability))
      case OwnershipLiteralSL(ownership) => OwnershipTemplataT(Conversions.evaluateOwnership(ownership))
      case VariabilityLiteralSL(variability) => VariabilityTemplataT(Conversions.evaluateVariability(variability))
      case StringLiteralSL(string) => StringTemplataT(string)
      case IntLiteralSL(num) => IntegerTemplataT(num)
    }
  }
}
*/
