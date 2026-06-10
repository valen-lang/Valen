use super::ast::*;
use super::errors::*;
use super::lexing_iterator::*;
use crate::Keywords;
use crate::parse_arena::ParseArena;
use std::result::Result as StdResult;
/*
package dev.vale.lexing

import dev.vale.{Accumulator, Err, Interner, Keywords, Ok, Profiler, Result, StrI, vassert, vassertSome, vfail, vimpl, vwat}

import scala.collection.mutable
*/

type Result<T> = StdResult<T, ParseError>;

/// Helper enum for string parsing
enum StringPartResult<'p> {
  Char(char),
  Expr(ScrambleLE<'p>),
}
/*
MIGALLOW: Scala didn't need this, it has Either for this.
*/

/// Vale lexer
/// Matches Scala's Lexer class
pub struct Lexer<'p, 'ctx> {
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
}
impl<'p, 'ctx> Lexer<'p, 'ctx>
where
  'p: 'ctx,
{
  /*
  class Lexer(interner: Interner, keywords: Keywords) {
  */
  pub fn new(parse_arena: &'ctx ParseArena<'p>, keywords: &'ctx Keywords<'p>) -> Self {
    Lexer { parse_arena, keywords }
  }

  /// Lex attributes on a declaration
  pub fn lex_attributes(&self, iter: &mut LexingIterator) -> Result<&'p [IAttributeL<'p>]>
  {
    let mut attributes = Vec::new();

    loop {
      iter.consume_comments_and_whitespace();
      match self.lex_attribute(iter)? {
        Some(attr) => attributes.push(attr),
        None => break,
      }
    }

    Ok(self.parse_arena.alloc_slice_from_vec(attributes))
  }
  /*
    def lexAttributes(iter: LexingIterator): Result[Vector[IAttributeL], IParseError] = {
      val attributesAccum = new Accumulator[IAttributeL]()
      while ({
        iter.consumeCommentsAndWhitespace()
        lexAttribute(iter) match {
          case Err(e) => return Err(e)
          case Ok(Some(attrL)) => attributesAccum.add(attrL); true
          case Ok(None) => false
        }
      }) {}
      Ok(attributesAccum.buildArray())
    }
  */

  /// Lex a single attribute
  pub fn lex_attribute(&self, iter: &mut LexingIterator) -> Result<Option<IAttributeL<'p>>>
  {
    let attribute_begin = iter.get_pos();

    // Check for macro calls
    if iter.try_skip_complete_word("#DeriveStructDrop") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::MacroCall {
        range: RangeL::new(attribute_begin, end),
        inclusion: IMacroInclusionL::CallMacro,
        name: WordLE {
          range: RangeL::new(attribute_begin, end),
          str: self.keywords.derive_struct_drop,
        },
      }));
    }

    if iter.try_skip_complete_word("#!DeriveStructDrop") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::MacroCall {
        range: RangeL::new(attribute_begin, end),
        inclusion: IMacroInclusionL::DontCallMacro,
        name: WordLE {
          range: RangeL::new(attribute_begin, end),
          str: self.keywords.derive_struct_drop,
        },
      }));
    }

    if iter.try_skip_complete_word("#DeriveAnonymousSubstruct") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::MacroCall {
        range: RangeL::new(attribute_begin, end),
        inclusion: IMacroInclusionL::CallMacro,
        name: WordLE {
          range: RangeL::new(attribute_begin, end),
          str: self.keywords.derive_anonymous_substruct,
        },
      }));
    }

    if iter.try_skip_complete_word("#!DeriveAnonymousSubstruct") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::MacroCall {
        range: RangeL::new(attribute_begin, end),
        inclusion: IMacroInclusionL::DontCallMacro,
        name: WordLE {
          range: RangeL::new(attribute_begin, end),
          str: self.keywords.derive_anonymous_substruct,
        },
      }));
    }

    if iter.try_skip_complete_word("#DeriveInterfaceDrop") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::MacroCall {
        range: RangeL::new(attribute_begin, end),
        inclusion: IMacroInclusionL::CallMacro,
        name: WordLE {
          range: RangeL::new(attribute_begin, end),
          str: self.keywords.derive_interface_drop,
        },
      }));
    }

    if iter.try_skip_complete_word("#!DeriveInterfaceDrop") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::MacroCall {
        range: RangeL::new(attribute_begin, end),
        inclusion: IMacroInclusionL::DontCallMacro,
        name: WordLE {
          range: RangeL::new(attribute_begin, end),
          str: self.keywords.derive_interface_drop,
        },
      }));
    }

    // Regular attributes
    if iter.try_skip_complete_word("abstract") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::AbstractAttribute(RangeL::new(
        attribute_begin,
        end,
      ))));
    }

    if iter.try_skip_complete_word("pure") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::PureAttribute(RangeL::new(
        attribute_begin,
        end,
      ))));
    }

    if iter.try_skip_complete_word("additive") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::AdditiveAttribute(RangeL::new(
        attribute_begin,
        end,
      ))));
    }

    if iter.try_skip_complete_word("extern") {
      let maybe_custom_name = if iter.peek() == '(' {
        Some(self.lex_parend(iter)?.unwrap())
      } else {
        None
      };
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::ExternAttribute::<'p> {
        range: RangeL::new(attribute_begin, end),
        maybe_custom_name,
      }));
    }

    if iter.try_skip_complete_word("exported") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::ExportAttribute(RangeL::new(
        attribute_begin,
        end,
      ))));
    }

    if iter.try_skip_complete_word("linear") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::LinearAttribute(RangeL::new(
        attribute_begin,
        end,
      ))));
    }

    if iter.try_skip_complete_word("weakable") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::WeakableAttribute(RangeL::new(
        attribute_begin,
        end,
      ))));
    }

    if iter.try_skip_complete_word("sealed") {
      let end = iter.get_pos();
      return Ok(Some(IAttributeL::SealedAttribute(RangeL::new(
        attribute_begin,
        end,
      ))));
    }

    Ok(None)
  }

  /*
    def lexAttribute(iter: LexingIterator): Result[Option[IAttributeL], IParseError] = {
      // Optimize: can xor the next 8 chars into a u64 and then
      // use xor and bitwise and to do these string comparisons
      // Might be a good idea to switch to trait so it fits in 8 letters lol
      val attributeBegin = iter.getPos()
      if (iter.trySkipCompleteWord("#DeriveStructDrop")) {
        val end = iter.getPos()
        Ok(
          Some(
            MacroCallL(
              RangeL(attributeBegin, end),
              CallMacroL,
              WordLE(RangeL(attributeBegin, end), keywords.DeriveStructDrop))))
      } else if (iter.trySkipCompleteWord("#!DeriveStructDrop")) {
        val end = iter.getPos()
        Ok(
          Some(
            MacroCallL(
              RangeL(attributeBegin, end),
              DontCallMacroL,
              WordLE(RangeL(attributeBegin, end), keywords.DeriveStructDrop))))
      } else if (iter.trySkipCompleteWord("#DeriveStructConstructor")) {
        val end = iter.getPos()
        Ok(
          Some(
            MacroCallL(
              RangeL(attributeBegin, end),
              CallMacroL,
              WordLE(RangeL(attributeBegin, end), keywords.DeriveStructConstructor))))
      } else if (iter.trySkipCompleteWord("#!DeriveStructConstructor")) {
        val end = iter.getPos()
        Ok(
          Some(
            MacroCallL(
              RangeL(attributeBegin, end),
              DontCallMacroL,
              WordLE(RangeL(attributeBegin, end), keywords.DeriveStructConstructor))))
      } else if (iter.trySkipCompleteWord("#DeriveAnonymousSubstruct")) {
          val end = iter.getPos()
          Ok(
            Some(
              MacroCallL(
                RangeL(attributeBegin, end),
                CallMacroL,
                WordLE(RangeL(attributeBegin, end), keywords.DeriveAnonymousSubstruct))))
      } else if (iter.trySkipCompleteWord("#!DeriveAnonymousSubstruct")) {
        val end = iter.getPos()
        Ok(
          Some(
            MacroCallL(
              RangeL(attributeBegin, end),
              DontCallMacroL,
              WordLE(RangeL(attributeBegin, end), keywords.DeriveAnonymousSubstruct))))
      } else if (iter.trySkipCompleteWord("#DeriveInterfaceDrop")) {
        val end = iter.getPos()
        Ok(
          Some(
            MacroCallL(
              RangeL(attributeBegin, end),
              CallMacroL,
              WordLE(RangeL(attributeBegin, end), keywords.DeriveInterfaceDrop))))
      } else if (iter.trySkipCompleteWord("#!DeriveInterfaceDrop")) {
        val end = iter.getPos()
        Ok(
          Some(
            MacroCallL(
              RangeL(attributeBegin, end),
              DontCallMacroL,
              WordLE(RangeL(attributeBegin, end), keywords.DeriveInterfaceDrop))))
      } else if (iter.trySkipCompleteWord("abstract")) {
        val end = iter.getPos()
        Ok(Some(AbstractAttributeL(RangeL(attributeBegin, end))))
      } else if (iter.trySkipCompleteWord("pure")) {
        val end = iter.getPos()
        Ok(Some(PureAttributeL(RangeL(attributeBegin, end))))
      } else if (iter.trySkipCompleteWord("additive")) {
        val end = iter.getPos()
        Ok(Some(AdditiveAttributeL(RangeL(attributeBegin, end))))
      } else if (iter.trySkipCompleteWord("extern")) {
        val maybeCustomName =
          if (iter.peek() == '(') {
            lexParend(iter) match {
              case Err(e) => return Err(e)
              case Ok(None) => vwat()
              case Ok(Some(x)) => Some(x)
            }
          } else {
            None
          }
        val end = iter.getPos()
        Ok(Some(ExternAttributeL(RangeL(attributeBegin, end), maybeCustomName)))
      } else if (iter.trySkipCompleteWord("exported")) {
        val end = iter.getPos()
        Ok(Some(ExportAttributeL(RangeL(attributeBegin, end))))
      } else if (iter.trySkipCompleteWord("linear")) {
        val end = iter.getPos()
        Ok(Some(LinearAttributeL(RangeL(attributeBegin, end))))
      } else if (iter.trySkipCompleteWord("weakable")) {
        val end = iter.getPos()
        Ok(Some(WeakableAttributeL(RangeL(attributeBegin, end))))
      } else if (iter.trySkipCompleteWord("sealed")) {
        val end = iter.getPos()
        Ok(Some(SealedAttributeL(RangeL(attributeBegin, end))))
      } else {
        Ok(None)
      }
    }
  */

  /// Lex a top-level denizen (function, struct, interface, impl, import, export)
  pub fn lex_denizen(&self, iter: &mut LexingIterator) -> Result<IDenizenL<'p>>
  {
    let denizen_begin = iter.get_pos();

    let attributes = self.lex_attributes(iter)?;
    iter.consume_comments_and_whitespace();

    // Try function
    if let Some(func) = self.lex_function(iter, denizen_begin, attributes)? {
      return Ok(IDenizenL::TopLevelFunction(func));
    }

    // Try struct
    if let Some(strukt) = self.lex_struct(iter, denizen_begin, attributes)? {
      return Ok(IDenizenL::TopLevelStruct(strukt));
    }

    // Try interface
    if let Some(interface) = self.lex_interface(iter, denizen_begin, attributes)? {
      return Ok(IDenizenL::TopLevelInterface(interface));
    }

    // Try impl
    if let Some(impl_) = self.lex_impl(iter, denizen_begin, attributes)? {
      return Ok(IDenizenL::TopLevelImpl(impl_));
    }

    // Try import
    if let Some(import) = self.lex_import(iter, denizen_begin, attributes)? {
      return Ok(IDenizenL::TopLevelImport(import));
    }

    // Try export
    if let Some(export) = self.lex_export(iter, denizen_begin, attributes)? {
      return Ok(IDenizenL::TopLevelExportAs(export));
    }

    Err(ParseError::UnrecognizedDenizenError(iter.get_pos()))
  }

  /*
    def lexDenizen(iter: LexingIterator):
    Result[IDenizenL, IParseError] = {
      val denizenBegin = iter.getPos()

      val attributes =
        lexAttributes(iter) match {
          case Err(e) => return Err(e)
          case Ok(a) => a
        }

      iter.consumeCommentsAndWhitespace()

      lexFunction(iter, denizenBegin, attributes) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(TopLevelFunctionL(x))
        case Ok(None) =>
      }

      lexStruct(iter, denizenBegin, attributes) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(TopLevelStructL(x))
        case Ok(None) =>
      }

      lexInterface(iter, denizenBegin, attributes) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(TopLevelInterfaceL(x))
        case Ok(None) =>
      }

      lexImpl(iter, denizenBegin, attributes) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(TopLevelImplL(x))
        case Ok(None) =>
      }

      lexImport(iter, denizenBegin, attributes) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(TopLevelImportL(x))
        case Ok(None) =>
      }

      lexExport(iter, denizenBegin, attributes) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(TopLevelExportAsL(x))
        case Ok(None) =>
      }

      return Err(UnrecognizedDenizenError(iter.getPos()))
    }
  */

  /// Lex an impl block
  pub fn lex_impl(
    &self,
    iter: &mut LexingIterator,
    begin: i32,
    attributes: &'p [IAttributeL<'p>],
  ) -> Result<Option<ImplL<'p>>>
  {
    if !iter.try_skip_complete_word("impl") {
      return Ok(None);
    }

    iter.consume_comments_and_whitespace();

    let maybe_identifying_runes = self.lex_angled(iter)?;
    iter.consume_comments_and_whitespace();

    let interface_ownership_symbols = self.lex_impl_ownership_prefix(iter);
    let interface_name = self
        .lex_identifier(iter)
        .ok_or(ParseError::BadImplInterface(iter.get_pos()))?;
    iter.consume_comments_and_whitespace();

    let maybe_interface_generic_args = self.lex_angled(iter)?;

    let mut interface_elements: Vec<&'p INodeLEEnum<'p>> = Vec::new();
    for sym in &interface_ownership_symbols {
      interface_elements.push(&*self.parse_arena.alloc(INodeLEEnum::Symbol(*sym)));
    }
    interface_elements.push(&*self.parse_arena.alloc(INodeLEEnum::Word::<'p>(interface_name)));
    if let Some(interface_generic_args) = maybe_interface_generic_args {
      interface_elements.push(&*self.parse_arena.alloc(INodeLEEnum::Angled::<'p>(interface_generic_args)));
    }
    let interface_begin = interface_elements.first().expect("interface_elements empty").range().begin();
    let interface_end = interface_elements.last().expect("interface_elements empty").range().end();
    let interface = ScrambleLE::<'p> {
      range: RangeL::new(interface_begin, interface_end),
      elements: self.parse_arena.alloc_slice_copy(&interface_elements),
    };

    iter.consume_comments_and_whitespace();

    if !iter.try_skip_complete_word("for") {
      return Err(ParseError::BadImplFor(iter.get_pos()));
    }

    iter.consume_comments_and_whitespace();

    let struct_ownership_symbols = self.lex_impl_ownership_prefix(iter);
    let struct_name = self
        .lex_identifier(iter)
        .ok_or(ParseError::BadImplStruct(iter.get_pos()))?;
    iter.consume_comments_and_whitespace();

    let maybe_struct_generic_args = self.lex_angled(iter)?;

    let mut struct_elements: Vec<&'p INodeLEEnum<'p>> = Vec::new();
    for sym in &struct_ownership_symbols {
      struct_elements.push(&*self.parse_arena.alloc(INodeLEEnum::Symbol(*sym)));
    }
    struct_elements.push(&*self.parse_arena.alloc(INodeLEEnum::Word(struct_name)));
    if let Some(struct_generic_args) = maybe_struct_generic_args {
      struct_elements.push(&*self.parse_arena.alloc(INodeLEEnum::Angled(struct_generic_args)));
    }
    let struct_begin = struct_elements.first().expect("struct_elements empty").range().begin();
    let struct_end = struct_elements.last().expect("struct_elements empty").range().end();
    let struct_ = ScrambleLE {
      range: RangeL::new(struct_begin, struct_end),
      elements: self.parse_arena.alloc_slice_copy(&struct_elements),
    };

    iter.consume_comments_and_whitespace();

    let maybe_rules = if iter.try_skip_complete_word("where") {
      Some(self.lex_scramble(iter, true, false, true)?)
    } else {
      None
    };

    iter.consume_comments_and_whitespace();

    if !iter.try_skip(';') {
      return Err(ParseError::BadImplEnd(iter.get_pos()));
    }

    iter.consume_comments_and_whitespace();

    let end = iter.get_pos();

    Ok(Some(ImplL::<'p> {
      range: RangeL::new(begin, end),
      identifying_runes: maybe_identifying_runes,
      template_rules: maybe_rules,
      struct_: Some(struct_),
      interface,
      attributes,
    }))
  }

  /*
    def lexImpl(iter: LexingIterator, begin: Int, attributes: Vector[IAttributeL]):
    Result[Option[ImplL], IParseError] = {
      if (!iter.trySkipCompleteWord("impl")) {
        return Ok(None)
      }

      iter.consumeCommentsAndWhitespace()

      val maybeIdentifyingRunes =
        lexAngled(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      iter.consumeCommentsAndWhitespace()
      val interfaceOwnershipSymbols = lexImplOwnershipPrefix(iter)
      val interfaceName =
        lexIdentifier(iter) match {
          case None => return Err(BadImplInterface(iter.getPos()))
          case Some(x) => x
        }

      iter.consumeCommentsAndWhitespace()

      val maybeInterfaceGenericArgs =
        lexAngled(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      val interface = {
        val elements = interfaceOwnershipSymbols ++ Vector(interfaceName) ++ maybeInterfaceGenericArgs.toVector
        val begin = elements.head.range.begin
        val end = elements.last.range.end
        ScrambleLE(RangeL(begin, end), elements)
      }

      iter.consumeCommentsAndWhitespace()

      if (!iter.trySkipCompleteWord("for")) {
        return Err(BadImplFor(iter.getPos()))
      }

      iter.consumeCommentsAndWhitespace()
      val structOwnershipSymbols = lexImplOwnershipPrefix(iter)
      val structName =
        lexIdentifier(iter) match {
          case None => return Err(BadImplStruct(iter.getPos()))
          case Some(x) => x
        }

      iter.consumeCommentsAndWhitespace()

      val maybeStructGenericArgs =
        lexAngled(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      val struct = {
        val elements = structOwnershipSymbols ++ Vector(structName) ++ maybeStructGenericArgs.toVector
        val begin = elements.head.range.begin
        val end = elements.last.range.end
        ScrambleLE(RangeL(begin, end), elements)
      }

      iter.consumeCommentsAndWhitespace()

      val maybeRules =
        if (iter.trySkipCompleteWord("where")) {
          Some(
            lexScramble(iter, true, false, true) match {
              case Err(e) => return Err(e)
              case Ok(x) => x
            })
        } else {
          None
        }

      iter.consumeCommentsAndWhitespace()

      if (!iter.trySkip(';')) {
        return Err(BadImplEnd(iter.getPos()))
      }

      iter.consumeCommentsAndWhitespace()

      val end = iter.getPos()

      val impl =
        ImplL(
          RangeL(begin, end),
          maybeIdentifyingRunes,
          maybeRules,
          Some(struct),
          interface,
          attributes)
      Ok(Some(impl))
    }
  */

  /// Lex a function definition
  /// MIGBAD: Scala uses iter.peek(2) and iter.peek(3) for multi-character lookahead (lines 703-740), while Rust uses iter.peek_string() method. Change Rust to match Scala.
  pub fn lex_function(
    &self,
    iter: &mut LexingIterator,
    begin: i32,
    attributes: &'p [IAttributeL<'p>],
  ) -> Result<Option<FunctionL<'p>>>
  {
    if !iter.try_skip_complete_word("func") && !iter.try_skip_complete_word("funky") {
      return Ok(None);
    }

    iter.consume_comments_and_whitespace();

    let name_begin = iter.get_pos();
    let name = if let Some(id) = self.lex_identifier(iter) {
      id
    } else {
      // Check for operator names
      let name_str = match iter.peek() {
        '!' if iter.peek_string("!=") => {
          iter.skip_to(name_begin as usize + 2);
          self.keywords.not_equals
        }
        '=' => {
          if iter.peek_string("===") {
            iter.skip_to(name_begin as usize + 3);
            self.keywords.triple_equals
          } else if iter.peek_string("==") {
            iter.skip_to(name_begin as usize + 2);
            self.keywords.double_equals
          } else {
            return Err(ParseError::BadFunctionName(iter.get_pos()));
          }
        }
        '>' if iter.peek_string(">=") => {
          iter.skip_to(name_begin as usize + 2);
          self.keywords.greater_equals
        }
        '>' => {
          iter.advance();
          self.keywords.greater
        }
        '<' => {
          if iter.peek_string("<=>") {
            iter.skip_to(name_begin as usize + 3);
            self.keywords.spaceship
          } else if iter.peek_string("<=") {
            iter.skip_to(name_begin as usize + 2);
            self.keywords.less_equals
          } else {
            iter.advance();
            self.keywords.less
          }
        }
        '+' => {
          iter.advance();
          self.keywords.plus
        }
        '-' => {
          iter.advance();
          self.keywords.minus
        }
        '*' => {
          iter.advance();
          self.keywords.asterisk
        }
        '/' => {
          iter.advance();
          self.keywords.slash
        }
        _ => return Err(ParseError::BadFunctionName(iter.get_pos())),
      };

      let end = iter.get_pos();
      WordLE {
        range: RangeL::new(name_begin, end),
        str: name_str,
      }
    };

    let maybe_generic_args = self.lex_angled(iter)?;
    iter.consume_comments_and_whitespace();

    let params = self
        .lex_parend(iter)?
        .ok_or(ParseError::BadFunctionParamsBegin(iter.get_pos()))?;

    iter.consume_comments_and_whitespace();

    let trailing_details = self.lex_scramble(iter, true, false, true)?;
    iter.consume_comments_and_whitespace();

    let header_end = iter.get_pos();

    let maybe_body = if iter.try_skip(';') {
      None
    } else {
      let body = self
          .lex_curlied(iter, false)?
          .ok_or(ParseError::BadFunctionBodyError(iter.get_pos()))?;
      Some(FunctionBodyL { body })
    };

    let end = iter.get_pos();

    let header = FunctionHeaderL {
      range: RangeL::new(begin, header_end),
      name,
      attributes,
      maybe_user_specified_identifying_runes: maybe_generic_args,
      params,
      trailing_details,
    };

    Ok(Some(FunctionL::<'p> {
      range: RangeL::new(begin, end),
      header,
      body: maybe_body,
    }))
  }

  /*
    def lexFunction(iter: LexingIterator, begin: Int, attributes: Vector[IAttributeL]):
    Result[Option[FunctionL], IParseError] = {
      if (!iter.trySkipCompleteWord("func") && !iter.trySkipCompleteWord("funky")) {
        return Ok(None)
      }

      iter.consumeCommentsAndWhitespace()

      val nameBegin = iter.getPos()
      val name =
        lexIdentifier(iter) match {
          case None => {
            val begin = iter.getPos()
            val nameStr =
              iter.peek() match {
                case '!' => {
                  iter.peek(2) match {
                    case Some("!=") => keywords.notEquals
                    case _ => return Err(BadFunctionName(iter.getPos()))
                  }
                }
                case '=' => {
                  iter.peek(2) match {
                    case Some("==") => {
                      iter.peek(3) match {
                        case Some("===") => keywords.tripleEquals
                        case _ => keywords.doubleEquals
                      }
                    }
                    case _ => return Err(BadFunctionName(iter.getPos()))
                  }
                }
                case '>' => {
                  iter.peek(2) match {
                    case Some(">=") => keywords.greaterEquals
                    case _ => keywords.greater
                  }
                }
                case '<' => {
                  iter.peek(2) match {
                    case Some("<=") => {
                      iter.peek(3) match {
                        case Some("<=>") => keywords.spaceship
                        case _ => keywords.lessEquals
                      }
                    }
                    case _ => keywords.less
                  }
                }
                case '+' => keywords.plus
                case '-' => keywords.minus
                case '*' => keywords.asterisk
                case '/' => keywords.slash
              }
            iter.skipTo(begin + nameStr.str.length)
            val end = iter.getPos()
            WordLE(RangeL(begin, iter.getPos()), nameStr)
          }
          case Some(x) => x
        }

      val maybeGenericArgs =
        lexAngled(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      iter.consumeCommentsAndWhitespace()

      val params =
        lexParend(iter) match {
          case Err(e) => return Err(e)
          case Ok(None) => vfail() // MIGALLOW: Rust should handle this better
          case Ok(Some(x)) => x
        }

      iter.consumeCommentsAndWhitespace()

      val trailingDetails =
        lexScramble(iter, true, false, true) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      iter.consumeCommentsAndWhitespace()

      val headerEnd = iter.getPos()

      val maybeBody =
        if (iter.trySkip(';')) {
          None
        } else {
          val body =
            lexCurlied(iter, false) match {
              case Err(e) => return Err(e)
              case Ok(Some(x)) => x
              case Ok(None) => return Err(BadFunctionBodyError(iter.getPos()))
            }

          Some(FunctionBodyL(body))
        }

      val end = iter.getPos()

      val header =
        FunctionHeaderL(
          RangeL(begin, headerEnd),
          name,
          attributes,
          maybeGenericArgs,
          params,
          trailingDetails)
      val func = FunctionL(RangeL(begin, end), header, maybeBody)
      Ok(Some(func))
    }
  */

  /// Lex a struct definition
  pub fn lex_struct(
    &self,
    iter: &mut LexingIterator,
    begin: i32,
    attributes: &'p [IAttributeL<'p>],
  ) -> Result<Option<StructL<'p>>>
  {
    if !iter.try_skip_complete_word("struct") {
      return Ok(None);
    }

    iter.consume_comments_and_whitespace();

    let name = self
        .lex_identifier(iter)
        .ok_or(ParseError::BadStructName(iter.get_pos()))?;
    iter.consume_comments_and_whitespace();

    let maybe_generic_args = self.lex_angled(iter)?;
    iter.consume_comments_and_whitespace();

    let maybe_mutability = if !iter.peek_complete_word("where") && iter.peek() != '{' {
      Some(self.lex_scramble(iter, true, true, true)?)
    } else {
      None
    };

    iter.consume_comments_and_whitespace();

    let maybe_rules = if iter.try_skip_complete_word("where") {
      Some(self.lex_scramble(iter, true, false, true)?)
    } else {
      None
    };

    iter.consume_comments_and_whitespace();

    let header_end = iter.get_pos();

    let _maybe_default_region = if iter.try_skip_complete_word("region") {
      Some(self.lex_node(iter, true, false)?)
    } else {
      None
    };

    iter.consume_comments_and_whitespace();

    let mut members_acc: Vec<ScrambleLE<'p>> = Vec::new();
    let mut methods_acc: Vec<FunctionL<'p>> = Vec::new();
    let contents_range = if iter.peek() == ';' {
      let r = RangeL::new(iter.get_pos(), iter.get_pos());
      if !iter.try_skip(';') { panic!("vwat"); }
      iter.consume_comments_and_whitespace();
      r
    } else if iter.try_skip('{') {
      iter.consume_comments_and_whitespace();
      let contents_begin = iter.get_pos();
      let mut keep_going = true;
      while keep_going {
        if iter.at_end() || iter.peek() == '}' {
          keep_going = false;
        } else {
          let member_begin = iter.get_pos();
          let mut func_trial_iter = iter.clone();
          let trial_attributes = self.lex_attributes(&mut func_trial_iter)?;
          match self.lex_function(&mut func_trial_iter, member_begin, trial_attributes)? {
            Some(func) => {
              iter.position = func_trial_iter.position;
              iter.consume_comments_and_whitespace();
              methods_acc.push(func);
            }
            None => {
              members_acc.push(self.lex_scramble(iter, false, false, true)?);
              iter.consume_comments_and_whitespace();
              if !iter.try_skip(';') {
                return Err(ParseError::BadStructContentsEnd(iter.get_pos()));
              }
              iter.consume_comments_and_whitespace();
            }
          }
        }
      }
      let contents_end = iter.get_pos();
      if !iter.try_skip('}') {
        return Err(ParseError::BadStructContentsEnd(iter.get_pos()));
      }
      iter.consume_comments_and_whitespace();
      RangeL::new(contents_begin, contents_end)
    } else {
      return Err(ParseError::BadStructContentsBegin(iter.get_pos()));
    };

    Ok(Some(StructL {
      range: RangeL::new(begin, header_end),
      name,
      attributes,
      mutability: maybe_mutability,
      identifying_runes: maybe_generic_args,
      template_rules: maybe_rules,
      contents_range,
      members: self.parse_arena.alloc_slice_from_vec(members_acc),
      methods: self.parse_arena.alloc_slice_from_vec(methods_acc),
    }))
  }

  /*
    def lexStruct(iter: LexingIterator, begin: Int, attributes: Vector[IAttributeL]):
    Result[Option[StructL], IParseError] = {
      if (!iter.trySkipCompleteWord("struct")) {
        return Ok(None)
      }

      iter.consumeCommentsAndWhitespace()

      val nameBegin = iter.getPos() // MIGALLOW: Unused
      val name =
        lexIdentifier(iter) match {
          case None => return Err(BadStructName(iter.getPos()))
          case Some(x) => x
        }

      iter.consumeCommentsAndWhitespace()

      val maybeGenericArgs =
        lexAngled(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      iter.consumeCommentsAndWhitespace()

      val maybeMutability =
        if (!iter.peekCompleteWord("where") && iter.peek() != '{') {
          lexScramble(iter, true, true, true) match {
            case Err(e) => return Err(e)
            case Ok(x) => Some(x)
          }
        } else {
          None
        }

      iter.consumeCommentsAndWhitespace()

      val maybeRules =
        if (iter.trySkipCompleteWord("where")) {
          Some(
            lexScramble(iter, true, false, true) match {
              case Err(e) => return Err(e)
              case Ok(x) => x
            })
        } else {
          None
        }

      iter.consumeCommentsAndWhitespace()

      val headerEnd = iter.getPos()

      val maybeDefaultRegion =
        if (iter.trySkipCompleteWord("region")) {
          Some(
            lexNode(iter, true, false) match {
              case Err(e) => return Err(e)
              case Ok(x) => x
            })
        } else {
          None
        }

      iter.consumeCommentsAndWhitespace()
      val membersAcc = new Accumulator[ScrambleLE]()
      val methodsAcc = new Accumulator[FunctionL]()
      val contentsRange =
        if (iter.peek() == ';') {
          val r = RangeL(iter.getPos(), iter.getPos())
          if (!iter.trySkip(';')) vwat()
          iter.consumeCommentsAndWhitespace()
          r
        } else if (iter.trySkip('{')) {
          iter.consumeCommentsAndWhitespace()
          val contentsBegin = iter.getPos()
          var keepGoing = true
          while (keepGoing) {
            if (iter.atEnd() || iter.peek() == '}') {
              keepGoing = false
            } else {
              val memberBegin = iter.getPos()
              val funcTrialIter = iter.clone()
              val trialAttributes =
                lexAttributes(funcTrialIter) match {
                  case Err(e) => return Err(e)
                  case Ok(a) => a
                }
              lexFunction(funcTrialIter, memberBegin, trialAttributes) match {
                case Err(e) => return Err(e)
                case Ok(Some(func)) => {
                  iter.position = funcTrialIter.position
                  iter.consumeCommentsAndWhitespace()
                  methodsAcc.add(func)
                }
                case Ok(None) => {
                  membersAcc.add(
                    lexScramble(iter, false, false, true) match {
                      case Err(e) => return Err(e)
                      case Ok(x) => x
                    })
                  iter.consumeCommentsAndWhitespace()
                  if (!iter.trySkip(';')) {
                    return Err(BadStructContentsEnd(iter.getPos()))
                  }
                  iter.consumeCommentsAndWhitespace()
                }
              }
            }
          }
          val contentsEnd = iter.getPos()
          if (!iter.trySkip('}')) {
            return Err(BadStructContentsEnd(iter.getPos()))
          }
          iter.consumeCommentsAndWhitespace()
          RangeL(contentsBegin, contentsEnd)
        } else {
          return Err(BadStructContentsBegin(iter.getPos()))
        }
      val end = iter.getPos()

      val struct =
        StructL(
          RangeL(begin, headerEnd),
          name,
          attributes,
          maybeMutability,
          maybeGenericArgs,
          maybeRules,
          contentsRange,
          membersAcc.buildArray(),
          methodsAcc.buildArray())
      Ok(Some(struct))
    }
  */

  /// Lex an interface definition
  pub fn lex_interface(
    &self,
    iter: &mut LexingIterator,
    begin: i32,
    attributes: &'p [IAttributeL<'p>],
  ) -> Result<Option<InterfaceL<'p>>>
  {
    if !iter.try_skip_complete_word("interface") {
      return Ok(None);
    }

    iter.consume_comments_and_whitespace();

    let name = self
        .lex_identifier(iter)
        .ok_or(ParseError::BadInterfaceName(iter.get_pos()))?;
    iter.consume_comments_and_whitespace();

    let maybe_generic_args = self.lex_angled(iter)?;
    iter.consume_comments_and_whitespace();

    let maybe_mutability = if !iter.peek_complete_word("where") && iter.peek() != '{' {
      Some(self.lex_scramble(iter, true, true, true)?)
    } else {
      None
    };

    iter.consume_comments_and_whitespace();

    let maybe_rules = if iter.try_skip_complete_word("where") {
      Some(self.lex_scramble(iter, true, false, true)?)
    } else {
      None
    };

    iter.consume_comments_and_whitespace();

    let header_end = iter.get_pos();

    let _maybe_default_region = if iter.try_skip_complete_word("region") {
      Some(self.lex_node(iter, true, false)?)
    } else {
      None
    };

    iter.consume_comments_and_whitespace();

    if !iter.try_skip('{') {
      return Err(ParseError::BadInterfaceContentsBegin(iter.get_pos()));
    }
    let members_begin = iter.get_pos();

    iter.consume_comments_and_whitespace();

    let mut members = Vec::new();
    while !iter.at_end() && !iter.try_skip('}') {
      let member_begin = iter.get_pos();
      let member_attributes = self.lex_attributes(iter)?;

      if let Some(func) = self.lex_function(iter, member_begin, member_attributes)? {
        members.push(func);
      } else {
        return Err(ParseError::BadInterfaceMember(iter.get_pos()));
      }

      iter.consume_comments_and_whitespace();
    }

    iter.consume_comments_and_whitespace();

    let end = iter.get_pos();

    Ok(Some(InterfaceL {
      range: RangeL::new(begin, header_end),
      name,
      attributes,
      mutability: maybe_mutability,
      maybe_identifying_runes: maybe_generic_args,
      template_rules: maybe_rules,
      body_range: RangeL::new(members_begin, end),
      members: self.parse_arena.alloc_slice_from_vec(members),
    }))
  }

  /*
    def lexInterface(iter: LexingIterator, begin: Int, attributes: Vector[IAttributeL]):
    Result[Option[InterfaceL], IParseError] = {
      if (!iter.trySkipCompleteWord("interface")) {
        return Ok(None)
      }

      iter.consumeCommentsAndWhitespace()

      val nameBegin = iter.getPos()
      val name =
        lexIdentifier(iter) match {
          case None => return Err(BadInterfaceName(iter.getPos()))
          case Some(x) => x
        }

      iter.consumeCommentsAndWhitespace()

      val maybeGenericArgs =
        lexAngled(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      iter.consumeCommentsAndWhitespace()

      val maybeMutability =
        if (!iter.peekCompleteWord("where") && iter.peek() != '{') {
          lexScramble(iter, true, true, true) match {
            case Err(e) => return Err(e)
            case Ok(x) => Some(x)
          }
        } else {
          None
        }

      iter.consumeCommentsAndWhitespace()

      val maybeRules =
        if (iter.trySkipCompleteWord("where")) {
          Some(
            lexScramble(iter, true, false, true) match {
              case Err(e) => return Err(e)
              case Ok(x) => x
            })
        } else {
          None
        }

      iter.consumeCommentsAndWhitespace()

      val headerEnd = iter.getPos()

      val maybeDefaultRegion =
        if (iter.trySkipCompleteWord("region")) {
          Some(
            lexNode(iter, true, false) match {
              case Err(e) => return Err(e)
              case Ok(x) => x
            })
        } else {
          None
        }

      iter.consumeCommentsAndWhitespace()

      if (!iter.trySkip('{')) {
        return Err(BadInterfaceContentsBegin(iter.getPos()))
      }
      val membersBegin = iter.getPos()

      iter.consumeCommentsAndWhitespace()

      val membersAcc = new Accumulator[FunctionL]()
      while (!iter.atEnd() && !iter.trySkip('}')) {
        val memberBegin = iter.getPos()
        val attributes =
          lexAttributes(iter) match {
            case Err(e) => return Err(e)
            case Ok(a) => a
          }
        lexFunction(iter, memberBegin, attributes) match {
          case Err(e) => return Err(e)
          case Ok(Some(func)) => {
            membersAcc.add(func)
          }
          case Ok(None) => return Err(BadInterfaceMember(iter.getPos()))
        }
        iter.consumeCommentsAndWhitespace()
      }
      val members = membersAcc.buildArray()

      iter.consumeCommentsAndWhitespace()

      val end = iter.getPos()

      val interface =
        InterfaceL(
          RangeL(begin, headerEnd),
          name,
          attributes,
          maybeMutability,
          maybeGenericArgs,
          maybeRules,
          RangeL(membersBegin, end),
          members)
      Ok(Some(interface))
    }
  */

  /// Lex an import declaration
  pub fn lex_import(
    &self,
    iter: &mut LexingIterator,
    begin: i32,
    attributes: &'p [IAttributeL<'p>],
  ) -> Result<Option<ImportL<'p>>>
  {
    if !iter.try_skip_complete_word(self.keywords.impoort.as_str()) {
      return Ok(None);
    }

    if !attributes.is_empty() {
      return Err(ParseError::UnexpectedAttributes(iter.get_pos()));
    }

    let mut steps = Vec::new();
    loop {
      iter.consume_comments_and_whitespace();

      let step_begin = iter.get_pos();
      let name = if iter.try_skip('*') {
        WordLE {
          range: RangeL::new(step_begin, iter.get_pos()),
          str: self.keywords.asterisk,
        }
      } else {
        self
            .lex_identifier(iter)
            .ok_or(ParseError::BadImportName(iter.get_pos()))?
      };
      steps.push(name);

      iter.consume_comments_and_whitespace();

      if iter.try_skip('.') {
        continue;
      } else if iter.try_skip(';') {
        break;
      } else {
        return Err(ParseError::BadImportEnd(iter.get_pos()));
      }
    }

    let module_name = steps[0].clone();
    let importee_name = steps.last().unwrap().clone();
    let package_steps = self.parse_arena.alloc_slice_copy(&steps[1..steps.len() - 1]);

    Ok(Some(ImportL {
      range: RangeL::new(begin, iter.get_pos()),
      module_name,
      package_steps,
      importee_name,
    }))
  }
  /*
    def lexImport(iter: LexingIterator, begin: Int, attributes: Vector[IAttributeL]):
    Result[Option[ImportL], IParseError] = {
      if (!iter.trySkipCompleteWord(keywords.impoort.str)) {
        return Ok(None)
      }

      if (attributes.nonEmpty) {
        return Err(UnexpectedAttributes(iter.getPos()))
      }

      val steps = mutable.ArrayBuffer[WordLE]()
      while ({
        iter.consumeCommentsAndWhitespace()

        val stepBegin = iter.getPos()
        val name =
          if (iter.trySkip('*')) {
            WordLE(RangeL(stepBegin, iter.getPos()), keywords.asterisk)
          } else {
            lexIdentifier(iter) match {
              case None => return Err(BadImportName(iter.getPos()))
              case Some(n) => n
            }
          }
        steps += name

        iter.consumeCommentsAndWhitespace()

        if (iter.trySkip('.')) {
          true
        } else if (iter.trySkip(';')) {
          false
        } else {
          return Err(BadImportEnd(iter.getPos()))
        }
      }) {}

      val moduleName = steps.head
      val importee = steps.last
      val packageSteps = steps.init.tail
      val imporrt = ImportL(RangeL(begin, iter.getPos()), moduleName, packageSteps.toVector, importee)
      Ok(Some(imporrt))
    }
  */

  /// Lex an export declaration
  pub fn lex_export(
    &self,
    iter: &mut LexingIterator,
    begin: i32,
    attributes: &'p [IAttributeL<'p>],
  ) -> Result<Option<ExportAsL<'p>>>
  {
    if !iter.try_skip_complete_word(self.keywords.export.as_str()) {
      return Ok(None);
    }

    if !attributes.is_empty() {
      return Err(ParseError::UnexpectedAttributes(iter.get_pos()));
    }

    let scramble = self.lex_scramble(iter, false, false, true)?;
    iter.consume_comments_and_whitespace();

    if iter.try_skip('.') {
      // Continue
    } else if iter.try_skip(';') {
      // End
    } else {
      return Err(ParseError::BadImportEnd(iter.get_pos()));
    }

    Ok(Some(ExportAsL {
      range: RangeL::new(begin, iter.get_pos()),
      contents: scramble,
    }))
  }
  /*
    def lexExport(iter: LexingIterator, begin: Int, attributes: Vector[IAttributeL]):
    Result[Option[ExportAsL], IParseError] = {
      if (!iter.trySkipCompleteWord(keywords.export.str)) {
        return Ok(None)
      }

      if (attributes.nonEmpty) {
        return Err(UnexpectedAttributes(iter.getPos()))
      }

      val scramble =
        lexScramble(iter, false, false, true) match {
          case Err(e) => return Err(e)
          case Ok(s) => s
        }

      iter.consumeCommentsAndWhitespace()

      if (iter.trySkip('.')) {
        true
      } else if (iter.trySkip(';')) {
        false
      } else {
        return Err(BadImportEnd(iter.getPos()))
      }

      val export = ExportAsL(RangeL(begin, iter.getPos()), scramble)
      Ok(Some(export))
    }
  */

  /// Lex parenthesized expression
  fn lex_parend(&self, iter: &mut LexingIterator) -> Result<Option<ParendLE<'p>>> {
    let begin = iter.get_pos();

    if !iter.try_skip('(') {
      return Ok(None);
    }

    iter.consume_comments_and_whitespace();

    let innards = self.lex_scramble(iter, false, false, false)?;

    iter.consume_comments_and_whitespace();

    if !iter.try_skip(')') {
      return Err(ParseError::BadExpressionEnd(iter.get_pos()));
    }

    let end = iter.get_pos();

    Ok(Some(ParendLE::<'p> {
      range: RangeL::new(begin, end),
      contents: innards,
    }))
  }
  /*
    def lexParend(iter: LexingIterator): Result[Option[ParendLE], IParseError] = {
      Profiler.frame(() => {
        val begin = iter.getPos()

        if (!iter.trySkip('(')) {
          return Ok(None)
        }

        iter.consumeCommentsAndWhitespace()

        val innards =
          lexScramble(iter, false, false, false) match {
            case Err(e) => return Err(e)
            case Ok(x) => x
          }

        iter.consumeCommentsAndWhitespace()

        if (!iter.trySkip(')')) {
          vfail() // MIGALLOW: Rust handles this better
        }

        val end = iter.getPos()

        Ok(Some(ParendLE(RangeL(begin, end), innards)))
      })
    }
  */

  /// Lex curly braced block
  fn lex_curlied(
    &self,
    iter: &mut LexingIterator,
    stop_on_open_brace: bool,
  ) -> Result<Option<CurliedLE<'p>>> {
    let begin = iter.get_pos();

    if iter.peek() == '{' && stop_on_open_brace {
      return Ok(None);
    }

    if !iter.try_skip('{') {
      return Ok(None);
    }

    iter.consume_comments_and_whitespace();

    let innards = self.lex_scramble(iter, false, false, false)?;

    iter.consume_comments_and_whitespace();

    if iter.try_skip(')') {
      return Err(ParseError::BadStartOfStatementError(iter.get_pos()));
    }

    if iter.try_skip(']') {
      return Err(ParseError::BadStartOfStatementError(iter.get_pos()));
    }

    if !iter.try_skip('}') {
      return Err(ParseError::BadExpressionEnd(iter.get_pos()));
    }

    let end = iter.get_pos();

    Ok(Some(CurliedLE {
      range: RangeL::new(begin, end),
      contents: innards,
    }))
  }
  /*
    def lexCurlied(iter: LexingIterator, stopOnOpenBrace: Boolean): Result[Option[CurliedLE], IParseError] = {
      Profiler.frame(() => {
        val begin = iter.getPos()

        if (iter.peek() == '{' && stopOnOpenBrace) {
          return Ok(None)
        }

        if (!iter.trySkip('{')) {
          return Ok(None)
        }

        iter.consumeCommentsAndWhitespace()

        val innards =
          lexScramble(iter, false, false, false) match {
            case Err(e) => return Err(e)
            case Ok(x) => x
          }

        iter.consumeCommentsAndWhitespace()

        if (iter.trySkip(')')) {
          return Err(BadStartOfStatementError(iter.getPos()))
        }

        if (iter.trySkip(']')) {
          return Err(BadStartOfStatementError(iter.getPos()))
        }

        if (!iter.trySkip('}')) {
          vfail() // MIGALLOW: Rust handles this better
        }

        val end = iter.getPos()

        Ok(Some(CurliedLE(RangeL(begin, end), innards)))
      })
    }
  */

  /// Lex square bracketed expression
  fn lex_squared(&self, iter: &mut LexingIterator) -> Result<Option<SquaredLE<'p>>> {
    let begin = iter.get_pos();

    if !iter.try_skip('[') {
      return Ok(None);
    }

    iter.consume_comments_and_whitespace();

    let innards = self.lex_scramble(iter, false, false, false)?;

    iter.consume_comments_and_whitespace();

    if !iter.try_skip(']') {
      return Err(ParseError::BadExpressionEnd(iter.get_pos()));
    }

    let end = iter.get_pos();

    Ok(Some(SquaredLE {
      range: RangeL::new(begin, end),
      contents: innards,
    }))
  }
  /*
    def lexSquared(iter: LexingIterator): Result[Option[SquaredLE], IParseError] = {
      Profiler.frame(() => {
        val begin = iter.getPos()

        if (!iter.trySkip('[')) {
          return Ok(None)
        }

        iter.consumeCommentsAndWhitespace()

        val innards =
          lexScramble(iter, false, false, false) match {
            case Err(e) => return Err(e)
            case Ok(x) => x
          }

        iter.consumeCommentsAndWhitespace()

        if (!iter.trySkip(']')) {
          vfail() // MIGALLOW: Rust handles this better
        }

        val end = iter.getPos()

        Ok(Some(SquaredLE(RangeL(begin, end), innards)))
      })
    }
  */

  /// Lex angle bracketed expression (generics)
  fn lex_angled(&self, iter: &mut LexingIterator) -> Result<Option<AngledLE<'p>>> {
    let begin = iter.get_pos();

    if !(iter.peek() == '<' && self.angle_is_open_or_close(iter)) {
      return Ok(None);
    }
    iter.advance();

    iter.consume_comments_and_whitespace();

    let innards = self.lex_scramble(iter, false, false, false)?;

    iter.consume_comments_and_whitespace();

    if !iter.try_skip('>') {
      return Err(ParseError::BadExpressionEnd(iter.get_pos()));
    }

    let end = iter.get_pos();

    Ok(Some(AngledLE {
      range: RangeL::new(begin, end),
      contents: innards,
    }))
  }
  /*
    def lexAngled(iter: LexingIterator): Result[Option[AngledLE], IParseError] = {
      Profiler.frame(() => {
        val begin = iter.getPos()

        if (!(iter.peek() == '<' && angleIsOpenOrClose(iter))) {
          return Ok(None)
        }
        iter.advance()

        iter.consumeCommentsAndWhitespace()

        val innards =
          lexScramble(iter, false, false, false) match {
            case Err(e) => return Err(e)
            case Ok(x) => x
          }

        iter.consumeCommentsAndWhitespace()

        if (!iter.trySkip('>')) {
          vfail() // MIGALLOW: Rust handles this better
        }

        val end = iter.getPos()

        Ok(Some(AngledLE(RangeL(begin, end), innards)))
      })
    }
  */

  /// Check if < or > is an open/close bracket vs a comparison operator
  fn angle_is_open_or_close(&self, iter: &LexingIterator) -> bool {
    let c = iter.peek();
    if c != '<' && c != '>' {
      return false;
    }

    // >= and <= are comparison operators, not brackets
    if iter.peek_string(">=") || iter.peek_string("<=") {
      return false;
    }

    // => is lambda arrow, not a closer
    if iter.code[..iter.position].chars().next_back() == Some('=') && c == '>' {
      return false;
    }

    // If whitespace on both sides, it's a comparison operator
    let whitespace_before = matches!(
      iter.code[..iter.position].chars().next_back(),
      Some(' ' | '\t' | '\n' | '\r')
    );

    let whitespace_after = matches!(
      iter.code[iter.position..].chars().nth(1),
      Some(' ' | '\t' | '\n' | '\r')
    );

    let whitespace_on_both_sides = whitespace_before && whitespace_after;
    !whitespace_on_both_sides
  }
  /*
    def angleIsOpenOrClose(iter: LexingIterator): Boolean = {
      iter.peek() match {
        case '<' | '>' => // continue
        case _ => return false
      }
      iter.peek(2) match {
        case Some(">=") => return false
        case Some("<=") => return false
        case _ => // continue
      }
      // If we're encountering a > after a =, then its a => like in a lambda.
      // Not a closer.
      if (iter.position - 1 >= 0 &&
          iter.code.charAt(iter.position - 1) == '=' &&
          iter.code.charAt(iter.position) == '>') {
        return false
      }

      val whitespaceBefore =
        iter.position - 1 >= 0 &&
          (iter.code.charAt(iter.position - 1) match {
            case ' ' | '\t' | '\n' | '\r' => true
            case _ => false
          })
      val whitespaceAfter =
        iter.position + 1 < iter.code.length &&
          (iter.code.charAt(iter.position + 1) match {
            case ' ' | '\t' | '\n' | '\r' => true
            case _ => false
          })
      val whitespaceOnBothSides = whitespaceBefore && whitespaceAfter
      val isOpenOrClose = !whitespaceOnBothSides
      isOpenOrClose
    }
  */

  /*
  //  def lexCommaSeparatedList(iter: LexingIterator, stopOnOpenBrace: Boolean, stopOnWhere: Boolean): Result[CommaSeparatedListLE, IParseError] = {
  //    val begin = iter.getPos()
  //
  //    // If this encounters a ; or or ) or } a non-binary > then it should stop.
  //    iter.consumeCommentsAndWhitespace()
  //
  //    val innards = new Accumulator[ScrambleLE]()
  //    var trailingComma = false
  //
  //    while (!atEnd(iter, stopOnOpenBrace, stopOnWhere) && !iter.trySkip(',')) {
  //      iter.consumeCommentsAndWhitespace()
  //
  //      if (atEnd(iter, stopOnOpenBrace, stopOnWhere)) {
  //        trailingComma = true
  //      } else {
  //        val node =
  //          lexScramble(iter, stopOnOpenBrace, stopOnWhere) match {
  //            case Err(e) => return Err(e)
  //            case Ok(x) => x
  //          }
  //        innards.add(node)
  //      }
  //
  //      iter.consumeCommentsAndWhitespace()
  //    }
  //
  //    val end = iter.getPos()
  //
  //    Ok(CommaSeparatedListLE(RangeL(begin, end), innards.buildArray(), trailingComma))
  //  }
  */

  /// Check if we're at the end of a scramble
  fn at_end(
    &self,
    iter: &LexingIterator,
    stop_on_open_brace: bool,
    stop_on_where: bool,
    stop_on_semicolon: bool,
  ) -> bool {
    if iter.at_end() {
      return true;
    }

    if stop_on_where && iter.peek_string("where") {
      return true;
    }

    match iter.peek() {
      ')' | '}' | ']' => true,
      '{' => stop_on_open_brace,
      ';' => stop_on_semicolon,
      '>' => self.angle_is_open_or_close(iter),
      _ => false,
    }
  }
  /*
    def atEnd(iter: LexingIterator, stopOnOpenBrace: Boolean, stopOnWhere: Boolean, stopOnSemicolon: Boolean): Boolean = {
      if (iter.atEnd()) {
        return true
      }
      if (stopOnWhere && iter.peekString("where")) {
        return true
      }
      iter.peek() match {
        case ')' | '}' | ']' => true
        case '{' => stopOnOpenBrace
        case ';' => stopOnSemicolon
        case '>' => angleIsOpenOrClose(iter)
        case _ => false
      }
    }
  */

  /// Lex a scramble of nodes (unstructured sequence)
  pub fn lex_scramble(
    &self,
    iter: &mut LexingIterator,
    stop_on_open_brace: bool,
    stop_on_where: bool,
    stop_on_semicolon: bool,
  ) -> Result<ScrambleLE<'p>> {
    let begin = iter.get_pos();

    iter.consume_comments_and_whitespace();

    let mut innards: Vec<&'p INodeLEEnum<'p>> = Vec::new();

    while !self.at_end(iter, stop_on_open_brace, stop_on_where, stop_on_semicolon) {
      let node = self.lex_node(iter, stop_on_open_brace, stop_on_where)?;
      innards.push(&*self.parse_arena.alloc(node));

      iter.consume_comments_and_whitespace();
    }

    let end = iter.get_pos();

    Ok(ScrambleLE::<'p> {
      range: RangeL::new(begin, end),
      elements: self.parse_arena.alloc_slice_copy(&innards),
    })
  }
  /*
    def lexScramble(iter: LexingIterator, stopOnOpenBrace: Boolean, stopOnWhere: Boolean, stopOnSemicolon: Boolean): Result[ScrambleLE, IParseError] = {
      Profiler.frame(() => {
        val begin = iter.getPos()

        iter.consumeCommentsAndWhitespace()

        val innards = new Accumulator[INodeLE]()

        // If this encounters a ; or or ) or } a non-binary > then it should stop.
        while (!atEnd(iter, stopOnOpenBrace, stopOnWhere, stopOnSemicolon)) {
          val node =
            lexNode(iter, stopOnOpenBrace, stopOnWhere) match {
              case Err(e) => return Err(e)
              case Ok(x) => x
            }

          innards.add(node)

          iter.consumeCommentsAndWhitespace()
        }

        val end = iter.getPos()

        Ok(ScrambleLE(RangeL(begin, end), innards.buildArray()))
      })
    }

  */

  /// Lex a single node
  fn lex_node(
    &self,
    iter: &mut LexingIterator,
    stop_on_open_brace: bool,
    stop_on_where: bool,
  ) -> Result<INodeLEEnum<'p>> {
    // Try angled
    if let Some(x) = self.lex_angled(iter)? {
      return Ok(INodeLEEnum::Angled(x));
    }

    // Try squared
    if let Some(x) = self.lex_squared(iter)? {
      return Ok(INodeLEEnum::Squared(x));
    }

    // Try curlied
    if let Some(x) = self.lex_curlied(iter, stop_on_open_brace)? {
      return Ok(INodeLEEnum::Curlied(x));
    }

    // Try parend
    if let Some(x) = self.lex_parend(iter)? {
      return Ok(INodeLEEnum::Parend(x));
    }

    // Lex atom
    self.lex_atom(iter, stop_on_where)
  }
  /*
    def lexNode(iter: LexingIterator, stopOnOpenBrace: Boolean, stopOnWhere: Boolean): Result[INodeLE, IParseError] = {
      lexAngled(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }
      lexSquared(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }
      lexCurlied(iter, stopOnOpenBrace) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }
      lexParend(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(x)) => return Ok(x)
        case Ok(None) =>
      }
      lexAtom(iter, stopOnWhere) match {
        case Err(e) => Err(e)
        case Ok(x) => Ok(x)
      }
    }
  */

  /// Lex an atomic element (identifier, number, string, symbol)
  fn lex_atom(&self, iter: &mut LexingIterator, stop_on_where: bool) -> Result<INodeLEEnum<'p>> {
    assert!(!(stop_on_where && iter.try_skip_complete_word("where")));

    // Try number
    if let Some(n) = self.lex_number(iter)? {
      return Ok(n);
    }

    // Try string
    if let Some(s) = self.lex_string(iter)? {
      return Ok(s);
    }

    let begin = iter.get_pos();

    // Try identifier - use is_unicode_identifier_part to match Scala's isUnicodeIdentifierPart
    if Self::is_unicode_identifier_part(iter.peek()) {
      let id = self
          .lex_identifier(iter)
          .expect("lexIdentifier should return Some when peek is unicode identifier part");
      return Ok(INodeLEEnum::Word(id));
    }

    // Otherwise it's a symbol
    let c = iter.advance();
    Ok(INodeLEEnum::Symbol(SymbolLE(
      RangeL::new(begin, iter.get_pos()),
      c,
    )))
  }
  /*
    // Optimize: we can make a perfect hash map beforehand based off of the u64s
    // of all the keywords. Then those can point at the pre-interned things?
    def lexAtom(iter: LexingIterator, stopOnWhere: Boolean): Result[INodeLE, IParseError] = {
      vassert(!(stopOnWhere && iter.trySkipCompleteWord("where")))

      lexNumber(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(n)) => return Ok(n)
        case Ok(None) =>
      }

      lexString(iter) match {
        case Err(e) => return Err(e)
        case Ok(Some(n)) => return Ok(n)
        case Ok(None) =>
      }

      val begin = iter.getPos()
      if (iter.peek().isUnicodeIdentifierPart) {
        Ok(vassertSome(lexIdentifier(iter)))
      } else {
        iter.advance()
        Ok(SymbolLE(RangeL(begin, iter.getPos()), iter.code.charAt(begin)))
      }
    }
  */

  /// Check if a character is a Unicode identifier part (matches Java's isUnicodeIdentifierPart)
  fn is_unicode_identifier_part(c: char) -> bool {
    // This matches Java's Character.isUnicodeIdentifierPart behavior
    c.is_alphabetic() || c.is_numeric() || c == '_' ||
        c == '\u{00B7}' || // Middle dot
        (c >= '\u{0300}' && c <= '\u{036F}') || // Combining diacritical marks
        (c >= '\u{203F}' && c <= '\u{2040}') // Undertie and character tie
  }
  /*
  MIGALLOW: Scala didn't need this because it had Character.isUnicodeIdentifierPart.
  */

  /// Lex an identifier
  fn lex_identifier(&self, iter: &mut LexingIterator) -> Option<WordLE<'p>> {
    let begin = iter.get_pos();

    // Keep eating identifier characters using isUnicodeIdentifierPart to match Scala
    while !iter.at_end() && Self::is_unicode_identifier_part(iter.peek()) {
      iter.advance();
    }

    let end = iter.get_pos();
    let word = &iter.code[begin as usize..end as usize];

    if word.is_empty() {
      None
    } else {
      Some(WordLE {
        range: RangeL::new(begin, end),
        str: self.parse_arena.intern_str(word),
      })
    }
  }
  // Lex optional ownership prefix symbols (&, &&, ^) for impl interface/struct positions.
  // Returns SymbolLE nodes for each prefix character, e.g. && becomes two SymbolLE('&').
  fn lex_impl_ownership_prefix(&self, iter: &mut LexingIterator) -> Vec<SymbolLE> {
    let mut symbols: Vec<SymbolLE> = Vec::new();
    while !iter.at_end() && (iter.peek() == '&' || iter.peek() == '^') {
      let begin = iter.get_pos();
      let c = iter.peek();
      iter.advance();
      symbols.push(SymbolLE(RangeL::new(begin, iter.get_pos()), c));
    }
    symbols
  }
  /*
  // Lex optional ownership prefix symbols (&, &&, ^) for impl interface/struct positions.
  // Returns SymbolLE nodes for each prefix character, e.g. && becomes two SymbolLE('&').
  private def lexImplOwnershipPrefix(iter: LexingIterator): Vector[SymbolLE] = {
    var symbols = Vector[SymbolLE]()
    while (!iter.atEnd() && (iter.peek() == '&' || iter.peek() == '^')) {
      val begin = iter.getPos()
      val c = iter.peek()
      iter.advance()
      symbols = symbols :+ SymbolLE(RangeL(begin, iter.getPos()), c)
    }
    symbols
  }
    def lexIdentifier(iter: LexingIterator): Option[WordLE] = {
      val begin = iter.getPos()
      // If it was a word char, then keep eating until we reach the end of the word chars.
      while (!iter.atEnd() && iter.peek().isUnicodeIdentifierPart) {
        iter.advance()
      }
      val end = iter.getPos()
      val word = iter.code.slice(begin, end)
      if (word.isEmpty) {
        None
      } else {
        Some(WordLE(RangeL(begin, end), interner.intern(StrI(word))))
      }
    }
  */

  fn _lex_region(&self, _iter: &mut LexingIterator) -> Option<ScrambleLE<'p>> {
    panic!("Unimplemented");
  }
  /*
  def lexRegion(originalIter: LexingIterator): Option[ScrambleLE] = {
    val begin = originalIter.getPos()

    val tentativeIter = originalIter.clone()

    val name =
      lexIdentifier(tentativeIter) match {
        case None => return None
        case Some(x) => x
      }

    val symbolBegin = tentativeIter.getPos()
    if (!tentativeIter.trySkip('\'')) {
      return None
    }
    val symbolEnd = tentativeIter.getPos()

    originalIter.skipTo(tentativeIter.getPos())
    val end = originalIter.getPos()

    val symbolL = SymbolLE(RangeL(symbolBegin, symbolEnd), '\'')
    val scramble = ScrambleLE(RangeL(begin, end), Vector(name, symbolL))
    return Some(scramble)
  }
  */
  /*
  */

  /// Check if we're at the end of a string
  fn lex_string_end(&self, iter: &mut LexingIterator, is_long_string: bool) -> bool {
    if iter.at_end() {
      return true;
    }

    if is_long_string {
      iter.try_skip_str("\"\"\"")
    } else {
      iter.try_skip('"')
    }
  }
  /*
    def lexStringEnd(iter: LexingIterator, isLongString: Boolean): Boolean = {
      iter.atEnd() || iter.trySkip(if (isLongString) "\"\"\"" else "\"")
    }
  */

  /// Lex a string literal (with interpolation support)
  fn lex_string(&self, iter: &mut LexingIterator) -> Result<Option<INodeLEEnum<'p>>> {
    let begin = iter.get_pos();

    let is_long_string = if iter.try_skip_str("\"\"\"") {
      true
    } else if iter.try_skip('"') {
      false
    } else {
      return Ok(None);
    };

    let mut parts = Vec::new();
    let mut string_so_far = String::new();
    let mut string_so_far_begin = iter.get_pos();

    while !self.lex_string_end(iter, is_long_string) {
      let string_so_far_end_pos = iter.get_pos();

      match self.lex_string_part(iter, begin)? {
        StringPartResult::Char(c) => {
          string_so_far.push(c);
        }
        StringPartResult::Expr(expr) => {
          if !string_so_far.is_empty() {
            parts.push(StringPart::Literal {
              range: RangeL::new(string_so_far_begin, string_so_far_end_pos),
              s: self.parse_arena.intern_str(&string_so_far),
            });
            string_so_far.clear();
          }
          parts.push(StringPart::Expr(expr));

          if !iter.try_skip('}') {
            return Err(ParseError::BadStringInterpolationEnd(iter.get_pos()));
          }
          string_so_far_begin = iter.get_pos();
        }
      }
    }

    if !string_so_far.is_empty() {
      parts.push(StringPart::Literal {
        range: RangeL::new(string_so_far_begin, iter.get_pos()),
        s: self.parse_arena.intern_str(&string_so_far),
      });
    }

    Ok(Some(INodeLEEnum::String(StringLE {
      range: RangeL::new(begin, iter.get_pos()),
      parts: self.parse_arena.alloc_slice_from_vec(parts),
    })))
  }
  /*
    def lexString(iter: LexingIterator): Result[Option[INodeLE], IParseError] = {
      val begin = iter.getPos()
      val isLongString =
        if (iter.trySkip("\"\"\"")) {
          true
        } else if (iter.trySkip("\"")) {
          false
        } else {
          return Ok(None)
        }

      val parts = new Accumulator[StringPart]()
      var stringSoFarBegin = iter.getPos()
      var stringSoFar = new StringBuilder()

      while (!lexStringEnd(iter, isLongString)) {
        val stringSoFarEndPos = iter.getPos()
        lexStringPart(iter, begin) match {
          case Err(e) => return Err(e)
          case Ok(Left(c)) => {
            stringSoFar += c
          }
          case Ok(Right(expr)) => {
            if (stringSoFar.nonEmpty) {
              parts.add(StringPartLiteral(RangeL(stringSoFarBegin, stringSoFarEndPos), stringSoFar.toString()))
              stringSoFar.clear()
            }
            parts.add(StringPartExpr(expr))
            if (!iter.trySkip('}')) {
              return Err(BadStringInterpolationEnd(iter.getPos()))
            }
            stringSoFarBegin = iter.getPos()
          }
        }
      }
      if (stringSoFar.nonEmpty) {
        parts.add(StringPartLiteral(RangeL(stringSoFarBegin, iter.getPos()), stringSoFar.toString()))
        stringSoFar.clear()
      }
      Ok(Some(StringLE(RangeL(begin, iter.getPos()), parts.buildArray())))
    }
  */
  /// Lex a part of a string (character or interpolated expression)
  fn lex_string_part(
    &self,
    iter: &mut LexingIterator,
    _string_begin_pos: i32,
  ) -> Result<StringPartResult<'p>> {
    // Handle interpolation
    if iter.try_skip_str("{\\\n") {
      // Line ending in {\
      let expr = self.lex_scramble(iter, false, false, false)?;
      return Ok(StringPartResult::Expr(expr));
    } else if iter.peek_string("{\n") {
      // Line ending in { - treat as literal
      iter.advance();
      return Ok(StringPartResult::Char('{'));
    } else if iter.try_skip('{') {
      // { with stuff after - interpolation
      let expr = self.lex_scramble(iter, false, false, false)?;
      return Ok(StringPartResult::Expr(expr));
    }

    // Handle escape sequences
    if iter.try_skip('\\') {
      if iter.try_skip('r') || iter.try_skip('\r') {
        Ok(StringPartResult::Char('\r'))
      } else if iter.try_skip('t') {
        Ok(StringPartResult::Char('\t'))
      } else if iter.try_skip('n') || iter.try_skip('\n') {
        Ok(StringPartResult::Char('\n'))
      } else if iter.try_skip('\\') {
        Ok(StringPartResult::Char('\\'))
      } else if iter.try_skip('"') {
        Ok(StringPartResult::Char('"'))
      } else if iter.try_skip('/') {
        Ok(StringPartResult::Char('/'))
      } else if iter.try_skip('{') {
        Ok(StringPartResult::Char('{'))
      } else if iter.try_skip('}') {
        Ok(StringPartResult::Char('}'))
      } else if iter.try_skip('u') {
        // Unicode escape
        let num = self
            .parse_four_digit_hex_num(iter, 0)
            .ok_or(ParseError::BadUnicodeChar(iter.get_pos()))?;
        Ok(StringPartResult::Char(
          char::from_u32(num as u32).unwrap_or('\u{FFFD}'),
        ))
      } else {
        // Unknown escape, just take next char
        Ok(StringPartResult::Char(iter.advance()))
      }
    } else {
      // Regular character
      let c = iter.advance();
      Ok(StringPartResult::Char(c))
    }
  }

  /*
    def lexStringPart(iter: LexingIterator, stringBeginPos: Int):
    Result[Either[Char, ScrambleLE], IParseError] = {
      // Normally, we interpret as an expression anything after a `{`. However, we handle
      // newlines in a special way.
      // We don't want to interpolate { if its followed by a newline, such as in
      //   """
      //     if true {
      //       3 + 4
      //     }
      //   """
      // but if they want to interpolate inside, they can use {\ like
      //   """
      //     if true {\
      //       3 + 4
      //     }
      //   """
      // which becomes
      //   """
      //     if true 7
      //   """
      // The newline is because we dont want to interpolate when its a { then a newline.
      // If they want that, then they should do {\
      if (iter.trySkip("{\\\n")) { // line ending in {\
        lexScramble(iter, false, false, false).map(Right(_))
      } else if (iter.peekString("{\n")) { // line ending in {
        iter.advance()
        Ok(Left('{'))
      } else if (iter.trySkip('{')) { // { with stuff after
        lexScramble(iter, false, false, false).map(Right(_))
      } else if (iter.trySkip('\\')) {
        if (iter.trySkip('r') || iter.trySkip('\r')) {
          Ok(Left('\r'))
        } else if (iter.trySkip('t')) {
          Ok(Left('\t'))
        } else if (iter.trySkip('n') || iter.trySkip('\n')) {
          Ok(Left('\n'))
        } else if (iter.trySkip('\\')) {
          Ok(Left('\\'))
        } else if (iter.trySkip('"')) {
          Ok(Left('\"'))
        } else if (iter.trySkip('/')) {
          Ok(Left('/'))
        } else if (iter.trySkip('{')) {
          Ok(Left('{'))
        } else if (iter.trySkip('}')) {
          Ok(Left('}'))
        } else if (iter.trySkip('u')) {
          val num =
            parseFourDigitHexNum(iter) match {
              case None => {
                return Err(BadUnicodeChar(iter.getPos()))
              }
              case Some(x) => x
            }
          Ok(Left(num.toChar))
        } else {
          Ok(Left(iter.advance()))
        }
      } else {
        val c = iter.advance()
        Ok(Left(c))
      }
    }
  */

  /// Parse a four-digit hexadecimal number
  pub fn parse_four_digit_hex_num(&self, iter: &mut LexingIterator, _offset: usize) -> Option<i32> {
    let str = iter.peek_exact(4)?;

    for c in str.chars() {
      if !c.is_ascii_hexdigit() {
        return None;
      }
    }

    let str_owned = str.to_string();

    // Advance past the 4 characters
    for _ in 0..4 {
      iter.advance();
    }

    i32::from_str_radix(&str_owned, 16).ok()
  }
  /*
    def parseFourDigitHexNum(iter: LexingIterator): Option[Int] = {
      val str =
        iter.peek(4) match {
          case None => return None
          case Some(s) => s
        }
      var i = 0;
      while (i < 4) {
        // MIGALLOW: Scala has a bug here, in that it advances the iterator even if we end up
        // returning None.
        val c = iter.advance()
        if (c >= '0' && c <= '9') {
        } else if (c >= 'a' && c <= 'f') {
        } else if (c >= 'A' && c <= 'F') {
        } else {
          return None
        }
        i = i + 1;
      }
      Some(Integer.parseInt(str, 16)) // MIGALLOW: parseInt vs from_str_radix
    }
  */

  /// Lex a number (integer or float)
  fn lex_number(&self, original_iter: &mut LexingIterator) -> Result<Option<INodeLEEnum<'p>>> {
    let begin = original_iter.get_pos();

    // Check if preceded by a dot (for array access like arr.2.1)
    let is_name = original_iter.position >= 1
        && original_iter.code.chars().nth(original_iter.position - 1) == Some('.');

    let mut tentative_iter = original_iter.clone();
    let negative = tentative_iter.try_skip('-');

    let peeked = tentative_iter.peek();
    if !peeked.is_ascii_digit() {
      return Ok(None);
    }

    original_iter.skip_to(tentative_iter.position);
    let iter = original_iter;

    let mut digits_consumed = 0;
    let mut integer: i64 = 0;

    while !iter.at_end() {
      let c = iter.peek();
      if c.is_ascii_digit() {
        integer = integer * 10 + ((c as i64) - ('0' as i64));
        digits_consumed += 1;
        iter.advance();
      } else {
        break;
      }
    }

    assert!(digits_consumed > 0);

    // Check for range operator (..)
    if iter.peek_string("..") {
      return Ok(Some(INodeLEEnum::ParsedInteger(ParsedIntegerLE {
        range: RangeL::new(begin, iter.get_pos()),
        value: integer,
        bits: None,
      })));
    }

    // If this is a name (array/tuple access), stop here
    if is_name {
      return Ok(Some(INodeLEEnum::ParsedInteger(ParsedIntegerLE {
        range: RangeL::new(begin, iter.get_pos()),
        value: integer,
        bits: None,
      })));
    }

    // Try to parse as float
    if iter.try_skip('.') {
      let mut mantissa = 0.0;
      let mut digit_multiplier = 1.0;

      while !iter.at_end() {
        let c = iter.peek();
        if c.is_ascii_digit() {
          digit_multiplier *= 0.1;
          mantissa += ((c as i64) - ('0' as i64)) as f64 * digit_multiplier;
          iter.advance();
        } else {
          break;
        }
      }

      if iter.try_skip('f') {
        panic!("Float type suffix 'f' not yet implemented (vimpl)");
      }

      let result = (integer as f64 + mantissa) * (if negative { -1.0 } else { 1.0 });
      return Ok(Some(INodeLEEnum::ParsedDouble(ParsedDoubleLE {
        range: RangeL::new(begin, iter.get_pos()),
        value: result,
        bits: None,
      })));
    }

    // Check for integer type suffix (i32, i64, etc.)
    let bits = if iter.try_skip('i') {
      let mut bits = 0i64;
      while !iter.at_end() {
        let c = iter.peek();
        if c.is_ascii_digit() {
          bits = bits * 10 + ((c as i64) - ('0' as i64));
          iter.advance();
        } else {
          break;
        }
      }
      assert!(
        bits > 0,
        "Integer type suffix 'i' must be followed by a number"
      );
      Some(bits)
    } else {
      None
    };

    let result = integer * (if negative { -1 } else { 1 });

    Ok(Some(INodeLEEnum::ParsedInteger(ParsedIntegerLE {
      range: RangeL::new(begin, iter.get_pos()),
      value: result,
      bits,
    })))
  }
  /*
    // This is here in the lexer because it's a little awkward to identify things with i like 7i32.
    // But it's only a minor reason, we can move it to the parser if we want.
    def lexNumber(originalIter: LexingIterator): Result[Option[IParsedNumberLE], IParseError] = {
      val begin = originalIter.getPos()

      // This is so if we have an expression like arr.2.1 to get an element in
      // a 2D array, we don't interpret that 2.1 as a float.
      val isName =
        originalIter.position >= 1 && originalIter.code(originalIter.position - 1) == '.'

      val tentativeIter = originalIter.clone()

      val negative = tentativeIter.trySkip("-")

      val peeked = tentativeIter.peek()
      if (peeked < '0' || peeked > '9') {
        return Ok(None)
      }

      originalIter.skipTo(tentativeIter.position)
      val iter = originalIter

      var digitsConsumed = 0
      var integer = 0L

      while ({
        val c = iter.peek()
        if (c >= '0' && c <= '9') {
          integer = integer * 10L + (c.toInt - '0')
          digitsConsumed += 1
          iter.advance()
          true
        } else {
          false
        }
      }) {}
      vassert(digitsConsumed > 0)

      if (iter.peekString("..")) {
        // This is followed by the range operator, so just stop here.
        Ok(Some(ParsedIntegerLE(RangeL(begin, iter.getPos()), integer, None)))
      } else if (isName) {
        // This is a name for accessing in a tuple or array, so stop here.
        Ok(Some(ParsedIntegerLE(RangeL(begin, iter.getPos()), integer, None)))
      } else {
        if (iter.trySkip('.')) {
          var mantissa = 0.0
          var digitMultiplier = 1.0

          while ( {
            val c = iter.peek()
            if (c >= '0' && c <= '9') {
              digitMultiplier = digitMultiplier * 0.1
              mantissa = mantissa + (c.toInt - '0') * digitMultiplier
              iter.advance()
              true
            } else {
              false
            }
          }) {}

          if (iter.trySkip("f")) {
            vimpl()
          }

          val result = (integer + mantissa) * (if (negative) -1 else 1)
          Ok(Some(ParsedDoubleLE(RangeL(begin, iter.getPos()), result, None)))
        } else {
          val bits =
            if (iter.trySkip("i")) {
              var bits = 0L
              while ( {
                val c = iter.peek()
                if (c >= '0' && c <= '9') {
                  bits = bits * 10 + (c.toInt - '0')
                  iter.advance()
                  true
                } else {
                  false
                }
              }) {}
              vassert(bits > 0)
              Some(bits)
            } else {
              None
            }

          val result = integer * (if (negative) -1 else 1)

          Ok(Some(ParsedIntegerLE(RangeL(begin, iter.getPos()), result, bits)))
        }
      }
    }
  }
  */
}
/*
//return Err(BadFunctionAfterParam(iter.getPos()))
//return Err(BadFunctionBodyError(iter.position))
//return Err(BadStartOfStatementError(iter.getPos()))
*/
