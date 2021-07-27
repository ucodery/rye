pub mod tokenize;
pub mod tokens;

fn main() {
    let ts = tokenize::TokenStream::new("foo = (bar==4*3//21+7)");
    for found in ts {
        match found {
            tokens::Token {
                token_type: t,
                size: Some(s),
            } => {
                println!("Found {:?} starting at {} and of size {}", t, "?", s)
            }
            tokens::Token {
                token_type: t,
                size: None,
            } => {
                println!("Found {:?} starting at {} and of size {}", t, "?", "?")
            }
        }
    }
}
