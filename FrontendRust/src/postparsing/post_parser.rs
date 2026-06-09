// From Frontend/PostParsingPass/src/dev/vale/postparsing/PostParser.scala
// Coordinates the Scout (post-parsing) pass

// AFTERM: rename Denizen to Definition, and maybe Citizen to TypeDefinition
// AFTERM: rename ScoutCompilation to PostParserCompilation

use crate::compile_options::GlobalOptions;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::{
  FileP, GenericParameterP, IAttributeP, IDenizenP, IMacroInclusionP, IStructContent, ITemplexPT,
  MutabilityP, MutabilityPT, StructP,
};
use crate::parsing::ast::IRuneAttributeP::{
  AdditiveRegionRuneAttribute, ImmutableRegionRuneAttribute, ImmutableRuneAttribute,
  MutableRuneAttribute, ReadOnlyRegionRuneAttribute, ReadWriteRegionRuneAttribute,
};
use crate::parsing::ast::rules::get_ordered_rune_declarations_from_rulexes_with_duplicates;
use crate::parsing::parser::ParserCompilation;
use crate::postparsing::ast::{
  CoordGenericParameterTypeS, ExportAsS, GenericParameterDefaultS, GenericParameterS,
  IBodyS, ICitizenAttributeS,
  IGenericParameterTypeS, IRegionMutabilityS, ImportS, ImplS, InterfaceS, IStructMemberS,
  LocationInDenizenBuilder, MacroCallS, NormalStructMemberS, OtherGenericParameterTypeS,
  ProgramS, RegionGenericParameterTypeS, StructS, VariadicStructMemberS,
};
use crate::postparsing::expressions::{ConsecutorSE, IExpressionSE};
use crate::postparsing::function_scout::IFunctionParent;
use crate::postparsing::itemplatatype::{
  CoordTemplataType, ITemplataType, KindTemplataType, MutabilityTemplataType, PackTemplataType,
  RegionTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::{
  CodeNameS, CodeRuneS, ExportAsNameS, IFunctionDeclarationNameS, IImpreciseNameS, IImpreciseNameValS,
  INameS, INameValS, DenizenDefaultRegionRuneS, IRuneS, IRuneValS, IVarNameS, ImplDeclarationNameS,
  TopLevelInterfaceDeclarationNameS, TopLevelStructDeclarationNameS,
};
use crate::postparsing::rules::rule_scout::{translate_rulexes, translate_type};
use crate::postparsing::rules::templex_scout::translate_templex;
use crate::postparsing::rules::rules::{
  EqualsSR, IRulexSR, RuneUsage,
};
use crate::postparsing::variable_uses::{VariableDeclarations, VariableUses};
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{FileCoordinate, IPackageResolver, PackageCoordinate};
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::utils::range::{CodeLocationS, RangeS};
use std::collections::HashMap;
use indexmap::IndexSet;
use crate::parsing::ast::IImpreciseNameP;
use crate::postparsing::names::{IterableNameS, IteratorNameS, IterationOptionNameS};
use crate::postparsing::identifiability_solver::IdentifiabilitySolveError;
use crate::parse_arena::ParseArena;
use crate::postparsing::ast::FunctionS;
use crate::parsing::ast::ImplP;
use crate::parsing::ast::IRulexPR;
use crate::parsing::ast::ExportAsP;
use crate::parsing::ast::ImportP;
use crate::postparsing::rules::rules::ILiteralSL;
use crate::postparsing::ast::ExportS;
use crate::postparsing::ast::SealedS;
use crate::parsing::ast::InterfaceP;

/*
package dev.vale.postparsing

//import dev.vale.astronomer.{Astronomer, AstroutsBox, Environment, IRuneS, ITemplataType}
import dev.vale.options.GlobalOptions
import dev.vale.postparsing.patterns.PatternScout
import dev.vale.postparsing.rules.{IRulexSR, LiteralSR, MutabilityLiteralSL, RuleScout, RuneUsage, TemplexScout}
import dev.vale._
import dev.vale.parsing._
import dev.vale.parsing.ast._
import PostParser.{determineDenizenType, evalRange}
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.parsing.ParserCompilation
import dev.vale.parsing.ast.{AnonymousRunePT, BoolPT, CallPT, ExportAsP, ExportAttributeP, FileP, IAttributeP, IImpreciseNameP, ITemplexPT, ImplP, ImportP, InlinePT, IntPT, InterfaceP, InterpretedPT, IterableNameP, IterationOptionNameP, IteratorNameP, LocationPT, LookupNameP, MacroCallP, MutabilityP, MutabilityPT, NameOrRunePT, NameP, NormalStructMemberP, OwnershipPT, RulePUtils, SealedAttributeP, StaticSizedArrayPT, StructMembersP, StructMethodP, StructP, TopLevelExportAsP, TopLevelFunctionP, TopLevelImplP, TopLevelImportP, TopLevelInterfaceP, TopLevelStructP, TuplePT, VariadicStructMemberP, WeakableAttributeP}
//import dev.vale.postparsing.predictor.RuneTypeSolveError
import dev.vale.postparsing.rules._
import dev.vale.Err

import scala.collection.immutable.List
import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
*/
/*
case class CompileErrorExceptionS(err: ICompileErrorS) extends RuntimeException { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
#[derive(Debug, PartialEq)]
pub struct CompileErrorExceptionS<'s> {
  pub err: ICompileErrorS<'s>,
}

#[derive(Clone, Debug, PartialEq)]
// SPORK
// V: whats the common theme between all SPORK comments?
// VA: SPORK marks things that exist in Rust but have no direct Scala counterpart — deviations from
// VA: Scala parity. Examples: Clone+PartialEq derives on types Scala didn't derive (ICompileErrorS,
// VA: IEnvironmentS), Rust-only convenience methods (CodeLocationS::internal, RangeS::file), and
// VA: restructured trait shapes (SolverDelegate::rule_to_puzzles). It flags novel Rust logic for
// VA: review under shields like ATDCX and NCWSRX. 6 occurrences across post_parser.rs, solver.rs,
// VA: and utils/range.rs.
pub enum ICompileErrorS<'s> {
  CouldntFindVarToMutateS(CouldntFindVarToMutateS<'s>),
  CouldntFindRuneS(CouldntFindRuneS<'s>),
  StatementAfterReturnS(StatementAfterReturnS<'s>),
  VariableNameAlreadyExists(VariableNameAlreadyExists<'s>),
  InterfaceMethodNeedsSelf(InterfaceMethodNeedsSelf<'s>),
  VirtualAndAbstractGoTogether(VirtualAndAbstractGoTogether<'s>),
  RuneExplicitTypeConflictS(RuneExplicitTypeConflictS<'s>),
  InitializingRuntimeSizedArrayRequiresSizeAndCallable(
    InitializingRuntimeSizedArrayRequiresSizeAndCallable<'s>,
  ),
  InitializingStaticSizedArrayRequiresSizeAndCallable(
    InitializingStaticSizedArrayRequiresSizeAndCallable<'s>,
  ),
  ExternHasBodyS(ExternHasBodyS<'s>),
  IdentifyingRunesIncompleteS(IdentifyingRunesIncompleteS<'s>),
  RangedInternalErrorS(RangedInternalErrorS<'s>),
}
/*
sealed trait ICompileErrorS { def range: RangeS }
*/

impl ICompileErrorS<'_> {
  pub fn range(&self) -> &RangeS<'_> {
    match self {
      ICompileErrorS::CouldntFindVarToMutateS(x) => &x.range,
      ICompileErrorS::CouldntFindRuneS(x) => &x.range,
      ICompileErrorS::StatementAfterReturnS(x) => &x.range,
      ICompileErrorS::VariableNameAlreadyExists(x) => &x.range,
      ICompileErrorS::InterfaceMethodNeedsSelf(x) => &x.range,
      ICompileErrorS::VirtualAndAbstractGoTogether(x) => &x.range,
      ICompileErrorS::RuneExplicitTypeConflictS(x) => &x.range,
      ICompileErrorS::InitializingRuntimeSizedArrayRequiresSizeAndCallable(x) => &x.range,
      ICompileErrorS::InitializingStaticSizedArrayRequiresSizeAndCallable(x) => &x.range,
      ICompileErrorS::ExternHasBodyS(x) => &x.range,
      ICompileErrorS::IdentifyingRunesIncompleteS(x) => &x.range,
      ICompileErrorS::RangedInternalErrorS(x) => &x.range,
    }
  }
  /*
  Guardian: disable-all
  */
}
/*
Guardian: disable-all
*/

/*
case class UnknownRuleFunctionS(range: RangeS, name: String) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
case class BadRuneAttributeErrorS(range: RangeS, attr: IRuneAttributeP) extends ICompileErrorS {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CantHaveMultipleMutabilitiesS(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
case class UnimplementedExpression(range: RangeS, expressionName: String) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
#[derive(Clone, Debug, PartialEq)]
pub struct CouldntFindVarToMutateS<'s> {
  pub range: RangeS<'s>,
  pub name: String,
}
/*
case class CouldntFindVarToMutateS(range: RangeS, name: String) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct CouldntFindRuneS<'s> {
  pub range: RangeS<'s>,
  pub name: String,
}
/*
case class CouldntFindRuneS(range: RangeS, name: String) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StatementAfterReturnS<'s> {
  pub range: RangeS<'s>,
}
/*
case class StatementAfterReturnS(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

/*
case class ForgotSetKeywordError(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
case class UnknownRegionError(range: RangeS, name: String) extends ICompileErrorS {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExternHasBodyS<'s> {
  pub range: RangeS<'s>,
}
/*
case class ExternHasBody(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializingRuntimeSizedArrayRequiresSizeAndCallable<'s> {
  pub range: RangeS<'s>,
}
/*
case class InitializingRuntimeSizedArrayRequiresSizeAndCallable(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializingStaticSizedArrayRequiresSizeAndCallable<'s> {
  pub range: RangeS<'s>,
}
/*
case class InitializingStaticSizedArrayRequiresSizeAndCallable(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

/*
case class CantOwnershipInterfaceInImpl(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
case class CantOwnershipStructInImpl(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
case class CantOverrideOwnershipped(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VariableNameAlreadyExists<'s> {
  pub range: RangeS<'s>,
  pub name: IVarNameS<'s>,
}
/*
case class VariableNameAlreadyExists(range: RangeS, name: IVarNameS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InterfaceMethodNeedsSelf<'s> {
  pub range: RangeS<'s>,
}
/*
case class InterfaceMethodNeedsSelf(range: RangeS) extends ICompileErrorS {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VirtualAndAbstractGoTogether<'s> {
  pub range: RangeS<'s>,
}
/*
case class VirtualAndAbstractGoTogether(range: RangeS) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

/*
case class CouldntSolveRulesS(range: RangeS, error: RuneTypeSolveError) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct RuneExplicitTypeConflictS<'s> {
  pub range: RangeS<'s>,
  pub rune: IRuneS<'s>,
  pub types: Vec<ITemplataType<'s>>,
}
/*
case class RuneExplicitTypeConflictS(range: RangeS, rune: IRuneS, types: Vector[ITemplataType]) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct IdentifyingRunesIncompleteS<'s> {
  pub range: RangeS<'s>,
  pub error: IdentifiabilitySolveError<'s>,
}
/*
case class IdentifyingRunesIncompleteS(range: RangeS, error: IdentifiabilitySolveError) extends ICompileErrorS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RangedInternalErrorS<'s> {
  pub range: RangeS<'s>,
  pub message: String,
}
/*
case class RangedInternalErrorS(range: RangeS, message: String) extends ICompileErrorS {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
// SPORK
pub enum IEnvironmentS<'s> {
  Environment(EnvironmentS<'s>),
  FunctionEnvironment(FunctionEnvironmentS<'s>),
}
/*
sealed trait IEnvironmentS {
*/
impl<'s> IEnvironmentS<'s> {
  pub fn file(&self) -> &'s FileCoordinate<'s> {
    match self {
      IEnvironmentS::Environment(environment) => environment.file,
      IEnvironmentS::FunctionEnvironment(function_environment) => function_environment.file,
    }
  }
  /*
    def file: FileCoordinate
  */

  /*
    def name: INameS
  */

  pub fn all_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    match self {
      IEnvironmentS::Environment(environment) => environment.all_declared_runes(),
      IEnvironmentS::FunctionEnvironment(function_environment) => function_environment.all_declared_runes(),
    }
  }
  /*
    def allDeclaredRunes(): Set[IRuneS]
  */
  pub fn local_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    match self {
      IEnvironmentS::Environment(environment) => environment.local_declared_runes(),
      IEnvironmentS::FunctionEnvironment(function_environment) => {
        function_environment.local_declared_runes()
      }
    }
  }
  /*
    def localDeclaredRunes(): Set[IRuneS]
  */
/*
}
*/
}


#[derive(Clone, Debug, PartialEq)]
pub struct EnvironmentS<'s> {
  pub file: &'s FileCoordinate<'s>,
  pub parent_env: Option<Box<EnvironmentS<'s>>>,
  pub name: INameS<'s>,
  pub user_declared_runes: IndexSet<IRuneS<'s>>,
}
/*
// Someday we might split this into PackageEnvironment and CitizenEnvironment
case class EnvironmentS(
    file: FileCoordinate,
    parentEnv: Option[EnvironmentS],
    name: INameS,
    userDeclaredRunes: Set[IRuneS]
) extends IEnvironmentS {
*/
impl<'s> EnvironmentS<'s> {
/*
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/

  pub fn local_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    self.user_declared_runes.clone()
  }
  /*
    override def localDeclaredRunes(): Set[IRuneS] = {
      userDeclaredRunes
    }
  */

  pub fn all_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    let mut runes = self.user_declared_runes.clone();
    if let Some(parent_env) = &self.parent_env {
      runes.extend(parent_env.all_declared_runes());
    }
    runes
  }
  /*
    override def allDeclaredRunes(): Set[IRuneS] = {
      userDeclaredRunes ++ parentEnv.toVector.flatMap(_.allDeclaredRunes())
    }
  */
}
/*
Guardian: disable-all
*/
/*
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionEnvironmentS<'s> {
  pub file: &'s FileCoordinate<'s>,
  pub name: IFunctionDeclarationNameS<'s>,
  pub parent_env: Option<Box<IEnvironmentS<'s>>>,
  pub declared_runes: IndexSet<IRuneS<'s>>,
  pub num_explicit_params: i32,
  pub is_interface_internal_method: bool,
}
/*
case class FunctionEnvironmentS(
  file: FileCoordinate,
  name: IFunctionDeclarationNameS,
  parentEnv: Option[IEnvironmentS],
  // Contains all the identifying runes and otherwise declared runes from this function's rules.
  // These are important for knowing whether e.g. T is a type or a rune when we process all the runes.
  // See: Must Scan For Declared Runes First (MSFDRF)
  private val declaredRunes: Set[IRuneS],
  // So that when we run into a magic param, we can add this to the number of previous magic
  // params to get the final param index.
  numExplicitParams: Int,
  // Whether this is an abstract method inside defined inside an interface.
  // (Maybe we can instead determine this by looking at parentEnv?)
  isInterfaceInternalMethod: Boolean
) extends IEnvironmentS {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/

impl<'s> FunctionEnvironmentS<'s> {
  pub fn local_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    self.declared_runes.clone()
  }
/*
  override def localDeclaredRunes(): Set[IRuneS] = {
    declaredRunes
  }
*/
  pub fn all_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    let mut runes = self.declared_runes.clone();
    if let Some(parent_env) = &self.parent_env {
      runes.extend(parent_env.all_declared_runes());
    }
    runes
  }
/*
  override def allDeclaredRunes(): Set[IRuneS] = {
    declaredRunes ++ parentEnv.toVector.flatMap(_.allDeclaredRunes()).toSet
  }
*/
  pub fn child(&self) -> FunctionEnvironmentS<'s> {
    FunctionEnvironmentS::<'s> {
      file: self.file,
      name: self.name.clone(),
      parent_env: Some(Box::new(IEnvironmentS::FunctionEnvironment(self.clone()))),
      declared_runes: IndexSet::new(),
      num_explicit_params: self.num_explicit_params,
      is_interface_internal_method: false,
    }
  }
/*
  def child(): FunctionEnvironmentS = {
    FunctionEnvironmentS(file, name, Some(this), Set(), numExplicitParams, false)
  }
}
*/

}
/*
Guardian: disable-all
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StackFrame<'s> {
  pub file: &'s FileCoordinate<'s>,
  pub name: IFunctionDeclarationNameS<'s>,
  pub parent_env: FunctionEnvironmentS<'s>,
  pub maybe_parent: Option<Box<StackFrame<'s>>>,
  pub context_region: IRuneS<'s>,
  pub pure_height: i32,
  pub locals: VariableDeclarations<'s>,
}
/*
case class StackFrame(
    file: FileCoordinate,
    name: IFunctionDeclarationNameS,
    parentEnv: FunctionEnvironmentS,
    maybeParent: Option[StackFrame],
    contextRegion: IRuneS,
    pureHeight: Int,
    locals: VariableDeclarations) {
*/
impl<'s> StackFrame<'s> {
/*
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/
// MIGALLOW: ++ -> plus
pub fn plus(&self, new_vars: &VariableDeclarations<'s>) -> StackFrame<'s> {
  StackFrame::<'s> {
    file: self.file,
    name: self.name.clone(),
    parent_env: self.parent_env.clone(),
    maybe_parent: self.maybe_parent.clone(),
    context_region: self.context_region.clone(),
    pure_height: self.pure_height,
    locals: self.locals.plus_plus(new_vars),
  }
}
/*
  def ++(newVars: VariableDeclarations): StackFrame = {
    StackFrame(file, name, parentEnv, maybeParent, contextRegion, pureHeight, locals ++ newVars)
  }
*/

pub fn all_declarations(&self) -> VariableDeclarations<'_> {
  match &self.maybe_parent {
    Some(parent) => self.locals.plus_plus(&parent.all_declarations()),
    None => self.locals.plus_plus(&PostParser::<'s, '_, '_>::no_declarations()),
  }
}
/*
  def allDeclarations: VariableDeclarations = {
    locals ++ maybeParent.map(_.allDeclarations).getOrElse(PostParser.noDeclarations)
  }
*/
pub fn find_variable(&self, name: &IImpreciseNameS<'s>) -> Option<IVarNameS<'s>> {
  match self.locals.find(name) {
    Some(full_name_s) => Some(full_name_s),
    None => match &self.maybe_parent {
      None => None,
      Some(parent) => parent.find_variable(name),
    },
  }
}
/*
  def findVariable(name: IImpreciseNameS): Option[IVarNameS] = {
    locals.find(name) match {
      case Some(fullNameS) => Some(fullNameS)
      case None => {
        maybeParent match {
          case None => None
          case Some(parent) => parent.findVariable(name)
        }
      }
    }
  }
  */
}
/*
Guardian: disable-all
*/
/*
}
*/
/*
object PostParser {
//  val VIRTUAL_DROP_FUNCTION_NAME = "vdrop"
  // Interface's drop function simply calls vdrop.
  // A struct's vdrop function calls the struct's drop function.
*/
// MIGALLOW: noVariableUses -> no_variable_uses
// MIGALLOW: noDeclarations -> no_declarations
impl<'s, 'p, 'ctx> PostParser<'s, 'p, 'ctx>
{
  pub fn no_variable_uses() -> VariableUses<'s> {
    VariableUses::<'s>::empty()
  }
  /*
  def noVariableUses = VariableUses(Vector.empty)
  */
  // AFTERM: consider moving no_declarations out of PostParser
  pub fn no_declarations() -> VariableDeclarations<'s> {
    VariableDeclarations { vars: Vec::new() }
  }
  /*
  def noDeclarations = VariableDeclarations(Vector.empty)
  */

  pub fn eval_range(file: &'s FileCoordinate<'s>, range: RangeL) -> RangeS<'s> {
    RangeS::new(
      Self::eval_pos(file, range.begin()),
      Self::eval_pos(file, range.end()),
    )
  }
  /*
  def evalRange(file: FileCoordinate, range: RangeL): RangeS = {
    RangeS(evalPos(file, range.begin), evalPos(file, range.end))
  }
  */
}

impl<'s, 'p, 'ctx> PostParser<'s, 'p, 'ctx>
{
  pub fn eval_pos(file: &'s FileCoordinate<'s>, pos: i32) -> CodeLocationS<'s> {
    CodeLocationS {
      file,
      offset: pos,
    }
  }
  /*
    def evalPos(file: FileCoordinate, pos: Int): CodeLocationS = {
      CodeLocationS(file, pos)
    }
  */
}

pub(crate) fn translate_imprecise_name<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  file: &'s FileCoordinate<'s>,
  name: &IImpreciseNameP<'p>,
) -> IImpreciseNameS<'s> {
  match name {
    // Re-intern string from 'p into 's
    IImpreciseNameP::LookupName(n) => scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str(n.str().as_str()) })),
    IImpreciseNameP::IterableName(range) => scout_arena.intern_imprecise_name(IImpreciseNameValS::IterableName(IterableNameS {
      range: PostParser::eval_range(file, *range),
    })),
    IImpreciseNameP::IteratorName(range) => scout_arena.intern_imprecise_name(IImpreciseNameValS::IteratorName(IteratorNameS {
      range: PostParser::eval_range(file, *range),
    })),
    IImpreciseNameP::IterationOptionName(range) => scout_arena.intern_imprecise_name(IImpreciseNameValS::IterationOptionName(IterationOptionNameS {
      range: PostParser::eval_range(file, *range),
    })),
  }
}
/*
  def translateImpreciseName(interner: Interner, file: FileCoordinate, name: IImpreciseNameP): IImpreciseNameS = {
    name match {
      case LookupNameP(name) => interner.intern(CodeNameS(name.str))
      case IterableNameP(range) => IterableNameS(PostParser.evalRange(file,  range))
      case IteratorNameP(range) => IteratorNameS(PostParser.evalRange(file,  range))
      case IterationOptionNameP(range) => IterationOptionNameS(PostParser.evalRange(file,  range))
    }
  }
*/
fn determine_denizen_type<'s>(
  _template_result_type: ITemplataType<'s>,
  _identifying_runes_s: &[IRuneS<'s>],
  _rune_a_to_type: &std::collections::HashMap<IRuneS<'s>, ITemplataType<'s>>,
) -> Result<ITemplataType<'s>, IRuneS<'s>> {
  panic!("Unimplemented determine_denizen_type");
}
/*
  // Err is the missing rune
  def determineDenizenType(
    templateResultType: ITemplataType,
    identifyingRunesS: Vector[IRuneS],
    runeAToType: Map[IRuneS, ITemplataType]):
  Result[ITemplataType, IRuneS] = {
    val isTemplate = identifyingRunesS.nonEmpty

    val tyype =
      if (isTemplate) {
        TemplateTemplataType(
          identifyingRunesS.map(identifyingRuneA => {
            runeAToType.get(identifyingRuneA) match {
              case None => return Err(identifyingRuneA)
              case Some(x) => x
            }
          }),
          templateResultType)
      } else {
        templateResultType
      }
    Ok(tyype)
  }
*/
fn get_human_name<'s, 'p>(
  _scout_arena: &ScoutArena<'s>,
  _templex: &ITemplexPT<'p>,
) -> IImpreciseNameS<'s> {
  panic!("Unimplemented get_human_name");
}
/*
  def getHumanName(interner: Interner, templex: ITemplexPT): IImpreciseNameS = {
    templex match {
      //      case NullablePT(_, inner) => getHumanName(inner)
      case InlinePT(_, inner) => getHumanName(interner, inner)
      //      case PermissionedPT(_, permission, inner) => getHumanName(inner)
      case InterpretedPT(_, ownership, region, inner) => getHumanName(interner, inner)
      case AnonymousRunePT(_) => vwat()
      case NameOrRunePT(NameP(_, name)) => interner.intern(CodeNameS(name))
      case CallPT(_, template, args) => getHumanName(interner, template)
      case StaticSizedArrayPT(_, mutability, variability, size, element) => vwat()
      case TuplePT(_, members) => vwat()
      case IntPT(_, value) => vwat()
      case BoolPT(_, value) => vwat()
      case OwnershipPT(_, ownership) => vwat()
      case MutabilityPT(_, mutability) => vwat()
      case LocationPT(_, location) => vwat()
    }
  }
*/
impl<'s, 'p, 'ctx> PostParser<'s, 'p, 'ctx>
{
  pub fn consecutive(&self, exprs: Vec<&'s IExpressionSE<'s>>) -> &'s IExpressionSE<'s> {
    assert!(!exprs.is_empty(), "POSTPARSER_CONSECUTIVE_EMPTY");
    if exprs.len() == 1 {
      return exprs.into_iter().next().unwrap();
    }
    let mut flattened = Vec::new();
    for expr in exprs {
      match expr {
        IExpressionSE::Consecutor(consecutor) => flattened.extend(consecutor.exprs.iter().copied()),
        other => flattened.push(other),
      }
    }
    let slice = self.scout_arena.alloc_slice_from_vec(flattened);
    &*self.scout_arena.alloc(IExpressionSE::Consecutor(ConsecutorSE { exprs: slice }))
  }
/*
  def consecutive(exprs: Vector[IExpressionSE]): IExpressionSE = {
    if (exprs.isEmpty) {
      vcurious()
    } else if (exprs.size == 1) {
      exprs.head
    } else {
      ConsecutorSE(
        exprs.flatMap({
          case ConsecutorSE(exprs) => exprs
          case other => List(other)
        }))
    }
  }
*/
pub(crate) fn scout_generic_parameter(
  &self,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rune_to_explicit_type: &mut Vec<(IRuneS<'s>, ITemplataType<'s>)>,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  // This might seem a bit weird, because the region rune usually comes last and is usually
  // mentioned at the end of the header too. But indeed we need it for knowing the region to use
  // for generic params' default values.
  context_region: IRuneS<'s>,
  generic_param_p: &GenericParameterP<'p>,
  param_rune_s: RuneUsage<'s>,
  // Returns a possible implicit region generic param (see MNRFGC), and the translated original
  // generic param.
) -> GenericParameterS<'s> {
  let file = env.file();
  let generic_param_range_s = PostParser::eval_range(file, generic_param_p.range);
  let rune_s = param_rune_s;

  let type_s = match &generic_param_p.maybe_type {
    None => ITemplataType::CoordTemplataType(CoordTemplataType {}),
    Some(type_p) => translate_type(self.scout_arena, type_p.tyype),
  };
  rune_to_explicit_type.push((rune_s.rune.clone(), type_s.clone()));

  assert!(
    generic_param_p.coord_region.is_none(),
    "POSTPARSER_SCOUT_GENERIC_PARAMETER_COORD_REGION_NOT_YET_IMPLEMENTED"
  );

  let generic_param_type_s = match type_s {
    ITemplataType::CoordTemplataType(_) => {
      let immutable_attrs = generic_param_p
        .attributes
        .iter()
        .filter(|x| matches!(x, ImmutableRuneAttribute(_)))
        .collect::<Vec<_>>();
      let mutable_attrs = generic_param_p
        .attributes
        .iter()
        .filter(|x| matches!(x, MutableRuneAttribute(_)))
        .collect::<Vec<_>>();
      let remaining_attributes = generic_param_p
        .attributes
        .iter()
        .filter(|x| {
          !matches!(
            x,
            ImmutableRuneAttribute(_) | MutableRuneAttribute(_)
          )
        })
        .collect::<Vec<_>>();
      if !remaining_attributes.is_empty() {
        panic!("POSTPARSER_SCOUT_GENERIC_PARAMETER_BAD_COORD_RUNE_ATTRIBUTE");
      }
      IGenericParameterTypeS::CoordGenericParameterType(CoordGenericParameterTypeS {
        coord_region: None,
        // vregionmut() // we really need to figure out this kind immutable stuff.
        kind_mutable: immutable_attrs.is_empty(),
        region_mutable: !mutable_attrs.is_empty(),
      })
    }
    ITemplataType::RegionTemplataType(_) => {
      let mutability_attrs = generic_param_p
        .attributes
        .iter()
        .filter_map(|x| match x {
          ImmutableRegionRuneAttribute(_) => {
            Some(IRegionMutabilityS::ImmutableRegion)
          }
          AdditiveRegionRuneAttribute(_) => {
            Some(IRegionMutabilityS::AdditiveRegion)
          }
          ReadWriteRegionRuneAttribute(_) => {
            Some(IRegionMutabilityS::ReadWriteRegion)
          }
          ReadOnlyRegionRuneAttribute(_) => {
            Some(IRegionMutabilityS::ReadOnlyRegion)
          }
          _ => None,
        })
        .collect::<Vec<_>>();
      let remaining_attributes = generic_param_p
        .attributes
        .iter()
        .filter(|x| {
          !matches!(
            x,
            ImmutableRegionRuneAttribute(_)
              | AdditiveRegionRuneAttribute(_)
              | ReadWriteRegionRuneAttribute(_)
              | ReadOnlyRegionRuneAttribute(_)
          )
        })
        .collect::<Vec<_>>();
      if !remaining_attributes.is_empty() {
        panic!("POSTPARSER_SCOUT_GENERIC_PARAMETER_BAD_REGION_RUNE_ATTRIBUTE");
      }
      if mutability_attrs.len() > 1 {
        panic!("POSTPARSER_SCOUT_GENERIC_PARAMETER_MULTIPLE_REGION_MUTABILITIES");
      }
      IGenericParameterTypeS::RegionGenericParameterType(
        RegionGenericParameterTypeS {
        mutability: mutability_attrs
          .first()
          .cloned()
          .unwrap_or(IRegionMutabilityS::ReadOnlyRegion),
        },
      )
    }
    _ => {
      if !generic_param_p.attributes.is_empty() {
        panic!("POSTPARSER_SCOUT_GENERIC_PARAMETER_BAD_OTHER_RUNE_ATTRIBUTE");
      }
      IGenericParameterTypeS::OtherGenericParameterType(
        OtherGenericParameterTypeS::new(type_s),
      )
    }
  };
  let default_s = generic_param_p.maybe_default.map(|default_pt| {
    let mut uncategorized_rules = Vec::new();
    let result_rune = translate_templex(
      self.scout_arena, self.keywords, env, lidb, &mut uncategorized_rules, context_region, &default_pt,
    );
    uncategorized_rules.push(IRulexSR::Equals(EqualsSR {
      range: generic_param_range_s,
      left: rune_s.clone(),
      right: result_rune.clone(),
    }));

    let mut rules_to_leave_in_default_argument = Vec::new();
    for r in uncategorized_rules {
      match r {
        IRulexSR::Pack(_) => rule_builder.push(r), // Hoist it up into regular rules
        IRulexSR::Literal(_) => rules_to_leave_in_default_argument.push(&*self.scout_arena.alloc(r)),
        IRulexSR::MaybeCoercingLookup(_) => rules_to_leave_in_default_argument.push(&*self.scout_arena.alloc(r)),
        IRulexSR::Resolve(_) => rules_to_leave_in_default_argument.push(&*self.scout_arena.alloc(r)),
        // Per @DRSINI, this EqualsSR aliases the param rune to the default's resultRune.
        // We KEEP it in the default's rules (rather than hoisting) so the default is fully
        // self-contained — it travels intact when GenericParameterS is inherited (e.g. by
        // struct internal methods). At default-fire time, the typing pass registers the
        // default-only runes via solverState.registerRunes(default.runeToType.keys).
        IRulexSR::Equals(_) => rules_to_leave_in_default_argument.push(&*self.scout_arena.alloc(r)),
        IRulexSR::CallSiteFunc(_) => rule_builder.push(r), // Hoist it up into regular rules
        IRulexSR::DefinitionFunc(_) => rule_builder.push(r), // Hoist it up into regular rules
        other => panic!("vwat: {:?}", other),
      }
    }

    let default_rune_to_type = self.scout_arena.alloc_slice_from_vec(
      vec![(result_rune.rune.clone(), generic_param_type_s.tyype())]);
    GenericParameterDefaultS {
      result_rune: result_rune.rune,
      rules: self.scout_arena.alloc_slice_from_vec(rules_to_leave_in_default_argument),
      rune_to_type: default_rune_to_type,
    }
  });

  return GenericParameterS {
      range: generic_param_range_s,
      rune: rune_s,
      tyype: generic_param_type_s,
      default: default_s,
    };
  }
/*
  def scoutGenericParameter(
      templexScout: TemplexScout,
      env: IEnvironmentS,
      lidb: LocationInDenizenBuilder,
      runeToExplicitType: mutable.ArrayBuffer[(IRuneS, ITemplataType)],
      ruleBuilder: ArrayBuffer[IRulexSR],
      // This might seem a bit weird, because the region rune usually comes last and is usually
      // mentioned at the end of the header too. But indeed we need it for knowing the region to use
      // for generic params' default values.
      contextRegion: IRuneS,
      genericParamP: GenericParameterP,
      paramRuneS: RuneUsage):
  // Returns a possible implicit region generic param (see MNRFGC), and the translated original
  // generic param.
  GenericParameterS  = {
    val GenericParameterP(genericParamRangeP, _, maybeType, maybeCoordRegionP, originalAttributesP, maybeDefault) = genericParamP
    val genericParamRangeS = PostParser.evalRange(env.file, genericParamRangeP)
    val runeS = paramRuneS

    val typeS =
      maybeType match {
        case None => CoordTemplataType()
        case Some(typeP) => RuleScout.translateType(typeP.tyype)
      }
    runeToExplicitType += ((runeS.rune, typeS))

    val maybeExplicitCoordRegionS =
      maybeCoordRegionP match {
        case Some(RegionRunePT(rangeP, name)) => {
          val rangeS = PostParser.evalRange(env.file, rangeP)
          Some(RuneUsage(rangeS, CodeRuneS(vassertSome(name).str))) // impl isolates
        }
        case None => None
      }


    val genericParamTypeS =
      typeS match {
        case CoordTemplataType() => {
          val (immutableAttrs, remainingAttributes1P) =
            U.extract[IRuneAttributeP, Unit](originalAttributesP, {
              case ImmutableRuneAttributeP(rangeP) => Unit
            })
          val (mutableAttrs, remainingAttributes0P) =
            U.extract[IRuneAttributeP, Unit](remainingAttributes1P, {
              case MutableRuneAttributeP(rangeP) => Unit
            })
          if (remainingAttributes0P.nonEmpty) {
            val first = remainingAttributes0P.head
            throw CompileErrorExceptionS(BadRuneAttributeErrorS(evalRange(env.file, first.range), first))
          }
          vregionmut() // we really need to figure out this kind immutable stuff.
          val kindMutable = immutableAttrs.isEmpty
          val regionMutable = mutableAttrs.nonEmpty
          CoordGenericParameterTypeS(None, kindMutable, regionMutable)
        }
        case RegionTemplataType() => {
          val (mutabilityAttrsS, remainingAttributesP) =
            U.extract[IRuneAttributeP, IRegionMutabilityS](originalAttributesP, {
              case ImmutableRegionRuneAttributeP(_) => ImmutableRegionS
              case AdditiveRegionRuneAttributeP(_) => AdditiveRegionS
              case ReadWriteRegionRuneAttributeP(_) => ReadWriteRegionS
              case ReadOnlyRegionRuneAttributeP(_) => ReadOnlyRegionS
            })
          if (remainingAttributesP.nonEmpty) {
            val first = remainingAttributesP.head
            throw CompileErrorExceptionS(BadRuneAttributeErrorS(evalRange(env.file, first.range), first))
          }

          if (mutabilityAttrsS.size > 1) {
            throw CompileErrorExceptionS(CantHaveMultipleMutabilitiesS(evalRange(env.file, genericParamRangeP)))
          }
          val mutability = mutabilityAttrsS.headOption.getOrElse(ReadOnlyRegionS)
          RegionGenericParameterTypeS(mutability)
        }
        case other => {
          if (originalAttributesP.nonEmpty) {
            val first = originalAttributesP.head
            throw CompileErrorExceptionS(BadRuneAttributeErrorS(evalRange(env.file, first.range), first))
          }
          OtherGenericParameterTypeS(other)
        }
      }

    val defaultS =
      maybeDefault.map(defaultPT => {
        val uncategorizedRules = ArrayBuffer[IRulexSR]()
        val resultRune = templexScout.translateTemplex(env, lidb, uncategorizedRules, contextRegion, defaultPT)
        // Per @DRSINI, this EqualsSR is hoisted into main rules below. It just aliases the
        // param rune and the default result rune — harmless until LiteralSR gives the result
        // rune a value. LiteralSR is added incrementally (not eagerly) by solveForResolving.
        uncategorizedRules += EqualsSR(genericParamRangeS, runeS, resultRune)

        val rulesToLeaveInDefaultArgument = new Accumulator[IRulexSR]()
        uncategorizedRules.foreach({
          case r @ PackSR(_, _, _) => ruleBuilder += r // Hoist it up into regular rules
          case r @ LiteralSR(_, _, _) => rulesToLeaveInDefaultArgument.add(r)
          case r @ MaybeCoercingLookupSR(_, _, _) => rulesToLeaveInDefaultArgument.add(r)
          case r @ ResolveSR(_, _, _, _, _) => rulesToLeaveInDefaultArgument.add(r)
          // Per @DRSINI, this EqualsSR aliases the param rune to the default's resultRune.
          // We KEEP it in the default's rules (rather than hoisting) so the default is fully
          // self-contained — it travels intact when GenericParameterS is inherited (e.g. by
          // struct internal methods). At default-fire time, the typing pass registers the
          // default-only runes via solverState.registerRunes(default.runeToType.keys).
          case r @ EqualsSR(_, _, _) => rulesToLeaveInDefaultArgument.add(r)
          case r @ CallSiteFuncSR(_, _, _, _, _) => ruleBuilder += r // Hoist it up into regular rules
          case r @ DefinitionFuncSR(_, _, _, _, _) => ruleBuilder += r // Hoist it up into regular rules
          case other => vwat(other)
        })

        // Default-only runeToType. resultRune is typed the same as the parent param via
        // the connecting EqualsSR. (For complex defaults that introduce additional
        // default-only runes via MaybeCoercingLookupSR/ResolveSR/etc., those runes' types
        // would also belong here — left as a follow-up.)
        val defaultRuneToType = Map(resultRune.rune -> genericParamTypeS.tyype)
        GenericParameterDefaultS(
          resultRune.rune, rulesToLeaveInDefaultArgument.buildArray().toVector, defaultRuneToType)
      })

//    val (maybeImplicitRegionGenericParam, maybeCoordRegionS) =
//      (typeS, maybeExplicitCoordRegionS) match {
//        case (CoordTemplataType(), Some(explicitCoordRegionS)) => {
//          (None, Some(explicitCoordRegionS))
//        }
//        case (CoordTemplataType(), None) => {
////          val implicitRegionRune = ImplicitRegionRuneS(runeS.rune)
////          val implicitRegionGenericParam =
////            GenericParameterS(
////              genericParamRangeS, RuneUsage(genericParamRangeS, implicitRegionRune), RegionTemplataType(), None, Vector(), None)
//          (None, Some(RuneUsage(genericParamRangeS, implicitRegionRune)))
//
//        }
//        case _ => (None, None)
//      }

    val genericParamS =
      GenericParameterS(genericParamRangeS, runeS, genericParamTypeS, defaultS)
    genericParamS
  }
*/
}
/*
}
*/
pub struct PostParser<'s, 'p, 'ctx> {
  pub global_options: GlobalOptions,
  pub scout_arena: &'ctx ScoutArena<'s>,
  pub keywords: &'ctx Keywords<'s>,
  pub keywords_p: &'ctx Keywords<'p>, // Per @PPSPASTNZ, synthetic parser nodes need 'p-interned keyword strings
  pub parse_arena: &'ctx ParseArena<'p>, // Per @PPSPASTNZ, for allocating synthetic parser AST nodes
}
/*
class PostParser(
    globalOptions: GlobalOptions,
    interner: Interner,
    keywords: Keywords) {
  val templexScout = new TemplexScout(interner, keywords)
  val ruleScout = new RuleScout(interner, keywords, templexScout)
  val functionScout = new FunctionScout(this, interner, keywords, templexScout, ruleScout)
*/

impl<'s, 'p, 'ctx> PostParser<'s, 'p, 'ctx>
{
  // MIGALLOW: new -> new
  pub fn new(
    global_options: GlobalOptions,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    keywords_p: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
  ) -> Self {
    Self {
      global_options,
      scout_arena,
      keywords,
      keywords_p,
      parse_arena,
    }
  }

  pub fn scout_program(
    &self,
    file_coordinate: &'s FileCoordinate<'s>,
    parsed: &FileP<'p>,
  ) -> Result<ProgramS<'s>, ICompileErrorS<'s>>
  {
    let mut structs: Vec<&'s StructS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelStruct(struct_p) = denizen {
        structs.push(&*self.scout_arena.alloc(self.scout_struct(file_coordinate, struct_p)?));
      }
    }

    let mut interfaces: Vec<&'s InterfaceS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelInterface(interface_p) = denizen {
        interfaces.push(&*self.scout_arena.alloc(self.scout_interface(file_coordinate, interface_p)?));
      }
    }

    let mut impls: Vec<&'s ImplS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelImpl(impl_p) = denizen {
        impls.push(&*self.scout_arena.alloc(self.scout_impl(file_coordinate, impl_p)?));
      }
    }

    let mut implemented_functions: Vec<&'s FunctionS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelFunction(function_p) = denizen {
        let (function_s, function_uses) =
          self.scout_function(
            file_coordinate,
            function_p,
            IFunctionParent::FunctionNoParent,
          )?;
        assert!(function_uses.uses.is_empty());
        if let IBodyS::CodeBody(code_body_s) = &function_s.body {
          assert!(
            code_body_s.body.closured_names.is_empty(),
            "POSTPARSER_SCOUT_PROGRAM_TOP_LEVEL_FUNCTION_USES_PARENT_VARS"
          );
        }
        implemented_functions.push(function_s);
      }
    }

    let mut exports: Vec<&'s ExportAsS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelExportAs(export_as_p) = denizen {
        exports.push(&*self.scout_arena.alloc(self.scout_export_as(file_coordinate, export_as_p)));
      }
    }

    let mut imports: Vec<&'s ImportS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelImport(import_p) = denizen {
        imports.push(&*self.scout_arena.alloc(self.scout_import(file_coordinate, import_p)));
      }
    }

    Ok(ProgramS {
      structs: self.scout_arena.alloc_slice_from_vec(structs),
      interfaces: self.scout_arena.alloc_slice_from_vec(interfaces),
      impls: self.scout_arena.alloc_slice_from_vec(impls),
      implemented_functions: self.scout_arena.alloc_slice_from_vec(implemented_functions),
      exports: self.scout_arena.alloc_slice_from_vec(exports),
      imports: self.scout_arena.alloc_slice_from_vec(imports),
    })
  }
/*
  def scoutProgram(fileCoordinate: FileCoordinate, parsed: FileP): Result[ProgramS, ICompileErrorS] = {
    Profiler.frame(() => {
      try {
        val structsS = parsed.denizens.collect({ case TopLevelStructP(s) => s }).map(scoutStruct(fileCoordinate, _));
        val interfacesS = parsed.denizens.collect({ case TopLevelInterfaceP(i) => i }).map(scoutInterface(fileCoordinate, _));
        val implsS = parsed.denizens.collect({ case TopLevelImplP(i) => i }).map(scoutImpl(fileCoordinate, _))
        val functionsS =
          parsed.denizens
            .collect({ case TopLevelFunctionP(f) => f })
            .map(functionP => {
              val (functionS, variableUses) =
                functionScout.scoutFunction(fileCoordinate, functionP, FunctionNoParent())
              vassert(variableUses.uses.isEmpty)
              functionS
            })
        val exportsS = parsed.denizens.collect({ case TopLevelExportAsP(e) => e }).map(scoutExportAs(fileCoordinate, _))
        val importsS = parsed.denizens.collect({ case TopLevelImportP(e) => e }).map(scoutImport(fileCoordinate, _))
        val programS = ProgramS(structsS.toVector, interfacesS.toVector, implsS.toVector, functionsS.toVector, exportsS.toVector, importsS.toVector)
        Ok(programS)
      } catch {
        case CompileErrorExceptionS(err) => Err(err)
      }
    })
  }
*/
fn scout_impl(
  &self,
  file: &'s FileCoordinate<'s>,
  impl0: &ImplP<'p>,
) -> Result<ImplS<'s>, ICompileErrorS<'s>> {
  let range_s = PostParser::eval_range(file, impl0.range);

  match &impl0.interface {
    ITemplexPT::Interpreted(interpreted) => {
      panic!(
        "POSTPARSER_SCOUT_IMPL_CANT_OWNERSHIP_INTERFACE_IN_IMPL_NOT_YET_MIGRATED: {:?}",
        PostParser::eval_range(file, interpreted.range)
      );
    }
    _ => {}
  }

  match &impl0.struct_ {
    Some(ITemplexPT::Interpreted(interpreted)) => {
      panic!(
        "POSTPARSER_SCOUT_IMPL_CANT_OWNERSHIP_STRUCT_IN_IMPL_NOT_YET_MIGRATED: {:?}",
        PostParser::eval_range(file, interpreted.range)
      );
    }
    _ => {}
  }

  let template_rules_p: &[IRulexPR<'p>] = impl0
    .template_rules
    .as_ref()
    .map(|template_rules_p| template_rules_p.rules)
    .unwrap_or(&[]);

  let code_location = range_s.begin.clone();
  let impl_name = ImplDeclarationNameS { code_location };

  let user_specified_identifying_runes = impl0
    .generic_params
    .as_ref()
    .map(|generic_parameters_p| {
      generic_parameters_p
        .params
        .iter()
        .map(|generic_parameter_p| RuneUsage {
          range: PostParser::eval_range(file, generic_parameter_p.name.range()),
          rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str(generic_parameter_p.name.str().as_str()),
          })),
        })
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();

  // Mirrors:
  // RulePUtils.getOrderedRuneDeclarationsFromRulexesWithDuplicates(templateRulesP)
  let runes_from_rules =
    crate::parsing::ast::rules::get_ordered_rune_declarations_from_rulexes_with_duplicates(
      template_rules_p,
    )
    .into_iter()
    .map(|name_p| RuneUsage {
      range: PostParser::eval_range(file, name_p.range()),
      rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.scout_arena.intern_str(name_p.str().as_str()) })),
    })
    .collect::<Vec<_>>();

  let mut user_declared_runes = user_specified_identifying_runes.clone();
  user_declared_runes.extend(runes_from_rules);

  let user_declared_runes_set = user_declared_runes
    .iter()
    .map(|rune_usage| rune_usage.rune.clone())
    .collect::<Vec<_>>();

  let impl_env = IEnvironmentS::Environment(EnvironmentS {
    file,
    parent_env: None,
    name: self.scout_arena.intern_name(INameValS::ImplDeclaration(impl_name.clone())),
    user_declared_runes: user_declared_runes
      .iter()
      .map(|rune_usage| rune_usage.rune.clone())
      .collect(),
  });

  let mut lidb = LocationInDenizenBuilder::new(Vec::new());
  let mut rule_builder = Vec::<IRulexSR<'s>>::new();
  let mut rune_to_explicit_type = Vec::<(IRuneS<'s>, ITemplataType<'s>)>::new();

  let default_region_rune_range_s = RangeS::new(
    range_s.end.clone(),
    range_s.end.clone(),
  );
  let default_region_rune_s = self.scout_arena.intern_rune(IRuneValS::DenizenDefaultRegionRune(
    DenizenDefaultRegionRuneS {
      denizen_name: self.scout_arena.intern_name(INameValS::ImplDeclaration(impl_name.clone())),
    },
  ));
  let maybe_region_generic_param = Some(GenericParameterS {
    range: default_region_rune_range_s.clone(),
    rune: RuneUsage {
      range: default_region_rune_range_s.clone(),
      rune: default_region_rune_s.clone(),
    },
    tyype: IGenericParameterTypeS::RegionGenericParameterType(
      RegionGenericParameterTypeS {
        mutability: IRegionMutabilityS::ReadWriteRegion,
      },
    ),
    default: None,
  });

  let generic_parameters_p = impl0
    .generic_params
    .as_ref()
    .map(|generic_parameters_p| generic_parameters_p.params)
    .unwrap_or(&[]);

  // We'll add the implicit runes to the end, see IRRAE.
  let user_specified_generic_parameters_s: Vec<&'s GenericParameterS<'s>> = generic_parameters_p
    .iter()
    .zip(user_specified_identifying_runes.iter())
    .map(|(g, r)| {
      let mut child_lidb = lidb.child();
      &*self.scout_arena.alloc(self.scout_generic_parameter(
        impl_env.clone(),
        &mut child_lidb,
        &mut rune_to_explicit_type,
        &mut rule_builder,
        default_region_rune_s.clone(),
        g,
        r.clone(),
      ))
    })
    .collect::<Vec<_>>();
  let _user_specified_runes_implicit_region_runes_s =
    maybe_region_generic_param.as_ref().map(|_x| Vec::<GenericParameterS<'s>>::new());

  {
    let mut child_lidb = lidb.child();
    crate::postparsing::rules::rule_scout::translate_rulexes(self.scout_arena,
      self.keywords,
      impl_env.clone(),
      &mut child_lidb,
      &mut rule_builder,
      &mut rune_to_explicit_type,
      default_region_rune_s.clone(),
      template_rules_p,
    );
  }

  let struct_ = match &impl0.struct_ {
    None => {
      return Err(ICompileErrorS::RangedInternalErrorS(RangedInternalErrorS {
        range: range_s.clone(),
        message: "Impl needs struct!".to_string(),
      }));
    }
    Some(x) => x,
  };

  let struct_rune = {
    let mut child_lidb = lidb.child();
    crate::postparsing::rules::templex_scout::translate_maybe_type_into_rune(
      self.scout_arena,
        self.keywords,
      impl_env.clone(),
      &mut child_lidb,
      range_s.clone(),
      &mut rule_builder,
      default_region_rune_s.clone(),
      Some(struct_),
    )
  };

  let interface_rune = {
    let mut child_lidb = lidb.child();
    crate::postparsing::rules::templex_scout::translate_maybe_type_into_rune(
      self.scout_arena,
        self.keywords,
      impl_env.clone(),
      &mut child_lidb,
      range_s.clone(),
      &mut rule_builder,
      default_region_rune_s.clone(),
      Some(&impl0.interface),
    )
  };

  let sub_citizen_imprecise_name = match struct_ {
    ITemplexPT::Call(call)
      if matches!(call.template, ITemplexPT::NameOrRune(_))
        && match call.template {
          ITemplexPT::NameOrRune(name)
            if !user_declared_runes_set.contains(&self.scout_arena.intern_rune(IRuneValS::CodeRune(
              CodeRuneS {
                name: self.scout_arena.intern_str(name.0.str().as_str()),
              },
            ))) =>
          {
            true
          }
          _ => false,
        } =>
    {
      let ITemplexPT::NameOrRune(name) = call.template else {
        panic!("POSTPARSER_SCOUT_IMPL_IMPOSSIBLE_CALL_TEMPLATE_SHAPE");
      };
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.scout_arena.intern_str(name.0.str().as_str()),
      }))
    }
    ITemplexPT::NameOrRune(name)
      if !user_declared_runes_set.contains(&self.scout_arena.intern_rune(IRuneValS::CodeRune(
        CodeRuneS {
          name: self.scout_arena.intern_str(name.0.str().as_str()),
        },
      ))) =>
    {
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.scout_arena.intern_str(name.0.str().as_str()),
      }))
    }
    _ => {
      return Err(ICompileErrorS::RangedInternalErrorS(RangedInternalErrorS {
        range: PostParser::eval_range(file, struct_.range()),
        message: "Can't determine name of struct!".to_string(),
      }));
    }
  };

  let super_interface_imprecise_name = match &impl0.interface {
    ITemplexPT::Call(call)
      if matches!(call.template, ITemplexPT::NameOrRune(_))
        && match call.template {
          ITemplexPT::NameOrRune(name)
            if !user_declared_runes_set.contains(&self.scout_arena.intern_rune(IRuneValS::CodeRune(
              CodeRuneS {
                name: self.scout_arena.intern_str(name.0.str().as_str()),
              },
            ))) =>
          {
            true
          }
          _ => false,
        } =>
    {
      let ITemplexPT::NameOrRune(name) = call.template else {
        panic!("POSTPARSER_SCOUT_IMPL_IMPOSSIBLE_CALL_TEMPLATE_SHAPE");
      };
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.scout_arena.intern_str(name.0.str().as_str()),
      }))
    }
    ITemplexPT::NameOrRune(name)
      if !user_declared_runes_set.contains(&self.scout_arena.intern_rune(IRuneValS::CodeRune(
        CodeRuneS {
          name: self.scout_arena.intern_str(name.0.str().as_str()),
        },
      ))) =>
    {
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.scout_arena.intern_str(name.0.str().as_str()),
      }))
    }
    _ => {
      return Err(ICompileErrorS::RangedInternalErrorS(RangedInternalErrorS {
        // Intentionally mirrors Scala's `struct.range` here.
        range: PostParser::eval_range(file, struct_.range()),
        message: "Can't determine name of struct!".to_string(),
      }));
    }
  };

  let generic_parameters_s = user_specified_generic_parameters_s;
  // ++ userSpecifiedRunesImplicitRegionRunesS
  let _maybe_region_generic_param = maybe_region_generic_param;

  let param_types_vec: Vec<ITemplataType<'s>> = generic_parameters_s
      .iter()
      .map(|generic_parameter_s| generic_parameter_s.tyype.tyype())
      .collect();
  let tyype = ITemplataType::TemplateTemplataType(TemplateTemplataType {
    param_types: self.scout_arena.alloc_slice_copy(&param_types_vec),
    return_type: self.scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
  });

  Ok(ImplS {
    range: range_s,
    name: impl_name,
    user_specified_identifying_runes: self.scout_arena.alloc_slice_from_vec(generic_parameters_s),
    rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
    rune_to_explicit_type: self.scout_arena.alloc_index_map_from_iter(rune_to_explicit_type.into_iter()),
    tyype,
    struct_kind_rune: struct_rune,
    sub_citizen_imprecise_name,
    interface_kind_rune: interface_rune,
    super_interface_imprecise_name,
  })
}
/*
  private def scoutImpl(file: FileCoordinate, impl0: ImplP): ImplS = {
    val ImplP(rangeP, maybeGenericParametersP, maybeTemplateRulesP, maybeStruct, interface, attributes) = impl0
    val rangeS = PostParser.evalRange(file, rangeP)

    interface match {
      case InterpretedPT(range, _, _, _) => {
        throw CompileErrorExceptionS(CantOwnershipInterfaceInImpl(PostParser.evalRange(file, range)))
      }
      case _ =>
    }

    maybeStruct match {
      case Some(InterpretedPT(range, _, _, _)) => {
        throw CompileErrorExceptionS(CantOwnershipStructInImpl(PostParser.evalRange(file, range)))
      }
      case _ =>
    }

    val templateRulesP = maybeTemplateRulesP.toVector.flatMap(_.rules)

    val codeLocation = rangeS.begin
    val implName = interner.intern(ImplDeclarationNameS(codeLocation))

    val userSpecifiedIdentifyingRunes =
      maybeGenericParametersP
        .toVector.flatMap(_.params)
        .map(_.name)
        .map({ case NameP(range, identifyingRuneName) => rules.RuneUsage(PostParser.evalRange(file, range), CodeRuneS(identifyingRuneName)) })
    val runesFromRules =
      RulePUtils.getOrderedRuneDeclarationsFromRulexesWithDuplicates(templateRulesP)
        .map({ case NameP(range, identifyingRuneName) => rules.RuneUsage(PostParser.evalRange(file, range), CodeRuneS(identifyingRuneName)) })
    val userDeclaredRunes = userSpecifiedIdentifyingRunes ++ runesFromRules
    val userDeclaredRunesSet = userDeclaredRunes.map(_.rune).toSet

    val implEnv = postparsing.EnvironmentS(file, None, implName, userDeclaredRunes.map(_.rune).toSet)

    val lidb = new LocationInDenizenBuilder(Vector())
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    val runeToExplicitType = mutable.ArrayBuffer[(IRuneS, ITemplataType)]()

    val (defaultRegionRuneRangeS, defaultRegionRuneS, maybeRegionGenericParam) =
    {
      val regionRange = RangeS(rangeS.end, rangeS.end)
      val rune = DenizenDefaultRegionRuneS(implName)
      vregionmut() // Put back in when we have regions
      // runeToExplicitType += ((rune, RegionTemplataType()))
      val implicitRegionGenericParam =
        GenericParameterS(regionRange, RuneUsage(regionRange, rune), RegionGenericParameterTypeS(ReadWriteRegionS), None)
      (regionRange, rune, Some(implicitRegionGenericParam))
    }

    val genericParametersP =
      maybeGenericParametersP
        .toVector
        .flatMap(_.params)

    // We'll add the implicit runes to the end, see IRRAE.
    val userSpecifiedGenericParametersS =
      genericParametersP.zip(userSpecifiedIdentifyingRunes)
        .map({ case (g, r) =>
          PostParser.scoutGenericParameter(
            templexScout, implEnv, lidb.child(), runeToExplicitType, ruleBuilder, defaultRegionRuneS, g, r)
        })
//    val userSpecifiedRunesImplicitRegionRunesS = userSpecifiedRunesImplicitRegionRunesUnflattenedS.flatten

    ruleScout.translateRulexes(implEnv, lidb.child(), ruleBuilder, runeToExplicitType, defaultRegionRuneS, templateRulesP)

    val struct =
      maybeStruct match {
        case None => throw CompileErrorExceptionS(postparsing.RangedInternalErrorS(rangeS, "Impl needs struct!"))
        case Some(x) => x
      }

    val structRune =
      templexScout.translateMaybeTypeIntoRune(
        implEnv,
        lidb.child(),
        rangeS,
        ruleBuilder,
        defaultRegionRuneS,
        Some(struct))

    val interfaceRune =
      templexScout.translateMaybeTypeIntoRune(
        implEnv,
        lidb.child(),
        rangeS,
        ruleBuilder,
        defaultRegionRuneS,
        Some(interface))

    val subCitizenImpreciseName =
      struct match {
        case CallPT(_, NameOrRunePT(name), _) if !userDeclaredRunesSet.contains(CodeRuneS(name.str)) => interner.intern(CodeNameS(name.str))
        case NameOrRunePT(name) if !userDeclaredRunesSet.contains(CodeRuneS(name.str)) => interner.intern(CodeNameS(name.str))
        case _ => throw CompileErrorExceptionS(RangedInternalErrorS(PostParser.evalRange(file, struct.range), "Can't determine name of struct!"))
      }

    val superInterfaceImpreciseName =
      interface match {
        case CallPT(_, NameOrRunePT(name), _) if !userDeclaredRunesSet.contains(CodeRuneS(name.str)) => interner.intern(CodeNameS(name.str))
        case NameOrRunePT(name) if !userDeclaredRunesSet.contains(CodeRuneS(name.str)) => interner.intern(CodeNameS(name.str))
        case _ => throw CompileErrorExceptionS(RangedInternalErrorS(PostParser.evalRange(file, struct.range), "Can't determine name of struct!"))
      }

    val genericParametersS =
      userSpecifiedGenericParametersS
        //++ userSpecifiedRunesImplicitRegionRunesS

    val tyype = TemplateTemplataType(genericParametersS.map(_.tyype.tyype), KindTemplataType())

    ImplS(
      rangeS,
      implName,
      genericParametersS,
      ruleBuilder.toVector,
      runeToExplicitType.toMap,
      tyype,
      structRune,
      subCitizenImpreciseName,
      interfaceRune,
      superInterfaceImpreciseName)
  }
*/
fn scout_export_as(
  &self,
  file: &'s FileCoordinate<'s>,
  export_as_p: &ExportAsP<'p>,
) -> ExportAsS<'s> {
  let range_s = Self::eval_range(file, export_as_p.range);
  let pos = range_s.begin.clone();
  let export_name = self.scout_arena.intern_name(INameValS::ExportAsName(ExportAsNameS { code_location: pos }));
  let export_env = IEnvironmentS::Environment(EnvironmentS {
    file,
    parent_env: None,
    name: export_name,
    user_declared_runes: Default::default(),
  });
  let mut lidb = LocationInDenizenBuilder::new(Vec::new());
  let mut rule_builder = Vec::<IRulexSR<'s>>::new();
  let mut rune_to_explicit_type = Vec::<(IRuneS<'s>, ITemplataType<'s>)>::new();
  let region_range = RangeS { begin: range_s.end.clone(), end: range_s.end.clone() };
  let default_region_rune_s = self.scout_arena.intern_rune(IRuneValS::DenizenDefaultRegionRune(
    DenizenDefaultRegionRuneS { denizen_name: export_name },
  ));
  rune_to_explicit_type.push((default_region_rune_s.clone(), ITemplataType::RegionTemplataType(RegionTemplataType {})));
  let _region_generic_param = GenericParameterS {
    range: region_range.clone(),
    rune: RuneUsage { range: region_range, rune: default_region_rune_s.clone() },
    tyype: IGenericParameterTypeS::RegionGenericParameterType(RegionGenericParameterTypeS { mutability: IRegionMutabilityS::ReadWriteRegion }),
    default: None,
  };
  let rune_s = translate_templex(
    self.scout_arena,
    self.keywords,
    export_env,
    &mut lidb,
    &mut rule_builder,
    default_region_rune_s,
    &export_as_p.struct_,
  );
  ExportAsS {
    range: range_s,
    rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
    export_name: ExportAsNameS { code_location: pos },
    rune: rune_s,
    exported_name: self.scout_arena.intern_str(export_as_p.exported_name.str().as_str()),
  }
}
/*
  private def scoutExportAs(file: FileCoordinate, exportAsP: ExportAsP): ExportAsS = {
    val ExportAsP(rangeP, templexP, exportedName) = exportAsP

    val rangeS = PostParser.evalRange(file, rangeP)
    val pos = rangeS.begin
    val exportName = interner.intern(ExportAsNameS(pos))
    val exportEnv = EnvironmentS(file, None, exportName, Set())

    val lidb = new LocationInDenizenBuilder(Vector())
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    val runeToExplicitType = mutable.ArrayBuffer[(IRuneS, ITemplataType)]()

    val (defaultRegionRuneRangeS, defaultRegionRuneS, regionGenericParam) =
    {
      val regionRange = RangeS(rangeS.end, rangeS.end)
      val rune = DenizenDefaultRegionRuneS(exportName)
      runeToExplicitType += ((rune, RegionTemplataType()))
      val implicitRegionGenericParam =
        GenericParameterS(regionRange, RuneUsage(regionRange, rune), RegionGenericParameterTypeS(ReadWriteRegionS), None)
      (regionRange, rune, implicitRegionGenericParam)
    }

    val runeS = templexScout.translateTemplex(exportEnv, lidb, ruleBuilder, defaultRegionRuneS, templexP)

    postparsing.ExportAsS(rangeS, ruleBuilder.toVector, exportName, runeS, exportedName.str)
  }
*/
fn scout_import(
  &self,
  file: &'s FileCoordinate<'s>,
  import_p: &ImportP<'p>,
) -> ImportS<'s> {
  let _pos = PostParser::eval_pos(file, import_p.range.begin());

  ImportS {
    range: PostParser::eval_range(file, import_p.range),
    module_name: self.scout_arena.intern_str(import_p.module_name.str().as_str()),
    package_names: self.scout_arena.alloc_slice_from_vec(
      import_p.package_steps.iter().map(|n| self.scout_arena.intern_str(n.str().as_str())).collect(),
    ),
    importee_name: self.scout_arena.intern_str(import_p.importee_name.str().as_str()),
  }
}
/*
  private def scoutImport(file: FileCoordinate, importP: ImportP): ImportS = {
    val ImportP(range, moduleName, packageNames, importeeName) = importP

    val pos = PostParser.evalPos(file, range.begin)

    postparsing.ImportS(PostParser.evalRange(file, range), moduleName.str, packageNames.map(_.str), importeeName.str)
  }
*/
fn predict_mutability(
  _range_s: RangeS<'s>,
  mutability_rune_s: IRuneS<'s>,
  rules_s: &[IRulexSR<'s>],
) -> Option<MutabilityP> {
  let predicted_mutabilities = rules_s
    .iter()
    .filter_map(|rule| match rule {
      IRulexSR::Literal(literal_rule)
        if literal_rule.rune.rune == mutability_rune_s
          && matches!(literal_rule.literal, ILiteralSL::MutabilityLiteral(_)) =>
      {
        let ILiteralSL::MutabilityLiteral(ref mutability_literal) = literal_rule.literal else {
          unreachable!()
        };
        Some(mutability_literal.mutability)
      }
      _ => None,
    })
    .collect::<Vec<_>>();

  match predicted_mutabilities.len() {
    0 => None,
    1 => predicted_mutabilities.first().copied(),
    _ => panic!("POSTPARSER_PREDICT_MUTABILITY_TOO_MANY_MUTABILITIES"),
  }
}
/*
  private def predictMutability(rangeS: RangeS, mutabilityRuneS: IRuneS, rulesS: Vector[IRulexSR]):
  Option[MutabilityP] = {
    val predictedMutabilities =
      rulesS.collect({
        case LiteralSR(_, runeS, MutabilityLiteralSL(mutability)) if runeS.rune == mutabilityRuneS => mutability
      })
    val predictedMutability =
      predictedMutabilities.size match {
        case 0 => None
        case 1 => Some(predictedMutabilities.head)
        case _ => throw CompileErrorExceptionS(RangedInternalErrorS(rangeS, "Too many mutabilities: " + predictedMutabilities.mkString("[", ", ", "]")))
      }
    predictedMutability
  }
*/
  fn scout_struct(
    &self,
    file: &'s FileCoordinate<'s>,
    head: &StructP<'p>,
  ) -> Result<StructS<'s>, ICompileErrorS<'s>> {
    let struct_range_s = Self::eval_range(file, head.range);
    let struct_name = self.scout_arena.intern_struct_declaration_name(TopLevelStructDeclarationNameS {
      name: self.scout_arena.intern_str(head.name.str().as_str()),
      range: Self::eval_range(file, head.name.range()),
    });
    let body_range_s = Self::eval_range(file, head.body_range);
    let mut lidb = LocationInDenizenBuilder::new(Vec::new());

    let generic_parameters_p: &[GenericParameterP<'p>] = head
      .identifying_runes
      .as_ref()
      .map(|x| x.params as &[GenericParameterP<'p>])
      .unwrap_or(&[]);
    let user_specified_identifying_runes = generic_parameters_p
      .iter()
      .map(|generic_parameter| RuneUsage {
        range: Self::eval_range(file, generic_parameter.name.range()),
        rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(generic_parameter.name.str().as_str()),
        })),
      })
      .collect::<Vec<_>>();
    let template_rules_p: &[IRulexPR<'p>] = head
      .template_rules
      .as_ref()
      .map(|x| x.rules as &[IRulexPR<'p>])
      .unwrap_or(&[]);
    let runes_from_rules =
      get_ordered_rune_declarations_from_rulexes_with_duplicates(&template_rules_p)
      .iter()
      .map(|name_p| RuneUsage {
        range: Self::eval_range(file, name_p.range()),
        rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(name_p.str().as_str()),
        })),
      })
      .collect::<Vec<_>>();
    let user_declared_runes = user_specified_identifying_runes
      .iter()
      .chain(runes_from_rules.iter())
      .cloned()
      .collect::<Vec<_>>();
    let struct_env = IEnvironmentS::Environment(EnvironmentS {
      file,
      parent_env: None,
      name: self.scout_arena.intern_name(INameValS::TopLevelStructDeclaration(struct_name.clone())),
      user_declared_runes: user_declared_runes
        .iter()
        .map(|x| x.rune.clone())
        .collect(),
    });

    let mut header_rule_builder = Vec::<IRulexSR<'s>>::new();
    let mut header_rune_to_explicit_type = Vec::<(IRuneS<'s>, ITemplataType<'s>)>::new();

    let (_default_region_rune_range_s, default_region_rune_s, _maybe_region_generic_param) =
      match &head.maybe_default_region_rune {
        None => {
          let region_range = RangeS::new(
            body_range_s.begin.clone(),
            body_range_s.begin.clone(),
          );
          let rune = self
            .scout_arena
            .intern_rune(IRuneValS::DenizenDefaultRegionRune(
              DenizenDefaultRegionRuneS {
              denizen_name: self.scout_arena.intern_name(INameValS::TopLevelStructDeclaration(
                struct_name.clone(),
              )),
            }));
          // Put back in when we have regions
          // header_rune_to_explicit_type.push((rune.clone(), ITemplataType::RegionTemplataType(RegionTemplataType {})));
          let implicit_region_generic_param = GenericParameterS {
            range: region_range.clone(),
            rune: RuneUsage {
              range: region_range.clone(),
              rune: rune.clone(),
            },
            tyype: IGenericParameterTypeS::RegionGenericParameterType(
              RegionGenericParameterTypeS {
                mutability: IRegionMutabilityS::ReadWriteRegion,
              },
            ),
            default: None,
          };
          (region_range, rune, Some(implicit_region_generic_param))
        }
        Some(region_rune_p) => {
          let region_range_s = Self::eval_range(file, region_rune_p.range);
          let region_name = match &region_rune_p.name {
            None => panic!("impl isolates"),
            Some(name) => name.str(),
          };
          let rune = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str(region_name.as_str()),
          }));
          if !struct_env.all_declared_runes().contains(&rune) {
            return Err(ICompileErrorS::CouldntFindRuneS(CouldntFindRuneS {
              range: region_range_s.clone(),
              name: region_name.as_str().to_string(),
            }));
          }
          (region_range_s, rune, None)
        }
      };

    let struct_user_specified_generic_parameters_s: Vec<&'s GenericParameterS<'s>> = generic_parameters_p
      .iter()
      .zip(user_specified_identifying_runes.iter())
      .map(|(g, r)| {
        &*self.scout_arena.alloc(self.scout_generic_parameter(
          struct_env.clone(),
          &mut lidb.child(),
          &mut header_rune_to_explicit_type,
          &mut header_rule_builder,
          default_region_rune_s.clone(),
          g,
          r.clone(),
        ))
      })
      .collect::<Vec<_>>();
    // Put back in when we have regions
    // let generic_parameters_s = struct_user_specified_generic_parameters_s ++ maybe_region_generic_param ++ user_specified_runes_implicit_region_runes_s;
    let generic_parameters_s = struct_user_specified_generic_parameters_s;

    translate_rulexes(self.scout_arena,
      self.keywords,
      struct_env.clone(),
      &mut lidb.child(),
      &mut header_rule_builder,
      &mut header_rune_to_explicit_type,
      default_region_rune_s.clone(),
      &template_rules_p,
    );

    let mut member_rule_builder = Vec::<IRulexSR<'s>>::new();
    let mut members_rune_to_explicit_type = self.scout_arena.alloc_index_map::<IRuneS, ITemplataType<'s>>();

    let default_mutability = ITemplexPT::Mutability(
      MutabilityPT(
        RangeL(head.body_range.begin(), head.body_range.begin()),
        MutabilityP::Mutable,
      ),
    );
    let mutability: &ITemplexPT<'p> = head.mutability.as_ref().unwrap_or(&default_mutability);
    let mutability_rune_s = translate_templex(
      self.scout_arena,
      self.keywords,
      struct_env.clone(),
      &mut lidb.child(),
      &mut header_rule_builder,
      default_region_rune_s.clone(),
      mutability,
    );
    header_rune_to_explicit_type.push((
      mutability_rune_s.rune.clone(),
      ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
    ));

    let mut internal_methods_p = Vec::<&'p crate::parsing::ast::FunctionP<'p>>::new();
    let members_s = head
      .members
      .contents
      .iter()
      .flat_map(|member| match member {
        IStructContent::NormalStructMember(member) => {
          let member_rune = translate_templex(
            self.scout_arena,
            self.keywords,
            struct_env.clone(),
            &mut lidb.child(),
            &mut member_rule_builder,
            default_region_rune_s.clone(),
            &member.tyype,
          );
          members_rune_to_explicit_type.insert(
            member_rune.rune.clone(),
            ITemplataType::CoordTemplataType(CoordTemplataType {}),
          );
          vec![IStructMemberS::NormalStructMember(NormalStructMemberS {
            range: Self::eval_range(file, member.range),
            name: self.scout_arena.intern_str(member.name.str().as_str()),
            variability: member.variability,
            type_rune: member_rune,
          })]
        }
        IStructContent::VariadicStructMember(member) => {
          let member_rune = translate_templex(
            self.scout_arena,
            self.keywords,
            struct_env.clone(),
            &mut lidb.child(),
            &mut member_rule_builder,
            default_region_rune_s.clone(),
            &member.tyype,
          );
          members_rune_to_explicit_type.insert(
            member_rune.rune.clone(),
            ITemplataType::PackTemplataType(PackTemplataType {
              element_type: &*self.scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {})),
            }),
          );
          vec![IStructMemberS::VariadicStructMember(VariadicStructMemberS {
            range: Self::eval_range(file, member.range),
            variability: member.variability,
            type_rune: member_rune,
          })]
        }
        IStructContent::StructMethod(func_p) => {
          internal_methods_p.push(func_p);
          vec![]
        }
      })
      .collect::<Vec<_>>();

    let header_rules_s = header_rule_builder;
    let member_rules_s = member_rule_builder;
    let all_rules_s = header_rules_s
      .iter()
      .chain(member_rules_s.iter())
      .cloned()
      .collect::<Vec<_>>();
    let mut all_rune_to_explicit_type = header_rune_to_explicit_type.clone();
    all_rune_to_explicit_type.extend(
      members_rune_to_explicit_type
        .iter()
        .map(|(rune, tyype)| (rune.clone(), tyype.clone())),
    );

    let identifying_runes_s = user_specified_identifying_runes
      .iter()
      .map(|x| x.rune.clone())
      .collect::<Vec<_>>();
    let rune_to_predicted_type = Self::predict_rune_types(
      self.scout_arena,
      struct_range_s.clone(),
      &identifying_runes_s,
      &mut all_rune_to_explicit_type,
      &all_rules_s,
    )?;
    let predicted_mutability =
      Self::predict_mutability(struct_range_s.clone(), mutability_rune_s.rune.clone(), &all_rules_s);
    let runes_from_header = user_declared_runes
      .iter()
      .map(|x| x.rune.clone())
      .chain(header_rules_s.iter().flat_map(|rule| {
        rule
          .rune_usages()
          .into_iter()
          .map(|usage| usage.rune.clone())
      }))
      .collect::<std::collections::HashSet<_>>();
    let header_rune_to_predicted_type = self.scout_arena.alloc_index_map_from_iter(
      rune_to_predicted_type
        .iter()
        .filter(|(rune, _)| runes_from_header.contains(*rune))
        .map(|(rune, tyype)| (rune.clone(), tyype.clone())),
    );
    let members_rune_to_predicted_type = self.scout_arena.alloc_index_map_from_iter(
      rune_to_predicted_type
        .iter()
        .filter(|(rune, _)| !runes_from_header.contains(*rune))
        .map(|(rune, tyype)| (rune.clone(), tyype.clone())),
    );

    let param_types_vec: Vec<ITemplataType<'s>> = generic_parameters_s
        .iter()
        .map(|x| x.tyype.tyype())
        .collect();
    let tyype = TemplateTemplataType {
      param_types: self.scout_arena.alloc_slice_copy(&param_types_vec),
      return_type: self.scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
    };
    let weakable = head
      .attributes
      .iter()
      .any(|attr| matches!(attr, IAttributeP::WeakableAttribute(_)));
    let attrs_without_linear_s = Self::translate_citizen_attributes(
      self.scout_arena,
      file,
      self.scout_arena.intern_name(INameValS::TopLevelStructDeclaration(struct_name.clone())),
      &head
        .attributes
        .iter()
        .filter(|attr| {
          !matches!(
            attr,
            IAttributeP::WeakableAttribute(_) | IAttributeP::LinearAttribute(_)
          )
        })
        .cloned()
        .collect::<Vec<_>>(),
    );
    let mut attrs_s = attrs_without_linear_s;
    if let Some(IAttributeP::LinearAttribute(attr)) = head
      .attributes
      .iter()
      .find(|attr| matches!(attr, IAttributeP::LinearAttribute(_)))
    {
      attrs_s.push(ICitizenAttributeS::MacroCall(MacroCallS {
        range: Self::eval_range(file, attr.range),
        include: IMacroInclusionP::DontCallMacro,
        macro_name: self.keywords.derive_struct_drop,
      }));
    }

    let generic_parameters_s_arena: &'s [&'s GenericParameterS<'s>] = self.scout_arena.alloc_slice_from_vec(generic_parameters_s.clone());
    let internal_methods_s_vec: Vec<&'s FunctionS<'s>> = internal_methods_p.iter().map(|method| -> Result<&'s FunctionS<'s>, ICompileErrorS<'s>> {
      self.scout_interface_member(
        crate::postparsing::function_scout::ParentCitizen {
          citizen_is_interface: false,
          citizen_env: struct_env.clone(),
          citizen_generic_params: generic_parameters_s_arena,
          citizen_rules: all_rules_s.clone(),
          citizen_rune_to_explicit_type: all_rune_to_explicit_type.iter().map(|(r, t)| (r.clone(), t.clone())).collect(),
        },
        method,
      )
    }).collect::<Result<Vec<_>, _>>()?;

    Ok(StructS::new(
      struct_range_s,
      struct_name,
      self.scout_arena.alloc_slice_from_vec(attrs_s),
      weakable,
      self.scout_arena.alloc_slice_from_vec(generic_parameters_s),
      mutability_rune_s,
      predicted_mutability,
      tyype,
      self.scout_arena.alloc_index_map_from_iter(header_rune_to_explicit_type.into_iter()),
      header_rune_to_predicted_type,
      self.scout_arena.alloc_slice_from_vec(header_rules_s),
      members_rune_to_explicit_type,
      members_rune_to_predicted_type,
      self.scout_arena.alloc_slice_from_vec(member_rules_s),
      self.scout_arena.alloc_slice_from_vec(members_s),
      self.scout_arena.alloc_slice_from_vec(internal_methods_s_vec),
    ))
  }
/*
Guardian: disable: TUCMPX
  private def scoutStruct(file: FileCoordinate, head: StructP): StructS = {
    val StructP(rangeP, NameP(structNameRange, structHumanName), attributesP, mutabilityPT, maybeGenericParametersP, maybeTemplateRulesP, maybeDefaultRegionRuneP, bodyRangeP, StructMembersP(_, members)) = head

    val structRangeS = PostParser.evalRange(file, rangeP)
    val structName = interner.intern(postparsing.TopLevelStructDeclarationNameS(structHumanName, PostParser.evalRange(file, structNameRange)))
    val bodyRangeS = PostParser.evalRange(file, bodyRangeP)

    val lidb = new LocationInDenizenBuilder(Vector())

    val genericParametersP =
      maybeGenericParametersP
        .toVector
        .flatMap(_.params)

    val userSpecifiedIdentifyingRunes =
      genericParametersP
        .map({ case GenericParameterP(_, NameP(range, identifyingRuneName), _, _, _, _) =>
          rules.RuneUsage(PostParser.evalRange(file, range), CodeRuneS(identifyingRuneName))
        })

    val templateRulesP = maybeTemplateRulesP.toVector.flatMap(_.rules)

    val runesFromRules =
      RulePUtils.getOrderedRuneDeclarationsFromRulexesWithDuplicates(templateRulesP)
        .map({ case NameP(range, identifyingRuneName) => rules.RuneUsage(PostParser.evalRange(file, range), CodeRuneS(identifyingRuneName)) })
    val userDeclaredRunes = userSpecifiedIdentifyingRunes ++ runesFromRules
    val structEnv = postparsing.EnvironmentS(file, None, structName, userDeclaredRunes.map(_.rune).toSet)

    val headerRuleBuilder = ArrayBuffer[IRulexSR]()
    val headerRuneToExplicitType = mutable.ArrayBuffer[(IRuneS, ITemplataType)]()

    val (defaultRegionRuneRangeS, defaultRegionRuneS, maybeRegionGenericParam) =
      maybeDefaultRegionRuneP match {
        case None => {
          val regionRange = RangeS(bodyRangeS.begin, bodyRangeS.begin)
          val rune = DenizenDefaultRegionRuneS(structName)
          vregionmut() // Put back in when we have regions
          // headerRuneToExplicitType += ((rune, RegionTemplataType()))
          val implicitRegionGenericParam =
            GenericParameterS(regionRange, RuneUsage(regionRange, rune), RegionGenericParameterTypeS(ReadWriteRegionS), None)
          (regionRange, rune, Some(implicitRegionGenericParam))
        }
        case Some(RegionRunePT(regionRangeP, regionName)) => {
          val regionRangeS = evalRange(file, regionRangeP)
          val rune = CodeRuneS(vassertSome(regionName).str) // impl isolates
          if (!structEnv.allDeclaredRunes().contains(rune)) {
            throw CompileErrorExceptionS(CouldntFindRuneS(regionRangeS, rune.name.str))
          }
          (regionRangeS, rune, None)
        }
      }


    val structUserSpecifiedGenericParametersS =
      genericParametersP.zip(userSpecifiedIdentifyingRunes)
        .map({ case (g, r) =>
          PostParser.scoutGenericParameter(
            templexScout, structEnv, lidb.child(), headerRuneToExplicitType, headerRuleBuilder, defaultRegionRuneS, g, r)
        })
//    val userSpecifiedRunesImplicitRegionRunesS = userSpecifiedRunesImplicitRegionRunesUnflattenedS.flatten

    val genericParametersS =
      structUserSpecifiedGenericParametersS
        vregionmut() // Put back in when we have regions
        //++ maybeRegionGenericParam
        //++ userSpecifiedRunesImplicitRegionRunesS

    ruleScout.translateRulexes(structEnv, lidb.child(), headerRuleBuilder, headerRuneToExplicitType, defaultRegionRuneS, templateRulesP)

    val memberRuleBuilder = ArrayBuffer[IRulexSR]()
    val membersRuneToExplicitType = mutable.HashMap[IRuneS, ITemplataType]()

    val mutability =
      mutabilityPT.getOrElse(MutabilityPT(RangeL(bodyRangeP.begin, bodyRangeP.begin), MutableP))
    val mutabilityRuneS =
      templexScout.translateTemplex(
        structEnv, lidb.child(), headerRuleBuilder, defaultRegionRuneS, mutability)
    headerRuneToExplicitType += ((mutabilityRuneS.rune, MutabilityTemplataType()))

    val internalMethodsP = mutable.ArrayBuffer[FunctionP]()
    val membersS =
      members.flatMap({
        case NormalStructMemberP(range, name, variability, memberType) => {
          val memberRune =
            templexScout.translateTemplex(
              structEnv, lidb.child(), memberRuleBuilder, defaultRegionRuneS, memberType)
          membersRuneToExplicitType.put(memberRune.rune, CoordTemplataType())
          Vector(NormalStructMemberS(PostParser.evalRange(structEnv.file, range), name.str, variability, memberRune))
        }
        case VariadicStructMemberP(range, variability, memberType) => {
          val memberRune =
            templexScout.translateTemplex(
              structEnv, lidb.child(), memberRuleBuilder, defaultRegionRuneS, memberType)
          membersRuneToExplicitType.put(memberRune.rune, PackTemplataType(CoordTemplataType()))
          Vector(VariadicStructMemberS(PostParser.evalRange(structEnv.file, range), variability, memberRune))
        }
        case StructMethodP(funcP) => {
          internalMethodsP += funcP
          Vector.empty
        }
      })

    val headerRulesS = headerRuleBuilder.toVector
    val memberRulesS = memberRuleBuilder.toVector

    val allRulesS = headerRulesS ++ memberRulesS
    val allRuneToExplicitType = headerRuneToExplicitType ++ membersRuneToExplicitType

    val runeToPredictedType = predictRuneTypes(structRangeS, userSpecifiedIdentifyingRunes.map(_.rune), allRuneToExplicitType, allRulesS)

    val predictedMutability = predictMutability(structRangeS, mutabilityRuneS.rune, allRulesS)

    val runesFromHeader = (userDeclaredRunes.map(_.rune) ++ headerRulesS.flatMap(_.runeUsages.map(_.rune))).toSet
    val headerRuneToPredictedType = runeToPredictedType.filter(x => runesFromHeader.contains(x._1))
    val membersRuneToPredictedType = runeToPredictedType.filter(x => !runesFromHeader.contains(x._1))

    val tyype = TemplateTemplataType(genericParametersS.map(_.tyype.tyype), KindTemplataType())

    val weakable = attributesP.exists({ case w @ WeakableAttributeP(_) => true case _ => false })
    val attrsWithoutLinearS = translateCitizenAttributes(file, structName, attributesP.filter({ case WeakableAttributeP(_) => false case LinearAttributeP(_) => false case _ => true}))
    val attrsS =
      attrsWithoutLinearS ++
          (attributesP.collectFirst({ case w@LinearAttributeP(_) => w }) match {
            case None => None
            case Some(LinearAttributeP(range)) => {
              Some(MacroCallS(evalRange(file, range), DontCallMacroP, keywords.DeriveStructDrop))
            }
          })

//    val runeSToCanonicalRune = ruleBuilder.runeSToTentativeRune.mapValues(tentativeRune => tentativeRuneToCanonicalRune(tentativeRune))

    val internalMethodsS =
      internalMethodsP.toVector.map(method => {
        functionScout.scoutInterfaceMember(
          ParentCitizen(
            false,  // not an interface — struct internal methods don't require virtual self
            structEnv,
            genericParametersS.toVector,
            allRulesS,
            allRuneToExplicitType.toMap),
          method)
      })

    StructS(
      structRangeS,
      structName,
      attrsS,
      weakable,
      genericParametersS,
      mutabilityRuneS,
      predictedMutability,
      tyype,
      headerRuneToExplicitType.toMap,
      headerRuneToPredictedType,
      headerRulesS,
      membersRuneToExplicitType.toMap,
      membersRuneToPredictedType,
      memberRulesS,
      membersS,
      internalMethodsS)
  }
*/
fn translate_citizen_attributes(
  interner: &ScoutArena<'s>,
  file: &'s FileCoordinate<'s>,
  _denizen_name: INameS<'s>,
  attrs_p: &[IAttributeP<'p>],
) -> Vec<ICitizenAttributeS<'s>> {
  attrs_p
    .iter()
    .map(|attr_p| match attr_p {
      IAttributeP::ExportAttribute(_) => {
        ICitizenAttributeS::Export(ExportS {
          package_coordinate: file.package_coord,
        })
      }
      IAttributeP::ExternAttribute(_) => {
        ICitizenAttributeS::Extern(crate::postparsing::ast::ExternS {
          package_coord: file.package_coord,
        })
      }
      IAttributeP::SealedAttribute(_) => {
        ICitizenAttributeS::Sealed(SealedS)
      }
      IAttributeP::MacroCall(macro_call_p) => {
        ICitizenAttributeS::MacroCall(MacroCallS {
          range: PostParser::eval_range(file, macro_call_p.range),
          include: macro_call_p.inclusion,
          macro_name: interner.intern_str(macro_call_p.name.str().as_str()),
        })
      }
      _ => panic!("POSTPARSER_TRANSLATE_CITIZEN_ATTRIBUTES_NOT_YET_IMPLEMENTED"),
    })
    .collect()
}
/*
  def translateCitizenAttributes(file: FileCoordinate, denizenName: INameS, attrsP: Vector[IAttributeP]): Vector[ICitizenAttributeS] = {
    attrsP.map({
      case ExportAttributeP(_) => ExportS(file.packageCoordinate)
      case ExternAttributeP(_) => ExternS(file.packageCoordinate)
      case SealedAttributeP(_) => SealedS
      case MacroCallP(range, dontCall, NameP(_, str)) => MacroCallS(PostParser.evalRange(file, range), dontCall, str)
      case x => vimpl(x.toString)
    })
  }
*/
pub(crate) fn predict_rune_types(
  scout_arena: &ScoutArena<'s>,
  range_s: RangeS<'s>,
  _identifying_runes_s: &[IRuneS<'s>],
  rune_to_explicit_type: &mut Vec<(IRuneS<'s>, ITemplataType<'s>)>,
  _rules_s: &[IRulexSR<'s>],
) -> Result<
  ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
  ICompileErrorS<'s>,
> {
  let mut grouped_explicit_types = std::collections::HashMap::<
    IRuneS<'s>,
    Vec<ITemplataType<'s>>,
  >::new();
  for (rune, explicit_type) in rune_to_explicit_type.iter() {
    grouped_explicit_types
      .entry(rune.clone())
      .or_default()
      .push(explicit_type.clone());
  }

  let mut rune_to_explicit_type = scout_arena.alloc_index_map::<IRuneS<'s>, ITemplataType>();
  for (rune, explicit_types) in grouped_explicit_types {
    let mut distinct_explicit_types =
      Vec::<ITemplataType>::new();
    for explicit_type in explicit_types {
      if !distinct_explicit_types.contains(&explicit_type) {
        distinct_explicit_types.push(explicit_type);
      }
    }
    if distinct_explicit_types.len() > 1 {
      return Err(ICompileErrorS::RuneExplicitTypeConflictS(RuneExplicitTypeConflictS {
        range: range_s.clone(),
        rune,
        types: distinct_explicit_types,
      }));
    }
    let explicit_type = match distinct_explicit_types.first() {
      None => panic!("POSTPARSER_PREDICT_RUNE_TYPES_EMPTY_EXPLICIT_TYPE_GROUP"),
      Some(tyype) => tyype.clone(),
    };
    rune_to_explicit_type.insert(rune, explicit_type);
  }
  Ok(rune_to_explicit_type)
}
/*

  def predictRuneTypes(
    rangeS: RangeS,
    identifyingRunesS: Vector[IRuneS],
    runeToExplicitTypeArray: mutable.ArrayBuffer[(IRuneS, ITemplataType)],
    rulesS: Vector[IRulexSR]):
  Map[IRuneS, ITemplataType] = {
    Profiler.frame(() => {
      val runeToExplicitType =
        runeToExplicitTypeArray
          .toVector
          .groupBy(_._1)
          .mapValues(_.map(_._2))
          .mapValues(_.distinct)
          .map({ case (rune, explicitTypes) =>
            if (explicitTypes.size > 1) {
              throw CompileErrorExceptionS(RuneExplicitTypeConflictS(rangeS, rune, explicitTypes))
            }
            (rune, vassertOne(explicitTypes))
          })
      val env =
        new IRuneTypeSolverEnv {
          override def lookup(range: RangeS, name: IImpreciseNameS):
          Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
            vimpl()
          }
        }
      val runeSToLocallyPredictedTypes =
        new RuneTypeSolver(interner).solve(
          globalOptions.sanityCheck,
          globalOptions.useOptimizedSolver,
          env, List(rangeS), true, rulesS, identifyingRunesS, false, runeToExplicitType) match {
          case Ok(t) => t
          // This likely cannot happen because we aren't even asking for a complete solve.
          case Err(e) => throw CompileErrorExceptionS(CouldntSolveRulesS(rangeS, e))
        }
      runeSToLocallyPredictedTypes
    })
  }
*/
pub(crate) fn check_identifiability(
  &self,
  range_s: RangeS<'s>,
  identifying_runes_s: &[IRuneS<'s>],
  rules_s: &'s [IRulexSR<'s>],
) -> Result<(), ICompileErrorS<'s>> {
  match crate::postparsing::identifiability_solver::solve_identifiability(
    self.global_options.sanity_check,
    self.global_options.use_optimized_solver,
    self.scout_arena,
    &[range_s.clone()],
    rules_s,
    identifying_runes_s,
  ) {
    Ok(_) => Ok(()),
    Err(e) => Err(ICompileErrorS::IdentifyingRunesIncompleteS(IdentifyingRunesIncompleteS {
      range: range_s,
      error: e,
    })),
  }
}
/*

  def checkIdentifiability(
    rangeS: RangeS,
    identifyingRunesS: Vector[IRuneS],
    rulesS: Vector[IRulexSR]):
  Unit = {
    IdentifiabilitySolver.solve(
      globalOptions.sanityCheck,
      globalOptions.useOptimizedSolver,
      interner,
      List(rangeS), rulesS, identifyingRunesS) match {
      case Ok(_) =>
      case Err(e) => throw CompileErrorExceptionS(IdentifyingRunesIncompleteS(rangeS, e))
    }
  }
*/
  fn scout_interface(
    &self,
    file: &'s FileCoordinate<'s>,
    interface: &InterfaceP<'p>,
  ) -> Result<InterfaceS<'s>, ICompileErrorS<'s>> {
    let interface_range_s = Self::eval_range(file, interface.range);
    let _body_range_s = Self::eval_range(file, interface.body_range);
    let interface_name = self.scout_arena.intern_interface_declaration_name(TopLevelInterfaceDeclarationNameS {
      name: self.scout_arena.intern_str(interface.name.str().as_str()),
      range: Self::eval_range(file, interface.name.range()),
    });
    let rules_p: &[IRulexPR<'p>] = interface
      .template_rules
      .as_ref()
      .map(|x| x.rules as &[IRulexPR<'p>])
      .unwrap_or(&[]);

    let mut lidb = LocationInDenizenBuilder::new(Vec::new());
    // V: is this whole function now closer or further from scala?
    // VA: Mostly closer — the full pipeline (attributes, generic params, rules, mutability, members,
    // VA: InterfaceS construction) is wired up and matches Scala's sequencing. The primary remaining
    // VA: gap is maybeDefaultRegionRuneP handling: Scala has a full match that synthesizes an implicit
    // VA: GenericParameterS with RegionGenericParameterTypeS(ReadWriteRegionS) on the None branch;
    // VA: Rust replaces this with two assert!(is_none) panics. Also: attribute computation is reordered
    // VA: (before generic params instead of after internalMethods), and predictRuneTypes receives
    // VA: &identifying_runes_s instead of Scala's empty ArrayBuffer.
    assert!(
      interface.maybe_default_region_rune.is_none(),
      "POSTPARSER_SCOUT_INTERFACE_DEFAULT_REGION_RUNE_NOT_YET_IMPLEMENTED"
    );

    let weakable = interface
      .attributes
      .iter()
      .any(|attr| matches!(attr, IAttributeP::WeakableAttribute(_)));
    let attrs_without_linear_s = Self::translate_citizen_attributes(
      self.scout_arena,
      file,
      self.scout_arena.intern_name(INameValS::TopLevelInterfaceDeclaration(interface_name.clone())),
      &interface
        .attributes
        .iter()
        .filter(|attr| {
          !matches!(
            attr,
            IAttributeP::WeakableAttribute(_) | IAttributeP::LinearAttribute(_)
          )
        })
        .cloned()
        .collect::<Vec<_>>(),
    );
    let mut attributes = attrs_without_linear_s;
    if let Some(IAttributeP::LinearAttribute(attr)) = interface
      .attributes
      .iter()
      .find(|attr| matches!(attr, IAttributeP::LinearAttribute(_)))
    {
      attributes.push(ICitizenAttributeS::MacroCall(MacroCallS {
        range: Self::eval_range(file, attr.range),
        include: IMacroInclusionP::DontCallMacro,
        macro_name: self.keywords.derive_struct_drop,
      }));
    }

    let generic_parameters_p: &[GenericParameterP<'p>] = interface
      .maybe_identifying_runes
      .as_ref()
      .map(|x| x.params as &[GenericParameterP<'p>])
      .unwrap_or(&[]);

    let user_specified_identifying_runes = generic_parameters_p
      .iter()
      .map(|generic_parameter| RuneUsage {
        range: Self::eval_range(file, generic_parameter.name.range()),
        rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(generic_parameter.name.str().as_str()),
        })),
      })
      .collect::<Vec<_>>();

    let runes_from_rules =
      get_ordered_rune_declarations_from_rulexes_with_duplicates(&rules_p)
      .iter()
      .map(|name_p| RuneUsage {
        range: Self::eval_range(file, name_p.range()),
        rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(name_p.str().as_str()),
        })),
      })
      .collect::<Vec<_>>();
    let user_declared_runes: IndexSet<IRuneS<'s>> = user_specified_identifying_runes
      .iter()
      .chain(runes_from_rules.iter())
      .map(|x| x.rune.clone())
      .collect();
    let interface_env = IEnvironmentS::Environment(EnvironmentS {
      file,
      parent_env: None,
      name: self.scout_arena.intern_name(INameValS::TopLevelInterfaceDeclaration(interface_name.clone())),
      user_declared_runes,
    });

    let mut rule_builder = Vec::<IRulexSR<'s>>::new();
    // This is an array instead of a map so we can detect conflicts afterward
    let mut rune_to_explicit_type = Vec::<(IRuneS<'s>, ITemplataType<'s>)>::new();

    // Put this back in when we have regions
    let default_region_rune_s = self.scout_arena.intern_rune(IRuneValS::DenizenDefaultRegionRune(
      DenizenDefaultRegionRuneS {
        denizen_name: self.scout_arena.intern_name(INameValS::TopLevelInterfaceDeclaration(
          interface_name.clone(),
        )),
      },
    ));

    let generic_parameters_s: Vec<&'s GenericParameterS<'s>> = generic_parameters_p
      .iter()
      .zip(user_specified_identifying_runes.iter())
      .map(|(g, r)| {
        &*self.scout_arena.alloc(self.scout_generic_parameter(
          interface_env.clone(),
          &mut lidb.child(),
          &mut rune_to_explicit_type,
          &mut rule_builder,
          default_region_rune_s.clone(),
          g,
          r.clone(),
        ))
      })
      .collect::<Vec<_>>();

    translate_rulexes(self.scout_arena,
      self.keywords,
      interface_env.clone(),
      &mut lidb.child(),
      &mut rule_builder,
      &mut rune_to_explicit_type,
      default_region_rune_s.clone(),
      &rules_p,
    );

    let default_mutability = ITemplexPT::Mutability(
      MutabilityPT(
        RangeL(interface.body_range.begin(), interface.body_range.begin()),
        MutabilityP::Mutable,
      ),
    );
    let mutability: &ITemplexPT<'p> = interface.mutability.as_ref().unwrap_or(&default_mutability);
    let mutability_rune_s = translate_templex(
      self.scout_arena,
      self.keywords,
      interface_env.clone(),
      &mut lidb.child(),
      &mut rule_builder,
      default_region_rune_s.clone(),
      mutability,
    );

    let rules_s = rule_builder;
    let identifying_runes_s = user_specified_identifying_runes
      .iter()
      .map(|x| x.rune.clone())
      .collect::<Vec<_>>();
    let predicted_rune_to_type = Self::predict_rune_types(
      self.scout_arena,
      interface_range_s.clone(),
      &identifying_runes_s,
      &mut rune_to_explicit_type.clone(),
      &rules_s,
    )?;

    let predicted_mutability =
      Self::predict_mutability(interface_range_s.clone(), mutability_rune_s.rune.clone(), &rules_s);

    let generic_parameters_s: &'s [&'s GenericParameterS<'s>] = self.scout_arena.alloc_slice_from_vec(generic_parameters_s);

    let param_types_vec: Vec<ITemplataType<'s>> = generic_parameters_s.iter().map(|x| x.tyype.tyype()).collect();
    let tyype = TemplateTemplataType {
      param_types: self.scout_arena.alloc_slice_copy(&param_types_vec),
      return_type: self.scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
    };

    let mut internal_methods = Vec::new();
    for member in interface.members {
      internal_methods.push(self.scout_interface_member(
        crate::postparsing::function_scout::ParentCitizen {
          citizen_is_interface: true,
          citizen_env: interface_env.clone(),
          citizen_generic_params: generic_parameters_s,
          citizen_rules: rules_s.clone(),
          citizen_rune_to_explicit_type: rune_to_explicit_type.iter().cloned().collect(),
        },
        member,
      )?);
    }

    Ok(InterfaceS::new(
      interface_range_s,
      interface_name,
      self.scout_arena.alloc_slice_from_vec(attributes),
      weakable,
      generic_parameters_s,
      self.scout_arena.alloc_index_map_from_iter(rune_to_explicit_type.into_iter()),
      mutability_rune_s,
      predicted_mutability,
      predicted_rune_to_type,
      tyype,
      self.scout_arena.alloc_slice_from_vec(rules_s),
      self.scout_arena.alloc_slice_from_vec(internal_methods),
    ))
  }
/*
  private def scoutInterface(
    file: FileCoordinate,
    containingInterfaceP: InterfaceP):
  InterfaceS = {
    val InterfaceP(interfaceRangeP, NameP(interfaceNameRangeS, interfaceHumanName), attributesP, mutabilityPT, maybeGenericParametersP, maybeRulesP, maybeDefaultRegionRuneP, bodyRangeP, internalMethodsP) = containingInterfaceP
    val interfaceRangeS = PostParser.evalRange(file, interfaceRangeP)
    val bodyRangeS = PostParser.evalRange(file, bodyRangeP)
    val interfaceFullName = interner.intern(postparsing.TopLevelInterfaceDeclarationNameS(interfaceHumanName, PostParser.evalRange(file, interfaceNameRangeS)))
    val rulesP = maybeRulesP.toVector.flatMap(_.rules)

    val lidb = new LocationInDenizenBuilder(Vector())

    val genericParametersP =
      maybeGenericParametersP
        .toVector
        .flatMap(_.params)

    val userSpecifiedIdentifyingRunes =
      genericParametersP
        .map({ case GenericParameterP(_, NameP(range, identifyingRuneName), _, _, _, _) =>
          rules.RuneUsage(PostParser.evalRange(file, range), CodeRuneS(identifyingRuneName))
        })

    val runesFromRules =
      RulePUtils.getOrderedRuneDeclarationsFromRulexesWithDuplicates(rulesP)
        .map({ case NameP(range, identifyingRuneName) => rules.RuneUsage(PostParser.evalRange(file, range), CodeRuneS(identifyingRuneName)) })
    val userDeclaredRunes = (userSpecifiedIdentifyingRunes.map(_.rune) ++ runesFromRules.map(_.rune)).distinct
    val interfaceEnv = postparsing.EnvironmentS(file, None, interfaceFullName, userDeclaredRunes.toSet)

    val ruleBuilder = ArrayBuffer[IRulexSR]()
    // This is an array instead of a map so we can detect conflicts afterward
    val runeToExplicitType = mutable.ArrayBuffer[(IRuneS, ITemplataType)]()

    val (defaultRegionRuneRangeS, defaultRegionRuneS, maybeRegionGenericParam) =
      maybeDefaultRegionRuneP match {
        case None => {
          val regionRange = RangeS(bodyRangeS.begin, bodyRangeS.begin)
          val rune = DenizenDefaultRegionRuneS(interfaceFullName)
          vregionmut() // Put this back in when we have regions
          // runeToExplicitType += ((rune, RegionTemplataType()))
          val implicitRegionGenericParam =
            GenericParameterS(regionRange, RuneUsage(regionRange, rune), RegionGenericParameterTypeS(ReadWriteRegionS), None)
          (regionRange, rune, Some(implicitRegionGenericParam))
        }
        case Some(RegionRunePT(regionRangeP, regionName)) => {
          val regionRangeS = evalRange(file, regionRangeP)
          val rune = CodeRuneS(vassertSome(regionName).str) // impl isolates
          if (!interfaceEnv.allDeclaredRunes().contains(rune)) {
            throw CompileErrorExceptionS(CouldntFindRuneS(regionRangeS, rune.name.str))
          }
          (regionRangeS, rune, None)
        }
      }

    val interfaceUserSpecifiedGenericParametersS =
      genericParametersP.zip(userSpecifiedIdentifyingRunes)
        .map({ case (g, r) =>
          PostParser.scoutGenericParameter(
            templexScout, interfaceEnv, lidb.child(), runeToExplicitType, ruleBuilder, defaultRegionRuneS, g, r)
        })

    val genericParametersS =
      interfaceUserSpecifiedGenericParametersS
      //++ userSpecifiedRunesImplicitRegionRunesS

    ruleScout.translateRulexes(interfaceEnv, lidb.child(), ruleBuilder, runeToExplicitType, defaultRegionRuneS, rulesP)

    val mutability =
      mutabilityPT.getOrElse(MutabilityPT(RangeL(bodyRangeP.begin, bodyRangeP.begin), MutableP))
    val mutabilityRuneS =
      templexScout.translateTemplex(
        interfaceEnv, lidb.child(), ruleBuilder, defaultRegionRuneS, mutability)


    val rulesS = ruleBuilder.toVector

    val runeToPredictedType = predictRuneTypes(interfaceRangeS, userDeclaredRunes, mutable.ArrayBuffer(), rulesS)

    val predictedMutability = predictMutability(interfaceRangeS, mutabilityRuneS.rune, rulesS)

    val tyype = TemplateTemplataType(genericParametersS.map(_.tyype.tyype), KindTemplataType())

    val internalMethodsS =
      internalMethodsP.map(method => {
        functionScout.scoutInterfaceMember(
          ParentCitizen(
            true,  // interface internal methods do require virtual self
            interfaceEnv,
            genericParametersS.toVector,
            rulesS,
            runeToExplicitType.toMap),
          method)
      })

    val weakable = attributesP.exists({ case w @ WeakableAttributeP(_) => true case _ => false })
    val attrsWithoutLinearS = translateCitizenAttributes(file, interfaceFullName, attributesP.filter({ case WeakableAttributeP(_) => false case LinearAttributeP(_) => false case _ => true }))
    val attrsS =
      attrsWithoutLinearS ++
          (attributesP.collectFirst({ case w@LinearAttributeP(_) => w }) match {
            case None => None
            case Some(LinearAttributeP(range)) => {
              Some(MacroCallS(evalRange(file, range), DontCallMacroP, keywords.DeriveStructDrop))
            }
          })

    val interfaceS =
      InterfaceS(
        interfaceRangeS,
        interfaceFullName,
        attrsS,
        weakable,
//        knowableValueRunes,
        genericParametersS,
        runeToExplicitType.toMap,
//        localRunes,
//        maybePredictedType,
        mutabilityRuneS,
        predictedMutability,
        runeToPredictedType,
        tyype,
//        isTemplate,
        rulesS,
//        runeSToCanonicalRune,
        internalMethodsS)

    interfaceS
  }

}
*/
}
/*
Guardian: disable-all
*/

pub struct ScoutCompilation<'s, 'ctx, 'p> {
  global_options: GlobalOptions,
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  keywords_p: &'ctx Keywords<'p>,
  parse_arena: &'ctx ParseArena<'p>,
  parser_compilation: ParserCompilation<'p, 'ctx>,
  scoutput_cache: Option<FileCoordinateMap<'s, ProgramS<'s>>>,
}
/*
class ScoutCompilation(
  globalOptions: GlobalOptions,
  interner: Interner,
  keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]]) {
  var parserCompilation = new ParserCompilation(globalOptions, interner, keywords, packagesToBuild, packageToContentsResolver)
  var scoutputCache: Option[FileCoordinateMap[ProgramS]] = None
*/

impl<'s, 'ctx, 'p> ScoutCompilation<'s, 'ctx, 'p>
{
  // MIGALLOW: new -> new (From PostParser.scala lines 922-928)
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    global_options: GlobalOptions,
  ) -> Self {
    let parser_compilation = ParserCompilation::new(
      global_options.clone(),
      parse_arena,
      parser_keywords,
      packages_to_build,
      package_to_contents_resolver,
    );

    ScoutCompilation {
      global_options,
      scout_arena,
      keywords,
      keywords_p: parser_keywords,
      parse_arena,
      parser_compilation,
      scoutput_cache: None,
    }
    // V: anything enforce that we actually have to call this constructor?
    // VA: All ScoutCompilation fields are private, so external callers are forced through new().
    // But code within the same module can bypass it via struct literal — no language-level
    // enforcement within the module.
    // V: is there a way we can make it so people cant bypass it?
    // VA: Yes — move ScoutCompilation to its own submodule (e.g. postparsing/scout_compilation.rs).
    // VA: Rust's privacy is module-scoped, so code in the same module can always bypass private
    // VA: fields via struct literal syntax. A separate module would make the private fields
    // VA: truly inaccessible from the rest of postparsing.
  }
  /*
  Guardian: disable-all
  */

  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.parser_compilation.get_code_map()
  }
  /*
    def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = parserCompilation.getCodeMap()
  */

  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
    self.parser_compilation.get_parseds()
  }
  /*
    def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = parserCompilation.getParseds()
  */

  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.parser_compilation.get_vpst_map()
  }
  /*
    def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = parserCompilation.getVpstMap()
  */
}
/*
Guardian: disable-all
*/

impl<'s, 'ctx, 'p> ScoutCompilation<'s, 'ctx, 'p>
{
  // From PostParser.scala lines 935-950: getScoutput
  pub fn get_scoutput(&mut self) -> Result<&FileCoordinateMap<'s, ProgramS<'s>>, ICompileErrorS<'s>> {
    if self.scoutput_cache.is_some() {
      return Ok(self.scoutput_cache.as_ref().unwrap());
    }

    let parseds = self.parser_compilation.expect_parseds();
    let post_parser = PostParser::new(
      self.global_options.clone(),
      self.scout_arena,
      self.keywords,
      self.keywords_p,
      self.parse_arena,
    );
    let mut scoutput: FileCoordinateMap<'s, ProgramS<'s>> = FileCoordinateMap::new();
    for (file_coordinate_p, (file_p, _comments_and_ranges)) in &parseds.file_coord_to_contents {
      // Cross-pass translation: re-intern FileCoordinate from 'p into 's
      // V: should we have arcana for this? also its weird how verbose this is compared to the scala version below
      // VA: The verbosity is inherent to the 'p/'s lifetime split — already explained in the VA below.
      // VA: The verbosity is an inherent consequence of the 'p/'s lifetime split. Scala's single
      // GC-backed interner didn't need cross-pass re-interning — coordinates were just reused.
      // In Rust, every StrI<'p> must become StrI<'s> by re-interning through scout_arena.
      // Good candidate for arcana documentation.
      // V: can we have an intern_file_coordinate method on scout_arena that takes one from a
      // different arena like p?
      // VA: Yes. A method like `intern_file_coordinate_cross_pass(&self, fc: &FileCoordinate<'_>) -> &'s FileCoordinate<'s>`
      // VA: would re-intern each package string via .as_str(), call intern_package_coordinate, then
      // VA: intern_file_coordinate. Needs only &self — no ParseArena param, since StrI exposes .as_str().
      // VA: Would reduce the two repeated 6-line blocks in this loop to one-liner calls.
      let package_coord_s: &'s PackageCoordinate<'s> = self.scout_arena.intern_package_coordinate(
        self.scout_arena.intern_str(file_coordinate_p.package_coord.module.as_str()),
        &file_coordinate_p.package_coord.packages.iter()
          .map(|s| self.scout_arena.intern_str(s.as_str()))
          .collect::<Vec<_>>(),
      );
      let file_coordinate_s: &'s FileCoordinate<'s> = self.scout_arena.intern_file_coordinate(
        package_coord_s,
        file_coordinate_p.filepath.as_str(),
      );
      let program_s = post_parser.scout_program(file_coordinate_s, file_p)?;
      scoutput.put(file_coordinate_s, program_s);
    }
    // Re-intern package_coord_to_file_coords from 'p to 's
    for (pkg_p, files_p) in &parseds.package_coord_to_file_coords {
      let pkg_s = self.scout_arena.intern_package_coordinate(
        self.scout_arena.intern_str(pkg_p.module.as_str()),
        &pkg_p.packages.iter().map(|s| self.scout_arena.intern_str(s.as_str())).collect::<Vec<_>>(),
      );
      let files_s: Vec<&'s FileCoordinate<'s>> = files_p.iter().map(|fc| {
        self.scout_arena.intern_file_coordinate(pkg_s, fc.filepath.as_str())
      }).collect();
      scoutput.package_coord_to_file_coords.insert(pkg_s, files_s);
    }
    self.scoutput_cache = Some(scoutput);
    Ok(self.scoutput_cache.as_ref().unwrap())
  }

/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = {
    scoutputCache match {
      case Some(scoutput) => Ok(scoutput)
      case None => {
        val scoutput =
          parserCompilation.expectParseds().map({ case (fileCoordinate, (code, commentsAndRanges)) =>
            new PostParser(globalOptions, interner, keywords).scoutProgram(fileCoordinate, code) match {
              case Err(e) => return Err(e)
              case Ok(p) => p
            }
          })
        scoutputCache = Some(scoutput)
        Ok(scoutput)
      }
    }
  }
*/
  // From PostParser.scala lines 951-964: expectScoutput
  pub fn expect_scoutput(&mut self) -> &FileCoordinateMap<'s, ProgramS<'s>> {
    match self.get_scoutput() {
      Ok(x) => x,
      Err(e) => {
        panic!("ScoutCompilation.expect_scoutput failed: {:?}", e)
      }
    }
  }

/*
  def expectScoutput(): FileCoordinateMap[ProgramS] = {
    getScoutput() match {
      case Ok(x) => x
      case Err(e) => {
        val codeMap = getCodeMap().getOrDie()
        vfail(PostParserErrorHumanizer.humanize(
          SourceCodeUtils.humanizePos(codeMap, _),
          SourceCodeUtils.linesBetween(codeMap, _, _),
          SourceCodeUtils.lineRangeContaining(codeMap, _),
          SourceCodeUtils.lineContaining(codeMap, _),
          e))
      }
    }
  }
*/
}
/*
}
*/
