use std::collections::{hash_map::RandomState, HashMap, HashSet};

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

    let mut counts = vec![1; cards.len()];

    for (i, g) in cards.iter().enumerate() {
        let winning = g.count_winning();
        println!("Card {} has {} copies and {} wins", i, counts[i], winning);
        for j in i + 1..i + 1 + winning {
            if j < counts.len() {
                counts[j] += counts[i];
            }
        }
    }

    dbg!(&counts);

    let sum: i32 = counts.iter().sum();
    println!("sum {}", sum);

    Ok(())
}
