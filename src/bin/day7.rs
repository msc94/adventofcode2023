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
    bid: u64,
}

impl Hand {
    fn get_as_tuple(&self) -> (i32, i32, i32, i32, i32) {
        self.hand
            .chars()
            .map(|c| {
                if let Some(digit) = c.to_digit(10) {
                    return digit as i32;
                } else {
                    match c {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Unexpected char {}", c),
                    }
                }
            })
            .next_tuple()
            .unwrap()
    }

    fn get_type(&self) -> Type {
        let mut counts = HashMap::new();

        let (jokers, non_jokers): (Vec<char>, Vec<char>) =
            self.hand.to_owned().chars().partition(|c| *c == 'J');

        for c in non_jokers {
            *counts.entry(c).or_insert(0) += 1;
        }

        if counts.is_empty() {
            // Only jokers
            return Type::FiveOfAKind;
        }

        let max = counts
            .iter()
            .max_by(|a, b| a.cmp(b))
            .expect("a maximum value element")
            .0
            .to_owned();

        *counts.get_mut(&max).unwrap() += jokers.len();

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
    let (input, bid) = nom::character::complete::u64(input)?;
    Ok((
        input,
        Hand {
            hand: hand.to_owned(),
            bid,
        },
    ))
}

fn main() -> Result<()> {
    let lines = get_lines("day7.txt")?;

    let mut hands = vec![];
    for line in lines {
        hands.push(parse(&line).map_err(|e| anyhow!("{}", e))?.1);
    }

    hands.sort_by(|a, b| {
        if a.get_type() < b.get_type() {
            return Ordering::Less;
        }

        if a.get_type() > b.get_type() {
            return Ordering::Greater;
        }

        if a.get_as_tuple() < b.get_as_tuple() {
            return Ordering::Less;
        }

        if a.get_as_tuple() > b.get_as_tuple() {
            return Ordering::Greater;
        }

        return Ordering::Equal;
    });

    dbg!(&hands);

    let result: usize = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum();

    println!("{}", result);

    Ok(())
}
