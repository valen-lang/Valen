// From Frontend/PassManager/src/dev/vale/passmanager/PassManager.scala
// Main entry point for the Vale compiler

use crate::compile_options::GlobalOptions;
use crate::higher_typing::higher_typing_error_humanizer;
use crate::utils::source_code_utils;
use crate::interner::StrI;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::pass_manager::FullCompilation;
use crate::pass_manager::FullCompilationOptions;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use bumpalo::Bump;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::builtins::builtins::get_code_map as get_builtins_code_map;
use crate::final_ast::ast::PackageH;
use crate::simplifying::hammer::Hammer;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::typing::typing_interner::TypingInterner;
use std::collections::HashSet;
use std::fs::write;
use std::path::Path;
use std::process::exit;
use std::time::Instant;
/*
package dev.vale.passmanager

import dev.vale.highertyping.HigherTypingErrorHumanizer
import dev.vale.simplifying.VonHammer
import dev.vale.finalast.{PackageH, ProgramH}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.{ParseErrorHumanizer, ParsedLoader, ParserVonifier}
import dev.vale.postparsing.PostParserErrorHumanizer
import dev.vale.typing.CompilerErrorHumanizer
import dev.vale.testvm.Vivem
import dev.vale.{Builtins, CodeLocationS, Err, FileCoordinate, Interner, Keywords, Ok, PackageCoordinate, Result, SourceCodeUtils, StrI, passmanager, vassert, vassertOne, vcheck, vcurious, vfail}

import java.io.{BufferedWriter, File, FileNotFoundException, FileOutputStream, FileWriter, OutputStream, PrintStream}
import java.util.InputMismatchException
import dev.vale.highertyping.ProgramA
import dev.vale.simplifying.Hammer
import dev.vale.finalast.PackageH
import dev.vale.lexing.{FailedParse, InputException}
import dev.vale.postparsing.PostParserErrorHumanizer
import dev.vale.typing.CompilerErrorHumanizer
import dev.vale.von.{IVonData, JsonSyntax, VonPrinter}

import java.nio.charset.Charset
import scala.io.Source
import scala.sys.process._
import scala.util.matching.Regex
*/

/*
object PassManager {
  def DEFAULT_PACKAGE_COORD(interner: Interner, keywords: Keywords) = interner.intern(PackageCoordinate(keywords.my_module, Vector.empty))
*/

#[derive(Clone)]
pub enum IFrontendInput<'a> {
  SourceInput {
    package_coord: &'a PackageCoordinate<'a>,
    name: String,
    code: String,
  },
  ModulePathInput {
    module: StrI<'a>,
    module_path: String,
  },
  DirectFilePathInput {
    package_coord: &'a PackageCoordinate<'a>,
    path: String,
  },
}
impl<'a> IFrontendInput<'a> {
  pub fn package_coord<'ctx>(&self, parse_arena: &'ctx ParseArena<'a>) -> &'a PackageCoordinate<'a> {
    match self {
      IFrontendInput::SourceInput { package_coord, .. } => *package_coord,
      IFrontendInput::ModulePathInput { module, .. } => {
        parse_arena.intern_package_coordinate(*module, &[])
      }
      IFrontendInput::DirectFilePathInput { package_coord, .. } => *package_coord,
    }
  }
}
/*

  sealed trait IFrontendInput {
    def packageCoord(interner: Interner): PackageCoordinate
  }
  case class ModulePathInput(moduleName: StrI, path: String) extends IFrontendInput {
    val hash = runtime.ScalaRunTime._hashCode(this)
    override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
    override def packageCoord(interner: Interner): PackageCoordinate = interner.intern(PackageCoordinate(moduleName, Vector.empty))
  }
  case class DirectFilePathInput(packageCoordinate: PackageCoordinate, path: String) extends IFrontendInput {
    val hash = runtime.ScalaRunTime._hashCode(this)
    override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
    override def packageCoord(interner: Interner): PackageCoordinate = packageCoordinate
  }
  case class SourceInput(
      packageCoordinate: PackageCoordinate,
      // Name isnt guaranteed to be unique, we sometimes hand in strings like "builtins.vale"
      name: String,
      code: String) extends IFrontendInput {
    val hash = runtime.ScalaRunTime._hashCode(this)
    override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
    override def packageCoord(interner: Interner): PackageCoordinate = packageCoordinate
  }
*/
// From PassManager.scala lines 52-68: Options
pub struct Options<'a> {
  pub inputs: Vec<IFrontendInput<'a>>,
  pub output_dir_path: Option<String>,
  pub benchmark: bool,
  pub output_vast: bool,
  pub include_builtins: bool,
  pub mode: Option<String>,
  pub sanity_check: bool,
  pub use_optimized_solver: bool,
  pub use_overload_index: bool,
  pub verbose_errors: bool,
  pub debug_output: bool,
}
/*
  case class Options(
    inputs: Vector[IFrontendInput],
//    modulePaths: Map[String, String],
//    packagesToBuild: Vector[PackageCoordinate],
    outputDirPath: Option[String],
    inputVpstDir: Option[String],
    benchmark: Boolean,
    outputVPST: Boolean,
    outputVAST: Boolean,
    outputHighlights: Boolean,
    includeBuiltins: Boolean,
    mode: Option[String], // build v run etc
    sanityCheck: Boolean,
    useOptimizedSolver: Boolean,
    useOverloadIndex: Boolean,
    verboseErrors: Boolean,
    debugOutput: Boolean
  ) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/

// From PassManager.scala lines 71-150: parseOpts
pub fn parse_opts<'a>(parse_arena: &'a ParseArena<'a>, opts: Options<'a>, list: Vec<String>) -> Options<'a> {
  parse_opts_recursive(parse_arena, opts, &list, 0)
}

fn parse_opts_recursive<'a>(
  parse_arena: &'a ParseArena<'a>,
  mut opts: Options<'a>,
  list: &[String],
  index: usize,
) -> Options<'a> {
  // From PassManager.scala line 72-73: case Nil => opts
  if index >= list.len() {
    return opts;
  }

  let arg = &list[index];

  // From PassManager.scala lines 74-111: Handle flags
  match arg.as_str() {
    "--output_dir" => {
      // From PassManager.scala lines 74-77
      if index + 1 >= list.len() {
        eprintln!("--output_dir requires a value");
        exit(22);
      }
      if opts.output_dir_path.is_some() {
        eprintln!("Multiple output files specified!");
        exit(22);
      }
      opts.output_dir_path = Some(list[index + 1].clone());
      parse_opts_recursive(parse_arena, opts, list, index + 2)
    }
    "--output_vast" => {
      // From PassManager.scala lines 85-87
      if index + 1 >= list.len() {
        eprintln!("--output_vast requires a value");
        exit(22);
      }
      opts.output_vast = list[index + 1].parse().unwrap_or(false);
      parse_opts_recursive(parse_arena, opts, list, index + 2)
    }
    "--sanity_check" => {
      // From PassManager.scala lines 88-90
      if index + 1 >= list.len() {
        eprintln!("--sanity_check requires a value");
        exit(22);
      }
      opts.sanity_check = list[index + 1].parse().unwrap_or(false);
      parse_opts_recursive(parse_arena, opts, list, index + 2)
    }
    "--include_builtins" => {
      // From PassManager.scala lines 91-93
      if index + 1 >= list.len() {
        eprintln!("--include_builtins requires a value");
        exit(22);
      }
      opts.include_builtins = list[index + 1].parse().unwrap_or(false);
      parse_opts_recursive(parse_arena, opts, list, index + 2)
    }
    "--use_overload_index" => {
      // From PassManager.scala lines 94-96
      if index + 1 >= list.len() {
        eprintln!("--use_overload_index requires a value");
        exit(22);
      }
      opts.use_overload_index = list[index + 1].parse().unwrap_or(false);
      parse_opts_recursive(parse_arena, opts, list, index + 2)
    }
    "--simple_solver" => {
      // From PassManager.scala lines 97-99
      if index + 1 >= list.len() {
        eprintln!("--simple_solver requires a value");
        exit(22);
      }
      opts.use_optimized_solver = !list[index + 1].parse().unwrap_or(false);
      parse_opts_recursive(parse_arena, opts, list, index + 2)
    }
    "--benchmark" => {
      // From PassManager.scala lines 100-102
      opts.benchmark = true;
      parse_opts_recursive(parse_arena, opts, list, index + 1)
    }
    "-v" | "--verbose" => {
      // From PassManager.scala lines 106-108
      opts.verbose_errors = true;
      parse_opts_recursive(parse_arena, opts, list, index + 1)
    }
    "--debug_output" => {
      // From PassManager.scala lines 109-111
      opts.debug_output = true;
      parse_opts_recursive(parse_arena, opts, list, index + 1)
    }
    _ if arg.starts_with("-") => {
      // From PassManager.scala line 112
      eprintln!("Unknown option {}", arg);
      exit(22);
    }
    _ => {
      // From PassManager.scala lines 113-149: Handle positional arguments
      if opts.mode.is_none() {
        // From PassManager.scala lines 114-115
        opts.mode = Some(arg.clone());
        parse_opts_recursive(parse_arena, opts, list, index + 1)
      } else {
        // From PassManager.scala lines 116-148
        if arg.contains("=") {
          // From PassManager.scala lines 117-144
          let parts: Vec<&str> = arg.split('=').collect();
          if parts.len() != 2 {
            eprintln!("Arguments can only have 1 equals. Saw: {}", arg);
            exit(22);
          }
          if parts[0].is_empty() {
            eprintln!("Must have a module name before equals. Saw: {}", arg);
            exit(22);
          }
          if parts[1].is_empty() {
            eprintln!("Must have a file path after equals. Saw: {}", arg);
            exit(22);
          }

          let package_coord_str = parts[0];
          let path = parts[1];

          // From PassManager.scala lines 123-134
          let package_coordinate = if package_coord_str.contains(".") {
            let package_coord_parts: Vec<&str> = package_coord_str.split('.').collect();
            let module = parse_arena.intern_str(package_coord_parts[0]);
            let packages: Vec<StrI<'a>> = package_coord_parts[1..]
              .iter()
              .map(|s| parse_arena.intern_str(s))
              .collect();
            parse_arena.intern_package_coordinate(module, &packages)
          } else {
            parse_arena.intern_package_coordinate(parse_arena.intern_str(package_coord_str), &[])
          };

          // From PassManager.scala lines 135-143
          let input = if path.ends_with(".vale") {
            IFrontendInput::DirectFilePathInput {
              package_coord: package_coordinate,
              path: path.to_string(),
            }
          } else {
            if !package_coordinate.packages.is_empty() {
              eprintln!("Cannot define a directory for a specific package, only for a module.");
              exit(22);
            }
            IFrontendInput::ModulePathInput {
              module: package_coordinate.module,
              module_path: path.to_string(),
            }
          };

          opts.inputs.push(input);
          parse_opts_recursive(parse_arena, opts, list, index + 1)
        } else {
          // From PassManager.scala lines 145-147
          eprintln!("Unrecognized input: {}", arg);
          exit(22);
        }
      }
    }
  }
}
/*
  def parseOpts(interner: Interner, opts: Options, list: List[String]) : Options = {
    list match {
      case Nil => opts
      case "--output_dir" :: value :: tail => {
        vcheck(opts.outputDirPath.isEmpty, "Multiple output files specified!", InputException)
        parseOpts(interner, opts.copy(outputDirPath = Some(value)), tail)
      }
      case "--input_vpst" :: value :: tail => {
        vcheck(opts.inputVpstDir.isEmpty, "Multiple --input_vpst specified!", InputException)
        parseOpts(interner, opts.copy(inputVpstDir = Some(value)), tail)
      }
      case "--output_vpst" :: value :: tail => {
        parseOpts(interner, opts.copy(outputVPST = value.toBoolean), tail)
      }
      case "--output_vast" :: value :: tail => {
        parseOpts(interner, opts.copy(outputVAST = value.toBoolean), tail)
      }
      case "--sanity_check" :: value :: tail => {
        parseOpts(interner, opts.copy(sanityCheck = value.toBoolean), tail)
      }
      case "--include_builtins" :: value :: tail => {
        parseOpts(interner, opts.copy(includeBuiltins = value.toBoolean), tail)
      }
      case "--use_overload_index" :: value :: tail => {
        parseOpts(interner, opts.copy(useOverloadIndex = value.toBoolean), tail)
      }
      case "--simple_solver" :: value :: tail => {
        parseOpts(interner, opts.copy(useOptimizedSolver = !value.toBoolean), tail)
      }
      case "--benchmark" :: tail => {
        parseOpts(interner, opts.copy(benchmark = true), tail)
      }
      case "--output_highlights" :: value :: tail => {
        parseOpts(interner, opts.copy(outputHighlights = value.toBoolean), tail)
      }
      case ("-v" | "--verbose") :: tail => {
        parseOpts(interner, opts.copy(verboseErrors = true), tail)
      }
      case ("--debug_output") :: tail => {
        parseOpts(interner, opts.copy(debugOutput = true), tail)
      }
      case value :: _ if value.startsWith("-") => throw InputException("Unknown option " + value)
      case value :: tail => {
        if (opts.mode.isEmpty) {
          parseOpts(interner, opts.copy(mode = Some(value)), tail)
        } else {
          if (value.contains("=")) {
            val packageCoordAndPath = value.split("=")
            vcheck(packageCoordAndPath.size == 2, "Arguments can only have 1 equals. Saw: " + value, InputException)
            vcheck(packageCoordAndPath(0) != "", "Must have a module name before a colon. Saw: " + value, InputException)
            vcheck(packageCoordAndPath(1) != "", "Must have a file path after a colon. Saw: " + value, InputException)
            val Array(packageCoordStr, path) = packageCoordAndPath
            val packageCoordinate =
              if (packageCoordStr.contains(".")) {
                val packageCoordinateParts = packageCoordStr.split("\\.")
                interner.intern(
                  PackageCoordinate(
                    interner.intern(StrI(packageCoordinateParts.head)),
                    packageCoordinateParts.tail.toVector.map(s => interner.intern(StrI(s)))))
              } else {
                interner.intern(
                  PackageCoordinate(
                    interner.intern(StrI(packageCoordStr)), Vector.empty))
              }
            val input =
              if (path.endsWith(".vale") || path.endsWith(".vpst")) {
                DirectFilePathInput(packageCoordinate, path)
              } else {
                if (packageCoordinate.packages.nonEmpty) {
                  throw InputException("Cannot define a directory for a specific package, only for a module.")
                }
                ModulePathInput(packageCoordinate.module, path)
              }
            parseOpts(interner, opts.copy(inputs = opts.inputs :+ input), tail)
          } else {
            throw InputException("Unrecognized input: " + value)
          }
        }
      }
    }
  }
*/

// From PassManager.scala lines 153-201: Resolver that reads .vale files from filesystem
pub struct FileSystemResolver<'a> {
  module_roots: HashMap<String, PathBuf>,
  direct_file_inputs: HashMap<&'a PackageCoordinate<'a>, PathBuf>,
}

impl<'a> FileSystemResolver<'a> {
  pub fn new(
    module_roots: HashMap<String, PathBuf>,
    direct_file_inputs: HashMap<&'a PackageCoordinate<'a>, PathBuf>,
  ) -> Self {
    FileSystemResolver {
      module_roots,
      direct_file_inputs,
    }
  }
}

impl<'a> IPackageResolver<'a, HashMap<String, String>> for FileSystemResolver<'a> {
  // From PassManager.scala lines 153-201
  fn resolve(&self, package_coord: &'a PackageCoordinate<'a>) -> Option<HashMap<String, String>> {
    // From PassManager.scala lines 190-196: Check for DirectFilePathInput first
    if let Some(file_path) = self.direct_file_inputs.get(package_coord) {
      if let Ok(code) = fs::read_to_string(file_path) {
        let filepath = file_path.to_string_lossy().to_string();
        let mut result = HashMap::new();
        result.insert(filepath, code);
        return Some(result);
      }
    }

    // From PassManager.scala lines 168-189: ModulePathInput - find all files in directory
    let module_name = package_coord.module.as_str();
    let module_root = self.module_roots.get(module_name)?;

    // Build path: module_root/package1/package2/...
    let mut dir_path = module_root.clone();
    for package_step in &package_coord.packages {
      dir_path.push(package_step.as_str());
    }

    // Find all .vale files in this directory
    let mut results = HashMap::new();

    if let Ok(entries) = fs::read_dir(&dir_path) {
      for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("vale") {
          if let Ok(code) = fs::read_to_string(&path) {
            let filepath = path.to_string_lossy().to_string();
            results.insert(filepath, code);
          }
        }
      }
    }

    if results.is_empty() {
      None
    } else {
      Some(results)
    }
  }
}

// From PassManager.scala lines 153-201: resolvePackageContents
fn resolve_package_contents<'a>(
  parse_arena: &ParseArena<'a>,
  inputs: &[IFrontendInput<'a>],
  package_coord: &PackageCoordinate<'a>,
) -> Option<HashMap<String, String>>
{
  // From PassManager.scala line 158
  let module = &package_coord.module;
  let packages = &package_coord.packages;

  // From PassManager.scala lines 162-197
  let mut source_inputs: Vec<(String, String)> = Vec::new();

  for (index, input) in inputs.iter().enumerate() {
    if input.package_coord(parse_arena).module != *module {
      continue;
    }

    match input {
      IFrontendInput::SourceInput {
        package_coord: _,
        name,
        code,
      } => {
        // From PassManager.scala lines 164-167: SourceInput (for .vpst and .vale direct inputs)
        if packages.is_empty() {
          source_inputs.push((format!("{}({})", index, name), code.clone()));
        }
      }
      IFrontendInput::ModulePathInput {
        module: _,
        module_path,
      } => {
        // From PassManager.scala lines 168-188: ModulePathInput
        let mut directory_path = module_path.clone();
        for package_step in packages {
          directory_path.push('/');
          directory_path.push_str(package_step.as_str());
        }

        let directory = Path::new(&directory_path);
        if let Ok(entries) = fs::read_dir(directory) {
          for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name() {
              let name_str = name.to_string_lossy();
              if name_str.ends_with(".vale") {
                if let Ok(code) = fs::read_to_string(&path) {
                  source_inputs.push((path.display().to_string(), code));
                }
              }
            }
          }
        }
      }
      IFrontendInput::DirectFilePathInput {
        package_coord: _,
        path,
      } => {
        // From PassManager.scala lines 190-196: DirectFilePathInput
        if let Ok(code) = fs::read_to_string(path) {
          source_inputs.push((path.clone(), code));
        }
      }
    }
  }

  // From PassManager.scala lines 198-200: Group by filepath and check for overlaps
  let mut filepath_to_source: HashMap<String, String> = HashMap::new();
  for (filepath, code) in source_inputs {
    if filepath_to_source.contains_key(&filepath) {
      panic!("Input filepaths overlap!");
    }
    filepath_to_source.insert(filepath, code);
  }

  Some(filepath_to_source)
}

/*
  AFTERM: dedup the above with FileSystemResolver.
  def resolvePackageContents(
    interner: Interner,
      inputs: Vector[IFrontendInput],
      packageCoord: PackageCoordinate):
  Option[Map[String, String]] = {
    val PackageCoordinate(module, packages) = packageCoord

//    println("resolving " + packageCoord + " with inputs:\n" + inputs)

    val sourceInputs =
      inputs.zipWithIndex.filter(_._1.packageCoord(interner).module == module).flatMap({
        case (SourceInput(_, name, code), index) if (packages == Vector.empty) => {
          // All .vpst and .vale direct inputs are considered part of the root paackage.
          Vector((index + "(" + name + ")" -> code))
        }
        case (mpi @ ModulePathInput(_, modulePath), _) => {
//          println("checking with modulepathinput " + mpi)
          val directoryPath = modulePath + packages.map(File.separator + _.str).mkString("")
//          println("looking in dir " + directoryPath)
          val directory = new java.io.File(directoryPath)
          val filesInDirectory = directory.listFiles()
          if (filesInDirectory == null) {
            Vector()
          } else {
            val inputFiles =
              filesInDirectory.filter(_.getName.endsWith(".vale")) ++
                filesInDirectory.filter(_.getName.endsWith(".vpst"))
            //          println("found files: " + inputFiles)
            val inputFilePaths = inputFiles.map(_.getPath)
            inputFilePaths.toVector.map(filepath => {
              val bufferedSource = Source.fromFile(filepath)
              val code = bufferedSource.getLines.mkString("\n")
              bufferedSource.close
              (filepath -> code)
            })
          }
        }
        case (DirectFilePathInput(_, path), _) => {
          val file = path
          val bufferedSource = Source.fromFile(file)
          val code = bufferedSource.getLines.mkString("\n")
          bufferedSource.close
          Vector((path -> code))
        }
      })
    val filepathToSource = sourceInputs.groupBy(_._1).mapValues(_.head._2)
    vassert(sourceInputs.size == filepathToSource.size, "Input filepaths overlap!")
    Some(filepathToSource)
  }

*/


/// Returns `(exit_code, package_coord_stems)`. The stems are dot-joined
/// `(project, package_steps...)` strings (e.g. `"__vale"`, `"stdlib.collections.hashmap"`)
/// — one per compiled package. CoordinatorRust uses them to find the matching
/// `<project>/<package_steps>/native/*.c` files for the link step.
pub fn build<'p, 'ctx>(
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  opts: &Options<'p>,
  backend_argv: &[&str],
) -> Result<(i32, Vec<String>), String>
where
  'p: 'ctx,
{
  let output_dir_path = opts.output_dir_path.as_ref().unwrap();
  fs::create_dir_all(output_dir_path)
    .map_err(|e| format!("Failed to create output directory: {}", e))?;
  fs::create_dir_all(format!("{}/include", output_dir_path))
    .map_err(|e| format!("Failed to create include directory: {}", e))?;

  let all_inputs = &opts.inputs;

  let package_coords: Vec<&PackageCoordinate<'p>> = all_inputs
    .iter()
    .map(|input| input.package_coord(parse_arena))
    .collect::<HashSet<_>>()
    .into_iter()
    .collect();

  let builtins_code_map = get_builtins_code_map(parse_arena, keywords);
  let mut packages_to_build = vec![PackageCoordinate::builtin(parse_arena, keywords)];
  packages_to_build.extend(package_coords);

  let all_inputs_clone = all_inputs.clone();
  let resolver = builtins_code_map.or(move |package_coord: &'p PackageCoordinate<'p>| {
    resolve_package_contents(parse_arena, &all_inputs_clone, &*package_coord)
  });

  let options = FullCompilationOptions {
    global_options: GlobalOptions {
      sanity_check: opts.sanity_check,
      use_overload_index: opts.use_overload_index,
      use_optimized_solver: opts.use_optimized_solver,
      verbose_errors: opts.verbose_errors,
      debug_output: opts.debug_output,
    },
    debug_out: if opts.debug_output {
      Arc::new(|s: &str| println!("#: {}", s))
    } else {
      Arc::new(|_: &str| {})
    },
  };

  let scout_bump = bumpalo::Bump::new();
  let typing_bump = bumpalo::Bump::new();
  let hammer_bump = bumpalo::Bump::new();
  let instantiating_bump = bumpalo::Bump::new();
  let scout_arena = ScoutArena::new(&scout_bump);
  let scout_keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(parse_arena);
  let hammer_interner = HammerInterner::new(&hammer_bump);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compilation = FullCompilation::new(
    &scout_arena,
    &hammer_interner,
    &typing_interner,
    &scout_keywords,
    &parser_keywords,
    parse_arena,
    packages_to_build,
    &resolver,
    options,
    &instantiating_bump,
  );

  let vale_code_map = compilation.get_code_map().expect("getCodeMap failed");

  let parseds = match compilation.get_parseds() {
    Err(failed_parse) => panic!(
      "ParseErrorHumanizer.humanize not yet implemented. FailedParse: {:?}",
      failed_parse
    ),
    Ok(p) => p,
  };

  match compilation.get_scoutput() {
    Err(e) => panic!("PostParserErrorHumanizer.humanize not yet implemented: {:?}", e),
    Ok(_) => {}
  }
  match compilation.get_astrouts() {
    Err(error) => return Err(higher_typing_error_humanizer::humanize(
      &|x| source_code_utils::humanize_pos_code_map(&vale_code_map, &x),
      &|a, b| source_code_utils::lines_between(&vale_code_map, &a, &b),
      &|x| source_code_utils::line_range_containing(&vale_code_map, &x),
      &|x| source_code_utils::line_containing(&vale_code_map, &x),
      &error)),
    Ok(_) => {}
  }
  match compilation.get_compiler_outputs() {
    Err(e) => return Err(crate::typing::compiler_error_humanizer::humanize(
      &scout_arena, &typing_interner, opts.verbose_errors,
      &|x| crate::utils::source_code_utils::humanize_pos_code_map(&vale_code_map, &x),
      &|a, b| crate::utils::source_code_utils::lines_between(&vale_code_map, &a, &b),
      &|x| crate::utils::source_code_utils::line_range_containing(&vale_code_map, &x),
      &|x| crate::utils::source_code_utils::line_containing(&vale_code_map, &x),
      e)),
    Ok(_) => {}
  }

  let program_h = compilation.get_hamuts();

  // Collect (project, package_steps) stems for each compiled package, so
  // CoordinatorRust can find their matching native/*.c dirs at link time.
  // Empty module → "__vale" (Backend's `userFuncName` convention; see the
  // walker's lower_package_coord and Backend/src/vale.cpp).
  let package_coord_stems: Vec<String> = program_h.packages.package_coord_to_contents.iter()
    .map(|(coord, _pkg)| {
      let module = if coord.module.0.is_empty() { "__vale" } else { coord.module.0 };
      let pkg_steps: String = coord.packages.iter().map(|p| format!(".{}", p.0)).collect();
      format!("{}{}", module, pkg_steps)
    })
    .collect();

  // MetalLowerer: H-AST → MetalCache via FFI, replacing readjson.cpp.
  let cache = crate::backend_ffi::metal_cache::MetalCache::new();
  let program = crate::backend_ffi::metal_lowerer::populate_metal_cache(&cache, program_h);

  let rc = crate::backend_ffi::backend_compile_program_safe(&cache, &program, backend_argv);
  Ok((rc, package_coord_stems))
}

/*
  def build(interner: Interner, keywords: Keywords, opts: Options):
  Result[Option[ProgramH], String] = {
    new java.io.File(opts.outputDirPath.get).mkdirs()
    new java.io.File(opts.outputDirPath.get + "/vast").mkdir()
    new java.io.File(opts.outputDirPath.get + "/vpst").mkdir()

    val startTime = java.lang.System.currentTimeMillis()

    // If --input_vpst is provided, load .vpst files
    // The Rust parser already filtered to only needed packages via import-driven parsing
    val allInputs = opts.inputVpstDir match {
      case Some(vpstDir) => {
        val vpstFiles = new java.io.File(vpstDir).listFiles().filter(_.getName.endsWith(".vpst"))
        val vpstInputs = vpstFiles.map(file => {
          val code = Source.fromFile(file).mkString
          val fileP = new ParsedLoader(interner).load(code) match {
            case Err(e) => return Err(s"Failed to load ${file.getName}: $e")
            case Ok(f) => f
          }
          SourceInput(fileP.fileCoord.packageCoordinate, file.getPath, code)
        }).toVector
        opts.inputs ++ vpstInputs
      }
      case None => opts.inputs
    }
    val packageCoords = allInputs.map(_.packageCoord(interner)).distinct

    val compilation =
      new FullCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords)) ++ packageCoords,
        Builtins.getCodeMap(interner, keywords)
          .or(packageCoord => resolvePackageContents(interner, allInputs, packageCoord)),
        passmanager.FullCompilationOptions(
          GlobalOptions(
            sanityCheck = opts.sanityCheck,
            useOverloadIndex = opts.useOverloadIndex,
            useOptimizedSolver = opts.useOptimizedSolver,
            verboseErrors = opts.verboseErrors,
            debugOutput = opts.debugOutput),
          if (opts.debugOutput) {
            (x => {
              println("#: " + x)
            })
          } else {
            x => Unit // do nothing with it
          }
        )
      )

    val startLoadAndParseTime = java.lang.System.currentTimeMillis()

    val parseds =
      compilation.getParseds() match {
        case Err(FailedParse(code, fileCoord, err)) => {
          vfail(ParseErrorHumanizer.humanize(SourceCodeUtils.humanizeFile(fileCoord), code, err))
        }
        case Ok(p) => p
      }
    val valeCodeMap = compilation.getCodeMap().getOrDie()

    val humanizePos = (x: CodeLocationS) => SourceCodeUtils.humanizePos(valeCodeMap, x)
    val linesBetween = (x: CodeLocationS, y: CodeLocationS) => SourceCodeUtils.linesBetween(valeCodeMap, x, y)
    val lineRangeContaining = (x: CodeLocationS) => SourceCodeUtils.lineRangeContaining(valeCodeMap, x)
    val lineContaining = (x: CodeLocationS) => SourceCodeUtils.lineContaining(valeCodeMap, x)

    if (opts.outputVPST) {
      parseds.map({ case (FileCoordinate(_, filepath), (programP, commentRanges)) =>
        val von = ParserVonifier.vonifyFile(programP)
        val vpstJson = new VonPrinter(JsonSyntax, 120).print(von)
        val parts = filepath.split("[/\\\\]")
        val vpstFilepath = opts.outputDirPath.get + "/vpst/" + parts.last.replaceAll("\\.vale", ".vpst")
        writeFile(vpstFilepath, vpstJson)
      })
    }

    val startScoutTime = java.lang.System.currentTimeMillis()
    if (opts.benchmark) {
      println("Loading and parsing duration: " + (startScoutTime - startLoadAndParseTime))
    }

    if (opts.outputVAST) {
      compilation.getScoutput() match {
        case Err(e) => return Err(PostParserErrorHumanizer.humanize(humanizePos, linesBetween, lineRangeContaining, lineContaining, e))
        case Ok(p) => p
      }

      val startHigherTypingTime = java.lang.System.currentTimeMillis()
      if (opts.benchmark) {
        println("Scout phase duration: " + (startHigherTypingTime - startScoutTime))
      }

      compilation.getAstrouts() match {
        case Err(error) => return Err(HigherTypingErrorHumanizer.humanize(humanizePos, linesBetween, lineRangeContaining, lineContaining, error))
        case Ok(result) => result
      }

      val startTypingPassTime = java.lang.System.currentTimeMillis()
      if (opts.benchmark) {
        println("Higher typing phase duration: " + (startTypingPassTime - startHigherTypingTime))
      }

      compilation.getCompilerOutputs() match {
        case Err(error) => return Err(CompilerErrorHumanizer.humanize(opts.verboseErrors, humanizePos, linesBetween, lineRangeContaining, lineContaining, error))
        case Ok(x) => x
      }

      val startHammerTime = java.lang.System.currentTimeMillis()
      if (opts.benchmark) {
        println("Compiler phase duration: " + (startHammerTime - startTypingPassTime))
      }

      val programH = compilation.getHamuts()

      val finishTime = java.lang.System.currentTimeMillis()
      if (opts.benchmark) {
        println("Hammer phase duration: " + (finishTime - startHammerTime))
      }

      programH.packages.flatMap({ case (packageCoord, paackage) =>
        val outputVastFilepath =
          opts.outputDirPath.get + "/vast/" +
          (if (packageCoord.isInternal) {
            "__vale"
          } else {
            packageCoord.module.str + packageCoord.packages.map("." + _.str).mkString("")
          }) +
          ".vast"
        val json = jsonifyPackage(compilation.getVonHammer(), packageCoord, paackage)
        writeFile(outputVastFilepath, json)
//        println("Wrote VAST to file " + outputVastFilepath)
      })

      Ok(Some(programH))
    } else {
      Ok(None)
    }
  }
*/


// From PassManager.scala lines 551-560: writeFile
fn write_file(filepath: &str, s: &str) {
  if filepath == "stdout:" {
    println!("{}", s);
  } else {
    let bytes = s.as_bytes();
    write(filepath, bytes)
      .unwrap_or_else(|e| panic!("Failed to write file {}: {}", filepath, e));
  }
}
/*
  def writeFile(filepath: String, s: String): Unit = {
    if (filepath == "stdout:") {
      println(s)
    } else {
      val bytes = s.getBytes(Charset.forName("UTF-8"))
      val outputStream = new FileOutputStream(filepath)
      outputStream.write(bytes)
      outputStream.close()
    }
  }
}
*/
