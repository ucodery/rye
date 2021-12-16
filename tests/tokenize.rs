use rstest::*;

use rye::{tokenize, tokens};

/// turn `source` into a stream of tokens
/// perform checks on the stream and return the first token for any further checks
fn single_symbol_checks(source: &str) -> tokens::Token {
    let source_len = source.chars().count();

    let maybe_tokens: Result<Vec<tokens::Token>, String> =
        tokenize::TokenStream::new(source).into_iter().collect();
    assert!(
        maybe_tokens.is_ok(),
        "{} while tokenizing source {}",
        format!("{:?}", maybe_tokens),
        source
    );

    let mut tokens = maybe_tokens.unwrap();
    assert!(
        tokens.len() <= 3,
        "Too many tokens found: {}",
        format!("{:?}", tokens)
    );
    assert!(
        tokens.len() == 3,
        "Too few tokens found: {}",
        format!("{:?}", tokens)
    );

    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents,
        col_start,
        col_end,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        tokens::TokenType::ENDMARKER,
        "Token Stream did not end in expected ENDMARKER"
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::ENDMARKER,
        "Token Stream did not end in expected ENDMARKER"
    );
    assert_eq!(
        token_contents, "",
        "ENDMARKER does not have expected contents"
    );
    assert_eq!(
        col_start,
        source_len + 1,
        "ENDMARKER did not start after NEWLINE"
    );
    assert_eq!(col_end, source_len + 1, "ENDMARKER is not of expected size");

    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents,
        col_start,
        col_end,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        tokens::TokenType::NEWLINE,
        "Token Stream did not end in expected NEWLINE"
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::NEWLINE,
        "Token Stream did not end in expected NEWLINE"
    );
    assert_eq!(
        token_contents, "\n",
        "NEWLINE does not have expected contents"
    );
    assert_eq!(col_start, source_len, "NEWLINE did not start after TOKEN");
    assert_eq!(col_end, source_len + 1, "NEWLINE is not of expected size");

    let tok = tokens.pop().unwrap();
    let tokens::Token {
        token_type: _,
        exact_token_type: _,
        token_contents,
        col_start,
        col_end,
    } = tok.clone();
    assert_eq!(
        token_contents,
        source,
        "Token ({}) does not look like source ({})",
        format!("{:?}", token_contents),
        source
    );
    assert_eq!(col_start, 0, "Token did not start at start of source");
    assert_eq!(col_end, source_len, "Token did not end at end of source");
    tok
}

/// turn `source` into a stream of tokens
/// perform minimum checks on the stream
/// return a vec of the tokens, minus the mandatory ENDMARKER
fn source_to_tokens(source: &str) -> Vec<tokens::Token> {
    let source_len = source.chars().count();

    let maybe_tokens: Result<Vec<tokens::Token>, String> =
        tokenize::TokenStream::new(source).into_iter().collect();
    assert!(
        maybe_tokens.is_ok(),
        "{} while tokenizing source {}",
        format!("{:?}", maybe_tokens),
        source
    );

    let mut tokens = maybe_tokens.unwrap();
    assert!(
        tokens.len() > 0,
        "No tokens found: {}",
        format!("{:?}", tokens)
    );

    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents,
        col_start,
        col_end,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        tokens::TokenType::ENDMARKER,
        "Token Stream did not end in expected ENDMARKER"
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::ENDMARKER,
        "Token Stream did not end in expected ENDMARKER"
    );
    assert_eq!(
        token_contents, "",
        "ENDMARKER does not have expected contents"
    );
    assert_eq!(
        col_start,
        source_len + 1,
        "ENDMARKER did not start after source"
    );
    assert_eq!(col_end, source_len + 1, "ENDMARKER is not of expected size");

    tokens
}

#[rstest]
#[case("~", tokens::TokenType::TILDE)]
#[case("}", tokens::TokenType::RBRACE)]
#[case("|", tokens::TokenType::VBAR)]
#[case("{", tokens::TokenType::LBRACE)]
#[case("^", tokens::TokenType::CIRCUMFLEX)]
#[case("]", tokens::TokenType::RSQB)]
#[case("[", tokens::TokenType::LSQB)]
#[case("@", tokens::TokenType::AT)]
#[case("=", tokens::TokenType::EQUAL)]
#[case("<", tokens::TokenType::LESS)]
#[case(";", tokens::TokenType::SEMI)]
#[case(":", tokens::TokenType::COLON)]
#[case("/", tokens::TokenType::SLASH)]
#[case(".", tokens::TokenType::DOT)]
#[case("-", tokens::TokenType::MINUS)]
#[case(",", tokens::TokenType::COMMA)]
#[case("+", tokens::TokenType::PLUS)]
#[case("*", tokens::TokenType::STAR)]
#[case(")", tokens::TokenType::RPAR)]
#[case("(", tokens::TokenType::LPAR)]
#[case("&", tokens::TokenType::AMPER)]
#[case("%", tokens::TokenType::PERCENT)]
#[case("|=", tokens::TokenType::VBAREQUAL)]
#[case("^=", tokens::TokenType::CIRCUMFLEXEQUAL)]
#[case("@=", tokens::TokenType::ATEQUAL)]
#[case(">>", tokens::TokenType::RIGHTSHIFT)]
#[case(">=", tokens::TokenType::GREATEREQUAL)]
#[case("==", tokens::TokenType::EQEQUAL)]
#[case("<>", tokens::TokenType::NOTEQUAL)]
#[case("<=", tokens::TokenType::LESSEQUAL)]
#[case("<<", tokens::TokenType::LEFTSHIFT)]
#[case(":=", tokens::TokenType::COLONEQUAL)]
#[case("/=", tokens::TokenType::SLASHEQUAL)]
#[case("//", tokens::TokenType::DOUBLESLASH)]
#[case("->", tokens::TokenType::RARROW)]
#[case("-=", tokens::TokenType::MINEQUAL)]
#[case("+=", tokens::TokenType::PLUSEQUAL)]
#[case("*=", tokens::TokenType::STAREQUAL)]
#[case("**", tokens::TokenType::DOUBLESTAR)]
#[case("&=", tokens::TokenType::AMPEREQUAL)]
#[case("%=", tokens::TokenType::PERCENTEQUAL)]
#[case("!=", tokens::TokenType::NOTEQUAL)]
#[case(">>=", tokens::TokenType::RIGHTSHIFTEQUAL)]
#[case("<<=", tokens::TokenType::LEFTSHIFTEQUAL)]
#[case("//=", tokens::TokenType::DOUBLESLASHEQUAL)]
#[case("...", tokens::TokenType::ELLIPSIS)]
#[case("**=", tokens::TokenType::DOUBLESTAREQUAL)]
fn single_symbol_token(#[case] source: &str, #[case] exact: tokens::TokenType) {
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(source);
    assert_eq!(
        token_type,
        tokens::TokenType::OP,
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
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(source);
    assert_eq!(
        token_type,
        tokens::TokenType::NAME,
        "Symbol Token not of type NAME, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::NAME,
        "Symbol Token not of exact type NAME, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("1")]
#[case("0")]
#[case("9")]
#[case("0000")]
#[case("1234")]
#[case("1_2_3")]
#[case("1_000")]
fn single_intiger_token(#[case] source: &str) {
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(source);
    assert_eq!(
        token_type,
        tokens::TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::INTEGER,
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
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(source);
    assert_eq!(
        token_type,
        tokens::TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::BININT,
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
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(source);
    assert_eq!(
        token_type,
        tokens::TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::OCTINT,
        "Symbol Token not of exact type OCTINT, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("0x0")]
#[case("0x000")]
#[case("0x1")]
#[case("0xABC")]
#[case("0X0_18A_0f")]
#[case("0X100")]
#[case("0x0b1_050_e3")]
#[case("0xb101")]
fn single_hexint_token(#[case] source: &str) {
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(source);
    assert_eq!(
        token_type,
        tokens::TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::HEXINT,
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
#[case("0000.0000e0000")]
#[case("0123e456")]
#[case("1.2_34e5_6_78")]
#[case("1.e+234")]
#[case(".1e-234")]
#[case("1e+23_4")]
#[case("1e-0_2")]
fn single_float_token(#[case] source: &str) {
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(source);
    assert_eq!(
        token_type,
        tokens::TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::FLOAT,
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
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(source);
    assert_eq!(
        token_type,
        tokens::TokenType::NUMBER,
        "Symbol Token not of type NUMBER, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::IMAGINARY,
        "Symbol Token not of exact type IMAGINARY, got type {}",
        format!("{:?}", exact_token_type)
    );
}

#[rstest]
#[case("#")]
#[case("##")]
#[case("#r")]
#[case("# ")]
#[case("# rye")]
#[case("# rye # eyr")]
#[case("#\"rye\"")]
#[case("##")]
fn single_comment(#[case] source: &str) {
    let mut tokens = source_to_tokens(source);
    assert!(
        tokens.len() == 1,
        "Too many tokens found: {}",
        format!("{:?},", tokens)
    );

    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents,
        col_start,
        col_end,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        tokens::TokenType::COMMENT,
        "Symbol Token not of type COMMENT, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::COMMENT,
        "Symbol Token not of exact type COMMENT, got type {}",
        format!("{:?}", exact_token_type)
    );
    assert_eq!(
        token_contents,
        source,
        "Token ({}) does not look like source ({})",
        format!("{:?}", token_contents),
        source
    );
    assert_eq!(col_start, 0, "Token did not start at start of source");
    assert_eq!(
        col_end,
        source.chars().count(),
        "Token did not end at end of source"
    );
}

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
#[case("\n")]
#[case("    \n")]
#[case("\n\t")]
//#[case("\r")]
//#[case("\r\n")]
fn insignificant_newlines(#[case] source: &str) {
    let mut tokens = source_to_tokens(source);
    assert!(
        tokens.len() == 1,
        "Too many tokens found: {}",
        format!("{:?},", tokens)
    );

    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        tokens::TokenType::NL,
        "Symbol Token not of type NL, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::NL,
        "Symbol Token not of exact type NL, got type {}",
        format!("{:?}", exact_token_type)
    );
}

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
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(&py_string);
    assert_eq!(
        token_type,
        tokens::TokenType::STRING,
        "Symbol Token not of type STRING, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::STRING,
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
    let tokens::Token {
        token_type,
        exact_token_type,
        token_contents: _,
        col_start: _,
        col_end: _,
    } = single_symbol_checks(&py_string);
    assert_eq!(
        token_type,
        tokens::TokenType::STRING,
        "Symbol Token not of type STRING, got type {}",
        format!("{:?}", token_type)
    );
    assert_eq!(
        exact_token_type,
        tokens::TokenType::STRING,
        "Symbol Token not of exact type STRING, got type {}",
        format!("{:?}", exact_token_type)
    );
}
