use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::ast::ScrambleLE;
use crate::lexing::errors::ParseError;
use crate::lexing::iterator::LexingIterator;
use crate::lexing::lexer::Lexer;
use crate::parsing::ast::{FileP, FileCoordinate, PackageCoordinate, IDenizenP, IExpressionPE, IRulexPR, ITemplexPT, PatternPP, StructP};
use crate::parsing::parser::Parser;
use crate::parsing::expression_parser::ExpressionParser;
use crate::parsing::templex_parser::TemplexParser;
use crate::parsing::pattern_parser::PatternParser;
use crate::parsing::scramble_iterator::ScrambleIterator;
use std::sync::{Arc, Mutex};

/// Test utilities for parsing tests
/// Mirrors dev/vale/parsing/TestParseUtils.scala

pub fn make_interner() -> Interner {
    Interner::new()
}

pub fn make_keywords(interner: &mut Interner) -> Keywords {
    Keywords::new(interner)
}

pub fn make_lexer(interner: Arc<Mutex<Interner>>, keywords: Arc<Keywords>) -> Lexer {
    Lexer::new(interner, keywords)
}

pub fn make_parser(interner: Arc<Mutex<Interner>>, keywords: Arc<Keywords>) -> Parser {
    Parser::new(interner, keywords)
}

/// Compile a Vale file and return the FileP AST
pub fn compile_file(code: &str) -> Result<FileP, ParseError> {
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);
    
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
    
    let empty_module = {
        let module_str = interner.lock().unwrap().intern("");
        Arc::new(module_str)
    };
    
    Ok(FileP {
        file_coord: FileCoordinate {
            package_coord: PackageCoordinate {
                module: empty_module,
                packages: vec![],
            },
            filepath: "test.vale".to_string(),
        },
        comments_ranges: vec![],
        denizens,
    })
}

/// Compile a Vale file and panic if it fails (for tests)
pub fn compile_file_expect(code: &str) -> FileP {
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
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);
    
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut expression_parser = ExpressionParser::new(keywords.clone());
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
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);
    
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut expression_parser = ExpressionParser::new(keywords.clone());
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
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);
    
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
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);
    
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
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);
    
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
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);
    
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

