use anyhow::{anyhow, bail, Result};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};
use utils::get_lines;

#[derive(Debug)]
struct SeedData {
    seeds: Vec<u64>,
}

#[derive(Debug)]
struct MapData {
    map_values: Vec<Vec<u64>>,
}

fn parse_seeds(input: &str) -> IResult<&str, SeedData> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = space1(input)?;
    let (input, seeds) = separated_list1(space1, nom::character::complete::u64)(input)?;
    Ok((input, SeedData { seeds }))
}

fn parse_maps(input: &str) -> IResult<&str, MapData> {
    let (input, _) = take_until("map:")(input)?;
    let (input, _) = tag("map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, map_values) = separated_list1(
        line_ending,
        separated_list1(space1, nom::character::complete::u64),
    )(input)?;
    Ok((input, MapData { map_values }))
}

fn main() -> Result<()> {
    let lines = get_lines("day5.txt")?;
    let input = lines.join("\n");
    let mut input = input.as_str();

    let seeds;
    (input, seeds) = parse_seeds(&input).map_err(|e| anyhow!("{}", e))?;

    let mut maps = vec![];

    for _ in 0..7 {
        let map;
        (input, map) = parse_maps(&input).map_err(|e| anyhow!("{}", e))?;
        maps.push(map);
    }

    dbg!(&seeds, &maps);

    if input != "" {
        bail!("Input left")
    }

    let mut result = vec![];

    for &seed in &seeds.seeds {
        println!("\nProcessing {}", &seed);

        let mut current = seed;

        for map in &maps {
            let mut mapped = None;

            for range in &map.map_values {
                if mapped.is_some() {
                    break;
                }

                assert!(range.len() == 3);

                let destination = range[0];
                let source = range[1];
                let size = range[2];

                if current >= source && current < source + size {
                    let delta = current - source;
                    mapped = Some(destination + delta);
                }
            }

            if let Some(m) = mapped {
                println!("  {} -> {}", current, m);
                current = m;
            } else {
                println!("  {} -> {}", current, current);
            }
        }

        println!("  -> {}", current);
        result.push(current);
    }

    println!("{}", result.iter().min().unwrap());

    Ok(())
}
