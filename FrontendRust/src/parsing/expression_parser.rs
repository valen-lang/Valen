use crate::keywords::Keywords;
use crate::StrI;
use crate::lexing::ast::*;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::parse_utils::try_skip_past_equals_while;
use crate::parsing::parse_utils::try_skip_past_keyword_while;
use crate::parsing::pattern_parser::PatternParser;
use crate::parsing::templex_parser::TemplexParser;
use crate::parse_arena::ParseArena;
/*
package dev.vale.parsing

import dev.vale.options.GlobalOptions
import dev.vale.parsing.templex.TemplexParser
import ExpressionParser.{MAX_PRECEDENCE, MIN_PRECEDENCE}
import dev.vale.parsing.ast._
import dev.vale.{Accumulator, Err, Interner, Keywords, Ok, Profiler, Result, StrI, U, parsing, vassert, vcurious, vfail, vimpl, vwat}
import dev.vale.parsing.ast._
import dev.vale.lexing._

import scala.collection.immutable.{List, Map}
import scala.collection.mutable
import scala.util.matching.Regex
*/
/*
sealed trait IStopBefore
case object StopBeforeComma extends IStopBefore
case object StopBeforeFileEnd extends IStopBefore
case object StopBeforeCloseBrace extends IStopBefore
case object StopBeforeCloseParen extends IStopBefore
case object StopBeforeEquals extends IStopBefore
case object StopBeforeCloseSquare extends IStopBefore
case object StopBeforeCloseChevron extends IStopBefore
// Such as after the if's condition or the foreach's iterable.
case object StopBeforeOpenBrace extends IStopBefore
*/

/*
sealed trait IExpressionElement
case class DataElement(expr: IExpressionPE) extends IExpressionElement
case class BinaryCallElement(symbol: NameP, precedence: Int) extends IExpressionElement
*/


type ParseResult<T> = Result<T, ParseError>;

/*
object ExpressionParser {
  val MAX_PRECEDENCE = 6
  val MIN_PRECEDENCE = 1
}
*/

// Helper enum for expression parsing
#[derive(Debug)]
enum ExpressionElement<'p> {
  Data(&'p IExpressionPE<'p>),
  BinaryCall(NameP<'p>, i32), // name and precedence
}

/// Iterator over a scramble of lexed nodes
/// Matches Scala's ScrambleIterator (holds reference to scramble, like Scala)
#[derive(Clone, Debug)]
pub struct ScrambleIterator<'p, 's> {
  pub scramble: &'s ScrambleLE<'p>,
  pub index: usize,
  pub end: usize,
}
/*
class ScrambleIterator(
    val scramble: ScrambleLE,
    var index: Int,
    var end: Int) {
*/
impl<'p, 's> ScrambleIterator<'p, 's> {
  /// Create a new iterator over the entire scramble
  pub fn new(scramble: &'s ScrambleLE<'p>) -> Self {
    let end = scramble.elements.len();
    ScrambleIterator {
      scramble,
      index: 0,
      end,
    }
  }
  /*
    def this(scramble: ScrambleLE) {
      this(scramble, 0, scramble.elements.length)
    }
    assert(end <= scramble.elements.length)
  */

  /// Create a new iterator with custom bounds
  pub fn with_bounds(scramble: &'s ScrambleLE<'p>, index: usize, end: usize) -> Self {
    assert!(end <= scramble.elements.len());
    ScrambleIterator {
      scramble,
      index,
      end,
    }
  }

  /// Check if at end of iteration
  pub fn at_end(&self) -> bool {
    self.index == self.end
  }
  /*
    def atEnd: Boolean = {
      index == end
    }
  */

  /// Get the range covered by remaining elements
  pub fn range(&self) -> RangeL {
    if self.index < self.end {
      RangeL(
        self.scramble.elements[self.index].range().begin(),
        self.scramble.elements[self.end - 1].range().end(),
      )
    } else {
      assert!(self.index == self.end);
      RangeL(self.scramble.range.end(), self.scramble.range.end())
    }
  }
  /*
    def range: RangeL = {
      if (index < end) {
        RangeL(
          scramble.elements(index).range.begin,
          scramble.elements(end - 1).range.end)
      } else {
        vassert(index == end)
        RangeL(scramble.range.end, scramble.range.end)
      }
    }
  */

  /// Get current position
  pub fn get_pos(&self) -> i32 {
    if self.index >= self.end {
      self.scramble.range.end()
    } else {
      self.scramble.elements[self.index].range().begin()
    }
  }
  /*
    def getPos(): Int = {
      if (index >= end) {
        scramble.range.end
      } else {
        scramble.elements(index).range.begin
      }
    }
  */

  /// Get the end position of the previous element
  pub fn get_prev_end_pos(&self) -> i32 {
    if self.index == 0 {
      self.scramble.range.begin()
    } else {
      self.scramble.elements[self.index - 1].range().end()
    }
  }
  /*
    def getPrevEndPos(): Int = {
      if (index == 0) {
        scramble.range.begin
      } else {
        scramble.elements(index - 1).range.end
      }
    }
  */

  /// Peek at the previous element
  pub fn peek_prev(&self) -> Option<&INodeLEEnum<'p>> {
    if self.index > 0 {
      Some(&self.scramble.elements[self.index - 1])
    } else {
      None
    }
  }

  /// Skip to the position of another iterator
  pub fn skip_to(&mut self, that: &ScrambleIterator<'p, 's>) {
    self.index = that.index;
  }
  /*
    def skipTo(that: ScrambleIterator): Unit = {
      index = that.index
    }
  */

  /// Stop iteration (move to end)
  pub fn stop(&mut self) {
    self.index = self.end;
  }
  /*
    def stop(): Unit = {
      index = end
    }
  */

  /*
    override def clone(): ScrambleIterator = new ScrambleIterator(scramble, index, end)
  */

  /// Check if there are more elements
  pub fn has_next(&self) -> bool {
    self.index < self.end
  }
  /*
    def hasNext: Boolean = index < end
  */

  /// Peek at the current element
  pub fn peek(&self) -> Option<&INodeLEEnum<'p>> {
    if self.index >= self.end {
      None
    } else {
      Some(&**&self.scramble.elements[self.index])
    }
  }

  /// Peek at the current element, returning owned clone to avoid borrow conflicts.
  pub fn peek_cloned(&self) -> Option<INodeLEEnum<'p>> {
    self.peek().cloned()
  }
  /*
    def peek(): Option[INodeLE] = {
      if (index >= end) None
      else Some(scramble.elements(index))
    }
  */

  /// Take the current element and advance (returning owned)
  pub fn take(&mut self) -> Option<INodeLEEnum<'p>> {
    if self.index >= self.end {
      None
    } else {
      let result = (*self.scramble.elements[self.index]).clone();
      self.index += 1;
      Some(result)
    }
  }
  /*
    def take(): Option[INodeLE] = {
      if (index >= end) None
      else Some(advance())
    }
  */

  /// Peek at the next n elements
  pub fn peek_n(&self, n: usize) -> Vec<Option<&INodeLEEnum<'p>>> {
    (0..n)
      .map(|i| {
        let idx = self.index + i;
        if idx < self.end {
          Some(&**&self.scramble.elements[idx])
        } else {
          None
        }
      })
      .collect()
  }
  /*
    // This is an Vector[Option[INodeLE]] instead of an Vector[INodeLE]
    // because we like to be able to ignore the tail end of something like
    // case Vector(Some(whatever), _)
    def peek(n: Int): Vector[Option[INodeLE]] = {
      U.mapRange[Option[INodeLE]](
        index,
        index + n,
        i => {
          if (i < end) Some(scramble.elements(i))
          else None
        })
    }
  */

  /// Peek at the next 2 elements
  pub fn peek2(&self) -> (Option<&INodeLEEnum<'p>>, Option<&INodeLEEnum<'p>>) {
    let first = if self.index < self.end {
      Some(&**&self.scramble.elements[self.index])
    } else {
      None
    };
    let second = if self.index + 1 < self.end {
      Some(&**&self.scramble.elements[self.index + 1])
    } else {
      None
    };
    (first, second)
  }

  /// Peek at the next 2 elements, returning owned clones to avoid borrow conflicts.
  pub fn peek2_cloned(&self) -> (Option<INodeLEEnum<'p>>, Option<INodeLEEnum<'p>>) {
    let (a, b) = self.peek2();
    (a.cloned(), b.cloned())
  }
  /*
    def peek2(): (Option[INodeLE], Option[INodeLE]) = {
      (
        (if (index + 0 < end) Some(scramble.elements(index + 0)) else None),
        (if (index + 1 < end) Some(scramble.elements(index + 1)) else None))
    }
  */

  /// Peek at the next 3 elements
  pub fn peek3(
    &self,
  ) -> (
    Option<&INodeLEEnum<'p>>,
    Option<&INodeLEEnum<'p>>,
    Option<&INodeLEEnum<'p>>,
  ) {
    let first = if self.index < self.end {
      Some(&**&self.scramble.elements[self.index])
    } else {
      None
    };
    let second = if self.index + 1 < self.end {
      Some(&**&self.scramble.elements[self.index + 1])
    } else {
      None
    };
    let third = if self.index + 2 < self.end {
      Some(&**&self.scramble.elements[self.index + 2])
    } else {
      None
    };
    (first, second, third)
  }

  /// Peek at the next 3 elements, returning owned clones to avoid borrow conflicts.
  pub fn peek3_cloned(
    &self,
  ) -> (
    Option<INodeLEEnum<'p>>,
    Option<INodeLEEnum<'p>>,
    Option<INodeLEEnum<'p>>,
  ) {
    let (a, b, c) = self.peek3();
    (a.cloned(), b.cloned(), c.cloned())
  }
  /*
    def peek3(): (Option[INodeLE], Option[INodeLE], Option[INodeLE]) = {
      (
        (if (index + 0 < end) Some(scramble.elements(index + 0)) else None),
        (if (index + 1 < end) Some(scramble.elements(index + 1)) else None),
        (if (index + 2 < end) Some(scramble.elements(index + 2)) else None))
    }
  */

  /// Check if next element is a specific word
  pub fn peek_word(&self, word: StrI<'_>) -> bool {
    match self.peek() {
      Some(INodeLEEnum::Word(WordLE { str, .. })) => *str == word,
      _ => false,
    }
  }
  /*
    def peekWord(word: StrI): Boolean = {
      peek() match {
        case Some(WordLE(_, s)) => s == word
        case _ => false
      }
    }
  */

  /// Advance and return a reference to the current element
  pub fn advance(&mut self) -> &INodeLEEnum<'p> {
    assert!(self.has_next());
    let result = &**&self.scramble.elements[self.index];
    self.index += 1;
    result
  }
  /*
    def advance(): INodeLE = {
      vassert(hasNext)
      val result = scramble.elements(index)
      index = index + 1
      result
    }
  */

  /*
    def trySkip[R](f: PartialFunction[INodeLE, R]): Option[INodeLE] = {
      peek().filter(f.isDefinedAt)
    }
  */
  /*
    def trySkipAll[R](f: Array[PartialFunction[INodeLE, Unit]]): Boolean = {
      vassert(index + f.length < scramble.elements.length)
      U.loop(f.length, i => {
        if (!f(i).isDefinedAt(scramble.elements(index + i))) {
          return false
        }
      })
      true
    }
  */

  /// Try to skip a symbol
  pub fn try_skip_symbol(&mut self, symbol: char) -> bool {
    match self.peek() {
      Some(INodeLEEnum::Symbol(SymbolLE(_, c))) if *c == symbol => {
        self.index += 1;
        true
      }
      _ => false,
    }
  }
  /*
    def trySkipSymbol(symbol: Char): Boolean = {
      peek() match {
        case Some(SymbolLE(_, s)) if s == symbol => {
          advance()
          true
        }
        case _ => false
      }
    }
  */

  /// Try to skip multiple symbols in sequence
  pub fn try_skip_symbols(&mut self, symbols: &[char]) -> bool {
    if self.index + symbols.len() > self.end {
      return false;
    }

    for (i, &expected) in symbols.iter().enumerate() {
      match &**&self.scramble.elements[self.index + i] {
        INodeLEEnum::Symbol(SymbolLE(_, c)) if *c == expected => {}
        _ => return false,
      }
    }

    self.index += symbols.len();
    true
  }
  /*
    def trySkipSymbols(symbols: Vector[Char]): Boolean = {
      if (index + symbols.length >= end) {
        return false
      }
      var i = 0
      while (i < symbols.length) {
        scramble.elements(index + i) match {
          case SymbolLE(_, s) if s == symbols(i) =>
          case _ => return false
        }
        i = i + 1
      }
      index = index + symbols.length
      true
    }
  */

  /// Get the next word element
  pub fn next_word(&mut self) -> Option<WordLE<'p>> {
    match self.peek() {
      Some(INodeLEEnum::Word(w)) => {
        let result = w.clone();
        self.index += 1;
        Some(result)
      }
      _ => None,
    }
  }
  /*
    def nextWord(): Option[WordLE] = {
      peek() match {
        case Some(w @ WordLE(_, _)) => {
          advance()
          Some(w)
        }
        case _ => None
      }
    }
  */

  /// Expect a specific word (panics if not found)
  pub fn expect_word(&mut self, str: StrI<'_>) {
    let found = self.try_skip_word(str).is_some();
    assert!(found, "Expected word {:?}", str);
  }
  /*
    def expectWord(str: StrI): Unit = {
      val found = trySkipWord(str).nonEmpty
      vassert(found)
    }
  */

  /// Try to skip a specific word
  pub fn try_skip_word(&mut self, str: StrI<'_>) -> Option<RangeL> {
    match self.peek() {
      Some(INodeLEEnum::Word(WordLE { range, str: s })) if *s == str => {
        let result = *range;
        self.index += 1;
        Some(result)
      }
      _ => None,
    }
  }
  /*
    def trySkipWord(str: StrI): Option[RangeL] = {
      peek() match {
        case Some(WordLE(range, s)) if s == str => {
          advance()
          Some(range)
        }
        case _ => None
      }
    }
  */

  /*
  //  def exists(func: scala.Function1[INodeLE, Boolean]): Boolean = {
  //    U.exists(scramble.elements, func, index, end)
  //  }
  */

  /// Find the index where a condition is true
  pub fn find_index_where<F>(&self, func: F) -> Option<usize>
  where
    F: Fn(&INodeLEEnum) -> bool,
  {
    for i in self.index..self.end {
      if func(&**&self.scramble.elements[i]) {
        return Some(i);
      }
    }
    None
  }
  /*
    def findIndexWhere(func: scala.Function1[INodeLE, Boolean]): Option[Int] = {
      U.findIndexWhereFromUntil(scramble.elements, func, index, end)
    }
  */

  /// Split the scramble on a specific symbol
  pub fn split_on_symbol(
    &self,
    needle: char,
    include_empty_trailing: bool,
  ) -> Vec<ScrambleIterator<'p, 's>> {
    let mut iters = Vec::new();
    let mut start = self.index;
    let mut i = start;

    while i < self.end {
      match &**&self.scramble.elements[i] {
        INodeLEEnum::Symbol(SymbolLE(_, c)) if *c == needle => {
          iters.push(ScrambleIterator::with_bounds(self.scramble, start, i));
          start = i + 1;
          i += 1;
        }
        _ => {
          i += 1;
        }
      }
    }

    if start < self.end {
      iters.push(ScrambleIterator::with_bounds(self.scramble, start, self.end));
    } else if start == self.end && include_empty_trailing {
      iters.push(ScrambleIterator::with_bounds(self.scramble, start, self.end));
    }

    iters
  }
  /*
    // We use this splitOnSymbol method for things like comma-separated
    // lists and things.
    // TODO: Soon, it will fall apart on certain cases. For example,
    // in a struct, we can have:
    //   struct Moo {
    //     x int;
    //     func bork() { }
    //     func zork() { }
    //   }
    // so it doesn't make much sense to split on semicolon.
    // Instead, we should make the iterator go until it finds a certain symbol.
    //
    // includeEmptyTrailingSection means that if we end with a needle,
    // we'll still return an empty iterator for the end.
    def splitOnSymbol(needle: Char, includeEmptyTrailing: Boolean): Vector[ScrambleIterator] = {
      val iters = new Accumulator[ScrambleIterator]()
      var start = index
      var i = start
      while (i < end) {
        scramble.elements(i) match {
          case SymbolLE(_, c) if c == needle => {
            iters.add(new ScrambleIterator(scramble, start, i))
            start = i + 1
            i = i + 1 // Note the 2 here
          }
          case _ => {
            i = i + 1
          }
        }
      }
      if (start < end) {
        // If we get in here, the scramble didnt end in this needle.
        // So, just add this as the last result.
        iters.add(new ScrambleIterator(scramble, start, end))
      } else if (start == end) {
        // If start == end, then we ended in a needle.
        if (includeEmptyTrailing) {
          iters.add(new ScrambleIterator(scramble, start, end))
        }
      }

      iters.buildArray()
    }
  */

  /// Get remaining elements count
  pub fn remaining(&self) -> usize {
    if self.end > self.index {
      self.end - self.index
    } else {
      0
    }
  }

  /// Check if there are at least n elements remaining
  pub fn has_at_least(&self, n: usize) -> bool {
    self.index + n <= self.end
  }

  /// Consume and return all remaining elements
  pub fn consume_rest(&mut self) -> Vec<INodeLEEnum<'p>> {
    let mut result = Vec::new();
    while self.has_next() {
      result.push(self.take().unwrap());
    }
    result
  }
}
/*
}
*/

pub struct ExpressionParser<'p, 'ctx> {
  parse_arena: &'ctx ParseArena<'p>,
  pub keywords: &'ctx Keywords<'p>,
}
/*
class ExpressionParser(interner: Interner, keywords: Keywords, opts: GlobalOptions, patternParser: PatternParser, templexParser: TemplexParser) {
*/

impl<'p, 'ctx> ExpressionParser<'p, 'ctx>
where
    'p: 'ctx,
{
  /// Parse a while loop
  /// Mirrors parseWhile in ExpressionParser.scala lines 242-279
  fn parse_while(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    let while_begin = iter.get_pos();

    let mut tentative_iter = iter.clone();

    let pure = tentative_iter.try_skip_word(self.keywords.pure);

    if tentative_iter
      .try_skip_word(self.keywords.whiile)
      .is_none()
    {
      return Ok(None);
    }

    iter.skip_to(&tentative_iter);

    // Parse condition (lines 255-259)
    let condition = self.parse_block_contents(iter, true, templex_parser, pattern_parser)?;

    // Parse body (lines 261-271)
    let body = match iter.peek_cloned() {
      Some(INodeLEEnum::Curlied(CurliedLE { range: _, contents })) => {
        let contents = contents.clone();
        iter.advance();
        let mut body_iter = ScrambleIterator::new(&contents);
        self.parse_block_contents(&mut body_iter, false, templex_parser, pattern_parser)?
      }
      _ => return Err(ParseError::BadExpressionBegin(iter.get_pos())),
    };

    Ok(Some(IExpressionPE::While(WhilePE {
      range: RangeL(while_begin, iter.get_prev_end_pos()),
      condition: self.parse_arena.alloc(condition),
      body: self.parse_arena.alloc(BlockPE {
        range: body.range(),
        maybe_pure: pure,
        maybe_default_region: None,
        inner: self.parse_arena.alloc(body),
      }),
    })))
  }
  /*
    private def parseWhile(iter: ScrambleIterator): Result[Option[WhilePE], IParseError] = {
      val whileBegin = iter.getPos()

      val tentativeIter = iter.clone()

      val pure = tentativeIter.trySkipWord(keywords.pure)

      if (tentativeIter.trySkipWord(keywords.whiile).isEmpty) {
        return Ok(None)
      }

      iter.skipTo(tentativeIter)

      val condition =
        parseBlockContents(iter, true) match {
          case Ok(result) => result
          case Err(cpe) => return Err(cpe)
        }

      val body =
        iter.peek() match {
          case Some(CurliedLE(range, contents)) => {
            iter.advance()
            parseBlockContents(new ScrambleIterator(contents), false) match {
              case Ok(result) => result
              case Err(cpe) => return Err(cpe)
            }
          }
          case _ => return Err(BadStartOfWhileBody(iter.getPos()))
        }

      Ok(
        Some(
          ast.WhilePE(
            RangeL(whileBegin, iter.getPrevEndPos()),
            condition,
            BlockPE(body.range, pure, None, body))))
    }
  */

  /// Parse an explicit block
  /// Mirrors parseExplicitBlock in ExpressionParser.scala lines 281-311
  fn parse_explicit_block(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    let block_begin = iter.get_pos();

    let mut tentative_iter = iter.clone();

    let pure = tentative_iter.try_skip_word(self.keywords.pure);

    if tentative_iter.try_skip_word(self.keywords.block).is_none() {
      return Ok(None);
    }

    iter.skip_to(&tentative_iter);

    // Parse body
    let contents = match iter.peek_cloned() {
      Some(INodeLEEnum::Curlied(CurliedLE { contents, .. })) => {
        let contents = contents.clone();
        iter.advance();
        let mut body_iter = ScrambleIterator::new(&contents);
        self.parse_block_contents(&mut body_iter, false, templex_parser, pattern_parser)?
      }
      _ => return Err(ParseError::BadExpressionBegin(iter.get_pos())),
    };

    Ok(Some(IExpressionPE::Block(BlockPE {
      range: RangeL(block_begin, iter.get_prev_end_pos()),
      maybe_pure: pure,
      maybe_default_region: None,
      inner: self.parse_arena.alloc(contents),
    })))
  }
  /*
    private def parseExplicitBlock(iter: ScrambleIterator): Result[Option[BlockPE], IParseError] = {
      val whileBegin = iter.getPos()

      val tentativeIter = iter.clone()

      val pure = tentativeIter.trySkipWord(keywords.pure)

      if (tentativeIter.trySkipWord(keywords.block).isEmpty) {
        return Ok(None)
      }

      iter.skipTo(tentativeIter)

      val body =
        iter.peek() match {
          case Some(CurliedLE(range, contents)) => {
            iter.advance()
            parseBlockContents(new ScrambleIterator(contents), false) match {
              case Ok(result) => result
              case Err(cpe) => return Err(cpe)
            }
          }
          case _ => return Err(BadStartOfBlock(iter.getPos()))
        }

      Ok(
        Some(
          ast.BlockPE(
            RangeL(whileBegin, iter.getPrevEndPos()),
            pure,
            None,
            body)))
    }
  */

  fn parse_foreach(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    let each_begin = original_iter.get_pos();

    let mut tentative_iter: ScrambleIterator<'p, '_> = original_iter.clone();

    if tentative_iter
      .try_skip_word(self.keywords.parallel)
      .is_some()
    {
      // do nothing for now
    }

    let pure = tentative_iter.try_skip_word(self.keywords.pure);

    if tentative_iter
      .try_skip_word(self.keywords.foreeach)
      .is_none()
    {
      return Ok(None);
    }
    original_iter.skip_to(&tentative_iter);
    let iter: &mut ScrambleIterator<'p, '_> = original_iter;

    let (in_range, pattern) = match try_skip_past_keyword_while(iter, self.keywords.r#in, |it| {
      match it.peek() {
        // Stop if we hit the end or a semicolon or a curly brace
        None => false,
        Some(INodeLEEnum::Symbol(SymbolLE(_, ';'))) => false,
        Some(INodeLEEnum::Curlied(_)) => false,
        // Continue for anything else
        Some(_) => true,
      }
    }) {
      None => return Err(ParseError::BadForeachInError(iter.get_pos())),
      Some((in_word, mut pattern_iter)) => {
        let pattern_begin = pattern_iter.get_pos();
        let pattern: PatternPP<'p> = pattern_parser.parse_pattern(
          &mut pattern_iter,
          templex_parser,
          pattern_begin,
          0,
          false,
          false,
          false,
          None,
        )?;
        (in_word.range, pattern)
      }
    };

    let iterable_expr = self.parse_block_contents(iter, true, templex_parser, pattern_parser)?;

    let _body_begin = iter.get_pos();

    let body = match iter.peek_cloned() {
      Some(INodeLEEnum::Curlied(CurliedLE { contents, .. })) => {
        let contents = contents.clone();
        iter.advance();
        self.parse_block_contents(
          &mut ScrambleIterator::new(&contents),
          false,
          templex_parser,
          pattern_parser,
        )?
      }
      _ => return Err(ParseError::BadStartOfWhileBody(iter.get_pos())),
    };

    Ok(Some(IExpressionPE::Each(EachPE {
      range: RangeL(each_begin, iter.get_prev_end_pos()),
      maybe_pure: pure,
      entry_pattern: pattern,
      in_keyword_range: in_range,
      iterable_expr: self.parse_arena.alloc(iterable_expr),
      body: self.parse_arena.alloc(BlockPE {
        range: body.range(),
        maybe_pure: None,
        maybe_default_region: None,
        inner: self.parse_arena.alloc(body),
      }),
    })))
  }

  /*
    private def parseForeach(
      originalIter: ScrambleIterator):
    Result[Option[EachPE], IParseError] = {
      val eachBegin = originalIter.getPos()

      val tentativeIter = originalIter.clone()

      if (tentativeIter.trySkipWord(keywords.parallel).nonEmpty) {
        // do nothing for now
      }

      val pure = tentativeIter.trySkipWord(keywords.pure)

      if (tentativeIter.trySkipWord(keywords.foreeach).isEmpty) {
        return Ok(None)
      }
      originalIter.skipTo(tentativeIter)
      val iter = originalIter

      val (inRange, pattern) =
        ParseUtils.trySkipPastKeywordWhile(
          iter,
          keywords.in,
          it => {
            it.peek() match {
              // Stop if we hit the end or a semicolon or a curly brace
              case None => false
              case Some(SymbolLE(_, ';')) => false
              case Some(CurliedLE(_, _)) => false
              // Continue for anything else
              case Some(_) => true
            }
          }) match {
          case None => return Err(BadForeachInError(iter.getPos()))
          case Some((in, patternIter)) => {
            patternParser.parsePattern(patternIter, patternIter.getPos(), 0, false, false, false, None) match {
              case Err(cpe) => return Err(cpe)
              case Ok(result) => (in.range, result)
            }
          }
        }

      val iterableExpr =
        parseBlockContents(iter, true) match {
          case Err(err) => return Err(err)
          case Ok(expression) => expression
        }

      val bodyBegin = iter.getPos()

      val body =
        iter.peek() match {
          case Some(CurliedLE(_, contents)) => {
            iter.advance()
            parseBlockContents(new ScrambleIterator(contents), false) match {
              case Err(cpe) => return Err(cpe)
              case Ok(result) => result
            }
          }
          case _ => return Err(BadStartOfWhileBody(iter.getPos()))
        }

      Ok(
        Some(
          EachPE(
            RangeL(eachBegin, iter.getPrevEndPos()),
            pure,
            pattern,
            inRange,
            iterableExpr,
            ast.BlockPE(RangeL(bodyBegin, iter.getPrevEndPos()), None, None, body))))
    }
  */

  /// Parse an if ladder (if/else if/else)
  /// Mirrors parseIfLadder in ExpressionParser.scala lines 388-478
  fn parse_if_ladder(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    let if_ladder_begin = iter.get_pos();

    // Check for 'if' keyword (lines 391-394)
    match iter.peek_cloned() {
      Some(INodeLEEnum::Word(WordLE { str, .. })) if str == self.keywords.iff => {}
      _ => return Ok(None),
    }

    // Parse root if (lines 396-400)
    let root_if = self.parse_if_part(iter, templex_parser, pattern_parser)?;

    // Parse else if parts (lines 402-415)
    let mut if_elses = Vec::new();
    while match iter.peek2_cloned() {
      (
        Some(INodeLEEnum::Word(WordLE { str: elsse, .. })),
        Some(INodeLEEnum::Word(WordLE { str: iff, .. })),
      ) if elsse == self.keywords.elsse && iff == self.keywords.iff => true,
      _ => false,
    } {
      iter.advance(); // Skip the else
      if_elses.push(self.parse_if_part(iter, templex_parser, pattern_parser)?);
    }

    // Parse else block (lines 417-436)
    let else_begin = iter.get_pos();
    let maybe_else_block = if iter.try_skip_word(self.keywords.elsse).is_some() {
      let body = match iter.peek_cloned() {
        Some(INodeLEEnum::Curlied(b)) => {
          let b = b.clone();
          iter.advance();
          b
        }
        _ => return Err(ParseError::BadExpressionBegin(iter.get_pos())),
      };

      let mut else_body_iter = ScrambleIterator::new(&body.contents);
      let else_body =
        self.parse_block_contents(&mut else_body_iter, false, templex_parser, pattern_parser)?;

      let else_end = iter.get_pos();
      Some(BlockPE {
        range: RangeL(else_begin, else_end),
        maybe_pure: None,
        maybe_default_region: None,
        inner: self.parse_arena.alloc(else_body),
      })
    } else {
      None
    };

    // Build final else block (lines 438-448)
    let final_else = match maybe_else_block {
      None => {
        let pos = iter.get_prev_end_pos();
        BlockPE {
          range: RangeL(pos, pos),
          maybe_pure: None,
          maybe_default_region: None,
          inner: self.parse_arena.alloc(IExpressionPE::Void(VoidPE {
            range: RangeL(pos, pos),
          })),
        }
      }
      Some(block) => block,
    };

    // Fold right to build nested if/else (lines 449-466)
    let mut root_else_block = final_else;
    for (cond_block, then_block) in if_elses.into_iter().rev() {
      root_else_block = BlockPE {
        range: RangeL(cond_block.range().begin(), then_block.range.end()),
        maybe_pure: None,
        maybe_default_region: None,
        inner: self.parse_arena.alloc(IExpressionPE::If(IfPE {
          range: RangeL(cond_block.range().begin(), then_block.range.end()),
          condition: self.parse_arena.alloc(cond_block),
          then_body: self.parse_arena.alloc(then_block),
          else_body: self.parse_arena.alloc(root_else_block),
        })),
      };
    }

    let (root_condition, root_then) = root_if;
    Ok(Some(IExpressionPE::If(IfPE {
      range: RangeL(if_ladder_begin, iter.get_prev_end_pos()),
      condition: self.parse_arena.alloc(root_condition),
      then_body: self.parse_arena.alloc(root_then),
      else_body: self.parse_arena.alloc(root_else_block),
    })))
  }

  /*
    private def parseIfLadder(iter: ScrambleIterator): Result[Option[IfPE], IParseError] = {
      val ifLadderBegin = iter.getPos()

      iter.peek() match {
        case Some(WordLE(_, str)) if str == keywords.iff =>
        case _ => return Ok(None)
      }

      val rootIf =
        parseIfPart(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      val ifElses = mutable.MutableList[(IExpressionPE, BlockPE)]()
      while (iter.peek2() match {
        case (Some(WordLE(_, elsse)), Some(WordLE(_, iff)))
          if elsse == keywords.elsse && iff == keywords.iff => true
        case _ => false
      }) {
        iter.advance() // Skip the else

        ifElses += (
          parseIfPart(iter) match {
            case Err(e) => return Err(e)
            case Ok(x) => x
          })
      }

      val elseBegin = iter.getPos()
      val maybeElseBlock =
        if (iter.trySkipWord(keywords.elsse).nonEmpty) {
          val body =
            iter.peek() match {
              case Some(b @ CurliedLE(_, _)) => iter.advance(); b
              case _ => return Err(BadStartOfElseBody(iter.getPos()))
            }

          val elseBody =
            parseBlockContents(new ScrambleIterator(body.contents), false) match {
              case Ok(result) => result
              case Err(cpe) => return Err(cpe)
            }

          val elseEnd = iter.getPos()
          Some(ast.BlockPE(RangeL(elseBegin, elseEnd), None, None, elseBody))
        } else {
          None
        }

      val finalElse: BlockPE =
        maybeElseBlock match {
          case None => {
            BlockPE(
              RangeL(iter.getPrevEndPos(), iter.getPrevEndPos()),
              None,
              None,
              VoidPE(RangeL(iter.getPrevEndPos(), iter.getPrevEndPos())))
          }
          case Some(block) => block
        }
      val rootElseBlock =
        ifElses.foldRight(finalElse)({
          case ((condBlock, thenBlock), elseBlock) => {
            // We don't check that both branches produce because of cases like:
            //   if blah {
            //     return 3;
            //   } else {
            //     6
            //   }
            BlockPE(
              RangeL(condBlock.range.begin, thenBlock.range.end),
              None,
              None,
              IfPE(
                RangeL(condBlock.range.begin, thenBlock.range.end),
                condBlock, thenBlock, elseBlock))
          }
        })
      val (rootConditionLambda, rootThenLambda) = rootIf
      // We don't check that both branches produce because of cases like:
      //   if blah {
      //     return 3;
      //   } else {
      //     6
      //   }
      Ok(
        Some(
          ast.IfPE(
            RangeL(ifLadderBegin, iter.getPrevEndPos()),
            rootConditionLambda,
            rootThenLambda,
            rootElseBlock)))
    }
  */

  fn next_is_set_expr(&self, iter: &ScrambleIterator) -> bool {
    match iter.peek2_cloned() {
      (
        Some(INodeLEEnum::Word(WordLE {
          range: set_range,
          str: set,
        })),
        Some(other),
      ) if set == self.keywords.set && set_range.end() < other.range().begin() => {
        // Then there's indeed a space after the set. Continue!
        true
      }
      _ => false,
    }
  }
  /*
    private def nextIsSetExpr(iter: ScrambleIterator): Boolean = {
      iter.peek2() match {
        case (Some(WordLE(setRange, set)), Some(other))
          if set == keywords.set && setRange.end < other.range.begin => {
          // Then there's indeed a space after the set. Continue!
          true
        }
        case _ => false
      }
    }
  */

  fn parse_mut_expr(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    let mutate_begin = iter.get_pos();
    if !self.next_is_set_expr(iter) {
      return Ok(None);
    }
    iter.advance();

    // Use try_skip_past_equals_while to find the mutatee expression
    let mutatee_expr =
        match try_skip_past_equals_while(iter, |scouting_iter| match scouting_iter.peek_cloned() {
          None => false,
          Some(INodeLEEnum::Symbol(SymbolLE(_, ';'))) => false,
          _ => true,
        }) {
          None => return Err(ParseError::BadMutateEqualsError(iter.get_pos())),
          Some(mut dest_iter) => self.parse_expression(
            &mut dest_iter,
            stop_on_curlied,
            templex_parser,
            pattern_parser,
          )?,
        };

    let source_expr =
        self.parse_expression(iter, stop_on_curlied, templex_parser, pattern_parser)?;

    Ok(Some(IExpressionPE::Mutate(MutatePE {
      range: RangeL(mutate_begin, iter.get_prev_end_pos()),
      mutatee: self.parse_arena.alloc(mutatee_expr),
      source: self.parse_arena.alloc(source_expr),
    })))
  }
  /*
    private def parseMutExpr(
      iter: ScrambleIterator,
      stopOnCurlied: Boolean):
    Result[Option[MutatePE], IParseError] = {

      val mutateBegin = iter.getPos()
      if (!nextIsSetExpr(iter)) {
        return Ok(None)
      }
      iter.advance()

      val mutateeExpr =
        ParseUtils.trySkipPastEqualsWhile(iter, scoutingIter => {
          scoutingIter.peek() match {
            case None => false
            case Some(SymbolLE(_, ';')) => false
            case _ => true
          }
        }) match {
          case None => return Err(BadMutateEqualsError(iter.getPos()))
          case Some(destIter) => {
            parseExpression(destIter, stopOnCurlied) match {
              case Err(err) => return Err(err)
              case Ok(expression) => expression
            }
          }
        }

      val sourceExpr =
        parseExpression(iter, stopOnCurlied) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      Ok(Some(MutatePE(RangeL(mutateBegin, iter.getPrevEndPos()), mutateeExpr, sourceExpr)))
    }
  */

  fn parse_let(
    &self,
    pattern_iter: &mut ScrambleIterator<'p, '_>,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<LetPE<'p>> {
    let pattern_begin = pattern_iter.get_pos();
    let pattern = pattern_parser.parse_pattern(
      pattern_iter,
      templex_parser,
      pattern_begin,
      0,
      false,
      false,
      false,
      None,
    )?;

    // Validate the pattern doesn't use 'set' keyword
    if let Some(DestinationLocalP {
                  decl: INameDeclarationP::LocalNameDeclaration(NameP(_, name)),
                  mutate: None,
                }) = &pattern.destination
    {
      assert!(*name != self.keywords.set);
    }

    let source_expr =
        self.parse_expression(iter, stop_on_curlied, templex_parser, pattern_parser)?;

    Ok(LetPE {
      range: RangeL(pattern.range.begin(), source_expr.range().end()),
      pattern: &*self.parse_arena.alloc(pattern),
      source: self.parse_arena.alloc(source_expr),
    })
  }
  /*
    private def parseLet(
      patternIter: ScrambleIterator,
      iter: ScrambleIterator,
      stopOnCurlied: Boolean):
    Result[LetPE, IParseError] = {
      val pattern =
        patternParser.parsePattern(patternIter, patternIter.getPos(), 0, false, false, false, None) match {
          case Ok(result) => result
          case Err(e) => return Err(e)
        }

      pattern.destination match {
        case Some(DestinationLocalP(LocalNameDeclarationP(name), None)) => vassert(name.str != keywords.set)
        case _ =>
      }

      val sourceExpr =
        parseExpression(iter, stopOnCurlied) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }
      Ok(LetPE(RangeL(pattern.range.begin, sourceExpr.range.end), pattern, sourceExpr))
    }
  */

  /// Parse a single if part (condition and then block)
  /// Mirrors parseIfPart in ExpressionParser.scala lines 313-386
  fn parse_if_part(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<(&'p IExpressionPE<'p>, BlockPE<'p>)> {
    let if_begin = iter.get_pos();

    if iter.try_skip_word(self.keywords.iff).is_none() {
      return Err(ParseError::BadExpressionBegin(iter.get_pos()));
    }

    // Parse condition (lines 318-321)
    let condition = self.parse_block_contents(iter, true, templex_parser, pattern_parser)?;

    // Parse then block (lines 323-369)
    let body = match iter.peek_cloned() {
      Some(INodeLEEnum::Curlied(CurliedLE { range: _, contents })) => {
        let contents = contents.clone();
        iter.advance();
        let mut body_iter = ScrambleIterator::new(&contents);
        self.parse_block_contents(&mut body_iter, false, templex_parser, pattern_parser)?
      }
      _ => return Err(ParseError::BadExpressionBegin(iter.get_pos())),
    };

    Ok((
      condition,
      BlockPE {
        range: RangeL(if_begin, iter.get_prev_end_pos()),
        maybe_pure: None,
        maybe_default_region: None,
        inner: body,
      },
    ))
  }

  /*
    private def parseIfPart(
      iter: ScrambleIterator):
    Result[(IExpressionPE, BlockPE), IParseError] = {
      if (iter.trySkipWord(keywords.iff).isEmpty) {
        vwat()
      }

      val conditionPE =
        parseBlockContents(iter, true) match {
          case Err(err) => return Err(err)
          case Ok(expression) => expression
        }

      val body =
        iter.peek() match {
          case Some(CurliedLE(_, contents)) => {
            iter.advance()
            parseBlockContents(new ScrambleIterator(contents), false) match {
              case Ok(result) => result
              case Err(cpe) => return Err(cpe)
            }
          }
          case None => return Err(BadStartOfIfBody(iter.getPos()))
        }

      Ok(
        (
          conditionPE,
          ast.BlockPE(body.range, None, None, body)))
    }
  */

  pub fn new(
    parse_arena: &'ctx ParseArena<'p>,
    keywords: &'ctx Keywords<'p>,
  ) -> Self {
    ExpressionParser { parse_arena, keywords }
  }

  /// Parse a block from a curlied expression
  /// Mirrors parseBlock in ExpressionParser.scala lines 586-589
  pub fn parse_block(
    &self,
    block_l: &CurliedLE<'p>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<&'p IExpressionPE<'p>> {
    let mut iter = ScrambleIterator::new(&block_l.contents);
    self.parse_block_contents(&mut iter, false, templex_parser, pattern_parser)
  }
  /*
    def parseBlock(blockL: CurliedLE): Result[IExpressionPE, IParseError] = {
      parseBlockContents(new ScrambleIterator(blockL.contents), false)
    }
  */

  /// Parse block contents
  /// Mirrors parseBlockContents in ExpressionParser.scala lines 590-640
  pub fn parse_block_contents(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<&'p IExpressionPE<'p>> {
    let mut statements: Vec<&'p IExpressionPE<'p>> = Vec::new();

    // Parse statements (lines 603-615)
    while match iter.peek_cloned() {
      None => false,
      Some(INodeLEEnum::Curlied(_)) if stop_on_curlied => false,
      Some(_) => {
        let statement =
          self.parse_statement(iter, stop_on_curlied, templex_parser, pattern_parser)?;
        statements.push(statement);
        true
      }
    } {}

    // If we just ate a semicolon, but there's nothing after it, then add a void (lines 617-633)
    if iter.has_next() {
      match iter.peek_cloned() {
        Some(INodeLEEnum::Symbol(SymbolLE(_, ')'))) => {
          // vcurious() - unexpected but continue
        }
        Some(INodeLEEnum::Symbol(SymbolLE(_, ']'))) => {
          // vcurious() - unexpected but continue
        }
        _ => {}
      }
    } else {
      if let Some(prev) = iter.peek_prev() {
        if let INodeLEEnum::Symbol(SymbolLE(range, ';')) = prev {
          statements.push(self.parse_arena.alloc(IExpressionPE::Void(VoidPE {
            range: RangeL(range.end(), range.end()),
          })));
        }
      }
    }

    // Return result (lines 635-639)
    match statements.len() {
      0 => Ok(self.parse_arena.alloc(IExpressionPE::Void(VoidPE {
        range: RangeL(iter.get_pos(), iter.get_pos()),
      }))),
      1 => Ok(statements.into_iter().next().unwrap()),
      _ => Ok(self.parse_arena.alloc(IExpressionPE::Consecutor(ConsecutorPE {
        inners: self.parse_arena.alloc_slice_from_vec(statements),
      })))
    }
  }
  /*
    def parseBlockContents(iter: ScrambleIterator, stopOnCurlied: Boolean): Result[IExpressionPE, IParseError] = {
      val statementsP = new Accumulator[IExpressionPE]()

      //    val endedInSemicolon =
      //      if (iter.scramble.elements.nonEmpty) {
      //        iter.scramble.elements(iter.end - 1) match {
      //          case SymbolLE(range, ';') => true
      //          case _ => false
      //        }
      //      } else {
      //        false
      //      }

      while (iter.peek() match {
        case None => false
        case Some(CurliedLE(range, contents)) if stopOnCurlied => false
        case Some(_) => {
          val statementP =
            parseStatement(iter, stopOnCurlied) match {
              case Err(error) => return Err(error)
              case Ok(s) => s
            }
          statementsP.add(statementP)
          true
        }
      }) {}

      // If we just ate a semicolon, but there's nothing after it, then add a void.
      if (iter.hasNext) {
        iter.peek() match {
          case Some(SymbolLE(_, ')')) => vcurious()
          case Some(SymbolLE(_, ']')) => vcurious()
          case _ =>
        }
      } else {
        if (iter.scramble.elements.nonEmpty) {
          iter.scramble.elements(iter.index - 1) match {
            case SymbolLE(range, ';') => {
              statementsP.add(VoidPE(RangeL(range.end, range.end)))
            }
            case _ =>
          }
        }
      }

      statementsP.size match {
        case 0 => Ok(VoidPE(RangeL(iter.getPos(), iter.getPos())))
        case 1 => Ok(statementsP.head)
        case _ => Ok(ConsecutorPE(statementsP.buildArray().toVector))
      }
    }
  */

  /// Parse lone block
  /// Parse lone block expression
  /// Mirrors parseLoneBlock in ExpressionParser.scala lines 642-676
  fn parse_lone_block(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    // Mirrors ExpressionParser.scala line 645
    let mut tentative_iter = iter.clone();

    // Mirrors ExpressionParser.scala lines 647-650
    // The pure/unsafe is a hack to get syntax highlighting work for
    // the future pure block feature.
    tentative_iter.try_skip_word(self.keywords.r#unsafe);
    let pure = tentative_iter.try_skip_word(self.keywords.pure);

    // Mirrors ExpressionParser.scala lines 652-654
    if tentative_iter.try_skip_word(self.keywords.block).is_none() {
      return Ok(None);
    }

    // Mirrors ExpressionParser.scala lines 656-657
    iter.skip_to(&tentative_iter);

    // Mirrors ExpressionParser.scala line 659
    let begin = iter.get_pos();

    // Mirrors ExpressionParser.scala lines 661-673
    let inner = match iter.peek_cloned() {
      Some(INodeLEEnum::Curlied(curlied)) => {
        let curlied_contents = curlied.contents.clone();
        iter.advance();
        let mut contents_iter = ScrambleIterator::new(&curlied_contents);
        match self.parse_block_contents(&mut contents_iter, false, templex_parser, pattern_parser) {
          Err(error) => return Err(error),
          Ok(result) => result,
        }
      }
      _ => {
        return Err(ParseError::BadStartOfBlock(iter.get_pos()));
      }
    };

    // Mirrors ExpressionParser.scala line 675
    Ok(Some(IExpressionPE::Block(BlockPE {
      range: RangeL(begin, iter.get_prev_end_pos()),
      maybe_pure: pure,
      maybe_default_region: None,
      inner: self.parse_arena.alloc(inner),
    })))
  }
  /*
    private def parseLoneBlock(
      originalIter: ScrambleIterator):
    Result[Option[IExpressionPE], IParseError] = {
      val tentativeIter = originalIter.clone()

      // The pure/unsafe is a hack to get syntax highlighting work for
      // the future pure block feature.
      tentativeIter.trySkipWord(keywords.unsafe)
      val pure = tentativeIter.trySkipWord(keywords.pure)

      if (tentativeIter.trySkipWord(keywords.block).isEmpty) {
        return Ok(None)
      }

      originalIter.skipTo(tentativeIter)
      val iter = originalIter

      val begin = iter.getPos()

      val contents =
        iter.peek() match {
          case Some(CurliedLE(_, contents)) => {
            iter.advance()
            parseBlockContents(new ScrambleIterator(contents), false) match {
              case Err(error) => return Err(error)
              case Ok(result) => result
            }
          }
          case None => {
            return Err(BadStartOfBlock(iter.getPos()))
          }
        }

      Ok(Some(ast.BlockPE(RangeL(begin, iter.getPrevEndPos()), pure, None, contents)))
    }
  */


  fn parse_destruct(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    // Mirrors ExpressionParser.scala line 682
    let begin = iter.get_pos();

    // Mirrors ExpressionParser.scala lines 683-685
    if iter.try_skip_word(self.keywords.destruct).is_none() {
      return Ok(None);
    }

    // Mirrors ExpressionParser.scala lines 687-691
    let inner_expr =
        match self.parse_expression(iter, stop_on_curlied, templex_parser, pattern_parser) {
          Err(e) => return Err(e),
          Ok(x) => x,
        };

    // Mirrors ExpressionParser.scala line 693
    Ok(Some(IExpressionPE::Destruct(DestructPE {
      range: RangeL(begin, iter.get_prev_end_pos()),
      inner: self.parse_arena.alloc(inner_expr),
    })))
  }
  /*
    private def parseDestruct(
      iter: ScrambleIterator,
      stopOnCurlied: Boolean):
    Result[Option[IExpressionPE], IParseError] = {
      val begin = iter.getPos()
      if (iter.trySkipWord(keywords.destruct).isEmpty) {
        return Ok(None)
      }

      val innerExpr =
        parseExpression(iter, stopOnCurlied) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      Ok(Some(DestructPE(RangeL(begin, iter.getPrevEndPos()), innerExpr)))
    }
  */

  /// Parse unlet
  /// Mirrors parseUnlet in ExpressionParser.scala
  fn parse_unlet(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<Option<IExpressionPE<'p>>> {
    // Check for 'unlet' keyword
    if let Some(range) = iter.try_skip_word(self.keywords.unlet) {
      // Parse the name to unlet
      match iter.peek_cloned() {
        Some(INodeLEEnum::Word(WordLE {
                                 range: name_range,
                                 str: name_str,
                               })) => {
          let name = IImpreciseNameP::LookupName(NameP(name_range, name_str));
          iter.advance();
          Ok(Some(IExpressionPE::Unlet(UnletPE {
            range: RangeL::new(range.begin(), iter.get_prev_end_pos()),
            name,
          })))
        }
        _ => Ok(None),
      }
    } else {
      Ok(None)
    }
  }
  /*
    private def parseUnlet(
      iter: ScrambleIterator):
    Result[Option[IExpressionPE], IParseError] = {
      val begin = iter.getPos()
      if (iter.trySkipWord(keywords.unlet).isEmpty) {
        return Ok(None)
      }
      val local =
        iter.nextWord() match {
          case None => return Err(BadLocalNameInUnlet(iter.getPos()))
          case Some(WordLE(range, str)) => LookupNameP(NameP(range, str))
        }
      Ok(Some(UnletPE(RangeL(begin, iter.getPrevEndPos()), local)))
    }
  */

  fn parse_return(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    let begin = iter.get_pos();
    if iter.try_skip_word(self.keywords.retuurn).is_none() {
      return Ok(None);
    }

    let inner_expr =
        self.parse_expression(iter, stop_on_curlied, templex_parser, pattern_parser)?;

    if !iter.try_skip_symbol(';') {
      return Err(ParseError::BadExpressionEnd(iter.get_pos()));
    }

    Ok(Some(IExpressionPE::Return(ReturnPE {
      range: RangeL(begin, iter.get_prev_end_pos()),
      expr: self.parse_arena.alloc(inner_expr),
    })))
  }
  /*
    private def parseReturn(
      iter: ScrambleIterator,
        stopOnCurlied: Boolean):
    Result[Option[IExpressionPE], IParseError] = {
      val begin = iter.getPos()
      if (iter.trySkipWord(keywords.retuurn).isEmpty) {
        return Ok(None)
      }

      val innerExpr =
        parseExpression(iter, stopOnCurlied) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      if (!iter.trySkipSymbol(';')) {
        return Err(BadExpressionEnd(iter.getPos()))
      }

      Ok(Some(ReturnPE(RangeL(begin, iter.getPrevEndPos()), innerExpr)))
    }
  */

  fn parse_break(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<Option<IExpressionPE<'p>>> {
    let begin = iter.get_pos();
    if iter.try_skip_word(self.keywords.r#break).is_none() {
      return Ok(None);
    }
    if !iter.try_skip_symbol(';') {
      return Err(ParseError::BadExpressionEnd(iter.get_pos()));
    }
    Ok(Some(IExpressionPE::Break(BreakPE {
      range: RangeL(begin, iter.get_prev_end_pos()),
    })))
  }
  /*
    private def parseBreak(
      iter: ScrambleIterator):
    Result[Option[IExpressionPE], IParseError] = {
      val begin = iter.getPos()
      if (iter.trySkipWord(keywords.break).isEmpty) {
        return Ok(None)
      }
      if (!iter.trySkipSymbol(';')) {
        return Err(BadExpressionEnd(iter.getPos()))
      }
      Ok(Some(BreakPE(RangeL(begin, iter.getPrevEndPos()))))
    }
  */

  /// Parse a statement
  /// Mirrors parseStatement in ExpressionParser.scala lines 746-829
  pub fn parse_statement(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<&'p IExpressionPE<'p>> {
    if !iter.has_next() {
      return Err(ParseError::BadExpressionBegin(iter.get_pos()));
    }

    // Try various statement types (lines 754-785)
    if let Some(x) = self.parse_while(iter, templex_parser, pattern_parser)? {
      return Ok(self.parse_arena.alloc(x));
    }
    if let Some(x) = self.parse_explicit_block(iter, templex_parser, pattern_parser)? {
      return Ok(self.parse_arena.alloc(x));
    }
    if let Some(x) = self.parse_if_ladder(iter, templex_parser, pattern_parser)? {
      return Ok(self.parse_arena.alloc(x));
    }
    if let Some(x) = self.parse_foreach(iter, templex_parser, pattern_parser)? {
      return Ok(self.parse_arena.alloc(x));
    }
    if let Some(x) = self.parse_break(iter)? {
      return Ok(self.parse_arena.alloc(x));
    }
    if let Some(x) = self.parse_return(iter, stop_on_curlied, templex_parser, pattern_parser)? {
      return Ok(self.parse_arena.alloc(x));
    }

    assert!(iter.has_next());

    // Parse let or lone expression (lines 789-818)
    let let_or_lone_expr: &'p IExpressionPE<'p> = if self.next_is_set_expr(iter) {
      self.parse_arena.alloc(self
          .parse_mut_expr(iter, stop_on_curlied, templex_parser, pattern_parser)?
          .expect("parse_mut_expr should return Some when next_is_set_expr is true"))
    } else {
      match try_skip_past_equals_while(iter, |scouting_iter| match scouting_iter.peek_cloned() {
        None => false,
        Some(INodeLEEnum::Curlied(_)) if stop_on_curlied => false,
        Some(INodeLEEnum::Symbol(SymbolLE(_, ';'))) => false,
        _ => true,
      }) {
        Some(mut dest_iter) => {
          match self.parse_let(&mut dest_iter, iter, stop_on_curlied, templex_parser, pattern_parser) {
            Ok(let_expr) => self.parse_arena.alloc(IExpressionPE::Let(let_expr)),
            Err(ParseError::BadThingAfterTypeInPattern(_)) => {
              return Err(ParseError::ForgotSetKeyword(dest_iter.get_pos()))
            }
            Err(e) => return Err(e),
          }
        }
        None => {
          self.parse_expression(iter, stop_on_curlied, templex_parser, pattern_parser)?
        }
      }
    };

    // Consume optional semicolon (lines 819-827)
    match iter.peek_cloned() {
      None => {}                                             // okay, hit the end
      Some(INodeLEEnum::Curlied(_)) if stop_on_curlied => {} // okay, hit the end
      Some(INodeLEEnum::Symbol(SymbolLE(_, ';'))) => {
        iter.advance(); // consume it to end the statement
      }
      _ => return Err(ParseError::BadExpressionEnd(iter.get_pos())),
    }

    Ok(let_or_lone_expr)
  }
  /*
    private[parsing] def parseStatement(
      iter: ScrambleIterator,
      stopOnCurlied: Boolean):
    Result[IExpressionPE, IParseError] = {
      if (!iter.hasNext) {
        return Err(BadExpressionBegin(iter.getPos()))
      }

      parseWhile(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }
      parseExplicitBlock(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }
      parseIfLadder(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }
      parseForeach(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      parseBreak(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      parseReturn(iter, stopOnCurlied) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      vassert(iter.hasNext)

      val letOrLoneExpr =
        if (nextIsSetExpr(iter)) {
          parseMutExpr(iter, stopOnCurlied) match {
            case Err(e) => return Err(e)
            case Ok(None) => vwat()
            case Ok(Some(x)) => x
          }
        } else {
          ParseUtils.trySkipPastEqualsWhile(iter, scoutingIter => {
            scoutingIter.peek() match {
              case None => false
              case Some(CurliedLE(range, contents)) if stopOnCurlied => false
              case Some(SymbolLE(_, ';')) => false
              case _ => true
            }
          }) match {
            case Some(destIter) => {
              parseLet(destIter, iter, stopOnCurlied) match {
                case Err(BadThingAfterTypeInPattern(_)) => return Err(ForgotSetKeyword(destIter.getPos()))
                case Err(e) => return Err(e)
                case Ok(x) => x
              }
            }
            case None => {
              parseExpression(iter, stopOnCurlied) match {
                case Err(e) => return Err(e)
                case Ok(x) => x
              }
            }
          }
        }
      iter.peek() match {
        case None => // okay, hit the end, continue
        case Some(CurliedLE(range, contents)) if stopOnCurlied => // okay, hit the end, continue
        case Some(SymbolLE(range, ';')) => {
          iter.advance() // consume it to end the statement.
          // continue
        }
        case _ => return Err(BadExpressionEnd(iter.getPos()))
      }
      Ok(letOrLoneExpr)
    }
  */

  /// Get operator precedence
  /// Mirrors getPrecedence in ExpressionParser.scala lines 831-844
  /// Get operator precedence
  /// Mirrors getPrecedence in ExpressionParser.scala lines 831-843
  pub fn get_precedence(&self, str: StrI<'_>) -> i32 {
    if str == self.keywords.dot_dot {
      6
    } else if str == self.keywords.asterisk || str == self.keywords.slash {
      5
    } else if str == self.keywords.plus || str == self.keywords.minus {
      4
    } else if str == self.keywords.spaceship
      || str == self.keywords.less_equals
      || str == self.keywords.less
      || str == self.keywords.greater_equals
      || str == self.keywords.greater
      || str == self.keywords.triple_equals
      || str == self.keywords.double_equals
      || str == self.keywords.not_equals
    {
      2
    } else if str == self.keywords.and || str == self.keywords.or {
      1
    } else {
      3 // Default precedence for custom operators like "mod", "florgle", etc. (Scala line 842)
    }
  }
  /*
    def getPrecedence(str: StrI): Int = {
      if (str == keywords.DOT_DOT) 6
      else if (str == keywords.asterisk || str == keywords.slash) 5
      else if (str == keywords.plus || str == keywords.minus) 4
      // _ => 3 Everything else is 3, see end case
      else if (str == keywords.asterisk || str == keywords.slash) 5
      else if (str == keywords.spaceship || str == keywords.lessEquals ||
        str == keywords.less || str == keywords.greaterEquals ||
        str == keywords.greater || str == keywords.tripleEquals ||
        str == keywords.doubleEquals || str == keywords.notEquals) 2
      else if (str == keywords.and || str == keywords.or) 1
      else 3 // This is so we can have 3 mod 2 == 1
    }
  */

  /// Parse an expression
  /// Mirrors parseExpression in ExpressionParser.scala lines 845-897
  pub fn parse_expression(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<&'p IExpressionPE<'p>> {
    if !iter.has_next() {
      return Err(ParseError::BadExpressionBegin(iter.get_pos()));
    }

    let mut elements = Vec::new();

    // Parse expression elements (lines 853-890)
    loop {
      let sub_expr = self.parse_expression_data_element(
        iter,
        stop_on_curlied,
        templex_parser,
        pattern_parser,
      )?;
      let sub_expr_range_end = sub_expr.range().end();
      elements.push(ExpressionElement::Data(sub_expr));

      if self.at_expression_end(iter, stop_on_curlied) {
        break;
      } else {
        if sub_expr_range_end == iter.get_pos() {
          return Err(ParseError::NeedWhitespaceAroundBinaryOperator(
            iter.get_pos(),
          ));
        }

        match self.parse_binary_call(iter)? {
          None => break,
          Some(symbol) => {
            let precedence = self.get_precedence(symbol.str());
            elements.push(ExpressionElement::BinaryCall(symbol.clone(), precedence));

            match iter.peek_cloned() {
              None => return Err(ParseError::BadExpressionEnd(iter.get_pos())),
              Some(node) => {
                if symbol.range().end() == node.range().begin() {
                  return Err(ParseError::NeedWhitespaceAroundBinaryOperator(
                    iter.get_pos(),
                  ));
                }
              }
            }
          }
        }
      }
    }

    // Descramble the expression (lines 892-894)
    let (expr_pe, _) = self.descramble_elements(&elements, 0, elements.len() - 1, 1)?;
    Ok(expr_pe)
  }
  /*
    def parseExpression(iter: ScrambleIterator, stopOnCurlied: Boolean): Result[IExpressionPE, IParseError] = {
      Profiler.frame(() => {
        if (!iter.hasNext) {
          return Err(BadExpressionBegin(iter.getPos()))
        }

        val elements = mutable.ArrayBuffer[IExpressionElement]()

        while ({
          val subExpr =
            parseExpressionDataElement(iter, stopOnCurlied) match {
              case Err(error) => return Err(error)
              case Ok(x) => x
            }
          elements += parsing.DataElement(subExpr)

          if (atExpressionEnd(iter, stopOnCurlied)) {
            false
          } else {
            if (subExpr.range.end == iter.getPos()) {
              return Err(NeedWhitespaceAroundBinaryOperator(iter.getPos()))
            }

            parseBinaryCall(iter) match {
              case Err(error) => return Err(error)
              case Ok(None) => false
              case Ok(Some(symbol)) => {
                vassert(MIN_PRECEDENCE == 1)
                vassert(MAX_PRECEDENCE == 6)
                val precedence = getPrecedence(symbol.str)
                elements += parsing.BinaryCallElement(symbol, precedence)


                iter.peek() match {
                  case None => return new Err(BadExpressionEnd(iter.getPos()))
                  case Some(node) => {
                    if (symbol.range.end == node.range.begin) {
                      return Err(NeedWhitespaceAroundBinaryOperator(iter.getPos()))
                    }
                  }
                }
                true
              }
            }
          }
        }) {}

        val (exprPE, _) =
          descramble(elements.toVector, 0, elements.size - 1, MIN_PRECEDENCE)
        Ok(exprPE)
      })
    }
  */

  /// Parse a lookup expression
  /// Mirrors parseLookup in ExpressionParser.scala lines 898-939
  pub fn parse_lookup(&self, iter: &mut ScrambleIterator<'p, '_>) -> Option<IExpressionPE<'p>> {
    let begin = iter.get_pos();
    match iter.peek3_cloned() {
      (
        Some(INodeLEEnum::Symbol(SymbolLE(_, '<'))),
        Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
        Some(INodeLEEnum::Symbol(SymbolLE(_, '>'))),
      ) => {
        iter.advance();
        iter.advance();
        iter.advance();
        Some(IExpressionPE::Lookup(self.parse_arena.alloc(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(
            RangeL(begin, iter.get_prev_end_pos()),
            self.keywords.spaceship,
          )),
          template_args: None,
        })))
      }
      (
        Some(INodeLEEnum::Symbol(SymbolLE(
          range1,
          c1 @ ('=' | '>' | '<' | '!'),
        ))),
        Some(INodeLEEnum::Symbol(SymbolLE(range2, '='))),
        _,
      ) => {
        iter.advance();
        iter.advance();
        let combined = format!("{}{}", c1, '=');
        Some(IExpressionPE::Lookup(self.parse_arena.alloc(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(
            RangeL(range1.begin(), range2.end()),
            self.parse_arena.intern_str(&combined),
          )),
          template_args: None,
        })))
      }
      (Some(INodeLEEnum::Symbol(SymbolLE(range, c))), _, _) => {
        iter.advance();
        Some(IExpressionPE::Lookup(self.parse_arena.alloc(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(
            range,
            self.parse_arena.intern_str(&c.to_string()),
          )),
          template_args: None,
        })))
      }
      (Some(INodeLEEnum::Word(WordLE { range, str })), _, _) => {
        iter.advance();
        Some(IExpressionPE::Lookup(self.parse_arena.alloc(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(range, str)),
          template_args: None,
        })))
      }
      _ => None,
    }
  }

  /*
    def parseLookup(iter: ScrambleIterator): Option[IExpressionPE] = {
      val begin = iter.getPos()
      iter.peek3() match {
        case (Some(SymbolLE(_, '<')), Some(SymbolLE(_, '=')), Some(SymbolLE(_, '>'))) => {
          iter.advance()
          iter.advance()
          iter.advance()
          Some(
            LookupPE(
              LookupNameP(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.spaceship)),
              None))
        }
        case (Some(SymbolLE(range1, c1 @ ('=' | '>' | '<' | '!'))), Some(SymbolLE(range2, c2 @ '=')), _) => {
          iter.advance()
          iter.advance()
          Some(
            LookupPE(
              LookupNameP(NameP(RangeL(range1.begin, range2.end), interner.intern(StrI(c1.toString + c2)))),
              None))
        }
        case (Some(SymbolLE(range, c)), _, _) => {
          iter.advance()
          Some(
            LookupPE(
              LookupNameP(NameP(range, interner.intern(StrI(c.toString)))),
              None))
        }
        case (Some(WordLE(range, str)), _, _) => {
          iter.advance()
          Some(
            LookupPE(
              LookupNameP(NameP(range, str)),
              None))
        }
        case _ => None
      }
  //    Parser.parseFunctionOrLocalOrMemberName(iter) match {
  //      case Some(name) => Some(LookupPE(LookupNameP(name), None))
  //      case None => None
  //    }
    }
  */

  /// Parse a boolean literal
  /// Mirrors parseBoolean in ExpressionParser.scala lines 940-954
  pub fn parse_boolean(&self, iter: &mut ScrambleIterator<'p, '_>) -> Option<IExpressionPE<'p>> {
    if let Some(range) = iter.try_skip_word(self.keywords.truue) {
      return Some(IExpressionPE::ConstantBool(ConstantBoolPE {
        range,
        value: true,
      }));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.faalse) {
      return Some(IExpressionPE::ConstantBool(ConstantBoolPE {
        range,
        value: false,
      }));
    }
    None
  }
  /*
    def parseBoolean(iter: ScrambleIterator): Option[IExpressionPE] = {
      val start = iter.getPos()
      iter.trySkipWord(keywords.truue) match {
        case Some(range) => return Some(ConstantBoolPE(range, true))
        case _ =>
      }
      iter.trySkipWord(keywords.faalse) match {
        case Some(range) => return Some(ConstantBoolPE(range, false))
        case _ =>
      }
      return None
    }
  */

  /// Parse an atomic expression
  /// Mirrors parseAtom in ExpressionParser.scala lines 955-1092
  pub fn parse_atom(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<IExpressionPE<'p>> {
    assert!(iter.has_next());
    let begin = iter.get_pos();

    // Check for keywords that can't be used in expressions (lines 960-969)
    if iter.try_skip_word(self.keywords.r#break).is_some() {
      return Err(ParseError::CantUseBreakInExpression(iter.get_pos()));
    }
    if iter.try_skip_word(self.keywords.retuurn).is_some() {
      return Err(ParseError::CantUseReturnInExpression(iter.get_pos()));
    }
    if iter.try_skip_word(self.keywords.whiile).is_some() {
      return Err(ParseError::CantUseWhileInExpression(iter.get_pos()));
    }

    // Check for underscore (magic param lookup) (lines 970-973)
    if let Some(range) = iter.try_skip_word(self.keywords.underscore) {
      return Ok(IExpressionPE::MagicParamLookup(MagicParamLookupPE {
        range,
      }));
    }

    // Try foreach (lines 974-978)
    if let Some(x) = self.parse_foreach(iter, templex_parser, pattern_parser)? {
      return Ok(x);
    }

    // Try mut expression (lines 980-984)
    if let Some(x) = self.parse_mut_expr(iter, stop_on_curlied, templex_parser, pattern_parser)? {
      return Ok(x);
    }

    // Parse literals (lines 986-1014)
    match iter.peek_cloned() {
      Some(INodeLEEnum::ParsedInteger(ParsedIntegerLE { range, value, bits })) => {
        iter.advance();
        return Ok(IExpressionPE::ConstantInt(ConstantIntPE {
          range,
          value,
          bits,
        }));
      }
      Some(INodeLEEnum::ParsedDouble(ParsedDoubleLE { range, value, .. })) => {
        iter.advance();
        return Ok(IExpressionPE::ConstantFloat(ConstantFloatPE {
          range,
          value,
        }));
      }
      Some(INodeLEEnum::String(StringLE { range, parts })) => {
        iter.advance();

        // Check if it's a simple literal string
        if parts.len() == 1 {
          if let StringPart::Literal { s, .. } = &parts[0] {
            return Ok(IExpressionPE::ConstantStr(ConstantStrPE {
              range,
              value: self.parse_arena.intern_str(s),
            }));
          }
        }

        // String interpolation
        let mut parts_p: Vec<&'p IExpressionPE<'p>> = Vec::new();
        for part in parts {
          match part {
            StringPart::Literal { range, s } => {
              parts_p.push(self.parse_arena.alloc(IExpressionPE::ConstantStr(ConstantStrPE {
                range: *range,
                value: self.parse_arena.intern_str(s.as_str()),
              })));
            }
            StringPart::Expr(scramble) => {
              let scramble_clone = scramble.clone();
              let mut part_iter = ScrambleIterator::new(&scramble_clone);
              let expr =
                self.parse_expression(&mut part_iter, false, templex_parser, pattern_parser)?;
              parts_p.push(expr);
            }
          }
        }
        return Ok(IExpressionPE::StrInterpolate(StrInterpolatePE {
          range,
          parts: self.parse_arena.alloc_slice_from_vec(parts_p),
        }));
      }
      _ => {}
    }

    // Try boolean (lines 1015-1018)
    if let Some(e) = self.parse_boolean(iter) {
      return Ok(e);
    }

    // Try array (lines 1019-1023)
    if let Some(e) = self.parse_array(iter, templex_parser, pattern_parser)? {
      return Ok(e);
    }

    // Try lambda (lines 1024-1028)
    if let Some(e) = self.parse_lambda(iter, templex_parser, pattern_parser)? {
      return Ok(e);
    }

    // Try lookup (lines 1029-1032)
    if let Some(e) = self.parse_lookup(iter) {
      return Ok(e);
    }

    // Try tuple or sub-expression (lines 1033-1039)
    if let Some(e) = self.parse_tuple_or_sub_expression(iter, templex_parser, pattern_parser)? {
      return Ok(e);
    }

    // If nothing matched, error (continuing from line 1039+)
    Err(ParseError::BadExpressionBegin(begin))
  }
  /*
    // Note that this can consume an unbounded number of tokens, for example if
    // we encounter a set expression.
    def parseAtom(iter: ScrambleIterator, stopOnCurlied: Boolean): Result[IExpressionPE, IParseError] = {
      vassert(iter.hasNext)
      val begin = iter.getPos()

      // See BRCOBS
      if (iter.trySkipWord(keywords.break).nonEmpty) {
        return Err(CantUseBreakInExpression(iter.getPos()))
      }
      // See BRCOBS
      if (iter.trySkipWord(keywords.retuurn).nonEmpty) {
        return Err(CantUseReturnInExpression(iter.getPos()))
      }
      if (iter.trySkipWord(keywords.whiile).nonEmpty) {
        return Err(CantUseWhileInExpression(iter.getPos()))
      }
      iter.trySkipWord(keywords.UNDERSCORE) match {
        case Some(range) => return Ok(MagicParamLookupPE(range))
        case _ =>
      }
      parseForeach(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      parseMutExpr(iter, stopOnCurlied) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      iter.peek() match {
        case Some(ParsedIntegerLE(range, num, bits)) => {
          iter.advance()
          return Ok(ConstantIntPE(range, num, bits))
        }
        case Some(ParsedDoubleLE(range, num, bits)) => {
          iter.advance()
          return Ok(ConstantFloatPE(range, num))
        }
        case Some(StringLE(range, Vector(StringPartLiteral(_, s)))) => {
          iter.advance()
          return Ok(ConstantStrPE(range, s))
        }
        case Some(StringLE(range, partsL)) => {
          iter.advance()
          val partsP =
            U.map[StringPart, IExpressionPE](partsL, {
              case StringPartLiteral(range, s) => ConstantStrPE(range, s)
              case StringPartExpr(scramble) => {
                parseExpression(new ScrambleIterator(scramble), false) match {
                  case Err(e) => return Err(e)
                  case Ok(x) => x
                }
              }
            })
          return Ok(StrInterpolatePE(range, partsP.toVector))
        }
        case _ =>
      }
      parseBoolean(iter) match {
        case Some(e) => return Ok(e)
        case None =>
      }
      parseArray(iter) match {
        case Err(err) => return Err(err)
        case Ok(Some(e)) => return Ok(e)
        case Ok(None) =>
      }
      parseLambda(iter) match {
        case Err(err) => return Err(err)
        case Ok(Some(e)) => return Ok(e)
        case Ok(None) =>
      }
      parseLookup(iter) match {
        case Some(e) => return Ok(e)
        case None =>
      }
      parseTupleOrSubExpression(iter) match {
        case Err(err) => return Err(err)
        case Ok(Some(e)) => {
          return Ok(e)
        }
        case Ok(None) =>
      }
      return Err(BadExpressionBegin(iter.getPos()))
    }
  */
/*
//  def parseNumberExpr(originalIter: ScrambleIterator): Result[Option[IExpressionPE], IParseError] = {
//    val tentativeIter = originalIter.clone()
//
//    val begin = tentativeIter.getPos()
//
//    val isNegative =
//      tentativeIter.peek(2) match {
//        case Vector(SymbolLE(range, '-'), IntLE(intRange, _, _)) => {
//          // Only consider it a negative if it's right next to the next thing
//          if (range.end != intRange.begin) {
//            return Ok(None)
//          }
//          tentativeIter.advance()
//          true
//        }
//        case Vector(IntLE(_, _, _), _) => false
//        case _ => return Ok(None)
//      }
//    val integer =
//      tentativeIter.advance() match {
//        case IntLE(_, innt, _) => innt
//        case _ => return Ok(None)
//      }
//    originalIter.skipTo(tentativeIter)
//
//    if (tentativeIter.trySkipSymbol('.')) {
//      val mantissaPos = tentativeIter.getPos()
//      val mantissa =
//        tentativeIter.advance() match {
//          case IntLE(range, innt, numDigits) => innt.toDouble / numDigits
//          case _ => return Err(BadMantissa(mantissaPos))
//        }
//      val double = (if (isNegative) -1 else 1) * (integer + mantissa)
//      originalIter.skipTo(tentativeIter)
//      Ok(ConstantFloatPE(RangeL(begin, tentativeIter.getPos()), double))
//    } else {
//      if (tentativeIter.trySkipSymbol('i'))
//
//      originalIter.skipTo(tentativeIter)
//      Ok(ConstantIntPE(RangeL(begin, tentativeIter.getPos()), integer, bits))
//    }
//
//    Parser.parseNumber(originalIter) match {
//      case Ok(Some(ParsedInteger(range, int, bits))) => Ok(Some(ConstantIntPE(range, int, bits)))
//      case Ok(Some(ParsedDouble(range, int, bits))) => Ok(Some(ConstantFloatPE(range, int)))
//      case Ok(None) => Ok(None)
//      case Err(e) => Err(e)
//    }
//  }
*/

  /// Parse a spree step (method call, field access, etc.)
  /// Mirrors parseSpreeStep in ExpressionParser.scala lines 1093-1223
  pub fn parse_spree_step(
    &self,
    spree_begin: i32,
    iter: &mut ScrambleIterator<'p, '_>,
    expr_so_far: &'p IExpressionPE<'p>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    let operator_begin = iter.get_pos();

    // Check for & (borrow augmentation)
    if iter.try_skip_symbol('&') {
      let range_pe = AugmentPE {
      range: RangeL(spree_begin, iter.get_prev_end_pos()),
      target_ownership: OwnershipP::Borrow,
        inner: expr_so_far,
      };
      return Ok(Some(IExpressionPE::Augment(range_pe)));
    }

    // Try template lookup
    match self.parse_template_lookup(iter, expr_so_far, templex_parser)? {
      Some(call) => return Ok(Some(IExpressionPE::Lookup(self.parse_arena.alloc(call)))),
      None => {}
    }

    // Try function call
    match self.parse_function_call(
      iter,
      spree_begin,
      expr_so_far,
      templex_parser,
      pattern_parser,
    )? {
      Some(call) => return Ok(Some(call)),
      None => {}
    }

    // Try brace pack (e.g., foo[1, 2, 3])
    match self.parse_brace_pack(iter, templex_parser, pattern_parser)? {
      Some(arg_exprs) => {
        return Ok(Some(IExpressionPE::BraceCall(BraceCallPE {
          range: RangeL(spree_begin, iter.get_prev_end_pos()),
          operator_range: RangeL(operator_begin, iter.get_prev_end_pos()),
          subject_expr: expr_so_far,
          arg_exprs: self.parse_arena.alloc_slice_from_vec(arg_exprs),
          callable_readwrite: false,
        })));
      }
      None => {}
    }

    // Check for range operator (..)
    if iter.try_skip_symbols(&['.', '.']) {
      let operand = self.parse_atom(iter, stop_on_curlied, templex_parser, pattern_parser)?;
      let range_pe = RangePE {
        range: RangeL(spree_begin, iter.get_prev_end_pos()),
        from_expr: expr_so_far,
        to_expr: self.parse_arena.alloc(operand),
      };
      return Ok(Some(IExpressionPE::Range(range_pe)));
    }

    // Check for map call (*.) or method call (.)
    let is_map_call = iter.try_skip_symbols(&['*', '.']);
    let is_method_call = if is_map_call {
      false
    } else {
      iter.try_skip_symbol('.')
    };

    let operator_end = iter.get_prev_end_pos();

    if is_method_call || is_map_call {
      let name_begin = iter.get_pos();
      let name = match iter.peek_cloned() {
        Some(INodeLEEnum::ParsedInteger(ParsedIntegerLE {
          value: int, bits, ..
        })) => {
          let bits = bits.clone();
          iter.advance();
          if int < 0 {
            return Err(ParseError::BadDot(iter.get_pos()));
          }
          if bits.is_some() {
            return Err(ParseError::BadDot(iter.get_pos()));
          }
          NameP(
            RangeL(name_begin, iter.get_prev_end_pos()),
            self.parse_arena.intern_str(&int.to_string()),
          )
        }
        Some(INodeLEEnum::Symbol(_)) => {
          let name = match iter.peek3_cloned() {
            (
              Some(INodeLEEnum::Symbol(SymbolLE(_, '<'))),
              Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
              Some(INodeLEEnum::Symbol(SymbolLE(_, '>'))),
            ) => self.keywords.spaceship,
            (
              Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
              Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
              Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
            ) => self.keywords.triple_equals,
            (
              Some(INodeLEEnum::Symbol(SymbolLE(_, '>'))),
              Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
              _,
            ) => self.keywords.greater_equals,
            (
              Some(INodeLEEnum::Symbol(SymbolLE(_, '<'))),
              Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
              _,
            ) => self.keywords.less_equals,
            (
              Some(INodeLEEnum::Symbol(SymbolLE(_, '!'))),
              Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
              _,
            ) => self.keywords.not_equals,
            (
              Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
              Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
              _,
            ) => self.keywords.double_equals,
            (Some(INodeLEEnum::Symbol(SymbolLE(_, '+'))), _, _) => self.keywords.plus,
            (Some(INodeLEEnum::Symbol(SymbolLE(_, '-'))), _, _) => self.keywords.minus,
            (Some(INodeLEEnum::Symbol(SymbolLE(_, '*'))), _, _) => self.keywords.asterisk,
            (Some(INodeLEEnum::Symbol(SymbolLE(_, '/'))), _, _) => self.keywords.slash,
            _ => return Err(ParseError::BadDot(iter.get_pos())),
          };
          // Advance by the length of the keyword
          for _ in 0..name.as_str().len() {
            iter.advance();
          }
          NameP(
            RangeL(name_begin, iter.get_prev_end_pos()),
            name,
          )
        }
        Some(INodeLEEnum::Word(WordLE { str, .. })) => {
          iter.advance();
          NameP(
            RangeL(name_begin, iter.get_prev_end_pos()),
            str,
          )
        }
        _ => return Err(ParseError::BadDot(iter.get_pos())),
      };

      let maybe_template_args = match self.parse_chevron_pack(iter, templex_parser)? {
        None => None,
        Some(template_args) => Some(TemplateArgsP {
          range: RangeL(operator_begin, iter.get_prev_end_pos()),
          args: self.parse_arena.alloc_slice_from_vec(template_args),
        }),
      };

      match self.parse_pack(iter, templex_parser, pattern_parser)? {
        Some((range, arg_exprs)) => {
          return Ok(Some(IExpressionPE::MethodCall(MethodCallPE {
            range: RangeL(operator_begin, range.end()),
            subject_expr: expr_so_far,
            operator_range: RangeL(operator_begin, operator_end),
            method_lookup: self.parse_arena.alloc(LookupPE {
              name: IImpreciseNameP::LookupName(name),
              template_args: maybe_template_args,
            }),
            arg_exprs: self.parse_arena.alloc_slice_from_vec(arg_exprs),
          })));
        }
        None => {
          if maybe_template_args.is_some() {
            return Err(ParseError::CantTemplateCallMember(iter.get_pos()));
          }

          return Ok(Some(IExpressionPE::Dot(DotPE {
            range: RangeL(spree_begin, iter.get_prev_end_pos()),
            left: expr_so_far,
            operator_range: RangeL(operator_begin, operator_end),
            member: name,
          })));
        }
      }
    }

    Ok(None)
  }
  /*
    def parseSpreeStep(spreeBegin: Int, iter: ScrambleIterator, exprSoFar: IExpressionPE, stopOnCurlied: Boolean):
    Result[Option[IExpressionPE], IParseError] = {
      val operatorBegin = iter.getPos()

      if (iter.trySkipSymbol('&')) {
        val rangePE = AugmentPE(RangeL(spreeBegin, iter.getPrevEndPos()), BorrowP, exprSoFar)
        return Ok(Some(rangePE))
      }

      parseTemplateLookup(iter, exprSoFar) match {
        case Err(e) => return Err(e)
        case Ok(Some(call)) => return Ok(Some(call))
        case Ok(None) =>
      }

      parseFunctionCall(iter, spreeBegin, exprSoFar) match {
        case Err(e) => return Err(e)
        case Ok(Some(call)) => return Ok(Some(call))
        case Ok(None) =>
      }

      parseBracePack(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(args)) => {
          return Ok(
            Some(
              BraceCallPE(
                RangeL(spreeBegin, iter.getPrevEndPos()),
                RangeL(operatorBegin, iter.getPrevEndPos()),
                exprSoFar,
                args,
                false)))
        }
        case Ok(None) =>
      }

      if (iter.trySkipSymbols(Vector('.', '.'))) {
        parseAtom(iter, stopOnCurlied) match {
          case Err(err) => return Err(err)
          case Ok(operand) => {
            val rangePE = RangePE(RangeL(spreeBegin, iter.getPrevEndPos()), exprSoFar, operand)
            return Ok(Some(rangePE))
          }
        }
      }

      val isMapCall = iter.trySkipSymbols(Vector('*', '.'))
      val isMethodCall =
        if (isMapCall) { false }
        else iter.trySkipSymbol('.')

      val operatorEnd = iter.getPrevEndPos()

      if (isMethodCall || isMapCall) {
        val nameBegin = iter.getPos()
        val name =
          iter.peek() match {
            case Some(ParsedIntegerLE(_, int, bits)) => {
              iter.advance()
              if (int < 0) {
                return Err(BadDot(iter.getPos()))
              }
              if (bits.nonEmpty) {
                return Err(BadDot(iter.getPos()))
              }
              NameP(RangeL(nameBegin, iter.getPrevEndPos()), interner.intern(StrI(int.toString)))
            }
            case Some(SymbolLE(_, _)) => {
              val name =
                iter.peek3() match {
                  case (Some(SymbolLE(_, '<')), Some(SymbolLE(_, '=')), Some(SymbolLE(_, '>'))) => keywords.spaceship
                  case (Some(SymbolLE(_, '=')), Some(SymbolLE(_, '=')), Some(SymbolLE(_, '='))) => keywords.tripleEquals
                  case (Some(SymbolLE(_, '>')), Some(SymbolLE(_, '=')), _) => keywords.greaterEquals
                  case (Some(SymbolLE(_, '<')), Some(SymbolLE(_, '=')), _) => keywords.lessEquals
                  case (Some(SymbolLE(_, '!')), Some(SymbolLE(_, '=')), _) => keywords.notEquals
                  case (Some(SymbolLE(_, '=')), Some(SymbolLE(_, '=')), _) => keywords.doubleEquals
                  case (Some(SymbolLE(_, '+')), _, _) => keywords.plus
                  case (Some(SymbolLE(_, '-')), _, _) => keywords.minus
                  case (Some(SymbolLE(_, '*')), _, _) => keywords.asterisk
                  case (Some(SymbolLE(_, '/')), _, _) => keywords.slash
                }
              U.loop(name.str.length, _ => iter.advance())
              NameP(RangeL(nameBegin, iter.getPrevEndPos()), name)
            }
            case Some(WordLE(_, str)) => {
              iter.advance()
              NameP(RangeL(nameBegin, iter.getPrevEndPos()), str)
            }
            case _ => return Err(BadDot(iter.getPos()))
          }

        val maybeTemplateArgs =
          parseChevronPack(iter) match {
            case Err(e) => return Err(e)
            case Ok(None) => None
            case Ok(Some(templateArgs)) => {
              Some(TemplateArgsP(RangeL(operatorBegin, iter.getPrevEndPos()), templateArgs))
            }
          }

        parsePack(iter) match {
          case Err(e) => return Err(e)
          case Ok(Some((range, x))) => {
            return Ok(
              Some(
                MethodCallPE(
                  RangeL(operatorBegin, range.end),
                  exprSoFar,
                  RangeL(operatorBegin, operatorEnd),
                  LookupPE(LookupNameP(name), maybeTemplateArgs),
                  x)))
          }
          case Ok(None) => {
            if (maybeTemplateArgs.nonEmpty) {
              return Err(CantTemplateCallMember(iter.getPos()))
            }

            return Ok(
              Some(
                DotPE(
                  RangeL(spreeBegin, iter.getPrevEndPos()),
                  exprSoFar,
                  RangeL(operatorBegin, operatorEnd),
                  name)))
          }
        }
      }

      Ok(None)
    }
  */

  /// Parse a function call
  /// Mirrors parseFunctionCall in ExpressionParser.scala lines 1224-1245
  pub fn parse_function_call(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
    spree_begin: i32,
    expr_so_far: &'p IExpressionPE<'p>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>>
  {
    let mut tentative_iter = original_iter.clone();
    let operator_begin = tentative_iter.get_pos();

    match self.parse_pack(&mut tentative_iter, templex_parser, pattern_parser)? {
      None => Ok(None),
      Some((range, args)) => {
        original_iter.skip_to(&tentative_iter);
        Ok(Some(IExpressionPE::FunctionCall(FunctionCallPE {
          range: RangeL(spree_begin, range.end()),
          operator_range: RangeL(operator_begin, range.end()),
          callable_expr: expr_so_far,
          arg_exprs: self.parse_arena.alloc_slice_from_vec(args),
        })))
      }
    }
  }
  /*
    def parseFunctionCall(originalIter: ScrambleIterator, spreeBegin: Int, exprSoFar: IExpressionPE):
    Result[Option[IExpressionPE], IParseError] = {
      val tentativeIter = originalIter.clone()
      val operatorBegin = tentativeIter.getPos()

      parsePack(tentativeIter) match {
        case Err(e) => Err(e)
        case Ok(None) => Ok(None)
        case Ok(Some((range, args))) => {
          originalIter.skipTo(tentativeIter)
          val iter = originalIter
          Ok(
            Some(
              FunctionCallPE(
                RangeL(spreeBegin, range.end),
                RangeL(operatorBegin, range.end),
                exprSoFar,
                args)))
        }
      }
    }
  */

  /// Parse an atom and tight suffixes
  /// Mirrors parseAtomAndTightSuffixes in ExpressionParser.scala lines 1246-1272
  pub fn parse_atom_and_tight_suffixes(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<&'p IExpressionPE<'p>> {
    assert!(iter.has_next());
    let begin = iter.get_pos();

    let mut expr_so_far: &'p IExpressionPE<'p> = self.parse_arena.alloc(
      self.parse_atom(iter, stop_on_curlied, templex_parser, pattern_parser)?);

    let mut continuing = true;
    while continuing && iter.has_next() {
      match self.parse_spree_step(
        begin,
        iter,
        expr_so_far,
        stop_on_curlied,
        templex_parser,
        pattern_parser,
      )? {
        None => {
          continuing = false;
        }
        Some(new_expr) => {
          expr_so_far = self.parse_arena.alloc(new_expr);
        }
      }
    }

    Ok(expr_so_far)
  }
  /*
    def parseAtomAndTightSuffixes(iter: ScrambleIterator, stopOnCurlied: Boolean):
    Result[IExpressionPE, IParseError] = {
      vassert(iter.hasNext)
      val begin = iter.getPos()

      var exprSoFar =
        parseAtom(iter, stopOnCurlied) match {
          case Err(err) => return Err(err)
          case Ok(e) => e
        }

      var continuing = true
      while (continuing && iter.hasNext) {
        parseSpreeStep(begin, iter, exprSoFar, stopOnCurlied) match {
          case Err(err) => return Err(err)
          case Ok(None) => {
            continuing = false
          }
          case Ok(Some(newExpr)) => {
            exprSoFar = newExpr
          }
        }
      }

      Ok(exprSoFar)
    }
  */

  /// Parse chevron pack (template arguments)
  /// Mirrors parseChevronPack in ExpressionParser.scala lines 1273-1292
  pub fn parse_chevron_pack(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
  ) -> ParseResult<Option<Vec<&'p ITemplexPT<'p>>>>
  {
    match iter.peek_cloned() {
      Some(INodeLEEnum::Angled(AngledLE { contents, .. })) => {
        let contents = contents.clone();
        iter.advance();

        let scramble = ScrambleIterator::new(&contents);
        let element_iters = scramble.split_on_symbol(',', false);

        let mut result: Vec<&'p ITemplexPT<'p>> = vec![];
        for mut element_iter in element_iters {
          let templex = templex_parser.parse_templex(&mut element_iter)?;
          result.push(&*self.parse_arena.alloc(templex));
        }

        Ok(Some(result))
      }
      _ => Ok(None),
    }
  }
  /*
    def parseChevronPack(iter: ScrambleIterator): Result[Option[Vector[ITemplexPT]], IParseError] = {
      iter.peek() match {
        case Some(AngledLE(range, innerScramble)) => {
          iter.advance()

          Ok(
            Some(
              U.map[ScrambleIterator, ITemplexPT](
                new ScrambleIterator(innerScramble).splitOnSymbol(',', false),
                elementIter => {
                  templexParser.parseTemplex(elementIter) match {
                    case Err(e) => return Err(e)
                    case Ok(x) => x
                  }
                }).toVector))
        }
        case _ => Ok(None)
      }
    }
  */

  /// Parse a template lookup
  /// Mirrors parseTemplateLookup in ExpressionParser.scala lines 1293-1313
  pub fn parse_template_lookup(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    expr_so_far: &'p IExpressionPE<'p>,
    templex_parser: &TemplexParser<'p, 'ctx>,
  ) -> ParseResult<Option<LookupPE<'p>>> {
    let operator_begin = iter.get_pos();

    let template_args = match self.parse_chevron_pack(iter, templex_parser)? {
      None => return Ok(None),
      Some(template_args) => TemplateArgsP {
        range: RangeL(operator_begin, iter.get_prev_end_pos()),
        args: self.parse_arena.alloc_slice_from_vec(template_args),
      },
    };

    let result_pe = match expr_so_far {
      IExpressionPE::Lookup(lookup) if lookup.template_args.is_none() => LookupPE {
        name: lookup.name.clone(),
        template_args: Some(template_args),
      },
      _ => return Err(ParseError::BadTemplateCallee(operator_begin)),
    };

    Ok(Some(result_pe))
  }
  /*
    def parseTemplateLookup(iter: ScrambleIterator, exprSoFar: IExpressionPE): Result[Option[LookupPE], IParseError] = {
      val operatorBegin = iter.getPos()

      val templateArgs =
        parseChevronPack(iter) match {
          case Err(e) => return Err(e)
          case Ok(None) => return Ok(None)
          case Ok(Some(templateArgs)) => {
            ast.TemplateArgsP(RangeL(operatorBegin, iter.getPrevEndPos()), templateArgs)
          }
        }

      val resultPE =
        exprSoFar match {
          case LookupPE(name, None) => ast.LookupPE(name, Some(templateArgs))
          case _ => return Err(BadTemplateCallee(operatorBegin))
        }

      Ok(Some(resultPE))
    }
  */

  /// Parse a pack (parens, squares, or curlies)
  /// Mirrors parsePack in ExpressionParser.scala lines 1314-1333
  pub fn parse_pack(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<(RangeL, Vec<&'p IExpressionPE<'p>>)>> {
    let parend_le = match iter.peek_cloned() {
      Some(INodeLEEnum::Parend(p)) => {
        let p = p.clone();
        iter.advance();
        p
      }
      _ => return Ok(None),
    };

    let segment_iters = ScrambleIterator::new(&parend_le.contents).split_on_symbol(',', false);

    let mut elements = vec![];
    for mut element_iter in segment_iters {
      let expr = self.parse_expression(&mut element_iter, false, templex_parser, pattern_parser)?;
      elements.push(expr);
    }

    Ok(Some((parend_le.range, elements)))
  }
  /*
    def parsePack(iter: ScrambleIterator):
    Result[Option[(RangeL, Vector[IExpressionPE])], IParseError] = {
      val parendLE =
        iter.peek() match {
          case Some(p @ ParendLE(_, _)) => iter.advance(); p
          case _ => return Ok(None)
        }

      val elements =
        U.map[ScrambleIterator, IExpressionPE](
          new ScrambleIterator(parendLE.contents).splitOnSymbol(',', false),
          elementIter => {
            parseExpression(elementIter, false) match {
              case Err(e) => return Err(e)
              case Ok(expr) => expr
            }
        })
      Ok(Some((parendLE.range, elements.toVector)))
    }
  */

  /// Parse a square pack (array/seq literal)
  /// Mirrors parseSquarePack in ExpressionParser.scala lines 1334-1352
  pub fn parse_square_pack(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<Vec<&'p IExpressionPE<'p>>>> {
    let squared_le = match iter.peek_cloned() {
      Some(INodeLEEnum::Squared(p)) => {
        let p = p.clone();
        iter.advance();
        p
      }
      None => return Ok(None),
      _ => return Ok(None),
    };

    let segment_iters = ScrambleIterator::new(&squared_le.contents).split_on_symbol(',', false);

    let mut elements_p = vec![];
    for mut element_iter in segment_iters {
      let expr = self.parse_expression(&mut element_iter, false, templex_parser, pattern_parser)?;
      elements_p.push(expr);
    }

    Ok(Some(elements_p))
  }
  /*
    def parseSquarePack(iter: ScrambleIterator): Result[Option[Vector[IExpressionPE]], IParseError] = {
      val squaredLE =
        iter.peek() match {
          case Some(p @ SquaredLE(_, _)) => iter.advance(); p
          case None => return Ok(None)
        }

      val elementsP =
        U.map[ScrambleIterator, IExpressionPE](
          new ScrambleIterator(squaredLE.contents).splitOnSymbol(',', false),
          elementIter => {
            parseExpression(elementIter, false) match {
              case Err(e) => return Err(e)
              case Ok(expr) => expr
            }
          })
      Ok(Some(elementsP.toVector))
    }
  */

  /// Parse a brace pack
  /// Mirrors parseBracePack in ExpressionParser.scala lines 1353-1371
  pub fn parse_brace_pack(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<Vec<&'p IExpressionPE<'p>>>> {
    match iter.peek_cloned() {
      Some(INodeLEEnum::Squared(SquaredLE { contents, .. })) => {
        let contents = contents.clone();
        iter.advance();

        let scramble_iter = ScrambleIterator::new(&contents);
        let element_iters: Vec<ScrambleIterator<'p, '_>> = scramble_iter.split_on_symbol(',', false);

        let mut elements = vec![];
        for mut element_iter in element_iters {
          let expr =
            self.parse_expression(&mut element_iter, false, templex_parser, pattern_parser)?;
          elements.push(expr);
        }

        Ok(Some(elements))
      }
      _ => Ok(None),
    }
  }
  /*
    def parseBracePack(iter: ScrambleIterator): Result[Option[Vector[IExpressionPE]], IParseError] = {
      iter.peek() match {
        case Some(SquaredLE(_, contents)) => {
          iter.advance()
          val elements =
            U.map[ScrambleIterator, IExpressionPE](
              new ScrambleIterator(contents).splitOnSymbol(',', false),
              elementIter => {
                parseExpression(elementIter, false) match {
                  case Err(e) => return Err(e)
                  case Ok(expr) => expr
                }
              })
          Ok(Some(elements.toVector))
        }
        case _ => Ok(None)
      }
    }
  */

  /// Parse a tuple or sub-expression
  /// Mirrors parseTupleOrSubExpression in ExpressionParser.scala lines 1372-1417
  pub fn parse_tuple_or_sub_expression(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    match iter.peek_cloned() {
      Some(INodeLEEnum::Parend(ParendLE { range, contents })) => {
        let contents = contents.clone();
        iter.advance();

        let mut iters = ScrambleIterator::new(&contents).split_on_symbol(',', true);

        assert!(!iters.is_empty());

        if iters.len() == 1 {
          if !iters[0].has_next() {
            // Then we have e.g. ()
            return Ok(Some(IExpressionPE::Tuple(TuplePE {
              range,
              elements: self.parse_arena.alloc_slice_from_vec(vec![]),
            })));
          } else {
            // Then we have e.g. (true)
            let inner =
              self.parse_expression(&mut iters[0], false, templex_parser, pattern_parser)?;
            return Ok(Some(IExpressionPE::SubExpression(SubExpressionPE {
              range,
              inner: self.parse_arena.alloc(inner),
            })));
          }
        } else {
          // Then we have e.g. (true,) or (true,true) etc.
          // Mirrors ExpressionParser.scala lines 1394-1400
          let mut element_iters = if !iters.last().unwrap().has_next() {
            // Last is empty, like in (true,) so take it out
            iters.pop();
            iters
          } else {
            iters
          };

          let mut elements_p = vec![];
          for element_iter in element_iters.iter_mut() {
            let expr =
              self.parse_expression(element_iter, false, templex_parser, pattern_parser)?;
            elements_p.push(expr);
          }

          return Ok(Some(IExpressionPE::Tuple(TuplePE {
            range,
            elements: self.parse_arena.alloc_slice_from_vec(elements_p),
          })));
        }
      }
      _ => Ok(None),
    }
  }
  /*
    def parseTupleOrSubExpression(iter: ScrambleIterator): Result[Option[IExpressionPE], IParseError] = {
      iter.peek() match {
        case Some(ParendLE(range, contents)) => {
          iter.advance()
          val iters =
            new ScrambleIterator(contents).splitOnSymbol(',', true)
          vassert(iters.nonEmpty)
          if (iters.length == 1) {
            if (!iters.head.hasNext) {
              // Then we have e.g. ()
              return Ok(Some(TuplePE(range, Vector())))
            } else {
              // Then we have e.g. (true)
              val inner =
                parseExpression(iters.head, false) match {
                  case Err(e) => return Err(e)
                  case Ok(x) => x
                }
              return Ok(Some(SubExpressionPE(range, inner)))
            }
          } else {
            // Then we have e.g. (true,) or (true,true) etc.
            val elementIters =
              if (!iters.last.hasNext) {
                // Last is empty, like in (true,) so take it out
                iters.init
              } else {
                iters
              }
            val elementsP =
              U.map[ScrambleIterator, IExpressionPE](
                elementIters,
                elementIter => {
                  parseExpression(elementIter, false) match {
                    case Err(e) => return Err(e)
                    case Ok(x) => x
                  }
                })
            Ok(Some(TuplePE(range, elementsP.toVector)))
          }
        }
        case _ => Ok(None)
      }
    }
  */

  /// Parse expression data element
  /// Mirrors parseExpressionDataElement in ExpressionParser.scala lines 1418-1543
  pub fn parse_expression_data_element(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    stop_on_curlied: bool,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<&'p IExpressionPE<'p>> {
    assert!(iter.has_next());

    let begin = iter.get_pos();

    // Handle … symbol (Scala line 1422-1424)
    if iter.try_skip_symbol('…') {
      return Ok(self.parse_arena.alloc(IExpressionPE::ConstantInt(ConstantIntPE {
        range: RangeL::new(begin, iter.get_prev_end_pos()),
        value: 0,
        bits: None,
      })));
    }

    // Handle single quote prefix (Scala line 1426-1432)
    match iter.peek2_cloned() {
      (Some(INodeLEEnum::Symbol(SymbolLE(_, '\''))), Some(INodeLEEnum::Word(_))) => {
        iter.advance();
        iter.advance();
        return self.parse_expression_data_element(
          iter,
          stop_on_curlied,
          templex_parser,
          pattern_parser,
        );
      }
      _ => {}
    }

    // Handle 'not' keyword (Scala line 1438-1445)
    if iter.try_skip_word(self.keywords.not).is_some() {
      let inner_pe = self.parse_expression_data_element(
        iter,
        stop_on_curlied,
        templex_parser,
        pattern_parser,
      )?;
      let end = inner_pe.range().end();
      return Ok(self.parse_arena.alloc(IExpressionPE::Not(NotPE {
        range: RangeL::new(begin, end),
        inner: inner_pe,
      })));
    }

    // Handle lone blocks (Scala line 1447-1451)
    if let Some(block) = self.parse_lone_block(iter, templex_parser, pattern_parser)? {
      return Ok(self.parse_arena.alloc(block));
    }

    // Handle if ladders (Scala line 1453-1457)
    if let Some(if_expr) = self.parse_if_ladder(iter, templex_parser, pattern_parser)? {
      return Ok(self.parse_arena.alloc(if_expr));
    }

    // Handle destruct (Scala line 1461-1465)
    if let Some(destruct) =
      self.parse_destruct(iter, stop_on_curlied, templex_parser, pattern_parser)?
    {
      return Ok(self.parse_arena.alloc(destruct));
    }

    // Handle foreach (Scala line 1467-1471)
    if let Some(foreach) = self.parse_foreach(iter, templex_parser, pattern_parser)? {
      return Ok(self.parse_arena.alloc(foreach));
    }

    // Handle unlet (Scala line 1473-1477)
    if let Some(unlet) = self.parse_unlet(iter)? {
      return Ok(self.parse_arena.alloc(unlet));
    }

    // Handle transmigration region'expr (Scala line 1479-1493)
    match iter.peek2_cloned() {
      (
        Some(INodeLEEnum::Word(WordLE {
          range: region_range,
          str: region,
        })),
        Some(INodeLEEnum::Symbol(SymbolLE(_, '\''))),
      ) => {
        let region_name = NameP(region_range, region);
        iter.advance();
        iter.advance();
        let inner_pe = self.parse_atom_and_tight_suffixes(
          iter,
          stop_on_curlied,
          templex_parser,
          pattern_parser,
        )?;
        return Ok(self.parse_arena.alloc(IExpressionPE::Transmigrate(TransmigratePE {
          range: RangeL::new(begin, iter.get_prev_end_pos()),
          target_region: region_name,
          inner: inner_pe,
        })));
      }
      _ => {}
    }

    // Handle ownership prefixes ^ & && inl (Scala line 1495-1531)
    let maybe_target_ownership = match iter.peek_cloned() {
      Some(INodeLEEnum::Symbol(SymbolLE(_, '^'))) => {
        iter.advance();
        Some(OwnershipP::Own)
      }
      Some(INodeLEEnum::Symbol(SymbolLE(_, '&'))) => {
        iter.advance();
        match iter.peek_cloned() {
          Some(INodeLEEnum::Symbol(SymbolLE(_, '&'))) => {
            iter.advance();
            Some(OwnershipP::Weak)
          }
          _ => Some(OwnershipP::Borrow),
        }
      }
      Some(INodeLEEnum::Word(WordLE { str, .. })) if str == self.keywords.r#inl => {
        iter.advance();
        Some(OwnershipP::Own)
      }
      _ => None,
    };

    if let Some(target_ownership) = maybe_target_ownership {
      let inner_pe = self.parse_atom_and_tight_suffixes(
        iter,
        stop_on_curlied,
        templex_parser,
        pattern_parser,
      )?;
      return Ok(self.parse_arena.alloc(IExpressionPE::Augment(AugmentPE {
        range: RangeL::new(begin, iter.get_prev_end_pos()),
        target_ownership,
        inner: inner_pe,
      })));
    }

    // Now parse the atom and tight suffixes (Scala line 1541)
    self.parse_atom_and_tight_suffixes(iter, stop_on_curlied, templex_parser, pattern_parser)
  }
  /*
    // An expression data element is an expression without binary operators. It has a definite end.
    def parseExpressionDataElement(iter: ScrambleIterator, stopOnCurlied: Boolean): Result[IExpressionPE, IParseError] = {
      vassert(iter.hasNext)

      val begin = iter.getPos()
      if (iter.trySkipSymbol('…')) {
        return Ok(ConstantIntPE(RangeL(begin, iter.getPrevEndPos()), 0, None))
      }

      iter.peek2() match {
        case (Some(SymbolLE(_, '\'')), Some(WordLE(range, str))) => {
          iter.advance()
          iter.advance()
          return parseExpressionDataElement(iter, stopOnCurlied)
        }
        case _ =>
      }

      // First, get the prefixes out of the way, such as & not etc.
      // Then we'll parse the atom and suffixes (.moo, ..5, etc.) and
      // *then* wrap those in the prefixes, so we get e.g. not(x.moo)
      if (iter.trySkipWord(keywords.not).nonEmpty) {
        val innerPE =
          parseExpressionDataElement(iter, stopOnCurlied) match {
            case Err(e) => return Err(e)
            case Ok(x) => x
          }
        return Ok(NotPE(RangeL(begin, innerPE.range.end), innerPE))
      }

      parseLoneBlock(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      parseIfLadder(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(e)) => return Ok(e)
        case Ok(None) =>
      }

      // This is here so we can do things like: [name] = destruct event;
      parseDestruct(iter, stopOnCurlied) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      parseForeach(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(e)) => return Ok(e)
        case Ok(None) =>
      }

      parseUnlet(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      iter.peek2() match {
        case (Some(WordLE(regionRange, region)), Some(SymbolLE(_, '\''))) => {
          iter.advance()
          iter.advance()
          val regionName = NameP(regionRange, region)
          val innerPE =
            parseAtomAndTightSuffixes(iter, stopOnCurlied) match {
              case Err(err) => return Err(err)
              case Ok(e) => e
            }
          val transmigratePE = TransmigratePE(RangeL(begin, iter.getPrevEndPos()), regionName, innerPE)
          return Ok(transmigratePE)
        }
        case _ =>
      }

      val maybeTargetOwnership =
        iter.peek() match {
          case Some(SymbolLE(range, '^')) => {
            iter.advance()
            Some(OwnP)
          }
          case Some(SymbolLE(range, '&')) => {
            iter.advance()
            iter.peek() match {
              case Some(SymbolLE(range, '&')) => {
                iter.advance()
                Some(WeakP)
              }
              case _ => {
                Some(BorrowP)
              }
            }
          }
          // This is just a hack to get the syntax highlighter to highlight inl
          case Some(WordLE(range, inl)) if inl == keywords.inl => {
            iter.advance()
            Some(OwnP)
          }
          case _ => None
        }
      maybeTargetOwnership match {
        case Some(targetOwnership) => {
          val innerPE =
            parseAtomAndTightSuffixes(iter, stopOnCurlied) match {
              case Err(err) => return Err(err)
              case Ok(e) => e
            }
          val augmentPE = ast.AugmentPE(RangeL(begin, iter.getPrevEndPos()), targetOwnership, innerPE)
          return Ok(augmentPE)
        }
        case None =>
      }

      // Now, do some "right recursion"; parse the atom (e.g. true, 4, x)
      // and then parse any suffixes, like
      // .moo
      // .foo(5)
      // ..5
      // which all have tighter precedence than the prefixes.
      // Then we'll ret, and our callers will wrap it in the prefixes
      // like & not etc.
      return parseAtomAndTightSuffixes(iter, stopOnCurlied)
    }
  */



  /// Parse a braced body
  /// Mirrors parseBracedBody in ExpressionParser.scala lines 1544-1561
  pub fn parse_braced_body(&self, _iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<BlockPE<'p>> {
    panic!("parse_braced_body: NOT IMPLEMENTED - marked vimpl() in Scala ExpressionParser.scala line 1545")
  }
  /*
    def parseBracedBody(iter: ScrambleIterator):  Result[BlockPE, IParseError] = {
      vimpl()
  //    if (iter.trySkipWord("\\s*\\{").isEmpty) {
  //      return Ok(None)
  //    }
  //
  //    val bodyBegin = iter.getPos()
  //    val bodyContents =
  //      parseBlockContents(iter) match {
  //        case Err(e) => return Err(e)
  //        case Ok(x) => x
  //      }
  //    if (iter.trySkipWord("\\}").isEmpty) {
  //      vwat()
  //    }
  //    Ok(Some(ast.BlockPE(RangeL(bodyBegin, iter.getPos()), bodyContents)))
    }
  */

  /// Parse single-arg lambda begin
  /// Mirrors parseSingleArgLambdaBegin in ExpressionParser.scala lines 1562-1585
  pub fn parse_single_arg_lambda_begin(
    &self,
    _original_iter: &mut ScrambleIterator<'p, '_>,
  ) -> Option<ParamsP<'p>> {
    panic!("parse_single_arg_lambda_begin: NOT IMPLEMENTED - marked vimpl() in Scala ExpressionParser.scala line 1563")
  }
  /*
    def parseSingleArgLambdaBegin(originalIter: ScrambleIterator): Option[ParamsP] = {
      vimpl()
  //    val tentativeIter = originalIter.clone()
  //    val begin = tentativeIter.getPos()
  //    val argName =
  //      Parser.parseLocalOrMemberName(tentativeIter) match {
  //        case None => return None
  //        case Some(n) => n
  //      }
  //    val paramsEnd = tentativeIter.getPos()
  //
  //    tentativeIter.consumeWhitespace()
  //    if (!tentativeIter.trySkipWord("=>")) {
  //      return None
  //    }
  //
  //    originalIter.skipTo(tentativeIter.position)
  //
  //    val range = RangeL(begin, paramsEnd)
  //    val capture = LocalNameDeclarationP(argName)
  //    val pattern = PatternPP(RangeL(begin, paramsEnd), None, Some(capture), None, None, None)
  //    Some(ParamsP(range, Vector(pattern)))
    }
  */

  /// Parse multi-arg lambda begin
  /// Mirrors parseMultiArgLambdaBegin in ExpressionParser.scala lines 1586-1634
  pub fn parse_multi_arg_lambda_begin(
    &self,
    _original_iter: &mut ScrambleIterator<'p, '_>,
  ) -> Option<ParamsP<'p>> {
    panic!("parse_multi_arg_lambda_begin: NOT IMPLEMENTED - marked vimpl() in Scala ExpressionParser.scala line 1587")
  }
  /*
    def parseMultiArgLambdaBegin(originalIter: ScrambleIterator): Option[ParamsP] = {
      vimpl()
  //    val tentativeIter = originalIter.clone()
  //
  //    val begin = tentativeIter.getPos()
  //    if (!tentativeIter.trySkipWord("\\s*\\(")) {
  //      return None
  //    }
  //    tentativeIter.consumeWhitespace()
  //    val patterns = new mutable.ArrayBuffer[PatternPP]()
  //
  //    while (!tentativeIter.trySkipWord("\\s*\\)")) {
  //      val pattern =
  //        new PatternParser().parsePattern(tentativeIter) match {
  //          case Ok(result) => result
  //          case Err(cpe) => return None
  //        }
  //      patterns += pattern
  //      tentativeIter.consumeWhitespace()
  //      if (tentativeIter.peek(() => "^\\s*,\\s*\\)")) {
  //        val found = tentativeIter.trySkipWord("\\s*,")
  //        vassert(found)
  //        vassert(tentativeIter.peek(() => "^\\s*\\)"))
  //      } else if (tentativeIter.trySkipWord("\\s*,")) {
  //        // good, continue
  //      } else if (tentativeIter.peek(() => "^\\s*\\)")) {
  //        // good, continue
  //      } else {
  //        // At some point, we should return an error here.
  //        // With a pre-parser that looks for => it would be possible.
  //        return None
  //      }
  //      tentativeIter.consumeWhitespace()
  //    }
  //
  //    val paramsEnd = tentativeIter.getPos()
  //
  //    tentativeIter.consumeWhitespace()
  //    if (!tentativeIter.trySkipWord("=>")) {
  //      return None
  //    }
  //
  //    val params = ast.ParamsP(RangeL(begin, paramsEnd), patterns.toVector)
  //
  //    originalIter.skipTo(tentativeIter.position)
  //
  //    Some(params)
    }
  */

  /// Parse a lambda
  /// Mirrors parseLambda in ExpressionParser.scala lines 1635-1728
  pub fn parse_lambda(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    let begin = iter.get_pos();

    let header_p = match iter.peek3_cloned() {
      // Just a curlied block with no params (e.g., { ... })
      (Some(INodeLEEnum::Curlied(CurliedLE { range, .. })), _, _) => {
        let retuurn = FunctionReturnP {
          range: RangeL(iter.get_pos(), iter.get_pos()),
          ret_type: None,
        };
        // Don't iter.advance() because we still need to parse this later
        FunctionHeaderP {
          range,
          name: None,
          attributes: self.parse_arena.alloc_slice_from_vec(vec![]),
          generic_parameters: None,
          template_rules: None,
          params: None,
          ret: retuurn,
        }
      }
      // Single param lambda: x => ...
      (
        Some(INodeLEEnum::Word(WordLE {
          range: param_range,
          str: param_name,
        })),
        Some(INodeLEEnum::Symbol(SymbolLE(eq_range, '='))),
        Some(INodeLEEnum::Symbol(SymbolLE(gt_range, '>'))),
      ) => {
        if eq_range.end() != gt_range.begin() {
          return Err(ParseError::BadLambdaBegin(eq_range.begin()));
        }
        iter.advance();
        iter.advance();
        iter.advance();

        let param = ParameterP {
          range: param_range,
          virtuality: None,
          maybe_pre_checked: None,
          self_borrow: None,
          pattern: Some(PatternPP {
            range: param_range,
            destination: Some(DestinationLocalP {
              decl: INameDeclarationP::LocalNameDeclaration(NameP(param_range, param_name)),
              mutate: None,
            }),
            templex: None,
            destructure: None,
          }),
        };
        let params = ParamsP {
          range: param_range,
          params: self.parse_arena.alloc_slice_from_vec(vec![param]),
        };
        let retuurn = FunctionReturnP {
          range: RangeL(iter.get_pos(), iter.get_pos()),
          ret_type: None,
        };
        let range = RangeL(begin, iter.get_prev_end_pos());
        FunctionHeaderP {
          range,
          name: None,
          attributes: self.parse_arena.alloc_slice_from_vec(vec![]),
          generic_parameters: None,
          template_rules: None,
          params: Some(params),
          ret: retuurn,
        }
      }
      // Multi-param lambda: (x, y) => ...
      (
        Some(INodeLEEnum::Parend(ParendLE {
          range: params_range,
          contents: params_contents,
        })),
        Some(INodeLEEnum::Symbol(SymbolLE(eq_range, '='))),
        Some(INodeLEEnum::Symbol(SymbolLE(gt_range, '>'))),
      ) => {
        let params_contents = params_contents.clone();
        if eq_range.end() != gt_range.begin() {
          return Err(ParseError::BadLambdaBegin(eq_range.begin()));
        }
        iter.advance();
        iter.advance();
        iter.advance();

        let param_iters = ScrambleIterator::new(&params_contents).split_on_symbol(',', false);

        let mut patterns = vec![];
        for (index, mut pattern_iter) in param_iters.into_iter().enumerate() {
          let param = pattern_parser.parse_parameter(
            &mut pattern_iter,
            templex_parser,
            index,
            false,
            true,
            true,
          )?;
          patterns.push(param);
        }

        let params_p = ParamsP {
          range: params_range,
          params: self.parse_arena.alloc_slice_from_vec(patterns),
        };
        let retuurn = FunctionReturnP {
          range: RangeL(iter.get_pos(), iter.get_pos()),
          ret_type: None,
        };
        let range = RangeL(begin, iter.get_prev_end_pos());
        FunctionHeaderP {
          range,
          name: None,
          attributes: self.parse_arena.alloc_slice_from_vec(vec![]),
          generic_parameters: None,
          template_rules: None,
          params: Some(params_p),
          ret: retuurn,
        }
      }
      (_, _, _) => return Ok(None),
    };

    // Mirrors ExpressionParser.scala lines 1693-1723
    let body_p = match iter.peek_cloned() {
      Some(INodeLEEnum::Curlied(block_l)) => {
        let block_l = block_l.clone();
        iter.advance();
        // parseBlock returns IExpressionPE, we need to wrap it in BlockPE
        // Scala lines 1697-1707
        let statements_p = self.parse_block(&block_l, templex_parser, pattern_parser)?;
        BlockPE {
          range: block_l.range,
          maybe_pure: None,
          // Would we ever want a lambda with a different default region?
          maybe_default_region: None,
          inner: self.parse_arena.alloc(statements_p),
        }
      }
      Some(_) => {
        // Scala lines 1709-1720
        let result = self.parse_expression(iter, false, templex_parser, pattern_parser)?;
        BlockPE {
          range: result.range(),
          maybe_pure: None,
          // Would we ever want a lambda with a different default region?
          maybe_default_region: None,
          inner: self.parse_arena.alloc(result),
        }
      }
      None => panic!("LAMBDA_MISSING_BODY: Expected body for lambda - not in Scala"),
    };

    let lam = LambdaPE {
      captures: None,
      function: FunctionP {
        range: RangeL(begin, iter.get_prev_end_pos()),
        header: header_p,
        body: Some(self.parse_arena.alloc(body_p)),
      },
    };

    Ok(Some(IExpressionPE::Lambda(lam)))
  }
  /*
    def parseLambda(iter: ScrambleIterator): Result[Option[IExpressionPE], IParseError] = {
      val begin = iter.getPos()
      val headerP =
        iter.peek3() match {
          case (Some(CurliedLE(range, contents)), _, _) => {
            val retuurn = FunctionReturnP(RangeL(iter.getPos(), iter.getPos()), None)
            // Don't iter.advance() because we still need to parse this later
            FunctionHeaderP(range, None, Vector(), None, None, None, retuurn)
          }
          case (Some(CurliedLE(range, contents)), _, _) => {
            val retuurn = FunctionReturnP(RangeL(iter.getPos(), iter.getPos()), None)
            // Don't iter.advance() because we still need to parse this later
            FunctionHeaderP(range, None, Vector(), None, None, None, retuurn)
          }
          case (Some(WordLE(paramRange, paramName)), Some(SymbolLE(eqRange, '=')), Some(SymbolLE(gtRange, '>'))) => {
            if (eqRange.end != gtRange.begin) {
              return Err(BadLambdaBegin(eqRange.begin))
            }
            iter.advance()
            iter.advance()
            iter.advance()
            val param =
              ParameterP(
                paramRange,
                None,
                None,
                None,
                Some(PatternPP(paramRange, Some(DestinationLocalP(LocalNameDeclarationP(NameP(paramRange, paramName)), None)), None, None)))
            val params = ParamsP(paramRange, Vector(param))
            val retuurn = FunctionReturnP(RangeL(iter.getPos(), iter.getPos()), None)
            val range = RangeL(begin, iter.getPrevEndPos())
            FunctionHeaderP(range, None, Vector(), None, None, Some(params), retuurn)
          }
          case (Some(ParendLE(paramsRange, paramsContents)), Some(SymbolLE(eqRange, '=')), Some(SymbolLE(gtRange, '>'))) => {
            if (eqRange.end != gtRange.begin) {
              return Err(BadLambdaBegin(eqRange.begin))
            }
            iter.advance()
            iter.advance()
            iter.advance()
            val paramsP =
              ParamsP(
                paramsRange,
                U.mapWithIndex[ScrambleIterator, ParameterP](
                  new ScrambleIterator(paramsContents).splitOnSymbol(',', false),
                  (index, patternIter) => {
                    patternParser.parseParameter(patternIter, index, false, true, true) match {
                      case Err(e) => return Err(e)
                      case Ok(x) => x
                    }
                  }))
            val retuurn = FunctionReturnP(RangeL(iter.getPos(), iter.getPos()), None)
            val range = RangeL(begin, iter.getPrevEndPos())
            FunctionHeaderP(range, None, Vector(), None, None, Some(paramsP), retuurn)
          }
          case (_, _, _) => return Ok(None)
        }

      val bodyP =
        iter.peek() match {
          case Some(blockL@CurliedLE(range, contents)) => {
            iter.advance()
            val statementsP =
              parseBlock(blockL) match {
                case Err(err) => return Err(err)
                case Ok(result) => result
              }
            BlockPE(
              blockL.range,
              None,
              // Would we ever want a lambda with a different default region?
              None,
              statementsP)
          }
          case Some(_) => {
            parseExpression(iter, false) match {
              case Err(err) => return Err(err)
              case Ok(result) => {
                BlockPE(
                  result.range,
                  None,
                  // Would we ever want a lambda with a different default region?
                  None,
                  result)
              }
            }
          }
          case _ => vwat()
        }

      val lam = LambdaPE(None, FunctionP(RangeL(begin, iter.getPrevEndPos()), headerP, Some(bodyP)))
      Ok(Some(lam))
    }
  */

  /// Parse an array literal
  /// Mirrors parseArray in ExpressionParser.scala lines 1729-1822
  pub fn parse_array(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_parser: &PatternParser<'p, 'ctx>,
  ) -> ParseResult<Option<IExpressionPE<'p>>> {
    let mut tentative_iter = original_iter.clone();
    let begin = tentative_iter.get_pos();

    let mutability = if tentative_iter.try_skip_symbol('#') {
      ITemplexPT::Mutability(MutabilityPT(
        RangeL(begin, tentative_iter.get_prev_end_pos()),
        MutabilityP::Immutable,
      ))
    } else {
      ITemplexPT::Mutability(MutabilityPT(RangeL(begin, begin), MutabilityP::Mutable))
    };

    // If there's no square, we're not making an array.
    let sizer = match tentative_iter.peek_cloned() {
      Some(INodeLEEnum::Squared(s)) => s.clone(),
      _ => return Ok(None),
    };
    tentative_iter.advance();

    let is_array = match tentative_iter.peek_cloned() {
      // If there's nothing after the square brackets, it's not an array.
      None => false,
      Some(INodeLEEnum::Symbol(SymbolLE(_, '.'))) => false,
      _ => true,
    };

    if !is_array {
      // Not an array, bail.
      // TODO: Someday, we could interpret this occurrence as a way to make a List.
      return Ok(None);
    }

    original_iter.skip_to(&tentative_iter);
    let iter = original_iter;

    let sizer_contents = sizer.contents.clone();
    let mut sizer_iter = ScrambleIterator::new(&sizer_contents);
    let size = if sizer_iter.try_skip_symbol('#') {
      let size_pt = if sizer_iter.has_next() {
        Some(templex_parser.parse_templex(&mut sizer_iter)?)
      } else {
        None
      };
      IArraySizeP::StaticSized(StaticSizedArraySizeP { size_pt })
    } else {
      IArraySizeP::RuntimeSized
    };

    let tyype = match iter.peek_cloned() {
      Some(INodeLEEnum::Parend(_)) => None,
      Some(_) => Some(templex_parser.parse_templex(iter)?),
      None => return Err(ParseError::BadArraySpecifier(iter.get_pos())),
    };

    let args = match self.parse_pack(iter, templex_parser, pattern_parser)? {
      None => return Err(ParseError::BadArraySpecifier(iter.get_pos())),
      Some((_range, e)) => e,
    };

    let initializing_individual_elements = match &size {
      IArraySizeP::StaticSized(StaticSizedArraySizeP { size_pt: None }) => true, // e.g. [#](3, 4, 5)
      IArraySizeP::StaticSized(StaticSizedArraySizeP { size_pt: Some(_) }) => false, // Anything else should take a function.
      IArraySizeP::RuntimeSized => false, // Any runtime sized array should take a function.
    };

    let array_pe = ConstructArrayPE {
      range: RangeL(begin, iter.get_prev_end_pos()),
      type_pt: tyype,
      mutability_pt: Some(mutability),
      variability_pt: None,
      size,
      initializing_individual_elements,
      args: self.parse_arena.alloc_slice_from_vec(args),
    };

    Ok(Some(IExpressionPE::ConstructArray(array_pe)))
  }
  /*
    def parseArray(originalIter: ScrambleIterator): Result[Option[IExpressionPE], IParseError] = {
      val tentativeIter = originalIter.clone()
      val begin = tentativeIter.getPos()

      val mutability =
        if (tentativeIter.trySkipSymbol('#')) {
          MutabilityPT(RangeL(begin, tentativeIter.getPrevEndPos()), ImmutableP)
        } else {
          MutabilityPT(RangeL(begin, begin), MutableP)
        }

      // If there's no square, we're not making an array.
      val sizer =
        tentativeIter.peek() match {
          case Some(s @ SquaredLE(_, _)) => s
          case _ => return Ok(None)
        }
      tentativeIter.advance()


      val isArray =
        tentativeIter.peek() match {
          // If there's nothing after the square brackets, it's not an array.
          case None => false
          case Some(SymbolLE(range, '.')) => false
          case _ => true
        }

      if (!isArray) {
        // Not an array, bail.
        // TODO: Someday, we could interpret this occurrence as a way to make a List.
        return Ok(None)
      }

      originalIter.skipTo(tentativeIter)
      val iter = originalIter

      val sizerIter = new ScrambleIterator(sizer.contents)
      val size =
        if (sizerIter.trySkipSymbol('#')) {
          val sizeTemplex =
            if (sizerIter.hasNext) {
              templexParser.parseTemplex(sizerIter) match {
                case Err(e) => return Err(e)
                case Ok(e) => Some(e)
              }
            } else {
              None
            }
          StaticSizedP(sizeTemplex)
        } else {
          RuntimeSizedP
        }

      val tyype =
        iter.peek() match {
          case Some(ParendLE(range, contents)) => None
          case _ => {
            templexParser.parseTemplex(iter) match {
              case Err(e) => return Err(e)
              case Ok(e) => Some(e)
            }
          }
          case _ => return Err(BadArraySpecifier(iter.getPos()))
        }

      val args =
        parsePack(iter) match {
          case Ok(None) => return Err(BadArraySpecifier(iter.getPos()))
          case Ok(Some((range, e))) => e
          case Err(e) => return Err(e)
        }

      val initializingByValues =
        (size, args.size) match {
          case (StaticSizedP(None), _) => true // e.g. [#](3, 4, 5)
          case (StaticSizedP(Some(_)), _) => false // Anything else should take a function.
          case (RuntimeSizedP, _) => false // Any runtime sized array should take a function.
        }

      val arrayPE =
        ConstructArrayPE(
          RangeL(begin, iter.getPrevEndPos()),
          tyype,
          Some(mutability),
          None,
          size,
          initializingByValues,
          args)
      Ok(Some(arrayPE))
    }
  */

  /// Descramble - converts scrambled expression elements to properly structured AST
  /// Mirrors descramble in ExpressionParser.scala lines 1823-1880
  fn descramble_elements(
    &self,
    elements: &[ExpressionElement<'p>],
    begin_index_inclusive: usize,
    end_index_inclusive: usize,
    min_precedence: i32,
  ) -> ParseResult<(&'p IExpressionPE<'p>, usize)> {
    assert!(!elements.is_empty());
    assert!(elements.len() % 2 == 1);

    const MAX_PRECEDENCE: i32 = 6;

    // Base cases (lines 1832-1839)
    if begin_index_inclusive == end_index_inclusive {
      if let ExpressionElement::Data(expr) = &elements[begin_index_inclusive] {
        return Ok((*expr, begin_index_inclusive + 1));
      } else {
        panic!("Expected DataElement");
      }
    }
    if min_precedence == MAX_PRECEDENCE {
      if let ExpressionElement::Data(expr) = &elements[begin_index_inclusive] {
        return Ok((*expr, begin_index_inclusive + 1));
      } else {
        panic!("Expected DataElement");
      }
    }

    // Recursive descent (lines 1841-1842)
    let (mut left_operand, mut next_index) = self.descramble_elements(
      elements,
      begin_index_inclusive,
      end_index_inclusive,
      min_precedence + 1,
    )?;

    // Process operators at this precedence level (lines 1844-1876)
    while next_index < end_index_inclusive {
      if let ExpressionElement::BinaryCall(_, precedence) = &elements[next_index] {
        if *precedence != min_precedence {
          break;
        }
      } else {
        break;
      }

      let binary_call = if let ExpressionElement::BinaryCall(symbol, _) = &elements[next_index] {
        symbol.clone()
      } else {
        panic!("Expected BinaryCallElement");
      };
      next_index += 1;

      let (right_operand, new_next_index) = self.descramble_elements(
        elements,
        next_index,
        end_index_inclusive,
        min_precedence + 1,
      )?;
      next_index = new_next_index;

      // Construct the appropriate expression (lines 1854-1875)
      left_operand = if binary_call.str() == self.keywords.and {
        self.parse_arena.alloc(IExpressionPE::And(AndPE {
          range: RangeL(left_operand.range().begin(), right_operand.range().end()),
          left: left_operand,
          right: self.parse_arena.alloc(BlockPE {
            range: right_operand.range(),
            maybe_pure: None,
            maybe_default_region: None,
            inner: right_operand,
          }),
        }))
      } else if binary_call.str() == self.keywords.or {
        self.parse_arena.alloc(IExpressionPE::Or(OrPE {
          range: RangeL(left_operand.range().begin(), right_operand.range().end()),
          left: left_operand,
          right: self.parse_arena.alloc(BlockPE {
            range: right_operand.range(),
            maybe_pure: None,
            maybe_default_region: None,
            inner: right_operand,
          }),
        }))
      } else {
        self.parse_arena.alloc(IExpressionPE::BinaryCall(BinaryCallPE {
          range: RangeL(left_operand.range().begin(), right_operand.range().end()),
          function_name: binary_call,
          left_expr: left_operand,
          right_expr: right_operand,
        }))
      };
    }

    Ok((left_operand, next_index))
  }
  /*
    // Returns the index we stopped at, which will be either
    // the end of the array or one past endIndexInclusive.
    def descramble(
      elements: Vector[IExpressionElement],
      beginIndexInclusive: Int,
      endIndexInclusive: Int,
      minPrecedence: Int):
    (IExpressionPE, Int) = {
      vassert(elements.nonEmpty)
      vassert(elements.size % 2 == 1)

      if (beginIndexInclusive == endIndexInclusive) {
        val onlyElement = elements(beginIndexInclusive).asInstanceOf[DataElement].expr
        return (onlyElement, beginIndexInclusive + 1)
      }
      if (minPrecedence == MAX_PRECEDENCE) {
        val onlyElement = elements(beginIndexInclusive).asInstanceOf[DataElement].expr
        return (onlyElement, beginIndexInclusive + 1)
      }

      var (leftOperand, nextIndex) =
        descramble(elements, beginIndexInclusive, endIndexInclusive, minPrecedence + 1)

      while (nextIndex < endIndexInclusive &&
        elements(nextIndex).asInstanceOf[BinaryCallElement].precedence == minPrecedence) {

        val binaryCall = elements(nextIndex).asInstanceOf[BinaryCallElement]
        nextIndex += 1

        val (rightOperand, newNextIndex) =
          descramble(elements, nextIndex, endIndexInclusive, minPrecedence + 1)
        nextIndex = newNextIndex

        leftOperand =
          binaryCall.symbol.str match {
            case s if s == keywords.and => {
              AndPE(
                RangeL(leftOperand.range.begin, leftOperand.range.end),
                leftOperand,
                BlockPE(rightOperand.range, None, None, rightOperand))
            }
            case s if s == keywords.or => {
              OrPE(
                RangeL(leftOperand.range.begin, leftOperand.range.end),
                leftOperand,
                BlockPE(rightOperand.range, None, None, rightOperand))
            }
            case _ => {
              BinaryCallPE(
                RangeL(leftOperand.range.begin, leftOperand.range.end),
                binaryCall.symbol,
                leftOperand,
                rightOperand)
            }
          }
      }

      (leftOperand, nextIndex)
    }
  */

  /// Parse a binary call
  /// Mirrors parseBinaryCall in ExpressionParser.scala lines 1881-1923
  pub fn parse_binary_call(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<Option<NameP<'p>>> {
    let name = match iter.peek3_cloned() {
      (Some(INodeLEEnum::Word(WordLE { range, str })), _, _) => {
        iter.advance();
        NameP(range, str)
      }
      (
        Some(INodeLEEnum::Symbol(SymbolLE(range, s @ ('+' | '-' | '*' | '/')))),
        _,
        _,
      ) => {
        iter.advance();
        let str_i = match s {
          '+' => self.keywords.plus,
          '-' => self.keywords.minus,
          '*' => self.keywords.asterisk,
          '/' => self.keywords.slash,
          _ => unreachable!(),
        };
        NameP(range, str_i)
      }
      (
        Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
        Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
        Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
      ) => {
        let begin = iter.get_pos();
        iter.advance();
        iter.advance();
        iter.advance();
        let end = iter.get_prev_end_pos();
        NameP(RangeL(begin, end), self.keywords.triple_equals)
      }
      (
        Some(INodeLEEnum::Symbol(SymbolLE(_, '<'))),
        Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
        Some(INodeLEEnum::Symbol(SymbolLE(_, '>'))),
      ) => {
        let begin = iter.get_pos();
        iter.advance();
        iter.advance();
        iter.advance();
        let end = iter.get_prev_end_pos();
        NameP(RangeL(begin, end), self.keywords.spaceship)
      }
      (
        Some(INodeLEEnum::Symbol(SymbolLE(
          range1,
          s1 @ ('>' | '<' | '=' | '!'),
        ))),
        Some(INodeLEEnum::Symbol(SymbolLE(range2, '='))),
        _,
      ) => {
        let begin = range1.begin();
        let end = range2.end();
        iter.advance();
        iter.advance();
        let str_i = match s1 {
          '!' => self.keywords.not_equals,
          '=' => self.keywords.double_equals,
          '<' => self.keywords.less_equals,
          '>' => self.keywords.greater_equals,
          _ => unreachable!(),
        };
        NameP(RangeL(begin, end), str_i)
      }
      (
        Some(INodeLEEnum::Symbol(SymbolLE(range, s @ ('>' | '<')))),
        _,
        _,
      ) => {
        iter.advance();
        let str_i = match s {
          '>' => self.keywords.greater,
          '<' => self.keywords.less,
          _ => unreachable!(),
        };
        NameP(range, str_i)
      }
      _ => return Ok(None),
    };

    Ok(Some(name))
  }
  /*
    def parseBinaryCall(iter: ScrambleIterator):
    Result[Option[NameP], IParseError] = {
      val name =
        iter.peek3() match {
          case (Some(WordLE(range, str)), _, _) => {
            iter.advance()
            NameP(range, str)
          }
          case (Some(SymbolLE(range, s @ ('+' | '-' | '*' | '/'))), _, _) => {
            iter.advance()
            NameP(range, interner.intern(StrI(s.toString)))
          }
          case (Some(SymbolLE(_, '=')), Some(SymbolLE(_, '=')), Some(SymbolLE(_, '='))) => {
            val begin = iter.getPos()
            iter.advance()
            iter.advance()
            iter.advance()
            val end = iter.getPrevEndPos()
            NameP(RangeL(begin, end), keywords.tripleEquals)
          }
          case (Some(SymbolLE(_, '<')), Some(SymbolLE(_, '=')), Some(SymbolLE(_, '>'))) => {
            val begin = iter.getPos()
            iter.advance()
            iter.advance()
            iter.advance()
            val end = iter.getPrevEndPos()
            NameP(RangeL(begin, end), keywords.spaceship)
          }
          case (Some(SymbolLE(range1, s1 @ ('>' | '<' | '=' | '!'))), Some(SymbolLE(range2, '=')), _) => {
            iter.advance()
            iter.advance()
            NameP(RangeL(range1.begin, range2.end), interner.intern(StrI(s1.toString + '=')))
          }
          case (Some(SymbolLE(range, s @ ('>' | '<'))), _, _) => {
            iter.advance()
            NameP(range, interner.intern(StrI(s.toString)))
          }
          case _ => return Ok(None)
        }

      Ok(Some(name))
    }
  */

  /// Check if at expression end
  /// Mirrors atExpressionEnd in ExpressionParser.scala lines 1924-1933
  pub fn at_expression_end(&self, iter: &ScrambleIterator, stop_on_curlied: bool) -> bool {
    match iter.peek_cloned() {
      None => true,
      Some(INodeLEEnum::Symbol(SymbolLE(_, ';'))) => true,
      Some(INodeLEEnum::Curlied(_)) if stop_on_curlied => true,
      _ => false,
    }
  }
  /*
    def atExpressionEnd(iter: ScrambleIterator, stopOnCurlied: Boolean): Boolean = {
      iter.peek() match {
        case None => true
        case Some(SymbolLE(range, ';')) => true
        case Some(CurliedLE(range, contents)) if stopOnCurlied => true
        case _ => false
      }
  //    return Parser.atEnd(iter) || iter.peek(() => "^\\s*;")
    }
  */
}

/*
}
*/
