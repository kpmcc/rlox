use crate::error;
use crate::is_alpha;
use crate::is_alphanumeric;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    chars: Vec<char>,
    text_buffer: String,
    start: usize,
    current: usize,
    line: i64,
    reserved: HashMap<String, TokenType>,
}

pub fn new_scanner(source: String) -> Scanner {
    let mut s = Scanner {
        source,
        tokens: Vec::new(),
        chars: Vec::new(),
        text_buffer: "".to_string(),
        start: 0,
        current: 0,
        line: 1,
        reserved: HashMap::new(),
    };
    s.chars = s.source.chars().collect();
    s.reserved.insert(String::from("and"), TokenType::And);
    s.reserved.insert(String::from("class"), TokenType::Class);
    s.reserved.insert(String::from("else"), TokenType::Else);
    s.reserved.insert(String::from("false"), TokenType::False);
    s.reserved.insert(String::from("for"), TokenType::For);
    s.reserved.insert(String::from("fun"), TokenType::Fun);
    s.reserved.insert(String::from("if"), TokenType::If);
    s.reserved.insert(String::from("nil"), TokenType::Nil);
    s.reserved.insert(String::from("or"), TokenType::Or);
    s.reserved.insert(String::from("print"), TokenType::Print);
    s.reserved.insert(String::from("return"), TokenType::Return);
    s.reserved.insert(String::from("super"), TokenType::Super);
    s.reserved.insert(String::from("this"), TokenType::This);
    s.reserved.insert(String::from("true"), TokenType::True);
    s.reserved.insert(String::from("var"), TokenType::Var);
    s.reserved.insert(String::from("while"), TokenType::While);
    s
}

impl Scanner {
    fn parse_identifier(&mut self) -> Option<Token> {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = &self.text_buffer;
        match self.reserved.get(text).copied() {
            Some(t) => self.add_token(t),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn parse_number(&mut self) -> Option<Token> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && (self.peek_next().is_ascii_digit()) {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let num = self.text_buffer.parse::<f64>().unwrap();
        Some(self.add_token_float_literal(TokenType::Number, num))
    }

    fn parse_string(&mut self) -> Option<Token> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.".to_string());
        }

        // For the closing "
        let _ = self.advance();

        let substring: &[char] = &self.chars[self.start + 1..self.current - 1];
        let mut s: String = String::new();
        for c in substring {
            s += &c.to_string();
        }

        Some(self.add_token_string_literal(TokenType::String, s))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&self, t: TokenType) -> Option<Token> {
        match t {
            TokenType::False => Some(self.add_token_bool_literal(t, false)),
            TokenType::True => Some(self.add_token_bool_literal(t, true)),
            _ => Some(self.add_token_base(t)),
        }
    }

    fn add_token_base(&self, t: TokenType) -> Token {
        Token {
            tok_type: t,
            lexeme: self.text_buffer.clone(),
            line: self.line,
            bool_literal: false,
            float_literal: 0.0,
            string_literal: String::new(),
        }
    }

    fn add_token_string_literal(&self, t: TokenType, literal: String) -> Token {
        let mut t = self.add_token_base(t);
        t.string_literal = literal;
        t
    }

    fn add_token_float_literal(&self, t: TokenType, literal: f64) -> Token {
        let mut t = self.add_token_base(t);
        t.float_literal = literal;
        t
    }

    fn add_token_bool_literal(&self, t: TokenType, literal: bool) -> Token {
        let mut t = self.add_token_base(t);
        t.bool_literal = literal;
        t
    }

    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.text_buffer += &c.to_string();
        self.current += 1;
        c
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.chars[self.current + 1];
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.chars[self.current];
    }

    fn tok_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.chars[self.current] != expected {
            return false;
        }
        self.text_buffer += &expected.to_string();
        self.current += 1;
        return true;
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut new_tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.advance();
            let t = match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                ' ' => None,
                '\r' => None,
                '\t' => None,
                '\n' => {
                    self.line += 1;
                    None
                }
                '!' => {
                    if self.tok_match('=') {
                        self.add_token(TokenType::BangEqual)
                    } else {
                        self.add_token(TokenType::Bang)
                    }
                }
                '=' => {
                    if self.tok_match('=') {
                        self.add_token(TokenType::EqualEqual)
                    } else {
                        self.add_token(TokenType::Equal)
                    }
                }
                '<' => {
                    if self.tok_match('=') {
                        self.add_token(TokenType::GreaterEqual)
                    } else {
                        self.add_token(TokenType::Greater)
                    }
                }
                '>' => {
                    if self.tok_match('=') {
                        self.add_token(TokenType::LessEqual)
                    } else {
                        self.add_token(TokenType::Less)
                    }
                }
                '/' => {
                    if self.tok_match('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                        None
                    } else {
                        self.add_token(TokenType::Slash)
                    }
                }
                '"' => self.parse_string(),
                _ => {
                    if c.is_ascii_digit() {
                        self.parse_number()
                    } else if is_alpha(c) {
                        self.parse_identifier()
                    } else {
                        let msg = format!("Unexpected character: {}", c);
                        error(self.line, msg);
                        None
                    }
                }
            };
            if let Some(tok) = t {
                new_tokens.push(tok)
            }
            self.text_buffer = String::new();
        }
        new_tokens.push(self.add_token_base(TokenType::EOF));
        return new_tokens;
    }
}
