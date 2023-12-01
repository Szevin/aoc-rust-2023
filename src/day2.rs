use std::fs;

pub fn solve_a(input_file_path: &str) -> u32 {
    BufReader::new(fs::File::open(input_file_path).unwrap()).lines()
}

pub fn solve_b(input_file_path: &str) -> u32 {
    BufReader::new(fs::File::open(input_file_path).unwrap()).lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(".\\src\\test_input\\day2a.txt"), 142);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(".\\src\\test_input\\day2b.txt"), 281);
    }
}
