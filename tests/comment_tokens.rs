use rstest::*;

use rye::tokens::{Token, TokenType};

mod common;
use common::check_single_token;

#[rstest]
#[case("#")]
#[case("##")]
#[case("#r")]
#[case("# ")]
#[case("# rye")]
#[case("# rye # eyr")]
#[case("#\"rye\"")]
#[case("##")]
fn single_comment_token(#[case] source: &str) {
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token(source);
    assert_eq!(
        token_type,
        TokenType::COMMENT,
        "Symbol Token not of type COMMENT, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::COMMENT,
        "Symbol Token not of exact type COMMENT, got type {}",
        format!("{:?}", exact_token_type)
    );
}
