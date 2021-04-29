#[macro_export]
macro_rules! log {
    ($message: tt) => {
        println!(
            "{reset}{msg}",
            msg = $message,
            reset = termion::color::Fg(termion::color::Reset)
        );
    };

    ($message: tt, $color: ident) => {
        print!(
            "{clr}{msg}",
            msg = $message,
            clr = termion::color::Fg(termion::color::$color),
        )
    };
}

#[macro_export]
macro_rules! log_debug {
    ($message: tt) => {
        println!(
            "{}{:?}",
            termion::color::Fg(termion::color::Reset),
            $message,
        );
    };

    ($message: tt, $color: ident) => {
        print!(
            "{}{:?}",
            termion::color::Fg(termion::color::$color),
            $message,
        )
    };
}

#[macro_export]
macro_rules! error {
    ($message: tt) => {
        println!(
            "{clrbg}{clrfg} Error {msgbg}{resetfg} {msg} {resetbg}",
            clrbg = termion::color::Bg(termion::color::LightRed),
            resetbg = termion::color::Bg(termion::color::Reset),
            clrfg = termion::color::Fg(termion::color::Black),
            resetfg = termion::color::Fg(termion::color::Reset),
            msgbg = termion::color::Bg(termion::color::Black),
            msg = $message
        )
    };
}

#[macro_export]
macro_rules! exit {
    ($code: literal) => {{
        println!("");
        std::process::exit($code)
    }};
}

#[macro_export]
macro_rules! pop {
    ($from: ident) => {{
        $from
            .pop()
            .expect_fancy("Tried to pop from an empty stack.")
    }
    .clone()};
}

#[macro_export]
macro_rules! pop_num {
    ($from: ident) => {{
        match pop!($from) {
            crate::parser::Token::NUMBER(num) => Some(num.clone()),
            _ => None,
        }
        .unwrap()
    }};
}
