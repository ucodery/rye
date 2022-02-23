use crate::tokens::{Token, TokenType};
use std::cmp;

use unicode_categories::UnicodeCategories;

struct RawSource {
    input: Vec<char>,
    cursor: usize,
    lookahead: usize,
}

impl RawSource {
    pub fn new(input: &str) -> RawSource {
        RawSource {
            input: input.chars().collect(),
            cursor: 0,
            lookahead: 0,
        }
    }

    fn at_end(&self) -> bool {
        self.cursor >= self.input.len()
    }

    fn committed_index(&self) -> usize {
        self.cursor
    }

    fn peeked_index(&self) -> usize {
        self.lookahead
    }

    /// return a slice of input stream of size window or the rest of the input
    /// starting from the last peek
    fn peek(&mut self, window: usize) -> &[char] {
        let window_open = cmp::min(self.lookahead, self.input.len());
        let window_close = cmp::min(self.lookahead + window, self.input.len());
        self.lookahead += window;
        &self.input[window_open..window_close]
    }

    fn hide(&mut self, window: usize) {
        self.lookahead -= window;
        assert!(
            self.lookahead >= self.cursor,
            "Cannot hide what was already consumed"
        );
    }

    fn commit(&mut self) {
        self.cursor = cmp::min(self.lookahead, self.input.len());
    }

    fn revert(&mut self) {
        self.lookahead = self.cursor;
    }

    fn peeked_string(&self) -> String {
        return self.input[self.cursor..self.lookahead].iter().collect();
    }
}

pub struct TokenStream {
    source: RawSource,
    within_statement: bool,
    indents_seen: Vec<usize>,
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(input: &str) -> TokenStream {
        TokenStream {
            source: RawSource::new(input),
            within_statement: false,
            indents_seen: vec![0],
            tokens: Vec::new(),
        }
    }

    fn add_token(
        &mut self,
        token_type: TokenType,
        exact_token_type: TokenType,
        token_contents: String,
        col_start: usize,
        col_end: usize,
    ) {
        self.tokens.push(Token {
            token_type,
            exact_token_type,
            token_contents,
            col_start,
            col_end,
        });

    }

    fn commit_to_token(&mut self, token_type: TokenType, exact_token_type: TokenType) {
        self.add_token(
            token_type,
            exact_token_type,
            self.source.peeked_string(),
            self.source.committed_index(),
            self.source.peeked_index(),
        );
        self.source.commit();
    }

    /// Attempt to consume the longest valid op token from the source
    /// advance the cursor if any op is found
    fn consume_next_op_token(&mut self) -> bool {
        let exact_token_type: TokenType;
        match self.source.peek(3) {
            ['*', '*', '='] => exact_token_type = TokenType::DOUBLESTAREQUAL,
            ['.', '.', '.'] => exact_token_type = TokenType::ELLIPSIS,
            ['/', '/', '='] => exact_token_type = TokenType::DOUBLESLASHEQUAL,
            ['<', '<', '='] => exact_token_type = TokenType::LEFTSHIFTEQUAL,
            ['>', '>', '='] => exact_token_type = TokenType::RIGHTSHIFTEQUAL,
            ['!', '=', ..] => {
                exact_token_type = TokenType::NOTEQUAL;
                self.source.hide(1);
            }
            ['%', '=', ..] => {
                exact_token_type = TokenType::PERCENTEQUAL;
                self.source.hide(1);
            }
            ['&', '=', ..] => {
                exact_token_type = TokenType::AMPEREQUAL;
                self.source.hide(1);
            }
            ['*', '*', ..] => {
                exact_token_type = TokenType::DOUBLESTAR;
                self.source.hide(1);
            }
            ['*', '=', ..] => {
                exact_token_type = TokenType::STAREQUAL;
                self.source.hide(1);
            }
            ['+', '=', ..] => {
                exact_token_type = TokenType::PLUSEQUAL;
                self.source.hide(1);
            }
            ['-', '=', ..] => {
                exact_token_type = TokenType::MINEQUAL;
                self.source.hide(1);
            }
            ['-', '>', ..] => {
                exact_token_type = TokenType::RARROW;
                self.source.hide(1);
            }
            ['/', '/', ..] => {
                exact_token_type = TokenType::DOUBLESLASH;
                self.source.hide(1);
            }
            ['/', '=', ..] => {
                exact_token_type = TokenType::SLASHEQUAL;
                self.source.hide(1);
            }
            [':', '=', ..] => {
                exact_token_type = TokenType::COLONEQUAL;
                self.source.hide(1);
            }
            ['<', '<', ..] => {
                exact_token_type = TokenType::LEFTSHIFT;
                self.source.hide(1);
            }
            ['<', '=', ..] => {
                exact_token_type = TokenType::LESSEQUAL;
                self.source.hide(1);
            }
            ['<', '>', ..] => {
                exact_token_type = TokenType::NOTEQUAL;
                self.source.hide(1);
            }
            ['=', '=', ..] => {
                exact_token_type = TokenType::EQEQUAL;
                self.source.hide(1);
            }
            ['>', '=', ..] => {
                exact_token_type = TokenType::GREATEREQUAL;
                self.source.hide(1);
            }
            ['>', '>', ..] => {
                exact_token_type = TokenType::RIGHTSHIFT;
                self.source.hide(1);
            }
            ['@', '=', ..] => {
                exact_token_type = TokenType::ATEQUAL;
                self.source.hide(1);
            }
            ['^', '=', ..] => {
                exact_token_type = TokenType::CIRCUMFLEXEQUAL;
                self.source.hide(1);
            }
            ['|', '=', ..] => {
                exact_token_type = TokenType::VBAREQUAL;
                self.source.hide(1);
            }
            ['%', ..] => {
                exact_token_type = TokenType::PERCENT;
                self.source.hide(2);
            }
            ['&', ..] => {
                exact_token_type = TokenType::AMPER;
                self.source.hide(2);
            }
            ['(', ..] => {
                exact_token_type = TokenType::LPAR;
                self.source.hide(2);
            }
            [')', ..] => {
                exact_token_type = TokenType::RPAR;
                self.source.hide(2);
            }
            ['*', ..] => {
                exact_token_type = TokenType::STAR;
                self.source.hide(2);
            }
            ['+', ..] => {
                exact_token_type = TokenType::PLUS;
                self.source.hide(2);
            }
            [',', ..] => {
                exact_token_type = TokenType::COMMA;
                self.source.hide(2);
            }
            ['-', ..] => {
                exact_token_type = TokenType::MINUS;
                self.source.hide(2);
            }
            ['.', ..] => {
                exact_token_type = TokenType::DOT;
                self.source.hide(2);
            }
            ['/', ..] => {
                exact_token_type = TokenType::SLASH;
                self.source.hide(2);
            }
            [':', ..] => {
                exact_token_type = TokenType::COLON;
                self.source.hide(2);
            }
            [';', ..] => {
                exact_token_type = TokenType::SEMI;
                self.source.hide(2);
            }
            ['<', ..] => {
                exact_token_type = TokenType::LESS;
                self.source.hide(2);
            }
            ['=', ..] => {
                exact_token_type = TokenType::EQUAL;
                self.source.hide(2);
            }
            ['>', ..] => {
                exact_token_type = TokenType::GREATER;
                self.source.hide(2);
            }
            ['@', ..] => {
                exact_token_type = TokenType::AT;
                self.source.hide(2);
            }
            ['[', ..] => {
                exact_token_type = TokenType::LSQB;
                self.source.hide(2);
            }
            [']', ..] => {
                exact_token_type = TokenType::RSQB;
                self.source.hide(2);
            }
            ['^', ..] => {
                exact_token_type = TokenType::CIRCUMFLEX;
                self.source.hide(2);
            }
            ['{', ..] => {
                exact_token_type = TokenType::LBRACE;
                self.source.hide(2);
            }
            ['|', ..] => {
                exact_token_type = TokenType::VBAR;
                self.source.hide(2);
            }
            ['}', ..] => {
                exact_token_type = TokenType::RBRACE;
                self.source.hide(2);
            }
            ['~', ..] => {
                exact_token_type = TokenType::TILDE;
                self.source.hide(2);
            }
            _ => {
                self.source.revert();
                return false;
            }
        }
        self.commit_to_token(TokenType::OP, exact_token_type);
        true
    }

    fn is_start_of_name(c: &char) -> bool {
        // XXX: also chars with Other_ID_Start property
        c.is_letter_uppercase()
            || c.is_letter_lowercase()
            || c.is_letter_titlecase()
            || c.is_letter_modifier()
            || c.is_letter_other()
            || c.is_number_letter()
            || *c == '_'
    }

    fn is_part_of_name(c: &char) -> bool {
        // XXX: also chars with Other_ID_Continue
        Self::is_start_of_name(c)
            || c.is_mark_nonspacing()
            || c.is_mark_spacing_combining()
            || c.is_number_decimal_digit()
            || c.is_punctuation_connector()
    }

    /// Attempt to consume the longest valid name token from the source
    /// advance the cursor if any name is found
    fn consume_next_name_token(&mut self) -> bool {
        if let [next] = self.source.peek(1) {
            if !Self::is_start_of_name(next) {
                self.source.hide(1);
                return false;
            }
        } else {
            self.source.hide(1);
            return false;
        };

        loop {
            if let [next] = self.source.peek(1) {
                if !Self::is_part_of_name(next) {
                    self.source.hide(1);
                    break;
                }
            } else {
                self.source.hide(1);
                break;
            };
        }

        self.commit_to_token(TokenType::NAME, TokenType::NAME);
        true
    }

    fn is_bin_digit(c: &char) -> bool {
        *c == '0' || *c == '1'
    }

    fn is_oct_digit(c: &char) -> bool {
        Self::is_bin_digit(c)
            || *c == '2'
            || *c == '3'
            || *c == '4'
            || *c == '5'
            || *c == '6'
            || *c == '7'
    }

    fn is_dec_digit(c: &char) -> bool {
        Self::is_oct_digit(c) || *c == '8' || *c == '9'
    }

    fn is_hex_digit(c: &char) -> bool {
        Self::is_dec_digit(c)
            || *c == 'a'
            || *c == 'b'
            || *c == 'c'
            || *c == 'd'
            || *c == 'e'
            || *c == 'f'
            || *c == 'A'
            || *c == 'B'
            || *c == 'C'
            || *c == 'D'
            || *c == 'E'
            || *c == 'F'
    }

    /// only to be called after the smallest matching sequence of characters has been found
    /// for non-decimal integers this means "0[bBoOxX][0-9a-fA-F]" has already been matched
    /// for decimal integers just the first digit has already been matched
    fn find_end_of_integer(&mut self, valid_digit: fn(&char) -> bool) {
        let mut last_under = false;
        loop {
            match self.source.peek(1) {
                ['_'] => {
                    if last_under {
                        // neither of these "_" are part of a number
                        self.source.hide(2);
                        return;
                    } else {
                        last_under = true;
                    }
                }
                [next] if valid_digit(next) => {
                    last_under = false;
                }
                _ => {
                    if last_under {
                        self.source.hide(2);
                    } else {
                        self.source.hide(1);
                    };
                    return;
                }
            }
        }
    }

    /// only to be called after a potential exponent was found after an already valid number token
    /// this means that "[0-9][.]?[eE]" has already been matched
    fn find_end_of_exponent(&mut self) -> bool {
        match self.source.peek(2) {
            [next, ..] if Self::is_dec_digit(next) => {
                self.source.hide(1); // last peek was unchecked, leave it to next call
                self.find_end_of_integer(Self::is_dec_digit);
                true
            }
            ['-' | '+', next] if Self::is_dec_digit(next) => {
                self.find_end_of_integer(Self::is_dec_digit);
                true
            }
            _ => {
                // also hide the "e" that was already matched
                // this returns the caller's view of the token to a valid end of number
                self.source.hide(3);
                false
            }
        }
    }

    /// Attempt to consume the longest valid number token from the source
    /// advance the cursor if any name is found
    fn consume_next_number_token(&mut self) -> bool {
        let number_type: TokenType;

        match self.source.peek(1) {
            ['0'] => {
                match self.source.peek(2) {
                    ['b' | 'B', next] if Self::is_bin_digit(next) => {
                        number_type = TokenType::BININT;
                        self.find_end_of_integer(Self::is_bin_digit);
                    }
                    ['o' | 'O', next] if Self::is_oct_digit(next) => {
                        number_type = TokenType::OCTINT;
                        self.find_end_of_integer(Self::is_oct_digit);
                    }
                    ['x' | 'X', next] if Self::is_hex_digit(next) => {
                        number_type = TokenType::HEXINT;
                        self.find_end_of_integer(Self::is_hex_digit);
                    }
                    [next, ..] if Self::is_dec_digit(next) => {
                        let is_zero = *next == '0';
                            // put unchecked char back
                        self.source.hide(1);
                        if is_zero {
                            // this is the longest possible integer token as only zero can have
                            // leading 0s
                            self.find_end_of_integer(|c| *c == '0');
                        };
                        match self.source.peek(1) {
                            ['.'] => {
                                // found fraction with integer part zero
                                number_type = TokenType::FLOAT;
                                self.find_end_of_integer(Self::is_dec_digit);
                                if let ['e' | 'E'] = self.source.peek(1) {
                                    self.find_end_of_exponent();
                                } else {
                                    self.source.hide(1);
                                };
                            }
                            ['e' | 'E'] => {
                                if self.find_end_of_exponent() {
                                    // found exponent with base part zero
                                    number_type = TokenType::FLOAT;
                                } else {
                                    // found decimal number zero spelled with multiple 0s
                                    self.source.hide(1);
                                    number_type = TokenType::INTEGER;
                                };
                            }
                            [next] if Self::is_dec_digit(next) => {
                                // 0 digits are certain to be part of one token but non-0 digits
                                // are only part of the same token if it ends up being a float or
                                // imaginary
                                let last_zero = self.source.peeked_index() - 1;
                                self.find_end_of_integer(Self::is_dec_digit);
                                match self.source.peek(1) {
                                    ['.'] => {
                                        // found fraction with integer part non-zero but leading 0s
                                        number_type = TokenType::FLOAT;
                                        self.find_end_of_integer(Self::is_dec_digit);
                                        if let ['e' | 'E'] = self.source.peek(1) {
                                            self.find_end_of_exponent();
                                        } else {
                                            self.source.hide(1);
                                        };
                                    }
                                    ['e' | 'E'] => {
                                        if self.find_end_of_exponent() {
                                            // found exponent with base part non-zero but with
                                            // leading 0s
                                            number_type = TokenType::FLOAT;
                                        } else {
                                            // found decimal number zero spelled with multiple 0s
                                            number_type = TokenType::INTEGER;
                                            self.source
                                                .hide(self.source.peeked_index() - last_zero);
                                        };
                                    }
                                    ['j' | 'J'] => {
                                        // found imaginary whole number with leading zeros
                                        // identification of imaginary tokens is normally done at
                                        // the end of this function, but intigers and imaginary
                                        // whole number tokens have different lexing rules around 0
                                        number_type = TokenType::IMAGINARY;
                                    }
                                    _ => {
                                        // found decimal number zero spelled with multiple 0s
                                        number_type = TokenType::INTEGER;
                                        self.source.hide(self.source.peeked_index() - last_zero);
                                    }
                                };
                            }
                            _ => {
                                // found decimal number zero spelled with multiple 0s
                                number_type = TokenType::INTEGER;
                                self.source.hide(1);
                            }
                        };
                    }
                    ['.', ..] => {
                        // put unchecked char back
                        self.source.hide(1);
                        number_type = TokenType::FLOAT;
                        self.find_end_of_integer(Self::is_dec_digit);
                        if let ['e' | 'E'] = self.source.peek(1) {
                            self.find_end_of_exponent();
                        } else {
                            self.source.hide(1);
                        };
                    }
                    ['e' | 'E', ..] => {
                        // put unchecked char back
                        self.source.hide(1);
                        if self.find_end_of_exponent() {
                            // found exponent with base part zero
                            number_type = TokenType::FLOAT;
                        } else {
                            // found decimal number zero spelled with one 0
                            number_type = TokenType::INTEGER;
                        };
                    }
                    _ => {
                        // found decimal number zero spelled with one 0
                        number_type = TokenType::INTEGER;
                        self.source.hide(2);
                    }
                };
            }
            ['1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'] => {
                self.find_end_of_integer(Self::is_dec_digit);
                println!("{:?}", self.source.peeked_string());
                match self.source.peek(1) {
                    ['.'] => {
                        // found fraction with integer part
                        number_type = TokenType::FLOAT;
                        self.find_end_of_integer(Self::is_dec_digit);
                        if let ['e' | 'E'] = self.source.peek(1) {
                            self.find_end_of_exponent();
                        } else {
                            self.source.hide(1);
                        };
                    }
                    ['e' | 'E'] => {
                        if self.find_end_of_exponent() {
                            // found exponent with base part zero
                            number_type = TokenType::FLOAT;
                        } else {
                            // found decimal number zero spelled with multiple 0s
                            number_type = TokenType::INTEGER;
                        };
                    }
                    _ => {
                        // found decimal number non-zero
                        number_type = TokenType::INTEGER;
                        self.source.hide(1);
                    }
                };
                println!("{:?}", self.source.peeked_string());
            }
            ['.'] => {
                match self.source.peek(1) {
                    [next] if Self::is_dec_digit(next) => {
                        // found float with no integer part
                        number_type = TokenType::FLOAT;
                        self.find_end_of_integer(Self::is_dec_digit);
                        if let ['e' | 'E'] = self.source.peek(1) {
                            self.find_end_of_exponent();
                        } else {
                            self.source.hide(1);
                        };
                    }
                    _ => {
                        // just a dot
                        self.source.revert();
                        return false;
                    }
                }
            }
            _ => {
                // no number here
                self.source.revert();
                return false;
            }
        };

        let exact_token_type: TokenType;
        if let TokenType::INTEGER | TokenType::FLOAT = number_type {
            if let ['j' | 'J'] = self.source.peek(1) {
                exact_token_type = TokenType::IMAGINARY;
            } else {
                self.source.hide(1);
                exact_token_type = number_type;
            };
        } else {
            exact_token_type = number_type;
        };

        self.commit_to_token(TokenType::NUMBER, exact_token_type);
        true
    }

    /// Attempt to consume a newline
    /// advance the cursor if a newline is detected
    fn consume_next_newline(&mut self) -> Option<bool> {
        return match self.source.peek(2) {
            ['\\', '\n'] => {
                // no tokens produced when newline escaped
                self.source.commit();
                Some(false)
            }
            ['\n', ..] => {
                self.source.hide(1);
                if self.within_statement {
                    self.commit_to_token(TokenType::NEWLINE, TokenType::NEWLINE);
                    self.within_statement = false;
                    Some(true)
                } else {
                    self.commit_to_token(TokenType::NL, TokenType::NL);
                    Some(true)
                }
            }
            _ => {
                self.source.revert();
                None
            }
        };
    }

    /// Attempt to consume leading whitespace from a logical line in the source
    /// advance the cursor if any name is found
    /// This must be called first after every NEWLINE but not after other tokens
    fn consume_next_dent(&mut self) -> Result<bool, String> {
        let mut spaces: usize = 0;
        let mut no_more_source = true;
        while let [next] = self.source.peek(1) {
            if *next == ' ' {
                spaces += 1;
            } else if *next == '\t' {
                // round up to the next multiple of 8 spaces
                spaces += 8 - (spaces % 8);
            } else if *next == '\n' || *next == '\\' || *next == '#' {
                // there is no code on this line and no tokens are produced from any indent
                // any indent does not have to line up with any other line and has no significance
                self.source.hide(1);
                self.source.commit();
                return Ok(false);
            } else {
                no_more_source = false;
                break;
            };
        }
        self.source.hide(1);
        if no_more_source {
            // there is no code on this line
            self.source.commit();
            return Ok(false);
        };

        match *self.indents_seen.last().unwrap() {
            s if s == spaces => {
                // this line is the same indentation level as the current block
                self.source.commit();
                Ok(false)
            }
            s if s < spaces => {
                self.indents_seen.push(spaces);
                self.commit_to_token(TokenType::INDENT, TokenType::INDENT);
                Ok(true)
            }
            _ => {
                let last_indent = self.indents_seen.pop().unwrap();
                if last_indent < spaces {
                    return Err(String::from(
                        "unindent does not match any outer indentation level",
                    ));
                };
                self.commit_to_token(TokenType::DEDENT, TokenType::DEDENT);
                Ok(true)
            }
        }
    }

    /// Attempt to consume any whitespace from the source
    /// advance the cursor if any whitespace if found
    /// This is NOT for finding INDENT/DEDENT or NL/NEWLINE tokens
    fn consume_next_whitespace(&mut self) -> bool {
        while let [next] = self.source.peek(1) {
            // space, tab, and formfeed are valid inter-token whitespace
            if *next != ' ' && *next != '\t' && *next != '\u{000C}' {
                break;
            };
        }
        // everything but the last peek was whitespace
        self.source.hide(1);
        self.source.commit();
        false
    }

    fn consume_next_comment(&mut self) -> bool {
        if let [next] = self.source.peek(1) {
            if *next == '#' {
                while let [next] = self.source.peek(1) {
                    if *next == '\n' {
                        break;
                    };
                }
                self.source.hide(1);
                self.commit_to_token(TokenType::COMMENT, TokenType::COMMENT);
                return true;
            };
        };
        self.source.revert();
        false
    }

    fn find_end_tripple_quote(&mut self, end_match: [char; 3]) -> Result<(), String> {
        let mut last_escape = false;
        while let [a, b, c] = self.source.peek(3) {
            if [*a, *b, *c] == end_match && !last_escape {
                return Ok(());
            } else if [*a, *b, *c] == ['\\', '\\', '\\'] && !last_escape
                || *c == '\\' && *b != '\\'
                || *a == '\\' && !last_escape
            {
                last_escape = true;
            } else if last_escape {
                last_escape = false;
            };
            self.source.hide(2);
        }
        Err(String::from("EOF in multi-line string"))
    }

    fn find_end_quote(&mut self, end_match: [char; 1]) -> bool {
        let mut last_escape = false;
        while let [a] = self.source.peek(1) {
            if [*a] == end_match && !last_escape {
                return true;
            } else if [*a] == ['\n'] {
                return false;
            } else if [*a] == ['\\'] && !last_escape {
                last_escape = true;
            } else if last_escape {
                last_escape = false;
            };
        }
        false
    }

    fn consume_next_string_token(&mut self) -> Result<bool, String> {
        let qt: char;
        match self.source.peek(3) {
            [q, ..] if q == &'\'' || q == &'"' => {
                qt = *q;
                self.source.hide(2);
            }
            ['b' | 'B', q, ..] | ['f' | 'F', q, ..] | ['r' | 'R', q, ..] | ['u' | 'U', q, ..]
                if q == &'\'' || q == &'"' =>
            {
                qt = *q;
                self.source.hide(1);
            }
            ['r' | 'R', 'b' | 'B' | 'f' | 'F', q] | ['b' | 'B' | 'f' | 'F', 'r' | 'R', q]
                if q == &'\'' || q == &'"' =>
            {
                qt = *q;
            }
            _ => {
                self.source.revert();
                return Ok(false);
            }
        };
        match self.source.peek(2) {
            [a, b] if [*a, *b] == [qt, qt] => {
                if let Err(errmsg) = self.find_end_tripple_quote([qt, qt, qt]) {
                    return Err(errmsg);
                };
            }
            _ => {
                self.source.hide(2);
                if !self.find_end_quote([qt]) {
                    self.source.revert();
                    self.source.peek(1);
                    self.commit_to_token(TokenType::ERRORTOKEN, TokenType::ERRORTOKEN);
                    return Ok(true);
                };
            }
        };
        self.commit_to_token(TokenType::STRING, TokenType::STRING);
        Ok(true)
    }
}

impl Iterator for TokenStream {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.at_end() {
            if self.within_statement {
                // all statements must end in a newline, even if not present in the source
                self.within_statement = false;
                self.add_token(
                    TokenType::NEWLINE,
                    TokenType::NEWLINE,
                    String::from("\n"),
                    self.source.committed_index(),
                    self.source.committed_index() + 1,
                );
                return Some(Ok(self.tokens.last().unwrap().clone()));
            } else if self.indents_seen.len() > 1 {
                // bottom of the stack is indent of size 0 and does not need a DEDENT
                self.indents_seen.pop();
                self.add_token(
                    TokenType::DEDENT,
                    TokenType::DEDENT,
                    String::from(""),
                    self.source.committed_index() + 1,
                    self.source.committed_index() + 1,
                );
                return Some(Ok(self.tokens.last().unwrap().clone()));
            } else if let Some(Token {
                token_type: TokenType::ENDMARKER,
                exact_token_type: _,
                token_contents: _,
                col_start: _,
                col_end: _,
            }) = self.tokens.last()
            {
                return None;
            } else {
                self.add_token(
                    TokenType::ENDMARKER,
                    TokenType::ENDMARKER,
                    String::from(""),
                    self.source.committed_index() + 1,
                    self.source.committed_index() + 1,
                );
                return Some(Ok(self.tokens.last().unwrap().clone()));
            };
        };

        // consume any significant whitespace
        // may not produce a token, even if the cursor is advanced
        if !self.within_statement {
            match self.consume_next_dent() {
                Ok(true) => {
                    self.within_statement = true;
                    return Some(Ok(self.tokens.last().unwrap().clone()));
                }

                Ok(false) => (),
                Err(e) => return Some(Err(e)),
            };
        };
        if self.consume_next_whitespace() {
            // re-enter as there may be an indent after insignificant whitespace
            return self.next();
        };
        if let Some(produced_token) = self.consume_next_newline() {
            if produced_token {
                return Some(Ok(self.tokens.last().unwrap().clone()));
            } else {
                // re-enter; escaped newline is insignificant whitespace
                return self.next();
            };
        };
        // number must come before op to correctly capture a leading decimal point
        if self.consume_next_number_token() {
            self.within_statement = true;
            return Some(Ok(self.tokens.last().unwrap().clone()));
        };
        if self.consume_next_op_token() {
            self.within_statement = true;
            return Some(Ok(self.tokens.last().unwrap().clone()));
        };
        // string must come before name to correctly capture prefix directives
        match self.consume_next_string_token() {
            Ok(true) => {
                self.within_statement = true;
                return Some(Ok(self.tokens.last().unwrap().clone()));
            }
            Ok(false) => (),
            Err(e) => return Some(Err(e)),
        };
        if self.consume_next_name_token() {
            self.within_statement = true;
            return Some(Ok(self.tokens.last().unwrap().clone()));
        };
        if self.consume_next_comment() {
            return Some(Ok(self.tokens.last().unwrap().clone()));
        };

        // no tokens found
        if self.source.at_end() {
            return self.next();
        } else {
            self.source.peek(1);
            self.commit_to_token(TokenType::ERRORTOKEN, TokenType::ERRORTOKEN);
            return Some(Ok(self.tokens.last().unwrap().clone()));
        };
    }
}
