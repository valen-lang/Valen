use crate::postparsing::names::{INameS, IVarNameS};
use crate::postparsing::post_parser::ICompileErrorS;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::names::IRuneS;
use crate::postparsing::rules::rules::IRulexSR;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::rules::rules::ILiteralSL;
use crate::parsing::ast::MutabilityP;
use crate::parsing::ast::VariabilityP;
use crate::parsing::ast::OwnershipP;
use crate::postparsing::rules::rules::RuneUsage;
use crate::postparsing::rune_type_solver::IRuneTypeRuleError;
/*
package dev.vale.postparsing

import dev.vale.{CodeLocationS, FileCoordinateMap, RangeS, vimpl}
import dev.vale.postparsing.rules._
import dev.vale.solver.SolverErrorHumanizer
import dev.vale.SourceCodeUtils.{humanizePos, lineContaining, nextThingAndRestOfLine}
import dev.vale.parsing.ast.{BorrowP, FinalP, ImmutableP, MutabilityP, MutableP, OwnP, OwnershipP, ShareP, VariabilityP, VaryingP, WeakP}
import dev.vale.parsing.ast._
import dev.vale.postparsing.rules._

object PostParserErrorHumanizer {
*/

pub fn humanize<'s, HP, LB, LRC, LC>(
  humanize_pos: HP,
  _lines_between: LB,
  _line_range_containing: LRC,
  line_containing: LC,
  err: &'s ICompileErrorS<'s>,
) -> String
where
  HP: Fn(&CodeLocationS<'s>) -> String,
  LB: Fn(&CodeLocationS<'s>, &CodeLocationS<'s>) -> Vec<RangeS<'s>>,
  LRC: Fn(&CodeLocationS<'s>) -> RangeS<'s>,
  LC: Fn(&CodeLocationS<'s>) -> String,
{
  let error_str_body = match err {
    ICompileErrorS::VariableNameAlreadyExists(x) => {
      format!(
        "Local named {} already exists!\n(If you meant to modify the variable, use the `set` keyword beforehand.)",
        humanize_var_name(x.name.clone())
      )
    }
    ICompileErrorS::InterfaceMethodNeedsSelf(_) => {
      "Interface's method needs a virtual param of interface's type!".to_string()
    }
    ICompileErrorS::VirtualAndAbstractGoTogether(_) => {
      "Abstract function needs a `virtual` parameter.".to_string()
    }
    ICompileErrorS::ExternHasBodyS(_) => "Extern function can't have a body too.".to_string(),
    ICompileErrorS::IdentifyingRunesIncompleteS(_) => {
      "Not enough identifying runes.".to_string()
    }
    _ => panic!("Unimplemented humanize branch for {:?}", err),
  };
  let range = err.range();
  let pos_str = humanize_pos(&range.begin);
  let next_stuff = line_containing(&range.begin);
  let error_id = "S";
  format!("{} error {}: {}\n{}\n", pos_str, error_id, error_str_body, next_stuff)
}
/*
  def humanize(
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
    err: ICompileErrorS):
  String = {
    val errorStrBody =
      (err match {
        case RuneExplicitTypeConflictS(range, rune, types) => "Too many explicit types for rune " + humanizeRune(rune) + "" + types.map(humanizeTemplataType).mkString(", ")
        case RangedInternalErrorS(range, message) => " " + message
        case UnknownRuleFunctionS(range, name) => "Unknown rule function name: "+ name
        case UnknownRegionError(range, name) => "Unknown region: " + name
        case UnimplementedExpression(range, expressionName) => s"${expressionName} not supported yet.\n"
        case CouldntFindRuneS(range, name) => "Couldn't find generic parameter \"" + name + "\".\n"
        case CouldntFindVarToMutateS(range, name) => s"No variable named ${name}. Try declaring it above, like `${name} = 42;`\n"
        case CantOwnershipInterfaceInImpl(range) => s"Can only impl a plain interface, remove symbol."
        case CantOwnershipStructInImpl(range) => s"Only a plain struct/interface can be in an impl, remove symbol."
        case CantOverrideOwnershipped(range) => s"Can only impl a plain interface, remove symbol."
        case BadRuneAttributeErrorS(range, attr) => "Bad rune attribute: " + attr
        case CouldntSolveRulesS(range, error) => {
          s"Couldn't solve:\n" +
          SolverErrorHumanizer.humanizeFailedSolve[IRulexSR, IRuneS, ITemplataType, IRuneTypeRuleError](
            codeMap,
            linesBetween,
            lineRangeContaining,
            lineContaining,
            PostParserErrorHumanizer.humanizeRune,
            (tyype: ITemplataType) => tyype.toString,
            (u: IRuneTypeRuleError) => humanizeRuneTypeError(codeMap, u),
            (rule: IRulexSR) => rule.range,
            (rule: IRulexSR) => rule.runeUsages.map(u => (u.rune, u.range)),
            (rule: IRulexSR) => rule.runeUsages.map(_.rune),
            PostParserErrorHumanizer.humanizeRule,
            error.failedSolve)._1
        }
        case IdentifyingRunesIncompleteS(range, error) => {
          s"Not enough identifying runes:\n" +
            SolverErrorHumanizer.humanizeFailedSolve[IRulexSR, IRuneS, Boolean, IIdentifiabilityRuleError](
              codeMap,
              linesBetween,
              lineRangeContaining,
              lineContaining,
              PostParserErrorHumanizer.humanizeRune,
              (identified: Boolean) => identified.toString,
              (u: IIdentifiabilityRuleError) => humanizeIdentifiabilityRuleErrorr(codeMap, u),
              (rule: IRulexSR) => rule.range,
              (rule: IRulexSR) => rule.runeUsages.map(u => (u.rune, u.range)),
              (rule: IRulexSR) => rule.runeUsages.map(_.rune),
              PostParserErrorHumanizer.humanizeRule,
              error.failedSolve)._1
        }
        case VariableNameAlreadyExists(range, name) => s"Local named " + humanizeName(name) + " already exists!\n(If you meant to modify the variable, use the `set` keyword beforehand.)"
        case InterfaceMethodNeedsSelf(range) => s"Interface's method needs a virtual param of interface's type!"
        case VirtualAndAbstractGoTogether(range) => s"Abstract function needs a `virtual` parameter."
        case ForgotSetKeywordError(range) => s"Changing a struct's member must start with the `set` keyword."
        case ExternHasBody(range) => s"Extern function can't have a body too."
//        case CantInitializeIndividualElementsOfRuntimeSizedArray(range) => s"Can't initialize individual elements of a runtime-sized array."
        case InitializingRuntimeSizedArrayRequiresSizeAndCallable(range) => s"Initializing a runtime-sized array requires 1-2 arguments: a capacity, and optionally a function that will populate that many elements."
        case InitializingStaticSizedArrayRequiresSizeAndCallable(range) => s"Initializing a statically-sized array requires one argument: a function that will populate the elements."
//        case InitializingStaticSizedArrayFromCallableNeedsSizeTemplex(range) => s"Initializing a statically-sized array requires a size in-between the square brackets."
      })

    val posStr = codeMap(err.range.begin)
    val nextStuff = lineContaining(err.range.begin)
    val errorId = "S"
    f"${posStr} error ${errorId}: ${errorStrBody}\n${nextStuff}\n"
  }
*/
pub fn humanize_rune_type_error<'s>(
  _code_map: &dyn Fn(CodeLocationS<'s>) -> String,
  error: &IRuneTypeRuleError<'s>,
) -> String {
  match error {
    IRuneTypeRuleError::FoundTemplataDidntMatchExpectedType(_) => panic!("implement: humanize_rune_type_error FoundTemplataDidntMatchExpectedType"),
    IRuneTypeRuleError::CouldntFindType(e) => {
      format!("Couldn't find anything with the name '{}'", humanize_imprecise_name(e.name))
    }
    IRuneTypeRuleError::NotEnoughArgumentsForGenericCall(_) => panic!("implement: humanize_rune_type_error NotEnoughArgumentsForGenericCall"),
    _ => panic!("implement: humanize_rune_type_error other"),
  }
}
/*
  def humanizeRuneTypeError(
    codeMap: CodeLocationS => String,
    error: IRuneTypeRuleError):
  String = {
    error match {
      case FoundTemplataDidntMatchExpectedType(range, expectedType, actualType) => {
        "Expected " + humanizeTemplataType(expectedType) + " but found " + humanizeTemplataType(actualType)
      }
      case RuneTypingCouldntFindType(range, name) => {
        "Couldn't find anything with the name '" + humanizeImpreciseName(name) + "'"
      }
      case NotEnoughArgumentsForGenericCall(range, indexOfNonDefaultingParam) => {
        "Not enough arguments for generic call, expected at least " + (indexOfNonDefaultingParam + 1)
      }
      case FoundPrimitiveDidntMatchExpectedType(range, expectedType, actualType) => {
        "Found primitive didnt match expected type. Expected " + humanizeTemplataType(expectedType) + " but was " + humanizeTemplataType(actualType)
      }
      case other => vimpl(other)
    }
  }
*/
fn humanize_identifiability_rule_errorr<'s>(
  _error: &(),
) -> String {
  panic!("Unimplemented humanize_identifiability_rule_errorr");
}
/*
  def humanizeIdentifiabilityRuleErrorr(
    codeMap: CodeLocationS => String,
    error: IIdentifiabilityRuleError):
  String = {
    error match {
      case other => vimpl(other)
    }
  }
*/
fn humanize_var_name<'s>(var_name: IVarNameS<'s>) -> String {
  match var_name {
    IVarNameS::CodeVarName(n) => n.as_str().to_string(),
    IVarNameS::ClosureParamName(_) => "(closure)".to_string(),
    _ => panic!("Unimplemented humanize_var_name branch for IVarNameS"),
  }
}

fn humanize_name<'s>(name: INameS<'s>) -> String {
  match name {
    INameS::VarName(var_name) => humanize_var_name((*var_name).clone()),
    _ => panic!("Unimplemented humanize_name branch for INameS"),
  }
}
/*
  def humanizeName(name: INameS): String = {
    name match {
//      case UnnamedLocalNameS(codeLocation) => "(unnamed)"
      case ClosureParamNameS(_) => "(closure)"
//      case FreeDeclarationNameS(_) => "(free)"
//      case CodeNameS(n) => n
      case GlobalFunctionFamilyNameS(n) => n
//      case DropNameS(_) => "(drop)"
      case MagicParamNameS(codeLocation) => "(magic)"
      case ConstructorNameS(inner) => "constructor<" + humanizeName(inner) + ">"
      case CodeVarNameS(name) => name.str
      case ArbitraryNameS() => "(arbitrary)"
      case RuneNameS(rune) => humanizeRune(rune)
      case ConstructingMemberNameS(name) => "member " + name
      case FunctionNameS(name, codeLocation) => name.str
      case AnonymousSubstructTemplateNameS(tlcd) => humanizeName(tlcd) + ".anonymous"
      case AnonymousSubstructImplDeclarationNameS(interface) => humanizeName(interface) + ".anonimpl"
      case ForwarderFunctionDeclarationNameS(inner, index) => humanizeName(inner) + ".forwarder" + index
      case TopLevelCitizenDeclarationNameS(name, range) => name.str
      case RuntimeSizedArrayDeclarationNameS() => "__rsa"
      case ImplDeclarationNameS(_) => "(impl)"
    }
  }
*/
pub fn humanize_imprecise_name<'s>(
  name: IImpreciseNameS<'s>,
) -> String {
  match name {
    IImpreciseNameS::ArbitraryName(_) => "_arby".to_string(),
    IImpreciseNameS::SelfName(_) => "_Self".to_string(),
    IImpreciseNameS::CodeName(n) => n.name.0.to_string(),
    IImpreciseNameS::RuneName(rune) => humanize_rune(rune.rune),
    IImpreciseNameS::AnonymousSubstructTemplateImpreciseName(_) => panic!("implement: humanize_imprecise_name AnonymousSubstructTemplateImpreciseName"),
    IImpreciseNameS::LambdaStructImpreciseName(_) => panic!("implement: humanize_imprecise_name LambdaStructImpreciseName"),
    IImpreciseNameS::LambdaImpreciseName(_) => "_Lam".to_string(),
    _ => panic!("implement: humanize_imprecise_name other"),
  }
}
/*
  def humanizeImpreciseName(name: IImpreciseNameS): String = {
    name match {
      case ArbitraryNameS() => "_arby"
      case SelfNameS() => "_Self"
      case CodeNameS(n) => n.str
//      case FreeImpreciseNameS() => "_Free"
      case RuneNameS(rune) => humanizeRune(rune)
      case AnonymousSubstructTemplateImpreciseNameS(interfaceHumanName) => humanizeImpreciseName(interfaceHumanName) + "._AnonSub"
      case LambdaStructImpreciseNameS(lambdaName) => humanizeImpreciseName(lambdaName) + ".struct"
      case LambdaImpreciseNameS() => "_Lam"
//      case VirtualFreeImpreciseNameS() => "(abstract virtual free)"
//      case VirtualFreeImpreciseNameS() => "(override virtual free)"
    }
  }
*/
pub fn humanize_rune<'s>(
  rune: IRuneS<'s>,
) -> String {
  match rune {
    IRuneS::ImplicitRune(r) => "_".to_string() + &r.lid.path.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(""),
    IRuneS::MagicParamRune(_) => panic!("implement: humanize_rune MagicParamRune"),
    IRuneS::CodeRune(r) => r.name.0.to_string(),
    IRuneS::ArgumentRune(_) => panic!("implement: humanize_rune ArgumentRune"),
    IRuneS::SelfKindRune(_) => panic!("implement: humanize_rune SelfKindRune"),
    IRuneS::SelfOwnershipRune(_) => panic!("implement: humanize_rune SelfOwnershipRune"),
    IRuneS::SelfKindTemplateRune(_) => panic!("implement: humanize_rune SelfKindTemplateRune"),
    IRuneS::PatternInputRune(_) => panic!("implement: humanize_rune PatternInputRune"),
    IRuneS::SelfRune(_) => panic!("implement: humanize_rune SelfRune"),
    IRuneS::SelfCoordRune(_) => panic!("implement: humanize_rune SelfCoordRune"),
    IRuneS::ReturnRune(_) => panic!("implement: humanize_rune ReturnRune"),
    IRuneS::AnonymousSubstructParentInterfaceTemplateRune(_) => panic!("implement: humanize_rune AnonymousSubstructParentInterfaceTemplateRune"),
    IRuneS::ImplDropVoidRune(_) => panic!("implement: humanize_rune ImplDropVoidRune"),
    IRuneS::ImplDropCoordRune(_) => panic!("implement: humanize_rune ImplDropCoordRune"),
    IRuneS::FreeOverrideInterfaceRune(_) => panic!("implement: humanize_rune FreeOverrideInterfaceRune"),
    IRuneS::FreeOverrideStructRune(_) => panic!("implement: humanize_rune FreeOverrideStructRune"),
    IRuneS::AnonymousSubstructKindRune(_) => panic!("implement: humanize_rune AnonymousSubstructKindRune"),
    IRuneS::AnonymousSubstructCoordRune(_) => panic!("implement: humanize_rune AnonymousSubstructCoordRune"),
    IRuneS::AnonymousSubstructTemplateRune(_) => panic!("implement: humanize_rune AnonymousSubstructTemplateRune"),
    IRuneS::AnonymousSubstructParentInterfaceKindRune(_) => panic!("implement: humanize_rune AnonymousSubstructParentInterfaceKindRune"),
    IRuneS::AnonymousSubstructParentInterfaceCoordRune(_) => panic!("implement: humanize_rune AnonymousSubstructParentInterfaceCoordRune"),
    IRuneS::StructNameRune(_) => panic!("implement: humanize_rune StructNameRune"),
    IRuneS::FreeOverrideStructTemplateRune(_) => panic!("implement: humanize_rune FreeOverrideStructTemplateRune"),
    IRuneS::FunctorPrototypeRuneName(_) => panic!("implement: humanize_rune FunctorPrototypeRuneName"),
    IRuneS::MacroSelfKindRune(_) => panic!("implement: humanize_rune MacroSelfKindRune"),
    IRuneS::MacroSelfCoordRune(_) => panic!("implement: humanize_rune MacroSelfCoordRune"),
    IRuneS::MacroVoidKindRune(_) => panic!("implement: humanize_rune MacroVoidKindRune"),
    IRuneS::MacroVoidCoordRune(_) => panic!("implement: humanize_rune MacroVoidCoordRune"),
    IRuneS::MacroSelfKindTemplateRune(_) => panic!("implement: humanize_rune MacroSelfKindTemplateRune"),
    IRuneS::AnonymousSubstructMemberRune(_) => panic!("implement: humanize_rune AnonymousSubstructMemberRune"),
    IRuneS::AnonymousSubstructFunctionBoundParamsListRune(_) => panic!("implement: humanize_rune AnonymousSubstructFunctionBoundParamsListRune"),
    IRuneS::AnonymousSubstructFunctionBoundPrototypeRune(_) => panic!("implement: humanize_rune AnonymousSubstructFunctionBoundPrototypeRune"),
    IRuneS::AnonymousSubstructFunctionInterfaceTemplateRune(_) => panic!("implement: humanize_rune AnonymousSubstructFunctionInterfaceTemplateRune"),
    IRuneS::AnonymousSubstructFunctionInterfaceKindRune(_) => panic!("implement: humanize_rune AnonymousSubstructFunctionInterfaceKindRune"),
    IRuneS::AnonymousSubstructDropBoundParamsListRune(_) => panic!("implement: humanize_rune AnonymousSubstructDropBoundParamsListRune"),
    IRuneS::AnonymousSubstructDropBoundPrototypeRune(_) => panic!("implement: humanize_rune AnonymousSubstructDropBoundPrototypeRune"),
    IRuneS::AnonymousSubstructMethodInheritedRune(_) => panic!("implement: humanize_rune AnonymousSubstructMethodInheritedRune"),
    IRuneS::AnonymousSubstructMethodSelfOwnCoordRune(_) => panic!("implement: humanize_rune AnonymousSubstructMethodSelfOwnCoordRune"),
    IRuneS::AnonymousSubstructMethodSelfBorrowCoordRune(_) => panic!("implement: humanize_rune AnonymousSubstructMethodSelfBorrowCoordRune"),
    IRuneS::DenizenDefaultRegionRune(_) => panic!("implement: humanize_rune DenizenDefaultRegionRune"),
    IRuneS::ExternDefaultRegionRune(_) => panic!("implement: humanize_rune ExternDefaultRegionRune"),
    IRuneS::AnonymousSubstructVoidKindRune(_) => panic!("implement: humanize_rune AnonymousSubstructVoidKindRune"),
    IRuneS::AnonymousSubstructVoidCoordRune(_) => panic!("implement: humanize_rune AnonymousSubstructVoidCoordRune"),
    IRuneS::ImplicitCoercionOwnershipRune(_) => panic!("implement: humanize_rune ImplicitCoercionOwnershipRune"),
    IRuneS::ImplicitCoercionKindRune(_) => panic!("implement: humanize_rune ImplicitCoercionKindRune"),
    IRuneS::ImplicitCoercionTemplateRune(_) => panic!("implement: humanize_rune ImplicitCoercionTemplateRune"),
    IRuneS::ImplicitRegionRune(_) => panic!("implement: humanize_rune ImplicitRegionRune"),
    IRuneS::CallRegionRune(_) => panic!("implement: humanize_rune CallRegionRune"),
    IRuneS::CaseRuneFromImpl(_) => panic!("implement: humanize_rune CaseRuneFromImpl"),
    IRuneS::DispatcherRuneFromImpl(_) => panic!("implement: humanize_rune DispatcherRuneFromImpl"),
    IRuneS::PureBlockRegionRune(_) => panic!("implement: humanize_rune PureBlockRegionRune"),
    IRuneS::CallPureMergeRegionRune(_) => panic!("implement: humanize_rune CallPureMergeRegionRune"),
    IRuneS::ReachablePrototypeRune(_) => panic!("implement: humanize_rune ReachablePrototypeRune"),
    IRuneS::MemberRune(_) => panic!("implement: humanize_rune MemberRune"),
    IRuneS::LocalDefaultRegionRune(_) => panic!("implement: humanize_rune LocalDefaultRegionRune"),
    IRuneS::ExportDefaultRegionRune(_) => panic!("implement: humanize_rune ExportDefaultRegionRune"),
    IRuneS::ArraySizeImplicitRune(_) => panic!("implement: humanize_rune ArraySizeImplicitRune"),
    IRuneS::ArrayMutabilityImplicitRune(_) => panic!("implement: humanize_rune ArrayMutabilityImplicitRune"),
    IRuneS::ArrayVariabilityImplicitRune(_) => panic!("implement: humanize_rune ArrayVariabilityImplicitRune"),
    IRuneS::InterfaceNameRune(_) => panic!("implement: humanize_rune InterfaceNameRune"),
    IRuneS::LetImplicitRune(_) => panic!("implement: humanize_rune LetImplicitRune"),
    IRuneS::ExplicitTemplateArgRune(_) => panic!("implement: humanize_rune ExplicitTemplateArgRune"),
    IRuneS::FunctorParamRuneName(_) => panic!("implement: humanize_rune FunctorParamRuneName"),
    IRuneS::FunctorReturnRuneName(_) => panic!("implement: humanize_rune FunctorReturnRuneName"),
  }
}
/*
  def humanizeRune(rune: IRuneS): String = {
    rune match {
      case ImplicitRuneS(lid) => "_" + lid.path.mkString("")
      case MagicParamRuneS(lid) => "_" + lid.path.mkString("")
      case CodeRuneS(name) => name.str
      case ArgumentRuneS(paramIndex) => "(arg " + paramIndex + ")"
      case SelfKindRuneS() => "(self kind)"
      case SelfOwnershipRuneS() => "(self ownership)"
      case SelfKindTemplateRuneS(_) => "(self kind template)"
      case PatternInputRuneS(codeLoc) => "(pattern input " + codeLoc + ")"
      case SelfRuneS() => "(self)"
      case SelfCoordRuneS() => "(self ref)"
      case ReturnRuneS() => "(ret)"
      case AnonymousSubstructParentInterfaceTemplateRuneS() => "(anon sub parent interface template)"
      case ImplDropVoidRuneS() => "(impl drop void)"
      case ImplDropCoordRuneS() => "(impl drop coord)"
      case FreeOverrideInterfaceRuneS() => "(freeing interface)"
      case FreeOverrideStructRuneS() => "(freeing struct)"
      case AnonymousSubstructKindRuneS() => "(anon substruct kind)"
      case AnonymousSubstructCoordRuneS() => "(anon substruct ref)"
      case AnonymousSubstructTemplateRuneS() => "(anon substruct template)"
      case AnonymousSubstructParentInterfaceTemplateRuneS() => "(anon sub parent interface template)"
      case AnonymousSubstructParentInterfaceKindRuneS() => "(anon sub parent kind)"
      case AnonymousSubstructParentInterfaceCoordRuneS() => "(anon sub parent ref)"
      case StructNameRuneS(inner) => humanizeName(inner)
      case FreeOverrideStructTemplateRuneS() => "(free override template)"
      case FunctorPrototypeRuneNameS() => "(functor prototype)"
      case MacroSelfKindRuneS() => "_MSelfK"
      case MacroSelfCoordRuneS() => "_MSelf"
      case MacroVoidKindRuneS() => "_MVoidK"
      case MacroVoidCoordRuneS() => "_MVoid"
      case MacroSelfKindTemplateRuneS() => "_MSelfKT"
      case AnonymousSubstructMemberRuneS(interface, method) => "$" + humanizeName(interface) + ".anon." + humanizeName(method) + ".functor"
      case AnonymousSubstructFunctionBoundParamsListRuneS(interface, method) => "$" + humanizeName(interface) + ".anon." + humanizeName(method) + ".params"
      case AnonymousSubstructFunctionBoundPrototypeRuneS(interface, method) => "$" + humanizeName(interface) + ".anon." + humanizeName(method) + ".proto"
      case AnonymousSubstructFunctionInterfaceTemplateRune(interface, method) => "$" + humanizeName(interface) + ".anon." + humanizeName(method) + ".itemplate"
     case AnonymousSubstructFunctionInterfaceKindRune(interface, method) => "$" + humanizeName(interface) + ".anon." + humanizeName(method) + ".ikind"
//      case AnonymousSubstructFunctionInterfaceOwnershipRune(interface, method) => "$" + humanizeName(interface) + ".anon." + humanizeName(method) + ".iown"
      case AnonymousSubstructDropBoundParamsListRuneS(interface, method) => "$" + humanizeName(interface) + ".anon.drop.params"
      case AnonymousSubstructDropBoundPrototypeRuneS(interface, method) => "$" + humanizeName(interface) + ".anon.drop.proto"
      case AnonymousSubstructMethodInheritedRuneS(interface, method, inner) => "$" + humanizeName(interface) + ".anon." + humanizeName(method) + ":" + humanizeRune(inner)
      case AnonymousSubstructMethodSelfOwnCoordRuneS(interface, method) => "$" + humanizeName(interface) + ".anon." + humanizeName(method) + ".ownself"
      case AnonymousSubstructMethodSelfBorrowCoordRuneS(interface, method) => "$" + humanizeName(interface) + ".anon." + humanizeName(method) + ".borrowself"
      case DenizenDefaultRegionRuneS(denizenName) => humanizeName(denizenName) + "'"
      case ExternDefaultRegionRuneS(denizenName) => humanizeName(denizenName) + "'"
      case AnonymousSubstructVoidKindRuneS() => "anon.void.kind"
      case AnonymousSubstructVoidCoordRuneS() => "anon.void"
      case ImplicitCoercionOwnershipRuneS(_, inner) => humanizeRune(inner) + ".own"
      case ImplicitCoercionKindRuneS(_, inner) => humanizeRune(inner) + ".kind"
      case ImplicitCoercionTemplateRuneS(_, inner) => humanizeRune(inner) + ".gen"
      case ImplicitRegionRuneS(originalRune) => humanizeRune(originalRune) + ".region"
      case CallRegionRuneS(lid) => "_" + lid.path.mkString("") + ".pcall"
      case CaseRuneFromImplS(innerRune) => "case:" + humanizeRune(innerRune)
      case DispatcherRuneFromImplS(innerRune) => "disimpl:" + humanizeRune(innerRune)
      case other => vimpl(other)
    }
  }
*/
pub fn humanize_templata_type(
  _tyype: &ITemplataType,
) -> String {
  panic!("Unimplemented humanize_templata_type");
}
/*
  def humanizeTemplataType(tyype: ITemplataType): String = {
    tyype match {
      case KindTemplataType() => "Kind"
      case CoordTemplataType() => "Type"
      case FunctionTemplataType() => "Func"
      case IntegerTemplataType() => "Int"
      case RegionTemplataType() => "Region"
      case BooleanTemplataType() => "Bool"
      case MutabilityTemplataType() => "Mut"
      case PrototypeTemplataType() => "Prot"
      case StringTemplataType() => "Str"
      case LocationTemplataType() => "Loc"
      case OwnershipTemplataType() => "Own"
      case VariabilityTemplataType() => "Vary"
      case PackTemplataType(elementType) => "Pack<" + humanizeTemplataType(elementType) + ">"
      case TemplateTemplataType(params, ret) => humanizeTemplataType(ret) + "<" + params.map(humanizeTemplataType).mkString(",") + ">"
    }
  }
*/
pub fn humanize_rule<'s>(
  rule: &IRulexSR<'s>,
) -> String {
  match rule {
    IRulexSR::KindComponents(r) => {
      humanize_rune(r.kind_rune.rune) + " = Kind[" + &humanize_rune(r.mutability_rune.rune) + "]"
    }
    IRulexSR::CoordComponents(r) => {
      humanize_rune(r.result_rune.rune) + " = Ref[" + &humanize_rune(r.ownership_rune.rune) + ", " + &humanize_rune(r.kind_rune.rune) + "]"
    }
    IRulexSR::PrototypeComponents(_) => panic!("implement: humanize_rule PrototypeComponents"),
    IRulexSR::OneOf(_) => panic!("implement: humanize_rule OneOf"),
    IRulexSR::IsInterface(_) => panic!("implement: humanize_rule IsInterface"),
    IRulexSR::IsStruct(_) => panic!("implement: humanize_rule IsStruct"),
    IRulexSR::RefListCompoundMutability(_) => panic!("implement: humanize_rule RefListCompoundMutability"),
    IRulexSR::DefinitionCoordIsa(_) => panic!("implement: humanize_rule DefinitionCoordIsa"),
    IRulexSR::CallSiteCoordIsa(_) => panic!("implement: humanize_rule CallSiteCoordIsa"),
    IRulexSR::CoordSend(_) => panic!("implement: humanize_rule CoordSend"),
    IRulexSR::CoerceToCoord(_) => panic!("implement: humanize_rule CoerceToCoord"),
    IRulexSR::MaybeCoercingCall(_) => panic!("implement: humanize_rule MaybeCoercingCall"),
    IRulexSR::MaybeCoercingLookup(r) => humanize_rune(r.rune.rune) + " = \"" + &humanize_imprecise_name(r.name) + "\"",
    IRulexSR::Call(_) => panic!("implement: humanize_rule Call"),
    IRulexSR::Lookup(_) => panic!("implement: humanize_rule Lookup"),
    IRulexSR::Literal(_) => panic!("implement: humanize_rule Literal"),
    IRulexSR::Augment(_) => panic!("implement: humanize_rule Augment"),
    IRulexSR::Equals(_) => panic!("implement: humanize_rule Equals"),
    IRulexSR::RuneParentEnvLookup(_) => panic!("implement: humanize_rule RuneParentEnvLookup"),
    IRulexSR::Pack(_) => panic!("implement: humanize_rule Pack"),
    IRulexSR::Resolve(_) => panic!("implement: humanize_rule Resolve"),
    IRulexSR::CallSiteFunc(_) => panic!("implement: humanize_rule CallSiteFunc"),
    IRulexSR::DefinitionFunc(_) => panic!("implement: humanize_rule DefinitionFunc"),
    other => panic!("vimpl humanize_rule: {:?}", other),
  }
}
/*
  def humanizeRule(rule: IRulexSR): String = {
    rule match {
      case KindComponentsSR(range, kindRune, mutabilityRune) => {
        humanizeRune(kindRune.rune) + " = Kind[" + humanizeRune(mutabilityRune.rune) + "]"
      }
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => {
        humanizeRune(resultRune.rune) + " = Ref[" + humanizeRune(ownershipRune.rune) + ", " + humanizeRune(kindRune.rune) + "]"
      }
      case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => {
        humanizeRune(resultRune.rune) + " = Prot[" + humanizeRune(paramsRune.rune) + ", " + humanizeRune(returnRune.rune) + "]"
      }
      case OneOfSR(range, resultRune, literals) => {
        humanizeRune(resultRune.rune) + " = " + literals.map(_.toString).mkString(" | ")
      }
      case IsInterfaceSR(range, resultRune) => "isInterface(" + humanizeRune(resultRune.rune) + ")"
      case IsStructSR(range, resultRune) => "isStruct(" + humanizeRune(resultRune.rune) + ")"
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => humanizeRune(resultRune.rune) + " = refListCompoundMutability(" + humanizeRune(coordListRune.rune) + ")"
      case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => humanizeRune(resultRune.rune) + " = " + humanizeRune(subRune.rune) + " def-isa " + humanizeRune(superRune.rune)
      case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) => resultRune.map(r => humanizeRune(r.rune)).getOrElse("_") + " = " + humanizeRune(subRune.rune) + " call-isa " + humanizeRune(superRune.rune)
      case CoordSendSR(range, senderRune, receiverRune) => humanizeRune(senderRune.rune) + " -> " + humanizeRune(receiverRune.rune)
      case CoerceToCoordSR(range, coordRune, kindRune) => "coerceToCoord(" + humanizeRune(coordRune.rune) + ", " + humanizeRune(kindRune.rune) + ")"
      case MaybeCoercingCallSR(range, resultRune, templateRune, argRunes) => humanizeRune(resultRune.rune) + " = " + humanizeRune(templateRune.rune) + "<" + argRunes.map(_.rune).map(humanizeRune).mkString(", ") + ">"
      case MaybeCoercingLookupSR(range, rune, name) => humanizeRune(rune.rune) + " = " + "\"" + humanizeImpreciseName(name) + "\""
      case CallSR(range, resultRune, templateRune, argRunes) => humanizeRune(resultRune.rune) + " = " + humanizeRune(templateRune.rune) + "<" + argRunes.map(_.rune).map(humanizeRune).mkString(", ") + ">"
      case LookupSR(range, rune, name) => humanizeRune(rune.rune) + " = \"" + humanizeImpreciseName(name) + "\""
      case LiteralSR(range, rune, literal) => humanizeRune(rune.rune) + " = " + humanizeLiteral(literal)
      case AugmentSR(range, resultRune, ownership, innerRune) => humanizeRune(resultRune.rune) + " = " + ownership.map(humanizeOwnership).getOrElse("") + humanizeRune(innerRune.rune)
      case EqualsSR(range, left, right) => humanizeRune(left.rune) + " = " + humanizeRune(right.rune)
      case RuneParentEnvLookupSR(range, rune) => "inherit " + humanizeRune(rune.rune)
      case PackSR(range, resultRune, members) => humanizeRune(resultRune.rune) + " = (" + members.map(x => humanizeRune(x.rune)).mkString(", ") + ")"
      case ResolveSR(range, resultRune, name, paramsListRune, returnRune) => {
        humanizeRune(resultRune.rune) + " = resolve-func " + name + "(" + humanizeRune(paramsListRune.rune) + ")" + humanizeRune(returnRune.rune)
      }
      case CallSiteFuncSR(range, resultRune, name, paramsListRune, returnRune) => {
        humanizeRune(resultRune.rune) + " = callsite-func " + name + "(" + humanizeRune(paramsListRune.rune) + ")" + humanizeRune(returnRune.rune)
      }
      case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => {
        humanizeRune(resultRune.rune) + " = definition-func " + name + "(" + humanizeRune(paramsListRune.rune) + ")" + humanizeRune(returnRune.rune)
      }
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => {
//        humanizeRune(resultRune.rune) + " = " + "[#" + humanizeRune(sizeRune.rune) + "]<" + humanizeRune(mutabilityRune.rune) + ", " + humanizeRune(variabilityRune.rune) + ">" + humanizeRune(elementRune.rune)
//      }
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => {
//        humanizeRune(resultRune.rune) + " = " + "[]<" + humanizeRune(mutabilityRune.rune) + ">" + humanizeRune(elementRune.rune)
//      }
      case other => vimpl(other)
    }
  }
*/
fn humanize_literal(
  _literal: &ILiteralSL,
) -> String {
  panic!("Unimplemented humanize_literal");
}
/*
  def humanizeLiteral(literal: ILiteralSL): String = {
    literal match {
      case OwnershipLiteralSL(ownership) => humanizeOwnership(ownership)
      case MutabilityLiteralSL(mutability) => humanizeMutability(mutability)
      case VariabilityLiteralSL(variability) => humanizeVariability(variability)
      case IntLiteralSL(value) => value.toString
      case StringLiteralSL(value) => "\"" + value + "\""
      case other => vimpl(other)
    }
  }
*/
fn humanize_mutability(
  _p: MutabilityP,
) -> String {
  panic!("Unimplemented humanize_mutability");
}
/*
  def humanizeMutability(p: MutabilityP) = {
    p match {
      case MutableP => "mut"
      case ImmutableP => "imm"
    }
  }
*/
fn humanize_variability(
  _p: VariabilityP,
) -> String {
  panic!("Unimplemented humanize_variability");
}
/*
  def humanizeVariability(p: VariabilityP) = {
    p match {
      case VaryingP => "vary"
      case FinalP => "final"
    }
  }
*/
fn humanize_ownership(
  _p: OwnershipP,
) -> String {
  panic!("Unimplemented humanize_ownership");
}
/*
  def humanizeOwnership(p: OwnershipP) = {
    p match {
      case OwnP => "^"
      case ShareP => "@"
      case BorrowP => "&"
      case WeakP => "&&"
    }
  }
*/
fn humanize_region<'s>(
  _r: &RuneUsage<'s>,
) -> String {
  panic!("Unimplemented humanize_region");
}
/*
  def humanizeRegion(r: RuneUsage) = {
    vimpl(r)
  }
}
*/