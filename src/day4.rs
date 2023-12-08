use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

fn parse_input(input: &str) -> (u32, Vec<u32>, Vec<u32>) {
  let binding = input;
  let (id, game) = binding.split_once(": ").unwrap();

  let (numbers, guesses) = game.split_once("|").unwrap();

  (
    id.split_whitespace()
      .filter_map(|n| n.parse::<u32>().ok())
      .last()
      .unwrap(),
    numbers
      .split_whitespace()
      .filter_map(|n| n.parse::<u32>().ok())
      .collect::<Vec<u32>>(),
    guesses
      .split_whitespace()
      .filter_map(|g| g.parse::<u32>().ok())
      .collect::<Vec<u32>>(),
  )
}

pub fn solve_a(input_file_path: &str) -> u64 {
  BufReader::new(fs::File::open(input_file_path).unwrap())
    .lines()
    .map(|line| parse_input(&line.unwrap()))
    .map(|(_, numbers, guesses)| {
      numbers
        .iter()
        .map(|n| guesses.iter().any(|g| g == n) as i32)
        .map(|b| b as i32)
        .sum::<i32>()
    })
    .map(|count| match count {
      0 => 0,
      x => i32::pow(2, (x - 1) as u32) as u64,
    })
    .sum()
}

pub fn solve_b(input_file_path: &str) -> u64 {
  BufReader::new(fs::File::open(input_file_path).unwrap())
    .lines()
    .map(|line| parse_input(&line.unwrap()))
    .map(|(id, numbers, guesses)| {
      (
        id,
        numbers
          .iter()
          .map(|n| guesses.iter().any(|g| g == n) as i32)
          .map(|b| b as i32)
          .sum::<i32>(),
      )
    })
    .fold(HashMap::new(), |mut acc, (id, count)| {
      if acc.get(&(id as i32)).is_none() {
        acc.insert(id as i32, 1);
      }

      ((id as i32) + 1..=(count + (id as i32))).for_each(|i| {
        let entry = *acc.clone().get(&(i)).unwrap_or(&1);
        let id_copy = id.clone() as i32;
        let times = *acc.clone().get(&id_copy).unwrap_or(&1);
        acc.insert(
          i,
          match i == id as i32 {
            true => 1,
            false => entry + times,
          },
        );
      });

      acc
    })
    .values()
    .sum::<u32>()
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_a() {
    assert_eq!(solve_a(".\\src\\test_input\\day4.txt"), 13);
  }

  #[test]
  fn test_solve_b() {
    assert_eq!(solve_b(".\\src\\test_input\\day4.txt"), 30);
  }
}
