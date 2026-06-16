
use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::instantiating::ast::types::{
    CoordI, OwnershipI, MutabilityI, VariabilityI,
    InterfaceIT, RuntimeSizedArrayIT, StaticSizedArrayIT, StructIT,
    KindIT, BoolIT,
};
use crate::instantiating::ast::names::{IdI, IVarNameI};
use crate::instantiating::ast::ast::{
    IVariableI, ILocalVariableI, PrototypeI, ReferenceLocalVariableI,
};
use crate::instantiating::ast::types::FloatIT;
use crate::instantiating::ast::types::IntIT;
use crate::instantiating::ast::types::StrIT;
use crate::instantiating::ast::types::VoidIT;
use std::marker::PhantomData;

// mig: trait ExpressionIE — realized as enum per @ATDCX (no dyn, mirrors typing-pass ExpressionTE).
/// Arena-allocated (see @TFITCX)
//
// No PartialEq/Hash derive: Scala uses `vcurious` (panic on equals) on every
// expression case class; Rust's "no impl" gives a strictly stronger compile-time
// error. Mirrors typing/ast/expressions.rs ExpressionTE.
#[derive(Copy, Clone, Debug)]
pub enum ExpressionIE<'s, 'i> {
    Reference(ReferenceExpressionIE<'s, 'i>),
    Address(AddressExpressionIE<'s, 'i>),
}

// mig: fn result
impl<'s, 'i> ExpressionIE<'s, 'i> {
    pub fn result(&self) -> CoordI<'s, 'i> {
        match self {
            ExpressionIE::Reference(r) => r.result(),
            ExpressionIE::Address(a) => panic!("ExpressionIE::result: Address branch"),
        }
    }
}
impl<'s, 'i> ReferenceExpressionIE<'s, 'i> {
    pub fn result(&self) -> CoordI<'s, 'i> {
        match self {
            ReferenceExpressionIE::LetAndLend(x) => x.result,
            ReferenceExpressionIE::LockWeak(x) => x.result,
            ReferenceExpressionIE::BorrowToWeak(x) => x.result,
            ReferenceExpressionIE::LetNormal(x) => x.result,
            ReferenceExpressionIE::Restackify(_) => panic!("RE::result: Restackify"),
            ReferenceExpressionIE::Unlet(x) => x.result,
            ReferenceExpressionIE::Discard(_) => panic!("RE::result: Discard"),
            ReferenceExpressionIE::Defer(x) => x.result,
            ReferenceExpressionIE::If(x) => x.result,
            ReferenceExpressionIE::While(_) => panic!("RE::result: While"),
            ReferenceExpressionIE::Mutate(m) => m.result,
            ReferenceExpressionIE::Return(_) => panic!("RE::result: Return"),
            ReferenceExpressionIE::Break(_) => panic!("RE::result: Break"),
            ReferenceExpressionIE::Block(x) => x.result,
            ReferenceExpressionIE::Mutabilify(_) => panic!("RE::result: Mutabilify"),
            ReferenceExpressionIE::Immutabilify(_) => panic!("RE::result: Immutabilify"),
            ReferenceExpressionIE::PreCheckBorrow(_) => panic!("RE::result: PreCheckBorrow"),
            ReferenceExpressionIE::Consecutor(x) => x.result,
            ReferenceExpressionIE::Tuple(x) => x.result,
            ReferenceExpressionIE::StaticArrayFromValues(s) => s.result_reference,
            ReferenceExpressionIE::ArraySize(_) => panic!("RE::result: ArraySize"),
            ReferenceExpressionIE::IsSameInstance(x) => x.result(),
            ReferenceExpressionIE::AsSubtype(x) => x.result,
            ReferenceExpressionIE::VoidLiteral(v) => v.result(),
            ReferenceExpressionIE::ConstantInt(x) => x.result(),
            ReferenceExpressionIE::ConstantBool(x) => x.result(),
            ReferenceExpressionIE::ConstantStr(x) => x.result(),
            ReferenceExpressionIE::ConstantFloat(x) => x.result(),
            ReferenceExpressionIE::ArgLookup(x) => x.coord,
            ReferenceExpressionIE::ArrayLength(x) => x.result(),
            ReferenceExpressionIE::InterfaceFunctionCall(x) => x.result,
            ReferenceExpressionIE::ExternFunctionCall(e) => e.result,
            ReferenceExpressionIE::FunctionCall(c) => c.result,
            ReferenceExpressionIE::Reinterpret(_) => panic!("RE::result: Reinterpret"),
            ReferenceExpressionIE::Construct(c) => c.result,
            ReferenceExpressionIE::NewMutRuntimeSizedArray(n) => n.result,
            ReferenceExpressionIE::StaticArrayFromCallable(s) => s.result,
            ReferenceExpressionIE::DestroyStaticSizedArrayIntoFunction(d) => d.result(),
            ReferenceExpressionIE::DestroyStaticSizedArrayIntoLocals(_) => CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT {  }) },
            ReferenceExpressionIE::DestroyMutRuntimeSizedArray(_) => CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT {  }) },
            ReferenceExpressionIE::RuntimeSizedArrayCapacity(r) => r.result(),
            ReferenceExpressionIE::PushRuntimeSizedArray(_) => CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT {  }) },
            ReferenceExpressionIE::PopRuntimeSizedArray(p) => p.result,
            ReferenceExpressionIE::InterfaceToInterfaceUpcast(i) => i.result,
            ReferenceExpressionIE::Upcast(u) => u.result,
            ReferenceExpressionIE::SoftLoad(s) => s.result,
            ReferenceExpressionIE::Destroy(_) => CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT {  }) },
            ReferenceExpressionIE::DestroyImmRuntimeSizedArray(_) => CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT {  }) },
            ReferenceExpressionIE::NewImmRuntimeSizedArray(n) => n.result,
        }
    }
}

// mig: trait ReferenceExpressionIE — realized as enum per @ATDCX.
#[derive(Copy, Clone, Debug)]
pub enum ReferenceExpressionIE<'s, 'i> {
    LetAndLend(&'i LetAndLendIE<'s, 'i>),
    LockWeak(&'i LockWeakIE<'s, 'i>),
    BorrowToWeak(&'i BorrowToWeakIE<'s, 'i>),
    LetNormal(&'i LetNormalIE<'s, 'i>),
    Restackify(&'i RestackifyIE<'s, 'i>),
    Unlet(&'i UnletIE<'s, 'i>),
    Discard(&'i DiscardIE<'s, 'i>),
    Defer(&'i DeferIE<'s, 'i>),
    If(&'i IfIE<'s, 'i>),
    While(&'i WhileIE<'s, 'i>),
    Mutate(&'i MutateIE<'s, 'i>),
    Return(&'i ReturnIE<'s, 'i>),
    Break(&'i BreakIE),
    Block(&'i BlockIE<'s, 'i>),
    Mutabilify(&'i MutabilifyIE<'s, 'i>),
    Immutabilify(&'i ImmutabilifyIE<'s, 'i>),
    PreCheckBorrow(&'i PreCheckBorrowIE<'s, 'i>),
    Consecutor(&'i ConsecutorIE<'s, 'i>),
    Tuple(&'i TupleIE<'s, 'i>),
    StaticArrayFromValues(&'i StaticArrayFromValuesIE<'s, 'i>),
    ArraySize(&'i ArraySizeIE<'s, 'i>),
    IsSameInstance(&'i IsSameInstanceIE<'s, 'i>),
    AsSubtype(&'i AsSubtypeIE<'s, 'i>),
    VoidLiteral(&'i VoidLiteralIE),
    ConstantInt(&'i ConstantIntIE),
    ConstantBool(&'i ConstantBoolIE),
    ConstantStr(&'i ConstantStrIE<'s>),
    ConstantFloat(&'i ConstantFloatIE),
    ArgLookup(&'i ArgLookupIE<'s, 'i>),
    ArrayLength(&'i ArrayLengthIE<'s, 'i>),
    InterfaceFunctionCall(&'i InterfaceFunctionCallIE<'s, 'i>),
    ExternFunctionCall(&'i ExternFunctionCallIE<'s, 'i>),
    FunctionCall(&'i FunctionCallIE<'s, 'i>),
    Reinterpret(&'i ReinterpretIE<'s, 'i>),
    Construct(&'i ConstructIE<'s, 'i>),
    NewMutRuntimeSizedArray(&'i NewMutRuntimeSizedArrayIE<'s, 'i>),
    StaticArrayFromCallable(&'i StaticArrayFromCallableIE<'s, 'i>),
    DestroyStaticSizedArrayIntoFunction(&'i DestroyStaticSizedArrayIntoFunctionIE<'s, 'i>),
    DestroyStaticSizedArrayIntoLocals(&'i DestroyStaticSizedArrayIntoLocalsIE<'s, 'i>),
    DestroyMutRuntimeSizedArray(&'i DestroyMutRuntimeSizedArrayIE<'s, 'i>),
    RuntimeSizedArrayCapacity(&'i RuntimeSizedArrayCapacityIE<'s, 'i>),
    PushRuntimeSizedArray(&'i PushRuntimeSizedArrayIE<'s, 'i>),
    PopRuntimeSizedArray(&'i PopRuntimeSizedArrayIE<'s, 'i>),
    InterfaceToInterfaceUpcast(&'i InterfaceToInterfaceUpcastIE<'s, 'i>),
    Upcast(&'i UpcastIE<'s, 'i>),
    SoftLoad(&'i SoftLoadIE<'s, 'i>),
    Destroy(&'i DestroyIE<'s, 'i>),
    DestroyImmRuntimeSizedArray(&'i DestroyImmRuntimeSizedArrayIE<'s, 'i>),
    NewImmRuntimeSizedArray(&'i NewImmRuntimeSizedArrayIE<'s, 'i>),
}

// mig: trait AddressExpressionIE — realized as enum per @ATDCX.
#[derive(Copy, Clone, Debug)]
pub enum AddressExpressionIE<'s, 'i> {
    LocalLookup(&'i LocalLookupIE<'s, 'i>),
    StaticSizedArrayLookup(&'i StaticSizedArrayLookupIE<'s, 'i>),
    RuntimeSizedArrayLookup(&'i RuntimeSizedArrayLookupIE<'s, 'i>),
    ReferenceMemberLookup(&'i ReferenceMemberLookupIE<'s, 'i>),
    AddressMemberLookup(&'i AddressMemberLookupIE<'s, 'i>),
}

// mig: struct LetAndLendIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct LetAndLendIE<'s, 'i> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub target_ownership: OwnershipI,
	pub result: CoordI<'s, 'i>,
}
// mig: impl LetAndLendIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LetAndLendIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LetAndLendIE` below.)

// mig: struct LockWeakIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct LockWeakIE<'s, 'i> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i>,
	pub result_opt_borrow_type: CoordI<'s, 'i>,
	pub some_constructor: PrototypeI<'s, 'i>,
	pub none_constructor: PrototypeI<'s, 'i>,
	pub some_impl_name: IdI<'s, 'i>,
	pub none_impl_name: IdI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl LockWeakIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LockWeakIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LockWeakIE` below.)

// mig: struct BorrowToWeakIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct BorrowToWeakIE<'s, 'i> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl BorrowToWeakIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for BorrowToWeakIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for BorrowToWeakIE` below.)

// mig: struct LetNormalIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct LetNormalIE<'s, 'i> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl LetNormalIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LetNormalIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LetNormalIE` below.)

// mig: struct RestackifyIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct RestackifyIE<'s, 'i> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl RestackifyIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for RestackifyIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for RestackifyIE` below.)

// mig: struct UnletIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct UnletIE<'s, 'i> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl UnletIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for UnletIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for UnletIE` below.)

// mig: struct DiscardIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DiscardIE<'s, 'i> {
	pub expr: ReferenceExpressionIE<'s, 'i>,
}
// mig: impl DiscardIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DiscardIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DiscardIE` below.)

// mig: fn result
impl<'s, 'i> DiscardIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}

// mig: struct DeferIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DeferIE<'s, 'i> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i>,
	pub deferred_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl DeferIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DeferIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DeferIE` below.)

// mig: struct IfIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct IfIE<'s, 'i> {
	pub condition: ReferenceExpressionIE<'s, 'i>,
	pub then_call: ReferenceExpressionIE<'s, 'i>,
	pub else_call: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl IfIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for IfIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for IfIE` below.)

// mig: struct WhileIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct WhileIE<'s, 'i> {
	pub block: BlockIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl WhileIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for WhileIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for WhileIE` below.)

// mig: struct MutateIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct MutateIE<'s, 'i> {
	pub destination_expr: AddressExpressionIE<'s, 'i>,
	pub source_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl MutateIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for MutateIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for MutateIE` below.)

// mig: struct ReturnIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ReturnIE<'s, 'i> {
	pub source_expr: ReferenceExpressionIE<'s, 'i>,
}
// mig: impl ReturnIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReturnIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReturnIE` below.)

// mig: fn result
impl<'s, 'i> ReturnIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, NeverIT(false))
	}
}

// mig: struct BreakIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct BreakIE;
// mig: impl BreakIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for BreakIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for BreakIE` below.)

// mig: fn result
impl BreakIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, NeverIT(true))
	}
}

// mig: struct BlockIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct BlockIE<'s, 'i> {
	pub inner: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl BlockIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for BlockIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for BlockIE` below.)

// mig: struct MutabilifyIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct MutabilifyIE<'s, 'i> {
	pub inner: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl MutabilifyIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for MutabilifyIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for MutabilifyIE` below.)

// mig: struct ImmutabilifyIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ImmutabilifyIE<'s, 'i> {
	pub inner: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl ImmutabilifyIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ImmutabilifyIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ImmutabilifyIE` below.)

// mig: struct PreCheckBorrowIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct PreCheckBorrowIE<'s, 'i> {
	pub inner: ReferenceExpressionIE<'s, 'i>,
}
// mig: impl PreCheckBorrowIE

// mig: fn result
impl<'s, 'i> PreCheckBorrowIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// inner.result
	}
}

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for PreCheckBorrowIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for PreCheckBorrowIE` below.)

// mig: struct ConsecutorIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConsecutorIE<'s, 'i> {
	pub exprs: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}
// mig: impl ConsecutorIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConsecutorIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConsecutorIE` below.)

// mig: struct TupleIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct TupleIE<'s, 'i> {
	pub elements: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}
// mig: impl TupleIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for TupleIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for TupleIE` below.)

// mig: struct StaticArrayFromValuesIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromValuesIE<'s, 'i> {
	pub elements: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result_reference: CoordI<'s, 'i>,
	pub array_type: StaticSizedArrayIT<'s, 'i>,
}
// mig: impl StaticArrayFromValuesIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for StaticArrayFromValuesIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for StaticArrayFromValuesIE` below.)

// mig: fn result
impl<'s, 'i> StaticArrayFromValuesIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// resultReference
	}
}

// mig: struct ArraySizeIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ArraySizeIE<'s, 'i> {
	pub array: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl ArraySizeIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ArraySizeIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ArraySizeIE` below.)

// mig: struct IsSameInstanceIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct IsSameInstanceIE<'s, 'i> {
	pub left: ReferenceExpressionIE<'s, 'i>,
	pub right: ReferenceExpressionIE<'s, 'i>,
}
// mig: impl IsSameInstanceIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for IsSameInstanceIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for IsSameInstanceIE` below.)

// mig: fn result
impl<'s, 'i> IsSameInstanceIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::BoolIT(BoolIT {  }) }
	}
}

// mig: struct AsSubtypeIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct AsSubtypeIE<'s, 'i> {
	pub source_expr: ReferenceExpressionIE<'s, 'i>,
	pub target_type: CoordI<'s, 'i>,
	pub result_result_type: CoordI<'s, 'i>,
	pub ok_constructor: &'i PrototypeI<'s, 'i>,
	pub err_constructor: &'i PrototypeI<'s, 'i>,
	pub impl_name: IdI<'s, 'i>,
	pub ok_impl_name: IdI<'s, 'i>,
	pub err_impl_name: IdI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl AsSubtypeIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for AsSubtypeIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for AsSubtypeIE` below.)

// mig: struct VoidLiteralIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct VoidLiteralIE;
// mig: impl VoidLiteralIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for VoidLiteralIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for VoidLiteralIE` below.)

// mig: fn result
impl VoidLiteralIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT {  }) }
	}
}

// mig: struct ConstantIntIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstantIntIE {
	pub value: i64,
	pub bits: i32,
}
// mig: impl ConstantIntIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstantIntIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstantIntIE` below.)

// mig: fn result
impl ConstantIntIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		CoordI {
			ownership: OwnershipI::MutableShare,
			kind: KindIT::IntIT(IntIT { bits: self.bits }),
		}
	}
}

// mig: struct ConstantBoolIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstantBoolIE {
	pub value: bool,
}
// mig: impl ConstantBoolIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstantBoolIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstantBoolIE` below.)

// mig: fn result
impl ConstantBoolIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::BoolIT(BoolIT {  }) }
	}
}

// mig: struct ConstantStrIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstantStrIE<'s> {
	pub _marker: PhantomData<(&'s (),)>,
	pub value: &'s str,
}
// mig: impl ConstantStrIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstantStrIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstantStrIE` below.)

// mig: fn result
impl<'s> ConstantStrIE<'s> {
	pub fn result<'i>(&self) -> CoordI<'s, 'i> {
		CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::StrIT(StrIT {  }) }
	}
}

// mig: struct ConstantFloatIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstantFloatIE {
	pub value: f64,
}
// mig: impl ConstantFloatIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstantFloatIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstantFloatIE` below.)

// mig: fn result
impl ConstantFloatIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::FloatIT(FloatIT {  }) }
	}
}

// mig: struct LocalLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct LocalLookupIE<'s, 'i> {
	pub local_variable: ILocalVariableI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl LocalLookupIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LocalLookupIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LocalLookupIE` below.)

// mig: struct ArgLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ArgLookupIE<'s, 'i> {
	pub param_index: i32,
	pub coord: CoordI<'s, 'i>,
}
// mig: impl ArgLookupIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ArgLookupIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ArgLookupIE` below.)

// mig: fn result
impl<'s, 'i> ArgLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// coord
	}
}

// mig: struct StaticSizedArrayLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct StaticSizedArrayLookupIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub index_expr: ReferenceExpressionIE<'s, 'i>,
	pub element_type: CoordI<'s, 'i>,
	pub variability: VariabilityI,
}
// mig: impl StaticSizedArrayLookupIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for StaticSizedArrayLookupIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for StaticSizedArrayLookupIE` below.)

// mig: fn result
impl<'s, 'i> StaticSizedArrayLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// elementType
	}
}

// mig: struct RuntimeSizedArrayLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayLookupIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub index_expr: ReferenceExpressionIE<'s, 'i>,
	pub element_type: CoordI<'s, 'i>,
	pub variability: VariabilityI,
}
// mig: impl RuntimeSizedArrayLookupIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for RuntimeSizedArrayLookupIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for RuntimeSizedArrayLookupIE` below.)

// mig: fn result
impl<'s, 'i> RuntimeSizedArrayLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// elementType
	}
}

// mig: struct ArrayLengthIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ArrayLengthIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
}
// mig: impl ArrayLengthIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ArrayLengthIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ArrayLengthIE` below.)

// mig: fn result
impl<'s, 'i> ArrayLengthIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		CoordI {
			ownership: OwnershipI::MutableShare,
			kind: KindIT::IntIT(IntIT { bits: 32 }),
		}
	}
}

// mig: struct ReferenceMemberLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ReferenceMemberLookupIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub struct_expr: ReferenceExpressionIE<'s, 'i>,
	pub member_name: IVarNameI<'s, 'i>,
	pub member_reference: CoordI<'s, 'i>,
	pub variability: VariabilityI,
}
// mig: impl ReferenceMemberLookupIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReferenceMemberLookupIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReferenceMemberLookupIE` below.)

// mig: fn result
impl<'s, 'i> ReferenceMemberLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// memberReference
	}
}

// mig: struct AddressMemberLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct AddressMemberLookupIE<'s, 'i> {
	pub struct_expr: ReferenceExpressionIE<'s, 'i>,
	pub member_name: IVarNameI<'s, 'i>,
	pub member_reference: CoordI<'s, 'i>,
	pub variability: VariabilityI,
}
// mig: impl AddressMemberLookupIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for AddressMemberLookupIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for AddressMemberLookupIE` below.)

// mig: fn result
impl<'s, 'i> AddressMemberLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// memberReference
	}
}

// mig: struct InterfaceFunctionCallIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct InterfaceFunctionCallIE<'s, 'i> {
	pub super_function_prototype: &'i PrototypeI<'s, 'i>,
	pub virtual_param_index: i32,
	pub args: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}
// mig: impl InterfaceFunctionCallIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for InterfaceFunctionCallIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for InterfaceFunctionCallIE` below.)

// mig: struct ExternFunctionCallIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ExternFunctionCallIE<'s, 'i> {
	pub prototype2: PrototypeI<'s, 'i>,
	pub args: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}
// mig: impl ExternFunctionCallIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ExternFunctionCallIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ExternFunctionCallIE` below.)

// mig: struct FunctionCallIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct FunctionCallIE<'s, 'i> {
	pub callable: PrototypeI<'s, 'i>,
	pub args: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}
// mig: impl FunctionCallIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionCallIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionCallIE` below.)

// mig: struct ReinterpretIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ReinterpretIE<'s, 'i> {
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub result_reference: CoordI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl ReinterpretIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReinterpretIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReinterpretIE` below.)

// mig: struct ConstructIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstructIE<'s, 'i> {
	pub struct_tt: StructIT<'s, 'i>,
	pub result: CoordI<'s, 'i>,
	pub args: &'i[ExpressionIE<'s, 'i>],
}
// mig: impl ConstructIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstructIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstructIE` below.)

// mig: struct NewMutRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct NewMutRuntimeSizedArrayIE<'s, 'i> {
	pub array_type: RuntimeSizedArrayIT<'s, 'i>,
	pub capacity_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl NewMutRuntimeSizedArrayIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for NewMutRuntimeSizedArrayIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for NewMutRuntimeSizedArrayIE` below.)

// mig: struct StaticArrayFromCallableIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromCallableIE<'s, 'i> {
	pub array_type: StaticSizedArrayIT<'s, 'i>,
	pub generator: ReferenceExpressionIE<'s, 'i>,
	pub generator_method: PrototypeI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl StaticArrayFromCallableIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for StaticArrayFromCallableIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for StaticArrayFromCallableIE` below.)

// mig: struct DestroyStaticSizedArrayIntoFunctionIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoFunctionIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub array_type: StaticSizedArrayIT<'s, 'i>,
	pub consumer: ReferenceExpressionIE<'s, 'i>,
	pub consumer_method: PrototypeI<'s, 'i>,
}
// mig: impl DestroyStaticSizedArrayIntoFunctionIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DestroyStaticSizedArrayIntoFunctionIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DestroyStaticSizedArrayIntoFunctionIE` below.)

// mig: fn result
impl<'s, 'i> DestroyStaticSizedArrayIntoFunctionIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT {  }) }
	}
}

// mig: struct DestroyStaticSizedArrayIntoLocalsIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsIE<'s, 'i> {
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub static_sized_array: StaticSizedArrayIT<'s, 'i>,
	pub destination_reference_variables: &'i[ReferenceLocalVariableI<'s, 'i>],
}
// mig: impl DestroyStaticSizedArrayIntoLocalsIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DestroyStaticSizedArrayIntoLocalsIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DestroyStaticSizedArrayIntoLocalsIE` below.)

// mig: fn result
impl<'s, 'i> DestroyStaticSizedArrayIntoLocalsIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}

// mig: struct DestroyMutRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyMutRuntimeSizedArrayIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
}
// mig: impl DestroyMutRuntimeSizedArrayIE

// mig: fn result
impl<'s, 'i> DestroyMutRuntimeSizedArrayIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}

// mig: struct RuntimeSizedArrayCapacityIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayCapacityIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
}
// mig: impl RuntimeSizedArrayCapacityIE

// mig: fn result
impl<'s, 'i> RuntimeSizedArrayCapacityIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::IntIT(IntIT { bits: 32 }) }
	}
}

// mig: struct PushRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct PushRuntimeSizedArrayIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub new_element_expr: ReferenceExpressionIE<'s, 'i>,
}
// mig: impl PushRuntimeSizedArrayIE

// mig: fn result
impl<'s, 'i> PushRuntimeSizedArrayIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}

// mig: struct PopRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct PopRuntimeSizedArrayIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl PopRuntimeSizedArrayIE

// mig: struct InterfaceToInterfaceUpcastIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct InterfaceToInterfaceUpcastIE<'s, 'i> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i>,
	pub target_interface: InterfaceIT<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl InterfaceToInterfaceUpcastIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for InterfaceToInterfaceUpcastIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for InterfaceToInterfaceUpcastIE` below.)

// mig: struct UpcastIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct UpcastIE<'s, 'i> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i>,
	pub target_interface: InterfaceIT<'s, 'i>,
	pub impl_name: IdI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl UpcastIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for UpcastIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for UpcastIE` below.)

// mig: struct SoftLoadIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct SoftLoadIE<'s, 'i> {
	pub expr: AddressExpressionIE<'s, 'i>,
	pub target_ownership: OwnershipI,
	pub result: CoordI<'s, 'i>,
}
// mig: impl SoftLoadIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for SoftLoadIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for SoftLoadIE` below.)

// mig: struct DestroyIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyIE<'s, 'i> {
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub struct_tt: StructIT<'s, 'i>,
	pub destination_reference_variables: &'i[ReferenceLocalVariableI<'s, 'i>],
}
// mig: impl DestroyIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DestroyIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DestroyIE` below.)

// mig: fn result
impl<'s, 'i> DestroyIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}

// mig: struct DestroyImmRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyImmRuntimeSizedArrayIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub array_type: RuntimeSizedArrayIT<'s, 'i>,
	pub consumer: ReferenceExpressionIE<'s, 'i>,
	pub consumer_method: PrototypeI<'s, 'i>,
}
// mig: impl DestroyImmRuntimeSizedArrayIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DestroyImmRuntimeSizedArrayIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DestroyImmRuntimeSizedArrayIE` below.)

// mig: fn result
impl<'s, 'i> DestroyImmRuntimeSizedArrayIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}

// mig: struct NewImmRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct NewImmRuntimeSizedArrayIE<'s, 'i> {
	pub array_type: RuntimeSizedArrayIT<'s, 'i>,
	pub size_expr: ReferenceExpressionIE<'s, 'i>,
	pub generator: ReferenceExpressionIE<'s, 'i>,
	pub generator_method: PrototypeI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
// mig: impl NewImmRuntimeSizedArrayIE

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for NewImmRuntimeSizedArrayIE` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for NewImmRuntimeSizedArrayIE` below.)

// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<ReferenceExpressionIE>` or inline match.)

// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<ReferenceExpressionIE>` or inline match.)
