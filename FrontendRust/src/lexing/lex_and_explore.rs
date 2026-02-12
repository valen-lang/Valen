use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::ast::{IDenizenL, ImportL, RangeL};
use crate::lexing::errors::FailedParse;
use crate::lexing::lexer::Lexer;
use crate::lexing::lexing_iterator::LexingIterator;
use crate::utils::code_hierarchy::{
  FileCoordinate, FileCoordinateMap, IPackageResolver, PackageCoordinate,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

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

/// Main generic lexing function with import-driven package discovery
/// From LexAndExplore.scala lines 43-150
pub fn lex_and_explore<D, F, R>(
  interner: Arc<Interner>,
  keywords: Arc<Keywords>,
  packages: Vec<Arc<PackageCoordinate>>,
  resolver: &R,
  mut denizen_handler: impl FnMut(&Arc<FileCoordinate>, &str, &[ImportL], &IDenizenL) -> D,
  mut file_handler: impl FnMut(&Arc<FileCoordinate>, &str, &[RangeL], &[D]) -> F,
) -> Result<Vec<F>, FailedParse>
where
  R: IPackageResolver<HashMap<String, String>>,
{
  let mut unexplored_packages: HashSet<Arc<PackageCoordinate>> = packages.into_iter().collect();
  let mut started_packages: HashSet<Arc<PackageCoordinate>> = HashSet::new();
  let mut already_found_file_to_code = FileCoordinateMap::<String>::new();

  let mut files_acc = Vec::new();

  while !unexplored_packages.is_empty() {
    let needed_package_coord = unexplored_packages.iter().next().cloned().unwrap();
    unexplored_packages.remove(&needed_package_coord);
    started_packages.insert(needed_package_coord.clone());

    let filepaths_and_contents = match resolver.resolve(&needed_package_coord) {
      None => {
        panic!("Couldn't find: {:?}", needed_package_coord);
      }
      Some(filepath_to_code) => {
        let mut result = Vec::new();
        for (filepath, code) in filepath_to_code {
          let file_coord = interner.intern_file_coordinate(FileCoordinate {
            package_coord: needed_package_coord.clone(),
            filepath: filepath.clone(),
          });
          result.push((file_coord, code));
        }
        result
      }
    };

    let filepaths_map: HashMap<Arc<FileCoordinate>, String> = filepaths_and_contents
      .iter()
      .map(|(fc, code)| (fc.clone(), code.clone()))
      .collect();
    already_found_file_to_code.put_package(needed_package_coord.clone(), filepaths_map);

    for (file_coord, code) in filepaths_and_contents {
      let mut result_acc = Vec::new();

      let mut iter = LexingIterator::new(code.clone());
      let mut lexer = Lexer::new(interner.clone(), keywords.clone());

      iter.consume_comments_and_whitespace();

      let mut maybe_imports_accum: Option<Vec<ImportL>> = Some(Vec::new());
      let mut maybe_imports: Option<Vec<ImportL>> = None;

      // Imports must come first, so that we can ship these denizens off with all
      // their relevant imports.

      while !iter.at_end() {
        let denizen = match lexer.lex_denizen(&mut iter) {
          Err(e) => {
            return Err(FailedParse {
              code: code.clone(),
              file_coord: (*file_coord).clone(),
              error: e,
            })
          }
          Ok(x) => x,
        };
        iter.consume_comments_and_whitespace();

        match &denizen {
          IDenizenL::TopLevelImport(im) => {
            match &mut maybe_imports_accum {
              None => panic!("Imports must come before everything else"),
              Some(imports_accum) => imports_accum.push(im.clone()),
            }

            // This is where we could fire off another thread to do any parsing in parallel,
            // because we're still only partway through the lexing.
            let next_needed_package_coord = interner.intern_package_coordinate(PackageCoordinate {
              module: im.module_name.str.clone(),
              packages: im.package_steps.iter().map(|x| x.str.clone()).collect(),
            });

            if !started_packages.contains(&next_needed_package_coord) {
              unexplored_packages.insert(next_needed_package_coord);
            }

            let denizen_result = denizen_handler(&file_coord, &code, &[], &denizen);
            result_acc.push(denizen_result);
          }
          _ => {
            match maybe_imports_accum.take() {
              None => {}
              Some(imports_accum) => {
                maybe_imports = Some(imports_accum);
              }
            }

            let imports = maybe_imports
              .as_ref()
              .expect("maybe_imports should be Some");
            let denizen_result = denizen_handler(&file_coord, &code, imports, &denizen);
            result_acc.push(denizen_result);
          }
        }
      }

      // Add any remaining imports to unexplored packages
      if let Some(imports) = &maybe_imports {
        for import in imports {
          let import_coord = interner.intern_package_coordinate(PackageCoordinate {
            module: import.module_name.str.clone(),
            packages: import.package_steps.iter().map(|x| x.str.clone()).collect(),
          });
          if !started_packages.contains(&import_coord) {
            unexplored_packages.insert(import_coord);
          }
        }
      }

      let comments_ranges = iter.comments.clone();
      let file = file_handler(&file_coord, &code, &comments_ranges, &result_acc);
      files_acc.push(file);
    }
  }

  Ok(files_acc)
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
*/

/// Helper function that collects all denizens and files
/// From LexAndExplore.scala lines 12-40
pub fn lex_and_explore_and_collect<R>(
  interner: Arc<Interner>,
  keywords: Arc<Keywords>,
  packages: Vec<Arc<PackageCoordinate>>,
  resolver: &R,
) -> Result<
  (
    Vec<(Arc<FileCoordinate>, String, Vec<ImportL>, IDenizenL)>,
    Vec<(Arc<FileCoordinate>, String, Vec<RangeL>, Vec<IDenizenL>)>,
  ),
  FailedParse,
>
where
  R: IPackageResolver<HashMap<String, String>>,
{
  let mut denizens = Vec::new();
  let mut files = Vec::new();

  lex_and_explore(
    interner,
    keywords,
    packages,
    resolver,
    |file_coord, code, imports, denizen| {
      denizens.push((
        file_coord.clone(),
        code.to_string(),
        imports.to_vec(),
        denizen.clone(),
      ));
      denizen.clone()
    },
    |file_coord, code, ranges, denizens_in_file| {
      files.push((
        file_coord.clone(),
        code.to_string(),
        ranges.to_vec(),
        denizens_in_file.to_vec(),
      ));
    },
  )?;

  Ok((denizens, files))
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

/*
}
*/
