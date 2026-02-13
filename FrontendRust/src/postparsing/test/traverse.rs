use crate::postparsing::ast::{
  AbstractBodyS, AdditiveS, BuiltinS, CodeBodyS, ExportAsS, ExportS, ExternBodyS, ExternS,
  FileS, FunctionS, GeneratedBodyS, GenericParameterDefaultS, GenericParameterS, IBodyS, ICitizenAttributeS,
  ICitizenDenizenS, ICitizenS, IDenizenS, IFunctionAttributeS, IGenericParameterTypeS, IStructMemberS, ImplS,
  ImportS, InterfaceS, MacroCallS, NormalStructMemberS, ParameterS, ProgramS, PureS, SealedS, SimpleParameterS,
  StructS, TopLevelExportAsS, TopLevelFunctionS, TopLevelImplS, TopLevelImportS, TopLevelInterfaceS,
  TopLevelStructS, UserFunctionS, VariadicStructMemberS,
};
use crate::postparsing::expressions::{
  BlockSE, BodySE, DotSE, IExpressionSE, LocalS, OutsideLoadSE, OwnershippedSE, PureSE, ReturnSE,
};
use crate::postparsing::names::{
  CodeNameS, CodeRuneS, DenizenDefaultRegionRuneS, ExportAsNameS as ExportAsNameFromNamesS, FunctionNameS,
  IFunctionDeclarationNameS, IImpreciseNameS, INameS, IRuneS, IVarNameS, ImplDeclarationNameS,
  LambdaDeclarationNameS, TopLevelCitizenDeclarationNameS, TopLevelInterfaceDeclarationNameS,
  TopLevelStructDeclarationNameS,
};
use crate::postparsing::patterns::{AtomSP, CaptureS};
use crate::postparsing::rules::rules::{
  BoolLiteralSL, ILiteralSL, IRulexSR, IntLiteralSL, LiteralSR, LocationLiteralSL, MaybeCoercingLookupSR,
  MutabilityLiteralSL, OwnershipLiteralSL, StringLiteralSL, VariabilityLiteralSL,
};
use crate::postparsing::rules::RuneUsage;

pub enum NodeRefS<'a> {
  Program(&'a ProgramS),
  File(&'a FileS),

  Struct(&'a StructS),
  Interface(&'a InterfaceS),
  Impl(&'a ImplS),
  Function(&'a FunctionS),
  ExportAs(&'a ExportAsS),
  Import(&'a ImportS),

  Denizen(&'a IDenizenS),
  TopLevelFunction(&'a TopLevelFunctionS),
  TopLevelImpl(&'a TopLevelImplS),
  TopLevelExportAs(&'a TopLevelExportAsS),
  TopLevelImport(&'a TopLevelImportS),
  TopLevelStruct(&'a TopLevelStructS),
  TopLevelInterface(&'a TopLevelInterfaceS),
  CitizenDenizen(&'a ICitizenDenizenS),

  CitizenAttribute(&'a ICitizenAttributeS),
  FunctionAttribute(&'a IFunctionAttributeS),
  ExternAttribute(&'a ExternS),
  BuiltinAttribute(&'a BuiltinS),
  MacroCallAttribute(&'a MacroCallS),
  ExportAttribute(&'a ExportS),
  SealedAttribute(&'a SealedS),
  PureAttribute(&'a PureS),
  AdditiveAttribute(&'a AdditiveS),
  UserFunctionAttribute(&'a UserFunctionS),

  StructMember(&'a IStructMemberS),
  NormalStructMember(&'a NormalStructMemberS),
  VariadicStructMember(&'a VariadicStructMemberS),

  GenericParameter(&'a GenericParameterS),
  GenericParameterDefault(&'a GenericParameterDefaultS),
  GenericParameterType(&'a IGenericParameterTypeS),
  Parameter(&'a ParameterS),
  SimpleParameter(&'a SimpleParameterS),

  Body(&'a IBodyS),
  ExternBody(&'a ExternBodyS),
  AbstractBody(&'a AbstractBodyS),
  GeneratedBody(&'a GeneratedBodyS),
  CodeBody(&'a CodeBodyS),

  BodyExpr(&'a BodySE),
  Local(&'a LocalS),
  Expression(&'a IExpressionSE),
  BlockExpr(&'a BlockSE),
  PureExpr(&'a PureSE),

  Pattern(&'a AtomSP),
  Capture(&'a CaptureS),

  Rulex(&'a IRulexSR),
  PlaceholderRule(&'a crate::postparsing::rules::rules::PlaceholderRuleSR),
  LiteralRule(&'a LiteralSR),
  MaybeCoercingLookupRule(&'a MaybeCoercingLookupSR),
  RuneUsage(&'a RuneUsage),
  Literal(&'a ILiteralSL),
  IntLiteral(&'a IntLiteralSL),
  StringLiteral(&'a StringLiteralSL),
  BoolLiteral(&'a BoolLiteralSL),
  MutabilityLiteral(&'a MutabilityLiteralSL),
  LocationLiteral(&'a LocationLiteralSL),
  OwnershipLiteral(&'a OwnershipLiteralSL),
  VariabilityLiteral(&'a VariabilityLiteralSL),

  Name(&'a INameS),
  FunctionDeclarationName(&'a IFunctionDeclarationNameS),
  FunctionName(&'a FunctionNameS),
  LambdaDeclarationName(&'a LambdaDeclarationNameS),
  TopLevelCitizenDeclarationName(TopLevelCitizenDeclarationNameS),
  TopLevelStructDeclarationName(&'a TopLevelStructDeclarationNameS),
  TopLevelInterfaceDeclarationName(&'a TopLevelInterfaceDeclarationNameS),
  ImplDeclarationName(&'a ImplDeclarationNameS),
  ExportAsName(&'a ExportAsNameFromNamesS),
  ImpreciseName(&'a IImpreciseNameS),
  CodeName(&'a CodeNameS),
  VarName(&'a IVarNameS),
  Rune(&'a IRuneS),
  CodeRune(&'a CodeRuneS),
  DenizenDefaultRegionRune(&'a DenizenDefaultRegionRuneS),
}

fn collect_if<'a, T, F>(pred: &F, out: &mut Vec<T>, node: NodeRefS<'a>)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  if let Some(x) = pred(node) {
    out.push(x);
  }
}

pub fn collect_in_program<'a, T, F>(program: &'a ProgramS, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_program(predicate, &mut out, program);
  out
}

pub fn collect_in_file<'a, T, F>(file: &'a FileS, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_file(predicate, &mut out, file);
  out
}

pub fn collect_in_citizen<'a, T, F>(citizen: &'a ICitizenS, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_citizen(predicate, &mut out, citizen);
  out
}

pub fn collect_in_citizen_denizen<'a, T, F>(denizen: &'a ICitizenDenizenS, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_citizen_denizen(predicate, &mut out, denizen);
  out
}

pub fn collect_in_struct<'a, T, F>(strukt: &'a StructS, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_struct(predicate, &mut out, strukt);
  out
}

pub fn collect_in_srulex<'a, T, F>(rulex: &'a IRulexSR, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_rulex(predicate, &mut out, rulex);
  out
}

fn visit_program<'a, T, F>(pred: &F, out: &mut Vec<T>, program: &'a ProgramS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Program(program));
  for strukt in &program.structs {
    visit_struct(pred, out, strukt);
  }
  for interface in &program.interfaces {
    visit_interface(pred, out, interface);
  }
  for impl_ in &program.impls {
    visit_impl(pred, out, impl_);
  }
  for function in &program.implemented_functions {
    visit_function(pred, out, function);
  }
  for export in &program.exports {
    visit_export_as(pred, out, export);
  }
  for imporrt in &program.imports {
    visit_import(pred, out, imporrt);
  }
}

fn visit_file<'a, T, F>(pred: &F, out: &mut Vec<T>, file: &'a FileS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::File(file));
  for denizen in &file.denizens {
    visit_denizen(pred, out, denizen);
  }
}

fn visit_denizen<'a, T, F>(pred: &F, out: &mut Vec<T>, denizen: &'a IDenizenS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Denizen(denizen));
  match denizen {
    IDenizenS::TopLevelFunction(x) => {
      collect_if(pred, out, NodeRefS::TopLevelFunction(x));
      visit_function(pred, out, &x.function);
    }
    IDenizenS::TopLevelImpl(x) => {
      collect_if(pred, out, NodeRefS::TopLevelImpl(x));
      visit_impl(pred, out, &x.impl_);
    }
    IDenizenS::TopLevelExportAs(x) => {
      collect_if(pred, out, NodeRefS::TopLevelExportAs(x));
      visit_export_as(pred, out, &x.export);
    }
    IDenizenS::TopLevelImport(x) => {
      collect_if(pred, out, NodeRefS::TopLevelImport(x));
      visit_import(pred, out, &x.imporrt);
    }
    IDenizenS::TopLevelStruct(x) => {
      collect_if(pred, out, NodeRefS::TopLevelStruct(x));
      visit_struct(pred, out, &x.strukt);
    }
    IDenizenS::TopLevelInterface(x) => {
      collect_if(pred, out, NodeRefS::TopLevelInterface(x));
      visit_interface(pred, out, &x.interface);
    }
  }
}

fn visit_citizen<'a, T, F>(pred: &F, out: &mut Vec<T>, citizen: &'a ICitizenS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  match citizen {
    ICitizenS::Struct(x) => visit_struct(pred, out, x),
    ICitizenS::Interface(x) => visit_interface(pred, out, x),
  }
}

fn visit_citizen_denizen<'a, T, F>(pred: &F, out: &mut Vec<T>, denizen: &'a ICitizenDenizenS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::CitizenDenizen(denizen));
  match denizen {
    ICitizenDenizenS::TopLevelStruct(x) => {
      collect_if(pred, out, NodeRefS::TopLevelStruct(x));
      visit_struct(pred, out, &x.strukt);
    }
    ICitizenDenizenS::TopLevelInterface(x) => {
      collect_if(pred, out, NodeRefS::TopLevelInterface(x));
      visit_interface(pred, out, &x.interface);
    }
  }
}

fn visit_struct<'a, T, F>(pred: &F, out: &mut Vec<T>, strukt: &'a StructS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Struct(strukt));
  collect_if(
    pred,
    out,
    NodeRefS::TopLevelCitizenDeclarationName(TopLevelCitizenDeclarationNameS::from(&strukt.name)),
  );
  collect_if(pred, out, NodeRefS::TopLevelStructDeclarationName(&strukt.name));
  for attribute in &strukt.attributes {
    visit_citizen_attribute(pred, out, attribute);
  }
  for param in &strukt.generic_params {
    visit_generic_parameter(pred, out, param);
  }
  visit_rune_usage(pred, out, &strukt.mutability_rune);
  for rule in &strukt.header_rules {
    visit_rulex(pred, out, rule);
  }
  for rule in &strukt.member_rules {
    visit_rulex(pred, out, rule);
  }
  for member in &strukt.members {
    visit_struct_member(pred, out, member);
  }
}

fn visit_interface<'a, T, F>(pred: &F, out: &mut Vec<T>, interface: &'a InterfaceS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Interface(interface));
  collect_if(
    pred,
    out,
    NodeRefS::TopLevelCitizenDeclarationName(TopLevelCitizenDeclarationNameS::from(&interface.name)),
  );
  collect_if(pred, out, NodeRefS::TopLevelInterfaceDeclarationName(&interface.name));
  for attribute in &interface.attributes {
    visit_citizen_attribute(pred, out, attribute);
  }
  for param in &interface.generic_params {
    visit_generic_parameter(pred, out, param);
  }
  visit_rune_usage(pred, out, &interface.mutability_rune);
  for rule in &interface.rules {
    visit_rulex(pred, out, rule);
  }
  for method in &interface.internal_methods {
    visit_function(pred, out, method);
  }
}

fn visit_impl<'a, T, F>(pred: &F, out: &mut Vec<T>, impl_: &'a ImplS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Impl(impl_));
  collect_if(pred, out, NodeRefS::ImplDeclarationName(&impl_.name));
  for param in &impl_.user_specified_identifying_runes {
    visit_generic_parameter(pred, out, param);
  }
  for rule in &impl_.rules {
    visit_rulex(pred, out, rule);
  }
  visit_rune_usage(pred, out, &impl_.struct_kind_rune);
  visit_imprecise_name(pred, out, &impl_.sub_citizen_imprecise_name);
  visit_rune_usage(pred, out, &impl_.interface_kind_rune);
  visit_imprecise_name(pred, out, &impl_.super_interface_imprecise_name);
}

fn visit_export_as<'a, T, F>(pred: &F, out: &mut Vec<T>, export: &'a ExportAsS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::ExportAs(export));
  collect_if(pred, out, NodeRefS::ExportAsName(&export.export_name));
  for rule in &export.rules {
    visit_rulex(pred, out, rule);
  }
  visit_rune_usage(pred, out, &export.rune);
}

fn visit_import<'a, T, F>(pred: &F, out: &mut Vec<T>, import: &'a ImportS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Import(import));
}

fn visit_function<'a, T, F>(pred: &F, out: &mut Vec<T>, function: &'a FunctionS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Function(function));
  visit_function_declaration_name(pred, out, &function.name);
  for attribute in &function.attributes {
    visit_function_attribute(pred, out, attribute);
  }
  for param in &function.generic_params {
    visit_generic_parameter(pred, out, param);
  }
  for param in &function.params {
    visit_parameter(pred, out, param);
  }
  if let Some(rune) = &function.maybe_ret_coord_rune {
    visit_rune_usage(pred, out, rune);
  }
  for rule in &function.rules {
    visit_rulex(pred, out, rule);
  }
  visit_body(pred, out, &function.body);
}

fn visit_parameter<'a, T, F>(pred: &F, out: &mut Vec<T>, parameter: &'a ParameterS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Parameter(parameter));
  visit_pattern(pred, out, &parameter.pattern);
}

fn visit_generic_parameter<'a, T, F>(pred: &F, out: &mut Vec<T>, parameter: &'a GenericParameterS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::GenericParameter(parameter));
  visit_rune_usage(pred, out, &parameter.rune);
  visit_generic_parameter_type(pred, out, &parameter.tyype);
  if let Some(default) = &parameter.default {
    visit_generic_parameter_default(pred, out, default);
  }
}

fn visit_generic_parameter_default<'a, T, F>(pred: &F, out: &mut Vec<T>, default: &'a GenericParameterDefaultS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::GenericParameterDefault(default));
  visit_rune(pred, out, &default.result_rune);
  for rule in &default.rules {
    visit_rulex(pred, out, rule);
  }
}

fn visit_generic_parameter_type<'a, T, F>(pred: &F, out: &mut Vec<T>, tyype: &'a IGenericParameterTypeS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::GenericParameterType(tyype));
  if let IGenericParameterTypeS::CoordGenericParameterType(x) = tyype {
    if let Some(coord_region) = &x.coord_region {
      visit_rune_usage(pred, out, coord_region);
    }
  }
}

fn visit_body<'a, T, F>(pred: &F, out: &mut Vec<T>, body: &'a IBodyS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Body(body));
  match body {
    IBodyS::ExternBody(x) => {
      collect_if(pred, out, NodeRefS::ExternBody(x));
    }
    IBodyS::AbstractBody(x) => {
      collect_if(pred, out, NodeRefS::AbstractBody(x));
    }
    IBodyS::GeneratedBody(x) => {
      collect_if(pred, out, NodeRefS::GeneratedBody(x));
    }
    IBodyS::CodeBody(x) => {
      collect_if(pred, out, NodeRefS::CodeBody(x));
      visit_body_expr(pred, out, &x.body);
    }
  }
}

fn visit_body_expr<'a, T, F>(pred: &F, out: &mut Vec<T>, body: &'a BodySE)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::BodyExpr(body));
  for closured_name in &body.closured_names {
    visit_var_name(pred, out, closured_name);
  }
  visit_block(pred, out, &body.block);
}

fn visit_expression<'a, T, F>(pred: &F, out: &mut Vec<T>, expression: &'a IExpressionSE)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Expression(expression));
  match expression {
    IExpressionSE::Let(x) => {
      for rule in &x.rules {
        visit_rulex(pred, out, rule);
      }
      visit_pattern(pred, out, &x.pattern);
      visit_expression(pred, out, x.expr.as_ref());
    }
    IExpressionSE::Consecutor(x) => {
      for expr in &x.exprs {
        visit_expression(pred, out, expr);
      }
    }
    IExpressionSE::Block(x) => visit_block(pred, out, x),
    IExpressionSE::Pure(x) => visit_pure(pred, out, x),
    IExpressionSE::Return(x) => visit_return(pred, out, x),
    IExpressionSE::ConstantInt(_) => {}
    IExpressionSE::Dot(x) => visit_dot(pred, out, x),
    IExpressionSE::FunctionCall(x) => {
      visit_expression(pred, out, x.callable_expr.as_ref());
      for arg in &x.arg_exprs {
        visit_expression(pred, out, arg);
      }
    }
    IExpressionSE::LocalLoad(x) => visit_var_name(pred, out, &x.name),
    IExpressionSE::OutsideLoad(x) => visit_outside_load(pred, out, x),
    IExpressionSE::Ownershipped(x) => visit_ownershipped(pred, out, x),
    _ => panic!("POSTPARSING_TRAVERSE_EXPRESSION_VARIANT_NOT_YET_IMPLEMENTED"),
  }
}

fn visit_return<'a, T, F>(pred: &F, out: &mut Vec<T>, ret: &'a ReturnSE)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  visit_expression(pred, out, ret.inner.as_ref());
}

fn visit_dot<'a, T, F>(pred: &F, out: &mut Vec<T>, dot: &'a DotSE)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  visit_expression(pred, out, dot.left.as_ref());
}

fn visit_outside_load<'a, T, F>(pred: &F, out: &mut Vec<T>, outside_load: &'a OutsideLoadSE)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  for rule in &outside_load.rules {
    visit_rulex(pred, out, rule);
  }
  visit_imprecise_name(pred, out, &outside_load.name);
  if let Some(template_args) = &outside_load.maybe_template_args {
    for template_arg in template_args {
      visit_rune_usage(pred, out, template_arg);
    }
  }
}

fn visit_ownershipped<'a, T, F>(pred: &F, out: &mut Vec<T>, ownershipped: &'a OwnershippedSE)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  visit_expression(pred, out, ownershipped.inner_expr.as_ref());
}

fn visit_block<'a, T, F>(pred: &F, out: &mut Vec<T>, block: &'a BlockSE)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::BlockExpr(block));
  for local in &block.locals {
    visit_local(pred, out, local);
  }
  visit_expression(pred, out, block.expr.as_ref());
}

fn visit_pure<'a, T, F>(pred: &F, out: &mut Vec<T>, pure: &'a PureSE)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::PureExpr(pure));
  visit_expression(pred, out, pure.inner.as_ref());
}

fn visit_local<'a, T, F>(pred: &F, out: &mut Vec<T>, local: &'a LocalS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Local(local));
  visit_var_name(pred, out, &local.var_name);
}

fn visit_pattern<'a, T, F>(pred: &F, out: &mut Vec<T>, pattern: &'a AtomSP)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Pattern(pattern));
  if let Some(capture) = &pattern.name {
    visit_capture(pred, out, capture);
  }
  if let Some(coord_rune) = &pattern.coord_rune {
    visit_rune_usage(pred, out, coord_rune);
  }
  if let Some(destructure) = &pattern.destructure {
    for child_pattern in destructure {
      visit_pattern(pred, out, child_pattern);
    }
  }
}

fn visit_capture<'a, T, F>(pred: &F, out: &mut Vec<T>, capture: &'a CaptureS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Capture(capture));
  visit_var_name(pred, out, &capture.name);
}

fn visit_struct_member<'a, T, F>(pred: &F, out: &mut Vec<T>, member: &'a IStructMemberS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::StructMember(member));
  match member {
    IStructMemberS::NormalStructMember(x) => {
      collect_if(pred, out, NodeRefS::NormalStructMember(x));
      visit_rune_usage(pred, out, &x.type_rune);
    }
    IStructMemberS::VariadicStructMember(x) => {
      collect_if(pred, out, NodeRefS::VariadicStructMember(x));
      visit_rune_usage(pred, out, &x.type_rune);
    }
  }
}

fn visit_citizen_attribute<'a, T, F>(pred: &F, out: &mut Vec<T>, attribute: &'a ICitizenAttributeS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::CitizenAttribute(attribute));
  match attribute {
    ICitizenAttributeS::Extern(x) => collect_if(pred, out, NodeRefS::ExternAttribute(x)),
    ICitizenAttributeS::Sealed(x) => collect_if(pred, out, NodeRefS::SealedAttribute(x)),
    ICitizenAttributeS::Builtin(x) => collect_if(pred, out, NodeRefS::BuiltinAttribute(x)),
    ICitizenAttributeS::MacroCall(x) => collect_if(pred, out, NodeRefS::MacroCallAttribute(x)),
    ICitizenAttributeS::Export(x) => collect_if(pred, out, NodeRefS::ExportAttribute(x)),
  }
}

fn visit_function_attribute<'a, T, F>(pred: &F, out: &mut Vec<T>, attribute: &'a IFunctionAttributeS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::FunctionAttribute(attribute));
  match attribute {
    IFunctionAttributeS::Extern(x) => collect_if(pred, out, NodeRefS::ExternAttribute(x)),
    IFunctionAttributeS::Pure(x) => collect_if(pred, out, NodeRefS::PureAttribute(x)),
    IFunctionAttributeS::Additive(x) => collect_if(pred, out, NodeRefS::AdditiveAttribute(x)),
    IFunctionAttributeS::Builtin(x) => collect_if(pred, out, NodeRefS::BuiltinAttribute(x)),
    IFunctionAttributeS::Export(x) => collect_if(pred, out, NodeRefS::ExportAttribute(x)),
    IFunctionAttributeS::UserFunction(x) => collect_if(pred, out, NodeRefS::UserFunctionAttribute(x)),
  }
}

fn visit_rulex<'a, T, F>(pred: &F, out: &mut Vec<T>, rulex: &'a IRulexSR)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Rulex(rulex));
  match rulex {
    IRulexSR::Placeholder(x) => {
      collect_if(pred, out, NodeRefS::PlaceholderRule(x));
    }
    IRulexSR::Literal(x) => {
      collect_if(pred, out, NodeRefS::LiteralRule(x));
      visit_rune_usage(pred, out, &x.rune);
      visit_literal(pred, out, &x.literal);
    }
    IRulexSR::MaybeCoercingLookup(x) => {
      collect_if(pred, out, NodeRefS::MaybeCoercingLookupRule(x));
      visit_rune_usage(pred, out, &x.rune);
      visit_imprecise_name(pred, out, &x.name);
    }
  }
}

fn visit_literal<'a, T, F>(pred: &F, out: &mut Vec<T>, literal: &'a ILiteralSL)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Literal(literal));
  match literal {
    ILiteralSL::IntLiteral(x) => collect_if(pred, out, NodeRefS::IntLiteral(x)),
    ILiteralSL::StringLiteral(x) => collect_if(pred, out, NodeRefS::StringLiteral(x)),
    ILiteralSL::BoolLiteral(x) => collect_if(pred, out, NodeRefS::BoolLiteral(x)),
    ILiteralSL::MutabilityLiteral(x) => collect_if(pred, out, NodeRefS::MutabilityLiteral(x)),
    ILiteralSL::LocationLiteral(x) => collect_if(pred, out, NodeRefS::LocationLiteral(x)),
    ILiteralSL::OwnershipLiteral(x) => collect_if(pred, out, NodeRefS::OwnershipLiteral(x)),
    ILiteralSL::VariabilityLiteral(x) => collect_if(pred, out, NodeRefS::VariabilityLiteral(x)),
  }
}

fn visit_function_declaration_name<'a, T, F>(pred: &F, out: &mut Vec<T>, name: &'a IFunctionDeclarationNameS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::FunctionDeclarationName(name));
  match name {
    IFunctionDeclarationNameS::FunctionName(x) => collect_if(pred, out, NodeRefS::FunctionName(x)),
    IFunctionDeclarationNameS::LambdaDeclarationName(x) => {
      collect_if(pred, out, NodeRefS::LambdaDeclarationName(x))
    }
    _ => panic!("POSTPARSING_TRAVERSE_FUNCTION_DECL_NAME_NOT_YET_IMPLEMENTED"),
  }
}

fn visit_imprecise_name<'a, T, F>(pred: &F, out: &mut Vec<T>, name: &'a IImpreciseNameS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::ImpreciseName(name));
  match name {
    IImpreciseNameS::CodeName(x) => collect_if(pred, out, NodeRefS::CodeName(x.as_ref())),
    _ => panic!("POSTPARSING_TRAVERSE_IMPRECISE_NAME_NOT_YET_IMPLEMENTED"),
  }
}

fn visit_var_name<'a, T, F>(pred: &F, out: &mut Vec<T>, name: &'a IVarNameS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::VarName(name));
}

fn visit_rune_usage<'a, T, F>(pred: &F, out: &mut Vec<T>, rune_usage: &'a RuneUsage)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::RuneUsage(rune_usage));
  visit_rune(pred, out, &rune_usage.rune);
}

fn visit_rune<'a, T, F>(pred: &F, out: &mut Vec<T>, rune: &'a IRuneS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Rune(rune));
  match rune {
    IRuneS::CodeRune(x) => collect_if(pred, out, NodeRefS::CodeRune(x)),
    IRuneS::DenizenDefaultRegionRune(x) => {
      collect_if(pred, out, NodeRefS::DenizenDefaultRegionRune(x));
      visit_name(pred, out, &x.denizen_name);
    }
    _ => panic!("POSTPARSING_TRAVERSE_RUNE_VARIANT_NOT_YET_IMPLEMENTED"),
  }
}

fn visit_name<'a, T, F>(pred: &F, out: &mut Vec<T>, name: &'a INameS)
where
  F: Fn(NodeRefS<'a>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Name(name));
  match name {
    INameS::FunctionDeclaration(x) => visit_function_declaration_name(pred, out, x),
    INameS::ImplDeclaration(x) => collect_if(pred, out, NodeRefS::ImplDeclarationName(x)),
    INameS::ExportAsName(x) => collect_if(pred, out, NodeRefS::ExportAsName(x)),
    INameS::TopLevelStructDeclaration(x) => collect_if(pred, out, NodeRefS::TopLevelStructDeclarationName(x)),
    INameS::TopLevelInterfaceDeclaration(x) => {
      collect_if(pred, out, NodeRefS::TopLevelInterfaceDeclarationName(x))
    }
    INameS::VarName(x) => visit_var_name(pred, out, x),
    _ => panic!("POSTPARSING_TRAVERSE_NAME_VARIANT_NOT_YET_IMPLEMENTED"),
  }
}

#[macro_export]
macro_rules! collect_where_sprogram {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    $crate::postparsing::test::traverse::collect_in_program($expr, &|node| match node {
      $pattern => $body,
      _ => None,
    })
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    $crate::postparsing::test::traverse::collect_in_program($expr, &|node| match node {
      $pattern if $guard => $body,
      _ => None,
    })
  }};
}

#[macro_export]
macro_rules! collect_only_sprogram {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    let mut matches = $crate::collect_where_sprogram!($expr, $pattern => $body);
    assert_eq!(1, matches.len());
    matches.remove(0)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    let mut matches = $crate::collect_where_sprogram!($expr, $pattern if $guard => $body);
    assert_eq!(1, matches.len());
    matches.remove(0)
  }};
}

#[macro_export]
macro_rules! collect_where_sstruct {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    $crate::postparsing::test::traverse::collect_in_struct($expr, &|node| match node {
      $pattern => $body,
      _ => None,
    })
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    $crate::postparsing::test::traverse::collect_in_struct($expr, &|node| match node {
      $pattern if $guard => $body,
      _ => None,
    })
  }};
}

#[macro_export]
macro_rules! collect_only_sstruct {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    let mut matches = $crate::collect_where_sstruct!($expr, $pattern => $body);
    assert_eq!(1, matches.len());
    matches.remove(0)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    let mut matches = $crate::collect_where_sstruct!($expr, $pattern if $guard => $body);
    assert_eq!(1, matches.len());
    matches.remove(0)
  }};
}
