/*
package dev.vale.instantiating.ast

import dev.vale._
import dev.vale.postparsing._
*/
// mig: trait ExpressionIE
pub trait ExpressionIE<'s, 't> {}
/*
trait ExpressionI  {
*/
// mig: fn result
pub fn result(self_: &dyn ExpressionIE<'s, 't>) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
/*
  def result: CoordI[cI]
}
*/
// mig: trait ReferenceExpressionIE
pub trait ReferenceExpressionIE<'s, 't>: ExpressionIE<'s, 't> {}
/*
trait ReferenceExpressionIE extends ExpressionI { }
*/
// mig: trait AddressExpressionIE
pub trait AddressExpressionIE<'s, 't>: ExpressionIE<'s, 't> {}
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct LetAndLendIE<'s, 't> {
	pub variable: ILocalVariableI<'s, 't>,
	pub expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub target_ownership: OwnershipI,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct LockWeakIE<'s, 't> {
	pub inner_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result_opt_borrow_type: CoordI<'s, 't>,
	pub some_constructor: PrototypeI<'s, 't>,
	pub none_constructor: PrototypeI<'s, 't>,
	pub some_impl_name: IdI<'s, 't>,
	pub none_impl_name: IdI<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct BorrowToWeakIE<'s, 't> {
	pub inner_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct LetNormalIE<'s, 't> {
	pub variable: ILocalVariableI<'s, 't>,
	pub expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct RestackifyIE<'s, 't> {
	pub variable: ILocalVariableI<'s, 't>,
	pub expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct UnletIE<'s, 't> {
	pub variable: ILocalVariableI<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct DiscardIE<'s, 't> {
	pub expr: &'t dyn ReferenceExpressionIE<'s, 't>,
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
impl<'s, 't> DiscardIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct DeferIE<'s, 't> {
	pub inner_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub deferred_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct IfIE<'s, 't> {
	pub condition: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub then_call: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub else_call: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct WhileIE<'s, 't> {
	pub block: BlockIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct MutateIE<'s, 't> {
	pub destination_expr: &'t dyn AddressExpressionIE<'s, 't>,
	pub source_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ReturnIE<'s, 't> {
	pub source_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
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
impl<'s, 't> ReturnIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, NeverIT(false))
}
*/
// mig: struct BreakIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct BreakIE<'s, 't> {}
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
impl<'s, 't> BreakIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, NeverIT(true))
}

*/
// mig: struct BlockIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct BlockIE<'s, 't> {
	pub inner: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct MutabilifyIE<'s, 't> {
	pub inner: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ImmutabilifyIE<'s, 't> {
	pub inner: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct PreCheckBorrowIE<'s, 't> {
	pub inner: &'t dyn ReferenceExpressionIE<'s, 't>,
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
impl<'s, 't> PreCheckBorrowIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ConsecutorIE<'s, 't> {
	pub exprs: &'t [&'t dyn ReferenceExpressionIE<'s, 't>],
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct TupleIE<'s, 't> {
	pub elements: &'t [&'t dyn ReferenceExpressionIE<'s, 't>],
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct StaticArrayFromValuesIE<'s, 't> {
	pub elements: &'t [&'t dyn ReferenceExpressionIE<'s, 't>],
	pub result_reference: CoordI<'s, 't>,
	pub array_type: StaticSizedArrayIT<'s, 't>,
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
impl<'s, 't> StaticArrayFromValuesIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = resultReference
}
*/
// mig: struct ArraySizeIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ArraySizeIE<'s, 't> {
	pub array: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct IsSameInstanceIE<'s, 't> {
	pub left: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub right: &'t dyn ReferenceExpressionIE<'s, 't>,
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
impl<'s, 't> IsSameInstanceIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, BoolIT())
}
*/
// mig: struct AsSubtypeIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct AsSubtypeIE<'s, 't> {
	pub source_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub target_type: CoordI<'s, 't>,
	pub result_result_type: CoordI<'s, 't>,
	pub ok_constructor: PrototypeI<'s, 't>,
	pub err_constructor: PrototypeI<'s, 't>,
	pub impl_name: IdI<'s, 't>,
	pub ok_impl_name: IdI<'s, 't>,
	pub err_impl_name: IdI<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct VoidLiteralIE<'s, 't> {}
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
impl<'s, 't> VoidLiteralIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct ConstantIntIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ConstantIntIE<'s, 't> {
	pub value: i64,
	pub bits: i32,
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
impl<'s, 't> ConstantIntIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = CoordI[cI](MutableShareI, IntIT(bits))
}
*/
// mig: struct ConstantBoolIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ConstantBoolIE<'s, 't> {
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
impl<'s, 't> ConstantBoolIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = CoordI[cI](MutableShareI, BoolIT())
}
*/
// mig: struct ConstantStrIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ConstantStrIE<'s, 't> {
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
impl<'s, 't> ConstantStrIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = CoordI[cI](MutableShareI, StrIT())
}
*/
// mig: struct ConstantFloatIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ConstantFloatIE<'s, 't> {
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
impl<'s, 't> ConstantFloatIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result = CoordI[cI](MutableShareI, FloatIT())
}

*/
// mig: struct LocalLookupIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct LocalLookupIE<'s, 't> {
	pub local_variable: ILocalVariableI<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ArgLookupIE<'s, 't> {
	pub param_index: i32,
	pub coord: CoordI<'s, 't>,
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
impl<'s, 't> ArgLookupIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = coord
}
*/
// mig: struct StaticSizedArrayLookupIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct StaticSizedArrayLookupIE<'s, 't> {
	pub range: RangeS<'s>,
	pub array_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub index_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub element_type: CoordI<'s, 't>,
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
impl<'s, 't> StaticSizedArrayLookupIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  // See RMLRMO why we just return the element type.
  override def result: CoordI[cI] = elementType
}
*/
// mig: struct RuntimeSizedArrayLookupIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct RuntimeSizedArrayLookupIE<'s, 't> {
	pub array_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub index_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub element_type: CoordI<'s, 't>,
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
impl<'s, 't> RuntimeSizedArrayLookupIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  // See RMLRMO why we just return the element type.
  override def result: CoordI[cI] = elementType
}
*/
// mig: struct ArrayLengthIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ArrayLengthIE<'s, 't> {
	pub array_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
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
impl<'s, 't> ArrayLengthIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, IntIT(32))
}
*/
// mig: struct ReferenceMemberLookupIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ReferenceMemberLookupIE<'s, 't> {
	pub range: RangeS<'s>,
	pub struct_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub member_name: IVarNameI<'s, 't>,
	pub member_reference: CoordI<'s, 't>,
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
impl<'s, 't> ReferenceMemberLookupIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  // See RMLRMO why we just return the member type.
  override def result: CoordI[cI] = memberReference
}
*/
// mig: struct AddressMemberLookupIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct AddressMemberLookupIE<'s, 't> {
	pub struct_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub member_name: IVarNameI<'s, 't>,
	pub member_reference: CoordI<'s, 't>,
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
impl<'s, 't> AddressMemberLookupIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  // See RMLRMO why we just return the member type.
  override def result: CoordI[cI] = memberReference
}
*/
// mig: struct InterfaceFunctionCallIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct InterfaceFunctionCallIE<'s, 't> {
	pub super_function_prototype: PrototypeI<'s, 't>,
	pub virtual_param_index: i32,
	pub args: &'t [&'t dyn ReferenceExpressionIE<'s, 't>],
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ExternFunctionCallIE<'s, 't> {
	pub prototype2: PrototypeI<'s, 't>,
	pub args: &'t [&'t dyn ReferenceExpressionIE<'s, 't>],
	pub result: CoordI<'s, 't>,
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
    case ExternFunctionNameI(_, _) =>
    case _ => vwat()
  }



//  override def resultRemoveMe = ReferenceResultI(prototype2.returnType)
}
*/
// mig: struct FunctionCallIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct FunctionCallIE<'s, 't> {
	pub callable: PrototypeI<'s, 't>,
	pub args: &'t [&'t dyn ReferenceExpressionIE<'s, 't>],
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ReinterpretIE<'s, 't> {
	pub expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result_reference: CoordI<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ConstructIE<'s, 't> {
	pub struct_tt: StructIT<'s, 't>,
	pub result: CoordI<'s, 't>,
	pub args: &'t [&'t dyn ExpressionIE<'s, 't>],
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct NewMutRuntimeSizedArrayIE<'s, 't> {
	pub array_type: RuntimeSizedArrayIT<'s, 't>,
	pub capacity_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct StaticArrayFromCallableIE<'s, 't> {
	pub array_type: StaticSizedArrayIT<'s, 't>,
	pub generator: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub generator_method: PrototypeI<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct DestroyStaticSizedArrayIntoFunctionIE<'s, 't> {
	pub array_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub array_type: StaticSizedArrayIT<'s, 't>,
	pub consumer: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub consumer_method: PrototypeI<'s, 't>,
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
impl<'s, 't> DestroyStaticSizedArrayIntoFunctionIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct DestroyStaticSizedArrayIntoLocalsIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct DestroyStaticSizedArrayIntoLocalsIE<'s, 't> {
	pub expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub static_sized_array: StaticSizedArrayIT<'s, 't>,
	pub destination_reference_variables: &'t [ReferenceLocalVariableI<'s, 't>],
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
impl<'s, 't> DestroyStaticSizedArrayIntoLocalsIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())

  vassert(expr.result.kind == staticSizedArray)
}
*/
// mig: struct DestroyMutRuntimeSizedArrayIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct DestroyMutRuntimeSizedArrayIE<'s, 't> {
	pub array_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
}
// mig: impl DestroyMutRuntimeSizedArrayIE
/*
case class DestroyMutRuntimeSizedArrayIE(
  arrayExpr: ReferenceExpressionIE
) extends ReferenceExpressionIE {
*/
// mig: fn result
impl<'s, 't> DestroyMutRuntimeSizedArrayIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct RuntimeSizedArrayCapacityIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct RuntimeSizedArrayCapacityIE<'s, 't> {
	pub array_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
}
// mig: impl RuntimeSizedArrayCapacityIE
/*
case class RuntimeSizedArrayCapacityIE(
  arrayExpr: ReferenceExpressionIE
) extends ReferenceExpressionIE {
*/
// mig: fn result
impl<'s, 't> RuntimeSizedArrayCapacityIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, IntIT(32))
//  override def resultRemoveMe: CoordI[cI] = ReferenceResultI(CoordI[cI](MutableShareI, IntIT(32)))
}
*/
// mig: struct PushRuntimeSizedArrayIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct PushRuntimeSizedArrayIE<'s, 't> {
	pub array_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub new_element_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
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
impl<'s, 't> PushRuntimeSizedArrayIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct PopRuntimeSizedArrayIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct PopRuntimeSizedArrayIE<'s, 't> {
	pub array_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct InterfaceToInterfaceUpcastIE<'s, 't> {
	pub inner_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub target_interface: InterfaceIT<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct UpcastIE<'s, 't> {
	pub inner_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub target_interface: InterfaceIT<'s, 't>,
	pub impl_name: IdI<'s, 't>,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct SoftLoadIE<'s, 't> {
	pub expr: &'t dyn AddressExpressionIE<'s, 't>,
	pub target_ownership: OwnershipI,
	pub result: CoordI<'s, 't>,
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
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct DestroyIE<'s, 't> {
	pub expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub struct_tt: StructIT<'s, 't>,
	pub destination_reference_variables: &'t [ReferenceLocalVariableI<'s, 't>],
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
impl<'s, 't> DestroyIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct DestroyImmRuntimeSizedArrayIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct DestroyImmRuntimeSizedArrayIE<'s, 't> {
	pub array_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub array_type: RuntimeSizedArrayIT<'s, 't>,
	pub consumer: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub consumer_method: PrototypeI<'s, 't>,
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
impl<'s, 't> DestroyImmRuntimeSizedArrayIE<'s, 't> {
	pub fn result(&self) -> CoordI<'s, 't> { panic!("Unimplemented: result"); }
}
/*
  override def result: CoordI[cI] = CoordI[cI](MutableShareI, VoidIT())
}
*/
// mig: struct NewImmRuntimeSizedArrayIE
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct NewImmRuntimeSizedArrayIE<'s, 't> {
	pub array_type: RuntimeSizedArrayIT<'s, 't>,
	pub size_expr: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub generator: &'t dyn ReferenceExpressionIE<'s, 't>,
	pub generator_method: PrototypeI<'s, 't>,
	pub result: CoordI<'s, 't>,
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