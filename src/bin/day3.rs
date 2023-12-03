use itertools::iproduct;
use std::collections::{HashMap, HashSet};

use anyhow::Result;
use utils::get_lines;

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() -> Result<()> {
    let lines = get_lines("day3.txt").unwrap();
    let chars = parse(&lines);
    let (w, h) = (chars[0].len() as i64, chars.len() as i64);

    let get = |x: i64, y: i64| -> Option<char> {
        if x < 0 || y < 0 || x >= w || y >= h {
            None
        } else {
            Some(chars[y as usize][x as usize])
        }
    };

    let mut current_number = None;
    let mut current_adjacent = HashSet::new();
    let mut adjacent = HashMap::new();

    for y in 0..h {
        for x in 0..w {
            let current = get(x, y).expect("A char");

            if let Some(d) = current.to_digit(10) {
                iproduct!(-1..=1, -1..=1)
                    .filter_map(|(dx, dy)| get(x + dx, y + dy).map(|c| (c, x + dx, y + dy)))
                    .filter(|(c, _, _)| *c == '*')
                    .for_each(|(_, x, y)| {
                        current_adjacent.insert((x, y));
                    });

                let instance = current_number.get_or_insert(0);
                *instance *= 10;
                *instance += d as i64;
            } else if let Some(n) = current_number {
                for a in current_adjacent {
                    adjacent.entry(a).or_insert(vec![]).push(n);
                }
                current_adjacent = HashSet::new();
                current_number = None;
            }
        }
    }

    let sum: i64 = adjacent
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product::<i64>())
        .sum();

    println!("{sum}");

    Ok(())
}
