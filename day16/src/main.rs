extern crate nom;

use nom::character::complete::{digit1, space0};
use nom::multi::separated_list0;
use nom::InputTakeAtPosition;
use nom::{bytes::complete::tag, combinator::map_res, sequence::tuple, IResult};
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
struct TicketRange {
    from: i32,
    to: i32,
}

impl TicketRange {
    fn contains(&self, value: i32) -> bool {
        value >= self.from && value <= self.to
    }
}

// class: 1-3 or 5-7
#[derive(Debug)]
struct TicketRule {
    field_name: String,
    ranges: Vec<TicketRange>,
}

#[derive(Debug)]
struct TicketValues(Vec<i32>);

#[derive(Debug)]
struct ProgramInput {
    rules: Vec<TicketRule>,
    my_ticket: TicketValues,
    nearby_tickets: Vec<TicketValues>,
}

impl ProgramInput {
    fn all_ranges<'a>(&'a self) -> impl Iterator<Item = &'a TicketRange> {
        (&self.rules).into_iter().flat_map(|r| &r.ranges)
    }

    fn contained_by_any_range(&self, value: i32) -> bool {
        self.all_ranges().any(|r| r.contains(value))
    }
}

fn parse_ticket_range(input: &str) -> IResult<&str, TicketRange> {
    let (input, from) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, to) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;
    Ok((input, TicketRange { from, to }))
}

fn parse_ticket_rule(input: &str) -> IResult<&str, TicketRule> {
    let (input, field_name) = input.split_at_position1_complete(
        |item| !(item.is_alphanumeric() || item == ' '),
        nom::error::ErrorKind::Fail,
    )?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, ranges) =
        separated_list0(tuple((space0, tag("or"), space0)), parse_ticket_range)(input)?;
    Ok((
        input,
        TicketRule {
            field_name: field_name.to_string(),
            ranges,
        },
    ))
}

fn parse_ticket_values(input: &str) -> Result<TicketValues, <i32 as FromStr>::Err> {
    let values = input
        .split(',')
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;
    Ok(TicketValues(values))
}

fn parse_program_input<L>(lines: L) -> Result<ProgramInput, Box<dyn std::error::Error>>
where
    L: Iterator<Item = io::Result<String>>,
{
    enum ParseState {
        Init,
        MyTicket,
        NearbyTickets,
    }

    let mut parse_state = ParseState::Init;

    let mut rules: Vec<TicketRule> = Vec::new();
    let mut my_ticket: Option<TicketValues> = None;
    let mut nearby_tickets: Vec<TicketValues> = Vec::new();

    for line in lines {
        let line = line?;
        if line == "" {
            continue;
        }
        match parse_state {
            ParseState::Init => {
                if line == "your ticket:" {
                    parse_state = ParseState::MyTicket;
                } else {
                    let (_, ticket_rule) = parse_ticket_rule(&line).map_err(|_e| {
                        io::Error::new(io::ErrorKind::Other, "Failed to parse ticket rule")
                    })?;
                    rules.push(ticket_rule);
                }
            }
            ParseState::MyTicket => {
                if line == "nearby tickets:" {
                    parse_state = ParseState::NearbyTickets;
                } else {
                    my_ticket = Some(parse_ticket_values(&line)?);
                }
            }
            ParseState::NearbyTickets => {
                nearby_tickets.push(parse_ticket_values(&line)?);
            }
        }
    }

    Ok(ProgramInput {
        rules,
        my_ticket: my_ticket.expect("No value found for your ticket!"),
        nearby_tickets,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(INPUT_FILENAME)?;
    let lines = io::BufReader::new(file).lines();

    let program_input = parse_program_input(lines)?;
    println!("Parsed program input: {:?}", program_input);

    let mut error_rate = 0;
    for nearby_ticket in &program_input.nearby_tickets {
        for value in &(nearby_ticket.0) {
            if !program_input.contained_by_any_range(*value) {
                error_rate += value;
            }
        }
    }

    println!("Error rate: {}", error_rate);

    Ok(())
}
