/// Templex (Type Expression) Parser
/// Mirrors Frontend/ParsingPass/src/dev/vale/parsing/templex/TemplexParser.scala (732 lines)
///
/// This file implements type expression parsing exactly as in the Scala version.
/// All method names, variable names, and logic flow match the Scala implementation.

use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::ast::*;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::scramble_iterator::ScrambleIterator;
use std::sync::{Arc, Mutex};

type ParseResult<T> = Result<T, ParseError>;

/// TemplexParser - parses type expressions
/// Mirrors Scala's TemplexParser class (line 13 in TemplexParser.scala)
#[derive(Clone)]
pub struct TemplexParser {
    _interner: Arc<Mutex<Interner>>,
    keywords: Arc<Keywords>,
}

impl TemplexParser {
    pub fn new(interner: Arc<Mutex<Interner>>, keywords: Arc<Keywords>) -> Self {
        TemplexParser {
            _interner: interner,
            keywords,
        }
    }

    /// Parse an array type expression
    /// Mirrors parseArray in TemplexParser.scala lines 14-85
    pub fn parse_array(&mut self, original_iter: &mut ScrambleIterator) -> ParseResult<Option<ITemplexPT>> {
        let begin = original_iter.get_pos();

        let mut tentative_iter = original_iter.clone();

        let immutable = tentative_iter.try_skip_symbol('#');

        // Check for squared brackets
        let size_scramble_iter_l = match tentative_iter.peek() {
            Some(INodeLEEnum::Squared(squared)) => {
                let contents = squared.contents.clone();
                tentative_iter.advance();
                ScrambleIterator::new(contents)
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

        let mutability = match (immutable, maybe_template_args.as_ref().and_then(|v| v.get(0))) {
            (true, Some(_)) => return Err(ParseError::FoundBothImmutableAndMutabilityInArray(begin)),
            (false, Some(templex)) => templex.clone(),
            (true, None) => ITemplexPT::Mutability {
                range: RangeL { begin: template_args_begin, end: template_args_end },
                mutability: MutabilityP::Immutable,
            },
            (false, None) => ITemplexPT::Mutability {
                range: RangeL { begin: template_args_begin, end: template_args_end },
                mutability: MutabilityP::Mutable,
            },
        };

        let variability = maybe_template_args
            .as_ref()
            .and_then(|v| v.get(1))
            .cloned()
            .unwrap_or_else(|| {
                ITemplexPT::Variability {
                    range: RangeL { begin: template_args_begin, end: template_args_end },
                    variability: VariabilityP::Final,
                }
            });

        let element_type = self.parse_templex(iter)?;

        let result = match maybe_size_templex {
            None => ITemplexPT::RuntimeSizedArray {
                range: RangeL { begin, end: iter.get_prev_end_pos() },
                mutability: Box::new(mutability),
                element: Box::new(element_type),
            },
            Some(size_templex) => ITemplexPT::StaticSizedArray {
                range: RangeL { begin, end: iter.get_prev_end_pos() },
                mutability: Box::new(mutability),
                variability: Box::new(variability),
                size: Box::new(size_templex),
                element: Box::new(element_type),
            },
        };

        Ok(Some(result))
    }

    /// Parse a function name (including operator names)
    /// Mirrors parseFunctionName in TemplexParser.scala lines 87-161
    pub fn parse_function_name(&mut self, iter: &mut ScrambleIterator) -> Option<NameP> {
        match iter.peek() {
            Some(INodeLEEnum::Word(word)) => {
                let range = word.range;
                let str = word.str.clone();
                iter.advance();
                Some(NameP {
                    range,
                    str: Arc::new(str),
                })
            }
            Some(INodeLEEnum::Symbol(_)) => {
                let begin = iter.get_pos();
                match iter.peek3() {
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '=', .. })),
                     Some(INodeLEEnum::Symbol(SymbolLE { c: '=', .. })),
                     Some(INodeLEEnum::Symbol(SymbolLE { c: '=', .. }))) => {
                        iter.advance();
                        iter.advance();
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.triple_equals.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '<', .. })),
                     Some(INodeLEEnum::Symbol(SymbolLE { c: '=', .. })),
                     Some(INodeLEEnum::Symbol(SymbolLE { c: '>', .. }))) => {
                        iter.advance();
                        iter.advance();
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.spaceship.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '=', .. })),
                     Some(INodeLEEnum::Symbol(SymbolLE { c: '=', .. })), _) => {
                        iter.advance();
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.double_equals.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '!', .. })),
                     Some(INodeLEEnum::Symbol(SymbolLE { c: '=', .. })), _) => {
                        iter.advance();
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.not_equals.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '<', .. })),
                     Some(INodeLEEnum::Symbol(SymbolLE { c: '=', .. })), _) => {
                        iter.advance();
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.less_equals.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '>', .. })),
                     Some(INodeLEEnum::Symbol(SymbolLE { c: '=', .. })), _) => {
                        iter.advance();
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.greater_equals.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '<', .. })), _, _) => {
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.less.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '>', .. })), _, _) => {
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.greater.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '+', .. })), _, _) => {
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.plus.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '-', .. })), _, _) => {
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.minus.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '*', .. })), _, _) => {
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.asterisk.clone()),
                        })
                    }
                    (Some(INodeLEEnum::Symbol(SymbolLE { c: '/', .. })), _, _) => {
                        iter.advance();
                        Some(NameP {
                            range: RangeL { begin, end: iter.get_prev_end_pos() },
                            str: Arc::new(self.keywords.slash.clone()),
                        })
                    }
                    _ => None,
                }
            }
            Some(INodeLEEnum::Parend(ParendLE { range, .. })) => {
                // Don't advance, we do that elsewhere
                Some(NameP {
                    range: RangeL { begin: range.begin, end: range.begin },
                    str: Arc::new(self.keywords.underscores_call.clone()),
                })
            }
            _ => None,
        }
    }

    /// Parse a function prototype type
    /// Mirrors parsePrototype in TemplexParser.scala lines 163-189
    pub fn parse_prototype(&mut self, iter: &mut ScrambleIterator) -> ParseResult<Option<ITemplexPT>> {
        let begin = iter.get_pos();

        if iter.try_skip_word(&self.keywords.func).is_none() {
            return Ok(None);
        }

        let name = match self.parse_function_name(iter) {
            Some(n) => n,
            None => return Err(ParseError::BadPrototypeName(iter.get_pos())),
        };

        let args_begin = iter.get_pos();
        let args = match self.parse_tuple(iter)? {
            None => return Err(ParseError::BadPrototypeParams(iter.get_pos())),
            Some(ITemplexPT::Tuple { elements, .. }) => elements,
            Some(_) => return Err(ParseError::BadPrototypeParams(iter.get_pos())),
        };
        let args_end = iter.get_prev_end_pos();

        let return_type = self.parse_templex(iter)?;

        let result = ITemplexPT::Func {
            range: RangeL { begin, end: iter.get_prev_end_pos() },
            name,
            params_range: RangeL { begin: args_begin, end: args_end },
            parameters: args,
            return_type: Box::new(return_type),
        };

        Ok(Some(result))
    }

    /// Parse template call arguments <...>
    /// Mirrors parseTemplateCallArgs in TemplexParser.scala lines 443-461
    pub fn parse_template_call_args(&mut self, iter: &mut ScrambleIterator) -> ParseResult<Option<Vec<ITemplexPT>>> {
        let angled = match iter.peek() {
            Some(INodeLEEnum::Angled(a)) => a.clone(),
            Some(_) => return Ok(None),
            None => return Ok(None),
        };
        
        iter.advance();
        
        let mut elements_p = Vec::new();
        let contents_iter = ScrambleIterator::new(angled.contents.clone());
        let element_iters = contents_iter.split_on_symbol(',', false);
        
        for element_iter in element_iters {
            let mut elem_iter = element_iter.clone();
            elements_p.push(self.parse_templex(&mut elem_iter)?);
        }
        
        Ok(Some(elements_p))
    }

    /// Parse a tuple type
    /// Mirrors parseTuple in TemplexParser.scala lines 463-481
    pub fn parse_tuple(&mut self, outer_iter: &mut ScrambleIterator) -> ParseResult<Option<ITemplexPT>> {
        let _begin = outer_iter.get_pos();
        
        match outer_iter.peek() {
            Some(INodeLEEnum::Parend(ParendLE { range, contents })) => {
                let range = *range;
                let contents = contents.clone();
                outer_iter.advance();
                
                let mut elements = Vec::new();
                let contents_iter = ScrambleIterator::new(contents);
                let iter_splits = contents_iter.split_on_symbol(',', false);
                
                for iter_split in iter_splits {
                    let mut iter = iter_split.clone();
                    elements.push(self.parse_templex(&mut iter)?);
                }
                
                Ok(Some(ITemplexPT::Tuple { range, elements }))
            }
            _ => Ok(None),
        }
    }

    /// Parse interpreted type (with ownership/region prefixes)
    /// Mirrors parseInterpreted in TemplexParser.scala lines 273-303
    pub fn parse_interpreted(&mut self, iter: &mut ScrambleIterator) -> ParseResult<Option<ITemplexPT>> {
        let begin = iter.get_pos();

        // Parse ownership prefix (^, @, &&, &)
        let maybe_ownership = if iter.try_skip_symbol('^') {
            Some(OwnershipPT {
                range: RangeL { begin, end: iter.get_pos() },
                ownership: OwnershipP::Own,
            })
        } else if iter.try_skip_symbol('@') {
            Some(OwnershipPT {
                range: RangeL { begin, end: iter.get_pos() },
                ownership: OwnershipP::Share,
            })
        } else if iter.try_skip_symbols(&['&', '&']) {
            Some(OwnershipPT {
                range: RangeL { begin, end: iter.get_pos() },
                ownership: OwnershipP::Weak,
            })
        } else if iter.try_skip_symbol('&') {
            Some(OwnershipPT {
                range: RangeL { begin, end: iter.get_pos() },
                ownership: OwnershipP::Borrow,
            })
        } else {
            None
        };

        // Parse region (e.g., a' in a'T)
        let maybe_region = Self::parse_region_helper(iter)?;

        // If we have neither ownership nor region, return None
        match (&maybe_ownership, &maybe_region) {
            (None, None) => return Ok(None),
            _ => {}
        }

        // Parse the inner templex
        let inner = self.parse_templex_atom_and_call_and_prefixes(iter)?;

        Ok(Some(ITemplexPT::Interpreted {
            range: RangeL { begin, end: iter.get_prev_end_pos() },
            maybe_ownership: maybe_ownership.map(Box::new),
            maybe_region: maybe_region.map(Box::new),
            inner: Box::new(inner),
        }))
    }

    /// Parse ending region suffix (type')
    /// Mirrors parseEndingRegion in TemplexParser.scala lines 306-323
    pub fn parse_ending_region(&mut self, original_iter: &mut ScrambleIterator) -> ParseResult<Option<RegionRunePT>> {
        let mut tentative_iter = original_iter.clone();

        let region = match Self::parse_region_helper(&mut tentative_iter)? {
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

    /// Helper to parse region - mirrors Parser.parseRegion in Parser.scala lines 826-853
    fn parse_region_helper(original_iter: &mut ScrambleIterator) -> ParseResult<Option<RegionRunePT>> {
        let mut tentative_iter = original_iter.clone();

        let rune_begin = tentative_iter.get_pos();
        let maybe_rune = if tentative_iter.try_skip_symbol('\'') {
            // Anonymous region, in other words an isolate
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

        let range = RangeL { begin: rune_begin, end: rune_end };
        Ok(Some(RegionRunePT {
            range,
            name: maybe_rune.map(|z| NameP {
                range: RangeL { begin: rune_begin, end: rune_end },
                str: Arc::new(z.str),
            }),
        }))
    }

    /// Parse templex atom and any following calls/prefixes/suffixes
    /// Mirrors parseTemplexAtomAndCallAndPrefixesAndSuffixes in TemplexParser.scala lines 326-334
    pub fn parse_templex_atom_and_call_and_prefixes_and_suffixes(&mut self, original_iter: &mut ScrambleIterator) -> ParseResult<ITemplexPT> {
        let inner = self.parse_templex_atom_and_call_and_prefixes(original_iter)?;
        Ok(inner)
    }

    /// Parse a templex atom (basic type expression)
    /// Mirrors parseTemplexAtom in TemplexParser.scala lines 336-441
    pub fn parse_templex_atom(&mut self, iter: &mut ScrambleIterator) -> ParseResult<ITemplexPT> {
        assert!(iter.peek().is_some());
        let _begin = iter.get_pos();

        // Try keywords first (lines 340-403)
        if let Some(range) = iter.try_skip_word(&self.keywords.underscore) {
            return Ok(ITemplexPT::AnonymousRune(range));
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.truue) {
            return Ok(ITemplexPT::Bool { range, value: true });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.faalse) {
            return Ok(ITemplexPT::Bool { range, value: false });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.own) {
            return Ok(ITemplexPT::Ownership { range, ownership: OwnershipP::Own });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.borrow) {
            return Ok(ITemplexPT::Ownership { range, ownership: OwnershipP::Borrow });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.weak) {
            return Ok(ITemplexPT::Ownership { range, ownership: OwnershipP::Weak });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.share) {
            return Ok(ITemplexPT::Ownership { range, ownership: OwnershipP::Share });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.inl) {
            return Ok(ITemplexPT::Location { range, location: LocationP::Inline });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.heap) {
            return Ok(ITemplexPT::Location { range, location: LocationP::Yonder });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.imm) {
            return Ok(ITemplexPT::Mutability { range, mutability: MutabilityP::Immutable });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.r#mut) {
            return Ok(ITemplexPT::Mutability { range, mutability: MutabilityP::Mutable });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.vary) {
            return Ok(ITemplexPT::Variability { range, variability: VariabilityP::Varying });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.fiinal) {
            return Ok(ITemplexPT::Variability { range, variability: VariabilityP::Final });
        }
        // Duplicate checks for weak, own, share (lines 392-403 in Scala)
        if let Some(range) = iter.try_skip_word(&self.keywords.weak) {
            return Ok(ITemplexPT::Ownership { range, ownership: OwnershipP::Weak });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.own) {
            return Ok(ITemplexPT::Ownership { range, ownership: OwnershipP::Own });
        }
        if let Some(range) = iter.try_skip_word(&self.keywords.share) {
            return Ok(ITemplexPT::Ownership { range, ownership: OwnershipP::Share });
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
        match iter.peek().expect("peek should not be empty") {
            INodeLEEnum::String(StringLE { range, parts }) => {
                let range = *range;
                let parts = parts.clone();
                iter.advance();
                match parts.as_slice() {
                    [StringPart::Literal { range, s }] => {
                        Ok(ITemplexPT::String { range: *range, str: s.clone() })
                    }
                    _ => Err(ParseError::BadStringInTemplex(range.begin)),
                }
            }
            INodeLEEnum::ParsedInteger(ParsedIntegerLE { range, value, .. }) => {
                let range = *range;
                let value = *value;
                iter.advance();
                Ok(ITemplexPT::Int { range, value })
            }
            INodeLEEnum::ParsedDouble(ParsedDoubleLE { range, .. }) => {
                let pos = range.begin;
                iter.advance();
                Err(ParseError::RangedInternalError {
                    pos,
                    msg: "Floats in types not supported!".to_string(),
                })
            }
            INodeLEEnum::Word(WordLE { range, str }) => {
                let range = *range;
                let str = str.clone();
                iter.advance();
                Ok(ITemplexPT::NameOrRune(NameP {
                    range,
                    str: Arc::new(str),
                }))
            }
            _ => Err(ParseError::BadTypeExpression(iter.get_pos())),
        }
    }

    /// Parse templex atom and any following call
    /// Mirrors parseTemplexAtomAndCall in TemplexParser.scala lines 483-499
    pub fn parse_templex_atom_and_call(&mut self, iter: &mut ScrambleIterator) -> ParseResult<ITemplexPT> {
        let begin = iter.get_pos();

        let atom = self.parse_templex_atom(iter)?;

        match self.parse_template_call_args(iter)? {
            Some(args) => {
                return Ok(ITemplexPT::Call {
                    range: RangeL { begin, end: iter.get_prev_end_pos() },
                    template: Box::new(atom),
                    args,
                });
            }
            None => {}
        }

        Ok(atom)
    }

    /// Parse templex atom, call, and prefixes
    /// Mirrors parseTemplexAtomAndCallAndPrefixes in TemplexParser.scala lines 501-539
    pub fn parse_templex_atom_and_call_and_prefixes(&mut self, iter: &mut ScrambleIterator) -> ParseResult<ITemplexPT> {
        assert!(iter.has_next());

        // Check for 'in' keyword - should not be interpreted as a templex (lines 506-515)
        match iter.peek() {
            Some(INodeLEEnum::Word(WordLE { str, .. })) if *str == self.keywords.r#in => {
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

    /// Main entry point for parsing a templex
    /// Mirrors parseTemplex in TemplexParser.scala lines 541-545
    pub fn parse_templex(&mut self, iter: &mut ScrambleIterator) -> ParseResult<ITemplexPT> {
        self.parse_templex_atom_and_call_and_prefixes_and_suffixes(iter)
    }

    /// Parse a typed rune (T: Type)
    /// Mirrors parseTypedRune in TemplexParser.scala lines 547-571
    pub fn parse_typed_rune(&mut self, original_iter: &mut ScrambleIterator) -> ParseResult<Option<IRulexPR>> {
        match original_iter.peek2() {
            // Don't parse "func moo()void" (lines 550-552)
            (Some(INodeLEEnum::Word(WordLE { str: name_str, .. })), _) 
                if *name_str == self.keywords.func => {
                Ok(None)
            }
            (Some(INodeLEEnum::Word(WordLE { range: name_range, str: name_str })), 
             Some(INodeLEEnum::Word(WordLE { range: type_range, .. }))) => {
                let name_range = *name_range;
                let type_range = *type_range;
                
                // Parse the rune name (or underscore for anonymous)
                let maybe_name = if *name_str == self.keywords.underscore {
                    None
                } else {
                    Some(NameP {
                        range: name_range,
                        str: Arc::new(name_str.clone()),
                    })
                };
                
                original_iter.advance();
                
                // Parse the rune type
                let tyype = match self.parse_rune_type(original_iter)? {
                    None => panic!("Expected rune type"),
                    Some(x) => x,
                };
                
                Ok(Some(IRulexPR::Typed {
                    range: RangeL { begin: name_range.begin, end: type_range.end },
                    rune: maybe_name,
                    tyype,
                }))
            }
            _ => Ok(None),
        }
    }

    /// Parse a rule call
    /// Mirrors parseRuleCall in TemplexParser.scala lines 573-607
    pub fn parse_rule_call(&mut self, iter: &mut ScrambleIterator) -> ParseResult<Option<IRulexPR>> {
        match iter.peek2() {
            (Some(INodeLEEnum::Word(WordLE { str, .. })), _) if *str == self.keywords.func => {
                return Ok(None);
            }
            (Some(INodeLEEnum::Word(WordLE { range: name_range, str: name })), 
             Some(INodeLEEnum::Parend(ParendLE { range: args_range, contents: args_lr }))) => {
                let name_range = *name_range;
                let args_range = *args_range;
                let range = RangeL { begin: name_range.begin, end: args_range.end };
                
                // Parse the arguments
                let mut args_pr = Vec::new();
                let args_iter = ScrambleIterator::new(args_lr.clone());
                let arg_iters = args_iter.split_on_symbol(',', false);
                
                for arg_iter in arg_iters {
                    let mut iter = arg_iter.clone();
                    args_pr.push(self.parse_rule(&mut iter)?);
                }
                
                Ok(Some(IRulexPR::BuiltinCall {
                    range,
                    name: NameP {
                        range: name_range,
                        str: Arc::new(name.clone()),
                    },
                    args: args_pr,
                }))
            }
            _ => Ok(None),
        }
    }

    /// Parse a rule destructure
    /// Mirrors parseRuleDestructure in TemplexParser.scala lines 609-632
    pub fn parse_rule_destructure(&mut self, original_iter: &mut ScrambleIterator) -> ParseResult<Option<IRulexPR>> {
        // Extract data from peek2() before mutating
        let (begin, end, components_l) = match original_iter.peek2() {
            (Some(INodeLEEnum::Word(WordLE { range: word_range, .. })), 
             Some(INodeLEEnum::Squared(SquaredLE { range: squared_range, contents: components_l }))) => {
                (word_range.begin, squared_range.end, components_l.clone())
            }
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
        let components_iter = ScrambleIterator::new(components_l);
        let component_iters = components_iter.split_on_symbol(',', false);
        
        for component_iter in component_iters {
            let mut iter = component_iter.clone();
            components_p.push(self.parse_rule(&mut iter)?);
        }
        
        Ok(Some(IRulexPR::Components {
            range: RangeL { begin, end },
            container: rune_type,
            components: components_p,
        }))
    }

    /// Parse a rule atom
    /// Mirrors parseRuleAtom in TemplexParser.scala lines 634-659
    pub fn parse_rule_atom(&mut self, iter: &mut ScrambleIterator) -> ParseResult<IRulexPR> {
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

    /// Parse a rule up to equals precedence
    /// Mirrors parseRuleUpToEqualsPrecedence in TemplexParser.scala lines 661-689
    /// Mirrors parseRuleUpToEqualsPrecedence in TemplexParser.scala lines 661-689
    pub fn parse_rule_up_to_equals_precedence(&mut self, iter: &mut ScrambleIterator) -> ParseResult<IRulexPR> {
        // Try to find an equals sign while scouting ahead (lines 663-672)
        let maybe_before_iter = self.try_skip_past_equals_while(iter, |scouting_iter| {
            match scouting_iter.peek() {
                None => false,
                // Stop on comma
                Some(INodeLEEnum::Symbol(SymbolLE { c: ',', .. })) => false,
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
                Ok(IRulexPR::Equals {
                    range: RangeL {
                        begin: left.range().begin,
                        end: right.range().end,
                    },
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
        }
    }

    /// Helper method to skip past an equals sign while a condition is true
    /// Mirrors ParseUtils.trySkipPastEqualsWhile in ParseUtils.scala
    fn try_skip_past_equals_while<F>(
        &self,
        iter: &mut ScrambleIterator,
        continue_while: F,
    ) -> Option<ScrambleIterator>
    where
        F: Fn(&ScrambleIterator) -> bool,
    {
        let mut scouting_iter = iter.clone();
        while continue_while(&scouting_iter) {
            match scouting_iter.peek3() {
                (Some(prev), Some(INodeLEEnum::Symbol(SymbolLE { range, c: '=' })), Some(next)) => {
                    let surrounded_by_spaces =
                        prev.range().end < range.begin && range.end < next.range().begin;
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

    /// Main entry point for parsing a rule
    /// Mirrors parseRule in TemplexParser.scala lines 691-693
    pub fn parse_rule(&mut self, iter: &mut ScrambleIterator) -> ParseResult<IRulexPR> {
        self.parse_rule_up_to_equals_precedence(iter)
    }

    /// Parse a rune type (Ref, Int, etc.)
    /// Mirrors parseRuneType in TemplexParser.scala lines 695-732
    pub fn parse_rune_type(&mut self, iter: &mut ScrambleIterator) -> ParseResult<Option<ITypePR>> {
        match iter.peek() {
            None => Ok(None),
            
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.int_capitalized => {
                iter.advance();
                Ok(Some(ITypePR::IntType))
            }
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.ref_ => {
                iter.advance();
                Ok(Some(ITypePR::CoordType))
            }
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.kind => {
                iter.advance();
                Ok(Some(ITypePR::KindType))
            }
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.region => {
                iter.advance();
                Ok(Some(ITypePR::RegionType))
            }
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.prot => {
                iter.advance();
                Ok(Some(ITypePR::PrototypeType))
            }
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.ref_list => {
                iter.advance();
                Ok(Some(ITypePR::CoordListType))
            }
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.ownership => {
                iter.advance();
                Ok(Some(ITypePR::OwnershipType))
            }
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.variability => {
                iter.advance();
                Ok(Some(ITypePR::VariabilityType))
            }
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.mutability => {
                iter.advance();
                Ok(Some(ITypePR::MutabilityType))
            }
            Some(INodeLEEnum::Word(WordLE { str: w, .. })) if *w == self.keywords.location => {
                iter.advance();
                Ok(Some(ITypePR::LocationType))
            }
            _ => Err(ParseError::BadRuneTypeError(iter.get_pos())),
        }
    }
}

