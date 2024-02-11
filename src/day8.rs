use std::collections::HashMap;

const DAY: u8 = 8;
pub const INPUT: &str = include_str!("../inputs/day8.txt");

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
    let (mut lr, elements) = parse_part1(input);

    let mut key = "AAA";
    let mut steps = 0;
    loop {
        key = elements.get(key).unwrap()[lr.next()];
        steps += 1;
        if key == "ZZZ" {
            break;
        }
    }
    steps
}

fn solve_part2(input: &str) -> u64 {
    let (lr, elements, nodes) = parse_part2(input);

    // Looks like for each starting position an end position is reached in a cycle.
    let mut periods = nodes.iter().map(|mut node| {
        let mut steps = 0u64;
        let mut lr = lr.clone();

        loop {
            steps += 1;
            node = &elements.get(node).unwrap()[lr.next()];

            if node.ends_with('Z') {
                break;
            }
        }

        steps
    });

    let mut lcm = periods.next().unwrap();
    for p in periods {
        lcm = num::integer::lcm(lcm, p);
    }

    lcm
}

#[derive(Debug, Clone)]
struct LR {
    iter: std::iter::Cycle<std::vec::IntoIter<usize>>,
}

impl LR {
    fn from_lr_sequence(input: &str) -> Self {
        let sequence = input.chars().map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        });
        LR {
            iter: sequence.collect::<Vec<_>>().into_iter().cycle(),
        }
    }

    fn next(&mut self) -> usize {
        self.iter.next().unwrap()
    }
}

fn parse_part1(input: &str) -> (LR, HashMap<&str, [&str; 2]>) {
    let mut lines = input.lines();

    let lr = LR::from_lr_sequence(lines.next().unwrap());
    lines.next(); // eat empty line after LR sequence

    let mut elements = HashMap::new();

    for line in lines {
        let (key, dst) = line.split_once(" = ").unwrap();
        let (left, right) = dst
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();
        elements.insert(key, [left, right]);
    }

    (lr, elements)
}

fn parse_part2(input: &str) -> (LR, HashMap<&str, [&str; 2]>, Vec<&str>) {
    let mut lines = input.lines();

    let lr = LR::from_lr_sequence(lines.next().unwrap());
    lines.next(); // eat empty line after LR sequence

    let mut elements = HashMap::new();
    let mut start_elements = Vec::new();

    for line in lines {
        let (key, dst) = line.split_once(" = ").unwrap();
        let (left, right) = dst
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();
        elements.insert(key, [left, right]);

        if key.ends_with('A') {
            start_elements.push(key);
        }
    }

    (lr, elements, start_elements)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    "};

    const TEST_INPUT2: &str = indoc::indoc! {"
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    "};

    const TEST_INPUT3: &str = indoc::indoc! {"
    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    "};

    #[test]
    fn test_parse() {
        let (mut lr, _) = parse_part1(TEST_INPUT2);

        for _ in 0..10 {
            println!("{:?}", lr.next());
        }
    }

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 2);

        let answer = solve_part1(TEST_INPUT2);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 2);

        let answer = solve_part2(TEST_INPUT2);
        assert_eq!(answer, 6);

        let answer = solve_part2(TEST_INPUT3);
        assert_eq!(answer, 6);
    }

    #[test]
    fn periods() {
        let (mut lr, elements, mut nodes) = parse_part2(INPUT);

        // Looks like for each starting position an end position is reached in a cycle.
        let orig_nodes = nodes.clone();
        let mut periods = HashMap::new();
        let mut steps = HashMap::new();
        for _ in 0..1_000_000 {
            let lr_index = lr.next();
            for (key, orig_key) in nodes.iter_mut().zip(orig_nodes.iter()) {
                *key = elements.get(key).unwrap()[lr_index];
                *steps.entry(orig_key).or_insert(0) += 1;
                if key.ends_with('Z') {
                    periods
                        .entry(orig_key)
                        .or_insert_with(HashSet::new)
                        .insert(steps[orig_key]);
                    *steps.entry(orig_key).or_insert(0) = 0;
                }
            }
        }

        println!("{:?}", periods);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 15_989);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 13_830_919_117_339);
    }
}
