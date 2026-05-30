// From Frontend/SimplifyingPass/src/dev/vale/simplifying/Conversions.scala
#[allow(unused_imports)]
use crate::utils::range::CodeLocationS;
#[allow(unused_imports)]
use crate::instantiating::ast::types::{MutabilityI, VariabilityI, OwnershipI, LocationI};
#[allow(unused_imports)]
use crate::final_ast::types::{CodeLocation, Mutability, Variability, OwnershipH, LocationH};
#[allow(unused_imports)]
use crate::postparsing::itemplatatype::ITemplataType;
/*
package dev.vale.simplifying

import dev.vale.{CodeLocationS, finalast, vimpl}
import dev.vale.finalast._
import dev.vale.postparsing._
import dev.vale.highertyping._
import dev.vale.finalast._
import dev.vale.instantiating.ast._
import dev.vale.postparsing.rules._
import dev.vale.{finalast => m, postparsing => s}

object Conversions {
*/
// mig: fn evaluate_code_location
pub fn evaluate_code_location(loc: CodeLocationS) -> CodeLocation {
    panic!("Unimplemented: evaluate_code_location");
}
/*
  def evaluateCodeLocation(loc: CodeLocationS): CodeLocation = {
    val CodeLocationS(line, col) = loc
    finalast.CodeLocation(line, col)
  }
*/
// mig: fn evaluate_mutability
pub fn evaluate_mutability(mutability: MutabilityI) -> Mutability {
    panic!("Unimplemented: evaluate_mutability");
}
/*
  def evaluateMutability(mutability: MutabilityI): Mutability = {
    mutability match {
      case MutableI => Mutable
      case ImmutableI => Immutable
    }
  }
*/
// mig: fn evaluate_mutability_templata
pub fn evaluate_mutability_templata(mutability: MutabilityI) -> Mutability {
    match mutability {
        MutabilityI::Mutable => Mutability::Mutable,
        MutabilityI::Immutable => Mutability::Immutable,
    }
}
/*
  def evaluateMutabilityTemplata(mutability: MutabilityI): Mutability = {
    mutability match {
      case MutableI => Mutable
      case ImmutableI => Immutable
    }
  }
*/
// mig: fn evaluate_variability_templata
pub fn evaluate_variability_templata(mutability: VariabilityI) -> Variability {
    panic!("Unimplemented: evaluate_variability_templata");
}
/*
  def evaluateVariabilityTemplata(mutability: VariabilityI): Variability = {
    mutability match {
      case VaryingI => Varying
      case FinalI => Final
    }
  }
*/
// mig: fn evaluate_location
pub fn evaluate_location(location: LocationI) -> LocationH {
    panic!("Unimplemented: evaluate_location");
}
/*
  def evaluateLocation(location: LocationI): LocationH = {
    location match {
      case InlineI => InlineH
      case YonderI => YonderH
    }
  }
*/
// mig: fn evaluate_variability
pub fn evaluate_variability(variability: VariabilityI) -> Variability {
    match variability {
        VariabilityI::Final => Variability::Final,
        VariabilityI::Varying => Variability::Varying,
    }
}
/*
  def evaluateVariability(variability: VariabilityI): Variability = {
    variability match {
      case FinalI => Final
      case VaryingI => Varying
    }
  }
*/
// mig: fn evaluate_ownership
pub fn evaluate_ownership(ownership: OwnershipI) -> OwnershipH {
    match ownership {
        OwnershipI::Own => OwnershipH::OwnH,
        OwnershipI::ImmutableBorrow => OwnershipH::ImmutableBorrowH,
        OwnershipI::MutableBorrow => OwnershipH::MutableBorrowH,
        OwnershipI::ImmutableShare => OwnershipH::ImmutableShareH,
        OwnershipI::MutableShare => OwnershipH::MutableShareH,
        OwnershipI::Weak => OwnershipH::WeakH,
    }
}
/*
  def evaluateOwnership(ownership: OwnershipI): OwnershipH = {
    ownership match {
      case OwnI => OwnH
      case ImmutableBorrowI => ImmutableBorrowH
      case MutableBorrowI => MutableBorrowH
      case ImmutableShareI => ImmutableShareH
      case MutableShareI => MutableShareH
      case WeakI => WeakH
    }
  }
*/
// mig: fn unevaluate_templata_type
pub fn unevaluate_templata_type(tyype: ITemplataType) -> ITemplataType {
    panic!("Unimplemented: unevaluate_templata_type");
}
/*
  def unevaluateTemplataType()(tyype: ITemplataType): ITemplataType = {
    tyype match {
      case CoordTemplataType() => CoordTemplataType()
      case KindTemplataType() => KindTemplataType()
      case IntegerTemplataType() => IntegerTemplataType()
      case BooleanTemplataType() => BooleanTemplataType()
      case MutabilityTemplataType() => MutabilityTemplataType()
      case LocationTemplataType() => LocationTemplataType()
      case OwnershipTemplataType() => OwnershipTemplataType()
      case VariabilityTemplataType() => VariabilityTemplataType()
      case TemplateTemplataType(_, _) => vimpl() // can we even specify template types in the syntax?
    }
  }
}
*/
