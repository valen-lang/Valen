use crate::keywords::Keywords;
use crate::lexing::ast::*;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::scramble_iterator::ScrambleIterator;
use crate::parsing::templex_parser::TemplexParser;
use crate::utils::arena_utils::alloc_slice_from_vec;
use bumpalo::Bump;

/*
package dev.vale.parsing

import dev.vale.{Err, Interner, Keywords, Ok, Result, StrI, U, vassert, vassertSome, vimpl, vwat}
import dev.vale.parsing.ast.{AbstractP, ConstructingMemberNameDeclarationP, DestructureP, INameDeclarationP, IgnoredLocalNameDeclarationP, LocalNameDeclarationP, NameP, PatternPP}
import dev.vale.parsing.templex.TemplexParser
import dev.vale.lexing._
import dev.vale.parsing.ast._

import scala.collection.mutable
*/
type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone)]
pub struct PatternParser<'p, 'ctx> {
  #[allow(dead_code)]
  parse_arena: &'ctx crate::parse_arena::ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  arena: &'p Bump,
}
/*
class PatternParser(interner: Interner, keywords: Keywords, templexParser: TemplexParser) {
Guardian: disable: NECX
*/

impl<'p, 'ctx> PatternParser<'p, 'ctx>
where
  'p: 'ctx,
{
  pub fn new(parse_arena: &'ctx crate::parse_arena::ParseArena<'p>, keywords: &'ctx Keywords<'p>, arena: &'p Bump) -> Self {
    PatternParser {
      parse_arena,
      keywords,
      arena,
    }
  }

  /// Parse a parameter
  /// Mirrors parseParameter in PatternParser.scala lines 13-72
  pub fn parse_parameter(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    index: usize,
    is_in_citizen: bool,
    is_in_function: bool,
    is_in_lambda: bool,
  ) -> ParseResult<ParameterP<'p>> {
    let pattern_begin = iter.get_pos();
    let pattern_range = iter.range();

    if !iter.has_next() {
      return Err(ParseError::EmptyPattern(pattern_begin));
    }

    // Check for 'virtual' keyword (lines 21-29)
    let maybe_virtual = match iter.peek_cloned() {
      None => return Err(ParseError::EmptyParameter(pattern_range.begin())),
      Some(INodeLEEnum::Word(WordLE { range, str })) if str == self.keywords.r#virtual => {
        iter.advance();
        Some(AbstractP { range })
      }
      Some(_) => None,
    };

    // Check for '&self' (lines 31-41)
    let maybe_self_borrow = match iter.peek_n(2).as_slice() {
      [] => return Err(ParseError::EmptyParameter(pattern_range.begin())),
      [None] => return Err(ParseError::EmptyParameter(pattern_range.begin())),
      [None, None] => return Err(ParseError::EmptyParameter(pattern_range.begin())),
      [Some(INodeLEEnum::Symbol(SymbolLE(range1, '&'))), Some(INodeLEEnum::Word(WordLE { range: range2, str }))]
        if *str == self.keywords.self_ =>
      {
        let begin = range1.begin();
        let end = range2.end();
        iter.advance();
        iter.advance();
        Some(RangeL(begin, end))
      }
      _ => None,
    };

    // If we have self borrow, return early (lines 42-45)
    if let Some(_) = maybe_self_borrow {
      return Ok(ParameterP {
        range: pattern_range,
        virtuality: maybe_virtual,
        maybe_pre_checked: None,
        self_borrow: maybe_self_borrow,
        pattern: None,
      });
    }

    // Parse optional name (lines 47-62)
    let maybe_name = match iter.peek2_cloned() {
      (Some(INodeLEEnum::Squared(_)), _) => {
        // Destructure parameter with no name or type, like func moo([a, b, c])
        None
      }
      (Some(INodeLEEnum::Word(_)), Some(INodeLEEnum::Squared(_))) => {
        // Destructure parameter with type but no name, like func moo(Vec3[a, b, c])
        None
      }
      (Some(INodeLEEnum::Word(w)), _) => {
        let word = w.clone();
        iter.advance();
        Some(word)
      }
      _ => return Err(ParseError::BadLocalName(iter.get_pos())),
    };

    // Check for 'pre' keyword (line 64)
    let maybe_pre_checked = iter.try_skip_word(self.keywords.pre);

    // Parse the pattern (lines 66-69)
    let pattern = self.parse_pattern(
      iter,
      templex_parser,
      pattern_begin,
      index,
      is_in_citizen,
      is_in_function,
      is_in_lambda,
      maybe_name,
    )?;

    Ok(ParameterP {
      range: pattern_range,
      virtuality: maybe_virtual,
      maybe_pre_checked,
      self_borrow: maybe_self_borrow,
      pattern: Some(pattern),
    })
  }

  /*
    def parseParameter(iter: ScrambleIterator, index: Int, isInCitizen: Boolean, isInFunction: Boolean, isInLambda: Boolean): Result[ParameterP, IParseError] = {
      val patternBegin = iter.getPos()
      val patternRange = iter.range

      if (!iter.hasNext) {
        return Err(EmptyPattern(patternBegin))
      }

      val maybeVirtual =
        iter.peek_cloned() match {
          case None => return Err(EmptyParameter(patternRange.begin))
          case Some(WordLE(range, s)) if s == keywords.virtual => {
            iter.advance()
            Some(AbstractP(range))
          }
          case Some(_) => None
        }

      val maybeSelfBorrow =
        iter.peek(2) match {
          case Vector() => return Err(EmptyParameter(patternRange.begin))
          case Vector(None) => return Err(EmptyParameter(patternRange.begin))
          case Vector(None, None) => return Err(EmptyParameter(patternRange.begin))
          case Vector(Some(SymbolLE(RangeL(begin, _), '&')), Some(WordLE(RangeL(_, end), s))) if s == keywords.self => {
            iter.advance()
            Some(RangeL(begin, end))
          }
          case _ => None
        }
      maybeSelfBorrow match {
        case Some(_) => {
          Ok(ParameterP(patternRange, maybeVirtual, None, maybeSelfBorrow, None))
        }
        case None => {
          val maybeName =
            iter.peek2_cloned() match {
              case (Some(SquaredLE(_, _)), _) => {
                // This is a destructure parameter with no name or type, like func moo([a, b, c])
                None
              }
              case (Some(w@WordLE(_, _)), Some(SquaredLE(_, _))) => {
                // This is a destructure parameter with no name, like func moo(Vec3[a, b, c])
                None
              }
              case (Some(w @ WordLE(range, str)), _) => {
                iter.advance()
                Some(w)
              }
              case _ => return Err(BadLocalName(iter.getPos()))
            }

          val maybePreChecked = iter.trySkipWord(keywords.pre)

          parsePattern(iter, patternBegin, index, isInCitizen, isInFunction, isInLambda, maybeName) match {
            case Ok(pattern) => Ok(ParameterP(patternRange, maybeVirtual, maybePreChecked, maybeSelfBorrow, Some(pattern)))
            case Err(x) => Err(x)
          }
        }
      }
    }
  */

  /// Parse a pattern
  /// Mirrors parsePattern in PatternParser.scala lines 74-221
  pub fn parse_pattern(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
    templex_parser: &TemplexParser<'p, 'ctx>,
    pattern_begin: i32,
    index: usize,
    is_in_citizen: bool,
    is_in_function: bool,
    is_in_lambda: bool,
    maybe_name_from_parameter: Option<WordLE<'p>>,
  ) -> ParseResult<PatternPP<'p>> {
    // Mirrors PatternParser.scala lines 75-88
    // The Scala code used to have an early return here, but it was dead code and has been commented out.
    // We just check for empty pattern with no name.
    if !iter.has_next() {
      if maybe_name_from_parameter.is_none() {
        return Err(ParseError::EmptyPattern(pattern_begin));
      }
      // Fall through to validation logic below
    }

    // Check for 'self.' prefix for constructing members (lines 90-99)
    let is_constructing = match iter.peek2_cloned() {
      (
        Some(INodeLEEnum::Word(WordLE { str: self_str, .. })),
        Some(INodeLEEnum::Symbol(SymbolLE(_, '.'))),
      ) if self_str == self.keywords.self_ => {
        iter.advance();
        iter.advance();
        true
      }
      _ => false,
    };

    // Check for 'set' keyword (lines 101-104)
    let maybe_mutate = iter.try_skip_word(self.keywords.set);
    if maybe_mutate.is_some() && !iter.has_next() {
      return Err(ParseError::CantUseThatLocalName {
        pos: iter.get_pos(),
        name: "set".to_string(),
      });
    }

    // Parse destination local (lines 106-151)
    let maybe_destination_local = match maybe_name_from_parameter {
      Some(WordLE { range, str }) => {
        if str == self.keywords.underscore {
          Some(DestinationLocalP::<'p> {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(range),
            mutate: None,
          })
        } else {
          Some(DestinationLocalP::<'p> {
            decl: INameDeclarationP::LocalNameDeclaration(NameP(range, str)),
            mutate: None,
          })
        }
      }
      None => {
        // Determine if the next thing is a name (lines 116-129)
        let name_is_next = match iter.peek2_cloned() {
          (None, None) => {
            panic!("Impossible: peek2 should not return (None, None) when has_next is true")
          }
          (None, Some(_)) => panic!("Impossible: peek2 should not return (None, Some(_))"),
          (Some(_), None) => true,
          (Some(first), Some(second)) => {
            // There's a space after the first thing if ranges don't touch
            first.range().end() < second.range().begin()
          }
        };

        if name_is_next {
          match iter.peek_cloned() {
            Some(INodeLEEnum::Word(WordLE { range, str })) => {
              iter.advance();
              if str == self.keywords.underscore {
                Some(DestinationLocalP {
                  decl: INameDeclarationP::IgnoredLocalNameDeclaration(range),
                  mutate: maybe_mutate,
                })
              } else {
                if is_constructing {
                  Some(DestinationLocalP {
                    decl: INameDeclarationP::ConstructingMemberNameDeclaration(NameP(range, str)),
                    mutate: maybe_mutate,
                  })
                } else {
                  Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP(range, str)),
                    mutate: maybe_mutate,
                  })
                }
              }
            }
            Some(INodeLEEnum::Squared(_)) => None,
            _ => return Err(ParseError::BadLocalName(iter.get_pos())),
          }
        } else {
          None
        }
      }
    };

    // Stop if we see 'in' keyword (lines 153-158)
    match iter.peek_cloned() {
      Some(INodeLEEnum::Word(WordLE { str, .. })) if str == self.keywords.r#in => {
        // Don't consume it, just stop processing
      }
      _ => {}
    }

    // Determine if next thing is a type (lines 160-174)
    let next_is_type = match iter.peek2_cloned() {
      (None, None) => false,
      (None, Some(_)) => panic!("Impossible: peek2 should not return (None, Some(_))"),
      (Some(INodeLEEnum::Squared(_)), maybe_after) => {
        // If there's something after the squared brackets, it's an array type
        maybe_after.is_some()
      }
      (Some(_), _) => {
        // There's something that's not square-braced, so it's a type
        true
      }
    };

    // Parse optional type (lines 175-194)
    let maybe_type: Option<ITemplexPT<'p>> = if next_is_type {
      Some(templex_parser.parse_templex(iter)?)
    } else {
      if is_in_lambda {
        // Allow it, lambdas can figure out their type from the callee
        None
      } else if is_in_citizen {
        // Allow it, just assume it's the containing struct
        None
      } else if is_in_function {
        return Err(ParseError::LightFunctionMustHaveParamTypes {
          pos: pattern_begin,
          param_index: index as i32,
        });
      } else {
        // Allow it, just a regular pattern
        None
      }
    };

    // Parse optional destructure (lines 196-215)
    let maybe_destructure = match iter.peek_cloned() {
      Some(INodeLEEnum::Squared(SquaredLE {
        range: destructure_range,
        contents: destructure_elements,
      })) => {
        let destructure_elements = destructure_elements.clone();
        iter.advance();

        let destructure_iter = ScrambleIterator::new(&destructure_elements);
        let element_iters = destructure_iter.split_on_symbol(',', false);

        let mut patterns = Vec::new();
        for (index, mut element_iter) in element_iters.into_iter().enumerate() {
          let pos = element_iter.get_pos();
          let pattern = self.parse_pattern(
            &mut element_iter,
            templex_parser,
            pos,
            index,
            false,
            false,
            false,
            None,
          )?;
          patterns.push(pattern);
        }

        Some(DestructureP {
          range: destructure_range,
          patterns: alloc_slice_from_vec(self.arena, patterns),
        })
      }
      Some(other) => return Err(ParseError::BadThingAfterTypeInPattern(other.range().begin())),
      None => None,
    };

    // Return the complete pattern (lines 217-220)
    Ok(PatternPP::<'p> {
      range: RangeL(pattern_begin, iter.get_prev_end_pos()),
      destination: maybe_destination_local,
      templex: maybe_type,
      destructure: maybe_destructure,
    })
  }
  /*
    def parsePattern(iter: ScrambleIterator, patternBegin: Int, index: Int, isInCitizen: Boolean, isInFunction: Boolean, isInLambda: Boolean, maybeNameFromParameter: Option[WordLE]): Result[PatternPP, IParseError] = {
      if (!iter.hasNext) {
        maybeNameFromParameter match {
          case None => return Err(EmptyPattern(patternBegin))
          case Some(WordLE(range, str)) => {
  //          Ok(
  //            PatternPP(
  //              RangeL(patternBegin, iter.getPrevEndPos()),
  //              Some(DestinationLocalP(LocalNameDeclarationP(NameP(range, str)), None)),
  //              None,
  //              None))
          }
        }

      }

      val isConstructing =
        iter.peek2_cloned() match {
          case (Some(WordLE(_, self)), Some(SymbolLE(range, '.')))
            if self == keywords.self => {
            iter.advance()
            iter.advance()
            true
          }
          case _ => false
        }

      val maybeMutate = iter.trySkipWord(keywords.set)
      if (maybeMutate.nonEmpty && !iter.hasNext) {
        return Err(CantUseThatLocalName(iter.getPos(), "set"))
      }

      val maybeDestinationLocal =
        maybeNameFromParameter match {
          case Some(WordLE(range, str)) => {
            if (str == keywords.UNDERSCORE) {
              Some(DestinationLocalP(IgnoredLocalNameDeclarationP(range), None))
            } else {
              Some(DestinationLocalP(LocalNameDeclarationP(NameP(range, str)), None))
            }
          }
          case None => {
            val nameIsNext =
              iter.peek2_cloned() match {
                case (None, None) => vwat() // impossible
                case (Some(_), None) => true
                case (Some(first), Some(second)) => {
                  if (first.range.end < second.range.begin) {
                    // There's a space after the first thing, so it's a name.
                    true
                  } else {
                    // There's no space after the first thing, so not a name.
                    false
                  }
                }
              }
            if (nameIsNext) {
              iter.peek_cloned() match {
                case Some(WordLE(range, str)) => {
                  iter.advance()
                  if (str == keywords.UNDERSCORE) {
                    Some(DestinationLocalP(IgnoredLocalNameDeclarationP(range), maybeMutate))
                  } else {
                    if (isConstructing) {
                      Some(DestinationLocalP(ConstructingMemberNameDeclarationP(NameP(range, str)), maybeMutate))
                    } else {
                      Some(DestinationLocalP(LocalNameDeclarationP(NameP(range, str)), maybeMutate))
                    }
                  }
                }
                case Some(SquaredLE(_, _)) => None
                case _ => return Err(BadLocalName(iter.getPos()))
              }
            } else {
              None
            }
          }
        }

      // We look ahead so we dont parse "in" as a type in: foreach x in myList { ... }
      iter.peek_cloned() match {
        case None =>
        case Some(WordLE(_, in)) if in == keywords.in => iter.stop()
        case Some(_) =>
      }

      // The next thing might be a type or a destructure.
      // If it's a square-braced thing with nothing after it, it's a destructure.
      // See https://github.com/ValeLang/Vale/issues/434
      val nextIsType =
        iter.peek2_cloned() match {
          case (None, None) => false
          case (Some(SquaredLE(_, _)), maybeAfter) => {
            // If there's something after it, it's an array.
            maybeAfter.nonEmpty
          }
          case (Some(_), _) => {
            // There's something that's not square-braced, so it's a type.
            true
          }
        }
      val maybeType =
        if (nextIsType) {
          templexParser.parseTemplex(iter) match {
            case Err(e) => return Err(e)
            case Ok(x) => Some(x)
          }
        } else {
          if (isInLambda) {
            // Allow it, lambdas can figure out their type from the callee.
            None
          } else if (isInCitizen) {
            // Allow it, just assume it's the containing struct.
            None
          } else if (isInFunction) {
            return Err(LightFunctionMustHaveParamTypes(patternBegin, index))
          } else {
            // Allow it, just a regular pattern
            None
          }
        }

      val maybeDestructure =
        iter.peek_cloned() match {
          case Some(SquaredLE(destructureRange, destructureElements)) => {
            iter.advance()
            val destructure =
              DestructureP(
                destructureRange,
                U.mapWithIndex[ScrambleIterator, PatternPP](
                  new ScrambleIterator(destructureElements).splitOnSymbol(',', false),
                  (index, destructureElementIter) => {
                    parsePattern(destructureElementIter, destructureElementIter.getPos(), index, false, false, false, None) match {
                      case Err(e) => return Err(e)
                      case Ok(x) => x
                    }
                }).toVector)
            Some(destructure)
          }
          case Some(other) => return Err(BadThingAfterTypeInPattern(other.range.begin))
          case None => None
        }

        Ok(
          PatternPP(
            RangeL(patternBegin, iter.getPrevEndPos()),
            maybeDestinationLocal, maybeType, maybeDestructure))
    }

    //    pos ~
    //    opt(pstr("virtual") <~ white) ~
    //    (
    //
    //      // First, the ones with types:
    //        // Yes capture, yes type, yes destructure:
    //        underscoreOr(patternCapture) ~ (white ~> templex) ~ destructure ^^ { case capture ~ tyype ~ destructure => (None, capture, Some(tyype), Some(destructure)) } |
    //        // Yes capture, yes type, no destructure:
    //        underscoreOr(patternCapture) ~ (white ~> templex) ^^ { case capture ~ tyype => (None, capture, Some(tyype), None) } |
    //        // No capture, yes type, yes destructure:
    //        templex ~ destructure ^^ { case tyype ~ destructure => (None, None, Some(tyype), Some(destructure)) } |
    //        // No capture, yes type, no destructure: impossible.
    //      // Now, the ones with destructuring:
    //        // Yes capture, no type, yes destructure:
    //        underscoreOr(patternCapture) ~ (white ~> destructure) ^^ { case capture ~ destructure => (None, capture, None, Some(destructure)) } |
    //        // No capture, no type, yes destructure:
    //        destructure ^^ { case destructure => (None, None, None, Some(destructure)) } |
    //      // Now, a simple capture:
    //        // Yes capture, no type, no destructure:
    //        underscoreOr(patternCapture) ^^ { case capture => (None, capture, None, None) } |
    //        // Hacked in for highlighting, still need to incorporate into the above
    //        existsMW("*") ~ existsMW("!") ~ underscoreOr(patternCapture) ^^ { case selfBorrow ~ readwrite ~ capture => (selfBorrow, capture, None, None) }
    //    ) ~
    //    opt(white ~> "impl" ~> white ~> templex) ~
    //    pos ^^ {
    //      case begin ~ maybeVirtual ~ maybeselfBorrowAndMaybeCaptureAndMaybeTypeAndMaybeDestructure ~ maybeInterface ~ end => {
    //        val (maybeselfBorrow, maybeCapture, maybeType, maybeDestructure) = maybeselfBorrowAndMaybeCaptureAndMaybeTypeAndMaybeDestructure
    //        val maybeVirtuality =
    //          (maybeVirtual, maybeInterface) match {
    //            case (None, None) => None
    //            case (Some(range), None) => Some(AbstractP(range.range))
    //            case (None, Some(interface)) => Some(OverrideP(ast.RangeP(begin, end), interface))
    //            case (Some(_), Some(_)) => vfail()
    //          }
    //        ast.PatternPP(ast.RangeP(begin, end), maybeselfBorrow, maybeCapture, maybeType, maybeDestructure, maybeVirtuality)
    //      }
    //    }
  }
  */
}
