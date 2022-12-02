use rstest::*;

use rye::tokens::{Token, TokenType};

pub mod common;
use common::source_to_tokens;

#[rstest]
#[case("\n")]
#[case("    \n")]
#[case("\n\t")]
//#[case("\r")]
//#[case("\r\n")]
fn insignificant_newlines(#[case] source: &str) {
    let mut tokens = source_to_tokens(source);
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        TokenType::NL,
        "Symbol Token not of type NL, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::NL,
        "Symbol Token not of exact type NL, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("rye\n")]
#[case("rye\t\n")]
fn significant_newline_token(#[case] source: &str) {
    let mut tokens = source_to_tokens(source);
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        TokenType::NEWLINE,
        "Symbol Token not of type NEWLINE, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::NEWLINE,
        "Symbol Token not of exact type NEWLINE, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case(
    "rye(
)"
)]
fn logical_lines(#[case] source: &str) {
    let mut newlines_found = 0;
    let tokens = source_to_tokens(source);
    for Token {
        token_type,
        exact_token_type: _,
        token_contents: _,
        col_start: _,
        col_end: _,
    } in tokens.iter()
    {
        if *token_type == TokenType::NEWLINE {
            newlines_found += 1;
        };
    }
    //assert only one NEWLINE
    assert_eq!(
        newlines_found, 1,
        "Multiple NEWLINEs found. Was expecting all other newline characters to be NL"
    );
}
