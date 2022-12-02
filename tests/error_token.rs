#![feature(assert_matches)]
use std::assert_matches::assert_matches;

use rstest::*;

use rye::tokenize::TokenStream;
use rye::tokens::{Token, TokenType};

pub mod common;
use common::source_to_tokens;

#[rstest]
#[case("'''''")]
#[case("''''")]
#[case(
    "
\"\"\"
    rye
"
)]
#[case("(")]
#[case("[")]
#[case("{")]
fn tokenizing_error(#[case] source: &str) {
    let maybe_tokens = TokenStream::new(source);
    assert_matches!(
        maybe_tokens
            .into_iter()
            .collect::<Result<Vec<Token>, String>>(),
        Err(_)
    );
}

#[rstest]
#[case("?")]
#[case("!")]
#[case("'  rye  ")]
#[case("'  rye  \n")]
#[case("'  rye  \n'")]
#[case("'rye\"")]
#[case("'rye\\'")]
#[case("\"  rye  ")]
#[case("\"  rye  \n")]
#[case("\"  rye  \n\"")]
#[case("\"rye'")]
#[case("\"rye\\\"")]
fn errortoken(#[case] source: &str) {
    let tokens = source_to_tokens(source);
    assert!(
        tokens.len() > 0,
        "Too few tokens found: {}",
        format!("{:?}", tokens)
    );

    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = tokens[0];
    assert_eq!(
        token_type,
        TokenType::ERRORTOKEN,
        "Symbol Token not of type ERRORTOKEN, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::ERRORTOKEN,
        "Symbol Token not of exact type ERRORTOKEN, got type {}",
        format!("{:?}", token_type)
    );

    for token in tokens {
        let Token {
            token_type,
            exact_token_type: _,
            token_contents: _,
            col_start: _,
            col_end: _,
        } = token;
        assert_ne!(token_type, TokenType::STRING, "Got unexpected STRING Token");
    }
}
