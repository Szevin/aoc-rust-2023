#![allow(dead_code, unused_imports, unused_variables)]
use std::fs;

type Coord = (u128, u128);

#[derive(Debug)]
struct Cell {
  coord: Coord,
  symbol: char,
}

#[derive(Debug)]
struct Galaxy {
  planets: Vec<Coord>,
  empty_rows: Vec<u128>,
  empty_cols: Vec<u128>,
}

impl Galaxy {
  fn new(input: &str) -> Self {
    let mut planets: Vec<Coord> = Vec::new();
    let empty_rows: Vec<u128> = input
      .lines()
      .enumerate()
      .filter(|(_, line)| line.chars().all(|c| c == '.'))
      .map(|(i, _)| i as u128)
      .collect();
    let mut empty_cols: Vec<u128> = input.lines().enumerate().map(|(x, _)| x as u128).collect();

    for (y, line) in input.lines().enumerate() {
      for (x, symbol) in line.chars().enumerate() {
        match symbol {
          '#' => {
            planets.push((y as u128, x as u128));
            empty_cols.retain(|&i| i != x as u128);
          }
          _ => {}
        }
      }
    }

    Galaxy {
      planets,
      empty_rows,
      empty_cols,
    }
  }

  fn expand(&mut self, expansion_multiplier: u128) -> &Self {
    self.planets = self
      .planets
      .iter()
      .map(|planet| {
        let mut expand_distace: Coord = (0, 0);

        self.empty_cols.iter().for_each(|&col| {
          if planet.1 > col {
            expand_distace.1 += expansion_multiplier - 1;
          }
        });
        self.empty_rows.iter().for_each(|&row| {
          if planet.0 > row {
            expand_distace.0 += expansion_multiplier - 1;
          }
        });

        (planet.0 + expand_distace.0, planet.1 + expand_distace.1)
      })
      .collect();

    self
  }

  fn manhattan_distances(&self) -> Vec<u64> {
    self
      .planets
      .iter()
      .enumerate()
      .map(|(i, planet)| {
        self
          .planets
          .iter()
          .enumerate()
          .filter_map(move |(j, other_planet)| {
            if i != j {
              Some(manhattan_distance(*planet, *other_planet))
            } else {
              None
            }
          })
          .sum::<u64>()
      })
      .collect::<Vec<u64>>()
  }
}

fn manhattan_distance(a: Coord, b: Coord) -> u64 {
  ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as u64
}

pub fn solve_a(input_file_path: &str) -> u64 {
  let mut galaxy = Galaxy::new(&fs::read_to_string(input_file_path).unwrap());
  galaxy.expand(2);
  galaxy.manhattan_distances().iter().sum::<u64>() / 2
}

pub fn solve_b(input_file_path: &str, expansion_multiplier: u128) -> u64 {
  let mut galaxy = Galaxy::new(&fs::read_to_string(input_file_path).unwrap());
  galaxy.expand(expansion_multiplier);
  galaxy.manhattan_distances().iter().sum::<u64>() / 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_a() {
    assert_eq!(solve_a(".\\src\\test_input\\day11.txt"), 374);
  }

  #[test]
  fn test_solve_b() {
    assert_eq!(solve_b(".\\src\\test_input\\day11.txt", 10), 1030);
    assert_eq!(solve_b(".\\src\\test_input\\day11.txt", 100), 8410);
  }
}
