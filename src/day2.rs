use std::fs;
use std::io::{BufRead, BufReader};

fn parse_input(input: &str) -> (u32, Vec<(u32, String)>) {
    let binding = input;
    let (score, game) = binding.split_once(": ").unwrap();

    let score = score.split_once(" ").unwrap().1;

    let game = game
        .split(|c| c == ',' || c == ';')
        .map(|s| s.trim())
        .map(|ball| {
            let (amount, color) = ball.split_once(' ').unwrap();
            let amount = amount.parse::<u32>().unwrap();
            let color = color.to_string();
            (amount, color)
        })
        .collect::<Vec<(u32, String)>>();

    (score.parse::<u32>().unwrap(), game)
}

pub fn solve_a(input_file_path: &str) -> u32 {
    BufReader::new(fs::File::open(input_file_path).unwrap())
        .lines()
        .map(|line| parse_input(&line.unwrap()))
        .filter(|(_, game)| {
            !game.iter().any(|(amount, color)| match color.as_str() {
                "red" => amount > &12,
                "green" => amount > &13,
                "blue" => amount > &14,
                _ => false,
            })
        })
        .map(|(score, _)| score)
        .sum::<u32>()
}

pub fn solve_b(input_file_path: &str) -> u32 {
    BufReader::new(fs::File::open(input_file_path).unwrap())
        .lines()
        .map(|line| parse_input(&line.unwrap()))
        .map(|line| {
            let game = line
                .1
                .iter()
                .fold((0, 0, 0), |acc, ball| match ball.1.as_str() {
                    "red" => {
                        if ball.0 > acc.0 {
                            (ball.0, acc.1, acc.2)
                        } else {
                            acc
                        }
                    }
                    "green" => {
                        if ball.0 > acc.1 {
                            (acc.0, ball.0, acc.2)
                        } else {
                            acc
                        }
                    }
                    "blue" => {
                        if ball.0 > acc.2 {
                            (acc.0, acc.1, ball.0)
                        } else {
                            acc
                        }
                    }
                    _ => acc,
                });

            game.0 * game.1 * game.2
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(".\\src\\test_input\\day2.txt"), 8);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(".\\src\\test_input\\day2.txt"), 2286);
    }
}
