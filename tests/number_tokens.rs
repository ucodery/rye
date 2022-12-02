use rstest::*;

use rye::tokens::{Token, TokenType};

pub mod common;
use common::{check_single_token_statement, source_to_tokens};

#[rstest]
#[case("1")]
#[case("0")]
#[case("9")]
#[case("0000")]
#[case("1234")]
#[case("1_2_3")]
#[case("0_00_0")]
#[case("1_000")]
fn single_intiger_token(#[case] source: &str) {
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(source);
    assert_eq!(
        token_type,
        TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::INTEGER,
        "Symbol Token not of exact type INTEGER, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("0b0")]
#[case("0b000")]
#[case("0b1")]
#[case("0B111")]
#[case("0B0101")]
#[case("0B101")]
#[case("0b00_11_0")]
fn single_binint_token(#[case] source: &str) {
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(source);
    assert_eq!(
        token_type,
        TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::BININT,
        "Symbol Token not of exact type BININT, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("0o0")]
#[case("0o000")]
#[case("0O1")]
#[case("0O720")]
#[case("0O0_020_0")]
#[case("0o777")]
#[case("0o04_50_2")]
fn single_octint_token(#[case] source: &str) {
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(source);
    assert_eq!(
        token_type,
        TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::OCTINT,
        "Symbol Token not of exact type OCTINT, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("0x0")]
#[case("0x000")]
#[case("0x1")]
#[case("0xABC")]
#[case("0xfFfF")]
#[case("0X0_18D_0f")]
#[case("0X100")]
#[case("0x0b1_050_e3")]
#[case("0xb101")]
fn single_hexint_token(#[case] source: &str) {
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(source);
    assert_eq!(
        token_type,
        TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::HEXINT,
        "Symbol Token not of exact type HEXINT, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("0.")]
#[case("00.")]
#[case("0.0")]
#[case(".0")]
#[case(".00")]
#[case("000.000")]
#[case("1.")]
#[case("01.")]
#[case("1.2")]
#[case(".2")]
#[case(".10")]
#[case("00_2.34_0")]
#[case("1_234.")]
#[case(".1_2_3")]
#[case("0123.456")]
#[case("0.e1")]
#[case("00.e1")]
#[case("0.0e1")]
#[case("0e0")]
#[case("010.23")]
#[case("0000.0000e0000")]
#[case("0123e456")]
#[case("1.2_34e5_6_78")]
#[case("09e050")]
#[case("098.765e43")]
#[case("1.e+234")]
#[case(".1e-234")]
#[case("1e+23_4")]
#[case("1e-0_2")]
fn single_float_token(#[case] source: &str) {
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(source);
    assert_eq!(
        token_type,
        TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::FLOAT,
        "Symbol Token not of exact type FLOAT, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("0j")]
#[case("00J")]
#[case("1j")]
#[case("001J")]
#[case("000_123_4j")]
#[case("123.45e+6j")]
#[case("1.j")]
#[case(".01j")]
fn single_imaginary_token(#[case] source: &str) {
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(source);
    assert_eq!(
        token_type,
        TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        TokenType::IMAGINARY,
        "Symbol Token not of exact type IMAGINARY, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("123_4_", 5, TokenType::INTEGER, TokenType::NAME, TokenType::NAME)]
#[case("0_", 1, TokenType::INTEGER, TokenType::NAME, TokenType::NAME)]
#[case("00_", 2, TokenType::INTEGER, TokenType::NAME, TokenType::NAME)]
#[case("0e", 1, TokenType::INTEGER, TokenType::NAME, TokenType::NAME)]
#[case("00e", 2, TokenType::INTEGER, TokenType::NAME, TokenType::NAME)]
#[case("000e", 3, TokenType::INTEGER, TokenType::NAME, TokenType::NAME)]
#[case("123__4", 3, TokenType::INTEGER, TokenType::NAME, TokenType::NAME)]
#[case("123eyr", 3, TokenType::INTEGER, TokenType::NAME, TokenType::NAME)]
#[case("000123", 3, TokenType::INTEGER, TokenType::NUMBER, TokenType::INTEGER)]
#[case("010234", 1, TokenType::INTEGER, TokenType::NUMBER, TokenType::INTEGER)]
#[case("0_12_3", 1, TokenType::INTEGER, TokenType::NAME, TokenType::NAME)]
#[case("123.e", 4, TokenType::FLOAT, TokenType::NAME, TokenType::NAME)]
#[case("12jeep", 3, TokenType::IMAGINARY, TokenType::NAME, TokenType::NAME)]
fn runon_number_tokens(
    #[case] source: &str,
    #[case] split: usize,
    #[case] exact_number: TokenType,
    #[case] runon_type: TokenType,
    #[case] runon_exact: TokenType,
) {
    let size = source.chars().count();
    let tokens = source_to_tokens(source);
    assert_eq!(
        &tokens[..],
        &[
            Token {
                token_type: TokenType::NUMBER,
                exact_token_type: exact_number,
                token_contents: source[..split].to_string(),
                col_start: 0,
                col_end: split,
            },
            Token {
                token_type: runon_type,
                exact_token_type: runon_exact,
                token_contents: source[split..].to_string(),
                col_start: split,
                col_end: size,
            },
            Token {
                token_type: TokenType::NEWLINE,
                exact_token_type: TokenType::NEWLINE,
                token_contents: String::from(""),
                col_start: size,
                col_end: (size + 1),
            },
        ]
    );
}

#[test]
fn multiple_runon_number_tokens() {
    let source = "0012eyr";
    let tokens = source_to_tokens(source);
    assert_eq!(
        &tokens[..],
        &[
            Token {
                token_type: TokenType::NUMBER,
                exact_token_type: TokenType::INTEGER,
                token_contents: String::from("00"),
                col_start: 0,
                col_end: 2,
            },
            Token {
                token_type: TokenType::NUMBER,
                exact_token_type: TokenType::INTEGER,
                token_contents: String::from("12"),
                col_start: 2,
                col_end: 4,
            },
            Token {
                token_type: TokenType::NAME,
                exact_token_type: TokenType::NAME,
                token_contents: String::from("eyr"),
                col_start: 4,
                col_end: 7,
            },
            Token {
                token_type: TokenType::NEWLINE,
                exact_token_type: TokenType::NEWLINE,
                token_contents: String::from(""),
                col_start: 7,
                col_end: 8,
            },
        ]
    );
}
