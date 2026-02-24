use crate::interner::StrI;
use crate::postparsing::ast::{FunctionS, IExpressionSE as IExpressionSETrait, LocationInDenizen};
use crate::postparsing::names::{CodeNameS, IImpreciseNameS, IRuneS, IVarNameS};
use crate::postparsing::patterns::AtomSP;
use crate::postparsing::rules::{IRulexSR, RuneUsage};
use crate::parsing::ast::LoadAsP;
use crate::utils::range::RangeS;

/*
package dev.vale.postparsing

import dev.vale.parsing.ast.{LoadAsBorrowP, LoadAsP, LoadAsWeakP, MoveP}
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.postparsing.rules.{IRulexSR, RuneUsage}
import dev.vale.{RangeS, StrI, vassert, vcurious, vpass, vwat}
import dev.vale.parsing.ast._
import dev.vale.postparsing.rules.ILiteralSL

*/
/*
// patternId is a unique number, can be used to make temporary variables that wont
// collide with other things
case class LetSE(
    range: RangeS,
    rules: Vector[IRulexSR],
    pattern: AtomSP,
    expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct LetSE<'a, 's> {
  pub range: RangeS<'a>,
  pub rules: Vec<IRulexSR<'a>>,
  pub pattern: AtomSP<'a>,
  pub expr: &'s IExpressionSE<'a, 's>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IfSE<'a, 's> {
  pub range: RangeS<'a>,
  pub condition: &'s IExpressionSE<'a, 's>,
  pub then_body: &'s BlockSE<'a, 's>,
  pub else_body: &'s BlockSE<'a, 's>,
}
/*
case class IfSE(
  range: RangeS,
  condition: IExpressionSE,
  thenBody: BlockSE,
  elseBody: BlockSE
) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()

  vcurious(!condition.isInstanceOf[BlockSE])
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct LoopSE<'a, 's> {
  pub range: RangeS<'a>,
  pub body: &'s BlockSE<'a, 's>,
}
/*
case class LoopSE(range: RangeS, body: BlockSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct BreakSE<'a> {
  pub range: RangeS<'a>,
}
/*
case class BreakSE(range: RangeS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct WhileSE<'a, 's> {
  pub range: RangeS<'a>,
  pub body: &'s BlockSE<'a, 's>,
}
/*
case class WhileSE(range: RangeS, body: BlockSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct MapSE<'a, 's> {
  pub range: RangeS<'a>,
  pub body: &'s BlockSE<'a, 's>,
}
/*
case class MapSE(range: RangeS, body: BlockSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ExprMutateSE<'a, 's> {
  pub range: RangeS<'a>,
  pub mutatee: &'s IExpressionSE<'a, 's>,
  pub expr: &'s IExpressionSE<'a, 's>,
}
/*
case class ExprMutateSE(range: RangeS, mutatee: IExpressionSE, expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalMutateSE<'a, 's> {
  pub range: RangeS<'a>,
  pub name: CodeNameS<'a>,
  pub expr: &'s IExpressionSE<'a, 's>,
}
/*
case class GlobalMutateSE(range: RangeS, name: CodeNameS, expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct LocalMutateSE<'a, 's> {
  pub range: RangeS<'a>,
  pub name: IVarNameS<'a>,
  pub expr: &'s IExpressionSE<'a, 's>,
}
/*
case class LocalMutateSE(range: RangeS, name: IVarNameS, expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct OwnershippedSE<'a, 's> {
  pub range: RangeS<'a>,
  pub inner_expr: &'s IExpressionSE<'a, 's>,
  pub target_ownership: LoadAsP,
}
/*
case class OwnershippedSE(range: RangeS, innerExpr1: IExpressionSE, targetOwnership: LoadAsP) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()

  targetOwnership match {
    case LoadAsBorrowP =>
    case LoadAsWeakP =>
    case MoveP =>
  }
}
*/
/*
// when we make a closure, we make a struct full of pointers to all our variables
// and the first element is our parent closure
// this can live on the stack, since blocks are additive to this expression
// later we can optimize it to only have the things we use

*/

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IVariableUseCertainty {
  Used,
  NotUsed,
}

/*
sealed trait IVariableUseCertainty
case object Used extends IVariableUseCertainty
case object NotUsed extends IVariableUseCertainty
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocalS<'a> {
  pub var_name: IVarNameS<'a>,
  pub self_borrowed: IVariableUseCertainty,
  pub self_moved: IVariableUseCertainty,
  pub self_mutated: IVariableUseCertainty,
  pub child_borrowed: IVariableUseCertainty,
  pub child_moved: IVariableUseCertainty,
  pub child_mutated: IVariableUseCertainty,
}

/*
case class LocalS(
    varName: IVarNameS,
    selfBorrowed: IVariableUseCertainty,
    selfMoved: IVariableUseCertainty,
    selfMutated: IVariableUseCertainty,
    childBorrowed: IVariableUseCertainty,
    childMoved: IVariableUseCertainty,
    childMutated: IVariableUseCertainty) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct BodySE<'a, 's> {
  pub range: RangeS<'a>,
  pub closured_names: Vec<IVarNameS<'a>>,
  pub block: &'s BlockSE<'a, 's>,
}

/*
case class BodySE(
    range: RangeS,
    // These are all the variables we use from parent environments.
    // We have these so typingpass doesn't have to dive through all the functions
    // that it calls (impossible) to figure out what's needed in a closure struct.
    closuredNames: Vector[IVarNameS],

    block: BlockSE
) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct PureSE<'a, 's> {
  pub range: RangeS<'a>,
  pub location: LocationInDenizen,
  pub inner: &'s IExpressionSE<'a, 's>,
}

/*
case class PureSE(
  range: RangeS,
  location: LocationInDenizen,
  inner: IExpressionSE
) extends IExpressionSE {
  inner match {
    case BlockSE(range, locals, expr) =>
    case other => vwat() // Pures always contain blocks, see PSBOB.
  }
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct BlockSE<'a, 's> {
  pub range: RangeS<'a>,
  pub locals: Vec<LocalS<'a>>,
  pub expr: &'s IExpressionSE<'a, 's>,
}

/*
case class BlockSE(
  range: RangeS,
  locals: Vector[LocalS],
  expr: IExpressionSE,
) extends IExpressionSE {

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()

  vassert(locals.map(_.varName) == locals.map(_.varName).distinct)
//  expr match {
//    case BlockSE(range, locals, expr) => vcurious()
//    case _ =>
//  }
}
*/
#[derive(Clone, Debug, PartialEq)]
pub enum IExpressionSE<'a, 's> {
  Let(LetSE<'a, 's>),
  If(IfSE<'a, 's>),
  Loop(LoopSE<'a, 's>),
  Break(BreakSE<'a>),
  While(WhileSE<'a, 's>),
  Map(MapSE<'a, 's>),
  ExprMutate(ExprMutateSE<'a, 's>),
  GlobalMutate(GlobalMutateSE<'a, 's>),
  LocalMutate(LocalMutateSE<'a, 's>),
  Consecutor(ConsecutorSE<'a, 's>),
  ArgLookup(ArgLookupSE<'a>),
  RepeaterBlock(RepeaterBlockSE<'a, 's>),
  RepeaterBlockIterator(RepeaterBlockIteratorSE<'a, 's>),
  Void(VoidSE<'a>),
  Tuple(TupleSE<'a, 's>),
  StaticArrayFromValues(StaticArrayFromValuesSE<'a, 's>),
  StaticArrayFromCallable(StaticArrayFromCallableSE<'a, 's>),
  NewRuntimeSizedArray(NewRuntimeSizedArraySE<'a, 's>),
  RepeaterPack(RepeaterPackSE<'a, 's>),
  RepeaterPackIterator(RepeaterPackIteratorSE<'a, 's>),
  Block(&'s BlockSE<'a, 's>),
  Pure(PureSE<'a, 's>),
  Return(ReturnSE<'a, 's>),
  ConstantInt(ConstantIntSE<'a>),
  ConstantBool(ConstantBoolSE<'a>),
  ConstantStr(ConstantStrSE<'a>),
  ConstantFloat(ConstantFloatSE<'a>),
  Destruct(DestructSE<'a, 's>),
  Unlet(UnletSE<'a>),
  Function(FunctionSE<'a, 's>),
  Dot(DotSE<'a, 's>),
  Index(IndexSE<'a, 's>),
  FunctionCall(FunctionCallSE<'a, 's>),
  LocalLoad(LocalLoadSE<'a>),
  OutsideLoad(OutsideLoadSE<'a>),
  RuneLookup(RuneLookupSE<'a>),
  Ownershipped(OwnershippedSE<'a, 's>),
}

impl<'a, 's> IExpressionSETrait<'a> for IExpressionSE<'a, 's> {
  fn range(&self) -> RangeS<'a> {
    match self {
      IExpressionSE::Let(x) => x.range.clone(),
      IExpressionSE::If(x) => x.range.clone(),
      IExpressionSE::Loop(x) => x.range.clone(),
      IExpressionSE::Break(x) => x.range.clone(),
      IExpressionSE::While(x) => x.range.clone(),
      IExpressionSE::Map(x) => x.range.clone(),
      IExpressionSE::ExprMutate(x) => x.range.clone(),
      IExpressionSE::GlobalMutate(x) => x.range.clone(),
      IExpressionSE::LocalMutate(x) => x.range.clone(),
      IExpressionSE::Consecutor(x) => x.range(),
      IExpressionSE::ArgLookup(x) => x.range.clone(),
      IExpressionSE::RepeaterBlock(x) => x.range.clone(),
      IExpressionSE::RepeaterBlockIterator(x) => x.range.clone(),
      IExpressionSE::Void(x) => x.range.clone(),
      IExpressionSE::Tuple(x) => x.range.clone(),
      IExpressionSE::StaticArrayFromValues(x) => x.range.clone(),
      IExpressionSE::StaticArrayFromCallable(x) => x.range.clone(),
      IExpressionSE::NewRuntimeSizedArray(x) => x.range.clone(),
      IExpressionSE::RepeaterPack(x) => x.range.clone(),
      IExpressionSE::RepeaterPackIterator(x) => x.range.clone(),
      IExpressionSE::Block(x) => x.range.clone(),
      IExpressionSE::Pure(x) => x.range.clone(),
      IExpressionSE::Return(x) => x.range.clone(),
      IExpressionSE::ConstantInt(x) => x.range.clone(),
      IExpressionSE::ConstantBool(x) => x.range.clone(),
      IExpressionSE::ConstantStr(x) => x.range.clone(),
      IExpressionSE::ConstantFloat(x) => x.range.clone(),
      IExpressionSE::Destruct(x) => x.range.clone(),
      IExpressionSE::Unlet(x) => x.range.clone(),
      IExpressionSE::Function(x) => x.function.range.clone(),
      IExpressionSE::Dot(x) => x.range.clone(),
      IExpressionSE::Index(x) => x.range.clone(),
      IExpressionSE::FunctionCall(x) => x.range.clone(),
      IExpressionSE::LocalLoad(x) => x.range.clone(),
      IExpressionSE::OutsideLoad(x) => x.range.clone(),
      IExpressionSE::RuneLookup(x) => x.range.clone(),
      IExpressionSE::Ownershipped(x) => x.range.clone(),
    }
  }
}

/*
case class ConsecutorSE(
  exprs: Vector[IExpressionSE],
) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()

  override def range: RangeS = RangeS(exprs.head.range.begin, exprs.last.range.end)

  // Should have at least one expression, because we'll
  // return the last expression's result as its result.
  vassert(exprs.size > 1)
  vassert(exprs.collect({ case ConsecutorSE(_) => }).isEmpty)


//  if (exprs.size >= 2) {
//    exprs.last match {
//      case VoidSE(_) => {
//        exprs.init.last match {
//          case ReturnSE(_, _) => vcurious()
//          case VoidSE(_) => vcurious()
//          case _ =>
//        }
//      }
//      case _ =>
//    }
//  }
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ConsecutorSE<'a, 's> {
  pub exprs: &'s [&'s IExpressionSE<'a, 's>],
}

impl<'a, 's> ConsecutorSE<'a, 's> {
  pub fn range(&self) -> RangeS<'a> {
    assert!(!self.exprs.is_empty());
    RangeS {
      begin: self.exprs.first().unwrap().range().begin,
      end: self.exprs.last().unwrap().range().end,
    }
  }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ArgLookupSE<'a> {
  pub range: RangeS<'a>,
  pub index: i32,
}
/*
case class ArgLookupSE(range: RangeS, index: Int) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RepeaterBlockSE<'a, 's> {
  pub range: RangeS<'a>,
  pub expression: &'s IExpressionSE<'a, 's>,
}
/*
 // These things will be separated by semicolons, and all be joined in a block
case class RepeaterBlockSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
 }
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RepeaterBlockIteratorSE<'a, 's> {
  pub range: RangeS<'a>,
  pub expression: &'s IExpressionSE<'a, 's>,
}
/*
// Results in a pack, represents the differences between the expressions
case class RepeaterBlockIteratorSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ReturnSE<'a, 's> {
  pub range: RangeS<'a>,
  pub inner: &'s IExpressionSE<'a, 's>,
}
/*
case class ReturnSE(range: RangeS, inner: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  inner match {
    case ReturnSE(_, _) => vwat()
    case _ =>
  }
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct VoidSE<'a> {
  pub range: RangeS<'a>,
}
/*
case class VoidSE(range: RangeS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct TupleSE<'a, 's> {
  pub range: RangeS<'a>,
  pub elements: &'s [&'s IExpressionSE<'a, 's>],
}
/*
case class TupleSE(range: RangeS, elements: Vector[IExpressionSE]) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct StaticArrayFromValuesSE<'a, 's> {
  pub range: RangeS<'a>,
  pub rules: Vec<IRulexSR<'a>>,
  pub maybe_element_type_st: Option<RuneUsage<'a>>,
  pub mutability_st: RuneUsage<'a>,
  pub variability_st: RuneUsage<'a>,
  pub size_st: RuneUsage<'a>,
  pub elements: &'s [&'s IExpressionSE<'a, 's>],
}
/*
case class StaticArrayFromValuesSE(
  range: RangeS,
  rules: Vector[IRulexSR],
  maybeElementTypeST: Option[RuneUsage],
  mutabilityST: RuneUsage,
  variabilityST: RuneUsage,
  sizeST: RuneUsage,
  elements: Vector[IExpressionSE]
) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct StaticArrayFromCallableSE<'a, 's> {
  pub range: RangeS<'a>,
  pub rules: Vec<IRulexSR<'a>>,
  pub maybe_element_type_st: Option<RuneUsage<'a>>,
  pub mutability_st: RuneUsage<'a>,
  pub variability_st: RuneUsage<'a>,
  pub size_st: RuneUsage<'a>,
  pub callable: &'s IExpressionSE<'a, 's>,
}
/*
case class StaticArrayFromCallableSE(
  range: RangeS,
  rules: Vector[IRulexSR],
  maybeElementTypeST: Option[RuneUsage],
  mutabilityST: RuneUsage,
  variabilityST: RuneUsage,
  sizeST: RuneUsage,
  callable: IExpressionSE
) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct NewRuntimeSizedArraySE<'a, 's> {
  pub range: RangeS<'a>,
  pub rules: Vec<IRulexSR<'a>>,
  pub maybe_element_type_st: Option<RuneUsage<'a>>,
  pub mutability_st: RuneUsage<'a>,
  pub size: &'s IExpressionSE<'a, 's>,
  pub callable: Option<&'s IExpressionSE<'a, 's>>,
}
/*
case class NewRuntimeSizedArraySE(
  range: RangeS,
  rules: Vector[IRulexSR],
  maybeElementTypeST: Option[RuneUsage],
  mutabilityST: RuneUsage,
  size: IExpressionSE,
  callable: Option[IExpressionSE]
) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RepeaterPackSE<'a, 's> {
  pub range: RangeS<'a>,
  pub expression: &'s IExpressionSE<'a, 's>,
}
/*
// This thing will be repeated, separated by commas, and all be joined in a pack
case class RepeaterPackSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RepeaterPackIteratorSE<'a, 's> {
  pub range: RangeS<'a>,
  pub expression: &'s IExpressionSE<'a, 's>,
}
/*
// Results in a pack, represents the differences between the elements
case class RepeaterPackIteratorSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
/*
case class ConstantIntSE(range: RangeS, value: Long, bits: Int) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantIntSE<'a> {
  pub range: RangeS<'a>,
  pub value: i64,
  pub bits: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantBoolSE<'a> {
  pub range: RangeS<'a>,
  pub value: bool,
}
/*
case class ConstantBoolSE(range: RangeS, value: Boolean) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantStrSE<'a> {
  pub range: RangeS<'a>,
  pub value: String,
}
/*
case class ConstantStrSE(range: RangeS, value: String) extends IExpressionSE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantFloatSE<'a> {
  pub range: RangeS<'a>,
  pub value: f64,
}
/*
case class ConstantFloatSE(range: RangeS, value: Double) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct DestructSE<'a, 's> {
  pub range: RangeS<'a>,
  pub inner: &'s IExpressionSE<'a, 's>,
}
/*
case class DestructSE(range: RangeS, inner: IExpressionSE) extends IExpressionSE {
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct UnletSE<'a> {
  pub range: RangeS<'a>,
  pub name: IVarNameS<'a>,
}
/*
case class UnletSE(range: RangeS, name: IVarNameS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSE<'a, 's> {
  pub function: &'s FunctionS<'a, 's>,
}
/*
case class FunctionSE(function: FunctionS) extends IExpressionSE {
  override def range: RangeS = function.range
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct DotSE<'a, 's> {
  pub range: RangeS<'a>,
  pub left: &'s IExpressionSE<'a, 's>,
  pub member: StrI<'a>,
  pub borrow_container: bool,
}
/*
case class DotSE(range: RangeS, left: IExpressionSE, member: StrI, borrowContainer: Boolean) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct IndexSE<'a, 's> {
  pub range: RangeS<'a>,
  pub left: &'s IExpressionSE<'a, 's>,
  pub index_expr: &'s IExpressionSE<'a, 's>,
}
/*
case class IndexSE(range: RangeS, left: IExpressionSE, indexExpr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
/*
case class FunctionCallSE(range: RangeS, location: LocationInDenizen, callableExpr: IExpressionSE, argsExprs1: Vector[IExpressionSE]) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCallSE<'a, 's> {
  pub range: RangeS<'a>,
  pub location: LocationInDenizen,
  pub callable_expr: &'s IExpressionSE<'a, 's>,
  pub arg_exprs: &'s [&'s IExpressionSE<'a, 's>],
}
/*

case class LocalLoadSE(range: RangeS, name: IVarNameS, targetOwnership: LoadAsP) extends IExpressionSE {
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct LocalLoadSE<'a> {
  pub range: RangeS<'a>,
  pub name: IVarNameS<'a>,
  pub target_ownership: LoadAsP,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OutsideLoadSE<'a> {
  pub range: RangeS<'a>,
  pub rules: Vec<IRulexSR<'a>>,
  pub name: IImpreciseNameS<'a>,
  pub maybe_template_args: Option<Vec<crate::postparsing::rules::RuneUsage<'a>>>,
  pub target_ownership: LoadAsP,
}
/*
// Loads a non-local. In well formed code, this will be a function, but the user also likely
// tried to access a variable they forgot to declare.
case class OutsideLoadSE(
  range: RangeS,
  rules: Vector[IRulexSR],
  name: IImpreciseNameS,
  maybeTemplateArgs: Option[Vector[RuneUsage]],
  targetOwnership: LoadAsP
) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RuneLookupSE<'a> {
  pub range: RangeS<'a>,
  pub rune: IRuneS<'a>,
}
/*
case class RuneLookupSE(range: RangeS, rune: IRuneS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/