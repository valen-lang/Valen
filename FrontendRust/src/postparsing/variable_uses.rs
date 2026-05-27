use crate::postparsing::expressions::IVariableUseCertainty;
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::names::IVarNameS;


/*
package dev.vale.postparsing

import dev.vale.{vassert, vcurious, vfail}
import dev.vale.vimpl
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableUseS<'s> {
  pub name: IVarNameS<'s>,
  pub borrowed: Option<IVariableUseCertainty>,
  pub moved: Option<IVariableUseCertainty>,
  pub mutated: Option<IVariableUseCertainty>,
}

/*
case class VariableUse(
    name: IVarNameS,
    borrowed: Option[IVariableUseCertainty],
    moved: Option[IVariableUseCertainty],
    mutated: Option[IVariableUseCertainty]) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableDeclarationS<'s> {
  pub name: IVarNameS<'s>,
}
/*
case class VariableDeclaration(
    name: IVarNameS) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableDeclarations<'s> {
  pub vars: Vec<VariableDeclarationS<'s>>,
}

impl<'s> VariableDeclarations<'s> {
  // MIGALLOW: empty -> empty
  pub fn empty() -> VariableDeclarations<'s> {
    VariableDeclarations { vars: Vec::new() }
  }

/*
case class VariableDeclarations(vars: Vector[VariableDeclaration]) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

  vassert(vars.distinct == vars)
*/
// MIGALLOW: ++ -> plus_plus
pub fn plus_plus(&self, that: &VariableDeclarations<'s>) -> VariableDeclarations<'s> {
  let mut vars = self.vars.clone();
  vars.extend(that.vars.clone());
  VariableDeclarations { vars }
}
/*
  // MIGALLOW: ++ -> plus_plus
  def ++(that: VariableDeclarations): VariableDeclarations = {
    VariableDeclarations(vars ++ that.vars)
  }
*/
pub fn find(&self, needle: &IImpreciseNameS<'s>) -> Option<IVarNameS<'s>> {
  match needle {
    IImpreciseNameS::CodeName(needle_name) => self.vars.iter().find_map(|decl| match &decl.name {
      IVarNameS::CodeVarName(haystack_name) if *haystack_name == needle_name.name => {
        Some(decl.name.clone())
      }
      _ => None,
    }),
    IImpreciseNameS::IterableName(needle_name) => self.vars.iter().find_map(|decl| match &decl.name {
      IVarNameS::IterableName(haystack_name) if *haystack_name == needle_name.range => {
        Some(decl.name.clone())
      }
      _ => None,
    }),
    IImpreciseNameS::IteratorName(needle_name) => self.vars.iter().find_map(|decl| match &decl.name {
      IVarNameS::IteratorName(haystack_name) if *haystack_name == needle_name.range => {
        Some(decl.name.clone())
      }
      _ => None,
    }),
    IImpreciseNameS::IterationOptionName(needle_name) => self.vars.iter().find_map(|decl| match &decl.name {
      IVarNameS::IterationOptionName(haystack_name) if *haystack_name == needle_name.range => {
        Some(decl.name.clone())
      }
      _ => None,
    }),
    IImpreciseNameS::SelfName(_) => self.vars.iter().find_map(|decl| match &decl.name {
      IVarNameS::SelfName => Some(decl.name.clone()),
      _ => None,
    }),
    _ => None,
  }
}
/*
  def find(needle: IImpreciseNameS): Option[IVarNameS] = {
    (needle match {
      case CodeNameS(needle) => {
        vars.map(_.name).collect({ case v @ CodeVarNameS(hay) if hay == needle => v })
      }
      case IterableNameS(needle) => {
        vars.map(_.name).collect({ case v @ IterableNameS(hay) if hay == needle => v })
      }
      case IteratorNameS(needle) => {
        vars.map(_.name).collect({ case v @ IteratorNameS(hay) if hay == needle => v })
      }
      case IterationOptionNameS(needle) => {
        vars.map(_.name).collect({ case v @ IterationOptionNameS(hay) if hay == needle => v })
      }
    }).headOption
  }
}
*/
  
}
/*
Guardian: disable-all
*/

#[derive(Clone, Debug, PartialEq)]
pub struct VariableUses<'s> {
  pub uses: Vec<VariableUseS<'s>>,
}
/*
case class VariableUses(uses: Vector[VariableUse]) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

  vassert(uses.map(_.name).distinct == uses.map(_.name))
*/
impl<'s> VariableUses<'s> {

  // MIGALLOW: empty -> empty
  pub fn empty() -> VariableUses<'s> {
    VariableUses { uses: Vec::new() }
  }

  pub fn is_empty(&self) -> bool {
    self.uses.is_empty()
  }
/*
  def isEmpty: Boolean = uses.isEmpty
*/

  pub fn all_used_names(&self) -> Vec<IVarNameS<'_>> {
    self.uses.iter().map(|use_| use_.name.clone()).collect()
  }
/*
  def allUsedNames: Vector[IVarNameS] = uses.map(_.name)
*/
pub fn mark_borrowed(&self, name: IVarNameS<'s>) -> VariableUses<'s>
{
  self.merge_new_use(VariableUseS {
    name,
    borrowed: Some(IVariableUseCertainty::Used),
    moved: None,
    mutated: None,
  }, Self::then_merge_certainty)
}
/*
  def markBorrowed(name: IVarNameS): VariableUses = {
    merge(VariableUse(name, Some(Used), None, None), thenMerge)
  }
*/
pub fn mark_moved(&self, name: IVarNameS<'s>) -> VariableUses<'s>
{
  self.merge_new_use(VariableUseS {
    name,
    borrowed: None,
    moved: Some(IVariableUseCertainty::Used),
    mutated: None,
  }, Self::then_merge_certainty)
}
/*
  def markMoved(name: IVarNameS): VariableUses = {
    merge(VariableUse(name, None, Some(Used), None), thenMerge)
  }
*/
pub fn mark_mutated(&self, name: IVarNameS<'s>) -> VariableUses<'s>
{
  self.merge_new_use(VariableUseS {
    name,
    borrowed: None,
    moved: None,
    mutated: Some(IVariableUseCertainty::Used),
  }, Self::then_merge_certainty)
}
/*
  def markMutated(name: IVarNameS): VariableUses = {
    merge(VariableUse(name, None, None, Some(Used)), thenMerge)
  }
  // Incorporate this new use into
*/
// MIGALLOW: thenMerge -> then_merge
pub fn then_merge(&self, new_uses: &VariableUses<'s>) -> VariableUses<'s>
{
  self.combine(new_uses, Self::then_merge_certainty)
}
/*
  def thenMerge(newUses: VariableUses): VariableUses = {
    combine(newUses, thenMerge)
  }
*/
// MIGALLOW: branchMerge -> branch_merge
pub fn branch_merge(&self, new_uses: &VariableUses<'s>) -> VariableUses<'s>
{
  self.combine(new_uses, Self::branch_merge_certainty)
}
/*
  def branchMerge(newUses: VariableUses): VariableUses = {
    combine(newUses, branchMerge)
  }
*/
pub fn is_borrowed(&self, name: &IVarNameS<'_>) -> IVariableUseCertainty {
  self
    .uses
    .iter()
    .find(|use_| &use_.name == name)
    .and_then(|use_| use_.borrowed)
    .unwrap_or(IVariableUseCertainty::NotUsed)
}
/*
  def isBorrowed(name: IVarNameS): IVariableUseCertainty = {
    uses.find(_.name == name) match {
      case None => NotUsed
      case Some(use) => use.borrowed.getOrElse(NotUsed)
    }
  }
*/
pub fn is_moved(&self, name: &IVarNameS<'_>) -> IVariableUseCertainty {
  self
    .uses
    .iter()
    .find(|use_| &use_.name == name)
    .and_then(|use_| use_.moved)
    .unwrap_or(IVariableUseCertainty::NotUsed)
}
/*
  def isMoved(name: IVarNameS): IVariableUseCertainty = {
    uses.find(_.name == name) match {
      case None => NotUsed
      case Some(use) => use.moved.getOrElse(NotUsed)
    }
  }
*/
pub fn is_mutated(&self, name: &IVarNameS<'_>) -> IVariableUseCertainty {
  self
    .uses
    .iter()
    .find(|use_| &use_.name == name)
    .and_then(|use_| use_.mutated)
    .unwrap_or(IVariableUseCertainty::NotUsed)
}
/*
  def isMutated(name: IVarNameS): IVariableUseCertainty = {
    uses.find(_.name == name) match {
      case None => NotUsed
      case Some(use) => use.mutated.getOrElse(NotUsed)
    }
  }
*/
fn combine(
  &self,
  that: &VariableUses<'s>,
  certainty_merger: fn(
    Option<IVariableUseCertainty>,
    Option<IVariableUseCertainty>,
  ) -> Option<IVariableUseCertainty>,
) -> VariableUses<'s>
{
  let mut names: Vec<IVarNameS<'_>> = self.uses.iter().map(|use_| use_.name.clone()).collect();
  for name in that.uses.iter().map(|use_| use_.name.clone()) {
    if !names.contains(&name) {
      names.push(name);
    }
  }
  let merged_uses = names
    .into_iter()
    .map(|name| {
      let this_use = self.uses.iter().find(|use_| use_.name == name);
      let that_use = that.uses.iter().find(|use_| use_.name == name);
      match (this_use, that_use) {
        (None, Some(only_that)) => Self::merge_uses(
          &VariableUseS { name: name.clone(), borrowed: None, moved: None, mutated: None },
          only_that,
          certainty_merger,
        ),
        (Some(only_this), None) => Self::merge_uses(
          only_this,
          &VariableUseS { name: name.clone(), borrowed: None, moved: None, mutated: None },
          certainty_merger,
        ),
        (Some(this_use), Some(that_use)) => {
          Self::merge_uses(this_use, that_use, certainty_merger)
        }
        (None, None) => {
          panic!("POSTPARSER_VARIABLE_USES_COMBINE_BOTH_NONE")
        }
      }
    })
    .collect();
  VariableUses { uses: merged_uses }
}
/*
  def combine(
      that: VariableUses,
      certaintyMerger: (Option[IVariableUseCertainty], Option[IVariableUseCertainty]) => Option[IVariableUseCertainty]):
  VariableUses = {
    val mergedUses =
      (uses.map(_.name) ++ that.uses.map(_.name)).distinct.map({ name =>
        (uses.find(_.name == name), that.uses.find(_.name == name)) match {
          case (None, Some(use)) => merge(VariableUse(name, None, None, None), use, certaintyMerger)
          case (Some(use), None) => merge(use, VariableUse(name, None, None, None), certaintyMerger)
          case (Some(thisUse), Some(thatUse)) => merge(thisUse, thatUse, certaintyMerger)
        }
      })
    VariableUses(mergedUses)
  }
*/
fn merge_new_use(
  &self,
  new_use: VariableUseS<'s>,
  certainty_merger: fn(
    Option<IVariableUseCertainty>,
    Option<IVariableUseCertainty>,
  ) -> Option<IVariableUseCertainty>,
) -> VariableUses<'s>
{
  match self.uses.iter().find(|use_| use_.name == new_use.name) {
    None => {
      let mut uses = self.uses.clone();
      uses.push(new_use);
      VariableUses { uses }
    }
    Some(existing_use) => {
      let mut uses: Vec<VariableUseS<'_>> = self
        .uses
        .iter()
        .filter(|use_| use_.name != existing_use.name)
        .cloned()
        .collect();
      uses.push(Self::merge_uses(existing_use, &new_use, certainty_merger));
      VariableUses { uses }
    }
  }
}
/*
  // MIGALLOW: merge -> merge_new_use
  private def merge(
      newUse: VariableUse,
      certaintyMerger: (Option[IVariableUseCertainty], Option[IVariableUseCertainty]) => Option[IVariableUseCertainty]):
  VariableUses = {
    uses.find(_.name == newUse.name) match {
      case None => VariableUses((uses :+ newUse).distinct)
      case Some(existingUse) => {
        VariableUses(uses.filter(_.name != existingUse.name) :+ merge(existingUse, newUse, certaintyMerger))
      }
    }
  }
*/
fn merge_uses<'b>(
  existing_use: &VariableUseS<'b>,
  new_use: &VariableUseS<'b>,
  certainty_merger: fn(
    Option<IVariableUseCertainty>,
    Option<IVariableUseCertainty>,
  ) -> Option<IVariableUseCertainty>,
) -> VariableUseS<'b> {
  VariableUseS {
    name: new_use.name.clone(),
    borrowed: certainty_merger(existing_use.borrowed, new_use.borrowed),
    moved: certainty_merger(existing_use.moved, new_use.moved),
    mutated: certainty_merger(existing_use.mutated, new_use.mutated),
  }
}
/*
  // MIGALLOW: merge -> merge_uses
  private def merge(
      existingUse: VariableUse,
      newUse: VariableUse,
      certaintyMerger: (Option[IVariableUseCertainty], Option[IVariableUseCertainty]) => Option[IVariableUseCertainty]):
  VariableUse = {
    val VariableUse(name, newlyBorrowed, newlyMoved, newlyMutated) = newUse
    val VariableUse(_, alreadyBorrowed, alreadyMoved, alreadyMutated) = existingUse
    VariableUse(
      name,
      certaintyMerger(alreadyBorrowed, newlyBorrowed),
      certaintyMerger(alreadyMoved, newlyMoved),
      certaintyMerger(alreadyMutated, newlyMutated))
  }
*/
fn then_merge_certainty(
  a: Option<IVariableUseCertainty>,
  b: Option<IVariableUseCertainty>,
) -> Option<IVariableUseCertainty> {
  match (a, b) {
    (None, other) => other,
    (other, None) => other,
    (Some(IVariableUseCertainty::NotUsed), Some(IVariableUseCertainty::Used)) => {
      Some(IVariableUseCertainty::Used)
    }
    (Some(IVariableUseCertainty::Used), Some(IVariableUseCertainty::NotUsed)) => {
      Some(IVariableUseCertainty::Used)
    }
    (Some(IVariableUseCertainty::Used), Some(IVariableUseCertainty::Used)) => {
      Some(IVariableUseCertainty::Used)
    }
    (Some(IVariableUseCertainty::NotUsed), Some(IVariableUseCertainty::NotUsed)) => {
      Some(IVariableUseCertainty::NotUsed)
    }
  }
}
// MIGALLOW: thenMerge -> then_merge_certainty
/*
  // If A happens, then B happens, we want the resulting use to reflect that.
  private def thenMerge(
      a: Option[IVariableUseCertainty],
      b: Option[IVariableUseCertainty]):
  Option[IVariableUseCertainty] = {
    (a, b) match {
      case (None, other) => other
      case (other, None) => other
      case (Some(NotUsed), Some(Used)) => Some(Used)
      case (Some(Used), Some(NotUsed)) => Some(Used)
      case (Some(Used), Some(Used)) => Some(Used)
      case (Some(NotUsed), Some(NotUsed)) => Some(NotUsed)
      case _ => vfail("wat")
    }
  }
*/
fn branch_merge_certainty(
  a: Option<IVariableUseCertainty>,
  b: Option<IVariableUseCertainty>,
) -> Option<IVariableUseCertainty> {
  match (a, b) {
    (None, None) => None,
    (None, Some(IVariableUseCertainty::NotUsed)) => Some(IVariableUseCertainty::NotUsed),
    (None, Some(IVariableUseCertainty::Used)) => Some(IVariableUseCertainty::Used),
    (Some(IVariableUseCertainty::NotUsed), None) => Some(IVariableUseCertainty::NotUsed),
    (Some(IVariableUseCertainty::NotUsed), Some(IVariableUseCertainty::NotUsed)) => {
      Some(IVariableUseCertainty::NotUsed)
    }
    (Some(IVariableUseCertainty::NotUsed), Some(IVariableUseCertainty::Used)) => {
      Some(IVariableUseCertainty::Used)
    }
    (Some(IVariableUseCertainty::Used), None) => Some(IVariableUseCertainty::Used),
    (Some(IVariableUseCertainty::Used), Some(IVariableUseCertainty::NotUsed)) => {
      Some(IVariableUseCertainty::Used)
    }
    (Some(IVariableUseCertainty::Used), Some(IVariableUseCertainty::Used)) => {
      Some(IVariableUseCertainty::Used)
    }
  }
}
/*
  // If A happens, OR B happens, we want the resulting use to reflect that.
  // MIGALLOW: branchMerge -> branch_merge_certainty
  private def branchMerge(
      a: Option[IVariableUseCertainty],
      b: Option[IVariableUseCertainty]):
  Option[IVariableUseCertainty] = {
    (a, b) match {
      case (None, None) => None
      case (None, Some(NotUsed)) => Some(NotUsed)
      case (None, Some(Used)) => Some(Used)
      case (Some(NotUsed), None) => Some(NotUsed)
      case (Some(NotUsed), Some(NotUsed)) => Some(NotUsed)
      case (Some(NotUsed), Some(Used)) => Some(Used)
      case (Some(Used), None) => Some(Used)
      case (Some(Used), Some(NotUsed)) => Some(Used)
      case (Some(Used), Some(Used)) => Some(Used)
    }
  }
*/
}
/*
}
*/
