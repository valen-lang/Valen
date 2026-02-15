use bumpalo::Bump;
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::lexing::lexer::Lexer;
use crate::lexing::lexing_iterator::LexingIterator;
use crate::parsing::ast::*;
use crate::parsing::expression_parser::ExpressionParser;
use crate::parsing::parser::Parser;
use crate::parsing::pattern_parser::PatternParser;
use crate::parsing::scramble_iterator::ScrambleIterator;
use crate::parsing::templex_parser::TemplexParser;
use crate::parsing::tests::traverse::NodeRefP;

/// MIGTODO: Remove this function and use the one in ParserTestCompilation.scala instead
/// so that it does a round-trip through vonprinter and parsedloader.
/// Compile a Vale file and return the FileP AST
pub fn compile_file<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<FileP<'a>, ParseError>
where
  'a: 'ctx,
{
  let lexer = Lexer::new(interner, keywords);
  let parser = Parser::new(interner, keywords);

  // Lex the entire file
  let mut iter_for_lex = LexingIterator::new(code.to_string());

  // Parse denizens one by one
  let mut denizens = Vec::new();
  while !iter_for_lex.at_end() {
    iter_for_lex.consume_comments_and_whitespace();
    if iter_for_lex.at_end() {
      break;
    }
    let denizen_l = lexer.lex_denizen(&mut iter_for_lex)?;
    let denizen_p = parser.parse_denizen(denizen_l)?;
    denizens.push(denizen_p);
  }

  let empty_module = interner.intern("");

  let package_coord = interner.intern_package_coordinate(empty_module, &[]);

  let file_coord = interner.intern_file_coordinate(package_coord, "test.vale");

  Ok(FileP { file_coord: file_coord, comments_ranges: vec![], denizens })
}

/// Compile a Vale file and panic if it fails (for tests)
pub fn compile<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> FileP<'a>
where
  'a: 'ctx,
{
  compile_file(interner, keywords, code).unwrap_or_else(|e| panic!("Failed to parse file: {:?}", e))
}

/// Compile denizens (top-level declarations) from code
pub fn compile_denizens<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<Vec<IDenizenP<'a>>, ParseError>
where
  'a: 'ctx,
{
  compile_file(interner, keywords, code).map(|file| file.denizens)
}

/// Compile a single denizen from code
pub fn compile_denizen<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<IDenizenP<'a>, ParseError>
where
  'a: 'ctx,
{
  let denizens = compile_denizens(interner, keywords, code)?;
  assert_eq!(denizens.len(), 1, "Expected exactly one denizen");
  Ok(denizens.into_iter().next().unwrap())
}

/// Compile a single denizen and panic if it fails
pub fn compile_denizen_expect<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> IDenizenP<'a>
where
  'a: 'ctx,
{
  compile_denizen(interner, keywords, code).unwrap_or_else(|e| panic!("Failed to parse denizen: {:?}", e))
}

/// Compile a single denizen and expect it to fail with an error, passing the error to a callback
pub fn compile_denizen_for_error<'a, 'ctx, F>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
  callback: F,
)
where
  'a: 'ctx,
  F: FnOnce(ParseError),
{
  match compile_denizen(interner, keywords, code) {
    Ok(_) => panic!("Expected parsing to fail, but it succeeded"),
    Err(e) => callback(e),
  }
}

/// Compile an expression from code
pub fn compile_expression<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<IExpressionPE<'a>, ParseError>
where
  'a: 'ctx,
{
  let lexer = Lexer::new(interner, keywords);
  let expression_parser = ExpressionParser::new(interner, keywords);
  let mut templex_parser = TemplexParser::new(interner, keywords);
  let mut pattern_parser = PatternParser::new(interner, keywords);

  let mut iter_for_lex = LexingIterator::new(code.to_string());
  let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, false)?;
  let mut iter = ScrambleIterator::new(scramble);
  expression_parser.parse_expression(&mut iter, false, &mut templex_parser, &mut pattern_parser)
}

/// Compile an expression and panic if it fails
pub fn compile_expression_expect<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> IExpressionPE<'a>
where
  'a: 'ctx,
{
  compile_expression(interner, keywords, code).unwrap_or_else(|e| panic!("Failed to parse expression: {:?}", e))
}

/// Compile an expression and expect it to fail, returning the error
pub fn compile_expression_for_error<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> ParseError
where
  'a: 'ctx,
{
  compile_expression(interner, keywords, code).expect_err("Expected parsing to fail")
}

/// Compile a statement from code
pub fn compile_statement<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<IExpressionPE<'a>, ParseError>
where
  'a: 'ctx,
{
  let lexer = Lexer::new(interner, keywords);
  let expression_parser = ExpressionParser::new(interner, keywords);
  let mut templex_parser = TemplexParser::new(interner, keywords);
  let mut pattern_parser = PatternParser::new(interner, keywords);

  let mut iter_for_lex = LexingIterator::new(code.to_string());
  let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, false)?;
  let mut iter = ScrambleIterator::new(scramble);
  expression_parser.parse_statement(&mut iter, false, &mut templex_parser, &mut pattern_parser)
}

/// Compile a statement and panic if it fails
pub fn compile_statement_expect<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> IExpressionPE<'a>
where
  'a: 'ctx,
{
  compile_statement(interner, keywords, code).unwrap_or_else(|e| panic!("Failed to parse statement: {:?}", e))
}

/// Compile block contents from code
pub fn compile_block_contents<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<IExpressionPE<'a>, ParseError>
where
  'a: 'ctx,
{
  let lexer = Lexer::new(interner, keywords);
  let mut parser = Parser::new(interner, keywords);

  let mut iter_for_lex = LexingIterator::new(code.to_string());
  let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, false)?;
  let mut iter = ScrambleIterator::new(scramble);
  parser.expression_parser.parse_block_contents(
    &mut iter,
    false,
    &mut parser.templex_parser,
    &mut parser.pattern_parser,
  )
}

/// Compile block contents and panic if it fails
pub fn compile_block_contents_expect<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> IExpressionPE<'a>
where
  'a: 'ctx,
{
  compile_block_contents(interner, keywords, code).unwrap_or_else(|e| panic!("Failed to parse block contents: {:?}", e))
}

/// Compile a pattern from code
pub fn compile_pattern<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<PatternPP<'a>, ParseError>
where
  'a: 'ctx,
{
  let lexer = Lexer::new(interner, keywords);
  let mut parser = Parser::new(interner, keywords);

  let mut iter_for_lex = LexingIterator::new(code.to_string());
  let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, false)?;
  let begin = scramble.range.begin;
  let mut iter = ScrambleIterator::new(scramble);
  parser.pattern_parser.parse_pattern(
    &mut iter,
    &mut parser.templex_parser,
    begin,
    0,
    false,
    false,
    false,
    None,
  )
}

/// Compile a pattern and panic if it fails
pub fn compile_pattern_expect<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> PatternPP<'a>
where
  'a: 'ctx,
{
  compile_pattern(interner, keywords, code).unwrap_or_else(|e| panic!("Failed to parse pattern: {:?}", e))
}

/// Compile a templex (type expression) from code
pub fn compile_templex<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<ITemplexPT<'a>, ParseError>
where
  'a: 'ctx,
{
  let lexer = Lexer::new(interner, keywords);
  let parser = Parser::new(interner, keywords);

  let mut iter_for_lex = LexingIterator::new(code.to_string());
  let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, true)?;
  let mut iter = ScrambleIterator::new(scramble);
  parser.templex_parser.parse_templex(&mut iter)
}

/// Compile a templex and panic if it fails
pub fn compile_templex_expect<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> ITemplexPT<'a>
where
  'a: 'ctx,
{
  compile_templex(interner, keywords, code).unwrap_or_else(|e| panic!("Failed to parse templex: {:?}", e))
}

/// Compile a rulex (rule expression) from code
pub fn compile_rulex<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<IRulexPR<'a>, ParseError>
where
  'a: 'ctx,
{
  let lexer = Lexer::new(interner, keywords);
  let parser = Parser::new(interner, keywords);

  let mut iter_for_lex = LexingIterator::new(code.to_string());
  let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, true)?;
  let mut iter = ScrambleIterator::new(scramble);
  parser.templex_parser.parse_rule(&mut iter)
}

/// Compile a rulex and panic if it fails
pub fn compile_rulex_expect<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> IRulexPR<'a>
where
  'a: 'ctx,
{
  compile_rulex(interner, keywords, code).unwrap_or_else(|e| panic!("Failed to parse rulex: {:?}", e))
}

/// Compile a struct from code
pub fn compile_struct<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> Result<StructP<'a>, ParseError>
where
  'a: 'ctx,
{
  let denizen = compile_denizen(interner, keywords, code)?;
  match denizen {
    IDenizenP::TopLevelStruct(s) => Ok(s),
    _ => panic!("Expected TopLevelStruct, got: {:?}", denizen),
  }
}

/// Compile a struct and panic if it fails
pub fn compile_struct_expect<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> StructP<'a>
where
  'a: 'ctx,
{
  compile_struct(interner, keywords, code).unwrap_or_else(|e| panic!("Failed to parse struct: {:?}", e))
}

/// Returns the function with the given name.
/// See test_find_func_named_returns_function for an example.
pub fn find_func_named<'a, 'p>(file: &'p FileP<'a>, name: &str) -> &'p FunctionP<'a> {
  crate::collect_only!(
      file,
      NodeRefP::Function(function @ FunctionP {
          header: FunctionHeaderP {
              name: Some(NameP { str: ref s, .. }),
              ..
          },
          ..
      }) if s.str == name => Some(function)
  )
}
#[test]
fn test_find_func_named_returns_function() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, "exported func main() int {}");
  let main_function = find_func_named(&program, "main");
  assert!(main_function.header.params.as_ref().unwrap().params.is_empty());
}

/// Returns the struct with the given name. See find_func_named's test for a similar example.
pub fn find_struct_named<'a, 'f>(file: &'f FileP<'a>, name: &str) -> &'f StructP<'a> {
  crate::collect_only!(
      file,
      NodeRefP::Struct(struct_ @ StructP {
          name: NameP { str: ref s, .. },
          ..
      }) if s.str == name => Some(struct_)
  )
}

pub fn compile_for_error<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> ParseError
where
  'a: 'ctx,
{
  compile_file(interner, keywords, code).expect_err("Should be error")
}

pub fn assert_lookup_name(expr: &IExpressionPE, expected: &str) {
  let lookup = cast!(expr, IExpressionPE::Lookup);
  assert_name(&lookup.name, expected);
  assert!(lookup.template_args.is_none());
}

pub fn assert_templex_name(expr: &ITemplexPT, expected: &str) {
  let templex_lookup = cast!(expr, ITemplexPT::NameOrRune);
  assert_eq!(templex_lookup.name.str.str, expected);
}

pub fn assert_name(name: &IImpreciseNameP, expected: &str) {
  let lookup_name = cast!(name, IImpreciseNameP::LookupName);
  assert_eq!(lookup_name.str.str, expected);
}

pub fn assert_destination_local_name(destination: &DestinationLocalP, expected: &str) {
  let local_name = cast!(&destination.decl, INameDeclarationP::LocalNameDeclaration);
  assert_eq!(local_name.str.str, expected);
}

/// Asserts that a slice has exactly 1 element and returns it.
pub fn expect_1<T>(elements: &[T]) -> &T {
  assert_eq!(elements.len(), 1, "Expected exactly 1 element, got {}", elements.len());
  &elements[0]
}

/// Asserts that a slice has exactly 2 elements and returns them.
pub fn expect_2<T>(elements: &[T]) -> (&T, &T) {
  assert_eq!(elements.len(), 2, "Expected exactly 2 elements, got {}", elements.len());
  (&elements[0], &elements[1])
}

/// Asserts that a slice has exactly 3 elements and returns them.
pub fn expect_3<T>(elements: &[T]) -> (&T, &T, &T) {
  assert_eq!(elements.len(), 3, "Expected exactly 3 elements, got {}", elements.len());
  (&elements[0], &elements[1], &elements[2])
}

/// Asserts that a slice has exactly 4 elements and returns them.
pub fn expect_4<T>(elements: &[T]) -> (&T, &T, &T, &T) {
  assert_eq!(elements.len(), 4, "Expected exactly 4 elements, got {}", elements.len());
  (&elements[0], &elements[1], &elements[2], &elements[3])
}

/// Asserts that a slice has exactly 5 elements and returns them.
pub fn expect_5<T>(elements: &[T]) -> (&T, &T, &T, &T, &T) {
  assert_eq!(elements.len(), 5, "Expected exactly 5 elements, got {}", elements.len());
  (
    &elements[0],
    &elements[1],
    &elements[2],
    &elements[3],
    &elements[4],
  )
}

/// Asserts that a slice has exactly 6 elements and returns them.
pub fn expect_6<T>(elements: &[T]) -> (&T, &T, &T, &T, &T, &T) {
  assert_eq!(elements.len(), 6, "Expected exactly 6 elements, got {}", elements.len());
  (
    &elements[0],
    &elements[1],
    &elements[2],
    &elements[3],
    &elements[4],
    &elements[5],
  )
}

/// Unwraps an ESCCD-compliant enum (single field enum) and returns a reference to the thing
/// inside. Intended for use in tests only.
#[macro_export]
macro_rules! cast {
  ($value:expr, $variant:path) => {{
    match $value {
      $variant(inner) => inner,
      actual => panic!("expected {}, got {:?}", stringify!($variant), actual),
    }
  }};
}
