use std::{error::Error, fmt};

use crate::parser::{Expression, LiteralType};
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct InterpreterError {
    pub tok: Token,
    pub msg: String,
}

impl Error for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

fn is_truthy(lit: &LiteralType) -> bool {
    match lit {
        LiteralType::Nil { lit } => false,
        LiteralType::Bool { lit } => lit.clone(),
        LiteralType::Float { lit } => true,
        LiteralType::String { lit } => true,
    }
}

fn is_equal(a: LiteralType, b: LiteralType) -> bool {
    match a {
        LiteralType::Nil { lit } => match b {
            LiteralType::Nil { lit } => true,
            _ => false,
        },
        LiteralType::Bool { lit: al } => match b {
            LiteralType::Bool { lit: bl } => (al == bl),
            _ => false,
        },
        LiteralType::Float { lit: al } => match b {
            LiteralType::Float { lit: bl } => (al == bl),
            _ => false, // maybe panic?
        },
        LiteralType::String { lit: al } => match b {
            LiteralType::String { lit: bl } => (al == bl),
            _ => false, // maybe panic?
        },
    }
}

fn check_number_operands(
    left: &LiteralType,
    right: &LiteralType,
    operator: &Token,
) -> Result<bool, InterpreterError> {
    let mut rv = true;
    match left {
        LiteralType::Bool { lit } => rv &= false,
        LiteralType::String { lit } => rv &= false,
        LiteralType::Nil { lit } => rv &= false,
        LiteralType::Float { lit } => rv &= true,
    }
    match right {
        LiteralType::Bool { lit } => rv &= false,
        LiteralType::String { lit } => rv &= false,
        LiteralType::Nil { lit } => rv &= false,
        LiteralType::Float { lit } => rv &= true,
    }
    if rv {
        Ok(rv)
    } else {
        Err(InterpreterError {
            tok: operator.clone(),
            msg: "Operands must be numbers.".to_string(),
        })
    }
}

fn check_same_literal_type(
    left: &LiteralType,
    right: &LiteralType,
    operator: &Token,
) -> Result<bool, InterpreterError> {
    let mut rv = true;
    match left {
        LiteralType::Float { lit } => match right {
            LiteralType::Float { lit } => rv &= true,
            _ => rv &= false,
        },
        LiteralType::String { lit } => match right {
            LiteralType::String { lit } => rv &= true,
            _ => rv &= false,
        },
        _ => rv &= false,
    }
    if rv {
        Ok(rv)
    } else {
        Err(InterpreterError {
            tok: operator.clone(),
            msg: "Operands must be two numbers or two strings.".to_string(),
        })
    }
}

fn evaluate_binary(
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Token,
) -> Result<LiteralType, InterpreterError> {
    let left_lit = evaluate(*left)?;
    let right_lit = evaluate(*right)?;

    if matches!(
        operator.tok_type,
        TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Slash
            | TokenType::Minus
            | TokenType::Star
    ) {
        check_number_operands(&left_lit, &right_lit, &operator)?;
    }

    if operator.tok_type == TokenType::Plus {
        check_same_literal_type(&left_lit, &right_lit, &operator)?;
    }

    let lt = match left_lit {
        LiteralType::Float { lit: l } => match right_lit {
            LiteralType::Float { lit: r } => match operator.tok_type {
                TokenType::Plus => LiteralType::Float { lit: l + r },
                TokenType::Minus => LiteralType::Float { lit: l - r },
                TokenType::Slash => LiteralType::Float { lit: l / r },
                TokenType::Star => LiteralType::Float { lit: l * r },
                TokenType::Greater => LiteralType::Bool { lit: l > r },
                TokenType::GreaterEqual => LiteralType::Bool { lit: l >= r },
                TokenType::Less => LiteralType::Bool { lit: l < r },
                TokenType::LessEqual => LiteralType::Bool { lit: l <= r },
                TokenType::BangEqual => LiteralType::Bool {
                    lit: !is_equal(left_lit, right_lit),
                },
                TokenType::EqualEqual => LiteralType::Bool {
                    lit: is_equal(left_lit, right_lit),
                },
                _ => todo!(),
            },
            _ => {
                panic!()
            }
        },
        LiteralType::String { lit: l } => match right_lit {
            LiteralType::String { lit: r } => match operator.tok_type {
                TokenType::Plus => LiteralType::String {
                    lit: l + r.as_str(),
                },
                TokenType::BangEqual => LiteralType::Bool { lit: !(l == r) },
                TokenType::EqualEqual => LiteralType::Bool { lit: l == r },
                _ => todo!(),
            },
            _ => todo!(),
        },
        _ => todo!(),
    };

    Ok(lt)
}

fn evaluate_unary(
    operator: &Token,
    right: Box<Expression>,
) -> Result<LiteralType, InterpreterError> {
    let r = right;
    let o = operator;
    let right = evaluate(*r)?;
    match o.tok_type {
        TokenType::Bang => {
            let truthy = is_truthy(&right);
            Ok(LiteralType::Bool { lit: !truthy })
        }
        TokenType::Minus => match right {
            LiteralType::Float { lit } => Ok(LiteralType::Float { lit: (lit * -1.0) }),
            _ => Err(InterpreterError {
                tok: o.clone(),
                msg: "Operand must be a number.".to_string(),
            }),
        },
        _ => Err(InterpreterError {
            tok: o.clone(),
            msg: "interpreter - Got TokenType other than Minus or Bang for unary".to_string(),
        }),
    }
}

pub fn evaluate(expr: Expression) -> Result<LiteralType, InterpreterError> {
    let lt = match expr {
        Expression::Binary {
            left,
            operator,
            right,
        } => evaluate_binary(left, right, operator),
        Expression::Unary { operator, right } => evaluate_unary(&operator, right),
        Expression::Grouping { group } => {
            return evaluate(*group);
        }
        Expression::Literal { lit } => {
            if matches!(lit.tok_type, TokenType::False | TokenType::True) {
                return Ok(LiteralType::Bool {
                    lit: lit.bool_literal,
                });
            }
            if matches!(lit.tok_type, TokenType::String) {
                return Ok(LiteralType::String {
                    lit: lit.string_literal,
                });
            }
            if matches!(lit.tok_type, TokenType::Number) {
                return Ok(LiteralType::Float {
                    lit: lit.float_literal,
                });
            }
            panic!("interpreter - could not match literal token type");
        }
    };
    lt
}
