use std::collections::{hash_map::RandomState, HashSet};

use anyhow::{anyhow, bail, Result};
use nom::{
    bytes::complete::tag,
    character::{self, complete::space1},
    complete,
    multi::separated_list1,
    number,
    sequence::{terminated, tuple},
    IResult,
};
use utils::get_lines;

#[derive(Debug)]
struct Game {
    numbers: Vec<i32>,
    winning: Vec<i32>,
}

impl Game {
    fn count_winning(&self) -> usize {
        let set: HashSet<i32> = HashSet::from_iter(self.winning.iter().map(|x| *x));
        let winning = self.numbers.iter().filter(|x| set.contains(x)).count();
        winning
    }
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    let (input, _) = tuple((
        tag("Card"),
        space1,
        character::complete::i32,
        tag(":"),
        space1,
    ))(input)?;

    let (input, numbers) = separated_list1(space1, character::complete::i32)(input)?;

    let (input, _) = tuple((space1, tag("|"), space1))(input)?;

    let (input, winning) = separated_list1(space1, character::complete::i32)(input)?;

    Ok((input, Game { numbers, winning }))
}

fn main() -> Result<()> {
    let lines = get_lines("day4.txt")?;

    let cards = lines
        .iter()
        .map(|l| {
            parse_line(l)
                .map_err(|e| anyhow!("Failed parsing line: {e}"))
                .and_then(|(_, g)| Ok(g))
        })
        .collect::<Result<Vec<Game>>>()?;
    dbg!(&cards);

    let winning: Vec<_> = cards.iter().map(Game::count_winning).collect();
    dbg!(&winning);

    let powers: Vec<_> = winning
        .iter()
        .map(|x| if *x == 0 { 0 } else { 2_i32.pow(*x as u32 - 1) })
        .collect();
    dbg!(&powers);

    println!("{}", powers.iter().sum::<i32>());

    Ok(())
}
