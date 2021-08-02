use std::cmp;

use crate::tokens;

use unicode_categories::UnicodeCategories;

pub struct TokenStream<'a> {
    input: &'a str,
    input_end: usize,
    cursor: usize,
}

impl TokenStream<'_> {
    pub fn new(input: &str) -> TokenStream {
        TokenStream {
            input,
            input_end: input.chars().count(),
            cursor: 0,
        }
    }

    /// Attempt to consume the longest valid op token under the cursor
    /// advance the cursor if any op is found
    fn consume_next_op_token(&mut self) -> Option<tokens::Token> {
        let largest_window = cmp::min(self.input_end - self.cursor, 3);
        let tok = tokens::get_op(&self.input[self.cursor..self.cursor + largest_window]);
        match tok {
            Some(tokens::Token {
                token_type: _,
                size: Some(size),
            }) => self.cursor += size,
            _ => (),
        };
        tok
    }

    fn is_start_of_name(c: char) -> bool {
        if c.is_letter_uppercase() || c.is_letter_lowercase() || c.is_letter_titlecase() || c.is_letter_modifier() || c.is_letter_other() || c.is_number_letter() || c == '_'{
            // XXX: also chars with Other_ID_Start property
            true
        } else {
            false
        }
    }

    fn is_part_of_name(c: char) -> bool {
        if Self::is_start_of_name(c) || c.is_mark_nonspacing() || c.is_mark_spacing_combining() || c.is_number_decimal_digit() || c.is_punctuation_connector() {
            // XXX: also chars with Other_ID_Continue
            true
        } else {
            false
        }
    }

    fn consume_next_name_token(&mut self) -> Option<tokens::Token> {
        let next_char = self.input[self.cursor..self.cursor+1].chars().next().unwrap();
        if !Self::is_start_of_name(next_char) {
            return None
        }

        let mut name_end = self.cursor + 1;
        while Self::is_part_of_name(self.input[name_end..name_end+1].chars().next().unwrap()) {
            name_end += 1;
        }
        let tok = Some(tokens::Token {token_type: tokens::TokenType::NAME, size: Some(name_end - self.cursor)});
        self.cursor = name_end;
        tok
    }
}

impl Iterator for TokenStream<'_> {
    type Item = tokens::Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // check for end
            if self.cursor > self.input_end {
                return None
            }
            if self.cursor == self.input_end {
                self.cursor += 1;
                return Some(tokens::Token {token_type: tokens::TokenType::ENDMARKER, size: None})
            }

            if let Some(tok) = self.consume_next_op_token() {
                return Some(tok)
            };
            if let Some(tok) = self.consume_next_name_token() {
                return Some(tok)
            };

            // no tokens found
            self.cursor += 1;
            return Some(tokens::Token {token_type: tokens::TokenType::ERRORTOKEN, size: None})
        }
    }
}
