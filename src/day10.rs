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

#[derive(Debug)]
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
                '.' => vec![],
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

        Pipes {
            pipes,
            start,
            main_loop: vec![],
        }
    }

    fn solve(&mut self) -> &Self {
        let mut current_pipe: &Pipe = self.get(self.start).unwrap();
        let mut previous_pipe: &Pipe = current_pipe;
        let mut main_loop = vec![];

        loop {
            let next_pipe = self.next(current_pipe, previous_pipe);

            if next_pipe.is_none() || next_pipe.unwrap().symbol == 'S' {
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
}

fn parse_input(input: &str) -> Pipes {
    Pipes::new(input)
}

pub fn solve_a(input_file_path: &str) -> u64 {
    let input = fs::read_to_string(input_file_path).unwrap();
    let mut pipes = parse_input(&input);
    let pipes = pipes.solve();

    for row in &pipes.pipes {
        for pipe in row {
            if pipes.main_loop.contains(&pipe.coord) {
                print!("{}", pipe.symbol.to_string().red());
            } else {
                print!("{}", pipe.symbol);
            }
        }
        println!();
    }

    ((pipes.main_loop.len() + 1) / 2) as u64
}

pub fn solve_b(input_file_path: &str) -> u64 {
    let input = fs::read_to_string(input_file_path).unwrap();

    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(".\\src\\test_input\\day10.txt"), 8);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(".\\src\\test_input\\day10.txt"), 2);
    }
}
