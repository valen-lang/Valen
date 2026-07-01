use crate::postparsing::names::{INameS, IFunctionDeclarationNameS, IStructDeclarationNameS, IVarNameS};
use crate::postparsing::post_parser::ICompileErrorS;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::names::IRuneS;
use crate::postparsing::rules::rules::IRulexSR;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::rules::rules::ILiteralSL;
use crate::parsing::ast::SharednessP;
use crate::parsing::ast::OwnershipP;
use crate::postparsing::rules::rules::RuneUsage;
use crate::postparsing::rune_type_solver::IRuneTypeRuleError;


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
    ICompileErrorS::RuneExplicitTypeConflictS(_) => {
      panic!("implement: humanize RuneExplicitTypeConflictS");
      // "Too many explicit types for rune " + humanizeRune(rune) + "" + types.map(humanizeTemplataType).mkString(", ")
    }
    ICompileErrorS::RangedInternalErrorS(_) => {
      panic!("implement: humanize RangedInternalErrorS");
      // " " + message
    }
    ICompileErrorS::CouldntFindRuneS(_) => {
      panic!("implement: humanize CouldntFindRuneS");
      // "Couldn't find generic parameter \"" + name + "\".\n"
    }
    ICompileErrorS::CouldntFindVarToMutateS(_) => {
      panic!("implement: humanize CouldntFindVarToMutateS");
      // s"No variable named ${name}. Try declaring it above, like `${name} = 42;`\n"
    }
    ICompileErrorS::CantOwnershipInterfaceInImpl(_) => {
      panic!("implement: humanize CantOwnershipInterfaceInImpl");
      // s"Can only impl a plain interface, remove symbol."
    }
    ICompileErrorS::CantOwnershipStructInImpl(_) => {
      panic!("implement: humanize CantOwnershipStructInImpl");
      // s"Only a plain struct/interface can be in an impl, remove symbol."
    }
    ICompileErrorS::InitializingRuntimeSizedArrayRequiresSizeAndCallable(_) => {
      panic!("implement: humanize InitializingRuntimeSizedArrayRequiresSizeAndCallable");
      // s"Initializing a runtime-sized array requires 1-2 arguments: a capacity, and optionally a function that will populate that many elements."
    }
    ICompileErrorS::InitializingStaticSizedArrayRequiresSizeAndCallable(_) => {
      panic!("implement: humanize InitializingStaticSizedArrayRequiresSizeAndCallable");
      // s"Initializing a statically-sized array requires one argument: a function that will populate the elements."
    }
    _ => panic!("Unimplemented humanize branch for {:?}", err),
  };
  let range = err.range();
  let pos_str = humanize_pos(&range.begin);
  let next_stuff = line_containing(&range.begin);
  let error_id = "S";
  format!("{} error {}: {}\n{}\n", pos_str, error_id, error_str_body, next_stuff)
}

pub fn humanize_rune_type_error<'s>(
  _code_map: &dyn Fn(CodeLocationS<'s>) -> String,
  error: &IRuneTypeRuleError<'s>,
) -> String {
  match error {
    IRuneTypeRuleError::FoundTemplataDidntMatchExpectedType(_) => {
      panic!("implement: humanize_rune_type_error FoundTemplataDidntMatchExpectedType");
      // "Expected " + humanizeTemplataType(expectedType) + " but found " + humanizeTemplataType(actualType)
    }
    IRuneTypeRuleError::CouldntFindType(e) => {
      format!("Couldn't find anything with the name '{}'", humanize_imprecise_name(e.name))
    }
    IRuneTypeRuleError::NotEnoughArgumentsForGenericCall(_) => {
      panic!("implement: humanize_rune_type_error NotEnoughArgumentsForGenericCall");
      // "Not enough arguments for generic call, expected at least " + (indexOfNonDefaultingParam + 1)
    }
    _ => panic!("implement: humanize_rune_type_error other"),
  }
}

fn humanize_identifiability_rule_errorr<'s>(
  _error: &(),
) -> String {
  panic!("Unimplemented humanize_identifiability_rule_errorr");
  // error match {
  //   case other => vimpl(other)
  // }
}

fn humanize_var_name<'s>(var_name: IVarNameS<'s>) -> String {
  match var_name {
    IVarNameS::CodeVarName(n) => n.as_str().to_string(),
    IVarNameS::ClosureParamName(_) => "(closure)".to_string(),
    _ => panic!("Unimplemented humanize_var_name branch for IVarNameS"),
  }
}

fn humanize_function_declaration_name<'s>(name: IFunctionDeclarationNameS<'s>) -> String {
  match name {
    IFunctionDeclarationNameS::FunctionName(n) => n.name.as_str().to_string(),
    IFunctionDeclarationNameS::LambdaDeclarationName(_) => {
      panic!("implement: humanize_function_declaration_name LambdaDeclarationName");
    }
    IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(n) => {
      humanize_function_declaration_name(n.inner) + ".forwarder" + &n.index.to_string()
    }
    IFunctionDeclarationNameS::ConstructorName(_) => {
      panic!("implement: humanize_function_declaration_name ConstructorName");
      // "constructor<" + humanizeName(inner.tlcd) + ">"
    }
    IFunctionDeclarationNameS::ImmConcreteDestructorName(_) => {
      panic!("implement: humanize_function_declaration_name ImmConcreteDestructorName");
    }
    IFunctionDeclarationNameS::ImmInterfaceDestructorName(_) => {
      panic!("implement: humanize_function_declaration_name ImmInterfaceDestructorName");
    }
  }
}

pub fn humanize_name_for_struct_declaration<'s>(name: IStructDeclarationNameS<'s>) -> String {
  match name {
    IStructDeclarationNameS::TopLevelStructDeclarationName(n) => n.name.as_str().to_string(),
    IStructDeclarationNameS::AnonymousSubstructTemplateName(n) => n.interface_name.name.as_str().to_string() + ".anonymous",
  }
}

fn humanize_name<'s>(name: INameS<'s>) -> String {
  match name {
    INameS::VarName(var_name) => humanize_var_name((*var_name).clone()),
    INameS::GlobalFunctionFamilyName(_) => {
      panic!("implement: humanize_name GlobalFunctionFamilyName");
      // n
    }
    INameS::ArbitraryName(_) => {
      panic!("implement: humanize_name ArbitraryName");
      // "(arbitrary)"
    }
    INameS::RuneName(_) => {
      panic!("implement: humanize_name RuneName");
      // humanizeRune(rune)
    }
    INameS::AnonymousSubstructTemplateName(_) => {
      panic!("implement: humanize_name AnonymousSubstructTemplateName");
      // humanizeName(tlcd) + ".anonymous"
    }
    INameS::AnonymousSubstructImplDeclaration(_) => {
      panic!("implement: humanize_name AnonymousSubstructImplDeclaration");
      // humanizeName(interface) + ".anonimpl"
    }
    INameS::TopLevelStructDeclaration(_) | INameS::TopLevelInterfaceDeclaration(_) => {
      panic!("implement: humanize_name TopLevelCitizenDeclaration");
      // name.str
    }
    INameS::RuntimeSizedArrayDeclarationName(_) => {
      panic!("implement: humanize_name RuntimeSizedArrayDeclarationName");
      // "__rsa"
    }
    INameS::ImplDeclaration(_) => {
      panic!("implement: humanize_name ImplDeclaration");
      // "(impl)"
    }
    _ => panic!("Unimplemented humanize_name branch for INameS"),
  }
}

pub fn humanize_imprecise_name<'s>(
  name: IImpreciseNameS<'s>,
) -> String {
  match name {
    IImpreciseNameS::ArbitraryName(_) => "_arby".to_string(),
    IImpreciseNameS::SelfName(_) => "_Self".to_string(),
    IImpreciseNameS::CodeName(n) => n.name.0.to_string(),
    IImpreciseNameS::RuneName(rune) => humanize_rune(rune.rune),
    IImpreciseNameS::AnonymousSubstructTemplateImpreciseName(n) => {
      humanize_imprecise_name(n.interface_imprecise_name) + "._AnonSub"
    }
    IImpreciseNameS::LambdaStructImpreciseName(n) => humanize_imprecise_name(n.lambda_name) + ".struct",
    IImpreciseNameS::LambdaImpreciseName(_) => "_Lam".to_string(),
    _ => panic!("implement: humanize_imprecise_name other"),
  }
}

pub fn humanize_rune<'s>(
  rune: IRuneS<'s>,
) -> String {
  match rune {
    IRuneS::ImplicitRune(r) => "_".to_string() + &r.lid.path.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(""),
    IRuneS::MagicParamRune(r) => "_".to_string() + &r.lid.path.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(""),
    IRuneS::CodeRune(r) => r.name.0.to_string(),
    IRuneS::ArgumentRune(r) => "(arg ".to_string() + &r.arg_index.to_string() + ")",
    IRuneS::SelfKindRune(_) => "(self kind)".to_string(),
    IRuneS::SelfOwnershipRune(_) => panic!("implement: humanize_rune SelfOwnershipRune"),
    IRuneS::SelfKindTemplateRune(_) => "(self kind template)".to_string(),
    IRuneS::PatternInputRune(_) => panic!("implement: humanize_rune PatternInputRune"),
    IRuneS::SelfRune(_) => panic!("implement: humanize_rune SelfRune"),
    IRuneS::SelfCoordRune(_) => "(self ref)".to_string(),
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
    IRuneS::MacroSelfKindRune(_) => "_MSelfK".to_string(),
    IRuneS::MacroSelfCoordRune(_) => "_MSelf".to_string(),
    IRuneS::MacroVoidKindRune(_) => "_MVoidK".to_string(),
    IRuneS::MacroVoidCoordRune(_) => "_MVoid".to_string(),
    IRuneS::MacroSelfKindTemplateRune(_) => "_MSelfKT".to_string(),
    IRuneS::AnonymousSubstructMemberRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon." +
        &humanize_function_declaration_name(r.method) + ".functor",
    IRuneS::AnonymousSubstructFunctionBoundParamsListRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon." +
        &humanize_function_declaration_name(r.method) + ".params",
    IRuneS::AnonymousSubstructFunctionBoundPrototypeRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon." +
        &humanize_function_declaration_name(r.method) + ".proto",
    IRuneS::AnonymousSubstructFunctionInterfaceTemplateRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon." +
        &humanize_function_declaration_name(r.method) + ".itemplate",
    IRuneS::AnonymousSubstructFunctionInterfaceKindRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon." +
        &humanize_function_declaration_name(r.method) + ".ikind",
    IRuneS::AnonymousSubstructDropBoundParamsListRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon.drop.params",
    IRuneS::AnonymousSubstructDropBoundPrototypeRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon.drop.proto",
    IRuneS::AnonymousSubstructMethodInheritedRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon." +
        &humanize_function_declaration_name(r.method) + ":" + &humanize_rune(r.inner),
    IRuneS::AnonymousSubstructMethodSelfOwnCoordRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon." +
        &humanize_function_declaration_name(r.method) + ".ownself",
    IRuneS::AnonymousSubstructMethodSelfBorrowCoordRune(r) =>
      "$".to_string() + r.interface.name.as_str() + ".anon." +
        &humanize_function_declaration_name(r.method) + ".borrowself",
    IRuneS::DenizenDefaultRegionRune(_) => panic!("implement: humanize_rune DenizenDefaultRegionRune"),
    IRuneS::ExternDefaultRegionRune(_) => panic!("implement: humanize_rune ExternDefaultRegionRune"),
    IRuneS::AnonymousSubstructVoidKindRune(_) => "anon.void.kind".to_string(),
    IRuneS::AnonymousSubstructVoidCoordRune(_) => "anon.void".to_string(),
    IRuneS::ImplicitCoercionOwnershipRune(_) => panic!("implement: humanize_rune ImplicitCoercionOwnershipRune"),
    IRuneS::ImplicitCoercionKindRune(inner) => humanize_rune(inner.original_coord_rune) + ".kind",
    IRuneS::ImplicitCoercionTemplateRune(inner) => humanize_rune(inner.original_kind_rune) + ".gen",
    IRuneS::ImplicitRegionRune(_) => panic!("implement: humanize_rune ImplicitRegionRune"),
    IRuneS::CallRegionRune(_) => panic!("implement: humanize_rune CallRegionRune"),
    IRuneS::CaseRuneFromImpl(_) => panic!("implement: humanize_rune CaseRuneFromImpl"),
    IRuneS::DispatcherRuneFromImpl(_) => panic!("implement: humanize_rune DispatcherRuneFromImpl"),
    IRuneS::CallPureMergeRegionRune(_) => panic!("implement: humanize_rune CallPureMergeRegionRune"),
    IRuneS::ReachablePrototypeRune(_) => panic!("implement: humanize_rune ReachablePrototypeRune"),
    IRuneS::MemberRune(_) => panic!("implement: humanize_rune MemberRune"),
    IRuneS::LocalDefaultRegionRune(_) => panic!("implement: humanize_rune LocalDefaultRegionRune"),
    IRuneS::ExportDefaultRegionRune(_) => panic!("implement: humanize_rune ExportDefaultRegionRune"),
    IRuneS::ArraySizeImplicitRune(_) => panic!("implement: humanize_rune ArraySizeImplicitRune"),
    IRuneS::ArrayMutabilityImplicitRune(_) => panic!("implement: humanize_rune ArrayMutabilityImplicitRune"),
    IRuneS::InterfaceNameRune(_) => panic!("implement: humanize_rune InterfaceNameRune"),
    IRuneS::LetImplicitRune(_) => panic!("implement: humanize_rune LetImplicitRune"),
    IRuneS::ExplicitTemplateArgRune(_) => panic!("implement: humanize_rune ExplicitTemplateArgRune"),
    IRuneS::FunctorParamRuneName(_) => panic!("implement: humanize_rune FunctorParamRuneName"),
    IRuneS::FunctorReturnRuneName(_) => panic!("implement: humanize_rune FunctorReturnRuneName"),
  }
}

pub fn humanize_templata_type(
  tyype: &ITemplataType,
) -> String {
  match tyype {
    ITemplataType::KindTemplataType(_) => "Kind".to_string(),
    ITemplataType::CoordTemplataType(_) => "Type".to_string(),
    ITemplataType::FunctionTemplataType(_) => "Func".to_string(),
    ITemplataType::IntegerTemplataType(_) => "Int".to_string(),
    ITemplataType::RegionTemplataType(_) => "Region".to_string(),
    ITemplataType::BooleanTemplataType(_) => "Bool".to_string(),
    ITemplataType::SharednessTemplataType(_) => "Mut".to_string(),
    ITemplataType::PrototypeTemplataType(_) => "Prot".to_string(),
    ITemplataType::StringTemplataType(_) => "Str".to_string(),
    ITemplataType::LocationTemplataType(_) => "Loc".to_string(),
    ITemplataType::OwnershipTemplataType(_) => "Own".to_string(),
    ITemplataType::PackTemplataType(p) => "Pack<".to_string() + &humanize_templata_type(p.element_type) + ">",
    ITemplataType::TemplateTemplataType(t) => humanize_templata_type(t.return_type) + "<" + &t.param_types.iter().map(humanize_templata_type).collect::<Vec<_>>().join(",") + ">",
    ITemplataType::ImplTemplataType(_) => panic!("implement: humanize_templata_type ImplTemplataType"),
  }
}

pub fn humanize_rule<'s>(
  rule: &IRulexSR<'s>,
) -> String {
  match rule {
    IRulexSR::KindComponents(r) => {
      humanize_rune(r.kind_rune.rune) + " = Kind"
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
    IRulexSR::CallSiteCoordIsa(r) =>
      r.result_rune.map(|rr| humanize_rune(rr.rune)).unwrap_or_else(|| "_".to_string()) + " = " +
        &humanize_rune(r.sub_rune.rune) + " call-isa " + &humanize_rune(r.super_rune.rune),
    IRulexSR::CoordSend(r) => humanize_rune(r.sender_rune.rune) + " -> " + &humanize_rune(r.receiver_rune.rune),
    IRulexSR::CoerceToCoord(r) => "coerceToCoord(".to_string() + &humanize_rune(r.coord_rune.rune) + ", " + &humanize_rune(r.kind_rune.rune) + ")",
    IRulexSR::MaybeCoercingCall(r) => humanize_rune(r.result_rune.rune) + " = " + &humanize_rune(r.template_rune.rune) + "<" + &r.args.iter().map(|x| humanize_rune(x.rune)).collect::<Vec<_>>().join(", ") + ">",
    IRulexSR::MaybeCoercingLookup(r) => humanize_rune(r.rune.rune) + " = \"" + &humanize_imprecise_name(r.name) + "\"",
    IRulexSR::Call(r) => humanize_rune(r.result_rune.rune) + " = " + &humanize_rune(r.template_rune.rune) + "<" + &r.args.iter().map(|x| humanize_rune(x.rune)).collect::<Vec<_>>().join(", ") + ">",
    IRulexSR::Lookup(r) => humanize_rune(r.rune.rune) + " = \"" + &humanize_imprecise_name(r.name) + "\"",
    IRulexSR::Literal(r) => humanize_rune(r.rune.rune) + " = " + &humanize_literal(&r.literal),
    IRulexSR::Augment(r) => humanize_rune(r.result_rune.rune) + " = " + &r.ownership.map(humanize_ownership).unwrap_or_else(String::new) + &humanize_rune(r.inner_rune.rune),
    IRulexSR::Equals(r) => humanize_rune(r.left.rune) + " = " + &humanize_rune(r.right.rune),
    IRulexSR::RuneParentEnvLookup(r) => "inherit ".to_string() + &humanize_rune(r.rune.rune),
    IRulexSR::Pack(r) => humanize_rune(r.result_rune.rune) + " = (" + &r.members.iter().map(|x| humanize_rune(x.rune)).collect::<Vec<_>>().join(", ") + ")",
    IRulexSR::Resolve(r) => humanize_rune(r.result_rune.rune) + " = resolve-func " + r.name.0 + "(" + &humanize_rune(r.params_list_rune.rune) + ")" + &humanize_rune(r.return_rune.rune),
    IRulexSR::CallSiteFunc(r) => humanize_rune(r.prototype_rune.rune) + " = callsite-func " + r.name.0 + "(" + &humanize_rune(r.params_list_rune.rune) + ")" + &humanize_rune(r.return_rune.rune),
    IRulexSR::DefinitionFunc(_) => panic!("implement: humanize_rule DefinitionFunc"),
    other => panic!("vimpl humanize_rule: {:?}", other),
  }
}

fn humanize_literal(
  literal: &ILiteralSL,
) -> String {
  match literal {
    ILiteralSL::OwnershipLiteral(_) => {
      panic!("Unimplemented: humanize_literal OwnershipLiteral");
      // humanizeOwnership(ownership)
    }
    ILiteralSL::MutabilityLiteral(x) => humanize_mutability(x.mutability),
    ILiteralSL::IntLiteral(x) => x.value.to_string(),
    ILiteralSL::StringLiteral(_) => {
      panic!("Unimplemented: humanize_literal StringLiteral");
      // "\"" + value + "\""
    }
    other => panic!("vimpl humanize_literal: {:?}", other),
  }
}

fn humanize_mutability(
  p: SharednessP,
) -> String {
  match p {
    SharednessP::Single => "mut".to_string(),
    SharednessP::Shared => "imm".to_string(),
  }
}

pub fn humanize_ownership(
  p: OwnershipP,
) -> String {
  match p {
    OwnershipP::Own => "^".to_string(),
    OwnershipP::Share => "@".to_string(),
    OwnershipP::Borrow => "&".to_string(),
    OwnershipP::Weak => "&&".to_string(),
    OwnershipP::Live => panic!("Unimplemented: humanize_ownership Live"),
  }
}

fn humanize_region<'s>(
  _r: &RuneUsage<'s>,
) -> String {
  panic!("Unimplemented humanize_region");
  // vimpl(r)
}
