// From Frontend/PassManager/src/dev/vale/passmanager/FullCompilation.scala
// Coordinates the full compilation pipeline

use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;
use crate::parse_arena::ParseArena;
use crate::simplifying::hammer_compilation::{HammerCompilation, HammerCompilationOptions};
use crate::simplifying::hammer_interner::HammerInterner;
use crate::typing::typing_interner::TypingInterner;

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
pub struct FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where
  's: 'h,
  's: 't,
  's: 'i,
  'p: 'ctx,
{
  pub hammer_compilation: HammerCompilation<'s, 'h, 'ctx, 't, 'i, 'p>,
}
/*
class FullCompilation(
  interner: Interner,
  keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: FullCompilationOptions = FullCompilationOptions()) {
*/

impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where
  's: 'h,
  's: 't,
  's: 'i,
  'p: 'ctx,
{
  // From FullCompilation.scala lines 30-45
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    interner: &'ctx HammerInterner<'s, 'h>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    // VV: crate::
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    options: FullCompilationOptions,
    instantiating_bump: &'i Bump,
  ) -> Self {
    let hammer_options = HammerCompilationOptions {
      debug_out: options.debug_out,
      global_options: options.global_options,
    };
    let hammer_compilation = HammerCompilation::new(
      scout_arena,
      interner,
      typing_interner,
      keywords,
      parser_keywords,
      parse_arena,
      packages_to_build,
      package_to_contents_resolver,
      hammer_options,
      instantiating_bump,
    );
    FullCompilation { hammer_compilation }
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

}

// mig: fn get_code_map
impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.hammer_compilation.get_code_map()
  }
}
/*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = hammerCompilation.getCodeMap()
*/

// mig: fn get_parseds
impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
    self.hammer_compilation.get_parseds()
  }
}
/*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = hammerCompilation.getParseds()
*/

// mig: fn get_vpst_map
impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.hammer_compilation.get_vpst_map()
  }
}
/*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = hammerCompilation.getVpstMap()
*/

// mig: fn get_scoutput
impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn get_scoutput(&mut self) -> Result<&crate::utils::code_hierarchy::FileCoordinateMap<'s, crate::postparsing::ast::ProgramS<'s>>, crate::postparsing::post_parser::ICompileErrorS<'s>> {
    self.hammer_compilation.get_scoutput()
  }
}
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = hammerCompilation.getScoutput()
*/

// mig: fn get_astrouts
impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn get_astrouts(&mut self) -> Result<(), String> {
    self.hammer_compilation.get_astrouts()
  }
}
/*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = hammerCompilation.getAstrouts()
*/

// mig: fn get_compiler_outputs
impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn get_compiler_outputs(&mut self) -> Result<&crate::typing::hinputs_t::HinputsT<'s, 't>, crate::typing::compiler_error_reporter::ICompileErrorT<'s, 't>> {
    self.hammer_compilation.get_compiler_outputs()
  }
}
/*
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = hammerCompilation.getCompilerOutputs()
*/

// mig: fn expect_compiler_outputs
impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn expect_compiler_outputs(&mut self) -> &crate::typing::hinputs_t::HinputsT<'s, 't> {
    self.hammer_compilation.expect_compiler_outputs()
  }
}
/*
  def expectCompilerOutputs(): HinputsT = hammerCompilation.expectCompilerOutputs()
*/

// mig: fn get_hamuts
impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn get_hamuts(&mut self) -> &'h crate::final_ast::ast::ProgramH<'s, 'h> {
    self.hammer_compilation.get_hamuts()
  }
}
/*
  def getHamuts(): ProgramH = hammerCompilation.getHamuts()
*/

// mig: fn get_monouts
impl<'s, 'h, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn get_monouts(&mut self) -> &crate::instantiating::ast::hinputs::HinputsI<'s, 'i> {
    self.hammer_compilation.get_monouts()
  }
}
/*
  def getMonouts(): HinputsI = hammerCompilation.getMonouts()
}
*/
