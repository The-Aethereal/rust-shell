
use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Word(s) => write!(f, "{}", s),
            Token::Pipe => write!(f, "|"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Pipe,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // 1️⃣ Skip whitespace
            ' ' | '\t' | '\n' => {
                chars.next();
            }

            // 2️⃣ Pipe operator
            '|' => {
                chars.next();
                tokens.push(Token::Pipe);
            }

            // 3️⃣ Quoted string
            '"' => {
                chars.next(); // consume opening quote
                let mut value = String::new();

                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    value.push(c);
                }

                tokens.push(Token::Word(value));
            }

            // 4️⃣ Normal word
            _ => {
                let mut value = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() || c == '|' {
                        break;
                    }
                    value.push(c);
                    chars.next();
                }

                tokens.push(Token::Word(value));
            }
        }
    }

    Ok(tokens)
}
