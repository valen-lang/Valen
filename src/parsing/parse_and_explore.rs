use crate::compile_options::GlobalOptions;
use crate::lexing::ast::{IDenizenL, ImportL, RangeL};
use crate::lexing::errors::{FailedParse, ParseError};
use crate::lexing::lex_and_explore;
use crate::parsing::ast::IDenizenP;
use crate::parsing::Parser;
use crate::code_source::CodeSource;
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::Keywords;
use crate::parse_arena::ParseArena;

// VCOORD: revisit this, probably shouldnt do this
/// Unwraps a denizen parse, reporting the error itself rather than discarding it.
///
/// This closure's return type is the denizen, not a `Result`, so a parse failure can't be
/// propagated from here without restructuring `lex_and_explore`'s callback contract. Until
/// that happens the failure must still be loud — but it names the error, its position, and
/// the offending line, so a caller can act on it.
fn expect_parsed<'p, T>(
  result: Result<T, ParseError>,
  denizen_kind: &str,
  file_coord: &FileCoordinate<'p>,
  code: &str,
) -> T {
  match result {
    Ok(parsed) => parsed,
    Err(error) => {
      let pos = (error.pos().max(0) as usize).min(code.len());
      let line_begin = code.get(..pos).and_then(|before| before.rfind('\n')).map_or(0, |i| i + 1);
      let line_end = code.get(pos..).and_then(|after| after.find('\n')).map_or(code.len(), |i| pos + i);
      let line = code.get(line_begin..line_end).unwrap_or("<source unavailable>");
      panic!(
        "parse_{} failed [{}] at {:?} offset {}:\n  {}\n  {:>width$}\n{:?}",
        denizen_kind,
        error.error_id(),
        file_coord,
        pos,
        line,
        "^",
        error,
        width = pos - line_begin + 1,
      );
    }
  }
}


pub fn parse_and_explore<'p, 'ctx, D, F, HandleParsedDenizen, FileHandler>(
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  _opts: GlobalOptions,
  parser: &Parser<'p, 'ctx>,
  packages: Vec<&'p PackageCoordinate<'p>>,
  source: &CodeSource<'p>,
  mut handle_parsed_denizen: HandleParsedDenizen,
  mut file_handler: FileHandler,
) -> Result<Vec<F>, FailedParse<'p>>
where
  'p: 'ctx,
  HandleParsedDenizen: FnMut(&'p FileCoordinate<'p>, &str, &[ImportL<'p>], IDenizenP<'p>) -> D,
  FileHandler: FnMut(&'p FileCoordinate<'p>, &str, &[RangeL], Vec<D>) -> F,
{
  lex_and_explore::lex_and_explore(
    parse_arena,
    keywords,
    packages,
    source,
    |file_coord: &'p FileCoordinate<'p>,
     code: &str,
     imports: &[ImportL<'p>],
     denizen_l: &IDenizenL<'p>|
     -> D {
      let denizen_p: IDenizenP<'p> = match denizen_l {
        IDenizenL::TopLevelImport(import) => IDenizenP::TopLevelImport(expect_parsed(
          parser.parse_import(import.clone()), "import", file_coord, code)),
        IDenizenL::TopLevelFunction(function_l) => IDenizenP::TopLevelFunction(expect_parsed(
          parser.parse_function(function_l.clone(), false), "function", file_coord, code)),
        IDenizenL::TopLevelStruct(struct_l) => IDenizenP::TopLevelStruct(expect_parsed(
          parser.parse_struct(struct_l.clone()), "struct", file_coord, code)),
        IDenizenL::TopLevelInterface(interface_l) => IDenizenP::TopLevelInterface(expect_parsed(
          parser.parse_interface(interface_l.clone()), "interface", file_coord, code)),
        IDenizenL::TopLevelImpl(impl_l) => IDenizenP::TopLevelImpl(expect_parsed(
          parser.parse_impl(impl_l.clone()), "impl", file_coord, code)),
        IDenizenL::TopLevelExportAs(export) => IDenizenP::TopLevelExportAs(expect_parsed(
          parser.parse_export_as(export.clone()), "export_as", file_coord, code)),
      };
      handle_parsed_denizen(file_coord, code, imports, denizen_p)
    },
    |file_coord: &'p FileCoordinate<'p>,
     code: &str,
     comment_ranges: &[RangeL],
     denizens: Vec<D>|
     -> F {
      file_handler(file_coord, code, comment_ranges, denizens)
    },
  )
}
