const DAY: u8 = 14;
pub const INPUT: &str = include_str!("../inputs/day14.txt");

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
    // Iterate the rows in reverse and count the rolling rocks in each column.
    // If we find a square rock in a column, we know where all of the rolling rocks
    // below it will stop. Hence we can calculate the weight of those rocks.
    // We continue until we reach the end of the input.
    // Then finally we'll need to add the final rolling rocks too that stopped at the top edge.

    let cols = input.lines().next().unwrap().chars().count();

    let mut rolling_rocks = vec![0; cols];
    let mut total_load = 0;
    let mut max_weight = 0;
    for (l, weight) in input.lines().rev().zip(1..) {
        // dbg!(weight, l);
        for (i, c) in l.chars().enumerate() {
            match c {
                'O' => rolling_rocks[i] += 1,
                '.' => {}
                '#' => {
                    // for i in 0..rolling_rocks[i] {
                    //     // -1 because the square rock is on the line,
                    //     // rolling rocks start from previous line
                    //     total_load += weight - 1 - i;
                    // }

                    // above has closed form solution
                    let num_rolling_rocks = rolling_rocks[i];
                    if num_rolling_rocks != 0 {
                        total_load +=
                            num_rolling_rocks * (weight - 1) - triag_number(num_rolling_rocks - 1);
                    }
                    rolling_rocks[i] = 0;
                }
                _ => unreachable!(),
            }
        }
        max_weight = weight;
    }

    for num_rolling_rocks in rolling_rocks.into_iter().filter(|&r| r != 0) {
        total_load += num_rolling_rocks * max_weight - triag_number(num_rolling_rocks - 1);
    }

    total_load
}

/// Calculates the triangular number of n.
///
/// That is 1 + 2 + 3 + 4 + ... + n
fn triag_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn solve_part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    "};

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 136);
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
        assert_eq!(answer, 108144);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        // assert_eq!(answer, todo!());
    }
}
