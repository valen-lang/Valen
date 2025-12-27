use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::ast::*;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::scramble_iterator::ScrambleIterator;
use crate::parsing::expression_parser::ExpressionParser;
use crate::parsing::templex_parser::TemplexParser;
use crate::parsing::pattern_parser::PatternParser;
use std::sync::{Arc, Mutex};

type ParseResult<T> = Result<T, ParseError>;

/// Main parser coordinating all parsing operations
/// Matches Scala's Parser class
pub struct Parser {
    interner: Arc<Mutex<Interner>>,
    keywords: Arc<Keywords>,
    pub templex_parser: TemplexParser,
    pub pattern_parser: PatternParser,
    pub expression_parser: ExpressionParser,
}

impl Parser {
    pub fn new(interner: Arc<Mutex<Interner>>, keywords: Arc<Keywords>) -> Self {
        let templex_parser = TemplexParser::new(interner.clone(), keywords.clone());
        let pattern_parser = PatternParser::new(interner.clone(), keywords.clone());
        let expression_parser = ExpressionParser::new(keywords.clone());

        Parser {
            interner,
            keywords,
            templex_parser,
            pattern_parser,
            expression_parser,
        }
    }

    /// Try to skip past a keyword, returning the portion before it
    /// Mirrors trySkipPastKeywordWhile in ParseUtils.scala lines 77-102
    fn try_skip_past_keyword_while<F>(
        iter: &mut ScrambleIterator,
        keyword: &crate::interner::StrI,
        continue_while: F,
    ) -> Option<(WordLE, ScrambleIterator)>
    where
        F: Fn(&ScrambleIterator) -> bool,
    {
        // Mirrors ParseUtils.scala line 82
        let mut scouting_iter = iter.clone();
        
        // Mirrors ParseUtils.scala line 83
        while continue_while(&scouting_iter) {
            // Mirrors ParseUtils.scala lines 84-98
            match scouting_iter.peek() {
                Some(INodeLEEnum::Word(w)) if w.str == *keyword => {
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

    /// Parse a complete file from lexer output
    pub fn parse_file(&mut self, file: FileL) -> ParseResult<FileP> {
        let FileL { denizens, comment_ranges } = file;

        let mut parsed_denizens = Vec::new();

        for denizen in denizens {
            let parsed = self.parse_denizen(denizen)?;
            parsed_denizens.push(parsed);
        }

        Ok(FileP {
            file_coord: FileCoordinate {
                package_coord: PackageCoordinate {
                    module: Arc::new(self.interner.lock().unwrap().intern("")),
                    packages: vec![],
                },
                filepath: "".to_string(),
            },
            comments_ranges: comment_ranges,
            denizens: parsed_denizens,
        })
    }

    /// Parse a top-level denizen
    pub fn parse_denizen(&mut self, denizen: IDenizenL) -> ParseResult<IDenizenP> {
        match denizen {
            IDenizenL::TopLevelFunction(func) => {
                // Top-level functions are not in a citizen (struct/interface)
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
    fn parse_identifying_runes(&mut self, node: &AngledLE) -> ParseResult<GenericParametersP> {
        let iter = ScrambleIterator::new(node.contents.clone());
        let parts = iter.split_on_symbol(',', false);

        let mut params = Vec::new();
        for part in parts {
            let param = self.parse_generic_parameter(part)?;
            params.push(param);
        }

        Ok(GenericParametersP {
            range: node.range,
            params,
        })
    }

    /// Parse a single generic parameter
    fn parse_generic_parameter(&mut self, mut iter: ScrambleIterator) -> ParseResult<GenericParameterP> {
        let range = iter.range();

        // Parse optional prefixing region
        let maybe_coord_region = self.parse_prefixing_region(&mut iter)?;

        // Parse the main parameter
        let (name, maybe_type, attributes) = match self.parse_region(&mut iter)? {
            None => {
                // Regular rune parameter
                let name = match iter.peek() {
                    Some(INodeLEEnum::Word(WordLE { range, str })) => {
                        let result = NameP {
                            range: *range,
                            str: Arc::new(str.clone()),
                        };
                        iter.advance();
                        result
                    }
                    _ => return Err(ParseError::BadRuneNameError(iter.get_pos())),
                };

                let type_begin = iter.get_pos();
                let maybe_rune_type = self.templex_parser.parse_rune_type(&mut iter)?;

                let maybe_type = maybe_rune_type.map(|tyype| GenericParameterTypeP {
                    range: RangeL {
                        begin: type_begin,
                        end: iter.get_prev_end_pos(),
                    },
                    tyype,
                });

                let maybe_attrs = if iter.try_skip_word(&Arc::new(self.keywords.imm.clone())).is_some() {
                    vec![IRuneAttributeP::ImmutableRuneAttribute(RangeL { begin: iter.get_prev_end_pos(), end: iter.get_prev_end_pos() })]
                } else {
                    vec![]
                };

                (name, maybe_type, maybe_attrs)
            }
            Some(region) => {
                // Region parameter
                let attributes = if let Some(range) = iter.try_skip_word(&Arc::new(self.keywords.ro.clone())) {
                    vec![IRuneAttributeP::ReadOnlyRegionRuneAttribute(range)]
                } else if let Some(range) = iter.try_skip_word(&Arc::new(self.keywords.rw.clone())) {
                    vec![IRuneAttributeP::ReadWriteRegionRuneAttribute(range)]
                } else if let Some(range) = iter.try_skip_word(&Arc::new(self.keywords.additive.clone())) {
                    vec![IRuneAttributeP::AdditiveRegionRuneAttribute(range)]
                } else if let Some(range) = iter.try_skip_word(&Arc::new(self.keywords.imm.clone())) {
                    vec![IRuneAttributeP::ImmutableRegionRuneAttribute(range)]
                } else {
                    vec![]
                };

                let tyype = GenericParameterTypeP {
                    range: RangeL {
                        begin: range.begin,
                        end: iter.get_prev_end_pos(),
                    },
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

        Ok(GenericParameterP {
            range,
            name,
            maybe_type,
            coord_region: maybe_coord_region,
            attributes,
            maybe_default,
        })
    }

    /// Parse optional prefixing region (e.g., `'a`)
    fn parse_prefixing_region(&mut self, original_iter: &mut ScrambleIterator) -> ParseResult<Option<RegionRunePT>> {
        let mut tentative_iter = original_iter.clone();

        let region = match self.parse_region(&mut tentative_iter)? {
            Some(region) => {
                // Check if the next token immediately follows (no gap)
                match tentative_iter.peek() {
                    Some(next) if next.range().begin == region.range.end => region,
                    _ => return Ok(None),
                }
            }
            None => return Ok(None),
        };

        original_iter.skip_to(&tentative_iter);
        Ok(Some(region))
    }

    /// Parse optional region marker
    fn parse_region(&mut self, original_iter: &mut ScrambleIterator) -> ParseResult<Option<RegionRunePT>> {
        let mut tentative_iter = original_iter.clone();
        let rune_begin = tentative_iter.get_pos();

        let maybe_rune = if tentative_iter.try_skip_symbol('\'') {
            // Anonymous region (isolate)
            None
        } else {
            let region_rune = match tentative_iter.next_word() {
                None => return Ok(None),
                Some(r) => r,
            };

            if !tentative_iter.try_skip_symbol('\'') {
                return Ok(None);
            }

            Some(region_rune)
        };

        let rune_end = tentative_iter.get_prev_end_pos();
        original_iter.skip_to(&tentative_iter);

        let range = RangeL {
            begin: rune_begin,
            end: rune_end,
        };

        Ok(Some(RegionRunePT {
            range,
            name: maybe_rune.map(|z| NameP {
                range: RangeL {
                    begin: rune_begin,
                    end: rune_end,
                },
                str: Arc::new(z.str),
            }),
        }))
    }

    /// Parse struct member
    fn parse_struct_member(&mut self, iter: &mut ScrambleIterator) -> ParseResult<IStructContent> {
        let begin = iter.get_pos();

        // Parse name (can be a word or integer for variadic)
        let name = match iter.peek() {
            Some(INodeLEEnum::ParsedInteger(ParsedIntegerLE { range, value, .. })) => {
                let result = NameP {
                    range: *range,
                    str: Arc::new(self.interner.lock().unwrap().intern(&value.to_string())),
                };
                iter.advance();
                result
            }
            _ => match iter.next_word() {
                Some(WordLE { range, str }) => NameP { range, str: Arc::new(str) },
                None => {
                    return Err(ParseError::BadStructMember(iter.get_pos()))
                }
            },
        };

        // Parse variability (! means varying)
        let variability = if iter.try_skip_symbol('!') {
            VariabilityP::Varying
        } else {
            VariabilityP::Final
        };

        // Check for variadic (..)
        let variadic = matches!(iter.peek2(), (Some(INodeLEEnum::Symbol(SymbolLE { c: '.', .. })), Some(INodeLEEnum::Symbol(SymbolLE { c: '.', .. }))));
        if variadic {
            iter.advance();
            iter.advance();
        }

        // Parse type
        let tyype = self.templex_parser.parse_templex(iter)?;

        if variadic {
            if *name.str != self.keywords.underscore {
                return Err(ParseError::VariadicStructMemberHasName(iter.get_pos()));
            }

            Ok(IStructContent::VariadicStructMember {
                range: RangeL {
                    begin,
                    end: iter.get_prev_end_pos(),
                },
                variability,
                tyype,
            })
        } else {
            Ok(IStructContent::NormalStructMember {
                range: RangeL {
                    begin,
                    end: iter.get_prev_end_pos(),
                },
                name,
                variability,
                tyype,
            })
        }
    }

    /// Parse a struct definition
    fn parse_struct(&mut self, struct_l: StructL) -> ParseResult<StructP> {
        let StructL {
            range: struct_range,
            name: name_l,
            attributes: attributes_l,
            mutability: maybe_mutability_l,
            identifying_runes: maybe_identifying_runes_l,
            template_rules: maybe_template_rules_l,
            members: contents,
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
                let iter = ScrambleIterator::new(rules_scramble.clone());
                let parts = iter.split_on_symbol(',', false);
                let mut rules = Vec::new();
                for mut part in parts {
                    rules.push(self.templex_parser.parse_rule(&mut part)?);
                }
                Ok(TemplateRulesP {
                    range: rules_scramble.range,
                    rules,
                })
            })
            .transpose()?;

        // Parse attributes
        let mut attributes = Vec::new();
        for attr_l in attributes_l {
            attributes.push(self.parse_attribute(attr_l)?);
        }

        // Parse mutability
        let maybe_mutability = maybe_mutability_l
            .map(|mut_l| {
                let mut iter = ScrambleIterator::new(mut_l);
                self.templex_parser.parse_templex(&mut iter)
            })
            .transpose()?;

        // Parse struct members
        let iter = ScrambleIterator::new(contents.clone());
        let parts = iter.split_on_symbol(';', false);
        let mut members_vec = Vec::new();
        for mut part in parts {
            if !part.at_end() {
                members_vec.push(self.parse_struct_member(&mut part)?);
            }
        }

        let members = StructMembersP {
            range: contents.range,
            contents: members_vec,
        };

        Ok(StructP {
            range: struct_range,
            name: self.to_name(name_l),
            attributes,
            mutability: maybe_mutability,
            identifying_runes: maybe_identifying_runes,
            template_rules: maybe_template_rules,
            maybe_default_region_rune: None,
            body_range: contents.range,
            members,
        })
    }

    /// Parse an interface definition
    fn parse_interface(&mut self, interface_l: InterfaceL) -> ParseResult<InterfaceP> {
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
                let iter = ScrambleIterator::new(rules_scramble.clone());
                let parts = iter.split_on_symbol(',', false);
                let mut rules = Vec::new();
                for mut part in parts {
                    rules.push(self.templex_parser.parse_rule(&mut part)?);
                }
                Ok(TemplateRulesP {
                    range: rules_scramble.range,
                    rules,
                })
            })
            .transpose()?;

        // Parse attributes
        let mut attributes = Vec::new();
        for attr_l in attributes_l {
            attributes.push(self.parse_attribute(attr_l)?);
        }

        // Parse mutability
        let maybe_mutability = maybe_mutability_l
            .map(|mut_l| {
                let mut iter = ScrambleIterator::new(mut_l);
                self.templex_parser.parse_templex(&mut iter)
            })
            .transpose()?;

        // Parse interface methods
        // Interface methods are in a citizen (interface), so is_in_citizen = true
        let mut members_vec = Vec::new();
        for method_l in methods {
            members_vec.push(self.parse_function(method_l, true)?);
        }

        Ok(InterfaceP {
            range: interface_range,
            name: self.to_name(name_l),
            attributes,
            mutability: maybe_mutability,
            maybe_identifying_runes,
            template_rules: maybe_template_rules,
            maybe_default_region_rune: None,
            body_range,
            members: members_vec,
        })
    }

    /// Parse an impl block
    /// Mirrors Parser.parseImpl in Parser.scala lines 397-461
    fn parse_impl(&mut self, impl_l: ImplL) -> ParseResult<ImplP> {
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
                let iter = ScrambleIterator::new(template_rules_scramble.clone());
                let rule_iters = iter.split_on_symbol(',', false);
                let mut elements_pr = Vec::new();
                
                for rule_iter in rule_iters {
                    elements_pr.push(self.templex_parser.parse_rule(&mut rule_iter.clone())?);
                }
                
                Some(TemplateRulesP {
                    range: template_rules_scramble.range,
                    rules: elements_pr,
                })
            }
            None => None,
        };

        // Parse struct templex if present
        let struct_p = match struct_l {
            None => None,
            Some(struct_l) => {
                let mut iter = ScrambleIterator::new(struct_l);
                Some(self.templex_parser.parse_templex(&mut iter)?)
            }
        };

        // Parse interface templex
        let mut iter = ScrambleIterator::new(interface_l);
        let interface_p = self.templex_parser.parse_templex(&mut iter)?;

        // Parse attributes
        let mut attributes_p = Vec::new();
        for attribute_l in attributes_l {
            attributes_p.push(self.parse_attribute(attribute_l)?);
        }

        Ok(ImplP {
            range: impl_range,
            generic_params: maybe_identifying_runes,
            template_rules: maybe_template_rules_p,
            struct_: struct_p,
            interface: interface_p,
            attributes: attributes_p,
        })
    }

    /// Parse an export-as declaration
    /// Mirrors Parser.parseExportAs in Parser.scala lines 465-497
    fn parse_export_as(&mut self, export_l: ExportAsL) -> ParseResult<ExportAsP> {
        let mut iter = ScrambleIterator::new(export_l.contents.clone());

        // Try to find "as" keyword and get everything before it
        // Mirrors ParseUtils.trySkipPastKeywordWhile in ParseUtils.scala lines 77-102
        let exportee = {
            let mut scouting_iter = iter.clone();
            let mut found_as = false;
            let mut before_iter = iter.clone();
            
            while scouting_iter.has_next() {
                // Check if we should continue (not at semicolon)
                let should_continue = match scouting_iter.peek() {
                    None => false,
                    Some(INodeLEEnum::Symbol(SymbolLE { c: ';', .. })) => false,
                    _ => true,
                };
                
                if !should_continue {
                    break;
                }
                
                // Check if this is the "as" keyword
                if let Some(INodeLEEnum::Word(WordLE { str, .. })) = scouting_iter.peek() {
                    if *str == self.keywords.r#as {
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
        let name = match iter.peek() {
            None => return Err(ParseError::BadExportEnd(iter.get_pos())),
            Some(INodeLEEnum::Word(word)) => {
                self.to_name(word.clone())
            }
            _ => return Err(ParseError::BadExportEnd(iter.get_pos())),
        };

        Ok(ExportAsP {
            range: export_l.range,
            struct_: exportee,
            exported_name: name,
        })
    }

    /// Parse an import declaration
    /// Mirrors Parser.parseImport in Parser.scala lines 499-516
    fn parse_import(&mut self, import_l: ImportL) -> ParseResult<ImportP> {
        let ImportL {
            range,
            module_name: module_name_l,
            package_steps: package_steps_l,
            importee_name: importee_name_l,
        } = import_l;

        let module_name_p = self.to_name(module_name_l);

        let mut package_steps_p = Vec::new();
        for step in package_steps_l {
            package_steps_p.push(self.to_name(step));
        }

        let importee_name_p = self.to_name(importee_name_l);

        Ok(ImportP {
            range,
            module_name: module_name_p,
            package_steps: package_steps_p,
            importee_name: importee_name_p,
        })
    }

    /// Parse a function
    /// Mirrors Parser.parseFunction in Parser.scala lines 552-654
    fn parse_function(&mut self, func_l: FunctionL, is_in_citizen: bool) -> ParseResult<FunctionP> {
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
        let params_iter = ScrambleIterator::new(params_l.contents.clone());
        let param_iters = params_iter.split_on_symbol(',', false);
        
        // Use field splitting to borrow parsers separately
        let Self { pattern_parser, templex_parser, .. } = self;
        
        for (index, pattern_iter) in param_iters.into_iter().enumerate() {
            let mut iter = pattern_iter.clone();
            params_p_vec.push(
                pattern_parser.parse_parameter(&mut iter, templex_parser, index, is_in_citizen, true, false)?
            );
        }
        
        let params_p = ParamsP {
            range: params_l.range,
            params: params_p_vec,
        };

        // Parse trailing details to extract return type, where clause, and default region
        let (trailing_details_with_return_and_where, maybe_default_region) =
            self.parse_body_default_region(original_trailing_details_l.clone());

        // Mirrors Parser.scala lines 582-618
        // TODO: simplify this. It's really just trying to split on "where".
        let mut return_and_where_iter = ScrambleIterator::new(trailing_details_with_return_and_where.clone());
        
        // Mirrors Parser.scala lines 584-589
        let (maybe_return_iter, return_end_pos, maybe_rules_iter) =
            match Self::try_skip_past_keyword_while(
                &mut return_and_where_iter,
                &self.keywords.r#where,
                |it| it.has_next(),
            ) {
                None => {
                    // No "where" was found. Use everything remaining.
                    (Some(return_and_where_iter.clone()), trailing_details_with_return_and_where.range.end, None)
                }
                Some((_, return_iter)) => {
                    (Some(return_iter), return_and_where_iter.scramble.range.end, Some(return_and_where_iter.clone()))
                }
            };
        
        // Mirrors Parser.scala lines 591-602
        let return_begin_pos = trailing_details_with_return_and_where.range.begin;
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
            range: RangeL {
                begin: return_begin_pos,
                end: return_end_pos,
            },
            ret_type: maybe_return_type_p,
        };
        
        // Mirrors Parser.scala lines 605-618
        let maybe_rules_p = maybe_rules_iter.map(|rules_iter| {
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
                rules,
            })
        }).transpose()?;

        // Parse attributes
        let mut attributes_p = Vec::new();
        for attribute_l in attributes_l {
            attributes_p.push(self.parse_attribute(attribute_l)?);
        }

        let header = FunctionHeaderP {
            range: header_range_l,
            name: Some(self.to_name(name_l)),
            attributes: attributes_p,
            generic_parameters: maybe_identifying_runes,
            template_rules: maybe_rules_p,
            params: Some(params_p),
            ret: return_p,
        };

        // Parse body if present
        // Use field splitting to borrow parsers separately
        let Self { expression_parser, templex_parser, pattern_parser, .. } = self;
        
        let body_p = match maybe_body_l {
            Some(body_l) => {
                let FunctionBodyL { body: block_l } = body_l;
                let statements_p = expression_parser.parse_block(&block_l, templex_parser, pattern_parser)?;
                Some(Box::new(BlockPE {
                    range: block_l.range,
                    maybe_pure: None,
                    maybe_default_region: maybe_default_region,
                    inner: Box::new(statements_p),
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

    /// Parse body default region from trailing details
    /// Mirrors Parser.parseBodyDefaultRegion in Parser.scala lines 660-691
    fn parse_body_default_region(&self, input_scramble: ScrambleLE) -> (ScrambleLE, Option<RegionRunePT>) {
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
                    INodeLEEnum::Symbol(SymbolLE { range: symbol_range, c: '\'' }) => {
                        Some(RegionRunePT {
                            range: *symbol_range,
                            name: None,
                        })
                    }
                    _ => None,
                }
            }
            n if n >= 2 => {
                // Check if the last two elements are word then apostrophe
                let last_two = &input_scramble.elements[n - 2..n];
                match (&*last_two[0], &*last_two[1]) {
                    (INodeLEEnum::Word(WordLE { range: word_range, str: region_name }), 
                     INodeLEEnum::Symbol(SymbolLE { range: symbol_range, c: '\'' })) => {
                        let range = RangeL {
                            begin: word_range.begin,
                            end: symbol_range.end,
                        };
                        Some(RegionRunePT {
                            range,
                            name: Some(NameP {
                                range: *word_range,
                                str: Arc::new(region_name.clone()),
                            }),
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
        
        let preceding_elements: Vec<_> = input_scramble.elements[..input_scramble.elements.len() - elements_to_remove]
            .iter()
            .cloned()
            .collect();
        
        let preceding_elements_range = if preceding_elements.is_empty() {
            RangeL {
                begin: input_scramble.range.begin,
                end: input_scramble.range.begin,
            }
        } else {
            RangeL {
                begin: preceding_elements.first().unwrap().range().begin,
                end: preceding_elements.last().unwrap().range().end,
            }
        };

        let preceding_elements_scramble = ScrambleLE {
            range: preceding_elements_range,
            elements: preceding_elements,
        };

        (preceding_elements_scramble, default_region)
    }

    /// Parse an attribute
    fn parse_attribute(&mut self, attr_l: IAttributeL) -> ParseResult<IAttributeP> {
        match attr_l {
            IAttributeL::WeakableAttribute(range) => Ok(IAttributeP::WeakableAttribute(range)),
            IAttributeL::SealedAttribute(range) => Ok(IAttributeP::SealedAttribute(range)),
            IAttributeL::MacroCall { range, inclusion, name } => {
                Ok(IAttributeP::MacroCall {
                    range,
                    inclusion: match inclusion {
                        IMacroInclusionL::CallMacro => IMacroInclusionP::CallMacro,
                        IMacroInclusionL::DontCallMacro => IMacroInclusionP::DontCallMacro,
                    },
                    name: self.to_name(name),
                })
            }
            IAttributeL::AbstractAttribute(range) => Ok(IAttributeP::AbstractAttribute(range)),
            IAttributeL::ExternAttribute { range, maybe_custom_name } => {
                // Mirrors Parser.scala parseAttribute handling of ExternAttribute
                match maybe_custom_name {
                    None => Ok(IAttributeP::ExternAttribute(range)),
                    Some(parend) => {
                        // extern("name") becomes BuiltinAttribute
                        let iter = ScrambleIterator::new(parend.contents.clone());
                        if let Some(INodeLEEnum::String(string_le)) = iter.peek() {
                            // Extract the string value from the parts
                            // For a simple string like "bork", there should be one Literal part
                            if string_le.parts.len() == 1 {
                                if let StringPart::Literal { s, .. } = &string_le.parts[0] {
                                    let name = NameP {
                                        range: string_le.range,
                                        str: Arc::new(self.interner.lock().unwrap().intern(s)),
                                    };
                                    return Ok(IAttributeP::BuiltinAttribute {
                                        range,
                                        generator_name: name,
                                    });
                                }
                            }
                            Err(ParseError::BadExternAttribute(range.begin))
                        } else {
                            Err(ParseError::BadExternAttribute(range.begin))
                        }
                    }
                }
            }
            IAttributeL::ExportAttribute(range) => Ok(IAttributeP::ExportAttribute(range)),
            IAttributeL::PureAttribute(range) => Ok(IAttributeP::PureAttribute(range)),
            IAttributeL::AdditiveAttribute(range) => Ok(IAttributeP::AdditiveAttribute(range)),
            IAttributeL::LinearAttribute(range) => Ok(IAttributeP::LinearAttribute(range)),
        }
    }

    /// Helper to convert WordLE to NameP
    fn to_name(&self, word: WordLE) -> NameP {
        NameP {
            range: word.range,
            str: Arc::new(word.str),
        }
    }
}

// TemplexParser and PatternParser are defined in their respective modules

// ExpressionParser is defined in expression_parser.rs

