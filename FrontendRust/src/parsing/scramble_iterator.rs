use crate::lexing::ast::*;
use crate::StrI;

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
  assert(end <= scramble.elements.length)
Guardian: disable: NECX
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
  ///
  /// `include_empty_trailing`: If true and the scramble ends with the needle,
  /// include an empty iterator at the end
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
      // Scramble didn't end in the needle, add the last section
      iters.push(ScrambleIterator::with_bounds(self.scramble, start, self.end));
    } else if start == self.end && include_empty_trailing {
      // Ended in a needle and we want to include the empty section
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_iteration() {
    let scramble = ScrambleLE {
      range: RangeL(0, 10),
      elements: vec![
        Box::new(INodeLEEnum::Symbol(SymbolLE(RangeL(0, 1), '('))),
        Box::new(INodeLEEnum::Symbol(SymbolLE(RangeL(1, 2), ')'))),
      ],
    };

    let mut iter = ScrambleIterator::new(&scramble);
    assert!(!iter.at_end());
    assert!(iter.has_next());
    assert_eq!(iter.remaining(), 2);

    iter.advance();
    assert_eq!(iter.remaining(), 1);

    iter.advance();
    assert!(iter.at_end());
    assert_eq!(iter.remaining(), 0);
  }

  #[test]
  fn test_split_on_symbol() {
    let scramble = ScrambleLE {
      range: RangeL(0, 10),
      elements: vec![
        Box::new(INodeLEEnum::Symbol(SymbolLE(RangeL(0, 1), 'a'))),
        Box::new(INodeLEEnum::Symbol(SymbolLE(RangeL(1, 2), ','))),
        Box::new(INodeLEEnum::Symbol(SymbolLE(RangeL(2, 3), 'b'))),
      ],
    };

    let iter = ScrambleIterator::new(&scramble);
    let parts = iter.split_on_symbol(',', false);
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].remaining(), 1);
    assert_eq!(parts[1].remaining(), 1);
  }
}

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
object ExpressionParser {
  val MAX_PRECEDENCE = 6
  val MIN_PRECEDENCE = 1
}
*/
/*
  override def clone(): ScrambleIterator = new ScrambleIterator(scramble, index, end)
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
/*
//  def exists(func: scala.Function1[INodeLE, Boolean]): Boolean = {
//    U.exists(scramble.elements, func, index, end)
//  }
*/
/*
}
*/
