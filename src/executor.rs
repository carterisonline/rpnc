use crate::{parser::Token, pop, pop_num, util::FancyThrow};

pub trait Executable<T> {
    fn execute(&self, stack: &mut T);
    fn exec_token(&self, step: usize, token: &Token, stack: &mut T) -> bool;
}

impl Executable<Vec<Token>> for Vec<Token> {
    fn execute(&self, stack: &mut Vec<Token>) {
        for (step, token) in self.iter().enumerate() {
            if !Self::exec_token(self, step, token, stack) {
                break;
            }
        }
    }

    fn exec_token(&self, step: usize, token: &Token, stack: &mut Vec<Token>) -> bool {
        let mut rt = true;
        match token {
            Token::DEGREES => {
                unimplemented!();
            }
            Token::NUMBER(num) => stack.push(Token::NUMBER(*num)),
            Token::MULTIPLE(times) => {
                for _ in 1..*times {
                    let target_token = &(self.clone()[step - 1]);
                    self.exec_token(step - 1, target_token, stack);
                }
            }
            Token::NOT => {
                let i1 = pop_num!(stack);
                stack.push(Token::NUMBER(!(i1.floor() as i64) as f64))
            }
            Token::CLEAR => *stack = Vec::new(),
            Token::POP => {
                stack.pop();
            }
            s => {
                let i1 = pop_num!(stack);
                let i2 = pop_num!(stack);
                match s {
                    Token::ADD => {
                        stack.push(Token::NUMBER(i1 + i2));
                    }
                    Token::SUBTRACT => {
                        stack.push(Token::NUMBER(i2 - i1));
                    }
                    Token::MULTIPLY => {
                        stack.push(Token::NUMBER(i1 * i2));
                    }
                    Token::DIVIDE => {
                        stack.push(Token::NUMBER(i2 / i1));
                    }
                    Token::POWER => stack.push(Token::NUMBER(i2.powf(i1))),
                    Token::AND => {
                        stack.push(Token::NUMBER(
                            (i2.floor() as i64 & i1.floor() as i64) as f64,
                        ));
                    }
                    Token::OR => stack.push(Token::NUMBER(
                        (i2.floor() as i64 | i1.floor() as i64) as f64,
                    )),
                    Token::XOR => stack.push(Token::NUMBER(
                        (i2.floor() as i64 ^ i1.floor() as i64) as f64,
                    )),
                    Token::SHIFTLEFT => stack.push(Token::NUMBER(
                        ((i2.floor() as i64) << (i1.floor() as i64)) as f64,
                    )),
                    Token::SHIFTRIGHT => stack.push(Token::NUMBER(
                        ((i2.floor() as i64) >> (i1.floor() as i64)) as f64,
                    )),
                    _ => rt = false,
                }
            }
        }
        rt
    }
}
