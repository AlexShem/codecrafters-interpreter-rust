use std::env;
use std::fs;
use std::io::{self, Write};

enum TokenType {
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
    Unknown(String),
}

fn tokenize(content: String) -> Vec<TokenType> {
    let chars = content.chars();
    let mut tokens: Vec<TokenType> = Vec::with_capacity(content.len());

    for char in chars {
        let token = match char.to_string().as_str() {
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
            ch => TokenType::Unknown(ch.to_string()),
        };
        tokens.push(token);
    }
    tokens
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let tokens = tokenize(file_contents);
                for token in tokens {
                    match token {
                        TokenType::LeftParen => println!("LEFT_PAREN ( null"),
                        TokenType::RightParen => println!("RIGHT_PAREN ) null"),
                        TokenType::LeftBrace => println!("LEFT_BRACE {{ null"),
                        TokenType::RightBrace => println!("RIGHT_BRACE }} null"),
                        TokenType::Comma => println!("COMMA , null"),
                        TokenType::Dot => println!("DOT . null"),
                        TokenType::Minus => println!("MINUS - null"),
                        TokenType::Plus => println!("PLUS + null"),
                        TokenType::Semicolon => println!("SEMICOLON ; null"),
                        TokenType::Star => println!("STAR * null"),
                        TokenType::Unknown(ch) => println!("Unknown Token {} null", ch),
                    }
                }
                println!("EOF  null");
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
