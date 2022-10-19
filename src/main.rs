use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Copy, Clone, Debug)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

struct Symbol;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    chars: Vec<char>,
    text_buffer: String,
    start: usize,
    current: usize,
    line: i64,
    reserved: HashMap<String, TokenType>,
}

fn new_scanner(source: String) -> Scanner {
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

fn is_alpha(c: char) -> bool {
    let b = c.is_ascii_alphabetic() || (c == '_');
    b
}

fn is_alphanumeric(c: char) -> bool {
    let an = is_alpha(c) || c.is_ascii_digit();
    an
}

fn get_double_token_type_with_possible_equals(c: &char, o: Option<&char>) -> Option<TokenType> {
    match o {
        Some(x) => {
            if *x == '=' {
                match c {
                    '!' => Some(TokenType::BangEqual),
                    '=' => Some(TokenType::EqualEqual),
                    '<' => Some(TokenType::GreaterEqual),
                    '>' => Some(TokenType::LessEqual),
                    _ => None, // TODO
                }
            } else {
                get_double_token_type(c)
            }
        }
        None => get_double_token_type(c),
    }
}

fn get_double_token_type(c: &char) -> Option<TokenType> {
    match c {
        '!' => Some(TokenType::Bang),
        '=' => Some(TokenType::Equal),
        '<' => Some(TokenType::Greater),
        '>' => Some(TokenType::Less),
        _ => None, // TODO
    }
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

        Some(self.add_token_string_literal(TokenType::String, s.clone()))
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
        let text: String = self.text_buffer.clone(); // TODO this seems v inefficient
        Token {
            tok_type: t,
            lexeme: text,
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

    fn scan_tokens(&mut self) -> Vec<Token> {
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

struct Token {
    tok_type: TokenType,
    lexeme: String,
    line: i64,
    bool_literal: bool,
    float_literal: f64,
    string_literal: String,
}

fn build_token(
    tok_type: TokenType,
    lexeme: String,
    line: i64,
    bool_literal: bool,
    float_literal: f64,
    string_literal: String,
) -> Token {
    Token {
        tok_type,
        lexeme,
        line,
        bool_literal,
        float_literal,
        string_literal,
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Minus => "Minus",
            TokenType::Plus => "Plus",
            TokenType::Semicolon => "Semicolon",
            TokenType::Slash => "Slash",
            TokenType::Star => "Star",
            TokenType::Bang => "Bang",
            TokenType::BangEqual => "BangEqual",
            TokenType::Equal => "Equal",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::Greater => "Greater",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::Less => "Less",
            TokenType::LessEqual => "LessEqual",

            TokenType::Identifier => "Identifier",
            TokenType::String => "String",
            TokenType::Number => "Number",

            TokenType::And => "And",
            TokenType::Class => "Class",
            TokenType::Else => "Eles",
            TokenType::False => "False",
            TokenType::Fun => "Fun",
            TokenType::For => "For",
            TokenType::If => "If",
            TokenType::Nil => "Nil",
            TokenType::Or => "Or",
            TokenType::Print => "Print",
            TokenType::Return => "Return",
            TokenType::Super => "Super",
            TokenType::This => "This",
            TokenType::True => "True",
            TokenType::Var => "Var",
            TokenType::While => "While",

            TokenType::EOF => "EOF",
        };

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match self.tok_type {
            TokenType::Number => {
                format!("{}", &self.float_literal)
            }
            TokenType::String => {
                format!("{}", &self.string_literal)
            }
            TokenType::True => {
                format!("{}", &self.bool_literal)
            }
            TokenType::False => {
                format!("{}", &self.bool_literal)
            }
            TokenType::Nil => {
                format!("{}", "Nil")
            }
            _ => {
                format!("{}", "Other")
            }
        };
        write!(
            f,
            "({}, {}, {}, {})",
            self.tok_type, self.lexeme, literal, self.line
        )
    }
}

fn error(line: i64, message: String) {
    let s = "";
    report(line, s.to_string(), message);
}

fn report(line: i64, location: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}

fn run(_s: String) {
    let mut scanner = new_scanner(_s);
    let tokens = scanner.scan_tokens();
    for t in tokens {
        println!("{}", t);
    }
}

fn run_file(path: String) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    Ok(run(contents))
}

fn run_prompt() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        print!("> ");
        io::stdout().flush()?;
        let read_len = io::stdin().read_line(&mut buffer)?;
        if read_len == 0 {
            break;
        } else {
            run(buffer.clone());
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64)
    } else if args.len() == 2 {
        run_file(args[1].to_string())?;
    } else {
        run_prompt()?;
    }
    Ok(())
}
