#[allow(unused_imports)]
use crate::utils::range::CodeLocationS;
#[allow(unused_imports)]
use crate::instantiating::ast::types::{OwnershipI, LocationI};
#[allow(unused_imports)]
use crate::final_ast::types::{CodeLocation, Sharedness, OwnershipH, LocationH};
#[allow(unused_imports)]
use crate::postparsing::itemplatatype::ITemplataType;

pub fn evaluate_code_location(loc: CodeLocationS) -> CodeLocation {
    panic!("Unimplemented: evaluate_code_location");
    // val CodeLocationS(line, col) = loc
    // finalast.CodeLocation(line, col)
}

pub fn evaluate_location(location: LocationI) -> LocationH {
    panic!("Unimplemented: evaluate_location");
    // location match { case InlineI => InlineH; case YonderI => YonderH }
}


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

pub fn unevaluate_templata_type(tyype: ITemplataType) -> ITemplataType {
    panic!("Unimplemented: unevaluate_templata_type");
    // tyype match {
    //   case CoordTemplataType() => CoordTemplataType()
    //   case KindTemplataType() => KindTemplataType()
    //   case IntegerTemplataType() => IntegerTemplataType()
    //   case BooleanTemplataType() => BooleanTemplataType()
    //   case MutabilityTemplataType() => MutabilityTemplataType()
    //   case LocationTemplataType() => LocationTemplataType()
    //   case OwnershipTemplataType() => OwnershipTemplataType()
    //   case VariabilityTemplataType() => VariabilityTemplataType()
    //   case TemplateTemplataType(_, _) => vimpl()
    // }
}

