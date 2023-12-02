use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, space0, u32},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use utils::get_lines;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Color {
    Blue,
    Red,
    Green,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct CountColorPair {
    color: Color,
    count: u32,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    game_number: u32,
    sequences: Vec<Vec<CountColorPair>>,
}

// Helper parser to convert a recognized color string to a Color enum
fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, color) = alpha1(input)?;
    match color.to_lowercase().as_str() {
        "blue" => Ok((input, Color::Blue)),
        "red" => Ok((input, Color::Red)),
        "green" => Ok((input, Color::Green)),
        _ => unreachable!(),
    }
}

fn parse_count_color_pair(input: &str) -> IResult<&str, CountColorPair> {
    map(
        separated_pair(u32, space0, parse_color),
        |(count, color)| CountColorPair { count, color },
    )(input)
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<CountColorPair>> {
    separated_list0(terminated(tag(","), space0), parse_count_color_pair)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    map(
        tuple((
            preceded(tag("Game"), preceded(space0, u32)),
            preceded(
                terminated(tag(":"), multispace1),
                separated_list0(terminated(tag(";"), space0), parse_sequence),
            ),
        )),
        |(game_number, sequences)| Game {
            game_number,
            sequences,
        },
    )(input)
}

fn main() -> Result<()> {
    let lines = get_lines("day2.txt").unwrap();
    dbg!(&lines);

    let mut result = 0;

    for line in lines {
        let parsed = parse_game(&line).map_err(|_| anyhow!("Parsing of {} failed", line))?;

        if !parsed.0.is_empty() {
            anyhow::bail!("Input left after parsing");
        }
        let game = parsed.1;

        let maximums = game
            .sequences
            .iter()
            .flatten()
            .fold(HashMap::new(), |mut acc, pair| {
                let entry = acc.entry(&pair.color).or_insert(0);
                if *entry < pair.count {
                    *entry = pair.count;
                }
                acc
            });

        result += maximums.get(&Color::Red).unwrap_or(&0)
            * maximums.get(&Color::Green).unwrap_or(&0)
            * maximums.get(&Color::Blue).unwrap_or(&0)
    }

    println!("{result}");

    Ok(())
}
