use std::collections::HashMap;

use anyhow::Result;
use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::is_alphabetic,
    sequence::terminated,
    IResult,
};
use utils::get_lines;

#[derive(Debug)]
struct MapEntry {
    id: String,
    left_direction: String,
    right_direction: String,
}

fn parse_line(line: &str) -> IResult<&str, MapEntry> {
    let input = line;
    let (input, id) = take_until(" = (")(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left_direction) = take_until(",")(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right_direction) = take_until(")")(input)?;

    return Ok((
        input,
        MapEntry {
            id: id.to_owned(),
            left_direction: left_direction.to_owned(),
            right_direction: right_direction.to_owned(),
        },
    ));
}

fn main() -> Result<()> {
    let input = get_lines("day8.txt")?;
    let mut input_iter = input.iter();

    let directions: Vec<char> = input_iter
        .next()
        .expect("directions line")
        .chars()
        .collect();

    // Empty line
    let _ = input_iter.next();

    let entries: HashMap<String, MapEntry> = input_iter
        .map(|l| parse_line(l).expect("correctly parsed line").1)
        .map(|e| (e.id.to_owned(), e))
        .collect();

    dbg!(&directions, &entries);

    let mut current = "AAA";
    let mut iteration = 0usize;

    while current != "ZZZ" {
        let index = iteration % directions.len();
        let direction = directions[index];

        println!(
            "Index {}, direction {}, current {}",
            index, direction, current
        );

        let map_entry = entries.get(current).unwrap();

        match direction {
            'L' => {
                current = &map_entry.left_direction;
            }
            'R' => {
                current = &map_entry.right_direction;
            }
            default => {
                panic!("Unexpected direction {}", default);
            }
        }
        iteration += 1;
    }

    println!("{}", iteration);

    Ok(())
}
