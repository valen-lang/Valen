use crate::postparsing::names::IVarNameS;
use crate::postparsing::rules::RuneUsage;
use crate::utils::range::RangeS;
/*
package dev.vale.postparsing.patterns

import dev.vale.postparsing.IVarNameS
import dev.vale.postparsing.rules.RuneUsage
import dev.vale._
import dev.vale.postparsing._

import scala.collection.immutable.List
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CaptureS<'s> {
  pub name: IVarNameS<'s>,
  pub mutate: bool,
}

/*
case class CaptureS(
    name: IVarNameS,
    mutate: Boolean) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AtomSP<'s> {
  pub range: RangeS<'s>,
  pub name: Option<CaptureS<'s>>,
  pub coord_rune: Option<RuneUsage<'s>>,
  pub destructure: Option<&'s [AtomSP<'s>]>,
}

/*
case class AtomSP(
  range: RangeS,
  // This is an option because in PatternCompiler, if it's None, we'll explode the
  // expression into the destructure or throw the incoming thing away right now (see DIPRA),
  // and if it's Some, we'll make this variable an owning ref.
  // This is a CaptureS instead of a LocalS, which is slightly annoying for Compiler, since it has to
  // remember the LocalSs in scope. But it'd be even more difficult for Scout to know the Used/NotUsed
  // etc up-front to include in the pattern.
  name: Option[CaptureS],
  coordRune: Option[RuneUsage],
  destructure: Option[Vector[AtomSP]]) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()

  name match {
    case Some(CaptureS(CodeVarNameS(StrI("_")), _)) => vwat()
    case _ =>
  }
}
*/
// V: does this need to be clone?
// VA: No. Neither CaptureS nor AtomSP is ever cloned anywhere in the codebase. Both are
// VA: Clone-without-Copy (ATDCX violation). The root blocker for Copy is AtomSP.destructure:
// VA: Option<Vec<AtomSP>> — Vec prevents Copy. If destructure became Option<&'s [AtomSP<'s>]>
// VA: (arena-allocated), then AtomSP could be Copy (all other fields are Copy: IVarNameS, bool,
// VA: RangeS, RuneUsage). CaptureS could then also be Copy. The Vec is also an AASSNCMCX violation.