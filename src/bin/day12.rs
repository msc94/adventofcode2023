use core::panic;

use anyhow::Result;
use nom::{
    bytes::complete::{self, tag, take_until, take_while1},
    character::complete::space1,
    multi::separated_list1,
    IResult, Parser,
};
use utils::get_lines;

#[derive(Debug)]
enum SpringType {
    UNKNOWN,
    WORKING,
    BROKEN,
}

#[derive(Debug)]
struct Problem {
    positions: Vec<SpringType>,
    wanted_groups: Vec<usize>,
    current_groups: Vec<(usize, usize)>,
}

impl Problem {
    fn calculate_current_groups(&mut self) {
        let mut i = 0;
    }
}

fn process_problem(problem: &Problem) -> Result<i32> {
    println!("Processing {:?}", problem);

    let result = 1;
    println!(" -> {}", result);
    Ok(result)
}

fn parse_line(line: &str) -> IResult<&str, Problem> {
    let (input, str) = take_while1(|c: char| !c.is_whitespace())(line)?;
    let (input, _) = space1(input)?;
    let (input, damaged_groups) = separated_list1(tag(","), nom::character::complete::u64)(input)?;

    let positions = str
        .chars()
        .map(|c| match c {
            '.' => SpringType::WORKING,
            '#' => SpringType::BROKEN,
            '?' => SpringType::UNKNOWN,
            default => panic!("Unexpected char {}", default),
        })
        .collect();

    let damaged_groups = damaged_groups.iter().map(|x| *x as usize).collect();

    Ok((
        input,
        Problem {
            positions,
            wanted_groups: damaged_groups,
            current_groups: vec![],
        },
    ))
}

fn main() -> Result<()> {
    let input = get_lines("day12_example.txt")?;
    dbg!(&input);

    let sum: i32 = input
        .iter()
        .map(|l| parse_line(&l).expect("line can be parsed").1)
        .map(|p| process_problem(&p).expect("line can be processed"))
        .sum();

    println!("{}", sum);

    Ok(())
}
