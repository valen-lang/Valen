// From Frontend/PassManager/src/dev/vale/passmanager/FullCompilation.scala
// Coordinates the full compilation pipeline

use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use std::marker::PhantomData;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;
use crate::parse_arena::ParseArena;

/*
package dev.vale.passmanager

import dev.vale.finalast.ProgramH
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.postparsing._
import dev.vale.{Builtins, Err, FileCoordinate, FileCoordinateMap, IPackageResolver, Interner, Keywords, Ok, PackageCoordinate, PackageCoordinateMap, Profiler, Result, vassert, vassertSome, vcurious, vfail, vimpl, vwat}
import dev.vale.highertyping.ICompileErrorA
import PassManager.SourceInput
import dev.vale.highertyping.{ICompileErrorA, ProgramA}
import dev.vale.instantiating.ast.HinputsI
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.simplifying._
import dev.vale.typing.{HinputsT, ICompileErrorT}
import dev.vale.instantiating.ast.HinputsI
import dev.vale.postparsing.PostParser
import dev.vale.simplifying._
import dev.vale.typing.ICompileErrorT
import dev.vale.testvm.ReferenceV

import scala.collection.immutable.List
*/

// From FullCompilation.scala lines 23-28: FullCompilationOptions
pub struct FullCompilationOptions {
  pub global_options: GlobalOptions,
  pub debug_out: Arc<dyn Fn(&str) + Send + Sync>,
}
/*
case class FullCompilationOptions(
  globalOptions: GlobalOptions = GlobalOptions(false, true, true, false, false),
  debugOut: (=> String) => Unit = (x => {
    println("##: " + x)
  }),
) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/

// From FullCompilation.scala lines 30-57: FullCompilation class
pub struct FullCompilation<'s, 'ctx, 't, 'p>
where
  's: 't,
  'p: 'ctx,
{
  // Hammer wiring is stubbed pending the simplifying-pass body migration
  // (transplanted HammerCompilation has no constructor yet). PhantomData
  // keeps the four lifetimes live without holding a real HammerCompilation.
  _marker: PhantomData<(&'t &'s (), &'ctx &'p ())>,
}
/*
class FullCompilation(
  interner: Interner,
  keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: FullCompilationOptions = FullCompilationOptions()) {
*/

impl<'s, 'ctx, 't, 'p> FullCompilation<'s, 'ctx, 't, 'p>
where
  's: 't,
  'p: 'ctx,
{
  // From FullCompilation.scala lines 30-45
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    // VV: crate::
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    options: FullCompilationOptions,
    typing_bump: &'t Bump,
  ) -> Self {
    panic!("Unimplemented: HammerCompilation wiring pending simplifying-pass body migration (transplanted HammerCompilation has no constructor yet) - see HammerCompilation.scala");
  }
/*
  var hammerCompilation =
    new HammerCompilation(
      interner,
      keywords,
      packagesToBuild,
      packageToContentsResolver,
      HammerCompilationOptions(
        options.debugOut,
        options.globalOptions))

  def getVonHammer(): VonHammer = hammerCompilation.getVonHammer()
*/

  // From FullCompilation.scala line 48: getCodeMap
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    panic!("FullCompilation.get_code_map: HammerCompilation wiring pending simplifying-pass body migration")
  }
/*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = hammerCompilation.getCodeMap()
*/

  // From FullCompilation.scala line 49: getParseds
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
    panic!("FullCompilation.get_parseds: HammerCompilation wiring pending simplifying-pass body migration")
  }
/*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = hammerCompilation.getParseds()
*/

  // From FullCompilation.scala line 50: getVpstMap
  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    panic!("FullCompilation.get_vpst_map: HammerCompilation wiring pending simplifying-pass body migration")
  }
/*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = hammerCompilation.getVpstMap()
*/

  // From FullCompilation.scala line 51: getScoutput
  pub fn get_scoutput(&mut self) -> Result<(), String> {
    panic!("FullCompilation.get_scoutput not yet implemented - see FullCompilation.scala line 51")
  }
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = hammerCompilation.getScoutput()
*/

  // From FullCompilation.scala line 52: getAstrouts
  pub fn get_astrouts(&mut self) -> Result<(), String> {
    panic!("FullCompilation.get_astrouts not yet implemented - see FullCompilation.scala line 52")
  }
/*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = hammerCompilation.getAstrouts()
*/

  // From FullCompilation.scala line 53: getCompilerOutputs
  pub fn get_compiler_outputs(&mut self) -> Result<(), String> {
    panic!("FullCompilation.get_compiler_outputs not yet implemented - see FullCompilation.scala line 53")
  }
/*
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = hammerCompilation.getCompilerOutputs()
*/

  // From FullCompilation.scala line 54: expectCompilerOutputs
  pub fn expect_compiler_outputs(&mut self) -> () {
    panic!("FullCompilation.expect_compiler_outputs not yet implemented - see FullCompilation.scala line 54")
  }
/*
  def expectCompilerOutputs(): HinputsT = hammerCompilation.expectCompilerOutputs()
*/

  // From FullCompilation.scala line 55: getHamuts
  pub fn get_hamuts(&mut self) -> () {
    panic!("FullCompilation.get_hamuts not yet implemented - see FullCompilation.scala line 55")
  }
/*
  def getHamuts(): ProgramH = hammerCompilation.getHamuts()
*/

  // From FullCompilation.scala line 56: getMonouts
  pub fn get_monouts(&mut self) -> () {
    panic!("FullCompilation.get_monouts not yet implemented - see FullCompilation.scala line 56")
  }
/*
  def getMonouts(): HinputsI = hammerCompilation.getMonouts()
}
*/
}
