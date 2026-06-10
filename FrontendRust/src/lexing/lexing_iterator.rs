use super::ast::RangeL;
use std::cmp::min;
/*
package dev.vale.lexing

import dev.vale.{Accumulator, Ok, Profiler, vassert, vcurious, vfail, vwat}

import scala.util.matching.Regex
*/

  /*
  case class LexingIterator(code: String, var position: Int = 0) {
    override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious();
  */

/// Lexing iterator for traversing source code
/// Matches Scala's LexingIterator
#[derive(Clone, Debug)]
pub struct LexingIterator<'a> {
  pub code: &'a str,
  pub position: usize, // Byte position in the string
  pub comments: Vec<RangeL>,
  /*
    val comments = new Accumulator[RangeL]()
  */
}
impl<'a> LexingIterator<'a> {
  /// Get the rest of the code from current position (for debugging)
  pub fn rest(&self) -> &str {
    &self.code[self.position..]
  }
  /*
    // For debugging
    def rest(): String = {
      code.slice(position, code.length)
    }
  */
  /// Consume comments and whitespace
  pub fn consume_comments_and_whitespace(&mut self) {
    // consumeComments will consume any whitespace that comes before the comment
    self.consume_comments();
    self.consume_whitespace();
  }
  /*
    def consumeCommentsAndWhitespace(): Unit = {
      // consumeComments will consume any whitespace that come before the comment
      consumeComments()
      consumeWhitespace()
    }
  */

  /// Find end of whitespace without consuming
  fn find_whitespace_end(&self) -> usize {
    let mut pos = self.position;
    while pos < self.code.len() {
      match self.code[pos..].chars().next().unwrap() {
        ' ' | '\n' | '\r' | '\t' => {
          pos += 1;
        }
        _ => break,
      }
    }
    pos
  }
  /*
    def findWhitespaceEnd(): Int = {
      var tentativePosition = position
      // Skip whitespace
      while ({
        if (tentativePosition == code.length) {
          return tentativePosition
        }
        code.charAt(tentativePosition) match {
          case ' ' | '\n' | '\r' | '\t' => {
            tentativePosition = tentativePosition + 1
            true
          }
          case _ => false
        }
      }) {}
      tentativePosition
    }
  */

  /// Consume all types of comments
  fn consume_comments(&mut self) {
    self.consume_line_comments();
    self.consume_chevron_comments();
    self.consume_ellipses_comments();
  }
  /*
    def consumeComments(): Unit = {
      consumeLineComments()
      consumeChevronComments()
      consumeEllipsesComments()
    }
  */
/*
  def getUntil(needle: Char): Option[String] = {
    val begin = position
    if (code.charAt(position) == needle) {
      return Some(code.slice(begin, position))
    }
    while (true) {
      if (position == code.length) {
        return None
      } else {
        position = position + 1
        vassert(position <= code.length)
        if (code.charAt(position) == needle) {
          return Some(code.slice(begin, position))
        }
      }
    }
    vwat()
  }
*/

  /// Skip to past a specific character
  fn skip_to_past(&mut self, needle: char) -> bool {
    while !self.at_end() {
      let c = self.advance();
      if c == needle {
        return true;
      }
    }
    false
  }
  /*
    def skipToPast(needle: Char): Boolean = {
      while ({
        if (position == code.length) {
          return false
        } else {
          val isNeedle = code.charAt(position) == needle
          position = position + 1
          vassert(position <= code.length)
          !isNeedle
        }
      }) {}
      true
    }
  */
  /// Consume chevron comments (« »)
  fn consume_chevron_comments(&mut self) {
    let pos_after_whitespace = self.find_whitespace_end();

    if pos_after_whitespace < self.code.len() && self.code[pos_after_whitespace..].starts_with('«')
    {
      let begin = self.position;
      self.position = pos_after_whitespace + '«'.len_utf8();

      self.skip_to_past('»');

      self.comments.push(RangeL(begin as i32, self.position as i32));

      self.consume_comments();
    }
  }
  /*
    def consumeChevronComments(): Unit = {
      val tentativePositionAfterWhitespace = findWhitespaceEnd()
      if (tentativePositionAfterWhitespace < code.length &&
        code.charAt(tentativePositionAfterWhitespace) == '«') {
        val begin = position
        position = tentativePositionAfterWhitespace + 1
        vassert(position <= code.length)
        skipToPast('»')
        comments.add(RangeL(begin, position))
        consumeComments()
      }
    }
  */

  /// Consume ellipses comments (...)
  fn consume_ellipses_comments(&mut self) {
    let pos_after_whitespace = self.find_whitespace_end();

    if pos_after_whitespace + 2 < self.code.len()
      && self.code.as_bytes()[pos_after_whitespace] == b'.'
      && self.code.as_bytes()[pos_after_whitespace + 1] == b'.'
      && self.code.as_bytes()[pos_after_whitespace + 2] == b'.'
    {
      let begin = self.position;
      self.position = pos_after_whitespace + 3;
      assert!(self.position <= self.code.len());
      self.comments.push(RangeL(begin as i32, self.position as i32));
      self.consume_comments();
    }

    self.try_skip_str("...");
  }
  /*
    def consumeEllipsesComments(): Unit = {
      val tentativePositionAfterWhitespace = findWhitespaceEnd()
      if (tentativePositionAfterWhitespace < code.length &&
        code.charAt(tentativePositionAfterWhitespace) == '.' &&
        code.charAt(tentativePositionAfterWhitespace + 1) == '.' &&
        code.charAt(tentativePositionAfterWhitespace + 2) == '.') {
        val begin = position
        position = tentativePositionAfterWhitespace + 3
        vassert(position <= code.length)
        comments.add(RangeL(begin, position))
        consumeComments()
      }

      trySkip("...")
    }
  */

  /// Consume line comments (//)
  fn consume_line_comments(&mut self) {
    let pos_after_whitespace = self.find_whitespace_end();

    if pos_after_whitespace + 2 <= self.code.len()
      && self.code.as_bytes()[pos_after_whitespace] == b'/'
      && self.code.as_bytes()[pos_after_whitespace + 1] == b'/'
    {
      let begin = pos_after_whitespace;
      self.position = pos_after_whitespace + 2;
      assert!(self.position <= self.code.len());
      self.skip_to_past('\n');
      self.comments.push(RangeL(begin as i32, (self.position - 1) as i32));
      self.consume_comments();
    }
  }
  /*
    def consumeLineComments(): Unit = {
      val tentativePositionAfterWhitespace = findWhitespaceEnd()
      if (tentativePositionAfterWhitespace + 2 <= code.length &&
          code.charAt(tentativePositionAfterWhitespace) == '/' &&
          code.charAt(tentativePositionAfterWhitespace + 1) == '/') {
        val begin = tentativePositionAfterWhitespace
        position = tentativePositionAfterWhitespace + 2
        vassert(position <= code.length)
        skipToPast('\n')
        comments.add(RangeL(begin, position - 1))
        consumeComments()
      }
    }
  */

  /// Peek ahead to get a substring of exact length
  pub fn peek_exact(&self, n: usize) -> Option<&str> {
    if self.position + n > self.code.len() {
      None
    } else {
      Some(&self.code[self.position..self.position + n])
    }
  }

  /// Advance by one character and return it
  pub fn advance(&mut self) -> char {
    if self.at_end() {
      '\0'
    } else {
      let c = self.peek();
      self.position += c.len_utf8();
      c
    }
  }
  /*
    def advance(): Char = {
      val c = peek()
      position = position + 1
      vassert(position <= code.length)
      c
    }
  */

  /// Try to skip a specific character
  pub fn try_skip(&mut self, c: char) -> bool {
    if self.peek() == c {
      self.advance();
      true
    } else {
      false
    }
  }
  /*
    def trySkip(c: Char): Boolean = {
      if (position + 1 <= code.length) {
        // good, continue
      } else {
        return false
      }
      val wasAsExpected = (peek() == c)
      position = position + (if (wasAsExpected) 1 else 0)
      vassert(position <= code.length)
      wasAsExpected
    }
  */

  // Optimize: could replace with xor and bitwise and for small strings
  /// Try to skip a specific string
  pub fn try_skip_str(&mut self, s: &str) -> bool {
    if self.code[self.position..].starts_with(s) {
      self.position += s.len();
      true
    } else {
      false
    }
  }
  /*
    // Optimize: replace with xor and bitwise and for small strings
    def trySkip(s: String): Boolean = {
      val wasAsExpected = peekString(s)
      position = position + (if (wasAsExpected) 1 else 0) * s.length
      vassert(position <= code.length)
      wasAsExpected
    }
  */

  // Optimize: could replace with xor and bitwise and for small strings
  // A complete word is one that doesn't have any more word characters after it
  /// Try to skip a complete word (must be followed by non-identifier char)
  pub fn try_skip_complete_word(&mut self, word: &str) -> bool {
    if !self.code[self.position..].starts_with(word) {
      return false;
    }

    // Check that the next character is not an identifier character
    let after_pos = self.position + word.len();
    if after_pos < self.code.len() {
      let next_char = self.code[after_pos..].chars().next().unwrap();
      if next_char.is_alphanumeric() || next_char == '_' {
        return false;
      }
    }

    self.position += word.len();
    true
  }
  /*
    // Optimize: replace with xor and bitwise and for small strings
    // A complete word is one that doesnt have any more word characters after it
    def trySkipCompleteWord(s: String): Boolean = {
      if (position + s.length <= code.length) {
        // good, continue
      } else {
        return false
      }
      var i = 0
      var wasAsExpected = true
      while (i < s.length) {
        wasAsExpected = wasAsExpected && code.charAt(position + i) == s.charAt(i)
        i = i + 1
      }
      // Now check if we're ending the word, by peeking at the next thing
      wasAsExpected = wasAsExpected && (
        position + i == code.length ||
        !(
          code.charAt(position + 1) == '_' ||
          (code.charAt(position + i) >= 'a' && code.charAt(position + i) >= 'z') ||
          (code.charAt(position + i) >= 'A' && code.charAt(position + i) >= 'Z')))

      position = position + (if (wasAsExpected) 1 else 0) * s.length
      vassert(position <= code.length)
      wasAsExpected
    }
  */

/*
  override def clone(): LexingIterator = LexingIterator(code, position)
*/

  pub fn new(code: &'a str) -> Self {
    LexingIterator {
      code,
      position: 0,
      comments: Vec::new(),
    }
  }

  pub fn at_end(&self) -> bool {
    self.position >= self.code.len()
  }
  /*
    def atEnd(): Boolean = { position >= code.length }
  */

  /// Skip to a specific position
  pub fn skip_to(&mut self, pos: usize) {
    self.position = pos;
  }
  /*
    def skipTo(newPosition: Int) = {
      vassert(newPosition >= position)
      position = newPosition
      vassert(position <= code.length)
    }
  */

  pub fn get_pos(&self) -> i32 {
    self.position as i32
  }
  /*
    def getPos(): Int = {
      position
    }
  */

  /// Consume whitespace
  pub fn consume_whitespace(&mut self) {
    while !self.at_end() {
      match self.peek() {
        ' ' | '\n' | '\r' | '\t' => {
          self.advance();
        }
        _ => break,
      }
    }
  }
  /*
    def consumeWhitespace(): Boolean = {
      var foundAny = false
      while (!atEnd()) {
        peek() match {
          case ' ' | '\t' | '\n' | '\r' => foundAny = true
          case _ => return foundAny
        }
        advance()
      }
      return false
    }
  */


/*
//  private def at(regexF: () => Regex): Boolean = {
//    runRegexFrame(regexF, code).nonEmpty
//  }
//
//  def trySkip(regexF: () => Regex): Boolean = {
//    runRegexFrame(regexF, code) match {
//      case None => false
//      case Some(matchedStr) => {
//        skipTo(position + matchedStr.length)
//        true
//      }
//    }
//  }
//
//  def tryy(regexF: () => Regex): Option[String] = {
//    runRegexFrame(regexF, code) match {
//      case None => None
//      case Some(matchedStr) => {
//        skipTo(position + matchedStr.length)
//        Some(matchedStr)
//      }
//    }
//  }
//
//  def runRegexFrame(regexF: () => Regex, code: String): Option[String] = {
//    Profiler.frame(() => {
//      val regex = regexF()
//      vassert(regex.pattern.pattern().startsWith("^"))
//      regex.findFirstIn(code.slice(position, code.length))
//    })
//  }

//  def peek(): Char = {
//    if (position >= code.length)
//      return '\0'
//    else
//      return code.charAt(position)
//  }
*/

  /// Peek if a string matches (without advancing)
  pub fn peek_string(&self, s: &str) -> bool {
    self.code[self.position..].starts_with(s)
  }
  /*
    def peekString(s: String): Boolean = {
      var tentativePosition = position
      if (tentativePosition + s.length <= code.length) {
        // good, continue
      } else {
        return false
      }
      var i = 0
      var wasAsExpected = true
      while (i < s.length) {
        wasAsExpected = wasAsExpected && code.charAt(tentativePosition + i) == s.charAt(i)
        i = i + 1
      }
      tentativePosition = tentativePosition + (if (wasAsExpected) 1 else 0) * s.length

      wasAsExpected
    }
  */

  /// Peek if a complete word matches (without advancing)
  pub fn peek_complete_word(&self, word: &str) -> bool {
    if !self.code[self.position..].starts_with(word) {
      return false;
    }

    let after_pos = self.position + word.len();
    if after_pos < self.code.len() {
      let next_char = self.code[after_pos..].chars().next().unwrap();
      if next_char.is_alphanumeric() || next_char == '_' {
        return false;
      }
    }

    true
  }
  /*
    def peekCompleteWord(s: String): Boolean = {
      var wasAsExpected = peekString(s)
      val posAfterWord = position + s.length

      // Now check if we're ending the word, by peeking at the next thing
      wasAsExpected =
        wasAsExpected &&
          (posAfterWord == code.length || !code.charAt(posAfterWord).isUnicodeIdentifierPart)

      wasAsExpected
    }
  */

  /// Peek at the current character without advancing
  pub fn peek(&self) -> char {
    if self.at_end() {
      '\0'
    } else {
      self.code[self.position..].chars().next().unwrap()
    }
  }
  /*
    def peek(): Char = {
      if (position >= code.length) '\0'
      else code.charAt(position)
    }
  */

  /// Peek ahead n characters (returns String)
  pub fn peek_n(&self, n: usize) -> Option<String> {
    if self.position + n > self.code.len() {
      None
    } else {
      Some(self.code[self.position..min(self.position + n, self.code.len())].to_string())
    }
  }
  /*
    def peek(n: Int): Option[String] = {
      val s = code.slice(position, position + n)
      if (s.length < n) { None } else { Some(s) }
    }
  */

/*
//  def trySkipIfPeekNext(
//    toConsumeF: () => Regex,
//    ifNextPeekF: () => Regex):
//  Boolean = {
//    val tentativeIter = this.clone()
//    if (!tentativeIter.trySkip(toConsumeF)) {
//      return false
//    }
//    val pos = tentativeIter.getPos()
//    if (!tentativeIter.peek(ifNextPeekF)) {
//      return false
//    }
//    this.skipTo(pos)
//    return true
//  }
}
*/
}
