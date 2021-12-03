// https://adriann.github.io/rust_parser.html

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Number(i32),
    Char(char),
    Colon,
    Pipe,
    Quote,
}
