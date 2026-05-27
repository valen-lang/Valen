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
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct LetSE<'s> {
  pub range: RangeS<'s>,
  pub rules: &'s [IRulexSR<'s>],
  pub pattern: AtomSP<'s>,
  pub expr: &'s IExpressionSE<'s>,
}
#[derive(Debug, PartialEq)]
pub struct IfSE<'s> {
  pub range: RangeS<'s>,
  pub condition: &'s IExpressionSE<'s>,
  pub then_body: &'s BlockSE<'s>,
  pub else_body: &'s BlockSE<'s>,
}
/*
case class IfSE(
  range: RangeS,
  condition: IExpressionSE,
  thenBody: BlockSE,
  elseBody: BlockSE
) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

  vcurious(!condition.isInstanceOf[BlockSE])
}
*/
#[derive(Debug, PartialEq)]
pub struct LoopSE<'s> {
  pub range: RangeS<'s>,
  pub body: &'s BlockSE<'s>,
}
/*
case class LoopSE(range: RangeS, body: BlockSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Debug, PartialEq)]
pub struct BreakSE<'s> {
  pub range: RangeS<'s>,
}
/*
case class BreakSE(range: RangeS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct WhileSE<'s> {
  pub range: RangeS<'s>,
  pub body: &'s BlockSE<'s>,
}
/*
case class WhileSE(range: RangeS, body: BlockSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Debug, PartialEq)]
pub struct MapSE<'s> {
  pub range: RangeS<'s>,
  pub body: &'s BlockSE<'s>,
}
/*
case class MapSE(range: RangeS, body: BlockSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Debug, PartialEq)]
pub struct ExprMutateSE<'s> {
  pub range: RangeS<'s>,
  pub mutatee: &'s IExpressionSE<'s>,
  pub expr: &'s IExpressionSE<'s>,
}
/*
case class ExprMutateSE(range: RangeS, mutatee: IExpressionSE, expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct GlobalMutateSE<'s> {
  pub range: RangeS<'s>,
  pub name: CodeNameS<'s>,
  pub expr: &'s IExpressionSE<'s>,
}
/*
case class GlobalMutateSE(range: RangeS, name: CodeNameS, expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct LocalMutateSE<'s> {
  pub range: RangeS<'s>,
  pub name: IVarNameS<'s>,
  pub expr: &'s IExpressionSE<'s>,
}
/*
case class LocalMutateSE(range: RangeS, name: IVarNameS, expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct OwnershippedSE<'s> {
  pub range: RangeS<'s>,
  pub inner_expr: &'s IExpressionSE<'s>,
  pub target_ownership: LoadAsP,
}
/*
case class OwnershippedSE(range: RangeS, innerExpr1: IExpressionSE, targetOwnership: LoadAsP) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocalS<'s> {
  pub var_name: IVarNameS<'s>,
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
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct BodySE<'s> {
  pub range: RangeS<'s>,
  pub closured_names: &'s [IVarNameS<'s>],
  pub block: &'s BlockSE<'s>,
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
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Debug, PartialEq)]
pub struct PureSE<'s> {
  pub range: RangeS<'s>,
  pub location: LocationInDenizen<'s>,
  pub inner: &'s IExpressionSE<'s>,
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
#[derive(Debug, PartialEq)]
pub struct BlockSE<'s> {
  pub range: RangeS<'s>,
  pub locals: &'s [LocalS<'s>],
  pub expr: &'s IExpressionSE<'s>,
}

/*
case class BlockSE(
  range: RangeS,
  locals: Vector[LocalS],
  expr: IExpressionSE,
) extends IExpressionSE {

  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

  vassert(locals.map(_.varName) == locals.map(_.varName).distinct)
//  expr match {
//    case BlockSE(range, locals, expr) => vcurious()
//    case _ =>
//  }
}
*/
#[derive(Debug, PartialEq)]
pub enum IExpressionSE<'s> {
  Let(LetSE<'s>),
  If(IfSE<'s>),
  Loop(LoopSE<'s>),
  Break(BreakSE<'s>),
  While(WhileSE<'s>),
  Map(MapSE<'s>),
  ExprMutate(ExprMutateSE<'s>),
  GlobalMutate(GlobalMutateSE<'s>),
  LocalMutate(LocalMutateSE<'s>),
  Consecutor(ConsecutorSE<'s>),
  ArgLookup(ArgLookupSE<'s>),
  RepeaterBlock(RepeaterBlockSE<'s>),
  RepeaterBlockIterator(RepeaterBlockIteratorSE<'s>),
  Void(VoidSE<'s>),
  Tuple(TupleSE<'s>),
  StaticArrayFromValues(StaticArrayFromValuesSE<'s>),
  StaticArrayFromCallable(StaticArrayFromCallableSE<'s>),
  NewRuntimeSizedArray(NewRuntimeSizedArraySE<'s>),
  RepeaterPack(RepeaterPackSE<'s>),
  RepeaterPackIterator(RepeaterPackIteratorSE<'s>),
  Block(&'s BlockSE<'s>),
  Pure(PureSE<'s>),
  Return(ReturnSE<'s>),
  ConstantInt(ConstantIntSE<'s>),
  ConstantBool(ConstantBoolSE<'s>),
  ConstantStr(ConstantStrSE<'s>),
  ConstantFloat(ConstantFloatSE<'s>),
  Destruct(DestructSE<'s>),
  Unlet(UnletSE<'s>),
  Function(FunctionSE<'s>),
  Dot(DotSE<'s>),
  Index(IndexSE<'s>),
  FunctionCall(FunctionCallSE<'s>),
  LocalLoad(LocalLoadSE<'s>),
  OverloadSet(OverloadSetSE<'s>),
  TemplataLoad(TemplataLoadSE<'s>),
  RuneLookup(RuneLookupSE<'s>),
  Ownershipped(OwnershippedSE<'s>),
}

impl<'s> IExpressionSETrait<'s> for IExpressionSE<'s> {
  fn range(&self) -> RangeS<'s> {
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
      IExpressionSE::OverloadSet(x) => x.lookup.range.clone(),
      IExpressionSE::TemplataLoad(x) => x.range.clone(),
      IExpressionSE::RuneLookup(x) => x.range.clone(),
      IExpressionSE::Ownershipped(x) => x.range.clone(),
    }
  }
  /* Guardian: disable-all */
}

#[derive(Debug, PartialEq)]
pub struct ConsecutorSE<'s> {
  pub exprs: &'s [&'s IExpressionSE<'s>],
}
/*
case class ConsecutorSE(
  exprs: Vector[IExpressionSE],
) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/

impl<'s> ConsecutorSE<'s> {
  pub fn range(&self) -> RangeS<'s> {
    assert!(!self.exprs.is_empty());
    RangeS::new(
      self.exprs.first().unwrap().range().begin,
      self.exprs.last().unwrap().range().end,
    )
  }
/*
override def range: RangeS = RangeS(exprs.head.range.begin, exprs.last.range.end)
*/
/*
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
}
/* Guardian: disable-all */

#[derive(Debug, PartialEq)]
pub struct ArgLookupSE<'s> {
  pub range: RangeS<'s>,
  pub index: i32,
}
/*
case class ArgLookupSE(range: RangeS, index: Int) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct RepeaterBlockSE<'s> {
  pub range: RangeS<'s>,
  pub expression: &'s IExpressionSE<'s>,
}
/*
 // These things will be separated by semicolons, and all be joined in a block
case class RepeaterBlockSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
 }
*/
#[derive(Debug, PartialEq)]
pub struct RepeaterBlockIteratorSE<'s> {
  pub range: RangeS<'s>,
  pub expression: &'s IExpressionSE<'s>,
}
/*
// Results in a pack, represents the differences between the expressions
case class RepeaterBlockIteratorSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct ReturnSE<'s> {
  pub range: RangeS<'s>,
  pub inner: &'s IExpressionSE<'s>,
}
/*
case class ReturnSE(range: RangeS, inner: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  inner match {
    case ReturnSE(_, _) => vwat()
    case _ =>
  }
}
*/
#[derive(Debug, PartialEq)]
pub struct VoidSE<'s> {
  pub range: RangeS<'s>,
}
/*
case class VoidSE(range: RangeS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct TupleSE<'s> {
  pub range: RangeS<'s>,
  pub elements: &'s [&'s IExpressionSE<'s>],
}
/*
case class TupleSE(range: RangeS, elements: Vector[IExpressionSE]) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct StaticArrayFromValuesSE<'s> {
  pub range: RangeS<'s>,
  pub rules: &'s [IRulexSR<'s>],
  pub maybe_element_type_st: Option<RuneUsage<'s>>,
  pub mutability_st: RuneUsage<'s>,
  pub variability_st: RuneUsage<'s>,
  pub size_st: RuneUsage<'s>,
  pub elements: &'s [&'s IExpressionSE<'s>],
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
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct StaticArrayFromCallableSE<'s> {
  pub range: RangeS<'s>,
  pub rules: &'s [IRulexSR<'s>],
  pub maybe_element_type_st: Option<RuneUsage<'s>>,
  pub mutability_st: RuneUsage<'s>,
  pub variability_st: RuneUsage<'s>,
  pub size_st: RuneUsage<'s>,
  pub callable: &'s IExpressionSE<'s>,
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
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct NewRuntimeSizedArraySE<'s> {
  pub range: RangeS<'s>,
  pub rules: &'s [IRulexSR<'s>],
  pub maybe_element_type_st: Option<RuneUsage<'s>>,
  pub mutability_st: RuneUsage<'s>,
  pub size: &'s IExpressionSE<'s>,
  pub callable: Option<&'s IExpressionSE<'s>>,
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
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct RepeaterPackSE<'s> {
  pub range: RangeS<'s>,
  pub expression: &'s IExpressionSE<'s>,
}
/*
// This thing will be repeated, separated by commas, and all be joined in a pack
case class RepeaterPackSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct RepeaterPackIteratorSE<'s> {
  pub range: RangeS<'s>,
  pub expression: &'s IExpressionSE<'s>,
}
/*
// Results in a pack, represents the differences between the elements
case class RepeaterPackIteratorSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class ConstantIntSE(range: RangeS, value: Long, bits: Int) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct ConstantIntSE<'s> {
  pub range: RangeS<'s>,
  pub value: i64,
  pub bits: i32,
}
#[derive(Debug, PartialEq)]
pub struct ConstantBoolSE<'s> {
  pub range: RangeS<'s>,
  pub value: bool,
}
/*
case class ConstantBoolSE(range: RangeS, value: Boolean) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct ConstantStrSE<'s> {
  pub range: RangeS<'s>,
  pub value: StrI<'s>,
}
/*
case class ConstantStrSE(range: RangeS, value: String) extends IExpressionSE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct ConstantFloatSE<'s> {
  pub range: RangeS<'s>,
  pub value: f64,
}
/*
case class ConstantFloatSE(range: RangeS, value: Double) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct DestructSE<'s> {
  pub range: RangeS<'s>,
  pub inner: &'s IExpressionSE<'s>,
}
/*
case class DestructSE(range: RangeS, inner: IExpressionSE) extends IExpressionSE {
  vpass()
}
*/
#[derive(Debug, PartialEq)]
pub struct UnletSE<'s> {
  pub range: RangeS<'s>,
  pub name: IVarNameS<'s>,
}
/*
case class UnletSE(range: RangeS, name: IVarNameS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct FunctionSE<'s> {
  pub function: &'s FunctionS<'s>,
}
/*
case class FunctionSE(function: FunctionS) extends IExpressionSE {
  override def range: RangeS = function.range
}
*/
#[derive(Debug, PartialEq)]
pub struct DotSE<'s> {
  pub range: RangeS<'s>,
  pub left: &'s IExpressionSE<'s>,
  pub member: StrI<'s>,
  pub borrow_container: bool,
}
/*
case class DotSE(range: RangeS, left: IExpressionSE, member: StrI, borrowContainer: Boolean) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct IndexSE<'s> {
  pub range: RangeS<'s>,
  pub left: &'s IExpressionSE<'s>,
  pub index_expr: &'s IExpressionSE<'s>,
}
/*
case class IndexSE(range: RangeS, left: IExpressionSE, indexExpr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class FunctionCallSE(range: RangeS, location: LocationInDenizen, callableExpr: IExpressionSE, argsExprs1: Vector[IExpressionSE]) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct FunctionCallSE<'s> {
  pub range: RangeS<'s>,
  pub location: LocationInDenizen<'s>,
  pub callable_expr: &'s IExpressionSE<'s>,
  pub arg_exprs: &'s [&'s IExpressionSE<'s>],
}
/*

case class LocalLoadSE(range: RangeS, name: IVarNameS, targetOwnership: LoadAsP) extends IExpressionSE {
  vpass()
}
*/
#[derive(Debug, PartialEq)]
pub struct LocalLoadSE<'s> {
  pub range: RangeS<'s>,
  pub name: IVarNameS<'s>,
  pub target_ownership: LoadAsP,
}
// One step in a OutsideLoadSE. See OutsideLoadSE comments.
#[derive(Debug, PartialEq)]
pub struct LoadPartSE<'s> {
  pub name: IImpreciseNameS<'s>,
  pub explicit_template_args: &'s [RuneUsage<'s>],
}
/*
// One step in a OutsideLoadSE. See OutsideLoadSE comments.
case class LoadPartSE(
  name: IImpreciseNameS,
  explicitTemplateArgs: Vector[RuneUsage]
) {
  vpass()
}
*/

// A load from something that lives outside the current definition.
// For example:
//     v = Vec<int>.with_capacity(42)
// would have a OutsideLoadSE for the `Vec<int>.with_capacity` part.
// It would look like this:
// - parts: [LoadPartSE("Vec", [$0]), LoadPartSE("with_capacity", [])]
// - rules: [$0 = LookupSR("int")]
// Per @PRIIROZ, we add containers' generic params *after* the function's generic params.
// Example:
//     number_to_corresponding_string = HashMap<int, str>.create_and_fill(64, 42, i => to_string(i))
// Would look like this:
// - parts: [LoadPartSE("HashMap", [$0, $1]), LoadPartSE("create_and_fill", [$2])]
// - rules: [$0 = "int", $1 = "str", $2 = main:lambda:1]
//
// This is only used by OverloadSetSE so far, but someday it could be used for looking up associated aliases on structs
// or something.
#[derive(Debug, PartialEq)]
pub struct OutsideLoadSE<'s> {
  pub range: RangeS<'s>,
  pub rules: &'s [IRulexSR<'s>],
  // parts' explicitArgs are runes that refer to the above rules.
  pub parts: &'s [&'s LoadPartSE<'s>],
}
/*
// A load from something that lives outside the current definition.
// For example:
//     v = Vec<int>.with_capacity(42)
// would have a OutsideLoadSE for the `Vec<int>.with_capacity` part.
// It would look like this:
// - parts: [LoadPartSE("Vec", [$0]), LoadPartSE("with_capacity", [])]
// - rules: [$0 = LookupSR("int")]
// Per @PRIIROZ, we add containers' generic params *after* the function's generic params.
// Example:
//     number_to_corresponding_string = HashMap<int, str>.create_and_fill(64, 42, i => to_string(i))
// Would look like this:
// - parts: [LoadPartSE("HashMap", [$0, $1]), LoadPartSE("create_and_fill", [$2])]
// - rules: [$0 = "int", $1 = "str", $2 = main:lambda:1]
//
// This is only used by OverloadSetSE so far, but someday it could be used for looking up associated aliases on structs
// or something.
case class OutsideLoadSE(
  range: RangeS,
  rules: Vector[IRulexSR],
  // parts' explicitArgs are runes that refer to the above rules.
  parts: Vector[LoadPartSE]
) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/

#[derive(Debug, PartialEq)]
pub struct OverloadSetSE<'s> {
  pub lookup: OutsideLoadSE<'s>,
}
/*
case class OverloadSetSE(
    lookup: OutsideLoadSE
) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
  override def hashCode(): Int = vcurious()
  override def range = lookup.range
}
*/

#[derive(Debug, PartialEq)]
pub struct TemplataLoadSE<'s> {
  pub range: RangeS<'s>,
  pub rules: &'s [IRulexSR<'s>],
  pub target_ownership: LoadAsP,
}
/*
case class TemplataLoadSE(
    range: RangeS,
    rules: Vector[IRulexSR],
    targetOwnership: LoadAsP
)  extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
  override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Debug, PartialEq)]
pub struct RuneLookupSE<'s> {
  pub range: RangeS<'s>,
  pub rune: IRuneS<'s>,
}
/*
case class RuneLookupSE(range: RangeS, rune: IRuneS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/