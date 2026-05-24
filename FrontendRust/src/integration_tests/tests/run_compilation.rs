/*
package dev.vale

import dev.vale.finalast.ProgramH
import dev.vale.highertyping.{ICompileErrorA, ProgramA}
import dev.vale.instantiating.ast.HinputsI
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.passmanager.{FullCompilation, FullCompilationOptions}
import dev.vale.postparsing.{ICompileErrorS, ProgramS}
import dev.vale.testvm.{Heap, PrimitiveKindV, ReferenceV, Vivem}
import dev.vale.typing.{HinputsT, ICompileErrorT}
import dev.vale.von.IVonData

object RunCompilation {
*/
// mig: fn test
pub fn test(code: &str, include_all_builtins: bool) -> RunCompilation { panic!("Unimplemented: test"); }
/*
  def test(code: String, includeAllBuiltins: Boolean = true): RunCompilation = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    new RunCompilation(
      interner,
      keywords,
      (if (includeAllBuiltins) {
        Vector(PackageCoordinate.BUILTIN(interner, keywords))
      } else {
        Vector()
      }) ++
          Vector(
            PackageCoordinate.TEST_TLD(interner, keywords)),
      (if (includeAllBuiltins) {
        Builtins.getCodeMap(interner, keywords)
      } else {
        Builtins.getModulizedCodeMap(interner, keywords)
      })
          .or(FileCoordinateMap.test(interner, Vector(code)))
          .or(Tests.getPackageToResourceResolver),
      FullCompilationOptions(GlobalOptions(true, true, true, true, true)))
  }
}
*/

// mig: struct RunCompilation
pub struct RunCompilation;
/*
class RunCompilation(
    val interner: Interner,
    val keywords: Keywords,
    packagesToBuild: Vector[PackageCoordinate],
    packageToContentsResolver: IPackageResolver[Map[String, String]],
    options: FullCompilationOptions = FullCompilationOptions()) {
  var fullCompilation = new FullCompilation(interner, keywords, packagesToBuild, packageToContentsResolver, options)
*/
impl RunCompilation {
  // mig: fn get_code_map
  pub fn get_code_map(&self) { panic!("Unimplemented: get_code_map"); }
  /*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = fullCompilation.getCodeMap()
  */

  // mig: fn get_parseds
  pub fn get_parseds(&self) { panic!("Unimplemented: get_parseds"); }
  /*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = fullCompilation.getParseds()
  */

  // mig: fn get_vpst_map
  pub fn get_vpst_map(&self) { panic!("Unimplemented: get_vpst_map"); }
  /*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = fullCompilation.getVpstMap()
  */

  // mig: fn get_scoutput
  pub fn get_scoutput(&self) { panic!("Unimplemented: get_scoutput"); }
  /*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = fullCompilation.getScoutput()
  */

  // mig: fn get_astrouts
  pub fn get_astrouts(&self) { panic!("Unimplemented: get_astrouts"); }
  /*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = fullCompilation.getAstrouts()
  */

  // mig: fn get_compiler_outputs
  pub fn get_compiler_outputs(&self) { panic!("Unimplemented: get_compiler_outputs"); }
  /*
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = fullCompilation.getCompilerOutputs()
  */

  // mig: fn expect_compiler_outputs
  pub fn expect_compiler_outputs(&self) { panic!("Unimplemented: expect_compiler_outputs"); }
  /*
  def expectCompilerOutputs(): HinputsT = fullCompilation.expectCompilerOutputs()
  */

  // mig: fn get_monouts
  pub fn get_monouts(&self) { panic!("Unimplemented: get_monouts"); }
  /*
  def getMonouts(): HinputsI = fullCompilation.getMonouts()
  */

  // mig: fn get_hamuts
  pub fn get_hamuts(&self) { panic!("Unimplemented: get_hamuts"); }
  /*
  def getHamuts(): ProgramH = {
    val hamuts = fullCompilation.getHamuts()
    fullCompilation.getVonHammer().vonifyProgram(hamuts)
    hamuts
  }
  */

  // The following methods drive the reference backend (Vivem/Heap/ReferenceV/
  // PrimitiveKindV), which has not yet been migrated to Rust. They are also Scala
  // overloads (evalForKind ×3, run ×2) that cannot share Rust names, so they remain
  // Scala-only here until the backend is migrated.
  /*
  def evalForKind(heap: Heap, args: Vector[ReferenceV]): IVonData = {
    Vivem.executeWithHeap(getHamuts(), heap, args, System.out, Vivem.emptyStdin, Vivem.regularStdout)
  }
  def run(heap: Heap, args: Vector[ReferenceV]): Unit = {
    Vivem.executeWithHeap(getHamuts(), heap, args, System.out, Vivem.emptyStdin, Vivem.regularStdout)
  }
  def run(args: Vector[PrimitiveKindV]): Unit = {
    Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.emptyStdin, Vivem.regularStdout)
  }
  def evalForKind(args: Vector[PrimitiveKindV]): IVonData = {
    Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.emptyStdin, Vivem.regularStdout)
  }
  def evalForKind(
      args: Vector[PrimitiveKindV],
      stdin: Vector[String]):
  IVonData = {
    Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.stdinFromList(stdin), Vivem.regularStdout)
  }
  def evalForStdout(args: Vector[PrimitiveKindV]): String = {
    val (stdoutStringBuilder, stdoutFunc) = Vivem.stdoutCollector()
    Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.emptyStdin, stdoutFunc)
    stdoutStringBuilder.mkString
  }
  def evalForKindAndStdout(args: Vector[PrimitiveKindV]): (IVonData, String) = {
    val (stdoutStringBuilder, stdoutFunc) = Vivem.stdoutCollector()
    val kind = Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.emptyStdin, stdoutFunc)
    (kind, stdoutStringBuilder.mkString)
  }
}
*/
}
