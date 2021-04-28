#[macro_export]
macro_rules! log {
    ($message: tt) => {
        println!(
            "{reset}{msg}\n",
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
macro_rules! error {
    ($message: tt) => {
        println!(
            "{clrbg}{clrfg} Error {msgbg}{resetfg} {msg} {resetbg}\n",
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
