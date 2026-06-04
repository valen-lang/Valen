/*
package dev.vale.instantiating.ast

import dev.vale._
import dev.vale.postparsing._
*/
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

// mig: trait ExpressionIE — realized as enum per @ATDCX (no dyn, mirrors typing-pass ExpressionTE).
/// Arena-allocated (see @TFITCX)
//
// No PartialEq/Hash derive: Scala uses `vcurious` (panic on equals) on every
// expression case class; Rust's "no impl" gives a strictly stronger compile-time
// error. Mirrors typing/ast/expressions.rs ExpressionTE.
#[derive(Copy, Clone, Debug)]
pub enum ExpressionIE<'s, 'i, R> {
    Reference(ReferenceExpressionIE<'s, 'i, R>),
    Address(AddressExpressionIE<'s, 'i, R>),
}
/*
trait ExpressionI  {
*/
// mig: fn result
impl<'s, 'i, R: Copy> ExpressionIE<'s, 'i, R> {
    pub fn result(&self) -> CoordI<'s, 'i, R> {
        match self {
            ExpressionIE::Reference(r) => r.result(),
            ExpressionIE::Address(a) => panic!("ExpressionIE::result: Address branch"),
        }
    }
}
impl<'s, 'i, R: Copy> ReferenceExpressionIE<'s, 'i, R> {
    pub fn result(&self) -> CoordI<'s, 'i, R> {
        match self {
            ReferenceExpressionIE::LetAndLend(x) => x.result,
            ReferenceExpressionIE::LockWeak(_) => panic!("RE::result: LockWeak"),
            ReferenceExpressionIE::BorrowToWeak(_) => panic!("RE::result: BorrowToWeak"),
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
            ReferenceExpressionIE::IsSameInstance(_) => panic!("RE::result: IsSameInstance"),
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
            ReferenceExpressionIE::StaticArrayFromCallable(_) => panic!("RE::result: StaticArrayFromCallable"),
            ReferenceExpressionIE::DestroyStaticSizedArrayIntoFunction(d) => d.result(),
            ReferenceExpressionIE::DestroyStaticSizedArrayIntoLocals(_) => panic!("RE::result: DestroyStaticSizedArrayIntoLocals"),
            ReferenceExpressionIE::DestroyMutRuntimeSizedArray(_) => CoordI { ownership: crate::instantiating::ast::types::OwnershipI::MutableShare, kind: crate::instantiating::ast::types::KindIT::VoidIT(crate::instantiating::ast::types::VoidIT { _marker: std::marker::PhantomData }) },
            ReferenceExpressionIE::RuntimeSizedArrayCapacity(_) => panic!("RE::result: RuntimeSizedArrayCapacity"),
            ReferenceExpressionIE::PushRuntimeSizedArray(_) => CoordI { ownership: crate::instantiating::ast::types::OwnershipI::MutableShare, kind: crate::instantiating::ast::types::KindIT::VoidIT(crate::instantiating::ast::types::VoidIT { _marker: std::marker::PhantomData }) },
            ReferenceExpressionIE::PopRuntimeSizedArray(p) => p.result,
            ReferenceExpressionIE::InterfaceToInterfaceUpcast(_) => panic!("RE::result: InterfaceToInterfaceUpcast"),
            ReferenceExpressionIE::Upcast(u) => u.result,
            ReferenceExpressionIE::SoftLoad(s) => s.result,
            ReferenceExpressionIE::Destroy(_) => panic!("RE::result: Destroy"),
            ReferenceExpressionIE::DestroyImmRuntimeSizedArray(_) => panic!("RE::result: DestroyImmRuntimeSizedArray"),
            ReferenceExpressionIE::NewImmRuntimeSizedArray(n) => n.result,
        }
    }
}
/*
Guardian: temp-disable: SPDMX — Multi-variant dispatcher; the Scala overrides live on the per-variant case classes (Expressions.scala: PushRuntimeSizedArrayIE :750-756 `override def result = CoordI(MutableShareI, VoidIT())`; DestroyMutRuntimeSizedArrayIE :737-740 same; PopRuntimeSizedArrayIE has its own `result` field). Guardian's diff window only sees the abstract trait method `def result: CoordI[cI]`, not the per-variant overrides which were sliced onto the case-class bodies. Same precedent as the StackifyH / ConstantIntH dispatcher temp-disables at final_ast/instructions.rs:1255-1380. TL-attribution exception: NAGDX blocks JR's hand-write and JR's MCP session can't see the new `target_line` param without a process kill+respawn; ordained TL writes the directive as a one-time tooling-blocked workaround. — /Volumes/V/Vale2/FrontendRust/guardian-logs/request-552-1780528341799/hook-552/result--43.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
*/
/*
  def result: CoordI[cI]
}
*/
// mig: trait ReferenceExpressionIE — realized as enum per @ATDCX.
#[derive(Copy, Clone, Debug)]
pub enum ReferenceExpressionIE<'s, 'i, R> {
    LetAndLend(&'i LetAndLendIE<'s, 'i, R>),
    LockWeak(&'i LockWeakIE<'s, 'i, R>),
    BorrowToWeak(&'i BorrowToWeakIE<'s, 'i, R>),
    LetNormal(&'i LetNormalIE<'s, 'i, R>),
    Restackify(&'i RestackifyIE<'s, 'i, R>),
    Unlet(&'i UnletIE<'s, 'i, R>),
    Discard(&'i DiscardIE<'s, 'i, R>),
    Defer(&'i DeferIE<'s, 'i, R>),
    If(&'i IfIE<'s, 'i, R>),
    While(&'i WhileIE<'s, 'i, R>),
    Mutate(&'i MutateIE<'s, 'i, R>),
    Return(&'i ReturnIE<'s, 'i, R>),
    Break(&'i BreakIE<'s, 'i, R>),
    Block(&'i BlockIE<'s, 'i, R>),
    Mutabilify(&'i MutabilifyIE<'s, 'i, R>),
    Immutabilify(&'i ImmutabilifyIE<'s, 'i, R>),
    PreCheckBorrow(&'i PreCheckBorrowIE<'s, 'i, R>),
    Consecutor(&'i ConsecutorIE<'s, 'i, R>),
    Tuple(&'i TupleIE<'s, 'i, R>),
    StaticArrayFromValues(&'i StaticArrayFromValuesIE<'s, 'i, R>),
    ArraySize(&'i ArraySizeIE<'s, 'i, R>),
    IsSameInstance(&'i IsSameInstanceIE<'s, 'i, R>),
    AsSubtype(&'i AsSubtypeIE<'s, 'i, R>),
    VoidLiteral(&'i VoidLiteralIE<'s, 'i, R>),
    ConstantInt(&'i ConstantIntIE<'s, 'i, R>),
    ConstantBool(&'i ConstantBoolIE<'s, 'i, R>),
    ConstantStr(&'i ConstantStrIE<'s, 'i, R>),
    ConstantFloat(&'i ConstantFloatIE<'s, 'i, R>),
    ArgLookup(&'i ArgLookupIE<'s, 'i, R>),
    ArrayLength(&'i ArrayLengthIE<'s, 'i, R>),
    InterfaceFunctionCall(&'i InterfaceFunctionCallIE<'s, 'i, R>),
    ExternFunctionCall(&'i ExternFunctionCallIE<'s, 'i, R>),
    FunctionCall(&'i FunctionCallIE<'s, 'i, R>),
    Reinterpret(&'i ReinterpretIE<'s, 'i, R>),
    Construct(&'i ConstructIE<'s, 'i, R>),
    NewMutRuntimeSizedArray(&'i NewMutRuntimeSizedArrayIE<'s, 'i, R>),
    StaticArrayFromCallable(&'i StaticArrayFromCallableIE<'s, 'i, R>),
    DestroyStaticSizedArrayIntoFunction(&'i DestroyStaticSizedArrayIntoFunctionIE<'s, 'i, R>),
    DestroyStaticSizedArrayIntoLocals(&'i DestroyStaticSizedArrayIntoLocalsIE<'s, 'i, R>),
    DestroyMutRuntimeSizedArray(&'i DestroyMutRuntimeSizedArrayIE<'s, 'i, R>),
    RuntimeSizedArrayCapacity(&'i RuntimeSizedArrayCapacityIE<'s, 'i, R>),
    PushRuntimeSizedArray(&'i PushRuntimeSizedArrayIE<'s, 'i, R>),
    PopRuntimeSizedArray(&'i PopRuntimeSizedArrayIE<'s, 'i, R>),
    InterfaceToInterfaceUpcast(&'i InterfaceToInterfaceUpcastIE<'s, 'i, R>),
    Upcast(&'i UpcastIE<'s, 'i, R>),
    SoftLoad(&'i SoftLoadIE<'s, 'i, R>),
    Destroy(&'i DestroyIE<'s, 'i, R>),
    DestroyImmRuntimeSizedArray(&'i DestroyImmRuntimeSizedArrayIE<'s, 'i, R>),
    NewImmRuntimeSizedArray(&'i NewImmRuntimeSizedArrayIE<'s, 'i, R>),
}
/*
trait ReferenceExpressionIE extends ExpressionI { }
*/
// mig: trait AddressExpressionIE — realized as enum per @ATDCX.
#[derive(Copy, Clone, Debug)]
pub enum AddressExpressionIE<'s, 'i, R> {
    LocalLookup(&'i LocalLookupIE<'s, 'i, R>),
    StaticSizedArrayLookup(&'i StaticSizedArrayLookupIE<'s, 'i, R>),
    RuntimeSizedArrayLookup(&'i RuntimeSizedArrayLookupIE<'s, 'i, R>),
    ReferenceMemberLookup(&'i ReferenceMemberLookupIE<'s, 'i, R>),
    AddressMemberLookup(&'i AddressMemberLookupIE<'s, 'i, R>),
}
/*
// This is an Expression2 because we sometimes take an address and throw it
// directly into a struct (closures!), which can have addressible members.
trait AddressExpressionIE extends ExpressionI {
//  def range: RangeS

//  // Whether or not we can change where this address points to
//  def variability: VariabilityI
}
*/
// mig: struct LetAndLendIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct LetAndLendIE<'s, 'i, R> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub expr: ReferenceExpressionIE<'s, 'i, R>,
	pub target_ownership: OwnershipI,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl LetAndLendIE
/*
case class LetAndLendIE(
  variable: ILocalVariableI,
  expr: ReferenceExpressionIE,
  targetOwnership: OwnershipI,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LetAndLendIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LetAndLendIE` below.)
/*
override def hashCode(): Int = vcurious()
  vassert(variable.collapsedCoord == expr.result)

  (expr.result.ownership, targetOwnership) match {
    case (MutableShareI, MutableShareI) =>
    case (ImmutableShareI, ImmutableShareI) =>
    case (OwnI | MutableBorrowI | WeakI | MutableShareI, MutableBorrowI) =>
    case (ImmutableBorrowI, ImmutableBorrowI) =>
  }

  expr match {
    case BreakIE() | ReturnIE(_) => vwat() // See BRCOBS
    case _ =>
  }
}
*/
// mig: struct LockWeakIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct LockWeakIE<'s, 'i, R> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub result_opt_borrow_type: CoordI<'s, 'i, R>,
	pub some_constructor: PrototypeI<'s, 'i, R>,
	pub none_constructor: PrototypeI<'s, 'i, R>,
	pub some_impl_name: IdI<'s, 'i, R>,
	pub none_impl_name: IdI<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl LockWeakIE
/*
case class LockWeakIE(
  innerExpr: ReferenceExpressionIE,
  // We could just calculaIE this, but it feels better to let the StructCompiler
  // make it, so we're sure it's created.
  resultOptBorrowType: CoordI[cI],

  // Function to give a borrow ref to to make a Some(borrow ref)
  someConstructor: PrototypeI[cI],
  // Function to make a None of the right type
  noneConstructor: PrototypeI[cI],

  // This is the impl we use to allow/permit the upcast from the some to the none.
  // It'll be useful for monomorphization and later on for locating the itable ptr to put in fat pointers.
  someImplName: IdI[cI, IImplNameI[cI]],
  // This is the impl we use to allow/permit the upcast from the some to the none.
  // It'll be useful for monomorphization and later on for locating the itable ptr to put in fat pointers.
  noneImplName: IdI[cI, IImplNameI[cI]],

  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LockWeakIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LockWeakIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe: CoordI[cI] = resultOptBorrowType
}
*/
// mig: struct BorrowToWeakIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct BorrowToWeakIE<'s, 'i, R> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl BorrowToWeakIE
/*
// Turns a borrow ref into a weak ref
// NoIE that we can also get a weak ref from LocalLoad2'ing a
// borrow ref local into a weak ref.
case class BorrowToWeakIE(
  innerExpr: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
  vassert(
    innerExpr.result.ownership == ImmutableBorrowI ||
      innerExpr.result.ownership == MutableBorrowI)

*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for BorrowToWeakIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for BorrowToWeakIE` below.)
/*
override def hashCode(): Int = vcurious()
  innerExpr.result.ownership match {
    case MutableBorrowI | ImmutableBorrowI =>
  }
  vassert(result.ownership == vregionmut(WeakI))

//  override def resultRemoveMe: CoordI[cI] = {
//    vimpl()//ReferenceResultI(CoordI[cI](WeakI, innerExpr.kind))
//  }
}
*/
// mig: struct LetNormalIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct LetNormalIE<'s, 'i, R> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub expr: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl LetNormalIE
/*
case class LetNormalIE(
  variable: ILocalVariableI,
  expr: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LetNormalIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LetNormalIE` below.)
/*
override def hashCode(): Int = vcurious()

  expr.result.kind match {
    case NeverIT(_) => // then we can put it into whatever type we want
    case _ => {
      variable.collapsedCoord.kind match {
        case NeverIT(_) => vfail() // can't receive into a never
        case _ => vassert(variable.collapsedCoord == expr.result)
      }
    }
  }

  expr match {
    case BreakIE() | ReturnIE(_) => vwat() // See BRCOBS
    case _ =>
  }
}
*/
// mig: struct RestackifyIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct RestackifyIE<'s, 'i, R> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub expr: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl RestackifyIE
/*
case class RestackifyIE(
  variable: ILocalVariableI,
  expr: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for RestackifyIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for RestackifyIE` below.)
/*
override def hashCode(): Int = vcurious()

  expr.result.kind match {
    case NeverIT(_) => // then we can put it into whatever type we want
    case _ => {
      variable.collapsedCoord.kind match {
        case NeverIT(_) => vfail() // can't receive into a never
        case _ => vassert(variable.collapsedCoord == expr.result)
      }
    }
  }

  expr match {
    case BreakIE() | ReturnIE(_) => vwat() // See BRCOBS
    case _ =>
  }
}
*/
// mig: struct UnletIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct UnletIE<'s, 'i, R> {
	pub variable: ILocalVariableI<'s, 'i>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl UnletIE
/*
// Only ExpressionCompiler.unletLocal should make these
case class UnletIE(
  variable: ILocalVariableI,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for UnletIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for UnletIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe = variable.collapsedCoord

  vpass()
}
*/
// mig: struct DiscardIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DiscardIE<'s, 'i, R> {
	pub expr: ReferenceExpressionIE<'s, 'i, R>,
}
// mig: impl DiscardIE
/*
// Throws away a reference.
// Unless given to an instruction which consumes it, all borrow and share
// references must eventually hit a Discard2, just like all owning
// references must eventually hit a Destructure2.
// Depending on the backend, it will either be a no-op (like for GC'd backends)
// or a decrement+maybedestruct (like for RC'd backends)
// See DINSIE for why this isnt three instructions, and why we dont have the
// destructor in here for shareds.
case class DiscardIE(
  expr: ReferenceExpressionIE
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DiscardIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DiscardIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> DiscardIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())

  expr.result.ownership match {
    case MutableBorrowI =>
    case ImmutableBorrowI =>
    case MutableShareI | ImmutableShareI =>
    case WeakI =>
  }

  expr match {
    case ConsecutorIE(exprs, _) => {
      exprs.last match {
        case DiscardIE(_) => vwat()
        case _ =>
      }
    }
    case _ =>
  }
}
*/
// mig: struct DeferIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DeferIE<'s, 'i, R> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub deferred_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl DeferIE
/*
case class DeferIE(
  innerExpr: ReferenceExpressionIE,
  // Every deferred expression should discard its result, IOW, return Void.
  deferredExpr: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DeferIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DeferIE` below.)
/*
override def hashCode(): Int = vcurious()

//  override def resultRemoveMe = ReferenceResultI(innerExpr.result)

  vassert(deferredExpr.result == CoordI[cI](MutableShareI, VoidIT()))
}
*/
// mig: struct IfIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct IfIE<'s, 'i, R> {
	pub condition: ReferenceExpressionIE<'s, 'i, R>,
	pub then_call: ReferenceExpressionIE<'s, 'i, R>,
	pub else_call: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl IfIE
/*
// Eventually, when we want to do if-let, we'll have a different construct
// entirely. See comment below If2.
// These are blocks because we don't want inner locals to escape.
case class IfIE(
  condition: ReferenceExpressionIE,
  thenCall: ReferenceExpressionIE,
  elseCall: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for IfIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for IfIE` below.)
/*
override def hashCode(): Int = vcurious()
  private val conditionResultCoord = condition.result
  private val thenResultCoord = thenCall.result
  private val elseResultCoord = elseCall.result

  conditionResultCoord match {
    case CoordI(MutableShareI | ImmutableShareI, BoolIT()) =>
    case other => vfail(other)
  }

  (thenResultCoord.kind, thenResultCoord.kind) match {
    case (NeverIT(_), _) =>
    case (_, NeverIT(_)) =>
    case (a, b) if a == b =>
    case _ => vwat()
  }

  private val commonSupertype =
    thenResultCoord.kind match {
      case NeverIT(_) => elseResultCoord
      case _ => thenResultCoord
    }

//  override def resultRemoveMe = ReferenceResultI(commonSupertype)
}
*/
// mig: struct WhileIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct WhileIE<'s, 'i, R> {
	pub block: BlockIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl WhileIE
/*
// The block is expected to return a boolean (false = stop, true = keep going).
// The block will probably contain an If2(the condition, the body, false)
case class WhileIE(
  block: BlockIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for WhileIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for WhileIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe = ReferenceResultI(resultCoord)
  vpass()
}
*/
// mig: struct MutateIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct MutateIE<'s, 'i, R> {
	pub destination_expr: AddressExpressionIE<'s, 'i, R>,
	pub source_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl MutateIE
/*
case class MutateIE(
  destinationExpr: AddressExpressionIE,
  sourceExpr: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for MutateIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for MutateIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe = ReferenceResultI(destinationExpr.result)
}
*/
// mig: struct ReturnIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ReturnIE<'s, 'i, R> {
	pub source_expr: ReferenceExpressionIE<'s, 'i, R>,
}
// mig: impl ReturnIE
/*
case class ReturnIE(
  sourceExpr: ReferenceExpressionIE
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReturnIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReturnIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> ReturnIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, NeverIT(false))
}
*/
// mig: struct BreakIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct BreakIE<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
// mig: impl BreakIE
/*
case class BreakIE() extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for BreakIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for BreakIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> BreakIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, NeverIT(true))
}

*/
// mig: struct BlockIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct BlockIE<'s, 'i, R> {
	pub inner: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl BlockIE
/*
// when we make a closure, we make a struct full of pointers to all our variables
// and the first element is our parent closure
// this can live on the stack, since blocks are additive to this expression
// later we can optimize it to only have the things we use

// Block2 is required to unlet all the variables it introduces.
case class BlockIE(
  inner: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
  vpass()
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for BlockIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for BlockIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe = inner.result
}
*/
// mig: struct MutabilifyIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct MutabilifyIE<'s, 'i, R> {
	pub inner: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl MutabilifyIE
/*
// A pure block will:
// 1. Create a new region (someday possibly with an allocator)
// 2. Freeze the existing region
// 3. Run the inner code
// 4. Un-freeze the existing region
// 5. Merge (transmigrate) any results from the new region into the existing region
// 6. Destroy the new region
case class MutabilifyIE(
  inner: ReferenceExpressionIE,
  result: CoordI[cI] // See HCCSCS
) extends ReferenceExpressionIE {
  vpass()
  vassert(inner.result.kind == result.kind)
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for MutabilifyIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for MutabilifyIE` below.)
/*
override def hashCode(): Int = vcurious()
}
*/
// mig: struct ImmutabilifyIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ImmutabilifyIE<'s, 'i, R> {
	pub inner: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl ImmutabilifyIE
/*
// See NPFCASTN
case class ImmutabilifyIE(
    inner: ReferenceExpressionIE,
    result: CoordI[cI]
) extends ReferenceExpressionIE {
  vpass()
  vassert(inner.result.kind == result.kind)
  vassert(inner.result.ownership == MutableBorrowI || inner.result.ownership == MutableShareI)
  vassert(result.ownership == ImmutableBorrowI || result.ownership == ImmutableShareI)
  inner match {
    case SoftLoadIE(_, _, _) => {
      // The SoftLoadIE should be immutabilifying on its own.
      // We should have code that looks for this and simplifies it away.
      vwat()
    }
    case _ =>
  }
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ImmutabilifyIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ImmutabilifyIE` below.)
/*
override def hashCode(): Int = vcurious()
}
*/
// mig: struct PreCheckBorrowIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct PreCheckBorrowIE<'s, 'i, R> {
	pub inner: ReferenceExpressionIE<'s, 'i, R>,
}
// mig: impl PreCheckBorrowIE
/*
case class PreCheckBorrowIE(
  inner: ReferenceExpressionIE
) extends ReferenceExpressionIE {
  vpass()
  vassert(inner.result.ownership == MutableBorrowI)
*/
// mig: fn result
impl<'s, 'i, R> PreCheckBorrowIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = inner.result
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for PreCheckBorrowIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for PreCheckBorrowIE` below.)
/*
override def hashCode(): Int = vcurious()
}
*/
// mig: struct ConsecutorIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConsecutorIE<'s, 'i, R> {
	pub exprs: &'i[ReferenceExpressionIE<'s, 'i, R>],
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl ConsecutorIE
/*
case class ConsecutorIE(
  exprs: Vector[ReferenceExpressionIE],
  result: CoordI[cI]
  ) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConsecutorIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConsecutorIE` below.)
/*
override def hashCode(): Int = vcurious()
  // There shouldn't be a 0-element consecutor.
  // If we want a consecutor that returns nothing, put a VoidLiteralIE in it.
  vassert(exprs.nonEmpty)
}
*/
// mig: struct TupleIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct TupleIE<'s, 'i, R> {
	pub elements: &'i[ReferenceExpressionIE<'s, 'i, R>],
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl TupleIE
/*
case class TupleIE(
  elements: Vector[ReferenceExpressionIE],
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for TupleIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for TupleIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe = ReferenceResultI(resultReference)
}
*/
// mig: struct StaticArrayFromValuesIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromValuesIE<'s, 'i, R> {
	pub elements: &'i[ReferenceExpressionIE<'s, 'i, R>],
	pub result_reference: CoordI<'s, 'i, R>,
	pub array_type: StaticSizedArrayIT<'s, 'i, R>,
}
// mig: impl StaticArrayFromValuesIE
/*
//// Discards a reference, whether it be owned or borrow or whatever.
//// This is used after panics or other never-returning things, to signal that a certain
//// variable should be considered gone. See AUMAP.
//// This can also be used if theres anything after a panic in a block, like
////   exported func main() int {
////     __panic();
////     println("hi");
////   }
//case class UnreachableMootIE(innerExpr: ReferenceExpressionIE) extends ReferenceExpressionIE {
//  override def equals(obj: Any): Boolean = vcurious();
//override def hashCode(): Int = vcurious()
//  override def resultRemoveMe = ReferenceResulIT(CoordI[cI](MutableShareI, NeverI()))
//}

case class StaticArrayFromValuesIE(
  elements: Vector[ReferenceExpressionIE],
  resultReference: CoordI[cI],
  arrayType: StaticSizedArrayIT[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for StaticArrayFromValuesIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for StaticArrayFromValuesIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> StaticArrayFromValuesIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = resultReference
}
*/
// mig: struct ArraySizeIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ArraySizeIE<'s, 'i, R> {
	pub array: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl ArraySizeIE
/*
case class ArraySizeIE(
  array: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ArraySizeIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ArraySizeIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe = ReferenceResultI(CoordI[cI](MutableShareI, IntIT.i32))
}
*/
// mig: struct IsSameInstanceIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct IsSameInstanceIE<'s, 'i, R> {
	pub left: ReferenceExpressionIE<'s, 'i, R>,
	pub right: ReferenceExpressionIE<'s, 'i, R>,
}
// mig: impl IsSameInstanceIE
/*
// Can we do an === of objects in two regions? It could be pretty useful.
case class IsSameInstanceIE(
  left: ReferenceExpressionIE,
  right: ReferenceExpressionIE
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for IsSameInstanceIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for IsSameInstanceIE` below.)
/*
override def hashCode(): Int = vcurious()
  vassert(left.result == right.result)
*/
// mig: fn result
impl<'s, 'i, R> IsSameInstanceIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, BoolIT())
}
*/
// mig: struct AsSubtypeIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct AsSubtypeIE<'s, 'i, R> {
	pub source_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub target_type: CoordI<'s, 'i, R>,
	pub result_result_type: CoordI<'s, 'i, R>,
	pub ok_constructor: &'i PrototypeI<'s, 'i, R>,
	pub err_constructor: &'i PrototypeI<'s, 'i, R>,
	pub impl_name: IdI<'s, 'i, R>,
	pub ok_impl_name: IdI<'s, 'i, R>,
	pub err_impl_name: IdI<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl AsSubtypeIE
/*
case class AsSubtypeIE(
  sourceExpr: ReferenceExpressionIE,
  targetType: CoordI[cI],

  // We could just calculaIE this, but it feels better to let the StructCompiler
  // make it, so we're sure it's created.
  resultResultType: CoordI[cI],
  // Function to give a borrow ref to to make a Some(borrow ref)
  okConstructor: PrototypeI[cI],
  // Function to make a None of the right type
  errConstructor: PrototypeI[cI],

  // This is the impl we use to allow/permit the downcast. It'll be useful for monomorphization.
  implName: IdI[cI, IImplNameI[cI]],

  // These are the impls that we conceptually use to upcast the created Ok/Err to Result.
  // Really they're here so the instantiator can know what impls it needs to instantiaIE.
  okImplName: IdI[cI, IImplNameI[cI]],
  errImplName: IdI[cI, IImplNameI[cI]],

  result: CoordI[cI]
) extends ReferenceExpressionIE {
  vpass()
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for AsSubtypeIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for AsSubtypeIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe = ReferenceResultI(resultResultType)
}
*/
// mig: struct VoidLiteralIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct VoidLiteralIE<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
// mig: impl VoidLiteralIE
/*
case class VoidLiteralIE() extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for VoidLiteralIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for VoidLiteralIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> VoidLiteralIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> {
		CoordI { ownership: crate::instantiating::ast::types::OwnershipI::MutableShare, kind: crate::instantiating::ast::types::KindIT::VoidIT(crate::instantiating::ast::types::VoidIT { _marker: std::marker::PhantomData }) }
	}
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct ConstantIntIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstantIntIE<'s, 'i, R> {
	pub value: i64,
	pub bits: i32,
	pub _marker: std::marker::PhantomData<(&'s (), &'i (), R)>,
}
// mig: impl ConstantIntIE
/*
case class ConstantIntIE(value: Long, bits: Int) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstantIntIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstantIntIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> ConstantIntIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> {
		CoordI {
			ownership: crate::instantiating::ast::types::OwnershipI::MutableShare,
			kind: crate::instantiating::ast::types::KindIT::IntIT(crate::instantiating::ast::types::IntIT { bits: self.bits, _marker: std::marker::PhantomData }),
		}
	}
}
/*
  override def result = CoordI[cI](MutableShareI, IntIT(bits))
}
*/
// mig: struct ConstantBoolIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstantBoolIE<'s, 'i, R> {
	pub _marker: std::marker::PhantomData<(&'s (), &'i (), R)>,
	pub value: bool,
}
// mig: impl ConstantBoolIE
/*
case class ConstantBoolIE(value: Boolean) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstantBoolIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstantBoolIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> ConstantBoolIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> {
		CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::BoolIT(BoolIT { _marker: std::marker::PhantomData }) }
	}
}
/*
  override def result = CoordI[cI](MutableShareI, BoolIT())
}
*/
// mig: struct ConstantStrIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstantStrIE<'s, 'i, R> {
	pub _marker: std::marker::PhantomData<(&'s (), &'i (), R)>,
	pub value: &'s str,
}
// mig: impl ConstantStrIE
/*
case class ConstantStrIE(value: String) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstantStrIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstantStrIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> ConstantStrIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> {
		CoordI { ownership: crate::instantiating::ast::types::OwnershipI::MutableShare, kind: crate::instantiating::ast::types::KindIT::StrIT(crate::instantiating::ast::types::StrIT { _marker: std::marker::PhantomData }) }
	}
}
/*
  override def result = CoordI[cI](MutableShareI, StrIT())
}
*/
// mig: struct ConstantFloatIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstantFloatIE<'s, 'i, R> {
	pub _marker: std::marker::PhantomData<(&'s (), &'i (), R)>,
	pub value: f64,
}
// mig: impl ConstantFloatIE
/*
case class ConstantFloatIE(value: Double) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstantFloatIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstantFloatIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> ConstantFloatIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> {
		CoordI { ownership: crate::instantiating::ast::types::OwnershipI::MutableShare, kind: crate::instantiating::ast::types::KindIT::FloatIT(crate::instantiating::ast::types::FloatIT { _marker: std::marker::PhantomData }) }
	}
}
/*
  override def result = CoordI[cI](MutableShareI, FloatIT())
}

*/
// mig: struct LocalLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct LocalLookupIE<'s, 'i, R> {
	pub local_variable: ILocalVariableI<'s, 'i>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl LocalLookupIE
/*
case class LocalLookupIE(
  // This is the local variable at the time it was created
  localVariable: ILocalVariableI,
//  // The instantiator might want to load this as a different region mutability than the mutability
//  // when originally created, so tihs field will be able to hold that.
//  // Conceptually, it's the current mutability of the source region at the time of the local lookup.
//  pureHeight: Int,
  // nevermind, we leave it to SoftLoad to figure out the target ownership/immutability

  //  reference: CoordI[cI],
  //  variability: VariabilityI
  result: CoordI[cI]
) extends AddressExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LocalLookupIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LocalLookupIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def variability: VariabilityI = localVariable.variability
}
*/
// mig: struct ArgLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ArgLookupIE<'s, 'i, R> {
	pub param_index: i32,
	pub coord: CoordI<'s, 'i, R>,
}
// mig: impl ArgLookupIE
/*
case class ArgLookupIE(
  paramIndex: Int,
  coord: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ArgLookupIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ArgLookupIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> ArgLookupIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = coord
}
*/
// mig: struct StaticSizedArrayLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct StaticSizedArrayLookupIE<'s, 'i, R> {
	pub range: RangeS<'s>,
	pub array_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub index_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub element_type: CoordI<'s, 'i, R>,
	pub variability: VariabilityI,
}
// mig: impl StaticSizedArrayLookupIE
/*
case class StaticSizedArrayLookupIE(
  range: RangeS,
  arrayExpr: ReferenceExpressionIE,
  indexExpr: ReferenceExpressionIE,
  // See RMLRMO for why this is the same ownership as the original field.
  elementType: CoordI[cI],
  // See RMLRMO for why we dont have a targetOwnership field here.
  variability: VariabilityI
) extends AddressExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for StaticSizedArrayLookupIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for StaticSizedArrayLookupIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> StaticSizedArrayLookupIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  // See RMLRMO why we just return the element type.
  override def result: CoordI[cI] = elementType
}
*/
// mig: struct RuntimeSizedArrayLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayLookupIE<'s, 'i, R> {
	pub array_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub index_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub element_type: CoordI<'s, 'i, R>,
	pub variability: VariabilityI,
}
// mig: impl RuntimeSizedArrayLookupIE
/*
case class RuntimeSizedArrayLookupIE(
  arrayExpr: ReferenceExpressionIE,
//  arrayType: RuntimeSizedArrayIT[cI],
  indexExpr: ReferenceExpressionIE,
  // See RMLRMO for why this is the same ownership as the original field.
  elementType: CoordI[cI],
  // See RMLRMO for why we dont have a targetOwnership field here.
  variability: VariabilityI
) extends AddressExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for RuntimeSizedArrayLookupIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for RuntimeSizedArrayLookupIE` below.)
/*
override def hashCode(): Int = vcurious()
//  vassert(arrayExpr.result.kind == arrayType)
*/
// mig: fn result
impl<'s, 'i, R> RuntimeSizedArrayLookupIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  // See RMLRMO why we just return the element type.
  override def result: CoordI[cI] = elementType
}
*/
// mig: struct ArrayLengthIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ArrayLengthIE<'s, 'i, R> {
	pub array_expr: ReferenceExpressionIE<'s, 'i, R>,
}
// mig: impl ArrayLengthIE
/*
case class ArrayLengthIE(arrayExpr: ReferenceExpressionIE) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ArrayLengthIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ArrayLengthIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> ArrayLengthIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> {
		CoordI {
			ownership: crate::instantiating::ast::types::OwnershipI::MutableShare,
			kind: crate::instantiating::ast::types::KindIT::IntIT(crate::instantiating::ast::types::IntIT { bits: 32, _marker: std::marker::PhantomData }),
		}
	}
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, IntIT(32))
}
*/
// mig: struct ReferenceMemberLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ReferenceMemberLookupIE<'s, 'i, R> {
	pub range: RangeS<'s>,
	pub struct_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub member_name: IVarNameI<'s, 'i, R>,
	pub member_reference: CoordI<'s, 'i, R>,
	pub variability: VariabilityI,
}
// mig: impl ReferenceMemberLookupIE
/*
case class ReferenceMemberLookupIE(
  range: RangeS,
  structExpr: ReferenceExpressionIE,
  memberName: IVarNameI[cI],
  // See RMLRMO for why this is the same ownership as the original field.
  memberReference: CoordI[cI],
  // See RMLRMO for why we dont have a targetOwnership field here.
  variability: VariabilityI
) extends AddressExpressionIE {
  vpass()
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReferenceMemberLookupIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReferenceMemberLookupIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> ReferenceMemberLookupIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  // See RMLRMO why we just return the member type.
  override def result: CoordI[cI] = memberReference
}
*/
// mig: struct AddressMemberLookupIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct AddressMemberLookupIE<'s, 'i, R> {
	pub struct_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub member_name: IVarNameI<'s, 'i, R>,
	pub member_reference: CoordI<'s, 'i, R>,
	pub variability: VariabilityI,
}
// mig: impl AddressMemberLookupIE
/*
case class AddressMemberLookupIE(
  structExpr: ReferenceExpressionIE,
  memberName: IVarNameI[cI],
  // See RMLRMO for why this is the same ownership as the original field.
  memberReference: CoordI[cI],
  variability: VariabilityI
) extends AddressExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for AddressMemberLookupIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for AddressMemberLookupIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> AddressMemberLookupIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  // See RMLRMO why we just return the member type.
  override def result: CoordI[cI] = memberReference
}
*/
// mig: struct InterfaceFunctionCallIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct InterfaceFunctionCallIE<'s, 'i, R> {
	pub super_function_prototype: &'i PrototypeI<'s, 'i, R>,
	pub virtual_param_index: i32,
	pub args: &'i[ReferenceExpressionIE<'s, 'i, R>],
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl InterfaceFunctionCallIE
/*
case class InterfaceFunctionCallIE(
  superFunctionPrototype: PrototypeI[cI],
  virtualParamIndex: Int,
  args: Vector[ReferenceExpressionIE],
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for InterfaceFunctionCallIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for InterfaceFunctionCallIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe: CoordI[cI] = ReferenceResultI(resultReference)
}
*/
// mig: struct ExternFunctionCallIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ExternFunctionCallIE<'s, 'i, R> {
	pub prototype2: PrototypeI<'s, 'i, R>,
	pub args: &'i[ReferenceExpressionIE<'s, 'i, R>],
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl ExternFunctionCallIE
/*
case class ExternFunctionCallIE(
  prototype2: PrototypeI[cI],
  args: Vector[ReferenceExpressionIE],
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ExternFunctionCallIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ExternFunctionCallIE` below.)
/*
override def hashCode(): Int = vcurious()
  // We dont:
  //   vassert(prototype2.fullName.last.templateArgs.isEmpty)
  // because we totally can have extern templates.
  // Will one day be useful for plugins, and we already use it for
  // lock<T>, which is generated by the backend.

  prototype2.id.localName match {
    case ExternFunctionNameI(_, _, _) =>
    case _ => vwat()
  }



//  override def resultRemoveMe = ReferenceResultI(prototype2.returnType)
}
*/
// mig: struct FunctionCallIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct FunctionCallIE<'s, 'i, R> {
	pub callable: PrototypeI<'s, 'i, R>,
	pub args: &'i[ReferenceExpressionIE<'s, 'i, R>],
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl FunctionCallIE
/*
case class FunctionCallIE(
  callable: PrototypeI[cI],
  args: Vector[ReferenceExpressionIE],
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionCallIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionCallIE` below.)
/*
override def hashCode(): Int = vcurious()

  vassert(callable.paramTypes.size == args.size)
  args.map(_.result).zip(callable.paramTypes).foreach({
    case (CoordI(_, NeverIT(_)), _) =>
    case (a, b) => vassert(a == b)
  })

//  override def resultRemoveMe: CoordI[cI] = {
//    ReferenceResultI(callable.returnType)
//  }
}
*/
// mig: struct ReinterpretIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ReinterpretIE<'s, 'i, R> {
	pub expr: ReferenceExpressionIE<'s, 'i, R>,
	pub result_reference: CoordI<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl ReinterpretIE
/*
// A typingpass reinterpret is interpreting a type as a different one which is hammer-equivalent.
// For example, a pack and a struct are the same thing to hammer.
// Also, a closure and a struct are the same thing to hammer.
// But, Compiler attaches different meanings to these things. The typingpass is free to reinterpret
// between hammer-equivalent things as it wants.
case class ReinterpretIE(
  expr: ReferenceExpressionIE,
  resultReference: CoordI[cI],
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReinterpretIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReinterpretIE` below.)
/*
override def hashCode(): Int = vcurious()
  vassert(expr.result != resultReference)

//  override def resultRemoveMe = ReferenceResultI(resultReference)

  expr.result.kind match {
    // Unless it's a Never...
    case NeverIT(_) =>
    case _ => {
      if (resultReference.ownership != expr.result.ownership) {
        // Cant reinterpret to a different ownership!
        vfail("wat");
      }
    }
  }
}
*/
// mig: struct ConstructIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct ConstructIE<'s, 'i, R> {
	pub struct_tt: StructIT<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
	pub args: &'i[ExpressionIE<'s, 'i, R>],
}
// mig: impl ConstructIE
/*
case class ConstructIE(
  structTT: StructIT[cI],
  result: CoordI[cI],
  args: Vector[ExpressionI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstructIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstructIE` below.)
/*
override def hashCode(): Int = vcurious()
  vpass()

//  override def resultRemoveMe = ReferenceResultI(resultReference)
}
*/
// mig: struct NewMutRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct NewMutRuntimeSizedArrayIE<'s, 'i, R> {
	pub array_type: RuntimeSizedArrayIT<'s, 'i, R>,
	pub capacity_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl NewMutRuntimeSizedArrayIE
/*
// NoIE: the functionpointercall's last argument is a Placeholder2,
// it's up to later stages to replace that with an actual index
case class NewMutRuntimeSizedArrayIE(
  arrayType: RuntimeSizedArrayIT[cI],
  capacityExpr: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for NewMutRuntimeSizedArrayIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for NewMutRuntimeSizedArrayIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe: CoordI[cI] = {
//    ReferenceResultI(
//      CoordI[cI](
//        arrayType.mutability match {
//          case MutableI => OwnI
//          case ImmutableI => MutableShareI
//        },
//        arrayType))
//  }
}
*/
// mig: struct StaticArrayFromCallableIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct StaticArrayFromCallableIE<'s, 'i, R> {
	pub array_type: StaticSizedArrayIT<'s, 'i, R>,
	pub generator: ReferenceExpressionIE<'s, 'i, R>,
	pub generator_method: PrototypeI<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl StaticArrayFromCallableIE
/*
case class StaticArrayFromCallableIE(
  arrayType: StaticSizedArrayIT[cI],
  generator: ReferenceExpressionIE,
  generatorMethod: PrototypeI[cI],
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for StaticArrayFromCallableIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for StaticArrayFromCallableIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe: CoordI[cI] = {
//    ReferenceResultI(
//      CoordI[cI](
//        arrayType.mutability match {
//          case MutableI => OwnI
//          case ImmutableI => MutableShareI
//        },
//        arrayType))
//  }
}
*/
// mig: struct DestroyStaticSizedArrayIntoFunctionIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoFunctionIE<'s, 'i, R> {
	pub array_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub array_type: StaticSizedArrayIT<'s, 'i, R>,
	pub consumer: ReferenceExpressionIE<'s, 'i, R>,
	pub consumer_method: PrototypeI<'s, 'i, R>,
}
// mig: impl DestroyStaticSizedArrayIntoFunctionIE
/*
// NoIE: the functionpointercall's last argument is a Placeholder2,
// it's up to later stages to replace that with an actual index
// This returns nothing, as opposed to DrainStaticSizedArray2 which returns a
// sequence of results from the call.
case class DestroyStaticSizedArrayIntoFunctionIE(
  arrayExpr: ReferenceExpressionIE,
  arrayType: StaticSizedArrayIT[cI],
  consumer: ReferenceExpressionIE,
  consumerMethod: PrototypeI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DestroyStaticSizedArrayIntoFunctionIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DestroyStaticSizedArrayIntoFunctionIE` below.)
/*
override def hashCode(): Int = vcurious()
  vassert(consumerMethod.paramTypes.size == 2)
//  vassert(consumerMethod.paramTypes(0) == consumer.result)
  vassert(consumerMethod.paramTypes(1) == arrayType.elementType.coord)

  // See https://github.com/ValeLang/Vale/issues/375
  consumerMethod.returnType.kind match {
    case StructIT(IdI(_, _, StructNameI(StructTemplateNameI(name), _))) => {
      vassert(name.str == "Tup")
    }
    case VoidIT() =>
    case _ => vwat()
  }
*/
// mig: fn result
impl<'s, 'i, R> DestroyStaticSizedArrayIntoFunctionIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> {
		CoordI { ownership: crate::instantiating::ast::types::OwnershipI::MutableShare, kind: KindIT::VoidIT(crate::instantiating::ast::types::VoidIT { _marker: std::marker::PhantomData }) }
	}
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct DestroyStaticSizedArrayIntoLocalsIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsIE<'s, 'i, R> {
	pub expr: ReferenceExpressionIE<'s, 'i, R>,
	pub static_sized_array: StaticSizedArrayIT<'s, 'i, R>,
	pub destination_reference_variables: &'i[ReferenceLocalVariableI<'s, 'i>],
}
// mig: impl DestroyStaticSizedArrayIntoLocalsIE
/*
// We destroy both Share and Own things
// If the struct contains any addressibles, those die immediately and aren't stored
// in the destination variables, which is why it's a list of ReferenceLocalVariable2.
case class DestroyStaticSizedArrayIntoLocalsIE(
  expr: ReferenceExpressionIE,
  staticSizedArray: StaticSizedArrayIT[cI],
  destinationReferenceVariables: Vector[ReferenceLocalVariableI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DestroyStaticSizedArrayIntoLocalsIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DestroyStaticSizedArrayIntoLocalsIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> DestroyStaticSizedArrayIntoLocalsIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())

  vassert(expr.result.kind == staticSizedArray)
}
*/
// mig: struct DestroyMutRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyMutRuntimeSizedArrayIE<'s, 'i, R> {
	pub array_expr: ReferenceExpressionIE<'s, 'i, R>,
}
// mig: impl DestroyMutRuntimeSizedArrayIE
/*
case class DestroyMutRuntimeSizedArrayIE(
  arrayExpr: ReferenceExpressionIE
) extends ReferenceExpressionIE {
*/
// mig: fn result
impl<'s, 'i, R> DestroyMutRuntimeSizedArrayIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct RuntimeSizedArrayCapacityIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct RuntimeSizedArrayCapacityIE<'s, 'i, R> {
	pub array_expr: ReferenceExpressionIE<'s, 'i, R>,
}
// mig: impl RuntimeSizedArrayCapacityIE
/*
case class RuntimeSizedArrayCapacityIE(
  arrayExpr: ReferenceExpressionIE
) extends ReferenceExpressionIE {
*/
// mig: fn result
impl<'s, 'i, R> RuntimeSizedArrayCapacityIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, IntIT(32))
//  override def resultRemoveMe: CoordI[cI] = ReferenceResultI(CoordI[cI](MutableShareI, IntIT(32)))
}
*/
// mig: struct PushRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct PushRuntimeSizedArrayIE<'s, 'i, R> {
	pub array_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub new_element_expr: ReferenceExpressionIE<'s, 'i, R>,
}
// mig: impl PushRuntimeSizedArrayIE
/*
case class PushRuntimeSizedArrayIE(
  arrayExpr: ReferenceExpressionIE,
  //  arrayType: RuntimeSizedArrayIT[cI],
  newElementExpr: ReferenceExpressionIE,
  //  newElementType: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn result
impl<'s, 'i, R> PushRuntimeSizedArrayIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct PopRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct PopRuntimeSizedArrayIE<'s, 'i, R> {
	pub array_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl PopRuntimeSizedArrayIE
/*
case class PopRuntimeSizedArrayIE(
  arrayExpr: ReferenceExpressionIE,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
  private val elementType =
    arrayExpr.result.kind match {
      case contentsRuntimeSizedArrayIT(_, e, _) => e
      case other => vwat(other)
    }
//  override def resultRemoveMe: CoordI[cI] = ReferenceResultI(elementType)
}
*/
// mig: struct InterfaceToInterfaceUpcastIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct InterfaceToInterfaceUpcastIE<'s, 'i, R> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub target_interface: InterfaceIT<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl InterfaceToInterfaceUpcastIE
/*
case class InterfaceToInterfaceUpcastIE(
  innerExpr: ReferenceExpressionIE,
  targetInterface: InterfaceIT[cI],
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for InterfaceToInterfaceUpcastIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for InterfaceToInterfaceUpcastIE` below.)
/*
override def hashCode(): Int = vcurious()
//  def result: ReferenceResultI = {
//    ReferenceResultI(
//      CoordI[cI](
//        innerExpr.result.ownership,
//        targetInterface))
//  }
}
*/
// mig: struct UpcastIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct UpcastIE<'s, 'i, R> {
	pub inner_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub target_interface: InterfaceIT<'s, 'i, R>,
	pub impl_name: IdI<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl UpcastIE
/*
// This used to be StructToInterfaceUpcastIE, and then we added generics.
// Now, it could be that we're upcasting a placeholder to an interface, or a
// placeholder to another placeholder. For all we know, this'll eventually be
// upcasting an int to an int.
// So, the target kind can be anything, not just an interface.
case class UpcastIE(
  innerExpr: ReferenceExpressionIE,
  targetInterface: InterfaceIT[cI],
  // This is the impl we use to allow/permit the upcast. It'll be useful for monomorphization
  // and later on for locating the itable ptr to put in fat pointers.
  implName: IdI[cI, IImplNameI[cI]],
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for UpcastIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for UpcastIE` below.)
/*
override def hashCode(): Int = vcurious()
//  def result: ReferenceResultI = {
//    ReferenceResultI(
//      CoordI[cI](
//        innerExpr.result.ownership,
//        targetSuperKind))
//  }
}
*/
// mig: struct SoftLoadIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct SoftLoadIE<'s, 'i, R> {
	pub expr: AddressExpressionIE<'s, 'i, R>,
	pub target_ownership: OwnershipI,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl SoftLoadIE
/*
// A soft load is one that turns an int&& into an int*. a hard load turns an int* into an int.
// Turns an Addressible(Pointer) into an OwningPointer. Makes the source owning pointer into null

// If the source was an own and target is borrow, that's a point

case class SoftLoadIE(
  expr: AddressExpressionIE,
  targetOwnership: OwnershipI,
  result: CoordI[cI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for SoftLoadIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for SoftLoadIE` below.)
/*
override def hashCode(): Int = vcurious()

  vassert(targetOwnership == result.ownership)

  if (targetOwnership == MutableShareI) {
    vassert(expr.result.ownership != ImmutableShareI)
  }
  vassert(targetOwnership != OwnI) // need to unstackify or destroy to get an owning reference
  // This is just here to try the asserts inside Coord's constructor
  CoordI[cI](targetOwnership, expr.result.kind)

  result.kind match {
    case IntIT(_) | BoolIT() | FloatIT() => {
      vassert(targetOwnership == MutableShareI)
    }
    case _ =>
  }

//  override def resultRemoveMe: CoordI[cI] = {
//    ReferenceResultI(CoordI[cI](targetOwnership, expr.result.kind))
//  }
}
*/
// mig: struct DestroyIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyIE<'s, 'i, R> {
	pub expr: ReferenceExpressionIE<'s, 'i, R>,
	pub struct_tt: StructIT<'s, 'i, R>,
	pub destination_reference_variables: &'i[ReferenceLocalVariableI<'s, 'i>],
}
// mig: impl DestroyIE
/*
// Destroy an object.
// If the struct contains any addressibles, those die immediately and aren't stored
// in the destination variables, which is why it's a list of ReferenceLocalVariable2.
//
// We also destroy shared things with this, see DDSOT.
case class DestroyIE(
  expr: ReferenceExpressionIE,
  structTT: StructIT[cI],
  destinationReferenceVariables: Vector[ReferenceLocalVariableI]
) extends ReferenceExpressionIE {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DestroyIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DestroyIE` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 'i, R> DestroyIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct DestroyImmRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct DestroyImmRuntimeSizedArrayIE<'s, 'i, R> {
	pub array_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub array_type: RuntimeSizedArrayIT<'s, 'i, R>,
	pub consumer: ReferenceExpressionIE<'s, 'i, R>,
	pub consumer_method: PrototypeI<'s, 'i, R>,
}
// mig: impl DestroyImmRuntimeSizedArrayIE
/*
case class DestroyImmRuntimeSizedArrayIE(
  arrayExpr: ReferenceExpressionIE,
  arrayType: RuntimeSizedArrayIT[cI],
  consumer: ReferenceExpressionIE,
  consumerMethod: PrototypeI[cI]
) extends ReferenceExpressionIE {
  arrayType.mutability match {
    case ImmutableI =>
    case _ => vwat()
  }
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for DestroyImmRuntimeSizedArrayIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for DestroyImmRuntimeSizedArrayIE` below.)
/*
override def hashCode(): Int = vcurious()
  vassert(consumerMethod.paramTypes.size == 2)
  vassert(consumerMethod.paramTypes(0) == consumer.result)
  //  vassert(consumerMethod.paramTypes(1) == Program2.intType)
  vassert(consumerMethod.paramTypes(1) == arrayType.elementType.coord)

  // See https://github.com/ValeLang/Vale/issues/375
  consumerMethod.returnType.kind match {
    case VoidIT() =>
  }
*/
// mig: fn result
impl<'s, 'i, R> DestroyImmRuntimeSizedArrayIE<'s, 'i, R> {
	pub fn result(&self) -> CoordI<'s, 'i, R> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct NewImmRuntimeSizedArrayIE
/// Arena-allocated (see @TFITCX) — no equality; mirrors Scala vcurious.
#[derive(Copy, Clone, Debug)]
pub struct NewImmRuntimeSizedArrayIE<'s, 'i, R> {
	pub array_type: RuntimeSizedArrayIT<'s, 'i, R>,
	pub size_expr: ReferenceExpressionIE<'s, 'i, R>,
	pub generator: ReferenceExpressionIE<'s, 'i, R>,
	pub generator_method: PrototypeI<'s, 'i, R>,
	pub result: CoordI<'s, 'i, R>,
}
// mig: impl NewImmRuntimeSizedArrayIE
/*
// NoIE: the functionpointercall's last argument is a Placeholder2,
// it's up to later stages to replace that with an actual index
case class NewImmRuntimeSizedArrayIE(
  arrayType: RuntimeSizedArrayIT[cI],
  sizeExpr: ReferenceExpressionIE,
  generator: ReferenceExpressionIE,
  generatorMethod: PrototypeI[cI],
  result: CoordI[cI]
) extends ReferenceExpressionIE {
  arrayType.mutability match {
    case ImmutableI =>
    case _ => vwat()
  }
  // We dont want to own the generator
  generator.result.ownership match {
    case MutableBorrowI | ImmutableBorrowI | ImmutableShareI | MutableShareI =>
    case other => vwat(other)
  }
  generatorMethod.returnType.ownership match {
    case ImmutableShareI | MutableShareI =>
    case other => vwat(other)
  }
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for NewImmRuntimeSizedArrayIE` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for NewImmRuntimeSizedArrayIE` below.)
/*
override def hashCode(): Int = vcurious()
//  override def resultRemoveMe: CoordI[cI] = {
//    ReferenceResultI(
//      CoordI[cI](
//        arrayType.mutability match {
//          case MutableI => OwnI
//          case ImmutableI => MutableShareI
//        },
//        arrayType))
//  }
}

object referenceExprResultStructName {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<ReferenceExpressionIE>` or inline match.)
/*
  def unapply(expr: ReferenceExpressionIE): Option[StrI] = {
    expr.result.kind match {
      case StructIT(IdI(_, _, StructNameI(StructTemplateNameI(name), _))) => Some(name)
      case _ => None
    }
  }
}

object referenceExprResultKind {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<ReferenceExpressionIE>` or inline match.)
/*
  def unapply(expr: ReferenceExpressionIE): Option[KindIT[cI]] = {
    Some(expr.result.kind)
  }
}
*/