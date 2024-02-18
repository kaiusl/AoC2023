const DAY: u8 = 15;
pub const INPUT: &str = include_str!("../inputs/day15.txt");

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
    input.split(',').map(|s| hash(s) as u64).sum()
}

fn solve_part2(input: &str) -> u64 {
    todo!()
}

fn hash(input: &str) -> u8 {
    input.as_bytes().iter().fold(0, |acc, &ascii| {
        let mut tmp = acc as u16;
        tmp += ascii as u16;
        tmp *= 17;
        (tmp % 256) as u8
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 1320);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, todo!());
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 513214);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, todo!());
    }
}
