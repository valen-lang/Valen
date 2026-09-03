use crate::parsing::ast::SharednessP;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::names::IRuneS;
use crate::postparsing::names::{
  ICitizenDeclarationNameS, IFunctionDeclarationNameS, INameS, IStructDeclarationNameS, IVarDeclarationNameS,
};
use crate::postparsing::post_parser::ICompileErrorS;
use crate::postparsing::rules::rules::ILiteralSL;
use crate::postparsing::rules::rules::IRulexSR;
use crate::postparsing::rules::rules::RuneUsage;
use crate::utils::range::{CodeLocationS, RangeS};

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
    ICompileErrorS::StatementAfterReturnS(_) => {
      panic!("implement: humanize StatementAfterReturnS");
    }
    ICompileErrorS::ParamDestructureRequiresBody { .. } => {
      "This function has no body block (extern/abstract/generated), so its parameters can't use destructuring syntax. Take the whole value and destructure it inside the body.".to_string()
    }
  };
  let range = err.range();
  let pos_str = humanize_pos(&range.begin);
  let next_stuff = line_containing(&range.begin);
  let error_id = "S";
  format!("{} error {}: {}\n{}\n", pos_str, error_id, error_str_body, next_stuff)
}

fn humanize_var_name<'s>(var_name: IVarDeclarationNameS<'s>) -> String {
  match var_name {
    IVarDeclarationNameS::CodeVarName(n) => n.imprecise_name.name.as_str().to_string(),
    IVarDeclarationNameS::ClosureParamName(_) => "(closure)".to_string(),
    _ => panic!("Unimplemented humanize_var_name branch for IVarDeclarationNameS"),
  }
}

fn humanize_function_declaration_name<'s>(name: IFunctionDeclarationNameS<'s>) -> String {
  match name {
    IFunctionDeclarationNameS::FunctionName(n) => n.imprecise_name.name.as_str().to_string(),
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
  }
}

pub fn humanize_name_for_struct_declaration<'s>(name: IStructDeclarationNameS<'s>) -> String {
  match name {
    IStructDeclarationNameS::TopLevelStructDeclarationName(n) => n.name.as_str().to_string(),
    IStructDeclarationNameS::AnonymousSubstructTemplateName(n) => {
      n.interface_name.name.as_str().to_string() + ".anonymous"
    }
  }
}

pub fn humanize_citizen_declaration_name<'s>(name: ICitizenDeclarationNameS<'s>) -> String {
  match name {
    ICitizenDeclarationNameS::TopLevelStructDeclarationName(n) => n.name.as_str().to_string(),
    ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(n) => n.name.as_str().to_string(),
    ICitizenDeclarationNameS::AnonymousSubstructTemplateName(n) => {
      n.interface_name.name.as_str().to_string() + ".anonymous"
    }
  }
}

pub fn humanize_imprecise_name<'s>(name: IImpreciseNameS<'s>) -> String {
  match name {
    IImpreciseNameS::ArbitraryName(_) => "_arby".to_string(),
    IImpreciseNameS::SelfName(_) => "_Self".to_string(),
    IImpreciseNameS::CodeName(n) => n.name.0.to_string(),
    IImpreciseNameS::RuneName(rune) => humanize_rune(rune.rune),
    IImpreciseNameS::AnonymousSubstructTemplateImpreciseName(n) => {
      humanize_imprecise_name(n.interface_imprecise_name) + "._AnonSub"
    }
    IImpreciseNameS::LambdaStructImpreciseName(n) => {
      humanize_imprecise_name(n.lambda_name) + ".struct"
    }
    IImpreciseNameS::LambdaImpreciseName(_) => "_Lam".to_string(),
    _ => panic!("implement: humanize_imprecise_name other"),
  }
}

pub fn humanize_rune<'s>(rune: IRuneS<'s>) -> String {
  match rune {
    IRuneS::ImplicitRune(r) => {
      "_".to_string() + &r.lid.path.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("")
    }
    IRuneS::MagicParamRune(r) => {
      "_".to_string() + &r.lid.path.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("")
    }
    IRuneS::CodeRune(r) => r.name.0.to_string(),
    IRuneS::ArgumentRune(r) => "(arg ".to_string() + &r.arg_index.to_string() + ")",
    IRuneS::SelfKindRune(_) => "(self kind)".to_string(),
    IRuneS::SelfFullTypeRune(_) => "(self full type)".to_string(),
    IRuneS::SelfKindTemplateRune(_) => "(self kind template)".to_string(),
    IRuneS::PatternInputRune(_) => panic!("implement: humanize_rune PatternInputRune"),
    IRuneS::SelfRune(_) => panic!("implement: humanize_rune SelfRune"),
    IRuneS::ReturnRune(_) => "(return)".to_string(),
    IRuneS::AnonymousSubstructParentInterfaceTemplateRune(_) => {
      panic!("implement: humanize_rune AnonymousSubstructParentInterfaceTemplateRune")
    }
    IRuneS::ImplDropVoidRune(_) => panic!("implement: humanize_rune ImplDropVoidRune"),
    IRuneS::ImplDropKindRune(_) => panic!("implement: humanize_rune ImplDropKindRune"),
    IRuneS::FreeOverrideInterfaceRune(_) => {
      panic!("implement: humanize_rune FreeOverrideInterfaceRune")
    }
    IRuneS::FreeOverrideStructRune(_) => panic!("implement: humanize_rune FreeOverrideStructRune"),
    IRuneS::AnonymousSubstructKindRune(_) => {
      panic!("implement: humanize_rune AnonymousSubstructKindRune")
    }
    IRuneS::AnonymousSubstructTemplateRune(_) => {
      panic!("implement: humanize_rune AnonymousSubstructTemplateRune")
    }
    IRuneS::AnonymousSubstructParentInterfaceKindRune(_) => {
      panic!("implement: humanize_rune AnonymousSubstructParentInterfaceKindRune")
    }
    IRuneS::StructNameRune(r) => humanize_citizen_declaration_name(r.struct_name),
    IRuneS::FreeOverrideStructTemplateRune(_) => {
      panic!("implement: humanize_rune FreeOverrideStructTemplateRune")
    }
    IRuneS::FunctorPrototypeRuneName(_) => {
      panic!("implement: humanize_rune FunctorPrototypeRuneName")
    }
    IRuneS::MacroSelfKindRune(_) => "_MSelfK".to_string(),
    IRuneS::MacroVoidKindRune(_) => "_MVoidK".to_string(),
    IRuneS::MacroSelfKindTemplateRune(_) => "_MSelfKT".to_string(),
    IRuneS::AnonymousSubstructMemberRune(r) => {
      "$".to_string()
        + r.interface.name.as_str()
        + ".anon."
        + &humanize_function_declaration_name(r.method)
        + ".functor"
    }
    IRuneS::AnonymousSubstructFunctionBoundParamsListRune(r) => {
      "$".to_string()
        + r.interface.name.as_str()
        + ".anon."
        + &humanize_function_declaration_name(r.method)
        + ".params"
    }
    IRuneS::AnonymousSubstructFunctionBoundPrototypeRune(r) => {
      "$".to_string()
        + r.interface.name.as_str()
        + ".anon."
        + &humanize_function_declaration_name(r.method)
        + ".proto"
    }
    IRuneS::AnonymousSubstructFunctionInterfaceTemplateRune(r) => {
      "$".to_string()
        + r.interface.name.as_str()
        + ".anon."
        + &humanize_function_declaration_name(r.method)
        + ".itemplate"
    }
    IRuneS::AnonymousSubstructFunctionInterfaceKindRune(r) => {
      "$".to_string()
        + r.interface.name.as_str()
        + ".anon."
        + &humanize_function_declaration_name(r.method)
        + ".ikind"
    }
    IRuneS::AnonymousSubstructDropBoundParamsListRune(r) => {
      "$".to_string() + r.interface.name.as_str() + ".anon.drop.params"
    }
    IRuneS::AnonymousSubstructDropBoundPrototypeRune(r) => {
      "$".to_string() + r.interface.name.as_str() + ".anon.drop.proto"
    }
    IRuneS::StructDropBoundParamsListRune(_) => "$structdrop.params".to_string(),
    IRuneS::StructDropBoundPrototypeRune(_) => "$structdrop.proto".to_string(),
    IRuneS::AnonymousSubstructMethodInheritedRune(r) => {
      "$".to_string()
        + r.interface.name.as_str()
        + ".anon."
        + &humanize_function_declaration_name(r.method)
        + ":"
        + &humanize_rune(r.inner)
    }
    IRuneS::AnonymousSubstructMethodSelfBorrowKindRune(r) => {
      "$".to_string()
        + r.interface.name.as_str()
        + ".anon."
        + &humanize_function_declaration_name(r.method)
        + ".borrowself"
    }
    IRuneS::DenizenDefaultRegionRune(_) => {
      panic!("implement: humanize_rune DenizenDefaultRegionRune")
    }
    IRuneS::ExternDefaultRegionRune(_) => {
      panic!("implement: humanize_rune ExternDefaultRegionRune")
    }
    IRuneS::AnonymousSubstructVoidKindRune(_) => "anon.void.kind".to_string(),
    IRuneS::ImplicitCoercionTemplateRune(inner) => humanize_rune(inner.original_kind_rune) + ".gen",
    IRuneS::ImplicitRegionRune(_) => panic!("implement: humanize_rune ImplicitRegionRune"),
    IRuneS::CallRegionRune(_) => panic!("implement: humanize_rune CallRegionRune"),
    IRuneS::CaseRuneFromImpl(r) => "case:".to_string() + &humanize_rune(r.inner_rune),
    IRuneS::DispatcherRuneFromImpl(r) => "dis:".to_string() + &humanize_rune(r.inner_rune),
    IRuneS::CallPureMergeRegionRune(_) => {
      panic!("implement: humanize_rune CallPureMergeRegionRune")
    }
    IRuneS::ReachablePrototypeRune(_) => panic!("implement: humanize_rune ReachablePrototypeRune"),
    IRuneS::MemberRune(_) => panic!("implement: humanize_rune MemberRune"),
    IRuneS::LocalDefaultRegionRune(_) => panic!("implement: humanize_rune LocalDefaultRegionRune"),
    IRuneS::ExportDefaultRegionRune(_) => {
      panic!("implement: humanize_rune ExportDefaultRegionRune")
    }
    IRuneS::ArraySizeImplicitRune(_) => panic!("implement: humanize_rune ArraySizeImplicitRune"),
    IRuneS::ArrayMutabilityImplicitRune(_) => {
      panic!("implement: humanize_rune ArrayMutabilityImplicitRune")
    }
    IRuneS::InterfaceNameRune(_) => panic!("implement: humanize_rune InterfaceNameRune"),
    IRuneS::LetImplicitRune(_) => panic!("implement: humanize_rune LetImplicitRune"),
    IRuneS::ExplicitTemplateArgRune(_) => {
      panic!("implement: humanize_rune ExplicitTemplateArgRune")
    }
    IRuneS::FunctorParamRuneName(_) => panic!("implement: humanize_rune FunctorParamRuneName"),
    IRuneS::FunctorReturnRuneName(_) => panic!("implement: humanize_rune FunctorReturnRuneName"),
  }
}

pub fn humanize_templata_type(tyype: &ITemplataType) -> String {
  match tyype {
    ITemplataType::KindTemplataType(_) => "Kind".to_string(),
    ITemplataType::FunctionTemplataType(_) => "Func".to_string(),
    ITemplataType::IntegerTemplataType(_) => "Int".to_string(),
    ITemplataType::GroupTemplataType(_) => "Group".to_string(),
    ITemplataType::BooleanTemplataType(_) => "Bool".to_string(),
    ITemplataType::StringTemplataType(_) => "Str".to_string(),
    ITemplataType::PackTemplataType(p) => {
      "Pack<".to_string() + &humanize_templata_type(p.element_type) + ">"
    }
    ITemplataType::PrototypeTemplataType(_) => "Prot".to_string(),
    ITemplataType::TemplateTemplataType(t) => {
      humanize_templata_type(t.return_type)
        + "<"
        + &t.param_types.iter().map(humanize_templata_type).collect::<Vec<_>>().join(",")
        + ">"
    }
    ITemplataType::ImplTemplataType(_) => {
      panic!("implement: humanize_templata_type ImplTemplataType")
    }
  }
}

pub fn humanize_rule<'s>(rule: &IRulexSR<'s>) -> String {
  match rule {
    IRulexSR::BorrowRef(r) => "&".to_string() + &humanize_rune(r.inner_rune.rune),
    IRulexSR::WeakRef(r) => "weak ".to_string() + &humanize_rune(r.inner_rune.rune),
    IRulexSR::OwnRef(r) => "own ".to_string() + &humanize_rune(r.inner_rune.rune),
    IRulexSR::Call(r) => {
      humanize_rune(r.result_rune.rune)
        + " = "
        + &humanize_rune(r.template_rune.rune)
        + "<"
        + &r.args.iter().map(|x| humanize_rune(x.rune)).collect::<Vec<_>>().join(", ")
        + ">"
    }
    // Joined rather than asserted-single: a humanizer runs while reporting a failure, so it must
    // render whatever it is handed rather than adding a second failure on top of the first.
    IRulexSR::Lookup(r) => {
      humanize_rune(r.rune.rune)
        + " = \""
        + &r.parts.iter().map(|p| humanize_imprecise_name(*p)).collect::<Vec<_>>().join(".")
        + "\""
    }
    IRulexSR::Literal(r) => humanize_rune(r.rune.rune) + " = " + &humanize_literal(&r.literal),
    IRulexSR::Equals(r) => humanize_rune(r.left.rune) + " = " + &humanize_rune(r.right.rune),
    IRulexSR::RuneParentEnvLookup(r) => "inherit ".to_string() + &humanize_rune(r.rune.rune),
    IRulexSR::KindList(r) => {
      humanize_rune(r.result_rune.rune)
        + " = ("
        + &r.members.iter().map(|x| humanize_rune(x.rune)).collect::<Vec<_>>().join(", ")
        + ")"
    }
    IRulexSR::Resolve(r) => {
      humanize_rune(r.result_rune.rune)
        + " = resolve-func "
        + r.name.0
        + "("
        + &humanize_rune(r.params_list_rune.rune)
        + ")"
        + &humanize_rune(r.return_rune.rune)
    }
    IRulexSR::CallSiteFunc(r) => {
      humanize_rune(r.prototype_rune.rune)
        + " = callsite-func "
        + r.name.0
        + "("
        + &humanize_rune(r.params_list_rune.rune)
        + ")"
        + &humanize_rune(r.return_rune.rune)
    }
    IRulexSR::DefinitionFunc(_) => panic!("implement: humanize_rule DefinitionFunc"),
  }
}

fn humanize_literal(literal: &ILiteralSL) -> String {
  match literal {
    ILiteralSL::IntLiteral(x) => x.value.to_string(),
    ILiteralSL::StringLiteral(_) => {
      panic!("Unimplemented: humanize_literal StringLiteral");
      // "\"" + value + "\""
    }
    ILiteralSL::BoolLiteral(x) => x.value.to_string(),
  }
}

fn _humanize_region<'s>(_r: &RuneUsage<'s>) -> String {
  panic!("Unimplemented humanize_region");
}
