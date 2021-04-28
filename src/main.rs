use std::io::stdin;

use rpnc::log;
use rpnc::util::FancyThrow;

fn main() {
    let stdin = stdin();
    loop {
        let mut x = String::new();
        stdin
            .read_line(&mut x)
            .expect_fancy("Error reading new line from stdin");

        log!("Command: ", Red);
        log!(x);
    }
}
