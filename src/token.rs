#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
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

#[derive(Clone, Debug)]
pub struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
    pub line: i64,
    pub bool_literal: bool,
    pub float_literal: f64,
    pub string_literal: String,
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
