use crate::compile_options::GlobalOptions;
use crate::keywords::Keywords;
use crate::lexing::ast::*;
use crate::lexing::errors::FailedParse;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::expression_parser::ExpressionParser;
use crate::parsing::parse_utils::{parse_region as parse_region_shared, try_skip_past_keyword_while};
use crate::parsing::pattern_parser::PatternParser;
use crate::parsing::expression_parser::ScrambleIterator;
use crate::parsing::templex_parser::TemplexParser;
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::utils::code_hierarchy::{FileCoordinateMap, IPackageResolver};
use std::collections::HashMap;
use crate::parsing::parse_and_explore;
use crate::parsing::parsed_loader;
use crate::parsing::vonifier::ParserVonifier;
use crate::von::printer::VonPrinter;
use crate::parse_arena::ParseArena;

/*
package dev.vale.parsing

import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast._
import dev.vale.parsing.templex.TemplexParser
import dev.vale._
import dev.vale.lexing._
import dev.vale.parsing.Parser.{parsePrefixingRegion, parseRegion}
import dev.vale.parsing.ast._
import dev.vale.von.{JsonSyntax, VonPrinter}

import scala.collection.immutable.{List, Map}
import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
import scala.util.matching.Regex
*/

type ParseResult<T> = Result<T, ParseError>;

/// Main parser coordinating all parsing operations
/// Matches Scala's Parser class
pub struct Parser<'p, 'ctx> {
  // VV: crate::
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  pub templex_parser: TemplexParser<'p, 'ctx>,
  pub pattern_parser: PatternParser<'p, 'ctx>,
  pub expression_parser: ExpressionParser<'p, 'ctx>,
}
/*
class Parser(interner: Interner, keywords: Keywords, opts: GlobalOptions) {
  val templexParser = new TemplexParser(interner, keywords)
  val patternParser = new PatternParser(interner, keywords, templexParser)
  val expressionParser = new ExpressionParser(interner, keywords, opts, patternParser, templexParser)
*/
impl<'p, 'ctx> Parser<'p, 'ctx>
where
    'p: 'ctx,
{
  /// Parse a single generic parameter
  fn parse_generic_parameter(
    &self,
    mut iter: ScrambleIterator<'p, '_>,
  ) -> ParseResult<GenericParameterP<'p>> {
    let range = iter.range();

    // Parse optional prefixing region
    let maybe_coord_region = self.parse_prefixing_region(&mut iter)?;

    // Parse the main parameter
    let (name, maybe_type, attributes) = match self.parse_region(&mut iter)? {
      None => {
        // Regular rune parameter
        let name = match iter.peek_cloned() {
          Some(INodeLEEnum::Word(WordLE { range, str })) => {
            let result = NameP(range, str);
            iter.advance();
            result
          }
          _ => return Err(ParseError::BadRuneNameError(iter.get_pos())),
        };

        let type_begin = iter.get_pos();
        let maybe_rune_type = self.templex_parser.parse_rune_type(&mut iter)?;

        let maybe_type = maybe_rune_type.map(|tyype| GenericParameterTypeP {
          range: RangeL(type_begin, iter.get_prev_end_pos()),
          tyype,
        });

        let maybe_attrs = if iter.try_skip_word(self.keywords.imm).is_some() {
          vec![IRuneAttributeP::ImmutableRuneAttribute(RangeL(
            iter.get_prev_end_pos(),
            iter.get_prev_end_pos(),
          ))]
        } else {
          vec![]
        };

        (name, maybe_type, maybe_attrs)
      }
      Some(region) => {
        // Region parameter
        let attributes = if let Some(range) = iter.try_skip_word(self.keywords.ro) {
          vec![IRuneAttributeP::ReadOnlyRegionRuneAttribute(range)]
        } else if let Some(range) = iter.try_skip_word(self.keywords.rw) {
          vec![IRuneAttributeP::ReadWriteRegionRuneAttribute(range)]
        } else if let Some(range) = iter.try_skip_word(self.keywords.additive) {
          vec![IRuneAttributeP::AdditiveRegionRuneAttribute(range)]
        } else if let Some(range) = iter.try_skip_word(self.keywords.imm) {
          vec![IRuneAttributeP::ImmutableRegionRuneAttribute(range)]
        } else {
          vec![]
        };

        let tyype = GenericParameterTypeP {
          range: RangeL(range.begin(), iter.get_prev_end_pos()),
          tyype: ITypePR::RegionType,
        };

        let region_name = region
          .name
          .ok_or(ParseError::BadRuneNameError(iter.get_pos()))?;

        (region_name, Some(tyype), attributes)
      }
    };

    // Parse optional default value
    let maybe_default = if iter.try_skip_symbol('=') {
      Some(self.templex_parser.parse_templex(&mut iter)?)
    } else {
      None
    };

    assert!(iter.at_end());

    Ok(GenericParameterP::<'p> {
      range,
      name,
      maybe_type,
      coord_region: maybe_coord_region,
      attributes: self.parse_arena.alloc_slice_from_vec(attributes),
      maybe_default,
    })
  }
  /*
    private[parsing] def parseGenericParameter(iter: ScrambleIterator):
    Result[GenericParameterP, IParseError] = {
      val range = iter.range

      val maybeCoordRegion =
        parsePrefixingRegion(iter) match {
          case Err(x) => return Err(x)
          case Ok(maybeRegion) => maybeRegion
        }

      val (name, maybeType, attributes) =
        parseRegion(iter) match {
          case Err(x) => return Err(x)
          case Ok(None) => {
            val name =
              iter.peek() match {
                case Some(WordLE(range, str)) => {
                  iter.advance()
                  NameP(range, str)
                }
                case _ => return Err(BadRuneNameError(iter.getPos()))
              }

            val typeBegin = iter.getPos()
            val maybeRuneType =
              templexParser.parseRuneType(iter) match {
                case Err(e) => return Err(e)
                case Ok(Some(x)) => Some(GenericParameterTypeP(RangeL(typeBegin, iter.getPrevEndPos()), x))
                case Ok(None) => None
              }

            val maybeAttrs =
              iter.trySkipWord(keywords.imm) match {
                case Some(range) => Vector(ImmutableRuneAttributeP(range))
                case None => Vector()
              }

            (name, maybeRuneType, maybeAttrs)
          }
          case Ok(Some(region)) => {
            val attributes =
                (iter.trySkipWord(keywords.ro) match {
                  case Some(range) => Vector(ReadOnlyRegionRuneAttributeP(range))
                  case None => {
                    iter.trySkipWord(keywords.rw) match {
                      case Some(range) => Vector(ReadWriteRegionRuneAttributeP(range))
                      case None => {
                        iter.trySkipWord(keywords.additive) match {
                          case Some(range) => Vector(AdditiveRegionRuneAttributeP(range))
                          case None => {
                            iter.trySkipWord(keywords.imm) match {
                              case Some(range) => Vector(ImmutableRegionRuneAttributeP(range))
                              case None => Vector()
                            }
                          }
                        }
                      }
                    }
                  }
                })

            val tyype =
              GenericParameterTypeP(RangeL(range.begin, iter.getPrevEndPos()), RegionTypePR)

            val regionName =
              region.name match {
                case None => return Err(BadRuneNameError(iter.getPos()))
                case Some(z) => z
              }

            (regionName, Some(tyype), attributes)
          }
        }

      val maybeDefaultPT =
        if (iter.trySkipSymbol('=')) {
          templexParser.parseTemplex(iter) match {
            case Err(e) => return Err(e)
            case Ok(x) => Some(x)
          }
        } else {
          None
        }

      vassert(iter.atEnd)

      Ok(GenericParameterP(range, NameP(name.range, name.str), maybeType, maybeCoordRegion, attributes, maybeDefaultPT))
    }
  */

  pub fn new(
    parse_arena: &'ctx ParseArena<'p>,
    keywords: &'ctx Keywords<'p>,
  ) -> Self {
    let templex_parser = TemplexParser::new(parse_arena, keywords);
    let pattern_parser = PatternParser::new(parse_arena, keywords);
    let expression_parser = ExpressionParser::new(parse_arena, keywords);

    Parser {
      parse_arena,
      keywords,
      templex_parser,
      pattern_parser,
      expression_parser,
    }
  }

  /// Parse a complete file from lexer output
  pub fn parse_file(&self, file: FileL<'p>) -> ParseResult<FileP<'p>> {
    let FileL {
      denizens,
      comment_ranges,
    } = file;

    let mut parsed_denizens = Vec::new();

    for denizen in denizens {
      let parsed = self.parse_denizen(*denizen)?;
      parsed_denizens.push(parsed);
    }

    let empty_str = self.parse_arena.intern_str("");
    let empty_package = self.parse_arena.intern_package_coordinate(empty_str, &[]);
    let empty_file = self.parse_arena.intern_file_coordinate(empty_package, "");

    Ok(FileP {
      file_coord: empty_file,
      comments_ranges: comment_ranges,
      denizens: self.parse_arena.alloc_slice_from_vec(parsed_denizens),
    })
  }

  /// Parse a top-level denizen
  pub fn parse_denizen(&self, denizen: IDenizenL<'p>) -> ParseResult<IDenizenP<'p>> {
    match denizen {
      IDenizenL::TopLevelFunction(func) => {
        let parsed = self.parse_function(func, false)?;
        Ok(IDenizenP::TopLevelFunction(parsed))
      }
      IDenizenL::TopLevelStruct(struct_) => {
        let parsed = self.parse_struct(struct_)?;
        Ok(IDenizenP::TopLevelStruct(parsed))
      }
      IDenizenL::TopLevelInterface(interface) => {
        let parsed = self.parse_interface(interface)?;
        Ok(IDenizenP::TopLevelInterface(parsed))
      }
      IDenizenL::TopLevelImpl(impl_) => {
        let parsed = self.parse_impl(impl_)?;
        Ok(IDenizenP::TopLevelImpl(parsed))
      }
      IDenizenL::TopLevelExportAs(export) => {
        let parsed = self.parse_export_as(export)?;
        Ok(IDenizenP::TopLevelExportAs(parsed))
      }
      IDenizenL::TopLevelImport(import) => {
        let parsed = self.parse_import(import)?;
        Ok(IDenizenP::TopLevelImport(parsed))
      }
    }
  }

  /// Parse generic parameters from angled brackets
  fn parse_identifying_runes(&self, node: &AngledLE<'p>) -> ParseResult<GenericParametersP<'p>> {
    let iter = ScrambleIterator::new(&node.contents);
    let parts = iter.split_on_symbol(',', false);

    let mut params = Vec::new();
    for part in parts {
      let param = self.parse_generic_parameter(part)?;
      params.push(param);
    }

    Ok(GenericParametersP {
      range: node.range,
      params: self.parse_arena.alloc_slice_from_vec(params),
    })
  }
  /*
    private[parsing] def parseIdentifyingRunes(node: AngledLE):
    Result[GenericParametersP, IParseError] = {
      val runesP =
        U.map[ScrambleIterator, GenericParameterP](
          new ScrambleIterator(node.contents).splitOnSymbol(',', false),
          inner => {
          parseGenericParameter(inner) match {
            case Err(e) => return Err(e)
            case Ok(x) => x
          }
        })

      Ok(GenericParametersP(node.range, runesP.toVector))
    }
  */

  /// Parse struct member
  fn parse_struct_member(&self, iter: &mut ScrambleIterator<'p, '_>) -> ParseResult<IStructContent<'p>> {
    let begin = iter.get_pos();

    // Parse name (can be a word or integer for variadic)
    let name = match iter.peek_cloned() {
      Some(INodeLEEnum::ParsedInteger(ParsedIntegerLE { range, value, .. })) => {
        let result: NameP<'_> = NameP(range, self.parse_arena.intern_str(&value.to_string()));
        iter.advance();
        result
      }
      _ => match iter.next_word() {
        Some(WordLE { range, str }) => NameP(range, str),
        None => return Err(ParseError::BadStructMember(iter.get_pos())),
      },
    };

    // Parse variability (! means varying)
    let variability = if iter.try_skip_symbol('!') {
      VariabilityP::Varying
    } else {
      VariabilityP::Final
    };

    // Check for variadic (..)
    let variadic = matches!(
      iter.peek2_cloned(),
      (
        Some(INodeLEEnum::Symbol(SymbolLE(_, '.'))),
        Some(INodeLEEnum::Symbol(SymbolLE(_, '.')))
      )
    );
    if variadic {
      iter.advance();
      iter.advance();
    }

    // Parse type
    let tyype = self.templex_parser.parse_templex(iter)?;

    if variadic {
      if name.str() != self.keywords.underscore {
        return Err(ParseError::VariadicStructMemberHasName(iter.get_pos()));
      }

      Ok(IStructContent::VariadicStructMember(
        VariadicStructMemberP {
          range: RangeL(begin, iter.get_prev_end_pos()),
          variability,
          tyype,
        },
      ))
    } else {
      Ok(IStructContent::NormalStructMember(NormalStructMemberP::<'p> {
        range: RangeL(begin, iter.get_prev_end_pos()),
        name,
        variability,
        tyype,
      }))
    }
  }
  /*
    private[parsing] def parseStructMember(
      iter: ScrambleIterator):
    Result[IStructContent, IParseError] = {
      val begin = iter.getPos()

      val name =
        iter.peek() match {
          case Some(ParsedIntegerLE(range, int, _)) => {
            // This is just temporary until we add proper variadics again, see TAVWG.
            iter.advance()
            NameP(range, interner.intern(StrI(int.toString)))
          }
          case _ => {
            iter.nextWord() match {
              case None => return Err(BadStructMember(iter.getPos()))
              case Some(WordLE(range, str)) => NameP(range, str)
            }
          }
        }

      val variability = if (iter.trySkipSymbol('!')) VaryingP else FinalP

      val variadic =
        iter.peek2() match {
          case (Some(SymbolLE(_, '.')), Some(SymbolLE(_, '.'))) => {
            iter.advance()
            iter.advance()
            true
          }
          case _ => false
        }

      val tyype =
        templexParser.parseTemplex(iter) match {
          case Err(e) => return Err(e)
          case Ok(x) => x
        }

      if (variadic) {
        if (name.str != keywords.UNDERSCORE) {
          return Err(VariadicStructMemberHasName(iter.getPos()))
        }

        Ok(VariadicStructMemberP(RangeL(begin, iter.getPrevEndPos()), variability, tyype))
      } else {
        Ok(NormalStructMemberP(RangeL(begin, iter.getPrevEndPos()), name, variability, tyype))
      }
    }
  */

  /// Parse a struct definition
  pub fn parse_struct(&self, struct_l: StructL<'p>) -> ParseResult<StructP<'p>> {
    let StructL {
      range: struct_range,
      name: name_l,
      attributes: attributes_l,
      mutability: maybe_mutability_l,
      identifying_runes: maybe_identifying_runes_l,
      template_rules: maybe_template_rules_l,
      contents_range,
      members: members_l,
      methods: methods_l,
    } = struct_l;

    // Parse identifying runes
    let maybe_identifying_runes = maybe_identifying_runes_l
      .as_ref()
      .map(|runes| self.parse_identifying_runes(runes))
      .transpose()?;

    // Parse template rules
    let maybe_template_rules = maybe_template_rules_l
      .as_ref()
      .map(|rules_scramble| {
        let iter = ScrambleIterator::new(&rules_scramble);
        let parts = iter.split_on_symbol(',', false);
        let mut rules = Vec::new();
        for mut part in parts {
          rules.push(self.templex_parser.parse_rule(&mut part)?);
        }
        Ok(TemplateRulesP {
          range: rules_scramble.range,
          rules: self.parse_arena.alloc_slice_from_vec(rules),
        })
      })
      .transpose()?;

    // Parse attributes
    let mut attributes = Vec::new();
    for attr_l in attributes_l {
      attributes.push(self.parse_attribute(*attr_l)?);
    }

    // Parse mutability
    let maybe_mutability = maybe_mutability_l
      .map(|mut_l| {
        let mut iter = ScrambleIterator::new(&mut_l);
        self.templex_parser.parse_templex(&mut iter)
      })
      .transpose()?;

    // Parse struct members
    let mut contents_vec = Vec::new();
    for member_l in members_l {
      let mut iter = ScrambleIterator::new(member_l);
      contents_vec.push(self.parse_struct_member(&mut iter)?);
    }
    for method_l in methods_l {
      contents_vec.push(IStructContent::StructMethod(self.parse_function(*method_l, true)?));
    }

    let members = StructMembersP {
      range: contents_range,
      contents: self.parse_arena.alloc_slice_from_vec(contents_vec),
    };

    Ok(StructP::<'p> {
      range: struct_range,
      name: self.to_name(name_l),
      attributes: self.parse_arena.alloc_slice_from_vec(attributes),
      mutability: maybe_mutability,
      identifying_runes: maybe_identifying_runes,
      template_rules: maybe_template_rules,
      maybe_default_region_rune: None,
      body_range: contents_range,
      members,
    })
  }

  /*
    def parseStruct(functionL: StructL):
    Result[StructP, IParseError] = {
      Profiler.frame(() => {
        val StructL(structRange, nameL, attributesL, maybeMutabilityL, maybeIdentifyingRunesL, maybeTemplateRulesL, contentsRange, membersL, methodsL) = functionL

        val maybeIdentifyingRunes =
          maybeIdentifyingRunesL.map(userSpecifiedIdentifyingRunes => {
            parseIdentifyingRunes(userSpecifiedIdentifyingRunes) match {
              case Err(cpe) => return Err(cpe)
              case Ok(x) => x
            }
          })


        val maybeTemplateRulesP =
          maybeTemplateRulesL.map(templateRulesScramble => {
            val elementsPR =
              U.map[ScrambleIterator, IRulexPR](
                new ScrambleIterator(templateRulesScramble).splitOnSymbol(',', false),
                ruleIter => {
                  templexParser.parseRule(ruleIter) match {
                    case Err(e) => return Err(e)
                    case Ok(x) => x
                  }
                })
            TemplateRulesP(templateRulesScramble.range, elementsPR.toVector)
          })

        val attributesP =
          U.map[IAttributeL, IAttributeP](
            attributesL,
            attributeL => {
              parseAttribute(attributeL) match {
                case Err(e) => return Err(e)
                case Ok(x) => x
              }
            })

        val maybeMutabilityP =
          maybeMutabilityL.map(returnTypeL => {
            val scramble =
              returnTypeL match {
                case s @ ScrambleLE(_, _) => s
                case other => ScrambleLE(other.range, Vector(other))
              }
            templexParser.parseTemplex(new ScrambleIterator(scramble, 0, scramble.elements.length)) match {
              case Err(e) => return Err(e)
              case Ok(x) => x
            }
          })

        val membersP =
          StructMembersP(
            contentsRange,
            U.map[ScrambleLE, IStructContent](
              membersL,
              member => {
                parseStructMember(new ScrambleIterator(member)) match {
                  case Err(e) => return Err(e)
                  case Ok(x) => x
                }
              }).toVector ++
              U.map[FunctionL, IStructContent](
                methodsL,
                methodL => {
                  parseFunction(methodL, true) match {
                    case Err(e) => return Err(e)
                    case Ok(x) => StructMethodP(x)
                  }
                }).toVector)

        val maybeDefaultRegionP = vregionmut(None)

        val struct =
          StructP(
            structRange,
            toName(nameL),
            attributesP.toVector,
            maybeMutabilityP,
            maybeIdentifyingRunes,
            maybeTemplateRulesP,
            maybeDefaultRegionP,
            contentsRange,
            membersP)
        Ok(struct)
      })
    }
  */

  /// Parse an interface definition
  pub fn parse_interface(&self, interface_l: InterfaceL<'p>) -> ParseResult<InterfaceP<'p>> {
    let InterfaceL {
      range: interface_range,
      name: name_l,
      attributes: attributes_l,
      mutability: maybe_mutability_l,
      maybe_identifying_runes: maybe_identifying_runes_l,
      template_rules: maybe_template_rules_l,
      body_range,
      members: methods,
    } = interface_l;

    // Parse identifying runes
    let maybe_identifying_runes = maybe_identifying_runes_l
      .as_ref()
      .map(|runes| self.parse_identifying_runes(runes))
      .transpose()?;

    // Parse template rules
    let maybe_template_rules = maybe_template_rules_l
      .as_ref()
      .map(|rules_scramble| {
        let iter = ScrambleIterator::new(&rules_scramble);
        let parts = iter.split_on_symbol(',', false);
        let mut rules = Vec::new();
        for mut part in parts {
          rules.push(self.templex_parser.parse_rule(&mut part)?);
        }
        Ok(TemplateRulesP {
          range: rules_scramble.range,
          rules: self.parse_arena.alloc_slice_from_vec(rules),
        })
      })
      .transpose()?;

    // Parse attributes
    let mut attributes = Vec::new();
    for attr_l in attributes_l {
      attributes.push(self.parse_attribute(*attr_l)?);
    }

    // Parse mutability
    let maybe_mutability = maybe_mutability_l
      .map(|mut_l| {
        let mut iter = ScrambleIterator::new(&mut_l);
        self.templex_parser.parse_templex(&mut iter)
      })
      .transpose()?;

    // Parse interface methods
    // Interface methods are in a citizen (interface), so is_in_citizen = true
    let mut members_vec = Vec::new();
    for method_l in methods {
      members_vec.push(self.parse_function(*method_l, true)?);
    }

    Ok(InterfaceP {
      range: interface_range,
      name: self.to_name(name_l),
      attributes: self.parse_arena.alloc_slice_from_vec(attributes),
      mutability: maybe_mutability,
      maybe_identifying_runes,
      template_rules: maybe_template_rules,
      maybe_default_region_rune: None,
      body_range,
      members: self.parse_arena.alloc_slice_from_vec(members_vec),
    })
  }

  /*
    def parseInterface(interfaceL: InterfaceL):
    Result[InterfaceP, IParseError] = {
      Profiler.frame(() => {
        val InterfaceL(interfaceRange, nameL, attributesL, maybeMutabilityL, maybeIdentifyingRunesL, maybeTemplateRulesL, bodyRange, methodsL) = interfaceL

        val maybeIdentifyingRunes =
          maybeIdentifyingRunesL.map(userSpecifiedIdentifyingRunes => {
            parseIdentifyingRunes(userSpecifiedIdentifyingRunes) match {
              case Err(cpe) => return Err(cpe)
              case Ok(x) => x
            }
          })


        val maybeTemplateRulesP =
          maybeTemplateRulesL.map(templateRulesScramble => {
            val elementsPR =
              U.map[ScrambleIterator, IRulexPR](
                new ScrambleIterator(templateRulesScramble).splitOnSymbol(',', false),
                ruleIter => {
                  templexParser.parseRule(ruleIter) match {
                    case Err(e) => return Err(e)
                    case Ok(x) => x
                  }
                })
            TemplateRulesP(templateRulesScramble.range, elementsPR.toVector)
          })

        val attributesP =
          U.map[IAttributeL, IAttributeP](
            attributesL,
            attributeL => {
              parseAttribute(attributeL) match {
                case Err(e) => return Err(e)
                case Ok(x) => x
              }
            })

        val maybeMutabilityP =
          maybeMutabilityL.map(returnTypeL => {
            val scramble =
              returnTypeL match {
                case s @ ScrambleLE(_, _) => s
                case other => ScrambleLE(other.range, Vector(other))
              }
            templexParser.parseTemplex(new ScrambleIterator(scramble, 0, scramble.elements.length)) match {
              case Err(e) => return Err(e)
              case Ok(x) => x
            }
          })

        val membersP =
            U.map[FunctionL, FunctionP](
              methodsL,
              methodL => {
                parseFunction(methodL, true) match {
                  case Err(e) => return Err(e)
                  case Ok(x) => x
                }
              })

        val maybeDefaultRegionP = vregionmut(None)

        val interface =
          InterfaceP(
            interfaceRange,
            toName(nameL),
            attributesP.toVector,
            maybeMutabilityP,
            maybeIdentifyingRunes,
            maybeTemplateRulesP,
            maybeDefaultRegionP,
            bodyRange,
            membersP.toVector)
        Ok(interface)
      })

  //    if (!iter.trySkip("interface")) {
  //      return Ok(None)
  //    }
  //
  //    val name =
  //      Parser.parseTypeName(iter) match {
  //        case None => return Err(BadStructName(iter.getPos()))
  //        case Some(x) => x
  //      }
  //
  //    val maybeIdentifyingRunes =
  //      parseIdentifyingRunes(iter) match {
  //        case Err(e) => vwat()
  //        case Ok(x) => x
  //      }
  //
  //    val (mutabilityRange, maybeMutability, maybeTemplateRules) =
  //      parseCitizenSuffix(iter) match {
  //        case Err(e) => return Err(e)
  //        case Ok((a, b, c)) => (a, b, c)
  //      }
  //
  //
  //
  //    val contentsBegin = iter.getPos()
  //
  //    if (!iter.trySkip("\\{")) {
  //      return Err(BadStructContentsBegin(iter.getPos()))
  //    }
  //
  //    val methods = ArrayBuffer[FunctionP]()
  //
  //    while (!Parser.atEnd(iter, StopBeforeCloseBrace)) {
  //
  //      parseDenizen(iter) match {
  //        case Err(e) => return Err(e)
  //        case Ok(Some(TopLevelFunctionP(f))) => methods += f
  //        case Ok(Some(other)) => {
  //          return Err(UnexpectedDenizen(iter.getPos(), other))
  //        }
  //        case Ok(None) => return Err(BadInterfaceMember(iter.getPos()))
  //      }
  //    }
  //
  //
  //
  //    if (!iter.trySkip("\\}")) {
  //      return Err(BadStructContentsEnd(iter.getPos()))
  //    }
  //
  //    val contentsEnd = iter.getPos()
  //
  //    val interface =
  //      ast.InterfaceP(
  //        ast.RangeL(begin, iter.getPos()),
  //        name,
  //        attributes.toVector,
  //        maybeMutability.getOrElse(ast.MutabilityPT(mutabilityRange, MutableP)),
  //        maybeIdentifyingRunes,
  //        maybeTemplateRules,
  //        methods.toVector)
  //    Ok(Some(interface))
    }
  */

  /// Parse an impl block
  /// Mirrors Parser.parseImpl in Parser.scala lines 397-461
  pub fn parse_impl(&self, impl_l: ImplL<'p>) -> ParseResult<ImplP<'p>> {
    let ImplL {
      range: impl_range,
      identifying_runes: maybe_identifying_runes_l,
      template_rules: maybe_template_rules_l,
      struct_: struct_l,
      interface: interface_l,
      attributes: attributes_l,
    } = impl_l;

    // Parse identifying runes if present
    let maybe_identifying_runes = match maybe_identifying_runes_l {
      Some(user_specified_identifying_runes) => {
        Some(self.parse_identifying_runes(&user_specified_identifying_runes)?)
      }
      None => None,
    };

    // Parse template rules if present
    let maybe_template_rules_p = match maybe_template_rules_l {
      Some(template_rules_scramble) => {
        let iter = ScrambleIterator::new(&template_rules_scramble);
        let rule_iters = iter.split_on_symbol(',', false);
        let mut elements_pr = Vec::new();

        for rule_iter in rule_iters {
          elements_pr.push(self.templex_parser.parse_rule(&mut rule_iter.clone())?);
        }

        Some(TemplateRulesP {
          range: template_rules_scramble.range,
          rules: self.parse_arena.alloc_slice_from_vec(elements_pr),
        })
      }
      None => None,
    };

    // Parse struct templex if present
    let struct_p = match struct_l {
      None => None,
      Some(struct_l) => {
        let mut iter = ScrambleIterator::new(&struct_l);
        Some(self.templex_parser.parse_templex(&mut iter)?)
      }
    };

    // Parse interface templex
    let mut iter = ScrambleIterator::new(&interface_l);
    let interface_p = self.templex_parser.parse_templex(&mut iter)?;

    // Parse attributes
    let mut attributes_p = Vec::new();
    for attribute_l in attributes_l {
      attributes_p.push(self.parse_attribute(*attribute_l)?);
    }

    Ok(ImplP {
      range: impl_range,
      generic_params: maybe_identifying_runes,
      template_rules: maybe_template_rules_p,
      struct_: struct_p,
      interface: interface_p,
      attributes: self.parse_arena.alloc_slice_from_vec(attributes_p),
    })
  }
  /*
    def parseImpl(functionL: ImplL):
    Result[ImplP, IParseError] = {
      Profiler.frame(() => {
        val ImplL(implRange, maybeIdentifyingRunesL, maybeTemplateRulesL, structL, interfaceL, attributesL) = functionL

        val maybeIdentifyingRunes =
          maybeIdentifyingRunesL.map(userSpecifiedIdentifyingRunes => {
            parseIdentifyingRunes(userSpecifiedIdentifyingRunes) match {
              case Err(cpe) => return Err(cpe)
              case Ok(x) => x
            }
          })

        val maybeTemplateRulesP =
          maybeTemplateRulesL.map(templateRulesScramble => {
            val elementsPR =
              U.map[ScrambleIterator, IRulexPR](
                new ScrambleIterator(templateRulesScramble).splitOnSymbol(',', false),
                ruleIter => {
                  templexParser.parseRule(ruleIter) match {
                    case Err(e) => return Err(e)
                    case Ok(x) => x
                  }
                })
            TemplateRulesP(templateRulesScramble.range, elementsPR.toVector)
          })

        val structP =
          structL match {
            case None => None
            case Some(structL) => {
              templexParser.parseTemplex(new ScrambleIterator(structL)) match {
                case Err(e) => return Err(e)
                case Ok(x) => Some(x)
              }
            }
          }

        val interfaceP =
          templexParser.parseTemplex(new ScrambleIterator(interfaceL)) match {
            case Err(e) => return Err(e)
            case Ok(x) => x
          }

        val attributesP =
          U.map[IAttributeL, IAttributeP](
            attributesL,
            attributeL => {
              parseAttribute(attributeL) match {
                case Err(e) => return Err(e)
                case Ok(x) => x
              }
            })

        val impl =
          ImplP(
            implRange,
            maybeIdentifyingRunes,
            maybeTemplateRulesP,
            structP,
            interfaceP,
            attributesP.toVector)
        Ok(impl)
      })
    }
  */

  /// Helper to convert WordLE to NameP
  fn to_name(&self, word: WordLE<'p>) -> NameP<'p> {
    NameP(word.range, word.str)
  }

/*
  val export = interner.intern(StrI("export"))
*/

  /// Parse an export-as declaration
  /// Mirrors Parser.parseExportAs in Parser.scala lines 465-497
  pub fn parse_export_as(&self, export_l: ExportAsL<'p>) -> ParseResult<ExportAsP<'p>> {
    let mut iter = ScrambleIterator::new(&export_l.contents);

    // Try to find "as" keyword and get everything before it
    // Mirrors ParseUtils.trySkipPastKeywordWhile in ParseUtils.scala lines 77-102
    let exportee = {
      let mut scouting_iter = iter.clone();
      let mut found_as = false;
      let mut before_iter = iter.clone();

      while scouting_iter.has_next() {
        // Check if we should continue (not at semicolon)
        let should_continue = match scouting_iter.peek_cloned() {
          None => false,
          Some(INodeLEEnum::Symbol(SymbolLE(_, ';'))) => false,
          _ => true,
        };

        if !should_continue {
          break;
        }

        // Check if this is the "as" keyword
        if let Some(INodeLEEnum::Word(WordLE { str, .. })) = scouting_iter.peek_cloned() {
          if str == self.keywords.r#as {
            // Found "as"! Create iterator for everything before it
            before_iter.end = scouting_iter.index;
            iter.skip_to(&scouting_iter);
            iter.advance(); // Skip past "as"
            found_as = true;
            break;
          }
        }

        scouting_iter.advance();
      }

      if !found_as {
        return Err(ParseError::BadExportAs(iter.get_pos()));
      }

      // Parse the templex from everything before "as"
      self.templex_parser.parse_templex(&mut before_iter)?
    };

    // Get the name after "as"
    let name = match iter.peek_cloned() {
      None => return Err(ParseError::BadExportEnd(iter.get_pos())),
      Some(INodeLEEnum::Word(word)) => self.to_name(word.clone()),
      _ => return Err(ParseError::BadExportEnd(iter.get_pos())),
    };

    Ok(ExportAsP {
      range: export_l.range,
      struct_: exportee,
      exported_name: name,
    })
  }
  /*
    def parseExportAs(
      expoort: ExportAsL):
    Result[ExportAsP, IParseError] = {
      val iter = new ScrambleIterator(expoort.contents)

      val exportee =
        ParseUtils.trySkipPastKeywordWhile(
          iter,
          keywords.as,
          iter => iter.peek() match {
            case None => false
            case Some(SymbolLE(range, ';')) => false
            case _ => true
          }) match {
          case None => return Err(BadExportAs(iter.getPos()))
          case Some((asKeyword, beforeAsIter)) => {
            val templex =
              templexParser.parseTemplex(beforeAsIter) match {
                case Err(e) => return Err(e)
                case Ok(x) => x
              }
            templex
          }
        }

      val name =
        iter.peek() match {
          case None => return Err(BadExportEnd(iter.getPos()))
          case Some(WordLE(range, str)) => NameP(range, str)
        }

      Ok(ast.ExportAsP(expoort.range, exportee, name))
    }
  */

  /// Parse an import declaration
  /// Mirrors Parser.parseImport in Parser.scala lines 499-516
  pub fn parse_import(&self, import_l: ImportL<'p>) -> ParseResult<ImportP<'p>> {
    let ImportL {
      range,
      module_name: module_name_l,
      package_steps: package_steps_l,
      importee_name: importee_name_l,
    } = import_l;

    let module_name_p = self.to_name(module_name_l);

    let mut package_steps_p = Vec::new();
    for step in package_steps_l {
      package_steps_p.push(self.to_name(*step));
    }

    let importee_name_p = self.to_name(importee_name_l);

    Ok(ImportP {
      range,
      module_name: module_name_p,
      package_steps: self.parse_arena.alloc_slice_from_vec(package_steps_p),
      importee_name: importee_name_p,
    })
  }
  /*
    def parseImport(
      importL: ImportL):
    Result[ImportP, IParseError] = {
      val ImportL(range, moduleNameL, packageStepsL, importeeNameL) = importL

      val WordLE(moduleNameRange, moduleNameStr) = moduleNameL
      val moduleNameP = NameP(moduleNameRange, moduleNameStr)

      val packageStepsP =
        U.map[WordLE, NameP](packageStepsL, { case WordLE(moduleNameRange, moduleNameStr) =>
          NameP(moduleNameRange, moduleNameStr)
        })

      val WordLE(importeeNameRange, importeeNameStr) = importeeNameL
      val importeeNameP = NameP(importeeNameRange, importeeNameStr)

      Ok(ImportP(range, moduleNameP, packageStepsP.toVector, importeeNameP))
    }
  */

  /// Parse an attribute
  fn parse_attribute(&self, attr_l: IAttributeL<'p>) -> ParseResult<IAttributeP<'p>> {
    match attr_l {
      IAttributeL::WeakableAttribute(range) => {
        Ok(IAttributeP::WeakableAttribute(WeakableAttributeP { range }))
      }
      IAttributeL::SealedAttribute(range) => {
        Ok(IAttributeP::SealedAttribute(SealedAttributeP { range }))
      }
      IAttributeL::MacroCall {
        range,
        inclusion,
        name,
      } => Ok(IAttributeP::MacroCall(MacroCallP {
        range,
        inclusion: match inclusion {
          IMacroInclusionL::CallMacro => IMacroInclusionP::CallMacro,
          IMacroInclusionL::DontCallMacro => IMacroInclusionP::DontCallMacro,
        },
        name: self.to_name(name),
      })),
      IAttributeL::AbstractAttribute(range) => {
        Ok(IAttributeP::AbstractAttribute(AbstractAttributeP { range }))
      }
      IAttributeL::ExternAttribute {
        range,
        maybe_custom_name,
      } => {
        // Mirrors Parser.scala parseAttribute handling of ExternAttribute
        match maybe_custom_name {
          None => Ok(IAttributeP::ExternAttribute(ExternAttributeP { range })),
          Some(parend) => {
            // extern("name") becomes BuiltinAttribute
            let iter = ScrambleIterator::new(&parend.contents);
            if let Some(INodeLEEnum::String(string_le)) = iter.peek_cloned() {
              // Extract the string value from the parts
              // For a simple string like "bork", there should be one Literal part
              if string_le.parts.len() == 1 {
                if let StringPart::Literal { s, .. } = &string_le.parts[0] {
                  let name = NameP(string_le.range, *s);
                  return Ok(IAttributeP::BuiltinAttribute(BuiltinAttributeP {
                    range,
                    generator_name: name,
                  }));
                }
              }
              Err(ParseError::BadExternAttribute(range.begin()))
            } else {
              Err(ParseError::BadExternAttribute(range.begin()))
            }
          }
        }
      }
      IAttributeL::ExportAttribute(range) => {
        Ok(IAttributeP::ExportAttribute(ExportAttributeP { range }))
      }
      IAttributeL::PureAttribute(range) => Ok(IAttributeP::PureAttribute(PureAttributeP { range })),
      IAttributeL::AdditiveAttribute(range) => {
        Ok(IAttributeP::AdditiveAttribute(AdditiveAttributeP { range }))
      }
      IAttributeL::LinearAttribute(range) => {
        Ok(IAttributeP::LinearAttribute(LinearAttributeP { range }))
      }
    }
  }
  /*
    def parseAttribute(attrL: IAttributeL):
    Result[IAttributeP, IParseError] = {
      attrL match {
        case AbstractAttributeL(range) => Ok(AbstractAttributeP(range))
        case ExternAttributeL(range, None) => Ok(ExternAttributeP(range))
        case LinearAttributeL(range) => Ok(LinearAttributeP(range))
        case ExternAttributeL(range, Some(maybeName)) => {
          val name =
            maybeName.contents match {
              case ScrambleLE(_, Vector(StringLE(_, Vector(StringPartLiteral(range, s))))) => {
                NameP(range, interner.intern(StrI(s)))
              }
              case _ => vfail("Bad builtin extern!")
            }
          Ok(BuiltinAttributeP(range, name))
        }
        case ExportAttributeL(range) => Ok(ExportAttributeP(range))
        case PureAttributeL(range) => Ok(PureAttributeP(range))
        case AdditiveAttributeL(range) => Ok(AdditiveAttributeP(range))
        case WeakableAttributeL(range) => Ok(WeakableAttributeP(range))
        case SealedAttributeL(range) => Ok(SealedAttributeP(range))
        case MacroCallL(range, inclusion, name) => {
          Ok(
            MacroCallP(
              range,
              inclusion match {
                case CallMacroL => CallMacroP
                case DontCallMacroL => DontCallMacroP
              },
              toName(name)))
        }
      }
    }
  */

  /// Parse a function
  /// Mirrors Parser.parseFunction in Parser.scala lines 552-654
  pub fn parse_function(
    &self,
    func_l: FunctionL<'p>,
    is_in_citizen: bool,
  ) -> ParseResult<FunctionP<'p>> {
    let FunctionL {
      range: func_range_l,
      header: header_l,
      body: maybe_body_l,
    } = func_l;

    let FunctionHeaderL {
      range: header_range_l,
      name: name_l,
      attributes: attributes_l,
      maybe_user_specified_identifying_runes: maybe_identifying_runes_l,
      params: params_l,
      trailing_details: original_trailing_details_l,
    } = header_l;

    // Parse identifying runes if present
    let maybe_identifying_runes = match maybe_identifying_runes_l {
      Some(user_specified_identifying_runes) => {
        Some(self.parse_identifying_runes(&user_specified_identifying_runes)?)
      }
      None => None,
    };

    // Parse parameters
    let mut params_p_vec = Vec::new();
    let params_iter = ScrambleIterator::new(&params_l.contents);
    let param_iters = params_iter.split_on_symbol(',', false);

    // Use field splitting to borrow parsers separately
    let Self {
      pattern_parser,
      templex_parser,
      ..
    } = self;

    for (index, pattern_iter) in param_iters.into_iter().enumerate() {
      let mut iter = pattern_iter.clone();
      params_p_vec.push(pattern_parser.parse_parameter(
        &mut iter,
        templex_parser,
        index,
        is_in_citizen,
        true,
        false,
      )?);
    }

    let params_p = ParamsP {
      range: params_l.range,
      params: self.parse_arena.alloc_slice_from_vec(params_p_vec),
    };

    // Parse trailing details to extract return type, where clause, and default region
    let (trailing_details_with_return_and_where, maybe_default_region) =
      self.parse_body_default_region(original_trailing_details_l.clone());

    // Mirrors Parser.scala lines 582-618
    // TODO: simplify this. It's really just trying to split on "where".
    let mut return_and_where_iter =
      ScrambleIterator::new(&trailing_details_with_return_and_where);

    // Mirrors Parser.scala lines 584-589
    let (maybe_return_iter, return_end_pos, maybe_rules_iter) =
      match try_skip_past_keyword_while(&mut return_and_where_iter, self.keywords.r#where, |it| {
        it.has_next()
      }) {
        None => {
          // No "where" was found. Use everything remaining.
          (
            Some(return_and_where_iter.clone()),
            trailing_details_with_return_and_where.range.end(),
            None,
          )
        }
        Some((_, return_iter)) => (
          Some(return_iter),
          return_and_where_iter.scramble.range.end(),
          Some(return_and_where_iter.clone()),
        ),
      };

    // Mirrors Parser.scala lines 591-602
    let return_begin_pos = trailing_details_with_return_and_where.range.begin();
    let maybe_return_type_p = if let Some(mut return_iter) = maybe_return_iter {
      if return_iter.has_next() {
        Some(self.templex_parser.parse_templex(&mut return_iter)?)
      } else {
        None
      }
    } else {
      None
    };

    // Mirrors Parser.scala line 603
    let return_p = FunctionReturnP {
      range: RangeL(return_begin_pos, return_end_pos),
      ret_type: maybe_return_type_p,
    };

    // Mirrors Parser.scala lines 605-618
    let maybe_rules_p = maybe_rules_iter
      .map(|rules_iter| {
        let _begin = rules_iter.get_pos();
        let rule_iters = rules_iter.split_on_symbol(',', false);
        let mut rules = Vec::new();
        for mut templex_iter in rule_iters {
          match self.templex_parser.parse_rule(&mut templex_iter) {
            Err(e) => return Err(e),
            Ok(x) => rules.push(x),
          }
        }
        Ok(TemplateRulesP {
          range: rules_iter.scramble.range,
          rules: self.parse_arena.alloc_slice_from_vec(rules),
        })
      })
      .transpose()?;

    // Parse attributes
    let mut attributes_p = Vec::new();
    for attribute_l in attributes_l {
      attributes_p.push(self.parse_attribute(*attribute_l)?);
    }

    let header = FunctionHeaderP {
      range: header_range_l,
      name: Some(self.to_name(name_l)),
      attributes: self.parse_arena.alloc_slice_from_vec(attributes_p),
      generic_parameters: maybe_identifying_runes,
      template_rules: maybe_rules_p,
      params: Some(params_p),
      ret: return_p,
    };

    // Parse body if present
    // Use field splitting to borrow parsers separately
    let Self {
      expression_parser,
      templex_parser,
      pattern_parser,
      ..
    } = self;

    let body_p = match maybe_body_l {
      Some(body_l) => {
        let FunctionBodyL { body: block_l } = body_l;
        let statements_p =
          expression_parser.parse_block(&block_l, templex_parser, pattern_parser)?;
        Some(&*self.parse_arena.alloc(BlockPE {
          range: block_l.range,
          maybe_pure: None,
          maybe_default_region: maybe_default_region,
          inner: &*self.parse_arena.alloc(statements_p),
        }))
      }
      None => None,
    };

    Ok(FunctionP {
      range: func_range_l,
      header,
      body: body_p,
    })
  }
  /*
    def parseFunction(functionL: FunctionL, isInCitizen: Boolean):
    Result[FunctionP, IParseError] = {
      Profiler.frame(() => {
        val FunctionL(funcRangeL, headerL, maybeBodyL) = functionL
        val FunctionHeaderL(headerRangeL, nameL, attributesL, maybeIdentifyingRunesL, paramsL, originalTrailingDetailsL) = headerL

        val maybeIdentifyingRunes =
          maybeIdentifyingRunesL.map(userSpecifiedIdentifyingRunes => {
            parseIdentifyingRunes(userSpecifiedIdentifyingRunes) match {
              case Err(cpe) => return Err(cpe)
              case Ok(x) => x
            }
          })

        val paramsP =
          ParamsP(
            paramsL.range,
            U.mapWithIndex[ScrambleIterator, ParameterP](
              new ScrambleIterator(paramsL.contents).splitOnSymbol(',', false),
              (index, patternIter) => {
                patternParser.parseParameter(patternIter, index, isInCitizen, true, false) match {
                  case Err(e) => return Err(e)
                  case Ok(x) => x
                }
              }))

        val trailingDetailsWithReturnAndWhereAndDefaultRegion = originalTrailingDetailsL
        val (trailingDetailsWithReturnAndWhere, maybeDefaultRegion) =
          parseBodyDefaultRegion(trailingDetailsWithReturnAndWhereAndDefaultRegion)

        // TODO: simplify this. It's really just trying to split on "where".
        val returnAndWhereIter = new ScrambleIterator(trailingDetailsWithReturnAndWhere)
        val (maybeReturnIter, returnEndPos, maybeRulesIter) =
          ParseUtils.trySkipPastKeywordWhile(
            returnAndWhereIter, keywords.where, it => it.hasNext) match {
            case None => (Some(returnAndWhereIter), returnAndWhereIter.scramble.range.end, None) // No "where" was found. Use everything remaining.
            case Some((_, returnIter)) => (Some(returnIter), returnIter.scramble.range.end, Some(returnAndWhereIter))
          }

        val returnBeginPos = returnAndWhereIter.scramble.range.begin
        val maybeReturnTypeP =
          maybeReturnIter.flatMap(returnIter => {
            if (returnIter.hasNext) {
              templexParser.parseTemplex(returnIter) match {
                case Err(e) => return Err(e)
                case Ok(x) => Some(x)
              }
            } else {
              None
            }
          })
        maybeReturnTypeP match {
          case Some(FuncPT(range, _, _, _, _)) =>
            return Err(FuncBoundWithoutWhere(range.begin))
          case _ =>
        }
        val returnP = FunctionReturnP(RangeL(returnBeginPos, returnEndPos), maybeReturnTypeP)

        val maybeRulesP =
          maybeRulesIter.map(rulesIter => {
            val begin = rulesIter.getPos()
            val rules =
              U.map[ScrambleIterator, IRulexPR](
              rulesIter.splitOnSymbol(',', false),
              templexL => {
                templexParser.parseRule(templexL) match {
                  case Err(e) => return Err(e)
                  case Ok(x) => x
                }
              })
            TemplateRulesP(rulesIter.scramble.range, rules)
          })

        val attributesP =
          U.map[IAttributeL, IAttributeP](
            attributesL,
            attributeL => {
              parseAttribute(attributeL) match {
                case Err(e) => return Err(e)
                case Ok(x) => x
              }
            })


        val header =
          FunctionHeaderP(
            headerL.range,
            Some(toName(nameL)),
            attributesP.toVector,
            maybeIdentifyingRunes,
            maybeRulesP,
            Some(paramsP),
            returnP)

        val bodyP =
          maybeBodyL.map(bodyL => {
            val FunctionBodyL(blockL) = bodyL
            val statementsP =
              expressionParser.parseBlock(blockL) match {
                case Err(err) => return Err(err)
                case Ok(result) => result
              }
            BlockPE(blockL.range, None, maybeDefaultRegion, statementsP)
          })

        Ok(FunctionP(funcRangeL, header, bodyP))
      })
    }
  */

  /// Parse body default region from trailing details
  /// Mirrors Parser.parseBodyDefaultRegion in Parser.scala lines 660-691
  fn parse_body_default_region(
    &self,
    input_scramble: ScrambleLE<'p>,
  ) -> (ScrambleLE<'p>, Option<RegionRunePT<'p>>) {
    if input_scramble.elements.len() < 2 {
      return (input_scramble, None);
    }

    let _last_two = &input_scramble.elements[input_scramble.elements.len() - 2..];

    // Mirrors Parser.scala parseBodyDefaultRegion lines 669-679
    // The Scala code checks:
    // - Vector(SymbolLE(_, '\'')) => anonymous region (ONE element: just ')
    // - Vector(WordLE(...), SymbolLE(_, '\'')) => named region (TWO elements: word then ')
    // So we need to check the length of elements, not just the last two
    let default_region = match input_scramble.elements.len() {
      1 => {
        // Check if it's just a single apostrophe
        match &*input_scramble.elements[0] {
          INodeLEEnum::Symbol(SymbolLE(symbol_range, '\'')) => Some(RegionRunePT {
            range: *symbol_range,
            name: None,
          }),
          _ => None,
        }
      }
      n if n >= 2 => {
        // Check if the last two elements are word then apostrophe
        let last_two = &input_scramble.elements[n - 2..n];
        match (&*last_two[0], &*last_two[1]) {
          (
            INodeLEEnum::Word(WordLE {
              range: word_range,
              str: region_name,
            }),
            INodeLEEnum::Symbol(SymbolLE(symbol_range, '\'')),
          ) => {
            let range = RangeL(word_range.begin(), symbol_range.end());
            Some(RegionRunePT {
              range,
            name: Some(NameP(*word_range, *region_name)),
            })
          }
          _ => None,
        }
      }
      _ => None,
    };

    if default_region.is_none() {
      return (input_scramble, None);
    }

    // Remove the elements that made up the default region
    let elements_to_remove = match input_scramble.elements.len() {
      1 => 1, // Just the apostrophe
      _ => 2, // Word and apostrophe
    };

    let preceding_elements = &input_scramble.elements
      [..input_scramble.elements.len() - elements_to_remove];

    let preceding_elements_range = if preceding_elements.is_empty() {
      RangeL(input_scramble.range.begin(), input_scramble.range.begin())
    } else {
      RangeL(
        preceding_elements.first().unwrap().range().begin(),
        preceding_elements.last().unwrap().range().end(),
      )
    };

    let preceding_elements_scramble = ScrambleLE {
      range: preceding_elements_range,
      elements: preceding_elements,
    };

    (preceding_elements_scramble, default_region)
  }
  /*
    // Returns:
    // - A scramble of everything before the default region. If there's no default region, it's the
    //   same as the input scramble.
    // - The default region if it existed.
    private def parseBodyDefaultRegion(inputScramble: ScrambleLE):
    (ScrambleLE, Option[RegionRunePT]) = {
      if (inputScramble.elements.size < 2) {
        return (inputScramble, None)
      }

      val lastTwo =
        inputScramble.elements.slice(inputScramble.elements.length - 2, inputScramble.elements.length)

      val defaultRegion =
        lastTwo match {
          case Vector(SymbolLE(symbolRange, '\'')) => {
            RegionRunePT(symbolRange, None)
          }
          case Vector(WordLE(wordRange, regionName), SymbolLE(symbolRange, '\'')) => {
            val range = RangeL(wordRange.begin, symbolRange.end)
            RegionRunePT(range, Some(NameP(wordRange, regionName)))
          }
          case _ => return (inputScramble, None)
        }

      val precedingElements = inputScramble.elements.slice(0, inputScramble.elements.length - 2)
      val precedingElementsRange =
        if (precedingElements.isEmpty) {
          RangeL(inputScramble.range.begin, inputScramble.range.begin)
        } else {
          RangeL(precedingElements.head.range.begin, precedingElements.last.range.end)
        }
      val precedingElementsScramble = ScrambleLE(precedingElementsRange, precedingElements)

      (precedingElementsScramble, Some(defaultRegion))
    }
  */
}
/*
  def toName(wordL: WordLE): NameP = {
    val WordLE(range, s) = wordL
    NameP(range, s)
  }
}
*/

// From Parser.scala lines 699-854: ParserCompilation class
// 'p: interner
// 'p: parsed arena (parsed data outlives 'p; interner outlives parsed)
// Arena is passed in by reference, caller owns it
pub struct ParserCompilation<'p, 'ctx> {
  opts: GlobalOptions,
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  packages_to_build: Vec<&'p PackageCoordinate<'p>>,
  package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
  code_map_cache: Option<FileCoordinateMap<'p, String>>,
  vpst_map_cache: Option<FileCoordinateMap<'p, String>>,
  parseds_cache: Option<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>>,
}
impl<'p, 'ctx> ParserCompilation<'p, 'ctx>
where
  'p: 'ctx,
{
  /*
  class ParserCompilation(
    opts: GlobalOptions,
    interner: Interner,
    keywords: Keywords,
    packagesToBuild: Vector[PackageCoordinate],
    packageToContentsResolver: IPackageResolver[Map[String, String]]
  ) {
    val parser = new Parser(interner, keywords, opts)
  */

  // From Parser.scala lines 699-706
  pub fn new(
    opts: GlobalOptions,
    parse_arena: &'ctx ParseArena<'p>,
    keywords: &'ctx Keywords<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
  ) -> Self {
    ParserCompilation {
      opts,
      parse_arena,
      keywords,
      packages_to_build,
      package_to_contents_resolver,
      code_map_cache: None,
      vpst_map_cache: None,
      parseds_cache: None,
    }
  }

  // From Parser.scala lines 708-773: loadAndParse
  // From Parser.scala lines 708-773: loadAndParse
  fn load_and_parse(
    &self,
    needed_packages: &[&'p PackageCoordinate<'p>],
    resolver: &dyn IPackageResolver<'p, HashMap<String, String>>,
  ) -> Result<
    (
      FileCoordinateMap<'p, String>,
      FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>,
    ),
    FailedParse<'p>,
  > {
    // From Parser.scala line 712: Check for duplicates
    let unique_packages: std::collections::HashSet<_> = needed_packages.iter().collect();
    assert!(
      unique_packages.len() == needed_packages.len(),
      "Duplicate modules in: {:?}",
      needed_packages
    );

    // From Parser.scala lines 714-715
    let mut found_code_map: FileCoordinateMap<'p, String> = FileCoordinateMap::<String>::new();
    let mut parsed_map: FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)> =
      FileCoordinateMap::new();

    // From Parser.scala lines 717-740: Load .vpst files directly
    for package_coord in needed_packages {
      if let Some(filepath_to_code) = resolver.resolve(package_coord) {
        for (filepath, _code) in filepath_to_code {
          if filepath.ends_with(".vpst") {
            panic!("ParsedLoader not yet implemented - see Parser.scala lines 724-735. Need to load .vpst file: {}", filepath);
          }
        }
      }
    }

    // From Parser.scala lines 742-749: Create resolver that filters out .vpst files
    struct ValeOnlyResolver<'p, 'ctx> {
      inner: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    }
    impl<'p, 'ctx> IPackageResolver<'p, HashMap<String, String>> for ValeOnlyResolver<'p, 'ctx> {
      fn resolve(
        &self,
        package_coord: &'p PackageCoordinate<'p>,
      ) -> Option<HashMap<String, String>> {
        self.inner.resolve(package_coord).map(|filepath_to_code| {
          filepath_to_code
            .into_iter()
            .filter(|(filepath, _)| filepath.ends_with(".vale"))
            .collect()
        })
      }
    }
    let vale_only_resolver = ValeOnlyResolver { inner: resolver };

    // From Parser.scala lines 751-770: Process .vale files through lex/parse flow
    let parser = Parser::new(self.parse_arena, self.keywords);
    parse_and_explore::parse_and_explore(
            self.parse_arena,
            self.keywords,
            self.opts.clone(),
            &parser,
            needed_packages.to_vec(),
            &vale_only_resolver,
            |_file_coord, _code, _imports, denizen| denizen,
            |file_coord: &'p FileCoordinate<'p>, code, comment_ranges, denizens: Vec<IDenizenP<'p>>| {
                // From Parser.scala lines 756-766
                found_code_map.put(file_coord, code.to_string());
                let comments_slice = self.parse_arena.alloc_slice_copy(comment_ranges);
                let denizens_slice = self.parse_arena.alloc_slice_from_vec(denizens);
                let file = FileP {
                    file_coord: file_coord,
                    comments_ranges: comments_slice,
                    denizens: denizens_slice,
                };
                
                // From Parser.scala lines 759-764: Sanity check
                if self.opts.sanity_check {

                    let json = VonPrinter::new().print(&ParserVonifier::vonify_file(&file));
                    let loaded_file = parsed_loader::load(self.parse_arena, &json).unwrap_or_else(|e| {
                        panic!(
                            "Sanity check failed to load generated VPST for {}: {:?}",
                            file_coord.filepath, e
                        )
                    });
                    let second_json =
                        VonPrinter::new().print(&ParserVonifier::vonify_file(&loaded_file));
                    assert_eq!(
                        json, second_json,
                        "Sanity check round-trip mismatch for {}",
                        file_coord.filepath
                    );
                }
                
                // From Parser.scala line 766
                parsed_map.put(file_coord, (file, comment_ranges.to_vec()));
            },
        ).map_err(|e| e)?;

    // From Parser.scala line 772
    Ok((found_code_map, parsed_map))
  }
  /*
    def loadAndParse(
      neededPackages: Vector[PackageCoordinate],
      resolver: IPackageResolver[Map[String, String]]):
    Result[(FileCoordinateMap[String], FileCoordinateMap[(FileP, Vector[RangeL])]), FailedParse] = {
      vassert(neededPackages.size == neededPackages.distinct.size, "Duplicate modules in: " + neededPackages.mkString(", "))

      val foundCodeMap = new FileCoordinateMap[String]()
      val parsedMap = new FileCoordinateMap[(FileP, Vector[RangeL])]()

      // First, load all .vpst files directly, bypassing lexing and parsing
      neededPackages.foreach(packageCoord => {
        resolver.resolve(packageCoord) match {
          case None => // Package not found, will be handled by ParseAndExplore
          case Some(filepathToCode) => {
            filepathToCode.foreach({ case (filepath, code) =>
              if (filepath.endsWith(".vpst")) {
                val fileCoord = interner.intern(FileCoordinate(packageCoord, filepath))
                foundCodeMap.put(fileCoord, code)

                // Load the .vpst file using ParsedLoader
                val parsedLoader = new ParsedLoader(interner)
                parsedLoader.load(code) match {
                  case Err(e) => return Err(FailedParse(code, fileCoord, e))
                  case Ok(fileP) => {
                    // .vpst files don't have comment ranges tracked by Rust parser yet
                    parsedMap.put(fileCoord, (fileP, Vector.empty))
                  }
                }
              }
            })
          }
        }
      })

      // Create a resolver that filters out .vpst files (already processed above)
      val valeOnlyResolver = new IPackageResolver[Map[String, String]] {
        override def resolve(packageCoord: PackageCoordinate): Option[Map[String, String]] = {
          resolver.resolve(packageCoord).map(filepathToCode => {
            filepathToCode.filter({ case (filepath, _) => filepath.endsWith(".vale") })
          })
        }
      }

      // Now process .vale files through the normal lex/parse flow
      ParseAndExplore.parseAndExplore[IDenizenP, Unit](
        interner, keywords, opts, parser, packagesToBuild.toVector, valeOnlyResolver,
        (fileCoord, code, imports, denizen) => denizen,
        (fileCoord, code, commentRanges, denizens) => {
          foundCodeMap.put(fileCoord, code)
          val file = FileP(fileCoord, commentRanges.buildArray(), denizens.buildArray())

          if (opts.sanityCheck) {
            val json = new VonPrinter(JsonSyntax, 120).print(ParserVonifier.vonifyFile(file))
            val loadedFile = new ParsedLoader(interner).load(json).getOrDie()
            val secondJson = new VonPrinter(JsonSyntax, 120).print(ParserVonifier.vonifyFile(loadedFile))
            vassert(json == secondJson)
          }

          parsedMap.put(fileCoord, (file, commentRanges.buildArray().toVector))
        }) match {
        case Err(e) => return Err(e)
        case Ok(_) =>
      }

      Ok((foundCodeMap, parsedMap))
    }
  */

  /*
    var codeMapCache: Option[FileCoordinateMap[String]] = None
    var vpstMapCache: Option[FileCoordinateMap[String]] = None
    var parsedsCache: Option[FileCoordinateMap[(FileP, Vector[RangeL])]] = None
  */
  // From Parser.scala lines 779-784: getCodeMap
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.get_parseds()?;
    Ok(self.code_map_cache.clone().unwrap())
  }
  /*
    def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = {
      getParseds() match {
        case Ok(_) => Ok(codeMapCache.get)
        case Err(e) => Err(e)
      }
    }
    def expectCodeMap(): FileCoordinateMap[String] = {
      vassertSome(codeMapCache)
    }
  */

  // From Parser.scala lines 785-787: expectCodeMap
  pub fn expect_code_map(&self) -> FileCoordinateMap<'p, String> {
    self
      .code_map_cache
      .clone()
      .expect("code_map_cache should be populated")
  }

  // From Parser.scala lines 789-816: getParseds
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
    if let Some(ref parseds) = self.parseds_cache {
      return Ok(parseds.clone());
    }

    let packages = self.packages_to_build.clone();
    let (code_map, program_p_map) =
      self.load_and_parse(&packages, self.package_to_contents_resolver)?;

    self.code_map_cache = Some(code_map);
    self.parseds_cache = Some(program_p_map);
    Ok(self.parseds_cache.clone().unwrap())
  }
  /*
    def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = {
      parsedsCache match {
        case Some(parseds) => Ok(parseds)
        case None => {
          // Also build the "" module, which has all the builtins
          val (codeMap, programPMap) =
            loadAndParse(packagesToBuild, packageToContentsResolver) match {
              case Ok((codeMap, programPMap)) => (codeMap, programPMap)
              case Err(e) => return Err(e)
            }
          codeMapCache = Some(codeMap)
          parsedsCache = Some(programPMap)
          Ok(parsedsCache.get)
        }
      }
    }
  */

  // From Parser.scala lines 818-826: expectParseds
  pub fn expect_parseds(&mut self) -> FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)> {
    match self.get_parseds() {
      Err(FailedParse {
        code: _code,
        file_coord: _file_coord,
        error,
      }) => {
        panic!(
          "Parse error: {:?} - need ParseErrorHumanizer.humanize - see Parser.scala lines 818-826",
          error
        )
      }
      Ok(x) => x,
    }
  }
  /*
    def expectParseds(): FileCoordinateMap[(FileP, Vector[RangeL])] = {
      getParseds() match {
        case Err(FailedParse(code, fileCoord, err)) => {
          vfail(ParseErrorHumanizer.humanize(SourceCodeUtils.humanizeFile(fileCoord), code, err))
        }
        case Ok(x) => x
      }
    }
  */

  // From Parser.scala lines 829-846: getVpstMap
  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    if let Some(ref vpst) = self.vpst_map_cache {
      return Ok(vpst.clone());
    }

    let _parseds = self.get_parseds()?;
    panic!("ParserCompilation.get_vpst_map not yet fully implemented - need to vonify and print. See Parser.scala lines 829-846")
  }
  /*
    def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = {
      vpstMapCache match {
        case Some(vpst) => Ok(vpst)
        case None => {
          getParseds() match {
            case Err(e) => Err(e)
            case Ok(parseds) => {
              Ok(
                parseds.map({ case (fileCoord, (programP, commentRanges)) =>
                  val von = ParserVonifier.vonifyFile(programP)
                  val json = new VonPrinter(JsonSyntax, 120).print(von)
                  json
                }))
            }
          }
        }
      }
    }
  */

  // From Parser.scala lines 849-851: expectVpstMap
  pub fn expect_vpst_map(&mut self) -> FileCoordinateMap<'p, String> {
    self.get_vpst_map().expect("getVpstMap should succeed")
  }
  /*
    def expectVpstMap(): FileCoordinateMap[String] = {
      getVpstMap().getOrDie()
    }
  */
}
/*
}
object Parser {
*/

impl<'p, 'ctx> Parser<'p, 'ctx>
where
    'p: 'ctx,
{
  /// Parse optional prefixing region (e.g., `'a`)
  fn parse_prefixing_region(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<RegionRunePT<'p>>> {
    let mut tentative_iter = original_iter.clone();

    let region = match parse_region_shared(&mut tentative_iter)? {
      Some(region) => {
        // Check if the next token immediately follows (no gap)
        match tentative_iter.peek_cloned() {
          Some(next) if next.range().begin() == region.range.end() => region,
          _ => return Ok(None),
        }
      }
      None => return Ok(None),
    };

    original_iter.skip_to(&tentative_iter);
    Ok(Some(region))
  }
  /*
    // A prefixing region is one that appears before something else to modify it, like t'T.
    def parsePrefixingRegion(originalIter: ScrambleIterator): Result[Option[RegionRunePT], IParseError] = {
      val tentativeIter = originalIter.clone()

      val region =
        parseRegion(tentativeIter) match {
          case Err(x) => return Err(x)
          case Ok(Some(region)) => {
            tentativeIter.peek() match {
              case Some(next) if next.range.begin == region.range.end => {
                region
              }
              case _ => return Ok(None)
            }
          }
          case _ => return Ok(None)
        }

      originalIter.skipTo(tentativeIter)

      Ok(Some(region))
    }
  */

  /// Parse optional region marker - delegates to shared parse_region (Parser.parseRegion in Scala)
  fn parse_region(
    &self,
    original_iter: &mut ScrambleIterator<'p, '_>,
  ) -> ParseResult<Option<RegionRunePT<'p>>> {
    parse_region_shared(original_iter)
  }
  /*
    def parseRegion(originalIter: ScrambleIterator): Result[Option[RegionRunePT], IParseError] = {
      val tentativeIter = originalIter.clone()

      val runeBegin = tentativeIter.getPos()
      val maybeRune =
        if (tentativeIter.trySkipSymbol('\'')) {
          // Anonymous region, in other words an isolate
          None
        } else {
          val regionRune =
            tentativeIter.nextWord() match {
              case None => return Ok(None)
              case Some(r) => r
            }

          if (!tentativeIter.trySkipSymbol('\'')) {
            return Ok(None)
          }

          Some(regionRune)
        }
      val runeEnd = tentativeIter.getPrevEndPos()

      originalIter.skipTo(tentativeIter)

      val range = RangeL(runeBegin, runeEnd)
      return Ok(Some(RegionRunePT(range, maybeRune.map(z => NameP(RangeL(runeBegin, runeEnd), z.str)))))
    }
  */
}
/*
}
 */
