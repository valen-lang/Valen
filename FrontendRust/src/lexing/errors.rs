use crate::utils::code_hierarchy::FileCoordinate;

/// Failed parse with context
#[derive(Debug)]
pub struct FailedParse<'p> {
  pub code: String,
  pub file_coord: FileCoordinate<'p>,
  pub error: ParseError,
}

/// Parse error types
#[derive(Debug)]
pub enum ParseError {
  RangedInternalError { pos: i32, msg: String },
  UnrecognizableExpressionAfterAugment(i32),
  OnlyRegionRunesCanHaveMutability(i32),
  BadMemberEnd(i32),
  BadLambdaBegin(i32),
  BadLambdaBodyBegin(i32),
  EmptyParameter(i32),
  CantUseThatLocalName { pos: i32, name: String },
  LightFunctionMustHaveParamTypes { pos: i32, param_index: i32 },
  EmptyPattern(i32),
  BadNameBeforeDestructure(i32),
  BadLocalNameInUnlet(i32),
  FoundBothAbstractAndOverride(i32),
  FoundBothImmutableAndMutabilityInArray(i32),
  BadStringInTemplex(i32),
  BadPrototypeName(i32),
  BadPrototypeParams(i32),
  BadRuleCallParam(i32),
  BadTypeExpression(i32),
  BadTemplateCallParam(i32),
  BadTupleElement(i32),
  BadDestructureError(i32),
  ShareCantBeReadwrite(i32),
  BadRuneEnd(i32),
  BadRegionName(i32),
  BadRuneNameError(i32),
  RegionRuneHasType(i32),
  BadRuneTypeError(i32),
  UnrecognizedDenizenError(i32),
  BadStartOfStatementError(i32),
  BadExpressionEnd(i32),
  BadRule(i32),
  UnexpectedAttributes(i32),
  IfBlocksMustBothOrNeitherReturn(i32),
  BadExpressionBegin(i32),
  BadStringChar { string_begin_pos: i32, pos: i32 },
  BadFunctionName(i32),
  BadParamEnd(i32),
  ForgotSetKeyword(i32),
  FuncBoundWithoutWhere(i32),
  BadBinaryFunctionName(i32),
  BadTemplateCallee(i32),
  UnknownTupleOrSubExpression(i32),
  BadRangeOperand(i32),
  CantUseBreakInExpression(i32),
  CantUseReturnInExpression(i32),
  CantUseWhileInExpression(i32),
  NeedSemicolon(i32),
  DontNeedSemicolon(i32),
  BadDot(i32),
  CantTemplateCallMember(i32),
  NeedWhitespaceAroundBinaryOperator(i32),
  BadFunctionBodyError(i32),
  BadUnicodeChar(i32),
  BadStringInterpolationEnd(i32),
  BadStartOfBlock(i32),
  BadEndOfBlock(i32),
  BadStartOfWhileCondition(i32),
  BadEndOfWhileCondition(i32),
  BadStartOfWhileBody(i32),
  BadEndOfWhileBody(i32),
  BadStartOfIfCondition(i32),
  BadEndOfIfCondition(i32),
  BadStartOfIfBody(i32),
  BadEndOfIfBody(i32),
  BadStartOfElseBody(i32),
  BadEndOfElseBody(i32),
  BadLetEqualsError(i32),
  BadMutateEqualsError(i32),
  BadLetEndError(i32),
  BadVPSTError { message: String },
  BadArraySizerEnd(i32),
  BadArraySizer(i32),
  BadStructName(i32),
  BadInterfaceName(i32),
  BadStructContentsBegin(i32),
  BadStructContentsEnd(i32),
  BadStructMember(i32),
  BadStructMemberType(i32),
  VariadicStructMemberHasName(i32),
  BadInterfaceMember(i32),
  BadInterfaceContentsBegin(i32),
  BadInterfaceHeader(i32),
  BadImpl(i32),
  BadImplStruct(i32),
  BadImplInterface(i32),
  BadImplEnd(i32),
  BadImplFor(i32),
  BadExportAs(i32),
  BadImportEnd(i32),
  BadImportName(i32),
  BadFunctionParamsBegin(i32),
  BadFunctionAfterParam(i32),
  BadAttributeError(i32),
  BadExternAttribute(i32),
  BadExportName(i32),
  BadExportEnd(i32),
  BadForeachIterableError(i32),
  BadArraySpecifier(i32),
  BadLocalName(i32),
  BadThingAfterTypeInPattern(i32),
  BadLetSourceError { pos: i32, cause: Box<ParseError> },
  BadForeachInError(i32),
}

impl ParseError {
  pub fn pos(&self) -> i32 {
    match self {
      ParseError::RangedInternalError { pos, .. } => *pos,
      ParseError::UnrecognizableExpressionAfterAugment(p) => *p,
      ParseError::OnlyRegionRunesCanHaveMutability(p) => *p,
      ParseError::BadMemberEnd(p) => *p,
      ParseError::BadLambdaBegin(p) => *p,
      ParseError::BadLambdaBodyBegin(p) => *p,
      ParseError::EmptyParameter(p) => *p,
      ParseError::CantUseThatLocalName { pos, .. } => *pos,
      ParseError::LightFunctionMustHaveParamTypes { pos, .. } => *pos,
      ParseError::EmptyPattern(p) => *p,
      ParseError::BadNameBeforeDestructure(p) => *p,
      ParseError::BadLocalNameInUnlet(p) => *p,
      ParseError::FoundBothAbstractAndOverride(p) => *p,
      ParseError::FoundBothImmutableAndMutabilityInArray(p) => *p,
      ParseError::BadStringInTemplex(p) => *p,
      ParseError::BadPrototypeName(p) => *p,
      ParseError::BadPrototypeParams(p) => *p,
      ParseError::BadRuleCallParam(p) => *p,
      ParseError::BadTypeExpression(p) => *p,
      ParseError::BadTemplateCallParam(p) => *p,
      ParseError::BadTupleElement(p) => *p,
      ParseError::BadDestructureError(p) => *p,
      ParseError::ShareCantBeReadwrite(p) => *p,
      ParseError::BadRuneEnd(p) => *p,
      ParseError::BadRegionName(p) => *p,
      ParseError::BadRuneNameError(p) => *p,
      ParseError::RegionRuneHasType(p) => *p,
      ParseError::BadRuneTypeError(p) => *p,
      ParseError::UnrecognizedDenizenError(p) => *p,
      ParseError::BadStartOfStatementError(p) => *p,
      ParseError::BadExpressionEnd(p) => *p,
      ParseError::BadRule(p) => *p,
      ParseError::UnexpectedAttributes(p) => *p,
      ParseError::IfBlocksMustBothOrNeitherReturn(p) => *p,
      ParseError::BadExpressionBegin(p) => *p,
      ParseError::BadStringChar { pos, .. } => *pos,
      ParseError::BadFunctionName(p) => *p,
      ParseError::BadParamEnd(p) => *p,
      ParseError::ForgotSetKeyword(p) => *p,
      ParseError::FuncBoundWithoutWhere(p) => *p,
      ParseError::BadBinaryFunctionName(p) => *p,
      ParseError::BadTemplateCallee(p) => *p,
      ParseError::UnknownTupleOrSubExpression(p) => *p,
      ParseError::BadRangeOperand(p) => *p,
      ParseError::CantUseBreakInExpression(p) => *p,
      ParseError::CantUseReturnInExpression(p) => *p,
      ParseError::CantUseWhileInExpression(p) => *p,
      ParseError::NeedSemicolon(p) => *p,
      ParseError::DontNeedSemicolon(p) => *p,
      ParseError::BadDot(p) => *p,
      ParseError::CantTemplateCallMember(p) => *p,
      ParseError::NeedWhitespaceAroundBinaryOperator(p) => *p,
      ParseError::BadFunctionBodyError(p) => *p,
      ParseError::BadUnicodeChar(p) => *p,
      ParseError::BadStringInterpolationEnd(p) => *p,
      ParseError::BadStartOfBlock(p) => *p,
      ParseError::BadEndOfBlock(p) => *p,
      ParseError::BadStartOfWhileCondition(p) => *p,
      ParseError::BadEndOfWhileCondition(p) => *p,
      ParseError::BadStartOfWhileBody(p) => *p,
      ParseError::BadEndOfWhileBody(p) => *p,
      ParseError::BadStartOfIfCondition(p) => *p,
      ParseError::BadEndOfIfCondition(p) => *p,
      ParseError::BadStartOfIfBody(p) => *p,
      ParseError::BadEndOfIfBody(p) => *p,
      ParseError::BadStartOfElseBody(p) => *p,
      ParseError::BadEndOfElseBody(p) => *p,
      ParseError::BadLetEqualsError(p) => *p,
      ParseError::BadMutateEqualsError(p) => *p,
      ParseError::BadLetEndError(p) => *p,
      ParseError::BadVPSTError { .. } => 0,
      ParseError::BadArraySizerEnd(p) => *p,
      ParseError::BadArraySizer(p) => *p,
      ParseError::BadStructName(p) => *p,
      ParseError::BadInterfaceName(p) => *p,
      ParseError::BadStructContentsBegin(p) => *p,
      ParseError::BadStructContentsEnd(p) => *p,
      ParseError::BadStructMember(p) => *p,
      ParseError::BadStructMemberType(p) => *p,
      ParseError::VariadicStructMemberHasName(p) => *p,
      ParseError::BadInterfaceMember(p) => *p,
      ParseError::BadInterfaceContentsBegin(p) => *p,
      ParseError::BadInterfaceHeader(p) => *p,
      ParseError::BadImpl(p) => *p,
      ParseError::BadImplStruct(p) => *p,
      ParseError::BadImplInterface(p) => *p,
      ParseError::BadImplEnd(p) => *p,
      ParseError::BadImplFor(p) => *p,
      ParseError::BadExportAs(p) => *p,
      ParseError::BadImportEnd(p) => *p,
      ParseError::BadImportName(p) => *p,
      ParseError::BadFunctionParamsBegin(p) => *p,
      ParseError::BadFunctionAfterParam(p) => *p,
      ParseError::BadAttributeError(p) => *p,
      ParseError::BadExternAttribute(p) => *p,
      ParseError::BadExportName(p) => *p,
      ParseError::BadExportEnd(p) => *p,
      ParseError::BadForeachIterableError(p) => *p,
      ParseError::BadArraySpecifier(p) => *p,
      ParseError::BadLocalName(p) => *p,
      ParseError::BadThingAfterTypeInPattern(p) => *p,
      ParseError::BadLetSourceError { pos, .. } => *pos,
      ParseError::BadForeachInError(p) => *p,
    }
  }

  pub fn error_id(&self) -> &str {
    match self {
      ParseError::BadStartOfStatementError(_) => "P1002",
      ParseError::BadExpressionEnd(_)
      | ParseError::BadRule(_)
      | ParseError::UnexpectedAttributes(_)
      | ParseError::IfBlocksMustBothOrNeitherReturn(_)
      | ParseError::BadExpressionBegin(_)
      | ParseError::BadStringChar { .. }
      | ParseError::BadFunctionName(_)
      | ParseError::BadParamEnd(_)
      | ParseError::ForgotSetKeyword(_)
      | ParseError::BadBinaryFunctionName(_)
      | ParseError::BadTemplateCallee(_)
      | ParseError::UnknownTupleOrSubExpression(_)
      | ParseError::BadRangeOperand(_)
      | ParseError::CantUseBreakInExpression(_)
      | ParseError::CantUseReturnInExpression(_)
      | ParseError::CantUseWhileInExpression(_)
      | ParseError::NeedSemicolon(_)
      | ParseError::DontNeedSemicolon(_)
      | ParseError::BadDot(_)
      | ParseError::CantTemplateCallMember(_) => "P1005",
      ParseError::NeedWhitespaceAroundBinaryOperator(_) => "P1006",
      ParseError::BadFunctionBodyError(_) => "P1006",
      ParseError::BadStartOfWhileCondition(_) => "P1007",
      ParseError::BadEndOfWhileCondition(_) => "P1008",
      ParseError::BadUnicodeChar(_)
      | ParseError::BadStringInterpolationEnd(_)
      | ParseError::BadStartOfBlock(_)
      | ParseError::BadEndOfBlock(_)
      | ParseError::BadStartOfWhileBody(_) => "P1009",
      ParseError::BadEndOfWhileBody(_) => "P1010",
      ParseError::BadStartOfIfCondition(_) => "P1011",
      ParseError::BadEndOfIfCondition(_) => "P1012",
      ParseError::BadStartOfIfBody(_) => "P1013",
      ParseError::BadEndOfIfBody(_) => "P1014",
      ParseError::BadStartOfElseBody(_) => "P1015",
      ParseError::BadEndOfElseBody(_) => "P1016",
      ParseError::BadLetEqualsError(_) => "P1017",
      ParseError::BadMutateEqualsError(_) => "P1018",
      ParseError::BadLetEndError(_) => "P1019",
      ParseError::BadVPSTError { .. } => "P1020",
      ParseError::BadArraySizerEnd(_) | ParseError::BadArraySizer(_) => "P1022",
      ParseError::BadStructName(_)
      | ParseError::BadInterfaceName(_)
      | ParseError::BadStructContentsBegin(_)
      | ParseError::BadStructContentsEnd(_)
      | ParseError::BadStructMember(_)
      | ParseError::BadStructMemberType(_)
      | ParseError::VariadicStructMemberHasName(_)
      | ParseError::BadInterfaceMember(_)
      | ParseError::BadInterfaceContentsBegin(_) => "P1027",
      ParseError::BadInterfaceHeader(_) => "P1028",
      ParseError::BadImpl(_)
      | ParseError::BadImplStruct(_)
      | ParseError::BadImplInterface(_)
      | ParseError::BadImplEnd(_)
      | ParseError::BadImplFor(_)
      | ParseError::BadExportAs(_) => "P1029",
      ParseError::BadImportEnd(_) | ParseError::BadImportName(_) => "P1031",
      ParseError::BadFunctionParamsBegin(_)
      | ParseError::BadFunctionAfterParam(_)
      | ParseError::BadAttributeError(_)
      | ParseError::BadExternAttribute(_) => "P1032",
      ParseError::BadExportName(_)
      | ParseError::BadExportEnd(_)
      | ParseError::BadForeachIterableError(_) => "P1037",
      ParseError::BadArraySpecifier(_) => "P1039",
      ParseError::BadLocalName(_)
      | ParseError::BadThingAfterTypeInPattern(_)
      | ParseError::BadForeachInError(_) => "P1041",
      ParseError::BadLetSourceError { .. } => "P1042",
      _ => "P1001", // Default for all others
    }
  }
}

/*

package dev.vale.lexing

import dev.vale.{FileCoordinate, vcurious, vpass}

case class FailedParse(
  code: String,
  fileCoord: FileCoordinate,
  error: IParseError,
) { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
sealed trait IParseError {
  def pos: Int
  def errorId: String
}
case class RangedInternalErrorP(pos: Int, msg: String) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class UnrecognizableExpressionAfterAugment(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class OnlyRegionRunesCanHaveMutability(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadMemberEnd(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadLambdaBegin(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadLambdaBodyBegin(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class EmptyParameter(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class CantUseThatLocalName(pos: Int, name: String) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class LightFunctionMustHaveParamTypes(pos: Int, paramIndex: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class EmptyPattern(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadNameBeforeDestructure(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadLocalNameInUnlet(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class FoundBothAbstractAndOverride(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class FoundBothImmutableAndMutabilityInArray(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStringInTemplex(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadPrototypeName(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadPrototypeParams(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadRuleCallParam(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadTypeExpression(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadTemplateCallParam(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadTupleElement(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadDestructureError(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class ShareCantBeReadwrite(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadRuneEnd(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadRegionName(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadRuneNameError(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class RegionRuneHasType(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadRuneTypeError(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class UnrecognizedDenizenError(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStartOfStatementError(pos: Int) extends IParseError { override def errorId: String = "P1002";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadExpressionEnd(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadRule(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class UnexpectedAttributes(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class IfBlocksMustBothOrNeitherReturn(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadExpressionBegin(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStringChar(stringBeginPos: Int, pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadFunctionName(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadParamEnd(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class ForgotSetKeyword(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class FuncBoundWithoutWhere(pos: Int) extends IParseError { override def errorId: String = "P1001";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadBinaryFunctionName(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadTemplateCallee(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class UnknownTupleOrSubExpression(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadRangeOperand(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class CantUseBreakInExpression(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class CantUseReturnInExpression(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class CantUseWhileInExpression(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class NeedSemicolon(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class DontNeedSemicolon(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadDot(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class CantTemplateCallMember(pos: Int) extends IParseError { override def errorId: String = "P1005";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class NeedWhitespaceAroundBinaryOperator(pos: Int) extends IParseError { override def errorId: String = "P1006";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadFunctionBodyError(pos: Int) extends IParseError { override def errorId: String = "P1006";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadUnicodeChar(pos: Int) extends IParseError { override def errorId: String = "P1009";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStringInterpolationEnd(pos: Int) extends IParseError { override def errorId: String = "P1009";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStartOfBlock(pos: Int) extends IParseError { override def errorId: String = "P1009";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadEndOfBlock(pos: Int) extends IParseError { override def errorId: String = "P1009";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStartOfWhileCondition(pos: Int) extends IParseError { override def errorId: String = "P1007";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadEndOfWhileCondition(pos: Int) extends IParseError { override def errorId: String = "P1008";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStartOfWhileBody(pos: Int) extends IParseError { override def errorId: String = "P1009";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadEndOfWhileBody(pos: Int) extends IParseError { override def errorId: String = "P1010";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStartOfIfCondition(pos: Int) extends IParseError { override def errorId: String = "P1011";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadEndOfIfCondition(pos: Int) extends IParseError { override def errorId: String = "P1012";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStartOfIfBody(pos: Int) extends IParseError { override def errorId: String = "P1013";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadEndOfIfBody(pos: Int) extends IParseError { override def errorId: String = "P1014";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStartOfElseBody(pos: Int) extends IParseError { override def errorId: String = "P1015";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadEndOfElseBody(pos: Int) extends IParseError { override def errorId: String = "P1016";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadLetEqualsError(pos: Int) extends IParseError { override def errorId: String = "P1017";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadMutateEqualsError(pos: Int) extends IParseError { override def errorId: String = "P1018";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadLetEndError(pos: Int) extends IParseError { override def errorId: String = "P1019";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadVPSTException(err: BadVPSTError) extends RuntimeException
case class BadVPSTError(message: String) extends IParseError {
  override def pos = 0;
override def errorId: String = "P1020";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}

// TODO: Get rid of all the below when we've migrated away from combinators.

case class BadArraySizerEnd(pos: Int) extends IParseError { override def errorId: String = "P1022";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadArraySizer(pos: Int) extends IParseError { override def errorId: String = "P1022";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStructName(pos: Int) extends IParseError { override def errorId: String = "P1027";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadInterfaceName(pos: Int) extends IParseError { override def errorId: String = "P1027";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStructContentsBegin(pos: Int) extends IParseError { override def errorId: String = "P1027";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStructContentsEnd(pos: Int) extends IParseError { override def errorId: String = "P1027";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStructMember(pos: Int) extends IParseError { override def errorId: String = "P1027";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadStructMemberType(pos: Int) extends IParseError { override def errorId: String = "P1027";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class VariadicStructMemberHasName(pos: Int) extends IParseError { override def errorId: String = "P1027";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadInterfaceMember(pos: Int) extends IParseError { override def errorId: String = "P1027";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadInterfaceContentsBegin(pos: Int) extends IParseError { override def errorId: String = "P1027";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadInterfaceHeader(pos: Int) extends IParseError { override def errorId: String = "P1028";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadImpl(pos: Int) extends IParseError { override def errorId: String = "P1029";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadImplStruct(pos: Int) extends IParseError { override def errorId: String = "P1029";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadImplInterface(pos: Int) extends IParseError { override def errorId: String = "P1029";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadImplEnd(pos: Int) extends IParseError { override def errorId: String = "P1029";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadImplFor(pos: Int) extends IParseError { override def errorId: String = "P1029";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadExportAs(pos: Int) extends IParseError { override def errorId: String = "P1029";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadImportEnd(pos: Int) extends IParseError { override def errorId: String = "P1031";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadImportName(pos: Int) extends IParseError { override def errorId: String = "P1031";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadFunctionParamsBegin(pos: Int) extends IParseError { override def errorId: String = "P1032";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadFunctionAfterParam(pos: Int) extends IParseError { override def errorId: String = "P1032";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadAttributeError(pos: Int) extends IParseError { override def errorId: String = "P1032";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadExportName(pos: Int) extends IParseError { override def errorId: String = "P1037";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadExportEnd(pos: Int) extends IParseError { override def errorId: String = "P1037";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadForeachIterableError(pos: Int) extends IParseError { override def errorId: String = "P1037";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadArraySpecifier(pos: Int) extends IParseError { override def errorId: String = "P1039";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadLocalName(pos: Int) extends IParseError { override def errorId: String = "P1041";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadThingAfterTypeInPattern(pos: Int) extends IParseError { override def errorId: String = "P1041";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadLetSourceError(pos: Int, cause: IParseError) extends IParseError { override def errorId: String = "P1042";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class BadForeachInError(pos: Int) extends IParseError { override def errorId: String = "P1041";
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
case class InputException(message: String) extends Throwable {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  override def toString: String = message
}
*/
