// From Frontend/SimplifyingPass/src/dev/vale/simplifying/HammerCompilation.scala
// Coordinates the Hammer (simplifying) pass

use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::instantiating::InstantiatedCompilation;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::pass_manager::FullCompilationOptions;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;
use crate::parse_arena::ParseArena;

/*
package dev.vale.simplifying

import dev.vale.highertyping.{ICompileErrorA, ProgramA}
import dev.vale.finalast.ProgramH
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.postparsing._
import dev.vale.{FileCoordinateMap, IPackageResolver, Interner, Keywords, PackageCoordinate, PackageCoordinateMap, Profiler, Result, vassertSome, vcurious, vimpl}
import dev.vale.highertyping.ICompileErrorA
import dev.vale.instantiating.ast.HinputsI
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.instantiating.{InstantiatedCompilation, InstantiatorCompilationOptions}
import dev.vale.postparsing.ICompileErrorS
import dev.vale.typing.{HinputsT, ICompileErrorT}

import scala.collection.immutable.List
*/

// From HammerCompilation.scala lines 18-23: HammerCompilationOptions
pub struct HammerCompilationOptions {
  pub debug_out: Arc<dyn Fn(&str) + Send + Sync>,
  pub global_options: GlobalOptions,
}
/*
case class HammerCompilationOptions(
  debugOut: (=> String) => Unit = (x => {
    println("##: " + x)
  }),
  globalOptions: GlobalOptions = GlobalOptions()
) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/

// From HammerCompilation.scala lines 25-66: HammerCompilation class
pub struct HammerCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{
  instantiated_compilation: InstantiatedCompilation<'s, 'ctx, 't, 'p>,
  #[allow(dead_code)]
  hamuts_cache: Option<()>, // ProgramH not yet ported
  #[allow(dead_code)]
  von_hammer_cache: Option<()>, // VonHammer not yet ported
}
/*
class HammerCompilation(
  val interner: Interner,
  val keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: HammerCompilationOptions = HammerCompilationOptions()) {
*/

impl<'s, 'ctx, 't, 'p> HammerCompilation<'s, 'ctx, 't, 'p>
where
  's: 't,
  'p: 'ctx,
{
  // From HammerCompilation.scala lines 25-40
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    options: FullCompilationOptions,
    typing_bump: &'t Bump,
  ) -> Self {
    let hammer_options = HammerCompilationOptions {
      debug_out: options.debug_out.clone(),
      global_options: options.global_options,
    };

    let instantiated_compilation = InstantiatedCompilation::new(
      scout_arena,
      keywords,
      parser_keywords,
      parse_arena,
      packages_to_build,
      package_to_contents_resolver,
      hammer_options,
      typing_bump,
    );

    HammerCompilation {
      instantiated_compilation,
      hamuts_cache: None,
      von_hammer_cache: None,
    }
  }
/*
  var instantiatedCompilation =
    new InstantiatedCompilation(
      interner,
      keywords,
      packagesToBuild,
      packageToContentsResolver,
      InstantiatorCompilationOptions(
        options.globalOptions,
        options.debugOut))
  var hamutsCache: Option[ProgramH] = None
  var vonHammerCache: Option[VonHammer] = None
*/

  // From HammerCompilation.scala line 43: getVonHammer
  pub fn get_von_hammer(&self) -> () {
    panic!(
      "HammerCompilation.get_von_hammer not yet implemented - see HammerCompilation.scala line 43"
    )
  }
/*
  def getVonHammer() = vassertSome(vonHammerCache)
*/

  // From HammerCompilation.scala line 45: getCodeMap
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.instantiated_compilation.get_code_map()
  }
/*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = instantiatedCompilation.getCodeMap()
*/

  // From HammerCompilation.scala line 46: getParseds
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
    self.instantiated_compilation.get_parseds()
  }
/*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = instantiatedCompilation.getParseds()
*/

  // From HammerCompilation.scala line 47: getVpstMap
  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.instantiated_compilation.get_vpst_map()
  }
/*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = instantiatedCompilation.getVpstMap()
*/

  // From HammerCompilation.scala line 48: getScoutput
  pub fn get_scoutput(&mut self) -> Result<(), String> {
    panic!(
      "HammerCompilation.get_scoutput not yet implemented - see HammerCompilation.scala line 48"
    )
  }
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = instantiatedCompilation.getScoutput()
*/

  // From HammerCompilation.scala line 49: getAstrouts
  pub fn get_astrouts(&mut self) -> Result<(), String> {
    panic!(
      "HammerCompilation.get_astrouts not yet implemented - see HammerCompilation.scala line 49"
    )
  }
/*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = instantiatedCompilation.getAstrouts()
*/

  // From HammerCompilation.scala line 50: getCompilerOutputs
  pub fn get_compiler_outputs(&mut self) -> Result<(), String> {
    panic!("HammerCompilation.get_compiler_outputs not yet implemented - see HammerCompilation.scala line 50")
  }
/*
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = instantiatedCompilation.getCompilerOutputs()
*/

  // From HammerCompilation.scala line 51: getMonouts
  pub fn get_monouts(&mut self) -> () {
    panic!(
      "HammerCompilation.get_monouts not yet implemented - see HammerCompilation.scala line 51"
    )
  }
/*
  def getMonouts(): HinputsI = instantiatedCompilation.getMonouts()
*/

  // From HammerCompilation.scala line 52: expectCompilerOutputs
  pub fn expect_compiler_outputs(&mut self) -> () {
    panic!("HammerCompilation.expect_compiler_outputs not yet implemented - see HammerCompilation.scala line 52")
  }
/*
  def expectCompilerOutputs(): HinputsT = instantiatedCompilation.expectCompilerOutputs()
*/

  // From HammerCompilation.scala lines 54-65: getHamuts
  pub fn get_hamuts(&mut self) -> () {
    panic!(
      "HammerCompilation.get_hamuts not yet implemented - see HammerCompilation.scala lines 54-65"
    )
  }
/*
  def getHamuts(): ProgramH = {
    hamutsCache match {
      case Some(hamuts) => hamuts
      case None => {
        val hammer = new Hammer(interner, keywords)
        val hamuts = hammer.translate(instantiatedCompilation.getMonouts())
        hamutsCache = Some(hamuts)
        vonHammerCache = Some(hammer.vonHammer)
        hamuts
      }
    }
  }
}
*/
}
