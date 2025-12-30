// From Frontend/LexingPass/src/dev/vale/lexing/LexAndExplore.scala
// Import-driven parsing logic


use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::lexing_iterator::LexingIterator;
use crate::lexing::lexer::Lexer;
use crate::parsing::parser::Parser;
use crate::parsing::parse_error_humanizer::ParseErrorHumanizer;
use crate::parsing::ast::{FileP, IDenizenP};
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use std::collections::{HashMap, HashSet};
use crate::parsing::vonifier::ParserVonifier;
use crate::von::printer::VonPrinter;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::{Arc, Mutex};

/*
package dev.vale.lexing

import dev.vale.options.GlobalOptions
import dev.vale._

import scala.collection.immutable.Map
import scala.collection.mutable
*/
/*
object LexAndExplore {
*/

// From LexAndExplore.scala lines 43-150: Main import-driven parsing loop
pub fn lex_and_explore(
    interner: Arc<Interner>,
    keywords: Arc<Keywords>,
    packages: Vec<Arc<PackageCoordinate>>,
    module_roots: HashMap<String, PathBuf>,
    direct_file_inputs: HashMap<Arc<PackageCoordinate>, PathBuf>,
    vpst_dir: &Path,
) {
    // From LexAndExplore.scala lines 52-53
    let mut unexplored_packages: HashSet<Arc<PackageCoordinate>> = packages.into_iter().collect();
    let mut started_packages: HashSet<Arc<PackageCoordinate>> = HashSet::new();
    
    // From LexAndExplore.scala lines 58-146: Main loop
    while !unexplored_packages.is_empty() {
        // From LexAndExplore.scala lines 59-61
        let needed_package_coord = unexplored_packages.iter().next().cloned().unwrap();
        unexplored_packages.remove(&needed_package_coord);
        started_packages.insert(needed_package_coord.clone());
        
        // From LexAndExplore.scala lines 65-79: Resolve package to file paths
        let filepaths_and_contents = resolve_package(interner.clone(), &needed_package_coord, &module_roots, &direct_file_inputs);
        
        if filepaths_and_contents.is_empty() {
            eprintln!("Error: Couldn't find package: {}.{}", 
                needed_package_coord.module.str,
                needed_package_coord.packages.iter().map(|p| p.str.as_str()).collect::<Vec<_>>().join("."));
            process::exit(1);
        }
        
        // From LexAndExplore.scala lines 82-145: Process each file in the package
        for (file_coord, code) in filepaths_and_contents {
            // Parse the file and extract imports
            let file_p = parse_file_and_extract_imports(
                &file_coord,
                &code,
                interner.clone(),
                keywords.clone(),
            );
            
            // From LexAndExplore.scala lines 105-119, 137-140: Extract imports and add to unexplored
            let imports = extract_imports(&file_p);
            for import_coord in imports {
                if !started_packages.contains(&import_coord) {
                    unexplored_packages.insert(import_coord);
                }
            }
            
            // Generate .vpst file
            write_vpst(&file_p, vpst_dir);
        }
    }
}

/*
  // It would be pretty cool to turn this into an iterator of some sort
  def lexAndExplore[D, F](
    interner: Interner,
    keywords: Keywords,
    packages: Vector[PackageCoordinate],
    resolver: IPackageResolver[Map[String, String]],
    denizenHandler: (FileCoordinate, String, Vector[ImportL], IDenizenL) => D,
    fileHandler: (FileCoordinate, String, Accumulator[RangeL], Accumulator[D]) => F):
  Result[Accumulator[F], FailedParse] = {
    Profiler.frame(() => {
      val unexploredPackages = mutable.HashSet[PackageCoordinate](packages: _*)
      val startedPackages = mutable.HashSet[PackageCoordinate]()
      val alreadyFoundFileToCode = new FileCoordinateMap[String]()

      val filesAcc = new Accumulator[F]()

      while (unexploredPackages.nonEmpty) {
        val neededPackageCoord = unexploredPackages.head
        unexploredPackages.remove(neededPackageCoord)
        startedPackages.add(neededPackageCoord)

//        println(s"Processing ${neededPackageCoord}")

        val filepathsAndContents =
          resolver.resolve(neededPackageCoord) match {
            case None => {
              throw InputException("Couldn't find: " + neededPackageCoord)
            }
            case Some(filepathToCode) => {
              U.map[(String, String), (FileCoordinate, String)](filepathToCode.toVector, { case (filepath, code) =>
                vassert(interner != null)
//                println(s"Found ${neededPackageCoord} file ${filepath}")
                val fileCoord = interner.intern(FileCoordinate(neededPackageCoord, filepath))
                vassert(!alreadyFoundFileToCode.fileCoordToContents.contains(fileCoord))
                fileCoord -> code
              })
            }
          }
        alreadyFoundFileToCode.putPackage(interner, neededPackageCoord, filepathsAndContents.toMap)

        U.foreach[(FileCoordinate, String)](filepathsAndContents, { case (fileCoord, code) =>
          val resultAcc = new Accumulator[D]()

          val iter = new LexingIterator(code, 0)
          val lexer = new Lexer(interner, keywords)

          iter.consumeCommentsAndWhitespace()

          var maybeImportsAccum: Option[Accumulator[ImportL]] = Some(new Accumulator[ImportL]())
          var maybeImports: Option[Vector[ImportL]] = None

          // Imports must come first, so that we can ship these denizens off with all
          // their relevant imports.

          while (!iter.atEnd()) {
            val denizen =
              lexer.lexDenizen(iter) match {
                case Err(e) => return Err(FailedParse(code, fileCoord, e))
                case Ok(x) => x
              }
            iter.consumeCommentsAndWhitespace()

            denizen match {
              case TopLevelImportL(im@ImportL(range, moduleName, packageSteps, importeeName)) => {
                maybeImportsAccum match {
                  case None => vfail("Imports must come before everything else")
                  case Some(importsAccum) => importsAccum.add(im)
                }

                // This is where we could fire off another thread to do any parsing in parallel,
                // because we're still only partway through the lexing.
                val nextNeededPackageCoord =
                  interner.intern(PackageCoordinate(moduleName.str, U.map[WordLE, StrI](packageSteps, x => x.str).toVector))
//                println(s"Want to import ${nextNeededPackageCoord}")
                if (!startedPackages.contains(nextNeededPackageCoord)) {
//                  println(s"Unseen, so adding.")
                  unexploredPackages.add(nextNeededPackageCoord)
                }
                val denizenResult = denizenHandler(fileCoord, code, Vector(), denizen)
                resultAcc.add(denizenResult)
              }
              case _ => {
                maybeImportsAccum match {
                  case None =>
                  case Some(importsAccum) => {
                    maybeImports = Some(importsAccum.buildArray())
                    maybeImportsAccum = None
                  }
                }
                val denizenResult = denizenHandler(fileCoord, code, vassertSome(maybeImports), denizen)
                resultAcc.add(denizenResult)
              }
            }
          }

          unexploredPackages ++=
            U.map[ImportL, PackageCoordinate](maybeImports.toVector.flatten, x => {
              interner.intern(PackageCoordinate(x.moduleName.str, U.map[WordLE, StrI](x.packageSteps, _.str).toVector))
            }).toSet -- startedPackages

          val commentsRanges = iter.comments
          val file = fileHandler(fileCoord, code, commentsRanges, resultAcc)
          filesAcc.add(file)
        })
      }

      Ok(filesAcc)
    })
  }
}
*/

// Write FileP as .vpst file
fn write_vpst(file_p: &FileP, vpst_dir: &Path) {
    // Vonify (convert to JSON-serializable format)
    let von_data = ParserVonifier::vonify_file(file_p);

    // Serialize to JSON
    let von_printer = VonPrinter::new();
    let json_str = von_printer.print(&von_data);

    // Create output filename: module.package1.package2.filename.vpst
    let file_name = Path::new(&file_p.file_coord.filepath)
        .file_stem()
        .unwrap()
        .to_string_lossy();
    
    let mut output_filename = file_p.file_coord.package_coord.module.str.clone();
    for package_step in &file_p.file_coord.package_coord.packages {
        output_filename.push('.');
        output_filename.push_str(&package_step.str);
    }
    if !file_p.file_coord.package_coord.packages.is_empty() {
        output_filename.push('.');
    }
    output_filename.push_str(&file_name);
    output_filename.push_str(".vpst");

    let output_path = vpst_dir.join(&output_filename);
    
    // Write VPST file
    if let Err(e) = fs::write(&output_path, json_str) {
        eprintln!("Error writing {}: {}", output_path.display(), e);
        process::exit(1);
    }

    println!("    -> {}", output_path.display());
}


// From PassManager.scala lines 153-201: Resolve package coordinate to file paths
fn resolve_package(
    interner: Arc<Interner>,
    package_coord: &Arc<PackageCoordinate>,
    module_roots: &HashMap<String, PathBuf>,
    direct_file_inputs: &HashMap<Arc<PackageCoordinate>, PathBuf>,
) -> Vec<(Arc<FileCoordinate>, String)> {
    // From PassManager.scala line 190-196: Check for DirectFilePathInput first
    if let Some(file_path) = direct_file_inputs.get(package_coord) {
        if let Ok(code) = fs::read_to_string(file_path) {
            let file_coord = interner.intern_file_coordinate(FileCoordinate {
                package_coord: package_coord.clone(),
                filepath: file_path.to_string_lossy().to_string(),
            });
            return vec![(file_coord, code)];
        }
    }
    
    // From PassManager.scala line 168-189: ModulePathInput - find all files in directory
    let module_name = &package_coord.module.str;
    let module_root = match module_roots.get(module_name.as_str()) {
        Some(root) => root,
        None => {
            eprintln!("Error: No module root registered for module '{}'", module_name);
            process::exit(1);
        }
    };
    
    // Build path: module_root/package1/package2/...
    let mut dir_path = module_root.clone();
    for package_step in &package_coord.packages {
        dir_path.push(&package_step.str);
    }
    
    // Find all .vale files in this directory
    let mut results = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("vale") {
                if let Ok(code) = fs::read_to_string(&path) {
                    let file_coord = interner.intern_file_coordinate(FileCoordinate {
                        package_coord: package_coord.clone(),
                        filepath: path.to_string_lossy().to_string(),
                    });
                    results.push((file_coord, code));
                }
            }
        }
    }
    
    results
}

// From LexAndExplore.scala lines 82-135: Parse a file and return FileP
fn parse_file_and_extract_imports(
    file_coord: &Arc<FileCoordinate>,
    code: &str,
    interner: Arc<Interner>,
    keywords: Arc<Keywords>,
) -> FileP {
    println!("  Parsing: {}", file_coord.filepath);
    
    // From LexAndExplore.scala lines 85-102: Lex the file
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut lex_iter = LexingIterator::new(code.to_string());
    
    let mut denizens = Vec::new();
    while !lex_iter.at_end() {
        lex_iter.consume_comments_and_whitespace();
        if lex_iter.at_end() {
            break;
        }
        
        match lexer.lex_denizen(&mut lex_iter) {
            Ok(denizen) => denizens.push(denizen),
            Err(e) => {
                let error_msg = ParseErrorHumanizer::humanize(Path::new(&file_coord.filepath), code, &e);
                eprint!("{}", error_msg);
                process::exit(22);
            }
        }
    }

    // Parse the denizens
    let mut parser = Parser::new(interner.clone(), keywords.clone());
    let mut parsed_denizens = Vec::new();
    
    for denizen in denizens {
        match parser.parse_denizen(denizen) {
            Ok(parsed) => parsed_denizens.push(parsed),
            Err(e) => {
                let error_msg = ParseErrorHumanizer::humanize(Path::new(&file_coord.filepath), code, &e);
                eprint!("{}", error_msg);
                process::exit(22);
            }
        }
    }

    FileP {
        file_coord: file_coord.clone(),
        comments_ranges: vec![],
        denizens: parsed_denizens,
    }
}

// From LexAndExplore.scala lines 105-119, 137-140: Extract imports from parsed file
fn extract_imports(file_p: &FileP) -> Vec<Arc<PackageCoordinate>> {
    let mut imports = Vec::new();
    
    for denizen in &file_p.denizens {
        if let IDenizenP::TopLevelImport(import) = denizen {
            // Build PackageCoordinate from import
            let package_coord = Arc::new(PackageCoordinate {
                module: import.module_name.str.clone(),
                packages: import.package_steps.iter().map(|name| name.str.clone()).collect(),
            });
            imports.push(package_coord);
        }
    }
    
    imports
}

/*
  // This is a helper function that one doesn't need to use, but it can be handy and also
  // serves as a great example on how to use the lexAndExplore() method.
  def lexAndExploreAndCollect[D, F](
    interner: Interner,
    keywords: Keywords,
    packages: Vector[PackageCoordinate],
    resolver: IPackageResolver[Map[String, String]]):
  Result[
    (
      Accumulator[(FileCoordinate, String, Vector[ImportL], IDenizenL)],
      Accumulator[(FileCoordinate, String, Vector[RangeL], Vector[IDenizenL])]),
  FailedParse] = {
    val denizens = new Accumulator[(FileCoordinate, String, Vector[ImportL], IDenizenL)]()
    val files = new Accumulator[(FileCoordinate, String, Vector[RangeL], Vector[IDenizenL])]()

    lexAndExplore[IDenizenL, Unit](
      interner, keywords, packages, resolver,
      (file, code, imports, denizen) => {
        denizens.add((file, code, imports, denizen))
        denizen
      },
      (file, code, ranges, denizens) => {
        files.add((file, code, ranges.buildArray(), denizens.buildArray()))
        Unit
      }) match {
      case Err(e) => return Err(e)
      case Ok(_) =>
    }

    Ok((denizens, files))
  }
*/