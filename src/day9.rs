use std::fs;

fn solve(input: &str, reverse: bool) -> u64 {
  input
    .lines()
    .map(|line| {
      let mut histories: Vec<Vec<i64>> = Vec::new();

      let first_history: Vec<i64> = line
        .split(" ")
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

      histories.push(if reverse {
        first_history.iter().rev().map(|x| *x).collect()
      } else {
        first_history
      });

      while !histories.last().unwrap().iter().all(|x| *x == 0) {
        let last = histories.last().unwrap();
        let mut next_history: Vec<i64> = Vec::new();

        for i in 0..last.len() - 1 {
          next_history.push(last[i + 1] - last[i]);
        }

        histories.push(next_history);
      }

      histories
        .iter()
        .map(|history| history.last().unwrap())
        .sum::<i64>()
    })
    .sum::<i64>() as u64
}

pub fn solve_a(input_file_path: &str) -> u64 {
  let input = fs::read_to_string(input_file_path).unwrap();

  solve(&input, false)
}

pub fn solve_b(input_file_path: &str) -> u64 {
  let input = fs::read_to_string(input_file_path).unwrap();

  solve(&input, true)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_a() {
    assert_eq!(solve_a(".\\src\\test_input\\day9.txt"), 114);
  }

  #[test]
  fn test_solve_b() {
    assert_eq!(solve_b(".\\src\\test_input\\day9.txt"), 2);
  }
}
