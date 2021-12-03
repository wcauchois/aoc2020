use std::iter::{FromIterator, Peekable};

// https://adriann.github.io/rust_parser.html

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Number(i32),
    Char(char),
    Colon,
    Pipe,
    Quote,
}

fn get_number<T: Iterator<Item = char>>(first_c: char, it: &mut Peekable<T>) -> i32 {
    let mut buf = String::from_iter([first_c]);
    loop {
        match it.peek() {
            Some(&c) if c.is_digit(10) => {
                it.next();
                buf.push(c);
            }
            _ => return buf.parse::<i32>().unwrap(),
        }
    }
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                result.push(Token::Number(get_number(c, &mut it)));
            }
            'a'..='z' | 'A'..='Z' => {
                it.next();
                result.push(Token::Char(c));
            }
            '|' => {
                it.next();
                result.push(Token::Pipe);
            }
            '"' => {
                it.next();
                result.push(Token::Quote);
            }
            ':' => {
                it.next();
                result.push(Token::Colon);
            }
            ' ' => {
                it.next();
            }
            _ => panic!("Unexpected character {}", c),
        }
    }
    result
}
