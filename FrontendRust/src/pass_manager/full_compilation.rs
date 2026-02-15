// From Frontend/PassManager/src/dev/vale/passmanager/FullCompilation.scala
// Coordinates the full compilation pipeline

use crate::compile_options::GlobalOptions;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::simplifying::HammerCompilation;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;

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

// From FullCompilation.scala lines 30-57: FullCompilation class
pub struct FullCompilation<'a, 'i, 'k, 'b>
where
  'a: 'i,
  'a: 'k,
  'a: 'b,
{
  hammer_compilation: HammerCompilation<'a, 'i, 'k, 'b>,
}

impl<'a, 'i, 'k, 'b> FullCompilation<'a, 'i, 'k, 'b>
where
  'a: 'i,
  'a: 'k,
  'a: 'b,
{
  // From FullCompilation.scala lines 30-45
  pub fn new(
    interner: &'i Interner<'a>,
    keywords: &'k Keywords<'a>,
    packages_to_build: Vec<&'a PackageCoordinate<'a>>,
    package_to_contents_resolver: &'b dyn IPackageResolver<'a, HashMap<String, String>>,
    options: FullCompilationOptions,
  ) -> Self {
    let hammer_compilation = HammerCompilation::new(
      interner,
      keywords,
      packages_to_build,
      package_to_contents_resolver,
      options,
    );
    FullCompilation { hammer_compilation }
  }

  // From FullCompilation.scala line 48: getCodeMap
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse> {
    self.hammer_compilation.get_code_map()
  }

  // From FullCompilation.scala line 49: getParseds
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'a, (FileP, Vec<RangeL>)>, FailedParse> {
    self.hammer_compilation.get_parseds()
  }

  // From FullCompilation.scala line 50: getVpstMap
  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse> {
    self.hammer_compilation.get_vpst_map()
  }

  // From FullCompilation.scala line 51: getScoutput
  pub fn get_scoutput(&mut self) -> Result<(), String> {
    panic!("FullCompilation.get_scoutput not yet implemented - see FullCompilation.scala line 51")
  }

  // From FullCompilation.scala line 52: getAstrouts
  pub fn get_astrouts(&mut self) -> Result<(), String> {
    panic!("FullCompilation.get_astrouts not yet implemented - see FullCompilation.scala line 52")
  }

  // From FullCompilation.scala line 53: getCompilerOutputs
  pub fn get_compiler_outputs(&mut self) -> Result<(), String> {
    panic!("FullCompilation.get_compiler_outputs not yet implemented - see FullCompilation.scala line 53")
  }

  // From FullCompilation.scala line 54: expectCompilerOutputs
  pub fn expect_compiler_outputs(&mut self) -> () {
    panic!("FullCompilation.expect_compiler_outputs not yet implemented - see FullCompilation.scala line 54")
  }

  // From FullCompilation.scala line 55: getHamuts
  pub fn get_hamuts(&mut self) -> () {
    panic!("FullCompilation.get_hamuts not yet implemented - see FullCompilation.scala line 55")
  }

  // From FullCompilation.scala line 56: getMonouts
  pub fn get_monouts(&mut self) -> () {
    panic!("FullCompilation.get_monouts not yet implemented - see FullCompilation.scala line 56")
  }
}

/*

case class FullCompilationOptions(
  globalOptions: GlobalOptions = GlobalOptions(false, true, true, false, false),
  debugOut: (=> String) => Unit = (x => {
    println("##: " + x)
  }),
) { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }

class FullCompilation(
  interner: Interner,
  keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: FullCompilationOptions = FullCompilationOptions()) {
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

  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = hammerCompilation.getCodeMap()
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = hammerCompilation.getParseds()
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = hammerCompilation.getVpstMap()
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = hammerCompilation.getScoutput()
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = hammerCompilation.getAstrouts()
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = hammerCompilation.getCompilerOutputs()
  def expectCompilerOutputs(): HinputsT = hammerCompilation.expectCompilerOutputs()
  def getHamuts(): ProgramH = hammerCompilation.getHamuts()
  def getMonouts(): HinputsI = hammerCompilation.getMonouts()
}
*/
