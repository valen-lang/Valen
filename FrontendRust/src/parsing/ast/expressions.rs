use super::ast::{FunctionP, NameP, OwnershipP, UnitP};
use super::pattern::PatternPP;
use super::templex::{ITemplexPT, RegionRunePT};
use crate::lexing::RangeL;
/*
package dev.vale.parsing.ast

import dev.vale.lexing.RangeL
import dev.vale.{vassert, vcurious, vpass}
import dev.vale.vpass
*/

/// Expression enum - idiomatic Rust replacement for Scala's trait hierarchy
#[derive(Clone, Debug, PartialEq)]
pub enum IExpressionPE<'a, 'p> {
  Void(VoidPE),
  Pack(PackPE<'a, 'p>),
  SubExpression(SubExpressionPE<'a, 'p>),
  And(AndPE<'a, 'p>),
  Or(OrPE<'a, 'p>),
  If(IfPE<'a, 'p>),
  While(WhilePE<'a, 'p>),
  Each(EachPE<'a, 'p>),
  Range(RangePE<'a, 'p>),
  Destruct(DestructPE<'a, 'p>),
  Unlet(UnletPE<'a>),
  Mutate(MutatePE<'a, 'p>),
  Return(ReturnPE<'a, 'p>),
  Break(BreakPE),
  Let(LetPE<'a, 'p>),
  Tuple(TuplePE<'a, 'p>),
  ConstructArray(ConstructArrayPE<'a, 'p>),
  ConstantInt(ConstantIntPE),
  ConstantBool(ConstantBoolPE),
  ConstantStr(ConstantStrPE),
  ConstantFloat(ConstantFloatPE),
  StrInterpolate(StrInterpolatePE<'a, 'p>),
  Dot(DotPE<'a, 'p>),
  Index(IndexPE<'a, 'p>),
  FunctionCall(FunctionCallPE<'a, 'p>),
  BraceCall(BraceCallPE<'a, 'p>),
  Not(NotPE<'a, 'p>),
  Augment(AugmentPE<'a, 'p>),
  Transmigrate(TransmigratePE<'a, 'p>),
  BinaryCall(BinaryCallPE<'a, 'p>),
  MethodCall(MethodCallPE<'a, 'p>),
  Lookup(LookupPE<'a, 'p>),
  MagicParamLookup(MagicParamLookupPE),
  Lambda(LambdaPE<'a, 'p>),
  Block(BlockPE<'a, 'p>),
  Consecutor(ConsecutorPE<'a, 'p>),
  Shortcall(ShortcallPE<'a, 'p>),
}
impl IExpressionPE<'_, '_> {
  pub fn range(&self) -> RangeL {
    match self {
      IExpressionPE::Void(x) => x.range,
      IExpressionPE::Pack(x) => x.range,
      IExpressionPE::SubExpression(x) => x.range,
      IExpressionPE::And(x) => x.range,
      IExpressionPE::Or(x) => x.range,
      IExpressionPE::If(x) => x.range,
      IExpressionPE::While(x) => x.range,
      IExpressionPE::Each(x) => x.range,
      IExpressionPE::Range(x) => x.range,
      IExpressionPE::Destruct(x) => x.range,
      IExpressionPE::Unlet(x) => x.range,
      IExpressionPE::Mutate(x) => x.range,
      IExpressionPE::Return(x) => x.range,
      IExpressionPE::Break(x) => x.range,
      IExpressionPE::Let(x) => x.range,
      IExpressionPE::Tuple(x) => x.range,
      IExpressionPE::ConstructArray(x) => x.range,
      IExpressionPE::ConstantInt(x) => x.range,
      IExpressionPE::ConstantBool(x) => x.range,
      IExpressionPE::ConstantStr(x) => x.range,
      IExpressionPE::ConstantFloat(x) => x.range,
      IExpressionPE::StrInterpolate(x) => x.range,
      IExpressionPE::Dot(x) => x.range,
      IExpressionPE::Index(x) => x.range,
      IExpressionPE::FunctionCall(x) => x.range,
      IExpressionPE::BraceCall(x) => x.range,
      IExpressionPE::Not(x) => x.range,
      IExpressionPE::Augment(x) => x.range,
      IExpressionPE::Transmigrate(x) => x.range,
      IExpressionPE::BinaryCall(x) => x.range,
      IExpressionPE::MethodCall(x) => x.range,
      IExpressionPE::Lookup(x) => x.name.range(),
      IExpressionPE::MagicParamLookup(x) => x.range,
      IExpressionPE::Lambda(x) => x.function.range,
      IExpressionPE::Block(x) => x.range,
      IExpressionPE::Consecutor(x) => {
        assert!(!x.inners.is_empty());
        let begin = x.inners.first().unwrap().range().begin();
        let end = x.inners.last().unwrap().range().end();
        RangeL(begin, end)
      }
      IExpressionPE::Shortcall(x) => x.range,
    }
  }

  pub fn needs_semicolon_before_next_statement(&self) -> bool {
    match self {
      IExpressionPE::Void(_) => false,
      IExpressionPE::Pack(_) => true,
      IExpressionPE::SubExpression(_) => true,
      IExpressionPE::And(_) => true,
      IExpressionPE::Or(_) => true,
      IExpressionPE::If(_) => false,
      IExpressionPE::While(_) => false,
      IExpressionPE::Each(_) => false,
      IExpressionPE::Range(_) => true,
      IExpressionPE::Destruct(_) => true,
      IExpressionPE::Unlet(_) => true,
      IExpressionPE::Mutate(_) => true,
      IExpressionPE::Return(_) => true,
      IExpressionPE::Break(_) => true,
      IExpressionPE::Let(_) => true,
      IExpressionPE::Tuple(_) => true,
      IExpressionPE::ConstructArray(_) => true,
      IExpressionPE::ConstantInt(_) => true,
      IExpressionPE::ConstantBool(_) => true,
      IExpressionPE::ConstantStr(_) => true,
      IExpressionPE::ConstantFloat(_) => true,
      IExpressionPE::StrInterpolate(_) => true,
      IExpressionPE::Dot(_) => true,
      IExpressionPE::Index(_) => true,
      IExpressionPE::FunctionCall(_) => true,
      IExpressionPE::BraceCall(_) => true,
      IExpressionPE::Not(_) => true,
      IExpressionPE::Augment(_) => true,
      IExpressionPE::Transmigrate(_) => true,
      IExpressionPE::BinaryCall(_) => true,
      IExpressionPE::MethodCall(_) => true,
      IExpressionPE::Lookup(_) => true,
      IExpressionPE::MagicParamLookup(_) => true,
      IExpressionPE::Lambda(_) => true,
      IExpressionPE::Block(_) => false,
      IExpressionPE::Consecutor(x) => x
        .inners
        .last()
        .unwrap()
        .needs_semicolon_before_next_statement(),
      IExpressionPE::Shortcall(_) => true,
    }
  }

  pub fn produces_result(&self) -> bool {
    match self {
      IExpressionPE::Void(_) => false,
      IExpressionPE::Pack(_) => true,
      IExpressionPE::SubExpression(_) => true,
      IExpressionPE::And(_) => true,
      IExpressionPE::Or(_) => true,
      IExpressionPE::If(x) => x.then_body.inner.produces_result(),
      IExpressionPE::While(_) => false,
      IExpressionPE::Each(x) => x.body.inner.produces_result(),
      IExpressionPE::Range(_) => true,
      IExpressionPE::Destruct(_) => false,
      IExpressionPE::Unlet(_) => false,
      IExpressionPE::Mutate(_) => true,
      IExpressionPE::Return(_) => false,
      IExpressionPE::Break(_) => false,
      IExpressionPE::Let(_) => false,
      IExpressionPE::Tuple(_) => true,
      IExpressionPE::ConstructArray(_) => true,
      IExpressionPE::ConstantInt(_) => true,
      IExpressionPE::ConstantBool(_) => true,
      IExpressionPE::ConstantStr(_) => true,
      IExpressionPE::ConstantFloat(_) => true,
      IExpressionPE::StrInterpolate(_) => true,
      IExpressionPE::Dot(_) => true,
      IExpressionPE::Index(_) => true,
      IExpressionPE::FunctionCall(_) => true,
      IExpressionPE::BraceCall(_) => true,
      IExpressionPE::Not(_) => true,
      IExpressionPE::Augment(_) => true,
      IExpressionPE::Transmigrate(_) => true,
      IExpressionPE::BinaryCall(_) => true,
      IExpressionPE::MethodCall(_) => true,
      IExpressionPE::Lookup(_) => true,
      IExpressionPE::MagicParamLookup(_) => true,
      IExpressionPE::Lambda(_) => true,
      IExpressionPE::Block(x) => x.inner.produces_result(),
      IExpressionPE::Consecutor(x) => x.inners.last().unwrap().produces_result(),
      IExpressionPE::Shortcall(_) => true,
    }
  }
}
/*
trait IExpressionPE {
  def range: RangeL
  def needsSemicolonBeforeNextStatement: Boolean
  def producesResult(): Boolean
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct VoidPE {
  pub range: RangeL,
}
/*
case class VoidPE(range: RangeL) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = false
  override def producesResult(): Boolean = false
  vpass()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct PackPE<'a, 'p> {
  pub range: RangeL,
  pub inners: &'p [IExpressionPE<'a, 'p>],
}
/*
// We have this because it sometimes even a single-member pack can change the semantics.
// (moo).someMethod() will move moo, and moo.someMethod() will point moo.
// There's probably a better way to distinguish this...
case class PackPE(range: RangeL, inners: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

// Parens that we use for precedence
#[derive(Clone, Debug, PartialEq)]
pub struct SubExpressionPE<'a, 'p> {
  pub range: RangeL,
  pub inner: Box<IExpressionPE<'a, 'p>>,
}
/*
// Parens that we use for precedence
case class SubExpressionPE(range: RangeL, inner: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct AndPE<'a, 'p> {
  pub range: RangeL,
  pub left: Box<IExpressionPE<'a, 'p>>,
  pub right: Box<BlockPE<'a, 'p>>,
}
/*
case class AndPE(range: RangeL, left: IExpressionPE, right: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct OrPE<'a, 'p> {
  pub range: RangeL,
  pub left: Box<IExpressionPE<'a, 'p>>,
  pub right: Box<BlockPE<'a, 'p>>,
}
/*
case class OrPE(range: RangeL, left: IExpressionPE, right: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct IfPE<'a, 'p> {
  pub range: RangeL,
  pub condition: Box<IExpressionPE<'a, 'p>>,
  pub then_body: Box<BlockPE<'a, 'p>>,
  pub else_body: Box<BlockPE<'a, 'p>>,
}
/*
case class IfPE(range: RangeL, condition: IExpressionPE, thenBody: BlockPE, elseBody: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = false
  vcurious(!condition.isInstanceOf[BlockPE])

  // assert(thenBody.producesResult() == elseBody.producesResult())
  // We dont do the above assert because we might have cases like this:
  //   if blah {
  //     return 3;
  //   } else {
  //     6
  //   }

  override def producesResult(): Boolean = {
    thenBody.producesResult()
  }
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct WhilePE<'a, 'p> {
  pub range: RangeL,
  pub condition: Box<IExpressionPE<'a, 'p>>,
  pub body: Box<BlockPE<'a, 'p>>,
}
/*
// condition and body are both blocks because otherwise, if we declare a variable inside them, then
// we could be declaring a variable twice. a block ensures that its scope is cleaned up, which helps
// know we can run it again.
case class WhilePE(range: RangeL, condition: IExpressionPE, body: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = false
  override def producesResult(): Boolean = false
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct EachPE<'a, 'p> {
  pub range: RangeL,
  pub maybe_pure: Option<RangeL>,
  pub entry_pattern: PatternPP<'a, 'p>,
  pub in_keyword_range: RangeL,
  pub iterable_expr: Box<IExpressionPE<'a, 'p>>,
  pub body: Box<BlockPE<'a, 'p>>,
}
/*
case class EachPE(range: RangeL, maybePure: Option[RangeL], entryPattern: PatternPP, inKeywordRange: RangeL, iterableExpr: IExpressionPE, body: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = false
  override def producesResult(): Boolean = body.producesResult()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct RangePE<'a, 'p> {
  pub range: RangeL,
  pub from_expr: Box<IExpressionPE<'a, 'p>>,
  pub to_expr: Box<IExpressionPE<'a, 'p>>,
}
/*
case class RangePE(range: RangeL, fromExpr: IExpressionPE, toExpr: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct DestructPE<'a, 'p> {
  pub range: RangeL,
  pub inner: Box<IExpressionPE<'a, 'p>>,
}
/*
case class DestructPE(range: RangeL, inner: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct UnletPE<'a> {
  pub range: RangeL,
  pub name: IImpreciseNameP<'a>,
}
/*
case class UnletPE(range: RangeL, name: IImpreciseNameP) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct MutatePE<'a, 'p> {
  pub range: RangeL,
  pub mutatee: Box<IExpressionPE<'a, 'p>>,
  pub source: Box<IExpressionPE<'a, 'p>>,
}
/*
//case class MatchPE(range: RangeP, condition: IExpressionPE, lambdas: Vector[LambdaPE]) extends IExpressionPE {
//  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
//  override def needsSemicolonAtEndOfStatement: Boolean = false
//}
case class MutatePE(range: RangeL, mutatee: IExpressionPE, source: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnPE<'a, 'p> {
  pub range: RangeL,
  pub expr: Box<IExpressionPE<'a, 'p>>,
}
/*
case class ReturnPE(range: RangeL, expr: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct BreakPE {
  pub range: RangeL,
}
/*
case class BreakPE(range: RangeL) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct LetPE<'a, 'p> {
  pub range: RangeL,
  pub pattern: PatternPP<'a, 'p>,
  pub source: Box<IExpressionPE<'a, 'p>>,
}
/*
case class LetPE(
  range: RangeL,
//  templateRules: Option[TemplateRulesP],
  pattern: PatternPP,
  source: IExpressionPE
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TuplePE<'a, 'p> {
  pub range: RangeL,
  pub elements: &'p [IExpressionPE<'a, 'p>],
}
/*
case class TuplePE(range: RangeL, elements: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StaticSizedArraySizeP<'a, 'p> {
  pub size_pt: Option<ITemplexPT<'a, 'p>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IArraySizeP<'a, 'p> {
  RuntimeSized,
  StaticSized(StaticSizedArraySizeP<'a, 'p>),
}
/*
sealed trait IArraySizeP
case object RuntimeSizedP extends IArraySizeP
case class StaticSizedP(sizePT: Option[ITemplexPT]) extends IArraySizeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ConstructArrayPE<'a, 'p> {
  pub range: RangeL,
  pub type_pt: Option<ITemplexPT<'a, 'p>>,
  pub mutability_pt: Option<ITemplexPT<'a, 'p>>,
  pub variability_pt: Option<ITemplexPT<'a, 'p>>,
  pub size: IArraySizeP<'a, 'p>,
  pub initializing_individual_elements: bool,
  pub args: &'p [IExpressionPE<'a, 'p>],
}
/*
case class ConstructArrayPE(
  range: RangeL,
  typePT: Option[ITemplexPT],
  mutabilityPT: Option[ITemplexPT],
  variabilityPT: Option[ITemplexPT],
  size: IArraySizeP,
  // True if theyre making it like [3](10, 20, 30)
  // False if theyre making it like [3]({ _ * 10 })
  initializingIndividualElements: Boolean,
  args: Vector[IExpressionPE]
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantIntPE {
  pub range: RangeL,
  pub value: i64,
  pub bits: Option<i64>,
}
/*
case class ConstantIntPE(range: RangeL, value: Long, bits: Option[Long]) extends IExpressionPE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantBoolPE {
  pub range: RangeL,
  pub value: bool,
}
/*
case class ConstantBoolPE(range: RangeL, value: Boolean) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantStrPE {
  pub range: RangeL,
  pub value: String,
}
/*
case class ConstantStrPE(range: RangeL, value: String) extends IExpressionPE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantFloatPE {
  pub range: RangeL,
  pub value: f64,
}
/*
case class ConstantFloatPE(range: RangeL, value: Double) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StrInterpolatePE<'a, 'p> {
  pub range: RangeL,
  pub parts: &'p [IExpressionPE<'a, 'p>],
}
/*
case class StrInterpolatePE(range: RangeL, parts: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct DotPE<'a, 'p> {
  pub range: RangeL,
  pub left: Box<IExpressionPE<'a, 'p>>,
  pub operator_range: RangeL,
  pub member: NameP<'a>,
}
/*
case class DotPE(
  range: RangeL,
  left: IExpressionPE,
  operatorRange: RangeL,
  member: NameP) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct IndexPE<'a, 'p> {
  pub range: RangeL,
  pub left: Box<IExpressionPE<'a, 'p>>,
  pub args: &'p [IExpressionPE<'a, 'p>],
}
/*
case class IndexPE(range: RangeL, left: IExpressionPE, args: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCallPE<'a, 'p> {
  pub range: RangeL,
  pub operator_range: RangeL,
  pub callable_expr: Box<IExpressionPE<'a, 'p>>,
  pub arg_exprs: &'p [IExpressionPE<'a, 'p>],
}
/*
case class FunctionCallPE(
  range: RangeL,
  operatorRange: RangeL,
  callableExpr: IExpressionPE,
  argExprs: Vector[IExpressionPE]
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct BraceCallPE<'a, 'p> {
  pub range: RangeL,
  pub operator_range: RangeL,
  pub subject_expr: Box<IExpressionPE<'a, 'p>>,
  pub arg_exprs: &'p [IExpressionPE<'a, 'p>],
  pub callable_readwrite: bool,
}
/*
case class BraceCallPE(
  range: RangeL,
  operatorRange: RangeL,
  subjectExpr: IExpressionPE,
  argExprs: Vector[IExpressionPE],
  callableReadwrite: Boolean
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct NotPE<'a, 'p> {
  pub range: RangeL,
  pub inner: Box<IExpressionPE<'a, 'p>>,
}
/*
case class NotPE(range: RangeL, inner: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
  vpass()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct AugmentPE<'a, 'p> {
  pub range: RangeL,
  pub target_ownership: OwnershipP,
  pub inner: Box<IExpressionPE<'a, 'p>>,
}
/*
case class AugmentPE(
  range: RangeL,
  targetOwnership: OwnershipP,
  inner: IExpressionPE
) extends IExpressionPE {

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
  vpass()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TransmigratePE<'a, 'p> {
  pub range: RangeL,
  pub target_region: NameP<'a>,
  pub inner: Box<IExpressionPE<'a, 'p>>,
}
/*
case class TransmigratePE(
    range: RangeL,
    targetRegion: NameP,
    inner: IExpressionPE
) extends IExpressionPE {

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
  vpass()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryCallPE<'a, 'p> {
  pub range: RangeL,
  pub function_name: NameP<'a>,
  pub left_expr: Box<IExpressionPE<'a, 'p>>,
  pub right_expr: Box<IExpressionPE<'a, 'p>>,
}
/*
case class BinaryCallPE(
  range: RangeL,
  functionName: NameP,
  leftExpr: IExpressionPE,
  rightExpr: IExpressionPE
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct MethodCallPE<'a, 'p> {
  pub range: RangeL,
  pub subject_expr: Box<IExpressionPE<'a, 'p>>,
  pub operator_range: RangeL,
  pub method_lookup: Box<LookupPE<'a, 'p>>,
  pub arg_exprs: &'p [IExpressionPE<'a, 'p>],
}
/*
case class MethodCallPE(
  range: RangeL,
  subjectExpr: IExpressionPE,
  operatorRange: RangeL,
  methodLookup: LookupPE,
  argExprs: Vector[IExpressionPE]
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
  vpass()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IImpreciseNameP<'a> {
  LookupName(NameP<'a>),
  IterableName(RangeL),
  IteratorName(RangeL),
  IterationOptionName(RangeL),
}
impl IImpreciseNameP<'_> {
  pub fn range(&self) -> RangeL {
    match self {
      IImpreciseNameP::LookupName(n) => n.range(),
      IImpreciseNameP::IterableName(r) => *r,
      IImpreciseNameP::IteratorName(r) => *r,
      IImpreciseNameP::IterationOptionName(r) => *r,
    }
  }
}
/*
sealed trait IImpreciseNameP {
  def range: RangeL
}
case class LookupNameP(name: NameP) extends IImpreciseNameP { override def range: RangeL = name.range }
case class IterableNameP(range: RangeL) extends IImpreciseNameP
case class IteratorNameP(range: RangeL) extends IImpreciseNameP
case class IterationOptionNameP(range: RangeL) extends IImpreciseNameP
*/

#[derive(Clone, Debug, PartialEq)]
pub struct LookupPE<'a, 'p> {
  pub name: IImpreciseNameP<'a>,
  pub template_args: Option<TemplateArgsP<'a, 'p>>,
}
/*
case class LookupPE(
  name: IImpreciseNameP,
  templateArgs: Option[TemplateArgsP]
) extends IExpressionPE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def range: RangeL = name.range
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateArgsP<'a, 'p> {
  pub range: RangeL,
  pub args: &'p [ITemplexPT<'a, 'p>],
}
/*
case class TemplateArgsP(range: RangeL, args: Vector[ITemplexPT]) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct MagicParamLookupPE {
  pub range: RangeL,
}
/*
case class MagicParamLookupPE(range: RangeL) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct LambdaPE<'a, 'p> {
  pub captures: Option<UnitP>,
  pub function: FunctionP<'a, 'p>,
}
/*
case class LambdaPE(
  // Just here for syntax highlighting so far
  captures: Option[UnitP],
  function: FunctionP
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def range: RangeL = function.range
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct BlockPE<'a, 'p> {
  pub range: RangeL,
  pub maybe_pure: Option<RangeL>,
  pub maybe_default_region: Option<RegionRunePT<'a>>,
  pub inner: Box<IExpressionPE<'a, 'p>>,
}
/*
case class BlockPE(range: RangeL, maybePure: Option[RangeL], maybeDefaultRegion: Option[RegionRunePT], inner: IExpressionPE) extends IExpressionPE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = false
  override def producesResult(): Boolean = inner.producesResult()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ConsecutorPE<'a, 'p> {
  pub inners: &'p [IExpressionPE<'a, 'p>],
}
/*
case class ConsecutorPE(inners: Vector[IExpressionPE]) extends IExpressionPE {
  // Should have at least one expression, because a block will
  // return the last expression's result as its result.
  // Even empty blocks aren't empty, they have a void() at the end.
  vassert(inners.size >= 1)

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious();

  override def range: RangeL = RangeL(inners.head.range.begin, inners.last.range.end)

  override def needsSemicolonBeforeNextStatement: Boolean = inners.last.needsSemicolonBeforeNextStatement
  override def producesResult(): Boolean = inners.last.producesResult()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ShortcallPE<'a, 'p> {
  pub range: RangeL,
  pub arg_exprs: &'p [IExpressionPE<'a, 'p>],
}
/*
case class ShortcallPE(range: RangeL, argExprs: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true

  vpass()
}
*/
