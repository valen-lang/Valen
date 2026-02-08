// TODO: move to tests/test_parse_utils.rs
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::lexing::lexing_iterator::LexingIterator;
use crate::lexing::lexer::Lexer;
use crate::parsing::ast::{FileP, IDenizenP, IExpressionPE, IRulexPR, ITemplexPT, PatternPP, StructP};
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::parsing::parser::Parser;
use crate::parsing::expression_parser::ExpressionParser;
use crate::parsing::templex_parser::TemplexParser;
use crate::parsing::pattern_parser::PatternParser;
use crate::parsing::scramble_iterator::ScrambleIterator;
use std::sync::{Arc};

/// Test utilities for parsing tests
/// Mirrors dev/vale/parsing/TestParseUtils.scala

pub fn make_interner() -> Arc<Interner> {
    Arc::new(Interner::new())
}

pub fn make_keywords(interner: &Arc<Interner>) -> Arc<Keywords> {
    Arc::new(Keywords::new(interner))
}

pub fn make_lexer(interner: Arc<Interner>, keywords: Arc<Keywords>) -> Lexer {
    Lexer::new(interner, keywords)
}

pub fn make_parser(interner: Arc<Interner>, keywords: Arc<Keywords>) -> Parser {
    Parser::new(interner, keywords)
}

/// Compile a Vale file and return the FileP AST
pub fn compile_file(code: &str) -> Result<FileP, ParseError> {
    let interner = Arc::new(Interner::new());
    let keywords = Arc::new(Keywords::new(&interner));
    
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut parser = Parser::new(interner.clone(), keywords.clone());
    
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
    
    let package_coord = interner.intern_package_coordinate(PackageCoordinate {
        module: empty_module,
        packages: vec![],
    });
    
    let file_coord = interner.intern_file_coordinate(FileCoordinate {
        package_coord,
        filepath: "test.vale".to_string(),
    });
    
    Ok(FileP {
        file_coord,
        comments_ranges: vec![],
        denizens,
    })
}

/// Compile a Vale file and panic if it fails (for tests)
pub fn compile(code: &str) -> FileP {
    compile_file(code).unwrap_or_else(|e| panic!("Failed to parse file: {:?}", e))
}

/// Compile denizens (top-level declarations) from code
pub fn compile_denizens(code: &str) -> Result<Vec<IDenizenP>, ParseError> {
    compile_file(code).map(|file| file.denizens)
}

/// Compile a single denizen from code
pub fn compile_denizen(code: &str) -> Result<IDenizenP, ParseError> {
    let denizens = compile_denizens(code)?;
    assert_eq!(denizens.len(), 1, "Expected exactly one denizen");
    Ok(denizens.into_iter().next().unwrap())
}

/// Compile a single denizen and panic if it fails
pub fn compile_denizen_expect(code: &str) -> IDenizenP {
    compile_denizen(code).unwrap_or_else(|e| panic!("Failed to parse denizen: {:?}", e))
}

/// Compile a single denizen and expect it to fail with an error, passing the error to a callback
pub fn compile_denizen_for_error<F>(code: &str, callback: F)
where
    F: FnOnce(ParseError),
{
    match compile_denizen(code) {
        Ok(_) => panic!("Expected parsing to fail, but it succeeded"),
        Err(e) => callback(e),
    }
}

/// Compile an expression from code
pub fn compile_expression(code: &str) -> Result<IExpressionPE, ParseError> {
    let interner = Arc::new(Interner::new());
    let keywords = Arc::new(Keywords::new(&interner));
    
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut expression_parser = ExpressionParser::new(interner.clone(), keywords.clone());
    let mut templex_parser = TemplexParser::new(interner.clone(), keywords.clone());
    let mut pattern_parser = PatternParser::new(interner.clone(), keywords.clone());
    
    let mut iter_for_lex = LexingIterator::new(code.to_string());
    let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, false)?;
    let mut iter = ScrambleIterator::new(scramble);
    expression_parser.parse_expression(&mut iter, false, &mut templex_parser, &mut pattern_parser)
}

/// Compile an expression and panic if it fails
pub fn compile_expression_expect(code: &str) -> IExpressionPE {
    compile_expression(code).unwrap_or_else(|e| panic!("Failed to parse expression: {:?}", e))
}

/// Compile an expression and expect it to fail, returning the error
pub fn compile_expression_for_error(code: &str) -> ParseError {
    compile_expression(code).expect_err("Expected parsing to fail")
}

/// Compile a statement from code
pub fn compile_statement(code: &str) -> Result<IExpressionPE, ParseError> {
    let interner = Arc::new(Interner::new());
    let keywords = Arc::new(Keywords::new(&interner));
    
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut expression_parser = ExpressionParser::new(interner.clone(), keywords.clone());
    let mut templex_parser = TemplexParser::new(interner.clone(), keywords.clone());
    let mut pattern_parser = PatternParser::new(interner.clone(), keywords.clone());
    
    let mut iter_for_lex = LexingIterator::new(code.to_string());
    let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, false)?;
    let mut iter = ScrambleIterator::new(scramble);
    expression_parser.parse_statement(&mut iter, false, &mut templex_parser, &mut pattern_parser)
}

/// Compile a statement and panic if it fails
pub fn compile_statement_expect(code: &str) -> IExpressionPE {
    compile_statement(code).unwrap_or_else(|e| panic!("Failed to parse statement: {:?}", e))
}

/// Compile block contents from code
pub fn compile_block_contents(code: &str) -> Result<IExpressionPE, ParseError> {
    let interner = Arc::new(Interner::new());
    let keywords = Arc::new(Keywords::new(&interner));
    
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut parser = Parser::new(interner.clone(), keywords.clone());
    
    let mut iter_for_lex = LexingIterator::new(code.to_string());
    let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, false)?;
    let mut iter = ScrambleIterator::new(scramble);
    parser.expression_parser.parse_block_contents(&mut iter, false, &mut parser.templex_parser, &mut parser.pattern_parser)
}

/// Compile block contents and panic if it fails
pub fn compile_block_contents_expect(code: &str) -> IExpressionPE {
    compile_block_contents(code).unwrap_or_else(|e| panic!("Failed to parse block contents: {:?}", e))
}

/// Compile a pattern from code
pub fn compile_pattern(code: &str) -> Result<PatternPP, ParseError> {
    let interner = Arc::new(Interner::new());
    let keywords = Arc::new(Keywords::new(&interner));
    
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut parser = Parser::new(interner.clone(), keywords.clone());
    
    let mut iter_for_lex = LexingIterator::new(code.to_string());
    let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, false)?;
    let begin = scramble.range.begin;
    let mut iter = ScrambleIterator::new(scramble);
    parser.pattern_parser.parse_pattern(&mut iter, &mut parser.templex_parser, begin, 0, false, false, false, None)
}

/// Compile a pattern and panic if it fails
pub fn compile_pattern_expect(code: &str) -> PatternPP {
    compile_pattern(code).unwrap_or_else(|e| panic!("Failed to parse pattern: {:?}", e))
}

/// Compile a templex (type expression) from code
pub fn compile_templex(code: &str) -> Result<ITemplexPT, ParseError> {
    let interner = Arc::new(Interner::new());
    let keywords = Arc::new(Keywords::new(&interner));
    
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut parser = Parser::new(interner.clone(), keywords.clone());
    
    let mut iter_for_lex = LexingIterator::new(code.to_string());
    let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, true)?;
    let mut iter = ScrambleIterator::new(scramble);
    parser.templex_parser.parse_templex(&mut iter)
}

/// Compile a templex and panic if it fails
pub fn compile_templex_expect(code: &str) -> ITemplexPT {
    compile_templex(code).unwrap_or_else(|e| panic!("Failed to parse templex: {:?}", e))
}

/// Compile a rulex (rule expression) from code
pub fn compile_rulex(code: &str) -> Result<IRulexPR, ParseError> {
    let interner = Arc::new(Interner::new());
    let keywords = Arc::new(Keywords::new(&interner));
    
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut parser = Parser::new(interner.clone(), keywords.clone());
    
    let mut iter_for_lex = LexingIterator::new(code.to_string());
    let scramble = lexer.lex_scramble(&mut iter_for_lex, false, false, true)?;
    let mut iter = ScrambleIterator::new(scramble);
    parser.templex_parser.parse_rule(&mut iter)
}

/// Compile a rulex and panic if it fails
pub fn compile_rulex_expect(code: &str) -> IRulexPR {
    compile_rulex(code).unwrap_or_else(|e| panic!("Failed to parse rulex: {:?}", e))
}

/// Compile a struct from code
pub fn compile_struct(code: &str) -> Result<StructP, ParseError> {
    let denizen = compile_denizen(code)?;
    match denizen {
        IDenizenP::TopLevelStruct(s) => Ok(s),
        _ => panic!("Expected TopLevelStruct, got: {:?}", denizen),
    }
}

/// Compile a struct and panic if it fails
pub fn compile_struct_expect(code: &str) -> StructP {
    compile_struct(code).unwrap_or_else(|e| panic!("Failed to parse struct: {:?}", e))
}

/// Helper macro for pattern matching in tests
/// Mirrors Scala's `shouldHave` pattern matching
/// EXCEPT that this doesn't look deeply. This is just a surface-level match.
#[macro_export]
macro_rules! should_have {
    // Pattern with guard
    ($expr:expr, $pattern:pat if $guard:expr => $body:expr) => {
        match $expr {
            $pattern if $guard => $body,
            ref other => panic!("Pattern did not match. Got: {:?}", other),
        }
    };
    // Pattern without guard
    ($expr:expr, $pattern:pat => $body:expr) => {
        match $expr {
            $pattern => $body,
            ref other => panic!("Pattern did not match. Got: {:?}", other),
        }
    };
}

/// Helper macro for checking if a value matches a pattern
#[macro_export]
macro_rules! matches_pattern {
    ($expr:expr, $pattern:pat) => {
        matches!($expr, $pattern)
    };
    ($expr:expr, $pattern:pat if $guard:expr) => {
        matches!($expr, $pattern if $guard)
    };
}

/*
package dev.vale.parsing

import dev.vale.lexing.{FailedParse, IParseError, Lexer, LexingIterator}
import dev.vale.{Err, FileCoordinate, FileCoordinateMap, IPackageResolver, Interner, Keywords, Ok, PackageCoordinate, PackageCoordinateMap, Result, SourceCodeUtils, U, vassertOne, vassertSome, vfail, vimpl}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.{FileP, IDenizenP, IExpressionPE, IRulexPR, ITemplexPT, PatternPP}
import dev.vale.parsing.templex.TemplexParser

import scala.collection.immutable.Map

trait TestParseUtils {
  def compileMaybe[T](parser: (ScrambleIterator) => Result[Option[T], IParseError], untrimpedCode: String): T = {
    vimpl()
//    val interner = new Interner()
//    val code = untrimpedCode.trim()
//    // The trim is in here because things inside the parser don't expect whitespace before and after
//    val iter = new ScrambleIterator(code.trim(), 0)
//    parser(iter) match {
//      case Ok(None) => {
//        vfail("Couldn't parse, not applicable!");
//      }
//      case Err(err) => {
//        vfail(
//          "Couldn't parse!\n" +
//            ParseErrorHumanizer.humanize(
//              FileCoordinateMap.test(interner, code).fileCoordToContents.toMap,
//              FileCoordinate.test(interner),
//              err))
//      }
//      case Ok(Some(result)) => {
//        if (!iter.atEnd()) {
//          vfail("Couldn't parse all of the input. Remaining:\n" + code.slice(iter.getPos(), code.length))
//        }
//        result
//      }
//    }
  }

  def compile[T](parser: (LexingIterator) => Result[T, IParseError], untrimpedCode: String): T = {
    vimpl()
//    val interner = new Interner()
//    val code = untrimpedCode.trim()
//    // The trim is in here because things inside the parser don't expect whitespace before and after
//    val iter = ParsingIterator(code.trim(), 0)
//    parser(iter) match {
//      case Err(err) => {
//        vfail(
//          "Couldn't parse!\n" +
//            ParseErrorHumanizer.humanize(
//              FileCoordinateMap.test(interner, code).fileCoordToContents.toMap,
//              FileCoordinate.test(interner),
//              err))
//      }
//      case Ok(result) => {
//        if (!iter.atEnd()) {
//          vfail("Couldn't parse all of the input. Remaining:\n" + code.slice(iter.getPos(), code.length))
//        }
//        result
//      }
//    }
  }

  def compileForError[T](parser: (ScrambleIterator) => Result[T, IParseError], untrimpedCode: String): IParseError = {
    vimpl()
//    val code = untrimpedCode.trim()
//    // The trim is in here because things inside the parser don't expect whitespace before and after
//    val iter = ParsingIterator(code.trim(), 0)
//    parser(iter) match {
//      case Err(err) => {
//        err
//      }
//      case Ok(result) => {
//        if (!iter.atEnd()) {
//          vfail("Couldn't parse all of the input. Remaining:\n" + code.slice(iter.getPos(), code.length))
//        }
//        vfail("We expected parse to fail, but it succeeded:\n" + result)
//      }
//    }
  }

  def compileForRest[T](parser: (ScrambleIterator) => Result[T, IParseError], untrimpedCode: String, expectedRest: String): Unit = {
    vimpl()
//    val interner = new Interner()
//    val code = untrimpedCode.trim()
//    // The trim is in here because things inside the parser don't expect whitespace before and after
//    val iter = ParsingIterator(code.trim(), 0)
//    parser(iter) match {
//      case Err(err) => {
//        vfail(
//          "Couldn't parse!\n" +
//            ParseErrorHumanizer.humanize(
//              FileCoordinateMap.test(interner, code).fileCoordToContents.toMap,
//              FileCoordinate.test(interner),
//              err))
//      }
//      case Ok(_) => {
//        val rest = iter.code.slice(iter.position, iter.code.length)
//        if (rest != expectedRest) {
//          vfail("Couldn't parse all of the input. Remaining:\n" + code.slice(iter.getPos(), code.length))
//        }
//      }
//    }
  }

  def compileExpressionForError(code: String): IParseError = {
    compileExpression(code).expectErr()
  }

  def makeParser(): Parser = {
    vimpl()
//    new Parser(GlobalOptions(true, true, true, true))
  }

  def makeExpressionParser(): ExpressionParser = {
    vimpl()
//    new ExpressionParser(GlobalOptions(true, true, true, true))
  }

  def compileFileInner(
    interner: Interner,
    keywords: Keywords,
    codeMap: FileCoordinateMap[String]):
  Result[FileP, FailedParse] = {
    val opts = GlobalOptions(true, true, true, true, true)
    val p = new Parser(interner, keywords, opts)
    ParseAndExplore.parseAndExploreAndCollect(
      interner,
      keywords,
      opts,
      p,
      Vector(PackageCoordinate.TEST_TLD(interner, keywords)),
      new IPackageResolver[Map[String, String]]() {
        override def resolve(packageCoord: PackageCoordinate): Option[Map[String, String]] = {
          // For testing the parser, we dont want it to fetch things with import statements
          Some(codeMap.resolve(packageCoord).getOrElse(Map()))
        }
      }) match {
      case Err(e) => Err(e)
      case Ok(x) => Ok(vassertOne(U.map[(String, FileP), FileP](x.buildArray(), _._2)))
    }
  }

  def compileFile(code: String): Result[FileP, FailedParse] = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val codeMap = FileCoordinateMap.test(interner, Vector(code))
    compileFileInner(interner, keywords, codeMap)
  }

  def compileFileExpect(code: String): FileP = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val codeMap = FileCoordinateMap.test(interner, Vector(code))
    compileFileInner(interner, keywords, codeMap) match {
      case Err(e) => vfail(ParseErrorHumanizer.humanizeFromMap(codeMap.fileCoordToContents.toMap, e.fileCoord, e.error))
      case Ok(x) => x
    }
  }

  def compileDenizens(code: String): Result[Vector[IDenizenP], FailedParse] = {
    compileFile(code) match {
      case Err(e) => Err(e)
      case Ok(x) => Ok(x.denizens)
    }
  }


  def compileDenizen(code: String): Result[IDenizenP, FailedParse] = {
    compileDenizens(code) match {
      case Err(e) => Err(e)
      case Ok(x) => Ok(vassertOne(x))
    }
  }

  def compileDenizenExpect(code: String): IDenizenP = {
    compileDenizen(code) match {
      case Err(FailedParse(code, fileCoord, error)) => {
        vfail(
          ParseErrorHumanizer.humanize(
            SourceCodeUtils.humanizeFile(fileCoord), code, error))
      }
      case Ok(x) => x
    }
  }

  def compileExpression(code: String): Result[IExpressionPE, IParseError] = {
    val opts = GlobalOptions(true, false, false, true, true)
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val lexer = new Lexer(interner, keywords)
    val node =
      lexer.lexScramble(LexingIterator(code), false, false, false)
        .getOrDie()
    val parser = new Parser(interner, keywords, opts)
    parser.expressionParser.parseExpression(new ScrambleIterator(node), false)
  }

  def compileExpressionExpect(code: String): IExpressionPE = {
    compileExpression(code).getOrDie()
  }

  def compileBlockContents(code: String): Result[IExpressionPE, IParseError] = {
    val opts = GlobalOptions(true, false, false, true, true)
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val lexer = new Lexer(interner, keywords)
    val iter = LexingIterator(code)
    val node =
      lexer.lexScramble(iter, false, false, false)
        .getOrDie()
    val parser = new Parser(interner, keywords, opts)
    parser.expressionParser.parseBlockContents(new ScrambleIterator(node), false)
  }

  def compileBlockContentsExpect(code: String): IExpressionPE = {
    compileBlockContents(code).getOrDie()
  }

  def compileStatement(code: String): Result[IExpressionPE, IParseError] = {
    val opts = GlobalOptions(true, false, false, true, true)
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val lexer = new Lexer(interner, keywords)
    val node =
      lexer.lexScramble(LexingIterator(code), false, false, false) match {
        case Err(e) => return Err(e)
        case Ok(x) => x
      }
    val parser = new Parser(interner, keywords, opts)
    parser.expressionParser.parseStatement(new ScrambleIterator(node), false)
  }

  def compileStatementExpect(code: String): IExpressionPE = {
    compileStatement(code).getOrDie()
  }

  def compilePattern(code: String): PatternPP = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val lexer = new Lexer(interner, keywords)
    val node =
      lexer.lexScramble(LexingIterator(code), false, false, false)
        .getOrDie()
    val templexParser = new TemplexParser(interner, keywords)
    val exprP =
      new PatternParser(interner, keywords, templexParser)
        .parsePattern(new ScrambleIterator(node), node.range.begin, 0, false, false, false, None)
        .getOrDie()
    exprP
  }

  def compileTemplex(code: String): ITemplexPT = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val node =
      new Lexer(interner, keywords)
        .lexScramble(LexingIterator(code), false, false, true)
        .getOrDie()
    val exprP =
        new TemplexParser(interner, keywords)
          .parseTemplex(new ScrambleIterator(node))
          .getOrDie()
    exprP
  }

  def compileRulex(code: String): IRulexPR = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val node =
      new Lexer(interner, keywords)
        .lexScramble(LexingIterator(code), false, false, true)
        .getOrDie()
    val exprP =
          new TemplexParser(interner, keywords)
            .parseRule(new ScrambleIterator(node))
            .getOrDie()
    exprP
  }
}
*/