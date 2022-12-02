use rstest::*;

use rye::tokens::{Token, TokenType};

pub mod common;
use common::{check_single_token_statement, source_to_tokens};

#[rstest]
#[case("~", TokenType::TILDE)]
#[case("}", TokenType::RBRACE)]
#[case("|", TokenType::VBAR)]
#[case("^", TokenType::CIRCUMFLEX)]
#[case("]", TokenType::RSQB)]
#[case(")", TokenType::RPAR)]
#[case("@", TokenType::AT)]
#[case("=", TokenType::EQUAL)]
#[case("<", TokenType::LESS)]
#[case(">", TokenType::GREATER)]
#[case(";", TokenType::SEMI)]
#[case(":", TokenType::COLON)]
#[case("/", TokenType::SLASH)]
#[case(".", TokenType::DOT)]
#[case("-", TokenType::MINUS)]
#[case(",", TokenType::COMMA)]
#[case("+", TokenType::PLUS)]
#[case("*", TokenType::STAR)]
#[case("&", TokenType::AMPER)]
#[case("%", TokenType::PERCENT)]
#[case("|=", TokenType::VBAREQUAL)]
#[case("^=", TokenType::CIRCUMFLEXEQUAL)]
#[case("@=", TokenType::ATEQUAL)]
#[case(">>", TokenType::RIGHTSHIFT)]
#[case(">=", TokenType::GREATEREQUAL)]
#[case("==", TokenType::EQEQUAL)]
#[case("<>", TokenType::NOTEQUAL)]
#[case("<=", TokenType::LESSEQUAL)]
#[case("<<", TokenType::LEFTSHIFT)]
#[case(":=", TokenType::COLONEQUAL)]
#[case("/=", TokenType::SLASHEQUAL)]
#[case("//", TokenType::DOUBLESLASH)]
#[case("->", TokenType::RARROW)]
#[case("-=", TokenType::MINEQUAL)]
#[case("+=", TokenType::PLUSEQUAL)]
#[case("*=", TokenType::STAREQUAL)]
#[case("**", TokenType::DOUBLESTAR)]
#[case("&=", TokenType::AMPEREQUAL)]
#[case("%=", TokenType::PERCENTEQUAL)]
#[case("!=", TokenType::NOTEQUAL)]
#[case(">>=", TokenType::RIGHTSHIFTEQUAL)]
#[case("<<=", TokenType::LEFTSHIFTEQUAL)]
#[case("//=", TokenType::DOUBLESLASHEQUAL)]
#[case("...", TokenType::ELLIPSIS)]
#[case("**=", TokenType::DOUBLESTAREQUAL)]
fn single_symbol_token(#[case] source: &str, #[case] exact: TokenType) {
    let Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = check_single_token_statement(source);
    assert_eq!(
        token_type,
        TokenType::OP,
        "Symbol Token not of type OP, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        exact,
        "Symbol Token not of exact type {}, got type {}",
        format!("{:?}", exact),
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("()", TokenType::LPAR, "(", TokenType::RPAR, ")")]
#[case("[]", TokenType::LSQB, "[", TokenType::RSQB, "]")]
#[case("{}", TokenType::LBRACE, "{", TokenType::RBRACE, "}")]
fn symbol_pair_tokens(
    #[case] source: &str,
    #[case] first_type: TokenType,
    #[case] first_str: &str,
    #[case] second_type: TokenType,
    #[case] second_str: &str,
) {
    let mut tokens = source_to_tokens(source);

    assert!(
        tokens.len() == 3,
        "Not enough tokens found: {}",
        format!("{:?}", tokens)
    );

    let Token {
        token_type,
        exact_token_type,
        token_contents,
        col_start: _,
        col_end: _,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        TokenType::NEWLINE,
        "Token Stream did not end in expected NEWLINE"
    );
    assert_eq!(
        exact_token_type,
        TokenType::NEWLINE,
        "Token Stream did not end in expected NEWLINE"
    );
    assert_eq!(
        token_contents, "",
        "NEWLINE does not have expected contents"
    );

    let Token {
        token_type,
        exact_token_type,
        token_contents,
        col_start: _,
        col_end: _,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        TokenType::OP,
        "Token Stream did not end in expected TOKEN"
    );
    assert_eq!(
        exact_token_type, second_type,
        "Token Stream did not end in expected TOKEN"
    );
    assert_eq!(
        token_contents, second_str,
        "TOKEN does not have expected contents"
    );

    let Token {
        token_type,
        exact_token_type,
        token_contents,
        col_start: _,
        col_end: _,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        TokenType::OP,
        "Token Stream did not end in expected TOKEN"
    );
    assert_eq!(
        exact_token_type, first_type,
        "Token Stream did not end in expected TOKEN"
    );
    assert_eq!(
        token_contents, first_str,
        "TOKEN does not have expected contents"
    );
}
