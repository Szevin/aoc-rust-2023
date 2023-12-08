use std::fs;

#[derive(Debug)]
struct Number {
  value: u32,
  position_start: (u32, u32),
  position_end: (u32, u32),
}

impl Clone for Number {
  fn clone(&self) -> Number {
    Number {
      value: self.value,
      position_start: self.position_start,
      position_end: self.position_end,
    }
  }
}

impl Copy for Number {}

impl PartialEq for Number {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value
      && self.position_start == other.position_start
      && self.position_end == other.position_end
  }
}

impl Number {
  fn new(char: char, position_start: (u32, u32), position_end: (u32, u32)) -> Number {
    Number {
      value: char.to_digit(10).unwrap(),
      position_start,
      position_end,
    }
  }

  fn expand_number(&mut self, char: char, position: (u32, u32)) {
    self.value *= 10;
    self.value += char.to_digit(10).unwrap();
    self.position_end = position;
  }

  fn generate_adjacent_coords(&self) -> Vec<(u32, u32)> {
    let mut adjacent_coords: Vec<(u32, u32)> = Vec::new();

    for x in self.position_start.0 as i32 - 1..=self.position_end.0 as i32 + 1 {
      for y in self.position_start.1 as i32 - 1..=self.position_end.1 as i32 + 1 {
        if x != 0 && y != 0 && x.is_positive() && y.is_positive() {
          adjacent_coords.push((x as u32, y as u32));
        }
      }
    }

    adjacent_coords
  }
}

fn check_coords(
  board: &String,
  coords: Vec<(u32, u32)>,
  valid_chars: &Vec<char>,
) -> Vec<(u32, u32)> {
  let mut valid_coords: Vec<(u32, u32)> = Vec::new();

  coords.iter().for_each(|(x, y)| {
    let max_y = board.lines().count() as u32;
    let max_x = board.lines().nth(0).unwrap().chars().count() as u32;

    if *x >= max_x || *y >= max_y {
      return;
    }

    let mut char_iter = board.lines().nth(*y as usize).unwrap().chars();
    let char = char_iter.nth(*x as usize).unwrap();

    if valid_chars.contains(&char) {
      valid_coords.push((*x, *y));
    }
  });

  valid_coords
}

fn get_numbers_for_valid_chars(board: &String, valid_chars: Vec<char>) -> Vec<Number> {
  let mut numbers: Vec<Number> = Vec::new();

  for (y, line) in board.lines().enumerate() {
    let mut prev_char: char = '.';
    let mut current_number: Number = Number {
      value: 0,
      position_start: (0, 0),
      position_end: (0, 0),
    };
    for (x, char) in line.chars().enumerate() {
      if char.is_digit(10) {
        if prev_char.is_digit(10) {
          current_number.expand_number(char, (x as u32, y as u32));
        } else {
          current_number = Number::new(char, (x as u32, y as u32), (x as u32, y as u32));
        }
      }

      if !char.is_digit(10)
        && prev_char.is_digit(10)
        && check_coords(
          &board,
          current_number.generate_adjacent_coords(),
          &valid_chars.to_vec(),
        )
        .len()
          > 0
      {
        numbers.push(current_number);
      }

      if x == line.len() - 1
        && char.is_digit(10)
        && check_coords(
          &board,
          current_number.generate_adjacent_coords(),
          &valid_chars.to_vec(),
        )
        .len()
          > 0
      {
        numbers.push(current_number);
      }

      prev_char = char;
    }
  }

  numbers
}

pub fn solve_a(input_file_path: &str) -> u64 {
  let board = fs::read_to_string(input_file_path).unwrap();

  get_numbers_for_valid_chars(
    &board,
    ['*', '-', '#', '/', '=', '%', '$', '&', '@', '+'].to_vec(),
  )
  .iter()
  .fold(0, |acc, number| acc + number.value)
  .try_into()
  .unwrap()
}

pub fn solve_b(input_file_path: &str) -> u64 {
  let board = fs::read_to_string(input_file_path).unwrap();
  let numbers = get_numbers_for_valid_chars(&board, ['*'].to_vec());

  // only keep numbers, that have a common * adjacent to them
  let matching_numbers = numbers.iter().map(|number| {
    let adjacent_coords = check_coords(&board, number.generate_adjacent_coords(), &['*'].to_vec());

    let other_number = numbers.iter().find(|other_number| {
      if number == *other_number {
        return false;
      }

      check_coords(
        &board,
        other_number.generate_adjacent_coords(),
        &['*'].to_vec(),
      )
      .iter()
      .any(|coord| adjacent_coords.contains(coord))
    });

    let other_number_value = match other_number {
      Some(other_number) => other_number.value,
      None => 0,
    };

    (number.value, other_number_value)
  });

  (matching_numbers.fold(0, |acc, (number_value, other_number_value)| {
    acc + number_value * other_number_value
  }) / 2)
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_a() {
    assert_eq!(solve_a(".\\src\\test_input\\day3.txt"), 4361);
  }

  #[test]
  fn test_solve_b() {
    assert_eq!(solve_b(".\\src\\test_input\\day3.txt"), 467835);
  }
}
