use super::ast::{FunctionP, NameP, OwnershipP, UnitP};
use crate::interner::StrI;
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
#[derive(Debug, PartialEq)]
pub enum IExpressionPE<'p> {
  Void(VoidPE),
  Pack(PackPE<'p>),
  SubExpression(SubExpressionPE<'p>),
  And(AndPE<'p>),
  Or(OrPE<'p>),
  If(IfPE<'p>),
  While(WhilePE<'p>),
  Each(EachPE<'p>),
  Range(RangePE<'p>),
  Destruct(DestructPE<'p>),
  Unlet(UnletPE<'p>),
  Mutate(MutatePE<'p>),
  Return(ReturnPE<'p>),
  Break(BreakPE),
  Let(LetPE<'p>),
  Tuple(TuplePE<'p>),
  ConstructArray(ConstructArrayPE<'p>),
  ConstantInt(ConstantIntPE),
  ConstantBool(ConstantBoolPE),
  ConstantStr(ConstantStrPE<'p>),
  ConstantFloat(ConstantFloatPE),
  StrInterpolate(StrInterpolatePE<'p>),
  Dot(DotPE<'p>),
  Index(IndexPE<'p>),
  FunctionCall(FunctionCallPE<'p>),
  BraceCall(BraceCallPE<'p>),
  Not(NotPE<'p>),
  Augment(AugmentPE<'p>),
  Transmigrate(TransmigratePE<'p>),
  BinaryCall(BinaryCallPE<'p>),
  MethodCall(MethodCallPE<'p>),
  Lookup(&'p LookupPE<'p>),
  MagicParamLookup(MagicParamLookupPE),
  Lambda(LambdaPE<'p>),
  Block(BlockPE<'p>),
  Consecutor(ConsecutorPE<'p>),
  Shortcall(ShortcallPE<'p>),
}
impl IExpressionPE<'_> {
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

#[derive(Debug, PartialEq)]
pub struct VoidPE {
  pub range: RangeL,
}
/*
case class VoidPE(range: RangeL) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = false
  override def producesResult(): Boolean = false
  vpass()
}
*/

#[derive(Debug, PartialEq)]
pub struct PackPE<'p> {
  pub range: RangeL,
  pub inners: &'p [&'p IExpressionPE<'p>],
}
/*
// We have this because it sometimes even a single-member pack can change the semantics.
// (moo).someMethod() will move moo, and moo.someMethod() will point moo.
// There's probably a better way to distinguish this...
case class PackPE(range: RangeL, inners: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

// Parens that we use for precedence
#[derive(Debug, PartialEq)]
pub struct SubExpressionPE<'p> {
  pub range: RangeL,
  pub inner: &'p IExpressionPE<'p>,
}
/*
// Parens that we use for precedence
case class SubExpressionPE(range: RangeL, inner: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct AndPE<'p> {
  pub range: RangeL,
  pub left: &'p IExpressionPE<'p>,
  pub right: &'p BlockPE<'p>,
}
/*
case class AndPE(range: RangeL, left: IExpressionPE, right: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct OrPE<'p> {
  pub range: RangeL,
  pub left: &'p IExpressionPE<'p>,
  pub right: &'p BlockPE<'p>,
}
/*
case class OrPE(range: RangeL, left: IExpressionPE, right: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct IfPE<'p> {
  pub range: RangeL,
  pub condition: &'p IExpressionPE<'p>,
  pub then_body: &'p BlockPE<'p>,
  pub else_body: &'p BlockPE<'p>,
}
/*
case class IfPE(range: RangeL, condition: IExpressionPE, thenBody: BlockPE, elseBody: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
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

#[derive(Debug, PartialEq)]
pub struct WhilePE<'p> {
  pub range: RangeL,
  pub condition: &'p IExpressionPE<'p>,
  pub body: &'p BlockPE<'p>,
}
/*
// condition and body are both blocks because otherwise, if we declare a variable inside them, then
// we could be declaring a variable twice. a block ensures that its scope is cleaned up, which helps
// know we can run it again.
case class WhilePE(range: RangeL, condition: IExpressionPE, body: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = false
  override def producesResult(): Boolean = false
}
*/

#[derive(Debug, PartialEq)]
pub struct EachPE<'p> {
  pub range: RangeL,
  pub maybe_pure: Option<RangeL>,
  pub entry_pattern: PatternPP<'p>,
  pub in_keyword_range: RangeL,
  pub iterable_expr: &'p IExpressionPE<'p>,
  pub body: &'p BlockPE<'p>,
}
/*
case class EachPE(range: RangeL, maybePure: Option[RangeL], entryPattern: PatternPP, inKeywordRange: RangeL, iterableExpr: IExpressionPE, body: BlockPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = false
  override def producesResult(): Boolean = body.producesResult()
}
*/

#[derive(Debug, PartialEq)]
pub struct RangePE<'p> {
  pub range: RangeL,
  pub from_expr: &'p IExpressionPE<'p>,
  pub to_expr: &'p IExpressionPE<'p>,
}
/*
case class RangePE(range: RangeL, fromExpr: IExpressionPE, toExpr: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct DestructPE<'p> {
  pub range: RangeL,
  pub inner: &'p IExpressionPE<'p>,
}
/*
case class DestructPE(range: RangeL, inner: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Debug, PartialEq)]
pub struct UnletPE<'p> {
  pub range: RangeL,
  pub name: IImpreciseNameP<'p>,
}
/*
case class UnletPE(range: RangeL, name: IImpreciseNameP) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Debug, PartialEq)]
pub struct MutatePE<'p> {
  pub range: RangeL,
  pub mutatee: &'p IExpressionPE<'p>,
  pub source: &'p IExpressionPE<'p>,
}
/*
//case class MatchPE(range: RangeP, condition: IExpressionPE, lambdas: Vector[LambdaPE]) extends IExpressionPE {
//  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
//  override def needsSemicolonAtEndOfStatement: Boolean = false
//}
case class MutatePE(range: RangeL, mutatee: IExpressionPE, source: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct ReturnPE<'p> {
  pub range: RangeL,
  pub expr: &'p IExpressionPE<'p>,
}
/*
case class ReturnPE(range: RangeL, expr: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Debug, PartialEq)]
pub struct BreakPE {
  pub range: RangeL,
}
/*
case class BreakPE(range: RangeL) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Debug, PartialEq)]
pub struct LetPE<'p> {
  pub range: RangeL,
  pub pattern: &'p PatternPP<'p>,
  pub source: &'p IExpressionPE<'p>,
}
/*
case class LetPE(
  range: RangeL,
//  templateRules: Option[TemplateRulesP],
  pattern: PatternPP,
  source: IExpressionPE
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = false
}
*/

#[derive(Debug, PartialEq)]
pub struct TuplePE<'p> {
  pub range: RangeL,
  pub elements: &'p [&'p IExpressionPE<'p>],
}
/*
case class TuplePE(range: RangeL, elements: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct StaticSizedArraySizeP<'p> {
  pub size_pt: Option<ITemplexPT<'p>>,
}

#[derive(Debug, PartialEq)]
pub enum IArraySizeP<'p> {
  RuntimeSized,
  StaticSized(StaticSizedArraySizeP<'p>),
}
/*
sealed trait IArraySizeP
case object RuntimeSizedP extends IArraySizeP
case class StaticSizedP(sizePT: Option[ITemplexPT]) extends IArraySizeP { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Debug, PartialEq)]
pub struct ConstructArrayPE<'p> {
  pub range: RangeL,
  pub type_pt: Option<ITemplexPT<'p>>,
  pub mutability_pt: Option<ITemplexPT<'p>>,
  pub variability_pt: Option<ITemplexPT<'p>>,
  pub size: IArraySizeP<'p>,
  pub initializing_individual_elements: bool,
  pub args: &'p [&'p IExpressionPE<'p>],
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
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct ConstantIntPE {
  pub range: RangeL,
  pub value: i64,
  pub bits: Option<i64>,
}
/*
case class ConstantIntPE(range: RangeL, value: Long, bits: Option[Long]) extends IExpressionPE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct ConstantBoolPE {
  pub range: RangeL,
  pub value: bool,
}
/*
case class ConstantBoolPE(range: RangeL, value: Boolean) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct ConstantStrPE<'p> {
  pub range: RangeL,
  pub value: StrI<'p>,
}
/*
case class ConstantStrPE(range: RangeL, value: String) extends IExpressionPE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct ConstantFloatPE {
  pub range: RangeL,
  pub value: f64,
}
/*
case class ConstantFloatPE(range: RangeL, value: Double) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct StrInterpolatePE<'p> {
  pub range: RangeL,
  pub parts: &'p [&'p IExpressionPE<'p>],
}
/*
case class StrInterpolatePE(range: RangeL, parts: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct DotPE<'p> {
  pub range: RangeL,
  pub left: &'p IExpressionPE<'p>,
  pub operator_range: RangeL,
  pub member: NameP<'p>,
}
/*
case class DotPE(
  range: RangeL,
  left: IExpressionPE,
  operatorRange: RangeL,
  member: NameP) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct IndexPE<'p> {
  pub range: RangeL,
  pub left: &'p IExpressionPE<'p>,
  pub args: &'p [&'p IExpressionPE<'p>],
}
/*
case class IndexPE(range: RangeL, left: IExpressionPE, args: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct FunctionCallPE<'p> {
  pub range: RangeL,
  pub operator_range: RangeL,
  pub callable_expr: &'p IExpressionPE<'p>,
  pub arg_exprs: &'p [&'p IExpressionPE<'p>],
}
/*
case class FunctionCallPE(
  range: RangeL,
  operatorRange: RangeL,
  callableExpr: IExpressionPE,
  argExprs: Vector[IExpressionPE]
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct BraceCallPE<'p> {
  pub range: RangeL,
  pub operator_range: RangeL,
  pub subject_expr: &'p IExpressionPE<'p>,
  pub arg_exprs: &'p [&'p IExpressionPE<'p>],
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
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct NotPE<'p> {
  pub range: RangeL,
  pub inner: &'p IExpressionPE<'p>,
}
/*
case class NotPE(range: RangeL, inner: IExpressionPE) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
  vpass()
}
*/

#[derive(Debug, PartialEq)]
pub struct AugmentPE<'p> {
  pub range: RangeL,
  pub target_ownership: OwnershipP,
  pub inner: &'p IExpressionPE<'p>,
}
/*
case class AugmentPE(
  range: RangeL,
  targetOwnership: OwnershipP,
  inner: IExpressionPE
) extends IExpressionPE {

  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
  vpass()
}
*/

#[derive(Debug, PartialEq)]
pub struct TransmigratePE<'p> {
  pub range: RangeL,
  pub target_region: NameP<'p>,
  pub inner: &'p IExpressionPE<'p>,
}
/*
case class TransmigratePE(
    range: RangeL,
    targetRegion: NameP,
    inner: IExpressionPE
) extends IExpressionPE {

  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
  vpass()
}
*/

#[derive(Debug, PartialEq)]
pub struct BinaryCallPE<'p> {
  pub range: RangeL,
  pub function_name: NameP<'p>,
  pub left_expr: &'p IExpressionPE<'p>,
  pub right_expr: &'p IExpressionPE<'p>,
}
/*
case class BinaryCallPE(
  range: RangeL,
  functionName: NameP,
  leftExpr: IExpressionPE,
  rightExpr: IExpressionPE
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct MethodCallPE<'p> {
  pub range: RangeL,
  pub subject_expr: &'p IExpressionPE<'p>,
  pub operator_range: RangeL,
  pub method_lookup: &'p LookupPE<'p>,
  pub arg_exprs: &'p [&'p IExpressionPE<'p>],
}
/*
case class MethodCallPE(
  range: RangeL,
  subjectExpr: IExpressionPE,
  operatorRange: RangeL,
  methodLookup: LookupPE,
  argExprs: Vector[IExpressionPE]
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
  vpass()
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IImpreciseNameP<'p> {
  LookupName(NameP<'p>),
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

#[derive(Debug, PartialEq)]
pub struct LookupPE<'p> {
  pub name: IImpreciseNameP<'p>,
  pub template_args: Option<TemplateArgsP<'p>>,
}
/*
case class LookupPE(
  name: IImpreciseNameP,
  templateArgs: Option[TemplateArgsP]
) extends IExpressionPE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def range: RangeL = name.range
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct TemplateArgsP<'p> {
  pub range: RangeL,
  pub args: &'p [&'p ITemplexPT<'p>],
}
/*
case class TemplateArgsP(range: RangeL, args: Vector[ITemplexPT]) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/

#[derive(Debug, PartialEq)]
pub struct MagicParamLookupPE {
  pub range: RangeL,
}
/*
case class MagicParamLookupPE(range: RangeL) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct LambdaPE<'p> {
  pub captures: Option<UnitP>,
  pub function: FunctionP<'p>,
}
/*
case class LambdaPE(
  // Just here for syntax highlighting so far
  captures: Option[UnitP],
  function: FunctionP
) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def range: RangeL = function.range
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true
}
*/

#[derive(Debug, PartialEq)]
pub struct BlockPE<'p> {
  pub range: RangeL,
  pub maybe_pure: Option<RangeL>,
  pub maybe_default_region: Option<RegionRunePT<'p>>,
  pub inner: &'p IExpressionPE<'p>,
}
/*
case class BlockPE(range: RangeL, maybePure: Option[RangeL], maybeDefaultRegion: Option[RegionRunePT], inner: IExpressionPE) extends IExpressionPE {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def needsSemicolonBeforeNextStatement: Boolean = false
  override def producesResult(): Boolean = inner.producesResult()
}
*/

#[derive(Debug, PartialEq)]
pub struct ConsecutorPE<'p> {
  pub inners: &'p [&'p IExpressionPE<'p>],
}
/*
case class ConsecutorPE(inners: Vector[IExpressionPE]) extends IExpressionPE {
  // Should have at least one expression, because a block will
  // return the last expression's result as its result.
  // Even empty blocks aren't empty, they have a void() at the end.
  vassert(inners.size >= 1)

  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();

  override def range: RangeL = RangeL(inners.head.range.begin, inners.last.range.end)

  override def needsSemicolonBeforeNextStatement: Boolean = inners.last.needsSemicolonBeforeNextStatement
  override def producesResult(): Boolean = inners.last.producesResult()
}
*/

#[derive(Debug, PartialEq)]
pub struct ShortcallPE<'p> {
  pub range: RangeL,
  pub arg_exprs: &'p [&'p IExpressionPE<'p>],
}
/*
case class ShortcallPE(range: RangeL, argExprs: Vector[IExpressionPE]) extends IExpressionPE {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  override def needsSemicolonBeforeNextStatement: Boolean = true
  override def producesResult(): Boolean = true

  vpass()
}
*/
