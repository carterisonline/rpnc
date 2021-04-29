use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rpnc::log;
use rpnc::parser::Token;
use rpnc::parser::Tokenizer;
use rpnc::util::FancyThrow;
use rpnc::{executor::Executable, log_debug};
use std::io::stdin;

static STACK: Lazy<Mutex<Vec<Token>>> = Lazy::new(|| Mutex::new(vec![]));

fn main() {
    let stdin = stdin();
    loop {
        let mut x = String::new();
        stdin
            .read_line(&mut x)
            .expect_fancy("Error reading new line from stdin");
        let tokened = x.tokenize();
        log!("Parsed Input: ", Yellow);
        log_debug!(tokened);
        let mut s = (*STACK).lock();
        tokened.execute(&mut s);
        log!("Resulting Stack: ", Green);
        log_debug!((&s));
    }
}
