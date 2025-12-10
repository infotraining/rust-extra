use std::iter::Peekable;
use std::str::Chars;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    Number(f64),
}

#[derive(Error, Debug, PartialEq)]
pub enum TokenizingError {
    #[error("Unexpected token '{0}'")]
    InvalidCharacter(char),
    #[error("Invalid number format")]
    InvalidNumber,
}

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &str) -> Tokenizer<'_> {
        Tokenizer {
            expr: expr.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(' ') = self.expr.peek() {
            self.expr.next();
        }
    }

    fn tokenize_number(&mut self, c: char) -> Result<Token, TokenizingError> {
        let mut number_str = c.to_string();
        while let Some('0'..='9') | Some('.') = self.expr.peek() {
            number_str.push(self.expr.next().unwrap());
        }

        let number = number_str
            .parse::<f64>()
            .map(|n| Token::Number(n))
            .map_err(|_| TokenizingError::InvalidNumber);

        number
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizingError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if let Some(c) = self.expr.next() {
            match c {
                '+' => Some(Ok(Token::Plus)),
                '-' => Some(Ok(Token::Minus)),
                '*' => Some(Ok(Token::Star)),
                '/' => Some(Ok(Token::Slash)),
                '(' => Some(Ok(Token::LeftParen)),
                ')' => Some(Ok(Token::RightParen)),
                '0'..='9' => Some(self.tokenize_number(c)),
                invalid => Some(Err(TokenizingError::InvalidCharacter(invalid))),
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

        let tokens: Vec<Token> = tokenizer
            .collect::<Result<Vec<Token>, TokenizingError>>()
            .unwrap();
        assert_eq!(tokens, expected_tokens);
    }

    #[rstest]
    #[case("3", vec![Token::Number(3.0)])]
    #[case("3.14", vec![Token::Number(3.14)])]
    #[case("93.14", vec![Token::Number(93.14)])]
    fn tokenizer_numbers(#[case] expr: &str, #[case] expected_tokens: Vec<Token>) {
        let tokenizer = Tokenizer::new(expr);

        let tokens: Vec<Token> = tokenizer
            .collect::<Result<Vec<Token>, TokenizingError>>()
            .unwrap();
        assert_eq!(tokens, expected_tokens);
    }

    #[rstest]
    #[case("1a", 'a')]
    #[case("2#", '#')]
    fn tokenizer_invalid_characters(#[case] expr: &str, #[case] expected: char) {
        let tokenizer = Tokenizer::new(expr);

        let result = tokenizer
            .collect::<Result<Vec<Token>, TokenizingError>>()
            .unwrap_err();

        assert_eq!(result, TokenizingError::InvalidCharacter(expected));
    }

    #[rstest]
    #[case("1.324.3")]
    #[case("1....")]
    #[case("1 + 3.33.3.3")]
    fn tokenizer_invalid_number(#[case] expr: &str) {
        let tokenizer = Tokenizer::new(expr);

        let result = tokenizer
            .collect::<Result<Vec<Token>, TokenizingError>>()
            .unwrap_err();
        assert_eq!(result, TokenizingError::InvalidNumber);

        let error_msg = format!("{}", result);
        assert_eq!(error_msg, "Invalid number format");
    }

    #[rstest]
    #[case("1+2", vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)])]
    #[case("1 + 2", vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)])]
    fn tokenizer_expressions(#[case] expr: &str, #[case] expected_tokens: Vec<Token>) {
        let tokenizer = Tokenizer::new(expr);

        let tokens = tokenizer
            .collect::<Result<Vec<Token>, TokenizingError>>()
            .unwrap();
        assert_eq!(tokens, expected_tokens);
    }
}
