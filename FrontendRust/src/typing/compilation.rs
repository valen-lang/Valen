// From Frontend/TypingPass/src/dev/vale/typing/Compilation.scala
// Coordinates the Typing pass

use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::higher_typing::HigherTypingCompilation;
use crate::instantiating::InstantiatorCompilationOptions;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;
use crate::parse_arena::ParseArena;

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

/// Miscellaneous (see @TFITCX)
pub struct TypingPassCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{
  higher_typing_compilation: HigherTypingCompilation<'s, 'ctx, 'p>,
  hinputs_cache: Option<HinputsT<'s, 't>>,
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  options: TypingPassOptions<'s>,
  pub typing_interner: TypingInterner<'s, 't>,
}
/*
class TypingPassCompilation(
  val interner: Interner,
  val keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: TypingPassOptions = TypingPassOptions()) {
*/
impl<'s, 'ctx, 't, 'p> TypingPassCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    global_options: GlobalOptions,
    instantiator_options: InstantiatorCompilationOptions,
    typing_bump: &'t Bump,
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
      typing_options.global_options.clone(),
    );

    let typing_interner = TypingInterner::new(typing_bump);

    TypingPassCompilation {
      higher_typing_compilation,
      hinputs_cache: None,
      scout_arena,
      keywords,
      options: typing_options,
      typing_interner,
    }
  }
  /*
    var higherTypingCompilation =
      new HigherTypingCompilation(
        options.globalOptions, interner, keywords, packagesToBuild, packageToContentsResolver)
    var hinputsCache: Option[HinputsT] = None
  */

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
pub fn get_compiler_outputs(&mut self) -> Result<&HinputsT<'s, 't>, ICompileErrorT<'s, 't>> {
  if self.hinputs_cache.is_some() {
    return Ok(self.hinputs_cache.as_ref().unwrap());
  }
  let code_map = self.get_code_map().expect("getCodeMap failed");
  let astrouts = self.higher_typing_compilation.expect_astrouts();
  let compiler = Compiler::new(self.scout_arena, &self.typing_interner, self.keywords, &self.options);
  match compiler.evaluate(&code_map, astrouts) {
    Err(e) => Err(e),
    Ok(hinputs) => {
      self.hinputs_cache = Some(hinputs);
      Ok(self.hinputs_cache.as_ref().unwrap())
    }
  }
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
pub fn expect_compiler_outputs(&mut self) -> &HinputsT<'s, 't> {
/*
  def expectCompilerOutputs(): HinputsT = {
    getCompilerOutputs() match {
*/
  match self.get_compiler_outputs() {
/*
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
*/
    Err(_err) => panic!("Not yet implemented: CompilerErrorHumanizer.humanize: {:?}", _err),
/*
      case Ok(x) => x
    }
  }
}
*/
    Ok(x) => x,
  }
}
  /*
  */
  // Rust adaptation: `&self` read of the already-computed compiler outputs, so a caller can borrow it
  // alongside another field of this struct in one expression (`&mut expect_compiler_outputs` would
  // conflict). Caller must have run `expect_compiler_outputs` first.
  pub fn cached_compiler_outputs(&self) -> &HinputsT<'s, 't> {
    self.hinputs_cache.as_ref().expect("compiler outputs not computed")
  }
  /* Guardian: disable-all */
}
