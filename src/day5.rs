use std::ops::Range;

use rangemap::RangeMap;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const DAY: u8 = 5;
pub const INPUT: &str = include_str!("../inputs/day5.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

fn solve_part1(input: &str) -> u64 {
    let (seeds, data) = parse_part1(input);

    let mut min_location = u64::MAX;
    for seed in seeds {
        let location = data.get_location(seed);
        min_location = min_location.min(location);
        //print!("{} ", location);
    }

    min_location
}

fn solve_part2(input: &str) -> u64 {
    let (seeds, data) = parse_part2(input);

    seeds
        .into_par_iter()
        .flat_map(|r| r.into_par_iter())
        .map(|seed| data.get_location(seed))
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Almanac {
    seed_to_soil_map: RangeMap<u64, u64>,
    soil_to_fertilizer_map: RangeMap<u64, u64>,
    fertilizer_to_water_map: RangeMap<u64, u64>,
    water_to_light_map: RangeMap<u64, u64>,
    light_to_temperature_map: RangeMap<u64, u64>,
    temperature_to_humidity_map: RangeMap<u64, u64>,
    humidity_to_location_map: RangeMap<u64, u64>,
}

impl Almanac {
    fn get_location(&self, seed: u64) -> u64 {
        fn get_mapped_value(map: &RangeMap<u64, u64>, key: u64) -> u64 {
            let (key_range, next_start) = map
                .get_key_value(&key)
                .map(|(a, b)| (a.clone(), *b))
                .unwrap_or((key..key + 1, key));

            next_start + (key - key_range.start)
        }

        let soil = get_mapped_value(&self.seed_to_soil_map, seed);
        let fertilizer = get_mapped_value(&self.soil_to_fertilizer_map, soil);
        let water = get_mapped_value(&self.fertilizer_to_water_map, fertilizer);
        let light = get_mapped_value(&self.water_to_light_map, water);
        let temperature = get_mapped_value(&self.light_to_temperature_map, light);
        let humidity = get_mapped_value(&self.temperature_to_humidity_map, temperature);

        get_mapped_value(&self.humidity_to_location_map, humidity)
    }
}

fn parse_part1(input: &str) -> (Vec<u64>, Almanac) {
    let mut lines = input.lines();
    (parse_seeds_part1(&mut lines), parse_maps(&mut lines))
}

fn parse_part2(input: &str) -> (Vec<Range<u64>>, Almanac) {
    let mut lines = input.lines();
    (parse_seeds_part2(&mut lines), parse_maps(&mut lines))
}

fn parse_maps<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Almanac {
    Almanac {
        seed_to_soil_map: parse_map(lines),
        soil_to_fertilizer_map: parse_map(lines),
        fertilizer_to_water_map: parse_map(lines),
        water_to_light_map: parse_map(lines),
        light_to_temperature_map: parse_map(lines),
        temperature_to_humidity_map: parse_map(lines),
        humidity_to_location_map: parse_map(lines),
    }
}

fn parse_seeds_part1<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<u64> {
    let line = lines.next().expect("expected non empty input");
    assert!(
        line.starts_with("seeds: "),
        "expected line to start with 'seeds: '"
    );
    let line = line.trim_start_matches("seeds: ");
    let result = line.split(' ').map(|s| s.parse().unwrap()).collect();
    lines.next(); // eat empty line after seeds
    result
}

fn parse_seeds_part2<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<Range<u64>> {
    let line = lines.next().expect("expected non empty input");
    assert!(
        line.starts_with("seeds: "),
        "expected line to start with 'seeds: '"
    );
    let line = line.trim_start_matches("seeds: ");
    let mut nums = line.split(' ').map(|s| s.parse::<u64>().unwrap());

    let mut seeds = Vec::new();
    loop {
        match (nums.next(), nums.next()) {
            (Some(start), Some(len)) => seeds.push(start..start + len),
            (None, None) => break,
            _ => panic!("invalid input: {}", line),
        }
    }

    lines.next(); // eat empty line after seeds
    seeds
}

fn parse_map<'a>(lines: &mut impl Iterator<Item = &'a str>) -> RangeMap<u64, u64> {
    let mut map = RangeMap::new();

    let line = lines.next().unwrap();
    assert!(line.ends_with("map:"), "{}", line);

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut nums = line.split(' ').map(|s| s.parse::<u64>().unwrap());
        let dst_start = nums.next().unwrap();
        let src_start = nums.next().unwrap();
        let len = nums.next().unwrap();
        assert!(nums.next().is_none());

        map.insert(src_start..src_start + len, dst_start);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    "};

    #[test]
    fn test_parser() {
        let answer = parse_part1(TEST_INPUT1);
        println!("{:#?}", answer);
    }

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 35);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 46);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 484023871);
    }

    #[divan::bench(sample_count = 5, sample_size = 1)]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 46294175);
    }
}
