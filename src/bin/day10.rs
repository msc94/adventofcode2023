use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Write,
};

use anyhow::Result;
use utils::get_lines;

type NodeId = (i64, i64);

fn dfs(
    graph: &HashMap<NodeId, Vec<NodeId>>,
    visited: &mut HashSet<NodeId>,
    node: &NodeId,
    end: &NodeId,
) -> Option<Vec<NodeId>> {
    if node == end && !visited.is_empty() {
        return Some(vec![*node]);
    }

    if visited.contains(node) {
        return None;
    }

    visited.insert(*node);

    if let Some(neighbor_list) = graph.get(node) {
        for neighbor in neighbor_list {
            let nodes = dfs(graph, visited, neighbor, end);
            if let Some(mut nodes) = nodes {
                nodes.push(*node);
                return Some(nodes);
            }
        }
    }

    return None;
}

fn connect_nodes(graph: &mut HashMap<NodeId, Vec<NodeId>>, a: &NodeId, b: &NodeId) {
    graph.entry(*a).or_insert(vec![]).push(*b);
    graph.entry(*b).or_insert(vec![]).push(*a);
}

fn main() -> Result<()> {
    let input = get_lines("day10.txt")?;
    let input: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    dbg!(&input);

    let mut graph = HashMap::new();
    let mut start = None;

    input.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, char)| {
            let x = x as i64;
            let y = y as i64;
            match char {
                '.' => {}
                '|' => {
                    connect_nodes(&mut graph, &(x, y), &(x, y - 1));
                    connect_nodes(&mut graph, &(x, y), &(x, y + 1));
                }
                '-' => {
                    connect_nodes(&mut graph, &(x, y), &(x + 1, y));
                    connect_nodes(&mut graph, &(x, y), &(x - 1, y));
                }
                'L' => {
                    connect_nodes(&mut graph, &(x, y), &(x, y - 1));
                    connect_nodes(&mut graph, &(x, y), &(x + 1, y));
                }
                'J' => {
                    connect_nodes(&mut graph, &(x, y), &(x, y - 1));
                    connect_nodes(&mut graph, &(x, y), &(x - 1, y));
                }
                '7' => {
                    connect_nodes(&mut graph, &(x, y), &(x, y + 1));
                    connect_nodes(&mut graph, &(x, y), &(x - 1, y));
                }
                'F' => {
                    connect_nodes(&mut graph, &(x, y), &(x, y + 1));
                    connect_nodes(&mut graph, &(x, y), &(x + 1, y));
                }
                'S' => {
                    start = Some((x, y));
                }
                default => {
                    panic!("Unexpected character {}", default);
                }
            }
        })
    });

    let start = start.expect("a start");
    let mut visited = HashSet::new();
    let path = dfs(&graph, &mut visited, &start, &start);

    if let Some(path) = path {
        for node in &path {
            let char = input[node.1 as usize][node.0 as usize];
            print!(" -> {:?} {}", node, char);
        }
        println!();
        println!("Path length {}", path.len());
    }

    Ok(())
}
