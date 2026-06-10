use crate::utils::code_hierarchy::IPackageResolver;
use crate::builtins::builtins::get_code_map;
use crate::builtins::builtins::get_modulized_code_map;
use crate::compile_options::GlobalOptions;
use crate::final_ast::ast::ProgramH;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parse_arena::ParseArena;
use crate::parsing::ast::FileP;
use crate::postparsing::ast::ProgramS;
use crate::postparsing::post_parser::ICompileErrorS;
use crate::scout_arena::ScoutArena;
use crate::simplifying::hammer_compilation::HammerCompilation;
use crate::simplifying::hammer_compilation::HammerCompilationOptions;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::tests::tests::get_package_to_resource_resolver;
use crate::testvm::heap::HeapV;
use crate::testvm::values::PrimitiveKindV;
use crate::testvm::values::ReferenceV;
use crate::testvm::vivem::VmRuntimeErrorV;
use crate::testvm::vivem::empty_stdin;
use crate::testvm::vivem::execute_with_heap;
use crate::testvm::vivem::execute_with_primitive_args;
use crate::testvm::vivem::regular_stdout;
use crate::testvm::vivem::stdin_from_list;
use crate::testvm::vivem::stdout_collector;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::code_hierarchy::test_from_vec;
use crate::von::ast::IVonData;
use std::collections::HashMap;
use std::io::stdout;

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
pub fn test<'s, 'h, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx bumpalo::Bump,
    interner: &'ctx HammerInterner<'s, 'h>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i bumpalo::Bump,
    code: &str,
) -> RunCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
    let packages_to_build: Vec<&'p PackageCoordinate<'p>> =
        vec![
            PackageCoordinate::builtin(parse_arena, parser_keywords),
            PackageCoordinate::test_tld(parse_arena, parser_keywords),
        ];
    let base_code_map = get_code_map(parse_arena, parser_keywords);
    let resolver_concrete = base_code_map
        .or(test_from_vec(parse_arena, vec![code.to_string()]))
        .or(get_package_to_resource_resolver());
    let resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>> =
        compilation_bump.alloc(resolver_concrete);
    let options = HammerCompilationOptions {
        global_options: GlobalOptions {
            sanity_check: true,
            use_overload_index: true,
            use_optimized_solver: true,
            verbose_errors: true,
            debug_output: true,
        },
        ..HammerCompilationOptions::new()
    };
    RunCompilation {
        interner: typing_interner,
        hammer_compilation: HammerCompilation::new(
            scout_arena, interner, typing_interner, keywords, parser_keywords, parse_arena,
            packages_to_build, resolver, options, instantiating_bump,
        ),
    }
}
/*
  def test(code: String): RunCompilation = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    new RunCompilation(
      interner,
      keywords,
      Vector(
        PackageCoordinate.BUILTIN(interner, keywords),
        PackageCoordinate.TEST_TLD(interner, keywords)),
      Builtins.getCodeMap(interner, keywords)
          .or(FileCoordinateMap.test(interner, Vector(code)))
          .or(Tests.getPackageToResourceResolver),
      FullCompilationOptions(GlobalOptions(true, true, true, true, true)))
  }
*/

// mig: fn test_no_builtins
pub fn test_no_builtins<'s, 'h, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx bumpalo::Bump,
    interner: &'ctx HammerInterner<'s, 'h>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i bumpalo::Bump,
    code: &str,
) -> RunCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
    let packages_to_build: Vec<&'p PackageCoordinate<'p>> =
        vec![
            PackageCoordinate::test_tld(parse_arena, parser_keywords),
        ];
    let base_code_map =
        get_modulized_code_map(parse_arena, parser_keywords)
            .expect("Builtins code map failed to load");
    let resolver_concrete = base_code_map
        .or(test_from_vec(parse_arena, vec![code.to_string()]))
        .or(get_package_to_resource_resolver());
    let resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>> =
        compilation_bump.alloc(resolver_concrete);
    let options = HammerCompilationOptions {
        global_options: GlobalOptions {
            sanity_check: true,
            use_overload_index: true,
            use_optimized_solver: true,
            verbose_errors: true,
            debug_output: true,
        },
        ..HammerCompilationOptions::new()
    };
    RunCompilation {
        interner: typing_interner,
        hammer_compilation: HammerCompilation::new(
            scout_arena, interner, typing_interner, keywords, parser_keywords, parse_arena,
            packages_to_build, resolver, options, instantiating_bump,
        ),
    }
}
/*
  def testNoBuiltins(code: String): RunCompilation = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    new RunCompilation(
      interner,
      keywords,
      Vector(
        PackageCoordinate.TEST_TLD(interner, keywords)),
      Builtins.getModulizedCodeMap(interner, keywords)
          .or(FileCoordinateMap.test(interner, Vector(code)))
          .or(Tests.getPackageToResourceResolver),
      FullCompilationOptions(GlobalOptions(true, true, true, true, true)))
  }
}
*/

// mig: struct RunCompilation
pub struct RunCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx,
{
    pub interner: &'ctx TypingInterner<'s, 't>,
    pub hammer_compilation: HammerCompilation<'s, 'h, 'ctx, 't, 'i, 'p>,
}
/*
class RunCompilation(
    val interner: Interner,
    val keywords: Keywords,
    packagesToBuild: Vector[PackageCoordinate],
    packageToContentsResolver: IPackageResolver[Map[String, String]],
    options: FullCompilationOptions = FullCompilationOptions()) {
  var fullCompilation = new FullCompilation(interner, keywords, packagesToBuild, packageToContentsResolver, options)
*/
impl<'s, 'h, 'ctx, 't, 'i, 'p> RunCompilation<'s, 'h, 'ctx, 't, 'i, 'p>
where 's: 'h, 's: 't, 's: 'i, 'p: 'ctx, 'ctx: 'h, 'p: 'h,
{
  // mig: fn get_code_map
  pub fn get_code_map(&self) { panic!("Unimplemented: get_code_map"); }
  /*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = fullCompilation.getCodeMap()
  */

  // mig: fn get_parseds
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
    self.hammer_compilation.get_parseds()
  }
  /*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = fullCompilation.getParseds()
  */

  // mig: fn get_vpst_map
  pub fn get_vpst_map(&self) { panic!("Unimplemented: get_vpst_map"); }
  /*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = fullCompilation.getVpstMap()
  */

  // mig: fn get_scoutput
  pub fn get_scoutput(&mut self) -> Result<&FileCoordinateMap<'s, ProgramS<'s>>, ICompileErrorS<'s>> {
      self.hammer_compilation.get_scoutput()
  }
  /*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = fullCompilation.getScoutput()
  */

  // mig: fn get_astrouts
  pub fn get_astrouts(&self) { panic!("Unimplemented: get_astrouts"); }
  /*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = fullCompilation.getAstrouts()
  */

  // mig: fn get_compiler_outputs
  pub fn get_compiler_outputs(&mut self) -> Result<&HinputsT<'s, 't>, ICompileErrorT<'s, 't>> {
      self.hammer_compilation.get_compiler_outputs()
  }
  /*
  def getCompilerOutputs(): Result[HinputsT, ICompileErrorT] = fullCompilation.getCompilerOutputs()
  */

  // mig: fn expect_compiler_outputs
  pub fn expect_compiler_outputs(&mut self) -> &HinputsT<'s, 't> {
    self.hammer_compilation.expect_compiler_outputs()
  }
  /*
  def expectCompilerOutputs(): HinputsT = fullCompilation.expectCompilerOutputs()
  */

  // mig: fn get_monouts
  pub fn get_monouts(&mut self) -> &HinputsI<'s, 'i> {
    self.hammer_compilation.get_monouts()
  }
  /*
  def getMonouts(): HinputsI = fullCompilation.getMonouts()
  */

  // mig: fn get_hamuts
  pub fn get_hamuts(&mut self) -> &'h ProgramH<'s, 'h> {
      let hamuts = self.hammer_compilation.get_hamuts();
      self.hammer_compilation.get_von_hammer().vonify_program(hamuts);
      hamuts
  }
  /*
  def getHamuts(): ProgramH = {
    val hamuts = fullCompilation.getHamuts()
    fullCompilation.getVonHammer().vonifyProgram(hamuts)
    hamuts
  }
  */

  // The following seven methods drive the reference backend (Vivem/HeapV/ReferenceV/
  // PrimitiveKindV). They're sliced as panic stubs because their bodies all delegate
  // into Vivem, which is itself entirely panic-stubbed (180 fns across the testvm module).
  //
  // Rust adaptation note: Scala has overloads `evalForKind ×3` and `run ×2` that share
  // the same name but differ by arg shape. Rust requires distinct fn names, so each
  // variant gets a suffix describing its arg shape, e.g. `_heap_args` / `_primitive_args`
  // / `_primitive_args_with_stdin`. evalForStdout and evalForKindAndStdout don't overload,
  // so they keep their Scala names verbatim (in snake_case).

  // mig: fn eval_for_kind_heap_args (Scala overload `evalForKind(heap, args: Vector[ReferenceV])`)
  pub fn eval_for_kind_heap_args<'v>(&self, _heap: HeapV<'v, 'h, 's>, _args: Vec<ReferenceV<'v, 'h, 's>>) -> IVonData { panic!("Unimplemented: eval_for_kind_heap_args"); }
  /*
  def evalForKind(heap: Heap, args: Vector[ReferenceV]): IVonData = {
    Vivem.executeWithHeap(getHamuts(), heap, args, System.out, Vivem.emptyStdin, Vivem.regularStdout)
  }
  */

  // mig: fn run_heap_args (Scala overload `run(heap, args: Vector[ReferenceV])`)
  pub fn run_heap_args<'v>(&mut self, mut heap: HeapV<'v, 'h, 's>, args: Vec<ReferenceV<'v, 'h, 's>>) -> Result<(), VmRuntimeErrorV<'s>> {
      let interner = self.hammer_compilation.interner;
      let scout_arena = self.hammer_compilation.scout_arena;
      let hamuts = self.get_hamuts();
      let input_argument_references: &'v [ReferenceV<'v, 'h, 's>] = heap.vivem_bump.alloc_slice_copy(&args);
      execute_with_heap(
          hamuts, interner, scout_arena, &mut heap, input_argument_references, &empty_stdin, &regular_stdout,
      ).map(|_| ())
  }
  /*
  def run(heap: Heap, args: Vector[ReferenceV]): Unit = {
    Vivem.executeWithHeap(getHamuts(), heap, args, System.out, Vivem.emptyStdin, Vivem.regularStdout)
  }
  */

  // mig: fn run_primitive_args (Scala overload `run(args: Vector[PrimitiveKindV])`)
  pub fn run_primitive_args<'v>(&mut self, args: Vec<PrimitiveKindV<'v, 'h, 's>>) -> Result<(), VmRuntimeErrorV<'s>> {
      let interner = self.hammer_compilation.interner;
      let scout_arena = self.hammer_compilation.scout_arena;
      let hamuts = self.get_hamuts();
      let mut vivem_dout = stdout();
      let vivem_bump = bumpalo::Bump::new();
      execute_with_primitive_args(
          hamuts, interner, scout_arena, &args, &mut vivem_dout, &vivem_bump, &empty_stdin, &regular_stdout,
      ).map(|_| ())
  }
  /*
  def run(args: Vector[PrimitiveKindV]): Unit = {
    Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.emptyStdin, Vivem.regularStdout)
  }
  */

  // mig: fn eval_for_kind_primitive_args (Scala overload `evalForKind(args: Vector[PrimitiveKindV])`)
  pub fn eval_for_kind_primitive_args<'v>(&mut self, args: Vec<PrimitiveKindV<'v, 'h, 's>>) -> Result<IVonData, VmRuntimeErrorV<'s>> {
      let interner = self.hammer_compilation.interner;
      let scout_arena = self.hammer_compilation.scout_arena;
      let hamuts = self.get_hamuts();
      let mut vivem_dout = stdout();
      let vivem_bump = bumpalo::Bump::new();
      execute_with_primitive_args(
          hamuts, interner, scout_arena, &args, &mut vivem_dout, &vivem_bump, &empty_stdin, &regular_stdout,
      )
  }
  /*
  def evalForKind(args: Vector[PrimitiveKindV]): IVonData = {
    Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.emptyStdin, Vivem.regularStdout)
  }
  */

  // mig: fn eval_for_kind_primitive_args_with_stdin (Scala overload `evalForKind(args, stdin)`)
  pub fn eval_for_kind_primitive_args_with_stdin<'v>(&mut self, args: Vec<PrimitiveKindV<'v, 'h, 's>>, stdin: Vec<String>) -> Result<IVonData, VmRuntimeErrorV<'s>> {
      let scout_arena = self.hammer_compilation.scout_arena;
      let interned_stdin: Vec<StrI<'s>> = stdin.iter().map(|s| scout_arena.intern_str(s)).collect();
      let interner = self.hammer_compilation.interner;
      let hamuts = self.get_hamuts();
      let mut vivem_dout = stdout();
      let vivem_bump = bumpalo::Bump::new();
      let stdin_fn = stdin_from_list(&interned_stdin);
      execute_with_primitive_args(
          hamuts, interner, scout_arena, &args, &mut vivem_dout, &vivem_bump, &*stdin_fn, &regular_stdout,
      )
  }
  /*
  def evalForKind(
      args: Vector[PrimitiveKindV],
      stdin: Vector[String]):
  IVonData = {
    Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.stdinFromList(stdin), Vivem.regularStdout)
  }
  */

  // mig: fn eval_for_stdout
  pub fn eval_for_stdout<'v>(&mut self, args: Vec<PrimitiveKindV<'v, 'h, 's>>) -> Result<String, VmRuntimeErrorV<'s>> {
      let (stdoutput_string_builder, stdout_func) = stdout_collector::<'s>();
      let interner = self.hammer_compilation.interner;
      let scout_arena = self.hammer_compilation.scout_arena;
      let hamuts = self.get_hamuts();
      let mut vivem_dout = stdout();
      let vivem_bump = bumpalo::Bump::new();
      execute_with_primitive_args(
          hamuts, interner, scout_arena, &args, &mut vivem_dout, &vivem_bump, &empty_stdin, &*stdout_func,
      )?;
      let result = stdoutput_string_builder.borrow().clone();
      Ok(result)
  }
  /*
  def evalForStdout(args: Vector[PrimitiveKindV]): String = {
    val (stdoutStringBuilder, stdoutFunc) = Vivem.stdoutCollector()
    Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.emptyStdin, stdoutFunc)
    stdoutStringBuilder.mkString
  }
  */

  // mig: fn eval_for_kind_and_stdout
  pub fn eval_for_kind_and_stdout<'v>(&mut self, args: Vec<PrimitiveKindV<'v, 'h, 's>>) -> Result<(IVonData, String), VmRuntimeErrorV<'s>> {
      let (stdoutput_string_builder, stdout_func) = stdout_collector::<'s>();
      let interner = self.hammer_compilation.interner;
      let scout_arena = self.hammer_compilation.scout_arena;
      let hamuts = self.get_hamuts();
      let mut vivem_dout = stdout();
      let vivem_bump = bumpalo::Bump::new();
      let kind = execute_with_primitive_args(
          hamuts, interner, scout_arena, &args, &mut vivem_dout, &vivem_bump, &empty_stdin, &*stdout_func,
      )?;
      let result = stdoutput_string_builder.borrow().clone();
      Ok((kind, result))
  }
  /*
  def evalForKindAndStdout(args: Vector[PrimitiveKindV]): (IVonData, String) = {
    val (stdoutStringBuilder, stdoutFunc) = Vivem.stdoutCollector()
    val kind = Vivem.executeWithPrimitiveArgs(getHamuts(), args, System.out, Vivem.emptyStdin, stdoutFunc)
    (kind, stdoutStringBuilder.mkString)
  }
}
*/
}
