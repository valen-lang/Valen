// From Frontend/InstantiatingPass/src/dev/vale/instantiating/InstantiatedCompilation.scala
// Coordinates the Instantiating pass

use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::simplifying::HammerCompilationOptions;
use crate::typing::TypingPassCompilation;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;

// From InstantiatedCompilation.scala lines 12-17: InstantiatorCompilationOptions
pub struct InstantiatorCompilationOptions {
  pub debug_out: Arc<dyn Fn(&str) + Send + Sync>,
}

// From InstantiatedCompilation.scala lines 19-56: InstantiatedCompilation class
pub struct InstantiatedCompilation<'a> {
  typing_pass_compilation: TypingPassCompilation<'a>,
  #[allow(dead_code)]
  monouts_cache: Option<()>, // HinputsI not yet ported
}

impl<'a> InstantiatedCompilation<'a> {
  // From InstantiatedCompilation.scala lines 19-34
  pub fn new(
    interner: &'a Interner<'a>,
    keywords: &'a Keywords<'a>,
    packages_to_build: Vec<&'a PackageCoordinate<'a>>,
    package_to_contents_resolver: &'a dyn IPackageResolver<'a, HashMap<String, String>>,
    options: HammerCompilationOptions,
  ) -> Self {
    let typing_options = InstantiatorCompilationOptions {
      debug_out: options.debug_out.clone(),
    };

    let typing_pass_compilation = TypingPassCompilation::new(
      interner,
      keywords,
      packages_to_build,
      package_to_contents_resolver,
      options.global_options,
      typing_options,
    );

    InstantiatedCompilation {
      typing_pass_compilation,
      monouts_cache: None,
    }
  }

  // From InstantiatedCompilation.scala line 36: getCodeMap
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse> {
    self.typing_pass_compilation.get_code_map()
  }

  // From InstantiatedCompilation.scala line 37: getParseds
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'a, (FileP, Vec<RangeL>)>, FailedParse> {
    self.typing_pass_compilation.get_parseds()
  }

  // From InstantiatedCompilation.scala line 38: getVpstMap
  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse> {
    self.typing_pass_compilation.get_vpst_map()
  }

  // From InstantiatedCompilation.scala line 39: getScoutput
  pub fn get_scoutput(&mut self) -> Result<(), String> {
    panic!("InstantiatedCompilation.get_scoutput not yet implemented - see InstantiatedCompilation.scala line 39")
  }

  // From InstantiatedCompilation.scala line 40: getAstrouts
  pub fn get_astrouts(&mut self) -> Result<(), String> {
    panic!("InstantiatedCompilation.get_astrouts not yet implemented - see InstantiatedCompilation.scala line 40")
  }

  // From InstantiatedCompilation.scala line 41: getCompilerOutputs
  pub fn get_compiler_outputs(&mut self) -> Result<(), String> {
    panic!("InstantiatedCompilation.get_compiler_outputs not yet implemented - see InstantiatedCompilation.scala line 41")
  }

  // From InstantiatedCompilation.scala line 42: expectCompilerOutputs
  pub fn expect_compiler_outputs(&mut self) -> () {
    panic!("InstantiatedCompilation.expect_compiler_outputs not yet implemented - see InstantiatedCompilation.scala line 42")
  }

  // From InstantiatedCompilation.scala lines 44-55: getMonouts
  pub fn get_monouts(&mut self) -> () {
    panic!("InstantiatedCompilation.get_monouts not yet implemented - see InstantiatedCompilation.scala lines 44-55")
  }
}

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

case class InstantiatorCompilationOptions(
  globalOptions: GlobalOptions = GlobalOptions(),
  debugOut: (=> String) => Unit = (x => {
    println("##: " + x)
  })
) { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }

class InstantiatedCompilation(
  val interner: Interner,
  val keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]],
  options: InstantiatorCompilationOptions = InstantiatorCompilationOptions()) {
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

  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = typingPassCompilation.getCodeMap()
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = typingPassCompilation.getParseds()
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = typingPassCompilation.getVpstMap()
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = typingPassCompilation.getScoutput()
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = typingPassCompilation.getAstrouts()
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = typingPassCompilation.getCompilerOutputs()
  def expectCompilerOutputs(): HinputsT = typingPassCompilation.expectCompilerOutputs()

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
}

*/
