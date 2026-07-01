
use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::instantiating::ast::types::{
	CoordI, OwnershipI, SharednessI,
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


/// Arena-allocated (see @TFITCX)
#[derive(Copy, Clone, Debug)]
pub enum ExpressionIE<'s, 'i> {
    Reference(ReferenceExpressionIE<'s, 'i>),
    Address(AddressExpressionIE<'s, 'i>),
}


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
            ReferenceExpressionIE::NewRuntimeSizedArray(n) => n.result,
            ReferenceExpressionIE::StaticArrayFromCallable(s) => s.result,
            ReferenceExpressionIE::DestroyStaticSizedArrayIntoFunction(d) => d.result(),
            ReferenceExpressionIE::DestroyStaticSizedArrayIntoLocals(_) => panic!("RE::result: DestroyStaticSizedArrayIntoLocals"),
            ReferenceExpressionIE::DestroyRuntimeSizedArray(_) => CoordI::new(OwnershipI::Own, KindIT::VoidIT(VoidIT {  })),
            ReferenceExpressionIE::RuntimeSizedArrayCapacity(r) => r.result(),
            ReferenceExpressionIE::PushRuntimeSizedArray(_) => CoordI::new(OwnershipI::Own, KindIT::VoidIT(VoidIT {  })),
            ReferenceExpressionIE::PopRuntimeSizedArray(p) => p.result,
            ReferenceExpressionIE::InterfaceToInterfaceUpcast(i) => i.result,
            ReferenceExpressionIE::Upcast(u) => u.result,
            ReferenceExpressionIE::SoftLoad(s) => s.result,
            ReferenceExpressionIE::Destroy(_) => CoordI::new(OwnershipI::MutableShare, KindIT::VoidIT(VoidIT {  })),
            ReferenceExpressionIE::CopyPrim(c) => c.result,
        }
    }
}


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
    SoftLoad(&'i SoftLoadIE<'s, 'i>),
    Destroy(&'i DestroyIE<'s, 'i>),
    CopyPrim(&'i CopyPrimIE<'s, 'i>),
}


#[derive(Copy, Clone, Debug)]
pub enum AddressExpressionIE<'s, 'i> {
    LocalLookup(&'i LocalLookupIE<'s, 'i>),
    StaticSizedArrayLookup(&'i StaticSizedArrayLookupIE<'s, 'i>),
    RuntimeSizedArrayLookup(&'i RuntimeSizedArrayLookupIE<'s, 'i>),
    ReferenceMemberLookup(&'i ReferenceMemberLookupIE<'s, 'i>),
    AddressMemberLookup(&'i AddressMemberLookupIE<'s, 'i>),
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct LetAndLendIE<'s, 'i> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub target_ownership: OwnershipI,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
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



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct BorrowToWeakIE<'s, 'i> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct LetNormalIE<'s, 'i> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct RestackifyIE<'s, 'i> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct UnletIE<'s, 'i> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DiscardIE<'s, 'i> {
	pub expr: ReferenceExpressionIE<'s, 'i>,
}



impl<'s, 'i> DiscardIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DeferIE<'s, 'i> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i>,
	pub deferred_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct IfIE<'s, 'i> {
	pub condition: ReferenceExpressionIE<'s, 'i>,
	pub then_call: ReferenceExpressionIE<'s, 'i>,
	pub else_call: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct WhileIE<'s, 'i> {
	pub block: BlockIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct MutateIE<'s, 'i> {
	pub destination_expr: AddressExpressionIE<'s, 'i>,
	pub source_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ReturnIE<'s, 'i> {
	pub source_expr: ReferenceExpressionIE<'s, 'i>,
}



impl<'s, 'i> ReturnIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, NeverIT(false))
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct BreakIE;



impl BreakIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, NeverIT(true))
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct BlockIE<'s, 'i> {
	pub inner: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct MutabilifyIE<'s, 'i> {
	pub inner: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ImmutabilifyIE<'s, 'i> {
	pub inner: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct PreCheckBorrowIE<'s, 'i> {
	pub inner: ReferenceExpressionIE<'s, 'i>,
}



impl<'s, 'i> PreCheckBorrowIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// inner.result
	}
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConsecutorIE<'s, 'i> {
	pub exprs: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct TupleIE<'s, 'i> {
	pub elements: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromValuesIE<'s, 'i> {
	pub elements: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result_reference: CoordI<'s, 'i>,
	pub array_type: StaticSizedArrayIT<'s, 'i>,
}



impl<'s, 'i> StaticArrayFromValuesIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// resultReference
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ArraySizeIE<'s, 'i> {
	pub array: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct IsSameInstanceIE<'s, 'i> {
	pub left: ReferenceExpressionIE<'s, 'i>,
	pub right: ReferenceExpressionIE<'s, 'i>,
}



impl<'s, 'i> IsSameInstanceIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		CoordI::new(OwnershipI::Own, KindIT::BoolIT(BoolIT {  }))
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
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



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct VoidLiteralIE;



impl VoidLiteralIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		CoordI::new(OwnershipI::Own, KindIT::VoidIT(VoidIT {  }))
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstantIntIE {
	pub value: i64,
	pub bits: i32,
}



impl ConstantIntIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		CoordI::new(
			OwnershipI::Own,
			KindIT::IntIT(IntIT { bits: self.bits }),
		)
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstantBoolIE {
	pub value: bool,
}



impl ConstantBoolIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		CoordI::new(OwnershipI::Own, KindIT::BoolIT(BoolIT {  }))
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstantStrIE<'s> {
	pub _marker: PhantomData<(&'s (),)>,
	pub value: &'s str,
}



impl<'s> ConstantStrIE<'s> {
	pub fn result<'i>(&self) -> CoordI<'s, 'i> {
		CoordI::new(OwnershipI::MutableShare, KindIT::StrIT(StrIT {  }))
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstantFloatIE {
	pub value: f64,
}



impl ConstantFloatIE {
	pub fn result<'s, 'i>(&self) -> CoordI<'s, 'i> {
		CoordI::new(OwnershipI::Own, KindIT::FloatIT(FloatIT {  }))
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct LocalLookupIE<'s, 'i> {
	pub local_variable: ILocalVariableI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ArgLookupIE<'s, 'i> {
	pub param_index: i32,
	pub coord: CoordI<'s, 'i>,
}



impl<'s, 'i> ArgLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// coord
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct StaticSizedArrayLookupIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub index_expr: ReferenceExpressionIE<'s, 'i>,
	pub element_type: CoordI<'s, 'i>,
}



impl<'s, 'i> StaticSizedArrayLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// elementType
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayLookupIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub index_expr: ReferenceExpressionIE<'s, 'i>,
	pub element_type: CoordI<'s, 'i>,
}



impl<'s, 'i> RuntimeSizedArrayLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// elementType
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ArrayLengthIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
}



impl<'s, 'i> ArrayLengthIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		CoordI::new(
			OwnershipI::Own,
			KindIT::IntIT(IntIT { bits: 32 }),
		)
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ReferenceMemberLookupIE<'s, 'i> {
	pub range: RangeS<'s>,
	pub struct_expr: ReferenceExpressionIE<'s, 'i>,
	pub member_name: IVarNameI<'s, 'i>,
	pub member_reference: CoordI<'s, 'i>,
}



impl<'s, 'i> ReferenceMemberLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// memberReference
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct AddressMemberLookupIE<'s, 'i> {
	pub struct_expr: ReferenceExpressionIE<'s, 'i>,
	pub member_name: IVarNameI<'s, 'i>,
	pub member_reference: CoordI<'s, 'i>,
}



impl<'s, 'i> AddressMemberLookupIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// memberReference
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct InterfaceFunctionCallIE<'s, 'i> {
	pub super_function_prototype: &'i PrototypeI<'s, 'i>,
	pub virtual_param_index: i32,
	pub args: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ExternFunctionCallIE<'s, 'i> {
	pub prototype2: PrototypeI<'s, 'i>,
	pub args: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct FunctionCallIE<'s, 'i> {
	pub callable: PrototypeI<'s, 'i>,
	pub args: &'i[ReferenceExpressionIE<'s, 'i>],
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ReinterpretIE<'s, 'i> {
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub result_reference: CoordI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}
#[derive(Copy, Clone, Debug)]
pub struct CopyPrimIE<'s, 'i> {
    pub inner: ReferenceExpressionIE<'s, 'i>,
    pub result: CoordI<'s, 'i>,
}

/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct ConstructIE<'s, 'i> {
	pub struct_tt: StructIT<'s, 'i>,
	pub result: CoordI<'s, 'i>,
	pub args: &'i[ExpressionIE<'s, 'i>],
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct NewRuntimeSizedArrayIE<'s, 'i> {
	pub array_type: RuntimeSizedArrayIT<'s, 'i>,
	pub capacity_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromCallableIE<'s, 'i> {
	pub array_type: StaticSizedArrayIT<'s, 'i>,
	pub generator: ReferenceExpressionIE<'s, 'i>,
	pub generator_method: PrototypeI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoFunctionIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub array_type: StaticSizedArrayIT<'s, 'i>,
	pub consumer: ReferenceExpressionIE<'s, 'i>,
	pub consumer_method: PrototypeI<'s, 'i>,
}



impl<'s, 'i> DestroyStaticSizedArrayIntoFunctionIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		CoordI::new(OwnershipI::Own, KindIT::VoidIT(VoidIT {  }))
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsIE<'s, 'i> {
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub static_sized_array: StaticSizedArrayIT<'s, 'i>,
	pub destination_reference_variables: &'i[ReferenceLocalVariableI<'s, 'i>],
}



impl<'s, 'i> DestroyStaticSizedArrayIntoLocalsIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DestroyRuntimeSizedArrayIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
}

impl<'s, 'i> DestroyRuntimeSizedArrayIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayCapacityIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
}



impl<'s, 'i> RuntimeSizedArrayCapacityIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		CoordI::new(OwnershipI::Own, KindIT::IntIT(IntIT { bits: 32 }))
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct PushRuntimeSizedArrayIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub new_element_expr: ReferenceExpressionIE<'s, 'i>,
}



impl<'s, 'i> PushRuntimeSizedArrayIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}


/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct PopRuntimeSizedArrayIE<'s, 'i> {
	pub array_expr: ReferenceExpressionIE<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct InterfaceToInterfaceUpcastIE<'s, 'i> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i>,
	pub target_interface: InterfaceIT<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct UpcastIE<'s, 'i> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i>,
	pub target_interface: InterfaceIT<'s, 'i>,
	pub impl_name: IdI<'s, 'i>,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct SoftLoadIE<'s, 'i> {
	pub expr: AddressExpressionIE<'s, 'i>,
	pub target_ownership: OwnershipI,
	pub result: CoordI<'s, 'i>,
}



/// Arena-allocated (see @TFITCX) — no equality.
#[derive(Copy, Clone, Debug)]
pub struct DestroyIE<'s, 'i> {
	pub expr: ReferenceExpressionIE<'s, 'i>,
	pub struct_tt: StructIT<'s, 'i>,
	pub destination_reference_variables: &'i[ReferenceLocalVariableI<'s, 'i>],
}



impl<'s, 'i> DestroyIE<'s, 'i> {
	pub fn result(&self) -> CoordI<'s, 'i> {
		panic!("Unimplemented: result");
		// CoordI[cI](MutableShareI, VoidIT())
	}
}

