use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use anyhow::Result;
use utils::get_lines;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Graph {
    graph: Vec<Vec<u32>>,
    width: i64,
    height: i64,
}

impl Graph {
    fn get_weight(&self, x: i64, y: i64) -> Option<u32> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            Some(self.graph[y as usize][x as usize])
        }
    }

    fn from_input(input: &Vec<String>) -> Self {
        let mut result = Graph {
            graph: vec![],
            height: input.len() as i64,
            width: input[0].len() as i64,
        };

        for row in input {
            result.graph.push(vec![]);
            let current_row = result.graph.last_mut().expect("a last row");
            for char in row.chars() {
                current_row.push(char.to_digit(10).expect("a valid digit"));
            }
        }

        result
    }
}

fn main() -> Result<()> {
    let input = get_lines("day17_example.txt")?;
    dbg!(&input);
    let graph = Graph::from_input(&input);
    dbg!(&graph);

    let start_node = (0, 0);
    let end_node = (graph.width - 1, graph.height - 1);

    let mut distances = HashMap::new();
    let mut priority_queue = BinaryHeap::new();
    let mut previous = HashMap::new();

    // Initialize

    for y in 0..graph.height {
        for x in 0..graph.width {
            distances.insert((x, y), u32::MAX);
        }
    }

    distances.insert(start_node, 0);
    priority_queue.push((Reverse(0), (0, 0), (Direction::Right, 0)));
    priority_queue.push((Reverse(0), (0, 0), (Direction::Down, 0)));

    while let Some((Reverse(distance), (x, y), (direction, steps))) = priority_queue.pop() {
        if distance > *distances.get(&(x, y)).expect("all distances filled") {
            continue;
        }

        let weight = graph.get_weight(x, y).expect("a weight");

        println!(
            "Visiting node ({}, {}) '{}' with distance {} going {:?} for {} steps",
            x, y, weight, distance, direction, steps
        );

        if end_node == (x, y) {
            println!("End reached with {}!", distance);
            break;
        }

        let mut update = |(cur_x, cur_y), (direction, steps), current_distance| {
            if let Some(neighbor_weight) = graph.get_weight(cur_x, cur_y) {
                let current_neighbor_distance = *distances
                    .get(&(cur_x, cur_y))
                    .expect("all distances filled");

                let new_distance = current_distance + neighbor_weight;

                if new_distance < current_neighbor_distance {
                    distances.insert((cur_x, cur_y), new_distance);
                    previous.insert((cur_x, cur_y), (x, y));
                    priority_queue.push((
                        Reverse(new_distance),
                        (cur_x, cur_y),
                        (direction, steps),
                    ));
                }
            }
        };

        match direction {
            Direction::Up | Direction::Down => {
                update((x - 1, y), (Direction::Left, 1), distance);
                update((x + 1, y), (Direction::Right, 1), distance);
            }
            Direction::Right | Direction::Left => {
                update((x, y - 1), (Direction::Up, 1), distance);
                update((x, y + 1), (Direction::Down, 1), distance);
            }
        }

        if steps < 3 {
            match direction {
                Direction::Up => {
                    update((x, y - 1), (direction, steps + 1), distance);
                }
                Direction::Right => {
                    update((x + 1, y), (direction, steps + 1), distance);
                }
                Direction::Down => {
                    update((x, y + 1), (direction, steps + 1), distance);
                }
                Direction::Left => {
                    update((x - 1, y), (direction, steps + 1), distance);
                }
            }
        }

        println!("  -> updated queue {:?}", priority_queue);
    }

    let mut cur = end_node;
    while cur != start_node {
        let reached_from = previous.get(&cur).unwrap();
        let weight = graph.get_weight(cur.0, cur.1).unwrap();
        println!("{:?} '{}'", cur, weight);
        cur = *reached_from;
    }

    Ok(())
}
