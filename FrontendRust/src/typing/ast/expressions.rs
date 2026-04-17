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
use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::ast::ast::*;

// mig: trait IExpressionResultT
pub enum IExpressionResultT<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
trait IExpressionResultT  {
*/
// mig: fn expect_reference
fn expression_result_expect_reference<'s, 't>() -> ReferenceResultT<'s, 't> { panic!("Unimplemented: expect_reference"); }
/*
  def expectReference(): ReferenceResultT = {
    this match {
      case r @ ReferenceResultT(_) => r
      case AddressResultT(_) => vfail("Expected a reference as a result, but got an address!")
    }
  }
*/
// mig: fn expect_address
fn expression_result_expect_address<'s, 't>() -> AddressResultT<'s, 't> { panic!("Unimplemented: expect_address"); }
/*
  def expectAddress(): AddressResultT = {
    this match {
      case a @ AddressResultT(_) => a
      case ReferenceResultT(_) => vfail("Expected an address as a result, but got a reference!")
    }
  }
*/
// mig: fn underlying_coord
fn expression_result_underlying_coord<'s, 't>() -> CoordT<'s, 't> { panic!("Unimplemented: underlying_coord"); }
/*
  def underlyingCoord: CoordT
*/
// mig: fn kind
fn expression_result_kind<'s, 't>() -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  def kind: KindT
}
*/
// mig: struct AddressResultT
pub struct AddressResultT<'s, 't> { pub coord: CoordT<'s, 't> }
// mig: impl AddressResultT
impl<'s, 't> AddressResultT<'s, 't> {}
/*
case class AddressResultT(coord: CoordT) extends IExpressionResultT {
*/
// mig: fn equals
impl<'s, 't> AddressResultT<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> AddressResultT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn underlying_coord
impl<'s, 't> AddressResultT<'s, 't> {
    fn underlying_coord(&self) -> CoordT<'s, 't> { panic!("Unimplemented: underlying_coord"); }
}
/*
  override def underlyingCoord: CoordT = coord
*/
// mig: fn kind
impl<'s, 't> AddressResultT<'s, 't> {
    fn kind(&self) -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
}
/*
  override def kind = coord.kind
}
*/
// mig: struct ReferenceResultT
pub struct ReferenceResultT<'s, 't> { pub coord: CoordT<'s, 't> }
// mig: impl ReferenceResultT
impl<'s, 't> ReferenceResultT<'s, 't> {}
/*
case class ReferenceResultT(coord: CoordT) extends IExpressionResultT {
*/
// mig: fn equals
impl<'s, 't> ReferenceResultT<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ReferenceResultT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn underlying_coord
impl<'s, 't> ReferenceResultT<'s, 't> {
    fn underlying_coord(&self) -> CoordT<'s, 't> { panic!("Unimplemented: underlying_coord"); }
}
/*
  override def underlyingCoord: CoordT = coord
*/
// mig: fn kind
impl<'s, 't> ReferenceResultT<'s, 't> {
    fn kind(&self) -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
}
/*
  override def kind = coord.kind
}
*/
// mig: trait ExpressionT
pub enum ExpressionT<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
trait ExpressionT  {
*/
// mig: fn result
fn expression_result<'s, 't>() -> IExpressionResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  def result: IExpressionResultT
*/
// mig: fn kind
fn expression_kind<'s, 't>() -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  def kind: KindT
}
*/
// mig: trait ReferenceExpressionTE
pub enum ReferenceExpressionTE<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
trait ReferenceExpressionTE extends ExpressionT {
*/
// mig: fn result
fn reference_expression_result<'s, 't>() -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT
*/
// mig: fn kind
fn reference_expression_kind<'s, 't>() -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  override def kind = result.coord.kind
}
*/
// mig: trait AddressExpressionTE
pub enum AddressExpressionTE<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
// This is an Expression2 because we sometimes take an address and throw it
// directly into a struct (closures!), which can have addressible members.
trait AddressExpressionTE extends ExpressionT {
*/
// mig: fn result
fn address_expression_result<'s, 't>() -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); }
/*
  override def result: AddressResultT
*/
// mig: fn kind
fn address_expression_kind<'s, 't>() -> KindT<'s, 't> { panic!("Unimplemented: kind"); }
/*
  override def kind = result.coord.kind
*/
// mig: fn range
fn address_expression_range<'s>() -> RangeS<'s> { panic!("Unimplemented: range"); }
/*
  def range: RangeS
*/
// mig: fn variability
fn address_expression_variability<'s, 't>() -> VariabilityT<'s, 't> { panic!("Unimplemented: variability"); }
/*
  // Whether or not we can change where this address points to
  def variability: VariabilityT
}

*/
// mig: struct LetAndLendTE
pub struct LetAndLendTE<'s, 't> { pub variable: ILocalVariableT<'s, 't>, pub expr: ReferenceExpressionTE<'s, 't>, pub target_ownership: OwnershipT }
// mig: impl LetAndLendTE
impl<'s, 't> LetAndLendTE<'s, 't> {}
/*
case class LetAndLendTE(
    variable: ILocalVariableT,
    expr: ReferenceExpressionTE,
  targetOwnership: OwnershipT
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> LetAndLendTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> LetAndLendTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
  override def hashCode(): Int = vcurious()
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
// mig: fn result
impl<'s, 't> LetAndLendTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: ReferenceResultT = {
    val CoordT(oldOwnership, region, kind) = expr.result.coord
    ReferenceResultT(CoordT(targetOwnership, region, kind))
  }
}

*/
// mig: struct LockWeakTE
pub struct LockWeakTE<'s, 't> { pub inner_expr: ReferenceExpressionTE<'s, 't>, pub result_opt_borrow_type: CoordT<'s, 't>, pub some_constructor: PrototypeT<'s, 't>, pub none_constructor: PrototypeT<'s, 't>, pub some_impl_name: IdT<'s, 't>, pub none_impl_name: IdT<'s, 't> }
// mig: impl LockWeakTE
impl<'s, 't> LockWeakTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> LockWeakTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> LockWeakTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> LockWeakTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(resultOptBorrowType)
  }
}

*/
// mig: struct BorrowToWeakTE
pub struct BorrowToWeakTE<'s, 't> { pub inner_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl BorrowToWeakTE
impl<'s, 't> BorrowToWeakTE<'s, 't> {}
/*
// Turns a borrow ref into a weak ref
// Note that we can also get a weak ref from LocalLoad2'ing a
// borrow ref local into a weak ref.
case class BorrowToWeakTE(
  innerExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
  vassert(innerExpr.result.coord.ownership == BorrowT)

*/
// mig: fn equals
impl<'s, 't> BorrowToWeakTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> BorrowToWeakTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
  override def hashCode(): Int = vcurious()
  innerExpr.result.coord.ownership match {
    case BorrowT =>
  }

*/
// mig: fn result
impl<'s, 't> BorrowToWeakTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(WeakT, innerExpr.result.coord.region, innerExpr.kind))
  }
}

*/
// mig: struct LetNormalTE
pub struct LetNormalTE<'s, 't> { pub variable: ILocalVariableT<'s, 't>, pub expr: ReferenceExpressionTE<'s, 't> }
// mig: impl LetNormalTE
impl<'s, 't> LetNormalTE<'s, 't> {}
/*
case class LetNormalTE(
    variable: ILocalVariableT,
    expr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> LetNormalTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> LetNormalTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> LetNormalTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
// mig: struct UnletTE
pub struct UnletTE<'s, 't> { pub variable: ILocalVariableT<'s, 't> }
// mig: impl UnletTE
impl<'s, 't> UnletTE<'s, 't> {}
/*
// Only ExpressionCompiler.unletLocal should make these
case class UnletTE(variable: ILocalVariableT) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> UnletTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> UnletTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> UnletTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = ReferenceResultT(variable.coord)

  vpass()
}

*/
// mig: struct DiscardTE
pub struct DiscardTE<'s, 't> { pub expr: ReferenceExpressionTE<'s, 't> }
// mig: impl DiscardTE
impl<'s, 't> DiscardTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> DiscardTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> DiscardTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> DiscardTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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
// mig: struct DeferTE
pub struct DeferTE<'s, 't> { pub inner_expr: ReferenceExpressionTE<'s, 't>, pub deferred_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl DeferTE
impl<'s, 't> DeferTE<'s, 't> {}
/*
case class DeferTE(
  innerExpr: ReferenceExpressionTE,
  // Every deferred expression should discard its result, IOW, return Void.
  deferredExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> DeferTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> DeferTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> DeferTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = ReferenceResultT(innerExpr.result.coord)

  vassert(deferredExpr.result.coord == CoordT(ShareT, innerExpr.result.coord.region, VoidT()))
}


*/
// mig: struct IfTE
pub struct IfTE<'s, 't> { pub condition: ReferenceExpressionTE<'s, 't>, pub then_call: ReferenceExpressionTE<'s, 't>, pub else_call: ReferenceExpressionTE<'s, 't> }
// mig: impl IfTE
impl<'s, 't> IfTE<'s, 't> {}
/*
// Eventually, when we want to do if-let, we'll have a different construct
// entirely. See comment below If2.
// These are blocks because we don't want inner locals to escape.
case class IfTE(
    condition: ReferenceExpressionTE,
    thenCall: ReferenceExpressionTE,
    elseCall: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> IfTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> IfTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> IfTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
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

  override def result = ReferenceResultT(commonSupertype)
}

*/
// mig: struct WhileTE
pub struct WhileTE<'s, 't> { pub block: BlockTE<'s, 't> }
// mig: impl WhileTE
impl<'s, 't> WhileTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> WhileTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> WhileTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> WhileTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = ReferenceResultT(resultCoord)
  vpass()
}

*/
// mig: struct MutateTE
pub struct MutateTE<'s, 't> { pub destination_expr: AddressExpressionTE<'s, 't>, pub source_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl MutateTE
impl<'s, 't> MutateTE<'s, 't> {}
/*
case class MutateTE(
  destinationExpr: AddressExpressionTE,
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> MutateTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> MutateTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> MutateTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = ReferenceResultT(destinationExpr.result.coord)
}

*/
// mig: struct RestackifyTE
pub struct RestackifyTE<'s, 't> { pub variable: ILocalVariableT<'s, 't>, pub source_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl RestackifyTE
impl<'s, 't> RestackifyTE<'s, 't> {}
/*
case class RestackifyTE(
  variable: ILocalVariableT,
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> RestackifyTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> RestackifyTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> RestackifyTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = ReferenceResultT(CoordT(ShareT, sourceExpr.result.coord.region, VoidT()))
}

*/
// mig: struct TransmigrateTE
pub struct TransmigrateTE<'s, 't> { pub source_expr: ReferenceExpressionTE<'s, 't>, pub target_region: RegionT }
// mig: impl TransmigrateTE
impl<'s, 't> TransmigrateTE<'s, 't> {}
/*
case class TransmigrateTE(
  sourceExpr: ReferenceExpressionTE,
  targetRegion: RegionT
) extends ReferenceExpressionTE {
  vassert(sourceExpr.kind.isPrimitive)
*/
// mig: fn equals
impl<'s, 't> TransmigrateTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> TransmigrateTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> TransmigrateTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = ReferenceResultT(sourceExpr.result.coord.copy(region = targetRegion))
}


*/
// mig: struct ReturnTE
pub struct ReturnTE<'s, 't> { pub source_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl ReturnTE
impl<'s, 't> ReturnTE<'s, 't> {}
/*
case class ReturnTE(
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ReturnTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ReturnTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ReturnTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(ShareT, sourceExpr.result.coord.region, NeverT(false)))
  }
}

*/
// mig: struct BreakTE
pub struct BreakTE<'s, 't> { pub region: RegionT, pub _phantom: std::marker::PhantomData<(&'s (), &'t ())> }
// mig: impl BreakTE
impl<'s, 't> BreakTE<'s, 't> {}
/*
case class BreakTE(region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> BreakTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> BreakTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> BreakTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = {
    ReferenceResultT(CoordT(ShareT, region, NeverT(true)))
  }
}

*/
// mig: struct BlockTE
pub struct BlockTE<'s, 't> { pub inner: ReferenceExpressionTE<'s, 't> }
// mig: impl BlockTE
impl<'s, 't> BlockTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> BlockTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> BlockTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> BlockTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = inner.result
}

*/
// mig: struct PureTE
pub struct PureTE<'s, 't> { pub newdefault_region: RegionT, pub inner: ReferenceExpressionTE<'s, 't>, pub result_type: CoordT<'s, 't> }
// mig: impl PureTE
impl<'s, 't> PureTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> PureTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> PureTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> PureTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: ReferenceResultT = ReferenceResultT(resultType)
}

*/
// mig: struct ConsecutorTE
pub struct ConsecutorTE<'s, 't> { pub exprs: Vec<ReferenceExpressionTE<'s, 't>> }
// mig: impl ConsecutorTE
impl<'s, 't> ConsecutorTE<'s, 't> {}
/*
case class ConsecutorTE(exprs: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ConsecutorTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ConsecutorTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ConsecutorTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
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

  override val result: ReferenceResultT =
    exprs.map(_.result.coord)
        .collectFirst({ case n @ CoordT(ShareT, _, NeverT(_)) => n }) match {
      case Some(n) => ReferenceResultT(n)
      case None => exprs.last.result
    }
*/
// mig: fn last_reference_expr
impl<'s, 't> ConsecutorTE<'s, 't> {
    fn last_reference_expr(&self) -> &ReferenceExpressionTE<'s, 't> { panic!("Unimplemented: last_reference_expr"); }
}
/*
  def lastReferenceExpr = exprs.last
}

*/
// mig: struct TupleTE
pub struct TupleTE<'s, 't> { pub elements: Vec<ReferenceExpressionTE<'s, 't>>, pub result_reference: CoordT<'s, 't> }
// mig: impl TupleTE
impl<'s, 't> TupleTE<'s, 't> {}
/*
case class TupleTE(
    elements: Vector[ReferenceExpressionTE],
    resultReference: CoordT) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> TupleTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> TupleTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> TupleTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
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
// mig: struct StaticArrayFromValuesTE
pub struct StaticArrayFromValuesTE<'s, 't> { pub elements: Vec<ReferenceExpressionTE<'s, 't>>, pub result_reference: CoordT<'s, 't>, pub array_type: StaticSizedArrayTT<'s, 't> }
// mig: impl StaticArrayFromValuesTE
impl<'s, 't> StaticArrayFromValuesTE<'s, 't> {}
/*
case class StaticArrayFromValuesTE(
  elements: Vector[ReferenceExpressionTE],
  resultReference: CoordT,
  arrayType: StaticSizedArrayTT,
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> StaticArrayFromValuesTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> StaticArrayFromValuesTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> StaticArrayFromValuesTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = ReferenceResultT(resultReference)
}

*/
// mig: struct ArraySizeTE
pub struct ArraySizeTE<'s, 't> { pub array: ReferenceExpressionTE<'s, 't> }
// mig: impl ArraySizeTE
impl<'s, 't> ArraySizeTE<'s, 't> {}
/*
case class ArraySizeTE(array: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ArraySizeTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ArraySizeTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ArraySizeTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = ReferenceResultT(CoordT(ShareT, array.result.coord.region, IntT.i32))
}

*/
// mig: struct IsSameInstanceTE
pub struct IsSameInstanceTE<'s, 't> { pub left: ReferenceExpressionTE<'s, 't>, pub right: ReferenceExpressionTE<'s, 't> }
// mig: impl IsSameInstanceTE
impl<'s, 't> IsSameInstanceTE<'s, 't> {}
/*
// Can we do an === of objects in two regions? It could be pretty useful.
case class IsSameInstanceTE(left: ReferenceExpressionTE, right: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> IsSameInstanceTE<'s, 't> {
    fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> IsSameInstanceTE<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
}
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> IsSameInstanceTE<'s, 't> {
    fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  vassert(left.result.coord == right.result.coord)

  override def result = ReferenceResultT(CoordT(ShareT, left.result.coord.region, BoolT()))
}

*/
// mig: struct AsSubtypeTE
pub struct AsSubtypeTE<'s, 't> { pub source_expr: ReferenceExpressionTE<'s, 't>, pub target_type: CoordT<'s, 't>, pub result_result_type: CoordT<'s, 't>, pub ok_constructor: PrototypeT<'s, 't>, pub err_constructor: PrototypeT<'s, 't>, pub impl_name: IdT<'s, 't>, pub ok_impl_name: IdT<'s, 't>, pub err_impl_name: IdT<'s, 't> }
// mig: impl AsSubtypeTE
impl<'s, 't> AsSubtypeTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> AsSubtypeTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> AsSubtypeTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> AsSubtypeTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result = ReferenceResultT(resultResultType)
}

*/
// mig: struct VoidLiteralTE
pub struct VoidLiteralTE<'s, 't> { pub region: RegionT, pub _phantom: std::marker::PhantomData<(&'s (), &'t ())> }
// mig: impl VoidLiteralTE
impl<'s, 't> VoidLiteralTE<'s, 't> {}
/*
case class VoidLiteralTE(region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> VoidLiteralTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> VoidLiteralTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> VoidLiteralTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result = ReferenceResultT(CoordT(ShareT, region, VoidT()))
}

*/
// mig: struct ConstantIntTE
pub struct ConstantIntTE<'s, 't> { pub value: ITemplataT<'s, 't>, pub bits: i32, pub region: RegionT, pub _phantom: std::marker::PhantomData<(&'s (), &'t ())> }
// mig: impl ConstantIntTE
impl<'s, 't> ConstantIntTE<'s, 't> {}
/*
case class ConstantIntTE(value: ITemplataT[IntegerTemplataType], bits: Int, region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ConstantIntTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ConstantIntTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ConstantIntTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result = {
    ReferenceResultT(CoordT(ShareT, region, IntT(bits)))
  }
}

*/
// mig: struct ConstantBoolTE
pub struct ConstantBoolTE<'s, 't> { pub value: bool, pub region: RegionT, pub _phantom: std::marker::PhantomData<(&'s (), &'t ())> }
// mig: impl ConstantBoolTE
impl<'s, 't> ConstantBoolTE<'s, 't> {}
/*
case class ConstantBoolTE(value: Boolean, region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ConstantBoolTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ConstantBoolTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ConstantBoolTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, region, BoolT()))
}

*/
// mig: struct ConstantStrTE
pub struct ConstantStrTE<'s, 't> { pub value: String, pub region: RegionT, pub _phantom: std::marker::PhantomData<(&'s (), &'t ())> }
// mig: impl ConstantStrTE
impl<'s, 't> ConstantStrTE<'s, 't> {}
/*
case class ConstantStrTE(value: String, region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ConstantStrTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ConstantStrTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ConstantStrTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, region, StrT()))
}

*/
// mig: struct ConstantFloatTE
pub struct ConstantFloatTE<'s, 't> { pub value: f64, pub region: RegionT, pub _phantom: std::marker::PhantomData<(&'s (), &'t ())> }
// mig: impl ConstantFloatTE
impl<'s, 't> ConstantFloatTE<'s, 't> {}
/*
case class ConstantFloatTE(value: Double, region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ConstantFloatTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ConstantFloatTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ConstantFloatTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result = ReferenceResultT(CoordT(ShareT, region, FloatT()))
}

*/
// mig: struct LocalLookupTE
pub struct LocalLookupTE<'s, 't> { pub range: RangeS<'s>, pub local_variable: ILocalVariableT<'s, 't> }
// mig: impl LocalLookupTE
impl<'s, 't> LocalLookupTE<'s, 't> {}
/*
case class LocalLookupTE(
  range: RangeS,
  // This is the local variable at the time it was created
  localVariable: ILocalVariableT
) extends AddressExpressionTE {
*/
// mig: fn equals
impl<'s, 't> LocalLookupTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> LocalLookupTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> LocalLookupTE<'s, 't> { fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: AddressResultT = AddressResultT(localVariable.coord)
*/
// mig: fn variability
impl<'s, 't> LocalLookupTE<'s, 't> { fn variability(&self) -> VariabilityT<'_, '_> { panic!("Unimplemented: variability"); } }
/*
  override def variability: VariabilityT = localVariable.variability
}

*/
// mig: struct ArgLookupTE
pub struct ArgLookupTE<'s, 't> { pub param_index: i32, pub coord: CoordT<'s, 't> }
// mig: impl ArgLookupTE
impl<'s, 't> ArgLookupTE<'s, 't> {}
/*
case class ArgLookupTE(
    paramIndex: Int,
    coord: CoordT
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ArgLookupTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ArgLookupTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ArgLookupTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result = ReferenceResultT(coord)
}

*/
// mig: struct StaticSizedArrayLookupTE
pub struct StaticSizedArrayLookupTE<'s, 't> { pub range: RangeS<'s>, pub array_expr: ReferenceExpressionTE<'s, 't>, pub array_type: StaticSizedArrayTT<'s, 't>, pub index_expr: ReferenceExpressionTE<'s, 't>, pub element_type: CoordT<'s, 't>, pub variability: VariabilityT<'s, 't> }
// mig: impl StaticSizedArrayLookupTE
impl<'s, 't> StaticSizedArrayLookupTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> StaticSizedArrayLookupTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> StaticSizedArrayLookupTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> StaticSizedArrayLookupTE<'s, 't> { fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result = {
    // See RMLRMO why we just return the element type.
    AddressResultT(elementType)
  }
}

*/
// mig: struct RuntimeSizedArrayLookupTE
pub struct RuntimeSizedArrayLookupTE<'s, 't> { pub range: RangeS<'s>, pub array_expr: ReferenceExpressionTE<'s, 't>, pub array_type: RuntimeSizedArrayTT<'s, 't>, pub index_expr: ReferenceExpressionTE<'s, 't>, pub variability: VariabilityT<'s, 't> }
// mig: impl RuntimeSizedArrayLookupTE
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> RuntimeSizedArrayLookupTE<'s, 't> { fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  vassert(arrayExpr.result.coord.kind == arrayType)

  override def result = {
    // See RMLRMO why we just return the element type.
    AddressResultT(arrayType.elementType)
  }
}

*/
// mig: struct ArrayLengthTE
pub struct ArrayLengthTE<'s, 't> { pub array_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl ArrayLengthTE
impl<'s, 't> ArrayLengthTE<'s, 't> {}
/*
case class ArrayLengthTE(arrayExpr: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ArrayLengthTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ArrayLengthTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ArrayLengthTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, IntT.i32))
}

*/
// mig: struct ReferenceMemberLookupTE
pub struct ReferenceMemberLookupTE<'s, 't> { pub range: RangeS<'s>, pub struct_expr: ReferenceExpressionTE<'s, 't>, pub member_name: IVarNameT<'s, 't>, pub member_reference: CoordT<'s, 't>, pub variability: VariabilityT<'s, 't> }
// mig: impl ReferenceMemberLookupTE
impl<'s, 't> ReferenceMemberLookupTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> ReferenceMemberLookupTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ReferenceMemberLookupTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ReferenceMemberLookupTE<'s, 't> { fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result = {
    // See RMLRMO why we just return the member type.
    AddressResultT(memberReference)
  }
}
*/
// mig: struct AddressMemberLookupTE
pub struct AddressMemberLookupTE<'s, 't> { pub range: RangeS<'s>, pub struct_expr: ReferenceExpressionTE<'s, 't>, pub member_name: IVarNameT<'s, 't>, pub result_type2: CoordT<'s, 't>, pub variability: VariabilityT<'s, 't> }
// mig: impl AddressMemberLookupTE
impl<'s, 't> AddressMemberLookupTE<'s, 't> {}
/*
case class AddressMemberLookupTE(
    range: RangeS,
    structExpr: ReferenceExpressionTE,
    memberName: IVarNameT,
    resultType2: CoordT,
    variability: VariabilityT) extends AddressExpressionTE {
*/
// mig: fn equals
impl<'s, 't> AddressMemberLookupTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> AddressMemberLookupTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> AddressMemberLookupTE<'s, 't> { fn result(&self) -> AddressResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result = AddressResultT(resultType2)
}

*/
// mig: struct InterfaceFunctionCallTE
pub struct InterfaceFunctionCallTE<'s, 't> { pub super_function_prototype: PrototypeT<'s, 't>, pub virtual_param_index: i32, pub result_reference: CoordT<'s, 't>, pub args: Vec<ReferenceExpressionTE<'s, 't>> }
// mig: impl InterfaceFunctionCallTE
impl<'s, 't> InterfaceFunctionCallTE<'s, 't> {}
/*
case class InterfaceFunctionCallTE(
    superFunctionPrototype: PrototypeT[IFunctionNameT],
    virtualParamIndex: Int,
    resultReference: CoordT,
    args: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> InterfaceFunctionCallTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> InterfaceFunctionCallTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> InterfaceFunctionCallTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = ReferenceResultT(resultReference)
}

*/
// mig: struct ExternFunctionCallTE
pub struct ExternFunctionCallTE<'s, 't> { pub prototype2: PrototypeT<'s, 't>, pub args: Vec<ReferenceExpressionTE<'s, 't>> }
// mig: impl ExternFunctionCallTE
impl<'s, 't> ExternFunctionCallTE<'s, 't> {}
/*
case class ExternFunctionCallTE(
    prototype2: PrototypeT[ExternFunctionNameT],
    args: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ExternFunctionCallTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ExternFunctionCallTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ExternFunctionCallTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
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
// mig: struct FunctionCallTE
pub struct FunctionCallTE<'s, 't> { pub callable: PrototypeT<'s, 't>, pub args: Vec<ReferenceExpressionTE<'s, 't>>, pub return_type: CoordT<'s, 't> }
// mig: impl FunctionCallTE
impl<'s, 't> FunctionCallTE<'s, 't> {}
/*
case class FunctionCallTE(
  callable: PrototypeT[IFunctionNameT],
  args: Vector[ReferenceExpressionTE],
  // We have this not only for convenience, but also because sometimes a call's result is in a different region than
  // what the prototype thinks.
  returnType: CoordT
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> FunctionCallTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> FunctionCallTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> FunctionCallTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  vassert(callable.paramTypes.size == args.size)
  args.map(_.result.coord).zip(callable.paramTypes).foreach({
    case (CoordT(_, _, NeverT(_)), _) =>
    case (a, b) => vassert(a == b)
  })

  override def result: ReferenceResultT = ReferenceResultT(returnType)
}

*/
// mig: struct ReinterpretTE
pub struct ReinterpretTE<'s, 't> { pub expr: ReferenceExpressionTE<'s, 't>, pub result_reference: CoordT<'s, 't> }
// mig: impl ReinterpretTE
impl<'s, 't> ReinterpretTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> ReinterpretTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ReinterpretTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ReinterpretTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  vassert(expr.result.coord != resultReference)

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
// mig: struct ConstructTE
pub struct ConstructTE<'s, 't> { pub struct_tt: StructTT<'s, 't>, pub result_reference: CoordT<'s, 't>, pub args: Vec<ExpressionT<'s, 't>> }
// mig: impl ConstructTE
impl<'s, 't> ConstructTE<'s, 't> {}
/*
case class ConstructTE(
    structTT: StructTT,
    resultReference: CoordT,
    args: Vector[ExpressionT],
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> ConstructTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> ConstructTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> ConstructTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  vpass()

  override def result = ReferenceResultT(resultReference)
}

*/
// mig: struct NewMutRuntimeSizedArrayTE
pub struct NewMutRuntimeSizedArrayTE<'s, 't> { pub array_type: RuntimeSizedArrayTT<'s, 't>, pub region: RegionT, pub capacity_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl NewMutRuntimeSizedArrayTE
impl<'s, 't> NewMutRuntimeSizedArrayTE<'s, 't> {}
/*
// Note: the functionpointercall's last argument is a Placeholder2,
// it's up to later stages to replace that with an actual index
case class NewMutRuntimeSizedArrayTE(
  arrayType: RuntimeSizedArrayTT,
  region: RegionT,
  capacityExpr: ReferenceExpressionTE,
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> NewMutRuntimeSizedArrayTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> NewMutRuntimeSizedArrayTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> NewMutRuntimeSizedArrayTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
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
// mig: struct StaticArrayFromCallableTE
pub struct StaticArrayFromCallableTE<'s, 't> { pub array_type: StaticSizedArrayTT<'s, 't>, pub region: RegionT, pub generator: ReferenceExpressionTE<'s, 't>, pub generator_method: PrototypeT<'s, 't> }
// mig: impl StaticArrayFromCallableTE
impl<'s, 't> StaticArrayFromCallableTE<'s, 't> {}
/*
case class StaticArrayFromCallableTE(
  arrayType: StaticSizedArrayTT,
  region: RegionT,
  generator: ReferenceExpressionTE,
  generatorMethod: PrototypeT[IFunctionNameT],
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> StaticArrayFromCallableTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> StaticArrayFromCallableTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> StaticArrayFromCallableTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
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
// mig: struct DestroyStaticSizedArrayIntoFunctionTE
pub struct DestroyStaticSizedArrayIntoFunctionTE<'s, 't> { pub array_expr: ReferenceExpressionTE<'s, 't>, pub array_type: StaticSizedArrayTT<'s, 't>, pub consumer: ReferenceExpressionTE<'s, 't>, pub consumer_method: PrototypeT<'s, 't> }
// mig: impl DestroyStaticSizedArrayIntoFunctionTE
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
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

  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
}

*/
// mig: struct DestroyStaticSizedArrayIntoLocalsTE
pub struct DestroyStaticSizedArrayIntoLocalsTE<'s, 't> { pub expr: ReferenceExpressionTE<'s, 't>, pub static_sized_array: StaticSizedArrayTT<'s, 't>, pub destination_reference_variables: Vec<ReferenceLocalVariableT<'s, 't>> }
// mig: impl DestroyStaticSizedArrayIntoLocalsTE
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, expr.result.coord.region, VoidT()))

  vassert(expr.kind == staticSizedArray)
  if (expr.result.coord.ownership == BorrowT) {
    vfail("wot")
  }
}

*/
// mig: struct DestroyMutRuntimeSizedArrayTE
pub struct DestroyMutRuntimeSizedArrayTE<'s, 't> { pub array_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl DestroyMutRuntimeSizedArrayTE
impl<'s, 't> DestroyMutRuntimeSizedArrayTE<'s, 't> {}
/*
case class DestroyMutRuntimeSizedArrayTE(
  arrayExpr: ReferenceExpressionTE,
) extends ReferenceExpressionTE {
*/
// mig: fn result
impl<'s, 't> DestroyMutRuntimeSizedArrayTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
  }
}

*/
// mig: struct RuntimeSizedArrayCapacityTE
pub struct RuntimeSizedArrayCapacityTE<'s, 't> { pub array_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl RuntimeSizedArrayCapacityTE
impl<'s, 't> RuntimeSizedArrayCapacityTE<'s, 't> {}
/*
case class RuntimeSizedArrayCapacityTE(
  arrayExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn result
impl<'s, 't> RuntimeSizedArrayCapacityTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, IntT(32)))
}

*/
// mig: struct PushRuntimeSizedArrayTE
pub struct PushRuntimeSizedArrayTE<'s, 't> { pub array_expr: ReferenceExpressionTE<'s, 't>, pub new_element_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl PushRuntimeSizedArrayTE
impl<'s, 't> PushRuntimeSizedArrayTE<'s, 't> {}
/*
case class PushRuntimeSizedArrayTE(
  arrayExpr: ReferenceExpressionTE,
//  arrayType: RuntimeSizedArrayTT,
  newElementExpr: ReferenceExpressionTE,
//  newElementType: CoordT,
) extends ReferenceExpressionTE {
*/
// mig: fn result
impl<'s, 't> PushRuntimeSizedArrayTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
}

*/
// mig: struct PopRuntimeSizedArrayTE
pub struct PopRuntimeSizedArrayTE<'s, 't> { pub array_expr: ReferenceExpressionTE<'s, 't> }
// mig: impl PopRuntimeSizedArrayTE
impl<'s, 't> PopRuntimeSizedArrayTE<'s, 't> {}
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
// mig: fn result
impl<'s, 't> PopRuntimeSizedArrayTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = ReferenceResultT(elementType)
}

*/
// mig: struct InterfaceToInterfaceUpcastTE
pub struct InterfaceToInterfaceUpcastTE<'s, 't> { pub inner_expr: ReferenceExpressionTE<'s, 't>, pub target_interface: InterfaceTT<'s, 't> }
// mig: impl InterfaceToInterfaceUpcastTE
impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> {}
/*
case class InterfaceToInterfaceUpcastTE(
    innerExpr: ReferenceExpressionTE,
    targetInterface: InterfaceTT) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> InterfaceToInterfaceUpcastTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
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
// mig: struct UpcastTE
pub struct UpcastTE<'s, 't> { pub inner_expr: ReferenceExpressionTE<'s, 't>, pub target_super_kind: ISuperKindTT<'s, 't>, pub impl_name: IdT<'s, 't> }
// mig: impl UpcastTE
impl<'s, 't> UpcastTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> UpcastTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> UpcastTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> UpcastTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
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
// mig: struct SoftLoadTE
pub struct SoftLoadTE<'s, 't> { pub expr: AddressExpressionTE<'s, 't>, pub target_ownership: OwnershipT }
// mig: impl SoftLoadTE
impl<'s, 't> SoftLoadTE<'s, 't> {}
/*
// A soft load is one that turns an int&& into an int*. a hard load turns an int* into an int.
// Turns an Addressible(Pointer) into an OwningPointer. Makes the source owning pointer into null

// If the source was an own and target is borrow, that's a point

case class SoftLoadTE(
    expr: AddressExpressionTE,
    targetOwnership: OwnershipT
) extends ReferenceExpressionTE {
*/
// mig: fn equals
impl<'s, 't> SoftLoadTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> SoftLoadTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> SoftLoadTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  vassert((targetOwnership == ShareT) == (expr.result.coord.ownership == ShareT))
  vassert(targetOwnership != OwnT) // need to unstackify or destroy to get an owning reference
  // This is just here to try the asserts inside Coord's constructor
  CoordT(targetOwnership, expr.result.coord.region, expr.result.coord.kind)

  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(targetOwnership, expr.result.coord.region, expr.result.coord.kind))
  }
}

*/
// mig: struct DestroyTE
pub struct DestroyTE<'s, 't> { pub expr: ReferenceExpressionTE<'s, 't>, pub struct_tt: StructTT<'s, 't>, pub destination_reference_variables: Vec<ReferenceLocalVariableT<'s, 't>> }
// mig: impl DestroyTE
impl<'s, 't> DestroyTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> DestroyTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> DestroyTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> DestroyTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(ShareT, expr.result.coord.region, VoidT()))
  }

  if (expr.result.coord.ownership == BorrowT) {
    vfail("wot")
  }
}

*/
// mig: struct DestroyImmRuntimeSizedArrayTE
pub struct DestroyImmRuntimeSizedArrayTE<'s, 't> { pub array_expr: ReferenceExpressionTE<'s, 't>, pub array_type: RuntimeSizedArrayTT<'s, 't>, pub consumer: ReferenceExpressionTE<'s, 't>, pub consumer_method: PrototypeT<'s, 't> }
// mig: impl DestroyImmRuntimeSizedArrayTE
impl<'s, 't> DestroyImmRuntimeSizedArrayTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> DestroyImmRuntimeSizedArrayTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> DestroyImmRuntimeSizedArrayTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> DestroyImmRuntimeSizedArrayTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
/*
  vassert(consumerMethod.paramTypes.size == 2)
  vassert(consumerMethod.paramTypes(0) == consumer.result.coord)
  //  vassert(consumerMethod.paramTypes(1) == Program2.intType)
  vassert(consumerMethod.paramTypes(1) == arrayType.elementType)

  // See https://github.com/ValeLang/Vale/issues/375
  consumerMethod.returnType.kind match {
    case VoidT() =>
  }

  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
}

*/
// mig: struct NewImmRuntimeSizedArrayTE
pub struct NewImmRuntimeSizedArrayTE<'s, 't> { pub array_type: RuntimeSizedArrayTT<'s, 't>, pub region: RegionT, pub size_expr: ReferenceExpressionTE<'s, 't>, pub generator: ReferenceExpressionTE<'s, 't>, pub generator_method: PrototypeT<'s, 't> }
// mig: impl NewImmRuntimeSizedArrayTE
impl<'s, 't> NewImmRuntimeSizedArrayTE<'s, 't> {}
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
// mig: fn equals
impl<'s, 't> NewImmRuntimeSizedArrayTE<'s, 't> { fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); } }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
impl<'s, 't> NewImmRuntimeSizedArrayTE<'s, 't> { fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); } }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
impl<'s, 't> NewImmRuntimeSizedArrayTE<'s, 't> { fn result(&self) -> ReferenceResultT<'s, 't> { panic!("Unimplemented: result"); } }
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
// mig: fn unapply
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
// mig: fn unapply
fn reference_expr_result_kind_unapply<'s, 't>(expr: &ReferenceExpressionTE<'s, 't>) -> Option<KindT<'s, 't>> { panic!("Unimplemented: unapply"); }
/*
  def unapply(expr: ReferenceExpressionTE): Option[KindT] = {
    Some(expr.result.coord.kind)
  }
}
*/