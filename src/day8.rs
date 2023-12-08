use std::collections::HashMap;
use std::fs;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
  Left,
  Right,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
  let mut lines = input.lines();
  let steps_unparsed = lines.next().unwrap();
  let steps = steps_unparsed
    .chars()
    .map(|step| match step {
      'L' => Direction::Left,
      'R' => Direction::Right,
      c => panic!("Invalid step: {}", c),
    })
    .collect();

  let mut map = HashMap::new();

  for line in lines.skip(1) {
    let (name, children) = line.split_once(" = ").unwrap();
    let (left, right) = children[1..children.len() - 1].split_once(", ").unwrap();

    map.insert(name.to_string(), (left.to_string(), right.to_string()));
  }

  (steps, map)
}

pub fn solve_a(input_file_path: &str) -> u64 {
  let input_unparsed = fs::read_to_string(input_file_path).unwrap();
  let (directions, map) = parse_input(&input_unparsed);

  let mut steps = 0;
  let mut current_node = "AAA";

  while current_node != "ZZZ" {
    let (left, right) = map.get(current_node).unwrap();
    let next_node = if directions[steps % directions.len()] == Direction::Left {
      left
    } else {
      right
    };

    current_node = next_node;
    steps += 1;
  }

  steps.try_into().unwrap()
}

fn lcm(first: usize, second: usize) -> usize {
  first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
  let mut max = first;
  let mut min = second;
  if min > max {
    let val = max;
    max = min;
    min = val;
  }

  loop {
    let res = max % min;
    if res == 0 {
      return min;
    }

    max = min;
    min = res;
  }
}

pub fn solve_b(input_file_path: &str) -> u64 {
  let input_unparsed = fs::read_to_string(input_file_path).unwrap();
  let (directions, map) = parse_input(&input_unparsed);

  let starting_nodes: Vec<String> = map
    .iter()
    .filter(|(name, _)| name.ends_with("A"))
    .map(|(name, _)| name.to_string())
    .collect();

  let current_nodes = starting_nodes.clone();

  let steps = current_nodes
    .par_iter()
    .map(|current_node| {
      let mut steps = 0;
      let mut current_node = current_node.as_str();
      let map = map.clone();
      let directions = directions.clone();

      while !current_node.ends_with("Z") {
        let (left, right) = map.get(current_node).unwrap();
        let next_node = if directions[steps % directions.len()] == Direction::Left {
          left
        } else {
          right
        };

        current_node = next_node;
        steps += 1;
      }

      steps as u32
    })
    .collect::<Vec<u32>>();

  let res = steps
    .iter()
    .fold(1, |acc, &step| lcm(acc as usize, step as usize));

  res.try_into().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_a() {
    assert_eq!(solve_a(".\\src\\test_input\\day8a.txt"), 6);
  }

  #[test]
  fn test_solve_b() {
    assert_eq!(solve_b(".\\src\\test_input\\day8b.txt"), 6);
  }
}
