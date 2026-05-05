use crate::parsing::ast::ast::*;
use crate::typing::types::types::*;

/*
package dev.vale.typing.templata

import dev.vale.parsing.ast.{BorrowP, FinalP, ImmutableP, InlineP, LocationP, MutabilityP, MutableP, OwnP, OwnershipP, ShareP, VariabilityP, VaryingP, WeakP, YonderP}
import dev.vale.typing.types._
import dev.vale.vimpl
import dev.vale.highertyping._
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing.rules._
import dev.vale.{postparsing => s}
import dev.vale.typing.{types => t}
import dev.vale.typing.types._

object Conversions {
*/
pub fn evaluate_mutability(mutability: MutabilityP) -> MutabilityT {
  panic!("Unimplemented: evaluate_mutability");
}
/*
  def evaluateMutability(mutability: MutabilityP): MutabilityT = {
    mutability match {
      case MutableP => MutableT
      case ImmutableP => ImmutableT
    }
  }
*/
pub fn evaluate_location(location: LocationP) -> LocationT {
  panic!("Unimplemented: evaluate_location");
}
/*
  def evaluateLocation(location: LocationP): LocationT = {
    location match {
      case InlineP => InlineT
      case YonderP => YonderT
    }
  }
*/
pub fn evaluate_variability(variability: VariabilityP) -> VariabilityT {
  panic!("Unimplemented: evaluate_variability");
}
/*
  def evaluateVariability(variability: VariabilityP): VariabilityT = {
    variability match {
      case FinalP => FinalT
      case VaryingP => VaryingT
    }
  }
*/
pub fn evaluate_ownership(ownership: OwnershipP) -> OwnershipT {
  match ownership {
    OwnershipP::Own => OwnershipT::Own,
    OwnershipP::Borrow => OwnershipT::Borrow,
    OwnershipP::Weak => OwnershipT::Weak,
    OwnershipP::Share => OwnershipT::Share,
    OwnershipP::Live => { panic!("implement: evaluate_ownership Live"); }
  }
}
/*
  def evaluateOwnership(ownership: OwnershipP): OwnershipT = {
    ownership match {
      case OwnP => OwnT
      case BorrowP => BorrowT
      case WeakP => WeakT
      case ShareP => ShareT
    }
  }
*/
pub fn evaluate_maybe_ownership(maybe_ownership: Option<OwnershipP>) -> Option<OwnershipT> {
  panic!("Unimplemented: evaluate_maybe_ownership");
}
/*
  def evaluateMaybeOwnership(maybeOwnership: Option[OwnershipP]): Option[OwnershipT] = {
    maybeOwnership.map({
      case OwnP => OwnT
      case WeakP => WeakT
      case ShareP => ShareT
    })
  }
*/
pub fn unevaluate_ownership(ownership: OwnershipT) -> OwnershipP {
  panic!("Unimplemented: unevaluate_ownership");
}
/*
  def unevaluateOwnership(ownership: OwnershipT): OwnershipP = {
    ownership match {
      case OwnT => OwnP
      case BorrowT => BorrowP
      case WeakT => WeakP
      case ShareT => ShareP
    }
  }
*/
pub fn unevaluate_mutability(mutability: MutabilityT) -> MutabilityP {
  panic!("Unimplemented: unevaluate_mutability");
}
/*
  def unevaluateMutability(mutability: MutabilityT): MutabilityP = {
    mutability match {
      case MutableT => MutableP
      case ImmutableT => ImmutableP
    }
  }
}
*/
