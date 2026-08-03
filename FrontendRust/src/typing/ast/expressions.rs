use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::ast::ast::*;
use crate::typing::types::types::{KindT, NeverT, VoidT};
use crate::typing::types::types::IntT;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::SharednessT;
use crate::typing::types::types::RegionT;
use crate::typing::types::types::BoolT;
use crate::typing::types::types::FloatT;
use crate::typing::typing_interner::TypingInterner;
use std::any::Any;
use std::marker::PhantomData;
use crate::typing::templata_compiler::{is_ref, peel_one_reference, replace_value_type_in_ref};

/// Arena-allocated (see @TFITCX)
//
// No `PartialEq`/`Hash` derive or impl — opts out of equality entirely. Getting
// a compile-time error on `==` is strictly stronger than a runtime panic.
//
// Per @TFITCX this is `Arena-allocated` (lifetime/storage in the typing arena), but
// per @IEOIBZ such types normally implement identity equality via `std::ptr::eq`.
// The expression hierarchy is the exception: it's stored in the arena for memory
// reasons (large, deeply nested trees with `&'t` child pointers) but has no
// identity semantics — two distinct allocations of `ConstantIntTE { value: 5 }`
// are neither `==` nor distinguishable by identity (no callers care).
#[derive(Copy, Clone, Debug)]
pub enum ExpressionTE<'s, 't> {
    LetAndLend(&'t LetAndLendTE<'s, 't>),
    LockWeak(&'t LockWeakTE<'s, 't>),
    BorrowToWeak(&'t BorrowToWeakTE<'s, 't>),
    LetNormal(&'t LetNormalTE<'s, 't>),
    Unlet(&'t UnletTE<'s, 't>),
    Discard(&'t DiscardTE<'s, 't>),
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
    CopyPrim(&'t CopyPrimTE<'s, 't>),
    LocalLookup(&'t LocalLookupTE<'s, 't>),
    StaticSizedArrayLookup(&'t StaticSizedArrayLookupTE<'s, 't>),
    RuntimeSizedArrayLookup(&'t RuntimeSizedArrayLookupTE<'s, 't>),
    ReferenceMemberLookup(&'t ReferenceMemberLookupTE<'s, 't>),
    AddressMemberLookup(&'t AddressMemberLookupTE<'s, 't>),
    Deref(&'t DerefTE<'s, 't>),
}

impl<'s, 't> ExpressionTE<'s, 't> where 's: 't {
    pub fn result(&self) -> KindT<'s, 't> {
        match self {
            ExpressionTE::LetAndLend(e) => KindT::BorrowRef(e.result),
            ExpressionTE::LockWeak(e) => e.result,
            ExpressionTE::BorrowToWeak(e) => KindT::WeakRef(e.result),
            ExpressionTE::LetNormal(e) => e.result,
            ExpressionTE::Unlet(e) => e.result,
            ExpressionTE::Discard(e) => e.result,
            ExpressionTE::Defer(e) => e.result,
            ExpressionTE::If(e) => e.result,
            ExpressionTE::While(e) => e.result,
            ExpressionTE::Mutate(e) => e.result,
            ExpressionTE::Restackify(e) => e.result,
            ExpressionTE::Return(e) => e.result,
            ExpressionTE::Break(e) => e.result,
            ExpressionTE::Block(e) => e.result,
            ExpressionTE::Consecutor(e) => e.result,
            ExpressionTE::Tuple(e) => e.result,
            ExpressionTE::StaticArrayFromValues(e) => e.result,
            ExpressionTE::ArraySize(e) => e.result,
            ExpressionTE::IsSameInstance(e) => e.result,
            ExpressionTE::AsSubtype(e) => e.result,
            ExpressionTE::VoidLiteral(e) => e.result,
            ExpressionTE::ConstantInt(e) => e.result,
            ExpressionTE::ConstantBool(e) => e.result,
            ExpressionTE::ConstantStr(e) => KindT::ShareRef(e.result),
            ExpressionTE::ConstantFloat(e) => e.result,
            ExpressionTE::ArgLookup(e) => e.result,
            ExpressionTE::ArrayLength(e) => e.result,
            ExpressionTE::InterfaceFunctionCall(e) => e.result,
            ExpressionTE::ExternFunctionCall(e) => e.result,
            ExpressionTE::FunctionCall(e) => e.result,
            ExpressionTE::Reinterpret(e) => e.result,
            ExpressionTE::Construct(e) => e.result,
            ExpressionTE::NewRuntimeSizedArray(e) => e.result,
            ExpressionTE::StaticArrayFromCallable(e) => e.result,
            ExpressionTE::DestroyStaticSizedArrayIntoFunction(e) => e.result,
            ExpressionTE::DestroyStaticSizedArrayIntoLocals(e) => e.result,
            ExpressionTE::DestroyRuntimeSizedArray(e) => e.result,
            ExpressionTE::RuntimeSizedArrayCapacity(e) => e.result,
            ExpressionTE::PushRuntimeSizedArray(e) => e.result,
            ExpressionTE::PopRuntimeSizedArray(e) => e.result,
            ExpressionTE::InterfaceToInterfaceUpcast(e) => e.result,
            ExpressionTE::Upcast(e) => e.result,
            ExpressionTE::Destroy(e) => e.result,
            ExpressionTE::CopyPrim(e) => e.result,
            ExpressionTE::LocalLookup(e) => KindT::BorrowRef(e.result),
            ExpressionTE::StaticSizedArrayLookup(e) => KindT::BorrowRef(e.result),
            ExpressionTE::RuntimeSizedArrayLookup(e) => KindT::BorrowRef(e.result),
            ExpressionTE::ReferenceMemberLookup(e) => KindT::BorrowRef(e.result),
            ExpressionTE::AddressMemberLookup(e) => KindT::BorrowRef(e.result),
            ExpressionTE::Deref(e) => e.result,
        }
    }
    
    pub fn kind(&self) -> KindT<'s, 't> {
        self.result()
    }
    
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LetAndLendTE<'s, 't>
where 's: 't,
{
    pub variable: LocalVariable<'s, 't>,
    pub expr: ExpressionTE<'s, 't>,
    // Stored instead of computed because I dont want getters to allocate.
    pub result: &'t BorrowRefT<'s, 't>,
    // Always produces a borrow reference, though i can see a world where we go back on that decision.

    // VCOORD: _sealed here
    _sealed: (),
}

impl<'s, 't> LetAndLendTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        variable: LocalVariable<'s, 't>,
        expr: ExpressionTE<'s, 't>,
    ) -> LetAndLendTE<'s, 't> {
        let result = interner.alloc(BorrowRefT { inner: expr.result(), region: RegionT::Default });
        LetAndLendTE { variable, expr, result, _sealed: () }
    }

    // VCOORD: get rid of result(), just inline it into the enum's dispatcher
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LockWeakTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    pub some_constructor: &'t PrototypeT<'s, 't>,
    pub none_constructor: &'t PrototypeT<'s, 't>,
    pub some_impl_name: IdT<'s, 't>,
    pub none_impl_name: IdT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> LockWeakTE<'s, 't> where 's: 't, {
    pub fn new(
        inner_expr: ExpressionTE<'s, 't>,
        result: KindT<'s, 't>,
        some_constructor: &'t PrototypeT<'s, 't>,
        none_constructor: &'t PrototypeT<'s, 't>,
        some_impl_name: IdT<'s, 't>,
        none_impl_name: IdT<'s, 't>,
    ) -> LockWeakTE<'s, 't> {
        LockWeakTE { inner_expr, result, some_constructor, none_constructor, some_impl_name, none_impl_name, _sealed: () }
    }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BorrowToWeakTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ExpressionTE<'s, 't>,
    pub result: &'t WeakRefT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> BorrowToWeakTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        inner_expr: ExpressionTE<'s, 't>,
    ) -> BorrowToWeakTE<'s, 't> {
        let result = interner.alloc(WeakRefT { inner: inner_expr.result() });
        BorrowToWeakTE { inner_expr, result, _sealed: () }
    }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LetNormalTE<'s, 't>
where 's: 't,
{
    pub variable: LocalVariable<'s, 't>,
    pub expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> LetNormalTE<'s, 't> where 's: 't, {
    pub fn new(
        variable: LocalVariable<'s, 't>,
        expr: ExpressionTE<'s, 't>,
    ) -> LetNormalTE<'s, 't> {
        LetNormalTE { variable, expr, result: KindT::Void(VoidT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct UnletTE<'s, 't> {
    pub variable: LocalVariable<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> UnletTE<'s, 't> where 's: 't, {
    pub fn new(variable: LocalVariable<'s, 't>) -> UnletTE<'s, 't> {
        let result = variable.tyype;
        UnletTE { variable, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DiscardTE<'s, 't>
where 's: 't,
{
    pub expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> DiscardTE<'s, 't> where 's: 't, {
    pub fn new(expr: ExpressionTE<'s, 't>) -> DiscardTE<'s, 't> {
        DiscardTE { expr, result: KindT::Void(VoidT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DeferTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ExpressionTE<'s, 't>,
    pub deferred_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> DeferTE<'s, 't> where 's: 't, {
    pub fn new(
        inner_expr: ExpressionTE<'s, 't>,
        deferred_expr: ExpressionTE<'s, 't>,
    ) -> DeferTE<'s, 't> {
        assert!(deferred_expr.result() == KindT::Void(VoidT));
        let result = inner_expr.result();
        DeferTE { inner_expr, deferred_expr, result, _sealed: () }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct IfTE<'s, 't>
where 's: 't,
{
    pub condition: ExpressionTE<'s, 't>,
    pub then_call: ExpressionTE<'s, 't>,
    pub else_call: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> IfTE<'s, 't> where 's: 't, {
    pub fn new(
        condition: ExpressionTE<'s, 't>,
        then_call: ExpressionTE<'s, 't>,
        else_call: ExpressionTE<'s, 't>,
    ) -> IfTE<'s, 't> {
        match condition.result() {
            KindT::Bool(_) => {}
            other => panic!("vfail: {:?}", other),
        }
        let then_result = then_call.result();
        let else_result = else_call.result();
        match (then_result, else_result) {
            (KindT::Never(_), _) => {}
            (_, KindT::Never(_)) => {}
            (a, b) if a == b => {}
            _ => panic!("vwat"),
        }
        let result = match then_result {
            KindT::Never(_) => else_result,
            _ => then_result,
        };
        IfTE { condition, then_call, else_call, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct WhileTE<'s, 't>
where 's: 't,
{
    pub block: BlockTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> WhileTE<'s, 't> where 's: 't, {
    pub fn new(block: BlockTE<'s, 't>) -> WhileTE<'s, 't> {
        let result = match block.result {
            KindT::Void(_) => block.result,
            KindT::Never(NeverT { from_break: true }) => KindT::Void(VoidT),
            KindT::Never(NeverT { from_break: false }) => block.result,
            _ => panic!("vwat"),
        };
        WhileTE { block, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct MutateTE<'s, 't>
where 's: 't,
{
    pub destination_expr: ExpressionTE<'s, 't>,
    pub source_expr: ExpressionTE<'s, 't>,
    // VCOORD: the old value that was replaced; onion old-value semantics to confirm.
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> MutateTE<'s, 't> where 's: 't, {
    pub fn new(
        destination_expr: ExpressionTE<'s, 't>,
        source_expr: ExpressionTE<'s, 't>,
    ) -> MutateTE<'s, 't> {
        let destination_inner_type =
            match destination_expr.result() {
                KindT::BorrowRef(BorrowRefT { inner: destination_inner_type, region: _ }) => {
                    *destination_inner_type
                }
                _ => panic!("Unexpected destination expr type in MutateTE::new: {:?} in expr: {:?}", destination_expr.result(), destination_expr)
            };
        assert_eq!(destination_inner_type, source_expr.result());
        let result = destination_inner_type;
        MutateTE { destination_expr, source_expr, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RestackifyTE<'s, 't>
where 's: 't,
{
    pub variable: LocalVariable<'s, 't>,
    pub source_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> RestackifyTE<'s, 't> where 's: 't, {
    pub fn new(
        variable: LocalVariable<'s, 't>,
        source_expr: ExpressionTE<'s, 't>,
    ) -> RestackifyTE<'s, 't> {
        RestackifyTE { variable, source_expr, result: KindT::Void(VoidT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReturnTE<'s, 't>
where 's: 't,
{
    pub source_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ReturnTE<'s, 't> where 's: 't, {
    pub fn new(source_expr: ExpressionTE<'s, 't>) -> ReturnTE<'s, 't> {
        ReturnTE { source_expr, result: KindT::Never(NeverT { from_break: false }), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BreakTE<'s, 't> {
    pub region: RegionT,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> BreakTE<'s, 't> where 's: 't, {
    pub fn new(region: RegionT) -> BreakTE<'s, 't> {
        BreakTE { region, result: KindT::Never(NeverT { from_break: true }), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BlockTE<'s, 't>
where 's: 't,
{
    pub inner: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> BlockTE<'s, 't> where 's: 't, {
    pub fn new(inner: ExpressionTE<'s, 't>) -> BlockTE<'s, 't> {
        let result = inner.result();
        BlockTE { inner, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConsecutorTE<'s, 't>
where 's: 't,
{
    pub exprs: &'t [ExpressionTE<'s, 't>],
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ConsecutorTE<'s, 't> where 's: 't, {
    pub fn new(exprs: &'t [ExpressionTE<'s, 't>]) -> ConsecutorTE<'s, 't> {
        // A `Never` anywhere makes the whole sequence `Never`; otherwise the last expr's result.
        let result = match exprs.iter().map(|e| e.result()).find(|c| matches!(c, KindT::Never(_))) {
            Some(n) => n,
            None => exprs.last().unwrap().result(),
        };
        ConsecutorTE { exprs, result, _sealed: () }
    }

    fn last_reference_expr(&self) -> &ExpressionTE<'s, 't> {
        panic!("Unimplemented: last_reference_expr");
        // exprs.last
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct TupleTE<'s, 't>
where 's: 't,
{
    pub elements: &'t [ExpressionTE<'s, 't>],
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> TupleTE<'s, 't> where 's: 't, {
    pub fn new(
        elements: &'t [ExpressionTE<'s, 't>],
        result: KindT<'s, 't>,
    ) -> TupleTE<'s, 't> {
        TupleTE { elements, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticArrayFromValuesTE<'s, 't>
where 's: 't,
{
    pub elements: &'t [ExpressionTE<'s, 't>],
    pub result: KindT<'s, 't>,
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> StaticArrayFromValuesTE<'s, 't> where 's: 't, {
    pub fn new(
        elements: &'t [ExpressionTE<'s, 't>],
        result: KindT<'s, 't>,
        array_type: &'t StaticSizedArrayTT<'s, 't>,
    ) -> StaticArrayFromValuesTE<'s, 't> {
        StaticArrayFromValuesTE { elements, result, array_type, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArraySizeTE<'s, 't>
where 's: 't,
{
    pub array: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ArraySizeTE<'s, 't> where 's: 't, {
    pub fn new(array: ExpressionTE<'s, 't>) -> ArraySizeTE<'s, 't> {
        ArraySizeTE { array, result: KindT::Int(IntT::I32), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct IsSameInstanceTE<'s, 't>
where 's: 't,
{
    pub left: ExpressionTE<'s, 't>,
    pub right: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> IsSameInstanceTE<'s, 't> where 's: 't, {
    pub fn new(left: ExpressionTE<'s, 't>, right: ExpressionTE<'s, 't>) -> IsSameInstanceTE<'s, 't> {
        IsSameInstanceTE { left, right, result: KindT::Bool(BoolT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct AsSubtypeTE<'s, 't>
where 's: 't,
{
    pub source_expr: ExpressionTE<'s, 't>,
    pub target_type: KindT<'s, 't>,
    pub result: KindT<'s, 't>,
    pub ok_constructor: &'t PrototypeT<'s, 't>,
    pub err_constructor: &'t PrototypeT<'s, 't>,
    pub impl_name: IdT<'s, 't>,
    pub ok_impl_name: IdT<'s, 't>,
    pub err_impl_name: IdT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> AsSubtypeTE<'s, 't> where 's: 't, {
    pub fn new(
        source_expr: ExpressionTE<'s, 't>,
        target_type: KindT<'s, 't>,
        result: KindT<'s, 't>,
        ok_constructor: &'t PrototypeT<'s, 't>,
        err_constructor: &'t PrototypeT<'s, 't>,
        impl_name: IdT<'s, 't>,
        ok_impl_name: IdT<'s, 't>,
        err_impl_name: IdT<'s, 't>,
    ) -> AsSubtypeTE<'s, 't> {
        AsSubtypeTE { source_expr, target_type, result, ok_constructor, err_constructor, impl_name, ok_impl_name, err_impl_name, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct VoidLiteralTE<'s, 't> {
    pub region: RegionT,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> VoidLiteralTE<'s, 't> where 's: 't, {
    pub fn new(region: RegionT) -> VoidLiteralTE<'s, 't> {
        VoidLiteralTE { region, result: KindT::Void(VoidT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantIntTE<'s, 't> {
    pub value: ITemplataT<'s, 't>,
    pub bits: i32,
    pub region: RegionT,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ConstantIntTE<'s, 't> where 's: 't, {
    pub fn new(
        value: ITemplataT<'s, 't>,
        bits: i32,
        region: RegionT,
    ) -> ConstantIntTE<'s, 't> {
        ConstantIntTE { value, bits, region, result: KindT::Int(IntT { bits }), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantBoolTE<'s, 't> {
    pub value: bool,
    pub region: RegionT,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ConstantBoolTE<'s, 't> where 's: 't, {
    pub fn new(value: bool, region: RegionT) -> ConstantBoolTE<'s, 't> {
        ConstantBoolTE { value, region, result: KindT::Bool(BoolT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantStrTE<'s, 't> {
    pub value: StrI<'s>,
    pub region: RegionT,
    // Str is share-flavored, so a string literal is a share reference.
    pub result: &'t ShareRefT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ConstantStrTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        value: StrI<'s>,
        region: RegionT,
    ) -> ConstantStrTE<'s, 't> {
        let result = interner.alloc(ShareRefT { inner: KindT::Str(StrT) });
        ConstantStrTE { value, region, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantFloatTE<'s, 't> {
    pub value: f64,
    pub region: RegionT,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ConstantFloatTE<'s, 't> where 's: 't, {
    pub fn new(value: f64, region: RegionT) -> ConstantFloatTE<'s, 't> {
        ConstantFloatTE { value, region, result: KindT::Float(FloatT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LocalLookupTE<'s, 't> {
    pub range: RangeS<'s>,
    pub local_variable: LocalVariable<'s, 't>,
    // A local lookup is a borrow reference to the variable's value.
    pub result: &'t BorrowRefT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> LocalLookupTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        range: RangeS<'s>,
        local_variable: LocalVariable<'s, 't>,
    ) -> LocalLookupTE<'s, 't> {
        let result = interner.alloc(BorrowRefT { inner: local_variable.tyype, region: RegionT::Default });
        LocalLookupTE { range, local_variable, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArgLookupTE<'s, 't> {
    pub param_index: i32,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ArgLookupTE<'s, 't> where 's: 't, {
    pub fn new(param_index: i32, result: KindT<'s, 't>) -> ArgLookupTE<'s, 't> {
        ArgLookupTE { param_index, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticSizedArrayLookupTE<'s, 't>
where 's: 't,
{
    pub range: RangeS<'s>,
    pub array_expr: ExpressionTE<'s, 't>,
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
    pub index_expr: ExpressionTE<'s, 't>,
    // A borrow reference to the indexed element.
    pub result: &'t BorrowRefT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> StaticSizedArrayLookupTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        range: RangeS<'s>,
        array_expr: ExpressionTE<'s, 't>,
        array_type: &'t StaticSizedArrayTT<'s, 't>,
        index_expr: ExpressionTE<'s, 't>,
    ) -> StaticSizedArrayLookupTE<'s, 't> {
        let result = interner.alloc(BorrowRefT { inner: array_type.element_type(), region: RegionT::Default });
        StaticSizedArrayLookupTE { range, array_expr, array_type, index_expr, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RuntimeSizedArrayLookupTE<'s, 't>
where 's: 't,
{
    pub range: RangeS<'s>,
    pub array_expr: ExpressionTE<'s, 't>,
    pub array_type: &'t RuntimeSizedArrayTT<'s, 't>,
    pub index_expr: ExpressionTE<'s, 't>,
    // See RMLRMO why the result is a borrow reference to the element type.
    pub result: &'t BorrowRefT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        range: RangeS<'s>,
        array_expr: ExpressionTE<'s, 't>,
        array_type: &'t RuntimeSizedArrayTT<'s, 't>,
        index_expr: ExpressionTE<'s, 't>,
    ) -> RuntimeSizedArrayLookupTE<'s, 't> {
        let result = interner.alloc(BorrowRefT { inner: array_type.element_type(), region: RegionT::Default });
        RuntimeSizedArrayLookupTE { range, array_expr, array_type, index_expr, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArrayLengthTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ArrayLengthTE<'s, 't> where 's: 't, {
    pub fn new(array_expr: ExpressionTE<'s, 't>) -> ArrayLengthTE<'s, 't> {
        ArrayLengthTE { array_expr, result: KindT::Int(IntT::I32), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReferenceMemberLookupTE<'s, 't>
where 's: 't,
{
    pub range: RangeS<'s>,
    pub struct_expr: ExpressionTE<'s, 't>,
    pub member_name: IVarNameT<'s, 't>,
    // See RMLRMO why the result is a borrow reference to the member.
    pub result: &'t BorrowRefT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ReferenceMemberLookupTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        range: RangeS<'s>,
        struct_expr: ExpressionTE<'s, 't>,
        member_name: IVarNameT<'s, 't>,
        member_kind: KindT<'s, 't>,
    ) -> ReferenceMemberLookupTE<'s, 't> {
        let result = interner.alloc(BorrowRefT { inner: member_kind, region: RegionT::Default });
        ReferenceMemberLookupTE { range, struct_expr, member_name, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct AddressMemberLookupTE<'s, 't>
where 's: 't,
{
    pub range: RangeS<'s>,
    pub struct_expr: ExpressionTE<'s, 't>,
    pub member_name: IVarNameT<'s, 't>,
    pub result: &'t BorrowRefT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> AddressMemberLookupTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        range: RangeS<'s>,
        struct_expr: ExpressionTE<'s, 't>,
        member_name: IVarNameT<'s, 't>,
        member_kind: KindT<'s, 't>,
    ) -> AddressMemberLookupTE<'s, 't> {
        let result = interner.alloc(BorrowRefT { inner: member_kind, region: RegionT::Default });
        AddressMemberLookupTE { range, struct_expr, member_name, result, _sealed: () }
    }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DerefTE<'s, 't>
where 's: 't,
{
    pub range: RangeS<'s>,
    pub inner: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> DerefTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        range: RangeS<'s>,
        inner: ExpressionTE<'s, 't>,
    ) -> DerefTE<'s, 't> {
        let result =
            if let Some(result) = peel_one_reference(&inner.result()) {
                result
            } else {
                panic!("DerefTE inner isnt a reference");
            };
        DerefTE { range, inner, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct InterfaceFunctionCallTE<'s, 't>
where 's: 't,
{
    pub super_function_prototype: &'t PrototypeT<'s, 't>,
    pub virtual_param_index: i32,
    pub result: KindT<'s, 't>,
    pub args: &'t [ExpressionTE<'s, 't>],
    _sealed: (),
}

impl<'s, 't> InterfaceFunctionCallTE<'s, 't> where 's: 't, {
    pub fn new(
        super_function_prototype: &'t PrototypeT<'s, 't>,
        virtual_param_index: i32,
        result: KindT<'s, 't>,
        args: &'t [ExpressionTE<'s, 't>],
    ) -> InterfaceFunctionCallTE<'s, 't> {
        InterfaceFunctionCallTE { super_function_prototype, virtual_param_index, result, args, _sealed: () }
    }
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericParametersInheritance {
  pub num_inherited_generic_parameters: i32,
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ExternFunctionCallTE<'s, 't>
where 's: 't,
{
    pub prototype2: &'t PrototypeT<'s, 't>,
    pub args: &'t [ExpressionTE<'s, 't>],
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ExternFunctionCallTE<'s, 't> where 's: 't, {
    pub fn new(
        prototype2: &'t PrototypeT<'s, 't>,
        args: &'t [ExpressionTE<'s, 't>],
    ) -> ExternFunctionCallTE<'s, 't> {
        let result = prototype2.return_type;
        ExternFunctionCallTE { prototype2, args, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct FunctionCallTE<'s, 't>
where 's: 't,
{
    pub callable: &'t PrototypeT<'s, 't>,
    pub args: &'t [ExpressionTE<'s, 't>],
    // VCOORD: rename to return_type
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> FunctionCallTE<'s, 't> where 's: 't, {
    pub fn new(
        callable: &'t PrototypeT<'s, 't>,
        args: &'t [ExpressionTE<'s, 't>],
        result: KindT<'s, 't>,
    ) -> FunctionCallTE<'s, 't> {
        FunctionCallTE { callable, args, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReinterpretTE<'s, 't>
where 's: 't,
{
    pub expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> ReinterpretTE<'s, 't> where 's: 't, {
    pub fn new(expr: ExpressionTE<'s, 't>, result: KindT<'s, 't>) -> ReinterpretTE<'s, 't> {
        ReinterpretTE { expr, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct CopyPrimTE<'s, 't> {
    pub inner: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}
impl<'s, 't> CopyPrimTE<'s, 't> where 's: 't, {
    pub fn new(inner: ExpressionTE<'s, 't>, result: KindT<'s, 't>) -> CopyPrimTE<'s, 't> {
        CopyPrimTE { inner, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstructTE<'s, 't>
where 's: 't,
{
    pub struct_tt: &'t StructTT<'s, 't>,
    pub result: KindT<'s, 't>,
    pub args: &'t [ExpressionTE<'s, 't>],
    _sealed: (),
}

impl<'s, 't> ConstructTE<'s, 't> where 's: 't, {
    pub fn new(
        struct_tt: &'t StructTT<'s, 't>,
        result: KindT<'s, 't>,
        args: &'t [ExpressionTE<'s, 't>],
    ) -> ConstructTE<'s, 't> {
        ConstructTE { struct_tt, result, args, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct NewRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_type: &'t RuntimeSizedArrayTT<'s, 't>,
    pub region: RegionT,
    pub capacity_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> NewRuntimeSizedArrayTE<'s, 't> where 's: 't, {
    pub fn new(
        array_type: &'t RuntimeSizedArrayTT<'s, 't>,
        region: RegionT,
        capacity_expr: ExpressionTE<'s, 't>,
    ) -> NewRuntimeSizedArrayTE<'s, 't> {
        let result = KindT::RuntimeSizedArray(array_type);
        NewRuntimeSizedArrayTE { array_type, region, capacity_expr, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticArrayFromCallableTE<'s, 't>
where 's: 't,
{
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
    pub region: RegionT,
    pub generator: ExpressionTE<'s, 't>,
    pub generator_method: &'t PrototypeT<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> StaticArrayFromCallableTE<'s, 't> where 's: 't, {
    pub fn new(
        array_type: &'t StaticSizedArrayTT<'s, 't>,
        region: RegionT,
        generator: ExpressionTE<'s, 't>,
        generator_method: &'t PrototypeT<'s, 't>,
    ) -> StaticArrayFromCallableTE<'s, 't> {
        let result = KindT::StaticSizedArray(array_type);
        StaticArrayFromCallableTE { array_type, region, generator, generator_method, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyStaticSizedArrayIntoFunctionTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
    pub consumer: ExpressionTE<'s, 't>,
    pub consumer_method: &'t PrototypeT<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> where 's: 't, {
    pub fn new(
        array_expr: ExpressionTE<'s, 't>,
        array_type: &'t StaticSizedArrayTT<'s, 't>,
        consumer: ExpressionTE<'s, 't>,
        consumer_method: &'t PrototypeT<'s, 't>,
    ) -> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> {
        DestroyStaticSizedArrayIntoFunctionTE { array_expr, array_type, consumer, consumer_method, result: KindT::Void(VoidT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsTE<'s, 't>
where 's: 't,
{
    pub expr: ExpressionTE<'s, 't>,
    pub static_sized_array: &'t StaticSizedArrayTT<'s, 't>,
    pub destination_reference_variables: &'t [LocalVariable<'s, 't>],
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> where 's: 't, {
    pub fn new(
        expr: ExpressionTE<'s, 't>,
        static_sized_array: &'t StaticSizedArrayTT<'s, 't>,
        destination_reference_variables: &'t [LocalVariable<'s, 't>],
    ) -> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> {
        DestroyStaticSizedArrayIntoLocalsTE { expr, static_sized_array, destination_reference_variables, result: KindT::Void(VoidT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> DestroyRuntimeSizedArrayTE<'s, 't> where 's: 't, {
    pub fn new(array_expr: ExpressionTE<'s, 't>) -> DestroyRuntimeSizedArrayTE<'s, 't> {
        DestroyRuntimeSizedArrayTE { array_expr, result: KindT::Void(VoidT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RuntimeSizedArrayCapacityTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> RuntimeSizedArrayCapacityTE<'s, 't> where 's: 't, {
    pub fn new(array_expr: ExpressionTE<'s, 't>) -> RuntimeSizedArrayCapacityTE<'s, 't> {
        RuntimeSizedArrayCapacityTE { array_expr, result: KindT::Int(IntT::I32), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct PushRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
    pub new_element_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> PushRuntimeSizedArrayTE<'s, 't> where 's: 't, {
    pub fn new(
        array_expr: ExpressionTE<'s, 't>,
        new_element_expr: ExpressionTE<'s, 't>,
    ) -> PushRuntimeSizedArrayTE<'s, 't> {
        PushRuntimeSizedArrayTE { array_expr, new_element_expr, result: KindT::Void(VoidT), _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct PopRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> PopRuntimeSizedArrayTE<'s, 't> where 's: 't, {
    pub fn new(
        array_expr: ExpressionTE<'s, 't>,
        result: KindT<'s, 't>,
    ) -> PopRuntimeSizedArrayTE<'s, 't> {
        PopRuntimeSizedArrayTE { array_expr, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct InterfaceToInterfaceUpcastTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ExpressionTE<'s, 't>,
    pub target_interface: &'t InterfaceTT<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> where 's: 't, {
    pub fn new(
        inner_expr: ExpressionTE<'s, 't>,
        target_interface: &'t InterfaceTT<'s, 't>,
    ) -> InterfaceToInterfaceUpcastTE<'s, 't> {
        // VCOORD: preserve the inner wrap and swap the innermost citizen to target_interface.
        unimplemented!("InterfaceToInterfaceUpcastTE onion result")
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct UpcastTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ExpressionTE<'s, 't>,
    pub target_super_kind: ISuperKindTT<'s, 't>,
    pub impl_name: IdT<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> UpcastTE<'s, 't> where 's: 't, {
    pub fn new(
        interner: &TypingInterner<'s, 't>,
        inner_expr: ExpressionTE<'s, 't>,
        target_super_kind: ISuperKindTT<'s, 't>,
        impl_name: IdT<'s, 't>,
    ) -> UpcastTE<'s, 't> {
        let result =
            replace_value_type_in_ref(interner, inner_expr.result(), target_super_kind.into());
        UpcastTE { inner_expr, target_super_kind, impl_name, result, _sealed: () }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyTE<'s, 't>
where 's: 't,
{
    pub expr: ExpressionTE<'s, 't>,
    pub struct_tt: &'t StructTT<'s, 't>,
    pub destination_reference_variables: &'t [LocalVariable<'s, 't>],
    pub result: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> DestroyTE<'s, 't> where 's: 't, {
    pub fn new(
        expr: ExpressionTE<'s, 't>,
        struct_tt: &'t StructTT<'s, 't>,
        destination_reference_variables: &'t [LocalVariable<'s, 't>],
    ) -> DestroyTE<'s, 't> {
        DestroyTE { expr, struct_tt, destination_reference_variables, result: KindT::Void(VoidT), _sealed: () }
    }
}
fn reference_expr_result_struct_name_unapply<'s, 't>(expr: &ExpressionTE<'s, 't>) -> Option<StrI<'s>> {
    panic!("Unimplemented: unapply");
    // expr.result.kind match {
    //   case StructTT(IdT(_, _, StructNameT(StructTemplateNameT(name), _))) => Some(name)
    //   case _ => None
    // }
}

fn reference_expr_result_kind_unapply<'s, 't>(expr: &ExpressionTE<'s, 't>) -> Option<KindT<'s, 't>> {
    panic!("Unimplemented: unapply");
    // Some(expr.result.kind)
}
