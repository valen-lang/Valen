use crate::postparsing::ast::LocationInDenizen;
use crate::typing::compiler::Compiler;
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::postparsing::expressions::*;
use crate::postparsing::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::compiler_outputs::*;
use crate::parsing::ast::*;
use crate::interner::Interner;

/*
package dev.vale.typing.expression

import dev.vale.{Interner, RangeS, vassert, vfail, vimpl}
import dev.vale.parsing.ast.{LoadAsBorrowP, LoadAsP, LoadAsWeakP, MoveP, UseP}
import dev.vale.postparsing._
import dev.vale.typing.{CantMoveOutOfMemberT, CompileErrorExceptionT, Compiler, CompilerOutputs, RangedInternalErrorT, TypingPassOptions, ast, env}
import dev.vale.typing.ast.{AddressExpressionTE, AddressMemberLookupTE, DeferTE, ExpressionT, LetAndLendTE, LocalLookupTE, LocationInFunctionEnvironmentT, ReferenceExpressionTE, ReferenceMemberLookupTE, RuntimeSizedArrayLookupTE, SoftLoadTE, StaticSizedArrayLookupTE, UnletTE}
import dev.vale.typing.env.{AddressibleLocalVariableT, ILocalVariableT, NodeEnvironmentBox, ReferenceLocalVariableT}
import dev.vale.typing.function.DestructorCompiler
import dev.vale.typing.names.{NameTranslator, TypingPassTemporaryVarNameT}
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing.LocalS
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.types._
import dev.vale.typing.{ast, _}
import dev.vale.typing.ast._
import dev.vale.typing.names.TypingPassTemporaryVarNameT

import scala.collection.immutable.List
*/
/*
class LocalHelper(
    opts: TypingPassOptions,
    interner: Interner,
    nameTranslator: NameTranslator,
    destructorCompiler: DestructorCompiler) {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_temporary_local(&self, nenv: &mut NodeEnvironmentBox<'s, 't>, life: LocationInFunctionEnvironmentT<'s>, coord: CoordT<'s, 't>) -> ReferenceLocalVariableT<'s, 't> {
        panic!("Unimplemented: make_temporary_local");
    }
/*
  def makeTemporaryLocal(
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    coord: CoordT):
  ReferenceLocalVariableT = {
    val varId = interner.intern(TypingPassTemporaryVarNameT(life))
    val rlv = ReferenceLocalVariableT(varId, FinalT, coord)
    nenv.addVariable(rlv)
    rlv
  }

*/
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_temporary_local_defer(&self, coutputs: &CompilerOutputs<'s, 't>, nenv: &mut NodeEnvironmentBox<'s, 't>, range: &[RangeS<'s>], call_location: LocationInDenizen<'s>, life: LocationInFunctionEnvironmentT<'s>, context_region: RegionT, r: ReferenceExpressionTE<'s, 't>, target_ownership: OwnershipT) -> DeferTE<'s, 't> {
        panic!("Unimplemented: make_temporary_local");
    }
/*
  // This makes a borrow ref, but can easily turn that into a weak
  // separately.
  def makeTemporaryLocal(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    life: LocationInFunctionEnvironmentT,
    contextRegion: RegionT,
    r: ReferenceExpressionTE,
    targetOwnership: OwnershipT):
  (DeferTE) = {
    targetOwnership match {
      case BorrowT =>
    }

    val rlv = makeTemporaryLocal(nenv, life, r.result.coord)
    val letExpr2 = LetAndLendTE(rlv, r, targetOwnership)

    val unlet = unletLocalWithoutDropping(nenv, rlv)
    val destructExpr2 =
      destructorCompiler.drop(nenv.snapshot, coutputs, range, callLocation, contextRegion, unlet)
    vassert(destructExpr2.kind == VoidT())

    // No Discard here because the destructor already returns void.

    (DeferTE(letExpr2, destructExpr2))
  }

*/
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn unlet_local_without_dropping(&self, nenv: &mut NodeEnvironmentBox<'s, 't>, local_var: &ILocalVariableT<'s, 't>) -> UnletTE<'s, 't> {
        nenv.mark_local_unstackified(local_var.name());
        UnletTE { variable: *local_var }
    }
/*
  def unletLocalWithoutDropping(nenv: NodeEnvironmentBox, localVar: ILocalVariableT):
  (UnletTE) = {
    nenv.markLocalUnstackified(localVar.name)
    UnletTE(localVar)
  }

*/
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn unlet_and_drop_all(&self, coutputs: &mut CompilerOutputs<'s, 't>, nenv: &mut NodeEnvironmentBox<'s, 't>, range: &[RangeS<'s>], call_location: LocationInDenizen<'s>, context_region: RegionT, variables: &[&ILocalVariableT<'s, 't>]) -> Vec<&'t ReferenceExpressionTE<'s, 't>> {
        variables.iter().map(|variable| {
            let unlet = self.unlet_local_without_dropping(nenv, variable);
            let unlet_ref = &*self.typing_interner.alloc(ReferenceExpressionTE::Unlet(unlet));
            let snapshot = nenv.snapshot(self.typing_interner);
            let snapshot_env = &*self.typing_interner.alloc(IInDenizenEnvironmentT::Node(snapshot));
            self.drop(snapshot_env, coutputs, range, call_location, context_region, unlet_ref)
        }).collect()
    }
/*
  def unletAndDropAll(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    variables: Vector[ILocalVariableT]):
  (Vector[ReferenceExpressionTE]) = {
    variables.map({ case variable =>
      val unlet = unletLocalWithoutDropping(nenv, variable)
      val maybeHeadExpr2 =
        destructorCompiler.drop(nenv.snapshot, coutputs, range, callLocation, contextRegion, unlet)
      maybeHeadExpr2
    })
  }

*/
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn unlet_all_without_dropping(&self, coutputs: &CompilerOutputs<'s, 't>, nenv: &mut NodeEnvironmentBox<'s, 't>, range: &[RangeS<'s>], variables: &[&ILocalVariableT<'s, 't>]) -> Vec<ReferenceExpressionTE<'s, 't>> {
        panic!("Unimplemented: unlet_all_without_dropping");
    }
/*
  def unletAllWithoutDropping(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    range: List[RangeS],
    variables: Vector[ILocalVariableT]):
  (Vector[ReferenceExpressionTE]) = {
    variables.map(variable => unletLocalWithoutDropping(nenv, variable))
  }

*/
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_user_local_variable(&self, coutputs: &CompilerOutputs<'s, 't>, nenv: &mut NodeEnvironmentBox<'s, 't>, range: &[RangeS<'s>], local_variable_a: &'s LocalS<'s>, reference_type2: CoordT<'s, 't>) -> ILocalVariableT<'s, 't> {
        let var_id = self.translate_var_name_step(local_variable_a.var_name);

        if nenv.get_variable(var_id, self.typing_interner).is_some() {
            panic!("There's already a variable named {:?}", var_id);
        }

        let variability = self.determine_local_variability(local_variable_a);

        let mutable = self.get_mutability(coutputs, reference_type2.kind);
        let addressible = self.determine_if_local_is_addressible(mutable, local_variable_a);

        let local_var = if addressible {
            panic!("implement: make_user_local_variable — addressible local");
        } else {
            ILocalVariableT::Reference(ReferenceLocalVariableT {
                name: var_id,
                variability,
                coord: reference_type2,
            })
        };
        nenv.add_variable(IVariableT::from(local_var));
        local_var
    }
/*
  // A user local variable is one that the user can address inside their code.
  // Users never see the names of non-user local variables, so they can't be
  // looked up.
  // Non-user local variables are reference local variables, so can't be
  // mutated from inside closures.
  def makeUserLocalVariable(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    range: List[RangeS],
    localVariableA: LocalS,
    referenceType2: CoordT):
  ILocalVariableT = {
    val varId = nameTranslator.translateVarNameStep(localVariableA.varName)

    if (nenv.getVariable(varId).nonEmpty) {
      throw CompileErrorExceptionT(RangedInternalErrorT(range, "There's already a variable named " + varId))
    }

    val variability = LocalHelper.determineLocalVariability(localVariableA)

    val mutable = Compiler.getMutability(coutputs, referenceType2.kind)
    val addressible = LocalHelper.determineIfLocalIsAddressible(mutable, localVariableA)

    val localVar =
      if (addressible) {
        AddressibleLocalVariableT(varId, variability, referenceType2)
      } else {
        env.ReferenceLocalVariableT(varId, variability, referenceType2)
      }
    nenv.addVariable(localVar)
    localVar
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn maybe_borrow_soft_load(&self, coutputs: &CompilerOutputs<'s, 't>, expr2: &ExpressionTE<'s, 't>) -> ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: maybe_borrow_soft_load");
    }
/*
  def maybeBorrowSoftLoad(
      coutputs: CompilerOutputs,
      expr2: ExpressionT):
  ReferenceExpressionTE = {
    expr2 match {
      case e : ReferenceExpressionTE => e
      case e : AddressExpressionTE => borrowSoftLoad(coutputs, e)
    }
  }

*/
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn soft_load(&self, nenv: &mut NodeEnvironmentBox<'s, 't>, load_range: &[RangeS<'s>], a: &'t AddressExpressionTE<'s, 't>, load_as_p: LoadAsP, region: RegionT) -> ReferenceExpressionTE<'s, 't> {
        match a.result().coord.ownership {
            OwnershipT::Share => {
                ReferenceExpressionTE::SoftLoad(SoftLoadTE { expr: a, target_ownership: OwnershipT::Share })
            }
            OwnershipT::Own => {
                panic!("implement: soft_load — OwnT");
            }
            OwnershipT::Borrow => {
                panic!("implement: soft_load — BorrowT");
            }
            OwnershipT::Weak => {
                panic!("implement: soft_load — WeakT");
            }
        }
    }
/*
  def softLoad(
      nenv: NodeEnvironmentBox,
      loadRange: List[RangeS],
      a: AddressExpressionTE,
    loadAsP: LoadAsP,
    region: RegionT):
  ReferenceExpressionTE = {
    a.result.coord.ownership match {
      case ShareT => {
        SoftLoadTE(a, ShareT)
      }
      case OwnT => {
        loadAsP match {
          case UseP => {
            a match {
              case LocalLookupTE(_, lv) => {
                nenv.markLocalUnstackified(lv.name)
                UnletTE(lv)
              }
              // See CSHROOR for why these aren't just Readwrite.
              case l @ RuntimeSizedArrayLookupTE(_, _, _, _, _) => SoftLoadTE(l, BorrowT)
              case l @ StaticSizedArrayLookupTE(_, _, _, _, _, _) => SoftLoadTE(l, BorrowT)
              case l @ ReferenceMemberLookupTE(_,_, _, _, _) => SoftLoadTE(l, BorrowT)
              case l @ AddressMemberLookupTE(_, _, _, _, _) => SoftLoadTE(l, BorrowT)
            }
          }
          case MoveP => {
            a match {
              case LocalLookupTE(_, lv) => {
                nenv.markLocalUnstackified(lv.name)
                UnletTE(lv)
              }
              case ReferenceMemberLookupTE(_,_, name, _, _) => {
                throw CompileErrorExceptionT(CantMoveOutOfMemberT(loadRange, name))
              }
              case AddressMemberLookupTE(_, _, name, _, _) => {
                throw CompileErrorExceptionT(CantMoveOutOfMemberT(loadRange, name))
              }
            }
          }
          case LoadAsBorrowP => SoftLoadTE(a, BorrowT)
          case LoadAsWeakP => SoftLoadTE(a, WeakT)
        }
      }
      case BorrowT => {
        loadAsP match {
          case MoveP => vfail()
          case UseP => SoftLoadTE(a, a.result.coord.ownership)
          case LoadAsBorrowP => SoftLoadTE(a, BorrowT)
          case LoadAsWeakP => SoftLoadTE(a, WeakT)
        }
      }
      case WeakT => {
        loadAsP match {
          case UseP => SoftLoadTE(a, WeakT)
          case MoveP => vfail()
          case LoadAsBorrowP => SoftLoadTE(a, WeakT)
          case LoadAsWeakP => SoftLoadTE(a, WeakT)
        }
      }
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn borrow_soft_load(&self, coutputs: &CompilerOutputs<'s, 't>, expr2: &AddressExpressionTE<'s, 't>) -> ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: borrow_soft_load");
    }
/*
  def borrowSoftLoad(coutputs: CompilerOutputs, expr2: AddressExpressionTE):
  ReferenceExpressionTE = {
    val ownership = getBorrowOwnership(coutputs, expr2.result.coord.kind)
    ast.SoftLoadTE(expr2, ownership)
  }

*/
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_borrow_ownership(&self, coutputs: &CompilerOutputs<'s, 't>, kind: &KindT<'s, 't>) -> OwnershipT {
        panic!("Unimplemented: get_borrow_ownership");
    }
/*
  def getBorrowOwnership(coutputs: CompilerOutputs, kind: KindT):
  OwnershipT = {
    kind match {
      case IntT(_) => ShareT
      case BoolT() => ShareT
      case FloatT() => ShareT
      case StrT() => ShareT
      case VoidT() => ShareT
      case contentsStaticSizedArrayTT(_, mutability, _, _, _) => {
        mutability match {
          case MutabilityTemplataT(MutableT) => BorrowT
          case MutabilityTemplataT(ImmutableT) => ShareT
          case PlaceholderTemplataT(idT, MutabilityTemplataType()) => BorrowT
        }
      }
      case contentsRuntimeSizedArrayTT(mutability, _, _) => {
        mutability match {
          case MutabilityTemplataT(MutableT) => BorrowT
          case MutabilityTemplataT(ImmutableT) => ShareT
          case PlaceholderTemplataT(idT, MutabilityTemplataType()) => BorrowT
        }
      }
      case p @ KindPlaceholderT(id) => {
        val mutability = Compiler.getMutability(coutputs, p)
        mutability match {
          case MutabilityTemplataT(MutableT) => BorrowT
          case MutabilityTemplataT(ImmutableT) => ShareT
          case PlaceholderTemplataT(idT, MutabilityTemplataType()) => BorrowT
        }
      }
      case sr2 @ StructTT(_) => {
        val mutability = Compiler.getMutability(coutputs, sr2)
        mutability match {
          case MutabilityTemplataT(MutableT) => BorrowT
          case MutabilityTemplataT(ImmutableT) => ShareT
          case PlaceholderTemplataT(idT, MutabilityTemplataType()) => BorrowT
        }
      }
      case ir2 @ InterfaceTT(_) => {
        val mutability = Compiler.getMutability(coutputs, ir2)
        mutability match {
          case MutabilityTemplataT(MutableT) => BorrowT
          case MutabilityTemplataT(ImmutableT) => ShareT
          case PlaceholderTemplataT(idT, MutabilityTemplataType()) => BorrowT
        }
      }
      case OverloadSetT(_, _) => {
        ShareT
      }
    }
  }
}

object LocalHelper {
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    // See ClosureTests for requirements here
    pub fn determine_if_local_is_addressible(
        &self,
        mutability: ITemplataT<'s, 't>,
        local_a: &'s LocalS<'s>,
    ) -> bool {
        match mutability {
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => {
                local_a.child_mutated != IVariableUseCertainty::NotUsed || local_a.child_moved != IVariableUseCertainty::NotUsed
            }
            _ => {
                local_a.child_mutated != IVariableUseCertainty::NotUsed
            }
        }
    }
/*
  // See ClosureTests for requirements here
  def determineIfLocalIsAddressible(mutability: ITemplataT[MutabilityTemplataType], localA: LocalS): Boolean = {
    mutability match {
      case MutabilityTemplataT(MutableT) => {
        localA.childMutated != NotUsed || localA.childMoved != NotUsed
      }
      case _ => {
        localA.childMutated != NotUsed
      }
    }
  }
*/
    /* Guardian: disable-all */
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn determine_local_variability(
        &self,
        local_a: &'s LocalS<'s>,
    ) -> VariabilityT {
        if local_a.self_mutated != IVariableUseCertainty::NotUsed || local_a.child_mutated != IVariableUseCertainty::NotUsed {
            VariabilityT::Varying
        } else {
            VariabilityT::Final
        }
    }
/*
  def determineLocalVariability(localA: LocalS): VariabilityT = {
    if (localA.selfMutated != NotUsed || localA.childMutated != NotUsed) {
      VaryingT
    } else {
      FinalT
    }
  }
*/
    /* Guardian: disable-all */
}
/*
}
*/