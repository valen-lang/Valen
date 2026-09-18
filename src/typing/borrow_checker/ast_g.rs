

use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::typing::ast::ast::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::names::names::*;
use crate::typing::templata::templata::*;
use crate::typing::templata_compiler::{is_ref, peel_one_reference, replace_value_type_in_ref};
use crate::typing::types::types::BoolT;
use crate::typing::types::types::FloatT;
use crate::typing::types::types::IntT;
use crate::typing::types::types::RegionT;
use crate::typing::types::types::SharednessT;
use crate::typing::types::types::*;
use crate::typing::types::types::{KindT, NeverT, VoidT};
use crate::typing::typing_interner::TypingInterner;
use std::any::Any;
use std::marker::PhantomData;
use crate::postparsing::names::IRuneS;
use crate::typing::ast::expressions::ExpressionTE;
use crate::typing::borrow_checker::kind_g::{BorrowRefGT, ISuperKindGT, InterfaceGT, KindGT, RuntimeSizedArrayGT, ShareRefGT, StaticSizedArrayGT, StructGT, WeakRefGT};
use crate::typing::borrow_checker::templata_g::ITemplataG;

// A specific mutation to a specific group (as opposed to GroupExprG which an expression for expressing the group(s) a ref might point at).
#[derive(Debug)]
pub struct MutEffectPath<'s, 't, 'g> {
  pub effecting_node_loc: LocT<'t>, // Which expr had this mut effect (e.g. loc of `level.tiles.clear()`)
  pub steps: &'g [&'g GroupStep<'s, 't>], // What group the effect mutated (e.g. ["level", "tiles"])
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GroupStep<'s, 't> {
  Rune(&'s IRuneS<'s>), // a group param, e.g. <g'>, resolved to its id
  ParamAnonymousGroup(&'t IVarNameT<'s, 't>), // A param's group if it doesn't come from a rune or another param. The string is the param name
  Local(&'t IVarNameT<'s, 't>), // A local's implicitly declared group.
  Member { member_name: StrI<'s> }, // `x.items`
  ChildElements, // the `[]` part of `x.items[]` if items is a Box/Vec/RSA
  InlineElements, // the `[]` part of `x.items[]` if items is a SSA.
  Variant { variant_name: StrI<'s> }, // an enum's variant, the `WarpEngine` part of `my_ship.engine_enum.WarpEngine`
  // No `Empty` variant, that just becomes not a MutEffectPath at all.
  // No `Union` variant, that just becomes multiple MutEffectPath.
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LocalVariableG<'s, 't, 'g>
where
    's: 't,
{
  pub name: IVarNameT<'s, 't>,
  pub tyype: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Copy, Clone, Debug)]
pub enum ExpressionGE<'s, 't, 'g> {
  LetAndLend(&'g LetAndLendGE<'s, 't, 'g>),
  LockWeak(&'g LockWeakGE<'s, 't, 'g>),
  BorrowToWeak(&'g BorrowToWeakGE<'s, 't, 'g>),
  LetNormal(&'g LetNormalGE<'s, 't, 'g>),
  Unlet(&'g UnletGE<'s, 't, 'g>),
  Discard(&'g DiscardGE<'s, 't, 'g>),
  If(&'g IfGE<'s, 't, 'g>),
  While(&'g WhileGE<'s, 't, 'g>),
  Mutate(&'g MutateGE<'s, 't, 'g>),
  Restackify(&'g RestackifyGE<'s, 't, 'g>),
  Return(&'g ReturnGE<'s, 't, 'g>),
  Break(&'g BreakGE<'s, 't, 'g>),
  Block(&'g BlockGE<'s, 't, 'g>),
  Consecutor(&'g ConsecutorGE<'s, 't, 'g>),
  StaticArrayFromValues(&'g StaticArrayFromValuesGE<'s, 't, 'g>),
  ArraySize(&'g ArraySizeGE<'s, 't, 'g>),
  IsSameInstance(&'g IsSameInstanceGE<'s, 't, 'g>),
  AsSubtype(&'g AsSubtypeGE<'s, 't, 'g>),
  VoidLiteral(&'g VoidLiteralGE<'s, 't, 'g>),
  ConstantInt(&'g ConstantIntGE<'s, 't, 'g>),
  ConstantBool(&'g ConstantBoolGE<'s, 't, 'g>),
  ConstantStr(&'g ConstantStrGE<'s, 't, 'g>),
  ConstantFloat(&'g ConstantFloatGE<'s, 't, 'g>),
  ArgLookup(&'g ArgLookupGE<'s, 't, 'g>),
  ArrayLength(&'g ArrayLengthGE<'s, 't, 'g>),
  InterfaceFunctionCall(&'g InterfaceFunctionCallGE<'s, 't, 'g>),
  ExternFunctionCall(&'g ExternFunctionCallGE<'s, 't, 'g>),
  FunctionCall(&'g FunctionCallGE<'s, 't, 'g>),
  BoundFunctionCall(&'g BoundFunctionCallGE<'s, 't, 'g>),
  Reinterpret(&'g ReinterpretGE<'s, 't, 'g>),
  Construct(&'g ConstructGE<'s, 't, 'g>),
  NewRuntimeSizedArray(&'g NewRuntimeSizedArrayGE<'s, 't, 'g>),
  StaticArrayFromCallable(&'g StaticArrayFromCallableGE<'s, 't, 'g>),
  DestroyStaticSizedArrayIntoFunction(&'g DestroyStaticSizedArrayIntoFunctionGE<'s, 't, 'g>),
  DestroyStaticSizedArrayIntoLocals(&'g DestroyStaticSizedArrayIntoLocalsGE<'s, 't, 'g>),
  DestroyRuntimeSizedArray(&'g DestroyRuntimeSizedArrayGE<'s, 't, 'g>),
  RuntimeSizedArrayCapacity(&'g RuntimeSizedArrayCapacityGE<'s, 't, 'g>),
  PushRuntimeSizedArray(&'g PushRuntimeSizedArrayGE<'s, 't, 'g>),
  PopRuntimeSizedArray(&'g PopRuntimeSizedArrayGE<'s, 't, 'g>),
  InterfaceToInterfaceUpcast(&'g InterfaceToInterfaceUpcastGE<'s, 't, 'g>),
  UpcastInterface(&'g UpcastInterfaceGE<'s, 't, 'g>),
  UpcastGeneric(&'g UpcastGenericGE<'s, 't, 'g>),
  Destroy(&'g DestroyGE<'s, 't, 'g>),
  CopyPrim(&'g CopyPrimGE<'s, 't, 'g>),
  LocalLookup(&'g LocalLookupGE<'s, 't, 'g>),
  StaticSizedArrayLookup(&'g StaticSizedArrayLookupGE<'s, 't, 'g>),
  RuntimeSizedArrayLookup(&'g RuntimeSizedArrayLookupGE<'s, 't, 'g>),
  MemberLookup(&'g MemberLookupGE<'s, 't, 'g>),
  Deref(&'g DerefGE<'s, 't, 'g>),
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LetAndLendGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub variable: &'g LocalVariableG<'s, 't, 'g>,
  pub expr: ExpressionGE<'s, 't, 'g>,
  // Stored instead of computed because I dont want getters to allocate.
  pub result: &'g BorrowRefGT<'s, 't, 'g>,
  // Always produces a borrow reference, though i can see a world where we go back on that decision.

  // VCOORD: _sealed here
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LockWeakGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub inner_expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
  pub some_constructor: &'t PrototypeT<'s, 't>,
  pub none_constructor: &'t PrototypeT<'s, 't>,
  pub some_impl_name: IdT<'s, 't>,
  pub none_impl_name: IdT<'s, 't>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BorrowToWeakGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub inner_expr: ExpressionGE<'s, 't, 'g>,
  pub result: &'g WeakRefGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LetNormalGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub variable: &'g LocalVariableG<'s, 't, 'g>,
  pub expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct UnletGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  pub variable: &'g LocalVariableG<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DiscardGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct IfGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub loct: LocT<'t>,
  pub condition: ExpressionGE<'s, 't, 'g>,
  pub then_call: ExpressionGE<'s, 't, 'g>,
  pub else_call: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct WhileGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub loct: LocT<'t>,
  pub block: BlockGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct MutateGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub loct: LocT<'t>,
  pub destination_expr: ExpressionGE<'s, 't, 'g>,
  pub source_expr: ExpressionGE<'s, 't, 'g>,
  // VCOORD: the old value that was replaced; onion old-value semantics to confirm.
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RestackifyGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub variable: &'g LocalVariableG<'s, 't, 'g>,
  pub source_expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReturnGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub source_expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BreakGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BlockGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub inner: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConsecutorGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub exprs: &'g [ExpressionGE<'s, 't, 'g>],
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticArrayFromValuesGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub elements: &'g [ExpressionGE<'s, 't, 'g>],
  pub result: KindGT<'s, 't, 'g>,
  pub array_type: &'g StaticSizedArrayGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArraySizeGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct IsSameInstanceGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub left: ExpressionGE<'s, 't, 'g>,
  pub right: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct AsSubtypeGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub source_expr: ExpressionGE<'s, 't, 'g>,
  pub target_type: KindGT<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
  pub ok_constructor: &'t PrototypeT<'s, 't>,
  pub err_constructor: &'t PrototypeT<'s, 't>,
  pub impl_name: IdT<'s, 't>,
  pub ok_impl_name: IdT<'s, 't>,
  pub err_impl_name: IdT<'s, 't>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct VoidLiteralGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantIntGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  pub value: ITemplataG<'s, 't>,
  pub bits: i32,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantBoolGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  pub value: bool,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantStrGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  pub value: StrI<'s>,
  // Str is share-flavored, so a string literal is a share reference.
  pub result: &'g ShareRefGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantFloatGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  pub value: f64,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LocalLookupGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  pub local_variable: &'g LocalVariableG<'s, 't, 'g>,
  // A local lookup is a borrow reference to the variable's value.
  pub result: &'g BorrowRefGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArgLookupGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  pub param_index: i32,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticSizedArrayLookupGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_expr: ExpressionGE<'s, 't, 'g>,
  pub array_type: &'g StaticSizedArrayGT<'s, 't, 'g>,
  pub index_expr: ExpressionGE<'s, 't, 'g>,
  // A borrow reference to the indexed element.
  pub result: &'g BorrowRefGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RuntimeSizedArrayLookupGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_expr: ExpressionGE<'s, 't, 'g>,
  pub array_type: &'g RuntimeSizedArrayGT<'s, 't, 'g>,
  pub index_expr: ExpressionGE<'s, 't, 'g>,
  // See RMLRMO why the result is a borrow reference to the element type.
  pub result: &'g BorrowRefGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArrayLengthGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct MemberLookupGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub struct_expr: ExpressionGE<'s, 't, 'g>,
  pub member_name: IVarNameT<'s, 't>,
  // See RMLRMO why the result is a borrow reference to the member.
  pub result: &'g BorrowRefGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DerefGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub loct: LocT<'t>,
  pub inner: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct InterfaceFunctionCallGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub super_function_prototype: &'t PrototypeT<'s, 't>,
  pub virtual_param_index: i32,
  pub result: KindGT<'s, 't, 'g>,
  pub args: &'g [ExpressionGE<'s, 't, 'g>],
  pub mut_effects: &'g [&'g MutEffectPath<'s, 't, 'g>],
}

/// Arena-allocated (see @TFITCX)
/// A method call on placeholder, using the interface we know it implements.
#[derive(Debug)]
pub struct BoundFunctionCallGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub impl_name: IdT<'s, 't>,
  pub abstract_prototype: &'t PrototypeT<'s, 't>,
  pub virtual_param_index: usize,
  pub result: KindGT<'s, 't, 'g>,
  pub args: &'g [ExpressionGE<'s, 't, 'g>],
  pub mut_effects: &'g [&'g MutEffectPath<'s, 't, 'g>],
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericParametersInheritance {
  pub num_inherited_generic_parameters: i32,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ExternFunctionCallGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub prototype2: &'t PrototypeT<'s, 't>,
  pub args: &'g [ExpressionGE<'s, 't, 'g>],
  pub result: KindGT<'s, 't, 'g>,
  pub mut_effects: &'g [&'g MutEffectPath<'s, 't, 'g>],
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct FunctionCallGE<'s, 't, 'g>
where
    's: 't,
{
  pub loct: LocT<'t>,
  /// The call's source range(s), for diagnostics that point at the call itself (e.g. the borrow
  /// checker locating a held-register use whose reference is this call's unnamed result).
  pub range: &'t [RangeS<'s>],
  pub callable: &'t PrototypeT<'s, 't>,
  pub args: &'g [ExpressionGE<'s, 't, 'g>],
  // VCOORD: rename to return_type
  pub result: KindGT<'s, 't, 'g>,
  pub mut_effects: &'g [&'g MutEffectPath<'s, 't, 'g>],
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReinterpretGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct CopyPrimGE<'s, 't, 'g> {
  pub range: RangeS<'s>,
  /// This load's location, so the borrow checker can name it as a restrict-region access site.
  pub loct: LocT<'t>,
  pub inner: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstructGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub struct_tt: &'g StructGT<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
  pub args: &'g [ExpressionGE<'s, 't, 'g>],
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct NewRuntimeSizedArrayGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_type: &'g RuntimeSizedArrayGT<'s, 't, 'g>,
  pub capacity_expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticArrayFromCallableGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_type: &'g StaticSizedArrayGT<'s, 't, 'g>,
  pub generator: ExpressionGE<'s, 't, 'g>,
  pub generator_method: &'t PrototypeT<'s, 't>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyStaticSizedArrayIntoFunctionGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_expr: ExpressionGE<'s, 't, 'g>,
  pub array_type: &'g StaticSizedArrayGT<'s, 't, 'g>,
  pub consumer: ExpressionGE<'s, 't, 'g>,
  pub consumer_method: &'t PrototypeT<'s, 't>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub expr: ExpressionGE<'s, 't, 'g>,
  pub static_sized_array: &'g StaticSizedArrayGT<'s, 't, 'g>,
  pub destination_reference_variables: &'g [&'g LocalVariableG<'s, 't, 'g>],
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyRuntimeSizedArrayGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RuntimeSizedArrayCapacityGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct PushRuntimeSizedArrayGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_expr: ExpressionGE<'s, 't, 'g>,
  pub new_element_expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct PopRuntimeSizedArrayGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub array_expr: ExpressionGE<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct InterfaceToInterfaceUpcastGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub inner_expr: ExpressionGE<'s, 't, 'g>,
  pub target_interface: &'g InterfaceGT<'s, 't, 'g>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct UpcastInterfaceGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub inner_expr: ExpressionGE<'s, 't, 'g>,
  pub target_super_kind: ISuperKindGT<'s, 't, 'g>,
  pub impl_name: IdT<'s, 't>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
/// An upcast of a placeholder to one of the interfaces that it implements.
/// The instantiator should make this evaporate.
#[derive(Debug)]
pub struct UpcastGenericGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub inner_expr: ExpressionGE<'s, 't, 'g>,
  pub target_super_kind: ISuperKindGT<'s, 't, 'g>,
  pub impl_name: IdT<'s, 't>,
  pub result: KindGT<'s, 't, 'g>,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyGE<'s, 't, 'g>
where
    's: 't,
{
  pub range: RangeS<'s>,
  pub expr: ExpressionGE<'s, 't, 'g>,
  pub struct_tt: &'g StructGT<'s, 't, 'g>,
  pub destination_reference_variables: &'g [&'g LocalVariableG<'s, 't, 'g>],
  pub result: KindGT<'s, 't, 'g>,
}


impl<'s, 't, 'g> ExpressionGE<'s, 't, 'g>
where
    's: 't,
{
  pub fn result(&self) -> KindGT<'s, 't, 'g> {
    match self {
      ExpressionGE::LetAndLend(e) => KindGT::BorrowRef(e.result),
      ExpressionGE::LockWeak(e) => e.result,
      ExpressionGE::BorrowToWeak(e) => KindGT::WeakRef(e.result),
      ExpressionGE::LetNormal(e) => e.result,
      ExpressionGE::Unlet(e) => e.result,
      ExpressionGE::Discard(e) => e.result,
      ExpressionGE::If(e) => e.result,
      ExpressionGE::While(e) => e.result,
      ExpressionGE::Mutate(e) => e.result,
      ExpressionGE::Restackify(e) => e.result,
      ExpressionGE::Return(e) => e.result,
      ExpressionGE::Break(e) => e.result,
      ExpressionGE::Block(e) => e.result,
      ExpressionGE::Consecutor(e) => e.result,
      ExpressionGE::StaticArrayFromValues(e) => e.result,
      ExpressionGE::ArraySize(e) => e.result,
      ExpressionGE::IsSameInstance(e) => e.result,
      ExpressionGE::AsSubtype(e) => e.result,
      ExpressionGE::VoidLiteral(e) => e.result,
      ExpressionGE::ConstantInt(e) => e.result,
      ExpressionGE::ConstantBool(e) => e.result,
      ExpressionGE::ConstantStr(e) => KindGT::ShareRef(e.result),
      ExpressionGE::ConstantFloat(e) => e.result,
      ExpressionGE::ArgLookup(e) => e.result,
      ExpressionGE::ArrayLength(e) => e.result,
      ExpressionGE::InterfaceFunctionCall(e) => e.result,
      ExpressionGE::ExternFunctionCall(e) => e.result,
      ExpressionGE::FunctionCall(e) => e.result,
      ExpressionGE::BoundFunctionCall(e) => e.result,
      ExpressionGE::Reinterpret(e) => e.result,
      ExpressionGE::Construct(e) => e.result,
      ExpressionGE::NewRuntimeSizedArray(e) => e.result,
      ExpressionGE::StaticArrayFromCallable(e) => e.result,
      ExpressionGE::DestroyStaticSizedArrayIntoFunction(e) => e.result,
      ExpressionGE::DestroyStaticSizedArrayIntoLocals(e) => e.result,
      ExpressionGE::DestroyRuntimeSizedArray(e) => e.result,
      ExpressionGE::RuntimeSizedArrayCapacity(e) => e.result,
      ExpressionGE::PushRuntimeSizedArray(e) => e.result,
      ExpressionGE::PopRuntimeSizedArray(e) => e.result,
      ExpressionGE::InterfaceToInterfaceUpcast(e) => e.result,
      ExpressionGE::UpcastInterface(e) => e.result,
      ExpressionGE::UpcastGeneric(e) => e.result,
      ExpressionGE::Destroy(e) => e.result,
      ExpressionGE::CopyPrim(e) => e.result,
      ExpressionGE::LocalLookup(e) => KindGT::BorrowRef(e.result),
      ExpressionGE::StaticSizedArrayLookup(e) => KindGT::BorrowRef(e.result),
      ExpressionGE::RuntimeSizedArrayLookup(e) => KindGT::BorrowRef(e.result),
      ExpressionGE::MemberLookup(e) => KindGT::BorrowRef(e.result),
      ExpressionGE::Deref(e) => e.result,
    }
  }
}
