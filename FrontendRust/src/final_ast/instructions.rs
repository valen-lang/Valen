
#[allow(unused_imports)]
use std::marker::PhantomData;

use crate::final_ast::ast::{IdH, PrototypeH};
use crate::final_ast::types::*;
use crate::final_ast::types::CoordH;
use crate::final_ast::types::IntHT;
use crate::final_ast::types::KindHT;
use crate::final_ast::types::LocationH;
use crate::final_ast::types::OwnershipH;
use crate::final_ast::types::VoidHT;



/// Polyvalue
#[derive(Copy, Clone, Debug)]
pub enum ExpressionH<'s, 'h> where 's: 'h {
    ConstantVoidH(&'h ConstantVoidH),
    ConstantIntH(&'h ConstantIntH),
    ConstantBoolH(&'h ConstantBoolH),
    ConstantStrH(&'h ConstantStrH<'h>),
    ConstantF64H(&'h ConstantF64H),
    ArgumentH(&'h ArgumentH<'s, 'h>),
    StackifyH(&'h StackifyH<'s, 'h>),
    RestackifyH(&'h RestackifyH<'s, 'h>),
    UnstackifyH(&'h UnstackifyH<'s, 'h>),
    DestroyH(&'h DestroyH<'s, 'h>),
    DestroyStaticSizedArrayIntoLocalsH(&'h DestroyStaticSizedArrayIntoLocalsH<'s, 'h>),
    StructToInterfaceUpcastH(&'h StructToInterfaceUpcastH<'s, 'h>),
    InterfaceToInterfaceUpcastH(&'h InterfaceToInterfaceUpcastH<'s, 'h>),
    LocalStoreH(&'h LocalStoreH<'s, 'h>),
    LocalLoadH(&'h LocalLoadH<'s, 'h>),
    MemberStoreH(&'h MemberStoreH<'s, 'h>),
    MemberLoadH(&'h MemberLoadH<'s, 'h>),
    NewArrayFromValuesH(&'h NewArrayFromValuesH<'s, 'h>),
    StaticSizedArrayStoreH(&'h StaticSizedArrayStoreH<'s, 'h>),
    RuntimeSizedArrayStoreH(&'h RuntimeSizedArrayStoreH<'s, 'h>),
    RuntimeSizedArrayLoadH(&'h RuntimeSizedArrayLoadH<'s, 'h>),
    StaticSizedArrayLoadH(&'h StaticSizedArrayLoadH<'s, 'h>),
    CallH(&'h CallH<'s, 'h>),
    ExternCallH(&'h ExternCallH<'s, 'h>),
    InterfaceCallH(&'h InterfaceCallH<'s, 'h>),
    IfH(&'h IfH<'s, 'h>),
    WhileH(&'h WhileH<'s, 'h>),
    ConsecutorH(&'h ConsecutorH<'s, 'h>),
    BlockH(&'h BlockH<'s, 'h>),
    MutabilifyH(&'h MutabilifyH<'s, 'h>),
    ImmutabilifyH(&'h ImmutabilifyH<'s, 'h>),
    ReturnH(&'h ReturnH<'s, 'h>),
    NewRuntimeSizedArrayH(&'h NewRuntimeSizedArrayH<'s, 'h>),
    PushRuntimeSizedArrayH(&'h PushRuntimeSizedArrayH<'s, 'h>),
    PopRuntimeSizedArrayH(&'h PopRuntimeSizedArrayH<'s, 'h>),
    StaticArrayFromCallableH(&'h StaticArrayFromCallableH<'s, 'h>),
    DestroyStaticSizedArrayIntoFunctionH(&'h DestroyStaticSizedArrayIntoFunctionH<'s, 'h>),
    DestroyRuntimeSizedArrayH(&'h DestroyRuntimeSizedArrayH<'s, 'h>),
    BreakH(&'h BreakH),
    NewStructH(&'h NewStructH<'s, 'h>),
    ArrayLengthH(&'h ArrayLengthH<'s, 'h>),
    ArrayCapacityH(&'h ArrayCapacityH<'s, 'h>),
    BorrowToWeakH(&'h BorrowToWeakH<'s, 'h>),
    IsSameInstanceH(&'h IsSameInstanceH<'s, 'h>),
    AsSubtypeH(&'h AsSubtypeH<'s, 'h>),
    LockWeakH(&'h LockWeakH<'s, 'h>),
    DiscardH(&'h DiscardH<'s, 'h>),
    PreCheckBorrowH(&'h PreCheckBorrowH<'s, 'h>),
    CopyPrimH(&'h CopyPrimH<'s, 'h>),
}


impl<'s, 'h> ExpressionH<'s, 'h> where 's: 'h {
    pub fn result_type(&self) -> CoordH<'s, 'h> {
    match self {
        ExpressionH::ConstantVoidH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
        ExpressionH::ConstantIntH(c) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::IntHT(IntHT { bits: c.bits })),
        ExpressionH::ConstantBoolH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::BoolHT(BoolHT)),
        ExpressionH::ConstantStrH(_) => CoordH::new(OwnershipH::MutableShareH, LocationH::YonderH, KindHT::StrHT(StrHT)),
        ExpressionH::ConstantF64H(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::FloatHT(FloatHT)),
        ExpressionH::ArgumentH(a) => a.result_type,
        ExpressionH::StackifyH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
        ExpressionH::RestackifyH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
        ExpressionH::UnstackifyH(u) => u.local.type_h,
        ExpressionH::DestroyH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
        ExpressionH::DestroyStaticSizedArrayIntoLocalsH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
        ExpressionH::StructToInterfaceUpcastH(u) => {
            let src = u.source_expression.result_type();
            CoordH::new(src.ownership, src.location, KindHT::InterfaceHT(u.target_interface))
        }
        ExpressionH::InterfaceToInterfaceUpcastH(_) => {
            panic!("Unimplemented: result_type for InterfaceToInterfaceUpcastH");
            // CoordH(sourceExpression.resultType.ownership, sourceExpression.resultType.location, targetInterface)
        }
        ExpressionH::LocalStoreH(s) => s.local.type_h,
        ExpressionH::LocalLoadH(l) => {
            let location = match (l.target_ownership, l.local.type_h.location) {
                (OwnershipH::ImmutableBorrowH, _) | (OwnershipH::MutableBorrowH, _) => LocationH::YonderH,
                (OwnershipH::WeakH, _) => LocationH::YonderH,
                (OwnershipH::OwnH, loc) => loc,
                (OwnershipH::MutableShareH, _) | (OwnershipH::ImmutableShareH, _) => LocationH::YonderH,
            };
            CoordH::new(l.target_ownership, location, l.local.type_h.kind)
        }
        ExpressionH::MemberStoreH(m) => m.result_type,
        ExpressionH::MemberLoadH(m) => m.result_type,
        ExpressionH::NewArrayFromValuesH(n) => n.result_type,
        ExpressionH::StaticSizedArrayStoreH(_) => {
            panic!("Unimplemented: result_type for StaticSizedArrayStoreH");
            // resultType (field)
        }
        ExpressionH::RuntimeSizedArrayStoreH(r) => r.result_type,
        ExpressionH::RuntimeSizedArrayLoadH(r) => r.result_type,
        ExpressionH::StaticSizedArrayLoadH(s) => s.result_type,
        ExpressionH::CallH(c) => c.function.return_type,
        ExpressionH::ExternCallH(c) => c.function.return_type,
        ExpressionH::InterfaceCallH(c) => c.function_type.return_type,
        ExpressionH::IfH(i) => i.common_supertype,
        ExpressionH::WhileH(w) => match w.body_block.result_type().kind {
            KindHT::VoidHT(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
            KindHT::NeverHT(NeverHT { from_break: true }) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
            KindHT::NeverHT(NeverHT { from_break: false }) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::NeverHT(NeverHT { from_break: false })),
            _ => panic!("WhileH::result_type: unexpected body_block kind"),
        },
        ExpressionH::ConsecutorH(c) => c.exprs.last().expect("ConsecutorH exprs nonEmpty").result_type(),
        ExpressionH::BlockH(b) => b.inner.result_type(),
        ExpressionH::MutabilifyH(m) => {
            let CoordH { ownership, location, kind, .. } = m.inner.result_type();
            CoordH::new(
                match ownership {
                    OwnershipH::OwnH => OwnershipH::OwnH,
                    OwnershipH::ImmutableBorrowH | OwnershipH::MutableBorrowH => OwnershipH::MutableBorrowH,
                    OwnershipH::ImmutableShareH | OwnershipH::MutableShareH => OwnershipH::MutableShareH,
                    OwnershipH::WeakH => {
                        panic!("MutabilifyH::result_type: WeakH unimplemented (vimpl)");
                        // vimpl()
                    }
                },
                location,
                kind,
            )
        }
        ExpressionH::ImmutabilifyH(im) => {
            let CoordH { ownership, location, kind, .. } = im.inner.result_type();
            CoordH::new(
                match ownership {
                    OwnershipH::OwnH => OwnershipH::OwnH,
                    OwnershipH::ImmutableBorrowH | OwnershipH::MutableBorrowH => OwnershipH::ImmutableBorrowH,
                    OwnershipH::ImmutableShareH | OwnershipH::MutableShareH => OwnershipH::ImmutableShareH,
                    OwnershipH::WeakH => {
                        panic!("ImmutabilifyH::result_type: WeakH unimplemented (vimpl)");
                        // vimpl()
                    }
                },
                location,
                kind,
            )
        }
        ExpressionH::ReturnH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::NeverHT(NeverHT { from_break: false })),
        ExpressionH::NewRuntimeSizedArrayH(n) => n.result_type,
        ExpressionH::PushRuntimeSizedArrayH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
        ExpressionH::PopRuntimeSizedArrayH(p) => p.element_type,
        ExpressionH::StaticArrayFromCallableH(s) => s.result_type,
        ExpressionH::DestroyStaticSizedArrayIntoFunctionH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
        ExpressionH::DestroyRuntimeSizedArrayH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
        ExpressionH::BreakH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::NeverHT(NeverHT { from_break: true })),
        ExpressionH::NewStructH(n) => n.result_type,
        ExpressionH::ArrayLengthH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::IntHT(IntHT { bits: 32 })),
        ExpressionH::ArrayCapacityH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::IntHT(IntHT { bits: 32 })),
        ExpressionH::BorrowToWeakH(b) => CoordH::new(OwnershipH::WeakH, LocationH::YonderH, b.ref_expression.result_type().kind),
        ExpressionH::IsSameInstanceH(x) => x.result_type(),
        ExpressionH::AsSubtypeH(a) => a.result_type,
        ExpressionH::LockWeakH(l) => l.result_type,
        ExpressionH::DiscardH(_) => CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::VoidHT(VoidHT)),
        ExpressionH::PreCheckBorrowH(_) => panic!("Unimplemented: result_type for PreCheckBorrowH"),
        ExpressionH::CopyPrimH(c) => c.result_type,
    }
    }


    pub fn expect_struct_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::StructHT(_) => *self,
            _ => panic!("expect_struct_access: not a struct"),
        }
    }


    pub fn expect_interface_access(&self) -> ExpressionH<'s, 'h> {
        panic!("Unimplemented: expect_interface_access");
        // resultType match { case CoordH(_, _, x @ InterfaceHT(_)) => this.asInstanceOf[ExpressionH[InterfaceHT]] }
    }


    pub fn expect_runtime_sized_array_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::RuntimeSizedArrayHT(_) => *self,
            _ => panic!("expect_runtime_sized_array_access: not a runtime sized array"),
        }
    }


    pub fn expect_static_sized_array_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::StaticSizedArrayHT(_) => *self,
            _ => panic!("expect_static_sized_array_access: not a static sized array"),
        }
    }


    pub fn expect_int_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::IntHT(_) => *self,
            _ => panic!("expect_int_access: not an int"),
        }
    }


    pub fn expect_i64_access(&self) -> ExpressionH<'s, 'h> {
        panic!("Unimplemented: expect_i64_access");
        // resultType match { case CoordH(_, _, x @ IntHT(64)) => this.asInstanceOf[ExpressionH[IntHT]] }
    }


    pub fn expect_bool_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::BoolHT(_) => *self,
            _ => panic!("expect_bool_access: not a bool"),
        }
    }
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantVoidH;








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantIntH {
    pub value: i64,
    pub bits: i32,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantBoolH {
    pub value: bool,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantStrH<'h> {
    pub value: &'h str,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantF64H {
    pub value: f64,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ArgumentH<'s, 'h> where 's: 'h {
    pub result_type: CoordH<'s, 'h>,
    pub argument_index: i32,
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct StackifyH<'s, 'h> where 's: 'h {
    pub source_expr: ExpressionH<'s, 'h>,
    pub local: Local<'s, 'h>,
    pub name: Option<&'h IdH<'s>>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct RestackifyH<'s, 'h> where 's: 'h {
    pub source_expr: ExpressionH<'s, 'h>,
    pub local: Local<'s, 'h>,
    pub name: Option<&'h IdH<'s>>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct UnstackifyH<'s, 'h> where 's: 'h {
    pub local: Local<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DestroyH<'s, 'h> where 's: 'h {
    pub struct_expression: ExpressionH<'s, 'h>,
    pub local_types: &'h [CoordH<'s, 'h>],
    pub local_indices: &'h [Local<'s, 'h>],
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsH<'s, 'h> where 's: 'h {
    pub struct_expression: ExpressionH<'s, 'h>,
    pub local_types: &'h [CoordH<'s, 'h>],
    pub local_indices: &'h [Local<'s, 'h>],
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct StructToInterfaceUpcastH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
    pub target_interface: &'h InterfaceHT<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct InterfaceToInterfaceUpcastH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
    pub target_interface: &'h InterfaceHT<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct LocalStoreH<'s, 'h> where 's: 'h {
    pub local: Local<'s, 'h>,
    pub source_expression: ExpressionH<'s, 'h>,
    pub local_name: &'h IdH<'s>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct LocalLoadH<'s, 'h> where 's: 'h {
    pub local: Local<'s, 'h>,
    pub target_ownership: OwnershipH,
    pub local_name: &'h IdH<'s>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct MemberStoreH<'s, 'h> where 's: 'h {
    pub result_type: CoordH<'s, 'h>,
    pub struct_expression: ExpressionH<'s, 'h>,
    pub member_index: i32,
    pub source_expression: ExpressionH<'s, 'h>,
    pub member_name: &'h IdH<'s>,
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct MemberLoadH<'s, 'h> where 's: 'h {
    pub struct_expression: ExpressionH<'s, 'h>,
    pub member_index: i32,
    pub expected_member_type: CoordH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
    pub member_name: &'h IdH<'s>,
}






/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct NewArrayFromValuesH<'s, 'h> where 's: 'h {
    pub result_type: CoordH<'s, 'h>,
    pub source_expressions: &'h [ExpressionH<'s, 'h>],
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct StaticSizedArrayStoreH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub index_expression: ExpressionH<'s, 'h>,
    pub source_expression: ExpressionH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}






/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayStoreH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub index_expression: ExpressionH<'s, 'h>,
    pub source_expression: ExpressionH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}






/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayLoadH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub index_expression: ExpressionH<'s, 'h>,
    pub target_ownership: OwnershipH,
    pub expected_element_type: CoordH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}






/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct StaticSizedArrayLoadH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub index_expression: ExpressionH<'s, 'h>,
    pub target_ownership: OwnershipH,
    pub expected_element_type: CoordH<'s, 'h>,
    pub array_size: i64,
    pub result_type: CoordH<'s, 'h>,
}






/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct CallH<'s, 'h> where 's: 'h {
    pub function: &'h PrototypeH<'s, 'h>,
    pub args_expressions: &'h [ExpressionH<'s, 'h>],
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ExternCallH<'s, 'h> where 's: 'h {
    pub function: &'h PrototypeH<'s, 'h>,
    pub args_expressions: &'h [ExpressionH<'s, 'h>],
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct InterfaceCallH<'s, 'h> where 's: 'h {
    pub args_expressions: &'h [ExpressionH<'s, 'h>],
    pub virtual_param_index: i32,
    pub interface_h: &'h InterfaceHT<'s, 'h>,
    pub index_in_edge: i32,
    pub function_type: &'h PrototypeH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct IfH<'s, 'h> where 's: 'h {
    pub condition_block: ExpressionH<'s, 'h>,
    pub then_block: ExpressionH<'s, 'h>,
    pub else_block: ExpressionH<'s, 'h>,
    pub common_supertype: CoordH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct WhileH<'s, 'h> where 's: 'h {
    pub body_block: ExpressionH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConsecutorH<'s, 'h> where 's: 'h {
    pub exprs: &'h [ExpressionH<'s, 'h>],
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct BlockH<'s, 'h> where 's: 'h {
    pub inner: ExpressionH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct MutabilifyH<'s, 'h> where 's: 'h {
    pub inner: ExpressionH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ImmutabilifyH<'s, 'h> where 's: 'h {
    pub inner: ExpressionH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ReturnH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct NewRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub capacity_expression: ExpressionH<'s, 'h>,
    pub element_type: CoordH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}






/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct PushRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub newcomer_expression: ExpressionH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct PopRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub element_type: CoordH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromCallableH<'s, 'h> where 's: 'h {
    pub generator_expression: ExpressionH<'s, 'h>,
    pub generator_method: &'h PrototypeH<'s, 'h>,
    pub element_type: CoordH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}






/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoFunctionH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub consumer_expression: ExpressionH<'s, 'h>,
    pub consumer_method: &'h PrototypeH<'s, 'h>,
    pub array_element_type: CoordH<'s, 'h>,
    pub array_size: i64,
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DestroyRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct BreakH;








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct NewStructH<'s, 'h> where 's: 'h {
    pub source_expressions: &'h [ExpressionH<'s, 'h>],
    pub target_member_names: &'h [&'h IdH<'s>],
    pub result_type: CoordH<'s, 'h>,
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ArrayLengthH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ArrayCapacityH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct BorrowToWeakH<'s, 'h> where 's: 'h {
    pub ref_expression: ExpressionH<'s, 'h>,
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct IsSameInstanceH<'s, 'h> where 's: 'h {
    pub left_expression: ExpressionH<'s, 'h>,
    pub right_expression: ExpressionH<'s, 'h>,
}
impl<'s, 'h> IsSameInstanceH<'s, 'h> where 's: 'h {
    pub fn result_type(&self) -> CoordH<'s, 'h> {
        CoordH::new(OwnershipH::OwnH, LocationH::InlineH, KindHT::BoolHT(BoolHT))
    }
    
}








/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct AsSubtypeH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
    pub target_type: KindHT<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
    pub some_constructor: &'h PrototypeH<'s, 'h>,
    pub none_constructor: &'h PrototypeH<'s, 'h>,
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct LockWeakH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
    pub some_constructor: &'h PrototypeH<'s, 'h>,
    pub none_constructor: &'h PrototypeH<'s, 'h>,
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DiscardH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
}


#[derive(Copy, Clone, Debug)]
pub struct CopyPrimH<'s, 'h> where 's: 'h {
    pub inner: ExpressionH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct PreCheckBorrowH<'s, 'h> where 's: 'h {
    pub inner_expression: ExpressionH<'s, 'h>,
}








/// Polyvalue
#[derive(Copy, Clone, Debug)]
pub enum IExpressionH<'s, 'h> where 's: 'h {
    ReferenceExpressionH(&'h ReferenceExpressionH<'s, 'h>),
    AddressExpressionH(&'h AddressExpressionH<'s, 'h>),
}


pub fn expect_reference_expression<'s, 'h>(this: &IExpressionH<'s, 'h>) -> &'h ReferenceExpressionH<'s, 'h> {
    panic!("Unimplemented: expect_reference_expression");
    // this match { case r @ ReferenceExpressionH(_) => r; case AddressExpressionH(_) => vfail(...) }
}


pub fn expect_address_expression<'s, 'h>(this: &IExpressionH<'s, 'h>) -> &'h AddressExpressionH<'s, 'h> {
    panic!("Unimplemented: expect_address_expression");
    // this match { case a @ AddressExpressionH(_) => a; case ReferenceExpressionH(_) => vfail(...) }
}


/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ReferenceExpressionH<'s, 'h> where 's: 'h {
    pub reference: CoordH<'s, 'h>,
}






/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct AddressExpressionH<'s, 'h> where 's: 'h {
    pub reference: CoordH<'s, 'h>,
}






/// Temporary state
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Local<'s, 'h> where 's: 'h {
    pub id: VariableIdH<'s, 'h>,
    pub type_h: CoordH<'s, 'h>,
}






/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VariableIdH<'s, 'h> where 's: 'h {
    pub number: i32,
    pub height: i32,
    pub name: Option<&'h IdH<'s>>,
}


