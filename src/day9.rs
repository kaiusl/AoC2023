use std::str::FromStr;

const DAY: u8 = 9;
pub const INPUT: &str = include_str!("../inputs/day9.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

// 10  13  16  21  30  45  68
//    3   3   5   9  15  23
//      0   2   4   6   8
//        2   2   2   2
//          0   0   0

fn solve_part1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|d| calc_p1(&d))
        .sum()
}

// 5  10  13  16  21  30  45
//   5   3   3   5   9  15
//    -2   0   2   4   6
//       2   2   2   2
//         0   0   0

fn solve_part2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|d| calc_p2(&d))
        .sum()
}

fn calc_p1(diffs: &[i64]) -> i64 {
    let next_diffs = calc_diffs(diffs);
    if next_diffs.iter().all(|v| v == &0) {
        *diffs.last().unwrap()
    } else {
        diffs.last().unwrap() + calc_p1(&next_diffs)
    }
}

fn calc_p2(diffs: &[i64]) -> i64 {
    let next_diffs = calc_diffs(diffs);
    if next_diffs.iter().all(|v| v == &0) {
        *diffs.first().unwrap()
    } else {
        diffs.first().unwrap() - calc_p2(&next_diffs)
    }
}

fn calc_diffs(diffs: &[i64]) -> Vec<i64> {
    diffs.windows(2).map(|w| w[1] - w[0]).collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(' ').map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    "};

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 114);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 2);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 1887980197);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 990);
    }
}
