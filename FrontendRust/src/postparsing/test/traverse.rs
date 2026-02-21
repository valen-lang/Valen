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
  IFunctionDeclarationNameS, IImpreciseNameS, INameS, IRuneS, IVarNameS, ImplDeclarationNameS, ImplicitRuneS,
  LambdaDeclarationNameS, TopLevelCitizenDeclarationNameS, TopLevelInterfaceDeclarationNameS,
  TopLevelStructDeclarationNameS, MagicParamRuneS,
};
use crate::postparsing::patterns::{AtomSP, CaptureS};
use crate::postparsing::rules::rules::{
  AugmentSR, BoolLiteralSL, ILiteralSL, IRulexSR, IntLiteralSL, LiteralSR, LocationLiteralSL,
  LookupSR, MaybeCoercingCallSR, MaybeCoercingLookupSR, MutabilityLiteralSL, OwnershipLiteralSL,
  StringLiteralSL, VariabilityLiteralSL,
};
use crate::postparsing::rules::RuneUsage;

pub enum NodeRefS<'a, 's> {
  Program(&'s ProgramS<'a, 's>),
  File(&'s FileS<'a, 's>),

  Struct(&'s StructS<'a, 's>),
  Interface(&'s InterfaceS<'a, 's>),
  Impl(&'s ImplS<'a, 's>),
  Function(&'s FunctionS<'a, 's>),
  ExportAs(&'s ExportAsS<'a, 's>),
  Import(&'s ImportS<'a, 's>),

  Denizen(&'s IDenizenS<'a, 's>),
  TopLevelFunction(&'s TopLevelFunctionS<'a, 's>),
  TopLevelImpl(&'s TopLevelImplS<'a, 's>),
  TopLevelExportAs(&'s TopLevelExportAsS<'a, 's>),
  TopLevelImport(&'s TopLevelImportS<'a, 's>),
  TopLevelStruct(&'s TopLevelStructS<'a, 's>),
  TopLevelInterface(&'s TopLevelInterfaceS<'a, 's>),
  CitizenDenizen(&'s ICitizenDenizenS<'a, 's>),

  CitizenAttribute(&'s ICitizenAttributeS<'a>),
  FunctionAttribute(&'s IFunctionAttributeS<'a>),
  ExternAttribute(&'s ExternS<'a>),
  BuiltinAttribute(&'s BuiltinS<'a>),
  MacroCallAttribute(&'s MacroCallS<'a>),
  ExportAttribute(&'s ExportS<'a>),
  SealedAttribute(&'s SealedS),
  PureAttribute(&'s PureS),
  AdditiveAttribute(&'s AdditiveS),
  UserFunctionAttribute(&'s UserFunctionS),

  StructMember(&'s IStructMemberS<'a>),
  NormalStructMember(&'s NormalStructMemberS<'a>),
  VariadicStructMember(&'s VariadicStructMemberS<'a>),

  GenericParameter(&'s GenericParameterS<'a>),
  GenericParameterDefault(&'s GenericParameterDefaultS<'a>),
  GenericParameterType(&'s IGenericParameterTypeS<'a>),
  Parameter(&'s ParameterS<'a>),
  SimpleParameter(&'s SimpleParameterS<'a>),

  Body(&'s IBodyS<'a, 's>),
  ExternBody(&'s ExternBodyS),
  AbstractBody(&'s AbstractBodyS),
  GeneratedBody(&'s GeneratedBodyS<'a>),
  CodeBody(&'s CodeBodyS<'a, 's>),

  BodyExpr(&'s BodySE<'a, 's>),
  Local(&'s LocalS<'a>),
  Expression(&'s IExpressionSE<'a, 's>),
  BlockExpr(&'s BlockSE<'a, 's>),
  PureExpr(&'s PureSE<'a, 's>),

  Pattern(&'s AtomSP<'a>),
  Capture(&'s CaptureS<'a>),

  Rulex(&'s IRulexSR<'a>),
  PlaceholderRule(&'s crate::postparsing::rules::rules::PlaceholderRuleSR<'a>),
  LiteralRule(&'s LiteralSR<'a>),
  MaybeCoercingLookupRule(&'s MaybeCoercingLookupSR<'a>),
  LookupRule(&'s LookupSR<'a>),
  MaybeCoercingCallRule(&'s MaybeCoercingCallSR<'a>),
  AugmentRule(&'s AugmentSR<'a>),
  RuneUsage(&'s RuneUsage<'a>),
  Literal(&'s ILiteralSL),
  IntLiteral(&'s IntLiteralSL),
  StringLiteral(&'s StringLiteralSL),
  BoolLiteral(&'s BoolLiteralSL),
  MutabilityLiteral(&'s MutabilityLiteralSL),
  LocationLiteral(&'s LocationLiteralSL),
  OwnershipLiteral(&'s OwnershipLiteralSL),
  VariabilityLiteral(&'s VariabilityLiteralSL),

  Name(&'s INameS<'a>),
  FunctionDeclarationName(&'s IFunctionDeclarationNameS<'a>),
  FunctionName(&'s FunctionNameS<'a>),
  LambdaDeclarationName(&'s LambdaDeclarationNameS<'a>),
  TopLevelCitizenDeclarationName(TopLevelCitizenDeclarationNameS<'a>),
  TopLevelStructDeclarationName(&'s TopLevelStructDeclarationNameS<'a>),
  TopLevelInterfaceDeclarationName(&'s TopLevelInterfaceDeclarationNameS<'a>),
  ImplDeclarationName(&'s ImplDeclarationNameS<'a>),
  ExportAsName(&'s ExportAsNameFromNamesS<'a>),
  ImpreciseName(&'s IImpreciseNameS<'a>),
  CodeName(&'s CodeNameS<'a>),
  VarName(&'s IVarNameS<'a>),
  Rune(&'s IRuneS<'a>),
  CodeRune(&'a CodeRuneS<'a>),
  ImplicitRune(&'s ImplicitRuneS),
  MagicParamRune(&'s MagicParamRuneS),
  DenizenDefaultRegionRune(&'s DenizenDefaultRegionRuneS<'a>),
}

fn collect_if<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, node: NodeRefS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  if let Some(x) = pred(node) {
    out.push(x);
  }
}

pub fn collect_in_program<'a, 's, T, F>(program: &'s ProgramS<'a, 's>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_program(predicate, &mut out, program);
  out
}

pub fn collect_in_file<'a, 's, T, F>(file: &'s FileS<'a, 's>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_file(predicate, &mut out, file);
  out
}

pub fn collect_in_citizen<'a, 's, T, F>(citizen: &'s ICitizenS<'a, 's>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_citizen(predicate, &mut out, citizen);
  out
}

pub fn collect_in_citizen_denizen<'a, 's, T, F>(
  denizen: &'s ICitizenDenizenS<'a, 's>,
  predicate: &F,
) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_citizen_denizen(predicate, &mut out, denizen);
  out
}

pub fn collect_in_struct<'a, 's, T, F>(strukt: &'s StructS<'a, 's>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_struct(predicate, &mut out, strukt);
  out
}

pub fn collect_in_srulex<'a, 's, T, F>(rulex: &'s IRulexSR<'a>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_rulex(predicate, &mut out, rulex);
  out
}

pub fn collect_in_sexpressions<'a, 's, T, F>(
  expressions: &'s [&'s IExpressionSE<'a, 's>],
  predicate: &F,
) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  let mut out = Vec::new();
  for expression in expressions {
    visit_expression(predicate, &mut out, expression);
  }
  out
}

pub fn collect_in_sexpression<'a, 's, T, F>(expression: &'s IExpressionSE<'a, 's>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  let mut out = Vec::new();
  visit_expression(predicate, &mut out, expression);
  out
}

fn visit_program<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, program: &'s ProgramS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Program(program));
  for strukt in program.structs {
    visit_struct(pred, out, strukt);
  }
  for interface in program.interfaces {
    visit_interface(pred, out, interface);
  }
  for impl_ in program.impls {
    visit_impl(pred, out, impl_);
  }
  for function in program.implemented_functions {
    visit_function(pred, out, function);
  }
  for export in program.exports {
    visit_export_as(pred, out, export);
  }
  for imporrt in program.imports {
    visit_import(pred, out, imporrt);
  }
}

fn visit_file<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, file: &'s FileS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::File(file));
  for denizen in &file.denizens {
    visit_denizen(pred, out, denizen);
  }
}

fn visit_denizen<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, denizen: &'s IDenizenS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_citizen<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, citizen: &'s ICitizenS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  match citizen {
    ICitizenS::Struct(x) => visit_struct(pred, out, x),
    ICitizenS::Interface(x) => visit_interface(pred, out, x),
  }
}

fn visit_citizen_denizen<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, denizen: &'s ICitizenDenizenS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_struct<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, strukt: &'s StructS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Struct(strukt));
  collect_if(
    pred,
    out,
    NodeRefS::TopLevelCitizenDeclarationName(TopLevelCitizenDeclarationNameS::from(&strukt.name)),
  );
  collect_if(pred, out, NodeRefS::TopLevelStructDeclarationName(&strukt.name));
  for attribute in strukt.attributes {
    visit_citizen_attribute(pred, out, attribute);
  }
  for param in strukt.generic_params {
    visit_generic_parameter(pred, out, param);
  }
  visit_rune_usage(pred, out, &strukt.mutability_rune);
  for rule in strukt.header_rules {
    visit_rulex(pred, out, rule);
  }
  for rule in strukt.member_rules {
    visit_rulex(pred, out, rule);
  }
  for member in strukt.members {
    visit_struct_member(pred, out, member);
  }
}

fn visit_interface<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, interface: &'s InterfaceS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Interface(interface));
  collect_if(
    pred,
    out,
    NodeRefS::TopLevelCitizenDeclarationName(TopLevelCitizenDeclarationNameS::from(&interface.name)),
  );
  collect_if(pred, out, NodeRefS::TopLevelInterfaceDeclarationName(&interface.name));
  for attribute in interface.attributes {
    visit_citizen_attribute(pred, out, attribute);
  }
  for param in interface.generic_params {
    visit_generic_parameter(pred, out, param);
  }
  visit_rune_usage(pred, out, &interface.mutability_rune);
  for rule in interface.rules {
    visit_rulex(pred, out, rule);
  }
  for method in interface.internal_methods {
    visit_function(pred, out, method);
  }
}

fn visit_impl<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, impl_: &'s ImplS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Impl(impl_));
  collect_if(pred, out, NodeRefS::ImplDeclarationName(&impl_.name));
  for param in impl_.user_specified_identifying_runes {
    visit_generic_parameter(pred, out, param);
  }
  for rule in impl_.rules {
    visit_rulex(pred, out, rule);
  }
  visit_rune_usage(pred, out, &impl_.struct_kind_rune);
  visit_imprecise_name(pred, out, &impl_.sub_citizen_imprecise_name);
  visit_rune_usage(pred, out, &impl_.interface_kind_rune);
  visit_imprecise_name(pred, out, &impl_.super_interface_imprecise_name);
}

fn visit_export_as<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, export: &'s ExportAsS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::ExportAs(export));
  collect_if(pred, out, NodeRefS::ExportAsName(&export.export_name));
  for rule in export.rules {
    visit_rulex(pred, out, rule);
  }
  visit_rune_usage(pred, out, &export.rune);
}

fn visit_import<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, import: &'s ImportS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Import(import));
}

fn visit_function<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, function: &'s FunctionS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Function(function));
  visit_function_declaration_name(pred, out, &function.name);
  for attribute in function.attributes {
    visit_function_attribute(pred, out, attribute);
  }
  for param in function.generic_params {
    visit_generic_parameter(pred, out, param);
  }
  for param in function.params {
    visit_parameter(pred, out, param);
  }
  if let Some(rune) = &function.maybe_ret_coord_rune {
    visit_rune_usage(pred, out, rune);
  }
  for rule in function.rules {
    visit_rulex(pred, out, rule);
  }
  visit_body(pred, out, &function.body);
}

fn visit_parameter<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, parameter: &'s ParameterS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Parameter(parameter));
  visit_pattern(pred, out, &parameter.pattern);
}

fn visit_generic_parameter<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, parameter: &'s GenericParameterS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::GenericParameter(parameter));
  visit_rune_usage(pred, out, &parameter.rune);
  visit_generic_parameter_type(pred, out, &parameter.tyype);
  if let Some(default) = &parameter.default {
    visit_generic_parameter_default(pred, out, default);
  }
}

fn visit_generic_parameter_default<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, default: &'s GenericParameterDefaultS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::GenericParameterDefault(default));
  visit_rune(pred, out, &default.result_rune);
  for rule in &default.rules {
    visit_rulex(pred, out, rule);
  }
}

fn visit_generic_parameter_type<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, tyype: &'s IGenericParameterTypeS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::GenericParameterType(tyype));
  if let IGenericParameterTypeS::CoordGenericParameterType(x) = tyype {
    if let Some(coord_region) = &x.coord_region {
      visit_rune_usage(pred, out, coord_region);
    }
  }
}

fn visit_body<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, body: &'s IBodyS<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_body_expr<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, body: &'s BodySE<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::BodyExpr(body));
  for closured_name in &body.closured_names {
    visit_var_name(pred, out, closured_name);
  }
  visit_block(pred, out, &body.block);
}

fn visit_expression<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, expression: &'s IExpressionSE<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Expression(expression));
  match expression {
    IExpressionSE::Let(x) => {
      for rule in &x.rules {
        visit_rulex(pred, out, rule);
      }
      visit_pattern(pred, out, &x.pattern);
      visit_expression(pred, out, x.expr);
    }
    IExpressionSE::If(x) => {
      visit_expression(pred, out, x.condition);
      visit_block(pred, out, &x.then_body);
      visit_block(pred, out, &x.else_body);
    }
    IExpressionSE::Loop(x) => visit_block(pred, out, &x.body),
    IExpressionSE::Break(_) => {}
    IExpressionSE::While(x) => visit_block(pred, out, &x.body),
    IExpressionSE::Map(x) => visit_block(pred, out, &x.body),
    IExpressionSE::ExprMutate(x) => {
      visit_expression(pred, out, x.mutatee);
      visit_expression(pred, out, x.expr);
    }
    IExpressionSE::GlobalMutate(x) => visit_expression(pred, out, x.expr),
    IExpressionSE::LocalMutate(x) => {
      visit_var_name(pred, out, &x.name);
      visit_expression(pred, out, x.expr);
    }
    IExpressionSE::Consecutor(x) => {
      for expr in x.exprs {
        visit_expression(pred, out, expr);
      }
    }
    IExpressionSE::ArgLookup(_) => {}
    IExpressionSE::RepeaterBlock(x) => visit_expression(pred, out, x.expression),
    IExpressionSE::RepeaterBlockIterator(x) => visit_expression(pred, out, x.expression),
    IExpressionSE::Void(_) => {}
    IExpressionSE::Tuple(x) => {
      for element in x.elements {
        visit_expression(pred, out, element);
      }
    }
    IExpressionSE::StaticArrayFromValues(x) => {
      for rule in &x.rules {
        visit_rulex(pred, out, rule);
      }
      if let Some(element_type_st) = &x.maybe_element_type_st {
        visit_rune_usage(pred, out, element_type_st);
      }
      visit_rune_usage(pred, out, &x.mutability_st);
      visit_rune_usage(pred, out, &x.variability_st);
      visit_rune_usage(pred, out, &x.size_st);
      for element in x.elements {
        visit_expression(pred, out, element);
      }
    }
    IExpressionSE::StaticArrayFromCallable(x) => {
      for rule in &x.rules {
        visit_rulex(pred, out, rule);
      }
      if let Some(element_type_st) = &x.maybe_element_type_st {
        visit_rune_usage(pred, out, element_type_st);
      }
      visit_rune_usage(pred, out, &x.mutability_st);
      visit_rune_usage(pred, out, &x.variability_st);
      visit_rune_usage(pred, out, &x.size_st);
      visit_expression(pred, out, x.callable);
    }
    IExpressionSE::NewRuntimeSizedArray(x) => {
      for rule in &x.rules {
        visit_rulex(pred, out, rule);
      }
      if let Some(element_type_st) = &x.maybe_element_type_st {
        visit_rune_usage(pred, out, element_type_st);
      }
      visit_rune_usage(pred, out, &x.mutability_st);
      visit_expression(pred, out, x.size);
      if let Some(callable) = x.callable {
        visit_expression(pred, out, callable);
      }
    }
    IExpressionSE::RepeaterPack(x) => visit_expression(pred, out, x.expression),
    IExpressionSE::RepeaterPackIterator(x) => visit_expression(pred, out, x.expression),
    IExpressionSE::Block(x) => visit_block(pred, out, x),
    IExpressionSE::Pure(x) => visit_pure(pred, out, x),
    IExpressionSE::Return(x) => visit_return(pred, out, x),
    IExpressionSE::ConstantInt(_) => {}
    IExpressionSE::ConstantBool(_) => {}
    IExpressionSE::ConstantStr(_) => {}
    IExpressionSE::ConstantFloat(_) => {}
    IExpressionSE::Destruct(x) => visit_expression(pred, out, x.inner),
    IExpressionSE::Unlet(x) => visit_var_name(pred, out, &x.name),
    IExpressionSE::Function(x) => visit_function(pred, out, &x.function),
    IExpressionSE::Dot(x) => visit_dot(pred, out, x),
    IExpressionSE::Index(x) => {
      visit_expression(pred, out, x.left);
      visit_expression(pred, out, x.index_expr);
    }
    IExpressionSE::FunctionCall(x) => {
      visit_expression(pred, out, x.callable_expr);
      for arg in x.arg_exprs {
        visit_expression(pred, out, arg);
      }
    }
    IExpressionSE::LocalLoad(x) => visit_var_name(pred, out, &x.name),
    IExpressionSE::OutsideLoad(x) => visit_outside_load(pred, out, x),
    IExpressionSE::RuneLookup(x) => visit_rune(pred, out, &x.rune),
    IExpressionSE::Ownershipped(x) => visit_ownershipped(pred, out, x),
  }
}

fn visit_return<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, ret: &'s ReturnSE<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  visit_expression(pred, out, ret.inner);
}

fn visit_dot<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, dot: &'s DotSE<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  visit_expression(pred, out, dot.left);
}

fn visit_outside_load<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, outside_load: &'s OutsideLoadSE<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_ownershipped<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, ownershipped: &'s OwnershippedSE<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  visit_expression(pred, out, ownershipped.inner_expr);
}

fn visit_block<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, block: &'s BlockSE<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::BlockExpr(block));
  for local in &block.locals {
    visit_local(pred, out, local);
  }
  visit_expression(pred, out, block.expr);
}

fn visit_pure<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, pure: &'s PureSE<'a, 's>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::PureExpr(pure));
  visit_expression(pred, out, pure.inner);
}

fn visit_local<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, local: &'s LocalS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Local(local));
  visit_var_name(pred, out, &local.var_name);
}

fn visit_pattern<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, pattern: &'s AtomSP<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_capture<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, capture: &'s CaptureS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Capture(capture));
  visit_var_name(pred, out, &capture.name);
}

fn visit_struct_member<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, member: &'s IStructMemberS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_citizen_attribute<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, attribute: &'s ICitizenAttributeS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_function_attribute<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, attribute: &'s IFunctionAttributeS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_rulex<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, rulex: &'s IRulexSR<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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
    IRulexSR::Lookup(x) => {
      collect_if(pred, out, NodeRefS::LookupRule(x));
      visit_rune_usage(pred, out, &x.rune);
      visit_imprecise_name(pred, out, &x.name);
    }
    IRulexSR::MaybeCoercingCall(x) => {
      collect_if(pred, out, NodeRefS::MaybeCoercingCallRule(x));
      visit_rune_usage(pred, out, &x.result_rune);
      visit_rune_usage(pred, out, &x.template_rune);
      for arg in &x.args {
        visit_rune_usage(pred, out, arg);
      }
    }
    IRulexSR::RuneParentEnvLookup(x) => {
      visit_rune_usage(pred, out, &x.rune);
    }
    IRulexSR::Augment(x) => {
      collect_if(pred, out, NodeRefS::AugmentRule(x));
      visit_rune_usage(pred, out, &x.result_rune);
      visit_rune_usage(pred, out, &x.inner_rune);
    }
  }
}

fn visit_literal<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, literal: &'s ILiteralSL)
where
  'a: 's,
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_function_declaration_name<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, name: &'s IFunctionDeclarationNameS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

fn visit_imprecise_name<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, name: &'s IImpreciseNameS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::ImpreciseName(name));
  match name {
    IImpreciseNameS::CodeName(x) => collect_if(pred, out, NodeRefS::CodeName(x)),
    _ => panic!("POSTPARSING_TRAVERSE_IMPRECISE_NAME_NOT_YET_IMPLEMENTED"),
  }
}

fn visit_var_name<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, name: &'s IVarNameS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::VarName(name));
}

fn visit_rune_usage<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, rune_usage: &'s RuneUsage<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::RuneUsage(rune_usage));
  visit_rune(pred, out, &rune_usage.rune);
}

fn visit_rune<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, rune: &'s IRuneS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  collect_if(pred, out, NodeRefS::Rune(rune));
  match rune {
    IRuneS::CodeRune(x) => collect_if(pred, out, NodeRefS::CodeRune(x)),
    IRuneS::ImplicitRune(x) => collect_if(pred, out, NodeRefS::ImplicitRune(x)),
    IRuneS::MagicParamRune(x) => collect_if(pred, out, NodeRefS::MagicParamRune(x)),
    IRuneS::DenizenDefaultRegionRune(x) => {
      collect_if(pred, out, NodeRefS::DenizenDefaultRegionRune(x));
      visit_name(pred, out, &x.denizen_name);
    }
    _ => panic!("POSTPARSING_TRAVERSE_RUNE_VARIANT_NOT_YET_IMPLEMENTED"),
  }
}

fn visit_name<'a, 's, T, F>(pred: &F, out: &mut Vec<T>, name: &'s INameS<'a>)
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
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

pub fn collect_in_snode<'a, 's, T, F>(node: &NodeRefS<'a, 's>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefS<'a, 's>) -> Option<T>,
{
  match node {
    NodeRefS::Program(program) => collect_in_program(program, predicate),
    NodeRefS::Struct(strukt) => collect_in_struct(strukt, predicate),
    NodeRefS::Function(function) => {
      let mut out = Vec::new();
      visit_function(predicate, &mut out, function);
      out
    }
    NodeRefS::Impl(impl_) => {
      let mut out = Vec::new();
      visit_impl(predicate, &mut out, impl_);
      out
    }
    NodeRefS::Expression(expression) => collect_in_sexpression(expression, predicate),
    _ => panic!("POSTPARSING_TEST_COLLECT_IN_SNODE_NODE_KIND_NOT_YET_IMPLEMENTED"),
  }
}

#[macro_export]
macro_rules! collect_in_snodes {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    let mut out = Vec::new();
    for node in $expr {
      out.extend($crate::postparsing::test::traverse::collect_in_snode(
        node,
        &|node| match node {
          $pattern => $body,
          _ => None,
        },
      ));
    }
    out
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    let mut out = Vec::new();
    for node in $expr {
      out.extend($crate::postparsing::test::traverse::collect_in_snode(
        node,
        &|node| match node {
          $pattern if $guard => $body,
          _ => None,
        },
      ));
    }
    out
  }};
}

#[macro_export]
macro_rules! collect_where_snodes {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    $crate::collect_in_snodes!($expr, $pattern => $body)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    $crate::collect_in_snodes!($expr, $pattern if $guard => $body)
  }};
}

#[macro_export]
macro_rules! collect_only_snodes {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    let mut matches = $crate::collect_where_snodes!($expr, $pattern => $body);
    assert_eq!(1, matches.len());
    matches.remove(0)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    let mut matches = $crate::collect_where_snodes!($expr, $pattern if $guard => $body);
    assert_eq!(1, matches.len());
    matches.remove(0)
  }};
}

#[macro_export]
macro_rules! collect_where_snode {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    $crate::collect_where_snodes!(&[$expr], $pattern => $body)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    $crate::collect_where_snodes!(&[$expr], $pattern if $guard => $body)
  }};
}

#[macro_export]
macro_rules! collect_only_snode {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    $crate::collect_only_snodes!(&[$expr], $pattern => $body)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    $crate::collect_only_snodes!(&[$expr], $pattern if $guard => $body)
  }};
}
