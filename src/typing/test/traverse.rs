use crate::typing::ast::ast::{
  EdgeT, FunctionDefinitionT, FunctionExportT, FunctionExternT, FunctionHeaderT,
  ICitizenAttributeT, IFunctionAttributeT, InterfaceEdgeBlueprintT, KindExportT, KindExternT,
  OverrideT, ParameterT, PrototypeT, SignatureT,
};
use crate::typing::ast::citizens::{InterfaceDefinitionT, StructDefinitionT, StructMemberT};
use crate::typing::ast::expressions::{
  AddressMemberLookupTE, ArgLookupTE, ArrayLengthTE, ArraySizeTE, AsSubtypeTE, BlockTE,
  BorrowToWeakTE, BreakTE, ConsecutorTE, ConstantBoolTE, ConstantFloatTE, ConstantIntTE,
  ConstantStrTE, ConstructTE, DeferTE, DerefTE, DestroyRuntimeSizedArrayTE,
  DestroyStaticSizedArrayIntoFunctionTE, DestroyStaticSizedArrayIntoLocalsTE, DestroyTE, DiscardTE,
  ExpressionTE, ExternFunctionCallTE, FunctionCallTE, IfTE, InterfaceFunctionCallTE,
  InterfaceToInterfaceUpcastTE, IsSameInstanceTE, LetAndLendTE, LetNormalTE, LocalLookupTE,
  LockWeakTE, MutateTE, NewRuntimeSizedArrayTE, PopRuntimeSizedArrayTE, PushRuntimeSizedArrayTE,
  ReferenceMemberLookupTE, ReinterpretTE, RestackifyTE, ReturnTE, RuntimeSizedArrayCapacityTE,
  RuntimeSizedArrayLookupTE, StaticArrayFromCallableTE, StaticArrayFromValuesTE,
  StaticSizedArrayLookupTE, TupleTE, UnletTE, UpcastTE, VoidLiteralTE, WhileTE,
};
use crate::typing::env::environment::IEnvironmentT;
use crate::typing::env::function_environment_t::LocalVariable;
use crate::typing::hinputs_t::{HinputsT, InstantiationBoundArgumentsT};
use crate::typing::names::names::{INameT, IVarNameT, IdT};
use crate::typing::templata::templata::{
  ExternFunctionTemplataT, FunctionTemplataT, ITemplataT, ImplDefinitionTemplataT,
  InterfaceDefinitionTemplataT, IsaTemplataT, KindListTemplataT, KindTemplataT,
  PlaceholderTemplataT, PrototypeTemplataT, StructDefinitionTemplataT,
};
use crate::typing::types::types::ICitizenTT;
use crate::typing::types::types::ISuperKindTT;
use crate::typing::types::types::{
  InterfaceTT, KindPlaceholderT, KindT, OverloadSetT, RuntimeSizedArrayTT, StaticSizedArrayTT,
  StructTT,
};

pub enum NodeRefT<'s, 't> {
  // ---- Top-level ----
  Hinputs(&'t HinputsT<'s, 't>),
  FunctionDefinition(&'t FunctionDefinitionT<'s, 't>),
  FunctionHeader(&'t FunctionHeaderT<'s, 't>),
  StructDefinition(&'t StructDefinitionT<'s, 't>),
  InterfaceDefinition(&'t InterfaceDefinitionT<'s, 't>),
  Edge(&'t EdgeT<'s, 't>),
  InterfaceEdgeBlueprint(&'t InterfaceEdgeBlueprintT<'s, 't>),
  Parameter(&'t ParameterT<'s, 't>),
  InstantiationBoundArguments(&'t InstantiationBoundArgumentsT<'s, 't>),

  // ---- Expression hierarchy ----
  Expression(ExpressionTE<'s, 't>),

  // 48 reference expression variants
  LetAndLend(&'t LetAndLendTE<'s, 't>),
  LockWeak(&'t LockWeakTE<'s, 't>),
  BorrowToWeak(&'t BorrowToWeakTE<'s, 't>),
  LetNormal(&'t LetNormalTE<'s, 't>),
  Unlet(&'t UnletTE<'s, 't>),
  Discard(&'t DiscardTE<'s, 't>),
  Deref(&'t DerefTE<'s, 't>),
  Defer(&'t DeferTE<'s, 't>),
  If(&'t IfTE<'s, 't>),
  While(&'t WhileTE<'s, 't>),
  Mutate(&'t MutateTE<'s, 't>),
  Restackify(&'t RestackifyTE<'s, 't>),
  Return(&'t ReturnTE<'s, 't>),
  Break(&'t BreakTE<'s, 't>),
  Block(&'t BlockTE<'s, 't>),
  Consecutor(&'t ConsecutorTE<'s, 't>),
  Tuple(&'t TupleTE<'s, 't>),
  StaticArrayFromValues(&'t StaticArrayFromValuesTE<'s, 't>),
  ArraySize(&'t ArraySizeTE<'s, 't>),
  IsSameInstance(&'t IsSameInstanceTE<'s, 't>),
  AsSubtype(&'t AsSubtypeTE<'s, 't>),
  VoidLiteral(&'t VoidLiteralTE<'s, 't>),
  ConstantInt(&'t ConstantIntTE<'s, 't>),
  ConstantBool(&'t ConstantBoolTE<'s, 't>),
  ConstantStr(&'t ConstantStrTE<'s, 't>),
  ConstantFloat(&'t ConstantFloatTE<'s, 't>),
  ArgLookup(&'t ArgLookupTE<'s, 't>),
  ArrayLength(&'t ArrayLengthTE<'s, 't>),
  InterfaceFunctionCall(&'t InterfaceFunctionCallTE<'s, 't>),
  ExternFunctionCall(&'t ExternFunctionCallTE<'s, 't>),
  FunctionCall(&'t FunctionCallTE<'s, 't>),
  Reinterpret(&'t ReinterpretTE<'s, 't>),
  Construct(&'t ConstructTE<'s, 't>),
  NewRuntimeSizedArray(&'t NewRuntimeSizedArrayTE<'s, 't>),
  StaticArrayFromCallable(&'t StaticArrayFromCallableTE<'s, 't>),
  DestroyStaticSizedArrayIntoFunction(&'t DestroyStaticSizedArrayIntoFunctionTE<'s, 't>),
  DestroyStaticSizedArrayIntoLocals(&'t DestroyStaticSizedArrayIntoLocalsTE<'s, 't>),
  DestroyRuntimeSizedArray(&'t DestroyRuntimeSizedArrayTE<'s, 't>),
  RuntimeSizedArrayCapacity(&'t RuntimeSizedArrayCapacityTE<'s, 't>),
  PushRuntimeSizedArray(&'t PushRuntimeSizedArrayTE<'s, 't>),
  PopRuntimeSizedArray(&'t PopRuntimeSizedArrayTE<'s, 't>),
  InterfaceToInterfaceUpcast(&'t InterfaceToInterfaceUpcastTE<'s, 't>),
  Upcast(&'t UpcastTE<'s, 't>),
  Destroy(&'t DestroyTE<'s, 't>),

  // 5 address expression variants
  LocalLookup(&'t LocalLookupTE<'s, 't>),
  StaticSizedArrayLookup(&'t StaticSizedArrayLookupTE<'s, 't>),
  RuntimeSizedArrayLookup(&'t RuntimeSizedArrayLookupTE<'s, 't>),
  ReferenceMemberLookup(&'t ReferenceMemberLookupTE<'s, 't>),
  AddressMemberLookup(&'t AddressMemberLookupTE<'s, 't>),

  // ---- Templata hierarchy ----
  Templata(&'t ITemplataT<'s, 't>),
  KindTemplata(&'t KindTemplataT<'s, 't>),
  PlaceholderTemplata(&'t PlaceholderTemplataT<'s, 't>),
  PrototypeTemplata(&'t PrototypeTemplataT<'s, 't>),
  IsaTemplata(&'t IsaTemplataT<'s, 't>),
  CoordListTemplata(&'t KindListTemplataT<'s, 't>),
  FunctionTemplata(&'t FunctionTemplataT<'s, 't>),
  StructDefinitionTemplata(&'t StructDefinitionTemplataT<'s, 't>),
  InterfaceDefinitionTemplata(&'t InterfaceDefinitionTemplataT<'s, 't>),
  ImplDefinitionTemplata(&'t ImplDefinitionTemplataT<'s, 't>),
  ExternFunctionTemplata(&'t ExternFunctionTemplataT<'s, 't>),

  // ---- Kinds + types ----
  Kind(KindT<'s, 't>),
  StructTT(&'t StructTT<'s, 't>),
  InterfaceTT(&'t InterfaceTT<'s, 't>),
  StaticSizedArrayTT(&'t StaticSizedArrayTT<'s, 't>),
  RuntimeSizedArrayTT(&'t RuntimeSizedArrayTT<'s, 't>),
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
  OverloadSet(&'t OverloadSetT<'s, 't>),
  Id(&'t IdT<'s, 't>),
  Signature(&'t SignatureT<'s, 't>),
  Prototype(&'t PrototypeT<'s, 't>),

  // ---- Names + envs (trait-level only; we do not enumerate sub-variants) ----
  Name(&'t INameT<'s, 't>),
  VarName(&'t IVarNameT<'s, 't>),
  Environment(IEnvironmentT<'s, 't>),

  // ---- Auxiliaries (trait-level only) ----
  FunctionAttribute(&'t IFunctionAttributeT<'s>),
  CitizenAttribute(&'t ICitizenAttributeT<'s>),
  StructMember(&'t StructMemberT<'s, 't>),
  LocalVariable(&'t LocalVariable<'s, 't>),

  // ---- Override / Edge children ----
  Override(&'t OverrideT<'s, 't>),

  // ---- Exports / externs ----
  KindExport(&'t KindExportT<'s, 't>),
  FunctionExport(&'t FunctionExportT<'s, 't>),
  KindExtern(&'t KindExternT<'s, 't>),
  FunctionExtern(&'t FunctionExternT<'s, 't>),
}

fn collect_if<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, node: NodeRefT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
{
  if let Some(v) = pred(node) {
    out.push(v);
  }
}

// ============================================================================
// Public entry points
// ============================================================================

pub fn collect_in_hinputs<'s, 't, T, F>(hinputs: &'t HinputsT<'s, 't>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  let mut out = Vec::new();
  visit_hinputs(predicate, &mut out, hinputs);
  out
}

pub fn collect_in_function<'s, 't, T, F>(
  func: &'t FunctionDefinitionT<'s, 't>,
  predicate: &F,
) -> Vec<T>
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  let mut out = Vec::new();
  visit_function_definition(predicate, &mut out, func);
  out
}

pub fn collect_in_struct<'s, 't, T, F>(s: &'t StructDefinitionT<'s, 't>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  let mut out = Vec::new();
  visit_struct_definition(predicate, &mut out, s);
  out
}

pub fn collect_in_interface<'s, 't, T, F>(
  i: &'t InterfaceDefinitionT<'s, 't>,
  predicate: &F,
) -> Vec<T>
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  let mut out = Vec::new();
  visit_interface_definition(predicate, &mut out, i);
  out
}

pub fn collect_in_expression<'s, 't, T, F>(e: ExpressionTE<'s, 't>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  let mut out = Vec::new();
  visit_expression_te(predicate, &mut out, e);
  out
}

pub fn collect_in_templata<'s, 't, T, F>(t: &'t ITemplataT<'s, 't>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  let mut out = Vec::new();
  visit_templata(predicate, &mut out, t);
  out
}

pub fn collect_in_kind<'s, 't, T, F>(k: KindT<'s, 't>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  let mut out = Vec::new();
  visit_kind(predicate, &mut out, k);
  out
}

// ============================================================================
// Top-level visitors
// ============================================================================

fn visit_hinputs<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, h: &'t HinputsT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Hinputs(h));
  for i in &h.interfaces {
    visit_interface_definition(pred, out, i);
  }
  for s in &h.structs {
    visit_struct_definition(pred, out, s);
  }
  for f in &h.functions {
    visit_function_definition(pred, out, f);
  }
  for blueprint in h.interface_to_edge_blueprints.values() {
    visit_interface_edge_blueprint(pred, out, blueprint);
  }
  for sub_to_edge in h.interface_to_sub_citizen_to_edge.values() {
    for edge in sub_to_edge.values() {
      visit_edge(pred, out, edge);
    }
  }
  for bounds in h.instantiation_name_to_instantiation_bounds.values() {
    visit_instantiation_bound_arguments(pred, out, bounds);
  }
  for ke in &h.kind_exports {
    visit_kind_export(pred, out, ke);
  }
  for fe in &h.function_exports {
    visit_function_export(pred, out, fe);
  }
  for ke in &h.kind_externs {
    visit_kind_extern(pred, out, ke);
  }
  for fe in &h.function_externs {
    visit_function_extern(pred, out, fe);
  }
}

fn visit_function_definition<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  f: &'t FunctionDefinitionT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::FunctionDefinition(f));
  visit_function_header(pred, out, f.header);
  visit_instantiation_bound_arguments(pred, out, f.instantiation_bound_params);
  visit_expression_te(pred, out, f.body);
}

fn visit_function_header<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, h: &'t FunctionHeaderT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::FunctionHeader(h));
  visit_id(pred, out, &h.id);
  for attr in h.attributes {
    visit_function_attribute(pred, out, attr);
  }
  for param in h.params {
    visit_parameter(pred, out, param);
  }
  visit_kind(pred, out, h.return_type);
  if let Some(t) = &h.maybe_origin_function_templata {
    visit_function_templata(pred, out, t);
  }
}

fn visit_struct_definition<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  s: &'t StructDefinitionT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::StructDefinition(s));
  visit_id(pred, out, &s.template_name);
  visit_struct_tt(pred, out, &s.instantiated_citizen);
  for attr in s.attributes {
    visit_citizen_attribute(pred, out, attr);
  }
  // mutability is now a plain SharednessT enum, not a templata — nothing to visit.
  let _ = &s.sharedness;
  for member in s.members {
    visit_struct_member(pred, out, member);
  }
  visit_instantiation_bound_arguments(pred, out, s.instantiation_bound_params);
}

fn visit_interface_definition<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  i: &'t InterfaceDefinitionT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::InterfaceDefinition(i));
  visit_id(pred, out, &i.template_name);
  visit_interface_tt(pred, out, &i.instantiated_interface);
  visit_interface_tt(pred, out, &i.ref_);
  for attr in i.attributes {
    visit_citizen_attribute(pred, out, attr);
  }
  let _ = &i.sharedness;
  visit_instantiation_bound_arguments(pred, out, i.instantiation_bound_params);
  for (proto, _idx) in i.internal_methods {
    visit_prototype(pred, out, proto);
  }
}

fn visit_edge<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, e: &'t EdgeT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Edge(e));
  visit_id(pred, out, &e.edge_id);
  visit_kind_citizen(pred, out, &e.sub_citizen);
  visit_id(pred, out, &e.super_interface);
  visit_instantiation_bound_arguments(pred, out, e.instantiation_bound_params);
  for (id, ovr) in &e.abstract_func_to_override_func {
    visit_id(pred, out, id);
    visit_override(pred, out, ovr);
  }
}

fn visit_kind_citizen<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, c: &'t ICitizenTT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  match c {
    ICitizenTT::Struct(s) => visit_struct_tt(pred, out, s),
    ICitizenTT::Interface(i) => visit_interface_tt(pred, out, i),
  }
}

fn visit_override<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, o: &'t OverrideT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Override(o));
  visit_id(pred, out, &o.dispatcher_call_id);
  for (id, t) in o.impl_placeholder_to_dispatcher_placeholder {
    visit_id(pred, out, id);
    visit_templata(pred, out, t);
  }
  for (id, t) in o.impl_placeholder_to_case_placeholder {
    visit_id(pred, out, id);
    visit_templata(pred, out, t);
  }
  visit_id(pred, out, &o.case_id);
  visit_prototype(pred, out, &o.override_prototype);
  visit_instantiation_bound_arguments(pred, out, o.dispatcher_instantiation_bound_params);
}

fn visit_interface_edge_blueprint<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  b: &'t InterfaceEdgeBlueprintT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::InterfaceEdgeBlueprint(b));
  visit_id(pred, out, &b.interface);
  for (proto, _idx) in b.super_family_root_headers {
    visit_prototype(pred, out, proto);
  }
}

fn visit_parameter<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, p: &'t ParameterT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Parameter(p));
  visit_var_name(pred, out, &p.name);
  visit_kind(pred, out, p.tyype);
}

fn visit_instantiation_bound_arguments<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  b: &'t InstantiationBoundArgumentsT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::InstantiationBoundArguments(b));
  for (_rune, proto) in &b.rune_to_bound_prototype {
    visit_prototype(pred, out, proto);
  }
  for (_rune, reachable) in &b.rune_to_citizen_rune_to_reachable_prototype {
    for (_rune2, proto) in &reachable.citizen_rune_to_reachable_prototype {
      visit_prototype(pred, out, proto);
    }
  }
  for (_rune, id) in &b.rune_to_bound_impl {
    visit_id(pred, out, id);
  }
}

// ============================================================================
// Expression hierarchy visitors
// ============================================================================

fn visit_expression_te<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, e: ExpressionTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Expression(e));
  match e {
    ExpressionTE::LetAndLend(x) => visit_let_and_lend(pred, out, x),
    ExpressionTE::LockWeak(x) => visit_lock_weak(pred, out, x),
    ExpressionTE::BorrowToWeak(x) => visit_borrow_to_weak(pred, out, x),
    ExpressionTE::LetNormal(x) => visit_let_normal(pred, out, x),
    ExpressionTE::Unlet(x) => visit_unlet(pred, out, x),
    ExpressionTE::Discard(x) => visit_discard(pred, out, x),
    ExpressionTE::Defer(x) => visit_defer(pred, out, x),
    ExpressionTE::If(x) => visit_if(pred, out, x),
    ExpressionTE::While(x) => visit_while(pred, out, x),
    ExpressionTE::Mutate(x) => visit_mutate(pred, out, x),
    ExpressionTE::Restackify(x) => visit_restackify(pred, out, x),
    ExpressionTE::Return(x) => visit_return(pred, out, x),
    ExpressionTE::Break(x) => visit_break(pred, out, x),
    ExpressionTE::Block(x) => visit_block(pred, out, x),
    ExpressionTE::Consecutor(x) => visit_consecutor(pred, out, x),
    ExpressionTE::Tuple(x) => visit_tuple(pred, out, x),
    ExpressionTE::StaticArrayFromValues(x) => visit_static_array_from_values(pred, out, x),
    ExpressionTE::ArraySize(x) => visit_array_size(pred, out, x),
    ExpressionTE::IsSameInstance(x) => visit_is_same_instance(pred, out, x),
    ExpressionTE::AsSubtype(x) => visit_as_subtype(pred, out, x),
    ExpressionTE::VoidLiteral(x) => visit_void_literal(pred, out, x),
    ExpressionTE::ConstantInt(x) => visit_constant_int(pred, out, x),
    ExpressionTE::ConstantBool(x) => visit_constant_bool(pred, out, x),
    ExpressionTE::ConstantStr(x) => visit_constant_str(pred, out, x),
    ExpressionTE::ConstantFloat(x) => visit_constant_float(pred, out, x),
    ExpressionTE::ArgLookup(x) => visit_arg_lookup(pred, out, x),
    ExpressionTE::ArrayLength(x) => visit_array_length(pred, out, x),
    ExpressionTE::InterfaceFunctionCall(x) => visit_interface_function_call(pred, out, x),
    ExpressionTE::ExternFunctionCall(x) => visit_extern_function_call(pred, out, x),
    ExpressionTE::FunctionCall(x) => visit_function_call(pred, out, x),
    ExpressionTE::Reinterpret(x) => visit_reinterpret(pred, out, x),
    ExpressionTE::Construct(x) => visit_construct(pred, out, x),
    ExpressionTE::NewRuntimeSizedArray(x) => visit_new_mut_runtime_sized_array(pred, out, x),
    ExpressionTE::StaticArrayFromCallable(x) => visit_static_array_from_callable(pred, out, x),
    ExpressionTE::DestroyStaticSizedArrayIntoFunction(x) => {
      visit_destroy_static_sized_array_into_function(pred, out, x)
    }
    ExpressionTE::DestroyStaticSizedArrayIntoLocals(x) => {
      visit_destroy_static_sized_array_into_locals(pred, out, x)
    }
    ExpressionTE::DestroyRuntimeSizedArray(x) => {
      visit_destroy_mut_runtime_sized_array(pred, out, x)
    }
    ExpressionTE::RuntimeSizedArrayCapacity(x) => visit_runtime_sized_array_capacity(pred, out, x),
    ExpressionTE::PushRuntimeSizedArray(x) => visit_push_runtime_sized_array(pred, out, x),
    ExpressionTE::PopRuntimeSizedArray(x) => visit_pop_runtime_sized_array(pred, out, x),
    ExpressionTE::InterfaceToInterfaceUpcast(x) => {
      visit_interface_to_interface_upcast(pred, out, x)
    }
    ExpressionTE::Upcast(x) => visit_upcast(pred, out, x),
    ExpressionTE::Destroy(x) => visit_destroy(pred, out, x),
    ExpressionTE::CopyPrim(x) => visit_expression_te(pred, out, x.inner),
    ExpressionTE::LocalLookup(x) => visit_local_lookup(pred, out, x),
    ExpressionTE::StaticSizedArrayLookup(x) => visit_static_sized_array_lookup(pred, out, x),
    ExpressionTE::RuntimeSizedArrayLookup(x) => visit_runtime_sized_array_lookup(pred, out, x),
    ExpressionTE::ReferenceMemberLookup(x) => visit_reference_member_lookup(pred, out, x),
    ExpressionTE::AddressMemberLookup(x) => visit_address_member_lookup(pred, out, x),
    ExpressionTE::Deref(x) => visit_deref(pred, out, x),
  }
}

// ---- 48 reference expression variant visitors ----

fn visit_let_and_lend<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t LetAndLendTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::LetAndLend(x));
  visit_local_variable(pred, out, &x.variable);
  visit_expression_te(pred, out, x.expr);
}

fn visit_lock_weak<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t LockWeakTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::LockWeak(x));
  visit_expression_te(pred, out, x.inner_expr);
  visit_kind(pred, out, x.result);
  visit_prototype(pred, out, x.some_constructor);
  visit_prototype(pred, out, x.none_constructor);
  visit_id(pred, out, &x.some_impl_name);
  visit_id(pred, out, &x.none_impl_name);
}

fn visit_borrow_to_weak<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t BorrowToWeakTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::BorrowToWeak(x));
  visit_expression_te(pred, out, x.inner_expr);
}

fn visit_let_normal<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t LetNormalTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::LetNormal(x));
  visit_local_variable(pred, out, &x.variable);
  visit_expression_te(pred, out, x.expr);
}

fn visit_unlet<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t UnletTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Unlet(x));
  visit_local_variable(pred, out, &x.variable);
}

fn visit_discard<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t DiscardTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Discard(x));
  visit_expression_te(pred, out, x.expr);
}

fn visit_deref<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t DerefTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Deref(x));
  visit_expression_te(pred, out, x.inner);
}

fn visit_defer<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t DeferTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Defer(x));
  visit_expression_te(pred, out, x.inner_expr);
  visit_expression_te(pred, out, x.deferred_expr);
}

fn visit_if<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t IfTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::If(x));
  visit_expression_te(pred, out, x.condition);
  visit_expression_te(pred, out, x.then_call);
  visit_expression_te(pred, out, x.else_call);
}

fn visit_while<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t WhileTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::While(x));
  visit_block(pred, out, &x.block);
}

fn visit_mutate<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t MutateTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Mutate(x));
  visit_expression_te(pred, out, x.destination_expr);
  visit_expression_te(pred, out, x.source_expr);
}

fn visit_restackify<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t RestackifyTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Restackify(x));
  visit_local_variable(pred, out, &x.variable);
  visit_expression_te(pred, out, x.source_expr);
}

fn visit_return<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ReturnTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Return(x));
  visit_expression_te(pred, out, x.source_expr);
}

fn visit_break<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t BreakTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Break(x));
}

fn visit_block<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t BlockTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Block(x));
  visit_expression_te(pred, out, x.inner);
}

fn visit_consecutor<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ConsecutorTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Consecutor(x));
  for e in x.exprs {
    visit_expression_te(pred, out, *e);
  }
}

fn visit_tuple<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t TupleTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Tuple(x));
  for e in x.elements {
    visit_expression_te(pred, out, *e);
  }
  visit_kind(pred, out, x.result);
}

fn visit_static_array_from_values<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t StaticArrayFromValuesTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::StaticArrayFromValues(x));
  for e in x.elements {
    visit_expression_te(pred, out, *e);
  }
  visit_kind(pred, out, x.result);
  visit_static_sized_array_tt(pred, out, x.array_type);
}

fn visit_array_size<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ArraySizeTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ArraySize(x));
  visit_expression_te(pred, out, x.array);
}

fn visit_is_same_instance<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t IsSameInstanceTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::IsSameInstance(x));
  visit_expression_te(pred, out, x.left);
  visit_expression_te(pred, out, x.right);
}

fn visit_as_subtype<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t AsSubtypeTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::AsSubtype(x));
  visit_expression_te(pred, out, x.source_expr);
  visit_kind(pred, out, x.target_type);
  visit_kind(pred, out, x.result);
  visit_prototype(pred, out, x.ok_constructor);
  visit_prototype(pred, out, x.err_constructor);
  visit_id(pred, out, &x.impl_name);
  visit_id(pred, out, &x.ok_impl_name);
  visit_id(pred, out, &x.err_impl_name);
}

fn visit_void_literal<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t VoidLiteralTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::VoidLiteral(x));
}

fn visit_constant_int<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ConstantIntTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ConstantInt(x));
  visit_templata(pred, out, &x.value);
}

fn visit_constant_bool<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ConstantBoolTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ConstantBool(x));
}

fn visit_constant_str<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ConstantStrTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ConstantStr(x));
}

fn visit_constant_float<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ConstantFloatTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ConstantFloat(x));
}

fn visit_arg_lookup<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ArgLookupTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ArgLookup(x));
  visit_kind(pred, out, x.result);
}

fn visit_array_length<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ArrayLengthTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ArrayLength(x));
  visit_expression_te(pred, out, x.array_expr);
}

fn visit_interface_function_call<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t InterfaceFunctionCallTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::InterfaceFunctionCall(x));
  visit_prototype(pred, out, x.super_function_prototype);
  visit_kind(pred, out, x.result);
  for a in x.args {
    visit_expression_te(pred, out, *a);
  }
}

fn visit_extern_function_call<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t ExternFunctionCallTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ExternFunctionCall(x));
  visit_prototype(pred, out, x.prototype2);
  for a in x.args {
    visit_expression_te(pred, out, *a);
  }
}

fn visit_function_call<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t FunctionCallTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::FunctionCall(x));
  visit_prototype(pred, out, x.callable);
  for a in x.args {
    visit_expression_te(pred, out, *a);
  }
  visit_kind(pred, out, x.result);
}

fn visit_reinterpret<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ReinterpretTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Reinterpret(x));
  visit_expression_te(pred, out, x.expr);
  visit_kind(pred, out, x.result);
}

fn visit_construct<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t ConstructTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Construct(x));
  visit_struct_tt(pred, out, x.struct_tt);
  visit_kind(pred, out, x.result);
  for a in x.args {
    visit_expression_te(pred, out, *a);
  }
}

fn visit_new_mut_runtime_sized_array<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t NewRuntimeSizedArrayTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::NewRuntimeSizedArray(x));
  visit_runtime_sized_array_tt(pred, out, x.array_type);
  visit_expression_te(pred, out, x.capacity_expr);
}

fn visit_static_array_from_callable<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t StaticArrayFromCallableTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::StaticArrayFromCallable(x));
  visit_static_sized_array_tt(pred, out, x.array_type);
  visit_expression_te(pred, out, x.generator);
  visit_prototype(pred, out, x.generator_method);
}

fn visit_destroy_static_sized_array_into_function<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t DestroyStaticSizedArrayIntoFunctionTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::DestroyStaticSizedArrayIntoFunction(x));
  visit_expression_te(pred, out, x.array_expr);
  visit_static_sized_array_tt(pred, out, x.array_type);
  visit_expression_te(pred, out, x.consumer);
  visit_prototype(pred, out, x.consumer_method);
}

fn visit_destroy_static_sized_array_into_locals<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t DestroyStaticSizedArrayIntoLocalsTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::DestroyStaticSizedArrayIntoLocals(x));
  visit_expression_te(pred, out, x.expr);
  visit_static_sized_array_tt(pred, out, x.static_sized_array);
  // destination_reference_variables: ReferenceLocalVariableT — stop at trait level
}

fn visit_destroy_mut_runtime_sized_array<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t DestroyRuntimeSizedArrayTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::DestroyRuntimeSizedArray(x));
  visit_expression_te(pred, out, x.array_expr);
}

fn visit_runtime_sized_array_capacity<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t RuntimeSizedArrayCapacityTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::RuntimeSizedArrayCapacity(x));
  visit_expression_te(pred, out, x.array_expr);
}

fn visit_push_runtime_sized_array<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t PushRuntimeSizedArrayTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::PushRuntimeSizedArray(x));
  visit_expression_te(pred, out, x.array_expr);
  visit_expression_te(pred, out, x.new_element_expr);
}

fn visit_pop_runtime_sized_array<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t PopRuntimeSizedArrayTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::PopRuntimeSizedArray(x));
  visit_expression_te(pred, out, x.array_expr);
}

fn visit_interface_to_interface_upcast<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t InterfaceToInterfaceUpcastTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::InterfaceToInterfaceUpcast(x));
  visit_expression_te(pred, out, x.inner_expr);
  visit_interface_tt(pred, out, x.target_interface);
}

fn visit_upcast<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t UpcastTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Upcast(x));
  visit_expression_te(pred, out, x.inner_expr);
  visit_super_kind(pred, out, &x.target_super_kind);
  visit_id(pred, out, &x.impl_name);
}

fn visit_super_kind<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, s: &'t ISuperKindTT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  match s {
    ISuperKindTT::Interface(i) => visit_interface_tt(pred, out, i),
    ISuperKindTT::KindPlaceholder(p) => visit_kind_placeholder(pred, out, p),
  }
}

fn visit_destroy<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t DestroyTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Destroy(x));
  visit_expression_te(pred, out, x.expr);
  visit_struct_tt(pred, out, x.struct_tt);
  // destination_reference_variables: ReferenceLocalVariableT — stop at trait level
}

// ---- 5 address expression variant visitors ----

fn visit_local_lookup<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t LocalLookupTE<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::LocalLookup(x));
  visit_local_variable(pred, out, &x.local_variable);
}

fn visit_static_sized_array_lookup<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t StaticSizedArrayLookupTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::StaticSizedArrayLookup(x));
  visit_expression_te(pred, out, x.array_expr);
  visit_static_sized_array_tt(pred, out, x.array_type);
  visit_expression_te(pred, out, x.index_expr);
  visit_kind(pred, out, KindT::BorrowRef(x.result));
}

fn visit_runtime_sized_array_lookup<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t RuntimeSizedArrayLookupTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::RuntimeSizedArrayLookup(x));
  visit_expression_te(pred, out, x.array_expr);
  visit_runtime_sized_array_tt(pred, out, x.array_type);
  visit_expression_te(pred, out, x.index_expr);
}

fn visit_reference_member_lookup<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t ReferenceMemberLookupTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ReferenceMemberLookup(x));
  visit_expression_te(pred, out, x.struct_expr);
  visit_var_name(pred, out, &x.member_name);
  visit_kind(pred, out, KindT::BorrowRef(x.result));
}

fn visit_address_member_lookup<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t AddressMemberLookupTE<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::AddressMemberLookup(x));
  visit_expression_te(pred, out, x.struct_expr);
  visit_var_name(pred, out, &x.member_name);
  visit_kind(pred, out, KindT::BorrowRef(x.result));
}

// ============================================================================
// Templata hierarchy
// ============================================================================

fn visit_templata<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, t: &'t ITemplataT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Templata(t));
  match t {
    ITemplataT::Kind(x) => visit_kind_templata(pred, out, x),
    ITemplataT::Placeholder(x) => visit_placeholder_templata(pred, out, x),
    ITemplataT::Integer(_) => {}
    ITemplataT::Boolean(_) => {}
    ITemplataT::String(_) => {}
    ITemplataT::Group(_) => {}
    ITemplataT::Prototype(x) => visit_prototype_templata(pred, out, x),
    ITemplataT::Isa(x) => visit_isa_templata(pred, out, x),
    ITemplataT::CoordList(x) => visit_coord_list_templata(pred, out, x),
    ITemplataT::RuntimeSizedArrayTemplate(_) => {}
    ITemplataT::StaticSizedArrayTemplate(_) => {}
    ITemplataT::Function(x) => visit_function_templata(pred, out, x),
    ITemplataT::StructDefinition(x) => visit_struct_definition_templata(pred, out, x),
    ITemplataT::InterfaceDefinition(x) => visit_interface_definition_templata(pred, out, x),
    ITemplataT::ImplDefinition(x) => visit_impl_definition_templata(pred, out, x),
    ITemplataT::ExternFunction(x) => visit_extern_function_templata(pred, out, x),
  }
}

fn visit_kind_templata<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t KindTemplataT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::KindTemplata(x));
  visit_kind(pred, out, x.kind);
}

fn visit_placeholder_templata<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t PlaceholderTemplataT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::PlaceholderTemplata(x));
  visit_id(pred, out, &x.id);
}

fn visit_prototype_templata<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t PrototypeTemplataT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::PrototypeTemplata(x));
  visit_prototype(pred, out, x.prototype);
}

fn visit_isa_templata<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, x: &'t IsaTemplataT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::IsaTemplata(x));
  visit_id(pred, out, &x.impl_name);
  visit_kind(pred, out, x.sub_kind);
  visit_kind(pred, out, x.super_kind);
}

fn visit_coord_list_templata<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t KindListTemplataT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::CoordListTemplata(x));
  for c in x.kinds {
    visit_kind(pred, out, *c);
  }
}

fn visit_function_templata<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t FunctionTemplataT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::FunctionTemplata(x));
  // Stop at trait level for env / FunctionS — see TL.md "What This Plan Deliberately Does NOT Cover".
}

fn visit_struct_definition_templata<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t StructDefinitionTemplataT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::StructDefinitionTemplata(x));
}

fn visit_interface_definition_templata<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t InterfaceDefinitionTemplataT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::InterfaceDefinitionTemplata(x));
}

fn visit_impl_definition_templata<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t ImplDefinitionTemplataT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ImplDefinitionTemplata(x));
}

fn visit_extern_function_templata<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  x: &'t ExternFunctionTemplataT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::ExternFunctionTemplata(x));
  visit_function_header(pred, out, x.header);
}

// ============================================================================
// Kinds + types
// ============================================================================

/// Emits a node for every onion layer on the way down, then for the base kind.
fn visit_kind<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, k: KindT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Kind(k));
  match k {
    KindT::Never(_) => {}
    KindT::Void(_) => {}
    KindT::Int(_) => {}
    KindT::Bool(_) => {}
    KindT::Str(_) => {}
    KindT::Float(_) => {}
    KindT::USize(_) => {}
    KindT::Struct(s) => visit_struct_tt(pred, out, s),
    KindT::Interface(i) => visit_interface_tt(pred, out, i),
    KindT::StaticSizedArray(a) => visit_static_sized_array_tt(pred, out, a),
    KindT::RuntimeSizedArray(a) => visit_runtime_sized_array_tt(pred, out, a),
    KindT::KindPlaceholder(p) => visit_kind_placeholder(pred, out, p),
    KindT::OverloadSet(o) => visit_overload_set(pred, out, o),
    KindT::BorrowRef(b) => visit_kind(pred, out, b.inner),
    KindT::OwnRef(h) => visit_kind(pred, out, h.inner),
    KindT::ShareRef(s) => visit_kind(pred, out, s.inner),
    KindT::WeakRef(w) => visit_kind(pred, out, w.inner),
  }
}

fn visit_struct_tt<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, s: &'t StructTT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::StructTT(s));
  visit_id(pred, out, &s.id);
}

fn visit_interface_tt<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, i: &'t InterfaceTT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::InterfaceTT(i));
  visit_id(pred, out, &i.id);
}

fn visit_static_sized_array_tt<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  a: &'t StaticSizedArrayTT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::StaticSizedArrayTT(a));
  visit_id(pred, out, &a.name);
}

fn visit_runtime_sized_array_tt<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  a: &'t RuntimeSizedArrayTT<'s, 't>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::RuntimeSizedArrayTT(a));
  visit_id(pred, out, &a.name);
}

fn visit_kind_placeholder<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, p: &'t KindPlaceholderT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::KindPlaceholder(p));
  visit_id(pred, out, &p.id);
}

fn visit_overload_set<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, o: &'t OverloadSetT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::OverloadSet(o));
  // Stop at trait level for env — see TL.md "What This Plan Deliberately Does NOT Cover".
}

fn visit_id<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, id: &'t IdT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Id(id));
  // Stop at trait level for INameT — see TL.md "What This Plan Deliberately Does NOT Cover".
}

fn visit_signature<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, s: &'t SignatureT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Signature(s));
  visit_id(pred, out, &s.id);
}

fn visit_prototype<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, p: &'t PrototypeT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::Prototype(p));
  visit_id(pred, out, &p.id);
  visit_kind(pred, out, p.return_type);
}

// ============================================================================
// Names / envs / aux (trait-level only — no descent)
// ============================================================================

fn visit_var_name<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, n: &'t IVarNameT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::VarName(n));
}

fn visit_function_attribute<'s, 't, T, F>(
  pred: &F,
  out: &mut Vec<T>,
  a: &'t IFunctionAttributeT<'s>,
) where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::FunctionAttribute(a));
}

fn visit_citizen_attribute<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, a: &'t ICitizenAttributeT<'s>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::CitizenAttribute(a));
}

fn visit_struct_member<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, m: &'t StructMemberT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::StructMember(m));
  visit_var_name(pred, out, &m.name);
  visit_kind(pred, out, m.tyype);
}

fn visit_local_variable<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, v: &'t LocalVariable<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::LocalVariable(v));
}

// ============================================================================
// Exports / externs
// ============================================================================

fn visit_kind_export<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, e: &'t KindExportT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::KindExport(e));
  visit_kind(pred, out, e.tyype);
  visit_id(pred, out, &e.id);
}

fn visit_function_export<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, e: &'t FunctionExportT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::FunctionExport(e));
  visit_prototype(pred, out, &e.prototype);
  visit_id(pred, out, &e.export_id);
}

fn visit_kind_extern<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, e: &'t KindExternT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::KindExtern(e));
  visit_kind(pred, out, e.tyype);
}

fn visit_function_extern<'s, 't, T, F>(pred: &F, out: &mut Vec<T>, e: &'t FunctionExternT<'s, 't>)
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  collect_if(pred, out, NodeRefT::FunctionExtern(e));
  visit_id(pred, out, &e.extern_placeholdered_id);
  visit_prototype(pred, out, &e.prototype);
}

// ============================================================================
// Dispatcher
// ============================================================================

pub fn collect_in_tnode<'s, 't, T, F>(node: &NodeRefT<'s, 't>, predicate: &F) -> Vec<T>
where
  F: Fn(NodeRefT<'s, 't>) -> Option<T>,
  's: 't,
{
  let mut out = Vec::new();
  match node {
    NodeRefT::Hinputs(h) => visit_hinputs(predicate, &mut out, h),
    NodeRefT::FunctionDefinition(f) => visit_function_definition(predicate, &mut out, f),
    NodeRefT::StructDefinition(s) => visit_struct_definition(predicate, &mut out, s),
    NodeRefT::InterfaceDefinition(i) => visit_interface_definition(predicate, &mut out, i),
    NodeRefT::Expression(e) => visit_expression_te(predicate, &mut out, *e),
    NodeRefT::Templata(t) => visit_templata(predicate, &mut out, t),
    NodeRefT::Kind(k) => visit_kind(predicate, &mut out, *k),
    _ => panic!("TYPING_TEST_COLLECT_IN_TNODE_NODE_KIND_NOT_YET_IMPLEMENTED"),
  }
  out
}

// ============================================================================
// Macros (verbatim-ported from postparsing/test/traverse.rs)
// ============================================================================

#[macro_export]
macro_rules! collect_in_tnodes {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    let mut out = Vec::new();
    for node in $expr {
      out.extend($crate::typing::test::traverse::collect_in_tnode(node, &|node| match node {
        $pattern => $body,
        _ => None,
      }));
    }
    out
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    let mut out = Vec::new();
    for node in $expr {
      out.extend($crate::typing::test::traverse::collect_in_tnode(node, &|node| match node {
        $pattern if $guard => $body,
        _ => None,
      }));
    }
    out
  }};
}

#[macro_export]
macro_rules! collect_where_tnodes {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    $crate::collect_in_tnodes!($expr, $pattern => $body)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    $crate::collect_in_tnodes!($expr, $pattern if $guard => $body)
  }};
}

#[macro_export]
macro_rules! collect_only_tnodes {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    let mut matches = $crate::collect_where_tnodes!($expr, $pattern => $body);
    assert_eq!(1, matches.len());
    matches.remove(0)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    let mut matches = $crate::collect_where_tnodes!($expr, $pattern if $guard => $body);
    assert_eq!(1, matches.len());
    matches.remove(0)
  }};
}

#[macro_export]
macro_rules! collect_where_tnode {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    $crate::collect_where_tnodes!(&[$expr], $pattern => $body)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    $crate::collect_where_tnodes!(&[$expr], $pattern if $guard => $body)
  }};
}

#[macro_export]
macro_rules! collect_only_tnode {
  ($expr:expr, $pattern:pat => $body:expr) => {{
    $crate::collect_only_tnodes!(&[$expr], $pattern => $body)
  }};
  ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {{
    $crate::collect_only_tnodes!(&[$expr], $pattern if $guard => $body)
  }};
}
