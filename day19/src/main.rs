mod lex;
mod utils;

use nom::branch::alt;
use nom::combinator::{map, verify};
use nom::error::ParseError;
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::{combinator::map_res, IResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use crate::lex::*;
use crate::utils::*;

const INPUT_FILENAME: &str = "testinput.txt";

#[derive(Debug)]
struct RuleClause {
    seq: Vec<i32>,
}

#[derive(Debug)]
enum Rule {
    Literal { lit: char },
    Composite { clauses: Vec<RuleClause> },
}

#[derive(Debug)]
struct PuzzleInput {
    rule_map: HashMap<i32, Rule>,
}

fn parse_number(input: &[Token]) -> IResult<&[Token], i32> {
    map_res(take1, |tok: Token| match tok {
        Token::Number(n) => Ok(n),
        _ => Err(nom::Err::Error((input, nom::error::ErrorKind::Verify))),
    })(input)
}

fn parse_char(input: &[Token]) -> IResult<&[Token], char> {
    map_res(take1, |tok: Token| match tok {
        Token::Char(c) => Ok(c),
        _ => Err(nom::Err::Error((input, nom::error::ErrorKind::Verify))),
    })(input)
}

fn parse_composite_rule(input: &[Token]) -> IResult<&[Token], Rule> {
    let (input, raw_clauses) =
        separated_list1(take1_verify(|tok| *tok == Token::Pipe), many1(parse_number))(input)?;
    let clauses = raw_clauses
        .into_iter()
        .map(|seq| RuleClause { seq })
        .collect::<Vec<_>>();
    Ok((input, Rule::Composite { clauses }))
}

fn parse_literal_rule(input: &[Token]) -> IResult<&[Token], Rule> {
    fn quote(input: &[Token]) -> IResult<&[Token], ()> {
        map(take1_verify(|tok| matches!(tok, Token::Quote)), |_| ())(input)
    }
    let (input, lit) = delimited(quote, parse_char, quote)(input)?;
    Ok((input, Rule::Literal { lit }))
}

fn parse_rule(input: &[Token]) -> IResult<&[Token], (i32, Rule)> {
    let (input, rule_idx) = parse_number(input)?;
    let (input, _) = verify(take1, |tok| matches!(tok, Token::Colon))(input)?;
    let (input, rule) = alt((parse_composite_rule, parse_literal_rule))(input)?;

    Ok((input, (rule_idx, rule)))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(INPUT_FILENAME)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line?;
    }

    parse_rule(&[
        Token::Number(0),
        Token::Colon,
        Token::Number(1),
        Token::Number(3),
        Token::Pipe,
        Token::Number(2),
        Token::Number(4),
    ])?;

    Ok(())
}
