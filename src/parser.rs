#![allow(dead_code)]
use std::fmt::Display;

use error::{ExpressionParseError, LexError};
use token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    Op(Box<Operator>),
}

#[derive(Debug, Clone)]
pub struct Operator {
    pub typ: OperatorType,
    pub lhs: Expression,
    pub rhs: Expression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

impl Display for OperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorType::Add => write!(f, "OpAdd"),
            OperatorType::Sub => write!(f, "OpSub"),
            OperatorType::Mul => write!(f, "OpMul"),
            OperatorType::Div => write!(f, "OpDiv"),
            OperatorType::Pow => write!(f, "OpPow"),
            OperatorType::Mod => write!(f, "OpMod"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BindingPower {
    lhs: f32,
    rhs: f32,
}

impl From<(f32, f32)> for BindingPower {
    fn from(value: (f32, f32)) -> Self {
        Self {
            lhs: value.0,
            rhs: value.1,
        }
    }
}

impl BindingPower {
    fn of(op: OperatorType) -> Self {
        match op {
            OperatorType::Add => (1.1, 1.0),
            OperatorType::Sub => (1.1, 1.0),
            OperatorType::Mul => (2.1, 2.0),
            OperatorType::Div => (2.0, 2.1),
            _ => todo!(),
        }
        .into()
    }

    fn left(&self) -> f32 {
        self.lhs
    }

    fn right(&self) -> f32 {
        self.lhs
    }
}

pub mod util {
    use crate::parser::Expression;

    pub fn print_tree(expr: &Expression) {
        print_tree_inner(expr, false, 0, false);
        println!()
    }

    fn print_tree_inner(expr: &Expression, new_line: bool, depth: usize, bottom: bool) {
        // Color if bottom
        // if bottom {
        //     print!("\x1b[33m");
        // } else {
        //     print!("\x1b[39m");
        // }
        if new_line {
            for i in 0..depth {
                if i == depth - 1 {
                    print!("        └─▶ ")
                } else if !bottom {
                    print!("        │   ")
                } else {
                    print!("            ")
                }
            }
        }

        match expr {
            Expression::Number(n) => print!("{n}"),
            Expression::Op(op) => {
                print!("{} ──┬─▶ ", op.typ);
                print_tree_inner(&op.lhs, false, depth + 1, false);
                println!();
                print_tree_inner(
                    &op.rhs,
                    true,
                    depth + 1,
                    if depth == 0 { true } else { bottom },
                );
            }
        }
    }
}

mod token {
    pub const OPERATORS: &[char] = &['+', '-', '*', '/'];
    #[derive(Debug, Clone, Copy)]
    pub enum Token {
        Number(i64),
        Operator(char),
    }
}

mod error {
    use std::{error::Error, fmt::Display, num::ParseIntError};

    use super::token::Token;

    #[derive(Debug)]
    pub enum LexError {
        InvalidNumber(ParseIntError),
        InvalidOperator(char),
        UnknownInput(String),
    }

    impl From<ParseIntError> for LexError {
        fn from(value: ParseIntError) -> Self {
            Self::InvalidNumber(value)
        }
    }

    impl Display for LexError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LexError::InvalidNumber(e) => write!(f, "{e}"),
                LexError::InvalidOperator(c) => write!(f, "invalid operator '{c}'"),
                LexError::UnknownInput(s) => write!(f, "unknown input: {s}"),
            }
        }
    }


    impl Error for LexError {}
}
pub fn lex(src: impl AsRef<str>) -> Result<Vec<token::Token>, LexError> {
    let mut tokens = Vec::new();
    for s in src.as_ref().split_whitespace() {
        let t: Result<Token, LexError> = if s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Token::Number(s.parse()?))
        } else if s.len() == 1 {
            let c = s.chars().next().unwrap();
            if token::OPERATORS.contains(&c) {
                Ok(Token::Operator(c))
            } else {
                Err(LexError::InvalidOperator(c))
            }
        } else {
            Err(LexError::UnknownInput(s.to_string()))
        };
        tokens.push(t?);
    }
    Ok(tokens)
}
