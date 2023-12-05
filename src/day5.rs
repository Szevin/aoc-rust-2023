use rayon::prelude::*;
use std::fs;

#[derive(Debug)]
struct Range {
  destination: i64,
  source: i64,
  range: i64,
}

#[derive(Debug)]
struct SeedMap {
  map: Vec<Range>,
}

#[derive(Debug)]
struct SeedMaps {
  seed_to_soil: SeedMap,
  soil_to_fertilizer: SeedMap,
  fertilizer_to_water: SeedMap,
  water_to_light: SeedMap,
  light_to_temperature: SeedMap,
  temperature_to_humidity: SeedMap,
  humidity_to_location: SeedMap,
}

impl SeedMaps {
  fn new() -> SeedMaps {
    SeedMaps {
      seed_to_soil: SeedMap::new(),
      soil_to_fertilizer: SeedMap::new(),
      fertilizer_to_water: SeedMap::new(),
      water_to_light: SeedMap::new(),
      light_to_temperature: SeedMap::new(),
      temperature_to_humidity: SeedMap::new(),
      humidity_to_location: SeedMap::new(),
    }
  }

  fn solve(&self, source: i64) -> i64 {
    self.humidity_to_location.find_in_map(
      self.temperature_to_humidity.find_in_map(
        self.light_to_temperature.find_in_map(
          self.water_to_light.find_in_map(
            self.fertilizer_to_water.find_in_map(
              self
                .soil_to_fertilizer
                .find_in_map(self.seed_to_soil.find_in_map(source)),
            ),
          ),
        ),
      ),
    )
  }
}

impl SeedMap {
  fn new() -> SeedMap {
    SeedMap { map: Vec::new() }
  }

  fn find_in_map(&self, source: i64) -> i64 {
    self
      .map
      .iter()
      .find_map(|range| {
        if range.source > source || source > range.source + range.range {
          return None;
        }

        (source..=source + range.range).find_map(|x| {
          if range.source <= x && x < range.source + range.range {
            return Some(range.destination - range.source + x);
          }
          return None;
        })
      })
      .unwrap_or(source)
  }
}

fn parse_input(input: &str) -> (Vec<i64>, SeedMaps) {
  let mut seed_map = SeedMaps::new();

  let groups = input.split("\n\n").collect::<Vec<&str>>();
  let seed = groups[0]
    .split_once(": ")
    .unwrap()
    .1
    .split_whitespace()
    .filter_map(|x| x.parse::<i64>().ok())
    .collect::<Vec<i64>>();

  for group in groups[1..].iter() {
    let mut lines = group.lines();
    let category = lines.next().unwrap();

    let ranges: Vec<Range> = lines
      .map(|line| {
        let mut parts = line.split_whitespace();
        let destination = parts.next().unwrap().parse::<i64>().unwrap();
        let source = parts.next().unwrap().parse::<i64>().unwrap();
        let range = parts.next().unwrap().parse::<i64>().unwrap();

        Range {
          destination,
          source,
          range,
        }
      })
      .collect::<Vec<Range>>();

    match category {
      "seed-to-soil map:" => {
        seed_map.seed_to_soil.map = ranges;
      }
      "soil-to-fertilizer map:" => {
        seed_map.soil_to_fertilizer.map = ranges;
      }
      "fertilizer-to-water map:" => {
        seed_map.fertilizer_to_water.map = ranges;
      }
      "water-to-light map:" => {
        seed_map.water_to_light.map = ranges;
      }
      "light-to-temperature map:" => {
        seed_map.light_to_temperature.map = ranges;
      }
      "temperature-to-humidity map:" => {
        seed_map.temperature_to_humidity.map = ranges;
      }
      "humidity-to-location map:" => {
        seed_map.humidity_to_location.map = ranges;
      }
      _ => {}
    }
  }

  (seed, seed_map)
}

pub fn solve_a(input_file_path: &str) -> u32 {
  let input = fs::read_to_string(input_file_path).unwrap();
  let (seeds, seed_map) = parse_input(&input);

  seeds
    .iter()
    .map(|seed| seed_map.solve(*seed))
    .min()
    .unwrap()
    .try_into()
    .unwrap()
}

pub fn solve_b(input_file_path: &str) -> u32 {
  let input = fs::read_to_string(input_file_path).unwrap();
  let (seeds, seed_map) = parse_input(&input);

  seeds
    .iter()
    .zip(seeds.iter().skip(1))
    .filter(|(i, _)| *i % 2 == 1)
    .collect::<Vec<_>>()
    .par_iter()
    .map(|(i, j)| {
      (**i..**i + **j)
        .map(|seed| seed_map.solve(seed))
        .min()
        .unwrap()
    })
    .min()
    .unwrap()
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_a() {
    assert_eq!(solve_a(".\\src\\test_input\\day5.txt"), 35);
  }

  #[test]
  fn test_solve_b() {
    assert_eq!(solve_b(".\\src\\test_input\\day5.txt"), 46);
  }
}
