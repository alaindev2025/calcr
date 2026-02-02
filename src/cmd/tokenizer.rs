use core::panic;

pub struct Tokenizer;

enum Token {
    Value(i32),
    Plus,
    Minus,
}

impl Tokenizer {
    fn tokenize(string: &str) -> Vec<Token> {
        string
            .split_whitespace()
            .map(|s| match s {
                "+" => Token::Plus,
                "-" => Token::Minus,
                val => Token::Value(val.parse().expect("Cannot parse values")),
            })
            .collect()
    }

    fn parse(tokens: Vec<Token>) -> i32 {
        let mut result = 0;
        let mut current_op = Token::Plus;

        for token in tokens {
            match token {
                Token::Value(n) => match current_op {
                    Token::Plus => result += n,
                    Token::Minus => result -= n,
                    _ => panic!("Cannot parse <VALUE> <VALUE>"),
                },
                Token::Plus => current_op = Token::Plus,
                Token::Minus => current_op = Token::Minus,
            }
        }

        result
    }
}

pub fn parse(expr: &str) -> i32 {
    let tokens = Tokenizer::tokenize(expr);
    Tokenizer::parse(tokens)
}
