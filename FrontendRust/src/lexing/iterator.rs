use super::ast::RangeL;

/// Lexing iterator for traversing source code
/// Matches Scala's LexingIterator
#[derive(Clone, Debug)]
pub struct LexingIterator {
    pub code: String,
    pub position: usize,  // Byte position in the string
    pub comments: Vec<RangeL>,
}

impl LexingIterator {
    pub fn new(code: String) -> Self {
        LexingIterator {
            code,
            position: 0,
            comments: Vec::new(),
        }
    }

    pub fn at_end(&self) -> bool {
        self.position >= self.code.len()
    }

    pub fn get_pos(&self) -> i32 {
        self.position as i32
    }

    /// Get the rest of the code from current position (for debugging)
    pub fn rest(&self) -> &str {
        &self.code[self.position..]
    }

    /// Peek at the current character without advancing
    pub fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.code[self.position..].chars().next().unwrap_or('\0')
        }
    }

    /// Peek ahead n characters (returns String)
    pub fn peek_n(&self, n: usize) -> Option<String> {
        if self.position + n > self.code.len() {
            None
        } else {
            Some(self.code[self.position..std::cmp::min(self.position + n, self.code.len())].to_string())
        }
    }

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

    /// Try to skip a specific character
    pub fn try_skip(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Try to skip a specific string
    pub fn try_skip_str(&mut self, s: &str) -> bool {
        if self.code[self.position..].starts_with(s) {
            self.position += s.len();
            true
        } else {
            false
        }
    }

    /// Skip to a specific position
    pub fn skip_to(&mut self, pos: usize) {
        self.position = pos;
    }

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

    /// Peek if a string matches (without advancing)
    pub fn peek_string(&self, s: &str) -> bool {
        self.code[self.position..].starts_with(s)
    }

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

    /// Consume comments and whitespace
    pub fn consume_comments_and_whitespace(&mut self) {
        self.consume_comments();
        self.consume_whitespace();
    }

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

    /// Consume all types of comments
    fn consume_comments(&mut self) {
        self.consume_line_comments();
        self.consume_chevron_comments();
        self.consume_ellipses_comments();
    }

    /// Consume line comments (//)
    fn consume_line_comments(&mut self) {
        let pos_after_whitespace = self.find_whitespace_end();
        
        // Use starts_with for Unicode-safe prefix checking
        if self.code[pos_after_whitespace..].starts_with("//") {
            let begin = self.position;
            self.position = pos_after_whitespace + 2;  // "//" is always 2 bytes (ASCII)
            
            // Skip to end of line
            while !self.at_end() {
                let c = self.advance();
                if c == '\n' {
                    break;
                }
            }
            
            self.comments.push(RangeL {
                begin: begin as i32,
                end: self.position as i32,
            });
            
            self.consume_comments();
        }
    }

    /// Consume chevron comments (« »)
    fn consume_chevron_comments(&mut self) {
        let pos_after_whitespace = self.find_whitespace_end();
        
        if pos_after_whitespace < self.code.len() 
            && self.code[pos_after_whitespace..].starts_with('«') {
            let begin = self.position;
            self.position = pos_after_whitespace + '«'.len_utf8();
            
            self.skip_to_past('»');
            
            self.comments.push(RangeL {
                begin: begin as i32,
                end: self.position as i32,
            });
            
            self.consume_comments();
        }
    }

    /// Consume ellipses comments (... or …)
    /// Note: Ellipses are treated as placeholder tokens, not line comments
    fn consume_ellipses_comments(&mut self) {
        let pos_after_whitespace = self.find_whitespace_end();
        
        // Check for "..." (three ASCII dots) or "…" (Unicode ellipsis U+2026)
        // Use starts_with for proper Unicode handling
        let rest_of_code = &self.code[pos_after_whitespace..];
        let has_ellipsis = rest_of_code.starts_with("...");
        let has_unicode_ellipsis = rest_of_code.starts_with('…');
        
        if has_ellipsis || has_unicode_ellipsis {
            let begin = self.position;
            self.position = if has_ellipsis {
                pos_after_whitespace + 3  // Skip "..."
            } else {
                pos_after_whitespace + '…'.len_utf8()  // Skip "…" (3 bytes)
            };
            
            // Unlike line comments, ellipses don't extend to end of line
            // They're just consumed as placeholder tokens (Scala line 103)
            self.comments.push(RangeL {
                begin: begin as i32,
                end: self.position as i32,
            });
            
            self.consume_comments();
        }
        
        // Also try to skip "..." unconditionally (Scala line 107)
        self.try_skip_str("...");
    }

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
}

