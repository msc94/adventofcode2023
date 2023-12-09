use anyhow::{anyhow, Result};
use utils::get_lines;

fn differences(vec: &Vec<i64>) -> Vec<i64> {
    vec.iter()
        .zip(vec.iter().skip(1))
        .map(|(current, following)| following - current)
        .collect()
}

#[derive(Debug)]
struct History {
    values: Vec<Vec<i64>>,
}

impl History {
    fn new(values: Vec<i64>) -> Self {
        History {
            values: vec![values],
        }
    }

    fn fill_differences(&mut self) {
        loop {
            let last = self.values.last().expect("at least one item in values");

            if last.is_empty() || last.iter().all(|x| *x == 0) {
                break;
            }

            self.values.push(differences(last));
        }
    }

    fn extrapolate(&mut self) -> i64 {
        println!();
        let mut current_diff = 0;
        for values in self.values.iter_mut().rev() {
            println!("Extrapolation {:?} with {}", &values, current_diff);
            let new_value = values.last().expect("at least one element") + current_diff;
            values.push(new_value);
            current_diff = new_value;
            println!(" -> {:?}", values);
        }
        return current_diff;
    }
}

fn main() -> Result<()> {
    let input = get_lines("day9.txt")?;

    let mut histories: Vec<History> = input
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<i64>().expect("number can be parsed"))
                .collect()
        })
        .map(|values| History::new(values))
        .collect();

    let mut sum = 0;
    for h in &mut histories {
        h.fill_differences();
        sum += h.extrapolate();
    }

    dbg!(&histories);
    println!("{}", sum);

    Ok(())
}
