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
use std::any::Any;
use std::marker::PhantomData;


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
    Break(&'t BreakTE),
    Block(&'t BlockTE<'s, 't>),
    Consecutor(&'t ConsecutorTE<'s, 't>),
    Tuple(&'t TupleTE<'s, 't>),
    StaticArrayFromValues(&'t StaticArrayFromValuesTE<'s, 't>),
    ArraySize(&'t ArraySizeTE<'s, 't>),
    IsSameInstance(&'t IsSameInstanceTE<'s, 't>),
    AsSubtype(&'t AsSubtypeTE<'s, 't>),
    VoidLiteral(&'t VoidLiteralTE),
    ConstantInt(&'t ConstantIntTE<'s, 't>),
    ConstantBool(&'t ConstantBoolTE),
    ConstantStr(&'t ConstantStrTE<'s>),
    ConstantFloat(&'t ConstantFloatTE),
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
    SoftLoad(&'t SoftLoadTE<'s, 't>),
    Destroy(&'t DestroyTE<'s, 't>),
    CopyPrim(&'t CopyPrimTE<'s, 't>),
    LocalLookup(&'t LocalLookupTE<'s, 't>),
    StaticSizedArrayLookup(&'t StaticSizedArrayLookupTE<'s, 't>),
    RuntimeSizedArrayLookup(&'t RuntimeSizedArrayLookupTE<'s, 't>),
    ReferenceMemberLookup(&'t ReferenceMemberLookupTE<'s, 't>),
    AddressMemberLookup(&'t AddressMemberLookupTE<'s, 't>),
}

impl<'s, 't> ExpressionTE<'s, 't> where 's: 't {
    pub fn result(&self) -> KindT<'s, 't> {
        match self {
            ExpressionTE::LetAndLend(e) => e.result(),
            ExpressionTE::LockWeak(e) => e.result(),
            ExpressionTE::BorrowToWeak(e) => e.result(),
            ExpressionTE::LetNormal(e) => e.result(),
            ExpressionTE::Unlet(e) => e.result(),
            ExpressionTE::Discard(e) => e.result(),
            ExpressionTE::Defer(e) => e.result(),
            ExpressionTE::If(e) => e.result(),
            ExpressionTE::While(e) => e.result(),
            ExpressionTE::Mutate(e) => e.result(),
            ExpressionTE::Restackify(e) => e.result(),
            ExpressionTE::Return(e) => e.result(),
            ExpressionTE::Break(e) => e.result(),
            ExpressionTE::Block(e) => e.result(),
            ExpressionTE::Consecutor(e) => e.result(),
            ExpressionTE::Tuple(e) => e.result(),
            ExpressionTE::StaticArrayFromValues(e) => e.result(),
            ExpressionTE::ArraySize(e) => e.result(),
            ExpressionTE::IsSameInstance(e) => e.result(),
            ExpressionTE::AsSubtype(e) => e.result(),
            ExpressionTE::VoidLiteral(e) => e.result(),
            ExpressionTE::ConstantInt(e) => e.result(),
            ExpressionTE::ConstantBool(e) => e.result(),
            ExpressionTE::ConstantStr(e) => e.result(),
            ExpressionTE::ConstantFloat(e) => e.result(),
            ExpressionTE::ArgLookup(e) => e.result(),
            ExpressionTE::ArrayLength(e) => e.result(),
            ExpressionTE::InterfaceFunctionCall(e) => e.result(),
            ExpressionTE::ExternFunctionCall(e) => e.result(),
            ExpressionTE::FunctionCall(e) => e.result(),
            ExpressionTE::Reinterpret(e) => e.result(),
            ExpressionTE::Construct(e) => e.result(),
            ExpressionTE::NewRuntimeSizedArray(e) => e.result(),
            ExpressionTE::StaticArrayFromCallable(e) => e.result(),
            ExpressionTE::DestroyStaticSizedArrayIntoFunction(e) => e.result(),
            ExpressionTE::DestroyStaticSizedArrayIntoLocals(e) => e.result(),
            ExpressionTE::DestroyRuntimeSizedArray(e) => e.result(),
            ExpressionTE::RuntimeSizedArrayCapacity(e) => e.result(),
            ExpressionTE::PushRuntimeSizedArray(e) => e.result(),
            ExpressionTE::PopRuntimeSizedArray(e) => e.result(),
            ExpressionTE::InterfaceToInterfaceUpcast(e) => e.result(),
            ExpressionTE::Upcast(e) => e.result(),
            ExpressionTE::SoftLoad(e) => e.result(),
            ExpressionTE::Destroy(e) => e.result(),
            ExpressionTE::CopyPrim(e) => e.result(),
            ExpressionTE::LocalLookup(e) => e.result(),
            ExpressionTE::StaticSizedArrayLookup(e) => e.result(),
            ExpressionTE::RuntimeSizedArrayLookup(e) => e.result(),
            ExpressionTE::ReferenceMemberLookup(e) => e.result(),
            ExpressionTE::AddressMemberLookup(e) => e.result(),
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
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: ExpressionTE<'s, 't>,
    // Stored instead of computed because I dont want getters to allocate.
    pub result: &'t BorrowRefT<'s, 't>,
    // Always produces a borrow reference, though i can see a world where we go back on that decision.

    // VCOORD: _sealed here
}

impl<'s, 't> LetAndLendTE<'s, 't> where 's: 't, {
    fn new(
        variable: ILocalVariableT<'s, 't>,
        expr: ExpressionTE<'s, 't>,
        result: &'t BorrowRefT<'s, 't>
    ) -> LetAndLendTE<'s, 't> {
        assert!(result.inner == expr.result());
        LetAndLendTE { variable, expr, result }
    }

    // VCOORD: get rid of result(), just inline it into the enum's dispatcher
    pub fn result(&self) -> KindT<'s, 't> {
        KindT::BorrowRef(self.result)
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LockWeakTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ExpressionTE<'s, 't>,
    pub result_opt_borrow_type: KindT<'s, 't>,
    pub some_constructor: &'t PrototypeT<'s, 't>,
    pub none_constructor: &'t PrototypeT<'s, 't>,
    pub some_impl_name: IdT<'s, 't>,
    pub none_impl_name: IdT<'s, 't>,
}

impl<'s, 't> LockWeakTE<'s, 't> {
    fn result(&self) -> KindT<'s, 't> { KindT { coord: self.result_opt_borrow_type } }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BorrowToWeakTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ExpressionTE<'s, 't>,
    pub result: &'t WeakRefT<'s, 't>,
}

impl<'s, 't> BorrowToWeakTE<'s, 't> {
    fn new(
        inner_expr: ExpressionTE<'s, 't>,
        result: &'t WeakRefT<'s, 't>
    ) -> BorrowToWeakTE<'s, 't> {
        assert_eq!(result.inner, inner_expr.result());
        BorrowToWeakTE { inner_expr, result }
    }

    fn result(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(OwnershipT::Weak, self.inner_expr.result().region, self.inner_expr.kind()) }
    }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LetNormalTE<'s, 't>
where 's: 't,
{
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> LetNormalTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.expr.result().region,
                KindT::Void(VoidT {}),
            )
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct UnletTE<'s, 't> {
    pub variable: ILocalVariableT<'s, 't>,
}

impl<'s, 't> UnletTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT { coord: self.variable.coord() }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DiscardTE<'s, 't>
where 's: 't,
{
    pub expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> DiscardTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.expr.result().region,
                KindT::Void(VoidT),
            )
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DeferTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ExpressionTE<'s, 't>,
    pub deferred_expr: ExpressionTE<'s, 't>,
    _sealed: (),
}

impl<'s, 't> DeferTE<'s, 't> {


    pub fn result(&self) -> KindT<'s, 't> {
        KindT { coord: self.inner_expr.result() }
    }

}
impl<'s, 't> DeferTE<'s, 't> where 's: 't, {
    pub fn new(
        inner_expr: ExpressionTE<'s, 't>,
        deferred_expr: ExpressionTE<'s, 't>,
    ) -> DeferTE<'s, 't> {
        let inner_coord = inner_expr.result();
        assert!(deferred_expr.result() == KindT::new(
            OwnershipT::Own,
            inner_coord.region,
            KindT::Void(VoidT),
        ));
        DeferTE { inner_expr, deferred_expr, _sealed: () }
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
    pub common_supertype: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> IfTE<'s, 't> {


    pub fn new(
        condition: ExpressionTE<'s, 't>,
        then_call: ExpressionTE<'s, 't>,
        else_call: ExpressionTE<'s, 't>,
    ) -> IfTE<'s, 't> {
        let condition_result_coord = condition.result();
        let then_result_coord = then_call.result();
        let else_result_coord = else_call.result();
        match condition_result_coord {
            KindT { kind: KindT::Bool(_), ownership: OwnershipT::Own, .. } => {}
            other => panic!("vfail: {:?}", other),
        }
        match (then_result_kind, then_result_kind) {
            (KindT::Never(_), _) => {}
            (_, KindT::Never(_)) => {}
            (a, b) if a == b => {}
            _ => panic!("vwat"),
        }
        let common_supertype = match then_result_kind {
            KindT::Never(_) => else_result_coord,
            _ => then_result_coord,
        };
        IfTE { condition, then_call, else_call, common_supertype, _sealed: () }
    }
    
    pub fn result(&self) -> KindT<'s, 't> {
        KindT { coord: self.common_supertype }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct WhileTE<'s, 't>
where 's: 't,
{
    pub block: BlockTE<'s, 't>,
    pub result_coord: KindT<'s, 't>,
    _sealed: (),
}

impl<'s, 't> WhileTE<'s, 't> {
    pub fn new(block: BlockTE<'s, 't>) -> WhileTE<'s, 't> {
        let result_coord = match block.result().kind {
            KindT::Void(_) => block.result(),
            KindT::Never(NeverT { from_break: true }) => KindT::new(
                OwnershipT::Own,
                block.result().region,
                KindT::Void(VoidT),
            ),
            KindT::Never(NeverT { from_break: false }) => block.result(),
            _ => panic!("vwat"),
        };
        WhileTE { block, result_coord, _sealed: () }
    }
    

    fn result(&self) -> KindT<'s, 't> { KindT { coord: self.result_coord } }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct MutateTE<'s, 't>
where 's: 't,
{
    pub destination_expr: ExpressionTE<'s, 't>,
    pub source_expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> MutateTE<'s, 't> {


    pub fn result(&self) -> KindT<'s, 't> {
        KindT { coord: self.destination_expr.result() }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RestackifyTE<'s, 't>
where 's: 't,
{
    pub variable: ILocalVariableT<'s, 't>,
    pub source_expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> RestackifyTE<'s, 't> {


    pub fn result(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(
            OwnershipT::Own,
            self.source_expr.result().region,
            KindT::Void(VoidT),
        ) }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReturnTE<'s, 't>
where 's: 't,
{
    pub source_expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> ReturnTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.source_expr.result().region,
                KindT::Never(NeverT { from_break: false }),
            )
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BreakTE {
    pub region: RegionT,
}

impl BreakTE {


    fn result<'s, 't>(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(OwnershipT::Own, self.region, KindT::Never(NeverT { from_break: true })) }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BlockTE<'s, 't>
where 's: 't,
{
    pub inner: ExpressionTE<'s, 't>,
}

impl<'s, 't> BlockTE<'s, 't> {


    pub fn result(&self) -> KindT<'s, 't> { self.inner.result() }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConsecutorTE<'s, 't>
where 's: 't,
{
    pub exprs: &'t [ExpressionTE<'s, 't>],
}

impl<'s, 't> ConsecutorTE<'s, 't> {


}
impl<'s, 't> ConsecutorTE<'s, 't> where 's: 't, {
    fn new(exprs: &'t [ExpressionTE<'s, 't>]) -> ConsecutorTE<'s, 't> { panic!("Unimplemented: ConsecutorTE::new"); }

}
impl<'s, 't> ConsecutorTE<'s, 't> {
    pub fn result(&self) -> KindT<'s, 't> {
        let never_coord = self.exprs.iter()
            .map(|e| e.result())
            .find(|c| matches!(c, KindT { ownership: OwnershipT::Own, kind: KindT::Never(_), .. }));
        match never_coord {
            Some(n) => KindT { coord: n },
            None => self.exprs.last().unwrap().result(),
        }
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
    pub result_reference: KindT<'s, 't>,
}

impl<'s, 't> TupleTE<'s, 't> {


    pub fn result(&self) -> KindT<'s, 't> { KindT { coord: self.result_reference } }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticArrayFromValuesTE<'s, 't>
where 's: 't,
{
    pub elements: &'t [ExpressionTE<'s, 't>],
    pub result_reference: KindT<'s, 't>,
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
}

impl<'s, 't> StaticArrayFromValuesTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> { KindT { coord: self.result_reference } }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArraySizeTE<'s, 't>
where 's: 't,
{
    pub array: ExpressionTE<'s, 't>,
}

impl<'s, 't> ArraySizeTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        panic!("Unimplemented: result");
        // KindT(KindT(ShareT, array.result.coord.region, IntT.i32))
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct IsSameInstanceTE<'s, 't>
where 's: 't,
{
    pub left: ExpressionTE<'s, 't>,
    pub right: ExpressionTE<'s, 't>,
}

impl<'s, 't> IsSameInstanceTE<'s, 't> {


}
impl<'s, 't> IsSameInstanceTE<'s, 't> where 's: 't, {
    fn new(left: ExpressionTE<'s, 't>, right: ExpressionTE<'s, 't>) -> IsSameInstanceTE<'s, 't> { panic!("Unimplemented: IsSameInstanceTE::new"); }

}
impl<'s, 't> IsSameInstanceTE<'s, 't> {
    pub fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.left.result().region,
                KindT::Bool(BoolT),
            ),
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct AsSubtypeTE<'s, 't>
where 's: 't,
{
    pub source_expr: ExpressionTE<'s, 't>,
    pub target_type: KindT<'s, 't>,
    pub result_result_type: KindT<'s, 't>,
    pub ok_constructor: &'t PrototypeT<'s, 't>,
    pub err_constructor: &'t PrototypeT<'s, 't>,
    pub impl_name: IdT<'s, 't>,
    pub ok_impl_name: IdT<'s, 't>,
    pub err_impl_name: IdT<'s, 't>,
}

impl<'s, 't> AsSubtypeTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> { KindT { coord: self.result_result_type } }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct VoidLiteralTE {
    pub region: RegionT,
}

impl VoidLiteralTE {


    fn result<'s, 't>(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.region,
                KindT::Void(VoidT),
            )
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantIntTE<'s, 't> {
    pub value: ITemplataT<'s, 't>,
    pub bits: i32,
    pub region: RegionT,
}

impl<'s, 't> ConstantIntTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(OwnershipT::Own, self.region, KindT::Int(IntT { bits: self.bits })) }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantBoolTE {
    pub value: bool,
    pub region: RegionT,
}

impl ConstantBoolTE {


    pub fn result<'s, 't>(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(OwnershipT::Own, self.region, KindT::Bool(BoolT)) }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantStrTE<'s> {
    pub value: StrI<'s>,
    pub region: RegionT,
}

impl<'s> ConstantStrTE<'s> {


    fn result<'t>(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(OwnershipT::Share, self.region, KindT::Str(StrT)) }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantFloatTE {
    pub value: f64,
    pub region: RegionT,
}

impl ConstantFloatTE {


    pub fn result<'s, 't>(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(
            OwnershipT::Own,
            self.region,
            KindT::Float(FloatT),
        ) }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LocalLookupTE<'s, 't> {
    pub range: RangeS<'s>,
    pub local_variable: ILocalVariableT<'s, 't>,
}

impl<'s, 't> LocalLookupTE<'s, 't> {


    pub fn result(&self) -> AddressResultT<'s, 't> {
        AddressResultT { coord: self.local_variable.coord() }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArgLookupTE<'s, 't> {
    pub param_index: i32,
    pub coord: KindT<'s, 't>,
}

impl<'s, 't> ArgLookupTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT { coord: self.coord }
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
    pub element_type: KindT<'s, 't>,
}

impl<'s, 't> StaticSizedArrayLookupTE<'s, 't> {


    fn result(&self) -> AddressResultT<'s, 't> { AddressResultT { coord: self.element_type } }

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
    _sealed: (),
}

impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> {


}
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> where 's: 't, {
    pub fn new(
        range: RangeS<'s>,
        array_expr: ExpressionTE<'s, 't>,
        array_type: &'t RuntimeSizedArrayTT<'s, 't>,
        index_expr: ExpressionTE<'s, 't>,
    ) -> RuntimeSizedArrayLookupTE<'s, 't> {
        assert_eq!(array_expr.result().kind, KindT::RuntimeSizedArray(array_type));
        RuntimeSizedArrayLookupTE { range, array_expr, array_type, index_expr, _sealed: () }
    }

}
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> {
    pub fn result(&self) -> AddressResultT<'s, 't> {
        // See RMLRMO why we just return the element type.
        AddressResultT { coord: self.array_type.element_type() }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArrayLengthTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> ArrayLengthTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.array_expr.result().region,
                KindT::Int(IntT::I32),
            ),
        }
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
    pub member_reference: KindT<'s, 't>,
}

impl<'s, 't> ReferenceMemberLookupTE<'s, 't> {


    pub fn result(&self) -> AddressResultT<'s, 't> {
        // See RMLRMO why we just return the member type.
        AddressResultT { coord: self.member_reference }
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
    pub result_type2: KindT<'s, 't>,
}

impl<'s, 't> AddressMemberLookupTE<'s, 't> {


    fn result(&self) -> AddressResultT<'s, 't> { AddressResultT { coord: self.result_type2 } }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct InterfaceFunctionCallTE<'s, 't>
where 's: 't,
{
    pub super_function_prototype: &'t PrototypeT<'s, 't>,
    pub virtual_param_index: i32,
    pub result_reference: KindT<'s, 't>,
    pub args: &'t [ExpressionTE<'s, 't>],
}

impl<'s, 't> InterfaceFunctionCallTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> { KindT { coord: self.result_reference } }

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
}

impl<'s, 't> ExternFunctionCallTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> { KindT { coord: self.prototype2.return_type } }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct FunctionCallTE<'s, 't>
where 's: 't,
{
    pub callable: &'t PrototypeT<'s, 't>,
    pub args: &'t [ExpressionTE<'s, 't>],
    pub return_type: KindT<'s, 't>,
}

impl<'s, 't> FunctionCallTE<'s, 't> {


}
impl<'s, 't> FunctionCallTE<'s, 't> where 's: 't, {
    fn new(
        callable: &'t PrototypeT<'s, 't>,
        args: &'t [ExpressionTE<'s, 't>],
        return_type: KindT<'s, 't>,
    ) -> FunctionCallTE<'s, 't> { panic!("Unimplemented: FunctionCallTE::new"); }

}
impl<'s, 't> FunctionCallTE<'s, 't> {
    fn result(&self) -> KindT<'s, 't> { KindT { coord: self.return_type } }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReinterpretTE<'s, 't>
where 's: 't,
{
    pub expr: ExpressionTE<'s, 't>,
    pub result_reference: KindT<'s, 't>,
}

impl<'s, 't> ReinterpretTE<'s, 't> {


}
impl<'s, 't> ReinterpretTE<'s, 't> where 's: 't, {
    fn new(expr: ExpressionTE<'s, 't>, result_reference: KindT<'s, 't>) -> ReinterpretTE<'s, 't> { panic!("Unimplemented: ReinterpretTE::new"); }

}
impl<'s, 't> ReinterpretTE<'s, 't> {
    fn result(&self) -> KindT<'s, 't> {
        KindT { coord: self.result_reference }
    }
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct CopyPrimTE<'s, 't> {
    pub inner: ExpressionTE<'s, 't>,
    pub result_coord: KindT<'s, 't>,
}
impl<'s, 't> CopyPrimTE<'s, 't> {
    pub fn result(&self) -> KindT<'s, 't> {
        KindT { coord: self.result_coord }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstructTE<'s, 't>
where 's: 't,
{
    pub struct_tt: &'t StructTT<'s, 't>,
    pub result_reference: KindT<'s, 't>,
    pub args: &'t [ExpressionTE<'s, 't>],
}

impl<'s, 't> ConstructTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> { KindT { coord: self.result_reference } }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct NewRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_type: &'t RuntimeSizedArrayTT<'s, 't>,
    pub region: RegionT,
    pub capacity_expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> NewRuntimeSizedArrayTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.region,
                KindT::RuntimeSizedArray(self.array_type),
            ),
        }
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
}

impl<'s, 't> StaticArrayFromCallableTE<'s, 't> {


    pub fn result(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(OwnershipT::Own, self.region, KindT::StaticSizedArray(self.array_type)) }
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
}

impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> {


}
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> where 's: 't, {
    fn new(
        array_expr: ExpressionTE<'s, 't>,
        array_type: &'t StaticSizedArrayTT<'s, 't>,
        consumer: ExpressionTE<'s, 't>,
        consumer_method: &'t PrototypeT<'s, 't>,
    ) -> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> { panic!("Unimplemented: DestroyStaticSizedArrayIntoFunctionTE::new"); }

}
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> {
    fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.array_expr.result().region,
                KindT::Void(VoidT),
            ),
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsTE<'s, 't>
where 's: 't,
{
    pub expr: ExpressionTE<'s, 't>,
    pub static_sized_array: &'t StaticSizedArrayTT<'s, 't>,
    pub destination_reference_variables: &'t [ReferenceLocalVariableT<'s, 't>],
}

impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(OwnershipT::Own, self.expr.result().region, KindT::Void(VoidT)) }
    }

}
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> where 's: 't, {
    fn new(
        expr: ExpressionTE<'s, 't>,
        static_sized_array: &'t StaticSizedArrayTT<'s, 't>,
        destination_reference_variables: &'t [ReferenceLocalVariableT<'s, 't>],
    ) -> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> { panic!("Unimplemented: DestroyStaticSizedArrayIntoLocalsTE::new"); }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> DestroyRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.array_expr.result().region,
                KindT::Void(VoidT),
            )
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RuntimeSizedArrayCapacityTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> RuntimeSizedArrayCapacityTE<'s, 't> {
    fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.array_expr.result().region,
                KindT::Int(IntT { bits: 32 }),
            ),
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct PushRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
    pub new_element_expr: ExpressionTE<'s, 't>,
}

impl<'s, 't> PushRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> KindT<'s, 't> {
        KindT {
            coord: KindT::new(
                OwnershipT::Own,
                self.array_expr.result().region,
                KindT::Void(VoidT),
            ),
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct PopRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ExpressionTE<'s, 't>,
    pub element_type: KindT<'s, 't>,
}

impl<'s, 't> PopRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> KindT<'s, 't> {
        KindT { coord: self.element_type }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct InterfaceToInterfaceUpcastTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ExpressionTE<'s, 't>,
    pub target_interface: &'t InterfaceTT<'s, 't>,
}

impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        panic!("Unimplemented: result");
        // KindT(
        //   KindT(
        //     innerExpr.result.coord.ownership,
        //     innerExpr.result.coord.region,
        //     targetInterface))
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
}

impl<'s, 't> UpcastTE<'s, 't> {


    pub fn result(&self) -> KindT<'s, 't> {
        let inner_coord = self.inner_expr.result();
        KindT {
            coord: KindT::new(
                inner_coord.ownership,
                inner_coord.region,
                self.target_super_kind.into(),
            )
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct SoftLoadTE<'s, 't>
where 's: 't,
{
    pub expr: ExpressionTE<'s, 't>,
    pub target_ownership: OwnershipT,
}

impl<'s, 't> SoftLoadTE<'s, 't> {


}
impl<'s, 't> SoftLoadTE<'s, 't> where 's: 't, {
    fn new(expr: ExpressionTE<'s, 't>, target_ownership: OwnershipT) -> SoftLoadTE<'s, 't> { panic!("Unimplemented: SoftLoadTE::new"); }

}
impl<'s, 't> SoftLoadTE<'s, 't> {
    fn result(&self) -> KindT<'s, 't> {
        let addr_result = self.expr.result();
        KindT {
            coord: KindT::new(
                self.target_ownership,
                addr_result.coord.region,
                addr_result.kind,
            )
        }
    }

}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyTE<'s, 't>
where 's: 't,
{
    pub expr: ExpressionTE<'s, 't>,
    pub struct_tt: &'t StructTT<'s, 't>,
    pub destination_reference_variables: &'t [ReferenceLocalVariableT<'s, 't>],
}

impl<'s, 't> DestroyTE<'s, 't> {


    fn result(&self) -> KindT<'s, 't> {
        KindT { coord: KindT::new(OwnershipT::Own, self.expr.result().region, KindT::Void(VoidT {})) }
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
