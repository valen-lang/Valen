
use crate::utils::range::RangeS;
use crate::instantiating::ast::types::{
	KindIT, BorrowRefIT,
	InterfaceIT, RuntimeSizedArrayIT, StaticSizedArrayIT, StructIT,
	BoolIT, IntIT, VoidIT, NeverIT, StrIT, FloatIT,
};
use crate::instantiating::ast::names::{IdI, IVarNameI};
use crate::instantiating::ast::ast::{
    LocalVariableI, PrototypeI,
};
use std::marker::PhantomData;


/// One flat expression IR — the pre-onion Reference/Address two-sort split is gone (every local is
/// storage; a lookup yields a borrow of that storage). Mirrors typing's flat `ExpressionTE`.
/// Every variant carries a `range: RangeS<'s>` (see `range()`) for per-instruction DWARF; the
/// instantiator copies it from the source TE node. Backend currently discards it.
/// Arena-allocated (see @TFITCX)
#[derive(Copy, Clone, Debug)]
pub enum ExpressionIE<'s, 'i> {
    LetAndLend(&'i LetAndLendIE<'s, 'i>),
    LockWeak(&'i LockWeakIE<'s, 'i>),
    BorrowToWeak(&'i BorrowToWeakIE<'s, 'i>),
    LetNormal(&'i LetNormalIE<'s, 'i>),
    Restackify(&'i RestackifyIE<'s, 'i>),
    Unlet(&'i UnletIE<'s, 'i>),
    Discard(&'i DiscardIE<'s, 'i>),
    If(&'i IfIE<'s, 'i>),
    While(&'i WhileIE<'s, 'i>),
    Mutate(&'i MutateIE<'s, 'i>),
    Return(&'i ReturnIE<'s, 'i>),
    Break(&'i BreakIE<'s>),
    Block(&'i BlockIE<'s, 'i>),
    Consecutor(&'i ConsecutorIE<'s, 'i>),
    StaticArrayFromValues(&'i StaticArrayFromValuesIE<'s, 'i>),
    ArraySize(&'i ArraySizeIE<'s, 'i>),
    IsSameInstance(&'i IsSameInstanceIE<'s, 'i>),
    AsSubtype(&'i AsSubtypeIE<'s, 'i>),
    VoidLiteral(&'i VoidLiteralIE<'s>),
    ConstantInt(&'i ConstantIntIE<'s>),
    ConstantBool(&'i ConstantBoolIE<'s>),
    ConstantStr(&'i ConstantStrIE<'s, 'i>),
    ConstantFloat(&'i ConstantFloatIE<'s>),
    ArgLookup(&'i ArgLookupIE<'s, 'i>),
    ArrayLength(&'i ArrayLengthIE<'s, 'i>),
    InterfaceFunctionCall(&'i InterfaceFunctionCallIE<'s, 'i>),
    ExternFunctionCall(&'i ExternFunctionCallIE<'s, 'i>),
    FunctionCall(&'i FunctionCallIE<'s, 'i>),
    Construct(&'i ConstructIE<'s, 'i>),
    NewRuntimeSizedArray(&'i NewRuntimeSizedArrayIE<'s, 'i>),
    StaticArrayFromCallable(&'i StaticArrayFromCallableIE<'s, 'i>),
    DestroyStaticSizedArrayIntoFunction(&'i DestroyStaticSizedArrayIntoFunctionIE<'s, 'i>),
    DestroyStaticSizedArrayIntoLocals(&'i DestroyStaticSizedArrayIntoLocalsIE<'s, 'i>),
    DestroyRuntimeSizedArray(&'i DestroyRuntimeSizedArrayIE<'s, 'i>),
    RuntimeSizedArrayCapacity(&'i RuntimeSizedArrayCapacityIE<'s, 'i>),
    PushRuntimeSizedArray(&'i PushRuntimeSizedArrayIE<'s, 'i>),
    PopRuntimeSizedArray(&'i PopRuntimeSizedArrayIE<'s, 'i>),
    InterfaceToInterfaceUpcast(&'i InterfaceToInterfaceUpcastIE<'s, 'i>),
    Upcast(&'i UpcastIE<'s, 'i>),
    Destroy(&'i DestroyIE<'s, 'i>),
    CopyPrim(&'i CopyPrimIE<'s, 'i>),
    LocalLookup(&'i LocalLookupIE<'s, 'i>),
    StaticSizedArrayLookup(&'i StaticSizedArrayLookupIE<'s, 'i>),
    RuntimeSizedArrayLookup(&'i RuntimeSizedArrayLookupIE<'s, 'i>),
    MemberLookup(&'i MemberLookupIE<'s, 'i>),
    Deref(&'i DerefIE<'s, 'i>),
}


impl<'s, 'i> ExpressionIE<'s, 'i> {
    pub fn result(&self) -> KindIT<'s, 'i> {
        match self {
            ExpressionIE::LetAndLend(x) => x.result,
            ExpressionIE::LockWeak(x) => x.result,
            ExpressionIE::BorrowToWeak(x) => x.result,
            ExpressionIE::LetNormal(x) => x.result,
            ExpressionIE::Restackify(x) => x.result,
            ExpressionIE::Unlet(x) => x.result,
            ExpressionIE::Discard(x) => x.result(),
            ExpressionIE::If(x) => x.result,
            ExpressionIE::While(x) => x.result,
            ExpressionIE::Mutate(m) => m.result,
            ExpressionIE::Return(x) => x.result(),
            ExpressionIE::Break(x) => x.result(),
            ExpressionIE::Block(x) => x.result,
            ExpressionIE::Consecutor(x) => x.result,
            ExpressionIE::StaticArrayFromValues(s) => s.result,
            ExpressionIE::ArraySize(x) => x.result,
            ExpressionIE::IsSameInstance(x) => x.result(),
            ExpressionIE::AsSubtype(x) => x.result,
            ExpressionIE::VoidLiteral(v) => v.result(),
            ExpressionIE::ConstantInt(x) => x.result(),
            ExpressionIE::ConstantBool(x) => x.result(),
            ExpressionIE::ConstantStr(x) => x.result(),
            ExpressionIE::ConstantFloat(x) => x.result(),
            ExpressionIE::ArgLookup(x) => x.tyype,
            ExpressionIE::ArrayLength(x) => x.result(),
            ExpressionIE::InterfaceFunctionCall(x) => x.result,
            ExpressionIE::ExternFunctionCall(e) => e.result,
            ExpressionIE::FunctionCall(c) => c.result,
            ExpressionIE::Construct(c) => c.result,
            ExpressionIE::NewRuntimeSizedArray(n) => n.result,
            ExpressionIE::StaticArrayFromCallable(s) => s.result,
            ExpressionIE::DestroyStaticSizedArrayIntoFunction(d) => d.result(),
            ExpressionIE::DestroyStaticSizedArrayIntoLocals(x) => x.result(),
            ExpressionIE::DestroyRuntimeSizedArray(x) => x.result(),
            ExpressionIE::RuntimeSizedArrayCapacity(r) => r.result(),
            ExpressionIE::PushRuntimeSizedArray(x) => x.result(),
            ExpressionIE::PopRuntimeSizedArray(p) => p.result,
            ExpressionIE::InterfaceToInterfaceUpcast(i) => i.result,
            ExpressionIE::Upcast(u) => u.result,
            ExpressionIE::Destroy(x) => x.result(),
            ExpressionIE::CopyPrim(c) => c.result,
            ExpressionIE::LocalLookup(x) => KindIT::BorrowRefIT(x.result),
            ExpressionIE::StaticSizedArrayLookup(x) => KindIT::BorrowRefIT(x.result),
            ExpressionIE::RuntimeSizedArrayLookup(x) => KindIT::BorrowRefIT(x.result),
            ExpressionIE::MemberLookup(x) => KindIT::BorrowRefIT(x.result),
            ExpressionIE::Deref(x) => x.result,
        }
    }

    /// The source range of this expression, for per-instruction DWARF. Populated by the
    /// instantiator from the source TE node.
    pub fn range(&self) -> RangeS<'s> {
        match self {
            ExpressionIE::LetAndLend(x) => x.range,
            ExpressionIE::LockWeak(x) => x.range,
            ExpressionIE::BorrowToWeak(x) => x.range,
            ExpressionIE::LetNormal(x) => x.range,
            ExpressionIE::Restackify(x) => x.range,
            ExpressionIE::Unlet(x) => x.range,
            ExpressionIE::Discard(x) => x.range,
            ExpressionIE::If(x) => x.range,
            ExpressionIE::While(x) => x.range,
            ExpressionIE::Mutate(x) => x.range,
            ExpressionIE::Return(x) => x.range,
            ExpressionIE::Break(x) => x.range,
            ExpressionIE::Block(x) => x.range,
            ExpressionIE::Consecutor(x) => x.range,
            ExpressionIE::StaticArrayFromValues(x) => x.range,
            ExpressionIE::ArraySize(x) => x.range,
            ExpressionIE::IsSameInstance(x) => x.range,
            ExpressionIE::AsSubtype(x) => x.range,
            ExpressionIE::VoidLiteral(x) => x.range,
            ExpressionIE::ConstantInt(x) => x.range,
            ExpressionIE::ConstantBool(x) => x.range,
            ExpressionIE::ConstantStr(x) => x.range,
            ExpressionIE::ConstantFloat(x) => x.range,
            ExpressionIE::ArgLookup(x) => x.range,
            ExpressionIE::ArrayLength(x) => x.range,
            ExpressionIE::InterfaceFunctionCall(x) => x.range,
            ExpressionIE::ExternFunctionCall(x) => x.range,
            ExpressionIE::FunctionCall(x) => x.range,
            ExpressionIE::Construct(x) => x.range,
            ExpressionIE::NewRuntimeSizedArray(x) => x.range,
            ExpressionIE::StaticArrayFromCallable(x) => x.range,
            ExpressionIE::DestroyStaticSizedArrayIntoFunction(x) => x.range,
            ExpressionIE::DestroyStaticSizedArrayIntoLocals(x) => x.range,
            ExpressionIE::DestroyRuntimeSizedArray(x) => x.range,
            ExpressionIE::RuntimeSizedArrayCapacity(x) => x.range,
            ExpressionIE::PushRuntimeSizedArray(x) => x.range,
            ExpressionIE::PopRuntimeSizedArray(x) => x.range,
            ExpressionIE::InterfaceToInterfaceUpcast(x) => x.range,
            ExpressionIE::Upcast(x) => x.range,
            ExpressionIE::Destroy(x) => x.range,
            ExpressionIE::CopyPrim(x) => x.range,
            ExpressionIE::LocalLookup(x) => x.range,
            ExpressionIE::StaticSizedArrayLookup(x) => x.range,
            ExpressionIE::RuntimeSizedArrayLookup(x) => x.range,
            ExpressionIE::MemberLookup(x) => x.range,
            ExpressionIE::Deref(x) => x.range,
        }
    }
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct LetAndLendIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub variable: &'i LocalVariableI<'s, 'i>,
	pub expr: ExpressionIE<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct LockWeakIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub inner_expr: ExpressionIE<'s, 'i>,
	pub source_type: KindIT<'s, 'i>,
	pub some_constructor: PrototypeI<'s, 'i>,
	pub none_constructor: PrototypeI<'s, 'i>,
	pub some_impl_name: IdI<'s, 'i>,
	pub none_impl_name: IdI<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct BorrowToWeakIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub inner_expr: ExpressionIE<'s, 'i>,
	pub source_type: KindIT<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct LetNormalIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub variable: &'i LocalVariableI<'s, 'i>,
	pub expr: ExpressionIE<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct RestackifyIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub variable: &'i LocalVariableI<'s, 'i>,
	pub source_expr: ExpressionIE<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct UnletIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub variable: &'i LocalVariableI<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DiscardIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub expr: ExpressionIE<'s, 'i>,
	pub source_type: KindIT<'s, 'i>,
}



impl<'s, 'i> DiscardIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::VoidIT(VoidIT {  })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct IfIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub condition: ExpressionIE<'s, 'i>,
	pub then_call: ExpressionIE<'s, 'i>,
	pub else_call: ExpressionIE<'s, 'i>,
	pub then_result_type: KindIT<'s, 'i>,
	pub else_result_type: KindIT<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct WhileIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub block: BlockIE<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct MutateIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub destination_expr: ExpressionIE<'s, 'i>,
	pub destination_type: &'i BorrowRefIT<'s, 'i>,
	pub source_expr: ExpressionIE<'s, 'i>,
	pub source_type: KindIT<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ReturnIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub source_expr: ExpressionIE<'s, 'i>,
	pub source_type: KindIT<'s, 'i>,
}



impl<'s, 'i> ReturnIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::NeverIT(NeverIT { from_break: false })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct BreakIE<'s> {
	pub range: RangeS<'s>,
}



impl<'s> BreakIE<'s> {
	pub fn result<'i>(&self) -> KindIT<'s, 'i> {
		KindIT::NeverIT(NeverIT { from_break: true })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct BlockIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub inner: ExpressionIE<'s, 'i>,
	pub inner_type: KindIT<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConsecutorIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub exprs: &'i[ExpressionIE<'s, 'i>],
	pub result: KindIT<'s, 'i>,
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromValuesIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub elements: &'i[ExpressionIE<'s, 'i>],
	pub result: KindIT<'s, 'i>,
	pub array_type: &'i StaticSizedArrayIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ArraySizeIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array: ExpressionIE<'s, 'i>,
	pub array_type: &'i BorrowRefIT<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct IsSameInstanceIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub left: ExpressionIE<'s, 'i>,
	pub left_type: KindIT<'s, 'i>,
	pub right: ExpressionIE<'s, 'i>,
	pub right_type: KindIT<'s, 'i>,
}



impl<'s, 'i> IsSameInstanceIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::BoolIT(BoolIT {  })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct AsSubtypeIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub source_expr: ExpressionIE<'s, 'i>,
	pub source_type: KindIT<'s, 'i>,
	pub target_type: KindIT<'s, 'i>,
	pub ok_constructor: &'i PrototypeI<'s, 'i>,
	pub err_constructor: &'i PrototypeI<'s, 'i>,
	pub impl_name: IdI<'s, 'i>,
	pub ok_impl_name: IdI<'s, 'i>,
	pub err_impl_name: IdI<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct VoidLiteralIE<'s> {
	pub range: RangeS<'s>,
}



impl<'s> VoidLiteralIE<'s> {
	pub fn result<'i>(&self) -> KindIT<'s, 'i> {
		KindIT::VoidIT(VoidIT {  })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstantIntIE<'s> {
	pub range: RangeS<'s>,
	pub value: i64,
	pub bits: i32,
}



impl<'s> ConstantIntIE<'s> {
	pub fn result<'i>(&self) -> KindIT<'s, 'i> {
		KindIT::IntIT(IntIT { bits: self.bits })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstantBoolIE<'s> {
	pub range: RangeS<'s>,
	pub value: bool,
}



impl<'s> ConstantBoolIE<'s> {
	pub fn result<'i>(&self) -> KindIT<'s, 'i> {
		KindIT::BoolIT(BoolIT {  })
	}
}


/// A string constant is share-wrapped (immutable/shared). Its ShareRefIT result is arena-allocated
/// at construction and stored here, since a wrap can't be built without the interner.
/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstantStrIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub _marker: PhantomData<(&'s (),)>,
	pub value: &'s str,
	pub result: KindIT<'s, 'i>,
}



impl<'s, 'i> ConstantStrIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		self.result
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstantFloatIE<'s> {
	pub range: RangeS<'s>,
	pub value: f64,
}



impl<'s> ConstantFloatIE<'s> {
	pub fn result<'i>(&self) -> KindIT<'s, 'i> {
		KindIT::FloatIT(FloatIT {  })
	}
}


/// A local lookup yields a borrow of the local's storage (mirrors typing's LocalLookupTE).
/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct LocalLookupIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub local_variable: &'i LocalVariableI<'s, 'i>,
	pub result: &'i BorrowRefIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ArgLookupIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub param_index: i32,
	pub tyype: KindIT<'s, 'i>,
}



impl<'s, 'i> ArgLookupIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		self.tyype
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct StaticSizedArrayLookupIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ExpressionIE<'s, 'i>,
	pub array_type: &'i BorrowRefIT<'s, 'i>,
	pub index_expr: ExpressionIE<'s, 'i>,
	pub index_type: KindIT<'s, 'i>,
	pub result: &'i BorrowRefIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayLookupIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ExpressionIE<'s, 'i>,
	pub array_type: &'i BorrowRefIT<'s, 'i>,
	pub index_expr: ExpressionIE<'s, 'i>,
	pub index_type: KindIT<'s, 'i>,
	pub result: &'i BorrowRefIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ArrayLengthIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ExpressionIE<'s, 'i>,
	pub array_type: &'i BorrowRefIT<'s, 'i>,
}



impl<'s, 'i> ArrayLengthIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::IntIT(IntIT { bits: 32 })
	}
}


/// A member lookup yields a borrow of the member's storage (mirrors typing's MemberLookupTE).
/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct MemberLookupIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub struct_expr: ExpressionIE<'s, 'i>,
	pub struct_type: &'i BorrowRefIT<'s, 'i>,
	pub member_index: i32,
	pub member_name: IVarNameI<'s, 'i>,
	pub member_type: KindIT<'s, 'i>,
	pub result: &'i BorrowRefIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct InterfaceFunctionCallIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub super_function_prototype: &'i PrototypeI<'s, 'i>,
	pub virtual_param_index: i32,
	// Vtable slot of the called method within its interface — the position of this method
	// in typing's InterfaceEdgeBlueprintT.super_family_root_headers (typing owns the order).
	pub index_in_edge: i32,
	pub args: &'i[ExpressionIE<'s, 'i>],
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ExternFunctionCallIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub prototype2: PrototypeI<'s, 'i>,
	pub args: &'i[ExpressionIE<'s, 'i>],
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct FunctionCallIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub callable: PrototypeI<'s, 'i>,
	pub args: &'i[ExpressionIE<'s, 'i>],
	pub result: KindIT<'s, 'i>,
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct CopyPrimIE<'s, 'i> {
	pub range: RangeS<'s>,
    pub inner: ExpressionIE<'s, 'i>,
    pub source_type: KindIT<'s, 'i>,
    pub result: KindIT<'s, 'i>,
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstructIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub struct_tt: StructIT<'s, 'i>,
	pub result: KindIT<'s, 'i>,
	pub args: &'i[ExpressionIE<'s, 'i>],
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct NewRuntimeSizedArrayIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_type: RuntimeSizedArrayIT<'s, 'i>,
	pub capacity_expr: ExpressionIE<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromCallableIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_type: StaticSizedArrayIT<'s, 'i>,
	pub generator: ExpressionIE<'s, 'i>,
	pub generator_method: PrototypeI<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoFunctionIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ExpressionIE<'s, 'i>,
	pub array_type: StaticSizedArrayIT<'s, 'i>,
	pub consumer: ExpressionIE<'s, 'i>,
	pub consumer_method: PrototypeI<'s, 'i>,
}



impl<'s, 'i> DestroyStaticSizedArrayIntoFunctionIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::VoidIT(VoidIT {  })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub expr: ExpressionIE<'s, 'i>,
	pub static_sized_array: StaticSizedArrayIT<'s, 'i>,
	pub destination_reference_variables: &'i[&'i LocalVariableI<'s, 'i>],
}



impl<'s, 'i> DestroyStaticSizedArrayIntoLocalsIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::VoidIT(VoidIT {  })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DestroyRuntimeSizedArrayIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ExpressionIE<'s, 'i>,
	pub array_type: KindIT<'s, 'i>,
}

impl<'s, 'i> DestroyRuntimeSizedArrayIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::VoidIT(VoidIT {  })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayCapacityIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ExpressionIE<'s, 'i>,
	pub array_type: &'i BorrowRefIT<'s, 'i>,
}



impl<'s, 'i> RuntimeSizedArrayCapacityIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::IntIT(IntIT { bits: 32 })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct PushRuntimeSizedArrayIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ExpressionIE<'s, 'i>,
	pub array_type: &'i BorrowRefIT<'s, 'i>,
	pub new_element_expr: ExpressionIE<'s, 'i>,
	pub element_type: KindIT<'s, 'i>,
}



impl<'s, 'i> PushRuntimeSizedArrayIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::VoidIT(VoidIT {  })
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct PopRuntimeSizedArrayIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ExpressionIE<'s, 'i>,
	pub array_type: &'i BorrowRefIT<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct InterfaceToInterfaceUpcastIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub inner_expr: ExpressionIE<'s, 'i>,
	pub source_type: KindIT<'s, 'i>,
	pub target_interface: InterfaceIT<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct UpcastIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub inner_expr: ExpressionIE<'s, 'i>,
	pub source_type: KindIT<'s, 'i>,
	pub target_interface: InterfaceIT<'s, 'i>,
	pub impl_name: IdI<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// A read that peels exactly one reference wrap off its inner (mirrors typing's DerefTE).
/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DerefIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub inner: ExpressionIE<'s, 'i>,
	pub source_type: KindIT<'s, 'i>,
	pub result: KindIT<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DestroyIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub expr: ExpressionIE<'s, 'i>,
	pub struct_tt: StructIT<'s, 'i>,
	pub destination_reference_variables: &'i[&'i LocalVariableI<'s, 'i>],
}



impl<'s, 'i> DestroyIE<'s, 'i> {
	pub fn result(&self) -> KindIT<'s, 'i> {
		KindIT::VoidIT(VoidIT {  })
	}
}
