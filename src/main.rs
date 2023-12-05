use std::collections::HashMap;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

type Solver = fn(&str) -> u32;

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

  let key = format!("{}{}", args[1], args[2]);

  let time;
  let time_elapsed;

  match solvers.get(&key) {
    Some(solver) => {
      time = std::time::Instant::now();
      let input = format!(".\\src\\input\\day{}.txt", args[1]);
      let result = solver(input.as_str());
      time_elapsed = time.elapsed().as_secs_f32();
      println!("\nResult: {}", result);
    }
    None => panic!("Unknown day or part"),
  }

  println!("\nTime: {}s", time_elapsed);
}
