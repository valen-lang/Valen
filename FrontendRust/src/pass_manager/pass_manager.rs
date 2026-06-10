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
use crate::parsing::vonifier::ParserVonifier;
use crate::von::printer::VonPrinter;
use crate::utils::code_hierarchy::FileCoordinateMap;
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
  pub input_vpst_dir: Option<String>,
  pub benchmark: bool,
  pub output_vpst: bool,
  pub output_vast: bool,
  pub output_highlights: bool,
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
    debugOutput: Boolean,
    valeRusterPath: Option[String] = None,
    rustCargoToml: Option[String] = None,
    rustOutputDir: Option[String] = None
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
    "--input_vpst" => {
      // From PassManager.scala lines 78-81
      if index + 1 >= list.len() {
        eprintln!("--input_vpst requires a value");
        exit(22);
      }
      if opts.input_vpst_dir.is_some() {
        eprintln!("Multiple --input_vpst specified!");
        exit(22);
      }
      opts.input_vpst_dir = Some(list[index + 1].clone());
      parse_opts_recursive(parse_arena, opts, list, index + 2)
    }
    "--output_vpst" => {
      // From PassManager.scala lines 82-84
      if index + 1 >= list.len() {
        eprintln!("--output_vpst requires a value");
        exit(22);
      }
      opts.output_vpst = list[index + 1].parse().unwrap_or(false);
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
    "--output_highlights" => {
      // From PassManager.scala lines 103-105
      if index + 1 >= list.len() {
        eprintln!("--output_highlights requires a value");
        exit(22);
      }
      opts.output_highlights = list[index + 1].parse().unwrap_or(false);
      parse_opts_recursive(parse_arena, opts, list, index + 2)
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
          let input = if path.ends_with(".vale") || path.ends_with(".vpst") {
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
      case "--vale_ruster_path" :: value :: tail => {
        parseOpts(interner, opts.copy(valeRusterPath = Some(value)), tail)
      }
      case "--rust_cargo_toml" :: value :: tail => {
        parseOpts(interner, opts.copy(rustCargoToml = Some(value)), tail)
      }
      case "--rust_output_dir" :: value :: tail => {
        parseOpts(interner, opts.copy(rustOutputDir = Some(value)), tail)
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
              if name_str.ends_with(".vale") || name_str.ends_with(".vpst") {
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

  private def invokeValeRusterIfNeeded(
    opts: Options,
    allInputs: Vector[IFrontendInput]):
  Option[String] = {
    (opts.valeRusterPath, opts.rustCargoToml, opts.rustOutputDir) match {
      case (Some(valeRusterPath), Some(cargoToml), Some(outputDir)) => {
        val importRegex = """import\s+rust\s*\.([\w\.]+)""".r
        val rustImports = allInputs.flatMap({
          case SourceInput(_, _, code) => importRegex.findAllMatchIn(code).map(_.group(1)).toVector
          case ModulePathInput(_, modulePath) => {
            val directory = new java.io.File(modulePath)
            val files = Option(directory.listFiles()).getOrElse(Array())
            files.filter(_.getName.endsWith(".vale")).flatMap(file => {
              val code = Source.fromFile(file).mkString
              importRegex.findAllMatchIn(code).map(_.group(1)).toVector
            }).toVector
          }
          case DirectFilePathInput(_, path) => {
            val code = Source.fromFile(path).mkString
            importRegex.findAllMatchIn(code).map(_.group(1)).toVector
          }
        }).distinct
        if (rustImports.nonEmpty) {
          val bindingsDir = outputDir + "/vale_bindings"
          new java.io.File(bindingsDir).mkdirs()
          val scratchDir = outputDir + "/rust_scratch"
          new java.io.File(scratchDir).mkdirs()
          val crateName = rustImports.head.split("\\.").head
          for (typePath <- rustImports) {
            import scala.sys.process._
            val cmd = Seq(
              valeRusterPath,
              "--crate", crateName,
              "--cargo_toml", cargoToml,
              "--vale_bindings_dir", bindingsDir,
              "--output_dir", scratchDir,
              "--type", typePath,
              "list")
            val exitCode = cmd.!
            if (exitCode != 0) {
              throw InputException("ValeRuster failed for type " + typePath + " (exit code " + exitCode + ")")
            }
          }
          Some(bindingsDir)
        } else {
          None
        }
      }
      case (None, None, None) => None
      case _ => throw InputException("Must specify all of --vale_ruster_path, --rust_cargo_toml, --rust_output_dir, or none of them.")
    }
  }
  private def resolveRustPackageContents(
    rustBindingsDir: Option[String],
    packageCoord: PackageCoordinate):
  Option[Map[String, String]] = {
    rustBindingsDir match {
      case None => None
      case Some(bindingsDir) => {
        if (packageCoord.module.str != "rust") {
          None
        } else {
          val subPath = packageCoord.packages.map(_.str).mkString(java.io.File.separator)
          val directory = new java.io.File(bindingsDir + java.io.File.separator + subPath)
          val files = Option(directory.listFiles()).getOrElse(Array())
          val valeFiles = files.filter(_.getName.endsWith(".vale"))
          if (valeFiles.isEmpty) {
            None
          } else {
            val filepathToCode = valeFiles.map(file => {
              val code = Source.fromFile(file).mkString
              (file.getPath -> code)
            }).toMap
            Some(filepathToCode)
          }
        }
      }
    }
  }
*/

// From PassManager.scala lines 356-366: buildAndOutput
fn build_and_output<'p>(parse_arena: &'p ParseArena<'p>, keywords: &'p Keywords<'p>, opts: &Options<'p>) {
  match build(parse_arena, keywords, opts) {
    Ok(_) => {
      // Success
    }
    Err(error) => {
      eprintln!("Error: {}", error);
      exit(22);
    }
  }
}

// From PassManager.scala lines 203-342: build function
pub fn build<'p, 'ctx>(
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  opts: &Options<'p>,
) -> Result<(), String>
where
  'p: 'ctx,
{
  // From PassManager.scala lines 205-207: Create output directories
  let output_dir_path = opts.output_dir_path.as_ref().unwrap();
  fs::create_dir_all(output_dir_path)
    .map_err(|e| format!("Failed to create output directory: {}", e))?;
  fs::create_dir_all(format!("{}/vast", output_dir_path))
    .map_err(|e| format!("Failed to create vast directory: {}", e))?;
  fs::create_dir_all(format!("{}/vpst", output_dir_path))
    .map_err(|e| format!("Failed to create vpst directory: {}", e))?;

  // From PassManager.scala line 209
  let _start_time = Instant::now();

  // From PassManager.scala lines 211-227: Load .vpst files if --input_vpst is provided
  if opts.input_vpst_dir.is_some() {
    panic!("Loading .vpst files not yet implemented - see PassManager.scala lines 213-225. Need ParsedLoader and SourceInput")
  }
  let all_inputs = &opts.inputs;

  // From PassManager.scala line 229: Get distinct package coordinates
  let package_coords: Vec<&PackageCoordinate<'p>> = all_inputs
    .iter()
    .map(|input| input.package_coord(parse_arena))
    .collect::<HashSet<_>>()
    .into_iter()
    .collect();

  // From PassManager.scala lines 231-253: Create FullCompilation
  // Note: Builtins are needed but we don't have builtins_dir available yet.
  // For now, create an empty builtins map. This will need to be fixed when
  // builtins are actually required for parsing.
  let builtins_code_map = FileCoordinateMap::<String>::new();

  // From PassManager.scala line 235: Add BUILTIN package coordinate
  let mut packages_to_build = vec![PackageCoordinate::builtin(parse_arena, keywords)];
  packages_to_build.extend(package_coords);

  // From PassManager.scala lines 236-237: Create resolver that tries builtins first, then resolvePackageContents
  let all_inputs_clone = all_inputs.clone();
  let resolver = builtins_code_map.or(move |package_coord: &'p PackageCoordinate<'p>| {
    resolve_package_contents(parse_arena, &all_inputs_clone, &*package_coord)
  });

  // From PassManager.scala lines 238-253: Create FullCompilationOptions
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

  // From PassManager.scala lines 231-233: Create FullCompilation
  // Under the per-pass arena model, the parser uses the 'p arena via parse_arena,
  // and the scout pass gets its own arena.
  // V: should we reference some docs here about how our arenas work
  // VA: (documentation task — see docs/background/arenas.md and docs/architecture/arenas.md)
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

  // From PassManager.scala line 255
  let _start_load_and_parse_time = Instant::now();

  // From PassManager.scala lines 266-269: Error humanizer functions (not used yet)
  // Keep this before get_parseds so mutable borrows don't overlap.
  let vale_code_map = compilation.get_code_map().expect("getCodeMap failed");

  // From PassManager.scala lines 257-263: Get parsed files
  let parseds = match compilation.get_parseds() {
    Err(failed_parse) => {
      // From PassManager.scala lines 259-261
      panic!(
        "ParseErrorHumanizer.humanize not yet implemented. FailedParse: {:?}",
        failed_parse
      );
    }
    Ok(p) => p,
  };
  // From PassManager.scala lines 271-279: Write VPST files if requested
  if opts.output_vpst {

    for (file_coord, (program_p, _comment_ranges)) in &parseds.file_coord_to_contents {
      // From PassManager.scala line 273
      let von = ParserVonifier::vonify_file(program_p);
      // From PassManager.scala line 274
      let vpst_json = VonPrinter::new().print(&von);
      // From PassManager.scala lines 275-276
      let parts: Vec<&str> = file_coord.filepath.split(&['/', '\\'][..]).collect();
      let filename = parts.last().unwrap().replace(".vale", ".vpst");
      let vpst_filepath = format!("{}/vpst/{}", output_dir_path, filename);
      // From PassManager.scala line 277
      write_file(&vpst_filepath, &vpst_json);
    }
  }

  // From PassManager.scala lines 281-284: Benchmark timing
  let _start_scout_time = Instant::now();
  if opts.benchmark {
    println!(
      "Loading and parsing duration: {:?}",
      _start_scout_time.duration_since(_start_load_and_parse_time)
    );
  }

  // From PassManager.scala lines 395-447: Full compilation (scout, typing, hammer) - only if outputVAST
  if opts.output_vast {
    // From PassManager.scala lines 396-398
    match compilation.get_scoutput() {
      Err(e) => panic!("PostParserErrorHumanizer.humanize not yet implemented: {:?}", e),
      Ok(_) => {}
    }

    // From PassManager.scala lines 401-404
    let _start_higher_typing_time = Instant::now();
    if opts.benchmark {
      println!(
        "Scout phase duration: {:?}",
        _start_higher_typing_time.duration_since(_start_scout_time)
      );
    }

    // From PassManager.scala lines 406-409
    match compilation.get_astrouts() {
      Err(error) => return Err(higher_typing_error_humanizer::humanize(
        &|x| source_code_utils::humanize_pos_code_map(&vale_code_map, &x),
        &|a, b| source_code_utils::lines_between(&vale_code_map, &a, &b),
        &|x| source_code_utils::line_range_containing(&vale_code_map, &x),
        &|x| source_code_utils::line_containing(&vale_code_map, &x),
        &error)),
      Ok(_) => {}
    }

    // From PassManager.scala lines 411-414
    let _start_typing_pass_time = Instant::now();
    if opts.benchmark {
      println!(
        "Higher typing phase duration: {:?}",
        _start_typing_pass_time.duration_since(_start_higher_typing_time)
      );
    }

    // From PassManager.scala lines 416-419
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

    // From PassManager.scala lines 421-424
    let _start_hammer_time = Instant::now();
    if opts.benchmark {
      println!(
        "Compiler phase duration: {:?}",
        _start_hammer_time.duration_since(_start_typing_pass_time)
      );
    }

    // From PassManager.scala line 426
    let program_h = compilation.get_hamuts();

    // From PassManager.scala lines 428-431
    let _finish_time = Instant::now();
    if opts.benchmark {
      println!(
        "Hammer phase duration: {:?}",
        _finish_time.duration_since(_start_hammer_time)
      );
    }

    // From PassManager.scala lines 433-446
    let von_hammer = compilation.get_von_hammer();
    program_h.packages.flat_map(|package_coord, paackage| {
      let output_vast_filepath = format!(
        "{}/vast/{}.vast",
        output_dir_path,
        if package_coord.is_internal() {
          "__vale".to_string()
        } else {
          format!(
            "{}{}",
            package_coord.module,
            package_coord.packages.iter().map(|p| format!(".{}", p)).collect::<String>()
          )
        }
      );
      let json = jsonify_package(&von_hammer, *package_coord, paackage);
      write_file(&output_vast_filepath, &json);
      // println!("Wrote VAST to file {}", output_vast_filepath);
    });
  }

  Ok(())
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
    val rustBindingsDir = invokeValeRusterIfNeeded(opts, allInputs)
    val packageCoords = allInputs.map(_.packageCoord(interner)).distinct

    val compilation =
      new FullCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords)) ++ packageCoords,
        Builtins.getCodeMap(interner, keywords)
          .or(packageCoord => resolveRustPackageContents(rustBindingsDir, packageCoord))
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


// From PassManager.scala lines 1028-1032: jsonifyPackage
fn jsonify_package<'s, 'i, 'h, 'ctx>(
  von_hammer: &Hammer<'s, 'i, 'h, 'ctx>,
  package_coord: PackageCoordinate<'s>,
  package_h: &PackageH<'s, 'h>,
) -> String
where 's: 'h, 's: 'i, 'i: 'h,
{
  let program_v = von_hammer.vonify_package(package_coord, package_h);
  let json = VonPrinter::new().print(&program_v);
  json
}
/*
  def jsonifyPackage(vonHammer: VonHammer, packageCoord: PackageCoordinate, packageH: PackageH): String = {
    val programV = vonHammer.vonifyPackage(packageCoord, packageH)
    val json = new VonPrinter(JsonSyntax, 120).print(programV)
    json
  }
*/
/*
  def jsonifyProgram(vonHammer: VonHammer, programH: ProgramH): String = {
    val programV = vonHammer.vonifyProgram(programH)
    val json = new VonPrinter(JsonSyntax, 120).print(programV)
    json
  }
*/
/*
  def buildAndOutput(interner: Interner, keywords: Keywords, opts: Options) = {
      build(interner, keywords, opts) match {
        case Ok(_) => {
        }
        case Err(error) => {
          System.err.println("Error: " + error)
          System.exit(22)
          vfail()
        }
      }
  }
*/
/*
  def run(program: ProgramH, verbose: Boolean): IVonData = {
    if (verbose) {
      Vivem.executeWithPrimitiveArgs(
        program, Vector(), System.out, Vivem.emptyStdin, Vivem.nullStdout)
    } else {
      Vivem.executeWithPrimitiveArgs(
        program,
        Vector(),
        new PrintStream(new OutputStream() {
          override def write(b: Int): Unit = {
            // System.out.write(b)
          }
        }),
        () => {
          scala.io.StdIn.readLine()
        },
        (str: String) => {
          print(str)
        })
    }
  }
*/

// From PassManager.scala lines 390-481: main
pub fn main(args: Vec<String>) {
  // From PassManager.scala lines 391-393
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);

  // From PassManager.scala lines 395-413
  let opts = parse_opts(
    &parse_arena,
    Options {
      inputs: vec![],
      output_dir_path: None,
      input_vpst_dir: None,
      benchmark: false,
      output_vpst: true,
      output_vast: true,
      output_highlights: false,
      include_builtins: true,
      mode: None,
      sanity_check: false,
      use_optimized_solver: true,
      use_overload_index: true,
      verbose_errors: false,
      debug_output: false,
    },
    args,
  );

  // From PassManager.scala lines 414-415
  if opts.mode.is_none() {
    eprintln!("No mode!");
    exit(22);
  }
  if opts.inputs.is_empty() && opts.input_vpst_dir.is_none() {
    eprintln!("No input files!");
    exit(22);
  }

  // From PassManager.scala lines 417-474
  match opts.mode.as_ref().unwrap().as_str() {
    "highlight" => {
      // Retired. See PassManager.scala — moved to VmdSiteGen/tools/highlighter
      // (inkjet + tree-sitter-vale).
      eprintln!("Highlight mode has been retired; use VmdSiteGen/tools/highlighter (inkjet + tree-sitter-vale) instead.");
      exit(22);
    }
    "build" => {
      // From PassManager.scala lines 467-469
      if opts.output_dir_path.is_none() {
        eprintln!("Must specify --output-dir!");
        exit(22);
      }
  build_and_output(&parse_arena, &keywords, &opts);
    }
    "run" => {
      // From PassManager.scala lines 471-473
      eprintln!("Run command has been disabled.");
      exit(22);
    }
    _ => {
      eprintln!("Unknown mode: {}", opts.mode.as_ref().unwrap());
      exit(22);
    }
  }
}
/*
  def main(args: Array[String]): Unit = {
    try {
      val interner = new Interner()
      val keywords = new Keywords(interner)

      val opts =
        parseOpts(
          interner,
          Options(
            inputs = Vector.empty,
            outputDirPath = None,
            inputVpstDir = None,
            benchmark = false,
            outputVPST = true,
            outputVAST = true,
            outputHighlights = false,
            includeBuiltins = true,
            mode = None,
            sanityCheck = false,
            useOptimizedSolver = true,
            useOverloadIndex = true,
            verboseErrors = false,
            debugOutput = false),
          args.toList)
      vcheck(opts.mode.nonEmpty, "No mode!", InputException)
      vcheck(opts.inputs.nonEmpty || opts.inputVpstDir.nonEmpty, "No input files!", InputException)

      opts.mode.get match {
        case "highlight" => {
          // Retired. Static-HTML highlighting now lives in VmdSiteGen/tools/highlighter
          // (Rust binary using inkjet + the tree-sitter-vale grammar). Editor
          // highlighting uses the same tree-sitter grammar directly. See
          // VmdSiteGen commit e4953c7 "Replace Java/highlight.js highlighting
          // with build-time inkjet binary".
          throw InputException("Highlight mode has been retired; use VmdSiteGen/tools/highlighter (inkjet + tree-sitter-vale) instead.")
        }
        case "build" => {
          vcheck(opts.outputDirPath.nonEmpty, "Must specify --output-dir!", InputException)
          buildAndOutput(interner, keywords, opts)
        }
        case "run" => {
          throw InputException("Run command has been disabled.");
        }
      }
    } catch {
      case ie @ InputException(msg) => {
        println(msg)
        System.exit(22)
      }
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
