pub mod executor;
mod macros;
pub mod parser;
pub mod util;

mod tests {
    #![allow(unused_imports)]
    #![allow(unused_parens)]
    use crate::executor::Executable;
    use crate::parser::Token;
    use crate::parser::Tokenizer;
    use crate::util::FancyThrow;

    macro_rules! unittest {
        ($name: ident, $input: literal, [$($expected: tt),*]) => {
            #[test]
            fn $name() {
                let mut expected = Vec::new();
                $(expected.push(Token::NUMBER($expected));)*
                let mut teststack: Vec<Token> = Vec::new();
                let input = String::from($input);
                let output = input.tokenize();
                output.execute(&mut teststack);
                assert_eq!(teststack, expected);
            }
        };
    }

    #[test]
    #[should_panic]
    fn expect_fancy() {
        let mut v: Vec<u8> = Vec::new();
        v.pop().expect_fancy("Can't pop item out of an empty Vec!");
    }
    unittest!(basic_addition, "1 1 +", [2.0]);
    unittest!(basic_subtraction, "1 2 -", [(-1.0)]);
    unittest!(basic_multiplication, "10 9 *", [90.0]);
    unittest!(basic_division, "12 3 /", [4.0]);
    unittest!(basic_power, "12 2 ^", [144.0]);
    unittest!(symbolic_mutliplication_1, "7 8 ×", [56.0]);
    unittest!(symbolic_mutliplication_2, "9 10 ·", [90.0]);
    unittest!(symbolic_division, "3105 5 ÷", [621.0]);
    unittest!(symbolic_power_1, "12 ¹", [12.0]);
    unittest!(symbolic_power_2, "12 ²", [144.0]);
    unittest!(symbolic_power_3, "12 ³", [1728.0]);
    unittest!(
        basic_multiplicity,
        "-1.2 5x",
        [(-1.2), (-1.2), (-1.2), (-1.2), (-1.2)]
    );
    unittest!(advanced_multiplicity_1, "1 1000x + 999x", [1000.0]);
    unittest!(advanced_multiplicity_2, "2 10x * 9x", [1024.0]);
    unittest!(bitwise_and, "100 68 i&", [68.0]);
    unittest!(bitwise_or, "56 53 i|", [61.0]);
    unittest!(bitwise_not, "12 i!", [(-13.0)]);
    unittest!(bitwise_xor, "12 9 i^", [5.0]);
    unittest!(bitwise_shiftleft, "43 5 i<<", [1376.0]);
    unittest!(bitwise_shiftright, "32 2 i>>", [8.0]);
}
