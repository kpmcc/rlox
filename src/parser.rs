use crate::token::{Token, TokenType};
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
enum LiteralType<'a> {
    Bool { lit: bool },
    Float { lit: f64 },
    String { lit: &'a str },
    Nil { lit: bool },
}

enum Expression {
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

enum Operator {
    Minus,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    current: usize,
    previous: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: VecDeque::from(tokens),
            current: 0,
            previous: Token {
                tok_type: TokenType::EOF,
                lexeme: String::new(),
                line: -1,
                bool_literal: false,
                float_literal: 0.0,
                string_literal: String::new(),
            },
        }
    }

    fn expression(&mut self) -> Box<Expression> {
        return self.equality();
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

    fn equality(&mut self) -> Box<Expression> {
        let mut expr = self.comparison();

        loop {
            let (m, t) = self.tok_match(vec![TokenType::BangEqual, TokenType::EqualEqual]);
            if !m {
                break;
            }

            match t {
                Some(t) => {
                    let operator = t;
                    let right = self.comparison();
                    expr = Box::new(Expression::Binary {
                        left: expr,
                        operator,
                        right,
                    });
                }
                None => {
                    break;
                }
            }
        }

        expr
    }

    fn advance(&mut self) -> Option<Token> {
        let t = &self.tokens[0];
        if t.tok_type != TokenType::EOF {
            return self.tokens.pop_front();
        } else {
            return None;
        }
    }

    fn comparison(&mut self) -> Box<Expression> {
        let expr = Box::new(Expression::Literal {
            lit: (Token {
                tok_type: TokenType::True,
                lexeme: "True".to_string(),
                line: 0,
                bool_literal: true,
                float_literal: 0.0,
                string_literal: "".to_string(),
            }),
        });
        expr
    }
}
