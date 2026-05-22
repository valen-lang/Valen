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

// mig: struct HammerCompilationOptions
#[derive(PartialEq, Eq, Hash)]
pub struct HammerCompilationOptions {
  pub debug_out: Arc<dyn Fn(&str) + Send + Sync>,
  pub global_options: GlobalOptions,
}
// mig: impl HammerCompilationOptions
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for HammerCompilationOptions` or `#[derive(Hash)]`.)
/*
case class HammerCompilationOptions(
  debugOut: (=> String) => Unit = (x => {
    println("##: " + x)
  }),
  globalOptions: GlobalOptions = GlobalOptions()
) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for HammerCompilationOptions` or `#[derive(PartialEq)]`.)
/*
override def equals(obj: Any): Boolean = vcurious(); }
*/

// mig: struct HammerCompilation
#[derive(PartialEq, Eq, Hash)]
pub struct HammerCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{
  pub interner: &'ctx HammerInterner<'h>,
  pub keywords: &'ctx Keywords<'s>,
  pub packages_to_build: Vec<&'ctx PackageCoordinate<'p>>,
  pub package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
  pub options: HammerCompilationOptions,
}
// mig: impl HammerCompilation
/*
class HammerCompilation(
  val interner: Interner,
  val keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: HammerCompilationOptions = HammerCompilationOptions()) {

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
// mig: fn get_von_hammer
pub fn get_von_hammer() -> () {
  panic!("Unimplemented: get_von_hammer");
}
/*
  def getVonHammer() = vassertSome(vonHammerCache)
*/

// mig: fn get_code_map
pub fn get_code_map() -> Result<(), String> {
  panic!("Unimplemented: get_code_map");
}
/*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = instantiatedCompilation.getCodeMap()
*/

// mig: fn get_parseds
pub fn get_parseds() -> Result<(), String> {
  panic!("Unimplemented: get_parseds");
}
/*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = instantiatedCompilation.getParseds()
*/

// mig: fn get_vpst_map
pub fn get_vpst_map() -> Result<(), String> {
  panic!("Unimplemented: get_vpst_map");
}
/*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = instantiatedCompilation.getVpstMap()
*/

// mig: fn get_scoutput
pub fn get_scoutput() -> Result<(), String> {
  panic!("Unimplemented: get_scoutput");
}
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = instantiatedCompilation.getScoutput()
*/

// mig: fn get_astrouts
pub fn get_astrouts() -> Result<(), String> {
  panic!("Unimplemented: get_astrouts");
}
/*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = instantiatedCompilation.getAstrouts()
*/

// mig: fn get_compiler_outputs
pub fn get_compiler_outputs() -> Result<(), String> {
  panic!("Unimplemented: get_compiler_outputs");
}
/*
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = instantiatedCompilation.getCompilerOutputs()
*/

// mig: fn get_monouts
pub fn get_monouts() -> () {
  panic!("Unimplemented: get_monouts");
}
/*
  def getMonouts(): HinputsI = instantiatedCompilation.getMonouts()
*/

// mig: fn expect_compiler_outputs
pub fn expect_compiler_outputs() -> () {
  panic!("Unimplemented: expect_compiler_outputs");
}
/*
  def expectCompilerOutputs(): HinputsT = instantiatedCompilation.expectCompilerOutputs()
*/

// mig: fn get_hamuts
pub fn get_hamuts() -> () {
  panic!("Unimplemented: get_hamuts");
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
