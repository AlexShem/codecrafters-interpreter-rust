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
    UnknownToken(String),
}

struct LoxError {
    line: u32,
    message: String,
}

pub struct Tokenizer {
    tokens: Vec<TokenType>,
    errors: Option<Vec<LoxError>>,
}

impl LoxError {
    fn new(line: u32, error: String) -> Self {
        Self {
            line,
            message: error,
        }
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            errors: None,
        }
    }

    pub fn has_errors(&self) -> bool {
        self.errors.is_some()
    }

    pub fn tokenize_file(&mut self, content: String) {
        let chars = content.chars();
        let mut tokens: Vec<TokenType> = Vec::with_capacity(content.len());
        let mut errors: Vec<LoxError> = Vec::new();

        for char in chars {
            let token = self.tokenize(char.to_string().as_str());
            if let TokenType::UnknownToken(lexical_error) = token {
                errors.push(LoxError::new(
                    1,
                    format!("Error: Unexpected character: {}", lexical_error),
                ));
            } else {
                tokens.push(token);
            }
        }

        self.tokens = tokens;
        if !errors.is_empty() {
            self.errors = Some(errors);
        }
    }

    fn tokenize(&self, token: &str) -> TokenType {
        match token {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "," => TokenType::Comma,
            "." => TokenType::Dot,
            "-" => TokenType::Minus,
            "+" => TokenType::Plus,
            ";" => TokenType::Semicolon,
            "*" => TokenType::Star,
            ch => TokenType::UnknownToken(ch.to_string()),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN ( null"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN ) null"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            TokenType::Comma => write!(f, "COMMA , null"),
            TokenType::Dot => write!(f, "DOT . null"),
            TokenType::Minus => write!(f, "MINUS - null"),
            TokenType::Plus => write!(f, "PLUS + null"),
            TokenType::Semicolon => write!(f, "SEMICOLON ; null"),
            TokenType::Star => write!(f, "STAR * null"),
            TokenType::UnknownToken(message) => write!(f, "Error: Unknown token {}", message),
        }
    }
}

impl Display for Tokenizer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(errors) = &self.errors {
            errors.iter().for_each(|error| {
                eprintln!("[line {}] {}", error.line, error.message);
            })
        }

        self.tokens.iter().for_each(|token| {
            writeln!(f, "{}", token).expect("Failed to represent TokenType as string");
        });

        write!(f, "EOF  null")
    }
}
