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
fn equals(&self, obj: &dyn std::any::Any) -> bool {
  panic!("Unimplemented: equals");
}
}
/*
override def equals(obj: Any): Boolean = vcurious(); }

*/

// mig: struct InstantiatedCompilation
pub struct InstantiatedCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{
  typing_pass_compilation: TypingPassCompilation<'s, 'ctx, 't, 'p>,
  #[allow(dead_code)]
  monouts_cache: Option<()>, // HinputsI not yet ported
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
impl<'s, 'ctx, 't, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'p>
where
    's: 't,
    'p: 'ctx,
{
  // From InstantiatedCompilation.scala lines 19-34
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    global_options: crate::compile_options::GlobalOptions,
    options: InstantiatorCompilationOptions,
    typing_bump: &'t Bump,
  ) -> Self {
    let typing_options = InstantiatorCompilationOptions {
      debug_out: options.debug_out.clone(),
    };

    let typing_pass_compilation = TypingPassCompilation::new(
      scout_arena,
      keywords,
      parser_keywords,
      parse_arena,
      packages_to_build,
      package_to_contents_resolver,
      global_options,
      typing_options,
      typing_bump,
    );

    InstantiatedCompilation {
      typing_pass_compilation,
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
impl<'s, 'ctx, 't, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'p>
where
    's: 't,
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
impl<'s, 'ctx, 't, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'p>
where
    's: 't,
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
impl<'s, 'ctx, 't, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'p>
where
    's: 't,
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
impl<'s, 'ctx, 't, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'p>
where
    's: 't,
    'p: 'ctx,
{
  pub fn get_scoutput(&mut self) -> Result<(), String> {
    panic!("InstantiatedCompilation.get_scoutput not yet implemented - see InstantiatedCompilation.scala line 39")
  }
}
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = typingPassCompilation.getScoutput()
*/
// mig: fn get_astrouts
impl<'s, 'ctx, 't, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'p>
where
    's: 't,
    'p: 'ctx,
{
  pub fn get_astrouts(&mut self) -> Result<(), String> {
    panic!("InstantiatedCompilation.get_astrouts not yet implemented - see InstantiatedCompilation.scala line 40")
  }
}
/*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = typingPassCompilation.getAstrouts()
*/
// mig: fn get_compiler_outputs
impl<'s, 'ctx, 't, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'p>
where
    's: 't,
    'p: 'ctx,
{
  pub fn get_compiler_outputs(&mut self) -> Result<(), String> {
    panic!("InstantiatedCompilation.get_compiler_outputs not yet implemented - see InstantiatedCompilation.scala line 41")
  }
}
/*
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = typingPassCompilation.getCompilerOutputs()
*/
// mig: fn expect_compiler_outputs
impl<'s, 'ctx, 't, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'p>
where
    's: 't,
    'p: 'ctx,
{
  pub fn expect_compiler_outputs(&mut self) -> () {
    panic!("InstantiatedCompilation.expect_compiler_outputs not yet implemented - see InstantiatedCompilation.scala line 42")
  }
}
/*
  def expectCompilerOutputs(): HinputsT = typingPassCompilation.expectCompilerOutputs()

*/
// mig: fn get_monouts
impl<'s, 'ctx, 't, 'p> InstantiatedCompilation<'s, 'ctx, 't, 'p>
where
    's: 't,
    'p: 'ctx,
{
  // Phase E (Slab 16j) signature: returns HinputsI<'s, 'i>.
  // 'i tied to 's by the bare-placeholder shape; instantiating arena is
  // managed by the InstantiatedCompilation itself (not yet wired — Slab 16j
  // sets the type-signature handoff only, body stays panic).
  pub fn get_monouts<'i>(&mut self) -> crate::instantiating::ast::hinputs::HinputsI<'s, 'i>
  where 's: 'i {
    panic!("InstantiatedCompilation.get_monouts not yet implemented - see InstantiatedCompilation.scala lines 44-55")
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