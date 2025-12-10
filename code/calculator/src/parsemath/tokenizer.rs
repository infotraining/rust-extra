use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
}

#[derive(Debug, PartialEq)]
pub struct TokenizingError {}

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &str) -> Tokenizer<'_> {
        Tokenizer {
            expr: expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizingError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.expr.next() {
            match c {
                '+' => Some(Ok(Token::Plus)),
                _ => Some(Err(TokenizingError {})),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn tokenize_operator() {
        let mut tokenizer = Tokenizer::new("+");

        let result = tokenizer.next();
        assert_eq!(result, Some(Ok(Token::Plus)));
        assert_eq!(tokenizer.next(), None);
    }
}
