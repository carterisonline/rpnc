use std::io::stdin;

use rpnc::log;
use rpnc::parser::Tokenizer;
use rpnc::util::FancyThrow;

fn main() {
    let stdin = stdin();
    loop {
        let mut x = String::new();
        stdin
            .read_line(&mut x)
            .expect_fancy("Error reading new line from stdin");
        println!("{:?}", x.tokenize());
    }
}
