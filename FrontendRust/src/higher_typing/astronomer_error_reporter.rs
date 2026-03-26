/*
VISTODO: rename
package dev.vale.highertyping

import dev.vale.{RangeS, vcurious, vpass}
import dev.vale.postparsing.rules.IRulexSR
import dev.vale.postparsing._
import dev.vale.postparsing.RuneTypeSolveError
import dev.vale.RangeS
*/
use crate::utils::range::RangeS;
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::rune_type_solver::RuneTypeSolveError;

// mig: struct CompileErrorExceptionA
pub struct CompileErrorExceptionA<'a, 's> {
    pub err: ICompileErrorA<'a, 's>,
}
/*
case class CompileErrorExceptionA(err: ICompileErrorA) extends RuntimeException {
  vpass()
*/
// mig: impl CompileErrorExceptionA
impl<'a, 's> CompileErrorExceptionA<'a, 's> {
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
}
/*
  override def hashCode(): Int = vcurious()
}
*/
// mig: trait ICompileErrorA
pub enum ICompileErrorA<'a, 's> {
    /*
    sealed trait ICompileErrorA {
    */
    CouldntFindType(CouldntFindTypeA<'a>),
    TooManyMatchingTypes(TooManyMatchingTypesA<'a>),
    CouldntSolveRules(CouldntSolveRulesA<'a, 's>),
    CircularModuleDependency(CircularModuleDependency<'a>),
    WrongNumArgsForTemplate(WrongNumArgsForTemplateA<'a>),
    RangedInternalError(RangedInternalErrorA<'a>),
    /*
       def range: RangeS
    */
}
impl<'a, 's> ICompileErrorA<'a, 's> {
    pub fn range(&self) -> RangeS<'a> {
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
pub enum ILookupFailedErrorA<'a> {
    CouldntFindType(CouldntFindTypeA<'a>),
    TooManyMatchingTypes(TooManyMatchingTypesA<'a>),
}
impl<'a, 's> From<ILookupFailedErrorA<'a>> for ICompileErrorA<'a, 's> {
    fn from(e: ILookupFailedErrorA<'a>) -> Self {
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
pub struct TooManyMatchingTypesA<'a> {
    pub range: RangeS<'a>,
    pub name: IImpreciseNameS<'a>,
}
/*
case class TooManyMatchingTypesA(range: RangeS, name: IImpreciseNameS) extends ILookupFailedErrorA {
*/
// mig: impl TooManyMatchingTypesA
impl<'a> TooManyMatchingTypesA<'a> {
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
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
pub struct CouldntFindTypeA<'a> {
    pub range: RangeS<'a>,
    pub name: IImpreciseNameS<'a>,
}
/*
case class CouldntFindTypeA(range: RangeS, name: IImpreciseNameS) extends ILookupFailedErrorA {
*/
// mig: impl CouldntFindTypeA
impl<'a> CouldntFindTypeA<'a> {
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
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
pub struct CouldntSolveRulesA<'a, 's> {
    pub range: RangeS<'a>,
    pub error: RuneTypeSolveError<'a, 's>,
}
/*
case class CouldntSolveRulesA(range: RangeS, error: RuneTypeSolveError) extends ICompileErrorA {
*/
// mig: impl CouldntSolveRulesA
impl<'a, 's> CouldntSolveRulesA<'a, 's> {
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
}
*/
// mig: struct CircularModuleDependency
pub struct CircularModuleDependency<'a> {
    pub range: RangeS<'a>,
    pub modules: std::collections::HashSet<String>,
}
/*
case class CircularModuleDependency(range: RangeS, modules: Set[String]) extends ICompileErrorA { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/
// mig: impl CircularModuleDependency
impl<'a> CircularModuleDependency<'a> {}
// mig: struct WrongNumArgsForTemplateA
pub struct WrongNumArgsForTemplateA<'a> {
    pub range: RangeS<'a>,
    pub expected_num_args: i32,
    pub actual_num_args: i32,
}
/*
case class WrongNumArgsForTemplateA(range: RangeS, expectedNumArgs: Int, actualNumArgs: Int) extends ICompileErrorA { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/
// mig: impl WrongNumArgsForTemplateA
impl<'a> WrongNumArgsForTemplateA<'a> {}
// mig: struct RangedInternalErrorA
pub struct RangedInternalErrorA<'a> {
    pub range: RangeS<'a>,
    pub message: String,
}
/*
case class RangedInternalErrorA(range: RangeS, message: String) extends ICompileErrorA { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }

object ErrorReporter {
*/
// mig: impl RangedInternalErrorA
impl<'a> RangedInternalErrorA<'a> {}

// mig: fn report
pub fn report<'a, 's>(_err: ICompileErrorA<'a, 's>) -> ! {
    panic!("Unimplemented: report");
}
/*
  def report(err: ICompileErrorA): Nothing = {
    throw CompileErrorExceptionA(err)
  }
}
*/