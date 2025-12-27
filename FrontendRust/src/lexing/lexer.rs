use super::ast::*;
use super::errors::*;
use super::iterator::*;
use crate::{Interner, Keywords};
use std::sync::{Arc, Mutex};

type Result<T> = std::result::Result<T, ParseError>;

/// Vale lexer
/// Matches Scala's Lexer class
pub struct Lexer {
    interner: Arc<Mutex<Interner>>,
    keywords: Arc<Keywords>,
}

impl Lexer {
    pub fn new(interner: Arc<Mutex<Interner>>, keywords: Arc<Keywords>) -> Self {
        Lexer { interner, keywords }
    }

    /// Lex attributes on a declaration
    pub fn lex_attributes(&mut self, iter: &mut LexingIterator) -> Result<Vec<IAttributeL>> {
        let mut attributes = Vec::new();
        
        loop {
            iter.consume_comments_and_whitespace();
            match self.lex_attribute(iter)? {
                Some(attr) => attributes.push(attr),
                None => break,
            }
        }
        
        Ok(attributes)
    }

    /// Lex a single attribute
    pub fn lex_attribute(&mut self, iter: &mut LexingIterator) -> Result<Option<IAttributeL>> {
        let attribute_begin = iter.get_pos();

        // Check for macro calls
        if iter.try_skip_complete_word("#DeriveStructDrop") {
            let end = iter.get_pos();
            return Ok(Some(IAttributeL::MacroCall {
                range: RangeL::new(attribute_begin, end),
                inclusion: IMacroInclusionL::CallMacro,
                name: WordLE {
                    range: RangeL::new(attribute_begin, end),
                    str: self.keywords.derive_struct_drop.clone(),
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
                    str: self.keywords.derive_struct_drop.clone(),
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
                    str: self.keywords.derive_anonymous_substruct.clone(),
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
                    str: self.keywords.derive_anonymous_substruct.clone(),
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
                    str: self.keywords.derive_interface_drop.clone(),
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
                    str: self.keywords.derive_interface_drop.clone(),
                },
            }));
        }

        // Regular attributes
        if iter.try_skip_complete_word("abstract") {
            let end = iter.get_pos();
            return Ok(Some(IAttributeL::AbstractAttribute(RangeL::new(attribute_begin, end))));
        }

        if iter.try_skip_complete_word("pure") {
            let end = iter.get_pos();
            return Ok(Some(IAttributeL::PureAttribute(RangeL::new(attribute_begin, end))));
        }

        if iter.try_skip_complete_word("additive") {
            let end = iter.get_pos();
            return Ok(Some(IAttributeL::AdditiveAttribute(RangeL::new(attribute_begin, end))));
        }

        if iter.try_skip_complete_word("extern") {
            let maybe_custom_name = if iter.peek() == '(' {
                Some(self.lex_parend(iter)?.unwrap())
            } else {
                None
            };
            let end = iter.get_pos();
            return Ok(Some(IAttributeL::ExternAttribute {
                range: RangeL::new(attribute_begin, end),
                maybe_custom_name,
            }));
        }

        if iter.try_skip_complete_word("exported") {
            let end = iter.get_pos();
            return Ok(Some(IAttributeL::ExportAttribute(RangeL::new(attribute_begin, end))));
        }

        if iter.try_skip_complete_word("linear") {
            let end = iter.get_pos();
            return Ok(Some(IAttributeL::LinearAttribute(RangeL::new(attribute_begin, end))));
        }

        if iter.try_skip_complete_word("weakable") {
            let end = iter.get_pos();
            return Ok(Some(IAttributeL::WeakableAttribute(RangeL::new(attribute_begin, end))));
        }

        if iter.try_skip_complete_word("sealed") {
            let end = iter.get_pos();
            return Ok(Some(IAttributeL::SealedAttribute(RangeL::new(attribute_begin, end))));
        }

        Ok(None)
    }

    /// Lex a top-level denizen (function, struct, interface, impl, import, export)
    pub fn lex_denizen(&mut self, iter: &mut LexingIterator) -> Result<IDenizenL> {
        let denizen_begin = iter.get_pos();

        let attributes = self.lex_attributes(iter)?;
        iter.consume_comments_and_whitespace();

        // Try function
        if let Some(func) = self.lex_function(iter, denizen_begin, attributes.clone())? {
            return Ok(IDenizenL::TopLevelFunction(func));
        }

        // Try struct
        if let Some(strukt) = self.lex_struct(iter, denizen_begin, attributes.clone())? {
            return Ok(IDenizenL::TopLevelStruct(strukt));
        }

        // Try interface
        if let Some(interface) = self.lex_interface(iter, denizen_begin, attributes.clone())? {
            return Ok(IDenizenL::TopLevelInterface(interface));
        }

        // Try impl
        if let Some(impl_) = self.lex_impl(iter, denizen_begin, attributes.clone())? {
            return Ok(IDenizenL::TopLevelImpl(impl_));
        }

        // Try import
        if let Some(import) = self.lex_import(iter, denizen_begin, attributes.clone())? {
            return Ok(IDenizenL::TopLevelImport(import));
        }

        // Try export
        if let Some(export) = self.lex_export(iter, denizen_begin, attributes.clone())? {
            return Ok(IDenizenL::TopLevelExportAs(export));
        }

        Err(ParseError::UnrecognizedDenizenError(iter.get_pos()))
    }

    /// Lex an impl block
    pub fn lex_impl(&mut self, iter: &mut LexingIterator, begin: i32, attributes: Vec<IAttributeL>) -> Result<Option<ImplL>> {
        if !iter.try_skip_complete_word("impl") {
            return Ok(None);
        }

        iter.consume_comments_and_whitespace();

        let maybe_identifying_runes = self.lex_angled(iter)?;
        iter.consume_comments_and_whitespace();

        let interface_name = self.lex_identifier(iter)
            .ok_or(ParseError::BadImplInterface(iter.get_pos()))?;
        iter.consume_comments_and_whitespace();

        let maybe_interface_generic_args = self.lex_angled(iter)?;

        let interface = if let Some(interface_generic_args) = maybe_interface_generic_args {
            ScrambleLE {
                range: RangeL::new(interface_name.range.begin, interface_generic_args.range.end),
                elements: vec![
                    Box::new(INodeLEEnum::Word(interface_name)),
                    Box::new(INodeLEEnum::Angled(interface_generic_args)),
                ],
            }
        } else {
            ScrambleLE {
                range: interface_name.range,
                elements: vec![Box::new(INodeLEEnum::Word(interface_name))],
            }
        };

        iter.consume_comments_and_whitespace();

        if !iter.try_skip_complete_word("for") {
            return Err(ParseError::BadImplFor(iter.get_pos()));
        }

        iter.consume_comments_and_whitespace();

        let struct_name = self.lex_identifier(iter)
            .ok_or(ParseError::BadImplStruct(iter.get_pos()))?;
        iter.consume_comments_and_whitespace();

        let maybe_struct_generic_args = self.lex_angled(iter)?;

        let struct_ = if let Some(struct_generic_args) = maybe_struct_generic_args {
            ScrambleLE {
                range: RangeL::new(struct_name.range.begin, struct_generic_args.range.end),
                elements: vec![
                    Box::new(INodeLEEnum::Word(struct_name)),
                    Box::new(INodeLEEnum::Angled(struct_generic_args)),
                ],
            }
        } else {
            ScrambleLE {
                range: struct_name.range,
                elements: vec![Box::new(INodeLEEnum::Word(struct_name))],
            }
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

        Ok(Some(ImplL {
            range: RangeL::new(begin, end),
            identifying_runes: maybe_identifying_runes,
            template_rules: maybe_rules,
            struct_: Some(struct_),
            interface,
            attributes,
        }))
    }

    /// Lex a function definition
    pub fn lex_function(&mut self, iter: &mut LexingIterator, begin: i32, attributes: Vec<IAttributeL>) -> Result<Option<FunctionL>> {
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
                    self.keywords.not_equals.clone()
                }
                '=' => {
                    if iter.peek_string("===") {
                        iter.skip_to(name_begin as usize + 3);
                        self.keywords.triple_equals.clone()
                    } else if iter.peek_string("==") {
                        iter.skip_to(name_begin as usize + 2);
                        self.keywords.double_equals.clone()
                    } else {
                        return Err(ParseError::BadFunctionName(iter.get_pos()));
                    }
                }
                '>' if iter.peek_string(">=") => {
                    iter.skip_to(name_begin as usize + 2);
                    self.keywords.greater_equals.clone()
                }
                '>' => {
                    iter.advance();
                    self.keywords.greater.clone()
                }
                '<' => {
                    if iter.peek_string("<=>") {
                        iter.skip_to(name_begin as usize + 3);
                        self.keywords.spaceship.clone()
                    } else if iter.peek_string("<=") {
                        iter.skip_to(name_begin as usize + 2);
                        self.keywords.less_equals.clone()
                    } else {
                        iter.advance();
                        self.keywords.less.clone()
                    }
                }
                '+' => {
                    iter.advance();
                    self.keywords.plus.clone()
                }
                '-' => {
                    iter.advance();
                    self.keywords.minus.clone()
                }
                '*' => {
                    iter.advance();
                    self.keywords.asterisk.clone()
                }
                '/' => {
                    iter.advance();
                    self.keywords.slash.clone()
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

        let params = self.lex_parend(iter)?
            .ok_or(ParseError::BadFunctionParamsBegin(iter.get_pos()))?;

        iter.consume_comments_and_whitespace();

        let trailing_details = self.lex_scramble(iter, true, false, true)?;
        iter.consume_comments_and_whitespace();

        let header_end = iter.get_pos();

        let maybe_body = if iter.try_skip(';') {
            None
        } else {
            let body = self.lex_curlied(iter, false)?
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

        Ok(Some(FunctionL {
            range: RangeL::new(begin, end),
            header,
            body: maybe_body,
        }))
    }

    /// Lex a struct definition
    pub fn lex_struct(&mut self, iter: &mut LexingIterator, begin: i32, attributes: Vec<IAttributeL>) -> Result<Option<StructL>> {
        if !iter.try_skip_complete_word("struct") {
            return Ok(None);
        }

        iter.consume_comments_and_whitespace();

        let name = self.lex_identifier(iter)
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

        if !iter.try_skip('{') {
            return Err(ParseError::BadStructContentsBegin(iter.get_pos()));
        }

        iter.consume_comments_and_whitespace();

        let contents = self.lex_scramble(iter, false, false, false)?;

        if !iter.try_skip('}') {
            return Err(ParseError::BadStructContentsEnd(iter.get_pos()));
        }

        iter.consume_comments_and_whitespace();

        let end = iter.get_pos();

        Ok(Some(StructL {
            range: RangeL::new(begin, end),
            name,
            attributes,
            mutability: maybe_mutability,
            identifying_runes: maybe_generic_args,
            template_rules: maybe_rules,
            members: contents,
        }))
    }

    /// Lex an interface definition
    pub fn lex_interface(&mut self, iter: &mut LexingIterator, begin: i32, attributes: Vec<IAttributeL>) -> Result<Option<InterfaceL>> {
        if !iter.try_skip_complete_word("interface") {
            return Ok(None);
        }

        iter.consume_comments_and_whitespace();

        let name = self.lex_identifier(iter)
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
            range: RangeL::new(begin, end),
            name,
            attributes,
            mutability: maybe_mutability,
            maybe_identifying_runes: maybe_generic_args,
            template_rules: maybe_rules,
            body_range: RangeL::new(members_begin, end),
            members,
        }))
    }

    /// Lex an import declaration
    pub fn lex_import(&mut self, iter: &mut LexingIterator, begin: i32, attributes: Vec<IAttributeL>) -> Result<Option<ImportL>> {
        if !iter.try_skip_complete_word(&self.keywords.impoort.str) {
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
                    str: self.keywords.asterisk.clone(),
                }
            } else {
                self.lex_identifier(iter)
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
        let package_steps = steps[1..steps.len() - 1].to_vec();

        Ok(Some(ImportL {
            range: RangeL::new(begin, iter.get_pos()),
            module_name,
            package_steps,
            importee_name,
        }))
    }

    /// Lex an export declaration
    pub fn lex_export(&mut self, iter: &mut LexingIterator, begin: i32, attributes: Vec<IAttributeL>) -> Result<Option<ExportAsL>> {
        if !iter.try_skip_complete_word(&self.keywords.export.str) {
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

    /// Lex parenthesized expression
    fn lex_parend(&mut self, iter: &mut LexingIterator) -> Result<Option<ParendLE>> {
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

        Ok(Some(ParendLE {
            range: RangeL::new(begin, end),
            contents: innards,
        }))
    }

    /// Lex curly braced block
    fn lex_curlied(&mut self, iter: &mut LexingIterator, stop_on_open_brace: bool) -> Result<Option<CurliedLE>> {
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

    /// Lex square bracketed expression
    fn lex_squared(&mut self, iter: &mut LexingIterator) -> Result<Option<SquaredLE>> {
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

    /// Lex angle bracketed expression (generics)
    fn lex_angled(&mut self, iter: &mut LexingIterator) -> Result<Option<AngledLE>> {
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
        if iter.position > 0 && iter.code.chars().nth(iter.position - 1) == Some('=') && c == '>' {
            return false;
        }

        // If whitespace on both sides, it's a comparison operator
        let whitespace_before = if iter.position > 0 {
            matches!(iter.code.chars().nth(iter.position - 1), Some(' ' | '\t' | '\n' | '\r'))
        } else {
            false
        };

        let whitespace_after = if iter.position + 1 < iter.code.len() {
            matches!(iter.code.chars().nth(iter.position + 1), Some(' ' | '\t' | '\n' | '\r'))
        } else {
            false
        };

        let whitespace_on_both_sides = whitespace_before && whitespace_after;
        !whitespace_on_both_sides
    }

    /// Check if we're at the end of a scramble
    fn at_end(&self, iter: &LexingIterator, stop_on_open_brace: bool, stop_on_where: bool, stop_on_semicolon: bool) -> bool {
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

    /// Lex a scramble of nodes (unstructured sequence)
    pub fn lex_scramble(&mut self, iter: &mut LexingIterator, stop_on_open_brace: bool, stop_on_where: bool, stop_on_semicolon: bool) -> Result<ScrambleLE> {
        let begin = iter.get_pos();

        iter.consume_comments_and_whitespace();

        let mut innards = Vec::new();

        while !self.at_end(iter, stop_on_open_brace, stop_on_where, stop_on_semicolon) {
            let node = self.lex_node(iter, stop_on_open_brace, stop_on_where)?;
            innards.push(Box::new(node));

            iter.consume_comments_and_whitespace();
        }

        let end = iter.get_pos();

        Ok(ScrambleLE {
            range: RangeL::new(begin, end),
            elements: innards,
        })
    }

    /// Lex a single node
    fn lex_node(&mut self, iter: &mut LexingIterator, stop_on_open_brace: bool, stop_on_where: bool) -> Result<INodeLEEnum> {
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

    /// Lex an atomic element (identifier, number, string, symbol)
    fn lex_atom(&mut self, iter: &mut LexingIterator, stop_on_where: bool) -> Result<INodeLEEnum> {
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
        
        // Try identifier
        if iter.peek().is_alphabetic() || iter.peek() == '_' {
            if let Some(id) = self.lex_identifier(iter) {
                return Ok(INodeLEEnum::Word(id));
            }
        }

        // Otherwise it's a symbol
        let c = iter.advance();
        Ok(INodeLEEnum::Symbol(SymbolLE {
            range: RangeL::new(begin, iter.get_pos()),
            c,
        }))
    }

    /// Lex an identifier
    fn lex_identifier(&mut self, iter: &mut LexingIterator) -> Option<WordLE> {
        let begin = iter.get_pos();
        
        if iter.at_end() {
            return None;
        }

        let first_char = iter.peek();
        if !first_char.is_alphabetic() && first_char != '_' {
            return None;
        }

        // Keep eating identifier characters
        while !iter.at_end() {
            let c = iter.peek();
            if c.is_alphanumeric() || c == '_' {
                iter.advance();
            } else {
                break;
            }
        }

        let end = iter.get_pos();
        let word = &iter.code[begin as usize..end as usize];
        
        if word.is_empty() {
            None
        } else {
            Some(WordLE {
                range: RangeL::new(begin, end),
                str: self.interner.lock().unwrap().intern(word),
            })
        }
    }

    /// Lex a number (integer or float)
    fn lex_number(&mut self, iter: &mut LexingIterator) -> Result<Option<INodeLEEnum>> {
        let begin = iter.get_pos();

        // Check if preceded by a dot (for array access like arr.2.1)
        let is_name = iter.position > 0 && iter.code.chars().nth(iter.position - 1) == Some('.');

        let tentative_pos = iter.position;
        let negative = iter.try_skip('-');

        let peeked = iter.peek();
        if !peeked.is_ascii_digit() {
            iter.position = tentative_pos;
            return Ok(None);
        }

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
                // Float type suffix (ignore for now)
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
            if bits > 0 {
                Some(bits)
            } else {
                None
            }
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

    /// Lex a string literal (with interpolation support)
    fn lex_string(&mut self, iter: &mut LexingIterator) -> Result<Option<INodeLEEnum>> {
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
                            s: string_so_far.clone(),
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
                s: string_so_far,
            });
        }

        Ok(Some(INodeLEEnum::String(StringLE {
            range: RangeL::new(begin, iter.get_pos()),
            parts,
        })))
    }

    /// Check if we're at the end of a string
    fn lex_string_end(&mut self, iter: &mut LexingIterator, is_long_string: bool) -> bool {
        if iter.at_end() {
            return true;
        }
        
        if is_long_string {
            iter.try_skip_str("\"\"\"")
        } else {
            iter.try_skip('"')
        }
    }

    /// Lex a part of a string (character or interpolated expression)
    fn lex_string_part(&mut self, iter: &mut LexingIterator, _string_begin_pos: i32) -> Result<StringPartResult> {
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
                let num = self.parse_four_digit_hex_num(iter, 0)
                    .ok_or(ParseError::BadUnicodeChar(iter.get_pos()))?;
                Ok(StringPartResult::Char(char::from_u32(num as u32).unwrap_or('\u{FFFD}')))
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
}

/// Helper enum for string parsing
enum StringPartResult {
    Char(char),
    Expr(ScrambleLE),
}

