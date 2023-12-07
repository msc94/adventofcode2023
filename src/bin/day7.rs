use std::{cmp::Ordering, collections::HashMap};

use anyhow::{anyhow, Result};
use indicatif::ProgressIterator;
use itertools::{any, Itertools};
use nom::{
    bytes::complete::{tag, take, take_until, take_while1},
    character::complete::space1,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use utils::get_lines;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: i64,
}

impl Hand {
    fn get_type(&self) -> Type {
        let mut counts = HashMap::new();

        for c in self.hand.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        dbg!(&self.hand, &counts);

        let mut has = HashMap::new();
        let mut two_pair = false;

        for (_, count) in counts {
            if count == 5 {
                has.insert(5, true);
            }
            if count == 4 {
                has.insert(4, true);
            }
            if count == 3 {
                has.insert(3, true);
            }
            if count == 2 {
                if has.contains_key(&2) {
                    two_pair = true;
                }
                has.insert(2, true);
            }
        }

        if has.contains_key(&5) {
            return Type::FiveOfAKind;
        }

        if has.contains_key(&4) {
            return Type::FourOfAKind;
        }

        if has.contains_key(&3) && has.contains_key(&2) {
            return Type::FullHouse;
        }

        if has.contains_key(&3) {
            return Type::ThreeOfAKind;
        }

        if two_pair {
            return Type::TwoPair;
        }

        if has.contains_key(&2) {
            return Type::OnePair;
        }

        return Type::HighCard;
    }
}

fn parse(line: &str) -> IResult<&str, Hand> {
    let (input, hand) = terminated(take(5u32), tag(" "))(line)?;
    let (input, bid) = nom::character::complete::i64(input)?;
    Ok((
        input,
        Hand {
            hand: hand.to_owned(),
            bid,
        },
    ))
}

fn main() -> Result<()> {
    let lines = get_lines("day7_example.txt")?;

    let mut hands = vec![];
    for line in lines {
        hands.push(parse(&line).map_err(|e| anyhow!("{}", e))?.1);
    }

    dbg!(&hands);

    for hand in &hands {
        dbg!(hand, hand.get_type());
    }

    hands.sort_by(|a, b| {
        if a.get_type() < b.get_type() {
            return Ordering::Less;
        }

        if a.get_type() > b.get_type() {
            return Ordering::Greater;
        }

    });

    Ok(())
}
