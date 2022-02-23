use rstest::*;

use rye::tokens::{Token, TokenType};

mod common;
use common::check_single_token_statement;

#[rstest]
#[case("~", TokenType::TILDE)]
#[case("}", TokenType::RBRACE)]
#[case("|", TokenType::VBAR)]
#[case("{", TokenType::LBRACE)]
#[case("^", TokenType::CIRCUMFLEX)]
#[case("]", TokenType::RSQB)]
#[case("[", TokenType::LSQB)]
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
#[case(")", TokenType::RPAR)]
#[case("(", TokenType::LPAR)]
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
