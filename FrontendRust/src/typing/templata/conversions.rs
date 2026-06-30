use crate::parsing::ast::ast::*;
use crate::typing::types::types::*;


pub fn evaluate_mutability(mutability: SharednessP) -> SharednessT {
  match mutability {
    SharednessP::Single => SharednessT::Single,
    SharednessP::Shared => SharednessT::Shared,
  }
}

pub fn evaluate_location(location: LocationP) -> LocationT {
  panic!("Unimplemented: evaluate_location");
  // location match { case InlineP => InlineT; case YonderP => YonderT }
}

pub fn evaluate_ownership(ownership: OwnershipP) -> OwnershipT {
  match ownership {
    OwnershipP::Own => OwnershipT::Own,
    OwnershipP::Borrow => OwnershipT::Borrow,
    OwnershipP::Weak => OwnershipT::Weak,
    OwnershipP::Share => OwnershipT::Share,
    OwnershipP::Live => { panic!("implement: evaluate_ownership Live"); }
  }
}

pub fn evaluate_maybe_ownership(maybe_ownership: Option<OwnershipP>) -> Option<OwnershipT> {
  panic!("Unimplemented: evaluate_maybe_ownership");
  // maybeOwnership.map({ case OwnP => OwnT; case WeakP => WeakT; case ShareP => ShareT })
}

pub fn unevaluate_ownership(ownership: OwnershipT) -> OwnershipP {
  match ownership {
    OwnershipT::Own => OwnershipP::Own,
    OwnershipT::Borrow => OwnershipP::Borrow,
    OwnershipT::Weak => OwnershipP::Weak,
    OwnershipT::Share => OwnershipP::Share,
  }
}

pub fn unevaluate_mutability(mutability: SharednessT) -> SharednessP {
  panic!("Unimplemented: unevaluate_mutability");
  // mutability match { case MutableT => MutableP; case ImmutableT => ImmutableP }
}

