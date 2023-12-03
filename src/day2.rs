use std::str::FromStr;

const DAY: u8 = 2;
pub const INPUT: &str = include_str!("../inputs/day2.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

#[derive(Debug)]
struct Game {
    id: u64,
    subsets: Vec<Subset>,
}

impl Game {
    fn from_str(s: &str) -> Self {
        let (game, subsets) = s
            .split_once(": ")
            .unwrap_or_else(|| panic!("invalid input for game `{}`", s));
        let (_, id) = game
            .split_once(' ')
            .unwrap_or_else(|| panic!("invalid input for game `{}`", s));
        let id = u64::from_str(id)
            .unwrap_or_else(|e| panic!("failed to parse game id `{id}` with error {e}"));

        Self {
            id,
            subsets: subsets.split("; ").map(Subset::from_str).collect(),
        }
    }
}

impl Game {
    fn is_possible_with(&self, reds: u64, greens: u64, blues: u64) -> bool {
        for s in self.subsets.iter() {
            for cube in s.cubes.iter() {
                match cube.color {
                    Color::Red => {
                        if cube.count > reds {
                            return false;
                        }
                    }
                    Color::Green => {
                        if cube.count > greens {
                            return false;
                        }
                    }
                    Color::Blue => {
                        if cube.count > blues {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn power_of_min_set_of_cubes(&self) -> u64 {
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;

        for s in self.subsets.iter() {
            for cube in s.cubes.iter() {
                match cube.color {
                    Color::Red => reds = reds.max(cube.count),
                    Color::Green => greens = greens.max(cube.count),
                    Color::Blue => blues = blues.max(cube.count),
                }
            }
        }

        reds * greens * blues
    }
}

#[derive(Debug)]
struct Subset {
    cubes: Vec<Cubes>,
}

impl Subset {
    fn from_str(input: &str) -> Self {
        Self {
            cubes: input.split(", ").map(Cubes::from_str).collect(),
        }
    }
}

#[derive(Debug)]
struct Cubes {
    count: u64,
    color: Color,
}

impl Cubes {
    fn from_str(input: &str) -> Self {
        let (count, color) = input
            .split_once(' ')
            .unwrap_or_else(|| panic!("invalid input for cube `{}`", input));
        let count = u64::from_str(count).unwrap_or_else(|e| {
            panic!("failed to parse cube count `{count}` for cube `{input}` with error {e}")
        });
        let color = Color::from_str(color)
            .unwrap_or_else(|_| panic!("failed to parse cube color `{color}` for cube `{input}`"));
        Self { count, color }
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_str(input: &str) -> Result<Self, ()> {
        Ok(match input {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => return Err(()),
        })
    }
}

fn solve_part1(input: &str) -> u64 {
    const MAX_RED: u64 = 12;
    const MAX_GREEN: u64 = 13;
    const MAX_BLUE: u64 = 14;

    input
        .lines()
        .map(Game::from_str)
        .filter_map(|g| {
            g.is_possible_with(MAX_RED, MAX_GREEN, MAX_BLUE)
                .then_some(g.id)
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .map(Game::from_str)
        .map(|g| g.power_of_min_set_of_cubes())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 8);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 2286);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 2239);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 83435);
    }
}
