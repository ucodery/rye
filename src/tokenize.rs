use std::cmp;

use crate::tokens;

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
}

impl Iterator for TokenStream<'_> {
    type Item = tokens::Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cursor > self.input_end {
                return None;
            }
            let largest_window = cmp::min(self.input_end - self.cursor, 3);
            let tok = tokens::get_op(&self.input[self.cursor..self.cursor + largest_window]);
            match tok {
                Some(tokens::Token {
                    token_type: _,
                    size: Some(size),
                }) => {
                    self.cursor += size;
                    return tok;
                }
                Some(tokens::Token {
                    token_type: _,
                    size: None,
                }) => return tok,
                None => self.cursor += 1,
            }
        }
    }
}
