pub mod executor;
mod macros;
pub mod parser;
pub mod util;

mod tests {
    #[allow(unused_imports)] // Fix incorrect issue with warning about unused imports
    use crate::util::FancyThrow;
    #[warn(unused_imports)]
    #[test]
    #[should_panic]
    fn expect_fancy() {
        let mut v: Vec<u8> = Vec::new();
        v.pop().expect_fancy("Can't pop item out of an empty Vec!");
    }
}
