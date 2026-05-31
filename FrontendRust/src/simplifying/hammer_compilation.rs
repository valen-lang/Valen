// From Frontend/SimplifyingPass/src/dev/vale/simplifying/HammerCompilation.scala
// Coordinates the Hammer (simplifying) pass

use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::simplifying::hammer::Hammer;
use crate::instantiating::instantiated_compilation::{InstantiatedCompilation, InstantiatorCompilationOptions};
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
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
pub struct HammerCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 'i,
{
  pub interner: &'ctx HammerInterner<'s, 'h>,
  pub keywords: &'ctx Keywords<'s>,
  pub scout_arena: &'ctx ScoutArena<'s>,
  pub packages_to_build: Vec<&'ctx PackageCoordinate<'p>>,
  pub package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
  pub options: HammerCompilationOptions,
  pub instantiated_compilation: crate::instantiating::instantiated_compilation::InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>,
  pub hamuts_cache: Option<&'h crate::final_ast::ast::ProgramH<'s, 'h>>,
  // Scala has `var vonHammerCache: Option[VonHammer]`. Dropped: per
  // typing-pass precedent the VonHammer compiler class was collapsed
  // onto `Hammer` (no separate VonHammer state), so there is nothing
  // to cache.
}
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
// mig: impl HammerCompilation
// mig: fn new
impl<'s, 'h, 'ctx, 't, 'i, 'p> HammerCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    interner: &'ctx HammerInterner<'s, 'h>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    options: HammerCompilationOptions,
    typing_bump: &'t Bump,
    instantiating_bump: &'i Bump,
  ) -> Self {
    let instantiated_compilation =
      InstantiatedCompilation::new(
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        packages_to_build.clone(),
        package_to_contents_resolver,
        options.global_options.clone(),
        InstantiatorCompilationOptions {
          debug_out: options.debug_out.clone(),
        },
        typing_bump,
        instantiating_bump);
    HammerCompilation {
      interner,
      keywords,
      scout_arena,
      packages_to_build,
      package_to_contents_resolver,
      options,
      instantiated_compilation,
      hamuts_cache: None,
    }
  }
}
/*
Guardian: temp-disable: SPDMX — The HammerCompilation struct (hammer_compilation.rs:67-71) explicitly documents dropping vonHammerCache per typing-pass precedent (VonHammer collapsed onto Hammer, nothing to cache); the field does not exist in the Rust struct by design, so the struct literal correctly omits it. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-643-1779915911666/hook-643/new--82.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
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
impl<'s, 'h, 'ctx, 't, 'i, 'p> HammerCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 'i,
{
  pub fn get_hamuts(&mut self) -> &'h crate::final_ast::ast::ProgramH<'s, 'h> {
    match self.hamuts_cache {
      Some(hamuts) => hamuts,
      None => {
        self.instantiated_compilation.get_monouts();
        let hinputs = self.instantiated_compilation.cached_monouts();
        let hammer = Hammer { interner: self.interner, keywords: self.keywords, scout_arena: self.scout_arena, instantiating_interner: &self.instantiated_compilation.instantiating_interner };
        let hamuts = hammer.translate(hinputs);
        self.hamuts_cache = Some(hamuts);
        hamuts
      }
    }
  }
}
/*
Guardian: temp-disable: SPDMX — The Scala `vonHammerCache = Some(hammer.vonHammer)` line is intentionally dropped because the VonHammer compiler class was collapsed onto Hammer (no separate VonHammer state), so the field does not exist in the Rust HammerCompilation struct by design — this is the same in-file precedent established by the temp-disable on `new` (hammer_compilation.rs:118) which drops the corresponding `var vonHammerCache` field. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-699-1779918117497/hook-699/get_hamuts--215.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
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
