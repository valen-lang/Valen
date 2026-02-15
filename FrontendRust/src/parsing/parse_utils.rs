use crate::lexing::ast::INodeLE;
use crate::lexing::ast::INodeLEEnum;
use crate::lexing::ast::SymbolLE;
use crate::lexing::WordLE;
use crate::parsing::ScrambleIterator;
/*
package dev.vale.parsing

import dev.vale.StrI
import dev.vale.lexing.{SymbolLE, WordLE}

object ParseUtils {
*/

/// Helper method to skip past an equals sign while a condition is true
/// Mirrors ParseUtils.trySkipPastEqualsWhile in ParseUtils.scala
pub fn try_skip_past_equals_while<'a, F>(
  iter: &mut ScrambleIterator<'a>,
  continue_while: F,
) -> Option<ScrambleIterator<'a>>
where
  F: Fn(&ScrambleIterator<'a>) -> bool,
{
  let mut scouting_iter = iter.clone();
  while continue_while(&scouting_iter) {
    match scouting_iter.peek3_cloned() {
      (Some(prev), Some(INodeLEEnum::Symbol(SymbolLE { range, c: '=' })), Some(next)) => {
        let surrounded_by_spaces = prev.range().end < range.begin && range.end < next.range().begin;
        if surrounded_by_spaces {
          // We'll return this iterator for the things that come before the =
          let mut before_iter = iter.clone();
          before_iter.end = scouting_iter.index + 1;

          // Now modify iter to skip past it
          iter.skip_to(&scouting_iter);
          iter.advance();
          iter.advance();

          return Some(before_iter);
        }
      }
      _ => {}
    }
    scouting_iter.advance();
  }

  None
}

/*
  // This method modifies the current iterator to skip it past the next = symbol
  // that's surrounded by spaces. Note that it won't catch an = at the beginning or
  // end of the statement.
  // It returns None if there wasn't one (which leaves self untouched) or a Some
  // containing everything we skipped past (minus the =).
  def trySkipPastEqualsWhile(iter: ScrambleIterator, continueWhile: ScrambleIterator => Boolean): Option[ScrambleIterator] = {
    val scoutingIter = iter.clone()
    while (continueWhile(scoutingIter)) {
      scoutingIter.peek3() match {
        case (Some(prev), Some(SymbolLE(range, '=')), Some(next)) => {
          val surroundedBySpaces =
            prev.range.end < range.begin && range.end < next.range.begin
          if (surroundedBySpaces) {
            // We'll return this iterator for the things that come before the =
            val beforeIter = iter.clone()
            beforeIter.end = scoutingIter.index + 1

            // Now modify self to skip past it.
            iter.skipTo(scoutingIter)
            iter.advance()
            iter.advance()

            return Some(beforeIter)
          }
        }
        case _ =>
      }
      scoutingIter.advance()
    }

    return None
  }
*/

/// Try to skip past a keyword, returning the portion before it
/// Mirrors trySkipPastKeywordWhile in ParseUtils.scala lines 77-102
pub fn try_skip_past_keyword_while<'a, F>(
  iter: &mut ScrambleIterator<'a>,
  keyword: &'a crate::interner::StrI,
  continue_while: F,
) -> Option<(WordLE<'a>, ScrambleIterator<'a>)>
where
  F: Fn(&ScrambleIterator<'a>) -> bool,
{
  // Mirrors ParseUtils.scala line 82
  let mut scouting_iter = iter.clone();

  // Mirrors ParseUtils.scala line 83
  while continue_while(&scouting_iter) {
    // Mirrors ParseUtils.scala lines 84-98
    match scouting_iter.peek_cloned() {
      Some(INodeLEEnum::Word(w)) if w.str == keyword => {
        // Mirrors ParseUtils.scala lines 86-88
        // We'll return this iterator for the things that come before the keyword
        let mut before_iter = iter.clone();
        before_iter.end = scouting_iter.index;

        // Mirrors ParseUtils.scala lines 90-92
        // Now modify self to skip past it.
        iter.skip_to(&scouting_iter);
        iter.advance();

        // Mirrors ParseUtils.scala line 94
        return Some((w.clone(), before_iter));
      }
      _ => {}
    }
    // Mirrors ParseUtils.scala line 98
    scouting_iter.advance();
  }

  // Mirrors ParseUtils.scala line 101
  None
}
/*
  // This method modifies the current iterator to skip it past the next = symbol
  // that's surrounded by spaces. Note that it won't catch an = at the beginning or
  // end of the statement.
  // It returns None if there wasn't one (which leaves self untouched) or a Some
  // containing everything we skipped past (minus the =).
  //
  // TODO: this is pretty confusing if youre not really deep into the "trySkipPast" mindset already.
  // Let's replace it with a method that just splits an iterator into two.
  def trySkipPastKeywordWhile(
      iter: ScrambleIterator,
      keyword: StrI,
      continueWhile: ScrambleIterator => Boolean):
  Option[(WordLE, ScrambleIterator)] = {
    val scoutingIter = iter.clone()
    while (continueWhile(scoutingIter)) {
      scoutingIter.peek() match {
        case Some(w @ WordLE(_, kw)) if kw == keyword => {
          // We'll return this iterator for the things that come before the =
          val beforeIter = iter.clone()
          beforeIter.end = scoutingIter.index

          // Now modify self to skip past it.
          iter.skipTo(scoutingIter)
          iter.advance()

          return Some((w, beforeIter))
        }
        case _ =>
      }
      scoutingIter.advance()
    }

    return None
  }
*/

/*
  // This method modifies the current iterator to skip it past the next = symbol
  // that's surrounded by spaces. Note that it won't catch an = at the beginning or
  // end of the statement.
  // It returns None if there wasn't one (which leaves self untouched) or a Some
  // containing everything we skipped past (minus the =).
  def trySkipPastSemicolonWhile(iter: ScrambleIterator, continueWhile: ScrambleIterator => Boolean): Option[ScrambleIterator] = {
    val scoutingIter = iter.clone()
    while (continueWhile(scoutingIter)) {
      scoutingIter.peek() match {
        case Some(SymbolLE(_, ';')) => {
          // We'll return this iterator for the things that come before the =
          val beforeIter = iter.clone()
          beforeIter.end = scoutingIter.index + 1

          // Now modify self to skip past it.
          iter.skipTo(scoutingIter)
          iter.advance()

          return Some(beforeIter)
        }
        case _ =>
      }
      scoutingIter.advance()
    }

    return None
  }
*/

/*
  def trySkipTo(
    iter: ScrambleIterator,
    stopAt: ScrambleIterator => Boolean):
  Option[ScrambleIterator] = {
    val scoutingIter = iter.clone()
    while ({
      if (stopAt(scoutingIter)) {
        // We'll return this iterator
        val beforeIter = iter.clone()
        beforeIter.end = scoutingIter.index

        // Now modify self to skip past it.
        iter.skipTo(scoutingIter)

        return Some(beforeIter)
      } else {
        // continue
        scoutingIter.advance()
        true
      }
    }) { }
    // We never hit the condition, so stop.

    return None
  }
*/
/*
}
*/
