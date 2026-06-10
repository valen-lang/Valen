// From Frontend/InstantiatingPass/src/dev/vale/instantiating/InstantiatedCompilation.scala
// Coordinates the Instantiating pass

use bumpalo::Bump;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
// Simplifying pass unlinked during instantiating bring-up (Slabs 16a–16j).
// use crate::simplifying::HammerCompilationOptions;
use crate::typing::TypingPassCompilation;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;
use crate::parse_arena::ParseArena;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::instantiator;
use crate::compile_options::GlobalOptions;
use crate::postparsing::ast::ProgramS;
use crate::postparsing::post_parser::ICompileErrorS;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::typing_interner::TypingInterner;
use std::any::Any;


/*
package dev.vale.instantiating

import dev.vale.highertyping.{ICompileErrorA, ProgramA}
import dev.vale.instantiating.ast.HinputsI
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.postparsing.{ICompileErrorS, ProgramS}
import dev.vale.{FileCoordinateMap, IPackageResolver, Interner, Keywords, PackageCoordinate, PackageCoordinateMap, Result, vassertSome, vcurious}
import dev.vale.typing.{HinputsT, ICompileErrorT, TypingPassCompilation, TypingPassOptions}

*/
// mig: struct InstantiatorCompilationOptions
pub struct InstantiatorCompilationOptions {
  pub debug_out: Arc<dyn Fn(&str) + Send + Sync>,
}
// mig: impl InstantiatorCompilationOptions
/*
case class InstantiatorCompilationOptions(
  globalOptions: GlobalOptions = GlobalOptions(),
  debugOut: (=> String) => Unit = (x => {
    println("##: " + x)
  })
) {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code
impl InstantiatorCompilationOptions {
fn hash_code(&self) -> i32 {
  panic!("Unimplemented: hash_code");
}
}
/*
override def hashCode(): Int = hash;
*/
// mig: fn equals
impl InstantiatorCompilationOptions {
fn equals(&self, obj: &dyn Any) -> bool {
  panic!("Unimplemented: equals");
}
}
/*
override def equals(obj: Any): Boolean = vcurious(); }

*/

// mig: struct InstantiatedCompilation
pub struct InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i,
{
  pub typing_pass_compilation: TypingPassCompilation<'s, 'ctx, 't, 'p>,
  // Retained from `new` to feed `Instantiator::translate` in `get_monouts`
  // (Scala's `val keywords` + `options` class fields).
  keywords: &'ctx Keywords<'s>,
  global_options: GlobalOptions,
  // The instantiating arena's interner, built from the externally-owned 'i Bump
  // passed to `new` — mirrors TypingPassCompilation's `typing_interner` (built
  // from `typing_bump: &'t Bump`).
  pub instantiating_interner: InstantiatingInterner<'s, 'i>,
  monouts_cache: Option<HinputsI<'s, 'i>>,
}
/*
class InstantiatedCompilation(
  val interner: Interner,
  val keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: InstantiatorCompilationOptions = InstantiatorCompilationOptions()) {
 */

// mig: impl InstantiatedCompilation
// mig: fn new
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  // From InstantiatedCompilation.scala lines 19-34
  // Rust adaptation (SPDMX Exception B): `typing_interner` borrowed from test wrapper, threaded down to TypingPassCompilation (mirrors Scala `val interner` flowing from RunCompilation through the pipeline).
  pub fn new(
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    global_options: GlobalOptions,
    options: InstantiatorCompilationOptions,
    instantiating_bump: &'i Bump,
  ) -> Self {
    let typing_options = InstantiatorCompilationOptions {
      debug_out: options.debug_out.clone(),
    };

    let typing_pass_compilation = TypingPassCompilation::new(
      typing_interner,
      scout_arena,
      keywords,
      parser_keywords,
      parse_arena,
      packages_to_build,
      package_to_contents_resolver,
      global_options.clone(),
      typing_options,
    );

    let instantiating_interner = InstantiatingInterner::new(instantiating_bump);

    InstantiatedCompilation {
      typing_pass_compilation,
      keywords,
      global_options,
      instantiating_interner,
      monouts_cache: None,
    }
  }
}
/*
  var typingPassCompilation =
    new TypingPassCompilation(
      interner,
      keywords,
      packagesToBuild,
      packageToContentsResolver,
      TypingPassOptions(
        options.globalOptions,
        options.debugOut))
  var monoutsCache: Option[HinputsI] = None
*/
// mig: fn get_code_map
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.typing_pass_compilation.get_code_map()
  }
}
/*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = typingPassCompilation.getCodeMap()
*/
// mig: fn get_parseds
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
    self.typing_pass_compilation.get_parseds()
  }
}
/*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = typingPassCompilation.getParseds()
*/
// mig: fn get_vpst_map
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.typing_pass_compilation.get_vpst_map()
  }
}
/*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = typingPassCompilation.getVpstMap()
*/
// mig: fn get_scoutput
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  pub fn get_scoutput(&mut self) -> Result<&FileCoordinateMap<'s, ProgramS<'s>>, ICompileErrorS<'s>> {
    self.typing_pass_compilation.get_scoutput()
  }
}
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = typingPassCompilation.getScoutput()
*/
// mig: fn get_astrouts
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  pub fn get_astrouts(&mut self) -> Result<&crate::utils::code_hierarchy::PackageCoordinateMap<'s, crate::higher_typing::ast::ProgramA<'s>>, crate::higher_typing::astronomer_error_reporter::ICompileErrorA<'s>> {
    self.typing_pass_compilation.get_astrouts()
  }
}
/*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = typingPassCompilation.getAstrouts()
*/
// mig: fn get_compiler_outputs
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  pub fn get_compiler_outputs(&mut self) -> Result<&HinputsT<'s, 't>, ICompileErrorT<'s, 't>> {
    self.typing_pass_compilation.get_compiler_outputs()
  }
}
/*
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = typingPassCompilation.getCompilerOutputs()
*/
// mig: fn expect_compiler_outputs
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  pub fn expect_compiler_outputs(&mut self) -> &HinputsT<'s, 't> {
    self.typing_pass_compilation.expect_compiler_outputs()
  }
}
/*
  def expectCompilerOutputs(): HinputsT = typingPassCompilation.expectCompilerOutputs()

*/
// mig: fn get_monouts
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  // Returns HinputsI<'s, 'i>, where 'i is the InstantiatedCompilation's own
  // instantiating-arena lifetime (the `instantiating_interner` field), mirroring
  // how TypingPassCompilation::get_compiler_outputs returns HinputsT<'s, 't>.
  pub fn get_monouts(&mut self) -> &HinputsI<'s, 'i> {
    if self.monouts_cache.is_some() {
      return self.monouts_cache.as_ref().unwrap();
    }
    // Populate the typing-pass output cache (the `&mut` borrow ends here), so the two reads below —
    // the typing_interner and the cached outputs, both fields of typing_pass_compilation — can coexist.
    self.typing_pass_compilation.expect_compiler_outputs();
    let monouts =
      instantiator::translate(
        &self.global_options, &self.instantiating_interner, &self.typing_pass_compilation.typing_interner, self.keywords, self.typing_pass_compilation.cached_compiler_outputs());
    self.monouts_cache = Some(monouts);
    self.monouts_cache.as_ref().unwrap()
  }
}
/*
  def getMonouts(): HinputsI = {
    monoutsCache match {
      case Some(monouts) => monouts
      case None => {
        val monouts =
          Instantiator.translate(
            options.globalOptions, interner, keywords, typingPassCompilation.expectCompilerOutputs())
        monoutsCache = Some(monouts)
        monouts
      }
    }
  }
*/
/*
}
*/
impl<'s, 'ctx, 't, 'i, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where
    's: 't,
    's: 'i,
    'p: 'ctx,
{
  /*
  */
  // Rust adaptation: `&self` read of the already-computed monouts so a caller can borrow it
  // alongside another field of this struct in one expression (`&mut get_monouts` would
  // conflict). Caller must have run `get_monouts` first to populate the cache.
  pub fn cached_monouts(&self) -> &HinputsI<'s, 'i> {
    self.monouts_cache.as_ref().expect("monouts not computed")
  }
  /* Guardian: disable-all */
}