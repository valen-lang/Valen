// From Frontend/SimplifyingPass/src/dev/vale/simplifying/HammerCompilation.scala
// Coordinates the Hammer (simplifying) pass

use crate::compile_options::GlobalOptions;
use crate::keywords::Keywords;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use crate::scout_arena::ScoutArena;
use crate::parse_arena::ParseArena;
use crate::instantiating::instantiated_compilation::{InstantiatedCompilation, InstantiatorCompilationOptions};
use bumpalo::Bump;
use std::collections::HashMap;
use std::sync::Arc;

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
//
// Drops PartialEq/Eq/Hash because `debug_out: Arc<dyn Fn(...)>` and
// `global_options: GlobalOptions` don't impl them. Scala uses vcurious.
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
pub struct HammerCompilation<'s, 'h, 'ctx, 't, 'p>
where 's: 'h,
{
  pub interner: &'ctx HammerInterner<'s, 'h>,
  pub keywords: &'ctx Keywords<'s>,
  pub packages_to_build: Vec<&'p PackageCoordinate<'p>>,
  pub package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
  pub options: HammerCompilationOptions,
  pub instantiated_compilation: crate::instantiating::instantiated_compilation::InstantiatedCompilation<'s, 'ctx, 't, 'p>,
  pub hamuts_cache: Option<crate::final_ast::ast::ProgramH<'s, 'h>>,
  // Scala has `var vonHammerCache: Option[VonHammer]`. Dropped: per
  // typing-pass precedent the VonHammer compiler class was collapsed
  // onto `Hammer` (no separate VonHammer state), so there is nothing
  // to cache.
}
// mig: impl HammerCompilation
/*
class HammerCompilation(
  val interner: Interner,
  val keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: HammerCompilationOptions = HammerCompilationOptions()) {
*/

// mig: fn new
// (Slice pipeline emitted no `new` stub — TL-added per NNDX escalation, mirroring
// InstantiatedCompilation::new's hoisted-arena signature plus a hammer Bump-backed
// HammerInterner. The Scala constructor body is the class param block below.)
impl<'s, 'h, 'ctx, 't, 'p> HammerCompilation<'s, 'h, 'ctx, 't, 'p>
where
    's: 'h,
    's: 't,
    'p: 'ctx,
{
  pub fn new(
    interner: &'ctx HammerInterner<'s, 'h>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    options: HammerCompilationOptions,
    typing_bump: &'t Bump,
  ) -> Self {
    let instantiator_options = InstantiatorCompilationOptions {
      debug_out: options.debug_out.clone(),
    };
    let instantiated_compilation =
        InstantiatedCompilation::new(
          scout_arena,
          keywords,
          parser_keywords,
          parse_arena,
          packages_to_build.clone(),
          package_to_contents_resolver,
          options.global_options.clone(),
          instantiator_options,
          typing_bump,
        );
    HammerCompilation {
      interner,
      keywords,
      packages_to_build,
      package_to_contents_resolver,
      options,
      instantiated_compilation,
      hamuts_cache: None,
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
}
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
