extern crate pest;
#[macro_use]
extern crate pest_derive;

mod interpreter;
mod parser;

mod cli {

    use std::env;

    pub fn collect_args() -> String {
        let mut all_args: Vec<String> = env::args().collect();

        let filename = all_args.remove(1);

        filename
    }
}

fn main() {
    let filename = cli::collect_args();
    interpreter::load::load_from_file(filename)
}
