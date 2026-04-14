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
// mig: trait IExpressionResultT
pub trait IExpressionResultT {}
/*
trait IExpressionResultT  {
*/
// mig: fn expect_reference
fn expect_reference(&self) -> &ReferenceResultT { panic!("Unimplemented: expect_reference"); }
/*
  def expectReference(): ReferenceResultT = {
    this match {
      case r @ ReferenceResultT(_) => r
      case AddressResultT(_) => vfail("Expected a reference as a result, but got an address!")
    }
  }
*/
// mig: fn expect_address
fn expect_address(&self) -> &AddressResultT { panic!("Unimplemented: expect_address"); }
/*
  def expectAddress(): AddressResultT = {
    this match {
      case a @ AddressResultT(_) => a
      case ReferenceResultT(_) => vfail("Expected an address as a result, but got a reference!")
    }
  }
*/
// mig: fn underlying_coord
fn underlying_coord(&self) -> CoordT { panic!("Unimplemented: underlying_coord"); }
/*
  def underlyingCoord: CoordT
*/
// mig: fn kind
fn kind(&self) -> KindT { panic!("Unimplemented: kind"); }
/*
  def kind: KindT
}
*/
// mig: struct AddressResultT
pub struct AddressResultT { pub coord: CoordT }
// mig: impl AddressResultT
impl AddressResultT {}
/*
case class AddressResultT(coord: CoordT) extends IExpressionResultT {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn underlying_coord
fn underlying_coord(&self) -> CoordT { panic!("Unimplemented: underlying_coord"); }
/*
  override def underlyingCoord: CoordT = coord
*/
// mig: fn kind
fn kind(&self) -> KindT { panic!("Unimplemented: kind"); }
/*
  override def kind = coord.kind
}
*/
// mig: struct ReferenceResultT
pub struct ReferenceResultT { pub coord: CoordT }
// mig: impl ReferenceResultT
impl ReferenceResultT {}
/*
case class ReferenceResultT(coord: CoordT) extends IExpressionResultT {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn underlying_coord
fn underlying_coord(&self) -> CoordT { panic!("Unimplemented: underlying_coord"); }
/*
  override def underlyingCoord: CoordT = coord
*/
// mig: fn kind
fn kind(&self) -> KindT { panic!("Unimplemented: kind"); }
/*
  override def kind = coord.kind
}
*/
// mig: trait ExpressionT
pub trait ExpressionT {}
/*
trait ExpressionT  {
*/
// mig: fn result
fn result(&self) -> IExpressionResultT { panic!("Unimplemented: result"); }
/*
  def result: IExpressionResultT
*/
// mig: fn kind
fn kind(&self) -> KindT { panic!("Unimplemented: kind"); }
/*
  def kind: KindT
}
*/
// mig: trait ReferenceExpressionTE
pub trait ReferenceExpressionTE {}
/*
trait ReferenceExpressionTE extends ExpressionT {
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT
*/
// mig: fn kind
fn kind(&self) -> KindT { panic!("Unimplemented: kind"); }
/*
  override def kind = result.coord.kind
}
*/
// mig: trait AddressExpressionTE
pub trait AddressExpressionTE {}
/*
// This is an Expression2 because we sometimes take an address and throw it
// directly into a struct (closures!), which can have addressible members.
trait AddressExpressionTE extends ExpressionT {
*/
// mig: fn result
fn result(&self) -> AddressResultT { panic!("Unimplemented: result"); }
/*
  override def result: AddressResultT
*/
// mig: fn kind
fn kind(&self) -> KindT { panic!("Unimplemented: kind"); }
/*
  override def kind = result.coord.kind
*/
// mig: fn range
fn range(&self) -> RangeS { panic!("Unimplemented: range"); }
/*
  def range: RangeS
*/
// mig: fn variability
fn variability(&self) -> VariabilityT { panic!("Unimplemented: variability"); }
/*
  // Whether or not we can change where this address points to
  def variability: VariabilityT
}

*/
// mig: struct LetAndLendTE
pub struct LetAndLendTE { pub variable: ILocalVariableT, pub expr: ReferenceExpressionTE, pub target_ownership: OwnershipT }
// mig: impl LetAndLendTE
impl LetAndLendTE {}
/*
case class LetAndLendTE(
    variable: ILocalVariableT,
    expr: ReferenceExpressionTE,
  targetOwnership: OwnershipT
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
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
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = {
    val CoordT(oldOwnership, region, kind) = expr.result.coord
    ReferenceResultT(CoordT(targetOwnership, region, kind))
  }
}

*/
// mig: struct LockWeakTE
pub struct LockWeakTE { pub inner_expr: ReferenceExpressionTE, pub result_opt_borrow_type: CoordT, pub some_constructor: PrototypeT, pub none_constructor: PrototypeT, pub some_impl_name: IdT, pub none_impl_name: IdT }
// mig: impl LockWeakTE
impl LockWeakTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(resultOptBorrowType)
  }
}

*/
// mig: struct BorrowToWeakTE
pub struct BorrowToWeakTE { pub inner_expr: ReferenceExpressionTE }
// mig: impl BorrowToWeakTE
impl BorrowToWeakTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
  innerExpr.result.coord.ownership match {
    case BorrowT =>
  }

*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(WeakT, innerExpr.result.coord.region, innerExpr.kind))
  }
}

*/
// mig: struct LetNormalTE
pub struct LetNormalTE { pub variable: ILocalVariableT, pub expr: ReferenceExpressionTE }
// mig: impl LetNormalTE
impl LetNormalTE {}
/*
case class LetNormalTE(
    variable: ILocalVariableT,
    expr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct UnletTE { pub variable: ILocalVariableT }
// mig: impl UnletTE
impl UnletTE {}
/*
// Only ExpressionCompiler.unletLocal should make these
case class UnletTE(variable: ILocalVariableT) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(variable.coord)

  vpass()
}

*/
// mig: struct DiscardTE
pub struct DiscardTE { pub expr: ReferenceExpressionTE }
// mig: impl DiscardTE
impl DiscardTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct DeferTE { pub inner_expr: ReferenceExpressionTE, pub deferred_expr: ReferenceExpressionTE }
// mig: impl DeferTE
impl DeferTE {}
/*
case class DeferTE(
  innerExpr: ReferenceExpressionTE,
  // Every deferred expression should discard its result, IOW, return Void.
  deferredExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(innerExpr.result.coord)

  vassert(deferredExpr.result.coord == CoordT(ShareT, innerExpr.result.coord.region, VoidT()))
}


*/
// mig: struct IfTE
pub struct IfTE { pub condition: ReferenceExpressionTE, pub then_call: ReferenceExpressionTE, pub else_call: ReferenceExpressionTE }
// mig: impl IfTE
impl IfTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct WhileTE { pub block: BlockTE }
// mig: impl WhileTE
impl WhileTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(resultCoord)
  vpass()
}

*/
// mig: struct MutateTE
pub struct MutateTE { pub destination_expr: AddressExpressionTE, pub source_expr: ReferenceExpressionTE }
// mig: impl MutateTE
impl MutateTE {}
/*
case class MutateTE(
  destinationExpr: AddressExpressionTE,
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(destinationExpr.result.coord)
}

*/
// mig: struct RestackifyTE
pub struct RestackifyTE { pub variable: ILocalVariableT, pub source_expr: ReferenceExpressionTE }
// mig: impl RestackifyTE
impl RestackifyTE {}
/*
case class RestackifyTE(
  variable: ILocalVariableT,
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(CoordT(ShareT, sourceExpr.result.coord.region, VoidT()))
}

*/
// mig: struct TransmigrateTE
pub struct TransmigrateTE { pub source_expr: ReferenceExpressionTE, pub target_region: RegionT }
// mig: impl TransmigrateTE
impl TransmigrateTE {}
/*
case class TransmigrateTE(
  sourceExpr: ReferenceExpressionTE,
  targetRegion: RegionT
) extends ReferenceExpressionTE {
  vassert(sourceExpr.kind.isPrimitive)
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(sourceExpr.result.coord.copy(region = targetRegion))
}


*/
// mig: struct ReturnTE
pub struct ReturnTE { pub source_expr: ReferenceExpressionTE }
// mig: impl ReturnTE
impl ReturnTE {}
/*
case class ReturnTE(
  sourceExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(ShareT, sourceExpr.result.coord.region, NeverT(false)))
  }
}

*/
// mig: struct BreakTE
pub struct BreakTE { pub region: RegionT }
// mig: impl BreakTE
impl BreakTE {}
/*
case class BreakTE(region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = {
    ReferenceResultT(CoordT(ShareT, region, NeverT(true)))
  }
}

*/
// mig: struct BlockTE
pub struct BlockTE { pub inner: ReferenceExpressionTE }
// mig: impl BlockTE
impl BlockTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = inner.result
}

*/
// mig: struct PureTE
pub struct PureTE { pub newdefault_region: RegionT, pub inner: ReferenceExpressionTE, pub result_type: CoordT }
// mig: impl PureTE
impl PureTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(resultType)
}

*/
// mig: struct ConsecutorTE
pub struct ConsecutorTE { pub exprs: Vec<ReferenceExpressionTE> }
// mig: impl ConsecutorTE
impl ConsecutorTE {}
/*
case class ConsecutorTE(exprs: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
fn last_reference_expr(&self) -> &ReferenceExpressionTE { panic!("Unimplemented: last_reference_expr"); }
/*
  def lastReferenceExpr = exprs.last
}

*/
// mig: struct TupleTE
pub struct TupleTE { pub elements: Vec<ReferenceExpressionTE>, pub result_reference: CoordT }
// mig: impl TupleTE
impl TupleTE {}
/*
case class TupleTE(
    elements: Vector[ReferenceExpressionTE],
    resultReference: CoordT) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct StaticArrayFromValuesTE { pub elements: Vec<ReferenceExpressionTE>, pub result_reference: CoordT, pub array_type: StaticSizedArrayTT }
// mig: impl StaticArrayFromValuesTE
impl StaticArrayFromValuesTE {}
/*
case class StaticArrayFromValuesTE(
  elements: Vector[ReferenceExpressionTE],
  resultReference: CoordT,
  arrayType: StaticSizedArrayTT,
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(resultReference)
}

*/
// mig: struct ArraySizeTE
pub struct ArraySizeTE { pub array: ReferenceExpressionTE }
// mig: impl ArraySizeTE
impl ArraySizeTE {}
/*
case class ArraySizeTE(array: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(CoordT(ShareT, array.result.coord.region, IntT.i32))
}

*/
// mig: struct IsSameInstanceTE
pub struct IsSameInstanceTE { pub left: ReferenceExpressionTE, pub right: ReferenceExpressionTE }
// mig: impl IsSameInstanceTE
impl IsSameInstanceTE {}
/*
// Can we do an === of objects in two regions? It could be pretty useful.
case class IsSameInstanceTE(left: ReferenceExpressionTE, right: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  vassert(left.result.coord == right.result.coord)

  override def result = ReferenceResultT(CoordT(ShareT, left.result.coord.region, BoolT()))
}

*/
// mig: struct AsSubtypeTE
pub struct AsSubtypeTE { pub source_expr: ReferenceExpressionTE, pub target_type: CoordT, pub result_result_type: CoordT, pub ok_constructor: PrototypeT, pub err_constructor: PrototypeT, pub impl_name: IdT, pub ok_impl_name: IdT, pub err_impl_name: IdT }
// mig: impl AsSubtypeTE
impl AsSubtypeTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(resultResultType)
}

*/
// mig: struct VoidLiteralTE
pub struct VoidLiteralTE { pub region: RegionT }
// mig: impl VoidLiteralTE
impl VoidLiteralTE {}
/*
case class VoidLiteralTE(region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(CoordT(ShareT, region, VoidT()))
}

*/
// mig: struct ConstantIntTE
pub struct ConstantIntTE { pub value: ITemplataT, pub bits: i32, pub region: RegionT }
// mig: impl ConstantIntTE
impl ConstantIntTE {}
/*
case class ConstantIntTE(value: ITemplataT[IntegerTemplataType], bits: Int, region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = {
    ReferenceResultT(CoordT(ShareT, region, IntT(bits)))
  }
}

*/
// mig: struct ConstantBoolTE
pub struct ConstantBoolTE { pub value: bool, pub region: RegionT }
// mig: impl ConstantBoolTE
impl ConstantBoolTE {}
/*
case class ConstantBoolTE(value: Boolean, region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, region, BoolT()))
}

*/
// mig: struct ConstantStrTE
pub struct ConstantStrTE { pub value: String, pub region: RegionT }
// mig: impl ConstantStrTE
impl ConstantStrTE {}
/*
case class ConstantStrTE(value: String, region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, region, StrT()))
}

*/
// mig: struct ConstantFloatTE
pub struct ConstantFloatTE { pub value: f64, pub region: RegionT }
// mig: impl ConstantFloatTE
impl ConstantFloatTE {}
/*
case class ConstantFloatTE(value: Double, region: RegionT) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(CoordT(ShareT, region, FloatT()))
}

*/
// mig: struct LocalLookupTE
pub struct LocalLookupTE { pub range: RangeS, pub local_variable: ILocalVariableT }
// mig: impl LocalLookupTE
impl LocalLookupTE {}
/*
case class LocalLookupTE(
  range: RangeS,
  // This is the local variable at the time it was created
  localVariable: ILocalVariableT
) extends AddressExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> AddressResultT { panic!("Unimplemented: result"); }
/*
  override def result: AddressResultT = AddressResultT(localVariable.coord)
*/
// mig: fn variability
fn variability(&self) -> VariabilityT { panic!("Unimplemented: variability"); }
/*
  override def variability: VariabilityT = localVariable.variability
}

*/
// mig: struct ArgLookupTE
pub struct ArgLookupTE { pub param_index: i32, pub coord: CoordT }
// mig: impl ArgLookupTE
impl ArgLookupTE {}
/*
case class ArgLookupTE(
    paramIndex: Int,
    coord: CoordT
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result = ReferenceResultT(coord)
}

*/
// mig: struct StaticSizedArrayLookupTE
pub struct StaticSizedArrayLookupTE { pub range: RangeS, pub array_expr: ReferenceExpressionTE, pub array_type: StaticSizedArrayTT, pub index_expr: ReferenceExpressionTE, pub element_type: CoordT, pub variability: VariabilityT }
// mig: impl StaticSizedArrayLookupTE
impl StaticSizedArrayLookupTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> AddressResultT { panic!("Unimplemented: result"); }
/*
  override def result = {
    // See RMLRMO why we just return the element type.
    AddressResultT(elementType)
  }
}

*/
// mig: struct RuntimeSizedArrayLookupTE
pub struct RuntimeSizedArrayLookupTE { pub range: RangeS, pub array_expr: ReferenceExpressionTE, pub array_type: RuntimeSizedArrayTT, pub index_expr: ReferenceExpressionTE, pub variability: VariabilityT }
// mig: impl RuntimeSizedArrayLookupTE
impl RuntimeSizedArrayLookupTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> AddressResultT { panic!("Unimplemented: result"); }
/*
  vassert(arrayExpr.result.coord.kind == arrayType)

  override def result = {
    // See RMLRMO why we just return the element type.
    AddressResultT(arrayType.elementType)
  }
}

*/
// mig: struct ArrayLengthTE
pub struct ArrayLengthTE { pub array_expr: ReferenceExpressionTE }
// mig: impl ArrayLengthTE
impl ArrayLengthTE {}
/*
case class ArrayLengthTE(arrayExpr: ReferenceExpressionTE) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, IntT.i32))
}

*/
// mig: struct ReferenceMemberLookupTE
pub struct ReferenceMemberLookupTE { pub range: RangeS, pub struct_expr: ReferenceExpressionTE, pub member_name: IVarNameT, pub member_reference: CoordT, pub variability: VariabilityT }
// mig: impl ReferenceMemberLookupTE
impl ReferenceMemberLookupTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> AddressResultT { panic!("Unimplemented: result"); }
/*
  override def result = {
    // See RMLRMO why we just return the member type.
    AddressResultT(memberReference)
  }
}
*/
// mig: struct AddressMemberLookupTE
pub struct AddressMemberLookupTE { pub range: RangeS, pub struct_expr: ReferenceExpressionTE, pub member_name: IVarNameT, pub result_type2: CoordT, pub variability: VariabilityT }
// mig: impl AddressMemberLookupTE
impl AddressMemberLookupTE {}
/*
case class AddressMemberLookupTE(
    range: RangeS,
    structExpr: ReferenceExpressionTE,
    memberName: IVarNameT,
    resultType2: CoordT,
    variability: VariabilityT) extends AddressExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> AddressResultT { panic!("Unimplemented: result"); }
/*
  override def result = AddressResultT(resultType2)
}

*/
// mig: struct InterfaceFunctionCallTE
pub struct InterfaceFunctionCallTE { pub super_function_prototype: PrototypeT, pub virtual_param_index: i32, pub result_reference: CoordT, pub args: Vec<ReferenceExpressionTE> }
// mig: impl InterfaceFunctionCallTE
impl InterfaceFunctionCallTE {}
/*
case class InterfaceFunctionCallTE(
    superFunctionPrototype: PrototypeT[IFunctionNameT],
    virtualParamIndex: Int,
    resultReference: CoordT,
    args: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(resultReference)
}

*/
// mig: struct ExternFunctionCallTE
pub struct ExternFunctionCallTE { pub prototype2: PrototypeT, pub args: Vec<ReferenceExpressionTE> }
// mig: impl ExternFunctionCallTE
impl ExternFunctionCallTE {}
/*
case class ExternFunctionCallTE(
    prototype2: PrototypeT[ExternFunctionNameT],
    args: Vector[ReferenceExpressionTE]) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct FunctionCallTE { pub callable: PrototypeT, pub args: Vec<ReferenceExpressionTE>, pub return_type: CoordT }
// mig: impl FunctionCallTE
impl FunctionCallTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct ReinterpretTE { pub expr: ReferenceExpressionTE, pub result_reference: CoordT }
// mig: impl ReinterpretTE
impl ReinterpretTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct ConstructTE { pub struct_tt: StructTT, pub result_reference: CoordT, pub args: Vec<ExpressionT> }
// mig: impl ConstructTE
impl ConstructTE {}
/*
case class ConstructTE(
    structTT: StructTT,
    resultReference: CoordT,
    args: Vector[ExpressionT],
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  vpass()

  override def result = ReferenceResultT(resultReference)
}

*/
// mig: struct NewMutRuntimeSizedArrayTE
pub struct NewMutRuntimeSizedArrayTE { pub array_type: RuntimeSizedArrayTT, pub region: RegionT, pub capacity_expr: ReferenceExpressionTE }
// mig: impl NewMutRuntimeSizedArrayTE
impl NewMutRuntimeSizedArrayTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct StaticArrayFromCallableTE { pub array_type: StaticSizedArrayTT, pub region: RegionT, pub generator: ReferenceExpressionTE, pub generator_method: PrototypeT }
// mig: impl StaticArrayFromCallableTE
impl StaticArrayFromCallableTE {}
/*
case class StaticArrayFromCallableTE(
  arrayType: StaticSizedArrayTT,
  region: RegionT,
  generator: ReferenceExpressionTE,
  generatorMethod: PrototypeT[IFunctionNameT],
) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct DestroyStaticSizedArrayIntoFunctionTE { pub array_expr: ReferenceExpressionTE, pub array_type: StaticSizedArrayTT, pub consumer: ReferenceExpressionTE, pub consumer_method: PrototypeT }
// mig: impl DestroyStaticSizedArrayIntoFunctionTE
impl DestroyStaticSizedArrayIntoFunctionTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct DestroyStaticSizedArrayIntoLocalsTE { pub expr: ReferenceExpressionTE, pub static_sized_array: StaticSizedArrayTT, pub destination_reference_variables: Vec<ReferenceLocalVariableT> }
// mig: impl DestroyStaticSizedArrayIntoLocalsTE
impl DestroyStaticSizedArrayIntoLocalsTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, expr.result.coord.region, VoidT()))

  vassert(expr.kind == staticSizedArray)
  if (expr.result.coord.ownership == BorrowT) {
    vfail("wot")
  }
}

*/
// mig: struct DestroyMutRuntimeSizedArrayTE
pub struct DestroyMutRuntimeSizedArrayTE { pub array_expr: ReferenceExpressionTE }
// mig: impl DestroyMutRuntimeSizedArrayTE
impl DestroyMutRuntimeSizedArrayTE {}
/*
case class DestroyMutRuntimeSizedArrayTE(
  arrayExpr: ReferenceExpressionTE,
) extends ReferenceExpressionTE {
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = {
    ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
  }
}

*/
// mig: struct RuntimeSizedArrayCapacityTE
pub struct RuntimeSizedArrayCapacityTE { pub array_expr: ReferenceExpressionTE }
// mig: impl RuntimeSizedArrayCapacityTE
impl RuntimeSizedArrayCapacityTE {}
/*
case class RuntimeSizedArrayCapacityTE(
  arrayExpr: ReferenceExpressionTE
) extends ReferenceExpressionTE {
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, IntT(32)))
}

*/
// mig: struct PushRuntimeSizedArrayTE
pub struct PushRuntimeSizedArrayTE { pub array_expr: ReferenceExpressionTE, pub new_element_expr: ReferenceExpressionTE }
// mig: impl PushRuntimeSizedArrayTE
impl PushRuntimeSizedArrayTE {}
/*
case class PushRuntimeSizedArrayTE(
  arrayExpr: ReferenceExpressionTE,
//  arrayType: RuntimeSizedArrayTT,
  newElementExpr: ReferenceExpressionTE,
//  newElementType: CoordT,
) extends ReferenceExpressionTE {
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(CoordT(ShareT, arrayExpr.result.coord.region, VoidT()))
}

*/
// mig: struct PopRuntimeSizedArrayTE
pub struct PopRuntimeSizedArrayTE { pub array_expr: ReferenceExpressionTE }
// mig: impl PopRuntimeSizedArrayTE
impl PopRuntimeSizedArrayTE {}
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
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
/*
  override def result: ReferenceResultT = ReferenceResultT(elementType)
}

*/
// mig: struct InterfaceToInterfaceUpcastTE
pub struct InterfaceToInterfaceUpcastTE { pub inner_expr: ReferenceExpressionTE, pub target_interface: InterfaceTT }
// mig: impl InterfaceToInterfaceUpcastTE
impl InterfaceToInterfaceUpcastTE {}
/*
case class InterfaceToInterfaceUpcastTE(
    innerExpr: ReferenceExpressionTE,
    targetInterface: InterfaceTT) extends ReferenceExpressionTE {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct UpcastTE { pub inner_expr: ReferenceExpressionTE, pub target_super_kind: ISuperKindTT, pub impl_name: IdT }
// mig: impl UpcastTE
impl UpcastTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct SoftLoadTE { pub expr: AddressExpressionTE, pub target_ownership: OwnershipT }
// mig: impl SoftLoadTE
impl SoftLoadTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct DestroyTE { pub expr: ReferenceExpressionTE, pub struct_tt: StructTT, pub destination_reference_variables: Vec<ReferenceLocalVariableT> }
// mig: impl DestroyTE
impl DestroyTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct DestroyImmRuntimeSizedArrayTE { pub array_expr: ReferenceExpressionTE, pub array_type: RuntimeSizedArrayTT, pub consumer: ReferenceExpressionTE, pub consumer_method: PrototypeT }
// mig: impl DestroyImmRuntimeSizedArrayTE
impl DestroyImmRuntimeSizedArrayTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
pub struct NewImmRuntimeSizedArrayTE { pub array_type: RuntimeSizedArrayTT, pub region: RegionT, pub size_expr: ReferenceExpressionTE, pub generator: ReferenceExpressionTE, pub generator_method: PrototypeT }
// mig: impl NewImmRuntimeSizedArrayTE
impl NewImmRuntimeSizedArrayTE {}
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
fn equals(&self, obj: &dyn std::any::Any) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn result
fn result(&self) -> ReferenceResultT { panic!("Unimplemented: result"); }
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
fn unapply(&self, expr: &ReferenceExpressionTE) -> Option<StrI> { panic!("Unimplemented: unapply"); }
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
fn unapply(&self, expr: &ReferenceExpressionTE) -> Option<KindT> { panic!("Unimplemented: unapply"); }
/*
  def unapply(expr: ReferenceExpressionTE): Option[KindT] = {
    Some(expr.result.coord.kind)
  }
}
*/