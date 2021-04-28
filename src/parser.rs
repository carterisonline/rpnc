use once_cell::sync::Lazy;
use regex::Regex;

use crate::{error, util::FancyThrow};

#[derive(Debug)]
pub enum Token {
    INNER(Box<Token>),
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    POWER,
    DEGREES,
    AND,
    OR,
    XOR,
    NOT,
    SHIFTLEFT,
    SHIFTRIGHT,
    SHIFTRIGHTZERO,
    NUMBER(f64),
}

static WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
static NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"-?\d+\.?(\d+)?").unwrap());

pub trait Tokenizer {
    fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizer for String {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let s = self.trim();
        for s in (*WHITESPACE).split(&s) {
            match s {
                "+" => tokens.push(Token::ADD),
                "-" => tokens.push(Token::SUBTRACT),
                "*" | "·" | "×" => tokens.push(Token::MULTIPLY),
                "/" | "÷" => tokens.push(Token::DIVIDE),
                "^" => tokens.push(Token::POWER),
                "°" => tokens.push(Token::DEGREES),
                "b&" => tokens.push(Token::AND),
                "b|" => tokens.push(Token::OR),
                "b^" => tokens.push(Token::XOR),
                "b!" => tokens.push(Token::NOT),
                "b<<" => tokens.push(Token::SHIFTLEFT),
                "b>>" => tokens.push(Token::SHIFTRIGHT),
                "b>>>" => tokens.push(Token::SHIFTRIGHTZERO),
                "¹" => {
                    tokens.push(Token::NUMBER(1.0));
                    tokens.push(Token::POWER);
                }
                "²" => {
                    tokens.push(Token::NUMBER(2.0));
                    tokens.push(Token::POWER);
                }
                "³" => {
                    tokens.push(Token::NUMBER(3.0));
                    tokens.push(Token::POWER);
                }
                n if NUMBER.is_match(s) => tokens.push(Token::NUMBER(
                    *n.parse::<f64>().expect_fancy("Failed to parse number"),
                )),
                other => error!((format!("Unknown token `{}`", other))),
            }
        }

        return tokens;
    }
}
