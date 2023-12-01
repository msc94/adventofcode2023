use std::collections::BTreeMap;

use anyhow::Result;
use utils::get_lines;

fn main() -> Result<()> {
    let lines = get_lines("day1.txt").unwrap();
    dbg!(&lines);

    let mut sum = 0;
    for l in &lines {
        dbg!(l);
        let numbers = 1..=9;
        let strings = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut matches = BTreeMap::new();
        for (i, s) in numbers.zip(strings) {
            if let Some(index) = l.find(&i.to_string()) {
                matches.insert(index, i);
            }

            if let Some(index) = l.find(&s) {
                matches.insert(index, i);
            }

            if let Some(index) = l.rfind(&i.to_string()) {
                matches.insert(index, i);
            }

            if let Some(index) = l.rfind(&s) {
                matches.insert(index, i);
            }
        }

        dbg!(&matches);

        sum += 10 * matches.first_key_value().unwrap().1 + matches.last_key_value().unwrap().1;
    }

    println!("{}", sum);

    Ok(())
}
