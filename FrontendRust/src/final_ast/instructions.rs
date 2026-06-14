// From Frontend/FinalAST/src/dev/vale/finalast/instructions.scala
//
// H-side instruction set: 50+ ExpressionH variants describing the lowered
// program. Mirrors src/instantiating/ast/expressions.rs pattern — enum
// dispatch with arena refs (no `dyn`). Per typing-pass parity, expression
// types opt out of `PartialEq`/`Hash` (Scala uses `vcurious`); just
// `Copy/Clone/Debug`.
//
// All variant payload structs are bare-placeholder (PhantomData) for now.
// Body migration restores fields per Scala.

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

/*
package dev.vale.finalast

import dev.vale.{vassert, vcurious, vfail, vimpl, vpass, vwat}

// Common trait for all instructions.
*/
// mig: enum ExpressionH
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
    NewImmRuntimeSizedArrayH(&'h NewImmRuntimeSizedArrayH<'s, 'h>),
    NewMutRuntimeSizedArrayH(&'h NewMutRuntimeSizedArrayH<'s, 'h>),
    PushRuntimeSizedArrayH(&'h PushRuntimeSizedArrayH<'s, 'h>),
    PopRuntimeSizedArrayH(&'h PopRuntimeSizedArrayH<'s, 'h>),
    StaticArrayFromCallableH(&'h StaticArrayFromCallableH<'s, 'h>),
    DestroyStaticSizedArrayIntoFunctionH(&'h DestroyStaticSizedArrayIntoFunctionH<'s, 'h>),
    DestroyImmRuntimeSizedArrayH(&'h DestroyImmRuntimeSizedArrayH<'s, 'h>),
    DestroyMutRuntimeSizedArrayH(&'h DestroyMutRuntimeSizedArrayH<'s, 'h>),
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
}
/*
sealed trait ExpressionH[+T <: KindHT] {
*/
// mig: fn result_type
impl<'s, 'h> ExpressionH<'s, 'h> where 's: 'h {
    pub fn result_type(&self) -> CoordH<'s, 'h> {
    match self {
        ExpressionH::ConstantVoidH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
        ExpressionH::ConstantIntH(c) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::IntHT(IntHT { bits: c.bits }) },
        ExpressionH::ConstantBoolH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::BoolHT(BoolHT) },
        ExpressionH::ConstantStrH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::YonderH, kind: KindHT::StrHT(StrHT) },
        ExpressionH::ConstantF64H(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::FloatHT(FloatHT) },
        ExpressionH::ArgumentH(a) => a.result_type,
        ExpressionH::StackifyH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
        ExpressionH::RestackifyH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
        ExpressionH::UnstackifyH(u) => u.local.type_h,
        ExpressionH::DestroyH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
        ExpressionH::DestroyStaticSizedArrayIntoLocalsH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
        ExpressionH::StructToInterfaceUpcastH(u) => {
            let src = u.source_expression.result_type();
            CoordH { ownership: src.ownership, location: src.location, kind: KindHT::InterfaceHT(u.target_interface) }
        }
        ExpressionH::InterfaceToInterfaceUpcastH(_) => panic!("Unimplemented: result_type for InterfaceToInterfaceUpcastH"),
        ExpressionH::LocalStoreH(s) => s.local.type_h,
        ExpressionH::LocalLoadH(l) => {
            let location = match (l.target_ownership, l.local.type_h.location) {
                (OwnershipH::ImmutableBorrowH, _) | (OwnershipH::MutableBorrowH, _) => LocationH::YonderH,
                (OwnershipH::WeakH, _) => LocationH::YonderH,
                (OwnershipH::OwnH, loc) => loc,
                (OwnershipH::MutableShareH, loc) | (OwnershipH::ImmutableShareH, loc) => loc,
            };
            CoordH { ownership: l.target_ownership, location, kind: l.local.type_h.kind }
        }
        ExpressionH::MemberStoreH(m) => m.result_type,
        ExpressionH::MemberLoadH(m) => m.result_type,
        ExpressionH::NewArrayFromValuesH(n) => n.result_type,
        ExpressionH::StaticSizedArrayStoreH(_) => panic!("Unimplemented: result_type for StaticSizedArrayStoreH"),
        ExpressionH::RuntimeSizedArrayStoreH(r) => r.result_type,
        ExpressionH::RuntimeSizedArrayLoadH(r) => r.result_type,
        ExpressionH::StaticSizedArrayLoadH(s) => s.result_type,
        ExpressionH::CallH(c) => c.function.return_type,
        ExpressionH::ExternCallH(c) => c.function.return_type,
        ExpressionH::InterfaceCallH(c) => c.function_type.return_type,
        ExpressionH::IfH(i) => i.common_supertype,
        ExpressionH::WhileH(w) => match w.body_block.result_type().kind {
            KindHT::VoidHT(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
            KindHT::NeverHT(NeverHT { from_break: true }) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
            KindHT::NeverHT(NeverHT { from_break: false }) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::NeverHT(NeverHT { from_break: false }) },
            _ => panic!("WhileH::result_type: unexpected body_block kind"),
        },
        ExpressionH::ConsecutorH(c) => c.exprs.last().expect("ConsecutorH exprs nonEmpty").result_type(),
        ExpressionH::BlockH(b) => b.inner.result_type(),
        ExpressionH::MutabilifyH(m) => {
            let CoordH { ownership, location, kind } = m.inner.result_type();
            CoordH {
                ownership: match ownership {
                    OwnershipH::OwnH => OwnershipH::OwnH,
                    OwnershipH::ImmutableBorrowH | OwnershipH::MutableBorrowH => OwnershipH::MutableBorrowH,
                    OwnershipH::ImmutableShareH | OwnershipH::MutableShareH => OwnershipH::MutableShareH,
                    OwnershipH::WeakH => panic!("MutabilifyH::result_type: WeakH unimplemented (vimpl)"),
                },
                location,
                kind,
            }
        }
        ExpressionH::ImmutabilifyH(im) => {
            let CoordH { ownership, location, kind } = im.inner.result_type();
            CoordH {
                ownership: match ownership {
                    OwnershipH::OwnH => OwnershipH::OwnH,
                    OwnershipH::ImmutableBorrowH | OwnershipH::MutableBorrowH => OwnershipH::ImmutableBorrowH,
                    OwnershipH::ImmutableShareH | OwnershipH::MutableShareH => OwnershipH::ImmutableShareH,
                    OwnershipH::WeakH => panic!("ImmutabilifyH::result_type: WeakH unimplemented (vimpl)"),
                },
                location,
                kind,
            }
        }
        ExpressionH::ReturnH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::NeverHT(NeverHT { from_break: false }) },
        ExpressionH::NewImmRuntimeSizedArrayH(n) => n.result_type,
        ExpressionH::NewMutRuntimeSizedArrayH(n) => n.result_type,
        ExpressionH::PushRuntimeSizedArrayH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
        ExpressionH::PopRuntimeSizedArrayH(p) => p.element_type,
        ExpressionH::StaticArrayFromCallableH(s) => s.result_type,
        ExpressionH::DestroyStaticSizedArrayIntoFunctionH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
        ExpressionH::DestroyImmRuntimeSizedArrayH(_) => panic!("Unimplemented: result_type for DestroyImmRuntimeSizedArrayH"),
        ExpressionH::DestroyMutRuntimeSizedArrayH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
        ExpressionH::BreakH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::NeverHT(NeverHT { from_break: true }) },
        ExpressionH::NewStructH(n) => n.result_type,
        ExpressionH::ArrayLengthH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::IntHT(IntHT { bits: 32 }) },
        ExpressionH::ArrayCapacityH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::IntHT(IntHT { bits: 32 }) },
        ExpressionH::BorrowToWeakH(b) => CoordH { ownership: OwnershipH::WeakH, location: LocationH::YonderH, kind: b.ref_expression.result_type().kind },
        ExpressionH::IsSameInstanceH(x) => x.result_type(),
        ExpressionH::AsSubtypeH(a) => a.result_type,
        ExpressionH::LockWeakH(l) => l.result_type,
        ExpressionH::DiscardH(_) => CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::VoidHT(VoidHT) },
        ExpressionH::PreCheckBorrowH(_) => panic!("Unimplemented: result_type for PreCheckBorrowH"),
    }
    }
/*
  def resultType: CoordH[T]

  // Convenience functions for accessing this expression as the kind returning
  // a certain type.
*/
// mig: fn expect_struct_access
    pub fn expect_struct_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::StructHT(_) => *self,
            _ => panic!("expect_struct_access: not a struct"),
        }
    }
/*
  def expectStructAccess(): ExpressionH[StructHT] = {
    resultType match {
      case CoordH(_, _, x @ StructHT(_)) => {
        this.asInstanceOf[ExpressionH[StructHT]]
      }
      case _ => vfail()
    }
  }
*/
// mig: fn expect_interface_access
    pub fn expect_interface_access(&self) -> ExpressionH<'s, 'h> {
        panic!("Unimplemented: expect_interface_access");
    }
/*
  def expectInterfaceAccess(): ExpressionH[InterfaceHT] = {
    resultType match {
      case CoordH(_, _, x @ InterfaceHT(_)) => {
        this.asInstanceOf[ExpressionH[InterfaceHT]]
      }
    }
  }
*/
// mig: fn expect_runtime_sized_array_access
    pub fn expect_runtime_sized_array_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::RuntimeSizedArrayHT(_) => *self,
            _ => panic!("expect_runtime_sized_array_access: not a runtime sized array"),
        }
    }
/*
  def expectRuntimeSizedArrayAccess(): ExpressionH[RuntimeSizedArrayHT] = {
    resultType match {
      case CoordH(_, _, x @ RuntimeSizedArrayHT(_)) => {
        this.asInstanceOf[ExpressionH[RuntimeSizedArrayHT]]
      }
    }
  }
*/
// mig: fn expect_static_sized_array_access
    pub fn expect_static_sized_array_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::StaticSizedArrayHT(_) => *self,
            _ => panic!("expect_static_sized_array_access: not a static sized array"),
        }
    }
/*
  def expectStaticSizedArrayAccess(): ExpressionH[StaticSizedArrayHT] = {
    resultType match {
      case CoordH(_, _, x @ StaticSizedArrayHT(_)) => {
        this.asInstanceOf[ExpressionH[StaticSizedArrayHT]]
      }
    }
  }
*/
// mig: fn expect_int_access
    pub fn expect_int_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::IntHT(_) => *self,
            _ => panic!("expect_int_access: not an int"),
        }
    }
/*
  def expectIntAccess(): ExpressionH[IntHT] = {
    resultType match {
      case CoordH(_, _, x @ IntHT(32)) => {
        this.asInstanceOf[ExpressionH[IntHT]]
      }
    }
  }
*/
// mig: fn expect_i64_access
    pub fn expect_i64_access(&self) -> ExpressionH<'s, 'h> {
        panic!("Unimplemented: expect_i64_access");
    }
/*
  def expectI64Access(): ExpressionH[IntHT] = {
    resultType match {
      case CoordH(_, _, x @ IntHT(64)) => {
        this.asInstanceOf[ExpressionH[IntHT]]
      }
    }
  }
*/
// mig: fn expect_bool_access
    pub fn expect_bool_access(&self) -> ExpressionH<'s, 'h> {
        match self.result_type().kind {
            KindHT::BoolHT(_) => *self,
            _ => panic!("expect_bool_access: not a bool"),
        }
    }
}
/*
  def expectBoolAccess(): ExpressionH[BoolHT] = {
    resultType match {
      case CoordH(_, _, x @ BoolHT()) => {
        this.asInstanceOf[ExpressionH[BoolHT]]
      }
    }
  }
}

// Produces a void.
*/
// mig: struct ConstantVoidH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantVoidH;
/*
case class ConstantVoidH() extends ExpressionH[VoidHT] {
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
override def hashCode(): Int = 1337
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[VoidHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

// Produces an integer.
*/
// mig: struct ConstantIntH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantIntH {
    pub value: i64,
    pub bits: i32,
}
/*
case class ConstantIntH(
  // The value of the integer.
  value: Long,
  bits: Int
) extends ExpressionH[IntHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[IntHT] = CoordH(MutableShareH, InlineH, IntHT(bits))
}

// Produces a boolean.
*/
// mig: struct ConstantBoolH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantBoolH {
    pub value: bool,
}
/*
case class ConstantBoolH(
  // The value of the boolean.
  value: Boolean
) extends ExpressionH[BoolHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[BoolHT] = CoordH(MutableShareH, InlineH, BoolHT())
}

// Produces a string.
*/
// mig: struct ConstantStrH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantStrH<'h> {
    pub value: &'h str,
}
/*
case class ConstantStrH(
  // The value of the string.
  value: String
) extends ExpressionH[StrHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[StrHT] = CoordH(MutableShareH, YonderH, StrHT())
}

// Produces a float.
*/
// mig: struct ConstantF64H
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConstantF64H {
    pub value: f64,
}
/*
case class ConstantF64H(
  // The value of the float.
  value: Double
) extends ExpressionH[FloatHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[FloatHT] = CoordH(MutableShareH, InlineH, FloatHT())
}

// Produces the value from an argument.
// There can only be one of these per argument; this conceptually destroys
// the containing argument and produces its value.
*/
// mig: struct ArgumentH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ArgumentH<'s, 'h> where 's: 'h {
    pub result_type: CoordH<'s, 'h>,
    pub argument_index: i32,
}
/*
case class ArgumentH(
  resultType: CoordH[KindHT],
  // The index of the argument, starting at 0.
  argumentIndex: Int
) extends ExpressionH[KindHT]

// Takes a value from the source expression and puts it into a local
// variable on the stack.
*/
// mig: struct StackifyH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct StackifyH<'s, 'h> where 's: 'h {
    pub source_expr: ExpressionH<'s, 'h>,
    pub local: Local<'s, 'h>,
    pub name: Option<&'h IdH<'s>>,
}
/*
case class StackifyH(
  // The expressions to read a value from.
  sourceExpr: ExpressionH[KindHT],
  // Describes the local we're making.
  local: Local,
  // Name of the local variable. Used for debugging.
  name: Option[IdH]
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();

  // See BRCOBS, source shouldn't be Never.
  sourceExpr.resultType.kind match { case NeverHT(_) => vwat() case _ => }
  vassert(sourceExpr.resultType == local.typeH)

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[VoidHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

// Takes a value from the source expression and puts it into a local
// variable on the stack.
*/
// mig: struct RestackifyH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct RestackifyH<'s, 'h> where 's: 'h {
    pub source_expr: ExpressionH<'s, 'h>,
    pub local: Local<'s, 'h>,
    pub name: Option<&'h IdH<'s>>,
}
/*
case class RestackifyH(
  // The expressions to read a value from.
  sourceExpr: ExpressionH[KindHT],
  // Describes the local we're making.
  local: Local,
  // Name of the local variable. Used for debugging.
  name: Option[IdH]
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();

  // See BRCOBS, source shouldn't be Never.
  sourceExpr.resultType.kind match { case NeverHT(_) => vwat() case _ => }
  vassert(sourceExpr.resultType == local.typeH)

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[VoidHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

// Takes a value from a local variable on the stack, and produces it.
// The local variable is now invalid, since its value has been taken out.
// See LocalLoadH for a similar instruction that *doesnt* invalidate the local var.
*/
// mig: struct UnstackifyH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct UnstackifyH<'s, 'h> where 's: 'h {
    pub local: Local<'s, 'h>,
}
/*
case class UnstackifyH(
  // Describes the local we're pulling from. This is equal to the corresponding
  // StackifyH's `local` member.
  local: Local
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  // Panics if this is ever not the case.
  vcurious(local.typeH == resultType)

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = local.typeH
}

// Takes a struct from the given expressions, and destroys it.
// All of its members are saved from the jaws of death, and put into the specified
// local variables.
// This creates those local variables, much as a StackifyH would, and puts into them
// the values from the dying struct instance.
*/
// mig: struct DestroyH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DestroyH<'s, 'h> where 's: 'h {
    pub struct_expression: ExpressionH<'s, 'h>,
    pub local_types: &'h [CoordH<'s, 'h>],
    pub local_indices: &'h [Local<'s, 'h>],
}
/*
case class DestroyH(
  // The expressions to take the struct from.
  structExpression: ExpressionH[StructHT],
  // A list of types, one for each local variable we'll make.
  // TODO: If the vcurious below doesn't panic, get rid of this redundant member.
  localTypes: Vector[CoordH[KindHT]],
  // The locals to put the struct's members into.
  localIndices: Vector[Local],
) extends ExpressionH[VoidHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  vassert(localTypes.size == localIndices.size)
  vcurious(localTypes == localIndices.map(_.typeH).toVector)

  // See BRCOBS, source shouldn't be Never.
  // Nevermind, type system guarantees it
  // structExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[VoidHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

// Takes a struct from the given expressions, and destroys it.
// All of its members are saved from the jaws of death, and put into the specified
// local variables.
// This creates those local variables, much as a StackifyH would, and puts into them
// the values from the dying struct instance.
*/
// mig: struct DestroyStaticSizedArrayIntoLocalsH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsH<'s, 'h> where 's: 'h {
    pub struct_expression: ExpressionH<'s, 'h>,
    pub local_types: &'h [CoordH<'s, 'h>],
    pub local_indices: &'h [Local<'s, 'h>],
}
/*
case class DestroyStaticSizedArrayIntoLocalsH(
  // The expressions to take the struct from.
  structExpression: ExpressionH[StaticSizedArrayHT],
  // A list of types, one for each local variable we'll make.
  // TODO: If the vcurious below doesn't panic, get rid of this redundant member.
  localTypes: Vector[CoordH[KindHT]],
  // The locals to put the struct's members into.
  localIndices: Vector[Local]
) extends ExpressionH[VoidHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  vassert(localTypes.size == localIndices.size)
  vcurious(localTypes == localIndices.map(_.typeH).toVector)

  // See BRCOBS, source shouldn't be Never.
  // Nevermind, type system guarantees it
  // structExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[VoidHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

// Takes a struct reference from the "source" expressions, and makes an interface reference
// to it, as the "target" reference, and puts it into another expressions.
*/
// mig: struct StructToInterfaceUpcastH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct StructToInterfaceUpcastH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
    pub target_interface: &'h InterfaceHT<'s, 'h>,
}
/*
case class StructToInterfaceUpcastH(
  // The expressions to get the struct reference from.
  sourceExpression: ExpressionH[StructHT],
  // The type of interface to cast to.
  targetInterface: InterfaceHT
) extends ExpressionH[InterfaceHT] {

  // See BRCOBS, source shouldn't be Never.
  // Nevermind, type system guarantees it
  //  sourceExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  // The resulting type will have the same ownership as the source expressions had.
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  def resultType = CoordH(sourceExpression.resultType.ownership, sourceExpression.resultType.location, targetInterface)
}

// Takes an interface reference from the "source" expressions, and makes another reference
// to it, as the "target" inference, and puts it into another expressions.
*/
// mig: struct InterfaceToInterfaceUpcastH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct InterfaceToInterfaceUpcastH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
    pub target_interface: &'h InterfaceHT<'s, 'h>,
}
/*
case class InterfaceToInterfaceUpcastH(
  // The expressions to get the source interface reference from.
  sourceExpression: ExpressionH[InterfaceHT],
  // The type of interface to cast to.
  targetInterface: InterfaceHT
) extends ExpressionH[InterfaceHT] {

  // See BRCOBS, source shouldn't be Never.
  // Nevermind, type system guarantees it
  // sourceExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  // The resulting type will have the same ownership as the source expressions had.
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  def resultType = CoordH(sourceExpression.resultType.ownership, sourceExpression.resultType.location, targetInterface)
}

// Takes a reference from the given "source" expressions, and puts it into an *existing*
// local variable.
*/
// mig: struct LocalStoreH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct LocalStoreH<'s, 'h> where 's: 'h {
    pub local: Local<'s, 'h>,
    pub source_expression: ExpressionH<'s, 'h>,
    pub local_name: &'h IdH<'s>,
}
/*
case class LocalStoreH(
  // The existing local to store into.
  local: Local,
  // The expressions to get the source reference from.
  sourceExpression: ExpressionH[KindHT],
  // Name of the local variable, for debug purposes.
  localName: IdH
) extends ExpressionH[KindHT] {

  // See BRCOBS, source shouldn't be Never.
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = local.typeH
}

// Takes a reference from the given local variable, and puts it into a new expressions.
// This can never move a reference, only alias it. The instruction which can move a
// reference is UnstackifyH.
*/
// mig: struct LocalLoadH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct LocalLoadH<'s, 'h> where 's: 'h {
    pub local: Local<'s, 'h>,
    pub target_ownership: OwnershipH,
    pub local_name: &'h IdH<'s>,
}
/*
case class LocalLoadH(
  // The existing local to load from.
  local: Local,
  // The ownership of the resulting reference. This doesn't have to
  // match the ownership of the source reference. For example, we might want
  // to load a borrow reference from an owning local.
  targetOwnership: OwnershipH,
  // Name of the local variable, for debug purposes.
  localName: IdH
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  vassert(targetOwnership != OwnH) // must unstackify to get an owning reference

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = {
    val location =
      (targetOwnership, local.typeH.location) match {
        case (ImmutableBorrowH | MutableBorrowH, _) => YonderH
        case (WeakH, _) => YonderH
        case (OwnH, location) => location
        case (MutableShareH | ImmutableShareH, location) => location
      }
    CoordH(targetOwnership, location, local.typeH.kind)
  }
}

// Takes a reference from the given "source" expressions, and swaps it into the given
// struct's member. The member's old reference is put into a new expressions.
*/
// mig: struct MemberStoreH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct MemberStoreH<'s, 'h> where 's: 'h {
    pub result_type: CoordH<'s, 'h>,
    pub struct_expression: ExpressionH<'s, 'h>,
    pub member_index: i32,
    pub source_expression: ExpressionH<'s, 'h>,
    pub member_name: &'h IdH<'s>,
}
/*
case class MemberStoreH(
  resultType: CoordH[KindHT],
  // Expression containing a reference to the struct whose member we will swap.
  structExpression: ExpressionH[StructHT],
  // Which member to swap out, starting at 0.
  memberIndex: Int,
  // Expression containing the new value for the struct's member.
  sourceExpression: ExpressionH[KindHT],
  // Name of the member, for debug purposes.
  memberName: IdH
) extends ExpressionH[KindHT] {

  // See BRCOBS, struct shouldn't be Never.
  // Nevermind, type system guarantees it
  //  structExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  // See BRCOBS, source shouldn't be Never.
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

}

// Takes a reference from the given "struct" expressions, and copies it into a new
// expressions. This can never move a reference, only alias it.
*/
// mig: struct MemberLoadH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct MemberLoadH<'s, 'h> where 's: 'h {
    pub struct_expression: ExpressionH<'s, 'h>,
    pub member_index: i32,
    pub expected_member_type: CoordH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
    pub member_name: &'h IdH<'s>,
}
/*
case class MemberLoadH(
  // Expression containing a reference to the struct whose member we will read.
  structExpression: ExpressionH[StructHT],
  // Which member to read from, starting at 0.
  memberIndex: Int,
  // The type we expect the member to be. This can easily be looked up, but is provided
  // here to be convenient for LLVM.
  expectedMemberType: CoordH[KindHT],
  // The type of the resulting reference.
  resultType: CoordH[KindHT],
  // Member's name, for debug purposes.
  memberName: IdH
) extends ExpressionH[KindHT] {

  // See BRCOBS, source shouldn't be Never.
  // Nevermind, type system guarantees it
  //  structExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
//  vassert(resultType.ownership == targetOwnership)
//  vassert(resultType.permission == targetPermission)

  if (resultType.ownership == WeakH) vassert(resultType.location == YonderH)
}

// Produces an array whose size is fixed and known at compile time, and puts it into
// a expressions.
*/
// mig: struct NewArrayFromValuesH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct NewArrayFromValuesH<'s, 'h> where 's: 'h {
    pub result_type: CoordH<'s, 'h>,
    pub source_expressions: &'h [ExpressionH<'s, 'h>],
}
/*
case class NewArrayFromValuesH(
  // The resulting type of the array.
  // TODO: See if we can infer this from the types in the expressions.
  resultType: CoordH[StaticSizedArrayHT],
  // The expressions from which we'll get the values to put into the array.
  sourceExpressions: Vector[ExpressionH[KindHT]]
) extends ExpressionH[StaticSizedArrayHT] {

  // See BRCOBS, source shouldn't be Never.
  sourceExpressions.foreach(expr => {
    expr.resultType.kind match { case NeverHT(_) => vwat() case _ => }
  })
}

// Loads from the "source" expressions and swaps it into the array from arrayExpression at
// the position specified by the integer in indexExpression. The old value from the
// array is moved out into expressionsId.
// This is for the kind of array whose size we know at compile time, the kind that
// doesn't need to carry around a size. For the corresponding instruction for the
// unknown-size-at-compile-time array, see RuntimeSizedArrayStoreH.
*/
// mig: struct StaticSizedArrayStoreH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct StaticSizedArrayStoreH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub index_expression: ExpressionH<'s, 'h>,
    pub source_expression: ExpressionH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}
/*
case class StaticSizedArrayStoreH(
  // Expression containing the array whose element we'll swap out.
  arrayExpression: ExpressionH[StaticSizedArrayHT],
  // Expression containing the index of the element we'll swap out.
  indexExpression: ExpressionH[IntHT],
  // Expression containing the value we'll swap into the array.
  sourceExpression: ExpressionH[KindHT],
  resultType: CoordH[KindHT],
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  vassert(indexExpression.resultType.kind == IntHT.i32)

  // See BRCOBS, source shouldn't be Never.
  // Nevermind, type system guarantees it
  //  arrayExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  //  indexExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }
}

// Loads from the "source" expressions and swaps it into the array from arrayExpression at
// the position specified by the integer in indexExpression. The old value from the
// array is moved out into expressionsId.
// This is for the kind of array whose size we don't know at compile time, the kind
// that needs to carry around a size. For the corresponding instruction for the
// known-size-at-compile-time array, see StaticSizedArrayStoreH.
*/
// mig: struct RuntimeSizedArrayStoreH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayStoreH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub index_expression: ExpressionH<'s, 'h>,
    pub source_expression: ExpressionH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}
/*
case class RuntimeSizedArrayStoreH(
  // Expression containing the array whose element we'll swap out.
  arrayExpression: ExpressionH[RuntimeSizedArrayHT],
  // Expression containing the index of the element we'll swap out.
  indexExpression: ExpressionH[IntHT],
  // Expression containing the value we'll swap into the array.
  sourceExpression: ExpressionH[KindHT],
  resultType: CoordH[KindHT],
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  vassert(indexExpression.resultType.kind == IntHT.i32)

  // See BRCOBS, source shouldn't be Never.
  // Nevermind, type system guarantees it
  //  arrayExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  //  indexExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }
}

// Loads from the array in arrayExpression at the index in indexExpression, and stores
// the result in expressionsId. This can never move a reference, only alias it.
// This is for the kind of array whose size we don't know at compile time, the kind
// that needs to carry around a size. For the corresponding instruction for the
// known-size-at-compile-time array, see StaticSizedArrayLoadH.
*/
// mig: struct RuntimeSizedArrayLoadH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayLoadH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub index_expression: ExpressionH<'s, 'h>,
    pub target_ownership: OwnershipH,
    pub expected_element_type: CoordH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}
/*
case class RuntimeSizedArrayLoadH(
  // Expression containing the array whose element we'll read.
  arrayExpression: ExpressionH[RuntimeSizedArrayHT],
  // Expression containing the index of the element we'll read.
  indexExpression: ExpressionH[IntHT],
  // The ownership to load as. For example, we might load a borrow reference from a
  // owning Car reference element.
  targetOwnership: OwnershipH,
  expectedElementType: CoordH[KindHT],
  resultType: CoordH[KindHT],
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  vassert(indexExpression.resultType.kind == IntHT.i32)

  // See BRCOBS, source shouldn't be Never.
  // Nevermind, type system guarantees it
  //arrayExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  //indexExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
}

// Loads from the array in arrayExpression at the index in indexExpression, and stores
// the result in expressionsId. This can never move a reference, only alias it.
// This is for the kind of array whose size we know at compile time, the kind that
// doesn't need to carry around a size. For the corresponding instruction for the
// known-size-at-compile-time array, see StaticSizedArrayStoreH.
*/
// mig: struct StaticSizedArrayLoadH
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
/*
case class StaticSizedArrayLoadH(
  // Expression containing the array whose element we'll read.
  arrayExpression: ExpressionH[StaticSizedArrayHT],
  // Expression containing the index of the element we'll read.
  indexExpression: ExpressionH[IntHT],
  // The ownership to load as. For example, we might load a borrow reference from a
  // owning Car reference element.
  targetOwnership: OwnershipH,
  expectedElementType: CoordH[KindHT],
  arraySize: Long,
  resultType: CoordH[KindHT],
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  vassert(indexExpression.resultType.kind == IntHT.i32)

  // See BRCOBS, source shouldn't be Never.
  // Nevermind, type system guarantees it
  //  arrayExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  //  indexExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
}

// Calls a function.
*/
// mig: struct CallH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct CallH<'s, 'h> where 's: 'h {
    pub function: &'h PrototypeH<'s, 'h>,
    pub args_expressions: &'h [ExpressionH<'s, 'h>],
}
/*
case class CallH(
  // Identifies which function to call.
  function: PrototypeH,
  // Expressions containing the arguments to pass to the function.
  argsExpressions: Vector[ExpressionH[KindHT]]
) extends ExpressionH[KindHT] {
  // See BRCOBS, no arguments should be Never.
  argsExpressions.foreach(expr => {
    expr.resultType.kind match { case NeverHT(_) => vwat() case _ => }
  })

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = function.returnType
}

// Calls a function defined in some other module.
*/
// mig: struct ExternCallH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ExternCallH<'s, 'h> where 's: 'h {
    pub function: &'h PrototypeH<'s, 'h>,
    pub args_expressions: &'h [ExpressionH<'s, 'h>],
}
/*
case class ExternCallH(
  // Identifies which function to call.
  function: PrototypeH,
  // Expressions containing the arguments to pass to the function.
  argsExpressions: Vector[ExpressionH[KindHT]]
) extends ExpressionH[KindHT] {
  vpass()

  // See BRCOBS, no arguments should be Never.
  argsExpressions.foreach(expr => {
    expr.resultType.kind match { case NeverHT(_) => vwat() case _ => }
  })

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = function.returnType
}

// Calls a function on an interface.
*/
// mig: struct InterfaceCallH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct InterfaceCallH<'s, 'h> where 's: 'h {
    pub args_expressions: &'h [ExpressionH<'s, 'h>],
    pub virtual_param_index: i32,
    pub interface_h: &'h InterfaceHT<'s, 'h>,
    pub index_in_edge: i32,
    pub function_type: &'h PrototypeH<'s, 'h>,
}
/*
case class InterfaceCallH(
  // Expressions containing the arguments to pass to the function.
  argsExpressions: Vector[ExpressionH[KindHT]],
  // Which parameter has the interface whose table we'll read to get the function.
  virtualParamIndex: Int,
  // The type of the interface.
  // TODO: Take this out, it's redundant, can get it from argsExpressions[virtualParamIndex]
  interfaceH: InterfaceHT,
  // The index in the vtable for the function.
  indexInEdge: Int,
  // The function we expect to be calling. Note that this is the prototype for the abstract
  // function, not the prototype for the function that will eventually be called. The
  // difference is that this prototype will have an interface at the virtualParamIndex'th
  // parameter, and the function that is eventually called will have a struct there.
  functionType: PrototypeH
) extends ExpressionH[KindHT] {
  // See BRCOBS, no arguments should be Never.
  argsExpressions.foreach(expr => {
    expr.resultType.kind match { case NeverHT(_) => vwat() case _ => }
  })

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = functionType.returnType
  vassert(indexInEdge >= 0)
}

// An if-statement. It will get a boolean from running conditionBlock, and use it to either
// call thenBlock or elseBlock. The result of the thenBlock or elseBlock will be put into
// expressionsId.
*/
// mig: struct IfH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct IfH<'s, 'h> where 's: 'h {
    pub condition_block: ExpressionH<'s, 'h>,
    pub then_block: ExpressionH<'s, 'h>,
    pub else_block: ExpressionH<'s, 'h>,
    pub common_supertype: CoordH<'s, 'h>,
}
/*
case class IfH(
  // The block for the condition. If this results in a true, we'll run thenBlock, otherwise
  // we'll run elseBlock.
  conditionBlock: ExpressionH[BoolHT],
  // The block to run if conditionBlock results in a true. The result of this block will be
  // put into expressionsId.
  thenBlock: ExpressionH[KindHT],
  // The block to run if conditionBlock results in a false. The result of this block will be
  // put into expressionsId.
  elseBlock: ExpressionH[KindHT],

  commonSupertype: CoordH[KindHT],
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  (thenBlock.resultType.kind, elseBlock.resultType.kind) match {
    case (NeverHT(false), _) =>
    case (_, NeverHT(false)) =>
    case (NeverHT(true), _) =>
    case (_, NeverHT(true)) =>
    case (a, b) if a == b =>
    case _ => vwat()
  }
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = commonSupertype
}

// A while loop. Continuously runs bodyBlock until it returns false.
*/
// mig: struct WhileH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct WhileH<'s, 'h> where 's: 'h {
    pub body_block: ExpressionH<'s, 'h>,
}
/*
case class WhileH(
  // The block to run until it returns false.
  bodyBlock: ExpressionH[KindHT]
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();

  val resultCoord =
    bodyBlock.resultType.kind match {
      case VoidHT() => CoordH(MutableShareH, InlineH, VoidHT())
      case NeverHT(true) => CoordH(MutableShareH, InlineH, VoidHT())
      case NeverHT(false) => CoordH(MutableShareH, InlineH, NeverHT(false))
      case _ => vwat()
    }

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = resultCoord
}

// A collection of instructions. The last one will be used as the block's result.
*/
// mig: struct ConsecutorH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ConsecutorH<'s, 'h> where 's: 'h {
    pub exprs: &'h [ExpressionH<'s, 'h>],
}
/*
case class ConsecutorH(
  // The instructions to run.
  exprs: Vector[ExpressionH[KindHT]],
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  // We should simplify these away
  vassert(exprs.nonEmpty)

  exprs.init.foreach(nonLastResultLine => {
    // The init ones should never just be VoidLiteralHs, those should be stripped out.
    // Use Hammer.consecutive to conform to this.
    nonLastResultLine match {
      case NewStructH(Vector(), Vector(), CoordH(_, InlineH, _)) => vfail("Should be no creating voids in the middle of a consecutor!")
      case _ =>
    }

    // The init ones should always return void structs.
    // If there's a never somewhere in there, then there should be nothing afterward.
    // Use Hammer.consecutive to conform to this.
    vassert(nonLastResultLine.resultType == CoordH(MutableShareH, InlineH, VoidHT()))
  })

  val indexOfFirstNever =
    exprs.map(_.resultType.kind).indexWhere({ case NeverHT(_) => true case _ => false })
  if (indexOfFirstNever >= 0) {
    // The first never should be the last line. There shouldn't be anything after never.
    if (indexOfFirstNever != exprs.size - 1) {
      vfail()
    }
  }

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = exprs.last.resultType
}

//// A collection of instructions to evaluate, knowing that we'll never
//// finish evaluating, we'll panic before then.
//// This is used when we have a source expression that's a never, such as:
////   someFunc(Ship(), panic())
//// We'll turn it into:
////   __consecrash(Ship(), panic())
//// This is different from consecutor in that:
//// 1. We'll make sure we crash at the end.
//// 2. We're allowed to leak all of these inner exprs, such as that Ship()
//case class ConsecutorH(
//  // The instructions to run.
//  exprs: Vector[ExpressionH[KindH]],
//) extends ExpressionH[KindH] {
//  val hash = runtime.ScalaRunTime._hashCode(this);
//override def hashCode(): Int = hash;
//override def equals(obj: Any): Boolean = vcurious();
//  // We should simplify these away
//  vassert(exprs.nonEmpty)
//
//  exprs.init.foreach(nonLastResultLine => {
//    // The init ones should never just be VoidLiteralHs, those should be stripped out.
//    // Use Hammer.consecutive to conform to this.
//    nonLastResultLine match {
//      case NewStructH(Vector(), Vector(), ReferenceH(_, InlineH, _, _)) => vfail("Should be no creating voids in the middle of a consecutor!")
//      case _ =>
//    }
//
//    // The init ones should always return void structs.
//    // If there's a never somewhere in there, then there should be nothing afterward.
//    // Use Hammer.consecutive to conform to this.
//    vassert(nonLastResultLine.resultType == ReferenceH(ShareH, InlineH, VoidH()))
//  })
//
//  val indexOfFirstNever = exprs.indexWhere(_.resultType.kind == NeverH())
//  if (indexOfFirstNever >= 0) {
//    // The first never should be the last line. There shouldn't be anything after never.
//    if (indexOfFirstNever != exprs.size - 1) {
//      vfail()
//    }
//  }
//
//  override def resultType: ReferenceH[KindH] = exprs.last.resultType
//}

// An expression where all locals declared inside will be destroyed by the time we exit.
*/
// mig: struct BlockH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct BlockH<'s, 'h> where 's: 'h {
    pub inner: ExpressionH<'s, 'h>,
}
/*
case class BlockH(
  // The instructions to run. This will probably be a consecutor.
  inner: ExpressionH[KindHT],
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = inner.resultType
}

// Casts an immutable reference to a mutable one.
*/
// mig: struct MutabilifyH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct MutabilifyH<'s, 'h> where 's: 'h {
    pub inner: ExpressionH<'s, 'h>,
}
/*
case class MutabilifyH(
  inner: ExpressionH[KindHT],
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = {
    val CoordH(ownership, location, kind) = inner.resultType
    CoordH(
      ownership match {
        case OwnH => OwnH
        case ImmutableBorrowH | MutableBorrowH => MutableBorrowH
        case ImmutableShareH | MutableShareH => MutableShareH
        case WeakH => vimpl()
      },
      location,
      kind)
  }
}

// Casts a mutable reference to an immutable one.
*/
// mig: struct ImmutabilifyH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ImmutabilifyH<'s, 'h> where 's: 'h {
    pub inner: ExpressionH<'s, 'h>,
}
/*
case class ImmutabilifyH(
  inner: ExpressionH[KindHT],
) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = {
    val CoordH(ownership, location, kind) = inner.resultType
    CoordH(
      ownership match {
        case OwnH => OwnH
        case ImmutableBorrowH | MutableBorrowH => ImmutableBorrowH
        case ImmutableShareH | MutableShareH => ImmutableShareH
        case WeakH => vimpl()
      },
      location,
      kind)
  }
}

// Ends the current function and returns a reference. A function will always end
// with a return statement.
*/
// mig: struct ReturnH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ReturnH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
}
/*
case class ReturnH(
  // The expressions to read from, whose value we'll return from the function.
  sourceExpression: ExpressionH[KindHT]
) extends ExpressionH[NeverHT] {
  // See BRCOBS, no arguments should be Never.
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[NeverHT] = CoordH(MutableShareH, InlineH, NeverHT(false))
}

// Constructs an immutable unknown-size array, whose length is the integer from sizeExpression,
// whose values are generated from the function from generatorExpression. Puts the
// result in a new expressions.
*/
// mig: struct NewImmRuntimeSizedArrayH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct NewImmRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub size_expression: ExpressionH<'s, 'h>,
    pub generator_expression: ExpressionH<'s, 'h>,
    pub generator_method: &'h PrototypeH<'s, 'h>,
    pub element_type: CoordH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}
/*
case class NewImmRuntimeSizedArrayH(
  // Expression containing the size of the new array.
  sizeExpression: ExpressionH[IntHT],
  // Expression containing the IFunction<int, T> interface reference which we'll
  // call to generate each element of the array.
  // More specifically, we'll call the "__call" function on the interface, which
  // should be the only function on it.
  // This is a borrow reference.
  generatorExpression: ExpressionH[KindHT],
  // The prototype for the "__call" function to call on the interface for each element.
  generatorMethod: PrototypeH,

  elementType: CoordH[KindHT],
  // The resulting type of the array.
  // TODO: Remove this, it's redundant with the generatorExpression's interface's
  // only method's return type.
  resultType: CoordH[RuntimeSizedArrayHT]
) extends ExpressionH[RuntimeSizedArrayHT] {
  // See BRCOBS, no arguments should be Never.
  // Nevermind, type system guarantees it
  //  sizeExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  generatorExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  generatorExpression.resultType.ownership match {
    case MutableShareH | ImmutableShareH | MutableBorrowH | ImmutableBorrowH =>
    case other => vwat(other)
  }
  vassert(sizeExpression.resultType.kind == IntHT.i32)
}

// Constructs an empty mutable unknown-size array, whose length is the integer from capacityExpression,
// whose values are generated from the function from generatorExpression. Puts the
// result in a new expressions.
*/
// mig: struct NewMutRuntimeSizedArrayH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct NewMutRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub capacity_expression: ExpressionH<'s, 'h>,
    pub element_type: CoordH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}
/*
case class NewMutRuntimeSizedArrayH(
  // Expression containing the capacity of the new array.
  capacityExpression: ExpressionH[IntHT],

  elementType: CoordH[KindHT],

  // The resulting type of the array.
  resultType: CoordH[RuntimeSizedArrayHT]
) extends ExpressionH[RuntimeSizedArrayHT] {
  // See BRCOBS, no arguments should be Never.
  // Nevermind, type system guarantees it
//  capacityExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
}

// Adds a new element to the end of a mutable unknown-size array.
*/
// mig: struct PushRuntimeSizedArrayH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct PushRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub newcomer_expression: ExpressionH<'s, 'h>,
}
/*
case class PushRuntimeSizedArrayH(
  // Expression for the array to add to.
  arrayExpression: ExpressionH[RuntimeSizedArrayHT],
  // Expression for the new element.
  newcomerExpression: ExpressionH[KindHT],
) extends ExpressionH[KindHT] {
  // See BRCOBS, no arguments should be Never.
  // Nevermind, type system guarantees it
//  arrayExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  newcomerExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

// Adds a new element to the end of a mutable unknown-size array.
*/
// mig: struct PopRuntimeSizedArrayH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct PopRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub element_type: CoordH<'s, 'h>,
}
/*
case class PopRuntimeSizedArrayH(
  // Expression for the array to add to.
  arrayExpression: ExpressionH[RuntimeSizedArrayHT],
  // The element type for the array.
  elementType: CoordH[KindHT]
) extends ExpressionH[KindHT] {
  // See BRCOBS, no arguments should be Never.
  // Nevermind, type system guarantees it
//  arrayExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();

*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = elementType
}

// Constructs an unknown-size array, whose length is the integer from sizeExpression,
// whose values are generated from the function from generatorExpression. Puts the
// result in a new expressions.
*/
// mig: struct StaticArrayFromCallableH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromCallableH<'s, 'h> where 's: 'h {
    pub generator_expression: ExpressionH<'s, 'h>,
    pub generator_method: &'h PrototypeH<'s, 'h>,
    pub element_type: CoordH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
}
/*
case class StaticArrayFromCallableH(
  // Expression containing the IFunction<int, T> interface reference which we'll
  // call to generate each element of the array.
  // More specifically, we'll call the "__call" function on the interface, which
  // should be the only function on it.
  // This is a borrow reference.
  generatorExpression: ExpressionH[KindHT],
  // The prototype for the "__call" function to call on the interface for each element.
  generatorMethod: PrototypeH,

  elementType: CoordH[KindHT],
  // The resulting type of the array.
  // TODO: Remove this, it's redundant with the generatorExpression's interface's
  // only method's return type.
  resultType: CoordH[StaticSizedArrayHT]
) extends ExpressionH[StaticSizedArrayHT] {
  // See BRCOBS, no arguments should be Never.
  generatorExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  vassert(
    generatorExpression.resultType.ownership == MutableBorrowH ||
      generatorExpression.resultType.ownership == ImmutableBorrowH ||
      generatorExpression.resultType.ownership == MutableShareH ||
      generatorExpression.resultType.ownership == ImmutableShareH)
}

// Destroys an array previously created with NewArrayFromValuesH.
*/
// mig: struct DestroyStaticSizedArrayIntoFunctionH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoFunctionH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub consumer_expression: ExpressionH<'s, 'h>,
    pub consumer_method: &'h PrototypeH<'s, 'h>,
    pub array_element_type: CoordH<'s, 'h>,
    pub array_size: i64,
}
/*
case class DestroyStaticSizedArrayIntoFunctionH(
  // Expression containing the array we'll destroy.
  // This is an owning reference.
  arrayExpression: ExpressionH[StaticSizedArrayHT],
  // Expression containing the argument we'll pass to consumerMethod with the element.
  consumerExpression: ExpressionH[KindHT],
  // The prototype for the "__call" function to call on the interface for each element.
  consumerMethod: PrototypeH,
  arrayElementType: CoordH[KindHT],
  arraySize: Long
) extends ExpressionH[VoidHT] {
  // See BRCOBS, no arguments should be Never.
  // Nevermind, type system guarantees it
//  arrayExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  consumerExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[VoidHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

// Destroys an array previously created with ConstructRuntimeSizedArrayH.
*/
// mig: struct DestroyImmRuntimeSizedArrayH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DestroyImmRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
    pub consumer_expression: ExpressionH<'s, 'h>,
    pub consumer_method: &'h PrototypeH<'s, 'h>,
    pub array_element_type: CoordH<'s, 'h>,
}
/*
case class DestroyImmRuntimeSizedArrayH(
  // Expression containing the array we'll destroy.
  // This is an owning reference.
  arrayExpression: ExpressionH[RuntimeSizedArrayHT],
  // Expression containing the argument we'll pass to consumerMethod with the element.
  consumerExpression: ExpressionH[KindHT],
  // The prototype for the "__call" function to call on the interface for each element.
  consumerMethod: PrototypeH,
  arrayElementType: CoordH[KindHT],
) extends ExpressionH[VoidHT] {
  // See BRCOBS, no arguments should be Never.
  // Nevermind, type system guarantees it
//  arrayExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }
  consumerExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[VoidHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

// Destroys an array previously created with ConstructRuntimeSizedArrayH.
*/
// mig: struct DestroyMutRuntimeSizedArrayH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DestroyMutRuntimeSizedArrayH<'s, 'h> where 's: 'h {
    pub array_expression: ExpressionH<'s, 'h>,
}
/*
case class DestroyMutRuntimeSizedArrayH(
  // Expression containing the array we'll destroy.
  // This is an owning reference.
  arrayExpression: ExpressionH[RuntimeSizedArrayHT]
) extends ExpressionH[VoidHT] {
  // See BRCOBS, no arguments should be Never.
  // Nevermind, type system guarantees it
//  arrayExpression.resultType.kind match { case NeverH(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[VoidHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

// Jumps to after the closest containing loop.
*/
// mig: struct BreakH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct BreakH;
/*
case class BreakH() extends ExpressionH[NeverHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[NeverHT] = CoordH(MutableShareH, InlineH, NeverHT(true))
}

// Creates a new struct instance.
*/
// mig: struct NewStructH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct NewStructH<'s, 'h> where 's: 'h {
    pub source_expressions: &'h [ExpressionH<'s, 'h>],
    pub target_member_names: &'h [&'h IdH<'s>],
    pub result_type: CoordH<'s, 'h>,
}
/*
case class NewStructH(
  // Expressions containing the values we'll use as members of the new struct.
  sourceExpressions: Vector[ExpressionH[KindHT]],
  // Names of the members of the struct, in order.
  targetMemberNames: Vector[IdH],
  // The type of struct we'll create.
  resultType: CoordH[StructHT]
) extends ExpressionH[StructHT] {
  // See BRCOBS, no arguments should be Never.
  sourceExpressions.foreach(expr => {
    expr.resultType.kind match { case NeverHT(_) => vwat() case _ => }
  })
}

// Gets the length of an unknown-sized array.
*/
// mig: struct ArrayLengthH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ArrayLengthH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
}
/*
case class ArrayLengthH(
  // Expression containing the array whose length we'll get.
  sourceExpression: ExpressionH[KindHT],
) extends ExpressionH[IntHT] {
  // See BRCOBS, no arguments should be Never.
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[IntHT] = CoordH(MutableShareH, InlineH, IntHT.i32)
}

// Gets the capacity of an unknown-sized array.
*/
// mig: struct ArrayCapacityH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ArrayCapacityH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
}
/*
case class ArrayCapacityH(
  // Expression containing the array whose length we'll get.
  sourceExpression: ExpressionH[KindHT],
) extends ExpressionH[IntHT] {
  // See BRCOBS, no arguments should be Never.
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[IntHT] = CoordH(MutableShareH, InlineH, IntHT.i32)
}

// Turns a borrow ref into a weak ref.
*/
// mig: struct BorrowToWeakH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct BorrowToWeakH<'s, 'h> where 's: 'h {
    pub ref_expression: ExpressionH<'s, 'h>,
}
/*
case class BorrowToWeakH(
  // Expression containing the borrow reference to turn into a weak ref.
  refExpression: ExpressionH[KindHT],
) extends ExpressionH[KindHT] {
  // See BRCOBS, no arguments should be Never.
  refExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  vassert(refExpression.resultType.ownership == ImmutableBorrowH || refExpression.resultType.ownership == MutableBorrowH)

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = CoordH(WeakH, YonderH, refExpression.resultType.kind)
}

// Checks if the given args are the same instance.
*/
// mig: struct IsSameInstanceH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct IsSameInstanceH<'s, 'h> where 's: 'h {
    pub left_expression: ExpressionH<'s, 'h>,
    pub right_expression: ExpressionH<'s, 'h>,
}
impl<'s, 'h> IsSameInstanceH<'s, 'h> where 's: 'h {
    pub fn result_type(&self) -> CoordH<'s, 'h> {
        CoordH { ownership: OwnershipH::MutableShareH, location: LocationH::InlineH, kind: KindHT::BoolHT(BoolHT) }
    }
    /* */
}
/*
case class IsSameInstanceH(
  leftExpression: ExpressionH[KindHT],
  rightExpression: ExpressionH[KindHT],
) extends ExpressionH[KindHT] {
  // See BRCOBS, no arguments should be Never.
  leftExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }
  rightExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = CoordH(MutableShareH, InlineH, BoolHT())
}

// Tries to downcast to the specified subtype and wrap in a Some.
// If it fails, will result in a None.
*/
// mig: struct AsSubtypeH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct AsSubtypeH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
    pub target_type: KindHT<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
    pub some_constructor: &'h PrototypeH<'s, 'h>,
    pub none_constructor: &'h PrototypeH<'s, 'h>,
}
/*
case class AsSubtypeH(
  // Expression whose result we'll try to downcast
  sourceExpression: ExpressionH[KindHT],
  // The subtype to try and cast the source to.
  targetType: KindHT,
  // Should be an owned ref to optional of something
  resultType: CoordH[InterfaceHT],
  // Function to give a ref to to make a Some(ref) {
  // val hash = runtime.ScalaRunTime._hashCode(this);
  // override def hashCode(): Int = hash;
  // override def equals(obj: Any): Boolean = vcurious(); }
  someConstructor: PrototypeH,
  // Function to make a None of the right type
  noneConstructor: PrototypeH,
) extends ExpressionH[KindHT] {
  // See BRCOBS, no arguments should be Never.
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }
}

// Locks a weak ref to turn it into an optional of borrow ref.
*/
// mig: struct LockWeakH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct LockWeakH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
    pub result_type: CoordH<'s, 'h>,
    pub some_constructor: &'h PrototypeH<'s, 'h>,
    pub none_constructor: &'h PrototypeH<'s, 'h>,
}
/*
case class LockWeakH(
  // Expression containing the array whose length we'll get.
  sourceExpression: ExpressionH[KindHT],
  // Should be an owned ref to optional of borrow ref of something
  resultType: CoordH[InterfaceHT],
  // Function to give a borrow ref to to make a Some(borrow ref)
  someConstructor: PrototypeH,
  // Function to make a None of the right type
  noneConstructor: PrototypeH,
) extends ExpressionH[KindHT] {
  // See BRCOBS, no arguments should be Never.
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }
}

// See DINSIE for why this isn't three instructions, and why we don't have the
// destructor prototype in it.
*/
// mig: struct DiscardH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct DiscardH<'s, 'h> where 's: 'h {
    pub source_expression: ExpressionH<'s, 'h>,
}
/*
case class DiscardH(sourceExpression: ExpressionH[KindHT]) extends ExpressionH[VoidHT] {
  // See BRCOBS, no arguments should be Never.
  sourceExpression.resultType.kind match { case NeverHT(_) => vwat() case _ => }

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  sourceExpression.resultType.ownership match {
    case MutableBorrowH | ImmutableBorrowH | MutableShareH | ImmutableShareH | WeakH =>
  }
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[VoidHT] = CoordH(MutableShareH, InlineH, VoidHT())
}

*/
// mig: struct PreCheckBorrowH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct PreCheckBorrowH<'s, 'h> where 's: 'h {
    pub inner_expression: ExpressionH<'s, 'h>,
}
/*
case class PreCheckBorrowH(innerExpression: ExpressionH[KindHT]) extends ExpressionH[KindHT] {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
  innerExpression.resultType.ownership match {
    case MutableBorrowH =>
  }
*/
// mig: fn result_type (realized-by-dispatcher)
/*
  override def resultType: CoordH[KindHT] = innerExpression.resultType
}

*/
// mig: trait IExpressionH
/// Polyvalue
#[derive(Copy, Clone, Debug)]
pub enum IExpressionH<'s, 'h> where 's: 'h {
    ReferenceExpressionH(&'h ReferenceExpressionH<'s, 'h>),
    AddressExpressionH(&'h AddressExpressionH<'s, 'h>),
}
/*
trait IExpressionH {
*/
// mig: fn expect_reference_expression
pub fn expect_reference_expression<'s, 'h>(this: &IExpressionH<'s, 'h>) -> &'h ReferenceExpressionH<'s, 'h> {
    panic!("Unimplemented: expect_reference_expression");
}
/*
  def expectReferenceExpression(): ReferenceExpressionH = {
    this match {
      case r @ ReferenceExpressionH(_) => r
      case AddressExpressionH(_) => vfail("Expected a reference as a result, but got an address!")
    }
  }
*/
// mig: fn expect_address_expression
pub fn expect_address_expression<'s, 'h>(this: &IExpressionH<'s, 'h>) -> &'h AddressExpressionH<'s, 'h> {
    panic!("Unimplemented: expect_address_expression");
}
/*
  def expectAddressExpression(): AddressExpressionH = {
    this match {
      case a @ AddressExpressionH(_) => a
      case ReferenceExpressionH(_) => vfail("Expected an address as a result, but got a reference!")
    }
  }
}
*/
// mig: struct ReferenceExpressionH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct ReferenceExpressionH<'s, 'h> where 's: 'h {
    pub reference: CoordH<'s, 'h>,
}
/*
case class ReferenceExpressionH(reference: CoordH[KindHT]) extends IExpressionH {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious(); }
*/
// mig: struct AddressExpressionH
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct AddressExpressionH<'s, 'h> where 's: 'h {
    pub reference: CoordH<'s, 'h>,
}
/*
case class AddressExpressionH(reference: CoordH[KindHT]) extends IExpressionH {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious(); }

// Identifies a local variable.
*/
// mig: struct Local
/// Temporary state
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Local<'s, 'h> where 's: 'h {
    pub id: VariableIdH<'s, 'h>,
    pub variability: Variability,
    pub type_h: CoordH<'s, 'h>,
}
/*
case class Local(
  // No two variables in a FunctionH have the same id.
  id: VariableIdH,

  // Whether the local is ever changed or not.
  variability: Variability,

  // The type of the reference this local variable has.
  typeH: CoordH[KindHT],

//  // Usually filled by catalyst, for Midas' benefit. Used in HGM.
//  keepAlive: Boolean
) {

  val hash = runtime.ScalaRunTime._hashCode(this)

*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
/*
override def equals(obj: Any): Boolean = vcurious();
}

*/
// mig: struct VariableIdH
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VariableIdH<'s, 'h> where 's: 'h {
    pub number: i32,
    pub height: i32,
    pub name: Option<&'h IdH<'s>>,
}
/*
case class VariableIdH(
  // Just to uniquify VariableIdH instances. No two variables in a FunctionH will have
  // the same number.
  number: Int,
  // Where the variable is relative to the stack frame's beginning.
  height: Int,
  // Just for debugging purposes
  name: Option[IdH]) {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
/*
  override def hashCode(): Int = hash;
}
*/


// ---- 50 expression-variant payload structs, Scala-parity fields ----













































