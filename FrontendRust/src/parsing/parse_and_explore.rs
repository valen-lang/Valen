use crate::compile_options::GlobalOptions;
use crate::lexing::ast::{IDenizenL, ImportL, RangeL};
use crate::lexing::errors::FailedParse;
use crate::lexing::lex_and_explore;
use crate::parsing::ast::IDenizenP;
use crate::parsing::Parser;
use crate::utils::code_hierarchy::{FileCoordinate, IPackageResolver, PackageCoordinate};
use crate::{Interner, Keywords};
use std::collections::HashMap;
/*
package dev.vale.parsing

import dev.vale.lexing.{FailedParse, IDenizenL, ImportL, LexAndExplore, RangeL, TopLevelExportAsL, TopLevelFunctionL, TopLevelImplL, TopLevelImportL, TopLevelInterfaceL, TopLevelStructL}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.{FileP, IDenizenP, TopLevelExportAsP, TopLevelFunctionP, TopLevelImplP, TopLevelImportP, TopLevelInterfaceP, TopLevelStructP}
import dev.vale.von.{JsonSyntax, VonPrinter}
import dev.vale._

import scala.collection.immutable.Map

object ParseAndExplore {
*/

/*
  // This is a helper function that one doesn't need to use, but it can be handy and also
  // serves as a great example on how to use the parseAndExplore() method.
  def parseAndExploreAndCollect(
    interner: Interner,
    keywords: Keywords,
    _opts: GlobalOptions,
    parser: Parser,
    packages: Vector[PackageCoordinate],
    resolver: IPackageResolver[Map[String, String]]):
  Result[Accumulator[(String, FileP)], FailedParse] = {
    parseAndExplore[IDenizenP, (String, FileP)](
      interner, keywords, opts, parser, packages, resolver,
      (file, code, imports, denizen) => denizen,
      (file, code, commentRanges, denizens) => {
        (code, FileP(file, commentRanges.buildArray(), denizens.buildArray()))
      })
  }
*/

// From ParseAndExplore.scala lines 35-101: parseAndExplore
pub fn parse_and_explore<'a, D, F, R, HandleParsedDenizen, FileHandler>(
  interner: &'a Interner<'a>,
  keywords: &'a Keywords<'a>,
  _opts: GlobalOptions,
  parser: &mut Parser<'a, '_>,
  packages: Vec<&'a PackageCoordinate<'a>>,
  resolver: &R,
  mut handle_parsed_denizen: HandleParsedDenizen,
  mut file_handler: FileHandler,
) -> Result<Vec<F>, FailedParse>
where
  R: IPackageResolver<'a, HashMap<String, String>>,
  HandleParsedDenizen: FnMut(&FileCoordinate<'a>, &str, &[ImportL], IDenizenP) -> D,
  FileHandler: FnMut(&FileCoordinate<'a>, &str, &[RangeL], &[D]) -> F,
{
  // From ParseAndExplore.scala lines 45-100: Call lexAndExplore with parsing logic
  lex_and_explore::lex_and_explore(
    interner,
    keywords,
    packages,
    resolver,
    |file_coord: &FileCoordinate<'a>,
     code: &str,
     imports: &[ImportL],
     denizen_l: &IDenizenL|
     -> D {
      // From ParseAndExplore.scala lines 51-95: Parse each denizen type
      let denizen_p: IDenizenP = match denizen_l {
        IDenizenL::TopLevelImport(import) => {
          // From ParseAndExplore.scala lines 53-59
          IDenizenP::TopLevelImport(
            parser
              .parse_import(import.clone())
              .expect("parse_import failed - error handling not yet fully implemented"),
          )
        }
        IDenizenL::TopLevelFunction(function_l) => {
          // From ParseAndExplore.scala lines 60-66
          IDenizenP::TopLevelFunction(
            parser
              .parse_function(function_l.clone(), false)
              .expect("parse_function failed - error handling not yet fully implemented"),
          )
        }
        IDenizenL::TopLevelStruct(struct_l) => {
          // From ParseAndExplore.scala lines 67-73
          IDenizenP::TopLevelStruct(
            parser
              .parse_struct(struct_l.clone())
              .expect("parse_struct failed - error handling not yet fully implemented"),
          )
        }
        IDenizenL::TopLevelInterface(interface_l) => {
          // From ParseAndExplore.scala lines 74-80
          IDenizenP::TopLevelInterface(
            parser
              .parse_interface(interface_l.clone())
              .expect("parse_interface failed - error handling not yet fully implemented"),
          )
        }
        IDenizenL::TopLevelImpl(impl_l) => {
          // From ParseAndExplore.scala lines 81-87
          IDenizenP::TopLevelImpl(
            parser
              .parse_impl(impl_l.clone())
              .expect("parse_impl failed - error handling not yet fully implemented"),
          )
        }
        IDenizenL::TopLevelExportAs(export) => {
          // From ParseAndExplore.scala lines 88-94
          IDenizenP::TopLevelExportAs(
            parser
              .parse_export_as(export.clone())
              .expect("parse_export_as failed - error handling not yet fully implemented"),
          )
        }
      };
      // From ParseAndExplore.scala line 96
      handle_parsed_denizen(file_coord, code, imports, denizen_p)
    },
    |file_coord: &Arc<FileCoordinate>,
     code: &str,
     comment_ranges: &[RangeL],
     denizens: &[D]|
     -> F {
      // From ParseAndExplore.scala lines 98-100
      file_handler(file_coord, code, comment_ranges, denizens)
    },
  )
}
/*
  def parseAndExplore[D, F](
    interner: Interner,
    keywords: Keywords,
    opts: GlobalOptions,
    parser: Parser,
    packages: Vector[PackageCoordinate],
    resolver: IPackageResolver[Map[String, String]],
    handleParsedDenizen: (FileCoordinate, String, Vector[ImportL], IDenizenP) => D,
    fileHandler: (FileCoordinate, String, Accumulator[RangeL], Accumulator[D]) => F
  ): Result[Accumulator[F], FailedParse] = {
    LexAndExplore.lexAndExplore[D, F](
      interner,
      keywords,
      packages,
      resolver,
      (fileCoord: FileCoordinate, code: String, imports: Vector[ImportL], denizenL: IDenizenL) => {
        val denizenP: IDenizenP =
          denizenL match {
            case TopLevelImportL(imporrt) => {
              TopLevelImportP(
                parser.parseImport(imporrt) match {
                  case Err(e) => return Err(FailedParse(code, fileCoord, e))
                  case Ok(x) => x
                })
            }
            case TopLevelFunctionL(functionL) => {
              TopLevelFunctionP(
                parser.parseFunction(functionL, false) match {
                  case Err(e) => return Err(FailedParse(code, fileCoord, e))
                  case Ok(x) => x
                })
            }
            case TopLevelStructL(structL) => {
              TopLevelStructP(
                parser.parseStruct(structL) match {
                  case Err(e) => return Err(FailedParse(code, fileCoord, e))
                  case Ok(x) => x
                })
            }
            case TopLevelInterfaceL(interfaceL) => {
              TopLevelInterfaceP(
                parser.parseInterface(interfaceL) match {
                  case Err(e) => return Err(FailedParse(code, fileCoord, e))
                  case Ok(x) => x
                })
            }
            case TopLevelImplL(structL) => {
              TopLevelImplP(
                parser.parseImpl(structL) match {
                  case Err(e) => return Err(FailedParse(code, fileCoord, e))
                  case Ok(x) => x
                })
            }
            case TopLevelExportAsL(export) => {
              TopLevelExportAsP(
                parser.parseExportAs(export) match {
                  case Err(e) => return Err(FailedParse(code, fileCoord, e))
                  case Ok(x) => x
                })
            }
          }
        handleParsedDenizen(fileCoord, code, imports, denizenP)
      },
      (fileCoord: FileCoordinate, code: String, commentsRanges: Accumulator[RangeL], denizensAcc: Accumulator[D]) => {
        fileHandler(fileCoord, code, commentsRanges, denizensAcc)
      })
  }
*/
/*
}
*/
