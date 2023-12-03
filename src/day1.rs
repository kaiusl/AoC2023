const DAY: u8 = 1;
pub const INPUT: &str = include_str!("../inputs/day1.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // iterating over bytes is much faster than looping over chars
            // but we know that we are looking for ascii characters
            let first = line
                .bytes()
                .find_map(|c| (c as char).to_digit(10))
                .expect("expected a line to contain at least one number");
            let last = line
                .bytes()
                .rev()
                .find_map(|c| (c as char).to_digit(10))
                .expect("expected a line to contain at least one number");

            first * 10 + last
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line
                .bytes()
                .enumerate()
                .find_map(|(pos, c)| get_digit(c, line, pos))
                .expect("expected a line to contain at least one number");
            let last = line
                .bytes()
                .enumerate()
                .rev()
                .find_map(|(pos, c)| rget_digit(c, line, pos))
                .expect("expected a line to contain at least one number");

            first * 10 + last
        })
        .sum()
}

fn rget_digit(char: u8, line: &str, pos: usize) -> Option<u32> {
    if let Some(d) = (char as char).to_digit(10) {
        return Some(d);
    }
    match char {
        b'e' if line.get(pos.saturating_sub(2)..pos) == Some("on") => Some(1),
        b'o' if line.get(pos.saturating_sub(2)..pos) == Some("tw") => Some(2),
        b'e' if line.get(pos.saturating_sub(4)..pos) == Some("thre") => Some(3),
        b'r' if line.get(pos.saturating_sub(3)..pos) == Some("fou") => Some(4),
        b'e' if line.get(pos.saturating_sub(3)..pos) == Some("fiv") => Some(5),
        b'x' if line.get(pos.saturating_sub(2)..pos) == Some("si") => Some(6),
        b'n' if line.get(pos.saturating_sub(4)..pos) == Some("seve") => Some(7),
        b't' if line.get(pos.saturating_sub(4)..pos) == Some("eigh") => Some(8),
        b'e' if line.get(pos.saturating_sub(3)..pos) == Some("nin") => Some(9),
        _ => None,
    }
}

fn get_digit(char: u8, line: &str, pos: usize) -> Option<u32> {
    if let Some(d) = (char as char).to_digit(10) {
        return Some(d);
    }
    match char {
        b'o' if line.get(pos + 1..pos + 3) == Some("ne") => Some(1),
        b't' if line.get(pos + 1..pos + 3) == Some("wo") => Some(2),
        b't' if line.get(pos + 1..pos + 5) == Some("hree") => Some(3),
        b'f' if line.get(pos + 1..pos + 4) == Some("our") => Some(4),
        b'f' if line.get(pos + 1..pos + 4) == Some("ive") => Some(5),
        b's' if line.get(pos + 1..pos + 3) == Some("ix") => Some(6),
        b's' if line.get(pos + 1..pos + 5) == Some("even") => Some(7),
        b'e' if line.get(pos + 1..pos + 5) == Some("ight") => Some(8),
        b'n' if line.get(pos + 1..pos + 4) == Some("ine") => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let answer = solve_part1(input);

        assert_eq!(answer, 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let answer = solve_part2(input);

        assert_eq!(answer, 281);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 56049);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 54530);
    }
}
