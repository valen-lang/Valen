/// Templex (Type Expression) Parser
/// Mirrors Frontend/ParsingPass/src/dev/vale/parsing/templex/TemplexParser.scala (732 lines)
///
/// This file implements type expression parsing exactly as in the Scala version.
/// All method names, variable names, and logic flow match the Scala implementation.
use crate::keywords::Keywords;
use crate::lexing::ast::*;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::parse_utils::{parse_region, try_skip_past_equals_while};
use crate::parsing::expression_parser::ScrambleIterator;
/*
package dev.vale.parsing.templex

import dev.vale.{Err, Interner, Keywords, Ok, Profiler, Result, StrI, U, vassert, vassertSome, vimpl, vwat}
import dev.vale.parsing.{Parser, StopBeforeCloseSquare, StopBeforeComma, ast}
import dev.vale.parsing.ast.{AnonymousRunePT, BoolPT, BorrowP, BuiltinCallPR, CallPT, ComponentsPR, EqualsPR, FinalP, FuncPT, IRulexPR, ITemplexPT, ImmutableP, InlineP, IntPT, InterpretedPT, LocationPT, MutabilityPT, MutableP, NameOrRunePT, NameP, OwnP, OwnershipPT, RegionRunePT, RuntimeSizedArrayPT, ShareP, StaticSizedArrayPT, StringPT, TemplexPR, TuplePT, TypedPR, VariabilityPT, VaryingP, WeakP, YonderP}
import dev.vale.lexing.{AngledLE, BadArraySizer, BadArraySizerEnd, BadPrototypeName, BadPrototypeParams, BadRegionName, BadRuleCallParam, BadRuneTypeError, BadStringChar, BadStringInTemplex, BadTemplateCallParam, BadTupleElement, BadTypeExpression, BadUnicodeChar, CurliedLE, FoundBothImmutableAndMutabilityInArray, INodeLE, IParseError, ParendLE, ParsedDoubleLE, ParsedIntegerLE, RangeL, RangedInternalErrorP, ScrambleLE, SquaredLE, StringLE, StringPartLiteral, SymbolLE, WordLE}
import dev.vale.parsing.Parser.parseRegion
import dev.vale.parsing._
import dev.vale.parsing.ast._

import scala.collection.mutable
*/

type ParseResult<T> = Result<T, ParseError>;

/// TemplexParser - parses type expressions
/// Mirrors Scala's TemplexParser class (line 13 in TemplexParser.scala)
pub struct TemplexParser<'p, 'ctx> {
  parse_arena: &'ctx crate::parse_arena::ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
}
/*
class TemplexParser(interner: Interner, keywords: Keywords) {
*/

impl<'p, 'ctx> TemplexParser<'p, 'ctx>
where
  'p: 'ctx,
{
  pub fn new(parse_arena: &'ctx crate::parse_arena::ParseArena<'p>, keywords: &'ctx Keywords<'p>) -> Self {
    TemplexParser {
      parse_arena,
      keywords,
    }
  }

  /// Parse an array type expression
  /// Mirrors parseArray in TemplexParser.scala lines 14-85
  pub fn parse_array(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<ITemplexPT<'p>>> {
    let begin = original_iter.get_pos();

    let mut tentative_iter = original_iter.clone();

    let immutable = tentative_iter.try_skip_symbol('#');

    // Check for squared brackets
    let squared_contents;
    let size_scramble_iter_l = match tentative_iter.peek_cloned() {
      Some(INodeLEEnum::Squared(squared)) => {
        squared_contents = squared.contents.clone();
        tentative_iter.advance();
        ScrambleIterator::new(&squared_contents)
      }
      _ => return Ok(None),
    };

    original_iter.skip_to(&tentative_iter);
    let iter = original_iter;

    // Parse size if present
    let maybe_size_templex = if size_scramble_iter_l.has_next() {
      let mut size_iter = size_scramble_iter_l;
      if size_iter.try_skip_symbol('#') {
        Some(self.parse_templex(&mut size_iter)?)
      } else {
        None
      }
    } else {
      None
    };

    // Parse template args for mutability/variability
    let template_args_begin = iter.get_pos();
    let maybe_template_args = self.parse_template_call_args(iter)?;
    let template_args_end = iter.get_pos();

    let mutability: &'p ITemplexPT<'p> = match (
      immutable,
      maybe_template_args.as_ref().and_then(|v| v.get(0)),
    ) {
      (true, Some(_)) => return Err(ParseError::FoundBothImmutableAndMutabilityInArray(begin)),
      (false, Some(templex)) => templex,
      (true, None) => &*self.parse_arena.alloc(ITemplexPT::Mutability(MutabilityPT(
        RangeL(template_args_begin, template_args_end),
        MutabilityP::Immutable,
      ))),
      (false, None) => &*self.parse_arena.alloc(ITemplexPT::Mutability(MutabilityPT(
        RangeL(template_args_begin, template_args_end),
        MutabilityP::Mutable,
      ))),
    };

    let variability: &'p ITemplexPT<'p> = maybe_template_args
        .as_ref()
        .and_then(|v| v.get(1).copied())
        .unwrap_or_else(|| {
          &*self.parse_arena.alloc(ITemplexPT::Variability(VariabilityPT(
            RangeL(template_args_begin, template_args_end),
            VariabilityP::Final,
          )))
        });

    let element_type = self.parse_templex(iter)?;

    let result = match maybe_size_templex {
      None => ITemplexPT::RuntimeSizedArray(RuntimeSizedArrayPT {
        range: RangeL(begin, iter.get_prev_end_pos()),
        mutability,
        element: &*self.parse_arena.alloc(element_type),
      }),
      Some(size_templex) => ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
        range: RangeL(begin, iter.get_prev_end_pos()),
        mutability,
        variability,
        size: &*self.parse_arena.alloc(size_templex),
        element: &*self.parse_arena.alloc(element_type),
      }),
    };

    Ok(Some(result))
  }
  /*
    def parseArray(originalIter: ScrambleIterator): Result[Option[ITemplexPT], IParseError] = {
      val begin = originalIter.getPos()

      val tentativeIter = originalIter.clone()

      val immutable = tentativeIter.trySkipSymbol('#')

      val sizeScrambleIterL =
        tentativeIter.peek() match {
          case Some(SquaredLE(range, squareContents)) => {
            tentativeIter.advance()
            new ScrambleIterator(squareContents)
          }
          case _ => return Ok(None)
        }

      originalIter.skipTo(tentativeIter)
      val iter = originalIter

      val maybeSizeTemplex =
        if (sizeScrambleIterL.hasNext) {
          if (sizeScrambleIterL.trySkipSymbol('#')) {
            parseTemplex(sizeScrambleIterL) match {
              case Err(e) => return Err(e)
              case Ok(x) => Some(x)
            }
          } else {
            None
          }
        } else {
          None
        }

      val templateArgsBegin = iter.getPos()
      val maybeTemplateArgs =
        parseTemplateCallArgs(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }
      val templateArgsEnd = iter.getPos()
      val mutability =
        (immutable, maybeTemplateArgs.toList.flatten.lift(0)) match {
          case (true, Some(_)) => return Err(FoundBothImmutableAndMutabilityInArray(begin))
          case (false, Some(templex)) => templex
          case (true, None) => MutabilityPT(RangeL(templateArgsBegin, templateArgsEnd), ImmutableP)
          case (false, None) => MutabilityPT(RangeL(templateArgsBegin, templateArgsEnd), MutableP)
        }
      val variability =
        maybeTemplateArgs.toList.flatten.lift(1)
          .getOrElse(VariabilityPT(RangeL(templateArgsBegin, templateArgsEnd), FinalP))

      val elementType = parseTemplex(iter) match { case Err(e) => return Err(e) case Ok(x) => x }

      val result =
        maybeSizeTemplex match {
          case None => {
            RuntimeSizedArrayPT(
              RangeL(begin, iter.getPrevEndPos()),
              mutability,
              elementType)
          }
          case Some(sizeTemplex) => {
            StaticSizedArrayPT(
              RangeL(begin, iter.getPrevEndPos()),
              mutability,
              variability,
              sizeTemplex,
              elementType)
          }
        }
      Ok(Some(result))
    }
  */

  /// Parse a function name (including operator names)
  /// Mirrors parseFunctionName in TemplexParser.scala lines 87-161
  pub fn parse_function_name(&self, iter: &mut ScrambleIterator<'p, '_>) -> Option<NameP<'p>> {
    match iter.peek_cloned() {
      Some(INodeLEEnum::Word(word)) => {
        let range = word.range;
        let str = word.str;
        iter.advance();
        Some(NameP(range, str))
      }
      Some(INodeLEEnum::Symbol(_)) => {
        let begin = iter.get_pos();
        match iter.peek3_cloned() {
          (
            Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
            Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
            Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
          ) => {
            iter.advance();
            iter.advance();
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.triple_equals,
            ))
          }
          (
            Some(INodeLEEnum::Symbol(SymbolLE(_, '<'))),
            Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
            Some(INodeLEEnum::Symbol(SymbolLE(_, '>'))),
          ) => {
            iter.advance();
            iter.advance();
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.spaceship,
            ))
          }
          (
            Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
            Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
            _,
          ) => {
            iter.advance();
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.double_equals,
            ))
          }
          (
            Some(INodeLEEnum::Symbol(SymbolLE(_, '!'))),
            Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
            _,
          ) => {
            iter.advance();
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.not_equals,
            ))
          }
          (
            Some(INodeLEEnum::Symbol(SymbolLE(_, '<'))),
            Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
            _,
          ) => {
            iter.advance();
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.less_equals,
            ))
          }
          (
            Some(INodeLEEnum::Symbol(SymbolLE(_, '>'))),
            Some(INodeLEEnum::Symbol(SymbolLE(_, '='))),
            _,
          ) => {
            iter.advance();
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.greater_equals,
            ))
          }
          (Some(INodeLEEnum::Symbol(SymbolLE(_, '<'))), _, _) => {
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.less,
            ))
          }
          (Some(INodeLEEnum::Symbol(SymbolLE(_, '>'))), _, _) => {
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.greater,
            ))
          }
          (Some(INodeLEEnum::Symbol(SymbolLE(_, '+'))), _, _) => {
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.plus,
            ))
          }
          (Some(INodeLEEnum::Symbol(SymbolLE(_, '-'))), _, _) => {
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.minus,
            ))
          }
          (Some(INodeLEEnum::Symbol(SymbolLE(_, '*'))), _, _) => {
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.asterisk,
            ))
          }
          (Some(INodeLEEnum::Symbol(SymbolLE(_, '/'))), _, _) => {
            iter.advance();
            Some(NameP(
              RangeL(begin, iter.get_prev_end_pos()),
              self.keywords.slash,
            ))
          }
          _ => None,
        }
      }
      Some(INodeLEEnum::Parend(ParendLE { range, .. })) => {
        // Don't advance, we do that elsewhere
        Some(NameP(
          RangeL(range.begin(), range.begin()),
          self.keywords.underscores_call,
        ))
      }
      _ => None,
    }
  }
  /*
    def parseFunctionName(iter: ScrambleIterator): Option[NameP] = {
      iter.peek() match {
        case Some(WordLE(range, name)) => {
          iter.advance()
          Some(NameP(range, name))
        }
        case Some(SymbolLE(_, _)) => {
          val begin = iter.getPos()
          iter.peek3() match {
            case (Some(SymbolLE(_, '=')), Some(SymbolLE(_, '=')), Some(SymbolLE(_, '='))) => {
              iter.advance()
              iter.advance()
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.tripleEquals))
            }
            case (Some(SymbolLE(_, '<')), Some(SymbolLE(_, '=')), Some(SymbolLE(_, '>'))) => {
              iter.advance()
              iter.advance()
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.spaceship))
            }
            case (Some(SymbolLE(_, '=')), Some(SymbolLE(_, '=')), _) => {
              iter.advance()
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.doubleEquals))
            }
            case (Some(SymbolLE(_, '!')), Some(SymbolLE(_, '=')), _) => {
              iter.advance()
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.notEquals))
            }
            case (Some(SymbolLE(_, '<')), Some(SymbolLE(_, '=')), _) => {
              iter.advance()
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.lessEquals))
            }
            case (Some(SymbolLE(_, '>')), Some(SymbolLE(_, '=')), _) => {
              iter.advance()
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.greaterEquals))
            }
            case (Some(SymbolLE(_, '<')), _, _) => {
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.less))
            }
            case (Some(SymbolLE(_, '>')), _, _) => {
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.greater))
            }
            case (Some(SymbolLE(_, '+')), _, _) => {
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.plus))
            }
            case (Some(SymbolLE(_, '-')), _, _) => {
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.minus))
            }
            case (Some(SymbolLE(_, '*')), _, _) => {
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.asterisk))
            }
            case (Some(SymbolLE(_, '/')), _, _) => {
              iter.advance()
              Some(NameP(RangeL(begin, iter.getPrevEndPos()), keywords.slash))
            }
            case _ => None
          }
        }
        case Some(ParendLE(range, _)) => {
          // Dont iter.advance(), we do that below.
          Some(NameP(RangeL(range.begin, range.begin), keywords.underscoresCall))
        }
        case _ => None
      }
    }
  */

  /// Parse a function prototype type
  /// Mirrors parsePrototype in TemplexParser.scala lines 163-189
  pub fn parse_prototype(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<ITemplexPT<'p>>> {
    let begin = iter.get_pos();

    if iter.try_skip_word(self.keywords.func).is_none() {
      return Ok(None);
    }

    let name = match self.parse_function_name(iter) {
      Some(n) => n,
      None => return Err(ParseError::BadPrototypeName(iter.get_pos())),
    };

    let args_begin = iter.get_pos();
    let args = match self.parse_tuple(iter)? {
      None => return Err(ParseError::BadPrototypeParams(iter.get_pos())),
      Some(ITemplexPT::Tuple(TuplePT { elements, .. })) => elements,
      Some(_) => return Err(ParseError::BadPrototypeParams(iter.get_pos())),
    };
    let args_end = iter.get_prev_end_pos();

    let return_type = self.parse_templex(iter)?;

    let result = ITemplexPT::Func(FuncPT {
      range: RangeL(begin, iter.get_prev_end_pos()),
      name,
      params_range: RangeL(args_begin, args_end),
      parameters: args,
      return_type: &*self.parse_arena.alloc(return_type),
    });

    Ok(Some(result))
  }
  /*
    def parsePrototype(iter: ScrambleIterator): Result[Option[ITemplexPT], IParseError] = {
      val begin = iter.getPos()

      if (iter.trySkipWord(keywords.func).isEmpty) {
        return Ok(None)
      }

      val name =
        parseFunctionName(iter) match {
          case Some(n) => n
          case None => return Err(BadPrototypeName(iter.getPos()))
        }

      val argsBegin = iter.getPos()
      val args =
        parseTuple(iter) match {
          case Err(e) => return Err(e)
          case Ok(None) => return Err(BadPrototypeParams(iter.getPos()))
          case Ok(Some(x)) => x.elements
        }
      val argsEnd = iter.getPrevEndPos()

      val returnType = parseTemplex(iter) match { case Err(e) => return Err(e) case Ok(x) => x }

      val result = FuncPT(RangeL(begin, iter.getPrevEndPos()), name, RangeL(argsBegin, argsEnd), args, returnType)
      Ok(Some(result))
    }
  */
  /*
  //  def parseRegioned(iter: ScrambleIterator): Result[Option[ITemplexPT], IParseError] = {
  //    val begin = iter.getPos()
  //    if (!iter.trySkipSymbol('\'')) {
  //      return Ok(None)
  //    }
  //
  //    val name =
  //      iter.nextWord() match {
  //        case None => return Err(BadRegionName(iter.getPos()))
  //        case Some(x) => x
  //      }
  //
  //    if (iter.hasNext) {
  //      val inner =
  //        parseTemplexAtomAndCallAndPrefixes(iter) match {
  //          case Err(e) => return Err(e)
  //          case Ok(t) => t
  //        }
  //      Ok(Some(inner))
  //    } else {
  //      val rune =
  //        RegionRunePT(
  //          RangeL(begin, iter.getPrevEndPos()),
  //          NameP(name.range, name.str))
  //      Ok(Some(rune))
  //    }
  //  }
  */
  /// Parse interpreted type (with ownership/region prefixes)
  /// Mirrors parseInterpreted in TemplexParser.scala lines 273-303
  pub fn parse_interpreted(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<ITemplexPT<'p>>> {
    let begin = iter.get_pos();

    // Parse ownership prefix (^, @, &&, &)
    let maybe_ownership = if iter.try_skip_symbol('^') {
      Some(OwnershipPT(RangeL(begin, iter.get_pos()), OwnershipP::Own))
    } else if iter.try_skip_symbol('@') {
      Some(OwnershipPT(RangeL(begin, iter.get_pos()), OwnershipP::Share))
    } else if iter.try_skip_symbols(&['&', '&']) {
      Some(OwnershipPT(RangeL(begin, iter.get_pos()), OwnershipP::Weak))
    } else if iter.try_skip_symbol('&') {
      Some(OwnershipPT(RangeL(begin, iter.get_pos()), OwnershipP::Borrow))
    } else {
      None
    };

    // Parse region (e.g., a' in a'T)
    let maybe_region = parse_region(iter)?;

    // If we have neither ownership nor region, return None
    match (&maybe_ownership, &maybe_region) {
      (None, None) => return Ok(None),
      _ => {}
    }

    // Parse the inner templex
    let inner = self.parse_templex_atom_and_call_and_prefixes(iter)?;

    Ok(Some(ITemplexPT::Interpreted(InterpretedPT {
      range: RangeL(begin, iter.get_prev_end_pos()),
      maybe_ownership: maybe_ownership.map(|x| &*self.parse_arena.alloc(x)),
      maybe_region: maybe_region.map(|x| &*self.parse_arena.alloc(x)),
      inner: &*self.parse_arena.alloc(inner),
    })))
  }
  /*
    def parseInterpreted(iter: ScrambleIterator): Result[Option[InterpretedPT], IParseError] = {
      val begin = iter.getPos()

      val maybeOwnership =
  //      if (iter.trySkipAll(Array({ case WordLE(_, pre) if pre == keywords.pre => }, { case SymbolLE(_, '&') => }))) { Some(OwnershipPT(RangeL(begin, iter.getPos()), PreCheckedBorrowP)) }
        if (iter.trySkipSymbol('^')) { Some(OwnershipPT(RangeL(begin, iter.getPos()), OwnP)) }
        else if (iter.trySkipSymbol('@')) { Some(OwnershipPT(RangeL(begin, iter.getPos()), ShareP)) }
        else if (iter.trySkipSymbols(Vector('&', '&'))) { Some(OwnershipPT(RangeL(begin, iter.getPos()), WeakP)) }
        else if (iter.trySkipSymbol('&')) { Some(OwnershipPT(RangeL(begin, iter.getPos()), BorrowP)) }
        else { None }

      val maybeRegion =
        parseRegion(iter) match {
          case Ok(None) => None
          case Ok(Some(regionRune)) => Some(regionRune)
          case Err(e) => return Err(e)
        }

      (maybeOwnership, maybeRegion) match {
        case (None, None) => return Ok(None)
        case (_, _) =>
      }

      val inner =
        parseTemplexAtomAndCallAndPrefixes(iter) match {
          case Err(e) => return Err(e)
          case Ok(t) => t
        }

      Ok(Some(ast.InterpretedPT(RangeL(begin, iter.getPrevEndPos()), maybeOwnership, maybeRegion, inner)))
    }
  */

  /// Parse ending region suffix (type')
  /// Mirrors parseEndingRegion in TemplexParser.scala lines 306-323
  pub fn parse_ending_region(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<RegionRunePT<'p>>> {
    let mut tentative_iter = original_iter.clone();

    let region = match parse_region(&mut tentative_iter)? {
      None => return Ok(None),
      Some(region_rune) => region_rune,
    };

    // This is an ending region, so nothing should follow
    if tentative_iter.has_next() {
      return Ok(None);
    }

    original_iter.skip_to(&tentative_iter);

    Ok(Some(region))
  }
  /*
    // This is a region that has nothing after it, like in Moo<a'>, not like Moo<a'T>.
    def parseEndingRegion(originalIter: ScrambleIterator): Result[Option[RegionRunePT], IParseError] = {
      val tentativeIter = originalIter.clone()

      val region =
        parseRegion(tentativeIter) match {
          case Err(e) => return Err(e)
          case Ok(None) => return Ok(None)
          case Ok(Some(regionRune)) => regionRune
        }

      if (tentativeIter.hasNext) {
        return Ok(None)
      }

      originalIter.skipTo(tentativeIter)

      Ok(Some(region))
    }
  */

  /// Parse templex atom and any following calls/prefixes/suffixes
  /// Mirrors parseTemplexAtomAndCallAndPrefixesAndSuffixes in TemplexParser.scala lines 326-334
  pub fn parse_templex_atom_and_call_and_prefixes_and_suffixes(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<ITemplexPT<'p>> {
    let inner = self.parse_templex_atom_and_call_and_prefixes(original_iter)?;
    Ok(inner)
  }
  /*
    def parseTemplexAtomAndCallAndPrefixesAndSuffixes(originalIter: ScrambleIterator): Result[ITemplexPT, IParseError] = {
      val inner =
        parseTemplexAtomAndCallAndPrefixes(originalIter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      return Ok(inner)
    }
  */

  /// Parse a templex atom (basic type expression)
  /// Mirrors parseTemplexAtom in TemplexParser.scala lines 336-441
  pub fn parse_templex_atom(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<ITemplexPT<'p>> {
    assert!(iter.peek_cloned().is_some());
    let _begin = iter.get_pos();

    // Try keywords first (lines 340-403)
    if let Some(range) = iter.try_skip_word(self.keywords.underscore) {
      return Ok(ITemplexPT::AnonymousRune(AnonymousRunePT { range }));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.truue) {
      return Ok(ITemplexPT::Bool(BoolPT { range, value: true }));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.faalse) {
      return Ok(ITemplexPT::Bool(BoolPT {
        range,
        value: false,
      }));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.own) {
      return Ok(ITemplexPT::Ownership(OwnershipPT(range, OwnershipP::Own)));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.borrow) {
      return Ok(ITemplexPT::Ownership(OwnershipPT(range, OwnershipP::Borrow)));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.weak) {
      return Ok(ITemplexPT::Ownership(OwnershipPT(range, OwnershipP::Weak)));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.share) {
      return Ok(ITemplexPT::Ownership(OwnershipPT(range, OwnershipP::Share)));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.inl) {
      return Ok(ITemplexPT::Location(LocationPT {
        range,
        location: LocationP::Inline,
      }));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.heap) {
      return Ok(ITemplexPT::Location(LocationPT {
        range,
        location: LocationP::Yonder,
      }));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.imm) {
      return Ok(ITemplexPT::Mutability(MutabilityPT(range, MutabilityP::Immutable)));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.r#mut) {
      return Ok(ITemplexPT::Mutability(MutabilityPT(range, MutabilityP::Mutable)));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.vary) {
      return Ok(ITemplexPT::Variability(VariabilityPT(range, VariabilityP::Varying)));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.fiinal) {
      return Ok(ITemplexPT::Variability(VariabilityPT(range, VariabilityP::Final)));
    }
    // Duplicate checks for weak, own, share (lines 392-403 in Scala)
    if let Some(range) = iter.try_skip_word(self.keywords.weak) {
      return Ok(ITemplexPT::Ownership(OwnershipPT(range, OwnershipP::Weak)));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.own) {
      return Ok(ITemplexPT::Ownership(OwnershipPT(range, OwnershipP::Own)));
    }
    if let Some(range) = iter.try_skip_word(self.keywords.share) {
      return Ok(ITemplexPT::Ownership(OwnershipPT(range, OwnershipP::Share)));
    }

    // Try parsing prototype (lines 404-408)
    if let Some(proto) = self.parse_prototype(iter)? {
      return Ok(proto);
    }

    // Try parsing tuple (lines 409-413)
    if let Some(tup) = self.parse_tuple(iter)? {
      return Ok(tup);
    }

    // Try parsing array (lines 414-418)
    if let Some(array) = self.parse_array(iter)? {
      return Ok(array);
    }

    // Parse other node types (lines 419-440)
    match iter.peek_cloned().expect("peek should not be empty") {
      INodeLEEnum::String(StringLE { range, parts }) => {
        let parts = parts.clone();
        iter.advance();
        match parts.as_slice() {
          [StringPart::Literal { range, s }] => Ok(ITemplexPT::String(StringPT {
            range: *range,
            str: self.parse_arena.intern_str(s.as_str()),
          })),
          _ => Err(ParseError::BadStringInTemplex(range.begin())),
        }
      }
      INodeLEEnum::ParsedInteger(ParsedIntegerLE { range, value, .. }) => {
        iter.advance();
        Ok(ITemplexPT::Int(IntPT { range, value }))
      }
      INodeLEEnum::ParsedDouble(ParsedDoubleLE { range, .. }) => {
        let pos = range.begin();
        iter.advance();
        Err(ParseError::RangedInternalError {
          pos,
          msg: "Floats in types not supported!".to_string(),
        })
      }
      INodeLEEnum::Word(WordLE { range, str }) => {
        iter.advance();
        Ok(ITemplexPT::NameOrRune(NameOrRunePT(NameP(range, str))))
      }
      _ => Err(ParseError::BadTypeExpression(iter.get_pos())),
    }
  }
  /*
    def parseTemplexAtom(iter: ScrambleIterator): Result[ITemplexPT, IParseError] = {
      vassert(iter.peek().nonEmpty)
      val begin = iter.getPos()

      iter.trySkipWord(keywords.UNDERSCORE) match {
        case Some(range) => return Ok(AnonymousRunePT(range))
        case None =>
      }
      iter.trySkipWord(keywords.truue) match {
        case Some(range) => return Ok(BoolPT(range, true))
        case None =>
      }
      iter.trySkipWord(keywords.faalse) match {
        case Some(range) => return Ok(BoolPT(range, false))
        case None =>
      }
      iter.trySkipWord(keywords.own) match {
        case Some(range) => return Ok(OwnershipPT(range, OwnP))
        case None =>
      }
      iter.trySkipWord(keywords.borrow) match {
        case Some(range) => return Ok(OwnershipPT(range, BorrowP))
        case None =>
      }
      iter.trySkipWord(keywords.weak) match {
        case Some(range) => return Ok(OwnershipPT(range, WeakP))
        case None =>
      }
      iter.trySkipWord(keywords.share) match {
        case Some(range) => return Ok(OwnershipPT(range, ShareP))
        case None =>
      }
      iter.trySkipWord(keywords.inl) match {
        case Some(range) => return Ok(LocationPT(range, InlineP))
        case None =>
      }
      iter.trySkipWord(keywords.heap) match {
        case Some(range) => return Ok(LocationPT(range, YonderP))
        case None =>
      }
      iter.trySkipWord(keywords.imm) match {
        case Some(range) => return Ok(MutabilityPT(range, ImmutableP))
        case None =>
      }
      iter.trySkipWord(keywords.mut) match {
        case Some(range) => return Ok(MutabilityPT(range, MutableP))
        case None =>
      }
      iter.trySkipWord(keywords.vary) match {
        case Some(range) => return Ok(VariabilityPT(range, VaryingP))
        case None =>
      }
      iter.trySkipWord(keywords.fiinal) match {
        case Some(range) => return Ok(VariabilityPT(range, FinalP))
        case None =>
      }
      iter.trySkipWord(keywords.weak) match {
        case Some(range) => return Ok(OwnershipPT(range, WeakP))
        case None =>
      }
      iter.trySkipWord(keywords.own) match {
        case Some(range) => return Ok(OwnershipPT(range, OwnP))
        case None =>
      }
      iter.trySkipWord(keywords.share) match {
        case Some(range) => return Ok(OwnershipPT(range, ShareP))
        case None =>
      }
      parsePrototype(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(tup)) => return Ok(tup)
        case Ok(None) =>
      }
      parseTuple(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(tup)) => return Ok(tup)
        case Ok(None) =>
      }
      parseArray(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(array)) => return Ok(array)
        case Ok(None) =>
      }
      vassertSome(iter.peek()) match {
        case StringLE(range, parts) => {
          iter.advance()
          parts match {
            case Vector(StringPartLiteral(range, s)) => Ok(StringPT(range, s))
            case _ => return Err(BadStringInTemplex(range.begin))
          }
        }
        case ParsedIntegerLE(range, int, bits) => {
          iter.advance()
          Ok(IntPT(range, int))
        }
        case ParsedDoubleLE(range, double, bits) => {
          iter.advance()
          return Err(RangedInternalErrorP(range.begin, "Floats in types not supported!"))
        }
        case WordLE(range, str) => {
          iter.advance()
          Ok(NameOrRunePT(NameP(range, str)))
        }
        case _ => return Err(BadTypeExpression(iter.getPos()))
      }
    }
  */

  /// Parse template call arguments <...>
  /// Mirrors parseTemplateCallArgs in TemplexParser.scala lines 443-461
  pub fn parse_template_call_args(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<&'p [&'p ITemplexPT<'p>]>> {
    let angled = match iter.peek_cloned() {
      Some(INodeLEEnum::Angled(a)) => a.clone(),
      Some(_) => return Ok(None),
      None => return Ok(None),
    };

    iter.advance();

    let mut elements_p: Vec<&'p ITemplexPT<'p>> = Vec::new();
    let angled_contents = angled.contents.clone();
    let contents_iter = ScrambleIterator::new(&angled_contents);
    let element_iters = contents_iter.split_on_symbol(',', false);

    for element_iter in element_iters {
      let mut elem_iter = element_iter.clone();
      elements_p.push(&*self.parse_arena.alloc(self.parse_templex(&mut elem_iter)?));
    }

    Ok(Some(self.parse_arena.alloc_slice_from_vec(elements_p)))
  }
  /*
    def parseTemplateCallArgs(iter: ScrambleIterator): Result[Option[Vector[ITemplexPT]], IParseError] = {
      val angled =
        iter.peek() match {
          case Some(a @ AngledLE(range, contents)) => a
          case Some(_) => return Ok(None)
          case None => return Ok(None)
        }
      iter.advance()
      val elementsP =
        U.map[ScrambleIterator, ITemplexPT](
          new ScrambleIterator(angled.contents).splitOnSymbol(',', false),
          elementIter => {
            parseTemplex(elementIter) match {
              case Err(e) => return Err(e)
              case Ok(x) => x
            }
          })
      Ok(Some(elementsP.toVector))
    }
  */

  /// Parse a tuple type
  /// Mirrors parseTuple in TemplexParser.scala lines 463-481
  pub fn parse_tuple(
    &self,
    outer_iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<ITemplexPT<'p>>> {
    let _begin = outer_iter.get_pos();

    match outer_iter.peek_cloned() {
      Some(INodeLEEnum::Parend(ParendLE { range, contents })) => {
        let contents = contents.clone();
        outer_iter.advance();

        let mut elements: Vec<&'p ITemplexPT<'p>> = Vec::new();
        let contents_iter = ScrambleIterator::new(&contents);
        let iter_splits = contents_iter.split_on_symbol(',', false);

        for iter_split in iter_splits {
          let mut iter = iter_split.clone();
          elements.push(&*self.parse_arena.alloc(self.parse_templex(&mut iter)?));
        }

        Ok(Some(ITemplexPT::Tuple(TuplePT {
          range,
          elements: self.parse_arena.alloc_slice_from_vec(elements),
        })))
      }
      _ => Ok(None),
    }
  }
  /*
    def parseTuple(outerIter: ScrambleIterator): Result[Option[TuplePT], IParseError] = {
      val begin = outerIter.getPos()
      outerIter.peek() match {
        case Some(ParendLE(range, contents)) => {
          outerIter.advance()
          val elements =
            U.map[ScrambleIterator, ITemplexPT](
              new ScrambleIterator(contents).splitOnSymbol(',', false),
              iter => {
                parseTemplex(iter) match {
                  case Err(e) => return Err(e)
                  case Ok(x) => x
                }
              })
          Ok(Some(TuplePT(range, elements.toVector)))
        }
        case _ => Ok(None)
      }
    }
  */


  /// Parse templex atom and any following call
  /// Mirrors parseTemplexAtomAndCall in TemplexParser.scala lines 483-499
  pub fn parse_templex_atom_and_call(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<ITemplexPT<'p>> {
    let begin = iter.get_pos();

    let atom = self.parse_templex_atom(iter)?;

    match self.parse_template_call_args(iter)? {
      Some(args) => {
        return Ok(ITemplexPT::Call(CallPT {
          range: RangeL(begin, iter.get_prev_end_pos()),
          template: &*self.parse_arena.alloc(atom),
          args,
        }));
      }
      None => {}
    }

    Ok(atom)
  }
  /*
    def parseTemplexAtomAndCall(iter: ScrambleIterator): Result[ITemplexPT, IParseError] = {
      val begin = iter.getPos()

      val atom =
        parseTemplexAtom(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      parseTemplateCallArgs(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(args)) => return Ok(CallPT(RangeL(begin, iter.getPrevEndPos()), atom, args))
        case Ok(None) =>
      }

      Ok(atom)
    }
  */

  /// Parse templex atom, call, and prefixes
  /// Mirrors parseTemplexAtomAndCallAndPrefixes in TemplexParser.scala lines 501-539
  pub fn parse_templex_atom_and_call_and_prefixes(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<ITemplexPT<'p>> {
    assert!(iter.has_next());

    // Check for 'in' keyword - should not be interpreted as a templex (lines 506-515)
    match iter.peek_cloned() {
      Some(INodeLEEnum::Word(WordLE { str, .. })) if str == self.keywords.r#in => {
        panic!("Should not interpret 'in' as a valid templex");
      }
      _ => {}
    }

    let _begin = iter.get_pos();

    // Try parsing an ending region (lines 525-529)
    if let Some(x) = self.parse_ending_region(iter)? {
      return Ok(ITemplexPT::RegionRune(x));
    }

    // Try parsing interpreted type with ownership/region prefixes (lines 531-535)
    if let Some(x) = self.parse_interpreted(iter)? {
      return Ok(x);
    }

    // Parse atom and call (line 537)
    self.parse_templex_atom_and_call(iter)
  }
  /*
    def parseTemplexAtomAndCallAndPrefixes(iter: ScrambleIterator): Result[ITemplexPT, IParseError] = {
      Profiler.frame(() => {
        vassert(iter.hasNext)

        iter.peek() match {
          case Some(WordLE(_, in)) if in == keywords.in => {
            // This is here so if we say:
            //   foreach x in myList { ... }
            // We won't interpret `x in` as a pattern, because
            // we don't interpret `in` as a valid templex.
            // The caller should prevent this.
            vwat()
          }
          case _ =>
        }

        val begin = iter.getPos()

        //    if (iter.trySkip(() => "^inl\\b".r)) {
        //
        //      val inner = parseTemplexAtomAndCallAndPrefixes(iter) match { case Err(e) => return Err(e) case Ok(t) => t }
        //      return Ok(InlinePT(RangeP(begin, iter.getPos()), inner))
        //    }

        parseEndingRegion(iter) match {
          case Err(e) => return Err(e)
          case Ok(Some(x)) => return Ok(x)
          case Ok(None) =>
        }

        parseInterpreted(iter) match {
          case Err(e) => return Err(e)
          case Ok(Some(x)) => return Ok(x)
          case Ok(None) =>
        }

        parseTemplexAtomAndCall(iter)
      })
    }
  */

  /// Main entry point for parsing a templex
  /// Mirrors parseTemplex in TemplexParser.scala lines 541-545
  pub fn parse_templex(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<ITemplexPT<'p>> {
    self.parse_templex_atom_and_call_and_prefixes_and_suffixes(iter)
  }
  /*
    def parseTemplex(iter: ScrambleIterator): Result[ITemplexPT, IParseError] = {
      Profiler.frame(() => {
        parseTemplexAtomAndCallAndPrefixesAndSuffixes(iter)
      })
    }
  */

  /// Parse a typed rune (T: Type)
  /// Mirrors parseTypedRune in TemplexParser.scala lines 547-571
  pub fn parse_typed_rune(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<IRulexPR<'p>>> {
    match original_iter.peek2_cloned() {
      // Don't parse "func moo()void" (lines 550-552)
      (Some(INodeLEEnum::Word(WordLE { str: name_str, .. })), _)
      if name_str == self.keywords.func =>
        {
          Ok(None)
        }
      (
        Some(INodeLEEnum::Word(WordLE {
                                 range: name_range,
                                 str: name_str,
                               })),
        Some(INodeLEEnum::Word(WordLE {
                                 range: type_range, ..
                               })),
      ) => {
        // Parse the rune name (or underscore for anonymous)
        let maybe_name = if name_str == self.keywords.underscore {
          None
        } else {
          Some(NameP(name_range, name_str))
        };

        original_iter.advance();

        // Parse the rune type
        let tyype = match self.parse_rune_type(original_iter)? {
          None => panic!("Expected rune type"),
          Some(x) => x,
        };

        Ok(Some(IRulexPR::Typed(TypedPR {
          range: RangeL(name_range.begin(), type_range.end()),
          rune: maybe_name,
          tyype,
        })))
      }
      _ => Ok(None),
    }
  }
  /*
    def parseTypedRune(originalIter: ScrambleIterator): Result[Option[TypedPR], IParseError] = {
      originalIter.peek2() match {
        // So we dont parse func moo()void
        case (Some(WordLE(nameRange, nameStr)), _) if nameStr == keywords.func => {
          Ok(None)
        }
        case (Some(WordLE(nameRange, nameStr)), Some(WordLE(typeRange, _))) => {
          val maybeName =
            if (nameStr == keywords.UNDERSCORE) {
              None
            } else {
              Some(NameP(nameRange, nameStr))
            }
          originalIter.advance()
          val tyype =
            parseRuneType(originalIter) match {
              case Err(e) => return Err(e)
              case Ok(None) => vwat()
              case Ok(Some(x)) => x
            }
          Ok(Some(ast.TypedPR(RangeL(nameRange.begin, typeRange.end), maybeName, tyype)))
        }
        case _ => Ok(None)
      }
    }
  */

  /// Parse a rule call
  /// Mirrors parseRuleCall in TemplexParser.scala lines 573-607
  pub fn parse_rule_call(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<Option<IRulexPR<'p>>> {
    match iter.peek2_cloned() {
      (Some(INodeLEEnum::Word(WordLE { str, .. })), _) if str == self.keywords.func => {
        return Ok(None);
      }
      (
        Some(INodeLEEnum::Word(WordLE {
                                 range: name_range,
                                 str: name,
                               })),
        Some(INodeLEEnum::Parend(ParendLE {
                                   range: args_range,
                                   contents: args_lr,
                                 })),
      ) => {
        let range = RangeL(name_range.begin(), args_range.end());

        // Parse the arguments
        let mut args_pr = Vec::new();
        let args_lr_clone = args_lr.clone();
        let args_iter = ScrambleIterator::new(&args_lr_clone);
        let arg_iters = args_iter.split_on_symbol(',', false);

        for arg_iter in arg_iters {
          let mut iter = arg_iter.clone();
          args_pr.push(self.parse_rule(&mut iter)?);
        }

        Ok(Some(IRulexPR::BuiltinCall(BuiltinCallPR {
          range,
          name: NameP(name_range, name),
          args: self.parse_arena.alloc_slice_from_vec(args_pr),
        })))
      }
      _ => Ok(None),
    }
  }
  /*
    def parseRuleCall(iter: ScrambleIterator): Result[Option[IRulexPR], IParseError] = {
      iter.peek2() match {
        case (Some(WordLE(_, StrI("func"))), _) => return Ok(None)
        case (Some(WordLE(nameRange, name)), Some(ParendLE(argsRange, argsLR))) => {
          val range = RangeL(nameRange.begin, argsRange.end)
          val argsPR =
            U.map[ScrambleIterator, IRulexPR](
              new ScrambleIterator(argsLR).splitOnSymbol(',', false),
              argIter => {
                parseRule(argIter) match {
                  case Err(e) => return Err(e)
                  case Ok(x) => x
                }
              })
          Ok(Some(BuiltinCallPR(range, NameP(nameRange, name), argsPR.toVector)))
        }
        case _ => return Ok(None)
      }

  //    val nameStr = nameAndOpenParen.init
  //    if (nameStr == "func") {
  //      return Ok(None)
  //    }
  //
  //    val nameEnd = tentativeIter.getPos() - 1
  //    val name = NameP(RangeP(begin, nameEnd), nameStr)
  //
  //    originalIter.skipTo(tentativeIter.getPos())
  //    val iter = originalIter
  //
  //    iter.consumeWhitespace()
  //    if (iter.trySkip(() => "^\\s*\\)".r)) {
  //      return Ok(Some(BuiltinCallPR(RangeP(begin, iter.getPos()), name, Vector())))
  //    }
    }
  */

  /// Parse a rule destructure
  /// Mirrors parseRuleDestructure in TemplexParser.scala lines 609-632
  pub fn parse_rule_destructure(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<IRulexPR<'p>>> {
    // Extract data from peek2() before mutating
    let (begin, end, components_l) = match original_iter.peek2_cloned() {
      (
        Some(INodeLEEnum::Word(WordLE {
                                 range: word_range, ..
                               })),
        Some(INodeLEEnum::Squared(SquaredLE {
                                    range: squared_range,
                                    contents: components_l,
                                  })),
      ) => (word_range.begin(), squared_range.end(), components_l.clone()),
      _ => return Ok(None),
    };

    // Parse the rune type
    let rune_type = match self.parse_rune_type(original_iter)? {
      None => panic!("Expected rune type"),
      Some(x) => x,
    };

    original_iter.advance();

    // Parse the components
    let mut components_p = Vec::new();
    let components_iter = ScrambleIterator::new(&components_l);
    let component_iters = components_iter.split_on_symbol(',', false);

    for component_iter in component_iters {
      let mut iter = component_iter.clone();
      components_p.push(self.parse_rule(&mut iter)?);
    }

    Ok(Some(IRulexPR::Components(ComponentsPR {
      range: RangeL(begin, end),
      container: rune_type,
      components: self.parse_arena.alloc_slice_from_vec(components_p),
    })))
  }
  /*
    def parseRuleDestructure(originalIter: ScrambleIterator): Result[Option[IRulexPR], IParseError] = {
      originalIter.peek2() match {
        case (Some(WordLE(RangeL(begin, _), _)), Some(SquaredLE(RangeL(_, end), componentsL))) => {
          val runeType =
            parseRuneType(originalIter) match {
              case Ok(None) => vwat()
              case Err(e) => return Err(e)
              case Ok(Some(x)) => x
            }
          originalIter.advance()
          val componentsP =
            U.map[ScrambleIterator, IRulexPR](
              new ScrambleIterator(componentsL).splitOnSymbol(',', false),
              componentIter => {
                parseRule(componentIter) match {
                  case Err(e) => return Err(e)
                  case Ok(x) => x
                }
              })
          Ok(Some(ComponentsPR(RangeL(begin, end), runeType, componentsP.toVector)))
        }
        case _ => Ok(None)
      }
    }
  */

  /// Parse a rule atom
  /// Mirrors parseRuleAtom in TemplexParser.scala lines 634-659
  pub fn parse_rule_atom(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<IRulexPR<'p>> {
    let _begin = iter.get_pos();

    // Try parsing a rule call (lines 637-641)
    if let Some(x) = self.parse_rule_call(iter)? {
      return Ok(x);
    }

    // Try parsing a rule destructure (lines 643-647)
    if let Some(x) = self.parse_rule_destructure(iter)? {
      return Ok(x);
    }

    // Try parsing a typed rune (lines 649-653)
    if let Some(x) = self.parse_typed_rune(iter)? {
      return Ok(x);
    }

    // Parse a templex as fallback (lines 655-658)
    let t = self.parse_templex(iter)?;
    Ok(IRulexPR::Templex(t))
  }
  /*
    def parseRuleAtom(iter: ScrambleIterator): Result[IRulexPR, IParseError] = {
      val begin = iter.getPos()

      parseRuleCall(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      parseRuleDestructure(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      parseTypedRune(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }

      parseTemplex(iter) match {
        case Err(e) => return Err(e)
        case Ok(t) => Ok(TemplexPR(t))
      }
    }
  */

  /// Parse a rule up to equals precedence
  /// Mirrors parseRuleUpToEqualsPrecedence in TemplexParser.scala lines 661-689
  /// Mirrors parseRuleUpToEqualsPrecedence in TemplexParser.scala lines 661-689
  pub fn parse_rule_up_to_equals_precedence(
    &self,
    iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<IRulexPR<'p>> {
    // Try to find an equals sign while scouting ahead (lines 663-672)
    let maybe_before_iter = try_skip_past_equals_while(iter, |scouting_iter| {
      match scouting_iter.peek_cloned() {
        None => false,
        // Stop on comma
        Some(INodeLEEnum::Symbol(SymbolLE(_, ','))) => false,
        // Stop if we hit an open brace, its the function body
        Some(INodeLEEnum::Curlied(_)) => false,
        _ => true,
      }
    });

    match maybe_before_iter {
      None => {
        // No equals found, just parse a rule atom (line 673)
        self.parse_rule_atom(iter)
      }
      Some(mut before_iter) => {
        // Found an equals, parse left and right sides (lines 674-687)
        let left = self.parse_rule_atom(&mut before_iter)?;
        let right = self.parse_rule_atom(iter)?;
        Ok(IRulexPR::Equals(EqualsPR {
          range: RangeL(left.range().begin(), right.range().end()),
          left: &*self.parse_arena.alloc(left),
          right: &*self.parse_arena.alloc(right),
        }))
      }
    }
  }
  /*
    def parseRuleUpToEqualsPrecedence(iter: ScrambleIterator): Result[IRulexPR, IParseError] = {
      Profiler.frame(() => {
        ParseUtils.trySkipPastEqualsWhile(iter, scoutingIter => {
          scoutingIter.peek() match {
            case None => false
            // Stop on comma
            case Some(SymbolLE(_, ',')) => false
            // Stop if we hit an open brace, its the function body
            case Some(CurliedLE(_, _)) => false
            case _ => true
          }
        })  match {
          case None => parseRuleAtom(iter)
          case Some(beforeIter) => {
            val left =
              parseRuleAtom(beforeIter) match {
                case Err(e) => return Err(e)
                case Ok(x) => x
              }
            val right =
              parseRuleAtom(iter) match {
                case Err(e) => return Err(e)
                case Ok(x) => x
              }
            Ok(EqualsPR(RangeL(left.range.begin, right.range.end), left, right))
          }
        }
      })
    }
  */

  /// Main entry point for parsing a rule
  /// Mirrors parseRule in TemplexParser.scala lines 691-693
  pub fn parse_rule(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<IRulexPR<'p>> {
    self.parse_rule_up_to_equals_precedence(iter)
  }
  /*
    def parseRule(s: ScrambleIterator): Result[IRulexPR, IParseError] = {
      parseRuleUpToEqualsPrecedence(s)
    }
  */

  /// Parse a rune type (Ref, Int, etc.)
  /// Mirrors parseRuneType in TemplexParser.scala lines 695-732
  pub fn parse_rune_type(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<Option<ITypePR>> {
    match iter.peek_cloned() {
      None => Ok(None),

      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.int_capitalized => {
        iter.advance();
        Ok(Some(ITypePR::IntType))
      }
      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.ref_ => {
        iter.advance();
        Ok(Some(ITypePR::CoordType))
      }
      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.kind => {
        iter.advance();
        Ok(Some(ITypePR::KindType))
      }
      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.region => {
        iter.advance();
        Ok(Some(ITypePR::RegionType))
      }
      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.prot => {
        iter.advance();
        Ok(Some(ITypePR::PrototypeType))
      }
      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.ref_list => {
        iter.advance();
        Ok(Some(ITypePR::CoordListType))
      }
      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.ownership => {
        iter.advance();
        Ok(Some(ITypePR::OwnershipType))
      }
      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.variability => {
        iter.advance();
        Ok(Some(ITypePR::VariabilityType))
      }
      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.mutability => {
        iter.advance();
        Ok(Some(ITypePR::MutabilityType))
      }
      Some(INodeLEEnum::Word(WordLE { str: w, .. })) if w == self.keywords.location => {
        iter.advance();
        Ok(Some(ITypePR::LocationType))
      }
      _ => Err(ParseError::BadRuneTypeError(iter.get_pos())),
    }
  }
  /*
    def parseRuneType(iter: ScrambleIterator):
    Result[Option[ITypePR], IParseError] = {
      iter.peek() match {
        case None => Ok(None)

        case Some(WordLE(_, w)) if w == keywords.IntCapitalized => {
          iter.advance(); Ok(Some(IntTypePR))
        }
        case Some(WordLE(_, w)) if w == keywords.Ref => {
          iter.advance(); Ok(Some(CoordTypePR))
        }
        case Some(WordLE(_, w)) if w == keywords.Kind => {
          iter.advance(); Ok(Some(KindTypePR))
        }
        case Some(WordLE(_, w)) if w == keywords.Region => {
          iter.advance(); Ok(Some(RegionTypePR))
        }
        case Some(WordLE(_, w)) if w == keywords.Prot => {
          iter.advance(); Ok(Some(PrototypeTypePR))
        }
        case Some(WordLE(_, w)) if w == keywords.RefList => {
          iter.advance(); Ok(Some(CoordListTypePR))
        }
        case Some(WordLE(_, w)) if w == keywords.Ownership => {
          iter.advance(); Ok(Some(OwnershipTypePR))
        }
        case Some(WordLE(_, w)) if w == keywords.Variability => {
          iter.advance(); Ok(Some(VariabilityTypePR))
        }
        case Some(WordLE(_, w)) if w == keywords.Mutability => {
          iter.advance(); Ok(Some(MutabilityTypePR))
        }
        case Some(WordLE(_, w)) if w == keywords.Location => {
          iter.advance(); Ok(Some(LocationTypePR))
        }
        case _ => return Err(BadRuneTypeError(iter.getPos()))
      }
    }
  */
}
/*
}
*/