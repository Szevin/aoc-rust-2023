mod day1;

fn main() {
  let args: Vec<_> = std::env::args().collect();
  if args.len() != 3 {
      panic!("Usage: {} <day> <part>", args[0]);
  }

  println!(
      "{}",
      match args[1].as_str() {
          "1" => match args[2].as_str() {
              "a" => day1::solve_a(format!(".\\src\\input\\day{}.txt", args[1]).as_str()),
              "b" => day1::solve_b(format!(".\\src\\input\\day{}.txt", args[1]).as_str()),
              _ => panic!("Unknown part {}", args[2]),
          },
          _ => panic!("Unknown day {}", args[1]),
      }
  )
}
