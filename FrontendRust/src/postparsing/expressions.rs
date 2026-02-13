use crate::postparsing::ast::{FunctionS, LocationInDenizen};
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
pub struct LetSE {
  pub range: RangeS,
  pub rules: Vec<IRulexSR>,
  pub pattern: AtomSP,
  pub expr: Box<IExpressionSE>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IfSE {
  pub range: RangeS,
  pub condition: Box<IExpressionSE>,
  pub then_body: BlockSE,
  pub else_body: BlockSE,
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
pub struct LoopSE {
  pub range: RangeS,
  pub body: BlockSE,
}
/*
case class LoopSE(range: RangeS, body: BlockSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct BreakSE {
  pub range: RangeS,
}
/*
case class BreakSE(range: RangeS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct WhileSE {
  pub range: RangeS,
  pub body: BlockSE,
}
/*
case class WhileSE(range: RangeS, body: BlockSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct MapSE {
  pub range: RangeS,
  pub body: BlockSE,
}
/*
case class MapSE(range: RangeS, body: BlockSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ExprMutateSE {
  pub range: RangeS,
  pub mutatee: Box<IExpressionSE>,
  pub expr: Box<IExpressionSE>,
}
/*
case class ExprMutateSE(range: RangeS, mutatee: IExpressionSE, expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalMutateSE {
  pub range: RangeS,
  pub name: CodeNameS,
  pub expr: Box<IExpressionSE>,
}
/*
case class GlobalMutateSE(range: RangeS, name: CodeNameS, expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct LocalMutateSE {
  pub range: RangeS,
  pub name: IVarNameS,
  pub expr: Box<IExpressionSE>,
}
/*
case class LocalMutateSE(range: RangeS, name: IVarNameS, expr: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct OwnershippedSE {
  pub range: RangeS,
  pub inner_expr: Box<IExpressionSE>,
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
pub struct LocalS {
  pub var_name: IVarNameS,
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
pub struct BodySE {
  pub range: RangeS,
  pub closured_names: Vec<IVarNameS>,
  pub block: BlockSE,
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
pub struct PureSE {
  pub range: RangeS,
  pub location: LocationInDenizen,
  pub inner: Box<IExpressionSE>,
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
pub struct BlockSE {
  pub range: RangeS,
  pub locals: Vec<LocalS>,
  pub expr: Box<IExpressionSE>,
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
pub enum IExpressionSE {
  Let(LetSE),
  If(IfSE),
  Loop(LoopSE),
  Break(BreakSE),
  While(WhileSE),
  Map(MapSE),
  ExprMutate(ExprMutateSE),
  GlobalMutate(GlobalMutateSE),
  LocalMutate(LocalMutateSE),
  Consecutor(ConsecutorSE),
  ArgLookup(ArgLookupSE),
  RepeaterBlock(RepeaterBlockSE),
  RepeaterBlockIterator(RepeaterBlockIteratorSE),
  Void(VoidSE),
  Tuple(TupleSE),
  StaticArrayFromValues(StaticArrayFromValuesSE),
  StaticArrayFromCallable(StaticArrayFromCallableSE),
  NewRuntimeSizedArray(NewRuntimeSizedArraySE),
  RepeaterPack(RepeaterPackSE),
  RepeaterPackIterator(RepeaterPackIteratorSE),
  Block(BlockSE),
  Pure(PureSE),
  Return(ReturnSE),
  ConstantInt(ConstantIntSE),
  ConstantBool(ConstantBoolSE),
  ConstantStr(ConstantStrSE),
  ConstantFloat(ConstantFloatSE),
  Destruct(DestructSE),
  Unlet(UnletSE),
  Function(FunctionSE),
  Dot(DotSE),
  Index(IndexSE),
  FunctionCall(FunctionCallSE),
  LocalLoad(LocalLoadSE),
  OutsideLoad(OutsideLoadSE),
  RuneLookup(RuneLookupSE),
  Ownershipped(OwnershippedSE),
}

impl IExpressionSE {
  pub fn range(&self) -> RangeS {
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
pub struct ConsecutorSE {
  pub exprs: Vec<IExpressionSE>,
}

impl ConsecutorSE {
  pub fn range(&self) -> RangeS {
    assert!(!self.exprs.is_empty());
    RangeS {
      begin: self.exprs.first().unwrap().range().begin,
      end: self.exprs.last().unwrap().range().end,
    }
  }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ArgLookupSE {
  pub range: RangeS,
  pub index: i32,
}
/*
case class ArgLookupSE(range: RangeS, index: Int) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RepeaterBlockSE {
  pub range: RangeS,
  pub expression: Box<IExpressionSE>,
}
/*
 // These things will be separated by semicolons, and all be joined in a block
case class RepeaterBlockSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
 }
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RepeaterBlockIteratorSE {
  pub range: RangeS,
  pub expression: Box<IExpressionSE>,
}
/*
// Results in a pack, represents the differences between the expressions
case class RepeaterBlockIteratorSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ReturnSE {
  pub range: RangeS,
  pub inner: Box<IExpressionSE>,
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
pub struct VoidSE {
  pub range: RangeS,
}
/*
case class VoidSE(range: RangeS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct TupleSE {
  pub range: RangeS,
  pub elements: Vec<IExpressionSE>,
}
/*
case class TupleSE(range: RangeS, elements: Vector[IExpressionSE]) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct StaticArrayFromValuesSE {
  pub range: RangeS,
  pub rules: Vec<IRulexSR>,
  pub maybe_element_type_st: Option<RuneUsage>,
  pub mutability_st: RuneUsage,
  pub variability_st: RuneUsage,
  pub size_st: RuneUsage,
  pub elements: Vec<IExpressionSE>,
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
pub struct StaticArrayFromCallableSE {
  pub range: RangeS,
  pub rules: Vec<IRulexSR>,
  pub maybe_element_type_st: Option<RuneUsage>,
  pub mutability_st: RuneUsage,
  pub variability_st: RuneUsage,
  pub size_st: RuneUsage,
  pub callable: Box<IExpressionSE>,
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
pub struct NewRuntimeSizedArraySE {
  pub range: RangeS,
  pub rules: Vec<IRulexSR>,
  pub maybe_element_type_st: Option<RuneUsage>,
  pub mutability_st: RuneUsage,
  pub size: Box<IExpressionSE>,
  pub callable: Option<Box<IExpressionSE>>,
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
pub struct RepeaterPackSE {
  pub range: RangeS,
  pub expression: Box<IExpressionSE>,
}
/*
// This thing will be repeated, separated by commas, and all be joined in a pack
case class RepeaterPackSE(range: RangeS, expression: IExpressionSE) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RepeaterPackIteratorSE {
  pub range: RangeS,
  pub expression: Box<IExpressionSE>,
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
pub struct ConstantIntSE {
  pub range: RangeS,
  pub value: i64,
  pub bits: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantBoolSE {
  pub range: RangeS,
  pub value: bool,
}
/*
case class ConstantBoolSE(range: RangeS, value: Boolean) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantStrSE {
  pub range: RangeS,
  pub value: String,
}
/*
case class ConstantStrSE(range: RangeS, value: String) extends IExpressionSE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantFloatSE {
  pub range: RangeS,
  pub value: f64,
}
/*
case class ConstantFloatSE(range: RangeS, value: Double) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct DestructSE {
  pub range: RangeS,
  pub inner: Box<IExpressionSE>,
}
/*
case class DestructSE(range: RangeS, inner: IExpressionSE) extends IExpressionSE {
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct UnletSE {
  pub range: RangeS,
  pub name: IVarNameS,
}
/*
case class UnletSE(range: RangeS, name: IVarNameS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSE {
  pub function: FunctionS,
}
/*
case class FunctionSE(function: FunctionS) extends IExpressionSE {
  override def range: RangeS = function.range
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct DotSE {
  pub range: RangeS,
  pub left: Box<IExpressionSE>,
  pub member: std::sync::Arc<crate::interner::StrI>,
  pub borrow_container: bool,
}
/*
case class DotSE(range: RangeS, left: IExpressionSE, member: StrI, borrowContainer: Boolean) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct IndexSE {
  pub range: RangeS,
  pub left: Box<IExpressionSE>,
  pub index_expr: Box<IExpressionSE>,
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
pub struct FunctionCallSE {
  pub range: RangeS,
  pub location: LocationInDenizen,
  pub callable_expr: Box<IExpressionSE>,
  pub arg_exprs: Vec<IExpressionSE>,
}
/*

case class LocalLoadSE(range: RangeS, name: IVarNameS, targetOwnership: LoadAsP) extends IExpressionSE {
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct LocalLoadSE {
  pub range: RangeS,
  pub name: IVarNameS,
  pub target_ownership: LoadAsP,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OutsideLoadSE {
  pub range: RangeS,
  pub rules: Vec<IRulexSR>,
  pub name: IImpreciseNameS,
  pub maybe_template_args: Option<Vec<crate::postparsing::rules::RuneUsage>>,
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
pub struct RuneLookupSE {
  pub range: RangeS,
  pub rune: IRuneS,
}
/*
case class RuneLookupSE(range: RangeS, rune: IRuneS) extends IExpressionSE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/