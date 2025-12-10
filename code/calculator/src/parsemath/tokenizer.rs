use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen
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
                '-' => Some(Ok(Token::Minus)),
                '*' => Some(Ok(Token::Star)),
                '/' => Some(Ok(Token::Slash)),
                '(' => Some(Ok(Token::LeftParen)),
                ')' => Some(Ok(Token::RightParen)),
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
    use rstest::rstest;

    #[rstest]
    #[case("+", Token::Plus)]
    #[case("-", Token::Minus)]
    #[case("/", Token::Slash)]
    #[case("*", Token::Star)]
    fn tokenize_operator(#[case] expr: &str, #[case] expected_token: Token) {
        let mut tokenizer = Tokenizer::new(expr);

        let result = tokenizer.next();
        assert_eq!(result, Some(Ok(expected_token)));
        assert_eq!(tokenizer.next(), None);
    }

    #[rstest]
    #[case("(", vec![Token::LeftParen])]
    #[case(")", vec![Token::RightParen])]
    fn tokenizer_parens(#[case] expr: &str, #[case] expected_tokens: Vec<Token>) {
        let tokenizer = Tokenizer::new(expr);

        let tokens: Vec<Token> = tokenizer.collect::<Result<Vec<Token>, TokenizingError>>().unwrap();
        assert_eq!(tokens, expected_tokens);
    }
}
