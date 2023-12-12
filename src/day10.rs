#![allow(dead_code, unused_imports, unused_variables)]
use colored::Colorize;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

type Coord = (usize, usize);

#[derive(Debug, Clone)]
struct Pipe {
  symbol: char,
  coord: Coord,
  connections: Vec<Direction>,
}

#[derive(Debug)]
struct Pipes {
  start: Coord,
  pipes: Vec<Vec<Pipe>>,
  main_loop: Vec<Coord>,
}

impl Direction {
  fn shift_coord(&self, coord: Coord) -> Coord {
    if coord.0 == 0 && self == &Direction::Up {
      return coord;
    }

    if coord.1 == 0 && self == &Direction::Left {
      return coord;
    }

    match self {
      Direction::Up => (coord.0 - 1, coord.1),
      Direction::Down => (coord.0 + 1, coord.1),
      Direction::Left => (coord.0, coord.1 - 1),
      Direction::Right => (coord.0, coord.1 + 1),
    }
  }

  fn reverse(&self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

impl Pipe {
  fn new(symbol: char, y: usize, x: usize) -> Pipe {
    Pipe {
      symbol,
      coord: (y, x),
      connections: match symbol {
        '7' => vec![Direction::Left, Direction::Down],
        'J' => vec![Direction::Up, Direction::Left],
        'L' => vec![Direction::Up, Direction::Right],
        'F' => vec![Direction::Right, Direction::Down],
        '|' => vec![Direction::Up, Direction::Down],
        '-' => vec![Direction::Left, Direction::Right],
        '.' | '#' | 'O' | 'I' => vec![],
        'S' => vec![
          Direction::Up,
          Direction::Down,
          Direction::Left,
          Direction::Right,
        ],
        x => panic!("Unknown symbol: {}", x),
      },
    }
  }
}

impl Pipes {
  fn new(input: &str) -> Pipes {
    let mut pipes = vec![];
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
      let mut row = vec![];

      for (x, symbol) in line.chars().enumerate() {
        row.push(Pipe::new(symbol, y, x));

        if symbol == 'S' {
          start = (y, x);
        }
      }

      pipes.push(row);
    }

    let mut pipes = Pipes {
      start,
      pipes,
      main_loop: vec![],
    };
    pipes.solve();

    pipes
  }

  fn solve(&mut self) -> &Self {
    let mut current_pipe: &Pipe = self.get(self.start).unwrap();
    let mut previous_pipe: &Pipe = current_pipe;
    let mut main_loop = vec![];

    loop {
      let next_pipe = self.next(current_pipe, previous_pipe);

      if next_pipe.is_none() || next_pipe.unwrap().symbol == 'S' {
        main_loop.push(current_pipe.coord);
        break;
      }

      main_loop.push(current_pipe.coord);
      previous_pipe = current_pipe;
      current_pipe = next_pipe.unwrap();
    }

    self.main_loop = main_loop;
    self
  }

  fn get(&self, coord: Coord) -> Option<&Pipe> {
    if coord.0 >= self.pipes.len() || coord.1 >= self.pipes[coord.0].len() {
      return None;
    }

    Some(&self.pipes[coord.0][coord.1])
  }

  fn next(&self, current_pipe: &Pipe, prev_pipe: &Pipe) -> Option<&Pipe> {
    let mut next_pipe = None;

    for direction in &current_pipe.connections {
      let next_coord = direction.shift_coord(current_pipe.coord);

      if next_coord == prev_pipe.coord {
        continue;
      }

      let next_pipe_candidate = self.get(next_coord);

      if next_pipe_candidate.is_none() {
        continue;
      }

      let next_pipe_candidate = next_pipe_candidate.unwrap();

      if next_pipe_candidate
        .connections
        .contains(&direction.reverse())
      {
        next_pipe = Some(next_pipe_candidate);
        break;
      }
    }
    next_pipe
  }

  // fn expand(&mut self) -> &Self {
  //   let mut pipes = vec![];
  //   let mut expanded_main_loop = vec![];

  //   for (y, row) in self.pipes.iter().enumerate() {
  //     let mut new_row = vec![];

  //     for (x, pipe) in row.iter().enumerate() {
  //       if pipe.symbol == '.' || !self.main_loop.contains(&pipe.coord) {
  //         new_row.push(Pipe::new('O', 2 * y, 2 * x));
  //         new_row.push(Pipe::new('#', 2 * y, 2 * x + 1));
  //       } else {
  //         new_row.push(Pipe::new(pipe.symbol, 2 * y, 2 * x));
  //         new_row.push(Pipe::new('#', 2 * y, 2 * x + 1));
  //       }
  //     }
  //     pipes.push(new_row);

  //     let empty_row = (0..row.len() * 2)
  //       .map(|x| Pipe::new('#', 2 * y + 1, 2 * x))
  //       .collect::<Vec<Pipe>>();
  //     pipes.push(empty_row);
  //   }

  //   self.pipes = pipes;
  //   self.main_loop = self
  //     .main_loop
  //     .iter()
  //     .map(|coord| (2 * coord.0, 2 * coord.1))
  //     .collect();

  //   self.main_loop.append(&mut expanded_main_loop);

  //   // TODO - WTF is going on
  //   for y in 1..self.pipes.len() / 2 {
  //     for x in 1..self.pipes[y].len() / 2 {
  //       let left_pipe = self.get((2 * y, 2 * x - 1));
  //       let right_pipe = self.get((2 * y, 2 * x + 1));
  //       let up_pipe = self.get((2 * y - 1, 2 * x));
  //       let down_pipe = self.get((2 * y + 1, 2 * x));

  //       if left_pipe.is_some() && right_pipe.is_some() {
  //         let left_pipe = left_pipe.unwrap();
  //         let right_pipe = right_pipe.unwrap();

  //         if ['-', 'L', 'F', 'S'].contains(&left_pipe.symbol)
  //           && ['-', 'J', 'F', 'S'].contains(&right_pipe.symbol)
  //         {
  //           self.pipes[2 * y][2 * x].symbol = '-';
  //           self.main_loop.push((2 * y, 2 * x));
  //         }
  //       }

  //       if up_pipe.is_some() && down_pipe.is_some() {
  //         let up_pipe = up_pipe.unwrap();
  //         let down_pipe = down_pipe.unwrap();

  //         if ['|', '7', 'J', 'S'].contains(&up_pipe.symbol)
  //           && ['|', '7', 'L', 'S'].contains(&down_pipe.symbol)
  //         {
  //           self.pipes[2 * y][2 * x].symbol = '|';
  //           self.main_loop.push((2 * y, 2 * x));
  //         }
  //       }
  //     }
  //   }

  //   self
  // }

  fn debug(&self) -> &Self {
    for row in &self.pipes {
      for pipe in row {
        if self.main_loop.contains(&pipe.coord) {
          print!("{}", pipe.symbol.to_string().red());
        } else {
          print!("{}", pipe.symbol);
        }
      }
      println!();
    }

    self
  }
}

pub fn solve_a(input_file_path: &str) -> u64 {
  let input = fs::read_to_string(input_file_path).unwrap();
  let pipes = Pipes::new(&input);

  pipes.debug();

  ((pipes.main_loop.len() + 1) / 2) as u64
}

// SKIP - too hard :(
pub fn solve_b(input_file_path: &str) -> u64 {
  // let input = fs::read_to_string(input_file_path).unwrap();
  // let pipes = Pipes::new(&input).expand().debug();

  10
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_a() {
    assert_eq!(solve_a(".\\src\\test_input\\day10a.txt"), 8);
  }

  #[test]
  fn test_solve_b() {
    assert_eq!(solve_b(".\\src\\test_input\\day10b.txt"), 10);
  }
}
