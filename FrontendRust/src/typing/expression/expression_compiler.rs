use crate::typing::compiler::Compiler;
use crate::postparsing::ast::{LocationInDenizen, FunctionS};
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::postparsing::expressions::*;
use crate::postparsing::patterns::patterns::AtomSP;
use crate::postparsing::rules::rules::IRulexSR;
use crate::higher_typing::ast::FunctionA;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::compiler_outputs::*;
use crate::parsing::ast::*;
use std::collections::{HashMap, HashSet};

/*
package dev.vale.typing.expression

import dev.vale
import dev.vale.highertyping.HigherTypingPass.explicifyLookups
import dev.vale.highertyping.{CompileErrorExceptionA, CouldntSolveRulesA, FunctionA, PatternSUtils}
import dev.vale._
import dev.vale.parsing.ast.{LoadAsBorrowP, LoadAsP, LoadAsWeakP, MoveP, UseP}
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.postparsing.rules.{IRulexSR, RuneParentEnvLookupSR, RuneUsage}
import dev.vale.postparsing._
import dev.vale.typing.{ArrayCompiler, CannotSubscriptT, CantMoveFromGlobal, CantMutateFinalElement, CantMutateFinalMember, CantReconcileBranchesResults, CantUnstackifyOutsideLocalFromInsideWhile, CantUseUnstackifiedLocal, CompileErrorExceptionT, Compiler, CompilerOutputs, ConvertHelper, CouldntConvertForMutateT, CouldntConvertForReturnT, CouldntFindIdentifierToLoadT, CouldntFindMemberT, HigherTypingInferError, IfConditionIsntBoolean, InferCompiler, OverloadResolver, RangedInternalErrorT, SequenceCompiler, TemplataCompiler, TypingPassOptions, ast, templata}
import dev.vale.typing.ast.{AddressExpressionTE, AddressMemberLookupTE, ArgLookupTE, BlockTE, BorrowToWeakTE, BreakTE, ConstantBoolTE, ConstantFloatTE, ConstantIntTE, ConstantStrTE, ConstructTE, DestroyTE, ExpressionT, IfTE, LetNormalTE, LocalLookupTE, LocationInFunctionEnvironmentT, MutateTE, PrototypeT, ReferenceExpressionTE, ReferenceMemberLookupTE, ReinterpretTE, ReturnTE, RuntimeSizedArrayLookupTE, StaticSizedArrayLookupTE, VoidLiteralTE, WhileTE}
import dev.vale.typing.citizen.{ImplCompiler, IsParent, IsntParent, StructCompiler}
import dev.vale.typing.env._
import dev.vale.typing.function._
import dev.vale.highertyping._
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing.RuneTypeSolver
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.{ast, _}
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.function._
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.typing.types._

import scala.collection.immutable.{List, Nil, Set}
import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

*/
pub struct TookWeakRefOfNonWeakableError;

/*
case class TookWeakRefOfNonWeakableError() extends Throwable {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }

*/
/*
trait IExpressionCompilerDelegate {
  def evaluateTemplatedFunctionFromCallForPrototype(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    functionTemplata: FunctionTemplataT,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    args: Vector[CoordT]):
  IEvaluateFunctionResult

  def evaluateGenericFunctionFromCallForPrototype(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    functionTemplata: FunctionTemplataT,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    args: Vector[CoordT]):
  IResolveFunctionResult

  def evaluateClosureStruct(
    coutputs: CompilerOutputs,
    containingNodeEnv: NodeEnvironmentT,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    name: IFunctionDeclarationNameS,
    function1: FunctionA):
  StructTT
}

*/
/*
class ExpressionCompiler(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    nameTranslator: NameTranslator,

    templataCompiler: TemplataCompiler,
    inferCompiler: InferCompiler,
    arrayCompiler: ArrayCompiler,
    structCompiler: StructCompiler,
    ancestorHelper: ImplCompiler,
    sequenceCompiler: SequenceCompiler,
    overloadCompiler: OverloadResolver,
    destructorCompiler: DestructorCompiler,
    implCompiler: ImplCompiler,
    convertHelper: ConvertHelper,
    delegate: IExpressionCompilerDelegate) {
  val localHelper = new LocalHelper(opts, interner, nameTranslator, destructorCompiler)
  val callCompiler = new CallCompiler(opts, interner, keywords, templataCompiler, convertHelper, localHelper, overloadCompiler)
  val patternCompiler = new PatternCompiler(opts, interner, keywords, inferCompiler, arrayCompiler, convertHelper, nameTranslator, destructorCompiler, localHelper)
  val blockCompiler = new BlockCompiler(opts, destructorCompiler, localHelper, new IBlockCompilerDelegate {
    override def evaluateAndCoerceToReferenceExpression(
      coutputs: CompilerOutputs,
      nenv: NodeEnvironmentBox,
      life: LocationInFunctionEnvironmentT,
      parentRanges: List[RangeS],
        callLocation: LocationInDenizen,
        region: RegionT,
      expr1: IExpressionSE):
    (ReferenceExpressionTE, Set[CoordT]) = {
      ExpressionCompiler.this.evaluateAndCoerceToReferenceExpression(
          coutputs, nenv, life, parentRanges,
          callLocation, region, expr1)
    }

    override def dropSince(
      coutputs: CompilerOutputs,
      startingNenv: NodeEnvironmentT,
      nenv: NodeEnvironmentBox,
      range: List[RangeS],
        callLocation: LocationInDenizen,
      life: LocationInFunctionEnvironmentT,
        region: RegionT,
      unresultifiedUndestructedExpressions: ReferenceExpressionTE):
    ReferenceExpressionTE = {
      ExpressionCompiler.this.dropSince(
          coutputs, startingNenv, nenv, range, callLocation, life, region, unresultifiedUndestructedExpressions)
    }
  })

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_and_coerce_to_reference_expressions(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        exprs_1: &[&'s IExpressionSE<'s>],
    ) -> (Vec<&'t ReferenceExpressionTE<'s, 't>>, HashSet<CoordT<'s, 't>>) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def evaluateAndCoerceToReferenceExpressions(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    exprs1: Vector[IExpressionSE]):
  (Vector[ReferenceExpressionTE], Set[CoordT]) = {
    val things =
      exprs1.zipWithIndex.map({ case (expr, index) =>
        evaluateAndCoerceToReferenceExpression(
          coutputs, nenv, life + index, parentRanges, callLocation, region, expr)
      })
    (things.map(_._1), things.map(_._2).flatten.toSet)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_lookup_for_load(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        name: IVarNameT<'s, 't>,
        target_ownership: LoadAsP,
    ) -> Option<ExpressionTE<'s, 't>> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  private def evaluateLookupForLoad(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    name: IVarNameT,
    targetOwnership: LoadAsP):
  Option[ExpressionT] = {
    evaluateAddressibleLookup(coutputs, nenv, range, region, name) match {
      case Some(x) => {
        val thing = localHelper.softLoad(nenv, range, x, targetOwnership, region)
        Some(thing)
      }
      case None => {
        nenv.lookupNearestWithName(name, Set(TemplataLookupContext)) match {
          case Some(IntegerTemplataT(num)) => (Some(ConstantIntTE(IntegerTemplataT(num), 32, region)))
          case Some(BooleanTemplataT(bool)) => (Some(ConstantBoolTE(bool, region)))
          case None => (None)
        }
      }
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_addressible_lookup_for_mutate(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        region: RegionT,
        load_range: RangeS<'s>,
        name_a: IVarNameS<'s>,
    ) -> Option<&'t AddressExpressionTE<'s, 't>> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  private def evaluateAddressibleLookupForMutate(
      coutputs: CompilerOutputs,
      nenv: NodeEnvironmentBox,
      parentRanges: List[RangeS],
    region: RegionT,
      loadRange: RangeS,
      nameA: IVarNameS):
  Option[AddressExpressionTE] = {
    nenv.getVariable(nameTranslator.translateVarNameStep(nameA)) match {
      case Some(alv @ AddressibleLocalVariableT(_, _, reference)) => {
        Some(LocalLookupTE(loadRange, alv))
      }
      case Some(rlv @ ReferenceLocalVariableT(id, _, reference)) => {
        Some(LocalLookupTE(loadRange, rlv))
      }
      case Some(AddressibleClosureVariableT(id, closuredVarsStructRef, variability, tyype)) => {
        val closuredVarsStructId = closuredVarsStructRef.id
        val closuredVarsStructTemplateId =
          TemplataCompiler.getStructTemplate(closuredVarsStructId)
        val closuredVarsStructTemplateName =
          closuredVarsStructTemplateId.localName match {
            case n @ LambdaCitizenTemplateNameT(_) => n
            case _ => vwat()
          }

        val mutability = Compiler.getMutability(coutputs, closuredVarsStructRef)
        val ownership =
          mutability match {
            case MutabilityTemplataT(MutableT) => BorrowT
            case MutabilityTemplataT(ImmutableT) => ShareT
            case PlaceholderTemplataT(idT, MutabilityTemplataType()) => vimpl()
          }
        val closuredVarsStructRefRef = CoordT(ownership, RegionT(), closuredVarsStructRef)
        val name2 = interner.intern(ClosureParamNameT(closuredVarsStructTemplateName.codeLocation))
        val borrowExpr =
          localHelper.borrowSoftLoad(
            coutputs,
            LocalLookupTE(
              loadRange,
              ReferenceLocalVariableT(name2, FinalT, closuredVarsStructRefRef)))

        val closuredVarsStructDef = coutputs.lookupStruct(closuredVarsStructRef.id)
        vassert(closuredVarsStructDef.members.exists(member => member.name == id))

        val index = closuredVarsStructDef.members.indexWhere(_.name == id)
//        val ownershipInClosureStruct = closuredVarsStructDef.members(index).tyype.reference.ownership
        val lookup = ast.AddressMemberLookupTE(loadRange, borrowExpr, id, tyype, variability)
        Some(lookup)
      }
      case Some(ReferenceClosureVariableT(varName, closuredVarsStructRef, variability, tyype)) => {
        val closuredVarsStructId = closuredVarsStructRef.id
        val closuredVarsStructTemplateId =
          TemplataCompiler.getStructTemplate(closuredVarsStructId)
        val closuredVarsStructTemplateName =
          closuredVarsStructTemplateId.localName match {
            case n @ LambdaCitizenTemplateNameT(_) => n
            case _ => vwat()
          }

        val mutability = Compiler.getMutability(coutputs, closuredVarsStructRef)
        val ownership =
          mutability match {
            case MutabilityTemplataT(MutableT) => BorrowT
            case MutabilityTemplataT(ImmutableT) => ShareT
            case PlaceholderTemplataT(idT, MutabilityTemplataType()) => vimpl()
          }
        val closuredVarsStructRefCoord = CoordT(ownership, RegionT(), closuredVarsStructRef)
        val borrowExpr =
          localHelper.borrowSoftLoad(
            coutputs,
            LocalLookupTE(
              loadRange,
              ReferenceLocalVariableT(interner.intern(ClosureParamNameT(closuredVarsStructTemplateName.codeLocation)), FinalT, closuredVarsStructRefCoord)))

        val lookup =
          ast.ReferenceMemberLookupTE(loadRange, borrowExpr, varName, tyype, variability)
        Some(lookup)
      }
      case None => None
      case _ => vwat()
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_addressible_lookup(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        ranges: &[RangeS<'s>],
        region: RegionT,
        name_2: IVarNameT<'s, 't>,
    ) -> Option<&'t AddressExpressionTE<'s, 't>> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  private def evaluateAddressibleLookup(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    ranges: List[RangeS],
    region: RegionT,
    name2: IVarNameT):
  Option[AddressExpressionTE] = {
    nenv.getVariable(name2) match {
      case Some(alv @ AddressibleLocalVariableT(varId, variability, reference)) => {
        vassert(!nenv.unstackifieds.contains(varId))
        Some(LocalLookupTE(ranges.head, alv))
      }
      case Some(rlv @ ReferenceLocalVariableT(varId, variability, reference)) => {
        if (nenv.unstackifieds.contains(varId)) {
          throw CompileErrorExceptionT(CantUseUnstackifiedLocal(ranges, varId))
        }
        Some(LocalLookupTE(ranges.head, rlv))
      }
      case Some(AddressibleClosureVariableT(id, closuredVarsStructRef, variability, tyype)) => {
        val closuredVarsStructId = closuredVarsStructRef.id
        val closuredVarsStructTemplateId =
          TemplataCompiler.getStructTemplate(closuredVarsStructId)
        val closuredVarsStructTemplateName =
          closuredVarsStructTemplateId.localName match {
            case n @ LambdaCitizenTemplateNameT(_) => n
            case _ => vwat()
          }

        val mutability = Compiler.getMutability(coutputs, closuredVarsStructRef)
        val ownership =
          mutability match {
            case MutabilityTemplataT(MutableT) => BorrowT
            case MutabilityTemplataT(ImmutableT) => ShareT
            case PlaceholderTemplataT(idT, MutabilityTemplataType()) => vimpl()
          }
        val closuredVarsStructRefRef = CoordT(ownership, RegionT(), closuredVarsStructRef)
        val closureParamVarName2 = interner.intern(ClosureParamNameT(closuredVarsStructTemplateName.codeLocation))

        val borrowExpr =
          localHelper.borrowSoftLoad(
            coutputs,
            LocalLookupTE(
              ranges.head,
              ReferenceLocalVariableT(closureParamVarName2, FinalT, closuredVarsStructRefRef)))
        val closuredVarsStructDef = coutputs.lookupStruct(closuredVarsStructRef.id)

//        vassert(closuredVarsStructRef.fullName.steps == id.steps.init)

        vassert(closuredVarsStructDef.members.map(_.name).contains(id))
        val lookup = AddressMemberLookupTE(ranges.head, borrowExpr, id, tyype, variability)
        Some(lookup)
      }
      case Some(ReferenceClosureVariableT(varName, closuredVarsStructRef, variability, tyype)) => {
        val closuredVarsStructId = closuredVarsStructRef.id
        val closuredVarsStructTemplateId =
          TemplataCompiler.getStructTemplate(closuredVarsStructId)
        val closuredVarsStructTemplateName =
          closuredVarsStructTemplateId.localName match {
            case n @ LambdaCitizenTemplateNameT(_) => n
            case _ => vwat()
          }
        val mutability = Compiler.getMutability(coutputs, closuredVarsStructRef)
        val ownership =
          mutability match {
            case MutabilityTemplataT(MutableT) => BorrowT
            case MutabilityTemplataT(ImmutableT) => ShareT
            case PlaceholderTemplataT(idT, MutabilityTemplataType()) => vimpl()
          }
        val closuredVarsStructRefCoord = CoordT(ownership, RegionT(), closuredVarsStructRef)
        val closuredVarsStructDef = coutputs.lookupStruct(closuredVarsStructRef.id)

        vassert(closuredVarsStructDef.members.map(_.name).contains(varName))

        val borrowExpr =
          localHelper.borrowSoftLoad(
            coutputs,
            LocalLookupTE(
              ranges.head,
              ReferenceLocalVariableT(interner.intern(ClosureParamNameT(closuredVarsStructTemplateName.codeLocation)), FinalT, closuredVarsStructRefCoord)))

        val lookup = ReferenceMemberLookupTE(ranges.head, borrowExpr, varName, tyype, variability)
        Some(lookup)
      }
      case None => None
      case _ => vwat()
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_closure_struct_construct_expression(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        range: &[RangeS<'s>],
        region: RegionT,
        closure_struct_ref: StructTT<'s, 't>,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  private def makeClosureStructConstructExpression(
      coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
      range: List[RangeS],
    region: RegionT,
      closureStructRef: StructTT):
  (ReferenceExpressionTE) = {
    val closureStructDef = coutputs.lookupStruct(closureStructRef.id);
    val substituter =
      TemplataCompiler.getPlaceholderSubstituter(
        opts.globalOptions.sanityCheck,
        interner, keywords,
        nenv.functionEnvironment.templateId,
        closureStructRef.id,
        InheritBoundsFromTypeItself)
    // Note, this is where the unordered closuredNames set becomes ordered.
    val lookupExpressions2 =
      closureStructDef.members.map({
        case VariadicStructMemberT(name, tyype) => {
          vwat() // closures cant contain variadic members
        }
        case NormalStructMemberT(memberName, variability, tyype) => {
          val lookup =
            evaluateAddressibleLookup(coutputs, nenv, range, region, memberName) match {
              case None => {
                throw CompileErrorExceptionT(RangedInternalErrorT(range, "Couldn't find " + memberName))
              }
              case Some(l) => l
            }
          tyype match {
            case ReferenceMemberTypeT(unsubstitutedCoord) => {
              val coord = substituter.substituteForCoord(coutputs, unsubstitutedCoord)
              // We might have to softload an own into a borrow, but the kinds
              // should at least be the same right here.
              vassert(coord.kind == lookup.result.coord.kind)
              // Closures never contain owning references.
              // If we're capturing an own, then on the inside of the closure
              // it's a borrow or a weak. See "Captured own is borrow" test for more.

              vassert(coord.ownership != OwnT)
              localHelper.borrowSoftLoad(coutputs, lookup)
            }
            case AddressMemberTypeT(unsubstitutedCoord) => {
              val coord = substituter.substituteForCoord(coutputs, unsubstitutedCoord)
              vassert(coord == lookup.result.coord)
              (lookup)
            }
            case _ => vwat()
          }
        }
      });
    val ownership =
      closureStructDef.mutability match {
        case MutabilityTemplataT(MutableT) => OwnT
        case MutabilityTemplataT(ImmutableT) => ShareT
        case PlaceholderTemplataT(idT, MutabilityTemplataType()) => vimpl()
      }
    val resultPointerType = CoordT(ownership, region, closureStructRef)

    val constructExpr2 =
      ConstructTE(closureStructRef, resultPointerType, lookupExpressions2)
    (constructExpr2)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_and_coerce_to_reference_expression(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        expr_1: &'s IExpressionSE<'s>,
    ) -> (&'t ReferenceExpressionTE<'s, 't>, HashSet<CoordT<'s, 't>>) {
        let (expr2, returns_from_expr) =
            self.evaluate_expression(coutputs, nenv, life, parent_ranges, call_location, region, expr_1);
        match expr2 {
            ExpressionTE::Reference(r) => (r, returns_from_expr),
            ExpressionTE::Address(_a) => {
                panic!("implement: evaluateAndCoerceToReferenceExpression — AddressExpressionTE");
            }
        }
    }
/*
  def evaluateAndCoerceToReferenceExpression(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    expr1: IExpressionSE):
  (ReferenceExpressionTE, Set[CoordT]) = {
    val (expr2, returnsFromExpr) =
      evaluate(coutputs, nenv, life, parentRanges, callLocation, region, expr1)
    expr2 match {
      case r : ReferenceExpressionTE => {
        (r, returnsFromExpr)
      }
      case a : AddressExpressionTE => {
        val expr = coerceToReferenceExpression(nenv, parentRanges, a, region)
        (expr, returnsFromExpr)
      }
      case _ => vwat()
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn coerce_to_reference_expression(
        &self,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        expr_2: ExpressionTE<'s, 't>,
        region: RegionT,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def coerceToReferenceExpression(
    nenv: NodeEnvironmentBox,
    parentRanges: List[RangeS],
    expr2: ExpressionT,
    region: RegionT
  ):
  (ReferenceExpressionTE) = {
    expr2 match {
      case r : ReferenceExpressionTE => (r)
      case a: AddressExpressionTE => {
        localHelper.softLoad(nenv, a.range :: parentRanges, a, UseP, region)
      }
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_expected_address_expression(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        expr_1: &'s IExpressionSE<'s>,
    ) -> (&'t AddressExpressionTE<'s, 't>, HashSet<CoordT<'s, 't>>) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  private def evaluateExpectedAddressExpression(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    expr1: IExpressionSE):
  (AddressExpressionTE, Set[CoordT]) = {
    val (expr2, returns) =
      evaluate(coutputs, nenv, life, parentRanges, callLocation, region, expr1)
    expr2 match {
      case a : AddressExpressionTE => (a, returns)
      case _: ReferenceExpressionTE => {
        throw CompileErrorExceptionT(
          RangedInternalErrorT(expr1.range :: parentRanges, "Expected reference expression!"))
      }
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_expression(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        outer_call_location: LocationInDenizen<'s>,
        region: RegionT,
        expr_1: &'s IExpressionSE<'s>,
    ) -> (ExpressionTE<'s, 't>, HashSet<CoordT<'s, 't>>) {
        match expr_1 {
            IExpressionSE::Void(_) => {
                (ExpressionTE::Reference(self.typing_interner.alloc(
                    ReferenceExpressionTE::VoidLiteral(VoidLiteralTE {
                        region,
                        _phantom: std::marker::PhantomData,
                    }))), HashSet::new())
            }
            IExpressionSE::ConstantInt(c) => {
                (ExpressionTE::Reference(self.typing_interner.alloc(
                    ReferenceExpressionTE::ConstantInt(ConstantIntTE {
                        value: ITemplataT::Integer(c.value),
                        bits: c.bits,
                        region,
                    }))), HashSet::new())
            }
            IExpressionSE::Return(ret) => {
                let (uncasted_inner_expr_2, returns_from_inner_expr) =
                    self.evaluate_and_coerce_to_reference_expression(
                        coutputs, nenv, life.add(0), parent_ranges,
                        outer_call_location, region, ret.inner);

                let inner_expr_2 = match nenv.maybe_return_type() {
                    None => uncasted_inner_expr_2,
                    Some(return_type) => {
                        let snapshot = nenv.snapshot(self.typing_interner);
                        let snapshot_env = &*self.typing_interner.alloc(IInDenizenEnvironmentT::Node(snapshot));
                        let range_list: Vec<RangeS<'s>> =
                            std::iter::once(ret.range).chain(parent_ranges.iter().copied()).collect();
                        match self.is_type_convertible(
                            coutputs, &snapshot_env, &range_list, outer_call_location,
                            uncasted_inner_expr_2.result().coord, return_type) {
                            false => {
                                panic!("implement: evaluate_expression ReturnSE — CouldntConvertForReturnT");
                            }
                            true => {
                                self.convert(
                                    snapshot_env, coutputs, &range_list, outer_call_location,
                                    uncasted_inner_expr_2, return_type)
                            }
                        }
                    }
                };

                let all_locals = nenv.get_all_locals();
                let unstackified_locals = nenv.get_all_unstackified_locals();
                let variables_to_destruct: Vec<&ILocalVariableT<'s, 't>> = all_locals.iter()
                    .filter(|x| !unstackified_locals.contains(&x.name()))
                    .collect();
                let reversed_variables_to_destruct: Vec<&ILocalVariableT<'s, 't>> =
                    variables_to_destruct.into_iter().rev().collect();

                let mut returns = returns_from_inner_expr;
                returns.insert(inner_expr_2.result().coord);

                let result_var_name = self.typing_interner.intern_typing_pass_function_result_var_name(
                    TypingPassFunctionResultVarNameT { _phantom: std::marker::PhantomData });
                let result_var_id = IVarNameT::TypingPassFunctionResultVar(result_var_name);
                let result_variable = ReferenceLocalVariableT {
                    name: result_var_id,
                    variability: VariabilityT::Final,
                    coord: inner_expr_2.result().coord,
                };
                let result_let = self.typing_interner.alloc(
                    ReferenceExpressionTE::LetNormal(LetNormalTE {
                        variable: ILocalVariableT::Reference(result_variable),
                        expr: inner_expr_2,
                    }));
                nenv.add_variable(IVariableT::ReferenceLocal(result_variable));

                let range_list: Vec<RangeS<'s>> =
                    std::iter::once(ret.range).chain(parent_ranges.iter().copied()).collect();
                let destruct_exprs_refs =
                    self.unlet_and_drop_all(
                        coutputs, nenv, &range_list, outer_call_location, region,
                        &reversed_variables_to_destruct);

                let get_result_expr = self.unlet_local_without_dropping(
                    nenv, &ILocalVariableT::Reference(result_variable));
                let get_result_expr_ref = self.typing_interner.alloc(
                    ReferenceExpressionTE::Unlet(get_result_expr));

                let mut all_exprs: Vec<&'t ReferenceExpressionTE<'s, 't>> = Vec::new();
                all_exprs.push(result_let);
                all_exprs.extend(destruct_exprs_refs);
                all_exprs.push(get_result_expr_ref);

                let consecutor = self.consecutive(&all_exprs);

                let return_te = self.typing_interner.alloc(
                    ReferenceExpressionTE::Return(ReturnTE {
                        source_expr: consecutor,
                    }));

                (ExpressionTE::Reference(return_te), returns)
            }
            IExpressionSE::Let(let_se) => {
                let (source_expr_2, returns_from_source) =
                    self.evaluate_and_coerce_to_reference_expression(
                        coutputs, nenv, life.add(0), parent_ranges, outer_call_location, nenv.default_region(), let_se.expr);

                let rune_type_solve_env = LetExprRuneTypeSolverEnv { nenv };
                let rune_to_initially_known_type: HashMap<_, _> =
                    crate::higher_typing::patterns::get_rune_types_from_pattern(&let_se.pattern)
                        .into_iter().collect();
                let range_list: Vec<RangeS<'s>> =
                    std::iter::once(let_se.range).chain(parent_ranges.iter().copied()).collect();
                let rune_to_type =
                    crate::postparsing::rune_type_solver::solve_rune_type(
                        self.scout_arena,
                        self.opts.global_options.sanity_check,
                        &rune_type_solve_env,
                        range_list,
                        false,
                        let_se.rules,
                        &[],
                        true,
                        rune_to_initially_known_type,
                    ).unwrap_or_else(|_e| {
                        panic!("implement: LetSE — HigherTypingInferError");
                    });

                let rules_vec: Vec<&'s IRulexSR<'s>> = let_se.rules.iter().collect();
                let result_te = self.infer_and_translate_pattern(
                    coutputs,
                    nenv,
                    life.add(1),
                    parent_ranges,
                    outer_call_location,
                    &rules_vec,
                    &rune_to_type,
                    &let_se.pattern,
                    source_expr_2,
                    region,
                    |_coutputs, nenv, _life, _live_capture_locals| {
                        self.typing_interner.alloc(
                            ReferenceExpressionTE::VoidLiteral(VoidLiteralTE {
                                region: nenv.default_region(),
                                _phantom: std::marker::PhantomData,
                            }))
                    },
                );

                (ExpressionTE::Reference(result_te), returns_from_source)
            }
            IExpressionSE::Consecutor(consecutor_se) => {
                assert!(region == nenv.default_region());
                let region_for_inners = region;

                let mut init_exprs_te: Vec<&'t ReferenceExpressionTE<'s, 't>> = Vec::new();
                let mut init_returns: HashSet<CoordT<'s, 't>> = HashSet::new();
                for (index, expr_se) in consecutor_se.exprs.iter().enumerate().take(consecutor_se.exprs.len() - 1) {
                    let (undropped_expr_te, returns) =
                        self.evaluate_and_coerce_to_reference_expression(
                            coutputs, nenv, life.add(index as i32), parent_ranges, outer_call_location, region_for_inners, expr_se);
                    let expr_te = match undropped_expr_te.result().coord.kind {
                        KindT::Void(_) => undropped_expr_te,
                        _ => {
                            panic!("implement: ConsecutorSE — drop non-void init expr");
                        }
                    };
                    init_exprs_te.push(expr_te);
                    init_returns.extend(returns);
                }

                let (last_expr_te, last_returns) =
                    self.evaluate_and_coerce_to_reference_expression(
                        coutputs, nenv,
                        life.add((consecutor_se.exprs.len() - 1) as i32),
                        parent_ranges,
                        outer_call_location,
                        region_for_inners,
                        consecutor_se.exprs.last().unwrap());

                init_exprs_te.push(last_expr_te);
                init_returns.extend(last_returns);

                let result = self.consecutive(&init_exprs_te);
                (ExpressionTE::Reference(result), init_returns)
            }
            _ => {
                panic!("implement: evaluate_expression — {:?}", std::mem::discriminant(expr_1));
            }
        }
    }
/*
  // returns:
  // - resulting expression
  // - all the types that are returned from inside the body via return
  private def evaluate(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    // Called outer because there are other calls below that come with their own call locations.
    // We should probably figure out how to make life, parentRanges, and callLocations into one unified thing.
    outerCallLocation: LocationInDenizen,
    region: RegionT,
    expr1: IExpressionSE):
  (ExpressionT, Set[CoordT]) = {
    Profiler.frame(() => {
      expr1 match {
        case VoidSE(range) => (VoidLiteralTE(region), Set())
        case ConstantIntSE(range, i, bits) => {
          (ConstantIntTE(
            IntegerTemplataT(i),
            bits,
            region), Set())
        }
        case ConstantBoolSE(range, i) => (ConstantBoolTE(i, region), Set())
        case ConstantStrSE(range, s) => (ConstantStrTE(s, region), Set())
        case ConstantFloatSE(range, f) => (ConstantFloatTE(f, region), Set())
        case ArgLookupSE(range, index) => {
          val paramCoordRune = nenv.function.params(index).pattern.coordRune.get
          val paramCoordTemplata = vassertOne(nenv.lookupNearestWithImpreciseName(interner.intern(RuneNameS(paramCoordRune.rune)), Set(TemplataLookupContext)))
          val CoordTemplataT(paramCoord) = paramCoordTemplata
          vassert(nenv.functionEnvironment.id.localName.parameters(index) == paramCoord)
          (ArgLookupTE(index, paramCoord), Set())
        }
        case FunctionCallSE(range, callLocation, OutsideLoadSE(_, rules, name, maybeTemplateArgs, callableTargetOwnership), argsExprs1) => {
//          vassert(callableTargetOwnership == PointConstraintP(Some(ReadonlyP)))
          val (argsExprs2, returnsFromArgs) =
            evaluateAndCoerceToReferenceExpressions(
              coutputs, nenv, life + 0, parentRanges, callLocation,
              // See SRIE
              nenv.defaultRegion,
              argsExprs1)
          val callExpr2 =
            callCompiler.evaluatePrefixCall(
              coutputs,
              nenv,
              life + 1,
              range :: parentRanges,
              callLocation,
              region,
              newGlobalFunctionGroupExpression(
                nenv.snapshot,
                coutputs,
                // i suppose this can instead take on the region of whatever's expected?
                nenv.defaultRegion,
                name),
              rules.toVector,
              maybeTemplateArgs.toVector.flatMap(_.map(_.rune)),
              argsExprs2)
          (callExpr2, returnsFromArgs)
        }
        case FunctionCallSE(range, callLocation, OutsideLoadSE(_, rules, name, templateArgTemplexesS, callableTargetOwnership), argsExprs1) => {
//          vassert(callableTargetOwnership == PointConstraintP(None))
          val (argsExprs2, returnsFromArgs) =
            evaluateAndCoerceToReferenceExpressions(coutputs, nenv, life + 0, parentRanges, callLocation, region, argsExprs1)
          val callExpr2 =
            callCompiler.evaluatePrefixCall(
              coutputs,
              nenv,
              life + 1,
              range :: parentRanges,
              callLocation,
              region,
              newGlobalFunctionGroupExpression(nenv.snapshot, coutputs, RegionT(), name),
              rules.toVector,
              templateArgTemplexesS.toVector.flatMap(_.map(_.rune)),
              argsExprs2)
          (callExpr2, returnsFromArgs)
        }
        case FunctionCallSE(range, callLocation, callableExpr1, argsExprs1) => {
          val (undecayedCallableExpr2, returnsFromCallable) =
            evaluateAndCoerceToReferenceExpression(
              coutputs, nenv, life + 0, parentRanges, callLocation, region, callableExpr1);
          val decayedCallableExpr2 =
            localHelper.maybeBorrowSoftLoad(coutputs, undecayedCallableExpr2)
          val decayedCallableReferenceExpr2 =
            coerceToReferenceExpression(nenv, parentRanges, decayedCallableExpr2, region)
          val (argsExprs2, returnsFromArgs) =
            evaluateAndCoerceToReferenceExpressions(
              coutputs, nenv, life + 1, parentRanges, callLocation, nenv.defaultRegion, argsExprs1)
          val functionPointerCall2 =
            callCompiler.evaluatePrefixCall(
              coutputs,
              nenv,
              life + 2,
              range :: parentRanges,
              callLocation,
              region,
              decayedCallableReferenceExpr2,
              Vector(),
              Vector(),
              argsExprs2)
          (functionPointerCall2, returnsFromCallable ++ returnsFromArgs)
        }

        case OwnershippedSE(range, sourceSE, loadAsP) => {
          val (sourceTE, returnsFromInner) =
            evaluateAndCoerceToReferenceExpression(
              coutputs, nenv, life + 0, parentRanges, outerCallLocation, region, sourceSE);
          val resultExpr2 =
            sourceTE.result.underlyingCoord.ownership match {
              case OwnT => {
                loadAsP match {
                  case MoveP => {
                    // this can happen if we put a ^ on an owning reference. No harm, let it go.
                    sourceTE
                  }
                  case LoadAsBorrowP => {
                    localHelper.makeTemporaryLocal(coutputs, nenv, range :: parentRanges, outerCallLocation, life + 1, region, sourceTE, BorrowT)
                  }
                  case LoadAsWeakP => {
                    val expr = localHelper.makeTemporaryLocal(coutputs, nenv, range :: parentRanges, outerCallLocation, life + 3, region, sourceTE, BorrowT)
                    weakAlias(coutputs, expr)
                  }
                  case UseP => vcurious()
                }
              }
              case BorrowT => {
                loadAsP match {
                  case MoveP => vcurious() // Can we even coerce to an owning reference?
                  case LoadAsBorrowP => sourceTE
                  case LoadAsWeakP => weakAlias(coutputs, sourceTE)
                  case UseP => sourceTE
                }
              }
              case WeakT => {
                loadAsP match {
                  case MoveP => vcurious() // Can we even coerce to an owning reference?
                  case LoadAsBorrowP => vimpl()
                  case LoadAsWeakP => sourceTE
                  case UseP => sourceTE
                }
              }
              case ShareT => {
                loadAsP match {
                  case MoveP => {
                    // Allow this, we can do ^ on a share ref, itll just give us a share ref.
                    sourceTE
                  }
                  case LoadAsBorrowP => {
                    // Allow this, we can do & on a share ref, itll just give us a share ref.
                    sourceTE
                  }
                  case LoadAsWeakP => {
                    vfail()
                  }
                  case UseP => sourceTE
                }
              }
            }
          (resultExpr2, returnsFromInner)
        }
        case LocalLoadSE(range, nameA, targetOwnership) => {
          val name = nameTranslator.translateVarNameStep(nameA)
          val lookupExpr1 =
            evaluateLookupForLoad(coutputs, nenv, range :: parentRanges, outerCallLocation, region, name, targetOwnership) match {
              case (None) => {
                throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Couldnt find " + name))
              }
              case (Some(x)) => (x)
            }
          (lookupExpr1, Set())
        }
        case OutsideLoadSE(range, rules, name, templateArgs, targetOwnership) => {
          // Note, we don't get here if we're about to call something with this, that's handled
          // by a different case.

          // We can't use *anything* from the global environment; we're in expression context,
          // not in templata context.

          val templataFromEnv =
            nenv.lookupAllWithImpreciseName(name, Set(ExpressionLookupContext)) match {
              case Array(BooleanTemplataT(value)) => ConstantBoolTE(value, region)
              case Array(IntegerTemplataT(value)) => {
                ConstantIntTE(
                  IntegerTemplataT(value),
                  32,
                  region)
              }
              case Array(t @ PlaceholderTemplataT(name, IntegerTemplataType())) => {
                ConstantIntTE(PlaceholderTemplataT(name, IntegerTemplataType()), 32, region)
              }
              case templatas if templatas.nonEmpty && templatas.collect({ case FunctionTemplataT(_, _) => }).size == templatas.size => {
                if (targetOwnership == MoveP) {
                  throw CompileErrorExceptionT(CantMoveFromGlobal(range :: parentRanges, "Can't move from globals. Name: " + name))
                }
                newGlobalFunctionGroupExpression(nenv.snapshot, coutputs, region, name)
              }
              case things if things.size > 1 => {
                throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Found too many different things named \"" + name + "\" in env:\n" + things.map("\n" + _)))
              }
              case Array() => {
                throw CompileErrorExceptionT(CouldntFindIdentifierToLoadT(range :: parentRanges, name))
                throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Couldn't find anything named \"" + name + "\" in env:\n" + nenv))
              }
            }
          (templataFromEnv, Set())
        }
        case LocalMutateSE(range, name, sourceExpr1) => {
          val (unconvertedSourceExpr2, returnsFromSource) =
            evaluateAndCoerceToReferenceExpression(
              coutputs, nenv, life, parentRanges, outerCallLocation, region, sourceExpr1)

          // We do this after the source because of statements like these:
          //   set ship = foo(ship);
          // which move the thing on the right and then restackify it on the left.
          val destinationExpr2 =
            evaluateAddressibleLookupForMutate(coutputs, nenv, parentRanges, region, range, name) match {
              case None => {
                throw CompileErrorExceptionT(
                  RangedInternalErrorT(range :: parentRanges, "Couldnt find " + name))
              }
              case Some(x) => x
            }

          // We should have inferred variability from the presents of sets
          vassert(destinationExpr2.variability == VaryingT)

          val isConvertible =
            templataCompiler.isTypeConvertible(
              coutputs, nenv.snapshot, range :: parentRanges, outerCallLocation, unconvertedSourceExpr2.result.coord, destinationExpr2.result.coord)
          if (!isConvertible) {
            throw CompileErrorExceptionT(
              CouldntConvertForMutateT(
                range :: parentRanges, destinationExpr2.result.coord, unconvertedSourceExpr2.result.coord))
          }
          vassert(isConvertible)
          val convertedSourceExpr2 =
            convertHelper.convert(nenv.snapshot, coutputs, range :: parentRanges, outerCallLocation, unconvertedSourceExpr2, destinationExpr2.result.coord);

          val exprTE =
            destinationExpr2 match {
              case LocalLookupTE(_, local) if nenv.unstackifieds.contains(local.name) => {
                // It was already moved, so this becomes a Restackify.
                nenv.markLocalRestackified(local.name)
                RestackifyTE(local, convertedSourceExpr2)
              }
              case _ => {
                MutateTE(destinationExpr2, convertedSourceExpr2)
              }
            }
          (exprTE, returnsFromSource)
        }
        case ExprMutateSE(range, destinationExpr1, sourceExpr1) => {
          vcurious(region == nenv.defaultRegion)
          val (unconvertedSourceExpr2, returnsFromSource) =
            evaluateAndCoerceToReferenceExpression(coutputs, nenv, life + 0, parentRanges, outerCallLocation, nenv.defaultRegion, sourceExpr1)
          val (destinationExpr2, returnsFromDestination) =
            evaluateExpectedAddressExpression(coutputs, nenv, life + 1, parentRanges, outerCallLocation, region, destinationExpr1)
          if (destinationExpr2.variability != VaryingT) {
            destinationExpr2 match {
              case ReferenceMemberLookupTE(range, structExpr, memberName, _, _) => {
                structExpr.kind match {
                  case s @ StructTT(_) => {
                    throw CompileErrorExceptionT(CantMutateFinalMember(range :: parentRanges, s, memberName))
                  }
                  case _ => vimpl(structExpr.kind.toString)
                }
              }
              case RuntimeSizedArrayLookupTE(range, arrayExpr, arrayType, _, _) => {
                throw CompileErrorExceptionT(CantMutateFinalElement(range :: parentRanges, arrayExpr.result.coord))
              }
              case StaticSizedArrayLookupTE(range, arrayExpr, arrayType, _, _, _) => {
                throw CompileErrorExceptionT(CantMutateFinalElement(range :: parentRanges, arrayExpr.result.coord))
              }
              case x => vimpl(x.toString)
            }
          }

          val isConvertible =
            templataCompiler.isTypeConvertible(coutputs, nenv.snapshot, range :: parentRanges, outerCallLocation, unconvertedSourceExpr2.result.coord, destinationExpr2.result.coord)
          if (!isConvertible) {
            throw CompileErrorExceptionT(CouldntConvertForMutateT(range :: parentRanges, destinationExpr2.result.coord, unconvertedSourceExpr2.result.coord))
          }
          val convertedSourceExpr2 =
            convertHelper.convert(nenv.snapshot, coutputs, range :: parentRanges, outerCallLocation, unconvertedSourceExpr2, destinationExpr2.result.coord);

          val mutate2 = MutateTE(destinationExpr2, convertedSourceExpr2);
          (mutate2, returnsFromSource ++ returnsFromDestination)
        }
        case OutsideLoadSE(range, rules, name, templateArgs1, targetOwnership) => {
          // So far, we only allow these when they're immediately called like functions
          throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Raw template specified lookups unimplemented!"))
        }
        case IndexSE(range, containerExpr1, indexExpr1) => {
          val (unborrowedContainerExpr2, returnsFromContainerExpr) =
            evaluate(coutputs, nenv, life + 0, parentRanges, outerCallLocation, nenv.defaultRegion, containerExpr1);
          val containerExpr2 =
            dotBorrow(coutputs, nenv, range :: parentRanges, outerCallLocation, life + 1, region, unborrowedContainerExpr2)

          val (indexExpr2, returnsFromIndexExpr) =
            evaluateAndCoerceToReferenceExpression(
              coutputs, nenv, life + 2, parentRanges, outerCallLocation, nenv.defaultRegion, indexExpr1);

          val exprTemplata =
            containerExpr2.result.coord.kind match {
              case rsa @ contentsRuntimeSizedArrayTT(_, _, _) => {
                arrayCompiler.lookupInUnknownSizedArray(parentRanges, range, containerExpr2, indexExpr2, rsa)
              }
              case at@contentsStaticSizedArrayTT(_, _, _, _, _) => {
                arrayCompiler.lookupInStaticSizedArray(range, containerExpr2, indexExpr2, at)
              }
//              case at@StructTT(FullNameT(ProgramT.topLevelName, Vector(), CitizenNameT(CitizenTemplateNameT(ProgramT.tupleHumanName), _))) => {
//                indexExpr2 match {
//                  case ConstantIntTE(index, _) => {
//                    val understructDef = coutputs.lookupStruct(at);
//                    val memberName = understructDef.fullName.addStep(understructDef.members(index.toInt).name)
//                    val memberType = understructDef.members(index.toInt).tyype
//
//                    vassert(understructDef.members.exists(member => understructDef.fullName.addStep(member.name) == memberName))
//
////                    val ownershipInClosureStruct = understructDef.members(index).tyype.reference.ownership
//
//                    val targetPermission =
//                      Compiler.intersectPermission(
//                        containerExpr2.result.reference.permission,
//                        memberType.reference.permission)
//
//                    ast.ReferenceMemberLookupTE(range, containerExpr2, memberName, memberType.reference, targetPermission, FinalT)
//                  }
//                  case _ => throw CompileErrorExceptionT(RangedInternalErrorT(range, "Struct random access not implemented yet!"))
//                }
//              }
              case _ => throw CompileErrorExceptionT(CannotSubscriptT(range :: parentRanges, containerExpr2.result.coord.kind))
              // later on, a map type could go here
            }
          (exprTemplata, returnsFromContainerExpr ++ returnsFromIndexExpr)
        }
        case DotSE(range, containerExpr1, memberNameStr, borrowContainer) => {
          val memberName = interner.intern(CodeVarNameT(memberNameStr))
          val (unborrowedContainerExpr2, returnsFromContainerExpr) =
            evaluate(coutputs, nenv, life + 0, parentRanges, outerCallLocation, region, containerExpr1)
          val containerExpr2 =
            dotBorrow(coutputs, nenv, range :: parentRanges, outerCallLocation, life + 1, region, unborrowedContainerExpr2)

          val expr2 =
            containerExpr2.result.coord.kind match {
              case structTT@StructTT(_) => {
                val structDef = coutputs.lookupStruct(structTT.id)
                val (structMember, memberIndex) =
                  structDef.getMemberAndIndex(memberName) match {
                    case None => {
                      throw CompileErrorExceptionT(
                        CouldntFindMemberT(range :: parentRanges, memberName.name.str))
                    }
                    case Some(x) => x
                  }
                val unsubstitutedMemberType = structMember.tyype.expectReferenceMember().reference;
                val memberType =
                  TemplataCompiler.getPlaceholderSubstituter(
                    opts.globalOptions.sanityCheck,
                    interner, keywords,
                    nenv.functionEnvironment.templateId,
                    structTT.id,
                    // Use the bounds that we supplied to the struct
                    UseBoundsFromContainer(
                      structDef.instantiationBoundParams,
                      vassertSome(coutputs.getInstantiationBounds(structTT.id))))
                    .substituteForCoord(coutputs, unsubstitutedMemberType)

                vassert(structDef.members.exists(_.name == memberName))

                ast.ReferenceMemberLookupTE(range, containerExpr2, memberName, memberType, structMember.variability)
              }
              case as@contentsStaticSizedArrayTT(_, _, _, _, _) => {
                if (memberNameStr.str.forall(Character.isDigit)) {
                  arrayCompiler.lookupInStaticSizedArray(
                    range,
                    containerExpr2,
                    ConstantIntTE(IntegerTemplataT(memberNameStr.str.toLong), 32, region),
                    as)
                } else {
                  throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Sequence has no member named " + memberNameStr))
                }
              }
              case at@contentsRuntimeSizedArrayTT(_, _, _) => {
                if (memberNameStr.str.forall(Character.isDigit)) {
                  arrayCompiler.lookupInUnknownSizedArray(
                    parentRanges, range, containerExpr2,
                    ConstantIntTE(IntegerTemplataT(memberNameStr.str.toLong), 32, region),
                    at)
                } else {
                  throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Array has no member named " + memberNameStr))
                }
              }
              case other => {
                throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Can't apply ." + memberNameStr + " to " + other))
              }
            }

          expr2.result.kind match {
            case ICitizenTT(id) => {
              vassert(coutputs.getInstantiationBounds(id).nonEmpty)
            }
            case _ =>
          }

          (expr2, returnsFromContainerExpr)
        }
        case FunctionSE(functionS @ FunctionS(range, name, _, _, _, _, _, _, _, _)) => {
          val callExpr2 = evaluateClosure(coutputs, nenv, range :: parentRanges, outerCallLocation, region, name, functionS)
          (callExpr2, Set())
        }
        case TupleSE(range, elements1) => {
          val (exprs2, returnsFromElements) =
            evaluateAndCoerceToReferenceExpressions(coutputs, nenv, life + 0, parentRanges, outerCallLocation, nenv.defaultRegion, elements1);

          // would we need a sequence templata? probably right?
          val expr2 = sequenceCompiler.resolveTuple(nenv.snapshot, coutputs, parentRanges, outerCallLocation, exprs2)
          (expr2, returnsFromElements)
        }
        case StaticArrayFromValuesSE(range, rules, maybeElementTypeRuneA, mutabilityRune, variabilityRune, sizeRuneA, elements1) => {
          val (exprs2, returnsFromElements) =
            evaluateAndCoerceToReferenceExpressions(
              coutputs, nenv, life, parentRanges, outerCallLocation, nenv.defaultRegion, elements1);
          // would we need a sequence templata? probably right?
          val expr2 =
            arrayCompiler.evaluateStaticSizedArrayFromValues(
              coutputs,
              nenv.snapshot,
              range :: parentRanges,
              outerCallLocation,
              rules.toVector,
              maybeElementTypeRuneA.map(_.rune),
              sizeRuneA.rune,
              mutabilityRune.rune,
              variabilityRune.rune,
              exprs2,
              region)
          (expr2, returnsFromElements)
        }
        case StaticArrayFromCallableSE(range, rules, maybeElementTypeRune, maybeMutabilityRune, maybeVariabilityRune, sizeRuneA, callableAE) => {
          val (callableTE, returnsFromCallable) =
            evaluateAndCoerceToReferenceExpression(
              coutputs, nenv, life, parentRanges, outerCallLocation, nenv.defaultRegion, callableAE);
          val expr2 =
            arrayCompiler.evaluateStaticSizedArrayFromCallable(
              coutputs,
              nenv.snapshot,
              region,
              range :: parentRanges,
              outerCallLocation,
              rules.toVector,
              maybeElementTypeRune.map(_.rune),
              sizeRuneA.rune,
              maybeMutabilityRune.rune,
              maybeVariabilityRune.rune,
              callableTE)
          (expr2, returnsFromCallable)
        }
        case NewRuntimeSizedArraySE(range, rulesA, maybeElementTypeRune, mutabilityRune, sizeAE, maybeCallableAE) => {
          val (sizeTE, returnsFromSize) =
            evaluateAndCoerceToReferenceExpression(
              coutputs,
              nenv,
              life + 0,
              parentRanges,
              outerCallLocation,
              region,
              sizeAE);
          val (maybeCallableTE, returnsFromCallable) =
            maybeCallableAE match {
              case None => (None, Vector())
              case Some(callableAE) => {
                val (callableTE, rets) =
                  evaluateAndCoerceToReferenceExpression(
                    coutputs, nenv, life + 1, parentRanges, outerCallLocation, nenv.defaultRegion, callableAE);
                (Some(callableTE), rets)
              }
            }


          val expr2 =
            arrayCompiler.evaluateRuntimeSizedArrayFromCallable(
              coutputs,
              nenv.snapshot,
              range :: parentRanges,
              outerCallLocation,
              region,
              rulesA.toVector,
              maybeElementTypeRune.map(_.rune),
              mutabilityRune.rune,
              sizeTE,
              maybeCallableTE)
          (expr2, returnsFromSize ++ returnsFromCallable)
        }
        case LetSE(range, rulesA, pattern, sourceExpr1) => {
          val (sourceExpr2, returnsFromSource) =
            evaluateAndCoerceToReferenceExpression(
              coutputs, nenv, life + 0, parentRanges, outerCallLocation, nenv.defaultRegion, sourceExpr1)


          val runeTypeSolveEnv =
            new IRuneTypeSolverEnv {
              override def lookup(range: RangeS, nameS: IImpreciseNameS):
              Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
                nenv.lookupNearestWithImpreciseName(nameS, Set(TemplataLookupContext)) match {
                  case Some(CitizenDefinitionTemplataT(environment, a)) => {
                    Ok(CitizenRuneTypeSolverLookupResult(a.tyype, a.genericParameters))
                  }
                  case Some(x) => Ok(TemplataLookupResult(x.tyype))
                  case None => Err(RuneTypingCouldntFindType(range, nameS))
                }
              }
            }

          val runeToInitiallyKnownType = PatternSUtils.getRuneTypesFromPattern(pattern)
          val runeToType =
            new RuneTypeSolver(interner).solve(
              opts.globalOptions.sanityCheck,
              opts.globalOptions.useOptimizedSolver,
              runeTypeSolveEnv,
              range :: parentRanges,
              false,
              rulesA,
              List(),
              true,
              runeToInitiallyKnownType.toMap) match {
              case Ok(r) => r
              case Err(e) => {
                throw CompileErrorExceptionT(HigherTypingInferError(
                  range ::
                      parentRanges, e))
              }
            }
          val resultTE =
            patternCompiler.inferAndTranslatePattern(
              coutputs,
              nenv,
              life + 1,
              parentRanges,
              outerCallLocation,
              rulesA.toVector,
              runeToType,
              pattern,
              sourceExpr2,
              region,
              (coutputs, nenv, life, liveCaptureLocals) => VoidLiteralTE(nenv.defaultRegion))

          (resultTE, returnsFromSource)
        }
        case r @ RuneLookupSE(range, runeA) => {
          val templata = vassertOne(nenv.lookupNearestWithImpreciseName(interner.intern(RuneNameS(runeA)), Set(TemplataLookupContext)))
          templata match {
            case IntegerTemplataT(value) => {
              (ConstantIntTE(
                IntegerTemplataT(value),
                32,
                region), Set())
            }
            case PlaceholderTemplataT(name, IntegerTemplataType()) => {
              (ConstantIntTE(
                PlaceholderTemplataT(name, IntegerTemplataType()),
                32,
                region), Set())
            }
            case pt @ PrototypeTemplataT(_) => {
              val tinyEnv =
                nenv.functionEnvironment.makeChildNodeEnvironment(r, life)
                  .addEntries(interner, Vector(ArbitraryNameT() -> TemplataEnvEntry(pt)))
              val expr =
                newGlobalFunctionGroupExpression(
                  tinyEnv, coutputs, RegionT(), interner.intern(ArbitraryNameS()))
              (expr, Set())
            }
          }
        }
        case IfSE(range, conditionSE, thenBodySE, elseBodySE) => {
          // We make a block for the if-statement which contains its condition (the "if block"),
          // and then two child blocks under that for the then and else blocks.
          // The then and else blocks are children of the block which contains the condition
          // so they can access any locals declared by the condition.

          val (conditionExpr, returnsFromCondition) =
            evaluateAndCoerceToReferenceExpression(
              coutputs, nenv, life + 1, parentRanges, outerCallLocation, nenv.defaultRegion, conditionSE)
          conditionExpr.result.coord match {
            case CoordT(ShareT, _, BoolT()) =>
            case _ => {
              throw CompileErrorExceptionT(IfConditionIsntBoolean(conditionSE.range :: parentRanges, conditionExpr.result.coord))
            }
          }

          val thenFate = NodeEnvironmentBox(nenv.makeChild(thenBodySE, None))

          val (thenExpressionsWithResult, thenReturnsFromExprs) =
            evaluateBlockStatements(
              coutputs,
              thenFate.snapshot,
              thenFate,
              life + 2,
              parentRanges,
              outerCallLocation,
              nenv.defaultRegion,
              thenBodySE)
          val uncoercedThenBlock2 = BlockTE(thenExpressionsWithResult)

          val (thenUnstackifiedAncestorLocals, thenRestackifiedAncestorLocals) = thenFate.snapshot.getEffectsSince(nenv.snapshot)
          val thenContinues =
            uncoercedThenBlock2.result.coord.kind match {
              case NeverT(_) => false
              case _ => true
            }

          val elseFate = NodeEnvironmentBox(nenv.makeChild(elseBodySE, None))

          val (elseExpressionsWithResult, elseReturnsFromExprs) =
            evaluateBlockStatements(
              coutputs,
              elseFate.snapshot,
              elseFate,
              life + 3,
              parentRanges,
              outerCallLocation,
              nenv.defaultRegion,
              elseBodySE)
          val uncoercedElseBlock2 = BlockTE(elseExpressionsWithResult)

          val (elseUnstackifiedAncestorLocals, elseRestackifiedAncestorLocals) = elseFate.snapshot.getEffectsSince(nenv.snapshot)
          val elseContinues =
            uncoercedElseBlock2.result.coord.kind match {
              case NeverT(_) => false
              case _ => true
            }

          if (thenContinues && elseContinues && uncoercedThenBlock2.result.coord.ownership != uncoercedElseBlock2.result.coord.ownership) {
            throw CompileErrorExceptionT(CantReconcileBranchesResults(range :: parentRanges, uncoercedThenBlock2.result.coord, uncoercedElseBlock2.result.coord))
          }

          val commonType =
            (uncoercedThenBlock2.kind, uncoercedElseBlock2.kind) match {
              // If one side has a return-never, use the other side.
              case (NeverT(false), _) => uncoercedElseBlock2.result.coord
              case (_, NeverT(false)) => uncoercedThenBlock2.result.coord
              // If we get here, theres no return-nevers in play.
              // If one side has a break-never, use the other side.
              case (NeverT(true), _) => uncoercedElseBlock2.result.coord
              case (_, NeverT(true)) => uncoercedThenBlock2.result.coord
              case (a, b) if a == b => uncoercedThenBlock2.result.coord
              case (a : ICitizenTT, b : ICitizenTT) => {
                val aAncestors = ancestorHelper.getParents(coutputs, parentRanges, outerCallLocation, nenv.snapshot, a).toSet
                val bAncestors = ancestorHelper.getParents(coutputs, parentRanges, outerCallLocation, nenv.snapshot, b).toSet
                val commonAncestors = aAncestors.intersect(bAncestors)

                if (uncoercedElseBlock2.result.coord.ownership != uncoercedElseBlock2.result.coord.ownership) {
                  throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Two branches of if have different ownerships!\\n${a}\\n${b}"))
                }
                val ownership = uncoercedElseBlock2.result.coord.ownership

                if (commonAncestors.isEmpty) {
                  throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, s"No common ancestors of two branches of if:\n${a}\n${b}"))
                } else if (commonAncestors.size > 1) {
                  throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, s"More than one common ancestor of two branches of if:\n${a}\n${b}"))
                } else {
                  CoordT(ownership, RegionT(), commonAncestors.head)
                }
              }
              case (a, b) => {
                throw CompileErrorExceptionT(CantReconcileBranchesResults(range :: parentRanges, uncoercedThenBlock2.result.coord, uncoercedElseBlock2.result.coord))
              }
            }
          val thenExpr2 = convertHelper.convert(thenFate.snapshot, coutputs, range :: parentRanges, outerCallLocation, uncoercedThenBlock2, commonType)
          val elseExpr2 = convertHelper.convert(elseFate.snapshot, coutputs, range :: parentRanges, outerCallLocation, uncoercedElseBlock2, commonType)

          val ifExpr2 = IfTE(conditionExpr, thenExpr2, elseExpr2)


          if (thenContinues == elseContinues) { // Both continue, or both don't
            // Each branch might have moved some things. Make sure they moved the same things.
            if (thenUnstackifiedAncestorLocals != elseUnstackifiedAncestorLocals) {
              throw CompileErrorExceptionT(RangedInternalErrorT(
                range :: parentRanges,
                "Must move same variables from inside branches!\nFrom then branch: " +
                  thenUnstackifiedAncestorLocals +
                  "\nFrom else branch: " +
                  elseUnstackifiedAncestorLocals))
            }
            if (thenRestackifiedAncestorLocals != elseRestackifiedAncestorLocals) {
              throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Must reinitialize same variables from inside branches!\nFrom then branch: " + thenUnstackifiedAncestorLocals + "\nFrom else branch: " + elseUnstackifiedAncestorLocals))
            }
            if (thenRestackifiedAncestorLocals != elseRestackifiedAncestorLocals) {
              throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Must reinitialize same variables from inside branches!\nFrom then branch: " + thenUnstackifiedAncestorLocals + "\nFrom else branch: " + elseUnstackifiedAncestorLocals))
            }
            thenUnstackifiedAncestorLocals.foreach(nenv.markLocalUnstackified)
            thenRestackifiedAncestorLocals.foreach(nenv.markLocalRestackified)
          } else {
            // One of them continues and the other does not.
            if (thenContinues) {
              thenUnstackifiedAncestorLocals.foreach(nenv.markLocalUnstackified)
              thenRestackifiedAncestorLocals.foreach(nenv.markLocalRestackified)
            } else if (elseContinues) {
              elseUnstackifiedAncestorLocals.foreach(nenv.markLocalUnstackified)
              elseRestackifiedAncestorLocals.foreach(nenv.markLocalRestackified)
            } else vfail()
          }

          val (ifBlockUnstackifiedAncestorLocals, ifBlockRestackifiedAncestorLocals) =
            nenv.snapshot.getEffectsSince(nenv.snapshot)
          ifBlockUnstackifiedAncestorLocals.foreach(nenv.markLocalUnstackified)
          ifBlockRestackifiedAncestorLocals.foreach(nenv.markLocalRestackified)

          (ifExpr2, returnsFromCondition ++ thenReturnsFromExprs ++ elseReturnsFromExprs)
        }
        case w @ WhileSE(range, bodySE) => {
          // We make a block for the while-statement which contains its condition (the "if block"),
          // and the body block, so they can access any locals declared by the condition.

          // See BEAFB for why we make a new environment for the While
          val loopNenv = nenv.makeChild(w, None)

          val loopBlockFate = NodeEnvironmentBox(loopNenv.makeChild(bodySE, None))
          val (bodyExpressionsWithResult, bodyReturnsFromExprs) =
            evaluateBlockStatements(
              coutputs,
              loopBlockFate.snapshot,
              loopBlockFate,
              life + 1,
              parentRanges,
              outerCallLocation,
              nenv.defaultRegion,
              bodySE)
          val uncoercedBodyBlock2 = BlockTE(bodyExpressionsWithResult)

          uncoercedBodyBlock2.kind match {
            case NeverT(_) =>
            case _ => {
              val (bodyUnstackifiedAncestorLocals, bodyRestackifiedAncestorLocals) =
                loopBlockFate.snapshot.getEffectsSince(nenv.snapshot)

              if (bodyUnstackifiedAncestorLocals.nonEmpty) {
                throw CompileErrorExceptionT(
                  CantUnstackifyOutsideLocalFromInsideWhile(
                    range :: parentRanges, bodyUnstackifiedAncestorLocals.head))
              }
              if (bodyRestackifiedAncestorLocals.nonEmpty) {
                throw CompileErrorExceptionT(CantRestackifyOutsideLocalFromInsideWhile(range :: parentRanges, bodyUnstackifiedAncestorLocals.head))
              }
              if (bodyRestackifiedAncestorLocals.nonEmpty) {
                throw CompileErrorExceptionT(CantRestackifyOutsideLocalFromInsideWhile(range :: parentRanges, bodyUnstackifiedAncestorLocals.head))
              }
            }
          }

          val loopExpr2 = WhileTE(uncoercedBodyBlock2)
          // (loopExpr2, returnsFromCondition ++ bodyReturnsFromExprs)
          (loopExpr2, bodyReturnsFromExprs)
        }
        case m @ MapSE(range, bodySE) => {
          // Preprocess the entire loop once, to predict what its result type
          // will be.
          // We can't just use this, because any returns inside won't drop
          // the temporary list.
          val elementRefT =
            {
              // See BEAFB for why we make a new environment for the While
              val loopNenv = nenv.makeChild(m, None)
              val loopBlockFate = NodeEnvironmentBox(loopNenv.makeChild(bodySE, None))
              val (bodyExpressionsWithResult, _) =
                evaluateBlockStatements(
                  coutputs,
                  loopBlockFate.snapshot,
                  loopBlockFate,
                  life + 1,
                  parentRanges,
                  outerCallLocation,
                  vregionmut(RegionT()),
                  bodySE)
              bodyExpressionsWithResult.result.coord
            }

          // Now that we know the result type, let's make a temporary list.

          val callEnv =
            nenv.snapshot
              .copy(templatas =
                nenv.snapshot.templatas
                  .addEntry(interner, interner.intern(RuneNameT(SelfRuneS())), TemplataEnvEntry(templata.CoordTemplataT(elementRefT))))
          val makeListTE =
            callCompiler.evaluatePrefixCall(
              coutputs,
              nenv,
              life + 1,
              range :: parentRanges,
              outerCallLocation,
              region,
              newGlobalFunctionGroupExpression(
                callEnv, coutputs, vregionmut(RegionT()), interner.intern(CodeNameS(keywords.List))),
              Vector(RuneParentEnvLookupSR(range, RuneUsage(range, SelfRuneS()))),
              Vector(SelfRuneS()),
              Vector())

          val listLocal =
            localHelper.makeTemporaryLocal(
              nenv, life + 2, makeListTE.result.coord)
          val letListTE =
            LetNormalTE(listLocal, makeListTE)

          val (loopTE, returnsFromLoop) =
            {
              // See BEAFB for why we make a new environment for the While
              val loopNenv = nenv.makeChild(m, vregionmut(None))

              val loopBlockFate = NodeEnvironmentBox(loopNenv.makeChild(bodySE, vregionmut(None)))
              val (userBodyTE, bodyReturnsFromExprs) =
                evaluateBlockStatements(
                  coutputs,
                  loopBlockFate.snapshot,
                  loopBlockFate,
                  life + 1,
                  parentRanges,
                  outerCallLocation,
                  vregionmut(RegionT()),
                  bodySE)

              // We store the iteration result in a local because the loop body will have
              // breaks, and we can't have a BreakTE inside a FunctionCallTE, see BRCOBS.
              val iterationResultLocal =
                localHelper.makeTemporaryLocal(
                  nenv, life + 3, userBodyTE.result.coord)
              val letIterationResultTE =
                LetNormalTE(iterationResultLocal, userBodyTE)

              val addCall =
                callCompiler.evaluatePrefixCall(
                  coutputs,
                  nenv,
                  life + 4,
                  range :: parentRanges,
                  outerCallLocation,
                  region,
                  newGlobalFunctionGroupExpression(callEnv, coutputs, RegionT(), interner.intern(CodeNameS(keywords.add))),
                  Vector(),
                  Vector(),
                  Vector(
                    localHelper.borrowSoftLoad(
                      coutputs,
                      LocalLookupTE(
                        range,
                        listLocal)),
                    localHelper.unletLocalWithoutDropping(nenv, iterationResultLocal)))
              val bodyTE = BlockTE(Compiler.consecutive(Vector(letIterationResultTE, addCall)))

              val (bodyUnstackifiedAncestorLocals, bodyRestackifiedAncestorLocals) =
                loopBlockFate.snapshot.getEffectsSince(nenv.snapshot)
              if (bodyUnstackifiedAncestorLocals.nonEmpty) {
                throw CompileErrorExceptionT(CantUnstackifyOutsideLocalFromInsideWhile(range :: parentRanges, bodyUnstackifiedAncestorLocals.head))
              }
              if (bodyRestackifiedAncestorLocals.nonEmpty) {
                throw CompileErrorExceptionT(CantRestackifyOutsideLocalFromInsideWhile(range :: parentRanges, bodyRestackifiedAncestorLocals.head))
              }

              val whileTE = WhileTE(bodyTE)
              (whileTE, bodyReturnsFromExprs)
            }

          val unletListTE =
            localHelper.unletLocalWithoutDropping(nenv, listLocal)

          val combinedTE =
            Compiler.consecutive(Vector(letListTE, loopTE, unletListTE))

          (combinedTE, returnsFromLoop)
        }
        case ConsecutorSE(exprsSE) => {
          vcurious(region == nenv.defaultRegion)
          val regionForInners = region

          val (initExprsTE, initReturnsUnflattened) =
            exprsSE.init.zipWithIndex.map({ case (exprSE, index) =>
              val (undroppedExprTE, returns) =
                evaluateAndCoerceToReferenceExpression(
                  coutputs, nenv, life + index, parentRanges, outerCallLocation, regionForInners, exprSE)
              val exprTE =
                undroppedExprTE.result.kind match {
                  case VoidT() => undroppedExprTE
                  case _ => {
                    destructorCompiler.drop(
                      nenv.snapshot, coutputs, exprSE.range :: parentRanges, outerCallLocation, region, undroppedExprTE)
                  }
                }
              (exprTE, returns)
            }).unzip

          val (lastExprTE, lastReturns) =
            evaluateAndCoerceToReferenceExpression(
              coutputs,
              nenv,
              life + (exprsSE.size - 1),
              parentRanges,
              outerCallLocation,
              regionForInners,
              exprsSE.last)

          (Compiler.consecutive(initExprsTE :+ lastExprTE), (initReturnsUnflattened.flatten ++ lastReturns).toSet)
        }
        case p@PureSE(range, location, inner) => {
          evaluateAndCoerceToReferenceExpression(
            coutputs, nenv, life + 0, parentRanges, outerCallLocation, region, inner)
        }
        case b @ BlockSE(range, locals, _) => {
          val childEnvironment = NodeEnvironmentBox(nenv.makeChild(b, None))

          val (expressionsWithResult, returnsFromExprs) =
            evaluateBlockStatements(
              coutputs,
              childEnvironment.snapshot,
              childEnvironment,
              life,
              parentRanges,
              outerCallLocation,
              nenv.defaultRegion,
              b)
          val block2 = BlockTE(expressionsWithResult)

          val (unstackifiedAncestorLocals, restackifiedAncestorLocals) =
            childEnvironment.snapshot.getEffectsSince(nenv.snapshot)
          unstackifiedAncestorLocals.foreach(nenv.markLocalUnstackified)
          restackifiedAncestorLocals.foreach(nenv.markLocalRestackified)

          (block2, returnsFromExprs)
        }
        case DestructSE(range, innerAE) => {
          val (innerExpr2, returnsFromArrayExpr) =
            evaluateAndCoerceToReferenceExpression(
              coutputs, nenv, life + 0, parentRanges, outerCallLocation, region, innerAE);

          // should just ignore others, TODO impl
          vcheck(innerExpr2.result.coord.ownership == OwnT, "can only destruct own")

          val destroy2 =
            innerExpr2.kind match {
              case structTT@StructTT(_) => {
                val structDef = coutputs.lookupStruct(structTT.id)
                val substituter =
                  TemplataCompiler.getPlaceholderSubstituter(
                    opts.globalOptions.sanityCheck,
                    interner, keywords,
                    nenv.functionEnvironment.templateId,
                    structTT.id,
                    // This type is already phrased in terms of our placeholders, so it can use the
                    // bounds it already has.
                    InheritBoundsFromTypeItself)
                DestroyTE(
                  innerExpr2,
                  structTT,
                  structDef.members
                    .zipWithIndex
                    .map({
                      case (NormalStructMemberT(_, _, ReferenceMemberTypeT(coord)), index) => (coord, index)
                      case (NormalStructMemberT(_, _, AddressMemberTypeT(_)), index) => vimpl()
                      case (VariadicStructMemberT(_, _), _) => vimpl()
                    })
                    .map({ case (unsubstitutedCoord, index) =>
                      val reference = substituter.substituteForCoord(coutputs, unsubstitutedCoord)
                      localHelper.makeTemporaryLocal(nenv, life + 1 + index, reference)
                    }))
              }
              case interfaceTT @ InterfaceTT(_) => {
                destructorCompiler.drop(nenv.snapshot, coutputs, range :: parentRanges, outerCallLocation, region, innerExpr2)
              }
              case _ => vfail("Can't destruct type: " + innerExpr2.kind)
            }
          (destroy2, returnsFromArrayExpr)
        }
        case UnletSE(range, nameA) => {
          val name = nameTranslator.translateVarNameStep(nameA)
          val local =
            nenv.getVariable(name) match {
              case Some(lv : ILocalVariableT) => lv
              case Some(_) => {
                throw CompileErrorExceptionT(RangedInternalErrorT(
                  range ::
                    parentRanges, "Can't unlet local: " + name))
              }
              case None => {
                throw CompileErrorExceptionT(RangedInternalErrorT(
                  range :: parentRanges,
                  "No local with name: " + name))
              }
            }
          val resultExpr = localHelper.unletLocalWithoutDropping(nenv, local)
          // This will likely be dropped, as theyre probably not doing anything with it.
          // But who knows, maybe they'll do something with it, like pass it as a parameter
          // to something.

          (resultExpr, Set())
        }
        case ReturnSE(range, innerExprA) => {
          val (uncastedInnerExpr2, returnsFromInnerExpr) =
            evaluateAndCoerceToReferenceExpression(coutputs, nenv, life + 0, parentRanges, outerCallLocation, region, innerExprA);

          val innerExpr2 =
            nenv.maybeReturnType match {
              case None => (uncastedInnerExpr2)
              case Some(returnType) => {
                templataCompiler.isTypeConvertible(coutputs, nenv.snapshot, range :: parentRanges, outerCallLocation, uncastedInnerExpr2.result.coord, returnType) match {
                  case (false) => {
                    throw CompileErrorExceptionT(
                      CouldntConvertForReturnT(range :: parentRanges, returnType, uncastedInnerExpr2.result.coord))
                  }
                  case (true) => {
                    convertHelper.convert(nenv.snapshot, coutputs, range :: parentRanges, outerCallLocation, uncastedInnerExpr2, returnType)
                  }
                }
              }
            }

          val allLocals = nenv.getAllLocals()
          val unstackifiedLocals = nenv.getAllUnstackifiedLocals()
          val variablesToDestruct = allLocals.filter(x => !unstackifiedLocals.contains(x.name))
          val reversedVariablesToDestruct = variablesToDestruct.reverse

          val returns = returnsFromInnerExpr + innerExpr2.result.coord

          val resultVarId = interner.intern(TypingPassFunctionResultVarNameT())
          val resultVariable = ReferenceLocalVariableT(resultVarId, FinalT, innerExpr2.result.coord)
          val resultLet = ast.LetNormalTE(resultVariable, innerExpr2)
          nenv.addVariable(resultVariable)

          val destructExprs =
            localHelper.unletAndDropAll(
              coutputs, nenv, range :: parentRanges, outerCallLocation, region, reversedVariablesToDestruct)

          val getResultExpr =
            localHelper.unletLocalWithoutDropping(nenv, resultVariable)

          val consecutor = Compiler.consecutive(Vector(resultLet) ++ destructExprs ++ Vector(getResultExpr))

          (ReturnTE(consecutor), returns)
        }
        case BreakSE(range) => {
          // See BEAFB, we need to find the nearest while to see local since then.
          nenv.nearestLoopEnv() match {
            case None => {
              throw CompileErrorExceptionT(RangedInternalErrorT(
                range :: parentRanges,
                "Using break while not inside loop!"))
            }
            case Some((whileNenv, _)) => {
              vcurious(region == nenv.defaultRegion)
              val dropsTE =
                dropSince(
                  coutputs,
                  whileNenv,
                  nenv,
                  range :: parentRanges,
                  outerCallLocation,
                  life,
                  region,
                  VoidLiteralTE(region))
              val dropsAndBreakTE = Compiler.consecutive(Vector(dropsTE, BreakTE(region)))
              (dropsAndBreakTE, Set())
            }
          }
        }
        case _ => {
          println(expr1)
          vfail(expr1.toString)
        }
      }
    })
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn check_array(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        array_mutability: MutabilityT,
        element_coord: CoordT<'s, 't>,
        generator_prototype: PrototypeT<'s, 't>,
        generator_type: CoordT<'s, 't>,
    ) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  private def checkArray(
      coutputs: CompilerOutputs,
      range: List[RangeS],
      arrayMutability: MutabilityT,
      elementCoord: CoordT,
      generatorPrototype: PrototypeT[IFunctionNameT],
      generatorType: CoordT
  ) = {
    if (generatorPrototype.returnType != elementCoord) {
      throw CompileErrorExceptionT(RangedInternalErrorT(range, "Generator return type doesn't agree with array element type!"))
    }
    if (generatorPrototype.paramTypes.size != 2) {
      throw CompileErrorExceptionT(RangedInternalErrorT(range, "Generator must take in 2 args!"))
    }
    if (generatorPrototype.paramTypes(0) != generatorType) {
      throw CompileErrorExceptionT(RangedInternalErrorT(range, "Generator first param doesn't agree with generator expression's result!"))
    }
    generatorPrototype.paramTypes(1) match {
      case CoordT(ShareT, _, IntT.i32) =>
      case _ => {
        throw CompileErrorExceptionT(
          RangedInternalErrorT(range, "Generator must take in an integer as its second param!"))
      }
    }
    if (arrayMutability == ImmutableT &&
      Compiler.getMutability(coutputs, elementCoord.kind) == MutabilityTemplataT(MutableT)) {
      throw CompileErrorExceptionT(RangedInternalErrorT(range, "Can't have an immutable array of mutable elements!"))
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_option(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &'t FunctionEnvironmentT<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        context_region: RegionT,
        contained_coord: CoordT<'s, 't>,
    ) -> (CoordT<'s, 't>, PrototypeT<'s, 't>, PrototypeT<'s, 't>, IdT<'s, 't>, IdT<'s, 't>) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def getOption(
    coutputs: CompilerOutputs,
    nenv: FunctionEnvironmentT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    containedCoord: CoordT):
  (CoordT, PrototypeT[IFunctionNameT], PrototypeT[IFunctionNameT], IdT[IImplNameT], IdT[IImplNameT]) = {
    val interfaceTemplata =
      nenv.lookupNearestWithImpreciseName(interner.intern(CodeNameS(keywords.Opt)), Set(TemplataLookupContext)).toList match {
        case List(it@InterfaceDefinitionTemplataT(_, _)) => it
        case _ => vfail()
      }
    val optInterfaceRef =
      structCompiler.resolveInterface(
        coutputs,
        nenv,
        range,
        callLocation,
        interfaceTemplata,
        Vector(CoordTemplataT(containedCoord))).expect().kind
    val ownOptCoord = CoordT(OwnT, RegionT(), optInterfaceRef)

    val someConstructorTemplata =
      nenv.lookupNearestWithImpreciseName(interner.intern(CodeNameS(keywords.Some)), Set(ExpressionLookupContext)).toList match {
        case List(ft@FunctionTemplataT(_, _)) => ft
        case _ => vwat();
      }
    val someConstructor =
      delegate.evaluateGenericFunctionFromCallForPrototype(
        coutputs, nenv, range, callLocation, someConstructorTemplata, Vector(CoordTemplataT(containedCoord)), contextRegion, Vector(containedCoord)) match {
        case fff@ResolveFunctionFailure(_) => {
          throw CompileErrorExceptionT(RangedInternalErrorT(range, fff.toString))
        }
        case ResolveFunctionSuccess(p, conclusions) => p.prototype
      }

    val noneConstructorTemplata =
      nenv.lookupNearestWithImpreciseName(interner.intern(CodeNameS(keywords.None)), Set(ExpressionLookupContext)).toList match {
        case List(ft@FunctionTemplataT(_, _)) => ft
        case _ => vwat();
      }
    val noneConstructor =
      delegate.evaluateGenericFunctionFromCallForPrototype(
        coutputs, nenv, range, callLocation, noneConstructorTemplata, Vector(CoordTemplataT(containedCoord)), contextRegion, Vector()) match {
        case fff@ResolveFunctionFailure(_) => {
          throw CompileErrorExceptionT(RangedInternalErrorT(
            range,
            fff.toString))
        }
        case ResolveFunctionSuccess(p, conclusions) => p.prototype
      }

    val someImplId =
      implCompiler.isParent(coutputs, nenv, range, callLocation, someConstructor.returnType.kind.expectCitizen(), optInterfaceRef) match {
        case IsParent(_, _, implId) => implId
        case IsntParent(_) => vwat()
      }

    val noneImplId =
      implCompiler.isParent(coutputs, nenv, range, callLocation, noneConstructor.returnType.kind.expectCitizen(), optInterfaceRef) match {
        case IsParent(_, _, implId) => implId
        case IsntParent(_) => vwat()
      }

    (ownOptCoord, someConstructor, noneConstructor, someImplId, noneImplId)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_result(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &'t FunctionEnvironmentT<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        contained_success_coord: CoordT<'s, 't>,
        contained_fail_coord: CoordT<'s, 't>,
    ) -> (CoordT<'s, 't>, PrototypeT<'s, 't>, IdT<'s, 't>, PrototypeT<'s, 't>, IdT<'s, 't>) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def getResult(
    coutputs: CompilerOutputs,
    nenv: FunctionEnvironmentT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    containedSuccessCoord: CoordT,
    containedFailCoord: CoordT):
  (CoordT, PrototypeT[IFunctionNameT], IdT[IImplNameT], PrototypeT[IFunctionNameT], IdT[IImplNameT]) = {
    val interfaceTemplata =
      nenv.lookupNearestWithImpreciseName(interner.intern(CodeNameS(keywords.Result)), Set(TemplataLookupContext)).toList match {
        case List(it@InterfaceDefinitionTemplataT(_, _)) => it
        case _ => vfail()
      }
    val resultInterfaceRef =
      structCompiler.resolveInterface(
        coutputs,
        nenv,
        range,
        callLocation,
        interfaceTemplata,
        Vector(
          CoordTemplataT(containedSuccessCoord),
          CoordTemplataT(containedFailCoord))).expect().kind
    val ownResultCoord = CoordT(OwnT, region, resultInterfaceRef)

    val okConstructorTemplata =
      nenv.lookupNearestWithImpreciseName(interner.intern(CodeNameS(keywords.Ok)), Set(ExpressionLookupContext)).toList match {
        case List(ft@FunctionTemplataT(_, _)) => ft
        case _ => vwat();
      }
    val okConstructor =
      delegate.evaluateGenericFunctionFromCallForPrototype(
        coutputs,
        nenv,
        range,
        callLocation,
        okConstructorTemplata,
        Vector(CoordTemplataT(containedSuccessCoord), CoordTemplataT(containedFailCoord)),
        region,
        Vector(containedSuccessCoord)) match {
        case fff@ResolveFunctionFailure(_) => {
          throw CompileErrorExceptionT(RangedInternalErrorT(
            range,
            fff.toString))
        }
        case ResolveFunctionSuccess(p, conclusions) => p.prototype
      }
    val okKind = okConstructor.returnType.kind
    val okResultImpl =
      implCompiler.isParent(coutputs, nenv, range, callLocation, okKind.expectStruct(), resultInterfaceRef) match {
        case IsParent(templata, conclusions, implId) => implId
        case IsntParent(candidates) => vfail()
      }

    val errConstructorTemplata =
      nenv.lookupNearestWithImpreciseName(interner.intern(CodeNameS(keywords.Err)), Set(ExpressionLookupContext)).toList match {
        case List(ft@FunctionTemplataT(_, _)) => ft
        case _ => vwat();
      }
    val errConstructor =
      delegate.evaluateGenericFunctionFromCallForPrototype(
        coutputs,
        nenv,
        range,
        callLocation,
        errConstructorTemplata,
        Vector(CoordTemplataT(containedSuccessCoord), CoordTemplataT(containedFailCoord)),
        region,
        Vector(containedFailCoord)) match {
        case fff@ResolveFunctionFailure(_) => {
          throw CompileErrorExceptionT(RangedInternalErrorT(
            range,
            fff.toString))
        }
        case ResolveFunctionSuccess(p, conclusions) => p.prototype
      }
    val errKind = errConstructor.returnType.kind
    val errResultImpl =
      implCompiler.isParent(coutputs, nenv, range, callLocation, errKind.expectStruct(), resultInterfaceRef) match {
        case IsParent(templata, conclusions, implId) => implId
        case IsntParent(candidates) => vfail()
      }

    (ownResultCoord, okConstructor, okResultImpl, errConstructor, errResultImpl)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn weak_alias(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        expr: &'t ReferenceExpressionTE<'s, 't>,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def weakAlias(coutputs: CompilerOutputs, expr: ReferenceExpressionTE): ReferenceExpressionTE = {
    expr.kind match {
      case sr @ StructTT(_) => {
        val structDef = coutputs.lookupStruct(sr.id)
        vcheck(structDef.weakable, TookWeakRefOfNonWeakableError)
      }
      case ir @ InterfaceTT(_) => {
        val interfaceDef = coutputs.lookupInterface(ir)
        vcheck(interfaceDef.weakable, TookWeakRefOfNonWeakableError)
      }
      case _ => vfail()
    }

    expr.result.coord.ownership match {
      case BorrowT => BorrowToWeakTE(expr)
      case other => vale.vwat(other)
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn dot_borrow(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        life: LocationInFunctionEnvironmentT<'s>,
        context_region: RegionT,
        undecayed_unborrowed_container_expr_2: ExpressionTE<'s, 't>,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  // Borrow like the . does. If it receives an owning reference, itll make a temporary.
  // If it receives an owning address, that's fine, just borrowsoftload from it.
  // Rename this someday.
  private def dotBorrow(
      coutputs: CompilerOutputs,
      nenv: NodeEnvironmentBox,
      range: List[RangeS],
    callLocation: LocationInDenizen,
      life: LocationInFunctionEnvironmentT,
    contextRegion: RegionT,
      undecayedUnborrowedContainerExpr2: ExpressionT):
  (ReferenceExpressionTE) = {
    undecayedUnborrowedContainerExpr2 match {
      case a: AddressExpressionTE => {
        (localHelper.borrowSoftLoad(coutputs, a))
      }
      case r: ReferenceExpressionTE => {
        val unborrowedContainerExpr2 = r// decaySoloPack(nenv, life + 0, r)
        unborrowedContainerExpr2.result.coord.ownership match {
          case OwnT => {
            localHelper.makeTemporaryLocal(
              coutputs,
              nenv,
              range,
              callLocation,
              life + 1,
              contextRegion,
              unborrowedContainerExpr2,
              BorrowT)
          }
          case BorrowT | ShareT => (unborrowedContainerExpr2)
        }
      }
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_closure(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        name: IFunctionDeclarationNameS<'s>,
        function_s: &'s FunctionS<'s>,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  // Given a function1, this will give a closure (an OrdinaryClosure2 or a TemplatedClosure2)
  // returns:
  // - coutputs
  // - resulting templata
  // - exported things (from let)
  // - hoistees; expressions to hoist (like initializing blocks)
  def evaluateClosure(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    name: IFunctionDeclarationNameS,
    functionS: FunctionS):
  (ReferenceExpressionTE) = {

    val functionA = astronomizeLambda(coutputs, nenv, parentRanges, functionS)

    val closurestructTT =
      delegate.evaluateClosureStruct(coutputs, nenv.snapshot, parentRanges, callLocation, name, functionA);
    val closureCoord =
      templataCompiler.pointifyKind(coutputs, closurestructTT, region, OwnT)

    val constructExpr2 =
      makeClosureStructConstructExpression(
        coutputs, nenv, functionA.range :: parentRanges, region, closurestructTT)
    vassert(constructExpr2.result.coord == closureCoord)
    // The result of a constructor is always an own or a share.

    // The below code was here, but i see no reason we need to put it in a temporary and point it out.
    // shouldnt this be done automatically if we try to call the function which accepts a borrow?
//    val closureVarId = FullName2(nenv.lambdaNumber, "__closure_" + function1.origin.lambdaNumber)
//    val closureLocalVar = ReferenceLocalVariable2(closureVarId, Final, resultExpr2.resultRegister.reference)
//    val letExpr2 = LetAndPoint2(closureLocalVar, resultExpr2)
//    val unlet2 = localHelper.unletLocal(nenv, closureLocalVar)
//    val dropExpr =
//      DestructorCompiler.drop(env, coutputs, nenv, unlet2)
//    val deferExpr2 = Defer2(letExpr2, dropExpr)
//    (coutputs, nenv, deferExpr2)

    constructExpr2
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn new_global_function_group_expression(
        &self,
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        region: RegionT,
        name: IImpreciseNameS<'s>,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  private def newGlobalFunctionGroupExpression(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    region: RegionT,
    name: IImpreciseNameS
  ):
  ReferenceExpressionTE = {
    ReinterpretTE(
      VoidLiteralTE(region),
      CoordT(
        ShareT,
        region,
        interner.intern(OverloadSetT(env, name))))
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_block_statements(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        starting_nenv: &'t NodeEnvironmentT<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        block: &'s BlockSE<'s>,
    ) -> (&'t ReferenceExpressionTE<'s, 't>, HashSet<CoordT<'s, 't>>) {
        self.evaluate_block_statements_block(
            coutputs, starting_nenv, nenv, parent_ranges, call_location,
            life, region, block)
    }
/*
  def evaluateBlockStatements(
    coutputs: CompilerOutputs,
    startingNenv: NodeEnvironmentT,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    block: BlockSE):
  (ReferenceExpressionTE, Set[CoordT]) = {
    blockCompiler.evaluateBlockStatements(
      coutputs, startingNenv, nenv, parentRanges, callLocation, life, region, block)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_pattern_list(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        patterns_1: &[&'s AtomSP<'s>],
        pattern_input_exprs_2: &[&'t ReferenceExpressionTE<'s, 't>],
        region: RegionT,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        self.translate_pattern_list_pattern(
            coutputs, nenv, life, parent_ranges, call_location,
            patterns_1, pattern_input_exprs_2, region,
            |_coutputs, nenv, _live_capture_locals| {
                self.typing_interner.alloc(ReferenceExpressionTE::VoidLiteral(VoidLiteralTE {
                    region: nenv.default_region,
                    _phantom: std::marker::PhantomData,
                }))
            })
    }
/*
  def translatePatternList(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    patterns1: Vector[AtomSP],
    patternInputExprs2: Vector[ReferenceExpressionTE],
    region: RegionT
  ): ReferenceExpressionTE = {
    patternCompiler.translatePatternList(
      coutputs, nenv, life, parentRanges, callLocation, patterns1, patternInputExprs2, region,
      (coutputs, nenv, liveCaptureLocals) => VoidLiteralTE(nenv.defaultRegion))
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn astronomize_lambda(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        function_s: &'s FunctionS<'s>,
    ) -> &'s FunctionA<'s> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def astronomizeLambda(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    parentRanges: List[RangeS],
    functionS: FunctionS):
  FunctionA = {
    val FunctionS(rangeS, nameS, attributesS, identifyingRunesS, runeToExplicitType, tyype, paramsS, maybeRetCoordRune, rulesWithImplicitlyCoercingLookupsS, bodyS) = functionS

    def lookupType(x: IImpreciseNameS) = {
      x match {
        // This is here because if we tried to look up this lambda struct, it wouldn't exist yet.
        // It's not an insurmountable problem, it will exist slightly later when we're inside StructCompiler,
        // but this workaround makes for a cleaner separation between FunctionCompiler and StructCompiler
        // at least for now.
        // If this proves irksome, consider rearranging FunctionCompiler and StructCompiler's steps in
        // evaluating lambdas.
        case LambdaStructImpreciseNameS(_) => {
          // Lambdas look up their struct as a KindTemplata in their environment, they dont look up
          // the origin template by name. Not sure why.
          KindTemplataType()
        }
        case n => {
          vassertSome(nenv.lookupNearestWithImpreciseName(n, Set(TemplataLookupContext))).tyype
        }
      }
    }

    val runeSToPreKnownTypeA =
      runeToExplicitType ++
        paramsS.map(_.pattern.coordRune.get.rune -> CoordTemplataType()).toMap

    val runeTypeSolveEnv = TemplataCompiler.createRuneTypeSolverEnv(nenv.snapshot)

    val runeAToTypeWithImplicitlyCoercingLookupsS =
      new RuneTypeSolver(interner).solve(
        opts.globalOptions.sanityCheck,
        opts.globalOptions.useOptimizedSolver,
        runeTypeSolveEnv,
        rangeS :: parentRanges,
        false, rulesWithImplicitlyCoercingLookupsS, identifyingRunesS.map(_.rune.rune), true, runeSToPreKnownTypeA) match {
        case Ok(t) => t
        case Err(e) => throw CompileErrorExceptionT(CouldntSolveRuneTypesT(rangeS :: parentRanges, e))
      }

    val runeAToType =
      mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
    // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
    // loose. We intentionally ignored the types of the things they're looking up, so we could know
    // what types we *expect* them to be, so we could coerce.
    // That coercion is good, but lets make it more explicit.
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(runeTypeSolveEnv, runeAToType, ruleBuilder, rulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionT(TooManyTypesWithNameT(range :: parentRanges, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionT(CouldntFindTypeT(range :: parentRanges, name))
      case Ok(()) =>
    }

    vale.highertyping.FunctionA(
      rangeS,
      nameS,
      attributesS ++ Vector(UserFunctionS),
      tyype,
      identifyingRunesS,
      runeAToType.toMap,
      paramsS,
      maybeRetCoordRune,
      ruleBuilder.toVector,
      bodyS)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn drop_since(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        starting_nenv: &'t NodeEnvironmentT<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        life: LocationInFunctionEnvironmentT<'s>,
        region: RegionT,
        expr_te: &'t ReferenceExpressionTE<'s, 't>,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        let snapshot = nenv.snapshot(self.typing_interner);
        let unreversed_variables_to_destruct =
            snapshot.get_live_variables_introduced_since(starting_nenv);

        if unreversed_variables_to_destruct.is_empty() {
            expr_te
        } else {
            panic!("implement: drop_since — non-empty variables to destruct");
        }
    }
/*
  def dropSince(
    coutputs: CompilerOutputs,
    startingNenv: NodeEnvironmentT,
    nenv: NodeEnvironmentBox,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    life: LocationInFunctionEnvironmentT,
    region: RegionT,
    exprTE: ReferenceExpressionTE):
  ReferenceExpressionTE = {
    val unreversedVariablesToDestruct = nenv.snapshot.getLiveVariablesIntroducedSince(startingNenv)

    val newExpr =
      if (unreversedVariablesToDestruct.isEmpty) {
        exprTE
      } else {
        exprTE.kind match {
          case VoidT() => {
            val reversedVariablesToDestruct = unreversedVariablesToDestruct.reverse
            // Dealiasing should be done by hammer. But destructors are done here
            val destroyExpressions =
              localHelper.unletAndDropAll(
                coutputs, nenv, range, callLocation, region, reversedVariablesToDestruct)

            Compiler.consecutive(
              (Vector(exprTE) ++ destroyExpressions) :+
                VoidLiteralTE(region))
          }
          case NeverT(_) => {
            // In this case, we want to not drop them, so we can support things like:
            //   func drop(self Server) {
            //     panic("unreachable");
            //   }
            // and not drop Server.

            val reversedVariablesToDestruct = unreversedVariablesToDestruct.reverse
            val destroyExpressions = localHelper.unletAllWithoutDropping(coutputs, nenv, range, reversedVariablesToDestruct)
            // Just dont add in the destroyExpressions, let em go.
            // We did the above simply to mark them as unstackified.
            exprTE
          }
          case _ => {
            val (resultifiedExpr, resultLocalVariable) =
              resultifyExpressions(nenv, life + 1, exprTE)

            val reversedVariablesToDestruct = unreversedVariablesToDestruct.reverse
            // Dealiasing should be done by hammer. But destructors are done here
            val destroyExpressions =
              localHelper.unletAndDropAll(
              coutputs,
              nenv,
              range,
              callLocation,
              region,
              reversedVariablesToDestruct)

            Compiler.consecutive(
              (Vector(resultifiedExpr) ++ destroyExpressions) :+
                localHelper.unletLocalWithoutDropping(nenv, resultLocalVariable))
          }
        }
      }
    newExpr
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resultify_expressions(
        &self,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        expr: &'t ReferenceExpressionTE<'s, 't>,
    ) -> (&'t ReferenceExpressionTE<'s, 't>, ReferenceLocalVariableT<'s, 't>) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  // Makes the last expression stored in a variable.
  // Dont call this for void or never or no expressions.
  // Maybe someday we can do this even for Never and Void, for consistency and so
  // we dont have any special casing.
  def resultifyExpressions(
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    expr: ReferenceExpressionTE):
  (ReferenceExpressionTE, ReferenceLocalVariableT) = {
    val resultVarId = interner.intern(TypingPassBlockResultVarNameT(life))
    val resultVariable = ReferenceLocalVariableT(resultVarId, FinalT, expr.result.coord)
    val resultLet = LetNormalTE(resultVariable, expr)
    nenv.addVariable(resultVariable)
    (resultLet, resultVariable)
  }

//  def mootAll(
//    coutputs: CompilerOutputs,
//    nenv: NodeEnvironmentBox,
//    variables: Vector[ILocalVariableT]):
//  (Vector[ReferenceExpressionTE]) = {
//    variables.map({ case head =>
//      ast.UnreachableMootTE(localHelper.unletLocal(nenv, head))
//    })
//  }
}
*/
}

// Concrete IRuneTypeSolverEnv for the LetSE arm of evaluate. The Scala anonymous
// `new IRuneTypeSolverEnv` at ExpressionCompiler.scala:959 closes over `nenv` and
// delegates to lookupNearestWithImpreciseName. This struct captures that field.
// Same shape as `HigherTypingRuneTypeSolverEnv` in higher_typing_pass.rs (which
// collapses 6 anonymous Scala impls into one named struct).
struct LetExprRuneTypeSolverEnv<'a, 's, 't>
where
    's: 't,
{
    nenv: &'a crate::typing::env::function_environment_t::NodeEnvironmentBox<'s, 't>,
}
/*
Guardian: disable-all
*/

impl<'a, 's, 't> crate::postparsing::rune_type_solver::IRuneTypeSolverEnv<'s>
    for LetExprRuneTypeSolverEnv<'a, 's, 't>
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
        let mut filter = std::collections::HashSet::new();
        filter.insert(crate::typing::env::environment::ILookupContext::TemplataLookupContext);
        match self.nenv.lookup_nearest_with_imprecise_name(name_s, &filter) {
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
                // Scala: `case Some(x) => Ok(TemplataLookupResult(x.tyype))`.
                // Requires `ITemplataT::tyype()` getter — separate scaffolding gap.
                panic!("LetExprRuneTypeSolverEnv: ITemplataT::tyype() not yet implemented");
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
/*
Guardian: disable-all
*/