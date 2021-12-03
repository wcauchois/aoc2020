mod lex;
mod utils;
mod message_reader;

use nom::branch::alt;
use nom::combinator::{map, verify};
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::{combinator::map_res, IResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use crate::lex::*;
use crate::utils::*;
use crate::message_reader::MessageReader;

const INPUT_FILENAME: &str = "testinput.txt";

#[derive(Debug)]
struct RuleClause {
    seq: Vec<i32>,
}

#[derive(Debug)]
enum RuleBody {
    Literal { lit: char },
    Composite { clauses: Vec<RuleClause> },
}

trait RuleEvalContext {
    fn get_rule(&self, idx: i32) -> &Rule;
}

#[derive(Debug)]
struct Rule {
    idx: i32,
    body: RuleBody,
}

impl Rule {
    fn matches<Ctx: RuleEvalContext>(&self, reader: &MessageReader, ctx: &Ctx) -> bool {
        println!("matching: [{}] {:?}; current_char {:?} & pos={}", self.idx, self.body, reader.peek(), reader.pos.get());
        match &self.body {
            RuleBody::Literal{ lit } => match reader.next() {
                Some(c) if *lit == c => true,
                _ => false,
            },
            RuleBody::Composite{ clauses } => {
                // Composite rule is true if any clauses match (in sequence)
                // ??? IDK
                for clause in clauses {
                    let _handle = reader.save();
                    if (&clause.seq).into_iter().all(|item_idx| {
                        let item = ctx.get_rule(*item_idx);
                        item.matches(reader, ctx)
                    }) {
                        println!(">> match {}", self.idx);
                        // shouldn't backtrack
                        return true;
                    }
                }
                println!("|| no match {}", self.idx);
                false
            },
        }
    }

    fn matches_fully<Ctx: RuleEvalContext>(&self, input: &str, ctx: &Ctx) -> bool {
        let reader = MessageReader::new(input);
        let did_match = self.matches(&reader, ctx);
        did_match && reader.peek().is_none()
    }
}

#[derive(Debug)]
struct PuzzleInput {
    rule_map: HashMap<i32, Rule>,
    messages: Vec<String>,
}

impl RuleEvalContext for PuzzleInput {
    fn get_rule(&self, idx: i32) -> &Rule {
        self.rule_map.get(&idx).unwrap()
    }
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

fn parse_composite_rule(input: &[Token]) -> IResult<&[Token], RuleBody> {
    let (input, raw_clauses) =
        separated_list1(take1_verify(|tok| *tok == Token::Pipe), many1(parse_number))(input)?;
    let clauses = raw_clauses
        .into_iter()
        .map(|seq| RuleClause { seq })
        .collect::<Vec<_>>();
    Ok((input, RuleBody::Composite { clauses }))
}

fn parse_literal_rule(input: &[Token]) -> IResult<&[Token], RuleBody> {
    fn quote(input: &[Token]) -> IResult<&[Token], ()> {
        map(take1_verify(|tok| matches!(tok, Token::Quote)), |_| ())(input)
    }
    let (input, lit) = delimited(quote, parse_char, quote)(input)?;
    Ok((input, RuleBody::Literal { lit }))
}

fn parse_rule(input: &[Token]) -> IResult<&[Token], Rule> {
    let (input, idx) = parse_number(input)?;
    let (input, _) = verify(take1, |tok| matches!(tok, Token::Colon))(input)?;
    let (input, body) = alt((parse_composite_rule, parse_literal_rule))(input)?;
    Ok((input, Rule { idx, body }))
}

fn read_puzzle_input() -> Result<PuzzleInput, Box<dyn std::error::Error>> {
    let file = File::open(INPUT_FILENAME)?;
    let lines = io::BufReader::new(file).lines();

    let mut rule_map: HashMap<i32, Rule> = HashMap::new();
    let mut messages: Vec<String> = Vec::new();

    enum ParseState {
        Rules,
        Messages,
    }

    let mut parse_state = ParseState::Rules;

    for line in lines {
        let line = line?;
        match parse_state {
            ParseState::Rules => {
                if line == "" {
                    parse_state = ParseState::Messages;
                    continue;
                }
                let (_, rule) = parse_rule(&lex(&line)).map_err(|_e| {
                    io::Error::new(io::ErrorKind::Other, "Failed to parse ticket rule")
                })?;
                rule_map.insert(rule.idx, rule);
            }
            ParseState::Messages => {
                messages.push(line);
            }
        }
    }

    Ok(PuzzleInput { rule_map, messages })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = read_puzzle_input()?;
    // println!("Parsed puzzle input: {:?}", puzzle_input);

    let rule0 = puzzle_input.get_rule(0);
    println!("{:?}", rule0.matches_fully("ababbb", &puzzle_input));
    // let mut num_matches = 0;
    // for message in &puzzle_input.messages {
    //     if rule0.matches_fully(message, &puzzle_input) {
    //         num_matches += 1;
    //     }
    // }
    // println!("Number of matches: {}", num_matches);

    Ok(())
}
