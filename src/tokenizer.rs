use std::fmt::{Display, Formatter};

pub enum TokenType {
    /// `(` Left parenthesis
    LeftParen,
    /// `)` Right parenthesis
    RightParen,
    /// `{` Left brace
    LeftBrace,
    /// `}` Right brace
    RightBrace,
    /// `,` Comma
    Comma,
    /// `.` Dot
    Dot,
    /// `-` Minus
    Minus,
    /// `+` Plus
    Plus,
    /// `;` Semicolon
    Semicolon,
    /// `*` Star
    Star,
    /// `=` Assignment Operator
    Equal,
    /// `==` Equality Operator
    EqualEqual,
    /// `!` Negation sign
    Bang,
    /// `!=` Inequality operator
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    /// End of string
    Eof,
    UnknownToken(String),
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    #[allow(unused)]
    line: i32,
}

struct LoxError {
    line: i32,
    message: String,
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
    errors: Option<Vec<LoxError>>,
}

impl LoxError {
    fn new(line: i32, error: String) -> Self {
        Self {
            line,
            message: error,
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: i32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        match &self.literal {
            None => format!("{} {} null", self.token_type, self.lexeme),
            Some(literal) => format!("{} {} {}", self.token_type, self.lexeme, literal),
        }
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: None,
        }
    }

    pub fn has_errors(&self) -> bool {
        self.errors.is_some()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn current_char(&self) -> &str {
        &self.source.as_str()[(self.start as usize)..(self.current as usize)]
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self
            .source
            .get(self.start as usize..(self.current as usize))
            .unwrap_or("");
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line))
    }

    pub fn scan_tokens(&mut self) {
        let mut errors: Vec<LoxError> = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            let (token_type, literal) = self.scan_token();
            if let TokenType::UnknownToken(unknown_token) = token_type {
                let error = LoxError::new(
                    self.line.clone(),
                    format!("Unexpected character: {}", unknown_token),
                );
                errors.push(error);
            } else {
                self.add_token(token_type, literal);
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));

        if !errors.is_empty() {
            self.errors = Some(errors);
        }
    }

    fn scan_token(&mut self) -> (TokenType, Option<String>) {
        self.advance();
        let result = match self.current_char() {
            "(" => (TokenType::LeftParen, None),
            ")" => (TokenType::RightParen, None),
            "{" => (TokenType::LeftBrace, None),
            "}" => (TokenType::RightBrace, None),
            "," => (TokenType::Comma, None),
            "." => (TokenType::Dot, None),
            "-" => (TokenType::Minus, None),
            "+" => (TokenType::Plus, None),
            ";" => (TokenType::Semicolon, None),
            "*" => (TokenType::Star, None),
            "=" => {
                if self.matches_next("=") {
                    (TokenType::EqualEqual, None)
                } else {
                    (TokenType::Equal, None)
                }
            }
            "!" => {
                if self.matches_next("=") {
                    (TokenType::BangEqual, None)
                } else {
                    (TokenType::Bang, None)
                }
            }
            "<" => {
                if self.matches_next("=") {
                    (TokenType::LessEqual, None)
                } else {
                    (TokenType::Less, None)
                }
            }
            ">" => {
                if self.matches_next("=") {
                    (TokenType::GreaterEqual, None)
                } else {
                    (TokenType::Greater, None)
                }
            }
            ch => (TokenType::UnknownToken(ch.to_string()), None),
        };

        result
    }

    fn matches_next(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        };
        match self
            .source
            .get((self.current as usize)..(self.current as usize + 1))
        {
            None => false,
            Some(next_char) => {
                if next_char != expected {
                    false
                } else {
                    self.current += 1;
                    true
                }
            }
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Star => write!(f, "STAR"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::UnknownToken(message) => write!(f, "Unknown token {}", message),
        }
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(errors) = &self.errors {
            errors.iter().for_each(|error| {
                eprintln!("[line {}] Error: {}", error.line, error.message);
            })
        }

        Ok(self.tokens.iter().for_each(|token| {
            writeln!(f, "{}", token.to_string()).expect("Failed to represent TokenType as string");
        }))
    }
}
