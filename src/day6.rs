use std::fs;

#[derive(Debug)]
struct HighScore {
  time: i64,
  distance: i64,
}

fn parse_input(input: &str) -> Vec<HighScore> {
  let times = input
    .lines()
    .nth(0)
    .unwrap()
    .split_whitespace()
    .filter_map(|num| num.parse::<i64>().ok())
    .collect::<Vec<i64>>();

  let distances = input
    .lines()
    .nth(1)
    .unwrap()
    .split_whitespace()
    .filter_map(|num| num.parse::<i64>().ok())
    .collect::<Vec<i64>>();

  times
    .iter()
    .zip(distances.iter())
    .map(|(time, distance)| HighScore {
      time: *time,
      distance: *distance,
    })
    .collect::<Vec<HighScore>>()
}

fn solve_quadratic(a: i64, b: i64, c: i64) -> (f64, f64) {
  let discriminant = b.pow(2) - 4 * a * c;
  let sqrt_discriminant = (discriminant as f64).sqrt();

  let x1 = (-b as f64 + sqrt_discriminant) / (2.0 * a as f64);
  let x2 = (-b as f64 - sqrt_discriminant) / (2.0 * a as f64);

  (x1, x2)
}

pub fn solve_a(input_file_path: &str) -> u32 {
  let high_scores = parse_input(&fs::read_to_string(input_file_path).unwrap());

  high_scores
    .iter()
    .map(|high_score| {
      let (x1, x2) = solve_quadratic(-1, high_score.time, -high_score.distance - 1);

      ((x1.ceil() - x2.floor()).abs() + 1.0) as u32
    })
    .fold(1, |acc, count| acc * count)
}

pub fn solve_b(input_file_path: &str) -> u32 {
  let high_score = parse_input(&fs::read_to_string(input_file_path).unwrap())
    .iter()
    .fold(
      HighScore {
        time: 0,
        distance: 0,
      },
      |acc, high_score| HighScore {
        time: (acc.time.to_string() + &high_score.time.to_string())
          .parse::<i64>()
          .unwrap(),
        distance: (acc.distance.to_string() + &high_score.distance.to_string())
          .parse::<i64>()
          .unwrap(),
      },
    );

  let (x1, x2) = solve_quadratic(-1, high_score.time, -high_score.distance - 1);

  ((x1.ceil() - x2.floor()).abs() + 1.0) as u32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_a() {
    assert_eq!(solve_a(".\\src\\test_input\\day6.txt"), 288);
  }

  #[test]
  fn test_solve_b() {
    assert_eq!(solve_b(".\\src\\test_input\\day6.txt"), 71503);
  }
}
