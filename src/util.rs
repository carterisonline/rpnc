use crate::error;

fn expect_failed(msg: &str) -> ! {
    error!(msg);
    panic!("{}", msg)
}

pub trait FancyThrow<T> {
    fn expect_fancy(&self, message: &str) -> &T;
}

impl<T, E> FancyThrow<T> for Result<T, E> {
    fn expect_fancy(&self, message: &str) -> &T {
        match self {
            Ok(f) => f,
            Err(_) => expect_failed(&message),
        }
    }
}

impl<T> FancyThrow<T> for Option<T> {
    fn expect_fancy(&self, message: &str) -> &T {
        match self {
            Some(f) => f,
            None => expect_failed(&message),
        }
    }
}
