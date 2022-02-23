#[cfg(test)]

use rye::tokenize::TokenStream;
use rye::tokens::{Token, TokenType};

/// turn `source` into a stream of tokens
/// perform minimum checks on the stream
/// return the tokens, minus the mandatory ENDMARKER
pub fn source_to_tokens(source: &str) -> Vec<Token> {
    let source_len = source.chars().count();

    let maybe_tokens: Result<Vec<Token>, String> =
        TokenStream::new(source).into_iter().collect();
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

    let Token {
        token_type,
        exact_token_type,
        token_contents,
        col_start,
        col_end,
    } = tokens.pop().unwrap();
    assert_eq!(
        token_type,
        TokenType::ENDMARKER,
        "Token Stream did not end in expected ENDMARKER"
    );
    assert_eq!(
        exact_token_type,
        TokenType::ENDMARKER,
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

/// turn `source` into a stream of tokens
/// assert that the tokens contain a single statement of a single token
/// return the one token found for any further checks
pub fn check_single_token_statement(source: &str) -> Token {
    let source_len = source.chars().count();
    let mut tokens = source_to_tokens(source);

    assert!(
        tokens.len() < 3,
        "Too many tokens found: {}",
        format!("{:?}", tokens)
    );
    assert!(
        tokens.len() == 2,
        "Too few tokens found: {}",
        format!("{:?}", tokens)
    );

    let Token {
        token_type,
        exact_token_type,
        token_contents,
        col_start,
        col_end,
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
        token_contents, "\n",
        "NEWLINE does not have expected contents"
    );
    assert_eq!(col_start, source_len, "NEWLINE did not start after TOKEN");
    assert_eq!(col_end, source_len + 1, "NEWLINE is not of expected size");

    let tok = tokens.pop().unwrap();
    let Token {
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

///turn `source` into a stream of tokens
/// assert that the tokens contain a single token
/// return the one token found for any further checks
pub fn check_single_token(source: &str) -> Token {
    let source_len = source.chars().count();
    let mut tokens = source_to_tokens(source);

    assert!(
        tokens.len() == 1,
        "Too many tokens found: {}",
        format!("{:?}", tokens)
    );

    let tok = tokens.pop().unwrap();
    let Token {
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
