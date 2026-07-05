// VCOORD: review
//
// Resurrected from the retired `higher_typing/patterns.rs` at commit
// `b5bde70e6` with the onion-era renames applied: `pattern.coord_rune` →
// `pattern.kind_rune` (postparse slice), and the seeded rune type flipped
// from `CoordTemplataType` to `KindTemplataType` (postparse slice retired
// Coord). The pattern-traversal shape is unchanged.
//
// Sole caller today: `typing/expression/expression_compiler.rs` at the
// `IExpressionSE::Let` arm — seeds the rune-type solver with the pattern's
// type-annotated runes before let-binding inference.

use crate::postparsing::patterns::patterns::AtomSP;
use crate::postparsing::names::IRuneS;
use crate::postparsing::itemplatatype::{ITemplataType, KindTemplataType};


pub fn get_rune_types_from_pattern<'s>(
    pattern: &'s AtomSP<'s>,
) -> Vec<(IRuneS<'s>, ITemplataType<'s>)> {
    let mut runes_from_destructures: Vec<(
        IRuneS<'s>,
        ITemplataType<'s>,
    )> = Vec::new();
    if let Some(destructure) = pattern.destructure {
        for sub_pattern in destructure {
            runes_from_destructures.extend(get_rune_types_from_pattern(sub_pattern));
        }
    }
    if let Some(kind_rune) = pattern.kind_rune {
        runes_from_destructures.push((
            kind_rune.rune,
            ITemplataType::KindTemplataType(
                KindTemplataType {},
            ),
        ));
    }
    let mut result: Vec<(
        IRuneS<'s>,
        ITemplataType<'s>,
    )> = Vec::new();
    for item in runes_from_destructures {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    result
}
