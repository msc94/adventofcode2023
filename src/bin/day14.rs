use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap, HashSet,
    },
    io::stdin,
};

use anyhow::Result;
use indicatif::ProgressIterator;
use utils::get_lines;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Game {
    rows: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Game {
    fn get_tile(&self, row: usize, column: usize) -> &char {
        &self.rows[row][column]
    }

    fn set_tile(&mut self, row: usize, column: usize, tile: char) {
        self.rows[row][column] = tile;
    }

    fn move_tile(&mut self, row: usize, column: usize, direction: Direction) -> bool {
        let mut moved = false;

        let tile = self.get_tile(row, column);
        assert!(*tile == 'O');

        let mut new_row = row;
        let mut new_column = column;

        match direction {
            Direction::North => {
                while new_row > 0 && *self.get_tile(new_row - 1, new_column) == '.' {
                    new_row -= 1;
                    moved = true;
                }
            }
            Direction::East => {
                while new_column < self.width - 1 && *self.get_tile(new_row, new_column + 1) == '.'
                {
                    new_column += 1;
                    moved = true;
                }
            }
            Direction::South => {
                while new_row < self.height - 1 && *self.get_tile(new_row + 1, new_column) == '.' {
                    new_row += 1;
                    moved = true;
                }
            }
            Direction::West => {
                while new_column > 0 && *self.get_tile(new_row, new_column - 1) == '.' {
                    new_column -= 1;
                    moved = true;
                }
            }
        }

        self.set_tile(row, column, '.');
        self.set_tile(new_row, new_column, 'O');

        moved
    }

    fn step(&mut self, direction: Direction) -> bool {
        let mut moved = false;

        match direction {
            Direction::North => {
                for current_row in 0..self.height {
                    for current_column in 0..self.width {
                        let tile = self.get_tile(current_row, current_column);
                        if *tile == 'O' {
                            if self.move_tile(current_row, current_column, direction) {
                                moved = true;
                            }
                        }
                    }
                }
            }
            Direction::East => {
                for current_row in 0..self.height {
                    for current_column in (0..self.width).rev() {
                        let tile = self.get_tile(current_row, current_column);
                        if *tile == 'O' {
                            if self.move_tile(current_row, current_column, direction) {
                                moved = true;
                            }
                        }
                    }
                }
            }
            Direction::South => {
                for current_row in (0..self.height).rev() {
                    for current_column in 0..self.width {
                        let tile = self.get_tile(current_row, current_column);
                        if *tile == 'O' {
                            if self.move_tile(current_row, current_column, direction) {
                                moved = true;
                            }
                        }
                    }
                }
            }
            Direction::West => {
                for current_row in 0..self.height {
                    for current_column in 0..self.width {
                        let tile = self.get_tile(current_row, current_column);
                        if *tile == 'O' {
                            if self.move_tile(current_row, current_column, direction) {
                                moved = true;
                            }
                        }
                    }
                }
            }
        }

        // println!("After moving in direction {:?}:", direction);
        // self.print();
        // println!();

        moved
    }

    fn print(&self) {
        let num_stones = self
            .rows
            .iter()
            .flatten()
            .filter(|c| **c == '#' || **c == 'O')
            .count();

        // println!("Number of stones: {}", num_stones);

        for current_row in 0..self.height {
            for current_column in 0..self.width {
                print!("{}", self.get_tile(current_row, current_column));
            }
            println!();
        }
    }

    fn cycle(&mut self) {
        self.step(Direction::North);
        self.step(Direction::West);
        self.step(Direction::South);
        self.step(Direction::East);
    }

    fn count_weight(&self) -> usize {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, tile)| (i, j, tile)))
            .filter_map(|(row, _, tile)| {
                if *tile != 'O' {
                    None
                } else {
                    Some(self.height - row)
                }
            })
            .sum()
    }
}

fn parse_input(input: &Vec<String>) -> Result<Game> {
    let mut result = Game {
        rows: vec![],
        width: 0,
        height: 0,
    };

    for row in input {
        result.rows.push(vec![]);
        let current_row = result.rows.last_mut().expect("last element");

        for column in row.chars() {
            current_row.push(column);
        }
    }

    result.height = result.rows.len();
    result.width = result.rows.first().expect("at least one row").len();

    Ok(result)
}

fn main() -> Result<()> {
    let input = get_lines("day14.txt")?;
    let mut game = parse_input(&input)?;
    let mut seen = HashMap::new();

    for i in (0..1_000_000_000).progress() {
        game.cycle();

        let entry = seen.entry(game.clone());
        match entry {
            Occupied(entry) => {
                println!(
                    "Already seen the game from iteration {} at iteration {}",
                    i,
                    entry.get()
                );
            }
            Vacant(entry) => {
                entry.insert(i);
            }
        }

        // println!("After {} cycles", i + 1);
        // game.print();
        // println!();

        // let mut buf = String::new();
        // stdin().read_line(&mut buf).unwrap();
    }

    game.print();
    println!("{}", game.count_weight());

    Ok(())
}
