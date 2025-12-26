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
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        if ch == '|' {
            chars.next();
            tokens.push(Token::Pipe);
            continue;
        }
        //else start a word.
        let mut value = String::new();
        while let Some(&c) = chars.peek() {

            if c.is_whitespace() || c == '|' {
                break;
            }
            match c {

            '"' => {  //handling quotation strng
                chars.next(); // consume opening quote

                while let Some(ch2) = chars.next() {
                    if ch2 == '"' {
                        break;
                    }
                    value.push(ch2);
                }

            }
            '\'' => {  
                chars.next(); 

                while let Some(ch2) = chars.next() {
                    if ch2 == '\'' {
                        break;
                    }
                    value.push(ch2);
                }
            }
            
            _ => {  //fallback(for normal word)
                    value.push(c);
                    chars.next();
                }

            }
        }
                tokens.push(Token::Word(value));
    }

    Ok(tokens)
}
