use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::ast::ast::*;

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
fn expression_result_expect_reference<'s, 't>() -> ReferenceResultT<'s, 't> { panic!("Unimplemented: expect_reference"); }
/*
  def expectReference(): ReferenceResultT = {
    this match {
      case r @ ReferenceResultT(_) => r
      case AddressResultT(_) => vfail("Expected a reference as a result, but got an address!")
    }
  }
*/
fn expression_result_expect_address<'s, 't>() -> AddressResultT<'s, 't> { panic!("Unimplemented: expect_address"); }
/*
  def expectAddress(): AddressResultT = {
    this match {
      case a @ AddressResultT(_) => a
      case ReferenceResultT(_) => vfail("Expected an address as a result, but got a reference!")
    }
  }
*/
fn expression_result_underlying_coord<'s, 't>() -> CoordT<'s, 't> { panic!("Unimplemented: underlying_coord"); }
/*
  def underlyingCoord: CoordT
*/
fn expression_result_kind<'s, 't>() -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  def kind: KindT
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressResultT<'s, 't> { pub coord: CoordT<'s, 't> }
/*
case class AddressResultT(coord: CoordT) extends IExpressionResultT {
*/
impl<'s, 't> AddressResultT<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> AddressResultT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> AddressResultT<'s, 't> {
    fn underlying_coord(&self) -> CoordT<'s, 't> { panic!("Unimplemented: underlying_coord"); }
/*
  override def underlyingCoord: CoordT = coord
*/
}
impl<'s, 't> AddressResultT<'s, 't> {
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ReferenceResultT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ReferenceResultT<'s, 't> {
    fn underlying_coord(&self) -> CoordT<'s, 't> { panic!("Unimplemented: underlying_coord"); }
/*
  override def underlyingCoord: CoordT = coord
*/
}
impl<'s, 't> ReferenceResultT<'s, 't> {
    fn kind(&self) -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  override def kind = coord.kind
}
*/
}
/// Value-type (see @TFITCX)
//
// Wrapper holding `&'t ReferenceExpressionTE` / `&'t AddressExpressionTE`. The inner
// expression hierarchies opt out of equality entirely (mirroring Scala's `vcurious`
// equals overrides — see comment above `ReferenceExpressionTE`), so this wrapper
// can't `derive(PartialEq)` either: the derive would call the inner type's eq, which
// doesn't exist. Misuse fails at compile time.
#[derive(Copy, Clone, Debug)]
pub enum ExpressionTE<'s, 't> {
    Reference(&'t ReferenceExpressionTE<'s, 't>),
    Address(&'t AddressExpressionTE<'s, 't>),
}
/*
trait ExpressionT  {
*/
fn expression_result<'s, 't>() -> IExpressionResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  def result: IExpressionResultT
*/
fn expression_kind<'s, 't>() -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  def kind: KindT
}
*/
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
#[derive(Debug)]
pub enum ReferenceExpressionTE<'s, 't> {
    LetAndLend(LetAndLendTE<'s, 't>),
    LockWeak(LockWeakTE<'s, 't>),
    BorrowToWeak(BorrowToWeakTE<'s, 't>),
    LetNormal(LetNormalTE<'s, 't>),
    Unlet(UnletTE<'s, 't>),
    Discard(DiscardTE<'s, 't>),
    Defer(DeferTE<'s, 't>),
    If(IfTE<'s, 't>),
    While(WhileTE<'s, 't>),
    Mutate(MutateTE<'s, 't>),
    Restackify(RestackifyTE<'s, 't>),
    Transmigrate(TransmigrateTE<'s, 't>),
    Return(ReturnTE<'s, 't>),
    Break(BreakTE<'s, 't>),
    Block(BlockTE<'s, 't>),
    Pure(PureTE<'s, 't>),
    Consecutor(ConsecutorTE<'s, 't>),
    Tuple(TupleTE<'s, 't>),
    StaticArrayFromValues(StaticArrayFromValuesTE<'s, 't>),
    ArraySize(ArraySizeTE<'s, 't>),
    IsSameInstance(IsSameInstanceTE<'s, 't>),
    AsSubtype(AsSubtypeTE<'s, 't>),
    VoidLiteral(VoidLiteralTE<'s, 't>),
    ConstantInt(ConstantIntTE<'s, 't>),
    ConstantBool(ConstantBoolTE<'s, 't>),
    ConstantStr(ConstantStrTE<'s, 't>),
    ConstantFloat(ConstantFloatTE<'s, 't>),
    ArgLookup(ArgLookupTE<'s, 't>),
    ArrayLength(ArrayLengthTE<'s, 't>),
    InterfaceFunctionCall(InterfaceFunctionCallTE<'s, 't>),
    ExternFunctionCall(ExternFunctionCallTE<'s, 't>),
    FunctionCall(FunctionCallTE<'s, 't>),
    Reinterpret(ReinterpretTE<'s, 't>),
    Construct(ConstructTE<'s, 't>),
    NewMutRuntimeSizedArray(NewMutRuntimeSizedArrayTE<'s, 't>),
    StaticArrayFromCallable(StaticArrayFromCallableTE<'s, 't>),
    DestroyStaticSizedArrayIntoFunction(DestroyStaticSizedArrayIntoFunctionTE<'s, 't>),
    DestroyStaticSizedArrayIntoLocals(DestroyStaticSizedArrayIntoLocalsTE<'s, 't>),
    DestroyMutRuntimeSizedArray(DestroyMutRuntimeSizedArrayTE<'s, 't>),
    RuntimeSizedArrayCapacity(RuntimeSizedArrayCapacityTE<'s, 't>),
    PushRuntimeSizedArray(PushRuntimeSizedArrayTE<'s, 't>),
    PopRuntimeSizedArray(PopRuntimeSizedArrayTE<'s, 't>),
    InterfaceToInterfaceUpcast(InterfaceToInterfaceUpcastTE<'s, 't>),
    Upcast(UpcastTE<'s, 't>),
    SoftLoad(SoftLoadTE<'s, 't>),
    Destroy(DestroyTE<'s, 't>),
    DestroyImmRuntimeSizedArray(DestroyImmRuntimeSizedArrayTE<'s, 't>),
    NewImmRuntimeSizedArray(NewImmRuntimeSizedArrayTE<'s, 't>),
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
}
fn reference_expression_kind<'s, 't>() -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  override def kind = result.coord.kind
}
*/
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub enum AddressExpressionTE<'s, 't> {
    LocalLookup(LocalLookupTE<'s, 't>),
    StaticSizedArrayLookup(StaticSizedArrayLookupTE<'s, 't>),
    RuntimeSizedArrayLookup(RuntimeSizedArrayLookupTE<'s, 't>),
    ReferenceMemberLookup(ReferenceMemberLookupTE<'s, 't>),
    AddressMemberLookup(AddressMemberLookupTE<'s, 't>),
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
}
fn address_expression_kind<'s, 't>() -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  override def kind = result.coord.kind
*/
fn address_expression_range<'s>() -> RangeS<'s> { panic!("Unimplemented: range"); }
/*
  def range: RangeS
*/
fn address_expression_variability() -> VariabilityT { panic!("Unimplemented: variability"); }
/*
  // Whether or not we can change where this address points to
  def variability: VariabilityT
}

*/
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LetAndLendTE<'s, 't>
where 's: 't,
{
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> LetAndLendTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> LetAndLendTE<'s, 't> where 's: 't, {
    fn new(
        variable: ILocalVariableT<'s, 't>,
        expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub inner_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> LockWeakTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> LockWeakTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub inner_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> BorrowToWeakTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
  innerExpr.result.coord.ownership match {
    case BorrowT =>
  }

*/
}
impl<'s, 't> BorrowToWeakTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub expr: &'t ReferenceExpressionTE<'s, 't>,
}
/*
case class LetNormalTE(
    variable: ILocalVariableT,
    expr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> LetNormalTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> LetNormalTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> LetNormalTE<'s, 't> {
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> UnletTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> UnletTE<'s, 't> {
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
    pub expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> DiscardTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> DiscardTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub inner_expr: &'t ReferenceExpressionTE<'s, 't>,
    pub deferred_expr: &'t ReferenceExpressionTE<'s, 't>,
}
/*
case class DeferTE(
  innerExpr: ReferenceExpressionTE,
  // Every deferred expression should discard its result, IOW, return Void.
  deferredExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> DeferTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> DeferTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> DeferTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(innerExpr.result.coord)

*/
}
impl<'s, 't> DeferTE<'s, 't> where 's: 't, {
    fn new(
        inner_expr: &'t ReferenceExpressionTE<'s, 't>,
        deferred_expr: &'t ReferenceExpressionTE<'s, 't>,
    ) -> DeferTE<'s, 't> { panic!("Unimplemented: DeferTE::new"); }
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
    pub condition: &'t ReferenceExpressionTE<'s, 't>,
    pub then_call: &'t ReferenceExpressionTE<'s, 't>,
    pub else_call: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> IfTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> IfTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
}
/*
// The block is expected to return a boolean (false = stop, true = keep going).
// The block will probably contain an If2(the condition, the body, false)
case class WhileTE(block: BlockTE) extends ReferenceExpressionTE {
  // While loops must always produce void.
  // If we want a foreach/map/whatever construct, the loop should instead
  // add things to a list inside; WhileTE shouldnt do it for it.
  val resultCoord =
    block.result.coord match {
      case CoordT(_, _, VoidT()) => block.result.coord
      case CoordT(_, region, NeverT(true)) => CoordT(ShareT, region, VoidT())
      case CoordT(_, _, NeverT(false)) => block.result.coord
      case _ => vwat()
    }

*/
impl<'s, 't> WhileTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> WhileTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> WhileTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub destination_expr: &'t AddressExpressionTE<'s, 't>,
    pub source_expr: &'t ReferenceExpressionTE<'s, 't>,
}
/*
case class MutateTE(
  destinationExpr: AddressExpressionTE,
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> MutateTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> MutateTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> MutateTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub source_expr: &'t ReferenceExpressionTE<'s, 't>,
}
/*
case class RestackifyTE(
  variable: ILocalVariableT,
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> RestackifyTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> RestackifyTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> RestackifyTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub source_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> TransmigrateTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> TransmigrateTE<'s, 't> {
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
    pub source_expr: &'t ReferenceExpressionTE<'s, 't>,
}
/*
case class ReturnTE(
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> ReturnTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ReturnTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ReturnTE<'s, 't> {
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
pub struct BreakTE<'s, 't> {
    pub region: RegionT,
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class BreakTE(region: RegionT) extends ReferenceExpressionTE {
*/
impl<'s, 't> BreakTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> BreakTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> BreakTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub inner: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> BlockTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> BlockTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { self.inner.result() }
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
    pub inner: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> PureTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> PureTE<'s, 't> {
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
    pub exprs: &'t [&'t ReferenceExpressionTE<'s, 't>],
}
/*
case class ConsecutorTE(exprs: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
impl<'s, 't> ConsecutorTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ConsecutorTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ConsecutorTE<'s, 't> where 's: 't, {
    fn new(exprs: &'t [&'t ReferenceExpressionTE<'s, 't>]) -> ConsecutorTE<'s, 't> { panic!("Unimplemented: ConsecutorTE::new"); }
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
    fn result(&self) -> ReferenceResultT<'s, 't> {
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
}
impl<'s, 't> ConsecutorTE<'s, 't> {
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> TupleTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> TupleTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
override def hashCode(): Int = vcurious()
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> StaticArrayFromValuesTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> StaticArrayFromValuesTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub array: &'t ReferenceExpressionTE<'s, 't>,
}
/*
case class ArraySizeTE(array: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
impl<'s, 't> ArraySizeTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ArraySizeTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ArraySizeTE<'s, 't> {
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
    pub left: &'t ReferenceExpressionTE<'s, 't>,
    pub right: &'t ReferenceExpressionTE<'s, 't>,
}
/*
// Can we do an === of objects in two regions? It could be pretty useful.
case class IsSameInstanceTE(left: ReferenceExpressionTE, right: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
impl<'s, 't> IsSameInstanceTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> IsSameInstanceTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> IsSameInstanceTE<'s, 't> where 's: 't, {
    fn new(left: &'t ReferenceExpressionTE<'s, 't>, right: &'t ReferenceExpressionTE<'s, 't>) -> IsSameInstanceTE<'s, 't> { panic!("Unimplemented: IsSameInstanceTE::new"); }
/*
  vassert(left.result.coord == right.result.coord)

*/
}
impl<'s, 't> IsSameInstanceTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub source_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> AsSubtypeTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> AsSubtypeTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(resultResultType)
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct VoidLiteralTE<'s, 't> {
    pub region: RegionT,
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class VoidLiteralTE(region: RegionT) extends ReferenceExpressionTE {
*/
impl<'s, 't> VoidLiteralTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> VoidLiteralTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> VoidLiteralTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ConstantIntTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ConstantIntTE<'s, 't> {
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
pub struct ConstantBoolTE<'s, 't> {
    pub value: bool,
    pub region: RegionT,
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class ConstantBoolTE(value: Boolean, region: RegionT) extends ReferenceExpressionTE {
*/
impl<'s, 't> ConstantBoolTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ConstantBoolTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ConstantBoolTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, region, BoolT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantStrTE<'s, 't> {
    pub value: StrI<'s>,
    pub region: RegionT,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ConstantStrTE(value: String, region: RegionT) extends ReferenceExpressionTE {
*/
impl<'s, 't> ConstantStrTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ConstantStrTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ConstantStrTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, region, StrT()))
}

*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct ConstantFloatTE<'s, 't> {
    pub value: f64,
    pub region: RegionT,
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class ConstantFloatTE(value: Double, region: RegionT) extends ReferenceExpressionTE {
*/
impl<'s, 't> ConstantFloatTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ConstantFloatTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ConstantFloatTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> LocalLookupTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> LocalLookupTE<'s, 't> {
    fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: AddressResultT = AddressResultT(localVariable.coord)
*/
}
impl<'s, 't> LocalLookupTE<'s, 't> {
    fn variability(&self) -> VariabilityT { panic!("Unimplemented: variability"); }
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ArgLookupTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ArgLookupTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub array_expr: &'t ReferenceExpressionTE<'s, 't>,
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
    pub index_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> StaticSizedArrayLookupTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> StaticSizedArrayLookupTE<'s, 't> {
    fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub array_expr: &'t ReferenceExpressionTE<'s, 't>,
    pub array_type: &'t RuntimeSizedArrayTT<'s, 't>,
    pub index_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> where 's: 't, {
    fn new(
        range: RangeS<'s>,
        array_expr: &'t ReferenceExpressionTE<'s, 't>,
        array_type: &'t RuntimeSizedArrayTT<'s, 't>,
        index_expr: &'t ReferenceExpressionTE<'s, 't>,
        variability: VariabilityT,
    ) -> RuntimeSizedArrayLookupTE<'s, 't> { panic!("Unimplemented: RuntimeSizedArrayLookupTE::new"); }
/*
  vassert(arrayExpr.result.coord.kind == arrayType)

*/
}
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> {
    fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub array_expr: &'t ReferenceExpressionTE<'s, 't>,
}
/*
case class ArrayLengthTE(arrayExpr: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
impl<'s, 't> ArrayLengthTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ArrayLengthTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ArrayLengthTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub struct_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ReferenceMemberLookupTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ReferenceMemberLookupTE<'s, 't> {
    fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub struct_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> AddressMemberLookupTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> AddressMemberLookupTE<'s, 't> {
    fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> InterfaceFunctionCallTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> InterfaceFunctionCallTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(resultReference)
}

*/
}
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
    args: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
impl<'s, 't> ExternFunctionCallTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ExternFunctionCallTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ExternFunctionCallTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  // We dont:
  //   vassert(prototype2.fullName.last.templateArgs.isEmpty)
  // because we totally can have extern templates.
  // Will one day be useful for plugins, and we already use it for
  // lock<T>, which is generated by the backend.

  prototype2.id.localName match {
    case ExternFunctionNameT(_, _) =>
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> FunctionCallTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
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
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ReinterpretTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ReinterpretTE<'s, 't> where 's: 't, {
    fn new(expr: &'t ReferenceExpressionTE<'s, 't>, result_reference: CoordT<'s, 't>) -> ReinterpretTE<'s, 't> { panic!("Unimplemented: ReinterpretTE::new"); }
/*
  vassert(expr.result.coord != resultReference)

*/
}
impl<'s, 't> ReinterpretTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ConstructTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> ConstructTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub capacity_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> NewMutRuntimeSizedArrayTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> NewMutRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub generator: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> StaticArrayFromCallableTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> StaticArrayFromCallableTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub array_expr: &'t ReferenceExpressionTE<'s, 't>,
    pub array_type: &'t StaticSizedArrayTT<'s, 't>,
    pub consumer: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> where 's: 't, {
    fn new(
        array_expr: &'t ReferenceExpressionTE<'s, 't>,
        array_type: &'t StaticSizedArrayTT<'s, 't>,
        consumer: &'t ReferenceExpressionTE<'s, 't>,
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
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, expr.result.coord.region, VoidT()))

*/
}
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> where 's: 't, {
    fn new(
        expr: &'t ReferenceExpressionTE<'s, 't>,
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
    pub array_expr: &'t ReferenceExpressionTE<'s, 't>,
}
/*
case class DestroyMutRuntimeSizedArrayTE(
  arrayExpr: ReferenceExpressionTE,
) extends ReferenceExpressionTE {
*/
impl<'s, 't> DestroyMutRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub array_expr: &'t ReferenceExpressionTE<'s, 't>,
}
/*
case class RuntimeSizedArrayCapacityTE(
  arrayExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
impl<'s, 't> RuntimeSizedArrayCapacityTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub array_expr: &'t ReferenceExpressionTE<'s, 't>,
    pub new_element_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub array_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub inner_expr: &'t ReferenceExpressionTE<'s, 't>,
    pub target_interface: &'t InterfaceTT<'s, 't>,
}
/*
case class InterfaceToInterfaceUpcastTE(
    innerExpr: ReferenceExpressionTE,
    targetInterface: InterfaceTT) extends ReferenceExpressionTE {
*/
impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> {
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
    pub inner_expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> UpcastTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> UpcastTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub expr: &'t AddressExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> SoftLoadTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> SoftLoadTE<'s, 't> where 's: 't, {
    fn new(expr: &'t AddressExpressionTE<'s, 't>, target_ownership: OwnershipT) -> SoftLoadTE<'s, 't> { panic!("Unimplemented: SoftLoadTE::new"); }
/*
  vassert((targetOwnership == ShareT) == (expr.result.coord.ownership == ShareT))
  vassert(targetOwnership != OwnT) // need to unstackify or destroy to get an owning reference
  // This is just here to try the asserts inside Coord's constructor
  CoordT(targetOwnership, expr.result.coord.region, expr.result.coord.kind)

*/
}
impl<'s, 't> SoftLoadTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub expr: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> DestroyTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> DestroyTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
    pub array_expr: &'t ReferenceExpressionTE<'s, 't>,
    pub array_type: &'t RuntimeSizedArrayTT<'s, 't>,
    pub consumer: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> DestroyImmRuntimeSizedArrayTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> DestroyImmRuntimeSizedArrayTE<'s, 't> where 's: 't, {
    fn new(
        array_expr: &'t ReferenceExpressionTE<'s, 't>,
        array_type: &'t RuntimeSizedArrayTT<'s, 't>,
        consumer: &'t ReferenceExpressionTE<'s, 't>,
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
    pub size_expr: &'t ReferenceExpressionTE<'s, 't>,
    pub generator: &'t ReferenceExpressionTE<'s, 't>,
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
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> NewImmRuntimeSizedArrayTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
}
impl<'s, 't> NewImmRuntimeSizedArrayTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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