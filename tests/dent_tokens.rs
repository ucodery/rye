#![feature(assert_matches)]
use std::assert_matches::assert_matches;

use rstest::*;

use rye::tokenize::TokenStream;
use rye::tokens::{Token, TokenType};

mod common;
use common::source_to_tokens;

#[rstest]
#[case("    rye", 1)]
#[case("\trye", 1)]
#[case("  rye", 1)]
#[case("  \t    \trye", 1)]
#[case(
    "
    rye
    cheese
", 1
)]
#[case(
    "
    rye

", 1
)]
#[case(
    "
rye
    cheese
        bread
done
", 2
)]
#[case(
    "
  rye
    cheese
", 2
)]
#[case(
    "
  rye
      cheese
  bread
", 2
)]
#[case(
    "
    rye
        cheese
            bread
", 3
)]
#[case(
    "
\u{000C}    rye
    \u{000C}cheese
    bread
", 1
)]
#[case(
    "
        rye
\tcheese
", 1
)]
fn dent_tokens(#[case] source: &str, #[case] total_indents: usize) {
    let mut indents_found = 0;
    let mut dedents_found = 0;
    let tokens = source_to_tokens(source);

    for Token{ token_type, exact_token_type, token_contents: _, col_start: _, col_end: _, } in tokens.iter() {
        if *token_type == TokenType::INDENT {
            indents_found = indents_found + 1;
            assert_eq!(*exact_token_type, TokenType::INDENT);
            assert!(indents_found <= total_indents, "{} INDENTs were found when only {} were expected", indents_found, total_indents);
        };
        if *token_type == TokenType::DEDENT {
            dedents_found = dedents_found + 1;
            assert_eq!(*exact_token_type, TokenType::DEDENT);
            assert!(dedents_found <= indents_found, "{} DEDENTs were found when only {} INDENTs had previously been found", dedents_found, indents_found);
        };
    };
    assert_eq!(indents_found, total_indents, "Not enough INDENTs were found. Was expecting {} got {}", total_indents, indents_found);
    assert_eq!(indents_found, dedents_found, "Not every INDENT had a DEDENT. Found {} more INDENTs", (indents_found - dedents_found));
}

#[test]
fn unmatched_dent() {
    let maybe_tokens = TokenStream::new(
"
    rye
        cheese
          bread
  unmatched
"
);
    assert_matches!(maybe_tokens.into_iter().collect::<Result<Vec<Token>, String>>(), Err(_));
}
