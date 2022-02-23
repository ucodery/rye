pub mod tokenize;
pub mod tokens;

fn main() {
    let ts = tokenize::TokenStream::new(
        r#"
if not "":
    def foo(a, b=12.0):
        c = a + b

        if (\
            c > 42
        ):\
            return False

        return True

        
"#,
    );
    for found in ts {
        let tokens::Token {
            token_type,
            exact_token_type,
            token_contents,
            col_start,
            col_end,
        } = found.unwrap();
        println!(
            "{}-{}:\t\t{:?}:{:?}\t\t{:?}",
            col_start,
            (col_end - col_start),
            token_type,
            exact_token_type,
            token_contents
        )
    }
}
