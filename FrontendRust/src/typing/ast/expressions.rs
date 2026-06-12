use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::ast::ast::*;
use crate::typing::types::types::{CoordT, KindT, NeverT, OwnershipT, VoidT};
use crate::typing::types::types::IntT;
use crate::typing::templata::templata::{ITemplataT, MutabilityTemplataT};
use crate::typing::types::types::MutabilityT;
use crate::typing::types::types::RegionT;
use crate::typing::types::types::BoolT;
use crate::typing::types::types::FloatT;
use std::any::Any;
use std::marker::PhantomData;

/*
package dev.vale.typing.ast

//import dev.vale.astronomer.IVarNameA
import dev.vale.typing.env.{ILocalVariableT, ReferenceLocalVariableT}
import dev.vale.typing.names._
import dev.vale.{RangeS, vassert, vcurious, vfail, vpass, vwat}
import dev.vale.typing.types._
import dev.vale._
import dev.vale.postparsing._
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.types._
import dev.vale.typing.templata._
*/

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IExpressionResultT<'s, 't> {
    Reference(ReferenceResultT<'s, 't>),
    Address(AddressResultT<'s, 't>),
}
/*
trait IExpressionResultT  {
*/
impl<'s, 't> IExpressionResultT<'s, 't> where 's: 't {
    pub fn expect_reference(&self) -> ReferenceResultT<'s, 't> {
        match self {
            IExpressionResultT::Reference(r) => *r,
            IExpressionResultT::Address(_) => panic!("Expected a reference as a result, but got an address!"),
        }
    }
    /*
      def expectReference(): ReferenceResultT = {
        this match {
          case r @ ReferenceResultT(_) => r
          case AddressResultT(_) => vfail("Expected a reference as a result, but got an address!")
        }
      }
    */
    pub fn expect_address(&self) -> AddressResultT<'s, 't> {
        match self {
            IExpressionResultT::Address(a) => *a,
            IExpressionResultT::Reference(_) => panic!("Expected an address as a result, but got a reference!"),
        }
    }
    /*
      def expectAddress(): AddressResultT = {
        this match {
          case a @ AddressResultT(_) => a
          case ReferenceResultT(_) => vfail("Expected an address as a result, but got a reference!")
        }
      }
    */
    pub fn underlying_coord(&self) -> CoordT<'s, 't> {
        match self {
            IExpressionResultT::Reference(r) => r.coord,
            IExpressionResultT::Address(a) => a.coord,
        }
    }
    /*
      def underlyingCoord: CoordT
    */
    pub fn kind(&self) -> KindT<'s, 't> {
        match self {
            IExpressionResultT::Reference(r) => panic!("Unimplemented: kind Reference"),
            IExpressionResultT::Address(a) => panic!("Unimplemented: kind Address"),
        }
    }
    /*
      def kind: KindT
    }
    */
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressResultT<'s, 't> { pub coord: CoordT<'s, 't> }
/*
case class AddressResultT(coord: CoordT) extends IExpressionResultT {
*/
impl<'s, 't> AddressResultT<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
*/
    fn underlying_coord(&self) -> CoordT<'s, 't> { panic!("Unimplemented: underlying_coord"); }
/*
  override def underlyingCoord: CoordT = coord
*/
    fn kind(&self) -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  override def kind = coord.kind
}
*/
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceResultT<'s, 't> { pub coord: CoordT<'s, 't> }
/*
case class ReferenceResultT(coord: CoordT) extends IExpressionResultT {
*/
impl<'s, 't> ReferenceResultT<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
*/
    pub fn underlying_coord(&self) -> CoordT<'s, 't> { self.coord }
/*
  override def underlyingCoord: CoordT = coord
*/
    fn kind(&self) -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  override def kind = coord.kind
}
*/
}
/// Value-type (see @TFITCX)
//
// Wrapper holding `ReferenceExpressionTE` / `AddressExpressionTE`. The inner
// expression hierarchies opt out of equality entirely (mirroring Scala's `vcurious`
// equals overrides — see comment above `ReferenceExpressionTE`), so this wrapper
// can't `derive(PartialEq)` either: the derive would call the inner type's eq, which
// doesn't exist. Misuse fails at compile time.
#[derive(Copy, Clone, Debug)]
pub enum ExpressionTE<'s, 't> {
    Reference(ReferenceExpressionTE<'s, 't>),
    Address(AddressExpressionTE<'s, 't>),
}
/*
trait ExpressionT  {
*/
impl<'s, 't> ExpressionTE<'s, 't> where 's: 't {
    pub fn result(&self) -> IExpressionResultT<'s, 't> {
        match self {
            ExpressionTE::Reference(e) => IExpressionResultT::Reference(e.result()),
            ExpressionTE::Address(e) => IExpressionResultT::Address(e.result()),
        }
    }
    /*
      def result: IExpressionResultT
    */
    pub fn kind(&self) -> KindT<'s, 't> {
        match self {
            ExpressionTE::Reference(e) => panic!("Unimplemented: kind Reference"),
            ExpressionTE::Address(e) => panic!("Unimplemented: kind Address"),
        }
    }
    /*
      def kind: KindT
    }
    */
}
/// Arena-allocated (see @TFITCX)
//
// No `PartialEq`/`Hash` derive or impl — opts out of equality entirely, mirroring
// Scala's `override def equals(obj: Any): Boolean = vcurious()` on every expression
// case class in `ast/expressions.scala` (52 occurrences). Scala's `vcurious` panics
// at runtime; Rust's "no impl" gives a strictly stronger compile-time error.
//
// Per @TFITCX this is `Arena-allocated` (lifetime/storage in the typing arena), but
// per @IEOIBZ such types normally implement identity equality via `std::ptr::eq`.
// The expression hierarchy is the exception: it's stored in the arena for memory
// reasons (large, deeply nested trees with `&'t` child pointers) but has no
// identity semantics — two distinct allocations of `ConstantIntTE { value: 5 }` are
// neither `==` (Scala vfails) nor distinguishable by identity (no callers care).
#[derive(Copy, Clone, Debug)]
pub enum ReferenceExpressionTE<'s, 't> {
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
    Transmigrate(&'t TransmigrateTE<'s, 't>),
    Return(&'t ReturnTE<'s, 't>),
    Break(&'t BreakTE),
    Block(&'t BlockTE<'s, 't>),
    Pure(&'t PureTE<'s, 't>),
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
    NewMutRuntimeSizedArray(&'t NewMutRuntimeSizedArrayTE<'s, 't>),
    StaticArrayFromCallable(&'t StaticArrayFromCallableTE<'s, 't>),
    DestroyStaticSizedArrayIntoFunction(&'t DestroyStaticSizedArrayIntoFunctionTE<'s, 't>),
    DestroyStaticSizedArrayIntoLocals(&'t DestroyStaticSizedArrayIntoLocalsTE<'s, 't>),
    DestroyMutRuntimeSizedArray(&'t DestroyMutRuntimeSizedArrayTE<'s, 't>),
    RuntimeSizedArrayCapacity(&'t RuntimeSizedArrayCapacityTE<'s, 't>),
    PushRuntimeSizedArray(&'t PushRuntimeSizedArrayTE<'s, 't>),
    PopRuntimeSizedArray(&'t PopRuntimeSizedArrayTE<'s, 't>),
    InterfaceToInterfaceUpcast(&'t InterfaceToInterfaceUpcastTE<'s, 't>),
    Upcast(&'t UpcastTE<'s, 't>),
    SoftLoad(&'t SoftLoadTE<'s, 't>),
    Destroy(&'t DestroyTE<'s, 't>),
    DestroyImmRuntimeSizedArray(&'t DestroyImmRuntimeSizedArrayTE<'s, 't>),
    NewImmRuntimeSizedArray(&'t NewImmRuntimeSizedArrayTE<'s, 't>),
}
/*
trait ReferenceExpressionTE extends ExpressionT {
*/
impl<'s, 't> ReferenceExpressionTE<'s, 't> where 's: 't {
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        match self {
            ReferenceExpressionTE::LetAndLend(e) => e.result(),
            ReferenceExpressionTE::LockWeak(e) => e.result(),
            ReferenceExpressionTE::BorrowToWeak(e) => e.result(),
            ReferenceExpressionTE::LetNormal(e) => e.result(),
            ReferenceExpressionTE::Unlet(e) => e.result(),
            ReferenceExpressionTE::Discard(e) => e.result(),
            ReferenceExpressionTE::Defer(e) => e.result(),
            ReferenceExpressionTE::If(e) => e.result(),
            ReferenceExpressionTE::While(e) => e.result(),
            ReferenceExpressionTE::Mutate(e) => e.result(),
            ReferenceExpressionTE::Restackify(e) => e.result(),
            ReferenceExpressionTE::Transmigrate(e) => e.result(),
            ReferenceExpressionTE::Return(e) => e.result(),
            ReferenceExpressionTE::Break(e) => e.result(),
            ReferenceExpressionTE::Block(e) => e.result(),
            ReferenceExpressionTE::Pure(e) => e.result(),
            ReferenceExpressionTE::Consecutor(e) => e.result(),
            ReferenceExpressionTE::Tuple(e) => e.result(),
            ReferenceExpressionTE::StaticArrayFromValues(e) => e.result(),
            ReferenceExpressionTE::ArraySize(e) => e.result(),
            ReferenceExpressionTE::IsSameInstance(e) => e.result(),
            ReferenceExpressionTE::AsSubtype(e) => e.result(),
            ReferenceExpressionTE::VoidLiteral(e) => e.result(),
            ReferenceExpressionTE::ConstantInt(e) => e.result(),
            ReferenceExpressionTE::ConstantBool(e) => e.result(),
            ReferenceExpressionTE::ConstantStr(e) => e.result(),
            ReferenceExpressionTE::ConstantFloat(e) => e.result(),
            ReferenceExpressionTE::ArgLookup(e) => e.result(),
            ReferenceExpressionTE::ArrayLength(e) => e.result(),
            ReferenceExpressionTE::InterfaceFunctionCall(e) => e.result(),
            ReferenceExpressionTE::ExternFunctionCall(e) => e.result(),
            ReferenceExpressionTE::FunctionCall(e) => e.result(),
            ReferenceExpressionTE::Reinterpret(e) => e.result(),
            ReferenceExpressionTE::Construct(e) => e.result(),
            ReferenceExpressionTE::NewMutRuntimeSizedArray(e) => e.result(),
            ReferenceExpressionTE::StaticArrayFromCallable(e) => e.result(),
            ReferenceExpressionTE::DestroyStaticSizedArrayIntoFunction(e) => e.result(),
            ReferenceExpressionTE::DestroyStaticSizedArrayIntoLocals(e) => e.result(),
            ReferenceExpressionTE::DestroyMutRuntimeSizedArray(e) => e.result(),
            ReferenceExpressionTE::RuntimeSizedArrayCapacity(e) => e.result(),
            ReferenceExpressionTE::PushRuntimeSizedArray(e) => e.result(),
            ReferenceExpressionTE::PopRuntimeSizedArray(e) => e.result(),
            ReferenceExpressionTE::InterfaceToInterfaceUpcast(e) => e.result(),
            ReferenceExpressionTE::Upcast(e) => e.result(),
            ReferenceExpressionTE::SoftLoad(e) => e.result(),
            ReferenceExpressionTE::Destroy(e) => e.result(),
            ReferenceExpressionTE::DestroyImmRuntimeSizedArray(e) => e.result(),
            ReferenceExpressionTE::NewImmRuntimeSizedArray(e) => e.result(),
        }
    }
    /*
      override def result: ReferenceResultT
    */
    pub fn kind(&self) -> KindT<'s, 't> {
        self.result().coord.kind
    }
    /*
      override def kind = result.coord.kind
    }
    */
}
/// Arena-allocated (see @TFITCX)
#[derive(Copy, Clone, Debug)]
pub enum AddressExpressionTE<'s, 't> {
    LocalLookup(&'t LocalLookupTE<'s, 't>),
    StaticSizedArrayLookup(&'t StaticSizedArrayLookupTE<'s, 't>),
    RuntimeSizedArrayLookup(&'t RuntimeSizedArrayLookupTE<'s, 't>),
    ReferenceMemberLookup(&'t ReferenceMemberLookupTE<'s, 't>),
    AddressMemberLookup(&'t AddressMemberLookupTE<'s, 't>),
}
/*
// This is an Expression2 because we sometimes take an address and throw it
// directly into a struct (closures!), which can have addressible members.
trait AddressExpressionTE extends ExpressionT {
*/
impl<'s, 't> AddressExpressionTE<'s, 't> where 's: 't {
    pub fn result(&self) -> AddressResultT<'s, 't> {
        match self {
            AddressExpressionTE::LocalLookup(e) => e.result(),
            AddressExpressionTE::StaticSizedArrayLookup(e) => e.result(),
            AddressExpressionTE::RuntimeSizedArrayLookup(e) => e.result(),
            AddressExpressionTE::ReferenceMemberLookup(e) => e.result(),
            AddressExpressionTE::AddressMemberLookup(e) => e.result(),
        }
    }
    /*
      override def result: AddressResultT
    */
    pub fn kind(&self) -> KindT<'s, 't> {
        panic!("Unimplemented: kind");
    }
    /*
      override def kind = result.coord.kind
    */
    pub fn range(&self) -> RangeS<'s> {
        match self {
            AddressExpressionTE::LocalLookup(e) => panic!("Unimplemented: range LocalLookup"),
            AddressExpressionTE::StaticSizedArrayLookup(e) => e.range,
            AddressExpressionTE::RuntimeSizedArrayLookup(e) => e.range,
            AddressExpressionTE::ReferenceMemberLookup(e) => e.range,
            AddressExpressionTE::AddressMemberLookup(e) => panic!("Unimplemented: range AddressMemberLookup"),
        }
    }
    /*
      def range: RangeS
    */
    pub fn variability(&self) -> VariabilityT {
        match self {
            AddressExpressionTE::LocalLookup(e) => e.variability(),
            AddressExpressionTE::StaticSizedArrayLookup(e) => e.variability,
            AddressExpressionTE::RuntimeSizedArrayLookup(e) => e.variability,
            AddressExpressionTE::ReferenceMemberLookup(e) => e.variability,
            AddressExpressionTE::AddressMemberLookup(e) => e.variability,
        }
    }
    /*
      // Whether or not we can change where this address points to
      def variability: VariabilityT
    }

    */
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LetAndLendTE<'s, 't>
where 's: 't,
{
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: ReferenceExpressionTE<'s, 't>,
    pub target_ownership: OwnershipT,
}
/*
case class LetAndLendTE(
    variable: ILocalVariableT,
    expr: ReferenceExpressionTE,
  targetOwnership: OwnershipT
) extends ReferenceExpressionTE {
*/
impl<'s, 't> LetAndLendTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> LetAndLendTE<'s, 't> where 's: 't, {
    fn new(
        variable: ILocalVariableT<'s, 't>,
        expr: ReferenceExpressionTE<'s, 't>,
        target_ownership: OwnershipT,
    ) -> LetAndLendTE<'s, 't> { panic!("Unimplemented: LetAndLendTE::new"); }
/*
  vassert(variable.coord == expr.result.coord)

  (expr.result.coord.ownership, targetOwnership) match {
    case (ShareT, ShareT) =>
    case (OwnT | BorrowT | WeakT, BorrowT) =>
  }

  expr match {
    case BreakTE(_) | ReturnTE(_) => vwat() // See BRCOBS
    case _ =>
  }

*/
}
impl<'s, 't> LetAndLendTE<'s, 't> {
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        let CoordT { ownership: _old_ownership, region, kind } = self.expr.result().coord;
        ReferenceResultT { coord: CoordT { ownership: self.target_ownership, region, kind } }
    }
/*
  override def result: ReferenceResultT = {
    val CoordT(oldOwnership, region, kind) = expr.result.coord
    ReferenceResultT(CoordT(targetOwnership, region, kind))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LockWeakTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ReferenceExpressionTE<'s, 't>,
    pub result_opt_borrow_type: CoordT<'s, 't>,
    pub some_constructor: &'t PrototypeT<'s, 't>,
    pub none_constructor: &'t PrototypeT<'s, 't>,
    pub some_impl_name: IdT<'s, 't>,
    pub none_impl_name: IdT<'s, 't>,
}
/*
case class LockWeakTE(
  innerExpr: ReferenceExpressionTE,
  // We could just calculate this, but it feels better to let the StructCompiler
  // make it, so we're sure it's created.
  resultOptBorrowType: CoordT,

  // Function to give a borrow ref to to make a Some(borrow ref)
  someConstructor: PrototypeT[IFunctionNameT],
  // Function to make a None of the right type
  noneConstructor: PrototypeT[IFunctionNameT],

  // This is the impl we use to allow/permit the upcast from the some to the none.
  // It'll be useful for monomorphization and later on for locating the itable ptr to put in fat pointers.
  someImplName: IdT[IImplNameT],
  // This is the impl we use to allow/permit the upcast from the some to the none.
  // It'll be useful for monomorphization and later on for locating the itable ptr to put in fat pointers.
  noneImplName: IdT[IImplNameT],
) extends ReferenceExpressionTE {
*/
impl<'s, 't> LockWeakTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { ReferenceResultT { coord: self.result_opt_borrow_type } }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(resultOptBorrowType)
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BorrowToWeakTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ReferenceExpressionTE<'s, 't>,
}
/*
// Turns a borrow ref into a weak ref
// Note that we can also get a weak ref from LocalLoad2'ing a
// borrow ref local into a weak ref.
case class BorrowToWeakTE(
  innerExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
  vassert(innerExpr.result.coord.ownership == BorrowT)

*/
impl<'s, 't> BorrowToWeakTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
  innerExpr.result.coord.ownership match {
    case BorrowT =>
  }

*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: CoordT { ownership: OwnershipT::Weak, region: self.inner_expr.result().coord.region, kind: self.inner_expr.kind() } }
    }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(WeakT, innerExpr.result.coord.region, innerExpr.kind))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LetNormalTE<'s, 't>
where 's: 't,
{
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: ReferenceExpressionTE<'s, 't>,
}
/*
case class LetNormalTE(
    variable: ILocalVariableT,
    expr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> LetNormalTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.expr.result().coord.region,
                kind: KindT::Void(VoidT {}),
            }
        }
    }
/*
  override def result = {
    ReferenceResultT(CoordT(ShareT, expr.result.coord.region, VoidT()))
  }

  expr.kind match {
    case NeverT(_) => // then we can put it into whatever type we want
    case _ => {
      variable.coord.kind match {
        case NeverT(_) => vfail() // can't receive into a never
        case _ => vassert(variable.coord == expr.result.coord)
      }
    }
  }

  expr match {
    case BreakTE(_) | ReturnTE(_) => vwat() // See BRCOBS
    case _ =>
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct UnletTE<'s, 't> {
    pub variable: ILocalVariableT<'s, 't>,
}
/*
// Only ExpressionCompiler.unletLocal should make these
case class UnletTE(variable: ILocalVariableT) extends ReferenceExpressionTE {
*/
impl<'s, 't> UnletTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: self.variable.coord() }
    }
/*
  override def result = ReferenceResultT(variable.coord)

  vpass()
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DiscardTE<'s, 't>
where 's: 't,
{
    pub expr: ReferenceExpressionTE<'s, 't>,
}
/*
// Throws away a reference.
// Unless given to an instruction which consumes it, all borrow and share
// references must eventually hit a Discard2, just like all owning
// references must eventually hit a Destructure2.
// Depending on the backend, it will either be a no-op (like for GC'd backends)
// or a decrement+maybedestruct (like for RC'd backends)
// See DINSIE for why this isnt three instructions, and why we dont have the
// destructor in here for shareds.
case class DiscardTE(
  expr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> DiscardTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.expr.result().coord.region,
                kind: KindT::Void(VoidT),
            }
        }
    }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(ShareT, expr.result.coord.region, VoidT()))
  }

  expr.result.coord.ownership match {
    case BorrowT =>
    case ShareT =>
    case WeakT =>
  }

  expr match {
    case ConsecutorTE(exprs) => {
      exprs.last match {
        case DiscardTE(_) => vwat()
        case _ =>
      }
    }
    case _ =>
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DeferTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ReferenceExpressionTE<'s, 't>,
    pub deferred_expr: ReferenceExpressionTE<'s, 't>,
}
/*
case class DeferTE(
  innerExpr: ReferenceExpressionTE,
  // Every deferred expression should discard its result, IOW, return Void.
  deferredExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> DeferTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: self.inner_expr.result().coord }
    }
/*
  override def result = ReferenceResultT(innerExpr.result.coord)

*/
}
impl<'s, 't> DeferTE<'s, 't> where 's: 't, {
    pub fn new(
        inner_expr: ReferenceExpressionTE<'s, 't>,
        deferred_expr: ReferenceExpressionTE<'s, 't>,
    ) -> DeferTE<'s, 't> {
        // Rust adaptation: Scala class-body vassert moved to constructor.
        let inner_coord = inner_expr.result().coord;
        assert!(deferred_expr.result().coord == CoordT {
            ownership: OwnershipT::Share,
            region: inner_coord.region,
            kind: KindT::Void(VoidT),
        });
        DeferTE { inner_expr, deferred_expr }
    }
/*
  vassert(deferredExpr.result.coord == CoordT(ShareT, innerExpr.result.coord.region, VoidT()))
}


*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct IfTE<'s, 't>
where 's: 't,
{
    pub condition: ReferenceExpressionTE<'s, 't>,
    pub then_call: ReferenceExpressionTE<'s, 't>,
    pub else_call: ReferenceExpressionTE<'s, 't>,
    // Rust adaptation: Scala's `private val commonSupertype` stored as a field.
    pub common_supertype: CoordT<'s, 't>,
}
/*
// Eventually, when we want to do if-let, we'll have a different construct
// entirely. See comment below If2.
// These are blocks because we don't want inner locals to escape.
case class IfTE(
    condition: ReferenceExpressionTE,
    thenCall: ReferenceExpressionTE,
    elseCall: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
impl<'s, 't> IfTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    // Rust adaptation: Scala's class-body `private val`s and runtime `match`
    // assertions live here in the constructor. Only `common_supertype` escapes
    // as a stored field; the other three intermediates are locals.
    pub fn new(
        condition: ReferenceExpressionTE<'s, 't>,
        then_call: ReferenceExpressionTE<'s, 't>,
        else_call: ReferenceExpressionTE<'s, 't>,
    ) -> IfTE<'s, 't> {
        let condition_result_coord = condition.result().coord;
        let then_result_coord = then_call.result().coord;
        let else_result_coord = else_call.result().coord;
        match condition_result_coord {
            CoordT { kind: KindT::Bool(_), ownership: OwnershipT::Share, .. } => {}
            other => panic!("vfail: {:?}", other),
        }
        match (then_result_coord.kind, then_result_coord.kind) {
            (KindT::Never(_), _) => {}
            (_, KindT::Never(_)) => {}
            (a, b) if a == b => {}
            _ => panic!("vwat"),
        }
        let common_supertype = match then_result_coord.kind {
            KindT::Never(_) => else_result_coord,
            _ => then_result_coord,
        };
        IfTE { condition, then_call, else_call, common_supertype }
    }
    /*
    private val conditionResultCoord = condition.result.coord
    private val thenResultCoord = thenCall.result.coord
    private val elseResultCoord = elseCall.result.coord

    conditionResultCoord match {
      case CoordT(ShareT, _, BoolT()) =>
      case other => vfail(other)
    }

    (thenResultCoord.kind, thenResultCoord.kind) match {
      case (NeverT(_), _) =>
      case (_, NeverT(_)) =>
      case (a, b) if a == b =>
      case _ => vwat()
    }

    private val commonSupertype =
      thenResultCoord.kind match {
        case NeverT(_) => elseResultCoord
        case _ => thenResultCoord
      }
    */
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: self.common_supertype }
    }
/*
  override def result = ReferenceResultT(commonSupertype)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct WhileTE<'s, 't>
where 's: 't,
{
    pub block: BlockTE<'s, 't>,
    pub result_coord: CoordT<'s, 't>,
}
/*
// The block is expected to return a boolean (false = stop, true = keep going).
// The block will probably contain an If2(the condition, the body, false)
case class WhileTE(block: BlockTE) extends ReferenceExpressionTE {
  // While loops must always produce void.
  // If we want a foreach/map/whatever construct, the loop should instead
  // add things to a list inside; WhileTE shouldnt do it for it.
*/
impl<'s, 't> WhileTE<'s, 't> {
    // Rust adaptation: Scala's `val resultCoord = ... match { ... }` is a class-body
    // computed val. Rust has no class-body computed fields, so the same computation
    // lives in this constructor and the result is stored on the struct.
    pub fn new(block: BlockTE<'s, 't>) -> WhileTE<'s, 't> {
        let result_coord = match block.result().coord.kind {
            KindT::Void(_) => block.result().coord,
            KindT::Never(NeverT { from_break: true }) => CoordT {
                ownership: OwnershipT::Share,
                region: block.result().coord.region,
                kind: KindT::Void(VoidT),
            },
            KindT::Never(NeverT { from_break: false }) => block.result().coord,
            _ => panic!("vwat"),
        };
        WhileTE { block, result_coord }
    }
    /*
    val resultCoord =
      block.result.coord match {
        case CoordT(_, _, VoidT()) => block.result.coord
        case CoordT(_, region, NeverT(true)) => CoordT(ShareT, region, VoidT())
        case CoordT(_, _, NeverT(false)) => block.result.coord
        case _ => vwat()
      }
    */
/*

*/
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { ReferenceResultT { coord: self.result_coord } }
/*
  override def result = ReferenceResultT(resultCoord)
  vpass()
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct MutateTE<'s, 't>
where 's: 't,
{
    pub destination_expr: AddressExpressionTE<'s, 't>,
    pub source_expr: ReferenceExpressionTE<'s, 't>,
}
/*
case class MutateTE(
  destinationExpr: AddressExpressionTE,
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> MutateTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: self.destination_expr.result().coord }
    }
/*
  override def result = ReferenceResultT(destinationExpr.result.coord)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RestackifyTE<'s, 't>
where 's: 't,
{
    pub variable: ILocalVariableT<'s, 't>,
    pub source_expr: ReferenceExpressionTE<'s, 't>,
}
/*
case class RestackifyTE(
  variable: ILocalVariableT,
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> RestackifyTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: CoordT {
            ownership: OwnershipT::Share,
            region: self.source_expr.result().coord.region,
            kind: KindT::Void(VoidT),
        } }
    }
/*
  override def result = ReferenceResultT(CoordT(ShareT, sourceExpr.result.coord.region, VoidT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct TransmigrateTE<'s, 't>
where 's: 't,
{
    pub source_expr: ReferenceExpressionTE<'s, 't>,
    pub target_region: RegionT,
}
/*
case class TransmigrateTE(
  sourceExpr: ReferenceExpressionTE,
  targetRegion: RegionT
) extends ReferenceExpressionTE {
  vassert(sourceExpr.kind.isPrimitive)
*/
impl<'s, 't> TransmigrateTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(sourceExpr.result.coord.copy(region = targetRegion))
}


*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReturnTE<'s, 't>
where 's: 't,
{
    pub source_expr: ReferenceExpressionTE<'s, 't>,
}
/*
case class ReturnTE(
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> ReturnTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.source_expr.result().coord.region,
                kind: KindT::Never(NeverT { from_break: false }),
            }
        }
    }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(ShareT, sourceExpr.result.coord.region, NeverT(false)))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BreakTE {
    pub region: RegionT,
}
/*
case class BreakTE(region: RegionT) extends ReferenceExpressionTE {
*/
impl BreakTE {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result<'s, 't>(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: CoordT { ownership: OwnershipT::Share, region: self.region, kind: KindT::Never(NeverT { from_break: true }) } }
    }
/*
  override def result = {
    ReferenceResultT(CoordT(ShareT, region, NeverT(true)))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BlockTE<'s, 't>
where 's: 't,
{
    pub inner: ReferenceExpressionTE<'s, 't>,
}
/*
// when we make a closure, we make a struct full of pointers to all our variables
// and the first element is our parent closure
// this can live on the stack, since blocks are limited to this expression
// later we can optimize it to only have the things we use

// Block2 is required to unlet all the variables it introduces.
case class BlockTE(
    inner: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> BlockTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result(&self) -> ReferenceResultT<'s, 't> { self.inner.result() }
/*
  override def result = inner.result
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct PureTE<'s, 't>
where 's: 't,
{
    pub newdefault_region: RegionT,
    pub inner: ReferenceExpressionTE<'s, 't>,
    pub result_type: CoordT<'s, 't>,
}
/*
// A pure block will:
// 1. Create a new region (someday possibly with an allocator)
// 2. Freeze the existing region
// 3. Run the inner code
// 4. Un-freeze the existing region
// 5. Merge (transmigrate) any results from the new region into the existing region
// 6. Destroy the new region
case class PureTE(
//  location: LocationInDenizen,
//  newDefaultRegionName: IdT[INameT],
  newdefaultRegion: RegionT,
//  oldRegionToNewRegion: Vector[(ITemplataT[RegionTemplataType], ITemplataT[RegionTemplataType])],
  inner: ReferenceExpressionTE,
  resultType: CoordT
) extends ReferenceExpressionTE {
  vpass()

*/
impl<'s, 't> PureTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(resultType)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConsecutorTE<'s, 't>
where 's: 't,
{
    pub exprs: &'t [ReferenceExpressionTE<'s, 't>],
}
/*
case class ConsecutorTE(exprs: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
impl<'s, 't> ConsecutorTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ConsecutorTE<'s, 't> where 's: 't, {
    fn new(exprs: &'t [ReferenceExpressionTE<'s, 't>]) -> ConsecutorTE<'s, 't> { panic!("Unimplemented: ConsecutorTE::new"); }
/*
  // There shouldn't be a 0-element consecutor.
  // If we want a consecutor that returns nothing, put a VoidLiteralTE in it.
  vassert(exprs.nonEmpty)

  if (exprs.size > 1) {
    vassert(exprs.init.collect({ case VoidLiteralTE(_) => }).isEmpty)
  }

  // There shouldn't be a 1-element consecutor.
  // This isn't a hard technical requirement, but it does simplify the resulting AST a bit.
  // Call Compiler.consecutive to conform to this.
  vassert(exprs.size >= 2)

  // A consecutor should never contain another consecutor.
  // This isn't a hard technical requirement, but it does simplify the resulting AST a bit.
  // Call Compiler.consecutive to make new consecutors in a way that conforms to this.
  exprs.collect({ case ConsecutorTE(_) => vfail() })

  // Everything but the last should result in a Void or a Never.
  // The last can be anything, even a Void or a Never.
  exprs.init.foreach(expr => {
    expr.kind match {
      case VoidT() | NeverT(_) =>
      case _ => vwat()
    }
  })

  //  // If there's a Never2() anywhere, then the entire block should end in an unreachable
  //  // or panic or something.
  //  if (exprs.exists(_.kind == NeverT())) {
  //    vassert(exprs.last.kind == NeverT())
  //  }
  // Nevermind, we made it so the consecutor's result is Never if there's
  // a Never *anywhere* inside it.

  vassert(exprs.collect({
    case ReturnTE(_) =>
  }).size <= 1)

*/
}
impl<'s, 't> ConsecutorTE<'s, 't> {
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        let never_coord = self.exprs.iter()
            .map(|e| e.result().coord)
            .find(|c| matches!(c, CoordT { ownership: OwnershipT::Share, kind: KindT::Never(_), .. }));
        match never_coord {
            Some(n) => ReferenceResultT { coord: n },
            None => self.exprs.last().unwrap().result(),
        }
    }
/*
  override val result: ReferenceResultT =
    exprs.map(_.result.coord)
        .collectFirst({ case n @ CoordT(ShareT, _, NeverT(_)) => n }) match {
      case Some(n) => ReferenceResultT(n)
      case None => exprs.last.result
    }
*/
    fn last_reference_expr(&self) -> &ReferenceExpressionTE<'s, 't> { panic!("Unimplemented: last_reference_expr"); }
/*
  def lastReferenceExpr = exprs.last
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct TupleTE<'s, 't>
where 's: 't,
{
    pub elements: &'t [ReferenceExpressionTE<'s, 't>],
    pub result_reference: CoordT<'s, 't>,
}
/*
case class TupleTE(
    elements: Vector[ReferenceExpressionTE],
    resultReference: CoordT) extends ReferenceExpressionTE {
*/
impl<'s, 't> TupleTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result(&self) -> ReferenceResultT<'s, 't> { ReferenceResultT { coord: self.result_reference } }
/*
  override def result = ReferenceResultT(resultReference)
}

//// Discards a reference, whether it be owned or borrow or whatever.
//// This is used after panics or other never-returning things, to signal that a certain
//// variable should be considered gone. See AUMAP.
//// This can also be used if theres anything after a panic in a block, like
////   exported func main() int {
////     __panic();
////     println("hi");
////   }
//case class UnreachableMootTE(innerExpr: ReferenceExpressionTE) extends ReferenceExpressionTE {
//  override def equals(obj: Any): Boolean = vcurious();
//override def hashCode(): Int = vcurious()
//  override def result = ReferenceResultT(CoordT(ShareT, NeverT()))
//}
*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticArrayFromValuesTE<'s, 't>
where 's: 't,
{
    pub elements: &'t [ReferenceExpressionTE<'s, 't>],
    pub result_reference: CoordT<'s, 't>,
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
}
/*
case class StaticArrayFromValuesTE(
  elements: Vector[ReferenceExpressionTE],
  resultReference: CoordT,
  arrayType: StaticSizedArrayTT,
) extends ReferenceExpressionTE {
*/
impl<'s, 't> StaticArrayFromValuesTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { ReferenceResultT { coord: self.result_reference } }
/*
  override def result = ReferenceResultT(resultReference)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArraySizeTE<'s, 't>
where 's: 't,
{
    pub array: ReferenceExpressionTE<'s, 't>,
}
/*
case class ArraySizeTE(array: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
impl<'s, 't> ArraySizeTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(CoordT(ShareT, array.result.coord.region, IntT.i32))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct IsSameInstanceTE<'s, 't>
where 's: 't,
{
    pub left: ReferenceExpressionTE<'s, 't>,
    pub right: ReferenceExpressionTE<'s, 't>,
}
/*
// Can we do an === of objects in two regions? It could be pretty useful.
case class IsSameInstanceTE(left: ReferenceExpressionTE, right: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
impl<'s, 't> IsSameInstanceTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> IsSameInstanceTE<'s, 't> where 's: 't, {
    fn new(left: ReferenceExpressionTE<'s, 't>, right: ReferenceExpressionTE<'s, 't>) -> IsSameInstanceTE<'s, 't> { panic!("Unimplemented: IsSameInstanceTE::new"); }
/*
  vassert(left.result.coord == right.result.coord)

*/
}
impl<'s, 't> IsSameInstanceTE<'s, 't> {
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.left.result().coord.region,
                kind: KindT::Bool(BoolT),
            },
        }
    }
/*
  override def result = ReferenceResultT(CoordT(ShareT, left.result.coord.region, BoolT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct AsSubtypeTE<'s, 't>
where 's: 't,
{
    pub source_expr: ReferenceExpressionTE<'s, 't>,
    pub target_type: CoordT<'s, 't>,
    pub result_result_type: CoordT<'s, 't>,
    pub ok_constructor: &'t PrototypeT<'s, 't>,
    pub err_constructor: &'t PrototypeT<'s, 't>,
    pub impl_name: IdT<'s, 't>,
    pub ok_impl_name: IdT<'s, 't>,
    pub err_impl_name: IdT<'s, 't>,
}
/*
case class AsSubtypeTE(
    sourceExpr: ReferenceExpressionTE,
    targetType: CoordT,

    // We could just calculate this, but it feels better to let the StructCompiler
    // make it, so we're sure it's created.
    resultResultType: CoordT,
    // Function to give a borrow ref to to make a Some(borrow ref)
    okConstructor: PrototypeT[IFunctionNameT],
    // Function to make a None of the right type
    errConstructor: PrototypeT[IFunctionNameT],


    // This is the impl we use to allow/permit the downcast. It'll be useful for monomorphization.
    implName: IdT[IImplNameT],

    // These are the impls that we conceptually use to upcast the created Ok/Err to Result.
    // Really they're here so the instantiator can know what impls it needs to instantiate.
    okImplName: IdT[IImplNameT],
    errImplName: IdT[IImplNameT],
) extends ReferenceExpressionTE {
  vpass()

*/
impl<'s, 't> AsSubtypeTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { ReferenceResultT { coord: self.result_result_type } }
/*
  override def result = ReferenceResultT(resultResultType)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct VoidLiteralTE {
    pub region: RegionT,
}
/*
case class VoidLiteralTE(region: RegionT) extends ReferenceExpressionTE {
*/
impl VoidLiteralTE {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result<'s, 't>(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.region,
                kind: KindT::Void(VoidT),
            }
        }
    }
/*
  override def result = ReferenceResultT(CoordT(ShareT, region, VoidT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantIntTE<'s, 't> {
    pub value: ITemplataT<'s, 't>,
    pub bits: i32,
    pub region: RegionT,
}
/*
case class ConstantIntTE(value: ITemplataT[IntegerTemplataType], bits: Int, region: RegionT) extends ReferenceExpressionTE {
*/
impl<'s, 't> ConstantIntTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: CoordT { ownership: OwnershipT::Share, region: self.region, kind: KindT::Int(IntT { bits: self.bits }) } }
    }
/*
  override def result = {
    ReferenceResultT(CoordT(ShareT, region, IntT(bits)))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantBoolTE {
    pub value: bool,
    pub region: RegionT,
}
/*
case class ConstantBoolTE(value: Boolean, region: RegionT) extends ReferenceExpressionTE {
*/
impl ConstantBoolTE {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result<'s, 't>(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: CoordT { ownership: OwnershipT::Share, region: self.region, kind: KindT::Bool(BoolT) } }
    }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, region, BoolT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantStrTE<'s> {
    pub value: StrI<'s>,
    pub region: RegionT,
}
/*
case class ConstantStrTE(value: String, region: RegionT) extends ReferenceExpressionTE {
*/
impl<'s> ConstantStrTE<'s> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result<'t>(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: CoordT { ownership: OwnershipT::Share, region: self.region, kind: KindT::Str(StrT) } }
    }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, region, StrT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantFloatTE {
    pub value: f64,
    pub region: RegionT,
}
/*
case class ConstantFloatTE(value: Double, region: RegionT) extends ReferenceExpressionTE {
*/
impl ConstantFloatTE {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result<'s, 't>(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: CoordT {
            ownership: OwnershipT::Share,
            region: self.region,
            kind: KindT::Float(FloatT),
        } }
    }
/*
  override def result = ReferenceResultT(CoordT(ShareT, region, FloatT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LocalLookupTE<'s, 't> {
    pub range: RangeS<'s>,
    pub local_variable: ILocalVariableT<'s, 't>,
}
/*
case class LocalLookupTE(
  range: RangeS,
  // This is the local variable at the time it was created
  localVariable: ILocalVariableT
) extends AddressExpressionTE {
*/
impl<'s, 't> LocalLookupTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result(&self) -> AddressResultT<'s, 't> {
        AddressResultT { coord: self.local_variable.coord() }
    }
/*
  override def result: AddressResultT = AddressResultT(localVariable.coord)
*/
    pub fn variability(&self) -> VariabilityT { self.local_variable.variability() }
/*
  override def variability: VariabilityT = localVariable.variability
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArgLookupTE<'s, 't> {
    pub param_index: i32,
    pub coord: CoordT<'s, 't>,
}
/*
case class ArgLookupTE(
    paramIndex: Int,
    coord: CoordT
) extends ReferenceExpressionTE {
*/
impl<'s, 't> ArgLookupTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: self.coord }
    }
/*
  override def result = ReferenceResultT(coord)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticSizedArrayLookupTE<'s, 't>
where 's: 't,
{
    pub range: RangeS<'s>,
    pub array_expr: ReferenceExpressionTE<'s, 't>,
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
    pub index_expr: ReferenceExpressionTE<'s, 't>,
    pub element_type: CoordT<'s, 't>,
    pub variability: VariabilityT,
}
/*
case class StaticSizedArrayLookupTE(
  range: RangeS,
    arrayExpr: ReferenceExpressionTE,
    arrayType: StaticSizedArrayTT,
    indexExpr: ReferenceExpressionTE,
  // See RMLRMO for why this is the same ownership as the original field.
    elementType: CoordT,
    // See RMLRMO for why we dont have a targetOwnership field here.
    variability: VariabilityT
) extends AddressExpressionTE {
*/
impl<'s, 't> StaticSizedArrayLookupTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> AddressResultT<'s, 't> { AddressResultT { coord: self.element_type } }
/*
  override def result = {
    // See RMLRMO why we just return the element type.
    AddressResultT(elementType)
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RuntimeSizedArrayLookupTE<'s, 't>
where 's: 't,
{
    pub range: RangeS<'s>,
    pub array_expr: ReferenceExpressionTE<'s, 't>,
    pub array_type: &'t RuntimeSizedArrayTT<'s, 't>,
    pub index_expr: ReferenceExpressionTE<'s, 't>,
    pub variability: VariabilityT,
}
/*
case class RuntimeSizedArrayLookupTE(
  range: RangeS,
    arrayExpr: ReferenceExpressionTE,
    arrayType: RuntimeSizedArrayTT,
    indexExpr: ReferenceExpressionTE,
  // See RMLRMO for why we dont have a targetOwnership field here.
  variability: VariabilityT
) extends AddressExpressionTE {
*/
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> where 's: 't, {
    pub fn new(
        range: RangeS<'s>,
        array_expr: ReferenceExpressionTE<'s, 't>,
        array_type: &'t RuntimeSizedArrayTT<'s, 't>,
        index_expr: ReferenceExpressionTE<'s, 't>,
        variability: VariabilityT,
    ) -> RuntimeSizedArrayLookupTE<'s, 't> {
        assert_eq!(array_expr.result().coord.kind, KindT::RuntimeSizedArray(array_type));
        RuntimeSizedArrayLookupTE { range, array_expr, array_type, index_expr, variability }
    }
/*
  vassert(arrayExpr.result.coord.kind == arrayType)

*/
}
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> {
    pub fn result(&self) -> AddressResultT<'s, 't> {
        // See RMLRMO why we just return the element type.
        AddressResultT { coord: self.array_type.element_type() }
    }
/*
  override def result = {
    // See RMLRMO why we just return the element type.
    AddressResultT(arrayType.elementType)
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ArrayLengthTE<'s, 't>
where 's: 't,
{
    pub array_expr: ReferenceExpressionTE<'s, 't>,
}
/*
case class ArrayLengthTE(arrayExpr: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
impl<'s, 't> ArrayLengthTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.array_expr.result().coord.region,
                kind: KindT::Int(IntT::I32),
            },
        }
    }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, IntT.i32))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReferenceMemberLookupTE<'s, 't>
where 's: 't,
{
    pub range: RangeS<'s>,
    pub struct_expr: ReferenceExpressionTE<'s, 't>,
    pub member_name: IVarNameT<'s, 't>,
    pub member_reference: CoordT<'s, 't>,
    pub variability: VariabilityT,
}
/*
case class ReferenceMemberLookupTE(
    range: RangeS,
    structExpr: ReferenceExpressionTE,
    memberName: IVarNameT,
  // We include this so the instantiator doesn't have to translate the entire struct definition to
  // figure out what type it's pulling out.
  // See RMLRMO for why this is the same ownership as the original field.
    memberReference: CoordT,
    // See RMLRMO for why we dont have a targetOwnership field here.
    variability: VariabilityT) extends AddressExpressionTE {
*/
impl<'s, 't> ReferenceMemberLookupTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result(&self) -> AddressResultT<'s, 't> {
        // See RMLRMO why we just return the member type.
        AddressResultT { coord: self.member_reference }
    }
/*
  override def result = {
    // See RMLRMO why we just return the member type.
    AddressResultT(memberReference)
  }
}
*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct AddressMemberLookupTE<'s, 't>
where 's: 't,
{
    pub range: RangeS<'s>,
    pub struct_expr: ReferenceExpressionTE<'s, 't>,
    pub member_name: IVarNameT<'s, 't>,
    pub result_type2: CoordT<'s, 't>,
    pub variability: VariabilityT,
}
/*
case class AddressMemberLookupTE(
    range: RangeS,
    structExpr: ReferenceExpressionTE,
    memberName: IVarNameT,
    resultType2: CoordT,
    variability: VariabilityT) extends AddressExpressionTE {
*/
impl<'s, 't> AddressMemberLookupTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> AddressResultT<'s, 't> { AddressResultT { coord: self.result_type2 } }
/*
  override def result = AddressResultT(resultType2)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct InterfaceFunctionCallTE<'s, 't>
where 's: 't,
{
    pub super_function_prototype: &'t PrototypeT<'s, 't>,
    pub virtual_param_index: i32,
    pub result_reference: CoordT<'s, 't>,
    pub args: &'t [ReferenceExpressionTE<'s, 't>],
}
/*
case class InterfaceFunctionCallTE(
    superFunctionPrototype: PrototypeT[IFunctionNameT],
    virtualParamIndex: Int,
    resultReference: CoordT,
    args: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
impl<'s, 't> InterfaceFunctionCallTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { ReferenceResultT { coord: self.result_reference } }
/*
  override def result: ReferenceResultT = ReferenceResultT(resultReference)
}

*/
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericParametersInheritance {
  pub num_inherited_generic_parameters: i32,
}
/*
// Metadata for when functions were declared inside a containing denizen, like:
//     extern struct Vec<T> {
//       extern fn with_capacity(size: i64) Vec<T>;
//     }
// This doesn't apply to top-level functions.
case class GenericParametersInheritance(
    // The above `with_capacity` actually lowers to a `fn with_capacity<T>`.
    // This int is how many generic parameters were inherited from the containing struct.
    // Later passes do like to know whether a generic parameter was inherited, to enable Rust
    // interop. The container's template name isn't stored here; if a downstream consumer
    // ever needs it, recompute via `prototype.id.initId(interner)` (the same pattern that
    // populates this field at the definition site in FunctionCompilerCore.makeExternFunction).
    numInheritedGenericParameters: Int,
)
*/
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ExternFunctionCallTE<'s, 't>
where 's: 't,
{
    pub prototype2: &'t PrototypeT<'s, 't>,
    pub args: &'t [ReferenceExpressionTE<'s, 't>],
}
/*
case class ExternFunctionCallTE(
    prototype2: PrototypeT[ExternFunctionNameT],
    args: Vector[ReferenceExpressionTE]
) extends ReferenceExpressionTE {
*/
impl<'s, 't> ExternFunctionCallTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { ReferenceResultT { coord: self.prototype2.return_type } }
/*
  // We dont:
  //   vassert(prototype2.fullName.last.templateArgs.isEmpty)
  // because we totally can have extern templates.
  // Will one day be useful for plugins, and we already use it for
  // lock<T>, which is generated by the backend.

  prototype2.id.localName match {
    case ExternFunctionNameT(_, _, _) =>
    case _ => vwat()
  }



  override def result = ReferenceResultT(prototype2.returnType)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct FunctionCallTE<'s, 't>
where 's: 't,
{
    pub callable: &'t PrototypeT<'s, 't>,
    pub args: &'t [ReferenceExpressionTE<'s, 't>],
    pub return_type: CoordT<'s, 't>,
}
/*
case class FunctionCallTE(
  callable: PrototypeT[IFunctionNameT],
  args: Vector[ReferenceExpressionTE],
  // We have this not only for convenience, but also because sometimes a call's result is in a different region than
  // what the prototype thinks.
  returnType: CoordT
) extends ReferenceExpressionTE {
*/
impl<'s, 't> FunctionCallTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> FunctionCallTE<'s, 't> where 's: 't, {
    fn new(
        callable: &'t PrototypeT<'s, 't>,
        args: &'t [ReferenceExpressionTE<'s, 't>],
        return_type: CoordT<'s, 't>,
    ) -> FunctionCallTE<'s, 't> { panic!("Unimplemented: FunctionCallTE::new"); }
/*
  vassert(callable.paramTypes.size == args.size)
  args.map(_.result.coord).zip(callable.paramTypes).foreach({
    case (CoordT(_, _, NeverT(_)), _) =>
    case (a, b) => vassert(a == b)
  })

*/
}
impl<'s, 't> FunctionCallTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { ReferenceResultT { coord: self.return_type } }
/*
  override def result: ReferenceResultT = ReferenceResultT(returnType)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ReinterpretTE<'s, 't>
where 's: 't,
{
    pub expr: ReferenceExpressionTE<'s, 't>,
    pub result_reference: CoordT<'s, 't>,
}
/*
// A typingpass reinterpret is interpreting a type as a different one which is hammer-equivalent.
// For example, a pack and a struct are the same thing to hammer.
// Also, a closure and a struct are the same thing to hammer.
// But, Compiler attaches different meanings to these things. The typingpass is free to reinterpret
// between hammer-equivalent things as it wants.
case class ReinterpretTE(
    expr: ReferenceExpressionTE,
    resultReference: CoordT) extends ReferenceExpressionTE {
*/
impl<'s, 't> ReinterpretTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ReinterpretTE<'s, 't> where 's: 't, {
    fn new(expr: ReferenceExpressionTE<'s, 't>, result_reference: CoordT<'s, 't>) -> ReinterpretTE<'s, 't> { panic!("Unimplemented: ReinterpretTE::new"); }
/*
  vassert(expr.result.coord != resultReference)

*/
}
impl<'s, 't> ReinterpretTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: self.result_reference }
    }
/*
  override def result = ReferenceResultT(resultReference)

  expr.result.coord.kind match {
    // Unless it's a Never...
    case NeverT(_) =>
    case _ => {
      if (resultReference.ownership != expr.result.coord.ownership) {
        // Cant reinterpret to a different ownership!
        vfail("wat");
      }
    }
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstructTE<'s, 't>
where 's: 't,
{
    pub struct_tt: &'t StructTT<'s, 't>,
    pub result_reference: CoordT<'s, 't>,
    pub args: &'t [ExpressionTE<'s, 't>],
}
/*
case class ConstructTE(
    structTT: StructTT,
    resultReference: CoordT,
    args: Vector[ExpressionT],
) extends ReferenceExpressionTE {
*/
impl<'s, 't> ConstructTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { ReferenceResultT { coord: self.result_reference } }
/*
  vpass()

  override def result = ReferenceResultT(resultReference)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct NewMutRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_type: &'t RuntimeSizedArrayTT<'s, 't>,
    pub region: RegionT,
    pub capacity_expr: ReferenceExpressionTE<'s, 't>,
}
/*
// Note: the functionpointercall's last argument is a Placeholder2,
// it's up to later stages to replace that with an actual index
case class NewMutRuntimeSizedArrayTE(
  arrayType: RuntimeSizedArrayTT,
  region: RegionT,
  capacityExpr: ReferenceExpressionTE,
) extends ReferenceExpressionTE {
*/
impl<'s, 't> NewMutRuntimeSizedArrayTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        let ownership = match self.array_type.mutability() {
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => OwnershipT::Own,
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => OwnershipT::Share,
            ITemplataT::Placeholder(_) => panic!("vimpl"),
            _ => panic!("vwat"),
        };
        ReferenceResultT {
            coord: CoordT {
                ownership,
                region: self.region,
                kind: KindT::RuntimeSizedArray(self.array_type),
            },
        }
    }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(
      CoordT(
        arrayType.mutability match {
          case MutabilityTemplataT(MutableT) => OwnT
          case MutabilityTemplataT(ImmutableT) => ShareT
          case PlaceholderTemplataT(idT, MutabilityTemplataType()) => vimpl()
        },
        region,
        arrayType))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct StaticArrayFromCallableTE<'s, 't>
where 's: 't,
{
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
    pub region: RegionT,
    pub generator: ReferenceExpressionTE<'s, 't>,
    pub generator_method: &'t PrototypeT<'s, 't>,
}
/*
case class StaticArrayFromCallableTE(
  arrayType: StaticSizedArrayTT,
  region: RegionT,
  generator: ReferenceExpressionTE,
  generatorMethod: PrototypeT[IFunctionNameT],
) extends ReferenceExpressionTE {
*/
impl<'s, 't> StaticArrayFromCallableTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        let ownership = match self.array_type.mutability() {
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => OwnershipT::Own,
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => OwnershipT::Share,
            ITemplataT::Placeholder(_) => panic!("Unimplemented: StaticArrayFromCallableTE result PlaceholderTemplataT"),
            _ => panic!("vwat"),
        };
        ReferenceResultT { coord: CoordT { ownership, region: self.region, kind: KindT::StaticSizedArray(self.array_type) } }
    }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(
      CoordT(
        arrayType.mutability match {
          case MutabilityTemplataT(MutableT) => OwnT
          case MutabilityTemplataT(ImmutableT) => ShareT
          case PlaceholderTemplataT(idT, MutabilityTemplataType()) => vimpl()
        },
        region,
        arrayType))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyStaticSizedArrayIntoFunctionTE<'s, 't>
where 's: 't,
{
    pub array_expr: ReferenceExpressionTE<'s, 't>,
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
    pub consumer: ReferenceExpressionTE<'s, 't>,
    pub consumer_method: &'t PrototypeT<'s, 't>,
}
/*
// Note: the functionpointercall's last argument is a Placeholder2,
// it's up to later stages to replace that with an actual index
// This returns nothing, as opposed to DrainStaticSizedArray2 which returns a
// sequence of results from the call.
case class DestroyStaticSizedArrayIntoFunctionTE(
    arrayExpr: ReferenceExpressionTE,
    arrayType: StaticSizedArrayTT,
    consumer: ReferenceExpressionTE,
    consumerMethod: PrototypeT[IFunctionNameT]) extends ReferenceExpressionTE {
*/
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> where 's: 't, {
    fn new(
        array_expr: ReferenceExpressionTE<'s, 't>,
        array_type: &'t StaticSizedArrayTT<'s, 't>,
        consumer: ReferenceExpressionTE<'s, 't>,
        consumer_method: &'t PrototypeT<'s, 't>,
    ) -> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> { panic!("Unimplemented: DestroyStaticSizedArrayIntoFunctionTE::new"); }
/*
  vassert(consumerMethod.paramTypes.size == 2)
  vassert(consumerMethod.paramTypes(0) == consumer.result.coord)
  vassert(consumerMethod.paramTypes(1) == arrayType.elementType)

  // See https://github.com/ValeLang/Vale/issues/375
  consumerMethod.returnType.kind match {
    case StructTT(IdT(_, _, StructNameT(StructTemplateNameT(name), _))) => {
      vassert(name.str == "Tup")
    }
    case VoidT() =>
    case _ => vwat()
  }

*/
}
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.array_expr.result().coord.region,
                kind: KindT::Void(VoidT),
            },
        }
    }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyStaticSizedArrayIntoLocalsTE<'s, 't>
where 's: 't,
{
    pub expr: ReferenceExpressionTE<'s, 't>,
    pub static_sized_array: &'t StaticSizedArrayTT<'s, 't>,
    pub destination_reference_variables: &'t [ReferenceLocalVariableT<'s, 't>],
}
/*
// We destroy both Share and Own things
// If the struct contains any addressibles, those die immediately and aren't stored
// in the destination variables, which is why it's a list of ReferenceLocalVariable2.
case class DestroyStaticSizedArrayIntoLocalsTE(
  expr: ReferenceExpressionTE,
  staticSizedArray: StaticSizedArrayTT,
  destinationReferenceVariables: Vector[ReferenceLocalVariableT]
) extends ReferenceExpressionTE {
*/
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: CoordT { ownership: OwnershipT::Share, region: self.expr.result().coord.region, kind: KindT::Void(VoidT) } }
    }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, expr.result.coord.region, VoidT()))

*/
}
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> where 's: 't, {
    fn new(
        expr: ReferenceExpressionTE<'s, 't>,
        static_sized_array: &'t StaticSizedArrayTT<'s, 't>,
        destination_reference_variables: &'t [ReferenceLocalVariableT<'s, 't>],
    ) -> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> { panic!("Unimplemented: DestroyStaticSizedArrayIntoLocalsTE::new"); }
/*
  vassert(expr.kind == staticSizedArray)
  if (expr.result.coord.ownership == BorrowT) {
    vfail("wot")
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyMutRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ReferenceExpressionTE<'s, 't>,
}
/*
case class DestroyMutRuntimeSizedArrayTE(
  arrayExpr: ReferenceExpressionTE,
) extends ReferenceExpressionTE {
*/
impl<'s, 't> DestroyMutRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.array_expr.result().coord.region,
                kind: KindT::Void(VoidT),
            }
        }
    }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct RuntimeSizedArrayCapacityTE<'s, 't>
where 's: 't,
{
    pub array_expr: ReferenceExpressionTE<'s, 't>,
}
/*
case class RuntimeSizedArrayCapacityTE(
  arrayExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> RuntimeSizedArrayCapacityTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.array_expr.result().coord.region,
                kind: KindT::Int(IntT { bits: 32 }),
            },
        }
    }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, IntT(32)))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct PushRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ReferenceExpressionTE<'s, 't>,
    pub new_element_expr: ReferenceExpressionTE<'s, 't>,
}
/*
case class PushRuntimeSizedArrayTE(
  arrayExpr: ReferenceExpressionTE,
//  arrayType: RuntimeSizedArrayTT,
  newElementExpr: ReferenceExpressionTE,
//  newElementType: CoordT,
) extends ReferenceExpressionTE {
*/
impl<'s, 't> PushRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT {
            coord: CoordT {
                ownership: OwnershipT::Share,
                region: self.array_expr.result().coord.region,
                kind: KindT::Void(VoidT),
            },
        }
    }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct PopRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ReferenceExpressionTE<'s, 't>,
    pub element_type: CoordT<'s, 't>,
}
/*
case class PopRuntimeSizedArrayTE(
  arrayExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
  private val elementType =
    arrayExpr.result.coord.kind match {
      case contentsRuntimeSizedArrayTT(_, e, _) => e
      case other => vwat(other)
    }
*/
impl<'s, 't> PopRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: self.element_type }
    }
/*
  override def result: ReferenceResultT = ReferenceResultT(elementType)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct InterfaceToInterfaceUpcastTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ReferenceExpressionTE<'s, 't>,
    pub target_interface: &'t InterfaceTT<'s, 't>,
}
/*
case class InterfaceToInterfaceUpcastTE(
    innerExpr: ReferenceExpressionTE,
    targetInterface: InterfaceTT) extends ReferenceExpressionTE {
*/
impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  def result: ReferenceResultT = {
    ReferenceResultT(
      CoordT(
        innerExpr.result.coord.ownership,
        innerExpr.result.coord.region,
        targetInterface))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct UpcastTE<'s, 't>
where 's: 't,
{
    pub inner_expr: ReferenceExpressionTE<'s, 't>,
    pub target_super_kind: ISuperKindTT<'s, 't>,
    pub impl_name: IdT<'s, 't>,
}
/*
// This used to be StructToInterfaceUpcastTE, and then we added generics.
// Now, it could be that we're upcasting a placeholder to an interface, or a
// placeholder to another placeholder. For all we know, this'll eventually be
// upcasting an int to an int.
// So, the target kind can be anything, not just an interface.
case class UpcastTE(
  innerExpr: ReferenceExpressionTE,
  targetSuperKind: ISuperKindTT,
  // This is the impl we use to allow/permit the upcast. It'll be useful for monomorphization
  // and later on for locating the itable ptr to put in fat pointers.
  implName: IdT[IImplNameT],
) extends ReferenceExpressionTE {
*/
impl<'s, 't> UpcastTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    pub fn result(&self) -> ReferenceResultT<'s, 't> {
        let inner_coord = self.inner_expr.result().coord;
        ReferenceResultT {
            coord: CoordT {
                ownership: inner_coord.ownership,
                region: inner_coord.region,
                kind: self.target_super_kind.into(),
            }
        }
    }
/*
  def result: ReferenceResultT = {
    ReferenceResultT(
      CoordT(
        innerExpr.result.coord.ownership,
        innerExpr.result.coord.region,
        targetSuperKind))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct SoftLoadTE<'s, 't>
where 's: 't,
{
    pub expr: AddressExpressionTE<'s, 't>,
    pub target_ownership: OwnershipT,
}
/*
// A soft load is one that turns an int&& into an int*. a hard load turns an int* into an int.
// Turns an Addressible(Pointer) into an OwningPointer. Makes the source owning pointer into null

// If the source was an own and target is borrow, that's a point

case class SoftLoadTE(
    expr: AddressExpressionTE,
    targetOwnership: OwnershipT
) extends ReferenceExpressionTE {
*/
impl<'s, 't> SoftLoadTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> SoftLoadTE<'s, 't> where 's: 't, {
    fn new(expr: AddressExpressionTE<'s, 't>, target_ownership: OwnershipT) -> SoftLoadTE<'s, 't> { panic!("Unimplemented: SoftLoadTE::new"); }
/*
  vassert((targetOwnership == ShareT) == (expr.result.coord.ownership == ShareT))
  vassert(targetOwnership != OwnT) // need to unstackify or destroy to get an owning reference
  // This is just here to try the asserts inside Coord's constructor
  CoordT(targetOwnership, expr.result.coord.region, expr.result.coord.kind)

*/
}
impl<'s, 't> SoftLoadTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> {
        let addr_result = self.expr.result();
        ReferenceResultT {
            coord: CoordT {
                ownership: self.target_ownership,
                region: addr_result.coord.region,
                kind: addr_result.coord.kind,
            }
        }
    }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(targetOwnership, expr.result.coord.region, expr.result.coord.kind))
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyTE<'s, 't>
where 's: 't,
{
    pub expr: ReferenceExpressionTE<'s, 't>,
    pub struct_tt: &'t StructTT<'s, 't>,
    pub destination_reference_variables: &'t [ReferenceLocalVariableT<'s, 't>],
}
/*
// Destroy an object.
// If the struct contains any addressibles, those die immediately and aren't stored
// in the destination variables, which is why it's a list of ReferenceLocalVariable2.
//
// We also destroy shared things with this, see DDSOT.
case class DestroyTE(
    expr: ReferenceExpressionTE,
    structTT: StructTT,
    destinationReferenceVariables: Vector[ReferenceLocalVariableT]
) extends ReferenceExpressionTE {
*/
impl<'s, 't> DestroyTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        ReferenceResultT { coord: CoordT { ownership: OwnershipT::Share, region: self.expr.result().coord.region, kind: KindT::Void(VoidT {}) } }
    }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(ShareT, expr.result.coord.region, VoidT()))
  }

  if (expr.result.coord.ownership == BorrowT) {
    vfail("wot")
  }
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct DestroyImmRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_expr: ReferenceExpressionTE<'s, 't>,
    pub array_type: &'t RuntimeSizedArrayTT<'s, 't>,
    pub consumer: ReferenceExpressionTE<'s, 't>,
    pub consumer_method: &'t PrototypeT<'s, 't>,
}
/*
case class DestroyImmRuntimeSizedArrayTE(
  arrayExpr: ReferenceExpressionTE,
  arrayType: RuntimeSizedArrayTT,
  consumer: ReferenceExpressionTE,
  consumerMethod: PrototypeT[IFunctionNameT],
) extends ReferenceExpressionTE {
  arrayType.mutability match {
    case MutabilityTemplataT(ImmutableT) =>
    case _ => vwat()
  }

*/
impl<'s, 't> DestroyImmRuntimeSizedArrayTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> DestroyImmRuntimeSizedArrayTE<'s, 't> where 's: 't, {
    fn new(
        array_expr: ReferenceExpressionTE<'s, 't>,
        array_type: &'t RuntimeSizedArrayTT<'s, 't>,
        consumer: ReferenceExpressionTE<'s, 't>,
        consumer_method: &'t PrototypeT<'s, 't>,
    ) -> DestroyImmRuntimeSizedArrayTE<'s, 't> { panic!("Unimplemented: DestroyImmRuntimeSizedArrayTE::new"); }
/*
  vassert(consumerMethod.paramTypes.size == 2)
  vassert(consumerMethod.paramTypes(0) == consumer.result.coord)
  //  vassert(consumerMethod.paramTypes(1) == Program2.intType)
  vassert(consumerMethod.paramTypes(1) == arrayType.elementType)

  // See https://github.com/ValeLang/Vale/issues/375
  consumerMethod.returnType.kind match {
    case VoidT() =>
  }

*/
}
impl<'s, 't> DestroyImmRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct NewImmRuntimeSizedArrayTE<'s, 't>
where 's: 't,
{
    pub array_type: &'t RuntimeSizedArrayTT<'s, 't>,
    pub region: RegionT,
    pub size_expr: ReferenceExpressionTE<'s, 't>,
    pub generator: ReferenceExpressionTE<'s, 't>,
    pub generator_method: &'t PrototypeT<'s, 't>,
}
/*
// Note: the functionpointercall's last argument is a Placeholder2,
// it's up to later stages to replace that with an actual index
case class NewImmRuntimeSizedArrayTE(
  arrayType: RuntimeSizedArrayTT,
  region: RegionT,
  sizeExpr: ReferenceExpressionTE,
  generator: ReferenceExpressionTE,
  generatorMethod: PrototypeT[IFunctionNameT],
) extends ReferenceExpressionTE {
  arrayType.mutability match {
    case MutabilityTemplataT(ImmutableT) =>
    case _ => vwat()
  }
  // We dont want to own the generator
  generator.result.coord.ownership match {
    case BorrowT | ShareT =>
    case other => vwat(other)
  }
  generatorMethod.returnType.ownership match {
    case ShareT =>
    case other => vwat(other)
  }

*/
impl<'s, 't> NewImmRuntimeSizedArrayTE<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
override def hashCode(): Int = vcurious()
*/
    fn result(&self) -> ReferenceResultT<'s, 't> {
        let ownership = match self.array_type.mutability() {
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => OwnershipT::Own,
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => OwnershipT::Share,
            ITemplataT::Placeholder(_) => panic!("vimpl"),
            _ => panic!("vwat"),
        };
        ReferenceResultT {
            coord: CoordT {
                ownership,
                region: self.region,
                kind: KindT::RuntimeSizedArray(self.array_type),
            },
        }
    }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(
      CoordT(
        arrayType.mutability match {
          case MutabilityTemplataT(MutableT) => OwnT
          case MutabilityTemplataT(ImmutableT) => ShareT
          case PlaceholderTemplataT(_, MutabilityTemplataType()) => vimpl()
        },
        region,
        arrayType))
  }
}

object referenceExprResultStructName {
*/
}
fn reference_expr_result_struct_name_unapply<'s, 't>(expr: &ReferenceExpressionTE<'s, 't>) -> Option<StrI<'s>> { panic!("Unimplemented: unapply"); }
/*
  def unapply(expr: ReferenceExpressionTE): Option[StrI] = {
    expr.result.coord.kind match {
      case StructTT(IdT(_, _, StructNameT(StructTemplateNameT(name), _))) => Some(name)
      case _ => None
    }
  }
}

object referenceExprResultKind {
*/
fn reference_expr_result_kind_unapply<'s, 't>(expr: &ReferenceExpressionTE<'s, 't>) -> Option<KindT<'s, 't>> { panic!("Unimplemented: unapply"); }
/*
  def unapply(expr: ReferenceExpressionTE): Option[KindT] = {
    Some(expr.result.coord.kind)
  }
}
*/