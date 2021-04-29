use crate::{parser::Token, pop, pop_num, util::FancyThrow};

pub trait Executable<T> {
    fn execute(&self, destination: &mut T);
}

impl Executable<Vec<Token>> for Vec<Token> {
    fn execute(&self, destination: &mut Vec<Token>) {
        for token in self {
            match token {
                Token::INNER(_) => {
                    unimplemented!();
                }
                Token::DEGREES => {
                    unimplemented!();
                }
                Token::NUMBER(num) => destination.push(Token::NUMBER(*num)),
                Token::NOT => {
                    let i1 = pop_num!(destination);
                    destination.push(Token::NUMBER(!(i1.floor() as i64) as f64))
                }
                s => {
                    let i1 = pop_num!(destination);
                    let i2 = pop_num!(destination);
                    match s {
                        Token::ADD => {
                            destination.push(Token::NUMBER(i1 + i2));
                        }
                        Token::SUBTRACT => {
                            destination.push(Token::NUMBER(i2 - i1));
                        }
                        Token::MULTIPLY => {
                            destination.push(Token::NUMBER(i1 * i2));
                        }
                        Token::DIVIDE => {
                            destination.push(Token::NUMBER(i2 / i1));
                        }
                        Token::POWER => destination.push(Token::NUMBER(i2.powf(i1))),
                        Token::AND => {
                            destination.push(Token::NUMBER(
                                (i2.floor() as i64 & i1.floor() as i64) as f64,
                            ));
                        }
                        Token::OR => destination.push(Token::NUMBER(
                            (i2.floor() as i64 | i1.floor() as i64) as f64,
                        )),
                        Token::XOR => destination.push(Token::NUMBER(
                            (i2.floor() as i64 ^ i1.floor() as i64) as f64,
                        )),
                        Token::SHIFTLEFT => destination.push(Token::NUMBER(
                            ((i2.floor() as i64) << (i1.floor() as i64)) as f64,
                        )),
                        Token::SHIFTRIGHT => destination.push(Token::NUMBER(
                            ((i2.floor() as i64) >> (i1.floor() as i64)) as f64,
                        )),
                        _ => break,
                    }
                }
            }
        }
    }
}
