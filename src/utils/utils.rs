use std::{env, fs};
use anyhow::Result;

pub fn get_input(name: &str) -> Result<String> {
    let cwd = env::current_dir()?;
    let input_path = cwd.join("input/").join(name);

    Ok(fs::read_to_string(input_path)?)
}

pub fn get_lines(name: &str) -> Result<Vec<String>> {
    let input = get_input(name)?;
    let lines = input.lines();
    Ok(lines.map(|s| s.to_string()).collect())
}
