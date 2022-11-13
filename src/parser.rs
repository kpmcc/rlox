use crate::token::{Token, TokenType};
use std::collections::VecDeque;
use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub enum LiteralType {
    Bool { lit: bool },
    Float { lit: f64 },
    String { lit: String },
    Nil { lit: bool },
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            LiteralType::Bool { lit } => {
                write!(f, "LiteralType - Bool: {}", lit)
            }
            LiteralType::Float { lit } => {
                write!(f, "LiteralType - Float: {}", lit)
            }
            LiteralType::String { lit } => {
                write!(f, "LiteralType - String: {}", lit)
            }
            LiteralType::Nil { lit } => {
                write!(f, "LiteralType - Nil")
            }
        }
    }
}

pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Literal {
        lit: Token,
    },
    Grouping {
        group: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator, *left, *right)
            }
            Expression::Unary { operator, right } => {
                write!(f, "({} {})", operator, *right)
            }
            Expression::Literal { lit } => {
                write!(f, "({})", lit)
            }
            Expression::Grouping { group } => {
                write!(f, "(group {})", *group)
            }
        }
    }
}

enum Operator {
    Minus,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    had_error: bool,
}

#[derive(Debug)]
pub struct ParseError {
    line: i64,
    msg: String,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.line, self.msg)
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: VecDeque::from(tokens),
            had_error: false,
        }
    }
    fn is_at_end(&self) -> bool {
        if let Some(t) = self.tokens.front() {
            return t.tok_type == TokenType::EOF;
        } else {
            return false;
        }
    }

    fn check(&self, tt: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(t) = self.tokens.front() {
            return t.tok_type == tt;
        } else {
            return false;
        }
    }

    fn tok_match(&mut self, candidates: Vec<TokenType>) -> (bool, Option<Token>) {
        let mut m = false;
        for c in candidates {
            m = self.check(c);
            if m {
                break;
            }
        }
        if m {
            return (m, self.advance());
        } else {
            return (m, None);
        }
    }

    pub fn parse(&mut self) -> Result<Option<Expression>, ParseError> {
        if self.tokens.len() == 1 && self.tokens[0].tok_type == TokenType::EOF {
            return Ok(None);
        }
        let expr = self.expression();
        match expr {
            Ok(expr) => Ok(Some(expr)),
            Err(e) => {
                eprintln!("{}", e);
                Err(e)
            }
        }
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        //println!("Parsing expression");
        let expr = self.equality()?;
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        //println!("Parsing equality");
        let mut expr = self.comparison()?;

        loop {
            let (m, t) = self.tok_match(vec![TokenType::BangEqual, TokenType::EqualEqual]);
            if !m {
                break;
            }

            match t {
                Some(t) => {
                    let operator = t;
                    let right = self.comparison()?;
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                None => {
                    break;
                }
            }
        }

        Ok(expr)
    }

    fn advance(&mut self) -> Option<Token> {
        let t = &self.tokens[0];
        if t.tok_type != TokenType::EOF {
            return self.tokens.pop_front();
        } else {
            return None;
        }
    }

    fn error(&mut self, t: Token, m: String) -> ParseError {
        self.had_error = true;
        let msg: String;
        if t.tok_type == TokenType::EOF {
            msg = format!("at end {}", m);
        } else {
            msg = format!("at '{}' {}", t.lexeme, m)
        }
        ParseError { line: t.line, msg }
    }

    fn synchronize(&mut self) {
        while !self.is_at_end() {
            let previous = self.advance();
            let previous = match previous {
                Some(previous) => previous,
                None => {
                    // This might be a parseerror??
                    return;
                }
            };
            if previous.tok_type == TokenType::Semicolon {
                return;
            }

            if let Some(t) = self.tokens.front() {
                if matches!(
                    t.tok_type,
                    TokenType::Class
                        | TokenType::Fun
                        | TokenType::Var
                        | TokenType::For
                        | TokenType::If
                        | TokenType::While
                        | TokenType::Print
                        | TokenType::Return
                ) {
                    return;
                }
            } else {
                return;
            }
        }
    }

    fn consume(&mut self, tt: TokenType, msg: String) -> Result<Option<Token>, ParseError> {
        if self.check(tt) {
            return Ok(self.advance());
        } else {
            let t = self.tokens.front();
            match t {
                Some(t) => {
                    let r = t.clone();
                    let err = self.error(r, msg);
                    return Err(err);
                }
                None => {
                    panic!("consume - no tokens remaining")
                }
            }
        }
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        //println!("Parsing primary");
        let (m, t) = self.tok_match(vec![
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ]);
        if m {
            match t {
                Some(t) => {
                    //println!("Got {}", t);
                    let b = Expression::Literal { lit: (t) };
                    return Ok(b);
                }
                None => {
                    panic!("primary: tok_match returned True and None")
                }
            }
        }

        let (m, t) = self.tok_match(vec![TokenType::LeftParen]);
        if m {
            match t {
                Some(_) => {
                    let expr = self.expression()?;
                    self.consume(
                        TokenType::RightParen,
                        "Expect ')' after expression.".to_string(),
                    )?;
                    return Ok(Expression::Grouping {
                        group: Box::new(expr),
                    });
                }
                None => {
                    panic!("primary: tok_match returned True and None")
                }
            }
        }
        if let Some(t) = self.tokens.front() {
            Err(self.error(t.clone(), "Expect expression.".to_string()))
        } else {
            panic!("primary - no tokens remaining")
        }
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        //println!("parsing unary");
        let (m, t) = self.tok_match(vec![TokenType::Bang, TokenType::Minus]);
        if !m {
            let p = self.primary()?;
            return Ok(p);
        }
        match t {
            Some(t) => {
                let operator = t;
                let right = self.unary()?;
                return Ok(Expression::Unary {
                    operator,
                    right: Box::new(right),
                });
            }
            None => {
                if let Some(t) = self.tokens.front() {
                    let e = self.error(t.clone(), "Expected unary.".to_string());
                    Err(e)
                } else {
                    panic!("unary - no tokens remaining")
                }
            }
        }
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;
        loop {
            let (m, t) = self.tok_match(vec![TokenType::Slash, TokenType::Star]);
            if !m {
                break;
            }

            match t {
                Some(t) => {
                    let operator = t;
                    let right = self.unary()?;
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                None => {
                    break;
                }
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        //println!("parsing term");
        let mut expr = self.factor()?;
        loop {
            let (m, t) = self.tok_match(vec![TokenType::Minus, TokenType::Plus]);
            if !m {
                break;
            }

            match t {
                Some(t) => {
                    let operator = t;
                    let right = self.term()?;
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                None => {
                    break;
                }
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        //println!("parsing comparison");
        let mut expr = self.term()?;
        loop {
            let (m, t) = self.tok_match(vec![
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ]);
            if !m {
                break;
            }

            match t {
                Some(t) => {
                    let operator = t;
                    let right = self.term()?;
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                None => {
                    break;
                }
            }
        }
        Ok(expr)
    }
}
