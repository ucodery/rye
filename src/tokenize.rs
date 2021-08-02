use crate::tokens::{get_op, Token, TokenType};

use unicode_categories::UnicodeCategories;

pub struct TokenStream {
    input: Vec<char>,
    cursor: usize,
}

impl TokenStream {
    pub fn new(input: &str) -> TokenStream {
        TokenStream {
            input: input.chars().collect(),
            cursor: 0,
        }
    }

    /// Attempt to consume the longest valid op token under the cursor
    /// advance the cursor if any op is found
    fn consume_next_op_token(&mut self) -> Option<Token> {
        let tok = get_op(&self.input[self.cursor..]);
        match tok {
            Some(Token {
                token_type: _,
                size: Some(size),
            }) => self.cursor += size,
            _ => (),
        };
        tok
    }

    fn is_start_of_name(c: &char) -> bool {
        if c.is_letter_uppercase()
            || c.is_letter_lowercase()
            || c.is_letter_titlecase()
            || c.is_letter_modifier()
            || c.is_letter_other()
            || c.is_number_letter()
            || *c == '_'
        {
            // XXX: also chars with Other_ID_Start property
            true
        } else {
            false
        }
    }

    fn is_part_of_name(c: &char) -> bool {
        if Self::is_start_of_name(c)
            || c.is_mark_nonspacing()
            || c.is_mark_spacing_combining()
            || c.is_number_decimal_digit()
            || c.is_punctuation_connector()
        {
            // XXX: also chars with Other_ID_Continue
            true
        } else {
            false
        }
    }

    fn consume_next_name_token(&mut self) -> Option<Token> {
        let next_char = self.input[self.cursor..=self.cursor].first().unwrap();
        if !Self::is_start_of_name(next_char) {
            return None;
        }

        let mut name_end = self.cursor + 1;
        while Self::is_part_of_name(self.input[name_end..=name_end].first().unwrap()) {
            name_end += 1;
        }
        let tok = Some(Token {
            token_type: TokenType::NAME,
            size: Some(name_end - self.cursor),
        });
        self.cursor = name_end;
        tok
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // check for end
            if self.cursor > self.input.len() {
                return None;
            }
            if self.cursor == self.input.len() {
                self.cursor += 1;
                return Some(Token {
                    token_type: TokenType::ENDMARKER,
                    size: None,
                });
            }

            if let Some(tok) = self.consume_next_op_token() {
                return Some(tok);
            };
            if let Some(tok) = self.consume_next_name_token() {
                return Some(tok);
            };

            // no tokens found
            self.cursor += 1;
            return Some(Token {
                token_type: TokenType::ERRORTOKEN,
                size: None,
            });
        }
    }
}
