// From Frontend/TypingPass/src/dev/vale/typing/Compilation.scala
// Coordinates the Typing pass

use crate::compile_options::GlobalOptions;
use crate::higher_typing::HigherTypingCompilation;
use crate::instantiating::InstantiatorCompilationOptions;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;

// From Compilation.scala lines 16-20: TypingPassOptions
pub struct TypingPassOptions {
  pub global_options: GlobalOptions,
  pub debug_out: Arc<dyn Fn(&str) + Send + Sync>,
  pub tree_shaking_enabled: bool,
}

// From Compilation.scala lines 22-78: TypingPassCompilation class
pub struct TypingPassCompilation {
  higher_typing_compilation: HigherTypingCompilation,
  #[allow(dead_code)]
  hinputs_cache: Option<()>, // HinputsT not yet ported
}

impl TypingPassCompilation {
  // From Compilation.scala lines 22-30
  pub fn new(
    interner: Arc<Interner>,
    keywords: Arc<Keywords>,
    packages_to_build: Vec<Arc<PackageCoordinate>>,
    package_to_contents_resolver: Arc<dyn IPackageResolver<HashMap<String, String>>>,
    global_options: GlobalOptions,
    instantiator_options: InstantiatorCompilationOptions,
  ) -> Self {
    let typing_options = TypingPassOptions {
      global_options,
      debug_out: instantiator_options.debug_out.clone(),
      tree_shaking_enabled: true,
    };

    let higher_typing_compilation = HigherTypingCompilation::new(
      interner,
      keywords,
      packages_to_build,
      package_to_contents_resolver,
      typing_options.global_options,
    );

    TypingPassCompilation {
      higher_typing_compilation,
      hinputs_cache: None,
    }
  }

  // From Compilation.scala line 33: getCodeMap
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<String>, FailedParse> {
    self.higher_typing_compilation.get_code_map()
  }

  // From Compilation.scala line 34: getParseds
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<(FileP, Vec<RangeL>)>, FailedParse> {
    self.higher_typing_compilation.get_parseds()
  }

  // From Compilation.scala line 35: getVpstMap
  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<String>, FailedParse> {
    self.higher_typing_compilation.get_vpst_map()
  }

  // From Compilation.scala line 36: getScoutput
  pub fn get_scoutput(&mut self) -> Result<(), String> {
    panic!("TypingPassCompilation.get_scoutput not yet implemented - see Compilation.scala line 36")
  }

  // From Compilation.scala line 38: getAstrouts
  pub fn get_astrouts(&mut self) -> Result<(), String> {
    panic!("TypingPassCompilation.get_astrouts not yet implemented - see Compilation.scala line 38")
  }

  // From Compilation.scala lines 40-58: getCompilerOutputs
  pub fn get_compiler_outputs(&mut self) -> Result<(), String> {
    panic!("TypingPassCompilation.get_compiler_outputs not yet implemented - see Compilation.scala lines 40-58")
  }

  // From Compilation.scala lines 60-77: expectCompilerOutputs
  pub fn expect_compiler_outputs(&mut self) -> () {
    panic!("TypingPassCompilation.expect_compiler_outputs not yet implemented - see Compilation.scala lines 60-77")
  }
}

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

case class TypingPassOptions(
  globalOptions: GlobalOptions = GlobalOptions(),
  debugOut: (=> String) => Unit = DefaultPrintyThing.print,
  treeShakingEnabled: Boolean = true
) { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }

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

  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = higherTypingCompilation.getCodeMap()
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = higherTypingCompilation.getParseds()
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = higherTypingCompilation.getVpstMap()
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = higherTypingCompilation.getScoutput()

  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = higherTypingCompilation.getAstrouts()

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
