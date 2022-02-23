use rstest::*;

use rye::tokens::{Token, TokenType};

mod common;
use common::source_to_tokens;

#[rstest]
#[case("")]
#[case(" ")]
#[case("\t")]
#[case("\u{000C}")] // form feed
#[case("\\\n")]
#[case(" \t \t")]
fn insignificant_whitespace(#[case] source: &str) {
    let tokens = source_to_tokens(source);
    assert!(
        tokens.len() == 0,
        "Too many tokens found: {}",
        format!("{:?},", tokens)
    );
}

#[rstest]
#[case(
    r"\
\

", false
)]
#[case(
    r"(\
\
)", true
)]
#[case(
    r"    \
\
\
  \
      \

",
    false
)]
#[case(
    r"name\
+\
_name",
    true
)]
#[case(
    r"'one string'\
'two string'\
'last string'",
    true
)]
#[case(
    r"\
\
# closing comment
",
    false
)]
fn explicit_line_join(#[case] source: &str, #[case] is_statement: bool) {
    let final_token = match is_statement {
        true => TokenType::NEWLINE,
        false => TokenType::NL,
    };
    let mut tokens = source_to_tokens(source);
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
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        final_token,
        "Symbol Token not of type {}, got type {}",
        format!("{:?}", final_token),
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        final_token,
        "Symbol Token not of exact type {}, got type {}",
        format!("{:?}", final_token),
        format!("{:?}", exact_token_type)
    );

    for token in tokens {
        let Token {
            token_type,
            exact_token_type: _,
            token_contents: _,
            col_start: _,
            col_end: _,
        } = token;
        assert_ne!(token_type, TokenType::NL, "Got extra NL Token",);
        assert_ne!(
            token_type,
            TokenType::NEWLINE,
            "Got extra NEWLINE Token",
        );
    }
}
