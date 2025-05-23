#![allow(dead_code)]
use std::fmt::Display;

use error::{ExpressionParseError, LexError, UnknownOperatorTypeError};
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
impl TryFrom<char> for OperatorType {
	type Error = UnknownOperatorTypeError;

	/// Atempt converting a char that represents an operator to its [OperatorType]
	///
	/// Eg.
	/// ```
	///# use calc::parser::OperatorType;
	///let typ: OperatorType = '+'.try_into().unwrap();
	///assert_eq!(typ, OperatorType::Add);
	/// ```
	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'+' => Ok(OperatorType::Add),
			'-' => Ok(OperatorType::Sub),
			'*' => Ok(OperatorType::Mul),
			'/' => Ok(OperatorType::Div),
			'^' => Ok(OperatorType::Pow),
			'%' => Ok(OperatorType::Mod),
			c => Err(c.into()),
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

	/// Display a the operator tree of an [Expression]
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

	#[derive(Debug)]
	pub enum ExpressionParseError {
		UnexpectedToken(Token),
		EndOfInput(String), // Expected token name in String
	}

	impl Display for ExpressionParseError {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				ExpressionParseError::UnexpectedToken(t) => write!(f, "unexpected token '{t:?}'"),
				ExpressionParseError::EndOfInput(expectation) => {
					write!(f, "expected {expectation}, reached end of input")
				}
			}
		}
	}

	#[derive(Debug, Clone, Copy)]
	pub struct UnknownOperatorTypeError {
		op: char,
	}

	impl UnknownOperatorTypeError {
		fn from(c: char) -> Self {
			Self { op: c }
		}
	}

	impl Display for UnknownOperatorTypeError {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "unknown operator tyoe '{}'", self.op)
		}
	}

	impl From<char> for UnknownOperatorTypeError {
		fn from(value: char) -> Self {
			Self::from(value)
		}
	}

	impl Error for LexError {}
	impl Error for ExpressionParseError {}
	impl Error for UnknownOperatorTypeError {}
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

pub fn parse(src: &[Token]) -> Result<Expression, ExpressionParseError> {
	let lhs = Expression::Number(match parse_token(src, false)? {
		Token::Number(n) => n,
		Token::Operator(_) => unreachable!(),
	});
	let op_tok = match parse_token(&src[1..], true) {
		Ok(t) => t,
		Err(e) => match e {
			ExpressionParseError::UnexpectedToken(t) => {
				return Err(ExpressionParseError::UnexpectedToken(t));
			}
			ExpressionParseError::EndOfInput(_) => return Ok(lhs),
		},
	};
	Ok(Expression::Op(Box::new(Operator {
		typ: match op_tok {
			Token::Number(n) => {
				return Err(ExpressionParseError::UnexpectedToken(Token::Number(n)));
			}
			Token::Operator(op) => op.try_into().unwrap(),
		},
		lhs,
		rhs: { parse(&src[2..])? },
	})))
}

fn parse_token(src: &[Token], expecting_op: bool) -> Result<Token, ExpressionParseError> {
	match src.first() {
		Some(t) => match t {
			Token::Operator(_) if expecting_op => Ok(*t),
			Token::Number(_) if !expecting_op => Ok(*t),
			t => Err(ExpressionParseError::UnexpectedToken(*t)),
		},
		None => match expecting_op {
			true => Err(ExpressionParseError::EndOfInput(
				"Token::Operator".to_string(),
			)),
			false => Err(ExpressionParseError::EndOfInput(
				"Token::Number".to_string(),
			)),
		},
	}
}
