use std::collections::HashMap;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

type Solver = fn(&str) -> u64;

fn main() {
  let args: Vec<_> = std::env::args().collect();
  if args.len() != 3 {
    panic!("Usage: {} <day> <part>", args[0]);
  }

  let mut solvers: HashMap<String, Solver> = HashMap::new();
  solvers.insert("1a".to_string(), day1::solve_a);
  solvers.insert("1b".to_string(), day1::solve_b);
  solvers.insert("2a".to_string(), day2::solve_a);
  solvers.insert("2b".to_string(), day2::solve_b);
  solvers.insert("3a".to_string(), day3::solve_a);
  solvers.insert("3b".to_string(), day3::solve_b);
  solvers.insert("4a".to_string(), day4::solve_a);
  solvers.insert("4b".to_string(), day4::solve_b);
  solvers.insert("5a".to_string(), day5::solve_a);
  solvers.insert("5b".to_string(), day5::solve_b);
  solvers.insert("6a".to_string(), day6::solve_a);
  solvers.insert("6b".to_string(), day6::solve_b);
  solvers.insert("7a".to_string(), day7::solve_a);
  solvers.insert("7b".to_string(), day7::solve_b);
  solvers.insert("8a".to_string(), day8::solve_a);
  solvers.insert("8b".to_string(), day8::solve_b);
  solvers.insert("9a".to_string(), day9::solve_a);
  solvers.insert("9b".to_string(), day9::solve_b);

  let key = format!("{}{}", args[1], args[2]);

  let time;
  let time_elapsed;

  match solvers.get(&key) {
    Some(solver) => {
      let input = format!(".\\src\\input\\day{}.txt", args[1]);
      time = std::time::Instant::now();
      let result = solver(input.as_str());
      time_elapsed = time.elapsed().as_secs_f32();
      println!("\nResult: {}", result);
    }
    None => panic!("Unknown day or part"),
  }

  println!("\nTime: {}s", time_elapsed);
}
