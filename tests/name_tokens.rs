use rstest::*;

use rye::tokens::{Token, TokenType};

pub mod common;
use common::check_single_token_statement;

#[rstest]
#[case("spam")]
#[case("_spam")]
#[case("spam_")]
#[case("__spam__")]
#[case("i32")]
#[case("i_32")]
#[case("_32")]
#[case("s_p__a_m")]
#[case("Spam_Eggs")]
#[case("S")]
#[case("_")]
#[case("__")]
fn single_name_token(#[case] source: &str) {
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(source);
    assert_eq!(
        token_type,
        TokenType::NAME,
        "Symbol Token not of type NAME, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::NAME,
        "Symbol Token not of exact type NAME, got type {}",
        format!("{:?}", exact_token_type)
    );
}
