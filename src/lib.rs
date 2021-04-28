mod macros;
pub mod util;

mod tests {
    use crate::util::FancyThrow;
    #[test]
    #[should_panic]
    fn expect_fancy() {
        let mut v: Vec<u8> = Vec::new();
        v.pop()
            .expect_fancy("Failed to do a certain important thing");
    }
}
