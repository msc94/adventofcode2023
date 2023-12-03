use itertools::iproduct;
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use utils::get_lines;

enum State {
    NotNumber,
    Number,
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() -> Result<()> {
    let lines = get_lines("day3.txt").unwrap();
    let chars = parse(&lines);
    let (w, h) = (chars[0].len() as i64, chars.len() as i64);

    let get = |x: i64, y: i64| -> Option<char> {
        if x < 0 || y < 0 || x >= w || y >= h {
            return None;
        } else {
            return Some(chars[y as usize][x as usize]);
        }
    };

    let mut sum: i64 = 0;
    let mut current_number = None;
    let mut adjacent = false;

    for y in 0..h {
        for x in 0..w {
            let current = get(x, y).expect("A char");
            // println!("Looking at {x} {y} -> {current}, sum {sum}, current_number {current_number:?}, adjacent {adjacent}");

            if let Some(d) = current.to_digit(10) {
                let current_adjacent = iproduct!(-1..=1, -1..=1)
                    .filter_map(|(dx, dy)| get(x + dx, y + dy))
                    .any(|c| c != '.' && !c.is_digit(10));

                if current_adjacent {
                    adjacent = true;
                }

                let value = current_number.get_or_insert(0);
                *value *= 10;
                *value += d as i64;
            } else {
                if adjacent {
                    sum += current_number.expect("A number");
                }

                current_number = None;
                adjacent = false;
            }
        }
    }

    println!("sum {sum}");

    Ok(())
}
