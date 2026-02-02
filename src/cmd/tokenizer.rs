use core::panic;

pub struct Tokenizer;

enum Token {
    Value(i32),
    Plus,
    Minus,
}

impl Tokenizer {
    fn tokenize(args: Vec<String>) -> Vec<Token> {
        args.into_iter()
            .map(|s| match s.as_str() {
                "+" => Token::Plus,
                "-" => Token::Minus,
                val => Token::Value(val.parse().expect("Error: cannot parse values")),
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

pub fn parse(expr: Vec<String>) -> i32 {
    let tokens = Tokenizer::tokenize(expr);
    Tokenizer::parse(tokens)
}
