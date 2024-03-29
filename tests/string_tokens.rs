use rstest::*;

use rye::tokens::{Token, TokenType};

pub mod common;
use common::check_single_token_statement;

#[rstest]
#[case("")]
#[case("\0")]
#[case(" ")]
#[case("rye")]
#[case("#not a comment")]
#[case(r"\\")]
#[case(r"\\\\")]
#[case(r"\'")]
#[case(r"\\\'")]
#[case("\\\"")]
#[case("\\\\\\\"")]
#[case(r"\a")]
#[case(r"\b")]
#[case(r"\f")]
#[case(r"\t")]
#[case(r"\r")]
#[case(r"\n")]
#[case(r"\v")]
#[case(r"\0")]
#[case(r"\x15")]
#[case("}{")]
#[case("!@$%^&*()-_=+[]|;:<>?,./`~")]
#[case("\t")]
#[case("\u{1F980}")]
fn single_always_valid_string(
    #[case] source: &str,
    #[values(
        "", "f", "F", "r", "R", "u", "U", "b", "B", "rf", "rF", "Rf", "RF", "fr", "Fr", "fR", "FR",
        "br", "bR", "Br", "BR", "rb", "rB", "Rb", "RB"
    )]
    prefix: &str,
    #[values("\"", "'", "\"\"\"", "'''")] quotes: &str,
) {
    let py_string = format!(r#"{}{}{}{}"#, prefix, quotes, source, quotes);
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(&py_string);
    assert_eq!(
        token_type,
        TokenType::STRING,
        "Symbol Token not of type STRING, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::STRING,
        "Symbol Token not of exact type STRING, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("\" ")]
#[case("\"\" ")]
#[case("' ")]
#[case("'' ")]
#[case("\n")]
#[case("\r")]
#[case("\n\r")]
fn single_triple_string(
    #[case] source: &str,
    #[values(
        "", "f", "F", "r", "R", "u", "U", "b", "B", "rf", "rF", "Rf", "RF", "fr", "Fr", "fR", "FR",
        "br", "bR", "Br", "BR", "rb", "rB", "Rb", "RB"
    )]
    prefix: &str,
    #[values("\"\"\"", "'''")] quotes: &str,
) {
    let py_string = format!(r#"{}{}{}{}"#, prefix, quotes, source, quotes);
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(&py_string);
    assert_eq!(
        token_type,
        TokenType::STRING,
        "Symbol Token not of type STRING, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::STRING,
        "Symbol Token not of exact type STRING, got type {}",
        format!("{:?}", exact_token_type)
    );
}
