use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    Symbol(String),
    Integer(i64),
    Float(f64),
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "{}", i),
            Token::Float(fl) => write!(f, "{}", fl),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct TokenError {
    ch: char,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected character: {}", self.ch)
    }
}

impl Error for TokenError {}

pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    let program = program.replace("(", " ( ").replace(")", " ) ");
    let words = program.split_whitespace();
    let mut tokens = Vec::new();
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let i = word.parse::<i64>();
                match i {
                    Ok(i) => tokens.push(Token::Integer(i)),
                    Err(_) => {
                        let f = word.parse::<f64>();
                        match f {
                            Ok(f) => tokens.push(Token::Float(f)),
                            Err(_) => tokens.push(Token::Symbol(word.to_string())),
                        }
                    }
                }
            }
        }
    }
    Ok(tokens)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let tokens = tokenize("(+ 1 2)").unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        )
    }

    #[test]
    fn it_works_with_floats() {
        let tokens = tokenize("(+ 1.0 2.0)").unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Float(1.0),
                Token::Float(2.0),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn circle_area() {
        let program = "
            (
              (define r 10)
              (define pi 3)
              (* pi (* r r))
            )
            ";

        let tokens = tokenize(program).unwrap_or(vec![]);

        assert_eq!(
            tokens,
            [
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("pi".to_string()),
                Token::Integer(3),
                Token::RParen,
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen
            ]
        );
    }


    #[test]
    fn circle_area_with_float() {
        let program = "
            (
              (define r 10)
              (define pi 3.14)
              (* pi (* r r))
            )
            ";

        let tokens = tokenize(program).unwrap_or(vec![]);

        assert_eq!(
            tokens,
            [
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("pi".to_string()),
                Token::Float(3.14),
                Token::RParen,
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen
            ]
        );
    }
}
