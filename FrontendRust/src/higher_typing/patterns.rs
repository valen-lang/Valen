/*
package dev.vale.highertyping

import dev.vale.postparsing._
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.postparsing.patterns._
import dev.vale.postparsing._
import dev.vale.vimpl

import scala.collection.immutable.List

object PatternSUtils {
*/
// mig: fn get_rune_types_from_pattern
pub fn get_rune_types_from_pattern<'s>(
    pattern: &'s AtomSP<'s>,
) -> Vec<(crate::postparsing::names::IRuneS<'s>, crate::postparsing::itemplatatype::ITemplataType<'s>)> {
    let mut runes_from_destructures: Vec<(
        crate::postparsing::names::IRuneS<'s>,
        crate::postparsing::itemplatatype::ITemplataType<'s>,
    )> = Vec::new();
    if let Some(destructure) = pattern.destructure {
        for sub_pattern in destructure {
            runes_from_destructures.extend(get_rune_types_from_pattern(sub_pattern));
        }
    }
    if let Some(coord_rune) = pattern.coord_rune {
        runes_from_destructures.push((
            coord_rune.rune,
            crate::postparsing::itemplatatype::ITemplataType::CoordTemplataType(
                crate::postparsing::itemplatatype::CoordTemplataType {},
            ),
        ));
    }
    let mut result: Vec<(
        crate::postparsing::names::IRuneS<'s>,
        crate::postparsing::itemplatatype::ITemplataType<'s>,
    )> = Vec::new();
    for item in runes_from_destructures {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    result
}
/*
  def getRuneTypesFromPattern(pattern: AtomSP): Iterable[(IRuneS, ITemplataType)] = {
    val runesFromDestructures =
      pattern.destructure.toVector.flatten.flatMap(getRuneTypesFromPattern)
    (runesFromDestructures ++ pattern.coordRune.map(_.rune -> CoordTemplataType())).distinct
  }

}
*/

use crate::postparsing::patterns::patterns::AtomSP;