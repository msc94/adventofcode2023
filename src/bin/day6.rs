// Time:      7  15   30
// Distance:  9  40  200

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::space1,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};
use utils::get_lines;

fn parse_line(line: &str) -> IResult<&str, Vec<i32>> {
    let (input, _) = tuple((take_until(":"), tag(":"), space1))(line)?;
    let (input, vec) = separated_list1(space1, nom::character::complete::i32)(input)?;
    Ok((input, vec))
}

fn main() -> Result<()> {
    let lines = get_lines("day6_example.txt")?;

    let times = parse_line(&lines[0])
        .map_err(|e| anyhow!("Parsing failed: {}", e))?
        .1;
    let distances = parse_line(&lines[1])
        .map_err(|e| anyhow!("Parsing failed: {}", e))?
        .1;

    dbg!(&times, &distances);

    let product: usize = (times.iter().zip(&distances).map(|(time, distance)| {
        (0..=*time)
            .map(|hold| (time - hold) * hold)
            .filter(|traveled| traveled > distance)
            .count()
    }))
    .product();

    println!("{}", product);

    Ok(())
}

