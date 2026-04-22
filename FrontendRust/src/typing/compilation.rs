// From Frontend/TypingPass/src/dev/vale/typing/Compilation.scala
// Coordinates the Typing pass

use crate::compile_options::GlobalOptions;
use crate::higher_typing::HigherTypingCompilation;
use crate::instantiating::InstantiatorCompilationOptions;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;

/*
package dev.vale.typing

import dev.vale.highertyping.{HigherTypingCompilation, ICompileErrorA, ProgramA}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.postparsing._
import dev.vale.{Err, FileCoordinateMap, IPackageResolver, Ok, PackageCoordinate, PackageCoordinateMap, Result, vcurious, vfail}
import dev.vale._
import dev.vale.highertyping._
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.postparsing.ICompileErrorS

import scala.collection.immutable.{List, ListMap, Map, Set}
import scala.collection.mutable
*/
/// Miscellaneous (see @TFITCX)
pub struct TypingPassOptions<'s> {
  pub global_options: GlobalOptions,
  pub debug_out: Arc<dyn Fn(&str) + Send + Sync>,
  pub tree_shaking_enabled: bool,
  pub _phantom: std::marker::PhantomData<&'s ()>,
}
/*
case class TypingPassOptions(
  globalOptions: GlobalOptions = GlobalOptions(),
  debugOut: (=> String) => Unit = DefaultPrintyThing.print,
  treeShakingEnabled: Boolean = true
) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/

pub fn run_typing_pass<'s, 'ctx, 't>(
    scout_arena: &'ctx ScoutArena<'s>,
    typing_interner: &'ctx crate::typing::typing_interner::TypingInterner<'s, 't>,
    keywords: &'ctx Keywords<'s>,
    opts: &'ctx TypingPassOptions<'s>,
    program_a: &'s crate::higher_typing::ast::ProgramA<'s>,
) -> Result<
    crate::typing::hinputs_t::HinputsT<'s, 't>,
    crate::typing::compiler_error_reporter::ICompileErrorT<'s, 't>,
>
where 's: 't,
{
    panic!("Unimplemented: run_typing_pass — Slab 8");
}

/// Miscellaneous (see @TFITCX)
pub struct TypingPassCompilation<'s, 'ctx, 't, 'p> {
  higher_typing_compilation: HigherTypingCompilation<'s, 'ctx, 'p>,
  hinputs_cache: Option<()>,
  _phantom: std::marker::PhantomData<&'t ()>,
}
/*
class TypingPassCompilation(
  val interner: Interner,
  val keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: TypingPassOptions = TypingPassOptions()) {
  var higherTypingCompilation =
    new HigherTypingCompilation(
      options.globalOptions, interner, keywords, packagesToBuild, packageToContentsResolver)
  var hinputsCache: Option[HinputsT] = None
*/
impl<'s, 'ctx, 't, 'p> TypingPassCompilation<'s, 'ctx, 't, 'p>
where
{
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx crate::parse_arena::ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    global_options: GlobalOptions,
    instantiator_options: InstantiatorCompilationOptions,
  ) -> Self {
    let typing_options = TypingPassOptions {
      global_options,
      debug_out: instantiator_options.debug_out.clone(),
      tree_shaking_enabled: true,
      _phantom: std::marker::PhantomData,
    };

    let higher_typing_compilation = HigherTypingCompilation::new(
      scout_arena,
      keywords,
      parser_keywords,
      parse_arena,
      packages_to_build,
      package_to_contents_resolver,
      typing_options.global_options,
    );

    TypingPassCompilation {
      higher_typing_compilation,
      hinputs_cache: None,
      _phantom: std::marker::PhantomData,
    }
  }
pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
  self.higher_typing_compilation.get_code_map()
}
/*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = higherTypingCompilation.getCodeMap()
*/
pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
  self.higher_typing_compilation.get_parseds()
}
/*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = higherTypingCompilation.getParseds()
*/
pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
  self.higher_typing_compilation.get_vpst_map()
}
/*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = higherTypingCompilation.getVpstMap()
*/
pub fn get_scoutput(&mut self) -> Result<(), String> {
  panic!("TypingPassCompilation.get_scoutput not yet implemented - see Compilation.scala line 36")
}
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = higherTypingCompilation.getScoutput()
*/
pub fn get_astrouts(&mut self) -> Result<(), String> {
  panic!("TypingPassCompilation.get_astrouts not yet implemented - see Compilation.scala line 38")
}
/*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = higherTypingCompilation.getAstrouts()
*/
pub fn get_compiler_outputs(&mut self) -> Result<(), String> {
  panic!("TypingPassCompilation.get_compiler_outputs not yet implemented - see Compilation.scala lines 40-58")
}
/*
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = {
    hinputsCache match {
      case Some(coutputs) => Ok(coutputs)
      case None => {
        val compiler =
          new Compiler(
            options,
            interner,
            keywords)
        compiler.evaluate(getCodeMap().getOrDie(), higherTypingCompilation.expectAstrouts()) match {
          case Err(e) => Err(e)
          case Ok(hinputs) => {
            hinputsCache = Some(hinputs)
            Ok(hinputs)
          }
        }
      }
    }
  }
*/
pub fn expect_compiler_outputs(&mut self) -> () {
  panic!("TypingPassCompilation.expect_compiler_outputs not yet implemented - see Compilation.scala lines 60-77")
}
/*
  def expectCompilerOutputs(): HinputsT = {
    getCompilerOutputs() match {
      case Err(err) => {

        val codeMap = getCodeMap().getOrDie()
        val errorText =
          CompilerErrorHumanizer.humanize(
            true,
            SourceCodeUtils.humanizePos(codeMap, _),
            SourceCodeUtils.linesBetween(codeMap, _, _),
            SourceCodeUtils.lineRangeContaining(codeMap, _),
            SourceCodeUtils.lineContaining(codeMap, _),
            err)
        vfail(errorText)
      }
      case Ok(x) => x
    }
  }
}

*/
}
