use crate::utils::range::RangeS;
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::rune_type_solver::RuneTypeSolveError;
use std::any::Any;
use std::collections::HashSet;
// VISTODO: rename
/*
package dev.vale.highertyping

import dev.vale.{RangeS, vcurious, vpass}
import dev.vale.postparsing.rules.IRulexSR
import dev.vale.postparsing._
import dev.vale.postparsing.RuneTypeSolveError
import dev.vale.RangeS
*/

// mig: struct CompileErrorExceptionA
pub struct CompileErrorExceptionA<'s> {
    pub err: ICompileErrorA<'s>,
}
/*
case class CompileErrorExceptionA(err: ICompileErrorA) extends RuntimeException {
  vpass()
*/
// mig: impl CompileErrorExceptionA
impl<'s> CompileErrorExceptionA<'s> {
// mig: fn equals
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
}
/*
  override def hashCode(): Int = vcurious()
}
*/
// mig: trait ICompileErrorA
pub enum ICompileErrorA<'s> {
    /*
    sealed trait ICompileErrorA {
    */
    CouldntFindType(CouldntFindTypeA<'s>),
    TooManyMatchingTypes(TooManyMatchingTypesA<'s>),
    CouldntSolveRules(CouldntSolveRulesA<'s>),
    CircularModuleDependency(CircularModuleDependency<'s>),
    WrongNumArgsForTemplate(WrongNumArgsForTemplateA<'s>),
    RangedInternalError(RangedInternalErrorA<'s>),
    /*
       def range: RangeS
    */
}
impl<'s> ICompileErrorA<'s> {
    pub fn range(&self) -> RangeS<'s> {
        match self {
            ICompileErrorA::CouldntFindType(x) => x.range.clone(),
            ICompileErrorA::TooManyMatchingTypes(x) => x.range.clone(),
            ICompileErrorA::CouldntSolveRules(x) => x.range.clone(),
            ICompileErrorA::CircularModuleDependency(x) => x.range.clone(),
            ICompileErrorA::WrongNumArgsForTemplate(x) => x.range.clone(),
            ICompileErrorA::RangedInternalError(x) => x.range.clone(),
        }
    }
}
/*
}
*/
// mig: trait ILookupFailedErrorA
pub enum ILookupFailedErrorA<'s> {
    CouldntFindType(CouldntFindTypeA<'s>),
    TooManyMatchingTypes(TooManyMatchingTypesA<'s>),
}
impl<'s> From<ILookupFailedErrorA<'s>> for ICompileErrorA<'s> {
    fn from(e: ILookupFailedErrorA<'s>) -> Self {
        match e {
            ILookupFailedErrorA::CouldntFindType(x) => ICompileErrorA::CouldntFindType(x),
            ILookupFailedErrorA::TooManyMatchingTypes(x) => ICompileErrorA::TooManyMatchingTypes(x),
        }
    }
}
/*
sealed trait ILookupFailedErrorA extends ICompileErrorA
*/
// mig: struct TooManyMatchingTypesA
pub struct TooManyMatchingTypesA<'s> {
    pub range: RangeS<'s>,
    pub name: IImpreciseNameS<'s>,
}
/*
case class TooManyMatchingTypesA(range: RangeS, name: IImpreciseNameS) extends ILookupFailedErrorA {
*/
// mig: impl TooManyMatchingTypesA
impl<'s> TooManyMatchingTypesA<'s> {
// mig: fn equals
// mig: fn hash_code
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CouldntFindTypeA
pub struct CouldntFindTypeA<'s> {
    pub range: RangeS<'s>,
    pub name: IImpreciseNameS<'s>,
}
/*
case class CouldntFindTypeA(range: RangeS, name: IImpreciseNameS) extends ILookupFailedErrorA {
*/
// mig: impl CouldntFindTypeA
impl<'s> CouldntFindTypeA<'s> {
// mig: fn equals
// mig: fn hash_code
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CouldntSolveRulesA
pub struct CouldntSolveRulesA<'s> {
    pub range: RangeS<'s>,
    pub error: RuneTypeSolveError<'s>,
}
/*
case class CouldntSolveRulesA(range: RangeS, error: RuneTypeSolveError) extends ICompileErrorA {
*/
// mig: impl CouldntSolveRulesA
impl<'s> CouldntSolveRulesA<'s> {
// mig: fn equals
// mig: fn hash_code
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
}
*/
// mig: struct CircularModuleDependency
pub struct CircularModuleDependency<'s> {
    pub range: RangeS<'s>,
    pub modules: HashSet<String>,
}
/*
case class CircularModuleDependency(range: RangeS, modules: Set[String]) extends ICompileErrorA { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: impl CircularModuleDependency
impl<'s> CircularModuleDependency<'s> {}
// mig: struct WrongNumArgsForTemplateA
pub struct WrongNumArgsForTemplateA<'s> {
    pub range: RangeS<'s>,
    pub expected_num_args: i32,
    pub actual_num_args: i32,
}
/*
case class WrongNumArgsForTemplateA(range: RangeS, expectedNumArgs: Int, actualNumArgs: Int) extends ICompileErrorA { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: impl WrongNumArgsForTemplateA
impl<'s> WrongNumArgsForTemplateA<'s> {}
// mig: struct RangedInternalErrorA
pub struct RangedInternalErrorA<'s> {
    pub range: RangeS<'s>,
    pub message: String,
}
/*
case class RangedInternalErrorA(range: RangeS, message: String) extends ICompileErrorA { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }

object ErrorReporter {
*/
// mig: impl RangedInternalErrorA
impl<'s> RangedInternalErrorA<'s> {}

// mig: fn report
pub fn report<'s>(_err: ICompileErrorA<'s>) -> ! {
    panic!("Unimplemented: report");
}
/*
  def report(err: ICompileErrorA): Nothing = {
    throw CompileErrorExceptionA(err)
  }
}
*/