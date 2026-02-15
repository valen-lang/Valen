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

pub enum NodeRefS<'a, 'p> {
  Program(&'p ProgramS<'a>),
  File(&'p FileS<'a>),

  Struct(&'p StructS<'a>),
  Interface(&'p InterfaceS<'a>),
  Impl(&'p ImplS<'a>),
  Function(&'p FunctionS<'a>),
  ExportAs(&'p ExportAsS<'a>),
  Import(&'p ImportS<'a>),

  Denizen(&'p IDenizenS<'a>),
  TopLevelFunction(&'p TopLevelFunctionS<'a>),
  TopLevelImpl(&'p TopLevelImplS<'a>),
  TopLevelExportAs(&'p TopLevelExportAsS<'a>),
  TopLevelImport(&'p TopLevelImportS<'a>),
  TopLevelStruct(&'p TopLevelStructS<'a>),
  TopLevelInterface(&'p TopLevelInterfaceS<'a>),
  CitizenDenizen(&'p ICitizenDenizenS<'a>),

  CitizenAttribute(&'p ICitizenAttributeS<'a>),
  FunctionAttribute(&'p IFunctionAttributeS<'a>),
  ExternAttribute(&'p ExternS<'a>),
  BuiltinAttribute(&'p BuiltinS<'a>),
  MacroCallAttribute(&'p MacroCallS<'a>),
  ExportAttribute(&'p ExportS<'a>),
  SealedAttribute(&'p SealedS),
  PureAttribute(&'p PureS),
  AdditiveAttribute(&'p AdditiveS),
  UserFunctionAttribute(&'p UserFunctionS),

  StructMember(&'p IStructMemberS<'a>),
  NormalStructMember(&'p NormalStructMemberS<'a>),
  VariadicStructMember(&'p VariadicStructMemberS<'a>),

  GenericParameter(&'p GenericParameterS<'a>),
  GenericParameterDefault(&'p GenericParameterDefaultS<'a>),
  GenericParameterType(&'p IGenericParameterTypeS<'a>),
  Parameter(&'p ParameterS<'a>),
  SimpleParameter(&'p SimpleParameterS<'a>),

  Body(&'p IBodyS<'a>),
  ExternBody(&'p ExternBodyS),
  AbstractBody(&'p AbstractBodyS),
  GeneratedBody(&'p GeneratedBodyS<'a>),
  CodeBody(&'p CodeBodyS<'a>),

  BodyExpr(&'p BodySE<'a>),
  Local(&'p LocalS<'a>),
  Expression(&'p IExpressionSE<'a>),
  BlockExpr(&'p BlockSE<'a>),
  PureExpr(&'p PureSE<'a>),

  Pattern(&'p AtomSP<'a>),
  Capture(&'p CaptureS<'a>),

  Rulex(&'p IRulexSR<'a>),
  PlaceholderRule(&'p crate::postparsing::rules::rules::PlaceholderRuleSR<'a>),
  LiteralRule(&'p LiteralSR<'a>),
  MaybeCoercingLookupRule(&'p MaybeCoercingLookupSR<'a>),
  RuneUsage(&'p RuneUsage<'a>),
  Literal(&'p ILiteralSL),
  IntLiteral(&'p IntLiteralSL),
  StringLiteral(&'p StringLiteralSL),
  BoolLiteral(&'p BoolLiteralSL),
  MutabilityLiteral(&'p MutabilityLiteralSL),
  LocationLiteral(&'p LocationLiteralSL),
  OwnershipLiteral(&'p OwnershipLiteralSL),
  VariabilityLiteral(&'p VariabilityLiteralSL),

  Name(&'p INameS<'a>),
  FunctionDeclarationName(&'p IFunctionDeclarationNameS<'a>),
  FunctionName(&'p FunctionNameS<'a>),
  LambdaDeclarationName(&'p LambdaDeclarationNameS<'a>),
  TopLevelCitizenDeclarationName(TopLevelCitizenDeclarationNameS<'a>),
  TopLevelStructDeclarationName(&'p TopLevelStructDeclarationNameS<'a>),
  TopLevelInterfaceDeclarationName(&'p TopLevelInterfaceDeclarationNameS<'a>),
  ImplDeclarationName(&'p ImplDeclarationNameS<'a>),
  ExportAsName(&'p ExportAsNameFromNamesS<'a>),
  ImpreciseName(&'p IImpreciseNameS<'a>),
  CodeName(&'p CodeNameS<'a>),
  VarName(&'p IVarNameS<'a>),
  Rune(&'p IRuneS<'a>),
  CodeRune(&'a CodeRuneS<'a>),
  DenizenDefaultRegionRune(&'p DenizenDefaultRegionRuneS<'a>),
}

fn collect_if<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, node: NodeRefS<'a, 'p>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  if let Some(x) = pred(node) {
    out.push(x);
  }
}

pub fn collect_in_program<'a, 'p, T, F>(program: &'p ProgramS<'a>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_program(predicate, &mut out, program);
  out
}

pub fn collect_in_file<'a, 'p, T, F>(file: &'p FileS<'a>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_file(predicate, &mut out, file);
  out
}

pub fn collect_in_citizen<'a, 'p, T, F>(citizen: &'p ICitizenS<'a>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_citizen(predicate, &mut out, citizen);
  out
}

pub fn collect_in_citizen_denizen<'a, 'p, T, F>(denizen: &'p ICitizenDenizenS<'a>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_citizen_denizen(predicate, &mut out, denizen);
  out
}

pub fn collect_in_struct<'a, 'p, T, F>(strukt: &'p StructS<'a>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_struct(predicate, &mut out, strukt);
  out
}

pub fn collect_in_srulex<'a, 'p, T, F>(rulex: &'p IRulexSR<'a>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_rulex(predicate, &mut out, rulex);
  out
}

fn visit_program<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, program: &'p ProgramS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_file<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, file: &'p FileS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::File(file));
  for denizen in &file.denizens {
    visit_denizen(pred, out, denizen);
  }
}

fn visit_denizen<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, denizen: &'p IDenizenS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_citizen<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, citizen: &'p ICitizenS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  match citizen {
    ICitizenS::Struct(x) => visit_struct(pred, out, x),
    ICitizenS::Interface(x) => visit_interface(pred, out, x),
  }
}

fn visit_citizen_denizen<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, denizen: &'p ICitizenDenizenS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_struct<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, strukt: &'p StructS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_interface<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, interface: &'p InterfaceS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_impl<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, impl_: &'p ImplS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_export_as<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, export: &'p ExportAsS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::ExportAs(export));
  collect_if(pred, out, NodeRefS::ExportAsName(&export.export_name));
  for rule in &export.rules {
    visit_rulex(pred, out, rule);
  }
  visit_rune_usage(pred, out, &export.rune);
}

fn visit_import<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, import: &'p ImportS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Import(import));
}

fn visit_function<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, function: &'p FunctionS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_parameter<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, parameter: &'p ParameterS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Parameter(parameter));
  visit_pattern(pred, out, &parameter.pattern);
}

fn visit_generic_parameter<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, parameter: &'p GenericParameterS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::GenericParameter(parameter));
  visit_rune_usage(pred, out, &parameter.rune);
  visit_generic_parameter_type(pred, out, &parameter.tyype);
  if let Some(default) = &parameter.default {
    visit_generic_parameter_default(pred, out, default);
  }
}

fn visit_generic_parameter_default<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, default: &'p GenericParameterDefaultS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::GenericParameterDefault(default));
  visit_rune(pred, out, &default.result_rune);
  for rule in &default.rules {
    visit_rulex(pred, out, rule);
  }
}

fn visit_generic_parameter_type<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, tyype: &'p IGenericParameterTypeS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::GenericParameterType(tyype));
  if let IGenericParameterTypeS::CoordGenericParameterType(x) = tyype {
    if let Some(coord_region) = &x.coord_region {
      visit_rune_usage(pred, out, coord_region);
    }
  }
}

fn visit_body<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, body: &'p IBodyS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_body_expr<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, body: &'p BodySE<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::BodyExpr(body));
  for closured_name in &body.closured_names {
    visit_var_name(pred, out, closured_name);
  }
  visit_block(pred, out, &body.block);
}

fn visit_expression<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, expression: &'p IExpressionSE<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_return<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, ret: &'p ReturnSE<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  visit_expression(pred, out, ret.inner.as_ref());
}

fn visit_dot<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, dot: &'p DotSE<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  visit_expression(pred, out, dot.left.as_ref());
}

fn visit_outside_load<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, outside_load: &'p OutsideLoadSE<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_ownershipped<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, ownershipped: &'p OwnershippedSE<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  visit_expression(pred, out, ownershipped.inner_expr.as_ref());
}

fn visit_block<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, block: &'p BlockSE<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::BlockExpr(block));
  for local in &block.locals {
    visit_local(pred, out, local);
  }
  visit_expression(pred, out, block.expr.as_ref());
}

fn visit_pure<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, pure: &'p PureSE<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::PureExpr(pure));
  visit_expression(pred, out, pure.inner.as_ref());
}

fn visit_local<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, local: &'p LocalS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Local(local));
  visit_var_name(pred, out, &local.var_name);
}

fn visit_pattern<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, pattern: &'p AtomSP<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_capture<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, capture: &'p CaptureS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Capture(capture));
  visit_var_name(pred, out, &capture.name);
}

fn visit_struct_member<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, member: &'p IStructMemberS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_citizen_attribute<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, attribute: &'p ICitizenAttributeS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_function_attribute<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, attribute: &'p IFunctionAttributeS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_rulex<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, rulex: &'p IRulexSR<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_literal<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, literal: &'p ILiteralSL)
where
  'a: 'p,
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_function_declaration_name<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, name: &'p IFunctionDeclarationNameS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_imprecise_name<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, name: &'p IImpreciseNameS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::ImpreciseName(name));
  match name {
    IImpreciseNameS::CodeName(x) => collect_if(pred, out, NodeRefS::CodeName(x)),
    _ => panic!("POSTPARSING_TRAVERSE_IMPRECISE_NAME_NOT_YET_IMPLEMENTED"),
  }
}

fn visit_var_name<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, name: &'p IVarNameS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::VarName(name));
}

fn visit_rune_usage<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, rune_usage: &'p RuneUsage<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::RuneUsage(rune_usage));
  visit_rune(pred, out, &rune_usage.rune);
}

fn visit_rune<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, rune: &'p IRuneS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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

fn visit_name<'a, 'p, T, F>(pred: &F, out: &mut Vec<T>, name: &'p INameS<'a>)
where
  F: Fn(NodeRefS<'a, 'p>) -> Option<T>,
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
